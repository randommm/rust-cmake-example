use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "foo", kind = "static")]
extern "C" {
    pub fn pass_a_float(v: f32);
    pub fn pass_a_string(v: *const c_char);
}

pub fn main() {
    unsafe {
        pass_a_float(7.);
    };

    let some_string = "Hi there!\0";
    let c_string = CStr::from_bytes_with_nul(some_string.as_bytes()).unwrap();
    unsafe {
        pass_a_string(c_string.as_ptr());
    };
}
