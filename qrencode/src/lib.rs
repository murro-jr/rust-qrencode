#![no_std]
use libc::{c_char, c_int, c_uchar, c_void};

extern "C" {
    pub fn encode_string(
        string: *mut c_char,
        version: c_int,
        level: c_int,
        case_senstive: c_int,
    ) -> *mut c_void;

    pub fn encode_string_8bit(string: *mut c_char, version: c_int, level: c_int) -> *mut c_void;

    pub fn encode_string_mqr(
        string: *mut c_char,
        version: c_int,
        level: c_int,
        case_senstive: c_int,
    ) -> *mut c_void;

    pub fn encode_string_8bit_mqr(string: *mut c_char, version: c_int, level: c_int)
        -> *mut c_void;

    pub fn encode_data(
        size: c_int,
        data: *mut c_uchar,
        version: c_int,
        level: c_int,
    ) -> *mut c_void;

    pub fn encode_data_mqr(
        size: c_int,
        data: *mut c_uchar,
        version: c_int,
        level: c_int,
    ) -> *mut c_void;

    pub fn free(qrcode: *mut c_void) -> c_void;
}
