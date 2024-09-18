#include "Connection.h"

#include "IWebSocket.h"
#include "WebSocketsModule.h"
#include "Modules/ModuleManager.h"

#include "Logging/StructuredLog.h"
#include "logic/logic.hpp"

DEFINE_LOG_CATEGORY(LogConnection);

void UConnection::Initialize(const char *ServerAddress) {
    this->Address = ServerAddress;
}

void UConnection::Connect() {

    // HACK WebSocket fails to connect if there is no path/query in the URL. I assume it's related
    // to this one https://github.com/warmcat/libwebsockets/issues/414. If logs enabled via "Log
    // LogWebSockets Verbose" in the console then LogWebSockets will log an error
    // LWS_CALLBACK_CLIENT_CONNECTION_ERROR
    FString ServerURL = TEXT("wss://api.deusvent.com/?ws");

    UE_LOGFMT(LogConnection, Display, "Connecting to {0}", ServerURL);
    Connection = FWebSocketsModule::Get().CreateWebSocket(ServerURL);

    Connection->OnConnected().AddLambda([]() { UE_LOGFMT(LogConnection, Display, "Connected"); });

    Connection->OnConnectionError().AddLambda([](const FString &Error) {
        UE_LOGFMT(LogConnection, Display, "Connection error: {0}", Error);
    });

    Connection->OnClosed().AddLambda([](int32 StatusCode, const FString &Reason, bool bWasClean) {
        UE_LOGFMT(LogConnection,
                  Display,
                  "Connection closed: Status={0}, Reason={0}",
                  StatusCode,
                  Reason);
    });

    Connection->OnMessage().AddLambda([this](const FString &Message) {
        // Testing message tag and deserialization
        auto Prefix = FString(logic::server_status_message_tag().c_str());
        if (Message.StartsWith(Prefix)) {
            auto Deserialized = logic::ServerStatusSerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto ServerHealth = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received ServerHealth: {0}",
                      FString(Deserialized->debug_string().c_str()));
        } else {
            UE_LOGFMT(LogConnection,
                      Display,
                      "Unknown message tag received: {0}, wanted prefix: {1}",
                      Message,
                      Prefix);
        }

        this->OnCommonServerInfo().Broadcast(Message);
    });

    Connection->OnMessageSent().AddLambda([](const FString &Message) {
        UE_LOGFMT(LogConnection, Display, "Message sent: {0}", Message);
    });

    Connection->Connect();
}

void UConnection::Disconnect() {
    if (!Connection.IsValid()) {
        UE_LOGFMT(LogConnection, Display, "No active WebSocket connection to disconnect");
        return;
    }
    Connection->Close();
    Connection.Reset();
    UE_LOGFMT(LogConnection, Display, "Disconnected");
}

void UConnection::SendPing() const {
    if (!Connection.IsValid()) {
        // TODO Implement proper re-connecting and queueing of messages
        UE_LOGFMT(LogConnection, Error, "Cannot send health message");
        return;
    }

    // Testing sending message using new serializers
    auto Data = logic::Ping{.ts = logic::Timestamp::now()};
    auto Serializer = logic::PingSerializer::init(Data);
    auto Msg = FString(Serializer->serialize().c_str());
    UE_LOGFMT(LogConnection, Display, "Sending Ping Msg: {0}", Msg);
    Connection->Send(Msg);
}
