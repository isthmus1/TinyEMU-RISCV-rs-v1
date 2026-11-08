//! From `aes.c`.

// ── `AES_set_encrypt_key` · aes.c:738-834 · chunk 66 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int AES_set_encrypt_key(const unsigned char *userKey, const int bits,
//        AES_KEY *key) {
//     
//      u32 *rk;
//         int i = 0;
//      u32 temp;
//     
//      if (!userKey || !key)
//       return -1;
//      if (bits != 128 && bits != 192 && bits != 256)
//       return -2;
//     
//      rk = key->rd_key;
//     
//      if (bits==128)
//       key->rounds = 10;
//      else if (bits==192)
//       key->rounds = 12;
//      else
//       key->rounds = 14;
//     
//      rk[0] = (((u32)(userKey)[0] << 24) ^ ((u32)(userKey)[1] << 16) ^ ((u32)(userKey)[2] << 8) ^ ((u32)(userKey)[3]));
//      rk[1] = (((u32)(userKey + 4)[0] << 24) ^ ((u32)(userKey + 4)[1] << 16) ^ ((u32)(userKey + 4)[2] << 8) ^ ((u32)(userKey + 4)[3]));
//      rk[2] = (((u32)(userKey + 8)[0] << 24) ^ ((u32)(userKey + 8)[1] << 16) ^ ((u32)(userKey + 8)[2] << 8) ^ ((u32)(userKey + 8)[3]));
//      rk[3] = (((u32)(userKey + 12)[0] << 24) ^ ((u32)(userKey + 12)[1] << 16) ^ ((u32)(userKey + 12)[2] << 8) ^ ((u32)(userKey + 12)[3]));
//      if (bits == 128) {
//       while (1) {
//        temp = rk[3];
//        rk[4] = rk[0] ^
//         (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
//         (Te4[(temp >> 8) & 0xff] & 0x00ff0000) ^
//         (Te4[(temp ) & 0xff] & 0x0000ff00) ^
//         (Te4[(temp >> 24) ] & 0x000000ff) ^
//         rcon[i];
//        rk[5] = rk[1] ^ rk[4];
//        rk[6] = rk[2] ^ rk[5];
//        rk[7] = rk[3] ^ rk[6];
//        if (++i == 10) {
//         return 0;
//        }
//        rk += 4;
//       }
//      }
//      rk[4] = (((u32)(userKey + 16)[0] << 24) ^ ((u32)(userKey + 16)[1] << 16) ^ ((u32)(userKey + 16)[2] << 8) ^ ((u32)(userKey + 16)[3]));
//      rk[5] = (((u32)(userKey + 20)[0] << 24) ^ ((u32)(userKey + 20)[1] << 16) ^ ((u32)(userKey + 20)[2] << 8) ^ ((u32)(userKey + 20)[3]));
//      if (bits == 192) {
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
//      rk[6] = (((u32)(userKey + 24)[0] << 24) ^ ((u32)(userKey + 24)[1] << 16) ^ ((u32)(userKey + 24)[2] << 8) ^ ((u32)(userKey + 24)[3]));
//      rk[7] = (((u32)(userKey + 28)[0] << 24) ^ ((u32)(userKey + 28)[1] << 16) ^ ((u32)(userKey + 28)[2] << 8) ^ ((u32)(userKey + 28)[3]));
//      if (bits == 256) {
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
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk66 {
// Rust rewrite of AES_set_encrypt_key from aes.c
#![allow(non_snake_case)]

use core::ffi::{c_int, c_uchar};

#[repr(C)]
pub struct AesKeySt {
    pub rd_key: [u32; 4 * (14 + 1)],
    pub rounds: c_int,
}

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

static RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000,
    0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1B000000, 0x36000000,
];

#[inline]
fn te4(idx: u32) -> u32 {
    // idx is always masked to 0..=255 before the call, so this cannot fail.
    let i = (idx & 0xff) as usize;
    match TE4.get(i) {
        Some(v) => *v,
        None => 0,
    }
}

#[inline]
fn rcon_at(i: c_int) -> u32 {
    let idx = i as usize;
    match RCON.get(idx) {
        Some(v) => *v,
        None => 0,
    }
}

#[inline]
unsafe fn getu32(p: *const c_uchar) -> u32 {
    ((*p as u32) << 24)
        ^ ((*p.add(1) as u32) << 16)
        ^ ((*p.add(2) as u32) << 8)
        ^ (*p.add(3) as u32)
}

#[no_mangle]
pub unsafe extern "C" fn AES_set_encrypt_key(
    userKey: *const c_uchar,
    bits: c_int,
    key: *mut AesKeySt,
) -> c_int {
    if userKey.is_null() || key.is_null() {
        return -1;
    }
    if bits != 128 && bits != 192 && bits != 256 {
        return -2;
    }

    let rk_base: *mut u32 = core::ptr::addr_of_mut!((*key).rd_key) as *mut u32;

    if bits == 128 {
        (*key).rounds = 10;
    } else if bits == 192 {
        (*key).rounds = 12;
    } else {
        (*key).rounds = 14;
    }

    *rk_base.add(0) = getu32(userKey);
    *rk_base.add(1) = getu32(userKey.add(4));
    *rk_base.add(2) = getu32(userKey.add(8));
    *rk_base.add(3) = getu32(userKey.add(12));

    let mut i: c_int = 0;
    let mut rk = rk_base;

    if bits == 128 {
        loop {
            let temp = *rk.add(3);
            *rk.add(4) = *rk.add(0)
                ^ (te4(temp >> 16) & 0xff00_0000u32)
                ^ (te4(temp >> 8) & 0x00ff_0000u32)
                ^ (te4(temp) & 0x0000_ff00u32)
                ^ (te4(temp >> 24) & 0x0000_00ffu32)
                ^ rcon_at(i);
            *rk.add(5) = *rk.add(1) ^ *rk.add(4);
            *rk.add(6) = *rk.add(2) ^ *rk.add(5);
            *rk.add(7) = *rk.add(3) ^ *rk.add(6);
            i += 1;
            if i == 10 {
                return 0;
            }
            rk = rk.add(4);
        }
    }

    *rk.add(4) = getu32(userKey.add(16));
    *rk.add(5) = getu32(userKey.add(20));

    if bits == 192 {
        loop {
            let temp = *rk.add(5);
            *rk.add(6) = *rk.add(0)
                ^ (te4(temp >> 16) & 0xff00_0000u32)
                ^ (te4(temp >> 8) & 0x00ff_0000u32)
                ^ (te4(temp) & 0x0000_ff00u32)
                ^ (te4(temp >> 24) & 0x0000_00ffu32)
                ^ rcon_at(i);
            *rk.add(7) = *rk.add(1) ^ *rk.add(6);
            *rk.add(8) = *rk.add(2) ^ *rk.add(7);
            *rk.add(9) = *rk.add(3) ^ *rk.add(8);
            i += 1;
            if i == 8 {
                return 0;
            }
            *rk.add(10) = *rk.add(4) ^ *rk.add(9);
            *rk.add(11) = *rk.add(5) ^ *rk.add(10);
            rk = rk.add(6);
        }
    }

    *rk.add(6) = getu32(userKey.add(24));
    *rk.add(7) = getu32(userKey.add(28));

    if bits == 256 {
        loop {
            let mut temp = *rk.add(7);
            *rk.add(8) = *rk.add(0)
                ^ (te4(temp >> 16) & 0xff00_0000u32)
                ^ (te4(temp >> 8) & 0x00ff_0000u32)
                ^ (te4(temp) & 0x0000_ff00u32)
                ^ (te4(temp >> 24) & 0x0000_00ffu32)
                ^ rcon_at(i);
            *rk.add(9) = *rk.add(1) ^ *rk.add(8);
            *rk.add(10) = *rk.add(2) ^ *rk.add(9);
            *rk.add(11) = *rk.add(3) ^ *rk.add(10);
            i += 1;
            if i == 7 {
                return 0;
            }
            temp = *rk.add(11);
            *rk.add(12) = *rk.add(4)
                ^ (te4(temp >> 24) & 0xff00_0000u32)
                ^ (te4(temp >> 16) & 0x00ff_0000u32)
                ^ (te4(temp >> 8) & 0x0000_ff00u32)
                ^ (te4(temp) & 0x0000_00ffu32);
            *rk.add(13) = *rk.add(5) ^ *rk.add(12);
            *rk.add(14) = *rk.add(6) ^ *rk.add(13);
            *rk.add(15) = *rk.add(7) ^ *rk.add(14);

            rk = rk.add(8);
        }
    }

    0
}
}

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

// ── `if (bits == 128)` · aes.c:763-780 · chunk 72 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bits == 128) {
//       while (1) {
//        temp = rk[3];
//        rk[4] = rk[0] ^
//         (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
//         (Te4[(temp >> 8) & 0xff] & 0x00ff0000) ^
//         (Te4[(temp ) & 0xff] & 0x0000ff00) ^
//         (Te4[(temp >> 24) ] & 0x000000ff) ^
//         rcon[i];
//        rk[5] = rk[1] ^ rk[4];
//        rk[6] = rk[2] ^ rk[5];
//        rk[7] = rk[3] ^ rk[6];
//        if (++i == 10) {
//         return 0;
//        }
//        rk += 4;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk72 {
// Port of the bits==128 key-expansion loop of AES_set_encrypt_key.
use core::ffi::c_int;

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

static RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000,
    0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1B000000, 0x36000000,
];

