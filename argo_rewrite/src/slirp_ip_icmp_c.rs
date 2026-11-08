//! From `slirp/ip_icmp.c`.

// ── `register struct icmp *icp; register struct ip *ip=((struct ip *)(m)->m_hdr.mh_data); int icmple…` · slirp/ip_icmp.c:69-76 · chunk 2317 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct icmp *icp;
//       register struct ip *ip=((struct ip *)(m)->m_hdr.mh_data);
//       int icmplen=ip->ip_len;
//       Slirp *slirp = m->slirp;
//     
//       ;
//       ;
//       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2317 {
// Rust port of the prologue of slirp's icmp_input(): it computes the IP header
// pointer, the ICMP length and the Slirp instance pointer for the caller.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct ip` (packed, 20 bytes).
#[repr(C, packed)]
pub struct Ip {
    pub ip_vhl: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

// Opaque mirror of `typedef struct Slirp Slirp`.
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

// Mirror of `struct m_hdr`.
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

// Mirror of `struct mbuf`.
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
}

#[no_mangle]
pub unsafe extern "C" fn icmp_input__part2317(
    m: *mut Mbuf,
    pip: *mut *mut Ip,
    picmplen: *mut c_int,
    pslirp: *mut *mut Slirp,
) {
    let ip: *mut Ip = (*m).m_hdr.mh_data as *mut Ip;
    let icmplen: c_int = (*ip).ip_len as c_int;
    let slirp: *mut Slirp = (*m).slirp;

    *pip = ip;
    *picmplen = icmplen;
    *pslirp = slirp;
}
}

// ── `if (icmplen < 8)` · slirp/ip_icmp.c:82-86 · chunk 2318 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (icmplen < 8) {
//       freeit:
//         m_free(m);
//         goto end_error;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2318 {
// Ported piece of icmp_input: free the mbuf (`m_free(m)` on the freeit path).
use core::ffi::{c_char, c_int};

// struct Slirp / struct socket are only used through pointers here.
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
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
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; } -- size/alignment of a pointer.
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub m_dat: *mut c_char,
}

extern "C" {
    fn m_free(m: *mut mbuf);
}

#[no_mangle]
pub extern "C" fn icmp_input__part2318(m: *mut mbuf) {
    unsafe {
        m_free(m);
    }
}
}

// ── `m->m_hdr.mh_len -= hlen; m->m_hdr.mh_data += hlen; icp = ((struct icmp *)(m)->m_hdr.mh_data);` · slirp/ip_icmp.c:88-90 · chunk 2319 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_len -= hlen;
//       m->m_hdr.mh_data += hlen;
//       icp = ((struct icmp *)(m)->m_hdr.mh_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2319 {
// Port of icmp_input's header adjustment + icmp pointer computation.
// Returns the new data pointer (the value C assigns to `icp`).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MbHdr {
    pub mh_next: *mut MbHdr,
    pub mh_prev: *mut MbHdr,
    pub mh_nextpkt: *mut MbHdr,
    pub mh_prevpkt: *mut MbHdr,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[no_mangle]
pub extern "C" fn icmp_input__part2319(m: *mut MbHdr, hlen: c_int) -> *mut c_char {
    unsafe {
        let hdr = &mut *m;
        hdr.mh_len -= hlen;
        hdr.mh_data = hdr.mh_data.offset(hlen as isize);
        hdr.mh_data
    }
}
}

// ── `if (cksum(m, icmplen))` · slirp/ip_icmp.c:91-93 · chunk 2320 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cksum(m, icmplen)) {
//         goto freeit;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2320 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn cksum(m: *mut c_void, len: c_int) -> c_int;
}

// Returns non-zero when the ICMP checksum is bad (C: `goto freeit`).
#[no_mangle]
pub extern "C" fn icmp_input__part2320(m: *mut c_void, icmplen: c_int) -> c_int {
    unsafe {
        if cksum(m, icmplen) != 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `m->m_hdr.mh_len += hlen; m->m_hdr.mh_data -= hlen; ;` · slirp/ip_icmp.c:94-97 · chunk 2321 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_len += hlen;
//       m->m_hdr.mh_data -= hlen;
//     
//       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2321 {
// Ported piece of icmp_input(): restore the mbuf header after checksumming.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Mhdr {
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
    pub m_hdr: Mhdr,
}

#[no_mangle]
pub extern "C" fn icmp_input__part2321(m: *mut Mbuf, hlen: c_int) {
    unsafe {
        let mh = &mut (*m).m_hdr;
        mh.mh_len = mh.mh_len.wrapping_add(hlen);
        mh.mh_data = mh.mh_data.wrapping_offset(-(hlen as isize));
    }
}
}

