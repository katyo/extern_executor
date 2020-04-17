#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod types;
mod userdata;
mod executor;

pub(crate) use types::*;
pub(crate) use userdata::*;
pub use executor::*;

pub use futures::FutureExt;

/// C function which can create new tasks
pub type RawTaskNewFn = fn() -> RawUserData;

/// C function which can run created tasks
pub type RawTaskRunFn = fn(RawUserData, RawUserData);

static mut TASK_NEW: RawUserData = null_mut();
static mut TASK_RUN: RawUserData = null_mut();
static mut TASK_WAKE: RawUserData = null_mut();

/// Initialize async executor by providing task API calls
#[no_mangle]
pub extern fn rust_async_executor_init(task_new: RawTaskNewFn, task_run: RawTaskRunFn, task_wake: RawTaskWakeFn) {
    unsafe {
        TASK_NEW = task_new as _;
        TASK_RUN = task_run as _;
        TASK_WAKE = task_wake as _;
    }
}

/// Task poll function which should be called to resume task
#[no_mangle]
pub extern fn rust_async_executor_poll(data: RawUserData) -> bool {
    let task = unsafe { &*(data as *mut BoxedTask) };
    let res = task.poll();
    res
}

/// Task drop function which should be called to delete task
#[no_mangle]
pub extern fn rust_async_executor_drop(data: RawUserData) {
    let _task = unsafe { Box::from_raw(data as *mut BoxedTask) };
}

pub(crate) fn wake(data: RawUserData) {
    let task_wake: RawTaskWakeFn = unsafe { transmute(TASK_WAKE) };

    task_wake(data);
}

/// Spawn task
///
/// Create task for future and run it
pub fn spawn(future: impl Future + Send + 'static) {
    let future = Box::pin(future.map(|_| ()));

    let task_new: RawTaskNewFn = unsafe { transmute(TASK_NEW) };
    let task_run: RawTaskRunFn = unsafe { transmute(TASK_RUN) };

    let task = task_new();
    task_run(task, task_new_raw(future, task));
}
