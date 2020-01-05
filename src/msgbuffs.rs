use erpc_sys::ffi::{self};
use crate::context::AppContext;

pub struct MsgBuffers {
    pub inner: *mut ffi::MsgBuffers,
}

impl MsgBuffers {
    pub fn from_context(context: &AppContext, tag: usize) -> Self {
        let msg_buffs = unsafe { ffi::erpc_msgbuffs_get_by_tag(context.inner, tag) };
        MsgBuffers {
            inner: msg_buffs,
        }
    }

    pub fn get_req_msgbuf(&self) -> Vec<u8> {
        let data: *mut u8;
        let data_size : usize = 0;
        unsafe { data = ffi::erpc_msgbuffs_req_msgbuf(self.inner, &data_size) };

        let s = unsafe { String::from_raw_parts(data, data_size, 0) };
        s.into_bytes()
    }

    pub fn get_resp_msgbuf(&self) -> Vec<u8> {
        let data: *mut u8;
        let data_size : usize = 0;
        unsafe { data = ffi::erpc_msgbuffs_resp_msgbuf(self.inner, &data_size) };

        let s = unsafe { String::from_raw_parts(data, data_size, 0) };
        s.into_bytes()
    }
}

impl Drop for MsgBuffers {
    fn drop(&mut self) {
        unsafe { ffi::erpc_msgbuffs_destroy(self.inner) }
    }
}