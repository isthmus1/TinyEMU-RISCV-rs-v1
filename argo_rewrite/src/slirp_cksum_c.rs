//! From `slirp/cksum.c`.

// ── `cksum` · slirp/cksum.c:48-139 · chunk 2267 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int cksum(struct mbuf *m, int len)
//     {
//      register uint16_t *w;
//      register int sum = 0;
//      register int mlen = 0;
//      int byte_swapped = 0;
//     
//      union {
//       uint8_t c[2];
//       uint16_t s;
//      } s_util;
//      union {
//       uint16_t s[2];
//       uint32_t l;
//      } l_util;
//     
//      if (m->m_hdr.mh_len == 0)
//         goto cont;
//      w = ((uint16_t *)(m)->m_hdr.mh_data);
//     
//      mlen = m->m_hdr.mh_len;
//     
//      if (len < mlen)
//         mlen = len;
//     
//     
//     
//     
//     
//     
//      if ((1 & (long) w) && (mlen > 0)) {
//       {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//       sum <<= 8;
//       s_util.c[0] = *(uint8_t *)w;
//       w = (uint16_t *)((int8_t *)w + 1);
//       mlen--;
//       byte_swapped = 1;
//      }
//     
//     
//     
//     
//      while ((mlen -= 32) >= 0) {
//       sum += w[0]; sum += w[1]; sum += w[2]; sum += w[3];
//       sum += w[4]; sum += w[5]; sum += w[6]; sum += w[7];
//       sum += w[8]; sum += w[9]; sum += w[10]; sum += w[11];
//       sum += w[12]; sum += w[13]; sum += w[14]; sum += w[15];
//       w += 16;
//      }
//      mlen += 32;
//      while ((mlen -= 8) >= 0) {
//       sum += w[0]; sum += w[1]; sum += w[2]; sum += w[3];
//       w += 4;
//      }
//      mlen += 8;
//      if (mlen == 0 && byte_swapped == 0)
//         goto cont;
//      {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//      while ((mlen -= 2) >= 0) {
//       sum += *w++;
//      }
//     
//      if (byte_swapped) {
//       {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//       sum <<= 8;
//       if (mlen == -1) {
//        s_util.c[1] = *(uint8_t *)w;
//        sum += s_util.s;
//        mlen = 0;
//       } else
//     
//          mlen = -1;
//      } else if (mlen == -1)
//         s_util.c[0] = *(uint8_t *)w;
//     
//     cont:
//     
//     
//     
//     
//     
//     
//      if (mlen == -1) {
//     
//     
//     
//       s_util.c[1] = 0;
//       sum += s_util.s;
//      }
//      {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//      return (~sum & 0xffff);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2267 {
// Port of slirp/cksum.c cksum() to Rust.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MBufHdr {
    pub mh_next: *mut c_void,
    pub mh_prev: *mut c_void,
    pub mh_nextpkt: *mut c_void,
    pub mh_prevpkt: *mut c_void,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MBufHdr,
}

#[inline]
fn rd16(w: *mut u16, i: usize) -> u16 {
    unsafe { (w.add(i) as *const u16).read_unaligned() }
}

#[inline]
fn rdu16(w: *mut u16) -> u16 {
    unsafe { (w as *const u16).read_unaligned() }
}

#[inline]
fn rd8(w: *mut u16) -> u8 {
    unsafe { *(w as *const u8) }
}

#[inline]
fn reduce(sum: &mut c_int) {
    // REDUCE: l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; ADDCARRY(sum);
    let l: u32 = *sum as u32;
    let mut s: c_int = (l & 0xffff) as c_int + ((l >> 16) & 0xffff) as c_int;
    if s > 65535 {
        s -= 65535;
    }
    *sum = s;
}

