#include "Storage.h"

#include "Async/Async.h"
#include "Logging/StructuredLog.h"
#include "SQLiteDatabase.h"

DEFINE_LOG_CATEGORY(LogStorage);

void PrepareSQLiteStatement(FSQLiteDatabase *DB,
                            const char *SQL,
                            FSQLitePreparedStatement *&Statement) {
    Statement = new FSQLitePreparedStatement();
    auto SQLQuery = FString(UTF8_TO_TCHAR(SQL));
    if (!Statement->Create(*DB, *SQLQuery, ESQLitePreparedStatementFlags::Persistent)) {
        UE_LOGFMT(
            LogStorage, Fatal, "Error preparing statement {0}: {1}", *SQL, *DB->GetLastError());
    }
}

void UStorage::Connect(const FString &DBName) {
    auto DBPath = FPaths::Combine(FPaths::ProjectSavedDir(), DBName);
    UE_LOGFMT(LogStorage, Display, "Connecting to DB: {0}", DBPath);
    DB = new FSQLiteDatabase();
    if (!DB->Open(*DBPath, ESQLiteDatabaseOpenMode::ReadWriteCreate)) {
        UE_LOGFMT(
            LogStorage, Fatal, "Error opening database at {0}: {1}", DBPath, *DB->GetLastError());
    }
    const auto CreateTableSQL = TEXT(R"(
        CREATE TABLE IF NOT EXISTS Items (
            Key TEXT PRIMARY KEY NOT NULL,
            Value TEXT NOT NULL
        );
    )");
    auto CreateTable = new FSQLitePreparedStatement();
    if (!CreateTable->Create(*DB, CreateTableSQL)) {
        UE_LOGFMT(LogStorage,
                  Fatal,
                  "Error preparing create table statement at {0}: {1}",
                  DBPath,
                  *DB->GetLastError());
    }
    if (!CreateTable->Execute()) {
        UE_LOGFMT(LogStorage,
                  Fatal,
                  "Error executing creating table statement {0}: {1}",
                  DBPath,
                  *DB->GetLastError());
    }

    // Pre-create all the statements
    PrepareSQLiteStatement(DB, "DELETE FROM Items", StatementClear);
    PrepareSQLiteStatement(DB, "SELECT COUNT(*) FROM Items", StatementItemCount);
    PrepareSQLiteStatement(DB, "SELECT Value FROM Items WHERE Key = ?", StatementGetItem);
    PrepareSQLiteStatement(
        DB, "INSERT OR REPLACE INTO Items (Key, Value) VALUES (?, ?)", StatementSetItem);
    PrepareSQLiteStatement(DB, "DELETE FROM Items WHERE Key = ?", StatementRemoveItem);
    PrepareSQLiteStatement(
        DB, "SELECT Value from Items WHERE Key LIKE ? ORDER BY Key", StatementValues);
}

void UStorage::Disconnect() {
    FScopeLock Lock(&ConnectionLock);
    if (!DB->Close()) {
        UE_LOGFMT(LogStorage, Fatal, "Error closing database: {0}", *DB->GetLastError());
    }
    DB = nullptr;
}

void UStorage::Clear() {
    FScopeLock Lock(&ConnectionLock);
    const auto Statement = StatementClear;
    Statement->Reset();
    UE_LOGFMT(LogStorage, Display, "Clearing the storage");
    if (!Statement->Execute()) {
        UE_LOGFMT(LogStorage, Fatal, "Error clearing database: {0}", *DB->GetLastError());
    }
}

TFuture<int32> UStorage::ItemCount() {
    auto Promise = MakeShared<TPromise<int32>>();
    Async(EAsyncExecution::Thread, [this, Promise]() {
        FScopeLock Lock(&ConnectionLock);
        const auto Statement = StatementItemCount;
        Statement->Reset();
        int32 Count = 0;
        if (Statement->Step() == ESQLitePreparedStatementStepResult::Row &&
            Statement->GetColumnValueByIndex(0, Count)) {
            // Count is set
        } else {
            UE_LOGFMT(LogStorage, Fatal, "Error getting item count: {0}", *DB->GetLastError());
        }
        Promise->SetValue(Count);
    });
    return Promise->GetFuture();
}

