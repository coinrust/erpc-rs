use erpc_sys::ffi::{self};

pub struct ReqHandle {
    pub inner: *mut ffi::ReqHandle,
}

impl ReqHandle {
    pub fn from_raw(req_handle: *mut ffi::ReqHandle) -> Self {
        ReqHandle {
            inner: req_handle,
        }
    }

    pub fn get_req_msgbuf(&mut self) -> Vec<u8> {
        let data: *mut u8;
        let data_size : usize = 0;
        unsafe { data = ffi::erpc_get_req_msgbuf(self.inner, &data_size) };
        //println!("data_size: {:?} {}", data, data_size);
        let s = unsafe { String::from_raw_parts(data, data_size, 0) };
        s.into_bytes()
    }
}

impl Drop for ReqHandle {
    fn drop(&mut self) {}
}