#[no_mangle]
pub extern "C" fn cksum(m: *mut MBuf, len: c_int) -> c_int {
    let mut w: *mut u16 = core::ptr::null_mut();
    let mut sum: c_int = 0;
    let mut mlen: c_int = 0;
    let mut byte_swapped: c_int = 0;
    let mut s_util_c: [u8; 2] = [0, 0];

    let m_len = unsafe { (*m).m_hdr.mh_len };
    if m_len != 0 {
        w = unsafe { (*m).m_hdr.mh_data as *mut u16 };
        mlen = m_len;
        if len < mlen {
            mlen = len;
        }

        // Force to even boundary.
        if (1 & (w as isize)) != 0 && mlen > 0 {
            reduce(&mut sum);
            sum <<= 8;
            s_util_c[0] = rd8(w);
            w = ((w as *mut u8).wrapping_add(1)) as *mut u16;
            mlen -= 1;
            byte_swapped = 1;
        }

        // Unrolled loop.
        loop {
            mlen -= 32;
            if mlen < 0 {
                break;
            }
            sum = sum.wrapping_add(rd16(w, 0) as c_int);
            sum = sum.wrapping_add(rd16(w, 1) as c_int);
            sum = sum.wrapping_add(rd16(w, 2) as c_int);
            sum = sum.wrapping_add(rd16(w, 3) as c_int);
            sum = sum.wrapping_add(rd16(w, 4) as c_int);
            sum = sum.wrapping_add(rd16(w, 5) as c_int);
            sum = sum.wrapping_add(rd16(w, 6) as c_int);
            sum = sum.wrapping_add(rd16(w, 7) as c_int);
            sum = sum.wrapping_add(rd16(w, 8) as c_int);
            sum = sum.wrapping_add(rd16(w, 9) as c_int);
            sum = sum.wrapping_add(rd16(w, 10) as c_int);
            sum = sum.wrapping_add(rd16(w, 11) as c_int);
            sum = sum.wrapping_add(rd16(w, 12) as c_int);
            sum = sum.wrapping_add(rd16(w, 13) as c_int);
            sum = sum.wrapping_add(rd16(w, 14) as c_int);
            sum = sum.wrapping_add(rd16(w, 15) as c_int);
            w = w.wrapping_add(16);
        }
        mlen += 32;
        loop {
            mlen -= 8;
            if mlen < 0 {
                break;
            }
            sum = sum.wrapping_add(rd16(w, 0) as c_int);
            sum = sum.wrapping_add(rd16(w, 1) as c_int);
            sum = sum.wrapping_add(rd16(w, 2) as c_int);
            sum = sum.wrapping_add(rd16(w, 3) as c_int);
            w = w.wrapping_add(4);
        }
        mlen += 8;

        if !(mlen == 0 && byte_swapped == 0) {
            reduce(&mut sum);
            loop {
                mlen -= 2;
                if mlen < 0 {
                    break;
                }
                sum = sum.wrapping_add(rdu16(w) as c_int);
                w = w.wrapping_add(1);
            }

            if byte_swapped != 0 {
                reduce(&mut sum);
                sum <<= 8;
                if mlen == -1 {
                    s_util_c[1] = rd8(w);
                    sum = sum.wrapping_add(u16::from_le_bytes(s_util_c) as c_int);
                    mlen = 0;
                } else {
                    mlen = -1;
                }
            } else if mlen == -1 {
                s_util_c[0] = rd8(w);
            }
        }
    }

    // cont:
    if mlen == -1 {
        // The last mbuf has odd # of bytes; append the byte per host endianness.
        s_util_c[1] = 0;
        sum = sum.wrapping_add(u16::from_le_bytes(s_util_c) as c_int);
    }
    reduce(&mut sum);
    (!sum & 0xffff) as c_int
}
}

// ── `if (m->m_hdr.mh_len == 0)` · slirp/cksum.c:64-65 · chunk 2268 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_len == 0)
//         goto cont;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2268 {
// Port of the "empty mbuf" early-exit test at the top of cksum().
// Returns nonzero when m->m_hdr.mh_len == 0 (the C code then jumps to `cont`).
#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut u8,
    pub mh_data: *mut u8,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
}

