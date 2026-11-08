//! From `slirp/socket.c`.

// ── `solookup` · slirp/socket.c:14-32 · chunk 2562 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *
//     solookup(struct socket *head, struct in_addr laddr, u_int lport,
//              struct in_addr faddr, u_int fport)
//     {
//      struct socket *so;
//     
//      for (so = head->so_next; so != head; so = so->so_next) {
//       if (so->so_lport == lport &&
//           so->so_laddr.s_addr == laddr.s_addr &&
//           so->so_faddr.s_addr == faddr.s_addr &&
//           so->so_fport == fport)
//          break;
//      }
//     
//      if (so == head)
//         return (struct socket *)
//                                ((void *)0)
//                                    ;
//      return so;
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2562 {
// Rust rewrite of slirp's solookup().
use core::ffi::{c_int, c_uint, c_ushort, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// Prefix of the C `struct socket` (only the fields this function touches).
#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: c_ushort,
    pub so_lport: c_ushort,
}

#[no_mangle]
pub unsafe extern "C" fn solookup(
    head: *mut socket,
    laddr: in_addr,
    lport: c_uint,
    faddr: in_addr,
    fport: c_uint,
) -> *mut socket {
    let mut so: *mut socket = (*head).so_next;

    while so != head {
        if (*so).so_lport as c_uint == lport
            && (*so).so_laddr.s_addr == laddr.s_addr
            && (*so).so_faddr.s_addr == faddr.s_addr
            && (*so).so_fport as c_uint == fport
        {
            break;
        }
        so = (*so).so_next;
    }

    if so == head {
        return core::ptr::null_mut();
    }
    so
}
}

// ── `for (so = head->so_next; so != head; so = so->so_next)` · slirp/socket.c:20-26 · chunk 2563 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (so = head->so_next; so != head; so = so->so_next) {
//       if (so->so_lport == lport &&
//           so->so_laddr.s_addr == laddr.s_addr &&
//           so->so_faddr.s_addr == faddr.s_addr &&
//           so->so_fport == fport)
//          break;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2563 {
// Port of the lookup loop of solookup() from slirp/socket.c
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
#[derive(Clone, Copy)]
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
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut core::ffi::c_void,
}

/// Walks the circular socket list starting at head->so_next, returning the
/// first socket matching (laddr, lport, faddr, fport), or head if none match.
#[no_mangle]
pub extern "C" fn solookup__part2563(
    head: *mut socket,
    laddr: in_addr,
    lport: c_uint,
    faddr: in_addr,
    fport: c_uint,
) -> *mut socket {
    unsafe {
        let mut so: *mut socket = (*head).so_next;
        while so != head {
            if (*so).so_lport as c_uint == lport
                && (*so).so_laddr.s_addr == laddr.s_addr
                && (*so).so_faddr.s_addr == faddr.s_addr
                && (*so).so_fport as c_uint == fport
            {
                break;
            }
            so = (*so).so_next;
        }
        so
    }
}
}

// ── `if (so == head)` · slirp/socket.c:28-29 · chunk 2564 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so == head)
//         return (struct socket *)
//                                ((void *)0)
//                                    ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2564 {
// Port of the tail-check of solookup(): does the search pointer come back to head?
// Only pointers are compared, so the struct socket contents are irrelevant here.

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn solookup__part2564(
    so: *mut socket,
    head: *mut socket,
) -> core::ffi::c_int {
    if so == head {
        1
    } else {
        0
    }
}
}

// ── `return so;` · slirp/socket.c:30-30 · chunk 2565 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return so;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2565 {
// Identity helper: returns the socket pointer unchanged.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn solookup__part2565(so: *mut c_void) -> *mut c_void {
    so
}
}

// ── `socreate` · slirp/socket.c:39-52 · chunk 2566 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *
//     socreate(Slirp *slirp)
//     {
//       struct socket *so;
//     
//       so = (struct socket *)malloc(sizeof(struct socket));
//       if(so) {
//         memset(so, 0, sizeof(struct socket));
//         so->so_state = 0x001;
//         so->s = -1;
//         so->slirp = slirp;
//       }
//       return(so);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2566 {
// Port of slirp's socreate(): allocate and initialise a struct socket.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of the C `struct sbuf` (only layout matters).
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

// Mirror of the C `struct socket`, field for field.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

const SS_NOFDREF: c_int = 0x001;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn socreate(slirp: *mut Slirp) -> *mut Socket {
    let size = core::mem::size_of::<Socket>();
    let so = malloc(size) as *mut Socket;
    if !so.is_null() {
        memset(so as *mut c_void, 0, size);
        (*so).so_state = SS_NOFDREF;
        (*so).s = -1;
        (*so).slirp = slirp;
    }
    so
}
}

// ── `struct socket *so; so = (struct socket *)malloc(sizeof(struct socket));` · slirp/socket.c:42-44 · chunk 2567 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *so;
//     
//       so = (struct socket *)malloc(sizeof(struct socket));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2567 {
// Allocate a socket-sized block; mirrors `(struct socket *)malloc(sizeof(struct socket))`.
use core::ffi::c_void;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn socreate__part2567(size: usize) -> *mut c_void {
    unsafe { malloc(size) }
}
}

// ── `if (so)` · slirp/socket.c:45-50 · chunk 2568 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if(so) {
//         memset(so, 0, sizeof(struct socket));
//         so->so_state = 0x001;
//         so->s = -1;
//         so->slirp = slirp;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2568 {
use core::ffi::{c_int, c_uchar, c_uint, c_ushort, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
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
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn socreate__part2568(
    so: *mut Socket,
    slirp: *mut c_void,
    size: usize,
) -> c_int {
    if !so.is_null() {
        memset(so as *mut c_void, 0, size);
        (*so).so_state = 0x001;
        (*so).s = -1;
        (*so).slirp = slirp;
        1
    } else {
        0
    }
}
}

// ── `return(so);` · slirp/socket.c:51-51 · chunk 2569 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2569 {
// Port of the `return(so);` statement of socreate() (slirp/socket.c:51).
// The socket object itself stays in C; this only hands the pointer back.

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn socreate__part2569(so: *mut socket) -> *mut socket {
    so
}
}

// ── `sofree` · slirp/socket.c:57-77 · chunk 2570 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sofree(struct socket *so)
//     {
//       Slirp *slirp = so->slirp;
//     
//       if (so->so_emu==0x8 && so->extra) {
//      sofree(so->extra);
//      so->extra=
//               ((void *)0)
//                   ;
//       }
//       if (so == slirp->tcp_last_so) {
//           slirp->tcp_last_so = &slirp->tcb;
//       } else if (so == slirp->udp_last_so) {
//           slirp->udp_last_so = &slirp->udb;
//       }
//       m_free(so->so_m);
//     
//       if(so->so_next && so->so_prev)
//         slirp_remque(so);
//     
//       free(so);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2570 {
// Rust rewrite of slirp/socket.c:sofree
#![allow(non_snake_case, non_camel_case_types, dead_code, clippy::all)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

pub type u_int = c_uint;
pub type u_char = core::ffi::c_uchar;
pub type caddr_t = *mut c_char;
pub type tcp_seq = u32;

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
    pub mh_so: *mut socket,
    pub mh_data: caddr_t,
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
    pub m_dat: M_dat,
}

#[repr(C, packed)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u_int,
    pub sb_datalen: u_int,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct tcpcb {
    // opaque: never dereferenced here
    _opaque: [u8; 0],
}

pub type tcpcb_t = tcpcb;

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u_char,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: u_int,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
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
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    pub ipq: ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: socket,
    pub tcp_last_so: *mut socket,
    pub tcp_iss: tcp_seq,
    pub tcp_now: u32,
    pub udb: socket,
    pub udp_last_so: *mut socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn m_free(m: *mut mbuf);
    fn slirp_remque(q: *mut c_void);
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn sofree(so: *mut socket) {
    let slirp = (*so).slirp;

    if (*so).so_emu == 0x8 && !(*so).extra.is_null() {
        sofree((*so).extra as *mut socket);
        (*so).extra = core::ptr::null_mut();
    }
    if so == (*slirp).tcp_last_so {
        (*slirp).tcp_last_so = core::ptr::addr_of_mut!((*slirp).tcb);
    } else if so == (*slirp).udp_last_so {
        (*slirp).udp_last_so = core::ptr::addr_of_mut!((*slirp).udb);
    }
    m_free((*so).so_m);

    if !(*so).so_next.is_null() && !(*so).so_prev.is_null() {
        slirp_remque(so as *mut c_void);
    }

    free(so as *mut c_void);
}
}

// ── `if (so->so_emu==0x8 && so->extra)` · slirp/socket.c:62-65 · chunk 2571 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_emu==0x8 && so->extra) {
//      sofree(so->extra);
//      so->extra=
//               ((void *)0)
//                   ;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2571 {
// Ported piece of slirp's sofree(): the emu-socket recursion.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sb {
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
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

extern "C" {
    fn sofree(so: *mut Socket);
}

#[no_mangle]
pub unsafe extern "C" fn sofree__part2571(so: *mut Socket) -> c_int {
    if (*so).so_emu == 0x8 && !(*so).extra.is_null() {
        sofree((*so).extra as *mut Socket);
        (*so).extra = core::ptr::null_mut();
        1
    } else {
        0
    }
}
}

// ── `if (so == slirp->tcp_last_so) … else` · slirp/socket.c:66-70 · chunk 2572 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so == slirp->tcp_last_so) {
//           slirp->tcp_last_so = &slirp->tcb;
//       } else if (so == slirp->udp_last_so) {
//           slirp->udp_last_so = &slirp->udb;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2572 {
// Rust port of the "reset last-so cursor" piece of slirp's sofree().
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MHeader {
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
#[derive(Copy, Clone)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mbuf {
    pub m_hdr: MHeader,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub ipq: Ipq,
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

#[no_mangle]
pub extern "C" fn sofree__part2572(so: *mut Socket, slirp: *mut Slirp) {
    unsafe {
        if so == (*slirp).tcp_last_so {
            (*slirp).tcp_last_so = core::ptr::addr_of_mut!((*slirp).tcb);
        } else if so == (*slirp).udp_last_so {
            (*slirp).udp_last_so = core::ptr::addr_of_mut!((*slirp).udb);
        }
    }
}
}

// ── `m_free(so->so_m);` · slirp/socket.c:71-71 · chunk 2573 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m_free(so->so_m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2573 {
// Port of m_free(so->so_m) from slirp's sofree().
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
}

// Prefix of the C `struct socket` up to (and including) the field we read.
#[repr(C)]
pub struct SocketPrefix {
    pub so_next: *mut SocketPrefix,
    pub so_prev: *mut SocketPrefix,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut Mbuf,
}

extern "C" {
    fn m_free(m: *mut Mbuf);
}

#[no_mangle]
pub unsafe extern "C" fn sofree__part2573(so: *mut SocketPrefix) {
    if so.is_null() {
        return;
    }
    let m = (*so).so_m;
    m_free(m);
}
}

// ── `if (so->so_next && so->so_prev)` · slirp/socket.c:73-74 · chunk 2574 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(so->so_next && so->so_prev)
//         slirp_remque(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2574 {
// Port of the queue-unlink check from sofree() (slirp/socket.c lines 73-74).

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
}

extern "C" {
    // void slirp_remque(void *);
    fn slirp_remque(a: *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn sofree__part2574(so: *mut Socket) {
    if so.is_null() {
        return;
    }
    let next = (*so).so_next;
    let prev = (*so).so_prev;
    if !next.is_null() && !prev.is_null() {
        slirp_remque(so as *mut core::ffi::c_void);
    }
}
}

// ── `free(so);` · slirp/socket.c:76-76 · chunk 2575 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2575 {
// Port of the `free(so);` statement at the end of sofree().
use core::ffi::c_void;

extern "C" {
    // libc free
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn sofree__part2575(so: *mut c_void) {
    unsafe {
        free(so);
    }
}
}

// ── `sopreprbuf` · slirp/socket.c:79-136 · chunk 2576 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     size_t sopreprbuf(struct socket *so, struct iovec *iov, int *np)
//     {
//      int n, lss, total;
//      struct sbuf *sb = &so->so_snd;
//      int len = sb->sb_datalen - sb->sb_cc;
//      int mss = so->so_tcpcb->t_maxseg;
//     
//      ;
//      ;
//     
//      if (len <= 0)
//       return 0;
//     
//      iov[0].iov_base = sb->sb_wptr;
//             iov[1].iov_base = 
//                              ((void *)0)
//                                  ;
//             iov[1].iov_len = 0;
//      if (sb->sb_wptr < sb->sb_rptr) {
//       iov[0].iov_len = sb->sb_rptr - sb->sb_wptr;
//     
//       if (iov[0].iov_len > len)
//          iov[0].iov_len = len;
//       if (iov[0].iov_len > mss)
//          iov[0].iov_len -= iov[0].iov_len%mss;
//       n = 1;
//      } else {
//       iov[0].iov_len = (sb->sb_data + sb->sb_datalen) - sb->sb_wptr;
//     
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       len -= iov[0].iov_len;
//       if (len) {
//        iov[1].iov_base = sb->sb_data;
//        iov[1].iov_len = sb->sb_rptr - sb->sb_data;
//        if(iov[1].iov_len > len)
//           iov[1].iov_len = len;
//        total = iov[0].iov_len + iov[1].iov_len;
//        if (total > mss) {
//         lss = total%mss;
//         if (iov[1].iov_len > lss) {
//          iov[1].iov_len -= lss;
//          n = 2;
//         } else {
//          lss -= iov[1].iov_len;
//          iov[0].iov_len -= lss;
//          n = 1;
//         }
//        } else
//         n = 2;
//       } else {
//        if (iov[0].iov_len > mss)
//           iov[0].iov_len -= iov[0].iov_len%mss;
//        n = 1;
//       }
//      }
//      if (np)
//       *np = n;
//     
//      return iov[0].iov_len + (n - 1) * iov[1].iov_len;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2576 {
// Rust rewrite of slirp/socket.c:sopreprbuf
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
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
pub struct InAddr {
    pub s_addr: u32,
}

// Only the leading fields of struct tcpcb are needed; the layout up to
// t_maxseg matches the C definition exactly.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
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
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn sopreprbuf(so: *mut Socket, iov: *mut Iovec, np: *mut c_int) -> usize {
    let sb: *mut Sbuf = core::ptr::addr_of_mut!((*so).so_snd);

    let sb_cc = (*sb).sb_cc;
    let sb_datalen = (*sb).sb_datalen;
    let mut len: c_int = sb_datalen.wrapping_sub(sb_cc) as c_int;
    let mss: c_int = (*((*so).so_tcpcb)).t_maxseg as c_int;

    if len <= 0 {
        return 0;
    }

    // mss comes from an unsigned short, so it is non-negative here.
    let mss_u: usize = mss as u32 as usize;

    let iov0 = iov;
    let iov1 = iov.add(1);

    let wptr = (*sb).sb_wptr;
    let rptr = (*sb).sb_rptr;
    let data = (*sb).sb_data;

    (*iov0).iov_base = wptr as *mut c_void;
    (*iov1).iov_base = core::ptr::null_mut();
    (*iov1).iov_len = 0;

    let mut l0: usize;
    let mut l1: usize = 0;
    let n: c_int;

    if (wptr as usize) < (rptr as usize) {
        l0 = (rptr as isize).wrapping_sub(wptr as isize) as usize;
        // Should never succeed, but...
        if l0 > len as usize {
            l0 = len as usize;
        }
        if l0 > mss_u && mss_u != 0 {
            l0 -= l0 % mss_u;
        }
        n = 1;
    } else {
        l0 = (data as isize)
            .wrapping_add(sb_datalen as isize)
            .wrapping_sub(wptr as isize) as usize;
        // Should never succeed, but...
        if l0 > len as usize {
            l0 = len as usize;
        }
        len = len.wrapping_sub(l0 as c_int);
        if len != 0 {
            (*iov1).iov_base = data as *mut c_void;
            l1 = (rptr as isize).wrapping_sub(data as isize) as usize;
            if l1 > len as usize {
                l1 = len as usize;
            }
            let total: c_int = l0.wrapping_add(l1) as c_int;
            if total > mss && mss != 0 {
                let lss: c_int = total % mss;
                if l1 > lss as usize {
                    l1 -= lss as usize;
                    n = 2;
                } else {
                    let mut lss_u: usize = lss as usize;
                    lss_u -= l1;
                    l0 -= lss_u;
                    n = 1;
                }
            } else {
                n = 2;
            }
        } else {
            if l0 > mss_u && mss_u != 0 {
                l0 -= l0 % mss_u;
            }
            n = 1;
        }
    }

    (*iov0).iov_len = l0;
    (*iov1).iov_len = l1;

    if !np.is_null() {
        *np = n;
    }

    l0.wrapping_add((n as usize).wrapping_sub(1).wrapping_mul(l1))
}
}

// ── `int n, lss, total; struct sbuf *sb = &so->so_snd; int len = sb->sb_datalen - sb->sb_cc; int mss…` · slirp/socket.c:81-87 · chunk 2577 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int n, lss, total;
//      struct sbuf *sb = &so->so_snd;
//      int len = sb->sb_datalen - sb->sb_cc;
//      int mss = so->so_tcpcb->t_maxseg;
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2577 {
// Port of the local-variable setup part of sopreprbuf() from slirp/socket.c.
// Computes:
//   struct sbuf *sb = &so->so_snd;
//   int len = sb->sb_datalen - sb->sb_cc;
//   int mss = so->so_tcpcb->t_maxseg;
// and hands them back to the C caller.

use core::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void};

// struct sbuf from slirp/sbuf.h (preprocessed)
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// struct in_addr from <netinet/in.h> (preprocessed)
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct tcpcb from slirp/tcp_var.h (preprocessed); only fields up to the
// one being read (t_maxseg) need to be exact.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
}

// struct socket from slirp/socket.h (preprocessed)
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
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn sopreprbuf__part2577(
    so: *mut Socket,
    psb: *mut *mut Sbuf,
    plen: *mut c_int,
    pmss: *mut c_int,
) {
    unsafe {
        let sb = core::ptr::addr_of_mut!((*so).so_snd);
        *psb = sb;

        // int len = sb->sb_datalen - sb->sb_cc;
        // The subtraction is unsigned (u_int); the conversion to int wraps.
        let len: c_int = (*sb).sb_datalen.wrapping_sub((*sb).sb_cc) as c_int;
        *plen = len;

        // int mss = so->so_tcpcb->t_maxseg;
        let tp = (*so).so_tcpcb;
        let mss: c_int = (*tp).t_maxseg as c_int;
        *pmss = mss;
    }
}
}

// ── `if (len <= 0)` · slirp/socket.c:89-90 · chunk 2578 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len <= 0)
//       return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2578 {
// Port of the `if (len <= 0) return 0;` guard inside sopreprbuf().
// Returns nonzero when the caller must return 0 from sopreprbuf.
#[no_mangle]
pub extern "C" fn sopreprbuf__part2578(len: core::ffi::c_int) -> core::ffi::c_int {
    if len <= 0 {
        1
    } else {
        0
    }
}
}

