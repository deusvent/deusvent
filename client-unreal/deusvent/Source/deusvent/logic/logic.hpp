#pragma once

#include <bit>
#include <cstdint>
#include <exception>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <stdexcept>
#include <streambuf>
#include <type_traits>
#include <variant>

#include "logic_scaffolding.hpp"

namespace logic {
struct DecayQuerySerializer;
struct DecaySerializer;
struct Duration;
struct EncryptedString;
struct IdentitySerializer;
struct PingSerializer;
struct PlayerId;
struct PrivateKey;
struct PublicKey;
struct ServerStatusSerializer;
struct ServerTimestamp;
struct SyncedTimestamp;
struct Timestamp; 
struct Decay; 
struct DecayQuery; 
struct Identity; 
struct Keys; 
struct Ping; 
struct ServerStatus;
struct EncryptionError;
struct SerializationError;
enum class Status;

namespace uniffi {
    struct FfiConverterServerTimestamp;
} // namespace uniffi

struct ServerTimestamp {
    friend uniffi::FfiConverterServerTimestamp;

    ServerTimestamp() = delete;

    ServerTimestamp(const ServerTimestamp &) = delete;
    ServerTimestamp(ServerTimestamp &&) = delete;

    ServerTimestamp &operator=(const ServerTimestamp &) = delete;
    ServerTimestamp &operator=(ServerTimestamp &&) = delete;

    ~ServerTimestamp();
    static std::shared_ptr<ServerTimestamp> from_milliseconds(uint64_t milliseconds);
    std::string as_string();

private:
    ServerTimestamp(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPublicKey;
} // namespace uniffi

struct PublicKey {
    friend uniffi::FfiConverterPublicKey;

    PublicKey() = delete;

    PublicKey(const PublicKey &) = delete;
    PublicKey(PublicKey &&) = delete;

    PublicKey &operator=(const PublicKey &) = delete;
    PublicKey &operator=(PublicKey &&) = delete;

    ~PublicKey();
    static std::shared_ptr<PublicKey> deserialize(const std::vector<uint8_t> &data);
    std::string as_string();
    std::vector<uint8_t> serialize();

private:
    PublicKey(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPrivateKey;
} // namespace uniffi

struct PrivateKey {
    friend uniffi::FfiConverterPrivateKey;

    PrivateKey() = delete;

    PrivateKey(const PrivateKey &) = delete;
    PrivateKey(PrivateKey &&) = delete;

    PrivateKey &operator=(const PrivateKey &) = delete;
    PrivateKey &operator=(PrivateKey &&) = delete;

    ~PrivateKey();
    static std::shared_ptr<PrivateKey> deserialize(const std::vector<uint8_t> &data);
    std::vector<uint8_t> serialize();

private:
    PrivateKey(void *);

    void *instance;
};


enum class Status: int32_t {
    kOk = 1
};

namespace uniffi {
    struct FfiConverterDuration;
} // namespace uniffi

struct Duration {
    friend uniffi::FfiConverterDuration;

    Duration() = delete;

    Duration(const Duration &) = delete;
    Duration(Duration &&) = delete;

    Duration &operator=(const Duration &) = delete;
    Duration &operator=(Duration &&) = delete;

    ~Duration();
    static std::shared_ptr<Duration> from_milliseconds(uint64_t milliseconds);
    uint64_t whole_days();
    uint64_t whole_hours();
    uint64_t whole_minutes();

private:
    Duration(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterEncryptedString;
} // namespace uniffi

struct EncryptedString {
    friend uniffi::FfiConverterEncryptedString;

    EncryptedString() = delete;

    EncryptedString(const EncryptedString &) = delete;
    EncryptedString(EncryptedString &&) = delete;

    EncryptedString &operator=(const EncryptedString &) = delete;
    EncryptedString &operator=(EncryptedString &&) = delete;

    ~EncryptedString();
    static std::shared_ptr<EncryptedString> init(const std::string &plaintext, const std::shared_ptr<PrivateKey> &private_key);
    std::string decrypt(const std::shared_ptr<PrivateKey> &private_key);

private:
    EncryptedString(void *);

