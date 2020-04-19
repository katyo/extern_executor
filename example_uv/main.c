#include <string.h>
#include <stdio.h>
#include <uv.h>
#include <rust_async_executor_uv.h>
#include <rust_example_lib.h>

static void delay_cb(void* userdata) {
    (void)userdata;

    printf("async delay() end\n");
}

static void read_file_cb(const char* data, void* userdata) {
    (void)userdata;

    printf("async read_file() end: %u bytes read\n", (unsigned int)strlen(data));
}

int main(void) {
    uv_loop_t loop;

    uv_loop_init(&loop);
    rust_async_executor_uv_init((void*)&loop);

    printf("async delay() start\n");
    delay(2.5, delay_cb, NULL);

    printf("async read_file() start\n");
    read_file("main.c", read_file_cb, NULL);

    uv_run(&loop, UV_RUN_DEFAULT);
    uv_loop_close(&loop);

    return 0;
}