#[no_mangle]
pub extern "C" fn cksum__part2268(m: *mut mbuf) -> c_int {
    if m.is_null() {
        return 0;
    }
    let len = unsafe { (*m).m_hdr.mh_len };
    if len == 0 {
        1
    } else {
        0
    }
}
}

// ── `w = ((uint16_t *)(m)->m_hdr.mh_data); mlen = m->m_hdr.mh_len;` · slirp/cksum.c:66-68 · chunk 2269 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     w = ((uint16_t *)(m)->m_hdr.mh_data);
//     
//      mlen = m->m_hdr.mh_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2269 {
// Port of a piece of slirp's cksum(): initialize the 16-bit word pointer
// from the mbuf header data and hand out the mbuf header length.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
}

#[no_mangle]
pub extern "C" fn cksum__part2269(m: *mut Mbuf, mlen_out: *mut c_int) -> *mut u16 {
    unsafe {
        let hdr = &(*m).m_hdr;
        *mlen_out = hdr.mh_len;
        hdr.mh_data as *mut u16
    }
}
}

// ── `if (len < mlen)` · slirp/cksum.c:70-71 · chunk 2270 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len < mlen)
//         mlen = len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2270 {
use core::ffi::c_int;

// int cksum(struct mbuf *m, int len) -- lines 70-71:
//     if (len < mlen)
//         mlen = len;
// mlen is a reassigned local, so the caller stores the returned value back.
#[no_mangle]
pub extern "C" fn cksum__part2270(len: c_int, mlen: c_int) -> c_int {
    let mut mlen = mlen;
    if len < mlen {
        mlen = len;
    }
    mlen
}
}

// ── `if (1 & (long) w) && (mlen > 0)` · slirp/cksum.c:78-85 · chunk 2271 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if ((1 & (long) w) && (mlen > 0)) {
//       {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//       sum <<= 8;
//       s_util.c[0] = *(uint8_t *)w;
//       w = (uint16_t *)((int8_t *)w + 1);
//       mlen--;
//       byte_swapped = 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2271 {
// Piece of cksum(): force the checksum pointer onto an even boundary.
use core::ffi::{c_int, c_long, c_uchar};

// Mirrors the C locals that the piece reads and writes.
#[repr(C)]
pub struct CksumIo {
    pub w: *mut u16,
    pub sum: c_int,
    pub mlen: c_int,
    pub byte_swapped: c_int,
}

// Mirrors the C local union { uint16_t s[2]; uint32_t l; } l_util
#[repr(C)]
union LUtil {
    s: [u16; 2],
    l: u32,
}

// Silences unused warnings without changing layout/behaviour.
#[allow(dead_code)]
const _LAYOUT_CHECK: usize = core::mem::size_of::<CksumIo>();

#[no_mangle]
pub extern "C" fn cksum__part2271(io: *mut CksumIo, s_c: *mut c_uchar) {
    if io.is_null() || s_c.is_null() {
        return;
    }

    unsafe {
        let state = &mut *io;
        let wp: *mut u16 = state.w;
        let cur_mlen: c_int = state.mlen;

        // if ((1 & (long) w) && (mlen > 0))
        if ((wp as c_long) & 1) != 0 && cur_mlen > 0 {
            // REDUCE: { l_util.l = sum; sum = l_util.s[0] + l_util.s[1];
            //           ADDCARRY(sum); }
            let parts = LUtil { l: state.sum as u32 }.s;
            let mut reduced: c_int = parts[0] as c_int + parts[1] as c_int;
            if reduced > 65535 {
                reduced -= 65535;
            }
            state.sum = reduced;

            // sum <<= 8;
            state.sum = state.sum.wrapping_shl(8);

            // s_util.c[0] = *(uint8_t *)w;
            *s_c = *(wp as *const c_uchar);

            // w = (uint16_t *)((int8_t *)w + 1);
            state.w = (wp as *mut i8).add(1) as *mut u16;

            // mlen--;
            state.mlen = cur_mlen - 1;

            // byte_swapped = 1;
            state.byte_swapped = 1;
        }
    }
}
}

