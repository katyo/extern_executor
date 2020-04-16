#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod types;
mod userdata;
mod executor;

pub(crate) use types::*;
pub(crate) use userdata::*;
pub use executor::*;

/// C function which can create new tasks
pub type RawTaskNewFn = fn(*mut RawWakeFn, *mut RawUserData);

/// C function which can run created tasks
pub type RawTaskRunFn = fn(RawPollFn, RawDropFn, RawUserData);

static mut TASK_NEW: RawUserData = null_mut();
static mut TASK_RUN: RawUserData = null_mut();

#[no_mangle]
pub extern fn rust_async_executor_init(task_new: RawTaskNewFn, task_run: RawTaskRunFn) {
    unsafe {
        TASK_NEW = task_new as _;
        TASK_RUN = task_run as _;
    }
}

pub fn spawn(future: impl Future + Send + 'static) {
    let future = Box::pin(future);

    let task_new: RawTaskNewFn = unsafe { core::mem::transmute(TASK_NEW) };
    let task_run: RawTaskRunFn = unsafe { core::mem::transmute(TASK_RUN) };

    let mut wake = MaybeUninit::uninit();
    let mut data = MaybeUninit::uninit();

    task_new(wake.as_mut_ptr(), data.as_mut_ptr());

    let (poll, drop, data) = task_new_raw(
        future,
        unsafe { wake.assume_init() },
        unsafe { data.assume_init() },
    );
    task_run(poll, drop, data);
}
