#include <uv.h>
#include <rust_async_executor.h>

static void task_wake(RustAsyncExecutorExternTask data) {
    uv_async_t* handle = data;
    uv_async_send(handle);
}

static void task_poll(uv_async_t* handle) {
    if (rust_async_executor_poll(handle->data)) {
        printf("task_poll() = true\n");
        // pending
    } else {
        printf("task_poll() = false\n");
        // complete
        rust_async_executor_drop(handle->data);
        uv_close((uv_handle_t*)handle, NULL);
    }
}

static RustAsyncExecutorExternTask task_new(RustAsyncExecutorUserData data) {
    printf("task_new()\n");
    uv_loop_t* loop = data;
    uv_async_t* handle = malloc(sizeof(uv_async_t));
    uv_async_init(loop, handle, task_poll);
    return handle;
}

static void task_run(RustAsyncExecutorExternTask task, RustAsyncExecutorInternTask data) {
    printf("task_run()\n");
    uv_async_t* handle = task;
    handle->data = data;
    uv_async_send(handle);
}

void ffi_init(uv_loop_t *loop) {
    rust_async_executor_init(task_new, task_run, task_wake, loop);
}
