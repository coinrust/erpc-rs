use erpc_rs::context::AppContext;
use erpc_rs::msgbuffs;
use erpc_rs::nexus::Nexus;
use erpc_rs::rpc::Rpc;
use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};

use msgbuffs::MsgBuffers;
use std::thread;
use std::thread::JoinHandle;

const LOCAL_URI: &str = "127.0.0.1:31851";
const SERVER_URI: &str = "127.0.0.1:31850";

extern "C" fn sm_handler(
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

extern "C" fn cont_func(_context: *mut c_void, tag: *mut c_void) {
    println!("cont_func start");
    let context = AppContext::from_raw(_context);
    let tag = tag as usize;

    let msg_buffs = MsgBuffers::from_context(&context, tag);
    let s = msg_buffs.get_resp_msgbuf();
    let s = String::from_utf8(s).expect("");
    println!("cont_func tag: {} resp: {}", tag, s);

    println!("cont_func end");
}

fn main() {
    let nexus = Nexus::new(LOCAL_URI.to_string(), 0, 0);

    let mut wait_vec: Vec<JoinHandle<()>> = Vec::new();

    let num_threads = 2;

    for i in 0..num_threads {
        let context = AppContext::new();
        let nexus = nexus.clone();

        let handle = thread::spawn(move || {
            let rpc = Rpc::new(&context, &nexus, i, sm_handler, 0);

            let session_num = context.connect_session(SERVER_URI.to_string(), 0);

            println!("session_num: {}", session_num);

            while !rpc.is_connected(session_num) {
                rpc.run_event_loop_once();
            }

            println!("connected");

            loop {
                rpc.run_event_loop(1000);
                let s = "hello".to_string().into_bytes();
                rpc.enqueue_request(&context, session_num, 1, s, cont_func, 0, 8);
            }
        });

        wait_vec.push(handle);
    }

    for handle in wait_vec {
        handle.join().unwrap();
    }
}
