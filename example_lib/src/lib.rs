#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
use core::ffi::c_void;

#[cfg(any(feature = "read-file", feature = "ns-lookup"))]
use std::os::raw::c_char;

#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
use extern_executor::spawn;

// Wrapped used data pointer for Rust
#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UserData(*mut c_void);

// Allow sending user data between threads
#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
unsafe impl Send for UserData {}

#[cfg(feature = "delay")]
#[no_mangle]
pub extern "C" fn delay(duration: f32, callback: fn(UserData), userdata: UserData) {
    use std::time::Duration;
    use futures_timer::Delay;

    spawn(async move {
        Delay::new(Duration::from_secs_f32(duration)).await;
        callback(userdata);
    });
}

#[cfg(feature = "read-file")]
#[no_mangle]
pub extern "C" fn read_file(path: *const c_char, callback: fn(*mut c_char, *mut c_char, UserData), userdata: UserData) {
    use std::{ptr::null_mut, ffi::{CStr, CString}};
    use async_std::{prelude::*, fs::File};

    let path = unsafe { CStr::from_ptr(path) };
    spawn(async move {
        let path = match path.to_str() {
            Ok(path) => path,
            Err(err) => {
                let error = CString::new(err.to_string()).unwrap().into_raw();
                callback(null_mut(), error, userdata);
                return;
            },
        };
        let (data, error) = match _read_file(path).await {
            Ok(data) => match CString::new(data) {
                Ok(data) => (data.into_raw(), null_mut()),
                Err(err) => (null_mut(), CString::new(err.to_string()).unwrap().into_raw()),
            },
            Err(err) => (null_mut(), CString::new(err).unwrap().into_raw()),
        };
        callback(data, error, userdata);
    });

    async fn _read_file(path: &str) -> Result<String, String> {
        use std::path::Path;

        let path = Path::new(path);
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        let mut data = String::new();
        file.read_to_string(&mut data).await.map_err(|e| e.to_string())?;
        Ok(data)
    }
}

#[cfg(feature = "ns-lookup")]
pub mod ns_lookup {
    use super::*;
    use core::mem::transmute;
    use std::net::{IpAddr};

    #[derive(Clone, Copy)]
    #[repr(u8)]
    pub enum IPKind {
        V4 = 4,
        V6 = 6,
    }

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union IPData {
        pub v4: [u8; 4],
        pub v6: [u16; 8],
    }

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct IPAddr {
        pub data: IPData,
        pub kind: IPKind,
    }

    impl From<IpAddr> for IPAddr {
        fn from(addr: IpAddr) -> Self {
            match addr {
                IpAddr::V4(addr) => IPAddr { kind: IPKind::V4, data: IPData { v4: addr.octets() } },
                IpAddr::V6(addr) => IPAddr { kind: IPKind::V6, data: IPData { v6: unsafe { transmute(addr.octets()) } } },
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn ns_lookup(domain: *const c_char, callback: fn(*mut IPAddr, *mut c_char, UserData), userdata: UserData) {
        use std::{ptr::null_mut, ffi::{CStr, CString}};

        let domain = unsafe { CStr::from_ptr(domain) };
        spawn(async move {
            let domain = match domain.to_str() {
                Ok(domain) => domain,
                Err(err) => {
                    let error = CString::new(err.to_string()).unwrap().into_raw();
                    callback(null_mut(), error, userdata);
                    return;
                },
            };
            let (address, error) = match _ns_lookup(domain).await {
                Ok(addr) => (Box::into_raw(Box::new(addr.into())), null_mut()),
                Err(err) => (null_mut() as *mut IPAddr, CString::new(err).unwrap().into_raw()),
            };
            callback(address, error, userdata);
        });
    }

    async fn _ns_lookup(domain: &str) -> Result<IpAddr, String> {
        use async_std_resolver::{resolver, config};

        let resolver = resolver(
            config::ResolverConfig::default(),
            config::ResolverOpts::default(),
        ).await.map_err(|e| e.to_string())?;

        let response = resolver.lookup_ip(domain).await.map_err(|e| e.to_string())?;

        response.iter().next().ok_or_else(|| "No A or AAAA reconds found".to_string())
    }
}
