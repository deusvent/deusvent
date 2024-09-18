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
struct Duration;
struct PingSerializer;
struct ServerStatusSerializer;
struct ServerTimestamp;
struct SyncedTimestamp;
struct Timestamp; 
struct Ping; 
struct ServerStatus;
struct SerializationError;
enum class Status;


enum class Status: int32_t {
    kOk = 1
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


struct ServerStatus {
    std::shared_ptr<ServerTimestamp> timestamp;
    Status status;
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
    std::string serialize();

private:
    PingSerializer(void *);

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
    static std::shared_ptr<ServerStatusSerializer> deserialize(const std::string &input);
    ServerStatus data();
    std::string debug_string();
    std::string serialize();

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


struct Ping {
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

struct FfiConverterUInt64 {
    static uint64_t lift(uint64_t);
    static uint64_t lower(uint64_t);
    static uint64_t read(RustStream &);
    static void write(RustStream &, uint64_t);
    static int32_t allocation_size(uint64_t);
};
struct FfiConverterString {
    static std::string lift(RustBuffer buf);
    static RustBuffer lower(const std::string &);
    static std::string read(RustStream &);
    static void write(RustStream &, const std::string &);
    static int32_t allocation_size(const std::string &);
};

struct FfiConverterDuration {
    static std::shared_ptr<Duration> lift(void *);
    static void *lower(const std::shared_ptr<Duration> &);
    static std::shared_ptr<Duration> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<Duration> &);
    static int32_t allocation_size(const std::shared_ptr<Duration> &);
};

struct FfiConverterPingSerializer {
    static std::shared_ptr<PingSerializer> lift(void *);
    static void *lower(const std::shared_ptr<PingSerializer> &);
    static std::shared_ptr<PingSerializer> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<PingSerializer> &);
    static int32_t allocation_size(const std::shared_ptr<PingSerializer> &);
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

std::string server_status_message_tag();
} // namespace logic