// ── `switch (icp->icmp_type)` · slirp/ip_icmp.c:98-161 · chunk 2322 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (icp->icmp_type) {
//       case 8:
//         icp->icmp_type = 0;
//         ip->ip_len += hlen;
//         if (ip->ip_dst.s_addr == slirp->vhost_addr.s_addr) {
//           icmp_reflect(m);
//         } else {
//           struct socket *so;
//           struct sockaddr_in addr;
//           if ((so = socreate(slirp)) == 
//                                        ((void *)0)
//                                            ) goto freeit;
//           if(udp_attach(so) == -1) {
//     
//                                  ;
//      sofree(so);
//      m_free(m);
//      goto end_error;
//           }
//           so->so_m = m;
//           so->so_faddr = ip->ip_dst;
//           so->so_fport = htons(7);
//           so->so_laddr = ip->ip_src;
//           so->so_lport = htons(9);
//           so->so_iptos = ip->ip_tos;
//           so->so_type = 
//                        IPPROTO_ICMP
//                                    ;
//           so->so_state = 0x004;
//     
//     
//           addr.sin_family = 
//                            2
//                                   ;
//           if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//               slirp->vnetwork_addr.s_addr) {
//     
//      if (so->so_faddr.s_addr == slirp->vnameserver_addr.s_addr) {
//        if (get_dns_addr(&addr.sin_addr) < 0)
//          addr.sin_addr = loopback_addr;
//      } else {
//        addr.sin_addr = loopback_addr;
//      }
//           } else {
//      addr.sin_addr = so->so_faddr;
//           }
//           addr.sin_port = so->so_fport;
//           if(sendto(so->s, icmp_ping_msg, strlen(icmp_ping_msg), 0,
//       (struct sockaddr *)&addr, sizeof(addr)) == -1) {
//     
//                                  ;
//      icmp_error(m, 3,0, 0,strerror(
//                                                             (*__errno_location ())
//                                                                  ));
//      udp_detach(so);
//           }
//         }
//         break;
//       case 3:
//     
//       case 11:
//       case 12:
//       case 4:
//       case 13:
//       case 17:
//       case 5:
//         m_free(m);
//         break;
//     
//       default:
//         m_free(m);
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2322 {
// Port of the icmp_input() dispatch switch (slirp/ip_icmp.c lines 98-161).
// Returns 0 normally, 1 to request "goto freeit", 2 to request "goto end_error".

use core::ffi::{c_char, c_int, c_uchar, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [c_uchar; 8],
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
}

#[repr(C)]
pub struct IpUf {
    pub ip_hl_v: c_uchar,
    pub ip_tos: c_uchar,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: c_uchar,
    pub ip_p: c_uchar,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
pub struct IcmpUf {
    pub icmp_type: c_uchar,
    pub icmp_code: c_uchar,
    pub icmp_cksum: u16,
}

#[repr(C)]
pub struct SocketUf {
    pub so_next: *mut SocketUf,
    pub so_prev: *mut SocketUf,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: c_uchar,
    pub so_emu: c_uchar,
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_int,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
}

extern "C" {
    fn icmp_reflect(m: *mut c_void);
    fn icmp_error(
        msrc: *mut c_void,
        ty: c_uchar,
        code: c_uchar,
        minsize: c_int,
        message: *const c_char,
    );
    fn socreate(slirp: *mut Slirp) -> *mut SocketUf;
    fn sofree(so: *mut SocketUf);
    fn udp_attach(so: *mut SocketUf) -> c_int;
    fn udp_detach(so: *mut SocketUf);
    fn m_free(m: *mut c_void);
    fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn strlen(s: *const c_char) -> usize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn sendto(
        fd: c_int,
        buf: *const c_void,
        n: usize,
        flags: c_int,
        addr: *const SockaddrIn,
        addr_len: u32,
    ) -> isize;
}

static ICMP_PING_MSG: [c_uchar; 82] = *b"This is a pseudo-PING packet used by Slirp to emulate ICMP ECHO-REQUEST packets.\n\0";

static mut LOOPBACK_ADDR: InAddr = InAddr { s_addr: 0 };

#[no_mangle]
pub unsafe extern "C" fn icmp_input__part2322(
    m: *mut c_void,
    icp: *mut IcmpUf,
    ip: *mut IpUf,
    slirp: *mut Slirp,
    hlen: c_int,
) -> c_int {
    match (*icp).icmp_type as c_int {
        8 => {
            (*icp).icmp_type = 0;
            (*ip).ip_len = (*ip).ip_len.wrapping_add(hlen as u16);
            if (*ip).ip_dst.s_addr == (*slirp).vhost_addr.s_addr {
                icmp_reflect(m);
            } else {
                let mut addr: SockaddrIn = core::mem::zeroed();
                let so: *mut SocketUf = socreate(slirp);
                if so.is_null() {
                    return 1;
                }
                if udp_attach(so) == -1 {
                    sofree(so);
                    m_free(m);
                    return 2;
                }
                (*so).so_m = m;
                (*so).so_faddr = InAddr {
                    s_addr: (*ip).ip_dst.s_addr,
                };
                (*so).so_fport = htons(7);
                (*so).so_laddr = InAddr {
                    s_addr: (*ip).ip_src.s_addr,
                };
                (*so).so_lport = htons(9);
                (*so).so_iptos = (*ip).ip_tos;
                (*so).so_type = 1; // IPPROTO_ICMP
                (*so).so_state = 0x004;

                addr.sin_family = 2;
                if ((*so).so_faddr.s_addr & (*slirp).vnetwork_mask.s_addr)
                    == (*slirp).vnetwork_addr.s_addr
                {
                    if (*so).so_faddr.s_addr == (*slirp).vnameserver_addr.s_addr {
                        let mut dns: InAddr = InAddr { s_addr: 0 };
                        if get_dns_addr(&mut dns) < 0 {
                            addr.sin_addr = InAddr {
                                s_addr: LOOPBACK_ADDR.s_addr,
                            };
                        } else {
                            addr.sin_addr = InAddr { s_addr: dns.s_addr };
                        }
                    } else {
                        addr.sin_addr = InAddr {
                            s_addr: LOOPBACK_ADDR.s_addr,
                        };
                    }
                } else {
                    addr.sin_addr = InAddr {
                        s_addr: (*so).so_faddr.s_addr,
                    };
                }
                addr.sin_port = (*so).so_fport;
                let msg_ptr: *const c_char = ICMP_PING_MSG.as_ptr() as *const c_char;
                let msg_len: usize = strlen(msg_ptr);
                if sendto(
                    (*so).s,
                    msg_ptr as *const c_void,
                    msg_len,
                    0,
                    &addr as *const SockaddrIn,
                    core::mem::size_of::<SockaddrIn>() as u32,
                ) == -1
                {
                    let err: c_int = *__errno_location();
                    icmp_error(m, 3, 0, 0, strerror(err));
                    udp_detach(so);
                }
            }
            0
        }
        3 | 11 | 12 | 4 | 13 | 17 | 5 => {
            m_free(m);
            0
        }
        _ => {
            m_free(m);
            0
        }
    }
}
}

// ── `end_error: return;` · slirp/ip_icmp.c:163-165 · chunk 2323 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     end_error:
//     
//       return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2323 {
// Ported piece of icmp_input: the end_error label body, which just returns.
#[no_mangle]
pub extern "C" fn icmp_input__part2323() {}
}

