#ifndef __RUST_ASYNC_EXECUTOR_UV_H__
#define __RUST_ASYNC_EXECUTOR_UV_H__

/* Generated with cbindgen:0.14.1 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Libuv loop handle
 */
typedef struct RustAsyncExecutorUvLoop RustAsyncExecutorUvLoop;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Initialize libuv-driven async task executor
 */
void rust_async_executor_uv_init(RustAsyncExecutorUvLoop *uv_loop);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* __RUST_ASYNC_EXECUTOR_UV_H__ */
