//! From `slirp/ip_output.c`.

// ── `Slirp *slirp = m0->slirp; register struct ip *ip; register struct mbuf *m = m0; register int hl…` · slirp/ip_output.c:56-73 · chunk 2400 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = m0->slirp;
//      register struct ip *ip;
//      register struct mbuf *m = m0;
//      register int hlen = sizeof(struct ip );
//      int len, off, error = 0;
//     
//      ;
//      ;
//      ;
//     
//      ip = ((struct ip *)(m)->m_hdr.mh_data);
//     
//     
//     
//      ip->ip_v = 4;
//      ip->ip_off &= 0x4000;
//      ip->ip_id = htons(slirp->ip_id++);
//      ip->ip_hl = hlen >> 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2400 {
// Port of the prologue of slirp's ip_output(): reads the mbuf/Slirp, derives the
// IP header pointer, and fills in ip_v / ip_off / ip_id / ip_hl.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

extern "C" {
    // libc network-order helper, exactly as declared in the translation unit
    fn htons(__hostshort: u16) -> u16;
}

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
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
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
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Only ever referenced through pointers inside struct socket; never dereferenced here.
#[repr(C)]
pub struct tcpiphdr {
    pub _opaque: [u8; 0],
}

// Only ever referenced through pointers inside struct socket; never dereferenced here.
#[repr(C)]
pub struct tcpcb {
    pub _opaque: [u8; 0],
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
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
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
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: socket,
    pub udp_last_so: *mut socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],
    pub opaque: *mut c_void,
}

// struct ip is __attribute__((packed)); the "u_int ip_hl:4, ip_v:4" bitfields
// occupy exactly the first byte (ip_hl in the low nibble, ip_v in the high one).
#[repr(C, packed)]
pub struct Ip {
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}

#[no_mangle]
pub extern "C" fn ip_output__part2400(
    m0: *mut mbuf,
    slirp_out: *mut *mut Slirp,
    hlen_out: *mut c_int,
) -> *mut Ip {
    unsafe {
        let slirp: *mut Slirp = (*m0).slirp;
        let ip: *mut Ip = (*m0).m_hdr.mh_data as *mut Ip;
        let hlen: c_int = core::mem::size_of::<Ip>() as c_int;

        // ip->ip_v = 4;   (high nibble of the first byte)
        let b0 = (*ip).ip_hl_v;
        (*ip).ip_hl_v = (b0 & 0x0f) | 0x40;

        // ip->ip_off &= 0x4000;
        let off = (*ip).ip_off;
        (*ip).ip_off = off & 0x4000u16;

        // ip->ip_id = htons(slirp->ip_id++);
        let id = (*slirp).ip_id;
        (*slirp).ip_id = id.wrapping_add(1);
        (*ip).ip_id = htons(id);

        // ip->ip_hl = hlen >> 2;  (low nibble of the first byte)
        let b1 = (*ip).ip_hl_v;
        (*ip).ip_hl_v = (b1 & 0xf0) | (((hlen >> 2) as u32) as u8 & 0x0f);

        if !slirp_out.is_null() {
            *slirp_out = slirp;
        }
        if !hlen_out.is_null() {
            *hlen_out = hlen;
        }

        ip
    }
}
}

// ── `if ((uint16_t)ip->ip_len <= 1500)` · slirp/ip_output.c:78-86 · chunk 2401 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((uint16_t)ip->ip_len <= 1500) {
//       ip->ip_len = htons((uint16_t)ip->ip_len);
//       ip->ip_off = htons((uint16_t)ip->ip_off);
//       ip->ip_sum = 0;
//       ip->ip_sum = cksum(m, hlen);
//     
//       if_output(so, m);
//       goto done;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2401 {
// Port of the "small packet" fast path of ip_output (slirp/ip_output.c).
// Returns 1 when the fast path was taken (and the packet was sent), 0 otherwise.

#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct ip is declared __attribute__((packed)) in the C source.
#[repr(C, packed)]
pub struct ip {
    pub ip_hl_v: u8, // u_int ip_hl:4, ip_v:4
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}

// Opaque views of types only used behind pointers here.
#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

extern "C" {
    fn htons(x: u16) -> u16;
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
    fn if_output(so: *mut socket, m: *mut mbuf);
}

#[no_mangle]
pub extern "C" fn ip_output__part2401(
    so: *mut socket,
    m: *mut mbuf,
    ip: *mut ip,
    hlen: c_int,
) -> c_int {
    unsafe {
        let len_field: u16 = (*ip).ip_len;
        // (uint16_t)ip->ip_len <= 1500, with integer promotion to int.
        if (len_field as c_int) <= 1500 {
            (*ip).ip_len = htons(len_field);
            (*ip).ip_off = htons((*ip).ip_off);
            (*ip).ip_sum = 0;
            let sum: c_int = cksum(m, hlen);
            (*ip).ip_sum = sum as u16;
            if_output(so, m);
            1
        } else {
            0
        }
    }
}
}

