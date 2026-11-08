//! From `aes.c`.

// ── `if (!userKey || !key)` · aes.c:745-746 · chunk 67 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (!userKey || !key)
//       return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk67 {
use core::ffi::c_int;
use std::os::raw::c_uchar;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part67(
    userKey: *const c_uchar,
    key: *mut c_void,
) -> c_int {
    if userKey.is_null() || key.is_null() {
        return -1;
    }
    0
}
}

// ── `if (bits != 128 && bits != 192 && bits != 256)` · aes.c:747-748 · chunk 68 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bits != 128 && bits != 192 && bits != 256)
//       return -2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk68 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part68(bits: c_int) -> c_int {
    if bits != 128 && bits != 192 && bits != 256 {
        return -2;
    }
    0
}
}

// ── `rk = key->rd_key;` · aes.c:750-750 · chunk 69 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     rk = key->rd_key;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk69 {
#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part69(mut rk: *mut u32, key_rd_key: *mut u32) -> *mut u32 {
    rk = key_rd_key;
    rk
}
}

// ── `if (bits==128) … else` · aes.c:752-757 · chunk 70 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (bits==128)
//       key->rounds = 10;
//      else if (bits==192)
//       key->rounds = 12;
//      else
//       key->rounds = 14;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk70 {
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_int;

#[repr(C)]
struct aes_key_st {
    rd_key: [u32; 60],
    rounds: c_int,
}

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part70(bits: c_int, key: *mut aes_key_st) {
    unsafe {
        if bits == 128 {
            (*key).rounds = 10;
        } else if bits == 192 {
            (*key).rounds = 12;
        } else {
            (*key).rounds = 14;
        }
    }
}
}

// ── `rk[0] = (((u32)(userKey)[0] << 24) ^ ((u32)(userKey)[1] << 16) ^ ((u32)(userKey)[2] << 8) ^ ((u…` · aes.c:759-762 · chunk 71 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     rk[0] = (((u32)(userKey)[0] << 24) ^ ((u32)(userKey)[1] << 16) ^ ((u32)(userKey)[2] << 8) ^ ((u32)(userKey)[3]));
//      rk[1] = (((u32)(userKey + 4)[0] << 24) ^ ((u32)(userKey + 4)[1] << 16) ^ ((u32)(userKey + 4)[2] << 8) ^ ((u32)(userKey + 4)[3]));
//      rk[2] = (((u32)(userKey + 8)[0] << 24) ^ ((u32)(userKey + 8)[1] << 16) ^ ((u32)(userKey + 8)[2] << 8) ^ ((u32)(userKey + 8)[3]));
//      rk[3] = (((u32)(userKey + 12)[0] << 24) ^ ((u32)(userKey + 12)[1] << 16) ^ ((u32)(userKey + 12)[2] << 8) ^ ((u32)(userKey + 12)[3]));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk71 {
// Rust implementation of AES_set_encrypt_key__part71
// This function initializes the first 4 words of the AES round key from the user key

use core::ffi::c_uint;
use core::ffi::c_uchar;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part71(rk: *mut c_uint, userKey: *const c_uchar) -> c_int {
    unsafe {
        // Initialize rk[0] from userKey[0..3]
        *rk.wrapping_add(0) = ((*userKey as c_uint) << 24) 
              ^ ((*userKey.add(1) as c_uint) << 16) 
              ^ ((*userKey.add(2) as c_uint) << 8) 
              ^ (*userKey.add(3) as c_uint);
        // Initialize rk[1] from userKey[4..7]
        *rk.wrapping_add(1) = ((*userKey.add(4) as c_uint) << 24) 
              ^ ((*userKey.add(5) as c_uint) << 16) 
              ^ ((*userKey.add(6) as c_uint) << 8) 
              ^ (*userKey.add(7) as c_uint);
        // Initialize rk[2] from userKey[8..11]
        *rk.wrapping_add(2) = ((*userKey.add(8) as c_uint) << 24) 
              ^ ((*userKey.add(9) as c_uint) << 16) 
              ^ ((*userKey.add(10) as c_uint) << 8) 
              ^ (*userKey.add(11) as c_uint);
        // Initialize rk[3] from userKey[12..15]
        *rk.wrapping_add(3) = ((*userKey.add(12) as c_uint) << 24) 
              ^ ((*userKey.add(13) as c_uint) << 16) 
              ^ ((*userKey.add(14) as c_uint) << 8) 
              ^ (*userKey.add(15) as c_uint);
        return 0;
    }
}
}

// ── `rk[4] = (((u32)(userKey + 16)[0] << 24) ^ ((u32)(userKey + 16)[1] << 16) ^ ((u32)(userKey + 16)…` · aes.c:781-782 · chunk 73 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     rk[4] = (((u32)(userKey + 16)[0] << 24) ^ ((u32)(userKey + 16)[1] << 16) ^ ((u32)(userKey + 16)[2] << 8) ^ ((u32)(userKey + 16)[3]));
//      rk[5] = (((u32)(userKey + 20)[0] << 24) ^ ((u32)(userKey + 20)[1] << 16) ^ ((u32)(userKey + 20)[2] << 8) ^ ((u32)(userKey + 20)[3]));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk73 {
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part73(rk: *mut c_uint, user_key: *const u8) {
    unsafe {
        // rk[4] = big-endian u32 from user_key[16..20]
        let k0 = (*user_key.add(16) as c_uint) << 24 |
                 (*user_key.add(17) as c_uint) << 16 |
                 (*user_key.add(18) as c_uint) << 8 |
                 (*user_key.add(19) as c_uint);
        *rk.add(4) = k0;

        // rk[5] = big-endian u32 from user_key[20..24]
        let k1 = (*user_key.add(20) as c_uint) << 24 |
                 (*user_key.add(21) as c_uint) << 16 |
                 (*user_key.add(22) as c_uint) << 8 |
                 (*user_key.add(23) as c_uint);
        *rk.add(5) = k1;
    }
}
}

// ── `if (bits == 192)` · aes.c:783-802 · chunk 74 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bits == 192) {
//       while (1) {
//        temp = rk[ 5];
//        rk[ 6] = rk[ 0] ^
//         (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
//         (Te4[(temp >> 8) & 0xff] & 0x00ff0000) ^
//         (Te4[(temp ) & 0xff] & 0x0000ff00) ^
//         (Te4[(temp >> 24) ] & 0x000000ff) ^
//         rcon[i];
//        rk[ 7] = rk[ 1] ^ rk[ 6];
//        rk[ 8] = rk[ 2] ^ rk[ 7];
//        rk[ 9] = rk[ 3] ^ rk[ 8];
//        if (++i == 8) {
//         return 0;
//        }
//        rk[10] = rk[ 4] ^ rk[ 9];
//        rk[11] = rk[ 5] ^ rk[10];
//        rk += 6;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk74 {
use std::os::raw::c_int;
use std::os::raw::c_uint;

type u32 = c_uint;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part74(
    rk: *mut u32,
    i: *mut c_int,
    Te4: *const u32,
    rcon: *const u32
) -> c_int {
    unsafe {
        let temp = *rk.add(5);
        
        let rk0 = *rk;
        let rk1 = *rk.add(1);
        let rk2 = *rk.add(2);
        let rk3 = *rk.add(3);
        let rk4 = *rk.add(4);
        
        // rk[6] = rk[0] ^ Te4[(temp >> 16) & 0xff] & 0xff000000) ^ Te4[(temp >> 8) & 0xff] & 0x00ff0000) ^ Te4[temp & 0xff] & 0x0000ff00) ^ Te4[(temp >> 24)] & 0x000000ff) ^ rcon[i]
        *rk.add(6) = rk0 
            ^ ((*Te4.add(((temp >> 16) as usize & 0xff) as usize) as u32) & 0xff000000)
            ^ ((*Te4.add(((temp >> 8) as usize & 0xff) as usize) as u32) & 0x00ff0000)
            ^ ((*Te4.add((temp as usize & 0xff) as usize) as u32) & 0x0000ff00)
            ^ ((*Te4.add(((temp >> 24) as usize & 0xff) as usize) as u32) & 0x000000ff)
            ^ (*rcon.add(i as usize) as u32);
        
        // rk[7] = rk[1] ^ rk[6]
        *rk.add(7) = rk1 ^ *rk.add(6);
        // rk[8] = rk[2] ^ rk[7]
        *rk.add(8) = rk2 ^ *rk.add(7);
        // rk[9] = rk[3] ^ rk[8]
        *rk.add(9) = rk3 ^ *rk.add(8);
        
        // Increment loop counter i
        let i_new = i.read().wrapping_add(1);
        *i = i_new;
        
        // Check if expansion complete (i == 8 for 192-bit keys)
        if i_new == 8 {
            return 0;  // key expansion done
        }
        
        // rk[10] = rk[4] ^ rk[9]
        *rk.add(10) = rk4 ^ *rk.add(9);
        // rk[11] = rk[5] ^ rk[10]
        *rk.add(11) = temp ^ *rk.add(10);
        
        1  // continue (not done yet)
    }
}
}

// ── `rk[6] = (((u32)(userKey + 24)[0] << 24) ^ ((u32)(userKey + 24)[1] << 16) ^ ((u32)(userKey + 24)…` · aes.c:803-804 · chunk 75 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     rk[6] = (((u32)(userKey + 24)[0] << 24) ^ ((u32)(userKey + 24)[1] << 16) ^ ((u32)(userKey + 24)[2] << 8) ^ ((u32)(userKey + 24)[3]));
//      rk[7] = (((u32)(userKey + 28)[0] << 24) ^ ((u32)(userKey + 28)[1] << 16) ^ ((u32)(userKey + 28)[2] << 8) ^ ((u32)(userKey + 28)[3]));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk75 {
// Port of AES_set_encrypt_key__part75 from aes.c
// This function sets rk[6] and rk[7] from the user key for 256-bit AES keys

use core::ffi::c_uint;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part75(user_key: *const c_uchar, rk: *mut c_uint) {
    unsafe {
        // rk[6] = (((u32)(userKey + 24)[0] << 24) ^ ((u32)(userKey + 24)[1] << 16) ^ ((u32)(userKey + 24)[2] << 8) ^ ((u32)(userKey + 24)[3]));
        let b0 = *user_key.offset(24) as c_uint;
        let b1 = *user_key.offset(25) as c_uint;
        let b2 = *user_key.offset(26) as c_uint;
        let b3 = *user_key.offset(27) as c_uint;
        rk.add(6).write((b0 << 24) | (b1 << 16) | (b2 << 8) | b3);
        
        // rk[7] = (((u32)(userKey + 28)[0] << 24) ^ ((u32)(userKey + 28)[1] << 16) ^ ((u32)(userKey + 28)[2] << 8) ^ ((u32)(userKey + 28)[3]));
        let b0 = *user_key.offset(28) as c_uint;
        let b1 = *user_key.offset(29) as c_uint;
        let b2 = *user_key.offset(30) as c_uint;
        let b3 = *user_key.offset(31) as c_uint;
        rk.add(7).write((b0 << 24) | (b1 << 16) | (b2 << 8) | b3);
    }
}
}