// ── `iov[0].iov_base = sb->sb_wptr; iov[1].iov_base = ((void *)0) ; iov[1].iov_len = 0;` · slirp/socket.c:92-94 · chunk 2579 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     iov[0].iov_base = sb->sb_wptr;
//             iov[1].iov_base = 
//                              ((void *)0)
//                                  ;
//             iov[1].iov_len = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2579 {
// Part of sopreprbuf: initialize the two iovec entries for the send buffer.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn sopreprbuf__part2579(iov: *mut Iovec, wptr: *mut c_char) {
    // iov[0].iov_base = sb->sb_wptr;
    (*iov).iov_base = wptr as *mut c_void;

    // iov[1].iov_base = NULL; iov[1].iov_len = 0;
    let second = iov.add(1);
    (*second).iov_base = core::ptr::null_mut();
    (*second).iov_len = 0;
}
}

// ── `if (sb->sb_wptr < sb->sb_rptr) … else` · slirp/socket.c:95-131 · chunk 2580 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_wptr < sb->sb_rptr) {
//       iov[0].iov_len = sb->sb_rptr - sb->sb_wptr;
//     
//       if (iov[0].iov_len > len)
//          iov[0].iov_len = len;
//       if (iov[0].iov_len > mss)
//          iov[0].iov_len -= iov[0].iov_len%mss;
//       n = 1;
//      } else {
//       iov[0].iov_len = (sb->sb_data + sb->sb_datalen) - sb->sb_wptr;
//     
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       len -= iov[0].iov_len;
//       if (len) {
//        iov[1].iov_base = sb->sb_data;
//        iov[1].iov_len = sb->sb_rptr - sb->sb_data;
//        if(iov[1].iov_len > len)
//           iov[1].iov_len = len;
//        total = iov[0].iov_len + iov[1].iov_len;
//        if (total > mss) {
//         lss = total%mss;
//         if (iov[1].iov_len > lss) {
//          iov[1].iov_len -= lss;
//          n = 2;
//         } else {
//          lss -= iov[1].iov_len;
//          iov[0].iov_len -= lss;
//          n = 1;
//         }
//        } else
//         n = 2;
//       } else {
//        if (iov[0].iov_len > mss)
//           iov[0].iov_len -= iov[0].iov_len%mss;
//        n = 1;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2580 {
// Port of the iovec-preparation block of sopreprbuf() (slirp/socket.c).
// Builds iov[0]/iov[1] from the socket send buffer and returns the number of
// iovec entries used (the C local `n`).  iov[0].iov_len / iov[1].iov_base /
// iov[1].iov_len are written through the caller's array.

use core::ffi::{c_char, c_int, c_uint, c_void};

// Mirror of `struct sbuf` (slirp/sbuf.h) exactly, field order and types.
#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Mirror of `struct iovec` (sys/uio.h); size_t == usize on this target.
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

// size_t (unsigned) modulo without any trap: b == 0 is UB in C, stay panic-free.
#[inline]
fn usize_rem(a: usize, b: usize) -> usize {
    if b == 0 {
        0
    } else {
        a % b
    }
}

// int modulo with C truncating semantics, but never panicking.
#[inline]
fn int_rem(a: c_int, b: c_int) -> c_int {
    match a.checked_rem(b) {
        Some(v) => v,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sopreprbuf__part2580(
    sb: *mut sbuf,
    iov: *mut iovec,
    len_in: c_int,
    mss: c_int,
) -> c_int {
    let mut len: c_int = len_in;
    let n: c_int;

    let wptr = (*sb).sb_wptr;
    let rptr = (*sb).sb_rptr;
    let data = (*sb).sb_data;
    let datalen = (*sb).sb_datalen;

    let iov0: *mut iovec = iov;
    let iov1: *mut iovec = iov.add(1);

    if (wptr as usize) < (rptr as usize) {
        let mut l: usize = (rptr as usize).wrapping_sub(wptr as usize);

        if l > len as usize {
            l = len as usize;
        }
        if l > mss as usize {
            l = l.wrapping_sub(usize_rem(l, mss as usize));
        }
        (*iov0).iov_len = l;
        n = 1;
    } else {
        let mut l: usize =
            (data as usize).wrapping_add(datalen as usize).wrapping_sub(wptr as usize);

        if l > len as usize {
            l = len as usize;
        }
        (*iov0).iov_len = l;
        // len -= iov[0].iov_len;  (computed in size_t, stored back into int)
        len = (len as usize).wrapping_sub(l) as c_int;
        if len != 0 {
            (*iov1).iov_base = data as *mut c_void;
            let mut l1: usize = (rptr as usize).wrapping_sub(data as usize);
            if l1 > len as usize {
                l1 = len as usize;
            }
            (*iov1).iov_len = l1;

            let total: c_int = l.wrapping_add(l1) as c_int;
            if total > mss {
                let lss: c_int = int_rem(total, mss);
                if l1 > lss as usize {
                    (*iov1).iov_len = l1.wrapping_sub(lss as usize);
                    n = 2;
                } else {
                    // lss -= iov[1].iov_len;  (size_t arithmetic, back into int)
                    let lss2: c_int = (lss as usize).wrapping_sub(l1) as c_int;
                    (*iov0).iov_len = l.wrapping_sub(lss2 as usize);
                    n = 1;
                }
            } else {
                n = 2;
            }
        } else {
            if l > mss as usize {
                (*iov0).iov_len = l.wrapping_sub(usize_rem(l, mss as usize));
            }
            n = 1;
        }
    }

    n
}
}

// ── `if (np)` · slirp/socket.c:132-133 · chunk 2581 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (np)
//       *np = n;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2581 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sopreprbuf__part2581(np: *mut c_int, n: c_int) {
    if !np.is_null() {
        unsafe {
            *np = n;
        }
    }
}
}

// ── `return iov[0].iov_len + (n - 1) * iov[1].iov_len;` · slirp/socket.c:135-135 · chunk 2582 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return iov[0].iov_len + (n - 1) * iov[1].iov_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2582 {
// Compute the size reported by sopreprbuf from the filled iovec array.
#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut core::ffi::c_void,
    pub iov_len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn sopreprbuf__part2582(iov: *const Iovec, n: core::ffi::c_int) -> usize {
    let len0 = (*iov).iov_len;
    let len1 = (*iov.add(1)).iov_len;
    // (n - 1) is an int in C, converted to size_t for the multiplication.
    let k = (n.wrapping_sub(1)) as usize;
    len0.wrapping_add(k.wrapping_mul(len1))
}
}

// ── `soread` · slirp/socket.c:143-202 · chunk 2583 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     soread(struct socket *so)
//     {
//      int n, nn;
//      struct sbuf *sb = &so->so_snd;
//      struct iovec iov[2];
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//      sopreprbuf(so, iov, &n);
//     
//     
//     
//     
//     
//      nn = recv(so->s, iov[0].iov_base, iov[0].iov_len,0);
//     
//      if (nn <= 0) {
//       if (nn < 0 && (
//                     (*__errno_location ()) 
//                           == 
//                              4 
//                                    || 
//                                       (*__errno_location ()) 
//                                             == 
//                                                11
//                                                      ))
//        return 0;
//       else {
//        ;
//        sofcantrcvmore(so);
//        tcp_sockclosed(((so)->so_tcpcb));
//        return -1;
//       }
//      }
//      if (n == 2 && nn == iov[0].iov_len) {
//                 int ret;
//                 ret = recv(so->s, iov[1].iov_base, iov[1].iov_len,0);
//                 if (ret > 0)
//                     nn += ret;
//             }
//     
//      ;
//     
//     
//     
//      sb->sb_cc += nn;
//      sb->sb_wptr += nn;
//      if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_wptr -= sb->sb_datalen;
//      return nn;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2583 {
// Rust port of slirp's soread() plus the file-static helper sofcantrcvmore().
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
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
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
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

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [c_long; 1024 / (8 * 8)],
}

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_FCANTRCVMORE: c_int = 0x008;
const SS_FCANTSENDMORE: c_int = 0x010;
const SS_PERSISTENT_MASK: c_int = 0xf000;

const EINTR: c_int = 4;
const EAGAIN: c_int = 11;

extern "C" {
    fn sopreprbuf(so: *mut socket, iov: *mut iovec, np: *mut c_int) -> usize;
    fn tcp_sockclosed(tp: *mut c_void);
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
    static mut global_writefds: *mut fd_set;
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

// FD_CLR(fd, set) -- same bit arithmetic as glibc's macro.
unsafe fn fd_clr(fd: c_int, set: *mut fd_set) {
    let bits: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
    let idx = (fd / bits) as isize;
    let bit = (fd % bits) as u32;
    let p = (*set).fds_bits.as_mut_ptr().offset(idx);
    *p &= !((1 as c_long).wrapping_shl(bit));
}

// Ported from the static C helper (no linker symbol).
unsafe fn sofcantrcvmore(so: *mut socket) {
    if ((*so).so_state & SS_NOFDREF) == 0 {
        shutdown((*so).s, 0);
        let wfds = global_writefds;
        if !wfds.is_null() {
            fd_clr((*so).s, wfds);
        }
    }
    (*so).so_state &= !SS_ISFCONNECTING;
    if ((*so).so_state & SS_FCANTSENDMORE) != 0 {
        (*so).so_state &= SS_PERSISTENT_MASK;
        (*so).so_state |= SS_NOFDREF;
    } else {
        (*so).so_state |= SS_FCANTRCVMORE;
    }
}

#[no_mangle]
pub unsafe extern "C" fn soread(so: *mut socket) -> c_int {
    let n: c_int;
    let nn: c_int;
    let sb: *mut sbuf = ptr::addr_of_mut!((*so).so_snd);
    let mut iov: [iovec; 2] = [
        iovec { iov_base: ptr::null_mut(), iov_len: 0 },
        iovec { iov_base: ptr::null_mut(), iov_len: 0 },
    ];
    let mut n_out: c_int = 0;

    sopreprbuf(so, iov.as_mut_ptr(), &mut n_out);
    n = n_out;

    let ret = recv(
        (*so).s,
        iov[0].iov_base,
        iov[0].iov_len,
        0,
    );
    nn = ret as c_int;

    if nn <= 0 {
        if nn < 0 && (errno() == EINTR || errno() == EAGAIN) {
            return 0;
        } else {
            sofcantrcvmore(so);
            tcp_sockclosed((*so).so_tcpcb);
            return -1;
        }
    }

    let mut nn = nn;
    if n == 2 && (nn as usize) == iov[0].iov_len {
        let ret2 = recv(
            (*so).s,
            iov[1].iov_base,
            iov[1].iov_len,
            0,
        ) as c_int;
        if ret2 > 0 {
            nn = nn.wrapping_add(ret2);
        }
    }

    (*sb).sb_cc = (*sb).sb_cc.wrapping_add(nn as c_uint);
    (*sb).sb_wptr = (*sb).sb_wptr.offset(nn as isize);
    if (*sb).sb_wptr >= (*sb).sb_data.offset((*sb).sb_datalen as isize) {
        (*sb).sb_wptr = (*sb).sb_wptr.offset(-((*sb).sb_datalen as isize));
    }
    nn
}
}

// ── `int n, nn; struct sbuf *sb = &so->so_snd; struct iovec iov[2]; ; ; sopreprbuf(so, iov, &n); nn…` · slirp/socket.c:146-163 · chunk 2584 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int n, nn;
//      struct sbuf *sb = &so->so_snd;
//      struct iovec iov[2];
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//      sopreprbuf(so, iov, &n);
//     
//     
//     
//     
//     
//      nn = recv(so->s, iov[0].iov_base, iov[0].iov_len,0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2584 {
// Rust counterpart of the first part of slirp/socket.c soread():
// declares n/nn/sb/iov, prepares the send buffer iovec via sopreprbuf()
// and performs the first recv().

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct Sb {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[repr(C)]
pub struct InAddr {
    s_addr: u32,
}

#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut c_void,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: Sb,
    so_snd: Sb,
    extra: *mut c_void,
}

#[repr(C)]
pub struct Iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

extern "C" {
    fn sopreprbuf(so: *mut Socket, iov: *mut Iovec, np: *mut c_int) -> usize;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn soread__part2584(
    so: *mut Socket,
    iov: *mut Iovec,
    np: *mut c_int,
) -> c_int {
    // DEBUG_CALL / DEBUG_ARG expand to nothing
    sopreprbuf(so, iov, np);

    let fd = (*so).s;
    let base = (*iov).iov_base;
    let len = (*iov).iov_len;

    let nn = recv(fd, base, len, 0) as c_int;

    nn
}
}

// ── `if (nn <= 0)` · slirp/socket.c:165-174 · chunk 2585 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (nn <= 0) {
//       if (nn < 0 && (
//                     (*__errno_location ()) 
//                           == 
//                              4 
//                                    || 
//                                       (*__errno_location ()) 
//                                             == 
//                                                11
//                                                      ))
//        return 0;
//       else {
//        ;
//        sofcantrcvmore(so);
//        tcp_sockclosed(((so)->so_tcpcb));
//        return -1;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2585 {
// Port of the error-handling part of soread() from slirp/socket.c, plus the
// static helper sofcantrcvmore() (no linker symbol, so its logic lives here).

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

// fd_set: struct { __fd_mask fds_bits[1024 / (8 * sizeof(__fd_mask))]; }
// __fd_mask is `long int`; 1024 / 64 == 16 on this target.
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16],
}

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn tcp_sockclosed(tp: *mut c_void);
    static mut global_writefds: *mut FdSet;
}

unsafe fn sofcantrcvmore(so: *mut Socket) {
    if ((*so).so_state & 0x001) == 0 {
        let s = (*so).s;
        shutdown(s, 0);
        let fds = global_writefds;
        if !fds.is_null() {
            let word_bits: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
            let idx: c_int = s / word_bits;
            let sh: c_int = s % word_bits;
            if idx >= 0 && (idx as usize) < 16 {
                let mask: c_long = (1u64.wrapping_shl(sh as u32)) as c_long;
                (*fds).fds_bits[idx as usize] &= !mask;
            }
        }
    }
    (*so).so_state &= !0x002;
    if ((*so).so_state & 0x010) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x008;
    }
}

// Return 1 to continue in soread, otherwise the value soread must return.
#[no_mangle]
pub extern "C" fn soread__part2585(so: *mut Socket, nn: c_int) -> c_int {
    if nn > 0 {
        return 1;
    }
    if nn < 0 {
        let err = unsafe { *__errno_location() };
        if err == 4 || err == 11 {
            return 0;
        }
    }
    unsafe {
        sofcantrcvmore(so);
        let tp: *mut c_void = if so.is_null() {
            core::ptr::null_mut()
        } else {
            (*so).so_tcpcb
        };
        tcp_sockclosed(tp);
    }
    -1
}
}

// ── `if (n == 2 && nn == iov[0].iov_len)` · slirp/socket.c:186-191 · chunk 2586 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n == 2 && nn == iov[0].iov_len) {
//                 int ret;
//                 ret = recv(so->s, iov[1].iov_base, iov[1].iov_len,0);
//                 if (ret > 0)
//                     nn += ret;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2586 {
// Port of the second-read block of slirp's soread() (slirp/socket.c lines 186-191).
// Reads at most one more chunk into iov[1] when the first recv filled iov[0]
// and slirp prepared two buffers, and reports the updated byte count.

use core::ffi::{c_int, c_void};

extern "C" {
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
}

#[no_mangle]
pub extern "C" fn soread__part2586(
    fd: c_int,
    iov1_base: *mut c_void,
    iov1_len: usize,
    n: c_int,
    iov0_len: usize,
    nn: c_int,
) -> c_int {
    // C: if (n == 2 && nn == iov[0].iov_len)
    // nn (int) is converted to size_t for the comparison.
    if n == 2 && (nn as usize) == iov0_len {
        let ret: isize = unsafe { recv(fd, iov1_base, iov1_len, 0) };
        if ret > 0 {
            // nn += ret;  int = (int)((long)nn + (long)ret)
            return (nn as isize).wrapping_add(ret) as c_int;
        }
    }
    nn
}
}

// ── `; sb->sb_cc += nn; sb->sb_wptr += nn;` · slirp/socket.c:193-198 · chunk 2587 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//     
//     
//     
//      sb->sb_cc += nn;
//      sb->sb_wptr += nn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2587 {
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
pub extern "C" fn soread__part2587(sb: *mut Sbuf, nn: c_int) {
    unsafe {
        (*sb).sb_cc = (*sb).sb_cc.wrapping_add(nn as c_uint);
        (*sb).sb_wptr = (*sb).sb_wptr.offset(nn as isize);
    }
}
}

// ── `if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen)` · slirp/socket.c:199-200 · chunk 2588 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_wptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2588 {
use core::ffi::{c_char, c_uint};

// Mirror of C `struct sbuf` (slirp/sbuf.h) — accessed only through a pointer.
#[repr(C)]
pub struct sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[no_mangle]
pub extern "C" fn soread__part2588(sb: *mut sbuf) {
    // if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen))
    //   sb->sb_wptr -= sb->sb_datalen;
    if sb.is_null() {
        return;
    }
    unsafe {
        let datalen = (*sb).sb_datalen as usize;
        let wptr = (*sb).sb_wptr;
        let end = (*sb).sb_data.add(datalen);
        if wptr >= end {
            (*sb).sb_wptr = wptr.sub(datalen);
        }
    }
}
}

// ── `return nn;` · slirp/socket.c:201-201 · chunk 2589 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return nn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2589 {
// Port of the tail of soread(): return the number of bytes received.
#[no_mangle]
pub extern "C" fn soread__part2589(nn: core::ffi::c_int) -> core::ffi::c_int {
    nn
}
}

// ── `soreadbuf` · slirp/socket.c:204-244 · chunk 2590 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int soreadbuf(struct socket *so, const char *buf, int size)
//     {
//         int n, nn, copy = size;
//      struct sbuf *sb = &so->so_snd;
//      struct iovec iov[2];
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//      if (sopreprbuf(so, iov, &n) < size)
//             goto err;
//     
//         nn = ((iov[0].iov_len) < (copy) ? (iov[0].iov_len) : (copy));
//         memcpy(iov[0].iov_base, buf, nn);
//     
//         copy -= nn;
//         buf += nn;
//     
//         if (copy == 0)
//             goto done;
//     
//         memcpy(iov[1].iov_base, buf, copy);
//     
//     done:
//     
//      sb->sb_cc += size;
//      sb->sb_wptr += size;
//      if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_wptr -= sb->sb_datalen;
//         return size;
//     err:
//     
//         sofcantrcvmore(so);
//         tcp_sockclosed(((so)->so_tcpcb));
//         fprintf(
//                stderr
//                      , "soreadbuf buffer to small");
//         return -1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2590 {
// Rust rewrite of slirp's soreadbuf().
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

// struct iovec { void *iov_base; size_t iov_len; }
#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

// struct in_addr { uint32_t s_addr; }
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct sbuf from slirp/sbuf.h
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// struct socket from slirp/socket.h (full layout, in order)
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

