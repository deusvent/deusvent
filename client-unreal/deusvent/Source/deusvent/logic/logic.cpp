/* This file was generated by uniffi-bindgen-cpp. */
#include <string>

#include "logic.hpp"

namespace logic {
namespace uniffi {
template <class> inline constexpr bool always_false_v = false;

namespace {
void ensure_initialized() {
    auto bindings_contract_version = 24;
    auto scaffolding_contract_version = ffi_logic_uniffi_contract_version();

    if (bindings_contract_version != scaffolding_contract_version) {
        throw std::runtime_error(
            "UniFFI contract version mismatch: try cleaning and rebuilding your project");
    }
    if (uniffi_logic_checksum_method_syncedtimestamp_adjust() != 25234) {
        throw std::runtime_error(
            "UniFFI API checksum mismatch: try cleaning and rebuilding your project");
    }
    if (uniffi_logic_checksum_method_syncedtimestamp_now() != 26588) {
        throw std::runtime_error(
            "UniFFI API checksum mismatch: try cleaning and rebuilding your project");
    }
    if (uniffi_logic_checksum_constructor_syncedtimestamp_new() != 16689) {
        throw std::runtime_error(
            "UniFFI API checksum mismatch: try cleaning and rebuilding your project");
    }
    if (uniffi_logic_checksum_constructor_timestamp_now() != 21649) {
        throw std::runtime_error(
            "UniFFI API checksum mismatch: try cleaning and rebuilding your project");
    }
}

// Note: we need this indirection here and can't inline this code in the rust_call function
// as it's a templated function
void initialize() {
    static std::once_flag init_flag;
    std::call_once(init_flag, ensure_initialized);
}
} // namespace

template <typename F> void check_rust_call(const RustCallStatus &status, F error_cb) {
    switch (status.code) {
    case 0:
        return;

    case 1:
        if constexpr (!std::is_null_pointer_v<F>) {
            error_cb(status.error_buf)->throw_underlying();
        }
        break;

    case 2:
        if (status.error_buf.len > 0) {
            throw std::runtime_error(FfiConverterString::lift(status.error_buf));
        }

        throw std::runtime_error("A Rust panic has occurred");
    }

    throw std::runtime_error("Unexpected Rust call status");
}

template <typename F,
          typename EF,
          typename... Args,
          typename R = std::invoke_result_t<F, Args..., RustCallStatus *>>
R rust_call(F f, EF error_cb, Args... args) {
    initialize();

    RustCallStatus status = {0};

    if constexpr (std::is_void_v<R>) {
        f(args..., &status);
        check_rust_call(status, error_cb);
    } else {
        auto ret = f(args..., &status);
        check_rust_call(status, error_cb);

        return ret;
    }
}

RustBuffer rustbuffer_alloc(int32_t len) {
    RustCallStatus status = {0};
    auto buffer = ffi_logic_rustbuffer_alloc(len, &status);

    check_rust_call(status, nullptr);

    return buffer;
}

RustBuffer rustbuffer_from_bytes(const ForeignBytes &bytes) {
    RustCallStatus status = {0};
    auto buffer = ffi_logic_rustbuffer_from_bytes(bytes, &status);

    check_rust_call(status, nullptr);

    return buffer;
}

void rustbuffer_free(RustBuffer buf) {
    RustCallStatus status = {0};

    ffi_logic_rustbuffer_free(std::move(buf), &status);
    check_rust_call(status, nullptr);
}

std::string FfiConverterString::lift(RustBuffer buf) {
    auto string = std::string(reinterpret_cast<char *>(buf.data), buf.len);

    rustbuffer_free(buf);

    return string;
}

RustBuffer FfiConverterString::lower(const std::string &val) {
    auto len = static_cast<int32_t>(val.length());
    auto bytes = ForeignBytes{len, reinterpret_cast<uint8_t *>(const_cast<char *>(val.data()))};

    return rustbuffer_from_bytes(bytes);
}

std::string FfiConverterString::read(RustStream &stream) {
    int32_t len;
    std::string string;

    stream >> len;

    string.resize(len);
    stream.read(string.data(), len);

    return string;
}

void FfiConverterString::write(RustStream &stream, const std::string &val) {
    stream << static_cast<int32_t>(val.length());
    stream.write(val.data(), val.length());
}

int32_t FfiConverterString::allocation_size(const std::string &val) {
    return static_cast<int32_t>(sizeof(int32_t) + val.length());
}
} // namespace uniffi

ServerTimestamp::ServerTimestamp(void *ptr) : instance(ptr) {
}

ServerTimestamp::~ServerTimestamp() {
    uniffi::rust_call(uniffi_logic_fn_free_servertimestamp, nullptr, this->instance);
}

SyncedTimestamp::SyncedTimestamp(void *ptr) : instance(ptr) {
}

std::shared_ptr<SyncedTimestamp> SyncedTimestamp::init() {
    return std::shared_ptr<SyncedTimestamp>(new SyncedTimestamp(
        uniffi::rust_call(uniffi_logic_fn_constructor_syncedtimestamp_new, nullptr)));
}

void SyncedTimestamp::adjust(const std::shared_ptr<ServerTimestamp> &server_time,
                             const std::shared_ptr<Timestamp> &sent_at,
                             const std::shared_ptr<Timestamp> &received_at) {
    uniffi::rust_call(uniffi_logic_fn_method_syncedtimestamp_adjust,
                      nullptr,
                      this->instance,
                      uniffi::FfiConverterServerTimestamp::lower(server_time),
                      uniffi::FfiConverterTimestamp::lower(sent_at),
                      uniffi::FfiConverterTimestamp::lower(received_at));
}
std::shared_ptr<Timestamp> SyncedTimestamp::now() {
    return uniffi::FfiConverterTimestamp::lift(
        uniffi::rust_call(uniffi_logic_fn_method_syncedtimestamp_now, nullptr, this->instance));
}

SyncedTimestamp::~SyncedTimestamp() {
    uniffi::rust_call(uniffi_logic_fn_free_syncedtimestamp, nullptr, this->instance);
}

Timestamp::Timestamp(void *ptr) : instance(ptr) {
}

std::shared_ptr<Timestamp> Timestamp::now() {
    return std::shared_ptr<Timestamp>(
        new Timestamp(uniffi::rust_call(uniffi_logic_fn_constructor_timestamp_now, nullptr)));
}

Timestamp::~Timestamp() {
    uniffi::rust_call(uniffi_logic_fn_free_timestamp, nullptr, this->instance);
}

namespace uniffi {

std::shared_ptr<ServerTimestamp> FfiConverterServerTimestamp::lift(void *ptr) {
    return std::shared_ptr<ServerTimestamp>(new ServerTimestamp(ptr));
}

void *FfiConverterServerTimestamp::lower(const std::shared_ptr<ServerTimestamp> &obj) {
    return obj->instance;
}

std::shared_ptr<ServerTimestamp> FfiConverterServerTimestamp::read(RustStream &stream) {
    std::uintptr_t ptr;
    stream >> ptr;

    return std::shared_ptr<ServerTimestamp>(new ServerTimestamp(reinterpret_cast<void *>(ptr)));
}

void FfiConverterServerTimestamp::write(RustStream &stream,
                                        const std::shared_ptr<ServerTimestamp> &obj) {
    stream << reinterpret_cast<std::uintptr_t>(obj->instance);
}

int32_t FfiConverterServerTimestamp::allocation_size(const std::shared_ptr<ServerTimestamp> &) {
    return 8;
}

std::shared_ptr<SyncedTimestamp> FfiConverterSyncedTimestamp::lift(void *ptr) {
    return std::shared_ptr<SyncedTimestamp>(new SyncedTimestamp(ptr));
}

void *FfiConverterSyncedTimestamp::lower(const std::shared_ptr<SyncedTimestamp> &obj) {
    return obj->instance;
}

std::shared_ptr<SyncedTimestamp> FfiConverterSyncedTimestamp::read(RustStream &stream) {
    std::uintptr_t ptr;
    stream >> ptr;

    return std::shared_ptr<SyncedTimestamp>(new SyncedTimestamp(reinterpret_cast<void *>(ptr)));
}

void FfiConverterSyncedTimestamp::write(RustStream &stream,
                                        const std::shared_ptr<SyncedTimestamp> &obj) {
    stream << reinterpret_cast<std::uintptr_t>(obj->instance);
}

int32_t FfiConverterSyncedTimestamp::allocation_size(const std::shared_ptr<SyncedTimestamp> &) {
    return 8;
}

std::shared_ptr<Timestamp> FfiConverterTimestamp::lift(void *ptr) {
    return std::shared_ptr<Timestamp>(new Timestamp(ptr));
}

void *FfiConverterTimestamp::lower(const std::shared_ptr<Timestamp> &obj) {
    return obj->instance;
}

std::shared_ptr<Timestamp> FfiConverterTimestamp::read(RustStream &stream) {
    std::uintptr_t ptr;
    stream >> ptr;

    return std::shared_ptr<Timestamp>(new Timestamp(reinterpret_cast<void *>(ptr)));
}

void FfiConverterTimestamp::write(RustStream &stream, const std::shared_ptr<Timestamp> &obj) {
    stream << reinterpret_cast<std::uintptr_t>(obj->instance);
}

int32_t FfiConverterTimestamp::allocation_size(const std::shared_ptr<Timestamp> &) {
    return 8;
}

} // namespace uniffi
} // namespace logic