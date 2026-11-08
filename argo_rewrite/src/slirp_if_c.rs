//! From `slirp/if.c`.

// ── `ifs_insque` · slirp/if.c:12-19 · chunk 2285 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     ifs_insque(struct mbuf *ifm, struct mbuf *ifmhead)
//     {
//      ifm->m_hdr.mh_nextpkt = ifmhead->m_hdr.mh_nextpkt;
//      ifmhead->m_hdr.mh_nextpkt = ifm;
//      ifm->m_hdr.mh_prevpkt = ifmhead;
//      ifm->m_hdr.mh_nextpkt->m_hdr.mh_prevpkt = ifm;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2285 {
// Rust rewrite of slirp/if.c ifs_insque (static helper, exported under the C name).
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::{c_char, c_int};

/// Opaque placeholder for C `Slirp` (never dereferenced here).
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

/// Opaque placeholder for C `struct socket` (never dereferenced here).
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
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

#[no_mangle]
pub extern "C" fn ifs_insque(ifm: *mut mbuf, ifmhead: *mut mbuf) {
    unsafe {
        (*ifm).m_hdr.mh_nextpkt = (*ifmhead).m_hdr.mh_nextpkt;
        (*ifmhead).m_hdr.mh_nextpkt = ifm;
        (*ifm).m_hdr.mh_prevpkt = ifmhead;
        (*(*ifm).m_hdr.mh_nextpkt).m_hdr.mh_prevpkt = ifm;
    }
}
}

// ── `ifm->m_hdr.mh_nextpkt = ifmhead->m_hdr.mh_nextpkt; ifmhead->m_hdr.mh_nextpkt = ifm; ifm->m_hdr.…` · slirp/if.c:15-18 · chunk 2286 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ifm->m_hdr.mh_nextpkt = ifmhead->m_hdr.mh_nextpkt;
//      ifmhead->m_hdr.mh_nextpkt = ifm;
//      ifm->m_hdr.mh_prevpkt = ifmhead;
//      ifm->m_hdr.mh_nextpkt->m_hdr.mh_prevpkt = ifm;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2286 {
// Port of the insertion part of ifs_insque (slirp/if.c).

#[repr(C)]
pub struct MbHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbHdr,
}

#[no_mangle]
pub extern "C" fn ifs_insque__part2286(ifm: *mut Mbuf, ifmhead: *mut Mbuf) {
    unsafe {
        (*ifm).m_hdr.mh_nextpkt = (*ifmhead).m_hdr.mh_nextpkt;
        (*ifmhead).m_hdr.mh_nextpkt = ifm;
        (*ifm).m_hdr.mh_prevpkt = ifmhead;
        let nextpkt = (*ifm).m_hdr.mh_nextpkt;
        (*nextpkt).m_hdr.mh_prevpkt = ifm;
    }
}
}

// ── `ifs_remque` · slirp/if.c:21-26 · chunk 2287 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     ifs_remque(struct mbuf *ifm)
//     {
//      ifm->m_hdr.mh_prevpkt->m_hdr.mh_nextpkt = ifm->m_hdr.mh_nextpkt;
//      ifm->m_hdr.mh_nextpkt->m_hdr.mh_prevpkt = ifm->m_hdr.mh_prevpkt;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2287 {
// Rust rewrite of slirp/if.c ifs_remque()
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

// Mirror of the C `struct m_hdr` (slirp/mbuf.h)
#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void, // struct socket *
    pub mh_data: *mut c_char,          // caddr_t
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; }
#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// Mirror of the C `struct mbuf`
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut core::ffi::c_void, // Slirp *
    pub m_dat: M_dat,
}

#[no_mangle]
pub unsafe extern "C" fn ifs_remque(ifm: *mut mbuf) {
    let prev = (*ifm).m_hdr.mh_prevpkt;
    let next = (*ifm).m_hdr.mh_nextpkt;
    (*prev).m_hdr.mh_nextpkt = next;
    (*next).m_hdr.mh_prevpkt = prev;
}
}

// ── `ifm->m_hdr.mh_prevpkt->m_hdr.mh_nextpkt = ifm->m_hdr.mh_nextpkt; ifm->m_hdr.mh_nextpkt->m_hdr.m…` · slirp/if.c:24-25 · chunk 2288 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ifm->m_hdr.mh_prevpkt->m_hdr.mh_nextpkt = ifm->m_hdr.mh_nextpkt;
//      ifm->m_hdr.mh_nextpkt->m_hdr.mh_prevpkt = ifm->m_hdr.mh_prevpkt;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2288 {
use core::ffi::{c_char, c_int};

// Mirror of the C `struct m_hdr` (slirp/if.c / mbuf.h) reachable through
// `struct mbuf *`; only the pointer fields are touched here.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: [u8; 8],
}

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

#[no_mangle]
pub unsafe extern "C" fn ifs_remque__part2288(ifm: *mut Mbuf) {
    let prevpkt = (*ifm).m_hdr.mh_prevpkt;
    let nextpkt = (*ifm).m_hdr.mh_nextpkt;
    (*prevpkt).m_hdr.mh_nextpkt = nextpkt;
    let prevpkt = (*ifm).m_hdr.mh_prevpkt;
    let nextpkt = (*ifm).m_hdr.mh_nextpkt;
    (*nextpkt).m_hdr.mh_prevpkt = prevpkt;
}
}

