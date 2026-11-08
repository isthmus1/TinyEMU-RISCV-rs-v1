//! From `slirp/sbuf.c`.

// ── `sbfree` · slirp/sbuf.c:12-16 · chunk 2459 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sbfree(struct sbuf *sb)
//     {
//      free(sb->sb_data);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2459 {
// Port of slirp/sbuf.c sbfree()
use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn sbfree(sb: *mut Sbuf) {
    if sb.is_null() {
        return;
    }
    free((*sb).sb_data as *mut core::ffi::c_void);
}
}

// ── `free(sb->sb_data);` · slirp/sbuf.c:15-15 · chunk 2460 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(sb->sb_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2460 {
use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

#[no_mangle]
pub extern "C" fn sbfree__part2460(sb: *mut Sbuf) {
    unsafe {
        let data = (*sb).sb_data;
        free(data as *mut core::ffi::c_void);
    }
}
}

// ── `sbdrop` · slirp/sbuf.c:18-32 · chunk 2461 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sbdrop(struct sbuf *sb, int num)
//     {
//     
//     
//     
//     
//      if(num > sb->sb_cc)
//       num = sb->sb_cc;
//      sb->sb_cc -= num;
//      sb->sb_rptr += num;
//      if(sb->sb_rptr >= sb->sb_data + sb->sb_datalen)
//       sb->sb_rptr -= sb->sb_datalen;
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2461 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn sbdrop(sb: *mut Sbuf, num: c_int) {
    if sb.is_null() {
        return;
    }
    let s = &mut *sb;
    let mut num = num;
    // Comparison is int vs unsigned int in C: num is converted to unsigned.
    if (num as u32) > s.sb_cc {
        num = s.sb_cc as c_int;
    }
    s.sb_cc = s.sb_cc.wrapping_sub(num as u32);
    // Pointer + int (sign-extended) arithmetic, done with wrapping semantics.
    s.sb_rptr = s.sb_rptr.wrapping_offset(num as isize);
    let end = (s.sb_data as usize).wrapping_add(s.sb_datalen as usize);
    if (s.sb_rptr as usize) >= end {
        s.sb_rptr = ((s.sb_rptr as usize).wrapping_sub(s.sb_datalen as usize)) as *mut c_char;
    }
}
}

// ── `if (num > sb->sb_cc)` · slirp/sbuf.c:25-26 · chunk 2462 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(num > sb->sb_cc)
//       num = sb->sb_cc;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2462 {
// Port of the head of sbdrop(): clamp `num` to the buffer's byte count.
// C: if(num > sb->sb_cc) num = sb->sb_cc;
// num is int, sb_cc is u_int, so the comparison happens in unsigned int.

use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn sbdrop__part2462(num: c_int, sb_cc: c_uint) -> c_int {
    if (num as c_uint) > sb_cc {
        sb_cc as c_int
    } else {
        num
    }
}
}

// ── `sb->sb_cc -= num; sb->sb_rptr += num;` · slirp/sbuf.c:27-28 · chunk 2463 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sb->sb_cc -= num;
//      sb->sb_rptr += num;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2463 {
use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub extern "C" fn sbdrop__part2463(sb: *mut sbuf, num: c_int) {
    unsafe {
        let s = &mut *sb;
        s.sb_cc = s.sb_cc.wrapping_sub(num as c_uint);
        s.sb_rptr = s.sb_rptr.offset(num as isize);
    }
}
}

// ── `if (sb->sb_rptr >= sb->sb_data + sb->sb_datalen)` · slirp/sbuf.c:29-30 · chunk 2464 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(sb->sb_rptr >= sb->sb_data + sb->sb_datalen)
//       sb->sb_rptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2464 {
use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub extern "C" fn sbdrop__part2464(sb: *mut Sbuf) {
    unsafe {
        let sb = &mut *sb;
        let end = sb.sb_data.wrapping_add(sb.sb_datalen as usize);
        if sb.sb_rptr >= end {
            sb.sb_rptr = sb.sb_rptr.wrapping_sub(sb.sb_datalen as usize);
        }
    }
}
}