// ── `if (bits == 256)` · aes.c:805-832 · chunk 76 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (bits == 256) {
//       while (1) {
//        temp = rk[ 7];
//        rk[ 8] = rk[ 0] ^
//         (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
//         (Te4[(temp >> 8) & 0xff] & 0x00ff0000) ^
//         (Te4[(temp ) & 0xff] & 0x0000ff00) ^
//         (Te4[(temp >> 24) ] & 0x000000ff) ^
//         rcon[i];
//        rk[ 9] = rk[ 1] ^ rk[ 8];
//        rk[10] = rk[ 2] ^ rk[ 9];
//        rk[11] = rk[ 3] ^ rk[10];
//        if (++i == 7) {
//         return 0;
//        }
//        temp = rk[11];
//        rk[12] = rk[ 4] ^
//         (Te4[(temp >> 24) ] & 0xff000000) ^
//         (Te4[(temp >> 16) & 0xff] & 0x00ff0000) ^
//         (Te4[(temp >> 8) & 0xff] & 0x0000ff00) ^
//         (Te4[(temp ) & 0xff] & 0x000000ff);
//        rk[13] = rk[ 5] ^ rk[12];
//        rk[14] = rk[ 6] ^ rk[13];
//        rk[15] = rk[ 7] ^ rk[14];
//     
//        rk += 8;
//              }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk76 {
use core::ffi::c_int;
use core::ffi::c_uint;

#[no_mangle] pub extern "C" fn AES_set_encrypt_key__part76(
    mut rk: *mut c_uint,
    i: *mut c_int,
    Te4: *const c_uint,
    rcon: *const c_uint
) -> c_int {
    loop {
        unsafe {
            let temp = *rk.offset(7);
            
            let rk0 = *rk.offset(0);
            let rk1 = *rk.offset(1);
            let rk2 = *rk.offset(2);
            let rk3 = *rk.offset(3);
            
            let t16 = ((temp >> 16) & 0xff) as isize;
            let t8 = ((temp >> 8) & 0xff) as isize;
            let t0 = (temp & 0xff) as isize;
            let t24 = ((temp >> 24) & 0xff) as isize;
            
            let te4_16 = *Te4.offset(t16);
            let te4_8 = *Te4.offset(t8);
            let te4_0 = *Te4.offset(t0);
            let te4_24 = *Te4.offset(t24);
            
            let rcon_i = *rcon.offset(*i as isize);
            
            let rk8 = rk0 
                ^ (te4_16 & 0xff000000) 
                ^ (te4_8 & 0x00ff0000) 
                ^ (te4_0 & 0x0000ff00) 
                ^ (te4_24 & 0x000000ff) 
                ^ rcon_i;
            
            *rk.offset(8) = rk8;
            *rk.offset(9) = rk1 ^ rk8;
            *rk.offset(10) = rk2 ^ *rk.offset(9);
            *rk.offset(11) = rk3 ^ *rk.offset(10);
            
            *i = *i + 1;
            if *i == 7 {
                return 0;
            }
            
            let temp_new = *rk.offset(11);
            
            let rk4 = *rk.offset(4);
            let rk5 = *rk.offset(5);
            let rk6 = *rk.offset(6);
            let rk7_val = *rk.offset(7);
            
            let t_new_24 = ((temp_new >> 24) & 0xff) as isize;
            let t_new_16 = ((temp_new >> 16) & 0xff) as isize;
            let t_new_8 = ((temp_new >> 8) & 0xff) as isize;
            let t_new_0 = (temp_new & 0xff) as isize;
            
            let te4_new_24 = *Te4.offset(t_new_24);
            let te4_new_16 = *Te4.offset(t_new_16);
            let te4_new_8 = *Te4.offset(t_new_8);
            let te4_new_0 = *Te4.offset(t_new_0);
            
            let rk12 = rk4
                ^ (te4_new_24 & 0xff000000) 
                ^ (te4_new_16 & 0x00ff0000) 
                ^ (te4_new_8 & 0x0000ff00) 
                ^ (te4_new_0 & 0x000000ff);
            
            let rk13 = rk5 ^ rk12;
            let rk14 = rk6 ^ rk13;
            let rk15 = rk7_val ^ rk14;
            
            *rk.offset(12) = rk12;
            *rk.offset(13) = rk13;
            *rk.offset(14) = rk14;
            *rk.offset(15) = rk15;
            
            rk = rk.offset(8);
        }
    }
}
}

// ── `return 0;` · aes.c:833-833 · chunk 77 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk77 {
// Rust wrapper for AES_set_encrypt_key__part77
// Replaces return 0; in AES_set_encrypt_key function

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_set_encrypt_key__part77() -> c_int {
    0
}
}

// ── `u32 *rk; int i, j, status; u32 temp; status = AES_set_encrypt_key(userKey, bits, key);` · aes.c:842-847 · chunk 79 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     u32 *rk;
//      int i, j, status;
//      u32 temp;
//     
//     
//      status = AES_set_encrypt_key(userKey, bits, key);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk79 {
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_uchar;

#[repr(C)]
struct AesKey {
    rd_key: [c_uint; 4 * (14 + 1)],
    rounds: c_int,
}

extern "C" {
    fn AES_set_encrypt_key(userKey: *const c_uchar, bits: c_int, key: *mut AesKey) -> c_int;
}

#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part79(
    userKey: *const c_uchar,
    bits: c_int,
    key: *mut AesKey,
) -> c_int {
    let mut status: c_int;
    status = unsafe { AES_set_encrypt_key(userKey, bits, key) };
    status
}
}

// ── `if (status < 0)` · aes.c:848-849 · chunk 80 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (status < 0)
//       return status;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk80 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part80(status: c_int) -> c_int {
    if status < 0 {
        status
    } else {
        0
    }
}
}

// ── `rk = key->rd_key;` · aes.c:851-851 · chunk 81 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     rk = key->rd_key;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk81 {
use std::os::raw::c_uint;
use std::os::raw::c_int;

#[repr(C)]
struct AES_KEY {
    rd_key: [c_uint; 4 * (14 + 1)],
    rounds: c_int,
}

#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part81(key: *const AES_KEY) -> *mut c_uint {
    unsafe {
        (*key).rd_key.as_ptr() as *mut c_uint
    }
}
}

// ── `for (i = 0, j = 4*(key->rounds; i < j; i += 4, j -= 4)` · aes.c:854-859 · chunk 82 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0, j = 4*(key->rounds); i < j; i += 4, j -= 4) {
//       temp = rk[i ]; rk[i ] = rk[j ]; rk[j ] = temp;
//       temp = rk[i + 1]; rk[i + 1] = rk[j + 1]; rk[j + 1] = temp;
//       temp = rk[i + 2]; rk[i + 2] = rk[j + 2]; rk[j + 2] = temp;
//       temp = rk[i + 3]; rk[i + 3] = rk[j + 3]; rk[j + 3] = temp;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk82 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part82(rk: *mut u32, rounds: c_int) {
    let mut i: c_int = 0;
    let mut j: c_int = 4 * rounds;
    let mut temp: u32;

    while i < j {
        unsafe {
            temp = *rk.add(i as usize);
            *rk.add(i as usize) = *rk.add(j as usize);
            *rk.add(j as usize) = temp;

            temp = *rk.add(i as usize + 1);
            *rk.add(i as usize + 1) = *rk.add(j as usize + 1);
            *rk.add(j as usize + 1) = temp;

            temp = *rk.add(i as usize + 2);
            *rk.add(i as usize + 2) = *rk.add(j as usize + 2);
            *rk.add(j as usize + 2) = temp;

            temp = *rk.add(i as usize + 3);
            *rk.add(i as usize + 3) = *rk.add(j as usize + 3);
            *rk.add(j as usize + 3) = temp;
        }

        i += 4;
        j -= 4;
    }
}
}

// ── `return 0;` · aes.c:884-884 · chunk 84 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk84 {
use core::ffi::c_int;
use std::os::raw::c_uchar;

#[repr(C)]
struct aes_key_st {
    rd_key: [u32; 60],
    rounds: c_int,
}

#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part84(
    user_key: *const c_uchar,
    bits: c_int,
    key: *mut aes_key_st,
) -> c_int {
    0
}
}

