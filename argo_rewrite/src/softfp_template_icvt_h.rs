//! From `softfp_template_icvt.h`.

// ── `internal_cvt_sf32_i32` · softfp_template_icvt.h:38-108 · chunk 3294 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int32_t internal_cvt_sf32_i32(uint32_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
//     
//         if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 23)) {
//                 r = (uint32_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3294 {
use core::ffi::{c_int, c_uint};

// Port of the static helper rshift_rnd32 (no linker symbol in the C program).
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask != 0) as u32);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32(
    a: u32,
    rm: c_uint,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i32 {
    let mut a_sign: u32;
    let mut addend: u32;
    let mut rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let r: u32;
    let r_max: u32;

    a_sign = a >> 31;
    a_exp = ((a >> 23) & 0xffu32) as i32;
    a_mant = a & ((1u32 << 23) - 1);
    if a_exp == 0xff && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= 7; // RND_SIZE
    a_exp = a_exp - (0xff / 2) - 23;

    if is_unsigned != 0 {
        r_max = a_sign.wrapping_sub(1);
    } else {
        r_max = (1u32 << 31) - (a_sign ^ 1);
    }

    if a_exp >= 0 {
        if a_exp <= 8 {
            r = (a_mant >> 7).wrapping_shl(a_exp as u32);
            if r <= r_max {
                if a_sign != 0 {
                    return r.wrapping_neg() as i32;
                }
                return r as i32;
            }
        }
        unsafe {
            *pfflags |= 1u32 << 4; // FFLAG_INVALID_OP
        }
        return r_max as i32;
    }

    a_mant = rshift_rnd32(a_mant, -a_exp);

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
            // RM_RDN, RM_RUP and default
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    rnd_bits = a_mant & ((1u32 << 7) - 1);
    a_mant = a_mant.wrapping_add(addend) >> 7;
    // half way: select even result
    if rm == 0 && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }
    if a_mant > r_max {
        unsafe {
            *pfflags |= 1u32 << 4; // FFLAG_INVALID_OP
        }
        return r_max as i32;
    }
    r = a_mant;
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32; // FFLAG_INEXACT
        }
    }
    let r = if a_sign != 0 { r.wrapping_neg() } else { r };
    r as i32
}
}

// ── `internal_cvt_sf32_i64` · softfp_template_icvt.h:38-108 · chunk 3316 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t internal_cvt_sf32_i64(uint32_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
//     
//         if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 23)) {
//                 r = (uint64_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3316 {
// Port of static int64_t internal_cvt_sf32_i64(uint32_t a, RoundingModeEnum rm,
//                                              uint32_t *pfflags, BOOL is_unsigned)
#![allow(non_snake_case)]

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID_OP: u32 = 1 << 4;

// static uint32_t rshift_rnd32(uint32_t a, int d) -- inlined, no linker symbol
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

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64(
    a: u32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i64 {
    let mut a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let mut r: u64 = 0;
    let r_max: u64;

    a_sign = a >> (32 - 1);
    a_exp = ((a >> 23) & ((1i32 << 8) - 1) as u32) as i32;
    a_mant = a & (((1u32) << 23) - 1);
    if a_exp == (1i32 << 8) - 1 && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= (1u32) << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - ((1i32 << 8) - 1) / 2 - 23;

    if is_unsigned != 0 {
        r_max = (a_sign as u64).wrapping_sub(1);
    } else {
        r_max = ((1u64) << (64 - 1)) - ((a_sign ^ 1) as u64);
    }

    let mut do_overflow = false;
    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 23) {
            r = ((a_mant >> ((32 - 2) - 23)) as u64) << (a_exp as u32);
            if r > r_max {
                do_overflow = true;
            }
        } else {
            do_overflow = true;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((32 - 2) - 23) - 1),
            RM_RTZ => 0,
            _ => {
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((32 - 2) - 23)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = (a_mant + addend) >> ((32 - 2) - 23);
        // half way: select even result
        if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if (a_mant as u64) > r_max {
            do_overflow = true;
        } else {
            r = a_mant as u64;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= FFLAG_INEXACT;
                }
            }
        }
    }

    if do_overflow {
        unsafe {
            *pfflags |= FFLAG_INVALID_OP;
        }
        return r_max as i64;
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}
}

// ── `internal_cvt_sf32_i128` · softfp_template_icvt.h:38-108 · chunk 3338 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int128_t internal_cvt_sf32_i128(uint32_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
//         if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
//         a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
//     
//         if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 23)) {
//                 r = (uint128_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3338 {
// Port of softfp_template_icvt.h internal_cvt_sf32_i128 (static function).

use core::ffi::c_int;

#[inline]
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
pub extern "C" fn internal_cvt_sf32_i128(
    a: u32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i128 {
    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let mut a_sign: u32 = a >> 31;
    let mut a_exp: i32 = ((a >> 23) & 0xff) as i32;
    let mut a_mant: u32 = a & 0x007f_ffff;

    if a_exp == 0xff && a_mant != 0 {
        // NaN is like +infinity
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= 7;
    a_exp = a_exp - 127 - 23;

    let r_max: u128 = if is_unsigned != 0 {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << 127) - ((a_sign ^ 1) as u128)
    };

    let r: u128;
    if a_exp >= 0 {
        if a_exp <= 104 {
            let v: u128 = ((a_mant >> 7) as u128) << a_exp;
            if v > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max as i128;
            }
            r = v;
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1u32 << 6,
            1 => 0,
            _ => {
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << 7) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << 7) - 1);
        a_mant = a_mant.wrapping_add(addend) >> 7;

        // half way: select even result
        if rm == 0 && rnd_bits == (1u32 << 6) {
            a_mant &= !1u32;
        }
        if (a_mant as u128) > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i128;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    let r = if a_sign != 0 { r.wrapping_neg() } else { r };
    r as i128
}
}

// ── `internal_cvt_sf64_i32` · softfp_template_icvt.h:38-108 · chunk 3360 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static int32_t internal_cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
//     
//         if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 52)) {
//                 r = (uint32_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3360 {
// Port of internal_cvt_sf64_i32 from softfp_template_icvt.h (ICVT_SIZE=32, F_SIZE=64).
#![allow(clippy::all)]

use core::ffi::c_int;

type BOOL = c_int;
type RoundingModeEnum = c_int;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RDN: RoundingModeEnum = 2;
const RM_RUP: RoundingModeEnum = 3;
const RM_RMM: RoundingModeEnum = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID_OP: u32 = 1 << 4;

// static uint64_t rshift_rnd64(uint64_t a, int d)
#[inline]
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let dd = d as u32;
            let mask: u64 = (1u64 << dd).wrapping_sub(1);
            a = (a >> dd) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub unsafe extern "C" fn internal_cvt_sf64_i32(
    a: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
    is_unsigned: BOOL,
) -> i32 {
    const MANT_SIZE: i32 = 52;
    const EXP_MASK: i32 = (1 << 11) - 1;
    const RND_SIZE: i32 = (64 - 2) - 52; // 10
    const ICVT_SIZE: i32 = 32;

    let mut a_sign: u32;
    let addend: u32;
    let rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u64;
    let mut r: u32 = 0;
    let r_max: u32;

    a_sign = (a >> (64 - 1)) as u32;
    let mant_mask: u64 = (1u64 << MANT_SIZE) - 1;
    a_exp = ((a >> MANT_SIZE) & (EXP_MASK as u64)) as i32;
    a_mant = a & mant_mask;
    if a_exp == EXP_MASK && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << MANT_SIZE;
    }
    a_mant <<= RND_SIZE;
    a_exp = a_exp - (EXP_MASK / 2) - MANT_SIZE;

    if is_unsigned != 0 {
        r_max = a_sign.wrapping_sub(1);
    } else {
        r_max = (1u32 << (ICVT_SIZE - 1)) - (a_sign ^ 1);
    }

    let mut overflow = false;
    if a_exp >= 0 {
        if a_exp <= (ICVT_SIZE - 1 - MANT_SIZE) {
            r = ((a_mant >> RND_SIZE) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        match rm {
            RM_RNE | RM_RMM => {
                addend = 1u32 << (RND_SIZE - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP and anything else
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    addend = (1u32 << RND_SIZE) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & ((1u64 << RND_SIZE) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> RND_SIZE;

        // half way: select even result
        if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max as u64 {
            overflow = true;
        } else {
            r = a_mant as u32;
            if rnd_bits != 0 {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if overflow {
        *pfflags |= FFLAG_INVALID_OP;
        return r_max as i32;
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}
}

// ── `internal_cvt_sf64_i64` · softfp_template_icvt.h:38-108 · chunk 3382 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t internal_cvt_sf64_i64(uint64_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
//     
//         if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 52)) {
//                 r = (uint64_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3382 {
use core::ffi::{c_int, c_uint};

// Mirrors static uint64_t rshift_rnd64(uint64_t a, int d)
#[inline]
fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let sh = d as u32;
            let mask = (1u64 << sh).wrapping_sub(1);
            a = (a >> sh) | ((a & mask) != 0) as u64;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64(
    a: u64,
    rm: c_uint,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i64 {
    const RND_SIZE: u32 = (64 - 2) - 52; // 10

    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1i64 << 11) - 1) as u64) as i32;
    let mut a_mant: u64 = a & (((1u64) << 52) - 1);

    if a_exp == ((1i64 << 11) - 1) as i32 && a_mant != 0 {
        /* NaN is like +infinity */
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= (1u64) << 52;
    }
    a_mant = a_mant.wrapping_shl(RND_SIZE);
    a_exp = a_exp - (((1i64 << 11) - 1) / 2) as i32 - 52;

    let r_max: u64 = if is_unsigned != 0 {
        (a_sign as u64).wrapping_sub(1)
    } else {
        ((1u64) << (64 - 1)) - (a_sign ^ 1) as u64
    };

    let r: u64;
    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 52) as i32 {
            r = (a_mant >> RND_SIZE).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max as i64;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 /* RM_RNE */ | 4 /* RM_RMM */ => 1u32 << (RND_SIZE - 1),
            1 /* RM_RTZ */ => 0,
            _ /* RM_RDN (2), RM_RUP (3), default */ => {
                if (a_sign ^ (rm & 1)) != 0 {
                    (1u32 << RND_SIZE) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << RND_SIZE) - 1)) as u32;
        a_mant = (a_mant + addend as u64) >> RND_SIZE;
        /* half way: select even result */
        if rm == 0 /* RM_RNE */ && rnd_bits == (1u32 << (RND_SIZE - 1)) {
            a_mant &= !(1u64);
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i64;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    let mut r = r;
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}
}

// ── `internal_cvt_sf64_i128` · softfp_template_icvt.h:38-108 · chunk 3404 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int128_t internal_cvt_sf64_i128(uint64_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
//         if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
//         a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
//     
//         if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 52)) {
//                 r = (uint128_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3404 {
// Port of softfp_template_icvt.h internal_cvt_sf64_i128 (ICVT_SIZE=128, F_SIZE=64).

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RMM: i32 = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID_OP: u32 = 1 << 4;

// static uint64_t rshift_rnd64(uint64_t a, int d)
#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            (a != 0) as u64
        } else {
            let sh = d as u32;
            let mask = (1u64 << sh) - 1;
            (a >> sh) | ((a & mask) != 0) as u64
        }
    } else {
        a
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128(
    a: u64,
    rm: i32,
    pfflags: *mut u32,
    is_unsigned: i32,
) -> i128 {
    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - (((1i32 << 11) - 1) / 2) - 52;

    let r_max: u128 = if is_unsigned != 0 {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << (128 - 1)) - ((a_sign ^ 1) as u128)
    };

    let mut r: u128 = 0;
    let mut overflow: bool = false;

    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 52) {
            r = ((a_mant >> ((64 - 2) - 52)) as u128) << (a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((64 - 2) - 52) - 1),
            RM_RTZ => 0,
            _ => {
                // RM_RDN, RM_RUP (and default)
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);
        // half way: select even result
        if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if (a_mant as u128) > r_max {
            overflow = true;
        } else {
            r = a_mant as u128;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= FFLAG_INEXACT;
                }
            }
        }
    }

    if overflow {
        unsafe {
            *pfflags |= FFLAG_INVALID_OP;
        }
        return r_max as i128;
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}
}

// ── `internal_cvt_sf128_i32` · softfp_template_icvt.h:38-108 · chunk 3426 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int32_t internal_cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
//     
//         if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 112)) {
//                 r = (uint32_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3426 {
// Port of:
// static int32_t internal_cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
//                                       uint32_t *pfflags, BOOL is_unsigned)
// sfloat128 (binary128): F_SIZE=128, MANT_SIZE=112, EXP_MASK=(1<<15)-1,
// RND_SIZE=(128-2)-112=14.

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID_OP: u32 = 1 << 4;

const EXP_MASK: i32 = (1 << 15) - 1; // 0x7fff / 32767
const MANT_SIZE: i32 = 112;
const RND_SIZE: u32 = 14;

// Equivalent of the static helper rshift_rnd128() from the C translation unit.
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

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32(
    a: u128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i32 {
    let mut a_sign: u32 = (a >> (128 - 1)) as u32;
    let mut a_exp: i32 = ((a >> MANT_SIZE) & (EXP_MASK as u128)) as i32;
    let mut a_mant: u128 = a & ((1u128 << MANT_SIZE) - 1);

    if a_exp == EXP_MASK && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << MANT_SIZE;
    }
    a_mant <<= RND_SIZE;
    a_exp = a_exp - (EXP_MASK / 2) - MANT_SIZE;

    let r_max: u32;
    if is_unsigned != 0 {
        r_max = a_sign.wrapping_sub(1);
    } else {
        r_max = (1u32 << (32 - 1)).wrapping_sub(a_sign ^ 1);
    }

    let mut r: u32;
    if a_exp >= 0 {
        if a_exp <= (32 - 1 - MANT_SIZE) {
            r = ((a_mant >> RND_SIZE) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
                return r_max as i32;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return r_max as i32;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32;
        match rm {
            0 | 4 => {
                // RM_RNE | RM_RMM
                addend = 1u32 << (RND_SIZE - 1);
            }
            1 => {
                // RM_RTZ
                addend = 0;
            }
            _ => {
                // RM_RDN(2), RM_RUP(3), default
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << RND_SIZE) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        let rnd_bits: u32 = (a_mant & (((1u128 << RND_SIZE) - 1) as u128)) as u32;
        a_mant = (a_mant + (addend as u128)) >> RND_SIZE;
        // half way: select even result
        if rm == 0 && rnd_bits == (1u32 << (RND_SIZE - 1)) {
            a_mant &= !(1u128);
        }
        if a_mant > (r_max as u128) {
            // overflow:
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return r_max as i32;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}
}

// ── `internal_cvt_sf128_i64` · softfp_template_icvt.h:38-108 · chunk 3448 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t internal_cvt_sf128_i64(uint128_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
//     
//         if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 112)) {
//                 r = (uint64_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3448 {
// Port of softfp_template_icvt.h: internal_cvt_sf128_i64 (ICVT_SIZE=64, F_SIZE=128)
// F_SIZE = 128, MANT_SIZE = 112, RND_SIZE = (128 - 2) - 112 = 14
// EXP_MASK = (1 << 15) - 1 = 0x7fff
// FFLAG_INVALID_OP = (1 << 4), FFLAG_INEXACT = (1 << 0)
// RoundingModeEnum: RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4

#![allow(non_snake_case)]

use core::ffi::c_int;

const F_SIZE: u32 = 128;
const MANT_SIZE: u32 = 112;
const RND_SIZE: u32 = (F_SIZE - 2) - MANT_SIZE; // 14
const EXP_MASK: i32 = (1i32 << 15) - 1; // 0x7fff

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const FFLAG_INVALID_OP: u32 = 1 << 4;
const FFLAG_INEXACT: u32 = 1 << 0;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: i32) -> u128 {
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

#[no_mangle]
pub unsafe extern "C" fn internal_cvt_sf128_i64(
    a: u128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i64 {
    let mut a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let mut r: u64 = 0;
    let r_max: u64;
    let mut addend: u32;
    let mut rnd_bits: u32;

    a_sign = (a >> (F_SIZE - 1)) as u32;
    a_exp = ((a >> MANT_SIZE) as i32) & EXP_MASK;
    a_mant = a & ((1u128 << MANT_SIZE) - 1);
    if a_exp == EXP_MASK && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << MANT_SIZE;
    }
    a_mant <<= RND_SIZE;
    a_exp = a_exp - (EXP_MASK / 2) - (MANT_SIZE as i32);

    if is_unsigned != 0 {
        r_max = (a_sign as u64).wrapping_sub(1);
    } else {
        r_max = (1u64 << (64 - 1)) - ((a_sign ^ 1) as u64);
    }

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - (MANT_SIZE as i32)) {
            r = ((a_mant >> RND_SIZE) as u64).wrapping_shl(a_exp as u32);
            if r > r_max {
                // overflow:
                *pfflags |= FFLAG_INVALID_OP;
                return r_max as i64;
            }
        } else {
            // overflow:
            *pfflags |= FFLAG_INVALID_OP;
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        match rm {
            RM_RNE | RM_RMM => {
                addend = 1u32 << (RND_SIZE - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP and anything else
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << RND_SIZE) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & ((1u128 << RND_SIZE) - 1)) as u32;
        a_mant = (a_mant + (addend as u128)) >> RND_SIZE;

        // half way: select even result
        if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > (r_max as u128) {
            *pfflags |= FFLAG_INVALID_OP;
            return r_max as i64;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}
}

// ── `internal_cvt_sf128_i128` · softfp_template_icvt.h:38-108 · chunk 3470 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int128_t internal_cvt_sf128_i128(uint128_t a, RoundingModeEnum rm,
//                                                                              uint32_t *pfflags, BOOL is_unsigned)
//     {
//         uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
//         if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
//         if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
//         a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
//     
//         if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
//         if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 112)) {
//                 r = (uint128_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
//         if (a_sign)
//             r = -r;
//         return r;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3470 {
// Port of static int128_t internal_cvt_sf128_i128(uint128_t a, RoundingModeEnum rm,
//                                                uint32_t *pfflags, BOOL is_unsigned)
// softfp_template_icvt.h, ICVT_SIZE == 128, F_SIZE == 128

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID_OP: u32 = 1 << 4;

// RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4

// static uint128_t rshift_rnd128(uint128_t a, int d)
#[inline]
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else {
            let mask: u128 = (1u128 << d) - 1;
            (a >> d) | ((a & mask) != 0) as u128
        }
    } else {
        a
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128(
    a: u128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i128 {
    let mut a_sign: u32;
    let mut addend: u32;
    let mut rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let mut r: u128 = 0;
    let mut r_max: u128;

    a_sign = (a >> 127) as u32;
    a_exp = ((a >> 112) & 0x7fff) as i32;
    a_mant = a & ((1u128 << 112) - 1);
    if a_exp == 0x7fff && a_mant != 0 {
        a_sign = 0; // NaN is like +infinity
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= 14;
    a_exp = a_exp - 16383 - 112;

    if is_unsigned != 0 {
        r_max = (a_sign as u128).wrapping_sub(1);
    } else {
        r_max = (1u128 << 127).wrapping_sub((a_sign ^ 1) as u128);
    }
    if a_exp >= 0 {
        if a_exp <= 15 {
            let rr: u128 = (a_mant >> 14) << (a_exp as u32);
            if rr > r_max {
                unsafe {
                    *pfflags |= FFLAG_INVALID_OP;
                }
                return r_max as i128;
            }
            r = rr;
        } else {
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        match rm {
            0 | 4 => {
                // RM_RNE, RM_RMM
                addend = 1u32 << 13;
            }
            1 => {
                // RM_RTZ
                addend = 0;
            }
            _ => {
                // default: RM_RDN, RM_RUP
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << 14) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & ((1u128 << 14) - 1)) as u32;
        a_mant = (a_mant + addend as u128) >> 14;
        // half way: select even result
        if rm == 0 && rnd_bits == (1u32 << 13) {
            a_mant &= !1u128;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= FFLAG_INVALID_OP;
            }
            return r_max as i128;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}
}

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint32_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3295 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3295 {
// Computation of the sign, exponent and mantissa fields of the
// softfloat->int32 conversion preamble (part of internal_cvt_sf32_i32).
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3295(
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

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint64_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3317 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3317 {
// Ported piece: initial unpacking of an IEEE-754 binary32 bit pattern.
// a_sign = a >> 31 ; a_exp = (a >> 23) & 0xFF ; a_mant = a & 0x7FFFFF

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3317(
    a: u32,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u32,
) {
    unsafe {
        if !a_sign.is_null() {
            *a_sign = a >> (32 - 1);
        }
        if !a_exp.is_null() {
            *a_exp = (((a >> 23) & ((1u32 << 8) - 1)) as i32);
        }
        if !a_mant.is_null() {
            *a_mant = a & ((1u32 << 23) - 1);
        }
    }
}
}

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint128_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3339 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (32 - 1);
//         a_exp = (a >> 23) & ((1 << 8) - 1);
//         a_mant = a & (((uint32_t)1 << 23) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3339 {
// Ported piece of internal_cvt_sf32_i128: decode sign, exponent and mantissa
// of a binary32 bit pattern into separate locals.
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3339(
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

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint32_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3361 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3361 {
// Extracted piece of internal_cvt_sf64_i32: unpack sign, exponent and mantissa
// fields of a binary64 value.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3361(
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

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint64_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3383 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3383 {
// Extracted piece of internal_cvt_sf64_i64: unpack the sign, exponent and
// mantissa fields of the raw 64-bit soft-float value.

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3383(
    a: u64,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u64,
) {
    let sign: u32 = (a >> (64 - 1)) as u32;
    let exp: i32 = (((a >> 52) & ((1u64 << 11) - 1)) as u32) as i32;
    let mant: u64 = a & ((1u64 << 52) - 1);

    unsafe {
        *a_sign = sign;
        *a_exp = exp;
        *a_mant = mant;
    }
}
}

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint128_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3405 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (64 - 1);
//         a_exp = (a >> 52) & ((1 << 11) - 1);
//         a_mant = a & (((uint64_t)1 << 52) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3405 {
// Extracted from internal_cvt_sf64_i128: declaration initialisation of the
// sign, exponent and mantissa fields of a binary64 value.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3405(
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

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint32_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3427 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint32_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3427 {
// Port of the exponent/sign/mantissa unpacking prefix of internal_cvt_sf128_i32.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3427(
    a: u128,
    pa_sign: *mut u32,
    pa_exp: *mut i32,
    pa_mant: *mut u128,
) {
    // a_sign = a >> (128 - 1);
    let a_sign: u32 = (a >> 127) as u32;
    // a_exp = (a >> 112) & ((1 << 15) - 1);
    let a_exp: i32 = ((a >> 112) & (((1u32 << 15) - 1) as u128)) as i32;
    // a_mant = a & (((uint128_t)1 << 112) - 1);
    let a_mant: u128 = a & ((1u128 << 112) - 1);

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

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint64_t r, r_max; a_sign =…` · softfp_template_icvt.h:41-48 · chunk 3449 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint64_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3449 {
// Port of the unpack/sign-extraction prologue of internal_cvt_sf128_i64.
// Computes a_sign, a_exp (biased) and a_mant (raw 112-bit fraction) from the
// packed 128-bit softfloat value `a`.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3449(
    a: u128,
    p_a_sign: *mut u32,
    p_a_exp: *mut i32,
    p_a_mant: *mut u128,
) {
    unsafe {
        let a_sign: u32 = (a >> (128 - 1)) as u32;
        let a_exp: i32 = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
        let a_mant: u128 = a & ((1u128 << 112) - 1);

        if !p_a_sign.is_null() {
            *p_a_sign = a_sign;
        }
        if !p_a_exp.is_null() {
            *p_a_exp = a_exp;
        }
        if !p_a_mant.is_null() {
            *p_a_mant = a_mant;
        }
    }
}
}

// ── `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint128_t r, r_max; a_sign…` · softfp_template_icvt.h:41-48 · chunk 3471 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t a_sign, addend, rnd_bits;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint128_t r, r_max;
//     
//         a_sign = a >> (128 - 1);
//         a_exp = (a >> 112) & ((1 << 15) - 1);
//         a_mant = a & (((uint128_t)1 << 112) - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3471 {
// Port of the header/unpacking part of internal_cvt_sf128_i128.
// Decodes the sign bit, biased exponent and mantissa field of a binary128
// value (passed by value) into the caller's locals through pointers.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3471(
    a: u128,
    a_sign: *mut u32,
    a_exp: *mut i32,
    a_mant: *mut u128,
) {
    unsafe {
        // a_sign = a >> (128 - 1);
        *a_sign = (a >> (128 - 1)) as u32;
        // a_exp = (a >> 112) & ((1 << 15) - 1);
        *a_exp = ((a >> 112) & (((1u128 << 15) - 1) as u128)) as i32;
        // a_mant = a & (((uint128_t)1 << 112) - 1);
        *a_mant = a & ((1u128 << 112) - 1);
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3296 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3296 {
// Port of the NaN-sign normalization step of internal_cvt_sf32_i32.
// Reads a_exp (int32_t) and a_mant (uint32_t); returns the possibly updated a_sign.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3296(a_exp: i32, a_mant: u32, a_sign: u32) -> u32 {
    if a_exp == 255 && a_mant != 0 {
        0u32
    } else {
        a_sign
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3318 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3318 {
// Port of the NaN-sign-clearing guard from internal_cvt_sf32_i64.
// Computes the (possibly updated) a_sign value.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3318(
    a_exp: core::ffi::c_int,
    a_mant: core::ffi::c_uint,
    a_sign: core::ffi::c_uint,
) -> core::ffi::c_uint {
    // if (a_exp == 255 && a_mant != 0) a_sign = 0;
    if a_exp == 255 && a_mant != 0 {
        0
    } else {
        a_sign
    }
}
}

// ── `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3340 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 8) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3340 {
// Port of the NaN-detection adjustment from internal_cvt_sf32_i128.
// If the exponent is all ones and the mantissa is nonzero (NaN), clear the sign.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3340(
    a_exp: core::ffi::c_int,
    a_mant: u32,
    a_sign: *mut u32,
) {
    if a_exp == ((1i32 << 8) - 1) && a_mant != 0 {
        unsafe {
            *a_sign = 0;
        }
    }
}
}

// ── `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3362 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3362 {
// Port of the sign fix-up for NaN inputs in internal_cvt_sf64_i32.
// Returns the (possibly cleared) sign bit.

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3362(
    a_exp: i32,
    a_mant: u64,
    a_sign: u32,
) -> u32 {
    if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
        0
    } else {
        a_sign
    }
}
}

// ── `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3384 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3384 {
// Piece of internal_cvt_sf64_i64: NaN detection clears the sign flag.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3384(a_exp: i32, a_mant: u64, a_sign: u32) -> u32 {
    if a_exp == 2047 && a_mant != 0 {
        0
    } else {
        a_sign
    }
}
}

// ── `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3406 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 11) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3406 {
// Port of the NaN-sign fixup inside internal_cvt_sf64_i128.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3406(a_exp: i32, a_mant: u64, a_sign: *mut u32) {
    unsafe {
        if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
            *a_sign = 0;
        }
    }
}
}

// ── `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3428 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3428 {
// Port of the NaN-detection normalization step of internal_cvt_sf128_i32.
// C original:
//     if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//         a_sign = 0;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3428(
    a_exp: c_int,
    a_mant: u128,
    a_sign: *mut u32,
) {
    let exp_max: c_int = ((1u32 << 15) - 1) as c_int;
    if a_exp == exp_max && a_mant != 0 {
        unsafe {
            *a_sign = 0;
        }
    }
}
}

// ── `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3450 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3450 {
// Handles the NaN detection branch of internal_cvt_sf128_i64:
// a_sign is cleared when the exponent is all ones and the mantissa is nonzero.
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3450(a_exp: i32, a_mant: u128, a_sign: *mut u32) {
    if a_exp == 0x7fff && a_mant != 0 {
        unsafe {
            *a_sign = 0;
        }
    }
}
}

// ── `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` · softfp_template_icvt.h:49-50 · chunk 3472 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//             a_sign = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3472 {
// TARGET: the "signan" fixup of internal_cvt_sf128_i128:
//   if (a_exp == ((1 << 15) - 1) && a_mant != 0)
//       a_sign = 0;
// Reads a_exp (int32_t), a_mant (uint128_t) and a_sign (uint32_t);
// returns the (possibly updated) a_sign.

use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3472(
    a_exp: c_int,
    a_mant: u128,
    a_sign: c_uint,
) -> c_uint {
    // (1 << 15) - 1 == 0x7fff
    if a_exp == 0x7fff && a_mant != 0 {
        0
    } else {
        a_sign
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3297 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3297 {
// Port of the implicit-bit / denormal-exponent fixup inside internal_cvt_sf32_i32.
// Reads and writes the caller's a_exp (int32_t) and a_mant (uint32_t).

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3297(a_exp: *mut i32, a_mant: *mut u32) {
    unsafe {
        if !a_exp.is_null() && !a_mant.is_null() {
            if *a_exp == 0 {
                *a_exp = 1;
            } else {
                *a_mant |= 1u32 << 23;
            }
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3319 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3319 {
// Port of the exponent/mantissa normalization step of internal_cvt_sf32_i64.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3319(a_exp: *mut i32, a_mant: *mut u32) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u32 << 23;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3341 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint32_t)1 << 23;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3341 {
// Ported piece of internal_cvt_sf32_i128: handle the implicit/explicit
// leading-bit fixup of the mantissa and the subnormal exponent bump.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3341(a_exp: *mut i32, a_mant: *mut u32) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u32 << 23;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3363 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3363 {
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3363(a_exp: *mut i32, a_mant: *mut u64) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u64 << 52;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3385 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3385 {
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3385(a_exp: *mut i32, a_mant: *mut u64) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u64 << 52;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3407 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint64_t)1 << 52;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3407 {
// Portion of internal_cvt_sf64_i128 (softfp_template_icvt.h lines 51-55):
// subnormal/hidden-bit fixup done right after exponent extraction.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3407(a_exp: *mut i32, a_mant: *mut u64) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u64 << 52;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3429 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3429 {
// Ported piece of internal_cvt_sf128_i32:
//   if (a_exp == 0) { a_exp = 1; } else { a_mant |= (uint128_t)1 << 112; }
// uint128_t is `unsigned __int128`, mirrored as Rust u128.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3429(a_exp: *mut i32, a_mant: *mut u128) {
    unsafe {
        let exp: i32 = *a_exp;
        if exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u128 << 112;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3451 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3451 {
// Port of softfp_template_icvt.h lines 51-55:
//   if (a_exp == 0) { a_exp = 1; } else { a_mant |= (uint128_t)1 << 112; }
// a_exp (int32_t) and a_mant (uint128_t) are assigned, so both are passed by
// pointer.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3451(pa_exp: *mut i32, pa_mant: *mut u128) {
    unsafe {
        if *pa_exp == 0 {
            *pa_exp = 1;
        } else {
            *pa_mant |= 1u128 << 112;
        }
    }
}
}

// ── `if (a_exp == 0) … else` · softfp_template_icvt.h:51-55 · chunk 3473 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp == 0) {
//             a_exp = 1;
//         } else {
//             a_mant |= (uint128_t)1 << 112;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3473 {
// Port of the implicit-bit / subnormal-exponent fixup of internal_cvt_sf128_i128.
// Reads *a_exp, may write *a_exp (subnormal case) or *a_mant (normal case).
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3473(a_exp: *mut i32, a_mant: *mut u128) {
    unsafe {
        if *a_exp == 0 {
            *a_exp = 1;
        } else {
            *a_mant |= 1u128 << 112;
        }
    }
}
}

// ── `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` · softfp_template_icvt.h:56-57 · chunk 3298 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3298 {
// Extracted from internal_cvt_sf32_i32 (softfp_template_icvt.h lines 56-57).
// a_mant <<= ((32 - 2) - 23);
// a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
// Returns the new a_mant and stores the new a_exp through the pointer.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3298(a_mant: u32, a_exp: *mut i32) -> u32 {
    let mant = a_mant.wrapping_shl(((32u32 - 2) - 23) & 31);
    let exp = unsafe { *a_exp };
    let new_exp = exp.wrapping_sub(((1i32 << 8) - 1) / 2).wrapping_sub(23);
    unsafe {
        *a_exp = new_exp;
    }
    mant
}
}

// ── `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` · softfp_template_icvt.h:56-57 · chunk 3320 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3320 {
// Port of the a_mant shift and a_exp adjustment of internal_cvt_sf32_i64.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3320(a_mant: *mut u32, a_exp: *mut i32) {
    unsafe {
        let m: u32 = *a_mant;
        *a_mant = m << ((32 - 2) - 23);
        let e: i32 = *a_exp;
        *a_exp = e - (((1 << 8) - 1) / 2) - 23;
    }
}
}

// ── `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` · softfp_template_icvt.h:56-57 · chunk 3342 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((32 - 2) - 23);
//         a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3342 {
// Ported piece of internal_cvt_sf32_i128: mantissa/exponent adjustment.
// a_mant is a uint32_t (shifted left by 7), a_exp is an int32_t
// (subtracted by ((1<<8)-1)/2 + 23 == 150).

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3342(pa_mant: *mut u32, pa_exp: *mut i32) {
    unsafe {
        if !pa_mant.is_null() {
            let mant = core::ptr::read(pa_mant);
            core::ptr::write(pa_mant, mant.wrapping_shl((32u32 - 2 - 23) & 31));
        }
        if !pa_exp.is_null() {
            let exp = core::ptr::read(pa_exp);
            let bias: i32 = (((1i32 << 8) - 1) / 2) + 23;
            core::ptr::write(pa_exp, exp.wrapping_sub(bias));
        }
    }
}
}

// ── `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` · softfp_template_icvt.h:56-57 · chunk 3364 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3364 {
// Ported piece: mantissa normalization shift and exponent rebias of
// internal_cvt_sf64_i32 (softfp_template_icvt.h lines 56-57).
// a_mant: uint64_t*, a_exp: int32_t*

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3364(a_mant: *mut u64, a_exp: *mut i32) {
    unsafe {
        if !a_mant.is_null() {
            // a_mant <<= ((64 - 2) - 52);
            *a_mant = (*a_mant).wrapping_shl(((64 - 2) - 52) as u32);
        }
        if !a_exp.is_null() {
            // a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
            *a_exp = (*a_exp).wrapping_sub(((1 << 11) - 1) / 2).wrapping_sub(52);
        }
    }
}
}

// ── `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` · softfp_template_icvt.h:56-57 · chunk 3386 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3386 {
// Ported piece of internal_cvt_sf64_i64: mantissa normalization shift and
// exponent rebias.

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3386(a_mant: *mut u64, a_exp: *mut i32) {
    unsafe {
        if !a_mant.is_null() {
            let m = *a_mant;
            *a_mant = m << 10; // ((64 - 2) - 52)
        }
        if !a_exp.is_null() {
            let e = *a_exp;
            *a_exp = e.wrapping_sub(1023).wrapping_sub(52); // (((1<<11)-1)/2) == 1023
        }
    }
}
}

// ── `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` · softfp_template_icvt.h:56-57 · chunk 3408 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((64 - 2) - 52);
//         a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3408 {
// Port of internal_cvt_sf64_i128 lines 56-57:
//   a_mant <<= ((64 - 2) - 52);
//   a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3408(a_mant: *mut u64, a_exp: *mut i32) {
    unsafe {
        // a_mant <<= ((64 - 2) - 52)  ->  shift left by 10 bits
        *a_mant = (*a_mant) << 10u32;
        // a_exp = a_exp - (((1 << 11) - 1) / 2) - 52  ->  a_exp - 1023 - 52
        *a_exp = (*a_exp).wrapping_sub(((1i32 << 11) - 1) / 2).wrapping_sub(52);
    }
}
}