// ── `if (ip->ip_off & 0x4000)` · slirp/ip_output.c:92-95 · chunk 2402 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip->ip_off & 0x4000) {
//       error = -1;
//       goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2402 {
// Port of the "don't fragment" rejection check from ip_output():
// if (ip->ip_off & 0x4000) { error = -1; goto bad; }
// Returns nonzero when the DF bit is set, so the caller performs the
// error/bad control flow.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ip_output__part2402(ip_off: u16) -> c_int {
    if ((ip_off as c_int) & 0x4000) != 0 {
        1
    } else {
        0
    }
}
}

// ── `len = (1500 - hlen) &~ 7;` · slirp/ip_output.c:97-97 · chunk 2403 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     len = (1500 - hlen) &~ 7;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2403 {
// Extracted from slirp ip_output: compute the fragment payload length.
// C: len = (1500 - hlen) &~ 7;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ip_output__part2403(hlen: c_int) -> c_int {
    (1500i32.wrapping_sub(hlen)) & !7i32
}
}

// ── `if (len < 8)` · slirp/ip_output.c:98-101 · chunk 2404 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len < 8) {
//       error = -1;
//       goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2404 {
use core::ffi::c_int;

// TARGET: the "if (len < 8) { error = -1; goto bad; }" check in ip_output().
// Returns nonzero when the "bad" path must be taken; sets *error = -1 there.
#[no_mangle]
pub extern "C" fn ip_output__part2404(len: c_int, error: *mut c_int) -> c_int {
    if len < 8 {
        unsafe {
            *error = -1;
        }
        1
    } else {
        0
    }
}
}

// ── `{ … }` · slirp/ip_output.c:103-164 · chunk 2405 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     {
//      int mhlen, firstlen = len;
//      struct mbuf **mnext = &m->m_hdr.mh_nextpkt;
//     
//     
//     
//     
//     
//      m0 = m;
//      mhlen = sizeof (struct ip);
//      for (off = hlen + len; off < (uint16_t)ip->ip_len; off += len) {
//        register struct ip *mhip;
//        m = m_get(slirp);
//               if (m == 
//                       ((void *)0)
//                           ) {
//          error = -1;
//          goto sendorfree;
//        }
//        m->m_hdr.mh_data += (2 + 14 + 40);
//        mhip = ((struct ip *)(m)->m_hdr.mh_data);
//        *mhip = *ip;
//     
//        m->m_hdr.mh_len = mhlen;
//        mhip->ip_off = ((off - hlen) >> 3) + (ip->ip_off & ~0x2000);
//        if (ip->ip_off & 0x2000)
//          mhip->ip_off |= 0x2000;
//        if (off + len >= (uint16_t)ip->ip_len)
//          len = (uint16_t)ip->ip_len - off;
//        else
//          mhip->ip_off |= 0x2000;
//        mhip->ip_len = htons((uint16_t)(len + mhlen));
//     
//        if (m_copy(m, m0, off, len) < 0) {
//          error = -1;
//          goto sendorfree;
//        }
//     
//        mhip->ip_off = htons((uint16_t)mhip->ip_off);
//        mhip->ip_sum = 0;
//        mhip->ip_sum = cksum(m, mhlen);
//        *mnext = m;
//        mnext = &m->m_hdr.mh_nextpkt;
//      }
//     
//     
//     
//     
//      m = m0;
//      m_adj(m, hlen + firstlen - (uint16_t)ip->ip_len);
//      ip->ip_len = htons((uint16_t)m->m_hdr.mh_len);
//      ip->ip_off = htons((uint16_t)(ip->ip_off | 0x2000));
//      ip->ip_sum = 0;
//      ip->ip_sum = cksum(m, hlen);
//     sendorfree:
//      for (m = m0; m; m = m0) {
//       m0 = m->m_hdr.mh_nextpkt;
//                     m->m_hdr.mh_nextpkt = 
//                                   ((void *)0)
//                                       ;
//       if (error == 0)
//        if_output(so, m);
//       else
//        m_free(m);
//      }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2405 {
// Port of the fragmentation/send block of ip_output (slirp/ip_output.c).
// Locals m0, m and len are dead after this block in the enclosing function,
// so they are taken by value; only `error` is written back.

use core::ffi::{c_char, c_int, c_void};

// struct ip (packed, 20 bytes): the bitfield ip_hl:4, ip_v:4 occupies byte 0.
#[repr(C, packed)]
pub struct Ip {
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: u32,
    pub ip_dst: u32,
}

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
pub struct Mbuf {
    pub m_hdr: MbHdr,
    pub slirp: *mut c_void,
    pub m_dat: [u8; 8],
}

extern "C" {
    fn m_get(slirp: *mut c_void) -> *mut Mbuf;
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: c_int);
    fn m_copy(n: *mut Mbuf, m: *mut Mbuf, off: c_int, len: c_int) -> c_int;
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
    fn if_output(so: *mut c_void, m: *mut Mbuf);
    fn htons(x: u16) -> u16;
}