// ── `sbreserve` · slirp/sbuf.c:34-55 · chunk 2465 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sbreserve(struct sbuf *sb, int size)
//     {
//      if (sb->sb_data) {
//     
//       if (sb->sb_datalen != size) {
//        sb->sb_wptr = sb->sb_rptr = sb->sb_data = (char *)realloc(sb->sb_data, size);
//        sb->sb_cc = 0;
//        if (sb->sb_wptr)
//           sb->sb_datalen = size;
//        else
//           sb->sb_datalen = 0;
//       }
//      } else {
//       sb->sb_wptr = sb->sb_rptr = sb->sb_data = (char *)malloc(size);
//       sb->sb_cc = 0;
//       if (sb->sb_wptr)
//          sb->sb_datalen = size;
//       else
//          sb->sb_datalen = 0;
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2465 {
// Rewrite of slirp/sbuf.c:sbreserve
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn sbreserve(sb: *mut Sbuf, size: c_int) {
    let sb = &mut *sb;
    if sb.sb_data.is_null() {
        sb.sb_wptr = malloc(size as usize) as *mut c_char;
        sb.sb_rptr = sb.sb_wptr;
        sb.sb_data = sb.sb_wptr;
        sb.sb_cc = 0;
        if !sb.sb_wptr.is_null() {
            sb.sb_datalen = size as c_uint;
        } else {
            sb.sb_datalen = 0;
        }
    } else if sb.sb_datalen != size as c_uint {
        sb.sb_wptr = realloc(sb.sb_data as *mut c_void, size as usize) as *mut c_char;
        sb.sb_rptr = sb.sb_wptr;
        sb.sb_data = sb.sb_wptr;
        sb.sb_cc = 0;
        if !sb.sb_wptr.is_null() {
            sb.sb_datalen = size as c_uint;
        } else {
            sb.sb_datalen = 0;
        }
    }
}
}

// ── `if (sb->sb_data) … else` · slirp/sbuf.c:37-54 · chunk 2466 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_data) {
//     
//       if (sb->sb_datalen != size) {
//        sb->sb_wptr = sb->sb_rptr = sb->sb_data = (char *)realloc(sb->sb_data, size);
//        sb->sb_cc = 0;
//        if (sb->sb_wptr)
//           sb->sb_datalen = size;
//        else
//           sb->sb_datalen = 0;
//       }
//      } else {
//       sb->sb_wptr = sb->sb_rptr = sb->sb_data = (char *)malloc(size);
//       sb->sb_cc = 0;
//       if (sb->sb_wptr)
//          sb->sb_datalen = size;
//       else
//          sb->sb_datalen = 0;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2466 {
// Rust rewrite of the body of sbreserve (slirp/sbuf.c lines 37-54).
use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
}

#[no_mangle]
pub extern "C" fn sbreserve__part2466(sb: *mut Sbuf, size: c_int) {
    unsafe {
        let sb = &mut *sb;
        if !sb.sb_data.is_null() {
            if sb.sb_datalen != size as c_uint {
                let p = realloc(sb.sb_data as *mut core::ffi::c_void, size as usize) as *mut c_char;
                sb.sb_data = p;
                sb.sb_rptr = p;
                sb.sb_wptr = p;
                sb.sb_cc = 0;
                if !sb.sb_wptr.is_null() {
                    sb.sb_datalen = size as c_uint;
                } else {
                    sb.sb_datalen = 0;
                }
            }
        } else {
            let p = malloc(size as usize) as *mut c_char;
            sb.sb_data = p;
            sb.sb_rptr = p;
            sb.sb_wptr = p;
            sb.sb_cc = 0;
            if !sb.sb_wptr.is_null() {
                sb.sb_datalen = size as c_uint;
            } else {
                sb.sb_datalen = 0;
            }
        }
    }
}
}

