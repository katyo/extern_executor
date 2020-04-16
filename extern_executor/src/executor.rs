use crate::{
    BoxFuture,
    UserData, RawUserData,
    Arc, Mutex, mutex_lock,
    Context, Poll,
    Wake, waker_ref,
};

/// Raw C wake function
///
/// This function will be called when pending future need to be polled again
pub type RawWakeFn = fn(RawUserData);

/// Raw C poll function
///
/// This function must be called to poll future on each wake event
pub type RawPollFn = fn(RawUserData) -> bool;

/// Raw C drop function
///
/// This function must be called to cleanup either pending or completed future
pub type RawDropFn = fn(RawUserData);

/// Create task for polling specified future by external event loop
pub fn task_new_raw<T>(future: BoxFuture<'static, T>, wake: RawWakeFn, data: RawUserData) -> (RawPollFn, RawDropFn, RawUserData) {
    let data = UserData::from(data);
    let notify = || { wake(*data); };

    let task = Arc::new(Task::new(future, notify));

    PollerFn::from(task.poller()).into()
}

struct Task<T, U> {
    future: Mutex<BoxFuture<'static, T>>,
    notify: Mutex<U>,
}

impl<T, U> Wake for Task<T, U>
where
    U: FnMut() + Send,
{
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let notify = &mut *mutex_lock(&arc_self.notify);
        notify();
    }
}

impl<T, U> Task<T, U>
where
    U: FnMut() + Send,
{
    pub fn new(future: BoxFuture<'static, T>, notify: U) -> Self {
        let future = Mutex::new(future);
        let notify = Mutex::new(notify);
        Self { future, notify }
    }

    pub fn poll(self: &Arc<Self>) -> bool {
        let mut future = mutex_lock(&self.future);
        let waker = waker_ref(&self);
        let context = &mut Context::from_waker(&*waker);

        if let Poll::Pending = future.as_mut().poll(context) {
            true
        } else {
            false
        }
    }

    pub fn poller(self: &Arc<Self>) -> impl FnMut() -> bool {
        let cloned_self = self.clone();
        move || cloned_self.poll()
    }
}

#[repr(transparent)]
pub struct PollerFn<U> {
    func: U,
}

impl<U> From<U> for PollerFn<U>
where
    U: FnMut() -> bool,
{
    fn from(func: U) -> Self {
        Self { func }
    }
}

impl<U> Into<(RawPollFn, RawDropFn, RawUserData)> for PollerFn<U>
where
    U: FnMut() -> bool,
{
    fn into(self) -> (RawPollFn, RawDropFn, RawUserData) {
        let poller = Box::into_raw(Box::new(self.func));
        (Self::poll_fn, Self::drop_fn, poller as RawUserData)
    }
}

impl<U> PollerFn<U>
where
    U: FnMut() -> bool,
{
    fn poll_fn(data: RawUserData) -> bool {
        let poller = unsafe { &mut *(data as *mut U) };
        poller()
    }

    fn drop_fn(data: RawUserData) {
        let _ = unsafe { Box::from_raw(data as *mut U) };
    }
}