// ── `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` · softfp_template_icvt.h:56-57 · chunk 3430 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3430 {
// Port of the mantissa shift / exponent bias step of internal_cvt_sf128_i32.
// uint128_t is `unsigned __int128` == Rust u128, int32_t == i32.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3430(a_mant: *mut u128, a_exp: *mut i32) {
    unsafe {
        // a_mant <<= ((128 - 2) - 112);
        *a_mant = *a_mant << 14;
        // a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
        *a_exp = (*a_exp).wrapping_sub(16383).wrapping_sub(112);
    }
}
}

// ── `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` · softfp_template_icvt.h:56-57 · chunk 3452 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3452 {
// Portion of internal_cvt_sf128_i64: normalize the mantissa and rebias the
// exponent.
//
// The C code being replaced:
//     a_mant <<= ((128 - 2) - 112);          // shift left by 14
//     a_exp  = a_exp - (((1 << 15) - 1) / 2) - 112;   // subtract 16495
//
// a_mant (uint128_t) and a_exp (int32_t) are read and written, so they are
// passed by pointer.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3452(a_mant: *mut u128, a_exp: *mut i32) {
    unsafe {
        if !a_mant.is_null() {
            // ((128 - 2) - 112) == 14, unsigned wrap-around shift semantics.
            *a_mant = (*a_mant) << 14;
        }
        if !a_exp.is_null() {
            // (((1 << 15) - 1) / 2) + 112 == 16383 + 112 == 16495
            *a_exp = (*a_exp).wrapping_sub(16495);
        }
    }
}
}

