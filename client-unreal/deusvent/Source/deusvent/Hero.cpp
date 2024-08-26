#include "Hero.h"
#include "Kismet/GameplayStatics.h"
#include "MainPlatformGameInstance.h"
#include "logic/logic.hpp"

AHero::AHero() {
    PrimaryActorTick.bCanEverTick = true;
}

void AHero::BeginPlay() {
    Super::BeginPlay();
    // Testing of Rust integration
    const auto Result = logic::add(11, 13);
    UE_LOG(LogTemp, Display, TEXT("Result=%u"), Result);

    // Testing of WebSocket connection
    const auto GameInstance =
        Cast<UMainPlatformGameInstance>(UGameplayStatics::GetGameInstance(GetWorld()));
    GameInstance->Connection->OnPong().AddUObject(this, &AHero::OnPong);
    GameInstance->Connection->OnPong().AddUObject(this, &AHero::OnPong2);
    GameInstance->Connection->SendHealth();
}

void AHero::OnPong() {
    UE_LOG(LogTemp, Display, TEXT("AHero::OnPong"));
}

void AHero::OnPong2() {
    UE_LOG(LogTemp, Display, TEXT("AHero::OnPong2"));
}