// ── `unsigned hlen, shlen, s_ip_len; register struct ip *ip; register struct icmp *icp; register str…` · slirp/ip_icmp.c:192-199 · chunk 2325 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     unsigned hlen, shlen, s_ip_len;
//       register struct ip *ip;
//       register struct icmp *icp;
//       register struct mbuf *m;
//     
//       ;
//       ;
//       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2325 {
// Ported piece of icmp_error() (slirp/ip_icmp.c): the local declarations
//   unsigned hlen, shlen, s_ip_len;
//   register struct ip *ip;
//   register struct icmp *icp;
//   register struct mbuf *m;
// (the three empty statements that followed came from macros that expand to
// nothing). Every one of these locals is assigned before it is read in the
// rest of the C function, so the initial values written here are the
// (unspecified) initial state of those declarations.
use core::ffi::{c_uint, c_void};

#[no_mangle]
pub extern "C" fn icmp_error__part2325(
    hlen: *mut c_uint,
    shlen: *mut c_uint,
    s_ip_len: *mut c_uint,
    ip: *mut *mut c_void,
    icp: *mut *mut c_void,
    m: *mut *mut c_void,
) {
    unsafe {
        if !hlen.is_null() {
            *hlen = 0;
        }
        if !shlen.is_null() {
            *shlen = 0;
        }
        if !s_ip_len.is_null() {
            *s_ip_len = 0;
        }
        if !ip.is_null() {
            *ip = core::ptr::null_mut();
        }
        if !icp.is_null() {
            *icp = core::ptr::null_mut();
        }
        if !m.is_null() {
            *m = core::ptr::null_mut();
        }
    }
}
}

// ── `if (type!=3 && type!=11)` · slirp/ip_icmp.c:201-201 · chunk 2326 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(type!=3 && type!=11) goto end_error;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2326 {
// Ported piece of icmp_error: reject ICMP error types other than 3 and 11.
// Returns nonzero when the caller must jump to end_error.

#[no_mangle]
pub extern "C" fn icmp_error__part2326(r#type: u8) -> core::ffi::c_int {
    let t = r#type as core::ffi::c_int;
    if t != 3 && t != 11 {
        1
    } else {
        0
    }
}
}

// ── `if (!msrc)` · slirp/ip_icmp.c:204-204 · chunk 2327 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(!msrc) goto end_error;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2327 {
// Extracted piece of icmp_error():  if (!msrc) goto end_error;
// Returns 1 when control must jump to end_error, 0 otherwise.

use core::ffi::c_int;

#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn icmp_error__part2327(msrc: *mut mbuf) -> c_int {
    if msrc.is_null() {
        1
    } else {
        0
    }
}
}

// ── `ip = ((struct ip *)(msrc)->m_hdr.mh_data);` · slirp/ip_icmp.c:205-205 · chunk 2328 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ip = ((struct ip *)(msrc)->m_hdr.mh_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2328 {
// Port of the `ip = ((struct ip *)(msrc)->m_hdr.mh_data);` statement from
// slirp's icmp_error(): fetch the IP header pointer out of the source mbuf.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

