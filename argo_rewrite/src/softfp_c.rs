//! From `softfp.c`.

// ── `clz32` · softfp.c:32-41 · chunk 2915 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int clz32(uint32_t a)
//     {
//         int r;
//         if (a == 0) {
//             r = 32;
//         } else {
//             r = __builtin_clz(a);
//         }
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2915 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}
}

// ── `if (a == 0) … else` · softfp.c:35-39 · chunk 2916 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a == 0) {
//             r = 32;
//         } else {
//             r = __builtin_clz(a);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2916 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn clz32__part2916(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}
}

// ── `return r;` · softfp.c:40-40 · chunk 2917 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2917 {
// Returns the given value; the caller's `clz32` hands over its computed result `r`.
#[no_mangle]
pub extern "C" fn clz32__part2917(r: core::ffi::c_int) -> core::ffi::c_int {
    r
}
}

// ── `clz64` · softfp.c:43-53 · chunk 2918 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int clz64(uint64_t a)
//     {
//         int r;
//         if (a == 0) {
//             r = 64;
//         } else
//         {
//             r = __builtin_clzll(a);
//         }
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2918 {
// Port of `static inline int clz64(uint64_t a)` from softfp.c.
// Counts leading zero bits of a 64-bit value; returns 64 for an input of 0.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn clz64(a: u64) -> c_int {
    let r: c_int;
    if a == 0 {
        r = 64;
    } else {
        // Equivalent of __builtin_clzll(a); a is known nonzero here.
        r = a.leading_zeros() as c_int;
    }
    r
}
}

// ── `if (a == 0) … else` · softfp.c:46-51 · chunk 2919 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a == 0) {
//             r = 64;
//         } else
//         {
//             r = __builtin_clzll(a);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2919 {
use core::ffi::c_int;

// Part of clz64: compute the number of leading zero bits.
#[no_mangle]
pub extern "C" fn clz64__part2919(a: u64) -> c_int {
    if a == 0 {
        64 as c_int
    } else {
        a.leading_zeros() as c_int
    }
}
}

// ── `return r;` · softfp.c:52-52 · chunk 2920 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2920 {
// Port of the `return r;` tail of clz64: simply yields the computed result.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn clz64__part2920(r: c_int) -> c_int {
    r
}
}

// ── `clz128` · softfp.c:56-72 · chunk 2921 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int clz128(uint128_t a)
//     {
//         int r;
//         if (a == 0) {
//             r = 128;
//         } else
//         {
//             uint64_t ah, al;
//             ah = a >> 64;
//             al = a;
//             if (ah != 0)
//                 r = __builtin_clzll(ah);
//             else
//                 r = __builtin_clzll(al) + 64;
//         }
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2921 {
// Port of static inline int clz128(uint128_t a) from softfp.c.
// uint128_t is `unsigned __int128`, which maps to Rust's u128 with the same ABI.

#[no_mangle]
pub extern "C" fn clz128(a: u128) -> core::ffi::c_int {
    // C: if (a == 0) r = 128; else count leading zeros of the 128-bit value
    // (high half first, then low half + 64), which is exactly leading_zeros().
    if a == 0 {
        128 as core::ffi::c_int
    } else {
        a.leading_zeros() as core::ffi::c_int
    }
}
}

// ── `if (a == 0) … else` · softfp.c:59-70 · chunk 2922 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a == 0) {
//             r = 128;
//         } else
//         {
//             uint64_t ah, al;
//             ah = a >> 64;
//             al = a;
//             if (ah != 0)
//                 r = __builtin_clzll(ah);
//             else
//                 r = __builtin_clzll(al) + 64;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2922 {
// Port of the clz128 body: count leading zeros of a 128-bit value.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn clz128__part2922(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        // uint64_t ah = a >> 64; uint64_t al = a;
        let ah: u64 = (a >> 64) as u64;
        let al: u64 = a as u64;
        if ah != 0 {
            // __builtin_clzll(ah)
            ah.leading_zeros() as c_int
        } else {
            // __builtin_clzll(al) + 64
            al.leading_zeros() as c_int + 64
        }
    }
}
}

// ── `return r;` · softfp.c:71-71 · chunk 2923 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2923 {
// Ported piece of `clz128` (softfp.c line 71): `return r;`
#[no_mangle]
pub extern "C" fn clz128__part2923(r: core::ffi::c_int) -> core::ffi::c_int {
    r
}
}

