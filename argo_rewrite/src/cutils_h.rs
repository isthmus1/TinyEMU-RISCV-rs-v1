//! From `cutils.h`.

// ── `if (a > b) … else` · cutils.h:71-74 · chunk 118 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (a > b)
//             return a;
//         else
//             return b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk118 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn max_int__part118(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}
}

// ── `if (a < b) … else` · cutils.h:79-82 · chunk 120 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (a < b)
//             return a;
//         else
//             return b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk120 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn min_int__part120(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}
}

// ── `return ptr[0] | (ptr[1] << 8);` · cutils.h:109-109 · chunk 122 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     return ptr[0] | (ptr[1] << 8);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk122 {
#[no_mangle]
pub extern "C" fn get_le16__part122(ptr: *const u8) -> u16 {
    unsafe {
        let b0 = *ptr;
        let b1 = *ptr.add(1);
        (b0 as u16) | ((b1 as u16) << 8)
    }
}
}

// ── `return ptr[0] | (ptr[1] << 8) | (ptr[2] << 16) | (ptr[3] << 24);` · cutils.h:114-114 · chunk 124 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     return ptr[0] | (ptr[1] << 8) | (ptr[2] << 16) | (ptr[3] << 24);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk124 {
// Port of get_le32 to Rust
#[no_mangle]
pub extern "C" fn get_le32__part124(ptr: *const u8) -> u32 {
    unsafe {
        let b0 = *ptr;
        let b1 = *ptr.add(1);
        let b2 = *ptr.add(2);
        let b3 = *ptr.add(3);
        (b0 as u32) | ((b1 as u32) << 8) | ((b2 as u32) << 16) | ((b3 as u32) << 24)
    }
}
}

// ── `return get_le32(ptr) | ((uint64_t)get_le32(ptr + 4) << 32);` · cutils.h:119-119 · chunk 126 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return get_le32(ptr) | ((uint64_t)get_le32(ptr + 4) << 32);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk126 {
#[no_mangle]
pub extern "C" fn get_le64__part126(ptr: *const u8) -> u64 {
    unsafe {
        let lo = *ptr as u64
            | (*(ptr.add(1)) as u64) << 8
            | (*(ptr.add(2)) as u64) << 16
            | (*(ptr.add(3)) as u64) << 24;
        let hi = *ptr.add(4) as u64
            | (*(ptr.add(5)) as u64) << 8
            | (*(ptr.add(6)) as u64) << 16
            | (*(ptr.add(7)) as u64) << 24;
        lo | (hi << 32)
    }
}
}

// ── `ptr[0] = v; ptr[1] = v >> 8;` · cutils.h:124-125 · chunk 128 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ptr[0] = v;
//         ptr[1] = v >> 8;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk128 {
// Automatically generated Rust rewrite of put_le16__part128
#[no_mangle]
pub extern "C" fn put_le16__part128(ptr: *mut u8, v: u16) {
    unsafe {
        *ptr = v as u8;
        *ptr.add(1) = (v >> 8) as u8;
    }
}
}

// ── `ptr[0] = v; ptr[1] = v >> 8; ptr[2] = v >> 16; ptr[3] = v >> 24;` · cutils.h:130-133 · chunk 130 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     ptr[0] = v;
//         ptr[1] = v >> 8;
//         ptr[2] = v >> 16;
//         ptr[3] = v >> 24;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk130 {
/// Port of put_le32 body to Rust - writes a u32 as little-endian bytes
#[no_mangle]
pub extern "C" fn put_le32__part130(ptr: *mut u8, v: u32) {
    unsafe {
        let mut p = ptr;
        p = p.wrapping_add(0);
        *p = v as u8;
        p = p.wrapping_add(1);
        *p = (v >> 8) as u8;
        p = p.wrapping_add(1);
        *p = (v >> 16) as u8;
        p = p.wrapping_add(1);
        *p = (v >> 24) as u8;
    }
}
}

// ── `put_le32(ptr, v); put_le32(ptr + 4, v >> 32);` · cutils.h:138-139 · chunk 132 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le32(ptr, v);
//         put_le32(ptr + 4, v >> 32);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk132 {
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ulonglong;