// Opaque C types (only ever used behind pointers here).
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct ip from ip.h (packed; ip_hl/ip_v is one 4+4 bit bitfield packed in a byte)
#[repr(C, packed)]
pub struct ip {
    pub ip_hl_ip_v: u8,
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

// struct m_hdr from mbuf.h
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

// struct mbuf from mbuf.h
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[no_mangle]
pub extern "C" fn icmp_error__part2328(msrc: *mut mbuf) -> *mut ip {
    unsafe {
        if msrc.is_null() {
            return core::ptr::null_mut();
        }
        let data = core::ptr::addr_of!((*msrc).m_hdr.mh_data).read();
        data as *mut ip
    }
}
}

// ── `if (ip->ip_off & 0x1fff)` · slirp/ip_icmp.c:213-213 · chunk 2329 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(ip->ip_off & 0x1fff) goto end_error;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2329 {
// Port of the fragment `if (ip->ip_off & 0x1fff) goto end_error;`
// from icmp_error() in slirp/ip_icmp.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirrors `struct ip` (packed) from the translation unit. The leading
// `u_int ip_hl:4, ip_v:4;` bit-field pair occupies a single packed byte.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ip {
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[no_mangle]
pub unsafe extern "C" fn icmp_error__part2329(ip: *const Ip) -> core::ffi::c_int {
    if ip.is_null() {
        return 0;
    }
    let off = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_off));
    if (off & 0x1fffu16) != 0 {
        1
    } else {
        0
    }
}
}

// ── `shlen=ip->ip_hl << 2; s_ip_len=ip->ip_len;` · slirp/ip_icmp.c:215-216 · chunk 2330 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     shlen=ip->ip_hl << 2;
//       s_ip_len=ip->ip_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2330 {
// Port of `shlen=ip->ip_hl << 2; s_ip_len=ip->ip_len;` from icmp_error().
use core::ffi::{c_uint, c_uchar, c_ushort};

// Mirror of `struct in_addr`.
#[repr(C, packed)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct ip` (packed). The C declaration starts with
//   u_int ip_hl:4, ip_v:4;
// which occupies exactly one byte (low nibble = ip_hl) in this packed struct.
#[repr(C, packed)]
pub struct Ip {
    pub ip_hl_ip_v: c_uchar,
    pub ip_tos: c_uchar,
    pub ip_len: c_ushort,
    pub ip_id: c_ushort,
    pub ip_off: c_ushort,
    pub ip_ttl: c_uchar,
    pub ip_p: c_uchar,
    pub ip_sum: c_ushort,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[no_mangle]
pub extern "C" fn icmp_error__part2330(
    ip: *const Ip,
    shlen: *mut c_uint,
    s_ip_len: *mut c_uint,
) {
    unsafe {
        let hl = ((*ip).ip_hl_ip_v & 0x0f) as c_uint;
        *shlen = hl << 2;
        *s_ip_len = (*ip).ip_len as c_uint;
    }
}
}

// ── `if (ip->ip_p == IPPROTO_ICMP)` · slirp/ip_icmp.c:217-224 · chunk 2331 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(ip->ip_p == 
//                     IPPROTO_ICMP
//                                 ) {
//         icp = (struct icmp *)((char *)ip + shlen);
//     
//     
//     
//     
//         if(icp->icmp_type>18 || icmp_flush[icp->icmp_type]) goto end_error;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2331 {
// Port of part of icmp_error: the ICMP-protocol sanity check on the
// embedded ICMP header (icmp_type range + icmp_flush table).
// Returns 1 when the caller must go to end_error, 0 otherwise.

use core::ffi::{c_int, c_uint};

// Mirror of `struct ip` (packed). Only ip_p is read here; the leading
// ip_hl:4/ip_v:4 bitfield pair occupies the first byte.
#[repr(C, packed)]
struct IpLayout {
    ip_hl_v: u8,
    ip_tos: u8,
    ip_len: u16,
    ip_id: u16,
    ip_off: u16,
    ip_ttl: u8,
    ip_p: u8,
    ip_sum: u16,
    ip_src: u32,
    ip_dst: u32,
}

const IPPROTO_ICMP: u8 = 1;

// The C file's `static const int icmp_flush[19]` has no linker symbol,
// so its contents live here.
static ICMP_FLUSH: [c_int; 19] = [
    0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
];

#[no_mangle]
pub extern "C" fn icmp_error__part2331(ip: *const IpLayout, shlen: c_uint) -> c_int {
    if ip.is_null() {
        return 0;
    }
    unsafe {
        let p = &*ip;
        if p.ip_p == IPPROTO_ICMP {
            let icp = (ip as *const u8).add(shlen as usize);
            let t = *icp;
            if t > 18 || ICMP_FLUSH[t as usize] != 0 {
                return 1;
            }
        }
    }
    0
}
}

// ── `m = m_get(msrc->slirp);` · slirp/ip_icmp.c:227-227 · chunk 2332 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m = m_get(msrc->slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2332 {
// Ported piece of slirp icmp_error(): `m = m_get(msrc->slirp);`
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
    fn m_get(slirp: *mut c_void) -> *mut mbuf;
}

