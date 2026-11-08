//! From `softfp.c`.

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

