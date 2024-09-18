#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef UNIFFI_CPP_INTERNALSTRUCTS
#define UNIFFI_CPP_INTERNALSTRUCTS
struct ForeignBytes {
    int32_t len;
    uint8_t *data;
};

struct RustBuffer {
    int32_t capacity;
    int32_t len;
    uint8_t *data;
};

struct RustCallStatus {
    int8_t code;
    RustBuffer error_buf;
};

#endif

typedef int ForeignCallback(uint64_t handle, uint32_t method, uint8_t *args_data, int32_t args_len, RustBuffer *buf_ptr);

void uniffi_logic_fn_free_duration(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_duration_from_milliseconds(uint64_t milliseconds, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_days(void * ptr, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_hours(void * ptr, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_minutes(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_pingserializer(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_pingserializer_deserialize(RustBuffer data, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_pingserializer_new(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_pingserializer_data(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_pingserializer_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_pingserializer_serialize(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_serverstatusserializer(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_serverstatusserializer_deserialize(RustBuffer input, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_serverstatusserializer_new(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_serverstatusserializer_data(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_serverstatusserializer_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_serverstatusserializer_serialize(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_servertimestamp(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_servertimestamp_from_milliseconds(uint64_t milliseconds, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_servertimestamp_as_string(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_syncedtimestamp(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_syncedtimestamp_new(RustCallStatus *out_status);
void uniffi_logic_fn_method_syncedtimestamp_adjust(void * ptr, void * server_time, void * sent_at, void * received_at, RustCallStatus *out_status);
void * uniffi_logic_fn_method_syncedtimestamp_now(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_timestamp(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_timestamp_from_milliseconds(uint64_t milliseconds, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_timestamp_now(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_timestamp_as_string(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_method_timestamp_diff(void * ptr, void * other, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_func_serverstatus_message_tag(RustCallStatus *out_status);
RustBuffer ffi_logic_rustbuffer_alloc(int32_t size, RustCallStatus *out_status);
RustBuffer ffi_logic_rustbuffer_from_bytes(ForeignBytes bytes, RustCallStatus *out_status);
void ffi_logic_rustbuffer_free(RustBuffer buf, RustCallStatus *out_status);
RustBuffer ffi_logic_rustbuffer_reserve(RustBuffer buf, int32_t additional, RustCallStatus *out_status);
void ffi_logic_rust_future_continuation_callback_set(intptr_t callback);
void ffi_logic_rust_future_poll_u8(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_u8(intptr_t handle);
void ffi_logic_rust_future_free_u8(intptr_t handle);
uint8_t ffi_logic_rust_future_complete_u8(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_i8(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_i8(intptr_t handle);
void ffi_logic_rust_future_free_i8(intptr_t handle);
int8_t ffi_logic_rust_future_complete_i8(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_u16(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_u16(intptr_t handle);
void ffi_logic_rust_future_free_u16(intptr_t handle);
uint16_t ffi_logic_rust_future_complete_u16(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_i16(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_i16(intptr_t handle);
void ffi_logic_rust_future_free_i16(intptr_t handle);
int16_t ffi_logic_rust_future_complete_i16(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_u32(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_u32(intptr_t handle);
void ffi_logic_rust_future_free_u32(intptr_t handle);
uint32_t ffi_logic_rust_future_complete_u32(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_i32(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_i32(intptr_t handle);
void ffi_logic_rust_future_free_i32(intptr_t handle);
int32_t ffi_logic_rust_future_complete_i32(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_u64(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_u64(intptr_t handle);
void ffi_logic_rust_future_free_u64(intptr_t handle);
uint64_t ffi_logic_rust_future_complete_u64(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_i64(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_i64(intptr_t handle);
void ffi_logic_rust_future_free_i64(intptr_t handle);
int64_t ffi_logic_rust_future_complete_i64(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_f32(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_f32(intptr_t handle);
void ffi_logic_rust_future_free_f32(intptr_t handle);
float ffi_logic_rust_future_complete_f32(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_f64(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_f64(intptr_t handle);
void ffi_logic_rust_future_free_f64(intptr_t handle);
double ffi_logic_rust_future_complete_f64(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_pointer(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_pointer(intptr_t handle);
void ffi_logic_rust_future_free_pointer(intptr_t handle);
void * ffi_logic_rust_future_complete_pointer(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_rust_buffer(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_rust_buffer(intptr_t handle);
void ffi_logic_rust_future_free_rust_buffer(intptr_t handle);
RustBuffer ffi_logic_rust_future_complete_rust_buffer(intptr_t handle, RustCallStatus *out_status);
void ffi_logic_rust_future_poll_void(intptr_t handle, intptr_t uniffi_callback);
void ffi_logic_rust_future_cancel_void(intptr_t handle);
void ffi_logic_rust_future_free_void(intptr_t handle);
void ffi_logic_rust_future_complete_void(intptr_t handle, RustCallStatus *out_status);
uint16_t uniffi_logic_checksum_func_serverstatus_message_tag();
uint16_t uniffi_logic_checksum_method_duration_whole_days();
uint16_t uniffi_logic_checksum_method_duration_whole_hours();
uint16_t uniffi_logic_checksum_method_duration_whole_minutes();
uint16_t uniffi_logic_checksum_method_pingserializer_data();
uint16_t uniffi_logic_checksum_method_pingserializer_debug_string();
uint16_t uniffi_logic_checksum_method_pingserializer_serialize();
uint16_t uniffi_logic_checksum_method_serverstatusserializer_data();
uint16_t uniffi_logic_checksum_method_serverstatusserializer_debug_string();
uint16_t uniffi_logic_checksum_method_serverstatusserializer_serialize();
uint16_t uniffi_logic_checksum_method_servertimestamp_as_string();
uint16_t uniffi_logic_checksum_method_syncedtimestamp_adjust();
uint16_t uniffi_logic_checksum_method_syncedtimestamp_now();
uint16_t uniffi_logic_checksum_method_timestamp_as_string();
uint16_t uniffi_logic_checksum_method_timestamp_diff();
uint16_t uniffi_logic_checksum_constructor_duration_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_pingserializer_deserialize();
uint16_t uniffi_logic_checksum_constructor_pingserializer_new();
uint16_t uniffi_logic_checksum_constructor_serverstatusserializer_deserialize();
uint16_t uniffi_logic_checksum_constructor_serverstatusserializer_new();
uint16_t uniffi_logic_checksum_constructor_servertimestamp_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_syncedtimestamp_new();
uint16_t uniffi_logic_checksum_constructor_timestamp_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_timestamp_now();
uint32_t ffi_logic_uniffi_contract_version();

#ifdef __cplusplus
}
#endif