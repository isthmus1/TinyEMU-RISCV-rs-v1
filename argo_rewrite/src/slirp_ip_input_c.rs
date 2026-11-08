//! From `slirp/ip_input.c`.

// ── `slirp->ipq.ip_link.next = slirp->ipq.ip_link.prev = &slirp->ipq.ip_link; udp_init(slirp); tcp_i…` · slirp/ip_input.c:61-63 · chunk 2347 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->ipq.ip_link.next = slirp->ipq.ip_link.prev = &slirp->ipq.ip_link;
//         udp_init(slirp);
//         tcp_init(slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2347 {
// Port of the initialization prologue of ip_init(): reinitialize the IP
// fragment queue list head and initialize the UDP and TCP subsystems.

use core::ffi::c_void;

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

extern "C" {
    fn udp_init(slirp: *mut c_void);
    fn tcp_init(slirp: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ip_init__part2347(slirp: *mut c_void, ip_link: *mut Qlink) {
    if !ip_link.is_null() {
        (*ip_link).next = ip_link as *mut c_void;
        (*ip_link).prev = ip_link as *mut c_void;
    }
    unsafe {
        udp_init(slirp);
        tcp_init(slirp);
    }
}
}

// ── `Slirp *slirp = m->slirp; register struct ip *ip; int hlen; ; ; ;` · slirp/ip_input.c:73-79 · chunk 2349 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = m->slirp;
//      register struct ip *ip;
//      int hlen;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2349 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Opaque mirror of `struct Slirp` (only ever handled through a pointer).
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

// Mirror of `struct m_hdr` (layout only).
#[repr(C)]
pub struct m_hdr {
    mh_next: *mut mbuf,
    mh_prev: *mut mbuf,
    mh_nextpkt: *mut mbuf,
    mh_prevpkt: *mut mbuf,
    mh_flags: c_int,
    mh_size: c_int,
    mh_so: *mut c_void,
    mh_data: *mut c_char,
    mh_len: c_int,
}

// Mirror of `struct mbuf` (layout only); we read the `slirp` field.
#[repr(C)]
pub struct mbuf {
    m_hdr: m_hdr,
    slirp: *mut Slirp,
}

#[no_mangle]
pub extern "C" fn ip_input__part2349(m: *mut mbuf) -> *mut Slirp {
    unsafe { (*m).slirp }
}
}

// ── `if (m->m_hdr.mh_len < sizeof (struct ip))` · slirp/ip_input.c:81-83 · chunk 2350 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_len < sizeof (struct ip)) {
//       return;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2350 {
// Port of the leading length check of ip_input():
//   if (m->m_hdr.mh_len < sizeof(struct ip)) return;
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct MBuf {
    m_hdr: MHdr,
}

#[repr(C)]
pub struct MHdr {
    mh_next: *mut MBuf,
    mh_prev: *mut MBuf,
    mh_nextpkt: *mut MBuf,
    mh_prevpkt: *mut MBuf,
    mh_flags: c_int,
    mh_size: c_int,
    mh_so: *mut c_void,
    mh_data: *mut c_char,
    mh_len: c_int,
}

// sizeof(struct ip) for the packed IPv4 header on this target.
const SIZEOF_STRUCT_IP: c_int = 20;

/// Returns nonzero when the caller must return early (packet too short).
#[no_mangle]
pub extern "C" fn ip_input__part2350(m: *mut MBuf) -> c_int {
    let len = unsafe { (*m).m_hdr.mh_len };
    (len < SIZEOF_STRUCT_IP) as c_int
}
}

// ── `ip = ((struct ip *)(m)->m_hdr.mh_data);` · slirp/ip_input.c:85-85 · chunk 2351 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ip = ((struct ip *)(m)->m_hdr.mh_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2351 {
// Port of: ip = ((struct ip *)(m)->m_hdr.mh_data);
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

// Opaque mirror of `struct ip`: only pointers to it are ever handled here.
#[repr(C)]
pub struct ip {
    _opaque: [u8; 0],
}

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
pub struct mbuf {
    pub m_hdr: m_hdr,
}

#[no_mangle]
pub extern "C" fn ip_input__part2351(m: *mut mbuf) -> *mut ip {
    unsafe { (*m).m_hdr.mh_data as *mut ip }
}
}

// ── `if (ip->ip_v != 4)` · slirp/ip_input.c:87-89 · chunk 2352 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip->ip_v != 4) {
//       goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2352 {
// Port of the IP version check from ip_input() in slirp/ip_input.c.
//
// The C struct ip is packed and begins with the bitfields
//     u_int ip_hl:4, ip_v:4;
// so the first byte holds ip_hl in its low nibble and ip_v in its high
// nibble.  Only that first byte (plus the rest of the layout, mirrored for
// completeness) is described here.

use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Ip {
    // ip_hl:4, ip_v:4 packed into one byte
    pub ip_hl_ip_v: u8,
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

/// Returns 1 when the caller must take the `bad:` path (ip_v != 4),
/// 0 otherwise.
#[no_mangle]
pub extern "C" fn ip_input__part2352(ip: *mut Ip) -> c_int {
    unsafe {
        if ip.is_null() {
            return 0;
        }
        let ip_v = ((*ip).ip_hl_ip_v >> 4) & 0x0f;
        if ip_v != 4 {
            1
        } else {
            0
        }
    }
}
}

// ── `hlen = ip->ip_hl << 2;` · slirp/ip_input.c:91-91 · chunk 2353 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     hlen = ip->ip_hl << 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2353 {
// Port of a single statement of slirp's ip_input():
//     hlen = ip->ip_hl << 2;
// The packed `struct ip` is mirrored below; its first byte holds the
// `u_int ip_hl:4, ip_v:4` bitfields (little-endian: ip_hl is bits 0..3).

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct ip {
    // bitfields `u_int ip_hl:4, ip_v:4` packed into byte 0
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
pub extern "C" fn ip_input__part2353(ip: *const ip) -> core::ffi::c_int {
    // ip->ip_hl is an unsigned 4-bit bitfield
    let ip_hl: u32 = unsafe { ((*ip).ip_hl_v & 0x0f) as u32 };
    (ip_hl << 2) as core::ffi::c_int
}
}

// ── `if (hlen<sizeof(struct ip ) || hlen>m->m_hdr.mh_len)` · slirp/ip_input.c:92-94 · chunk 2354 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (hlen<sizeof(struct ip ) || hlen>m->m_hdr.mh_len) {
//        goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2354 {
// Ported fragment of slirp ip_input(): header-length sanity check.
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
    pub slirp: *mut c_void,
}

// sizeof(struct ip) == 20 (packed IPv4 header)
const SIZEOF_IP: usize = 20;

#[no_mangle]
pub extern "C" fn ip_input__part2354(m: *mut Mbuf, hlen: c_int) -> c_int {
    // returns 1 when the caller must `goto bad`, 0 to continue
    let mh_len = unsafe { (*m).m_hdr.mh_len };
    // C compares int against size_t: both sides are converted to size_t.
    if (hlen as usize) < SIZEOF_IP || (hlen as usize) > (mh_len as usize) {
        1
    } else {
        0
    }
}
}

// ── `if (cksum(m,hlen))` · slirp/ip_input.c:100-102 · chunk 2355 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(cksum(m,hlen)) {
//        goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2355 {
use core::ffi::{c_int, c_void};

extern "C" {
    // int cksum(struct mbuf *m, int len);
    fn cksum(m: *mut c_void, len: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn ip_input__part2355(m: *mut c_void, hlen: c_int) -> c_int {
    unsafe { cksum(m, hlen) }
}
}

// ── `((ip->ip_len) = ntohs((uint16_t)(ip->ip_len)));` · slirp/ip_input.c:107-107 · chunk 2356 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((ip->ip_len) = ntohs((uint16_t)(ip->ip_len)));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2356 {
// Port of the statement: ip->ip_len = ntohs((uint16_t)ip->ip_len);
// in ip_input() (slirp/ip_input.c).

#[repr(C, packed)]
pub struct Ip {
    // bitfields: u_int ip_hl:4, ip_v:4;
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

#[no_mangle]
pub unsafe extern "C" fn ip_input__part2356(ip: *mut Ip) {
    if ip.is_null() {
        return;
    }
    // ntohs(): network (big endian) 16-bit value -> host order.
    let p = core::ptr::addr_of_mut!((*ip).ip_len);
    let cur: u16 = core::ptr::read_unaligned(p);
    let host: u16 = u16::from_be(cur);
    core::ptr::write_unaligned(p, host);
}
}

// ── `if (ip->ip_len < hlen)` · slirp/ip_input.c:108-110 · chunk 2357 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip->ip_len < hlen) {
//       goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2357 {
// Port of the ip_len/hlen sanity check from slirp's ip_input().
// In C both operands are promoted to int: ip->ip_len is uint16_t (so it
// promotes to int) and hlen is int, hence the signed comparison below.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ip_input__part2357(ip_len: u16, hlen: c_int) -> c_int {
    if (ip_len as c_int) < hlen {
        1
    } else {
        0
    }
}
}

