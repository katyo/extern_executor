use std::{
    time::Duration,
    path::Path,
    os::raw::{c_void, c_char},
    ffi::{CStr, CString},
};
use extern_executor::{spawn};
use futures::FutureExt;
use futures_timer::Delay;
use async_std::{
    prelude::*,
    fs::File,
};

// Wrapped used data pointer for Rust
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UserData(*mut c_void);

// Allow sending user data between threads
unsafe impl Send for UserData {}

#[no_mangle]
pub extern "C" fn delay(duration: f32, callback: fn(UserData), userdata: UserData) {
    spawn(
        Delay::new(Duration::from_secs_f32(duration))
            .map(move |_| callback(userdata))
    );
}

#[no_mangle]
pub extern "C" fn read_file(path: *const c_char, callback: fn(*const c_char, UserData), userdata: UserData) {
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    spawn(async move {
        let path = Path::new(path);
        let mut file = File::open(path).await.unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).await.unwrap();
        let data = CString::new(data).unwrap();
        callback(data.into_raw(), userdata);
    });
}
