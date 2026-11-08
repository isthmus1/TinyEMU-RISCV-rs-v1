//! From `slirp/bootp.c`.

// ── `for (i = 0; i < 16; i++)` · slirp/bootp.c:45-49 · chunk 2233 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 16; i++) {
//             bc = &slirp->bootp_clients[i];
//             if (!bc->allocated || !memcmp(macaddr, bc->macaddr, 6))
//                 goto found;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2233 {
// Ported piece of get_new_addr (slirp/bootp.c): the search loop over the
// 16 BOOTP client slots. Returns the matching/free slot index, or -1.

use core::ffi::{c_int, c_void};
use core::ptr;

// Mirror of: typedef struct { uint16_t allocated; uint8_t macaddr[6]; } BOOTPClient;
#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

extern "C" {
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

#[no_mangle]
pub extern "C" fn get_new_addr__part2233(
    clients: *mut BootpClient,
    macaddr: *const u8,
) -> c_int {
    let mut i: c_int = 0;
    while i < 16 {
        unsafe {
            let bc = clients.add(i as usize);
            let allocated = (*bc).allocated;
            if allocated == 0
                || memcmp(
                    macaddr as *const c_void,
                    ptr::addr_of!((*bc).macaddr) as *const c_void,
                    6,
                ) == 0
            {
                return i;
            }
        }
        i += 1;
    }
    -1
}
}

// ── `return ((void *)0) ;` · slirp/bootp.c:50-50 · chunk 2234 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2234 {
// Port of the `return NULL;` statement of get_new_addr (slirp/bootp.c).
// The "no free BOOTP client slot found" path simply yields a null pointer.

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[no_mangle]
pub extern "C" fn get_new_addr__part2234() -> *mut BOOTPClient {
    core::ptr::null_mut()
}
}

// ── `found: bc = &slirp->bootp_clients[i];` · slirp/bootp.c:51-52 · chunk 2235 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     found:
//         bc = &slirp->bootp_clients[i];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2235 {
// Port of `bc = &slirp->bootp_clients[i];` (label `found:` in get_new_addr).
#![allow(dead_code, non_camel_case_types, non_snake_case)]

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
pub struct MHdr {
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

// Mirrors `union M_dat { char m_dat_[1]; char *m_ext_; }` (size 8, align 8).
#[repr(C)]
pub struct MDat {
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

#[repr(C, packed)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
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

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

// Only the fields preceding (and including) bootp_clients are mirrored.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [u8; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void,
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
}

#[no_mangle]
pub extern "C" fn get_new_addr__part2235(slirp: *mut Slirp, i: c_int) -> *mut BOOTPClient {
    unsafe {
        let base: *mut BOOTPClient = core::ptr::addr_of_mut!((*slirp).bootp_clients) as *mut BOOTPClient;
        base.add(i as usize)
    }
}
}

// ── `bc->allocated = 1; paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i); return bc;` · slirp/bootp.c:53-55 · chunk 2236 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     bc->allocated = 1;
//         paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i);
//         return bc;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2236 {
// Port of the "found:" tail of get_new_addr from slirp/bootp.c
//   bc->allocated = 1;
//   paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i);
//   return bc;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

extern "C" {
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn get_new_addr__part2236(
    bc: *mut BootpClient,
    paddr: *mut InAddr,
    vdhcp_startaddr_s_addr: u32,
    i: core::ffi::c_int,
) -> *mut BootpClient {
    if !bc.is_null() {
        (*bc).allocated = 1;
    }
    if !paddr.is_null() {
        (*paddr).s_addr = vdhcp_startaddr_s_addr.wrapping_add(htonl(i as u32));
    }
    bc
}
}

// ── `if (req_addr >= dhcp_addr && req_addr < (dhcp_addr + 16)` · slirp/bootp.c:65-72 · chunk 2238 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (req_addr >= dhcp_addr &&
//             req_addr < (dhcp_addr + 16)) {
//             bc = &slirp->bootp_clients[req_addr - dhcp_addr];
//             if (!bc->allocated || !memcmp(macaddr, bc->macaddr, 6)) {
//                 bc->allocated = 1;
//                 return bc;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2238 {
// Port of the address-request lookup inside request_addr() from slirp/bootp.c.
// Operates on the bootp client table base pointer so the full Slirp layout
// does not need to be mirrored.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

extern "C" {
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn request_addr__part2238(
    clients: *mut BootpClient,
    req_addr: u32,
    dhcp_addr: u32,
    macaddr: *const u8,
) -> *mut BootpClient {
    // uint32_t arithmetic wraps; keep the exact C semantics of dhcp_addr + 16.
    if req_addr >= dhcp_addr && req_addr < dhcp_addr.wrapping_add(16) {
        let idx = req_addr.wrapping_sub(dhcp_addr) as usize;
        let bc = clients.add(idx);
        if (*bc).allocated == 0
            || memcmp(
                macaddr as *const c_void,
                (*bc).macaddr.as_ptr() as *const c_void,
                6,
            ) == 0
        {
            (*bc).allocated = 1;
            return bc;
        }
    }
    core::ptr::null_mut()
}
}

// ── `return ((void *)0) ;` · slirp/bootp.c:73-73 · chunk 2239 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2239 {
// Returns the null pointer (the failure path of request_addr).
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn request_addr__part2239() -> *mut c_void {
    core::ptr::null_mut()
}
}

// ── `for (i = 0; i < 16; i++)` · slirp/bootp.c:82-85 · chunk 2241 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 16; i++) {
//             if (!memcmp(macaddr, slirp->bootp_clients[i].macaddr, 6))
//                 goto found;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2241 {
use core::ffi::c_int;

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

// Returns the index of the first client whose macaddr equals macaddr,
// or -1 if none of the 16 entries matches.
#[no_mangle]
pub extern "C" fn find_addr__part2241(macaddr: *const u8, clients: *const BOOTPClient) -> c_int {
    let mut i: c_int = 0;
    while i < 16 {
        unsafe {
            let cmac: *const u8 = (*clients.offset(i as isize)).macaddr.as_ptr();
            let mut same = true;
            let mut k: usize = 0;
            while k < 6 {
                if *macaddr.add(k) != *cmac.add(k) {
                    same = false;
                    break;
                }
                k += 1;
            }
            if same {
                return i;
            }
        }
        i += 1;
    }
    -1
}
}

// ── `return ((void *)0) ;` · slirp/bootp.c:86-86 · chunk 2242 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2242 {
// Piece of find_addr(): the "no matching client" return of a null pointer.

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[no_mangle]
pub extern "C" fn find_addr__part2242() -> *mut BOOTPClient {
    core::ptr::null_mut()
}
}