// ── `for (; ; )` · aes.c:988-1043 · chunk 87 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     for (;;) {
//             t0 =
//                 Te0[(s0 >> 24) ] ^
//                 Te1[(s1 >> 16) & 0xff] ^
//                 Te2[(s2 >> 8) & 0xff] ^
//                 Te3[(s3 ) & 0xff] ^
//                 rk[4];
//             t1 =
//                 Te0[(s1 >> 24) ] ^
//                 Te1[(s2 >> 16) & 0xff] ^
//                 Te2[(s3 >> 8) & 0xff] ^
//                 Te3[(s0 ) & 0xff] ^
//                 rk[5];
//             t2 =
//                 Te0[(s2 >> 24) ] ^
//                 Te1[(s3 >> 16) & 0xff] ^
//                 Te2[(s0 >> 8) & 0xff] ^
//                 Te3[(s1 ) & 0xff] ^
//                 rk[6];
//             t3 =
//                 Te0[(s3 >> 24) ] ^
//                 Te1[(s0 >> 16) & 0xff] ^
//                 Te2[(s1 >> 8) & 0xff] ^
//                 Te3[(s2 ) & 0xff] ^
//                 rk[7];
//     
//             rk += 8;
//             if (--r == 0) {
//                 break;
//             }
//     
//             s0 =
//                 Te0[(t0 >> 24) ] ^
//                 Te1[(t1 >> 16) & 0xff] ^
//                 Te2[(t2 >> 8) & 0xff] ^
//                 Te3[(t3 ) & 0xff] ^
//                 rk[0];
//             s1 =
//                 Te0[(t1 >> 24) ] ^
//                 Te1[(t2 >> 16) & 0xff] ^
//                 Te2[(t3 >> 8) & 0xff] ^
//                 Te3[(t0 ) & 0xff] ^
//                 rk[1];
//             s2 =
//                 Te0[(t2 >> 24) ] ^
//                 Te1[(t3 >> 16) & 0xff] ^
//                 Te2[(t0 >> 8) & 0xff] ^
//                 Te3[(t1 ) & 0xff] ^
//                 rk[2];
//             s3 =
//                 Te0[(t3 >> 24) ] ^
//                 Te1[(t0 >> 16) & 0xff] ^
//                 Te2[(t1 >> 8) & 0xff] ^
//                 Te3[(t2 ) & 0xff] ^
//                 rk[3];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk87 {
use core::ffi::c_uint;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn AES_encrypt__part87(
    s0: *mut c_uint,
    s1: *mut c_uint,
    s2: *mut c_uint,
    s3: *mut c_uint,
    r: *mut c_int,
    rk: *const c_uint,
    te0: *const c_uint,
    te1: *const c_uint,
    te2: *const c_uint,
    te3: *const c_uint,
    t0: *mut c_uint,
    t1: *mut c_uint,
    t2: *mut c_uint,
    t3: *mut c_uint,
) {
    unsafe {
        let mut r_val = *r;
        let mut s0_val = *s0;
        let mut s1_val = *s1;
        let mut s2_val = *s2;
        let mut s3_val = *s3;

        let mut rk_ptr = rk;
        let mut t0_val: c_uint;
        let mut t1_val: c_uint;
        let mut t2_val: c_uint;
        let mut t3_val: c_uint;

        loop {
            t0_val = *te0.add(((s0_val >> 24) & 0xff) as usize) ^
                *te1.add(((s1_val >> 16) & 0xff) as usize) ^
                *te2.add(((s2_val >> 8) & 0xff) as usize) ^
                *te3.add((s3_val & 0xff) as usize) ^
                *rk_ptr.add(4);
            t1_val = *te0.add(((s1_val >> 24) & 0xff) as usize) ^
                *te1.add(((s2_val >> 16) & 0xff) as usize) ^
                *te2.add(((s3_val >> 8) & 0xff) as usize) ^
                *te3.add((s0_val & 0xff) as usize) ^
                *rk_ptr.add(5);
            t2_val = *te0.add(((s2_val >> 24) & 0xff) as usize) ^
                *te1.add(((s3_val >> 16) & 0xff) as usize) ^
                *te2.add(((s0_val >> 8) & 0xff) as usize) ^
                *te3.add((s1_val & 0xff) as usize) ^
                *rk_ptr.add(6);
            t3_val = *te0.add(((s3_val >> 24) & 0xff) as usize) ^
                *te1.add(((s0_val >> 16) & 0xff) as usize) ^
                *te2.add(((s1_val >> 8) & 0xff) as usize) ^
                *te3.add((s2_val & 0xff) as usize) ^
                *rk_ptr.add(7);

            rk_ptr = rk_ptr.add(8);
            r_val -= 1;
            if r_val == 0 {
                break;
            }

            s0_val = *te0.add((t0_val >> 24) as usize) ^
                *te1.add(((t1_val >> 16) & 0xff) as usize) ^
                *te2.add(((t2_val >> 8) & 0xff) as usize) ^
                *te3.add((t3_val & 0xff) as usize) ^
                *rk_ptr;
            s1_val = *te0.add((t1_val >> 24) as usize) ^
                *te1.add(((t2_val >> 16) & 0xff) as usize) ^
                *te2.add(((t3_val >> 8) & 0xff) as usize) ^
                *te3.add((t0_val & 0xff) as usize) ^
                *rk_ptr.add(1);
            s2_val = *te0.add((t2_val >> 24) as usize) ^
                *te1.add(((t3_val >> 16) & 0xff) as usize) ^
                *te2.add(((t0_val >> 8) & 0xff) as usize) ^
                *te3.add((t1_val & 0xff) as usize) ^
                *rk_ptr.add(2);
            s3_val = *te0.add((t3_val >> 24) as usize) ^
                *te1.add(((t0_val >> 16) & 0xff) as usize) ^
                *te2.add(((t1_val >> 8) & 0xff) as usize) ^
                *te3.add((t2_val & 0xff) as usize) ^
                *rk_ptr.add(3);
        }

        *r = r_val;
        *s0 = s0_val;
        *s1 = s1_val;
        *s2 = s2_val;
        *s3 = s3_val;
        *t0 = t0_val;
        *t1 = t1_val;
        *t2 = t2_val;
        *t3 = t3_val;
    }
}
}

// ── `s0 = (Te4[(t0 >> 24) ] & 0xff000000) ^ (Te4[(t1 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t2 >> 8) &…` · aes.c:1049-1054 · chunk 88 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     s0 =
//       (Te4[(t0 >> 24) ] & 0xff000000) ^
//       (Te4[(t1 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t2 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t3 ) & 0xff] & 0x000000ff) ^
//       rk[0];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk88 {
use core::ffi::c_uint;

static TE4: [c_uint; 256] = [
    0x63636363, 0x7c7c7c7c, 0x77777777, 0x7b7b7b7b,
    0xf2f2f2f2, 0x6b6b6b6b, 0x6f6f6f6f, 0xc5c5c5c5,
    0x30303030, 0x01010101, 0x67676767, 0x2b2b2b2b,
    0xfefefefe, 0xd7d7d7d7, 0xabababab, 0x76767676,
    0xcacacaca, 0x82828282, 0xc9c9c9c9, 0x7d7d7d7d,
    0xfafafafa, 0x59595959, 0x47474747, 0xf0f0f0f0,
    0xadadadad, 0xd4d4d4d4, 0xa2a2a2a2, 0xafafafaf,
    0x9c9c9c9c, 0xa4a4a4a4, 0x72727272, 0xc0c0c0c0,
    0xb7b7b7b7, 0xfdfdfdfd, 0x93939393, 0x26262626,
    0x36363636, 0x3f3f3f3f, 0xf7f7f7f7, 0xcccccccc,
    0x34343434, 0xa5a5a5a5, 0xe5e5e5e5, 0xf1f1f1f1,
    0x71717171, 0xd8d8d8d8, 0x31313131, 0x15151515,
    0x04040404, 0xc7c7c7c7, 0x23232323, 0xc3c3c3c3,
    0x18181818, 0x96969696, 0x05050505, 0x9a9a9a9a,
    0x07070707, 0x12121212, 0x80808080, 0xe2e2e2e2,
    0xebebebeb, 0x27272727, 0xb2b2b2b2, 0x75757575,
    0x09090909, 0x83838383, 0x2c2c2c2c, 0x1a1a1a1a,
    0x1b1b1b1b, 0x6e6e6e6e, 0x5a5a5a5a, 0xa0a0a0a0,
    0x52525252, 0x3b3b3b3b, 0xd6d6d6d6, 0xb3b3b3b3,
    0x29292929, 0xe3e3e3e3, 0x2f2f2f2f, 0x84848484,
    0x53535353, 0xd1d1d1d1, 0x00000000, 0xededede,
    0x20202020, 0xfcfcfcfc, 0xb1b1b1b1, 0x5b5b5b5b,
    0x6a6a6a6a, 0xcbcbcbcb, 0xbebebebe, 0x39393939,
    0x4a4a4a4a, 0x4c4c4c4c, 0x58585858, 0xcfcfcfcf,
    0xd0d0d0d0, 0xefefefef, 0xaaaaaaaa, 0xfbfbfbfb,
    0x43434343, 0x4d4d4d4d, 0x33333333, 0x85858585,
    0x45454545, 0xf9f9f9f9, 0x02020202, 0x7f7f7f7f,
    0x50505050, 0x3c3c3c3c, 0x9f9f9f9f, 0xa8a8a8a8,
    0x51515151, 0xa3a3a3a3, 0x40404040, 0x8f8f8f8f,
    0x92929292, 0x9d9d9d9d, 0x38383838, 0xf5f5f5f5,
    0xbcbcbcbc, 0xb6b6b6b6, 0xdadadada, 0x21212121,
    0x10101010, 0xffffffff, 0xf3f3f3f3, 0xd2d2d2d2,
    0xcdcdcdcd, 0x0c0c0c0c, 0x13131313, 0xecececec,
    0x5f5f5f5f, 0x97979797, 0x44444444, 0x17171717,
    0xc4c4c4c4, 0xa7a7a7a7, 0x7e7e7e7e, 0x3d3d3d3d,
    0x64646464, 0x5d5d5d5d, 0x19191919, 0x73737373,
    0x60606060, 0x81818181, 0x4f4f4f4f, 0xdcdcdcdc,
    0x22222222, 0x2a2a2a2a, 0x90909090, 0x88888888,
    0x46464646, 0xeeeeeeee, 0xb8b8b8b8, 0x14141414,
    0xdededede, 0x5e5e5e5e, 0x0b0b0b0b, 0xdbdbdbdb,
    0xe0e0e0e0, 0x32323232, 0x3a3a3a3a, 0x0a0a0a0a,
    0x49494949, 0x06060606, 0x24242424, 0x5c5c5c5c,
    0xc2c2c2c2, 0xd3d3d3d3, 0xacacacac, 0x62626262,
    0x91919191, 0x95959595, 0xe4e4e4e4, 0x79797979,
    0xe7e7e7e7, 0xc8c8c8c8, 0x37373737, 0x6d6d6d6d,
    0x8d8d8d8d, 0xd5d5d5d5, 0x4e4e4e4e, 0xa9a9a9a9,
    0x6c6c6c6c, 0x56565656, 0xf4f4f4f4, 0xeaeaeaea,
    0x65656565, 0x7a7a7a7a, 0xaeaeaeae, 0x08080808,
    0xbabababa, 0x78787878, 0x25252525, 0x2e2e2e2e,
    0x1c1c1c1c, 0xa6a6a6a6, 0xb4b4b4b4, 0xc6c6c6c6,
    0xe8e8e8e8, 0xdddddddd, 0x74747474, 0x1f1f1f1f,
    0x4b4b4b4b, 0xbdbdbdbd, 0x8b8b8b8b, 0x8a8a8a8a,
    0x70707070, 0x3e3e3e3e, 0xb5b5b5b5, 0x66666666,
    0x48484848, 0x03030303, 0xf6f6f6f6, 0x0e0e0e0e,
    0x61616161, 0x35353535, 0x57575757, 0xb9b9b9b9,
    0x86868686, 0xc1c1c1c1, 0x1d1d1d1d, 0x9e9e9e9e,
    0xe1e1e1e1, 0xf8f8f8f8, 0x98989898, 0x11111111,
    0x69696969, 0xd9d9d9d9, 0x8e8e8e8e, 0x94949494,
    0x9b9b9b9b, 0x1e1e1e1e, 0x87878787, 0xe9e9e9e9,
    0xcececece, 0x55555555, 0x28282828, 0xdfdfdfdf,
    0x8c8c8c8c, 0xa1a1a1a1, 0x89898989, 0x0d0d0d0d,
    0xbfbfbfbf, 0xe6e6e6e6, 0x42424242, 0x68686868,
    0x41414141, 0x99999999, 0x2d2d2d2d, 0x0f0f0f0f,
    0xb0b0b0b0, 0x54545454, 0xbbbbbbbb, 0x16161616,
];

#[no_mangle]
pub extern "C" fn AES_encrypt__part88(t0: c_uint, t1: c_uint, t2: c_uint, t3: c_uint, rk: *const c_uint) -> c_uint {
    unsafe {
        let i0 = (t0 >> 24) as usize;
        let i1 = ((t1 >> 16) & 0xff) as usize;
        let i2 = ((t2 >> 8) & 0xff) as usize;
        let i3 = (t3 & 0xff) as usize;
        let s0 = (TE4[i0] & 0xff000000) ^
                 (TE4[i1] & 0x00ff0000) ^
                 (TE4[i2] & 0x0000ff00) ^
                 (TE4[i3] & 0x000000ff) ^
                 *rk;
        s0
    }
}
}

// ── `{ … }` · aes.c:1055-1055 · chunk 89 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     { (out)[0] = (u8)((s0) >> 24); (out)[1] = (u8)((s0) >> 16); (out)[2] = (u8)((s0) >> 8); (out)[3] = (u8)(s0); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk89 {
use core::ffi::c_uchar;
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn AES_encrypt__part89(out: *mut c_uchar, s0: c_uint) {
    unsafe {
        *out.offset(0) = ((s0) >> 24) as c_uchar;
        *out.offset(1) = ((s0) >> 16) as c_uchar;
        *out.offset(2) = ((s0) >> 8) as c_uchar;
        *out.offset(3) = (s0) as c_uchar;
    }
}
}

