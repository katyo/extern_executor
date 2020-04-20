#include <string.h>
#include <stdio.h>
#include <uv.h>
#include <rust_async_executor_uv.h>
#include <rust_example_lib.h>

static void delay_cb(void* userdata) {
    const char* secs = userdata;

    printf("async delay(%s) end\n", secs);
}

static void read_file_cb(char* data, char* error, void* userdata) {
    const char* name = userdata;

    if (data) {
        printf("async read_file('%s') ok: %u bytes\n", name, (unsigned int)strlen(data));
        free(data);
    } else {
        printf("async read_file('%s') error: %s\n", name, error);
        free(error);
    }
}

static void ns_lookup_cb(IPAddr* addr, char* error, void* userdata) {
    const char* name = userdata;

    if (addr) {
        printf("async ns_lookup('%s') ok: ", name);
        switch (addr->kind) {
        case 4: {
            unsigned char* b = addr->data.v4;
            printf("%u.%u.%u.%u\n",
                   (unsigned)b[0], (unsigned)b[1], (unsigned)b[2], (unsigned)b[3]);
        } break;
        case 6: {
            unsigned short* b = addr->data.v6;
            printf("%X:%X:%X:%X:%X:%X:%X:%X\n",
                   (unsigned)b[0], (unsigned)b[1], (unsigned)b[2], (unsigned)b[3],
                   (unsigned)b[4], (unsigned)b[5], (unsigned)b[6], (unsigned)b[7]);
        } break;
        }
        free(addr);
    } else {
        printf("async ns_lookup('%s') error: %s\n", name, error);
        free(error);
    }
}

int main(void) {
    uv_loop_t loop;

    uv_loop_init(&loop);
    rust_async_executor_uv_init((void*)&loop);

    printf("async delay(2.5) start\n");
    delay(2.5, delay_cb, "2.5");

    printf("async read_file('main.c') start\n");
    read_file("main.c", read_file_cb, "main.c");

    printf("async read_file('other.c') start\n");
    read_file("other.c", read_file_cb, "other.c");

    printf("async ns_lookup('illumium.org') start\n");
    ns_lookup("illumium.org", ns_lookup_cb, "illumium.org");

    printf("async ns_lookup('nihil.illumium.org') start\n");
    ns_lookup("nihil.illumium.org", ns_lookup_cb, "nihil.illumium.org");

    uv_run(&loop, UV_RUN_DEFAULT);
    uv_loop_close(&loop);

    return 0;
}
