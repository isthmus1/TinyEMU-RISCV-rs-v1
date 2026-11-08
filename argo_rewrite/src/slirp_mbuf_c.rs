//! From `slirp/mbuf.c`.

// ── `m_init` · slirp/mbuf.c:28-33 · chunk 2410 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     m_init(Slirp *slirp)
//     {
//         slirp->m_freelist.m_hdr.mh_next = slirp->m_freelist.m_hdr.mh_prev = &slirp->m_freelist;
//         slirp->m_usedlist.m_hdr.mh_next = slirp->m_usedlist.m_hdr.mh_prev = &slirp->m_usedlist;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2410 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long};

// Mirror of the C structs accessed through the Slirp pointer.
// Only the leading layout up to (and including) m_freelist/m_usedlist is
// needed, but every field is reproduced in order so the offsets match.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct M_hdr {
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
#[derive(Copy, Clone)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut core::ffi::c_void,
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    // Remaining fields of struct Slirp follow here; they are not touched by
    // m_init and are not needed to compute the offsets used below.
}

#[no_mangle]
pub extern "C" fn m_init(slirp: *mut Slirp) {
    unsafe {
        if slirp.is_null() {
            return;
        }
        let free_ptr: *mut Mbuf = &mut (*slirp).m_freelist as *mut Mbuf;
        (*free_ptr).m_hdr.mh_next = free_ptr;
        (*free_ptr).m_hdr.mh_prev = free_ptr;

        let used_ptr: *mut Mbuf = &mut (*slirp).m_usedlist as *mut Mbuf;
        (*used_ptr).m_hdr.mh_next = used_ptr;
        (*used_ptr).m_hdr.mh_prev = used_ptr;
    }
}
}

// ── `slirp->m_freelist.m_hdr.mh_next = slirp->m_freelist.m_hdr.mh_prev = &slirp->m_freelist; slirp->…` · slirp/mbuf.c:31-32 · chunk 2411 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->m_freelist.m_hdr.mh_next = slirp->m_freelist.m_hdr.mh_prev = &slirp->m_freelist;
//         slirp->m_usedlist.m_hdr.mh_next = slirp->m_usedlist.m_hdr.mh_prev = &slirp->m_usedlist;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2411 {
// SPDX-License-Identifier: BSD-3-Clause
// Portion of slirp/mbuf.c: initialization of the mbuf free/used list heads.
// The two list head mbufs are passed by pointer; each head's m_hdr.mh_next and
// m_hdr.mh_prev are made to point at the head itself (empty circular list).

use core::ffi::{c_char, c_int, c_void};

/// `struct m_hdr` from slirp/mbuf.h, exact field order and types.
#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void, /* struct socket * */
    pub mh_data: *mut c_char, /* caddr_t */
    pub mh_len: c_int,
}

/// `union M_dat` from slirp/mbuf.h.
#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

/// `struct mbuf` from slirp/mbuf.h.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void, /* Slirp * */
    pub m_dat: MDat,
}

#[no_mangle]
pub extern "C" fn m_init__part2411(freelist: *mut Mbuf, usedlist: *mut Mbuf) {
    unsafe {
        if !freelist.is_null() {
            (*freelist).m_hdr.mh_next = freelist;
            (*freelist).m_hdr.mh_prev = freelist;
        }
        if !usedlist.is_null() {
            (*usedlist).m_hdr.mh_next = usedlist;
            (*usedlist).m_hdr.mh_prev = usedlist;
        }
    }
}
}

// ── `m_get` · slirp/mbuf.c:43-76 · chunk 2412 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct mbuf *
//     m_get(Slirp *slirp)
//     {
//      register struct mbuf *m;
//      int flags = 0;
//     
//      ;
//     
//      if (slirp->m_freelist.m_hdr.mh_next == &slirp->m_freelist) {
//       m = (struct mbuf *)malloc((1500 + (2 + 14 + 40) + 
//                                __builtin_offsetof (
//                                struct mbuf
//                                , 
//                                M_dat.m_dat_
//                                ) 
//                                + 6));
//       if (m == 
//               ((void *)0)
//                   ) goto end_error;
//       slirp->mbuf_alloced++;
//       if (slirp->mbuf_alloced > 30)
//        flags = 0x08;
//       m->slirp = slirp;
//      } else {
//       m = slirp->m_freelist.m_hdr.mh_next;
//       slirp_remque(m);
//      }
//     
//     
//      slirp_insque(m,&slirp->m_usedlist);
//      m->m_hdr.mh_flags = (flags | 0x04);
//     
//     
//      m->m_hdr.mh_size = (1500 + (2 + 14 + 40) + 
//                 __builtin_offsetof (
//                 struct mbuf
//                 , 
//                 M_dat.m_dat_
//                 ) 
//                 + 6) - 
//                               __builtin_offsetof (
//                               struct mbuf
//                               , 
//                               M_dat.m_dat_
//                               )
//                                                           ;
//      m->m_hdr.mh_data = m->M_dat.m_dat_;
//      m->m_hdr.mh_len = 0;
//             m->m_hdr.mh_nextpkt = 
//                           ((void *)0)
//                               ;
//             m->m_hdr.mh_prevpkt = 
//                           ((void *)0)
//                               ;
//     end_error:
//      ;
//      return m;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2412 {
// Port of slirp/mbuf.c m_get().
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of `struct m_hdr` (slirp/mbuf.h).
#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void, // struct socket *
    pub mh_data: *mut c_char, // caddr_t
    pub mh_len: c_int,
}

