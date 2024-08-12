#include "Storage.h"

#include "Async/Async.h"
#include "Logging/StructuredLog.h"
#include "sqlite3.h"

DEFINE_LOG_CATEGORY(LogStorage);

void UStorage::Connect(const FString& dbName) {
  UE_LOGFMT(LogStorage, Display, "Connecting to {0}", dbName);
  auto dbPath = FPaths::Combine(FPaths::ProjectSavedDir(), dbName);
  int flags =
      SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_FULLMUTEX;
  if (sqlite3_open_v2(TCHAR_TO_UTF8(*dbPath), &DB, flags, nullptr) !=
      SQLITE_OK) {
    auto error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
    UE_LOGFMT(LogStorage, Fatal, "Error opening database at {0}: {1}", dbPath,
              *error);
  }
  auto tableCreate = R"(
        CREATE TABLE IF NOT EXISTS Items (
            Key TEXT PRIMARY KEY NOT NULL,
            Value TEXT NOT NULL
        );
    )";
  if (sqlite3_exec(DB, tableCreate, nullptr, nullptr, nullptr) != SQLITE_OK) {
    auto error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
    UE_LOGFMT(LogStorage, Fatal, "Error creating table: {0}", *error);
  }
  UE_LOGFMT(LogStorage, Display, "Connected to {0}", dbPath);
}

void UStorage::Close() {
  if (sqlite3_close(DB) != SQLITE_OK) {
    auto error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
    UE_LOGFMT(LogStorage, Fatal, "Error closing database: {0}", *error);
  }
}

void UStorage::Clear() {
  const char* ClearQuery = "DELETE FROM Items;";
  if (sqlite3_exec(DB, ClearQuery, nullptr, nullptr, nullptr) != SQLITE_OK) {
    auto error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
    UE_LOGFMT(LogStorage, Fatal, "Error clearing database: {0}", *error);
  }
}

TFuture<int32> UStorage::ItemCount() {
  auto promise = MakeShared<TPromise<int32>>();
  Async(EAsyncExecution::Thread, [this, promise]() {
    int32 count = 0;
    sqlite3_stmt* statement = nullptr;
    if (sqlite3_prepare_v2(DB, "SELECT COUNT(*) FROM Items;", -1, &statement,
                           nullptr) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
      count = sqlite3_column_int(statement, 0);
    } else {
      UE_LOGFMT(LogStorage, Fatal, "Error counting rows: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_finalize(statement);
    promise->SetValue(count);
  });
  return promise->GetFuture();
}

TFuture<TOptional<FString>> UStorage::GetItem(const FString& key) {
  auto promise = MakeShared<TPromise<TOptional<FString>>>();
  Async(EAsyncExecution::Thread, [this, promise, key]() {
    sqlite3_stmt* statement = nullptr;
    if (sqlite3_prepare_v2(DB, "SELECT Value FROM Items WHERE Key = ?;", -1,
                           &statement, nullptr) != SQLITE_OK) {
      UE_LOGFMT(LogStorage, Fatal, "Error preparing get statement: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_bind_text(statement, 1, TCHAR_TO_UTF8(*key), -1, SQLITE_TRANSIENT);
    auto step_result = sqlite3_step(statement);
    if (step_result == SQLITE_ROW) {
      promise->SetValue(
          FString(UTF8_TO_TCHAR(sqlite3_column_text(statement, 0))));
    } else if (step_result == SQLITE_DONE) {
      promise->SetValue(TOptional<FString>());
    } else {
      UE_LOGFMT(LogStorage, Fatal, "Error retrieving item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_finalize(statement);
  });
  return promise->GetFuture();
}

TFuture<void> UStorage::SetItem(const FString& key, const FString& value) {
  auto promise = MakeShared<TPromise<void>>();
  Async(EAsyncExecution::Thread, [this, promise, key, value]() {
    sqlite3_stmt* statement = nullptr;
    if (sqlite3_prepare_v2(
            DB, "INSERT OR REPLACE INTO Items (Key, Value) VALUES (?, ?);", -1,
            &statement, nullptr) != SQLITE_OK) {
      UE_LOGFMT(LogStorage, Fatal, "Error saving item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_bind_text(statement, 1, TCHAR_TO_UTF8(*key), -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 2, TCHAR_TO_UTF8(*value), -1,
                      SQLITE_TRANSIENT);
    if (sqlite3_step(statement) != SQLITE_DONE) {
      UE_LOGFMT(LogStorage, Fatal, "Error saving item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_finalize(statement);
    promise->SetValue();
  });
  return promise->GetFuture();
}

TFuture<void> UStorage::RemoveItem(const FString& key) {
  auto promise = MakeShared<TPromise<void>>();
  Async(EAsyncExecution::Thread, [this, promise, key]() {
    sqlite3_stmt* statement = nullptr;
    const char* removeQuery = "DELETE FROM Items WHERE Key = ?;";
    if (sqlite3_prepare_v2(DB, removeQuery, -1, &statement, nullptr) !=
        SQLITE_OK) {
      UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_bind_text(statement, 1, TCHAR_TO_UTF8(*key), -1, SQLITE_TRANSIENT);
    if (sqlite3_step(statement) != SQLITE_DONE) {
      UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_finalize(statement);
    promise->SetValue();
  });
  return promise->GetFuture();
}

TFuture<TArray<FString>> UStorage::Values(const FString& keyPrefix) {
  auto promise = MakeShared<TPromise<TArray<FString>>>();
  Async(EAsyncExecution::Thread, [this, promise, keyPrefix]() {
    TArray<FString> values;
    sqlite3_stmt* statement = nullptr;
    const char* query =
        "SELECT Value from Items WHERE Key LIKE ? ORDER BY Key;";
    if (sqlite3_prepare_v2(DB, query, -1, &statement, nullptr) != SQLITE_OK) {
      UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
                *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
    }
    sqlite3_bind_text(statement, 1, TCHAR_TO_UTF8(*(keyPrefix + TEXT("%"))), -1,
                      SQLITE_TRANSIENT);
    while (sqlite3_step(statement) == SQLITE_ROW) {
      values.Add(FString(UTF8_TO_TCHAR(sqlite3_column_text(statement, 0))));
    }
    sqlite3_finalize(statement);
    promise->SetValue(MoveTemp(values));
  });
  return promise->GetFuture();
}