    void *instance;
};


struct Decay {
    std::shared_ptr<ServerTimestamp> started_at;
    std::shared_ptr<Duration> length;
};

namespace uniffi {
struct FfiConverterTypeSafeString;
} // namespace uniffi

struct SafeString {
    friend uniffi::FfiConverterTypeSafeString;
    struct kEncrypted {
        std::shared_ptr<EncryptedString> data;
    };
    struct kPlaintext {
        std::string value;
    };
    SafeString(kEncrypted variant): variant(variant) {}
    SafeString(kPlaintext variant): variant(variant) {}

    SafeString(const SafeString &other): variant(other.variant) {}
    SafeString(SafeString &&other): variant(std::move(other.variant)) {}

    SafeString &operator=(const SafeString &other) {
        variant = other.variant;
        return *this;
    }

    SafeString &operator=(SafeString &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kEncrypted, kPlaintext> &get_variant() const {
        return variant;
    }

private:
    std::variant<kEncrypted, kPlaintext> variant;

    SafeString();
};


struct Keys {
    std::shared_ptr<PublicKey> public_key;
    std::shared_ptr<PrivateKey> private_key;
};


struct ServerStatus {
    std::shared_ptr<ServerTimestamp> timestamp;
    Status status;
};


struct Identity {
    SafeString name;
};

namespace uniffi {
    struct FfiConverterDecayQuerySerializer;
} // namespace uniffi

struct DecayQuerySerializer {
    friend uniffi::FfiConverterDecayQuerySerializer;

    DecayQuerySerializer() = delete;

    DecayQuerySerializer(const DecayQuerySerializer &) = delete;
    DecayQuerySerializer(DecayQuerySerializer &&) = delete;

    DecayQuerySerializer &operator=(const DecayQuerySerializer &) = delete;
    DecayQuerySerializer &operator=(DecayQuerySerializer &&) = delete;

    ~DecayQuerySerializer();
    static std::shared_ptr<DecayQuerySerializer> init(const DecayQuery &data, const std::shared_ptr<PublicKey> &public_key);
    static std::shared_ptr<DecayQuerySerializer> deserialize(const std::string &data);
    DecayQuery data();
    std::string debug_string();
    std::string serialize(uint8_t request_id, const std::shared_ptr<PrivateKey> &private_key);

private:
    DecayQuerySerializer(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterDecaySerializer;
} // namespace uniffi

struct DecaySerializer {
    friend uniffi::FfiConverterDecaySerializer;

    DecaySerializer() = delete;

    DecaySerializer(const DecaySerializer &) = delete;
    DecaySerializer(DecaySerializer &&) = delete;

    DecaySerializer &operator=(const DecaySerializer &) = delete;
    DecaySerializer &operator=(DecaySerializer &&) = delete;

    ~DecaySerializer();
    static std::shared_ptr<DecaySerializer> init(const Decay &data);
    static std::shared_ptr<DecaySerializer> deserialize(const std::string &data);
    Decay data();
    std::string debug_string();
    uint8_t request_id();
    std::string serialize(uint8_t request_id);

private:
    DecaySerializer(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterIdentitySerializer;
} // namespace uniffi

struct IdentitySerializer {
    friend uniffi::FfiConverterIdentitySerializer;

    IdentitySerializer() = delete;

    IdentitySerializer(const IdentitySerializer &) = delete;
    IdentitySerializer(IdentitySerializer &&) = delete;

    IdentitySerializer &operator=(const IdentitySerializer &) = delete;
    IdentitySerializer &operator=(IdentitySerializer &&) = delete;

    ~IdentitySerializer();
    static std::shared_ptr<IdentitySerializer> init(const Identity &data, const std::shared_ptr<PublicKey> &public_key);
    static std::shared_ptr<IdentitySerializer> deserialize(const std::string &data);
    Identity data();
    std::string debug_string();
    std::string serialize(uint8_t request_id, const std::shared_ptr<PrivateKey> &private_key);

private:
    IdentitySerializer(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPingSerializer;
} // namespace uniffi

struct PingSerializer {
    friend uniffi::FfiConverterPingSerializer;