// fd_set { __fd_mask fds_bits[1024 / (8 * sizeof(__fd_mask))]; } with __fd_mask = long
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [i64; 1024 / (8 * 8)],
}

extern "C" {
    fn sopreprbuf(so: *mut Socket, iov: *mut Iovec, np: *mut c_int) -> usize;
    fn tcp_sockclosed(tp: *mut c_void);
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut global_writefds: *mut FdSet;
    static mut stderr: *mut c_void;
}

// Private port of the C file's `static void sofcantrcvmore(struct socket *so)`.
unsafe fn sofcantrcvmore(so: *mut Socket) {
    if ((*so).so_state & 0x001) == 0 {
        shutdown((*so).s, 0);
        let fds: *mut FdSet = global_writefds;
        if !fds.is_null() {
            let fd: c_int = (*so).s;
            let bits: *mut i64 = core::ptr::addr_of_mut!((*fds).fds_bits) as *mut i64;
            let idx: usize = (fd / 64) as usize;
            let bit: i64 = (fd % 64) as i64;
            *bits.wrapping_add(idx) &= !((1u64 << (bit as u32)) as i64);
        }
    }
    (*so).so_state &= !0x002;
    if ((*so).so_state & 0x010) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x008;
    }
}

#[no_mangle]
pub unsafe extern "C" fn soreadbuf(so: *mut Socket, buf: *const c_char, size: c_int) -> c_int {
    let mut n: c_int = 0;
    let mut copy: c_int = size;
    let mut cur: *const c_char = buf;

    let mut iov: [Iovec; 2] = [
        Iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
        Iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
    ];

    // C: `sopreprbuf(...) < size` -- size_t vs int: the int is converted to size_t.
    let avail: u64 = sopreprbuf(so, iov.as_mut_ptr(), &mut n as *mut c_int) as u64;
    let size_as_size_t: u64 = size as i64 as u64;
    if avail < size_as_size_t {
        sofcantrcvmore(so);
        tcp_sockclosed((*so).so_tcpcb);
        fprintf(
            stderr,
            b"soreadbuf buffer to small\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    let iov0_base: *mut c_void = iov[0].iov_base;
    let iov0_len: usize = iov[0].iov_len;
    let iov1_base: *mut c_void = iov[1].iov_base;

    // nn = min(iov[0].iov_len, copy) -- compared in size_t, then stored into an int
    let copy_as_size_t: u64 = copy as isize as usize as u64;
    let nn_sz: u64 = if (iov0_len as u64) < copy_as_size_t {
        iov0_len as u64
    } else {
        copy_as_size_t
    };
    let nn: c_int = nn_sz as u32 as c_int;

    memcpy(iov0_base, cur as *const c_void, nn as isize as usize);

    copy = copy.wrapping_sub(nn);
    cur = cur.wrapping_add(nn as isize as usize);

    if copy != 0 {
        memcpy(iov1_base, cur as *const c_void, copy as isize as usize);
    }

    // Update fields
    let sb: *mut Sbuf = core::ptr::addr_of_mut!((*so).so_snd);
    (*sb).sb_cc = (*sb).sb_cc.wrapping_add(size as u32);
    (*sb).sb_wptr = (*sb).sb_wptr.wrapping_add(size as isize as usize);
    let end: *mut c_char = (*sb).sb_data.wrapping_add((*sb).sb_datalen as usize);
    if (*sb).sb_wptr >= end {
        (*sb).sb_wptr = (*sb).sb_wptr.wrapping_sub((*sb).sb_datalen as usize);
    }
    size
}
}

// ── `int n, nn, copy = size; struct sbuf *sb = &so->so_snd; struct iovec iov[2]; ; ;` · slirp/socket.c:206-211 · chunk 2591 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int n, nn, copy = size;
//      struct sbuf *sb = &so->so_snd;
//      struct iovec iov[2];
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2591 {
// Portion of soreadbuf(): initialization of n, nn, copy, sb and iov.
// The DEBUG_CALL/DEBUG_ARG macros in the C source expand to nothing, so only
// the variable initializations remain.  Everything the rest of the C function
// needs is handed back through the out-parameters.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct IoVec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

// struct sbuf { u_int sb_cc; u_int sb_datalen; char *sb_wptr; char *sb_rptr;
//               char *sb_data; };
#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// struct in_addr { uint32_t s_addr; };
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct socket (see slirp/socket.h) - needed only to reach so_snd.
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn soreadbuf__part2591(
    so: *mut Socket,
    size: c_int,
    np: *mut c_int,
    nnp: *mut c_int,
    copyp: *mut c_int,
    sbp: *mut *mut SBuf,
    iov: *mut IoVec,
) {
    // int n, nn, copy = size;
    if !np.is_null() {
        *np = 0;
    }
    if !nnp.is_null() {
        *nnp = 0;
    }
    if !copyp.is_null() {
        *copyp = size;
    }

    // struct sbuf *sb = &so->so_snd;
    if !sbp.is_null() {
        *sbp = if so.is_null() {
            core::ptr::null_mut()
        } else {
            core::ptr::addr_of_mut!((*so).so_snd)
        };
    }

    // struct iovec iov[2];  (uninitialized in C, filled in by sopreprbuf)
    if !iov.is_null() {
        core::ptr::write_bytes(iov, 0u8, 2);
    }

    // DEBUG_CALL("soreadbuf"); DEBUG_ARG("so = %p", so);  -> no-ops
    let _ = so;
}
}

// ── `if (sopreprbuf(so, iov, &n) < size)` · slirp/socket.c:217-218 · chunk 2592 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sopreprbuf(so, iov, &n) < size)
//             goto err;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2592 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

extern "C" {
    fn sopreprbuf(so: *mut c_void, iov: *mut iovec, np: *mut c_int) -> usize;
}

// Returns nonzero when sopreprbuf() reported less room than requested
// (the C `goto err` condition).
#[no_mangle]
pub extern "C" fn soreadbuf__part2592(
    so: *mut c_void,
    iov: *mut iovec,
    np: *mut c_int,
    size: c_int,
) -> c_int {
    let have: usize = unsafe { sopreprbuf(so, iov, np) };
    // C compares size_t against int: the int is converted to size_t
    // (sign-extended modulo 2^64).
    let want: usize = size as i64 as u64 as usize;
    if have < want {
        1
    } else {
        0
    }
}
}

// ── `nn = ((iov[0].iov_len) < (copy) ? (iov[0].iov_len) : (copy)); memcpy(iov[0].iov_base, buf, nn);…` · slirp/socket.c:220-224 · chunk 2593 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     nn = ((iov[0].iov_len) < (copy) ? (iov[0].iov_len) : (copy));
//         memcpy(iov[0].iov_base, buf, nn);
//     
//         copy -= nn;
//         buf += nn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2593 {
// Port of slirp/socket.c soreadbuf lines 220-224:
//   nn = min(iov[0].iov_len, copy); memcpy(iov[0].iov_base, buf, nn);
//   copy -= nn; buf += nn;
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn soreadbuf__part2593(
    iov0_base: *mut c_void,
    iov0_len: usize,
    bufp: *mut *const c_char,
    copyp: *mut c_int,
) -> c_int {
    unsafe {
        let copy: c_int = *copyp;
        let buf: *const c_char = *bufp;

        // C: (iov[0].iov_len < copy) ? iov[0].iov_len : copy  -- copy is
        // converted to size_t for the comparison, result is size_t, then
        // truncated into the int variable nn.
        let copy_as_size = copy as usize;
        let nn_usize: usize = if iov0_len < copy_as_size {
            iov0_len
        } else {
            copy_as_size
        };
        let nn: c_int = nn_usize as c_int;

        // memcpy(iov[0].iov_base, buf, nn) with nn converted to size_t.
        memcpy(iov0_base, buf as *const c_void, nn as usize);

        // copy -= nn; buf += nn;
        let copy_new: c_int = copy.wrapping_sub(nn);
        let buf_new: *const c_char = buf.wrapping_offset(nn as isize);

        *copyp = copy_new;
        *bufp = buf_new;

        nn
    }
}
}

// ── `if (copy == 0)` · slirp/socket.c:226-227 · chunk 2594 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (copy == 0)
//             goto done;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2594 {
use core::ffi::c_int;

// Ported piece of soreadbuf(): decides whether the remaining copy count is
// zero, which makes the caller jump to its `done` label.
#[no_mangle]
pub extern "C" fn soreadbuf__part2594(copy: c_int) -> c_int {
    if copy == 0 {
        1
    } else {
        0
    }
}
}

// ── `memcpy(iov[1].iov_base, buf, copy);` · slirp/socket.c:229-229 · chunk 2595 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(iov[1].iov_base, buf, copy);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2595 {
use core::ffi::{c_char, c_ulong, c_void};

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn soreadbuf__part2595(dest: *mut c_void, src: *const c_char, n: c_ulong) {
    unsafe {
        memcpy(dest, src as *const c_void, n);
    }
}
}

// ── `done: sb->sb_cc += size;` · slirp/socket.c:231-233 · chunk 2596 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     done:
//     
//      sb->sb_cc += size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2596 {
use core::ffi::{c_char, c_int, c_uint};

// Mirror of the C struct sbuf (field order and types preserved).
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Corresponds to: sb->sb_cc += size;
// Unsigned int arithmetic: the int is converted to unsigned and may wrap.
#[no_mangle]
pub extern "C" fn soreadbuf__part2596(sb: *mut Sbuf, size: c_int) {
    unsafe {
        if sb.is_null() {
            return;
        }
        let cc = (*sb).sb_cc;
        (*sb).sb_cc = cc.wrapping_add(size as c_uint);
    }
}
}

// ── `sb->sb_wptr += size;` · slirp/socket.c:234-234 · chunk 2597 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sb->sb_wptr += size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2597 {
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
pub extern "C" fn soreadbuf__part2597(sb: *mut Sbuf, size: c_int) {
    if sb.is_null() {
        return;
    }
    unsafe {
        let wptr = (*sb).sb_wptr;
        (*sb).sb_wptr = wptr.offset(size as isize);
    }
}
}

// ── `if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen)` · slirp/socket.c:235-236 · chunk 2598 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_wptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2598 {
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
pub extern "C" fn soreadbuf__part2598(sb: *mut Sbuf) {
    unsafe {
        if sb.is_null() {
            return;
        }
        let s = &mut *sb;
        let end = (s.sb_data as usize).wrapping_add(s.sb_datalen as usize);
        if (s.sb_wptr as usize) >= end {
            s.sb_wptr =
                (s.sb_wptr as usize).wrapping_sub(s.sb_datalen as usize) as *mut c_char;
        }
    }
}
}

// ── `return size;` · slirp/socket.c:237-237 · chunk 2599 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2599 {
// Ported piece of soreadbuf: the "done" return of the original size.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn soreadbuf__part2599(size: c_int) -> c_int {
    size
}
}

// ── `err: sofcantrcvmore(so);` · slirp/socket.c:238-240 · chunk 2600 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     err:
//     
//         sofcantrcvmore(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2600 {
// Port of the `err:` path of soreadbuf (slirp/socket.c), i.e. the inlined
// static helper sofcantrcvmore(so) which has no linker symbol.

use core::ffi::{c_int, c_long, c_uint, c_void};

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_FCANTRCVMORE: c_int = 0x008;
const SS_FCANTSENDMORE: c_int = 0x010;
const SS_PERSISTENT_MASK: c_int = 0xf000;

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16usize],
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

extern "C" {
    static mut global_writefds: *mut FdSet;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn soreadbuf__part2600(so: *mut Socket) {
    if so.is_null() {
        return;
    }
    unsafe {
        if ((*so).so_state & SS_NOFDREF) == 0 {
            let _ = shutdown((*so).s, 0);
            let wr = global_writefds;
            if !wr.is_null() {
                let bits_per: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
                let fd: c_int = (*so).s;
                let idx: usize = (fd / bits_per) as usize;
                let shift: u32 = (fd % bits_per) as u32;
                let mask: c_long = (1 as c_long).wrapping_shl(shift);
                let base: *mut c_long = (&mut (*wr).fds_bits) as *mut [c_long; 16] as *mut c_long;
                let slot = base.add(idx);
                *slot &= !mask;
            }
        }
        (*so).so_state &= !SS_ISFCONNECTING;
        if ((*so).so_state & SS_FCANTSENDMORE) != 0 {
            (*so).so_state &= SS_PERSISTENT_MASK;
            (*so).so_state |= SS_NOFDREF;
        } else {
            (*so).so_state |= SS_FCANTRCVMORE;
        }
    }
}
}

// ── `tcp_sockclosed(((so)->so_tcpcb)); fprintf( stderr , "soreadbuf buffer to small"); return -1;` · slirp/socket.c:241-243 · chunk 2601 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tcp_sockclosed(((so)->so_tcpcb));
//         fprintf(
//                stderr
//                      , "soreadbuf buffer to small");
//         return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2601 {
// Port of the `err` tail of slirp/socket.c:soreadbuf():
//   tcp_sockclosed(so->so_tcpcb);
//   fprintf(stderr, "soreadbuf buffer to small");
//   return -1;

use core::ffi::{c_char, c_int, c_void};

// Mirror of `struct socket` (fields in order, up to and including so_tcpcb;
// the tail after it is never touched here so it is omitted).
#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: u32, // struct in_addr { in_addr_t s_addr; }
    so_laddr: u32, // struct in_addr
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut c_void,
}

extern "C" {
    static mut stderr: *mut c_void;
    fn tcp_sockclosed(tp: *mut c_void);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

static MSG: [u8; 26] = *b"soreadbuf buffer to small\0";

#[no_mangle]
pub extern "C" fn soreadbuf__part2601(so: *mut Socket) -> c_int {
    unsafe {
        let tp: *mut c_void = (*so).so_tcpcb;
        tcp_sockclosed(tp);
        fprintf(stderr, MSG.as_ptr() as *const c_char);
    }
    -1
}
}

// ── `sorecvoob` · slirp/socket.c:253-274 · chunk 2602 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sorecvoob(struct socket *so)
//     {
//      struct tcpcb *tp = ((so)->so_tcpcb);
//     
//      ;
//      ;
//      soread(so);
//      tp->snd_up = tp->snd_una + so->so_snd.sb_cc;
//      tp->t_force = 1;
//      tcp_output(tp);
//      tp->t_force = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2602 {
// Rust port of slirp/socket.c: sorecvoob()
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// ---- mirror of the C structures the target touches (exact field order) ----

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct mbuf_ptr is declared __attribute__((packed))
#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut c_void,
}

// struct ipovly is packed
#[repr(C, packed)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

// struct tcphdr; the bitfields th_x2:4, th_off:4 share one u_int unit
#[repr(C)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u32, // u_int th_x2:4, th_off:4
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,       // u_int
    pub sb_datalen: u32,  // u_int
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
    pub so_m: *mut c_void,   /* struct mbuf * */
    pub so_ti: *mut tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: u32,      // u_int
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut tcpiphdr,
    pub seg_prev: *mut tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: tcpiphdr,
    pub t_socket: *mut socket,
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub snd_up: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub iss: u32,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub rcv_nxt: u32,
    pub rcv_up: u32,
    pub irs: u32,
    pub rcv_adv: u32,
    pub snd_max: u32,
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub t_idle: i16,
    pub t_rtt: i16,
    pub t_rtseq: u32,
    pub t_srtt: i16,
    pub t_rttvar: i16,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: i16,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

extern "C" {
    fn soread(so: *mut socket) -> c_int;
    fn tcp_output(tp: *mut tcpcb) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn sorecvoob(so: *mut socket) {
    let tp: *mut tcpcb = (*so).so_tcpcb;

    /*
     * We take a guess at how much urgent data has arrived.
     * In most situations, when urgent data arrives, the next
     * read() should get all the urgent data.  This guess will
     * be wrong however if more data arrives just after the
     * urgent data, or the read() doesn't return all the
     * urgent data.
     */
    soread(so);
    (*tp).snd_up = (*tp).snd_una.wrapping_add((*so).so_snd.sb_cc);
    (*tp).t_force = 1;
    tcp_output(tp);
    (*tp).t_force = 0;
}
}

// ── `struct tcpcb *tp = ((so)->so_tcpcb); ; ; soread(so); tp->snd_up = tp->snd_una + so->so_snd.sb_c…` · slirp/socket.c:256-273 · chunk 2603 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct tcpcb *tp = ((so)->so_tcpcb);
//     
//      ;
//      ;
//      soread(so);
//      tp->snd_up = tp->snd_una + so->so_snd.sb_cc;
//      tp->t_force = 1;
//      tcp_output(tp);
//      tp->t_force = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2603 {
// Port of the body of sorecvoob() from slirp/socket.c
use core::ffi::{c_char, c_int, c_uint, c_short, c_ushort, c_void};

// struct sbuf
#[repr(C)]
pub struct Sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

// struct tcpiphdr (ipovly + tcphdr): 44 bytes, alignment 4.
// Only used as an opaque member of struct tcpcb, never accessed.
#[repr(C, align(4))]
pub struct Tcpiphdr {
    _opaque: [u8; 44],
}

// struct tcpcb: mirrored up to the last field the port touches (snd_up).
#[repr(C)]
pub struct Tcpcb {
    seg_next: *mut c_void,
    seg_prev: *mut c_void,
    t_state: c_short,
    t_timer: [c_short; 4],
    t_rxtshift: c_short,
    t_rxtcur: c_short,
    t_dupacks: c_short,
    t_maxseg: c_ushort,
    t_force: c_char,
    t_flags: c_ushort,
    t_template: Tcpiphdr,
    t_socket: *mut Socket,
    snd_una: u32,
    snd_nxt: u32,
    snd_up: u32,
}

// struct socket: mirrored up to so_snd.sb_cc (the only part touched).
#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: u32,
    so_laddr: u32,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut Tcpcb,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: Sbuf,
    so_snd: Sbuf,
}

extern "C" {
    fn soread(so: *mut Socket) -> c_int;
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn sorecvoob__part2603(so: *mut Socket) {
    unsafe {
        let tp: *mut Tcpcb = (*so).so_tcpcb;

        soread(so);
        (*tp).snd_up = (*tp).snd_una.wrapping_add((*so).so_snd.sb_cc);
        (*tp).t_force = 1;
        tcp_output(tp);
        (*tp).t_force = 0;
    }
}
}

