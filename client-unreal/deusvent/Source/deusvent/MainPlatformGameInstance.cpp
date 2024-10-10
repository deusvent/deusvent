#include "MainPlatformGameInstance.h"
#include "Connection.h"

void UMainPlatformGameInstance::Init() {
    Super::Init();
    // TODO Temp keys for now
    auto Keys = logic::generate_new_keys();
    Connection = new UConnection("https://api.deusvent.com", Keys);
    Connection->Connect();
}