// ── `sbappend` · slirp/sbuf.c:63-117 · chunk 2467 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sbappend(struct socket *so, struct mbuf *m)
//     {
//      int ret = 0;
//     
//      ;
//      ;
//      ;
//      ;
//     
//     
//      if (m->m_hdr.mh_len <= 0) {
//       m_free(m);
//       return;
//      }
//     
//     
//     
//     
//     
//     
//      if (so->so_urgc) {
//       sbappendsb(&so->so_rcv, m);
//       m_free(m);
//       sosendoob(so);
//       return;
//      }
//     
//     
//     
//     
//     
//      if (!so->so_rcv.sb_cc)
//         ret = slirp_send(so, m->m_hdr.mh_data, m->m_hdr.mh_len, 0);
//     
//      if (ret <= 0) {
//     
//     
//     
//     
//     
//     
//       sbappendsb(&so->so_rcv, m);
//      } else if (ret != m->m_hdr.mh_len) {
//     
//     
//     
//     
//       m->m_hdr.mh_len -= ret;
//       m->m_hdr.mh_data += ret;
//       sbappendsb(&so->so_rcv, m);
//      }
//     
//      m_free(m);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2467 {
//! Port of slirp/sbuf.c: sbappend()
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void};
use core::ptr::addr_of_mut;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: c_ushort,
    pub so_lport: c_ushort,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn m_free(m: *mut mbuf);
    fn sosendoob(so: *mut socket) -> c_int;
    fn slirp_send(so: *mut socket, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// Copy the data from m into sb (the static C helper sbappendsb)
unsafe fn sbappendsb(sb: *mut sbuf, m: *mut mbuf) {
    let mut len: c_int = (*m).m_hdr.mh_len;
    let mut n: c_int;

    if (*sb).sb_wptr < (*sb).sb_rptr {
        n = ((*sb).sb_rptr as isize - (*sb).sb_wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        if n > 0 {
            memcpy(
                (*sb).sb_wptr as *mut c_void,
                (*m).m_hdr.mh_data as *const c_void,
                n as usize,
            );
        }
    } else {
        /* Do the right edge first */
        n = ((*sb).sb_data as isize + (*sb).sb_datalen as isize - (*sb).sb_wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        if n > 0 {
            memcpy(
                (*sb).sb_wptr as *mut c_void,
                (*m).m_hdr.mh_data as *const c_void,
                n as usize,
            );
        }
        len -= n;
        if len != 0 {
            /* Now the left edge */
            let mut nn: c_int = ((*sb).sb_rptr as isize - (*sb).sb_data as isize) as c_int;
            if nn > len {
                nn = len;
            }
            if nn > 0 {
                memcpy(
                    (*sb).sb_data as *mut c_void,
                    (*m).m_hdr.mh_data.offset(n as isize) as *const c_void,
                    nn as usize,
                );
            }
            n += nn;
        }
    }

    (*sb).sb_cc = (*sb).sb_cc.wrapping_add(n as c_uint);
    (*sb).sb_wptr = (*sb).sb_wptr.offset(n as isize);
    if (*sb).sb_wptr >= (*sb).sb_data.offset((*sb).sb_datalen as isize) {
        (*sb).sb_wptr = (*sb).sb_wptr.offset(-((*sb).sb_datalen as isize));
    }
}

#[no_mangle]
pub extern "C" fn sbappend(so: *mut socket, m: *mut mbuf) {
    unsafe {
        let mut ret: c_int = 0;

        /* Shouldn't happen, but...  e.g. foreign host closes connection */
        if (*m).m_hdr.mh_len <= 0 {
            m_free(m);
            return;
        }

        /*
         * If there is urgent data, call sosendoob
         * if not all was sent, sowrite will take care of the rest
         */
        if (*so).so_urgc != 0 {
            sbappendsb(addr_of_mut!((*so).so_rcv), m);
            m_free(m);
            sosendoob(so);
            return;
        }

        /*
         * We only write if there's nothing in the buffer,
         * otherwise it'll arrive out of order, and hence corrupt
         */
        if (*so).so_rcv.sb_cc == 0 {
            ret = slirp_send(
                so,
                (*m).m_hdr.mh_data as *const c_void,
                (*m).m_hdr.mh_len as usize,
                0,
            ) as c_int;
        }

        if ret <= 0 {
            /* Nothing was written */
            sbappendsb(addr_of_mut!((*so).so_rcv), m);
        } else if ret != (*m).m_hdr.mh_len {
            /* Something was written, but not everything.. sbappendsb the rest */
            (*m).m_hdr.mh_len -= ret;
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(ret as isize);
            sbappendsb(addr_of_mut!((*so).so_rcv), m);
        }

        /* Whatever happened, we free the mbuf */
        m_free(m);
    }
}
}

// ── `int ret = 0; ; ; ; ;` · slirp/sbuf.c:66-71 · chunk 2468 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int ret = 0;
//     
//      ;
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2468 {
// Ported piece of sbappend(): initialisation of the local `ret` counter
// (the DEBUG_CALL/DEBUG_ARG statements expand to nothing in this build).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sbappend__part2468() -> c_int {
    0
}
}

// ── `if (m->m_hdr.mh_len <= 0)` · slirp/sbuf.c:74-77 · chunk 2469 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_len <= 0) {
//       m_free(m);
//       return;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2469 {
// Rust port of the early-exit branch of slirp/sbuf.c sbappend():
//   if (m->m_hdr.mh_len <= 0) { m_free(m); return; }
// Returns 1 when the branch was taken (caller must return), 0 otherwise.

use core::ffi::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub M_dat: M_dat,
}

