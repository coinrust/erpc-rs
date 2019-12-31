use erpc_sys::ffi::{self};

pub struct AppContext {
    pub inner: *mut ffi::AppContext,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            inner: unsafe { ffi::app_context_new() },
        }
    }
}

impl Drop for AppContext {
    fn drop(&mut self) {
        unsafe {
            ffi::app_context_destroy(self.inner);
        }
    }
}