// ── `found: bc = &slirp->bootp_clients[i];` · slirp/bootp.c:87-88 · chunk 2243 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     found:
//         bc = &slirp->bootp_clients[i];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2243 {
// Ported piece of slirp/bootp.c: find_addr()'s "found:" label body.
// Computes &slirp->bootp_clients[i] where BOOTPClient is
//   typedef struct { uint16_t allocated; uint8_t macaddr[6]; } BOOTPClient;
// (size 8, alignment 2), which the caller passes as the base of the array.

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[no_mangle]
pub extern "C" fn find_addr__part2243(
    clients: *mut BootpClient,
    i: core::ffi::c_int,
) -> *mut BootpClient {
    if clients.is_null() {
        return core::ptr::null_mut();
    }
    let idx: usize = if i < 0 { 0 } else { i as usize };
    unsafe { clients.add(idx) }
}
}

// ── `bc->allocated = 1; paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i); return bc;` · slirp/bootp.c:89-91 · chunk 2244 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     bc->allocated = 1;
//         paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i);
//         return bc;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2244 {
// Port of find_addr()'s "found:" tail: mark the client allocated, compute the
// address and hand back the client entry.

use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

extern "C" {
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn find_addr__part2244(
    bc: *mut BootpClient,
    paddr: *mut InAddr,
    vdhcp_startaddr: *const InAddr,
    i: c_int,
) -> *mut BootpClient {
    unsafe {
        (*bc).allocated = 1;
        (*paddr).s_addr = (*vdhcp_startaddr).s_addr.wrapping_add(htonl(i as u32));
    }
    bc
}
}

// ── `const uint8_t *p, *p_end; int len, tag; *pmsg_type = 0; preq_addr->s_addr = htonl(0L); p = bp->…` · slirp/bootp.c:97-104 · chunk 2246 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const uint8_t *p, *p_end;
//         int len, tag;
//     
//         *pmsg_type = 0;
//         preq_addr->s_addr = htonl(0L);
//     
//         p = bp->bp_vend;
//         p_end = p + 312;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2246 {
use core::ffi::c_int;