// Mirror of `union M_dat`.
#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// Mirror of `struct mbuf`.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

// Partial mirror of `struct Slirp`: only the fields up to and including
// mbuf_alloced are needed, and the layout up to there matches the C struct.
// Below mbuf_alloced the C struct continues, but we never touch it.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut c_void, // struct ex_list *
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn slirp_insque(next: *mut c_void, prev: *mut c_void);
    fn slirp_remque(elem: *mut c_void);
}

const M_EXT: c_int = 0x01;
const M_USEDLIST: c_int = 0x04;
const M_DOFREE: c_int = 0x08;
const MBUF_THRESH: c_int = 30;
const IF_MTU: usize = 1500;
const IF_MAXLINKHDR: usize = 2 + 14 + 40;

// offsetof(struct mbuf, m_dat)
const OFF_MDAT: usize = core::mem::size_of::<MHdr>() + core::mem::size_of::<*mut Slirp>();
// SLIRP_MSIZE
const SLIRP_MSIZE: usize = IF_MTU + IF_MAXLINKHDR + OFF_MDAT + 6;

#[no_mangle]
pub extern "C" fn m_get(slirp: *mut Slirp) -> *mut Mbuf {
    let _ = M_EXT;
    unsafe {
        let mut flags: c_int = 0;

        let freelist: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).m_freelist);
        let usedlist: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).m_usedlist);

        let m: *mut Mbuf;

        if (*freelist).m_hdr.mh_next == freelist {
            let raw = malloc(SLIRP_MSIZE) as *mut Mbuf;
            if raw.is_null() {
                // goto end_error: m is NULL here
                return core::ptr::null_mut();
            }
            m = raw;
            (*slirp).mbuf_alloced = (*slirp).mbuf_alloced.wrapping_add(1);
            if (*slirp).mbuf_alloced > MBUF_THRESH {
                flags = M_DOFREE;
            }
            (*m).slirp = slirp;
        } else {
            m = (*freelist).m_hdr.mh_next;
            slirp_remque(m as *mut c_void);
        }

        // Insert it in the used list
        slirp_insque(m as *mut c_void, usedlist as *mut c_void);
        (*m).m_hdr.mh_flags = flags | M_USEDLIST;

        // Initialise it
        (*m).m_hdr.mh_size = (SLIRP_MSIZE - OFF_MDAT) as c_int;
        (*m).m_hdr.mh_data = core::ptr::addr_of_mut!((*m).m_dat) as *mut c_char;
        (*m).m_hdr.mh_len = 0;
        (*m).m_hdr.mh_nextpkt = core::ptr::null_mut();
        (*m).m_hdr.mh_prevpkt = core::ptr::null_mut();

        m
    }
}
}

// ── `register struct mbuf *m; int flags = 0; ;` · slirp/mbuf.c:46-49 · chunk 2413 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct mbuf *m;
//      int flags = 0;
//     
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2413 {
// Ported entry block of slirp's m_get(): the local register declarations plus
// `int flags = 0;` and the DEBUG_CALL("m_get") statement, which expands to an
// empty statement.  `m` stays a plain C declaration because the remaining C
// code stores an allocated mbuf pointer into it; only the zero initialisation
// of `flags` is produced here.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn m_get__part2413() -> c_int {
    // int flags = 0;
    // (DEBUG_CALL("m_get"); compiles to nothing)
    0 as c_int
}
}

// ── `if (slirp->m_freelist.m_hdr.mh_next == &slirp->m_freelist) … else` · slirp/mbuf.c:51-61 · chunk 2414 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->m_freelist.m_hdr.mh_next == &slirp->m_freelist) {
//       m = (struct mbuf *)malloc((1500 + (2 + 14 + 40) + 
//                                __builtin_offsetof (
//                                struct mbuf
//                                , 
//                                M_dat.m_dat_
//                                ) 
//                                + 6));
//       if (m == 
//               ((void *)0)
//                   ) goto end_error;
//       slirp->mbuf_alloced++;
//       if (slirp->mbuf_alloced > 30)
//        flags = 0x08;
//       m->slirp = slirp;
//      } else {
//       m = slirp->m_freelist.m_hdr.mh_next;
//       slirp_remque(m);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2414 {
// Port of the mbuf allocation branch of slirp m_get().
// Mirrors struct mbuf / struct m_hdr / union M_dat exactly as in slirp/mbuf.h.
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

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

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    // void slirp_remque(void *);  (external definition, slirp/misc.c)
    fn slirp_remque(element: *mut c_void);
}

const fn align_up(x: usize, a: usize) -> usize {
    if a == 0 {
        x
    } else {
        ((x + a - 1) / a) * a
    }
}

// offsetof(struct mbuf, M_dat.m_dat_)
const OFF_MDAT: usize = {
    let p = core::mem::align_of::<*mut c_void>();
    let after_hdr = align_up(core::mem::size_of::<m_hdr>(), p) + core::mem::size_of::<*mut c_void>();
    align_up(after_hdr, core::mem::align_of::<M_dat>())
};

// SLIRP_MSIZE = IF_MTU + IF_MAXLINKHDR + offsetof(struct mbuf, m_dat) + 6
const SLIRP_MSIZE: usize = 1500 + (2 + 14 + 40) + OFF_MDAT + 6;