TFuture<TOptional<FString>> UStorage::GetItem(const FString &Key) {
    auto Promise = MakeShared<TPromise<TOptional<FString>>>();
    Async(EAsyncExecution::Thread, [this, Promise, Key]() {
        FScopeLock Lock(&ConnectionLock);
        const auto Statement = StatementGetItem;
        Statement->Reset();
        if (!Statement->SetBindingValueByIndex(1, Key)) {
            UE_LOGFMT(
                LogStorage, Fatal, "Error binding for getting item: {0}", *DB->GetLastError());
        }
        const auto StepResult = Statement->Step();
        if (StepResult == ESQLitePreparedStatementStepResult::Row) {
            FString Value;
            Statement->GetColumnValueByIndex(0, Value);
            Promise->SetValue(TOptional<FString>(Value));
        } else if (StepResult == ESQLitePreparedStatementStepResult::Done) {
            Promise->SetValue(TOptional<FString>());
        } else {
            UE_LOGFMT(LogStorage, Fatal, "Error retrieving item: {0}", *DB->GetLastError());
            Promise->SetValue(TOptional<FString>());
        }
    });
    return Promise->GetFuture();
}

TFuture<void> UStorage::SetItem(const FString &Key, const FString &Value) {
    UE_LOGFMT(LogStorage, Display, "Setting a value for the key {0}", Key);
    auto Promise = MakeShared<TPromise<void>>();
    Async(EAsyncExecution::Thread, [this, Promise, Key, Value]() {
        FScopeLock Lock(&ConnectionLock);
        const auto Statement = StatementSetItem;
        Statement->Reset();
        if (!Statement->SetBindingValueByIndex(1, Key) ||
            !Statement->SetBindingValueByIndex(2, Value)) {
            UE_LOGFMT(
                LogStorage, Fatal, "Error binding for setting item: {0}", *DB->GetLastError());
        }
        if (Statement->Step() != ESQLitePreparedStatementStepResult::Done) {
            UE_LOGFMT(LogStorage, Fatal, "Error saving item: {0}", *DB->GetLastError());
        }
        Promise->SetValue();
    });
    return Promise->GetFuture();
}

TFuture<void> UStorage::RemoveItem(const FString &Key) {
    UE_LOGFMT(LogStorage, Display, "Removing item for the key {0}", Key);
    auto Promise = MakeShared<TPromise<void>>();
    Async(EAsyncExecution::Thread, [this, Promise, Key]() {
        FScopeLock Lock(&ConnectionLock);
        const auto Statement = StatementRemoveItem;
        Statement->Reset();
        if (!Statement->SetBindingValueByIndex(1, Key)) {
            UE_LOGFMT(
                LogStorage, Fatal, "Error binding for removing item: {0}", *DB->GetLastError());
        }
        if (Statement->Step() != ESQLitePreparedStatementStepResult::Done) {
            UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}", *DB->GetLastError());
        }
        Promise->SetValue();
    });
    return Promise->GetFuture();
}

// TODO Probably we need to return a pair of key and a value
// TODO It's O(n) for the memory consumption, we can use callback style instead
TFuture<TArray<FString>> UStorage::Values(const FString &KeyPrefix) {
    auto Promise = MakeShared<TPromise<TArray<FString>>>();
    Async(EAsyncExecution::Thread, [this, Promise, KeyPrefix]() {
        FScopeLock Lock(&ConnectionLock);
        const auto Statement = StatementValues;
        Statement->Reset();
        TArray<FString> Values;
        if (!Statement->SetBindingValueByIndex(1, KeyPrefix + TEXT("%"))) {
            UE_LOGFMT(
                LogStorage, Fatal, "Error binding for finding values: {0}", *DB->GetLastError());
        }
        auto Result = Statement->Execute([&Values](const FSQLitePreparedStatement &Row)
                                             -> ESQLitePreparedStatementExecuteRowResult {
            FString Value;
            if (Row.GetColumnValueByIndex(0, Value)) {
                Values.Add(Value);
            } else {
                return ESQLitePreparedStatementExecuteRowResult::Error;
            }
            return ESQLitePreparedStatementExecuteRowResult::Continue;
        });
        if (Result == INDEX_NONE) {
            UE_LOGFMT(
                LogStorage, Fatal, "Error iterating for finding values: {0}", *DB->GetLastError());
        }
        Promise->SetValue(MoveTemp(Values));
    });
    return Promise->GetFuture();
}