extern "C" {
    fn m_free(m: *mut mbuf);
}

#[no_mangle]
pub extern "C" fn sbappend__part2469(m: *mut mbuf) -> c_int {
    unsafe {
        if (*m).m_hdr.mh_len <= 0 {
            m_free(m);
            return 1;
        }
    }
    0
}
}

// ── `if (so->so_urgc)` · slirp/sbuf.c:84-89 · chunk 2470 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_urgc) {
//       sbappendsb(&so->so_rcv, m);
//       m_free(m);
//       sosendoob(so);
//       return;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2470 {
// Port of the urgent-data branch of sbappend() from slirp/sbuf.c.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union m_dat_union {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub m_dat: m_dat_union,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn m_free(m: *mut mbuf);
    fn sosendoob(so: *mut socket) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// Ported from the file-local static helper sbappendsb().
unsafe fn sbappendsb(sb: *mut sbuf, m: *mut mbuf) {
    let mut len: c_int = (*m).m_hdr.mh_len;
    let mut n: c_int;

    if ((*sb).sb_wptr as usize) < ((*sb).sb_rptr as usize) {
        n = ((*sb).sb_rptr as isize - (*sb).sb_wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        memcpy(
            (*sb).sb_wptr as *mut c_void,
            (*m).m_hdr.mh_data as *const c_void,
            n as usize,
        );
    } else {
        n = (((*sb).sb_data as isize)
            .wrapping_add((*sb).sb_datalen as isize)
            .wrapping_sub((*sb).sb_wptr as isize)) as c_int;
        if n > len {
            n = len;
        }
        memcpy(
            (*sb).sb_wptr as *mut c_void,
            (*m).m_hdr.mh_data as *const c_void,
            n as usize,
        );
        len -= n;
        if len != 0 {
            let mut nn: c_int = ((*sb).sb_rptr as isize - (*sb).sb_data as isize) as c_int;
            if nn > len {
                nn = len;
            }
            memcpy(
                (*sb).sb_data as *mut c_void,
                (*m).m_hdr.mh_data.offset(n as isize) as *const c_void,
                nn as usize,
            );
            n += nn;
        }
    }

    (*sb).sb_cc = (*sb).sb_cc.wrapping_add(n as c_uint);
    (*sb).sb_wptr = (*sb).sb_wptr.offset(n as isize);
    if ((*sb).sb_wptr as usize)
        >= ((*sb).sb_data as usize).wrapping_add((*sb).sb_datalen as usize)
    {
        (*sb).sb_wptr = (*sb).sb_wptr.offset(-((*sb).sb_datalen as isize));
    }
}

// Returns 1 when the urgent-data branch was taken (the caller must `return;`).
#[no_mangle]
pub extern "C" fn sbappend__part2470(so: *mut socket, m: *mut mbuf) -> c_int {
    unsafe {
        if (*so).so_urgc != 0 {
            sbappendsb(core::ptr::addr_of_mut!((*so).so_rcv), m);
            m_free(m);
            sosendoob(so);
            return 1;
        }
    }
    0
}
}

// ── `if (!so->so_rcv.sb_cc)` · slirp/sbuf.c:95-96 · chunk 2471 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!so->so_rcv.sb_cc)
//         ret = slirp_send(so, m->m_hdr.mh_data, m->m_hdr.mh_len, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2471 {
use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut c_void,
    pub mh_prev: *mut c_void,
    pub mh_nextpkt: *mut c_void,
    pub mh_prevpkt: *mut c_void,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
}

extern "C" {
    fn slirp_send(so: *mut Socket, buf: *const c_void, len: usize, flags: c_int) -> c_long;
}

