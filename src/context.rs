use erpc_sys::ffi::{self};
use std::os::raw::{c_void};
use std::ffi::CString;

pub struct AppContext {
    pub inner: *mut ffi::AppContext,
    owner: bool,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            inner: unsafe { ffi::app_context_new() },
            owner: true,
        }
    }

    pub fn from_raw(context: *mut c_void) -> Self {
        let context: *mut ffi::AppContext = context as *mut ffi::AppContext;
        AppContext {
            inner: context,
            owner: false,
        }
    }

    pub fn get_session_num(&self) -> i32 {
        unsafe { ffi::app_context_get_session_num(self.inner) }
    }

    pub fn connect_session(&self, server_uri: String, rem_rpc_id: u8) -> i32 {
        let server_uri = CString::new(server_uri).expect("");
        unsafe { ffi::erpc_connect_session(self.inner, server_uri.as_ptr(), rem_rpc_id) }
    }

    pub fn get_resp_msgbuf(&self, _tag: usize) -> Vec<u8> {
        let data: *mut u8;
        let data_size : usize = 0;
        unsafe { data = ffi::erpc_get_resp_msgbuf(self.inner, &data_size) };
        //println!("data_size: {:?} {}", data, data_size);

        let s = unsafe { String::from_raw_parts(data, data_size, 0) };
        s.into_bytes()
    }
}

impl Drop for AppContext {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::app_context_destroy(self.inner) }
        }
    }
}