#[no_mangle]
pub unsafe extern "C" fn icmp_error__part2332(msrc: *mut mbuf) -> *mut mbuf {
    let slirp = (*msrc).slirp;
    m_get(slirp)
}
}

// ── `if (!m)` · slirp/ip_icmp.c:228-230 · chunk 2333 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!m) {
//           goto end_error;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2333 {
// Port of the null check on the freshly allocated mbuf inside icmp_error().
// Returns nonzero when the allocation failed so the C caller can goto end_error.

use core::ffi::c_int;

#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn icmp_error__part2333(m: *mut mbuf) -> c_int {
    if m.is_null() {
        1
    } else {
        0
    }
}
}

// ── `{ … }` · slirp/ip_icmp.c:232-235 · chunk 2334 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     { int new_m_size;
//         new_m_size=sizeof(struct ip )+8 +msrc->m_hdr.mh_len+(576 -28);
//         if(new_m_size>m->m_hdr.mh_size) m_inc(m, new_m_size);
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2334 {
// Port of the mbuf-size adjustment block from icmp_error() (slirp/ip_icmp.c).
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct MbHdr {
    pub mh_next: *mut MbUf,
    pub mh_prev: *mut MbUf,
    pub mh_nextpkt: *mut MbUf,
    pub mh_prevpkt: *mut MbUf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union Mdat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct MbUf {
    pub m_hdr: MbHdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: Mdat,
}

extern "C" {
    fn m_inc(m: *mut MbUf, n: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn icmp_error__part2334(msrc: *mut MbUf, m: *mut MbUf) {
    let new_m_size: c_int = 20 + 8 + (*msrc).m_hdr.mh_len + (576 - 28);
    if new_m_size > (*m).m_hdr.mh_size {
        m_inc(m, new_m_size);
    }
}
}

// ── `memcpy(m->m_hdr.mh_data, msrc->m_hdr.mh_data, msrc->m_hdr.mh_len); m->m_hdr.mh_len = msrc->m_hd…` · slirp/ip_icmp.c:236-247 · chunk 2335 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(m->m_hdr.mh_data, msrc->m_hdr.mh_data, msrc->m_hdr.mh_len);
//       m->m_hdr.mh_len = msrc->m_hdr.mh_len;
//     
//     
//       ip = ((struct ip *)(m)->m_hdr.mh_data);
//       hlen= sizeof(struct ip );
//     
//     
//       m->m_hdr.mh_data += hlen;
//       m->m_hdr.mh_len -= hlen;
//     
//       icp = ((struct icmp *)(m)->m_hdr.mh_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2335 {
// Port of the memcpy/pointer-advance piece of icmp_error (slirp/ip_icmp.c).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// Opaque mirrors of the C types (only pointers to them are used).
#[repr(C)]
pub struct ip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct icmp {
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

// sizeof(struct ip) in the C translation unit (struct ip is packed, 20 bytes).
const HDR_LEN: c_uint = 20;

#[no_mangle]
pub extern "C" fn icmp_error__part2335(
    m: *mut mbuf,
    msrc: *mut mbuf,
    out_ip: *mut *mut ip,
    out_hlen: *mut c_uint,
    out_icp: *mut *mut icmp,
) {
    unsafe {
        let hlen: c_uint = HDR_LEN;

        let src_len: c_int = (*msrc).m_hdr.mh_len;
        memcpy(
            (*m).m_hdr.mh_data as *mut c_void,
            (*msrc).m_hdr.mh_data as *const c_void,
            src_len as usize,
        );
        (*m).m_hdr.mh_len = src_len;

        let ip_ptr: *mut ip = (*m).m_hdr.mh_data as *mut ip;
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_offset(hlen as isize);
        (*m).m_hdr.mh_len -= hlen as c_int;

        let icp_ptr: *mut icmp = (*m).m_hdr.mh_data as *mut icmp;

        *out_ip = ip_ptr;
        *out_hlen = hlen;
        *out_icp = icp_ptr;
    }
}
}

// ── `if (minsize) … else` · slirp/ip_icmp.c:249-251 · chunk 2336 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(minsize) s_ip_len=shlen+8;
//       else if(s_ip_len>(576 -28))
//         s_ip_len=(576 -28);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2336 {
#[no_mangle]
pub extern "C" fn icmp_error__part2336(
    minsize: core::ffi::c_int,
    shlen: core::ffi::c_uint,
    s_ip_len: *mut core::ffi::c_uint,
) {
    unsafe {
        if s_ip_len.is_null() {
            return;
        }
        if minsize != 0 {
            *s_ip_len = shlen.wrapping_add(8);
        } else if *s_ip_len > 548u32 {
            *s_ip_len = 548u32;
        }
    }
}
}

// ── `m->m_hdr.mh_len=8 +s_ip_len; icp->icmp_type = type; icp->icmp_code = code; icp->icmp_hun.ih_ids…` · slirp/ip_icmp.c:253-296 · chunk 2337 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_len=8 +s_ip_len;
//     
//     
//     
//       icp->icmp_type = type;
//       icp->icmp_code = code;
//       icp->icmp_hun.ih_idseq.icd_id = 0;
//       icp->icmp_hun.ih_idseq.icd_seq = 0;
//     
//       memcpy(&icp->icmp_dun.id_ip.idi_ip, msrc->m_hdr.mh_data, s_ip_len);
//       ((icp->icmp_dun.id_ip.idi_ip.ip_len) = htons((uint16_t)(icp->icmp_dun.id_ip.idi_ip.ip_len)));
//       ((icp->icmp_dun.id_ip.idi_ip.ip_id) = htons((uint16_t)(icp->icmp_dun.id_ip.idi_ip.ip_id)));
//       ((icp->icmp_dun.id_ip.idi_ip.ip_off) = htons((uint16_t)(icp->icmp_dun.id_ip.idi_ip.ip_off)));
//       icp->icmp_cksum = 0;
//       icp->icmp_cksum = cksum(m, m->m_hdr.mh_len);
//     
//       m->m_hdr.mh_data -= hlen;
//       m->m_hdr.mh_len += hlen;
//     
//     
//       ip->ip_hl = hlen >> 2;
//       ip->ip_len = m->m_hdr.mh_len;
//     
//       ip->ip_tos=((ip->ip_tos & 0x1E) | 0xC0);
//     
//       ip->ip_ttl = 255;
//       ip->ip_p = 
//                 IPPROTO_ICMP
//                             ;
//       ip->ip_dst = ip->ip_src;
//       ip->ip_src = m->slirp->vhost_addr;
//     
//       (void ) ip_output((struct socket *)
//                                         ((void *)0)
//                                             , m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2337 {
// Rust port of a slice of slirp's icmp_error() (slirp/ip_icmp.c lines 253-296).
// The code fills in the ICMP error header, checksums it, then fixes up the
// outer IP header and hands the mbuf to ip_output().

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
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
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
}

// Only the leading members of struct Slirp are modelled; vhost_addr is the
// third member of the real definition.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
}

