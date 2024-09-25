#include "Connection.h"

#include "IWebSocket.h"
#include "WebSocketsModule.h"
#include "Modules/ModuleManager.h"

#include "Logging/StructuredLog.h"
#include "logic/logic.hpp"

DEFINE_LOG_CATEGORY(LogConnection);

void UConnection::Initialize(const char *ServerAddress) {
    this->Address = ServerAddress;
    this->RequestId = 0;
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
        auto PrefixServerStatus = FString(logic::server_status_message_tag().c_str());
        auto PrefixDecay = FString(logic::decay_message_tag().c_str());
        if (Message.StartsWith(PrefixServerStatus)) {
            auto Deserialized = logic::ServerStatusSerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto ServerHealth = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received ServerHealth: {0}",
                      FString(Deserialized->debug_string().c_str()));
            this->OnCommonServerInfo().Broadcast(Message);
        } else if (Message.StartsWith(PrefixDecay)) {
            auto Deserialized = logic::DecaySerializer::deserialize(TCHAR_TO_UTF8(*Message));
            auto Decay = Deserialized->data();
            UE_LOGFMT(LogConnection,
                      Display,
                      "Received Decay: {0}",
                      FString(Deserialized->debug_string().c_str()));
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
    if (!Connection.IsValid()) {
        // TODO Implement proper re-connecting and queueing of messages
        UE_LOGFMT(LogConnection, Error, "Cannot send health message");
        return;
    }

    // Testing sending message using new serializers
    auto Msg = logic::Ping{};
    auto Serializer = logic::PingSerializer::init(Msg);

    auto Data = FString(Serializer->serialize(this->RequestId++).c_str());
    UE_LOGFMT(LogConnection, Display, "Sending Ping Data: {0}", Data);
    Connection->Send(Data);
}

// Testing sending signed authenticated message
void UConnection::SendDecayQuery() {
    auto Keys = logic::generate_new_keys();
    auto Msg = logic::DecayQuery{.unused = false};
    auto Serializer = logic::DecayQuerySerializer::init(Msg, Keys.public_key);
    auto Data = FString(Serializer->serialize(this->RequestId++, Keys.private_key).c_str());
    UE_LOGFMT(LogConnection, Display, "Sending Query Data: {0}", Data);
    Connection->Send(Data);
}