// Returns the new mbuf, or NULL when malloc() failed (the caller then does
// `goto end_error`, exactly like the original code).
#[no_mangle]
pub extern "C" fn m_get__part2414(
    flags: *mut c_int,
    freelist: *mut mbuf,
    mbuf_alloced: *mut c_int,
    slirp: *mut c_void,
) -> *mut mbuf {
    unsafe {
        if (*freelist).m_hdr.mh_next == freelist {
            let m = malloc(SLIRP_MSIZE) as *mut mbuf;
            if m.is_null() {
                return m;
            }
            *mbuf_alloced += 1;
            if *mbuf_alloced > 30 {
                *flags = 0x08; // M_DOFREE
            }
            (*m).slirp = slirp;
            m
        } else {
            let m = (*freelist).m_hdr.mh_next;
            slirp_remque(m as *mut c_void);
            m
        }
    }
}
}

// ── `slirp_insque(m,&slirp->m_usedlist); m->m_hdr.mh_flags = (flags | 0x04); m->m_hdr.mh_size = (150…` · slirp/mbuf.c:64-72 · chunk 2415 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp_insque(m,&slirp->m_usedlist);
//      m->m_hdr.mh_flags = (flags | 0x04);
//     
//     
//      m->m_hdr.mh_size = (1500 + (2 + 14 + 40) + 
//                 __builtin_offsetof (
//                 struct mbuf
//                 , 
//                 M_dat.m_dat_
//                 ) 
//                 + 6) - 
//                               __builtin_offsetof (
//                               struct mbuf
//                               , 
//                               M_dat.m_dat_
//                               )
//                                                           ;
//      m->m_hdr.mh_data = m->M_dat.m_dat_;
//      m->m_hdr.mh_len = 0;
//             m->m_hdr.mh_nextpkt = 
//                           ((void *)0)
//                               ;
//             m->m_hdr.mh_prevpkt = 
//                           ((void *)0)
//                               ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2415 {
// Port of the tail of m_get(): link the new mbuf into the used list and
// initialise its header fields.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct M_hdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut c_void,
    pub m_dat: M_dat,
}

extern "C" {
    fn slirp_insque(next: *mut c_void, prev: *mut c_void);
}

pub const MBUF_SIZE: c_int = 1500 + (2 + 14 + 40) + 6;

#[no_mangle]
pub extern "C" fn m_get__part2415(m: *mut Mbuf, flags: c_int, usedlist: *mut Mbuf) {
    unsafe {
        slirp_insque(m as *mut c_void, usedlist as *mut c_void);
        let mb = &mut *m;
        mb.m_hdr.mh_flags = flags | 0x04;
        mb.m_hdr.mh_size = MBUF_SIZE;
        mb.m_hdr.mh_data = (&mb.m_dat as *const M_dat as *mut c_char);
        mb.m_hdr.mh_len = 0;
        mb.m_hdr.mh_nextpkt = core::ptr::null_mut();
        mb.m_hdr.mh_prevpkt = core::ptr::null_mut();
    }
}
}

// ── `end_error: ;` · slirp/mbuf.c:73-74 · chunk 2416 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     end_error:
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2416 {
// TARGET: the `end_error: ;` landing pad at the end of slirp's m_get().
// In the original code the label is only a jump target for the
// malloc-failure path and the statement after it is empty, so this
// piece has no observable behaviour at all: it takes no inputs and
// produces no output, exactly like `end_error: ;`.
#[no_mangle]
pub extern "C" fn m_get__part2416() {}
}

// ── `return m;` · slirp/mbuf.c:75-75 · chunk 2417 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return m;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2417 {
// Ported piece of m_get(): the trailing "return m;".
// The mbuf pointer is passed through unchanged.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn m_get__part2417(m: *mut c_void) -> *mut c_void {
    m
}
}

// ── `m_free` · slirp/mbuf.c:78-105 · chunk 2418 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     m_free(struct mbuf *m)
//     {
//     
//       ;
//       ;
//     
//       if(m) {
//     
//      if (m->m_hdr.mh_flags & 0x04)
//         slirp_remque(m);
//     
//     
//      if (m->m_hdr.mh_flags & 0x01)
//         free(m->M_dat.m_ext_);
//     
//     
//     
//     
//      if (m->m_hdr.mh_flags & 0x08) {
//       m->slirp->mbuf_alloced--;
//       free(m);
//      } else if ((m->m_hdr.mh_flags & 0x02) == 0) {
//       slirp_insque(m,&m->slirp->m_freelist);
//       m->m_hdr.mh_flags = 0x02;
//      }
//       }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2418 {
// Rewrite of slirp m_free() in Rust.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

// Only the leading part of struct Slirp that m_free() needs; the layout of
// every preceding field is mirrored exactly so mbuf_alloced lands where C puts it.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void,
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
}

extern "C" {
    fn free(ptr: *mut c_void);
    fn slirp_insque(elem: *mut c_void, list: *mut c_void);
    fn slirp_remque(elem: *mut c_void);
}

#[no_mangle]
pub extern "C" fn m_free(m: *mut Mbuf) {
    unsafe {
        if m.is_null() {
            return;
        }

        // Remove from m_usedlist
        if (*m).m_hdr.mh_flags & 0x04 != 0 {
            slirp_remque(m as *mut c_void);
        }

        // If it's M_EXT, free() it
        if (*m).m_hdr.mh_flags & 0x01 != 0 {
            let ext = (*m).M_dat.m_ext_;
            free(ext as *mut c_void);
        }

        // Either free() it or put it on the free list
        if (*m).m_hdr.mh_flags & 0x08 != 0 {
            let slirp = (*m).slirp;
            let alloced = (*slirp).mbuf_alloced;
            (*slirp).mbuf_alloced = alloced.wrapping_sub(1);
            free(m as *mut c_void);
        } else if (*m).m_hdr.mh_flags & 0x02 == 0 {
            let slirp = (*m).slirp;
            slirp_insque(
                m as *mut c_void,
                &mut (*slirp).m_freelist as *mut Mbuf as *mut c_void,
            );
            (*m).m_hdr.mh_flags = 0x02; // Clobber other flags
        }
    }
}
}

