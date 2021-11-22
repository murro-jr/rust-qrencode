use std::ffi::CString;

fn main() {
    let string = "Example String";
    let string = CString::new(string).expect("CString::new failed");

    unsafe {
        qrencode::encode_string(string.as_ptr(), 0, 0, 0);
    }
}
