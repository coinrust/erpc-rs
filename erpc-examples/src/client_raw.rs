use erpc_sys::ffi;
use libc::size_t;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

const LOCAL_URI: &str = "127.0.0.1:31851";
const SERVER_URI: &str = "127.0.0.1:31850";

extern "C" fn sm_handler_raw(
    session_num: c_int,
    sm_event_type: ffi::SmEventType,
    sm_err_type: ffi::SmErrType,
    context: *mut c_void,
) {
    println!(
        "sm_handler session_num: {} sm_event_type: {} sm_err_type: {}",
        session_num, sm_event_type, sm_err_type
    );
    let _ctx: *mut ffi::AppContext = context as *mut ffi::AppContext;
}

extern "C" fn cont_func(_context: *mut c_void, _tag: *mut c_void) {
    let ctx: *mut ffi::AppContext = _context as *mut ffi::AppContext;
    let tag = _tag as size_t;

    let data: *mut u8;
    let data_size: size_t = 0;
    let msgbufs = unsafe { ffi::erpc_msgbuffs_get_by_tag(ctx, tag) };
    data = unsafe { ffi::erpc_msgbuffs_resp_msgbuf(msgbufs, &data_size) };

    let s = unsafe { String::from_raw_parts(data, data_size, 0) };
    println!("cont_func tag: {} resp: {}", tag, s);

    unsafe { ffi::erpc_msgbuffs_destroy(msgbufs) };
}

fn main() {
    unsafe {
        let context = ffi::app_context_new();

        let local_uri = CString::new(LOCAL_URI).unwrap();
        let nexus = ffi::erpc_nexus_new(local_uri.as_ptr(), 0, 0);
        let rpc = ffi::erpc_rpc_new(nexus, context, 0, sm_handler_raw, 0);

        let server_uri = CString::new(SERVER_URI).unwrap();
        let session_num = ffi::erpc_connect_session(context, server_uri.as_ptr(), 0);
        println!("session_num: {}", session_num);

        // while (!rpc->is_connected(session_num)) rpc->run_event_loop_once();
        while !ffi::erpc_rpc_is_connected(rpc, session_num) {
            ffi::erpc_run_event_loop_once(rpc);
        }
        println!("connected");

        let s = "hello".to_string();
        ffi::erpc_enqueue_request(
            context,
            rpc,
            session_num,
            1,
            s.as_ptr(),
            s.len(),
            cont_func,
            1000,
            0,
        );

        ffi::erpc_rpc_run_event_loop(rpc, 1000 * 5);

        ffi::erpc_rpc_destroy(rpc);
        ffi::erpc_nexus_destroy(nexus);
        ffi::app_context_destroy(context);
    }
}