// ── `; s1 = (Te4[(t1 >> 24) ] & 0xff000000) ^ (Te4[(t2 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t3 >> 8)…` · aes.c:1055-1061 · chunk 90 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      s1 =
//       (Te4[(t1 >> 24) ] & 0xff000000) ^
//       (Te4[(t2 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t3 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t0 ) & 0xff] & 0x000000ff) ^
//       rk[1];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk90 {
#[no_mangle]
pub extern "C" fn AES_encrypt__part90(
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    rk1: u32,
    out: *mut u8,
) {
    const TE4: [u32; 256] = [
        0x63636363u32, 0x7c7c7c7cu32, 0x77777777u32, 0x7b7b7b7bu32,
        0xf2f2f2f2u32, 0x6b6b6b6bu32, 0x6f6f6f6fu32, 0xc5c5c5c5u32,
        0x30303030u32, 0x01010101u32, 0x67676767u32, 0x2b2b2b2bu32,
        0xfefefefeu32, 0xd7d7d7d7u32, 0xababababu32, 0x76767676u32,
        0xcacacacau32, 0x82828282u32, 0xc9c9c9c9u32, 0x7d7d7d7du32,
        0xfafafafau32, 0x59595959u32, 0x47474747u32, 0xf0f0f0f0u32,
        0xadadadadu32, 0xd4d4d4d4u32, 0xa2a2a2a2u32, 0xafafafafu32,
        0x9c9c9c9cu32, 0xa4a4a4a4u32, 0x72727272u32, 0xc0c0c0c0u32,
        0xb7b7b7b7u32, 0xfdfdfdfdu32, 0x93939393u32, 0x26262626u32,
        0x36363636u32, 0x3f3f3f3fu32, 0xf7f7f7f7u32, 0xccccccccu32,
        0x34343434u32, 0xa5a5a5a5u32, 0xe5e5e5e5u32, 0xf1f1f1f1u32,
        0x71717171u32, 0xd8d8d8d8u32, 0x31313131u32, 0x15151515u32,
        0x04040404u32, 0xc7c7c7c7u32, 0x23232323u32, 0xc3c3c3c3u32,
        0x18181818u32, 0x96969696u32, 0x05050505u32, 0x9a9a9a9au32,
        0x07070707u32, 0x12121212u32, 0x80808080u32, 0xe2e2e2e2u32,
        0xebebebebu32, 0x27272727u32, 0xb2b2b2b2u32, 0x75757575u32,
        0x09090909u32, 0x83838383u32, 0x2c2c2c2cu32, 0x1a1a1a1au32,
        0x1b1b1b1bu32, 0x6e6e6e6eu32, 0x5a5a5a5au32, 0xa0a0a0a0u32,
        0x52525252u32, 0x3b3b3b3bu32, 0xd6d6d6d6u32, 0xb3b3b3b3u32,
        0x29292929u32, 0xe3e3e3e3u32, 0x2f2f2f2fu32, 0x84848484u32,
        0x53535353u32, 0xd1d1d1d1u32, 0x00000000u32, 0xedededu32,
        0x20202020u32, 0xfcfcfcfcu32, 0xb1b1b1b1u32, 0x5b5b5b5bu32,
        0x6a6a6a6au32, 0xcbcbcbcu32, 0xbebebebeu32, 0x39393939u32,
        0x4a4a4a4au32, 0x4c4c4c4cu32, 0x58585858u32, 0xcfcfcfcfu32,
        0xd0d0d0d0u32, 0xefefefefu32, 0xaaaaaaau32, 0xfbfbfbfbu32,
        0x43434343u32, 0x4d4d4d4du32, 0x33333333u32, 0x85858585u32,
        0x45454545u32, 0xf9f9f9f9u32, 0x02020202u32, 0x7f7f7f7fu32,
        0x50505050u32, 0x3c3c3c3cu32, 0x9f9f9f9fu32, 0xa8a8a8a8u32,
        0x51515151u32, 0xa3a3a3a3u32, 0x40404040u32, 0x8f8f8f8fu32,
        0x92929292u32, 0x9d9d9d9du32, 0x38383838u32, 0xf5f5f5f5u32,
        0xbcbcbcbcu32, 0xb6b6b6b6u32, 0xdadadadu32, 0x21212121u32,
        0x10101010u32, 0xffffffffu32, 0xf3f3f3f3u32, 0xd2d2d2d2u32,
        0xcdcdcdcdu32, 0x0c0c0c0cu32, 0x13131313u32, 0xececececu32,
        0x5f5f5f5fu32, 0x97979797u32, 0x44444444u32, 0x17171717u32,
        0xc4c4c4c4u32, 0xa7a7a7a7u32, 0x7e7e7e7eu32, 0x3d3d3d3du32,
        0x64646464u32, 0x5d5d5d5du32, 0x19191919u32, 0x73737373u32,
        0x60606060u32, 0x81818181u32, 0x4f4f4f4fu32, 0xdcdcdcdu32,
        0x22222222u32, 0x2a2a2a2au32, 0x90909090u32, 0x88888888u32,
        0x46464646u32, 0xeeeeeeeeu32, 0xb8b8b8b8u32, 0x14141414u32,
        0xdedededeu32, 0x5e5e5e5eu32, 0x0b0b0b0bu32, 0xdbdbdbdu32,
        0xe0e0e0e0u32, 0x32323232u32, 0x3a3a3a3au32, 0x0a0a0a0au32,
        0x49494949u32, 0x06060606u32, 0x24242424u32, 0x5c5c5c5cu32,
        0xc2c2c2c2u32, 0xd3d3d3d3u32, 0xacacacacu32, 0x62626262u32,
        0x91919191u32, 0x95959595u32, 0xe4e4e4e4u32, 0x79797979u32,
        0xe7e7e7e7u32, 0xc8c8c8c8u32, 0x37373737u32, 0x6d6d6d6du32,
        0x8d8d8d8du32, 0xd5d5d5d5u32, 0x4e4e4e4eu32, 0xa9a9a9a9u32,
        0x6c6c6c6cu32, 0x56565656u32, 0xf4f4f4f4u32, 0xeaeaeaeau32,
        0x65656565u32, 0x7a7a7a7au32, 0xaeaeaeaeu32, 0x08080808u32,
        0xbabababu32, 0x78787878u32, 0x25252525u32, 0x2e2e2e2eu32,
        0x1c1c1c1cu32, 0xa6a6a6a6u32, 0xb4b4b4b4u32, 0xc6c6c6c6u32,
        0xe8e8e8e8u32, 0xdddddddu32, 0x74747474u32, 0x1f1f1f1fu32,
        0x4b4b4b4bu32, 0xbdbdbdbu32, 0x8b8b8b8bu32, 0x8a8a8a8au32,
        0x70707070u32, 0x3e3e3e3eu32, 0xb5b5b5b5u32, 0x66666666u32,
        0x48484848u32, 0x03030303u32, 0xf6f6f6f6u32, 0x0e0e0e0eu32,
        0x61616161u32, 0x35353535u32, 0x57575757u32, 0xb9b9b9b9u32,
        0x86868686u32, 0xc1c1c1c1u32, 0x1d1d1d1du32, 0x9e9e9e9eu32,
        0xe1e1e1e1u32, 0xf8f8f8f8u32, 0x98989898u32, 0x11111111u32,
        0x69696969u32, 0xd9d9d9d9u32, 0x8e8e8e8eu32, 0x94949494u32,
        0x9b9b9b9bu32, 0x1e1e1e1eu32, 0x87878787u32, 0xe9e9e9e9u32,
        0xcecececeu32, 0x55555555u32, 0x28282828u32, 0xdfdfdfdfu32,
        0x8c8c8c8cu32, 0xa1a1a1a1u32, 0x89898989u32, 0x0d0d0d0du32,
        0xbfbfbfbfu32, 0xe6e6e6e6u32, 0x42424242u32, 0x68686868u32,
        0x41414141u32, 0x99999999u32, 0x2d2d2d2du32, 0x0f0f0f0fu32,
        0xb0b0b0b0u32, 0x54545454u32, 0xbbbbbbbbu32, 0x16161616u32,
    ];

    let s1 = (
        TE4[((t1 >> 24) & 0xff) as usize] & 0xff000000u32 ^
        TE4[((t2 >> 16) & 0xff) as usize] & 0x00ff0000u32 ^
        TE4[((t3 >> 8) & 0xff) as usize] & 0x0000ff00u32 ^
        TE4[(t0 & 0xff) as usize] & 0x000000ffu32 ^
        rk1
    );

    unsafe {
        *out.add(0) = ((s1) >> 24) as u8;
        *out.add(1) = ((s1) >> 16) as u8;
        *out.add(2) = ((s1) >> 8) as u8;
        *out.add(3) = (s1) as u8;
    }
}
}

// ── `{ … }` · aes.c:1062-1062 · chunk 91 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 4)[0] = (u8)((s1) >> 24); (out + 4)[1] = (u8)((s1) >> 16); (out + 4)[2] = (u8)((s1) >> 8); (out + 4)[3] = (u8)(s1); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk91 {
use core::ffi::c_uint;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn AES_encrypt__part91(s1: c_uint, mut out: *mut c_uchar) {
    unsafe {
        *out = ((s1) >> 24) as c_uchar;
        out = out.add(1);
        *out = ((s1) >> 16) as c_uchar;
        out = out.add(1);
        *out = ((s1) >> 8) as c_uchar;
        out = out.add(1);
        *out = (s1) as c_uchar;
    }
}
}

