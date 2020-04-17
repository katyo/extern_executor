#include <string.h>
#include <stdio.h>
#include <uv.h>
#include <rust_example_lib.h>

void ffi_init(uv_loop_t *);

static void delay_cb(void) {
    printf("async delay() end\n");
}

static void read_file_cb(const char *data) {
    printf("async read_file() end: %lu bytes read\n", strlen(data));
}

int main(void) {
    uv_loop_t loop;

    uv_loop_init(&loop);
    ffi_init(&loop);

    printf("async delay() start\n");
    delay(2.5, delay_cb);

    printf("async read_file() start\n");
    read_file("main.c", read_file_cb);

    uv_run(&loop, UV_RUN_DEFAULT);
    uv_loop_close(&loop);

    return 0;
}