// struct ip is packed in C; the field offsets below are identical to the C
// layout.  Accesses are done with read_unaligned/write_unaligned because the
// packets themselves may be unaligned.
#[repr(C)]
pub struct Ip {
    pub ip_hl_v: u8, // ip_hl:4 (low nibble), ip_v:4 (high nibble)
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
pub struct IhIdseq {
    pub icd_id: u16,
    pub icd_seq: u16,
}

#[repr(C)]
pub struct IcmpHun {
    pub ih_idseq: IhIdseq,
}

#[repr(C)]
pub struct IdIp {
    pub idi_ip: Ip,
}

#[repr(C)]
pub struct IcmpDun {
    pub id_ip: IdIp,
}

#[repr(C)]
pub struct Icmp {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub icmp_cksum: u16,
    pub icmp_hun: IcmpHun,
    pub icmp_dun: IcmpDun,
}

const IPPROTO_ICMP: u8 = 1;

extern "C" {
    fn htons(hostshort: u16) -> u16;
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
    fn ip_output(so: *mut c_void, m: *mut Mbuf) -> c_int;
}

#[no_mangle]
pub extern "C" fn icmp_error__part2337(
    m: *mut Mbuf,
    ip: *mut Ip,
    icp: *mut Icmp,
    msrc: *mut Mbuf,
    hlen: c_uint,
    s_ip_len: c_uint,
    r#type: u8,
    code: u8,
) {
    unsafe {
        // m->m_hdr.mh_len = 8 + s_ip_len;
        let new_len = 8u32.wrapping_add(s_ip_len) as c_int;
        (*m).m_hdr.mh_len = new_len;

        // ICMP header
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*icp).icmp_type), r#type);
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*icp).icmp_code), code);
        core::ptr::write_unaligned(
            core::ptr::addr_of_mut!((*icp).icmp_hun.ih_idseq.icd_id),
            0u16,
        );
        core::ptr::write_unaligned(
            core::ptr::addr_of_mut!((*icp).icmp_hun.ih_idseq.icd_seq),
            0u16,
        );

        // memcpy(&icp->icmp_dun.id_ip.idi_ip, msrc->m_hdr.mh_data, s_ip_len);
        let dst = core::ptr::addr_of_mut!((*icp).icmp_dun.id_ip.idi_ip) as *mut u8;
        let src = (*msrc).m_hdr.mh_data as *const u8;
        core::ptr::copy_nonoverlapping(src, dst, s_ip_len as usize);

        // Byte-swap the copied inner IP header fields to network order.
        let inner = core::ptr::addr_of_mut!((*icp).icmp_dun.id_ip.idi_ip);
        let v = core::ptr::read_unaligned(core::ptr::addr_of!((*inner).ip_len));
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*inner).ip_len), htons(v));
        let v = core::ptr::read_unaligned(core::ptr::addr_of!((*inner).ip_id));
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*inner).ip_id), htons(v));
        let v = core::ptr::read_unaligned(core::ptr::addr_of!((*inner).ip_off));
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*inner).ip_off), htons(v));

        // icp->icmp_cksum = 0; icp->icmp_cksum = cksum(m, m->m_hdr.mh_len);
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*icp).icmp_cksum), 0u16);
        let sum = cksum(m, (*m).m_hdr.mh_len) as u16;
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*icp).icmp_cksum), sum);

        // Back up over the IP header.
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_sub(hlen as usize);
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(hlen as c_int);

        // Outer IP header.
        let hl = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_hl_v));
        core::ptr::write_unaligned(
            core::ptr::addr_of_mut!((*ip).ip_hl_v),
            (hl & 0xF0) | (((hlen >> 2) as u8) & 0x0F),
        );
        core::ptr::write_unaligned(
            core::ptr::addr_of_mut!((*ip).ip_len),
            (*m).m_hdr.mh_len as u16,
        );
        let tos = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_tos));
        core::ptr::write_unaligned(
            core::ptr::addr_of_mut!((*ip).ip_tos),
            (tos & 0x1E) | 0xC0,
        );
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_ttl), 255u8);
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_p), IPPROTO_ICMP);

        let s = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_src.s_addr));
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_dst.s_addr), s);
        let vhost = (*(*m).slirp).vhost_addr.s_addr;
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_src.s_addr), vhost);

        let _ = ip_output(core::ptr::null_mut(), m);
    }
}
}