// Mirror of `struct in_addr` (netinet/in.h)
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct ip` (packed); ip_hl:4 / ip_v:4 bitfields share one byte
#[repr(C, packed)]
pub struct Ip {
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

// Mirror of `struct udphdr`
#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

// Mirror of `struct bootp_t`
#[repr(C)]
pub struct BootpT {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

extern "C" {
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn dhcp_decode__part2246(
    bp: *const BootpT,
    pmsg_type: *mut c_int,
    preq_addr: *mut InAddr,
    pp: *mut *const u8,
    pp_end: *mut *const u8,
) {
    *pmsg_type = 0;
    (*preq_addr).s_addr = htonl(0);

    let p: *const u8 = core::ptr::addr_of!((*bp).bp_vend) as *const u8;
    *pp = p;
    *pp_end = p.add(312);
}
}

// ── `if (memcmp(p, rfc1533_cookie, 4) != 0)` · slirp/bootp.c:105-106 · chunk 2247 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (memcmp(p, rfc1533_cookie, 4) != 0)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2247 {
// Port of the cookie check at the start of dhcp_decode's vendor-option scan.
// The C file keeps `rfc1533_cookie` as a file-scope `static` array (no linker
// symbol), so the four cookie bytes are mirrored here.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn dhcp_decode__part2247(p: *const u8) -> c_int {
    if p.is_null() {
        return 1;
    }
    unsafe {
        let b0 = *p;
        let b1 = *p.add(1);
        let b2 = *p.add(2);
        let b3 = *p.add(3);
        if b0 != 99u8 || b1 != 130u8 || b2 != 83u8 || b3 != 99u8 {
            1
        } else {
            0
        }
    }
}
}

// ── `p += 4;` · slirp/bootp.c:107-107 · chunk 2248 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p += 4;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2248 {
// Portion of dhcp_decode: advance the vendor-option cursor past the
// RFC1533 cookie (p += 4;).

#[no_mangle]
pub extern "C" fn dhcp_decode__part2248(p: *const u8) -> *const u8 {
    // Wrapping arithmetic keeps defined behaviour even for a null/edge pointer.
    p.wrapping_add(4)
}
}

// ── `while (p < p_end)` · slirp/bootp.c:108-136 · chunk 2249 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (p < p_end) {
//             tag = p[0];
//             if (tag == 0) {
//                 p++;
//             } else if (tag == 255) {
//                 break;
//             } else {
//                 p++;
//                 if (p >= p_end)
//                     break;
//                 len = *p++;
//                 do{}while(0);
//     
//                 switch(tag) {
//                 case 53:
//                     if (len >= 1)
//                         *pmsg_type = p[0];
//                     break;
//                 case 50:
//                     if (len >= 4) {
//                         memcpy(&(preq_addr->s_addr), p, 4);
//                     }
//                     break;
//                 default:
//                     break;
//                 }
//                 p += len;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2249 {
// Port of the RFC1533/RFC2132 option-scanning loop from dhcp_decode().
// Scans the DHCP options area [p, p_end), recording the DHCP message type
// and the requested address when the corresponding tags are found.

#[no_mangle]
pub extern "C" fn dhcp_decode__part2249(
    p_in: *const u8,
    p_end: *const u8,
    pmsg_type: *mut core::ffi::c_int,
    preq_addr: *mut u8,
) {
    let mut p = p_in;
    unsafe {
        while (p as usize) < (p_end as usize) {
            let tag = *p as core::ffi::c_int;
            if tag == 0 {
                // RFC1533_PAD
                p = p.add(1);
            } else if tag == 255 {
                // RFC1533_END
                break;
            } else {
                p = p.add(1);
                if (p as usize) >= (p_end as usize) {
                    break;
                }
                let len = *p as core::ffi::c_int;
                p = p.add(1);

                match tag {
                    53 => {
                        // RFC2132_MSG_TYPE
                        if len >= 1 {
                            *pmsg_type = *p as core::ffi::c_int;
                        }
                    }
                    50 => {
                        // RFC2132_REQ_ADDR
                        if len >= 4 {
                            core::ptr::copy_nonoverlapping(p, preq_addr, 4);
                        }
                    }
                    _ => {}
                }
                p = p.wrapping_add(len as usize);
            }
        }
    }
}
}

// ── `if (*pmsg_type == 3 && preq_addr->s_addr == htonl(0L) && bp->bp_ciaddr.s_addr)` · slirp/bootp.c:137-140 · chunk 2250 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*pmsg_type == 3 && preq_addr->s_addr == htonl(0L) &&
//             bp->bp_ciaddr.s_addr) {
//             memcpy(&(preq_addr->s_addr), &bp->bp_ciaddr, 4);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2250 {
// Port of the tail of dhcp_decode (slirp/bootp.c):
// if (*pmsg_type == DHCPREQUEST && preq_addr->s_addr == htonl(0L) &&
//     bp->bp_ciaddr.s_addr)
//     memcpy(&(preq_addr->s_addr), &bp->bp_ciaddr, 4);

use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    // uint32_t htonl(uint32_t) __attribute__((const));
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn dhcp_decode__part2250(
    pmsg_type: c_int,
    ciaddr: u32,
    preq_addr: *mut InAddr,
) {
    unsafe {
        if preq_addr.is_null() {
            return;
        }
        // htonl(0L) is always 0, but keep the call for exact semantics.
        let zero = htonl(0);
        if pmsg_type == 3 && (*preq_addr).s_addr == zero && ciaddr != 0 {
            (*preq_addr).s_addr = ciaddr;
        }
    }
}
}

// ── `BOOTPClient *bc = ((void *)0) ; struct mbuf *m; struct bootp_t *rbp; struct sockaddr_in saddr,…` · slirp/bootp.c:145-154 · chunk 2252 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOTPClient *bc = 
//                          ((void *)0)
//                              ;
//         struct mbuf *m;
//         struct bootp_t *rbp;
//         struct sockaddr_in saddr, daddr;
//         struct in_addr preq_addr;
//         int dhcp_msg_type, val;
//         uint8_t *q;
//     
//     
//         dhcp_decode(bp, &dhcp_msg_type, &preq_addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2252 {
// Port of the TARGET part of bootp_reply(): the declarations plus the
// static dhcp_decode() helper (which has no linker symbol, so its logic is
// reproduced here as a private Rust fn).
#![allow(clippy::all)]

use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

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

#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct Bootp {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

const RFC1533_COOKIE: [u8; 4] = [99, 130, 83, 99];

#[inline]
fn htonl(x: u32) -> u32 {
    if cfg!(target_endian = "big") {
        x
    } else {
        x.swap_bytes()
    }
}

unsafe fn dhcp_decode(bp: *const Bootp, pmsg_type: *mut c_int, preq_addr: *mut InAddr) {
    let mut p: *const u8;
    let p_end: *const u8;

    *pmsg_type = 0;
    (*preq_addr).s_addr = htonl(0);

    p = core::ptr::addr_of!((*bp).bp_vend) as *const u8;
    p_end = p.wrapping_add(312);
    if core::ptr::read(p) != RFC1533_COOKIE[0]
        || core::ptr::read(p.add(1)) != RFC1533_COOKIE[1]
        || core::ptr::read(p.add(2)) != RFC1533_COOKIE[2]
        || core::ptr::read(p.add(3)) != RFC1533_COOKIE[3]
    {
        return;
    }
    p = p.wrapping_add(4);
    while (p as usize) < (p_end as usize) {
        let tag: c_int = core::ptr::read(p) as c_int;
        if tag == 0 {
            p = p.wrapping_add(1);
        } else if tag == 255 {
            break;
        } else {
            p = p.wrapping_add(1);
            if (p as usize) >= (p_end as usize) {
                break;
            }
            let len: c_int = core::ptr::read(p) as c_int;
            p = p.wrapping_add(1);
            match tag {
                53 => {
                    if len >= 1 {
                        *pmsg_type = core::ptr::read(p) as c_int;
                    }
                }
                50 => {
                    if len >= 4 {
                        core::ptr::copy_nonoverlapping(
                            p,
                            core::ptr::addr_of_mut!((*preq_addr).s_addr) as *mut u8,
                            4,
                        );
                    }
                }
                _ => {}
            }
            p = p.wrapping_add(len as usize);
        }
    }
    if *pmsg_type == 3
        && (*preq_addr).s_addr == htonl(0)
        && (*bp).bp_ciaddr.s_addr != 0
    {
        core::ptr::copy_nonoverlapping(
            core::ptr::addr_of!((*bp).bp_ciaddr.s_addr) as *const u8,
            core::ptr::addr_of_mut!((*preq_addr).s_addr) as *mut u8,
            4,
        );
    }
}

#[no_mangle]
pub extern "C" fn bootp_reply__part2252(
    bp: *const Bootp,
    pmsg_type: *mut c_int,
    preq_addr: *mut InAddr,
) {
    unsafe {
        dhcp_decode(bp, pmsg_type, preq_addr);
    }
}
}

// ── `do … while (0)` · slirp/bootp.c:155-155 · chunk 2253 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do{}while(0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2253 {
// Port of the DPRINTF("bootp packet op=%d msgtype=%d", ...) statement in
// bootp_reply. In this configuration DPRINTF expands to `do{}while(0)`, so the
// statement has no observable effect and reads no values from its context.
#[no_mangle]
pub extern "C" fn bootp_reply__part2253() {}
}

// ── `if (preq_addr.s_addr != htonl(0L)) … else` · slirp/bootp.c:156-159 · chunk 2254 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (preq_addr.s_addr != htonl(0L))
//             do{}while(0);
//         else
//             do{}while(0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2254 {
// Port of bootp_reply()'s requested-address / discovery check
// (slirp/bootp.c, the "if (preq_addr.s_addr != htonl(0L)) ... else ..." block).
// Both branches only contained compile-time-disabled debug hooks, so the sole
// effect of the TARGET is to evaluate the comparison.

extern "C" {
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn bootp_reply__part2254(preq_addr_s_addr: u32) {
    unsafe {
        if preq_addr_s_addr != htonl(0) {
            // DHCPREQUEST for a specific address: debug hook is compiled out.
        } else {
            // DHCPDISCOVER: debug hook is compiled out.
        }
    }
}
}

// ── `if (dhcp_msg_type == 0)` · slirp/bootp.c:161-162 · chunk 2255 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dhcp_msg_type == 0)
//             dhcp_msg_type = 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2255 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn bootp_reply__part2255(dhcp_msg_type: c_int) -> c_int {
    let mut dhcp_msg_type = dhcp_msg_type;
    if dhcp_msg_type == 0 {
        dhcp_msg_type = 3;
    }
    dhcp_msg_type
}
}

// ── `if (dhcp_msg_type != 1 && dhcp_msg_type != 3)` · slirp/bootp.c:164-166 · chunk 2256 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dhcp_msg_type != 1 &&
//             dhcp_msg_type != 3)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2256 {
// Check whether the DHCP message type is one of REQUEST(1) or BOOTREQUEST(3).
// Returns 1 when the caller must return early, 0 otherwise.
#[no_mangle]
pub extern "C" fn bootp_reply__part2256(dhcp_msg_type: core::ffi::c_int) -> core::ffi::c_int {
    if dhcp_msg_type != 1 && dhcp_msg_type != 3 {
        1
    } else {
        0
    }
}
}

// ── `memcpy(slirp->client_ethaddr, bp->bp_hwaddr, 6); m = m_get(slirp);` · slirp/bootp.c:168-170 · chunk 2257 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(slirp->client_ethaddr, bp->bp_hwaddr, 6);
//     
//         m = m_get(slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2257 {
// Ported piece of bootp_reply(): copy client hw address and allocate an mbuf.
use core::ffi::c_void;

// Opaque mirror of `struct mbuf` (only used through pointers here).
#[repr(C)]
pub struct Mbuf {
    pub _opaque: [u8; 0],
}

// Opaque mirror of `struct Slirp` (only used through pointers here).
#[repr(C)]
pub struct Slirp {
    pub _opaque: [u8; 0],
}

// Mirror of `struct ip` (packed, 20 bytes) - not accessed field-wise here.
#[repr(C)]
pub struct Ip {
    pub raw: [u8; 20],
}

// Mirror of `struct udphdr` (8 bytes) - not accessed field-wise here.
#[repr(C)]
pub struct Udphdr {
    pub raw: [u8; 8],
}

// Mirror of `struct in_addr`.
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct bootp_t` as seen through a pointer.
#[repr(C)]
pub struct BootpT {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

extern "C" {
    // struct mbuf *m_get(Slirp *);
    fn m_get(slirp: *mut Slirp) -> *mut Mbuf;
}

// Byte offset of Slirp::client_ethaddr:
// vnetwork_addr, vnetwork_mask, vhost_addr, vdhcp_startaddr, vnameserver_addr
// (5 * sizeof(struct in_addr)).
const CLIENT_ETHADDR_OFFSET: usize = 20;

#[no_mangle]
pub extern "C" fn bootp_reply__part2257(
    slirp: *mut Slirp,
    bp: *const BootpT,
) -> *mut Mbuf {
    unsafe {
        let dst = (slirp as *mut c_void as *mut u8).add(CLIENT_ETHADDR_OFFSET);
        let src = (*bp).bp_hwaddr.as_ptr();
        core::ptr::copy_nonoverlapping(src, dst, 6);
        m_get(slirp)
    }
}
}

