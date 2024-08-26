// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Pawn.h"
#include "Hero.generated.h"

UCLASS()
class DEUSVENT_API AHero : public APawn {
    GENERATED_BODY()

  public:
    AHero();
    void OnPong();
    void OnPong2();

  protected:
    virtual void BeginPlay() override;
};
