use erpc_sys::ffi::{self};
use std::os::raw::{c_int, c_void};
use std::ffi::CString;
use crate::context::AppContext;
use crate::nexus::Nexus;
use crate::reqhandle::ReqHandle;

pub struct Rpc {
    inner: *mut ffi::Rpc,
    owner: bool,
}

impl Rpc {
    pub fn new(context: &AppContext, nexus: &Nexus, rpc_id: u8,
                      sm_handler: extern fn(c_int, ffi::SmEventType, ffi::SmErrType, *mut c_void), phy_port: u8) -> Self {
        let rpc = unsafe { ffi::erpc_rpc_new(nexus.inner, context.inner, rpc_id, sm_handler, phy_port) };
        Rpc{inner: rpc, owner: true}
    }

    pub fn from_context(context: &AppContext) -> Self {
        let rpc = unsafe { ffi::app_context_rpc(context.inner) };
        Rpc{inner: rpc, owner: false}
    }

    pub fn connect_session(&mut self, server_uri: String, rem_rpc_id: u8) -> i32 {
        let server_uri = CString::new(server_uri).expect("");
        unsafe { ffi::erpc_connect_session(self.inner, server_uri.as_ptr(), rem_rpc_id) }
    }

    pub fn is_connected(&mut self, session_num: i32) -> bool {
        unsafe { ffi::erpc_rpc_is_connected(self.inner, session_num) }
    }

    pub fn run_event_loop_once(&mut self) -> () {
        unsafe { ffi::erpc_run_event_loop_once(self.inner) };
    }

    pub fn run_event_loop(&mut self, timeout_ms: usize) -> () {
        unsafe { ffi::erpc_rpc_run_event_loop(self.inner, timeout_ms) };
    }

    pub fn enqueue_request(&mut self, context: &AppContext, session_num: i32, req_type: u8, data: Vec<u8>, cont_func: extern fn(*mut c_void, *mut c_void),
                           tag: usize, cont_etid: usize) -> () {
        unsafe { ffi::erpc_enqueue_request(context.inner, self.inner, session_num, req_type, data.as_ptr(), data.len(), cont_func, tag, cont_etid) }
    }

    pub fn enqueue_response(&mut self, req_handle: &ReqHandle, data: Vec<u8>) -> () {
        unsafe { ffi::erpc_enqueue_response(self.inner, req_handle.inner, data.as_ptr(), data.len()) }
    }
}

impl Drop for Rpc {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::erpc_rpc_destroy(self.inner) }
        }
    }
}