// ── `if (!m)` · slirp/bootp.c:171-173 · chunk 2258 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!m) {
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2258 {
use core::ffi::{c_int, c_void};

// Port of the `if (!m) { return; }` check in bootp_reply.
// Returns nonzero when the caller should return (i.e. m was NULL).
#[no_mangle]
pub extern "C" fn bootp_reply__part2258(m: *mut c_void) -> c_int {
    if m.is_null() {
        1
    } else {
        0
    }
}
}

// ── `m->m_hdr.mh_data += (2 + 14 + 40); rbp = (struct bootp_t *)m->m_hdr.mh_data; m->m_hdr.mh_data +…` · slirp/bootp.c:174-177 · chunk 2259 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_data += (2 + 14 + 40);
//         rbp = (struct bootp_t *)m->m_hdr.mh_data;
//         m->m_hdr.mh_data += sizeof(struct udpiphdr);
//         memset(rbp, 0, sizeof(struct bootp_t));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2259 {
// Port of bootp_reply lines 174-177: bump mbuf data pointer by link+ip header
// sizes, remember the bootp header pointer, bump past the udp/ip header and
// zero the bootp header.

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct BootpT {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

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

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn bootp_reply__part2259(m: *mut Mbuf) -> *mut BootpT {
    if m.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        let mh = &mut (*m).m_hdr;
        // m->m_hdr.mh_data += (2 + 14 + 40);
        let mut data = mh.mh_data.add(2 + 14 + 40);
        // rbp = (struct bootp_t *)m->m_hdr.mh_data;
        let rbp = data as *mut BootpT;
        // m->m_hdr.mh_data += sizeof(struct udpiphdr);
        data = data.add(core::mem::size_of::<Udphdr>() + 20);
        mh.mh_data = data;
        // memset(rbp, 0, sizeof(struct bootp_t));
        core::ptr::write_bytes(rbp as *mut u8, 0, core::mem::size_of::<BootpT>());
        rbp
    }
}
}

// ── `if (dhcp_msg_type == 1) … else` · slirp/bootp.c:179-210 · chunk 2260 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (dhcp_msg_type == 1) {
//             if (preq_addr.s_addr != htonl(0L)) {
//                 bc = request_addr(slirp, &preq_addr, slirp->client_ethaddr);
//                 if (bc) {
//                     daddr.sin_addr = preq_addr;
//                 }
//             }
//             if (!bc) {
//              new_addr:
//                 bc = get_new_addr(slirp, &daddr.sin_addr, slirp->client_ethaddr);
//                 if (!bc) {
//                     do{}while(0);
//                     return;
//                 }
//             }
//             memcpy(bc->macaddr, slirp->client_ethaddr, 6);
//         } else if (preq_addr.s_addr != htonl(0L)) {
//             bc = request_addr(slirp, &preq_addr, slirp->client_ethaddr);
//             if (bc) {
//                 daddr.sin_addr = preq_addr;
//                 memcpy(bc->macaddr, slirp->client_ethaddr, 6);
//             } else {
//                 daddr.sin_addr.s_addr = 0;
//             }
//         } else {
//             bc = find_addr(slirp, &daddr.sin_addr, bp->bp_hwaddr);
//             if (!bc) {
//     
//     
//                 goto new_addr;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2260 {
// Ported piece of slirp's bootp_reply(): DHCP address selection.
//
// The C side hands over the few objects the original code touched (the
// preq_addr / daddr.sin_addr in_addr values, slirp->vdhcp_startaddr,
// slirp->bootp_clients, the client MAC and bp->bp_hwaddr), so no large
// struct layout mirrors are needed here.
//
// The static helpers request_addr / get_new_addr / find_addr have no linker
// symbol, so their logic is ported in as private Rust fns.

use core::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

const NB_BOOTP_CLIENTS: i32 = 16;