// ── `; s2 = (Te4[(t2 >> 24) ] & 0xff000000) ^ (Te4[(t3 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t0 >> 8)…` · aes.c:1062-1068 · chunk 92 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      s2 =
//       (Te4[(t2 >> 24) ] & 0xff000000) ^
//       (Te4[(t3 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t0 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t1 ) & 0xff] & 0x000000ff) ^
//       rk[2];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk92 {
static TE4: [u32; 256] = [
    0x63636363, 0x7c7c7c7c, 0x77777777, 0x7b7b7b7b,
    0xf2f2f2f2, 0x6b6b6b6b, 0x6f6f6f6f, 0xc5c5c5c5,
    0x30303030, 0x01010101, 0x67676767, 0x2b2b2b2b,
    0xfefefefe, 0xd7d7d7d7, 0xabababab, 0x76767676,
    0xcacacaca, 0x82828282, 0xc9c9c9c9, 0x7d7d7d7d,
    0xfafafafa, 0x59595959, 0x47474747, 0xf0f0f0f0,
    0xadadadad, 0xd4d4d4d4, 0xa2a2a2a2, 0xafafafaf,
    0x9c9c9c9c, 0xa4a4a4a4, 0x72727272, 0xc0c0c0c0,
    0xb7b7b7b7, 0xfdfdfdfd, 0x93939393, 0x26262626,
    0x36363636, 0x3f3f3f3f, 0xf7f7f7f7, 0xcccccccc,
    0x34343434, 0xa5a5a5a5, 0xe5e5e5e5, 0xf1f1f1f1,
    0x71717171, 0xd8d8d8d8, 0x31313131, 0x15151515,
    0x04040404, 0xc7c7c7c7, 0x23232323, 0xc3c3c3c3,
    0x18181818, 0x96969696, 0x05050505, 0x9a9a9a9a,
    0x07070707, 0x12121212, 0x80808080, 0xe2e2e2e2,
    0xebebebeb, 0x27272727, 0xb2b2b2b2, 0x75757575,
    0x09090909, 0x83838383, 0x2c2c2c2c, 0x1a1a1a1a,
    0x1b1b1b1b, 0x6e6e6e6e, 0x5a5a5a5a, 0xa0a0a0a0,
    0x52525252, 0x3b3b3b3b, 0xd6d6d6d6, 0xb3b3b3b3,
    0x29292929, 0xe3e3e3e3, 0x2f2f2f2f, 0x84848484,
    0x53535353, 0xd1d1d1d1, 0x00000000, 0xedededed,
    0x20202020, 0xfcfcfcfc, 0xb1b1b1b1, 0x5b5b5b5b,
    0x6a6a6a6a, 0xcbcbcbcb, 0xbebebebe, 0x39393939,
    0x4a4a4a4a, 0x4c4c4c4c, 0x58585858, 0xcfcfcfcf,
    0xd0d0d0d0, 0xefefefef, 0xaaaaaaaa, 0xfbfbfbfb,
    0x43434343, 0x4d4d4d4d, 0x33333333, 0x85858585,
    0x45454545, 0xf9f9f9f9, 0x02020202, 0x7f7f7f7f,
    0x50505050, 0x3c3c3c3c, 0x9f9f9f9f, 0xa8a8a8a8,
    0x51515151, 0xa3a3a3a3, 0x40404040, 0x8f8f8f8f,
    0x92929292, 0x9d9d9d9d, 0x38383838, 0xf5f5f5f5,
    0xbcbcbcbc, 0xb6b6b6b6, 0xdadadada, 0x21212121,
    0x10101010, 0xffffffff, 0xf3f3f3f3, 0xd2d2d2d2,
    0xcdcdcdcd, 0x0c0c0c0c, 0x13131313, 0xecececec,
    0x5f5f5f5f, 0x97979797, 0x44444444, 0x17171717,
    0xc4c4c4c4, 0xa7a7a7a7, 0x7e7e7e7e, 0x3d3d3d3d,
    0x64646464, 0x5d5d5d5d, 0x19191919, 0x73737373,
    0x60606060, 0x81818181, 0x4f4f4f4f, 0xdcdcdcdc,
    0x22222222, 0x2a2a2a2a, 0x90909090, 0x88888888,
    0x46464646, 0xeeeeeeee, 0xb8b8b8b8, 0x14141414,
    0xdededede, 0x5e5e5e5e, 0x0b0b0b0b, 0xdbdbdbdb,
    0xe0e0e0e0, 0x32323232, 0x3a3a3a3a, 0x0a0a0a0a,
    0x49494949, 0x06060606, 0x24242424, 0x5c5c5c5c,
    0xc2c2c2c2, 0xd3d3d3d3, 0xacacacac, 0x62626262,
    0x91919191, 0x95959595, 0xe4e4e4e4, 0x79797979,
    0xe7e7e7e7, 0xc8c8c8c8, 0x37373737, 0x6d6d6d6d,
    0x8d8d8d8d, 0xd5d5d5d5, 0x4e4e4e4e, 0xa9a9a9a9,
    0x6c6c6c6c, 0x56565656, 0xf4f4f4f4, 0xeaeaeaea,
    0x65656565, 0x7a7a7a7a, 0xaeaeaeae, 0x08080808,
    0xbabababa, 0x78787878, 0x25252525, 0x2e2e2e2e,
    0x1c1c1c1c, 0xa6a6a6a6, 0xb4b4b4b4, 0xc6c6c6c6,
    0xe8e8e8e8, 0xdddddddd, 0x74747474, 0x1f1f1f1f,
    0x4b4b4b4b, 0xbdbdbdbd, 0x8b8b8b8b, 0x8a8a8a8a,
    0x70707070, 0x3e3e3e3e, 0xb5b5b5b5, 0x66666666,
    0x48484848, 0x03030303, 0xf6f6f6f6, 0x0e0e0e0e,
    0x61616161, 0x35353535, 0x57575757, 0xb9b9b9b9,
    0x86868686, 0xc1c1c1c1, 0x1d1d1d1d, 0x9e9e9e9e,
    0xe1e1e1e1, 0xf8f8f8f8, 0x98989898, 0x11111111,
    0x69696969, 0xd9d9d9d9, 0x8e8e8e8e, 0x94949494,
    0x9b9b9b9b, 0x1e1e1e1e, 0x87878787, 0xe9e9e9e9,
    0xcececece, 0x55555555, 0x28282828, 0xdfdfdfdf,
    0x8c8c8c8c, 0xa1a1a1a1, 0x89898989, 0x0d0d0d0d,
    0xbfbfbfbf, 0xe6e6e6e6, 0x42424242, 0x68686868,
    0x41414141, 0x99999999, 0x2d2d2d2d, 0x0f0f0f0f,
    0xb0b0b0b0, 0x54545454, 0xbbbbbbbb, 0x16161616,
];

#[no_mangle]
pub extern "C" fn AES_encrypt__part92(t0: u32, t1: u32, t2: u32, t3: u32, rk2: u32) -> u32 {
    let s2 = (TE4[((t2 >> 24) & 0xff) as usize] & 0xff000000) ^
             (TE4[((t3 >> 16) & 0xff) as usize] & 0x00ff0000) ^
             (TE4[((t0 >> 8) & 0xff) as usize] & 0x0000ff00) ^
             (TE4[((t1 ) & 0xff) as usize] & 0x000000ff) ^
             rk2;
    s2
}
}

// ── `{ … }` · aes.c:1069-1069 · chunk 93 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 8)[0] = (u8)((s2) >> 24); (out + 8)[1] = (u8)((s2) >> 16); (out + 8)[2] = (u8)((s2) >> 8); (out + 8)[3] = (u8)(s2); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk93 {
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn AES_encrypt__part93(s2: c_uint, out: *mut u8) {
    unsafe {
        let ptr = out;
        *ptr.add(0) = ((s2) >> 24) as u8;
        *ptr.add(1) = ((s2) >> 16) as u8;
        *ptr.add(2) = ((s2) >> 8) as u8;
        *ptr.add(3) = (s2) as u8;
    }
}
}

// ── `; s3 = (Te4[(t3 >> 24) ] & 0xff000000) ^ (Te4[(t0 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t1 >> 8)…` · aes.c:1069-1075 · chunk 94 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      s3 =
//       (Te4[(t3 >> 24) ] & 0xff000000) ^
//       (Te4[(t0 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t1 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t2 ) & 0xff] & 0x000000ff) ^
//       rk[3];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk94 {
use core::ffi::c_uint;

const TE4: [u32; 256] = [
    0x63636363, 0x7c7c7c7c, 0x77777777, 0x7b7b7b7b,
    0xf2f2f2f2, 0x6b6b6b6b, 0x6f6f6f6f, 0xc5c5c5c5,
    0x30303030, 0x01010101, 0x67676767, 0x2b2b2b2b,
    0xfefefefe, 0xd7d7d7d7, 0xabababab, 0x76767676,
    0xcacacaca, 0x82828282, 0xc9c9c9c9, 0x7d7d7d7d,
    0xfafafafa, 0x59595959, 0x47474747, 0xf0f0f0f0,
    0xadadadad, 0xd4d4d4d4, 0xa2a2a2a2, 0xafafafaf,
    0x9c9c9c9c, 0xa4a4a4a4, 0x72727272, 0xc0c0c0c0,
    0xb7b7b7b7, 0xfdfdfdfd, 0x93939393, 0x26262626,
    0x36363636, 0x3f3f3f3f, 0xf7f7f7f7, 0xcccccccc,
    0x34343434, 0xa5a5a5a5, 0xe5e5e5e5, 0xf1f1f1f1,
    0x71717171, 0xd8d8d8d8, 0x31313131, 0x15151515,
    0x04040404, 0xc7c7c7c7, 0x23232323, 0xc3c3c3c3,
    0x18181818, 0x96969696, 0x05050505, 0x9a9a9a9a,
    0x07070707, 0x12121212, 0x80808080, 0xe2e2e2e2,
    0xebebebeb, 0x27272727, 0xb2b2b2b2, 0x75757575,
    0x09090909, 0x83838383, 0x2c2c2c2c, 0x1a1a1a1a,
    0x1b1b1b1b, 0x6e6e6e6e, 0x5a5a5a5a, 0xa0a0a0a0,
    0x52525252, 0x3b3b3b3b, 0xd6d6d6d6, 0xb3b3b3b3,
    0x29292929, 0xe3e3e3e3, 0x2f2f2f2f, 0x84848484,
    0x53535353, 0xd1d1d1d1, 0x00000000, 0xedededed,
    0x20202020, 0xfcfcfcfc, 0xb1b1b1b1, 0x5b5b5b5b,
    0x6a6a6a6a, 0xcbcbcbcb, 0xbebebebe, 0x39393939,
    0x4a4a4a4a, 0x4c4c4c4c, 0x58585858, 0xcfcfcfcf,
    0xd0d0d0d0, 0xefefefef, 0xaaaaaaaa, 0xfbfbfbfb,
    0x43434343, 0x4d4d4d4d, 0x33333333, 0x85858585,
    0x45454545, 0xf9f9f9f9, 0x02020202, 0x7f7f7f7f,
    0x50505050, 0x3c3c3c3c, 0x9f9f9f9f, 0xa8a8a8a8,
    0x51515151, 0xa3a3a3a3, 0x40404040, 0x8f8f8f8f,
    0x92929292, 0x9d9d9d9d, 0x38383838, 0xf5f5f5f5,
    0xbcbcbcbc, 0xb6b6b6b6, 0xdadadada, 0x21212121,
    0x10101010, 0xffffffff, 0xf3f3f3f3, 0xd2d2d2d2,
    0xcdcdcdcd, 0x0c0c0c0c, 0x13131313, 0xecececec,
    0x5f5f5f5f, 0x97979797, 0x44444444, 0x17171717,
    0xc4c4c4c4, 0xa7a7a7a7, 0x7e7e7e7e, 0x3d3d3d3d,
    0x64646464, 0x5d5d5d5d, 0x19191919, 0x73737373,
    0x60606060, 0x81818181, 0x4f4f4f4f, 0xdcdcdcdc,
    0x22222222, 0x2a2a2a2a, 0x90909090, 0x88888888,
    0x46464646, 0xeeeeeeee, 0xb8b8b8b8, 0x14141414,
    0xdededede, 0x5e5e5e5e, 0x0b0b0b0b, 0xdbdbdbdb,
    0xe0e0e0e0, 0x32323232, 0x3a3a3a3a, 0x0a0a0a0a,
    0x49494949, 0x06060606, 0x24242424, 0x5c5c5c5c,
    0xc2c2c2c2, 0xd3d3d3d3, 0xacacacac, 0x62626262,
    0x91919191, 0x95959595, 0xe4e4e4e4, 0x79797979,
    0xe7e7e7e7, 0xc8c8c8c8, 0x37373737, 0x6d6d6d6d,
    0x8d8d8d8d, 0xd5d5d5d5, 0x4e4e4e4e, 0xa9a9a9a9,
    0x6c6c6c6c, 0x56565656, 0xf4f4f4f4, 0xeaeaeaea,
    0x65656565, 0x7a7a7a7a, 0xaeaeaeae, 0x08080808,
    0xbabababa, 0x78787878, 0x25252525, 0x2e2e2e2e,
    0x1c1c1c1c, 0xa6a6a6a6, 0xb4b4b4b4, 0xc6c6c6c6,
    0xe8e8e8e8, 0xdddddddd, 0x74747474, 0x1f1f1f1f,
    0x4b4b4b4b, 0xbdbdbdbd, 0x8b8b8b8b, 0x8a8a8a8a,
    0x70707070, 0x3e3e3e3e, 0xb5b5b5b5, 0x66666666,
    0x48484848, 0x03030303, 0xf6f6f6f6, 0x0e0e0e0e,
    0x61616161, 0x35353535, 0x57575757, 0xb9b9b9b9,
    0x86868686, 0xc1c1c1c1, 0x1d1d1d1d, 0x9e9e9e9e,
    0xe1e1e1e1, 0xf8f8f8f8, 0x98989898, 0x11111111,
    0x69696969, 0xd9d9d9d9, 0x8e8e8e8e, 0x94949494,
    0x9b9b9b9b, 0x1e1e1e1e, 0x87878787, 0xe9e9e9e9,
    0xcececece, 0x55555555, 0x28282828, 0xdfdfdfdf,
    0x8c8c8c8c, 0xa1a1a1a1, 0x89898989, 0x0d0d0d0d,
    0xbfbfbfbf, 0xe6e6e6e6, 0x42424242, 0x68686868,
    0x41414141, 0x99999999, 0x2d2d2d2d, 0x0f0f0f0f,
    0xb0b0b0b0, 0x54545454, 0xbbbbbbbb, 0x16161616,
];

