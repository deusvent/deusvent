// Fill out your copyright notice in the Description page of Project Settings.

#include "Hero.h"
#include "logic/logic.hpp"

// Sets default values
AHero::AHero()
{
	// Set this pawn to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;
}

// Called when the game starts or when spawned
void AHero::BeginPlay()
{
	Super::BeginPlay();
	auto result = logic::add(11, 13);
	UE_LOG(LogTemp, Display, TEXT("Result=%u"), result);
}

// Called every frame
void AHero::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);
}

// Called to bind functionality to input
void AHero::SetupPlayerInputComponent(UInputComponent *PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);
}
