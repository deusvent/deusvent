#include "Connection.h"
#if !UE_BUILD_SHIPPING

#include "logic/logic.hpp"
#include "Tests/TestHarnessAdapter.h"

TEST_CASE_NAMED(WSConnectionTests, "Deusvent.Connection", "[unit]") {
    SECTION("SendMessages") {
        auto Keys = logic::generate_new_keys();
        auto Connection = new UConnection("https://api.deusvent.com", Keys);
        auto Response =
            Connection->SendPublicMessageAsync<logic::ServerStatusSerializer>(logic::Ping::init())
                .Get();
    }
}
#endif
