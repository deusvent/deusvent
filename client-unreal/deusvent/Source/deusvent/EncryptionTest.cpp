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

        // Check that data could be decrypted after serialization/deserilization
        auto Name = logic::SafeString(logic::SafeString::kEncrypted{.data = Encrypted});
        auto Identity = logic::Identity{.name = Name};
        auto Serializer = logic::IdentitySerializer::init(Identity, Keys.public_key);
        auto Data = Serializer->serialize(1, Keys.private_key);
        auto Deserialized = logic::IdentitySerializer::deserialize(Data);
        auto Got = Deserialized->data();
        auto Val = std::get<logic::SafeString::kEncrypted>(Got.name.get_variant()).data;
        auto DecryptedValue = Val->decrypt(Keys.private_key);
        TestEqual("Deserialized and decrypted value equal to initial plaintext",
                  DecryptedValue,
                  Plaintext);
    }
}
#endif
