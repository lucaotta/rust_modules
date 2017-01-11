extern crate libc;
// This will not work without a self::
// It's ok if the libc crate is imported in main.rs
use libc::c_int;

pub fn compute() -> c_int {
    return 4 as c_int;
}