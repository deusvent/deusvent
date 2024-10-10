#pragma once

#include "CoreMinimal.h"
#include "IWebSocket.h"
#include "logic/logic.hpp"

DECLARE_LOG_CATEGORY_EXTERN(LogConnection, Log, All);

class IResponseCallback {
  public:
    virtual ~IResponseCallback() = default;
    virtual void HandleResponse(const FString &Data) = 0;
    virtual void SetError(const logic::ServerError &Error) = 0;
};

template <typename B> class ResponseCallback : public IResponseCallback {
  public:
    TPromise<TVariant<B, logic::ServerError>> Promise;

    virtual void HandleResponse(const FString &Data) override {
        // B ResponseMessage = B::Deserialize(Data);
        // Promise.SetValue(TVariant(ResponseMessage));
    }

    virtual void SetError(const logic::ServerError &Error) override {
        TVariant<logic::ServerStatus, logic::ServerError> Result(TInPlaceType<logic::ServerError>(),
                                                                 Error);
        Promise.SetValue(Result);
    }
};

// TODO It's needed to be a UClass to ensure GC works as intended
// Creates a websocket connection with the backend API
class DEUSVENT_API UConnection {

  public:
    UConnection(const char *ServerAddress, const logic::Keys &Keys);
    void Connect();
    void Disconnect();
    void SendPing();
    void SendDecayQuery();

    DECLARE_EVENT_OneParam(UConnection, FCommonServerInfo, FString /* message::common.ServerInfo */)
        FCommonServerInfo &OnCommonServerInfo() {
        return CommonServerInfo;
    }

    template <typename MessageType> void SendPublicMessage(const MessageType &Msg);

    template <typename MessageType> void SendPlayerMessage(const MessageType &Msg);

    template <typename ResponseType, typename MessageType>
    TFuture<TVariant<std::shared_ptr<ResponseType>, logic::ServerError>>
    SendPublicMessageAsync(const MessageType &Msg);

    // // Subscribe to the incoming server message type O. Callback may be called multiple times,
    // once
    // // for each incoming message of the specified type
    // template <typename O> void OnMessageReceived(TWeakPtr<TFunction<void(const O &)>> OnMessage)

  private:
    FCommonServerInfo CommonServerInfo;
    const char *Address;

    logic::Keys Keys;
    TSharedPtr<IWebSocket> Connection;
    // Request id to callbacks - to map server messages responses to sent client messages
    TMap<uint8, TFunction<void(FString)>> Callbacks;
    // Message prefix to callbacks - to map server messages to subscribers
    TMap<FString, TFunction<void(FString)>> Subscribers;
    std::atomic<uint8> RequestId;
    FEvent *OnStop;
    FEvent *OnTriggerSending;
    FDateTime LastHealthCheck;

    uint8 NextRequestId();
};

template <typename MessageType> void UConnection::SendPublicMessage(const MessageType &Msg) {
    auto Data = FString(Msg->serialize(this->NextRequestId()).c_str());
    this->Connection->Send(Data);
}

template <typename MessageType> void UConnection::SendPlayerMessage(const MessageType &Msg) {
    auto Data = FString(Msg->serialize(this->NextRequestId(), this->Keys).c_str());
    this->Connection->Send(Data);
}
template <typename ResponseType, typename MessageType>
TFuture<TVariant<std::shared_ptr<ResponseType>, logic::ServerError>>
UConnection::SendPublicMessageAsync(const MessageType &Msg) {
    auto Promise = TPromise<TVariant<std::shared_ptr<ResponseType>, logic::ServerError>>();
    this->Callbacks.Add(1, [&Promise](const FString &Data) {
        // auto Msg = logic::ServerError{};
        // auto Response =
        //     TVariant<ResponseType, logic::ServerError>(TInPlaceType<logic::ServerError>(), Msg);
        auto Msg = ResponseType::deserialize(TCHAR_TO_UTF8(*Data));
        auto Response = TVariant<std::shared_ptr<ResponseType>, logic::ServerError>(
            TInPlaceType<std::shared_ptr<ResponseType>>(), Msg);
        Promise.SetValue(Response);
    });
    return Promise.GetFuture();
}