// ── `; ;` · slirp/mbuf.c:82-83 · chunk 2419 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2419 {
// Ported piece of m_free: the DEBUG_CALL / DEBUG_ARG statements, which expand to
// nothing when debug output is not compiled in. Kept as an exported no-op so the
// surrounding C can call it in place.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn m_free__part2419(_m: *mut c_void) {
    // Both original statements were empty after preprocessing; nothing to do.
}
}

// ── `if (m)` · slirp/mbuf.c:85-104 · chunk 2420 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(m) {
//     
//      if (m->m_hdr.mh_flags & 0x04)
//         slirp_remque(m);
//     
//     
//      if (m->m_hdr.mh_flags & 0x01)
//         free(m->M_dat.m_ext_);
//     
//     
//     
//     
//      if (m->m_hdr.mh_flags & 0x08) {
//       m->slirp->mbuf_alloced--;
//       free(m);
//      } else if ((m->m_hdr.mh_flags & 0x02) == 0) {
//       slirp_insque(m,&m->slirp->m_freelist);
//       m->m_hdr.mh_flags = 0x02;
//      }
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2420 {
// Rust rewrite of the body of m_free() (slirp/mbuf.c lines 85-104)
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

// struct in_addr { uint32_t s_addr; }
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct timeval { __time_t tv_sec; __suseconds_t tv_usec; }
#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// struct m_hdr
#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,  // struct socket *
    pub mh_data: *mut c_char, // caddr_t
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; }
#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// struct mbuf { struct m_hdr m_hdr; Slirp *slirp; union M_dat M_dat; }
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

// Mirror of struct Slirp truncated right after mbuf_alloced: only the
// offsets of m_freelist / m_usedlist / mbuf_alloced matter.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void, // struct ex_list *
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
}

extern "C" {
    fn slirp_remque(q: *mut c_void);
    fn slirp_insque(q: *mut c_void, head: *mut c_void);
    fn free(p: *mut c_void);
}

#[no_mangle]
pub extern "C" fn m_free__part2420(m: *mut Mbuf) {
    unsafe {
        if m.is_null() {
            return;
        }

        // Remove from m_usedlist
        if (*m).m_hdr.mh_flags & 0x04 != 0 {
            slirp_remque(m as *mut c_void);
        }

        // If it's M_EXT, free() it
        if (*m).m_hdr.mh_flags & 0x01 != 0 {
            let ext: *mut c_char = (*m).M_dat.m_ext_;
            free(ext as *mut c_void);
        }

        // Either free() it or put it on the free list
        if (*m).m_hdr.mh_flags & 0x08 != 0 {
            let slirp = (*m).slirp;
            (*slirp).mbuf_alloced = (*slirp).mbuf_alloced - 1;
            free(m as *mut c_void);
        } else if ((*m).m_hdr.mh_flags & 0x02) == 0 {
            let slirp = (*m).slirp;
            let head: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).m_freelist);
            slirp_insque(m as *mut c_void, head as *mut c_void);
            (*m).m_hdr.mh_flags = 0x02;
        }
    }
}
}

// ── `m_cat` · slirp/mbuf.c:112-125 · chunk 2421 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     m_cat(struct mbuf *m, struct mbuf *n)
//     {
//     
//     
//     
//      if ((((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len) < n->m_hdr.mh_len)
//       m_inc(m,m->m_hdr.mh_size+4096);
//     
//      memcpy(m->m_hdr.mh_data+m->m_hdr.mh_len, n->m_hdr.mh_data, n->m_hdr.mh_len);
//      m->m_hdr.mh_len += n->m_hdr.mh_len;
//     
//      m_free(n);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2421 {
// Rust rewrite of slirp's m_cat()

use core::ffi::{c_char, c_int, c_void};

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

extern "C" {
    fn m_free(m: *mut mbuf);
    fn m_inc(m: *mut mbuf, size: c_int);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

const M_EXT: c_int = 0x01;
const MINCSIZE: c_int = 4096;

#[no_mangle]
pub unsafe extern "C" fn m_cat(m: *mut mbuf, n: *mut mbuf) {
    if m.is_null() || n.is_null() {
        return;
    }

    let n_len = (*n).m_hdr.mh_len;

    // M_FREEROOM(m)
    let m_hdr = &mut (*m).m_hdr;
    let base: *mut c_char = if (m_hdr.mh_flags & M_EXT) != 0 {
        (*m).M_dat.m_ext_
    } else {
        (*m).M_dat.m_dat_.as_mut_ptr()
    };
    let size = m_hdr.mh_size;
    let data = m_hdr.mh_data;
    let len = m_hdr.mh_len;

    let end = (base as usize).wrapping_add(size as isize as usize);
    let freeroom: isize = (end.wrapping_sub(data as usize)) as isize
        - (len as isize);

    if freeroom < n_len as isize {
        let newsize = size + MINCSIZE;
        m_inc(m, newsize);
    }

    let dst = (*m).m_hdr.mh_data.offset((*m).m_hdr.mh_len as isize) as *mut c_void;
    let src = (*n).m_hdr.mh_data as *const c_void;
    memcpy(dst, src, n_len as usize);
    (*m).m_hdr.mh_len += n_len;

    m_free(n);
}
}

