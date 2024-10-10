#if !UE_BUILD_SHIPPING

#include "logic/logic.hpp"
#include "Tests/TestHarnessAdapter.h"

TEST_CASE_NAMED(EncryptedStringTests, "Deusvent.Encryption", "[unit]") {
    SECTION("EncryptedString") {
        auto Plaintext = std::string("foo");
        auto Keys = logic::generate_new_keys();
        auto Encrypted = logic::EncryptedString::init(Plaintext, Keys.private_key);
        auto Decrypted = Encrypted->decrypt(Keys.private_key);
        TestEqual("Decrypt value equal to initial plaintext", Decrypted, Plaintext);
    }
}
#endif
