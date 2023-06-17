//! widely used types and functions

#[cfg(feature = "no_std")]
pub use alloc::{boxed::Box, sync::Arc};

#[cfg(not(feature = "no_std"))]
pub use std::{boxed::Box, sync::Arc};

pub use core::pin::Pin;

pub use core::{
    future::Future,
    mem::transmute,
    ptr::null_mut,
    task::{Context, Poll},
};

#[cfg(not(feature = "spin"))]
pub use std::sync::{Mutex, MutexGuard};

#[cfg(feature = "spin")]
pub use spin::{Mutex, MutexGuard};

#[cfg(not(feature = "woke"))]
pub use futures_task::{waker_ref, ArcWake as Wake};

#[cfg(feature = "woke")]
pub use woke::{waker_ref, Woke as Wake};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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
