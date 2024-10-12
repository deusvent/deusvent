#include "Connection.h"
#if !UE_BUILD_SHIPPING

#include "logic/logic.hpp"
#include "Tests/TestHarnessAdapter.h"

TEST_CASE_NAMED(WSConnectionTests, "Deusvent.Connection", "[unit]") {
    SECTION("SendMessages") {
        auto Keys = logic::generate_new_keys();
        auto Connection = NewObject<UConnection>();
        Connection->Init("wss://api.deusvent.com", Keys);
        auto Response =
            Connection->SendPublicMessage<logic::ServerStatus>(logic::Ping::init()).Get();
        auto Status = std::get<std::shared_ptr<logic::ServerStatus>>(Response);
        TestEqual("Server status", Status->status(), logic::Status::kOk);
    }
}
#endif