// ── `if_init` · slirp/if.c:28-34 · chunk 2289 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     if_init(Slirp *slirp)
//     {
//         slirp->if_fastq.m_hdr.mh_next = slirp->if_fastq.m_hdr.mh_prev = &slirp->if_fastq;
//         slirp->if_batchq.m_hdr.mh_next = slirp->if_batchq.m_hdr.mh_prev = &slirp->if_batchq;
//         slirp->next_m = &slirp->if_batchq;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2289 {
// Port of slirp/if.c: if_init

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void, // struct socket *
    pub mh_data: *mut c_char,          // caddr_t
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
    pub m_dat: MDat,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// struct ex_list is only used through a pointer.
#[repr(C)]
pub struct ExList {
    _opaque: [u8; 0],
}

// Prefix of struct Slirp, up to and including next_m (the only fields used here).
// All preceding fields are reproduced exactly to preserve offsets.
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
    pub exec_list: *mut ExList,
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
}

#[no_mangle]
pub extern "C" fn if_init(slirp: *mut Slirp) {
    if slirp.is_null() {
        return;
    }
    unsafe {
        let fastq: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).if_fastq);
        (*fastq).m_hdr.mh_next = fastq;
        (*fastq).m_hdr.mh_prev = fastq;

        let batchq: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).if_batchq);
        (*batchq).m_hdr.mh_next = batchq;
        (*batchq).m_hdr.mh_prev = batchq;

        (*slirp).next_m = batchq;
    }
}
}

// ── `slirp->if_fastq.m_hdr.mh_next = slirp->if_fastq.m_hdr.mh_prev = &slirp->if_fastq; slirp->if_bat…` · slirp/if.c:31-33 · chunk 2290 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->if_fastq.m_hdr.mh_next = slirp->if_fastq.m_hdr.mh_prev = &slirp->if_fastq;
//         slirp->if_batchq.m_hdr.mh_next = slirp->if_batchq.m_hdr.mh_prev = &slirp->if_batchq;
//         slirp->next_m = &slirp->if_batchq;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2290 {
// Port of the queue-initialisation part of slirp's if_init().
// The caller passes the addresses of the two queue headers' m_hdr part
// (whose first two members are mh_next / mh_prev) and the address of the
// next_m slot, so no full struct layouts have to be mirrored.

use core::ffi::c_void;

/// The leading part of `struct m_hdr` that this piece touches.
#[repr(C)]
struct MHdrLinks {
    mh_next: *mut c_void,
    mh_prev: *mut c_void,
}

#[no_mangle]
pub extern "C" fn if_init__part2290(
    fastq_hdr: *mut MHdrLinks,
    batchq_hdr: *mut MHdrLinks,
    next_m: *mut *mut c_void,
) {
    unsafe {
        if !fastq_hdr.is_null() {
            (*fastq_hdr).mh_next = fastq_hdr as *mut c_void;
            (*fastq_hdr).mh_prev = fastq_hdr as *mut c_void;
        }
        if !batchq_hdr.is_null() {
            (*batchq_hdr).mh_next = batchq_hdr as *mut c_void;
            (*batchq_hdr).mh_prev = batchq_hdr as *mut c_void;
        }
        if !next_m.is_null() {
            *next_m = batchq_hdr as *mut c_void;
        }
    }
}
}

// ── `if_output` · slirp/if.c:49-139 · chunk 2291 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     if_output(struct socket *so, struct mbuf *ifm)
//     {
//      Slirp *slirp = ifm->slirp;
//      struct mbuf *ifq;
//      int on_fastq = 1;
//     
//      ;
//      ;
//      ;
//     
//     
//     
//     
//     
//     
//      if (ifm->m_hdr.mh_flags & 0x04) {
//       slirp_remque(ifm);
//       ifm->m_hdr.mh_flags &= ~0x04;
//      }
//      for (ifq = slirp->if_batchq.m_hdr.mh_prev; ifq != &slirp->if_batchq;
//           ifq = ifq->m_hdr.mh_prev) {
//       if (so == ifq->m_hdr.mh_so) {
//     
//        ifm->m_hdr.mh_so = so;
//        ifs_insque(ifm, ifq->m_hdr.mh_prevpkt);
//        goto diddit;
//       }
//      }
//     
//     
//      if (so && (so->so_iptos & 0x10)) {
//       ifq = slirp->if_fastq.m_hdr.mh_prev;
//       on_fastq = 1;
//     
//     
//     
//     
//       if (ifq->m_hdr.mh_so == so) {
//        ifm->m_hdr.mh_so = so;
//        ifs_insque(ifm, ifq->m_hdr.mh_prevpkt);
//        goto diddit;
//       }
//      } else
//       ifq = slirp->if_batchq.m_hdr.mh_prev;
//     
//     
//      ifm->m_hdr.mh_so = so;
//      ((ifm)->m_hdr.mh_nextpkt = (ifm)->m_hdr.mh_prevpkt = (ifm));
//      slirp_insque(ifm, ifq);
//     
//     diddit:
//      slirp->if_queued++;
//     
//      if (so) {
//     
//       so->so_queued++;
//       so->so_nqueued++;
//     
//     
//     
//     
//     
//     
//     
//       if (on_fastq && ((so->so_nqueued >= 6) &&
//          (so->so_nqueued - so->so_queued) >= 3)) {
//     
//     
//        slirp_remque(ifm->m_hdr.mh_nextpkt);
//     
//     
//        slirp_insque(ifm->m_hdr.mh_nextpkt, &slirp->if_batchq);
//       }
//      }
//     
//     
//     
//     
//     
//      if_start(ifm->slirp);
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2291 {
// Port of slirp/if.c if_output()
#![allow(non_camel_case_types, non_snake_case, dead_code)]

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
    pub mh_so: *mut Socket,
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
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
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
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
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
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
}

