#include "MainPlatformGameInstance.h"
#include "Connection.h"

void UMainPlatformGameInstance::Init() {
    Super::Init();
    // TODO Temp keys for now
    auto Keys = logic::generate_new_keys();
    // Pass this to ensure connection has access to game world and timer manager
    Connection = NewObject<UConnection>(this);
    Connection->Init("wss://api.deusvent.com", Keys);
}