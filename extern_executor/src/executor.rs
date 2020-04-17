use crate::{
    BoxFuture,
    UserData, RawUserData,
    Arc, Mutex, mutex_lock,
    Context, Poll,
    Wake, waker_ref, wake,
};

/// Raw C wake function
///
/// This function will be called when pending future need to be polled again
pub type RawTaskWakeFn = fn(RawUserData);

/// Raw C poll function
///
/// This function must be called to poll future on each wake event
pub type RawTaskPollFn = fn(RawUserData) -> bool;

/// Raw C drop function
///
/// This function must be called to cleanup either pending or completed future
pub type RawTaskDropFn = fn(RawUserData);

/// Task handle
pub(crate) type BoxedTask = Arc<Task>;

/// Create task for polling specified future by external event loop
pub fn task_new_raw(future: BoxFuture<'static, ()>, data: RawUserData) -> RawUserData {
    let data = UserData::from(data);
    let task = Arc::new(Task::new(future, data));

    Box::into_raw(Box::new(task)) as RawUserData
}

pub(crate) struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    data: UserData,
}

impl Wake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        wake(*arc_self.data);
    }
}

impl Task {
    pub fn new(future: BoxFuture<'static, ()>, data: UserData) -> Self {
        let future = Mutex::new(future);
        Self { future, data }
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
}
