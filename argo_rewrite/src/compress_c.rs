//! From `compress.c`.

// ── `if (len < 2 || buf[0] != 0x1f)` · compress.c:39-39 · chunk 401 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (len < 2 || buf[0] != 0x1f) return ALGORITHM_UNKNOWN;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk401 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn detect_algorithm__part401(buf: *const u8, len: c_int) -> c_int {
    if len < 2 || unsafe { *buf } != 0x1f {
        return 1;
    }
    0
}
}

// ── `switch (buf[1])` · compress.c:41-45 · chunk 402 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (buf[1]) {
//             case 0x08: return ALGORITHM_DEFLATE;
//             case 0x8b: return ALGORITHM_GZIP;
//             default: return ALGORITHM_UNKNOWN;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk402 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn detect_algorithm__part402(buf: *const core::ffi::c_uchar, len: c_int) -> c_int {
    let byte = unsafe { core::ptr::read_volatile(buf.offset(1)) };
    match byte {
        0x08 => 1,
        0x8b => 2,
        _ => 0,
    }
}
}

// ── `return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;` · compress.c:50-50 · chunk 404 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk404 {
use core::ffi::c_uchar;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn compress_detect_magic__part404(buf: *const c_uchar, len: c_int) -> c_int {
    if len < 2 {
        return 0;
    }
    let first = unsafe { *buf };
    if first != 0x1f {
        return 0;
    }
    let second = unsafe { *buf.offset(1) };
    match second {
        0x08 | 0x8b => 1,
        _ => 0,
    }
}
}

// ── `switch (detect_algorithm(in_buf, in_len))` · compress.c:57-66 · chunk 406 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch (detect_algorithm(in_buf, in_len)) {
//             case ALGORITHM_UNKNOWN:
//                 return -1;
//             case ALGORITHM_DEFLATE:
//                 windowBits = -
//                              15
//                                       ;
//                 break;
//             case ALGORITHM_GZIP:
//                 windowBits = 
//                             15 
//                                       | 16;
//                 break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk406 {
use core::ffi::c_int;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn decompress__part406(in_buf: *const c_uchar, in_len: c_int) -> c_int {
    unsafe {
        if in_len < 2 || *in_buf != 0x1f {
            return -1;
        }
        let second = *in_buf.offset(1);
        match second {
            0x08 => -15,
            0x8b => 31,
            _ => -1,
        }
    }
}
}

// ── `z_stream strm; strm.zalloc = 0 ; strm.zfree = 0 ; strm.opaque = 0 ; strm.avail_in = 0; strm.nex…` · compress.c:68-73 · chunk 407 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     z_stream strm;
//         strm.zalloc = 
//                      0
//                            ;
//         strm.zfree = 
//                     0
//                           ;
//         strm.opaque = 
//                      0
//                            ;
//         strm.avail_in = 0;
//         strm.next_in = 
//                       0
//                             ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk407 {
use core::ffi::{c_uchar, c_uint, c_ulong, c_char, c_void, c_int};
use core::ptr::null_mut;

#[repr(C)]
struct z_stream_s {
    next_in: *mut c_uchar,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_uchar,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: *mut c_void,
    zfree: *mut c_void,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

#[no_mangle]
pub extern "C" fn decompress__part407(strm: *mut z_stream_s) {
    unsafe {
        (*strm).zalloc = null_mut();
        (*strm).zfree = null_mut();
        (*strm).opaque = null_mut();
        (*strm).avail_in = 0;
        (*strm).next_in = null_mut();
    }
}
}

// ── `if (inflateInit2_(( &strm ), ( windowBits ), "1.3.2", (int)sizeof(z_stream)) != 0)` · compress.c:74-74 · chunk 408 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            inflateInit2_((
//            &strm
//            ), (
//            windowBits
//            ), "1.3.2", (int)sizeof(z_stream)) 
//                                            != 
//                                               0
//                                                   ) return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk408 {
use std::os::raw::{c_int, c_uint, c_ulong, c_char, c_void};
use std::ffi::CString;

#[repr(C)]
struct z_stream {
    next_in: *mut c_char,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_char,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: extern "C" fn(*mut c_void, c_uint, c_ulong) -> c_ulong,
    zfree: extern "C" fn(*mut c_void, *mut c_void),
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

extern "C" {
    fn inflateInit2_(
        strm: *mut z_stream,
        windowBits: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn decompress__part408(
    strm: *mut z_stream,
    windowBits: c_int,
) -> c_int {
    let version = CString::new("1.3.2").unwrap();
    unsafe { inflateInit2_(strm, windowBits, version.as_ptr(), std::mem::size_of::<z_stream>() as c_int) }
}
}

// ── `strm.avail_in = in_len; strm.next_in = (Bytef *)in_buf; strm.avail_out = out_len; strm.next_out…` · compress.c:76-79 · chunk 409 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     strm.avail_in = in_len;
//         strm.next_in = (Bytef *)in_buf;
//         strm.avail_out = out_len;
//         strm.next_out = (Bytef *)out_buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk409 {
use std::os::raw::{c_int, c_uint, c_ulong, c_char, c_void, c_uchar};

/// Mirror of C z_stream struct for FFI compatibility with zlib.
// Layout matches the C zlib z_stream struct with #[repr(C)].
#[repr(C)]
struct z_stream {
    next_in: *mut c_uchar,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_uchar,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: Option<extern "C" fn(c_uint, c_uint, c_void)>,
    zfree: Option<extern "C" fn(c_void, *mut c_void)>,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

#[no_mangle]
pub extern "C" fn decompress__part409(
    strm: *mut z_stream,
    in_buf: *const c_uchar,
    in_len: c_int,
    out_buf: *const c_uchar,
    out_len: c_int,
) {
    unsafe {
        if !strm.is_null() {
            (&mut *strm).avail_in = in_len as c_uint;
            (&mut *strm).next_in = in_buf as *mut c_uchar;
            (&mut *strm).avail_out = out_len as c_uint;
            (&mut *strm).next_out = out_buf as *mut c_uchar;
        }
    }
}
}

// ── `if (inflate(&strm, 4 ) != 1)` · compress.c:80-84 · chunk 410 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (inflate(&strm, 
//                           4
//                                   ) != 
//                                        1
//                                                    ) {
//             fprintf(
//                    stderr
//                          , "inflate() failed: %s\n", strm.msg);
//             inflateEnd(&strm);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk410 {
use core::ffi::{c_int, c_uint, c_ulong, c_char, c_void};

// Mirror of C z_stream struct for ABI compatibility
#[repr(C)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut u8,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: *mut c_void,
    zfree: *mut c_void,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

extern "C" {
    pub fn inflate(strm: *mut z_stream_s, window_bits: c_int) -> c_int;
    pub fn inflateEnd(strm: *mut z_stream_s) -> c_int;
}

#[no_mangle]
pub extern "C" fn decompress__part410(strm: *mut z_stream_s) -> c_int {
    unsafe {
        if inflate(strm, 4) != 1 {
            inflateEnd(strm);
            return -1;
        }
    }
    0
}
}

// ── `return strm.total_out;` · compress.c:86-86 · chunk 411 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     return strm.total_out;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk411 {
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn decompress__part411(total_out: c_ulong) -> c_ulong {
    total_out
}
}

