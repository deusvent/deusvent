#include "Connection.h"

#include "IWebSocket.h"
#include "WebSocketsModule.h"
#include "Modules/ModuleManager.h"

#include "Logging/StructuredLog.h"
#include "logic/logic.hpp"

DEFINE_LOG_CATEGORY(LogConnection);

UConnection::UConnection(const char *ServerAddress, const logic::Keys &Keys) {
    this->Address = ServerAddress;
    this->Keys = Keys;
    this->RequestId = 0;
    this->OnStop = FPlatformProcess::GetSynchEventFromPool(false);
    this->OnTriggerSending = FPlatformProcess::GetSynchEventFromPool(false);
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
        // TODO Let's imagine we get request_id from last byte of Message
        uint8 TempRequestId = 1;
        auto Callback = this->Callbacks[TempRequestId];
        Callback(Message);

        // Testing message tag and deserialization
        auto PrefixServerStatus = FString(logic::server_status_message_tag().c_str());
        auto PrefixDecay = FString(logic::decay_message_tag().c_str());
        auto ServerError = FString(logic::server_error_message_tag().c_str());
        if (Message.StartsWith(PrefixServerStatus)) {
            auto Deserialized = logic::ServerStatusSerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto ServerHealth = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received ServerHealth: {0}, ReqId={1}",
                      FString(Deserialized->debug_string().c_str()),
                      Deserialized->request_id());
            this->OnCommonServerInfo().Broadcast(Message);
        } else if (Message.StartsWith(PrefixDecay)) {
            auto Deserialized = logic::DecaySerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto Decay = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received Decay: {0}, ReqId={1}",
                      FString(Deserialized->debug_string().c_str()),
                      Deserialized->request_id());
        } else if (Message.StartsWith(ServerError)) {
            auto Deserialized = logic::ServerErrorSerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto Error = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received ServerError={0}. Debug={0}, ReqId={1}",
                      FString(Error.error_description.c_str()),
                      FString(Deserialized->debug_string().c_str()),
                      Deserialized->request_id());
        } else {
            UE_LOGFMT(LogConnection,
                      Display,
                      "Unknown message tag received: {0}, wanted prefix: {1}",
                      Message,
                      PrefixServerStatus);
        }
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

void UConnection::SendPing() {
    this->SendPublicMessage(logic::Ping::init());
}

void UConnection::SendDecayQuery() {
    this->SendPlayerMessage(logic::DecayQuery::init());
}

uint8 UConnection::NextRequestId() {
    auto Val = this->RequestId.fetch_add(1);
    // 0 is a special request id which API may return when request_id cannot be parsed from the
    // incoming message. Skip it from generating to avoid any confusion
    return Val == 0 ? this->NextRequestId() : Val;
}
