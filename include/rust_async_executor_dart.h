#ifndef __RUST_ASYNC_EXECUTOR_DART_H__
#define __RUST_ASYNC_EXECUTOR_DART_H__

/* Generated with cbindgen:0.14.1 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Dart's data structure
 */
typedef struct RustAsyncExecutorDartCObject RustAsyncExecutorDartCObject;

/**
 * Raw C userdata type
 */
typedef void *RustAsyncExecutorUserData;

/**
 * Raw external task handle
 */
typedef RustAsyncExecutorUserData RustAsyncExecutorExternTask;

/**
 * Dart's port identifier
 *
 * The port identifier for wake notifications should be set on initializing event loop
 */
typedef int64_t RustAsyncExecutorDartPort;

/**
 * Dart's function which is used to send datas to ports
 *
 * The pointer to this function should be set on initializing event loop
 */
typedef bool (*RustAsyncExecutorDartPostCObject)(RustAsyncExecutorDartPort, RustAsyncExecutorDartCObject*);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Delete task
 *
 * Completed tasks should be dropped to avoid leaks.
 *
 * In some unusual cases (say on emergency shutdown or when executed too long)
 * tasks may be deleted before completion.
 */
void rust_async_executor_dart_drop(RustAsyncExecutorExternTask task);

/**
 * Initialize dart-driven async task executor
 *
 * On a Dart side you should continuously read channel to get task addresses which needs to be polled.
 */
void rust_async_executor_dart_init(RustAsyncExecutorDartPort wake_port,
                                   RustAsyncExecutorDartPostCObject task_post);

/**
 * Poll incomplete task
 *
 * This function returns true when task is still pending and needs to be polled yet.
 * When task did completed false will be returned. In that case the task is free to drop.
 */
bool rust_async_executor_dart_poll(RustAsyncExecutorExternTask task);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* __RUST_ASYNC_EXECUTOR_DART_H__ */
