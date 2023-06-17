/*!
# External executor for async Rust

This project aims to provide simple executor which helps to run asynchronous Rust code using external event loops.

## Usage

On a Rust side you should add `extern_executor` as dependency to your `cdylib` crate and use `spawn()` function to run futures, like so:

```no_run
use extern_executor::spawn;

spawn(async {
  // your awaits
});
```

On a C side you should implement executor's driver using your preferred event loop API.
For example, when [libuv](https://github.com/libuv/libuv) is used it may looks like so:

```c
#include <uv.h>
#include <rust_async_executor.h>

static void task_wake(RustAsyncExecutorExternTask data) {
    uv_async_t* handle = data;
    // wakeup uv's async task
    uv_async_send(handle);
}

static void task_poll(uv_async_t* handle) {
    // poll internal task until task complete
    if (!rust_async_executor_poll(handle->data)) {
        // drop internal task when task complete
        rust_async_executor_drop(handle->data);
        // drop uv's async task handle
        uv_close((uv_handle_t*)handle, NULL);
    }
}

static RustAsyncExecutorExternTask
task_new(RustAsyncExecutorUserData data) {
    uv_loop_t* loop = data;
    // crate and initialize uv's async task handle
    uv_async_t* handle = malloc(sizeof(uv_async_t));
    uv_async_init(loop, handle, task_poll);
    return handle;
}

static void task_run(RustAsyncExecutorExternTask task,
                     RustAsyncExecutorInternTask data) {
    uv_async_t* handle = task;
    // store internal task handle to be able to poll it later
    handle->data = data;
    uv_async_send(handle); // do initial polling (important)
}

void uv_rust_async_executor_init(uv_loop_t *loop) {
    // send out executor API to Rust side
    rust_async_executor_init(task_new, task_run, task_wake, loop);
}
```

Now you can run your async code in __libuv__'s event loop like so:

```c
int main(void) {
    uv_loop_t loop;

    uv_loop_init(&loop);
    uv_rust_async_executor_init(&loop);

    my_async_function(my_async_callback);

    uv_run(&loop, UV_RUN_DEFAULT);
    uv_loop_close(&loop);

    return 0;
}
```

The C header _rust_async_executor.h_ generated using [cbindgen](https://github.com/eqrion/cbindgen/).
There are two options how you can get it:

* Copy from _include_ directory in this repo
* Generate by youself by using _cbindgen_ feature

In second case generated header will be available at `target/$PROFILE/include` directory.

## Built-in event-loop drivers

To simplify setup for some widely used event loops the built-in drivers was introduced.
To use driver you should enable corresponding feature. Currently supported next drivers:

- __uv__ built-in _libuv_ event loop integration (see [example_uv](http://github.com/katyo/extern_executor/tree/master/example_uv))
- __dart__ built-in _dart-lang_ event loop integration (see [example_dart](http://github.com/katyo/extern_executor/tree/master/example_uv))

## Linking issues

Rust currently have an issues related to re-exporting of symbols from crate's dependencies (#[2771](https://github.com/rust-lang/rfcs/issues/2771)).

As temporary solution you can setup build profile like so:

```toml
[profile.release]
lto = true
incremental = false
```

## Tokio compatibility

This executor incompatible with [tokio](https://github.com/tokio-rs/tokio)'s futures because _tokio_ still has non-trivial executor which mixed with reactor.
 */

#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod types;
mod userdata;
mod task;
pub mod ffi;

#[cfg(feature = "uv")]
pub mod uv;

#[cfg(feature = "dart")]
pub mod dart;

pub(crate) use types::*;
pub(crate) use userdata::*;
pub(crate) use task::*;
pub(crate) use ffi::*;

pub(crate) mod global {
    use super::{UserData, null_mut};

    pub static mut TASK_NEW: UserData = null_mut();
    pub static mut TASK_RUN: UserData = null_mut();
    pub static mut TASK_WAKE: UserData = null_mut();
    pub static mut TASK_DATA: UserData = null_mut();
}

/// Spawn task
///
/// Create task for future and run it
pub fn spawn(future: impl Future + Send + 'static) {
    let future = Box::pin(future);

    let task_new: TaskNew = unsafe { transmute(global::TASK_NEW) };
    let task_run: TaskRun = unsafe { transmute(global::TASK_RUN) };
    let task_data: ExternData = unsafe { global::TASK_DATA };

    let task = task_new(task_data);
    task_run(task, task_wrap(future, task));
}

#[deprecated]
#[macro_export]
macro_rules! externs {
    () => {
        /// Initialize async executor by providing task API calls
        #[no_mangle]
        pub extern "C" fn rust_async_executor_init(task_new: $crate::ffi::TaskNew, task_run: $crate::ffi::TaskRun, task_wake: $crate::ffi::TaskWake, task_data: $crate::ffi::ExternData) {
            $crate::ffi::task_init(task_new, task_run, task_wake, task_data);
        }

        /// Task poll function which should be called to resume task
        #[no_mangle]
        pub extern "C" fn rust_async_executor_poll(task: $crate::ffi::InternTask) -> bool {
            $crate::ffi::task_poll(task)
        }

        /// Task drop function which should be called to delete task
        #[no_mangle]
        pub extern "C" fn rust_aync_executor_drop(task: $crate::ffi::InternTask) {
            $crate::ffi::task_drop(task);
        }
    }
}
