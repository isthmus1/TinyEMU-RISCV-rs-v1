//! From `cutils.h`.

// ── `max_int` · cutils.h:69-75 · chunk 117 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int max_int(int a, int b)
//     {
//         if (a > b)
//             return a;
//         else
//             return b;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk117 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}
}

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

// ── `min_int` · cutils.h:77-83 · chunk 119 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int min_int(int a, int b)
//     {
//         if (a < b)
//             return a;
//         else
//             return b;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk119 {
// Rewrite of the C function:
//   static inline int min_int(int a, int b)
//   {
//       if (a < b)
//           return a;
//       else
//           return b;
//   }
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
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

// ── `get_le16` · cutils.h:107-110 · chunk 121 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint16_t get_le16(const uint8_t *ptr)
//     {
//         return ptr[0] | (ptr[1] << 8);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk121 {
// Port of the C inline helper:
//   static inline uint16_t get_le16(const uint8_t *ptr)
//   {
//       return ptr[0] | (ptr[1] << 8);
//   }

#[no_mangle]
pub unsafe extern "C" fn get_le16(ptr: *const u8) -> u16 {
    let lo = *ptr as u16;
    let hi = *ptr.add(1) as u16;
    lo | (hi << 8)
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

// ── `get_le32` · cutils.h:112-115 · chunk 123 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t get_le32(const uint8_t *ptr)
//     {
//         return ptr[0] | (ptr[1] << 8) | (ptr[2] << 16) | (ptr[3] << 24);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk123 {
// Little-endian 32-bit load, matching cutils.h get_le32.
#[no_mangle]
pub extern "C" fn get_le32(ptr: *const u8) -> u32 {
    unsafe {
        let p = ptr;
        (*p as u32)
            | ((*p.add(1) as u32) << 8)
            | ((*p.add(2) as u32) << 16)
            | ((*p.add(3) as u32) << 24)
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

// ── `get_le64` · cutils.h:117-120 · chunk 125 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint64_t get_le64(const uint8_t *ptr)
//     {
//         return get_le32(ptr) | ((uint64_t)get_le32(ptr + 4) << 32);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk125 {
// uint64_t get_le64(const uint8_t *ptr)
// Little-endian 64-bit load built from two little-endian 32-bit loads.
// get_le32 is a static inline in the C header, so its logic is inlined here.

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

#[no_mangle]
pub extern "C" fn get_le64(ptr: *const u8) -> u64 {
    unsafe {
        let lo = get_le32(ptr) as u64;
        let hi = get_le32(ptr.add(4)) as u64;
        lo | (hi << 32)
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

// ── `put_le16` · cutils.h:122-126 · chunk 127 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void put_le16(uint8_t *ptr, uint16_t v)
//     {
//         ptr[0] = v;
//         ptr[1] = v >> 8;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk127 {
use core::ffi::{c_uchar, c_ushort};

#[no_mangle]
pub unsafe extern "C" fn put_le16(ptr: *mut c_uchar, v: c_ushort) {
    if ptr.is_null() {
        return;
    }
    *ptr = v as c_uchar;
    *ptr.add(1) = (v >> 8) as c_uchar;
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

// ── `put_le32` · cutils.h:128-134 · chunk 129 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void put_le32(uint8_t *ptr, uint32_t v)
//     {
//         ptr[0] = v;
//         ptr[1] = v >> 8;
//         ptr[2] = v >> 16;
//         ptr[3] = v >> 24;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk129 {
// Port of: static inline void put_le32(uint8_t *ptr, uint32_t v)
#![allow(clippy::missing_safety_doc)]

use core::ffi::c_uchar;

#[no_mangle]
pub unsafe extern "C" fn put_le32(ptr: *mut c_uchar, v: u32) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        *ptr = v as c_uchar;
        *ptr.add(1) = (v >> 8) as c_uchar;
        *ptr.add(2) = (v >> 16) as c_uchar;
        *ptr.add(3) = (v >> 24) as c_uchar;
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

// ── `put_le64` · cutils.h:136-140 · chunk 131 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void put_le64(uint8_t *ptr, uint64_t v)
//     {
//         put_le32(ptr, v);
//         put_le32(ptr + 4, v >> 32);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk131 {
#[no_mangle]
pub extern "C" fn put_le64(ptr: *mut u8, v: u64) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // low 32 bits at ptr[0..4], high 32 bits at ptr[4..8], little-endian
        let mut i: usize = 0;
        while i < 8 {
            *ptr.add(i) = (v >> (8 * i as u32)) as u8;
            i += 1;
        }
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

// ── `get_be32` · cutils.h:142-145 · chunk 133 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t get_be32(const uint8_t *d)
//     {
//         return (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3];
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk133 {
// Big-endian 32-bit read from a byte buffer.
// Mirrors C: (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3]

use core::ffi::c_uchar;

#[no_mangle]
pub unsafe extern "C" fn get_be32(d: *const c_uchar) -> u32 {
    let b0 = *d as u32;
    let b1 = *d.add(1) as u32;
    let b2 = *d.add(2) as u32;
    let b3 = *d.add(3) as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
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

// ── `put_be32` · cutils.h:147-153 · chunk 135 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void put_be32(uint8_t *d, uint32_t v)
//     {
//         d[0] = v >> 24;
//         d[1] = v >> 16;
//         d[2] = v >> 8;
//         d[3] = v >> 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk135 {
// Big-endian store of a 32-bit value, matching cutils.h put_be32.
#[no_mangle]
pub unsafe extern "C" fn put_be32(d: *mut u8, v: u32) {
    if d.is_null() {
        return;
    }
    *d = (v >> 24) as u8;
    *d.add(1) = (v >> 16) as u8;
    *d.add(2) = (v >> 8) as u8;
    *d.add(3) = v as u8;
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

// ── `put_be64` · cutils.h:155-159 · chunk 137 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void put_be64(uint8_t *d, uint64_t v)
//     {
//         put_be32(d, v >> 32);
//         put_be32(d + 4, v);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk137 {
// Port of cutils.h:155-159
//
// static inline void put_be64(uint8_t *d, uint64_t v)
// {
//     put_be32(d, v >> 32);
//     put_be32(d + 4, v);
// }
//
// put_be32(b, x) stores x big-endian:
//   b[0] = x >> 24; b[1] = x >> 16; b[2] = x >> 8; b[3] = x >> 0;
// Expanded so the whole 64-bit value is written in one place.

#[no_mangle]
pub extern "C" fn put_be64(d: *mut u8, v: u64) {
    unsafe {
        // first word: v >> 32
        *d.add(0) = (v >> 56) as u8;
        *d.add(1) = (v >> 48) as u8;
        *d.add(2) = (v >> 40) as u8;
        *d.add(3) = (v >> 32) as u8;
        // second word: v (only the low 32 bits survive the u8 truncation)
        *d.add(4) = (v >> 24) as u8;
        *d.add(5) = (v >> 16) as u8;
        *d.add(6) = (v >> 8) as u8;
        *d.add(7) = v as u8;
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

// ── `cpu_to_be32` · cutils.h:167-170 · chunk 139 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t cpu_to_be32(uint32_t v)
//     {
//         return 
//               __bswap_32 (
//               v
//               )
//                          ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk139 {
// Rust rewrite of cpu_to_be32 (byte swap for little-endian hosts).
#[no_mangle]
pub extern "C" fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
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

// ── `ctz32` · cutils.h:173-206 · chunk 141 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int ctz32(uint32_t val)
//     {
//     
//         return val ? __builtin_ctz(val) : 32;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk141 {
// Port of the `ctz32` helper from cutils.h.
//
// C original:
//     static inline int ctz32(uint32_t val)
//     {
//         return val ? __builtin_ctz(val) : 32;
//     }
//
// `__builtin_ctz` is undefined for 0, so the conditional returns 32 in that case.
// `u32::trailing_zeros` matches `__builtin_ctz` for all nonzero inputs.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ctz32(val: u32) -> c_int {
    if val != 0 {
        val.trailing_zeros() as c_int
    } else {
        32
    }
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