extern "C" {
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    fn slirp_remque(a: *mut c_void);
    fn if_start(slirp: *mut Slirp);
}

// static void ifs_insque(struct mbuf *ifm, struct mbuf *ifmhead)
unsafe fn ifs_insque(ifm: *mut Mbuf, ifmhead: *mut Mbuf) {
    (*ifm).m_hdr.mh_nextpkt = (*ifmhead).m_hdr.mh_nextpkt;
    (*ifmhead).m_hdr.mh_nextpkt = ifm;
    (*ifm).m_hdr.mh_prevpkt = ifmhead;
    (*(*ifm).m_hdr.mh_nextpkt).m_hdr.mh_prevpkt = ifm;
}

#[no_mangle]
pub unsafe extern "C" fn if_output(so: *mut Socket, ifm: *mut Mbuf) {
    let slirp: *mut Slirp = (*ifm).slirp;
    let mut ifq: *mut Mbuf;
    let mut on_fastq: c_int = 1;

    /*
     * First remove the mbuf from m_usedlist,
     * since we're gonna use m_next and m_prev ourselves
     */
    if ((*ifm).m_hdr.mh_flags & 0x04) != 0 {
        slirp_remque(ifm as *mut c_void);
        (*ifm).m_hdr.mh_flags &= !0x04;
    }

    let batchq: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).if_batchq);
    let fastq: *mut Mbuf = core::ptr::addr_of_mut!((*slirp).if_fastq);

    /*
     * See if there's already a batchq list for this session.
     */
    let mut done = false;
    ifq = (*slirp).if_batchq.m_hdr.mh_prev;
    while ifq != batchq {
        if so == (*ifq).m_hdr.mh_so {
            /* A match! */
            (*ifm).m_hdr.mh_so = so;
            ifs_insque(ifm, (*ifq).m_hdr.mh_prevpkt);
            done = true;
            break;
        }
        ifq = (*ifq).m_hdr.mh_prev;
    }

    if !done {
        /* No match, check which queue to put it on */
        if !so.is_null() && ((*so).so_iptos & 0x10) != 0 {
            ifq = (*slirp).if_fastq.m_hdr.mh_prev;
            on_fastq = 1;
            /*
             * Check if this packet is a part of the last
             * packet's session
             */
            if (*ifq).m_hdr.mh_so == so {
                (*ifm).m_hdr.mh_so = so;
                ifs_insque(ifm, (*ifq).m_hdr.mh_prevpkt);
                done = true;
            }
        } else {
            ifq = (*slirp).if_batchq.m_hdr.mh_prev;
        }

        if !done {
            /* Create a new doubly linked list for this session */
            (*ifm).m_hdr.mh_so = so;
            /* ifs_init(ifm) */
            (*ifm).m_hdr.mh_nextpkt = ifm;
            (*ifm).m_hdr.mh_prevpkt = ifm;
            slirp_insque(ifm as *mut c_void, ifq as *mut c_void);
        }
    }
    let _ = fastq;

    // diddit:
    (*slirp).if_queued = (*slirp).if_queued.wrapping_add(1);

    if !so.is_null() {
        /* Update *_queued */
        (*so).so_queued = (*so).so_queued.wrapping_add(1);
        (*so).so_nqueued = (*so).so_nqueued.wrapping_add(1);
        /*
         * Check if the interactive session should be downgraded to
         * the batchq.
         */
        if on_fastq != 0
            && (*so).so_nqueued >= 6
            && ((*so).so_nqueued.wrapping_sub((*so).so_queued)) >= 3
        {
            /* Remove from current queue... */
            slirp_remque((*ifm).m_hdr.mh_nextpkt as *mut c_void);

            /* ...And insert in the new.  That'll teach ya! */
            slirp_insque((*ifm).m_hdr.mh_nextpkt as *mut c_void, batchq as *mut c_void);
        }
    }

    if_start((*ifm).slirp);
}
}

// ── `Slirp *slirp = ifm->slirp; struct mbuf *ifq; int on_fastq = 1; ; ; ;` · slirp/if.c:52-58 · chunk 2292 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = ifm->slirp;
//      struct mbuf *ifq;
//      int on_fastq = 1;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2292 {
// Port of the declaration/initialisation part of if_output (slirp/if.c).
// The three empty statements in the original are DEBUG_CALL/DEBUG_ARG macros
// that expand to nothing, so this piece only has to provide the derived
// values of the locals it declares.

use core::ffi::{c_char, c_int};

// struct socket is only ever used through pointers here.
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

// struct Slirp is only ever used through pointers here.
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct M_hdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
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
    pub slirp: *mut Slirp,
    pub m_dat: M_dat,
}

#[no_mangle]
pub extern "C" fn if_output__part2292(
    ifm: *mut Mbuf,
    slirp_out: *mut *mut Slirp,
    on_fastq_out: *mut c_int,
) {
    unsafe {
        // Slirp *slirp = ifm->slirp;
        if !slirp_out.is_null() {
            if ifm.is_null() {
                *slirp_out = core::ptr::null_mut();
            } else {
                *slirp_out = (*ifm).slirp;
            }
        }
        // int on_fastq = 1;
        if !on_fastq_out.is_null() {
            *on_fastq_out = 1;
        }
    }
}
}