#[no_mangle]
pub extern "C" fn AES_encrypt__part94(
    t0: c_uint,
    t1: c_uint,
    t2: c_uint,
    t3: c_uint,
    rk3: c_uint,
) -> c_uint {
    let s3 =
        (TE4[((t3 >> 24) & 0xff) as usize] & 0xff000000) ^
        (TE4[((t0 >> 16) & 0xff) as usize] & 0x00ff0000) ^
        (TE4[((t1 >> 8) & 0xff) as usize] & 0x0000ff00) ^
        (TE4[(t2 & 0xff) as usize] & 0x000000ff) ^
        rk3;
    s3
}
}

// ── `{ … }` · aes.c:1076-1076 · chunk 95 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 12)[0] = (u8)((s3) >> 24); (out + 12)[1] = (u8)((s3) >> 16); (out + 12)[2] = (u8)((s3) >> 8); (out + 12)[3] = (u8)(s3); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk95 {
use core::ffi::c_uint;
use core::ffi::c_uchar;

#[no_mangle] pub extern "C" fn AES_encrypt__part95(s3: c_uint, out: *mut c_uchar) {
    unsafe {
        *out.wrapping_add(0) = ((s3) >> 24) as c_uchar;
        *out.wrapping_add(1) = ((s3) >> 16) as c_uchar;
        *out.wrapping_add(2) = ((s3) >> 8) as c_uchar;
        *out.wrapping_add(3) = (s3) as c_uchar;
    }
}
}

// ── `;` · aes.c:1076-1076 · chunk 96 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk96 {
// Ported AES_encrypt__part96 function
#[no_mangle]
pub extern "C" fn AES_encrypt__part96() {
}
}

// ── `s0 = (Td4[(t0 >> 24) ] & 0xff000000) ^ (Td4[(t3 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t2 >> 8) &…` · aes.c:1240-1245 · chunk 100 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     s0 =
//          (Td4[(t0 >> 24) ] & 0xff000000) ^
//          (Td4[(t3 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t2 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t1 ) & 0xff] & 0x000000ff) ^
//          rk[0];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk100 {
use core::ffi::c_uint;

static Td4: [c_uint; 256] = [
    0x52525252, 0x09090909, 0x6a6a6a6a, 0xd5d5d5d5,
    0x30303030, 0x36363636, 0xa5a5a5a5, 0x38383838,
    0xbfbfbfbf, 0x40404040, 0xa3a3a3a3, 0x9e9e9e9e,
    0x81818181, 0xf3f3f3f3, 0xd7d7d7d7, 0xfbfbfbfb,
    0x7c7c7c7c, 0xe3e3e3e3, 0x39393939, 0x82828282,
    0x9b9b9b9b, 0x2f2f2f2f, 0xffffffff, 0x87878787,
    0x34343434, 0x8e8e8e8e, 0x43434343, 0x44444444,
    0xc4c4c4c4, 0xdededede, 0xe9e9e9e9, 0xcbcbcbcb,
    0x54545454, 0x7b7b7b7b, 0x94949494, 0x32323232,
    0xa6a6a6a6, 0xc2c2c2c2, 0x23232323, 0x3d3d3d3d,
    0xeeeeeeee, 0x4c4c4c4c, 0x95959595, 0x0b0b0b0b,
    0x42424242, 0xfafafafa, 0xc3c3c3c3, 0x4e4e4e4e,
    0x08080808, 0x2e2e2e2e, 0xa1a1a1a1, 0x66666666,
    0x28282828, 0xd9d9d9d9, 0x24242424, 0xb2b2b2b2,
    0x76767676, 0x5b5b5b5b, 0xa2a2a2a2, 0x49494949,
    0x6d6d6d6d, 0x8b8b8b8b, 0xd1d1d1d1, 0x25252525,
    0x72727272, 0xf8f8f8f8, 0xf6f6f6f6, 0x64646464,
    0x86868686, 0x68686868, 0x98989898, 0x16161616,
    0xd4d4d4d4, 0xa4a4a4a4, 0x5c5c5c5c, 0xcccccccc,
    0x5d5d5d5d, 0x65656565, 0xb6b6b6b6, 0x92929292,
    0x6c6c6c6c, 0x70707070, 0x48484848, 0x50505050,
    0xfdfdfdfd, 0xedededed, 0xb9b9b9b9, 0xdadadada,
    0x5e5e5e5e, 0x15151515, 0x46464646, 0x57575757,
    0xa7a7a7a7, 0x8d8d8d8d, 0x9d9d9d9d, 0x84848484,
    0x90909090, 0xd8d8d8d8, 0xabababab, 0x00000000,
    0x8c8c8c8c, 0xbcbcbcbc, 0xd3d3d3d3, 0x0a0a0a0a,
    0xf7f7f7f7, 0xe4e4e4e4, 0x58585858, 0x05050505,
    0xb8b8b8b8, 0xb3b3b3b3, 0x45454545, 0x06060606,
    0xd0d0d0d0, 0x2c2c2c2c, 0x1e1e1e1e, 0x8f8f8f8f,
    0xcacacaca, 0x3f3f3f3f, 0x0f0f0f0f, 0x02020202,
    0xc1c1c1c1, 0xafafafaf, 0xbdbdbdbd, 0x03030303,
    0x01010101, 0x13131313, 0x8a8a8a8a, 0x6b6b6b6b,
    0x3a3a3a3a, 0x91919191, 0x11111111, 0x41414141,
    0x4f4f4f4f, 0x67676767, 0xdcdcdcdc, 0xeaeaeaea,
    0x97979797, 0xf2f2f2f2, 0xcfcfcfcf, 0xcececece,
    0xf0f0f0f0, 0xb4b4b4b4, 0xe6e6e6e6, 0x73737373,
    0x96969696, 0xacacacac, 0x74747474, 0x22222222,
    0xe7e7e7e7, 0xadadadad, 0x35353535, 0x85858585,
    0xe2e2e2e2, 0xf9f9f9f9, 0x37373737, 0xe8e8e8e8,
    0x1c1c1c1c, 0x75757575, 0xdfdfdfdf, 0x6e6e6e6e,
    0x47474747, 0xf1f1f1f1, 0x1a1a1a1a, 0x71717171,
    0x1d1d1d1d, 0x29292929, 0xc5c5c5c5, 0x89898989,
    0x6f6f6f6f, 0xb7b7b7b7, 0x62626262, 0x0e0e0e0e,
    0xaaaaaaaa, 0x18181818, 0xbebebebe, 0x1b1b1b1b,
    0xfcfcfcfc, 0x56565656, 0x3e3e3e3e, 0x4b4b4b4b,
    0xc6c6c6c6, 0xd2d2d2d2, 0x79797979, 0x20202020,
    0x9a9a9a9a, 0xdbdbdbdb, 0xc0c0c0c0, 0xfefefefe,
    0x78787878, 0xcdcdcdcd, 0x5a5a5a5a, 0xf4f4f4f4,
    0x1f1f1f1f, 0xdddddddd, 0xa8a8a8a8, 0x33333333,
    0x88888888, 0x07070707, 0xc7c7c7c7, 0x31313131,
    0xb1b1b1b1, 0x12121212, 0x10101010, 0x59595959,
    0x27272727, 0x80808080, 0xecececec, 0x5f5f5f5f,
    0x60606060, 0x51515151, 0x7f7f7f7f, 0xa9a9a9a9,
    0x19191919, 0xb5b5b5b5, 0x4a4a4a4a, 0x0d0d0d0d,
    0x2d2d2d2d, 0xe5e5e5e5, 0x7a7a7a7a, 0x9f9f9f9f,
    0x93939393, 0xc9c9c9c9, 0x9c9c9c9c, 0xefefefef,
    0xa0a0a0a0, 0xe0e0e0e0, 0x3b3b3b3b, 0x4d4d4d4d,
    0xaeaeaeae, 0x2a2a2a2a, 0xf5f5f5f5, 0xb0b0b0b0,
    0xc8c8c8c8, 0xebebebeb, 0xbbbbbbbb, 0x3c3c3c3c,
    0x83838383, 0x53535353, 0x99999999, 0x61616161,
    0x17171717, 0x2b2b2b2b, 0x04040404, 0x7e7e7e7e,
    0xbabababa, 0x77777777, 0xd6d6d6d6, 0x26262626,
    0xe1e1e1e1, 0x69696969, 0x14141414, 0x63636363,
    0x55555555, 0x21212121, 0x0c0c0c0c, 0x7d7d7d7d,
];

