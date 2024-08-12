#include "MainGameMode.h"

#include "Logging/StructuredLog.h"
#include "Storage.h"

DEFINE_LOG_CATEGORY(LogMainGameMode);

// TODO Storage usage reference and tests. To be removed once we figure out how unit tests works
void AMainGameMode::InitGame(const FString& MapName, const FString& Options,
                             FString& ErrorMessage) {
  Super::InitGame(MapName, Options, ErrorMessage);
  auto storage = NewObject<UStorage>(this, UStorage::StaticClass());
  storage->Connect(TEXT("testdb.sqlite"));

  storage->ItemCount().Next([](int32 count) {
    UE_LOGFMT(LogMainGameMode, Display, "Got an itemCount={0}", count);
  });

  storage->GetItem(TEXT("foo1")).Next([](TOptional<FString> value) {
    if (value.IsSet()) {
      UE_LOGFMT(LogMainGameMode, Display, "Value for key foo1={0}",
                value.GetValue());
    } else {
      UE_LOGFMT(LogMainGameMode, Display, "No value for key foo1");
    }
  });

  storage->GetItem(TEXT("foo3")).Next([](TOptional<FString> value) {
    if (value.IsSet()) {
      UE_LOGFMT(LogMainGameMode, Display, "Value for key foo3={0}",
                value.GetValue());
    } else {
      UE_LOGFMT(LogMainGameMode, Display, "No value for key foo3");
    }
  });

  storage->SetItem(TEXT("foo2"), TEXT("BAR2")).Next([](int /*unused*/) {
    UE_LOGFMT(LogMainGameMode, Display, "Saved a key foo2");
  });

  storage->SetItem(TEXT("foo3"), TEXT("BAR3")).Next([](int /*unused*/) {
    UE_LOGFMT(LogMainGameMode, Display, "Saved a key foo3");
  });

  storage->RemoveItem(TEXT("foo1")).Next([](int /*unused*/) {
    UE_LOGFMT(LogMainGameMode, Display, "Deleted a key foo1");
  });

  storage->SetItem(TEXT("foo3"), TEXT("BAR4")).Next([](int /*unused*/) {
    UE_LOGFMT(LogMainGameMode, Display, "Saved a key foo3");
  });

  storage->Values(TEXT("foo")).Next([](TArray<FString> values) {
    UE_LOGFMT(LogMainGameMode, Display, "Values={0}", values);
  });
}