// ── `if (m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)…` · slirp/mbuf.c:118-119 · chunk 2422 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len) < n->m_hdr.mh_len)
//       m_inc(m,m->m_hdr.mh_size+4096);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2422 {
// Port of the "grow if needed" branch from m_cat() in slirp/mbuf.c
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut c_void,
    pub M_dat: MDat,
}

extern "C" {
    fn m_inc(m: *mut Mbuf, size: c_int);
}

#[no_mangle]
pub extern "C" fn m_cat__part2422(m: *mut Mbuf, n: *mut Mbuf) {
    unsafe {
        let mh = &(*m).m_hdr;
        let base: *mut c_char = if (mh.mh_flags & 0x01) != 0 {
            (*m).M_dat.m_ext_
        } else {
            (*m).M_dat.m_dat_.as_ptr() as *mut c_char
        };
        let avail: isize = ((base as isize).wrapping_add(mh.mh_size as isize)
            .wrapping_sub(mh.mh_data as isize))
            .wrapping_sub(mh.mh_len as isize);
        let nlen: isize = (*n).m_hdr.mh_len as isize;

        if avail < nlen {
            let new_size: c_int = (*m).m_hdr.mh_size.wrapping_add(4096);
            m_inc(m, new_size);
        }
    }
}
}

// ── `memcpy(m->m_hdr.mh_data+m->m_hdr.mh_len, n->m_hdr.mh_data, n->m_hdr.mh_len); m->m_hdr.mh_len +=…` · slirp/mbuf.c:121-124 · chunk 2423 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(m->m_hdr.mh_data+m->m_hdr.mh_len, n->m_hdr.mh_data, n->m_hdr.mh_len);
//      m->m_hdr.mh_len += n->m_hdr.mh_len;
//     
//      m_free(n);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2423 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void,
    pub m_dat: MDat,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; } -- size 8, align 8 on this target
#[repr(C)]
pub struct MDat {
    pub m_ext_: *mut c_char,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn m_free(m: *mut Mbuf);
}

#[no_mangle]
pub extern "C" fn m_cat__part2423(m: *mut Mbuf, n: *mut Mbuf) {
    unsafe {
        let n_len = (*n).m_hdr.mh_len;
        let dst_off = (*m).m_hdr.mh_len as usize;
        memcpy(
            (*m).m_hdr.mh_data.add(dst_off) as *mut c_void,
            (*n).m_hdr.mh_data as *const c_void,
            n_len as usize,
        );
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add((*n).m_hdr.mh_len);

        m_free(n);
    }
}
}

// ── `m_inc` · slirp/mbuf.c:129-154 · chunk 2424 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     m_inc(struct mbuf *m, int size)
//     {
//      int datasize;
//     
//     
//             if(m->m_hdr.mh_size>size) return;
//     
//             if (m->m_hdr.mh_flags & 0x01) {
//        datasize = m->m_hdr.mh_data - m->M_dat.m_ext_;
//        m->M_dat.m_ext_ = (char *)realloc(m->M_dat.m_ext_,size);
//        m->m_hdr.mh_data = m->M_dat.m_ext_ + datasize;
//             } else {
//        char *dat;
//        datasize = m->m_hdr.mh_data - m->M_dat.m_dat_;
//        dat = (char *)malloc(size);
//        memcpy(dat, m->M_dat.m_dat_, m->m_hdr.mh_size);
//     
//        m->M_dat.m_ext_ = dat;
//        m->m_hdr.mh_data = m->M_dat.m_ext_ + datasize;
//        m->m_hdr.mh_flags |= 0x01;
//             }
//     
//             m->m_hdr.mh_size = size;
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2424 {
// Rust rewrite of slirp's m_inc() from mbuf.c
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MbHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbHdr,
    pub slirp: *mut c_void,
    pub M_dat: MDat,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn m_inc(m: *mut Mbuf, size: c_int) {
    if m.is_null() {
        return;
    }
    let m = &mut *m;

    if m.m_hdr.mh_size > size {
        return;
    }

    let datasize: c_int;

    if (m.m_hdr.mh_flags & 0x01) != 0 {
        let old_ext = m.M_dat.m_ext_;
        datasize = (m.m_hdr.mh_data as isize - old_ext as isize) as c_int;
        let new_ext = realloc(old_ext as *mut c_void, size as usize) as *mut c_char;
        m.M_dat.m_ext_ = new_ext;
        m.m_hdr.mh_data = new_ext.wrapping_add(datasize as usize);
    } else {
        let old_dat = m.M_dat.m_dat_.as_mut_ptr();
        datasize = (m.m_hdr.mh_data as isize - old_dat as isize) as c_int;
        let dat = malloc(size as usize) as *mut c_char;
        memcpy(
            dat as *mut c_void,
            old_dat as *const c_void,
            m.m_hdr.mh_size as usize,
        );

        m.M_dat.m_ext_ = dat;
        m.m_hdr.mh_data = dat.wrapping_add(datasize as usize);
        m.m_hdr.mh_flags |= 0x01;
    }

    m.m_hdr.mh_size = size;
}
}