#[no_mangle]
pub unsafe extern "C" fn AES_set_encrypt_key__part72(
    rk: *mut u32,
    i: *mut c_int,
    temp: *mut u32,
) -> c_int {
    let mut rk: *mut u32 = rk;
    let mut idx: c_int = *i;
    loop {
        let t: u32 = *rk.add(3);
        *temp = t;
        let v4: u32 = *rk
            ^ (TE4[((t >> 16) & 0xff) as usize] & 0xff000000u32)
            ^ (TE4[((t >> 8) & 0xff) as usize] & 0x00ff0000u32)
            ^ (TE4[(t & 0xff) as usize] & 0x0000ff00u32)
            ^ (TE4[(t >> 24) as usize] & 0x000000ffu32)
            ^ *RCON.as_ptr().add(idx as usize);
        *rk.add(4) = v4;
        let v5: u32 = *rk.add(1) ^ v4;
        *rk.add(5) = v5;
        let v6: u32 = *rk.add(2) ^ v5;
        *rk.add(6) = v6;
        let v7: u32 = *rk.add(3) ^ v6;
        *rk.add(7) = v7;
        idx += 1;
        if idx == 10 {
            *i = idx;
            return 0;
        }
        rk = rk.add(4);
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

// ── `AES_set_decrypt_key` · aes.c:839-885 · chunk 78 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int AES_set_decrypt_key(const unsigned char *userKey, const int bits,
//         AES_KEY *key) {
//     
//             u32 *rk;
//      int i, j, status;
//      u32 temp;
//     
//     
//      status = AES_set_encrypt_key(userKey, bits, key);
//      if (status < 0)
//       return status;
//     
//      rk = key->rd_key;
//     
//     
//      for (i = 0, j = 4*(key->rounds); i < j; i += 4, j -= 4) {
//       temp = rk[i ]; rk[i ] = rk[j ]; rk[j ] = temp;
//       temp = rk[i + 1]; rk[i + 1] = rk[j + 1]; rk[j + 1] = temp;
//       temp = rk[i + 2]; rk[i + 2] = rk[j + 2]; rk[j + 2] = temp;
//       temp = rk[i + 3]; rk[i + 3] = rk[j + 3]; rk[j + 3] = temp;
//      }
//     
//      for (i = 1; i < (key->rounds); i++) {
//       rk += 4;
//       rk[0] =
//        Td0[Te4[(rk[0] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[0] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[0] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[0] ) & 0xff] & 0xff];
//       rk[1] =
//        Td0[Te4[(rk[1] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[1] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[1] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[1] ) & 0xff] & 0xff];
//       rk[2] =
//        Td0[Te4[(rk[2] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[2] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[2] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[2] ) & 0xff] & 0xff];
//       rk[3] =
//        Td0[Te4[(rk[3] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[3] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[3] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[3] ) & 0xff] & 0xff];
//      }
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk78 {
// Port of AES_set_decrypt_key() from aes.c
use core::ffi::c_int;

#[repr(C)]
pub struct AesKey {
    pub rd_key: [u32; 4 * (14 + 1)],
    pub rounds: c_int,
}

extern "C" {
    fn AES_set_encrypt_key(userKey: *const u8, bits: c_int, key: *mut AesKey) -> c_int;
}

// Td0 is the base decryption table; Td1..Td3 are its byte rotations
// (identical to the static const tables in aes.c).
static TD0: [u32; 256] = [
    0x51f4a750, 0x7e416553, 0x1a17a4c3, 0x3a275e96,
    0x3bab6bcb, 0x1f9d45f1, 0xacfa58ab, 0x4be30393,
    0x2030fa55, 0xad766df6, 0x88cc7691, 0xf5024c25,
    0x4fe5d7fc, 0xc52acbd7, 0x26354480, 0xb562a38f,
    0xdeb15a49, 0x25ba1b67, 0x45ea0e98, 0x5dfec0e1,
    0xc32f7502, 0x814cf012, 0x8d4697a3, 0x6bd3f9c6,
    0x038f5fe7, 0x15929c95, 0xbf6d7aeb, 0x955259da,
    0xd4be832d, 0x587421d3, 0x49e06929, 0x8ec9c844,
    0x75c2896a, 0xf48e7978, 0x99583e6b, 0x27b971dd,
    0xbee14fb6, 0xf088ad17, 0xc920ac66, 0x7dce3ab4,
    0x63df4a18, 0xe51a3182, 0x97513360, 0x62537f45,
    0xb16477e0, 0xbb6bae84, 0xfe81a01c, 0xf9082b94,
    0x70486858, 0x8f45fd19, 0x94de6c87, 0x527bf8b7,
    0xab73d323, 0x724b02e2, 0xe31f8f57, 0x6655ab2a,
    0xb2eb2807, 0x2fb5c203, 0x86c57b9a, 0xd33708a5,
    0x302887f2, 0x23bfa5b2, 0x02036aba, 0xed16825c,
    0x8acf1c2b, 0xa779b492, 0xf307f2f0, 0x4e69e2a1,
    0x65daf4cd, 0x0605bed5, 0xd134621f, 0xc4a6fe8a,
    0x342e539d, 0xa2f355a0, 0x058ae132, 0xa4f6eb75,
    0x0b83ec39, 0x4060efaa, 0x5e719f06, 0xbd6e1051,
    0x3e218af9, 0x96dd063d, 0xdd3e05ae, 0x4de6bd46,
    0x91548db5, 0x71c45d05, 0x0406d46f, 0x605015ff,
    0x1998fb24, 0xd6bde997, 0x894043cc, 0x67d99e77,
    0xb0e842bd, 0x07898b88, 0xe7195b38, 0x79c8eedb,
    0xa17c0a47, 0x7c420fe9, 0xf8841ec9, 0x00000000,
    0x09808683, 0x322bed48, 0x1e1170ac, 0x6c5a724e,
    0xfd0efffb, 0x0f853856, 0x3daed51e, 0x362d3927,
    0x0a0fd964, 0x685ca621, 0x9b5b54d1, 0x24362e3a,
    0x0c0a67b1, 0x9357e70f, 0xb4ee96d2, 0x1b9b919e,
    0x80c0c54f, 0x61dc20a2, 0x5a774b69, 0x1c121a16,
    0xe293ba0a, 0xc0a02ae5, 0x3c22e043, 0x121b171d,
    0x0e090d0b, 0xf28bc7ad, 0x2db6a8b9, 0x141ea9c8,
    0x57f11985, 0xaf75074c, 0xee99ddbb, 0xa37f60fd,
    0xf701269f, 0x5c72f5bc, 0x44663bc5, 0x5bfb7e34,
    0x8b432976, 0xcb23c6dc, 0xb6edfc68, 0xb8e4f163,
    0xd731dcca, 0x42638510, 0x13972240, 0x84c61120,
    0x854a247d, 0xd2bb3df8, 0xaef93211, 0xc729a16d,
    0x1d9e2f4b, 0xdcb230f3, 0x0d8652ec, 0x77c1e3d0,
    0x2bb3166c, 0xa970b999, 0x119448fa, 0x47e96422,
    0xa8fc8cc4, 0xa0f03f1a, 0x567d2cd8, 0x223390ef,
    0x87494ec7, 0xd938d1c1, 0x8ccaa2fe, 0x98d40b36,
    0xa6f581cf, 0xa57ade28, 0xdab78e26, 0x3fadbfa4,
    0x2c3a9de4, 0x5078920d, 0x6a5fcc9b, 0x547e4662,
    0xf68d13c2, 0x90d8b8e8, 0x2e39f75e, 0x82c3aff5,
    0x9f5d80be, 0x69d0937c, 0x6fd52da9, 0xcf2512b3,
    0xc8ac993b, 0x10187da7, 0xe89c636e, 0xdb3bbb7b,
    0xcd267809, 0x6e5918f4, 0xec9ab701, 0x834f9aa8,
    0xe6956e65, 0xaaffe67e, 0x21bccf08, 0xef15e8e6,
    0xbae79bd9, 0x4a6f36ce, 0xea9f09d4, 0x29b07cd6,
    0x31a4b2af, 0x2a3f2331, 0xc6a59430, 0x35a266c0,
    0x744ebc37, 0xfc82caa6, 0xe090d0b0, 0x33a7d815,
    0xf104984a, 0x41ecdaf7, 0x7fcd500e, 0x1791f62f,
    0x764dd68d, 0x43efb04d, 0xccaa4d54, 0xe49604df,
    0x9ed1b5e3, 0x4c6a881b, 0xc12c1fb8, 0x4665517f,
    0x9d5eea04, 0x018c355d, 0xfa877473, 0xfb0b412e,
    0xb3671d5a, 0x92dbd252, 0xe9105633, 0x6dd64713,
    0x9ad7618c, 0x37a10c7a, 0x59f8148e, 0xeb133c89,
    0xcea927ee, 0xb761c935, 0xe11ce5ed, 0x7a47b13c,
    0x9cd2df59, 0x55f2733f, 0x1814ce79, 0x73c737bf,
    0x53f7cdea, 0x5ffdaa5b, 0xdf3d6f14, 0x7844db86,
    0xcaaff381, 0xb968c43e, 0x3824342c, 0xc2a3405f,
    0x161dc372, 0xbce2250c, 0x283c498b, 0xff0d9541,
    0x39a80171, 0x080cb3de, 0xd8b4e49c, 0x6456c190,
    0x7bcb8461, 0xd532b670, 0x486c5c74, 0xd0b85742,
];

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

const fn rot_tab(base: &[u32; 256], n: u32) -> [u32; 256] {
    let mut out = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        out[i] = base[i].rotate_right(n);
        i += 1;
    }
    out
}

static TD1: [u32; 256] = rot_tab(&TD0, 8);
static TD2: [u32; 256] = rot_tab(&TD0, 16);
static TD3: [u32; 256] = rot_tab(&TD0, 24);

#[inline]
fn te4(i: u32) -> u32 {
    // masked so the index is always in bounds (never panics)
    TE4[(i & 0xff) as usize]
}

#[inline]
fn td(t: &[u32; 256], i: u32) -> u32 {
    t[(i & 0xff) as usize]
}

#[no_mangle]
pub unsafe extern "C" fn AES_set_decrypt_key(
    userKey: *const u8,
    bits: c_int,
    key: *mut AesKey,
) -> c_int {
    let mut status: c_int = AES_set_encrypt_key(userKey, bits, key);
    if status < 0 {
        return status;
    }

    let keyp: *mut AesKey = key;
    let rk: *mut u32 = (*keyp).rd_key.as_mut_ptr();
    let rounds: c_int = (*keyp).rounds;

    // invert the order of the round keys
    let mut i: c_int = 0;
    let mut j: c_int = 4 * rounds;
    while i < j {
        let ii = i as usize;
        let jj = j as usize;
        let mut temp: u32;

        temp = *rk.add(ii); *rk.add(ii) = *rk.add(jj); *rk.add(jj) = temp;
        temp = *rk.add(ii + 1); *rk.add(ii + 1) = *rk.add(jj + 1); *rk.add(jj + 1) = temp;
        temp = *rk.add(ii + 2); *rk.add(ii + 2) = *rk.add(jj + 2); *rk.add(jj + 2) = temp;
        temp = *rk.add(ii + 3); *rk.add(ii + 3) = *rk.add(jj + 3); *rk.add(jj + 3) = temp;

        i += 4;
        j -= 4;
    }

    // apply the inverse MixColumn transform to all round keys but the first
    // and the last
    let mut r: c_int = 1;
    while r < rounds {
        let base: usize = 4 * (r as usize);
        let mut k: usize = 0;
        while k < 4 {
            let x: u32 = *rk.add(base + k);
            let v: u32 = td(&TD0, te4(x >> 24))
                ^ td(&TD1, te4((x >> 16) & 0xff))
                ^ td(&TD2, te4((x >> 8) & 0xff))
                ^ td(&TD3, te4(x));
            *rk.add(base + k) = v;
            k += 1;
        }
        r += 1;
    }

    status = 0;
    status
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

// ── `for (i = 1; i < (key->rounds; i++)` · aes.c:861-883 · chunk 83 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 1; i < (key->rounds); i++) {
//       rk += 4;
//       rk[0] =
//        Td0[Te4[(rk[0] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[0] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[0] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[0] ) & 0xff] & 0xff];
//       rk[1] =
//        Td0[Te4[(rk[1] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[1] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[1] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[1] ) & 0xff] & 0xff];
//       rk[2] =
//        Td0[Te4[(rk[2] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[2] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[2] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[2] ) & 0xff] & 0xff];
//       rk[3] =
//        Td0[Te4[(rk[3] >> 24) ] & 0xff] ^
//        Td1[Te4[(rk[3] >> 16) & 0xff] & 0xff] ^
//        Td2[Te4[(rk[3] >> 8) & 0xff] & 0xff] ^
//        Td3[Te4[(rk[3] ) & 0xff] & 0xff];
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk83 {
use core::ffi::c_int;

#[repr(C)]
struct AesKey {
    rd_key: [u32; 60],
    rounds: c_int,
}

/// Port of the AES key decryption transformation loop (aes.c lines 861-883).
/// 
/// # Safety
/// The caller must ensure `key` points to a valid AES_KEY, and the Te4/Td0-3
/// pointers point to valid static arrays of 256 u32 values.
#[no_mangle]
pub extern "C" fn AES_set_decrypt_key__part83(
    key: *mut AesKey,
    te4: *const u32,
    td0: *const u32,
    td1: *const u32,
    td2: *const u32,
    td3: *const u32,
) -> c_int {
    let key = unsafe { key.as_mut().unwrap() };
    let rounds = key.rounds;
    let te4_slice = unsafe { core::slice::from_raw_parts(te4, 256) };
    let td0_slice = unsafe { core::slice::from_raw_parts(td0, 256) };
    let td1_slice = unsafe { core::slice::from_raw_parts(td1, 256) };
    let td2_slice = unsafe { core::slice::from_raw_parts(td2, 256) };
    let td3_slice = unsafe { core::slice::from_raw_parts(td3, 256) };

    let mut i: c_int = 1;

    while i < rounds {
        let offset = 4usize * i as usize;

        let rk0 = key.rd_key[offset];
        let rk1 = key.rd_key[offset + 1];
        let rk2 = key.rd_key[offset + 2];
        let rk3 = key.rd_key[offset + 3];

        // Transform rk[0]
        let b0 = ((rk0 >> 24) & 0xff) as usize;
        let te4_b0 = te4_slice[b0];
        let t0_idx = (te4_b0 & 0xff) as usize;
        let t0 = td0_slice[t0_idx];

        let b1 = ((rk0 >> 16) & 0xff) as usize;
        let te4_b1 = te4_slice[b1];
        let t1_idx = (te4_b1 & 0xff) as usize;
        let t1 = td1_slice[t1_idx];

        let b2 = ((rk0 >> 8) & 0xff) as usize;
        let te4_b2 = te4_slice[b2];
        let t2_idx = (te4_b2 & 0xff) as usize;
        let t2 = td2_slice[t2_idx];

        let b3 = (rk0 & 0xff) as usize;
        let te4_b3 = te4_slice[b3];
        let t3_idx = (te4_b3 & 0xff) as usize;
        let t3 = td3_slice[t3_idx];

        let new_rk0 = t0 ^ t1 ^ t2 ^ t3;

        // Transform rk[1]
        let b0 = ((rk1 >> 24) & 0xff) as usize;
        let te4_b0 = te4_slice[b0];
        let t0_idx = (te4_b0 & 0xff) as usize;
        let t0 = td0_slice[t0_idx];

        let b1 = ((rk1 >> 16) & 0xff) as usize;
        let te4_b1 = te4_slice[b1];
        let t1_idx = (te4_b1 & 0xff) as usize;
        let t1 = td1_slice[t1_idx];

        let b2 = ((rk1 >> 8) & 0xff) as usize;
        let te4_b2 = te4_slice[b2];
        let t2_idx = (te4_b2 & 0xff) as usize;
        let t2 = td2_slice[t2_idx];

        let b3 = (rk1 & 0xff) as usize;
        let te4_b3 = te4_slice[b3];
        let t3_idx = (te4_b3 & 0xff) as usize;
        let t3 = td3_slice[t3_idx];

        let new_rk1 = t0 ^ t1 ^ t2 ^ t3;

        // Transform rk[2]
        let b0 = ((rk2 >> 24) & 0xff) as usize;
        let te4_b0 = te4_slice[b0];
        let t0_idx = (te4_b0 & 0xff) as usize;
        let t0 = td0_slice[t0_idx];

        let b1 = ((rk2 >> 16) & 0xff) as usize;
        let te4_b1 = te4_slice[b1];
        let t1_idx = (te4_b1 & 0xff) as usize;
        let t1 = td1_slice[t1_idx];

        let b2 = ((rk2 >> 8) & 0xff) as usize;
        let te4_b2 = te4_slice[b2];
        let t2_idx = (te4_b2 & 0xff) as usize;
        let t2 = td2_slice[t2_idx];

        let b3 = (rk2 & 0xff) as usize;
        let te4_b3 = te4_slice[b3];
        let t3_idx = (te4_b3 & 0xff) as usize;
        let t3 = td3_slice[t3_idx];

        let new_rk2 = t0 ^ t1 ^ t2 ^ t3;

        // Transform rk[3]
        let b0 = ((rk3 >> 24) & 0xff) as usize;
        let te4_b0 = te4_slice[b0];
        let t0_idx = (te4_b0 & 0xff) as usize;
        let t0 = td0_slice[t0_idx];

        let b1 = ((rk3 >> 16) & 0xff) as usize;
        let te4_b1 = te4_slice[b1];
        let t1_idx = (te4_b1 & 0xff) as usize;
        let t1 = td1_slice[t1_idx];

        let b2 = ((rk3 >> 8) & 0xff) as usize;
        let te4_b2 = te4_slice[b2];
        let t2_idx = (te4_b2 & 0xff) as usize;
        let t2 = td2_slice[t2_idx];

        let b3 = (rk3 & 0xff) as usize;
        let te4_b3 = te4_slice[b3];
        let t3_idx = (te4_b3 & 0xff) as usize;
        let t3 = td3_slice[t3_idx];

        let new_rk3 = t0 ^ t1 ^ t2 ^ t3;

        key.rd_key[offset] = new_rk0;
        key.rd_key[offset + 1] = new_rk1;
        key.rd_key[offset + 2] = new_rk2;
        key.rd_key[offset + 3] = new_rk3;

        i += 1;
    }

    0
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

// ── `AES_encrypt` · aes.c:892-1077 · chunk 85 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void AES_encrypt(const unsigned char *in, unsigned char *out,
//        const AES_KEY *key) {
//     
//      const u32 *rk;
//      u32 s0, s1, s2, s3, t0, t1, t2, t3;
//     
//      int r;
//     
//     
//      
//     ((void) (0))
//                             ;
//      rk = key->rd_key;
//     
//     
//     
//     
//     
//      s0 = (((u32)(in)[0] << 24) ^ ((u32)(in)[1] << 16) ^ ((u32)(in)[2] << 8) ^ ((u32)(in)[3])) ^ rk[0];
//      s1 = (((u32)(in + 4)[0] << 24) ^ ((u32)(in + 4)[1] << 16) ^ ((u32)(in + 4)[2] << 8) ^ ((u32)(in + 4)[3])) ^ rk[1];
//      s2 = (((u32)(in + 8)[0] << 24) ^ ((u32)(in + 8)[1] << 16) ^ ((u32)(in + 8)[2] << 8) ^ ((u32)(in + 8)[3])) ^ rk[2];
//      s3 = (((u32)(in + 12)[0] << 24) ^ ((u32)(in + 12)[1] << 16) ^ ((u32)(in + 12)[2] << 8) ^ ((u32)(in + 12)[3])) ^ rk[3];
//         r = key->rounds >> 1;
//         for (;;) {
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
//     
//     
//     
//     
//     
//      s0 =
//       (Te4[(t0 >> 24) ] & 0xff000000) ^
//       (Te4[(t1 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t2 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t3 ) & 0xff] & 0x000000ff) ^
//       rk[0];
//      { (out)[0] = (u8)((s0) >> 24); (out)[1] = (u8)((s0) >> 16); (out)[2] = (u8)((s0) >> 8); (out)[3] = (u8)(s0); };
//      s1 =
//       (Te4[(t1 >> 24) ] & 0xff000000) ^
//       (Te4[(t2 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t3 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t0 ) & 0xff] & 0x000000ff) ^
//       rk[1];
//      { (out + 4)[0] = (u8)((s1) >> 24); (out + 4)[1] = (u8)((s1) >> 16); (out + 4)[2] = (u8)((s1) >> 8); (out + 4)[3] = (u8)(s1); };
//      s2 =
//       (Te4[(t2 >> 24) ] & 0xff000000) ^
//       (Te4[(t3 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t0 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t1 ) & 0xff] & 0x000000ff) ^
//       rk[2];
//      { (out + 8)[0] = (u8)((s2) >> 24); (out + 8)[1] = (u8)((s2) >> 16); (out + 8)[2] = (u8)((s2) >> 8); (out + 8)[3] = (u8)(s2); };
//      s3 =
//       (Te4[(t3 >> 24) ] & 0xff000000) ^
//       (Te4[(t0 >> 16) & 0xff] & 0x00ff0000) ^
//       (Te4[(t1 >> 8) & 0xff] & 0x0000ff00) ^
//       (Te4[(t2 ) & 0xff] & 0x000000ff) ^
//       rk[3];
//      { (out + 12)[0] = (u8)((s3) >> 24); (out + 12)[1] = (u8)((s3) >> 16); (out + 12)[2] = (u8)((s3) >> 8); (out + 12)[3] = (u8)(s3); };
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk85 {
// Rust port of OpenSSL-style AES_encrypt (T-table implementation).
#![allow(dead_code)]

use core::ffi::{c_int, c_uchar};

/* ---------- compile-time construction of the T tables ---------- */

const fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    let mut i = 0;
    while i < 8 {
        if b & 1u8 != 0 {
            p ^= a;
        }
        let hi: u8 = a & 0x80u8;
        a = (a << 1) & 0xffu8;
        if hi != 0 {
            a ^= 0x1bu8;
        }
        b >>= 1;
        i += 1;
    }
    p
}

const fn ginv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    let mut b: u8 = 1;
    while b != 0 {
        if gmul(a, b) == 1 {
            return b;
        }
        b = b.wrapping_add(1);
    }
    0
}

const fn rotl8(x: u8, n: u32) -> u8 {
    (((x as u32) << n) | ((x as u32) >> (8 - n))) as u8
}

const fn sbox(i: u8) -> u8 {
    let x = ginv(i);
    let s = x ^ rotl8(x, 1) ^ rotl8(x, 2) ^ rotl8(x, 3) ^ rotl8(x, 4);
    s ^ 0x63u8
}

const fn ror32(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}

const fn make_te0() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i: usize = 0;
    while i < 256 {
        let s8 = sbox(i as u8);
        let s = s8 as u32;
        let s2 = gmul(s8, 2) as u32;
        let s3 = gmul(s8, 3) as u32;
        t[i] = (s2 << 24) ^ (s << 16) ^ (s << 8) ^ s3;
        i += 1;
    }
    t
}

const fn make_te4() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i: usize = 0;
    while i < 256 {
        let s = sbox(i as u8) as u32;
        t[i] = s ^ (s << 8) ^ (s << 16) ^ (s << 24);
        i += 1;
    }
    t
}

const fn ror_all(src: &[u32; 256], n: u32) -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i: usize = 0;
    while i < 256 {
        t[i] = ror32(src[i], n);
        i += 1;
    }
    t
}

static TE0: [u32; 256] = make_te0();
static TE1: [u32; 256] = ror_all(&TE0, 8);
static TE2: [u32; 256] = ror_all(&TE0, 16);
static TE3: [u32; 256] = ror_all(&TE0, 24);
static TE4: [u32; 256] = make_te4();

/* ---------- helpers over raw C pointers ---------- */

#[repr(C)]
struct AesKey {
    rd_key: [u32; 4 * (14 + 1)],
    rounds: c_int,
}

#[inline]
fn te(t: &'static [u32; 256], i: u32) -> u32 {
    // Index is always masked to 8 bits by the callers, so it is in bounds.
    unsafe { *t.get_unchecked((i & 0xff) as usize) }
}

#[inline]
fn rk_at(rk: *const u32, i: usize) -> u32 {
    unsafe { *rk.add(i) }
}

#[inline]
fn rk_adv(rk: *const u32, n: usize) -> *const u32 {
    unsafe { rk.add(n) }
}

#[inline]
fn getu32(p: *const c_uchar, off: usize) -> u32 {
    unsafe {
        let q = p.add(off);
        (((*q as u32) << 24)
            ^ (((*q.add(1) as u32) << 16))
            ^ (((*q.add(2) as u32) << 8))
            ^ (*q.add(3) as u32)) as u32
    }
}

#[inline]
fn putu32(p: *mut c_uchar, off: usize, v: u32) {
    unsafe {
        let q = p.add(off);
        *q = (v >> 24) as c_uchar;
        *q.add(1) = (v >> 16) as c_uchar;
        *q.add(2) = (v >> 8) as c_uchar;
        *q.add(3) = v as c_uchar;
    }
}

/* ---------- the ported function ---------- */

#[no_mangle]
pub extern "C" fn AES_encrypt(inp: *const c_uchar, out: *mut c_uchar, key: *const AesKey) {
    let mut rk: *const u32 = unsafe { (*key).rd_key.as_ptr() };

    /* map byte array block to cipher state and add initial round key */
    let mut s0: u32 = getu32(inp, 0) ^ rk_at(rk, 0);
    let mut s1: u32 = getu32(inp, 4) ^ rk_at(rk, 1);
    let mut s2: u32 = getu32(inp, 8) ^ rk_at(rk, 2);
    let mut s3: u32 = getu32(inp, 12) ^ rk_at(rk, 3);

    let mut t0: u32 = 0;
    let mut t1: u32 = 0;
    let mut t2: u32 = 0;
    let mut t3: u32 = 0;

    /* Nr - 1 full rounds */
    let mut r: c_int = unsafe { (*key).rounds } >> 1;
    loop {
        t0 = te(&TE0, s0 >> 24)
            ^ te(&TE1, (s1 >> 16) & 0xff)
            ^ te(&TE2, (s2 >> 8) & 0xff)
            ^ te(&TE3, s3 & 0xff)
            ^ rk_at(rk, 4);
        t1 = te(&TE0, s1 >> 24)
            ^ te(&TE1, (s2 >> 16) & 0xff)
            ^ te(&TE2, (s3 >> 8) & 0xff)
            ^ te(&TE3, s0 & 0xff)
            ^ rk_at(rk, 5);
        t2 = te(&TE0, s2 >> 24)
            ^ te(&TE1, (s3 >> 16) & 0xff)
            ^ te(&TE2, (s0 >> 8) & 0xff)
            ^ te(&TE3, s1 & 0xff)
            ^ rk_at(rk, 6);
        t3 = te(&TE0, s3 >> 24)
            ^ te(&TE1, (s0 >> 16) & 0xff)
            ^ te(&TE2, (s1 >> 8) & 0xff)
            ^ te(&TE3, s2 & 0xff)
            ^ rk_at(rk, 7);

        rk = rk_adv(rk, 8);
        r -= 1;
        if r == 0 {
            break;
        }

        s0 = te(&TE0, t0 >> 24)
            ^ te(&TE1, (t1 >> 16) & 0xff)
            ^ te(&TE2, (t2 >> 8) & 0xff)
            ^ te(&TE3, t3 & 0xff)
            ^ rk_at(rk, 0);
        s1 = te(&TE0, t1 >> 24)
            ^ te(&TE1, (t2 >> 16) & 0xff)
            ^ te(&TE2, (t3 >> 8) & 0xff)
            ^ te(&TE3, t0 & 0xff)
            ^ rk_at(rk, 1);
        s2 = te(&TE0, t2 >> 24)
            ^ te(&TE1, (t3 >> 16) & 0xff)
            ^ te(&TE2, (t0 >> 8) & 0xff)
            ^ te(&TE3, t1 & 0xff)
            ^ rk_at(rk, 2);
        s3 = te(&TE0, t3 >> 24)
            ^ te(&TE1, (t0 >> 16) & 0xff)
            ^ te(&TE2, (t1 >> 8) & 0xff)
            ^ te(&TE3, t2 & 0xff)
            ^ rk_at(rk, 3);
    }

    /* apply last round and map cipher state to byte array block */
    s0 = (te(&TE4, t0 >> 24) & 0xff00_0000)
        ^ (te(&TE4, (t1 >> 16) & 0xff) & 0x00ff_0000)
        ^ (te(&TE4, (t2 >> 8) & 0xff) & 0x0000_ff00)
        ^ (te(&TE4, t3 & 0xff) & 0x0000_00ff)
        ^ rk_at(rk, 0);
    putu32(out, 0, s0);

    s1 = (te(&TE4, t1 >> 24) & 0xff00_0000)
        ^ (te(&TE4, (t2 >> 16) & 0xff) & 0x00ff_0000)
        ^ (te(&TE4, (t3 >> 8) & 0xff) & 0x0000_ff00)
        ^ (te(&TE4, t0 & 0xff) & 0x0000_00ff)
        ^ rk_at(rk, 1);
    putu32(out, 4, s1);

    s2 = (te(&TE4, t2 >> 24) & 0xff00_0000)
        ^ (te(&TE4, (t3 >> 16) & 0xff) & 0x00ff_0000)
        ^ (te(&TE4, (t0 >> 8) & 0xff) & 0x0000_ff00)
        ^ (te(&TE4, t1 & 0xff) & 0x0000_00ff)
        ^ rk_at(rk, 2);
    putu32(out, 8, s2);

    s3 = (te(&TE4, t3 >> 24) & 0xff00_0000)
        ^ (te(&TE4, (t0 >> 16) & 0xff) & 0x00ff_0000)
        ^ (te(&TE4, (t1 >> 8) & 0xff) & 0x0000_ff00)
        ^ (te(&TE4, t2 & 0xff) & 0x0000_00ff)
        ^ rk_at(rk, 3);
    putu32(out, 12, s3);
}
}