// ── `while (mlen -= 32) >= 0)` · slirp/cksum.c:90-96 · chunk 2272 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((mlen -= 32) >= 0) {
//       sum += w[0]; sum += w[1]; sum += w[2]; sum += w[3];
//       sum += w[4]; sum += w[5]; sum += w[6]; sum += w[7];
//       sum += w[8]; sum += w[9]; sum += w[10]; sum += w[11];
//       sum += w[12]; sum += w[13]; sum += w[14]; sum += w[15];
//       w += 16;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2272 {
use core::ffi::c_int;
use core::ptr::read_unaligned;

// Port of the 32-byte unrolled checksum accumulation loop from cksum().
// Reads/writes state through pointers because the C locals are `register`.
#[no_mangle]
pub extern "C" fn cksum__part2272(sum: *mut c_int, w: *mut *mut u16, mlen: *mut c_int) {
    if sum.is_null() || w.is_null() || mlen.is_null() {
        return;
    }
    unsafe {
        let mut s: c_int = *sum;
        let mut wp: *mut u16 = *w;
        let mut m: c_int = *mlen;

        loop {
            m = m.wrapping_sub(32);
            if m < 0 {
                break;
            }
            let mut i: usize = 0;
            while i < 16 {
                s = s.wrapping_add(read_unaligned(wp.add(i)) as c_int);
                i += 1;
            }
            wp = wp.add(16);
        }

        *sum = s;
        *w = wp;
        *mlen = m;
    }
}
}

// ── `mlen += 32;` · slirp/cksum.c:97-97 · chunk 2273 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     mlen += 32;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2273 {
// Ported piece of cksum: `mlen += 32;`
// mlen is declared `register int` in the C function, so it cannot be passed
// by address; the new value is returned instead.

#[no_mangle]
pub extern "C" fn cksum__part2273(mlen: core::ffi::c_int) -> core::ffi::c_int {
    mlen.wrapping_add(32)
}
}

// ── `while (mlen -= 8) >= 0)` · slirp/cksum.c:98-101 · chunk 2274 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((mlen -= 8) >= 0) {
//       sum += w[0]; sum += w[1]; sum += w[2]; sum += w[3];
//       w += 4;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2274 {
// Port of the 8-byte-at-a-time word summing loop in cksum().
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2274(sum: *mut c_int, w: *mut *mut u16, mlen: *mut c_int) {
    unsafe {
        let mut s: c_int = *sum;
        let mut p: *mut u16 = *w;
        let mut m: c_int = *mlen;

        loop {
            m = m.wrapping_sub(8);
            if m < 0 {
                break;
            }
            s = s.wrapping_add(*p as c_int);
            s = s.wrapping_add(*p.add(1) as c_int);
            s = s.wrapping_add(*p.add(2) as c_int);
            s = s.wrapping_add(*p.add(3) as c_int);
            p = p.add(4);
        }

        *sum = s;
        *w = p;
        *mlen = m;
    }
}
}

// ── `mlen += 8;` · slirp/cksum.c:102-102 · chunk 2275 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     mlen += 8;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2275 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2275(mlen: c_int) -> c_int {
    mlen.wrapping_add(8)
}
}

// ── `if (mlen == 0 && byte_swapped == 0)` · slirp/cksum.c:103-104 · chunk 2276 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mlen == 0 && byte_swapped == 0)
//         goto cont;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2276 {
// Port of the `goto cont;` guard in cksum: returns nonzero when the checksum
// loops finished on an even boundary without byte swapping, i.e. when the
// original code would have jumped to the `cont:` label.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2276(mlen: c_int, byte_swapped: c_int) -> c_int {
    if mlen == 0 && byte_swapped == 0 {
        1
    } else {
        0
    }
}
}

// ── `{ … }` · slirp/cksum.c:105-105 · chunk 2277 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2277 {
use core::ffi::c_int;

