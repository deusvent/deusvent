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

void uniffi_logic_fn_free_decay(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_decay_deserialize(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_decay_debug_string(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_method_decay_length(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_method_decay_started_at(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_decayquery(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_decayquery_new(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_decayquery_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_decayquery_serialize(void * ptr, uint8_t request_id, RustBuffer keys, RustCallStatus *out_status);
void uniffi_logic_fn_free_duration(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_duration_from_milliseconds(uint64_t milliseconds, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_days(void * ptr, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_hours(void * ptr, RustCallStatus *out_status);
uint64_t uniffi_logic_fn_method_duration_whole_minutes(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_encryptedstring(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_encryptedstring_new(RustBuffer plaintext, void * private_key, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_encryptedstring_decrypt(void * ptr, void * private_key, RustCallStatus *out_status);
void uniffi_logic_fn_free_identity(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_identity_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_identity_serialize(void * ptr, uint8_t request_id, RustBuffer keys, RustCallStatus *out_status);
void uniffi_logic_fn_free_ping(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_ping_new(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_ping_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_ping_serialize(void * ptr, uint8_t request_id, RustCallStatus *out_status);
void uniffi_logic_fn_free_playerid(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_privatekey(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_privatekey_deserialize(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_privatekey_serialize(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_publickey(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_publickey_deserialize(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_publickey_as_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_publickey_serialize(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_servererror(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_servererror_deserialize(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_servererror_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_servererror_error_code(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_servererror_error_context(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_servererror_error_description(void * ptr, RustCallStatus *out_status);
uint16_t uniffi_logic_fn_method_servererror_message_tag(void * ptr, RustCallStatus *out_status);
int8_t uniffi_logic_fn_method_servererror_recoverable(void * ptr, RustCallStatus *out_status);
uint8_t uniffi_logic_fn_method_servererror_request_id(void * ptr, RustCallStatus *out_status);
void uniffi_logic_fn_free_serverstatus(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_constructor_serverstatus_deserialize(RustBuffer data, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_serverstatus_debug_string(void * ptr, RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_method_serverstatus_status(void * ptr, RustCallStatus *out_status);
void * uniffi_logic_fn_method_serverstatus_timestamp(void * ptr, RustCallStatus *out_status);
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
RustBuffer uniffi_logic_fn_func_decay_message_tag(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_func_servererror_message_tag(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_func_serverstatus_message_tag(RustCallStatus *out_status);
RustBuffer uniffi_logic_fn_func_generate_new_keys(RustCallStatus *out_status);
uint8_t uniffi_logic_fn_func_parse_request_id(RustBuffer data, RustCallStatus *out_status);
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
uint16_t uniffi_logic_checksum_func_decay_message_tag();
uint16_t uniffi_logic_checksum_func_servererror_message_tag();
uint16_t uniffi_logic_checksum_func_serverstatus_message_tag();
uint16_t uniffi_logic_checksum_func_generate_new_keys();
uint16_t uniffi_logic_checksum_func_parse_request_id();
uint16_t uniffi_logic_checksum_method_decay_debug_string();
uint16_t uniffi_logic_checksum_method_decay_length();
uint16_t uniffi_logic_checksum_method_decay_started_at();
uint16_t uniffi_logic_checksum_method_decayquery_debug_string();
uint16_t uniffi_logic_checksum_method_decayquery_serialize();
uint16_t uniffi_logic_checksum_method_duration_whole_days();
uint16_t uniffi_logic_checksum_method_duration_whole_hours();
uint16_t uniffi_logic_checksum_method_duration_whole_minutes();
uint16_t uniffi_logic_checksum_method_encryptedstring_decrypt();
uint16_t uniffi_logic_checksum_method_identity_debug_string();
uint16_t uniffi_logic_checksum_method_identity_serialize();
uint16_t uniffi_logic_checksum_method_ping_debug_string();
uint16_t uniffi_logic_checksum_method_ping_serialize();
uint16_t uniffi_logic_checksum_method_privatekey_serialize();
uint16_t uniffi_logic_checksum_method_publickey_as_string();
uint16_t uniffi_logic_checksum_method_publickey_serialize();
uint16_t uniffi_logic_checksum_method_servererror_debug_string();
uint16_t uniffi_logic_checksum_method_servererror_error_code();
uint16_t uniffi_logic_checksum_method_servererror_error_context();
uint16_t uniffi_logic_checksum_method_servererror_error_description();
uint16_t uniffi_logic_checksum_method_servererror_message_tag();
uint16_t uniffi_logic_checksum_method_servererror_recoverable();
uint16_t uniffi_logic_checksum_method_servererror_request_id();
uint16_t uniffi_logic_checksum_method_serverstatus_debug_string();
uint16_t uniffi_logic_checksum_method_serverstatus_status();
uint16_t uniffi_logic_checksum_method_serverstatus_timestamp();
uint16_t uniffi_logic_checksum_method_servertimestamp_as_string();
uint16_t uniffi_logic_checksum_method_syncedtimestamp_adjust();
uint16_t uniffi_logic_checksum_method_syncedtimestamp_now();
uint16_t uniffi_logic_checksum_method_timestamp_as_string();
uint16_t uniffi_logic_checksum_method_timestamp_diff();
uint16_t uniffi_logic_checksum_constructor_decay_deserialize();
uint16_t uniffi_logic_checksum_constructor_decayquery_new();
uint16_t uniffi_logic_checksum_constructor_duration_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_encryptedstring_new();
uint16_t uniffi_logic_checksum_constructor_ping_new();
uint16_t uniffi_logic_checksum_constructor_privatekey_deserialize();
uint16_t uniffi_logic_checksum_constructor_publickey_deserialize();
uint16_t uniffi_logic_checksum_constructor_servererror_deserialize();
uint16_t uniffi_logic_checksum_constructor_serverstatus_deserialize();
uint16_t uniffi_logic_checksum_constructor_servertimestamp_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_syncedtimestamp_new();
uint16_t uniffi_logic_checksum_constructor_timestamp_from_milliseconds();
uint16_t uniffi_logic_checksum_constructor_timestamp_now();
uint32_t ffi_logic_uniffi_contract_version();

#ifdef __cplusplus
}
#endif