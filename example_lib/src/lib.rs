#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
use {core::ffi::c_void, extern_executor::spawn};

#[cfg(any(feature = "read-file", feature = "ns-lookup"))]
use std::os::raw::c_char;

// Wrapped used data pointer for Rust
#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UserData(*mut c_void);

// Allow sending user data between threads
#[cfg(any(feature = "delay", feature = "read-file", feature = "ns-lookup"))]
unsafe impl Send for UserData {}

#[cfg(any(
    feature = "tokio-delay",
    feature = "tokio-read-file",
    feature = "tokio-ns-lookup"
))]
pub fn with_tokio_reactor<T>(f: impl FnOnce() -> T) -> T {
    use std::sync::{Arc, Once};

    static ONCE: Once = Once::new();
    static mut RUNTIME: *const Arc<tokio::runtime::Runtime> = std::ptr::null_mut();

    ONCE.call_once(|| {
        unsafe {
            RUNTIME = Box::into_raw(Box::new(Arc::new(tokio::runtime::Runtime::new().unwrap())))
        };
    });

    let runtime = unsafe { &*RUNTIME };
    let _guard = runtime.enter();
    f()
}

#[cfg(feature = "delay")]
#[no_mangle]
pub extern "C" fn delay(duration: f32, callback: extern "C" fn(UserData), userdata: UserData) {
    spawn(async move {
        let duration = std::time::Duration::from_secs_f32(duration);

        #[cfg(feature = "futures-delay")]
        futures_timer::Delay::new(duration).await;

        #[cfg(feature = "async-std-delay")]
        async_std::task::sleep(duration).await;

        #[cfg(feature = "tokio-delay")]
        with_tokio_reactor(|| tokio::time::sleep(duration)).await;

        callback(userdata);
    });
}

/// Example read file fn
///
/// # Safety
///
/// Designed to be used from external code
#[cfg(feature = "read-file")]
#[no_mangle]
pub unsafe extern "C" fn read_file(
    path: *const c_char,
    callback: extern "C" fn(*mut c_char, *mut c_char, UserData),
    userdata: UserData,
) {
    use std::{
        ffi::{CStr, CString},
        ptr::null_mut,
    };

    let path = CStr::from_ptr(path);
    spawn(async move {
        let path = match path.to_str() {
            Ok(path) => path,
            Err(err) => {
                let error = CString::new(err.to_string()).unwrap().into_raw();
                callback(null_mut(), error, userdata);
                return;
            }
        };
        let (data, error) = match _read_file(path).await {
            Ok(data) => match CString::new(data) {
                Ok(data) => (data.into_raw(), null_mut()),
                Err(err) => (
                    null_mut(),
                    CString::new(err.to_string()).unwrap().into_raw(),
                ),
            },
            Err(err) => (null_mut(), CString::new(err).unwrap().into_raw()),
        };
        callback(data, error, userdata);
    });

    async fn _read_file(path: &str) -> Result<String, String> {
        use std::path::Path;

        #[cfg(feature = "async-std-read-file")]
        use async_std::{fs::File, prelude::*};

        #[cfg(feature = "tokio-read-file")]
        use tokio::{fs::File, io::AsyncReadExt};

        let path = Path::new(path);

        #[cfg(feature = "async-std-read-file")]
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;

        #[cfg(feature = "tokio-read-file")]
        let mut file = with_tokio_reactor(|| File::open(path))
            .await
            .map_err(|e| e.to_string())?;

        let mut data = String::new();
        file.read_to_string(&mut data)
            .await
            .map_err(|e| e.to_string())?;
        Ok(data)
    }
}

#[cfg(feature = "ns-lookup")]
pub mod ns_lookup {
    use super::*;
    use core::mem::transmute;
    use std::net::IpAddr;

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
                IpAddr::V4(addr) => IPAddr {
                    kind: IPKind::V4,
                    data: IPData { v4: addr.octets() },
                },
                IpAddr::V6(addr) => IPAddr {
                    kind: IPKind::V6,
                    data: IPData {
                        v6: unsafe { transmute(addr.octets()) },
                    },
                },
            }
        }
    }

    /// Example domain name lookup fn
    ///
    /// # Safety
    ///
    /// Designed to be used from external code.
    /// IP address should be freed using [free_ipaddr] fn.
    /// Error string should be freed using [free_cstr] fn.
    #[no_mangle]
    pub unsafe extern "C" fn ns_lookup(
        domain: *const c_char,
        callback: extern "C" fn(*mut IPAddr, *mut c_char, UserData),
        userdata: UserData,
    ) {
        use std::{
            ffi::{CStr, CString},
            ptr::null_mut,
        };

        let domain = CStr::from_ptr(domain);
        spawn(async move {
            let domain = match domain.to_str() {
                Ok(domain) => domain,
                Err(err) => {
                    let error = CString::new(err.to_string()).unwrap().into_raw();
                    callback(null_mut(), error, userdata);
                    return;
                }
            };
            let (address, error) = match _ns_lookup(domain).await {
                Ok(addr) => (Box::into_raw(Box::new(addr.into())), null_mut()),
                Err(err) => (
                    null_mut() as *mut IPAddr,
                    CString::new(err).unwrap().into_raw(),
                ),
            };
            callback(address, error, userdata);
        });
    }

    async fn _ns_lookup(domain: &str) -> Result<IpAddr, String> {
        #[cfg(feature = "async-std-ns-lookup")]
        {
            use async_std_resolver::{config, resolver};

            let resolver = resolver(
                config::ResolverConfig::default(),
                config::ResolverOpts::default(),
            )
            .await
            .map_err(|e| e.to_string())?;

            let response = resolver
                .lookup_ip(domain)
                .await
                .map_err(|e| e.to_string())?;

            response
                .iter()
                .next()
                .ok_or_else(|| "No A or AAAA reconds found".to_string())
        }

        #[cfg(feature = "tokio-ns-lookup")]
        {
            let mut response =
                with_tokio_reactor(|| tokio::net::lookup_host(format!("{}:0", domain)))
                    .await
                    .map_err(|e| e.to_string())?;

            response
                .next()
                .map(|addr| addr.ip())
                .ok_or_else(|| "No A or AAAA reconds found".to_string())
        }
    }

    /// IP address free fn
    ///
    /// # Safety
    ///
    /// Designed to be used from external code for IP address returned by ns_lookup.
    #[no_mangle]
    pub unsafe extern "C" fn free_ipaddr(data: *mut IPAddr) {
        let _ = Box::from_raw(data);
    }
}

/// C-string free fn
///
/// # Safety
///
/// Designed to be used from external code for returned C-strings.
#[cfg(any(feature = "read-file", feature = "ns-lookup"))]
#[no_mangle]
pub unsafe extern "C" fn free_cstr(data: *mut c_char) {
    use std::ffi::CString;

    let _ = CString::from_raw(data);
}
