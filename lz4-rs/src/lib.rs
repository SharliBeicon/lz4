use std::ffi::c_int;

pub const LZ4_MAX_INPUT_SIZE: u32 = 0x7E00_0000;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LZ4_rs_compressBound(isize: c_int) -> c_int {
    lz4_compress_bound(isize)
}

pub const fn lz4_compress_bound(input_size: c_int) -> c_int {
    if (input_size as u32) > LZ4_MAX_INPUT_SIZE {
        0
    } else {
        input_size + (input_size / 255) + 16
    }
}