// ret = slirp_send(so, m->m_hdr.mh_data, m->m_hdr.mh_len, 0);
#[no_mangle]
pub unsafe extern "C" fn sbappend__part2471(so: *mut Socket, m: *mut Mbuf) -> c_int {
    let hdr = &(*m).m_hdr;
    let len = hdr.mh_len;
    let ret = slirp_send(so, hdr.mh_data as *const c_void, len as usize, 0);
    ret as c_int
}
}

// ── `if (ret <= 0) … else` · slirp/sbuf.c:98-114 · chunk 2472 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret <= 0) {
//     
//     
//     
//     
//     
//     
//       sbappendsb(&so->so_rcv, m);
//      } else if (ret != m->m_hdr.mh_len) {
//     
//     
//     
//     
//       m->m_hdr.mh_len -= ret;
//       m->m_hdr.mh_data += ret;
//       sbappendsb(&so->so_rcv, m);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2472 {
// Port of the tail of sbappend() from slirp/sbuf.c (lines 98-114).
// Interface: the sbuf of the socket (so->so_rcv), the mbuf and the send result.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// Only m_hdr is accessed; the remaining C fields (slirp, M_dat) are never
// touched here and the type is only ever used behind a pointer.
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Port of the translation unit's `static void sbappendsb(struct sbuf *, struct mbuf *)`
// (a static C function has no linker symbol, so its logic lives here).
unsafe fn sbappendsb_rs(sb: *mut sbuf, m: *mut mbuf) {
    let mut len: c_int = (*m).m_hdr.mh_len;
    let mut n: c_int;
    let wptr = (*sb).sb_wptr;
    let rptr = (*sb).sb_rptr;

    if (wptr as usize) < (rptr as usize) {
        n = (rptr as isize - wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        core::ptr::copy_nonoverlapping((*m).m_hdr.mh_data, wptr, n as usize);
    } else {
        n = ((*sb).sb_data as isize + (*sb).sb_datalen as isize - wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        core::ptr::copy_nonoverlapping((*m).m_hdr.mh_data, wptr, n as usize);
        len = len.wrapping_sub(n);
        if len != 0 {
            let mut nn: c_int =
                ((*sb).sb_rptr as isize - (*sb).sb_data as isize) as c_int;
            if nn > len {
                nn = len;
            }
            core::ptr::copy_nonoverlapping(
                (*m).m_hdr.mh_data.wrapping_offset(n as isize),
                (*sb).sb_data,
                nn as usize,
            );
            n = n.wrapping_add(nn);
        }
    }

    (*sb).sb_cc = (*sb).sb_cc.wrapping_add(n as c_uint);
    (*sb).sb_wptr = (*sb).sb_wptr.wrapping_offset(n as isize);
    if (*sb).sb_wptr as usize
        >= ((*sb).sb_data as usize).wrapping_add((*sb).sb_datalen as usize)
    {
        (*sb).sb_wptr = (*sb).sb_wptr.wrapping_offset(-((*sb).sb_datalen as isize));
    }
}

#[no_mangle]
pub unsafe extern "C" fn sbappend__part2472(sb: *mut sbuf, m: *mut mbuf, ret: c_int) {
    if ret <= 0 {
        sbappendsb_rs(sb, m);
    } else if ret != (*m).m_hdr.mh_len {
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_sub(ret);
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_offset(ret as isize);
        sbappendsb_rs(sb, m);
    }
}
}

// ── `m_free(m);` · slirp/sbuf.c:116-116 · chunk 2473 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m_free(m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2473 {
// Port of the tail `m_free(m);` of sbappend() from slirp/sbuf.c
use core::ffi::c_void;

extern "C" {
    fn m_free(m: *mut c_void);
}

#[no_mangle]
pub extern "C" fn sbappend__part2473(m: *mut c_void) {
    unsafe {
        m_free(m);
    }
}
}