// ── `sosendoob` · slirp/socket.c:280-332 · chunk 2604 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     sosendoob(struct socket *so)
//     {
//      struct sbuf *sb = &so->so_rcv;
//      char buff[2048];
//     
//      int n, len;
//     
//      ;
//      ;
//      ;
//     
//      if (so->so_urgc > 2048)
//         so->so_urgc = 2048;
//     
//      if (sb->sb_rptr < sb->sb_wptr) {
//     
//       n = slirp_send(so, sb->sb_rptr, so->so_urgc, (
//                                                    MSG_OOB
//                                                           ));
//       so->so_urgc -= n;
//     
//       ;
//      } else {
//     
//     
//     
//     
//     
//       len = (sb->sb_data + sb->sb_datalen) - sb->sb_rptr;
//       if (len > so->so_urgc) len = so->so_urgc;
//       memcpy(buff, sb->sb_rptr, len);
//       so->so_urgc -= len;
//       if (so->so_urgc) {
//        n = sb->sb_wptr - sb->sb_data;
//        if (n > so->so_urgc) n = so->so_urgc;
//        memcpy((buff + len), sb->sb_data, n);
//        so->so_urgc -= n;
//        len += n;
//       }
//       n = slirp_send(so, buff, len, (
//                                     MSG_OOB
//                                            ));
//     
//     
//     
//     
//       ;
//      }
//     
//      sb->sb_cc -= n;
//      sb->sb_rptr += n;
//      if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_rptr -= sb->sb_datalen;
//     
//      return n;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2604 {
// Port of slirp's sosendoob() (slirp/socket.c).
use core::ffi::{c_char, c_int, c_uint, c_void};

const MSG_OOB: c_int = 0x01;

#[repr(C)]
struct Sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut c_void,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: Sbuf,
    so_snd: Sbuf,
    extra: *mut c_void,
}

extern "C" {
    fn slirp_send(so: *mut Socket, buf: *const c_void, len: usize, flags: c_int) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn sosendoob(so: *mut Socket) -> c_int {
    let sb: *mut Sbuf = core::ptr::addr_of_mut!((*so).so_rcv);
    let mut buff = [0u8; 2048];

    let n: c_int;
    let mut len: c_int;

    if (*so).so_urgc > 2048 {
        (*so).so_urgc = 2048;
    }

    if (*sb).sb_rptr < (*sb).sb_wptr {
        /* We can send it directly */
        n = slirp_send(
            so,
            (*sb).sb_rptr as *const c_void,
            (*so).so_urgc as usize,
            MSG_OOB,
        ) as c_int;
        (*so).so_urgc = (*so).so_urgc.wrapping_sub(n);
    } else {
        /*
         * Since there's no sendv or sendtov like writev,
         * we must copy all data to a linear buffer then
         * send it all
         */
        len = ((*sb).sb_data as isize + (*sb).sb_datalen as isize
            - (*sb).sb_rptr as isize) as c_int;
        if len > (*so).so_urgc {
            len = (*so).so_urgc;
        }
        core::ptr::copy_nonoverlapping(
            (*sb).sb_rptr as *const u8,
            buff.as_mut_ptr(),
            len as usize,
        );
        (*so).so_urgc = (*so).so_urgc.wrapping_sub(len);
        if (*so).so_urgc != 0 {
            let mut nn: c_int = ((*sb).sb_wptr as isize - (*sb).sb_data as isize) as c_int;
            if nn > (*so).so_urgc {
                nn = (*so).so_urgc;
            }
            core::ptr::copy_nonoverlapping(
                (*sb).sb_data as *const u8,
                buff.as_mut_ptr().wrapping_offset(len as isize),
                nn as usize,
            );
            (*so).so_urgc = (*so).so_urgc.wrapping_sub(nn);
            len = len.wrapping_add(nn);
        }
        n = slirp_send(so, buff.as_mut_ptr() as *const c_void, len as usize, MSG_OOB) as c_int;
    }

    (*sb).sb_cc = (*sb).sb_cc.wrapping_sub(n as c_uint);
    (*sb).sb_rptr = (*sb).sb_rptr.wrapping_offset(n as isize);
    if (*sb).sb_rptr >= (*sb).sb_data.wrapping_add((*sb).sb_datalen as usize) {
        (*sb).sb_rptr = (*sb).sb_rptr.wrapping_sub((*sb).sb_datalen as usize);
    }

    n
}
}

// ── `struct sbuf *sb = &so->so_rcv; char buff[2048]; int n, len; ; ; ;` · slirp/socket.c:283-290 · chunk 2605 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct sbuf *sb = &so->so_rcv;
//      char buff[2048];
//     
//      int n, len;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2605 {
// Port of the local-declaration prologue of sosendoob():
//   struct sbuf *sb = &so->so_rcv;
//   char buff[2048];
//   int n, len;
// The Rust function computes the address of so->so_rcv and returns it.
// buff/n/len stay declared (uninitialised as in C) in the caller.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn sosendoob__part2605(so: *mut Socket) -> *mut Sbuf {
    // Equivalent of `&so->so_rcv`.
    core::ptr::addr_of_mut!((*so).so_rcv)
}
}

// ── `if (so->so_urgc > 2048)` · slirp/socket.c:292-293 · chunk 2606 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_urgc > 2048)
//         so->so_urgc = 2048;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2606 {
// Clamp the urgent-data count of a socket to at most 2048 bytes.
// Ported from the head of sosendoob() in slirp/socket.c.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sosendoob__part2606(so_urgc: *mut c_int) {
    unsafe {
        if !so_urgc.is_null() && *so_urgc > 2048 {
            *so_urgc = 2048;
        }
    }
}
}

// ── `if (sb->sb_rptr < sb->sb_wptr) … else` · slirp/socket.c:295-324 · chunk 2607 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_rptr < sb->sb_wptr) {
//     
//       n = slirp_send(so, sb->sb_rptr, so->so_urgc, (
//                                                    MSG_OOB
//                                                           ));
//       so->so_urgc -= n;
//     
//       ;
//      } else {
//     
//     
//     
//     
//     
//       len = (sb->sb_data + sb->sb_datalen) - sb->sb_rptr;
//       if (len > so->so_urgc) len = so->so_urgc;
//       memcpy(buff, sb->sb_rptr, len);
//       so->so_urgc -= len;
//       if (so->so_urgc) {
//        n = sb->sb_wptr - sb->sb_data;
//        if (n > so->so_urgc) n = so->so_urgc;
//        memcpy((buff + len), sb->sb_data, n);
//        so->so_urgc -= n;
//        len += n;
//       }
//       n = slirp_send(so, buff, len, (
//                                     MSG_OOB
//                                            ));
//     
//     
//     
//     
//       ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2607 {
// Port of the OOB-send branch pair of sosendoob() (slirp/socket.c).
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn slirp_send(so: *mut Socket, buf: *const c_void, len: usize, flags: c_int) -> c_long;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

const MSG_OOB: c_int = 0x01;

#[no_mangle]
pub extern "C" fn sosendoob__part2607(
    so: *mut Socket,
    sb: *mut Sbuf,
    buff: *mut c_char,
) -> c_int {
    unsafe {
        let mut n: c_int = 0;
        let mut len: c_int = 0;

        let rptr = (*sb).sb_rptr;
        let wptr = (*sb).sb_wptr;

        if (rptr as usize) < (wptr as usize) {
            let sent = slirp_send(so, rptr as *const c_void, (*so).so_urgc as usize, MSG_OOB);
            n = sent as c_int;
            (*so).so_urgc = ((*so).so_urgc as c_long - sent) as c_int;
        } else {
            let tail: isize = ((*sb).sb_data as isize)
                .wrapping_add((*sb).sb_datalen as isize)
                .wrapping_sub((*sb).sb_rptr as isize);
            len = tail as c_int;
            if len > (*so).so_urgc {
                len = (*so).so_urgc;
            }
            memcpy(
                buff as *mut c_void,
                (*sb).sb_rptr as *const c_void,
                len as usize,
            );
            (*so).so_urgc -= len;
            if (*so).so_urgc != 0 {
                n = (*sb)
                    .sb_wptr
                    .wrapping_offset(-((*sb).sb_data as isize)) as isize as c_int;
                if n > (*so).so_urgc {
                    n = (*so).so_urgc;
                }
                memcpy(
                    buff.wrapping_offset(len as isize) as *mut c_void,
                    (*sb).sb_data as *const c_void,
                    n as usize,
                );
                (*so).so_urgc -= n;
                len = len.wrapping_add(n);
            }
            let sent = slirp_send(so, buff as *const c_void, len as usize, MSG_OOB);
            n = sent as c_int;
        }

        n
    }
}
}

// ── `sb->sb_cc -= n; sb->sb_rptr += n;` · slirp/socket.c:326-327 · chunk 2608 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sb->sb_cc -= n;
//      sb->sb_rptr += n;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2608 {
// Port of slirp/socket.c sosendoob: advance the receive buffer pointer and
// reduce the byte count after a send.
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
pub unsafe extern "C" fn sosendoob__part2608(sb: *mut Sbuf, n: c_int) {
    let sb = &mut *sb;
    sb.sb_cc = sb.sb_cc.wrapping_sub(n as c_uint);
    sb.sb_rptr = sb.sb_rptr.offset(n as isize);
}
}

// ── `if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen)` · slirp/socket.c:328-329 · chunk 2609 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_rptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2609 {
// Rewrite of the sbuf read-pointer wrap in sosendoob (slirp/socket.c:328-329).
// C code:  if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen))
//              sb->sb_rptr -= sb->sb_datalen;

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: core::ffi::c_uint,
    pub sb_datalen: core::ffi::c_uint,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn sosendoob__part2609(sb: *mut Sbuf) {
    if sb.is_null() {
        return;
    }
    let len = (*sb).sb_datalen;
    let data = (*sb).sb_data;
    let rptr = (*sb).sb_rptr;
    // (sb->sb_data + sb->sb_datalen) with C pointer arithmetic.
    let end = data.wrapping_add(len as usize);
    if rptr >= end {
        (*sb).sb_rptr = rptr.wrapping_sub(len as usize);
    }
}
}

// ── `return n;` · slirp/socket.c:331-331 · chunk 2610 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return n;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2610 {
// Ported piece of sosendoob(): the tail `return n;`.
// Takes the byte count n by value and returns it to the caller in C.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sosendoob__part2610(n: c_int) -> c_int {
    n
}
}

// ── `sowrite` · slirp/socket.c:338-425 · chunk 2611 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int
//     sowrite(struct socket *so)
//     {
//      int n,nn;
//      struct sbuf *sb = &so->so_rcv;
//      int len = sb->sb_cc;
//      struct iovec iov[2];
//     
//      ;
//      ;
//     
//      if (so->so_urgc) {
//       sosendoob(so);
//       if (sb->sb_cc == 0)
//        return 0;
//      }
//     
//     
//     
//     
//     
//     
//      iov[0].iov_base = sb->sb_rptr;
//             iov[1].iov_base = 
//                              ((void *)0)
//                                  ;
//             iov[1].iov_len = 0;
//      if (sb->sb_rptr < sb->sb_wptr) {
//       iov[0].iov_len = sb->sb_wptr - sb->sb_rptr;
//     
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       n = 1;
//      } else {
//       iov[0].iov_len = (sb->sb_data + sb->sb_datalen) - sb->sb_rptr;
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       len -= iov[0].iov_len;
//       if (len) {
//        iov[1].iov_base = sb->sb_data;
//        iov[1].iov_len = sb->sb_wptr - sb->sb_data;
//        if (iov[1].iov_len > len) iov[1].iov_len = len;
//        n = 2;
//       } else
//        n = 1;
//      }
//     
//     
//     
//     
//     
//     
//     
//      nn = slirp_send(so, iov[0].iov_base, iov[0].iov_len,0);
//     
//     
//      if (nn < 0 && (
//                    (*__errno_location ()) 
//                          == 
//                             11 
//                                    || 
//                                       (*__errno_location ()) 
//                                             == 
//                                                4
//                                                     ))
//       return 0;
//     
//      if (nn <= 0) {
//      
//                             ;
//       sofcantsendmore(so);
//       tcp_sockclosed(((so)->so_tcpcb));
//       return -1;
//      }
//     
//     
//      if (n == 2 && nn == iov[0].iov_len) {
//                 int ret;
//                 ret = slirp_send(so, iov[1].iov_base, iov[1].iov_len,0);
//                 if (ret > 0)
//                     nn += ret;
//             }
//             ;
//     
//     
//     
//      sb->sb_cc -= nn;
//      sb->sb_rptr += nn;
//      if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_rptr -= sb->sb_datalen;
//     
//     
//     
//     
//     
//      if ((so->so_state & 0x040) && sb->sb_cc == 0)
//       sofcantsendmore(so);
//     
//      return nn;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2611 {
// Port of slirp/socket.c:sowrite() (and the static helper sofcantsendmore()).
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
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

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
struct FdSet {
    fds_bits: [c_long; 16],
}

extern "C" {
    fn sosendoob(so: *mut socket) -> c_int;
    fn slirp_send(so: *mut socket, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn tcp_sockclosed(tp: *mut c_void);
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;

    static mut global_readfds: *mut FdSet;
    static mut global_xfds: *mut FdSet;
}

// FD_CLR() from <sys/select.h>: 8 * sizeof(__fd_mask) == 64 on this target.
unsafe fn fd_clr(fd: c_int, set: *mut FdSet) {
    let idx = fd / 64;
    let bit = fd % 64;
    let slot = (*set).fds_bits.as_mut_ptr().wrapping_add(idx as usize);
    *slot &= !(1 as c_long).wrapping_shl(bit as u32);
}

// static in the C file: logic ported here (no linker symbol).
unsafe fn sofcantsendmore(so: *mut socket) {
    if ((*so).so_state & 0x001) == 0 {
        shutdown((*so).s, 1);
        let rfds = global_readfds;
        if !rfds.is_null() {
            fd_clr((*so).s, rfds);
        }
        let xfds = global_xfds;
        if !xfds.is_null() {
            fd_clr((*so).s, xfds);
        }
    }
    (*so).so_state &= !0x002;
    if ((*so).so_state & 0x008) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x010;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sowrite(so: *mut socket) -> c_int {
    let sb: *mut sbuf = core::ptr::addr_of_mut!((*so).so_rcv);
    let mut n: c_int;
    let mut nn: c_int;
    let mut len: c_int = (*sb).sb_cc as c_int;
    let mut iov: [iovec; 2] = [
        iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
        iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
    ];

    if (*so).so_urgc != 0 {
        sosendoob(so);
        if (*sb).sb_cc == 0 {
            return 0;
        }
    }

    iov[0].iov_base = (*sb).sb_rptr as *mut c_void;
    iov[1].iov_base = core::ptr::null_mut();
    iov[1].iov_len = 0;
    if ((*sb).sb_rptr as usize) < ((*sb).sb_wptr as usize) {
        iov[0].iov_len = ((*sb).sb_wptr as usize).wrapping_sub((*sb).sb_rptr as usize);
        // C compares size_t with int: the int operand is sign-extended.
        if iov[0].iov_len > (len as i64 as usize) {
            iov[0].iov_len = len as i64 as usize;
        }
        n = 1;
    } else {
        iov[0].iov_len = ((*sb).sb_data as usize)
            .wrapping_add((*sb).sb_datalen as usize)
            .wrapping_sub((*sb).sb_rptr as usize);
        if iov[0].iov_len > (len as i64 as usize) {
            iov[0].iov_len = len as i64 as usize;
        }
        // len -= iov[0].iov_len with unsigned arithmetic, truncated back to int
        len = ((len as i64 as u64).wrapping_sub(iov[0].iov_len as u64)) as u32 as c_int;
        if len != 0 {
            iov[1].iov_base = (*sb).sb_data as *mut c_void;
            iov[1].iov_len = ((*sb).sb_wptr as usize).wrapping_sub((*sb).sb_data as usize);
            if iov[1].iov_len > (len as i64 as usize) {
                iov[1].iov_len = len as i64 as usize;
            }
            n = 2;
        } else {
            n = 1;
        }
    }

    nn = slirp_send(so, iov[0].iov_base, iov[0].iov_len, 0) as c_int;

    if nn < 0 && (*__errno_location() == 11 || *__errno_location() == 4) {
        return 0;
    }

    if nn <= 0 {
        sofcantsendmore(so);
        tcp_sockclosed((*so).so_tcpcb);
        return -1;
    }

    if n == 2 && (nn as i64 as usize) == iov[0].iov_len {
        let ret = slirp_send(so, iov[1].iov_base, iov[1].iov_len, 0);
        if ret > 0 {
            nn = (nn as isize).wrapping_add(ret) as c_int;
        }
    }

    // Update sbuf
    (*sb).sb_cc = (*sb).sb_cc.wrapping_sub(nn as u32);
    (*sb).sb_rptr = (*sb).sb_rptr.wrapping_offset(nn as isize);
    if ((*sb).sb_rptr as usize)
        >= ((*sb).sb_data as usize).wrapping_add((*sb).sb_datalen as usize)
    {
        (*sb).sb_rptr = ((*sb).sb_rptr as usize)
            .wrapping_sub((*sb).sb_datalen as usize) as *mut c_char;
    }

    if ((*so).so_state & 0x040) != 0 && (*sb).sb_cc == 0 {
        sofcantsendmore(so);
    }

    nn
}
}

// ── `int n,nn; struct sbuf *sb = &so->so_rcv; int len = sb->sb_cc; struct iovec iov[2]; ; ;` · slirp/socket.c:341-347 · chunk 2612 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int n,nn;
//      struct sbuf *sb = &so->so_rcv;
//      int len = sb->sb_cc;
//      struct iovec iov[2];
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2612 {
// Rust port of the prologue of sowrite(): compute sb = &so->so_rcv and
// len = sb->sb_cc.  The remaining locals (n, nn, iov) are left declared in C.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,        /* u_int */
    pub sb_datalen: u32,   /* u_int */
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32, /* in_addr_t */
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
    pub so_type: u8, /* u_char */
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: u32, /* u_int */
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn sowrite__part2612(
    so: *mut Socket,
    sb_out: *mut *mut Sbuf,
    len_out: *mut c_int,
) {
    unsafe {
        let sb: *mut Sbuf = core::ptr::addr_of_mut!((*so).so_rcv);
        *sb_out = sb;
        *len_out = (*sb).sb_cc as c_int;
    }
}
}

// ── `if (so->so_urgc)` · slirp/socket.c:349-353 · chunk 2613 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_urgc) {
//       sosendoob(so);
//       if (sb->sb_cc == 0)
//        return 0;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2613 {
use core::ffi::{c_char, c_int, c_uint, c_void};

// Mirror of `struct sbuf` (slirp/socket.c) - exact fields, in order.
#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Mirror of the leading part of `struct socket` up to and including
// `so_urgc`; later fields are not touched by this piece.
#[repr(C)]
pub struct socket_prefix {
    pub so_next: *mut c_void,
    pub so_prev: *mut c_void,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
}

extern "C" {
    fn sosendoob(so: *mut socket_prefix) -> c_int;
}

// Returns 1 when the caller of sowrite() must `return 0`, 0 otherwise.
#[no_mangle]
pub extern "C" fn sowrite__part2613(so: *mut socket_prefix, sb: *mut sbuf) -> c_int {
    unsafe {
        if !so.is_null() && (*so).so_urgc != 0 {
            sosendoob(so);
            if sb.is_null() || (*sb).sb_cc == 0 {
                return 1;
            }
        }
        0
    }
}
}