#[no_mangle]
pub extern "C" fn put_le64__part132(ptr: *mut c_uchar, v: c_ulonglong) {
    let low = v as c_uint;
    let high = (v >> 32) as c_uint;

    unsafe {
        // Write lower 32 bits (little-endian)
        ptr.write(low as c_uchar);
        ptr.add(1).write(((low >> 8) & 0xFF) as c_uchar);
        ptr.add(2).write(((low >> 16) & 0xFF) as c_uchar);
        ptr.add(3).write(((low >> 24) & 0xFF) as c_uchar);

        // Write upper 32 bits (little-endian)
        ptr.add(4).write(high as c_uchar);
        ptr.add(5).write(((high >> 8) & 0xFF) as c_uchar);
        ptr.add(6).write(((high >> 16) & 0xFF) as c_uchar);
        ptr.add(7).write(((high >> 24) & 0xFF) as c_uchar);
    }
}
}

// ── `return (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3];` · cutils.h:144-144 · chunk 134 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk134 {
#[no_mangle]
pub extern "C" fn get_be32__part134(d: *const core::ffi::c_uchar) -> core::ffi::c_uint {
    unsafe {
        let a = *d;
        let b = *d.offset(1);
        let c = *d.offset(2);
        let d_val = *d.offset(3);
        ((a as core::ffi::c_uint) << 24) | ((b as core::ffi::c_uint) << 16) | ((c as core::ffi::c_uint) << 8) | (d_val as core::ffi::c_uint)
    }
}
}

// ── `d[0] = v >> 24; d[1] = v >> 16; d[2] = v >> 8; d[3] = v >> 0;` · cutils.h:149-152 · chunk 136 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     d[0] = v >> 24;
//         d[1] = v >> 16;
//         d[2] = v >> 8;
//         d[3] = v >> 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk136 {
use core::ptr::write;
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn put_be32__part136(d: *mut u8, v: c_uint) {
    unsafe {
        write(d, (v >> 24) as u8);
        write(d.offset(1), (v >> 16) as u8);
        write(d.offset(2), (v >> 8) as u8);
        write(d.offset(3), (v >> 0) as u8);
    }
}
}

// ── `put_be32(d, v >> 32); put_be32(d + 4, v);` · cutils.h:157-158 · chunk 138 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     put_be32(d, v >> 32);
//         put_be32(d + 4, v);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk138 {
#[no_mangle]
pub extern "C" fn put_be64__part138(mut d: *mut u8, v: u64) {
    unsafe {
        let upper = (v >> 32) as u32;
        let lower = v as u32;
        
        let b0 = ((upper >> 24) & 0xFF) as u8;
        let b1 = ((upper >> 16) & 0xFF) as u8;
        let b2 = ((upper >> 8) & 0xFF) as u8;
        let b3 = (upper & 0xFF) as u8;
        
        let b4 = ((lower >> 24) & 0xFF) as u8;
        let b5 = ((lower >> 16) & 0xFF) as u8;
        let b6 = ((lower >> 8) & 0xFF) as u8;
        let b7 = (lower & 0xFF) as u8;
        
        *d = b0;
        d = d.wrapping_add(1);
        *d = b1;
        d = d.wrapping_add(1);
        *d = b2;
        d = d.wrapping_add(1);
        *d = b3;
        
        d = d.wrapping_add(4);
        *d = b4;
        d = d.wrapping_add(1);
        *d = b5;
        d = d.wrapping_add(1);
        *d = b6;
        d = d.wrapping_add(1);
        *d = b7;
    }
}
}

// ── `return __bswap_32 ( v ) ;` · cutils.h:169-169 · chunk 140 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               __bswap_32 (
//               v
//               )
//                          ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk140 {
#[no_mangle]
pub extern "C" fn cpu_to_be32__part140(v: u32) -> u32 {
    let b0 = v as u8;
    let b1 = (v >> 8) as u8;
    let b2 = (v >> 16) as u8;
    let b3 = (v >> 24) as u8;
    ((b0 as u32) << 24) | ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32)
}
}

// ── `return val ? __builtin_ctz(val) : 32;` · cutils.h:176-176 · chunk 142 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return val ? __builtin_ctz(val) : 32;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk142 {
use core::ffi::c_uint;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ctz32__part142(val: c_uint) -> c_int {
    (val as u32).trailing_zeros() as c_int
}
}