// ── `sbappendsb` · slirp/sbuf.c:123-153 · chunk 2474 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     sbappendsb(struct sbuf *sb, struct mbuf *m)
//     {
//      int len, n, nn;
//     
//      len = m->m_hdr.mh_len;
//     
//      if (sb->sb_wptr < sb->sb_rptr) {
//       n = sb->sb_rptr - sb->sb_wptr;
//       if (n > len) n = len;
//       memcpy(sb->sb_wptr, m->m_hdr.mh_data, n);
//      } else {
//     
//       n = sb->sb_data + sb->sb_datalen - sb->sb_wptr;
//       if (n > len) n = len;
//       memcpy(sb->sb_wptr, m->m_hdr.mh_data, n);
//       len -= n;
//       if (len) {
//     
//        nn = sb->sb_rptr - sb->sb_data;
//        if (nn > len) nn = len;
//        memcpy(sb->sb_data,m->m_hdr.mh_data+n,nn);
//        n += nn;
//       }
//      }
//     
//      sb->sb_cc += n;
//      sb->sb_wptr += n;
//      if (sb->sb_wptr >= sb->sb_data + sb->sb_datalen)
//       sb->sb_wptr -= sb->sb_datalen;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2474 {
// Port of slirp/sbuf.c sbappendsb()
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut core::ffi::c_void,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[inline]
unsafe fn ptr_addr(p: *mut c_char) -> isize {
    p as isize
}

#[no_mangle]
pub unsafe extern "C" fn sbappendsb(sb: *mut sbuf, m: *mut mbuf) {
    let sb = &mut *sb;
    let m = &*m;

    let mut len: c_int = m.m_hdr.mh_len;
    let mut n: c_int;

    let wptr = ptr_addr(sb.sb_wptr);
    let rptr = ptr_addr(sb.sb_rptr);

    if wptr < rptr {
        n = rptr.wrapping_sub(wptr) as c_int;
        if n > len {
            n = len;
        }
        core::ptr::copy_nonoverlapping(m.m_hdr.mh_data, sb.sb_wptr, n as usize);
    } else {
        // Do the right edge first
        let end = ptr_addr(sb.sb_data).wrapping_add(sb.sb_datalen as isize);
        n = end.wrapping_sub(wptr) as c_int;
        if n > len {
            n = len;
        }
        core::ptr::copy_nonoverlapping(m.m_hdr.mh_data, sb.sb_wptr, n as usize);
        len = len.wrapping_sub(n);
        if len != 0 {
            // Now the left edge
            let mut nn: c_int =
                rptr.wrapping_sub(ptr_addr(sb.sb_data)) as c_int;
            if nn > len {
                nn = len;
            }
            core::ptr::copy_nonoverlapping(
                m.m_hdr.mh_data.wrapping_offset(n as isize),
                sb.sb_data,
                nn as usize,
            );
            n = n.wrapping_add(nn);
        }
    }

    sb.sb_cc = sb.sb_cc.wrapping_add(n as c_uint);
    sb.sb_wptr = sb.sb_wptr.wrapping_offset(n as isize);
    let end = ptr_addr(sb.sb_data).wrapping_add(sb.sb_datalen as isize);
    if ptr_addr(sb.sb_wptr) >= end {
        sb.sb_wptr = sb
            .sb_wptr
            .wrapping_offset(-(sb.sb_datalen as isize));
    }
}
}

// ── `int len, n, nn; len = m->m_hdr.mh_len;` · slirp/sbuf.c:126-128 · chunk 2475 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int len, n, nn;
//     
//      len = m->m_hdr.mh_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2475 {
use core::ffi::{c_char, c_int};

// Mirror of the C struct m_hdr (field layout must stay identical).
#[repr(C)]
pub struct MHeader {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// Mirror of the C struct mbuf; only the leading m_hdr field is accessed.
#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHeader,
}

// int len, n, nn; len = m->m_hdr.mh_len;
#[no_mangle]
pub extern "C" fn sbappendsb__part2475(m: *mut MBuf) -> c_int {
    let len: c_int;
    unsafe {
        len = (*m).m_hdr.mh_len;
    }
    len
}
}

// ── `if (sb->sb_wptr < sb->sb_rptr) … else` · slirp/sbuf.c:130-147 · chunk 2476 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_wptr < sb->sb_rptr) {
//       n = sb->sb_rptr - sb->sb_wptr;
//       if (n > len) n = len;
//       memcpy(sb->sb_wptr, m->m_hdr.mh_data, n);
//      } else {
//     
//       n = sb->sb_data + sb->sb_datalen - sb->sb_wptr;
//       if (n > len) n = len;
//       memcpy(sb->sb_wptr, m->m_hdr.mh_data, n);
//       len -= n;
//       if (len) {
//     
//        nn = sb->sb_rptr - sb->sb_data;
//        if (nn > len) nn = len;
//        memcpy(sb->sb_data,m->m_hdr.mh_data+n,nn);
//        n += nn;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2476 {
// Ported piece of sbappendsb() from slirp/sbuf.c (lines 130-147):
// the branch that copies the mbuf data into the circular sbuf.