    PingSerializer() = delete;

    PingSerializer(const PingSerializer &) = delete;
    PingSerializer(PingSerializer &&) = delete;

    PingSerializer &operator=(const PingSerializer &) = delete;
    PingSerializer &operator=(PingSerializer &&) = delete;

    ~PingSerializer();
    static std::shared_ptr<PingSerializer> init(const Ping &data);
    static std::shared_ptr<PingSerializer> deserialize(const std::string &data);
    Ping data();
    std::string debug_string();
    std::string serialize(uint8_t request_id);

private:
    PingSerializer(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPlayerId;
} // namespace uniffi

struct PlayerId {
    friend uniffi::FfiConverterPlayerId;

    PlayerId() = delete;

    PlayerId(const PlayerId &) = delete;
    PlayerId(PlayerId &&) = delete;

    PlayerId &operator=(const PlayerId &) = delete;
    PlayerId &operator=(PlayerId &&) = delete;

    ~PlayerId();

private:
    PlayerId(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterServerStatusSerializer;
} // namespace uniffi

struct ServerStatusSerializer {
    friend uniffi::FfiConverterServerStatusSerializer;

    ServerStatusSerializer() = delete;

    ServerStatusSerializer(const ServerStatusSerializer &) = delete;
    ServerStatusSerializer(ServerStatusSerializer &&) = delete;

    ServerStatusSerializer &operator=(const ServerStatusSerializer &) = delete;
    ServerStatusSerializer &operator=(ServerStatusSerializer &&) = delete;

    ~ServerStatusSerializer();
    static std::shared_ptr<ServerStatusSerializer> init(const ServerStatus &data);
    static std::shared_ptr<ServerStatusSerializer> deserialize(const std::string &data);
    ServerStatus data();
    std::string debug_string();
    uint8_t request_id();
    std::string serialize(uint8_t request_id);

private:
    ServerStatusSerializer(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterSyncedTimestamp;
} // namespace uniffi

struct SyncedTimestamp {
    friend uniffi::FfiConverterSyncedTimestamp;

    SyncedTimestamp() = delete;

    SyncedTimestamp(const SyncedTimestamp &) = delete;
    SyncedTimestamp(SyncedTimestamp &&) = delete;

    SyncedTimestamp &operator=(const SyncedTimestamp &) = delete;
    SyncedTimestamp &operator=(SyncedTimestamp &&) = delete;

    ~SyncedTimestamp();
    static std::shared_ptr<SyncedTimestamp> init();
    void adjust(const std::shared_ptr<ServerTimestamp> &server_time, const std::shared_ptr<Timestamp> &sent_at, const std::shared_ptr<Timestamp> &received_at);
    std::shared_ptr<Timestamp> now();

private:
    SyncedTimestamp(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterTimestamp;
} // namespace uniffi

struct Timestamp {
    friend uniffi::FfiConverterTimestamp;

    Timestamp() = delete;

    Timestamp(const Timestamp &) = delete;
    Timestamp(Timestamp &&) = delete;

    Timestamp &operator=(const Timestamp &) = delete;
    Timestamp &operator=(Timestamp &&) = delete;

    ~Timestamp();
    static std::shared_ptr<Timestamp> from_milliseconds(uint64_t milliseconds);
    static std::shared_ptr<Timestamp> now();
    std::string as_string();
    std::shared_ptr<Duration> diff(const std::shared_ptr<Timestamp> &other);

private:
    Timestamp(void *);

    void *instance;
};


struct DecayQuery {
    bool unused;
};


struct Ping {
    bool unused;
};

namespace uniffi {
struct FfiConverterTypeEncryptionError;
} // namespace uniffi

struct EncryptionError: std::runtime_error {
    friend uniffi::FfiConverterTypeEncryptionError;

    EncryptionError() : std::runtime_error("") {}
    EncryptionError(const std::string &what_arg) : std::runtime_error(what_arg) {}

