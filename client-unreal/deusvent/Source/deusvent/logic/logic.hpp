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
struct Decay;
struct DecayQuery;
struct Duration;
struct EncryptedString;
struct Identity;
struct Ping;
struct Ping2;
struct PlayerId;
struct PrivateKey;
struct PublicKey;
struct Serializable;
struct ServerError;
struct ServerStatus;
struct ServerTimestamp;
struct SyncedTimestamp;
struct Timestamp;
struct Keys;
struct EncryptionError;
enum class ErrorCode;
struct SafeString;
struct SerializationError;
enum class Status;

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


struct Keys {
    std::shared_ptr<PublicKey> public_key;
    std::shared_ptr<PrivateKey> private_key;
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

namespace uniffi {
    struct FfiConverterDecay;
} // namespace uniffi

struct Decay {
    friend uniffi::FfiConverterDecay;

    Decay() = delete;

    Decay(const Decay &) = delete;
    Decay(Decay &&) = delete;

    Decay &operator=(const Decay &) = delete;
    Decay &operator=(Decay &&) = delete;

    ~Decay();
    static std::shared_ptr<Decay> deserialize(const std::string &data);
    std::string debug_string();
    std::shared_ptr<Duration> length();
    std::shared_ptr<ServerTimestamp> started_at();

private:
    Decay(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterDecayQuery;
} // namespace uniffi

struct DecayQuery {
    friend uniffi::FfiConverterDecayQuery;

    DecayQuery() = delete;

    DecayQuery(const DecayQuery &) = delete;
    DecayQuery(DecayQuery &&) = delete;

    DecayQuery &operator=(const DecayQuery &) = delete;
    DecayQuery &operator=(DecayQuery &&) = delete;

    ~DecayQuery();
    static std::shared_ptr<DecayQuery> init();
    std::string debug_string();
    std::string serialize(uint8_t request_id, const Keys &keys);

private:
    DecayQuery(void *);

    void *instance;
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
    struct FfiConverterIdentity;
} // namespace uniffi

struct Identity {
    friend uniffi::FfiConverterIdentity;

    Identity() = delete;

    Identity(const Identity &) = delete;
    Identity(Identity &&) = delete;

    Identity &operator=(const Identity &) = delete;
    Identity &operator=(Identity &&) = delete;

    ~Identity();
    std::string debug_string();
    std::string serialize(uint8_t request_id, const Keys &keys);

private:
    Identity(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPing;
} // namespace uniffi

struct Ping {
    friend uniffi::FfiConverterPing;

    Ping() = delete;

    Ping(const Ping &) = delete;
    Ping(Ping &&) = delete;

    Ping &operator=(const Ping &) = delete;
    Ping &operator=(Ping &&) = delete;

    ~Ping();
    static std::shared_ptr<Ping> init();
    std::string debug_string();
    std::string serialize(uint8_t request_id);

private:
    Ping(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterPing2;
} // namespace uniffi

struct Ping2 {
    friend uniffi::FfiConverterPing2;

    Ping2() = delete;

    Ping2(const Ping2 &) = delete;
    Ping2(Ping2 &&) = delete;

    Ping2 &operator=(const Ping2 &) = delete;
    Ping2 &operator=(Ping2 &&) = delete;

    ~Ping2();
    static std::shared_ptr<Ping2> init();
    std::string serialize(uint8_t request_id);

private:
    Ping2(void *);

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
    struct FfiConverterSerializable;
} // namespace uniffi

struct Serializable {
    friend uniffi::FfiConverterSerializable;

    Serializable() = delete;

    Serializable(const Serializable &) = delete;
    Serializable(Serializable &&) = delete;

    Serializable &operator=(const Serializable &) = delete;
    Serializable &operator=(Serializable &&) = delete;

    ~Serializable();
    std::string serialize(uint8_t request_id);

private:
    Serializable(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterServerError;
} // namespace uniffi

struct ServerError {
    friend uniffi::FfiConverterServerError;

    ServerError() = delete;

    ServerError(const ServerError &) = delete;
    ServerError(ServerError &&) = delete;

    ServerError &operator=(const ServerError &) = delete;
    ServerError &operator=(ServerError &&) = delete;

    ~ServerError();
    static std::shared_ptr<ServerError> deserialize(const std::string &data);
    std::string debug_string();
    ErrorCode error_code();
    std::optional<std::string> error_context();
    std::string error_description();
    uint16_t message_tag();
    bool recoverable();
    uint8_t request_id();

private:
    ServerError(void *);

    void *instance;
};

namespace uniffi {
    struct FfiConverterServerStatus;
} // namespace uniffi

struct ServerStatus {
    friend uniffi::FfiConverterServerStatus;

    ServerStatus() = delete;

    ServerStatus(const ServerStatus &) = delete;
    ServerStatus(ServerStatus &&) = delete;