// ── `if (ifm->m_hdr.mh_flags & 0x04)` · slirp/if.c:65-68 · chunk 2293 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ifm->m_hdr.mh_flags & 0x04) {
//       slirp_remque(ifm);
//       ifm->m_hdr.mh_flags &= ~0x04;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2293 {
// Ported piece of if_output (slirp/if.c lines 65-68):
//   if (ifm->m_hdr.mh_flags & 0x04) {
//     slirp_remque(ifm);
//     ifm->m_hdr.mh_flags &= ~0x04;
//   }

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
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
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

extern "C" {
    fn slirp_remque(ifm: *mut c_void);
}

#[no_mangle]
pub extern "C" fn if_output__part2293(ifm: *mut Mbuf) {
    if ifm.is_null() {
        return;
    }
    unsafe {
        if ((*ifm).m_hdr.mh_flags & 0x04) != 0 {
            slirp_remque(ifm as *mut c_void);
            (*ifm).m_hdr.mh_flags &= !0x04;
        }
    }
}
}

// ── `for (ifq = slirp->if_batchq.m_hdr.mh_prev; ifq != &slirp->if_batchq; ifq = ifq->m_hdr.mh_prev)` · slirp/if.c:77-85 · chunk 2294 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (ifq = slirp->if_batchq.m_hdr.mh_prev; ifq != &slirp->if_batchq;
//           ifq = ifq->m_hdr.mh_prev) {
//       if (so == ifq->m_hdr.mh_so) {
//     
//        ifm->m_hdr.mh_so = so;
//        ifs_insque(ifm, ifq->m_hdr.mh_prevpkt);
//        goto diddit;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2294 {
// Port of the batchq scan loop from slirp/if.c if_output().
// ifs_insque() is a `static` C helper (no linker symbol), so its logic is
// reproduced here as a private Rust function.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Socket {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: [u8; 8],
}

#[inline]
unsafe fn ifs_insque(ifm: *mut Mbuf, ifmhead: *mut Mbuf) {
    (*(ifm)).m_hdr.mh_nextpkt = (*(ifmhead)).m_hdr.mh_nextpkt;
    (*(ifmhead)).m_hdr.mh_nextpkt = ifm;
    (*(ifm)).m_hdr.mh_prevpkt = ifmhead;
    (*(*(ifm)).m_hdr.mh_nextpkt).m_hdr.mh_prevpkt = ifm;
}

/// Scans the batchq list for an entry belonging to `so`.
/// Returns 1 (the C code's `goto diddit`) when a match was found and ifm was
/// inserted into that session list; otherwise 0, with `*pifq` holding the last
/// visited queue entry for the continuation of the C code.
#[no_mangle]
pub unsafe extern "C" fn if_output__part2294(
    so: *mut Socket,
    ifm: *mut Mbuf,
    batchq: *mut Mbuf,
    pifq: *mut *mut Mbuf,
) -> c_int {
    let mut ifq: *mut Mbuf = (*(batchq)).m_hdr.mh_prev;
    while ifq != batchq {
        if so == (*(ifq)).m_hdr.mh_so {
            (*(ifm)).m_hdr.mh_so = so;
            ifs_insque(ifm, (*(ifq)).m_hdr.mh_prevpkt);
            if !pifq.is_null() {
                *pifq = ifq;
            }
            return 1;
        }
        ifq = (*(ifq)).m_hdr.mh_prev;
    }
    if !pifq.is_null() {
        *pifq = ifq;
    }
    0
}
}

// ── `if (so && (so->so_iptos & 0x10) … else` · slirp/if.c:88-101 · chunk 2295 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so && (so->so_iptos & 0x10)) {
//       ifq = slirp->if_fastq.m_hdr.mh_prev;
//       on_fastq = 1;
//     
//     
//     
//     
//       if (ifq->m_hdr.mh_so == so) {
//        ifm->m_hdr.mh_so = so;
//        ifs_insque(ifm, ifq->m_hdr.mh_prevpkt);
//        goto diddit;
//       }
//      } else
//       ifq = slirp->if_batchq.m_hdr.mh_prev;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2295 {
// Port of the fastq / batchq selection piece of if_output() in slirp/if.c.
// Opaque views of the C structures touched here (only the accessed prefix matters
// for field offsets).

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct MbufHdr {
    mh_next: *mut Mbuf,
    mh_prev: *mut Mbuf,
    mh_nextpkt: *mut Mbuf,
    mh_prevpkt: *mut Mbuf,
    mh_flags: core::ffi::c_int,
    mh_size: core::ffi::c_int,
    mh_so: *mut Socket,
    mh_data: *mut core::ffi::c_char,
    mh_len: core::ffi::c_int,
}

#[repr(C)]
struct Mbuf {
    m_hdr: MbufHdr,
    slirp: *mut Slirp,
}

#[repr(C)]
struct Sbuf {
    sb_cc: core::ffi::c_uint,
    sb_datalen: core::ffi::c_uint,
    sb_wptr: *mut core::ffi::c_char,
    sb_rptr: *mut core::ffi::c_char,
    sb_data: *mut core::ffi::c_char,
}

#[repr(C)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: core::ffi::c_int,
    slirp: *mut Slirp,
    so_m: *mut Mbuf,
    so_ti: *mut core::ffi::c_void,
    so_urgc: core::ffi::c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: core::ffi::c_int,
    so_tcpcb: *mut core::ffi::c_void,
    so_expire: core::ffi::c_uint,
    so_queued: core::ffi::c_int,
    so_nqueued: core::ffi::c_int,
    so_rcv: Sbuf,
    so_snd: Sbuf,
    extra: *mut core::ffi::c_void,
}

#[repr(C)]
struct Timeval {
    tv_sec: core::ffi::c_long,
    tv_usec: core::ffi::c_long,
}

