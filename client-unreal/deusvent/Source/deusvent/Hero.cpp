#include "Hero.h"
#include "Kismet/GameplayStatics.h"
#include "MainPlatformGameInstance.h"
#include "Logging/StructuredLog.h"
#include "logic/logic.hpp"

AHero::AHero() {
    PrimaryActorTick.bCanEverTick = true;
}

void AHero::BeginPlay() {
    Super::BeginPlay();
    // Testing of Rust integration
    const auto TimestampSending = logic::Timestamp::now();

    // Testing of WebSocket connection
    const auto GameInstance =
        Cast<UMainPlatformGameInstance>(UGameplayStatics::GetGameInstance(GetWorld()));

    GameInstance->Connection->SendPublicMessage<logic::ServerStatus>(logic::Ping::init())
        .Next([](std::variant<std::shared_ptr<logic::ServerStatus>,
                              std::shared_ptr<logic::ServerError>> Response) {
            if (auto ServerStatus = std::get_if<std::shared_ptr<logic::ServerStatus>>(&Response)) {
                UE_LOGFMT(LogTemp,
                          Display,
                          "Got server info: {0}",
                          FString(ServerStatus->get()->debug_string().c_str()));
            }
        });
}