// ── `end_error: return;` · slirp/ip_icmp.c:298-299 · chunk 2338 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     end_error:
//       return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2338 {
// Ported piece of icmp_error: the end_error label body (plain function exit).
// The label itself and the `return` stay in C so the surrounding gotos
// keep working.

#[no_mangle]
pub extern "C" fn icmp_error__part2338() {
    // nothing to do: the original body was just `return;`
}
}

// ── `register struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data); int hlen = ip->ip_hl << 2; int optl…` · slirp/ip_icmp.c:309-326 · chunk 2340 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data);
//       int hlen = ip->ip_hl << 2;
//       int optlen = hlen - sizeof(struct ip );
//       register struct icmp *icp;
//     
//     
//     
//     
//     
//       m->m_hdr.mh_data += hlen;
//       m->m_hdr.mh_len -= hlen;
//       icp = ((struct icmp *)(m)->m_hdr.mh_data);
//     
//       icp->icmp_cksum = 0;
//       icp->icmp_cksum = cksum(m, ip->ip_len - hlen);
//     
//       m->m_hdr.mh_data -= hlen;
//       m->m_hdr.mh_len += hlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2340 {
// Port of the checksum-fixup part of icmp_reflect().
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirrors `struct ip` (packed, host byte order fields as in C).
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
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

// Mirrors the head of `struct icmp`.
#[repr(C)]
pub struct Icmp {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub icmp_cksum: u16,
}

// Mirrors `struct m_hdr` (prefix of `struct mbuf`).
#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: core::ffi::c_int,
    pub mh_size: core::ffi::c_int,
    pub mh_so: *mut core::ffi::c_void,
    pub mh_data: *mut core::ffi::c_char,
    pub mh_len: core::ffi::c_int,
}

// Mirrors `struct mbuf` (only the header is touched here).
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
}

extern "C" {
    fn cksum(m: *mut Mbuf, len: core::ffi::c_int) -> core::ffi::c_int;
}

#[no_mangle]
pub extern "C" fn icmp_reflect__part2340(
    m: *mut Mbuf,
    pip: *mut *mut Ip,
    phlen: *mut core::ffi::c_int,
    poptlen: *mut core::ffi::c_int,
) {
    unsafe {
        let ip = (*m).m_hdr.mh_data as *mut Ip;
        let hl = ((*ip).ip_hl_v & 0x0f) as core::ffi::c_int;
        let hlen: core::ffi::c_int = hl << 2;
        let optlen: core::ffi::c_int = hlen - core::mem::size_of::<Ip>() as core::ffi::c_int;

        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(hlen as isize);
        (*m).m_hdr.mh_len -= hlen;
        let icp = (*m).m_hdr.mh_data as *mut Icmp;

        (*icp).icmp_cksum = 0;
        let cklen: core::ffi::c_int = (*ip).ip_len as core::ffi::c_int - hlen;
        (*icp).icmp_cksum = cksum(m, cklen) as u16;

        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(hlen as isize));
        (*m).m_hdr.mh_len += hlen;

        *pip = ip;
        *phlen = hlen;
        *poptlen = optlen;
    }
}
}

// ── `if (optlen > 0)` · slirp/ip_icmp.c:329-340 · chunk 2341 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (optlen > 0) {
//     
//     
//     
//     
//         memmove((caddr_t)(ip + 1), (caddr_t)ip + hlen,
//          (unsigned )(m->m_hdr.mh_len - hlen));
//         hlen -= optlen;
//         ip->ip_hl = hlen >> 2;
//         ip->ip_len -= optlen;
//         m->m_hdr.mh_len -= optlen;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2341 {
// Rust rewrite of the option-stripping block of icmp_reflect() (slirp/ip_icmp.c).

use core::ffi::{c_int, c_uint, c_void};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip: first byte holds the ip_hl:4 / ip_v:4 bitfields (ip_hl in the low nibble).
#[repr(C, packed)]
pub struct Ip {
    pub ip_hl: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut c_void,
    pub mh_prev: *mut c_void,
    pub mh_nextpkt: *mut c_void,
    pub mh_prevpkt: *mut c_void,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut core::ffi::c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void,
    pub m_dat: *mut core::ffi::c_char,
}