// The TCP/BOOTP/IPq members of Slirp are only modelled as opaque byte blobs so
// that the offsets of if_fastq / if_batchq stay exact.
#[repr(C)]
struct IpAddrPair {
    ip_next: *mut core::ffi::c_void,
    ip_prev: *mut core::ffi::c_void,
    ip_dst: InAddr,
    ip_src: InAddr,
    ip_id: u32,
    ip_type: core::ffi::c_int,
    ip_ttl: core::ffi::c_int,
}

#[repr(C)]
struct BootpClient {
    secs: i32,
    first_time: Timeval,
    expire: i32,
    client_ipaddr: InAddr,
    client_ethaddr: [u8; 6],
}

#[repr(C)]
struct Slirp {
    vnetwork_addr: InAddr,
    vnetwork_mask: InAddr,
    vhost_addr: InAddr,
    vdhcp_startaddr: InAddr,
    vnameserver_addr: InAddr,
    client_ethaddr: [u8; 6],
    client_ipaddr: InAddr,
    client_hostname: [core::ffi::c_char; 33],
    restricted: core::ffi::c_int,
    tt: Timeval,
    exec_list: *mut core::ffi::c_void,
    m_freelist: Mbuf,
    m_usedlist: Mbuf,
    mbuf_alloced: core::ffi::c_int,
    if_queued: core::ffi::c_int,
    if_fastq: Mbuf,
    if_batchq: Mbuf,
    next_m: *mut Mbuf,
    ipq: IpAddrPair,
    ip_id: u16,
    bootp_clients: [BootpClient; 16],
}

// Port of static ifs_insque() (no linker symbol in C).
unsafe fn ifs_insque(ifm: *mut Mbuf, ifmhead: *mut Mbuf) {
    (*ifm).m_hdr.mh_nextpkt = (*ifmhead).m_hdr.mh_nextpkt;
    (*ifmhead).m_hdr.mh_nextpkt = ifm;
    (*ifm).m_hdr.mh_prevpkt = ifmhead;
    (*(*ifm).m_hdr.mh_nextpkt).m_hdr.mh_prevpkt = ifm;
}

/// Returns 1 when the C code has to `goto diddit`, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn if_output__part2295(
    so: *mut Socket,
    ifm: *mut Mbuf,
    slirp: *mut Slirp,
    p_ifq: *mut *mut Mbuf,
    p_on_fastq: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut ifq: *mut Mbuf;
    let goto_diddit: core::ffi::c_int;

    if !so.is_null() && ((*so).so_iptos as core::ffi::c_int & 0x10) != 0 {
        ifq = (*(*slirp).if_fastq.m_hdr.mh_prev).m_hdr.mh_prev;
        *p_on_fastq = 1;

        if (*ifq).m_hdr.mh_so == so {
            (*ifm).m_hdr.mh_so = so;
            ifs_insque(ifm, (*ifq).m_hdr.mh_prevpkt);
            goto_diddit = 1;
        } else {
            goto_diddit = 0;
        }
    } else {
        ifq = (*(*slirp).if_batchq.m_hdr.mh_prev).m_hdr.mh_prev;
        goto_diddit = 0;
    }

    *p_ifq = ifq;
    goto_diddit
}
}

// ── `ifm->m_hdr.mh_so = so; ((ifm)->m_hdr.mh_nextpkt = (ifm)->m_hdr.mh_prevpkt = (ifm)); slirp_insqu…` · slirp/if.c:104-106 · chunk 2296 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ifm->m_hdr.mh_so = so;
//      ((ifm)->m_hdr.mh_nextpkt = (ifm)->m_hdr.mh_prevpkt = (ifm));
//      slirp_insque(ifm, ifq);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2296 {
// Rust port of the mbuf queue-insertion block of if_output() (slirp/if.c).
use core::ffi::c_void;

#[repr(C)]
pub struct socket {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: core::ffi::c_int,
    pub mh_size: core::ffi::c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut core::ffi::c_char,
    pub mh_len: core::ffi::c_int,
}

#[repr(C)]
pub struct M_dat {
    pub m_dat_: [core::ffi::c_char; 1],
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub M_dat: M_dat,
}

extern "C" {
    fn slirp_insque(ifm: *mut c_void, ifq: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn if_output__part2296(
    ifm: *mut mbuf,
    so: *mut socket,
    ifq: *mut mbuf,
) {
    if ifm.is_null() {
        return;
    }
    (*ifm).m_hdr.mh_so = so;
    (*ifm).m_hdr.mh_prevpkt = ifm;
    (*ifm).m_hdr.mh_nextpkt = ifm;
    slirp_insque(ifm as *mut c_void, ifq as *mut c_void);
}
}

// ── `diddit: slirp->if_queued++;` · slirp/if.c:108-109 · chunk 2297 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     diddit:
//      slirp->if_queued++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2297 {
// Increment of Slirp::if_queued (the "diddit" tail of if_output).
// The C side passes the address of the `if_queued` int field directly.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn if_output__part2297(if_queued: *mut c_int) {
    if if_queued.is_null() {
        return;
    }
    unsafe {
        let v = core::ptr::read(if_queued);
        core::ptr::write(if_queued, v.wrapping_add(1));
    }
}
}

