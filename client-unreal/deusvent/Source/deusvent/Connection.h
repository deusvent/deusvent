#pragma once

#include "CoreMinimal.h"
#include "IWebSocket.h"
#include "logic/logic.hpp"
#include "Connection.generated.h"

DECLARE_LOG_CATEGORY_EXTERN(LogConnection, Log, All);

// Creates a websocket connection with the backend API
UCLASS()
class DEUSVENT_API UConnection : public UObject {

    GENERATED_BODY()

  public:
    void Init(const char *ServerAddress, const logic::Keys &PlayerKeys);
    void Disconnect();

    template <typename ResponseType, typename MessageType>
    TFuture<std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>>
    SendPublicMessage(const MessageType &Msg);

  private:
    void TryToSendMessages();

    // Try to reconnect to the websocket after the specified delay
    void Reconnect(float DelaySeconds);

    // Request id to callbacks - to map server messages responses to sent client messages
    TMap<uint8, TFunction<void(FString)>> Callbacks;

    // Queue for all outgoing messages - contains serialized payload to be sent
    TQueue<FString> OutgoingMessages;

    logic::Keys Keys;
    TSharedPtr<IWebSocket> Connection;
    std::atomic<uint8> RequestId;
    FDateTime LastHealthCheck;
    FTimerHandle ReconnectTimerHandle;

    uint8 NextRequestId();
};

template <typename ResponseType, typename MessageType>
TFuture<std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>>
UConnection::SendPublicMessage(const MessageType &Msg) {
    // Generate next requestId and queue message to be sent
    auto MsgRequestId = this->NextRequestId();
    auto MessageData = FString(UTF8_TO_TCHAR(Msg->serialize(MsgRequestId).c_str()));
    this->OutgoingMessages.Enqueue(MessageData);

    // Register callback function when server message with same requestId would come
    TSharedPtr<
        TPromise<std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>>>
        Promise = MakeShared<TPromise<
            std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>>>();

    this->Callbacks.Add(MsgRequestId, [Promise](const FString &Data) {
        if (Data.StartsWith(FString(logic::server_error_message_tag().c_str()))) {
            auto Error = logic::ServerError::deserialize(TCHAR_TO_UTF8(*Data));
            auto Response =
                std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>(
                    Error);
            Promise->SetValue(Response);
        } else {
            auto Msg = ResponseType::deserialize(TCHAR_TO_UTF8(*Data));
            auto Response =
                std::variant<std::shared_ptr<ResponseType>, std::shared_ptr<logic::ServerError>>(
                    Msg);
            Promise->SetValue(Response);
        }
    });

    // Trigger message sending if connection is ready
    this->TryToSendMessages();

    // Return promise which will be resolved once we got a server message with right requestId
    return Promise->GetFuture();
}
