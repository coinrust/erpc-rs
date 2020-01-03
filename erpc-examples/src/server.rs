use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
use erpc_rs::context::AppContext;
use erpc_rs::nexus::Nexus;
use erpc_rs::reqhandle::ReqHandle;
use erpc_rs::rpc::Rpc;

extern fn req_handler(req_handle: *mut ffi::ReqHandle, context: *mut c_void) -> () {
    println!("req_handler start");
    let req_handle = ReqHandle::from_raw(req_handle);
    let s = req_handle.get_req_msgbuf();
    println!("req: {}", String::from_utf8(s).expect(""));

    let c = AppContext::from_raw(context);
    let r = Rpc::from_context(&c);
    let s = "world".to_string().into_bytes();
    r.enqueue_response(&req_handle, s);
    println!("req_handler end");
}

extern fn sm_handler(_session_num: c_int, _sm_event_type: ffi::SmEventType, _sm_err_type: ffi::SmErrType, _context: *mut c_void) {
    println!("sm_handler");
}

fn main() {
    // sudo rxe_cfg start
    // sudo rxe_cfg status
    let context = AppContext::new();
    let nexus = Nexus::new("127.0.0.1:31850".to_string(), 0, 0);

    nexus.register_req_func(1, req_handler, 0);

    let rpc = Rpc::new(&context, &nexus, 0, sm_handler, 0);

    loop {
        rpc.run_event_loop(1000);
    }
}