// ── `if (so)` · slirp/if.c:111-131 · chunk 2298 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so) {
//     
//       so->so_queued++;
//       so->so_nqueued++;
//     
//     
//     
//     
//     
//     
//     
//       if (on_fastq && ((so->so_nqueued >= 6) &&
//          (so->so_nqueued - so->so_queued) >= 3)) {
//     
//     
//        slirp_remque(ifm->m_hdr.mh_nextpkt);
//     
//     
//        slirp_insque(ifm->m_hdr.mh_nextpkt, &slirp->if_batchq);
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2298 {
// Port of the "update *_queued / maybe downgrade fastq->batchq" block of
// if_output() (slirp/if.c, lines 111-131).
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SbUf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
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
    pub so_m: *mut MbUf,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SbUf,
    pub so_snd: SbUf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut MbUf,
    pub mh_prev: *mut MbUf,
    pub mh_nextpkt: *mut MbUf,
    pub mh_prevpkt: *mut MbUf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct MbUf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void,
    pub M_dat: MDat,
}

extern "C" {
    fn slirp_insque(new_elem: *mut c_void, after: *mut c_void);
    fn slirp_remque(elem: *mut c_void);
}

#[no_mangle]
pub extern "C" fn if_output__part2298(
    so: *mut Socket,
    on_fastq: c_int,
    ifm: *mut MbUf,
    if_batchq: *mut MbUf,
) {
    if so.is_null() {
        return;
    }
    unsafe {
        /* Update *_queued */
        (*so).so_queued = (*so).so_queued.wrapping_add(1);
        (*so).so_nqueued = (*so).so_nqueued.wrapping_add(1);

        /*
         * Check if the interactive session should be downgraded to the
         * batchq.  A session is downgraded if it has queued 6 packets
         * without pausing, and at least 3 of those packets have been sent
         * over the link.
         */
        if on_fastq != 0
            && ((*so).so_nqueued >= 6
                && (*so).so_nqueued.wrapping_sub((*so).so_queued) >= 3)
        {
            /* Remove from current queue... */
            slirp_remque((*ifm).m_hdr.mh_nextpkt as *mut c_void);

            /* ...And insert in the new.  That'll teach ya! */
            slirp_insque((*ifm).m_hdr.mh_nextpkt as *mut c_void, if_batchq as *mut c_void);
        }
    }
}
}

// ── `if_start(ifm->slirp);` · slirp/if.c:137-137 · chunk 2299 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if_start(ifm->slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2299 {
use core::ffi::{c_char, c_int, c_void};

// Mirror of `struct m_hdr` (slirp/slirp.h).
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

// Mirror of `struct mbuf` up to (and including) the `slirp` field, which is
// the only member read here.  The trailing union is not accessed.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut c_void,
}

extern "C" {
    fn if_start(slirp: *mut c_void);
}

#[no_mangle]
pub extern "C" fn if_output__part2299(ifm: *mut Mbuf) {
    unsafe {
        if_start((*ifm).slirp);
    }
}
}

// ── `if_start` · slirp/if.c:153-209 · chunk 2300 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     if_start(Slirp *slirp)
//     {
//      struct mbuf *ifm, *ifqt;
//     
//      ;
//     
//      if (slirp->if_queued == 0)
//         return;
//     
//      again:
//     
//             if (!slirp_can_output(slirp->opaque))
//                 return;
//     
//     
//     
//     
//     
//      if (slirp->if_fastq.m_hdr.mh_next != &slirp->if_fastq) {
//       ifm = slirp->if_fastq.m_hdr.mh_next;
//      } else {
//     
//       if (slirp->next_m != &slirp->if_batchq)
//          ifm = slirp->next_m;
//       else
//          ifm = slirp->if_batchq.m_hdr.mh_next;
//     
//     
//       slirp->next_m = ifm->m_hdr.mh_next;
//      }
//     
//      ifqt = ifm->m_hdr.mh_prev;
//      slirp_remque(ifm);
//      slirp->if_queued--;
//     
//     
//      if (ifm->m_hdr.mh_nextpkt != ifm) {
//       slirp_insque(ifm->m_hdr.mh_nextpkt, ifqt);
//       ifs_remque(ifm);
//      }
//     
//     
//      if (ifm->m_hdr.mh_so) {
//       if (--ifm->m_hdr.mh_so->so_queued == 0)
//     
//          ifm->m_hdr.mh_so->so_nqueued = 0;
//      }
//     
//     
//             if_encap(slirp, (uint8_t *)ifm->m_hdr.mh_data, ifm->m_hdr.mh_len);
//     
//             m_free(ifm);
//     
//      if (slirp->if_queued)
//         goto again;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2300 {
// Rust rewrite of slirp/if.c: `void if_start(Slirp *slirp)`
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

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
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ipq is declared with __attribute__((packed))
#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
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
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
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
    pub tt: Timeval,
    pub exec_list: *mut ExList,

    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,

    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,

    pub ipq: IpQ,
    pub ip_id: u16,

    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,

    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,

    pub udb: Socket,
    pub udp_last_so: *mut Socket,

    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],

    pub opaque: *mut c_void,
}

extern "C" {
    fn slirp_can_output(opaque: *mut c_void) -> c_int;
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    fn slirp_remque(a: *mut c_void);
    fn if_encap(slirp: *mut Slirp, ip_data: *const u8, ip_data_len: c_int);
    fn m_free(m: *mut Mbuf);
}

// static void ifs_remque(struct mbuf *ifm)
unsafe fn ifs_remque(ifm: *mut Mbuf) {
    let prev: *mut Mbuf = (*ifm).m_hdr.mh_prevpkt;
    let next: *mut Mbuf = (*ifm).m_hdr.mh_nextpkt;
    (*prev).m_hdr.mh_nextpkt = next;
    (*next).m_hdr.mh_prevpkt = prev;
}