// ── `iov[0].iov_base = sb->sb_rptr; iov[1].iov_base = ((void *)0) ; iov[1].iov_len = 0;` · slirp/socket.c:360-362 · chunk 2614 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     iov[0].iov_base = sb->sb_rptr;
//             iov[1].iov_base = 
//                              ((void *)0)
//                                  ;
//             iov[1].iov_len = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2614 {
use core::ffi::{c_char, c_uint, c_void};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[no_mangle]
pub extern "C" fn sowrite__part2614(sb: *mut Sbuf, iov: *mut Iovec) {
    unsafe {
        if sb.is_null() || iov.is_null() {
            return;
        }
        (*iov).iov_base = (*sb).sb_rptr as *mut c_void;
        let iov1 = iov.add(1);
        (*iov1).iov_base = core::ptr::null_mut();
        (*iov1).iov_len = 0;
    }
}
}

// ── `if (sb->sb_rptr < sb->sb_wptr) … else` · slirp/socket.c:363-379 · chunk 2615 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_rptr < sb->sb_wptr) {
//       iov[0].iov_len = sb->sb_wptr - sb->sb_rptr;
//     
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       n = 1;
//      } else {
//       iov[0].iov_len = (sb->sb_data + sb->sb_datalen) - sb->sb_rptr;
//       if (iov[0].iov_len > len) iov[0].iov_len = len;
//       len -= iov[0].iov_len;
//       if (len) {
//        iov[1].iov_base = sb->sb_data;
//        iov[1].iov_len = sb->sb_wptr - sb->sb_data;
//        if (iov[1].iov_len > len) iov[1].iov_len = len;
//        n = 2;
//       } else
//        n = 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2615 {
// Port of the iovec-setup branch of sowrite() (slirp/socket.c).
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[no_mangle]
pub extern "C" fn sowrite__part2615(
    sb: *mut Sbuf,
    iov: *mut Iovec,
    n: *mut c_int,
    mut len: c_int,
) {
    unsafe {
        let s = &mut *sb;
        let rptr = s.sb_rptr as usize;
        let wptr = s.sb_wptr as usize;
        let data = s.sb_data as usize;
        let datalen = s.sb_datalen as usize;

        if rptr < wptr {
            (*iov).iov_len = wptr - rptr;
            if (*iov).iov_len > len as usize {
                (*iov).iov_len = len as usize;
            }
            *n = 1;
        } else {
            (*iov).iov_len = data.wrapping_add(datalen).wrapping_sub(rptr);
            if (*iov).iov_len > len as usize {
                (*iov).iov_len = len as usize;
            }
            len = (len as usize).wrapping_sub((*iov).iov_len) as c_int;
            if len != 0 {
                (*(iov.add(1))).iov_base = s.sb_data as *mut c_void;
                (*(iov.add(1))).iov_len = wptr.wrapping_sub(data);
                if (*(iov.add(1))).iov_len > len as usize {
                    (*(iov.add(1))).iov_len = len as usize;
                }
                *n = 2;
            } else {
                *n = 1;
            }
        }
    }
}
}

// ── `nn = slirp_send(so, iov[0].iov_base, iov[0].iov_len,0);` · slirp/socket.c:387-387 · chunk 2616 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     nn = slirp_send(so, iov[0].iov_base, iov[0].iov_len,0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2616 {
// Port of one statement of sowrite(): the first slirp_send() call.
use core::ffi::{c_int, c_long, c_void};

extern "C" {
    // ssize_t slirp_send(struct socket *so, const void *buf, size_t len, int flags);
    fn slirp_send(so: *mut c_void, buf: *const c_void, len: usize, flags: c_int) -> c_long;
}

#[no_mangle]
pub extern "C" fn sowrite__part2616(so: *mut c_void, buf: *const c_void, len: usize) -> c_long {
    unsafe { slirp_send(so, buf, len, 0) }
}
}

// ── `if (nn < 0 && ( (*__errno_location ()) == 11 || (*__errno_location ()) == 4)` · slirp/socket.c:390-391 · chunk 2617 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (nn < 0 && (
//                    (*__errno_location ()) 
//                          == 
//                             11 
//                                    || 
//                                       (*__errno_location ()) 
//                                             == 
//                                                4
//                                                     ))
//       return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2617 {
use core::ffi::c_int;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

// Returns nonzero when the write error is EAGAIN (11) or EINTR (4),
// mirroring: if (nn < 0 && (errno == 11 || errno == 4)) return 0;
#[no_mangle]
pub extern "C" fn sowrite__part2617(nn: c_int) -> c_int {
    if nn < 0 {
        let e: c_int = unsafe { *__errno_location() };
        if e == 11 || e == 4 {
            return 1;
        }
    }
    0
}
}

// ── `if (nn <= 0)` · slirp/socket.c:393-399 · chunk 2618 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (nn <= 0) {
//      
//                             ;
//       sofcantsendmore(so);
//       tcp_sockclosed(((so)->so_tcpcb));
//       return -1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2618 {
// Port of the "nn <= 0" failure branch of sowrite() from slirp/socket.c.
// Performs the static helper sofcantsendmore() inline (it has no linker
// symbol) and then calls tcp_sockclosed() on the socket's tcpcb.

use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
}

// fd_set as laid out by glibc: __fd_mask fds_bits[1024 / (8 * sizeof(__fd_mask))]
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16],
}

extern "C" {
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn tcp_sockclosed(tp: *mut c_void);
    static mut global_readfds: *mut FdSet;
}

unsafe fn clear_read_fd(fd: c_int) {
    let fds = global_readfds;
    if !fds.is_null() {
        let denom: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
        let idx: c_int = fd / denom;
        let bit: c_int = fd % denom;
        let slot = (fds as *mut FdSet).cast::<c_long>().offset(idx as isize);
        *slot &= !((1u64.wrapping_shl(bit as u32)) as c_long);
    }
}

// static void sofcantsendmore(struct socket *so)
unsafe fn sofcantsendmore(so: *mut Socket) {
    if ((*so).so_state & 0x001) == 0 {
        shutdown((*so).s, 1);
        clear_read_fd((*so).s);
    }
    (*so).so_state &= !0x002;
    if ((*so).so_state & 0x008) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x010;
    }
}

#[no_mangle]
pub extern "C" fn sowrite__part2618(so: *mut Socket) {
    unsafe {
        sofcantsendmore(so);
        tcp_sockclosed((*so).so_tcpcb);
    }
}
}

// ── `if (n == 2 && nn == iov[0].iov_len)` · slirp/socket.c:402-407 · chunk 2619 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n == 2 && nn == iov[0].iov_len) {
//                 int ret;
//                 ret = slirp_send(so, iov[1].iov_base, iov[1].iov_len,0);
//                 if (ret > 0)
//                     nn += ret;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2619 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn slirp_send(so: *mut c_void, buf: *const c_void, len: usize, flags: c_int) -> isize;
}

#[no_mangle]
pub extern "C" fn sowrite__part2619(
    so: *mut c_void,
    n: c_int,
    nn: c_int,
    iov1_base: *mut c_void,
    iov1_len: usize,
    iov0_len: usize,
) -> c_int {
    let mut nn = nn;
    if n == 2 && (nn as usize) == iov0_len {
        let ret: isize = unsafe { slirp_send(so, iov1_base, iov1_len, 0) };
        if ret > 0 {
            nn = (nn as isize).wrapping_add(ret) as c_int;
        }
    }
    nn
}
}

// ── `; sb->sb_cc -= nn; sb->sb_rptr += nn;` · slirp/socket.c:408-413 · chunk 2620 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//     
//     
//     
//      sb->sb_cc -= nn;
//      sb->sb_rptr += nn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2620 {
// Portion of sowrite(): buffer accounting after a successful send.
// sb->sb_cc -= nn;  sb->sb_rptr += nn;

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: core::ffi::c_uint,
    pub sb_datalen: core::ffi::c_uint,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

#[no_mangle]
pub unsafe extern "C" fn sowrite__part2620(sb: *mut Sbuf, nn: core::ffi::c_int) {
    let sb = &mut *sb;
    // unsigned arithmetic: the int operand is converted to unsigned, wrapping.
    sb.sb_cc = sb.sb_cc.wrapping_sub(nn as core::ffi::c_uint);
    // pointer arithmetic in char units.
    sb.sb_rptr = sb.sb_rptr.offset(nn as isize);
}
}

// ── `if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen)` · slirp/socket.c:414-415 · chunk 2621 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen))
//       sb->sb_rptr -= sb->sb_datalen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2621 {
// Port of the rptr-wrap piece of sowrite() from slirp/socket.c
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: core::ffi::c_uint,
    pub sb_datalen: core::ffi::c_uint,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

#[no_mangle]
pub extern "C" fn sowrite__part2621(sb: *mut Sbuf) {
    unsafe {
        if sb.is_null() {
            return;
        }
        let datalen = (*sb).sb_datalen as usize;
        let rptr = (*sb).sb_rptr;
        let data = (*sb).sb_data;
        let end = data.add(datalen);
        if rptr >= end {
            (*sb).sb_rptr = rptr.sub(datalen);
        }
    }
}
}

// ── `if (so->so_state & 0x040) && sb->sb_cc == 0)` · slirp/socket.c:421-422 · chunk 2622 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_state & 0x040) && sb->sb_cc == 0)
//       sofcantsendmore(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2622 {
// Port of the tail of sowrite(): if the socket is flagged and the receive
// buffer is drained, tell the peer we cannot send more.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

// fd_set on this target: __fd_mask is `long`, 1024 / (8 * 8) = 16 entries.
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [i64; 16],
}

extern "C" {
    static mut global_readfds: *mut FdSet;
    static mut global_xfds: *mut FdSet;
    fn shutdown(__fd: c_int, __how: c_int) -> c_int;
}

#[inline]
unsafe fn fd_clr(set: *mut FdSet, fd: c_int) {
    if !set.is_null() {
        let idx: c_int = fd / 64;
        let bit: c_int = fd % 64;
        let mask: i64 = (1u64 << ((bit as u32) & 63)) as i64;
        let bits: *mut i64 = (*set).fds_bits.as_mut_ptr();
        let p = bits.offset(idx as isize);
        *p &= !mask;
    }
}

// Static C function `sofcantsendmore`, ported in as a private helper.
unsafe fn sofcantsendmore(so: *mut Socket) {
    if ((*so).so_state & 0x001) == 0 {
        shutdown((*so).s, 1);
        if !global_readfds.is_null() {
            fd_clr(global_readfds, (*so).s);
        }
        if !global_xfds.is_null() {
            fd_clr(global_xfds, (*so).s);
        }
    }
    (*so).so_state &= !0x002;
    if ((*so).so_state & 0x008) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x010;
    }
}

#[no_mangle]
pub extern "C" fn sowrite__part2622(so: *mut Socket) {
    unsafe {
        if ((*so).so_state & 0x040) != 0 && (*so).so_rcv.sb_cc == 0 {
            sofcantsendmore(so);
        }
    }
}
}

// ── `return nn;` · slirp/socket.c:424-424 · chunk 2623 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return nn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2623 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sowrite__part2623(nn: c_int) -> c_int {
    nn
}
}

// ── `sorecvfrom` · slirp/socket.c:430-526 · chunk 2624 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sorecvfrom(struct socket *so)
//     {
//      struct sockaddr_in addr;
//      socklen_t addrlen = sizeof(struct sockaddr_in);
//     
//      ;
//      ;
//     
//      if (so->so_type == 
//                        IPPROTO_ICMP
//                                    ) {
//        char buff[256];
//        int len;
//     
//        len = recvfrom(so->s, buff, 256, 0,
//         (struct sockaddr *)&addr, &addrlen);
//     
//     
//        if(len == -1 || len == 0) {
//          u_char code=3;
//     
//          if(
//            (*__errno_location ()) 
//                  == 
//                     113
//                                 ) code=1;
//          else if(
//                 (*__errno_location ()) 
//                       == 
//                          101
//                                     ) code=0;
//     
//         
//                               ;
//          icmp_error(so->so_m, 3,code, 0,strerror(
//                                                            (*__errno_location ())
//                                                                 ));
//        } else {
//          icmp_reflect(so->so_m);
//                 so->so_m = 
//                           ((void *)0)
//                               ;
//        }
//     
//        udp_detach(so);
//      } else {
//        struct mbuf *m;
//               int len;
//     
//     
//     
//               int n;
//     
//     
//        m = m_get(so->slirp);
//        if (!m) {
//            return;
//        }
//        m->m_hdr.mh_data += (2 + 14 + 40);
//     
//     
//     
//     
//     
//        len = (((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len);
//     
//        ioctl(so->s, 
//                          0x541B
//                                  , &n);
//     
//        if (n > len) {
//          n = (m->m_hdr.mh_data - m->M_dat.m_dat_) + m->m_hdr.mh_len + n + 1;
//          m_inc(m, n);
//          len = (((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len);
//        }
//     
//     
//        m->m_hdr.mh_len = recvfrom(so->s, m->m_hdr.mh_data, len, 0,
//              (struct sockaddr *)&addr, &addrlen);
//       
//                                              ;
//        if(m->m_hdr.mh_len<0) {
//          u_char code=3;
//     
//          if(
//            (*__errno_location ()) 
//                  == 
//                     113
//                                 ) code=1;
//          else if(
//                 (*__errno_location ()) 
//                       == 
//                          101
//                                     ) code=0;
//     
//          ;
//          icmp_error(so->so_m, 3,code, 0,strerror(
//                                                            (*__errno_location ())
//                                                                 ));
//          m_free(m);
//        } else {
//     
//     
//     
//     
//     
//     
//          if (so->so_expire) {
//            if (so->so_fport == htons(53))
//       so->so_expire = curtime + 10000;
//            else
//       so->so_expire = curtime + 240000;
//          }
//     
//     
//     
//     
//     
//          udp_output(so, m, &addr);
//        }
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2624 {
// Port of slirp/socket.c: sorecvfrom(struct socket *so)
#![allow(non_camel_case_types, non_snake_case, dead_code, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type socklen_t = c_uint;
type u_char = u8;

// ---------------------------------------------------------------------------
// Constants (from slirp headers)
// ---------------------------------------------------------------------------
const IPPROTO_ICMP: c_int = 1;

const ICMP_UNREACH: u_char = 3;
const ICMP_UNREACH_NET: u_char = 0;
const ICMP_UNREACH_HOST: u_char = 1;
const ICMP_UNREACH_PORT: u_char = 3;

const EHOSTUNREACH: c_int = 113;
const ENETUNREACH: c_int = 101;

const SO_EXPIREFAST: c_uint = 10000;
const SO_EXPIRE: c_uint = 240000;

const IF_MAXLINKHDR: usize = 2 + 14 + 40;

const FIONREAD: c_ulong = 0x541B;

const M_EXT: c_int = 0x01;

// ---------------------------------------------------------------------------
// C types mirrored
// ---------------------------------------------------------------------------
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcpcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcpiphdr {
    _private: [u8; 0],
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
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u_char,
    pub so_state: c_int,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: c_uint,
    // remaining fields (so_queued, so_nqueued, so_rcv, so_snd, extra) are not
    // touched by this function; offsets above are unaffected.
}

// ---------------------------------------------------------------------------
// External C functions / globals
// ---------------------------------------------------------------------------
extern "C" {
    fn recvfrom(
        fd: c_int,
        buf: *mut c_void,
        n: usize,
        flags: c_int,
        addr: *mut c_void,
        addr_len: *mut socklen_t,
    ) -> isize;

    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    fn strerror(errnum: c_int) -> *mut c_char;

    fn __errno_location() -> *mut c_int;

    fn htons(hostshort: u16) -> u16;

    fn udp_output(so: *mut socket, m: *mut mbuf, saddr: *mut sockaddr_in) -> c_int;
    fn udp_detach(so: *mut socket);
    fn m_get(slirp: *mut Slirp) -> *mut mbuf;
    fn m_free(m: *mut mbuf);
    fn m_inc(m: *mut mbuf, n: c_int);
    fn icmp_error(
        msrc: *mut mbuf,
        type_: u_char,
        code: u_char,
        minsize: c_int,
        message: *const c_char,
    );
    fn icmp_reflect(m: *mut mbuf);

    static mut curtime: c_uint;
}

// ---------------------------------------------------------------------------
// M_ROOM / M_FREEROOM macros
// ---------------------------------------------------------------------------
unsafe fn m_room(m: *mut mbuf) -> isize {
    let flags = (*m).m_hdr.mh_flags;
    let size = (*m).m_hdr.mh_size;
    let data = (*m).m_hdr.mh_data as isize;
    if (flags & M_EXT) != 0 {
        let base = core::ptr::addr_of!((*m).M_dat) as *const *mut c_char;
        let ext = *base as isize;
        ext.wrapping_add(size as isize).wrapping_sub(data)
    } else {
        let base = core::ptr::addr_of!((*m).M_dat) as isize;
        base.wrapping_add(size as isize).wrapping_sub(data)
    }
}

unsafe fn m_freeroom(m: *mut mbuf) -> c_int {
    m_room(m).wrapping_sub((*m).m_hdr.mh_len as isize) as c_int
}

fn get_errno() -> c_int {
    unsafe { *__errno_location() }
}

fn errno_icmp_code(err: c_int) -> u_char {
    let mut code: u_char = ICMP_UNREACH_PORT;
    if err == EHOSTUNREACH {
        code = ICMP_UNREACH_HOST;
    } else if err == ENETUNREACH {
        code = ICMP_UNREACH_NET;
    }
    code
}

// ---------------------------------------------------------------------------
// sorecvfrom
// ---------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn sorecvfrom(so: *mut socket) {
    unsafe {
        let mut addr: sockaddr_in = core::mem::zeroed();
        let mut addrlen: socklen_t = core::mem::size_of::<sockaddr_in>() as socklen_t;
        let addr_ptr = &mut addr as *mut sockaddr_in as *mut c_void;
        let addrlen_ptr: *mut socklen_t = &mut addrlen;

        if (*so).so_type as c_int == IPPROTO_ICMP {
            /* This is a "ping" reply */
            let mut buff = [0 as c_char; 256];
            let len: c_int = recvfrom(
                (*so).s,
                buff.as_mut_ptr() as *mut c_void,
                256,
                0,
                addr_ptr,
                addrlen_ptr,
            ) as c_int;

            if len == -1 || len == 0 {
                let err = get_errno();
                let code = errno_icmp_code(err);
                icmp_error((*so).so_m, ICMP_UNREACH, code, 0, strerror(err) as *const c_char);
            } else {
                icmp_reflect((*so).so_m);
                (*so).so_m = core::ptr::null_mut(); /* Don't m_free() it again! */
            }
            /* No need for this socket anymore, udp_detach it */
            udp_detach(so);
        } else {
            /* A "normal" UDP packet */
            let m: *mut mbuf = m_get((*so).slirp);
            if m.is_null() {
                return;
            }
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(IF_MAXLINKHDR);

            let mut len: c_int = m_freeroom(m);

            let mut n: c_int = 0;
            ioctl((*so).s, FIONREAD, &mut n as *mut c_int);

            if n > len {
                let dat_base = core::ptr::addr_of!((*m).M_dat) as isize;
                n = ((*m).m_hdr.mh_data as isize)
                    .wrapping_sub(dat_base)
                    .wrapping_add((*m).m_hdr.mh_len as isize)
                    .wrapping_add(n as isize)
                    .wrapping_add(1) as c_int;
                m_inc(m, n);
                len = m_freeroom(m);
            }

            (*m).m_hdr.mh_len =
                recvfrom((*so).s, (*m).m_hdr.mh_data as *mut c_void, len as usize, 0, addr_ptr, addrlen_ptr)
                    as c_int;

            if (*m).m_hdr.mh_len < 0 {
                let err = get_errno();
                let code = errno_icmp_code(err);
                icmp_error((*so).so_m, ICMP_UNREACH, code, 0, strerror(err) as *const c_char);
                m_free(m);
            } else {
                /*
                 * Hack: domain name lookup will be used the most for UDP, so
                 * time them out much quicker.
                 */
                if (*so).so_expire != 0 {
                    let now = core::ptr::addr_of!(curtime).read();
                    if (*so).so_fport == htons(53) {
                        (*so).so_expire = now.wrapping_add(SO_EXPIREFAST);
                    } else {
                        (*so).so_expire = now.wrapping_add(SO_EXPIRE);
                    }
                }

                /*
                 * If this packet was destined for CTL_ADDR, make it look like
                 * that's where it came from, done by udp_output.
                 */
                let _ = udp_output(so, m, &mut addr as *mut sockaddr_in);
            }
        }
    }
}
}

