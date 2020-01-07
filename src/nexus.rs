use erpc_sys::ffi::{self};
use std::ffi::CString;
use std::os::raw::c_void;

pub struct Nexus {
    pub inner: *mut ffi::Nexus,
    owner: bool,
}

impl Clone for Nexus {
    fn clone(&self) -> Nexus {
        Nexus {
            inner: self.inner,
            owner: false,
        }
    }
}

unsafe impl Send for Nexus {}

impl Nexus {
    pub fn new(local_uri: String, numa_node: usize, num_bg_threads: usize) -> Self {
        let local_uri = CString::new(local_uri).expect("");
        Nexus {
            inner: unsafe { ffi::erpc_nexus_new(local_uri.as_ptr(), numa_node, num_bg_threads) },
            owner: true,
        }
    }

    pub fn register_req_func(
        &self,
        req_type: u8,
        req_func: extern "C" fn(*mut ffi::ReqHandle, *mut c_void) -> (),
        req_func_type: u8,
    ) {
        unsafe { ffi::erpc_nexus_register_req_func(self.inner, req_type, req_func, req_func_type) }
    }
}

impl Drop for Nexus {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::erpc_nexus_destroy(self.inner) }
        }
    }
}
