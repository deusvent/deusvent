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
struct ServerTimestamp;
struct SyncedTimestamp;
struct Timestamp;

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
    static std::shared_ptr<Timestamp> now();

private:
    Timestamp(void *);

    void *instance;
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
struct FfiConverterString {
    static std::string lift(RustBuffer buf);
    static RustBuffer lower(const std::string &);
    static std::string read(RustStream &);
    static void write(RustStream &, const std::string &);
    static int32_t allocation_size(const std::string &);
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
} // namespace uniffi

} // namespace logic