    ServerStatus &operator=(const ServerStatus &) = delete;
    ServerStatus &operator=(ServerStatus &&) = delete;

    ~ServerStatus();
    static std::shared_ptr<ServerStatus> deserialize(const std::string &data);
    std::string debug_string();
    Status status();
    std::shared_ptr<ServerTimestamp> timestamp();

private:
    ServerStatus(void *);

    void *instance;
};

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


enum class ErrorCode: int32_t {
    kAuthenticationError = 1,
    kSerializationError = 2,
    kInvalidData = 3,
    kIoError = 4,
    kServerError = 5
};

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


enum class Status: int32_t {
    kOk = 1
};

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

struct FfiConverterUInt16 {
    static uint16_t lift(uint16_t);
    static uint16_t lower(uint16_t);
    static uint16_t read(RustStream &);
    static void write(RustStream &, uint16_t);
    static int32_t allocation_size(uint16_t);
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

struct FfiConverterDecay {
    static std::shared_ptr<Decay> lift(void *);
    static void *lower(const std::shared_ptr<Decay> &);
    static std::shared_ptr<Decay> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Decay> &);
    static int32_t allocation_size(const std::shared_ptr<Decay> &);
};

struct FfiConverterDecayQuery {
    static std::shared_ptr<DecayQuery> lift(void *);
    static void *lower(const std::shared_ptr<DecayQuery> &);
    static std::shared_ptr<DecayQuery> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<DecayQuery> &);
    static int32_t allocation_size(const std::shared_ptr<DecayQuery> &);
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

struct FfiConverterIdentity {
    static std::shared_ptr<Identity> lift(void *);
    static void *lower(const std::shared_ptr<Identity> &);
    static std::shared_ptr<Identity> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Identity> &);
    static int32_t allocation_size(const std::shared_ptr<Identity> &);
};

struct FfiConverterPing {
    static std::shared_ptr<Ping> lift(void *);
    static void *lower(const std::shared_ptr<Ping> &);
    static std::shared_ptr<Ping> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Ping> &);
    static int32_t allocation_size(const std::shared_ptr<Ping> &);
};

struct FfiConverterPing2 {
    static std::shared_ptr<Ping2> lift(void *);
    static void *lower(const std::shared_ptr<Ping2> &);
    static std::shared_ptr<Ping2> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Ping2> &);
    static int32_t allocation_size(const std::shared_ptr<Ping2> &);
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

struct FfiConverterSerializable {
    static std::shared_ptr<Serializable> lift(void *);
    static void *lower(const std::shared_ptr<Serializable> &);
    static std::shared_ptr<Serializable> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Serializable> &);
    static int32_t allocation_size(const std::shared_ptr<Serializable> &);
};

struct FfiConverterServerError {
    static std::shared_ptr<ServerError> lift(void *);
    static void *lower(const std::shared_ptr<ServerError> &);
    static std::shared_ptr<ServerError> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<ServerError> &);
    static int32_t allocation_size(const std::shared_ptr<ServerError> &);
};

struct FfiConverterServerStatus {
    static std::shared_ptr<ServerStatus> lift(void *);
    static void *lower(const std::shared_ptr<ServerStatus> &);
    static std::shared_ptr<ServerStatus> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<ServerStatus> &);
    static int32_t allocation_size(const std::shared_ptr<ServerStatus> &);
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

struct FfiConverterTypeKeys {
    static Keys lift(RustBuffer);
    static RustBuffer lower(const Keys &);
    static Keys read(RustStream &);
    static void write(RustStream &, const Keys &);
    static int32_t allocation_size(const Keys &);
};

struct FfiConverterTypeEncryptionError {
    static std::unique_ptr<EncryptionError> lift(RustBuffer buf);
    static RustBuffer lower(const EncryptionError &);
    static std::unique_ptr<EncryptionError> read(RustStream &stream);
    static void write(RustStream &stream, const EncryptionError &);
    static int32_t allocation_size(const EncryptionError &);
};

struct FfiConverterTypeErrorCode {
    static ErrorCode lift(RustBuffer);
    static RustBuffer lower(const ErrorCode &);
    static ErrorCode read(RustStream &);
    static void write(RustStream &, const ErrorCode &);
    static int32_t allocation_size(const ErrorCode &);
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

struct FfiConverterOptionalString {
    static std::optional<std::string> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<std::string>& val);
    static std::optional<std::string> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<std::string>& value);
    static int32_t allocation_size(const std::optional<std::string> &val);
};
} // namespace uniffi

std::string decay_message_tag();
std::string server_error_message_tag();
std::string server_status_message_tag();
Keys generate_new_keys();
uint8_t parse_request_id(const std::string &data);
std::string serialize_me(const std::shared_ptr<Serializable> &msg, uint8_t request_id);
} // namespace logic