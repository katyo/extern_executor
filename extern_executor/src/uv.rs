use core::mem::MaybeUninit;

#[allow(non_camel_case_types, dead_code, improper_ctypes)]
mod libuv_sys {
    use core::ffi::c_void;
    pub type c_char = i8;
    pub type c_int = i32;
    pub type c_uint = u32;

    #[repr(C)]
    pub(crate) struct uv_loop_t;

    #[repr(C)]
    pub(crate) struct uv_handle_t {
        // public
        pub data: *mut c_void,
        // read-only
        pub uv_loop: *mut uv_loop_t,
        pub type_: u32,
        // private
        close_cb: *mut c_void,
        handle_queue: [*mut c_void; 2],
        reserved: [*mut c_void; 4],
        // platform private
        next_closing: *mut uv_handle_t,
        flags: c_uint,
    }

    #[repr(C)]
    pub(crate) struct uv_async_t {
        pub handle: uv_handle_t,
        // private
        priv_: uv_async_priv_t,
    }

    #[cfg(unix)]
    struct uv_async_priv_t {
        async_cb: *mut c_void,
        queue: [*mut c_void; 2],
        pending: c_int,
    }

    #[cfg(windows)]
    struct uv_async_priv_t {
        async_req: uv_req_s,
        async_cb: *mut c_void,
        async_sent: c_char,
    }

    #[cfg(windows)]
    #[repr(C)]
    struct uv_req_t {
        data: *mut c_void,
        type_: u32,
        reserved: [*mut c_void; 6],
        overlapped: OVERLAPPED,
        queue_bytes: usize,
        next_req: *mut uv_req_t,
    }

    #[cfg(windows)]
    #[repr(C)]
    struct OVERLAPPED {
        internal: *mut u32,
        internal_high: *mut u32,
        offset: u32,
        offset_high: u32,
        event: *mut c_void,
    }

    pub(crate) type uv_async_cb = extern "C" fn(
        handle: *mut uv_async_t,
    );

    extern "C" {
        pub(crate) fn uv_async_init(
            arg1: *mut uv_loop_t,
            async_: *mut uv_async_t,
            async_cb: uv_async_cb,
        ) -> c_int;

        pub(crate) fn uv_async_send(
            async_: *mut uv_async_t,
        ) -> c_int;

        pub(crate) fn uv_close(
            handle: *mut uv_handle_t,
            close_cb: *mut c_void,
        );
    }
}

use libuv_sys::{
    uv_handle_t,
    uv_loop_t,
    uv_async_t,
    uv_async_init,
    uv_async_send,
    uv_close,
};

use crate::{
    UserData,
    null_mut, transmute,
    ffi, ffi::{ExternData, ExternTask, InternTask},
};

/// Libuv loop handle
pub struct UvLoop;

extern "C" fn task_new(data: ExternData) -> ExternTask {
    let uv_loop = data as *mut uv_loop_t;
    let handle: uv_async_t = unsafe { MaybeUninit::uninit().assume_init() };

    let handle = Box::into_raw(Box::new(handle)) as _;
    unsafe { uv_async_init(uv_loop, handle, task_poll) };

    handle as _
}

extern "C" fn task_run(task: ExternTask, data: InternTask) {
    let handle = unsafe { &mut *(task as *mut uv_async_t) };
    handle.handle.data = data as _;
    unsafe { uv_async_send(handle) };
}

extern "C" fn task_poll(handle: *mut uv_async_t) {
    let handle = unsafe { &mut *(handle as *mut uv_async_t) };
    let task = handle.handle.data as InternTask;

    if !ffi::task_poll(task) {
        ffi::task_drop(task);
        unsafe { uv_close(handle as *mut _ as *mut uv_handle_t, transmute(null_mut() as UserData)) };
    }
}

extern "C" fn task_wake(task: ExternTask) {
    let handle = task as *mut uv_async_t;
    unsafe { uv_async_send(handle) };
}

/// Initialize libuv-driven async task executor
#[export_name = "rust_async_executor_uv_init"]
pub extern "C" fn loop_init(uv_loop: *mut UvLoop) {
    ffi::loop_init(task_new, task_run, task_wake, uv_loop as _);
}