// ── `((ip->ip_id) = ntohs((uint16_t)(ip->ip_id))); ((ip->ip_off) = ntohs((uint16_t)(ip->ip_off)));` · slirp/ip_input.c:111-112 · chunk 2358 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((ip->ip_id) = ntohs((uint16_t)(ip->ip_id)));
//      ((ip->ip_off) = ntohs((uint16_t)(ip->ip_off)));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2358 {
// Ported piece of slirp ip_input(): network-to-host conversion of ip_id/ip_off.
#![allow(non_snake_case)]

extern "C" {
    fn ntohs(__netshort: u16) -> u16;
}

// Mirror of the (packed) C `struct ip` from the preprocessed translation unit.
#[repr(C, packed)]
pub struct Ip {
    // u_int ip_hl:4, ip_v:4; packed into the first byte
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

#[no_mangle]
pub extern "C" fn ip_input__part2358(ip: *mut Ip) {
    if ip.is_null() {
        return;
    }
    unsafe {
        let id = (*ip).ip_id;
        (*ip).ip_id = ntohs(id);
        let off = (*ip).ip_off;
        (*ip).ip_off = ntohs(off);
    }
}
}

// ── `if (m->m_hdr.mh_len < ip->ip_len)` · slirp/ip_input.c:120-122 · chunk 2359 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_len < ip->ip_len) {
//       goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2359 {
// Mirror of the C structs accessed by ip_input__part2359.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub struct Slirp_opaque {
    _private: [u8; 0],
}

pub struct socket_opaque {
    _private: [u8; 0],
}

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
    pub mh_so: *mut socket_opaque,
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
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp_opaque,
    pub M_dat: M_dat,
}

// struct ip is packed; the two 4-bit bitfields (ip_hl, ip_v) occupy one byte.
#[repr(C, packed)]
pub struct ip {
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

/// Returns nonzero when the packet must be dropped (C `goto bad;`).
#[no_mangle]
pub extern "C" fn ip_input__part2359(m: *mut mbuf, ip: *mut ip) -> c_int {
    if m.is_null() || ip.is_null() {
        return 0;
    }
    unsafe {
        let mh_len: c_int = (*m).m_hdr.mh_len;
        let ip_len: c_int = (*ip).ip_len as c_int;
        if mh_len < ip_len {
            1
        } else {
            0
        }
    }
}
}

