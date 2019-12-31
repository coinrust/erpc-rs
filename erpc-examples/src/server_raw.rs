use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
use std::ffi::CString;
use libc::{size_t};

extern fn req_handler(req_handle: *mut ffi::ReqHandle, context: *mut c_void) -> () {
    println!("req_handler start");
    let data: *mut u8;
    let data_size : size_t = 0;
    unsafe { data = ffi::erpc_get_req_msgbuf(req_handle, &data_size) };
    //println!("data_size: {:?} {}", data, data_size);

    let s = unsafe { String::from_raw_parts(data, data_size, 0) };
    println!("req: {}", s);

    let ctx: *mut ffi::AppContext = context as *mut ffi::AppContext;
    let _rpc = unsafe { ffi::app_context_rpc(ctx) };
    let s = "world".to_string();
    unsafe { ffi::erpc_enqueue_response(_rpc, req_handle, s.as_ptr(), s.len()) };
    println!("req_handler end");
}

extern fn sm_handler(_session_num: c_int, _sm_event_type: ffi::SmEventType, _sm_err_type: ffi::SmErrType, _context: *mut c_void) {
    println!("sm_handler");
}

fn main() {
    // sudo rxe_cfg start
    // sudo rxe_cfg status
    //let context = AppContext{};
    unsafe {
        let context = ffi::app_context_new();

        let local_uri = CString::new("127.0.0.1:31850").unwrap();
        let nexus = ffi::erpc_nexus_new(local_uri.as_ptr(), 0, 0);
        ffi::erpc_nexus_register_req_func(nexus, 1, req_handler, 0);
        let rpc = ffi::erpc_rpc_new(nexus, context, 0, sm_handler, 0);

        loop {
            ffi::erpc_rpc_run_event_loop(rpc, 1000);
        }

        //ffi::erpc_rpc_destroy(rpc);
        //ffi::erpc_nexus_destroy(nexus);
        //ffi::app_context_destroy(context);
    }
    //println!("OK!");
}