// ── `if (m->m_hdr.mh_size>size)` · slirp/mbuf.c:135-135 · chunk 2425 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(m->m_hdr.mh_size>size) return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2425 {
// Port of the early-return guard of m_inc(): report whether m->m_hdr.mh_size > size.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct MHeader {
    pub mh_next: *mut Mbuff,
    pub mh_prev: *mut Mbuff,
    pub mh_nextpkt: *mut Mbuff,
    pub mh_prevpkt: *mut Mbuff,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuff {
    pub m_hdr: MHeader,
    pub slirp: *mut core::ffi::c_void,
    pub M_dat: MDat,
}

#[no_mangle]
pub unsafe extern "C" fn m_inc__part2425(m: *mut Mbuff, size: c_int) -> c_int {
    if (*m).m_hdr.mh_size > size {
        1
    } else {
        0
    }
}
}

// ── `if (m->m_hdr.mh_flags & 0x01) … else` · slirp/mbuf.c:137-150 · chunk 2426 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_flags & 0x01) {
//        datasize = m->m_hdr.mh_data - m->M_dat.m_ext_;
//        m->M_dat.m_ext_ = (char *)realloc(m->M_dat.m_ext_,size);
//        m->m_hdr.mh_data = m->M_dat.m_ext_ + datasize;
//             } else {
//        char *dat;
//        datasize = m->m_hdr.mh_data - m->M_dat.m_dat_;
//        dat = (char *)malloc(size);
//        memcpy(dat, m->M_dat.m_dat_, m->m_hdr.mh_size);
//     
//        m->M_dat.m_ext_ = dat;
//        m->m_hdr.mh_data = m->M_dat.m_ext_ + datasize;
//        m->m_hdr.mh_flags |= 0x01;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2426 {
// Port of the mbuf grow/detach branch of m_inc() from slirp/mbuf.c
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
#[allow(dead_code)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[allow(dead_code)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut c_void,
    pub M_dat: MDat,
}

extern "C" {
    fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn m_inc__part2426(m: *mut Mbuf, size: c_int) {
    unsafe {
        if m.is_null() {
            return;
        }
        let mp: *mut Mbuf = m;

        if ((*mp).m_hdr.mh_flags & 0x01) != 0 {
            // datasize = mh_data - m_ext_  (int truncation as in C)
            let datasize: c_int = ((*mp).m_hdr.mh_data as isize)
                .wrapping_sub((*mp).M_dat.m_ext_ as isize) as c_int;
            let newp = realloc((*mp).M_dat.m_ext_ as *mut c_void, size as usize) as *mut c_char;
            (*mp).M_dat.m_ext_ = newp;
            (*mp).m_hdr.mh_data = newp.wrapping_offset(datasize as isize);
        } else {
            let dat_base: *const c_char = core::ptr::addr_of!((*mp).M_dat.m_dat_) as *const c_char;
            let datasize: c_int =
                ((*mp).m_hdr.mh_data as isize).wrapping_sub(dat_base as isize) as c_int;
            let dat = malloc(size as usize) as *mut c_char;
            memcpy(
                dat as *mut c_void,
                dat_base as *const c_void,
                (*mp).m_hdr.mh_size as usize,
            );

            (*mp).M_dat.m_ext_ = dat;
            (*mp).m_hdr.mh_data = dat.wrapping_offset(datasize as isize);
            (*mp).m_hdr.mh_flags |= 0x01;
        }
    }
}
}

// ── `m->m_hdr.mh_size = size;` · slirp/mbuf.c:152-152 · chunk 2427 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_size = size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2427 {
// Portion of slirp m_inc(): set the mbuf header size field.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn m_inc__part2427(mh_size: *mut c_int, size: c_int) {
    unsafe {
        *mh_size = size;
    }
}
}

// ── `m_adj` · slirp/mbuf.c:158-172 · chunk 2428 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     m_adj(struct mbuf *m, int len)
//     {
//      if (m == 
//              ((void *)0)
//                  )
//       return;
//      if (len >= 0) {
//     
//       m->m_hdr.mh_data += len;
//       m->m_hdr.mh_len -= len;
//      } else {
//     
//       len = -len;
//       m->m_hdr.mh_len -= len;
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2428 {
// Rewrite of slirp's m_adj().
use core::ffi::{c_char, c_int};

// Mirror of `struct m_hdr` from slirp/mbuf.h (exact field order/types).
#[repr(C)]
pub struct MHdr {
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

// Mirror of `struct mbuf` (only the header is needed here, but the full
// layout is reproduced so field offsets stay identical).
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: char,
}

#[no_mangle]
pub extern "C" fn m_adj(m: *mut Mbuf, len: c_int) {
    unsafe {
        if m.is_null() {
            return;
        }
        if len >= 0 {
            // Trim from head
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(len as isize);
            (*m).m_hdr.mh_len -= len;
        } else {
            // Trim from tail
            let len = len.wrapping_neg();
            (*m).m_hdr.mh_len -= len;
        }
    }
}
}

// ── `if (m == ((void *)0)` · slirp/mbuf.c:161-162 · chunk 2429 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m == 
//              ((void *)0)
//                  )
//       return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2429 {
// Null-check guard extracted from slirp's m_adj().
// Returns 1 when the mbuf pointer is NULL (caller must return early),
// 0 otherwise. Only the pointer value is observed, so it is opaque here.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn m_adj__part2429(m: *mut c_void) -> c_int {
    if m.is_null() {
        1
    } else {
        0
    }
}
}