#[no_mangle]
pub unsafe extern "C" fn if_start(slirp: *mut Slirp) {
    if (*slirp).if_queued == 0 {
        return; // Nothing to do
    }

    loop {
        // check if we can really output
        if slirp_can_output((*slirp).opaque) == 0 {
            return;
        }

        // See which queue to get next packet from
        // If there's something in the fastq, select it immediately
        let ifm: *mut Mbuf;
        if (*slirp).if_fastq.m_hdr.mh_next != core::ptr::addr_of_mut!((*slirp).if_fastq) {
            ifm = (*slirp).if_fastq.m_hdr.mh_next;
        } else {
            // Nothing on fastq, see if next_m is valid
            if (*slirp).next_m != core::ptr::addr_of_mut!((*slirp).if_batchq) {
                ifm = (*slirp).next_m;
            } else {
                ifm = (*slirp).if_batchq.m_hdr.mh_next;
            }

            // Set which packet to send on next iteration
            (*slirp).next_m = (*ifm).m_hdr.mh_next;
        }

        // Remove it from the queue
        let ifqt: *mut Mbuf = (*ifm).m_hdr.mh_prev;
        slirp_remque(ifm as *mut c_void);
        (*slirp).if_queued = (*slirp).if_queued.wrapping_sub(1);

        // If there are more packets for this session, re-queue them
        if (*ifm).m_hdr.mh_nextpkt != ifm {
            slirp_insque(
                (*ifm).m_hdr.mh_nextpkt as *mut c_void,
                ifqt as *mut c_void,
            );
            ifs_remque(ifm);
        }

        // Update so_queued
        let so: *mut Socket = (*ifm).m_hdr.mh_so;
        if !so.is_null() {
            let q = (*so).so_queued.wrapping_sub(1);
            (*so).so_queued = q;
            if q == 0 {
                // If there's no more queued, reset nqueued
                (*so).so_nqueued = 0;
            }
        }

        // Encapsulate the packet for sending
        if_encap(
            slirp,
            (*ifm).m_hdr.mh_data as *const u8,
            (*ifm).m_hdr.mh_len,
        );

        m_free(ifm);

        if (*slirp).if_queued == 0 {
            return;
        }
    }
}
}

// ── `struct mbuf *ifm, *ifqt; ;` · slirp/if.c:156-158 · chunk 2301 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct mbuf *ifm, *ifqt;
//     
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2301 {
// Piece of if_start: the local pointer declarations (plus the trailing empty
// statement, which has no run-time effect). The declared locals are used by the
// rest of the C function, so they stay declared in the C; this Rust function
// only receives their addresses and performs the no-op the TARGET performed.

use core::ffi::{c_char, c_int};

#[repr(C)]
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
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: M_dat,
}

#[no_mangle]
pub extern "C" fn if_start__part2301(ifm: *mut *mut Mbuf, ifqt: *mut *mut Mbuf) {
    // The TARGET left both locals uninitialized; nothing to do.
    let _ = ifm;
    let _ = ifqt;
}
}

// ── `if (slirp->if_queued == 0)` · slirp/if.c:160-161 · chunk 2302 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->if_queued == 0)
//         return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2302 {
// Port of the early-exit check at the top of if_start():
//   if (slirp->if_queued == 0)
//       return;
// The queue count is passed by value; returns nonzero when the caller
// must return immediately.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn if_start__part2302(if_queued: c_int) -> c_int {
    if if_queued == 0 {
        1
    } else {
        0
    }
}
}

// ── `again: if (!slirp_can_output(slirp->opaque)) return;` · slirp/if.c:163-166 · chunk 2303 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     again:
//     
//             if (!slirp_can_output(slirp->opaque))
//                 return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2303 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn slirp_can_output(opaque: *mut c_void) -> c_int;
}

// Returns slirp_can_output(slirp->opaque); nonzero means output is allowed.
#[no_mangle]
pub extern "C" fn if_start__part2303(opaque: *mut c_void) -> c_int {
    unsafe { slirp_can_output(opaque) }
}
}

// ── `if (slirp->if_fastq.m_hdr.mh_next != &slirp->if_fastq) … else` · slirp/if.c:172-183 · chunk 2304 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->if_fastq.m_hdr.mh_next != &slirp->if_fastq) {
//       ifm = slirp->if_fastq.m_hdr.mh_next;
//      } else {
//     
//       if (slirp->next_m != &slirp->if_batchq)
//          ifm = slirp->next_m;
//       else
//          ifm = slirp->if_batchq.m_hdr.mh_next;
//     
//     
//       slirp->next_m = ifm->m_hdr.mh_next;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2304 {
// Port of slirp/if.c if_start() lines 172-183: select the next mbuf to send.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
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

// Leading part of struct Slirp, mirrored exactly up to `next_m`.
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
    pub exec_list: *mut c_void,
    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    // rest of the struct is never touched here
    pub __tail: [u8; 2048],
}

#[no_mangle]
pub extern "C" fn if_start__part2304(slirp: *mut Slirp) -> *mut mbuf {
    unsafe {
        let s = &mut *slirp;
        let ifm: *mut mbuf;
        let fastq: *mut mbuf = &mut s.if_fastq as *mut mbuf;
        if s.if_fastq.m_hdr.mh_next != fastq {
            ifm = s.if_fastq.m_hdr.mh_next;
        } else {
            let batchq: *mut mbuf = &mut s.if_batchq as *mut mbuf;
            if s.next_m != batchq {
                ifm = s.next_m;
            } else {
                ifm = s.if_batchq.m_hdr.mh_next;
            }
            s.next_m = (*ifm).m_hdr.mh_next;
        }
        ifm
    }
}
}

