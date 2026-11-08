//! From `fs_utils.h`.

// ── `isspace_nolf` · fs_utils.h:51-54 · chunk 350 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL isspace_nolf(int c)
//     {
//         return (c == ' ' || c == '\t');
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk350 {
// Port of fs_utils.h: `static inline BOOL isspace_nolf(int c)`
// BOOL is `typedef int BOOL;` (cutils.h), so the return type is c_int.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isspace_nolf(c: c_int) -> c_int {
    if c == ' ' as c_int || c == '\t' as c_int {
        1
    } else {
        0
    }
}
}

// ── `return (c == ' ' || c == '\t');` · fs_utils.h:53-53 · chunk 351 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (c == ' ' || c == '\t');
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk351 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isspace_nolf__part351(c: c_int) -> c_int {
    ((c == (' ' as c_int)) || (c == ('\t' as c_int))) as c_int
}
}

// ── `from_hex` · fs_utils.h:56-66 · chunk 352 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int from_hex(int c)
//     {
//         if (c >= '0' && c <= '9')
//             return c - '0';
//         else if (c >= 'A' && c <= 'F')
//             return c - 'A' + 10;
//         else if (c >= 'a' && c <= 'f')
//             return c - 'a' + 10;
//         else
//             return -1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk352 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn from_hex(c: c_int) -> c_int {
    if c >= b'0' as c_int && c <= b'9' as c_int {
        c - b'0' as c_int
    } else if c >= b'A' as c_int && c <= b'F' as c_int {
        c - b'A' as c_int + 10
    } else if c >= b'a' as c_int && c <= b'f' as c_int {
        c - b'a' as c_int + 10
    } else {
        -1
    }
}
}

// ── `if (c >= '0' && c <= '9') … else` · fs_utils.h:58-65 · chunk 353 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (c >= '0' && c <= '9')
//             return c - '0';
//         else if (c >= 'A' && c <= 'F')
//             return c - 'A' + 10;
//         else if (c >= 'a' && c <= 'f')
//             return c - 'a' + 10;
//         else
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk353 {
// Ported from C: static inline int from_hex(int c)
// The target body (if/c-else chain) has been moved to this Rust function
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn from_hex__part353(c: c_int) -> c_int {
    if c >= '0' as c_int && c <= '9' as c_int {
        return c - '0' as c_int;
    } else if c >= 'A' as c_int && c <= 'F' as c_int {
        return c - 'A' as c_int + 10;
    } else if c >= 'a' as c_int && c <= 'f' as c_int {
        return c - 'a' as c_int + 10;
    } else {
        return -1;
    }
}
}

// ── `block_align` · fs_utils.h:68-71 · chunk 354 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint64_t block_align(uint64_t val, uint64_t align)
//     {
//         return (val + align - 1) & ~(align - 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk354 {
// Port of the static inline helper `block_align` from fs_utils.h:
//   return (val + align - 1) & ~(align - 1);
// Unsigned arithmetic in C wraps, so mirror that with wrapping ops.

#[no_mangle]
pub extern "C" fn block_align(val: u64, align: u64) -> u64 {
    val.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}
}

// ── `return (val + align - 1) & ~(align - 1);` · fs_utils.h:70-70 · chunk 355 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (val + align - 1) & ~(align - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk355 {
use core::ffi::c_ulonglong;

#[no_mangle]
pub extern "C" fn block_align__part355(val: c_ulonglong, align: c_ulonglong) -> c_ulonglong {
    ((val + align - 1) & !(align - 1))
}
}