// ── `if (len >= 0) … else` · slirp/mbuf.c:163-171 · chunk 2430 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len >= 0) {
//     
//       m->m_hdr.mh_data += len;
//       m->m_hdr.mh_len -= len;
//      } else {
//     
//       len = -len;
//       m->m_hdr.mh_len -= len;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2430 {
// Port of slirp/mbuf.c m_adj body (lines 163-171): adjust the data pointer /
// length of a mbuf by a signed offset.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    // Remaining fields (slirp, union M_dat) are never touched here.
}

#[no_mangle]
pub unsafe extern "C" fn m_adj__part2430(m: *mut Mbuf, mut len: c_int) {
    if len >= 0 {
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(len as isize);
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_sub(len);
    } else {
        len = len.wrapping_neg();
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_sub(len);
    }
}
}

// ── `m_copy` · slirp/mbuf.c:178-187 · chunk 2431 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int
//     m_copy(struct mbuf *n, struct mbuf *m, int off, int len)
//     {
//      if (len > (((n->m_hdr.mh_flags & 0x01)? (((n)->M_dat.m_ext_ + (n)->m_hdr.mh_size) - (n)->m_hdr.mh_data) : (((n)->M_dat.m_dat_ + (n)->m_hdr.mh_size) - (n)->m_hdr.mh_data)) - (n)->m_hdr.mh_len))
//       return -1;
//     
//      memcpy((n->m_hdr.mh_data + n->m_hdr.mh_len), (m->m_hdr.mh_data + off), len);
//      n->m_hdr.mh_len += len;
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2431 {
// Rust port of slirp's m_copy().
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

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

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn m_copy(n: *mut mbuf, m: *mut mbuf, off: c_int, len: c_int) -> c_int {
    let nh = &(*n).m_hdr;
    // M_FREEROOM(n): (base + mh_size - mh_data) - mh_len, computed in byte units.
    let base: *mut c_char = if (nh.mh_flags & 0x01) != 0 {
        (*n).M_dat.m_ext_
    } else {
        (*n).M_dat.m_dat_.as_mut_ptr()
    };
    let room: isize = (base as isize)
        .wrapping_add(nh.mh_size as isize)
        .wrapping_sub(nh.mh_data as isize)
        .wrapping_sub(nh.mh_len as isize);

    if (len as isize) > room {
        return -1;
    }

    let dst = (nh.mh_data as isize).wrapping_add(nh.mh_len as isize) as *mut c_void;
    let src = ((*m).m_hdr.mh_data as isize).wrapping_add(off as isize) as *const c_void;
    memcpy(dst, src, len as usize);
    (*n).m_hdr.mh_len = (*n).m_hdr.mh_len.wrapping_add(len);
    0
}
}

// ── `if (len > (((n->m_hdr.mh_flags & 0x01)? (((n)->M_dat.m_ext_ + (n)->m_hdr.mh_size) - (n)->m_hdr.…` · slirp/mbuf.c:181-182 · chunk 2432 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len > (((n->m_hdr.mh_flags & 0x01)? (((n)->M_dat.m_ext_ + (n)->m_hdr.mh_size) - (n)->m_hdr.mh_data) : (((n)->M_dat.m_dat_ + (n)->m_hdr.mh_size) - (n)->m_hdr.mh_data)) - (n)->m_hdr.mh_len))
//       return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2432 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct M_hdr {
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

#[repr(C)]
pub union M_dat_union {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut c_void,
    pub M_dat: M_dat_union,
}

/// Returns nonzero when the requested `len` does not fit into the free room of
/// `n` (i.e. the caller must `return -1`), zero otherwise.
#[no_mangle]
pub unsafe extern "C" fn m_copy__part2432(n: *mut mbuf, len: c_int) -> c_int {
    let hdr = core::ptr::addr_of!((*n).m_hdr);
    let flags = (*hdr).mh_flags;
    let size = (*hdr).mh_size;
    let data = (*hdr).mh_data as usize;
    let mlen = (*hdr).mh_len;

    let room: isize = if (flags & 0x01) != 0 {
        let ext = core::ptr::addr_of!((*n).M_dat.m_ext_).read() as usize;
        ext.wrapping_add(size as usize).wrapping_sub(data) as isize
    } else {
        let dat = core::ptr::addr_of!((*n).M_dat.m_dat_).cast::<c_char>() as usize;
        dat.wrapping_add(size as usize).wrapping_sub(data) as isize
    };

    let avail = room.wrapping_sub(mlen as isize);

    if (len as isize) > avail {
        1
    } else {
        0
    }
}
}

// ── `memcpy((n->m_hdr.mh_data + n->m_hdr.mh_len), (m->m_hdr.mh_data + off), len); n->m_hdr.mh_len +=…` · slirp/mbuf.c:184-186 · chunk 2433 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy((n->m_hdr.mh_data + n->m_hdr.mh_len), (m->m_hdr.mh_data + off), len);
//      n->m_hdr.mh_len += len;
//      return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2433 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub m_dat: M_dat,
}

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

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn m_copy__part2433(
    n: *mut mbuf,
    m: *mut mbuf,
    off: c_int,
    len: c_int,
) -> c_int {
    let dest = (*n).m_hdr.mh_data.offset((*n).m_hdr.mh_len as isize) as *mut c_void;
    let src = (*m).m_hdr.mh_data.offset(off as isize) as *const c_void;
    memcpy(dest, src, len as usize);
    (*n).m_hdr.mh_len += len;
    0
}
}

