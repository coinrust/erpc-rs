use erpc_sys::ffi;

fn main() {
    unsafe {
        ffi::server_test();
    }
}