use core::ffi::{c_char, c_int, c_uint, c_void};

type SizeT = usize;

// struct m_hdr, exactly as in the C translation unit.
#[repr(C)]
pub struct MbufHdr {
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

// struct mbuf: only the leading m_hdr member is ever accessed here.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
}

// struct sbuf, exactly as in the C translation unit.
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn sbappendsb__part2476(
    sb: *mut Sbuf,
    m: *mut Mbuf,
    len_in: c_int,
    nn_out: *mut c_int,
) -> c_int {
    let sb = &mut *sb;
    let m = &*m;

    let mut len: c_int = len_in;
    let mut n: c_int;

    if (sb.sb_wptr as usize) < (sb.sb_rptr as usize) {
        // n = sb_rptr - sb_wptr;
        n = (sb.sb_rptr as isize).wrapping_sub(sb.sb_wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        memcpy(
            sb.sb_wptr as *mut c_void,
            m.m_hdr.mh_data as *const c_void,
            n as SizeT,
        );
    } else {
        // n = sb_data + sb_datalen - sb_wptr;
        let end = (sb.sb_data as isize).wrapping_add(sb.sb_datalen as isize);
        n = end.wrapping_sub(sb.sb_wptr as isize) as c_int;
        if n > len {
            n = len;
        }
        memcpy(
            sb.sb_wptr as *mut c_void,
            m.m_hdr.mh_data as *const c_void,
            n as SizeT,
        );
        len = len.wrapping_sub(n);
        if len != 0 {
            // nn = sb_rptr - sb_data;
            let mut nn: c_int =
                (sb.sb_rptr as isize).wrapping_sub(sb.sb_data as isize) as c_int;
            if nn > len {
                nn = len;
            }
            let src = (m.m_hdr.mh_data as isize).wrapping_add(n as isize) as *const c_void;
            memcpy(sb.sb_data as *mut c_void, src, nn as SizeT);
            n = n.wrapping_add(nn);
            if !nn_out.is_null() {
                *nn_out = nn;
            }
        }
    }

    n
}
}

// ── `sb->sb_cc += n; sb->sb_wptr += n;` · slirp/sbuf.c:149-150 · chunk 2477 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sb->sb_cc += n;
//      sb->sb_wptr += n;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2477 {
// Port of sbappendsb lines: sb->sb_cc += n; sb->sb_wptr += n;
use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn sbappendsb__part2477(sb: *mut Sbuf, n: c_int) {
    let s = &mut *sb;
    s.sb_cc = s.sb_cc.wrapping_add(n as c_uint);
    s.sb_wptr = s.sb_wptr.offset(n as isize);
}
}

// ── `if (sb->sb_wptr >= sb->sb_data + sb->sb_datalen)` · slirp/sbuf.c:151-152 · chunk 2478 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_wptr >= sb->sb_data + sb->sb_datalen)
//       sb->sb_wptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2478 {
use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Wrap the write pointer back to the start of the buffer when it has
// reached the end.  Mirrors:
//   if (sb->sb_wptr >= sb->sb_data + sb->sb_datalen)
//     sb->sb_wptr -= sb->sb_datalen;
#[no_mangle]
pub extern "C" fn sbappendsb__part2478(sb: *mut Sbuf) {
    if sb.is_null() {
        return;
    }
    unsafe {
        let s = &mut *sb;
        let end = (s.sb_data as usize).wrapping_add(s.sb_datalen as usize);
        if s.sb_wptr as usize >= end {
            s.sb_wptr = (s.sb_wptr as usize).wrapping_sub(s.sb_datalen as usize) as *mut c_char;
        }
    }
}
}

