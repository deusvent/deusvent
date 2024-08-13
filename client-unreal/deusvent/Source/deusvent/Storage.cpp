#include "Storage.h"

#include "Async/Async.h"
#include "Logging/StructuredLog.h"
#include "sqlite3.h"

DEFINE_LOG_CATEGORY(LogStorage);

void UStorage::Connect(const FString& DBName)
{
	auto DBPath = FPaths::Combine(FPaths::ProjectSavedDir(), DBName);
	UE_LOGFMT(LogStorage, Display, "Connecting to path {0}", DBPath);
	constexpr int Flags =
		SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_FULLMUTEX;
	if (sqlite3_open_v2(TCHAR_TO_UTF8(*DBPath), &DB, Flags, nullptr) !=
		SQLITE_OK)
	{
		const auto Error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
		UE_LOGFMT(LogStorage, Fatal, "Error opening database at {0}: {1}", DBPath,
		          *Error);
	}
	const auto TableCreate = R"(
        CREATE TABLE IF NOT EXISTS Items (
            Key TEXT PRIMARY KEY NOT NULL,
            Value TEXT NOT NULL
        );
    )";
	if (sqlite3_exec(DB, TableCreate, nullptr, nullptr, nullptr) != SQLITE_OK)
	{
		const auto Error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
		UE_LOGFMT(LogStorage, Fatal, "Error creating table: {0}", *Error);
	}
}

void UStorage::Disconnect()
{
	if (sqlite3_close(DB) != SQLITE_OK)
	{
		const auto Error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
		UE_LOGFMT(LogStorage, Fatal, "Error closing database: {0}", *Error);
	}
	DB = nullptr;
}

void UStorage::Clear() const
{
	UE_LOGFMT(LogStorage, Display, "Clearing the storage");
	const char* ClearQuery = "DELETE FROM Items;";
	if (sqlite3_exec(DB, ClearQuery, nullptr, nullptr, nullptr) !=
		SQLITE_OK)
	{
		const auto Error = FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get());
		UE_LOGFMT(LogStorage, Fatal, "Error clearing database: {0}", *Error);
	}
}

TFuture<int32> UStorage::ItemCount() const
{
	auto Promise = MakeShared<TPromise<int32>>();
	Async(EAsyncExecution::Thread, [this, Promise]()
	{
		int32 Count = 0;
		sqlite3_stmt* Statement = nullptr;
		if (sqlite3_prepare_v2(DB, "SELECT COUNT(*) FROM Items;", -1, &Statement,
		                       nullptr) == SQLITE_OK &&
			sqlite3_step(Statement) == SQLITE_ROW)
		{
			Count = sqlite3_column_int(Statement, 0);
		}
		else
		{
			UE_LOGFMT(LogStorage, Fatal, "Error counting rows: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_finalize(Statement);
		Promise->SetValue(Count);
	});
	return Promise->GetFuture();
}

TFuture<TOptional<FString>> UStorage::GetItem(const FString& Key) const
{
	auto Promise = MakeShared<TPromise<TOptional<FString>>>();
	Async(EAsyncExecution::Thread, [this, Promise, Key]()
	{
		sqlite3_stmt* Statement = nullptr;
		if (sqlite3_prepare_v2(DB, "SELECT Value FROM Items WHERE Key = ?;", -1,
		                       &Statement, nullptr) != SQLITE_OK)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error preparing get statement: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_bind_text(Statement, 1, TCHAR_TO_UTF8(*Key), -1, SQLITE_TRANSIENT);
		const auto StepResult = sqlite3_step(Statement);
		if (StepResult == SQLITE_ROW)
		{
			Promise->SetValue(
				FString(UTF8_TO_TCHAR(sqlite3_column_text(Statement, 0))));
		}
		else if (StepResult == SQLITE_DONE)
		{
			Promise->SetValue(TOptional<FString>());
		}
		else
		{
			UE_LOGFMT(LogStorage, Fatal, "Error retrieving item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_finalize(Statement);
	});
	return Promise->GetFuture();
}

TFuture<void> UStorage::SetItem(const FString& Key, const FString& Value) const
{
	UE_LOGFMT(LogStorage, Display, "Setting a value for the key {0}", Key);
	auto Promise = MakeShared<TPromise<void>>();
	Async(EAsyncExecution::Thread, [this, Promise, Key, Value]()
	{
		sqlite3_stmt* Statement = nullptr;
		if (sqlite3_prepare_v2(
			DB, "INSERT OR REPLACE INTO Items (Key, Value) VALUES (?, ?);", -1,
			&Statement, nullptr) != SQLITE_OK)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error saving item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_bind_text(Statement, 1, TCHAR_TO_UTF8(*Key), -1, SQLITE_TRANSIENT);
		sqlite3_bind_text(Statement, 2, TCHAR_TO_UTF8(*Value), -1,
		                  SQLITE_TRANSIENT);
		if (sqlite3_step(Statement) != SQLITE_DONE)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error saving item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_finalize(Statement);
		Promise->SetValue();
	});
	return Promise->GetFuture();
}

TFuture<void> UStorage::RemoveItem(const FString& Key) const
{
	UE_LOGFMT(LogStorage, Display, "Removing item for the key {0}", Key);
	auto Promise = MakeShared<TPromise<void>>();
	Async(EAsyncExecution::Thread, [this, Promise, Key]()
	{
		sqlite3_stmt* Statement = nullptr;
		const char* RemoveQuery = "DELETE FROM Items WHERE Key = ?;";
		if (sqlite3_prepare_v2(DB, RemoveQuery, -1, &Statement, nullptr) !=
			SQLITE_OK)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_bind_text(Statement, 1, TCHAR_TO_UTF8(*Key), -1, SQLITE_TRANSIENT);
		if (sqlite3_step(Statement) != SQLITE_DONE)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_finalize(Statement);
		Promise->SetValue();
	});
	return Promise->GetFuture();
}

TFuture<TArray<FString>> UStorage::Values(const FString& KeyPrefix) const
{
	auto Promise = MakeShared<TPromise<TArray<FString>>>();
	Async(EAsyncExecution::Thread, [this, Promise, KeyPrefix]()
	{
		TArray<FString> Values;
		sqlite3_stmt* Statement = nullptr;
		const char* Query =
			"SELECT Value from Items WHERE Key LIKE ? ORDER BY Key;";
		if (sqlite3_prepare_v2(DB, Query, -1, &Statement, nullptr) != SQLITE_OK)
		{
			UE_LOGFMT(LogStorage, Fatal, "Error removing item: {0}",
			          *FString(StringCast<TCHAR>(sqlite3_errmsg(DB)).Get()));
		}
		sqlite3_bind_text(Statement, 1, TCHAR_TO_UTF8(*(KeyPrefix + TEXT("%"))), -1,
		                  SQLITE_TRANSIENT);
		while (sqlite3_step(Statement) == SQLITE_ROW)
		{
			Values.Add(FString(UTF8_TO_TCHAR(sqlite3_column_text(Statement, 0))));
		}
		sqlite3_finalize(Statement);
		Promise->SetValue(MoveTemp(Values));
	});
	return Promise->GetFuture();
}