// ── `dtom` · slirp/mbuf.c:195-218 · chunk 2434 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct mbuf *
//     dtom(Slirp *slirp, void *dat)
//     {
//      struct mbuf *m;
//     
//      ;
//      ;
//     
//     
//      for (m = slirp->m_usedlist.m_hdr.mh_next; m != &slirp->m_usedlist;
//           m = m->m_hdr.mh_next) {
//        if (m->m_hdr.mh_flags & 0x01) {
//          if( (char *)dat>=m->M_dat.m_ext_ && (char *)dat<(m->M_dat.m_ext_ + m->m_hdr.mh_size) )
//            return m;
//        } else {
//          if( (char *)dat >= m->M_dat.m_dat_ && (char *)dat<(m->M_dat.m_dat_ + m->m_hdr.mh_size) )
//            return m;
//        }
//      }
//     
//      ;
//     
//      return (struct mbuf *)0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2434 {
// Rust port of slirp's dtom(): given a pointer into an mbuf, return the mbuf.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// ---- mirrored C types (layout only, fields in exact C order) ----

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

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

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: in_addr,
    pub vnetwork_mask: in_addr,
    pub vhost_addr: in_addr,
    pub vdhcp_startaddr: in_addr,
    pub vnameserver_addr: in_addr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: in_addr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: timeval,
    pub exec_list: *mut ex_list,
    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
}

const M_EXT: c_int = 0x01;

/// struct mbuf *
/// dtom(Slirp *slirp, void *dat)
#[no_mangle]
pub unsafe extern "C" fn dtom(slirp: *mut Slirp, dat: *mut c_void) -> *mut mbuf {
    if slirp.is_null() {
        return core::ptr::null_mut();
    }

    let usedlist: *mut mbuf = core::ptr::addr_of_mut!((*slirp).m_usedlist);
    let d = dat as usize;

    let mut m: *mut mbuf = (*usedlist).m_hdr.mh_next;
    while !core::ptr::eq(m, usedlist) {
        if m.is_null() {
            break;
        }
        if ((*m).m_hdr.mh_flags & M_EXT) != 0 {
            let base = (*m).M_dat.m_ext_ as usize;
            let end = base.wrapping_add((*m).m_hdr.mh_size as usize);
            if d >= base && d < end {
                return m;
            }
        } else {
            let base = core::ptr::addr_of!((*m).M_dat.m_dat_) as usize;
            let end = base.wrapping_add((*m).m_hdr.mh_size as usize);
            if d >= base && d < end {
                return m;
            }
        }
        m = (*m).m_hdr.mh_next;
    }

    core::ptr::null_mut()
}
}

// ── `struct mbuf *m; ; ;` · slirp/mbuf.c:198-201 · chunk 2435 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct mbuf *m;
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2435 {
// Piece of dtom(): declares the local `struct mbuf *m` (the DEBUG_CALL/DEBUG_ARG
// macros expand to nothing and the loop below assigns `m` before any use).
// The C code only needs an initial value for `m`, so this returns NULL exactly
// like an uninitialized-then-assigned pointer would be irrelevant here.

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn dtom__part2435() -> *mut mbuf {
    core::ptr::null_mut()
}
}

// ── `for (m = slirp->m_usedlist.m_hdr.mh_next; m != &slirp->m_usedlist; m = m->m_hdr.mh_next)` · slirp/mbuf.c:204-213 · chunk 2436 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (m = slirp->m_usedlist.m_hdr.mh_next; m != &slirp->m_usedlist;
//           m = m->m_hdr.mh_next) {
//        if (m->m_hdr.mh_flags & 0x01) {
//          if( (char *)dat>=m->M_dat.m_ext_ && (char *)dat<(m->M_dat.m_ext_ + m->m_hdr.mh_size) )
//            return m;
//        } else {
//          if( (char *)dat >= m->M_dat.m_dat_ && (char *)dat<(m->M_dat.m_dat_ + m->m_hdr.mh_size) )
//            return m;
//        }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2436 {
// Port of the dtom() list scan (slirp/mbuf.c lines 204-213).
// Walks the mbuf "used list" (a circular list whose sentinel is slirp->m_usedlist)
// and returns the mbuf whose data area contains `dat`, or null if none does.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[allow(non_snake_case)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void, /* struct socket * */
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
#[allow(non_snake_case)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void, /* Slirp * */
    pub M_dat: M_dat,
}

#[no_mangle]
pub extern "C" fn dtom__part2436(head: *mut mbuf, dat: *mut c_void) -> *mut mbuf {
    let dat_addr = dat as usize;
    unsafe {
        if head.is_null() {
            return core::ptr::null_mut();
        }
        let mut m: *mut mbuf = (*head).m_hdr.mh_next;
        while m != head {
            if m.is_null() {
                break;
            }
            let flags = (*m).m_hdr.mh_flags;
            let size = (*m).m_hdr.mh_size;
            let (base_addr, ok) = if flags & 0x01 != 0 {
                let base = (*m).M_dat.m_ext_;
                (base as usize, true)
            } else {
                (core::ptr::addr_of_mut!((*m).M_dat) as usize, false)
            };
            let _ = ok;
            let end_addr = (base_addr as isize).wrapping_add(size as isize) as usize;
            if dat_addr >= base_addr && dat_addr < end_addr {
                return m;
            }
            m = (*m).m_hdr.mh_next;
        }
        core::ptr::null_mut()
    }
}
}

// ── `; return (struct mbuf *)0;` · slirp/mbuf.c:215-217 · chunk 2437 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//     
//      return (struct mbuf *)0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2437 {
// Tail of dtom(): no mbuf matched, so return a null struct mbuf pointer.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn dtom__part2437() -> *mut c_void {
    core::ptr::null_mut()
}
}