// Port of the one's-complement fold snippet used inside cksum():
//   { l_util.l = sum; sum = l_util.s[0] + l_util.s[1];
//     (void)(sum > 65535 ? sum -= 65535 : sum); }
// l_util is a union of uint32_t l and uint16_t s[2] (little-endian halves).
#[no_mangle]
pub extern "C" fn cksum__part2277(sum: c_int) -> c_int {
    let l: u32 = sum as u32;
    let s0: c_int = (l & 0xffff) as c_int;
    let s1: c_int = (l >> 16) as c_int;
    let mut acc: c_int = s0 + s1;
    if acc > 65535 {
        acc -= 65535;
    }
    acc
}
}

// ── `;` · slirp/cksum.c:105-105 · chunk 2278 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2278 {
// Portion of cksum(): force-to-even-boundary prologue.
//   REDUCE; sum <<= 8; s_util.c[0] = *(uint8_t *)w; w = (int8_t*)w + 1;
//   mlen--; byte_swapped = 1;
// The C caller copies its `register` locals into addressable temporaries,
// so everything that must be updated is passed by pointer; `byte_swapped`
// is unconditionally set to 1 by the caller.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2278(
    sum: *mut c_int,
    w: *mut *mut u16,
    mlen: *mut c_int,
    s_c: *mut u8,
) {
    if sum.is_null() || w.is_null() || mlen.is_null() || s_c.is_null() {
        return;
    }
    unsafe {
        // REDUCE: l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; ADDCARRY
        let mut s: c_int = *sum;
        let l: u32 = s as u32;
        s = ((l & 0xffff) as c_int) + ((l >> 16) as c_int);
        if s > 65535 {
            s -= 65535;
        }
        // sum <<= 8;
        s = s.wrapping_shl(8);
        // s_util.c[0] = *(uint8_t *)w;
        let wp: *mut u16 = *w;
        *s_c = *(wp as *const u8);
        // w = (uint16_t *)((int8_t *)w + 1);
        *w = (wp as *const u8).add(1) as *mut u16;
        // mlen--;
        let m: c_int = *mlen;
        *mlen = m - 1;
        *sum = s;
    }
}
}

// ── `while (mlen -= 2) >= 0)` · slirp/cksum.c:106-108 · chunk 2279 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((mlen -= 2) >= 0) {
//       sum += *w++;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2279 {
// Port of the 16-bit word accumulation loop from slirp cksum().
// The C locals (mlen, sum, w) are `register` variables, so their addresses
// cannot be taken: values come in by value and results go out through a
// #[repr(C)] result struct.

use core::ffi::c_int;

#[repr(C)]
pub struct CksumPart2279Res {
    pub sum: c_int,
    pub mlen: c_int,
    pub w: *mut u16,
}

#[no_mangle]
pub extern "C" fn cksum__part2279(
    mut mlen: c_int,
    mut sum: c_int,
    mut w: *mut u16,
    out: *mut CksumPart2279Res,
) {
    loop {
        mlen = mlen.wrapping_sub(2);
        if mlen < 0 {
            break;
        }
        unsafe {
            if !w.is_null() {
                sum = sum.wrapping_add(*w as c_int);
            }
            w = w.wrapping_add(1);
        }
    }

    unsafe {
        if !out.is_null() {
            (*out).sum = sum;
            (*out).mlen = mlen;
            (*out).w = w;
        }
    }
}
}

// ── `if (byte_swapped) … else` · slirp/cksum.c:110-121 · chunk 2280 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (byte_swapped) {
//       {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);};
//       sum <<= 8;
//       if (mlen == -1) {
//        s_util.c[1] = *(uint8_t *)w;
//        sum += s_util.s;
//        mlen = 0;
//       } else
//     
//          mlen = -1;
//      } else if (mlen == -1)
//         s_util.c[0] = *(uint8_t *)w;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2280 {
// Port of the byte-swapped tail handling of cksum() (slirp/cksum.c).
// sum, mlen and w are `register` locals in the C function, so their addresses
// cannot be taken: values come in by value and modified results go out through
// plain out-parameters. byte_swapped and s_util are ordinary locals, so they
// are passed by pointer (s_util is read-modify-written across the call).