// ── `struct sockaddr_in addr; socklen_t addrlen = sizeof(struct sockaddr_in); ; ;` · slirp/socket.c:433-437 · chunk 2625 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct sockaddr_in addr;
//      socklen_t addrlen = sizeof(struct sockaddr_in);
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2625 {
// Port of the local-declaration piece of slirp/socket.c sorecvfrom():
//   struct sockaddr_in addr;
//   socklen_t addrlen = sizeof(struct sockaddr_in);
// The two empty statements that followed came from DEBUG_CALL/DEBUG_ARG.

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

#[no_mangle]
pub unsafe extern "C" fn sorecvfrom__part2625(
    addr: *mut SockAddrIn,
    addrlen: *mut u32,
) {
    if !addr.is_null() {
        // addr is uninitialized storage in C; give it a defined value.
        core::ptr::write_bytes(addr as *mut u8, 0, core::mem::size_of::<SockAddrIn>());
    }
    if !addrlen.is_null() {
        *addrlen = core::mem::size_of::<SockAddrIn>() as u32;
    }
}
}

// ── `if (so->so_type == IPPROTO_ICMP) … else` · slirp/socket.c:439-525 · chunk 2626 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_type == 
//                        IPPROTO_ICMP
//                                    ) {
//        char buff[256];
//        int len;
//     
//        len = recvfrom(so->s, buff, 256, 0,
//         (struct sockaddr *)&addr, &addrlen);
//     
//     
//        if(len == -1 || len == 0) {
//          u_char code=3;
//     
//          if(
//            (*__errno_location ()) 
//                  == 
//                     113
//                                 ) code=1;
//          else if(
//                 (*__errno_location ()) 
//                       == 
//                          101
//                                     ) code=0;
//     
//         
//                               ;
//          icmp_error(so->so_m, 3,code, 0,strerror(
//                                                            (*__errno_location ())
//                                                                 ));
//        } else {
//          icmp_reflect(so->so_m);
//                 so->so_m = 
//                           ((void *)0)
//                               ;
//        }
//     
//        udp_detach(so);
//      } else {
//        struct mbuf *m;
//               int len;
//     
//     
//     
//               int n;
//     
//     
//        m = m_get(so->slirp);
//        if (!m) {
//            return;
//        }
//        m->m_hdr.mh_data += (2 + 14 + 40);
//     
//     
//     
//     
//     
//        len = (((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len);
//     
//        ioctl(so->s, 
//                          0x541B
//                                  , &n);
//     
//        if (n > len) {
//          n = (m->m_hdr.mh_data - m->M_dat.m_dat_) + m->m_hdr.mh_len + n + 1;
//          m_inc(m, n);
//          len = (((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len);
//        }
//     
//     
//        m->m_hdr.mh_len = recvfrom(so->s, m->m_hdr.mh_data, len, 0,
//              (struct sockaddr *)&addr, &addrlen);
//       
//                                              ;
//        if(m->m_hdr.mh_len<0) {
//          u_char code=3;
//     
//          if(
//            (*__errno_location ()) 
//                  == 
//                     113
//                                 ) code=1;
//          else if(
//                 (*__errno_location ()) 
//                       == 
//                          101
//                                     ) code=0;
//     
//          ;
//          icmp_error(so->so_m, 3,code, 0,strerror(
//                                                            (*__errno_location ())
//                                                                 ));
//          m_free(m);
//        } else {
//     
//     
//     
//     
//     
//     
//          if (so->so_expire) {
//            if (so->so_fport == htons(53))
//       so->so_expire = curtime + 10000;
//            else
//       so->so_expire = curtime + 240000;
//          }
//     
//     
//     
//     
//     
//          udp_output(so, m, &addr);
//        }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2626 {
// Port of the ICMP/UDP receive branch of sorecvfrom().
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort};
use core::ffi::c_void;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: c_ushort,
    pub sin_port: c_ushort,
    pub sin_addr: InAddr,
    pub sin_zero: [c_uchar; 8],
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

pub struct Slirp {
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
pub union Mdat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: Mdat,
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
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn recvfrom(
        fd: c_int,
        buf: *mut c_void,
        n: usize,
        flags: c_int,
        addr: *mut c_void,
        addrlen: *mut c_uint,
    ) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn icmp_error(msrc: *mut Mbuf, r#type: c_uchar, code: c_uchar, minsize: c_int, message: *const c_char);
    fn icmp_reflect(m: *mut Mbuf);
    fn udp_detach(so: *mut Socket);
    fn udp_output(so: *mut Socket, m: *mut Mbuf, saddr: *mut SockaddrIn) -> c_int;
    fn m_get(slirp: *mut Slirp) -> *mut Mbuf;
    fn m_free(m: *mut Mbuf);
    fn m_inc(m: *mut Mbuf, n: c_int);
    fn htons(hostshort: c_ushort) -> c_ushort;
    static mut curtime: c_uint;
}

#[inline]
unsafe fn get_errno() -> c_int {
    *__errno_location()
}

// (((m)->m_hdr.mh_flags & 0x01) ? (m_ext_ + mh_size) - mh_data
//                              : (m_dat_ + mh_size) - mh_data) - mh_len
#[inline]
unsafe fn mbuf_room(m: *mut Mbuf) -> c_int {
    let flags = (*m).m_hdr.mh_flags;
    let base: *mut c_char = if (flags & 0x01) != 0 {
        (*m).M_dat.m_ext_
    } else {
        core::ptr::addr_of_mut!((*m).M_dat.m_dat_) as *mut c_char
    };
    let avail = (base as isize)
        .wrapping_add((*m).m_hdr.mh_size as isize)
        .wrapping_sub((*m).m_hdr.mh_data as isize)
        .wrapping_sub((*m).m_hdr.mh_len as isize);
    avail as c_int
}

#[inline]
unsafe fn errno_code() -> c_uchar {
    let mut code: c_uchar = 3;
    if get_errno() == 113 {
        code = 1;
    } else if get_errno() == 101 {
        code = 0;
    }
    code
}

#[no_mangle]
pub extern "C" fn sorecvfrom__part2626(
    so: *mut Socket,
    addr: *mut SockaddrIn,
    addrlen: *mut c_uint,
) -> c_int {
    unsafe {
        if (*so).so_type as c_int == 1 {
            let mut buff = [0 as c_char; 256];
            let len: c_int = recvfrom(
                (*so).s,
                buff.as_mut_ptr() as *mut c_void,
                256,
                0,
                addr as *mut c_void,
                addrlen,
            ) as c_int;

            if len == -1 || len == 0 {
                let code = errno_code();
                icmp_error((*so).so_m, 3, code, 0, strerror(get_errno()));
            } else {
                icmp_reflect((*so).so_m);
                (*so).so_m = core::ptr::null_mut();
            }

            udp_detach(so);
        } else {
            let m = m_get((*so).slirp);
            if m.is_null() {
                return 1;
            }
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_add((2 + 14 + 40) as usize);

            let mut len: c_int = mbuf_room(m);

            let mut n: c_int = 0;
            ioctl((*so).s, 0x541B as c_ulong, &mut n as *mut c_int);

            if n > len {
                let dat = core::ptr::addr_of_mut!((*m).M_dat.m_dat_) as *mut c_char as isize;
                n = ((*m).m_hdr.mh_data as isize)
                    .wrapping_sub(dat)
                    .wrapping_add((*m).m_hdr.mh_len as isize)
                    .wrapping_add(n as isize)
                    .wrapping_add(1) as c_int;
                m_inc(m, n);
                len = mbuf_room(m);
            }

            (*m).m_hdr.mh_len = recvfrom(
                (*so).s,
                (*m).m_hdr.mh_data as *mut c_void,
                len as usize,
                0,
                addr as *mut c_void,
                addrlen,
            ) as c_int;

            if (*m).m_hdr.mh_len < 0 {
                let code = errno_code();
                icmp_error((*so).so_m, 3, code, 0, strerror(get_errno()));
                m_free(m);
            } else {
                if (*so).so_expire != 0 {
                    if (*so).so_fport == htons(53) {
                        (*so).so_expire = curtime.wrapping_add(10000);
                    } else {
                        (*so).so_expire = curtime.wrapping_add(240000);
                    }
                }

                udp_output(so, m, addr);
            }
        }
    }
    0
}
}

// ── `sosendto` · slirp/socket.c:531-573 · chunk 2627 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     sosendto(struct socket *so, struct mbuf *m)
//     {
//      Slirp *slirp = so->slirp;
//      int ret;
//      struct sockaddr_in addr;
//     
//      ;
//      ;
//      ;
//     
//             addr.sin_family = 
//                              2
//                                     ;
//      if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//          slirp->vnetwork_addr.s_addr) {
//     
//        if (so->so_faddr.s_addr == slirp->vnameserver_addr.s_addr) {
//          if (get_dns_addr(&addr.sin_addr) < 0)
//            addr.sin_addr = loopback_addr;
//        } else {
//          addr.sin_addr = loopback_addr;
//        }
//      } else
//        addr.sin_addr = so->so_faddr;
//      addr.sin_port = so->so_fport;
//     
//      ;
//     
//     
//      ret = sendto(so->s, m->m_hdr.mh_data, m->m_hdr.mh_len, 0,
//            (struct sockaddr *)&addr, sizeof (struct sockaddr));
//      if (ret < 0)
//       return -1;
//     
//     
//     
//     
//     
//      if (so->so_expire)
//       so->so_expire = curtime + 240000;
//      so->so_state &= 0xf000;
//      so->so_state |= 0x004;
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2627 {
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// ---- C types mirrored from the preprocessed translation unit ----

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct sockaddr_in (glibc): sa_family_t (unsigned short), in_port_t
// (unsigned short), struct in_addr, unsigned char sin_zero[8]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
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

// union M_dat { char m_dat_[1]; char *m_ext_; } -> 8 bytes, pointer aligned
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: [u64; 1],
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
    pub so_ti: *mut c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

// Only the leading fields of struct Slirp are needed here.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
}

extern "C" {
    fn sendto(
        fd: c_int,
        buf: *const c_void,
        n: usize,
        flags: c_int,
        addr: *const c_void,
        addr_len: u32,
    ) -> isize;
    fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int;

    static mut curtime: c_uint;
    static mut loopback_addr: InAddr;
}

const AF_INET: u16 = 2;
const SO_EXPIRE: c_uint = 240000;
const SS_PERSISTENT_MASK: c_int = 0xf000;
const SS_ISFCONNECTED: c_int = 0x004;
const SOCKADDR_LEN: u32 = 16; // sizeof(struct sockaddr)

#[no_mangle]
pub extern "C" fn sosendto(so: *mut Socket, m: *mut Mbuf) -> c_int {
    unsafe {
        let slirp: *mut Slirp = (*so).slirp;
        let mut addr: SockaddrIn = core::mem::zeroed();

        addr.sin_family = AF_INET;
        let faddr: u32 = (*so).so_faddr.s_addr;
        if (faddr & (*slirp).vnetwork_mask.s_addr) == (*slirp).vnetwork_addr.s_addr {
            // It's an alias
            if faddr == (*slirp).vnameserver_addr.s_addr {
                if get_dns_addr(core::ptr::addr_of_mut!(addr.sin_addr)) < 0 {
                    addr.sin_addr = *core::ptr::addr_of!(loopback_addr);
                }
            } else {
                addr.sin_addr = *core::ptr::addr_of!(loopback_addr);
            }
        } else {
            addr.sin_addr = (*so).so_faddr;
        }
        addr.sin_port = (*so).so_fport;

        // Don't care what port we get
        let ret: c_int = sendto(
            (*so).s,
            (*m).m_hdr.mh_data as *const c_void,
            (*m).m_hdr.mh_len as usize,
            0,
            core::ptr::addr_of!(addr) as *const c_void,
            SOCKADDR_LEN,
        ) as c_int;
        if ret < 0 {
            return -1;
        }

        // Kill the socket if there's no reply in 4 minutes,
        // but only if it's an expirable socket
        if (*so).so_expire != 0 {
            (*so).so_expire = (*core::ptr::addr_of!(curtime)).wrapping_add(SO_EXPIRE);
        }
        (*so).so_state &= SS_PERSISTENT_MASK;
        (*so).so_state |= SS_ISFCONNECTED; // So that it gets select()ed
        0
    }
}
}

// ── `Slirp *slirp = so->slirp; int ret; struct sockaddr_in addr; ; ; ; addr.sin_family = 2 ;` · slirp/socket.c:534-542 · chunk 2628 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = so->slirp;
//      int ret;
//      struct sockaddr_in addr;
//     
//      ;
//      ;
//      ;
//     
//             addr.sin_family = 
//                              2
//                                     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2628 {
// Port of the opening piece of sosendto() from slirp/socket.c:
// it reads so->slirp and initializes addr.sin_family.

use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Only the leading fields are needed here; the remaining Slirp fields are
// opaque to this piece.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
}

// Only the leading fields are needed here; the remaining struct socket fields
// are opaque to this piece.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

#[no_mangle]
pub unsafe extern "C" fn sosendto__part2628(
    so: *mut Socket,
    addr: *mut SockaddrIn,
) -> *mut Slirp {
    let slirp = (*so).slirp;
    // addr is uninitialized C stack storage; only sin_family is written here.
    core::ptr::addr_of_mut!((*addr).sin_family).write(2u16);
    slirp
}
}

// ── `if (so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) == slirp->vnetwork_addr.s_addr) … else` · slirp/socket.c:543-553 · chunk 2629 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//          slirp->vnetwork_addr.s_addr) {
//     
//        if (so->so_faddr.s_addr == slirp->vnameserver_addr.s_addr) {
//          if (get_dns_addr(&addr.sin_addr) < 0)
//            addr.sin_addr = loopback_addr;
//        } else {
//          addr.sin_addr = loopback_addr;
//        }
//      } else
//        addr.sin_addr = so->so_faddr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2629 {
// Port of the "address selection" part of sosendto() from slirp/socket.c
#![allow(non_snake_case)]

use core::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int;
    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn sosendto__part2629(
    faddr: u32,
    vnetwork_mask: u32,
    vnetwork_addr: u32,
    vnameserver_addr: u32,
    sin_addr: *mut InAddr,
) {
    unsafe {
        if sin_addr.is_null() {
            return;
        }
        if (faddr & vnetwork_mask) == vnetwork_addr {
            // It's an alias
            if faddr == vnameserver_addr {
                if get_dns_addr(sin_addr) < 0 {
                    *sin_addr = loopback_addr;
                }
            } else {
                *sin_addr = loopback_addr;
            }
        } else {
            (*sin_addr).s_addr = faddr;
        }
    }
}
}

// ── `addr.sin_port = so->so_fport; ; ret = sendto(so->s, m->m_hdr.mh_data, m->m_hdr.mh_len, 0, (stru…` · slirp/socket.c:554-560 · chunk 2630 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     addr.sin_port = so->so_fport;
//     
//      ;
//     
//     
//      ret = sendto(so->s, m->m_hdr.mh_data, m->m_hdr.mh_len, 0,
//            (struct sockaddr *)&addr, sizeof (struct sockaddr));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2630 {
// Port of the sendto part of sosendto() (slirp/socket.c).
use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct Sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
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
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut c_void,
    pub m_dat_: [c_char; 1],
}

extern "C" {
    fn sendto(
        __fd: c_int,
        __buf: *const c_void,
        __n: usize,
        __flags: c_int,
        __addr: *const c_void,
        __addr_len: u32,
    ) -> c_long;
}

#[no_mangle]
pub unsafe extern "C" fn sosendto__part2630(
    so: *mut Socket,
    m: *mut Mbuf,
    addr: *mut SockaddrIn,
) -> c_int {
    (*addr).sin_port = (*so).so_fport;

    let ret = sendto(
        (*so).s,
        (*m).m_hdr.mh_data as *const c_void,
        (*m).m_hdr.mh_len as usize,
        0,
        addr as *const c_void,
        core::mem::size_of::<Sockaddr>() as u32,
    );

    ret as c_int
}
}

// ── `if (ret < 0)` · slirp/socket.c:561-562 · chunk 2631 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//       return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2631 {
// Piece of sosendto(): sign check on the sendto() return value.
// Returns 1 when the send failed (ret < 0) so the caller can `return -1;`,
// 0 otherwise. No C symbols are needed.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sosendto__part2631(ret: c_int) -> c_int {
    if ret < 0 {
        1
    } else {
        0
    }
}
}

// ── `if (so->so_expire)` · slirp/socket.c:568-569 · chunk 2632 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_expire)
//       so->so_expire = curtime + 240000;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2632 {
// Ported piece of sosendto(): refresh the socket expiration time.
// Reads/writes so->so_expire (u_int) and reads the global u_int curtime.
use core::ffi::c_uint;

extern "C" {
    static mut curtime: c_uint;
}

#[no_mangle]
pub extern "C" fn sosendto__part2632(so_expire: *mut c_uint) {
    unsafe {
        if so_expire.is_null() {
            return;
        }
        if *so_expire != 0 {
            *so_expire = curtime.wrapping_add(240000);
        }
    }
}
}

// ── `so->so_state &= 0xf000; so->so_state |= 0x004; return 0;` · slirp/socket.c:570-572 · chunk 2633 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= 0xf000;
//      so->so_state |= 0x004;
//      return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2633 {
// Ported piece of sosendto(): update so_state bits and report success.
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn sosendto__part2633(so: *mut Socket) -> c_int {
    unsafe {
        let state = (*so).so_state;
        (*so).so_state = state & 0xf000;
        let state = (*so).so_state;
        (*so).so_state = state | 0x004;
    }
    0
}
}

