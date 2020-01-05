# erpc-rs
eRPC library for Rust

# Installing rdma-core
```
curl -s https://packagecloud.io/install/repositories/linux-rdma/rdma-core/script.deb.sh | sudo bash
sudo apt install rdma-core
# start
sudo rxe_cfg start
```

# Installing junction & turf
```
$ git clone https://github.com/preshing/junction.git
$ git clone https://github.com/preshing/turf.git
$ cd junction
$ mkdir build
$ cd build
$ cmake -DCMAKE_INSTALL_PREFIX=~/junction-install -DJUNCTION_WITH_SAMPLES=OFF ..
$ cmake --build . --target install --config RelWithDebInfo
```

# Installing eRPC
```
sudo apt-get install libboost-dev libboost-filesystem-dev libboost-thread-dev libboost-program-options-dev libboost-python-dev libboost-dev

# Installing googletest
sudo apt install libgtest-dev build-essential cmake
cd /usr/src/googletest
sudo cmake .
sudo cmake --build . --target install

cd ~/
git clone https://github.com/erpc-io/eRPC.git
cd eRPC/
cmake . -DPERF=OFF -DTRANSPORT=infiniband -DROCE=on; make -j;
```

# Server
```rust,editable
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
```

# Client
```rust,editable
use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
use erpc_rs::context::AppContext;
use erpc_rs::msgbuffs;
use erpc_rs::rpc::Rpc;
use erpc_rs::nexus::Nexus;

use msgbuffs::MsgBuffers;

extern fn sm_handler(session_num: c_int, sm_event_type: ffi::SmEventType, sm_err_type: ffi::SmErrType, context: *mut c_void) {
    println!("sm_handler session_num: {} sm_event_type: {} sm_err_type: {}", session_num, sm_event_type, sm_err_type);
    let _ctx: *mut ffi::AppContext = context as *mut ffi::AppContext;
}

extern fn cont_func(_context: *mut c_void, tag: *mut c_void) {
    let context = AppContext::from_raw(_context);
    let tag = tag as usize;

    let msg_buffs = MsgBuffers::from_context(&context, tag);
    let s = msg_buffs.get_resp_msgbuf();
    //let s = context.get_resp_msgbuf(tag);
    let s = String::from_utf8(s).expect("");
    println!("cont_func tag: {} resp: {}", tag, s);

    let session_num = context.get_session_num();
    let rpc = Rpc::from_context(&context);
    let s = "hello".to_string().into_bytes();
    rpc.enqueue_request(&context, session_num, 1, s, cont_func, 1000, 0);
}

fn main() {
    let context = AppContext::new();
    let nexus = Nexus::new("127.0.0.1:31851".to_string(), 0, 0);
    let rpc = Rpc::new(&context, &nexus, 0, sm_handler, 0);

    let session_num = context.connect_session("127.0.0.1:31850".to_string(), 0);

    println!("session_num: {}", session_num);

    while !rpc.is_connected(session_num) {
        rpc.run_event_loop_once();
    }

    println!("connected");

    let s = "hello".to_string().into_bytes();
    rpc.enqueue_request(&context, session_num, 1, s, cont_func, 1000, 0);
    rpc.run_event_loop(1000*5);

    println!("OK");
}
```