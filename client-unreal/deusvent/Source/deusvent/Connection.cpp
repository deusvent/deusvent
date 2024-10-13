#include "Connection.h"

#include "IWebSocket.h"
#include "WebSocketsModule.h"
#include "Modules/ModuleManager.h"

#include "Logging/StructuredLog.h"
#include "logic/logic.hpp"

#include <thread>

DEFINE_LOG_CATEGORY(LogConnection);

constexpr int32 GConnection_Closed_Normally_Code = 1000;

void UConnection::Init(const char *ServerAddress, const logic::Keys &PlayerKeys) {
    this->Keys = PlayerKeys;
    this->RequestId = 0;

    // HACK WebSocket fails to connect if there is no path/query in the URL. I assume it's related
    // to this one https://github.com/warmcat/libwebsockets/issues/414. If logs enabled via "Log
    // LogWebSockets Verbose" in the console then LogWebSockets will log an error
    // LWS_CALLBACK_CLIENT_CONNECTION_ERROR
    FString ServerURL = FString(ServerAddress);
    if (ServerURL.EndsWith(TEXT(".com"))) {
        ServerURL.Append(TEXT("/?ws"));
    }

    UE_LOGFMT(LogConnection, Display, "Connecting to: {0}", ServerURL);
    Connection = FWebSocketsModule::Get().CreateWebSocket(ServerURL);

    Connection->OnConnected().AddLambda([this]() {
        UE_LOGFMT(LogConnection, Display, "Connected");
        this->TryToSendMessages();
    });

    Connection->OnConnectionError().AddLambda([this](const FString &Error) {
        UE_LOGFMT(LogConnection, Verbose, "Connection error: {0}", Error);
        this->Reconnect(0.3);
    });

    Connection->OnClosed().AddLambda(
        [this](int32 StatusCode, const FString &Reason, bool bWasClean) {
            UE_LOGFMT(LogConnection,
                      Display,
                      "Connection closed: Status={0}, Reason={0}",
                      StatusCode,
                      Reason);
            if (StatusCode != GConnection_Closed_Normally_Code) {
                this->Reconnect(0.3);
            }
        });

    Connection->OnMessage().AddLambda([this](const FString &Message) {
        UE_LOGFMT(LogConnection, Display, "Message received: {0}", Message);
        auto TagPrefix = Message.Mid(0, 2);
        auto RequestIDPrefix = Message.Mid(2, 2);
        auto MessageRequestId = logic::parse_request_id(TCHAR_TO_UTF8(*RequestIDPrefix));
        if (MessageRequestId == 0) {
            // No request id available, so it's either pushed message or an error
            if (Message.StartsWith(FString(logic::server_error_message_tag().c_str()))) {
                auto Error = logic::ServerError::deserialize(TCHAR_TO_UTF8(*Message));
                UE_LOGFMT(LogConnection,
                          Error,
                          "Server error: {0}",
                          FString(Error->debug_string().c_str()));
            } else {
                // Here we would need to add processing of pushed message - warnings for now if any
                UE_LOGFMT(LogConnection,
                          Warning,
                          "Unknown message tag for pushed message: Tag={0}",
                          TagPrefix);
            }
        } else {
            // Request id is available, so it's a response - look for the registered callbacks
            if (!this->Callbacks.Contains(MessageRequestId)) {
                UE_LOGFMT(LogConnection,
                          Warning,
                          "No callback registered for: RequestId={0}, Tag={1}",
                          MessageRequestId,
                          TagPrefix);
                return;
            }
            auto Callback = this->Callbacks[MessageRequestId];
            this->Callbacks.Remove(MessageRequestId);
            Callback(Message);
        }
    });

    Connection->OnMessageSent().AddLambda([](const FString &Message) {
        UE_LOGFMT(LogConnection, Verbose, "Message sent: {0}", Message);
    });

    this->Reconnect(0);
}

void UConnection::Disconnect() {
    UE_LOGFMT(LogConnection, Display, "Disconnecting...");
    this->ReconnectTimerHandle.Invalidate();
    Connection->Close(GConnection_Closed_Normally_Code, "Disconnect requested");
    Connection.Reset();
}

void UConnection::TryToSendMessages() {
    if (!this->Connection->IsConnected()) {
        UE_LOGFMT(LogTemp, Verbose, "No connection yet");
        return; // No connection yet
    }

    FString Data;
    while (this->OutgoingMessages.Dequeue(Data)) {
        UE_LOGFMT(LogTemp, Display, "Sending data {0}", Data);
        this->Connection->Send(Data);
    }
}

void UConnection::Reconnect(float DelaySeconds) {
    if (DelaySeconds == 0) {
        this->Connection->Connect();
    }
    GetWorld()->GetTimerManager().SetTimer(
        this->ReconnectTimerHandle,
        [this]() {
            UE_LOGFMT(LogConnection, Verbose, "Reconnecting...");
            this->Connection->Connect();
        },
        DelaySeconds,
        false);
}

uint8 UConnection::NextRequestId() {
    auto Val = this->RequestId.fetch_add(1);
    // 0 is a special request id which API may return when request_id cannot be parsed from the
    // incoming message or when server message was pushed by the server and there is no
    // corresponding request message. Skip it from generating to avoid any confusion
    return Val == 0 ? this->NextRequestId() : Val;
}