#[no_mangle]
pub extern "C" fn AES_decrypt__part100(
    mut t0: c_uint, mut t1: c_uint, mut t2: c_uint, mut t3: c_uint,
    rk0: c_uint,
    mut out: *mut u8,
) {
    let s0 =
        (Td4[((t0 >> 24) & 0xff) as usize] & 0xff000000) ^
        (Td4[((t3 >> 16) & 0xff) as usize] & 0x00ff0000) ^
        (Td4[((t2 >> 8) & 0xff) as usize] & 0x0000ff00) ^
        (Td4[((t1 ) & 0xff) as usize] & 0x000000ff) ^
        rk0;

    unsafe {
        *out = ((s0) >> 24) as u8;
        out = out.add(1);
        *out = ((s0) >> 16) as u8;
        out = out.add(1);
        *out = ((s0) >> 8) as u8;
        out = out.add(1);
        *out = (s0) as u8;
    }
}
}

// ── `{ … }` · aes.c:1246-1246 · chunk 101 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     { (out)[0] = (u8)((s0) >> 24); (out)[1] = (u8)((s0) >> 16); (out)[2] = (u8)((s0) >> 8); (out)[3] = (u8)(s0); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk101 {
use core::ffi::c_uchar;
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn AES_decrypt__part101(mut out: *mut c_uchar, s0: c_uint) {
    unsafe {
        *out = (s0 >> 24) as c_uchar;
        out = out.offset(1);
        *out = (s0 >> 16) as c_uchar;
        out = out.offset(1);
        *out = (s0 >> 8) as c_uchar;
        out = out.offset(1);
        *out = s0 as c_uchar;
    }
}
}

// ── `{ … }` · aes.c:1253-1253 · chunk 103 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 4)[0] = (u8)((s1) >> 24); (out + 4)[1] = (u8)((s1) >> 16); (out + 4)[2] = (u8)((s1) >> 8); (out + 4)[3] = (u8)(s1); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk103 {
// Automatically generated Rust port of AES_decrypt__part103
#[no_mangle]
pub extern "C" fn AES_decrypt__part103(s1: u32, mut out: *mut u8) {
    unsafe {
        *out = ((s1) >> 24) as u8;
        out = out.offset(1) as *mut u8;
        *out = ((s1) >> 16) as u8;
        out = out.offset(1) as *mut u8;
        *out = ((s1) >> 8) as u8;
        out = out.offset(1) as *mut u8;
        *out = (s1) as u8;
    }
}
}

// ── `; s2 = (Td4[(t2 >> 24) ] & 0xff000000) ^ (Td4[(t1 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t0 >> 8)…` · aes.c:1253-1259 · chunk 104 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//         s2 =
//          (Td4[(t2 >> 24) ] & 0xff000000) ^
//          (Td4[(t1 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t0 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t3 ) & 0xff] & 0x000000ff) ^
//          rk[2];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk104 {
// Port of AES_decrypt__part104 to Rust
use core::ffi::c_uint;
use core::ffi::c_uchar;
use core::ptr;

pub const Td4: [u32; 256] = [
    0x52525252, 0x09090909, 0x6a6a6a6a, 0xd5d5d5d5,
    0x30303030, 0x36363636, 0xa5a5a5a5, 0x38383838,
    0xbfbfbfbf, 0x40404040, 0xa3a3a3a3, 0x9e9e9e9e,
    0x81818181, 0xf3f3f3f3, 0xd7d7d7d7, 0xfbfbfbfb,
    0x7c7c7c7c, 0xe3e3e3e3, 0x39393939, 0x82828282,
    0x9b9b9b9b, 0x2f2f2f2f, 0xffffffff, 0x87878787,
    0x34343434, 0x8e8e8e8e, 0x43434343, 0x44444444,
    0xc4c4c4c4, 0xdededede, 0xe9e9e9e9, 0xcbcbcbcb,
    0x54545454, 0x7b7b7b7b, 0x94949494, 0x32323232,
    0xa6a6a6a6, 0xc2c2c2c2, 0x23232323, 0x3d3d3d3d,
    0xeeeeeeee, 0x4c4c4c4c, 0x95959595, 0x0b0b0b0b,
    0x42424242, 0xfafafafa, 0xc3c3c3c3, 0x4e4e4e4e,
    0x08080808, 0x2e2e2e2e, 0xa1a1a1a1, 0x66666666,
    0x28282828, 0xd9d9d9d9, 0x24242424, 0xb2b2b2b2,
    0x76767676, 0x5b5b5b5b, 0xa2a2a2a2, 0x49494949,
    0x6d6d6d6d, 0x8b8b8b8b, 0xd1d1d1d1, 0x25252525,
    0x72727272, 0xf8f8f8f8, 0xf6f6f6f6, 0x64646464,
    0x86868686, 0x68686868, 0x98989898, 0x16161616,
    0xd4d4d4d4, 0xa4a4a4a4, 0x5c5c5c5c, 0xcccccccc,
    0x5d5d5d5d, 0x65656565, 0xb6b6b6b6, 0x92929292,
    0x6c6c6c6c, 0x70707070, 0x48484848, 0x50505050,
    0xfdfdfdfd, 0xedededed, 0xb9b9b9b9, 0xdadadada,
    0x5e5e5e5e, 0x15151515, 0x46464646, 0x57575757,
    0xa7a7a7a7, 0x8d8d8d8d, 0x9d9d9d9d, 0x84848484,
    0x90909090, 0xd8d8d8d8, 0xabababab, 0x00000000,
    0x8c8c8c8c, 0xbcbcbcbc, 0xd3d3d3d3, 0x0a0a0a0a,
    0xf7f7f7f7, 0xe4e4e4e4, 0x58585858, 0x05050505,
    0xb8b8b8b8, 0xb3b3b3b3, 0x45454545, 0x06060606,
    0xd0d0d0d0, 0x2c2c2c2c, 0x1e1e1e1e, 0x8f8f8f8f,
    0xcacacaca, 0x3f3f3f3f, 0x0f0f0f0f, 0x02020202,
    0xc1c1c1c1, 0xafafafaf, 0xbdbdbdbd, 0x03030303,
    0x01010101, 0x13131313, 0x8a8a8a8a, 0x6b6b6b6b,
    0x3a3a3a3a, 0x91919191, 0x11111111, 0x41414141,
    0x4f4f4f4f, 0x67676767, 0xdcdcdcdc, 0xeaeaeaea,
    0x97979797, 0xf2f2f2f2, 0xcfcfcfcf, 0xcececece,
    0xf0f0f0f0, 0xb4b4b4b4, 0xe6e6e6e6, 0x73737373,
    0x96969696, 0xacacacac, 0x74747474, 0x22222222,
    0xe7e7e7e7, 0xadadadad, 0x35353535, 0x85858585,
    0xe2e2e2e2, 0xf9f9f9f9, 0x37373737, 0xe8e8e8e8,
    0x1c1c1c1c, 0x75757575, 0xdfdfdfdf, 0x6e6e6e6e,
    0x47474747, 0xf1f1f1f1, 0x1a1a1a1a, 0x71717171,
    0x1d1d1d1d, 0x29292929, 0xc5c5c5c5, 0x89898989,
    0x6f6f6f6f, 0xb7b7b7b7, 0x62626262, 0x0e0e0e0e,
    0xaaaaaaaa, 0x18181818, 0xbebebebe, 0x1b1b1b1b,
    0xfcfcfcfc, 0x56565656, 0x3e3e3e3e, 0x4b4b4b4b,
    0xc6c6c6c6, 0xd2d2d2d2, 0x79797979, 0x20202020,
    0x9a9a9a9a, 0xdbdbdbdb, 0xc0c0c0c0, 0xfefefefe,
    0x78787878, 0xcdcdcdcd, 0x5a5a5a5a, 0xf4f4f4f4,
    0x1f1f1f1f, 0xdddddddd, 0xa8a8a8a8, 0x33333333,
    0x88888888, 0x07070707, 0xc7c7c7c7, 0x31313131,
    0xb1b1b1b1, 0x12121212, 0x10101010, 0x59595959,
    0x27272727, 0x80808080, 0xecececec, 0x5f5f5f5f,
    0x60606060, 0x51515151, 0x7f7f7f7f, 0xa9a9a9a9,
    0x19191919, 0xb5b5b5b5, 0x4a4a4a4a, 0x0d0d0d0d,
    0x2d2d2d2d, 0xe5e5e5e5, 0x7a7a7a7a, 0x9f9f9f9f,
    0x93939393, 0xc9c9c9c9, 0x9c9c9c9c, 0xefefefef,
    0xa0a0a0a0, 0xe0e0e0e0, 0x3b3b3b3b, 0x4d4d4d4d,
    0xaeaeaeae, 0x2a2a2a2a, 0xf5f5f5f5, 0xb0b0b0b0,
    0xc8c8c8c8, 0xebebebeb, 0xbbbbbbbb, 0x3c3c3c3c,
    0x83838383, 0x53535353, 0x99999999, 0x61616161,
    0x17171717, 0x2b2b2b2b, 0x04040404, 0x7e7e7e7e,
    0xbabababa, 0x77777777, 0xd6d6d6d6, 0x26262626,
    0xe1e1e1e1, 0x69696969, 0x14141414, 0x63636363,
    0x55555555, 0x21212121, 0x0c0c0c0c, 0x7d7d7d7d,
];

#[no_mangle]
pub extern "C" fn AES_decrypt__part104(
    t0: c_uint, t1: c_uint, t2: c_uint, t3: c_uint,
    rk2: c_uint,
    out: *mut c_uchar,
) {
    let s2 = 
        (Td4[(t2 >> 24) as usize] & 0xff000000u32) ^
        (Td4[((t1 >> 16) & 0xff) as usize] & 0x00ff0000u32) ^
        (Td4[((t0 >> 8) & 0xff) as usize] & 0x0000ff00u32) ^
        (Td4[((t3 ) & 0xff) as usize] & 0x000000ffu32) ^
        rk2;
    
    unsafe {
        ptr::write(out, ((s2) >> 24) as c_uchar);
        ptr::write(out.add(1), (((s2) >> 16) & 0xff) as c_uchar);
        ptr::write(out.add(2), (((s2) >> 8) & 0xff) as c_uchar);
        ptr::write(out.add(3), (s2 & 0xff) as c_uchar);
    }
}
}

// ── `{ … }` · aes.c:1260-1260 · chunk 105 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 8)[0] = (u8)((s2) >> 24); (out + 8)[1] = (u8)((s2) >> 16); (out + 8)[2] = (u8)((s2) >> 8); (out + 8)[3] = (u8)(s2); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk105 {
#[no_mangle]
pub extern "C" fn AES_decrypt__part105(mut out: *mut u8, s2: u32) {
    unsafe {
        *out = ((s2 >> 24) & 0xff) as u8;
        out = out.add(1);
        *out = ((s2 >> 16) & 0xff) as u8;
        out = out.add(1);
        *out = ((s2 >> 8) & 0xff) as u8;
        out = out.add(1);
        *out = (s2 & 0xff) as u8;
    }
}
}

// ── `; s3 = (Td4[(t3 >> 24) ] & 0xff000000) ^ (Td4[(t2 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t1 >> 8)…` · aes.c:1260-1266 · chunk 106 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     ;
//         s3 =
//          (Td4[(t3 >> 24) ] & 0xff000000) ^
//          (Td4[(t2 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t1 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t0 ) & 0xff] & 0x000000ff) ^
//          rk[3];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk106 {
// Rust implementation of AES_decrypt final output block (lines 1627-1654 of original C)
// This replaces the TARGET code in AES_decrypt function