// ── `const u32 *rk; u32 s0, s1, s2, s3, t0, t1, t2, t3; int r; ((void) (0)) ; rk = key->rd_key; s0 =…` · aes.c:895-987 · chunk 86 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const u32 *rk;
//      u32 s0, s1, s2, s3, t0, t1, t2, t3;
//     
//      int r;
//     
//     
//      
//     ((void) (0))
//                             ;
//      rk = key->rd_key;
//     
//     
//     
//     
//     
//      s0 = (((u32)(in)[0] << 24) ^ ((u32)(in)[1] << 16) ^ ((u32)(in)[2] << 8) ^ ((u32)(in)[3])) ^ rk[0];
//      s1 = (((u32)(in + 4)[0] << 24) ^ ((u32)(in + 4)[1] << 16) ^ ((u32)(in + 4)[2] << 8) ^ ((u32)(in + 4)[3])) ^ rk[1];
//      s2 = (((u32)(in + 8)[0] << 24) ^ ((u32)(in + 8)[1] << 16) ^ ((u32)(in + 8)[2] << 8) ^ ((u32)(in + 8)[3])) ^ rk[2];
//      s3 = (((u32)(in + 12)[0] << 24) ^ ((u32)(in + 12)[1] << 16) ^ ((u32)(in + 12)[2] << 8) ^ ((u32)(in + 12)[3])) ^ rk[3];
//         r = key->rounds >> 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk86 {
// Port of the first statements of AES_encrypt: key schedule pointer,
// initial key addition and round counter setup.