// ── `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` · softfp_template_icvt.h:56-57 · chunk 3474 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant <<= ((128 - 2) - 112);
//         a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3474 {
// Ported piece of internal_cvt_sf128_i128:
//   a_mant <<= ((128 - 2) - 112);
//   a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;
// a_mant is uint128_t (unsigned __int128), a_exp is int32_t.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3474(a_mant: *mut u128, a_exp: *mut i32) {
    unsafe {
        let mant: u128 = *a_mant;
        *a_mant = mant << 14u32;

        let exp: i32 = *a_exp;
        // (((1 << 15) - 1) / 2) == 16383, plus 112
        *a_exp = exp.wrapping_sub(16383i32).wrapping_sub(112i32);
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3299 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3299 {
// Compute the saturation bound r_max used by internal_cvt_sf32_i32.
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3299(
    a_sign: u32,
    is_unsigned: core::ffi::c_int,
) -> u32 {
    if is_unsigned != 0 {
        // (uint32_t)a_sign - 1, wrapping unsigned arithmetic
        a_sign.wrapping_sub(1)
    } else {
        // ((uint32_t)1 << 31) - (uint32_t)(a_sign ^ 1)
        (1u32 << 31).wrapping_sub(a_sign ^ 1)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3321 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3321 {
// Computes the saturation bound r_max for the sf32 -> i64/u64 conversion.
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3321(
    a_sign: u32,
    is_unsigned: core::ffi::c_int,
) -> u64 {
    if is_unsigned != 0 {
        // (uint64_t)a_sign - 1  (wraps to UINT64_MAX when a_sign == 0)
        (a_sign as u64).wrapping_sub(1)
    } else {
        // ((uint64_t)1 << 63) - (uint64_t)(a_sign ^ 1)
        (1u64 << 63) - ((a_sign ^ 1) as u64)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3343 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3343 {
// Computes the saturation limit r_max for the float32 -> int128/uint128 conversion.
// uint128_t in C is `unsigned __int128`, mapped to Rust u128.
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3343(
    is_unsigned: core::ffi::c_int,
    a_sign: u32,
) -> u128 {
    if is_unsigned != 0 {
        // (uint128_t)a_sign - 1, unsigned modular arithmetic
        (a_sign as u128).wrapping_sub(1)
    } else {
        // ((uint128_t)1 << 127) - (uint128_t)(a_sign ^ 1)
        (1u128 << 127) - ((a_sign ^ 1) as u128)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3365 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3365 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3365(a_sign: u32, is_unsigned: c_int) -> u32 {
    if is_unsigned != 0 {
        a_sign.wrapping_sub(1)
    } else {
        0x8000_0000u32.wrapping_sub(a_sign ^ 1)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3387 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3387 {
use core::ffi::c_int;

// Computes the maximum representable result magnitude for the
// sf64 -> i64/u64 conversion.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3387(is_unsigned: c_int, a_sign: u32) -> u64 {
    if is_unsigned != 0 {
        // (uint64_t)a_sign - 1  (unsigned wrapping)
        (a_sign as u64).wrapping_sub(1)
    } else {
        // ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1)
        (1u64 << 63) - ((a_sign ^ 1) as u64)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3409 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3409 {
// Computes the maximum representable magnitude (r_max) for the conversion.
// Mirrors the C:
//   if (is_unsigned) r_max = (uint128_t)a_sign - 1;
//   else r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3409(
    a_sign: u32,
    is_unsigned: core::ffi::c_int,
) -> u128 {
    if is_unsigned != 0 {
        // (uint128_t)a_sign - 1 : wraps when a_sign == 0 (unsigned __int128 arithmetic)
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << 127) - ((a_sign ^ 1u32) as u128)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3431 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint32_t)a_sign - 1;
//         else
//             r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3431 {
// Computes the saturation bound r_max for internal_cvt_sf128_i32.
// a_sign is 0 or 1; is_unsigned is a C BOOL (int, truthy when nonzero).
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3431(a_sign: u32, is_unsigned: i32) -> u32 {
    if is_unsigned != 0 {
        // (uint32_t)a_sign - 1  (unsigned wraparound when a_sign == 0)
        a_sign.wrapping_sub(1)
    } else {
        // ((uint32_t)1 << 31) - (uint32_t)(a_sign ^ 1)
        0x8000_0000u32.wrapping_sub(a_sign ^ 1)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3453 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint64_t)a_sign - 1;
//         else
//             r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3453 {
// Port of the r_max computation inside internal_cvt_sf128_i64
// (softfp_template_icvt.h lines 59-62).
//
// C original:
//     if (is_unsigned)
//         r_max = (uint64_t)a_sign - 1;
//     else
//         r_max = ((uint64_t)1 << (64 - 1)) - (uint64_t)(a_sign ^ 1);
//
// Both subtractions are unsigned 64-bit arithmetic, so wrapping is well
// defined and matches the C semantics exactly.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3453(
    a_sign: u32,
    is_unsigned: core::ffi::c_int,
) -> u64 {
    if is_unsigned != 0 {
        (a_sign as u64).wrapping_sub(1)
    } else {
        (1u64 << 63).wrapping_sub((a_sign ^ 1) as u64)
    }
}
}

// ── `if (is_unsigned) … else` · softfp_template_icvt.h:59-62 · chunk 3475 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_unsigned)
//             r_max = (uint128_t)a_sign - 1;
//         else
//             r_max = ((uint128_t)1 << (128 - 1)) - (uint128_t)(a_sign ^ 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3475 {
// Computes the saturation limit r_max for the float128 -> int128 conversion.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3475(a_sign: u32, is_unsigned: c_int) -> u128 {
    if is_unsigned != 0 {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << 127u32).wrapping_sub((a_sign ^ 1) as u128)
    }
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3300 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 23)) {
//                 r = (uint32_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3300 {
// Port of the rounding/overflow part of internal_cvt_sf32_i32 (softfp_template_icvt.h).
// Returns 0 on success (with the result stored through `pr`), or nonzero when the
// "overflow" path was taken (caller must then set the flag-free `return r_max`).
// The flags themselves are updated here, exactly like the C code did.

const RM_RNE: core::ffi::c_int = 0;
const RM_RTZ: core::ffi::c_int = 1;
const RM_RDN: core::ffi::c_int = 2;
const RM_RUP: core::ffi::c_int = 3;
const RM_RMM: core::ffi::c_int = 4;

// Ported from the static C helper rshift_rnd32 (no linker symbol available).
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask: u32 = (1u32 << du).wrapping_sub(1);
            a = (a >> du) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3300(
    a_sign: u32,
    a_mant: u32,
    a_exp: i32,
    pfflags: *mut u32,
    rm: core::ffi::c_int,
    r_max: u32,
    pr: *mut u32,
) -> core::ffi::c_int {
    let mut a_mant = a_mant;
    let mut r: u32 = 0;

    if a_exp >= 0 {
        if a_exp <= 8 {
            r = (a_mant >> 7).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return 1;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return 1;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << 7,
            RM_RTZ => 0,
            _ => {
                // default: case RM_RDN: case RM_RUP:
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << 7) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << 7) - 1);
        a_mant = a_mant.wrapping_add(addend) >> 7;

        if rm == RM_RNE && rnd_bits == (1u32 << 7) {
            a_mant &= !1u32;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return 1;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    unsafe {
        *pr = r;
    }
    0
}

// keep the constants referenced so unused warnings never matter for the build
#[allow(dead_code)]
const _RM_ALL: [core::ffi::c_int; 5] = [RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM];
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3322 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 23)) {
//                 r = (uint64_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3322 {
// Port of the integer/rounding core of internal_cvt_sf32_i64 (softfp_template_icvt.h).
// The C caller keeps the sign negation and the final return.

use core::ffi::c_int;

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let sh = d as u32;
            let mask: u32 = (1u32 << sh) - 1;
            a = (a >> sh) | (((a & mask) != 0) as u32);
        }
    }
    a
}

// Returns 0 when the caller must continue with *pr (then apply a_sign),
// returns nonzero on overflow: *pr already holds r_max (return it directly).
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3322(
    a_exp: i32,
    a_mant_in: u32,
    a_sign: u32,
    rm: c_int,
    r_max: u64,
    pfflags: *mut u32,
    pr: *mut u64,
) -> c_int {
    let mut a_mant = a_mant_in;
    let mut r: u64 = 0;

    unsafe {
        if a_exp >= 0 {
            if a_exp <= 40 {
                r = ((a_mant >> 7) as u64) << (a_exp as u32);
                if r > r_max {
                    // overflow:
                    *pfflags |= 1 << 4;
                    *pr = r_max;
                    return 1;
                }
            } else {
                // overflow:
                *pfflags |= 1 << 4;
                *pr = r_max;
                return 1;
            }
        } else {
            a_mant = rshift_rnd32(a_mant, -a_exp);

            let addend: u32 = match rm {
                0 | 4 => 1u32 << 6, // RM_RNE, RM_RMM
                1 => 0u32,          // RM_RTZ
                _ => {
                    // default: RM_RDN, RM_RUP
                    if (a_sign ^ ((rm as u32) & 1)) != 0 {
                        (1u32 << 7) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = a_mant & ((1u32 << 7) - 1);
            a_mant = a_mant.wrapping_add(addend) >> 7;

            if rm == 0 && rnd_bits == (1u32 << 6) {
                a_mant &= !1u32;
            }
            if (a_mant as u64) > r_max {
                // overflow:
                *pfflags |= 1 << 4;
                *pr = r_max;
                return 1;
            }
            r = a_mant as u64;
            if rnd_bits != 0 {
                *pfflags |= 1 << 0;
            }
        }

        *pr = r;
    }

    0
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3344 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 23)) {
//                 r = (uint128_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd32(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((32 - 2) - 23) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((32 - 2) - 23)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((32 - 2) - 23) ) - 1);
//             a_mant = (a_mant + addend) >> ((32 - 2) - 23);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3344 {
// Port of the integer-conversion core (scaling + rounding) of
// internal_cvt_sf32_i128.  Returns 1 when the C code must take the
// "overflow" path (i.e. `return r_max` after setting the inexact/overflow
// flag), 0 when it falls through to the sign fix-up.

use core::ffi::c_int;

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3344(
    a_exp: c_int,
    rm: c_int,
    a_sign: u32,
    a_mant: *mut u32,
    r_max: *const u128,
    r: *mut u128,
    pfflags: *mut u32,
) -> c_int {
    unsafe {
        let r_max_v: u128 = *r_max;
        if a_exp >= 0 {
            if a_exp <= (128 - 1 - 23) {
                let v: u128 = ((*a_mant >> ((32 - 2) - 23)) as u128) << a_exp;
                *r = v;
                if v > r_max_v {
                    // goto overflow
                    *pfflags |= 1 << 4;
                    return 1;
                }
            } else {
                // overflow:
                *pfflags |= 1 << 4;
                return 1;
            }
        } else {
            let mut am: u32 = rshift_rnd32(*a_mant, -a_exp);

            let addend: u32 = match rm {
                0 | 4 => 1u32 << (((32 - 2) - 23) - 1), // RM_RNE, RM_RMM
                1 => 0,                                 // RM_RTZ
                _ => {
                    // RM_RDN, RM_RUP and default
                    if (a_sign ^ ((rm as u32) & 1)) != 0 {
                        (1u32 << ((32 - 2) - 23)) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = am & ((1u32 << ((32 - 2) - 23)) - 1);
            am = am.wrapping_add(addend) >> ((32 - 2) - 23);

            if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
                am &= !1u32;
            }
            *a_mant = am;
            if (am as u128) > r_max_v {
                // goto overflow
                *pfflags |= 1 << 4;
                return 1;
            }
            *r = am as u128;
            if rnd_bits != 0 {
                *pfflags |= 1 << 0;
            }
        }
        0
    }
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3366 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 52)) {
//                 r = (uint32_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3366 {
// Port of the rounding/overflow block of internal_cvt_sf64_i32.
// Returns 1 when the C code would have taken the "overflow" path
// (i.e. return r_max after setting the invalid flag), 0 otherwise.
// On 0, *r_out and *a_mant_out receive the updated values.

use core::ffi::{c_int, c_uint};

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

// (64 - 2) - 52
const MANT_SHIFT: u32 = 64 - 2 - 52;

// Ported from the static C helper rshift_rnd64 (no linker symbol).
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = (1u64 << d) - 1;
            let sticky: u64 = (a & mask != 0) as u64;
            a = (a >> d) | sticky;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3366(
    a_exp: c_int,
    mut a_mant: u64,
    rm: c_uint,
    a_sign: u32,
    r_max: u32,
    pfflags: *mut u32,
    r_out: *mut u32,
    a_mant_out: *mut u64,
) -> c_int {
    let rm_i: c_int = rm as c_int;
    let mut r: u32 = 0;

    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 52) {
            r = ((a_mant >> MANT_SHIFT) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return 1;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return 1;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm_i {
            RM_RNE => 1u32 << (MANT_SHIFT - 1),
            RM_RTZ => 0,
            _ => {
                // default / RM_RDN / RM_RUP
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << MANT_SHIFT) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << MANT_SHIFT) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> MANT_SHIFT;

        if rm_i == RM_RNE && rnd_bits as u64 == (1u64 << (MANT_SHIFT - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max as u64 {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return 1;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    unsafe {
        *r_out = r;
        *a_mant_out = a_mant;
    }
    0
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3388 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 52)) {
//                 r = (uint64_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3388 {
// Port of the magnitude/rounding branch of internal_cvt_sf64_i64.
// Returns 0 on success (result written through r_out) and 1 on overflow
// (in which case *pfflags has the overflow bit set and the caller must
// return r_max).

use core::ffi::{c_int, c_uint};

const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RDN: u32 = 2;
const RM_RUP: u32 = 3;
const RM_RMM: u32 = 4;

// (64 - 2) - 52
const MANT_SHIFT: u32 = 10;
// (64 - 1 - 52)
const EXP_MAX: i32 = 11;

#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

#[inline]
fn set_overflow(pfflags: *mut u32) -> c_int {
    unsafe {
        *pfflags |= 1u32 << 4;
    }
    1
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3388(
    a_mant_in: u64,
    a_exp: i32,
    a_sign: u32,
    rm: c_uint,
    r_max: u64,
    pfflags: *mut u32,
    r_out: *mut u64,
) -> c_int {
    let rm: u32 = rm;
    let mut a_mant: u64 = a_mant_in;

    if a_exp >= 0 {
        if a_exp <= EXP_MAX {
            let r: u64 = (a_mant >> MANT_SHIFT) << (a_exp as u32);
            if r > r_max {
                return set_overflow(pfflags);
            }
            unsafe {
                *r_out = r;
            }
        } else {
            return set_overflow(pfflags);
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u64 = match rm {
            RM_RNE | RM_RMM => 1u64 << (MANT_SHIFT - 1),
            RM_RTZ => 0,
            RM_RDN | RM_RUP | _ => {
                if (a_sign ^ (rm & 1)) != 0 {
                    (1u64 << MANT_SHIFT) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u64 = a_mant & ((1u64 << MANT_SHIFT) - 1);
        a_mant = (a_mant + addend) >> MANT_SHIFT;

        if rm == RM_RNE && rnd_bits == (1u64 << (MANT_SHIFT - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max {
            return set_overflow(pfflags);
        }
        unsafe {
            *r_out = a_mant;
        }
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 0;
            }
        }
    }
    0
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3410 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 52)) {
//                 r = (uint128_t)(a_mant >> ((64 - 2) - 52)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd64(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((64 - 2) - 52) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((64 - 2) - 52)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((64 - 2) - 52) ) - 1);
//             a_mant = (a_mant + addend) >> ((64 - 2) - 52);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3410 {
// Port of the fast-path / rounded-shrink block of internal_cvt_sf64_i128.
// Returns 1 when the caller must return r_max with the inexact(-overflow)
// flag already set in *pfflags, 0 when conversion fell through with the
// result stored in *p_r.

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    // C: if (d != 0) { if (d >= 64) a = (a != 0); else { mask = (1<<d)-1;
    //    a = (a >> d) | ((a & mask) != 0); } } return a;
    let mut a = a;
    if d <= 0 {
        return a;
    }
    if d >= 64 {
        a = if a != 0 { 1 } else { 0 };
    } else {
        let mask: u64 = ((1u64) << (d as u32)) - 1;
        a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3410(
    a_exp: i32,
    a_sign: u32,
    rm: u32,
    pfflags: *mut u32,
    p_r_max: *const u128,
    a_mant_in: u64,
    p_r: *mut u128,
) -> i32 {
    unsafe {
        let r_max: u128 = *p_r_max;
        let r: u128;

        // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
        if a_exp >= 0 {
            if a_exp <= (128 - 1 - 52) {
                r = ((a_mant_in >> ((64 - 2) - 52)) as u128) << (a_exp as u32);
                if r > r_max {
                    // goto overflow
                    *pfflags |= 1u32 << 4;
                    return 1;
                }
            } else {
                // overflow:
                *pfflags |= 1u32 << 4;
                return 1;
            }
        } else {
            let mut a_mant: u64 = rshift_rnd64(a_mant_in, a_exp.wrapping_neg());

            let addend: u32 = match rm {
                0 | 4 => 1u32 << (((64 - 2) - 52) - 1),
                1 => 0,
                _ => {
                    if (a_sign ^ (rm & 1)) != 0 {
                        (1u32 << ((64 - 2) - 52)) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = (a_mant & (((1u64) << ((64 - 2) - 52)) - 1)) as u32;
            a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

            if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
                a_mant &= !(1u64);
            }
            if (a_mant as u128) > r_max {
                // goto overflow
                *pfflags |= 1u32 << 4;
                return 1;
            }
            r = a_mant as u128;
            if rnd_bits != 0 {
                *pfflags |= 1u32 << 0;
            }
        }
        *p_r = r;
        0
    }
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3432 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (32 - 1 - 112)) {
//                 r = (uint32_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3432 {
// Ported piece of internal_cvt_sf128_i32: the exponent/rounding block.
// Returns 0 on the normal path (result stored through r_out), 1 on overflow
// (the C caller then sets nothing more and returns r_max).

const SH: u32 = (128u32 - 2u32) - 112u32; // 14

// private port of the C `static uint128_t rshift_rnd128(uint128_t a, int d)`
fn rshift_rnd128(mut a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u128 = (1u128 << (d as u32)).wrapping_sub(1);
            let sticky: u128 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> (d as u32)) | sticky;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3432(
    a_exp: i32,
    mut a_mant: u128,
    a_sign: u32,
    rm: i32,
    r_max: u32,
    pfflags: *mut u32,
    r_out: *mut u32,
) -> i32 {
    unsafe {
        if a_exp >= 0 {
            if a_exp <= (32 - 1 - 112) {
                let rr: u32 = ((a_mant >> SH) as u32).wrapping_shl(a_exp as u32);
                if rr > r_max {
                    *pfflags |= 1u32 << 4;
                    return 1;
                }
                *r_out = rr;
            } else {
                // overflow label
                *pfflags |= 1u32 << 4;
                return 1;
            }
        } else {
            a_mant = rshift_rnd128(a_mant, a_exp.wrapping_neg());

            let addend: u32 = match rm {
                0 | 4 => 1u32 << (SH - 1), // RM_RNE, RM_RMM
                1 => 0,                    // RM_RTZ
                _ => {
                    // default: RM_RDN, RM_RUP (and anything else)
                    if (a_sign ^ ((rm as u32) & 1)) != 0 {
                        (1u32 << SH) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = (a_mant & ((1u128 << SH) - 1)) as u32;
            a_mant = a_mant.wrapping_add(addend as u128) >> SH;

            if rm == 0 && rnd_bits == (1u32 << (SH - 1)) {
                a_mant &= !1u128;
            }
            if a_mant > r_max as u128 {
                // overflow label
                *pfflags |= 1u32 << 4;
                return 1;
            }
            *r_out = a_mant as u32;
            if rnd_bits != 0 {
                *pfflags |= 1u32 << 0;
            }
        }
        0
    }
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3454 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (64 - 1 - 112)) {
//                 r = (uint64_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3454 {
// Ported piece of internal_cvt_sf128_i64 (softfp_template_icvt.h lines 63-104)
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// Port of the static C helper rshift_rnd128 (no linker symbol).
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else {
            let mask: u128 = (1u128 << (d as u32)) - 1;
            (a >> (d as u32)) | ((a & mask) != 0) as u128
        }
    } else {
        a
    }
}

// Returns 1 when the C code must take the "overflow" path (caller returns
// r_max), 0 otherwise.  The result value is stored through `pr`.
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3454(
    a_exp: c_int,
    a_mant_in: u128,
    r_max: u64,
    rm: c_int,
    a_sign: u32,
    pfflags: *mut u32,
    pr: *mut u64,
) -> c_int {
    let mut a_mant: u128 = a_mant_in;
    let mut r: u64 = 0;

    unsafe {
        if a_exp >= 0 {
            if a_exp <= (64 - 1 - 112) {
                r = (a_mant >> 14) as u64;
                r = r.wrapping_shl(a_exp as u32);
                if r > r_max {
                    *pfflags |= 1u32 << 4;
                    return 1;
                }
            } else {
                *pfflags |= 1u32 << 4;
                return 1;
            }
        } else {
            a_mant = rshift_rnd128(a_mant, -a_exp);

            let addend: u32;
            if rm == RM_RNE || rm == RM_RMM {
                addend = 1u32 << 13;
            } else if rm == RM_RTZ {
                addend = 0;
            } else {
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << 14) - 1;
                } else {
                    addend = 0;
                }
            }

            let rnd_bits: u32 = (a_mant & (((1u128 << 14) - 1) as u128)) as u32;
            a_mant = a_mant.wrapping_add(addend as u128) >> 14;

            if rm == RM_RNE && rnd_bits == (1u32 << 13) {
                a_mant &= !1u128;
            }
            if a_mant > (r_max as u128) {
                *pfflags |= 1u32 << 4;
                return 1;
            }
            r = a_mant as u64;
            if rnd_bits != 0 {
                *pfflags |= 1u32 << 0;
            }
        }

        *pr = r;
    }

    0
}
}

// ── `if (a_exp >= 0) … else` · softfp_template_icvt.h:63-104 · chunk 3476 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_exp >= 0) {
//             if (a_exp <= (128 - 1 - 112)) {
//                 r = (uint128_t)(a_mant >> ((128 - 2) - 112)) << a_exp;
//                 if (r > r_max)
//                     goto overflow;
//             } else {
//             overflow:
//                 *pfflags |= (1 << 4);
//                 return r_max;
//             }
//         } else {
//             a_mant = rshift_rnd128(a_mant, -a_exp);
//     
//             switch(rm) {
//             case RM_RNE:
//             case RM_RMM:
//                 addend = (1 << (((128 - 2) - 112) - 1));
//                 break;
//             case RM_RTZ:
//                 addend = 0;
//                 break;
//             default:
//             case RM_RDN:
//             case RM_RUP:
//                 if (a_sign ^ (rm & 1))
//                     addend = (1 << ((128 - 2) - 112)) - 1;
//                 else
//                     addend = 0;
//                 break;
//             }
//     
//             rnd_bits = a_mant & ((1 << ((128 - 2) - 112) ) - 1);
//             a_mant = (a_mant + addend) >> ((128 - 2) - 112);
//     
//             if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)))
//                 a_mant &= ~1;
//             if (a_mant > r_max)
//                 goto overflow;
//             r = a_mant;
//             if (rnd_bits != 0)
//                 *pfflags |= (1 << 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3476 {
// Port of the exponent/rounding branch of internal_cvt_sf128_i128
// (softfp_template_icvt.h lines 63-104).
//
// Returns 0 when the caller should continue with the value written to *pr,
// and 1 when the original code would have jumped to `overflow` (i.e. raise
// the invalid flag, already OR-ed into *pfflags, and return r_max).

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RMM: i32 = 4;

// static C function rshift_rnd128(): no linker symbol, so its logic is
// reproduced here as a private Rust function.
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask: u128 = (1u128 << du).wrapping_sub(1);
            let lsb: u128 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> du) | lsb;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3476(
    a_mant_in: u128,
    a_exp: i32,
    r_max: u128,
    a_sign: u32,
    rm: i32,
    pfflags: *mut u32,
    pr: *mut u128,
) -> i32 {
    let mut a_mant: u128 = a_mant_in;
    let mut r: u128 = 0;

    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 112) {
            r = (a_mant >> ((128 - 2) - 112)) << (a_exp as u32);
            if r > r_max {
                // goto overflow
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return 1;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return 1;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, a_exp.wrapping_neg());

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((128 - 2) - 112) - 1),
            RM_RTZ => 0,
            _ => {
                // default: RM_RDN, RM_RUP
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((128 - 2) - 112)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & (((1u128 << ((128 - 2) - 112)) - 1))) as u32;
        a_mant = (a_mant.wrapping_add(addend as u128)) >> ((128 - 2) - 112);

        if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > r_max {
            // goto overflow
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return 1;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32;
            }
        }
    }

    unsafe {
        *pr = r;
    }
    0
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3301 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3301 {
// Ported piece of internal_cvt_sf32_i32: apply the sign to the result.
// C original:
//     if (a_sign)
//         r = -r;
// r is uint32_t, so the negation wraps; a_sign is a 0/1 value held in uint32_t.
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3301(a_sign: u32, r: u32) -> u32 {
    if a_sign != 0 {
        r.wrapping_neg()
    } else {
        r
    }
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3323 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3323 {
// Port of the trailing sign-fixup of internal_cvt_sf32_i64:
//   if (a_sign) r = -r;
// `r` is uint64_t, so the negation wraps modulo 2^64.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3323(a_sign: u32, r: *mut u64) -> u64 {
    if a_sign != 0 {
        unsafe {
            *r = (*r).wrapping_neg();
        }
    }
    unsafe { *r }
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3345 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3345 {
// Applies the sign of the source float to the converted unsigned result.
// C original: if (a_sign) r = -r;   (r is uint128_t, so negation wraps)

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3345(a_sign: u32, r: *mut u128) {
    if a_sign != 0 {
        unsafe {
            let v: u128 = *r;
            *r = 0u128.wrapping_sub(v);
        }
    }
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3367 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3367 {
// Ported piece of internal_cvt_sf64_i32: sign application on the result.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3367(a_sign: u32, r: u32) -> u32 {
    let mut r = r;
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3389 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3389 {
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3389(a_sign: u32, r: u64) -> u64 {
    if a_sign != 0 {
        r.wrapping_neg()
    } else {
        r
    }
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3411 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3411 {
// Port of the TARGET: `if (a_sign) r = -r;`
// a_sign is uint32_t (truthy when nonzero), r is uint128_t (unsigned 128-bit),
// so C's unary minus is the wrapping modular negation.

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3411(a_sign: u32, pr: *mut u128) {
    if a_sign != 0 && !pr.is_null() {
        // read/write unaligned so the code never relies on u128 alignment
        let r: u128 = unsafe { core::ptr::read_unaligned(pr) };
        let neg: u128 = 0u128.wrapping_sub(r);
        unsafe { core::ptr::write_unaligned(pr, neg) };
    }
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3433 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3433 {
// Port of the tail of internal_cvt_sf128_i32: apply the sign of the
// floating-point operand to the converted unsigned magnitude.
// In the C original `r` is a uint32_t, so `r = -r` is an unsigned wrap.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3433(a_sign: u32, r: u32) -> u32 {
    let mut r = r;
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3455 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3455 {
// Port of the tail of internal_cvt_sf128_i64 in softfp_template_icvt.h:
// apply the sign to the magnitude r.
use core::ffi::{c_uint, c_ulong};

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3455(a_sign: c_uint, mut r: c_ulong) -> c_ulong {
    // a_sign is a uint32_t holding 0 or 1
    if a_sign != 0 {
        // r is a uint64_t, so this is unsigned wrapping negation
        r = r.wrapping_neg();
    }
    r
}
}

// ── `if (a_sign)` · softfp_template_icvt.h:105-106 · chunk 3477 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (a_sign)
//             r = -r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3477 {
// Ported piece: negation of the 128-bit unsigned mantissa when the sign bit is set.
// C original:
//     if (a_sign)
//         r = -r;
// `r` is uint128_t (unsigned __int128), so the negation wraps modulo 2^128.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3477(a_sign: u32, r: u128) -> u128 {
    if a_sign != 0 {
        0u128.wrapping_sub(r)
    } else {
        r
    }
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3302 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3302 {
// Ported piece of internal_cvt_sf32_i32: convert the computed magnitude to the
// int32_t result value and return it (C: uint32_t -> int32_t is a bit reinterpret).
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i32__part3302(r: u32) -> i32 {
    r as i32
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3324 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3324 {
// Ported tail of internal_cvt_sf32_i64: `return r;`
// The C original converts the uint64_t accumulator `r` to int64_t on return.

#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i64__part3324(r: u64) -> i64 {
    // Same as the C implicit conversion uint64_t -> int64_t (two's complement,
    // modulo 2^64), which is what gcc emits.
    r as i64
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3346 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3346 {
// Ported tail of internal_cvt_sf32_i128: return the accumulated 128-bit result,
// converted from uint128_t to int128_t (two's-complement wrap-around as in C).
#[no_mangle]
pub extern "C" fn internal_cvt_sf32_i128__part3346(r: u128) -> i128 {
    r as i128
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3368 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3368 {
// Port of the tail of internal_cvt_sf64_i32: return the 32-bit result.
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i32__part3368(r: c_uint) -> c_int {
    // C: `return r;` where r is uint32_t and the function returns int32_t
    // (two's-complement wrap-around, as done by the C compiler).
    r as c_int
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3390 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3390 {
// Port of the tail `return r;` of internal_cvt_sf64_i64:
// converts the unsigned 64-bit magnitude `r` to the function's int64_t
// return type (same-width unsigned -> signed conversion, i.e. bit
// reinterpretation as done by gcc).
use core::ffi::{c_long, c_ulong};

#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i64__part3390(r: c_ulong) -> c_long {
    r as c_long
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3412 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3412 {
// Port of the final step of internal_cvt_sf64_i128:
// conversion of the unsigned 128-bit result `r` to the signed return type.
#[no_mangle]
pub extern "C" fn internal_cvt_sf64_i128__part3412(r: u128) -> i128 {
    r as i128
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3434 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3434 {
// Tail of internal_cvt_sf128_i32: convert the unsigned magnitude in `r`
// to the signed result type (two's-complement wrap-around, as in C).
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i32__part3434(r: u32) -> i32 {
    r as i32
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3456 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3456 {
// Tail of internal_cvt_sf128_i64: apply the sign and return the 64-bit result.
#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i64__part3456(r: u64, a_sign: u32) -> i64 {
    let mut r = r;
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}
}

// ── `return r;` · softfp_template_icvt.h:107-107 · chunk 3478 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return r;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3478 {
// Portion of internal_cvt_sf128_i128: the final `return r;`
// r is a uint128_t (unsigned __int128); the C function returns int128_t
// (__int128), so the value is reinterpreted modulo 2^128.

#[no_mangle]
pub extern "C" fn internal_cvt_sf128_i128__part3478(r: u128) -> i128 {
    r as i128
}
}

// ── `cvt_sf32_i32` · softfp_template_icvt.h:110-115 · chunk 3303 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int32_t cvt_sf32_i32(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i32(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3303 {
// Port of cvt_sf32_i32 (softfp_template_icvt.h): wrapper around the static
// internal_cvt_sf32_i32() with is_unsigned = 0, logic inlined here.

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else if d > 0 {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf32_i32(a: u32, rm: c_int, pfflags: *mut u32) -> i32 {
    unsafe {
        let mut a_sign: u32 = a >> (32 - 1);
        let mut a_exp: i32 = ((a >> 23) & ((1 << 8) - 1)) as i32;
        let mut a_mant: u32 = a & ((1u32 << 23) - 1);

        if a_exp == (1 << 8) - 1 && a_mant != 0 {
            a_sign = 0;
        }
        if a_exp == 0 {
            a_exp = 1;
        } else {
            a_mant |= 1u32 << 23;
        }
        a_mant <<= (32 - 2) - 23;
        a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;

        // is_unsigned == 0
        let r_max: u32 = (1u32 << (32 - 1)) - (a_sign ^ 1);

        let mut r: u32 = 0;
        let mut overflow = false;

        if a_exp >= 0 {
            if a_exp <= (32 - 1 - 23) {
                r = (a_mant >> ((32 - 2) - 23)) << a_exp;
                if r > r_max {
                    overflow = true;
                }
            } else {
                overflow = true;
            }
        } else {
            let d: i32 = (-a_exp) as i32;
            a_mant = rshift_rnd32(a_mant, d);

            let addend: u32 = match rm {
                0 | 4 => 1u32 << (((32 - 2) - 23) - 1), // RM_RNE, RM_RMM
                1 => 0,                                 // RM_RTZ
                _ => {
                    // RM_RDN, RM_RUP, default
                    if (a_sign ^ ((rm as u32) & 1)) != 0 {
                        (1u32 << ((32 - 2) - 23)) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
            a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

            if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
                a_mant &= !1u32;
            }
            if a_mant > r_max {
                overflow = true;
            } else {
                r = a_mant;
                if rnd_bits != 0 {
                    *pfflags |= FFLAG_INEXACT;
                }
            }
        }

        if overflow {
            *pfflags |= FFLAG_INVALID;
            return r_max as i32;
        }

        if a_sign != 0 {
            r = r.wrapping_neg();
        }
        r as i32
    }
}
}

// ── `cvt_sf32_i64` · softfp_template_icvt.h:110-115 · chunk 3325 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t cvt_sf32_i64(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i64(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3325 {
// Port of softfloat's cvt_sf32_i64 (softfp_template_icvt.h), including the
// static helper internal_cvt_sf32_i64 with is_unsigned = FALSE.

use core::ffi::c_int;

// RoundingModeEnum values
const RM_RNE: c_int = 0;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            (a != 0) as u32
        } else {
            let mask: u32 = (1u32 << d) - 1;
            (a >> d) | ((a & mask) != 0) as u32
        }
    } else {
        a
    }
}

fn internal_cvt_sf32_i64(a: u32, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> i64 {
    let mut a_sign: u32;
    let mut addend: u32;
    let mut rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let mut r: u64;
    let r_max: u64;

    unsafe {
        a_sign = a >> (32 - 1);
        a_exp = ((a >> 23) & ((1i32 << 8) - 1) as u32) as i32;
        a_mant = a & (((1u32 << 23) - 1) as u32);
        if a_exp == (1 << 8) - 1 && a_mant != 0 {
            a_sign = 0;
        }
        if a_exp == 0 {
            a_exp = 1;
        } else {
            a_mant |= 1u32 << 23;
        }
        a_mant = a_mant.wrapping_shl((32 - 2) - 23);
        a_exp = a_exp - ((1 << 8) - 1) / 2 - 23;

        if is_unsigned {
            r_max = (a_sign as u64).wrapping_sub(1);
        } else {
            r_max = ((1u64) << (64 - 1)) - (a_sign ^ 1) as u64;
        }

        let mut overflow = false;
        let mut r_out: u64 = 0;

        if a_exp >= 0 {
            if a_exp <= (64 - 1 - 23) {
                let v: u64 = ((a_mant >> ((32 - 2) - 23)) as u64).wrapping_shl(a_exp as u32);
                if v > r_max {
                    overflow = true;
                } else {
                    r_out = v;
                }
            } else {
                overflow = true;
            }
        } else {
            a_mant = rshift_rnd32(a_mant, -a_exp);

            match rm {
                RM_RNE | RM_RMM => {
                    addend = 1u32 << (((32 - 2) - 23) - 1);
                }
                1 => {
                    // RM_RTZ
                    addend = 0;
                }
                _ => {
                    // RM_RDN, RM_RUP, default
                    if (a_sign ^ ((rm & 1) as u32)) != 0 {
                        addend = (1u32 << ((32 - 2) - 23)) - 1;
                    } else {
                        addend = 0;
                    }
                }
            }

            rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
            a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

            if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
                a_mant &= !1u32;
            }
            if (a_mant as u64) > r_max {
                overflow = true;
            } else {
                r_out = a_mant as u64;
                if rnd_bits != 0 {
                    *pfflags |= FFLAG_INEXACT;
                }
            }
        }

        if overflow {
            *pfflags |= FFLAG_INVALID;
            return r_max as i64;
        }

        r = r_out;
        if a_sign != 0 {
            r = (-(r as i64)) as u64;
        }
        r as i64
    }
}

#[no_mangle]
pub extern "C" fn cvt_sf32_i64(a: u32, rm: c_int, pfflags: *mut u32) -> i64 {
    internal_cvt_sf32_i64(a, rm, pfflags, false)
}
}

// ── `cvt_sf32_i128` · softfp_template_icvt.h:110-115 · chunk 3347 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int128_t cvt_sf32_i128(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i128(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3347 {
// Port of: int128_t cvt_sf32_i128(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags)
// Inlines the static helper internal_cvt_sf32_i128(a, rm, pfflags, FALSE).

use core::ffi::{c_int, c_uint};

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint32_t rshift_rnd32(uint32_t a, int d)
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

#[no_mangle]
pub unsafe extern "C" fn cvt_sf32_i128(
    a: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> i128 {
    let mut a_sign: u32;
    let mut a_exp: c_int;
    let mut a_mant: u32;
    let mut r: u128;
    let mut r_max: u128;
    let mut addend: u32;
    let mut rnd_bits: u32;

    let is_unsigned: bool = false;

    a_sign = a >> (32 - 1);
    a_exp = ((a >> 23) & ((1u32 << 8) - 1)) as c_int;
    a_mant = a & ((1u32 << 23) - 1);
    if a_exp == (1 << 8) - 1 && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - ((1 << 8) - 1) / 2 - 23;

    if is_unsigned {
        r_max = (a_sign as u128).wrapping_sub(1);
    } else {
        r_max = (1u128 << (128 - 1)) - (a_sign ^ 1) as u128;
    }
    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 23) {
            r = ((a_mant >> ((32 - 2) - 23)) as u128).wrapping_shl(a_exp as u32);
            if r > r_max {
                *pfflags |= FFLAG_INVALID;
                return r_max as i128;
            }
        } else {
            *pfflags |= FFLAG_INVALID;
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        match rm {
            RM_RNE | RM_RMM => {
                addend = 1u32 << (((32 - 2) - 23) - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP and default
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    addend = (1u32 << ((32 - 2) - 23)) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if (a_mant as u128) > r_max {
            *pfflags |= FFLAG_INVALID;
            return r_max as i128;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}

// keep c_uint referenced (enum underlying type helper) without warnings
#[allow(dead_code)]
fn _unused(_x: c_uint) {}
}

// ── `cvt_sf64_i32` · softfp_template_icvt.h:110-115 · chunk 3369 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int32_t cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i32(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3369 {
// Port of softfp_template_icvt.h: cvt_sf64_i32
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u64 = (1u64 << d) - 1;
            let sticky: u64 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> d) | sticky;
        }
    }
    a
}

fn internal_cvt_sf64_i32(a: u64, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> i32 {
    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1i64 << 11) - 1) as u64) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

    let r_max: u32;
    if is_unsigned {
        r_max = a_sign.wrapping_sub(1);
    } else {
        r_max = (1u32 << (32 - 1)) - (a_sign ^ 1);
    }

    let mut r: u32 = 0;
    let mut overflow: bool = false;

    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 52) {
            // dead branch in practice (32 - 1 - 52 < 0), kept for fidelity
            let t: u32 = ((a_mant >> ((64 - 2) - 52)) as u32).wrapping_shl(a_exp as u32);
            if t > r_max {
                overflow = true;
            } else {
                r = t;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((64 - 2) - 52) - 1),
            RM_RTZ => 0,
            _ => {
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & (((1u64 << ((64 - 2) - 52)) as u64) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

        if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max as u64 {
            overflow = true;
        } else {
            r = a_mant as u32;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= FFLAG_INEXACT;
                }
            }
        }
    }

    if overflow {
        unsafe {
            *pfflags |= FFLAG_INVALID;
        }
        return r_max as i32;
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub extern "C" fn cvt_sf64_i32(a: u64, rm: c_int, pfflags: *mut u32) -> i32 {
    internal_cvt_sf64_i32(a, rm, pfflags, false)
}
}

// ── `cvt_sf64_i64` · softfp_template_icvt.h:110-115 · chunk 3391 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t cvt_sf64_i64(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i64(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3391 {
// Port of softfp_template_icvt.h: cvt_sf64_i64 (double -> int64 conversion)
// The C body forwards to the file-static internal_cvt_sf64_i64(); since that
// helper has no linker symbol, its logic is reproduced here as a private fn.

use core::ffi::{c_int};

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            (a != 0) as u64
        } else {
            let mask = (1u64 << d) - 1;
            (a >> d) | ((a & mask) != 0) as u64
        }
    } else {
        a
    }
}

fn internal_cvt_sf64_i64(a: u64, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> i64 {
    // RoundingModeEnum: RM_RNE=0, RM_RTZ=1, RM_RDN=2, RM_RUP=3, RM_RMM=4
    const FFLAG_INEXACT: u32 = 1 << 0;
    const FFLAG_INVALID: u32 = 1 << 4;

    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - ((1 << 11) - 1) / 2 - 52;

    let r_max: u64 = if is_unsigned {
        (a_sign as u64).wrapping_sub(1)
    } else {
        (1u64 << (64 - 1)).wrapping_sub((a_sign ^ 1) as u64)
    };

    let mut r: u64;

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 52) {
            r = (a_mant >> ((64 - 2) - 52)) << a_exp;
            if r > r_max {
                unsafe {
                    *pfflags |= FFLAG_INVALID;
                }
                return r_max as i64;
            }
        } else {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1u32 << (((64 - 2) - 52) - 1),
            1 => 0,
            _ => {
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

        if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}

#[no_mangle]
pub extern "C" fn cvt_sf64_i64(a: u64, rm: c_int, pfflags: *mut u32) -> i64 {
    internal_cvt_sf64_i64(a, rm, pfflags, false)
}
}

// ── `cvt_sf64_i128` · softfp_template_icvt.h:110-115 · chunk 3413 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int128_t cvt_sf64_i128(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i128(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3413 {
// Port of softfp_template_icvt.h: cvt_sf64_i128 (uint64_t -> int128_t)
// The C helper `internal_cvt_sf64_i128` is `static` (no linker symbol), so its
// logic is reproduced here as a private Rust function.

#![allow(non_camel_case_types)]

use core::ffi::c_uint;

pub type RoundingModeEnum = c_uint;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RMM: RoundingModeEnum = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint64_t rshift_rnd64(uint64_t a, int d)
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

// static int128_t internal_cvt_sf64_i128(uint64_t a, RoundingModeEnum rm,
//                                        uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_sf64_i128(
    a: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
    is_unsigned: bool,
) -> i128 {
    let mut a_sign: u32 = (a >> 63) as u32;
    let mut a_exp: i32 = ((a >> 52) & (((1u64 << 11) - 1) as u64)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant = a_mant.wrapping_shl(10);
    a_exp = a_exp - (((1i32 << 11) - 1) / 2) - 52;

    let r_max: u128 = if is_unsigned {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << 127) - ((a_sign ^ 1) as u128)
    };

    let mut r: u128;

    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 52) {
            r = ((a_mant >> 10) as u128) << (a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= FFLAG_INVALID;
                }
                return r_max as i128;
            }
        } else {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32;
        match rm {
            RM_RNE | RM_RMM => {
                addend = 1u32 << (((64 - 2) - 52) - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP and any other value
                if (a_sign ^ (rm & 1)) != 0 {
                    addend = (1u32 << ((64 - 2) - 52)) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        let rnd_bits: u32 = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1) as u64)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

        if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if (a_mant as u128) > r_max {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i128;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}

#[no_mangle]
pub extern "C" fn cvt_sf64_i128(a: u64, rm: RoundingModeEnum, pfflags: *mut u32) -> i128 {
    internal_cvt_sf64_i128(a, rm, pfflags, false)
}
}

// ── `cvt_sf128_i32` · softfp_template_icvt.h:110-115 · chunk 3435 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int32_t cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i32(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3435 {
// Port of cvt_sf128_i32 (softfp_template_icvt.h) plus the static helper
// internal_cvt_sf128_i32 which has no linker symbol.

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint128_t rshift_rnd128(uint128_t a, int d)
#[inline]
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d == 0 {
        return a;
    }
    if d >= 128 {
        return if a != 0 { 1 } else { 0 };
    }
    if d < 0 {
        // C would be undefined here; no caller does this.
        return a;
    }
    let dd = d as u32;
    let mask = (1u128 << dd).wrapping_sub(1);
    (a >> dd) | if (a & mask) != 0 { 1 } else { 0 }
}

fn internal_cvt_sf128_i32(a: u128, rm: u32, pfflags: *mut u32, is_unsigned: bool) -> i32 {
    let mut a_sign: u32 = (a >> 127) as u32;
    let mut a_exp: i32 = ((a >> 112) & 0x7fff) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == 0x7fff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= 14;
    a_exp = a_exp - 0x3fff - 112;

    let r_max: u32 = if is_unsigned {
        a_sign.wrapping_sub(1)
    } else {
        (1u32 << 31).wrapping_sub(a_sign ^ 1)
    };

    let mut r: u32;

    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 112) {
            r = ((a_mant >> 14) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= FFLAG_INVALID;
                }
                return r_max as i32;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i32;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1 << 13, // RM_RNE, RM_RMM
            1 => 0,           // RM_RTZ
            _ => {
                // RM_RDN, RM_RUP and default
                if (a_sign ^ (rm & 1)) != 0 {
                    (1 << 14) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & 0x3fff) as u32;
        a_mant = (a_mant + addend as u128) >> 14;

        if rm == 0 && rnd_bits == (1 << 13) {
            a_mant &= !1u128;
        }
        if a_mant > r_max as u128 {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i32;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i32(a: u128, rm: u32, pfflags: *mut u32) -> i32 {
    internal_cvt_sf128_i32(a, rm, pfflags, false)
}
}

// ── `cvt_sf128_i64` · softfp_template_icvt.h:110-115 · chunk 3457 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t cvt_sf128_i64(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i64(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3457 {
// Port of cvt_sf128_i64 (softfp_template_icvt.h lines 110-115).
// The C function forwards to the file-static internal_cvt_sf128_i64 with
// is_unsigned = FALSE; that static helper has no linker symbol, so its logic
// (specialised for the 128-bit float -> 64-bit signed integer case) is
// reproduced here.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let du = d as u32;
            let mask: u128 = (1u128 << du) - 1;
            a = (a >> du) | (((a & mask) != 0) as u128);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i64(a: u128, rm: c_int, pfflags: *mut u32) -> i64 {
    // internal_cvt_sf128_i64(a, rm, pfflags, FALSE)
    let mut a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let r_max: u64;

    a_sign = (a >> 127) as u32;
    a_exp = ((a >> 112) & 0x7fff) as i32;
    a_mant = a & ((1u128 << 112) - 1);
    if a_exp == 0x7fff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= 14; // (128 - 2) - 112
    a_exp = a_exp - 16383 - 112; // ((1 << 15) - 1) / 2 == 16383

    // is_unsigned == FALSE
    r_max = (1u64 << 63) - ((a_sign ^ 1) as u64);

    let mut r: u64;

    if a_exp >= 0 {
        // The non-overflow branch requires a_exp <= (64 - 1 - 112) == -49,
        // which can never hold while a_exp >= 0, so this is always overflow.
        if a_exp <= (64 - 1 - 112) {
            r = ((a_mant >> 14) as u64) << a_exp;
            if r > r_max {
                unsafe {
                    *pfflags |= FFLAG_INVALID;
                }
                return r_max as i64;
            }
        } else {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u128 = match rm {
            RM_RNE | RM_RMM => 1u128 << (14 - 1),
            RM_RTZ => 0,
            _ => {
                // RM_RDN, RM_RUP and anything else
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u128 << 14) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u128 << 14) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend) >> 14;

        if rm == RM_RNE && rnd_bits == (1u32 << (14 - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > (r_max as u128) {
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }

    if a_sign != 0 {
        r = 0u64.wrapping_sub(r);
    }
    r as i64
}
}

// ── `cvt_sf128_i128` · softfp_template_icvt.h:110-115 · chunk 3479 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int128_t cvt_sf128_i128(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i128(a, rm,
//                                                                         pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3479 {
// Port of cvt_sf128_i128 (softfp_template_icvt.h), with the static helper
// internal_cvt_sf128_i128 (is_unsigned == FALSE) and rshift_rnd128 inlined.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

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
            let mask = (1u128 << (d as u32)) - 1;
            (a >> (d as u32)) | (((a & mask) != 0) as u128)
        }
    } else {
        a
    }
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i128(a: u128, rm: c_int, pfflags: *mut u32) -> i128 {
    unsafe {
        let mut a_sign: u32 = (a >> 127) as u32;
        let mut a_exp: i32 = ((a >> 112) & 0x7fff) as i32;
        let mut a_mant: u128 = a & ((1u128 << 112) - 1);

        if a_exp == 0x7fff && a_mant != 0 {
            a_sign = 0;
        }
        if a_exp == 0 {
            a_exp = 1;
        } else {
            a_mant |= 1u128 << 112;
        }
        a_mant <<= 14;
        a_exp = a_exp - 16383 - 112;

        // is_unsigned == FALSE
        let r_max: u128 = (1u128 << 127) - ((a_sign ^ 1) as u128);

        let mut r: u128;

        if a_exp >= 0 {
            if a_exp <= 15 {
                r = (a_mant >> 14).wrapping_shl(a_exp as u32);
                if r > r_max {
                    *pfflags |= 1 << 4;
                    return r_max as i128;
                }
            } else {
                *pfflags |= 1 << 4;
                return r_max as i128;
            }
        } else {
            a_mant = rshift_rnd128(a_mant, a_exp.wrapping_neg());

            let addend: u32 = match rm {
                RM_RNE | RM_RMM => 1 << 13,
                RM_RTZ => 0,
                _ => {
                    // default: RM_RDN, RM_RUP
                    if (a_sign ^ ((rm & 1) as u32)) != 0 {
                        (1 << 14) - 1
                    } else {
                        0
                    }
                }
            };

            let rnd_bits: u32 = (a_mant & ((1u128 << 14) - 1)) as u32;
            a_mant = (a_mant + (addend as u128)) >> 14;

            if rm == RM_RNE && rnd_bits == (1 << 13) {
                a_mant &= !1u128;
            }
            if a_mant > r_max {
                *pfflags |= 1 << 4;
                return r_max as i128;
            }
            r = a_mant;
            if rnd_bits != 0 {
                *pfflags |= 1 << 0;
            }
        }

        if a_sign != 0 {
            r = r.wrapping_neg();
        }
        r as i128
    }
}
}

// ── `return internal_cvt_sf32_i32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3304 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i32(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3304 {
use core::ffi::c_int;

// Port of the (static, non-exported) C helper internal_cvt_sf32_i32 with
// is_unsigned == 0 (FALSE).
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
pub extern "C" fn cvt_sf32_i32__part3304(a: u32, rm: c_int, pfflags: *mut u32) -> c_int {
    // a_sign = a >> (32 - 1);
    let mut a_sign: u32 = a >> 31;
    // a_exp = (a >> 23) & ((1 << 8) - 1);
    let mut a_exp: i32 = ((a >> 23) & 0xff) as i32;
    // a_mant = a & (((uint32_t)1 << 23) - 1);
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);

    // if (a_exp == ((1 << 8) - 1) && a_mant != 0) a_sign = 0;
    if a_exp == 0xff && a_mant != 0 {
        a_sign = 0;
    }
    // if (a_exp == 0) { a_exp = 1; } else { a_mant |= (uint32_t)1 << 23; }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    // a_mant <<= ((32 - 2) - 23);   (RND_SIZE = 7)
    a_mant <<= 7;
    // a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
    a_exp = a_exp - 127 - 23;

    // is_unsigned == FALSE:
    // r_max = ((uint32_t)1 << (32 - 1)) - (uint32_t)(a_sign ^ 1);
    let r_max: u32 = (1u32 << 31) - (a_sign ^ 1);

    let mut r: u32 = 0;
    let mut overflow: bool = false;

    if a_exp >= 0 {
        if a_exp <= 8 {
            // r = (uint32_t)(a_mant >> 7) << a_exp;
            r = (a_mant >> 7).wrapping_shl(a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        // switch (rm) { ... }
        let addend: u32 = match rm {
            0 | 4 => 1u32 << 6, // RM_RNE, RM_RMM
            1 => 0,             // RM_RTZ
            _ => {
                // RM_RDN, RM_RUP (and default)
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << 7) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << 7) - 1);
        a_mant = a_mant.wrapping_add(addend) >> 7;

        if rm == 0 && rnd_bits == (1u32 << 6) {
            a_mant &= !1u32;
        }
        if a_mant > r_max {
            overflow = true;
        } else {
            r = a_mant;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= 1;
                }
            }
        }
    }

    if overflow {
        unsafe {
            *pfflags |= (1 << 4);
        }
        return r_max as c_int;
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as c_int
}
}

// ── `return internal_cvt_sf32_i64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3326 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i64(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3326 {
// Port of the TARGET: return internal_cvt_sf32_i64(a, rm, pfflags, FALSE);
// internal_cvt_sf32_i64 is a `static` C function, so its body is reproduced
// here as a private helper (with is_unsigned kept as a parameter).

#[inline]
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a: u32 = a;
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

fn internal_cvt_sf32_i64(a: u32, rm: i32, pfflags: *mut u32, is_unsigned: i32) -> i64 {
    let mut a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let mut addend: u32;
    let mut rnd_bits: u32;
    let mut r: u64;
    let mut overflow: bool = false;

    a_sign = a >> 31;
    a_exp = ((a >> 23) & 0xff) as i32;
    a_mant = a & 0x007f_ffffu32;
    if a_exp == 0xff && a_mant != 0 {
        // NaN is like +infinity
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= 7; // RND_SIZE = (32 - 2) - 23
    a_exp = a_exp - 127 - 23;

    let r_max: u64 = if is_unsigned != 0 {
        (a_sign as u64).wrapping_sub(1)
    } else {
        (1u64 << 63).wrapping_sub((a_sign ^ 1) as u64)
    };

    r = 0;
    if a_exp >= 0 {
        if a_exp <= 40 {
            // 64 - 1 - 23
            r = ((a_mant >> 7) as u64) << (a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

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
                // RM_RDN, RM_RUP (and anything else)
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << 7) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = a_mant & ((1u32 << 7) - 1);
        a_mant = a_mant.wrapping_add(addend) >> 7;
        // half way: select even result
        if rm == 0 && rnd_bits == (1u32 << 6) {
            a_mant &= !1u32;
        }
        if (a_mant as u64) > r_max {
            overflow = true;
        } else {
            r = a_mant as u64;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= 1u32;
                }
            }
        }
    }

    if overflow {
        unsafe {
            *pfflags |= 1u32 << 4;
        }
        return r_max as i64;
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}

#[no_mangle]
pub extern "C" fn cvt_sf32_i64__part3326(a: u32, rm: i32, pfflags: *mut u32) -> i64 {
    internal_cvt_sf32_i64(a, rm, pfflags, 0)
}
}

// ── `return internal_cvt_sf32_i128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3348 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i128(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3348 {
use core::ffi::c_int;

// Port of the static C helper rshift_rnd32() (no linker symbol exists for it).
#[inline]
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | (((a & mask) != 0) as u32);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf32_i128__part3348(a: u32, rm: c_int, pfflags: *mut u32) -> i128 {
    let mut a_sign: u32 = a >> (32 - 1);
    let mut a_exp: i32 = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);
    if a_exp == ((1u32 << 8) - 1) as i32 && a_mant != 0 {
        // NaN is like +infinity
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - (((1i32 << 8) - 1) / 2) - 23;

    // is_unsigned == FALSE
    let r_max: u128 = (1u128 << (128 - 1)) - ((a_sign ^ 1) as u128);

    let mut r: u128;
    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 23) {
            r = ((a_mant >> ((32 - 2) - 23)) as u128) << (a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return r_max as i128;
            }
        } else {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1u32 << (((32 - 2) - 23) - 1), // RM_RNE, RM_RMM
            1 => 0,                                 // RM_RTZ
            _ => {
                // RM_RDN, RM_RUP and default
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((32 - 2) - 23)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        // half way: select even result
        if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if (a_mant as u128) > r_max {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max as i128;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}
}

// ── `return internal_cvt_sf64_i32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3370 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i32(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3370 {
// Rust port of the TARGET: the call to the static C helper
// internal_cvt_sf64_i32(a, rm, pfflags, FALSE) from cvt_sf64_i32.
// Because the C helper is `static` (no linker symbol), its logic is
// reproduced here as private Rust functions.

use core::ffi::c_int;

// Mirror of the static C helper rshift_rnd64().
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            (a != 0) as u64
        } else if d > 0 {
            let dd = d as u32;
            let mask: u64 = (1u64 << dd) - 1;
            (a >> dd) | (((a & mask) != 0) as u64)
        } else {
            // not reachable from the ported code (C would be undefined here)
            a
        }
    } else {
        a
    }
}

#[no_mangle]
pub extern "C" fn cvt_sf64_i32__part3370(a: u64, rm: c_int, pfflags: *mut u32) -> i32 {
    // Booleans of the C logic:
    //   RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    //   FFLAG_INVALID_OP = 1 << 4, FFLAG_INEXACT = 1 << 0
    const FF_INVALID_OP: u32 = 1 << 4;
    const FF_INEXACT: u32 = 1 << 0;

    unsafe {
        let mut a_sign: u32 = (a >> (64 - 1)) as u32;
        let mut a_exp: i32 = ((a >> 52) & ((1i32 << 11) - 1) as u64) as i32;
        let mut a_mant: u64 = a & ((1u64 << 52) - 1);

        if a_exp == (1 << 11) - 1 && a_mant != 0 {
            a_sign = 0; // NaN is like +infinity
        }
        if a_exp == 0 {
            a_exp = 1;
        } else {
            a_mant |= 1u64 << 52;
        }
        a_mant <<= (64 - 2) - 52;
        a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

        // is_unsigned == FALSE
        let r_max: u32 = (1u32 << (32 - 1)) - (a_sign ^ 1);

        if a_exp >= 0 {
            // The guarded fast path needs (32 - 1 - 52) = -21 >= a_exp >= 0,
            // which is impossible, so control always reaches the overflow label.
            if a_exp <= (32 - 1 - 52) {
                let r: u32 = ((a_mant >> ((64 - 2) - 52)) as u32) << (a_exp as u32);
                if r > r_max {
                    *pfflags |= FF_INVALID_OP;
                    return r_max as i32;
                }
                let r = if a_sign != 0 { r.wrapping_neg() } else { r };
                return r as i32;
            } else {
                *pfflags |= FF_INVALID_OP;
                return r_max as i32;
            }
        } else {
            a_mant = rshift_rnd64(a_mant, -a_exp);

            let addend: u32 = if rm == 0 || rm == 4 {
                1u32 << (((64 - 2) - 52) - 1)
            } else if rm == 1 {
                0
            } else {
                // RM_RDN, RM_RUP and the switch default
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            };

            let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
            a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

            // half way: select even result
            if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
                a_mant &= !1u64;
            }
            if a_mant > r_max as u64 {
                *pfflags |= FF_INVALID_OP;
                return r_max as i32;
            }
            let mut r: u32 = a_mant as u32;
            if rnd_bits != 0 {
                *pfflags |= FF_INEXACT;
            }
            if a_sign != 0 {
                r = r.wrapping_neg();
            }
            return r as i32;
        }
    }
}
}

// ── `return internal_cvt_sf64_i64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3392 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i64(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3392 {
// Port of the body of the static C helper internal_cvt_sf64_i64(a, rm, pfflags, FALSE)
// plus the tail call that cvt_sf64_i64 performs.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// Port of the static C helper rshift_rnd64 (no linker symbol).
#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut r = a;
    if d != 0 {
        if d >= 64 {
            r = if r != 0 { 1 } else { 0 };
        } else {
            let mask: u64 = (1u64 << d) - 1;
            r = (r >> d) | if (r & mask) != 0 { 1 } else { 0 };
        }
    }
    r
}

// Port of static internal_cvt_sf64_i64 with is_unsigned == FALSE (0).
fn internal_cvt_sf64_i64(a: u64, rm: c_int, pfflags: *mut u32) -> i64 {
    let mut a_sign: u32 = (a >> 63) as u32;
    let mut a_exp: i32 = ((a >> 52) & 0x7ff) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == 0x7ff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= 10; // (64 - 2) - 52
    a_exp = a_exp - 1023 - 52;

    // is_unsigned is FALSE here
    let r_max: u64 = (1u64 << 63) - (a_sign ^ 1) as u64;

    let mut r: u64;
    if a_exp >= 0 {
        if a_exp <= 11 {
            r = (a_mant >> 10).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4; // FFLAG_INVALID_OP
                }
                return r_max as i64;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4; // FFLAG_INVALID_OP
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

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

        let rnd_bits: u32 = (a_mant & ((1u64 << 10) - 1)) as u32;
        a_mant = (a_mant.wrapping_add(addend as u64)) >> 10;

        if rm == RM_RNE && rnd_bits == (1 << 9) {
            a_mant &= !1u64;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= 1 << 4; // FFLAG_INVALID_OP
            }
            return r_max as i64;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0; // FFLAG_INEXACT
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}

#[no_mangle]
pub extern "C" fn cvt_sf64_i64__part3392(a: u64, rm: c_int, pfflags: *mut u32) -> i64 {
    internal_cvt_sf64_i64(a, rm, pfflags)
}
}

// ── `return internal_cvt_sf64_i128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3414 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i128(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3414 {
// Port of the static C helper internal_cvt_sf64_i128() for the
// is_unsigned == FALSE call made by cvt_sf64_i128().

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

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

#[no_mangle]
pub extern "C" fn cvt_sf64_i128__part3414(a: u64, rm: c_int, pfflags: *mut u32) -> i128 {
    let mut a_sign: u32 = (a >> 63) as u32;
    let mut a_exp: i32 = ((a >> 52) & 0x7ff) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == 0x7ff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= 10; // (64 - 2) - 52
    a_exp = a_exp - 1023 - 52;

    // is_unsigned == FALSE
    let r_max: u128 = (1u128 << 127) - ((a_sign ^ 1) as u128);

    let mut r: u128;
    if a_exp >= 0 {
        if a_exp <= 75 {
            r = ((a_mant >> 10) as u128) << (a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max as i128;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = if rm == RM_RNE || rm == RM_RMM {
            1u32 << 9
        } else if rm == RM_RTZ {
            0
        } else if (a_sign ^ ((rm as u32) & 1)) != 0 {
            (1u32 << 10) - 1
        } else {
            0
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << 10) - 1)) as u32;
        a_mant = (a_mant + addend as u64) >> 10;

        if rm == RM_RNE && rnd_bits == (1u32 << 9) {
            a_mant &= !1u64;
        }
        if (a_mant as u128) > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i128;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}
}

// ── `return internal_cvt_sf128_i32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3436 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i32(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3436 {
// Port of the call target `internal_cvt_sf128_i32(a, rm, pfflags, FALSE)`
// inside cvt_sf128_i32 (softfp_template_icvt.h lines 113-114).
//
// internal_cvt_sf128_i32 and rshift_rnd128 are file-static in the C
// translation unit (no linker symbol), so their logic is reproduced here as
// private Rust functions.

use core::ffi::c_int;

const FALSE: c_int = 0;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    // d == 0 keeps `a` unchanged below; d < 0 is undefined in C, treat as no-op.
    if d <= 0 {
        return a;
    }
    let d = d as u32;
    if d >= 128 {
        return if a != 0 { 1 } else { 0 };
    }
    let mask: u128 = (1u128 << d) - 1;
    (a >> d) | (if a & mask != 0 { 1 } else { 0 })
}

unsafe fn overflow_result(pfflags: *mut u32, r_max: u32) -> i32 {
    if !pfflags.is_null() {
        *pfflags |= 1u32 << 4;
    }
    r_max as i32
}

// static int32_t internal_cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
//                                       uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_sf128_i32(a: u128, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> i32 {
    let mut a_sign: u32 = (a >> 127) as u32;
    let mut a_exp: i32 = ((a >> 112) & 0x7fffu128) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == 0x7fff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant = a_mant.wrapping_shl(14); // <<= ((128 - 2) - 112)
    a_exp = a_exp.wrapping_sub(16383).wrapping_sub(112); // - ((1<<15)-1)/2 - 112

    let r_max: u32 = if is_unsigned != 0 {
        a_sign.wrapping_sub(1)
    } else {
        (1u32 << 31).wrapping_sub(a_sign ^ 1)
    };

    let mut r: u32 = 0;

    if a_exp >= 0 {
        // (32 - 1 - 112) == -81, so this arm is effectively dead but kept
        // faithful to the C source.
        if a_exp <= (32 - 1 - 112) {
            let v: u32 = (a_mant >> 14) as u32;
            let sh = a_exp as u32;
            r = if sh < 32 { v.wrapping_shl(sh) } else { 0 };
            if r > r_max {
                return unsafe { overflow_result(pfflags, r_max) };
            }
        } else {
            return unsafe { overflow_result(pfflags, r_max) };
        }
    } else {
        a_mant = rshift_rnd128(a_mant, a_exp.wrapping_neg());

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << 13,
            RM_RTZ => 0,
            _ => {
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << 14) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u128 << 14) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u128) >> 14;

        if rm == RM_RNE && rnd_bits == (1u32 << 13) {
            a_mant &= !1u128;
        }
        if a_mant > r_max as u128 {
            return unsafe { overflow_result(pfflags, r_max) };
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            if !pfflags.is_null() {
                unsafe {
                    *pfflags |= 1u32 << 0;
                }
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i32__part3436(a: u128, rm: c_int, pfflags: *mut u32) -> i32 {
    internal_cvt_sf128_i32(a, rm, pfflags, FALSE)
}
}

// ── `return internal_cvt_sf128_i64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3458 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i64(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3458 {
// Port of the TARGET: the body call of cvt_sf128_i64, i.e. a private Rust port of
// the static C helper internal_cvt_sf128_i64(a, rm, pfflags, FALSE).
#![allow(improper_ctypes)]

use core::ffi::c_int;

// static C helper: no linker symbol, so its logic lives here as a private fn.
fn rshift_rnd128(mut a: u128, d: i32) -> u128 {
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

fn internal_cvt_sf128_i64(a: u128, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> i64 {
    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    let mut a_sign: u32 = (a >> 127) as u32;
    let mut a_exp: i32 = ((a >> 112) & 0x7fffu128) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);
    if a_exp == 0x7fff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= 14; // ((128 - 2) - 112)
    a_exp = a_exp - 16383 - 112; // (((1 << 15) - 1) / 2) + 112

    let r_max: u64 = if is_unsigned {
        (a_sign as u64).wrapping_sub(1)
    } else {
        (1u64 << 63) - ((a_sign ^ 1) as u64)
    };

    let mut r: u64 = 0;

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 112) {
            let v: u64 = (a_mant >> 14) as u64;
            r = if a_exp < 64 {
                v.wrapping_shl(a_exp as u32)
            } else {
                0
            };
            if r > r_max {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return r_max as i64;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32;
        match rm {
            0 | 4 => {
                // RM_RNE, RM_RMM
                addend = 1u32 << 13;
            }
            1 => {
                // RM_RTZ
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP (and default)
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    addend = (1u32 << 14) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        let rnd_bits: u32 = (a_mant & ((1u32 << 14) as u128 - 1)) as u32;
        a_mant = (a_mant + addend as u128) >> 14;

        if rm == 0 && rnd_bits == (1u32 << 13) {
            a_mant &= !1u128;
        }
        if a_mant > r_max as u128 {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max as i64;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i64__part3458(a: u128, rm: c_int, pfflags: *mut u32) -> i64 {
    internal_cvt_sf128_i64(a, rm, pfflags, false)
}
}

// ── `return internal_cvt_sf128_i128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:113-114 · chunk 3480 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i128(a, rm,
//                                                                         pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3480 {
// Port of cvt_sf128_i128 -> internal_cvt_sf128_i128(a, rm, pfflags, FALSE)
// (internal_cvt_sf128_i128 and rshift_rnd128 are static in C, so their logic
//  lives here as private Rust functions).
use core::ffi::c_int;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d > 0 {
            let mask: u128 = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u128;
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf128_i128__part3480(a: u128, rm: c_int, pfflags: *mut u32) -> i128 {
    let mut a_sign: u32 = (a >> 127) as u32;
    let mut a_exp: i32 = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == ((1i32 << 15) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= 14;
    a_exp = a_exp - ((1i32 << 15) - 1) / 2 - 112;

    // is_unsigned == FALSE
    let r_max: u128 = (1u128 << 127) - ((a_sign ^ 1) as u128);

    let mut r: u128;

    if a_exp >= 0 {
        if a_exp <= 15 {
            r = (a_mant >> 14) << (a_exp as u32);
            if r > r_max {
                // overflow:
                if !pfflags.is_null() {
                    unsafe {
                        *pfflags |= 1u32 << 4;
                    }
                }
                return r_max as i128;
            }
        } else {
            // overflow:
            if !pfflags.is_null() {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            return r_max as i128;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32;
        if rm == 0 || rm == 4 {
            // RM_RNE, RM_RMM
            addend = 1u32 << 13;
        } else if rm == 1 {
            // RM_RTZ
            addend = 0;
        } else {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 14) - 1;
            } else {
                addend = 0;
            }
        }

        let rnd_bits: u32 = (a_mant & ((1u128 << 14) - 1)) as u32;
        a_mant = (a_mant + (addend as u128)) >> 14;

        if rm == 0 && rnd_bits == (1u32 << 13) {
            a_mant &= !1u128;
        }
        if a_mant > r_max {
            // overflow:
            if !pfflags.is_null() {
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
            }
            return r_max as i128;
        }
        r = a_mant;
        if rnd_bits != 0 {
            if !pfflags.is_null() {
                unsafe {
                    *pfflags |= 1u32 << 0;
                }
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i128
}
}

// ── `cvt_sf32_u32` · softfp_template_icvt.h:117-122 · chunk 3305 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_sf32_u32(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i32 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3305 {
// Port of cvt_sf32_u32 (softfp_template_icvt.h), together with the static
// helpers it uses (internal_cvt_sf32_i32 with is_unsigned = TRUE and
// rshift_rnd32), which have no linker symbols of their own.

use core::ffi::c_int;

// RoundingModeEnum values (C enum -> int).
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

#[inline]
fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

unsafe fn internal_cvt_sf32_i32(
    a: u32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: bool,
) -> i32 {
    let mut a_sign: u32 = a >> (32 - 1);
    let mut a_exp: i32 = ((a >> 23) & ((1i32 << 8) - 1).unsigned_abs()) as i32;
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);

    if a_exp == ((1 << 8) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;

    let r_max: u32 = if is_unsigned {
        a_sign.wrapping_sub(1)
    } else {
        (1u32 << (32 - 1)).wrapping_sub(a_sign ^ 1)
    };

    let mut r: u32;
    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 23) {
            r = (a_mant >> ((32 - 2) - 23)) << (a_exp as u32);
            if r > r_max {
                // overflow:
                *pfflags |= 1 << 4;
                return r_max as i32;
            }
        } else {
            // overflow:
            *pfflags |= 1 << 4;
            return r_max as i32;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = if rm == RM_RNE || rm == 4 {
            // RM_RNE, RM_RMM
            1u32 << (((32 - 2) - 23) - 1)
        } else if rm == RM_RTZ {
            0
        } else {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if a_mant > r_max {
            // overflow:
            *pfflags |= 1 << 4;
            return r_max as i32;
        }
        r = a_mant;
        if rnd_bits != 0 {
            *pfflags |= 1 << 0;
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub unsafe extern "C" fn cvt_sf32_u32(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_sf32_i32(a, rm, pfflags, true) as u32
}
}

// ── `cvt_sf32_u64` · softfp_template_icvt.h:117-122 · chunk 3327 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_sf32_u64(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i64 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3327 {
// Port of softfloat-style helper:
//   uint64_t cvt_sf32_u64(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags)
// which simply forwards to the static internal_cvt_sf32_i64(a, rm, pfflags, TRUE).

use core::ffi::{c_int, c_uint};

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << (d as u32)).wrapping_sub(1);
            a = (a >> (d as u32)) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf32_u64(a: c_uint, rm: c_int, pfflags: *mut c_uint) -> u64 {
    let a: u32 = a;
    let mut a_sign: u32 = a >> 31;
    let mut a_exp: i32 = ((a >> 23) & 0xff) as i32;
    let mut a_mant: u32 = a & 0x007f_ffff;

    if a_exp == 0xff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= 7; // (32 - 2) - 23
    a_exp = a_exp - 127 - 23; // ((1 << 8) - 1) / 2 == 127

    // is_unsigned == TRUE
    let r_max: u64 = (a_sign as u64).wrapping_sub(1);

    let mut r: u64;

    if a_exp >= 0 {
        if a_exp <= 40 {
            r = ((a_mant >> 7) as u64) << (a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return r_max;
            }
        } else {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1u32 << 6, // RM_RNE, RM_RMM
            1 => 0,             // RM_RTZ
            _ => {
                // RM_RDN, RM_RUP, default
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << 7) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << 7) - 1);
        a_mant = a_mant.wrapping_add(addend) >> 7;

        if rm == 0 && rnd_bits == (1u32 << 6) {
            a_mant &= !1u32;
        }
        if (a_mant as u64) > r_max {
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 0;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `cvt_sf32_u128` · softfp_template_icvt.h:117-122 · chunk 3349 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_sf32_u128(uint32_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf32_i128 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3349 {
// Port of uint128_t cvt_sf32_u128(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags)
// which forwards to the static internal_cvt_sf32_i128(a, rm, pfflags, TRUE).

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

#[inline]
fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
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

unsafe fn internal_cvt_sf32_i128(
    a: u32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: bool,
) -> u128 {
    let mut a_sign: u32 = a >> (32 - 1);
    let mut a_exp: i32 = ((a >> 23) & ((1u32 << 8) - 1)) as i32;
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);

    if a_exp == ((1i32 << 8) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - ((1i32 << 8) - 1) / 2 - 23;

    let r_max: u128;
    if is_unsigned {
        r_max = (a_sign as u128).wrapping_sub(1);
    } else {
        r_max = (1u128 << (128 - 1)) - ((a_sign ^ 1) as u128);
    }

    let mut r: u128;

    if a_exp >= 0 {
        if a_exp <= 128 - 1 - 23 {
            r = ((a_mant >> ((32 - 2) - 23)) as u128) << (a_exp as u32);
            if r > r_max {
                *pfflags |= 1 << 4;
                return r_max;
            }
        } else {
            *pfflags |= 1 << 4;
            return r_max;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((32 - 2) - 23) - 1),
            RM_RTZ => 0,
            _ => {
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << ((32 - 2) - 23)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if (a_mant as u128) > r_max {
            *pfflags |= 1 << 4;
            return r_max;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            *pfflags |= 1 << 0;
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub unsafe extern "C" fn cvt_sf32_u128(a: u32, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_sf32_i128(a, rm, pfflags, true)
}
}

// ── `cvt_sf64_u32` · softfp_template_icvt.h:117-122 · chunk 3371 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_sf64_u32(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i32 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3371 {
// Port of cvt_sf64_u32 (softfp_template_icvt.h lines 117-122).
// The static helper internal_cvt_sf64_i32 (lines 4891-4961 of the preprocessed
// translation unit) has no linker symbol, so its logic is inlined here.

use core::ffi::c_int;

// RoundingModeEnum values
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

// static uint64_t rshift_rnd64(uint64_t a, int d)
#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            (a != 0) as u64
        } else {
            let mask = (1u64 << (d as u32)) - 1;
            (a >> (d as u32)) | ((a & mask) != 0) as u64
        }
    } else {
        a
    }
}

// static int32_t internal_cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
//                                      uint32_t *pfflags, BOOL is_unsigned)
#[inline]
unsafe fn internal_cvt_sf64_i32(
    a: u64,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> i32 {
    // 64 - 2 - 52 == 10 for sfloat64 -> 32 bit integer conversions
    let mut a_sign: u32 = (a >> 63) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

    let r_max: u32 = if is_unsigned != 0 {
        (a_sign as u32).wrapping_sub(1)
    } else {
        (1u32 << (32 - 1)) - (a_sign ^ 1)
    };

    let mut r: u32;

    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 52) {
            r = ((a_mant >> ((64 - 2) - 52)) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                *pfflags |= 1 << 4;
                return r_max as i32;
            }
        } else {
            *pfflags |= 1 << 4;
            return r_max as i32;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            RM_RNE | RM_RMM => 1u32 << (((64 - 2) - 52) - 1),
            RM_RTZ => 0,
            _ => {
                // RM_RDN, RM_RUP and the C default case
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & (((1u64 << ((64 - 2) - 52)) - 1) as u64)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

        if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > (r_max as u64) {
            *pfflags |= 1 << 4;
            return r_max as i32;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            *pfflags |= 1 << 0;
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub unsafe extern "C" fn cvt_sf64_u32(a: u64, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_sf64_i32(a, rm, pfflags, 1) as u32
}
}

// ── `cvt_sf64_u64` · softfp_template_icvt.h:117-122 · chunk 3393 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_sf64_u64(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i64 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3393 {
// Port of `cvt_sf64_u64` (softfp_template_icvt.h) and of the static helper
// `internal_cvt_sf64_i64` it forwards to, specialized for is_unsigned = TRUE.

use core::ffi::c_int;

// F_SIZE = 64, ICVT_SIZE = 64
const MANT_SHIFT: i32 = (64 - 2) - 52; // 10
const MANT_BITS: u32 = 52;

#[inline]
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u64 = (1u64 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn internal_cvt_sf64_i64(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> MANT_BITS) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << MANT_BITS) - 1);

    if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << MANT_BITS;
    }
    a_mant <<= MANT_SHIFT as u32;
    a_exp = a_exp - (((1i32 << 11) - 1) / 2) - 52;

    // is_unsigned == TRUE
    let r_max: u64 = (a_sign as u64).wrapping_sub(1);

    let mut r: u64;

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 52) {
            r = (a_mant >> MANT_SHIFT as u32) << (a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            // RM_RNE, RM_RMM
            0 | 4 => 1u32 << (MANT_SHIFT - 1),
            // RM_RTZ
            1 => 0,
            // default: RM_RDN, RM_RUP
            _ => {
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u32 << MANT_SHIFT) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << MANT_SHIFT) - 1)) as u32;
        a_mant = (a_mant.wrapping_add(addend as u64)) >> (MANT_SHIFT as u32);

        if rm == 0 && rnd_bits == (1u32 << (MANT_SHIFT - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max {
            // overflow:
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub extern "C" fn cvt_sf64_u64(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_sf64_i64(a, rm, pfflags)
}
}

// ── `cvt_sf64_u128` · softfp_template_icvt.h:117-122 · chunk 3415 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_sf64_u128(uint64_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf64_i128 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3415 {
// Port of softfp_template_icvt.h cvt_sf64_u128, which is a thin wrapper around
// the static helper internal_cvt_sf64_i128(a, rm, pfflags, TRUE).  The helper has
// no linker symbol, so its logic (with is_unsigned = TRUE) is inlined here.

use core::ffi::c_int;

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u64 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

// static int128_t internal_cvt_sf64_i128(..., BOOL is_unsigned) with is_unsigned == TRUE
unsafe fn internal_cvt_sf64_i128_u(a: u64, rm: c_int, pfflags: *mut u32) -> u128 {
    // F_SIZE = 64, so F_NBITS = 11 and F_MANT_SIZE = 52.
    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

    // is_unsigned == TRUE
    let r_max: u128 = (a_sign as u128).wrapping_sub(1);

    let mut r: u128;
    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 52) {
            r = ((a_mant >> ((64 - 2) - 52)) as u128) << (a_exp as u32);
            if r > r_max {
                // goto overflow
                unsafe { *pfflags |= 1 << 4 };
                return r_max;
            }
        } else {
            // overflow:
            unsafe { *pfflags |= 1 << 4 };
            return r_max;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u64 = match rm {
            0 | 4 => 1u64 << (((64 - 2) - 52) - 1), // RM_RNE, RM_RMM
            1 => 0,                                 // RM_RTZ
            _ => {
                // default: RM_RDN, RM_RUP
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    (1u64 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend) >> ((64 - 2) - 52);

        if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if (a_mant as u128) > r_max {
            // goto overflow
            unsafe { *pfflags |= 1 << 4 };
            return r_max;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe { *pfflags |= 1 << 0 };
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub unsafe extern "C" fn cvt_sf64_u128(a: u64, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_sf64_i128_u(a, rm, pfflags)
}
}

// ── `cvt_sf128_u32` · softfp_template_icvt.h:117-122 · chunk 3437 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_sf128_u32(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i32 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3437 {
// Port of softfp_template_icvt.h: cvt_sf128_u32().
//
// The C version forwards to the file-static helper
// internal_cvt_sf128_i32(a, rm, pfflags, TRUE).  A static C function has no
// linker symbol, so the helper logic is reproduced here with is_unsigned = TRUE.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

// SOFTFLOAT_EXP_BITS = 15, SOFTFLOAT_FRAC_BITS = 112 for sf128.
const EXP_MAX: i32 = (1 << 15) - 1; // 0x7fff
const EXP_BIAS: i32 = ((1 << 15) - 1) / 2; // 16383
const FRAC_BITS: i32 = 112;
const GUARD: i32 = (128 - 2) - 112; // 14, number of extra low bits

// static uint128_t rshift_rnd128(uint128_t a, int d)
#[inline]
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            (a != 0) as u128
        } else {
            let mask = (1u128 << d) - 1;
            (a >> d) | (((a & mask) != 0) as u128)
        }
    } else {
        a
    }
}

#[no_mangle]
pub extern "C" fn cvt_sf128_u32(a: u128, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_sign: u32 = (a >> (128 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == EXP_MAX && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= GUARD as u32;
    a_exp = a_exp - EXP_BIAS - 112;

    // is_unsigned == TRUE
    let r_max: u32 = a_sign.wrapping_sub(1);

    if a_exp >= 0 {
        // The C code only stays on the non-overflow path when
        // a_exp <= (32 - 1 - 112) == -81, which cannot happen here, so control
        // always reaches the `overflow` label.
        unsafe {
            *pfflags |= FFLAG_INVALID;
        }
        return r_max;
    }

    a_mant = rshift_rnd128(a_mant, -a_exp);

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (GUARD - 1),
        RM_RTZ => 0,
        _ => {
            // RM_RDN / RM_RUP and anything else
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << GUARD) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32 = (a_mant & ((1u128 << GUARD) - 1)) as u32;
    a_mant = a_mant.wrapping_add(addend as u128) >> GUARD;

    if rm == RM_RNE && rnd_bits == (1u32 << (GUARD - 1)) {
        a_mant &= !1u128;
    }
    if a_mant > r_max as u128 {
        unsafe {
            *pfflags |= FFLAG_INVALID;
        }
        return r_max;
    }
    let mut r: u32 = a_mant as u32;
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `cvt_sf128_u64` · softfp_template_icvt.h:117-122 · chunk 3459 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_sf128_u64(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i64 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3459 {
// Port of cvt_sf128_u64 (softfp_template_icvt.h).
// The C wrapper only calls the static helper internal_cvt_sf128_i64 with
// is_unsigned = TRUE, so that helper (and its static helper rshift_rnd128)
// are inlined here as private Rust functions.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const TRUE: c_int = 1;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_INVALID: u32 = 1 << 4;

fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let shift = d as u32;
            let mask: u128 = (1u128 << shift) - 1;
            a = (a >> shift) | ((a & mask != 0) as u128);
        }
    }
    a
}

fn internal_cvt_sf128_i64(a: u128, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> i64 {
    let mut a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let mut r: u64;
    let r_max: u64;
    let mut addend: u32;
    let mut rnd_bits: u32;

    a_sign = (a >> (128 - 1)) as u32;
    a_exp = ((a >> 112) & ((1i32 << 15) - 1) as u128) as i32;
    a_mant = a & ((1u128 << 112) - 1);
    if a_exp == (1 << 15) - 1 && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= (128 - 2) - 112;
    a_exp = a_exp - (((1i32 << 15) - 1) / 2) - 112;

    if is_unsigned != 0 {
        r_max = (a_sign as u64).wrapping_sub(1);
    } else {
        r_max = (1u64 << (64 - 1)) - ((a_sign ^ 1) as u64);
    }

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 112) {
            r = ((a_mant >> ((128 - 2) - 112)) as u64).wrapping_shl(a_exp as u32);
            if (r as u128) > (r_max as u128) {
                // overflow:
                unsafe {
                    *pfflags |= FFLAG_INVALID;
                }
                return r_max as i64;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        match rm {
            RM_RNE | RM_RMM => {
                addend = 1u32 << (((128 - 2) - 112) - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                if (a_sign as i32 ^ (rm & 1)) != 0 {
                    addend = (1u32 << ((128 - 2) - 112)) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        a_mant = (a_mant + addend as u128) >> ((128 - 2) - 112);

        if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > r_max as u128 {
            // overflow:
            unsafe {
                *pfflags |= FFLAG_INVALID;
            }
            return r_max as i64;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_INEXACT;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i64
}

#[no_mangle]
pub extern "C" fn cvt_sf128_u64(a: u128, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_sf128_i64(a, rm, pfflags, TRUE) as u64
}
}

// ── `cvt_sf128_u128` · softfp_template_icvt.h:117-122 · chunk 3481 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_sf128_u128(uint128_t a, RoundingModeEnum rm,
//                                                               uint32_t *pfflags)
//     {
//         return internal_cvt_sf128_i128 (a, rm,
//                                                                          pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3481 {
// Port of softfp_template_icvt.h: cvt_sf128_u128
// The C version calls the file-static helper internal_cvt_sf128_i128 with
// is_unsigned = TRUE, plus the file-static helper rshift_rnd128.
// Both statics are inlined here as private Rust functions.
//
// RoundingModeEnum (C enum, int-sized):
//   RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
// FFLAG_INEXACT = (1 << 0), FFLAG_INVALID = (1 << 4)

use core::ffi::c_int;

// d >= 0 here (called with -a_exp where a_exp < 0)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut r = a;
    if d != 0 {
        if d >= 128 {
            r = if a != 0 { 1 } else { 0 };
        } else {
            let sh = d as u32;
            let mask: u128 = (1u128 << sh) - 1;
            r = (a >> sh) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    r
}

fn internal_cvt_sf128_i128(a: u128, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u128 {
    let mut a_sign: u32 = (a >> (128 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112) - 1);

    if a_exp == ((1 << 15) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= (128 - 2) - 112;
    a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;

    let r_max: u128 = if is_unsigned {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << (128 - 1)).wrapping_sub((a_sign ^ 1) as u128)
    };

    let mut r: u128;

    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 112) {
            r = (a_mant >> ((128 - 2) - 112)).wrapping_shl(a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => 1 << (((128 - 2) - 112) - 1),
            1 => 0,
            _ => {
                // RM_RDN, RM_RUP and the C default label share this code
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((128 - 2) - 112)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        a_mant = (a_mant.wrapping_add(addend as u128)) >> ((128 - 2) - 112);

        if rm == 0 && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub extern "C" fn cvt_sf128_u128(a: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_sf128_i128(a, rm, pfflags, true)
}
}

// ── `return internal_cvt_sf32_i32 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3306 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i32 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3306 {
// Rust port of the TARGET statement inside C function cvt_sf32_u32:
//     return internal_cvt_sf32_i32(a, rm, pfflags, TRUE);
//
// internal_cvt_sf32_i32 is a `static` C function (no linker symbol), so its
// logic -- and that of the `static inline` helper rshift_rnd32 -- is ported
// here as private Rust functions, with is_unsigned fixed to TRUE.

use core::ffi::c_int;

// RoundingModeEnum values (RM_RNE, RM_RTZ, RM_RDN, RM_RUP, RM_RMM)
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

/// uint32_t rshift_rnd32(uint32_t a, int d)  (sticky-bit right shift)
#[inline]
fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u32 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[inline]
unsafe fn set_flag(pfflags: *mut u32, bits: u32) {
    if !pfflags.is_null() {
        *pfflags |= bits;
    }
}

#[no_mangle]
pub extern "C" fn cvt_sf32_u32__part3306(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    // is_unsigned == TRUE for this instantiation of internal_cvt_sf32_i32.
    let mut a_sign: u32 = a >> (32 - 1);
    let mut a_exp: i32 = ((a >> 23) & ((1 << 8) - 1)) as i32;
    let mut a_mant: u32 = a & (((1u32 << 23) - 1) as u32);

    if a_exp == ((1 << 8) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;

    // is_unsigned:  r_max = (uint32_t)a_sign - 1;
    let r_max: u32 = a_sign.wrapping_sub(1);

    let mut r: u32 = 0;
    let mut overflow: bool = false;

    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 23) {
            r = (a_mant >> ((32 - 2) - 23)) << a_exp;
            if r > r_max {
                overflow = true; // goto overflow
            }
        } else {
            overflow = true; // goto overflow
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        // switch(rm) { case RM_RNE: case RM_RMM: ... ; case RM_RTZ: ... ;
        //              default: case RM_RDN: case RM_RUP: ... }
        let addend: u32 = if rm == RM_RNE || rm == RM_RMM {
            1u32 << (((32 - 2) - 23) - 1)
        } else if rm == RM_RTZ {
            0
        } else if (a_sign ^ ((rm as u32) & 1)) != 0 {
            (1u32 << ((32 - 2) - 23)) - 1
        } else {
            0
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if a_mant > r_max {
            overflow = true; // goto overflow
        } else {
            r = a_mant;
            if rnd_bits != 0 {
                unsafe { set_flag(pfflags, 1 << 0) };
            }
        }
    }

    // overflow: *pfflags |= (1 << 4); return r_max;
    if overflow {
        unsafe { set_flag(pfflags, 1 << 4) };
        return r_max;
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `return internal_cvt_sf32_i64 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3328 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i64 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3328 {
// Port of the TARGET: `return internal_cvt_sf32_i64(a, rm, pfflags, TRUE);`
// internal_cvt_sf32_i64 is a `static` C function (no linker symbol), so its
// logic (unsigned variant, is_unsigned == TRUE) is ported here, together with
// the static helper rshift_rnd32.

use core::ffi::c_int;

// static uint32_t rshift_rnd32(uint32_t a, int d)
unsafe fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << (d as u32)).wrapping_sub(1);
            let sticky: u32 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> (d as u32)) | sticky;
        }
    }
    a
}

// internal_cvt_sf32_i64(a, rm, pfflags, TRUE)
unsafe fn internal_cvt_sf32_i64_unsigned(a: u32, rm: c_int, pfflags: *mut u32) -> u64 {
    let addend: u32;
    let rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let mut r: u64 = 0;
    let r_max: u64;

    // a_sign = a >> (32 - 1);
    let mut a_sign: u32 = a >> 31;
    // a_exp = (a >> 23) & ((1 << 8) - 1);
    a_exp = ((a >> 23) & 0xff) as i32;
    // a_mant = a & (((uint32_t)1 << 23) - 1);
    a_mant = a & ((1u32 << 23) - 1);
    if a_exp == 0xff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    // a_mant <<= ((32 - 2) - 23);
    a_mant <<= 7;
    // a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;
    a_exp = a_exp - 127 - 23;

    // is_unsigned: r_max = (uint64_t)a_sign - 1;
    r_max = (a_sign as u64).wrapping_sub(1);

    if a_exp >= 0 {
        let mut overflow = false;
        if a_exp <= 40 {
            // r = (uint64_t)(a_mant >> ((32 - 2) - 23)) << a_exp;
            r = ((a_mant >> 7) as u64) << (a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
        if overflow {
            // overflow:
            *pfflags |= 1 << 4;
            return r_max;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        if rm == 0 || rm == 4 {
            // case RM_RNE: case RM_RMM:
            addend = 1u32 << 6;
        } else if rm == 1 {
            // case RM_RTZ:
            addend = 0;
        } else {
            // default: case RM_RDN: case RM_RUP:
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }

        // rnd_bits = a_mant & ((1 << ((32 - 2) - 23)) - 1);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        // a_mant = (a_mant + addend) >> ((32 - 2) - 23);
        a_mant = a_mant.wrapping_add(addend) >> 7;

        // if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)))
        //     a_mant &= ~1;
        if rm == 0 && rnd_bits == (1u32 << 6) {
            a_mant &= !1u32;
        }
        if (a_mant as u64) > r_max {
            // goto overflow;
            *pfflags |= 1 << 4;
            return r_max;
        }
        r = a_mant as u64;
        if rnd_bits != 0 {
            *pfflags |= 1 << 0;
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub extern "C" fn cvt_sf32_u64__part3328(a: u32, rm: c_int, pfflags: *mut u32) -> u64 {
    unsafe { internal_cvt_sf32_i64_unsigned(a, rm, pfflags) }
}
}

// ── `return internal_cvt_sf32_i128 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3350 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf32_i128 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3350 {
// Rust port of the TARGET inside cvt_sf32_u128:
//   return internal_cvt_sf32_i128(a, rm, pfflags, TRUE);
// internal_cvt_sf32_i128 is a `static` C function (no linker symbol), so its
// logic is ported here as a private Rust fn. The exported function keeps the
// exact C symbol name requested and the ABI of the call site.

use core::ffi::{c_int, c_uint};

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: c_int) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

// static int128_t internal_cvt_sf32_i128(uint32_t a, RoundingModeEnum rm,
//                                        uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_sf32_i128(a: u32, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> u128 {
    let mut a_sign: u32 = a >> (32 - 1);
    let mut a_exp: c_int = ((a >> 23) & ((1u32 << 8) - 1)) as c_int;
    let mut a_mant: u32 = a & ((1u32 << 23) - 1);

    if a_exp == ((1 << 8) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u32 << 23;
    }
    a_mant <<= (32 - 2) - 23;
    a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;

    let r_max: u128 = if is_unsigned != 0 {
        (a_sign as u128).wrapping_sub(1)
    } else {
        (1u128 << (128 - 1)) - ((a_sign ^ 1) as u128)
    };

    let mut r: u128;

    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 23) {
            r = ((a_mant >> ((32 - 2) - 23)) as u128) << (a_exp as u32);
            if r > r_max {
                // goto overflow;
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd32(a_mant, -a_exp);

        // switch (rm) { ... }
        let addend: u32 = if rm == 0 || rm == 4 {
            // RM_RNE, RM_RMM
            1u32 << (((32 - 2) - 23) - 1)
        } else if rm == 1 {
            // RM_RTZ
            0
        } else {
            // RM_RDN, RM_RUP (and default)
            if (a_sign ^ ((rm as c_uint) & 1)) != 0 {
                (1u32 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        };

        let rnd_bits: u32 = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

        if rm == 0 && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
            a_mant &= !1u32;
        }
        if (a_mant as u128) > r_max {
            // goto overflow;
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
        r = a_mant as u128;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}

#[no_mangle]
pub extern "C" fn cvt_sf32_u128__part3350(a: u32, rm: c_int, pfflags: *mut u32) -> u128 {
    // TRUE
    internal_cvt_sf32_i128(a, rm, pfflags, 1)
}
}

// ── `return internal_cvt_sf64_i32 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3372 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i32 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3372 {
// Port of the tail call `internal_cvt_sf64_i32(a, rm, pfflags, TRUE)`
// which lives inside cvt_sf64_u32 (softfp_template_icvt.h).
// internal_cvt_sf64_i32 is a static C function, so its logic is mirrored here.

use core::ffi::c_int;

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    if d != 0 {
        if d < 0 {
            // Unreachable for the call sites used here (C would be UB).
            return a;
        }
        if d >= 64 {
            return (a != 0) as u64;
        }
        let du = d as u32;
        let mask: u64 = (1u64 << du) - 1;
        (a >> du) | ((a & mask != 0) as u64)
    } else {
        a
    }
}

// static int32_t internal_cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
//                                      uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_sf64_i32(a: u64, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> i32 {
    let mut a_sign: u32 = (a >> (64 - 1)) as u32;
    let mut a_exp: i32 = ((a >> 52) & ((1u64 << 11) - 1)) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == ((1i32 << 11) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= (64 - 2) - 52;
    a_exp = a_exp - (((1i32 << 11) - 1) / 2) - 52;

    let r_max: u32 = if is_unsigned != 0 {
        a_sign.wrapping_sub(1)
    } else {
        (1u32 << (32 - 1)) - (a_sign ^ 1)
    };

    let mut r: u32;
    if a_exp >= 0 {
        if a_exp <= (32 - 1 - 52) {
            r = ((a_mant >> ((64 - 2) - 52)) as u32).wrapping_shl(a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max as i32;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i32;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32 = match rm {
            0 | 4 => {
                // RM_RNE, RM_RMM
                1u32 << (((64 - 2) - 52) - 1)
            }
            1 => {
                // RM_RTZ
                0
            }
            _ => {
                // default: RM_RDN, RM_RUP and anything else
                if (a_sign ^ ((rm & 1) as u32)) != 0 {
                    (1u32 << ((64 - 2) - 52)) - 1
                } else {
                    0
                }
            }
        };

        let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        a_mant = (a_mant + addend as u64) >> ((64 - 2) - 52);

        if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
            a_mant &= !1u64;
        }
        if a_mant > r_max as u64 {
            // overflow:
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i32;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub extern "C" fn cvt_sf64_u32__part3372(a: u64, rm: c_int, pfflags: *mut u32) -> u32 {
    unsafe { internal_cvt_sf64_i32(a, rm, pfflags, 1) as u32 }
}
}

// ── `return internal_cvt_sf64_i64 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3394 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i64 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3394 {
// Port of the static helper internal_cvt_sf64_i64 (called with is_unsigned = TRUE),
// together with the static helper rshift_rnd64 it uses. Neither has a linker symbol.

use core::ffi::c_int;

// RoundingModeEnum values:
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let du = d as u32;
            let mask = (1u64 << du) - 1;
            a = (a >> du) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf64_u64__part3394(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_sign: u32 = (a >> 63) as u32;
    let mut a_exp: i32 = ((a >> 52) & 0x7ff) as i32;
    let mut a_mant: u64 = a & ((1u64 << 52) - 1);

    if a_exp == 0x7ff && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u64 << 52;
    }
    a_mant <<= 10;
    a_exp = a_exp - 1023 - 52;

    // is_unsigned == TRUE
    let r_max: u64 = (a_sign as u64).wrapping_sub(1);

    let mut r: u64;
    if a_exp >= 0 {
        if a_exp <= 11 {
            r = (a_mant >> 10) << (a_exp as u32);
            if r > r_max {
                unsafe {
                    *pfflags |= 1 << 4;
                }
                return r_max;
            }
        } else {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd64(a_mant, -a_exp);

        let addend: u32;
        if rm == RM_RNE || rm == RM_RMM {
            addend = 1 << 9;
        } else if rm == RM_RTZ {
            addend = 0;
        } else {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1 << 10) - 1;
            } else {
                addend = 0;
            }
        }

        let rnd_bits: u32 = (a_mant & ((1u64 << 10) - 1)) as u32;
        a_mant = a_mant.wrapping_add(addend as u64) >> 10;

        if rm == RM_RNE && rnd_bits == (1 << 9) {
            a_mant &= !1u64;
        }
        if a_mant > r_max {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }

    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `return internal_cvt_sf64_i128 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3416 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf64_i128 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3416 {
// Port of the TARGET: return internal_cvt_sf64_i128(a, rm, pfflags, TRUE);
// internal_cvt_sf64_i128 is a static C function (no linker symbol), so its
// logic is ported here, specialised for is_unsigned == TRUE.

use core::ffi::c_uint;

// Port of the static C helper rshift_rnd64 (sticky-bit right shift).
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u64 = (1u64 << d) - 1;
            a = (a >> d) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf64_u128__part3416(a: u64, rm: c_uint, pfflags: *mut u32) -> u128 {
    unsafe {
        let mut a_sign: u32 = (a >> (64 - 1)) as u32;
        let mut a_exp: i32 = ((a >> 52) & ((1 << 11) - 1)) as i32;
        let mut a_mant: u64 = a & ((1u64 << 52) - 1);

        if a_exp == (1 << 11) - 1 && a_mant != 0 {
            a_sign = 0;
        }
        if a_exp == 0 {
            a_exp = 1;
        } else {
            a_mant |= 1u64 << 52;
        }
        a_mant <<= (64 - 2) - 52;
        a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;

        // is_unsigned is TRUE
        let r_max: u128 = (a_sign as u128).wrapping_sub(1);

        let mut r: u128 = 0;

        if a_exp >= 0 {
            if a_exp <= 128 - 1 - 52 {
                r = ((a_mant >> ((64 - 2) - 52)) as u128) << a_exp;
                if r > r_max {
                    // goto overflow
                    *pfflags |= 1u32 << 4;
                    return r_max;
                }
            } else {
                // overflow:
                *pfflags |= 1u32 << 4;
                return r_max;
            }
        } else {
            a_mant = rshift_rnd64(a_mant, -a_exp);

            let addend: u32;
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
                    // default: RM_RDN, RM_RUP and anything else
                    if (a_sign ^ (rm & 1)) != 0 {
                        addend = (1u32 << ((64 - 2) - 52)) - 1;
                    } else {
                        addend = 0;
                    }
                }
            }

            let rnd_bits: u32 = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
            a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

            if rm == 0 && (rnd_bits as u64) == (1u64 << (((64 - 2) - 52) - 1)) {
                a_mant &= !1u64;
            }
            if (a_mant as u128) > r_max {
                // goto overflow
                *pfflags |= 1u32 << 4;
                return r_max;
            }
            r = a_mant as u128;
            if rnd_bits != 0 {
                *pfflags |= 1u32;
            }
        }

        if a_sign != 0 {
            r = r.wrapping_neg();
        }
        r
    }
}
}

// ── `return internal_cvt_sf128_i32 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3438 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i32 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3438 {
// Port of `cvt_sf128_u32`'s tail call:
//   return internal_cvt_sf128_i32(a, rm, pfflags, TRUE);
// `internal_cvt_sf128_i32` and `rshift_rnd128` are `static` in C (no linker
// symbol), so their logic is reproduced here as private Rust functions.

use core::ffi::c_uint;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            // 0 < d < 128 here, so the shift amount is in range
            let mask: u128 = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (((a & mask) != 0) as u128);
        }
    }
    a
}

// static int32_t internal_cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
//                                       uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_sf128_i32(a: u128, rm: c_uint, pfflags: *mut u32, is_unsigned: bool) -> i32 {
    let mut a_sign: u32;
    let mut addend: u32;
    let mut rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let mut r: u32;
    let r_max: u32;

    a_sign = (a >> (128 - 1)) as u32;
    a_exp = ((a >> 112) & ((1u128 << 15) - 1)) as i32;
    a_mant = a & ((1u128 << 112) - 1);
    if a_exp == ((1i32 << 15) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= (128 - 2) - 112;
    a_exp = a_exp - (((1i32 << 15) - 1) / 2) - 112;

    if is_unsigned {
        r_max = a_sign.wrapping_sub(1);
    } else {
        r_max = (1u32 << (32 - 1)) - (a_sign ^ 1);
    }
    if a_exp >= 0 {
        // `a_exp <= (32 - 1 - 112)`, i.e. `a_exp <= -81`, cannot hold when
        // a_exp >= 0, so the (fall-through-into) overflow path is taken.
        unsafe {
            *pfflags |= 1 << 4;
        }
        return r_max as i32;
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        match rm {
            0 | 4 => {
                // RM_RNE, RM_RMM
                addend = 1 << (((128 - 2) - 112) - 1);
            }
            1 => {
                // RM_RTZ
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP (and default)
                if (a_sign ^ (rm & 1)) != 0 {
                    addend = (1 << ((128 - 2) - 112)) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        a_mant = (a_mant + (addend as u128)) >> ((128 - 2) - 112);

        if rm == 0 && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > (r_max as u128) {
            unsafe {
                *pfflags |= 1 << 4;
            }
            return r_max as i32;
        }
        r = a_mant as u32;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 0;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r as i32
}

#[no_mangle]
pub extern "C" fn cvt_sf128_u32__part3438(a: u128, rm: c_uint, pfflags: *mut u32) -> u32 {
    internal_cvt_sf128_i32(a, rm, pfflags, true) as u32
}
}

// ── `return internal_cvt_sf128_i64 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3460 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i64 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3460 {
// Port of cvt_sf128_u64's tail statement:
//   return internal_cvt_sf128_i64(a, rm, pfflags, TRUE);
// Both internal_cvt_sf128_i64 and rshift_rnd128 are `static` C functions, so
// their logic is reimplemented here as private Rust fns.

use core::ffi::c_int;

// Mirrors: static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask: u128 = (1u128 << du) - 1;
            a = (a >> du) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

// uint32_t bit patterns of the C RoundingModeEnum
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

#[no_mangle]
pub extern "C" fn cvt_sf128_u64__part3460(
    a: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    // internal_cvt_sf128_i64(a, rm, pfflags, TRUE)
    let mut a_sign: u32;
    let mut addend: u32;
    let rnd_bits: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let r: u64;
    let r_max: u64;

    a_sign = (a >> (128 - 1)) as u32;
    a_exp = ((a >> 112) & (((1u128 << 15) - 1) as u128)) as i32;
    a_mant = a & ((1u128 << 112) - 1);
    if a_exp == ((1 << 15) - 1) && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112;
    }
    a_mant <<= (128 - 2) - 112;
    a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;

    // is_unsigned == TRUE
    r_max = (a_sign as u64).wrapping_sub(1);

    let mut overflow = false;
    let mut r: u64 = 0;

    if a_exp >= 0 {
        if a_exp <= (64 - 1 - 112) {
            r = ((a_mant >> ((128 - 2) - 112)) as u64).wrapping_shl(a_exp as u32);
            if r > r_max {
                overflow = true;
            }
        } else {
            overflow = true;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        match rm {
            RM_RNE | RM_RMM => {
                addend = 1 << (((128 - 2) - 112) - 1);
            }
            RM_RTZ => {
                addend = 0;
            }
            _ => {
                // RM_RDN, RM_RUP and anything else
                if (a_sign ^ ((rm as u32) & 1)) != 0 {
                    addend = (1 << ((128 - 2) - 112)) - 1;
                } else {
                    addend = 0;
                }
            }
        }

        rnd_bits = (a_mant & (((1u128 << ((128 - 2) - 112)) - 1) as u128)) as u32;
        a_mant = (a_mant + (addend as u128)) >> ((128 - 2) - 112);

        if rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > (r_max as u128) {
            overflow = true;
        } else {
            r = a_mant as u64;
            if rnd_bits != 0 {
                unsafe {
                    *pfflags |= 1 << 0;
                }
            }
        }
    }

    if overflow {
        unsafe {
            *pfflags |= 1 << 4;
        }
        return r_max;
    }

    let mut r = r;
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `return internal_cvt_sf128_i128 (a, rm, pfflags, TRUE);` · softfp_template_icvt.h:120-121 · chunk 3482 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_sf128_i128 (a, rm,
//                                                                          pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3482 {
// Port of the static C helpers used by cvt_sf128_u128:
//   internal_cvt_sf128_i128(a, rm, pfflags, TRUE)  and  rshift_rnd128.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = (1u128 << (d as u32)) - 1u128;
            a = (a >> (d as u32)) | (((a & mask) != 0) as u128);
        }
    }
    a
}

#[no_mangle]
pub extern "C" fn cvt_sf128_u128__part3482(a: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    let mut a_sign: u32 = (a >> 127u32) as u32;
    let mut a_exp: i32 = ((a >> 112u32) & ((1u128 << 15u32) - 1u128)) as i32;
    let mut a_mant: u128 = a & ((1u128 << 112u32) - 1u128);

    if a_exp == (1 << 15) - 1 && a_mant != 0 {
        a_sign = 0;
    }
    if a_exp == 0 {
        a_exp = 1;
    } else {
        a_mant |= 1u128 << 112u32;
    }
    a_mant <<= (128 - 2) - 112;
    a_exp = a_exp - ((1 << 15) - 1) / 2 - 112;

    // is_unsigned == TRUE
    let r_max: u128 = (a_sign as u128).wrapping_sub(1u128);

    let mut r: u128;
    if a_exp >= 0 {
        if a_exp <= (128 - 1 - 112) {
            r = (a_mant >> ((128 - 2) - 112) as u32) << (a_exp as u32);
            if r > r_max {
                // overflow:
                unsafe {
                    *pfflags |= 1u32 << 4;
                }
                return r_max;
            }
        } else {
            // overflow:
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max;
        }
    } else {
        a_mant = rshift_rnd128(a_mant, -a_exp);

        let addend: u32;
        if rm == RM_RNE || rm == RM_RMM {
            addend = 1u32 << (((128 - 2) - 112) - 1);
        } else if rm == RM_RTZ {
            addend = 0;
        } else {
            // RM_RDN, RM_RUP and default
            let _ = RM_RDN;
            let _ = RM_RUP;
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1u32;
            } else {
                addend = 0;
            }
        }

        let rnd_bits: u32 = (a_mant & ((1u32 << ((128 - 2) - 112)) - 1u32) as u128) as u32;
        a_mant = (a_mant + addend as u128) >> ((128 - 2) - 112) as u32;

        if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
            a_mant &= !1u128;
        }
        if a_mant > r_max {
            // overflow:
            unsafe {
                *pfflags |= 1u32 << 4;
            }
            return r_max;
        }
        r = a_mant;
        if rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 0;
            }
        }
    }
    if a_sign != 0 {
        r = r.wrapping_neg();
    }
    r
}
}

// ── `internal_cvt_i32_sf32` · softfp_template_icvt.h:125-153 · chunk 3307 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t internal_cvt_i32_sf32(int32_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint32_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 32 - clz32(r) - (32 - 1);
//         if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3307 {
// Port of static uint32_t internal_cvt_i32_sf32(int32_t a, RoundingModeEnum rm,
//                                              uint32_t *pfflags, BOOL is_unsigned)
// plus the static helpers it needs (clz32, normalize_sf32, roundpack_sf32,
// rshift_rnd32, pack_sf32) which have no linker symbols in the C program.

#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_int;

type BOOL = c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
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

#[inline]
fn pack_sf32(a_sign: u32, a_exp: i32, a_mant: u32) -> u32 {
    ((a_sign as u32) << 31) | ((a_exp as u32) << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let diff: i32;
    let addend: u32;
    let mut rnd_bits: u32;
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and the default case
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: BOOL = if a_exp < 0
            || a_mant.wrapping_add(addend) < (1u32 << (32 - 1))
        {
            1
        } else {
            0
        };
        diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal != 0 && rnd_bits != 0 {
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
    pack_sf32(a_sign, a_exp, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp: i32, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf32(
    a: i32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: BOOL,
) -> u32 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u32;
    let mut r: u32;
    let mut mask: u32;
    let l: i32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = (((1 << 8) - 1) / 2) + 32 - 2;

    l = 32 - clz32(r) - (32 - 1);
    if l > 0 {
        mask = r & ((1u32.wrapping_shl(l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    a_mant = r;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `internal_cvt_i64_sf32` · softfp_template_icvt.h:125-153 · chunk 3329 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t internal_cvt_i64_sf32(int64_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint64_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 64 - clz64(r) - (32 - 1);
//         if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3329 {
// Rust port of the static C function `internal_cvt_i64_sf32` from
// softfp_template_icvt.h (F_SIZE = 32, ICVT_SIZE = 64).
//
// The C helpers normalize_sf32 / roundpack_sf32 / rshift_rnd32 / clz32 /
// pack_sf32 are `static` in the C translation unit (no linker symbols), so
// their logic is reimplemented here as private Rust functions with identical
// semantics.

use core::ffi::c_int;

const EXP_MASK: u32 = (1u32 << 8) - 1; // 255
const MANT_MASK: u32 = (1u32 << 23) - 1; // 0x007f_ffff
const MANT_SIZE: u32 = 23;
const RND_SIZE: u32 = (32 - 2) - MANT_SIZE; // 7

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
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
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << MANT_SIZE) | (a_mant & MANT_MASK)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let mut rnd_bits: u32;
    let mut diff: c_int;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (RND_SIZE - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (and any other value)
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << RND_SIZE) - 1;
            } else {
                addend = 0;
            }
        }
    }

    // potentially subnormal
    if a_exp <= 0 {
        // Note: the underflow flag is set if the rounded result is
        // subnormal and inexact.
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << RND_SIZE) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << RND_SIZE) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> RND_SIZE;
    // half way: select even result
    if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !1u32;
    }
    a_exp += (a_mant >> (MANT_SIZE + 1)) as c_int;
    if a_mant <= MANT_MASK {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= EXP_MASK as c_int {
        // overflow
        if addend == 0 {
            a_exp = (EXP_MASK as c_int) - 1;
            a_mant = MANT_MASK;
        } else {
            // infinity
            a_exp = EXP_MASK as c_int;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));
    // The C code asserts shift >= 0; this always holds for a 32-bit mantissa.
    let a_exp = if shift >= 0 {
        a_exp - shift
    } else {
        a_exp
    };
    let a_mant = if shift >= 0 {
        a_mant.wrapping_shl(shift as u32)
    } else {
        a_mant
    };
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf32(
    a: i64,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u32 {
    let a_sign: u32;
    let mut a_exp: c_int;
    let a_mant: u32;
    let mut r: u64;
    let mask: u64;
    let l: c_int;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    a_exp = ((1 << 8) - 1) / 2 + 32 - 2;

    // need to reduce range before generic float normalization
    l = 64 - clz64(r) - (32 - 1);
    if l > 0 {
        mask = r & ((1u64 << (l as u32)) - 1);
        r = (r >> (l as u32)) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    a_mant = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `internal_cvt_i128_sf32` · softfp_template_icvt.h:125-153 · chunk 3351 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t internal_cvt_i128_sf32(int128_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint32_t a_mant;
//         uint128_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 128 - clz128(r) - (32 - 1);
//         if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3351 {
// Port of internal_cvt_i128_sf32 (softfp_template_icvt.h, ICVT_SIZE=128, F_SIZE=32).
// The helpers clz128/clz32, pack_sf32, roundpack_sf32 and normalize_sf32 are
// `static` in the C translation unit, so they have no linker symbol and are
// mirrored here as private Rust functions.

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
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let sh = d as u32;
            let mask = (1u32 << sh).wrapping_sub(1);
            a = (a >> sh) | ((a & mask) != 0) as u32;
        }
    }
    a
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// FFLAG bits
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_INEXACT: u32 = 1 << 0;

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // RND_SIZE = (32 - 2) - 23 = 7
    let addend: u32 = match rm {
        0 | 4 => 1u32 << (7 - 1), // RM_RNE, RM_RMM
        1 => 0,                   // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP (and anything else)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 7) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << 7) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << 7) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    // half way: select even result
    if rm == 0 && rnd_bits == (1u32 << (7 - 1)) {
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

fn normalize_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // shift = clz(a_mant) - (F_SIZE - 1 - IMANT_SIZE) = clz32(a_mant) - 1
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // C has assert(shift >= 0) here; use wrapping shifts so a (never expected)
    // negative shift cannot panic across the FFI boundary.
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf32(
    a: i128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u32 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u32;
    let mut r: u128;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    a_exp = ((1 << 8) - 1) / 2 + 32 - 2;

    // need to reduce range before generic float normalization
    let l: i32 = 128 - clz128(r) - (32 - 1);
    if l > 0 {
        let sh = l as u32;
        let mask: u128 = r & (1u128.wrapping_shl(sh)).wrapping_sub(1);
        r = (r >> sh) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    a_mant = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `internal_cvt_i32_sf64` · softfp_template_icvt.h:125-153 · chunk 3373 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t internal_cvt_i32_sf64(int32_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint32_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 32 - clz32(r) - (64 - 1);
//         if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3373 {
// Rust port of softfp_template_icvt.h:internal_cvt_i32_sf64 (ICVT_SIZE=32, F_SIZE=64).
// normalize_sf64 / roundpack_sf64 / pack_sf64 / rshift_rnd64 are `static` in the C
// translation unit (no linker symbol), so their logic is reimplemented here.

#![allow(clippy::all)]

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

// static inline int clz32(uint32_t a)
#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static inline int clz64(uint64_t a)
#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// static uint64_t rshift_rnd64(uint64_t a, int d)
fn rshift_rnd64(a_in: u64, d: i32) -> u64 {
    let mut a = a_in;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let mask = (1u64.wrapping_shl(d as u32)).wrapping_sub(1);
            let sticky = if (a & mask) != 0 { 1u64 } else { 0 };
            a = a.wrapping_shr(d as u32) | sticky;
        }
    }
    a
}

// static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    (a_sign as u64).wrapping_shl(64 - 1)
        | (a_exp as u64).wrapping_shl(52)
        | (a_mant & ((1u64 << 52) - 1))
}

// static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let mut rnd_bits: u32;

    let addend: u32 = if rm == RM_RNE || rm == 4 {
        // RM_RMM == 4
        1u32 << ((64 - 2) - 52 - 1) // 1 << 9
    } else if rm == RM_RTZ {
        0
    } else {
        // default, RM_RDN, RM_RUP
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            (1u32 << ((64 - 2) - 52)) - 1 // (1 << 10) - 1
        } else {
            0
        }
    };

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant as u32) & ((1u32 << ((64 - 2) - 52)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant as u32) & ((1u32 << ((64 - 2) - 52)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64).wrapping_shr((64 - 2) - 52);

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

// static uint64_t normalize_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant_in) - (64 - 1 - (64 - 2)); // clz64(a_mant) - 1
    // the C code asserts shift >= 0
    let sh: u32 = if shift < 0 { 0 } else { shift as u32 };
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u64 = a_mant_in.wrapping_shl(sh);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf64(
    a: i32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u64 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u64;
    let mut r: u32;
    let mut mask: u32;
    let l: i32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = 0u32.wrapping_sub(a as u32);
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = (((1 << 11) - 1) / 2) + 64 - 2;

    // need to reduce range before generic float normalization
    l = 32 - clz32(r) - (64 - 1);
    if l > 0 {
        let lu: u32 = l as u32;
        mask = r & (1u32.wrapping_shl(lu)).wrapping_sub(1);
        let sticky: u32 = if (r & mask) != 0 { 1 } else { 0 };
        r = r.wrapping_shr(lu) | sticky;
        a_exp = a_exp.wrapping_add(l);
    }
    a_mant = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `internal_cvt_i64_sf64` · softfp_template_icvt.h:125-153 · chunk 3395 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t internal_cvt_i64_sf64(int64_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint64_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 64 - clz64(r) - (64 - 1);
//         if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3395 {
// Port of internal_cvt_i64_sf64 from softfp_template_icvt.h, together with the
// static helpers it needs (clz64, pack_sf64, rshift_rnd64, roundpack_sf64,
// normalize_sf64) which have no linker symbols in the C program.

use core::ffi::{c_int, c_uint};

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & (((1u64) << 52) - 1))
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = ((1u64) << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

unsafe fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let mut rnd_bits: u32;

    let addend: u32 = match rm {
        0 | 4 => (1u32) << (((64 - 2) - 52) - 1), // RM_RNE, RM_RMM
        1 => 0,                                   // RM_RTZ
        _ => {
            // default: RM_RDN, RM_RUP and anything else
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                ((1u32) << ((64 - 2) - 52)) - 1
            } else {
                0
            }
        }
    };

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < ((1u64) << (64 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & (((1u64) << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & (((1u64) << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1 << 0;
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == ((1u32) << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
    if a_mant <= (((1u64) << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = ((1u64) << 52) - 1;
        } else {
            a_exp = (1 << 11) - 1;
            a_mant = 0;
        }
        *pfflags |= (1 << 2) | (1 << 0);
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant_in) - (64 - 1 - (64 - 2));
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf64(
    a: i64,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u64 {
    let a_sign: u32;
    let mut r: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }

    let mut a_exp: i32 = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 64 - clz64(r) - (64 - 1);
    if l > 0 {
        let mask: u64 = r & (((1u64) << l) - 1);
        r = (r >> l) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant: u64 = r;

    unsafe { normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags) }
}

// keep the unused-import checker quiet about c_uint if it ever moves
#[allow(dead_code)]
type _UnusedCUint = c_uint;
}

// ── `internal_cvt_i128_sf64` · softfp_template_icvt.h:125-153 · chunk 3417 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t internal_cvt_i128_sf64(int128_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint64_t a_mant;
//         uint128_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 128 - clz128(r) - (64 - 1);
//         if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3417 {
// Port of the static C helper internal_cvt_i128_sf64 (softfp_template_icvt.h).
// All static helpers it needs (clz128, clz64, pack_sf64, rshift_rnd64,
// roundpack_sf64, normalize_sf64) have no linker symbol, so their logic is
// reproduced here as private Rust functions.

use core::ffi::{c_int, c_uint};

#[inline]
fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn rshift_rnd64(a: u64, d: c_int) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask = (1u64 << du) - 1;
            a = (a >> du) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
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
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << 9;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP, default
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 10) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: c_int = if a_exp < 0
            || a_mant.wrapping_add(addend as u64) < (1u64 << 63)
        {
            1
        } else {
            0
        };
        let diff = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
        if is_subnormal != 0 && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    if rm == 0 && rnd_bits == (1u32 << 9) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> 53) as c_int;
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
    a_exp_in: c_int,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift = clz64(a_mant) - 1;
    // The C code asserts shift >= 0 here; it always holds for its callers.
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf64(
    a: i128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u64 {
    let a_sign: u32;
    let mut a_exp: c_int;
    let a_mant: u64;
    let mut r: u128;
    let l: c_int;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    a_exp = (((1 << 11) - 1) / 2) + 64 - 2;

    l = 128 - clz128(r) - (64 - 1);
    if l > 0 {
        let lu = l as u32;
        let mask: u128 = r & ((1u128 << lu) - 1);
        r = (r >> lu) | if (r & mask) != 0 { 1 } else { 0 };
        a_exp += l;
    }
    a_mant = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

// Keep the unused-import lint quiet for c_uint without affecting semantics.
#[allow(dead_code)]
fn _unused_c_uint(_x: c_uint) {}
}

// ── `internal_cvt_i32_sf128` · softfp_template_icvt.h:125-153 · chunk 3439 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t internal_cvt_i32_sf128(int32_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint32_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 32 - clz32(r) - (128 - 1);
//         if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3439 {
use core::ffi::{c_int, c_uint};

// RoundingModeEnum values (see original.c typedef enum)
const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

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
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127)
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

// port of static rshift_rnd128 (no linker symbol in C)
fn rshift_rnd128(a_in: u128, d: i32) -> u128 {
    let mut a = a_in;
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

// port of static roundpack_sf128 (no linker symbol in C)
fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let addend: u32;
    let rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << 13;
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        // default: RM_RDN, RM_RUP (and anything else)
        if (a_sign ^ (rm & 1)) != 0 {
            addend = (1u32 << 14) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << 127);
        let diff: i32 = 1 - a_exp;
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
    a_mant = (a_mant + (addend as u128)) >> 14;

    if rm == RM_RNE && rnd_bits == (1 << 13) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 113) as i32);
    if a_mant <= (1u128 << 112) - 1 {
        // underflow
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        // overflow
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

// port of static normalize_sf128 (no linker symbol in C)
fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_uint,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let shift: i32 = clz128(a_mant) - 1;

    a_exp = a_exp.wrapping_sub(shift);
    if shift > 0 {
        if shift < 128 {
            a_mant <<= shift as u32;
        } else {
            a_mant = 0;
        }
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf128(
    a: i32,
    rm: c_uint,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u128 {
    let a_sign: u32;
    let mut r: u32;
    let mut a_exp: i32;
    let a_mant: u128;
    let mut l: i32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = ((1i32 << 15) - 1) / 2 + 128 - 2;

    // need to reduce range before generic float normalization
    l = 32 - clz32(r) - (128 - 1);
    if l > 0 {
        let mask: u32 = r & ((1u32 << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    a_mant = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `internal_cvt_i64_sf128` · softfp_template_icvt.h:125-153 · chunk 3461 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t internal_cvt_i64_sf128(int64_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint64_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 64 - clz64(r) - (128 - 1);
//         if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3461 {
// Rust port of the file-static C function internal_cvt_i64_sf128
// (softfp_template_icvt.h).  The helpers it calls (clz64 / clz128 /
// pack_sf128 / rshift_rnd128 / roundpack_sf128 / normalize_sf128) are
// `static` in C, so they have no linker symbol and are re-implemented here
// as private Rust functions with identical semantics.

use core::ffi::c_int;

const MANT_BITS: u32 = 112; // 128 - 2 - 14
const RND_BITS: u32 = (128 - 2) - 112; // 14
const EXP_MASK: i32 = (1 << 15) - 1; // 32767

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

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

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << MANT_BITS)
        | (a_mant & ((1u128 << MANT_BITS) - 1))
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

unsafe fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = match rm {
        0 | 4 => 1u32 << (RND_BITS - 1), // RM_RNE, RM_RMM
        1 => 0,                          // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP and anything else
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << RND_BITS) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << (128 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << RND_BITS) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1u32 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << RND_BITS) - 1)) as u32;
    }
    if rnd_bits != 0 {
        *pfflags |= 1u32 << 0;
    }
    a_mant = (a_mant + (addend as u128)) >> RND_BITS;

    if rm == 0 && rnd_bits == (1u32 << (RND_BITS - 1)) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> (MANT_BITS + 1)) as i32;
    if a_mant <= (1u128 << MANT_BITS) - 1 {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = (1u128 << MANT_BITS) - 1;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        *pfflags |= (1u32 << 2) | (1u32 << 0);
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    // shift = clz128(a_mant) - (128 - 1 - (128 - 2)); it is always >= 0
    // for the inputs produced below, matching the C assertion.
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf128(
    a: i64,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u128 {
    let a_sign: u32;
    let mut r: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    let mut a_exp: i32 = (((1 << 15) - 1) / 2) + 128 - 2;

    // need to reduce range before generic float normalization
    let l: i32 = 64 - clz64(r) - (128 - 1);
    if l > 0 {
        let mask: u64 = r & ((1u64.wrapping_shl(l as u32)) - 1);
        r = r.wrapping_shr(l as u32) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant: u128 = r as u128;
    unsafe { normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags) }
}
}

// ── `internal_cvt_i128_sf128` · softfp_template_icvt.h:125-153 · chunk 3483 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint128_t internal_cvt_i128_sf128(int128_t a,
//                                                                            RoundingModeEnum rm,
//                                                                            uint32_t *pfflags,
//                                                                            BOOL is_unsigned)
//     {
//         uint32_t a_sign;
//         int32_t a_exp;
//         uint128_t a_mant;
//         uint128_t r, mask;
//         int l;
//     
//         if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
//         a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 128 - clz128(r) - (128 - 1);
//         if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
//         a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3483 {
// Port of softfp_template_icvt.h: internal_cvt_i128_sf128 (128-bit int -> binary128)
// The helpers clz128 / rshift_rnd128 / pack_sf128 / roundpack_sf128 / normalize_sf128 are
// `static` in the C translation unit, so they are re-implemented here as private fns.
#![allow(dead_code)]

use core::ffi::{c_int, c_uint};

const F_SIZE: i32 = 128; // 128
const EXP_MASK: i32 = (1 << 15) - 1; // 32767
const MANT_MASK: u128 = (1u128 << 112) - 1;
const MANT_RND_BITS: u128 = (1u128 << ((128 - 2) - 112)) - 1; // 14 bits
const ROUND_ADDEND: u32 = 1u32 << (((128 - 2) - 112) - 1); // 1 << 13
const RM_RNE: c_int = 0;

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | (((a & mask) != 0) as u128);
        }
    }
    a
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        // RM_RNE
        0 => addend = ROUND_ADDEND,
        // RM_RMM
        4 => addend = ROUND_ADDEND,
        // RM_RTZ
        1 => addend = 0,
        // RM_RDN, RM_RUP, default
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = ROUND_ADDEND - 1;
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
        rnd_bits = (a_mant & MANT_RND_BITS) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & MANT_RND_BITS) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> (112 + 1)) as i32;
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
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    // C asserts shift >= 0; keep the operation panic-free regardless.
    if shift < 0 {
        return roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags);
    }
    a_exp -= shift;
    if (shift as u32) < 128 {
        a_mant <<= shift as u32;
    } else {
        a_mant = 0;
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf128(
    a: i128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u128 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u128;
    let mut r: u128;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    a_exp = (EXP_MASK / 2) + F_SIZE - 2;
    // need to reduce range before generic float normalization
    let l: i32 = 128 - clz128(r) - (F_SIZE - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128 << l) - 1);
        r = (r >> l) | (((r & mask) != 0) as u128);
        a_exp += l;
    }
    a_mant = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// keep the c_uint import used without warnings in case of future edits
#[allow(unused_imports)]
use core::ffi::c_ulong as _unused_c_ulong;
const _: fn() = || {
    let _x: c_uint = 0;
};
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3308 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3308 {
// Port of the sign/abs extraction part of internal_cvt_i32_sf32.
// a: int32_t (c_int), is_unsigned: BOOL (c_int),
// a_sign: uint32_t* out, r: uint32_t* out.
#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf32__part3308(
    a: i32,
    is_unsigned: i32,
    a_sign: *mut u32,
    r: *mut u32,
) {
    unsafe {
        if is_unsigned == 0 && a < 0 {
            *a_sign = 1;
            *r = (a as u32).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a as u32;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3330 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3330 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf32__part3330(
    a: i64,
    is_unsigned: c_int,
    a_sign: *mut u32,
    r: *mut u64,
) {
    unsafe {
        if is_unsigned == 0 && a < 0 {
            *a_sign = 1;
            *r = (a as u64).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a as u64;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3352 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3352 {
// Extracted from internal_cvt_i128_sf32: sign extraction and magnitude
// computation for the signed/unsigned 128-bit integer input.
#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf32__part3352(
    a: i128,
    is_unsigned: core::ffi::c_int,
    a_sign: *mut u32,
    r: *mut u128,
) {
    unsafe {
        if is_unsigned == 0 && a < 0 {
            *a_sign = 1;
            // -(uint128_t)a, with two's complement wrap-around
            *r = (a as u128).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a as u128;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3374 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3374 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf64__part3374(
    a: i32,
    is_unsigned: c_int,
    a_sign: *mut u32,
    r: *mut u32,
) {
    unsafe {
        if is_unsigned == 0 && a < 0 {
            *a_sign = 1;
            *r = (a as u32).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a as u32;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3396 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3396 {
// Port of the sign/abs computation from internal_cvt_i64_sf64.

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf64__part3396(
    a: i64,
    is_unsigned: core::ffi::c_int,
    a_sign: *mut u32,
    r: *mut u64,
) {
    if is_unsigned == 0 && a < 0 {
        unsafe {
            *a_sign = 1;
            *r = (a as u64).wrapping_neg();
        }
    } else {
        unsafe {
            *a_sign = 0;
            *r = a as u64;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3418 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3418 {
// Port of the sign-magnitude split of internal_cvt_i128_sf64:
//   if (!is_unsigned && a < 0) { a_sign = 1; r = -(uint128_t)a; }
//   else { a_sign = 0; r = a; }

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf64__part3418(
    a: i128,
    is_unsigned: c_int,
    pa_sign: *mut u32,
    pr: *mut u128,
) {
    let mut a_sign: u32 = 0;
    let mut r: u128;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        // r = -(uint128_t)a;  unsigned negation wraps modulo 2^128
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        // r = a;  implicit conversion to unsigned keeps the bit pattern
        r = a as u128;
    }

    unsafe {
        *pa_sign = a_sign;
        *pr = r;
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3440 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint32_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3440 {
use core::ffi::c_int;

#[no_mangle]
pub unsafe extern "C" fn internal_cvt_i32_sf128__part3440(
    a: c_int,
    is_unsigned: c_int,
    out_a_sign: *mut u32,
    out_r: *mut u32,
) {
    if is_unsigned == 0 && a < 0 {
        *out_a_sign = 1;
        *out_r = (a as u32).wrapping_neg();
    } else {
        *out_a_sign = 0;
        *out_r = a as u32;
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3462 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint64_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3462 {
// Sign handling and magnitude extraction for internal_cvt_i64_sf128.
use core::ffi::{c_int, c_long};

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf128__part3462(
    a: c_long,
    is_unsigned: c_int,
    a_sign: *mut u32,
    r: *mut u64,
) {
    let a64: i64 = a;
    let is_unsigned_nonzero = is_unsigned != 0;
    unsafe {
        if !is_unsigned_nonzero && a64 < 0 {
            *a_sign = 1;
            *r = (a64 as u64).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a64 as u64;
        }
    }
}
}

// ── `if (!is_unsigned && a < 0) … else` · softfp_template_icvt.h:136-142 · chunk 3484 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!is_unsigned && a < 0) {
//             a_sign = 1;
//             r = -(uint128_t)a;
//         } else {
//             a_sign = 0;
//             r = a;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3484 {
// Port of the sign/normalisation step of internal_cvt_i128_sf128.
// Takes: is_unsigned (BOOL == int), a (int128_t)
// Writes through pointers: a_sign (uint32_t), r (uint128_t)

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf128__part3484(
    is_unsigned: core::ffi::c_int,
    a: i128,
    a_sign: *mut u32,
    r: *mut u128,
) {
    unsafe {
        if is_unsigned == 0 && a < 0 {
            *a_sign = 1;
            *r = (a as u128).wrapping_neg();
        } else {
            *a_sign = 0;
            *r = a as u128;
        }
    }
}
}

// ── `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 32 - clz32(r) - (32 - 1);` · softfp_template_icvt.h:143-145 · chunk 3309 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 32 - clz32(r) - (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3309 {
// Ported piece: exponent seed and normalization shift for internal_cvt_i32_sf32.

// Mirrors the file-static `clz32` helper (no linker symbol, logic inlined here).
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf32__part3309(r: u32, p_a_exp: *mut i32, p_l: *mut i32) {
    // a_exp = ((1 << 8) - 1) / 2 + 32 - 2;
    let a_exp: i32 = ((1i32 << 8) - 1) / 2 + 32 - 2;
    // l = 32 - clz32(r) - (32 - 1);
    let l: i32 = 32 - clz32(r) - (32 - 1);

    unsafe {
        if !p_a_exp.is_null() {
            *p_a_exp = a_exp;
        }
        if !p_l.is_null() {
            *p_l = l;
        }
    }
}
}

// ── `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 64 - clz64(r) - (32 - 1);` · softfp_template_icvt.h:143-145 · chunk 3331 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 64 - clz64(r) - (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3331 {
// Ported piece of internal_cvt_i64_sf32:
//   a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//   l = 64 - clz64(r) - (32 - 1);
// clz64 is a static inline helper, so its logic is inlined here.

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf32__part3331(
    r: u64,
    pa_exp: *mut i32,
    pl: *mut core::ffi::c_int,
) {
    // static inline int clz64(uint64_t a) { return a == 0 ? 64 : __builtin_clzll(a); }
    let clz: i32 = if r == 0 {
        64
    } else {
        r.leading_zeros() as i32
    };

    let a_exp: i32 = (((1i32 << 8) - 1) / 2) + 32 - 2;
    let l: i32 = 64 - clz - (32 - 1);

    unsafe {
        if !pa_exp.is_null() {
            *pa_exp = a_exp;
        }
        if !pl.is_null() {
            *pl = l;
        }
    }
}
}

// ── `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 128 - clz128(r) - (32 - 1);` · softfp_template_icvt.h:143-145 · chunk 3353 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
//     
//         l = 128 - clz128(r) - (32 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3353 {
// Port of the a_exp initialization and leading-zero-count shift computation
// from internal_cvt_i128_sf32 (softfp_template_icvt.h lines 143-145).

// Ported from the static inline C helper clz128 (no linker symbol).
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
pub extern "C" fn internal_cvt_i128_sf32__part3353(
    r_hi: u64,
    r_lo: u64,
    pa_exp: *mut i32,
    pl: *mut core::ffi::c_int,
) {
    let r: u128 = ((r_hi as u128) << 64) | (r_lo as u128);

    // a_exp = (((1 << 8) - 1) / 2) + 32 - 2;
    let a_exp: i32 = ((1i32 << 8) - 1) / 2 + 32 - 2;

    // l = 128 - clz128(r) - (32 - 1);
    let l: i32 = 128 - clz128(r) - (32 - 1);

    unsafe {
        if !pa_exp.is_null() {
            *pa_exp = a_exp;
        }
        if !pl.is_null() {
            *pl = l;
        }
    }
}
}

// ── `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 32 - clz32(r) - (64 - 1);` · softfp_template_icvt.h:143-145 · chunk 3375 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 32 - clz32(r) - (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3375 {
// Port of part of internal_cvt_i32_sf64: exponent init and l computation.
use core::ffi::{c_int, c_uint};

// static inline int clz32(uint32_t a) -- no linker symbol, ported here.
#[inline]
fn clz32(a: c_uint) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf64__part3375(
    r: c_uint,
    a_exp: *mut c_int,
    l: *mut c_int,
) {
    unsafe {
        if !a_exp.is_null() {
            *a_exp = (((1i32 << 11) - 1) / 2) + 64 - 2;
        }
        if !l.is_null() {
            *l = 32 - clz32(r) - (64 - 1);
        }
    }
}
}

// ── `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 64 - clz64(r) - (64 - 1);` · softfp_template_icvt.h:143-145 · chunk 3397 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 64 - clz64(r) - (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3397 {
// Port of the TARGET piece of internal_cvt_i64_sf64:
//   a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//   l = 64 - clz64(r) - (64 - 1);
// clz64 is a static inline C function, so its logic is mirrored here.

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf64__part3397(
    r: u64,
    a_exp: *mut i32,
    l: *mut core::ffi::c_int,
) {
    // clz64: 0 -> 64, otherwise number of leading zero bits (leading_zeros does both)
    let clz: i32 = r.leading_zeros() as i32;
    let new_exp: i32 = (((1i32 << 11) - 1) / 2) + 64 - 2;
    let new_l: core::ffi::c_int = 64 - clz - (64 - 1);
    unsafe {
        if !a_exp.is_null() {
            *a_exp = new_exp;
        }
        if !l.is_null() {
            *l = new_l;
        }
    }
}
}

// ── `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 128 - clz128(r) - (64 - 1);` · softfp_template_icvt.h:143-145 · chunk 3419 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//     
//         l = 128 - clz128(r) - (64 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3419 {
// Port of the TARGET inside internal_cvt_i128_sf64:
//   a_exp = (((1 << 11) - 1) / 2) + 64 - 2;
//   l = 128 - clz128(r) - (64 - 1);
// clz128 is a static inline helper with no linker symbol, so its logic is
// reimplemented here as a private Rust function.

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
pub extern "C" fn internal_cvt_i128_sf64__part3419(
    r: u128,
    pa_exp: *mut i32,
    pl: *mut i32,
) {
    let a_exp: i32 = (((1i32 << 11) - 1) / 2) + 64 - 2;
    let l: i32 = 128 - clz128(r) - (64 - 1);
    unsafe {
        *pa_exp = a_exp;
        *pl = l;
    }
}
}

// ── `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 32 - clz32(r) - (128 - 1);` · softfp_template_icvt.h:143-145 · chunk 3441 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 32 - clz32(r) - (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3441 {
// Portion of internal_cvt_i32_sf128: computes the biased exponent and the
// leading-zero based shift amount.
use core::ffi::{c_int, c_uint};

// static inline clz32() has no linker symbol: reimplemented here.
#[inline]
fn clz32(a: c_uint) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf128__part3441(
    a_exp: *mut c_int,
    r: c_uint,
    l: *mut c_int,
) {
    unsafe {
        if !a_exp.is_null() {
            *a_exp = (((1i32 << 15) - 1) / 2) + 128 - 2;
        }
        if !l.is_null() {
            *l = 32 - clz32(r) - (128 - 1);
        }
    }
}
}

// ── `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 64 - clz64(r) - (128 - 1);` · softfp_template_icvt.h:143-145 · chunk 3463 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 64 - clz64(r) - (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3463 {
// Ported piece of internal_cvt_i64_sf128: computes the initial exponent and
// the shift count l from the magnitude r. clz64 is a static inline helper in
// the C translation unit, so its logic is mirrored here privately.

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf128__part3463(r: u64, a_exp: *mut i32, l: *mut i32) {
    let e: i32 = (((1i32 << 15) - 1) / 2) + 128 - 2;
    let lv: i32 = 64 - clz64(r) - (128 - 1);
    unsafe {
        if !a_exp.is_null() {
            *a_exp = e;
        }
        if !l.is_null() {
            *l = lv;
        }
    }
}
}

// ── `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 128 - clz128(r) - (128 - 1);` · softfp_template_icvt.h:143-145 · chunk 3485 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_exp = (((1 << 15) - 1) / 2) + 128 - 2;
//     
//         l = 128 - clz128(r) - (128 - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3485 {
// Port of the a_exp / l computation from internal_cvt_i128_sf128.
// clz128 is a static inline C helper with no linker symbol, so its logic
// is reimplemented here as a private Rust fn.

fn clz128(a: u128) -> core::ffi::c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as core::ffi::c_int
    }
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf128__part3485(
    r: u128,
    pa_exp: *mut i32,
) -> core::ffi::c_int {
    let a_exp: i32 = (((1i32 << 15) - 1) / 2) + 128 - 2;
    unsafe {
        *pa_exp = a_exp;
    }
    let l: core::ffi::c_int = 128 - clz128(r) - (128 - 1);
    l
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3310 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3310 {
// Port of the rounding/normalizing step of internal_cvt_i32_sf32.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf32__part3310(
    l: c_int,
    r: *mut u32,
    mask: *mut u32,
    a_exp: *mut i32,
) {
    unsafe {
        if l > 0 {
            let l = l as u32;
            let rv = *r;
            let m = rv & ((1u32.wrapping_shl(l)) - 1);
            *mask = m;
            *r = rv.wrapping_shr(l) | ((rv & m) != 0) as u32;
            *a_exp += l as i32;
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3332 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3332 {
// Port of the rounding/shift block of internal_cvt_i64_sf32.
// Shifts the 64-bit magnitude right by l bits, ORs in the sticky bit
// (whether any shifted-out bit was set), and bumps the exponent.

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf32__part3332(
    r: *mut u64,
    a_exp: *mut i32,
    l: i32,
) {
    if l > 0 {
        unsafe {
            let rv = *r;
            let sh = l as u32;
            let mask = rv & ((1u64 << sh) - 1);
            let mut nr = rv >> sh;
            if (rv & mask) != 0 {
                nr |= 1;
            }
            *r = nr;
            *a_exp += l;
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3354 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3354 {
// Port of internal_cvt_i128_sf32's normalization shift block.
// Reads r and a_exp, updates r, mask and a_exp in place.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf32__part3354(
    pr: *mut u128,
    pmask: *mut u128,
    pa_exp: *mut i32,
    l: c_int,
) {
    unsafe {
        if pr.is_null() || pmask.is_null() || pa_exp.is_null() {
            return;
        }
        if l > 0 {
            let r = *pr;
            let lu = l as u32;
            // C: (((uint128_t)1 << l) - 1); avoid a Rust shift-overflow panic.
            let low = if lu < 128 { (1u128 << lu) - 1 } else { u128::MAX };
            let mask = r & low;
            let shifted = if lu < 128 { r >> lu } else { 0 };
            let sticky = ((r & mask) != 0) as u128;
            *pmask = mask;
            *pr = shifted | sticky;
            *pa_exp = (*pa_exp).wrapping_add(l);
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3376 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3376 {
use core::ffi::c_int;

// Port of the right-shift/rounding step of internal_cvt_i32_sf64.
// Reads:  *r, *a_exp, l
// Writes: *r, *a_exp
#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf64__part3376(r: *mut u32, a_exp: *mut i32, l: c_int) {
    unsafe {
        if l > 0 {
            let rv: u32 = *r;
            let lu: u32 = l as u32;
            let one: u32 = 1;
            let mask: u32 = rv & one.wrapping_shl(lu).wrapping_sub(1);
            let sticky: u32 = if (rv & mask) != 0 { 1 } else { 0 };
            *r = rv.wrapping_shr(lu) | sticky;
            let exp: i32 = *a_exp;
            *a_exp = exp.wrapping_add(l);
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3398 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3398 {
// Ported piece of internal_cvt_i64_sf64: the l > 0 rounding/shift block.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf64__part3398(
    l: c_int,
    r: *mut u64,
    a_exp: *mut i32,
    mask: *mut u64,
) {
    unsafe {
        let rv = *r;
        let m = rv & (1u64.wrapping_shl(l as u32)).wrapping_sub(1);
        *mask = m;
        *r = rv.wrapping_shr(l as u32) | ((rv & m != 0) as u64);
        *a_exp = (*a_exp).wrapping_add(l);
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3420 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3420 {
// Port of the mantissa-shifting block of internal_cvt_i128_sf64.
// Operates in place on the C locals `r` (uint128_t) and `a_exp` (int32_t).

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf64__part3420(
    r: *mut u128,
    a_exp: *mut i32,
    l: i32,
) {
    if r.is_null() || a_exp.is_null() {
        return;
    }
    unsafe {
        if l > 0 {
            let sh = l as u32;
            // (uint128_t)1 << l ; l in (0, 65] here so no overflow / UB.
            let one_shifted: u128 = 1u128.wrapping_shl(sh);
            let mask: u128 = (*r) & (one_shifted.wrapping_sub(1));
            let sticky: u128 = if ((*r) & mask) != 0 { 1 } else { 0 };
            *r = (*r).wrapping_shr(sh) | sticky;
            *a_exp = (*a_exp).wrapping_add(l);
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3442 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint32_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3442 {
use core::ffi::c_int;

// Port of the l > 0 shift/round-sticky block of internal_cvt_i32_sf128.
// Reads:  l (by value), *r, *a_exp
// Writes: *mask, *r, *a_exp
#[no_mangle]
pub unsafe extern "C" fn internal_cvt_i32_sf128__part3442(
    l: c_int,
    r: *mut u32,
    mask: *mut u32,
    a_exp: *mut i32,
) {
    if l > 0 {
        let rv: u32 = *r;
        // (((uint32_t)1 << l) - 1) -- defensive against shift overflows (C UB there).
        let low: u32 = match (1u32).checked_shl(l as u32) {
            Some(v) => v.wrapping_sub(1),
            None => u32::MAX,
        };
        let m: u32 = rv & low;
        *mask = m;
        let shifted: u32 = match rv.checked_shr(l as u32) {
            Some(v) => v,
            None => 0u32,
        };
        let sticky: u32 = if (rv & m) != 0 { 1 } else { 0 };
        *r = shifted | sticky;
        *a_exp = (*a_exp).wrapping_add(l);
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3464 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint64_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3464 {
// Port of the rounding/normalisation step of internal_cvt_i64_sf128.
// Reads `l` and clobbers `r` and `a_exp` (both passed by address).

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf128__part3464(
    r: *mut u64,
    a_exp: *mut i32,
    l: core::ffi::c_int,
) {
    unsafe {
        if l > 0 {
            // x86-like shift masking; the shift count is 1..=63 for valid input.
            let s: u32 = (l as u32) & 63;
            let rv: u64 = *r;
            let mask: u64 = rv & ((1u64 << s).wrapping_sub(1));
            *r = (rv >> s) | ((rv & mask) != 0) as u64;
            *a_exp = (*a_exp).wrapping_add(l);
        }
    }
}
}

// ── `if (l > 0)` · softfp_template_icvt.h:146-150 · chunk 3486 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l > 0) {
//             mask = r & (((uint128_t)1 << l) - 1);
//             r = (r >> l) | ((r & mask) != 0);
//             a_exp += l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3486 {
// Port of the bit-rounding step of internal_cvt_i128_sf128 (softfp_template_icvt.h).
// Takes the leading-bit count `l`, the magnitude `r` and a pointer to the biased
// exponent; returns the possibly shortened magnitude and updates the exponent.

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf128__part3486(
    l: core::ffi::c_int,
    r_in: u128,
    pa_exp: *mut i32,
) -> u128 {
    let mut r: u128 = r_in;
    if l > 0 {
        let sh: u32 = l as u32;
        // ((uint128_t)1 << l) - 1 with wrapping/width-masking shift semantics
        let mask: u128 = r & 1u128.wrapping_shl(sh).wrapping_sub(1);
        let sticky: u128 = if (r & mask) != 0 { 1 } else { 0 };
        r = r.wrapping_shr(sh) | sticky;
        unsafe {
            if !pa_exp.is_null() {
                *pa_exp = (*pa_exp).wrapping_add(l);
            }
        }
    }
    r
}
}

// ── `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3311 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3311 {
// Ported piece of internal_cvt_i32_sf32: truncation of the tail
// (a_mant = r; return normalize_sf32(...)) together with the static
// helpers it needs (normalize_sf32, roundpack_sf32, rshift_rnd32,
// clz32, pack_sf32), which have no linker symbols in C.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

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
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(mut a: u32, d: c_int) -> u32 {
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

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default / RM_RDN / RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
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
    mut a_exp: c_int,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: c_int = clz32(a_mant) - (32 - 1 - (32 - 2));
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf32__part3311(
    a_sign: u32,
    a_exp: i32,
    r: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let a_mant: u32 = r;
    normalize_sf32(a_sign, a_exp as c_int, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3333 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3333 {
// Port of the tail of internal_cvt_i64_sf32 plus the static helpers it
// calls (clz32, normalize_sf32, roundpack_sf32, rshift_rnd32, pack_sf32),
// none of which have linker symbols.
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RMM: c_int = 4;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a_in: u32, d: c_int) -> u32 {
    let mut a = a_in;
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
    a_exp_in: c_int,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (((32 - 2) - 23) - 1);
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else {
        // RM_RDN, RM_RUP and default
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << ((32 - 2) - 23)) - 1;
        } else {
            addend = 0;
        }
    }

    let rnd_bits: u32;

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
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    // assert(shift >= 0) always holds: clz32 <= 32 and the offset is 1
    let a_exp = a_exp_in - shift;
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf32__part3333(
    a_sign: u32,
    a_exp: c_int,
    a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3355 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3355 {
// Port of the tail of internal_cvt_i128_sf32 (softfp_template_icvt.h lines 151-152):
//     a_mant = r;
//     return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);
//
// normalize_sf32 and the helpers it uses are `static` in the C translation unit
// (no linker symbols), so their logic is reproduced here as private Rust fns.

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RDN: i32 = 2;
const RM_RUP: i32 = 3;
const RM_RMM: i32 = 4;

// static inline int clz32(uint32_t a)
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// static uint32_t rshift_rnd32(uint32_t a, int d)
fn rshift_rnd32(a: u32, d: i32) -> u32 {
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

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: i32,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    // ((32 - 2) - 23) == 7
    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and any other value
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

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: i32,
    pfflags: *mut u32,
) -> u32 {
    // shift = clz32(a_mant) - (32 - 1 - (32 - 2))   with an assert shift >= 0
    let mut shift = clz32(a_mant_in) - ((32 - 1) - (32 - 2));
    if shift < 0 {
        // unreachable for valid inputs (C would abort); keep it panic-free
        shift = 0;
    }
    let a_exp = a_exp_in - shift;
    let a_mant = a_mant_in << shift;
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf32__part3355(
    a_sign: u32,
    a_exp: i32,
    r: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u32 {
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3377 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3377 {
// Port of the tail of internal_cvt_i32_sf64: mantissa assignment plus the
// normalize_sf64 path (static inline helpers roundpack_sf64, rshift_rnd64,
// pack_sf64 and clz64 are inlined here as private Rust fns).

#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_uint};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
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

fn roundpack_sf64(a_sign: u32, mut a_exp: i32, mut a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
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
            // default: RM_RDN, RM_RUP
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_mask: u64 = (1u64 << ((64 - 2) - 52)) - 1;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
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
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1i32 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(a_sign: u32, mut a_exp: i32, mut a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    if shift < 0 {
        unsafe {
            __assert_fail(
                b"shift >= 0\0".as_ptr() as *const c_char,
                b"softfp_template.h\0".as_ptr() as *const c_char,
                183u32,
                b"normalize_sf64\0".as_ptr() as *const c_char,
            )
        }
    }
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf64__part3377(
    a_sign: u32,
    a_exp: i32,
    r: u32,
    rm: u32,
    pfflags: *mut u32,
) -> u64 {
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3399 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3399 {
// Portion of internal_cvt_i64_sf64: mantissa hand-off plus normalize/link.
// normalize_sf64 and roundpack_sf64 are static in the C TU, so their logic is
// mirrored here as private Rust functions.

const RM_RNE: u32 = 0;

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
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

fn roundpack_sf64(mut a_sign: u32, mut a_exp: i32, mut a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let addend: u32;
    let mut rnd_bits: u32;

    // switch (rm)
    if rm == RM_RNE || rm == 4 {
        addend = 1u32 << (((64 - 2) - 52) - 1);
    } else if rm == 1 {
        addend = 0;
    } else {
        // default / RM_RDN (2) / RM_RUP (3)
        if (a_sign ^ (rm & 1)) != 0 {
            addend = (1u32 << ((64 - 2) - 52)) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal: i32;
        is_subnormal = (a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63)) as i32;
        let diff = 1i32.wrapping_sub(a_exp);
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1) as u64) as u32;
        if is_subnormal != 0 && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1) as u64) as u32;
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
    if a_mant <= (((1u64 << 52) - 1)) {
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

fn normalize_sf64(a_sign: u32, mut a_exp: i32, mut a_mant: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let shift = clz64(a_mant) - (64 - 1 - (64 - 2));
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i64_sf64__part3399(
    a_sign: u32,
    a_exp: i32,
    r: u64,
    rm: u32,
    pfflags: *mut u32,
) -> u64 {
    let a_mant = r;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3421 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3421 {
// Ported piece of internal_cvt_i128_sf64: the tail
//   a_mant = r;
//   return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);
// normalize_sf64 / roundpack_sf64 / pack_sf64 / clz64 / rshift_rnd64 are
// static (or static inline) C helpers with no linker symbol, so their logic
// is mirrored here as private Rust functions.

const RM_RNE: u32 = 0;
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
fn rshift_rnd64(a: u64, d: i32) -> u64 {
    if d != 0 {
        if d >= 64 {
            if a != 0 { 1 } else { 0 }
        } else {
            let mask: u64 = (1u64 << d) - 1;
            let sticky: u64 = if (a & mask) != 0 { 1 } else { 0 };
            (a >> d) | sticky
        }
    } else {
        a
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: u32, pfflags: *mut u32) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;
    let addend: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << 9;
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else {
        // RM_RDN, RM_RUP and (C) default
        if (a_sign ^ (rm & 1)) != 0 {
            addend = (1u32 << 10) - 1;
        } else {
            addend = 0;
        }
    }

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << 63);
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

    if rm == RM_RNE && rnd_bits == (1u32 << 9) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 53) as i32);

    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1i32 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }

    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: u32, pfflags: *mut u32) -> u64 {
    // C: shift = clz64(a_mant) - (64 - 1 - (64 - 2)); assert(shift >= 0)
    let shift: i32 = clz64(a_mant_in).wrapping_sub(1);
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u64 = if shift >= 0 && shift < 64 {
        a_mant_in << shift
    } else {
        // unreachable in practice (the C code asserts shift >= 0)
        a_mant_in
    };
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf64__part3421(
    a_sign: u32,
    a_exp: i32,
    a_mant: u64,
    rm: u32,
    pfflags: *mut u32,
) -> u64 {
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3443 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3443 {
// Port of the tail of internal_cvt_i32_sf128: a_mant = r;
// return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// normalize_sf128 / roundpack_sf128 / rshift_rnd128 / pack_sf128 / clz128
// are static in the C translation unit, so their logic is ported here.

#![allow(dead_code)]

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RDN: i32 = 2;
const RM_RUP: i32 = 3;
const RM_RMM: i32 = 4;

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let sh = d as u32;
            let mask = (1u128 << sh).wrapping_sub(1);
            let low: u128 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> sh) | low;
        }
    }
    a
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << 13,
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 14) - 1
            } else {
                0
            }
        }
    };

    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff: i32 = 1i32.wrapping_sub(a_exp);
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

    if rm == RM_RNE && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 113) as i32);

    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1i32 << 15) - 1;
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
    rm: i32,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    if shift < 0 {
        // C asserts shift >= 0 here; emulate the abort.
        std::process::abort();
    }
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = if shift >= 128 {
        0
    } else {
        a_mant << (shift as u32)
    };
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn internal_cvt_i32_sf128__part3443(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: core::ffi::c_int,
    pfflags: *mut u32,
) -> u128 {
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3465 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3465 {
// Port of the tail of internal_cvt_i64_sf128 from softfp_template_icvt.h:
//   a_mant = r;
//   return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
//
// normalize_sf128 / roundpack_sf128 / pack_sf128 / rshift_rnd128 / clz128 are
// `static` (or `static inline`) C functions without linker symbols, so their
// logic is reproduced here as private Rust functions.

use core::ffi::{c_int, c_uint};

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

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & (((1u128) << 112) - 1))
}

fn rshift_rnd128(mut a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask = ((1u128) << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

unsafe fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let addend: u32;

    if rm == 0 || rm == 4 {
        // RM_RNE, RM_RMM
        addend = (1u32) << (((128 - 2) - 112) - 1);
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else {
        // default: RM_RDN, RM_RUP
        if ((a_sign as c_int) ^ (rm & 1)) != 0 {
            addend = ((1u32) << ((128 - 2) - 112)) - 1;
        } else {
            addend = 0;
        }
    }

    let mut rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0
            || a_mant.wrapping_add(addend as u128) < ((1u128) << (128 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((((1u128) << ((128 - 2) - 112)) - 1))) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((((1u128) << ((128 - 2) - 112)) - 1))) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == 0 && rnd_bits == ((1u32) << (((128 - 2) - 112) - 1)) {
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
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf128(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: c_int = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let shift: c_int = clz128(a_mant) - (128 - 1 - (128 - 2));
    // assert(shift >= 0) in C; clz128(a_mant) >= 1 always here, so shift >= 0.
    a_exp -= shift;
    a_mant <<= shift;
    unsafe { roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags) }
}

#[no_mangle]
pub unsafe extern "C" fn internal_cvt_i64_sf128__part3465(
    a_sign: u32,
    a_exp: i32,
    r: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let a_mant: u128 = r as u128;
    unsafe { normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags) }
}

// Silence unused-import warnings while keeping the C-ish type spellings clear.
#[allow(dead_code)]
fn _unused(_: c_uint) {}
}

// ── `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` · softfp_template_icvt.h:151-152 · chunk 3487 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     a_mant = r;
//         return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3487 {
// Port of the tail of internal_cvt_i128_sf128 (softfp_template_icvt.h lines 151-152):
//   a_mant = r;
//   return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
// normalize_sf128 and its helpers are `static` in the C translation unit, so they
// are ported here as private Rust functions.

#[no_mangle]
pub extern "C" fn internal_cvt_i128_sf128__part3487(
    a_sign: u32,
    a_exp: core::ffi::c_int,
    r: u128,
    rm: core::ffi::c_int,
    pfflags: *mut u32,
) -> u128 {
    let a_mant: u128 = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RDN: i32 = 2;
const RM_RUP: i32 = 3;
const RM_RMM: i32 = 4;

// mask over the 14 rounding bits kept below the 112-bit mantissa field
const RND_MASK: u128 = (1u128 << 14) - 1;
const MANT_MASK: u128 = (1u128 << 112) - 1;
const EXP_MAX: i32 = (1 << 15) - 1;

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

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else if d > 0 {
            let mask: u128 = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u128;
        }
    }
    a
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((128 - 2) - 112) - 1),
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((128 - 2) - 112)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << (128 - 1));
        let diff: i32 = 1 - a_exp;
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
    a_mant = (a_mant + (addend as u128)) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> (112 + 1)) as i32;
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
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(a_sign: u32, a_exp_in: i32, a_mant_in: u128, rm: i32, pfflags: *mut u32) -> u128 {
    let mut a_mant: u128 = a_mant_in;
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    let mut a_exp: i32 = a_exp_in;
    // shift >= 0 is asserted by the C code
    a_exp -= shift;
    if shift > 0 && shift < 128 {
        a_mant <<= shift as u32;
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `cvt_i32_sf32` · softfp_template_icvt.h:155-160 · chunk 3312 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_i32_sf32(int32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf32(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3312 {
// Port of softfp_template_icvt.h: cvt_i32_sf32 (F_SIZE=32, ICVT_SIZE=32).
// internal_cvt_i32_sf32 and its helpers (normalize_sf32, roundpack_sf32,
// rshift_rnd32, clz32, pack_sf32) are `static` in the C translation unit and
// therefore have no linker symbol: their logic is reproduced here.

#![allow(clippy::all)]

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RMM: i32 = 4;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a_in: u32, d: i32) -> u32 {
    let mut a = a_in;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = ((1u32) << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn roundpack_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: i32, pfflags: *mut u32) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN (2) and RM_RUP (3)
            if a_sign ^ ((rm & 1) as u32) != 0 {
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

fn normalize_sf32(a_sign: u32, a_exp_in: i32, a_mant_in: u32, rm: i32, pfflags: *mut u32) -> u32 {
    // shift = clz32(a_mant) - (32 - 1 - (32 - 2)); C asserts shift >= 0.
    let shift = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

// Equivalent of internal_cvt_i32_sf32(a, rm, pfflags, FALSE).
fn internal_cvt_i32_sf32(a: i32, rm: i32, pfflags: *mut u32) -> u32 {
    let is_unsigned: i32 = 0; // FALSE
    let a_sign: u32;
    let mut r: u32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    // l = 32 - clz32(r) - (32 - 1)
    let l = 1 - clz32(r);
    if l > 0 {
        let mask = r & ((1u32 << (l as u32)) - 1);
        r = (r >> (l as u32)) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant = r;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i32_sf32(a: i32, rm: core::ffi::c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i32_sf32(a, rm as i32, pfflags)
}
}

// ── `cvt_i64_sf32` · softfp_template_icvt.h:155-160 · chunk 3334 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_i64_sf32(int64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf32(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3334 {
// Port of cvt_i64_sf32 (i64 -> binary32) plus the static helper chain
// internal_cvt_i64_sf32 / normalize_sf32 / roundpack_sf32 / rshift_rnd32,
// which have internal linkage in the C original.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RMM: c_int = 4;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
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
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a_in: u32, d: i32) -> u32 {
    let mut a = a_in;
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
    let mut rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (((32 - 2) - 23) - 1);
    } else if rm == 1 {
        // RM_RTZ
        addend = 0;
    } else if (a_sign ^ ((rm as u32) & 1)) != 0 {
        addend = (1u32 << ((32 - 2) - 23)) - 1;
    } else {
        addend = 0;
    }

    if a_exp <= 0 {
        let is_subnormal =
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

    a_exp = a_exp.wrapping_add((a_mant >> (23 + 1)) as i32);
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
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift = clz32(a_mant) - (32 - 1 - (32 - 2));
    // shift >= 0 is guaranteed (clz32 <= 32)
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf32(a: i64, rm: c_int, pfflags: *mut u32) -> u32 {
    // is_unsigned == FALSE for this entry point
    let a_sign: u32;
    let mut r: u64;

    if a < 0 {
        a_sign = 1;
        r = a.wrapping_neg() as u64;
    } else {
        a_sign = 0;
        r = a as u64;
    }

    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: i32 = 64 - clz64(r) - (32 - 1);
    if l > 0 {
        let mask = r & ((1u64 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u64;
        a_exp = a_exp.wrapping_add(l);
    }
    let a_mant = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf32(a: i64, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i64_sf32(a, rm, pfflags)
}
}

// ── `cvt_i128_sf32` · softfp_template_icvt.h:155-160 · chunk 3356 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_i128_sf32(int128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3356 {
// Port of cvt_i128_sf32 (softfp_template_icvt.h lines 155-160).
//
// The C body is:
//     return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);
// internal_cvt_i128_sf32, normalize_sf32, roundpack_sf32, pack_sf32,
// rshift_rnd32 and clz32 are all `static` (or static inline) in the C
// translation unit, so they have no linker symbol and their logic is
// reproduced here as private Rust functions.

#![allow(non_camel_case_types)]

use core::ffi::c_uint;

// RoundingModeEnum (C enum, non-negative values -> unsigned int)
const RM_RNE: c_uint = 0;
const RM_RTZ: c_uint = 1;
const RM_RDN: c_uint = 2;
const RM_RUP: c_uint = 3;
const RM_RMM: c_uint = 4;

// BOOL / FALSE / TRUE
const FALSE: i32 = 0;

#[inline]
fn clz32(a: u32) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn clz128(a: u128) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & (((1u32) << 23) - 1))
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    if d != 0 {
        if d >= 32 {
            (a != 0) as u32
        } else if d < 0 {
            // Not reachable from the caller (d >= 1 there); mirrors the
            // "no shift" behaviour rather than performing an invalid shift.
            a
        } else {
            let dd = d as u32;
            let mask = (1u32 << dd) - 1;
            (a >> dd) | ((a & mask) != 0) as u32
        }
    } else {
        a
    }
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
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
            // default: RM_RDN, RM_RUP and anything else
            if a_sign ^ (rm & 1) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((32 - 2) - 23);

    if rm == RM_RNE && rnd_bits == (1u32 << (((32 - 2) - 23) - 1)) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as i32;
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
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    a_exp: i32,
    a_mant: u32,
    rm: c_uint,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // C asserts shift >= 0 here.
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

// Port of static internal_cvt_i128_sf32(a, rm, pfflags, FALSE)
fn internal_cvt_i128_sf32(
    a: i128,
    rm: c_uint,
    pfflags: *mut u32,
    is_unsigned: i32,
) -> u32 {
    let a_sign: u32;
    let mut r: u128;

    if is_unsigned == FALSE && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    // need to reduce range before generic float normalization
    let l: i32 = 128 - clz128(r) - (32 - 1);
    if l > 0 {
        let lu = l as u32;
        let mask: u128 = r & (1u128.wrapping_shl(lu) - 1);
        r = (r >> lu) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf32(a: i128, rm: c_uint, pfflags: *mut u32) -> u32 {
    internal_cvt_i128_sf32(a, rm, pfflags, FALSE)
}
}

// ── `cvt_i32_sf64` · softfp_template_icvt.h:155-160 · chunk 3378 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_i32_sf64(int32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf64(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3378 {
// Port of cvt_i32_sf64 (softfp_template_icvt.h) with its static helpers
// (internal_cvt_i32_sf64, normalize_sf64, roundpack_sf64, pack_sf64, clz*) inlined.

use core::ffi::c_int;

type RoundingModeEnum = c_int;

const RM_RNE: RoundingModeEnum = 0;
const RM_RTZ: RoundingModeEnum = 1;
const RM_RDN: RoundingModeEnum = 2;
const RM_RUP: RoundingModeEnum = 3;
const RM_RMM: RoundingModeEnum = 4;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
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

fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
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

fn roundpack_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
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
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << ((64 - 2) - 52)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal = a_exp < 0
            || (a_mant.wrapping_add(addend as u64)) < (1u64 << (64 - 1));
        let diff = 1 - a_exp;
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
    mut a_exp: i32,
    mut a_mant: u64,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
) -> u64 {
    let shift = clz64(a_mant) - (64 - 1 - (64 - 2));
    // assert(shift >= 0) holds for all inputs reaching here.
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf64(
    a: i32,
    rm: RoundingModeEnum,
    pfflags: *mut u32,
    is_unsigned: i32,
) -> u64 {
    let a_sign: u32;
    let mut a_exp: i32;
    let mut r: u32;
    let mut l: i32;
    let a_mant: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = (((1 << 11) - 1) / 2) + 64 - 2;

    l = 32 - clz32(r) - (64 - 1);
    if l > 0 {
        let mask: u32 = r & ((1u32.wrapping_shl(l as u32)).wrapping_sub(1));
        r = (r >> l) | ((r & mask) != 0) as u32;
        a_exp = a_exp.wrapping_add(l);
    }
    a_mant = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i32_sf64(a: i32, rm: RoundingModeEnum, pfflags: *mut u32) -> u64 {
    internal_cvt_i32_sf64(a, rm, pfflags, 0)
}
}

// ── `cvt_i64_sf64` · softfp_template_icvt.h:155-160 · chunk 3400 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_i64_sf64(int64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf64(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3400 {
// Port of cvt_i64_sf64 (softfp_template_icvt.h / softfp_template.h, 64-bit int -> sf64).
// The C helpers internal_cvt_i64_sf64, normalize_sf64, roundpack_sf64, pack_sf64,
// rshift_rnd64 and clz64 are all `static` in the translation unit, so their logic
// is reproduced here.

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RDN: i32 = 2;
const RM_RUP: i32 = 3;
const RM_RMM: i32 = 4;

// FFLAG_INEXACT / FFLAG_UNDERFLOW / FFLAG_OVERFLOW
const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

const EXP_MASK: i32 = (1 << 11) - 1; // 2047
const MANT_DIG: i32 = (64 - 2) - 52; // 10 (RND_size)
const MANT_MASK64: u64 = (1u64 << 52) - 1;

fn clz64(a: u64) -> i32 {
    a.leading_zeros() as i32
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & MANT_MASK64)
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

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: i32,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;
    let mut rnd_bits: u32;
    let rnd_mask: u64 = (1u64 << MANT_DIG) - 1;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (MANT_DIG - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << MANT_DIG) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & rnd_mask) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & rnd_mask) as u32;
    }

    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> MANT_DIG;

    if rm == RM_RNE && rnd_bits == (1u32 << (MANT_DIG - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= MANT_MASK64 {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = MANT_MASK64;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= FFLAG_OVERFLOW | FFLAG_INEXACT;
        }
    }
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: i32, pfflags: *mut u32) -> u64 {
    let shift = clz64(a_mant_in) - (64 - 1 - (64 - 2));
    if shift < 0 {
        // the C code asserts shift >= 0; abort instead of unwinding across FFI
        std::process::abort();
    }
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = if shift < 64 { a_mant_in << shift } else { 0 };
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf64(a: i64, rm: i32, pfflags: *mut u32) -> u64 {
    // internal_cvt_i64_sf64(a, rm, pfflags, FALSE)
    let a_sign: u32;
    let mut r: u64;

    if a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    let mut a_exp: i32 = (EXP_MASK / 2) + 64 - 2;

    let l: i32 = 64 - clz64(r) - (64 - 1);
    if l > 0 {
        let mask = r & ((1u64 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    normalize_sf64(a_sign, a_exp, r, rm, pfflags)
}
}

// ── `cvt_i128_sf64` · softfp_template_icvt.h:155-160 · chunk 3422 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_i128_sf64(int128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf64(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3422 {
// Port of softfp_template_icvt.h: cvt_i128_sf64(int128_t, RoundingModeEnum, uint32_t *)
// and the static helpers it depends on (internal_cvt_i128_sf64, normalize_sf64,
// roundpack_sf64, rshift_rnd64, pack_sf64, clz128).

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

#[inline]
fn clz128(a: u128) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & (((1u64) << 52) - 1))
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 || d < 0 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let du = d as u32;
            let mask = ((1u64) << du) - 1;
            a = (a >> du) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn roundpack_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let addend: u32;
    match rm {
        RM_RNE | RM_RMM => {
            addend = (1i32 << (((64 - 2) - 52) - 1)) as u32;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and any other value
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = ((1i32 << ((64 - 2) - 52)) - 1) as u32;
            } else {
                addend = 0;
            }
        }
    }

    let addend64 = addend as u64;
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let rnd_bits: u64;

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend64) < ((1u64) << (64 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = a_mant & (((1u64) << ((64 - 2) - 52)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & (((1u64) << ((64 - 2) - 52)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == ((1u64) << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= (((1u64) << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = ((1u64) << 52) - 1;
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

fn normalize_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let shift = (a_mant.leading_zeros() as i32) - (64 - 1 - (64 - 2));
    // C version asserts shift >= 0 (always true for the reachable inputs).
    let a_exp = a_exp - shift;
    let mut a_mant = a_mant;
    if shift >= 0 && shift < 64 {
        a_mant <<= shift as u32;
    }
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i128_sf64(a: i128, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u64 {
    let a_sign: u32;
    let mut a_exp: i32;
    let mut r: u128;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = 0u128.wrapping_sub(a as u128);
    } else {
        a_sign = 0;
        r = a as u128;
    }

    a_exp = (((1i32 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 128 - clz128(r) - (64 - 1);
    if l > 0 {
        let lu = l as u32;
        let mask = r & (((1u128) << lu) - 1);
        r = (r >> lu) | (if (r & mask) != 0 { 1u128 } else { 0u128 });
        a_exp += l;
    }

    let a_mant = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf64(a: i128, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i128_sf64(a, rm, pfflags, false)
}
}

// ── `cvt_i32_sf128` · softfp_template_icvt.h:155-160 · chunk 3444 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_i32_sf128(int32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf128(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3444 {
// Port of cvt_i32_sf128 (softfp_template_icvt.h, ICVT_SIZE=32, F_SIZE=128).
// The C implementation delegates to the static helper internal_cvt_i32_sf128,
// which in turn uses the static helpers normalize_sf128 / roundpack_sf128 /
// pack_sf128 / rshift_rnd128 / clz32 / clz128.  None of those have linker
// symbols, so the whole chain is reproduced here as private Rust functions.

use core::ffi::c_int;

// ---- format constants for sf128 (F_SIZE = 128) ----------------------------
const EXP_MASK: i32 = (1 << 15) - 1; // 32767
const F_SIZE: i32 = 128;
const F_MANT_MASK: u128 = (1u128 << 112) - 1; // 2^112 - 1
const RND_MASK: u128 = (1u128 << ((128 - 2) - 112)) - 1; // 2^14 - 1
const ICVT_A_EXP_INIT: i32 = (EXP_MASK / 2) + F_SIZE - 2;

// RoundingModeEnum values
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// ---- static inline int clz32(uint32_t a) ---------------------------------
fn clz32(a: u32) -> i32 {
    let r;
    if a == 0 {
        r = 32;
    } else {
        r = a.leading_zeros() as i32;
    }
    r
}

// ---- static inline int clz128(uint128_t a) -------------------------------
fn clz128(a: u128) -> i32 {
    let r;
    if a == 0 {
        r = 128;
    } else {
        let ah: u64 = (a >> 64) as u64;
        let al: u64 = a as u64;
        if ah != 0 {
            r = ah.leading_zeros() as i32;
        } else {
            r = al.leading_zeros() as i32 + 64;
        }
    }
    r
}

// ---- static inline uint128_t pack_sf128(...) -----------------------------
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & F_MANT_MASK)
}

// ---- static uint128_t rshift_rnd128(uint128_t a, int d) ------------------
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    if d <= 0 {
        a
    } else if d >= 128 {
        if a != 0 {
            1
        } else {
            0
        }
    } else {
        let du = d as u32;
        let mask: u128 = (1u128 << du) - 1;
        (a >> du) | if (a & mask) != 0 { 1 } else { 0 }
    }
}

// ---- static uint128_t roundpack_sf128(...) -------------------------------
fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((128 - 2) - 112) - 1),
        RM_RTZ => 0,
        _ => {
            // default: RM_RDN, RM_RUP (and anything else)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
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
        let diff: i32 = 1 - a_exp;
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
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> (112 + 1)) as i32;
    if a_mant <= F_MANT_MASK {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = F_MANT_MASK;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

// ---- static uint128_t normalize_sf128(...) -------------------------------
fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    // The C version asserts shift >= 0; for this conversion path a_mant always
    // has a zero top bit, so shift can never be negative.
    if shift > 0 {
        a_exp -= shift;
        a_mant <<= shift as u32;
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// ---- static uint128_t internal_cvt_i32_sf128(...) ------------------------
fn internal_cvt_i32_sf128(
    a: i32,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: bool,
) -> u128 {
    let a_sign: u32;
    let mut r: u32;
    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    let mut a_exp: i32 = ICVT_A_EXP_INIT;

    // need to reduce range before generic float normalization
    let l: i32 = 32 - clz32(r) - (F_SIZE - 1);
    if l > 0 {
        let lu = l as u32;
        let mask: u32 = r & 1u32.wrapping_shl(lu).wrapping_sub(1);
        r = r.wrapping_shr(lu) | if (r & mask) != 0 { 1 } else { 0 };
        a_exp += l;
    }
    let a_mant: u128 = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// ---- uint128_t cvt_i32_sf128(int32_t a, RoundingModeEnum rm, uint32_t *pfflags)
#[no_mangle]
pub extern "C" fn cvt_i32_sf128(a: i32, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_i32_sf128(a, rm, pfflags, false)
}
}

// ── `cvt_i64_sf128` · softfp_template_icvt.h:155-160 · chunk 3466 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_i64_sf128(int64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf128(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3466 {
// Rust port of cvt_i64_sf128 (softfp_template_icvt.h), together with the
// private helpers it needs (internal_cvt_i64_sf128, normalize_sf128,
// roundpack_sf128, rshift_rnd128, clz128, clz64, pack_sf128), which are all
// `static` in the C translation unit and therefore have no linker symbol.
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ffi::c_int;

// RoundingModeEnum values.
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const F_SIZE: i32 = 128;
const EXP_MASK: i32 = (1 << 15) - 1;
// (F_SIZE - 2) - 112
const RND_SHIFT: u32 = 14;

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
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

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (F_SIZE as u32 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

fn set_flags(pfflags: *mut u32, f: u32) {
    if !pfflags.is_null() {
        unsafe {
            *pfflags |= f;
        }
    }
}

fn roundpack_sf128(a_sign: u32, a_exp: i32, a_mant: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (RND_SHIFT - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << RND_SHIFT) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u128;
    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (F_SIZE as u32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = a_mant & ((1u128 << RND_SHIFT) - 1);
        if is_subnormal && rnd_bits != 0 {
            set_flags(pfflags, 1 << 1);
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u128 << RND_SHIFT) - 1);
    }
    if rnd_bits != 0 {
        set_flags(pfflags, 1 << 0);
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> RND_SHIFT;

    if rm == RM_RNE && rnd_bits == (1u128 << (RND_SHIFT - 1)) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> 113) as i32;
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        set_flags(pfflags, (1 << 2) | (1 << 0));
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(a_sign: u32, a_exp: i32, a_mant: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    // C version asserts shift >= 0; clz128() is always >= 0 and a_mant == 0
    // yields 128, so shift is always non-negative for the reachable inputs.
    let shift = clz128(a_mant) - (F_SIZE - 1 - (F_SIZE - 2));
    let shift = if shift < 0 { 0 } else { shift };
    let a_exp = a_exp - shift;
    let a_mant = a_mant << (shift as u32);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf128(a: i64, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> u128 {
    let a_sign: u32;
    let mut r: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    let mut a_exp: i32 = (EXP_MASK / 2) + F_SIZE - 2;

    let l: i32 = 64 - clz64(r) - (F_SIZE - 1);
    if l > 0 {
        let mask = r & ((1u64 << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    let a_mant = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf128(a: i64, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_i64_sf128(a, rm, pfflags, 0)
}
}

// ── `cvt_i128_sf128` · softfp_template_icvt.h:155-160 · chunk 3488 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_i128_sf128(int128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf128(a, rm, pfflags, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3488 {
// Port of softfp_template_icvt.h cvt_i128_sf128() for ICVT_SIZE == 128,
// F_SIZE == 128 (binary128).  The static helpers it uses
// (internal_cvt_i128_sf128, normalize_sf128, roundpack_sf128,
// rshift_rnd128, pack_sf128, clz128) have no linker symbols, so their logic
// is reproduced here as private Rust functions.

extern "C" {
    fn abort() -> !;
}

// RoundingModeEnum values (see softfp.h).
const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RDN: u32 = 2;
const RM_RUP: u32 = 3;
const RM_RMM: u32 = 4;

// FFLAG bits used below: (1 << 0) inexact, (1 << 1) underflow, (1 << 2) overflow.
const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

const F_SIZE: i32 = 128;
const MANT_SIZE: i32 = 112;
const RND_SIZE: i32 = (F_SIZE - 2) - MANT_SIZE; // 14
const EXP_MASK: i32 = (1 << 15) - 1; // 32767

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn rshift_rnd128(mut a: u128, d: i32) -> u128 {
    if d != 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let mask: u128 = (1u128 << d) - 1;
            a = (a >> d) | ((a & mask != 0) as u128);
        }
    }
    a
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (F_SIZE as u32 - 1))
        | ((a_exp as u128) << (MANT_SIZE as u32))
        | (a_mant & ((1u128 << (MANT_SIZE as u32)) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (RND_SIZE - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ (rm & 1)) != 0 {
                addend = (1u32 << RND_SIZE) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (F_SIZE as u32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << RND_SIZE) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << RND_SIZE) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> RND_SIZE;

    if rm == RM_RNE && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> (MANT_SIZE as u32 + 1)) as i32;
    if a_mant <= ((1u128 << (MANT_SIZE as u32)) - 1) {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = (1u128 << (MANT_SIZE as u32)) - 1;
        } else {
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
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;

    let shift: i32 = clz128(a_mant) - (F_SIZE - 1 - (F_SIZE - 2));
    // assert(shift >= 0)
    if shift < 0 {
        unsafe {
            abort();
        }
    }
    a_exp -= shift;
    a_mant <<= shift as u32;
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// internal_cvt_i128_sf128(a, rm, pfflags, FALSE)
fn internal_cvt_i128_sf128(a: i128, rm: u32, pfflags: *mut u32) -> u128 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u128;
    let mut r: u128;

    if a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    a_exp = (EXP_MASK / 2) + F_SIZE - 2;
    // need to reduce range before generic float normalization
    let l: i32 = 128 - clz128(r) - (F_SIZE - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128 << l) - 1);
        r = (r >> l) | ((r & mask != 0) as u128);
        a_exp += l;
    }
    a_mant = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf128(a: i128, rm: u32, pfflags: *mut u32) -> u128 {
    internal_cvt_i128_sf128(a, rm, pfflags)
}
}

// ── `return internal_cvt_i32_sf32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3313 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf32(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3313 {
// Port of the target `return internal_cvt_i32_sf32(a, rm, pfflags, FALSE);`
// from cvt_i32_sf32 (softfp_template_icvt.h).  The static helpers it needs
// (internal_cvt_i32_sf32, normalize_sf32, roundpack_sf32, rshift_rnd32,
// clz32, pack_sf32) have no linker symbols, so their logic is ported here.

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
    a_sign.wrapping_shl(31 as u32) | a_exp.wrapping_shl(23 as u32) | (a_mant & ((1u32 << 23) - 1))
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
    a_exp_in: c_int,
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
            addend = 1u32 << 6;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and anything else
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff = 1 - a_exp;
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

    if rm == 0 && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> 24) as c_int;
    if a_mant <= ((1u32 << 23) - 1) {
        a_exp = 0;
    } else if a_exp >= 255 {
        if addend == 0 {
            a_exp = 254;
            a_mant = (1u32 << 23) - 1;
        } else {
            a_exp = 255;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | 1;
        }
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf32(a_sign: u32, a_exp: c_int, a_mant: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    let shift = clz32(a_mant) - 1;
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
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf32(a: c_int, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> u32 {
    let a_sign: u32;
    let mut r: u32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    let mut a_exp: c_int = (255 / 2) + 32 - 2;

    let l = 32 - clz32(r) - (32 - 1);
    if l > 0 {
        let mask = if l >= 32 {
            u32::MAX
        } else {
            ((1u32 << (l as u32)) - 1) & r
        };
        r = (r >> (l as u32)) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    let a_mant = r;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i32_sf32__part3313(a: c_int, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i32_sf32(a, rm, pfflags, 0)
}
}

// ── `return internal_cvt_i64_sf32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3335 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf32(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3335 {
// Port of the body of cvt_i64_sf32 (softfp icvt), which forwards to the
// static internal_cvt_i64_sf32(a, rm, pfflags, FALSE).  All statics it used
// (normalize_sf32, roundpack_sf32, rshift_rnd32, pack_sf32, clz32, clz64)
// are reproduced here as private Rust fns with the same semantics.

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf32(a_sign: u32, a_exp: i32, a_mant: u32) -> u32 {
    (a_sign << 31) | ((a_exp as u32) << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    if d <= 0 {
        return a;
    }
    if d >= 32 {
        return (a != 0) as u32;
    }
    let mask = (1u32 << (d as u32)) - 1;
    a.wrapping_shr(d as u32) | ((a & mask) != 0) as u32
}

fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: i32,
    pfflags: *mut u32,
) -> u32 {
    // (32 - 2) - 23 == 7, ((32 - 2) - 23) - 1 == 6
    let addend: u32 = match rm {
        0 | 4 => 1u32 << 6, // RM_RNE, RM_RMM
        1 => 0,             // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP and default
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 7) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff = 1 - a_exp;
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

    if rm == 0 && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> 24) as i32;
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
    pack_sf32(a_sign, a_exp, a_mant)
}

fn normalize_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: i32,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // assert(shift >= 0) holds for every input reaching here.
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf32(a: i64, rm: i32, pfflags: *mut u32, is_unsigned: bool) -> u32 {
    let a_sign: u32;
    let mut r: u64;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: i32 = 64 - clz64(r) - (32 - 1);
    if l > 0 && l < 64 {
        let mask: u64 = r & ((1u64 << (l as u32)) - 1);
        r = r.wrapping_shr(l as u32) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf32__part3335(
    a: i64,
    rm: core::ffi::c_int,
    pfflags: *mut u32,
) -> u32 {
    internal_cvt_i64_sf32(a, rm, pfflags, false)
}
}

// ── `return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3357 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3357 {
// Port of the TARGET `return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);`
// internal_cvt_i128_sf32 (and the static helpers it calls) has no linker
// symbol, so the whole chain is reproduced here as private Rust functions.

use core::ffi::c_int;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

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

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn rshift_rnd32(a: u32, d: c_int) -> u32 {
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
    a_exp_in: c_int,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    // RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4
    match rm {
        0 | 4 => {
            addend = 1u32 << 6;
        }
        1 => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP (and anything else)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff = 1 - a_exp;
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

    if rm == 0 && rnd_bits == (1u32 << 6) {
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

fn normalize_sf32(
    a_sign: u32,
    a_exp_in: c_int,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // C asserts shift >= 0; that always holds for this call site.
    let mut shift = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    if shift < 0 {
        shift = 0;
    }
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = a_mant_in << (shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i128_sf32(a: i128, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> u32 {
    let a_sign: u32;
    let mut r: u128;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    let mut a_exp: c_int = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: c_int = 128 - clz128(r) - (32 - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128 << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u128;
        a_exp = a_exp.wrapping_add(l);
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf32__part3357(a: i128, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i128_sf32(a, rm, pfflags, 0)
}
}

// ── `return internal_cvt_i32_sf64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3379 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf64(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3379 {
// Port of cvt_i32_sf64's TARGET: internal_cvt_i32_sf64(a, rm, pfflags, FALSE)
// plus the file-static helpers it needs (normalize_sf64, roundpack_sf64,
// rshift_rnd64, pack_sf64, clz32, clz64), all private here.

use core::ffi::c_int;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
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

fn roundpack_sf64(a_sign: u32, a_exp_in: i32, a_mant_in: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u64 = a_mant_in;

    let addend: u32 = match rm {
        0 | 4 => 1u32 << (((64 - 2) - 52) - 1),
        1 => 0,
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
            a_exp < 0 || (a_mant + (addend as u64)) < (1u64 << (64 - 1));
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
    a_mant = (a_mant + (addend as u64)) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
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

fn normalize_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf64(a: i32, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u64 {
    let a_sign: u32;
    let mut r: u32;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    let mut a_exp: i32 = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 32 - clz32(r) - (64 - 1);
    if l > 0 {
        let mask: u32 = r & (1u32.wrapping_shl(l as u32)).wrapping_sub(1);
        r = (r >> l) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i32_sf64__part3379(a: i32, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i32_sf64(a, rm, pfflags, false)
}
}

// ── `return internal_cvt_i64_sf64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3401 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf64(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3401 {
// Port of the C body:
//   static uint64_t internal_cvt_i64_sf64(int64_t a, RoundingModeEnum rm,
//                                        uint32_t *pfflags, BOOL is_unsigned)
// specialized for ICVT_SIZE == 64 and F_SIZE == 64, called with is_unsigned == FALSE.
// The static helpers it used (clz64, normalize_sf64, roundpack_sf64, pack_sf64,
// rshift_rnd64) have no linker symbols, so their logic is mirrored here.

use core::ffi::c_int;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;
const FFLAG_INVALID_OP: u32 = 1 << 4;

#[inline]
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
            let sh = d as u32;
            let mask = (1u64 << sh) - 1;
            a = (a >> sh) | ((a & mask != 0) as u64);
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
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = match rm {
        0 | 4 => 1 << 9, // RM_RNE, RM_RMM
        1 => 0,          // RM_RTZ
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 10) - 1
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
            unsafe {
                *pfflags |= FFLAG_UNDERFLOW;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << 10) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= FFLAG_INEXACT;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    if rm == 0 && rnd_bits == (1 << 9) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 53) as i32);
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
    a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift = clz64(a_mant) - 1;
    // C asserts shift >= 0
    if shift < 0 {
        return roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags);
    }
    let new_exp = a_exp.wrapping_sub(shift);
    let new_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, new_exp, new_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf64__part3401(a: i64, rm: c_int, pfflags: *mut u32) -> u64 {
    let mut a_exp: i32 = 1023 + 64 - 2;

    // is_unsigned == FALSE
    let (a_sign, mut r): (u32, u64) = if a < 0 {
        (1u32, (a as u64).wrapping_neg())
    } else {
        (0u32, a as u64)
    };

    let l: i32 = 64 - clz64(r) - (64 - 1);
    if l > 0 {
        let sh = l as u32;
        let mask = r & ((1u64 << sh) - 1);
        r = (r >> sh) | ((r & mask != 0) as u64);
        a_exp = a_exp.wrapping_add(l);
    }
    let a_mant = r;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return internal_cvt_i128_sf64(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3423 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf64(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3423 {
// Port of cvt_i128_sf64 (softfp_template_icvt.h) targeting the single
// statement "return internal_cvt_i128_sf64(a, rm, pfflags, FALSE);".
// internal_cvt_i128_sf64 is a C static function (no linker symbol), so its
// logic and the static helpers it needs (normalize_sf64, roundpack_sf64,
// rshift_rnd64, clz64/128, pack_sf64) are reproduced here verbatim.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const FFLAG_INEXACT: u32 = 1 << 0;
const FFLAG_UNDERFLOW: u32 = 1 << 1;
const FFLAG_OVERFLOW: u32 = 1 << 2;

#[inline]
fn clz64(a: u64) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn clz128(a: u128) -> i32 {
    a.leading_zeros() as i32
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
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

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    // (64 - 2) - 52 == 10
    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((64 - 2) - 52) - 1),
        RM_RTZ => 0,
        _ => {
            // default: / RM_RDN / RM_RUP
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                (1u32 << ((64 - 2) - 52)) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u64) < (1u64 << (64 - 1));
        let diff: i32 = 1 - a_exp;
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

    // C: a_exp += a_mant >> (52 + 1);  (int += uint64_t -> uint64_t math,
    // then truncated back into the int object)
    a_exp = ((a_exp as i64 as u64).wrapping_add(a_mant >> (52 + 1)) as u32) as i32;

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

fn normalize_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    // shift = clz64(a_mant) - (64 - 1 - (64 - 2));  assert(shift >= 0)
    let shift: i32 = clz64(a_mant_in) - (64 - 1 - (64 - 2));
    let a_exp = a_exp_in.wrapping_sub(shift);
    let a_mant = a_mant_in << shift;
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

// internal_cvt_i128_sf64(a, rm, pfflags, FALSE)
fn internal_cvt_i128_sf64(a: i128, rm: c_int, pfflags: *mut u32) -> u64 {
    let a_sign: u32;
    let mut r: u128;
    if a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    let mut a_exp: i32 = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 128 - clz128(r) - (64 - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf64__part3423(a: i128, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i128_sf64(a, rm, pfflags)
}
}

// ── `return internal_cvt_i32_sf128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3445 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf128(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3445 {
// Port of the C body of cvt_i32_sf128(): the call
//   internal_cvt_i32_sf128(a, rm, pfflags, FALSE)
// internal_cvt_i32_sf128 is a `static` C function, so its logic (and the
// static helpers it uses: clz32, clz128, pack_sf128, rshift_rnd128,
// roundpack_sf128, normalize_sf128) is ported here as private Rust fns.
//
// Only `core` is used; no panics can cross the FFI boundary.

use core::ffi::c_int;

const BOOL_FALSE: c_int = 0;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

const MANT_MASK_112: u128 = (1u128 << 112) - 1;
const RND_MASK_14: u128 = (1u128 << 14) - 1;

#[inline]
fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK_112)
}

#[inline]
fn set_flags(pfflags: *mut u32, bits: u32) {
    if !pfflags.is_null() {
        unsafe {
            *pfflags |= bits;
        }
    }
}

fn rshift_rnd128(mut a: u128, d: c_int) -> u128 {
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let mask = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
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
    let mut rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << 13;
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        // default: RM_RDN / RM_RUP / anything else
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << 14) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & RND_MASK_14) as u32;
        if is_subnormal && rnd_bits != 0 {
            set_flags(pfflags, 1 << 1);
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & RND_MASK_14) as u32;
    }
    if rnd_bits != 0 {
        set_flags(pfflags, 1 << 0);
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 14;

    if rm == RM_RNE && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add(((a_mant >> 113) as u32) as c_int);
    if a_mant <= MANT_MASK_112 {
        a_exp = 0;
    } else if a_exp >= (1 << 15) - 1 {
        if addend == 0 {
            a_exp = (1 << 15) - 2;
            a_mant = MANT_MASK_112;
        } else {
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        set_flags(pfflags, (1 << 2) | (1 << 0));
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    // The C code asserts shift >= 0 here; it always holds because a_mant
    // comes from a 32-bit value (clz128 >= 96), so the assert never fires.
    if shift >= 0 {
        a_exp -= shift;
        a_mant = a_mant.wrapping_shl(shift as u32);
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf128(a: i32, rm: c_int, pfflags: *mut u32, is_unsigned: c_int) -> u128 {
    let a_sign: u32;
    let mut a_exp: c_int;
    let mut r: u32;
    let mut l: c_int;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = 0u32.wrapping_sub(a as u32);
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = ((1 << 15) - 1) / 2 + 128 - 2;

    l = 32 - clz32(r) - (128 - 1);
    if l > 0 {
        let mask: u32 = r & ((1u32).wrapping_shl(l as u32)).wrapping_sub(1);
        r = (r >> (l as u32)) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant: u128 = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i32_sf128__part3445(
    a: i32,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    internal_cvt_i32_sf128(a, rm, pfflags, BOOL_FALSE)
}
}

// ── `return internal_cvt_i64_sf128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3467 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf128(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3467 {
// Port of the static C helpers needed by internal_cvt_i64_sf128
// (softfp_template_icvt.h / softfp_template.h), all static in the TU so they
// are re-implemented here as private Rust functions.

const RM_RNE: i32 = 0;
const RM_RTZ: i32 = 1;
const RM_RDN: i32 = 2;
const RM_RUP: i32 = 3;
const RM_RMM: i32 = 4;

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

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

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u128 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((128 - 2) - 112) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((128 - 2) - 112)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (112 + 1)) as i32);
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
    a_exp_in: i32,
    a_mant_in: u128,
    rm: i32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    // C asserts shift >= 0; keep it panic-free either way.
    if shift > 0 {
        a_exp = a_exp.wrapping_sub(shift);
        if shift >= 128 {
            a_mant = 0;
        } else {
            a_mant <<= shift as u32;
        }
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf128(a: i64, rm: i32, pfflags: *mut u32, is_unsigned: bool) -> u128 {
    let a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u128;
    let mut r: u64;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    a_exp = (((1 << 15) - 1) / 2) + 128 - 2;

    let l: i32 = 64 - clz64(r) - (128 - 1);
    if l > 0 {
        let mask = r & ((1u64 << (l as u32)) - 1);
        r = (r >> (l as u32)) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp = a_exp.wrapping_add(l);
    }
    a_mant = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i64_sf128__part3467(
    a: i64,
    rm: core::ffi::c_int,
    pfflags: *mut u32,
) -> u128 {
    internal_cvt_i64_sf128(a, rm, pfflags, false)
}
}

// ── `return internal_cvt_i128_sf128(a, rm, pfflags, FALSE);` · softfp_template_icvt.h:159-159 · chunk 3489 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf128(a, rm, pfflags, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3489 {
// Port of the internal_cvt_i128_sf128 body (static helper in C, no linker
// symbol) together with the static helpers it calls: clz128, normalize_sf128,
// roundpack_sf128, pack_sf128, rshift_rnd128.

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u128 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn roundpack_sf128(a_sign: u32, a_exp: i32, a_mant: u128, rm: i32, pfflags: *mut u32) -> u128 {
    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let addend: u32;
    let rnd_bits: u32;

    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << 14;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP (and default)
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 15) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let mask15: u128 = ((1u32 << 15) - 1) as u128;

    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & mask15) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & mask15) as u32;
    }

    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> 15;

    if rm == 0 && rnd_bits == (1u32 << 14) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> 113) as i32;
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1i32 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | (1u32 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(a_sign: u32, a_exp: i32, a_mant: u128, rm: i32, pfflags: *mut u32) -> u128 {
    // C: shift = clz128(a_mant) - (128 - 1 - (128 - 2)); assert(shift >= 0);
    let shift = clz128(a_mant) - 1;
    let a_exp = a_exp - shift;
    let a_mant = a_mant << shift;
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_i128_sf128__part3489(a: i128, rm: i32, pfflags: *mut u32) -> u128 {
    let a_sign: u32;
    let mut r: u128;

    if a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }
    let mut a_exp: i32 = (((1i32 << 15) - 1) / 2) + 128 - 2;

    let l = 128 - clz128(r) - 127;
    if l > 0 {
        let mask = r & ((1u128 << l) - 1);
        r = (r >> l) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `cvt_u32_sf32` · softfp_template_icvt.h:162-167 · chunk 3314 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_u32_sf32(uint32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf32(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3314 {
use core::ffi::c_int;

// RoundingModeEnum values (C enum, passed as an int)
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

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
    (a_sign.wrapping_shl(31)) | (a_exp.wrapping_shl(23)) | (a_mant & 0x7FFFFF)
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = (a != 0) as u32;
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
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
        RM_RNE | RM_RMM => addend = 1u32 << 6,
        RM_RTZ => addend = 0,
        _ => {
            // RM_RDN / RM_RUP (and default)
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 7) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
        let diff: i32 = 1i32.wrapping_sub(a_exp);
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
    if a_mant <= 0x7FFFFF {
        a_exp = 0;
    } else if a_exp >= 255 {
        if addend == 0 {
            a_exp = 255 - 1;
            a_mant = 0x7FFFFF;
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
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant) - 1;
    // shift >= 0 is guaranteed by the caller
    a_exp = a_exp.wrapping_sub(shift);
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

// Port of the static C helper internal_cvt_i32_sf32(a, rm, pfflags, TRUE)
fn internal_cvt_u32_sf32(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    // is_unsigned == TRUE, so the input is never treated as negative
    let a_sign: u32 = 0;
    let mut r: u32 = a;
    let mut a_exp: i32 = 157; // (EXP_MASK / 2) + F_SIZE - 2

    // need to reduce range before generic float normalization
    let l: i32 = 32 - clz32(r) - 31;
    if l > 0 {
        let mask: u32 = r & ((1u32 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    let a_mant: u32 = r;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf32(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_u32_sf32(a, rm, pfflags)
}
}

// ── `cvt_u64_sf32` · softfp_template_icvt.h:162-167 · chunk 3336 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_u64_sf32(uint64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf32(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3336 {
// Rust rewrite of cvt_u64_sf32 (softfp_template_icvt.h) with the static
// helpers it depends on ported as private Rust functions.
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
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
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u32 << d) - 1;
            a = (a >> d) | ((a & mask) != 0) as u32;
        }
    }
    a
}

unsafe fn roundpack_sf32(
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
        RM_RNE | 4 => {
            // RM_RNE, RM_RMM
            addend = 1u32 << (((32 - 2) - 23) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // RM_RDN, RM_RUP and anything else
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << ((32 - 2) - 23)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1u32 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        *pfflags |= 1u32 << 0;
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
        *pfflags |= (1u32 << 2) | (1u32 << 0);
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

unsafe fn internal_cvt_i64_sf32(
    a: i64,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: c_int,
) -> u32 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u32;
    let mut r: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    a_exp = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: i32 = 64 - clz64(r) - (32 - 1);
    if l > 0 {
        let mask: u64 = r & ((1u64 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    a_mant = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub unsafe extern "C" fn cvt_u64_sf32(a: u64, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i64_sf32(a as i64, rm, pfflags, 1)
}
}

// ── `cvt_u128_sf32` · softfp_template_icvt.h:162-167 · chunk 3358 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cvt_u128_sf32(uint128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf32(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3358 {
// Port of cvt_u128_sf32 (softfp_template_icvt.h) plus the static helpers it
// depends on (internal_cvt_i128_sf32, normalize_sf32, roundpack_sf32,
// rshift_rnd32, clz32, clz128, pack_sf32) which have no linker symbols.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

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
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
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

unsafe fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;
    let mut rnd_bits: u32;

    if rm == RM_RNE || rm == RM_RMM {
        addend = 1u32 << (((32 - 2) - 23) - 1);
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        // default, RM_RDN, RM_RUP
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = (1u32 << ((32 - 2) - 23)) - 1;
        } else {
            addend = 0;
        }
    }

    if a_exp <= 0 {
        let is_subnormal = a_exp < 0
            || a_mant.wrapping_add(addend) < (1u32 << (32 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd32(a_mant, diff);
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
        if is_subnormal && rnd_bits != 0 {
            *pfflags |= 1 << 1;
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & ((1u32 << ((32 - 2) - 23)) - 1);
    }
    if rnd_bits != 0 {
        *pfflags |= 1 << 0;
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
        *pfflags |= (1 << 2) | (1 << 0);
    }
    pack_sf32(a_sign, a_exp as u32, a_mant)
}

unsafe fn normalize_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    let a_exp = a_exp_in - shift;
    let a_mant = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub unsafe extern "C" fn cvt_u128_sf32(a: u128, rm: c_int, pfflags: *mut u32) -> u32 {
    // internal_cvt_i128_sf32(a, rm, pfflags, TRUE)
    let a_sign: u32 = 0;
    let mut r: u128 = a;
    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: i32 = 128 - clz128(r) - (32 - 1);
    if l > 0 {
        let lut = l as u32;
        let mask: u128 = r & ((1u128 << lut) - 1);
        r = (r >> lut) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `cvt_u32_sf64` · softfp_template_icvt.h:162-167 · chunk 3380 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_u32_sf64(uint32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf64(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3380 {
// Port of softfp_template_icvt.h: cvt_u32_sf64 (uint32_t -> sfloat64)
// which forwards to internal_cvt_i32_sf64(..., TRUE).
//
// The helpers normalize_sf64 / roundpack_sf64 / rshift_rnd64 / pack_sf64 /
// clz32 / clz64 are `static` in the C translation unit, so they have no
// linker symbol and are reproduced here as private Rust functions.

#![allow(dead_code)]
#![allow(non_snake_case)]

use core::ffi::c_int;

// RoundingModeEnum values (C enum, int-sized).
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

// sfloat64 layout parameters
const F_SIZE: i32 = 64;
const F_MANT: i32 = 52;
const F_EXP_BITS: i32 = 11;

#[inline]
fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
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
fn or_flags(pfflags: *mut u32, v: u32) {
    unsafe {
        if !pfflags.is_null() {
            *pfflags |= v;
        }
    }
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (F_SIZE as u32 - 1))
        | ((a_exp as u64) << (F_MANT as u32))
        | (a_mant & ((1u64 << (F_MANT as u32)) - 1))
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let du = d as u32;
            let mask: u64 = (1u64 << du) - 1;
            a = (a >> du) | ((a & mask) != 0) as u64;
        }
    }
    a
}

fn roundpack_sf64(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u64;

    // shift = (F_SIZE - 2) - F_MANT
    let shift: u32 = (F_SIZE - 2 - F_MANT) as u32;
    let rnd_mask: u64 = (1u64 << shift) - 1;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u64 << (shift - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP (and anything else)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = (1u64 << shift) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let mut rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u64 << (F_SIZE as u32 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & rnd_mask) as u32;
        if is_subnormal && rnd_bits != 0 {
            or_flags(pfflags, 1 << 1);
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & rnd_mask) as u32;
    }
    if rnd_bits != 0 {
        or_flags(pfflags, 1 << 0);
    }
    a_mant = a_mant.wrapping_add(addend) >> shift;

    if rm == RM_RNE && rnd_bits == (1u32 << (shift - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (F_MANT as u32 + 1)) as i32);
    if a_mant <= ((1u64 << (F_MANT as u32)) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << F_EXP_BITS) - 1) {
        if addend == 0 {
            a_exp = ((1 << F_EXP_BITS) - 1) - 1;
            a_mant = (1u64 << (F_MANT as u32)) - 1;
        } else {
            a_exp = (1 << F_EXP_BITS) - 1;
            a_mant = 0;
        }
        or_flags(pfflags, (1 << 2) | (1 << 0));
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
    let shift: i32 = clz64(a_mant) - (F_SIZE - 1 - (F_SIZE - 2));
    // upstream asserts shift >= 0 here (always true for its callers)
    let a_exp = a_exp.wrapping_sub(shift);
    let a_mant = if shift >= 0 && shift < 64 {
        a_mant << (shift as u32)
    } else if shift >= 64 {
        0
    } else {
        // negative shift is undefined behavior upstream; keep it total
        a_mant
    };
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf64(a: i32, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u64 {
    let a_sign: u32;
    let mut r: u32;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    let mut a_exp: i32 = (((1 << F_EXP_BITS) - 1) / 2) + F_SIZE - 2;

    let l: i32 = 32 - clz32(r) - (F_SIZE - 1);
    if l > 0 {
        let lu = l as u32;
        if lu < 32 {
            let mask: u32 = r & ((1u32 << lu) - 1);
            r = (r >> lu) | ((r & mask) != 0) as u32;
        } else {
            r = (r != 0) as u32;
        }
        a_exp += l;
    }
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf64(a: u32, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i32_sf64(a as i32, rm, pfflags, true)
}
}

// ── `cvt_u64_sf64` · softfp_template_icvt.h:162-167 · chunk 3402 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_u64_sf64(uint64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf64(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3402 {
// Port of cvt_u64_sf64 from softfp_template_icvt.h
// (uint64 -> binary64 software float conversion, unsigned source path).

use core::ffi::c_int;

// RoundingModeEnum values (C enum, int-sized)
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
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & (((1u64) << 52) - 1))
}

fn rshift_rnd64(a: u64, d: i32) -> u64 {
    let mut a = a;
    if d != 0 {
        if d >= 64 {
            a = (a != 0) as u64;
        } else {
            let mask: u64 = ((1u64) << (d as u32)) - 1;
            a = (a >> (d as u32)) | ((a & mask) != 0) as u64;
        }
    }
    a
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
    let addend: u64;

    if rm == RM_RNE || rm == RM_RMM {
        addend = (1u64) << (((64 - 2) - 52) - 1);
    } else if rm == RM_RTZ {
        addend = 0;
    } else {
        // default: RM_RDN, RM_RUP and anything else
        if (a_sign ^ ((rm as u32) & 1)) != 0 {
            addend = ((1u64) << ((64 - 2) - 52)) - 1;
        } else {
            addend = 0;
        }
    }

    let mut rnd_bits: u64;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < ((1u64) << (64 - 1));
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = a_mant & (((1u64) << ((64 - 2) - 52)) - 1);
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1u32 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = a_mant & (((1u64) << ((64 - 2) - 52)) - 1);
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == ((1u64) << (((64 - 2) - 52) - 1)) {
        a_mant &= !(1u64);
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
    if a_mant <= (((1u64) << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1 << 11) - 1) - 1;
            a_mant = ((1u64) << 52) - 1;
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

fn normalize_sf64(a_sign: u32, a_exp: i32, a_mant: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    // assert(shift >= 0) holds because a_mant != 0 here in all call paths used
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf64(a: i64, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u64 {
    let a_sign: u32;
    let mut r: u64;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    let mut a_exp: i32 = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 64 - clz64(r) - (64 - 1);
    if l > 0 {
        let mask: u64 = r & (((1u64) << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    let a_mant: u64 = r;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u64_sf64(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i64_sf64(a as i64, rm, pfflags, true)
}
}

// ── `cvt_u128_sf64` · softfp_template_icvt.h:162-167 · chunk 3424 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t cvt_u128_sf64(uint128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf64(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3424 {
// Port of softfp_template_icvt.h: cvt_u128_sf64 (uint128 -> binary64).
// internal_cvt_i128_sf64, normalize_sf64, roundpack_sf64, rshift_rnd64,
// pack_sf64 and clz64/clz128 are all `static` in the C translation unit, so
// their logic is reproduced here as private Rust functions.

use core::ffi::c_int;

// RoundingModeEnum values.
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const F_SIZE: i32 = 64;
const EXP_MASK: i32 = (1 << 11) - 1;

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

#[inline]
fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | ((a_exp as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
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
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let addend: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP
            if (a_sign as c_int) ^ (rm & 1) != 0 {
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
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == RM_RNE && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> (52 + 1)) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = (1u64 << 52) - 1;
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
    a_exp_in: i32,
    a_mant_in: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    // shift = clz64(a_mant) - (64 - 1 - (64 - 2));  C asserts shift >= 0.
    let shift: i32 = clz64(a_mant) - (F_SIZE - 1 - (F_SIZE - 2));
    if shift >= 0 {
        a_exp = a_exp.wrapping_sub(shift);
        a_mant = a_mant.wrapping_shl(shift as u32);
    }
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

// internal_cvt_i128_sf64(a, rm, pfflags, TRUE)
unsafe fn internal_cvt_i128_sf64(
    a: i128,
    rm: c_int,
    pfflags: *mut u32,
    is_unsigned: bool,
) -> u64 {
    let a_sign: u32;
    let mut r: u128;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u128;
    }

    let mut a_exp: i32 = (EXP_MASK / 2) + F_SIZE - 2;

    // ICVT_SIZE == 128
    let l: i32 = 128 - clz128(r) - (F_SIZE - 1);
    if l > 0 {
        let lu = l as u32;
        let mask: u128 = r & ((1u128 << lu) - 1);
        r = (r >> lu) | ((r & mask) != 0) as u128;
        a_exp = a_exp.wrapping_add(l);
    }
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u128_sf64(a: u128, rm: c_int, pfflags: *mut u32) -> u64 {
    unsafe { internal_cvt_i128_sf64(a as i128, rm, pfflags, true) }
}
}

// ── `cvt_u32_sf128` · softfp_template_icvt.h:162-167 · chunk 3446 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_u32_sf128(uint32_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i32_sf128(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3446 {
// Port of softfp_template_icvt.h: cvt_u32_sf128
//   return internal_cvt_i32_sf128(a, rm, pfflags, TRUE);
// internal_cvt_i32_sf128 (with is_unsigned == TRUE), normalize_sf128,
// roundpack_sf128, rshift_rnd128, clz128 and clz32 are `static` in C, so their
// logic is reproduced here as private Rust functions.

#![allow(non_snake_case)]

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
#[allow(dead_code)]
const RM_RDN: c_int = 2;
#[allow(dead_code)]
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

// F_SIZE = 128, M_SIZE = 112 -> (128 - 2) - 112 == 14
const RND_MASK: u32 = (1u32 << 14) - 1;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd128(a: u128, d: i32) -> u128 {
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

fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;

    // addend selection
    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << 13,
        RM_RTZ => 0,
        _ => {
            // default: RM_RDN, RM_RUP (and anything unexpected)
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 14) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || (a_mant + addend as u128) < (1u128 << 127);
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & RND_MASK as u128) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & RND_MASK as u128) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = (a_mant + addend as u128) >> 14;

    if rm == RM_RNE && rnd_bits == (1u32 << 13) {
        a_mant &= !(1u128);
    }

    a_exp += (a_mant >> 113) as i32;
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1i32 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    ((a_sign as u128) << 127)
        | (((a_exp as u32) as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant_in) - (128 - 1 - (128 - 2));
    let mut a_exp: i32 = a_exp_in;
    let mut a_mant: u128 = a_mant_in;
    a_exp -= shift;
    if shift >= 0 {
        a_mant = a_mant.wrapping_shl((shift as u32) & 127);
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// internal_cvt_i32_sf128(a, rm, pfflags, TRUE)
fn internal_cvt_i32_sf128(a: u32, rm: c_int, pfflags: *mut u32) -> u128 {
    // is_unsigned == TRUE, so the sign branch of the C code is never taken:
    // a_sign = 0, r = a.
    let a_sign: u32 = 0;
    let mut r: u32 = a;

    let mut a_exp: i32 = ((1i32 << 15) - 1) / 2 + 128 - 2;

    let l: i32 = 32 - clz32(r) - (128 - 1);
    if l > 0 && l < 32 {
        let mask: u32 = r & ((1u32 << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u32;
        a_exp += l;
    }
    let a_mant: u128 = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf128(a: u32, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_i32_sf128(a, rm, pfflags)
}
}

// ── `cvt_u64_sf128` · softfp_template_icvt.h:162-167 · chunk 3468 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_u64_sf128(uint64_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i64_sf128(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3468 {
// Port of cvt_u64_sf128 (softfp_template_icvt.h) plus the static helper chain it
// calls: internal_cvt_i64_sf128 -> normalize_sf128 -> roundpack_sf128
//   -> pack_sf128 / rshift_rnd128 / clz128.
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ffi::{c_int};

// RoundingModeEnum values (plain C enum, ABI = int)
const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

// ---- constants from the preprocessed C ----
// F_SIZE = 128, EXP_MASK = (1 << 15) - 1, F_MANT_SIZE = 112
const F_MANT_SIZE: u32 = 112;
const EXP_MASK: c_int = (1 << 15) - 1;

#[inline]
fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn clz128(a: u128) -> c_int {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as c_int
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & (((1u128) << 112) - 1))
}

fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    let mut a = a;
    if d > 0 {
        if d >= 128 {
            a = (a != 0) as u128;
        } else {
            let du = d as u32;
            let mask: u128 = ((1u128) << du) - 1;
            a = (a >> du) | ((a & mask) != 0) as u128;
        }
    }
    a
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
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = (1u32) << (((128 - 2) - F_MANT_SIZE) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        // default: RM_RDN, RM_RUP
        _ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                addend = ((1u32) << ((128 - 2) - F_MANT_SIZE)) - 1;
            } else {
                addend = 0;
            }
        }
    }

    let rnd_mask: u128 = (((1u128) << ((128 - 2) - F_MANT_SIZE)) - 1);

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || (a_mant.wrapping_add(addend as u128)) < ((1u128) << (128 - 1));
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & rnd_mask) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= (1 << 1) as u32;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & rnd_mask) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= (1 << 0) as u32;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> ((128 - 2) - F_MANT_SIZE);

    if rm == RM_RNE && rnd_bits == ((1u32) << (((128 - 2) - F_MANT_SIZE) - 1)) {
        a_mant &= !(1u128);
    }

    a_exp = a_exp.wrapping_add(((a_mant >> (112 + 1)) as u32) as c_int);
    if a_mant <= (((1u128) << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= EXP_MASK {
        if addend == 0 {
            a_exp = EXP_MASK - 1;
            a_mant = ((1u128) << 112) - 1;
        } else {
            a_exp = EXP_MASK;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= ((1 << 2) | (1 << 0)) as u32;
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
    let shift: c_int = clz128(a_mant_in).wrapping_sub((128 - 1 - (128 - 2)) as c_int);
    // C asserts shift >= 0 here; it is always satisfied on this path.
    let a_exp: c_int = a_exp_in.wrapping_sub(shift);
    let a_mant: u128 = if shift >= 0 && shift < 128 {
        a_mant_in << (shift as u32)
    } else {
        0
    };
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i64_sf128(a: i64, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u128 {
    let a_sign: u32;
    let r: u64;
    let mut a_exp: c_int;
    let mut a_mant: u128;

    if !is_unsigned && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    a_exp = (EXP_MASK / 2) + 128 - 2;

    let l: c_int = 64 - clz64(r) - (128 - 1);
    if l > 0 {
        // C: mask = r & (((uint64_t)1 << l) - 1);
        let mask: u64 = if l < 64 { r & (((1u64) << (l as u32)) - 1) } else { r };
        let shifted: u64 = if l < 64 { r >> (l as u32) } else { 0 };
        let sticky: bool = (r & mask) != 0;
        let nr: u64 = shifted | (sticky as u64);
        a_mant = nr as u128;
        a_exp += l;
        return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);
    }
    a_mant = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u64_sf128(a: u64, rm: c_int, pfflags: *mut u32) -> u128 {
    internal_cvt_i64_sf128(a as i64, rm, pfflags, true)
}
}

// ── `cvt_u128_sf128` · softfp_template_icvt.h:162-167 · chunk 3490 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint128_t cvt_u128_sf128(uint128_t a,
//                                                            RoundingModeEnum rm,
//                                                            uint32_t *pfflags)
//     {
//         return internal_cvt_i128_sf128(a, rm, pfflags, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3490 {
//! Port of `cvt_u128_sf128` from softfp_template_icvt.h (128-bit integer to
//! binary128 conversion, unsigned path).  The static helpers it depends on
//! (`internal_cvt_i128_sf128`, `normalize_sf128`, `roundpack_sf128`,
//! `rshift_rnd128`, `clz128`, `pack_sf128`) have no linker symbols, so their
//! logic is reproduced here as private Rust functions.

use core::ffi::{c_int, c_uint};

const MANT_MASK: u128 = (1u128 << 112) - 1;
const RND_SIZE: i32 = (128 - 2) - 112; // 14

#[inline]
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

#[inline]
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1)) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

#[inline]
fn rshift_rnd128(a: u128, d: i32) -> u128 {
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

fn roundpack_sf128(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    // RND_SIZE == 14 for binary128.
    let addend: u32 = match rm {
        0 /* RM_RNE */ | 4 /* RM_RMM */ => 1u32 << (RND_SIZE - 1),
        1 /* RM_RTZ */ => 0,
        _ /* RM_RDN, RM_RUP, default */ => {
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << RND_SIZE) - 1
            } else {
                0
            }
        }
    };

    let mut a_exp = a_exp;
    let mut a_mant = a_mant;
    let rnd_mask: u128 = (1u128 << RND_SIZE) - 1;
    let rnd_bits: u32;

    // potentially subnormal
    if a_exp <= 0 {
        let is_subnormal =
            a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << (128 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & rnd_mask) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1; // FFLAG_UNDERFLOW
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & rnd_mask) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0; // FFLAG_INEXACT
        }
    }
    a_mant = a_mant.wrapping_add(addend as u128) >> (RND_SIZE as u32);
    // half way: select even result
    if rm == 0 /* RM_RNE */ && rnd_bits == (1u32 << (RND_SIZE - 1)) {
        a_mant &= !1u128;
    }
    a_exp += (a_mant >> (112 + 1)) as i32;
    if a_mant <= MANT_MASK {
        // denormalized or zero
        a_exp = 0;
    } else if a_exp >= ((1 << 15) - 1) {
        // overflow
        if addend == 0 {
            a_exp = ((1 << 15) - 1) - 1;
            a_mant = MANT_MASK;
        } else {
            // infinity
            a_exp = (1 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0); // FFLAG_OVERFLOW | FFLAG_INEXACT
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf128(
    a_sign: u32,
    a_exp: i32,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    // shift = clz128(a_mant) - (128 - 1 - (128 - 2)); the C code asserts
    // shift >= 0; keep the shift panic-free in case that invariant is broken.
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    let sh: u32 = if shift < 0 { 0 } else { shift as u32 };
    let a_exp = a_exp - shift;
    let a_mant = a_mant.wrapping_shl(sh);
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

/// `internal_cvt_i128_sf128(a, rm, pfflags, TRUE)`
fn internal_cvt_i128_sf128_unsigned(a: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    // is_unsigned == TRUE, so the sign/magnitude path is never taken.
    let a_sign: u32 = 0;
    let mut r: u128 = a;

    let mut a_exp: i32 = (((1 << 15) - 1) / 2) + 128 - 2;
    // need to reduce range before generic float normalization
    let l: i32 = 128 - clz128(r) - (128 - 1);
    if l > 0 {
        let mask = r & ((1u128 << (l as u32)) - 1);
        r = (r >> (l as u32)) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    let a_mant = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u128_sf128(a: u128, rm: c_int, pfflags: *mut c_uint) -> u128 {
    internal_cvt_i128_sf128_unsigned(a, rm, pfflags)
}
}

// ── `return internal_cvt_i32_sf32(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3315 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf32(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3315 {
// Port of `return internal_cvt_i32_sf32(a, rm, pfflags, TRUE);`
// from `cvt_u32_sf32`.  The callee and all of its helpers are `static` C
// functions (no linker symbols), so their logic is reproduced here with the
// same 32-bit wrapping semantics.

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
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u32 = (1u32 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

// static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

// static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let rm_u: u32 = rm as u32;

    // switch(rm): RNE=0, RTZ=1, RDN=2, RUP=3, RMM=4
    let addend: u32 = match rm {
        0 | 4 => 1u32 << 3, // (1 << (((32 - 2) - 23) - 1))
        1 => 0,
        _ => {
            // default / RM_RDN / RM_RUP
            if (a_sign ^ (rm_u & 1)) != 0 {
                (1u32 << 7) - 1 // (1 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || a_mant.wrapping_add(addend) < (1u32 << 31);
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

    if rm == 0 && rnd_bits == (1u32 << 3) {
        a_mant &= !1u32;
    }

    a_exp += (a_mant >> (23 + 1)) as i32;
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
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }

    pack_sf32(a_sign, a_exp as u32, a_mant)
}

// static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
//                                RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf32(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // shift = clz32(a_mant) - (32 - 1 - (32 - 2)); asserted >= 0
    let shift: i32 = clz32(a_mant) - 1;
    a_exp -= shift;
    a_mant = a_mant.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf32__part3315(a: u32, rm: c_int, pfflags: *mut u32) -> u32 {
    // internal_cvt_i32_sf32(a, rm, pfflags, TRUE)
    let a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u32;
    let mut r: u32;
    let mut _mask: u32;

    // is_unsigned == TRUE, so the signed branch is never taken.
    a_sign = 0;
    r = a;

    a_exp = (((1i32 << 8) - 1) / 2) + 32 - 2; // 157

    let l: i32 = 32 - clz32(r) - (32 - 1);
    if l > 0 {
        _mask = r & ((1u32 << l) - 1);
        r = (r >> l) | (if (r & _mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    a_mant = r;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return internal_cvt_i64_sf32(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3337 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf32(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3337 {
// Port of the TARGET: return internal_cvt_i64_sf32(a, rm, pfflags, TRUE);
// for cvt_u64_sf32 (unsigned 64-bit -> binary32 softfloat conversion).
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd32(mut a: u32, d: i32) -> u32 {
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

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << 31) | (a_exp << 23) | (a_mant & 0x007f_ffffu32)
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
        RM_RNE | RM_RMM => {
            addend = 1u32 << 7;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
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
            *pfflags |= 1u32 << 0;
        }
    }
    a_mant = a_mant.wrapping_add(addend) >> 7;

    if rm == RM_RNE && rnd_bits == (1u32 << 6) {
        a_mant &= !1u32;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 24) as i32);
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
            *pfflags |= (1u32 << 2) | (1u32 << 0);
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
    let shift: i32 = clz32(a_mant) - (32 - 1 - (32 - 2));
    // The original code asserts shift >= 0; it holds on every reachable path
    // (a_mant never has bit 31 set here), so the assertion is elided.
    a_exp -= shift;
    if shift > 0 {
        a_mant <<= shift as u32;
    }
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u64_sf32__part3337(a: u64, rm: c_int, pfflags: *mut u32) -> u32 {
    let a_sign: u32;
    let mut r: u64;
    let mut a_exp: i32;

    // is_unsigned == TRUE
    a_sign = 0;
    r = a;
    a_exp = ((1i32 << 8) - 1) / 2 + 32 - 2;

    let l: i32 = 64 - clz64(r) - (32 - 1);
    if l > 0 {
        let lu: u32 = l as u32;
        let mask: u64 = r & ((1u64 << lu) - 1);
        r = (r >> lu) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return internal_cvt_i128_sf32(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3359 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf32(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3359 {
// Port of the TARGET statement inside cvt_u128_sf32:
//   return internal_cvt_i128_sf32(a, rm, pfflags, TRUE);
// internal_cvt_i128_sf32 (static C) and its static helpers are re-implemented
// here as private Rust functions, specialized for is_unsigned == TRUE.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd32(a: u32, d: i32) -> u32 {
    let mut a = a;
    if d != 0 {
        if d >= 32 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u32 << d) - 1;
            a = (a >> d) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn pack_sf32(a_sign: u32, a_exp: u32, a_mant: u32) -> u32 {
    (a_sign << (32 - 1)) | (a_exp << 23) | (a_mant & ((1u32 << 23) - 1))
}

fn roundpack_sf32(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let addend: u32 = match rm {
        RM_RNE | RM_RMM => 1u32 << (((32 - 2) - 23) - 1),
        RM_RTZ => 0,
        _ => {
            // default, RM_RDN, RM_RUP
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << ((32 - 2) - 23)) - 1
            } else {
                0
            }
        }
    };

    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
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
    a_exp_in: i32,
    a_mant_in: u32,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    let shift: i32 = clz32(a_mant_in) - (32 - 1 - (32 - 2));
    // C asserts shift >= 0 here; it always holds for this call path.
    if shift < 0 {
        // Keep the C behavior of never returning normally after a failed
        // assertion: avoid unwinding across FFI by trapping instead.
        unsafe {
            core::hint::unreachable_unchecked();
        }
    }
    let a_exp: i32 = a_exp_in - shift;
    let a_mant: u32 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i128_sf32_unsigned(
    a: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u32 {
    // is_unsigned == TRUE => a_sign = 0, r = a
    let a_sign: u32 = 0;
    let mut r: u128 = a;
    let mut a_exp: i32 = (((1 << 8) - 1) / 2) + 32 - 2;

    let l: i32 = 128 - clz128(r) - (32 - 1);
    if l > 0 {
        let mask: u128 = (1u128 << l) - 1;
        r = (r >> l) | (if (r & mask) != 0 { 1u128 } else { 0u128 });
        a_exp += l;
    }
    let a_mant: u32 = r as u32;
    normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u128_sf32__part3359(a: u128, rm: c_int, pfflags: *mut u32) -> u32 {
    internal_cvt_i128_sf32_unsigned(a, rm, pfflags)
}
}

// ── `return internal_cvt_i32_sf64(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3381 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf64(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3381 {
// Port of cvt_u32_sf64's tail: internal_cvt_i32_sf64(a, rm, pfflags, TRUE)
// plus the static helpers it needs (clz32/clz64, normalize_sf64, roundpack_sf64,
// pack_sf64, rshift_rnd64) inlined here as private Rust fns.

use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RMM: c_int = 4;

fn clz32(a: u32) -> c_int {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as c_int
    }
}

fn clz64(a: u64) -> c_int {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as c_int
    }
}

fn pack_sf64(a_sign: u32, a_exp: u32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1)) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
}

fn rshift_rnd64(mut a: u64, d: c_int) -> u64 {
    if d != 0 {
        if d >= 64 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask = (1u64 << (d as u32)) - 1;
            a = (a >> (d as u32)) | (if (a & mask) != 0 { 1 } else { 0 });
        }
    }
    a
}

fn roundpack_sf64(
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
            addend = 1u32 << (((64 - 2) - 52) - 1);
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
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
        let diff: c_int = 1 - a_exp;
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
    pack_sf64(a_sign, a_exp as u32, a_mant)
}

fn normalize_sf64(a_sign: u32, a_exp: c_int, a_mant: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let shift: c_int = clz64(a_mant) - (64 - 1 - (64 - 2));
    // The C code asserts shift >= 0; for every input reaching this port
    // (a_mant fits in 32 bits) shift is >= 31, so nothing to check.
    let mut exp: c_int = a_exp;
    let mut mant: u64 = a_mant;
    exp -= shift;
    if shift >= 0 {
        mant = mant.wrapping_shl(shift as u32);
    }
    roundpack_sf64(a_sign, exp, mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf64__part3381(a: u32, rm: c_int, pfflags: *mut u32) -> u64 {
    // internal_cvt_i32_sf64(a, rm, pfflags, TRUE)
    // is_unsigned is TRUE, so the sign-magnitude negation branch is never taken.
    let a_sign: u32 = 0;
    let mut a_exp: c_int;
    let mut r: u32 = a;

    a_exp = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: c_int = 32 - clz32(r) - (64 - 1);
    if l > 0 {
        let mask: u32 = r & ((1u32.wrapping_shl(l as u32)) - 1);
        r = (r >> (l as u32)) | (if (r & mask) != 0 { 1 } else { 0 });
        a_exp += l;
    }
    let a_mant: u64 = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return internal_cvt_i64_sf64(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3403 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf64(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3403 {
// Ported helper logic (all of it was `static`/`static inline` in C, so it lives here).
use core::ffi::c_int;

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
// RM_RDN = 2, RM_RUP = 3 handled via default branch
const RM_RMM: c_int = 4;

#[inline]
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
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
    ((a_sign as u64) << 63) | ((a_exp as u64) << 52) | (a_mant & ((1u64 << 52) - 1))
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
        RM_RNE | RM_RMM => {
            addend = 1u32 << 9;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN / RM_RUP
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 10) - 1;
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
            *pfflags |= 1u32;
        }
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> 10;

    if rm == RM_RNE && rnd_bits == (1u32 << 9) {
        a_mant &= !1u64;
    }

    a_exp = a_exp.wrapping_add((a_mant >> 53) as i32);
    if a_mant <= ((1u64 << 52) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 11) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 11) - 1) - 1;
            a_mant = (1u64 << 52) - 1;
        } else {
            a_exp = (1i32 << 11) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1u32 << 2) | 1u32;
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
    let shift: i32 = clz64(a_mant_in) - (63 - 62);
    // C code asserts shift >= 0 (always true for reachable inputs)
    let a_exp: i32 = a_exp_in.wrapping_sub(shift);
    let a_mant: u64 = a_mant_in.wrapping_shl(shift as u32);
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

// Port of `static uint64_t internal_cvt_i64_sf64(..., BOOL is_unsigned)`
// specialised to is_unsigned == TRUE (as used by cvt_u64_sf64).
#[no_mangle]
pub extern "C" fn cvt_u64_sf64__part3403(a: u64, rm: c_int, pfflags: *mut u32) -> u64 {
    let a_sign: u32;
    let mut r: u64;

    // is_unsigned is TRUE, so the negative branch is never taken.
    a_sign = 0;
    r = a;

    let mut a_exp: i32 = (((1i32 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 64 - clz64(r) - (64 - 1);
    if l > 0 {
        let mask: u64 = r & ((1u64 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u64;
        a_exp += l;
    }
    let a_mant: u64 = r;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}
}

// ── `return internal_cvt_i128_sf64(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3425 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf64(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3425 {
// Port of the unsigned 128-bit -> sf64 conversion (internal_cvt_i128_sf64 with
// is_unsigned = TRUE) together with the static helpers it relies on, which have
// no linker symbols in the C translation unit.

use core::ffi::c_int;

#[inline]
fn or_flags(pfflags: *mut u32, bits: u32) {
    unsafe {
        if !pfflags.is_null() {
            *pfflags |= bits;
        }
    }
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
fn rshift_rnd64(mut a: u64, d: i32) -> u64 {
    if d >= 64 {
        a = (a != 0) as u64;
    } else if d > 0 {
        let mask = (1u64 << d) - 1;
        a = (a >> d) | ((a & mask) != 0) as u64;
    }
    a
}

#[inline]
fn pack_sf64(a_sign: u32, a_exp: i32, a_mant: u64) -> u64 {
    ((a_sign as u64) << (64 - 1))
        | (((a_exp as i64) as u64) << 52)
        | (a_mant & ((1u64 << 52) - 1))
}

fn roundpack_sf64(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let addend: u32;
    match rm {
        0 | 4 => {
            // RM_RNE, RM_RMM
            addend = (1u32 << (((64 - 2) - 52) - 1)) as u32;
        }
        1 => {
            // RM_RTZ
            addend = 0;
        }
        _ => {
            // default, RM_RDN, RM_RUP
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
        let diff: i32 = 1 - a_exp;
        a_mant = rshift_rnd64(a_mant, diff);
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            or_flags(pfflags, 1 << 1);
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u64 << ((64 - 2) - 52)) - 1)) as u32;
    }
    if rnd_bits != 0 {
        or_flags(pfflags, 1 << 0);
    }
    a_mant = a_mant.wrapping_add(addend as u64) >> ((64 - 2) - 52);

    if rm == 0 && rnd_bits == (1u32 << (((64 - 2) - 52) - 1)) {
        a_mant &= !1u64;
    }

    a_exp += (a_mant >> (52 + 1)) as i32;
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
        or_flags(pfflags, (1 << 2) | (1 << 0));
    }
    pack_sf64(a_sign, a_exp, a_mant)
}

fn normalize_sf64(
    a_sign: u32,
    mut a_exp: i32,
    a_mant: u64,
    rm: c_int,
    pfflags: *mut u32,
) -> u64 {
    let shift: i32 = clz64(a_mant) - (64 - 1 - (64 - 2));
    a_exp -= shift;
    let a_mant: u64 = if shift >= 0 {
        a_mant.wrapping_shl(shift as u32)
    } else {
        a_mant
    };
    roundpack_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i128_sf64(a: u128, rm: c_int, pfflags: *mut u32, is_unsigned: bool) -> u64 {
    let a_signed: i128 = a as i128;
    let a_sign: u32;
    let mut a_exp: i32;
    let mut a_mant: u64;
    let mut r: u128;

    if !is_unsigned && a_signed < 0 {
        a_sign = 1;
        r = (a_signed as u128).wrapping_neg();
    } else {
        a_sign = 0;
        r = a;
    }
    a_exp = (((1 << 11) - 1) / 2) + 64 - 2;

    let l: i32 = 128 - clz128(r) - (64 - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128 << l) - 1);
        r = (r >> l) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    a_mant = r as u64;
    normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u128_sf64__part3425(a: u128, rm: c_int, pfflags: *mut u32) -> u64 {
    internal_cvt_i128_sf64(a, rm, pfflags, true)
}
}

// ── `return internal_cvt_i32_sf128(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3447 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i32_sf128(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3447 {
// Port of the static helper chain used by cvt_u32_sf128:
// internal_cvt_i32_sf128 (called with is_unsigned = TRUE) -> normalize_sf128 ->
// roundpack_sf128 -> rshift_rnd128 / pack_sf128, plus clz32 / clz128.
// All of these have internal linkage in C, so they are re-implemented here.

use core::ffi::c_int;

extern "C" {
    // used to mirror the C assert(shift >= 0) failure path (abort, never unwind)
    fn abort() -> !;
}

const RM_RNE: c_int = 0;
const RM_RTZ: c_int = 1;
const RM_RDN: c_int = 2;
const RM_RUP: c_int = 3;
const RM_RMM: c_int = 4;

const MANT_MASK: u128 = ((1u128 << 112) - 1);

fn clz32(a: u32) -> i32 {
    if a == 0 {
        32
    } else {
        a.leading_zeros() as i32
    }
}

fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        a.leading_zeros() as i32
    }
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & MANT_MASK)
}

fn rshift_rnd128(mut a: u128, d: i32) -> u128 {
    if d > 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else {
            let mask: u128 = (1u128 << (d as u32)) - 1;
            let sticky: u128 = if (a & mask) != 0 { 1 } else { 0 };
            a = (a >> (d as u32)) | sticky;
        }
    }
    a
}

fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: i32,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32;
    let mut rnd_bits: u32;

    match rm {
        RM_RNE | RM_RMM => {
            addend = 1u32 << 13;
        }
        RM_RTZ => {
            addend = 0;
        }
        _ => {
            // default: RM_RDN, RM_RUP and any other value
            if (a_sign ^ ((rm & 1) as u32)) != 0 {
                addend = (1u32 << 14) - 1;
            } else {
                addend = 0;
            }
        }
    }

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || a_mant.wrapping_add(addend as u128) < (1u128 << 127);
        let diff: i32 = 1 - a_exp;
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

    if rm == RM_RNE && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> 113) as i32;
    if a_mant <= MANT_MASK {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 15) - 1) - 1;
            a_mant = MANT_MASK;
        } else {
            a_exp = (1i32 << 15) - 1;
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
    mut a_exp: i32,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift: i32 = clz128(a_mant) - (128 - 1 - (128 - 2));
    if shift < 0 {
        // C: assert(shift >= 0) -> __assert_fail, which aborts
        unsafe {
            abort();
        }
    }
    a_exp -= shift;
    a_mant <<= shift as u32;
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

fn internal_cvt_i32_sf128(a: i32, rm: c_int, pfflags: *mut u32, is_unsigned: i32) -> u128 {
    let a_sign: u32;
    let mut a_exp: i32;
    let a_mant: u128;
    let mut r: u32;
    let mut mask: u32;
    let l: i32;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u32).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u32;
    }
    a_exp = (((1i32 << 15) - 1) / 2) + 128 - 2;

    l = 32 - clz32(r) - (128 - 1);
    if l > 0 {
        let lu: u32 = l as u32;
        let m: u32 = if lu >= 32 { u32::MAX } else { (1u32 << lu) - 1 };
        mask = r & m;
        let shifted: u32 = if lu >= 32 { 0 } else { r >> lu };
        let sticky: u32 = if (r & mask) != 0 { 1 } else { 0 };
        r = shifted | sticky;
        a_exp += l;
    }
    let _ = mask;
    a_mant = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u32_sf128__part3447(a: u32, rm: c_int, pfflags: *mut u32) -> u128 {
    // cvt_u32_sf128 passes its uint32_t argument to the int32_t parameter with
    // is_unsigned = TRUE (TRUE == 1).
    internal_cvt_i32_sf128(a as i32, rm, pfflags, 1)
}
}

// ── `return internal_cvt_i64_sf128(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3469 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i64_sf128(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3469 {
// Port of the C target in cvt_u64_sf128:
//     return internal_cvt_i64_sf128(a, rm, pfflags, TRUE);
// internal_cvt_i64_sf128 and all helpers it uses are `static` in the C
// translation unit, so their logic is reproduced here as private Rust fns.

#![allow(dead_code)]

const RM_RNE: u32 = 0;
const RM_RTZ: u32 = 1;
const RM_RDN: u32 = 2;
const RM_RUP: u32 = 3;
const RM_RMM: u32 = 4;

const TRUE: i32 = 1;

// static inline int clz64(uint64_t a)
fn clz64(a: u64) -> i32 {
    if a == 0 {
        64
    } else {
        a.leading_zeros() as i32
    }
}

// static inline int clz128(uint128_t a)
fn clz128(a: u128) -> i32 {
    if a == 0 {
        128
    } else {
        let ah = (a >> 64) as u64;
        let al = a as u64;
        if ah != 0 {
            clz64(ah)
        } else {
            clz64(al) + 64
        }
    }
}

// static uint128_t rshift_rnd128(uint128_t a, int d)
fn rshift_rnd128(a: u128, d: i32) -> u128 {
    let mut a = a;
    if d != 0 {
        if d >= 128 {
            a = if a != 0 { 1 } else { 0 };
        } else if d > 0 {
            let sh = d as u32;
            let mask: u128 = (1u128 << sh) - 1;
            a = (a >> sh) | if (a & mask) != 0 { 1 } else { 0 };
        }
    }
    a
}

// static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << (128 - 1))
        | ((a_exp as u128) << 112)
        | (a_mant & ((1u128 << 112) - 1))
}

// static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                  RoundingModeEnum rm, uint32_t *pfflags)
fn roundpack_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;

    let addend: u32 = match rm {
        RM_RNE | RM_RMM => (1u32 << (((128 - 2) - 112) - 1)),
        RM_RTZ => 0,
        _ => {
            if (a_sign ^ (rm & 1)) != 0 {
                (1u32 << ((128 - 2) - 112)) - 1
            } else {
                0
            }
        }
    };

    let rnd_bits: u32;
    if a_exp <= 0 {
        let is_subnormal: bool =
            a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << (128 - 1));
        let diff = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & ((1u128 << ((128 - 2) - 112)) - 1)) as u32;
    }

    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = (a_mant + (addend as u128)) >> ((128 - 2) - 112);

    if rm == RM_RNE && rnd_bits == (1u32 << (((128 - 2) - 112) - 1)) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> (112 + 1)) as i32;
    if a_mant <= ((1u128 << 112) - 1) {
        a_exp = 0;
    } else if a_exp >= ((1i32 << 15) - 1) {
        if addend == 0 {
            a_exp = ((1i32 << 15) - 1) - 1;
            a_mant = (1u128 << 112) - 1;
        } else {
            a_exp = (1i32 << 15) - 1;
            a_mant = 0;
        }
        unsafe {
            *pfflags |= (1 << 2) | (1 << 0);
        }
    }
    pack_sf128(a_sign, a_exp as u32, a_mant)
}

// static uint128_t normalize_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
//                                  RoundingModeEnum rm, uint32_t *pfflags)
fn normalize_sf128(
    a_sign: u32,
    a_exp_in: i32,
    a_mant_in: u128,
    rm: u32,
    pfflags: *mut u32,
) -> u128 {
    let mut a_exp = a_exp_in;
    let mut a_mant = a_mant_in;
    let shift = clz128(a_mant) - (128 - 1 - (128 - 2));
    // The C code asserts shift >= 0 here; it always holds for this path.
    a_exp -= shift;
    if shift > 0 {
        a_mant <<= shift as u32;
    }
    roundpack_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

// static uint128_t internal_cvt_i64_sf128(int64_t a, RoundingModeEnum rm,
//                                         uint32_t *pfflags, BOOL is_unsigned)
fn internal_cvt_i64_sf128(a: i64, rm: u32, pfflags: *mut u32, is_unsigned: i32) -> u128 {
    let a_sign: u32;
    let mut a_exp: i32;
    let mut r: u64;

    if is_unsigned == 0 && a < 0 {
        a_sign = 1;
        r = (a as u64).wrapping_neg();
    } else {
        a_sign = 0;
        r = a as u64;
    }
    a_exp = (((1i32 << 15) - 1) / 2) + 128 - 2;

    let l: i32 = 64 - clz64(r) - (128 - 1);
    if l > 0 {
        let sh = l as u32;
        let mask: u64 = r & ((1u64 << sh) - 1);
        r = (r >> sh) | if (r & mask) != 0 { 1 } else { 0 };
        a_exp += l;
    }
    let a_mant: u128 = r as u128;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u64_sf128__part3469(a: u64, rm: u32, pfflags: *mut u32) -> u128 {
    internal_cvt_i64_sf128(a as i64, rm, pfflags, TRUE)
}
}

// ── `return internal_cvt_i128_sf128(a, rm, pfflags, TRUE);` · softfp_template_icvt.h:166-166 · chunk 3491 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return internal_cvt_i128_sf128(a, rm, pfflags, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3491 {
// Port of cvt_u128_sf128's TARGET:
//   return internal_cvt_i128_sf128(a, rm, pfflags, TRUE);
// The C callee and all its helpers are file-static, so their logic is ported here.
use core::ffi::c_int;

// RM_RNE = 0, RM_RTZ = 1, RM_RDN = 2, RM_RUP = 3, RM_RMM = 4

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

fn rshift_rnd128(a: u128, d: c_int) -> u128 {
    if d <= 0 {
        a
    } else if d >= 128 {
        (a != 0) as u128
    } else {
        let mask: u128 = (1u128.wrapping_shl(d as u32)) - 1;
        a.wrapping_shr(d as u32) | ((a & mask) != 0) as u128
    }
}

fn pack_sf128(a_sign: u32, a_exp: u32, a_mant: u128) -> u128 {
    ((a_sign as u128) << 127) | ((a_exp as u128) << 112) | (a_mant & ((1u128 << 112) - 1))
}

fn roundpack_sf128(
    a_sign: u32,
    mut a_exp: c_int,
    mut a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let addend: u32 = match rm {
        0 | 4 => 1u32 << 13, // RM_RNE, RM_RMM
        1 => 0,              // RM_RTZ
        _ => {
            // RM_RDN, RM_RUP and anything else
            if (a_sign ^ ((rm as u32) & 1)) != 0 {
                (1u32 << 14) - 1
            } else {
                0
            }
        }
    };

    let mut rnd_bits: u32;

    if a_exp <= 0 {
        let is_subnormal: bool = a_exp < 0 || (a_mant + (addend as u128)) < (1u128 << 127);
        let diff: c_int = 1 - a_exp;
        a_mant = rshift_rnd128(a_mant, diff);
        rnd_bits = (a_mant & 16383u128) as u32;
        if is_subnormal && rnd_bits != 0 {
            unsafe {
                *pfflags |= 1 << 1;
            }
        }
        a_exp = 1;
    } else {
        rnd_bits = (a_mant & 16383u128) as u32;
    }
    if rnd_bits != 0 {
        unsafe {
            *pfflags |= 1 << 0;
        }
    }
    a_mant = (a_mant + (addend as u128)) >> 14;

    if rm == 0 && rnd_bits == (1u32 << 13) {
        a_mant &= !1u128;
    }

    a_exp += (a_mant >> 113) as c_int;
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
    mut a_exp: c_int,
    a_mant: u128,
    rm: c_int,
    pfflags: *mut u32,
) -> u128 {
    let shift: c_int = clz128(a_mant) - (128 - 1 - (128 - 2));
    a_exp -= shift;
    let m: u128 = a_mant.wrapping_shl((shift as u32) & 127);
    roundpack_sf128(a_sign, a_exp, m, rm, pfflags)
}

#[no_mangle]
pub extern "C" fn cvt_u128_sf128__part3491(a: u128, rm: c_int, pfflags: *mut u32) -> u128 {
    // internal_cvt_i128_sf128(a, rm, pfflags, TRUE): is_unsigned is TRUE, so the
    // "!is_unsigned && a < 0" branch is never taken.
    let a_sign: u32 = 0;
    let mut r: u128 = a;

    let mut a_exp: c_int = (((1 << 15) - 1) / 2) + 128 - 2;

    let l: c_int = 128 - clz128(r) - (128 - 1);
    if l > 0 {
        let mask: u128 = r & ((1u128.wrapping_shl(l as u32)) - 1);
        r = r.wrapping_shr(l as u32) | ((r & mask) != 0) as u128;
        a_exp += l;
    }
    let a_mant: u128 = r;
    normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags)
}
}