    virtual void throw_underlying() = 0;

    virtual ~EncryptionError() = default;
protected:
    virtual int32_t get_variant_idx() const {
        return 0;
    };
};
/**
 * Contains variants of EncryptionError
 */
namespace encryption_error {

struct InvalidData: EncryptionError {

    InvalidData() : EncryptionError("") {}
    InvalidData(const std::string &what_arg) : EncryptionError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

    int32_t get_variant_idx() const override {
        return 1;
    }
};
} // namespace encryption_error

namespace uniffi {
struct FfiConverterTypeSerializationError;
} // namespace uniffi

struct SerializationError: std::runtime_error {
    friend uniffi::FfiConverterTypeSerializationError;

    SerializationError() : std::runtime_error("") {}
    SerializationError(const std::string &what_arg) : std::runtime_error(what_arg) {}

    virtual void throw_underlying() = 0;

    virtual ~SerializationError() = default;
protected:
    virtual int32_t get_variant_idx() const {
        return 0;
    };
};
/**
 * Contains variants of SerializationError
 */
namespace serialization_error {

struct BadData: SerializationError {
    std::string msg;

    BadData() : SerializationError("") {}
    BadData(const std::string &what_arg) : SerializationError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

    int32_t get_variant_idx() const override {
        return 1;
    }
};
} // namespace serialization_error

namespace uniffi {struct RustStreamBuffer: std::basic_streambuf<char> {
    RustStreamBuffer(RustBuffer *buf) {
        char* data = reinterpret_cast<char*>(buf->data);
        this->setg(data, data, data + buf->len);
        this->setp(data, data + buf->capacity);
    }
    ~RustStreamBuffer() = default;

private:
    RustStreamBuffer() = delete;
    RustStreamBuffer(const RustStreamBuffer &) = delete;
    RustStreamBuffer(RustStreamBuffer &&) = delete;

    RustStreamBuffer &operator=(const RustStreamBuffer &) = delete;
    RustStreamBuffer &operator=(RustStreamBuffer &&) = delete;
};

struct RustStream: std::basic_iostream<char> {
    RustStream(RustBuffer *buf):
        std::basic_iostream<char>(&streambuf), streambuf(RustStreamBuffer(buf)) { }

    template <typename T, typename = std::enable_if_t<std::is_arithmetic_v<T>>>
    RustStream &operator>>(T &val) {
        read(reinterpret_cast<char *>(&val), sizeof(T));

        if (std::endian::native != std::endian::big) {
            auto bytes = reinterpret_cast<char *>(&val);

            std::reverse(bytes, bytes + sizeof(T));
        }

        return *this;
    }