// ── `ifqt = ifm->m_hdr.mh_prev; slirp_remque(ifm); slirp->if_queued--;` · slirp/if.c:185-187 · chunk 2305 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ifqt = ifm->m_hdr.mh_prev;
//      slirp_remque(ifm);
//      slirp->if_queued--;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2305 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MHeader {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
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
pub struct MBuf {
    pub m_hdr: MHeader,
    pub slirp: *mut c_void,
    pub M_dat: M_dat,
}

extern "C" {
    fn slirp_remque(a: *mut c_void);
}

#[no_mangle]
pub extern "C" fn if_start__part2305(
    ifm: *mut MBuf,
    ifqt: *mut *mut MBuf,
    if_queued: *mut c_int,
) {
    unsafe {
        if !ifqt.is_null() {
            *ifqt = (*ifm).m_hdr.mh_prev;
        }
        slirp_remque(ifm as *mut c_void);
        if !if_queued.is_null() {
            *if_queued = (*if_queued).wrapping_sub(1);
        }
    }
}
}

// ── `if (ifm->m_hdr.mh_nextpkt != ifm)` · slirp/if.c:190-193 · chunk 2306 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ifm->m_hdr.mh_nextpkt != ifm) {
//       slirp_insque(ifm->m_hdr.mh_nextpkt, ifqt);
//       ifs_remque(ifm);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2306 {
// Port of the packet-queue fixup piece of if_start().
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
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void,
    pub m_dat: MDat,
}

extern "C" {
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
}

// static void ifs_remque(struct mbuf *ifm) -- inlined from the translation unit
unsafe fn ifs_remque(ifm: *mut Mbuf) {
    let prev = (*ifm).m_hdr.mh_prevpkt;
    let next = (*ifm).m_hdr.mh_nextpkt;
    (*prev).m_hdr.mh_nextpkt = next;
    (*next).m_hdr.mh_prevpkt = prev;
}

#[no_mangle]
pub unsafe extern "C" fn if_start__part2306(ifm: *mut Mbuf, ifqt: *mut Mbuf) {
    if (*ifm).m_hdr.mh_nextpkt != ifm {
        slirp_insque((*ifm).m_hdr.mh_nextpkt as *mut c_void, ifqt as *mut c_void);
        ifs_remque(ifm);
    }
}
}

// ── `if (ifm->m_hdr.mh_so)` · slirp/if.c:196-200 · chunk 2307 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ifm->m_hdr.mh_so) {
//       if (--ifm->m_hdr.mh_so->so_queued == 0)
//     
//          ifm->m_hdr.mh_so->so_nqueued = 0;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2307 {
// Port of the if_start() socket-queue bookkeeping block:
//   if (ifm->m_hdr.mh_so) {
//       if (--ifm->m_hdr.mh_so->so_queued == 0)
//           ifm->m_hdr.mh_so->so_nqueued = 0;
//   }

use core::ffi::{c_char, c_int, c_void};

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

// Mirror of `struct socket`: only the layout matters here.
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
pub struct MBuf {
    pub m_hdr: MHdr,
}

#[no_mangle]
pub unsafe extern "C" fn if_start__part2307(ifm: *mut MBuf) {
    if ifm.is_null() {
        return;
    }
    let so = (*ifm).m_hdr.mh_so;
    if !so.is_null() {
        let queued = (*so).so_queued.wrapping_sub(1);
        (*so).so_queued = queued;
        if queued == 0 {
            (*so).so_nqueued = 0;
        }
    }
}
}

// ── `if_encap(slirp, (uint8_t *)ifm->m_hdr.mh_data, ifm->m_hdr.mh_len); m_free(ifm);` · slirp/if.c:203-205 · chunk 2308 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if_encap(slirp, (uint8_t *)ifm->m_hdr.mh_data, ifm->m_hdr.mh_len);
//     
//             m_free(ifm);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2308 {
// Ported piece of slirp if_start(): the encap + free of the dequeued mbuf.
use core::ffi::{c_char, c_int, c_void};

// Opaque mirror of the C `Slirp` type (never dereferenced here).
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

// Mirror of C `struct m_hdr` (the first member of `struct mbuf`).
#[repr(C)]
pub struct Mhdr {
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

extern "C" {
    fn if_encap(slirp: *mut Slirp, ip_data: *const u8, ip_data_len: c_int);
    fn m_free(m: *mut c_void);
}

#[no_mangle]
pub extern "C" fn if_start__part2308(slirp: *mut Slirp, ifm: *mut c_void) {
    unsafe {
        // `struct m_hdr m_hdr` is the first member of `struct mbuf`.
        let hdr = ifm as *mut Mhdr;
        let data = (*hdr).mh_data as *const u8;
        let len = (*hdr).mh_len;
        if_encap(slirp, data, len);
        m_free(ifm);
    }
}
}

// ── `if (slirp->if_queued)` · slirp/if.c:207-208 · chunk 2309 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->if_queued)
//         goto again;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2309 {
use core::ffi::c_int;

// Port of the tail of if_start():
//   if (slirp->if_queued) goto again;
// The C side keeps the `goto again`; this returns whether the loop must repeat.
#[no_mangle]
pub extern "C" fn if_start__part2309(if_queued: *const c_int) -> c_int {
    let v = unsafe { *if_queued };
    if v != 0 {
        1
    } else {
        0
    }
}
}

