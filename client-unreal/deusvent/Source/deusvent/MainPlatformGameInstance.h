#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintPlatformLibrary.h"
#include "Connection.h"
#include "MainPlatformGameInstance.generated.h"

UCLASS()
class DEUSVENT_API UMainPlatformGameInstance : public UPlatformGameInstance {
    GENERATED_BODY()

  public:
    virtual void Init() override;
    UPROPERTY()
    UConnection *Connection;
};