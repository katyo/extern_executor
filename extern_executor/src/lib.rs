#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod types;
mod userdata;
mod ffi;
mod task;

pub(crate) use types::*;
pub(crate) use userdata::*;
pub use ffi::*;
pub use task::*;

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

#[macro_export]
macro_rules! externs {
    () => {
        /// Initialize async executor by providing task API calls
        #[no_mangle]
        pub extern "C" fn rust_async_executor_init(task_new: $crate::TaskNew, task_run: $crate::TaskRun, task_wake: $crate::TaskWake, task_data: $crate::ExternData) {
            $crate::task_init(task_new, task_run, task_wake, task_data);
        }

        /// Task poll function which should be called to resume task
        #[no_mangle]
        pub extern "C" fn rust_async_executor_poll(task: $crate::InternTask) -> bool {
            $crate::task_poll(task)
        }

        /// Task drop function which should be called to delete task
        #[no_mangle]
        pub extern "C" fn rust_aync_executor_drop(task: $crate::InternTask) {
            $crate::task_drop(task);
        }
    }
}
