use dart_sys::{
    Dart_CObject, Dart_CObject_Type_Dart_CObject_kInt64, Dart_Port, Dart_PostCObject_Type,
};

use crate::{
    ffi,
    ffi::{ExternData, ExternTask, InternTask},
    null_mut, Box,
};

/// Dart's data structure
pub struct DartCObject;

pub(crate) mod global {
    use super::*;

    pub static mut WAKE_PORT: Dart_Port = 0;
    pub static mut TASK_POST: Dart_PostCObject_Type = None;
}

#[repr(transparent)]
struct DartTask {
    data: InternTask,
}

extern "C" fn task_new(_data: ExternData) -> ExternTask {
    Box::into_raw(Box::new(DartTask { data: null_mut() })) as _
}

extern "C" fn task_run(task: ExternTask, data: InternTask) {
    {
        let mut task = unsafe { &mut *(task as *mut DartTask) };
        task.data = data;
    }
    task_wake(task);
}

/// Poll incomplete task
///
/// This function returns true when task is still pending and needs to be polled yet.
/// When task did completed false will be returned. In that case the task is free to drop.
#[export_name = "rust_async_executor_dart_poll"]
pub extern "C" fn task_poll(task: ExternTask) -> bool {
    let task = unsafe { &mut *(task as *mut DartTask) };
    ffi::task_poll(task.data)
}

/// Delete task
///
/// Completed tasks should be dropped to avoid leaks.
///
/// In some unusual cases (say on emergency shutdown or when executed too long)
/// tasks may be deleted before completion.
#[export_name = "rust_async_executor_dart_drop"]
pub extern "C" fn task_drop(task: ExternTask) {
    let task = unsafe { Box::from_raw(task as *mut DartTask) };
    ffi::task_drop(task.data);
}

extern "C" fn task_wake(task: ExternTask) {
    use global::*;

    let wake_port = unsafe { WAKE_PORT };
    let task_post = unsafe { &TASK_POST }.expect("Expecting task_post should not be NULL");

    let mut task_addr = core::mem::MaybeUninit::<Dart_CObject>::uninit();

    unsafe {
        let mut task_addr = task_addr.assume_init_mut();
        task_addr.type_ = Dart_CObject_Type_Dart_CObject_kInt64;
        task_addr.value.as_int64 = task as _;
    };

    unsafe { task_post(wake_port, task_addr.as_mut_ptr()) };
}

/// Initialize dart-driven async task executor
///
/// On a Dart side you should continuously read channel to get task addresses which needs to be polled.
#[export_name = "rust_async_executor_dart_init"]
pub extern "C" fn loop_init(wake_port: Dart_Port, task_post: Dart_PostCObject_Type) {
    use global::*;

    if task_post.is_none() {
        panic!("Expecting task_post should not be NULL");
    }

    unsafe {
        WAKE_PORT = wake_port;
        TASK_POST = task_post;
    }

    ffi::loop_init(task_new, task_run, task_wake, null_mut());
}