// ── `if (slirp->restricted)` · slirp/ip_input.c:124-143 · chunk 2360 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->restricted) {
//             if ((ip->ip_dst.s_addr & slirp->vnetwork_mask.s_addr) ==
//                 slirp->vnetwork_addr.s_addr) {
//                 if (ip->ip_dst.s_addr == 0xffffffff && ip->ip_p != 
//                                                                   IPPROTO_UDP
//                                                                              )
//                     goto bad;
//             } else {
//                 uint32_t inv_mask = ~slirp->vnetwork_mask.s_addr;
//                 struct ex_list *ex_ptr;
//     
//                 if ((ip->ip_dst.s_addr & inv_mask) == inv_mask) {
//                     goto bad;
//                 }
//                 for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next)
//                     if (ex_ptr->ex_addr.s_addr == ip->ip_dst.s_addr)
//                         break;
//     
//                 if (!ex_ptr)
//                     goto bad;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2360 {
// Port of the "restricted mode" destination check from slirp ip_input().
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct ip` (packed 20-byte IPv4 header).  All fields of the C
// struct sit at naturally aligned offsets, so plain repr(C) reproduces the
// layout; the pointer we receive may be unaligned, so we read it unaligned.
#[repr(C)]
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

// Mirror of `struct ex_list`.
#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

const IPPROTO_UDP: u8 = 17;

/// Returns 0 when the packet may continue, 1 when the caller must `goto bad`.
#[no_mangle]
pub extern "C" fn ip_input__part2360(
    ip: *const Ip,
    restricted: c_int,
    vnetwork_mask: u32,
    vnetwork_addr: u32,
    exec_list: *mut ExList,
) -> c_int {
    if restricted == 0 {
        return 0;
    }

    let ipv: Ip = unsafe { core::ptr::read_unaligned(ip) };
    let dst: u32 = ipv.ip_dst.s_addr;

    if (dst & vnetwork_mask) == vnetwork_addr {
        if dst == 0xffff_ffff && ipv.ip_p != IPPROTO_UDP {
            return 1;
        }
    } else {
        let inv_mask: u32 = !vnetwork_mask;

        if (dst & inv_mask) == inv_mask {
            return 1;
        }

        let mut ex_ptr: *mut ExList = exec_list;
        while !ex_ptr.is_null() {
            if unsafe { (*ex_ptr).ex_addr.s_addr } == dst {
                break;
            }
            ex_ptr = unsafe { (*ex_ptr).ex_next };
        }

        if ex_ptr.is_null() {
            return 1;
        }
    }

    0
}
}

// ── `if (m->m_hdr.mh_len > ip->ip_len)` · slirp/ip_input.c:146-147 · chunk 2361 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_len > ip->ip_len)
//         m_adj(m, ip->ip_len - m->m_hdr.mh_len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2361 {
// Port of a piece of slirp's ip_input(): trim the mbuf down to the IP total length.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int};

// Opaque stand-in for the C `Slirp` type (only ever used through a pointer here).
pub struct Slirp {
    _private: [u8; 0],
}

// Opaque stand-in for the C `struct socket` (pointer only).
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

// union M_dat { char m_dat_[1]; char *m_ext_; }
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
pub struct in_addr {
    pub s_addr: u32,
}

// struct ip { u_int ip_hl:4, ip_v:4; uint8_t ip_tos; uint16_t ip_len; ... } __attribute__((packed));
// The two 4-bit fields pack into the first byte.
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

extern "C" {
    fn m_adj(m: *mut mbuf, len: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn ip_input__part2361(m: *mut mbuf, ip: *mut ip) {
    let mh_len: c_int = (*m).m_hdr.mh_len;
    let ip_len: c_int = (*ip).ip_len as c_int;

    if mh_len > ip_len {
        m_adj(m, ip_len.wrapping_sub(mh_len));
    }
}
}

// ── `if (ip->ip_ttl==0)` · slirp/ip_input.c:150-153 · chunk 2362 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(ip->ip_ttl==0) {
//        icmp_error(m, 11,0, 0,"ttl");
//        goto bad;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2362 {
// Port of the TTL-zero check from slirp's ip_input().
// Returns 1 when the TTL is zero (the C caller must then `goto bad`),
// 0 otherwise.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirrors `struct ip` (packed): the 4/4 bitfield pair `ip_hl:4, ip_v:4`
// occupies the first byte, then the fields follow with no padding.
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

// Opaque from Rust's point of view; only ever passed through.
#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
}

extern "C" {
    fn icmp_error(
        msrc: *mut Mbuf,
        type_: u8,
        code: u8,
        minsize: c_int,
        message: *const c_char,
    );
}

#[no_mangle]
pub extern "C" fn ip_input__part2362(m: *mut Mbuf, ip: *mut Ip) -> c_int {
    unsafe {
        if (*ip).ip_ttl == 0 {
            icmp_error(m, 11u8, 0u8, 0 as c_int, b"ttl\0".as_ptr() as *const c_char);
            return 1;
        }
    }
    0
}
}

// ── `if (ip->ip_off &~ 0x4000) … else` · slirp/ip_input.c:164-211 · chunk 2363 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip->ip_off &~ 0x4000) {
//        register struct ipq *fp;
//           struct qlink *l;
//     
//     
//     
//     
//       for (l = slirp->ipq.ip_link.next; l != &slirp->ipq.ip_link;
//            l = l->next) {
//                 fp = ({ const typeof( ((struct ipq *)0)->ip_link ) *__mptr = (l); (struct ipq *)( (char *)__mptr - 
//                     __builtin_offsetof (
//                     struct ipq
//                     , 
//                     ip_link
//                     ) 
//                     );});
//                 if (ip->ip_id == fp->ipq_id &&
//                         ip->ip_src.s_addr == fp->ipq_src.s_addr &&
//                         ip->ip_dst.s_addr == fp->ipq_dst.s_addr &&
//                         ip->ip_p == fp->ipq_p)
//           goto found;
//             }
//             fp = 
//                 ((void *)0)
//                     ;
//      found:
//     
//     
//     
//     
//     
//     
//       ip->ip_len -= hlen;
//       if (ip->ip_off & 0x2000)
//         ip->ip_tos |= 1;
//       else
//         ip->ip_tos &= ~1;
//     
//       ip->ip_off <<= 3;
//     
//     
//     
//     
//     
//     
//       if (ip->ip_tos & 1 || ip->ip_off) {
//        ip = ip_reass(slirp, ip, fp);
//                             if (ip == 
//                                      ((void *)0)
//                                          )
//         return;
//        m = dtom(slirp, ip);
//       } else
//        if (fp)
//              ip_freef(slirp, fp);
//     
//      } else
//       ip->ip_len -= hlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2363 {
// Port of the IP-fragment-queue handling part of slirp's ip_input().
// The static C helpers ip_reass() / ip_freef() / dtom() have no linkable
// symbols, so the C caller keeps driving those calls; this function only
// performs the queue lookup and header fixups and reports which call (if
// any) the C side must make.

use core::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip is __attribute__((packed)); the field offsets are identical in a
// naturally-aligned repr(C) layout, since every field is naturally aligned.
// The ip_hl:4 / ip_v:4 bitfield pair occupies the first byte.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ip {
    pub hl_v: u8,
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
pub struct Qlink {
    pub next: *mut Qlink,
    pub prev: *mut Qlink,
}

#[repr(C)]
pub struct IpQ {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

// Byte offset of ip_link inside struct ipq (frag_link, a struct qlink,
// precedes it).
const IP_LINK_OFFSET: usize = core::mem::size_of::<Qlink>();

// Return codes for the C caller:
//   0 - nothing further to do
//   1 - caller must do: ip = ip_reass(slirp, ip, fp); if (ip == NULL) return;
//                       m = dtom(slirp, ip);
//   2 - caller must do: if (fp) ip_freef(slirp, fp);
#[no_mangle]
pub unsafe extern "C" fn ip_input__part2363(
    ip: *mut Ip,
    hlen: c_int,
    head: *mut Qlink,
    fp_out: *mut *mut IpQ,
) -> c_int {
    if (*(ip)).ip_off & !0x4000u16 != 0 {
        let mut fp: *mut IpQ = core::ptr::null_mut();

        let ip_id = (*(ip)).ip_id;
        let src = (*(ip)).ip_src.s_addr;
        let dst = (*(ip)).ip_dst.s_addr;
        let proto = (*(ip)).ip_p;

        let mut l: *mut Qlink = (*(head)).next;
        while l != head {
            let cand: *mut IpQ = (l as *mut u8).sub(IP_LINK_OFFSET) as *mut IpQ;
            if (*(cand)).ipq_id == ip_id
                && (*(cand)).ipq_src.s_addr == src
                && (*(cand)).ipq_dst.s_addr == dst
                && (*(cand)).ipq_p == proto
            {
                fp = cand;
                break;
            }
            l = (*(l)).next;
        }

        // found:
        let hlen16 = hlen as u16;
        (*(ip)).ip_len = (*(ip)).ip_len.wrapping_sub(hlen16);
        if (*(ip)).ip_off & 0x2000 != 0 {
            (*(ip)).ip_tos |= 1;
        } else {
            (*(ip)).ip_tos &= !1u8;
        }
        (*(ip)).ip_off = (*(ip)).ip_off << 3;

        if (*(ip)).ip_tos & 1 != 0 || (*(ip)).ip_off != 0 {
            if !fp_out.is_null() {
                *(fp_out) = fp;
            }
            1
        } else if !fp.is_null() {
            if !fp_out.is_null() {
                *(fp_out) = fp;
            }
            2
        } else {
            0
        }
    } else {
        let hlen16 = hlen as u16;
        (*(ip)).ip_len = (*(ip)).ip_len.wrapping_sub(hlen16);
        0
    }
}
}

// ── `switch (ip->ip_p)` · slirp/ip_input.c:216-228 · chunk 2364 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (ip->ip_p) {
//       case 
//           IPPROTO_TCP
//                      :
//       tcp_input(m, hlen, (struct socket *)
//                                          ((void *)0)
//                                              );
//       break;
//       case 
//           IPPROTO_UDP
//                      :
//       udp_input(m, hlen);
//       break;
//       case 
//           IPPROTO_ICMP
//                       :
//       icmp_input(m, hlen);
//       break;
//       default:
//       m_free(m);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2364 {
// Port of the protocol dispatch switch at the end of ip_input() (slirp/ip_input.c).
use core::ffi::{c_int, c_void};

extern "C" {
    // void tcp_input(struct mbuf *, int, struct socket *);
    fn tcp_input(m: *mut c_void, hlen: c_int, so: *mut c_void);
    // void udp_input(register struct mbuf *, int);
    fn udp_input(m: *mut c_void, hlen: c_int);
    // void icmp_input(struct mbuf *, int);
    fn icmp_input(m: *mut c_void, hlen: c_int);
    // void m_free(struct mbuf *);
    fn m_free(m: *mut c_void);
}

// IPPROTO_ICMP = 1, IPPROTO_TCP = 6, IPPROTO_UDP = 17
#[no_mangle]
pub extern "C" fn ip_input__part2364(ip_p: u8, m: *mut c_void, hlen: c_int) {
    unsafe {
        match ip_p as c_int {
            6 => {
                tcp_input(m, hlen, core::ptr::null_mut());
            }
            17 => {
                udp_input(m, hlen);
            }
            1 => {
                icmp_input(m, hlen);
            }
            _ => {
                m_free(m);
            }
        }
    }
}
}

// ── `return;` · slirp/ip_input.c:229-229 · chunk 2365 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2365 {
// Ported piece of ip_input(): the successful-path `return;` (ip_input.c line 229).
// It has no observable behaviour of its own; the C side keeps the `return`
// statement that follows the call, so control flow out of ip_input is unchanged.
#[no_mangle]
pub extern "C" fn ip_input__part2365() {
    // nothing to do: the tail of ip_input ends here
}
}

// ── `bad: m_free(m);` · slirp/ip_input.c:230-231 · chunk 2366 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     bad:
//      m_free(m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2366 {
// Free the mbuf for the "bad" path of ip_input.
use core::ffi::c_void;

// Opaque mirror of `struct mbuf`; only its address is ever handled here.
#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

extern "C" {
    fn m_free(m: *mut mbuf);
}

#[no_mangle]
pub extern "C" fn ip_input__part2366(m: *mut mbuf) {
    let _ = core::mem::size_of::<c_void>();
    unsafe {
        m_free(m);
    }
}
}

// ── `return;` · slirp/ip_input.c:232-232 · chunk 2367 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2367 {
// Ported piece of ip_input: the trailing `return;` of the `bad:` cleanup path.
// The surrounding C keeps the actual control flow (`return;`), so this helper
// has no parameters, no side effects and no return value.

#[no_mangle]
pub extern "C" fn ip_input__part2367() {
    // The C caller performs the return itself; nothing to do here.
}
}

// ── `register struct mbuf *m = dtom(slirp, ip); register struct ipasfrag *q; int hlen = ip->ip_hl <<…` · slirp/ip_input.c:246-262 · chunk 2369 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct mbuf *m = dtom(slirp, ip);
//      register struct ipasfrag *q;
//      int hlen = ip->ip_hl << 2;
//      int i, next;
//     
//      ;
//      ;
//      ;
//      ;
//     
//     
//     
//     
//     
//     
//      m->m_hdr.mh_data += hlen;
//      m->m_hdr.mh_len -= hlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2369 {
// Portion of ip_reass(): locate the mbuf for the fragment, compute the IP
// header length and strip it from the mbuf data area.

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
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip is __attribute__((packed))
#[repr(C, packed)]
pub struct Ip {
    // u_int ip_hl:4, ip_v:4;  (little endian: ip_hl = low nibble)
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
    pub m_dat: MDat,
}

extern "C" {
    fn dtom(slirp: *mut Slirp, m: *mut c_void) -> *mut Mbuf;
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2369(slirp: *mut Slirp, ip: *mut Ip) -> *mut Mbuf {
    let m: *mut Mbuf = dtom(slirp, ip as *mut c_void);
    let hlen: c_int = (((*ip).ip_hl_v & 0x0f) as c_int) << 2;

    if !m.is_null() {
        let data = (*m).m_hdr.mh_data;
        (*m).m_hdr.mh_data = data.offset(hlen as isize);
        (*m).m_hdr.mh_len -= hlen;
    }

    m
}
}

// ── `if (fp == ((void *)0)` · slirp/ip_input.c:267-283 · chunk 2370 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (fp == 
//                      ((void *)0)
//                          ) {
//        struct mbuf *t = m_get(slirp);
//     
//        if (t == 
//                ((void *)0)
//                    ) {
//            goto dropfrag;
//        }
//        fp = ((struct ipq *)(t)->m_hdr.mh_data);
//        slirp_insque(&fp->ip_link, &slirp->ipq.ip_link);
//        fp->ipq_ttl = 60;
//        fp->ipq_p = ip->ip_p;
//        fp->ipq_id = ip->ip_id;
//        fp->frag_link.next = fp->frag_link.prev = &fp->frag_link;
//        fp->ipq_src = ip->ip_src;
//        fp->ipq_dst = ip->ip_dst;
//        q = (struct ipasfrag *)fp;
//        goto insert;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2370 {
// Port of the "allocate a new fragment queue" branch of ip_reass()
// (slirp/ip_input.c).  Returns 0 when fp != NULL (nothing to do, the C
// caller keeps its fp/q), 1 when a fresh queue was built and inserted
// (caller must jump to `insert` with the returned fp/q) and 2 when the
// mbuf allocation failed (caller must jump to `dropfrag`).
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr::{addr_of, addr_of_mut};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ip { u_int ip_hl:4, ip_v:4; ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ip {
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

// struct ipq { struct qlink frag_link; struct qlink ip_link; ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

// struct ipasfrag { struct qlink ipf_link; struct ip ipf_ip; } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipasfrag {
    pub ipf_link: qlink,
    pub ipf_ip: ip,
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
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut c_void,
    pub m_dat_: *mut c_char,
}

const _: () = assert!(core::mem::size_of::<ip>() == 20);
const _: () = assert!(core::mem::size_of::<ipq>() == 44);

extern "C" {
    fn m_get(slirp: *mut c_void) -> *mut mbuf;
    fn slirp_insque(new_elem: *mut c_void, list: *mut c_void);
}

#[no_mangle]
pub extern "C" fn ip_reass__part2370(
    slirp: *mut c_void,
    ip: *mut ip,
    ipq_link: *mut qlink,
    fp_in: *mut ipq,
    out_fp: *mut *mut ipq,
    out_q: *mut *mut ipasfrag,
) -> c_int {
    if !fp_in.is_null() {
        return 0;
    }
    unsafe {
        let t = m_get(slirp);
        if t.is_null() {
            return 2;
        }
        let fp = (*t).m_hdr.mh_data as *mut ipq;
        slirp_insque(
            addr_of_mut!((*fp).ip_link) as *mut c_void,
            ipq_link as *mut c_void,
        );
        addr_of_mut!((*fp).ipq_ttl).write_unaligned(60u8);
        let ip_p = addr_of!((*ip).ip_p).read_unaligned();
        addr_of_mut!((*fp).ipq_p).write_unaligned(ip_p);
        let ip_id = addr_of!((*ip).ip_id).read_unaligned();
        addr_of_mut!((*fp).ipq_id).write_unaligned(ip_id);
        let self_link = addr_of_mut!((*fp).frag_link) as *mut c_void;
        addr_of_mut!((*fp).frag_link.next).write_unaligned(self_link);
        addr_of_mut!((*fp).frag_link.prev).write_unaligned(self_link);
        let src = addr_of!((*ip).ip_src).read_unaligned();
        addr_of_mut!((*fp).ipq_src).write_unaligned(src);
        let dst = addr_of!((*ip).ip_dst).read_unaligned();
        addr_of_mut!((*fp).ipq_dst).write_unaligned(dst);

        *out_fp = fp;
        *out_q = fp as *mut ipasfrag;
        1
    }
}
}

// ── `for (q = fp->frag_link.next; q != (struct ipasfrag *)&fp->frag_link; q = q->ipf_link.next)` · slirp/ip_input.c:288-291 · chunk 2371 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (q = fp->frag_link.next; q != (struct ipasfrag *)&fp->frag_link;
//                 q = q->ipf_link.next)
//       if (q->ipf_ip.ip_off > ip->ip_off)
//        break;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2371 {
// Port of the fragment-positioning loop of ip_reass():
// find the first queued fragment whose ip_off is greater than this
// fragment's ip_off (or the queue sentinel), and hand it back to C.
#![allow(non_camel_case_types)]

use core::ffi::c_void;

// struct qlink { void *next, *prev; }
#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ip (packed); the leading `u_int ip_hl:4, ip_v:4;` bitfield pair
// occupies a single byte in the first byte of the header.
#[repr(C, packed)]
pub struct ip {
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

// struct ipasfrag { struct qlink ipf_link; struct ip ipf_ip; }
#[repr(C, packed)]
pub struct ipasfrag {
    pub ipf_link: qlink,
    pub ipf_ip: ip,
}

// struct ipq
#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: u32,
    pub ipq_dst: u32,
}

#[no_mangle]
pub extern "C" fn ip_reass__part2371(fp: *mut ipq, ip: *mut ip) -> *mut ipasfrag {
    unsafe {
        // (struct ipasfrag *)&fp->frag_link
        let sentinel: *mut ipasfrag = core::ptr::addr_of_mut!((*fp).frag_link) as *mut ipasfrag;
        let target_off: u16 = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_off));

        // q = fp->frag_link.next;
        let mut q: *mut ipasfrag = core::ptr::read_unaligned(core::ptr::addr_of!(
            (*fp).frag_link.next
        )) as *mut ipasfrag;

        // for (...; q != sentinel; q = q->ipf_link.next)
        //   if (q->ipf_ip.ip_off > ip->ip_off) break;
        while q != sentinel {
            let q_off: u16 =
                core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_ip.ip_off));
            if q_off > target_off {
                break;
            }
            q = core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_link.next))
                as *mut ipasfrag;
        }

        q
    }
}
}

// ── `if (q->ipf_link.prev != &fp->frag_link)` · slirp/ip_input.c:298-308 · chunk 2372 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (q->ipf_link.prev != &fp->frag_link) {
//             struct ipasfrag *pq = q->ipf_link.prev;
//       i = pq->ipf_ip.ip_off + pq->ipf_ip.ip_len - ip->ip_off;
//       if (i > 0) {
//        if (i >= ip->ip_len)
//         goto dropfrag;
//        m_adj(dtom(slirp, ip), i);
//        ip->ip_off += i;
//        ip->ip_len -= i;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2372 {
// Port of the "trim overlap with the previous fragment" block of ip_reass().
// Returns 0 to continue, 1 when the caller must take the dropfrag path.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct Ip {
    // u_int ip_hl:4, ip_v:4;  (packed into a single byte)
    pub ip_hl_ip_v: u8,
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

#[repr(C, packed)]
pub struct IpAsFrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

extern "C" {
    fn dtom(slirp: *mut c_void, p: *mut c_void) -> *mut c_void;
    fn m_adj(m: *mut c_void, len: c_int);
}

#[inline]
unsafe fn ip_off(p: *mut Ip) -> u16 {
    core::ptr::addr_of!((*p).ip_off).read_unaligned()
}

#[inline]
unsafe fn ip_len(p: *mut Ip) -> u16 {
    core::ptr::addr_of!((*p).ip_len).read_unaligned()
}

#[inline]
unsafe fn set_ip_off(p: *mut Ip, v: u16) {
    core::ptr::addr_of_mut!((*p).ip_off).write_unaligned(v);
}

#[inline]
unsafe fn set_ip_len(p: *mut Ip, v: u16) {
    core::ptr::addr_of_mut!((*p).ip_len).write_unaligned(v);
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2372(
    slirp: *mut c_void,
    ip: *mut Ip,
    fp: *mut Ipq,
    q: *mut IpAsFrag,
    i_out: *mut c_int,
) -> c_int {
    let frag_link: *mut c_void = core::ptr::addr_of_mut!((*fp).frag_link).cast::<c_void>();
    let q_prev: *mut c_void = core::ptr::addr_of!((*q).ipf_link.prev).read_unaligned();

    if q_prev != frag_link {
        let pq = q_prev as *mut IpAsFrag;
        let pq_off = core::ptr::addr_of!((*pq).ipf_ip.ip_off).read_unaligned() as c_int;
        let pq_len = core::ptr::addr_of!((*pq).ipf_ip.ip_len).read_unaligned() as c_int;
        let cur_off = ip_off(ip) as c_int;
        let i: c_int = pq_off + pq_len - cur_off;
        if i > 0 {
            if i >= ip_len(ip) as c_int {
                return 1;
            }
            m_adj(dtom(slirp, ip as *mut c_void), i);
            set_ip_off(ip, ip_off(ip).wrapping_add(i as u16));
            set_ip_len(ip, ip_len(ip).wrapping_sub(i as u16));
        }
        if !i_out.is_null() {
            *i_out = i;
        }
    }

    0
}
}

// ── `while (q != (struct ipasfrag*)&fp->frag_link && ip->ip_off + ip->ip_len > q->ipf_ip.ip_off)` · slirp/ip_input.c:314-326 · chunk 2373 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (q != (struct ipasfrag*)&fp->frag_link &&
//                 ip->ip_off + ip->ip_len > q->ipf_ip.ip_off) {
//       i = (ip->ip_off + ip->ip_len) - q->ipf_ip.ip_off;
//       if (i < q->ipf_ip.ip_len) {
//        q->ipf_ip.ip_len -= i;
//        q->ipf_ip.ip_off += i;
//        m_adj(dtom(slirp, q), i);
//        break;
//       }
//       q = q->ipf_link.next;
//       m_free(dtom(slirp, q->ipf_link.prev));
//       ip_deq(q->ipf_link.prev);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2373 {
// Rust port of the fragment-list trimming loop of ip_reass().
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Ip {
    pub hl_ver: u8,
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
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct Ipasfrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

#[repr(C)]
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
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
}

extern "C" {
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: c_int);
    fn dtom(slirp: *mut Slirp, p: *mut c_void) -> *mut Mbuf;
}

#[inline]
unsafe fn ip_off(ip: *mut Ip) -> u16 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_off))
}

#[inline]
unsafe fn ip_len(ip: *mut Ip) -> u16 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_len))
}

#[inline]
unsafe fn frag_off(q: *mut Ipasfrag) -> u16 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_ip.ip_off))
}

#[inline]
unsafe fn frag_len(q: *mut Ipasfrag) -> u16 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_ip.ip_len))
}

#[inline]
unsafe fn set_frag_off(q: *mut Ipasfrag, v: u16) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*q).ipf_ip.ip_off), v);
}

#[inline]
unsafe fn set_frag_len(q: *mut Ipasfrag, v: u16) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*q).ipf_ip.ip_len), v);
}

#[inline]
unsafe fn link_next(q: *mut Ipasfrag) -> *mut c_void {
    core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_link.next))
}

#[inline]
unsafe fn link_prev(q: *mut Ipasfrag) -> *mut c_void {
    core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_link.prev))
}

#[inline]
unsafe fn set_link_next(q: *mut Ipasfrag, v: *mut c_void) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*q).ipf_link.next), v);
}

#[inline]
unsafe fn set_link_prev(q: *mut Ipasfrag, v: *mut c_void) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*q).ipf_link.prev), v);
}

// static void ip_deq(register struct ipasfrag *p) -- ported logic.
unsafe fn ip_deq(p: *mut Ipasfrag) {
    let prev = link_prev(p) as *mut Ipasfrag;
    set_link_next(prev, link_next(p));
    let next = link_next(p) as *mut Ipasfrag;
    set_link_prev(next, link_prev(p));
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2373(
    slirp: *mut Slirp,
    ip: *mut Ip,
    fp: *mut Ipq,
    mut q: *mut Ipasfrag,
) -> *mut Ipasfrag {
    let sentinel = core::ptr::addr_of_mut!((*fp).frag_link) as *mut Ipasfrag;
    loop {
        if q == sentinel {
            break;
        }
        let end: c_int = ip_off(ip) as c_int + ip_len(ip) as c_int;
        if !(end > frag_off(q) as c_int) {
            break;
        }
        let i: c_int = end - frag_off(q) as c_int;
        let qlen: c_int = frag_len(q) as c_int;
        if i < qlen {
            set_frag_len(q, (qlen - i) as u16);
            set_frag_off(q, (frag_off(q) as c_int + i) as u16);
            m_adj(dtom(slirp, q as *mut c_void), i);
            break;
        }
        q = link_next(q) as *mut Ipasfrag;
        m_free(dtom(slirp, link_prev(q)));
        ip_deq(link_prev(q) as *mut Ipasfrag);
    }
    q
}
}

// ── `insert: ip_enq(((struct ipasfrag *)(((char*)(ip)) - sizeof(struct qlink))), q->ipf_link.prev);` · slirp/ip_input.c:328-333 · chunk 2374 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     insert:
//     
//     
//     
//     
//      ip_enq(((struct ipasfrag *)(((char*)(ip)) - sizeof(struct qlink))), q->ipf_link.prev);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2374 {
// Port of ip_enq(iptofrag(ip), q->ipf_link.prev) from ip_reass.
// ip_enq is a static C helper with no linker symbol, so its logic lives here.

use core::ffi::c_void;

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[no_mangle]
pub extern "C" fn ip_reass__part2374(ip: *mut c_void, prev: *mut c_void) {
    unsafe {
        // iptofrag(ip): (struct ipasfrag *)((char *)ip - sizeof(struct qlink))
        let p = (ip as *mut u8).sub(core::mem::size_of::<Qlink>()) as *mut Qlink;
        let prevl = prev as *mut Qlink;

        // p->ipf_link.prev = prev;
        (*p).prev = prev;
        // p->ipf_link.next = prev->ipf_link.next;
        let nextl = (*prevl).next;
        (*p).next = nextl;
        // ((struct ipasfrag *)(prev->ipf_link.next))->ipf_link.prev = p;
        (*(nextl as *mut Qlink)).prev = p as *mut c_void;
        // prev->ipf_link.next = p;
        (*prevl).next = p as *mut c_void;
    }
}
}

// ── `next = 0;` · slirp/ip_input.c:334-334 · chunk 2375 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     next = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2375 {
// Sets the fragment-reassembly offset accumulator to zero.
// Mirrors the C statement `next = 0;` inside ip_reass().
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ip_reass__part2375() -> c_int {
    0
}
}

// ── `for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link; q = q->ipf_link.next)` · slirp/ip_input.c:335-340 · chunk 2376 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link;
//                 q = q->ipf_link.next) {
//       if (q->ipf_ip.ip_off != next)
//                             return 
//                                   ((void *)0)
//                                       ;
//       next += q->ipf_ip.ip_len;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2376 {
// ip_reass: check whether the queued fragments cover the datagram completely.
// Returns the terminating fragment pointer (never NULL on success); a NULL
// return means the C code must `return NULL`.
use core::ffi::{c_int, c_void};
use core::ptr::{addr_of, read_unaligned};

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

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

#[repr(C, packed)]
pub struct Ipasfrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: u32,
    pub ipq_dst: u32,
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2376(
    fp: *mut Ipq,
    nextp: *mut c_int,
) -> *mut Ipasfrag {
    if fp.is_null() || nextp.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        let mut next = *nextp;
        let frag_link: *const Qlink = addr_of!((*fp).frag_link);
        let head = frag_link as *mut Ipasfrag;
        let mut q = (*frag_link).next as *mut Ipasfrag;

        while q != head {
            if q.is_null() {
                *nextp = next;
                return core::ptr::null_mut();
            }
            let ipf: *const Ip = addr_of!((*q).ipf_ip);
            let off = read_unaligned(addr_of!((*ipf).ip_off));
            if off as c_int != next {
                *nextp = next;
                return core::ptr::null_mut();
            }
            let len = read_unaligned(addr_of!((*ipf).ip_len));
            next = next.wrapping_add(len as c_int);

            let link: *const Qlink = addr_of!((*q).ipf_link);
            q = read_unaligned(addr_of!((*link).next)) as *mut Ipasfrag;
        }

        *nextp = next;
        q
    }
}
}

// ── `if ((struct ipasfrag *)(q->ipf_link.prev))->ipf_ip.ip_tos & 1)` · slirp/ip_input.c:341-342 · chunk 2377 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (((struct ipasfrag *)(q->ipf_link.prev))->ipf_ip.ip_tos & 1)
//                     return 
//                           ((void *)0)
//                               ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2377 {
// Port of the fragment check at slirp/ip_input.c:341-342 inside ip_reass().
// Returns nonzero when the reassembled datagram's last fragment carries the
// "more fragments" bit in ip_tos, in which case the caller must return NULL.

use core::ffi::{c_int, c_void};

#[repr(C, packed)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip (packed); ip_hl:4 / ip_v:4 share the first byte.
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

#[repr(C, packed)]
pub struct Ipasfrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2377(q: *mut Ipasfrag) -> c_int {
    if q.is_null() {
        return 0;
    }
    let prev = (*q).ipf_link.prev as *mut Ipasfrag;
    if prev.is_null() {
        return 0;
    }
    let tos = (*prev).ipf_ip.ip_tos;
    (tos & 1) as c_int
}
}

// ── `q = fp->frag_link.next; m = dtom(slirp, q); q = (struct ipasfrag *) q->ipf_link.next;` · slirp/ip_input.c:347-350 · chunk 2378 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     q = fp->frag_link.next;
//      m = dtom(slirp, q);
//     
//      q = (struct ipasfrag *) q->ipf_link.next;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2378 {
// Port of the fragment-list head retrieval in ip_reass (slirp/ip_input.c).
#![allow(non_camel_case_types)]

use core::ffi::c_void;

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct ipasfrag {
    pub ipf_link: qlink,
    // struct ip ipf_ip follows; not accessed here.
}

#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    // remaining fields not accessed here.
}

extern "C" {
    fn dtom(slirp: *mut Slirp, ptr: *mut c_void) -> *mut mbuf;
}

/// Equivalent of:
///   q = fp->frag_link.next;
///   m = dtom(slirp, q);
///   q = (struct ipasfrag *)q->ipf_link.next;
#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2378(
    slirp: *mut Slirp,
    fp: *mut ipq,
    q_out: *mut *mut ipasfrag,
    m_out: *mut *mut mbuf,
) {
    let first = (*fp).frag_link.next as *mut ipasfrag;
    let m = dtom(slirp, first as *mut c_void);
    let second = (*first).ipf_link.next as *mut ipasfrag;
    *q_out = second;
    *m_out = m;
}
}

// ── `while (q != (struct ipasfrag*)&fp->frag_link)` · slirp/ip_input.c:351-355 · chunk 2379 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (q != (struct ipasfrag*)&fp->frag_link) {
//        struct mbuf *t = dtom(slirp, q);
//        q = (struct ipasfrag *) q->ipf_link.next;
//        m_cat(m, t);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2379 {
// Port of the fragment-merging loop inside slirp's ip_reass():
//   while (q != (struct ipasfrag*)&fp->frag_link) {
//     struct mbuf *t = dtom(slirp, q);
//     q = (struct ipasfrag *) q->ipf_link.next;
//     m_cat(m, t);
//   }
use core::ffi::c_void;

// struct mbuf and Slirp are only passed through by pointer here.
#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut Qlink,
    pub prev: *mut Qlink,
}

// struct ipasfrag { struct qlink ipf_link; struct ip ipf_ip; } (packed)
#[repr(C, packed)]
pub struct IpAsFrag {
    pub ipf_link: Qlink,
    pub ipf_ip: [u8; 20],
}

// struct ipq { struct qlink frag_link; struct qlink ip_link; uint8_t ipq_ttl;
//             uint8_t ipq_p; uint16_t ipq_id; struct in_addr ipq_src, ipq_dst; }
#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

extern "C" {
    fn dtom(slirp: *mut Slirp, p: *mut c_void) -> *mut Mbuf;
    fn m_cat(m: *mut Mbuf, n: *mut Mbuf);
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2379(
    slirp: *mut Slirp,
    mut q: *mut IpAsFrag,
    fp: *mut IpQ,
    m: *mut Mbuf,
) -> *mut IpAsFrag {
    // &fp->frag_link, which sits at offset 0 of the packed struct ipq
    let head: *mut IpAsFrag = if fp.is_null() {
        core::ptr::null_mut()
    } else {
        core::ptr::addr_of_mut!((*fp).frag_link) as *mut IpAsFrag
    };

    while q != head {
        let t: *mut Mbuf = dtom(slirp, q as *mut c_void);
        let next: *mut Qlink = core::ptr::addr_of!((*q).ipf_link.next).read_unaligned();
        q = next as *mut IpAsFrag;
        m_cat(m, t);
    }

    q
}
}

// ── `q = fp->frag_link.next;` · slirp/ip_input.c:363-363 · chunk 2380 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     q = fp->frag_link.next;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2380 {
// Port of the statement `q = fp->frag_link.next;` from ip_reass().
// Returns the `next` pointer stored in the ipq's frag_link queue head.

#[repr(C)]
pub struct qlink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
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

#[no_mangle]
pub extern "C" fn ip_reass__part2380(fp: *mut ipq) -> *mut core::ffi::c_void {
    if fp.is_null() {
        return core::ptr::null_mut();
    }
    unsafe { (*fp).frag_link.next }
}
}

// ── `if (m->m_hdr.mh_flags & 0x01)` · slirp/ip_input.c:372-375 · chunk 2381 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->m_hdr.mh_flags & 0x01) {
//        int delta = (char *)q - m->M_dat.m_dat_;
//        q = (struct ipasfrag *)(m->M_dat.m_ext_ + delta);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2381 {
// Port of the M_EXT pointer-relocation fragment of slirp's ip_reass().
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// struct qlink { void *next, *prev; };
#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

// struct ip { ... } __attribute__((packed));
#[repr(C, packed)]
pub struct Ip {
    pub ip_hl_v: u8, // u_int ip_hl:4, ip_v:4;
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: [u8; 4],
    pub ip_dst: [u8; 4],
}

// struct ipasfrag { struct qlink ipf_link; struct ip ipf_ip; } __attribute__((packed));
#[repr(C, packed)]
pub struct Ipasfrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

// union M_dat { char m_dat_[1]; char *m_ext_; };
#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// struct m_hdr { ... };
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

// struct mbuf { struct m_hdr m_hdr; Slirp *slirp; union M_dat M_dat; };
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void, // Slirp *
    pub M_dat: MDat,
}

/// if (m->m_hdr.mh_flags & M_EXT) {
///     int delta = (char *)q - m->M_dat.m_dat_;
///     q = (struct ipasfrag *)(m->M_dat.m_ext_ + delta);
/// }
/// Returns the (possibly relocated) fragment pointer.
#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2381(
    q_in: *mut Ipasfrag,
    m: *mut Mbuf,
) -> *mut Ipasfrag {
    let mut q = q_in;
    if m.is_null() {
        return q;
    }
    if (*m).m_hdr.mh_flags & 0x01 != 0 {
        // address of the M_dat union == address of m_dat_ (offset 0, size 1)
        let dat = core::ptr::addr_of!((*m).M_dat) as *const c_char;
        let ext = (*m).M_dat.m_ext_;
        let delta: c_int = (q as *const c_char as isize).wrapping_sub(dat as isize) as c_int;
        q = (ext as *mut c_char).wrapping_offset(delta as isize) as *mut Ipasfrag;
    }
    q
}
}

// ── `ip = ((struct ip*)(((char*)(q)) + sizeof(struct qlink))); ip->ip_len = next; ip->ip_tos &= ~1;…` · slirp/ip_input.c:377-387 · chunk 2382 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ip = ((struct ip*)(((char*)(q)) + sizeof(struct qlink)));
//      ip->ip_len = next;
//      ip->ip_tos &= ~1;
//      ip->ip_src = fp->ipq_src;
//      ip->ip_dst = fp->ipq_dst;
//      slirp_remque(&fp->ip_link);
//      (void) m_free(dtom(slirp, fp));
//      m->m_hdr.mh_len += (ip->ip_hl << 2);
//      m->m_hdr.mh_data -= (ip->ip_hl << 2);
//     
//      return ip;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2382 {
// Ported fragment (tail) of static ip_reass() from slirp/ip_input.c:
// fix up the reassembled header, unlink/free the fragment queue, adjust the
// mbuf header and return the reassembled ip header.

use core::ffi::{c_char, c_int, c_void};

// Opaque: only ever passed through to dtom().
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ip (packed); ip_hl:4/ip_v:4 share the first byte (ip_hl = low nibble).
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Ipasfrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

#[repr(C)]
pub struct Mhdr {
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
    pub m_hdr: Mhdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

extern "C" {
    fn slirp_remque(p: *mut c_void);
    fn m_free(m: *mut Mbuf);
    fn dtom(slirp: *mut Slirp, dat: *mut c_void) -> *mut Mbuf;
}

#[no_mangle]
pub unsafe extern "C" fn ip_reass__part2382(
    slirp: *mut Slirp,
    q: *mut Ipasfrag,
    next: c_int,
    fp: *mut Ipq,
    m: *mut Mbuf,
) -> *mut Ip {
    // ip = (struct ip *)((char *)q + sizeof(struct qlink));
    let ip = (q as *mut u8).add(core::mem::size_of::<Qlink>()) as *mut Ip;

    // ip->ip_len = next;
    core::ptr::addr_of_mut!((*ip).ip_len).write_unaligned(next as u16);

    // ip->ip_tos &= ~1;
    let tos_ptr = core::ptr::addr_of_mut!((*ip).ip_tos);
    let tos = core::ptr::read_unaligned(tos_ptr);
    tos_ptr.write_unaligned(tos & !1u8);

    // ip->ip_src = fp->ipq_src; ip->ip_dst = fp->ipq_dst;
    let src = core::ptr::read_unaligned(core::ptr::addr_of!((*fp).ipq_src));
    let dst = core::ptr::read_unaligned(core::ptr::addr_of!((*fp).ipq_dst));
    core::ptr::addr_of_mut!((*ip).ip_src).write_unaligned(src);
    core::ptr::addr_of_mut!((*ip).ip_dst).write_unaligned(dst);

    // slirp_remque(&fp->ip_link);
    slirp_remque(core::ptr::addr_of_mut!((*fp).ip_link) as *mut c_void);

    // (void) m_free(dtom(slirp, fp));
    m_free(dtom(slirp, fp as *mut c_void));

    // m->m_hdr.mh_len += (ip->ip_hl << 2);
    // m->m_hdr.mh_data -= (ip->ip_hl << 2);
    let hlen: c_int = ((core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_hl_v)) & 0x0f) as c_int) << 2;

    let len_ptr = core::ptr::addr_of_mut!((*m).m_hdr.mh_len);
    let cur_len = len_ptr.read_unaligned();
    len_ptr.write_unaligned(cur_len.wrapping_add(hlen));

    let data_ptr = core::ptr::addr_of_mut!((*m).m_hdr.mh_data);
    let cur_data = data_ptr.read_unaligned();
    data_ptr.write_unaligned(cur_data.offset(-(hlen as isize)));

    ip
}
}

// ── `dropfrag: m_free(m);` · slirp/ip_input.c:389-390 · chunk 2383 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     dropfrag:
//      m_free(m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2383 {
// Ported piece of ip_reass: dropfrag label body -> free the mbuf.
use core::ffi::c_void;

extern "C" {
    // void m_free(struct mbuf *);
    fn m_free(m: *mut c_void);
}

#[no_mangle]
pub extern "C" fn ip_reass__part2383(m: *mut c_void) {
    unsafe {
        m_free(m);
    }
}
}

// ── `return ((void *)0) ;` · slirp/ip_input.c:391-391 · chunk 2384 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2384 {
// Port of the `return NULL;` at the dropfrag label of ip_reass().
// It simply yields a null `struct ip *` to the caller.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn ip_reass__part2384() -> *mut c_void {
    core::ptr::null_mut()
}
}

// ── `for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link; q = p)` · slirp/ip_input.c:403-407 · chunk 2386 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link; q = p) {
//       p = q->ipf_link.next;
//       ip_deq(q);
//       m_free(dtom(slirp, q));
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2386 {
// Port of the fragment-list drain loop at the top of ip_freef (slirp/ip_input.c).

use core::ffi::c_void;

// Opaque mirrors of C types only touched through pointers.
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ipq { struct qlink frag_link; struct qlink ip_link; uint8_t ipq_ttl;
//             uint8_t ipq_p; uint16_t ipq_id; struct in_addr ipq_src, ipq_dst; }
// __attribute__((packed))
#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

// struct ip { u_int ip_hl:4, ip_v:4; uint8_t ip_tos; uint16_t ip_len;
//             uint16_t ip_id; uint16_t ip_off; uint8_t ip_ttl; uint8_t ip_p;
//             uint16_t ip_sum; struct in_addr ip_src, ip_dst; } packed
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

// struct ipasfrag { struct qlink ipf_link; struct ip ipf_ip; } packed
#[repr(C, packed)]
pub struct IpAsFrag {
    pub ipf_link: Qlink,
    pub ipf_ip: Ip,
}

extern "C" {
    fn dtom(slirp: *mut Slirp, p: *mut c_void) -> *mut Mbuf;
    fn m_free(m: *mut Mbuf);
}

// static void ip_deq(register struct ipasfrag *p) -- static, so ported here.
#[inline]
unsafe fn ip_deq(p: *mut IpAsFrag) {
    let prev = core::ptr::read_unaligned(core::ptr::addr_of!((*p).ipf_link.prev)) as *mut IpAsFrag;
    let next = core::ptr::read_unaligned(core::ptr::addr_of!((*p).ipf_link.next)) as *mut IpAsFrag;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*prev).ipf_link.next), next as *mut c_void);
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*next).ipf_link.prev), prev as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ip_freef__part2386(slirp: *mut Slirp, fp: *mut IpQ) {
    let head = core::ptr::addr_of_mut!((*fp).frag_link) as *mut IpAsFrag;
    let mut q =
        core::ptr::read_unaligned(core::ptr::addr_of!((*fp).frag_link.next)) as *mut IpAsFrag;

    while q != head {
        let p = core::ptr::read_unaligned(core::ptr::addr_of!((*q).ipf_link.next)) as *mut IpAsFrag;
        ip_deq(q);
        m_free(dtom(slirp, q as *mut c_void));
        q = p;
    }
}
}

// ── `slirp_remque(&fp->ip_link); (void) m_free(dtom(slirp, fp));` · slirp/ip_input.c:408-409 · chunk 2387 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp_remque(&fp->ip_link);
//      (void) m_free(dtom(slirp, fp));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2387 {
// Port of the tail of slirp ip_freef: unlink the ipq from its list and free
// the mbuf that contains it.
use core::ffi::c_void;

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ipq is __attribute__((packed)) in C; keep the exact field order/types.
#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

extern "C" {
    fn slirp_remque(elem: *mut c_void);
    fn m_free(m: *mut c_void);
    fn dtom(slirp: *mut c_void, data: *mut c_void) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ip_freef__part2387(slirp: *mut c_void, fp: *mut IpQ) {
    let ip_link = core::ptr::addr_of_mut!((*fp).ip_link) as *mut c_void;
    slirp_remque(ip_link);
    let m = dtom(slirp, fp as *mut c_void);
    m_free(m);
}
}

// ── `; ; p->ipf_link.prev = prev; p->ipf_link.next = prev->ipf_link.next; ((struct ipasfrag *)(prev-…` · slirp/ip_input.c:419-424 · chunk 2389 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      ;
//      p->ipf_link.prev = prev;
//      p->ipf_link.next = prev->ipf_link.next;
//      ((struct ipasfrag *)(prev->ipf_link.next))->ipf_link.prev = p;
//      prev->ipf_link.next = p;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2389 {
// Ported piece of ip_enq: links fragment p after prev in the fragment list.
use core::ffi::c_void;

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct IpasFrag {
    pub ipf_link: Qlink,
}

#[no_mangle]
pub unsafe extern "C" fn ip_enq__part2389(p: *mut IpasFrag, prev: *mut IpasFrag) {
    (*p).ipf_link.prev = prev as *mut c_void;
    (*p).ipf_link.next = (*prev).ipf_link.next;
    let next: *mut IpasFrag = (*prev).ipf_link.next as *mut IpasFrag;
    (*next).ipf_link.prev = p as *mut c_void;
    (*prev).ipf_link.next = p as *mut c_void;
}
}

// ── `((struct ipasfrag *)(p->ipf_link.prev))->ipf_link.next = p->ipf_link.next; ((struct ipasfrag *)…` · slirp/ip_input.c:433-434 · chunk 2391 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((struct ipasfrag *)(p->ipf_link.prev))->ipf_link.next = p->ipf_link.next;
//      ((struct ipasfrag *)(p->ipf_link.next))->ipf_link.prev = p->ipf_link.prev;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2391 {
use core::ffi::c_void;

// layout-identical prefix of `struct ipasfrag` (struct qlink { void *next, *prev; })
#[repr(C)]
struct Qlink {
    next: *mut c_void,
    prev: *mut c_void,
}

#[no_mangle]
pub extern "C" fn ip_deq__part2391(p: *mut c_void) {
    unsafe {
        let q = p as *mut Qlink;
        let prev = (*q).prev;
        (*(prev as *mut Qlink)).next = (*q).next;
        let next = (*q).next;
        (*(next as *mut Qlink)).prev = (*q).prev;
    }
}
}

// ── `struct qlink *l; ; l = slirp->ipq.ip_link.next;` · slirp/ip_input.c:445-449 · chunk 2393 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct qlink *l;
//     
//      ;
//     
//         l = slirp->ipq.ip_link.next;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2393 {
// Part of ip_slowtimo: fetch the head link's `next` pointer.
// struct qlink { void *next, *prev; };

#[repr(C)]
pub struct Qlink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn ip_slowtimo__part2393(head: *mut Qlink) -> *mut Qlink {
    unsafe { (*head).next as *mut Qlink }
}
}

// ── `if (l == ((void *)0)` · slirp/ip_input.c:451-452 · chunk 2394 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (l == 
//                     ((void *)0)
//                         )
//         return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2394 {
// Port of the null check in ip_slowtimo: returns nonzero when l is NULL,
// telling the caller to return immediately.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn ip_slowtimo__part2394(l: *mut c_void) -> c_int {
    if l.is_null() {
        1
    } else {
        0
    }
}
}

// ── `while (l != &slirp->ipq.ip_link)` · slirp/ip_input.c:454-460 · chunk 2395 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (l != &slirp->ipq.ip_link) {
//             struct ipq *fp = ({ const typeof( ((struct ipq *)0)->ip_link ) *__mptr = (l); (struct ipq *)( (char *)__mptr - 
//                             __builtin_offsetof (
//                             struct ipq
//                             , 
//                             ip_link
//                             ) 
//                             );});
//             l = l->next;
//       if (--fp->ipq_ttl == 0) {
//        ip_freef(slirp, fp);
//       }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2395 {
// Port of the expiration loop of ip_slowtimo() (slirp/ip_input.c).
// The static helper ip_freef() (and ip_deq()) have no linker symbol, so
// their logic is mirrored here as private Rust functions.

use core::ffi::c_void;

// Opaque mirror of `Slirp` (only ever used through a pointer).
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

// struct qlink { void *next, *prev; }
#[repr(C)]
pub struct Qlink {
    next: *mut c_void,
    prev: *mut c_void,
}

// struct ipq { struct qlink frag_link; struct qlink ip_link;
//              uint8_t ipq_ttl; uint8_t ipq_p; uint16_t ipq_id;
//              struct in_addr ipq_src, ipq_dst; } __attribute__((packed));
// All members are naturally aligned, so repr(C) yields identical offsets.
#[repr(C)]
struct Ipq {
    frag_link: Qlink,
    ip_link: Qlink,
    ipq_ttl: u8,
    ipq_p: u8,
    ipq_id: u16,
    ipq_src: u32,
    ipq_dst: u32,
}

extern "C" {
    fn dtom(slirp: *mut Slirp, p: *mut c_void) -> *mut c_void;
    fn m_free(m: *mut c_void);
    fn slirp_remque(q: *mut c_void);
}

// static void ip_deq(register struct ipasfrag *p)
// `struct ipasfrag` starts with `struct qlink ipf_link`, so a Qlink
// pointer is layout compatible with the fragment header.
unsafe fn ip_deq(p: *mut Qlink) {
    let prev = (*p).prev as *mut Qlink;
    let next = (*p).next as *mut Qlink;
    (*prev).next = next as *mut c_void;
    (*next).prev = prev as *mut c_void;
}

// static void ip_freef(Slirp *slirp, struct ipq *fp)
unsafe fn ip_freef(slirp: *mut Slirp, fp: *mut Ipq) {
    let frag_head = core::ptr::addr_of_mut!((*fp).frag_link) as *mut Qlink;
    let mut q = (*fp).frag_link.next as *mut Qlink;
    while q != frag_head {
        let p = (*q).next as *mut Qlink;
        ip_deq(q);
        m_free(dtom(slirp, q as *mut c_void));
        q = p;
    }
    slirp_remque(core::ptr::addr_of_mut!((*fp).ip_link) as *mut c_void);
    m_free(dtom(slirp, fp as *mut c_void));
}

#[no_mangle]
pub unsafe extern "C" fn ip_slowtimo__part2395(
    slirp: *mut Slirp,
    mut l: *mut Qlink,
    head: *mut Qlink,
) {
    let ip_link_off = 2 * core::mem::size_of::<Qlink>();
    while l != head {
        let fp = (l as *mut u8).sub(ip_link_off) as *mut Ipq;
        l = (*l).next as *mut Qlink;
        let ttl = (*fp).ipq_ttl.wrapping_sub(1);
        (*fp).ipq_ttl = ttl;
        if ttl == 0 {
            ip_freef(slirp, fp);
        }
    }
}
}

// ── `register int i; struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data); register caddr_t opts; int o…` · slirp/ip_input.c:673-684 · chunk 2397 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register int i;
//      struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data);
//      register caddr_t opts;
//      int olen;
//     
//      olen = (ip->ip_hl<<2) - sizeof (struct ip);
//      opts = (caddr_t)(ip + 1);
//      i = m->m_hdr.mh_len - (sizeof (struct ip) + olen);
//      memcpy(opts, opts + olen, (unsigned)i);
//      m->m_hdr.mh_len -= olen;
//     
//      ip->ip_hl = sizeof(struct ip) >> 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2397 {
// Port of ip_stripoptions body (slirp/ip_input.c lines 673-684).
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
    pub slirp: *mut c_void,
    pub m_dat: [c_char; 1],
}

// struct ip is __attribute__((packed)); first byte holds ip_hl:4 (low) and ip_v:4.
#[repr(C, packed)]
pub struct Ip {
    pub hl_v: u8,
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

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ip_stripoptions__part2397(
    m: *mut Mbuf,
    _mopt: *mut Mbuf,
) {
    let ip_size = core::mem::size_of::<Ip>() as c_int;
    let ip = (*m).m_hdr.mh_data as *mut Ip;
    let base = ip as *mut u8;

    // olen = (ip->ip_hl << 2) - sizeof(struct ip)
    let hl = (*base & 0x0fu8) as c_int;
    let olen: c_int = (hl << 2) - ip_size;

    // opts = (caddr_t)(ip + 1)
    let opts: *mut u8 = base.add(ip_size as usize);

    // i = m->m_hdr.mh_len - (sizeof(struct ip) + olen)
    let i: c_int = (*m).m_hdr.mh_len - (ip_size + olen);

    memcpy(
        opts as *mut c_void,
        opts.offset(olen as isize) as *const c_void,
        (i as u32) as usize,
    );

    (*m).m_hdr.mh_len -= olen;

    // ip->ip_hl = sizeof(struct ip) >> 2
    *base = (*base & 0xf0u8) | ((ip_size >> 2) as u8);
}
}