use core::ffi::c_int;

// Mirror of the C anonymous union { uint8_t c[2]; uint16_t s; } l_util as used
// for the ones-complement fold (l_util.l read back as l_util.s[0] + l_util.s[1]).
#[repr(C)]
union LUtil {
    s: [u16; 2],
    l: u32,
}

#[inline]
fn fold(sum: c_int) -> c_int {
    let lu = LUtil { l: sum as u32 };
    // Reading the array field of a union holding the u32 gives the native
    // endianness view, exactly like the C l_util union.
    let parts = unsafe { lu.s };
    let mut s = parts[0] as c_int + parts[1] as c_int;
    if s > 65535 {
        s -= 65535;
    }
    s
}

#[no_mangle]
pub extern "C" fn cksum__part2280(
    sum_in: c_int,
    mlen_in: c_int,
    w: *mut u16,
    byte_swapped: c_int,
    s_util: *mut u8,
    out_sum: *mut c_int,
    out_mlen: *mut c_int,
) {
    let mut sum = sum_in;
    let mut mlen = mlen_in;

    unsafe {
        if byte_swapped != 0 {
            sum = fold(sum);
            sum = sum.wrapping_shl(8);
            if mlen == -1 {
                *s_util.add(1) = *(w as *const u8);
                // s_util.s in native byte order
                let sval = u16::from_ne_bytes([*s_util, *s_util.add(1)]) as c_int;
                sum = sum.wrapping_add(sval);
                mlen = 0;
            } else {
                mlen = -1;
            }
        } else if mlen == -1 {
            *s_util = *(w as *const u8);
        }

        *out_sum = sum;
        *out_mlen = mlen;
    }
}
}

// ── `cont: if (mlen == -1) { s_util.c[1] = 0; sum += s_util.s; }` · slirp/cksum.c:123-136 · chunk 2281 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     cont:
//     
//     
//     
//     
//     
//     
//      if (mlen == -1) {
//     
//     
//     
//       s_util.c[1] = 0;
//       sum += s_util.s;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2281 {
// Port of the "cont:" tail of cksum(): fold the trailing odd byte into the sum.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2281(mlen: c_int, sum: c_int, s_util: *mut u16) -> c_int {
    let mut sum = sum;
    if mlen == -1 {
        unsafe {
            // s_util.c[1] = 0;  (byte-wise, endian independent)
            let p = s_util as *mut u8;
            *p.add(1) = 0;
            // sum += s_util.s;
            sum = sum.wrapping_add(*s_util as c_int);
        }
    }
    sum
}
}

// ── `{ … }` · slirp/cksum.c:137-137 · chunk 2282 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2282 {
// Port of the REDUCE macro instance at the end of cksum():
//   {l_util.l = sum; sum = l_util.s[0] + l_util.s[1]; (void)(sum > 65535 ? sum -= 65535 : sum);}
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cksum__part2282(sum: c_int) -> c_int {
    let l: u32 = sum as u32;
    // Mirror the l_util union { uint16_t s[2]; uint32_t l; } using native endianness.
    let bytes = l.to_ne_bytes();
    let s0 = u16::from_ne_bytes([bytes[0], bytes[1]]) as c_int;
    let s1 = u16::from_ne_bytes([bytes[2], bytes[3]]) as c_int;
    let mut acc: c_int = s0 + s1;
    if acc > 65535 {
        acc -= 65535;
    }
    acc
}
}

// ── `; return (~sum & 0xffff);` · slirp/cksum.c:137-138 · chunk 2283 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      return (~sum & 0xffff);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2283 {
// Portion of slirp's cksum(): final one's-complement fold and result return.
// `;(void)REDUCE` behaviour is folded into the caller side; this piece only
// computes the returned checksum value from the accumulated sum.

#[no_mangle]
pub extern "C" fn cksum__part2283(sum: core::ffi::c_int) -> core::ffi::c_int {
    // return (~sum & 0xffff);
    (!sum) & 0xffff
}
}