// ── `tcp_listen` · slirp/socket.c:578-649 · chunk 2634 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *
//     tcp_listen(Slirp *slirp, uint32_t haddr, u_int hport, uint32_t laddr,
//                u_int lport, int flags)
//     {
//      struct sockaddr_in addr;
//      struct socket *so;
//      int s, opt = 1;
//      socklen_t addrlen = sizeof(addr);
//      memset(&addr, 0, addrlen);
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
//     
//      so = socreate(slirp);
//      if (!so) {
//        return 
//              ((void *)0)
//                  ;
//      }
//     
//     
//      if ((so->so_tcpcb = tcp_newtcpcb(so)) == 
//                                              ((void *)0)
//                                                  ) {
//       free(so);
//       return 
//             ((void *)0)
//                 ;
//      }
//      slirp_insque(so, &slirp->tcb);
//     
//     
//     
//     
//      if (flags & 0x200)
//         so->so_tcpcb->t_timer[2] = ( 75*2)*2;
//     
//      so->so_state &= 0xf000;
//      so->so_state |= (0x100 | flags);
//      so->so_lport = lport;
//      so->so_laddr.s_addr = laddr;
//     
//      addr.sin_family = 
//                       2
//                              ;
//      addr.sin_addr.s_addr = haddr;
//      addr.sin_port = hport;
//     
//      if (((s = os_socket(
//                         2
//                                ,
//                                 SOCK_STREAM
//                                            ,0)) < 0) ||
//          (setsockopt(s,
//                       1
//                                 ,
//                                  2
//                                              ,(char *)&opt,sizeof(int)) < 0) ||
//          (bind(s,(struct sockaddr *)&addr, sizeof(addr)) < 0) ||
//          (listen(s,1) < 0)) {
//       int tmperrno = 
//                     (*__errno_location ())
//                          ;
//     
//       close(s);
//       sofree(so);
//     
//     
//     
//     
//       
//      (*__errno_location ()) 
//            = tmperrno;
//     
//       return 
//             ((void *)0)
//                 ;
//      }
//      setsockopt(s,
//                  1
//                            ,
//                             10
//                                         ,(char *)&opt,sizeof(int));
//     
//      getsockname(s,(struct sockaddr *)&addr,&addrlen);
//      so->so_fport = addr.sin_port;
//      if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr)
//         so->so_faddr = slirp->vhost_addr;
//      else
//         so->so_faddr = addr.sin_addr;
//     
//      so->s = s;
//      return so;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2634 {
// Rust rewrite of slirp's tcp_listen() (slirp/socket.c).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type socklen_t = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
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

// union M_dat { char m_dat_[1]; char *m_ext_; } -- pointer sized.
#[repr(C)]
pub struct m_dat {
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: m_dat,
}

#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Prefix of struct tcpcb, enough to reach t_timer[].
#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: i16,
    pub t_timer: [i16; 4],
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
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
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

pub struct ex_list;

// Prefix of struct Slirp, up to and including the embedded tcb list head.
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
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    pub ipq: ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: socket,
}

extern "C" {
    fn socreate(slirp: *mut Slirp) -> *mut socket;
    fn sofree(so: *mut socket);
    fn tcp_newtcpcb(so: *mut socket) -> *mut tcpcb;
    fn slirp_insque(new: *mut c_void, head: *mut c_void);
    fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn bind(fd: c_int, addr: *const c_void, len: socklen_t) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn getsockname(fd: c_int, addr: *mut c_void, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;
    static loopback_addr: in_addr;
}

const AF_INET: u16 = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_OOBINLINE: c_int = 10;
const SS_FACCEPTONCE: c_int = 0x200;
const SS_PERSISTENT_MASK: c_int = 0xf000;
const SS_FACCEPTCONN: c_int = 0x100;
const TCPT_KEEP: usize = 2;
const TCPTV_KEEP_INIT: i16 = 75 * 2;

#[no_mangle]
pub unsafe extern "C" fn tcp_listen(
    slirp: *mut Slirp,
    haddr: u32,
    hport: c_uint,
    laddr: u32,
    lport: c_uint,
    flags: c_int,
) -> *mut socket {
    let mut addr: sockaddr_in = core::mem::zeroed();
    let mut addrlen: socklen_t = core::mem::size_of::<sockaddr_in>() as socklen_t;

    let so = socreate(slirp);
    if so.is_null() {
        return core::ptr::null_mut();
    }

    // Don't tcp_attach... we don't need so_snd nor so_rcv
    let tcb = tcp_newtcpcb(so);
    (*so).so_tcpcb = tcb;
    if tcb.is_null() {
        free(so as *mut c_void);
        return core::ptr::null_mut();
    }
    slirp_insque(
        so as *mut c_void,
        core::ptr::addr_of_mut!((*slirp).tcb) as *mut c_void,
    );

    // SS_FACCEPTONCE sockets must time out.
    if flags & SS_FACCEPTONCE != 0 {
        (*tcb).t_timer[TCPT_KEEP] = TCPTV_KEEP_INIT * 2;
    }

    (*so).so_state &= SS_PERSISTENT_MASK;
    (*so).so_state |= SS_FACCEPTCONN | flags;
    (*so).so_lport = lport as u16; // Kept in network format
    (*so).so_laddr.s_addr = laddr; // Ditto

    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = haddr;
    addr.sin_port = hport as u16;

    let opt: c_int = 1;
    let s: c_int = os_socket(AF_INET as c_int, SOCK_STREAM, 0);
    let mut failed: bool = s < 0;
    if !failed {
        failed = setsockopt(
            s,
            SOL_SOCKET,
            SO_REUSEADDR,
            core::ptr::addr_of!(opt) as *const c_void,
            core::mem::size_of::<c_int>() as socklen_t,
        ) < 0;
    }
    if !failed {
        failed = bind(
            s,
            core::ptr::addr_of!(addr) as *const c_void,
            core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0;
    }
    if !failed {
        failed = listen(s, 1) < 0;
    }
    if failed {
        // Don't clobber the real reason we failed
        let tmperrno: c_int = *__errno_location();

        close(s);
        sofree(so);
        // Restore the real errno
        *__errno_location() = tmperrno;

        return core::ptr::null_mut();
    }
    setsockopt(
        s,
        SOL_SOCKET,
        SO_OOBINLINE,
        core::ptr::addr_of!(opt) as *const c_void,
        core::mem::size_of::<c_int>() as socklen_t,
    );

    getsockname(
        s,
        core::ptr::addr_of_mut!(addr) as *mut c_void,
        core::ptr::addr_of_mut!(addrlen),
    );
    (*so).so_fport = addr.sin_port;
    if addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr {
        (*so).so_faddr = (*slirp).vhost_addr;
    } else {
        (*so).so_faddr = addr.sin_addr;
    }

    (*so).s = s;
    so
}
}

// ── `struct sockaddr_in addr; struct socket *so; int s, opt = 1; socklen_t addrlen = sizeof(addr); m…` · slirp/socket.c:582-595 · chunk 2635 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct sockaddr_in addr;
//      struct socket *so;
//      int s, opt = 1;
//      socklen_t addrlen = sizeof(addr);
//      memset(&addr, 0, addrlen);
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
//     
//      so = socreate(slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2635 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

extern "C" {
    // struct socket * socreate(Slirp *);
    fn socreate(slirp: *mut c_void) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn tcp_listen__part2635(
    slirp: *mut c_void,
    addr: *mut SockAddrIn,
    s: *mut c_int,
    opt: *mut c_int,
    addrlen: *mut u32,
) -> *mut c_void {
    unsafe {
        // socklen_t addrlen = sizeof(addr);
        let alen = core::mem::size_of::<SockAddrIn>() as u32;
        if !addrlen.is_null() {
            *addrlen = alen;
        }
        // memset(&addr, 0, addrlen);
        if !addr.is_null() {
            let a = &mut *addr;
            a.sin_family = 0;
            a.sin_port = 0;
            a.sin_addr.s_addr = 0;
            a.sin_zero = [0u8; 8];
        }
        // int s, opt = 1;
        if !opt.is_null() {
            *opt = 1;
        }
        if !s.is_null() {
            *s = 0;
        }
        // so = socreate(slirp);
        socreate(slirp)
    }
}
}

// ── `if (!so)` · slirp/socket.c:596-598 · chunk 2636 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!so) {
//        return 
//              ((void *)0)
//                  ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2636 {
// Port of the `if (!so) { return NULL; }` guard inside tcp_listen().

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

/// Returns 1 when `so` is NULL (the caller must return NULL), 0 otherwise.
#[no_mangle]
pub extern "C" fn tcp_listen__part2636(so: *mut socket) -> core::ffi::c_int {
    if so.is_null() {
        1
    } else {
        0
    }
}
}

// ── `if (so->so_tcpcb = tcp_newtcpcb(so)) == ((void *)0)` · slirp/socket.c:601-604 · chunk 2637 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_tcpcb = tcp_newtcpcb(so)) == 
//                                              ((void *)0)
//                                                  ) {
//       free(so);
//       return 
//             ((void *)0)
//                 ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2637 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SocketPrefix {
    pub so_next: *mut SocketPrefix,
    pub so_prev: *mut SocketPrefix,
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
}

extern "C" {
    fn tcp_newtcpcb(so: *mut SocketPrefix) -> *mut c_void;
    fn free(p: *mut c_void);
}

// Returns 1 when tcp_newtcpcb failed (socket already freed), 0 on success.
#[no_mangle]
pub extern "C" fn tcp_listen__part2637(so: *mut SocketPrefix) -> c_int {
    unsafe {
        let tp = tcp_newtcpcb(so);
        if !so.is_null() {
            (*so).so_tcpcb = tp;
        }
        if tp.is_null() {
            free(so as *mut c_void);
            return 1;
        }
        0
    }
}
}

// ── `slirp_insque(so, &slirp->tcb);` · slirp/socket.c:605-605 · chunk 2638 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp_insque(so, &slirp->tcb);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2638 {
use core::ffi::c_void;

extern "C" {
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
}

// Ported TARGET: slirp_insque(so, &slirp->tcb);
#[no_mangle]
pub unsafe extern "C" fn tcp_listen__part2638(so: *mut c_void, head: *mut c_void) {
    unsafe {
        slirp_insque(so, head);
    }
}
}

// ── `if (flags & 0x200)` · slirp/socket.c:610-611 · chunk 2639 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & 0x200)
//         so->so_tcpcb->t_timer[2] = ( 75*2)*2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2639 {
// Port of the SS_FACCEPTONCE timeout setup from tcp_listen().
// Reads: flags (int, by value), so->so_tcpcb (pointer to struct tcpcb).
// Writes: so->so_tcpcb->t_timer[TCPT_KEEP] (= index 2).

use core::ffi::{c_int, c_short};

// Mirror of the leading fields of `struct tcpcb` up to t_timer, so that the
// offset of t_timer is identical to the C layout.
#[repr(C)]
struct TcpcbPrefix {
    seg_next: *mut core::ffi::c_void,
    seg_prev: *mut core::ffi::c_void,
    t_state: c_short,
    t_timer: [c_short; 4],
}

#[no_mangle]
pub extern "C" fn tcp_listen__part2639(flags: c_int, tp: *mut core::ffi::c_void) {
    if flags & 0x200 != 0 {
        // TCPT_KEEP == 2, TCPTV_KEEP_INIT == 150, value is TCPTV_KEEP_INIT * 2
        unsafe {
            let tcp = tp as *mut TcpcbPrefix;
            (*tcp).t_timer[2] = 150 * 2;
        }
    }
}
}

// ── `so->so_state &= 0xf000; so->so_state |= (0x100 | flags); so->so_lport = lport; so->so_laddr.s_a…` · slirp/socket.c:613-620 · chunk 2640 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= 0xf000;
//      so->so_state |= (0x100 | flags);
//      so->so_lport = lport;
//      so->so_laddr.s_addr = laddr;
//     
//      addr.sin_family = 
//                       2
//                              ;
//      addr.sin_addr.s_addr = haddr;
//      addr.sin_port = hport;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2640 {
// Part of tcp_listen(): socket state/listen address initialisation.
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

// Mirror of `struct socket` up to and including so_state (same field order/types).
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
}

#[no_mangle]
pub extern "C" fn tcp_listen__part2640(
    so: *mut Socket,
    addr: *mut SockaddrIn,
    flags: c_int,
    lport: c_uint,
    laddr: u32,
    haddr: u32,
    hport: c_uint,
) {
    unsafe {
        let state = (*so).so_state;
        (*so).so_state = (state & 0xf000) | (0x100 | flags);
        (*so).so_lport = lport as u16;
        (*so).so_laddr.s_addr = laddr;

        (*addr).sin_family = 2;
        (*addr).sin_addr.s_addr = haddr;
        (*addr).sin_port = hport as u16;
    }
}
}

// ── `if (s = os_socket( 2 , SOCK_STREAM ,0)) < 0) || (setsockopt(s, 1 , 2 ,(char *)&opt,sizeof(int))…` · slirp/socket.c:622-637 · chunk 2641 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (((s = os_socket(
//                         2
//                                ,
//                                 SOCK_STREAM
//                                            ,0)) < 0) ||
//          (setsockopt(s,
//                       1
//                                 ,
//                                  2
//                                              ,(char *)&opt,sizeof(int)) < 0) ||
//          (bind(s,(struct sockaddr *)&addr, sizeof(addr)) < 0) ||
//          (listen(s,1) < 0)) {
//       int tmperrno = 
//                     (*__errno_location ())
//                          ;
//     
//       close(s);
//       sofree(so);
//     
//     
//     
//     
//       
//      (*__errno_location ()) 
//            = tmperrno;
//     
//       return 
//             ((void *)0)
//                 ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2641 {
// Port of the socket setup/error path of tcp_listen (slirp/socket.c).
// Returns 0 on success, 1 if the socket setup failed (caller must return NULL).

use core::ffi::{c_int, c_void};

type SocklenT = u32;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

extern "C" {
    fn os_socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn bind(fd: c_int, addr: *const c_void, len: SocklenT) -> c_int;
    fn listen(fd: c_int, n: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sofree(so: *mut c_void);
    fn __errno_location() -> *mut c_int;
}

#[no_mangle]
pub extern "C" fn tcp_listen__part2641(
    s_out: *mut c_int,
    addr: *const SockaddrIn,
    opt: c_int,
    so: *mut c_void,
) -> c_int {
    unsafe {
        let s = os_socket(2, 1 /* SOCK_STREAM */, 0);
        *s_out = s;

        let mut failed = s < 0;
        if !failed {
            if setsockopt(
                s,
                1,
                2,
                &opt as *const c_int as *const c_void,
                core::mem::size_of::<c_int>() as SocklenT,
            ) < 0
            {
                failed = true;
            } else if bind(
                s,
                addr as *const c_void,
                core::mem::size_of::<SockaddrIn>() as SocklenT,
            ) < 0
            {
                failed = true;
            } else if listen(s, 1) < 0 {
                failed = true;
            }
        }

        if failed {
            let tmperrno = *__errno_location();
            close(s);
            sofree(so);
            *__errno_location() = tmperrno;
            return 1;
        }
        0
    }
}
}

// ── `setsockopt(s, 1 , 10 ,(char *)&opt,sizeof(int)); getsockname(s,(struct sockaddr *)&addr,&addrle…` · slirp/socket.c:638-641 · chunk 2642 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     setsockopt(s,
//                  1
//                            ,
//                             10
//                                         ,(char *)&opt,sizeof(int));
//     
//      getsockname(s,(struct sockaddr *)&addr,&addrlen);
//      so->so_fport = addr.sin_port;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2642 {
// Port of the tail-end of tcp_listen(): set SO_KEEPALIVE, fetch the local
// socket name and record the peer port in the slirp socket.
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

// C: typedef unsigned int __socklen_t; typedef __socklen_t socklen_t;
type socklen_t = u32;
type u_int = u32;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// C: struct __attribute__ ((__may_alias__)) sockaddr_in
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

// C: struct sbuf
#[repr(C)]
pub struct sbuf {
    pub sb_cc: u_int,
    pub sb_datalen: u_int,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// C: struct socket (exact field order and types)
#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
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
    pub so_expire: u_int,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn setsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *const c_void,
        __optlen: socklen_t,
    ) -> c_int;
    fn getsockname(__fd: c_int, __addr: *mut sockaddr_in, __len: *mut socklen_t) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_listen__part2642(
    s: c_int,
    opt: *const c_int,
    so: *mut socket,
    addr: *mut sockaddr_in,
    addrlen: *mut socklen_t,
) {
    // setsockopt(s, SOL_SOCKET, SO_KEEPALIVE, (char *)&opt, sizeof(int));
    setsockopt(
        s,
        1,
        10,
        opt as *const c_void,
        core::mem::size_of::<c_int>() as socklen_t,
    );

    // getsockname(s, (struct sockaddr *)&addr, &addrlen);
    getsockname(s, addr, addrlen);

    // so->so_fport = addr.sin_port;
    (*so).so_fport = (*addr).sin_port;
}
}

// ── `if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr) … else` · slirp/socket.c:642-645 · chunk 2643 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr)
//         so->so_faddr = slirp->vhost_addr;
//      else
//         so->so_faddr = addr.sin_addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2643 {
// Ported fragment of tcp_listen() from slirp/socket.c:
//   if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr)
//       so->so_faddr = slirp->vhost_addr;
//   else
//       so->so_faddr = addr.sin_addr;
use core::ffi::{c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

// Prefix of `struct Slirp` in declaration order, up to `vhost_addr`.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
}

// Prefix of `struct socket` in declaration order, up to `so_faddr`.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
}

extern "C" {
    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn tcp_listen__part2643(
    so: *mut Socket,
    addr: *mut SockaddrIn,
    slirp: *mut Slirp,
) {
    unsafe {
        let a = (*addr).sin_addr.s_addr;
        let lb = loopback_addr.s_addr;
        let faddr = if a == 0 || a == lb {
            (*slirp).vhost_addr
        } else {
            (*addr).sin_addr
        };
        (*so).so_faddr = faddr;
    }
}
}

// ── `so->s = s; return so;` · slirp/socket.c:647-648 · chunk 2644 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->s = s;
//      return so;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2644 {
// Ported tail of tcp_listen: store the listening socket fd and return the socket.

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

#[repr(C)]
pub struct Slirp {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
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

#[no_mangle]
pub unsafe extern "C" fn tcp_listen__part2644(so: *mut Socket, s: c_int) -> *mut Socket {
    (*so).s = s;
    so
}
}

// ── `soisfconnecting` · slirp/socket.c:657-663 · chunk 2645 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     soisfconnecting(struct socket *so)
//     {
//      so->so_state &= ~(0x001|0x004|0x008|
//          0x010|0x040);
//      so->so_state |= 0x002;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2645 {
// Port of slirp/socket.c soisfconnecting().
use core::ffi::{c_int, c_uchar};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: c_uchar,
    pub so_state: c_int,
}

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_ISFCONNECTED: c_int = 0x004;
const SS_FCANTRCVMORE: c_int = 0x008;
const SS_FCANTSENDMORE: c_int = 0x010;
const SS_FWDRAIN: c_int = 0x040;

