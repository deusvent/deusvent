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
    GameInstance->Connection->OnCommonServerInfo().AddLambda(
        [](FString Message) { UE_LOGFMT(LogTemp, Display, "OnServerInfo: {0}", Message); });

    GameInstance->Connection->SendPing();
}