    template <typename T, typename = std::enable_if_t<std::is_arithmetic_v<T>>>
    RustStream &operator<<(T val) {
        if (std::endian::native != std::endian::big) {
            auto bytes = reinterpret_cast<char *>(&val);

            std::reverse(bytes, bytes + sizeof(T));
        }

        write(reinterpret_cast<char *>(&val), sizeof(T));

        return *this;
    }
private:
    RustStreamBuffer streambuf;
};


RustBuffer rustbuffer_alloc(int32_t);
RustBuffer rustbuffer_from_bytes(const ForeignBytes &);
void rustbuffer_free(RustBuffer);

struct FfiConverterUInt8 {
    static uint8_t lift(uint8_t);
    static uint8_t lower(uint8_t);
    static uint8_t read(RustStream &);
    static void write(RustStream &, uint8_t);
    static int32_t allocation_size(uint8_t);
};

struct FfiConverterUInt64 {
    static uint64_t lift(uint64_t);
    static uint64_t lower(uint64_t);
    static uint64_t read(RustStream &);
    static void write(RustStream &, uint64_t);
    static int32_t allocation_size(uint64_t);
};

struct FfiConverterBool {
    static bool lift(uint8_t);
    static uint8_t lower(bool);
    static bool read(RustStream &);
    static void write(RustStream &, bool);
    static int32_t allocation_size(bool);
};
struct FfiConverterString {
    static std::string lift(RustBuffer buf);
    static RustBuffer lower(const std::string &);
    static std::string read(RustStream &);
    static void write(RustStream &, const std::string &);
    static int32_t allocation_size(const std::string &);
};

struct FfiConverterBytes {
    static std::vector<uint8_t> lift(RustBuffer);
    static RustBuffer lower(const std::vector<uint8_t> &);
    static std::vector<uint8_t> read(RustStream &);
    static void write(RustStream &, const std::vector<uint8_t> &);
    static int32_t allocation_size(const std::vector<uint8_t> &);
};

struct FfiConverterDecayQuerySerializer {
    static std::shared_ptr<DecayQuerySerializer> lift(void *);
    static void *lower(const std::shared_ptr<DecayQuerySerializer> &);
    static std::shared_ptr<DecayQuerySerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<DecayQuerySerializer> &);
    static int32_t allocation_size(const std::shared_ptr<DecayQuerySerializer> &);
};

struct FfiConverterDecaySerializer {
    static std::shared_ptr<DecaySerializer> lift(void *);
    static void *lower(const std::shared_ptr<DecaySerializer> &);
    static std::shared_ptr<DecaySerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<DecaySerializer> &);
    static int32_t allocation_size(const std::shared_ptr<DecaySerializer> &);
};

struct FfiConverterDuration {
    static std::shared_ptr<Duration> lift(void *);
    static void *lower(const std::shared_ptr<Duration> &);
    static std::shared_ptr<Duration> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Duration> &);
    static int32_t allocation_size(const std::shared_ptr<Duration> &);
};

struct FfiConverterEncryptedString {
    static std::shared_ptr<EncryptedString> lift(void *);
    static void *lower(const std::shared_ptr<EncryptedString> &);
    static std::shared_ptr<EncryptedString> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<EncryptedString> &);
    static int32_t allocation_size(const std::shared_ptr<EncryptedString> &);
};

struct FfiConverterIdentitySerializer {
    static std::shared_ptr<IdentitySerializer> lift(void *);
    static void *lower(const std::shared_ptr<IdentitySerializer> &);
    static std::shared_ptr<IdentitySerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<IdentitySerializer> &);
    static int32_t allocation_size(const std::shared_ptr<IdentitySerializer> &);
};

struct FfiConverterPingSerializer {
    static std::shared_ptr<PingSerializer> lift(void *);
    static void *lower(const std::shared_ptr<PingSerializer> &);
    static std::shared_ptr<PingSerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<PingSerializer> &);
    static int32_t allocation_size(const std::shared_ptr<PingSerializer> &);
};

struct FfiConverterPlayerId {
    static std::shared_ptr<PlayerId> lift(void *);
    static void *lower(const std::shared_ptr<PlayerId> &);
    static std::shared_ptr<PlayerId> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<PlayerId> &);
    static int32_t allocation_size(const std::shared_ptr<PlayerId> &);
};

struct FfiConverterPrivateKey {
    static std::shared_ptr<PrivateKey> lift(void *);
    static void *lower(const std::shared_ptr<PrivateKey> &);
    static std::shared_ptr<PrivateKey> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<PrivateKey> &);
    static int32_t allocation_size(const std::shared_ptr<PrivateKey> &);
};

struct FfiConverterPublicKey {
    static std::shared_ptr<PublicKey> lift(void *);
    static void *lower(const std::shared_ptr<PublicKey> &);
    static std::shared_ptr<PublicKey> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<PublicKey> &);
    static int32_t allocation_size(const std::shared_ptr<PublicKey> &);
};

struct FfiConverterServerStatusSerializer {
    static std::shared_ptr<ServerStatusSerializer> lift(void *);
    static void *lower(const std::shared_ptr<ServerStatusSerializer> &);
    static std::shared_ptr<ServerStatusSerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<ServerStatusSerializer> &);
    static int32_t allocation_size(const std::shared_ptr<ServerStatusSerializer> &);
};

struct FfiConverterServerTimestamp {
    static std::shared_ptr<ServerTimestamp> lift(void *);
    static void *lower(const std::shared_ptr<ServerTimestamp> &);
    static std::shared_ptr<ServerTimestamp> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<ServerTimestamp> &);
    static int32_t allocation_size(const std::shared_ptr<ServerTimestamp> &);
};

struct FfiConverterSyncedTimestamp {
    static std::shared_ptr<SyncedTimestamp> lift(void *);
    static void *lower(const std::shared_ptr<SyncedTimestamp> &);
    static std::shared_ptr<SyncedTimestamp> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<SyncedTimestamp> &);
    static int32_t allocation_size(const std::shared_ptr<SyncedTimestamp> &);
};

struct FfiConverterTimestamp {
    static std::shared_ptr<Timestamp> lift(void *);
    static void *lower(const std::shared_ptr<Timestamp> &);
    static std::shared_ptr<Timestamp> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Timestamp> &);
    static int32_t allocation_size(const std::shared_ptr<Timestamp> &);
};

