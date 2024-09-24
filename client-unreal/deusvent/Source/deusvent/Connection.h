#pragma once

#include "CoreMinimal.h"
#include "IWebSocket.h"
#include "Connection.generated.h"

DECLARE_LOG_CATEGORY_EXTERN(LogConnection, Log, All);

// Creates a websocket connection with the backend API
UCLASS()
class DEUSVENT_API UConnection : public UObject {
    GENERATED_BODY()

  public:
    void Initialize(const char *ServerAddress);
    void Connect();
    void Disconnect();
    void SendPing() const;
    void SendDecayQuery() const;

    DECLARE_EVENT_OneParam(UConnection, FCommonServerInfo, FString /* message::common.ServerInfo */)
        FCommonServerInfo &OnCommonServerInfo() {
        return CommonServerInfo;
    }

  private:
    FCommonServerInfo CommonServerInfo;
    const char *Address;
    TSharedPtr<IWebSocket> Connection;
};
