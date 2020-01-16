# erpc-rs
eRPC library for Rust

# Dependencies
```
sudo apt install libibverbs-dev libnuma-dev libgflags-dev libgtest-dev libboost-dev
```

# Installing rdma-core
```
sudo apt install libibverbs-dev libibverbs1 rdma-core ibverbs-utils
# start
sudo rxe_cfg start
  Name  Link  Driver     Speed  NMTU  IPv4_addr  RDEV  RMTU  
  eth0  yes   hv_netvsc
# add <interface_name>
sudo rxe_cfg add eth0
# status
sudo rxe_cfg status
  Name  Link  Driver     Speed  NMTU  IPv4_addr  RDEV  RMTU          
  eth0  yes   hv_netvsc                          rxe0  1024  (3)
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

# Build & Run
```
# use -DERPC_INFINIBAND=true
ERPC_INFINIBAND=true cargo build

# use -DERPC_RAW=true
ERPC_RAW=true cargo build

# use -DERPC_DPDK=true
ERPC_DPDK=true cargo build

# Set up hugepages
sudo bash -c "echo 1024 > /sys/devices/system/node/node0/hugepages/hugepages-2048kB/nr_hugepages"

# Build examples
cargo build --examples

# Run examples
sudo ./target/debug/examples/server
sudo ./target/debug/examples/client
```

# Server
```rust,editable
use erpc_rs::context::AppContext;
use erpc_rs::nexus::Nexus;
use erpc_rs::reqhandle::ReqHandle;
use erpc_rs::rpc::Rpc;
use erpc_sys::ffi;
use std::os::raw::{c_int, c_void};
use std::thread;
use std::thread::JoinHandle;

const LOCAL_URI: &str = "127.0.0.1:31850";

extern "C" fn req_handler(req_handle: *mut ffi::ReqHandle, context: *mut c_void) -> () {
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

extern "C" fn sm_handler(
    _session_num: c_int,
    _sm_event_type: ffi::SmEventType,
    _sm_err_type: ffi::SmErrType,
    _context: *mut c_void,
) {
    println!("sm_handler");
}

fn main() {
    let nexus = Nexus::new(LOCAL_URI.to_string(), 0, 0);
    nexus.register_req_func(1, req_handler, 0);

    let mut wait_vec: Vec<JoinHandle<()>> = Vec::new();

    let num_threads = 2;

    for i in 0..num_threads {
        let context = AppContext::new();
        let nexus = nexus.clone();

        let handle = thread::spawn(move || {
            let rpc = Rpc::new(&context, &nexus, i, sm_handler, 0);
            loop {
                rpc.run_event_loop(1000);
            }
        });

        wait_vec.push(handle);
    }
    for handle in wait_vec {
        handle.join().unwrap();
    }
}
```

# Client
```rust,editable
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
    let context = AppContext::from_raw(_context);
    let tag = tag as usize;

    let msg_buffs = MsgBuffers::from_context(&context, tag);
    let s = msg_buffs.get_resp_msgbuf();
    let s = String::from_utf8(s).expect("");
    println!("cont_func tag: {} resp: {}", tag, s);

    let session_num = context.get_session_num();
    let rpc = Rpc::from_context(&context);
    let s = "hello".to_string().into_bytes();
    rpc.enqueue_request(&context, session_num, 1, s, cont_func, 1000, 0);
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
```
