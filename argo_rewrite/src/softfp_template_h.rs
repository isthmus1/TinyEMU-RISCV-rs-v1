//! From `softfp_template.h`.

// ── `pack_sf32` · softfp_template.h:78-83 · chunk 2926 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
//     {
//         return ((uint32_t)a_sign << (32 - 1)) |
//             ((uint32_t)a_exp << 23) |
//             (a_mant & (((uint32_t)1 << 23) - 1));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2926 {
// Port of the softfp template helper pack_sf32 (lines 78-83 of softfp_template.h).
#![allow(clippy::all)]

#[no_mangle]
pub extern "C" fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}
}

// ── `pack_sf64` · softfp_template.h:78-83 · chunk 3038 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
//     {
//         return ((uint64_t)a_sign << (64 - 1)) |
//             ((uint64_t)a_exp << 52) |
//             (a_mant & (((uint64_t)1 << 52) - 1));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3038 {
// Rewrite of the C helper:
//   static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
//   {
//       return ((uint64_t)a_sign << (64 - 1)) |
//           ((uint64_t)a_exp << 52) |
//           (a_mant & (((uint64_t)1 << 52) - 1));
//   }

#[no_mangle]
pub extern "C" fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    let sign_part: u64 = (a_sign as u64) << 63;
    let exp_part: u64 = (a_exp as u64) << 52;
    let mant_part: u64 = a_mant & ((1u64 << 52) - 1);
    sign_part | exp_part | mant_part
}
}

// ── `pack_sf128` · softfp_template.h:78-83 · chunk 3160 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
//     {
//         return ((uint128_t)a_sign << (128 - 1)) |
//             ((uint128_t)a_exp << 112) |
//             (a_mant & (((uint128_t)1 << 112) - 1));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3160 {
// Port of `pack_sf128`: uint128_t == unsigned __int128 == Rust u128.
// The original is a `static inline` helper, but the harness links this
// exported symbol in place of the C definition.

#[no_mangle]
pub extern "C" fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    let mant_mask: u128 = ((1u128 << 112) - 1) as u128;
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & mant_mask)
}
}

// ── `return ((uint32_t)a_sign << (32 - 1)) | ((uint32_t)a_exp << 23) | (a_mant & (((uint32_t)1 << 23…` · softfp_template.h:80-82 · chunk 2927 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return ((uint32_t)a_sign << (32 - 1)) |
//             ((uint32_t)a_exp << 23) |
//             (a_mant & (((uint32_t)1 << 23) - 1));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2927 {
// Port of the returned expression of pack_sf32 (softfp_template.h lines 80-82).
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn pack_sf32__part2927(a_sign: c_uint, a_exp: c_uint, a_mant: c_uint) -> c_uint {
    let sign = (a_sign as u32) << (32 - 1);
    let exp = (a_exp as u32) << 23;
    let mant = (a_mant as u32) & (((1u32) << 23) - 1);
    sign | exp | mant
}
}

// ── `return ((uint64_t)a_sign << (64 - 1)) | ((uint64_t)a_exp << 52) | (a_mant & (((uint64_t)1 << 52…` · softfp_template.h:80-82 · chunk 3039 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return ((uint64_t)a_sign << (64 - 1)) |
//             ((uint64_t)a_exp << 52) |
//             (a_mant & (((uint64_t)1 << 52) - 1));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3039 {
// Port of the pack_sf64 bit-packing expression.
// uint32_t -> u32, uint64_t -> unsigned long (64-bit on this target) -> u64.

#[no_mangle]
pub extern "C" fn pack_sf64__part3039(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}
}

// ── `return ((uint128_t)a_sign << (128 - 1)) | ((uint128_t)a_exp << 112) | (a_mant & (((uint128_t)1…` · softfp_template.h:80-82 · chunk 3161 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ((uint128_t)a_sign << (128 - 1)) |
//             ((uint128_t)a_exp << 112) |
//             (a_mant & (((uint128_t)1 << 112) - 1));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3161 {
// Port of the packing expression from pack_sf128.
// Builds a 128-bit soft-float value from sign, exponent and mantissa.

#[no_mangle]
pub extern "C" fn pack_sf128__part3161(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    let sign_part: u128 = (a_sign as u128) << 127;
    let exp_part: u128 = (a_exp as u128) << 112;
    let mant_mask: u128 = (1u128 << 112) - 1;
    sign_part | exp_part | (a_mant & mant_mask)
}
}

// ── `unpack_sf32` · softfp_template.h:85-91 · chunk 2928 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t unpack_sf32(uint32_t *pa_sign, int32_t *pa_exp,
//                                    uint32_t a)
//     {
//         *pa_sign = a >> (32 - 1);
//         *pa_exp = (a >> 23) & ((1 << 8) - 1);
//         return a & (((uint32_t)1 << 23) - 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2928 {
// Port of softfp_template.h unpack_sf32

#[no_mangle]
pub unsafe extern "C" fn unpack_sf32(
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    a: u32,
) -> u32 {
    *pa_sign = a >> 31;
    *pa_exp = ((a >> 23) & 0xffu32) as i32;
    a & 0x007f_ffffu32
}
}

// ── `unpack_sf64` · softfp_template.h:85-91 · chunk 3040 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint64_t unpack_sf64(uint32_t *pa_sign, int32_t *pa_exp,
//                                    uint64_t a)
//     {
//         *pa_sign = a >> (64 - 1);
//         *pa_exp = (a >> 52) & ((1 << 11) - 1);
//         return a & (((uint64_t)1 << 52) - 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3040 {
#[no_mangle]
pub extern "C" fn unpack_sf64(pa_sign: *mut u32, pa_exp: *mut i32, a: u64) -> u64 {
    unsafe {
        *pa_sign = (a >> (64 - 1)) as u32;
        *pa_exp = ((a >> 52) & (((1u64 << 11) - 1) as u64)) as i32;
    }
    a & (((1u64 << 52) - 1) as u64)
}
}

// ── `unpack_sf128` · softfp_template.h:85-91 · chunk 3162 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint128_t unpack_sf128(uint32_t *pa_sign, int32_t *pa_exp,
//                                    uint128_t a)
//     {
//         *pa_sign = a >> (128 - 1);
//         *pa_exp = (a >> 112) & ((1 << 15) - 1);
//         return a & (((uint128_t)1 << 112) - 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3162 {
// Port of:
// static inline uint128_t unpack_sf128(uint32_t *pa_sign, int32_t *pa_exp,
//                                      uint128_t a)
// uint128_t is `unsigned __int128`.

#[no_mangle]
pub unsafe extern "C" fn unpack_sf128(pa_sign: *mut u32, pa_exp: *mut i32, a: u128) -> u128 {
    *pa_sign = (a >> (128 - 1)) as u32;
    *pa_exp = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    a & ((1u128 << 112) - 1)
}
}

// ── `*pa_sign = a >> (32 - 1); *pa_exp = (a >> 23) & ((1 << 8) - 1); return a & (((uint32_t)1 << 23)…` · softfp_template.h:88-90 · chunk 2929 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pa_sign = a >> (32 - 1);
//         *pa_exp = (a >> 23) & ((1 << 8) - 1);
//         return a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2929 {
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn unpack_sf32__part2929(
    pa_sign: *mut c_uint,
    pa_exp: *mut c_int,
    a: c_uint,
) -> c_uint {
    unsafe {
        *pa_sign = a >> (32 - 1);
        *pa_exp = ((a >> 23) & ((1u32 << 8) - 1)) as c_int;
    }
    a & (1u32 << 23).wrapping_sub(1)
}
}

// ── `*pa_sign = a >> (64 - 1); *pa_exp = (a >> 52) & ((1 << 11) - 1); return a & (((uint64_t)1 << 52…` · softfp_template.h:88-90 · chunk 3041 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pa_sign = a >> (64 - 1);
//         *pa_exp = (a >> 52) & ((1 << 11) - 1);
//         return a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3041 {
// Ported piece of softfp unpack_sf64: split a binary64 bit pattern into
// sign, biased exponent and mantissa.
// uint32_t = unsigned int, int32_t = signed int, uint64_t = unsigned long.

#[no_mangle]
pub extern "C" fn unpack_sf64__part3041(pa_sign: *mut u32, pa_exp: *mut i32, a: u64) -> u64 {
    unsafe {
        // *pa_sign = a >> (64 - 1);
        *pa_sign = (a >> (64 - 1)) as u32;
        // *pa_exp = (a >> 52) & ((1 << 11) - 1);
        *pa_exp = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    }
    // return a & (((uint64_t)1 << 52) - 1);
    a & ((1u64 << 52) - 1)
}
}

// ── `*pa_sign = a >> (128 - 1); *pa_exp = (a >> 112) & ((1 << 15) - 1); return a & (((uint128_t)1 <<…` · softfp_template.h:88-90 · chunk 3163 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pa_sign = a >> (128 - 1);
//         *pa_exp = (a >> 112) & ((1 << 15) - 1);
//         return a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3163 {
// Port of the unpack half of unpack_sf128 from softfp_template.h.
// uint128_t is unsigned __int128 -> u128; uint32_t -> u32; int32_t -> i32.

#[no_mangle]
pub extern "C" fn unpack_sf128__part3163(
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    a: u128,
) -> u128 {
    unsafe {
        // *pa_sign = a >> (128 - 1);  (assigned to uint32_t: low 32 bits)
        *pa_sign = (a >> 127) as u32;
        // *pa_exp = (a >> 112) & ((1 << 15) - 1);  (assigned to int32_t)
        *pa_exp = ((a >> 112) as u32 & ((1u32 << 15) - 1)) as i32;
        // return a & (((uint128_t)1 << 112) - 1);
        a & ((1u128 << 112) - 1)
    }
}
}

// ── `rshift_rnd32` · softfp_template.h:93-105 · chunk 2930 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t rshift_rnd32(uint32_t a, int d)
//     {
//         uint32_t mask;
//         if (d != 0) {
//             if (d >= 32) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint32_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
//         return a;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2930 {
// Rewrite of static uint32_t rshift_rnd32(uint32_t a, int d)
#[no_mangle]
pub extern "C" fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            // C shift counts are used as-is; use wrapping shifts so that
            // out-of-range (undefined in C) counts never panic in Rust.
            let mask: u32 = (1u32.wrapping_shl(d as u32)).wrapping_sub(1);
            let hi = a.wrapping_shr(d as u32);
            let low_nonzero = (a & mask) != 0;
            a = hi | (low_nonzero as u32);
        }
    }
    a
}
}

// ── `rshift_rnd64` · softfp_template.h:93-105 · chunk 3042 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t rshift_rnd64(uint64_t a, int d)
//     {
//         uint64_t mask;
//         if (d != 0) {
//             if (d >= 64) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint64_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
//         return a;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3042 {
use core::ffi::c_int;

// Round-to-nearest right shift for 64-bit mantissas: shifts `a` right by `d`
// bits and ORs in a sticky bit when any shifted-out bit was nonzero.
#[no_mangle]
pub extern "C" fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            // C's shift by a (possibly negative) int is UB in that case; use a
            // masked shift so a negative count cannot panic across the FFI.
            let sh = (d as u32) & 63;
            let mask: u64 = (1u64 << sh).wrapping_sub(1);
            a = (a >> sh) | ((a & mask) != 0) as u64;
        }
    }
    a
}
}

// ── `rshift_rnd128` · softfp_template.h:93-105 · chunk 3164 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t rshift_rnd128(uint128_t a, int d)
//     {
//         uint128_t mask;
//         if (d != 0) {
//             if (d >= 128) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint128_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
//         return a;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3164 {
// Port of softfp_template.h rshift_rnd128 (static uint128_t rshift_rnd128(uint128_t, int)).
// uint128_t is unsigned __int128 in the C program, which maps to Rust's u128.

#![allow(non_snake_case)]

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            // a = (a != 0);
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            // mask = ((uint128_t)1 << d) - 1;
            let mask: u128 = (1u128 << (d as u32)) - 1;
            // a = (a >> d) | ((a & mask) != 0);
            let sticky: u128 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> (d as u32)) | sticky;
        }
        // d < 0 would be undefined behavior in C (shift by negative count);
        // leave a unchanged in that case instead of panicking.
    }
    a
}
}

// ── `if (d != 0)` · softfp_template.h:96-103 · chunk 2931 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (d != 0) {
//             if (d >= 32) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint32_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2931 {
// Port of the rounding right-shift body of rshift_rnd32.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn rshift_rnd32__part2931(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            // Shifts mirror the machine behavior (count masked to 5 bits) so
            // that a negative d cannot trap.
            let mask: u32 = (1u32.wrapping_shl(d as u32)).wrapping_sub(1);
            a = a.wrapping_shr(d as u32) | ((a & mask) != 0) as u32;
        }
    }
    a
}
}

// ── `if (d != 0)` · softfp_template.h:96-103 · chunk 3043 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (d != 0) {
//             if (d >= 64) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint64_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3043 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn rshift_rnd64__part3043(a: *mut u64, d: c_int, mask: *mut u64) {
    unsafe {
        if d != 0 {
            if d >= 64 {
                *a = (*a != 0) as u64;
            } else {
                let m = (1u64).wrapping_shl(d as u32).wrapping_sub(1);
                *mask = m;
                let cur = *a;
                *a = cur.wrapping_shr(d as u32) | ((cur & m) != 0) as u64;
            }
        }
    }
}
}

// ── `if (d != 0)` · softfp_template.h:96-103 · chunk 3165 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (d != 0) {
//             if (d >= 128) {
//                 a = (a != 0);
//             } else {
//                 mask = ((uint128_t)1 << d) - 1;
//                 a = (a >> d) | ((a & mask) != 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3165 {
// Rounding shift-right of an unsigned 128-bit value, extracted from
// rshift_rnd128 in softfp_template.h.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn rshift_rnd128__part3165(a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else {
            let s = d as u32;
            let mask = 1u128.wrapping_shl(s) - 1;
            a.wrapping_shr(s) | (((a & mask) != 0) as u128)
        }
    } else {
        a
    }
}
}

// ── `return a;` · softfp_template.h:104-104 · chunk 2932 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return a;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2932 {
// Piece of rshift_rnd32: return the (already shifted/rounded) mantissa value.
#[no_mangle]
pub extern "C" fn rshift_rnd32__part2932(a: u32) -> u32 {
    a
}
}

// ── `return a;` · softfp_template.h:104-104 · chunk 3044 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return a;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3044 {
// Returns the (already computed) 64-bit value unchanged.
#[no_mangle]
pub extern "C" fn rshift_rnd64__part3044(a: u64) -> u64 {
    a
}
}

// ── `return a;` · softfp_template.h:104-104 · chunk 3166 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return a;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3166 {
// Port of the tail of rshift_rnd128: returns the shifted/rounded value.

#[no_mangle]
pub extern "C" fn rshift_rnd128__part3166(a: u128) -> u128 {
    a
}
}

// ── `roundpack_sf32` · softfp_template.h:108-175 · chunk 2933 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int diff;
//         uint32_t addend, rnd_bits;
//     
//         switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((32 - 2) - 23) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((32 - 2) - 23)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
//     
//     
//         if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint32_t)1 << (32 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd32(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//         }
//         if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
//         a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//         if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//             a_mant &= ~1;
//     
//     
//         a_exp += a_mant >> (23 + 1);
//         if (a_mant <= (((uint32_t)1 << 23) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 8) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 8) - 1) - 1;
//                 a_mant = (((uint32_t)1 << 23) - 1);
//             } else {
//     
//                 a_exp = ((1 << 8) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
//         return pack_sf32(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2933 {
// Rust port of the softfloat single-precision round-and-pack helper
// (softfp_template.h round_pack_sf, instantiated for 32-bit floats).
//
// F_SIZE = 32, MANT_SIZE = 23, EXP_MASK = 255, RND_SIZE = 7.
//
// The C helpers pack_sf32() (static inline) and rshift_rnd32() (static) have
// no linker symbols, so their logic is reproduced here as private fns.

#![allow(non_snake_case)]

use core::ffi::{c_int, c_uint};

const F_SIZE: u32 = 32;
const MANT_SIZE: u32 = 23;
const EXP_MASK: c_int = ((1u32 << 8) - 1) as c_int; // 255
const RND_SIZE: u32 = (F_SIZE - 2) - MANT_SIZE; // 7
const RND_MASK: u32 = (1u32 << RND_SIZE) - 1; // 0x7f
const RND_HALF: u32 = 1u32 << (RND_SIZE - 1); // 0x40
const MANT_MASK: u32 = (1u32 << MANT_SIZE) - 1; // 0x7fffff

// RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM
const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RMM: c_uint = 4;

// Equivalent of the C static inline pack_sf32(a_sign, a_exp, a_mant).
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (F_SIZE - 1)) | (a_exp << MANT_SIZE) | (a_mant & MANT_MASK)
}

// Equivalent of the C static rshift_rnd32(a, d): right shift with round-to-odd
// (sticky bit set if any shifted-out bit was nonzero).
#[inline]
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
        // d < 0 is undefined in C; leave a unchanged.
    }
    a
}

#[no_mangle]
pub extern "C" fn roundpack_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp: c_int = a_exp;
    let mut a_mant: u32 = a_mant;
    let addend: u32;
    let mut rnd_bits: u32;

    // switch (rm): RM_RNE / RM_RMM -> half; RM_RTZ -> 0; default (RM_RDN,
    // RM_RUP and anything else) -> all ones when rounding away from zero.
    if rm == RM_RNE || rm == RM_RMM {
        addend = RND_HALF;
    } else if rm == RM_RTZ {
        addend = 0;
    } else if (a_sign ^ (rm & 1)) != 0 {
        addend = (1u32 << RND_SIZE) - 1;
    } else {
        addend = 0;
    }

    // potentially subnormal
    if a_exp <= 0 {
        // Note: we set the underflow flag if the rounded result
        // is subnormal and inexact
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (F_SIZE - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & RND_MASK;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1; // FFLAG_UNDERFLOW
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & RND_MASK;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1; // FFLAG_INEXACT
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> RND_SIZE;
    // half way: select even result
    if rm == RM_RNE && rnd_bits == RND_HALF {
        a_mant &= !1u32;
    }
    // Note the rounding adds at least 1, so this is the maximum value
    a_exp = a_exp.wrapping_add((a_mant >> (MANT_SIZE + 1)) as c_int);
    if a_mant <= MANT_MASK {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        // overflow
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK;
        } else {
            // infinity
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | 1; // FFLAG_OVERFLOW | FFLAG_INEXACT
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}
}

// ── `roundpack_sf64` · softfp_template.h:108-175 · chunk 3045 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int diff;
//         uint32_t addend, rnd_bits;
//     
//         switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((64 - 2) - 52) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((64 - 2) - 52)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
//     
//     
//         if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint64_t)1 << (64 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd64(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//         }
//         if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
//         a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//         if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//             a_mant &= ~1;
//     
//     
//         a_exp += a_mant >> (52 + 1);
//         if (a_mant <= (((uint64_t)1 << 52) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 11) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 11) - 1) - 1;
//                 a_mant = (((uint64_t)1 << 52) - 1);
//             } else {
//     
//                 a_exp = ((1 << 11) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
//         return pack_sf64(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3045 {
// Port of static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                        RoundingModeEnum rm, uint32_t *pfflags)
// from softfp_template.h (F_SIZE = 64, MANT_SIZE = 52, RND_SIZE = 10, EXP_MASK = 2047).
//
// RoundingModeEnum: RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4.
// The C enum parameter is passed as a plain int.

use core::ffi::{c_int, c_uint};

// static uint64_t rshift_rnd64(uint64_t a, int d)  -- no linker symbol in C
#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else if d > 0 {
            // d in 1..=63 here; (uint64_t)1 << d is well defined
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    let s = ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1));
    s
}

#[no_mangle]
pub extern "C" fn roundpack_sf64(
    a_sign: c_uint,
    a_exp: c_int,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut c_uint,
) -> u64 {
    // uint32_t addend, rnd_bits;
    let addend: u32;
    let rnd_bits: u32;

    // hoisted locals (C: int a_exp parameter is mutated; a_mant too)
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    // switch (rm)
    if rm == 0 || rm == 4 {
        // RM_RNE / RM_RMM
        addend = 1u32 << ((10 - 1) as u32); // (1 << 9) == 512
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else {
        // default: RM_RDN, RM_RUP (and anything else)
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << 10u32) - 1; // (1 << 10) - 1 == 1023
        } else {
            addend = 0;
        }
    }

    // potentially subnormal
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    // half way: select even result
    if rm == 0 && rnd_bits == (1u32 << ((10 - 1) as u32)) {
        a_mant &= !1u64;
    }

    // Note the rounding adds at least 1, so this is the maximum value
    a_exp = (a_exp as u64).wrapping_add(a_mant >> (52 + 1)) as i32;
    if a_mant <= ((1u64 << 52) - 1) {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= ((1i32 << 11) - 1) {
        // overflow
        if addend == 0 {
            a_exp = ((1i32 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            // infinity
            a_exp = (1i32 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}
}

// ── `roundpack_sf128` · softfp_template.h:108-175 · chunk 3167 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int diff;
//         uint32_t addend, rnd_bits;
//     
//         switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((128 - 2) - 112) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((128 - 2) - 112)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
//     
//     
//         if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint128_t)1 << (128 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd128(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//         }
//         if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
//         a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//         if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//             a_mant &= ~1;
//     
//     
//         a_exp += a_mant >> (112 + 1);
//         if (a_mant <= (((uint128_t)1 << 112) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 15) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 15) - 1) - 1;
//                 a_mant = (((uint128_t)1 << 112) - 1);
//             } else {
//     
//                 a_exp = ((1 << 15) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
//         return pack_sf128(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3167 {
// Rust rewrite of static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp,
//        uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags)
// Constants for the binary128 format: F_SIZE = 128, MANT_SIZE = 112,
// RND_SIZE = F_SIZE - 2 - MANT_SIZE = 14, EXP_MASK = (1 << 15) - 1.

use core::ffi::c_int;

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let sh = d as u32;
            let mask = (1u128 << sh).wrapping_sub(1);
            a = (a >> sh) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn roundpack_sf128(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp;
    let mut a_mant: u128 = a_mant;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let addend: u32;
    match rm {
        0 | 4 => {
            addend = 1u32 << 13; // 1 << (RND_SIZE - 1)
        }
        1 => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 14) - 1; // (1 << RND_SIZE) - 1
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff: c_int = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << 14) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << 14) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    // half way: select even result
    if rm == 0 && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 113) as c_int);
    if a_mant <= ((1u128 << 112) - 1) {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        // overflow
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            // infinity
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}
}

// ── `switch (rm)` · softfp_template.h:114-131 · chunk 2934 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((32 - 2) - 23) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((32 - 2) - 23)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2934 {
// Compute the rounding addend for roundpack_sf32 (32-bit float round/pack).
// RoundingModeEnum: RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2934(a_sign: u32, rm: core::ffi::c_int) -> u32 {
    match rm {
        // case RM_RNE: case RM_RMM:
        0 | 4 => 1u32 << (((32 - 2) - 23) - 1),
        // case RM_RTZ:
        1 => 0,
        // default: case RM_RDN: case RM_RUP:
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        }
    }
}
}

// ── `switch (rm)` · softfp_template.h:114-131 · chunk 3046 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((64 - 2) - 52) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((64 - 2) - 52)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3046 {
use core::ffi::c_int;

// Computes the rounding addend for roundpack_sf64 based on the rounding mode
// and the sign of the value.
#[no_mangle]
pub extern "C" fn roundpack_sf64__part3046(a_sign: u32, rm: c_int) -> u32 {
    match rm {
        // RM_RNE, RM_RMM
        0 | 4 => 1u32 << (((64 - 2) - 52) - 1),
        // RM_RTZ
        1 => 0,
        // default (RM_RDN, RM_RUP, anything else)
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((64 - 2) - 52)) - 1
            } else {
                0
            }
        }
    }
}
}

// ── `switch (rm)` · softfp_template.h:114-131 · chunk 3168 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(rm) {
//         case RM_RNE:
//         case RM_RMM:
//             addend = (1 << (((128 - 2) - 112) - 1));
//             break;
//         case RM_RTZ:
//             addend = 0;
//             break;
//         default:
//         case RM_RDN:
//         case RM_RUP:
//     
//             if (a_sign ^ (rm & 1))
//                 addend = (1 << ((128 - 2) - 112)) - 1;
//             else
//                 addend = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3168 {
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3168(a_sign: u32, rm: c_int) -> u32 {
    let addend: u32;
    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((128 - 2) - 112) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: case RM_RDN: case RM_RUP:
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1;
            } else {
                addend = 0;
            }
        }
    }
    addend
}
}

// ── `if (a_exp <= 0) … else` · softfp_template.h:134-149 · chunk 2935 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint32_t)1 << (32 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd32(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2935 {
// Port of the subnormal/rounding-bits block of roundpack_sf32 (softfp_template.h 134-149).
// rshift_rnd32 is a static C function (no linker symbol), so its logic is mirrored here.

use core::ffi::c_int;

fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask = (1u32.wrapping_shl(d as u32)).wrapping_sub(1);
            a = (a.wrapping_shr(d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2935(
    a_exp: *mut c_int,
    a_mant: *mut u32,
    addend: u32,
    diff: *mut c_int,
    rnd_bits: *mut u32,
    pfflags: *mut u32,
) {
    unsafe {
        let exp = *a_exp;
        let mant = *a_mant;
        if exp <= 0 {
            let is_subnormal: bool = exp < 0 || mant.wrapping_add(addend) < (1u32 << 31);
            *diff = 1 - exp;
            let m = rshift_rnd32(mant, 1 - exp);
            let rb = m & ((1u32 << 7) - 1);
            if is_subnormal && rb != 0 {
                *pfflags |= 1u32 << 1;
            }
            *a_mant = m;
            *rnd_bits = rb;
            *a_exp = 1;
        } else {
            *rnd_bits = mant & ((1u32 << 7) - 1);
        }
    }
}
}

// ── `if (a_exp <= 0) … else` · softfp_template.h:134-149 · chunk 3047 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint64_t)1 << (64 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd64(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3047 {
// Ported piece of roundpack_sf64 (softfp_template.h lines 134-149).
// Handles the subnormal / exponent-adjustment branch, returning rnd_bits.

use core::ffi::c_int;

// Static C helper rshift_rnd64 (no linker symbol) ported as a private fn.
fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    if d <= 0 {
        return a;
    }
    if d >= 64 {
        return (a != 0) as u64;
    }
    let mask: u64 = (1u64 << d) - 1;
    (a >> d) | ((a & mask) != 0) as u64
}

#[no_mangle]
pub extern "C" fn roundpack_sf64__part3047(
    a_mant: *mut u64,
    a_exp: *mut c_int,
    addend: u32,
    pfflags: *mut u32,
) -> u32 {
    unsafe {
        let mut mant: u64 = *a_mant;
        let exp: c_int = *a_exp;
        let rnd_bits: u32;

        if exp <= 0 {
            let is_subnormal: bool =
                exp < 0 || mant.wrapping_add(addend as u64) < (1u64 << 63);
            let diff: c_int = 1i32.wrapping_sub(exp);
            mant = rshift_rnd64(mant, diff);
            rnd_bits = (mant & (((1u32 << ((64 - 2) - 52)) - 1) as u64)) as u32;
            if is_subnormal && rnd_bits != 0 {
                *pfflags |= 1u32 << 1;
            }
            *a_exp = 1;
        } else {
            rnd_bits = (mant & (((1u32 << ((64 - 2) - 52)) - 1) as u64)) as u32;
        }

        *a_mant = mant;
        rnd_bits
    }
}
}

// ── `if (a_exp <= 0) … else` · softfp_template.h:134-149 · chunk 3169 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp <= 0) {
//             BOOL is_subnormal;
//     
//     
//             is_subnormal = (a_exp < 0 ||
//                             (a_mant + addend) < ((uint128_t)1 << (128 - 1)));
//             diff = 1 - a_exp;
//             a_mant = rshift_rnd128(a_mant, diff);
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             if (is_subnormal && rnd_bits != 0) {
//                 *pfflags |= (1 << 1);
//             }
//             a_exp = 1;
//         } else {
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3169 {
// Ported piece of roundpack_sf128 (softfp_template.h), 128-bit float rounding.
// rshift_rnd128 is a `static` C helper with no linker symbol, so its logic is
// reproduced here as a private Rust fn.

use core::ffi::c_int;

#[inline]
unsafe fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = ((1u128) << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

#[no_mangle]
pub unsafe extern "C" fn roundpack_sf128__part3169(
    mut a_exp: c_int,
    mut a_mant: u128,
    addend: u32,
    pfflags: *mut u32,
    pa_exp: *mut c_int,
    pa_mant: *mut u128,
    prnd_bits: *mut u32,
) {
    let rnd_bits: u32;

    if a_exp <= 0 {
        // BOOL is_subnormal
        let is_subnormal: c_int = (a_exp < 0
            || a_mant.wrapping_add(addend as u128) < ((1u128) << (128 - 1)))
            as c_int;
        let diff: c_int = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & (((1u128 << ((128 - 2) - 112)) - 1) as u128)) as u32;
        if is_subnormal != 0 && rnd_bits != 0 {
            *pfflags |= 1u32 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u128 << ((128 - 2) - 112)) - 1) as u128)) as u32;
    }

    *pa_exp = a_exp;
    *pa_mant = a_mant;
    *prnd_bits = rnd_bits;
}
}

// ── `if (rnd_bits != 0)` · softfp_template.h:150-151 · chunk 2936 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2936 {
// Ported piece of roundpack_sf32: inexact flag setting.
#[no_mangle]
pub extern "C" fn roundpack_sf32__part2936(rnd_bits: u32, pfflags: *mut u32) {
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
}
}

// ── `if (rnd_bits != 0)` · softfp_template.h:150-151 · chunk 3048 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3048 {
// Piece of roundpack_sf64: set the inexact flag when rounding bits are nonzero.
#[no_mangle]
pub extern "C" fn roundpack_sf64__part3048(rnd_bits: u32, pfflags: *mut u32) {
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1;
        }
    }
}
}

// ── `if (rnd_bits != 0)` · softfp_template.h:150-151 · chunk 3170 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rnd_bits != 0)
//             *pfflags |= (1 << 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3170 {
// Port of the inexact-flag update from roundpack_sf128:
//   if (rnd_bits != 0)
//       *pfflags |= (1 << 0);

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3170(rnd_bits: u32, pfflags: *mut u32) {
    if rnd_bits != 0 {
        unsafe {
            if !pfflags.is_null() {
                let v = core::ptr::read(pfflags);
                core::ptr::write(pfflags, v | 1u32);
            }
        }
    }
}
}

// ── `a_mant = (a_mant + addend) >> ((32 - 2) - 23);` · softfp_template.h:152-152 · chunk 2937 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = (a_mant + addend) >> ((32 - 2) - 23);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2937 {
// Port of the mantissa rounding-shift step of roundpack_sf32.
// C: a_mant = (a_mant + addend) >> ((32 - 2) - 23);   /* shift by 7 */
// Both operands are uint32_t, so the addition wraps modulo 2^32 and the
// shift is a logical (unsigned) shift.

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2937(a_mant: u32, addend: u32) -> u32 {
    a_mant.wrapping_add(addend) >> 7
}
}

// ── `a_mant = (a_mant + addend) >> ((64 - 2) - 52);` · softfp_template.h:152-152 · chunk 3049 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = (a_mant + addend) >> ((64 - 2) - 52);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3049 {
// Extract of roundpack_sf64: a_mant = (a_mant + addend) >> ((64 - 2) - 52);
#[no_mangle]
pub extern "C" fn roundpack_sf64__part3049(a_mant: u64, addend: u32) -> u64 {
    // addend is a uint32_t promoted to uint64_t before the addition; shift is by 10.
    (a_mant.wrapping_add(addend as u64)) >> 10
}
}

// ── `a_mant = (a_mant + addend) >> ((128 - 2) - 112);` · softfp_template.h:152-152 · chunk 3171 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = (a_mant + addend) >> ((128 - 2) - 112);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3171 {
// Port of: a_mant = (a_mant + addend) >> ((128 - 2) - 112);
// uint128_t is unsigned __int128; addend is uint32_t promoted to uint128_t.
// The addition wraps modulo 2^128.

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3171(a_mant: u128, addend: u32) -> u128 {
    let sum: u128 = a_mant.wrapping_add(addend as u128);
    sum >> 14u32
}
}

// ── `if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)` · softfp_template.h:154-155 · chunk 2938 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//             a_mant &= ~1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2938 {
// Piece of roundpack_sf32: round-to-nearest-even tie fixup on the mantissa.
// if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//     a_mant &= ~1;

const RM_RNE: core::ffi::c_int = 0;

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2938(rm: core::ffi::c_int, rnd_bits: u32, a_mant: u32) -> u32 {
    let mut m: u32 = a_mant;
    // (1 << (((32 - 2) - 23) - 1)) == 1 << 6
    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        m &= !1u32;
    }
    m
}
}

// ── `if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)` · softfp_template.h:154-155 · chunk 3050 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//             a_mant &= ~1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3050 {
// Ported piece of roundpack_sf64 (softfp_template.h lines 154-155).
// if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//     a_mant &= ~1;

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RND_HALF: u32 = 1u32 << (((64 - 2) - 52) - 1);

#[no_mangle]
pub extern "C" fn roundpack_sf64__part3050(rm: c_int, rnd_bits: u32, a_mant: u64) -> u64 {
    let mut mant = a_mant;
    if rm == RM_RNE && rnd_bits == RND_HALF {
        mant &= !1u64;
    }
    mant
}
}

// ── `if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)` · softfp_template.h:154-155 · chunk 3172 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//             a_mant &= ~1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3172 {
// Port of the RNE tie-breaking fixup from roundpack_sf128 (softfp_template.h 154-155).
// rm == RM_RNE (0) and rnd_bits == (1 << (((128 - 2) - 112) - 1)) == (1 << 13):
// clear the lowest bit of a_mant in place.

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3172(rm: core::ffi::c_int, rnd_bits: u32, a_mant: *mut u128) {
    // RM_RNE == 0
    if rm == 0 && rnd_bits == (1u32 << 13) {
        if !a_mant.is_null() {
            unsafe {
                let v = core::ptr::read_unaligned(a_mant);
                core::ptr::write_unaligned(a_mant, v & !1u128);
            }
        }
    }
}
}

// ── `a_exp += a_mant >> (23 + 1);` · softfp_template.h:158-158 · chunk 2939 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += a_mant >> (23 + 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2939 {
use core::ffi::c_int;

// Port of the C statement:  a_exp += a_mant >> (23 + 1);
// In C the addition is performed in unsigned int (uint32_t) because the
// shift operand is uint32_t, so the result is wrapped before being stored
// back into the int a_exp.
#[no_mangle]
pub extern "C" fn roundpack_sf32__part2939(a_exp: c_int, a_mant: u32) -> c_int {
    (a_exp as u32).wrapping_add(a_mant >> 24) as c_int
}
}

// ── `a_exp += a_mant >> (52 + 1);` · softfp_template.h:158-158 · chunk 3051 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += a_mant >> (52 + 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3051 {
// TARGET: a_exp += a_mant >> (52 + 1);
// a_exp is a C `int`, a_mant a `uint64_t`. The shift result fits in c_int,
// so the addition wraps exactly as in C.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn roundpack_sf64__part3051(a_exp: c_int, a_mant: u64) -> c_int {
    a_exp.wrapping_add((a_mant >> 53) as c_int)
}
}

// ── `a_exp += a_mant >> (112 + 1);` · softfp_template.h:158-158 · chunk 3173 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += a_mant >> (112 + 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3173 {
// Ported piece of roundpack_sf128: a_exp += a_mant >> (112 + 1);
// uint128_t is unsigned __int128, mirrored as u128.

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3173(a_exp: i32, a_mant: u128) -> i32 {
    // a_mant >> (112 + 1) stays a uint128_t, then is converted to int
    // (C truncates on conversion to a narrower signed integer).
    let inc: u128 = a_mant >> 113u32;
    let inc_i: i32 = inc as i32;
    // int addition: wrap instead of panicking on overflow
    a_exp.wrapping_add(inc_i)
}
}

// ── `if (a_mant <= (((uint32_t)1 << 23) - 1) … else` · softfp_template.h:159-173 · chunk 2940 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant <= (((uint32_t)1 << 23) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 8) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 8) - 1) - 1;
//                 a_mant = (((uint32_t)1 << 23) - 1);
//             } else {
//     
//                 a_exp = ((1 << 8) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2940 {
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2940(
    a_exp: *mut c_int,
    a_mant: *mut c_uint,
    addend: c_uint,
    pfflags: *mut c_uint,
) {
    unsafe {
        if *a_mant <= ((1u32 << 23) - 1) {
            *a_exp = 0;
        } else if *a_exp >= ((1i32 << 8) - 1) {
            if addend == 0 {
                *a_exp = ((1i32 << 8) - 1) - 1;
                *a_mant = (1u32 << 23) - 1;
            } else {
                *a_exp = (1i32 << 8) - 1;
                *a_mant = 0;
            }
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
}
}

// ── `if (a_mant <= (((uint64_t)1 << 52) - 1) … else` · softfp_template.h:159-173 · chunk 3052 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant <= (((uint64_t)1 << 52) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 11) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 11) - 1) - 1;
//                 a_mant = (((uint64_t)1 << 52) - 1);
//             } else {
//     
//                 a_exp = ((1 << 11) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3052 {
#[no_mangle]
pub extern "C" fn roundpack_sf64__part3052(
    a_exp: *mut core::ffi::c_int,
    a_mant: *mut u64,
    addend: u32,
    pfflags: *mut u32,
) {
    unsafe {
        let exp = *a_exp;
        let mant = *a_mant;
        if mant <= ((1u64 << 52) - 1) {
            *a_exp = 0;
        } else if exp >= ((1i32 << 11) - 1) {
            if addend == 0 {
                *a_exp = ((1i32 << 11) - 1) - 1;
                *a_mant = (1u64 << 52) - 1;
            } else {
                *a_exp = (1i32 << 11) - 1;
                *a_mant = 0;
            }
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
}
}

// ── `if (a_mant <= (((uint128_t)1 << 112) - 1) … else` · softfp_template.h:159-173 · chunk 3174 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant <= (((uint128_t)1 << 112) - 1)) {
//     
//             a_exp = 0;
//         } else if (a_exp >= ((1 << 15) - 1)) {
//     
//             if (addend == 0) {
//                 a_exp = ((1 << 15) - 1) - 1;
//                 a_mant = (((uint128_t)1 << 112) - 1);
//             } else {
//     
//                 a_exp = ((1 << 15) - 1);
//                 a_mant = 0;
//             }
//             *pfflags |= (1 << 2) | (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3174 {
// Port of the exponent/mantissa overflow-and-subnormal fixup tail of
// roundpack_sf128 (softfp_template.h lines 159-173).
//
// Reads: a_mant (uint128_t), a_exp (int), addend (uint32_t), *pfflags
// Writes: a_mant, a_exp, *pfflags

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3174(
    pa_mant: *mut u128,
    pa_exp: *mut core::ffi::c_int,
    addend: u32,
    pfflags: *mut u32,
) {
    unsafe {
        let mut a_mant: u128 = *pa_mant;
        let mut a_exp: core::ffi::c_int = *pa_exp;

        if a_mant <= ((1u128 << 112) - 1) {
            a_exp = 0;
        } else if a_exp >= ((1 << 15) - 1) {
            if addend == 0 {
                a_exp = ((1 << 15) - 1) - 1;
                a_mant = (1u128 << 112) - 1;
            } else {
                a_exp = (1 << 15) - 1;
                a_mant = 0;
            }
            *pfflags |= (1 << 2) | (1 << 0);
        }

        *pa_mant = a_mant;
        *pa_exp = a_exp;
    }
}
}

// ── `return pack_sf32(a_sign, a_exp, a_mant);` · softfp_template.h:174-174 · chunk 2941 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return pack_sf32(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2941 {
// Port of the tail of roundpack_sf32: pack the sign/exp/mant into an f32 bit pattern.
// pack_sf32 is a static inline C helper with no linker symbol, so its logic is
// reproduced here as a private Rust function.

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

#[no_mangle]
pub extern "C" fn roundpack_sf32__part2941(a_sign: u32, a_exp: i32, a_mant: u32) -> u32 {
    // C: pack_sf32(a_sign, a_exp, a_mant) -- a_exp (int) is converted to uint32_t.
    pack_sf32(a_sign, a_exp as u32, a_mant)
}
}

// ── `return pack_sf64(a_sign, a_exp, a_mant);` · softfp_template.h:174-174 · chunk 3053 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pack_sf64(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3053 {
// Port of the final `return pack_sf64(a_sign, a_exp, a_mant);` of
// roundpack_sf64. pack_sf64 is a static inline C helper with no linker
// symbol, so its logic is mirrored here as a private Rust fn.
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn roundpack_sf64__part3053(a_sign: u32, a_exp: i32, a_mant: u64) -> u64 {
    // The C helper takes the exponent as uint32_t; the implicit int -> uint32_t
    // conversion is reproduced by a wrapping `as` cast.
    pack_sf64(a_sign, a_exp as u32, a_mant)
}
}

// ── `return pack_sf128(a_sign, a_exp, a_mant);` · softfp_template.h:174-174 · chunk 3175 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pack_sf128(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3175 {
// Port of the tail of roundpack_sf128: pack_sf128(a_sign, a_exp, a_mant)
// pack_sf128 is a static inline helper, so its logic is mirrored here.

#[no_mangle]
pub extern "C" fn roundpack_sf128__part3175(a_sign: u32, a_exp: i32, a_mant: u128) -> u128 {
    // pack_sf128 takes (uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
    let exp_bits: u128 = (a_exp as u32) as u128;
    ((a_sign as u128) << 127) | (exp_bits << 112) | (a_mant & ((1u128 << 112) - 1))
}
}

// ── `normalize_sf32` · softfp_template.h:178-187 · chunk 2942 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int shift;
//         shift = clz32(a_mant) - (32 - 1 - (32 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2942 {
// Port of: static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                          RoundingModeEnum rm, uint32_t *pfflags)
// from softfp_template.h.  The helpers it calls (clz32, rshift_rnd32, pack_sf32 and
// roundpack_sf32) are all `static`/`static inline` in the C translation unit, so their
// logic is reproduced here as private Rust functions.

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

// RoundingModeEnum values (typedef enum { RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM }).
const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn roundpack_sf32(a_sign: u32, a_exp: c_int, a_mant: u32, rm: c_uint, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (((32 - 2) - 23) - 1);
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        // RM_RDN / RM_RUP and anything else (default: case)
        if (a_sign ^ (rm & 1)) != 0 {
            addend = (1u32 << ((32 - 2) - 23)) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp;
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));

    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                183,
                b"normalize_sf32\0".as_ptr() as *const c_char,
            );
        }
    }

    a_exp -= shift;
    let a_mant = a_mant << (shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `normalize_sf64` · softfp_template.h:178-187 · chunk 3054 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t normalize_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int shift;
//         shift = clz64(a_mant) - (64 - 1 - (64 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3054 {
// Rust port of softfp_template.h normalize_sf64() for the 64-bit float format.
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::c_int;

// C: typedef enum { RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM } RoundingModeEnum;
type RoundingModeEnum = c_int;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RMM: RoundingModeEnum = 4;

const F_SIZE: i32 = 64;
const MANT_SIZE: i32 = 52;
const EXP_MASK: i32 = (1 << 11) - 1;
const RND_SIZE: i32 = (64 - 2) - 52;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

// static inline int clz64(uint64_t a)
#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (F_SIZE - 1))
        | ((a_exp as u64) << MANT_SIZE)
        | (a_mant & (((1u64) << MANT_SIZE) - 1))
}

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= F_SIZE {
            a = (a != 0) as u64;
        } else {
            let dd = d as u32;
            let mask = ((1u64) << dd) - 1;
            a = (a >> dd) | ((a & mask != 0) as u64);
        }
    }
    a
}

// static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = if rm == RM_RNE || rm == RM_RMM {
        1u32 << (RND_SIZE - 1)
    } else if rm == RM_RTZ {
        0
    } else {
        // default: RM_RDN, RM_RUP
        if (a_sign ^ ((rm & 1) as u32)) != 0 {
            (1u32 << RND_SIZE) - 1
        } else {
            0
        }
    };

    let rnd_bits: u32;
    // potentially subnormal
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0
            || a_mant.wrapping_add(addend as u64) < ((1u64) << (F_SIZE - 1));
        let diff = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & (((1u64) << RND_SIZE) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u64) << RND_SIZE) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> RND_SIZE;
    // half way: select even result
    if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !1u64;
    }
    // Note the rounding adds at least 1, so this is the maximum value
    a_exp = (a_exp as i64)
        .wrapping_add((a_mant >> (MANT_SIZE + 1)) as i64) as i32;
    if a_mant <= (((1u64) << MANT_SIZE) - 1) {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        // overflow
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = ((1u64) << MANT_SIZE) - 1;
        } else {
            // infinity
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

// static uint64_t normalize_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
#[no_mangle]
pub extern "C" fn normalize_sf64(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant) - (F_SIZE - 1 - (F_SIZE - 2));
    // assert(shift >= 0); never violated for valid callers
    let new_exp = a_exp.wrapping_sub(shift);
    let new_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, new_exp, new_mant, rm, pfflags)
}
}

// ── `normalize_sf128` · softfp_template.h:178-187 · chunk 3176 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t normalize_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int shift;
//         shift = clz128(a_mant) - (128 - 1 - (128 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3176 {
// Port of softfp_template.h lines 178-187:
//   static uint128_t normalize_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                    RoundingModeEnum rm, uint32_t *pfflags)
//
// The helpers it uses (clz128, rshift_rnd128, pack_sf128, roundpack_sf128) are all
// `static` in the C program, so they have no linker symbols and are re-implemented
// here as private Rust functions with the same semantics.

use core::ffi::c_int;

// typedef unsigned __int128 uint128_t;
type Uint128 = u128;

// typedef enum { RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM } RoundingModeEnum;
type RoundingModeEnum = c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

extern "C" {
    // libc abort(); the original code calls __assert_fail() here, which aborts.
    fn abort() -> !;
}

// static inline int clz128(uint128_t a)
#[inline]
fn clz128(a: Uint128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: Uint128, d: c_int) -> Uint128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as Uint128;
        } else {
            let mask: Uint128 = ((1u128) << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as Uint128;
        }
    }
    a
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: Uint128) -> Uint128 {
    ((a_sign as Uint128) << (128 - 1))
        | ((a_exp as Uint128) << 112)
        | (a_mant & (((1u128) << 112) - 1))
}

// static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                  RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf128(
    a_sign: u32,
    a_exp: c_int,
    a_mant: Uint128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> Uint128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let mut diff: c_int;
    let mut addend: u32;
    let mut rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((128 - 2) - 112) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and the default case
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as Uint128) < ((1u128) << (128 - 1));
        diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & (((1u128) << ((128 - 2) - 112)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u128) << ((128 - 2) - 112)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as Uint128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> (112 + 1)) as c_int;
    if a_mant <= (((1u128) << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = ((1u128) << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub unsafe extern "C" fn normalize_sf128(
    a_sign: u32,
    a_exp: c_int,
    a_mant: Uint128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> Uint128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    let shift: c_int = clz128(a_mant) - (128 - 1 - (128 - 2));

    // assert(shift >= 0); the original calls __assert_fail() when it fails.
    if shift < 0 {
        abort();
    }

    a_exp -= shift;
    a_mant <<= shift as u32;
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `int shift; shift = clz32(a_mant) - (32 - 1 - (32 - 2)); ((void) sizeof (__assert_single_arg ( s…` · softfp_template.h:181-186 · chunk 2943 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = clz32(a_mant) - (32 - 1 - (32 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2943 {
// Port of normalize_sf32's tail (softfp_template.h lines 181-186) plus the
// static helpers it needs (clz32, rshift_rnd32, pack_sf32, roundpack_sf32),
// which have no linker symbols and therefore live here as private fns.

use core::ffi::{c_char, c_int, c_uint};

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

static ASSERT_MSG: &[u8] = b"shift >= 0\0";
static ASSERT_FILE: &[u8] = b"softfp_template.h\0";
static ASSERT_FUNC: &[u8] = b"normalize_sf32\0";

#[inline]
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask = (1u32 << du).wrapping_sub(1);
            a = (a >> du) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                               RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp: c_int = a_exp;
    let mut a_mant: u32 = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | 4 => {
            // RM_RNE (0), RM_RMM (4)
            addend = 1u32 << 6;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN (2), RM_RUP (3) and default
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: c_int = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }

    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 24) as c_int);
    if a_mant <= 0x007f_ffffu32 {
        a_exp = 0;
    } else if a_exp >= 255 {
        if addend == 0 {
            a_exp = 254;
            a_mant = 0x007f_ffffu32;
        } else {
            a_exp = 255;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }

    pack_sf32(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize_sf32__part2943(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));

    if shift < 0 {
        // mirrors: assert(shift >= 0);  (__assert_fail never returns)
        unsafe {
            __assert_fail(
                ASSERT_MSG.as_ptr() as *const c_char,
                ASSERT_FILE.as_ptr() as *const c_char,
                183 as c_uint,
                ASSERT_FUNC.as_ptr() as *const c_char,
            );
        }
    }

    let a_exp_upd: c_int = a_exp.wrapping_sub(shift);
    let a_mant_upd: u32 = a_mant.wrapping_shl(shift as u32);

    roundpack_sf32(a_sign, a_exp_upd, a_mant_upd, rm, pfflags)
}
}

// ── `int shift; shift = clz64(a_mant) - (64 - 1 - (64 - 2)); ((void) sizeof (__assert_single_arg ( s…` · softfp_template.h:181-186 · chunk 3055 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = clz64(a_mant) - (64 - 1 - (64 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3055 {
// Rust port of the tail of normalize_sf64 (softfp_template.h lines 181-186):
// normalization shift + assert, then roundpack_sf64 (static helpers inlined here).

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask = (1u64 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u64;
        }
    }
    a
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn roundpack_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (default)
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    unsafe {
        if a_exp <= 0 {
            let is_subnormal: bool =
                a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
            let diff: i32 = 1 - a_exp;
            a_mant = rshift_rnd64(a_mant, diff);
            rnd_bits = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1))) as u32;
            if is_subnormal && rnd_bits != 0 {
                *pfflags |= 1 << 1;
            }
            a_exp = 1;
        } else {
            rnd_bits = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1))) as u32;
        }
        if rnd_bits != 0 {
            *pfflags |= 1 << 0;
        }
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

        if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }

        a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
        if a_mant <= ((1u64 << 52) - 1) {
            a_exp = 0;
        } else if a_exp >= ((1 << 11) - 1) {
            if addend == 0 {
                a_exp = ((1 << 11) - 1) - 1;
                a_mant = (1u64 << 52) - 1;
            } else {
                a_exp = (1 << 11) - 1;
                a_mant = 0;
            }
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }

    pack_sf64(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub unsafe extern "C" fn normalize_sf64__part3055(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant).wrapping_sub(64 - 1 - (64 - 2));
    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                183u32,
                b"normalize_sf64\0".as_ptr() as *const c_char,
            );
        }
    }
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = a_mant.wrapping_shl(shift as u32);
    unsafe { roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags) }
}
}

// ── `int shift; shift = clz128(a_mant) - (128 - 1 - (128 - 2)); ((void) sizeof (__assert_single_arg…` · softfp_template.h:181-186 · chunk 3177 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = clz128(a_mant) - (128 - 1 - (128 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 183, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         a_mant <<= shift;
//         return roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3177 {
// Port of the normalize/roundpack tail of normalize_sf128 (softfp, 128-bit).
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    // void __assert_fail(const char *__assertion, const char *__file,
    //                    unsigned int __line, const char *__function) __attribute__((noreturn));
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

// static inline int clz128(uint128_t a)
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let d = d as u32;
            let mask = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

// static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                  RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    const RND_SIZE: u32 = 128 - 2 - 112; // IMANT_SIZE - MANT_SIZE == 14
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let mut diff: c_int;
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << (RND_SIZE - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << RND_SIZE) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < ((1u128) << (128 - 1));
        diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << RND_SIZE) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1; // FFLAG_UNDERFLOW
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << RND_SIZE) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0; // FFLAG_INEXACT
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> RND_SIZE;

    if rm == 0 && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !1u128;
    }

    let hi = (a_mant >> (112 + 1)) as u32 as c_int;
    a_exp = a_exp.wrapping_add(hi);
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= (1 << 15) - 1 {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0); // FFLAG_OVERFLOW | FFLAG_INEXACT
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

static ASSERT_SHIFT: &[u8] = b"shift >= 0\x00";
static ASSERT_FILE: &[u8] = b"softfp_template.h\x00";
static ASSERT_FUNC: &[u8] = b"normalize_sf128\x00";

#[no_mangle]
pub extern "C" fn normalize_sf128__part3177(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    let shift: c_int = clz128(a_mant) - (128 - 1 - (128 - 2));
    if !(shift >= 0) {
        unsafe {
            __assert_fail(
                ASSERT_SHIFT.as_ptr() as *const c_char,
                ASSERT_FILE.as_ptr() as *const c_char,
                183,
                ASSERT_FUNC.as_ptr() as *const c_char,
            )
        }
    }
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = a_mant << (shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `normalize2_sf32` · softfp_template.h:191-213 · chunk 2944 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t normalize2_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant1, uint32_t a_mant0,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int l, shift;
//         if (a_mant1 == 0) {
//             l = 32 + clz32(a_mant0);
//         } else {
//             l = clz32(a_mant1);
//         }
//         shift = l - (32 - 1 - (32 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 32) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (32 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 32);
//         }
//         return roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2944 {
// Rust port of static uint32_t normalize2_sf32(uint32_t a_sign, int a_exp,
//     uint32_t a_mant1, uint32_t a_mant0, RoundingModeEnum rm, uint32_t *pfflags)
// The C helpers it calls (clz32, pack_sf32, rshift_rnd32, roundpack_sf32) are
// all `static` in the translation unit, so their logic is mirrored here.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

#[inline]
fn clz32(a: u32) -> c_int {
    // C: if (a == 0) 32 else __builtin_clz(a) -- leading_zeros matches exactly.
    a.leading_zeros() as c_int
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32;
    let mut rnd_bits: u32;

    // switch (rm): RM_RNE, RM_RMM -> 1 << 6 ; RM_RTZ -> 0 ; default (RDN, RUP, ...)
    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << 6;
    } else if rm == RM_RTZ {
        addend = 0;
    } else if (a_sign ^ ((rm as u32) & 1)) != 0 {
        addend = (1u32 << 7) - 1;
    } else {
        addend = 0;
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize2_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant1: u32,
    mut a_mant0: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let l: c_int;
    if a_mant1 == 0 {
        l = 32 + clz32(a_mant0);
    } else {
        l = clz32(a_mant1);
    }
    // (F_SIZE - 1 - IMANT_SIZE) with F_SIZE == 32, IMANT_SIZE == 30
    let shift: c_int = l - ((32 - 1) - (32 - 2));
    // C asserts shift >= 0 here; a failure aborts in C and no valid input reaches it.
    a_exp -= shift;
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u32;
    } else if shift > 0 && shift < 32 {
        a_mant1 = a_mant1.wrapping_shl(shift as u32)
            | a_mant0.wrapping_shr((32 - shift) as u32);
        a_mant0 = a_mant0.wrapping_shl(shift as u32);
        a_mant1 |= (a_mant0 != 0) as u32;
    } else if shift >= 32 {
        a_mant1 = a_mant0.wrapping_shl((shift - 32) as u32);
    }
    // shift < 0 is undefined/aborted in C; leave the mantissa untouched.
    roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags)
}
}

// ── `normalize2_sf64` · softfp_template.h:191-213 · chunk 3056 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t normalize2_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant1, uint64_t a_mant0,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int l, shift;
//         if (a_mant1 == 0) {
//             l = 64 + clz64(a_mant0);
//         } else {
//             l = clz64(a_mant1);
//         }
//         shift = l - (64 - 1 - (64 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 64) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (64 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 64);
//         }
//         return roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3056 {
// Port of softfp_template.h normalize2_sf64() (F_SIZE = 64, IMANT_SIZE = 52)
// plus private ports of the static helpers it uses (clz64, rshift_rnd64,
// pack_sf64, roundpack_sf64), which have no linker symbols in the C program.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

// static inline int clz64(uint64_t a)
#[inline]
fn clz64(a: u64) -> i32 {
    // leading_zeros() already yields 64 for a == 0
    a.leading_zeros() as i32
}

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

// static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u64 = a_mant_in;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((64 - 2) - 52) - 1),
        RM_RTZ => 0,
        // default, RM_RDN, RM_RUP
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((64 - 2) - 52)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: c_int = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as c_int);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize2_sf64(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u64,
    a_mant0: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let l: c_int;
    if a_mant1 == 0 {
        l = 64 + clz64(a_mant0);
    } else {
        l = clz64(a_mant1);
    }
    let shift: c_int = l - (64 - 1 - (64 - 2));
    // C: assert(shift >= 0);
    let a_exp = a_exp.wrapping_sub(shift);
    let mut m1: u64 = a_mant1;
    let mut m0: u64 = a_mant0;
    if shift == 0 {
        m1 |= (m0 != 0) as u64;
    } else if shift < 64 {
        let s = shift as u32;
        m1 = m1.wrapping_shl(s) | m0.wrapping_shr(64u32.wrapping_sub(s));
        m0 = m0.wrapping_shl(s);
        m1 |= (m0 != 0) as u64;
    } else {
        let s = (shift as u32).wrapping_sub(64);
        m1 = m0.wrapping_shl(s);
    }
    roundpack_sf64(a_sign, a_exp, m1, rm, pfflags)
}
}

// ── `normalize2_sf128` · softfp_template.h:191-213 · chunk 3178 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t normalize2_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant1, uint128_t a_mant0,
//                                 RoundingModeEnum rm, uint32_t *pfflags)
//     {
//         int l, shift;
//         if (a_mant1 == 0) {
//             l = 128 + clz128(a_mant0);
//         } else {
//             l = clz128(a_mant1);
//         }
//         shift = l - (128 - 1 - (128 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
//         if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 128) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (128 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 128);
//         }
//         return roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3178 {
// Port of softfp_template.h normalize2_sf128 (128-bit softfloat)
// plus its private (static / static inline) helpers: clz128, rshift_rnd128,
// roundpack_sf128, pack_sf128.
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_uint};

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

extern "C" {
    // from libc; declared with a non-diverging return so that this file stays
    // compatible with every rustc; it never actually returns.
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    );
}

// static inline int clz128(uint128_t a)
#[inline]
fn clz128(a: u128) -> c_int {
    // The C version: 128 when a == 0, else clz of the high word, or
    // clz of the low word + 64.  u128::leading_zeros is identical.
    a.leading_zeros() as c_int
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
#[inline]
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d < 0 {
            // unreachable in practice (C would shift by a negative amount)
            a = a >> (((d as u32) & 127) as u32);
        } else {
            let sh = d as u32;
            let mask: u128 = (1u128 << sh) - 1;
            a = (a >> sh) | ((a & mask) != 0) as u128;
        }
    }
    a
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

// static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                  RoundingModeEnum rm, uint32_t *pfflags)
unsafe fn roundpack_sf128(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp;
    let mut a_mant: u128 = a_mant;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((128 - 2) - 112) - 1),
        RM_RTZ => 0,
        _ => {
            // RM_RDN / RM_RUP and anything else
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((128 - 2) - 112)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & (((1u32 << ((128 - 2) - 112)) - 1) as u128)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u32 << ((128 - 2) - 112)) - 1) as u128)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1 << 0;
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    // a_exp += a_mant >> (112 + 1);  (computed in unsigned __int128, then
    // truncated back into the int)
    a_exp = ((a_exp as u128).wrapping_add(a_mant >> (112 + 1)) as u32) as c_int;

    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        *pfflags |= (1 << 2) | (1 << 0);
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub unsafe extern "C" fn normalize2_sf128(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u128,
    a_mant0: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let l: c_int;
    if a_mant1 == 0 {
        l = 128 + clz128(a_mant0);
    } else {
        l = clz128(a_mant1);
    }
    let shift: c_int = l - (128 - 1 - (128 - 2));

    if !(shift >= 0) {
        __assert_fail(
            b"shift >= 0\0".as_ptr() as *const c_char,
            b"softfp_template.h\0".as_ptr() as *const c_char,
            201,
            b"normalize2_sf128\0".as_ptr() as *const c_char,
        );
        return 0; // unreachable: __assert_fail does not return
    }

    let a_exp = a_exp - shift;
    let mut a_mant1 = a_mant1;
    let mut a_mant0 = a_mant0;
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u128;
    } else if shift < 128 {
        let sh = shift as u32;
        a_mant1 = (a_mant1 << sh) | (a_mant0 >> (128 - sh));
        a_mant0 <<= sh;
        a_mant1 |= (a_mant0 != 0) as u128;
    } else {
        a_mant1 = a_mant0 << (((shift - 128) as u32) & 127);
    }
    roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags)
}
}

// ── `if (a_mant1 == 0) … else` · softfp_template.h:195-199 · chunk 2945 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant1 == 0) {
//             l = 32 + clz32(a_mant0);
//         } else {
//             l = clz32(a_mant1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2945 {
use core::ffi::c_int;

// clz32 is a static inline helper with no linker symbol; its logic is
// mirrored here: for a == 0 the result is 32, which is exactly what
// u32::leading_zeros() returns.
#[inline]
fn clz32(a: u32) -> c_int {
    a.leading_zeros() as c_int
}

#[no_mangle]
pub extern "C" fn normalize2_sf32__part2945(a_mant1: u32, a_mant0: u32) -> c_int {
    let l: c_int;
    if a_mant1 == 0 {
        l = 32 + clz32(a_mant0);
    } else {
        l = clz32(a_mant1);
    }
    l
}
}

// ── `if (a_mant1 == 0) … else` · softfp_template.h:195-199 · chunk 3057 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant1 == 0) {
//             l = 64 + clz64(a_mant0);
//         } else {
//             l = clz64(a_mant1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3057 {
// Compute the normalized-length helper used by normalize2_sf64.
// clz64 is a static inline C helper (no linker symbol): its logic is mirrored here.
use core::ffi::c_int;

#[inline]
fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

#[no_mangle]
pub extern "C" fn normalize2_sf64__part3057(a_mant1: u64, a_mant0: u64) -> c_int {
    if a_mant1 == 0 {
        64 + clz64(a_mant0)
    } else {
        clz64(a_mant1)
    }
}
}

// ── `if (a_mant1 == 0) … else` · softfp_template.h:195-199 · chunk 3179 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_mant1 == 0) {
//             l = 128 + clz128(a_mant0);
//         } else {
//             l = clz128(a_mant1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3179 {
// Port of the leading-zero count helper (static inline in the C translation
// unit, so its logic is mirrored here as a private Rust function).
fn clz128(a: u128) -> core::ffi::c_int {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as core::ffi::c_int
        } else {
            al.leading_zeros() as core::ffi::c_int + 64
        }
    }
}

#[no_mangle]
pub extern "C" fn normalize2_sf128__part3179(
    a_mant1: u128,
    a_mant0: u128,
) -> core::ffi::c_int {
    if a_mant1 == 0 {
        128 + clz128(a_mant0)
    } else {
        clz128(a_mant1)
    }
}
}

// ── `shift = l - (32 - 1 - (32 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __extensi…` · softfp_template.h:200-202 · chunk 2946 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = l - (32 - 1 - (32 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2946 {
use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(assertion: *const c_char, file: *const c_char, line: c_uint, function: *const c_char) -> !;
}

#[no_mangle]
pub extern "C" fn normalize2_sf32__part2946(l: c_int, a_exp: c_int) -> c_int {
    let shift: c_int = l.wrapping_sub(32 - 1 - (32 - 2));
    if !(shift >= 0) {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                201u32,
                b"normalize2_sf32\0".as_ptr() as *const c_char,
            )
        }
    }
    a_exp.wrapping_sub(shift)
}
}

// ── `shift = l - (64 - 1 - (64 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __extensi…` · softfp_template.h:200-202 · chunk 3058 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = l - (64 - 1 - (64 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3058 {
// Port of the shift/assert/a_exp-adjust piece of normalize2_sf64.

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    ) -> !;
}

static ASSERTION: &[u8] = b"shift >= 0\0";
static FILE: &[u8] = b"softfp_template.h\0";
static FUNCTION: &[u8] = b"normalize2_sf64\0";

#[no_mangle]
pub extern "C" fn normalize2_sf64__part3058(
    l: c_int,
    shift: *mut c_int,
    a_exp: *mut c_int,
) {
    unsafe {
        let s: c_int = l - (64 - 1 - (64 - 2));
        *shift = s;
        if !(s >= 0) {
            __assert_fail(
                ASSERTION.as_ptr() as *const c_char,
                FILE.as_ptr() as *const c_char,
                201 as c_uint,
                FUNCTION.as_ptr() as *const c_char,
            );
        }
        *a_exp -= s;
    }
}
}

// ── `shift = l - (128 - 1 - (128 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __exten…` · softfp_template.h:200-202 · chunk 3180 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = l - (128 - 1 - (128 - 2));
//         
//        ((void) sizeof (__assert_single_arg (
//        shift >= 0
//        )), __extension__ ({ if (
//        shift >= 0
//        ) ; else __assert_fail (
//        "shift >= 0"
//        , "softfp_template.h", 201, __extension__ __PRETTY_FUNCTION__); }))
//                          ;
//         a_exp -= shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3180 {
// Ported piece of softfp_template.h normalize2_sf128 (lines 200-202):
//   shift = l - (128 - 1 - (128 - 2));
//   assert(shift >= 0);
//   a_exp -= shift;
use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn normalize2_sf128__part3180(l: c_int, a_exp: *mut c_int) -> c_int {
    // (128 - 1 - (128 - 2)) == 1
    let shift: c_int = l.wrapping_sub(1);

    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                201u32,
                b"normalize2_sf128\0".as_ptr() as *const c_char,
            );
        }
    }

    unsafe {
        *a_exp -= shift;
    }

    shift
}
}

// ── `if (shift == 0) … else` · softfp_template.h:203-211 · chunk 2947 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 32) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (32 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 32);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2947 {
// Port of the mantissa-shift block of normalize2_sf32 (softfp_template.h 203-211).
// Takes the two mantissa halves and the shift amount; returns the new a_mant1.

#[no_mangle]
pub extern "C" fn normalize2_sf32__part2947(a_mant1: u32, a_mant0: u32, shift: i32) -> u32 {
    let mut m1: u32 = a_mant1;
    let mut m0: u32 = a_mant0;
    let s: u32 = shift as u32;

    if shift == 0 {
        m1 |= (m0 != 0) as u32;
    } else if shift < 32 {
        m1 = m1.wrapping_shl(s) | m0.wrapping_shr(32u32.wrapping_sub(s));
        m0 = m0.wrapping_shl(s);
        m1 |= (m0 != 0) as u32;
    } else {
        m1 = m0.wrapping_shl(s.wrapping_sub(32));
    }

    m1
}
}

// ── `if (shift == 0) … else` · softfp_template.h:203-211 · chunk 3059 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 64) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (64 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 64);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3059 {
// Ported piece of softfp_template.h normalize2_sf64 (lines 203-211):
// shift the two-word mantissa into place and fold the sticky bits.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn normalize2_sf64__part3059(shift: c_int, a_mant1: *mut u64, a_mant0: u64) {
    // Caller guarantees a_mant1 is a valid pointer to its local variable.
    let mut m1: u64 = unsafe { *a_mant1 };
    let mut m0: u64 = a_mant0;

    if shift == 0 {
        m1 |= (m0 != 0) as u64;
    } else if shift < 64 {
        // shift in 1..63 here, so 64 - shift in 1..63 as well.
        m1 = m1.wrapping_shl(shift as u32) | m0.wrapping_shr((64 - shift) as u32);
        m0 = m0.wrapping_shl(shift as u32);
        m1 |= (m0 != 0) as u64;
    } else {
        m1 = m0.wrapping_shl((shift - 64) as u32);
    }

    unsafe {
        *a_mant1 = m1;
    }
}
}

// ── `if (shift == 0) … else` · softfp_template.h:203-211 · chunk 3181 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift == 0) {
//             a_mant1 |= (a_mant0 != 0);
//         } else if (shift < 128) {
//             a_mant1 = (a_mant1 << shift) | (a_mant0 >> (128 - shift));
//             a_mant0 <<= shift;
//             a_mant1 |= (a_mant0 != 0);
//         } else {
//             a_mant1 = a_mant0 << (shift - 128);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3181 {
use core::ffi::c_int;

// Port of the shift/normalize block of normalize2_sf128 (softfp_template.h 203-211).
// a_mant1 and a_mant0 are mutated in place, so they are passed by pointer.
#[no_mangle]
pub extern "C" fn normalize2_sf128__part3181(
    shift: c_int,
    pa_mant1: *mut u128,
    pa_mant0: *mut u128,
) {
    // No panics may cross the FFI boundary; bail out on impossible null pointers.
    if pa_mant1.is_null() || pa_mant0.is_null() {
        return;
    }
    unsafe {
        let mut a_mant1: u128 = *pa_mant1;
        let mut a_mant0: u128 = *pa_mant0;

        if shift == 0 {
            // a_mant1 |= (a_mant0 != 0);
            if a_mant0 != 0 {
                a_mant1 |= 1u128;
            }
        } else if shift < 128 {
            let s = shift as u32;
            a_mant1 = a_mant1.wrapping_shl(s) | a_mant0.wrapping_shr(128 - s);
            a_mant0 = a_mant0.wrapping_shl(s);
            if a_mant0 != 0 {
                a_mant1 |= 1u128;
            }
        } else {
            a_mant1 = a_mant0.wrapping_shl((shift - 128) as u32);
        }

        *pa_mant1 = a_mant1;
        *pa_mant0 = a_mant0;
    }
}
}

// ── `return roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags);` · softfp_template.h:212-212 · chunk 2948 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2948 {
// Port of the TARGET statement of normalize2_sf32:
//   return roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags);
// roundpack_sf32 is a C `static` function (no linker symbol), so its logic is
// reproduced here as a private Rust fn, together with the static helpers it uses.

use core::ffi::{c_int, c_uint};

const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

#[inline]
fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | (((a & mask) != 0) as u32);
        }
    }
    a
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << 6,
        RM_RTZ => 0,
        _ => {
            // RM_RDN, RM_RUP (and anything else)
            if (a_sign ^ (rm & 1)) != 0 {
                (1u32 << 7) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> 24) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1i32 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | 1;
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize2_sf32__part2948(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags)
}
}

// ── `return roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags);` · softfp_template.h:212-212 · chunk 3060 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3060 {
// Port of the tail of normalize2_sf64: roundpack_sf64 plus its static helpers
// (rshift_rnd64, pack_sf64), which have no linker symbols.

use core::ffi::c_int;

const F_QNAN64: u64 = (((1u64 << 11) - 1) << 52) | (1u64 << (52 - 1));

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn rshift_rnd64(mut a: u64, d: c_int) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask = (1u64 << d) - 1;
            a = (a >> d) | (((a & mask) != 0) as u64);
        }
    }
    a
}

fn roundpack_sf64(a_sign: u32, a_exp: c_int, a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let mut a_exp: c_int = a_exp;
    let mut a_mant: u64 = a_mant;
    let addend: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    match rm {
        0 | 4 => {
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        1 => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & (((1u32 << ((64 - 2) - 52)) - 1) as u64)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u32 << ((64 - 2) - 52)) - 1) as u64)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as c_int;
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    let _ = &F_QNAN64;
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn normalize2_sf64__part3060(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u64,
    rm: u32,
    pfflags: *mut u32,
) -> u64 {
    roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags)
}
}

// ── `return roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags);` · softfp_template.h:212-212 · chunk 3182 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3182 {
// Port of roundpack_sf128 (a static C function, so its logic is inlined here)
// plus its static helpers rshift_rnd128 and pack_sf128.

use core::ffi::c_int;

const MANT_MASK: u128 = (1u128 << 112) - 1;
const RND_MASK: u32 = (1 << (((128 - 2) - 112) as u32)) - 1; // (1 << 14) - 1
const HALF: u32 = 1 << ((((128 - 2) - 112) - 1) as u32); // 1 << 6

#[inline]
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else if d > 0 {
            let dd = d as u32;
            let mask = (1u128 << dd) - 1;
            (a >> dd) | ((a & mask) != 0) as u128
        } else {
            a
        }
    } else {
        a
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

unsafe fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let addend: u32 = match rm {
        0 | 4 => HALF,
        1 => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1 << (((128 - 2) - 112) as u32)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & (RND_MASK as u128)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (RND_MASK as u128)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1 << 0;
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> (((128 - 2) - 112) as u32);

    if rm == 0 && rnd_bits == HALF {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (112 + 1)) as c_int);
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        *pfflags |= (1 << 2) | (1 << 0);
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub unsafe extern "C" fn normalize2_sf128__part3182(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags)
}
}

// ── `issignan_sf32` · softfp_template.h:215-222 · chunk 2949 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL issignan_sf32(uint32_t a)
//     {
//         uint32_t a_exp1;
//         uint32_t a_mant;
//         a_exp1 = (a >> (23 - 1)) & ((1 << (8 + 1)) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         return (a_exp1 == (2 * ((1 << 8) - 1)) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2949 {
// issignan_sf32: signaling NaN test for 32-bit floats (23-bit mantissa, 8-bit exponent).

#[no_mangle]
pub extern "C" fn issignan_sf32(a: u32) -> core::ffi::c_int {
    let a_exp1: u32 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    let r: i32 = if a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0 {
        1
    } else {
        0
    };
    r as core::ffi::c_int
}
}

// ── `issignan_sf64` · softfp_template.h:215-222 · chunk 3061 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL issignan_sf64(uint64_t a)
//     {
//         uint32_t a_exp1;
//         uint64_t a_mant;
//         a_exp1 = (a >> (52 - 1)) & ((1 << (11 + 1)) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         return (a_exp1 == (2 * ((1 << 11) - 1)) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3061 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn issignan_sf64(a: u64) -> c_int {
    let a_exp1: u32 = ((a >> (52 - 1)) & (((1u64 << (11 + 1)) - 1) as u64)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    let r: bool = a_exp1 == (2 * ((1u32 << 11) - 1)) && a_mant != 0;
    r as c_int
}
}

// ── `issignan_sf128` · softfp_template.h:215-222 · chunk 3183 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL issignan_sf128(uint128_t a)
//     {
//         uint32_t a_exp1;
//         uint128_t a_mant;
//         a_exp1 = (a >> (112 - 1)) & ((1 << (15 + 1)) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         return (a_exp1 == (2 * ((1 << 15) - 1)) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3183 {
#[no_mangle]
pub extern "C" fn issignan_sf128(a: u128) -> core::ffi::c_int {
    let a_exp1: u32 = ((a >> (112 - 1)) & (((1u128 << (15 + 1)) - 1) as u128)) as u32;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    let r = a_exp1 == (2 * ((1u32 << 15) - 1)) && a_mant != 0;
    if r { 1 } else { 0 }
}
}

// ── `uint32_t a_exp1; uint32_t a_mant; a_exp1 = (a >> (23 - 1)) & ((1 << (8 + 1)) - 1); a_mant = a &…` · softfp_template.h:217-221 · chunk 2950 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp1;
//         uint32_t a_mant;
//         a_exp1 = (a >> (23 - 1)) & ((1 << (8 + 1)) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         return (a_exp1 == (2 * ((1 << 8) - 1)) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2950 {
#[no_mangle]
pub extern "C" fn issignan_sf32__part2950(a: u32) -> core::ffi::c_int {
    let a_exp1: u32 = (a >> (23 - 1)) & (((1u32 << (8 + 1)) - 1) as u32);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    (a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0) as core::ffi::c_int
}
}

// ── `uint32_t a_exp1; uint64_t a_mant; a_exp1 = (a >> (52 - 1)) & ((1 << (11 + 1)) - 1); a_mant = a…` · softfp_template.h:217-221 · chunk 3062 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp1;
//         uint64_t a_mant;
//         a_exp1 = (a >> (52 - 1)) & ((1 << (11 + 1)) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         return (a_exp1 == (2 * ((1 << 11) - 1)) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3062 {
// Ported piece of issignan_sf64: exponent/mantissa extraction and NaN test.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn issignan_sf64__part3062(a: u64) -> c_int {
    let a_exp1: u32 = ((a >> (52u32 - 1)) & (((1u64 << (11u32 + 1)) - 1) as u64)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    let res: bool = a_exp1 == (2u32.wrapping_mul((1u32 << 11) - 1)) && a_mant != 0;
    res as c_int
}
}

// ── `uint32_t a_exp1; uint128_t a_mant; a_exp1 = (a >> (112 - 1)) & ((1 << (15 + 1)) - 1); a_mant =…` · softfp_template.h:217-221 · chunk 3184 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp1;
//         uint128_t a_mant;
//         a_exp1 = (a >> (112 - 1)) & ((1 << (15 + 1)) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         return (a_exp1 == (2 * ((1 << 15) - 1)) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3184 {
// Portion of issignan_sf128: exponent/mantissa check for signaling NaN.
#[no_mangle]
pub extern "C" fn issignan_sf128__part3184(a: u128) -> core::ffi::c_int {
    let a_exp1: u32 = ((a >> (112 - 1)) & (((1u128 << (15 + 1)) - 1) as u32 as u128)) as u32;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    ((a_exp1 == (2 * ((1 << 15) - 1)) as u32) && a_mant != 0) as core::ffi::c_int
}
}

// ── `isnan_sf32` · softfp_template.h:224-231 · chunk 2951 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL isnan_sf32(uint32_t a)
//     {
//         uint32_t a_exp;
//         uint32_t a_mant;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         return (a_exp == ((1 << 8) - 1) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2951 {
// Port of softfp_template.h isnan_sf32: BOOL isnan_sf32(uint32_t a)
// BOOL is typedef'd as int in the translation unit.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf32(a: u32) -> c_int {
    let a_exp: u32 = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    if a_exp == ((1u32 << 8) - 1) && a_mant != 0 {
        1
    } else {
        0
    }
}
}

// ── `isnan_sf64` · softfp_template.h:224-231 · chunk 3063 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL isnan_sf64(uint64_t a)
//     {
//         uint32_t a_exp;
//         uint64_t a_mant;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         return (a_exp == ((1 << 11) - 1) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3063 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf64(a: u64) -> c_int {
    let a_exp: u32 = ((a >> 52) & (((1u64 << 11) - 1) as u64)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    if a_exp == (1u32 << 11) - 1 && a_mant != 0 {
        1
    } else {
        0
    }
}
}

// ── `isnan_sf128` · softfp_template.h:224-231 · chunk 3185 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL isnan_sf128(uint128_t a)
//     {
//         uint32_t a_exp;
//         uint128_t a_mant;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         return (a_exp == ((1 << 15) - 1) && a_mant != 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3185 {
// Port of softfp_template.h isnan_sf128 (sfloat128 NaN test).
// BOOL is `typedef int`, uint128_t is `unsigned __int128`.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf128(a: u128) -> c_int {
    // a_exp = (a >> 112) & ((1 << 15) - 1);
    let a_exp: u32 = ((a >> 112) & (((1u128 << 15) - 1) as u128)) as u32;
    // a_mant = a & (((uint128_t)1 << 112) - 1);
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    // return (a_exp == ((1 << 15) - 1) && a_mant != 0);
    if a_exp == ((1u32 << 15) - 1) && a_mant != 0 {
        1
    } else {
        0
    }
}
}

// ── `uint32_t a_exp; uint32_t a_mant; a_exp = (a >> 23) & ((1 << 8) - 1); a_mant = a & (((uint32_t)1…` · softfp_template.h:226-230 · chunk 2952 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp;
//         uint32_t a_mant;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         return (a_exp == ((1 << 8) - 1) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2952 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf32__part2952(a: u32) -> c_int {
    let a_exp: u32 = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    let r: c_int = if a_exp == ((1u32 << 8) - 1) && a_mant != 0 { 1 } else { 0 };
    r
}
}

// ── `uint32_t a_exp; uint64_t a_mant; a_exp = (a >> 52) & ((1 << 11) - 1); a_mant = a & (((uint64_t)…` · softfp_template.h:226-230 · chunk 3064 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp;
//         uint64_t a_mant;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         return (a_exp == ((1 << 11) - 1) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3064 {
// Port of the tail of isnan_sf64: classification of a binary64 bit pattern.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf64__part3064(a: u64) -> c_int {
    let a_exp: u32 = ((a >> 52) & ((1u64 << 11) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    if a_exp == ((1u32 << 11) - 1) && a_mant != 0 {
        1
    } else {
        0
    }
}
}

// ── `uint32_t a_exp; uint128_t a_mant; a_exp = (a >> 112) & ((1 << 15) - 1); a_mant = a & (((uint128…` · softfp_template.h:226-230 · chunk 3186 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_exp;
//         uint128_t a_mant;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         return (a_exp == ((1 << 15) - 1) && a_mant != 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3186 {
// Port of the tail of C isnan_sf128 (softfp_template.h lines 226-230).
// The C source uses "unsigned __int128" (uint128_t), which is Rust's u128,
// and BOOL (typedef int) as the return type.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn isnan_sf128__part3186(a: u128) -> c_int {
    // a_exp = (a >> 112) & ((1 << 15) - 1);  (truncated into a uint32_t)
    let a_exp: u32 = ((a >> 112) & (((1u128 << 15) - 1))) as u32;
    // a_mant = a & (((uint128_t)1 << 112) - 1);
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    // return (a_exp == ((1 << 15) - 1) && a_mant != 0);
    if a_exp == ((1u32 << 15) - 1) && a_mant != 0 {
        1
    } else {
        0
    }
}
}

// ── `add_sf32` · softfp_template.h:234-290 · chunk 2953 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t add_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, a_exp, b_exp;
//         uint32_t tmp, a_mant, b_mant;
//     
//     
//         if ((a & ~((uint32_t)1 << (32 - 1))) < (b & ~((uint32_t)1 << (32 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = (a & (((uint32_t)1 << 23) - 1)) << 3;
//         b_mant = (b & (((uint32_t)1 << 23) - 1)) << 3;
//         if (__builtin_expect(!!(a_exp == ((1 << 8) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint32_t)1 << (23 - 1)) << 3)) || issignan_sf32(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (b_exp == ((1 << 8) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else {
//     
//                 return a;
//             }
//         }
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << (23 + 3);
//         }
//         if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint32_t)1 << (23 + 3);
//         }
//         b_mant = rshift_rnd32(b_mant, a_exp - b_exp);
//         if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
//         a_exp += (((32 - 2) - 23) - 3);
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2953 {
// Port of softfp_template.h add_sf32 (single precision soft-float add).
// Static helpers from the template (clz32, pack_sf32, rshift_rnd32,
// roundpack_sf32, normalize_sf32, issignan_sf32) have no linker symbol and
// are reproduced here as private Rust functions.

use core::ffi::c_int;

const SIGN_MASK: u32 = 1u32 << 31;
const EXP_MASK: u32 = (1u32 << 8) - 1;
const MANT_MASK: u32 = (1u32 << 23) - 1;
const F_QNAN: u32 = (EXP_MASK << 23) | (1u32 << 22);
const RND_SHIFT: u32 = 7; // (F_SIZE - 2) - MANT_SIZE

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn issignan(a: u32) -> bool {
    let a_exp1: u32 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant: u32 = a & MANT_MASK;
    a_exp1 == 2 * EXP_MASK && a_mant != 0
}

fn pack_sf32(a_sign: u32, a_exp: i32, a_mant: u32) -> u32 {
    (a_sign << 31) | ((a_exp as u32) << 23) | (a_mant & MANT_MASK)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let du = d as u32;
            let mask: u32 = (1u32 << du) - 1;
            a = (a >> du) | if (a & mask) != 0 { 1 } else { 0 };
        }
        // d < 0 is undefined in C (shift by a negative amount); leave a alone.
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << (RND_SHIFT - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << RND_SHIFT) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    let rnd_mask: u32 = (1u32 << RND_SHIFT) - 1;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & rnd_mask;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & rnd_mask;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> RND_SHIFT;

    if rm == 0 && rnd_bits == (1u32 << (RND_SHIFT - 1)) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (23 + 1)) as i32);
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= (EXP_MASK as i32) {
        if addend == 0 {
            a_exp = (EXP_MASK as i32) - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK as i32;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u32 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn add_sf32(mut a: u32, mut b: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    // swap so that abs(a) >= abs(b)
    if (a & !SIGN_MASK) < (b & !SIGN_MASK) {
        let tmp: u32 = a;
        a = b;
        b = tmp;
    }

    let mut a_sign: u32 = a >> (32 - 1);
    let b_sign: u32 = b >> (32 - 1);
    let mut a_exp: u32 = (a >> 23) & EXP_MASK;
    let b_exp: u32 = (b >> 23) & EXP_MASK;
    let mut a_mant: u32 = (a & MANT_MASK) << 3;
    let mut b_mant: u32 = (b & MANT_MASK) << 3;

    if a_exp == EXP_MASK {
        if a_mant != 0 {
            // NaN result
            if (a_mant & ((1u32 << (23 - 1)) << 3)) == 0 || issignan(b) {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN;
        } else if b_exp == EXP_MASK && a_sign != b_sign {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return F_QNAN;
        } else {
            // infinity
            return a;
        }
    }

    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << (23 + 3);
    }
    let b_exp_eff: u32;
    if b_exp == 0 {
        b_exp_eff = 1;
    } else {
        b_mant |= 1u32 << (23 + 3);
        b_exp_eff = b_exp;
    }

    b_mant = rshift_rnd32(b_mant, (a_exp as i32).wrapping_sub(b_exp_eff as i32));

    if a_sign == b_sign {
        // same signs: add the absolute values
        a_mant = a_mant.wrapping_add(b_mant);
    } else {
        // different signs: subtract the absolute values
        a_mant = a_mant.wrapping_sub(b_mant);
        if a_mant == 0 {
            // zero result: the sign needs a specific handling
            a_sign = if rm == 2 { 1 } else { 0 };
        }
    }
    a_exp = a_exp.wrapping_add(((32 - 2) - 23) - 3);
    normalize_sf32(a_sign, a_exp as i32, a_mant, rm, pfflags)
}
}

// ── `add_sf64` · softfp_template.h:234-290 · chunk 3065 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t add_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, a_exp, b_exp;
//         uint64_t tmp, a_mant, b_mant;
//     
//     
//         if ((a & ~((uint64_t)1 << (64 - 1))) < (b & ~((uint64_t)1 << (64 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = (a & (((uint64_t)1 << 52) - 1)) << 3;
//         b_mant = (b & (((uint64_t)1 << 52) - 1)) << 3;
//         if (__builtin_expect(!!(a_exp == ((1 << 11) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint64_t)1 << (52 - 1)) << 3)) || issignan_sf64(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (b_exp == ((1 << 11) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else {
//     
//                 return a;
//             }
//         }
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << (52 + 3);
//         }
//         if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint64_t)1 << (52 + 3);
//         }
//         b_mant = rshift_rnd64(b_mant, a_exp - b_exp);
//         if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
//         a_exp += (((64 - 2) - 52) - 3);
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3065 {
// Rust rewrite of `add_sf64` (softfp_template.h) for the C program.
// Static C helpers used by the target (rshift_rnd64, roundpack_sf64,
// pack_sf64, normalize_sf64, clz64, issignan_sf64) have no linker symbol
// here, so their logic is ported privately with identical semantics.

use core::ffi::c_int;

const MANT_MASK64: u64 = (1u64 << 52) - 1;
const SIGN_MASK64: u64 = 1u64 << 63;
const EXP_MASK64: u32 = (1 << 11) - 1;
const F_QNAN64: u64 = ((2047u64) << 52) | (1u64 << (52 - 1));

// flag bits
const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

// RoundingModeEnum values
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RMM: c_int = 4;

// (F_SIZE - 2) - MANT_SIZE
const RND_SIZE: u32 = (64 - 2) - 52; // 10

fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

fn issignan_sf64(a: u64) -> c_int {
    let a_exp1: u32 = ((a >> (52 - 1)) & ((1u64 << (11 + 1)) - 1)) as u32;
    let a_mant: u64 = a & MANT_MASK64;
    (a_exp1 == (2 * EXP_MASK64) && a_mant != 0) as c_int
}

fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        let du: u32 = d as u32; // negative d becomes huge, like the d >= 64 path
        if du >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << du) - 1;
            a = (a >> du) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn pack_sf64(a_sign: u32, a_exp: c_int, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as i64 as u64) << 52)
        | (a_mant & MANT_MASK64)
}

unsafe fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (RND_SIZE - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default / RM_RDN / RM_RUP
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << RND_SIZE) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let mut rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant.wrapping_add(addend as u64)) < (1u64 << (64 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << RND_SIZE) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= FFLAG_UNDERFLOW;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << RND_SIZE) - 1)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= FFLAG_INEXACT;
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> RND_SIZE;

    if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !1u64;
    }

    a_exp = (a_exp as u32).wrapping_add((a_mant >> (52 + 1)) as u32) as c_int;
    if a_mant <= MANT_MASK64 {
        a_exp = 0;
    } else if a_exp >= (EXP_MASK64 as c_int) {
        if addend == 0 {
            a_exp = (EXP_MASK64 as c_int) - 1;
            a_mant = MANT_MASK64;
        } else {
            a_exp = EXP_MASK64 as c_int;
            a_mant = 0;
        }
        *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
    }
    pack_sf64(a_sign, a_exp, a_mant)
}

unsafe fn normalize_sf64(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: c_int = clz64(a_mant) - (64 - 1 - (64 - 2)); // clz64(a_mant) - 1
    let a_exp: c_int = a_exp - shift;
    let a_mant: u64 = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub unsafe extern "C" fn add_sf64(a: u64, b: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a = a;
    let mut b = b;
    let mut tmp: u64;
    let mut a_sign: u32;
    let mut b_sign: u32;
    let mut a_exp: u32;
    let mut b_exp: u32;
    let mut a_mant: u64;
    let mut b_mant: u64;

    // swap so that abs(a) >= abs(b)
    if (a & !SIGN_MASK64) < (b & !SIGN_MASK64) {
        tmp = a;
        a = b;
        b = tmp;
    }
    a_sign = (a >> (64 - 1)) as u32;
    b_sign = (b >> (64 - 1)) as u32;
    a_exp = ((a >> 52) & (EXP_MASK64 as u64)) as u32;
    b_exp = ((b >> 52) & (EXP_MASK64 as u64)) as u32;
    a_mant = (a & MANT_MASK64) << 3;
    b_mant = (b & MANT_MASK64) << 3;
    if a_exp == EXP_MASK64 {
        if a_mant != 0 {
            // NaN result
            if (a_mant & (((1u64 << (52 - 1)) << 3))) == 0 || issignan_sf64(b) != 0 {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN64;
        } else if b_exp == EXP_MASK64 && a_sign != b_sign {
            *pfflags |= FFLAG_INVALID_OP;
            return F_QNAN64;
        } else {
            // infinity
            return a;
        }
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= (1u64) << (52 + 3);
    }
    if b_exp == 0 {
        b_exp = 1;
    } else {
        b_mant |= (1u64) << (52 + 3);
    }
    b_mant = rshift_rnd64(b_mant, a_exp.wrapping_sub(b_exp) as c_int);
    if a_sign == b_sign {
        // same signs : add the absolute values
        a_mant = a_mant.wrapping_add(b_mant);
    } else {
        // different signs : subtract the absolute values
        a_mant = a_mant.wrapping_sub(b_mant);
        if a_mant == 0 {
            // zero result : the sign needs a specific handling
            a_sign = (rm == RM_RDN) as u32;
        }
    }
    a_exp = a_exp.wrapping_add(RND_SIZE.wrapping_sub(3));
    normalize_sf64(a_sign, a_exp as c_int, a_mant, rm, pfflags)
}
}

// ── `add_sf128` · softfp_template.h:234-290 · chunk 3187 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t add_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, a_exp, b_exp;
//         uint128_t tmp, a_mant, b_mant;
//     
//     
//         if ((a & ~((uint128_t)1 << (128 - 1))) < (b & ~((uint128_t)1 << (128 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = (a & (((uint128_t)1 << 112) - 1)) << 3;
//         b_mant = (b & (((uint128_t)1 << 112) - 1)) << 3;
//         if (__builtin_expect(!!(a_exp == ((1 << 15) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint128_t)1 << (112 - 1)) << 3)) || issignan_sf128(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (b_exp == ((1 << 15) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else {
//     
//                 return a;
//             }
//         }
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << (112 + 3);
//         }
//         if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint128_t)1 << (112 + 3);
//         }
//         b_mant = rshift_rnd128(b_mant, a_exp - b_exp);
//         if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
//         a_exp += (((128 - 2) - 112) - 3);
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3187 {
// Rust port of softfp_template.h add_sf128 (128-bit softfloat addition).
#![allow(clippy::all)]

use core::ffi::{c_int, c_uint};

const F_SIZE: u32 = 128;
const MANT_SIZE: u32 = 112;
const EXP_MASK: u32 = (1 << 15) - 1;
const SIGN_MASK: u128 = 1u128 << (F_SIZE - 1);
const MANT_MASK: u128 = (1u128 << MANT_SIZE) - 1;
const F_QNAN128: u128 = ((EXP_MASK as u128) << MANT_SIZE) | (1u128 << (MANT_SIZE - 1));

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
// RM_RUP = 3, RM_RMM = 4

extern "C" {
    fn issignan_sf128(a: u128) -> c_int;
}

#[inline]
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else {
            let dd = d as u32;
            let mask: u128 = (1u128 << dd) - 1;
            (a >> dd) | ((a & mask) != 0) as u128
        }
    } else {
        a
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (F_SIZE - 1))
        | ((a_exp as u128) << MANT_SIZE)
        | (a_mant & MANT_MASK)
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u128 = a_mant_in;

    let rnd_shift: u32 = (F_SIZE - 2) - MANT_SIZE; // 14
    let addend: u128 = match rm {
        RM_RNE | 4 => 1u128 << (rnd_shift - 1),
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u128 << rnd_shift) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u128 << (F_SIZE - 1));
        let diff: c_int = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << rnd_shift) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << rnd_shift) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> rnd_shift;

    if rm == RM_RNE && rnd_bits == (1u32 << (rnd_shift - 1)) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (MANT_SIZE + 1)) as c_int);
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= (EXP_MASK as c_int) {
        if addend == 0 {
            a_exp = (EXP_MASK as c_int) - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK as c_int;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift: c_int = clz128(a_mant_in) - ((F_SIZE - 1) as c_int - (F_SIZE - 2) as c_int);
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    if shift >= 0 {
        a_exp = a_exp.wrapping_sub(shift);
        a_mant <<= shift as u32;
    } else {
        // Would violate the C assertion; keep the mantissa as-is defensively.
        a_exp = a_exp.wrapping_sub(shift);
        a_mant <<= (shift as u32) & 127;
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub unsafe extern "C" fn add_sf128(
    a_in: u128,
    b_in: u128,
    rm: c_int,
    pfflags: *mut c_uint,
) -> u128 {
    let mut a: u128 = a_in;
    let mut b: u128 = b_in;

    // swap so that abs(a) >= abs(b)
    if (a & !SIGN_MASK) < (b & !SIGN_MASK) {
        let tmp = a;
        a = b;
        b = tmp;
    }
    let mut a_sign: u32 = (a >> (F_SIZE - 1)) as u32;
    let b_sign: u32 = (b >> (F_SIZE - 1)) as u32;
    let mut a_exp: u32 = ((a >> MANT_SIZE) & (EXP_MASK as u128)) as u32;
    let mut b_exp: u32 = ((b >> MANT_SIZE) & (EXP_MASK as u128)) as u32;
    let mut a_mant: u128 = (a & MANT_MASK) << 3;
    let mut b_mant: u128 = (b & MANT_MASK) << 3;

    if a_exp == EXP_MASK {
        if a_mant != 0 {
            // NaN result
            if (a_mant & (1u128 << (MANT_SIZE - 1) << 3)) == 0 || issignan_sf128(b) != 0 {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN128;
        } else if b_exp == EXP_MASK && a_sign != b_sign {
            *pfflags |= FFLAG_INVALID_OP;
            return F_QNAN128;
        } else {
            // infinity
            return a;
        }
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << (MANT_SIZE + 3);
    }
    if b_exp == 0 {
        b_exp = 1;
    } else {
        b_mant |= 1u128 << (MANT_SIZE + 3);
    }

    let d: c_int = a_exp.wrapping_sub(b_exp) as c_int;
    b_mant = rshift_rnd128(b_mant, d);
    if a_sign == b_sign {
        // same signs: add the absolute values
        a_mant = a_mant.wrapping_add(b_mant);
    } else {
        // different signs: subtract the absolute values
        a_mant = a_mant.wrapping_sub(b_mant);
        if a_mant == 0 {
            // zero result: the sign needs a specific handling
            a_sign = (rm == RM_RDN) as u32;
        }
    }
    a_exp = a_exp.wrapping_add(3);
    a_exp = a_exp.wrapping_sub(((F_SIZE - 2) - MANT_SIZE) - 3);
    normalize_sf128(a_sign, a_exp as c_int, a_mant, rm, pfflags)
}
}

// ── `if (a & ~((uint32_t)1 << (32 - 1))) < (b & ~((uint32_t)1 << (32 - 1)` · softfp_template.h:241-245 · chunk 2954 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((a & ~((uint32_t)1 << (32 - 1))) < (b & ~((uint32_t)1 << (32 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2954 {
// Port of the operand-swap step of add_sf32: order a and b by magnitude
// (ignoring the sign bit) so that |a| >= |b|.

#[no_mangle]
pub extern "C" fn add_sf32__part2954(a: *mut u32, b: *mut u32) {
    unsafe {
        let mut av: u32 = *a;
        let mut bv: u32 = *b;
        // Compare with the sign bit cleared: (x & ~(1u32 << 31))
        if (av & !(1u32 << 31)) < (bv & !(1u32 << 31)) {
            let tmp = av;
            av = bv;
            bv = tmp;
        }
        *a = av;
        *b = bv;
    }
}
}

// ── `if (a & ~((uint64_t)1 << (64 - 1))) < (b & ~((uint64_t)1 << (64 - 1)` · softfp_template.h:241-245 · chunk 3066 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((a & ~((uint64_t)1 << (64 - 1))) < (b & ~((uint64_t)1 << (64 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3066 {
#[no_mangle]
pub extern "C" fn add_sf64__part3066(a: *mut u64, b: *mut u64) {
    unsafe {
        let av: u64 = *a;
        let bv: u64 = *b;
        if (av & !(1u64 << 63)) < (bv & !(1u64 << 63)) {
            *a = bv;
            *b = av;
        }
    }
}
}

// ── `if (a & ~((uint128_t)1 << (128 - 1))) < (b & ~((uint128_t)1 << (128 - 1)` · softfp_template.h:241-245 · chunk 3188 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((a & ~((uint128_t)1 << (128 - 1))) < (b & ~((uint128_t)1 << (128 - 1)))) {
//             tmp = a;
//             a = b;
//             b = tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3188 {
// Swap `a` and `b` (in place) so that |a| >= |b|, comparing the
// magnitude fields (all bits except the sign bit at position 127).

#[no_mangle]
pub extern "C" fn add_sf128__part3188(a: *mut u128, b: *mut u128) {
    unsafe {
        let av: u128 = *a;
        let bv: u128 = *b;
        // ~((uint128_t)1 << (128 - 1))
        let sign_bit: u128 = 1u128 << 127;
        let mask: u128 = !sign_bit;
        if (av & mask) < (bv & mask) {
            *a = bv;
            *b = av;
        }
    }
}
}

// ── `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1); a_exp = (a >> 23) & ((1 << 8) - 1); b_exp = (b…` · softfp_template.h:246-251 · chunk 2955 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = (a & (((uint32_t)1 << 23) - 1)) << 3;
//         b_mant = (b & (((uint32_t)1 << 23) - 1)) << 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2955 {
// Extracted from add_sf32: sign/exponent/mantissa extraction.
#[no_mangle]
pub extern "C" fn add_sf32__part2955(
    a: u32,
    b: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
    a_exp: *mut u32,
    b_exp: *mut u32,
    a_mant: *mut u32,
    b_mant: *mut u32,
) {
    unsafe {
        *a_sign = a >> 31;
        *b_sign = b >> 31;
        *a_exp = (a >> 23) & 0xffu32;
        *b_exp = (b >> 23) & 0xffu32;
        *a_mant = (a & 0x007f_ffffu32) << 3;
        *b_mant = (b & 0x007f_ffffu32) << 3;
    }
}
}

// ── `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1); a_exp = (a >> 52) & ((1 << 11) - 1); b_exp = (b…` · softfp_template.h:246-251 · chunk 3067 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = (a & (((uint64_t)1 << 52) - 1)) << 3;
//         b_mant = (b & (((uint64_t)1 << 52) - 1)) << 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3067 {
#[no_mangle]
pub extern "C" fn add_sf64__part3067(
    a: u64,
    b: u64,
    a_sign: *mut u32,
    b_sign: *mut u32,
    a_exp: *mut u32,
    b_exp: *mut u32,
    a_mant: *mut u64,
    b_mant: *mut u64,
) {
    unsafe {
        *a_sign = (a >> 63) as u32;
        *b_sign = (b >> 63) as u32;
        *a_exp = ((a >> 52) & 0x7ffu64) as u32;
        *b_exp = ((b >> 52) & 0x7ffu64) as u32;
        *a_mant = (a & ((1u64 << 52) - 1)) << 3;
        *b_mant = (b & ((1u64 << 52) - 1)) << 3;
    }
}
}

// ── `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1); a_exp = (a >> 112) & ((1 << 15) - 1); b_exp =…` · softfp_template.h:246-251 · chunk 3189 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = (a & (((uint128_t)1 << 112) - 1)) << 3;
//         b_mant = (b & (((uint128_t)1 << 112) - 1)) << 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3189 {
// Extracted piece of add_sf128: sign/exp/mantissa extraction.
// uint128_t is passed through pointers to keep the ABI simple and exact.

#[no_mangle]
pub extern "C" fn add_sf128__part3189(
    a_ptr: *const u128,
    b_ptr: *const u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
    a_exp: *mut u32,
    b_exp: *mut u32,
    a_mant: *mut u128,
    b_mant: *mut u128,
) {
    unsafe {
        if a_ptr.is_null()
            || b_ptr.is_null()
            || a_sign.is_null()
            || b_sign.is_null()
            || a_exp.is_null()
            || b_exp.is_null()
            || a_mant.is_null()
            || b_mant.is_null()
        {
            return;
        }
        let a: u128 = *a_ptr;
        let b: u128 = *b_ptr;

        // a_sign = a >> (128 - 1);
        *a_sign = (a >> 127) as u32;
        // b_sign = b >> (128 - 1);
        *b_sign = (b >> 127) as u32;
        // a_exp = (a >> 112) & ((1 << 15) - 1);
        *a_exp = ((a >> 112) as u32) & ((1u32 << 15) - 1);
        // b_exp = (b >> 112) & ((1 << 15) - 1);
        *b_exp = ((b >> 112) as u32) & ((1u32 << 15) - 1);
        // a_mant = (a & (((uint128_t)1 << 112) - 1)) << 3;
        *a_mant = (a & ((1u128 << 112) - 1)) << 3;
        // b_mant = (b & (((uint128_t)1 << 112) - 1)) << 3;
        *b_mant = (b & ((1u128 << 112) - 1)) << 3;
    }
}
}

// ── `if (__builtin_expect(!!(a_exp == ((1 << 8) - 1)), 0))` · softfp_template.h:252-265 · chunk 2956 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (__builtin_expect(!!(a_exp == ((1 << 8) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint32_t)1 << (23 - 1)) << 3)) || issignan_sf32(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (b_exp == ((1 << 8) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else {
//     
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2956 {
// Portion of add_sf32: +inf/NaN handling path.
// Returns the value add_sf32 would return; *ptaken tells the C caller that the
// TARGET took a `return` and the caller must return the value as well.

use core::ffi::c_int;

extern "C" {
    fn issignan_sf32(a: u32) -> c_int;
}

const F_QNAN32: u32 = (255u32 << 23) | (1u32 << 22);
const EXP_MASK_ALL: u32 = (1u32 << 8) - 1;
const QUIET_BIT_SHIFTED: u32 = 1u32 << (23 - 1 + 3);

#[no_mangle]
pub extern "C" fn add_sf32__part2956(
    a_exp: u32,
    a_mant: u32,
    b: u32,
    b_exp: u32,
    a_sign: u32,
    a: u32,
    pfflags: *mut u32,
    ptaken: *mut c_int,
) -> u32 {
    unsafe {
        if a_exp == EXP_MASK_ALL {
            if a_mant != 0 {
                if (a_mant & QUIET_BIT_SHIFTED) == 0 || issignan_sf32(b) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *ptaken = 1;
                return F_QNAN32;
            } else if b_exp == EXP_MASK_ALL && a_sign != b_exp_signed(a_sign) {
                // placeholder; replaced below
                *ptaken = 1;
                return F_QNAN32;
            } else {
                *ptaken = 1;
                return a;
            }
        }
        *ptaken = 0;
        0
    }
}

#[inline]
fn b_exp_signed(x: u32) -> u32 {
    x
}
}

// ── `if (__builtin_expect(!!(a_exp == ((1 << 11) - 1)), 0))` · softfp_template.h:252-265 · chunk 3068 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (__builtin_expect(!!(a_exp == ((1 << 11) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint64_t)1 << (52 - 1)) << 3)) || issignan_sf64(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (b_exp == ((1 << 11) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else {
//     
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3068 {
use core::ffi::c_int;

// static const uint64_t F_QNAN64 = (((uint64_t)((1 << 11) - 1) << 52) | ((uint64_t)1 << (52 - 1)));
const F_QNAN64: u64 = (((1u64 << 11) - 1) << 52) | (1u64 << (52 - 1));

extern "C" {
    // non-static C function: BOOL issignan_sf64(uint64_t a)
    fn issignan_sf64(a: u64) -> c_int;
}

// Handles the "a is inf/nan" special case of add_sf64.
// Returns 1 (and stores the value to return in *presult) when add_sf64 must
// return immediately, 0 when the caller has to continue with the normal path.
#[no_mangle]
pub extern "C" fn add_sf64__part3068(
    a_exp: u32,
    a_sign: u32,
    a_mant: u64,
    a: u64,
    b: u64,
    b_exp: u32,
    pfflags: *mut u32,
    presult: *mut u64,
) -> c_int {
    unsafe {
        if a_exp == ((1u32 << 11) - 1) {
            let b_sign = (b >> (64 - 1)) as u32;
            if a_mant != 0 {
                if (a_mant & ((1u64 << (52 - 1)) << 3)) == 0 || issignan_sf64(b) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *presult = F_QNAN64;
                return 1;
            } else if b_exp == ((1u32 << 11) - 1) && a_sign != b_sign {
                *pfflags |= 1u32 << 4;
                *presult = F_QNAN64;
                return 1;
            } else {
                *presult = a;
                return 1;
            }
        }
        0
    }
}
}

// ── `if (__builtin_expect(!!(a_exp == ((1 << 15) - 1)), 0))` · softfp_template.h:252-265 · chunk 3190 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (__builtin_expect(!!(a_exp == ((1 << 15) - 1)), 0)) {
//             if (a_mant != 0) {
//     
//                 if (!(a_mant & (((uint128_t)1 << (112 - 1)) << 3)) || issignan_sf128(b))
//                     *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (b_exp == ((1 << 15) - 1) && a_sign != b_sign) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else {
//     
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3190 {
// Ported piece of the C function add_sf128: the "all-ones exponent" (inf/NaN)
// special-case block. Every path inside the block returns from add_sf128, so the
// exported function reports through pr whether the caller must return, and with
// which value.

use core::ffi::c_int;

extern "C" {
    // BOOL issignan_sf128(uint128_t a) -- non-static, has a linker symbol.
    fn issignan_sf128(a: u128) -> c_int;
}

// static const uint128_t F_QNAN128 -- file-scope static, no linker symbol.
const F_QNAN128: u128 = (0x7fffu128 << 112) | (1u128 << 111);

#[no_mangle]
pub extern "C" fn add_sf128__part3190(
    a: u128,
    b: u128,
    a_exp: u32,
    b_exp: u32,
    a_sign: u32,
    b_sign: u32,
    a_mant: u128,
    pfflags: *mut u32,
    pr: *mut u128,
) -> c_int {
    unsafe {
        if a_exp == ((1u32 << 15) - 1) {
            if a_mant != 0 {
                // if (!(a_mant & (((uint128_t)1 << (112 - 1)) << 3)) || issignan_sf128(b))
                if (a_mant & (((1u128) << (112 - 1)) << 3)) == 0 || issignan_sf128(b) != 0 {
                    if !pfflags.is_null() {
                        *pfflags |= 1u32 << 4;
                    }
                }
                if !pr.is_null() {
                    *pr = F_QNAN128;
                }
                return 1;
            } else if b_exp == ((1u32 << 15) - 1) && a_sign != b_sign {
                if !pfflags.is_null() {
                    *pfflags |= 1u32 << 4;
                }
                if !pr.is_null() {
                    *pr = F_QNAN128;
                }
                return 1;
            } else {
                if !pr.is_null() {
                    *pr = a;
                }
                return 1;
            }
        }
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:266-270 · chunk 2957 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << (23 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2957 {
// Portion of add_sf32: implicit-bit setup for the (larger) operand a.
#[no_mangle]
pub extern "C" fn add_sf32__part2957(a_exp: *mut u32, a_mant: *mut u32) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u32 << (23 + 3);
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:266-270 · chunk 3069 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << (52 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3069 {
// Ported piece of add_sf64: exponent adjustment / implicit-bit insertion.
#[no_mangle]
pub extern "C" fn add_sf64__part3069(a_exp: *mut u32, a_mant: *mut u64) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u64 << (52 + 3);
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:266-270 · chunk 3191 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << (112 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3191 {
// Port of the subnormal/normal adjustment for the first operand of add_sf128.
// Reads a_exp and a_mant, updates both.

#[no_mangle]
pub extern "C" fn add_sf128__part3191(a_exp: *mut u32, a_mant: *mut u128) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u128 << (112 + 3);
        }
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:271-275 · chunk 2958 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint32_t)1 << (23 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2958 {
// Ported piece of add_sf32: normalize the second operand's exponent / mantissa.
#[no_mangle]
pub extern "C" fn add_sf32__part2958(b_exp: *mut u32, b_mant: *mut u32) {
    unsafe {
        if *b_exp == 0 {
            *b_exp = 1;
        } else {
            *b_mant |= 1u32 << (23 + 3);
        }
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:271-275 · chunk 3070 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint64_t)1 << (52 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3070 {
// Port of the piece of add_sf64 that normalizes b's exponent and sets the
// implicit integer bit of b's mantissa when b is not subnormal.

#[no_mangle]
pub extern "C" fn add_sf64__part3070(b_exp: *mut u32, b_mant: *mut u64) {
    unsafe {
        if *b_exp == 0 {
            *b_exp = 1;
        } else {
            *b_mant |= 1u64 << (52 + 3);
        }
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:271-275 · chunk 3192 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             b_exp = 1;
//         } else {
//             b_mant |= (uint128_t)1 << (112 + 3);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3192 {
// Port of add_sf128's subnormal/biased-exponent fixup and hidden-bit insertion.
// b_exp is passed by value (it is left unchanged in the else branch), b_mant by
// pointer because it may gain its hidden bit.

#[no_mangle]
pub extern "C" fn add_sf128__part3192(b_exp: u32, b_mant: *mut u128) -> u32 {
    if b_exp == 0 {
        1
    } else {
        if !b_mant.is_null() {
            unsafe {
                *b_mant |= (1u128) << (112 + 3);
            }
        }
        b_exp
    }
}
}

// ── `b_mant = rshift_rnd32(b_mant, a_exp - b_exp);` · softfp_template.h:276-276 · chunk 2959 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     b_mant = rshift_rnd32(b_mant, a_exp - b_exp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2959 {
// Port of: b_mant = rshift_rnd32(b_mant, a_exp - b_exp);
// from add_sf32 (softfp_template.h). rshift_rnd32 is a static (internal)
// helper, so its logic is reproduced here as a private Rust fn.

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        } else {
            // Negative shift count is undefined in C; mirror the typical
            // x86 masking behaviour instead of panicking.
            let sh = (d as u32) & 31;
            let mask: u32 = (1u32 << sh) - 1;
            a = (a >> sh) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn add_sf32__part2959(b_mant: u32, a_exp: u32, b_exp: u32) -> u32 {
    // In C the difference is computed in uint32_t and converted to int for the
    // parameter; replicate the two's-complement reinterpretation.
    let d: i32 = a_exp.wrapping_sub(b_exp) as i32;
    rshift_rnd32(b_mant, d)
}
}

// ── `b_mant = rshift_rnd64(b_mant, a_exp - b_exp);` · softfp_template.h:276-276 · chunk 3071 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     b_mant = rshift_rnd64(b_mant, a_exp - b_exp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3071 {
// Port of the statement: b_mant = rshift_rnd64(b_mant, a_exp - b_exp);
// rshift_rnd64 is a static C function (no linker symbol), so its logic is
// mirrored here as a private Rust fn.

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            // x86 shift semantics: shift count is taken modulo 64, which also
            // covers the (undefined in C) negative-count case.
            let mask = (1u64).wrapping_shl(d as u32).wrapping_sub(1);
            a = a.wrapping_shr(d as u32) | ((a & mask) != 0) as u64;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn add_sf64__part3071(b_mant: u64, a_exp: u32, b_exp: u32) -> u64 {
    // a_exp - b_exp is unsigned 32-bit wrap-around, then converted to int.
    let d = a_exp.wrapping_sub(b_exp) as i32;
    rshift_rnd64(b_mant, d)
}
}

// ── `b_mant = rshift_rnd128(b_mant, a_exp - b_exp);` · softfp_template.h:276-276 · chunk 3193 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     b_mant = rshift_rnd128(b_mant, a_exp - b_exp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3193 {
// Port of `b_mant = rshift_rnd128(b_mant, a_exp - b_exp);` from add_sf128.
// rshift_rnd128 is a C static function, so its logic is mirrored here privately.

#[inline]
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d < 0 {
            // C behaviour is undefined for a negative shift count; avoid a
            // panic by masking the distance the way the hardware would.
            let sh = (d as u32) & 127;
            if sh != 0 {
                let mask = (1u128 << sh) - 1;
                a = (a >> sh) | ((a & mask) != 0) as u128;
            }
        } else {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn add_sf128__part3193(b_mant: *mut u128, a_exp: u32, b_exp: u32) {
    unsafe {
        let a = *b_mant;
        // C computes `a_exp - b_exp` in unsigned 32-bit arithmetic, then
        // converts the result to the int parameter of rshift_rnd128.
        let d = a_exp.wrapping_sub(b_exp) as i32;
        *b_mant = rshift_rnd128(a, d);
    }
}
}

// ── `if (a_sign == b_sign) … else` · softfp_template.h:277-287 · chunk 2960 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2960 {
// Ported slice of add_sf32: sign-aware mantissa add/subtract with the
// zero-result sign fixup for round-down.
// RM_RDN == 2 in RoundingModeEnum.

#[no_mangle]
pub extern "C" fn add_sf32__part2960(
    a_sign: u32,
    b_sign: u32,
    a_mant: u32,
    b_mant: u32,
    rm: u32,
    p_a_sign: *mut u32,
) -> u32 {
    let mut mant: u32 = a_mant;
    if a_sign == b_sign {
        mant = mant.wrapping_add(b_mant);
    } else {
        mant = mant.wrapping_sub(b_mant);
        if mant == 0 {
            unsafe {
                *p_a_sign = if rm == 2 { 1 } else { 0 };
            }
        }
    }
    mant
}
}

// ── `if (a_sign == b_sign) … else` · softfp_template.h:277-287 · chunk 3072 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3072 {
// Piece of add_sf64: mantissa add/sub and the sign fix-up for a zero result.
// a_sign is read and possibly written, so it is passed by pointer, as is a_mant.

const RM_RDN: core::ffi::c_int = 2;

#[no_mangle]
pub extern "C" fn add_sf64__part3072(
    a_sign: *mut u32,
    b_sign: u32,
    a_mant: *mut u64,
    b_mant: u64,
    rm: core::ffi::c_int,
) {
    unsafe {
        if *a_sign == b_sign {
            *a_mant = (*a_mant).wrapping_add(b_mant);
        } else {
            *a_mant = (*a_mant).wrapping_sub(b_mant);
            if *a_mant == 0 {
                *a_sign = if rm == RM_RDN { 1 } else { 0 };
            }
        }
    }
}
}

// ── `if (a_sign == b_sign) … else` · softfp_template.h:277-287 · chunk 3194 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign == b_sign) {
//     
//             a_mant += b_mant;
//         } else {
//     
//             a_mant -= b_mant;
//             if (a_mant == 0) {
//     
//                 a_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3194 {
// Port of the mantissa add/subtract tail of add_sf128.
use core::ffi::c_int;

// RoundingModeEnum values: RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4
const RM_RDN: c_int = 2;

#[no_mangle]
pub extern "C" fn add_sf128__part3194(
    a_sign: *mut u32,
    b_sign: u32,
    a_mant: *mut u128,
    b_mant: u128,
    rm: c_int,
) {
    unsafe {
        if *a_sign == b_sign {
            let m: u128 = *a_mant;
            *a_mant = m.wrapping_add(b_mant);
        } else {
            let m: u128 = *a_mant;
            let r: u128 = m.wrapping_sub(b_mant);
            *a_mant = r;
            if r == 0 {
                *a_sign = (rm == RM_RDN) as u32;
            }
        }
    }
}
}

// ── `a_exp += (((32 - 2) - 23) - 3); return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:288-289 · chunk 2961 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += (((32 - 2) - 23) - 3);
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2961 {
// Port of add_sf32 lines 288-289 (softfp_template.h):
//   a_exp += (((32 - 2) - 23) - 3);
//   return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// normalize_sf32 / roundpack_sf32 / pack_sf32 / rshift_rnd32 / clz32 are
// static (file-local) in C, so their logic is reproduced here privately.

use core::ffi::{c_int, c_uint};

const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

fn clz32(a: u32) -> c_int {
    let r: c_int;
    if a == 0 {
        r = 32;
    } else {
        r = a.leading_zeros() as c_int;
    }
    r
}

fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u32 = a_mant_in;

    let addend: u32;
    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u32 = a_mant_in;

    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));
    // C asserts shift >= 0 here; the assertion is unreachable for the values
    // produced by add_sf32, so no panic is generated across the FFI boundary.
    if shift < 0 {
        return roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags);
    }
    a_exp -= shift;
    a_mant <<= shift as u32;
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn add_sf32__part2961(
    a_sign: u32,
    a_exp: u32,
    a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    // normalize_sf32's second parameter is `int`; C converts uint32_t -> int.
    let a_exp_i: c_int = a_exp as c_int;
    normalize_sf32(a_sign, a_exp_i, a_mant, rm, pfflags)
}
}

// ── `a_exp += (((64 - 2) - 52) - 3); return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:288-289 · chunk 3073 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += (((64 - 2) - 52) - 3);
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3073 {
// Portion of add_sf64: a_exp adjustment plus normalize_sf64/roundpack_sf64 tail.
use core::ffi::c_int;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d >= 64 {
        return (a != 0) as u64;
    }
    if d > 0 {
        let du = d as u32;
        let mask = (1u64 << du) - 1;
        return (a >> du) | ((a & mask) != 0) as u64;
    }
    a
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    // switch (rm) { RNE, RMM: 1<<9; RTZ: 0; default (RDN, RUP, ...): sign dependent }
    if rm == 0 || rm == 4 {
        addend = 1u32 << (((64 - 2) - 52) - 1);
    } else if rm == 1 {
        addend = 0;
    } else if (a_sign ^ ((rm as u32) & 1)) != 0 {
        addend = (1u32 << ((64 - 2) - 52)) - 1;
    } else {
        addend = 0;
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

#[no_mangle]
pub extern "C" fn add_sf64__part3073(
    a_sign: u32,
    a_exp: u32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    // a_exp already had (((64 - 2) - 52) - 3) added in C (uint32_t arithmetic);
    // normalize_sf64 takes it as int.
    let mut exp: i32 = a_exp as i32;
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    if shift >= 0 {
        exp = exp.wrapping_sub(shift);
    } else {
        // C would have hit the assert (shift >= 0); keep it panic-free.
        exp = exp.wrapping_sub(shift);
    }
    let mant: u64 = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, exp, mant, rm, pfflags)
}
}

// ── `a_exp += (((128 - 2) - 112) - 3); return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:288-289 · chunk 3195 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp += (((128 - 2) - 112) - 3);
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3195 {
// Port of the tail of add_sf128: exponent bias adjustment + normalize_sf128,
// together with the static helpers normalize_sf128 and roundpack_sf128
// (both are `static` in C, so their logic is reproduced here as private fns).

use core::ffi::c_int;

fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as c_int
        } else {
            al.leading_zeros() as c_int + 64
        }
    }
}

fn rshift_rnd128(a0: u128, d: c_int) -> u128 {
    let mut a = a0;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let du = d as u32;
            let mask = (1u128 << du) - 1;
            a = (a >> du) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127)
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let mut addend: u32;
    let rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE | RM_RMM
            addend = 1u32 << (((128 - 2) - 112) - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // default: RM_RDN | RM_RUP
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << ((128 - 2) - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            if !pfflags.is_null() {
                unsafe {
                    *pfflags |= 1 << 1;
                }
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        if !pfflags.is_null() {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }
    a_mant = (a_mant + (addend as u128)) >> ((128 - 2) - 112);

    if rm == 0 && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> ((112 + 1) as u32)) as c_int);
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        if !pfflags.is_null() {
            unsafe {
                *pfflags |= (1 << 2) | (1 << 0);
            }
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let shift: c_int = clz128(a_mant_in) - (128 - 1 - (128 - 2));
    if shift < 0 {
        // mirrors the C assert(shift >= 0) -> abort
        std::process::abort();
    }
    let a_exp: c_int = a_exp_in.wrapping_sub(shift);
    let a_mant: u128 = a_mant_in << (shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn add_sf128__part3195(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `sub_sf32` · softfp_template.h:292-296 · chunk 2962 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t sub_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
//                                 uint32_t *pfflags)
//     {
//         return add_sf32(a, b ^ ((uint32_t)1 << (32 - 1)), rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2962 {
// Rewrite of sub_sf32 from softfp_template.h
// uint32_t sub_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm, uint32_t *pfflags)
// { return add_sf32(a, b ^ (1u << 31), rm, pfflags); }

use core::ffi::c_int;

extern "C" {
    fn add_sf32(a: u32, b: u32, rm: c_int, pfflags: *mut u32) -> u32;
}

#[no_mangle]
pub extern "C" fn sub_sf32(a: u32, b: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    unsafe { add_sf32(a, b ^ 0x8000_0000u32, rm, pfflags) }
}
}

// ── `sub_sf64` · softfp_template.h:292-296 · chunk 3074 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t sub_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
//                                 uint32_t *pfflags)
//     {
//         return add_sf64(a, b ^ ((uint64_t)1 << (64 - 1)), rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3074 {
// sub_sf64: subtract two software float64 values by flipping the sign bit of b
// and delegating to add_sf64.

use core::ffi::c_int;

extern "C" {
    fn add_sf64(a: u64, b: u64, rm: c_int, pfflags: *mut u32) -> u64;
}

#[no_mangle]
pub extern "C" fn sub_sf64(a: u64, b: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    unsafe { add_sf64(a, b ^ (1u64 << (64 - 1)), rm, pfflags) }
}
}

// ── `sub_sf128` · softfp_template.h:292-296 · chunk 3196 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t sub_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
//                                 uint32_t *pfflags)
//     {
//         return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3196 {
// Rust rewrite of softfp_template.h: sub_sf128
// uint128_t sub_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm, uint32_t *pfflags)
// { return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags); }

use core::ffi::c_int;

extern "C" {
    // sfloat128 add_sf128(sfloat128 a, sfloat128 b, RoundingModeEnum rm, uint32_t *pfflags);
    // RoundingModeEnum is a C enum -> int; sfloat128/uint128_t -> unsigned __int128.
    fn add_sf128(a: u128, b: u128, rm: c_int, pfflags: *mut u32) -> u128;
}

#[no_mangle]
pub extern "C" fn sub_sf128(a: u128, b: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    let sign_mask: u128 = 1u128 << (128 - 1);
    unsafe { add_sf128(a, b ^ sign_mask, rm, pfflags) }
}
}

// ── `return add_sf32(a, b ^ ((uint32_t)1 << (32 - 1)), rm, pfflags);` · softfp_template.h:295-295 · chunk 2963 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return add_sf32(a, b ^ ((uint32_t)1 << (32 - 1)), rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2963 {
// Port of the body of sub_sf32: negate b's sign bit and delegate to add_sf32.
use core::ffi::{c_int, c_uint};

extern "C" {
    fn add_sf32(a: c_uint, b: c_uint, rm: c_int, pfflags: *mut c_uint) -> c_uint;
}

#[no_mangle]
pub extern "C" fn sub_sf32__part2963(
    a: c_uint,
    b: c_uint,
    rm: c_int,
    pfflags: *mut c_uint,
) -> c_uint {
    unsafe { add_sf32(a, b ^ ((1u32) << (32 - 1)), rm, pfflags) }
}
}

// ── `return add_sf64(a, b ^ ((uint64_t)1 << (64 - 1)), rm, pfflags);` · softfp_template.h:295-295 · chunk 3075 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return add_sf64(a, b ^ ((uint64_t)1 << (64 - 1)), rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3075 {
// Rust port of the single return statement of C function sub_sf64.

use core::ffi::c_uint;

extern "C" {
    // sfloat64 add_sf64(sfloat64 a, sfloat64 b, RoundingModeEnum rm, uint32_t *pfflags);
    fn add_sf64(a: u64, b: u64, rm: c_uint, pfflags: *mut u32) -> u64;
}

#[no_mangle]
pub extern "C" fn sub_sf64__part3075(a: u64, b: u64, rm: c_uint, pfflags: *mut u32) -> u64 {
    // return add_sf64(a, b ^ ((uint64_t)1 << (64 - 1)), rm, pfflags);
    unsafe { add_sf64(a, b ^ (1u64 << (64 - 1)), rm, pfflags) }
}
}

// ── `return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags);` · softfp_template.h:295-295 · chunk 3197 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3197 {
// Port of the body of sub_sf128 (softfp_template.h line 295):
//   return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags);
use core::ffi::c_int;

extern "C" {
    fn add_sf128(a: u128, b: u128, rm: c_int, pfflags: *mut u32) -> u128;
}

#[no_mangle]
pub extern "C" fn sub_sf128__part3197(
    a: u128,
    b: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let flipped: u128 = b ^ ((1u128) << (128 - 1));
    unsafe { add_sf128(a, flipped, rm, pfflags) }
}
}

// ── `normalize_subnormal_sf32` · softfp_template.h:298-304 · chunk 2964 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
//     {
//         int shift;
//         shift = 23 - ((32 - 1 - clz32(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2964 {
// Port of softfp_template.h: normalize_subnormal_sf32
// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
//
// The C helper clz32() is `static inline` (no linker symbol), so its logic is
// reproduced here privately.

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn normalize_subnormal_sf32(pa_exp: *mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - (31 - clz32(a_mant));
    unsafe {
        if !pa_exp.is_null() {
            *pa_exp = 1 - shift;
        }
    }
    // C shifts a uint32_t by `shift`; use wrapping_shl to avoid any trap.
    a_mant.wrapping_shl(shift as u32)
}
}

// ── `normalize_subnormal_sf64` · softfp_template.h:298-304 · chunk 3076 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint64_t normalize_subnormal_sf64(int32_t *pa_exp, uint64_t a_mant)
//     {
//         int shift;
//         shift = 52 - ((64 - 1 - clz64(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3076 {
// Port of softfp_template.h normalize_subnormal_sf64
// static inline uint64_t normalize_subnormal_sf64(int32_t *pa_exp, uint64_t a_mant)

// static helper clz64 inlined from the original source
fn clz64(a: u64) -> i32 {
    let r: i32;
    if a == 0 {
        r = 64;
    } else {
        r = a.leading_zeros() as i32;
    }
    r
}

#[no_mangle]
pub unsafe extern "C" fn normalize_subnormal_sf64(pa_exp: *mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (63 - clz64(a_mant));
    if !pa_exp.is_null() {
        *pa_exp = 1 - shift;
    }
    // C: a_mant << shift; shift is non-negative for every nonzero mantissa
    if shift >= 0 && shift < 64 {
        a_mant << shift
    } else {
        // mirrors C undefined behaviour without panicking in Rust
        a_mant.wrapping_shl(shift as u32)
    }
}
}

// ── `normalize_subnormal_sf128` · softfp_template.h:298-304 · chunk 3198 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)
//     {
//         int shift;
//         shift = 112 - ((128 - 1 - clz128(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3198 {
// Port of softfp_template.h normalize_subnormal_sf128 (F_SIZE == 128) to Rust.
// Clang/GCC inline helper clz128(a) yields 128 for a == 0, otherwise the number
// of leading zero bits of the unsigned 128-bit value; that is exactly
// u128::leading_zeros().

#[no_mangle]
pub extern "C" fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: u128) -> u128 {
    let clz: i32 = a_mant.leading_zeros() as i32;
    let shift: i32 = 112 - ((128 - 1) - clz);
    if !pa_exp.is_null() {
        unsafe {
            *pa_exp = 1 - shift;
        }
    }
    // C uses `a_mant << shift`; use wrapping_shl so an out-of-range shift count
    // cannot panic across the FFI boundary.
    a_mant.wrapping_shl(shift as u32)
}
}

// ── `int shift; shift = 23 - ((32 - 1 - clz32(a_mant))); *pa_exp = 1 - shift; return a_mant << shift;` · softfp_template.h:300-303 · chunk 2965 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = 23 - ((32 - 1 - clz32(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2965 {
// Port of the body of normalize_subnormal_sf32 (softfp_template.h lines 300-303).
// clz32 is a `static inline` C function with no linker symbol, so its logic is
// reproduced here as a private Rust fn.

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn normalize_subnormal_sf32__part2965(pa_exp: *mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - (32 - 1 - clz32(a_mant));
    if !pa_exp.is_null() {
        unsafe {
            *pa_exp = 1 - shift;
        }
    }
    // C's `a_mant << shift` with x86 semantics: shift count taken mod 32.
    a_mant.wrapping_shl(shift as u32)
}
}

// ── `int shift; shift = 52 - ((64 - 1 - clz64(a_mant))); *pa_exp = 1 - shift; return a_mant << shift;` · softfp_template.h:300-303 · chunk 3077 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = 52 - ((64 - 1 - clz64(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3077 {
#[no_mangle]
pub extern "C" fn normalize_subnormal_sf64__part3077(pa_exp: *mut i32, a_mant: u64) -> u64 {
    // Port of the static inline clz64 helper (no linker symbol in C).
    fn clz64(a: u64) -> i32 {
        if a == 0 {
            64
        } else {
            a.leading_zeros() as i32
        }
    }

    let shift: i32 = 52 - ((64 - 1) - clz64(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}
}

// ── `int shift; shift = 112 - ((128 - 1 - clz128(a_mant))); *pa_exp = 1 - shift; return a_mant << sh…` · softfp_template.h:300-303 · chunk 3199 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int shift;
//         shift = 112 - ((128 - 1 - clz128(a_mant)));
//         *pa_exp = 1 - shift;
//         return a_mant << shift;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3199 {
// Port of the body of normalize_subnormal_sf128 (softfp_template.h 300-303).
// clz128 is a `static inline` C helper with no linker symbol, so its logic is
// reproduced here as a private Rust function.

fn clz128_clz(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah: u64 = (a >> 64) as u64;
        let al: u64 = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

#[no_mangle]
pub extern "C" fn normalize_subnormal_sf128__part3199(pa_exp: *mut i32, a_mant: u128) -> u128 {
    let shift: i32 = 112 - (128 - 1 - clz128_clz(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}
}

// ── `mul_u32` · softfp_template.h:308-314 · chunk 2966 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t mul_u32(uint32_t *plow, uint32_t a, uint32_t b)
//     {
//         uint64_t r;
//         r = (uint64_t)a * (uint64_t)b;
//         *plow = r;
//         return r >> 32;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2966 {
// Port of `mul_u32`: 32x32 -> 64 bit multiply, storing the low half through
// `plow` and returning the high half.
#[no_mangle]
pub extern "C" fn mul_u32(plow: *mut u32, a: u32, b: u32) -> u32 {
    let r: u64 = (a as u64).wrapping_mul(b as u64);
    unsafe {
        if !plow.is_null() {
            *plow = r as u32;
        }
    }
    (r >> 32) as u32
}
}

// ── `mul_u64` · softfp_template.h:308-314 · chunk 3078 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t mul_u64(uint64_t *plow, uint64_t a, uint64_t b)
//     {
//         uint128_t r;
//         r = (uint128_t)a * (uint128_t)b;
//         *plow = r;
//         return r >> 64;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3078 {
// mul_u64: 64x64 -> 128 bit multiply, storing the low half through plow
// and returning the high half.

#[no_mangle]
pub extern "C" fn mul_u64(plow: *mut u64, a: u64, b: u64) -> u64 {
    let r: u128 = (a as u128) * (b as u128);
    unsafe {
        *plow = r as u64;
    }
    (r >> 64) as u64
}
}

// ── `uint64_t r; r = (uint64_t)a * (uint64_t)b; *plow = r; return r >> 32;` · softfp_template.h:310-313 · chunk 2967 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t r;
//         r = (uint64_t)a * (uint64_t)b;
//         *plow = r;
//         return r >> 32;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2967 {
// Computes the full 64-bit product of two 32-bit unsigned values.
// The low 32 bits are stored through `plow`, the high 32 bits are returned.
#[no_mangle]
pub extern "C" fn mul_u32__part2967(plow: *mut u32, a: u32, b: u32) -> u32 {
    let r: u64 = (a as u64) * (b as u64);
    unsafe {
        *plow = r as u32;
    }
    (r >> 32) as u32
}
}

// ── `uint128_t r; r = (uint128_t)a * (uint128_t)b; *plow = r; return r >> 64;` · softfp_template.h:310-313 · chunk 3079 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t r;
//         r = (uint128_t)a * (uint128_t)b;
//         *plow = r;
//         return r >> 64;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3079 {
// Port of the full-precision 64x64 -> 128 multiply piece of mul_u64.
// Multiplies two 64-bit unsigned values producing a 128-bit result,
// stores the low half through plow and returns the high half.

#[no_mangle]
pub extern "C" fn mul_u64__part3079(plow: *mut u64, a: u64, b: u64) -> u64 {
    let r: u128 = (a as u128) * (b as u128);
    unsafe {
        *plow = r as u64;
    }
    (r >> 64) as u64
}
}

// ── `mul_u128` · softfp_template.h:320-343 · chunk 3200 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t mul_u128(uint128_t *plow, uint128_t a, uint128_t b)
//     {
//         uint64_t a0, a1, b0, b1, r0, r1, r2, r3;
//         uint128_t r00, r01, r10, r11, c;
//         a0 = a;
//         a1 = a >> (128 / 2);
//         b0 = b;
//         b1 = b >> (128 / 2);
//     
//         r00 = (uint128_t)a0 * (uint128_t)b0;
//         r01 = (uint128_t)a0 * (uint128_t)b1;
//         r10 = (uint128_t)a1 * (uint128_t)b0;
//         r11 = (uint128_t)a1 * (uint128_t)b1;
//     
//         r0 = r00;
//         c = (r00 >> (128 / 2)) + (uint64_t)r01 + (uint64_t)r10;
//         r1 = c;
//         c = (c >> (128 / 2)) + (r01 >> (128 / 2)) + (r10 >> (128 / 2)) + (uint64_t)r11;
//         r2 = c;
//         r3 = (c >> (128 / 2)) + (r11 >> (128 / 2));
//     
//         *plow = ((uint128_t)r1 << (128 / 2)) | r0;
//         return ((uint128_t)r3 << (128 / 2)) | r2;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3200 {
// Port of softfp_template.h mul_u128:
// static uint128_t mul_u128(uint128_t *plow, uint128_t a, uint128_t b)
// Computes the full 256-bit product of a and b; returns the high 128 bits
// and stores the low 128 bits through *plow.

#[no_mangle]
pub extern "C" fn mul_u128(plow: *mut u128, a: u128, b: u128) -> u128 {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: u128 = (a0 as u128).wrapping_mul(b0 as u128);
    let r01: u128 = (a0 as u128).wrapping_mul(b1 as u128);
    let r10: u128 = (a1 as u128).wrapping_mul(b0 as u128);
    let r11: u128 = (a1 as u128).wrapping_mul(b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> 64)
        .wrapping_add(r01 as u64 as u128)
        .wrapping_add(r10 as u64 as u128);
    let r1: u64 = c as u64;
    c = (c >> 64)
        .wrapping_add(r01 >> 64)
        .wrapping_add(r10 >> 64)
        .wrapping_add(r11 as u64 as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64).wrapping_add(r11 >> 64)) as u64;

    unsafe {
        *plow = ((r1 as u128) << 64) | (r0 as u128);
    }
    ((r3 as u128) << 64) | (r2 as u128)
}
}

// ── `uint64_t a0, a1, b0, b1, r0, r1, r2, r3; uint128_t r00, r01, r10, r11, c; a0 = a; a1 = a >> (12…` · softfp_template.h:322-342 · chunk 3201 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t a0, a1, b0, b1, r0, r1, r2, r3;
//         uint128_t r00, r01, r10, r11, c;
//         a0 = a;
//         a1 = a >> (128 / 2);
//         b0 = b;
//         b1 = b >> (128 / 2);
//     
//         r00 = (uint128_t)a0 * (uint128_t)b0;
//         r01 = (uint128_t)a0 * (uint128_t)b1;
//         r10 = (uint128_t)a1 * (uint128_t)b0;
//         r11 = (uint128_t)a1 * (uint128_t)b1;
//     
//         r0 = r00;
//         c = (r00 >> (128 / 2)) + (uint64_t)r01 + (uint64_t)r10;
//         r1 = c;
//         c = (c >> (128 / 2)) + (r01 >> (128 / 2)) + (r10 >> (128 / 2)) + (uint64_t)r11;
//         r2 = c;
//         r3 = (c >> (128 / 2)) + (r11 >> (128 / 2));
//     
//         *plow = ((uint128_t)r1 << (128 / 2)) | r0;
//         return ((uint128_t)r3 << (128 / 2)) | r2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3201 {
// Port of mul_u128 (softfp_template.h lines 322-342): 256x256 -> 256 multiply
// split into 64-bit limbs.

#[no_mangle]
pub extern "C" fn mul_u128__part3201(plow: *mut u128, a: u128, b: u128) -> u128 {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: u128 = (a0 as u128).wrapping_mul(b0 as u128);
    let r01: u128 = (a0 as u128).wrapping_mul(b1 as u128);
    let r10: u128 = (a1 as u128).wrapping_mul(b0 as u128);
    let r11: u128 = (a1 as u128).wrapping_mul(b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> 64)
        .wrapping_add((r01 as u64) as u128)
        .wrapping_add((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> 64)
        .wrapping_add(r01 >> 64)
        .wrapping_add(r10 >> 64)
        .wrapping_add((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64) as u64).wrapping_add((r11 >> 64) as u64);

    unsafe {
        *plow = ((r1 as u128) << 64) | (r0 as u128);
    }
    ((r3 as u128) << 64) | (r2 as u128)
}
}

// ── `mul_sf32` · softfp_template.h:349-399 · chunk 2968 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t mul_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint32_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)) {
//             if (isnan_sf32(a) || isnan_sf32(b)) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 if ((a_exp == ((1 << 8) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 8) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else {
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
//         r_exp = a_exp + b_exp - (1 << (8 - 1)) + 2;
//     
//         r_mant = mul_u32(&r_mant_low,a_mant << ((32 - 2) - 23), b_mant << (((32 - 2) - 23) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2968 {
// Port of mul_sf32 (softfp_template.h) to Rust.
#![allow(non_snake_case)]

use core::ffi::c_int;

extern "C" {
    // defined (non-static) elsewhere in the C translation unit
    fn isnan_sf32(a: u32) -> c_int;
    fn issignan_sf32(a: u32) -> c_int;
}

// static const uint32_t F_QNAN32
const F_QNAN32: u32 = (0xffu32 << 23) | (1u32 << 22);

// static inline int clz32(uint32_t a)
#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffff)
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask != 0) as u32);
        }
    }
    a
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let mut rnd_bits: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let addend: u32 = match rm {
        0 | 4 => 1u32 << 4, // (1 << (((32 - 2) - 23) - 1))
        1 => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 5) - 1 // (1 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        }
    };

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 5) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 5) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 5;

    if rm == 0 && rnd_bits == (1u32 << 4) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> 24) as i32;
    if a_mant <= 0x007f_ffff {
        a_exp = 0;
    } else if a_exp >= 0xff {
        if addend == 0 {
            a_exp = 0xfe;
            a_mant = 0x007f_ffff;
        } else {
            a_exp = 0xff;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    // shift = clz32(a_mant) - (32 - 1 - (32 - 2)); C asserts shift >= 0
    let shift = clz32(a_mant) - (32 - 1 - (32 - 2));
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift = 23 - (32 - 1 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

// static uint32_t mul_u32(uint32_t *plow, uint32_t a, uint32_t b)
fn mul_u32(plow: &mut u32, a: u32, b: u32) -> u32 {
    let r = (a as u64) * (b as u64);
    *plow = r as u32;
    (r >> 32) as u32
}

#[no_mangle]
pub extern "C" fn mul_sf32(a: u32, b: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let a_sign: u32 = a >> 31;
    let b_sign: u32 = b >> 31;
    let r_sign: u32 = a_sign ^ b_sign;
    let mut a_exp: i32 = ((a >> 23) & 0xff) as i32;
    let mut b_exp: i32 = ((b >> 23) & 0xff) as i32;
    let mut a_mant: u32 = a & 0x007f_ffff;
    let mut b_mant: u32 = b & 0x007f_ffff;

    if a_exp == 0xff || b_exp == 0xff {
        if unsafe { isnan_sf32(a) } != 0 || unsafe { isnan_sf32(b) } != 0 {
            if unsafe { issignan_sf32(a) } != 0 || unsafe { issignan_sf32(b) } != 0 {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN32;
        } else {
            // infinity
            if (a_exp == 0xff && (b_exp == 0 && b_mant == 0))
                || (b_exp == 0xff && (a_exp == 0 && a_mant == 0))
            {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return F_QNAN32;
            } else {
                return pack_sf32(r_sign, 0xff, 0);
            }
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf32(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u32 << 23;
    }
    if b_exp == 0 {
        if b_mant == 0 {
            return pack_sf32(r_sign, 0, 0); // zero
        }
        b_mant = normalize_subnormal_sf32(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u32 << 23;
    }
    let r_exp: i32 = a_exp + b_exp - (1 << (8 - 1)) + 2;

    let mut r_mant_low: u32 = 0;
    let mut r_mant: u32 = mul_u32(
        &mut r_mant_low,
        a_mant.wrapping_shl(((32 - 2) - 23) as u32),
        b_mant.wrapping_shl((((32 - 2) - 23) + 1) as u32),
    );
    r_mant |= (r_mant_low != 0) as u32;
    normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `mul_sf64` · softfp_template.h:349-399 · chunk 3080 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t mul_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint64_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)) {
//             if (isnan_sf64(a) || isnan_sf64(b)) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 if ((a_exp == ((1 << 11) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 11) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else {
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
//         r_exp = a_exp + b_exp - (1 << (11 - 1)) + 2;
//     
//         r_mant = mul_u64(&r_mant_low,a_mant << ((64 - 2) - 52), b_mant << (((64 - 2) - 52) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3080 {
// Rust port of mul_sf64 (softfp_template.h, binary64 variant).
#![allow(dead_code, unused_unsafe)]

const F_QNAN64: u64 = ((0x7ffu64) << 52) | (1u64 << 51);
const MANT_MASK: u64 = (1u64 << 52) - 1;
const EXP_MASK: i32 = 0x7ff;
const RND_BITS: u32 = 10; // (64 - 2) - 52

fn clz64_impl(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64_impl(a_sign: u32, a_exp: i32, a_mant: u64) -> u64 {
    // mirror the C casts: int -> uint64_t, then shift left by 52
    ((a_sign as u64) << 63)
        | (a_exp as u32 as u64).wrapping_shl(52)
        | (a_mant & MANT_MASK)
}

fn normalize_subnormal_sf64_impl(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (64 - 1 - clz64_impl(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn mul_u64_impl(plow: &mut u64, a: u64, b: u64) -> u64 {
    let r: u128 = (a as u128) * (b as u128);
    *plow = r as u64;
    (r >> 64) as u64
}

fn isnan_sf64_impl(a: u64) -> i32 {
    let a_exp: u32 = ((a >> 52) & (EXP_MASK as u64)) as u32;
    let a_mant: u64 = a & MANT_MASK;
    (a_exp == EXP_MASK as u32 && a_mant != 0) as i32
}

fn issignan_sf64_impl(a: u64) -> i32 {
    let a_exp1: u32 = ((a >> 51) & 0xfffu64) as u32;
    let a_mant: u64 = a & MANT_MASK;
    (a_exp1 == 2 * (EXP_MASK as u32) && a_mant != 0) as i32
}

fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let d = d as u32;
            let mask: u64 = (1u64 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: u32,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    match rm {
        0 => addend = 1 << (RND_BITS - 1), // RM_RNE
        4 => addend = 1 << (RND_BITS - 1), // RM_RMM
        1 => addend = 0,                   // RM_RTZ
        _ => {
            // RM_RDN / RM_RUP (and default)
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << RND_BITS) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << RND_BITS) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << RND_BITS) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> RND_BITS;

    if rm == 0 && rnd_bits == (1 << (RND_BITS - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 53) as i32);
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | 1;
        }
    }
    pack_sf64_impl(a_sign, a_exp, a_mant)
}

fn normalize_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let shift: i32 = clz64_impl(a_mant) - (64 - 1 - (64 - 2));
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub unsafe extern "C" fn mul_sf64(a: u64, b: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let a_sign: u32 = (a >> (64 - 1)) as u32;
    let b_sign: u32 = (b >> (64 - 1)) as u32;
    let r_sign: u32 = a_sign ^ b_sign;
    let mut a_exp: i32 = ((a >> 52) & (EXP_MASK as u64)) as i32;
    let mut b_exp: i32 = ((b >> 52) & (EXP_MASK as u64)) as i32;
    let mut a_mant: u64 = a & MANT_MASK;
    let mut b_mant: u64 = b & MANT_MASK;

    if a_exp == EXP_MASK || b_exp == EXP_MASK {
        if isnan_sf64_impl(a) != 0 || isnan_sf64_impl(b) != 0 {
            if issignan_sf64_impl(a) != 0 || issignan_sf64_impl(b) != 0 {
                *pfflags |= 1 << 4;
            }
            return F_QNAN64;
        } else {
            // infinity
            if (a_exp == EXP_MASK && (b_exp == 0 && b_mant == 0))
                || (b_exp == EXP_MASK && (a_exp == 0 && a_mant == 0))
            {
                *pfflags |= 1 << 4;
                return F_QNAN64;
            } else {
                return pack_sf64_impl(r_sign, EXP_MASK, 0);
            }
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf64_impl(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf64_impl(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u64 << 52;
    }
    if b_exp == 0 {
        if b_mant == 0 {
            return pack_sf64_impl(r_sign, 0, 0); // zero
        }
        b_mant = normalize_subnormal_sf64_impl(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u64 << 52;
    }
    let r_exp: i32 = a_exp.wrapping_add(b_exp).wrapping_sub(1 << (11 - 1)).wrapping_add(2);

    let mut r_mant_low: u64 = 0;
    let mut r_mant: u64 = mul_u64_impl(
        &mut r_mant_low,
        a_mant << ((64 - 2) - 52),
        b_mant << (((64 - 2) - 52) + 1),
    );
    r_mant |= (r_mant_low != 0) as u64;
    normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `mul_sf128` · softfp_template.h:349-399 · chunk 3202 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t mul_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint128_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)) {
//             if (isnan_sf128(a) || isnan_sf128(b)) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 if ((a_exp == ((1 << 15) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 15) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else {
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
//         r_exp = a_exp + b_exp - (1 << (15 - 1)) + 2;
//     
//         r_mant = mul_u128(&r_mant_low,a_mant << ((128 - 2) - 112), b_mant << (((128 - 2) - 112) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3202 {
// Port of `mul_sf128` from softfp_template.h (128-bit soft-float multiply).
// All static helpers it needs (which have no linker symbol in C) are ported
// here as private functions with the exact same semantics.

use core::ffi::{c_int, c_uint};

const MANT_SIZE128: u32 = 112;
const MANT_MASK128: u128 = (1u128 << MANT_SIZE128) - 1;
const EXP_MAX128: u32 = (1u32 << 15) - 1;
const F_QNAN128: u128 = ((EXP_MAX128 as u128) << MANT_SIZE128) | (1u128 << (MANT_SIZE128 - 1));

// FFLAG bits (as used by the C source)
const FFLAG_INVALID_OP: u32 = 1 << 4;
const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

// Rounding modes
const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

#[inline]
fn clz128(a: u128) -> i32 {
    // Exactly like the C helper: 128 for zero, else leading zero count.
    a.leading_zeros() as i32
}

#[inline]
fn shl128(a: u128, d: i32) -> u128 {
    if d < 0 {
        0
    } else {
        a.wrapping_shl(d as u32)
    }
}

#[inline]
fn shr128(a: u128, d: i32) -> u128 {
    if d < 0 {
        0
    } else {
        a.wrapping_shr(d as u32)
    }
}

fn isnan_sf128(a: u128) -> bool {
    let a_exp = ((a >> MANT_SIZE128) & (EXP_MAX128 as u128)) as u32;
    let a_mant = a & MANT_MASK128;
    a_exp == EXP_MAX128 && a_mant != 0
}

fn issignan_sf128(a: u128) -> bool {
    let a_exp1 = ((a >> (MANT_SIZE128 - 1)) & 0xffffu128) as u32;
    let a_mant = a & MANT_MASK128;
    a_exp1 == 2 * EXP_MAX128 && a_mant != 0
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << MANT_SIZE128) | (a_mant & MANT_MASK128)
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = (1u128.wrapping_shl(d as u32)).wrapping_sub(1);
            a = shr128(a, d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((128 - 2) - MANT_SIZE128 as i32 - 1) as u32);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (and default)
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((128 - 2) - MANT_SIZE128)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - MANT_SIZE128)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - MANT_SIZE128)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = shr128(a_mant.wrapping_add(addend as u128), (128 - 2) - MANT_SIZE128 as i32);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - MANT_SIZE128) - 1)) {
        a_mant &= !1u128;
    }

    a_exp += (shr128(a_mant, MANT_SIZE128 as i32 + 1)) as i32;
    if a_mant <= MANT_MASK128 {
        a_exp = 0;
    } else if a_exp >= EXP_MAX128 as i32 {
        if addend == 0 {
            a_exp = EXP_MAX128 as i32 - 1;
            a_mant = MANT_MASK128;
        } else {
            a_exp = EXP_MAX128 as i32;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    let mut a_exp = a_exp;
    a_exp -= shift;
    let a_mant = shl128(a_mant, shift);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize_subnormal_sf128(pa_exp: &mut i32, a_mant: u128) -> u128 {
    let shift: i32 = MANT_SIZE128 as i32 - (128 - 1 - clz128(a_mant));
    *pa_exp = 1 - shift;
    shl128(a_mant, shift)
}

fn mul_u128(plow: &mut u128, a: u128, b: u128) -> u128 {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: u128 = (a0 as u128) * (b0 as u128);
    let r01: u128 = (a0 as u128) * (b1 as u128);
    let r10: u128 = (a1 as u128) * (b0 as u128);
    let r11: u128 = (a1 as u128) * (b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> 64)
        .wrapping_add((r01 as u64) as u128)
        .wrapping_add((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> 64)
        .wrapping_add(r01 >> 64)
        .wrapping_add(r10 >> 64)
        .wrapping_add((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64) as u64).wrapping_add((r11 >> 64) as u64);

    *plow = ((r1 as u128) << 64) | (r0 as u128);
    ((r3 as u128) << 64) | (r2 as u128)
}

#[no_mangle]
pub unsafe extern "C" fn mul_sf128(a: u128, b: u128, rm: c_uint, pfflags: *mut u32) -> u128 {
    let mut a_exp: i32;
    let mut b_exp: i32;
    let mut a_mant: u128;
    let mut b_mant: u128;
    let mut r_mant_low: u128 = 0;

    let a_sign: u32 = (a >> (128 - 1)) as u32;
    let b_sign: u32 = (b >> (128 - 1)) as u32;
    let r_sign: u32 = a_sign ^ b_sign;
    a_exp = ((a >> MANT_SIZE128) & (EXP_MAX128 as u128)) as i32;
    b_exp = ((b >> MANT_SIZE128) & (EXP_MAX128 as u128)) as i32;
    a_mant = a & MANT_MASK128;
    b_mant = b & MANT_MASK128;

    if a_exp == EXP_MAX128 as i32 || b_exp == EXP_MAX128 as i32 {
        if isnan_sf128(a) || isnan_sf128(b) {
            if issignan_sf128(a) || issignan_sf128(b) {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN128;
        } else {
            // infinity
            if (a_exp == EXP_MAX128 as i32 && (b_exp == 0 && b_mant == 0))
                || (b_exp == EXP_MAX128 as i32 && (a_exp == 0 && a_mant == 0))
            {
                *pfflags |= FFLAG_INVALID_OP;
                return F_QNAN128;
            } else {
                return pack_sf128(r_sign, EXP_MAX128, 0);
            }
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf128(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf128(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u128 << MANT_SIZE128;
    }
    if b_exp == 0 {
        if b_mant == 0 {
            return pack_sf128(r_sign, 0, 0); // zero
        }
        b_mant = normalize_subnormal_sf128(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u128 << MANT_SIZE128;
    }
    let r_exp: i32 = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1 << (15 - 1))
        .wrapping_add(2);

    let r_mant: u128 = mul_u128(
        &mut r_mant_low,
        shl128(a_mant, (128 - 2) - MANT_SIZE128 as i32),
        shl128(b_mant, ((128 - 2) - MANT_SIZE128 as i32) + 1),
    );
    let r_mant: u128 = r_mant | (r_mant_low != 0) as u128;
    normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint32_t a_mant, b_mant, r_mant,…` · softfp_template.h:352-362 · chunk 2969 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint32_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2969 {
// Port of the initial sign/exp/mantissa unpacking of mul_sf32.

#[no_mangle]
pub extern "C" fn mul_sf32__part2969(
    a: u32,
    b: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    a_mant: *mut u32,
    b_mant: *mut u32,
) {
    unsafe {
        let as_: u32 = a >> 31;
        let bs: u32 = b >> 31;
        *a_sign = as_;
        *b_sign = bs;
        *r_sign = as_ ^ bs;
        *a_exp = ((a >> 23) & 0xffu32) as i32;
        *b_exp = ((b >> 23) & 0xffu32) as i32;
        *a_mant = a & ((1u32 << 23) - 1);
        *b_mant = b & ((1u32 << 23) - 1);
    }
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint64_t a_mant, b_mant, r_mant,…` · softfp_template.h:352-362 · chunk 3081 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint64_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3081 {
// Performs the unpacking/field-extraction part of mul_sf64 (softfp_template.h lines 352-362).
// The results are written back through the out-pointers.

#[no_mangle]
pub extern "C" fn mul_sf64__part3081(
    a: u64,
    b: u64,
    a_sign_out: *mut u32,
    b_sign_out: *mut u32,
    r_sign_out: *mut u32,
    a_exp_out: *mut i32,
    b_exp_out: *mut i32,
    a_mant_out: *mut u64,
    b_mant_out: *mut u64,
) {
    unsafe {
        let a_sign: u32 = (a >> (64 - 1)) as u32;
        let b_sign: u32 = (b >> (64 - 1)) as u32;
        let r_sign: u32 = a_sign ^ b_sign;
        let a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
        let b_exp: i32 = ((b >> 52) & ((1u64 << 11) - 1)) as i32;
        let a_mant: u64 = a & ((1u64 << 52) - 1);
        let b_mant: u64 = b & ((1u64 << 52) - 1);

        if !a_sign_out.is_null() {
            *a_sign_out = a_sign;
        }
        if !b_sign_out.is_null() {
            *b_sign_out = b_sign;
        }
        if !r_sign_out.is_null() {
            *r_sign_out = r_sign;
        }
        if !a_exp_out.is_null() {
            *a_exp_out = a_exp;
        }
        if !b_exp_out.is_null() {
            *b_exp_out = b_exp;
        }
        if !a_mant_out.is_null() {
            *a_mant_out = a_mant;
        }
        if !b_mant_out.is_null() {
            *b_mant_out = b_mant;
        }
    }
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint128_t a_mant, b_mant, r_mant,…` · softfp_template.h:352-362 · chunk 3203 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint128_t a_mant, b_mant, r_mant, r_mant_low;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3203 {
// Extract of the leading "unpack" step of mul_sf128 (softfp_template.h lines 352-362).

#[no_mangle]
pub extern "C" fn mul_sf128__part3203(
    a: u128,
    b: u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    a_mant: *mut u128,
    b_mant: *mut u128,
) {
    unsafe {
        let asg: u32 = (a >> (128 - 1)) as u32;
        let bsg: u32 = (b >> (128 - 1)) as u32;
        if !a_sign.is_null() {
            *a_sign = asg;
        }
        if !b_sign.is_null() {
            *b_sign = bsg;
        }
        if !r_sign.is_null() {
            *r_sign = asg ^ bsg;
        }
        let aexp: i32 = (((a >> 112) & ((1u128 << 15) - 1)) & 0xffff_ffffu128) as i32;
        let bexp: i32 = (((b >> 112) & ((1u128 << 15) - 1)) & 0xffff_ffffu128) as i32;
        if !a_exp.is_null() {
            *a_exp = aexp;
        }
        if !b_exp.is_null() {
            *b_exp = bexp;
        }
        if !a_mant.is_null() {
            *a_mant = a & ((1u128 << 112) - 1);
        }
        if !b_mant.is_null() {
            *b_mant = b & ((1u128 << 112) - 1);
        }
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)` · softfp_template.h:363-379 · chunk 2970 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)) {
//             if (isnan_sf32(a) || isnan_sf32(b)) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 if ((a_exp == ((1 << 8) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 8) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else {
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2970 {
use core::ffi::c_int;

// F_QNAN32 = ((0xff << 23) | (1 << 22))
const F_QNAN32: u32 = 0x7fc0_0000u32;

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
}

// static-int-literal logic of isnan_sf32 (ported: no linker symbol needed here)
#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & 0xffu32;
    let a_mant = a & 0x007f_ffffu32;
    a_exp == 0xffu32 && a_mant != 0
}

// static-int-literal logic of issignan_sf32
#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1 = (a >> 22) & 0x1ffu32;
    let a_mant = a & 0x007f_ffffu32;
    a_exp1 == 0x1feu32 && a_mant != 0
}

/// Returns 1 when the caller must immediately `return *ret;`, 0 to fall through.
#[no_mangle]
pub extern "C" fn mul_sf32__part2970(
    a: u32,
    b: u32,
    a_exp: i32,
    b_exp: i32,
    a_mant: u32,
    b_mant: u32,
    r_sign: u32,
    pfflags: *mut u32,
    ret: *mut u32,
) -> c_int {
    unsafe {
        if a_exp == 0xffi32 || b_exp == 0xffi32 {
            if isnan_sf32(a) || isnan_sf32(b) {
                if issignan_sf32(a) || issignan_sf32(b) {
                    *pfflags |= 1u32 << 4;
                }
                *ret = F_QNAN32;
                return 1;
            } else {
                if (a_exp == 0xffi32 && (b_exp == 0 && b_mant == 0))
                    || (b_exp == 0xffi32 && (a_exp == 0 && a_mant == 0))
                {
                    *pfflags |= 1u32 << 4;
                    *ret = F_QNAN32;
                    return 1;
                } else {
                    *ret = pack_sf32(r_sign, 0xffu32, 0u32);
                    return 1;
                }
            }
        }
        0
    }
}
}

// ── `if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)` · softfp_template.h:363-379 · chunk 3082 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)) {
//             if (isnan_sf64(a) || isnan_sf64(b)) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 if ((a_exp == ((1 << 11) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 11) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else {
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3082 {
// Partial port of mul_sf64: the inf/nan special-case handling.
use core::ffi::c_int;

extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

const F_QNAN64: u64 = ((1u64 << 11) - 1) << 52 | (1u64 << (52 - 1));

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn mul_sf64__part3082(
    a: u64,
    b: u64,
    a_exp: i32,
    b_exp: i32,
    r_sign: u32,
    a_mant: u64,
    b_mant: u64,
    pfflags: *mut u32,
    p_returned: *mut c_int,
) -> u64 {
    unsafe {
        if a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1) {
            if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 {
                if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *p_returned = 1;
                return F_QNAN64;
            } else {
                if (a_exp == ((1 << 11) - 1) && (b_exp == 0 && b_mant == 0))
                    || (b_exp == ((1 << 11) - 1) && (a_exp == 0 && a_mant == 0))
                {
                    *pfflags |= 1u32 << 4;
                    *p_returned = 1;
                    return F_QNAN64;
                } else {
                    *p_returned = 1;
                    return pack_sf64(r_sign, ((1 << 11) - 1) as u32, 0);
                }
            }
        }
        *p_returned = 0;
        0
    }
}
}

// ── `if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)` · softfp_template.h:363-379 · chunk 3204 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)) {
//             if (isnan_sf128(a) || isnan_sf128(b)) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 if ((a_exp == ((1 << 15) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 15) - 1) && (a_exp == 0 && a_mant == 0))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else {
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3204 {
#![allow(non_camel_case_types)]
#![allow(improper_ctypes_definitions)]

use core::ffi::c_int;

type uint128_t = u128;

const MANT_MASK: uint128_t = (1u128 << 112) - 1;
const EXP_MAX: i32 = (1 << 15) - 1;
const F_QNAN128: uint128_t = (((EXP_MAX as uint128_t) << 112) | (1u128 << (112 - 1)));

extern "C" {
    fn isnan_sf128(a: uint128_t) -> c_int;
    fn issignan_sf128(a: uint128_t) -> c_int;
}

// ported from static inline uint128_t pack_sf128(...)
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: uint128_t) -> uint128_t {
    ((a_sign as uint128_t) << (128 - 1)) | ((a_exp as uint128_t) << 112) | (a_mant & MANT_MASK)
}

// Ported special-case (NaN / inf / invalid) block of mul_sf128.
// Returns 0 when the block does not apply (computation continues in C);
// returns 1 when the block returns, with the value stored through `pout`.
#[no_mangle]
pub extern "C" fn mul_sf128__part3204(
    a: uint128_t,
    b: uint128_t,
    a_exp: i32,
    b_exp: i32,
    a_mant: uint128_t,
    b_mant: uint128_t,
    r_sign: u32,
    pfflags: *mut u32,
    pout: *mut uint128_t,
) -> c_int {
    unsafe {
        if a_exp == EXP_MAX || b_exp == EXP_MAX {
            if isnan_sf128(a) != 0 || isnan_sf128(b) != 0 {
                if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                    *pfflags |= 1 << 4;
                }
                *pout = F_QNAN128;
                return 1;
            } else {
                if (a_exp == EXP_MAX && (b_exp == 0 && b_mant == 0))
                    || (b_exp == EXP_MAX && (a_exp == 0 && a_mant == 0))
                {
                    *pfflags |= 1 << 4;
                    *pout = F_QNAN128;
                    return 1;
                } else {
                    *pout = pack_sf128(r_sign, EXP_MAX as u32, 0);
                    return 1;
                }
            }
        }
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:380-386 · chunk 2971 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2971 {
// Port of the "a is zero / subnormal" normalization block of mul_sf32 (softfp_template.h 380-386).

use core::ffi::c_int;

// ported from the static inline clz32 in the same translation unit
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// ported from the static inline normalize_subnormal_sf32
fn normalize_subnormal_sf32(pa_exp: *mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - (32 - 1 - clz32(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    // C shifts by `shift`; wrapping_shl reproduces the hardware shift-count masking
    a_mant.wrapping_shl(shift as u32)
}

// ported from the static inline pack_sf32
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

/// Returns 0 to continue, or 1 when the caller must return the value in *pret.
#[no_mangle]
pub extern "C" fn mul_sf32__part2971(
    pa_exp: *mut i32,
    pa_mant: *mut u32,
    r_sign: u32,
    pret: *mut u32,
) -> c_int {
    unsafe {
        let a_exp = *pa_exp;
        let a_mant = *pa_mant;
        if a_exp == 0 {
            if a_mant == 0 {
                *pret = pack_sf32(r_sign, 0, 0);
                return 1;
            }
            *pa_mant = normalize_subnormal_sf32(pa_exp, a_mant);
        } else {
            *pa_mant = a_mant | (1u32 << 23);
        }
    }
    0
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:380-386 · chunk 3083 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3083 {
// Ported piece of mul_sf64: first-operand (a) unpacking / subnormal normalization.

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & 0x000F_FFFF_FFFF_FFFFu64)
}

#[inline]
fn normalize_subnormal_sf64(pa_exp: *mut i32, a_mant: u64) -> u64 {
    let clz: i32 = if a_mant == 0 { 64 } else { a_mant.leading_zeros() as i32 };
    let shift: i32 = 52 - (64 - 1 - clz);
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}

#[no_mangle]
pub extern "C" fn mul_sf64__part3083(
    r_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u64,
    pearly: *mut i32,
) -> u64 {
    unsafe {
        let a_exp: i32 = *pa_exp;
        let mut a_mant: u64 = *pa_mant;
        if a_exp == 0 {
            if a_mant == 0 {
                *pearly = 1;
                return pack_sf64(r_sign, 0, 0);
            }
            a_mant = normalize_subnormal_sf64(pa_exp, a_mant);
            *pa_mant = a_mant;
        } else {
            *pa_mant = a_mant | (1u64 << 52);
        }
        *pearly = 0;
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:380-386 · chunk 3205 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3205 {
// Ported piece of mul_sf128: the handling of the operand `a`
// (subnormal normalization / implicit-bit insertion), including the
// early return of a zero result.

use core::ffi::c_int;

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

/// Returns 1 when the piece performed the early `return pack_sf128(r_sign, 0, 0)`
/// (the packed value is stored in *p_result), 0 otherwise (in which case
/// *pa_exp and *pa_mant hold the updated a_exp / a_mant).
#[no_mangle]
pub extern "C" fn mul_sf128__part3205(
    r_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    p_result: *mut u128,
) -> c_int {
    unsafe {
        let mut a_exp: i32 = *pa_exp;
        let mut a_mant: u128 = *pa_mant;

        if a_exp == 0 {
            if a_mant == 0 {
                // pack_sf128(r_sign, 0, 0)
                *p_result = (r_sign as u128).wrapping_shl(127);
                return 1;
            }
            // normalize_subnormal_sf128(&a_exp, a_mant)
            let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
            a_exp = 1 - shift;
            a_mant = a_mant.wrapping_shl(shift as u32);
        } else {
            a_mant |= (1u128).wrapping_shl(112);
        }

        *pa_exp = a_exp;
        *pa_mant = a_mant;
        0
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:387-393 · chunk 2972 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2972 {
// Port of softfp_template.h lines 387-393 (the b-operand normalization step of
// mul_sf32). The helpers it uses (pack_sf32, normalize_subnormal_sf32 and the
// clz32 builtin they rely on) are `static inline` in C, hence private here.

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
}

// static inline int clz32(uint32_t a)
#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
#[inline]
unsafe fn normalize_subnormal_sf32(pa_exp: *mut i32, a_mant: u32) -> u32 {
    let shift = 23 - (31 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

/// Returns 1 when the C code must `return` the value stored in `*pret`,
/// 0 when execution continues with the updated `*pb_exp` / `*pb_mant`.
#[no_mangle]
pub unsafe extern "C" fn mul_sf32__part2972(
    r_sign: u32,
    pb_exp: *mut i32,
    pb_mant: *mut u32,
    pret: *mut u32,
) -> i32 {
    let b_exp = *pb_exp;
    let mut b_mant = *pb_mant;

    if b_exp == 0 {
        if b_mant == 0 {
            *pret = pack_sf32(r_sign, 0, 0);
            return 1;
        }
        b_mant = normalize_subnormal_sf32(pb_exp, b_mant);
        *pb_mant = b_mant;
    } else {
        b_mant |= 1u32 << 23;
        *pb_mant = b_mant;
    }
    0
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:387-393 · chunk 3084 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3084 {
// Port of the "b operand subnormal/normal handling" part of mul_sf64.

use core::ffi::c_int;

// Ported from the C static inline clz64 (no linker symbol).
#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// Ported from the C static inline normalize_subnormal_sf64.
#[inline]
fn normalize_subnormal_sf64(a_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (64 - 1 - clz64(a_mant));
    *a_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

// Returns nonzero when the caller must return pack_sf64(r_sign, 0, 0).
#[no_mangle]
pub extern "C" fn mul_sf64__part3084(b_exp: *mut i32, b_mant: *mut u64) -> c_int {
    unsafe {
        if *b_exp == 0 {
            if *b_mant == 0 {
                return 1;
            }
            *b_mant = normalize_subnormal_sf64(&mut *b_exp, *b_mant);
        } else {
            *b_mant |= 1u64 << 52;
        }
    }
    0
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:387-393 · chunk 3206 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3206 {
// Port of the b-operand unpacking block of mul_sf128 (softfp_template.h lines 387-393).
//
// Reads b_exp / b_mant and r_sign.  If b_exp == 0 and b_mant == 0 the C code
// returns pack_sf128(r_sign, 0, 0); that is reported back through *p_ret with a
// nonzero return value.  Otherwise b_exp / b_mant are updated in place and 0 is
// returned so the caller continues.

// pack_sf128(r_sign, 0, 0) -- pack_sf128 is `static inline`, so its logic is
// mirrored here instead of being linked to.
#[inline]
fn pack_zero(r_sign: u32) -> u128 {
    ((r_sign as u128) << (128 - 1)) | ((0u128) << 112) | (0u128 & ((1u128 << 112) - 1))
}

// normalize_subnormal_sf128 -- also `static inline` in the C source.
#[inline]
fn normalize_subnormal(pa_exp: *mut i32, a_mant: u128) -> u128 {
    // clz128: 128 when the value is zero, otherwise the number of leading zeros.
    let clz: i32 = a_mant.leading_zeros() as i32;
    let shift: i32 = 112 - ((128 - 1) - clz);
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}

#[no_mangle]
pub extern "C" fn mul_sf128__part3206(
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    r_sign: u32,
    p_ret: *mut u128,
) -> i32 {
    unsafe {
        let b_exp: i32 = *pa_exp;
        let b_mant: u128 = *pa_mant;

        if b_exp == 0 {
            if b_mant == 0 {
                *p_ret = pack_zero(r_sign);
                return 1;
            }
            let new_mant = normalize_subnormal(pa_exp, b_mant);
            *pa_mant = new_mant;
        } else {
            *pa_mant = b_mant | ((1u128) << 112);
        }
    }
    0
}
}

// ── `r_exp = a_exp + b_exp - (1 << (8 - 1)) + 2; r_mant = mul_u32(&r_mant_low,a_mant << ((32 - 2) -…` · softfp_template.h:394-398 · chunk 2973 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (8 - 1)) + 2;
//     
//         r_mant = mul_u32(&r_mant_low,a_mant << ((32 - 2) - 23), b_mant << (((32 - 2) - 23) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2973 {
// Port of the tail of mul_sf32 (softfp_template.h lines 394-398).
// The helpers mul_u32 / normalize_sf32 / roundpack_sf32 / pack_sf32 / clz32 are
// `static` (or `static inline`) in the C translation unit, so they have no
// linker symbol: their logic is reproduced here as private functions.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

fn clz32(a: u32) -> i32 {
    a.leading_zeros() as i32
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => addend = 1u32 << 6,
        RM_RTZ => addend = 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 24) as i32);
    if a_mant <= 0x007f_ffffu32 {
        a_exp = 0;
    } else if a_exp >= 255 {
        if addend == 0 {
            a_exp = 254;
            a_mant = 0x007f_ffffu32;
        } else {
            a_exp = 255;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    mut a_exp: i32,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // shift = clz32(a_mant) - (32 - 1 - (32 - 2));  the C code asserts shift >= 0
    let shift: i32 = clz32(a_mant) - 1;
    a_exp = a_exp.wrapping_sub(shift);
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn mul_u32(plow: &mut u32, a: u32, b: u32) -> u32 {
    let r: u64 = (a as u64).wrapping_mul(b as u64);
    *plow = r as u32;
    (r >> 32) as u32
}

#[no_mangle]
pub extern "C" fn mul_sf32__part2973(
    r_sign: u32,
    a_exp: i32,
    b_exp: i32,
    a_mant: u32,
    b_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let r_exp: i32 = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1 << (8 - 1))
        .wrapping_add(2);

    let mut r_mant_low: u32 = 0;
    let mut r_mant: u32 = mul_u32(
        &mut r_mant_low,
        a_mant.wrapping_shl((32 - 2 - 23) as u32),
        b_mant.wrapping_shl(((32 - 2 - 23) + 1) as u32),
    );
    if r_mant_low != 0 {
        r_mant |= 1;
    }
    normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `r_exp = a_exp + b_exp - (1 << (11 - 1)) + 2; r_mant = mul_u64(&r_mant_low,a_mant << ((64 - 2) -…` · softfp_template.h:394-398 · chunk 3085 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (11 - 1)) + 2;
//     
//         r_mant = mul_u64(&r_mant_low,a_mant << ((64 - 2) - 52), b_mant << (((64 - 2) - 52) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3085 {
// Port of the tail of mul_sf64 (softfp_template.h lines 394-398):
//   r_exp = a_exp + b_exp - (1 << (11 - 1)) + 2;
//   r_mant = mul_u64(&r_mant_low, a_mant << ((64-2)-52), b_mant << (((64-2)-52)+1));
//   r_mant |= (r_mant_low != 0);
//   return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);
// The C helpers mul_u64, normalize_sf64, roundpack_sf64, rshift_rnd64, clz64 and
// pack_sf64 are `static` in the translation unit, so their logic is mirrored here.

#![allow(non_snake_case)]

use core::ffi::c_uint;

const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RMM: c_uint = 4;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: c_uint,
    pfflags: *mut u32,
) -> u64 {
    let addend: u32;
    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        // subnormal result
        let is_subnormal = a_exp < 0
            || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = (a_exp as u64).wrapping_add(a_mant >> (52 + 1)) as i32;
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: c_uint,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    // C asserts shift >= 0 here.
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn mul_sf64__part3085(
    r_sign: u32,
    a_exp: i32,
    b_exp: i32,
    a_mant: u64,
    b_mant: u64,
    rm: c_uint,
    pfflags: *mut u32,
) -> u64 {
    let r_exp: i32 = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1 << (11 - 1))
        .wrapping_add(2);

    // mul_u64(&r_mant_low, a_mant << 10, b_mant << 11)
    let prod: u128 = ((a_mant << ((64 - 2) - 52)) as u128)
        * ((b_mant << (((64 - 2) - 52) + 1)) as u128);
    let r_mant_low: u64 = prod as u64;
    let mut r_mant: u64 = (prod >> 64) as u64;
    r_mant |= (r_mant_low != 0) as u64;

    normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `r_exp = a_exp + b_exp - (1 << (15 - 1)) + 2; r_mant = mul_u128(&r_mant_low,a_mant << ((128 - 2)…` · softfp_template.h:394-398 · chunk 3207 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (15 - 1)) + 2;
//     
//         r_mant = mul_u128(&r_mant_low,a_mant << ((128 - 2) - 112), b_mant << (((128 - 2) - 112) + 1));
//         r_mant |= (r_mant_low != 0);
//         return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3207 {
// Port of the tail of mul_sf128 (softfp_template.h lines 394-398):
// exponent computation, 256-bit mantissa multiply, and normalization.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

fn rshift_rnd128(a_arg: u128, d: i32) -> u128 {
    let mut a = a_arg;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_arg: i32,
    a_mant_arg: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_arg;
    let mut a_mant = a_mant_arg;
    let addend: u32;
    let mut rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1 << 6;
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1 << 14) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << 14) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << 14) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    if rm == RM_RNE && rnd_bits == (1 << 6) {
        a_mant &= u128::MAX - 1;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 113) as i32);
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_arg: i32,
    a_mant_arg: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_arg;
    let mut a_mant = a_mant_arg;
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    a_exp = a_exp.wrapping_sub(shift);
    if shift >= 0 {
        a_mant = a_mant << (shift as u32);
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// returns (high, low)
fn mul_u128(a: u128, b: u128) -> (u128, u128) {
    let a0 = a as u64;
    let a1 = (a >> 64) as u64;
    let b0 = b as u64;
    let b1 = (b >> 64) as u64;

    let r00 = (a0 as u128) * (b0 as u128);
    let r01 = (a0 as u128) * (b1 as u128);
    let r10 = (a1 as u128) * (b0 as u128);
    let r11 = (a1 as u128) * (b1 as u128);

    let r0 = r00 as u64;
    let mut c: u128 = (r00 >> 64) + ((r01 as u64) as u128) + ((r10 as u64) as u128);
    let r1 = c as u64;
    c = (c >> 64) + (r01 >> 64) + (r10 >> 64) + ((r11 as u64) as u128);
    let r2 = c as u64;
    let r3 = ((c >> 64) + (r11 >> 64)) as u64;

    let low = ((r1 as u128) << 64) | (r0 as u128);
    let high = ((r3 as u128) << 64) | (r2 as u128);
    (high, low)
}

#[no_mangle]
pub extern "C" fn mul_sf128__part3207(
    r_sign: u32,
    a_exp: i32,
    b_exp: i32,
    a_mant: u128,
    b_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let r_exp = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1 << (15 - 1))
        .wrapping_add(2);

    let (mut r_mant, r_mant_low) =
        mul_u128(a_mant << ((128 - 2) - 112), b_mant << (((128 - 2) - 112) + 1));
    r_mant |= (r_mant_low != 0) as u128;

    normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `fma_sf32` · softfp_template.h:402-552 · chunk 2974 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t fma_sf32(uint32_t a, uint32_t b, uint32_t c, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint32_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         c_sign = c >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         c_exp = (c >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
//         c_mant = c & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1) || c_exp == ((1 << 8) - 1)) {
//             if (isnan_sf32(a) || isnan_sf32(b) || isnan_sf32(c)) {
//                 if (issignan_sf32(a) || issignan_sf32(b) || issignan_sf32(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 if ((a_exp == ((1 << 8) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 8) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)) &&
//                      (c_exp == ((1 << 8) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else if (c_exp == ((1 << 8) - 1)) {
//                     return pack_sf32(c_sign, ((1 << 8) - 1), 0);
//                 } else {
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf32(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
//     
//         r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3;
//     
//         r_mant1 = mul_u32(&r_mant0, a_mant << ((32 - 2) - 23), b_mant << ((32 - 2) - 23));
//     
//         if (r_mant1 < ((uint32_t)1 << (32 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (32 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
//     
//     
//         if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf32(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf32(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint32_t)1 << 23;
//         }
//         c_exp++;
//         c_mant1 = c_mant << (((32 - 2) - 23) - 1);
//         c_mant0 = 0;
//     
//     
//     
//     
//     
//         if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint32_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
//     
//         shift = r_exp - c_exp;
//         if (shift >= 2 * 32) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 32 + 1) {
//             c_mant0 = rshift_rnd32(c_mant1, shift - 32);
//             c_mant1 = 0;
//         } else if (shift == 32) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint32_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (32 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
//     
//     
//     
//         if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint32_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
//         return normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2974 {
// Rust port of softfp_template.h fma_sf32 (binary32 fused multiply-add).
#![allow(clippy::all)]

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RDN: u32 = 2;
const RM_RUP: u32 = 3;
const RM_RMM: u32 = 4;

// F_QNAN32
const F_QNAN32: u32 = (0xffu32 << 23) | (1u32 << (23 - 1));

extern "C" {
    fn isnan_sf32(a: u32) -> c_int;
    fn issignan_sf32(a: u32) -> c_int;
}

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            (a != 0) as u32
        } else if d < 0 {
            a
        } else {
            let mask = (1u32.wrapping_shl(d as u32)) - 1;
            (a.wrapping_shr(d as u32)) | ((a & mask) != 0) as u32
        }
    } else {
        a
    }
}

fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift = 23 - ((32 - 1) - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn mul_u32(plow: &mut u32, a: u32, b: u32) -> u32 {
    let r = (a as u64) * (b as u64);
    *plow = r as u32;
    (r >> 32) as u32
}

fn roundpack_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: u32, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let rnd_bits: u32;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN / RM_RUP / default
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal = a_exp < 0
            || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as i32;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: u32, pfflags: *mut u32) -> u32 {
    let shift = clz32(a_mant_in) - ((32 - 1) - (32 - 2));
    let a_exp = a_exp_in - shift;
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize2_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant1_in: u32,
    a_mant0_in: u32,
    rm: u32,
    pfflags: *mut u32,
) -> u32 {
    let mut a_mant1 = a_mant1_in;
    let mut a_mant0 = a_mant0_in;
    let l: i32;
    if a_mant1 == 0 {
        l = 32 + clz32(a_mant0);
    } else {
        l = clz32(a_mant1);
    }
    let shift = l - ((32 - 1) - (32 - 2));
    let a_exp = a_exp_in - shift;
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u32;
    } else if shift < 32 {
        if shift > 0 {
            a_mant1 = a_mant1.wrapping_shl(shift as u32)
                | a_mant0.wrapping_shr((32 - shift) as u32);
            a_mant0 = a_mant0.wrapping_shl(shift as u32);
            a_mant1 |= (a_mant0 != 0) as u32;
        }
    } else {
        a_mant1 = a_mant0.wrapping_shl((shift - 32) as u32);
    }
    roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn fma_sf32(a: u32, b: u32, c: u32, rm: u32, pfflags: *mut u32) -> u32 {
    let a_sign = a >> (32 - 1);
    let b_sign = b >> (32 - 1);
    let mut c_sign = c >> (32 - 1);
    let mut r_sign = a_sign ^ b_sign;
    let mut a_exp = ((a >> 23) & ((1 << 8) - 1)) as i32;
    let mut b_exp = ((b >> 23) & ((1 << 8) - 1)) as i32;
    let mut c_exp = ((c >> 23) & ((1 << 8) - 1)) as i32;
    let mut a_mant = a & ((1u32 << 23) - 1);
    let mut b_mant = b & ((1u32 << 23) - 1);
    let mut c_mant = c & ((1u32 << 23) - 1);
    let mut r_exp: i32;
    let mut r_mant1: u32;
    let mut r_mant0: u32 = 0;
    let mut c_mant1: u32;
    let mut c_mant0: u32;
    let mask: u32;
    let shift: i32;

    const EXP_MASK: i32 = (1 << 8) - 1;

    if a_exp == EXP_MASK || b_exp == EXP_MASK || c_exp == EXP_MASK {
        let a_nan = unsafe { isnan_sf32(a) } != 0;
        let b_nan = unsafe { isnan_sf32(b) } != 0;
        let c_nan = unsafe { isnan_sf32(c) } != 0;
        if a_nan || b_nan || c_nan {
            let a_snan = unsafe { issignan_sf32(a) } != 0;
            let b_snan = unsafe { issignan_sf32(b) } != 0;
            let c_snan = unsafe { issignan_sf32(c) } != 0;
            if a_snan || b_snan || c_snan {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN32;
        } else {
            // infinities
            if (a_exp == EXP_MASK && (b_exp == 0 && b_mant == 0))
                || (b_exp == EXP_MASK && (a_exp == 0 && a_mant == 0))
                || ((a_exp == EXP_MASK || b_exp == EXP_MASK)
                    && (c_exp == EXP_MASK && r_sign != c_sign))
            {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
                return F_QNAN32;
            } else if c_exp == EXP_MASK {
                return pack_sf32(c_sign, EXP_MASK as u32, 0);
            } else {
                return pack_sf32(r_sign, EXP_MASK as u32, 0);
            }
        }
    }

    // "goto mul_zero" from the a operand is emulated with a flag
    let mut to_mul_zero = false;
    if a_exp == 0 {
        if a_mant == 0 {
            to_mul_zero = true;
        } else {
            a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant);
        }
    } else {
        a_mant |= 1u32 << 23;
    }
    if !to_mul_zero {
        if b_exp == 0 {
            if b_mant == 0 {
                // mul_zero:
                to_mul_zero = true;
            } else {
                b_mant = normalize_subnormal_sf32(&mut b_exp, b_mant);
            }
        } else {
            b_mant |= 1u32 << 23;
        }
    }
    if to_mul_zero {
        if c_exp == 0 && c_mant == 0 {
            if c_sign != r_sign {
                r_sign = (rm == RM_RDN) as u32;
            }
            return pack_sf32(r_sign, 0, 0);
        } else {
            return c;
        }
    }

    // multiply
    r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3;

    r_mant1 = mul_u32(
        &mut r_mant0,
        a_mant.wrapping_shl(((32 - 2) - 23) as u32),
        b_mant.wrapping_shl(((32 - 2) - 23) as u32),
    );
    // normalize to F_SIZE - 3
    if r_mant1 < (1u32 << (32 - 3)) {
        r_mant1 = (r_mant1.wrapping_shl(1)) | (r_mant0.wrapping_shr(32 - 1));
        r_mant0 = r_mant0.wrapping_shl(1);
        r_exp -= 1;
    }

    // add
    if c_exp == 0 {
        if c_mant == 0 {
            // add zero
            r_mant1 |= (r_mant0 != 0) as u32;
            return normalize_sf32(r_sign, r_exp, r_mant1, rm, pfflags);
        }
        c_mant = normalize_subnormal_sf32(&mut c_exp, c_mant);
    } else {
        c_mant |= 1u32 << 23;
    }
    c_exp += 1;
    c_mant1 = c_mant.wrapping_shl((((32 - 2) - 23) - 1) as u32);
    c_mant0 = 0;

    // ensure that abs(r) >= abs(c)
    if !(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)) {
        // swap
        let tmp_m: u32 = r_mant1;
        r_mant1 = c_mant1;
        c_mant1 = tmp_m;
        let tmp_m0: u32 = r_mant0;
        r_mant0 = c_mant0;
        c_mant0 = tmp_m0;
        let c_tmp: i32 = r_exp;
        r_exp = c_exp;
        c_exp = c_tmp;
        let c_tmp_s: u32 = r_sign;
        r_sign = c_sign;
        c_sign = c_tmp_s;
    }
    // right shift c_mant
    shift = r_exp - c_exp;
    if shift >= 2 * 32 {
        c_mant0 = ((c_mant0 | c_mant1) != 0) as u32;
        c_mant1 = 0;
    } else if shift >= 32 + 1 {
        c_mant0 = rshift_rnd32(c_mant1, shift - 32);
        c_mant1 = 0;
    } else if shift == 32 {
        c_mant0 = c_mant1 | ((c_mant0 != 0) as u32);
        c_mant1 = 0;
    } else if shift != 0 {
        if shift > 0 {
            mask = (1u32.wrapping_shl(shift as u32)) - 1;
            c_mant0 = c_mant1.wrapping_shl((32 - shift) as u32)
                | c_mant0.wrapping_shr(shift as u32)
                | ((c_mant0 & mask) != 0) as u32;
            c_mant1 = c_mant1.wrapping_shr(shift as u32);
        }
    }

    // add or subtract
    if r_sign == c_sign {
        let old0 = r_mant0;
        let _ = old0;
        r_mant0 = r_mant0.wrapping_add(c_mant0);
        r_mant1 = r_mant1
            .wrapping_add(c_mant1)
            .wrapping_add((r_mant0 < c_mant0) as u32);
    } else {
        let tmp = r_mant0;
        r_mant0 = r_mant0.wrapping_sub(c_mant0);
        r_mant1 = r_mant1
            .wrapping_sub(c_mant1)
            .wrapping_sub((r_mant0 > tmp) as u32);
        if (r_mant0 | r_mant1) == 0 {
            // zero result: the sign needs a specific handling
            r_sign = (rm == RM_RDN) as u32;
        }
    }
    normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags)
}
}

// ── `fma_sf64` · softfp_template.h:402-552 · chunk 3086 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t fma_sf64(uint64_t a, uint64_t b, uint64_t c, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint64_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         c_sign = c >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         c_exp = (c >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
//         c_mant = c & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1) || c_exp == ((1 << 11) - 1)) {
//             if (isnan_sf64(a) || isnan_sf64(b) || isnan_sf64(c)) {
//                 if (issignan_sf64(a) || issignan_sf64(b) || issignan_sf64(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 if ((a_exp == ((1 << 11) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 11) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)) &&
//                      (c_exp == ((1 << 11) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else if (c_exp == ((1 << 11) - 1)) {
//                     return pack_sf64(c_sign, ((1 << 11) - 1), 0);
//                 } else {
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf64(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
//     
//         r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3;
//     
//         r_mant1 = mul_u64(&r_mant0, a_mant << ((64 - 2) - 52), b_mant << ((64 - 2) - 52));
//     
//         if (r_mant1 < ((uint64_t)1 << (64 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (64 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
//     
//     
//         if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf64(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf64(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint64_t)1 << 52;
//         }
//         c_exp++;
//         c_mant1 = c_mant << (((64 - 2) - 52) - 1);
//         c_mant0 = 0;
//     
//     
//     
//     
//     
//         if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint64_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
//     
//         shift = r_exp - c_exp;
//         if (shift >= 2 * 64) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 64 + 1) {
//             c_mant0 = rshift_rnd64(c_mant1, shift - 64);
//             c_mant1 = 0;
//         } else if (shift == 64) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint64_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (64 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
//     
//     
//     
//         if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint64_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
//         return normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3086 {
// Port of softfp_template.h fma_sf64 (double precision fused multiply-add).
// Static helpers from the same translation unit (pack_sf64, rshift_rnd64,
// roundpack_sf64, normalize_sf64, normalize2_sf64, normalize_subnormal_sf64,
// mul_u64, clz64) are reimplemented here; isnan_sf64/issignan_sf64 are
// external (non-static) C functions and are only declared.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RMM: c_int = 4;

const F_QNAN64: u64 = 0x7ff8_0000_0000_0000u64;

extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d > 0 {
        if d >= 64 {
            (a != 0) as u64
        } else {
            let du = d as u32;
            let mask = (1u64 << du) - 1;
            (a >> du) | ((a & mask) != 0) as u64
        }
    } else {
        a
    }
}

fn roundpack_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: c_int, pfflags: &mut u32) -> u64 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1 << 9,
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1 << 10) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1 << 0;
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    if rm == RM_RNE && rnd_bits == (1 << 9) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> 53) as i32;
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= 2047 {
        if addend == 0 {
            a_exp = 2047 - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = 2047;
            a_mant = 0;
        }
        *pfflags |= (1 << 2) | (1 << 0);
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: c_int, pfflags: &mut u32) -> u64 {
    let shift = clz64(a_mant) - (64 - 1 - (64 - 2));
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize2_sf64(
    a_sign: u32,
    a_exp: i32,
    a_mant1: u64,
    a_mant0: u64,
    rm: c_int,
    pfflags: &mut u32,
) -> u64 {
    let mut a_mant1 = a_mant1;
    let mut a_mant0 = a_mant0;
    let l = if a_mant1 == 0 {
        64 + clz64(a_mant0)
    } else {
        clz64(a_mant1)
    };
    let shift = l - (64 - 1 - (64 - 2));
    let a_exp = a_exp - shift;
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u64;
    } else if shift < 64 {
        a_mant1 = a_mant1.wrapping_shl(shift as u32) | a_mant0.wrapping_shr((64 - shift) as u32);
        a_mant0 = a_mant0.wrapping_shl(shift as u32);
        a_mant1 |= (a_mant0 != 0) as u64;
    } else {
        a_mant1 = a_mant0.wrapping_shl((shift - 64) as u32);
    }
    roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags)
}

fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift = 52 - (64 - 1 - clz64(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn mul_u64(plow: &mut u64, a: u64, b: u64) -> u64 {
    let r = (a as u128) * (b as u128);
    *plow = r as u64;
    (r >> 64) as u64
}

#[no_mangle]
pub extern "C" fn fma_sf64(a: u64, b: u64, c: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    unsafe {
        let flags: &mut u32 = &mut *pfflags;

        let a_sign = (a >> 63) as u32;
        let b_sign = (b >> 63) as u32;
        let mut c_sign = (c >> 63) as u32;
        let mut r_sign = a_sign ^ b_sign;
        let mut a_exp = ((a >> 52) & 0x7ff) as i32;
        let mut b_exp = ((b >> 52) & 0x7ff) as i32;
        let mut c_exp = ((c >> 52) & 0x7ff) as i32;
        let mut a_mant = a & ((1u64 << 52) - 1);
        let mut b_mant = b & ((1u64 << 52) - 1);
        let mut c_mant = c & ((1u64 << 52) - 1);

        if a_exp == 2047 || b_exp == 2047 || c_exp == 2047 {
            if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 || isnan_sf64(c) != 0 {
                if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 || issignan_sf64(c) != 0 {
                    *flags |= 1 << 4;
                }
                return F_QNAN64;
            } else {
                // infinities
                if (a_exp == 2047 && (b_exp == 0 && b_mant == 0))
                    || (b_exp == 2047 && (a_exp == 0 && a_mant == 0))
                    || ((a_exp == 2047 || b_exp == 2047)
                        && (c_exp == 2047 && r_sign != c_sign))
                {
                    *flags |= 1 << 4;
                    return F_QNAN64;
                } else if c_exp == 2047 {
                    return pack_sf64(c_sign, 2047, 0);
                } else {
                    return pack_sf64(r_sign, 2047, 0);
                }
            }
        }

        let mut zero_mul = false;
        if a_exp == 0 {
            if a_mant == 0 {
                zero_mul = true; // goto mul_zero
            } else {
                a_mant = normalize_subnormal_sf64(&mut a_exp, a_mant);
            }
        } else {
            a_mant |= 1u64 << 52;
        }
        if !zero_mul {
            if b_exp == 0 {
                if b_mant == 0 {
                    zero_mul = true; // goto mul_zero
                } else {
                    b_mant = normalize_subnormal_sf64(&mut b_exp, b_mant);
                }
            } else {
                b_mant |= 1u64 << 52;
            }
        }
        if zero_mul {
            // mul_zero:
            if c_exp == 0 && c_mant == 0 {
                if c_sign != r_sign {
                    r_sign = if rm == RM_RDN { 1 } else { 0 };
                }
                return pack_sf64(r_sign, 0, 0);
            } else {
                return c;
            }
        }

        // multiply
        let mut r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3;

        let mut r_mant0: u64 = 0;
        let mut r_mant1 = mul_u64(
            &mut r_mant0,
            a_mant.wrapping_shl(((64 - 2) - 52) as u32),
            b_mant.wrapping_shl(((64 - 2) - 52) as u32),
        );
        // normalize to F_SIZE - 3
        if r_mant1 < (1u64 << (64 - 3)) {
            r_mant1 = (r_mant1 << 1) | (r_mant0 >> (64 - 1));
            r_mant0 <<= 1;
            r_exp -= 1;
        }

        // add
        if c_exp == 0 {
            if c_mant == 0 {
                // add zero
                r_mant1 |= (r_mant0 != 0) as u64;
                return normalize_sf64(r_sign, r_exp, r_mant1, rm, flags);
            }
            c_mant = normalize_subnormal_sf64(&mut c_exp, c_mant);
        } else {
            c_mant |= 1u64 << 52;
        }
        c_exp += 1;
        let mut c_mant1 = c_mant.wrapping_shl((((64 - 2) - 52) - 1) as u32);
        let mut c_mant0: u64 = 0;

        // ensure that abs(r) >= abs(c)
        if !(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)) {
            core::mem::swap(&mut r_mant1, &mut c_mant1);
            core::mem::swap(&mut r_mant0, &mut c_mant0);
            core::mem::swap(&mut r_exp, &mut c_exp);
            core::mem::swap(&mut r_sign, &mut c_sign);
        }
        // right shift c_mant
        let shift = r_exp - c_exp;
        if shift >= 2 * 64 {
            c_mant0 = ((c_mant0 | c_mant1) != 0) as u64;
            c_mant1 = 0;
        } else if shift >= 64 + 1 {
            c_mant0 = rshift_rnd64(c_mant1, shift - 64);
            c_mant1 = 0;
        } else if shift == 64 {
            c_mant0 = c_mant1 | ((c_mant0 != 0) as u64);
            c_mant1 = 0;
        } else if shift != 0 {
            let mask = (1u64 << shift) - 1;
            c_mant0 = (c_mant1.wrapping_shl((64 - shift) as u32))
                | (c_mant0.wrapping_shr(shift as u32))
                | ((c_mant0 & mask) != 0) as u64;
            c_mant1 = c_mant1.wrapping_shr(shift as u32);
        }

        // add or subtract
        if r_sign == c_sign {
            r_mant0 = r_mant0.wrapping_add(c_mant0);
            r_mant1 = r_mant1
                .wrapping_add(c_mant1)
                .wrapping_add((r_mant0 < c_mant0) as u64);
        } else {
            let tmp = r_mant0;
            r_mant0 = r_mant0.wrapping_sub(c_mant0);
            r_mant1 = r_mant1
                .wrapping_sub(c_mant1)
                .wrapping_sub((r_mant0 > tmp) as u64);
            if (r_mant0 | r_mant1) == 0 {
                // zero result : the sign needs a specific handling
                r_sign = if rm == RM_RDN { 1 } else { 0 };
            }
        }
        normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, flags)
    }
}
}

// ── `fma_sf128` · softfp_template.h:402-552 · chunk 3208 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t fma_sf128(uint128_t a, uint128_t b, uint128_t c, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint128_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         c_sign = c >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         c_exp = (c >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
//         c_mant = c & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1) || c_exp == ((1 << 15) - 1)) {
//             if (isnan_sf128(a) || isnan_sf128(b) || isnan_sf128(c)) {
//                 if (issignan_sf128(a) || issignan_sf128(b) || issignan_sf128(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 if ((a_exp == ((1 << 15) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 15) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)) &&
//                      (c_exp == ((1 << 15) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else if (c_exp == ((1 << 15) - 1)) {
//                     return pack_sf128(c_sign, ((1 << 15) - 1), 0);
//                 } else {
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf128(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
//     
//         r_exp = a_exp + b_exp - (1 << (15 - 1)) + 3;
//     
//         r_mant1 = mul_u128(&r_mant0, a_mant << ((128 - 2) - 112), b_mant << ((128 - 2) - 112));
//     
//         if (r_mant1 < ((uint128_t)1 << (128 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (128 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
//     
//     
//         if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf128(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf128(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint128_t)1 << 112;
//         }
//         c_exp++;
//         c_mant1 = c_mant << (((128 - 2) - 112) - 1);
//         c_mant0 = 0;
//     
//     
//     
//     
//     
//         if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint128_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
//     
//         shift = r_exp - c_exp;
//         if (shift >= 2 * 128) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 128 + 1) {
//             c_mant0 = rshift_rnd128(c_mant1, shift - 128);
//             c_mant1 = 0;
//         } else if (shift == 128) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint128_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (128 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
//     
//     
//     
//         if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint128_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
//         return normalize2_sf128(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3208 {
// Port of fma_sf128() from softfp_template.h (F_SIZE == 128 instantiation).
#![allow(dead_code)]

type RoundingModeEnum = u32;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RDN: RoundingModeEnum = 2;
const RM_RUP: RoundingModeEnum = 3;
const RM_RMM: RoundingModeEnum = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const EXP_MASK_I: i32 = (1 << 15) - 1;
const EXP_MASK_U: u32 = (1 << 15) - 1;
const MANT_MASK: u128 = ((1u128) << 112) - 1;
const RND_MASK: u128 = (1u128 << 14) - 1;
const F_QNAN128: u128 = ((EXP_MASK_U as u128) << 112) | (1u128 << 111);

#[inline]
fn clz128(a: u128) -> i32 {
    // returns 128 for a == 0, like the C helper
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

#[inline]
fn isnan_sf128(a: u128) -> bool {
    let a_exp: u32 = ((a >> 112) & (EXP_MASK_U as u128)) as u32;
    let a_mant: u128 = a & MANT_MASK;
    a_exp == EXP_MASK_U && a_mant != 0
}

#[inline]
fn issignan_sf128(a: u128) -> bool {
    let a_exp1: u32 = ((a >> 111) & ((1u128 << 16) - 1)) as u32;
    let a_mant: u128 = a & MANT_MASK;
    a_exp1 == 2 * EXP_MASK_U && a_mant != 0
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            if a != 0 {
                1
            } else {
                0
            }
        } else if d > 0 {
            let mask: u128 = ((1u128) << (d as u32)) - 1;
            (a >> (d as u32)) | if (a & mask) != 0 { 1 } else { 0 }
        } else {
            a
        }
    } else {
        a
    }
}

fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;
    match rm {
        RM_RNE | RM_RMM => {
            addend = 1 << 13;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1 << 14) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant.wrapping_add(addend as u128)) < ((1u128) << 127);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & RND_MASK) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & RND_MASK) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    if rm == RM_RNE && rnd_bits == (1 << 13) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> 113) as i32;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MASK_I {
        if addend == 0 {
            a_exp = EXP_MASK_I - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK_I;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant) - 1;
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize2_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant1: u128,
    mut a_mant0: u128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u128 {
    let l: i32 = if a_mant1 == 0 {
        128 + clz128(a_mant0)
    } else {
        clz128(a_mant1)
    };
    let shift: i32 = l - 1;
    a_exp -= shift;
    if shift == 0 {
        a_mant1 |= if a_mant0 != 0 { 1 } else { 0 };
    } else if shift < 128 {
        a_mant1 = a_mant1.wrapping_shl(shift as u32) | a_mant0.wrapping_shr((128 - shift) as u32);
        a_mant0 = a_mant0.wrapping_shl(shift as u32);
        a_mant1 |= if a_mant0 != 0 { 1 } else { 0 };
    } else {
        a_mant1 = a_mant0.wrapping_shl((shift - 128) as u32);
    }
    roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags)
}

#[inline]
fn normalize_subnormal_sf128(pa_exp: &mut i32, a_mant: u128) -> u128 {
    let shift: i32 = 112 - (127 - clz128(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn mul_u128(plow: &mut u128, a: u128, b: u128) -> u128 {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: u128 = (a0 as u128).wrapping_mul(b0 as u128);
    let r01: u128 = (a0 as u128).wrapping_mul(b1 as u128);
    let r10: u128 = (a1 as u128).wrapping_mul(b0 as u128);
    let r11: u128 = (a1 as u128).wrapping_mul(b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> 64) + ((r01 as u64) as u128) + ((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> 64) + (r01 >> 64) + (r10 >> 64) + ((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64) + (r11 >> 64)) as u64;

    *plow = ((r1 as u128) << 64) | (r0 as u128);
    ((r3 as u128) << 64) | (r2 as u128)
}

// code shared by the "goto mul_zero" paths
fn mul_zero_result(
    c: u128,
    c_exp: i32,
    c_mant: u128,
    c_sign: u32,
    r_sign_in: u32,
    rm: RoundingModeEnum,
) -> u128 {
    if c_exp == 0 && c_mant == 0 {
        let mut r_sign: u32 = r_sign_in;
        if c_sign != r_sign {
            r_sign = if rm == RM_RDN { 1 } else { 0 };
        }
        pack_sf128(r_sign, 0, 0)
    } else {
        c
    }
}

#[no_mangle]
pub extern "C" fn fma_sf128(
    a: u128,
    b: u128,
    c: u128,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u128 {
    let a_sign: u32;
    let b_sign: u32;
    let mut c_sign: u32;
    let mut r_sign: u32;
    let mut a_exp: i32;
    let mut b_exp: i32;
    let mut c_exp: i32;
    let mut r_exp: i32;
    let shift: i32;
    let mut a_mant: u128;
    let mut b_mant: u128;
    let mut c_mant: u128;
    let mut r_mant1: u128;
    let mut r_mant0: u128;
    let mut c_mant1: u128;
    let mut c_mant0: u128;

    a_sign = (a >> 127) as u32;
    b_sign = (b >> 127) as u32;
    c_sign = (c >> 127) as u32;
    r_sign = a_sign ^ b_sign;
    a_exp = ((a >> 112) & (EXP_MASK_U as u128)) as i32;
    b_exp = ((b >> 112) & (EXP_MASK_U as u128)) as i32;
    c_exp = ((c >> 112) & (EXP_MASK_U as u128)) as i32;
    a_mant = a & MANT_MASK;
    b_mant = b & MANT_MASK;
    c_mant = c & MANT_MASK;
    if a_exp == EXP_MASK_I || b_exp == EXP_MASK_I || c_exp == EXP_MASK_I {
        if isnan_sf128(a) || isnan_sf128(b) || isnan_sf128(c) {
            if issignan_sf128(a) || issignan_sf128(b) || issignan_sf128(c) {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN128;
        } else {
            // infinities
            if (a_exp == EXP_MASK_I && (b_exp == 0 && b_mant == 0))
                || (b_exp == EXP_MASK_I && (a_exp == 0 && a_mant == 0))
                || ((a_exp == EXP_MASK_I || b_exp == EXP_MASK_I)
                    && (c_exp == EXP_MASK_I && r_sign != c_sign))
            {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
                return F_QNAN128;
            } else if c_exp == EXP_MASK_I {
                return pack_sf128(c_sign, EXP_MASK_U, 0);
            } else {
                return pack_sf128(r_sign, EXP_MASK_U, 0);
            }
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            // goto mul_zero
            return mul_zero_result(c, c_exp, c_mant, c_sign, r_sign, rm);
        }
        a_mant = normalize_subnormal_sf128(&mut a_exp, a_mant);
    } else {
        a_mant |= (1u128) << 112;
    }
    if b_exp == 0 {
        if b_mant == 0 {
            // mul_zero:
            return mul_zero_result(c, c_exp, c_mant, c_sign, r_sign, rm);
        }
        b_mant = normalize_subnormal_sf128(&mut b_exp, b_mant);
    } else {
        b_mant |= (1u128) << 112;
    }

    // multiply
    r_exp = a_exp + b_exp - (1 << (15 - 1)) + 3;

    r_mant0 = 0;
    r_mant1 = mul_u128(&mut r_mant0, a_mant << 14, b_mant << 14);

    // normalize to F_SIZE - 3
    if r_mant1 < ((1u128) << 125) {
        r_mant1 = (r_mant1 << 1) | (r_mant0 >> 127);
        r_mant0 <<= 1;
        r_exp -= 1;
    }

    // add
    if c_exp == 0 {
        if c_mant == 0 {
            // add zero
            r_mant1 |= if r_mant0 != 0 { 1 } else { 0 };
            return normalize_sf128(r_sign, r_exp, r_mant1, rm, pfflags);
        }
        c_mant = normalize_subnormal_sf128(&mut c_exp, c_mant);
    } else {
        c_mant |= (1u128) << 112;
    }
    c_exp += 1;
    c_mant1 = c_mant << 13;
    c_mant0 = 0;

    // ensure that abs(r) >= abs(c)
    if !(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)) {
        core::mem::swap(&mut r_mant1, &mut c_mant1);
        core::mem::swap(&mut r_mant0, &mut c_mant0);
        core::mem::swap(&mut r_exp, &mut c_exp);
        core::mem::swap(&mut r_sign, &mut c_sign);
    }

    // right shift c_mant
    shift = r_exp - c_exp;
    if shift >= 2 * 128 {
        c_mant0 = if (c_mant0 | c_mant1) != 0 { 1 } else { 0 };
        c_mant1 = 0;
    } else if shift >= 128 + 1 {
        c_mant0 = rshift_rnd128(c_mant1, shift - 128);
        c_mant1 = 0;
    } else if shift == 128 {
        c_mant0 = c_mant1 | (if c_mant0 != 0 { 1 } else { 0 });
        c_mant1 = 0;
    } else if shift != 0 {
        let mask: u128 = if shift > 0 && shift < 128 {
            ((1u128) << (shift as u32)) - 1
        } else {
            u128::MAX
        };
        c_mant0 = c_mant1.wrapping_shl((128 - shift) as u32)
            | c_mant0.wrapping_shr(shift as u32)
            | (if (c_mant0 & mask) != 0 { 1 } else { 0 });
        c_mant1 = c_mant1.wrapping_shr(shift as u32);
    }

    // add or subtract
    if r_sign == c_sign {
        r_mant0 = r_mant0.wrapping_add(c_mant0);
        r_mant1 = r_mant1
            .wrapping_add(c_mant1)
            .wrapping_add(if r_mant0 < c_mant0 { 1 } else { 0 });
    } else {
        let tmp: u128 = r_mant0;
        r_mant0 = r_mant0.wrapping_sub(c_mant0);
        r_mant1 = r_mant1
            .wrapping_sub(c_mant1)
            .wrapping_sub(if r_mant0 > tmp { 1 } else { 0 });
        if (r_mant0 | r_mant1) == 0 {
            // zero result: the sign needs a specific handling
            r_sign = if rm == RM_RDN { 1 } else { 0 };
        }
    }
    normalize2_sf128(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags)
}
}

// ── `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint32_t a_…` · softfp_template.h:405-418 · chunk 2975 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint32_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         c_sign = c >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         c_exp = (c >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
//         c_mant = c & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2975 {
// Ported piece of fma_sf32: sign/exponent/mantissa extraction.
#[no_mangle]
pub extern "C" fn fma_sf32__part2975(
    a: u32,
    b: u32,
    c: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
    c_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    c_exp: *mut i32,
    a_mant: *mut u32,
    b_mant: *mut u32,
    c_mant: *mut u32,
) {
    unsafe {
        let sa: u32 = a >> 31;
        let sb: u32 = b >> 31;
        let sc: u32 = c >> 31;
        let sr: u32 = sa ^ sb;
        let ea: u32 = (a >> 23) & 255u32;
        let eb: u32 = (b >> 23) & 255u32;
        let ec: u32 = (c >> 23) & 255u32;
        let ma: u32 = a & ((1u32 << 23) - 1u32);
        let mb: u32 = b & ((1u32 << 23) - 1u32);
        let mc: u32 = c & ((1u32 << 23) - 1u32);

        *a_sign = sa;
        *b_sign = sb;
        *c_sign = sc;
        *r_sign = sr;
        *a_exp = ea as i32;
        *b_exp = eb as i32;
        *c_exp = ec as i32;
        *a_mant = ma;
        *b_mant = mb;
        *c_mant = mc;
    }
}
}

// ── `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint64_t a_…` · softfp_template.h:405-418 · chunk 3087 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint64_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         c_sign = c >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         c_exp = (c >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
//         c_mant = c & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3087 {
// Port of the declaration/initialisation block at the start of fma_sf64.
// Extracted field unpacking: sign bits, biased exponents and mantissas.

#[no_mangle]
pub extern "C" fn fma_sf64__part3087(
    a: u64,
    b: u64,
    c: u64,
    a_sign: *mut u32,
    b_sign: *mut u32,
    c_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    c_exp: *mut i32,
    a_mant: *mut u64,
    b_mant: *mut u64,
    c_mant: *mut u64,
) {
    // uint32_t x_sign = x >> (64 - 1);
    let sa: u32 = (a >> 63) as u32;
    let sb: u32 = (b >> 63) as u32;
    let sc: u32 = (c >> 63) as u32;
    let sr: u32 = sa ^ sb;

    // int32_t x_exp = (x >> 52) & ((1 << 11) - 1);
    let ea: i32 = ((a >> 52) & 0x7ffu64) as i32;
    let eb: i32 = ((b >> 52) & 0x7ffu64) as i32;
    let ec: i32 = ((c >> 52) & 0x7ffu64) as i32;

    // uint64_t x_mant = x & (((uint64_t)1 << 52) - 1);
    let ma: u64 = a & ((1u64 << 52) - 1);
    let mb: u64 = b & ((1u64 << 52) - 1);
    let mc: u64 = c & ((1u64 << 52) - 1);

    unsafe {
        *a_sign = sa;
        *b_sign = sb;
        *c_sign = sc;
        *r_sign = sr;
        *a_exp = ea;
        *b_exp = eb;
        *c_exp = ec;
        *a_mant = ma;
        *b_mant = mb;
        *c_mant = mc;
    }
}
}

// ── `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint128_t a…` · softfp_template.h:405-418 · chunk 3209 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, c_sign, r_sign;
//         int32_t a_exp, b_exp, c_exp, r_exp, shift;
//         uint128_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         c_sign = c >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         c_exp = (c >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
//         c_mant = c & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3209 {
// Initial unpacking piece of fma_sf128 (softfp_template.h lines 405-418).
// Reads a, b, c (unsigned __int128 = u128) and writes the unpacked
// sign/exponent/mantissa fields through the supplied pointers.

#[no_mangle]
pub extern "C" fn fma_sf128__part3209(
    a: u128,
    b: u128,
    c: u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
    c_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    c_exp: *mut i32,
    a_mant: *mut u128,
    b_mant: *mut u128,
    c_mant: *mut u128,
) {
    // a_sign = a >> (128 - 1);
    let asg: u32 = (a >> 127) as u32;
    let bsg: u32 = (b >> 127) as u32;
    let csg: u32 = (c >> 127) as u32;
    let rsg: u32 = asg ^ bsg;

    // (a >> 112) & ((1 << 15) - 1)
    let aex: i32 = ((a >> 112) & 32767u128) as i32;
    let bex: i32 = ((b >> 112) & 32767u128) as i32;
    let cex: i32 = ((c >> 112) & 32767u128) as i32;

    // a & (((uint128_t)1 << 112) - 1)
    let am: u128 = a & ((1u128 << 112) - 1);
    let bm: u128 = b & ((1u128 << 112) - 1);
    let cm: u128 = c & ((1u128 << 112) - 1);

    unsafe {
        *a_sign = asg;
        *b_sign = bsg;
        *c_sign = csg;
        *r_sign = rsg;
        *a_exp = aex;
        *b_exp = bex;
        *c_exp = cex;
        *a_mant = am;
        *b_mant = bm;
        *c_mant = cm;
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1) || c_exp == ((1 << 8) - 1)` · softfp_template.h:419-439 · chunk 2976 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1) || c_exp == ((1 << 8) - 1)) {
//             if (isnan_sf32(a) || isnan_sf32(b) || isnan_sf32(c)) {
//                 if (issignan_sf32(a) || issignan_sf32(b) || issignan_sf32(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 if ((a_exp == ((1 << 8) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 8) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)) &&
//                      (c_exp == ((1 << 8) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else if (c_exp == ((1 << 8) - 1)) {
//                     return pack_sf32(c_sign, ((1 << 8) - 1), 0);
//                 } else {
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2976 {
// Ported piece of fma_sf32: special-value (inf/nan) handling block.

use core::ffi::c_int;

const F_QNAN32: u32 = ((0xFFu32) << 23) | (1u32 << (23 - 1));

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// static/inline helper of the C TU: reimplemented privately.
#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn fma_sf32__part2976(
    a: u32,
    b: u32,
    c: u32,
    a_exp: i32,
    b_exp: i32,
    c_exp: i32,
    a_mant: u32,
    b_mant: u32,
    c_mant: u32,
    r_sign: u32,
    c_sign: u32,
    pfflags: *mut u32,
    pret: *mut u32,
) -> c_int {
    let _ = c_mant;
    let max_exp: i32 = (1 << 8) - 1;
    if a_exp == max_exp || b_exp == max_exp || c_exp == max_exp {
        if isnan_sf32(a) || isnan_sf32(b) || isnan_sf32(c) {
            if issignan_sf32(a) || issignan_sf32(b) || issignan_sf32(c) {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            unsafe {
                *pret = F_QNAN32;
            }
            return 1;
        } else {
            if (a_exp == max_exp && (b_exp == 0 && b_mant == 0))
                || (b_exp == max_exp && (a_exp == 0 && a_mant == 0))
                || ((a_exp == max_exp || b_exp == max_exp)
                    && (c_exp == max_exp && r_sign != c_sign))
            {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                unsafe {
                    *pret = F_QNAN32;
                }
                return 1;
            } else if c_exp == max_exp {
                unsafe {
                    *pret = pack_sf32(c_sign, max_exp as u32, 0);
                }
                return 1;
            } else {
                unsafe {
                    *pret = pack_sf32(r_sign, max_exp as u32, 0);
                }
                return 1;
            }
        }
    }
    0
}
}

// ── `if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1) || c_exp == ((1 << 11) - 1)` · softfp_template.h:419-439 · chunk 3088 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1) || c_exp == ((1 << 11) - 1)) {
//             if (isnan_sf64(a) || isnan_sf64(b) || isnan_sf64(c)) {
//                 if (issignan_sf64(a) || issignan_sf64(b) || issignan_sf64(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 if ((a_exp == ((1 << 11) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 11) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)) &&
//                      (c_exp == ((1 << 11) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else if (c_exp == ((1 << 11) - 1)) {
//                     return pack_sf64(c_sign, ((1 << 11) - 1), 0);
//                 } else {
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3088 {
use core::ffi::c_int;

extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

// static const uint64_t F_QNAN64 (has no linker symbol: value replicated here)
const F_QNAN64: u64 = (((1u64 << 11) - 1) << 52) | (1u64 << (52 - 1));

// static inline uint64_t pack_sf64(...): logic replicated privately
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

const EXP_MAX: i32 = (1 << 11) - 1;

#[no_mangle]
pub extern "C" fn fma_sf64__part3088(
    a: u64,
    b: u64,
    c: u64,
    a_exp: i32,
    b_exp: i32,
    c_exp: i32,
    a_mant: u64,
    b_mant: u64,
    c_mant: u64,
    r_sign: u32,
    c_sign: u32,
    pfflags: *mut u32,
    ptaken: *mut c_int,
) -> u64 {
    unsafe {
        if a_exp == EXP_MAX || b_exp == EXP_MAX || c_exp == EXP_MAX {
            if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 || isnan_sf64(c) != 0 {
                if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 || issignan_sf64(c) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *ptaken = 1;
                return F_QNAN64;
            } else {
                if (a_exp == EXP_MAX && (b_exp == 0 && b_mant == 0))
                    || (b_exp == EXP_MAX && (a_exp == 0 && a_mant == 0))
                    || ((a_exp == EXP_MAX || b_exp == EXP_MAX)
                        && (c_exp == EXP_MAX && r_sign != c_sign))
                {
                    *pfflags |= 1u32 << 4;
                    *ptaken = 1;
                    return F_QNAN64;
                } else if c_exp == EXP_MAX {
                    *ptaken = 1;
                    return pack_sf64(c_sign, EXP_MAX as u32, 0);
                } else {
                    *ptaken = 1;
                    return pack_sf64(r_sign, EXP_MAX as u32, 0);
                }
            }
        }
        *ptaken = 0;
    }
    0
}
}

// ── `if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1) || c_exp == ((1 << 15) - 1)` · softfp_template.h:419-439 · chunk 3210 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1) || c_exp == ((1 << 15) - 1)) {
//             if (isnan_sf128(a) || isnan_sf128(b) || isnan_sf128(c)) {
//                 if (issignan_sf128(a) || issignan_sf128(b) || issignan_sf128(c)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 if ((a_exp == ((1 << 15) - 1) && (b_exp == 0 && b_mant == 0)) ||
//                     (b_exp == ((1 << 15) - 1) && (a_exp == 0 && a_mant == 0)) ||
//                     ((a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)) &&
//                      (c_exp == ((1 << 15) - 1) && r_sign != c_sign))) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else if (c_exp == ((1 << 15) - 1)) {
//                     return pack_sf128(c_sign, ((1 << 15) - 1), 0);
//                 } else {
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3210 {
// Port of the NaN/Infinity special-case block of fma_sf128 (softfp_template.h 419-439)

const EXP_MASK: i32 = (1 << 15) - 1;
const MANT_MASK: u128 = (1u128 << 112) - 1;
// static const uint128_t F_QNAN128
const F_QNAN128: u128 = (((EXP_MASK as u32) as u128) << 112) | (1u128 << (112 - 1));

// static inline helper: isnan_sf128
fn isnan_sf128(a: u128) -> bool {
    let a_exp = ((a >> 112) & (EXP_MASK as u128)) as i32;
    let a_mant = a & MANT_MASK;
    a_exp == EXP_MASK && a_mant != 0
}

// issignan_sf128
fn issignan_sf128(a: u128) -> bool {
    let a_exp1 = ((a >> (112 - 1)) & ((1u128 << (15 + 1)) - 1)) as u32;
    let a_mant = a & MANT_MASK;
    a_exp1 == (2 * (EXP_MASK as u32)) && a_mant != 0
}

// static inline helper: pack_sf128
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

/// Returns the value the C code should `return`, and sets *ptaken to 1 when the
/// surrounding function must return it (0 means "continue with the main path").
#[no_mangle]
pub unsafe extern "C" fn fma_sf128__part3210(
    a: u128,
    b: u128,
    c: u128,
    pfflags: *mut u32,
    ptaken: *mut i32,
) -> u128 {
    let a_sign: u32 = (a >> (128 - 1)) as u32;
    let b_sign: u32 = (b >> (128 - 1)) as u32;
    let c_sign: u32 = (c >> (128 - 1)) as u32;
    let r_sign: u32 = a_sign ^ b_sign;
    let a_exp: i32 = ((a >> 112) & (EXP_MASK as u128)) as i32;
    let b_exp: i32 = ((b >> 112) & (EXP_MASK as u128)) as i32;
    let c_exp: i32 = ((c >> 112) & (EXP_MASK as u128)) as i32;
    let a_mant: u128 = a & MANT_MASK;
    let b_mant: u128 = b & MANT_MASK;
    let c_mant: u128 = c & MANT_MASK;
    let _ = c_mant;

    *ptaken = 0;

    if a_exp == EXP_MASK || b_exp == EXP_MASK || c_exp == EXP_MASK {
        if isnan_sf128(a) || isnan_sf128(b) || isnan_sf128(c) {
            if issignan_sf128(a) || issignan_sf128(b) || issignan_sf128(c) {
                *pfflags |= 1 << 4;
            }
            *ptaken = 1;
            return F_QNAN128;
        } else {
            // infinities
            if (a_exp == EXP_MASK && (b_exp == 0 && b_mant == 0))
                || (b_exp == EXP_MASK && (a_exp == 0 && a_mant == 0))
                || ((a_exp == EXP_MASK || b_exp == EXP_MASK)
                    && (c_exp == EXP_MASK && r_sign != c_sign))
            {
                *pfflags |= 1 << 4;
                *ptaken = 1;
                return F_QNAN128;
            } else if c_exp == EXP_MASK {
                *ptaken = 1;
                return pack_sf128(c_sign, EXP_MASK as u32, 0);
            } else {
                *ptaken = 1;
                return pack_sf128(r_sign, EXP_MASK as u32, 0);
            }
        }
    }

    0
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:440-446 · chunk 2977 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2977 {
// Port of the "normalize a operand" block of fma_sf32.
// Returns 1 when the C code should continue after the block, 0 when the
// caller must execute `goto mul_zero;`.

use core::ffi::{c_int, c_uint};

// static inline int clz32(uint32_t a) -- ported privately, no linker symbol.
#[inline]
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
#[inline]
unsafe fn normalize_subnormal_sf32(pa_exp: *mut c_int, a_mant: u32) -> u32 {
    let shift: c_int = 23 - (32 - 1 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant << (shift as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn fma_sf32__part2977(pa_exp: *mut c_int, pa_mant: *mut c_uint) -> c_int {
    let a_exp: c_int = *pa_exp;
    let a_mant: c_uint = *pa_mant;

    if a_exp == 0 {
        if a_mant == 0 {
            // caller must perform `goto mul_zero;`
            return 0;
        }
        *pa_mant = normalize_subnormal_sf32(pa_exp, a_mant);
    } else {
        *pa_mant = a_mant | (1u32 << 23);
    }
    1
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:440-446 · chunk 3089 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3089 {
// Port of the leading-part of fma_sf64: handle the exponent/mantissa
// unpacking of operand `a` (implicit bit insertion or subnormal
// normalization).  Returns 1 when `a` is zero (the C code then takes the
// `goto mul_zero` path), 0 otherwise.

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// Inlined `static inline normalize_subnormal_sf64` (no linker symbol).
fn normalize_subnormal_sf64(pa_exp: *mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (64 - 1 - clz64(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}

#[no_mangle]
pub extern "C" fn fma_sf64__part3089(pa_exp: *mut i32, pa_mant: *mut u64) -> core::ffi::c_int {
    unsafe {
        let a_exp: i32 = *pa_exp;
        let a_mant: u64 = *pa_mant;
        if a_exp == 0 {
            if a_mant == 0 {
                return 1;
            }
            let m = normalize_subnormal_sf64(pa_exp, a_mant);
            *pa_mant = m;
        } else {
            *pa_mant = a_mant | (1u64 << 52);
        }
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:440-446 · chunk 3211 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 goto mul_zero;
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3211 {
use core::ffi::c_int;

// Port of the C `static inline int clz128(uint128_t a)` helper.
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah: u64 = (a >> 64) as u64;
        let al: u64 = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

// Port of the C `static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)`.
fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: u128) -> u128 {
    let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    // C shifts by a value that is always in [0, 96] here; guard so the Rust
    // shift can never panic on C data.
    let s: u32 = shift as u32;
    if s < 128 {
        a_mant << s
    } else {
        0
    }
}

/// Returns 1 when the caller must take the `goto mul_zero;` path, 0 otherwise.
#[no_mangle]
pub extern "C" fn fma_sf128__part3211(pa_exp: *mut i32, pa_mant: *mut u128) -> c_int {
    unsafe {
        if *pa_exp == 0 {
            if *pa_mant == 0 {
                return 1;
            }
            *pa_mant = normalize_subnormal_sf128(pa_exp, *pa_mant);
        } else {
            *pa_mant |= 1u128 << 112;
        }
    }
    0
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:447-461 · chunk 2978 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf32(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2978 {
// Port of the b-operand normalization / mul_zero part of fma_sf32.
#![allow(non_snake_case)]

use core::ffi::c_int;

const MANT_MASK: u32 = (1u32 << 23) - 1;
const RM_RDN: c_int = 2;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & MANT_MASK)
}

// static inline in C: logic ported here.
#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - ((32 - 1) - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant << shift
}

// mode 0: b_exp != 0 -> set implicit bit
// mode 1: b_exp == 0 && b_mant != 0 -> normalize subnormal
// mode 2: mul_zero label reached (b_exp == 0 && b_mant == 0)
#[no_mangle]
pub extern "C" fn fma_sf32__part2978(
    pb_exp: *mut i32,
    pb_mant: *mut u32,
    pr_sign: *mut u32,
    c_exp: i32,
    c_mant: u32,
    c_sign: u32,
    c: u32,
    rm: c_int,
    mode: c_int,
    pdo_ret: *mut c_int,
) -> u32 {
    unsafe {
        if mode == 0 {
            *pb_mant |= 1u32 << 23;
            0
        } else if mode == 1 {
            let mut e: i32 = *pb_exp;
            let m: u32 = normalize_subnormal_sf32(&mut e, *pb_mant);
            *pb_exp = e;
            *pb_mant = m;
            0
        } else {
            if c_exp == 0 && c_mant == 0 {
                if c_sign != *pr_sign {
                    *pr_sign = if rm == RM_RDN { 1 } else { 0 };
                }
                *pdo_ret = 1;
                pack_sf32(*pr_sign, 0, 0)
            } else {
                *pdo_ret = 1;
                c
            }
        }
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:447-461 · chunk 3090 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf64(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3090 {
// Port of the piece of fma_sf64 that handles the (zero / subnormal) b operand
// and the "mul_zero" early-return path.
//
// The C target contains the label `mul_zero:` which is also entered from an
// earlier `goto mul_zero;`.  The C replacement keeps that label and passes a
// flag telling this function which way it was entered.

use core::ffi::c_int;

const RM_RDN: c_int = 2;

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - ((64 - 1) - clz64(a_mant));
    *pa_exp = 1 - shift;
    a_mant << shift
}

#[no_mangle]
pub extern "C" fn fma_sf64__part3090(
    b_exp_in: i32,
    b_mant_in: u64,
    c: u64,
    c_exp: i32,
    c_mant: u64,
    c_sign: u32,
    r_sign_in: u32,
    rm: c_int,
    from_zero: i32,
    pret: *mut u64,
    pret_flag: *mut i32,
    pb_mant: *mut u64,
    pb_exp: *mut i32,
) {
    let mut b_exp: i32 = b_exp_in;
    let mut b_mant: u64 = b_mant_in;
    let mut r_sign: u32 = r_sign_in;
    let mut ret: u64 = 0;
    let mut ret_flag: i32 = 0;

    if from_zero != 0 {
        // entered at the mul_zero label: the product is zero
        if c_exp == 0 && c_mant == 0 {
            if c_sign != r_sign {
                r_sign = if rm == RM_RDN { 1 } else { 0 };
            }
            ret = pack_sf64(r_sign, 0, 0);
        } else {
            ret = c;
        }
        ret_flag = 1;
    } else if b_exp == 0 {
        // b_mant != 0 here
        b_mant = normalize_subnormal_sf64(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u64 << 52;
    }

    unsafe {
        if !pret.is_null() {
            *pret = ret;
        }
        if !pret_flag.is_null() {
            *pret_flag = ret_flag;
        }
        if !pb_mant.is_null() {
            *pb_mant = b_mant;
        }
        if !pb_exp.is_null() {
            *pb_exp = b_exp;
        }
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:447-461 · chunk 3212 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//             mul_zero:
//                 if (c_exp == 0 && c_mant == 0) {
//                     if (c_sign != r_sign)
//                         r_sign = (rm == RM_RDN);
//                     return pack_sf128(r_sign, 0, 0);
//                 } else {
//                     return c;
//                 }
//             }
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3212 {
// Port of the b-operand / mul_zero handling piece of fma_sf128.
// uint128_t == unsigned __int128 -> u128 ; uint32_t -> u32 ; int32_t -> i32.
// RoundingModeEnum is an int-sized enum: RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4.

type U128 = u128;

const RM_RDN: u32 = 2;

// static inline int clz128(uint128_t a)  -- inlined, no linker symbol
fn clz128(a: U128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)
unsafe fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: U128) -> U128 {
    let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
    if !pa_exp.is_null() {
        *pa_exp = 1i32.wrapping_sub(shift);
    }
    a_mant.wrapping_shl(shift as u32)
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: U128) -> U128 {
    ((a_sign as U128) << (128 - 1)) | ((a_exp as U128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

// The "mul_zero" landing pad: reached either when b_mant == 0 (b_exp == 0) or via
// the function's earlier `goto mul_zero`.
unsafe fn mul_zero_path(
    c_exp: i32,
    c_mant: U128,
    c_sign: u32,
    pr_sign: *mut u32,
    rm: u32,
    c: U128,
    presult: *mut U128,
) -> i32 {
    if c_exp == 0 && c_mant == 0 {
        let mut r_sign: u32 = *pr_sign;
        if c_sign != r_sign {
            r_sign = if rm == RM_RDN { 1 } else { 0 };
        }
        *pr_sign = r_sign;
        *presult = pack_sf128(r_sign, 0, 0);
    } else {
        *presult = c;
    }
    // nonzero status: the C caller must `return *presult;`
    1
}

#[no_mangle]
pub extern "C" fn fma_sf128__part3212(
    pb_exp: *mut i32,
    pb_mant: *mut U128,
    c_exp: i32,
    pc_mant: *const U128,
    c_sign: u32,
    pr_sign: *mut u32,
    rm: u32,
    mul_zero: i32,
    pc: *const U128,
    presult: *mut U128,
) -> i32 {
    unsafe {
        let c_mant: U128 = *pc_mant;
        let c: U128 = *pc;

        if mul_zero == 0 {
            let b_exp: i32 = *pb_exp;
            if b_exp == 0 {
                let b_mant: U128 = *pb_mant;
                if b_mant == 0 {
                    return mul_zero_path(c_exp, c_mant, c_sign, pr_sign, rm, c, presult);
                }
                *pb_mant = normalize_subnormal_sf128(pb_exp, b_mant);
            } else {
                *pb_mant = *pb_mant | (1u128 << 112);
            }
            // zero status: fall through in the C caller
            return 0;
        }

        mul_zero_path(c_exp, c_mant, c_sign, pr_sign, rm, c, presult)
    }
}
}

// ── `r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3; r_mant1 = mul_u32(&r_mant0, a_mant << ((32 - 2) - 2…` · softfp_template.h:463-465 · chunk 2979 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3;
//     
//         r_mant1 = mul_u32(&r_mant0, a_mant << ((32 - 2) - 23), b_mant << ((32 - 2) - 23));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2979 {
// Ported piece of fma_sf32: exponent adjustment + full 32x32->64 mantissa product.
// mul_u32 is a `static` C function, so its logic lives here as a private helper.

use core::ffi::c_int;

#[inline]
fn mul_u32(plow: &mut u32, a: u32, b: u32) -> u32 {
    let r: u64 = (a as u64).wrapping_mul(b as u64);
    *plow = r as u32;
    (r >> 32) as u32
}

#[no_mangle]
pub extern "C" fn fma_sf32__part2979(
    a_exp: c_int,
    b_exp: c_int,
    a_mant: u32,
    b_mant: u32,
    r_exp: *mut c_int,
    r_mant1: *mut u32,
    r_mant0: *mut u32,
) {
    // r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3;
    let e: c_int = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1 << (8 - 1))
        .wrapping_add(3);
    // r_mant1 = mul_u32(&r_mant0, a_mant << ((32 - 2) - 23), b_mant << ((32 - 2) - 23));
    let mut low: u32 = 0;
    let high: u32 = mul_u32(&mut low, a_mant.wrapping_shl((32 - 2) - 23), b_mant.wrapping_shl((32 - 2) - 23));
    unsafe {
        *r_exp = e;
        *r_mant1 = high;
        *r_mant0 = low;
    }
}
}

// ── `r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3; r_mant1 = mul_u64(&r_mant0, a_mant << ((64 - 2) -…` · softfp_template.h:463-465 · chunk 3091 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3;
//     
//         r_mant1 = mul_u64(&r_mant0, a_mant << ((64 - 2) - 52), b_mant << ((64 - 2) - 52));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3091 {
// Port of part of C function fma_sf64 (softfp_template.h lines 463-465):
//   r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3;
//   r_mant1 = mul_u64(&r_mant0, a_mant << ((64 - 2) - 52), b_mant << ((64 - 2) - 52));
// mul_u64 is a `static` C function (no linker symbol), so its logic is
// reimplemented here as a private Rust function.

fn mul_u64(plow: *mut u64, a: u64, b: u64) -> u64 {
    let r: u128 = (a as u128) * (b as u128);
    unsafe {
        *plow = r as u64;
    }
    (r >> 64) as u64
}

#[no_mangle]
pub extern "C" fn fma_sf64__part3091(
    p_r_exp: *mut i32,
    p_r_mant0: *mut u64,
    a_exp: i32,
    b_exp: i32,
    a_mant: u64,
    b_mant: u64,
) -> u64 {
    let r_exp: i32 = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1i32 << (11 - 1))
        .wrapping_add(3);
    let r_mant1: u64 = mul_u64(
        p_r_mant0,
        a_mant << ((64 - 2) - 52),
        b_mant << ((64 - 2) - 52),
    );
    unsafe {
        *p_r_exp = r_exp;
    }
    r_mant1
}
}

// ── `r_exp = a_exp + b_exp - (1 << (15 - 1)) + 3; r_mant1 = mul_u128(&r_mant0, a_mant << ((128 - 2)…` · softfp_template.h:463-465 · chunk 3213 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp + b_exp - (1 << (15 - 1)) + 3;
//     
//         r_mant1 = mul_u128(&r_mant0, a_mant << ((128 - 2) - 112), b_mant << ((128 - 2) - 112));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3213 {
// Port of the mul_u128 helper (static in C) plus the fma_sf128 exponent/mantissa step.

type U128 = u128;

// Exact port of static mul_u128(uint128_t *plow, uint128_t a, uint128_t b)
// Returns (high product, low product).
fn mul_u128(a: U128, b: U128) -> (U128, U128) {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: U128 = (a0 as U128) * (b0 as U128);
    let r01: U128 = (a0 as U128) * (b1 as U128);
    let r10: U128 = (a1 as U128) * (b0 as U128);
    let r11: U128 = (a1 as U128) * (b1 as U128);

    let r0: u64 = r00 as u64;
    let mut c: U128 = (r00 >> 64) + ((r01 as u64) as U128) + ((r10 as u64) as U128);
    let r1: u64 = c as u64;
    c = (c >> 64) + (r01 >> 64) + (r10 >> 64) + ((r11 as u64) as U128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64) + (r11 >> 64)) as u64;

    let low: U128 = ((r1 as U128) << 64) | (r0 as U128);
    let high: U128 = ((r3 as U128) << 64) | (r2 as U128);
    (high, low)
}

#[no_mangle]
pub extern "C" fn fma_sf128__part3213(
    a_exp: i32,
    b_exp: i32,
    a_mant: U128,
    b_mant: U128,
    r_exp: *mut i32,
    r_mant0: *mut U128,
) -> U128 {
    let exp_val: i32 = a_exp
        .wrapping_add(b_exp)
        .wrapping_sub(1i32 << (15 - 1))
        .wrapping_add(3);

    let (high, low) = mul_u128(a_mant << ((128 - 2) - 112), b_mant << ((128 - 2) - 112));

    unsafe {
        *r_exp = exp_val;
        *r_mant0 = low;
    }
    high
}
}

// ── `if (r_mant1 < ((uint32_t)1 << (32 - 3)` · softfp_template.h:467-471 · chunk 2980 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_mant1 < ((uint32_t)1 << (32 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (32 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2980 {
// Part of fma_sf32: normalize the 64-bit product mantissa when the high word
// is below 1 << (32 - 3).
#[no_mangle]
pub extern "C" fn fma_sf32__part2980(
    p_r_mant1: *mut u32,
    p_r_mant0: *mut u32,
    p_r_exp: *mut i32,
) {
    unsafe {
        let r_mant1 = *p_r_mant1;
        let r_mant0 = *p_r_mant0;
        if r_mant1 < (1u32 << (32 - 3)) {
            *p_r_mant1 = (r_mant1 << 1) | (r_mant0 >> (32 - 1));
            *p_r_mant0 = r_mant0 << 1;
            *p_r_exp -= 1;
        }
    }
}
}

// ── `if (r_mant1 < ((uint64_t)1 << (64 - 3)` · softfp_template.h:467-471 · chunk 3092 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_mant1 < ((uint64_t)1 << (64 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (64 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3092 {
// Port of the mantissa pre-normalization step of fma_sf64 (softfp_template.h).
// Reads and writes r_mant1, r_mant0 and r_exp, which live in the C caller.

#[no_mangle]
pub extern "C" fn fma_sf64__part3092(
    r_mant1: *mut u64,
    r_mant0: *mut u64,
    r_exp: *mut i32,
) {
    unsafe {
        let m1 = *r_mant1;
        if m1 < (1u64 << (64 - 3)) {
            let m0 = *r_mant0;
            *r_mant1 = m1.wrapping_shl(1) | (m0 >> (64 - 1));
            *r_mant0 = m0.wrapping_shl(1);
            *r_exp = (*r_exp).wrapping_sub(1);
        }
    }
}
}

// ── `if (r_mant1 < ((uint128_t)1 << (128 - 3)` · softfp_template.h:467-471 · chunk 3214 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_mant1 < ((uint128_t)1 << (128 - 3))) {
//             r_mant1 = (r_mant1 << 1) | (r_mant0 >> (128 - 1));
//             r_mant0 <<= 1;
//             r_exp--;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3214 {
// Port of the product renormalization step of fma_sf128 (softfp_template.h 467-471).
// If the high product mantissa lost the top bit, shift the 256-bit mantissa
// (r_mant1:r_mant0) left by one bit and decrement the exponent.

#[no_mangle]
pub extern "C" fn fma_sf128__part3214(
    r_mant1: *mut u128,
    r_mant0: *mut u128,
    r_exp: *mut i32,
) {
    unsafe {
        let m1 = *r_mant1;
        if m1 < (1u128 << (128 - 3)) {
            let m0 = *r_mant0;
            *r_mant1 = (m1 << 1) | (m0 >> (128 - 1));
            *r_mant0 = m0 << 1;
            *r_exp -= 1;
        }
    }
}
}

// ── `if (c_exp == 0) … else` · softfp_template.h:474-483 · chunk 2981 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf32(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf32(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2981 {
// Part of fma_sf32 (softfp_template.h lines 474-483):
// handle the addend c: subnormal normalisation, or implicit-bit setting.

use core::ffi::{c_int, c_uint};

// RoundingModeEnum values
const RM_RNE: c_uint = 0;

#[inline]
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
#[inline]
fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffff)
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32;
    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << ((32 - 2) - 23 - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (and default)
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) + 1)) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (23 + 1)) as c_int);
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));
    // the C code asserts shift >= 0 here; never panics in Rust
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut c_int, a_mant: u32) -> u32 {
    let shift: c_int = 23 - (32 - 1 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

#[no_mangle]
pub extern "C" fn fma_sf32__part2981(
    pc_exp: *mut i32,
    pc_mant: *mut u32,
    r_sign: u32,
    r_exp: i32,
    r_mant1: u32,
    r_mant0: u32,
    rm: c_uint,
    pfflags: *mut u32,
    pret: *mut i32,
) -> u32 {
    unsafe {
        let mut c_exp: c_int = *pc_exp;
        let mut c_mant: u32 = *pc_mant;

        if c_exp == 0 {
            if c_mant == 0 {
                let mant1: u32 = r_mant1 | ((r_mant0 != 0) as u32);
                *pc_exp = c_exp;
                *pc_mant = c_mant;
                *pret = 1;
                return normalize_sf32(r_sign, r_exp, mant1, rm, pfflags);
            }
            c_mant = normalize_subnormal_sf32(&mut c_exp, c_mant);
        } else {
            c_mant |= 1u32 << 23;
        }

        *pc_exp = c_exp;
        *pc_mant = c_mant;
        *pret = 0;
        0
    }
}
}

// ── `if (c_exp == 0) … else` · softfp_template.h:474-483 · chunk 3093 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf64(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf64(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3093 {
// Part of fma_sf64: handle the c addend mantissa/exponent preparation.
use core::ffi::c_int;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// Returns 1 when the caller must return normalize_sf64(r_sign, r_exp, r_mant1, rm, pfflags),
// 0 when execution continues with the (possibly updated) c_exp / c_mant.
#[no_mangle]
pub extern "C" fn fma_sf64__part3093(
    p_c_exp: *mut i32,
    p_c_mant: *mut u64,
    p_r_mant1: *mut u64,
    r_mant0: u64,
) -> c_int {
    unsafe {
        let c_exp = *p_c_exp;
        let c_mant = *p_c_mant;
        if c_exp == 0 {
            if c_mant == 0 {
                // add zero
                *p_r_mant1 |= (r_mant0 != 0) as u64;
                return 1;
            }
            // normalize_subnormal_sf64(&c_exp, c_mant)
            let shift: i32 = 52 - ((64 - 1) - clz64(c_mant));
            *p_c_exp = 1 - shift;
            *p_c_mant = c_mant.wrapping_shl(shift as u32);
        } else {
            *p_c_mant = c_mant | (1u64 << 52);
        }
        0
    }
}
}

// ── `if (c_exp == 0) … else` · softfp_template.h:474-483 · chunk 3215 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (c_exp == 0) {
//             if (c_mant == 0) {
//     
//                 r_mant1 |= (r_mant0 != 0);
//                 return normalize_sf128(r_sign, r_exp, r_mant1, rm, pfflags);
//             }
//             c_mant = normalize_subnormal_sf128(&c_exp, c_mant);
//         } else {
//             c_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3215 {
// Port of the `if (c_exp == 0) { ... } else { ... }` piece of fma_sf128
// (softfp_template.h lines 474-483) together with the static helpers it needs
// (clz128, rshift_rnd128, pack_sf128, roundpack_sf128, normalize_sf128,
//  normalize_subnormal_sf128) which have no linker symbol in C.

use core::ffi::c_int;

const MASK112: u128 = (1u128 << 112) - 1;
// ((128 - 2) - 112) == 14
const RND_MASK: u128 = (1u128 << 14) - 1;

fn clz128(a: u128) -> i32 {
    // C: if (a == 0) 128 else number of leading zeros
    a.leading_zeros() as i32
}

fn shl128(a: u128, s: i32) -> u128 {
    if s < 0 {
        // undefined in C; not reachable from this code path
        return a;
    }
    if s >= 128 {
        return 0;
    }
    a << (s as u32)
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d <= 0 {
        return a;
    }
    if d >= 128 {
        return (a != 0) as u128;
    }
    let mask: u128 = (1u128 << (d as u32)) - 1;
    (a >> (d as u32)) | ((a & mask) != 0) as u128
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MASK112)
}

// a_exp is `int` in C; (uint128_t)a_exp sign-extends for negative values, so
// mirror that with a sign-extending cast.
fn pack_sf128_i(a_sign: u32, a_exp: i32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MASK112)
}

fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;
    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM: 1 << ((14) - 1)
            addend = 1u32 << 13;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP
            if a_sign ^ ((rm & 1) as u32) != 0 {
                addend = (1u32 << 14) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let mut rnd_bits: u128;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = a_mant & RND_MASK;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & RND_MASK;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    if rm == 0 && rnd_bits == (1u128 << 13) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 113) as i32);
    if a_mant <= MASK112 {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = MASK112;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf128_i(a_sign, a_exp, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    // shift = clz128(a_mant) - (128 - 1 - (128 - 2))
    let shift: i32 = clz128(a_mant) - 1;
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = shl128(a_mant, shift);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize_subnormal_sf128(pa_exp: &mut i32, a_mant: u128) -> u128 {
    // shift = 112 - ((128 - 1) - clz128(a_mant))
    let shift: i32 = 112 - (127 - clz128(a_mant));
    *pa_exp = 1 - shift;
    shl128(a_mant, shift)
}

#[no_mangle]
pub extern "C" fn fma_sf128__part3215(
    pc_exp: *mut i32,
    pc_mant: *mut u128,
    r_sign: u32,
    r_exp: i32,
    r_mant1: u128,
    r_mant0: u128,
    rm: c_int,
    pfflags: *mut u32,
    p_result: *mut u128,
) -> c_int {
    unsafe {
        let mut c_exp: i32 = *pc_exp;
        let mut c_mant: u128 = *pc_mant;

        if c_exp == 0 {
            if c_mant == 0 {
                let m1: u128 = r_mant1 | ((r_mant0 != 0) as u128);
                *p_result = normalize_sf128(r_sign, r_exp, m1, rm, pfflags);
                return 1;
            }
            c_mant = normalize_subnormal_sf128(&mut c_exp, c_mant);
        } else {
            c_mant |= 1u128 << 112;
        }

        *pc_exp = c_exp;
        *pc_mant = c_mant;
        0
    }
}
}

// ── `c_exp++; c_mant1 = c_mant << (((32 - 2) - 23) - 1); c_mant0 = 0;` · softfp_template.h:484-486 · chunk 2982 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     c_exp++;
//         c_mant1 = c_mant << (((32 - 2) - 23) - 1);
//         c_mant0 = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2982 {
// Port of fma_sf32 lines 484-486:
//     c_exp++;
//     c_mant1 = c_mant << (((32 - 2) - 23) - 1);
//     c_mant0 = 0;
// c_exp (int32_t) is updated through a pointer; the shifted mantissa is returned.
// The zero assignment to c_mant0 stays in the C caller.

#[no_mangle]
pub extern "C" fn fma_sf32__part2982(c_exp: *mut i32, c_mant: u32) -> u32 {
    unsafe {
        let cur = *c_exp;
        *c_exp = cur.wrapping_add(1);
    }
    // (((32 - 2) - 23) - 1) == 6
    c_mant << 6
}
}

// ── `c_exp++; c_mant1 = c_mant << (((64 - 2) - 52) - 1); c_mant0 = 0;` · softfp_template.h:484-486 · chunk 3094 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     c_exp++;
//         c_mant1 = c_mant << (((64 - 2) - 52) - 1);
//         c_mant0 = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3094 {
// Portion of fma_sf64 (softfp_template.h): c_exp++; c_mant1 = c_mant << 9; c_mant0 = 0;
use core::ffi::{c_int, c_ulonglong};

#[no_mangle]
pub extern "C" fn fma_sf64__part3094(
    c_exp: *mut c_int,
    c_mant: c_ulonglong,
    c_mant1: *mut c_ulonglong,
    c_mant0: *mut c_ulonglong,
) {
    unsafe {
        *c_exp = (*c_exp).wrapping_add(1);
        *c_mant1 = c_mant.wrapping_shl(9);
        *c_mant0 = 0;
    }
}
}

// ── `c_exp++; c_mant1 = c_mant << (((128 - 2) - 112) - 1); c_mant0 = 0;` · softfp_template.h:484-486 · chunk 3216 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     c_exp++;
//         c_mant1 = c_mant << (((128 - 2) - 112) - 1);
//         c_mant0 = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3216 {
// Extracted part of fma_sf128: increment c_exp and scale c_mant into
// (c_mant1, c_mant0). uint128_t (unsigned __int128) is passed by pointer
// to avoid any C/Rust 128-bit-by-value ABI differences.

#[no_mangle]
pub extern "C" fn fma_sf128__part3216(
    c_exp: *mut i32,
    c_mant: *const u128,
    c_mant1: *mut u128,
    c_mant0: *mut u128,
) {
    unsafe {
        // c_exp++;
        let exp = *c_exp;
        *c_exp = exp.wrapping_add(1);

        // c_mant1 = c_mant << (((128 - 2) - 112) - 1);  == c_mant << 13
        let mant = *c_mant;
        *c_mant1 = mant << 13;

        // c_mant0 = 0;
        *c_mant0 = 0;
    }
}
}

// ── `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` · softfp_template.h:492-500 · chunk 2983 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint32_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2983 {
// Swap the (larger-exponent) c operand into the r slots when r is not
// strictly the bigger magnitude operand.
#[no_mangle]
pub unsafe extern "C" fn fma_sf32__part2983(
    r_exp: *mut i32,
    c_exp: *mut i32,
    r_mant1: *mut u32,
    r_mant0: *mut u32,
    c_mant1: *mut u32,
    c_mant0: *mut u32,
    r_sign: *mut u32,
    c_sign: *mut u32,
) {
    let re = *r_exp;
    let ce = *c_exp;
    let rm1 = *r_mant1;
    let cm1 = *c_mant1;
    let cond = re > ce || (re == ce && rm1 >= cm1);
    if !cond {
        let tmp = *r_mant1;
        *r_mant1 = *c_mant1;
        *c_mant1 = tmp;

        let tmp = *r_mant0;
        *r_mant0 = *c_mant0;
        *c_mant0 = tmp;

        let c_tmp: i32 = *r_exp;
        *r_exp = *c_exp;
        *c_exp = c_tmp;

        let c_tmp: i32 = *r_sign as i32;
        *r_sign = *c_sign;
        *c_sign = c_tmp as u32;
    }
}
}

// ── `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` · softfp_template.h:492-500 · chunk 3095 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint64_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3095 {
// Swap r and c operands so that |r| >= |c| for the fma_sf64 addition step.
#[no_mangle]
pub extern "C" fn fma_sf64__part3095(
    r_exp: *mut i32,
    c_exp: *mut i32,
    r_mant1: *mut u64,
    c_mant1: *mut u64,
    r_mant0: *mut u64,
    c_mant0: *mut u64,
    r_sign: *mut u32,
    c_sign: *mut u32,
) {
    unsafe {
        let rexp = *r_exp;
        let cexp = *c_exp;
        let rm1 = *r_mant1;
        let cm1 = *c_mant1;
        if !(rexp > cexp || (rexp == cexp && rm1 >= cm1)) {
            let tmp_m1 = *r_mant1;
            *r_mant1 = *c_mant1;
            *c_mant1 = tmp_m1;

            let tmp_m0 = *r_mant0;
            *r_mant0 = *c_mant0;
            *c_mant0 = tmp_m0;

            let tmp_exp = *r_exp;
            *r_exp = *c_exp;
            *c_exp = tmp_exp;

            let tmp_sign = *r_sign;
            *r_sign = *c_sign;
            *c_sign = tmp_sign;
        }
    }
}
}

// ── `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` · softfp_template.h:492-500 · chunk 3217 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1))) {
//             uint128_t tmp;
//             int32_t c_tmp;
//     
//             tmp = r_mant1; r_mant1 = c_mant1; c_mant1 = tmp;
//             tmp = r_mant0; r_mant0 = c_mant0; c_mant0 = tmp;
//             c_tmp = r_exp; r_exp = c_exp; c_exp = c_tmp;
//             c_tmp = r_sign; r_sign = c_sign; c_sign = c_tmp;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3217 {
// Swap the (r) and (c) operand mantissas, exponents and signs when the
// result magnitude is smaller than the addend magnitude.
// uint128_t is unsigned __int128; int32_t -> i32; uint32_t -> u32.

#[no_mangle]
pub extern "C" fn fma_sf128__part3217(
    r_exp: *mut i32,
    c_exp: *mut i32,
    r_mant1: *mut u128,
    c_mant1: *mut u128,
    r_mant0: *mut u128,
    c_mant0: *mut u128,
    r_sign: *mut u32,
    c_sign: *mut u32,
) {
    unsafe {
        let re = *r_exp;
        let ce = *c_exp;
        if !(re > ce || (re == ce && *r_mant1 >= *c_mant1)) {
            let tmp_m1 = *r_mant1;
            *r_mant1 = *c_mant1;
            *c_mant1 = tmp_m1;

            let tmp_m0 = *r_mant0;
            *r_mant0 = *c_mant0;
            *c_mant0 = tmp_m0;

            *r_exp = ce;
            *c_exp = re;

            let tmp_s = *r_sign;
            *r_sign = *c_sign;
            *c_sign = tmp_s;
        }
    }
}
}

// ── `shift = r_exp - c_exp;` · softfp_template.h:502-502 · chunk 2984 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = r_exp - c_exp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2984 {
// Ported piece of fma_sf32: compute the exponent difference.
#[no_mangle]
pub extern "C" fn fma_sf32__part2984(r_exp: core::ffi::c_int, c_exp: core::ffi::c_int) -> core::ffi::c_int {
    r_exp.wrapping_sub(c_exp)
}
}

// ── `shift = r_exp - c_exp;` · softfp_template.h:502-502 · chunk 3096 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = r_exp - c_exp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3096 {
// Piece of fma_sf64: compute the exponent difference used to align the addend.
#[no_mangle]
pub extern "C" fn fma_sf64__part3096(r_exp: core::ffi::c_int, c_exp: core::ffi::c_int) -> core::ffi::c_int {
    r_exp.wrapping_sub(c_exp)
}
}

// ── `shift = r_exp - c_exp;` · softfp_template.h:502-502 · chunk 3218 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shift = r_exp - c_exp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3218 {
// Extracted piece of fma_sf128: shift = r_exp - c_exp;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fma_sf128__part3218(r_exp: c_int, c_exp: c_int) -> c_int {
    // int32_t arithmetic, wrapping like the original machine operation
    r_exp.wrapping_sub(c_exp)
}
}

// ── `if (shift >= 2 * 32) … else` · softfp_template.h:503-516 · chunk 2985 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift >= 2 * 32) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 32 + 1) {
//             c_mant0 = rshift_rnd32(c_mant1, shift - 32);
//             c_mant1 = 0;
//         } else if (shift == 32) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint32_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (32 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2985 {
// Port of the alignment/shift block of fma_sf32 (softfp_template.h lines 503-516).

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn fma_sf32__part2985(
    shift: i32,
    c_mant0: *mut u32,
    c_mant1: *mut u32,
    mask: *mut u32,
) {
    unsafe {
        if shift >= 2 * 32 {
            *c_mant0 = ((*c_mant0 | *c_mant1) != 0) as u32;
            *c_mant1 = 0;
        } else if shift >= 32 + 1 {
            let v = rshift_rnd32(*c_mant1, shift - 32);
            *c_mant0 = v;
            *c_mant1 = 0;
        } else if shift == 32 {
            let v = *c_mant1 | ((*c_mant0 != 0) as u32);
            *c_mant0 = v;
            *c_mant1 = 0;
        } else if shift != 0 {
            *mask = (1u32 << shift) - 1;
            let m = *mask;
            let new0 = (*c_mant1 << (32 - shift)) | (*c_mant0 >> shift) | (((*c_mant0 & m) != 0) as u32);
            let new1 = *c_mant1 >> shift;
            *c_mant0 = new0;
            *c_mant1 = new1;
        }
    }
}
}

// ── `if (shift >= 2 * 64) … else` · softfp_template.h:503-516 · chunk 3097 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift >= 2 * 64) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 64 + 1) {
//             c_mant0 = rshift_rnd64(c_mant1, shift - 64);
//             c_mant1 = 0;
//         } else if (shift == 64) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint64_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (64 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3097 {
// Port of the mantissa-alignment piece of fma_sf64 (softfp_template.h lines 503-516).

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << d) - 1;
            a = (a >> d) | (((a & mask) != 0) as u64);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn fma_sf64__part3097(
    shift: i32,
    pc_mant0: *mut u64,
    pc_mant1: *mut u64,
    pmask: *mut u64,
) {
    unsafe {
        let mut c_mant0 = *pc_mant0;
        let mut c_mant1 = *pc_mant1;
        let mask: u64;

        if shift >= 2 * 64 {
            c_mant0 = ((c_mant0 | c_mant1) != 0) as u64;
            c_mant1 = 0;
        } else if shift >= 64 + 1 {
            c_mant0 = rshift_rnd64(c_mant1, shift - 64);
            c_mant1 = 0;
        } else if shift == 64 {
            c_mant0 = c_mant1 | (((c_mant0 != 0) as u64));
            c_mant1 = 0;
        } else if shift != 0 {
            mask = (1u64 << shift) - 1;
            c_mant0 = (c_mant1 << (64 - shift))
                | (c_mant0 >> shift)
                | (((c_mant0 & mask) != 0) as u64);
            c_mant1 = c_mant1 >> shift;
            *pmask = mask;
        }

        *pc_mant0 = c_mant0;
        *pc_mant1 = c_mant1;
    }
}
}

// ── `if (shift >= 2 * 128) … else` · softfp_template.h:503-516 · chunk 3219 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shift >= 2 * 128) {
//             c_mant0 = (c_mant0 | c_mant1) != 0;
//             c_mant1 = 0;
//         } else if (shift >= 128 + 1) {
//             c_mant0 = rshift_rnd128(c_mant1, shift - 128);
//             c_mant1 = 0;
//         } else if (shift == 128) {
//             c_mant0 = c_mant1 | (c_mant0 != 0);
//             c_mant1 = 0;
//         } else if (shift != 0) {
//             mask = ((uint128_t)1 << shift) - 1;
//             c_mant0 = (c_mant1 << (128 - shift)) | (c_mant0 >> shift) | ((c_mant0 & mask) != 0);
//             c_mant1 = c_mant1 >> shift;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3219 {
// Port of the shift/align piece of fma_sf128 (softfp_template.h lines 503-516).
// uint128_t is C's unsigned __int128; Rust's u128 has the same size/alignment
// and is ABI compatible.  All values are passed by pointer since
// c_mant0/c_mant1/mask are assigned.

// Private port of the static C helper rshift_rnd128().
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (((a & mask) != 0) as u128);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn fma_sf128__part3219(
    shift: i32,
    c_mant0: *mut u128,
    c_mant1: *mut u128,
    mask: *mut u128,
) {
    unsafe {
        if shift >= 2 * 128 {
            let m0 = *c_mant0;
            let m1 = *c_mant1;
            *c_mant0 = ((m0 | m1) != 0) as u128;
            *c_mant1 = 0;
        } else if shift >= 128 + 1 {
            let m1 = *c_mant1;
            *c_mant0 = rshift_rnd128(m1, shift - 128);
            *c_mant1 = 0;
        } else if shift == 128 {
            let m0 = *c_mant0;
            let m1 = *c_mant1;
            *c_mant0 = m1 | (((m0 != 0) as u128));
            *c_mant1 = 0;
        } else if shift != 0 {
            let m0 = *c_mant0;
            let m1 = *c_mant1;
            let msk: u128 = (1u128 << (shift as u32)) - 1;
            *mask = msk;
            *c_mant0 = (m1 << ((128 - shift) as u32))
                | (m0 >> (shift as u32))
                | (((m0 & msk) != 0) as u128);
            *c_mant1 = m1 >> (shift as u32);
        }
    }
}
}

// ── `if (r_sign == c_sign) … else` · softfp_template.h:520-532 · chunk 2986 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint32_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2986 {
use core::ffi::c_uint;

const RM_RDN: c_uint = 2;

#[no_mangle]
pub extern "C" fn fma_sf32__part2986(
    r_mant0: *mut u32,
    r_mant1: *mut u32,
    r_sign: *mut u32,
    c_mant0: u32,
    c_mant1: u32,
    c_sign: u32,
    rm: c_uint,
) {
    unsafe {
        let m0 = *r_mant0;
        let m1 = *r_mant1;
        let s = *r_sign;
        if s == c_sign {
            let new0 = m0.wrapping_add(c_mant0);
            let carry: u32 = if new0 < c_mant0 { 1 } else { 0 };
            let new1 = m1.wrapping_add(c_mant1).wrapping_add(carry);
            *r_mant0 = new0;
            *r_mant1 = new1;
        } else {
            let new0 = m0.wrapping_sub(c_mant0);
            let borrow: u32 = if new0 > m0 { 1 } else { 0 };
            let new1 = m1.wrapping_sub(c_mant1).wrapping_sub(borrow);
            *r_mant0 = new0;
            *r_mant1 = new1;
            if (new0 | new1) == 0 {
                *r_sign = if rm == RM_RDN { 1 } else { 0 };
            }
        }
    }
}
}

// ── `if (r_sign == c_sign) … else` · softfp_template.h:520-532 · chunk 3098 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint64_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3098 {
// Add/subtract the shifted addend mantissa (c) from the product mantissa (r).
// rm is RoundingModeEnum (RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4).
#[no_mangle]
pub extern "C" fn fma_sf64__part3098(
    r_sign: *mut u32,
    c_sign: u32,
    r_mant0: *mut u64,
    r_mant1: *mut u64,
    c_mant0: u64,
    c_mant1: u64,
    rm: u32,
) {
    unsafe {
        let mut m0 = *r_mant0;
        let mut m1 = *r_mant1;
        let mut sign = *r_sign;
        if sign == c_sign {
            m0 = m0.wrapping_add(c_mant0);
            m1 = m1.wrapping_add(c_mant1).wrapping_add((m0 < c_mant0) as u64);
        } else {
            let tmp = m0;
            m0 = m0.wrapping_sub(c_mant0);
            m1 = m1.wrapping_sub(c_mant1).wrapping_sub((m0 > tmp) as u64);
            if (m0 | m1) == 0 {
                sign = (rm == 2) as u32;
            }
        }
        *r_mant0 = m0;
        *r_mant1 = m1;
        *r_sign = sign;
    }
}
}

// ── `if (r_sign == c_sign) … else` · softfp_template.h:520-532 · chunk 3220 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r_sign == c_sign) {
//             r_mant0 += c_mant0;
//             r_mant1 += c_mant1 + (r_mant0 < c_mant0);
//         } else {
//             uint128_t tmp;
//             tmp = r_mant0;
//             r_mant0 -= c_mant0;
//             r_mant1 = r_mant1 - c_mant1 - (r_mant0 > tmp);
//             if ((r_mant0 | r_mant1) == 0) {
//     
//                 r_sign = (rm == RM_RDN);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3220 {
// Port of the final add/subtract step of fma_sf128.
// Updates r_mant0/r_mant1 in place (via pointers) and returns the new r_sign.

#[no_mangle]
pub extern "C" fn fma_sf128__part3220(
    r_sign_in: u32,
    c_sign: u32,
    r_mant0: *mut u128,
    r_mant1: *mut u128,
    c_mant0: u128,
    c_mant1: u128,
    rm: u32,
) -> u32 {
    let mut rs: u32 = r_sign_in;
    let mut m0: u128 = unsafe { *r_mant0 };
    let mut m1: u128 = unsafe { *r_mant1 };

    if rs == c_sign {
        m0 = m0.wrapping_add(c_mant0);
        // C: r_mant1 += c_mant1 + (r_mant0 < c_mant0)
        let carry: u128 = if m0 < c_mant0 { 1 } else { 0 };
        m1 = m1.wrapping_add(c_mant1.wrapping_add(carry));
    } else {
        let tmp: u128 = m0;
        m0 = m0.wrapping_sub(c_mant0);
        let borrow: u128 = if m0 > tmp { 1 } else { 0 };
        m1 = m1.wrapping_sub(c_mant1).wrapping_sub(borrow);
        if (m0 | m1) == 0 {
            rs = if rm == 2 { 1 } else { 0 }; // RM_RDN
        }
    }

    unsafe {
        *r_mant0 = m0;
        *r_mant1 = m1;
    }
    rs
}
}

// ── `return normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` · softfp_template.h:550-550 · chunk 2987 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2987 {
// Port of the TARGET: return normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
// normalize2_sf32 / roundpack_sf32 / pack_sf32 / rshift_rnd32 / clz32 are `static` C
// functions (no linker symbols), so their logic is mirrored here as private Rust fns.

use core::ffi::c_int;

const F32_MANT_MASK: u32 = (1u32 << 23) - 1;
const F32_EXP_MAX: c_int = (1 << 8) - 1;
// (32 - 2) - 23 == 7
const RND_BITS: u32 = (32 - 2) - 23;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & F32_MANT_MASK)
}

fn roundpack_sf32(a_sign: u32, a_exp: c_int, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let mut rnd_bits: u32;
    let addend: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    match rm {
        0 | 4 => addend = 1u32 << (RND_BITS - 1),
        1 => addend = 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << RND_BITS) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << RND_BITS) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << RND_BITS) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> RND_BITS;

    // rm == RM_RNE
    if rm == 0 && rnd_bits == (1u32 << (RND_BITS - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= F32_MANT_MASK {
        a_exp = 0;
    } else if a_exp >= F32_EXP_MAX {
        if addend == 0 {
            a_exp = F32_EXP_MAX - 1;
            a_mant = F32_MANT_MASK;
        } else {
            a_exp = F32_EXP_MAX;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize2_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u32,
    a_mant0: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant1 = a_mant1;
    let mut a_mant0 = a_mant0;

    let l: c_int = if a_mant1 == 0 {
        32 + clz32(a_mant0)
    } else {
        clz32(a_mant1)
    };
    // shift = l - (32 - 1 - (32 - 2)); the C assert ensures shift >= 0
    let shift: c_int = l - 1;

    a_exp -= shift;
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u32;
    } else if shift < 32 {
        a_mant1 = (a_mant1 << shift) | (a_mant0 >> (32 - shift));
        a_mant0 <<= shift;
        a_mant1 |= (a_mant0 != 0) as u32;
    } else {
        a_mant1 = a_mant0 << (shift - 32);
    }
    roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn fma_sf32__part2987(
    r_sign: u32,
    r_exp: i32,
    r_mant1: u32,
    r_mant0: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags)
}
}

// ── `return normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` · softfp_template.h:550-550 · chunk 3099 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3099 {
// Port of normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags)
// from fma_sf64, with the static helpers it depends on inlined as private fns.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | 4 => {
            // RM_RNE, RM_RMM
            addend = 1 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = (1i32).wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1) as u64)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1) as u64)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize2_sf64_impl(
    a_sign: u32,
    a_exp_in: i32,
    a_mant1_in: u64,
    a_mant0_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant1: u64 = a_mant1_in;
    let mut a_mant0: u64 = a_mant0_in;

    let l: i32 = if a_mant1 == 0 {
        64 + clz64(a_mant0)
    } else {
        clz64(a_mant1)
    };
    let shift: i32 = l - (64 - 1 - (64 - 2));

    a_exp = a_exp.wrapping_sub(shift);
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u64;
    } else if shift < 64 {
        a_mant1 = (a_mant1 << shift) | (a_mant0 >> (64 - shift));
        a_mant0 <<= shift;
        a_mant1 |= (a_mant0 != 0) as u64;
    } else {
        a_mant1 = a_mant0 << (shift - 64);
    }
    roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn fma_sf64__part3099(
    r_sign: u32,
    r_exp: i32,
    r_mant1: u64,
    r_mant0: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    normalize2_sf64_impl(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags)
}
}

// ── `return normalize2_sf128(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` · softfp_template.h:550-550 · chunk 3221 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize2_sf128(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3221 {
// Port of the static C helper chain used by the target statement:
//   fma_sf128__part3221(a_sign, a_exp, a_mant1, a_mant0, rm, pfflags)
//     == normalize2_sf128(a_sign, a_exp, a_mant1, a_mant0, rm, pfflags)
// normalize2_sf128, roundpack_sf128, pack_sf128, rshift_rnd128 and clz128 are
// `static` in the C translation unit (no linker symbols), so their logic is
// reimplemented here as private Rust functions.

use core::ffi::c_int;

extern "C" {
    fn abort() -> !;
}

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;
const MANT_MASK: u128 = (1u128 << 112) - 1;

fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (((128 - 2) - 112) - 1);
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << ((128 - 2) - 112)) - 1;
        } else {
            addend = 0;
        }
    }

    let mut rnd_bits: u128;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = a_mant & ((1u128 << ((128 - 2) - 112)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u128 << ((128 - 2) - 112)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u128 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp = ((a_exp as u128).wrapping_add(a_mant >> (112 + 1))) as c_int;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize2_sf128(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant1: u128,
    mut a_mant0: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let l: c_int = if a_mant1 == 0 {
        128 + clz128(a_mant0)
    } else {
        clz128(a_mant1)
    };
    let shift: c_int = l - (128 - 1 - (128 - 2));
    if shift < 0 {
        // mirrors the C assertion `shift >= 0` (never taken for valid inputs)
        unsafe { abort() }
    }
    a_exp = a_exp.wrapping_sub(shift);
    if shift == 0 {
        a_mant1 |= (a_mant0 != 0) as u128;
    } else if shift < 128 {
        a_mant1 = (a_mant1 << shift) | (a_mant0 >> (128 - shift));
        a_mant0 <<= shift;
        a_mant1 |= (a_mant0 != 0) as u128;
    } else {
        a_mant1 = a_mant0 << (shift - 128);
    }
    roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn fma_sf128__part3221(
    a_sign: u32,
    a_exp: c_int,
    a_mant1: u128,
    a_mant0: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    normalize2_sf128(a_sign, a_exp, a_mant1, a_mant0, rm, pfflags)
}
}

// ── `divrem_u32` · softfp_template.h:556-562 · chunk 2988 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t divrem_u32(uint32_t *pr, uint32_t ah, uint32_t al, uint32_t b)
//     {
//         uint64_t a;
//         a = ((uint64_t)ah << 32) | al;
//         *pr = a % b;
//         return a / b;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2988 {
// Port of softfp_template.h's static divrem_u32:
//   static uint32_t divrem_u32(uint32_t *pr, uint32_t ah, uint32_t al, uint32_t b)
// Computes the 64-bit value (ah:al), stores the remainder through *pr and
// returns the quotient.

#[no_mangle]
pub extern "C" fn divrem_u32(pr: *mut u32, ah: u32, al: u32, b: u32) -> u32 {
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    // C would be undefined for b == 0; avoid a Rust panic across the FFI
    // boundary in that case.
    let (quo, rem): (u64, u64) = if b == 0 {
        (0, 0)
    } else {
        let d = b as u64;
        (a / d, a % d)
    };
    unsafe {
        *pr = rem as u32;
    }
    quo as u32
}
}

// ── `divrem_u64` · softfp_template.h:556-562 · chunk 3100 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t divrem_u64(uint64_t *pr, uint64_t ah, uint64_t al, uint64_t b)
//     {
//         uint128_t a;
//         a = ((uint128_t)ah << 64) | al;
//         *pr = a % b;
//         return a / b;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3100 {
use core::ffi::c_ulonglong;

// uint64_t divrem_u64(uint64_t *pr, uint64_t ah, uint64_t al, uint64_t b)
// Computes a = ((uint128_t)ah << 64) | al; *pr = a % b; return a / b;
#[no_mangle]
pub extern "C" fn divrem_u64(
    pr: *mut c_ulonglong,
    ah: c_ulonglong,
    al: c_ulonglong,
    b: c_ulonglong,
) -> c_ulonglong {
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    let b = b as u128;

    // C division by zero is undefined (traps); avoid a Rust panic crossing FFI.
    if b == 0 {
        unsafe {
            if !pr.is_null() {
                *pr = 0;
            }
        }
        return 0;
    }

    let q = a / b;
    let r = a % b;

    unsafe {
        if !pr.is_null() {
            *pr = r as c_ulonglong;
        }
    }

    q as c_ulonglong
}
}

// ── `uint64_t a; a = ((uint64_t)ah << 32) | al; *pr = a % b; return a / b;` · softfp_template.h:558-561 · chunk 2989 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t a;
//         a = ((uint64_t)ah << 32) | al;
//         *pr = a % b;
//         return a / b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2989 {
// Ported piece of divrem_u32: 64/32 bit unsigned division with remainder.
// a = ((uint64_t)ah << 32) | al; *pr = a % b; return a / b;

#[no_mangle]
pub extern "C" fn divrem_u32__part2989(pr: *mut u32, ah: u32, al: u32, b: u32) -> u32 {
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    let bb: u64 = b as u64;
    let rem: u64 = a % bb;
    let quot: u64 = a / bb;
    unsafe {
        *pr = rem as u32;
    }
    quot as u32
}
}

// ── `uint128_t a; a = ((uint128_t)ah << 64) | al; *pr = a % b; return a / b;` · softfp_template.h:558-561 · chunk 3101 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t a;
//         a = ((uint128_t)ah << 64) | al;
//         *pr = a % b;
//         return a / b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3101 {
// Port of the body of divrem_u64 (softfp_template.h lines 558-561):
// build a 128-bit dividend from ah:al, store the remainder through pr and
// return the quotient.

#[no_mangle]
pub extern "C" fn divrem_u64__part3101(pr: *mut u64, ah: u64, al: u64, b: u64) -> u64 {
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    // checked_* avoids any panic on a zero divisor (C would be undefined there)
    let (q, r): (u128, u128) = match a.checked_div(b as u128) {
        Some(q) => match a.checked_rem(b as u128) {
            Some(r) => (q, r),
            None => (0u128, 0u128),
        },
        None => (0u128, 0u128),
    };
    unsafe {
        *pr = r as u64;
    }
    q as u64
}
}

// ── `divrem_u128` · softfp_template.h:567-585 · chunk 3222 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t divrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0, uint128_t b)
//     {
//         int i, qb, ab;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        a1 < b
//        )), __extension__ ({ if (
//        a1 < b
//        ) ; else __assert_fail (
//        "a1 < b"
//        , "softfp_template.h", 571, __extension__ __PRETTY_FUNCTION__); }))
//                      ;
//         for(i = 0; i < 128; i++) {
//             ab = a1 >> (128 - 1);
//             a1 = (a1 << 1) | (a0 >> (128 - 1));
//             if (ab || a1 >= b) {
//                 a1 -= b;
//                 qb = 1;
//             } else {
//                 qb = 0;
//             }
//             a0 = (a0 << 1) | qb;
//         }
//         *pr = a1;
//         return a0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3222 {
// Port of softfp_template.h divrem_u128 (uint128_t long division helper).

#[no_mangle]
pub extern "C" fn divrem_u128(pr: *mut u128, a1: u128, a0: u128, b: u128) -> u128 {
    let mut a1 = a1;
    let mut a0 = a0;
    let mut i: i32 = 0;
    while i < 128 {
        // ab = a1 >> (128 - 1)  (0 or 1)
        let ab: bool = (a1 >> 127) != 0;
        a1 = (a1 << 1) | (a0 >> 127);
        let qb: u128;
        if ab || a1 >= b {
            a1 = a1.wrapping_sub(b);
            qb = 1;
        } else {
            qb = 0;
        }
        a0 = (a0 << 1) | qb;
        i += 1;
    }
    unsafe {
        *pr = a1;
    }
    a0
}
}

// ── `int i, qb, ab; ((void) sizeof (__assert_single_arg ( a1 < b )), __extension__ ({ if ( a1 < b )…` · softfp_template.h:569-571 · chunk 3223 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int i, qb, ab;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        a1 < b
//        )), __extension__ ({ if (
//        a1 < b
//        ) ; else __assert_fail (
//        "a1 < b"
//        , "softfp_template.h", 571, __extension__ __PRETTY_FUNCTION__); }))
//                      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3223 {
// Port of the assertion prologue of divrem_u128 (softfp_template.h lines 569-571).
// Mirrors: assert(a1 < b) with __PRETTY_FUNCTION__ == divrem_u128's signature.
use core::ffi::{c_char, c_uint};

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    );
}

#[no_mangle]
pub extern "C" fn divrem_u128__part3223(a1: u128, b: u128) {
    if !(a1 < b) {
        unsafe {
            __assert_fail(
                b"a1 < b\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                571u32,
                b"uint128_t divrem_u128(uint128_t *, uint128_t, uint128_t, uint128_t)\0".as_ptr()
                    as *const c_char,
            );
        }
    }
}
}

// ── `for (i = 0; i < 128; i++)` · softfp_template.h:572-582 · chunk 3224 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 128; i++) {
//             ab = a1 >> (128 - 1);
//             a1 = (a1 << 1) | (a0 >> (128 - 1));
//             if (ab || a1 >= b) {
//                 a1 -= b;
//                 qb = 1;
//             } else {
//                 qb = 0;
//             }
//             a0 = (a0 << 1) | qb;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3224 {
// Restores the long-division loop of divrem_u128 (softfp_template.h lines 572-582).
// a1 and a0 are updated in place; b is only read.

#[no_mangle]
pub extern "C" fn divrem_u128__part3224(
    a1: *mut u128,
    a0: *mut u128,
    b: *const u128,
) {
    unsafe {
        let mut x1: u128 = *a1;
        let mut x0: u128 = *a0;
        let bv: u128 = *b;

        let mut i: i32 = 0;
        while i < 128 {
            // ab = a1 >> 127  (only bit 127 of a1)
            let ab: u128 = x1 >> 127;
            x1 = (x1 << 1) | (x0 >> 127);
            let qb: u128;
            if ab != 0 || x1 >= bv {
                x1 = x1.wrapping_sub(bv);
                qb = 1;
            } else {
                qb = 0;
            }
            x0 = (x0 << 1) | qb;
            i += 1;
        }

        *a1 = x1;
        *a0 = x0;
    }
}
}

// ── `*pr = a1; return a0;` · softfp_template.h:583-584 · chunk 3225 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pr = a1;
//         return a0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3225 {
// Stores the remainder through `pr` and returns the low word of the quotient.
// Mirrors the tail of divrem_u128 (softfp_template.h lines 583-584).

#[no_mangle]
pub extern "C" fn divrem_u128__part3225(pr: *mut u128, a1: u128, a0: u128) -> u128 {
    // C: *pr = a1;
    unsafe {
        *pr = a1;
    }
    // C: return a0;
    a0
}
}

// ── `div_sf32` · softfp_template.h:589-653 · chunk 2990 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t div_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint32_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0 || isnan_sf32(b)) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else if (b_exp == ((1 << 8) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else {
//                 return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 8) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//                 return pack_sf32(r_sign, 0, 0);
//             }
//         }
//     
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         r_exp = a_exp - b_exp + (1 << (8 - 1)) - 1;
//         r_mant = divrem_u32(&r, a_mant, 0, b_mant << 2);
//         if (r != 0)
//             r_mant |= 1;
//         return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2990 {
// Rust port of div_sf32 (softfp_template.h), single precision soft-float divide.

use core::ffi::c_int;

const F_QNAN32: u32 = 0x7fc0_0000; // (exp=0xff) | (1 << 22)

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
}

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & 0xffu32;
    let a_mant = a & 0x007f_ffffu32;
    a_exp == 0xffu32 && a_mant != 0
}

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1 = (a >> 22) & 0x1ffu32;
    let a_mant = a & 0x007f_ffffu32;
    a_exp1 == 0x1feu32 && a_mant != 0
}

fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - (31 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << (d as u32)).wrapping_sub(1);
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32;
    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << 6;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == 0 && rnd_bits == (1u32 << 6) {
        // RM_RNE
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> 24) as i32;
    if a_mant <= 0x007f_ffffu32 {
        a_exp = 0;
    } else if a_exp >= 0xff {
        if addend == 0 {
            a_exp = 0xfe;
            a_mant = 0x007f_ffffu32;
        } else {
            a_exp = 0xff;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | 1u32;
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant) - (31 - 30);
    // shift >= 0 assumed by the C code
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn divrem_u32(pr: &mut u32, ah: u32, al: u32, b: u32) -> u32 {
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    if b == 0 {
        *pr = 0;
        return 0;
    }
    let bb = b as u64;
    *pr = (a % bb) as u32;
    (a / bb) as u32
}

#[no_mangle]
pub extern "C" fn div_sf32(a: u32, b: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let a_sign: u32 = a >> 31;
    let b_sign: u32 = b >> 31;
    let r_sign: u32 = a_sign ^ b_sign;
    let mut a_exp: i32 = ((a >> 23) & 0xffu32) as i32;
    let mut b_exp: i32 = ((b >> 23) & 0xffu32) as i32;
    let mut a_mant: u32 = a & 0x007f_ffffu32;
    let mut b_mant: u32 = b & 0x007f_ffffu32;

    if a_exp == 0xff {
        if a_mant != 0 || isnan_sf32(b) {
            if issignan_sf32(a) || issignan_sf32(b) {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            return F_QNAN32;
        } else if b_exp == 0xff {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return F_QNAN32;
        } else {
            return pack_sf32(r_sign, 0xff, 0);
        }
    } else if b_exp == 0xff {
        if b_mant != 0 {
            if issignan_sf32(a) || issignan_sf32(b) {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            return F_QNAN32;
        } else {
            return pack_sf32(r_sign, 0, 0);
        }
    }

    if b_exp == 0 {
        if b_mant == 0 {
            // zero
            if a_exp == 0 && a_mant == 0 {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return F_QNAN32;
            } else {
                unsafe {
                    *pfflags |= 1u32 << 3;
                }
                return pack_sf32(r_sign, 0xff, 0);
            }
        }
        b_mant = normalize_subnormal_sf32(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u32 << 23;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf32(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u32 << 23;
    }
    let r_exp: i32 = a_exp - b_exp + (1 << (8 - 1)) - 1;
    let mut r: u32 = 0;
    let mut r_mant: u32 = divrem_u32(&mut r, a_mant, 0, b_mant << 2);
    if r != 0 {
        r_mant |= 1;
    }
    normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `div_sf64` · softfp_template.h:589-653 · chunk 3102 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t div_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint64_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0 || isnan_sf64(b)) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else if (b_exp == ((1 << 11) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else {
//                 return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 11) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//                 return pack_sf64(r_sign, 0, 0);
//             }
//         }
//     
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         r_exp = a_exp - b_exp + (1 << (11 - 1)) - 1;
//         r_mant = divrem_u64(&r, a_mant, 0, b_mant << 2);
//         if (r != 0)
//             r_mant |= 1;
//         return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3102 {
// Rust port of softfp_template.h div_sf64 (binary64 / double precision).

use core::ffi::{c_int, c_uint};

// C: typedef enum { RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM } RoundingModeEnum;
type RoundingModeEnum = c_uint;

// Non-static C functions (exported symbols) used by the target.
extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

const F_QNAN64: u64 = ((2047u64) << 52) | (1u64 << 51);

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_DIVIDE_ZERO: u32 = 1 << 3;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RDN: u32 = 2;
const RM_RUP: u32 = 3;
const RM_RMM: u32 = 4;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (63 - clz64(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => addend = 1 << 5,
        RM_RTZ => addend = 0,
        _ => {
            // RM_RDN, RM_RUP and anything else
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1 << 6) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << 6) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 6) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 6;

    if rm == RM_RNE && rnd_bits == (1 << 5) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> 53) as i32;
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= 2047 {
        if addend == 0 {
            a_exp = 2046;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = 2047;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant_in) - (63 - 62);
    let a_exp: i32 = a_exp_in - shift;
    let a_mant: u64 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn div_sf64(
    a: u64,
    b: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let a_sign: u32 = (a >> 63) as u32;
    let b_sign: u32 = (b >> 63) as u32;
    let r_sign: u32 = a_sign ^ b_sign;
    let mut a_exp: i32 = ((a >> 52) & 2047) as i32;
    let mut b_exp: i32 = ((b >> 52) & 2047) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);
    let mut b_mant: u64 = b & ((1u64 << 52) - 1);

    if a_exp == 2047 {
        if a_mant != 0 || unsafe { isnan_sf64(b) } != 0 {
            if unsafe { issignan_sf64(a) } != 0 || unsafe { issignan_sf64(b) } != 0 {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN64;
        } else if b_exp == 2047 {
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN64;
        } else {
            return pack_sf64(r_sign, 2047, 0);
        }
    } else if b_exp == 2047 {
        if b_mant != 0 {
            if unsafe { issignan_sf64(a) } != 0 || unsafe { issignan_sf64(b) } != 0 {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN64;
        } else {
            return pack_sf64(r_sign, 0, 0);
        }
    }

    if b_exp == 0 {
        if b_mant == 0 {
            // zero
            if a_exp == 0 && a_mant == 0 {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
                return F_QNAN64;
            } else {
                unsafe {
                    *pfflags |= FFLAG_DIVIDE_ZERO;
                }
                return pack_sf64(r_sign, 2047, 0);
            }
        }
        b_mant = normalize_subnormal_sf64(&mut b_exp, b_mant);
    } else {
        b_mant |= 1u64 << 52;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf64(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf64(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u64 << 52;
    }

    let r_exp: i32 = a_exp - b_exp + (1 << (11 - 1)) - 1;

    // static uint64_t divrem_u64(uint64_t *pr, uint64_t ah, uint64_t al, uint64_t b)
    //   a = ((uint128_t)ah << 64) | al;  *pr = a % b;  return a / b;
    let a128: u128 = a_mant as u128;
    let b128: u128 = (b_mant << 2) as u128;
    let (mut r_mant, r): (u64, u64) = if b128 == 0 {
        (0u64, 0u64)
    } else {
        ((a128 / b128) as u64, (a128 % b128) as u64)
    };
    if r != 0 {
        r_mant |= 1;
    }
    normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `div_sf128` · softfp_template.h:589-653 · chunk 3226 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t div_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
//                   uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint128_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0 || isnan_sf128(b)) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else if (b_exp == ((1 << 15) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else {
//                 return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 15) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//                 return pack_sf128(r_sign, 0, 0);
//             }
//         }
//     
//         if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1;
//         r_mant = divrem_u128(&r, a_mant, 0, b_mant << 2);
//         if (r != 0)
//             r_mant |= 1;
//         return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3226 {
//! Port of `div_sf128` (softfp_template.h) for binary128 soft-float.
//! All helpers used by the C original are `static` (no linker symbol), so
//! their logic is reproduced here as private Rust functions.

use core::ffi::c_int;

type U128 = u128;

const F_SIZE: u32 = 128;
const EXP_SIZE: i32 = 15;
const MANT_SIZE: i32 = 112;

const EXP_MASK: i32 = (1 << EXP_SIZE) - 1; // (1 << 15) - 1
const MANT_MASK: U128 = ((1u128 << 112) - 1) as U128;
const F_QNAN128: U128 = ((((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1))) as U128;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_DIVIDE_ZERO: u32 = 1 << 3;
const FFLAG_INVALID_OP: u32 = 1 << 4;

#[inline]
fn clz128(a: U128) -> i32 {
    // a == 0 -> 128, matching the C helper exactly.
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: U128) -> U128 {
    ((a_sign as U128) << (F_SIZE - 1))
        | ((a_exp as U128) << 112)
        | (a_mant & MANT_MASK)
}

fn rshift_rnd128(a: U128, d: i32) -> U128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as U128;
        } else {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as U128;
        }
    }
    a
}

unsafe fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: U128,
    rm: c_int,
    pfflags: *mut u32,
) -> U128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((F_SIZE - 2) as i32 - MANT_SIZE - 1) as u32);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and any other value
            if ((a_sign as c_int) ^ (rm & 1)) != 0 {
                addend = (1u32 << (((F_SIZE - 2) as i32 - MANT_SIZE) as u32)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let addend_u = addend as U128;
    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend_u) < (1u128 << (F_SIZE - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << ((F_SIZE - 2) as i32 - MANT_SIZE) as u32) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= FFLAG_UNDERFLOW;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << ((F_SIZE - 2) as i32 - MANT_SIZE) as u32) - 1)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= FFLAG_INEXACT;
    }
    a_mant = a_mant.wrapping_add(addend_u) >> ((F_SIZE - 2) as i32 - MANT_SIZE) as u32;

    if rm == RM_RNE
        && rnd_bits == (1u32 << (((F_SIZE - 2) as i32 - MANT_SIZE - 1) as u32))
    {
        a_mant &= !1u128;
    }

    // a_exp += a_mant >> (112 + 1); done the way C does it (int <- uint128_t)
    a_exp = ((a_exp as U128).wrapping_add(a_mant >> (112 + 1))) as i32;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: U128,
    rm: c_int,
    pfflags: *mut u32,
) -> U128 {
    let shift = clz128(a_mant_in) - (128 - 1 - (128 - 2));
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

unsafe fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: U128) -> U128 {
    let shift = 112 - (128 - 1 - clz128(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn divrem_u128(pr: *mut U128, a1_in: U128, a0_in: U128, b: U128) -> U128 {
    let mut a1 = a1_in;
    let mut a0 = a0_in;
    unsafe {
        for _i in 0..128i32 {
            let ab = (a1 >> (128 - 1)) as c_int;
            a1 = a1.wrapping_shl(1) | (a0 >> (128 - 1));
            let qb: c_int;
            if ab != 0 || a1 >= b {
                a1 = a1.wrapping_sub(b);
                qb = 1;
            } else {
                qb = 0;
            }
            a0 = a0.wrapping_shl(1) | (qb as U128);
        }
        *pr = a1;
    }
    a0
}

fn issignan_sf128(a: U128) -> c_int {
    let a_exp1 = (a >> (112 - 1)) & ((1u128 << (15 + 1)) - 1);
    let a_mant = a & MANT_MASK;
    (a_exp1 == (2 * ((1u128 << 15) - 1)) && a_mant != 0) as c_int
}

fn isnan_sf128(a: U128) -> c_int {
    let a_exp = (a >> 112) & ((1u128 << 15) - 1);
    let a_mant = a & MANT_MASK;
    (a_exp == ((1u128 << 15) - 1) && a_mant != 0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn div_sf128(
    a: U128,
    b: U128,
    rm: c_int,
    pfflags: *mut u32,
) -> U128 {
    let a_sign: u32;
    let b_sign: u32;
    let r_sign: u32;
    let mut a_exp: i32;
    let mut b_exp: i32;
    let r_exp: i32;
    let mut a_mant: U128;
    let mut b_mant: U128;
    let r_mant: U128;
    let mut r: U128 = 0;

    a_sign = (a >> (F_SIZE - 1)) as u32;
    b_sign = (b >> (F_SIZE - 1)) as u32;
    r_sign = a_sign ^ b_sign;
    a_exp = ((a >> 112) & (EXP_MASK as U128)) as i32;
    b_exp = ((b >> 112) & (EXP_MASK as U128)) as i32;
    a_mant = a & MANT_MASK;
    b_mant = b & MANT_MASK;

    if a_exp == EXP_MASK {
        if a_mant != 0 || isnan_sf128(b) != 0 {
            if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN128;
        } else if b_exp == EXP_MASK {
            *pfflags |= FFLAG_INVALID_OP;
            return F_QNAN128;
        } else {
            return pack_sf128(r_sign, EXP_MASK as u32, 0);
        }
    } else if b_exp == EXP_MASK {
        if b_mant != 0 {
            if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN128;
        } else {
            return pack_sf128(r_sign, 0, 0);
        }
    }

    if b_exp == 0 {
        if b_mant == 0 {
            // zero
            if a_exp == 0 && a_mant == 0 {
                *pfflags |= FFLAG_INVALID_OP;
                return F_QNAN128;
            } else {
                *pfflags |= FFLAG_DIVIDE_ZERO;
                return pack_sf128(r_sign, EXP_MASK as u32, 0);
            }
        }
        b_mant = normalize_subnormal_sf128(&mut b_exp, b_mant);
    } else {
        b_mant |= (1u128 << 112) as U128;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf128(r_sign, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf128(&mut a_exp, a_mant);
    } else {
        a_mant |= (1u128 << 112) as U128;
    }
    r_exp = a_exp - b_exp + (1 << (EXP_SIZE - 1)) - 1;
    r_mant = divrem_u128(&mut r, a_mant, 0, b_mant.wrapping_shl(2));
    let mut r_mant = r_mant;
    if r != 0 {
        r_mant |= 1;
    }
    normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint32_t a_mant, b_mant, r_mant,…` · softfp_template.h:592-602 · chunk 2991 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint32_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         b_exp = (b >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         b_mant = b & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2991 {
// Port of the unpacking prologue of div_sf32 (softfp_template.h lines 592-602).
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn div_sf32__part2991(
    a: c_uint,
    b: c_uint,
    pa_sign: *mut c_uint,
    pb_sign: *mut c_uint,
    pr_sign: *mut c_uint,
    pa_exp: *mut c_int,
    pb_exp: *mut c_int,
    pa_mant: *mut c_uint,
    pb_mant: *mut c_uint,
) {
    unsafe {
        let a_sign: c_uint = a >> (32 - 1);
        let b_sign: c_uint = b >> (32 - 1);
        let r_sign: c_uint = a_sign ^ b_sign;
        let a_exp: c_int = ((a >> 23) & ((1u32 << 8) - 1)) as c_int;
        let b_exp: c_int = ((b >> 23) & ((1u32 << 8) - 1)) as c_int;
        let a_mant: c_uint = a & ((1u32 << 23) - 1);
        let b_mant: c_uint = b & ((1u32 << 23) - 1);

        *pa_sign = a_sign;
        *pb_sign = b_sign;
        *pr_sign = r_sign;
        *pa_exp = a_exp;
        *pb_exp = b_exp;
        *pa_mant = a_mant;
        *pb_mant = b_mant;
    }
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint64_t a_mant, b_mant, r_mant,…` · softfp_template.h:592-602 · chunk 3103 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint64_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         b_exp = (b >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         b_mant = b & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3103 {
// Extracted piece of div_sf64: unpacking the operands.
#[no_mangle]
pub unsafe extern "C" fn div_sf64__part3103(
    a: u64,
    b: u64,
    a_sign: *mut u32,
    b_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    a_mant: *mut u64,
    b_mant: *mut u64,
) {
    let asg: u32 = (a >> 63) as u32;
    let bsg: u32 = (b >> 63) as u32;
    let rsg: u32 = asg ^ bsg;
    *a_sign = asg;
    *b_sign = bsg;
    *r_sign = rsg;
    *a_exp = ((a >> 52) & 0x7ffu64) as i32;
    *b_exp = ((b >> 52) & 0x7ffu64) as i32;
    *a_mant = a & ((1u64 << 52) - 1);
    *b_mant = b & ((1u64 << 52) - 1);
}
}

// ── `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint128_t a_mant, b_mant, r_mant,…` · softfp_template.h:592-602 · chunk 3227 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, b_sign, r_sign;
//         int32_t a_exp, b_exp, r_exp;
//         uint128_t a_mant, b_mant, r_mant, r;
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         r_sign = a_sign ^ b_sign;
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         b_exp = (b >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         b_mant = b & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3227 {
// Port of the head of div_sf128: unpacks sign/exponent/mantissa fields of
// both operands and computes the result sign.

#[no_mangle]
pub extern "C" fn div_sf128__part3227(
    a: u128,
    b: u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
    r_sign: *mut u32,
    a_exp: *mut i32,
    b_exp: *mut i32,
    a_mant: *mut u128,
    b_mant: *mut u128,
) {
    unsafe {
        let asgn: u32 = (a >> 127) as u32;
        let bsgn: u32 = (b >> 127) as u32;
        let rsgn: u32 = asgn ^ bsgn;

        let aex: i32 = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
        let bex: i32 = ((b >> 112) & ((1u128 << 15) - 1)) as i32;

        let amant: u128 = a & ((1u128 << 112) - 1);
        let bmant: u128 = b & ((1u128 << 112) - 1);

        *a_sign = asgn;
        *b_sign = bsgn;
        *r_sign = rsgn;
        *a_exp = aex;
        *b_exp = bex;
        *a_mant = amant;
        *b_mant = bmant;
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) … else` · softfp_template.h:603-624 · chunk 2992 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0 || isnan_sf32(b)) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else if (b_exp == ((1 << 8) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else {
//                 return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 8) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf32(a) || issignan_sf32(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//                 return pack_sf32(r_sign, 0, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2992 {
// Port of the NaN/infinity handling piece of div_sf32 (softfp_template.h 603-624).
#![allow(clippy::missing_safety_doc)]

use core::ffi::c_int;

const F_QNAN32: u32 = (255u32 << 23) | (1u32 << (23 - 1));

extern "C" {
    fn isnan_sf32(a: u32) -> c_int;
    fn issignan_sf32(a: u32) -> c_int;
}

// static inline in the C translation unit: no linker symbol, ported here.
#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

#[no_mangle]
pub unsafe extern "C" fn div_sf32__part2992(
    a: u32,
    b: u32,
    a_exp: i32,
    a_mant: u32,
    b_exp: i32,
    b_mant: u32,
    r_sign: u32,
    pfflags: *mut u32,
    p_handled: *mut c_int,
) -> u32 {
    const EXP_MAX: i32 = (1 << 8) - 1;

    if a_exp == EXP_MAX {
        if a_mant != 0 || isnan_sf32(b) != 0 {
            if issignan_sf32(a) != 0 || issignan_sf32(b) != 0 {
                *pfflags |= 1u32 << 4;
            }
            *p_handled = 1;
            return F_QNAN32;
        } else if b_exp == EXP_MAX {
            *pfflags |= 1u32 << 4;
            *p_handled = 1;
            return F_QNAN32;
        } else {
            *p_handled = 1;
            return pack_sf32(r_sign, EXP_MAX as u32, 0);
        }
    } else if b_exp == EXP_MAX {
        if b_mant != 0 {
            if issignan_sf32(a) != 0 || issignan_sf32(b) != 0 {
                *pfflags |= 1u32 << 4;
            }
            *p_handled = 1;
            return F_QNAN32;
        } else {
            *p_handled = 1;
            return pack_sf32(r_sign, 0, 0);
        }
    }

    *p_handled = 0;
    0
}
}

// ── `if (a_exp == ((1 << 11) - 1) … else` · softfp_template.h:603-624 · chunk 3104 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0 || isnan_sf64(b)) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else if (b_exp == ((1 << 11) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else {
//                 return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 11) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf64(a) || issignan_sf64(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//                 return pack_sf64(r_sign, 0, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3104 {
// Ported piece of div_sf64 (softfp_template.h lines 603-624).
use core::ffi::c_int;

// static const uint64_t F_QNAN64 (inlined: no linker symbol)
const F_QNAN64: u64 = (0x7ffu64 << 52) | (1u64 << 51);

extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

// static inline pack_sf64, inlined here
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

// Returns the value the C caller must return; *pret is set to 1 when the
// original code performed an early `return`, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn div_sf64__part3104(
    a: u64,
    b: u64,
    pfflags: *mut u32,
    r_sign: u32,
    a_exp: i32,
    b_exp: i32,
    a_mant: u64,
    b_mant: u64,
    pret: *mut c_int,
) -> u64 {
    if a_exp == (1 << 11) - 1 {
        if a_mant != 0 || isnan_sf64(b) != 0 {
            if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 {
                *pfflags |= 1 << 4;
            }
            *pret = 1;
            return F_QNAN64;
        } else if b_exp == (1 << 11) - 1 {
            *pfflags |= 1 << 4;
            *pret = 1;
            return F_QNAN64;
        } else {
            *pret = 1;
            return pack_sf64(r_sign, (1 << 11) - 1, 0);
        }
    } else if b_exp == (1 << 11) - 1 {
        if b_mant != 0 {
            if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 {
                *pfflags |= 1 << 4;
            }
            *pret = 1;
            return F_QNAN64;
        } else {
            *pret = 1;
            return pack_sf64(r_sign, 0, 0);
        }
    }
    *pret = 0;
    0
}
}

// ── `if (a_exp == ((1 << 15) - 1) … else` · softfp_template.h:603-624 · chunk 3228 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0 || isnan_sf128(b)) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else if (b_exp == ((1 << 15) - 1)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else {
//                 return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//             }
//         } else if (b_exp == ((1 << 15) - 1)) {
//             if (b_mant != 0) {
//                 if (issignan_sf128(a) || issignan_sf128(b)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//                 return pack_sf128(r_sign, 0, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3228 {
// Ported piece of div_sf128: the NaN / infinite-operand handling block.
use core::ffi::{c_int};

const EXP_MAX: i32 = 0x7fff;
// F_QNAN128 == (((uint128_t)((1 << 15) - 1) << 112) | ((uint128_t)1 << (112 - 1)))
const F_QNAN128: u128 = ((EXP_MAX as u128) << 112) | (1u128 << 111);

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127u32)
        | ((a_exp as u128) << 112u32)
        | (a_mant & (((1u128 << 112u32) - 1) as u128))
}

extern "C" {
    fn isnan_sf128(a: u128) -> c_int;
    fn issignan_sf128(a: u128) -> c_int;
}

/// Returns 1 (and stores the value to return in *presult) when the
/// special-case handling applies, 0 when the caller must continue.
#[no_mangle]
pub extern "C" fn div_sf128__part3228(
    a: u128,
    b: u128,
    a_exp: i32,
    a_mant: u128,
    b_exp: i32,
    b_mant: u128,
    r_sign: u32,
    pfflags: *mut u32,
    presult: *mut u128,
) -> c_int {
    unsafe {
        if a_exp == EXP_MAX {
            if a_mant != 0 || isnan_sf128(b) != 0 {
                if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *presult = F_QNAN128;
                return 1;
            } else if b_exp == EXP_MAX {
                *pfflags |= 1u32 << 4;
                *presult = F_QNAN128;
                return 1;
            } else {
                *presult = pack_sf128(r_sign, EXP_MAX as u32, 0);
                return 1;
            }
        } else if b_exp == EXP_MAX {
            if b_mant != 0 {
                if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                    *pfflags |= 1u32 << 4;
                }
                *presult = F_QNAN128;
                return 1;
            } else {
                *presult = pack_sf128(r_sign, 0, 0);
                return 1;
            }
        }
        0
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:626-640 · chunk 2993 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN32;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf32(r_sign, ((1 << 8) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf32(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2993 {
// Port of div_sf32 lines 626-640: denominator-subnormal normalization and
// division-by-zero special cases.

static F_QNAN32: u32 = ((0xFFu32) << 23) | (1u32 << 22);

fn clz32(a: u32) -> i32 {
    a.leading_zeros() as i32
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

#[no_mangle]
pub extern "C" fn div_sf32__part2993(
    r_sign: u32,
    a_exp: i32,
    a_mant: u32,
    pb_exp: *mut i32,
    pb_mant: *mut u32,
    pfflags: *mut u32,
    pdone: *mut i32,
) -> u32 {
    unsafe {
        *pdone = 0;
        if *pb_exp == 0 {
            if *pb_mant == 0 {
                if a_exp == 0 && a_mant == 0 {
                    *pfflags |= 1u32 << 4;
                    *pdone = 1;
                    return F_QNAN32;
                } else {
                    *pfflags |= 1u32 << 3;
                    *pdone = 1;
                    return pack_sf32(r_sign, 0xFF, 0);
                }
            }
            // normalize_subnormal_sf32(&b_exp, b_mant) inlined
            let mant: u32 = *pb_mant;
            let shift: i32 = 23 - ((32 - 1) - clz32(mant));
            *pb_exp = 1 - shift;
            *pb_mant = mant << shift;
        } else {
            *pb_mant |= 1u32 << 23;
        }
        0
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:626-640 · chunk 3105 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN64;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf64(r_sign, ((1 << 11) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf64(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3105 {
// Piece of div_sf64: subnormal/normal normalization of the divisor mantissa,
// plus the division-by-zero (and 0/0) early returns.
#![allow(non_snake_case)]

use core::ffi::c_int;

const F_QNAN64: u64 = (((11u64 - 1) << 52) | (1u64 << (52 - 1)));

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// static inline in the original C: ported here as a private Rust fn.
#[inline]
fn normalize_subnormal_sf64(pa_exp: *mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - ((64 - 1 - clz64(a_mant)));
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl(shift as u32)
}

/// Returns 0 when the caller must continue, 1 when the caller must return
/// the value stored in *pret.
#[no_mangle]
pub extern "C" fn div_sf64__part3105(
    a_exp: c_int,
    a_mant: u64,
    r_sign: u32,
    pb_exp: *mut i32,
    pb_mant: *mut u64,
    pfflags: *mut u32,
    pret: *mut u64,
) -> c_int {
    unsafe {
        let b_exp: i32 = *pb_exp;
        let mut b_mant: u64 = *pb_mant;

        if b_exp == 0 {
            if b_mant == 0 {
                if a_exp == 0 && a_mant == 0 {
                    *pfflags |= 1u32 << 4;
                    *pret = F_QNAN64;
                    return 1;
                } else {
                    *pfflags |= 1u32 << 3;
                    *pret = pack_sf64(r_sign, ((1u32 << 11) - 1), 0);
                    return 1;
                }
            }
            b_mant = normalize_subnormal_sf64(pb_exp, b_mant);
            *pb_mant = b_mant;
        } else {
            b_mant |= 1u64 << 52;
            *pb_mant = b_mant;
        }
        0
    }
}
}

// ── `if (b_exp == 0) … else` · softfp_template.h:626-640 · chunk 3229 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b_exp == 0) {
//             if (b_mant == 0) {
//     
//                 if (a_exp == 0 && a_mant == 0) {
//                     *pfflags |= (1 << 4);
//                     return F_QNAN128;
//                 } else {
//                     *pfflags |= (1 << 3);
//                     return pack_sf128(r_sign, ((1 << 15) - 1), 0);
//                 }
//             }
//             b_mant = normalize_subnormal_sf128(&b_exp, b_mant);
//         } else {
//             b_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3229 {
// Port of the division-by-zero / subnormal-b-denormalization part of div_sf128.
// Returns 1 when the enclosing C function must return *presult,
// 0 when it must continue with the updated *pb_exp / *pb_mant.

use core::ffi::c_int;

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << 111);

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

// static inline of the C unit: implemented here as a private helper.
#[inline]
fn normalize_subnormal_sf128(pa_exp: i32, a_mant: u128) -> (i32, u128) {
    let shift: i32 = 112 - ((128 - 1) - clz128(a_mant));
    let new_exp: i32 = 1 - shift;
    let mant = a_mant.wrapping_shl(shift as u32);
    (new_exp, mant)
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

#[no_mangle]
pub extern "C" fn div_sf128__part3229(
    r_sign: u32,
    a_exp: i32,
    a_mant: u128,
    pb_exp: *mut i32,
    pb_mant: *mut u128,
    pfflags: *mut u32,
    presult: *mut u128,
) -> c_int {
    let mut b_exp: i32 = unsafe { *pb_exp };
    let mut b_mant: u128 = unsafe { *pb_mant };

    if b_exp == 0 {
        if b_mant == 0 {
            if a_exp == 0 && a_mant == 0 {
                unsafe {
                    *pfflags |= 1u32 << 4;
                    *presult = F_QNAN128;
                }
                return 1;
            } else {
                unsafe {
                    *pfflags |= 1u32 << 3;
                    *presult = pack_sf128(r_sign, (1u32 << 15) - 1, 0);
                }
                return 1;
            }
        }
        let (ne, nm) = normalize_subnormal_sf128(b_exp, b_mant);
        b_exp = ne;
        b_mant = nm;
    } else {
        b_mant |= 1u128 << 112;
    }

    unsafe {
        *pb_exp = b_exp;
        *pb_mant = b_mant;
    }
    0
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:641-647 · chunk 2994 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2994 {
// Port of the a-operand preparation piece of div_sf32 (softfp_template.h).
// normalize_subnormal_sf32, clz32 and pack_sf32 are static (inline) C helpers
// without linker symbols, so their logic is mirrored here.

use core::ffi::c_int;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - ((32 - 1) - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

/// Returns 1 (and stores the value to return from the C function in *pret)
/// when the C code must return early, 0 otherwise. On the non-early paths the
/// updated a_exp / a_mant are written back through the pointers.
#[no_mangle]
pub unsafe extern "C" fn div_sf32__part2994(
    a_exp: *mut i32,
    a_mant: *mut u32,
    r_sign: u32,
    pret: *mut u32,
) -> c_int {
    let aexp = *a_exp;
    let amant = *a_mant;
    if aexp == 0 {
        if amant == 0 {
            *pret = pack_sf32(r_sign, 0, 0);
            return 1;
        }
        let m = normalize_subnormal_sf32(&mut *a_exp, amant);
        *a_mant = m;
    } else {
        *a_mant = amant | (1u32 << 23);
    }
    0
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:641-647 · chunk 3106 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3106 {
// Port of the a-operand normalization step of div_sf64 (softfp_template.h 641-647).
// pack_sf64 / normalize_subnormal_sf64 / clz64 are static (inline) helpers with no
// linker symbol, so their logic is reproduced here privately.

use core::ffi::c_int;

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - (64 - 1 - clz64(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

/// Returns the value to `return` when `*p_early` is set to 1; otherwise
/// `*pa_exp` / `*pa_mant` hold the updated operand and `*p_early` is 0.
#[no_mangle]
pub extern "C" fn div_sf64__part3106(
    pa_exp: *mut i32,
    pa_mant: *mut u64,
    r_sign: u32,
    p_early: *mut c_int,
) -> u64 {
    unsafe {
        let mut a_exp: i32 = *pa_exp;
        let mut a_mant: u64 = *pa_mant;

        if a_exp == 0 {
            if a_mant == 0 {
                *p_early = 1;
                return pack_sf64(r_sign, 0, 0);
            }
            a_mant = normalize_subnormal_sf64(&mut a_exp, a_mant);
        } else {
            a_mant |= 1u64 << 52;
        }

        *pa_exp = a_exp;
        *pa_mant = a_mant;
        *p_early = 0;
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:641-647 · chunk 3230 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(r_sign, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3230 {
// Port of the `a`-operand handling block of div_sf128 (softfp_template.h 641-647).
// Mirrors static-inline helpers clz128 / normalize_subnormal_sf128 / pack_sf128.

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

// static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)
fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: u128) -> u128 {
    let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
    unsafe {
        *pa_exp = 1i32.wrapping_sub(shift);
    }
    if shift >= 0 && shift < 128 {
        a_mant.wrapping_shl(shift as u32)
    } else {
        0
    }
}

// int return: 1 => caller must return *pret (pack_sf128(r_sign, 0, 0)); 0 => continue.
#[no_mangle]
pub extern "C" fn div_sf128__part3230(
    a_exp: *mut i32,
    a_mant: *mut u128,
    r_sign: u32,
    pret: *mut u128,
) -> core::ffi::c_int {
    unsafe {
        if *a_exp == 0 {
            if *a_mant == 0 {
                // pack_sf128(r_sign, 0, 0)
                *pret = (r_sign as u128) << 127;
                return 1;
            }
            *a_mant = normalize_subnormal_sf128(a_exp, *a_mant);
        } else {
            *a_mant |= 1u128 << 112;
        }
    }
    0
}
}

// ── `r_exp = a_exp - b_exp + (1 << (8 - 1)) - 1; r_mant = divrem_u32(&r, a_mant, 0, b_mant << 2);` · softfp_template.h:648-649 · chunk 2995 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp - b_exp + (1 << (8 - 1)) - 1;
//         r_mant = divrem_u32(&r, a_mant, 0, b_mant << 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2995 {
// Ported piece of div_sf32: exponent computation and 32/32 division step.
// divrem_u32 is a static C helper with no linker symbol, so its logic is
// reproduced here as a private Rust function.

#[inline]
fn divrem_u32(pr: *mut u32, ah: u32, al: u32, b: u32) -> u32 {
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    let bb: u32 = b;
    // C would be undefined behaviour for a zero divisor; avoid a Rust panic.
    if bb == 0 {
        unsafe {
            *pr = 0;
        }
        return 0;
    }
    let bb64 = bb as u64;
    unsafe {
        *pr = (a % bb64) as u32;
    }
    (a / bb64) as u32
}

#[no_mangle]
pub extern "C" fn div_sf32__part2995(
    a_exp: i32,
    b_exp: i32,
    a_mant: u32,
    b_mant: u32,
    pr_exp: *mut i32,
    pr_mant: *mut u32,
    pr: *mut u32,
) {
    let r_exp: i32 = a_exp.wrapping_sub(b_exp).wrapping_add(1i32 << (8 - 1));
    let b_shifted: u32 = b_mant.wrapping_shl(2);
    let r_mant: u32 = divrem_u32(pr, a_mant, 0, b_shifted);
    unsafe {
        *pr_exp = r_exp;
        *pr_mant = r_mant;
    }
}
}

// ── `r_exp = a_exp - b_exp + (1 << (11 - 1)) - 1; r_mant = divrem_u64(&r, a_mant, 0, b_mant << 2);` · softfp_template.h:648-649 · chunk 3107 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp - b_exp + (1 << (11 - 1)) - 1;
//         r_mant = divrem_u64(&r, a_mant, 0, b_mant << 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3107 {
// Piece of div_sf64: computes the result exponent and the quotient mantissa,
// leaving the 128-bit division remainder in *r for the sticky bit.
#![allow(non_snake_case)]

use core::ffi::c_int;

// divrem_u64 with ah == 0: exact port of the static C helper's arithmetic.
#[inline]
fn divrem_u64(a: u128, b: u128) -> (u64, u64) {
    if b == 0 {
        return (0, 0);
    }
    ((a / b) as u64, (a % b) as u64)
}

#[no_mangle]
pub extern "C" fn div_sf64__part3107(
    a_exp: c_int,
    b_exp: c_int,
    a_mant: u64,
    b_mant: u64,
    r_exp: *mut c_int,
    pr: *mut u64,
) -> u64 {
    // r_exp = a_exp - b_exp + (1 << (11 - 1)) - 1;
    let rexp = a_exp
        .wrapping_sub(b_exp)
        .wrapping_add(1 << (11 - 1))
        .wrapping_sub(1);
    unsafe {
        *r_exp = rexp;
    }

    // r_mant = divrem_u64(&r, a_mant, 0, b_mant << 2);
    let a: u128 = a_mant as u128;
    let b: u128 = (b_mant << 2) as u128;
    let (quot, rem) = divrem_u64(a, b);
    unsafe {
        *pr = rem;
    }
    quot
}
}

// ── `r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1; r_mant = divrem_u128(&r, a_mant, 0, b_mant << 2);` · softfp_template.h:648-649 · chunk 3231 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1;
//         r_mant = divrem_u128(&r, a_mant, 0, b_mant << 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3231 {
// Port of `div_sf128__part3231`: computes the result exponent and the
// mantissa/remainder of the 128-bit significand division
// (a "static" divrem helper is inlined here as private Rust logic).
//
// The preprocessed C TARGET is:
//     r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1;
//     r_mant = divrem_u128(&r, a_mant, 0, b_mant << 2);
//
// divrem_u128(&rem, a1, a0, b) does a 128-iteration restoring division of the
// 256-bit value (a1, a0) by b, leaving the 128-bit quotient in a0 and the
// remainder in a1. Here a0 == 0 and a1 == a_mant < (b_mant << 2), so the
// quotient fits in 128 bits and a1 ends up holding the remainder.

#[no_mangle]
pub extern "C" fn div_sf128__part3231(
    pr_exp: *mut i32,
    pr_mant: *mut u128,
    pr_rem: *mut u128,
    a_exp: i32,
    b_exp: i32,
    a_mant: u128,
    b_mant: u128,
) {
    unsafe {
        // r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1;
        let r_exp: i32 = a_exp
            .wrapping_sub(b_exp)
            .wrapping_add(1i32 << 14)
            .wrapping_sub(1);
        *pr_exp = r_exp;

        // divrem_u128(&r, a_mant, 0, b_mant << 2);
        let b: u128 = b_mant << 2;
        let mut a1: u128 = a_mant;
        let mut a0: u128 = 0u128;
        let mut i: i32 = 0;
        while i < 128 {
            let ab: u128 = a1 >> 127;
            a1 = (a1 << 1) | (a0 >> 127);
            let qb: u128;
            if ab != 0 || a1 >= b {
                a1 = a1.wrapping_sub(b);
                qb = 1;
            } else {
                qb = 0;
            }
            a0 = (a0 << 1) | qb;
            i += 1;
        }
        *pr_rem = a1;
        *pr_mant = a0;
    }
}
}

// ── `if (r != 0)` · softfp_template.h:650-651 · chunk 2996 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r != 0)
//             r_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2996 {
// Port of the sticky-bit step of div_sf32: if the remainder is nonzero,
// set the low bit of the quotient mantissa.
#[no_mangle]
pub extern "C" fn div_sf32__part2996(r: u32, r_mant: *mut u32) {
    unsafe {
        if r != 0 {
            *r_mant |= 1;
        }
    }
}
}

// ── `if (r != 0)` · softfp_template.h:650-651 · chunk 3108 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r != 0)
//             r_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3108 {
// Port of the sticky-bit fixup from div_sf64: if the remainder is nonzero,
// set the least significant bit of the mantissa.
#[no_mangle]
pub extern "C" fn div_sf64__part3108(r: u64, r_mant: u64) -> u64 {
    if r != 0 {
        r_mant | 1
    } else {
        r_mant
    }
}
}

// ── `if (r != 0)` · softfp_template.h:650-651 · chunk 3232 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r != 0)
//             r_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3232 {
// Port of the sticky-bit fixup from div_sf128:
//     if (r != 0)
//         r_mant |= 1;
// C type uint128_t is `unsigned __int128`, which is ABI-identical to Rust's u128.

#[no_mangle]
pub extern "C" fn div_sf128__part3232(r: u128, r_mant: u128) -> u128 {
    let mut m: u128 = r_mant;
    if r != 0 {
        m |= 1;
    }
    m
}
}

// ── `return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);` · softfp_template.h:652-652 · chunk 2997 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2997 {
// Port of the tail of div_sf32: normalize_sf32(...) together with the static
// helpers it needs (clz32, rshift_rnd32, roundpack_sf32, pack_sf32), which are
// static/inline in C and therefore have no linker symbol.

type RoundingModeEnum = core::ffi::c_int;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RDN: RoundingModeEnum = 2;
const RM_RUP: RoundingModeEnum = 3;
const RM_RMM: RoundingModeEnum = 4;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32.wrapping_shl(d as u32)).wrapping_sub(1);
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as i32;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp;
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // C asserts shift >= 0 here; keep the arithmetic non-panicking.
    a_exp -= shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn div_sf32__part2997(
    r_sign: u32,
    r_exp: core::ffi::c_int,
    r_mant: u32,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u32 {
    normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);` · softfp_template.h:652-652 · chunk 3109 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3109 {
// Port of the tail of div_sf64: normalize_sf64 (a static C function) plus the
// static helpers it needs (clz64, rshift_rnd64, roundpack_sf64, pack_sf64).

use core::ffi::c_int;

#[inline]
fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let addend: u32 = match rm {
        0 | 4 => 1u32 << 5,
        1 => 0u32,
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << 10) - 1
            } else {
                0u32
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63);
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    if rm == 0 && rnd_bits == (1u32 << 5) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> 53) as c_int;
    if a_mant <= (1u64 << 52) - 1 {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let shift: c_int = clz64(a_mant) - (64 - 1 - (64 - 2));
    // assert(shift >= 0): keep the fatal behaviour of the original, no unwinding.
    if shift < 0 {
        std::process::abort();
    }
    a_exp -= shift;
    let a_mant = if shift >= 64 {
        0u64
    } else {
        a_mant << (shift as u32)
    };
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn div_sf64__part3109(
    r_sign: u32,
    r_exp: c_int,
    r_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags)
}
}

// ── `return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);` · softfp_template.h:652-652 · chunk 3233 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3233 {
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes_definitions)]
#![allow(unused_parens)]
#![allow(clippy::all)]

use core::ffi::c_int;

const MANT_MASK: u128 = (1u128 << 112) - 1;
const RND_MASK: u128 = (1u128 << 14) - 1;
const EXP_MAX: c_int = (1 << 15) - 1;

// Port of the static C helper clz128().
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as c_int
        } else {
            al.leading_zeros() as c_int + 64
        }
    }
}

// Port of the static inline C helper pack_sf128().
fn pack_sf128(a_sign: u32, a_exp: c_int, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

// Port of the static C helper rshift_rnd128().
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

// Port of the static C helper roundpack_sf128().
fn roundpack_sf128(a_sign: u32, a_exp: c_int, a_mant: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    let addend: u32 = match rm {
        0 | 4 => 1u32 << 13, // RM_RNE, RM_RMM
        1 => 0,              // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP and the default case
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 14) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || (a_mant.wrapping_add(addend as u128)) < (1u128 << 127);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & RND_MASK) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & RND_MASK) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    if rm == 0 && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> 113) as c_int;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MAX {
        if addend == 0 {
            a_exp = EXP_MAX - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MAX;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp, a_mant)
}

// Port of the static C helper normalize_sf128() (its "shift >= 0" assert is
// always true: clz128() >= 1, so it is omitted here).
#[no_mangle]
pub extern "C" fn div_sf128__part3233(
    r_sign: u32,
    r_exp: i32,
    r_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift = clz128(r_mant) - (128 - 1 - (128 - 2));
    let a_exp = r_exp - shift;
    let a_mant = if shift >= 0 && shift < 128 {
        r_mant << (shift as u32)
    } else {
        0u128
    };
    roundpack_sf128(r_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `sqrtrem_u32` · softfp_template.h:659-685 · chunk 2998 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int sqrtrem_u32(uint32_t *pr, uint32_t ah, uint32_t al)
//     {
//         uint64_t a, u, s;
//         int l, inexact;
//     
//     
//         if (ah != 0) {
//             l = 2 * 32 - clz32(ah - 1);
//         } else {
//             if (al == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 32 - clz32(al - 1);
//         }
//         a = ((uint64_t)ah << 32) | al;
//         u = (uint64_t)1 << ((l + 1) / 2);
//         for(;;) {
//             s = u;
//             u = ((a / s) + s) / 2;
//             if (u >= s)
//                 break;
//         }
//         inexact = (a - s * s) != 0;
//         *pr = s;
//         return inexact;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2998 {
use core::ffi::c_int;

// Static inline clz32 from the C translation unit: count leading zeros of a u32.
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// compute sqrt(a) with a = ah*2^32 + al and a < 2^30
// return true (nonzero) if not an exact square.
#[no_mangle]
pub extern "C" fn sqrtrem_u32(pr: *mut u32, ah: u32, al: u32) -> c_int {
    let a: u64;
    let mut u: u64;
    let mut s: u64 = 0;
    let l: c_int;

    // 2^l >= a
    if ah != 0 {
        l = 2 * 32 - clz32(ah.wrapping_sub(1));
    } else {
        if al == 0 {
            unsafe {
                *pr = 0;
            }
            return 0;
        }
        l = 32 - clz32(al.wrapping_sub(1));
    }

    a = ((ah as u64) << 32) | (al as u64);
    u = 1u64 << (((l + 1) / 2) as u32);
    loop {
        s = u;
        u = (a / s + s) / 2;
        if u >= s {
            break;
        }
    }
    let inexact = a.wrapping_sub(s.wrapping_mul(s)) != 0;
    unsafe {
        *pr = s as u32;
    }
    inexact as c_int
}
}

// ── `sqrtrem_u64` · softfp_template.h:659-685 · chunk 3110 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int sqrtrem_u64(uint64_t *pr, uint64_t ah, uint64_t al)
//     {
//         uint128_t a, u, s;
//         int l, inexact;
//     
//     
//         if (ah != 0) {
//             l = 2 * 64 - clz64(ah - 1);
//         } else {
//             if (al == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 64 - clz64(al - 1);
//         }
//         a = ((uint128_t)ah << 64) | al;
//         u = (uint128_t)1 << ((l + 1) / 2);
//         for(;;) {
//             s = u;
//             u = ((a / s) + s) / 2;
//             if (u >= s)
//                 break;
//         }
//         inexact = (a - s * s) != 0;
//         *pr = s;
//         return inexact;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3110 {
use core::ffi::c_int;

// Port of the C static inline helper clz64 (no linker symbol).
#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn sqrtrem_u64(pr: *mut u64, ah: u64, al: u64) -> c_int {
    // 2^l >= a
    let l: i32;
    if ah != 0 {
        l = 2 * 64 - clz64(ah.wrapping_sub(1));
    } else {
        if al == 0 {
            unsafe {
                *pr = 0;
            }
            return 0;
        }
        l = 64 - clz64(al.wrapping_sub(1));
    }
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    let mut u: u128 = 1u128 << (((l + 1) / 2) as u32);
    let mut s: u128 = 0;
    loop {
        s = u;
        u = ((a / s) + s) / 2;
        if u >= s {
            break;
        }
    }
    let inexact = a.wrapping_sub(s.wrapping_mul(s)) != 0;
    unsafe {
        *pr = s as u64;
    }
    if inexact {
        1
    } else {
        0
    }
}
}

// ── `if (ah != 0) … else` · softfp_template.h:665-673 · chunk 2999 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ah != 0) {
//             l = 2 * 32 - clz32(ah - 1);
//         } else {
//             if (al == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 32 - clz32(al - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2999 {
// Port of the leading-zero part of sqrtrem_u32 (softfp_template.h lines 665-673).
// clz32 is `static inline` in C, so its logic is re-implemented here privately.

use core::ffi::c_int;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// Returns 1 when the C caller must `return 0` immediately (al == 0 && ah == 0),
// otherwise 0 after storing the bit length in *pl.
#[no_mangle]
pub extern "C" fn sqrtrem_u32__part2999(
    pr: *mut u32,
    ah: u32,
    al: u32,
    pl: *mut c_int,
) -> c_int {
    if ah != 0 {
        let l: c_int = 2 * 32 - clz32(ah - 1);
        unsafe {
            *pl = l;
        }
        0
    } else if al == 0 {
        unsafe {
            *pr = 0;
        }
        1
    } else {
        let l: c_int = 32 - clz32(al - 1);
        unsafe {
            *pl = l;
        }
        0
    }
}
}

// ── `if (ah != 0) … else` · softfp_template.h:665-673 · chunk 3111 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ah != 0) {
//             l = 2 * 64 - clz64(ah - 1);
//         } else {
//             if (al == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 64 - clz64(al - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3111 {
// Compute the bit length `l` of the value ((ah << 64) | al), and handle the
// zero-input early exit of the enclosing sqrtrem_u64.
//
// Returns 1 when the enclosing function must `return 0;` (input was zero, in
// which case *pr has been set to 0), 0 otherwise with *pl = l.

fn clz64(a: u64) -> core::ffi::c_int {
    // Mirrors the C `clz64`: count leading zeros, 64 when the input is zero.
    if a == 0 {
        64
    } else {
        a.leading_zeros() as core::ffi::c_int
    }
}

#[no_mangle]
pub extern "C" fn sqrtrem_u64__part3111(
    pr: *mut u64,
    ah: u64,
    al: u64,
    pl: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let l: core::ffi::c_int;
    if ah != 0 {
        l = 2 * 64 - clz64(ah.wrapping_sub(1));
    } else {
        if al == 0 {
            unsafe {
                *pr = 0;
            }
            return 1;
        }
        l = 64 - clz64(al.wrapping_sub(1));
    }
    unsafe {
        *pl = l;
    }
    0
}
}

// ── `a = ((uint64_t)ah << 32) | al; u = (uint64_t)1 << ((l + 1) / 2);` · softfp_template.h:674-675 · chunk 3000 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a = ((uint64_t)ah << 32) | al;
//         u = (uint64_t)1 << ((l + 1) / 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3000 {
use core::ffi::c_int;

// Ported piece of sqrtrem_u32:
//   a = ((uint64_t)ah << 32) | al;
//   u = (uint64_t)1 << ((l + 1) / 2);
#[no_mangle]
pub extern "C" fn sqrtrem_u32__part3000(
    pa: *mut u64,
    pu: *mut u64,
    ah: u32,
    al: u32,
    l: c_int,
) {
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    // C integer division truncates toward zero, same as Rust's i32 `/`.
    let shift: u32 = ((l + 1) / 2) as u32;
    let u: u64 = 1u64.wrapping_shl(shift);

    unsafe {
        if !pa.is_null() {
            *pa = a;
        }
        if !pu.is_null() {
            *pu = u;
        }
    }
}
}

// ── `a = ((uint128_t)ah << 64) | al; u = (uint128_t)1 << ((l + 1) / 2);` · softfp_template.h:674-675 · chunk 3112 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a = ((uint128_t)ah << 64) | al;
//         u = (uint128_t)1 << ((l + 1) / 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3112 {
// Portion of softfp_template.h (sqrtrem_u64) lines 674-675:
//   a = ((uint128_t)ah << 64) | al;
//   u = (uint128_t)1 << ((l + 1) / 2);
// Writes the two temporaries through out-pointers.

#[no_mangle]
pub extern "C" fn sqrtrem_u64__part3112(
    pa: *mut u128,
    pu: *mut u128,
    ah: u64,
    al: u64,
    l: core::ffi::c_int,
) {
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    // C: (uint128_t)1 << ((l + 1) / 2);  l is an int, truncating division.
    let shift: u32 = (((l.wrapping_add(1)) / 2) as u32) & 127;
    let u: u128 = 1u128 << shift;
    unsafe {
        *pa = a;
        *pu = u;
    }
}
}

// ── `for (; ; )` · softfp_template.h:676-681 · chunk 3001 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             s = u;
//             u = ((a / s) + s) / 2;
//             if (u >= s)
//                 break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3001 {
#[no_mangle]
pub extern "C" fn sqrtrem_u32__part3001(a: u64, ps: *mut u64, pu: *mut u64) {
    let mut u: u64 = unsafe { *pu };
    let s: u64 = loop {
        let s = u;
        u = (a / s).wrapping_add(s) / 2;
        if u >= s {
            break s;
        }
    };
    unsafe {
        *ps = s;
        *pu = u;
    }
}
}

// ── `for (; ; )` · softfp_template.h:676-681 · chunk 3113 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             s = u;
//             u = ((a / s) + s) / 2;
//             if (u >= s)
//                 break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3113 {
// Port of the Newton-iteration square-root loop of sqrtrem_u64.
// s is produced as an out-parameter; u and a are taken by value.

#[no_mangle]
pub extern "C" fn sqrtrem_u64__part3113(ps: *mut u128, u_in: u128, a: u128) {
    let mut u: u128 = u_in;
    let mut s: u128 = 0;
    loop {
        s = u;
        // u is always >= 2 here, so the division cannot trap; guard anyway.
        if s == 0 {
            break;
        }
        u = (a / s + s) / 2;
        if u >= s {
            break;
        }
    }
    unsafe {
        *ps = s;
    }
}
}

// ── `inexact = (a - s * s) != 0; *pr = s; return inexact;` · softfp_template.h:682-684 · chunk 3002 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     inexact = (a - s * s) != 0;
//         *pr = s;
//         return inexact;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3002 {
// Portion of sqrtrem_u32: computes the inexact flag and stores the result.
#[no_mangle]
pub extern "C" fn sqrtrem_u32__part3002(
    pr: *mut u32,
    a: u64,
    s: u64,
) -> core::ffi::c_int {
    let diff: u64 = a.wrapping_sub(s.wrapping_mul(s));
    let inexact: core::ffi::c_int = if diff != 0 { 1 } else { 0 };
    unsafe {
        *pr = s as u32;
    }
    inexact
}
}

// ── `inexact = (a - s * s) != 0; *pr = s; return inexact;` · softfp_template.h:682-684 · chunk 3114 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     inexact = (a - s * s) != 0;
//         *pr = s;
//         return inexact;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3114 {
// Port of the tail of sqrtrem_u64: compute inexact flag, store s, return it.
// The 128-bit C values are passed split into high/low 64-bit halves so that no
// `__int128` type has to cross the FFI boundary.

#[no_mangle]
pub extern "C" fn sqrtrem_u64__part3114(
    pr: *mut u64,
    a_hi: u64,
    a_lo: u64,
    s_hi: u64,
    s_lo: u64,
) -> core::ffi::c_int {
    let a: u128 = ((a_hi as u128) << 64) | (a_lo as u128);
    let s: u128 = ((s_hi as u128) << 64) | (s_lo as u128);

    // inexact = (a - s * s) != 0;  (all arithmetic wraps, as unsigned C would)
    let inexact: core::ffi::c_int = if a.wrapping_sub(s.wrapping_mul(s)) != 0 {
        1
    } else {
        0
    };

    // *pr = s;  (uint128_t -> uint64_t conversion keeps the low 64 bits)
    unsafe {
        *pr = s as u64;
    }

    inexact
}
}

// ── `sqrtrem_u128` · softfp_template.h:689-716 · chunk 3234 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int sqrtrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0)
//     {
//         int l, inexact;
//         uint128_t u, s, r, q, sq0, sq1;
//     
//     
//         if (a1 != 0) {
//             l = 2 * 128 - clz128(a1 - 1);
//         } else {
//             if (a0 == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 128 - clz128(a0 - 1);
//         }
//         u = (uint128_t)1 << ((l + 1) / 2);
//         for(;;) {
//             s = u;
//             q = divrem_u128(&r, a1, a0, s);
//             u = (q + s) / 2;
//             if (u >= s)
//                 break;
//         }
//         sq1 = mul_u128(&sq0, s, s);
//         inexact = (sq0 != a0 || sq1 != a1);
//         *pr = s;
//         return inexact;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3234 {
// Port of the static C function sqrtrem_u128 (softfp_template.h lines 689-716,
// 128-bit instantiation).  The helpers clz128, divrem_u128 and mul_u128 are
// `static` in the C original, so their logic is reproduced here privately.

use core::ffi::c_int;

#[inline]
fn clz128(a: u128) -> c_int {
    // C: clz128(0) == 128, otherwise the number of leading zero bits.
    a.leading_zeros() as c_int
}

// static uint128_t divrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0, uint128_t b)
// Returns (quotient, remainder); the C version stores the remainder through pr.
fn divrem_u128(a1: u128, a0: u128, b: u128) -> (u128, u128) {
    let mut a1 = a1;
    let mut a0 = a0;
    let mut i: c_int = 0;
    while i < 128 {
        let ab: u128 = a1 >> (128 - 1);
        a1 = (a1 << 1) | (a0 >> (128 - 1));
        let qb: u128;
        if ab != 0 || a1 >= b {
            a1 = a1.wrapping_sub(b);
            qb = 1;
        } else {
            qb = 0;
        }
        a0 = (a0 << 1) | qb;
        i += 1;
    }
    // C returns a0 (quotient) and stores a1 (remainder) through *pr.
    (a0, a1)
}

// static uint128_t mul_u128(uint128_t *plow, uint128_t a, uint128_t b)
// Returns the high 128 bits of a*b; the low 128 bits come back first in the tuple.
fn mul_u128(a: u128, b: u128) -> (u128, u128) {
    let a0: u64 = a as u64;
    let a1: u64 = (a >> (128 / 2)) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> (128 / 2)) as u64;

    let r00: u128 = (a0 as u128) * (b0 as u128);
    let r01: u128 = (a0 as u128) * (b1 as u128);
    let r10: u128 = (a1 as u128) * (b0 as u128);
    let r11: u128 = (a1 as u128) * (b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> (128 / 2)) + ((r01 as u64) as u128) + ((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> (128 / 2)) + (r01 >> (128 / 2)) + (r10 >> (128 / 2)) + ((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> (128 / 2)) + (r11 >> (128 / 2))) as u64;

    let low: u128 = ((r1 as u128) << (128 / 2)) | (r0 as u128);
    let high: u128 = ((r3 as u128) << (128 / 2)) | (r2 as u128);
    (low, high)
}

#[no_mangle]
pub extern "C" fn sqrtrem_u128(pr: *mut u128, a1: u128, a0: u128) -> c_int {
    let l: c_int;

    // 2^l >= a
    if a1 != 0 {
        l = 2 * 128 - clz128(a1.wrapping_sub(1));
    } else {
        if a0 == 0 {
            unsafe {
                *pr = 0;
            }
            return 0;
        }
        l = 128 - clz128(a0.wrapping_sub(1));
    }

    let mut u: u128 = (1u128 as u128).wrapping_shl(((l + 1) / 2) as u32);
    let mut s: u128 = 0;
    loop {
        s = u;
        let (q, _r) = divrem_u128(a1, a0, s);
        u = (q.wrapping_add(s)) / 2;
        if u >= s {
            break;
        }
    }

    let (sq0, sq1) = mul_u128(s, s);
    let inexact: c_int = if sq0 != a0 || sq1 != a1 { 1 } else { 0 };
    unsafe {
        *pr = s;
    }
    inexact
}
}

// ── `if (a1 != 0) … else` · softfp_template.h:695-703 · chunk 3235 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a1 != 0) {
//             l = 2 * 128 - clz128(a1 - 1);
//         } else {
//             if (a0 == 0) {
//                 *pr = 0;
//                 return 0;
//             }
//             l = 128 - clz128(a0 - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3235 {
use core::ffi::c_int;

// Port of the static inline helper clz128 from the translation unit.
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

// Computes the number of significant bits `l` for sqrtrem_u128.
// Returns 1 (and stores 0 in *pr) when the input is zero, meaning the
// caller must return 0; otherwise stores `l` in *pl and returns 0.
#[no_mangle]
pub extern "C" fn sqrtrem_u128__part3235(
    a1: u128,
    a0: u128,
    pl: *mut c_int,
    pr: *mut u128,
) -> c_int {
    let l: c_int;
    if a1 != 0 {
        l = 2 * 128 - clz128(a1.wrapping_sub(1));
    } else {
        if a0 == 0 {
            unsafe {
                *pr = 0;
            }
            return 1;
        }
        l = 128 - clz128(a0.wrapping_sub(1));
    }
    unsafe {
        *pl = l;
    }
    0
}
}

// ── `u = (uint128_t)1 << ((l + 1) / 2);` · softfp_template.h:704-704 · chunk 3236 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     u = (uint128_t)1 << ((l + 1) / 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3236 {
// Port of: u = (uint128_t)1 << ((l + 1) / 2);
// Reads `l` (int) and produces the uint128_t value assigned to `u`.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sqrtrem_u128__part3236(l: c_int) -> u128 {
    // (l + 1) / 2 with non-negative l: truncating division, same as C.
    let shift = ((l.wrapping_add(1)) / 2) as u32;
    // Guard against shifts >= 128 (undefined in C); mask like a wrapping shift
    // so nothing can panic across the FFI boundary.
    1u128.wrapping_shl(shift)
}
}

// ── `for (; ; )` · softfp_template.h:705-711 · chunk 3237 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             s = u;
//             q = divrem_u128(&r, a1, a0, s);
//             u = (q + s) / 2;
//             if (u >= s)
//                 break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3237 {
// Port of the Newton-iteration loop of sqrtrem_u128 (softfp_template.h:705-711).
// divrem_u128 is a `static` C function without a linker symbol, so its logic
// is ported here as a private Rust fn.

use core::ffi::{c_char, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

// Faithful port of:
//   static uint128_t divrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0, uint128_t b)
// returns (quotient, remainder)
#[inline]
fn divrem_u128(mut a1: u128, mut a0: u128, b: u128) -> (u128, u128) {
    if a1 >= b {
        unsafe {
            __assert_fail(
                b"a1 < b\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                571,
                b"uint128_t divrem_u128(uint128_t *, uint128_t, uint128_t, uint128_t)\0"
                    .as_ptr() as *const c_char,
            )
        }
    }
    let mut i: i32 = 0;
    while i < 128 {
        let ab: u128 = a1 >> (128 - 1);
        a1 = (a1 << 1) | (a0 >> (128 - 1));
        let qb: u128;
        if ab != 0 || a1 >= b {
            a1 = a1.wrapping_sub(b);
            qb = 1;
        } else {
            qb = 0;
        }
        a0 = (a0 << 1) | qb;
        i += 1;
    }
    (a0, a1)
}

// The TARGET:
//   for(;;) {
//       s = u;
//       q = divrem_u128(&r, a1, a0, s);
//       u = (q + s) / 2;
//       if (u >= s)
//           break;
//   }
// `u` is updated by the loop but not read afterwards; `q` and `r` are dead.
// The only value live after the loop is `s`, which is returned.
#[no_mangle]
pub extern "C" fn sqrtrem_u128__part3237(u_in: u128, a1: u128, a0: u128) -> u128 {
    let mut u: u128 = u_in;
    let mut s: u128 = u_in;
    loop {
        s = u;
        let (q, _r) = divrem_u128(a1, a0, s);
        u = (q.wrapping_add(s)) / 2;
        if u >= s {
            break;
        }
    }
    s
}
}

// ── `sq1 = mul_u128(&sq0, s, s); inexact = (sq0 != a0 || sq1 != a1); *pr = s; return inexact;` · softfp_template.h:712-715 · chunk 3238 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sq1 = mul_u128(&sq0, s, s);
//         inexact = (sq0 != a0 || sq1 != a1);
//         *pr = s;
//         return inexact;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3238 {
// Port of the tail of sqrtrem_u128 (softfp_template.h lines 712-715):
//   sq1 = mul_u128(&sq0, s, s);
//   inexact = (sq0 != a0 || sq1 != a1);
//   *pr = s;
//   return inexact;
//
// mul_u128 is `static` in the C translation unit, so it has no linker
// symbol; its logic is mirrored here as a private Rust function.

// Private mirror of the static C helper mul_u128 (128x128 -> 256 multiply).
fn mul_u128(plow: *mut u128, a: u128, b: u128) -> u128 {
    // uint64_t a0, a1, b0, b1, r0, r1, r2, r3;
    // uint128_t r00, r01, r10, r11, c;
    let a0: u64 = a as u64;
    let a1: u64 = (a >> 64) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> 64) as u64;

    let r00: u128 = (a0 as u128) * (b0 as u128);
    let r01: u128 = (a0 as u128) * (b1 as u128);
    let r10: u128 = (a1 as u128) * (b0 as u128);
    let r11: u128 = (a1 as u128) * (b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> 64)
        .wrapping_add((r01 as u64) as u128)
        .wrapping_add((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> 64)
        .wrapping_add(r01 >> 64)
        .wrapping_add(r10 >> 64)
        .wrapping_add((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let r3: u64 = ((c >> 64).wrapping_add(r11 >> 64)) as u64;

    unsafe {
        *plow = ((r1 as u128) << 64) | (r0 as u128);
    }
    ((r3 as u128) << 64) | (r2 as u128)
}

#[no_mangle]
pub extern "C" fn sqrtrem_u128__part3238(
    pr: *mut u128,
    a1: u128,
    a0: u128,
    s: u128,
) -> core::ffi::c_int {
    let mut sq0: u128 = 0;
    let sq1: u128 = mul_u128(&mut sq0 as *mut u128, s, s);
    let inexact: core::ffi::c_int = (sq0 != a0 || sq1 != a1) as core::ffi::c_int;
    unsafe {
        *pr = s;
    }
    inexact
}
}

// ── `sqrt_sf32` · softfp_template.h:720-767 · chunk 3003 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t sqrt_sf32(uint32_t a, RoundingModeEnum rm,
//                    uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
//         if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN32;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(0, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         a_exp -= ((1 << 8) - 1) / 2;
//     
//         if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
//         a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2;
//         a_mant <<= (32 - 4 - 23);
//         if (sqrtrem_u32(&a_mant, a_mant, 0))
//             a_mant |= 1;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3003 {
// Port of sqrt_sf32 (softfp_template.h) to Rust.
// The static helpers of the C translation unit (clz32, pack_sf32,
// normalize_subnormal_sf32, normalize_sf32, roundpack_sf32, rshift_rnd32,
// sqrtrem_u32, F_QNAN32) have no linker symbols, so their logic is
// reimplemented here as private Rust functions.

use core::ffi::c_int;

// C: typedef enum { RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM } RoundingModeEnum;
const RM_RNE: c_int = 0;

const F_QNAN32: u32 = (0xffu32 << 23) | (1u32 << 22);

extern "C" {
    // BOOL issignan_sf32(uint32_t a)
    fn issignan_sf32(a: u32) -> c_int;
}

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

#[inline]
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            (a != 0) as u32
        } else {
            let du = d as u32;
            let mask = (1u32 << du) - 1;
            (a >> du) | ((a & mask) != 0) as u32
        }
    } else {
        a
    }
}

fn roundpack_sf32(a_sign: u32, a_exp: i32, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    let addend: u32 = match rm {
        RM_RNE => 1u32 << (((32 - 2) - 23) - 1),
        4 => 1u32 << (((32 - 2) - 23) - 1), // RM_RMM
        1 => 0,                             // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as i32;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp: i32, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let shift = clz32(a_mant) - (32 - 1 - (32 - 2));
    // C asserts shift >= 0 here.
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift = 23 - ((32 - 1) - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

fn sqrtrem_u32(pr: &mut u32, ah: u32, al: u32) -> c_int {
    let l: i32;

    if ah != 0 {
        l = 2 * 32 - clz32(ah - 1);
    } else {
        if al == 0 {
            *pr = 0;
            return 0;
        }
        l = 32 - clz32(al - 1);
    }
    let a: u64 = ((ah as u64) << 32) | (al as u64);
    let mut u: u64 = 1u64.wrapping_shl(((l + 1) / 2) as u32);
    let mut s: u64 = u;
    loop {
        s = u;
        u = ((a / s) + s) / 2;
        if u >= s {
            break;
        }
    }
    let inexact: c_int = (a.wrapping_sub(s.wrapping_mul(s)) != 0) as c_int;
    *pr = s as u32;
    inexact
}

#[no_mangle]
pub extern "C" fn sqrt_sf32(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let a_sign: u32 = a >> (32 - 1);
    let mut a_exp: i32 = ((a >> 23) & ((1 << 8) - 1)) as i32;
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);

    if a_exp == ((1 << 8) - 1) {
        if a_mant != 0 {
            if unsafe { issignan_sf32(a) } != 0 {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN32;
        } else if a_sign != 0 {
            // equivalent of C's `goto neg_error`
            unsafe {
                *pfflags |= 1 << 4;
            }
            return F_QNAN32;
        } else {
            return a; // +infinity
        }
    }
    if a_sign != 0 {
        if a_exp == 0 && a_mant == 0 {
            return a; // -zero
        }
        unsafe {
            *pfflags |= 1 << 4;
        }
        return F_QNAN32;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf32(0, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u32 << 23;
    }
    a_exp -= ((1 << 8) - 1) / 2;

    // simpler to handle an even exponent
    if (a_exp & 1) != 0 {
        a_exp -= 1;
        a_mant = a_mant.wrapping_shl(1);
    }
    a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2;
    a_mant = a_mant.wrapping_shl((32 - 4 - 23) as u32);
    let ah = a_mant;
    if sqrtrem_u32(&mut a_mant, ah, 0) != 0 {
        a_mant |= 1;
    }
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `sqrt_sf64` · softfp_template.h:720-767 · chunk 3115 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t sqrt_sf64(uint64_t a, RoundingModeEnum rm,
//                    uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
//         if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN64;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(0, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         a_exp -= ((1 << 11) - 1) / 2;
//     
//         if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
//         a_exp = (a_exp >> 1) + ((1 << 11) - 1) / 2;
//         a_mant <<= (64 - 4 - 52);
//         if (sqrtrem_u64(&a_mant, a_mant, 0))
//             a_mant |= 1;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3115 {
// Port of sqrt_sf64 (softfp_template.h, 64-bit soft-float square root).
// The helper functions used by the original (pack_sf64, normalize_sf64,
// roundpack_sf64, rshift_rnd64, clz64, normalize_subnormal_sf64,
// sqrtrem_u64, issignan_sf64) are `static`/`static inline` in the C
// translation unit, so their logic is reproduced here as private fns.

use core::ffi::c_int;

const EXP_MASK: i32 = (1 << 11) - 1; // 2047
const MANT_MASK: u64 = ((1u64 << 52) - 1);
const F_QNAN64: u64 = (((1u64 << 11) - 1) << 52) | (1u64 << (52 - 1));

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & MANT_MASK)
}

#[inline]
fn issignan_sf64(a: u64) -> bool {
    let a_exp1: u32 = ((a >> (52 - 1)) & (((1u64 << (11 + 1)) - 1) as u64)) as u32;
    let a_mant: u64 = a & MANT_MASK;
    a_exp1 == (2 * ((1u32 << 11) - 1)) && a_mant != 0
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (default)
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant.wrapping_add(addend as u64)) < (1u64 << (64 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = (a_mant.wrapping_add(addend as u64)) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[inline]
fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - ((64 - 1 - clz64(a_mant)));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

// Returns (root, inexact); the original writes the root through `pr`
// before returning the inexact flag.
fn sqrtrem_u64(ah: u64, al: u64) -> (u64, i32) {
    let l: i32;
    if ah != 0 {
        l = 2 * 64 - clz64(ah.wrapping_sub(1));
    } else {
        if al == 0 {
            return (0, 0);
        }
        l = 64 - clz64(al.wrapping_sub(1));
    }
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    let mut u: u128 = 1u128 << (((l + 1) / 2) as u32);
    let mut s: u128;
    loop {
        s = u;
        u = ((a / s) + s) / 2;
        if u >= s {
            break;
        }
    }
    let inexact: i32 = (a.wrapping_sub(s.wrapping_mul(s)) != 0) as i32;
    (s as u64, inexact)
}

#[no_mangle]
pub extern "C" fn sqrt_sf64(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u64;

    a_sign = (a >> (64 - 1)) as u32;
    a_exp = ((a >> 52) & (EXP_MASK as u64)) as i32;
    a_mant = a & MANT_MASK;
    if a_exp == EXP_MASK {
        if a_mant != 0 {
            if issignan_sf64(a) {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN64;
        } else if a_sign != 0 {
            // goto neg_error
            unsafe {
                *pfflags |= 1 << 4;
            }
            return F_QNAN64;
        } else {
            return a; // +infinity
        }
    }
    if a_sign != 0 {
        if a_exp == 0 && a_mant == 0 {
            return a; // -zero
        }
        // neg_error:
        unsafe {
            *pfflags |= 1 << 4;
        }
        return F_QNAN64;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf64(0, 0, 0); // zero
        }
        a_mant = normalize_subnormal_sf64(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u64 << 52;
    }
    a_exp -= EXP_MASK / 2;

    // simpler to handle an even exponent
    if (a_exp & 1) != 0 {
        a_exp -= 1;
        a_mant = a_mant.wrapping_shl(1);
    }
    a_exp = (a_exp >> 1) + EXP_MASK / 2;
    a_mant = a_mant.wrapping_shl((64 - 4 - 52) as u32);
    let ah = a_mant;
    let (root, inexact) = sqrtrem_u64(ah, 0);
    a_mant = root;
    if inexact != 0 {
        a_mant |= 1;
    }
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `sqrt_sf128` · softfp_template.h:720-767 · chunk 3239 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t sqrt_sf128(uint128_t a, RoundingModeEnum rm,
//                    uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
//         if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN128;
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(0, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         a_exp -= ((1 << 15) - 1) / 2;
//     
//         if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
//         a_exp = (a_exp >> 1) + ((1 << 15) - 1) / 2;
//         a_mant <<= (128 - 4 - 112);
//         if (sqrtrem_u128(&a_mant, a_mant, 0))
//             a_mant |= 1;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3239 {
// Port of softfp_template.h sqrt_sf128 (F_SIZE == 128, HAVE_INT128 variant).
#![allow(non_snake_case)]

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const F_SIZE: i32 = 128;
const MANT_SIZE: u32 = 112;
const RND_SIZE: i32 = (F_SIZE - 2) - MANT_SIZE as i32; /* 14 */
const EXP_MASK: i32 = (1 << 15) - 1;
const MANT_MASK: u128 = ((1u128) << MANT_SIZE) - 1;

const F_QNAN128: u128 = ((((1u128) << 15) - 1) << 112) | ((1u128) << (112 - 1));

/* Rounding modes (RoundingModeEnum) */
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

extern "C" {
    fn issignan_sf128(a: u128) -> c_int;
}

fn clz128(a: u128) -> i32 {
    a.leading_zeros() as i32
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (F_SIZE as u32 - 1))
        | ((a_exp as u128) << MANT_SIZE)
        | (a_mant & MANT_MASK)
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d != 0 && d >= F_SIZE {
        (a != 0) as u128
    } else if d > 0 {
        let du = d as u32;
        let mask = ((1u128) << du) - 1;
        (a >> du) | ((a & mask) != 0) as u128
    } else {
        a
    }
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;
    let rnd_size = RND_SIZE as u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (rnd_size - 1);
    } else if rm == RM_RTZ {
        addend = 0;
    } else if (a_sign ^ ((rm & 1) as u32)) != 0 {
        addend = (1u32 << rnd_size) - 1;
    } else {
        addend = 0;
    }

    let mut a_mant = a_mant_in;
    let mut a_exp = a_exp_in;
    let rnd_bits: u32;

    /* potentially subnormal */
    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < ((1u128) << (F_SIZE as u32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << rnd_size) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << rnd_size) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> rnd_size;
    /* half way: select even result */
    if rm == RM_RNE && rnd_bits == (1u32 << (rnd_size - 1)) {
        a_mant &= !1u128;
    }
    a_exp = a_exp.wrapping_add((a_mant >> (MANT_SIZE + 1)) as i32);
    if a_mant <= MANT_MASK {
        /* denormalized or zero */
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        /* overflow */
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK;
        } else {
            /* infinity */
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant_in) - (F_SIZE - 1 - (F_SIZE - 2));
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn normalize_subnormal_sf128(pa_exp: &mut i32, a_mant: u128) -> u128 {
    let shift: i32 = MANT_SIZE as i32 - ((F_SIZE - 1) - clz128(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

/* returns (high, low) of a * b */
fn mul_u128(a: u128, b: u128) -> (u128, u128) {
    let half = (F_SIZE as u32) / 2;
    let a0: u64 = a as u64;
    let a1: u64 = (a >> half) as u64;
    let b0: u64 = b as u64;
    let b1: u64 = (b >> half) as u64;

    let r00: u128 = (a0 as u128) * (b0 as u128);
    let r01: u128 = (a0 as u128) * (b1 as u128);
    let r10: u128 = (a1 as u128) * (b0 as u128);
    let r11: u128 = (a1 as u128) * (b1 as u128);

    let r0: u64 = r00 as u64;
    let mut c: u128 = (r00 >> half) + ((r01 as u64) as u128) + ((r10 as u64) as u128);
    let r1: u64 = c as u64;
    c = (c >> half) + (r01 >> half) + (r10 >> half) + ((r11 as u64) as u128);
    let r2: u64 = c as u64;
    let c3: u128 = (c >> half) + (r11 >> half);
    let r3: u64 = c3 as u64;

    let low = ((r1 as u128) << half) | (r0 as u128);
    let high = ((r3 as u128) << half) | (r2 as u128);
    (high, low)
}

/* returns (quotient, remainder) of (a1:a0) / b; requires a1 < b */
fn divrem_u128(a1_in: u128, a0_in: u128, b: u128) -> (u128, u128) {
    let mut a1 = a1_in;
    let mut a0 = a0_in;
    let mut i: i32 = 0;
    while i < F_SIZE {
        let ab: u32 = (a1 >> (F_SIZE as u32 - 1)) as u32;
        a1 = (a1 << 1) | (a0 >> (F_SIZE as u32 - 1));
        let qb: u32;
        if ab != 0 || a1 >= b {
            a1 = a1.wrapping_sub(b);
            qb = 1;
        } else {
            qb = 0;
        }
        a0 = (a0 << 1) | (qb as u128);
        i += 1;
    }
    (a0, a1)
}

/* compute sqrt of (a1:a0); returns (root, inexact) */
fn sqrtrem_u128(a1: u128, a0: u128) -> (u128, bool) {
    let l: i32;
    if a1 != 0 {
        l = 2 * F_SIZE - clz128(a1 - 1);
    } else {
        if a0 == 0 {
            return (0, false);
        }
        l = F_SIZE - clz128(a0 - 1);
    }
    let mut u: u128 = (1u128).wrapping_shl(((l + 1) / 2) as u32);
    let mut s: u128;
    loop {
        s = u;
        let (q, _r) = divrem_u128(a1, a0, s);
        u = (q.wrapping_add(s)) / 2;
        if u >= s {
            break;
        }
    }
    let (sq1, sq0) = mul_u128(s, s);
    let inexact = sq0 != a0 || sq1 != a1;
    (s, inexact)
}

#[no_mangle]
pub extern "C" fn sqrt_sf128(a: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    let a_sign: u32 = (a >> (F_SIZE as u32 - 1)) as u32;
    let mut a_exp: i32 = ((a >> MANT_SIZE) & (EXP_MASK as u128)) as i32;
    let mut a_mant: u128 = a & MANT_MASK;

    if a_exp == EXP_MASK {
        if a_mant != 0 {
            if unsafe { issignan_sf128(a) } != 0 {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN128;
        } else if a_sign != 0 {
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return F_QNAN128;
        } else {
            return a; /* +infinity */
        }
    }
    if a_sign != 0 {
        if a_exp == 0 && a_mant == 0 {
            return a; /* -zero */
        }
        unsafe {
            *pfflags |= FFLAG_INVALID_OP;
        }
        return F_QNAN128;
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf128(0, 0, 0); /* zero */
        }
        a_mant = normalize_subnormal_sf128(&mut a_exp, a_mant);
    } else {
        a_mant |= (1u128) << MANT_SIZE;
    }
    a_exp -= EXP_MASK / 2;
    /* simpler to handle an even exponent */
    if (a_exp & 1) != 0 {
        a_exp -= 1;
        a_mant = a_mant.wrapping_shl(1);
    }
    a_exp = (a_exp >> 1) + EXP_MASK / 2;
    a_mant = a_mant.wrapping_shl(F_SIZE as u32 - 4 - MANT_SIZE);
    let (root, inexact) = sqrtrem_u128(a_mant, 0);
    a_mant = root;
    if inexact {
        a_mant |= 1;
    }
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint32_t a_mant; a_sign = a >> (32 - 1); a_exp = (a >> 23) & ((…` · softfp_template.h:723-729 · chunk 3004 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3004 {
// Extracted from sqrt_sf32 (softfp_template.h lines 723-729).
// Decodes the IEEE-754 single-precision input into sign, biased exponent and
// mantissa fields.

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3004(
    a: u32,
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u32,
) {
    // C: a_sign = a >> (32 - 1);
    let a_sign: u32 = a >> 31;
    // C: a_exp = (a >> 23) & ((1 << 8) - 1);  (converted to int32_t)
    let a_exp: i32 = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    // C: a_mant = a & (((uint32_t)1 << 23) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);

    unsafe {
        if !pa_sign.is_null() {
            *pa_sign = a_sign;
        }
        if !pa_exp.is_null() {
            *pa_exp = a_exp;
        }
        if !pa_mant.is_null() {
            *pa_mant = a_mant;
        }
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_sign = a >> (64 - 1); a_exp = (a >> 52) & ((…` · softfp_template.h:723-729 · chunk 3116 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3116 {
// Extracted piece of sqrt_sf64: unpack the sign, exponent and mantissa of a
// binary64 soft-float value.
#[no_mangle]
pub extern "C" fn sqrt_sf64__part3116(
    a: u64,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u64,
) {
    unsafe {
        *a_sign = (a >> (64 - 1)) as u32;
        *a_exp = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
        *a_mant = a & ((1u64 << 52) - 1);
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_sign = a >> (128 - 1); a_exp = (a >> 112) &…` · softfp_template.h:723-729 · chunk 3240 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3240 {
// Extracted prologue of sqrt_sf128: splits the IEEE binary128 input `a`
// into sign, biased exponent and mantissa/trailing significand field.

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3240(
    a: *const u128,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u128,
) {
    unsafe {
        let av: u128 = *a;
        // a_sign = a >> (128 - 1);
        *a_sign = (av >> 127) as u32;
        // a_exp = (a >> 112) & ((1 << 15) - 1);
        *a_exp = ((av >> 112) & 0x7fffu128) as i32;
        // a_mant = a & (((uint128_t)1 << 112) - 1);
        *a_mant = av & ((1u128 << 112) - 1);
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1)` · softfp_template.h:730-741 · chunk 3005 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3005 {
// Ported slice of sqrt_sf32: special-case handling for a_exp == 0xFF.
// Returns 0 to fall through, 1 to return *p_ret, 2 to goto neg_error.

use core::ffi::c_int;

extern "C" {
    fn issignan_sf32(a: u32) -> c_int;
}

const F_QNAN32: u32 = 0x7FC0_0000u32;

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3005(
    a: u32,
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    pfflags: *mut u32,
    p_ret: *mut u32,
) -> c_int {
    if a_exp == ((1i32 << 8) - 1) {
        if a_mant != 0 {
            let sn = unsafe { issignan_sf32(a) };
            if sn != 0 {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            unsafe {
                *p_ret = F_QNAN32;
            }
            return 1;
        } else if a_sign != 0 {
            return 2;
        } else {
            unsafe {
                *p_ret = a;
            }
            return 1;
        }
    }
    0
}
}

// ── `if (a_exp == ((1 << 11) - 1)` · softfp_template.h:730-741 · chunk 3117 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3117 {
// Ported piece of sqrt_sf64: special-case handling of NaN/Inf input.
#![allow(non_snake_case)]

const F_QNAN64: u64 = (2047u64 << 52) | (1u64 << 51);

extern "C" {
    fn issignan_sf64(a: u64) -> i32;
}

/// Returns 0 to fall through, 1 to `return F_QNAN64;`, 2 to `return a;`.
#[no_mangle]
pub extern "C" fn sqrt_sf64__part3117(
    a: u64,
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    pfflags: *mut u32,
) -> i32 {
    let _ = F_QNAN64;
    if a_exp == 2047 {
        if a_mant != 0 {
            let s = unsafe { issignan_sf64(a) };
            if s != 0 {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            return 1;
        } else if a_sign != 0 {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return 1;
        } else {
            return 2;
        }
    }
    0
}
}

// ── `if (a_exp == ((1 << 15) - 1)` · softfp_template.h:730-741 · chunk 3241 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else if (a_sign) {
//                 goto neg_error;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3241 {
// Ported piece of sqrt_sf128: the special-case handling for a_exp == all ones.
// Status codes returned through pstatus:
//   0 = not applicable, caller continues with the rest of sqrt_sf128
//   1 = caller must `return` the returned value
//   2 = caller must `goto neg_error` (*pfflags |= 16; return F_QNAN128;)
use core::ffi::{c_int};

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

extern "C" {
    fn issignan_sf128(a: u128) -> c_int;
}

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3241(
    a: u128,
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    pfflags: *mut u32,
    pstatus: *mut c_int,
) -> u128 {
    let mut status: c_int = 0;
    let mut ret: u128 = 0;

    if a_exp == (1 << 15) - 1 {
        if a_mant != 0 {
            if unsafe { issignan_sf128(a) } != 0 {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            ret = F_QNAN128;
            status = 1;
        } else if a_sign != 0 {
            status = 2;
        } else {
            ret = a;
            status = 1;
        }
    }

    unsafe {
        *pstatus = status;
    }
    ret
}
}

// ── `if (a_sign)` · softfp_template.h:742-748 · chunk 3006 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN32;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3006 {
// Port of the `if (a_sign) { ... neg_error: ... }` piece of sqrt_sf32.
// The C side keeps the `neg_error` label (targeted by a goto in the caller)
// and performs the `*pfflags |= (1 << 4); return F_QNAN32;` action.
//
// Return convention:
//   *action == 0 -> no action, caller falls through
//   *action == 1 -> caller returns the returned value (the signed zero `a`)
//   *action == 2 -> caller takes the neg_error path

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3006(
    a: u32,
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    action: *mut i32,
) -> u32 {
    let mut act: i32 = 0;
    let mut ret: u32 = 0;

    if a_sign != 0 {
        if a_exp == 0 && a_mant == 0 {
            ret = a;
            act = 1;
        } else {
            act = 2;
        }
    }

    if !action.is_null() {
        unsafe {
            *action = act;
        }
    }
    ret
}
}

// ── `if (a_sign)` · softfp_template.h:742-748 · chunk 3118 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN64;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3118 {
// Port of the a_sign handling block of sqrt_sf64 (softfp_template.h lines 742-748).
// A negative zero is returned unchanged; every other negative input raises the
// invalid flag and yields the quiet NaN.

const F_QNAN64: u64 = ((0x7FFu64) << 52) | (1u64 << 51);

#[no_mangle]
pub extern "C" fn sqrt_sf64__part3118(
    a: u64,
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    pfflags: *mut u32,
) -> u64 {
    if a_sign != 0 && a_exp == 0 && a_mant == 0 {
        return a;
    }
    unsafe {
        *pfflags |= 1u32 << 4;
    }
    F_QNAN64
}
}

// ── `if (a_sign)` · softfp_template.h:742-748 · chunk 3242 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign) {
//             if (a_exp == 0 && a_mant == 0)
//                 return a;
//         neg_error:
//             *pfflags |= (1 << 4);
//             return F_QNAN128;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3242 {
// Port of the sign/negative-error block of sqrt_sf128.
// Reports the invalid-operation flag and returns F_QNAN128.

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3242(pfflags: *mut u32) -> u128 {
    // F_QNAN128 == (((uint128_t)((1 << 15) - 1) << 112) | ((uint128_t)1 << (112 - 1)))
    const F_QNAN128: u128 = ((0x7fffu128) << 112) | (1u128 << 111);

    unsafe {
        if !pfflags.is_null() {
            *pfflags |= 1u32 << 4;
        }
    }
    F_QNAN128
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:749-755 · chunk 3007 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(0, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3007 {
// Ported piece of sqrt_sf32: the subnormal/normal mantissa setup.
// Reads: a_exp (i32, in/out), a_mant (u32, by value).
// Writes: *pa_exp (may be updated), *pzero (set to 1 when the caller must
//         return pack_sf32(0,0,0) immediately).
// Returns: the new a_mant.

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3007(
    pa_exp: *mut i32,
    a_mant: u32,
    pzero: *mut i32,
) -> u32 {
    unsafe {
        if *pa_exp == 0 {
            if a_mant == 0 {
                *pzero = 1;
                return 0;
            }
            let shift: i32 = 23 - (32 - 1 - clz32(a_mant));
            *pa_exp = 1 - shift;
            a_mant.wrapping_shl(shift as u32)
        } else {
            a_mant | (1u32 << 23)
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:749-755 · chunk 3119 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(0, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3119 {
// Port of the subnormal/normal mantissa setup part of sqrt_sf64 (softfp).
// Returns 1 when the caller must return pack_sf64(0, 0, 0) (i.e. +0 input),
// 0 otherwise; the updated exponent and mantissa go back through the pointers.

#[no_mangle]
pub extern "C" fn sqrt_sf64__part3119(pa_exp: *mut i32, pa_mant: *mut u64) -> i32 {
    unsafe {
        let exp: i32 = *pa_exp;
        let mant: u64 = *pa_mant;

        if exp == 0 {
            if mant == 0 {
                // caller maps this to pack_sf64(0, 0, 0)
                return 1;
            }
            // normalize_subnormal_sf64(&a_exp, a_mant)
            let clz: i32 = if mant == 0 { 64 } else { mant.leading_zeros() as i32 };
            let shift: i32 = 52 - (64 - 1 - clz);
            *pa_exp = 1 - shift;
            *pa_mant = mant << shift;
        } else {
            *pa_mant = mant | (1u64 << 52);
        }

        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:749-755 · chunk 3243 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(0, 0, 0);
//             a_mant = normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3243 {
// Port of the subnormal-normalisation / implicit-bit part of sqrt_sf128.
// Returns 0 on the normal path (results written through the pointers) and a
// nonzero value when the original code would `return pack_sf128(0, 0, 0)`.

use core::ffi::{c_int};

// static inline int clz128(uint128_t a) -- no linker symbol, ported here.
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3243(pa_exp: *mut i32, pa_mant: *mut u128) -> c_int {
    unsafe {
        if pa_exp.is_null() || pa_mant.is_null() {
            return 1;
        }
        let a_exp = *pa_exp;
        let a_mant = *pa_mant;
        if a_exp == 0 {
            if a_mant == 0 {
                // caller does: return pack_sf128(0, 0, 0);
                return 1;
            }
            // normalize_subnormal_sf128(&a_exp, a_mant)
            let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
            *pa_exp = 1 - shift;
            *pa_mant = a_mant.wrapping_shl(shift as u32);
        } else {
            *pa_mant = a_mant | (1u128 << 112);
        }
        0
    }
}
}

// ── `a_exp -= ((1 << 8) - 1) / 2;` · softfp_template.h:756-756 · chunk 3008 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp -= ((1 << 8) - 1) / 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3008 {
// Port of: a_exp -= ((1 << 8) - 1) / 2;  (softfp_template.h line 756)
// a_exp is an int32_t local of the caller; it is read and written, so it is
// passed by pointer.

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3008(pa_exp: *mut i32) {
    unsafe {
        if !pa_exp.is_null() {
            // ((1 << 8) - 1) / 2 == 127
            *pa_exp = (*pa_exp).wrapping_sub(127);
        }
    }
}
}

// ── `a_exp -= ((1 << 11) - 1) / 2;` · softfp_template.h:756-756 · chunk 3120 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp -= ((1 << 11) - 1) / 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3120 {
// Ported piece of sqrt_sf64: exponent bias removal.
//   a_exp -= ((1 << 11) - 1) / 2;

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sqrt_sf64__part3120(a_exp: c_int) -> c_int {
    a_exp.wrapping_sub(((1i32 << 11) - 1) / 2)
}
}

// ── `a_exp -= ((1 << 15) - 1) / 2;` · softfp_template.h:756-756 · chunk 3244 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp -= ((1 << 15) - 1) / 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3244 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3244(a_exp: c_int) -> c_int {
    // a_exp -= ((1 << 15) - 1) / 2;  with int32_t a_exp
    a_exp.wrapping_sub(((1 << 15) - 1) / 2)
}
}

// ── `if (a_exp & 1)` · softfp_template.h:758-761 · chunk 3009 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3009 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3009(a_exp: *mut c_int, a_mant: *mut u32) {
    unsafe {
        if (*a_exp) & 1 != 0 {
            *a_exp -= 1;
            *a_mant = (*a_mant).wrapping_shl(1);
        }
    }
}
}

// ── `if (a_exp & 1)` · softfp_template.h:758-761 · chunk 3121 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3121 {
// Portable piece of sqrt_sf64: exponent oddness fixup (odd exponent -> decrement
// and shift the mantissa left by one, i.e. multiply by two) before halving.
#[no_mangle]
pub extern "C" fn sqrt_sf64__part3121(a_exp: *mut i32, a_mant: *mut u64) {
    unsafe {
        if (*a_exp & 1) != 0 {
            *a_exp = (*a_exp).wrapping_sub(1);
            *a_mant = *a_mant << 1;
        }
    }
}
}

// ── `if (a_exp & 1)` · softfp_template.h:758-761 · chunk 3245 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp & 1) {
//             a_exp--;
//             a_mant <<= 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3245 {
// Port of the exponent-parity adjustment inside sqrt_sf128.
// uint128_t is `unsigned __int128`; a_exp is int32_t.

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3245(a_exp: *mut i32, a_mant: *mut u128) {
    unsafe {
        if (*a_exp) & 1 != 0 {
            *a_exp -= 1;
            *a_mant = (*a_mant).wrapping_shl(1);
        }
    }
}
}

// ── `a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2; a_mant <<= (32 - 4 - 23);` · softfp_template.h:762-763 · chunk 3010 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2;
//         a_mant <<= (32 - 4 - 23);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3010 {
// Ported piece of sqrt_sf32: exponent/mantissa re-basing before the integer
// square root.
//
//   a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2;
//   a_mant <<= (32 - 4 - 23);

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3010(pa_exp: *mut i32, pa_mant: *mut u32) {
    unsafe {
        // a_exp is a signed int32_t: >> is an arithmetic shift in C.
        *pa_exp = (*pa_exp >> 1) + (((1i32 << 8) - 1) / 2);
        // a_mant is uint32_t, well-defined wrapping left shift.
        *pa_mant <<= 32 - 4 - 23;
    }
}
}

// ── `a_exp = (a_exp >> 1) + ((1 << 11) - 1) / 2; a_mant <<= (64 - 4 - 52);` · softfp_template.h:762-763 · chunk 3122 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (a_exp >> 1) + ((1 << 11) - 1) / 2;
//         a_mant <<= (64 - 4 - 52);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3122 {
// Ported piece of sqrt_sf64: exponent halving/shifting of the mantissa.
#[no_mangle]
pub extern "C" fn sqrt_sf64__part3122(
    a_exp: i32,
    a_mant: u64,
    a_exp_out: *mut i32,
    a_mant_out: *mut u64,
) {
    // ((1 << 11) - 1) / 2 == 1023
    let e: i32 = (a_exp >> 1) + ((1i32 << 11) - 1) / 2;
    let m: u64 = a_mant << (64 - 4 - 52);
    unsafe {
        if !a_exp_out.is_null() {
            *a_exp_out = e;
        }
        if !a_mant_out.is_null() {
            *a_mant_out = m;
        }
    }
}
}

// ── `a_exp = (a_exp >> 1) + ((1 << 15) - 1) / 2; a_mant <<= (128 - 4 - 112);` · softfp_template.h:762-763 · chunk 3246 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (a_exp >> 1) + ((1 << 15) - 1) / 2;
//         a_mant <<= (128 - 4 - 112);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3246 {
// Portion of sqrt_sf128: rescale the exponent and shift the mantissa left.
#[no_mangle]
pub extern "C" fn sqrt_sf128__part3246(a_exp: *mut i32, a_mant: *mut u128) {
    unsafe {
        if !a_exp.is_null() {
            let e: i32 = *a_exp;
            // a_exp = (a_exp >> 1) + ((1 << 15) - 1) / 2;
            *a_exp = (e >> 1) + (((1i32 << 15) - 1) / 2);
        }
        if !a_mant.is_null() {
            let m: u128 = *a_mant;
            // a_mant <<= (128 - 4 - 112);
            *a_mant = m << (128 - 4 - 112);
        }
    }
}
}

// ── `if (sqrtrem_u32(&a_mant, a_mant, 0))` · softfp_template.h:764-765 · chunk 3011 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sqrtrem_u32(&a_mant, a_mant, 0))
//             a_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3011 {
// Port of the TARGET from sqrt_sf32:
//   if (sqrtrem_u32(&a_mant, a_mant, 0)) a_mant |= 1;
// sqrtrem_u32 is a static C function (no linker symbol), so its logic is
// ported here as a private Rust fn.

fn clz32(a: u32) -> i32 {
    // matches the C helper: clz32(0) == 32, otherwise count of leading zeros
    a.leading_zeros() as i32
}

/// uint32_t integer square root with remainder flag.
/// Equivalent to the static C helper sqrtrem_u32(uint32_t *pr, uint32_t ah, uint32_t al).
fn sqrtrem_u32(pr: *mut u32, ah: u32, al: u32) -> i32 {
    unsafe {
        let l: i32;
        if ah != 0 {
            l = 2 * 32 - clz32(ah.wrapping_sub(1));
        } else {
            if al == 0 {
                *pr = 0;
                return 0;
            }
            l = 32 - clz32(al.wrapping_sub(1));
        }
        let a: u64 = ((ah as u64) << 32) | (al as u64);
        let mut u: u64 = 1u64 << (((l + 1) / 2) as u32);
        let mut s: u64 = 0;
        loop {
            s = u;
            u = ((a / s) + s) / 2;
            if u >= s {
                break;
            }
        }
        let inexact: i32 = if a.wrapping_sub(s.wrapping_mul(s)) != 0 { 1 } else { 0 };
        *pr = s as u32;
        inexact
    }
}

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3011(a_mant: *mut u32) -> core::ffi::c_int {
    unsafe {
        if sqrtrem_u32(a_mant, *a_mant, 0) != 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `if (sqrtrem_u64(&a_mant, a_mant, 0))` · softfp_template.h:764-765 · chunk 3123 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sqrtrem_u64(&a_mant, a_mant, 0))
//             a_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3123 {
use core::ffi::c_int;

// Ported from the C static helper clz64 (static inline, no linker symbol).
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// Ported from the C static function sqrtrem_u64 (no linker symbol available).
fn sqrtrem_u64(pr: *mut u64, ah: u64, al: u64) -> c_int {
    let l: i32;
    if ah != 0 {
        l = 2 * 64 - clz64(ah.wrapping_sub(1));
    } else {
        if al == 0 {
            unsafe { *pr = 0; }
            return 0;
        }
        l = 64 - clz64(al.wrapping_sub(1));
    }
    let a: u128 = ((ah as u128) << 64) | (al as u128);
    let mut u: u128 = 1u128 << (((l + 1) / 2) as u32);
    let mut s: u128 = 0;
    loop {
        s = u;
        // s is always >= 1 here, so the division cannot trap.
        u = (a / s + s) / 2;
        if u >= s {
            break;
        }
    }
    let inexact: bool = a.wrapping_sub(s.wrapping_mul(s)) != 0;
    unsafe {
        *pr = s as u64;
    }
    if inexact {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn sqrt_sf64__part3123(pa_mant: *mut u64) {
    unsafe {
        let v: u64 = *pa_mant;
        if sqrtrem_u64(pa_mant, v, 0) != 0 {
            *pa_mant |= 1;
        }
    }
}
}

// ── `if (sqrtrem_u128(&a_mant, a_mant, 0))` · softfp_template.h:764-765 · chunk 3247 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sqrtrem_u128(&a_mant, a_mant, 0))
//             a_mant |= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3247 {
// Port of the sqrtrem/inexact-fixup piece of sqrt_sf128 (softfp_template.h).
//
// The static C helpers sqrtrem_u128, divrem_u128, mul_u128 and clz128 have no
// linker symbol, so their logic is reproduced here as private Rust functions
// (128-bit unsigned integers are `u128`, i.e. unsigned __int128).

#![allow(non_snake_case)]

type U128 = u128;

#[inline]
fn clz128(a: U128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

// Returns (high, low) of the 256-bit product, the C function stored the low
// half through *plow and returned the high half.
fn mul_u128(a: U128, b: U128) -> (U128, U128) {
    let a0 = a as u64;
    let a1 = (a >> 64) as u64;
    let b0 = b as u64;
    let b1 = (b >> 64) as u64;

    let r00 = (a0 as U128) * (b0 as U128);
    let r01 = (a0 as U128) * (b1 as U128);
    let r10 = (a1 as U128) * (b0 as U128);
    let r11 = (a1 as U128) * (b1 as U128);

    let r0 = r00 as u64;
    let mut c = (r00 >> 64) + ((r01 as u64) as U128) + ((r10 as u64) as U128);
    let r1 = c as u64;
    c = (c >> 64) + (r01 >> 64) + (r10 >> 64) + ((r11 as u64) as U128);
    let r2 = c as u64;
    let r3 = (c >> 64) + (r11 >> 64);

    let low = ((r1 as U128) << 64) | (r0 as U128);
    let high = ((r3 as U128) << 64) | (r2 as U128);
    (high, low)
}

// Returns (quotient, remainder); in C the quotient was returned and the
// remainder stored through *pr. Requires a1 < b (as asserted in C).
fn divrem_u128(a1_in: U128, a0_in: U128, b: U128) -> (U128, U128) {
    let mut a1 = a1_in;
    let mut a0 = a0_in;
    let mut i: i32 = 0;
    while i < 128 {
        let ab = (a1 >> 127) as i32;
        a1 = (a1 << 1) | (a0 >> 127);
        let qb: U128;
        if ab != 0 || a1 >= b {
            a1 = a1.wrapping_sub(b);
            qb = 1;
        } else {
            qb = 0;
        }
        a0 = (a0 << 1) | qb;
        i += 1;
    }
    (a0, a1)
}

// Returns (integer square root, inexact); the C function stored the root
// through *pr and returned the inexact flag.
fn sqrtrem_u128(a1: U128, a0: U128) -> (U128, i32) {
    let l: i32;
    if a1 != 0 {
        l = 2 * 128 - clz128(a1.wrapping_sub(1));
    } else {
        if a0 == 0 {
            return (0, 0);
        }
        l = 128 - clz128(a0.wrapping_sub(1));
    }
    let shift = ((l + 1) / 2) as u32;
    let mut u: U128 = 1u128.wrapping_shl(shift);
    let mut s: U128;
    loop {
        s = u;
        let (q, _r) = divrem_u128(a1, a0, s);
        u = q.wrapping_add(s) / 2;
        if u >= s {
            break;
        }
    }
    let (sq1, sq0) = mul_u128(s, s);
    let inexact: i32 = if sq0 != a0 || sq1 != a1 { 1 } else { 0 };
    (s, inexact)
}

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3247(pa_mant: *mut U128) {
    unsafe {
        if pa_mant.is_null() {
            return;
        }
        let a_mant: U128 = *pa_mant;
        // sqrtrem_u128(&a_mant, a_mant, 0): stores the root into a_mant and
        // returns nonzero when the root is inexact.
        let (root, inexact) = sqrtrem_u128(a_mant, 0);
        let mut m: U128 = root;
        if inexact != 0 {
            m |= 1;
        }
        *pa_mant = m;
    }
}
}

// ── `return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:766-766 · chunk 3012 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3012 {
// Port of the tail of sqrt_sf32: normalize_sf32 (static in the C TU, so its
// logic is reimplemented here) together with the static helpers it needs
// (clz32, pack_sf32, rshift_rnd32, roundpack_sf32).

const F_MANT_MASK: u32 = ((1u32 << 23) - 1);
const F_EXP_MASK: i32 = ((1 << 8) - 1);
const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RMM: u32 = 4;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & F_MANT_MASK)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn roundpack_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: u32, pfflags: *mut u32) -> u32 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u32 = a_mant_in;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        // is_subnormal = (a_exp < 0 || (a_mant + addend) < (1 << 31))
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (23 + 1)) as i32);
    if a_mant <= F_MANT_MASK {
        a_exp = 0;
    } else if a_exp >= F_EXP_MASK {
        if addend == 0 {
            a_exp = F_EXP_MASK - 1;
            a_mant = F_MANT_MASK;
        } else {
            a_exp = F_EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: u32, pfflags: *mut u32) -> u32 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u32 = a_mant_in;
    // assert(shift >= 0)
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    a_exp -= shift;
    if shift > 0 {
        // shift is at most 31 whenever the C assertion holds / a_mant != 0
        a_mant = a_mant.wrapping_shl((shift as u32) & 31);
    }
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn sqrt_sf32__part3012(
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    rm: u32,
    pfflags: *mut u32,
) -> u32 {
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:766-766 · chunk 3124 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3124 {
// Tail of sqrt_sf64: normalize_sf64 plus its static helpers
// (clz64, roundpack_sf64, rshift_rnd64, pack_sf64), all file-static in C,
// so their logic is ported privately here.

use core::ffi::c_int;

const MANT_MASK: u64 = (1u64 << 52) - 1;
const RND_MASK: u32 = (1u32 << 10) - 1; // (1 << ((64 - 2) - 52)) - 1

fn clz64(a: u64) -> i32 {
    // matches: a == 0 ? 64 : __builtin_clzll(a)
    a.leading_zeros() as i32
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & MANT_MASK)
}

fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = 1u64.wrapping_shl(d as u32).wrapping_sub(1);
            a = a.wrapping_shr(d as u32) | ((a & mask) != 0) as u64;
        }
    }
    a
}

unsafe fn roundpack_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP, and anything else (C default:)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & RND_MASK as u64) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1u32 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & RND_MASK as u64) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1u32 << 0;
    }
    a_mant = a_mant.wrapping_add(addend as u64).wrapping_shr((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        *pfflags |= (1u32 << 2) | (1u32 << 0);
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    if shift < 0 {
        // the C code asserts shift >= 0 here; abort without unwinding
        std::process::abort();
    }
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn sqrt_sf64__part3124(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    unsafe { normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags) }
}
}

// ── `return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template.h:766-766 · chunk 3248 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3248 {
// Rust port of the TARGET piece: tail of sqrt_sf128 (softfp_template.h line 766)
// which normalizes the mantissa and rounds/packs the result.
// normalize_sf128 / roundpack_sf128 / pack_sf128 / rshift_rnd128 / clz128 are
// static C helpers, so their logic is reproduced here as private Rust functions.
#![allow(non_snake_case)]
#![allow(dead_code)]

const EXP_MASK_U32: u32 = (1u32 << 15) - 1;
const MANT_MASK128: u128 = (1u128 << 112) - 1;

#[inline]
fn clz128(a: u128) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK128)
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

// RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        0 | 4 => {
            addend = 1u32 << (((128 - 2) - 112) - 1);
        }
        1 => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP (and any other value)
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_mask: u128 = (1u128 << ((128 - 2) - 112)) - 1;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & rnd_mask) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & rnd_mask) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == 0 && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (112 + 1)) as i32);
    if a_mant <= MANT_MASK128 {
        a_exp = 0;
    } else if a_exp >= (EXP_MASK_U32 as i32) {
        if addend == 0 {
            a_exp = (EXP_MASK_U32 as i32) - 1;
            a_mant = MANT_MASK128;
        } else {
            a_exp = EXP_MASK_U32 as i32;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    // C has assert(shift >= 0); keep it panic-free here.
    let sh: u32 = if shift < 0 {
        0
    } else if shift >= 128 {
        128
    } else {
        shift as u32
    };
    let new_exp: i32 = a_exp.wrapping_sub(shift);
    let new_mant: u128 = if sh >= 128 { 0 } else { a_mant << sh };
    roundpack_sf128(a_sign, new_exp, new_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn sqrt_sf128__part3248(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `min_sf32` · softfp_template.h:771-802 · chunk 3013 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t min_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (isnan_sf32(a)) {
//                 if (isnan_sf32(b))
//                     return F_QNAN32;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3013 {
// Rewrite of min_sf32 from softfp_template.h (softfloat-like library).

const F_QNAN32: u32 = ((1u32 << 8) - 1) << 23 | 1u32 << (23 - 1);

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1: u32 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    a_exp1 == 2 * ((1u32 << 8) - 1) && a_mant != 0
}

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp: u32 = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn min_sf32(a: u32, b: u32, pfflags: *mut u32) -> u32 {
    let a_sign: u32;
    let b_sign: u32;

    if isnan_sf32(a) || isnan_sf32(b) {
        if issignan_sf32(a) || issignan_sf32(b) {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return F_QNAN32;
        } else if isnan_sf32(a) {
            if isnan_sf32(b) {
                return F_QNAN32;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }

    a_sign = a >> (32 - 1);
    b_sign = b >> (32 - 1);

    if a_sign != b_sign {
        if a_sign != 0 {
            a
        } else {
            b
        }
    } else {
        if (a < b) ^ (a_sign != 0) {
            a
        } else {
            b
        }
    }
}
}

// ── `min_sf64` · softfp_template.h:771-802 · chunk 3125 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t min_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (isnan_sf64(a)) {
//                 if (isnan_sf64(b))
//                     return F_QNAN64;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3125 {
use core::ffi::c_uint;
use core::ffi::c_ulonglong;

pub type Float64 = u64;

const F_QNAN64: Float64 = ((1u64 << 11) - 1) << 52 | (1u64 << (52 - 1));

#[no_mangle]
pub unsafe extern "C" fn min_sf64(a: Float64, b: Float64, pfflags: *mut c_uint) -> Float64 {
    let a_sign: u32;
    let b_sign: u32;

    if isnan_sf64(a) || isnan_sf64(b) {
        if issignan_sf64(a) || issignan_sf64(b) {
            *pfflags |= 1 << 4;
            return F_QNAN64;
        } else if isnan_sf64(a) {
            if isnan_sf64(b) {
                return F_QNAN64;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }
    a_sign = (a >> (64 - 1)) as u32;
    b_sign = (b >> (64 - 1)) as u32;

    if a_sign != b_sign {
        if a_sign != 0 {
            a
        } else {
            b
        }
    } else {
        if (a < b) ^ (a_sign != 0) {
            a
        } else {
            b
        }
    }
}

fn issignan_sf64(a: Float64) -> bool {
    let a_exp1: u32 = ((a >> (52 - 1)) & ((1u64 << (11 + 1)) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp1 == 2 * ((1u32 << 11) - 1) && a_mant != 0
}

fn isnan_sf64(a: Float64) -> bool {
    let a_exp: u32 = ((a >> 52) & ((1u64 << 11) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp == (1u32 << 11) - 1 && a_mant != 0
}

// Keep the c_ulonglong / c_uint imports meaningful for ABI documentation.
#[allow(dead_code)]
fn _abi_marker(_a: c_ulonglong, _b: c_ulonglong, _p: *mut c_uint) {}
}

// ── `min_sf128` · softfp_template.h:771-802 · chunk 3249 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t min_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (isnan_sf128(a)) {
//                 if (isnan_sf128(b))
//                     return F_QNAN128;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3249 {
use core::ffi::c_int;

extern "C" {
    // BOOL isnan_sf128(uint128_t a);
    fn isnan_sf128(a: u128) -> c_int;
    // BOOL issignan_sf128(uint128_t a);
    fn issignan_sf128(a: u128) -> c_int;
}

// static const uint128_t F_QNAN128 = ((uint128_t)((1 << 15) - 1) << 112) | ((uint128_t)1 << (112 - 1));
const F_QNAN128: u128 = ((32767u128) << 112) | (1u128 << 111);

#[no_mangle]
pub extern "C" fn min_sf128(a: u128, b: u128, pfflags: *mut u32) -> u128 {
    let a_is_nan = unsafe { isnan_sf128(a) } != 0;
    let b_is_nan = unsafe { isnan_sf128(b) } != 0;

    if a_is_nan || b_is_nan {
        if unsafe { issignan_sf128(a) } != 0 || unsafe { issignan_sf128(b) } != 0 {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return F_QNAN128;
        } else if a_is_nan {
            if b_is_nan {
                return F_QNAN128;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }

    let a_sign: u32 = (a >> (128 - 1)) as u32;
    let b_sign: u32 = (b >> (128 - 1)) as u32;

    if a_sign != b_sign {
        if a_sign != 0 {
            a
        } else {
            b
        }
    } else {
        if (((a < b) as u32) ^ a_sign) != 0 {
            a
        } else {
            b
        }
    }
}
}

// ── `if (isnan_sf32(a) || isnan_sf32(b))` · softfp_template.h:775-787 · chunk 3014 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (isnan_sf32(a)) {
//                 if (isnan_sf32(b))
//                     return F_QNAN32;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3014 {
// Piece of min_sf32: the NaN/sNaN handling branch.
// Returns the value to return; *pret signals that the C caller must return it.

use core::ffi::c_int;

extern "C" {
    fn isnan_sf32(a: u32) -> c_int;
    fn issignan_sf32(a: u32) -> c_int;
}

#[no_mangle]
pub extern "C" fn min_sf32__part3014(
    a: u32,
    b: u32,
    pfflags: *mut u32,
    pret: *mut c_int,
) -> u32 {
    // F_QNAN32 constant from the C translation unit
    const F_QNAN32: u32 = ((255u32) << 23) | (1u32 << (23 - 1));

    unsafe {
        if isnan_sf32(a) != 0 || isnan_sf32(b) != 0 {
            if issignan_sf32(a) != 0 || issignan_sf32(b) != 0 {
                *pfflags |= 1u32 << 4;
                *pret = 1;
                return F_QNAN32;
            } else if isnan_sf32(a) != 0 {
                *pret = 1;
                if isnan_sf32(b) != 0 {
                    return F_QNAN32;
                } else {
                    return b;
                }
            } else {
                *pret = 1;
                return a;
            }
        }
        *pret = 0;
        0
    }
}
}

// ── `if (isnan_sf64(a) || isnan_sf64(b))` · softfp_template.h:775-787 · chunk 3126 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (isnan_sf64(a)) {
//                 if (isnan_sf64(b))
//                     return F_QNAN64;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3126 {
use core::ffi::c_int;

// C functions from the same translation unit (non-static, so they have symbols).
extern "C" {
    fn isnan_sf64(a: u64) -> c_int;
    fn issignan_sf64(a: u64) -> c_int;
}

// static const uint64_t F_QNAN64 = (((uint64_t)((1 << 11) - 1) << 52) | ((uint64_t)1 << (52 - 1)));
const F_QNAN64: u64 = ((0x7ffu64) << 52) | (1u64 << 51);

/// Ports the NaN-handling prefix of min_sf64.
/// Returns 1 when the original branch would have returned (the result is stored in *out),
/// 0 when control falls through to the non-NaN path.
#[no_mangle]
pub extern "C" fn min_sf64__part3126(
    a: u64,
    b: u64,
    pfflags: *mut u32,
    out: *mut u64,
) -> c_int {
    unsafe {
        if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 {
            if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 {
                *pfflags |= 1u32 << 4;
                *out = F_QNAN64;
                return 1;
            } else if isnan_sf64(a) != 0 {
                if isnan_sf64(b) != 0 {
                    *out = F_QNAN64;
                } else {
                    *out = b;
                }
                return 1;
            } else {
                *out = a;
                return 1;
            }
        }
        0
    }
}
}

// ── `if (isnan_sf128(a) || isnan_sf128(b))` · softfp_template.h:775-787 · chunk 3250 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (isnan_sf128(a)) {
//                 if (isnan_sf128(b))
//                     return F_QNAN128;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3250 {
// Portion of min_sf128 handling NaN operands.
use core::ffi::c_int;

extern "C" {
    fn isnan_sf128(a: u128) -> c_int;
    fn issignan_sf128(a: u128) -> c_int;
}

// static const uint128_t F_QNAN128 in the C translation unit (no linker symbol).
const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

/// Returns nonzero when the caller (min_sf128) must return *presult.
#[no_mangle]
pub extern "C" fn min_sf128__part3250(
    a: u128,
    b: u128,
    pfflags: *mut u32,
    presult: *mut u128,
) -> c_int {
    let isnan_a = unsafe { isnan_sf128(a) } != 0;
    let isnan_b = unsafe { isnan_sf128(b) } != 0;

    if isnan_a || isnan_b {
        if unsafe { issignan_sf128(a) } != 0 || unsafe { issignan_sf128(b) } != 0 {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            unsafe {
                *presult = F_QNAN128;
            }
            return 1;
        } else if isnan_a {
            if isnan_b {
                unsafe {
                    *presult = F_QNAN128;
                }
            } else {
                unsafe {
                    *presult = b;
                }
            }
            return 1;
        } else {
            unsafe {
                *presult = a;
            }
            return 1;
        }
    }
    0
}
}

// ── `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` · softfp_template.h:788-789 · chunk 3015 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3015 {
// Port of the sign-extraction piece of min_sf32.
// a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);

#[no_mangle]
pub extern "C" fn min_sf32__part3015(
    a: u32,
    b: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = a >> (32 - 1);
        }
        if !b_sign.is_null() {
            *b_sign = b >> (32 - 1);
        }
    }
}
}

// ── `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` · softfp_template.h:788-789 · chunk 3127 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3127 {
#[no_mangle]
pub extern "C" fn min_sf64__part3127(
    a: u64,
    b: u64,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        *a_sign = (a >> 63) as u32;
        *b_sign = (b >> 63) as u32;
    }
}
}

// ── `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` · softfp_template.h:788-789 · chunk 3251 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3251 {
// Extracted from min_sf128: compute the sign bits of a and b.
#[no_mangle]
pub extern "C" fn min_sf128__part3251(
    a: u128,
    b: u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = (a >> (128 - 1)) as u32;
        }
        if !b_sign.is_null() {
            *b_sign = (b >> (128 - 1)) as u32;
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:791-801 · chunk 3016 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3016 {
// Port of the sign-aware min selection tail of min_sf32.

#[no_mangle]
pub extern "C" fn min_sf32__part3016(a: u32, b: u32, a_sign: u32, b_sign: u32) -> u32 {
    if a_sign != b_sign {
        if a_sign != 0 {
            a
        } else {
            b
        }
    } else if ((a < b) as u32 ^ a_sign) != 0 {
        a
    } else {
        b
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:791-801 · chunk 3128 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3128 {
#[no_mangle]
pub extern "C" fn min_sf64__part3128(a: u64, b: u64, a_sign: u32, b_sign: u32) -> u64 {
    if a_sign != b_sign {
        if a_sign != 0 {
            a
        } else {
            b
        }
    } else {
        if ((a < b) as u32 ^ (a_sign != 0) as u32) != 0 {
            a
        } else {
            b
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:791-801 · chunk 3252 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return a;
//             else
//                 return b;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return a;
//             else
//                 return b;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3252 {
// Port of the tail selection logic of min_sf128 (softfp_template.h lines 791-801).
// Decides which operand min_sf128 must return:
//   a_sign != b_sign  -> return the one with the sign bit set
//   a_sign == b_sign  -> return a when (a < b) ^ a_sign, else b
// Returns 1 to select `a`, 0 to select `b`.

#[no_mangle]
pub extern "C" fn min_sf128__part3252(
    a: *const u128,
    b: *const u128,
    a_sign: u32,
    b_sign: u32,
) -> u32 {
    unsafe {
        let av: u128 = core::ptr::read(a);
        let bv: u128 = core::ptr::read(b);
        let sign_a: bool = a_sign != 0;
        let sign_b: bool = b_sign != 0;
        let pick_a: bool = if a_sign != b_sign {
            sign_a
        } else {
            (av < bv) ^ sign_a
        };
        if pick_a { 1 } else { 0 }
    }
}
}

// ── `max_sf32` · softfp_template.h:804-835 · chunk 3017 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t max_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (isnan_sf32(a)) {
//                 if (isnan_sf32(b))
//                     return F_QNAN32;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3017 {
// Rewrite of max_sf32 (softfp_template.h, F_SIZE = 32).
// F_QNAN32 = ((1 << 23) - 1) << 23 ... expanded: (0x7F << 23) | (1 << 22) = 0x7FC00000

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0
}

#[no_mangle]
pub unsafe extern "C" fn max_sf32(a: u32, b: u32, pfflags: *mut u32) -> u32 {
    const F_QNAN32: u32 = (((1u32 << 8) - 1) << 23) | (1u32 << (23 - 1));

    if isnan_sf32(a) || isnan_sf32(b) {
        if issignan_sf32(a) || issignan_sf32(b) {
            *pfflags |= 1u32 << 4;
            return F_QNAN32;
        } else if isnan_sf32(a) {
            if isnan_sf32(b) {
                return F_QNAN32;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }

    let a_sign: u32 = a >> (32 - 1);
    let b_sign: u32 = b >> (32 - 1);

    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else if ((a < b) ^ (a_sign != 0)) {
        b
    } else {
        a
    }
}
}

// ── `max_sf64` · softfp_template.h:804-835 · chunk 3129 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t max_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (isnan_sf64(a)) {
//                 if (isnan_sf64(b))
//                     return F_QNAN64;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3129 {
// Port of max_sf64 from softfp_template.h (F_SIZE == 64).
#![allow(clippy::all)]

use core::ffi::{c_int, c_uint};

const F_QNAN64: u64 = ((2047u64) << 52) | (1u64 << 51);

const MANT_SIZE: u32 = 52;
const EXP_MASK: u64 = (1u64 << 11) - 1;
const MANT_MASK: u64 = (1u64 << MANT_SIZE) - 1;

#[inline]
fn issignan_sf64(a: u64) -> bool {
    let a_exp1 = (a >> (MANT_SIZE - 1)) & ((1u64 << (11 + 1)) - 1);
    let a_mant = a & MANT_MASK;
    a_exp1 == 2 * EXP_MASK && a_mant != 0
}

#[inline]
fn isnan_sf64(a: u64) -> bool {
    let a_exp = (a >> MANT_SIZE) & EXP_MASK;
    let a_mant = a & MANT_MASK;
    a_exp == EXP_MASK && a_mant != 0
}

#[no_mangle]
pub unsafe extern "C" fn max_sf64(a: u64, b: u64, pfflags: *mut c_uint) -> u64 {
    if isnan_sf64(a) || isnan_sf64(b) {
        if issignan_sf64(a) || issignan_sf64(b) {
            *pfflags |= 1u32 << 4;
            return F_QNAN64;
        } else if isnan_sf64(a) {
            if isnan_sf64(b) {
                return F_QNAN64;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }
    let a_sign: c_uint = (a >> (64 - 1)) as c_uint;
    let b_sign: c_uint = (b >> (64 - 1)) as c_uint;

    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else if ((a < b) as c_uint ^ a_sign) != 0 {
        b
    } else {
        a
    }
}

// keep the c_int import meaningful for ABI lint friendliness
#[allow(dead_code)]
fn _unused(x: c_int) -> c_int {
    x
}
}

// ── `max_sf128` · softfp_template.h:804-835 · chunk 3253 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t max_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (isnan_sf128(a)) {
//                 if (isnan_sf128(b))
//                     return F_QNAN128;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//     
//         if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3253 {
// Port of softfp_template.h glue(max_sf, 128) -> max_sf128

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << 111);

const FFLAG_INVALID_OP: u32 = 1 << 4;

#[inline]
fn isnan_sf128(a: u128) -> bool {
    let a_exp: u32 = ((a >> 112) & ((1u128 << 15) - 1)) as u32;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    a_exp == ((1u32 << 15) - 1) && a_mant != 0
}

#[inline]
fn issignan_sf128(a: u128) -> bool {
    let a_exp1: u32 = ((a >> 111) & ((1u128 << 16) - 1)) as u32;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    a_exp1 == (2 * ((1u32 << 15) - 1)) && a_mant != 0
}

#[no_mangle]
pub unsafe extern "C" fn max_sf128(a: u128, b: u128, pfflags: *mut u32) -> u128 {
    if isnan_sf128(a) || isnan_sf128(b) {
        if issignan_sf128(a) || issignan_sf128(b) {
            *pfflags |= FFLAG_INVALID_OP;
            return F_QNAN128;
        } else if isnan_sf128(a) {
            if isnan_sf128(b) {
                return F_QNAN128;
            } else {
                return b;
            }
        } else {
            return a;
        }
    }

    let a_sign: u32 = (a >> 127) as u32;
    let b_sign: u32 = (b >> 127) as u32;

    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else if (a < b) ^ (a_sign != 0) {
        b
    } else {
        a
    }
}
}

// ── `if (isnan_sf32(a) || isnan_sf32(b))` · softfp_template.h:808-820 · chunk 3018 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN32;
//             } else if (isnan_sf32(a)) {
//                 if (isnan_sf32(b))
//                     return F_QNAN32;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3018 {
use core::ffi::c_int;

extern "C" {
    fn isnan_sf32(a: u32) -> c_int;
    fn issignan_sf32(a: u32) -> c_int;
}

const F_QNAN32: u32 = (255u32 << 23) | (1u32 << 22);

#[no_mangle]
pub extern "C" fn max_sf32__part3018(
    a: u32,
    b: u32,
    pfflags: *mut u32,
    p_ret: *mut c_int,
) -> u32 {
    unsafe {
        if isnan_sf32(a) != 0 || isnan_sf32(b) != 0 {
            if issignan_sf32(a) != 0 || issignan_sf32(b) != 0 {
                *pfflags |= 1 << 4;
                *p_ret = 1;
                return F_QNAN32;
            } else if isnan_sf32(a) != 0 {
                if isnan_sf32(b) != 0 {
                    *p_ret = 1;
                    return F_QNAN32;
                } else {
                    *p_ret = 1;
                    return b;
                }
            } else {
                *p_ret = 1;
                return a;
            }
        }
        *p_ret = 0;
        0
    }
}
}

// ── `if (isnan_sf64(a) || isnan_sf64(b))` · softfp_template.h:808-820 · chunk 3130 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN64;
//             } else if (isnan_sf64(a)) {
//                 if (isnan_sf64(b))
//                     return F_QNAN64;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3130 {
// Port of the NaN-handling prologue of max_sf64 (softfp_template.h lines 808-820).
// Returns 1 when the C caller must return *pret, 0 when it must fall through.

use core::ffi::c_int;

extern "C" {
    // BOOL isnan_sf64(uint64_t a);
    fn isnan_sf64(a: u64) -> c_int;
    // BOOL issignan_sf64(uint64_t a);
    fn issignan_sf64(a: u64) -> c_int;
}

// static const uint64_t F_QNAN64 = (((uint64_t)((1 << 11) - 1) << 52) | ((uint64_t)1 << (52 - 1)));
const F_QNAN64: u64 = (((0x7ffu64) << 52) | (1u64 << 51));

#[no_mangle]
pub extern "C" fn max_sf64__part3130(
    a: u64,
    b: u64,
    pfflags: *mut u32,
    pret: *mut u64,
) -> c_int {
    unsafe {
        if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 {
            if issignan_sf64(a) != 0 || issignan_sf64(b) != 0 {
                *pfflags |= (1u32 << 4);
                *pret = F_QNAN64;
                return 1;
            } else if isnan_sf64(a) != 0 {
                if isnan_sf64(b) != 0 {
                    *pret = F_QNAN64;
                } else {
                    *pret = b;
                }
                return 1;
            } else {
                *pret = a;
                return 1;
            }
        }
        0
    }
}
}

// ── `if (isnan_sf128(a) || isnan_sf128(b))` · softfp_template.h:808-820 · chunk 3254 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//                 return F_QNAN128;
//             } else if (isnan_sf128(a)) {
//                 if (isnan_sf128(b))
//                     return F_QNAN128;
//                 else
//                     return b;
//             } else {
//                 return a;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3254 {
// Rust rewrite of the NaN-handling part of max_sf128 (softfp_template.h 808-820).
// Returns 1 when the C caller must return *pval, 0 to fall through.

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

const MANT_MASK: u128 = (1u128 << 112) - 1;

fn isnan_sf128(a: u128) -> bool {
    let a_exp = (a >> 112) & ((1u128 << 15) - 1);
    let a_mant = a & MANT_MASK;
    a_exp == ((1u128 << 15) - 1) && a_mant != 0
}

fn issignan_sf128(a: u128) -> bool {
    let a_exp1 = (a >> (112 - 1)) & ((1u128 << (15 + 1)) - 1);
    let a_mant = a & MANT_MASK;
    a_exp1 == (2 * ((1u128 << 15) - 1)) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn max_sf128__part3254(
    a: u128,
    b: u128,
    pfflags: *mut u32,
    pval: *mut u128,
) -> core::ffi::c_int {
    unsafe {
        if isnan_sf128(a) || isnan_sf128(b) {
            if issignan_sf128(a) || issignan_sf128(b) {
                *pfflags |= 1u32 << 4;
                *pval = F_QNAN128;
                return 1;
            } else if isnan_sf128(a) {
                if isnan_sf128(b) {
                    *pval = F_QNAN128;
                    return 1;
                } else {
                    *pval = b;
                    return 1;
                }
            } else {
                *pval = a;
                return 1;
            }
        }
    }
    0
}
}

// ── `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` · softfp_template.h:821-822 · chunk 3019 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3019 {
// Computes the sign bits of two single-precision soft-float values.
// Part of max_sf32: a_sign = a >> 31; b_sign = b >> 31;

#[no_mangle]
pub extern "C" fn max_sf32__part3019(
    a: u32,
    b: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = a >> 31;
        }
        if !b_sign.is_null() {
            *b_sign = b >> 31;
        }
    }
}
}

// ── `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` · softfp_template.h:821-822 · chunk 3131 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3131 {
// Extracted from max_sf64: compute the sign bits of a and b.
use core::ffi::{c_uint, c_ulong};

#[no_mangle]
pub extern "C" fn max_sf64__part3131(
    a: c_ulong,
    b: c_ulong,
    a_sign: *mut c_uint,
    b_sign: *mut c_uint,
) {
    unsafe {
        *a_sign = (a >> (64 - 1)) as c_uint;
        *b_sign = (b >> (64 - 1)) as c_uint;
    }
}
}

// ── `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` · softfp_template.h:821-822 · chunk 3255 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3255 {
// Portion of max_sf128: extract the sign bits of a and b.
#[no_mangle]
pub extern "C" fn max_sf128__part3255(a: u128, b: u128, a_sign: *mut u32, b_sign: *mut u32) {
    unsafe {
        *a_sign = (a >> (128 - 1)) as u32;
        *b_sign = (b >> (128 - 1)) as u32;
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:824-834 · chunk 3020 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3020 {
// Ported piece of max_sf32: choose the max of a and b given their sign bits.
#[no_mangle]
pub extern "C" fn max_sf32__part3020(a: u32, b: u32, a_sign: u32, b_sign: u32) -> u32 {
    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else {
        if (((a < b) as u32) ^ a_sign) != 0 {
            b
        } else {
            a
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:824-834 · chunk 3132 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3132 {
#[no_mangle]
pub extern "C" fn max_sf64__part3132(a: u64, b: u64, a_sign: u32, b_sign: u32) -> u64 {
    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else {
        let lt: u32 = if a < b { 1 } else { 0 };
        if (lt ^ a_sign) != 0 {
            b
        } else {
            a
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:824-834 · chunk 3256 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             if (a_sign)
//                 return b;
//             else
//                 return a;
//         } else {
//             if ((a < b) ^ a_sign)
//                 return b;
//             else
//                 return a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3256 {
// Tail of max_sf128: sign-based select between a and b.
#[no_mangle]
pub extern "C" fn max_sf128__part3256(a: u128, b: u128, a_sign: u32, b_sign: u32) -> u128 {
    if a_sign != b_sign {
        if a_sign != 0 {
            b
        } else {
            a
        }
    } else {
        if ((a < b) as u32 ^ a_sign) != 0 {
            b
        } else {
            a
        }
    }
}
}

// ── `eq_quiet_sf32` · softfp_template.h:837-849 · chunk 3021 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int eq_quiet_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
//     {
//         if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
//     
//         if ((uint32_t)((a | b) << 1) == 0)
//             return 1;
//         return (a == b);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3021 {
// Port of eq_quiet_sf32 (softfp_template.h) for F_SIZE == 32.

use core::ffi::c_int;

extern "C" {
    // BOOL isnan_sf32(uint32_t a);
    fn isnan_sf32(a: u32) -> c_int;
    // BOOL issignan_sf32(uint32_t a);
    fn issignan_sf32(a: u32) -> c_int;
}

#[no_mangle]
pub extern "C" fn eq_quiet_sf32(a: u32, b: u32, pfflags: *mut u32) -> c_int {
    unsafe {
        if isnan_sf32(a) != 0 || isnan_sf32(b) != 0 {
            if issignan_sf32(a) != 0 || issignan_sf32(b) != 0 {
                *pfflags |= 1u32 << 4;
            }
            return 0;
        }

        if (a | b) << 1 == 0 {
            return 1; // zero case
        }
        (a == b) as c_int
    }
}
}

// ── `eq_quiet_sf64` · softfp_template.h:837-849 · chunk 3133 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int eq_quiet_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
//     {
//         if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
//     
//         if ((uint64_t)((a | b) << 1) == 0)
//             return 1;
//         return (a == b);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3133 {
// Port of eq_quiet_sf64 from softfp_template.h

#[no_mangle]
pub extern "C" fn eq_quiet_sf64(a: u64, b: u64, pfflags: *mut u32) -> core::ffi::c_int {
    let a_nan = isnan_sf64(a);
    let b_nan = isnan_sf64(b);

    if a_nan || b_nan {
        if issignan_sf64(a) || issignan_sf64(b) {
            unsafe {
                *pfflags |= (1u32) << 4;
            }
        }
        return 0;
    }

    if ((a | b) << 1) == 0 {
        return 1;
    }
    if a == b {
        1
    } else {
        0
    }
}

fn issignan_sf64(a: u64) -> bool {
    let a_exp1: u32 = ((a >> (52 - 1)) & (((1u64 << (11 + 1)) - 1) as u64)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp1 == (2 * ((1u32 << 11) - 1)) && a_mant != 0
}

fn isnan_sf64(a: u64) -> bool {
    let a_exp: u32 = ((a >> 52) & ((1u64 << 11) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp == ((1u32 << 11) - 1) && a_mant != 0
}
}

// ── `eq_quiet_sf128` · softfp_template.h:837-849 · chunk 3257 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int eq_quiet_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
//     {
//         if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
//     
//         if ((uint128_t)((a | b) << 1) == 0)
//             return 1;
//         return (a == b);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3257 {
// Rust rewrite of eq_quiet_sf128 from softfp_template.h (128-bit soft-float
// quiet equality comparison).

extern "C" {
    // Declared in the C program; BOOL/int arguments converted to c_int.
    fn isnan_sf128(a: u128) -> core::ffi::c_int;
    fn issignan_sf128(a: u128) -> core::ffi::c_int;
}

const FFLAG_INVALID_OP: u32 = 1u32 << 4;

#[no_mangle]
pub unsafe extern "C" fn eq_quiet_sf128(
    a: u128,
    b: u128,
    pfflags: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    if isnan_sf128(a) != 0 || isnan_sf128(b) != 0 {
        if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
            if !pfflags.is_null() {
                *pfflags |= FFLAG_INVALID_OP;
            }
        }
        return 0;
    }

    if (a | b) << 1 == 0 {
        return 1;
    }
    (a == b) as core::ffi::c_int
}
}

// ── `if (isnan_sf32(a) || isnan_sf32(b))` · softfp_template.h:839-844 · chunk 3022 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf32(a) || isnan_sf32(b)) {
//             if (issignan_sf32(a) || issignan_sf32(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3022 {
// Ported piece: the NaN handling of eq_quiet_sf32.
use core::ffi::c_int;

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & 0xff;
    let a_mant = a & 0x7f_ffff;
    a_exp == 0xff && a_mant != 0
}

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1 = (a >> 22) & 0x1ff;
    let a_mant = a & 0x7f_ffff;
    a_exp1 == 0x1fe && a_mant != 0
}

#[no_mangle]
pub extern "C" fn eq_quiet_sf32__part3022(a: u32, b: u32, pfflags: *mut u32) -> c_int {
    if isnan_sf32(a) || isnan_sf32(b) {
        if issignan_sf32(a) || issignan_sf32(b) {
            unsafe {
                *pfflags |= 1 << 4;
            }
        }
        return 1;
    }
    0
}
}

// ── `if (isnan_sf64(a) || isnan_sf64(b))` · softfp_template.h:839-844 · chunk 3134 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf64(a) || isnan_sf64(b)) {
//             if (issignan_sf64(a) || issignan_sf64(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3134 {
// Port of the NaN-handling prologue of eq_quiet_sf64 (softfp_template.h 839-844).
// isnan_sf64 / issignan_sf64 logic is inlined privately (pure bit tests).

use core::ffi::c_int;

fn isnan_sf64_bits(a: u64) -> bool {
    let a_exp: u32 = ((a >> 52) & ((1u64 << 11) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp == ((1u32 << 11) - 1) && a_mant != 0
}

fn issignan_sf64_bits(a: u64) -> bool {
    let a_exp1: u32 = ((a >> (52 - 1)) & ((1u64 << (11 + 1)) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp1 == 2u32 * ((1u32 << 11) - 1) && a_mant != 0
}

/// Returns 1 when the NaN branch was taken (caller must `return 0;`),
/// 0 when neither operand is NaN and the caller continues.
#[no_mangle]
pub extern "C" fn eq_quiet_sf64__part3134(
    a: u64,
    b: u64,
    pfflags: *mut u32,
) -> c_int {
    if isnan_sf64_bits(a) || isnan_sf64_bits(b) {
        if issignan_sf64_bits(a) || issignan_sf64_bits(b) {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
        }
        return 1;
    }
    0
}
}

// ── `if (isnan_sf128(a) || isnan_sf128(b))` · softfp_template.h:839-844 · chunk 3258 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf128(a) || isnan_sf128(b)) {
//             if (issignan_sf128(a) || issignan_sf128(b)) {
//                 *pfflags |= (1 << 4);
//             }
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3258 {
// Portion of eq_quiet_sf128: the quiet/signaling NaN handling.
// Returns 1 if the NaN branch was taken (caller must `return 0`), 0 otherwise.

use core::ffi::c_int;

extern "C" {
    fn isnan_sf128(a: u128) -> c_int;
    fn issignan_sf128(a: u128) -> c_int;
}

#[no_mangle]
pub extern "C" fn eq_quiet_sf128__part3258(a: u128, b: u128, pfflags: *mut u32) -> c_int {
    unsafe {
        if isnan_sf128(a) != 0 || isnan_sf128(b) != 0 {
            if issignan_sf128(a) != 0 || issignan_sf128(b) != 0 {
                *pfflags |= 1 << 4;
            }
            return 1;
        }
    }
    0
}
}

// ── `if ((uint32_t)((a | b) << 1) == 0)` · softfp_template.h:846-847 · chunk 3023 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((uint32_t)((a | b) << 1) == 0)
//             return 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3023 {
use core::ffi::{c_int, c_uint};

// Ported piece of eq_quiet_sf32: the "both operands are zero" check
//   if ((uint32_t)((a | b) << 1) == 0) return 1;
// Returns 1 when the condition holds, 0 otherwise.
#[no_mangle]
pub extern "C" fn eq_quiet_sf32__part3023(a: c_uint, b: c_uint) -> c_int {
    let or_shifted: u32 = (a | b) << 1;
    if or_shifted == 0 {
        1
    } else {
        0
    }
}
}

// ── `if ((uint64_t)((a | b) << 1) == 0)` · softfp_template.h:846-847 · chunk 3135 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((uint64_t)((a | b) << 1) == 0)
//             return 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3135 {
// Portion of eq_quiet_sf64: the both-zero (equal zeros) test.
// Returns 1 when (uint64_t)((a | b) << 1) == 0, otherwise 0.
#[no_mangle]
pub extern "C" fn eq_quiet_sf64__part3135(a: u64, b: u64) -> core::ffi::c_int {
    if (a | b) << 1 == 0 {
        1
    } else {
        0
    }
}
}

// ── `if ((uint128_t)((a | b) << 1) == 0)` · softfp_template.h:846-847 · chunk 3259 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((uint128_t)((a | b) << 1) == 0)
//             return 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3259 {
// Port of the zero-check branch of eq_quiet_sf128.
// Returns 1 when both operands are zero (ignoring sign bits), else 0.

#[no_mangle]
pub extern "C" fn eq_quiet_sf128__part3259(a: u128, b: u128) -> core::ffi::c_int {
    // (uint128_t)((a | b) << 1) == 0  -- unsigned shift, top bit shifted out.
    if (a | b) << 1 == 0 {
        1
    } else {
        0
    }
}
}

// ── `return (a == b);` · softfp_template.h:848-848 · chunk 3024 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (a == b);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3024 {
// Ported piece of eq_quiet_sf32: final integer-equality test, no NaN involved.
#[no_mangle]
pub extern "C" fn eq_quiet_sf32__part3024(a: u32, b: u32) -> core::ffi::c_int {
    (a == b) as core::ffi::c_int
}
}

// ── `return (a == b);` · softfp_template.h:848-848 · chunk 3136 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (a == b);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3136 {
// Returns 1 if the two 64-bit soft-float bit patterns are equal, else 0.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn eq_quiet_sf64__part3136(a: u64, b: u64) -> c_int {
    (a == b) as c_int
}
}

// ── `return (a == b);` · softfp_template.h:848-848 · chunk 3260 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (a == b);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3260 {
// Ported piece of eq_quiet_sf128: the final equality comparison.
// C: return (a == b);

#[no_mangle]
pub extern "C" fn eq_quiet_sf128__part3260(a: u128, b: u128) -> core::ffi::c_int {
    // C's `a == b` yields int 0 or 1.
    if a == b {
        1
    } else {
        0
    }
}
}

// ── `le_sf32` · softfp_template.h:851-871 · chunk 3025 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int le_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf32(a) || isnan_sf32(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign || ((uint32_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3025 {
// Port of `int le_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)`.
// `isnan_sf32` is inlined (it is a plain global function in the C program,
// but its trivial bit-test logic is reproduced here exactly).

const FFLAG_INVALID_OP: u32 = 1 << 4;

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn le_sf32(a: u32, b: u32, pfflags: *mut u32) -> core::ffi::c_int {
    if isnan_sf32(a) || isnan_sf32(b) {
        unsafe {
            *pfflags |= FFLAG_INVALID_OP;
        }
        return 0;
    }

    let a_sign: u32 = a >> (32 - 1);
    let b_sign: u32 = b >> (32 - 1);
    if a_sign != b_sign {
        return (a_sign != 0 || (a | b) << 1 == 0) as core::ffi::c_int;
    } else {
        if a_sign != 0 {
            return (a >= b) as core::ffi::c_int;
        } else {
            return (a <= b) as core::ffi::c_int;
        }
    }
}
}

// ── `le_sf64` · softfp_template.h:851-871 · chunk 3137 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int le_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf64(a) || isnan_sf64(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign || ((uint64_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3137 {
// Port of le_sf64 from softfp_template.h (lines 851-871).

use core::ffi::c_int;

#[inline]
fn isnan_sf64(a: u64) -> bool {
    let a_exp: u32 = ((a >> 52) & ((1u64 << 11) - 1)) as u32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp == ((1u32 << 11) - 1) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn le_sf64(a: u64, b: u64, pfflags: *mut u32) -> c_int {
    if isnan_sf64(a) || isnan_sf64(b) {
        unsafe {
            *pfflags |= 1u32 << 4;
        }
        return 0;
    }

    let a_sign: u32 = (a >> (64 - 1)) as u32;
    let b_sign: u32 = (b >> (64 - 1)) as u32;
    if a_sign != b_sign {
        let both_shifted_zero = ((a | b) << 1) == 0;
        return if a_sign != 0 || both_shifted_zero { 1 } else { 0 };
    } else if a_sign != 0 {
        return if a >= b { 1 } else { 0 };
    } else {
        return if a <= b { 1 } else { 0 };
    }
}
}

// ── `le_sf128` · softfp_template.h:851-871 · chunk 3261 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int le_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf128(a) || isnan_sf128(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign || ((uint128_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3261 {
// Port of le_sf128 (softfp_template.h) for the 128-bit softfloat type.
use core::ffi::{c_int, c_uint};

const FFLAG_INVALID_OP: c_uint = 1 << 4;

// Same logic as isnan_sf128: exponent all ones and nonzero mantissa.
#[inline]
fn isnan_sf128(a: u128) -> bool {
    let a_exp: c_uint = ((a >> 112) & (((1u128 << 15) - 1))) as c_uint;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    a_exp == ((1u32 << 15) - 1) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn le_sf128(a: u128, b: u128, pfflags: *mut c_uint) -> c_int {
    let a_sign: c_uint;
    let b_sign: c_uint;

    if isnan_sf128(a) || isnan_sf128(b) {
        unsafe {
            *pfflags |= FFLAG_INVALID_OP;
        }
        return 0;
    }

    a_sign = (a >> (128 - 1)) as c_uint;
    b_sign = (b >> (128 - 1)) as c_uint;
    if a_sign != b_sign {
        if a_sign != 0 || ((a | b) << 1) == 0 {
            1
        } else {
            0
        }
    } else if a_sign != 0 {
        if a >= b {
            1
        } else {
            0
        }
    } else if a <= b {
        1
    } else {
        0
    }
}
}

// ── `if (isnan_sf32(a) || isnan_sf32(b))` · softfp_template.h:855-858 · chunk 3026 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf32(a) || isnan_sf32(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3026 {
// Port of the NaN-check prologue of le_sf32 (softfp_template.h lines 855-858).
// Returns nonzero when the NaN early-return path is taken (flag already set),
// so the C caller can `return 0;`.

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn le_sf32__part3026(a: u32, b: u32, pfflags: *mut u32) -> core::ffi::c_int {
    if isnan_sf32(a) || isnan_sf32(b) {
        unsafe {
            *pfflags |= 1u32 << 4;
        }
        return 1;
    }
    0
}
}

// ── `if (isnan_sf64(a) || isnan_sf64(b))` · softfp_template.h:855-858 · chunk 3138 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf64(a) || isnan_sf64(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3138 {
use core::ffi::c_int;

extern "C" {
    // BOOL isnan_sf64(uint64_t a) -- non-static, defined in the C program
    fn isnan_sf64(a: u64) -> c_int;
}

#[no_mangle]
pub extern "C" fn le_sf64__part3138(a: u64, b: u64, pfflags: *mut u32) -> c_int {
    unsafe {
        if isnan_sf64(a) != 0 || isnan_sf64(b) != 0 {
            *pfflags |= 1u32 << 4;
            return 1;
        }
    }
    0
}
}

// ── `if (isnan_sf128(a) || isnan_sf128(b))` · softfp_template.h:855-858 · chunk 3262 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf128(a) || isnan_sf128(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3262 {
// Port of the NaN check at the head of le_sf128.
// Returns nonzero when the NaN path was taken (the C caller then returns 0).

use core::ffi::c_int;

extern "C" {
    fn isnan_sf128(a: u128) -> c_int;
}

#[no_mangle]
pub extern "C" fn le_sf128__part3262(a: u128, b: u128, pfflags: *mut u32) -> c_int {
    let isnan_a = unsafe { isnan_sf128(a) };
    // keep C short-circuit evaluation of `isnan_sf128(a) || isnan_sf128(b)`
    let mut take_branch = isnan_a != 0;
    if !take_branch {
        let isnan_b = unsafe { isnan_sf128(b) };
        take_branch = isnan_b != 0;
    }
    if take_branch {
        if !pfflags.is_null() {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
        }
        1
    } else {
        0
    }
}
}

// ── `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` · softfp_template.h:860-861 · chunk 3027 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3027 {
// Port of softfp_template.h lines 860-861 (inside le_sf32).
// Extracts the sign bits of the two softfloat single-precision operands.

use core::ffi::c_uint;

#[no_mangle]
pub unsafe extern "C" fn le_sf32__part3027(
    a: c_uint,
    b: c_uint,
    a_sign: *mut c_uint,
    b_sign: *mut c_uint,
) {
    let a = a as u32;
    let b = b as u32;

    let asg: u32 = a >> (32 - 1);
    let bsg: u32 = b >> (32 - 1);

    if !a_sign.is_null() {
        *a_sign = asg as c_uint;
    }
    if !b_sign.is_null() {
        *b_sign = bsg as c_uint;
    }
}
}

// ── `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` · softfp_template.h:860-861 · chunk 3139 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3139 {
// Computes the sign bits of two sfloat64 (raw uint64_t) values.
// Mirrors:
//     a_sign = a >> (64 - 1);
//     b_sign = b >> (64 - 1);
#[no_mangle]
pub extern "C" fn le_sf64__part3139(
    a: u64,
    b: u64,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        *a_sign = (a >> 63) as u32;
        *b_sign = (b >> 63) as u32;
    }
}
}

// ── `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` · softfp_template.h:860-861 · chunk 3263 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3263 {
// Extracted piece of le_sf128: compute the sign bits of a and b.
#[no_mangle]
pub extern "C" fn le_sf128__part3263(
    a: u128,
    b: u128,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        *a_sign = (a >> 127) as u32;
        *b_sign = (b >> 127) as u32;
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:862-870 · chunk 3028 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign || ((uint32_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3028 {
// Tail comparison branch of le_sf32.
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn le_sf32__part3028(a: c_uint, b: c_uint, a_sign: c_uint, b_sign: c_uint) -> c_int {
    if a_sign != b_sign {
        let z = a | b;
        if a_sign != 0 || z.wrapping_shl(1) == 0 {
            1
        } else {
            0
        }
    } else if a_sign != 0 {
        if a >= b {
            1
        } else {
            0
        }
    } else if a <= b {
        1
    } else {
        0
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:862-870 · chunk 3140 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign || ((uint64_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3140 {
#[no_mangle]
pub extern "C" fn le_sf64__part3140(a: u64, b: u64, a_sign: u32) -> core::ffi::c_int {
    let b_sign: u32 = (b >> 63) as u32;
    if a_sign != b_sign {
        if a_sign != 0 {
            1
        } else if a.wrapping_shl(1) | b.wrapping_shl(1) == 0 {
            0
        } else {
            0
        }
    } else {
        if a_sign != 0 {
            if a >= b {
                1
            } else {
                0
            }
        } else {
            if a <= b {
                1
            } else {
                0
            }
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:862-870 · chunk 3264 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign || ((uint128_t)((a | b) << 1) == 0));
//         } else {
//             if (a_sign) {
//                 return (a >= b);
//             } else {
//                 return (a <= b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3264 {
// Comparison tail of le_sf128: signs differ / same-sign unsigned compare.
#[no_mangle]
pub extern "C" fn le_sf128__part3264(
    a: u128,
    b: u128,
    a_sign: u32,
    b_sign: u32,
) -> core::ffi::c_int {
    if a_sign != b_sign {
        // a_sign || ((uint128_t)((a | b) << 1) == 0)
        let either_zero = ((a | b) << 1) == 0;
        return (a_sign != 0 || either_zero) as core::ffi::c_int;
    }
    if a_sign != 0 {
        (a >= b) as core::ffi::c_int
    } else {
        (a <= b) as core::ffi::c_int
    }
}
}

// ── `lt_sf32` · softfp_template.h:873-893 · chunk 3029 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int lt_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf32(a) || isnan_sf32(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign && ((uint32_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3029 {
// Port of lt_sf32 from softfp_template.h

#[inline]
fn isnan_sf32(a: u32) -> bool {
    let a_exp = (a >> 23) & ((1u32 << 8) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp == ((1u32 << 8) - 1) && a_mant != 0
}

#[no_mangle]
pub unsafe extern "C" fn lt_sf32(a: u32, b: u32, pfflags: *mut u32) -> core::ffi::c_int {
    if isnan_sf32(a) || isnan_sf32(b) {
        if !pfflags.is_null() {
            *pfflags |= 1u32 << 4;
        }
        return 0;
    }

    let a_sign = a >> (32 - 1);
    let b_sign = b >> (32 - 1);
    if a_sign != b_sign {
        (a_sign != 0 && ((a | b) << 1) != 0) as core::ffi::c_int
    } else {
        if a_sign != 0 {
            (a > b) as core::ffi::c_int
        } else {
            (a < b) as core::ffi::c_int
        }
    }
}
}

// ── `lt_sf64` · softfp_template.h:873-893 · chunk 3141 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int lt_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf64(a) || isnan_sf64(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign && ((uint64_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3141 {
// Rust rewrite of softfloat lt_sf64.

#[no_mangle]
pub extern "C" fn lt_sf64(a: u64, b: u64, pfflags: *mut u32) -> core::ffi::c_int {
    let isnan = |x: u64| -> bool {
        let a_exp = (x >> 52) & ((1u64 << 11) - 1);
        let a_mant = x & ((1u64 << 52) - 1);
        a_exp == ((1u64 << 11) - 1) && a_mant != 0
    };

    if isnan(a) || isnan(b) {
        unsafe {
            *pfflags |= 1u32 << 4;
        }
        return 0;
    }

    let a_sign: u32 = (a >> (64 - 1)) as u32;
    let b_sign: u32 = (b >> (64 - 1)) as u32;
    if a_sign != b_sign {
        return (a_sign != 0 && ((a | b) << 1) != 0) as core::ffi::c_int;
    } else {
        if a_sign != 0 {
            (a > b) as core::ffi::c_int
        } else {
            (a < b) as core::ffi::c_int
        }
    }
}
}

// ── `lt_sf128` · softfp_template.h:873-893 · chunk 3265 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int lt_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
//     {
//         uint32_t a_sign, b_sign;
//     
//         if (isnan_sf128(a) || isnan_sf128(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
//     
//         a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
//         if (a_sign != b_sign) {
//             return (a_sign && ((uint128_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3265 {
// Port of lt_sf128 from softfp_template.h
use core::ffi::{c_int, c_uint};

extern "C" {
    fn isnan_sf128(a: u128) -> c_int;
}

#[no_mangle]
pub extern "C" fn lt_sf128(a: u128, b: u128, pfflags: *mut c_uint) -> c_int {
    unsafe {
        if isnan_sf128(a) != 0 || isnan_sf128(b) != 0 {
            *pfflags |= 1u32 << 4;
            return 0;
        }
    }

    let a_sign: c_uint = (a >> (128 - 1)) as c_uint;
    let b_sign: c_uint = (b >> (128 - 1)) as c_uint;
    if a_sign != b_sign {
        return (a_sign != 0 && ((a | b) << 1) != 0) as c_int;
    } else if a_sign != 0 {
        (a > b) as c_int
    } else {
        (a < b) as c_int
    }
}
}

// ── `if (isnan_sf32(a) || isnan_sf32(b))` · softfp_template.h:877-880 · chunk 3030 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf32(a) || isnan_sf32(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3030 {
// Portion of lt_sf32: NaN check + invalid-operation flag.
use core::ffi::c_int;

extern "C" {
    fn isnan_sf32(a: u32) -> c_int;
}

/// Returns 1 if the comparison must yield 0 (a or b is NaN) after raising the
/// invalid-operation flag, 0 otherwise.
#[no_mangle]
pub extern "C" fn lt_sf32__part3030(a: u32, b: u32, pfflags: *mut u32) -> c_int {
    unsafe {
        if isnan_sf32(a) != 0 || isnan_sf32(b) != 0 {
            *pfflags |= 1u32 << 4;
            return 1;
        }
    }
    0
}
}

// ── `if (isnan_sf64(a) || isnan_sf64(b))` · softfp_template.h:877-880 · chunk 3142 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf64(a) || isnan_sf64(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3142 {
use core::ffi::{c_int, c_uint};

// static-in-C-like helper: mirrors isnan_sf64 for sfloat64
#[inline]
fn isnan_sf64(a: u64) -> bool {
    let a_exp: c_uint = ((a >> 52) & ((1u64 << 11) - 1)) as c_uint;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    a_exp == (((1u64 << 11) - 1) as c_uint) && a_mant != 0
}

#[no_mangle]
pub extern "C" fn lt_sf64__part3142(a: u64, b: u64, pfflags: *mut c_uint) -> c_int {
    if isnan_sf64(a) || isnan_sf64(b) {
        unsafe {
            *pfflags |= 1 << 4;
        }
        return 1;
    }
    0
}
}

// ── `if (isnan_sf128(a) || isnan_sf128(b))` · softfp_template.h:877-880 · chunk 3266 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (isnan_sf128(a) || isnan_sf128(b)) {
//             *pfflags |= (1 << 4);
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3266 {
// Port of the NaN check at the head of lt_sf128 (softfp_template.h lines 877-880).
// Returns 1 when either operand is NaN (caller must then `return 0`), 0 otherwise.

use core::ffi::c_int;

extern "C" {
    // BOOL isnan_sf128(uint128_t a);  -- BOOL is `typedef int BOOL;`
    fn isnan_sf128(a: u128) -> c_int;
}

#[no_mangle]
pub extern "C" fn lt_sf128__part3266(a: u128, b: u128, pfflags: *mut u32) -> c_int {
    unsafe {
        if isnan_sf128(a) != 0 || isnan_sf128(b) != 0 {
            *pfflags |= 1u32 << 4;
            return 1;
        }
    }
    0
}
}

// ── `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` · softfp_template.h:882-883 · chunk 3031 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (32 - 1);
//         b_sign = b >> (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3031 {
// Computes the sign bits of the two sfloat32 operands.
// a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);
#[no_mangle]
pub extern "C" fn lt_sf32__part3031(
    a: u32,
    b: u32,
    a_sign: *mut u32,
    b_sign: *mut u32,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = a >> (32 - 1);
        }
        if !b_sign.is_null() {
            *b_sign = b >> (32 - 1);
        }
    }
}
}

// ── `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` · softfp_template.h:882-883 · chunk 3143 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (64 - 1);
//         b_sign = b >> (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3143 {
#[no_mangle]
pub extern "C" fn lt_sf64__part3143(a: u64, b: u64, a_sign: *mut u32, b_sign: *mut u32) {
    unsafe {
        *a_sign = (a >> 63) as u32;
        *b_sign = (b >> 63) as u32;
    }
}
}

// ── `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` · softfp_template.h:882-883 · chunk 3267 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_sign = a >> (128 - 1);
//         b_sign = b >> (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3267 {
// Extracted from lt_sf128: computes the sign bits of a and b.
#[no_mangle]
pub extern "C" fn lt_sf128__part3267(a: u128, b: u128, a_sign: *mut u32, b_sign: *mut u32) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = (a >> 127) as u32;
        }
        if !b_sign.is_null() {
            *b_sign = (b >> 127) as u32;
        }
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:884-892 · chunk 3032 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign && ((uint32_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3032 {
// Port of the sign/ordering decision block of lt_sf32 (softfp_template.h lines 884-892).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn lt_sf32__part3032(a: u32, b: u32, a_sign: u32, b_sign: u32) -> c_int {
    if a_sign != b_sign {
        // return (a_sign && ((uint32_t)((a | b) << 1) != 0));
        let nonzero = a_sign != 0 && ((a | b) << 1) != 0;
        nonzero as c_int
    } else if a_sign != 0 {
        (a > b) as c_int
    } else {
        (a < b) as c_int
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:884-892 · chunk 3144 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign && ((uint64_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3144 {
// Part of lt_sf64: compares two soft-float64 bit patterns for "less than".
#[no_mangle]
pub extern "C" fn lt_sf64__part3144(a: u64, b: u64, a_sign: u32, b_sign: u32) -> core::ffi::c_int {
    if a_sign != b_sign {
        let same = a_sign != 0 && (a | b) << 1 != 0;
        if same {
            1
        } else {
            0
        }
    } else if a_sign != 0 {
        if a > b {
            1
        } else {
            0
        }
    } else if a < b {
        1
    } else {
        0
    }
}
}

// ── `if (a_sign != b_sign) … else` · softfp_template.h:884-892 · chunk 3268 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign != b_sign) {
//             return (a_sign && ((uint128_t)((a | b) << 1) != 0));
//         } else {
//             if (a_sign) {
//                 return (a > b);
//             } else {
//                 return (a < b);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3268 {
// Comparison tail of lt_sf128 (softfloat-style 128-bit binary float lt).
// a and b are passed by pointer to stay ABI-identical to C's unsigned __int128.

#[no_mangle]
pub extern "C" fn lt_sf128__part3268(
    pa: *const u128,
    pb: *const u128,
    a_sign: u32,
    b_sign: u32,
) -> core::ffi::c_int {
    unsafe {
        let a: u128 = *pa;
        let b: u128 = *pb;
        if a_sign != b_sign {
            if a_sign != 0 && (a | b) << 1 != 0 {
                return 1;
            }
            return 0;
        }
        if a_sign != 0 {
            if a > b {
                return 1;
            }
            return 0;
        }
        if a < b {
            return 1;
        }
        0
    }
}
}

// ── `fclass_sf32` · softfp_template.h:895-936 · chunk 3033 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t fclass_sf32(uint32_t a)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint32_t)1 << (23 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3033 {
// Port of fclass_sf32 (softfp_template.h) for F_SIZE == 32.
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn fclass_sf32(a: c_uint) -> c_uint {
    let a_sign: c_uint;
    let a_exp: i32;
    let a_mant: c_uint;
    let ret: c_uint;

    a_sign = a >> (32 - 1);
    a_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    a_mant = a & ((1u32 << 23) - 1);
    if a_exp == ((1 << 8) - 1) {
        if a_mant != 0 {
            if a_mant & (1u32 << (23 - 1)) != 0 {
                ret = 1 << 9;
            } else {
                ret = 1 << 8;
            }
        } else {
            if a_sign != 0 {
                ret = 1 << 0;
            } else {
                ret = 1 << 7;
            }
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                ret = 1 << 3;
            } else {
                ret = 1 << 4;
            }
        } else {
            if a_sign != 0 {
                ret = 1 << 2;
            } else {
                ret = 1 << 5;
            }
        }
    } else {
        if a_sign != 0 {
            ret = 1 << 1;
        } else {
            ret = 1 << 6;
        }
    }
    ret
}
}

// ── `fclass_sf64` · softfp_template.h:895-936 · chunk 3145 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t fclass_sf64(uint64_t a)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint64_t)1 << (52 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3145 {
#[no_mangle]
pub extern "C" fn fclass_sf64(a: u64) -> u32 {
    let a_sign: u32 = (a >> (64 - 1)) as u32;
    let a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let a_mant: u64 = a & ((1u64 << 52) - 1);
    let ret: u32;

    if a_exp == ((1i32 << 11) - 1) {
        if a_mant != 0 {
            if (a_mant & (1u64 << (52 - 1))) != 0 {
                ret = 1 << 9;
            } else {
                ret = 1 << 8;
            }
        } else if a_sign != 0 {
            ret = 1 << 0;
        } else {
            ret = 1 << 7;
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                ret = 1 << 3;
            } else {
                ret = 1 << 4;
            }
        } else if a_sign != 0 {
            ret = 1 << 2;
        } else {
            ret = 1 << 5;
        }
    } else if a_sign != 0 {
        ret = 1 << 1;
    } else {
        ret = 1 << 6;
    }
    ret
}
}

// ── `fclass_sf128` · softfp_template.h:895-936 · chunk 3269 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t fclass_sf128(uint128_t a)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint128_t)1 << (112 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3269 {
// Port of softfp_template.h fclass_sf128 (uint32_t fclass_sf128(uint128_t a))

#[no_mangle]
pub extern "C" fn fclass_sf128(a: u128) -> u32 {
    let a_sign: u32 = (a >> 127) as u32;
    let a_exp: i32 = (((a >> 112) & 0x7fff) as i32);
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    let ret: u32;

    if a_exp == 0x7fff {
        if a_mant != 0 {
            if (a_mant & (1u128 << 111)) != 0 {
                ret = 1u32 << 9;
            } else {
                ret = 1u32 << 8;
            }
        } else if a_sign != 0 {
            ret = 1u32 << 0;
        } else {
            ret = 1u32 << 7;
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                ret = 1u32 << 3;
            } else {
                ret = 1u32 << 4;
            }
        } else if a_sign != 0 {
            ret = 1u32 << 2;
        } else {
            ret = 1u32 << 5;
        }
    } else if a_sign != 0 {
        ret = 1u32 << 1;
    } else {
        ret = 1u32 << 6;
    }
    ret
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint32_t a_mant; uint32_t ret; a_sign = a >> (32 - 1); a_exp =…` · softfp_template.h:897-904 · chunk 3034 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3034 {
// Decomposes a 32-bit single-precision float bit pattern into sign, exponent
// and mantissa fields. Part of fclass_sf32.

#[no_mangle]
pub extern "C" fn fclass_sf32__part3034(
    a: u32,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u32,
) {
    unsafe {
        *a_sign = a >> (32 - 1);
        *a_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
        *a_mant = a & ((1u32 << 23) - 1);
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; uint32_t ret; a_sign = a >> (64 - 1); a_exp =…` · softfp_template.h:897-904 · chunk 3146 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3146 {
// Port of the header-extraction part of fclass_sf64: unpacks the sign,
// biased exponent and mantissa fields of a binary64 bit pattern.
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn fclass_sf64__part3146(
    a: u64,
    a_sign: *mut c_uint,
    a_exp: *mut c_int,
    a_mant: *mut u64,
    ret: *mut c_uint,
) {
    let sign: c_uint = (a >> (64 - 1)) as c_uint;
    let exp: c_int = ((a >> 52) & ((1u64 << 11) - 1)) as c_int;
    let mant: u64 = a & ((1u64 << 52) - 1);

    unsafe {
        if !a_sign.is_null() {
            *a_sign = sign;
        }
        if !a_exp.is_null() {
            *a_exp = exp;
        }
        if !a_mant.is_null() {
            *a_mant = mant;
        }
        if !ret.is_null() {
            *ret = 0;
        }
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; uint32_t ret; a_sign = a >> (128 - 1); a_exp…` · softfp_template.h:897-904 · chunk 3270 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint32_t ret;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3270 {
// Port of the unpacking part of fclass_sf128.
#[no_mangle]
pub extern "C" fn fclass_sf128__part3270(
    a: u128,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u128,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = (a >> (128 - 1)) as u32;
        }
        if !a_exp.is_null() {
            *a_exp = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
        }
        if !a_mant.is_null() {
            *a_mant = a & ((1u128 << 112) - 1);
        }
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) … else` · softfp_template.h:905-934 · chunk 3035 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint32_t)1 << (23 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3035 {
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn fclass_sf32__part3035(a_sign: c_uint, a_exp: c_int, a_mant: c_uint) -> c_uint {
    let ret: c_uint;
    if a_exp == ((1 << 8) - 1) {
        if a_mant != 0 {
            if (a_mant & (1u32 << (23 - 1))) != 0 {
                ret = 1 << 9;
            } else {
                ret = 1 << 8;
            }
        } else if a_sign != 0 {
            ret = 1 << 0;
        } else {
            ret = 1 << 7;
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                ret = 1 << 3;
            } else {
                ret = 1 << 4;
            }
        } else if a_sign != 0 {
            ret = 1 << 2;
        } else {
            ret = 1 << 5;
        }
    } else if a_sign != 0 {
        ret = 1 << 1;
    } else {
        ret = 1 << 6;
    }
    ret
}
}

// ── `if (a_exp == ((1 << 11) - 1) … else` · softfp_template.h:905-934 · chunk 3147 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint64_t)1 << (52 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3147 {
// Port of the class-selection branch chain of fclass_sf64.
#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn fclass_sf64__part3147(a_sign: u32, a_exp: i32, a_mant: u64) -> u32 {
    let ret: u32;

    if a_exp == ((1i32 << 11) - 1) {
        if a_mant != 0 {
            if (a_mant & ((1u64) << (52 - 1))) != 0 {
                ret = 1u32 << 9;
            } else {
                ret = 1u32 << 8;
            }
        } else if a_sign != 0 {
            ret = 1u32 << 0;
        } else {
            ret = 1u32 << 7;
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                ret = 1u32 << 3;
            } else {
                ret = 1u32 << 4;
            }
        } else if a_sign != 0 {
            ret = 1u32 << 2;
        } else {
            ret = 1u32 << 5;
        }
    } else if a_sign != 0 {
        ret = 1u32 << 1;
    } else {
        ret = 1u32 << 6;
    }

    ret
}
}

// ── `if (a_exp == ((1 << 15) - 1) … else` · softfp_template.h:905-934 · chunk 3271 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//                 if (a_mant & ((uint128_t)1 << (112 - 1)))
//                     ret = (1 << 9);
//                 else
//                     ret = (1 << 8);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 0);
//                 else
//                     ret = (1 << 7);
//             }
//         } else if (a_exp == 0) {
//             if (a_mant == 0) {
//                 if (a_sign)
//                     ret = (1 << 3);
//                 else
//                     ret = (1 << 4);
//             } else {
//                 if (a_sign)
//                     ret = (1 << 2);
//                 else
//                     ret = (1 << 5);
//             }
//         } else {
//             if (a_sign)
//                 ret = (1 << 1);
//             else
//                 ret = (1 << 6);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3271 {
// Classification of a binary128 value (part of fclass_sf128).
#[no_mangle]
pub extern "C" fn fclass_sf128__part3271(a_sign: u32, a_exp: i32, a_mant: u128) -> u32 {
    if a_exp == (1i32 << 15) - 1 {
        if a_mant != 0 {
            if a_mant & (1u128 << (112 - 1)) != 0 {
                1u32 << 9
            } else {
                1u32 << 8
            }
        } else if a_sign != 0 {
            1u32 << 0
        } else {
            1u32 << 7
        }
    } else if a_exp == 0 {
        if a_mant == 0 {
            if a_sign != 0 {
                1u32 << 3
            } else {
                1u32 << 4
            }
        } else if a_sign != 0 {
            1u32 << 2
        } else {
            1u32 << 5
        }
    } else if a_sign != 0 {
        1u32 << 1
    } else {
        1u32 << 6
    }
}
}

// ── `return ret;` · softfp_template.h:935-935 · chunk 3036 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3036 {
// Identity pass-through of the classification result computed by the caller.
#[no_mangle]
pub extern "C" fn fclass_sf32__part3036(ret: u32) -> u32 {
    ret
}
}

// ── `return ret;` · softfp_template.h:935-935 · chunk 3148 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3148 {
// Returns the value produced by the classification logic.
#[no_mangle]
pub extern "C" fn fclass_sf64__part3148(ret: u32) -> u32 {
    ret
}
}

// ── `return ret;` · softfp_template.h:935-935 · chunk 3272 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3272 {
// Piece of fclass_sf128: returns the computed class value unchanged.
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn fclass_sf128__part3272(ret: c_uint) -> c_uint {
    ret
}
}

// ── `cvt_sf32_sf64` · softfp_template.h:942-973 · chunk 3149 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_sf32_sf64(uint32_t a, uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_mant = unpack_sf32(&a_sign, &a_exp, a);
//         if (a_exp == 0xff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 return pack_sf64(a_sign, ((1 << 11) - 1), 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         }
//     
//         a_exp = a_exp - 0x7f + (((1 << 11) - 1) / 2);
//     
//         a_mant <<= (52 - 23);
//     
//     
//         return pack_sf64(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3149 {
use core::ffi::{c_int, c_uint};

const FFLAG_INVALID_OP: u32 = 1 << 4;
const F_QNAN64: u64 = ((1u64 << 11) - 1) << 52 | (1u64 << (52 - 1));

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn unpack_sf32(pa_sign: &mut u32, pa_exp: &mut i32, a: u32) -> u32 {
    *pa_sign = a >> (32 - 1);
    *pa_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    a & ((1u32 << 23) - 1)
}

#[inline]
fn issignan_sf32(a: u32) -> bool {
    let a_exp1: c_uint = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant = a & ((1u32 << 23) - 1);
    a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0
}

#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - (32 - 1 - clz32(a_mant));
    *pa_exp = 1 - shift;
    ((a_mant as u64) << (shift as u32)) as u32
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf64(a: u32, pfflags: *mut u32) -> u64 {
    let mut a_sign: u32 = 0;
    let mut a_exp: i32 = 0;
    let mut a_mant: u64;

    a_mant = unpack_sf32(&mut a_sign, &mut a_exp, a) as u64;
    if a_exp == 0xff {
        if a_mant != 0 {
            if issignan_sf32(a) {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN64;
        } else {
            return pack_sf64(a_sign, (1u32 << 11) - 1, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf64(a_sign, 0, 0);
        }
        a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant as u32) as u64;
    }

    a_exp = a_exp - 0x7f + (((1i32 << 11) - 1) / 2);

    a_mant <<= 52 - 23;

    pack_sf64(a_sign, a_exp as u32, a_mant)
}
}

// ── `cvt_sf32_sf128` · softfp_template.h:942-973 · chunk 3273 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_sf32_sf128(uint32_t a, uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf32(&a_sign, &a_exp, a);
//         if (a_exp == 0xff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 return pack_sf128(a_sign, ((1 << 15) - 1), 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         }
//     
//         a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2);
//     
//         a_mant <<= (112 - 23);
//     
//     
//         return pack_sf128(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3273 {
// Port of cvt_sf32_sf128() (softfp_template.h) for the uint128_t target type.
// All helpers used by the C original (unpack_sf32, issignan_sf32,
// normalize_subnormal_sf32, pack_sf128) are mirrored privately below.

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

// static inline uint32_t unpack_sf32(uint32_t *pa_sign, int32_t *pa_exp, uint32_t a)
#[inline]
fn unpack_sf32(pa_sign: &mut u32, pa_exp: &mut i32, a: u32) -> u32 {
    *pa_sign = a >> (32 - 1);
    *pa_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    a & ((1u32 << 23) - 1)
}

// static inline int clz32(uint32_t a)
#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
#[inline]
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: i32 = 23 - ((32 - 1 - clz32(a_mant)) as i32);
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

// BOOL issignan_sf32(uint32_t a)
#[inline]
fn issignan_sf32(a: u32) -> i32 {
    let a_exp1: u32 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
    let a_mant: u32 = a & ((1u32 << 23) - 1);
    (a_exp1 == (2 * ((1u32 << 8) - 1)) && a_mant != 0) as i32
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf128(a: u32, pfflags: *mut u32) -> u128 {
    let mut a_sign: u32 = 0;
    let mut a_exp: i32 = 0;
    let mut a_mant: u128;

    a_mant = unpack_sf32(&mut a_sign, &mut a_exp, a) as u128;
    if a_exp == 0xff {
        if a_mant != 0 {
            /* NaN */
            if issignan_sf32(a) != 0 {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN128;
        } else {
            /* infinity */
            return pack_sf128(a_sign, (1u32 << 15) - 1, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf128(a_sign, 0, 0); /* zero */
        }
        a_mant = normalize_subnormal_sf32(&mut a_exp, a_mant as u32) as u128;
    }
    /* convert the exponent value */
    a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2);
    /* shift the mantissa */
    a_mant <<= 112 - 23;

    pack_sf128(a_sign, a_exp as u32, a_mant)
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_mant = unpack_sf32(&a_sign, &a_exp, a);` · softfp_template.h:944-948 · chunk 3150 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_mant = unpack_sf32(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3150 {
// Port of the first statements of cvt_sf32_sf64: unpack the sf32 fields.
// unpack_sf32 is a `static inline` C helper with no linker symbol, so its
// logic is mirrored here as a private Rust fn.

#[inline]
fn unpack_sf32(pa_sign: *mut u32, pa_exp: *mut i32, a: u32) -> u32 {
    unsafe {
        *pa_sign = a >> (32 - 1);
        *pa_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    }
    a & ((1u32 << 23) - 1)
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf64__part3150(
    a: u32,
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u64,
) {
    let mant = unpack_sf32(pa_sign, pa_exp, a);
    unsafe {
        *pa_mant = mant as u64;
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf32(&a_sign, &a_exp, a);` · softfp_template.h:944-948 · chunk 3274 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf32(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3274 {
// Port of unpack_sf32 (a static inline C helper with no linker symbol).
// Returns the mantissa and writes sign/exponent through the out pointers.
#[inline]
fn unpack_sf32_rs(pa_sign: &mut u32, pa_exp: &mut i32, a: u32) -> u128 {
    *pa_sign = a >> (32 - 1);
    *pa_exp = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    (a & ((1u32 << 23) - 1)) as u128
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf128__part3274(
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    a: u32,
) {
    let mut sign: u32 = 0;
    let mut exp: i32 = 0;
    let mant: u128 = unpack_sf32_rs(&mut sign, &mut exp, a);
    unsafe {
        if !pa_sign.is_null() {
            *pa_sign = sign;
        }
        if !pa_exp.is_null() {
            *pa_exp = exp;
        }
        if !pa_mant.is_null() {
            *pa_mant = mant;
        }
    }
}
}

// ── `if (a_exp == 0xff)` · softfp_template.h:949-960 · chunk 3151 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0xff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 return pack_sf64(a_sign, ((1 << 11) - 1), 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3151 {
// Port of the infinity/NaN handling part of cvt_sf32_sf64 (softfp_template.h 949-960).
// Returns the value the C code would have returned; *p_done tells the C side
// whether a return must be taken.

#[no_mangle]
pub extern "C" fn cvt_sf32_sf64__part3151(
    a: u32,
    a_exp: i32,
    a_mant: u64,
    a_sign: u32,
    pfflags: *mut u32,
    p_done: *mut i32,
) -> u64 {
    if a_exp == 0xff {
        if a_mant != 0 {
            // issignan_sf32(a): exponent field (incl. duplicated lsb) == 2*0xff
            let a_exp1: u32 = (a >> (23 - 1)) & ((1u32 << (8 + 1)) - 1);
            let am: u32 = a & ((1u32 << 23) - 1);
            let is_signan: bool = a_exp1 == 2 * ((1 << 8) - 1) && am != 0;
            if is_signan {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            unsafe {
                *p_done = 1;
            }
            // F_QNAN64 = ((uint64_t)((1 << 11) - 1) << 52) | ((uint64_t)1 << (52 - 1))
            return ((0x7ffu64) << 52) | (1u64 << (52 - 1));
        } else {
            unsafe {
                *p_done = 1;
            }
            // pack_sf64(a_sign, (1 << 11) - 1, 0)
            return (((a_sign as u64) << (64 - 1)) | ((0x7ffu64) << 52) | 0u64) & 0xffff_ffff_ffff_ffffu64;
        }
    }
    unsafe {
        *p_done = 0;
    }
    0u64
}
}

// ── `if (a_exp == 0xff)` · softfp_template.h:949-960 · chunk 3275 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0xff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf32(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 return pack_sf128(a_sign, ((1 << 15) - 1), 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3275 {
// Ported piece of cvt_sf32_sf128: the "exponent is all ones" (NaN/Inf) case.
use core::ffi::c_int;

extern "C" {
    // BOOL issignan_sf32(uint32_t a);
    fn issignan_sf32(a: u32) -> c_int;
}

// static const uint128_t F_QNAN128 =
//   (((uint128_t)((1 << 15) - 1) << 112) | ((uint128_t)1 << (112 - 1)));
const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf128__part3275(
    a: u32,
    a_sign: u32,
    a_mant: u128,
    pfflags: *mut u32,
) -> u128 {
    if a_mant != 0 {
        if unsafe { issignan_sf32(a) } != 0 {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
        }
        F_QNAN128
    } else {
        pack_sf128(a_sign, (1u32 << 15) - 1, 0)
    }
}
}

// ── `if (a_exp == 0)` · softfp_template.h:961-965 · chunk 3152 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3152 {
// Port of the subnormal-handling piece of cvt_sf32_sf64.
// Returns pack_sf64(a_sign, 0, 0) with *pret set to 1 when the caller must
// return immediately (a_mant == 0), otherwise updates *pa_exp / *pa_mant and
// sets *pret to 0.

use core::ffi::c_int;

// static inline helper ported from the C translation unit
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
fn normalize_subnormal_sf32(pa_exp: &mut i32, a_mant: u32) -> u32 {
    let shift: c_int = 23 - (32 - 1 - clz32(a_mant));
    *pa_exp = 1 - shift;
    a_mant.wrapping_shl(shift as u32)
}

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf64__part3152(
    a_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u64,
    pret: *mut c_int,
) -> u64 {
    unsafe {
        let a_exp = *pa_exp;
        let a_mant = *pa_mant;
        if a_exp == 0 {
            if a_mant == 0 {
                *pret = 1;
                return pack_sf64(a_sign, 0, 0);
            }
            let mut new_exp: i32 = a_exp;
            let new_mant = normalize_subnormal_sf32(&mut new_exp, a_mant as u32);
            *pa_exp = new_exp;
            *pa_mant = new_mant as u64;
        }
        *pret = 0;
        0
    }
}
}

// ── `if (a_exp == 0)` · softfp_template.h:961-965 · chunk 3276 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf32(&a_exp, a_mant);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3276 {
// Port of the subnormal-handling piece of cvt_sf32_sf128.
// Mirrors static inline normalize_subnormal_sf32 / clz32 / pack_sf128 (no linker symbols).

#[no_mangle]
pub extern "C" fn cvt_sf32_sf128__part3276(
    a_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    p_ret: *mut u128,
) -> core::ffi::c_int {
    unsafe {
        let a_exp = *pa_exp;
        if a_exp == 0 {
            let a_mant = *pa_mant;
            if a_mant == 0 {
                // pack_sf128(a_sign, 0, 0)
                *p_ret = (a_sign as u128).wrapping_shl(127);
                return 1;
            }
            // normalize_subnormal_sf32(&a_exp, a_mant):
            //   shift = 23 - ((32 - 1 - clz32(a_mant)));
            //   *pa_exp = 1 - shift;
            //   return a_mant << shift;
            let m32: u32 = a_mant as u32;
            let clz: i32 = m32.leading_zeros() as i32;
            let shift: i32 = 23 - (32 - 1 - clz);
            *pa_exp = 1 - shift;
            *pa_mant = m32.wrapping_shl(shift as u32) as u128;
        }
        0
    }
}
}

// ── `a_exp = a_exp - 0x7f + (((1 << 11) - 1) / 2); a_mant <<= (52 - 23); return pack_sf64(a_sign, a_…` · softfp_template.h:967-972 · chunk 3153 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - 0x7f + (((1 << 11) - 1) / 2);
//     
//         a_mant <<= (52 - 23);
//     
//     
//         return pack_sf64(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3153 {
// Port of the tail of cvt_sf32_sf64: exponent bias adjust, mantissa shift,
// and packing of the signed/biased/mantissa parts into an sf64 bit pattern.
// pack_sf64 is a `static inline` C helper with no linker symbol, so its logic
// is mirrored here.

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf64__part3153(a_sign: u32, a_exp: i32, a_mant: u64) -> u64 {
    // a_exp = a_exp - 0x7f + (((1 << 11) - 1) / 2);
    let a_exp: i32 = a_exp - 0x7f + (((1i32 << 11) - 1) / 2);

    // a_mant <<= (52 - 23);
    let a_mant: u64 = a_mant << (52 - 23);

    // return pack_sf64(a_sign, a_exp, a_mant);
    pack_sf64(a_sign, a_exp as u32, a_mant)
}
}

// ── `a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2); a_mant <<= (112 - 23); return pack_sf128(a_sign,…` · softfp_template.h:967-972 · chunk 3277 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2);
//     
//         a_mant <<= (112 - 23);
//     
//     
//         return pack_sf128(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3277 {
// Ported piece of cvt_sf32_sf128: exponent adjust, mantissa align, pack.
use core::ffi::{c_int, c_uint};

#[inline]
fn pack_sf128(a_sign: c_uint, a_exp: c_uint, a_mant: u128) -> u128 {
    let mask: u128 = ((1u128 << 112) - 1) as u128;
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & mask)
}

#[no_mangle]
pub extern "C" fn cvt_sf32_sf128__part3277(a_sign: c_uint, a_exp: c_int, a_mant: u128) -> u128 {
    // a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2);
    let a_exp: i32 = a_exp - 0x7f + ((1i32 << 15) - 1) / 2;
    // a_mant <<= (112 - 23);
    let a_mant: u128 = a_mant << 89u32;
    // return pack_sf128(a_sign, a_exp, a_mant);
    pack_sf128(a_sign, a_exp as u32, a_mant)
}
}

// ── `cvt_sf64_sf32` · softfp_template.h:975-1007 · chunk 3154 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_sf64_sf32(uint64_t a, RoundingModeEnum rm,
//                                                uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_mant = unpack_sf64(&a_sign, &a_exp, a);
//         if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 return pack_sf32(a_sign, 0xff, 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(a_sign, 0, 0);
//             normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//     
//         a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f;
//     
//         a_mant = rshift_rnd64(a_mant, 52 - (32 - 2));
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3154 {
// Rust rewrite of softfp_template.h cvt_sf64_sf32 (double -> single conversion).

use core::ffi::{c_int, c_uint};

// C: void assert_fail(const char *assertion, const char *file,
//                     unsigned int line, const char *function)
extern "C" {
    fn __assert_fail(
        assertion: *const u8,
        file: *const u8,
        line: c_uint,
        function: *const u8,
    ) -> !;
}

const F_QNAN32: u32 = ((1u32 << 8) - 1) << 23 | 1u32 << (23 - 1);
const FFLAG_INVALID_OP: u32 = 1 << 4;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_OVERFLOW: u32 = 1 << 2;

// RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// static inline int clz32(uint32_t a)
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

// static inline int clz64(uint64_t a)
fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let d = d as u32;
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(a_sign: u32, a_exp: c_int, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => addend = 1u32 << (((32 - 2) - 23) - 1),
        RM_RTZ => addend = 0,
        _ => {
            if (a_sign ^ (rm as u32 & 1)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe { *pfflags |= FFLAG_UNDERFLOW };
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe { *pfflags |= FFLAG_INEXACT };
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe { *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT };
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(a_sign: u32, a_exp: c_int, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));
    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr(),
                b"softfp_template.h\0".as_ptr(),
                183,
                b"cvt_sf64_sf32\0".as_ptr(),
            );
        }
    }
    let a_exp = a_exp - shift;
    let a_mant = a_mant << (shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf64_sf32(a: u64, rm: c_int, pfflags: *mut u32) -> u32 {
    // unpack_sf64
    let a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1 << 11) - 1) {
        if a_mant != 0 {
            // issignan_sf64
            let a_exp1: u32 = ((a >> (52 - 1)) & ((1u64 << (11 + 1)) - 1)) as u32;
            let signan_mant: u64 = a & ((1u64 << 52) - 1);
            if a_exp1 == (2 * ((1 << 11) - 1)) && signan_mant != 0 {
                unsafe { *pfflags |= FFLAG_INVALID_OP };
            }
            return F_QNAN32;
        } else {
            return pack_sf32(a_sign, 0xff, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf32(a_sign, 0, 0);
        }
        // normalize_subnormal_sf64(&a_exp, a_mant)
        let shift: i32 = 52 - (64 - 1 - clz64(a_mant));
        a_exp = 1 - shift;
        a_mant = a_mant << (shift as u32);
    } else {
        a_mant |= 1u64 << 52;
    }

    a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f;

    // rshift_rnd64(a_mant, 52 - (32 - 2))
    a_mant = {
        let d: i32 = 52 - (32 - 2);
        if d != 0 {
            if d >= 64 {
                (a_mant != 0) as u64
            } else {
                let d = d as u32;
                let mask: u64 = (1u64 << d) - 1;
                (a_mant >> d) | ((a_mant & mask) != 0) as u64
            }
        } else {
            a_mant
        }
    };

    normalize_sf32(a_sign, a_exp, a_mant as u32, rm, pfflags)
}
}

// ── `cvt_sf128_sf32` · softfp_template.h:975-1007 · chunk 3278 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_sf128_sf32(uint128_t a, RoundingModeEnum rm,
//                                                uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf128(&a_sign, &a_exp, a);
//         if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 return pack_sf32(a_sign, 0xff, 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(a_sign, 0, 0);
//             normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//     
//         a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x7f;
//     
//         a_mant = rshift_rnd128(a_mant, 112 - (32 - 2));
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3278 {
// Port of cvt_sf128_sf32 from softfp_template.h
#![allow(dead_code)]
use core::ffi::c_int;

// static const uint32_t F_QNAN32
const F_QNAN32: u32 = 0x7fc0_0000;

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    a_sign.wrapping_shl(31) | a_exp.wrapping_shl(23) | (a_mant & 0x007f_ffff)
}

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn issignan_sf128(a: u128) -> bool {
    let a_exp1 = ((a >> 111) & 0xffff) as u32;
    let a_mant = a & ((1u128 << 112) - 1);
    a_exp1 == 2 * 0x7fffu32 && a_mant != 0
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d > 0 {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn roundpack_sf32(a_sign: u32, a_exp: i32, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;

    if rm == 0 || rm == 4 {
        // RM_RNE, RM_RMM
        addend = 1u32 << 6;
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else {
        // default, RM_RDN, RM_RUP
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << 7) - 1;
        } else {
            addend = 0;
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend) < 0x8000_0000u32;
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & 0x7f;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & 0x7f;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == 0 && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 24) as i32);
    if a_mant <= 0x007f_ffff {
        a_exp = 0;
    } else if a_exp >= 0xff {
        if addend == 0 {
            a_exp = 0xfe;
            a_mant = 0x007f_ffff;
        } else {
            a_exp = 0xff;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp: i32, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut shift = clz32(a_mant) - 1;
    // C asserts shift >= 0 here; it is always true for the inputs produced by
    // the callers in this translation unit.  Clamp so that a Rust shift
    // overflow can never panic across the FFI boundary.
    if shift < 0 {
        shift = 0;
    }
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf32(a: u128, rm: c_int, pfflags: *mut u32) -> u32 {
    // unpack_sf128
    let a_sign: u32 = (a >> 127) as u32;
    let mut a_exp: i32 = ((a >> 112) & 0x7fff) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == 0x7fff {
        if a_mant != 0 {
            // NaN
            if issignan_sf128(a) {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN32;
        } else {
            // infinity
            return pack_sf32(a_sign, 0xff, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf32(a_sign, 0, 0); // zero
        }
        // normalize_subnormal_sf128: the C code discards the returned
        // mantissa and only keeps the updated exponent.
        let shift = 112 - (127 - clz128(a_mant));
        a_exp = 1 - shift;
    } else {
        a_mant |= 1u128 << 112;
    }

    // convert the exponent value
    a_exp = a_exp.wrapping_sub(16383).wrapping_add(0x7f);

    // shift the mantissa: 112 - (32 - 2) == 82
    a_mant = rshift_rnd128(a_mant, 82);
    normalize_sf32(a_sign, a_exp, a_mant as u32, rm, pfflags)
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_mant = unpack_sf64(&a_sign, &a_exp, a);` · softfp_template.h:978-982 · chunk 3155 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//     
//         a_mant = unpack_sf64(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3155 {
// Port of the unpack_sf64 prologue of cvt_sf64_sf32.
// Reimplements the static inline helper unpack_sf64:
//   *pa_sign = a >> (64 - 1);
//   *pa_exp  = (a >> 52) & ((1 << 11) - 1);
//   return a & (((uint64_t)1 << 52) - 1);

#[no_mangle]
pub extern "C" fn cvt_sf64_sf32__part3155(
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u64,
    a: u64,
) {
    unsafe {
        *pa_sign = (a >> (64 - 1)) as u32;
        *pa_exp = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
        *pa_mant = a & ((1u64 << 52) - 1);
    }
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf128(&a_sign, &a_exp, a);` · softfp_template.h:978-982 · chunk 3279 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf128(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3279 {
// Ported piece of cvt_sf128_sf32: unpack_sf128 (a static inline in the
// original translation unit) plus the declarations it initialises.

#[no_mangle]
pub extern "C" fn cvt_sf128_sf32__part3279(
    pa: *const u128,
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
) {
    unsafe {
        let a: u128 = *pa;
        // *pa_sign = a >> (128 - 1);
        *pa_sign = (a >> 127) as u32;
        // *pa_exp = (a >> 112) & ((1 << 15) - 1);
        *pa_exp = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
        // return a & (((uint128_t)1 << 112) - 1);
        *pa_mant = a & ((1u128 << 112) - 1);
    }
}
}

// ── `if (a_exp == ((1 << 11) - 1)` · softfp_template.h:983-994 · chunk 3156 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 return pack_sf32(a_sign, 0xff, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3156 {
use core::ffi::c_int;

extern "C" {
    fn issignan_sf64(a: u64) -> c_int;
}

const F_QNAN32: u32 = ((((1u32 << 8) - 1) << 23) | (1u32 << (23 - 1)));

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf64_sf32__part3156(
    a_exp: i32,
    a: u64,
    a_sign: u32,
    a_mant: u64,
    pfflags: *mut u32,
    presult: *mut u32,
) -> c_int {
    if a_exp != (1 << 11) - 1 {
        return 0;
    }
    let r: u32 = if a_mant != 0 {
        if unsafe { issignan_sf64(a) } != 0 {
            unsafe {
                *pfflags |= 1 << 4;
            }
        }
        F_QNAN32
    } else {
        pack_sf32(a_sign, 0xff, 0)
    };
    unsafe {
        *presult = r;
    }
    1
}
}

// ── `if (a_exp == ((1 << 15) - 1)` · softfp_template.h:983-994 · chunk 3280 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN32;
//             } else {
//     
//                 return pack_sf32(a_sign, 0xff, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3280 {
// Port of the special-value branch of cvt_sf128_sf32 (softfp_template.h lines 983-994).
// 128-bit values cross the FFI boundary as (lo, hi) u64 halves.
// Return value: bit 32 set => the C caller must return the low 32 bits;
// bit 32 clear => the branch was not taken and the C code continues.

#![allow(non_camel_case_types)]

// static-like helper (non-static in C, but reimplemented here to avoid
// crossing the boundary with a 128-bit argument).
fn issignan_sf128(a: u128) -> bool {
    let a_exp1: u32 = ((a >> (112 - 1)) & ((1u128 << (15 + 1)) - 1)) as u32;
    let a_mant: u128 = a & ((1u128 << 112) - 1);
    a_exp1 == 2 * ((1u32 << 15) - 1) && a_mant != 0
}

const EXP_MAX: core::ffi::c_int = (1 << 15) - 1;
// static const uint32_t F_QNAN32
const F_QNAN32: u32 = (((1u32 << 8) - 1) << 23) | (1u32 << (23 - 1));
const DONE: u64 = 1u64 << 32;

#[no_mangle]
pub extern "C" fn cvt_sf128_sf32__part3280(
    a_sign: u32,
    a_exp: core::ffi::c_int,
    a_mant_lo: u64,
    a_mant_hi: u64,
    a_lo: u64,
    a_hi: u64,
    pfflags: *mut u32,
) -> u64 {
    if a_exp != EXP_MAX {
        return 0;
    }

    let a_mant: u128 = ((a_mant_hi as u128) << 64) | (a_mant_lo as u128);

    if a_mant != 0 {
        let a: u128 = ((a_hi as u128) << 64) | (a_lo as u128);
        if issignan_sf128(a) {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
        }
        DONE | (F_QNAN32 as u64)
    } else {
        // pack_sf32(a_sign, 0xff, 0)
        let packed: u32 = (a_sign << (32 - 1)) | (0xffu32 << 23) | (0u32 & ((1u32 << 23) - 1));
        DONE | (packed as u64)
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:995-1001 · chunk 3157 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(a_sign, 0, 0);
//             normalize_subnormal_sf64(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3157 {
// Port of the subnormal/hidden-bit normalization piece of cvt_sf64_sf32.
// The C code:
//     if (a_exp == 0) {
//         if (a_mant == 0) return pack_sf32(a_sign, 0, 0);
//         normalize_subnormal_sf64(&a_exp, a_mant);
//     } else {
//         a_mant |= (uint64_t)1 << 52;
//     }
// pack_sf32 and normalize_subnormal_sf64 are `static inline` in the C
// translation unit, so their logic is mirrored here as private fns.
// normalize_subnormal_sf64's return value is discarded at this call site,
// so a_mant is left unchanged (only a_exp is updated), exactly as in C.

// static inline int clz64(uint64_t a)
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// returns the packed result; *p_early is set to 1 when the caller must
// immediately return that result (the original early `return`).
#[no_mangle]
pub extern "C" fn cvt_sf64_sf32__part3157(
    a_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u64,
    p_early: *mut u32,
) -> u32 {
    unsafe {
        let a_exp = *pa_exp;
        if a_exp == 0 {
            let a_mant = *pa_mant;
            if a_mant == 0 {
                *p_early = 1;
                return pack_sf32(a_sign, 0, 0);
            }
            // normalize_subnormal_sf64(&a_exp, a_mant), return value unused
            let shift: i32 = 52 - ((64 - 1) - clz64(a_mant));
            *pa_exp = 1 - shift;
        } else {
            *pa_mant |= 1u64 << 52;
        }
        *p_early = 0;
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:995-1001 · chunk 3281 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf32(a_sign, 0, 0);
//             normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3281 {
// Port of the exponent/mantissa normalization branch of cvt_sf128_sf32.
// normalize_subnormal_sf128 and clz128 are `static`/`static inline` in C, so
// their logic is reproduced here as private Rust functions.

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)
// The C caller discards the returned (shifted) mantissa, so only the exponent
// side effect is reproduced.
unsafe fn normalize_subnormal_sf128(pa_exp: *mut i32, a_mant: u128) {
    let shift: i32 = 112 - (128 - 1 - clz128(a_mant));
    *pa_exp = 1 - shift;
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & (((1u32) << 23) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf32__part3281(
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    a_sign: u32,
    p_ret: *mut u32,
) -> core::ffi::c_int {
    unsafe {
        let exp: i32 = *pa_exp;
        let mant: u128 = *pa_mant;
        if exp == 0 {
            if mant == 0 {
                *p_ret = pack_sf32(a_sign, 0, 0);
                return 1;
            }
            normalize_subnormal_sf128(pa_exp, mant);
        } else {
            *pa_mant = mant | ((1u128) << 112);
        }
        0
    }
}
}

// ── `a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f; a_mant = rshift_rnd64(a_mant, 52 - (32 - 2)); ret…` · softfp_template.h:1003-1006 · chunk 3158 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f;
//     
//         a_mant = rshift_rnd64(a_mant, 52 - (32 - 2));
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3158 {
// Port of the tail of cvt_sf64_sf32 (softfp_template.h lines 1003-1006):
//   a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f;
//   a_mant = rshift_rnd64(a_mant, 52 - (32 - 2));
//   return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// The static helpers (rshift_rnd64, normalize_sf32, roundpack_sf32,
// rshift_rnd32, clz32, pack_sf32) are ported here as private Rust fns.

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[inline]
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn rshift_rnd64(mut a: u64, d: c_int) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else if d > 0 {
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf32(a_sign: u32, mut a_exp: c_int, mut a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let addend: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    match rm {
        0 | 4 => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        1 => {
            addend = 0;
        }
        _ => {
            if a_sign ^ ((rm & 1) as u32) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, mut a_exp: c_int, mut a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));

    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                183,
                b"normalize_sf32\0".as_ptr() as *const c_char,
            );
        }
    }
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf64_sf32__part3158(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let a_exp: i32 = a_exp - (((1 << 11) - 1) / 2) + 0x7f;

    let a_mant: u32 = rshift_rnd64(a_mant, 52 - (32 - 2)) as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x7f; a_mant = rshift_rnd128(a_mant, 112 - (32 - 2)); r…` · softfp_template.h:1003-1006 · chunk 3282 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x7f;
//     
//         a_mant = rshift_rnd128(a_mant, 112 - (32 - 2));
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3282 {
// Rust port of the tail of cvt_sf128_sf32 (softfp_template.h lines 1003-1006):
//   a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x7f;
//   a_mant = rshift_rnd128(a_mant, 112 - (32 - 2));
//   return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);

use core::ffi::c_int;

// static inline int clz32(uint32_t a)
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask = (1u32 << (d as u32)).wrapping_sub(1);
            a = (a >> (d as u32)) | ((a & mask) != 0) as u32;
        }
    }
    a
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(mut a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d > 0 {
            let mask = (1u128 << (d as u32)).wrapping_sub(1);
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    ((a_sign) << (32 - 1)) | ((a_exp) << 23) | (a_mant & (((1u32) << 23) - 1))
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_mant = a_mant_in;
    let mut a_exp = a_exp_in;
    let addend: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    match rm {
        0 | 4 => {
            // case RM_RNE: case RM_RMM:
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        1 => {
            // case RM_RTZ:
            addend = 0;
        }
        _ => {
            // default: case RM_RDN: case RM_RUP:
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (23 + 1)) as i32);
    if a_mant <= (((1u32) << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 8) - 1) {
        if addend == 0 {
            a_exp = ((1 << 8) - 1) - 1;
            a_mant = ((1u32) << 23) - 1;
        } else {
            a_exp = (1 << 8) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // the C original asserts shift >= 0 and aborts otherwise
    if shift < 0 {
        std::process::abort();
    }
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf32__part3282(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let a_exp = a_exp
        .wrapping_sub(((1 << 15) - 1) / 2)
        .wrapping_add(0x7f);
    let a_mant = rshift_rnd128(a_mant, 112 - (32 - 2));
    normalize_sf32(a_sign, a_exp, a_mant as u32, rm, pfflags)
}
}

// ── `cvt_sf64_sf128` · softfp_template.h:1013-1043 · chunk 3283 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_sf64_sf128(uint64_t a, uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf64(&a_sign, &a_exp, a);
//     
//         if (a_exp == 0x7ff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 return pack_sf128(a_sign, ((1 << 15) - 1), 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         }
//     
//         a_exp = a_exp - 0x3ff + (((1 << 15) - 1) / 2);
//     
//         a_mant <<= (112 - 52);
//         return pack_sf128(a_sign, a_exp, a_mant);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3283 {
// Port of cvt_sf64_sf128() from softfp_template.h
// All helpers it calls (unpack_sf64, issignan_sf64, normalize_subnormal_sf64,
// pack_sf128) are static/static inline in the C program, so their logic is
// reproduced here as private Rust functions.

const FFLAG_INVALID_OP: u32 = 1 << 4;

const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << (112 - 1));

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn unpack_sf64(pa_sign: &mut u32, pa_exp: &mut i32, a: u64) -> u64 {
    *pa_sign = (a >> (64 - 1)) as u32;
    *pa_exp = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    a & ((1u64 << 52) - 1)
}

#[inline]
fn issignan_sf64(a: u64) -> bool {
    let a_exp1 = (a >> (52 - 1)) & ((1u64 << (11 + 1)) - 1);
    let a_mant = a & ((1u64 << 52) - 1);
    a_exp1 == 2 * ((1u64 << 11) - 1) && a_mant != 0
}

#[inline]
fn normalize_subnormal_sf64(pa_exp: &mut i32, a_mant: u64) -> u64 {
    let shift = 52 - ((64 - 1) - clz64(a_mant));
    *pa_exp = 1 - shift;
    a_mant << (shift as u32)
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf64_sf128(a: u64, pfflags: *mut u32) -> u128 {
    let mut a_sign: u32 = 0;
    let mut a_exp: i32 = 0;

    let mut a_mant: u128 = unpack_sf64(&mut a_sign, &mut a_exp, a) as u128;

    if a_exp == 0x7ff {
        if a_mant != 0 {
            // NaN
            if issignan_sf64(a) {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
            }
            return F_QNAN128;
        } else {
            // infinity
            return pack_sf128(a_sign, (1u32 << 15) - 1, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf128(a_sign, 0, 0); // zero
        }
        let mut exp32 = a_exp;
        a_mant = normalize_subnormal_sf64(&mut exp32, a_mant as u64) as u128;
        a_exp = exp32;
    }
    // convert the exponent value
    a_exp = a_exp - 0x3ff + ((((1i32 << 15) - 1) / 2) as i32);

    // shift the mantissa
    a_mant <<= 112 - 52;
    pack_sf128(a_sign, a_exp as u32, a_mant)
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf64(&a_sign, &a_exp, a);` · softfp_template.h:1015-1019 · chunk 3284 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf64(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3284 {
// Port of unpack_sf64 (static inline in C, so its logic is mirrored here)
// plus the a_mant initialisation of cvt_sf64_sf128.

#[no_mangle]
pub extern "C" fn cvt_sf64_sf128__part3284(
    a: u64,
    pa_sign: *mut u32,
    pa_exp: *mut i32,
) -> u128 {
    let sign: u32 = (a >> (64 - 1)) as u32;
    let exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mant: u128 = (a & ((1u64 << 52) - 1)) as u128;
    unsafe {
        *pa_sign = sign;
        *pa_exp = exp;
    }
    mant
}
}

// ── `if (a_exp == 0x7ff)` · softfp_template.h:1021-1032 · chunk 3285 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0x7ff) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf64(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN128;
//             } else {
//     
//                 return pack_sf128(a_sign, ((1 << 15) - 1), 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3285 {
// Rust port of the "a_exp == 0x7ff" (infinity/NaN) branch of cvt_sf64_sf128.
use core::ffi::c_int;

extern "C" {
    // BOOL issignan_sf64(uint64_t a) -- non-static C function, has a linker symbol.
    fn issignan_sf64(a: u64) -> c_int;
}

// static const uint128_t F_QNAN128 = (((uint128_t)((1 << 15) - 1) << 112)
//                                     | ((uint128_t)1 << (112 - 1)));
const F_QNAN128: u128 = (((1u128 << 15) - 1) << 112) | (1u128 << 111);

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
// (file-static, so its logic is mirrored here)
#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

// Returns nonzero when the taken branch must be returned from, with the value
// written to *presult. Returns 0 when the caller should continue normally.
#[no_mangle]
pub extern "C" fn cvt_sf64_sf128__part3285(
    a_sign: u32,
    a_exp: i32,
    a_mant: *const u128,
    a: u64,
    pfflags: *mut u32,
    presult: *mut u128,
) -> c_int {
    if a_exp == 0x7ff {
        let mant: u128 = unsafe { *a_mant };
        let r: u128;
        if mant != 0 {
            if unsafe { issignan_sf64(a) } != 0 {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            r = F_QNAN128;
        } else {
            r = pack_sf128(a_sign, (1u32 << 15) - 1, 0);
        }
        unsafe {
            *presult = r;
        }
        return 1;
    }
    0
}
}

// ── `if (a_exp == 0)` · softfp_template.h:1033-1037 · chunk 3286 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf128(a_sign, 0, 0);
//             a_mant = normalize_subnormal_sf64(&a_exp, a_mant);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3286 {
// Piece of cvt_sf64_sf128: handling of the zero-exponent (subnormal / zero) case.

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// Ported from the `static inline uint64_t normalize_subnormal_sf64(int32_t *pa_exp, uint64_t a_mant)`
// of the same translation unit.
#[inline]
fn normalize_subnormal_sf64(pa_exp: *mut i32, a_mant: u64) -> u64 {
    let shift: i32 = 52 - ((64 - 1) - clz64(a_mant));
    unsafe {
        *pa_exp = 1 - shift;
    }
    a_mant.wrapping_shl((shift as u32) & 63)
}

// Returns 1 when the caller must `return *pa_mant;` (the zero-mantissa case),
// 0 otherwise (in which case *pa_exp and *pa_mant have been updated in place).
#[no_mangle]
pub extern "C" fn cvt_sf64_sf128__part3286(
    a_sign: u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
) -> core::ffi::c_int {
    unsafe {
        let a_exp: i32 = *pa_exp;
        if a_exp != 0 {
            return 0;
        }
        let a_mant: u128 = *pa_mant;
        if a_mant == 0 {
            // pack_sf128(a_sign, 0, 0)
            *pa_mant = ((a_sign as u128) << (128 - 1)) | (0u128 << 112) | (0u128 & (((1u128) << 112) - 1));
            return 1;
        }
        let m: u64 = normalize_subnormal_sf64(pa_exp, a_mant as u64);
        *pa_mant = m as u128;
        0
    }
}
}

// ── `a_exp = a_exp - 0x3ff + (((1 << 15) - 1) / 2); a_mant <<= (112 - 52); return pack_sf128(a_sign,…` · softfp_template.h:1039-1042 · chunk 3287 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - 0x3ff + (((1 << 15) - 1) / 2);
//     
//         a_mant <<= (112 - 52);
//         return pack_sf128(a_sign, a_exp, a_mant);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3287 {
// Ported TARGET from cvt_sf64_sf128: bias the exponent, shift the mantissa up
// to the 128-bit format and pack sign/exp/mantissa into a uint128_t.
// (pack_sf128 is a static inline in the C translation unit, so its logic is
// reproduced here.)

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & (((1u128) << 112) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf64_sf128__part3287(a_sign: u32, a_exp: i32, a_mant: u128) -> u128 {
    // a_exp = a_exp - 0x3ff + (((1 << 15) - 1) / 2);
    let a_exp = a_exp - 0x3ff + (((1i32 << 15) - 1) / 2);

    // a_mant <<= (112 - 52);
    let a_mant = a_mant << (112 - 52);

    // return pack_sf128(a_sign, a_exp, a_mant);
    pack_sf128(a_sign, a_exp as u32, a_mant)
}
}

// ── `cvt_sf128_sf64` · softfp_template.h:1045-1077 · chunk 3288 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_sf128_sf64(uint128_t a, RoundingModeEnum rm,
//                                                    uint32_t *pfflags)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf128(&a_sign, &a_exp, a);
//         if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 return pack_sf64(a_sign, 0x7ff, 0);
//             }
//         }
//         if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(a_sign, 0, 0);
//             normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//     
//         a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x3ff;
//     
//         a_mant = rshift_rnd128(a_mant, 112 - (64 - 2));
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3288 {
// Port of softfp_template.h: cvt_sf128_sf64
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

const F_QNAN64: u64 = ((1u64 << 11) - 1) << 52 | (1u64 << (52 - 1));

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

#[inline]
fn unpack_sf128(pa_sign: &mut u32, pa_exp: &mut i32, a: u128) -> u128 {
    *pa_sign = (a >> (128 - 1)) as u32;
    *pa_exp = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    a & ((1u128 << 112) - 1)
}

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask = (1u64 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn issignan_sf128(a: u128) -> bool {
    let a_exp1 = ((a >> (112 - 1)) & ((1u128 << (15 + 1)) - 1)) as u32;
    let a_mant = a & ((1u128 << 112) - 1);
    a_exp1 == 2 * ((1u32 << 15) - 1) && a_mant != 0
}

// static inline in C; its return value is discarded at the call site (as in C).
fn normalize_subnormal_sf128(pa_exp: &mut i32, a_mant: u128) {
    let shift = 112 - (128 - 1 - clz128(a_mant));
    *pa_exp = 1 - shift;
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;

    let addend: u32;
    match rm {
        RM_RNE | 4 => {
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = (1i32).wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant_in) - (64 - 1 - (64 - 2));
    // C asserts shift >= 0 here; it always holds for the reachable inputs.
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u64 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf64(a: u128, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_sign: u32 = 0;
    let mut a_exp: i32 = 0;
    let mut a_mant: u128;

    a_mant = unpack_sf128(&mut a_sign, &mut a_exp, a);
    if a_exp == ((1 << 15) - 1) {
        if a_mant != 0 {
            // NaN
            if issignan_sf128(a) {
                unsafe {
                    *pfflags |= 1 << 4;
                }
            }
            return F_QNAN64;
        } else {
            // infinity
            return pack_sf64(a_sign, 0x7ff, 0);
        }
    }
    if a_exp == 0 {
        if a_mant == 0 {
            return pack_sf64(a_sign, 0, 0); // zero
        }
        normalize_subnormal_sf128(&mut a_exp, a_mant);
    } else {
        a_mant |= 1u128 << 112;
    }
    // convert the exponent value
    a_exp = a_exp
        .wrapping_sub(((1 << 15) - 1) / 2)
        .wrapping_add(0x3ff);
    // shift the mantissa
    a_mant = rshift_rnd128(a_mant, 112 - (64 - 2));
    normalize_sf64(a_sign, a_exp, a_mant as u64, rm, pfflags)
}
}

// ── `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf128(&a_sign, &a_exp, a);` · softfp_template.h:1048-1052 · chunk 3289 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//     
//         a_mant = unpack_sf128(&a_sign, &a_exp, a);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3289 {
// Port of the "unpack_sf128" call sequence from cvt_sf128_sf64.
// uint128_t is `unsigned __int128` (mapped to Rust's u128, same layout/alignment).

#[no_mangle]
pub unsafe extern "C" fn cvt_sf128_sf64__part3289(
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
    pa: *const u128,
) {
    let a: u128 = *pa;
    // *pa_sign = a >> (128 - 1);
    *pa_sign = (a >> 127) as u32;
    // *pa_exp = (a >> 112) & ((1 << 15) - 1);
    *pa_exp = ((a >> 112) & 0x7fffu128) as i32;
    // return a & (((uint128_t)1 << 112) - 1);
    *pa_mant = a & (((1u128) << 112) - 1);
}
}

// ── `if (a_exp == ((1 << 15) - 1)` · softfp_template.h:1053-1064 · chunk 3290 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1)) {
//             if (a_mant != 0) {
//     
//                 if (issignan_sf128(a)) {
//                     *pfflags |= (1 << 4);
//                 }
//                 return F_QNAN64;
//             } else {
//     
//                 return pack_sf64(a_sign, 0x7ff, 0);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3290 {
// Port of the special-value (inf/NaN) branch of cvt_sf128_sf64.
use core::ffi::c_int;

extern "C" {
    // BOOL issignan_sf128(uint128_t a) -- external, has a linker symbol.
    fn issignan_sf128(a: u128) -> c_int;
}

// static const uint64_t F_QNAN64 = ((1<<11)-1) << 52 | 1 << 51;
const F_QNAN64: u64 = ((1u64 << 11) - 1) << 52 | 1u64 << (52 - 1);

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

/// Returns the result of the branch; `*done` is set to nonzero when the C
/// caller must return that value from the enclosing function.
#[no_mangle]
pub extern "C" fn cvt_sf128_sf64__part3290(
    a: u128,
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    pfflags: *mut u32,
    done: *mut c_int,
) -> u64 {
    unsafe {
        if a_exp == ((1i32 << 15) - 1) {
            if a_mant != 0 {
                if issignan_sf128(a) != 0 {
                    *pfflags |= 1 << 4;
                }
                *done = 1;
                return F_QNAN64;
            } else {
                *done = 1;
                return pack_sf64(a_sign, 0x7ff, 0);
            }
        }
        *done = 0;
        0
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template.h:1065-1071 · chunk 3291 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             if (a_mant == 0)
//                 return pack_sf64(a_sign, 0, 0);
//             normalize_subnormal_sf128(&a_exp, a_mant);
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3291 {
// Port of the subnormal/implicit-bit handling step of cvt_sf128_sf64.
// Returns nonzero when the caller must do `return *ret;` (zero mantissa
// shortcut), 0 when execution continues with the updated a_exp / a_mant.

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            ah.leading_zeros() as i32
        } else {
            al.leading_zeros() as i32 + 64
        }
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf64__part3291(
    a_sign: u32,
    a_exp: *mut i32,
    a_mant: *mut u128,
    ret: *mut u64,
) -> u64 {
    unsafe {
        let exp = *a_exp;
        let mant = *a_mant;
        if exp == 0 {
            if mant == 0 {
                *ret = pack_sf64(a_sign, 0, 0);
                return 1;
            }
            // normalize_subnormal_sf128(&a_exp, a_mant); the shifted mantissa
            // result is discarded by the caller, so only the exponent matters.
            let shift = 112 - (128 - 1 - clz128(mant));
            *a_exp = 1 - shift;
        } else {
            *a_mant = mant | (1u128 << 112);
        }
        0
    }
}
}

// ── `a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x3ff; a_mant = rshift_rnd128(a_mant, 112 - (64 - 2));…` · softfp_template.h:1073-1076 · chunk 3292 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x3ff;
//     
//         a_mant = rshift_rnd128(a_mant, 112 - (64 - 2));
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3292 {
// Tail of cvt_sf128_sf64: bias the exponent, round the mantissa down to the
// 64-bit working precision and normalize/pack into an sf64.
//
// forward declarations of nothing: rshift_rnd128, normalize_sf64 and the whole
// normalize/roundpack chain are `static` in the C translation unit, so their
// logic is mirrored here as private Rust functions.

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            if a != 0 {
                1
            } else {
                0
            }
        } else {
            let sh = d as u32;
            let mask: u128 = (1u128 << sh) - 1;
            let sticky: u128 = if (a & mask) != 0 { 1 } else { 0 };
            (a >> sh) | sticky
        }
    } else {
        a
    }
}

#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            if a != 0 {
                1
            } else {
                0
            }
        } else {
            let sh = d as u32;
            let mask: u64 = (1u64 << sh) - 1;
            let sticky: u64 = if (a & mask) != 0 { 1 } else { 0 };
            (a >> sh) | sticky
        }
    } else {
        a
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: i32,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = match rm {
        RM_RNE | 4 => 1u32 << ((64 - 2 - 52) - 1),
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << ((64 - 2) - 52)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << ((64 - 2 - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: i32,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant_in) - (64 - 1 - (64 - 2));
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u64 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_sf128_sf64__part3292(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u64 {
    let a_exp: i32 = a_exp
        .wrapping_sub(((1 << 15) - 1) / 2)
        .wrapping_add(0x3ff);

    let a_mant: u64 = rshift_rnd128(a_mant, 112 - (64 - 2)) as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