struct FfiConverterTypeDecay {
    static Decay lift(RustBuffer);
    static RustBuffer lower(const Decay &);
    static Decay read(RustStream &);
    static void write(RustStream &, const Decay &);
    static int32_t allocation_size(const Decay &);
};

struct FfiConverterTypeDecayQuery {
    static DecayQuery lift(RustBuffer);
    static RustBuffer lower(const DecayQuery &);
    static DecayQuery read(RustStream &);
    static void write(RustStream &, const DecayQuery &);
    static int32_t allocation_size(const DecayQuery &);
};

struct FfiConverterTypeIdentity {
    static Identity lift(RustBuffer);
    static RustBuffer lower(const Identity &);
    static Identity read(RustStream &);
    static void write(RustStream &, const Identity &);
    static int32_t allocation_size(const Identity &);
};

struct FfiConverterTypeKeys {
    static Keys lift(RustBuffer);
    static RustBuffer lower(const Keys &);
    static Keys read(RustStream &);
    static void write(RustStream &, const Keys &);
    static int32_t allocation_size(const Keys &);
};

struct FfiConverterTypePing {
    static Ping lift(RustBuffer);
    static RustBuffer lower(const Ping &);
    static Ping read(RustStream &);
    static void write(RustStream &, const Ping &);
    static int32_t allocation_size(const Ping &);
};

struct FfiConverterTypeServerStatus {
    static ServerStatus lift(RustBuffer);
    static RustBuffer lower(const ServerStatus &);
    static ServerStatus read(RustStream &);
    static void write(RustStream &, const ServerStatus &);
    static int32_t allocation_size(const ServerStatus &);
};

struct FfiConverterTypeEncryptionError {
    static std::unique_ptr<EncryptionError> lift(RustBuffer buf);
    static RustBuffer lower(const EncryptionError &);
    static std::unique_ptr<EncryptionError> read(RustStream &stream);
    static void write(RustStream &stream, const EncryptionError &);
    static int32_t allocation_size(const EncryptionError &);
};

struct FfiConverterTypeSafeString {
    static SafeString lift(RustBuffer);
    static RustBuffer lower(const SafeString &);
    static SafeString read(RustStream &);
    static void write(RustStream &, const SafeString &);
    static int32_t allocation_size(const SafeString &);
};

struct FfiConverterTypeSerializationError {
    static std::unique_ptr<SerializationError> lift(RustBuffer buf);
    static RustBuffer lower(const SerializationError &);
    static std::unique_ptr<SerializationError> read(RustStream &stream);
    static void write(RustStream &stream, const SerializationError &);
    static int32_t allocation_size(const SerializationError &);
};

struct FfiConverterTypeStatus {
    static Status lift(RustBuffer);
    static RustBuffer lower(const Status &);
    static Status read(RustStream &);
    static void write(RustStream &, const Status &);
    static int32_t allocation_size(const Status &);
};
} // namespace uniffi

std::string decay_message_tag();
std::string server_status_message_tag();
Keys generate_new_keys();
} // namespace logic