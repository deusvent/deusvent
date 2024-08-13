#if !UE_BUILD_SHIPPING

#include "deusvent/Storage.h"
#include "Tests/TestHarnessAdapter.h"

void NewStorage(const bool ClearDB, const FString& Name, const TFunction<void(UStorage*)>& Callback)
{
	const auto Storage = NewObject<UStorage>();
	Storage->Connect(Name + ".unittest.sqlite");
	if (ClearDB)
	{
		Storage->Clear();
	}
	Callback(Storage);
	Storage->Disconnect();
}

TEST_CASE("Deusvent.Storage", "[unit]")
{
	SECTION("Connect->Disconnect")
	{
		NewStorage(true, "test", [](UStorage* _)
		{
			// Disconnect will be executed automatically
		});
	}

	SECTION("GetItem->SetItem")
	{
		NewStorage(true, "test", [this](const UStorage* DB)
		{
			const auto Key = TEXT("key");
			const auto Val = TEXT("val");
			TestFalse("Initial value should not exists", DB->GetItem(Key).Get().IsSet());
			DB->SetItem(Key, Val).Wait();
			const auto Got = DB->GetItem(Key).Get().GetValue();
			TestEqual("Value should be updated", Got, Val);
		});
	}

	SECTION("ItemCount")
	{
		NewStorage(true, "test", [this](const UStorage* DB)
		{
			TestEqual("Initial item count should be 0", DB->ItemCount().Get(), 0);
			DB->SetItem(TEXT("key1"), TEXT("val2")).Wait();
			TestEqual("One row added", DB->ItemCount().Get(), 1);
			DB->SetItem(TEXT("key2"), TEXT("val2")).Wait();
			TestEqual("Second row added", DB->ItemCount().Get(), 2);
			DB->SetItem(TEXT("key2"), TEXT("val3")).Wait();
			TestEqual("Second row updated", DB->ItemCount().Get(), 2);
		});
	}

	SECTION("Clear")
	{
		NewStorage(true, "test", [this](const UStorage* DB)
		{
			TestEqual("Initial item count should be 0", DB->ItemCount().Get(), 0);
			DB->SetItem(TEXT("key1"), TEXT("val1")).Wait();
			DB->SetItem(TEXT("key2"), TEXT("val2")).Wait();
			TestEqual("Two rows added", DB->ItemCount().Get(), 2);
			DB->Clear();
			TestEqual("No rows should exists", DB->ItemCount().Get(), 0);
		});
	}

	SECTION("Persistence")
	{
		NewStorage(true, "test", [this](const UStorage* DB)
		{
			DB->SetItem(TEXT("key"), TEXT("val")).Wait();
		});
		// Data should persist if we connect to the same database
		NewStorage(false, "test", [this](const UStorage* DB)
		{
			const auto Got = DB->GetItem(TEXT("key")).Get().GetValue();
			TestEqual("Data should remain persisted", Got, TEXT("val"));
		});
	}

	SECTION("Multiple storages")
	{
		NewStorage(true, "test1", [this](const UStorage* DB)
		{
			DB->SetItem(TEXT("key"), TEXT("val")).Wait();
		});
		NewStorage(false, "test2", [this](const UStorage* DB)
		{
			TestFalse("Second storage should remain clear", DB->GetItem(TEXT("key")).Get().IsSet());
		});
	}

	SECTION("Values")
	{
		NewStorage(true, "test", [this](const UStorage* DB)
		{
			const auto Got = DB->Values(TEXT("")).Get();
			TestEqual("No values by default", Got, TArray<FString>());
			const TMap<FString, FString> Data = {
				{TEXT("foo.1"), TEXT("bar1")},
				{TEXT("zzz.2"), TEXT("bar3")},
				{TEXT("foo.2"), TEXT("bar2")}
			};
			for (const auto& Pair : Data)
			{
				DB->SetItem(Pair.Key, Pair.Value).Wait();
			}

			// Values are sorted by the key
			const auto Values = DB->Values(TEXT("")).Get();
			TestEqual("All values", Values, {TEXT("bar1"), TEXT("bar2"), TEXT("bar3")});

			// Values are sorted and filtered by the prefix
			const auto Filtered = DB->Values(TEXT("foo")).Get();
			TestEqual("Filtered", Filtered, {TEXT("bar1"), TEXT("bar2")});
		});
	}
}

#endif
