use core::ffi::c_void;

/// Raw C userdata type
pub type RawUserData = *mut c_void;

/// C userdata type
#[repr(transparent)]
pub struct UserData(RawUserData);

unsafe impl Send for UserData {}
unsafe impl Sync for UserData {}

impl From<RawUserData> for UserData {
    fn from(raw: RawUserData) -> Self {
        Self(raw)
    }
}

impl Into<RawUserData> for UserData {
    fn into(self) -> RawUserData {
        self.0
    }
}

impl core::ops::Deref for UserData {
    type Target = RawUserData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for UserData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
