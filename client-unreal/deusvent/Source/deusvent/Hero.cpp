#include "Hero.h"
#include "Kismet/GameplayStatics.h"
#include "MainPlatformGameInstance.h"
#include "logic/logic.hpp"

AHero::AHero() {
    PrimaryActorTick.bCanEverTick = true;
}

void AHero::BeginPlay() {
    Super::BeginPlay();
    auto result = logic::add(11, 13);
    UE_LOG(LogTemp, Display, TEXT("Result=%u"), result);

    // TODO Temp testing of websocket connection
    auto gameInstance =
        Cast<UMainPlatformGameInstance>(UGameplayStatics::GetGameInstance(GetWorld()));
    gameInstance->connection->OnPong().AddUObject(this, &AHero::OnPong);
    gameInstance->connection->OnPong().AddUObject(this, &AHero::OnPong2);
    gameInstance->connection->SendHealth();
}

void AHero::OnPong() {
    UE_LOG(LogTemp, Display, TEXT("AHero::OnPong"));
}

void AHero::OnPong2() {
    UE_LOG(LogTemp, Display, TEXT("AHero::OnPong2"));
}

// Called every frame
void AHero::Tick(float DeltaTime) {
    Super::Tick(DeltaTime);
}

// Called to bind functionality to input
void AHero::SetupPlayerInputComponent(UInputComponent *PlayerInputComponent) {
    Super::SetupPlayerInputComponent(PlayerInputComponent);
}