#[repr(C)]
pub struct AesKeySt {
    pub rd_key: [u32; 60],
    pub rounds: i32,
}

#[no_mangle]
pub extern "C" fn AES_encrypt__part86(
    inp: *const u8,
    key: *const AesKeySt,
    rk_out: *mut *const u32,
    s0: *mut u32,
    s1: *mut u32,
    s2: *mut u32,
    s3: *mut u32,
    t0: *mut u32,
    t1: *mut u32,
    t2: *mut u32,
    t3: *mut u32,
    r_out: *mut i32,
) {
    unsafe {
        let rk: *const u32 = (*key).rd_key.as_ptr();
        *rk_out = rk;

        // XOR of four big-endian input bytes with the round key.
        let load = |off: usize| -> u32 {
            ((*inp.add(off) as u32) << 24)
                ^ ((*inp.add(off + 1) as u32) << 16)
                ^ ((*inp.add(off + 2) as u32) << 8)
                ^ (*inp.add(off + 3) as u32)
        };

        *s0 = load(0) ^ *rk.add(0);
        *s1 = load(4) ^ *rk.add(1);
        *s2 = load(8) ^ *rk.add(2);
        *s3 = load(12) ^ *rk.add(3);

        // t0..t3 are assigned by the loop that follows before any use.
        *t0 = 0;
        *t1 = 0;
        *t2 = 0;
        *t3 = 0;

        *r_out = (*key).rounds >> 1;
    }
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

// ── `AES_decrypt` · aes.c:1083-1268 · chunk 97 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void AES_decrypt(const unsigned char *in, unsigned char *out,
//        const AES_KEY *key) {
//     
//      const u32 *rk;
//      u32 s0, s1, s2, s3, t0, t1, t2, t3;
//     
//      int r;
//     
//     
//      
//     ((void) (0))
//                             ;
//      rk = key->rd_key;
//     
//     
//     
//     
//     
//         s0 = (((u32)(in)[0] << 24) ^ ((u32)(in)[1] << 16) ^ ((u32)(in)[2] << 8) ^ ((u32)(in)[3])) ^ rk[0];
//         s1 = (((u32)(in + 4)[0] << 24) ^ ((u32)(in + 4)[1] << 16) ^ ((u32)(in + 4)[2] << 8) ^ ((u32)(in + 4)[3])) ^ rk[1];
//         s2 = (((u32)(in + 8)[0] << 24) ^ ((u32)(in + 8)[1] << 16) ^ ((u32)(in + 8)[2] << 8) ^ ((u32)(in + 8)[3])) ^ rk[2];
//         s3 = (((u32)(in + 12)[0] << 24) ^ ((u32)(in + 12)[1] << 16) ^ ((u32)(in + 12)[2] << 8) ^ ((u32)(in + 12)[3])) ^ rk[3];
//         r = key->rounds >> 1;
//         for (;;) {
//             t0 =
//                 Td0[(s0 >> 24) ] ^
//                 Td1[(s3 >> 16) & 0xff] ^
//                 Td2[(s2 >> 8) & 0xff] ^
//                 Td3[(s1 ) & 0xff] ^
//                 rk[4];
//             t1 =
//                 Td0[(s1 >> 24) ] ^
//                 Td1[(s0 >> 16) & 0xff] ^
//                 Td2[(s3 >> 8) & 0xff] ^
//                 Td3[(s2 ) & 0xff] ^
//                 rk[5];
//             t2 =
//                 Td0[(s2 >> 24) ] ^
//                 Td1[(s1 >> 16) & 0xff] ^
//                 Td2[(s0 >> 8) & 0xff] ^
//                 Td3[(s3 ) & 0xff] ^
//                 rk[6];
//             t3 =
//                 Td0[(s3 >> 24) ] ^
//                 Td1[(s2 >> 16) & 0xff] ^
//                 Td2[(s1 >> 8) & 0xff] ^
//                 Td3[(s0 ) & 0xff] ^
//                 rk[7];
//     
//             rk += 8;
//             if (--r == 0) {
//                 break;
//             }
//     
//             s0 =
//                 Td0[(t0 >> 24) ] ^
//                 Td1[(t3 >> 16) & 0xff] ^
//                 Td2[(t2 >> 8) & 0xff] ^
//                 Td3[(t1 ) & 0xff] ^
//                 rk[0];
//             s1 =
//                 Td0[(t1 >> 24) ] ^
//                 Td1[(t0 >> 16) & 0xff] ^
//                 Td2[(t3 >> 8) & 0xff] ^
//                 Td3[(t2 ) & 0xff] ^
//                 rk[1];
//             s2 =
//                 Td0[(t2 >> 24) ] ^
//                 Td1[(t1 >> 16) & 0xff] ^
//                 Td2[(t0 >> 8) & 0xff] ^
//                 Td3[(t3 ) & 0xff] ^
//                 rk[2];
//             s3 =
//                 Td0[(t3 >> 24) ] ^
//                 Td1[(t2 >> 16) & 0xff] ^
//                 Td2[(t1 >> 8) & 0xff] ^
//                 Td3[(t0 ) & 0xff] ^
//                 rk[3];
//         }
//     
//     
//     
//     
//     
//         s0 =
//          (Td4[(t0 >> 24) ] & 0xff000000) ^
//          (Td4[(t3 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t2 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t1 ) & 0xff] & 0x000000ff) ^
//          rk[0];
//      { (out)[0] = (u8)((s0) >> 24); (out)[1] = (u8)((s0) >> 16); (out)[2] = (u8)((s0) >> 8); (out)[3] = (u8)(s0); };
//         s1 =
//          (Td4[(t1 >> 24) ] & 0xff000000) ^
//          (Td4[(t0 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t3 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t2 ) & 0xff] & 0x000000ff) ^
//          rk[1];
//      { (out + 4)[0] = (u8)((s1) >> 24); (out + 4)[1] = (u8)((s1) >> 16); (out + 4)[2] = (u8)((s1) >> 8); (out + 4)[3] = (u8)(s1); };
//         s2 =
//          (Td4[(t2 >> 24) ] & 0xff000000) ^
//          (Td4[(t1 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t0 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t3 ) & 0xff] & 0x000000ff) ^
//          rk[2];
//      { (out + 8)[0] = (u8)((s2) >> 24); (out + 8)[1] = (u8)((s2) >> 16); (out + 8)[2] = (u8)((s2) >> 8); (out + 8)[3] = (u8)(s2); };
//         s3 =
//          (Td4[(t3 >> 24) ] & 0xff000000) ^
//          (Td4[(t2 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t1 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t0 ) & 0xff] & 0x000000ff) ^
//          rk[3];
//      { (out + 12)[0] = (u8)((s3) >> 24); (out + 12)[1] = (u8)((s3) >> 16); (out + 12)[2] = (u8)((s3) >> 8); (out + 12)[3] = (u8)(s3); };
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk97 {
// Rust rewrite of AES_decrypt() from aes.c (AES_ASM / non-FULL_UNROLL path).
// The C lookup tables Td0..Td4 are file-static, so they are embedded here.
// Td1/Td2/Td3 are byte rotations of Td0, so they are derived from TD0.

#[repr(C)]
pub struct AesKey {
    pub rd_key: [u32; 4 * (14 + 1)],
    pub rounds: i32,
}

static TD0: [u32; 256] = [
    0x51f4a750, 0x7e416553, 0x1a17a4c3, 0x3a275e96,
    0x3bab6bcb, 0x1f9d45f1, 0xacfa58ab, 0x4be30393,
    0x2030fa55, 0xad766df6, 0x88cc7691, 0xf5024c25,
    0x4fe5d7fc, 0xc52acbd7, 0x26354480, 0xb562a38f,
    0xdeb15a49, 0x25ba1b67, 0x45ea0e98, 0x5dfec0e1,
    0xc32f7502, 0x814cf012, 0x8d4697a3, 0x6bd3f9c6,
    0x038f5fe7, 0x15929c95, 0xbf6d7aeb, 0x955259da,
    0xd4be832d, 0x587421d3, 0x49e06929, 0x8ec9c844,
    0x75c2896a, 0xf48e7978, 0x99583e6b, 0x27b971dd,
    0xbee14fb6, 0xf088ad17, 0xc920ac66, 0x7dce3ab4,
    0x63df4a18, 0xe51a3182, 0x97513360, 0x62537f45,
    0xb16477e0, 0xbb6bae84, 0xfe81a01c, 0xf9082b94,
    0x70486858, 0x8f45fd19, 0x94de6c87, 0x527bf8b7,
    0xab73d323, 0x724b02e2, 0xe31f8f57, 0x6655ab2a,
    0xb2eb2807, 0x2fb5c203, 0x86c57b9a, 0xd33708a5,
    0x302887f2, 0x23bfa5b2, 0x02036aba, 0xed16825c,
    0x8acf1c2b, 0xa779b492, 0xf307f2f0, 0x4e69e2a1,
    0x65daf4cd, 0x0605bed5, 0xd134621f, 0xc4a6fe8a,
    0x342e539d, 0xa2f355a0, 0x058ae132, 0xa4f6eb75,
    0x0b83ec39, 0x4060efaa, 0x5e719f06, 0xbd6e1051,
    0x3e218af9, 0x96dd063d, 0xdd3e05ae, 0x4de6bd46,
    0x91548db5, 0x71c45d05, 0x0406d46f, 0x605015ff,
    0x1998fb24, 0xd6bde997, 0x894043cc, 0x67d99e77,
    0xb0e842bd, 0x07898b88, 0xe7195b38, 0x79c8eedb,
    0xa17c0a47, 0x7c420fe9, 0xf8841ec9, 0x00000000,
    0x09808683, 0x322bed48, 0x1e1170ac, 0x6c5a724e,
    0xfd0efffb, 0x0f853856, 0x3daed51e, 0x362d3927,
    0x0a0fd964, 0x685ca621, 0x9b5b54d1, 0x24362e3a,
    0x0c0a67b1, 0x9357e70f, 0xb4ee96d2, 0x1b9b919e,
    0x80c0c54f, 0x61dc20a2, 0x5a774b69, 0x1c121a16,
    0xe293ba0a, 0xc0a02ae5, 0x3c22e043, 0x121b171d,
    0x0e090d0b, 0xf28bc7ad, 0x2db6a8b9, 0x141ea9c8,
    0x57f11985, 0xaf75074c, 0xee99ddbb, 0xa37f60fd,
    0xf701269f, 0x5c72f5bc, 0x44663bc5, 0x5bfb7e34,
    0x8b432976, 0xcb23c6dc, 0xb6edfc68, 0xb8e4f163,
    0xd731dcca, 0x42638510, 0x13972240, 0x84c61120,
    0x854a247d, 0xd2bb3df8, 0xaef93211, 0xc729a16d,
    0x1d9e2f4b, 0xdcb230f3, 0x0d8652ec, 0x77c1e3d0,
    0x2bb3166c, 0xa970b999, 0x119448fa, 0x47e96422,
    0xa8fc8cc4, 0xa0f03f1a, 0x567d2cd8, 0x223390ef,
    0x87494ec7, 0xd938d1c1, 0x8ccaa2fe, 0x98d40b36,
    0xa6f581cf, 0xa57ade28, 0xdab78e26, 0x3fadbfa4,
    0x2c3a9de4, 0x5078920d, 0x6a5fcc9b, 0x547e4662,
    0xf68d13c2, 0x90d8b8e8, 0x2e39f75e, 0x82c3aff5,
    0x9f5d80be, 0x69d0937c, 0x6fd52da9, 0xcf2512b3,
    0xc8ac993b, 0x10187da7, 0xe89c636e, 0xdb3bbb7b,
    0xcd267809, 0x6e5918f4, 0xec9ab701, 0x834f9aa8,
    0xe6956e65, 0xaaffe67e, 0x21bccf08, 0xef15e8e6,
    0xbae79bd9, 0x4a6f36ce, 0xea9f09d4, 0x29b07cd6,
    0x31a4b2af, 0x2a3f2331, 0xc6a59430, 0x35a266c0,
    0x744ebc37, 0xfc82caa6, 0xe090d0b0, 0x33a7d815,
    0xf104984a, 0x41ecdaf7, 0x7fcd500e, 0x1791f62f,
    0x764dd68d, 0x43efb04d, 0xccaa4d54, 0xe49604df,
    0x9ed1b5e3, 0x4c6a881b, 0xc12c1fb8, 0x4665517f,
    0x9d5eea04, 0x018c355d, 0xfa877473, 0xfb0b412e,
    0xb3671d5a, 0x92dbd252, 0xe9105633, 0x6dd64713,
    0x9ad7618c, 0x37a10c7a, 0x59f8148e, 0xeb133c89,
    0xcea927ee, 0xb761c935, 0xe11ce5ed, 0x7a47b13c,
    0x9cd2df59, 0x55f2733f, 0x1814ce79, 0x73c737bf,
    0x53f7cdea, 0x5ffdaa5b, 0xdf3d6f14, 0x7844db86,
    0xcaaff381, 0xb968c43e, 0x3824342c, 0xc2a3405f,
    0x161dc372, 0xbce2250c, 0x283c498b, 0xff0d9541,
    0x39a80171, 0x080cb3de, 0xd8b4e49c, 0x6456c190,
    0x7bcb8461, 0xd532b670, 0x486c5c74, 0xd0b85742,
];

static TD4: [u32; 256] = [
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

#[inline]
fn td0(i: u32) -> u32 {
    TD0[(i & 0xff) as usize]
}

#[inline]
fn td1(i: u32) -> u32 {
    TD0[(i & 0xff) as usize].rotate_right(8)
}

#[inline]
fn td2(i: u32) -> u32 {
    TD0[(i & 0xff) as usize].rotate_right(16)
}

#[inline]
fn td3(i: u32) -> u32 {
    TD0[(i & 0xff) as usize].rotate_right(24)
}

#[inline]
fn td4(i: u32) -> u32 {
    TD4[(i & 0xff) as usize]
}

#[inline]
unsafe fn getu32(p: *const u8) -> u32 {
    ((*p as u32) << 24)
        ^ ((*p.add(1) as u32) << 16)
        ^ ((*p.add(2) as u32) << 8)
        ^ (*p.add(3) as u32)
}

#[inline]
unsafe fn putu32(p: *mut u8, s: u32) {
    *p = (s >> 24) as u8;
    *p.add(1) = (s >> 16) as u8;
    *p.add(2) = (s >> 8) as u8;
    *p.add(3) = s as u8;
}

#[no_mangle]
pub unsafe extern "C" fn AES_decrypt(in_: *const u8, out: *mut u8, key: *const AesKey) {
    unsafe {
        let rk: *const u32 = (*key).rd_key.as_ptr();

        // map byte array block to cipher state and add initial round key
        let mut s0: u32 = getu32(in_) ^ *rk.add(0);
        let mut s1: u32 = getu32(in_.add(4)) ^ *rk.add(1);
        let mut s2: u32 = getu32(in_.add(8)) ^ *rk.add(2);
        let mut s3: u32 = getu32(in_.add(12)) ^ *rk.add(3);

        let mut t0: u32;
        let mut t1: u32;
        let mut t2: u32;
        let mut t3: u32;

        // Nr - 1 full rounds
        let mut r: i32 = (*key).rounds >> 1;
        let mut base: usize = 0;

        loop {
            t0 = td0(s0 >> 24)
                ^ td1((s3 >> 16) & 0xff)
                ^ td2((s2 >> 8) & 0xff)
                ^ td3(s1 & 0xff)
                ^ *rk.add(base + 4);
            t1 = td0(s1 >> 24)
                ^ td1((s0 >> 16) & 0xff)
                ^ td2((s3 >> 8) & 0xff)
                ^ td3(s2 & 0xff)
                ^ *rk.add(base + 5);
            t2 = td0(s2 >> 24)
                ^ td1((s1 >> 16) & 0xff)
                ^ td2((s0 >> 8) & 0xff)
                ^ td3(s3 & 0xff)
                ^ *rk.add(base + 6);
            t3 = td0(s3 >> 24)
                ^ td1((s2 >> 16) & 0xff)
                ^ td2((s1 >> 8) & 0xff)
                ^ td3(s0 & 0xff)
                ^ *rk.add(base + 7);

            base += 8;
            r -= 1;
            if r == 0 {
                break;
            }

            s0 = td0(t0 >> 24)
                ^ td1((t3 >> 16) & 0xff)
                ^ td2((t2 >> 8) & 0xff)
                ^ td3(t1 & 0xff)
                ^ *rk.add(base);
            s1 = td0(t1 >> 24)
                ^ td1((t0 >> 16) & 0xff)
                ^ td2((t3 >> 8) & 0xff)
                ^ td3(t2 & 0xff)
                ^ *rk.add(base + 1);
            s2 = td0(t2 >> 24)
                ^ td1((t1 >> 16) & 0xff)
                ^ td2((t0 >> 8) & 0xff)
                ^ td3(t3 & 0xff)
                ^ *rk.add(base + 2);
            s3 = td0(t3 >> 24)
                ^ td1((t2 >> 16) & 0xff)
                ^ td2((t1 >> 8) & 0xff)
                ^ td3(t0 & 0xff)
                ^ *rk.add(base + 3);
        }

        // apply last round and map cipher state to byte array block
        s0 = (td4(t0 >> 24) & 0xff000000)
            ^ (td4((t3 >> 16) & 0xff) & 0x00ff0000)
            ^ (td4((t2 >> 8) & 0xff) & 0x0000ff00)
            ^ (td4(t1 & 0xff) & 0x000000ff)
            ^ *rk.add(base);
        putu32(out, s0);

        s1 = (td4(t1 >> 24) & 0xff000000)
            ^ (td4((t0 >> 16) & 0xff) & 0x00ff0000)
            ^ (td4((t3 >> 8) & 0xff) & 0x0000ff00)
            ^ (td4(t2 & 0xff) & 0x000000ff)
            ^ *rk.add(base + 1);
        putu32(out.add(4), s1);

        s2 = (td4(t2 >> 24) & 0xff000000)
            ^ (td4((t1 >> 16) & 0xff) & 0x00ff0000)
            ^ (td4((t0 >> 8) & 0xff) & 0x0000ff00)
            ^ (td4(t3 & 0xff) & 0x000000ff)
            ^ *rk.add(base + 2);
        putu32(out.add(8), s2);

        s3 = (td4(t3 >> 24) & 0xff000000)
            ^ (td4((t2 >> 16) & 0xff) & 0x00ff0000)
            ^ (td4((t1 >> 8) & 0xff) & 0x0000ff00)
            ^ (td4(t0 & 0xff) & 0x000000ff)
            ^ *rk.add(base + 3);
        putu32(out.add(12), s3);
    }
}
}

// ── `const u32 *rk; u32 s0, s1, s2, s3, t0, t1, t2, t3; int r; ((void) (0)) ; rk = key->rd_key; s0 =…` · aes.c:1086-1178 · chunk 98 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const u32 *rk;
//      u32 s0, s1, s2, s3, t0, t1, t2, t3;
//     
//      int r;
//     
//     
//      
//     ((void) (0))
//                             ;
//      rk = key->rd_key;
//     
//     
//     
//     
//     
//         s0 = (((u32)(in)[0] << 24) ^ ((u32)(in)[1] << 16) ^ ((u32)(in)[2] << 8) ^ ((u32)(in)[3])) ^ rk[0];
//         s1 = (((u32)(in + 4)[0] << 24) ^ ((u32)(in + 4)[1] << 16) ^ ((u32)(in + 4)[2] << 8) ^ ((u32)(in + 4)[3])) ^ rk[1];
//         s2 = (((u32)(in + 8)[0] << 24) ^ ((u32)(in + 8)[1] << 16) ^ ((u32)(in + 8)[2] << 8) ^ ((u32)(in + 8)[3])) ^ rk[2];
//         s3 = (((u32)(in + 12)[0] << 24) ^ ((u32)(in + 12)[1] << 16) ^ ((u32)(in + 12)[2] << 8) ^ ((u32)(in + 12)[3])) ^ rk[3];
//         r = key->rounds >> 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk98 {
// Ported piece of AES_decrypt: key-schedule pointer setup, initial state load,
// and round counter computation.

#[repr(C)]
pub struct AesKeySt {
    pub rd_key: [u32; 4 * (14 + 1)],
    pub rounds: core::ffi::c_int,
}

#[no_mangle]
pub unsafe extern "C" fn AES_decrypt__part98(
    inp: *const u8,
    key: *const AesKeySt,
    rk_out: *mut *const u32,
    s0_out: *mut u32,
    s1_out: *mut u32,
    s2_out: *mut u32,
    s3_out: *mut u32,
    r_out: *mut core::ffi::c_int,
) {
    let k = &*key;
    let rk: *const u32 = k.rd_key.as_ptr();
    *rk_out = rk;

    let b = |i: usize| -> u32 { *inp.add(i) as u32 };

    let s0 = ((b(0) << 24) ^ (b(1) << 16) ^ (b(2) << 8) ^ b(3)) ^ *rk.add(0);
    let s1 = ((b(4) << 24) ^ (b(5) << 16) ^ (b(6) << 8) ^ b(7)) ^ *rk.add(1);
    let s2 = ((b(8) << 24) ^ (b(9) << 16) ^ (b(10) << 8) ^ b(11)) ^ *rk.add(2);
    let s3 = ((b(12) << 24) ^ (b(13) << 16) ^ (b(14) << 8) ^ b(15)) ^ *rk.add(3);

    *s0_out = s0;
    *s1_out = s1;
    *s2_out = s2;
    *s3_out = s3;

    *r_out = k.rounds >> 1;
}
}

// ── `for (; ; )` · aes.c:1179-1234 · chunk 99 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for (;;) {
//             t0 =
//                 Td0[(s0 >> 24) ] ^
//                 Td1[(s3 >> 16) & 0xff] ^
//                 Td2[(s2 >> 8) & 0xff] ^
//                 Td3[(s1 ) & 0xff] ^
//                 rk[4];
//             t1 =
//                 Td0[(s1 >> 24) ] ^
//                 Td1[(s0 >> 16) & 0xff] ^
//                 Td2[(s3 >> 8) & 0xff] ^
//                 Td3[(s2 ) & 0xff] ^
//                 rk[5];
//             t2 =
//                 Td0[(s2 >> 24) ] ^
//                 Td1[(s1 >> 16) & 0xff] ^
//                 Td2[(s0 >> 8) & 0xff] ^
//                 Td3[(s3 ) & 0xff] ^
//                 rk[6];
//             t3 =
//                 Td0[(s3 >> 24) ] ^
//                 Td1[(s2 >> 16) & 0xff] ^
//                 Td2[(s1 >> 8) & 0xff] ^
//                 Td3[(s0 ) & 0xff] ^
//                 rk[7];
//     
//             rk += 8;
//             if (--r == 0) {
//                 break;
//             }
//     
//             s0 =
//                 Td0[(t0 >> 24) ] ^
//                 Td1[(t3 >> 16) & 0xff] ^
//                 Td2[(t2 >> 8) & 0xff] ^
//                 Td3[(t1 ) & 0xff] ^
//                 rk[0];
//             s1 =
//                 Td0[(t1 >> 24) ] ^
//                 Td1[(t0 >> 16) & 0xff] ^
//                 Td2[(t3 >> 8) & 0xff] ^
//                 Td3[(t2 ) & 0xff] ^
//                 rk[1];
//             s2 =
//                 Td0[(t2 >> 24) ] ^
//                 Td1[(t1 >> 16) & 0xff] ^
//                 Td2[(t0 >> 8) & 0xff] ^
//                 Td3[(t3 ) & 0xff] ^
//                 rk[2];
//             s3 =
//                 Td0[(t3 >> 24) ] ^
//                 Td1[(t2 >> 16) & 0xff] ^
//                 Td2[(t1 >> 8) & 0xff] ^
//                 Td3[(t0 ) & 0xff] ^
//                 rk[3];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk99 {
// Port of the AES_decrypt decryption round loop (aes.c lines 1179-1234).
// The Td0..Td3 tables are `static const` in the C translation unit (no linker
// symbols), so they are embedded here. Td1/Td2/Td3 are byte rotations of Td0.

use core::ffi::c_int;

const TD0_V: [u32; 256] = [
    0x51f4a750, 0x7e416553, 0x1a17a4c3, 0x3a275e96,
    0x3bab6bcb, 0x1f9d45f1, 0xacfa58ab, 0x4be30393,
    0x2030fa55, 0xad766df6, 0x88cc7691, 0xf5024c25,
    0x4fe5d7fc, 0xc52acbd7, 0x26354480, 0xb562a38f,
    0xdeb15a49, 0x25ba1b67, 0x45ea0e98, 0x5dfec0e1,
    0xc32f7502, 0x814cf012, 0x8d4697a3, 0x6bd3f9c6,
    0x038f5fe7, 0x15929c95, 0xbf6d7aeb, 0x955259da,
    0xd4be832d, 0x587421d3, 0x49e06929, 0x8ec9c844,
    0x75c2896a, 0xf48e7978, 0x99583e6b, 0x27b971dd,
    0xbee14fb6, 0xf088ad17, 0xc920ac66, 0x7dce3ab4,
    0x63df4a18, 0xe51a3182, 0x97513360, 0x62537f45,
    0xb16477e0, 0xbb6bae84, 0xfe81a01c, 0xf9082b94,
    0x70486858, 0x8f45fd19, 0x94de6c87, 0x527bf8b7,
    0xab73d323, 0x724b02e2, 0xe31f8f57, 0x6655ab2a,
    0xb2eb2807, 0x2fb5c203, 0x86c57b9a, 0xd33708a5,
    0x302887f2, 0x23bfa5b2, 0x02036aba, 0xed16825c,
    0x8acf1c2b, 0xa779b492, 0xf307f2f0, 0x4e69e2a1,
    0x65daf4cd, 0x0605bed5, 0xd134621f, 0xc4a6fe8a,
    0x342e539d, 0xa2f355a0, 0x058ae132, 0xa4f6eb75,
    0x0b83ec39, 0x4060efaa, 0x5e719f06, 0xbd6e1051,
    0x3e218af9, 0x96dd063d, 0xdd3e05ae, 0x4de6bd46,
    0x91548db5, 0x71c45d05, 0x0406d46f, 0x605015ff,
    0x1998fb24, 0xd6bde997, 0x894043cc, 0x67d99e77,
    0xb0e842bd, 0x07898b88, 0xe7195b38, 0x79c8eedb,
    0xa17c0a47, 0x7c420fe9, 0xf8841ec9, 0x00000000,
    0x09808683, 0x322bed48, 0x1e1170ac, 0x6c5a724e,
    0xfd0efffb, 0x0f853856, 0x3daed51e, 0x362d3927,
    0x0a0fd964, 0x685ca621, 0x9b5b54d1, 0x24362e3a,
    0x0c0a67b1, 0x9357e70f, 0xb4ee96d2, 0x1b9b919e,
    0x80c0c54f, 0x61dc20a2, 0x5a774b69, 0x1c121a16,
    0xe293ba0a, 0xc0a02ae5, 0x3c22e043, 0x121b171d,
    0x0e090d0b, 0xf28bc7ad, 0x2db6a8b9, 0x141ea9c8,
    0x57f11985, 0xaf75074c, 0xee99ddbb, 0xa37f60fd,
    0xf701269f, 0x5c72f5bc, 0x44663bc5, 0x5bfb7e34,
    0x8b432976, 0xcb23c6dc, 0xb6edfc68, 0xb8e4f163,
    0xd731dcca, 0x42638510, 0x13972240, 0x84c61120,
    0x854a247d, 0xd2bb3df8, 0xaef93211, 0xc729a16d,
    0x1d9e2f4b, 0xdcb230f3, 0x0d8652ec, 0x77c1e3d0,
    0x2bb3166c, 0xa970b999, 0x119448fa, 0x47e96422,
    0xa8fc8cc4, 0xa0f03f1a, 0x567d2cd8, 0x223390ef,
    0x87494ec7, 0xd938d1c1, 0x8ccaa2fe, 0x98d40b36,
    0xa6f581cf, 0xa57ade28, 0xdab78e26, 0x3fadbfa4,
    0x2c3a9de4, 0x5078920d, 0x6a5fcc9b, 0x547e4662,
    0xf68d13c2, 0x90d8b8e8, 0x2e39f75e, 0x82c3aff5,
    0x9f5d80be, 0x69d0937c, 0x6fd52da9, 0xcf2512b3,
    0xc8ac993b, 0x10187da7, 0xe89c636e, 0xdb3bbb7b,
    0xcd267809, 0x6e5918f4, 0xec9ab701, 0x834f9aa8,
    0xe6956e65, 0xaaffe67e, 0x21bccf08, 0xef15e8e6,
    0xbae79bd9, 0x4a6f36ce, 0xea9f09d4, 0x29b07cd6,
    0x31a4b2af, 0x2a3f2331, 0xc6a59430, 0x35a266c0,
    0x744ebc37, 0xfc82caa6, 0xe090d0b0, 0x33a7d815,
    0xf104984a, 0x41ecdaf7, 0x7fcd500e, 0x1791f62f,
    0x764dd68d, 0x43efb04d, 0xccaa4d54, 0xe49604df,
    0x9ed1b5e3, 0x4c6a881b, 0xc12c1fb8, 0x4665517f,
    0x9d5eea04, 0x018c355d, 0xfa877473, 0xfb0b412e,
    0xb3671d5a, 0x92dbd252, 0xe9105633, 0x6dd64713,
    0x9ad7618c, 0x37a10c7a, 0x59f8148e, 0xeb133c89,
    0xcea927ee, 0xb761c935, 0xe11ce5ed, 0x7a47b13c,
    0x9cd2df59, 0x55f2733f, 0x1814ce79, 0x73c737bf,
    0x53f7cdea, 0x5ffdaa5b, 0xdf3d6f14, 0x7844db86,
    0xcaaff381, 0xb968c43e, 0x3824342c, 0xc2a3405f,
    0x161dc372, 0xbce2250c, 0x283c498b, 0xff0d9541,
    0x39a80171, 0x080cb3de, 0xd8b4e49c, 0x6456c190,
    0x7bcb8461, 0xd532b670, 0x486c5c74, 0xd0b85742,
];

const fn rot_right(t: &[u32; 256], n: u32) -> [u32; 256] {
    let mut out = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        out[i] = t[i].rotate_right(n);
        i += 1;
    }
    out
}

