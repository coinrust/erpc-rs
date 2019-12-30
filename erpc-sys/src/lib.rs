pub mod ffi;

#[cfg(test)]
mod tests {
    use crate::ffi::server_test;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn f() {
        unsafe {
            server_test();
        }
    }
}