const IF_MF: c_int = 0x2000;

#[no_mangle]
pub extern "C" fn ip_output__part2405(
    so: *mut c_void,
    slirp: *mut c_void,
    ip: *mut Ip,
    hlen: c_int,
    m0_in: *mut Mbuf,
    m_in: *mut Mbuf,
    len_in: c_int,
    errorp: *mut c_int,
) {
    unsafe {
        let mut m0: *mut Mbuf = m0_in;
        let mut m: *mut Mbuf = m_in;
        let mut len: c_int = len_in;
        let mut off: c_int;
        let mut error: c_int = *errorp;

        let firstlen: c_int = len;
        let mut mnext: *mut *mut Mbuf = core::ptr::addr_of_mut!((*m).m_hdr.mh_nextpkt);

        m0 = m;
        let mhlen: c_int = core::mem::size_of::<Ip>() as c_int; /* sizeof(struct ip) */

        let mut goto_sendorfree: bool = false;

        off = hlen + len;
        loop {
            if off >= ((*ip).ip_len as c_int) {
                break;
            }

            let mhip: *mut Ip;

            m = m_get(slirp);
            if m.is_null() {
                error = -1;
                goto_sendorfree = true;
                break;
            }
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(2 + 14 + 40);
            mhip = (*m).m_hdr.mh_data as *mut Ip;
            /* *mhip = *ip; */
            core::ptr::copy_nonoverlapping(ip as *const u8, mhip as *mut u8, mhlen as usize);

            (*m).m_hdr.mh_len = mhlen;
            let base: c_int = ((off - hlen) >> 3) + (((*ip).ip_off as c_int) & !IF_MF);
            (*mhip).ip_off = base as u16;
            if (((*ip).ip_off as c_int) & IF_MF) != 0 {
                (*mhip).ip_off |= IF_MF as u16;
            }
            if off + len >= ((*ip).ip_len as c_int) {
                len = ((*ip).ip_len as c_int) - off;
            } else {
                (*mhip).ip_off |= IF_MF as u16;
            }
            (*mhip).ip_len = htons((len + mhlen) as u16);

            if m_copy(m, m0, off, len) < 0 {
                error = -1;
                goto_sendorfree = true;
                break;
            }

            (*mhip).ip_off = htons((*mhip).ip_off);
            (*mhip).ip_sum = 0;
            (*mhip).ip_sum = cksum(m, mhlen) as u16;
            *mnext = m;
            mnext = core::ptr::addr_of_mut!((*m).m_hdr.mh_nextpkt);
            off += len;
        }

        if !goto_sendorfree {
            m = m0;
            m_adj(m, hlen + firstlen - ((*ip).ip_len as c_int));
            (*ip).ip_len = htons((*m).m_hdr.mh_len as u16);
            (*ip).ip_off = htons(((((*ip).ip_off as c_int) | IF_MF)) as u16);
            (*ip).ip_sum = 0;
            (*ip).ip_sum = cksum(m, hlen) as u16;
        }

        /* sendorfree: */
        m = m0;
        while !m.is_null() {
            m0 = (*m).m_hdr.mh_nextpkt;
            (*m).m_hdr.mh_nextpkt = core::ptr::null_mut();
            if error == 0 {
                if_output(so, m);
            } else {
                m_free(m);
            }
            m = m0;
        }

        *errorp = error;
    }
}
}

// ── `done: return (error);` · slirp/ip_output.c:166-167 · chunk 2406 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     done:
//      return (error);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2406 {
// Port of the `done:` label / return tail of ip_output.
// The label and the `return` stay in C: we only hand the error value out and
// get exactly the same value back so the C `return` keeps its semantics.

#[no_mangle]
pub extern "C" fn ip_output__part2406(error: core::ffi::c_int) -> core::ffi::c_int {
    error
}
}

// ── `bad: m_free(m0);` · slirp/ip_output.c:169-170 · chunk 2407 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     bad:
//      m_free(m0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2407 {
use core::ffi::c_void;

extern "C" {
    // void m_free(struct mbuf *);
    fn m_free(m: *mut c_void);
}

// Replaces the `bad:` path of ip_output: free the mbuf chain head.
#[no_mangle]
pub extern "C" fn ip_output__part2407(m0: *mut c_void) {
    unsafe {
        m_free(m0);
    }
}
}

// ── `goto done;` · slirp/ip_output.c:171-171 · chunk 2408 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     goto done;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2408 {
// TARGET: `goto done;` at the end of the "small packet" fast path of ip_output().
//
// The jump itself must stay in C, so this helper only reports "take the done
// branch" (always nonzero). C keeps the actual `goto done;`.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ip_output__part2408() -> c_int {
    // Nonzero => the caller performs `goto done;` (i.e. `return error;`,
    // where error is still 0 on this path).
    1
}
}

