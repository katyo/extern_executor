#ifndef __RUST_ASYNC_EXECUTOR_H__
#define __RUST_ASYNC_EXECUTOR_H__

/* Generated with cbindgen:0.14.1 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Raw C userdata type
 */
typedef void *RustAsyncExecutorUserData;

/**
 * Rust internal task handle
 */
typedef RustAsyncExecutorUserData RustAsyncExecutorInternTask;

/**
 * Raw external task handle
 */
typedef RustAsyncExecutorUserData RustAsyncExecutorExternTask;

/**
 * Raw external data
 */
typedef RustAsyncExecutorUserData RustAsyncExecutorExternData;

/**
 * C function which can create new tasks
 */
typedef RustAsyncExecutorExternTask (*RustAsyncExecutorTaskNew)(RustAsyncExecutorExternData);

/**
 * C function which can run created tasks
 */
typedef void (*RustAsyncExecutorTaskRun)(RustAsyncExecutorExternTask, RustAsyncExecutorInternTask);

/**
 * C function which can wake created task
 *
 * This function will be called when pending future need to be polled again
 */
typedef void (*RustAsyncExecutorTaskWake)(RustAsyncExecutorExternTask);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Task drop function which should be called to delete task
 */
void rust_async_executor_drop(RustAsyncExecutorInternTask data);

/**
 * Initialize async executor by providing task API calls
 */
void rust_async_executor_init(RustAsyncExecutorTaskNew task_new,
                              RustAsyncExecutorTaskRun task_run,
                              RustAsyncExecutorTaskWake task_wake,
                              RustAsyncExecutorExternData task_data);

/**
 * Task poll function which should be called to resume task
 */
bool rust_async_executor_poll(RustAsyncExecutorInternTask data);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* __RUST_ASYNC_EXECUTOR_H__ */
