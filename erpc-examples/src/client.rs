use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
//use std::thread;
use libc::{size_t};
use erpc_rs::context::AppContext;
use erpc_rs::rpc::Rpc;
use erpc_rs::nexus::Nexus;

extern fn sm_handler(session_num: c_int, sm_event_type: ffi::SmEventType, sm_err_type: ffi::SmErrType, context: *mut c_void) {
    println!("sm_handler session_num: {} sm_event_type: {} sm_err_type: {}", session_num, sm_event_type, sm_err_type);
    let _ctx: *mut ffi::AppContext = context as *mut ffi::AppContext;
}

extern fn cont_func(_context: *mut c_void, _tag: *mut c_void) {
    let ctx: *mut ffi::AppContext = _context as *mut ffi::AppContext;
    let tag = _tag as size_t;

    let data: *mut u8;
    let data_size : size_t = 0;
    unsafe { data = ffi::erpc_get_resp_msgbuf(ctx, &data_size) };
    //println!("data_size: {:?} {}", data, data_size);

    let s = unsafe { String::from_raw_parts(data, data_size, 0) };
    println!("cont_func tag: {} resp: {}", tag, s);
}

fn main() {
    let context = AppContext::new();
    let nexus = Nexus::new("127.0.0.1:31851".to_string(), 0, 0);
    let mut rpc = Rpc::new(&context, &nexus, 0, sm_handler, 0);

    let session_num = rpc.connect_session("127.0.0.1:31850".to_string(), 0);

    println!("session_num: {}", session_num);

    while !rpc.is_connected(session_num) {
        rpc.run_event_loop_once();
    }

    println!("connected");

    let s = "hello".to_string();
    rpc.enqueue_request(&context, session_num, 1, s.as_ptr(), s.len(), cont_func, 1000, 0);
    rpc.run_event_loop(1000*5);

    println!("OK");
}
