#pragma once

#include "CoreMinimal.h"
#include "Storage.generated.h"

DECLARE_LOG_CATEGORY_EXTERN(LogStorage, Log, All);

/**
 * Key/Value persistent storage based on sqlite3.
 *
 * UE_LOGFMT(Fatal, ...) is used for all errors that should only occur during
 * development; in runtime, it should never fail. The only error that may occur
 * in runtime is "disk full," but it will likely cause broader system failures.
 */
UCLASS()
class DEUSVENT_API UStorage : public UObject {
    GENERATED_BODY()

  public:
    // Opens or creates a database for a given name
    void Connect(const FString &DBName);

    // Closes a database connection
    void Disconnect();

    // Remove all the key/values from the database
    void Clear() const;

    // Returns number of keys in the database
    TFuture<int32> ItemCount() const;

    // Return and optional value for the given key
    TFuture<TOptional<FString>> GetItem(const FString &Key) const;

    // Saves the value for the given key
    // HACK: TFuture<void> is inherited from TFutureBase<int>, you need
    // to use int parameter in your callback .Next([](int /*unused*/) { })
    TFuture<void> SetItem(const FString &Key, const FString &Value) const;

    // Ensures that the row with the specified key no longer exists in the
    // database
    TFuture<void> RemoveItem(const FString &Key) const;

    // Returns array of values with keys that starts with the given prefix which
    // may be empty. Results are returned sorted by the key
    TFuture<TArray<FString>> Values(const FString &KeyPrefix) const;

  private:
    struct sqlite3 *DB;
};
