#include <string.h>
#include <uv.h>
#include <rust_async_executor.h>
#include <rust_example_lib.h>

uv_loop_t loop;

static void raw_task_wake(RustAsyncExecutorRawUserData data) {
    uv_async_t* handle = data;
    uv_async_send(handle);
}

static void raw_task_poll(uv_async_t* handle) {
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

static RustAsyncExecutorRawUserData raw_task_new() {
    printf("task_new()\n");
    uv_async_t* handle = malloc(sizeof(uv_async_t));
    uv_async_init(&loop, handle, raw_task_poll);
    return handle;
}

static void raw_task_run(RustAsyncExecutorRawUserData task, RustAsyncExecutorRawUserData data) {
    printf("task_run()\n");
    uv_async_t* handle = task;
    handle->data = data;
    uv_async_send(handle);
}

static void delay_cb(void) {
    printf("async delay() end\n");
}

static void read_file_cb(const char *data) {
    printf("async read_file() end: %u bytes read\n", strlen(data));
}

int main(void) {
    rust_async_executor_init(raw_task_new, raw_task_run, raw_task_wake);

    uv_loop_init(&loop);

    printf("async delay() start\n");
    delay(2.5, delay_cb);

    printf("async read_file() start\n");
    read_file("main.c", read_file_cb);

    uv_run(&loop, UV_RUN_DEFAULT);
    uv_loop_close(&loop);

    return 0;
}
