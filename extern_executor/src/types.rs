///! widely used types and functions

#[cfg(feature = "no_std")]
pub use alloc::sync::Arc;

#[cfg(not(feature = "no_std"))]
pub use std::sync::Arc;

#[cfg(feature = "no_std")]
pub use core::pin::Pin;

pub use core::{
    ptr::null_mut,
    mem::transmute,
    future::Future,
    task::{Context, Poll},
};

#[cfg(not(feature = "spin"))]
pub use std::sync::{Mutex, MutexGuard};

#[cfg(feature = "spin")]
pub use spin::{Mutex, MutexGuard};

#[cfg(not(feature = "woke"))]
pub use futures::task::{ArcWake as Wake, waker_ref};

#[cfg(feature = "woke")]
pub use woke::{Woke as Wake, waker_ref};

#[cfg(feature = "no_std")]
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[cfg(not(feature = "no_std"))]
pub use futures::future::BoxFuture;

pub fn mutex_lock<T>(mutex: &Mutex<T>) -> MutexGuard<T> {
    #[cfg(not(feature = "spin"))]
    {
        mutex.lock().unwrap()
    }

    #[cfg(feature = "spin")]
    {
        mutex.lock()
    }
}
