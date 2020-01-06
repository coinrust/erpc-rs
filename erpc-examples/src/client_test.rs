use erpc_sys::ffi;

fn main() {
    unsafe {
        ffi::client_test();
    }
}