// ── `sbcopy` · slirp/sbuf.c:160-181 · chunk 2479 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sbcopy(struct sbuf *sb, int off, int len, char *to)
//     {
//      char *from;
//     
//      from = sb->sb_rptr + off;
//      if (from >= sb->sb_data + sb->sb_datalen)
//       from -= sb->sb_datalen;
//     
//      if (from < sb->sb_wptr) {
//       if (len > sb->sb_cc) len = sb->sb_cc;
//       memcpy(to,from,len);
//      } else {
//     
//       off = (sb->sb_data + sb->sb_datalen) - from;
//       if (off > len) off = len;
//       memcpy(to,from,off);
//       len -= off;
//       if (len)
//          memcpy(to+off,sb->sb_data,len);
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2479 {
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn sbcopy(sb: *mut sbuf, mut off: c_int, mut len: c_int, to: *mut c_char) {
    let data_addr = (*sb).sb_data as usize;
    let datalen = (*sb).sb_datalen as usize;
    let end_addr = data_addr.wrapping_add(datalen);

    let mut from = ((*sb).sb_rptr as usize).wrapping_add(off as isize as usize);
    if from >= end_addr {
        from = from.wrapping_sub(datalen);
    }

    if from < (*sb).sb_wptr as usize {
        if (len as c_uint) > (*sb).sb_cc {
            len = (*sb).sb_cc as c_int;
        }
        memcpy(to as *mut c_void, from as *const c_void, len as usize);
    } else {
        off = end_addr.wrapping_sub(from) as c_int;
        if off > len {
            off = len;
        }
        memcpy(to as *mut c_void, from as *const c_void, off as usize);
        len = len.wrapping_sub(off);
        if len != 0 {
            memcpy(
                to.wrapping_add(off as isize as usize) as *mut c_void,
                (*sb).sb_data as *const c_void,
                len as usize,
            );
        }
    }
}
}

// ── `char *from; from = sb->sb_rptr + off;` · slirp/sbuf.c:163-165 · chunk 2480 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *from;
//     
//      from = sb->sb_rptr + off;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2480 {
use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Computes the read position inside the sbuf data area:
// from = sb->sb_rptr + off;
#[no_mangle]
pub extern "C" fn sbcopy__part2480(sb: *mut sbuf, off: c_int) -> *mut c_char {
    unsafe { (*sb).sb_rptr.offset(off as isize) }
}
}

// ── `if (from >= sb->sb_data + sb->sb_datalen)` · slirp/sbuf.c:166-167 · chunk 2481 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (from >= sb->sb_data + sb->sb_datalen)
//       from -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2481 {
use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub extern "C" fn sbcopy__part2481(sb: *mut Sbuf, from: *mut *mut c_char) {
    unsafe {
        if sb.is_null() || from.is_null() {
            return;
        }
        let s = &*sb;
        let f = *from;
        let end = s.sb_data.wrapping_add(s.sb_datalen as usize);
        if f >= end {
            *from = f.wrapping_sub(s.sb_datalen as usize);
        }
    }
}
}

// ── `if (from < sb->sb_wptr) … else` · slirp/sbuf.c:169-180 · chunk 2482 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (from < sb->sb_wptr) {
//       if (len > sb->sb_cc) len = sb->sb_cc;
//       memcpy(to,from,len);
//      } else {
//     
//       off = (sb->sb_data + sb->sb_datalen) - from;
//       if (off > len) off = len;
//       memcpy(to,from,off);
//       len -= off;
//       if (len)
//          memcpy(to+off,sb->sb_data,len);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2482 {
// Port of the tail of slirp's sbcopy(): the copy branch.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn sbcopy__part2482(
    sb: *mut Sbuf,
    from: *mut c_char,
    mut off: c_int,
    mut len: c_int,
    to: *mut c_char,
) {
    let sb_wptr = (*sb).sb_wptr;
    if (from as usize) < (sb_wptr as usize) {
        // from < sb->sb_wptr
        if len > (*sb).sb_cc as c_int {
            len = (*sb).sb_cc as c_int;
        }
        memcpy(to as *mut c_void, from as *const c_void, len as usize);
    } else {
        let data_end = ((*sb).sb_data as isize).wrapping_add((*sb).sb_datalen as isize);
        off = data_end.wrapping_sub(from as isize) as c_int;
        if off > len {
            off = len;
        }
        memcpy(to as *mut c_void, from as *const c_void, off as usize);
        len = len.wrapping_sub(off);
        if len != 0 {
            memcpy(
                to.wrapping_offset(off as isize) as *mut c_void,
                (*sb).sb_data as *const c_void,
                len as usize,
            );
        }
    }
}
}

