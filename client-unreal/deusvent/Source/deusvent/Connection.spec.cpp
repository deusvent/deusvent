#include "Connection.h"

#if !UE_BUILD_SHIPPING

#include "logic/logic.hpp"
#include "Async/Async.h"
#include "Misc/AutomationTest.h"
#include "Engine/GameInstance.h"

DEFINE_SPEC(ConnectionTest,
            "Deusvent.Connection.SendMessages",
            EAutomationTestFlags::ProductFilter | EAutomationTestFlags::ClientContext)
void ConnectionTest::Define() {
    Describe("Connection", [this]() {
        auto Keys = logic::generate_new_keys();
        auto Connection = NewObject<UConnection>(GEngine->GetWorldContexts()[0].World());
        Connection->Init("wss://api.deusvent.com", Keys);

        LatentIt("send public message, received async response", [=](const FDoneDelegate &Done) {
            Connection->SendPublicMessage<logic::ServerStatus>(logic::Ping::init())
                .Next([this, Done](auto Response) {
                    auto Status = std::get<std::shared_ptr<logic::ServerStatus>>(Response);
                    TestEqual("Server status", Status->status(), logic::Status::kOk);
                    Done.Execute();
                });
        });
    });
}

#endif