extern "C" {
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn icmp_reflect__part2341(
    ip: *mut Ip,
    m: *mut Mbuf,
    hlen: *mut c_int,
    optlen: c_int,
) {
    if optlen <= 0 {
        return;
    }

    let h0: c_int = unsafe { *hlen };
    let mh_len0: c_int = unsafe { (*m).m_hdr.mh_len };

    // memmove((caddr_t)(ip + 1), (caddr_t)ip + hlen, (unsigned)(m->m_hdr.mh_len - hlen));
    let n: c_uint = mh_len0.wrapping_sub(h0) as c_uint;
    let base: *mut u8 = ip as *mut u8;
    let dst: *mut c_void = unsafe { base.add(core::mem::size_of::<Ip>()) } as *mut c_void;
    let src: *const c_void = unsafe { base.add(h0 as usize) } as *const c_void;
    unsafe {
        memmove(dst, src, n as usize);
    }

    // hlen -= optlen;
    let new_hlen: c_int = h0.wrapping_sub(optlen);
    unsafe {
        *hlen = new_hlen;
    }

    // ip->ip_hl = hlen >> 2;  (4-bit unsigned bitfield: keep ip_v nibble, truncate mod 16)
    let hl_p = unsafe { core::ptr::addr_of_mut!((*ip).ip_hl) };
    let hl_old = unsafe { core::ptr::read_unaligned(hl_p) };
    let hl_new = (hl_old & 0xf0u8) | (((new_hlen >> 2) as u8) & 0x0f);
    unsafe {
        core::ptr::write_unaligned(hl_p, hl_new);
    }

    // ip->ip_len -= optlen;  (uint16_t, wraps mod 65536)
    let len_p = unsafe { core::ptr::addr_of_mut!((*ip).ip_len) };
    let len_old = unsafe { core::ptr::read_unaligned(len_p) };
    unsafe {
        core::ptr::write_unaligned(len_p, len_old.wrapping_sub(optlen as u16));
    }

    // m->m_hdr.mh_len -= optlen;
    let mh_p = unsafe { core::ptr::addr_of_mut!((*m).m_hdr.mh_len) };
    let mh_old = unsafe { core::ptr::read(mh_p) };
    unsafe {
        core::ptr::write(mh_p, mh_old.wrapping_sub(optlen));
    }
}
}

// ── `ip->ip_ttl = 255;` · slirp/ip_icmp.c:342-342 · chunk 2342 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     ip->ip_ttl = 255;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2342 {
// Rewrite of the `ip->ip_ttl = 255;` statement from icmp_reflect().
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

// Mirror of `struct ip` (packed). The 4-bit bitfields ip_hl/ip_v share one
// `unsigned int` unit (4 bytes) at offset 0; layout is otherwise byte-exact,
// so ip_ttl sits at offset 11.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ip {
    pub ip_hl_ip_v: [u8; 4],
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
pub unsafe extern "C" fn icmp_reflect__part2342(ip: *mut ip) {
    if ip.is_null() {
        return;
    }
    let ttl = core::ptr::addr_of_mut!((*ip).ip_ttl);
    core::ptr::write_unaligned(ttl, 255u8);
}
}

// ── `{ … }` · slirp/ip_icmp.c:343-348 · chunk 2343 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//         struct in_addr icmp_dst;
//         icmp_dst = ip->ip_dst;
//         ip->ip_dst = ip->ip_src;
//         ip->ip_src = icmp_dst;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2343 {
// Port of the swap of ip_src / ip_dst inside icmp_reflect().

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of "struct ip" (packed, ip_hl:4 / ip_v:4 share the first byte).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ip {
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[no_mangle]
pub unsafe extern "C" fn icmp_reflect__part2343(ip: *mut Ip) {
    if ip.is_null() {
        return;
    }
    let icmp_dst: InAddr = (*ip).ip_dst;
    (*ip).ip_dst = (*ip).ip_src;
    (*ip).ip_src = icmp_dst;
}
}

// ── `(void ) ip_output((struct socket *) ((void *)0) , m);` · slirp/ip_icmp.c:350-350 · chunk 2344 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     (void ) ip_output((struct socket *)
//                                         ((void *)0)
//                                             , m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2344 {
// Port of the tail call `(void) ip_output(NULL, m);` from icmp_reflect().

use core::ffi::{c_int, c_void};

// Opaque mirrors of the C types; only pointers to them cross the boundary.
#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

extern "C" {
    fn ip_output(so: *mut socket, m: *mut mbuf) -> c_int;
}

#[no_mangle]
pub extern "C" fn icmp_reflect__part2344(m: *mut mbuf) {
    let _ = c_void_size(); // keep the import meaningful without side effects
    unsafe {
        ip_output(core::ptr::null_mut::<socket>(), m);
    }
}

#[inline]
fn c_void_size() -> usize {
    core::mem::size_of::<c_void>()
}
}