// Equivalent of memcmp(a, b, 6) == 0
unsafe fn mac_equal(a: *const u8, b: *const u8) -> bool {
    let mut i: usize = 0;
    while i < 6 {
        if *a.add(i) != *b.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

// Equivalent of memcpy(dst, src, 6)
unsafe fn copy_mac(dst: *mut u8, src: *const u8) {
    let mut i: usize = 0;
    while i < 6 {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
}

// static BOOTPClient *request_addr(Slirp *slirp, const struct in_addr *paddr,
//                                  const uint8_t *macaddr)
unsafe fn request_addr(
    vdhcp_startaddr: *const InAddr,
    bootp_clients: *mut BootpClient,
    paddr: *const InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let req_addr = u32::from_be((*paddr).s_addr);
    let dhcp_addr = u32::from_be((*vdhcp_startaddr).s_addr);

    if req_addr >= dhcp_addr && req_addr < dhcp_addr.wrapping_add(NB_BOOTP_CLIENTS as u32) {
        let idx = req_addr.wrapping_sub(dhcp_addr);
        let bc = bootp_clients.add(idx as usize);
        if (*bc).allocated == 0 || mac_equal(macaddr, (*bc).macaddr.as_ptr()) {
            (*bc).allocated = 1;
            return bc;
        }
    }
    core::ptr::null_mut()
}

// static BOOTPClient *get_new_addr(Slirp *slirp, struct in_addr *paddr,
//                                  const uint8_t *macaddr)
unsafe fn get_new_addr(
    vdhcp_startaddr: *const InAddr,
    bootp_clients: *mut BootpClient,
    paddr: *mut InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let mut i: i32 = 0;
    while i < NB_BOOTP_CLIENTS {
        let bc = bootp_clients.add(i as usize);
        if (*bc).allocated == 0 || mac_equal(macaddr, (*bc).macaddr.as_ptr()) {
            (*bc).allocated = 1;
            (*paddr).s_addr = (*vdhcp_startaddr)
                .s_addr
                .wrapping_add((i as u32).to_be());
            return bc;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

// static BOOTPClient *find_addr(Slirp *slirp, struct in_addr *paddr,
//                               const uint8_t *macaddr)
unsafe fn find_addr(
    vdhcp_startaddr: *const InAddr,
    bootp_clients: *mut BootpClient,
    paddr: *mut InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let mut i: i32 = 0;
    while i < NB_BOOTP_CLIENTS {
        let bc = bootp_clients.add(i as usize);
        if mac_equal(macaddr, (*bc).macaddr.as_ptr()) {
            (*bc).allocated = 1;
            (*paddr).s_addr = (*vdhcp_startaddr)
                .s_addr
                .wrapping_add((i as u32).to_be());
            return bc;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

/// Returns 1 when the original code returned from bootp_reply() (no address
/// left), 0 otherwise.  The selected client record is stored through bc_out.
#[no_mangle]
pub extern "C" fn bootp_reply__part2260(
    dhcp_msg_type: c_int,
    preq_addr: *const InAddr,
    daddr_addr: *mut InAddr,
    vdhcp_startaddr: *const InAddr,
    bootp_clients: *mut BootpClient,
    client_ethaddr: *const u8,
    bp_hwaddr: *const u8,
    bc_out: *mut *mut BootpClient,
) -> c_int {
    unsafe {
        let mut bc: *mut BootpClient = core::ptr::null_mut();
        let mut do_return: c_int = 0;
        let preq_nonzero = (*preq_addr).s_addr != 0u32;

        if dhcp_msg_type == 1 {
            if preq_nonzero {
                bc = request_addr(vdhcp_startaddr, bootp_clients, preq_addr, client_ethaddr);
                if !bc.is_null() {
                    *daddr_addr = *preq_addr;
                }
            }
            if bc.is_null() {
                // new_addr:
                bc = get_new_addr(vdhcp_startaddr, bootp_clients, daddr_addr, client_ethaddr);
                if bc.is_null() {
                    do_return = 1;
                }
            }
            if do_return == 0 {
                copy_mac((*bc).macaddr.as_mut_ptr(), client_ethaddr);
            }
        } else if preq_nonzero {
            bc = request_addr(vdhcp_startaddr, bootp_clients, preq_addr, client_ethaddr);
            if !bc.is_null() {
                *daddr_addr = *preq_addr;
                copy_mac((*bc).macaddr.as_mut_ptr(), client_ethaddr);
            } else {
                (*daddr_addr).s_addr = 0;
            }
        } else {
            bc = find_addr(vdhcp_startaddr, bootp_clients, daddr_addr, bp_hwaddr);
            if bc.is_null() {
                // goto new_addr;
                bc = get_new_addr(vdhcp_startaddr, bootp_clients, daddr_addr, client_ethaddr);
                if bc.is_null() {
                    do_return = 1;
                }
            }
            if do_return == 0 {
                copy_mac((*bc).macaddr.as_mut_ptr(), client_ethaddr);
            }
        }

        *bc_out = bc;
        do_return
    }
}
}

// ── `saddr.sin_addr = slirp->vhost_addr; saddr.sin_port = htons(67); daddr.sin_port = htons(68); rbp…` · slirp/bootp.c:212-228 · chunk 2261 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     saddr.sin_addr = slirp->vhost_addr;
//         saddr.sin_port = htons(67);
//     
//         daddr.sin_port = htons(68);
//     
//         rbp->bp_op = 2;
//         rbp->bp_xid = bp->bp_xid;
//         rbp->bp_htype = 1;
//         rbp->bp_hlen = 6;
//         memcpy(rbp->bp_hwaddr, bp->bp_hwaddr, 6);
//     
//         rbp->bp_yiaddr = daddr.sin_addr;
//         rbp->bp_siaddr = saddr.sin_addr;
//     
//         q = rbp->bp_vend;
//         memcpy(q, rfc1533_cookie, 4);
//         q += 4;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2261 {
// Rust port of part of bootp_reply() from slirp/bootp.c
// (reply header setup: addresses, ports, BOOTP fields and RFC1533 cookie)

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip is __attribute__((packed)); the leading `u_int ip_hl:4, ip_v:4`
// bitfield pair occupies a single byte.
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

#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct BootpT {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

// static const uint8_t rfc1533_cookie[] = { 99, 130, 83, 99 };
const RFC1533_COOKIE: [u8; 4] = [99, 130, 83, 99];

#[no_mangle]
pub unsafe extern "C" fn bootp_reply__part2261(
    saddr_addr: *mut u32,
    saddr_port: *mut u16,
    daddr_addr: u32,
    daddr_port: *mut u16,
    rbp: *mut BootpT,
    bp: *const BootpT,
    vhost_addr: u32,
    q: *mut *mut u8,
) {
    if saddr_addr.is_null()
        || saddr_port.is_null()
        || daddr_port.is_null()
        || rbp.is_null()
        || bp.is_null()
        || q.is_null()
    {
        return;
    }

    *saddr_addr = vhost_addr;
    *saddr_port = 67u16.to_be();

    *daddr_port = 68u16.to_be();

    let rb = &mut *rbp;
    rb.bp_op = 2;
    rb.bp_xid = (*bp).bp_xid;
    rb.bp_htype = 1;
    rb.bp_hlen = 6;
    core::ptr::copy_nonoverlapping((*bp).bp_hwaddr.as_ptr(), rb.bp_hwaddr.as_mut_ptr(), 6);

    rb.bp_yiaddr = InAddr { s_addr: daddr_addr };
    rb.bp_siaddr = InAddr { s_addr: vhost_addr };

    let dst = rb.bp_vend.as_mut_ptr();
    core::ptr::copy_nonoverlapping(RFC1533_COOKIE.as_ptr(), dst, 4);
    *q = dst.add(4);
}
}

// ── `if (bc) … else` · slirp/bootp.c:230-297 · chunk 2262 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bc) {
//             do{}while(0)
//     
//                                                  ;
//     
//             if (dhcp_msg_type == 1) {
//                 *q++ = 53;
//                 *q++ = 1;
//                 *q++ = 2;
//             } else {
//                 *q++ = 53;
//                 *q++ = 1;
//                 *q++ = 5;
//             }
//     
//             if (slirp->bootp_filename)
//                 snprintf((char *)rbp->bp_file, sizeof(rbp->bp_file), "%s",
//                          slirp->bootp_filename);
//     
//             *q++ = 54;
//             *q++ = 4;
//             memcpy(q, &saddr.sin_addr, 4);
//             q += 4;
//     
//             *q++ = 1;
//             *q++ = 4;
//             memcpy(q, &slirp->vnetwork_mask, 4);
//             q += 4;
//     
//             if (!slirp->restricted) {
//                 *q++ = 3;
//                 *q++ = 4;
//                 memcpy(q, &saddr.sin_addr, 4);
//                 q += 4;
//     
//                 *q++ = 6;
//                 *q++ = 4;
//                 memcpy(q, &slirp->vnameserver_addr, 4);
//                 q += 4;
//             }
//     
//             *q++ = 51;
//             *q++ = 4;
//             val = htonl((24 * 3600));
//             memcpy(q, &val, 4);
//             q += 4;
//     
//             if (*slirp->client_hostname) {
//                 val = strlen(slirp->client_hostname);
//                 *q++ = 12;
//                 *q++ = val;
//                 memcpy(q, slirp->client_hostname, val);
//                 q += val;
//             }
//         } else {
//             static const char nak_msg[] = "requested address not available";
//     
//             do{}while(0);
//     
//             *q++ = 53;
//             *q++ = 1;
//             *q++ = 6;
//     
//             *q++ = 56;
//             *q++ = sizeof(nak_msg) - 1;
//             memcpy(q, nak_msg, sizeof(nak_msg) - 1);
//             q += sizeof(nak_msg) - 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2262 {
// Port of the DHCP option-encoding block of slirp's bootp_reply().
// Writes DHCP options (message type, server id, mask, router, dns, lease time,
// hostname) into rbp->bp_vend through q, or a NAK when no client address was
// found.  Returns the advanced q pointer.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn htonl(hostlong: u32) -> u32;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// static const char nak_msg[] = "requested address not available";
static NAK_MSG: [u8; 32] = *b"requested address not available\0";

#[no_mangle]
pub extern "C" fn bootp_reply__part2262(
    bc_present: c_int,
    dhcp_msg_type: c_int,
    saddr_addr: u32,
    vnetwork_mask: u32,
    vnameserver_addr: u32,
    restricted: c_int,
    client_hostname: *const c_char,
    bootp_filename: *const c_char,
    bp_file: *mut c_char,
    bp_file_size: usize,
    val: *mut c_int,
    q_in: *mut u8,
) -> *mut u8 {
    unsafe {
        let mut q: *mut u8 = q_in;

        if bc_present != 0 {
            if dhcp_msg_type == 1 {
                *q = 53;
                q = q.add(1);
                *q = 1;
                q = q.add(1);
                *q = 2;
                q = q.add(1);
            } else {
                *q = 53;
                q = q.add(1);
                *q = 1;
                q = q.add(1);
                *q = 5;
                q = q.add(1);
            }

            if !bootp_filename.is_null() {
                snprintf(
                    bp_file,
                    bp_file_size,
                    b"%s\0".as_ptr() as *const c_char,
                    bootp_filename,
                );
            }

            // server identifier
            *q = 54;
            q = q.add(1);
            *q = 4;
            q = q.add(1);
            memcpy(
                q as *mut c_void,
                &saddr_addr as *const u32 as *const c_void,
                4,
            );
            q = q.add(4);

            // subnet mask
            *q = 1;
            q = q.add(1);
            *q = 4;
            q = q.add(1);
            memcpy(
                q as *mut c_void,
                &vnetwork_mask as *const u32 as *const c_void,
                4,
            );
            q = q.add(4);

            if restricted == 0 {
                // router
                *q = 3;
                q = q.add(1);
                *q = 4;
                q = q.add(1);
                memcpy(
                    q as *mut c_void,
                    &saddr_addr as *const u32 as *const c_void,
                    4,
                );
                q = q.add(4);

                // domain name server
                *q = 6;
                q = q.add(1);
                *q = 4;
                q = q.add(1);
                memcpy(
                    q as *mut c_void,
                    &vnameserver_addr as *const u32 as *const c_void,
                    4,
                );
                q = q.add(4);
            }

            // lease time
            *q = 51;
            q = q.add(1);
            *q = 4;
            q = q.add(1);
            let lease: u32 = htonl((24 * 3600) as u32);
            *val = lease as c_int;
            memcpy(q as *mut c_void, &lease as *const u32 as *const c_void, 4);
            q = q.add(4);

            if *client_hostname != 0 {
                let host_len: c_int = strlen(client_hostname) as c_int;
                *val = host_len;
                *q = 12;
                q = q.add(1);
                *q = host_len as u8;
                q = q.add(1);
                memcpy(
                    q as *mut c_void,
                    client_hostname as *const c_void,
                    host_len as usize,
                );
                q = q.add(host_len as usize);
            }
        } else {
            *q = 53;
            q = q.add(1);
            *q = 1;
            q = q.add(1);
            *q = 6;
            q = q.add(1);

            *q = 56;
            q = q.add(1);
            *q = (NAK_MSG.len() - 1) as u8;
            q = q.add(1);
            let nak_len: usize = NAK_MSG.len() - 1;
            memcpy(
                q as *mut c_void,
                NAK_MSG.as_ptr() as *const c_void,
                nak_len,
            );
            q = q.add(nak_len);
        }

        q
    }
}
}

// ── `*q = 255; daddr.sin_addr.s_addr = 0xffffffffu; m->m_hdr.mh_len = sizeof(struct bootp_t) - sizeo…` · slirp/bootp.c:298-304 · chunk 2263 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q = 255;
//     
//         daddr.sin_addr.s_addr = 0xffffffffu;
//     
//         m->m_hdr.mh_len = sizeof(struct bootp_t) -
//             sizeof(struct ip) - sizeof(struct udphdr);
//         udp_output2(
//                    ((void *)0)
//                        , m, &saddr, &daddr, 0x10);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2263 {
// Ported tail of slirp's bootp_reply (slirp/bootp.c lines 298-304).
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

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
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C, packed)]
pub struct Ip {
    pub ip_v_hl: u8,
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
pub struct Bootp {
    pub ip: Ip,
    pub udp: Udphdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

#[repr(C)]
pub struct MHeader {
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
    pub m_hdr: MHeader,
}

// sizeof(struct bootp_t) - sizeof(struct ip) - sizeof(struct udphdr)
const BOOTP_LEN: c_int =
    (core::mem::size_of::<Bootp>() - core::mem::size_of::<Ip>() - core::mem::size_of::<Udphdr>())
        as c_int;
const _: () = assert!(BOOTP_LEN == 548);

extern "C" {
    fn udp_output2(
        so: *mut c_void,
        m: *mut Mbuf,
        saddr: *mut SockaddrIn,
        daddr: *mut SockaddrIn,
        iptos: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn bootp_reply__part2263(
    q: *mut u8,
    m: *mut Mbuf,
    saddr: *mut SockaddrIn,
    daddr: *mut SockaddrIn,
) {
    unsafe {
        *q = 255;

        (*daddr).sin_addr.s_addr = 0xffff_ffffu32;

        (*m).m_hdr.mh_len = BOOTP_LEN;

        udp_output2(core::ptr::null_mut(), m, saddr, daddr, 0x10);
    }
}
}

// ── `if (bp->bp_op == 1)` · slirp/bootp.c:311-313 · chunk 2265 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bp->bp_op == 1) {
//             bootp_reply(m->slirp, bp);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2265 {
// Rust port of bootp_input's dispatch (slirp/bootp.c) together with the static
// helpers it depends on (bootp_reply, dhcp_decode, get_new_addr, request_addr,
// find_addr), which have no linker symbols in the C program.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn htonl(x: u32) -> u32;
    fn htons(x: u16) -> u16;
    fn ntohl(x: u32) -> u32;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn m_get(slirp: *mut Slirp) -> *mut Mbuf;
    fn udp_output2(
        so: *mut Socket,
        m: *mut Mbuf,
        saddr: *mut SockAddrIn,
        daddr: *mut SockAddrIn,
        iptos: c_int,
    ) -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

// struct ip (packed): "u_int ip_hl:4, ip_v:4" occupies a single byte.
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UdpHdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct BootpT {
    pub ip: Ip,
    pub udp: UdpHdr,
    pub bp_op: u8,
    pub bp_htype: u8,
    pub bp_hlen: u8,
    pub bp_hops: u8,
    pub bp_xid: u32,
    pub bp_secs: u16,
    pub unused: u16,
    pub bp_ciaddr: InAddr,
    pub bp_yiaddr: InAddr,
    pub bp_siaddr: InAddr,
    pub bp_giaddr: InAddr,
    pub bp_hwaddr: [u8; 16],
    pub bp_sname: [u8; 64],
    pub bp_file: [u8; 128],
    pub bp_vend: [u8; 312],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ipq (packed)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ipq {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
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

// union M_dat { char m_dat_[1]; char *m_ext_; }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mdat {
    pub m_ext: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: Mdat,
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
    pub client_hostname: [u8; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut c_void,
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

const SIZE_UDPIPHDR: usize = 28; // sizeof(struct ipovly) + sizeof(struct udphdr)

static RFC1533_COOKIE: [u8; 4] = [99, 130, 83, 99];
static NAK_MSG: [u8; 31] = *b"requested address not available";

unsafe fn bytes_eq(a: *const u8, b: *const u8, n: usize) -> bool {
    let mut i: usize = 0;
    while i < n {
        if *a.add(i) != *b.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn copy_bytes(dst: *mut u8, src: *const u8, n: usize) {
    if n != 0 {
        core::ptr::copy_nonoverlapping(src, dst, n);
    }
}

unsafe fn get_new_addr(
    slirp: *mut Slirp,
    paddr: *mut InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let mut i: usize = 0;
    while i < 16 {
        let bc: *mut BootpClient = (*slirp).bootp_clients.as_mut_ptr().add(i);
        if (*bc).allocated == 0 || bytes_eq(macaddr, (*bc).macaddr.as_ptr(), 6) {
            // found:
            (*bc).allocated = 1;
            (*paddr).s_addr = (*slirp).vdhcp_startaddr.s_addr.wrapping_add(htonl(i as u32));
            return bc;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn request_addr(
    slirp: *mut Slirp,
    paddr: *const InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let req_addr: u32 = ntohl((*paddr).s_addr);
    let dhcp_addr: u32 = ntohl((*slirp).vdhcp_startaddr.s_addr);

    if req_addr >= dhcp_addr && req_addr < dhcp_addr.wrapping_add(16) {
        let idx: u32 = req_addr - dhcp_addr;
        let bc: *mut BootpClient = (*slirp).bootp_clients.as_mut_ptr().add(idx as usize);
        if (*bc).allocated == 0 || bytes_eq(macaddr, (*bc).macaddr.as_ptr(), 6) {
            (*bc).allocated = 1;
            return bc;
        }
    }
    core::ptr::null_mut()
}

unsafe fn find_addr(
    slirp: *mut Slirp,
    paddr: *mut InAddr,
    macaddr: *const u8,
) -> *mut BootpClient {
    let mut i: usize = 0;
    while i < 16 {
        let bc: *mut BootpClient = (*slirp).bootp_clients.as_mut_ptr().add(i);
        if bytes_eq(macaddr, (*bc).macaddr.as_ptr(), 6) {
            // found:
            (*bc).allocated = 1;
            (*paddr).s_addr = (*slirp).vdhcp_startaddr.s_addr.wrapping_add(htonl(i as u32));
            return bc;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

unsafe fn dhcp_decode(bp: *const BootpT, pmsg_type: *mut c_int, preq_addr: *mut InAddr) {
    *pmsg_type = 0;
    (*preq_addr).s_addr = htonl(0);

    let mut p: *const u8 = (*bp).bp_vend.as_ptr();
    let p_end: *const u8 = p.add(312);
    if !bytes_eq(p, RFC1533_COOKIE.as_ptr(), 4) {
        return;
    }
    p = p.add(4);
    while (p as usize) < (p_end as usize) {
        let tag: c_int = *p as c_int;
        if tag == 0 {
            p = p.add(1);
        } else if tag == 255 {
            break;
        } else {
            p = p.add(1);
            if (p as usize) >= (p_end as usize) {
                break;
            }
            let len: c_int = *p as c_int;
            p = p.add(1);

            match tag {
                53 => {
                    if len >= 1 {
                        *pmsg_type = *p as c_int;
                    }
                }
                50 => {
                    if len >= 4 {
                        copy_bytes(
                            &mut (*preq_addr).s_addr as *mut u32 as *mut u8,
                            p,
                            4,
                        );
                    }
                }
                _ => {}
            }
            p = p.add(len as usize);
        }
    }
    if *pmsg_type == 3
        && (*preq_addr).s_addr == htonl(0)
        && (*bp).bp_ciaddr.s_addr != 0
    {
        copy_bytes(
            &mut (*preq_addr).s_addr as *mut u32 as *mut u8,
            &(*bp).bp_ciaddr as *const InAddr as *const u8,
            4,
        );
    }
}

unsafe fn bootp_reply(slirp: *mut Slirp, bp: *const BootpT) {
    let mut bc: *mut BootpClient = core::ptr::null_mut();
    let mut dhcp_msg_type: c_int = 0;
    let mut preq_addr: InAddr = InAddr { s_addr: 0 };

    dhcp_decode(bp, &mut dhcp_msg_type, &mut preq_addr);

    if dhcp_msg_type == 0 {
        dhcp_msg_type = 3;
    }
    if dhcp_msg_type != 1 && dhcp_msg_type != 3 {
        return;
    }

    copy_bytes(
        (*slirp).client_ethaddr.as_mut_ptr(),
        (*bp).bp_hwaddr.as_ptr(),
        6,
    );

    let m: *mut Mbuf = m_get(slirp);
    if m.is_null() {
        return;
    }
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(2 + 14 + 40);
    let rbp: *mut BootpT = (*m).m_hdr.mh_data as *mut BootpT;
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(SIZE_UDPIPHDR);
    core::ptr::write_bytes(rbp as *mut u8, 0, core::mem::size_of::<BootpT>());

    let mut saddr: SockAddrIn = core::mem::zeroed();
    let mut daddr: SockAddrIn = core::mem::zeroed();

    if dhcp_msg_type == 1 {
        if preq_addr.s_addr != htonl(0) {
            bc = request_addr(slirp, &preq_addr, (*slirp).client_ethaddr.as_ptr());
            if !bc.is_null() {
                daddr.sin_addr = preq_addr;
            }
        }
        if bc.is_null() {
            // new_addr:
            bc = get_new_addr(slirp, &mut daddr.sin_addr, (*slirp).client_ethaddr.as_ptr());
            if bc.is_null() {
                return;
            }
        }
        copy_bytes(
            (*bc).macaddr.as_mut_ptr(),
            (*slirp).client_ethaddr.as_ptr(),
            6,
        );
    } else if preq_addr.s_addr != htonl(0) {
        bc = request_addr(slirp, &preq_addr, (*slirp).client_ethaddr.as_ptr());
        if !bc.is_null() {
            daddr.sin_addr = preq_addr;
            copy_bytes(
                (*bc).macaddr.as_mut_ptr(),
                (*slirp).client_ethaddr.as_ptr(),
                6,
            );
        } else {
            daddr.sin_addr.s_addr = 0;
        }
    } else {
        bc = find_addr(slirp, &mut daddr.sin_addr, (*bp).bp_hwaddr.as_ptr());
        if bc.is_null() {
            // goto new_addr
            bc = get_new_addr(slirp, &mut daddr.sin_addr, (*slirp).client_ethaddr.as_ptr());
            if bc.is_null() {
                return;
            }
            copy_bytes(
                (*bc).macaddr.as_mut_ptr(),
                (*slirp).client_ethaddr.as_ptr(),
                6,
            );
        }
    }

    saddr.sin_addr = (*slirp).vhost_addr;
    saddr.sin_port = htons(67);
    daddr.sin_port = htons(68);

    (*rbp).bp_op = 2;
    (*rbp).bp_xid = (*bp).bp_xid;
    (*rbp).bp_htype = 1;
    (*rbp).bp_hlen = 6;
    copy_bytes((*rbp).bp_hwaddr.as_mut_ptr(), (*bp).bp_hwaddr.as_ptr(), 6);

    (*rbp).bp_yiaddr = daddr.sin_addr;
    (*rbp).bp_siaddr = saddr.sin_addr;

    let mut q: *mut u8 = (*rbp).bp_vend.as_mut_ptr();
    copy_bytes(q, RFC1533_COOKIE.as_ptr(), 4);
    q = q.add(4);

    let mut val: c_int = 0;

    if !bc.is_null() {
        if dhcp_msg_type == 1 {
            *q = 53;
            q = q.add(1);
            *q = 1;
            q = q.add(1);
            *q = 2;
            q = q.add(1);
        } else {
            *q = 53;
            q = q.add(1);
            *q = 1;
            q = q.add(1);
            *q = 5;
            q = q.add(1);
        }

        if !(*slirp).bootp_filename.is_null() {
            snprintf(
                (*rbp).bp_file.as_mut_ptr() as *mut c_char,
                128,
                b"%s\0".as_ptr() as *const c_char,
                (*slirp).bootp_filename,
            );
        }

        *q = 54;
        q = q.add(1);
        *q = 4;
        q = q.add(1);
        copy_bytes(q, &saddr.sin_addr as *const InAddr as *const u8, 4);
        q = q.add(4);

        *q = 1;
        q = q.add(1);
        *q = 4;
        q = q.add(1);
        copy_bytes(
            q,
            &(*slirp).vnetwork_mask as *const InAddr as *const u8,
            4,
        );
        q = q.add(4);

        if (*slirp).restricted == 0 {
            *q = 3;
            q = q.add(1);
            *q = 4;
            q = q.add(1);
            copy_bytes(q, &saddr.sin_addr as *const InAddr as *const u8, 4);
            q = q.add(4);

            *q = 6;
            q = q.add(1);
            *q = 4;
            q = q.add(1);
            copy_bytes(
                q,
                &(*slirp).vnameserver_addr as *const InAddr as *const u8,
                4,
            );
            q = q.add(4);
        }

        *q = 51;
        q = q.add(1);
        *q = 4;
        q = q.add(1);
        val = htonl(24 * 3600) as c_int;
        copy_bytes(q, &val as *const c_int as *const u8, 4);
        q = q.add(4);

        if *(*slirp).client_hostname.as_ptr() != 0 {
            val = strlen((*slirp).client_hostname.as_ptr() as *const c_char) as c_int;
            *q = 12;
            q = q.add(1);
            *q = val as u8;
            q = q.add(1);
            copy_bytes(
                q,
                (*slirp).client_hostname.as_ptr(),
                val as usize,
            );
            q = q.add(val as usize);
        }
    } else {
        *q = 53;
        q = q.add(1);
        *q = 1;
        q = q.add(1);
        *q = 6;
        q = q.add(1);

        *q = 56;
        q = q.add(1);
        *q = (NAK_MSG.len()) as u8;
        q = q.add(1);
        copy_bytes(q, NAK_MSG.as_ptr(), NAK_MSG.len());
        q = q.add(NAK_MSG.len());
    }
    *q = 255;

    daddr.sin_addr.s_addr = 0xffff_ffffu32;

    (*m).m_hdr.mh_len = (core::mem::size_of::<BootpT>() - 20 - 8) as c_int;
    udp_output2(
        core::ptr::null_mut(),
        m,
        &mut saddr,
        &mut daddr,
        0x10,
    );
}

#[no_mangle]
pub extern "C" fn bootp_input__part2265(m: *mut Mbuf) {
    unsafe {
        let bp: *mut BootpT = (*m).m_hdr.mh_data as *mut BootpT;
        if (*bp).bp_op == 1 {
            bootp_reply((*m).slirp, bp);
        }
    }
}
}