#[no_mangle]
pub unsafe extern "C" fn soisfconnecting(so: *mut Socket) {
    if so.is_null() {
        return;
    }
    let so = &mut *so;
    so.so_state &= !(SS_NOFDREF | SS_ISFCONNECTED | SS_FCANTRCVMORE | SS_FCANTSENDMORE | SS_FWDRAIN);
    so.so_state |= SS_ISFCONNECTING;
}
}

// ── `so->so_state &= ~(0x001|0x004|0x008| 0x010|0x040); so->so_state |= 0x002;` · slirp/socket.c:660-662 · chunk 2646 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= ~(0x001|0x004|0x008|
//          0x010|0x040);
//      so->so_state |= 0x002;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2646 {
use core::ffi::c_int;

// Mirror of the part of `struct socket` reached through the pointer.
// Only the fields up to and including `so_state` matter for this piece,
// but the layout must be exact so the offsets match.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: u32,
    pub so_laddr: u32,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
}

#[no_mangle]
pub extern "C" fn soisfconnecting__part2646(so: *mut Socket) {
    unsafe {
        if so.is_null() {
            return;
        }
        let mask: c_int = !(0x001i32 | 0x004 | 0x008 | 0x010 | 0x040);
        (*so).so_state &= mask;
        (*so).so_state |= 0x002;
    }
}
}

// ── `soisfconnected` · slirp/socket.c:665-670 · chunk 2647 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     soisfconnected(struct socket *so)
//     {
//      so->so_state &= ~(0x002|0x040|0x001);
//      so->so_state |= 0x004;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2647 {
// Rust rewrite of slirp/socket.c: soisfconnected(struct socket *so)
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_uint, c_void};

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

// Opaque types only used behind pointers; layout never inspected here.
#[repr(C)]
pub struct Slirp {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct mbuf {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct tcpiphdr {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct tcpcb {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn soisfconnected(so: *mut socket) {
    if so.is_null() {
        return;
    }
    let state = &mut (*so).so_state;
    // so->so_state &= ~(SS_ISFCONNECTING|SS_FWDRAIN|SS_NOFDREF);
    *state &= !(0x002 | 0x040 | 0x001);
    // so->so_state |= SS_ISFCONNECTED; /* Clobber other states */
    *state |= 0x004;
}
}

// ── `so->so_state &= ~(0x002|0x040|0x001); so->so_state |= 0x004;` · slirp/socket.c:668-669 · chunk 2648 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= ~(0x002|0x040|0x001);
//      so->so_state |= 0x004;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2648 {
// Rust port of the two assignments in soisfconnected().
use core::ffi::{c_int, c_uchar, c_uint, c_ushort};

pub type UChar = c_uchar;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sbuff {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: c_ushort,
    pub so_lport: c_ushort,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: UChar,
    pub so_state: c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuff,
    pub so_snd: Sbuff,
    pub extra: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn soisfconnected__part2648(so: *mut Socket) {
    if so.is_null() {
        return;
    }
    // (*(so)).so_state &= ~(0x002|0x040|0x001);
    (*so).so_state &= !(0x002 | 0x040 | 0x001);
    // (*(so)).so_state |= 0x004;
    (*so).so_state |= 0x004;
}
}

// ── `sofcantrcvmore` · slirp/socket.c:672-688 · chunk 2649 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     sofcantrcvmore(struct socket *so)
//     {
//      if ((so->so_state & 0x001) == 0) {
//       shutdown(so->s,0);
//       if(global_writefds) {
//         
//        ((void) (((
//        global_writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                                     ;
//       }
//      }
//      so->so_state &= ~(0x002);
//      if (so->so_state & 0x010) {
//         so->so_state &= 0xf000;
//         so->so_state |= 0x001;
//      } else {
//         so->so_state |= 0x008;
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2649 {
// Port of slirp/socket.c sofcantrcvmore()
use core::ffi::{c_char, c_int, c_long, c_uint};

type SbufFields = ();

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct Sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[repr(C)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut core::ffi::c_void,
    so_m: *mut core::ffi::c_void,
    so_ti: *mut core::ffi::c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut core::ffi::c_void,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: Sbuf,
    so_snd: Sbuf,
    extra: *mut core::ffi::c_void,
}

#[repr(C)]
struct FdSet {
    fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_FCANTRCVMORE: c_int = 0x008;
const SS_FCANTSENDMORE: c_int = 0x010;
const SS_PERSISTENT_MASK: c_int = 0xf000;

extern "C" {
    fn shutdown(__fd: c_int, __how: c_int) -> c_int;
    static mut global_writefds: *mut FdSet;
}

#[no_mangle]
pub unsafe extern "C" fn sofcantrcvmore(so: *mut Socket) {
    let _ = core::mem::size_of::<SbufFields>();

    if (unsafe { (*so).so_state } & SS_NOFDREF) == 0 {
        unsafe {
            shutdown((*so).s, 0);
        }
        if unsafe { global_writefds } != core::ptr::null_mut() {
            let s = unsafe { (*so).s };
            let fd_mask_bits: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
            let idx = s.wrapping_div(fd_mask_bits) as isize;
            let bit = 1u64.wrapping_shl((s.wrapping_rem(fd_mask_bits)) as u32) as c_long;
            unsafe {
                let bits = (*global_writefds).fds_bits.as_mut_ptr().offset(idx);
                *bits &= !bit;
            }
        }
    }

    unsafe {
        (*so).so_state &= !SS_ISFCONNECTING;
        if ((*so).so_state & SS_FCANTSENDMORE) != 0 {
            (*so).so_state &= SS_PERSISTENT_MASK;
            (*so).so_state |= SS_NOFDREF;
        } else {
            (*so).so_state |= SS_FCANTRCVMORE;
        }
    }
}
}

// ── `if (so->so_state & 0x001) == 0)` · slirp/socket.c:675-680 · chunk 2650 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_state & 0x001) == 0) {
//       shutdown(so->s,0);
//       if(global_writefds) {
//         
//        ((void) (((
//        global_writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                                     ;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2650 {
use core::ffi::{c_int, c_long};

// Mirror of C `fd_set` as seen by the translation unit:
//   typedef struct { __fd_mask fds_bits[1024 / (8 * (int) sizeof(__fd_mask))]; } fd_set;
// with `typedef long int __fd_mask;`
#[repr(C)]
struct FdSet {
    fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

extern "C" {
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    static mut global_writefds: *mut FdSet;
}

#[no_mangle]
pub extern "C" fn sofcantrcvmore__part2650(s: c_int, so_state: c_int) {
    unsafe {
        if (so_state & 0x001) == 0 {
            shutdown(s, 0);
            let wfds = global_writefds;
            if !wfds.is_null() {
                let nbits: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
                let idx: c_int = s / nbits;
                let sh: c_int = s % nbits;
                let slot = (*wfds).fds_bits.as_mut_ptr().add(idx as usize);
                let mask: u64 = 1u64.wrapping_shl(sh as u32);
                *slot = *slot & !(mask as c_long);
            }
        }
    }
}
}

// ── `so->so_state &= ~(0x002);` · slirp/socket.c:681-681 · chunk 2651 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= ~(0x002);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2651 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: [u8; 4],
    pub so_laddr: [u8; 4],
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
}

#[no_mangle]
pub extern "C" fn sofcantrcvmore__part2651(so: *mut Socket) {
    unsafe {
        if !so.is_null() {
            (*so).so_state &= !0x002;
        }
    }
}
}

// ── `if (so->so_state & 0x010) … else` · slirp/socket.c:682-687 · chunk 2652 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_state & 0x010) {
//         so->so_state &= 0xf000;
//         so->so_state |= 0x001;
//      } else {
//         so->so_state |= 0x008;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2652 {
// Port of the tail of sofcantrcvmore: updates the socket state flags.
// The only thing read and written is so->so_state (an int), so it is passed
// by pointer.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sofcantrcvmore__part2652(so_state: *mut c_int) {
    unsafe {
        if so_state.is_null() {
            return;
        }
        let st: c_int = *so_state;
        if st & 0x010 != 0 {
            let mut v: c_int = st & 0xf000;
            v |= 0x001;
            *so_state = v;
        } else {
            *so_state = st | 0x008;
        }
    }
}
}

// ── `sofcantsendmore` · slirp/socket.c:690-709 · chunk 2653 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     sofcantsendmore(struct socket *so)
//     {
//      if ((so->so_state & 0x001) == 0) {
//                 shutdown(so->s,1);
//                 if (global_readfds) {
//                     
//                    ((void) (((
//                    global_readfds
//                    )->fds_bits)[((
//                    so->s
//                    ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//                    so->s
//                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                                ;
//                 }
//                 if (global_xfds) {
//                     
//                    ((void) (((
//                    global_xfds
//                    )->fds_bits)[((
//                    so->s
//                    ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//                    so->s
//                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                             ;
//                 }
//      }
//      so->so_state &= ~(0x002);
//      if (so->so_state & 0x008) {
//         so->so_state &= 0xf000;
//         so->so_state |= 0x001;
//      } else {
//         so->so_state |= 0x010;
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2653 {
// Port of slirp/socket.c sofcantsendmore()
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

// ---- types mirrored from the preprocessed C translation unit ----

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

// __fd_mask is `long int`
pub type __fd_mask = c_long;

const FD_SET_NBITS: usize = 1024;
const FD_MASK_BITS: usize = 8 * core::mem::size_of::<__fd_mask>();

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; FD_SET_NBITS / FD_MASK_BITS],
}

// Prefix of `struct socket` up to and including `so_state`.  Every field
// before so_state is reproduced exactly so that its offset matches the C
// definition; the remainder of the C struct is opaque to this function.
#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
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
}

extern "C" {
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    static mut global_readfds: *mut fd_set;
    static mut global_xfds: *mut fd_set;
}

#[inline]
unsafe fn fd_clr(fds: *mut fd_set, fd: c_int) {
    if fds.is_null() {
        return;
    }
    let idx = (fd / FD_MASK_BITS as c_int) as usize;
    let bit = ((fd % FD_MASK_BITS as c_int) as u32) & (FD_MASK_BITS as u32 - 1);
    let bits = (*fds).fds_bits.as_mut_ptr();
    let cur = *bits.add(idx);
    *bits.add(idx) = cur & !((1u64 << bit) as __fd_mask);
}

#[no_mangle]
pub unsafe extern "C" fn sofcantsendmore(so: *mut socket) {
    let so = match so.as_mut() {
        Some(s) => s,
        None => return,
    };

    if (so.so_state & 0x001) == 0 {
        shutdown(so.s, 1); // send FIN to fhost
        if !global_readfds.is_null() {
            fd_clr(global_readfds, so.s);
        }
        if !global_xfds.is_null() {
            fd_clr(global_xfds, so.s);
        }
    }
    so.so_state &= !0x002;
    if (so.so_state & 0x008) != 0 {
        so.so_state &= 0xf000;
        so.so_state |= 0x001;
    } else {
        so.so_state |= 0x010;
    }
    let _ = core::mem::size_of::<c_char>();
    let _ = core::mem::size_of::<c_uint>();
}
}

// ── `if (so->so_state & 0x001) == 0)` · slirp/socket.c:693-701 · chunk 2654 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_state & 0x001) == 0) {
//                 shutdown(so->s,1);
//                 if (global_readfds) {
//                     
//                    ((void) (((
//                    global_readfds
//                    )->fds_bits)[((
//                    so->s
//                    ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//                    so->s
//                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                                ;
//                 }
//                 if (global_xfds) {
//                     
//                    ((void) (((
//                    global_xfds
//                    )->fds_bits)[((
//                    so->s
//                    ) / (8 * (int) sizeof (__fd_mask)))] &= ~((__fd_mask) (1UL << ((
//                    so->s
//                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                             ;
//                 }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2654 {
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sb {
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
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16],
}

unsafe extern "C" {
    fn shutdown(__fd: c_int, __how: c_int) -> c_int;
    static mut global_readfds: *mut FdSet;
    static mut global_xfds: *mut FdSet;
}

unsafe fn clear_bit(set: *mut FdSet, fd: c_int) {
    if set.is_null() {
        return;
    }
    let bits_per_word: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
    let idx = fd / bits_per_word;
    // 1UL << (fd % bits_per_word), then cast to __fd_mask (long) and complemented.
    let shift = (fd % bits_per_word) as u32;
    let mask: c_ulong = (1 as c_ulong).wrapping_shl(shift);
    let target = (*set).fds_bits.as_mut_ptr().offset(idx as isize);
    *target &= !(mask as c_long);
}

#[no_mangle]
pub unsafe extern "C" fn sofcantsendmore__part2654(so: *mut Socket) {
    let state = (*so).so_state;
    if (state & 0x001) == 0 {
        shutdown((*so).s, 1);
        clear_bit(global_readfds, (*so).s);
        clear_bit(global_xfds, (*so).s);
    }
}
}

// ── `so->so_state &= ~(0x002);` · slirp/socket.c:702-702 · chunk 2655 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= ~(0x002);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2655 {
// Port of the single statement `so->so_state &= ~(0x002);` from
// sofcantsendmore() in slirp/socket.c.

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Prefix of `struct socket` up to (and including) `so_state`; only the
// layout matters, since the pointer is handed over by the C caller.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: core::ffi::c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: core::ffi::c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: core::ffi::c_int,
}

#[no_mangle]
pub extern "C" fn sofcantsendmore__part2655(so: *mut Socket) {
    unsafe {
        let p = core::ptr::addr_of_mut!((*so).so_state);
        let v = core::ptr::read(p);
        core::ptr::write(p, v & !0x002);
    }
}
}

// ── `if (so->so_state & 0x008) … else` · slirp/socket.c:703-708 · chunk 2656 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_state & 0x008) {
//         so->so_state &= 0xf000;
//         so->so_state |= 0x001;
//      } else {
//         so->so_state |= 0x010;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2656 {
// Port of slirp/socket.c sofcantsendmore tail state transition.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SoSocket {
    pub so_next: *mut SoSocket,
    pub so_prev: *mut SoSocket,
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
}

#[no_mangle]
pub extern "C" fn sofcantsendmore__part2656(so: *mut SoSocket) {
    if so.is_null() {
        return;
    }
    unsafe {
        let state = (*so).so_state;
        if state & 0x008 != 0 {
            (*so).so_state = (state & 0xf000) | 0x001;
        } else {
            (*so).so_state = state | 0x010;
        }
    }
}
}

// ── `sofwdrain` · slirp/socket.c:715-722 · chunk 2657 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     sofwdrain(struct socket *so)
//     {
//      if (so->so_rcv.sb_cc)
//       so->so_state |= 0x040;
//      else
//       sofcantsendmore(so);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2657 {
// Port of slirp/socket.c: sofwdrain()
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_void};

pub type __fd_mask = c_long;

pub const FD_SETSIZE: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; FD_SETSIZE / (8 * core::mem::size_of::<__fd_mask>())],
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
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
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
    fn shutdown(__fd: c_int, __how: c_int) -> c_int;
    static mut global_readfds: *mut fd_set;
    static mut global_writefds: *mut fd_set;
    static mut global_xfds: *mut fd_set;
}

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_ISFCONNECTED: c_int = 0x004;
const SS_FCANTSENDMORE: c_int = 0x008;
const SS_FCANTRCVMORE: c_int = 0x010;
const SS_FWDRAIN: c_int = 0x040;
const SS_PERSISTENT_MASK: c_int = 0xf000;

const FD_BITS_PER_MASK: c_int = (8 * core::mem::size_of::<__fd_mask>()) as c_int;

unsafe fn fd_clear(set: *mut fd_set, fd: c_int) {
    if set.is_null() {
        return;
    }
    let idx = (fd / FD_BITS_PER_MASK) as usize;
    let bit = (fd % FD_BITS_PER_MASK) as u32;
    let bits = (*set).fds_bits.as_mut_ptr().add(idx);
    *bits &= !((1u64 << bit) as __fd_mask);
}

unsafe fn sofcantsendmore(so: *mut socket) {
    if ((*so).so_state & SS_NOFDREF) == 0 {
        shutdown((*so).s, 1);
        if !global_readfds.is_null() {
            fd_clear(global_readfds, (*so).s);
        }
        if !global_xfds.is_null() {
            fd_clear(global_xfds, (*so).s);
        }
    }
    (*so).so_state &= !SS_ISFCONNECTING;
    if ((*so).so_state & SS_FCANTRCVMORE) != 0 {
        (*so).so_state &= SS_PERSISTENT_MASK;
        (*so).so_state |= SS_NOFDREF;
    } else {
        (*so).so_state |= SS_FCANTSENDMORE;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sofwdrain(so: *mut socket) {
    if (*so).so_rcv.sb_cc != 0 {
        (*so).so_state |= SS_FWDRAIN;
    } else {
        sofcantsendmore(so);
    }
}
}

// ── `if (so->so_rcv.sb_cc) … else` · slirp/socket.c:718-721 · chunk 2658 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_rcv.sb_cc)
//       so->so_state |= 0x040;
//      else
//       sofcantsendmore(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2658 {
// Port of the body of sofwdrain() from slirp/socket.c.
// The static helper sofcantsendmore() has no linker symbol, so its logic is
// ported here as a private Rust fn.

#[repr(C)]
pub struct Sb4 {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: core::ffi::c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: core::ffi::c_int,
    pub so_faddr: Sb4,
    pub so_laddr: Sb4,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: core::ffi::c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: u32,
    pub so_queued: core::ffi::c_int,
    pub so_nqueued: core::ffi::c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [core::ffi::c_long; 16],
}

const FD_MASK_BITS: core::ffi::c_int = 8 * core::mem::size_of::<core::ffi::c_long>() as core::ffi::c_int;

extern "C" {
    fn shutdown(fd: core::ffi::c_int, how: core::ffi::c_int) -> core::ffi::c_int;
    static mut global_readfds: *mut FdSet;
    static mut global_xfds: *mut FdSet;
}

unsafe fn fd_clear(set: *mut FdSet, fd: core::ffi::c_int) {
    let idx = (fd / FD_MASK_BITS) as usize;
    let bit = (1 as core::ffi::c_ulong) << ((fd % FD_MASK_BITS) as u32);
    (*set).fds_bits[idx] &= !(bit as core::ffi::c_long);
}

unsafe fn sofcantsendmore(so: *mut Socket) {
    if ((*so).so_state & 0x001) == 0 {
        shutdown((*so).s, 1);
        if !global_readfds.is_null() {
            fd_clear(global_readfds, (*so).s);
        }
        if !global_xfds.is_null() {
            fd_clear(global_xfds, (*so).s);
        }
    }
    (*so).so_state &= !(0x002);
    if ((*so).so_state & 0x008) != 0 {
        (*so).so_state &= 0xf000;
        (*so).so_state |= 0x001;
    } else {
        (*so).so_state |= 0x010;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sofwdrain__part2658(so: *mut Socket) {
    if (*so).so_rcv.sb_cc != 0 {
        (*so).so_state |= 0x040;
    } else {
        sofcantsendmore(so);
    }
}
}