// Td4 S-box table for AES decryption, matching the C static const u32 Td4[256]
// Using hex literals without suffix; values are within u32 range matching C unsigned int
const Td4: [u32; 256] = [
    0x52525252, 0x09090909, 0x6a6a6a6a, 0xd5d5d5d5,
    0x30303030, 0x36363636, 0xa5a5a5a5, 0x38383838,
    0xbfbfbfbf, 0x40404040, 0xa3a3a3a3, 0x9e9e9e9e,
    0x81818181, 0xf3f3f3f3, 0xd7d7d7d7, 0xfbfbfbfb,
    0x7c7c7c7c, 0xe3e3e3e3, 0x39393939, 0x82828282,
    0x9b9b9b9b, 0x2f2f2f2f, 0xffffffff, 0x87878787,
    0x34343434, 0x8e8e8e8e, 0x43434343, 0x44444444,
    0xc4c4c4c4, 0xdededede, 0xe9e9e9e9, 0xcbcbcbcb,
    0x54545454, 0x7b7b7b7b, 0x94949494, 0x32323232,
    0xa6a6a6a6, 0xc2c2c2c2, 0x23232323, 0x3d3d3d3d,
    0xeeeeeeee, 0x4c4c4c4c, 0x95959595, 0x0b0b0b0b,
    0x42424242, 0xfafafafa, 0xc3c3c3c3, 0x4e4e4e4e,
    0x08080808, 0x2e2e2e2e, 0xa1a1a1a1, 0x66666666,
    0x28282828, 0xd9d9d9d9, 0x24242424, 0xb2b2b2b2,
    0x76767676, 0x5b5b5b5b, 0xa2a2a2a2, 0x49494949,
    0x6d6d6d6d, 0x8b8b8b8b, 0xd1d1d1d1, 0x25252525,
    0x72727272, 0xf8f8f8f8, 0xf6f6f6f6, 0x64646464,
    0x86868686, 0x68686868, 0x98989898, 0x16161616,
    0xd4d4d4d4, 0xa4a4a4a4, 0x5c5c5c5c, 0xcccccccc,
    0x5d5d5d5d, 0x65656565, 0xb6b6b6b6, 0x92929292,
    0x6c6c6c6c, 0x70707070, 0x48484848, 0x50505050,
    0xfdfdfdfd, 0xededede, 0xb9b9b9b9, 0xdadadada,
    0x5e5e5e5e, 0x15151515, 0x46464646, 0x57575757,
    0xa7a7a7a7, 0x8d8d8d8d, 0x9d9d9d9d, 0x84848484,
    0x90909090, 0xd8d8d8d8, 0xabababab, 0x00000000,
    0x8c8c8c8c, 0xbcbcbcbc, 0xd3d3d3d3, 0x0a0a0a0a,
    0xf7f7f7f7, 0xe4e4e4e4, 0x58585858, 0x05050505,
    0xb8b8b8b8, 0xb3b3b3b3, 0x45454545, 0x06060606,
    0xd0d0d0d0, 0x2c2c2c2c, 0x1e1e1e1e, 0x8f8f8f8f,
    0xcacacaca, 0x3f3f3f3f, 0x0f0f0f0f, 0x02020202,
    0xc1c1c1c1, 0xafafafaf, 0xbdbdbdbd, 0x03030303,
    0x01010101, 0x13131313, 0x8a8a8a8a, 0x6b6b6b6b,
    0x3a3a3a3a, 0x91919191, 0x11111111, 0x41414141,
    0x4f4f4f4f, 0x67676767, 0xdcdcdcdc, 0xeaeaeaea,
    0x97979797, 0xf2f2f2f2, 0xcfcfcfcf, 0xcececece,
    0xf0f0f0f0, 0xb4b4b4b4, 0xe6e6e6e6, 0x73737373,
    0x96969696, 0xacacacac, 0x74747474, 0x22222222,
    0xe7e7e7e7, 0xadadadad, 0x35353535, 0x85858585,
    0xe2e2e2e2, 0xf9f9f9f9, 0x37373737, 0xe8e8e8e8,
    0x1c1c1c1c, 0x75757575, 0xdfdfdfdf, 0x6e6e6e6e,
    0x47474747, 0xf1f1f1f1, 0x1a1a1a1a, 0x71717171,
    0x1d1d1d1d, 0x29292929, 0xc5c5c5c5, 0x89898989,
    0x6f6f6f6f, 0xb7b7b7b7, 0x62626262, 0x0e0e0e0e,
    0xaaaaaaaa, 0x18181818, 0xbebebebe, 0x1b1b1b1b,
    0xfcfcfcfc, 0x56565656, 0x3e3e3e3e, 0x4b4b4b4b,
    0xc6c6c6c6, 0xd2d2d2d2, 0x79797979, 0x20202020,
    0x9a9a9a9a, 0xdbdbdbdb, 0xc0c0c0c0, 0xfefefefe,
    0x78787878, 0xcdcdcdcd, 0x5a5a5a5a, 0xf4f4f4f4,
    0x1f1f1f1f, 0xdddddddd, 0xa8a8a8a8, 0x33333333,
    0x88888888, 0x07070707, 0xc7c7c7c7, 0x31313131,
    0xb1b1b1b1, 0x12121212, 0x10101010, 0x59595959,
    0x27272727, 0x80808080, 0xecececec, 0x5f5f5f5f,
    0x60606060, 0x51515151, 0x7f7f7f7f, 0xa9a9a9a9,
    0x19191919, 0xb5b5b5b5, 0x4a4a4a4a, 0x0d0d0d0d,
    0x2d2d2d2d, 0xe5e5e5e5, 0x7a7a7a7a, 0x9f9f9f9f,
    0x93939393, 0xc9c9c9c9, 0x9c9c9c9c, 0xefefefef,
    0xa0a0a0a0, 0xe0e0e0e0, 0x3b3b3b3b, 0x4d4d4d4d,
    0xaeaeaeae, 0x2a2a2a2a, 0xf5f5f5f5, 0xb0b0b0b0,
    0xc8c8c8c8, 0xebebebeb, 0xbbbbbbbb, 0x3c3c3c3c,
    0x83838383, 0x53535353, 0x99999999, 0x61616161,
    0x17171717, 0x2b2b2b2b, 0x04040404, 0x7e7e7e7e,
    0xbabababa, 0x77777777, 0xd6d6d6d6, 0x26262626,
    0xe1e1e1e1, 0x69696969, 0x14141414, 0x63636363,
    0x55555555, 0x21212121, 0x0c0c0c0c, 0x7d7d7d7d,
];

#[no_mangle]
pub extern "C" fn AES_decrypt__part106(
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    rk: *const u32,
    out: *mut u8,
) {
    unsafe {
        // Compute s0 using Td4 and rk[0]
        let s0 = Td4[(t0 >> 24) as usize] & 0xff000000
            ^ Td4[(t3 >> 16) as usize & 0xff] & 0x00ff0000
            ^ Td4[(t2 >> 8) as usize & 0xff] & 0x0000ff00
            ^ Td4[(t1) as usize & 0xff] & 0x000000ff
            ^ *rk.add(0);
        *out.add(0) = (s0 >> 24) as u8;
        *out.add(1) = (s0 >> 16) as u8;
        *out.add(2) = (s0 >> 8) as u8;
        *out.add(3) = s0 as u8;

        // Compute s1 using Td4 and rk[1]
        let s1 = Td4[(t1 >> 24) as usize] & 0xff000000
            ^ Td4[(t0 >> 16) as usize & 0xff] & 0x00ff0000
            ^ Td4[(t3 >> 8) as usize & 0xff] & 0x0000ff00
            ^ Td4[(t2) as usize & 0xff] & 0x000000ff
            ^ *rk.add(1);
        *out.add(4) = (s1 >> 24) as u8;
        *out.add(5) = (s1 >> 16) as u8;
        *out.add(6) = (s1 >> 8) as u8;
        *out.add(7) = s1 as u8;

        // Compute s2 using Td4 and rk[2]
        let s2 = Td4[(t2 >> 24) as usize] & 0xff000000
            ^ Td4[(t1 >> 16) as usize & 0xff] & 0x00ff0000
            ^ Td4[(t0 >> 8) as usize & 0xff] & 0x0000ff00
            ^ Td4[(t3) as usize & 0xff] & 0x000000ff
            ^ *rk.add(2);
        *out.add(8) = (s2 >> 24) as u8;
        *out.add(9) = (s2 >> 16) as u8;
        *out.add(10) = (s2 >> 8) as u8;
        *out.add(11) = s2 as u8;

        // Compute s3 using Td4 and rk[3]
        let s3 = Td4[(t3 >> 24) as usize] & 0xff000000
            ^ Td4[(t2 >> 16) as usize & 0xff] & 0x00ff0000
            ^ Td4[(t1 >> 8) as usize & 0xff] & 0x0000ff00
            ^ Td4[(t0) as usize & 0xff] & 0x000000ff
            ^ *rk.add(3);
        *out.add(12) = (s3 >> 24) as u8;
        *out.add(13) = (s3 >> 16) as u8;
        *out.add(14) = (s3 >> 8) as u8;
        *out.add(15) = s3 as u8;
    }
}
}

// ── `{ … }` · aes.c:1267-1267 · chunk 107 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     { (out + 12)[0] = (u8)((s3) >> 24); (out + 12)[1] = (u8)((s3) >> 16); (out + 12)[2] = (u8)((s3) >> 8); (out + 12)[3] = (u8)(s3); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk107 {
use core::ffi::c_uint;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn AES_decrypt__part107(s3: c_uint, mut out: *mut c_uchar) {
    unsafe {
        *out = (s3 >> 24) as c_uchar;
        out = out.offset(1);
        *out = (s3 >> 16) as c_uchar;
        out = out.offset(1);
        *out = (s3 >> 8) as c_uchar;
        out = out.offset(1);
        *out = s3 as c_uchar;
    }
}
}

// ── `;` · aes.c:1267-1267 · chunk 108 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk108 {
#[no_mangle]
pub extern "C" fn AES_decrypt__part108() {
}
}

// ── `unsigned long n; unsigned long len = length; unsigned char tmp[16]; ((void) (0)) ;` · aes.c:1277-1281 · chunk 110 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     unsigned long n;
//      unsigned long len = length;
//      unsigned char tmp[16];
//     
//      
//     ((void) (0))
//                                     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk110 {
use core::ffi::c_ulong;
use core::ffi::c_uchar;

#[repr(C)]
struct AES_cbc_encrypt__part110_result {
    n: c_ulong,
    len: c_ulong,
    tmp: [c_uchar; 16],
}

#[no_mangle]
pub extern "C" fn AES_cbc_encrypt__part110(length: c_ulong) -> AES_cbc_encrypt__part110_result {
    AES_cbc_encrypt__part110_result {
        n: 0,
        len: length,
        tmp: [0; 16],
    }
}
}