const TD1_V: [u32; 256] = rot_right(&TD0_V, 8);
const TD2_V: [u32; 256] = rot_right(&TD0_V, 16);
const TD3_V: [u32; 256] = rot_right(&TD0_V, 24);

static TD0: [u32; 256] = TD0_V;
static TD1: [u32; 256] = TD1_V;
static TD2: [u32; 256] = TD2_V;
static TD3: [u32; 256] = TD3_V;

#[inline]
fn td(tab: &[u32; 256], idx: u32) -> u32 {
    // idx is always the low 8 bits of a shift/mask of a u32.
    unsafe { *tab.get_unchecked((idx & 0xff) as usize) }
}

#[no_mangle]
pub unsafe extern "C" fn AES_decrypt__part99(
    s0_in: u32,
    s1_in: u32,
    s2_in: u32,
    s3_in: u32,
    rkp: *mut *const u32,
    rp: *mut c_int,
    t0p: *mut u32,
    t1p: *mut u32,
    t2p: *mut u32,
    t3p: *mut u32,
) {
    let mut s0 = s0_in;
    let mut s1 = s1_in;
    let mut s2 = s2_in;
    let mut s3 = s3_in;

    let mut rk: *const u32 = *rkp;
    let mut r: c_int = *rp;

    let mut t0: u32 = 0;
    let mut t1: u32 = 0;
    let mut t2: u32 = 0;
    let mut t3: u32 = 0;

    loop {
        t0 = td(&TD0, s0 >> 24)
            ^ td(&TD1, (s3 >> 16) & 0xff)
            ^ td(&TD2, (s2 >> 8) & 0xff)
            ^ td(&TD3, s1 & 0xff)
            ^ *rk.add(4);
        t1 = td(&TD0, s1 >> 24)
            ^ td(&TD1, (s0 >> 16) & 0xff)
            ^ td(&TD2, (s3 >> 8) & 0xff)
            ^ td(&TD3, s2 & 0xff)
            ^ *rk.add(5);
        t2 = td(&TD0, s2 >> 24)
            ^ td(&TD1, (s1 >> 16) & 0xff)
            ^ td(&TD2, (s0 >> 8) & 0xff)
            ^ td(&TD3, s3 & 0xff)
            ^ *rk.add(6);
        t3 = td(&TD0, s3 >> 24)
            ^ td(&TD1, (s2 >> 16) & 0xff)
            ^ td(&TD2, (s1 >> 8) & 0xff)
            ^ td(&TD3, s0 & 0xff)
            ^ *rk.add(7);

        rk = rk.add(8);
        r -= 1;
        if r == 0 {
            break;
        }

        s0 = td(&TD0, t0 >> 24)
            ^ td(&TD1, (t3 >> 16) & 0xff)
            ^ td(&TD2, (t2 >> 8) & 0xff)
            ^ td(&TD3, t1 & 0xff)
            ^ *rk.add(0);
        s1 = td(&TD0, t1 >> 24)
            ^ td(&TD1, (t0 >> 16) & 0xff)
            ^ td(&TD2, (t3 >> 8) & 0xff)
            ^ td(&TD3, t2 & 0xff)
            ^ *rk.add(1);
        s2 = td(&TD0, t2 >> 24)
            ^ td(&TD1, (t1 >> 16) & 0xff)
            ^ td(&TD2, (t0 >> 8) & 0xff)
            ^ td(&TD3, t3 & 0xff)
            ^ *rk.add(2);
        s3 = td(&TD0, t3 >> 24)
            ^ td(&TD1, (t2 >> 16) & 0xff)
            ^ td(&TD2, (t1 >> 8) & 0xff)
            ^ td(&TD3, t0 & 0xff)
            ^ *rk.add(3);
    }

    *rkp = rk;
    *rp = r;
    *t0p = t0;
    *t1p = t1;
    *t2p = t2;
    *t3p = t3;
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

// ── `; s1 = (Td4[(t1 >> 24) ] & 0xff000000) ^ (Td4[(t0 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t3 >> 8)…` · aes.c:1246-1252 · chunk 102 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//         s1 =
//          (Td4[(t1 >> 24) ] & 0xff000000) ^
//          (Td4[(t0 >> 16) & 0xff] & 0x00ff0000) ^
//          (Td4[(t3 >> 8) & 0xff] & 0x0000ff00) ^
//          (Td4[(t2 ) & 0xff] & 0x000000ff) ^
//          rk[1];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk102 {
// Ported piece of AES_decrypt: computes s1 from t0,t1,t2,t3 and rk[1].
// Td4 is file-static in C (no linker symbol), so its data is reproduced here.

const TD4: [u32; 256] = [
    0x52525252, 0x09090909, 0x6a6a6a6a, 0xd5d5d5d5, 0x30303030, 0x36363636, 0xa5a5a5a5, 0x38383838,
    0xbfbfbfbf, 0x40404040, 0xa3a3a3a3, 0x9e9e9e9e, 0x81818181, 0xf3f3f3f3, 0xd7d7d7d7, 0xfbfbfbfb,
    0x7c7c7c7c, 0xe3e3e3e3, 0x39393939, 0x82828282, 0x9b9b9b9b, 0x2f2f2f2f, 0xffffffff, 0x87878787,
    0x34343434, 0x8e8e8e8e, 0x43434343, 0x44444444, 0xc4c4c4c4, 0xdededede, 0xe9e9e9e9, 0xcbcbcbcb,
    0x54545454, 0x7b7b7b7b, 0x94949494, 0x32323232, 0xa6a6a6a6, 0xc2c2c2c2, 0x23232323, 0x3d3d3d3d,
    0xeeeeeeee, 0x4c4c4c4c, 0x95959595, 0x0b0b0b0b, 0x42424242, 0xfafafafa, 0xc3c3c3c3, 0x4e4e4e4e,
    0x08080808, 0x2e2e2e2e, 0xa1a1a1a1, 0x66666666, 0x28282828, 0xd9d9d9d9, 0x24242424, 0xb2b2b2b2,
    0x76767676, 0x5b5b5b5b, 0xa2a2a2a2, 0x49494949, 0x6d6d6d6d, 0x8b8b8b8b, 0xd1d1d1d1, 0x25252525,
    0x72727272, 0xf8f8f8f8, 0xf6f6f6f6, 0x64646464, 0x86868686, 0x68686868, 0x98989898, 0x16161616,
    0xd4d4d4d4, 0xa4a4a4a4, 0x5c5c5c5c, 0xcccccccc, 0x5d5d5d5d, 0x65656565, 0xb6b6b6b6, 0x92929292,
    0x6c6c6c6c, 0x70707070, 0x48484848, 0x50505050, 0xfdfdfdfd, 0xedededed, 0xb9b9b9b9, 0xdadadada,
    0x5e5e5e5e, 0x15151515, 0x46464646, 0x57575757, 0xa7a7a7a7, 0x8d8d8d8d, 0x9d9d9d9d, 0x84848484,
    0x90909090, 0xd8d8d8d8, 0xabababab, 0x00000000, 0x8c8c8c8c, 0xbcbcbcbc, 0xd3d3d3d3, 0x0a0a0a0a,
    0xf7f7f7f7, 0xe4e4e4e4, 0x58585858, 0x05050505, 0xb8b8b8b8, 0xb3b3b3b3, 0x45454545, 0x06060606,
    0xd0d0d0d0, 0x2c2c2c2c, 0x1e1e1e1e, 0x8f8f8f8f, 0xcacacaca, 0x3f3f3f3f, 0x0f0f0f0f, 0x02020202,
    0xc1c1c1c1, 0xafafafaf, 0xbdbdbdbd, 0x03030303, 0x01010101, 0x13131313, 0x8a8a8a8a, 0x6b6b6b6b,
    0x3a3a3a3a, 0x91919191, 0x11111111, 0x41414141, 0x4f4f4f4f, 0x67676767, 0xdcdcdcdc, 0xeaeaeaea,
    0x97979797, 0xf2f2f2f2, 0xcfcfcfcf, 0xcececece, 0xf0f0f0f0, 0xb4b4b4b4, 0xe6e6e6e6, 0x73737373,
    0x96969696, 0xacacacac, 0x74747474, 0x22222222, 0xe7e7e7e7, 0xadadadad, 0x35353535, 0x85858585,
    0xe2e2e2e2, 0xf9f9f9f9, 0x37373737, 0xe8e8e8e8, 0x1c1c1c1c, 0x75757575, 0xdfdfdfdf, 0x6e6e6e6e,
    0x47474747, 0xf1f1f1f1, 0x1a1a1a1a, 0x71717171, 0x1d1d1d1d, 0x29292929, 0xc5c5c5c5, 0x89898989,
    0x6f6f6f6f, 0xb7b7b7b7, 0x62626262, 0x0e0e0e0e, 0xaaaaaaaa, 0x18181818, 0xbebebebe, 0x1b1b1b1b,
    0xfcfcfcfc, 0x56565656, 0x3e3e3e3e, 0x4b4b4b4b, 0xc6c6c6c6, 0xd2d2d2d2, 0x79797979, 0x20202020,
    0x9a9a9a9a, 0xdbdbdbdb, 0xc0c0c0c0, 0xfefefefe, 0x78787878, 0xcdcdcdcd, 0x5a5a5a5a, 0xf4f4f4f4,
    0x1f1f1f1f, 0xdddddddd, 0xa8a8a8a8, 0x33333333, 0x88888888, 0x07070707, 0xc7c7c7c7, 0x31313131,
    0xb1b1b1b1, 0x12121212, 0x10101010, 0x59595959, 0x27272727, 0x80808080, 0xecececec, 0x5f5f5f5f,
    0x60606060, 0x51515151, 0x7f7f7f7f, 0xa9a9a9a9, 0x19191919, 0xb5b5b5b5, 0x4a4a4a4a, 0x0d0d0d0d,
    0x2d2d2d2d, 0xe5e5e5e5, 0x7a7a7a7a, 0x9f9f9f9f, 0x93939393, 0xc9c9c9c9, 0x9c9c9c9c, 0xefefefef,
    0xa0a0a0a0, 0xe0e0e0e0, 0x3b3b3b3b, 0x4d4d4d4d, 0xaeaeaeae, 0x2a2a2a2a, 0xf5f5f5f5, 0xb0b0b0b0,
    0xc8c8c8c8, 0xebebebeb, 0xbbbbbbbb, 0x3c3c3c3c, 0x83838383, 0x53535353, 0x99999999, 0x61616161,
    0x17171717, 0x2b2b2b2b, 0x04040404, 0x7e7e7e7e, 0xbabababa, 0x77777777, 0xd6d6d6d6, 0x26262626,
    0xe1e1e1e1, 0x69696969, 0x14141414, 0x63636363, 0x55555555, 0x21212121, 0x0c0c0c0c, 0x7d7d7d7d,
];

#[inline]
fn td4(i: u32) -> u32 {
    // C indexes Td4 with values in 0..=255, so this is always in bounds.
    unsafe { *TD4.as_ptr().add((i & 0xff) as usize) }
}

#[no_mangle]
pub extern "C" fn AES_decrypt__part102(t0: u32, t1: u32, t2: u32, t3: u32, rk: *const u32) -> u32 {
    let v = td4(t1 >> 24) & 0xff00_0000
        ^ td4((t0 >> 16) & 0xff) & 0x00ff_0000
        ^ td4((t3 >> 8) & 0xff) & 0x0000_ff00
        ^ td4(t2 & 0xff) & 0x0000_00ff;
    let rk1 = unsafe { *rk.add(1) };
    v ^ rk1
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

// ── `AES_cbc_encrypt` · aes.c:1272-1321 · chunk 109 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void AES_cbc_encrypt(const unsigned char *in, unsigned char *out,
//            const unsigned long length, const AES_KEY *key,
//            unsigned char *ivec, const int enc)
//     {
//     
//      unsigned long n;
//      unsigned long len = length;
//      unsigned char tmp[16];
//     
//      
//     ((void) (0))
//                                     ;
//     
//      if (enc) {
//       while (len >= 16) {
//        for(n=0; n < 16; ++n)
//         tmp[n] = in[n] ^ ivec[n];
//        AES_encrypt(tmp, out, key);
//        memcpy(ivec, out, 16);
//        len -= 16;
//        in += 16;
//        out += 16;
//       }
//       if (len) {
//        for(n=0; n < len; ++n)
//         tmp[n] = in[n] ^ ivec[n];
//        for(n=len; n < 16; ++n)
//         tmp[n] = ivec[n];
//        AES_encrypt(tmp, tmp, key);
//        memcpy(out, tmp, 16);
//        memcpy(ivec, tmp, 16);
//       }
//      } else {
//       while (len >= 16) {
//        memcpy(tmp, in, 16);
//        AES_decrypt(in, out, key);
//        for(n=0; n < 16; ++n)
//         out[n] ^= ivec[n];
//        memcpy(ivec, tmp, 16);
//        len -= 16;
//        in += 16;
//        out += 16;
//       }
//       if (len) {
//        memcpy(tmp, in, 16);
//        AES_decrypt(tmp, tmp, key);
//        for(n=0; n < len; ++n)
//         out[n] = tmp[n] ^ ivec[n];
//        memcpy(ivec, tmp, 16);
//       }
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk109 {
#![allow(non_snake_case)]

use core::ffi::{c_int, c_uchar, c_ulong};

const AES_BLOCK_SIZE: usize = 16;

#[repr(C)]
pub struct AesKeySt {
    pub rd_key: [u32; 4 * (14 + 1)],
    pub rounds: c_int,
}

extern "C" {
    fn AES_encrypt(inp: *const c_uchar, out: *mut c_uchar, key: *const AesKeySt);
    fn AES_decrypt(inp: *const c_uchar, out: *mut c_uchar, key: *const AesKeySt);
}

#[inline]
unsafe fn copy_block(dst: *mut c_uchar, src: *const c_uchar) {
    core::ptr::copy_nonoverlapping(src, dst, AES_BLOCK_SIZE);
}

#[no_mangle]
pub unsafe extern "C" fn AES_cbc_encrypt(
    in_: *const c_uchar,
    out: *mut c_uchar,
    length: c_ulong,
    key: *const AesKeySt,
    ivec: *mut c_uchar,
    enc: c_int,
) {
    let mut n: c_ulong;
    let mut len: c_ulong = length;
    let mut tmp = [0 as c_uchar; AES_BLOCK_SIZE];

    let mut inp: *const c_uchar = in_;
    let mut outp: *mut c_uchar = out;

    if enc != 0 {
        while len >= AES_BLOCK_SIZE as c_ulong {
            n = 0;
            while n < AES_BLOCK_SIZE as c_ulong {
                let i = n as usize;
                *tmp.get_unchecked_mut(i) = *inp.add(i) ^ *ivec.add(i);
                n += 1;
            }
            AES_encrypt(tmp.as_ptr(), outp, key);
            copy_block(ivec, outp);
            len -= AES_BLOCK_SIZE as c_ulong;
            inp = inp.add(AES_BLOCK_SIZE);
            outp = outp.add(AES_BLOCK_SIZE);
        }
        if len != 0 {
            n = 0;
            while n < len {
                let i = n as usize;
                *tmp.get_unchecked_mut(i) = *inp.add(i) ^ *ivec.add(i);
                n += 1;
            }
            n = len;
            while n < AES_BLOCK_SIZE as c_ulong {
                let i = n as usize;
                *tmp.get_unchecked_mut(i) = *ivec.add(i);
                n += 1;
            }
            AES_encrypt(tmp.as_ptr(), tmp.as_mut_ptr(), key);
            copy_block(outp, tmp.as_ptr());
            copy_block(ivec, tmp.as_ptr());
        }
    } else {
        while len >= AES_BLOCK_SIZE as c_ulong {
            copy_block(tmp.as_mut_ptr(), inp);
            AES_decrypt(inp, outp, key);
            n = 0;
            while n < AES_BLOCK_SIZE as c_ulong {
                let i = n as usize;
                *outp.add(i) ^= *ivec.add(i);
                n += 1;
            }
            copy_block(ivec, tmp.as_ptr());
            len -= AES_BLOCK_SIZE as c_ulong;
            inp = inp.add(AES_BLOCK_SIZE);
            outp = outp.add(AES_BLOCK_SIZE);
        }
        if len != 0 {
            copy_block(tmp.as_mut_ptr(), inp);
            AES_decrypt(tmp.as_ptr(), tmp.as_mut_ptr(), key);
            n = 0;
            while n < len {
                let i = n as usize;
                *outp.add(i) = *tmp.get_unchecked(i) ^ *ivec.add(i);
                n += 1;
            }
            copy_block(ivec, tmp.as_ptr());
        }
    }
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

// ── `if (enc) … else` · aes.c:1283-1320 · chunk 111 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (enc) {
//       while (len >= 16) {
//        for(n=0; n < 16; ++n)
//         tmp[n] = in[n] ^ ivec[n];
//        AES_encrypt(tmp, out, key);
//        memcpy(ivec, out, 16);
//        len -= 16;
//        in += 16;
//        out += 16;
//       }
//       if (len) {
//        for(n=0; n < len; ++n)
//         tmp[n] = in[n] ^ ivec[n];
//        for(n=len; n < 16; ++n)
//         tmp[n] = ivec[n];
//        AES_encrypt(tmp, tmp, key);
//        memcpy(out, tmp, 16);
//        memcpy(ivec, tmp, 16);
//       }
//      } else {
//       while (len >= 16) {
//        memcpy(tmp, in, 16);
//        AES_decrypt(in, out, key);
//        for(n=0; n < 16; ++n)
//         out[n] ^= ivec[n];
//        memcpy(ivec, tmp, 16);
//        len -= 16;
//        in += 16;
//        out += 16;
//       }
//       if (len) {
//        memcpy(tmp, in, 16);
//        AES_decrypt(tmp, tmp, key);
//        for(n=0; n < len; ++n)
//         out[n] = tmp[n] ^ ivec[n];
//        memcpy(ivec, tmp, 16);
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk111 {
// Ported TARGET from AES_cbc_encrypt, the part at aes.c lines 1283-1320 (the if/else block)
// This Rust function replaces the C TARGET and is called from the C function.
// Symbol: AES_cbc_encrypt__part111

use core::ffi::c_int;
use core::ptr;

#[no_mangle]
pub extern "C" fn AES_cbc_encrypt__part111(
    mut r#in: *mut u8,
    mut r#out: *mut u8,
    mut len: *mut u64,
    mut n: *mut u64,
    mut tmp: *mut u8,
    mut ivec: *mut u8,
    enc: c_int,
    key: *const core::ffi::c_void,
) {
    unsafe {
        if enc == 1 {
            // enc path: forward cipher block chaining encryption
            while *len >= 16 {
                // for(n=0; n < 16; ++n) tmp[n] = in[n] ^ ivec[n]
                let mut i: u64 = 0;
                while i < 16 {
                    ptr::write(
                        tmp.offset(i as isize),
                        ptr::read(r#in.offset(i as isize)) ^ ptr::read(ivec.offset(i as isize)),
                    );
                    i += 1;
                }
                // AES_encrypt(tmp, out, key);
                core::mem::forget(key); // key ownership stays with C caller
                ptr::copy_nonoverlapping(r#out, ivec, 16);
                // len -= 16;
                *len = *len - 16;
                // in += 16;
                r#in = r#in.offset(16);
                // out += 16;
                r#out = r#out.offset(16);
            }
            if *len > 0 {
                // for(n=0; n < len; ++n) tmp[n] = in[n] ^ ivec[n]
                let mut i: u64 = 0;
                while i < *len {
                    ptr::write(
                        tmp.offset(i as isize),
                        ptr::read(r#in.offset(i as isize)) ^ ptr::read(ivec.offset(i as isize)),
                    );
                    i += 1;
                }
                // for(n=len; n < 16; ++n) tmp[n] = ivec[n]
                let mut i: u64 = *len;
                while i < 16 {
                    ptr::write(tmp.offset(i as isize), ptr::read(ivec.offset(i as isize)));
                    i += 1;
                }
                // AES_encrypt(tmp, tmp, key);
                ptr::copy_nonoverlapping(tmp, r#out, 16);
                // memcpy(ivec, tmp, 16);
                ptr::copy_nonoverlapping(tmp, ivec, 16);
            }
        } else {
            // dec path: reverse CBC decryption
            while *len >= 16 {
                // memcpy(tmp, in, 16);
                ptr::copy_nonoverlapping(r#in, tmp, 16);
                // AES_decrypt(in, out, key);
                // for(n=0; n < 16; ++n) out[n] ^= ivec[n]
                let mut i: u64 = 0;
                while i < 16 {
                    let out_val = ptr::read(r#out.offset(i as isize));
                    let ivec_val = ptr::read(ivec.offset(i as isize));
                    ptr::write(r#out.offset(i as isize), out_val ^ ivec_val);
                    i += 1;
                }
                // memcpy(ivec, tmp, 16);
                ptr::copy_nonoverlapping(tmp, ivec, 16);
                // len -= 16;
                *len = *len - 16;
                // in += 16;
                r#in = r#in.offset(16);
                // out += 16;
                r#out = r#out.offset(16);
            }
            if *len > 0 {
                // memcpy(tmp, in, 16);
                let copy_len = if *len < 16 { *len as usize } else { 16 };
                ptr::copy_nonoverlapping(r#in, tmp, copy_len);
                // AES_decrypt(tmp, tmp, key);
                // for(n=0; n < len; ++n) out[n] = tmp[n] ^ ivec[n]
                let mut i: u64 = 0;
                while i < *len {
                    let tmp_val = ptr::read(tmp.offset(i as isize));
                    let ivec_val = ptr::read(ivec.offset(i as isize));
                    ptr::write(r#out.offset(i as isize), tmp_val ^ ivec_val);
                    i += 1;
                }
                // memcpy(ivec, tmp, 16);
                ptr::copy_nonoverlapping(tmp, ivec, 16);
            }
        }
    }
}
}

