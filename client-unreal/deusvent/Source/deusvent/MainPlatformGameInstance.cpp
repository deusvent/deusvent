#include "MainPlatformGameInstance.h"
#include "Connection.h"

void UMainPlatformGameInstance::Init() {
    Super::Init();
    Connection = NewObject<UConnection>(this, UConnection::StaticClass());
    Connection->Initialize("https://api.deusvent.com");
    Connection->Connect();
}