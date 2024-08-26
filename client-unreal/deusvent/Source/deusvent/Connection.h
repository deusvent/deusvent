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
    void SendHealth() const;

    DECLARE_EVENT(UConnection, FPongEvent)
    FPongEvent &OnPong() {
        return PongEvent;
    }

  private:
    FPongEvent PongEvent;
    const char *Address;
    TSharedPtr<IWebSocket> Connection;
};
