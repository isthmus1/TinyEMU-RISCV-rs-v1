//! From `slirp/udp.c`.

// ── `slirp->udb.so_next = slirp->udb.so_prev = &slirp->udb; slirp->udp_last_so = &slirp->udb;` · slirp/udp.c:49-50 · chunk 2869 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->udb.so_next = slirp->udb.so_prev = &slirp->udb;
//         slirp->udp_last_so = &slirp->udb;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2869 {
// Port of the udp_init TARGET: initialise the UDP socket list head.
// Mirrors only the leading fields of `struct socket` that are accessed.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
}

#[no_mangle]
pub extern "C" fn udp_init__part2869(udb: *mut Socket, udp_last_so: *mut *mut Socket) {
    if udb.is_null() || udp_last_so.is_null() {
        return;
    }
    unsafe {
        (*udb).so_prev = udb;
        (*udb).so_next = udb;
        *udp_last_so = udb;
    }
}
}

// ── `udp_input` · slirp/udp.c:56-226 · chunk 2870 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     udp_input(register struct mbuf *m, int iphlen)
//     {
//      Slirp *slirp = m->slirp;
//      register struct ip *ip;
//      register struct udphdr *uh;
//      int len;
//      struct ip save_ip;
//      struct socket *so;
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
//     
//      if(iphlen > sizeof(struct ip)) {
//       ip_stripoptions(m, (struct mbuf *)0);
//       iphlen = sizeof(struct ip);
//      }
//     
//     
//     
//     
//      ip = ((struct ip *)(m)->m_hdr.mh_data);
//      uh = (struct udphdr *)((caddr_t)ip + iphlen);
//     
//     
//     
//     
//     
//      len = ntohs((uint16_t)uh->uh_ulen);
//     
//      if (ip->ip_len != len) {
//       if (len > ip->ip_len) {
//        goto bad;
//       }
//       m_adj(m, len - ip->ip_len);
//       ip->ip_len = len;
//      }
//     
//     
//     
//     
//     
//      save_ip = *ip;
//      save_ip.ip_len+= iphlen;
//     
//     
//     
//     
//      if (uh->uh_sum) {
//           memset(&((struct ipovly *)ip)->ih_mbuf, 0, sizeof(struct mbuf_ptr));
//        ((struct ipovly *)ip)->ih_x1 = 0;
//        ((struct ipovly *)ip)->ih_len = uh->uh_ulen;
//        if(cksum(m, len + sizeof(struct ip))) {
//          goto bad;
//        }
//      }
//     
//     
//     
//     
//             if (ntohs(uh->uh_dport) == 67) {
//                 bootp_input(m);
//                 goto bad;
//             }
//     
//             if (slirp->restricted) {
//                 goto bad;
//             }
//      so = slirp->udp_last_so;
//      if (so->so_lport != uh->uh_sport ||
//          so->so_laddr.s_addr != ip->ip_src.s_addr) {
//       struct socket *tmp;
//     
//       for (tmp = slirp->udb.so_next; tmp != &slirp->udb;
//            tmp = tmp->so_next) {
//        if (tmp->so_lport == uh->uh_sport &&
//            tmp->so_laddr.s_addr == ip->ip_src.s_addr) {
//         so = tmp;
//         break;
//        }
//       }
//       if (tmp == &slirp->udb) {
//         so = 
//             ((void *)0)
//                 ;
//       } else {
//         slirp->udp_last_so = so;
//       }
//      }
//     
//      if (so == 
//               ((void *)0)
//                   ) {
//     
//     
//     
//     
//        so = socreate(slirp);
//        if (!so) {
//            goto bad;
//        }
//        if(udp_attach(so) == -1) {
//         
//                               ;
//          sofree(so);
//          goto bad;
//        }
//     
//     
//     
//     
//        so->so_laddr = ip->ip_src;
//        so->so_lport = uh->uh_sport;
//     
//        if ((so->so_iptos = udp_tos(so)) == 0)
//          so->so_iptos = ip->ip_tos;
//     
//     
//     
//     
//     
//      }
//     
//             so->so_faddr = ip->ip_dst;
//             so->so_fport = uh->uh_dport;
//     
//      iphlen += sizeof(struct udphdr);
//      m->m_hdr.mh_len -= iphlen;
//      m->m_hdr.mh_data += iphlen;
//     
//     
//     
//     
//      if(sosendto(so,m) == -1) {
//        m->m_hdr.mh_len += iphlen;
//        m->m_hdr.mh_data -= iphlen;
//        *ip=save_ip;
//        ;
//        icmp_error(m, 3,0, 0,strerror(
//                                                               (*__errno_location ())
//                                                                    ));
//      }
//     
//      m_free(so->so_m);
//     
//     
//      m->m_hdr.mh_len += iphlen;
//      m->m_hdr.mh_data -= iphlen;
//      *ip=save_ip;
//      so->so_m=m;
//     
//      return;
//     bad:
//      m_free(m);
//      return;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2870 {
// Rust rewrite of slirp's udp_input().
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{
    addr_of_mut, copy_nonoverlapping, null_mut, read_unaligned, write_bytes, write_unaligned,
};

const BOOTP_SERVER: u16 = 67;
const ICMP_UNREACH: u8 = 3;
const ICMP_UNREACH_NET: u8 = 0;

// byte offsets inside the (packed) IP header
const IP_TOS: usize = 1;
const IP_LEN: usize = 2;
const IP_SUM: usize = 10;
const IP_SRC: usize = 12;
const IP_DST: usize = 16;

// ---------------------------------------------------------------- C types --

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
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ip { u_int ip_hl:4, ip_v:4; ... } __attribute__((packed));
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

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
}

// struct ipovly is laid out exactly like struct ip
#[repr(C, packed)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

#[repr(C)]
pub struct udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
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
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
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
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
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

#[repr(C)]
pub struct tos_t {
    pub lport: u16,
    pub fport: u16,
    pub tos: u8,
    pub emu: u8,
}

// ------------------------------------------------------------- C functions --

extern "C" {
    fn ip_stripoptions(m: *mut mbuf, m0: *mut mbuf);
    fn m_adj(m: *mut mbuf, len: c_int);
    fn m_free(m: *mut mbuf);
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
    fn bootp_input(m: *mut mbuf);
    fn socreate(slirp: *mut Slirp) -> *mut socket;
    fn udp_attach(so: *mut socket) -> c_int;
    fn sofree(so: *mut socket);
    fn sosendto(so: *mut socket, m: *mut mbuf) -> c_int;
    fn icmp_error(msrc: *mut mbuf, type_: u8, code: u8, minsize: c_int, message: *const c_char);
    fn __errno_location() -> *mut c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
}

// ---------------------------------------------------------------- helpers ---

#[inline]
fn ntohs(v: u16) -> u16 {
    u16::from_be(v)
}

#[inline]
unsafe fn ip_u16(p: *const ip, off: usize) -> u16 {
    read_unaligned((p as *const u8).add(off) as *const u16)
}

#[inline]
unsafe fn ip_set_u16(p: *mut ip, off: usize, v: u16) {
    write_unaligned((p as *mut u8).add(off) as *mut u16, v)
}

#[inline]
unsafe fn ip_in_addr(p: *const ip, off: usize) -> in_addr {
    read_unaligned((p as *const u8).add(off) as *const in_addr)
}

#[inline]
unsafe fn uh_u16(p: *const udphdr, off: usize) -> u16 {
    read_unaligned((p as *const u8).add(off) as *const u16)
}

static UDPTOS: [tos_t; 2] = [
    tos_t {
        lport: 0,
        fport: 53,
        tos: 0x10,
        emu: 0,
    },
    tos_t {
        lport: 0,
        fport: 0,
        tos: 0,
        emu: 0,
    },
];

// static uint8_t udp_tos(struct socket *so)
unsafe fn udp_tos(so: *mut socket) -> u8 {
    let mut i: usize = 0;
    loop {
        let t = match UDPTOS.get(i) {
            Some(t) => t,
            None => break,
        };
        if t.tos == 0 {
            break;
        }
        if (t.fport != 0 && ntohs((*so).so_fport) == t.fport)
            || (t.lport != 0 && ntohs((*so).so_lport) == t.lport)
        {
            (*so).so_emu = t.emu;
            return t.tos;
        }
        i += 1;
    }
    0
}

// ------------------------------------------------------------------ target --

#[no_mangle]
pub unsafe extern "C" fn udp_input(m: *mut mbuf, mut iphlen: c_int) {
    let slirp: *mut Slirp = (*m).slirp;

    // Strip IP options, if any.
    if iphlen > size_of::<ip>() as c_int {
        ip_stripoptions(m, null_mut());
        iphlen = size_of::<ip>() as c_int;
    }

    // Get IP and UDP header together in first mbuf.
    let ip: *mut ip = (*m).m_hdr.mh_data as *mut ip;
    let uh: *mut udphdr = (ip as *mut u8).add(iphlen as usize) as *mut udphdr;

    let uh_sport = uh_u16(uh, 0);
    let uh_dport = uh_u16(uh, 2);
    let uh_ulen = uh_u16(uh, 4);
    let uh_sum = uh_u16(uh, 6);

    // Make mbuf data length reflect UDP length.
    let len: c_int = ntohs(uh_ulen) as c_int;

    let cur_ip_len = ip_u16(ip, IP_LEN) as c_int;
    if cur_ip_len != len {
        if len > cur_ip_len {
            // bad:
            m_free(m);
            return;
        }
        m_adj(m, len - cur_ip_len);
        ip_set_u16(ip, IP_LEN, len as u16);
    }

    // Save a copy of the IP header in case we want to restore it.
    let mut save_ip = [0u8; size_of::<ip>()];
    copy_nonoverlapping(ip as *const u8, save_ip.as_mut_ptr(), size_of::<ip>());
    {
        let l = read_unaligned(save_ip.as_ptr().add(IP_LEN) as *const u16);
        let nl = (l as c_int).wrapping_add(iphlen) as u16;
        write_unaligned(save_ip.as_mut_ptr().add(IP_LEN) as *mut u16, nl);
    }

    // Checksum extended UDP header and data.
    if uh_sum != 0 {
        write_bytes(ip as *mut u8, 0, size_of::<mbuf_ptr>()); // ih_mbuf
        *((ip as *mut u8).add(size_of::<mbuf_ptr>())) = 0; // ih_x1
        ip_set_u16(ip, IP_SUM, uh_ulen); // ih_len = uh->uh_ulen
        if cksum(m, len + size_of::<ip>() as c_int) != 0 {
            // bad:
            m_free(m);
            return;
        }
    }

    // handle DHCP/BOOTP
    if ntohs(uh_dport) == BOOTP_SERVER {
        bootp_input(m);
        // bad:
        m_free(m);
        return;
    }

    if (*slirp).restricted != 0 {
        // bad:
        m_free(m);
        return;
    }

    let ip_src = ip_in_addr(ip, IP_SRC);

    // Locate pcb for datagram.
    let mut so: *mut socket = (*slirp).udp_last_so;
    if (*so).so_lport != uh_sport || (*so).so_laddr.s_addr != ip_src.s_addr {
        let udb_addr: *mut socket = addr_of_mut!((*slirp).udb);
        let mut tmp: *mut socket = (*slirp).udb.so_next;

        while tmp != udb_addr {
            if (*tmp).so_lport == uh_sport && (*tmp).so_laddr.s_addr == ip_src.s_addr {
                so = tmp;
                break;
            }
            tmp = (*tmp).so_next;
        }
        if tmp == udb_addr {
            so = null_mut();
        } else {
            (*slirp).udp_last_so = so;
        }
    }

    if so.is_null() {
        // If there's no socket for this packet, create one.
        so = socreate(slirp);
        if so.is_null() {
            // bad:
            m_free(m);
            return;
        }
        if udp_attach(so) == -1 {
            sofree(so);
            // bad:
            m_free(m);
            return;
        }

        // Setup fields
        (*so).so_laddr = ip_src;
        (*so).so_lport = uh_sport;

        let tos = udp_tos(so);
        (*so).so_iptos = tos;
        if tos == 0 {
            (*so).so_iptos = *((ip as *const u8).add(IP_TOS));
        }
    }

    (*so).so_faddr = ip_in_addr(ip, IP_DST);
    (*so).so_fport = uh_dport;

    iphlen += size_of::<udphdr>() as c_int;
    (*m).m_hdr.mh_len -= iphlen;
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(iphlen as isize);

    // Now we sendto() the packet.
    if sosendto(so, m) == -1 {
        (*m).m_hdr.mh_len += iphlen;
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(iphlen as isize));
        // *ip = save_ip
        copy_nonoverlapping(save_ip.as_ptr(), ip as *mut u8, size_of::<ip>());
        let err = *__errno_location();
        icmp_error(m, ICMP_UNREACH, ICMP_UNREACH_NET, 0, strerror(err));
    }

    m_free((*so).so_m); // used for ICMP if error on sorecvfrom

    // restore the orig mbuf packet
    (*m).m_hdr.mh_len += iphlen;
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(iphlen as isize));
    copy_nonoverlapping(save_ip.as_ptr(), ip as *mut u8, size_of::<ip>());
    (*so).so_m = m; // ICMP backup
}
}

// ── `Slirp *slirp = m->slirp; register struct ip *ip; register struct udphdr *uh; int len; struct ip…` · slirp/udp.c:59-68 · chunk 2871 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = m->slirp;
//      register struct ip *ip;
//      register struct udphdr *uh;
//      int len;
//      struct ip save_ip;
//      struct socket *so;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2871 {
// Ported piece of udp_input: reads the Slirp instance out of the mbuf.
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int};

// Opaque handles: only pointers to these are handled here.
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[repr(C)]
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

#[no_mangle]
pub extern "C" fn udp_input__part2871(m: *mut Mbuf) -> *mut Slirp {
    unsafe { (*m).slirp }
}
}

// ── `if (iphlen > sizeof(struct ip))` · slirp/udp.c:76-79 · chunk 2872 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(iphlen > sizeof(struct ip)) {
//       ip_stripoptions(m, (struct mbuf *)0);
//       iphlen = sizeof(struct ip);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2872 {
// Port of udp_input() lines 76-79: strip IP options if the IP header is larger
// than a plain struct ip (sizeof(struct ip) == 20, the struct is packed).
use core::ffi::{c_int, c_void};

extern "C" {
    // void ip_stripoptions(register struct mbuf *, struct mbuf *);
    fn ip_stripoptions(m: *mut c_void, o: *mut c_void);
}

const SIZEOF_STRUCT_IP: c_int = 20;

#[no_mangle]
pub extern "C" fn udp_input__part2872(m: *mut c_void, iphlen: *mut c_int) {
    unsafe {
        if *iphlen > SIZEOF_STRUCT_IP {
            ip_stripoptions(m, core::ptr::null_mut());
            *iphlen = SIZEOF_STRUCT_IP;
        }
    }
}
}

// ── `ip = ((struct ip *)(m)->m_hdr.mh_data); uh = (struct udphdr *)((caddr_t)ip + iphlen); len = nto…` · slirp/udp.c:84-91 · chunk 2873 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ip = ((struct ip *)(m)->m_hdr.mh_data);
//      uh = (struct udphdr *)((caddr_t)ip + iphlen);
//     
//     
//     
//     
//     
//      len = ntohs((uint16_t)uh->uh_ulen);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2873 {
// Portion of udp_input (slirp/udp.c): locate the IP and UDP headers and
// decode the UDP length.
use core::ffi::{c_char, c_int, c_void};

// Opaque mirror of `struct ip` (never dereferenced here).
#[repr(C)]
pub struct Ip {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
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
pub struct Mbuf {
    pub m_hdr: Mhdr,
    pub slirp: *mut c_void,
    pub m_dat: M_dat,
}

extern "C" {
    fn ntohs(netshort: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn udp_input__part2873(
    m: *mut Mbuf,
    iphlen: c_int,
    ipp: *mut *mut Ip,
    uhpp: *mut *mut Udphdr,
    lenp: *mut c_int,
) {
    let ip: *mut Ip = (*m).m_hdr.mh_data as *mut Ip;
    let uh: *mut Udphdr = (ip as *mut c_char).offset(iphlen as isize) as *mut Udphdr;
    let len: c_int = ntohs((*uh).uh_ulen as u16) as c_int;

    if !ipp.is_null() {
        *ipp = ip;
    }
    if !uhpp.is_null() {
        *uhpp = uh;
    }
    if !lenp.is_null() {
        *lenp = len;
    }
}
}

// ── `if (ip->ip_len != len)` · slirp/udp.c:93-99 · chunk 2874 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip->ip_len != len) {
//       if (len > ip->ip_len) {
//        goto bad;
//       }
//       m_adj(m, len - ip->ip_len);
//       ip->ip_len = len;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2874 {
// Ported piece of udp_input(): UDP length check / mbuf adjustment.
// Returns 0 to continue, nonzero to take the C `goto bad` path.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip is __attribute__((packed))
#[repr(C, packed)]
pub struct Ip {
    pub ip_vhl: u8, // u_int ip_hl:4, ip_v:4;
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

extern "C" {
    fn m_adj(m: *mut c_void, len: c_int);
}

#[no_mangle]
pub extern "C" fn udp_input__part2874(m: *mut c_void, ip: *mut Ip, len: c_int) -> c_int {
    if ip.is_null() {
        return 0;
    }
    unsafe {
        let lenp = core::ptr::addr_of_mut!((*ip).ip_len);
        let ip_len = core::ptr::read_unaligned(lenp) as c_int;
        if ip_len != len {
            if len > ip_len {
                return 1;
            }
            m_adj(m, len - ip_len);
            core::ptr::write_unaligned(lenp, len as u16);
        }
    }
    0
}
}

// ── `save_ip = *ip; save_ip.ip_len+= iphlen;` · slirp/udp.c:105-106 · chunk 2875 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     save_ip = *ip;
//      save_ip.ip_len+= iphlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2875 {
// Port of udp_input lines 105-106:
//   save_ip = *ip;
//   save_ip.ip_len += iphlen;
use core::ffi::c_int;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ip {
    // bitfields ip_hl:4, ip_v:4 share the first byte
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

#[no_mangle]
pub unsafe extern "C" fn udp_input__part2875(ip: *mut Ip, iphlen: c_int, save_ip: *mut Ip) {
    if ip.is_null() || save_ip.is_null() {
        return;
    }
    // save_ip = *ip;
    core::ptr::copy_nonoverlapping(ip as *const Ip, save_ip, 1);
    // save_ip.ip_len += iphlen;  (uint16_t promoted to int, then truncated back)
    let cur = (*ip).ip_len as c_int;
    (*save_ip).ip_len = cur.wrapping_add(iphlen) as u16;
}
}

// ── `if (uh->uh_sum)` · slirp/udp.c:111-118 · chunk 2876 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (uh->uh_sum) {
//           memset(&((struct ipovly *)ip)->ih_mbuf, 0, sizeof(struct mbuf_ptr));
//        ((struct ipovly *)ip)->ih_x1 = 0;
//        ((struct ipovly *)ip)->ih_len = uh->uh_ulen;
//        if(cksum(m, len + sizeof(struct ip))) {
//          goto bad;
//        }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2876 {
#![allow(non_camel_case_types)]

// Port of slirp/udp.c udp_input() extended-UDP-header checksum block.

use core::ffi::{c_char, c_int};

// Opaque stands-ins for structures only handled through pointers.
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct socket {
    _opaque: [u8; 0],
}

// struct m_hdr (slirp/mbuf.h), exact field order.
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

// union M_dat
#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// struct mbuf (slirp/mbuf.h), exact field order.
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

// struct udphdr (slirp/udp.h) -- note uh_ulen is int16_t.
#[repr(C)]
pub struct udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

extern "C" {
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
}

// Byte offsets inside `struct ipovly` (packed) that are written here.
const IH_MBUF_SIZE: usize = 8; // sizeof(struct mbuf_ptr)
const IH_X1_OFF: usize = 8;
const IH_LEN_OFF: usize = 10;
const IP_SIZE: c_int = 20; // sizeof(struct ip), packed

/// Returns nonzero when cksum() reports a bad checksum (C caller does `goto bad`).
#[no_mangle]
pub unsafe extern "C" fn udp_input__part2876(
    m: *mut mbuf,
    ip: *mut c_char,
    uh: *const udphdr,
    len: c_int,
) -> c_int {
    if (*uh).uh_sum != 0 {
        let base = ip as *mut u8;

        // memset(&((struct ipovly *)ip)->ih_mbuf, 0, sizeof(struct mbuf_ptr));
        let mut i: usize = 0;
        while i < IH_MBUF_SIZE {
            core::ptr::write_volatile(base.add(i), 0u8);
            i += 1;
        }

        // ((struct ipovly *)ip)->ih_x1 = 0;
        core::ptr::write_volatile(base.add(IH_X1_OFF), 0u8);

        // ((struct ipovly *)ip)->ih_len = uh->uh_ulen;
        let raw_len: u16 = (*uh).uh_ulen as u16;
        let bytes = raw_len.to_ne_bytes();
        core::ptr::write_volatile(base.add(IH_LEN_OFF), bytes[0]);
        core::ptr::write_volatile(base.add(IH_LEN_OFF + 1), bytes[1]);

        // if (cksum(m, len + sizeof(struct ip))) goto bad;
        if cksum(m, len.wrapping_add(IP_SIZE)) != 0 {
            return 1;
        }
    }
    0
}
}

// ── `if (ntohs(uh->uh_dport) == 67)` · slirp/udp.c:123-126 · chunk 2877 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ntohs(uh->uh_dport) == 67) {
//                 bootp_input(m);
//                 goto bad;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2877 {
// Portion of udp_input: BOOTP (port 67) early dispatch.
#![allow(non_snake_case)]

use core::ffi::{c_int, c_ushort};

// Mirror of the C `struct udphdr` (fields in order, same types).
#[repr(C)]
pub struct Udphdr {
    pub uh_sport: c_ushort,
    pub uh_dport: c_ushort,
    pub uh_ulen: i16,
    pub uh_sum: c_ushort,
}

extern "C" {
    fn bootp_input(m: *mut core::ffi::c_void);
}

// Returns 1 when the packet was handed to bootp_input (caller must jump to
// `bad`), 0 otherwise.
#[no_mangle]
pub extern "C" fn udp_input__part2877(m: *mut core::ffi::c_void, uh: *mut Udphdr) -> c_int {
    unsafe {
        let dport = (*uh).uh_dport;
        if u16::from_be(dport) == 67 {
            bootp_input(m);
            return 1;
        }
    }
    0
}
}

// ── `if (slirp->restricted)` · slirp/udp.c:128-130 · chunk 2878 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->restricted) {
//                 goto bad;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2878 {
// Piece of slirp's udp_input: the restricted-mode early bail-out check.
// Mirrors only the leading fields of struct Slirp (in declaration order) that
// precede `restricted`, which is the only field read here.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
}

#[no_mangle]
pub extern "C" fn udp_input__part2878(slirp: *const Slirp) -> c_int {
    if slirp.is_null() {
        return 0;
    }
    let restricted = unsafe { (*slirp).restricted };
    if restricted != 0 {
        1
    } else {
        0
    }
}
}

// ── `so = slirp->udp_last_so;` · slirp/udp.c:145-145 · chunk 2879 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so = slirp->udp_last_so;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2879 {
// Port of the piece `so = slirp->udp_last_so;` from the C function udp_input.
// Layout mirrors (prefix of) the C structs so that the field offset of
// Slirp::udp_last_so is identical to the C one.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

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
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
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
}

#[no_mangle]
pub extern "C" fn udp_input__part2879(slirp: *mut Slirp) -> *mut Socket {
    unsafe { (*slirp).udp_last_so }
}
}

// ── `if (so->so_lport != uh->uh_sport || so->so_laddr.s_addr != ip->ip_src.s_addr)` · slirp/udp.c:146-163 · chunk 2880 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_lport != uh->uh_sport ||
//          so->so_laddr.s_addr != ip->ip_src.s_addr) {
//       struct socket *tmp;
//     
//       for (tmp = slirp->udb.so_next; tmp != &slirp->udb;
//            tmp = tmp->so_next) {
//        if (tmp->so_lport == uh->uh_sport &&
//            tmp->so_laddr.s_addr == ip->ip_src.s_addr) {
//         so = tmp;
//         break;
//        }
//       }
//       if (tmp == &slirp->udb) {
//         so = 
//             ((void *)0)
//                 ;
//       } else {
//         slirp->udp_last_so = so;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2880 {
// Port of the socket-lookup piece of slirp's udp_input().
use core::ffi::{c_int, c_uchar, c_uint, c_void};

#[repr(C)]
#[allow(dead_code)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
}

#[repr(C)]
#[allow(dead_code)]
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

// struct ip from slirp's ip.h.  The C definition is packed, but its natural
// layout is already exactly 20 bytes with these offsets, so repr(C) matches.
#[repr(C)]
#[allow(dead_code)]
pub struct ip_hdr {
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

#[repr(C)]
#[allow(dead_code)]
pub struct udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[no_mangle]
pub unsafe extern "C" fn udp_input__part2880(
    so_p: *mut *mut socket,
    uh: *mut udphdr,
    ip: *mut ip_hdr,
    udb_next: *mut socket,
    udb_head: *mut socket,
    udp_last_so: *mut *mut socket,
) {
    let so: *mut socket = *so_p;
    let uh_sport: u16 = (*uh).uh_sport;
    let src_addr: u32 = (*ip).ip_src.s_addr;

    if (*so).so_lport != uh_sport || (*so).so_laddr.s_addr != src_addr {
        let mut tmp: *mut socket = udb_next;

        while tmp != udb_head {
            if (*tmp).so_lport == uh_sport && (*tmp).so_laddr.s_addr == src_addr {
                *so_p = tmp;
                break;
            }
            tmp = (*tmp).so_next;
        }

        if tmp == udb_head {
            *so_p = core::ptr::null_mut();
        } else {
            *udp_last_so = *so_p;
        }
    }
}
}

// ── `if (so == ((void *)0)` · slirp/udp.c:165-194 · chunk 2881 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so == 
//               ((void *)0)
//                   ) {
//     
//     
//     
//     
//        so = socreate(slirp);
//        if (!so) {
//            goto bad;
//        }
//        if(udp_attach(so) == -1) {
//         
//                               ;
//          sofree(so);
//          goto bad;
//        }
//     
//     
//     
//     
//        so->so_laddr = ip->ip_src;
//        so->so_lport = uh->uh_sport;
//     
//        if ((so->so_iptos = udp_tos(so)) == 0)
//          so->so_iptos = ip->ip_tos;
//     
//     
//     
//     
//     
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2881 {
// Port of the "create socket for this datagram" block of slirp's udp_input().
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr::addr_of;

// Opaque mirror of `Slirp` (only ever passed through to socreate()).
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sb {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Mirror of `struct socket` (slirp/socket.h), same fields in the same order.
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
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

// Mirror of `struct ip` (packed).
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

// Mirror of `struct udphdr`.
#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TosT {
    lport: u16,
    fport: u16,
    tos: u8,
    emu: u8,
}

// The `static const struct tos_t udptos[]` table of udp.c; udp_tos() is a
// static (non-exported) function, so its logic lives here.
static UDPTOS: [TosT; 2] = [
    TosT { lport: 0, fport: 53, tos: 0x10, emu: 0 },
    TosT { lport: 0, fport: 0, tos: 0, emu: 0 },
];

extern "C" {
    fn socreate(slirp: *mut Slirp) -> *mut Socket;
    fn udp_attach(so: *mut Socket) -> c_int;
    fn sofree(so: *mut Socket);
    fn ntohs(x: u16) -> u16;
}

unsafe fn udp_tos(so: *mut Socket) -> u8 {
    for e in UDPTOS.iter() {
        let e = *e;
        if e.tos == 0 {
            break;
        }
        if (e.fport != 0 && ntohs((*so).so_fport) == e.fport)
            || (e.lport != 0 && ntohs((*so).so_lport) == e.lport)
        {
            (*so).so_emu = e.emu;
            return e.tos;
        }
    }
    0
}

/// Returns 0 on success, -1 when the caller must `goto bad`.
#[no_mangle]
pub extern "C" fn udp_input__part2881(
    p_so: *mut *mut Socket,
    slirp: *mut Slirp,
    ip: *mut Ip,
    uh: *mut Udphdr,
) -> c_int {
    unsafe {
        let so = *p_so;
        if !so.is_null() {
            // A socket for this datagram already exists: nothing to do.
            return 0;
        }

        let nso = socreate(slirp);
        if nso.is_null() {
            return -1;
        }
        if udp_attach(nso) == -1 {
            sofree(nso);
            *p_so = nso;
            return -1;
        }

        (*nso).so_laddr = addr_of!((*ip).ip_src).read_unaligned();
        (*nso).so_lport = addr_of!((*uh).uh_sport).read_unaligned();

        let tos = udp_tos(nso);
        (*nso).so_iptos = tos;
        if tos == 0 {
            (*nso).so_iptos = addr_of!((*ip).ip_tos).read_unaligned();
        }

        *p_so = nso;
        0
    }
}
}

// ── `so->so_faddr = ip->ip_dst; so->so_fport = uh->uh_dport; iphlen += sizeof(struct udphdr); m->m_h…` · slirp/udp.c:196-201 · chunk 2882 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_faddr = ip->ip_dst;
//             so->so_fport = uh->uh_dport;
//     
//      iphlen += sizeof(struct udphdr);
//      m->m_hdr.mh_len -= iphlen;
//      m->m_hdr.mh_data += iphlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2882 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip (packed): ip_hl:4 / ip_v:4 share the first byte
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
}

// prefix of struct socket, up to so_lport, in exact declaration order
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
}

#[no_mangle]
pub extern "C" fn udp_input__part2882(
    so: *mut Socket,
    ip: *mut Ip,
    uh: *mut Udphdr,
    m: *mut Mbuf,
    iphlen: c_int,
) -> c_int {
    unsafe {
        // so->so_faddr = ip->ip_dst;
        let dst: InAddr = core::ptr::read_unaligned(core::ptr::addr_of!((*ip).ip_dst));
        (*so).so_faddr = dst;

        // so->so_fport = uh->uh_dport;
        let dport: u16 = (*uh).uh_dport;
        (*so).so_fport = dport;

        // iphlen += sizeof(struct udphdr);
        let new_iphlen = iphlen.wrapping_add(core::mem::size_of::<Udphdr>() as c_int);

        // m->m_hdr.mh_len -= iphlen; m->m_hdr.mh_data += iphlen;
        let mh = core::ptr::addr_of_mut!((*m).m_hdr);
        (*mh).mh_len = (*mh).mh_len.wrapping_sub(new_iphlen);
        (*mh).mh_data = (*mh).mh_data.wrapping_offset(new_iphlen as isize);

        new_iphlen
    }
}
}

// ── `if (sosendto(so,m) == -1)` · slirp/udp.c:206-212 · chunk 2883 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if(sosendto(so,m) == -1) {
//        m->m_hdr.mh_len += iphlen;
//        m->m_hdr.mh_data -= iphlen;
//        *ip=save_ip;
//        ;
//        icmp_error(m, 3,0, 0,strerror(
//                                                               (*__errno_location ())
//                                                                    ));
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2883 {
// Port of the `sosendto` failure handling block from slirp/udp.c udp_input().
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uchar};

// Opaque mirror of `struct socket` (never dereferenced here).
#[repr(C)]
pub struct socket {
    pub _opaque: [u8; 0],
}

// Opaque mirror of the `Slirp` type (never dereferenced here).
#[repr(C)]
pub struct Slirp {
    pub _opaque: [u8; 0],
}

#[derive(Clone, Copy)]
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

// `struct ip` is __attribute__((packed)); the leading bitfields
// `u_int ip_hl:4, ip_v:4` occupy a single byte.
#[derive(Clone, Copy)]
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

extern "C" {
    fn sosendto(s: *mut socket, m: *mut mbuf) -> c_int;
    fn icmp_error(
        msrc: *mut mbuf,
        type_: c_uchar,
        code: c_uchar,
        minsize: c_int,
        message: *const c_char,
    );
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

#[no_mangle]
pub extern "C" fn udp_input__part2883(
    so: *mut socket,
    m: *mut mbuf,
    iphlen: c_int,
    ip: *mut ip,
    save_ip: *mut ip,
) {
    unsafe {
        if sosendto(so, m) == -1 {
            (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(iphlen);
            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(iphlen as isize));
            *ip = *save_ip;
            icmp_error(m, 3, 0, 0, strerror(*__errno_location()));
        }
    }
}
}

// ── `m_free(so->so_m); m->m_hdr.mh_len += iphlen; m->m_hdr.mh_data -= iphlen; *ip=save_ip; so->so_m=…` · slirp/udp.c:214-222 · chunk 2884 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m_free(so->so_m);
//     
//     
//      m->m_hdr.mh_len += iphlen;
//      m->m_hdr.mh_data -= iphlen;
//      *ip=save_ip;
//      so->so_m=m;
//     
//      return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2884 {
// Portion of slirp udp_input(): hand the received mbuf to the socket.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip (packed); ip_hl:4/ip_v:4 share the first byte.
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
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
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
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
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
pub struct MDat {
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: MDat,
}

extern "C" {
    fn m_free(mb: *mut Mbuf);
}

#[no_mangle]
pub unsafe extern "C" fn udp_input__part2884(
    m: *mut Mbuf,
    iphlen: c_int,
    ip: *mut Ip,
    save_ip: *const Ip,
    so: *mut Socket,
) {
    m_free((*so).so_m);

    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(iphlen);
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(iphlen as isize));

    core::ptr::copy_nonoverlapping(save_ip, ip, 1);

    (*so).so_m = m;
}
}

// ── `bad: m_free(m);` · slirp/udp.c:223-224 · chunk 2885 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     bad:
//      m_free(m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2885 {
// Port of the `bad:` tail of udp_input(): frees the mbuf.
use core::ffi::c_void;

extern "C" {
    // void m_free(struct mbuf *);
    fn m_free(m: *mut c_void);
}

#[no_mangle]
pub extern "C" fn udp_input__part2885(m: *mut c_void) {
    unsafe {
        m_free(m);
    }
}
}

// ── `return;` · slirp/udp.c:225-225 · chunk 2886 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2886 {
// Port of the tail `return;` statement of udp_input() (slirp/udp.c line 222/225).
// The success path simply returns from udp_input after restoring the original
// mbuf packet; there is no state to touch here, so the ported piece is a no-op
// that the C caller follows with its own `return;`.
#[no_mangle]
pub extern "C" fn udp_input__part2886() {}
}

// ── `udp_output2` · slirp/udp.c:228-277 · chunk 2887 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int udp_output2(struct socket *so, struct mbuf *m,
//                     struct sockaddr_in *saddr, struct sockaddr_in *daddr,
//                     int iptos)
//     {
//      register struct udpiphdr *ui;
//      int error = 0;
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//     
//     
//     
//     
//      m->m_hdr.mh_data -= sizeof(struct udpiphdr);
//      m->m_hdr.mh_len += sizeof(struct udpiphdr);
//     
//     
//     
//     
//     
//      ui = ((struct udpiphdr *)(m)->m_hdr.mh_data);
//         memset(&ui->ui_i.ih_mbuf, 0 , sizeof(struct mbuf_ptr));
//      ui->ui_i.ih_x1 = 0;
//      ui->ui_i.ih_pr = 
//                 IPPROTO_UDP
//                            ;
//      ui->ui_i.ih_len = htons(m->m_hdr.mh_len - sizeof(struct ip));
//     
//             ui->ui_i.ih_src = saddr->sin_addr;
//      ui->ui_i.ih_dst = daddr->sin_addr;
//      ui->ui_u.uh_sport = saddr->sin_port;
//      ui->ui_u.uh_dport = daddr->sin_port;
//      ui->ui_u.uh_ulen = ui->ui_i.ih_len;
//     
//     
//     
//     
//      ui->ui_u.uh_sum = 0;
//      if ((ui->ui_u.uh_sum = cksum(m, m->m_hdr.mh_len)) == 0)
//       ui->ui_u.uh_sum = 0xffff;
//      ((struct ip *)ui)->ip_len = m->m_hdr.mh_len;
//     
//      ((struct ip *)ui)->ip_ttl = 64;
//      ((struct ip *)ui)->ip_tos = iptos;
//     
//      error = ip_output(so, m);
//     
//      return (error);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2887 {
// Rust port of slirp's udp_output2().
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

// ---------------------------------------------------------------------------
// C types this function can see (exact layout mirrors)
// ---------------------------------------------------------------------------

// typedef struct Slirp Slirp; opaque here.
#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

// struct socket; opaque here (only pointer passing / forwarding).
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

// struct in_addr { in_addr_t s_addr; } with typedef uint32_t in_addr_t;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct sockaddr_in
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct mbuf_ptr {
    pub mptr: *mut Mbuf,
}

// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

// struct udphdr { uint16_t uh_sport; uint16_t uh_dport; int16_t uh_ulen; uint16_t uh_sum; };
#[repr(C)]
#[derive(Clone, Copy)]
pub struct udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

// struct udpiphdr { struct ipovly ui_i; struct udphdr ui_u; };
#[repr(C)]
#[derive(Clone, Copy)]
pub struct udpiphdr {
    pub ui_i: ipovly,
    pub ui_u: udphdr,
}

// struct m_hdr { ... };
#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char, // caddr_t
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; };
#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// struct mbuf { struct m_hdr m_hdr; Slirp *slirp; union M_dat M_dat; };
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub m_dat: M_dat,
}

// ---------------------------------------------------------------------------
// Layout constants derived from the mirrors above
// ---------------------------------------------------------------------------

const PTR: usize = core::mem::size_of::<mbuf_ptr>();
const OFF_X1: usize = PTR; // ipovly.ih_x1  (also struct ip's ip_ttl overlay)
const OFF_PR: usize = PTR + 1; // ipovly.ih_pr  (also ip_p)
const OFF_LEN: usize = PTR + 2; // ipovly.ih_len (also ip_sum)
const OFF_SRC: usize = PTR + 4;
const OFF_DST: usize = PTR + 8;
const OFF_SPORT: usize = PTR + 12;
const OFF_DPORT: usize = PTR + 14;
const OFF_ULEN: usize = PTR + 16;
const OFF_SUM: usize = PTR + 18;

// struct ip overlay offsets
const OFF_IP_TOS: usize = 1;
const OFF_IP_LEN: usize = 2;

const SIZEOF_IP: c_int = 20; // sizeof(struct ip), packed
const IPPROTO_UDP: u8 = 17;
const IPDEFTTL: u8 = 64;

#[inline]
unsafe fn store<T>(base: *mut u8, off: usize, val: T) {
    core::ptr::write_unaligned(base.add(off) as *mut T, val);
}

extern "C" {
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
    fn ip_output(so: *mut Socket, m: *mut Mbuf) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn udp_output2(
    so: *mut Socket,
    m: *mut Mbuf,
    saddr: *mut SockaddrIn,
    daddr: *mut SockaddrIn,
    iptos: c_int,
) -> c_int {
    let mut error: c_int = 0;

    // Adjust for header
    // m->m_hdr.mh_data -= sizeof(struct udpiphdr);
    let hdr_size = core::mem::size_of::<udpiphdr>();
    (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(-(hdr_size as isize));
    // m->m_hdr.mh_len += sizeof(struct udpiphdr);
    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(hdr_size as c_int);

    // ui = mtod(m, struct udpiphdr *)
    let ui: *mut u8 = (*m).m_hdr.mh_data as *mut u8;

    // memset(&ui->ui_i.ih_mbuf, 0, sizeof(struct mbuf_ptr));
    core::ptr::write_bytes(ui, 0, core::mem::size_of::<mbuf_ptr>());
    store(ui, OFF_X1, 0u8);
    store(ui, OFF_PR, IPPROTO_UDP);
    // ui->ui_i.ih_len = htons(m->m_hdr.mh_len - sizeof(struct ip));
    let ih_len: u16 = (*m)
        .m_hdr
        .mh_len
        .wrapping_sub(SIZEOF_IP)
        .to_be() as u16;
    store(ui, OFF_LEN, ih_len);

    store(ui, OFF_SRC, (*saddr).sin_addr);
    store(ui, OFF_DST, (*daddr).sin_addr);
    store(ui, OFF_SPORT, (*saddr).sin_port);
    store(ui, OFF_DPORT, (*daddr).sin_port);
    store(ui, OFF_ULEN, ih_len as i16);

    // Stuff checksum and output datagram.
    store(ui, OFF_SUM, 0u16);
    let sum: u16 = cksum(m, (*m).m_hdr.mh_len) as u16;
    store(ui, OFF_SUM, sum);
    if sum == 0 {
        store(ui, OFF_SUM, 0xffffu16);
    }

    // ((struct ip *)ui)->ip_len = m->m_hdr.mh_len;
    store(ui, OFF_IP_LEN, (*m).m_hdr.mh_len as u16);
    // ((struct ip *)ui)->ip_ttl = IPDEFTTL;
    store(ui, OFF_X1, IPDEFTTL);
    // ((struct ip *)ui)->ip_tos = iptos;
    store(ui, OFF_IP_TOS, iptos as u8);

    error = ip_output(so, m);

    error
}
}

// ── `register struct udpiphdr *ui; int error = 0; ; ; ; ; ; m->m_hdr.mh_data -= sizeof(struct udpiph…` · slirp/udp.c:232-266 · chunk 2888 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     register struct udpiphdr *ui;
//      int error = 0;
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//     
//     
//     
//     
//      m->m_hdr.mh_data -= sizeof(struct udpiphdr);
//      m->m_hdr.mh_len += sizeof(struct udpiphdr);
//     
//     
//     
//     
//     
//      ui = ((struct udpiphdr *)(m)->m_hdr.mh_data);
//         memset(&ui->ui_i.ih_mbuf, 0 , sizeof(struct mbuf_ptr));
//      ui->ui_i.ih_x1 = 0;
//      ui->ui_i.ih_pr = 
//                 IPPROTO_UDP
//                            ;
//      ui->ui_i.ih_len = htons(m->m_hdr.mh_len - sizeof(struct ip));
//     
//             ui->ui_i.ih_src = saddr->sin_addr;
//      ui->ui_i.ih_dst = daddr->sin_addr;
//      ui->ui_u.uh_sport = saddr->sin_port;
//      ui->ui_u.uh_dport = daddr->sin_port;
//      ui->ui_u.uh_ulen = ui->ui_i.ih_len;
//     
//     
//     
//     
//      ui->ui_u.uh_sum = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2888 {
use core::ffi::{c_char, c_int, c_void};

const IPPROTO_UDP: u8 = 17;
const SIZEOF_IP: c_int = 20;

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

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Udpiphdr {
    pub ui_i: Ipovly,
    pub ui_u: Udphdr,
}

// struct m_hdr (start of struct mbuf, at offset 0)
#[repr(C)]
pub struct MHdr {
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
    fn htons(x: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn udp_output2__part2888(
    m: *mut MHdr,
    saddr: *mut SockaddrIn,
    daddr: *mut SockaddrIn,
) -> *mut Udpiphdr {
    let hdr_size = core::mem::size_of::<Udpiphdr>() as isize;

    // Adjust for header
    let data = (*m).mh_data;
    (*m).mh_data = data.offset(-hdr_size);
    (*m).mh_len = (*m).mh_len.wrapping_add(hdr_size as c_int);

    let ui = (*m).mh_data as *mut Udpiphdr;

    core::ptr::write_bytes(core::ptr::addr_of_mut!((*ui).ui_i.ih_mbuf), 0, 1);
    (*ui).ui_i.ih_x1 = 0;
    (*ui).ui_i.ih_pr = IPPROTO_UDP;
    let len16 = (*m).mh_len.wrapping_sub(SIZEOF_IP) as u16;
    (*ui).ui_i.ih_len = htons(len16);

    let src = (*saddr).sin_addr;
    let dst = (*daddr).sin_addr;
    (*ui).ui_i.ih_src = src;
    (*ui).ui_i.ih_dst = dst;
    (*ui).ui_u.uh_sport = (*saddr).sin_port;
    (*ui).ui_u.uh_dport = (*daddr).sin_port;
    let ih_len = core::ptr::addr_of!((*ui).ui_i.ih_len).read_unaligned();
    (*ui).ui_u.uh_ulen = ih_len as i16;

    (*ui).ui_u.uh_sum = 0;

    ui
}
}

// ── `if (ui->ui_u.uh_sum = cksum(m, m->m_hdr.mh_len)) == 0)` · slirp/udp.c:267-268 · chunk 2889 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((ui->ui_u.uh_sum = cksum(m, m->m_hdr.mh_len)) == 0)
//       ui->ui_u.uh_sum = 0xffff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2889 {
// Portion of udp_output2: compute the UDP checksum into ui->ui_u.uh_sum.

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

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
pub struct Udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct Udpiphdr {
    pub ui_i: Ipovly,
    pub ui_u: Udphdr,
}

extern "C" {
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn udp_output2__part2889(m: *mut Mbuf, ui: *mut Udpiphdr) {
    unsafe {
        let len: c_int = (*m).m_hdr.mh_len;
        let mut sum: u16 = cksum(m, len) as u16;
        if sum == 0 {
            sum = 0xffffu16;
        }
        (*ui).ui_u.uh_sum = sum;
    }
}
}

// ── `((struct ip *)ui)->ip_len = m->m_hdr.mh_len; ((struct ip *)ui)->ip_ttl = 64; ((struct ip *)ui)-…` · slirp/udp.c:269-276 · chunk 2890 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((struct ip *)ui)->ip_len = m->m_hdr.mh_len;
//     
//      ((struct ip *)ui)->ip_ttl = 64;
//      ((struct ip *)ui)->ip_tos = iptos;
//     
//      error = ip_output(so, m);
//     
//      return (error);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2890 {
// Port of the tail of slirp's udp_output2 (ip header fixups + ip_output call).

use core::ffi::{c_char, c_int};
use core::ptr::addr_of_mut;

#[repr(C)]
pub struct Slirp {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct socket {
    _priv: [u8; 0],
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
#[derive(Clone, Copy)]
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

// struct ip is __attribute__((packed)); the leading 4:4 bitfield pair
// (ip_hl, ip_v) occupies a single byte under a packed layout.
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

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
}

#[repr(C, packed)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

#[repr(C)]
pub struct udphdr {
    pub uh_sport: u16,
    pub uh_dport: u16,
    pub uh_ulen: i16,
    pub uh_sum: u16,
}

#[repr(C)]
pub struct udpiphdr {
    pub ui_i: ipovly,
    pub ui_u: udphdr,
}

extern "C" {
    fn ip_output(so: *mut socket, m: *mut mbuf) -> c_int;
}

#[no_mangle]
pub extern "C" fn udp_output2__part2890(
    ui: *mut udpiphdr,
    m: *mut mbuf,
    iptos: c_int,
) -> c_int {
    unsafe {
        let ip_ptr = ui as *mut ip;
        let len = (*m).m_hdr.mh_len;

        addr_of_mut!((*ip_ptr).ip_len).write_unaligned(len as u16);
        addr_of_mut!((*ip_ptr).ip_ttl).write_unaligned(64u8);
        addr_of_mut!((*ip_ptr).ip_tos).write_unaligned(iptos as u8);

        let so = (*m).m_hdr.mh_so;
        ip_output(so, m)
    }
}
}

// ── `udp_output` · slirp/udp.c:279-302 · chunk 2891 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int udp_output(struct socket *so, struct mbuf *m,
//                    struct sockaddr_in *addr)
//     
//     {
//         Slirp *slirp = so->slirp;
//         struct sockaddr_in saddr, daddr;
//     
//         saddr = *addr;
//         if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//             slirp->vnetwork_addr.s_addr) {
//             uint32_t inv_mask = ~slirp->vnetwork_mask.s_addr;
//     
//             if ((so->so_faddr.s_addr & inv_mask) == inv_mask) {
//                 saddr.sin_addr = slirp->vhost_addr;
//             } else if (addr->sin_addr.s_addr == loopback_addr.s_addr ||
//                        so->so_faddr.s_addr != slirp->vhost_addr.s_addr) {
//                 saddr.sin_addr = so->so_faddr;
//             }
//         }
//         daddr.sin_addr = so->so_laddr;
//         daddr.sin_port = so->so_lport;
//     
//         return udp_output2(so, m, &saddr, &daddr, so->so_iptos);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2891 {
// Rust rewrite of slirp's udp_output().

use core::ffi::{c_int, c_uchar, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [c_uchar; 8],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_hiwat: c_uint,
    pub sb_mbcnt: c_uint,
    pub sb_mbmax: c_uint,
    pub sb_lowat: c_int,
    pub sb_flags: c_int,
    pub sb_want: c_int,
    pub sb_wait: *mut c_void,
    pub sb_ctl: *mut c_void,
    pub sb_data: *mut c_void,
}

#[repr(C)]
#[allow(non_snake_case)]
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

#[repr(C)]
#[allow(non_snake_case)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [u8; 33],
}

extern "C" {
    fn udp_output2(
        so: *mut Socket,
        m: *mut c_void,
        saddr: *mut SockAddrIn,
        daddr: *mut SockAddrIn,
        iptos: c_int,
    ) -> c_int;
    #[allow(non_upper_case_globals)]
    static loopback_addr: InAddr;
}

#[no_mangle]
pub unsafe extern "C" fn udp_output(
    so: *mut Socket,
    m: *mut c_void,
    addr: *mut SockAddrIn,
) -> c_int {
    let slirp: *mut Slirp = (*so).slirp;

    let mut saddr: SockAddrIn = core::ptr::read(addr);
    let mut daddr: SockAddrIn = SockAddrIn {
        sin_family: 0,
        sin_port: 0,
        sin_addr: InAddr { s_addr: 0 },
        sin_zero: [0; 8],
    };

    if ((*so).so_faddr.s_addr & (*slirp).vnetwork_mask.s_addr)
        == (*slirp).vnetwork_addr.s_addr
    {
        let inv_mask: u32 = !(*slirp).vnetwork_mask.s_addr;

        if ((*so).so_faddr.s_addr & inv_mask) == inv_mask {
            saddr.sin_addr = (*slirp).vhost_addr;
        } else if (*addr).sin_addr.s_addr == loopback_addr.s_addr
            || (*so).so_faddr.s_addr != (*slirp).vhost_addr.s_addr
        {
            saddr.sin_addr = (*so).so_faddr;
        }
    }
    daddr.sin_addr = (*so).so_laddr;
    daddr.sin_port = (*so).so_lport;

    udp_output2(
        so,
        m,
        &mut saddr as *mut SockAddrIn,
        &mut daddr as *mut SockAddrIn,
        (*so).so_iptos as c_int,
    )
}
}

// ── `Slirp *slirp = so->slirp; struct sockaddr_in saddr, daddr; saddr = *addr;` · slirp/udp.c:283-286 · chunk 2892 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = so->slirp;
//         struct sockaddr_in saddr, daddr;
//     
//         saddr = *addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2892 {
// Portion of udp_output: extract slirp from socket, copy addr into saddr.
use core::ffi::{c_int, c_uchar};

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
    pub _opaque: [u8; 0],
}

// Prefix of `struct socket`, up to (and including) the `slirp` field.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
}

#[no_mangle]
pub extern "C" fn udp_output__part2892(
    so: *mut Socket,
    addr: *mut SockaddrIn,
    slirp_out: *mut *mut Slirp,
    saddr_out: *mut SockaddrIn,
) {
    unsafe {
        let slirp = (*so).slirp;
        *slirp_out = slirp;
        core::ptr::copy_nonoverlapping(addr, saddr_out, 1);
    }
}
}

// ── `if (so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) == slirp->vnetwork_addr.s_addr)` · slirp/udp.c:287-297 · chunk 2893 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//             slirp->vnetwork_addr.s_addr) {
//             uint32_t inv_mask = ~slirp->vnetwork_mask.s_addr;
//     
//             if ((so->so_faddr.s_addr & inv_mask) == inv_mask) {
//                 saddr.sin_addr = slirp->vhost_addr;
//             } else if (addr->sin_addr.s_addr == loopback_addr.s_addr ||
//                        so->so_faddr.s_addr != slirp->vhost_addr.s_addr) {
//                 saddr.sin_addr = so->so_faddr;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2893 {
// Rust port of the address-selection part of slirp's udp_output().
// Mirrors only the prefix of the C structs that is actually accessed.

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

// Prefix of `struct socket` (fields up to so_faddr).
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: i32,
    pub slirp: *mut Slirp,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: i32,
    pub so_faddr: InAddr,
}

// Prefix of `struct Slirp` (only the three address fields are read).
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
}

extern "C" {
    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn udp_output__part2893(
    so: *mut Socket,
    saddr: *mut SockaddrIn,
    addr: *mut SockaddrIn,
) {
    unsafe {
        let slirp = (*so).slirp;
        if ((*so).so_faddr.s_addr & (*slirp).vnetwork_mask.s_addr)
            == (*slirp).vnetwork_addr.s_addr
        {
            let inv_mask: u32 = !(*slirp).vnetwork_mask.s_addr;

            if ((*so).so_faddr.s_addr & inv_mask) == inv_mask {
                (*saddr).sin_addr = (*slirp).vhost_addr;
            } else if (*addr).sin_addr.s_addr == loopback_addr.s_addr
                || (*so).so_faddr.s_addr != (*slirp).vhost_addr.s_addr
            {
                (*saddr).sin_addr = (*so).so_faddr;
            }
        }
    }
}
}

// ── `daddr.sin_addr = so->so_laddr; daddr.sin_port = so->so_lport; return udp_output2(so, m, &saddr,…` · slirp/udp.c:298-301 · chunk 2894 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     daddr.sin_addr = so->so_laddr;
//         daddr.sin_port = so->so_lport;
//     
//         return udp_output2(so, m, &saddr, &daddr, so->so_iptos);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2894 {
// Port of the tail of slirp's udp_output(): fills in the destination
// sockaddr_in from the socket's local address/port and hands the packet
// to udp_output2().

use core::ffi::{c_char, c_int, c_uint, c_void};

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
    fn udp_output2(
        so: *mut Socket,
        m: *mut c_void,
        saddr: *mut SockaddrIn,
        daddr: *mut SockaddrIn,
        iptos: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn udp_output__part2894(
    so: *mut Socket,
    m: *mut c_void,
    saddr: *mut SockaddrIn,
    daddr: *mut SockaddrIn,
) -> c_int {
    unsafe {
        let laddr = (*so).so_laddr.s_addr;
        let lport = (*so).so_lport;
        let iptos = (*so).so_iptos as c_int;

        (*daddr).sin_addr.s_addr = laddr;
        (*daddr).sin_port = lport;

        udp_output2(so, m, saddr, daddr, iptos)
    }
}
}

// ── `udp_attach` · slirp/udp.c:304-312 · chunk 2895 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     udp_attach(struct socket *so)
//     {
//       if((so->s = os_socket(
//                            2
//                                   ,
//                                    SOCK_DGRAM
//                                              ,0)) != -1) {
//         so->so_expire = curtime + 240000;
//         slirp_insque(so, &so->slirp->udb);
//       }
//       return(so->s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2895 {
// Port of slirp udp_attach(): int udp_attach(struct socket *so)
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const SO_EXPIRE: c_uint = 240000;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
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
pub struct MHdr {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

#[repr(C)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

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
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct SBuf {
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
    pub so_m: *mut MBuf,
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
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

// Only the leading part of `struct Slirp` (through `udb`) is mirrored: the
// code below never needs anything past `udb`, and the offsets of every earlier
// field match the C layout exactly.
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
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
}

extern "C" {
    fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    static mut curtime: c_uint;
}

#[no_mangle]
pub extern "C" fn udp_attach(so: *mut Socket) -> c_int {
    unsafe {
        let s = os_socket(AF_INET, SOCK_DGRAM, 0);
        (*so).s = s;
        if s != -1 {
            (*so).so_expire = curtime.wrapping_add(SO_EXPIRE);
            let slirp = (*so).slirp;
            slirp_insque(
                so as *mut c_void,
                core::ptr::addr_of_mut!((*slirp).udb) as *mut c_void,
            );
        }
        (*so).s
    }
}
}

// ── `if (so->s = os_socket( 2 , SOCK_DGRAM ,0)) != -1)` · slirp/udp.c:307-310 · chunk 2896 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if((so->s = os_socket(
//                            2
//                                   ,
//                                    SOCK_DGRAM
//                                              ,0)) != -1) {
//         so->so_expire = curtime + 240000;
//         slirp_insque(so, &so->slirp->udb);
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2896 {
// Port of the body of udp_attach()'s socket-attach block.
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of the C `struct socket` prefix (all fields up to and including
// so_expire, which is as far as this code reaches).
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
}

extern "C" {
    fn os_socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    static mut curtime: c_uint;
}

#[no_mangle]
pub extern "C" fn udp_attach__part2896(so: *mut Socket, udb: *mut c_void) {
    // AF_INET == 2, SOCK_DGRAM == 2
    let fd = unsafe { os_socket(2, 2, 0) };
    unsafe {
        (*so).s = fd;
        if fd != -1 {
            let now = curtime;
            (*so).so_expire = now.wrapping_add(240000);
            slirp_insque(so as *mut c_void, udb);
        }
    }
}
}

// ── `return(so->s);` · slirp/udp.c:311-311 · chunk 2897 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return(so->s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2897 {
// Port of the tail of udp_attach(): return so->s;
use core::ffi::c_int;

// Prefix of `struct socket` up to and including the `int s` field, which is
// all this piece touches. Prefix layout matches the C struct exactly.
#[repr(C)]
pub struct SocketPrefix {
    pub so_next: *mut core::ffi::c_void,
    pub so_prev: *mut core::ffi::c_void,
    pub s: c_int,
}

#[no_mangle]
pub extern "C" fn udp_attach__part2897(so: *mut SocketPrefix) -> c_int {
    if so.is_null() {
        return 0;
    }
    unsafe { (*so).s }
}
}

// ── `udp_detach` · slirp/udp.c:314-319 · chunk 2898 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     udp_detach(struct socket *so)
//     {
//      close(so->s);
//      sofree(so);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2898 {
// Rewrite of slirp/udp.c: udp_detach
//
// void udp_detach(struct socket *so)
// {
//     close(so->s);   /* closesocket(s) => close(s) */
//     sofree(so);
// }

use core::ffi::c_int;

// Prefix of `struct socket` up to the `s` field; the layout of the
// accessed member is preserved exactly.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
}

extern "C" {
    fn close(fd: c_int) -> c_int;
    fn sofree(so: *mut Socket);
}

#[no_mangle]
pub extern "C" fn udp_detach(so: *mut Socket) {
    unsafe {
        if so.is_null() {
            return;
        }
        let _ = close((*so).s);
        sofree(so);
    }
}
}

// ── `close(so->s); sofree(so);` · slirp/udp.c:317-318 · chunk 2899 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     close(so->s);
//      sofree(so);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2899 {
// Portion of udp_detach: close the socket fd and release the socket.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
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

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

extern "C" {
    fn close(fd: c_int) -> c_int;
    fn sofree(so: *mut Socket);
}

#[no_mangle]
pub extern "C" fn udp_detach__part2899(so: *mut Socket) {
    unsafe {
        close((*so).s);
        sofree(so);
    }
}
}

// ── `udp_tos` · slirp/udp.c:326-341 · chunk 2901 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint8_t
//     udp_tos(struct socket *so)
//     {
//      int i = 0;
//     
//      while(udptos[i].tos) {
//       if ((udptos[i].fport && ntohs(so->so_fport) == udptos[i].fport) ||
//           (udptos[i].lport && ntohs(so->so_lport) == udptos[i].lport)) {
//            so->so_emu = udptos[i].emu;
//        return udptos[i].tos;
//       }
//       i++;
//      }
//     
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2901 {
// Rewrite of slirp/udp.c:udp_tos

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SocketPrefix {
    pub so_next: *mut SocketPrefix,
    pub so_prev: *mut SocketPrefix,
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
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TosT {
    lport: u16,
    fport: u16,
    tos: u8,
    emu: u8,
}

static UDPTOS: [TosT; 2] = [
    TosT {
        lport: 0,
        fport: 53,
        tos: 0x10,
        emu: 0,
    },
    TosT {
        lport: 0,
        fport: 0,
        tos: 0,
        emu: 0,
    },
];

extern "C" {
    fn ntohs(netshort: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn udp_tos(so: *mut SocketPrefix) -> u8 {
    let mut i: c_uint = 0;
    loop {
        let e = UDPTOS[i as usize];
        if e.tos == 0 {
            break;
        }
        let sp = &mut *so;
        let fport_match = e.fport != 0 && ntohs(sp.so_fport) == e.fport;
        let lport_match = e.lport != 0 && ntohs(sp.so_lport) == e.lport;
        if fport_match || lport_match {
            sp.so_emu = e.emu;
            return e.tos;
        }
        i += 1;
    }
    0
}
}

// ── `while (udptos[i].tos)` · slirp/udp.c:331-338 · chunk 2902 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while(udptos[i].tos) {
//       if ((udptos[i].fport && ntohs(so->so_fport) == udptos[i].fport) ||
//           (udptos[i].lport && ntohs(so->so_lport) == udptos[i].lport)) {
//            so->so_emu = udptos[i].emu;
//        return udptos[i].tos;
//       }
//       i++;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2902 {
// Port of the port-matching loop of udp_tos() (slirp/udp.c).

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of the prefix of `struct socket` up to (and including) so_emu.
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
}

// Mirror of `struct tos_t`.
#[repr(C)]
struct TosT {
    lport: u16,
    fport: u16,
    tos: u8,
    emu: u8,
}

// Mirror of the file-static `static const struct tos_t udptos[]`.
static UDPTOS: [TosT; 2] = [
    TosT {
        lport: 0,
        fport: 53,
        tos: 0x10,
        emu: 0,
    },
    TosT {
        lport: 0,
        fport: 0,
        tos: 0,
        emu: 0,
    },
];

extern "C" {
    fn ntohs(netshort: u16) -> u16;
}

/// Walks the udptos table starting at index `i`.  Returns the tos value of the
/// first matching entry (always nonzero, since the table is terminated by a
/// zero tos) after storing its emu value in so->so_emu, or 0 when no entry
/// matched (i.e. the C loop fell through).
#[no_mangle]
pub extern "C" fn udp_tos__part2902(so: *mut Socket, mut i: c_int) -> u8 {
    if so.is_null() {
        return 0;
    }
    unsafe {
        loop {
            let entry = match UDPTOS.get(i as usize) {
                Some(e) => e,
                None => break,
            };
            if entry.tos == 0 {
                break;
            }
            let fport = entry.fport;
            let lport = entry.lport;
            if (fport != 0 && ntohs((*so).so_fport) == fport)
                || (lport != 0 && ntohs((*so).so_lport) == lport)
            {
                (*so).so_emu = entry.emu;
                return entry.tos;
            }
            i += 1;
        }
    }
    0
}
}

// ── `return 0;` · slirp/udp.c:340-340 · chunk 2903 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2903 {
// Ported piece of udp_tos: the fall-through "no match" return value.
#[no_mangle]
pub extern "C" fn udp_tos__part2903() -> u8 {
    0
}
}

// ── `udp_listen` · slirp/udp.c:343-386 · chunk 2904 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *
//     udp_listen(Slirp *slirp, uint32_t haddr, u_int hport, uint32_t laddr,
//                u_int lport, int flags)
//     {
//      struct sockaddr_in addr;
//      struct socket *so;
//      socklen_t addrlen = sizeof(struct sockaddr_in), opt = 1;
//     
//      so = socreate(slirp);
//      if (!so) {
//          return 
//                ((void *)0)
//                    ;
//      }
//      so->s = os_socket(
//                       2
//                              ,
//                               SOCK_DGRAM
//                                         ,0);
//      so->so_expire = curtime + 240000;
//      slirp_insque(so, &slirp->udb);
//     
//      addr.sin_family = 
//                       2
//                              ;
//      addr.sin_addr.s_addr = haddr;
//      addr.sin_port = hport;
//     
//      if (bind(so->s,(struct sockaddr *)&addr, addrlen) < 0) {
//       udp_detach(so);
//       return 
//             ((void *)0)
//                 ;
//      }
//      setsockopt(so->s,
//                      1
//                                ,
//                                 2
//                                             ,(char *)&opt,sizeof(int));
//     
//      getsockname(so->s,(struct sockaddr *)&addr,&addrlen);
//      so->so_fport = addr.sin_port;
//      if (addr.sin_addr.s_addr == 0 ||
//          addr.sin_addr.s_addr == loopback_addr.s_addr) {
//         so->so_faddr = slirp->vhost_addr;
//      } else {
//         so->so_faddr = addr.sin_addr;
//      }
//      so->so_lport = lport;
//      so->so_laddr.s_addr = laddr;
//      if (flags != 0x200)
//         so->so_expire = 0;
//     
//      so->so_state &= 0xf000;
//      so->so_state |= 0x004 | flags;
//     
//      return so;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2904 {
// Rust port of slirp/udp.c: udp_listen()
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ushort};

pub type u_int = c_uint;
pub type u_char = core::ffi::c_uchar;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sa_family_t = c_ushort;
pub type socklen_t = c_uint;
pub type tcp_seq = u32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: core::ffi::c_long,
    pub tv_usec: core::ffi::c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct qlink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mbuf_ptr {
    pub mptr: *mut core::ffi::c_void,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct mbuf {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
    pub slirp: *mut Slirp,
    pub m_dat: [u8; 8],
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
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u_char,
    pub so_state: c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: u_int,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut core::ffi::c_void,
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
    pub opaque: *mut core::ffi::c_void,
}

extern "C" {
    static mut curtime: u_int;
    static mut loopback_addr: in_addr;

    fn socreate(slirp: *mut Slirp) -> *mut socket;
    fn udp_detach(so: *mut socket);
    fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn slirp_insque(next: *mut core::ffi::c_void, prev: *mut core::ffi::c_void);
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const core::ffi::c_void,
        optlen: socklen_t,
    ) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
}

const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_EXPIRE: u_int = 240000;
const SS_FACCEPTONCE: c_int = 0x200;
const SS_PERSISTENT_MASK: c_int = 0xf000;
const SS_ISFCONNECTED: c_int = 0x004;

#[no_mangle]
pub unsafe extern "C" fn udp_listen(
    slirp: *mut Slirp,
    haddr: u32,
    hport: u_int,
    laddr: u32,
    lport: u_int,
    flags: c_int,
) -> *mut socket {
    let mut addr: sockaddr_in = core::mem::zeroed();
    let mut addrlen: socklen_t = core::mem::size_of::<sockaddr_in>() as socklen_t;
    let mut opt: socklen_t = 1;

    let so = socreate(slirp);
    if so.is_null() {
        return core::ptr::null_mut();
    }
    (*so).s = os_socket(AF_INET, SOCK_DGRAM, 0);
    (*so).so_expire = curtime.wrapping_add(SO_EXPIRE);
    slirp_insque(
        so as *mut core::ffi::c_void,
        &mut (*slirp).udb as *mut socket as *mut core::ffi::c_void,
    );

    addr.sin_family = AF_INET as sa_family_t;
    addr.sin_addr.s_addr = haddr;
    addr.sin_port = hport as u16;

    if bind((*so).s, &addr as *const sockaddr_in as *const sockaddr, addrlen) < 0 {
        udp_detach(so);
        return core::ptr::null_mut();
    }
    setsockopt(
        (*so).s,
        SOL_SOCKET,
        SO_REUSEADDR,
        &opt as *const socklen_t as *const core::ffi::c_void,
        core::mem::size_of::<c_int>() as socklen_t,
    );

    getsockname(
        (*so).s,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        &mut addrlen,
    );
    (*so).so_fport = addr.sin_port;
    if addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr {
        (*so).so_faddr = (*slirp).vhost_addr;
    } else {
        (*so).so_faddr = addr.sin_addr;
    }
    (*so).so_lport = lport as u16;
    (*so).so_laddr.s_addr = laddr;
    if flags != SS_FACCEPTONCE {
        (*so).so_expire = 0;
    }

    (*so).so_state &= SS_PERSISTENT_MASK;
    (*so).so_state |= SS_ISFCONNECTED | flags;

    so
}
}

// ── `struct sockaddr_in addr; struct socket *so; socklen_t addrlen = sizeof(struct sockaddr_in), opt…` · slirp/udp.c:347-351 · chunk 2905 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct sockaddr_in addr;
//      struct socket *so;
//      socklen_t addrlen = sizeof(struct sockaddr_in), opt = 1;
//     
//      so = socreate(slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2905 {
// Port of the udp_listen() prologue: create the socket via socreate().
// The surrounding C code keeps its local declarations (addr, so, addrlen, opt).

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    _private: [u8; 0],
}

extern "C" {
    fn socreate(slirp: *mut Slirp) -> *mut socket;
}

#[no_mangle]
pub extern "C" fn udp_listen__part2905(slirp: *mut Slirp) -> *mut socket {
    unsafe { socreate(slirp) }
}
}

// ── `if (!so)` · slirp/udp.c:352-354 · chunk 2906 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!so) {
//          return 
//                ((void *)0)
//                    ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2906 {
use core::ffi::c_int;

// Opaque mirror of `struct socket` (only reached through a pointer here).
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn udp_listen__part2906(so: *mut Socket) -> c_int {
    if so.is_null() {
        1
    } else {
        0
    }
}
}

// ── `so->s = os_socket( 2 , SOCK_DGRAM ,0); so->so_expire = curtime + 240000; slirp_insque(so, &slir…` · slirp/udp.c:355-361 · chunk 2907 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->s = os_socket(
//                       2
//                              ,
//                               SOCK_DGRAM
//                                         ,0);
//      so->so_expire = curtime + 240000;
//      slirp_insque(so, &slirp->udb);
//     
//      addr.sin_family = 
//                       2
//                              ;
//      addr.sin_addr.s_addr = haddr;
//      addr.sin_port = hport;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2907 {
// Port of a piece of udp_listen(): create the host socket, register the
// socket in slirp's UDP queue and fill in the local bind address.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct sockaddr_in` (16 bytes on this target).
#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16, // sa_family_t
    pub sin_port: u16,   // in_port_t
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

// Mirror of the leading part of `struct socket` (everything up to and
// including so_expire, which is all this piece touches).
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
    pub so_type: u8, // u_char
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint, // u_int
}

extern "C" {
    fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    static mut curtime: c_uint;
}

#[no_mangle]
pub extern "C" fn udp_listen__part2907(
    so: *mut Socket,
    udb: *mut c_void,
    addr: *mut SockaddrIn,
    haddr: u32,
    hport: c_uint,
) {
    unsafe {
        // so->s = os_socket(AF_INET, SOCK_DGRAM, 0);
        (*so).s = os_socket(2, 2, 0);
        // so->so_expire = curtime + 240000;
        let now = core::ptr::read(core::ptr::addr_of!(curtime));
        (*so).so_expire = now.wrapping_add(240000);
        // slirp_insque(so, &slirp->udb);
        slirp_insque(so as *mut c_void, udb);

        // addr.sin_family = AF_INET;
        (*addr).sin_family = 2;
        // addr.sin_addr.s_addr = haddr;
        (*addr).sin_addr.s_addr = haddr;
        // addr.sin_port = hport;  (u_int -> in_port_t, truncating)
        (*addr).sin_port = hport as u16;
    }
}
}

// ── `if (bind(so->s,(struct sockaddr *)&addr, addrlen) < 0)` · slirp/udp.c:363-366 · chunk 2908 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bind(so->s,(struct sockaddr *)&addr, addrlen) < 0) {
//       udp_detach(so);
//       return 
//             ((void *)0)
//                 ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2908 {
// Port of slirp/udp.c udp_listen lines 363-366 (the bind() check).
// The C side keeps the failure path (udp_detach + return NULL); this
// function only performs the bind() call and reports its result.
use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    // int bind(int fd, const struct sockaddr *addr, socklen_t len);
    fn bind(fd: c_int, addr: *const c_void, len: c_uint) -> c_int;
}

#[no_mangle]
pub extern "C" fn udp_listen__part2908(
    s: c_int,
    addr: *const c_void,
    addrlen: c_uint,
) -> c_int {
    unsafe { bind(s, addr, addrlen) }
}
}

// ── `setsockopt(so->s, 1 , 2 ,(char *)&opt,sizeof(int)); getsockname(so->s,(struct sockaddr *)&addr,…` · slirp/udp.c:367-370 · chunk 2909 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     setsockopt(so->s,
//                      1
//                                ,
//                                 2
//                                             ,(char *)&opt,sizeof(int));
//     
//      getsockname(so->s,(struct sockaddr *)&addr,&addrlen);
//      so->so_fport = addr.sin_port;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2909 {
// Ported piece of udp_listen(): SO_REUSEADDR setsockopt, getsockname and
// extraction of the bound local port for the socket.
use core::ffi::{c_int, c_void};

// socklen_t is `typedef unsigned int __socklen_t;` in this translation unit.
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
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
    fn getsockname(fd: c_int, addr: *mut c_void, addrlen: *mut SocklenT) -> c_int;
}

#[no_mangle]
pub extern "C" fn udp_listen__part2909(
    s: c_int,
    addr: *mut SockaddrIn,
    addrlen: *mut SocklenT,
    opt: *const u32,
) -> u16 {
    unsafe {
        setsockopt(
            s,
            1, // SOL_SOCKET
            2, // SO_REUSEADDR
            opt as *const c_void,
            core::mem::size_of::<c_int>() as SocklenT,
        );
        getsockname(s, addr as *mut c_void, addrlen);
        if addr.is_null() {
            return 0;
        }
        (*addr).sin_port
    }
}
}

// ── `if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr) … else` · slirp/udp.c:371-376 · chunk 2910 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr.sin_addr.s_addr == 0 ||
//          addr.sin_addr.s_addr == loopback_addr.s_addr) {
//         so->so_faddr = slirp->vhost_addr;
//      } else {
//         so->so_faddr = addr.sin_addr;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2910 {
// Port of the so_faddr selection branch from udp_listen() in slirp/udp.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn udp_listen__part2910(
    addr_sin_addr_s_addr: u32,
    vhost_addr: *const InAddr,
    so_faddr_out: *mut InAddr,
) {
    unsafe {
        let value = if addr_sin_addr_s_addr == 0
            || addr_sin_addr_s_addr == loopback_addr.s_addr
        {
            *vhost_addr
        } else {
            InAddr {
                s_addr: addr_sin_addr_s_addr,
            }
        };
        *so_faddr_out = value;
    }
}
}

// ── `so->so_lport = lport; so->so_laddr.s_addr = laddr;` · slirp/udp.c:377-378 · chunk 2911 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_lport = lport;
//      so->so_laddr.s_addr = laddr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2911 {
// Port of slirp/udp.c: so->so_lport = lport; so->so_laddr.s_addr = laddr;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
}

#[no_mangle]
pub unsafe extern "C" fn udp_listen__part2911(
    so: *mut Socket,
    lport: core::ffi::c_uint,
    laddr: u32,
) {
    if so.is_null() {
        return;
    }
    (*so).so_lport = lport as u16;
    (*so).so_laddr.s_addr = laddr;
}
}

// ── `if (flags != 0x200)` · slirp/udp.c:379-380 · chunk 2912 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags != 0x200)
//         so->so_expire = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2912 {
// Port of the `flags != 0x200` expire-reset fragment of slirp's udp_listen().
use core::ffi::{c_int, c_uint, c_uchar};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct socket` as far as the fields we need (layout identical,
// later fields omitted).
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
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: c_uint,
}

#[no_mangle]
pub extern "C" fn udp_listen__part2912(so: *mut Socket, flags: c_int) {
    if flags != 0x200 {
        unsafe {
            (*so).so_expire = 0;
        }
    }
}
}

// ── `so->so_state &= 0xf000; so->so_state |= 0x004 | flags; return so;` · slirp/udp.c:382-385 · chunk 2913 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->so_state &= 0xf000;
//      so->so_state |= 0x004 | flags;
//     
//      return so;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2913 {
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of `struct socket` up to (and including) the fields this piece uses.
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
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut core::ffi::c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn udp_listen__part2913(
    so: *mut Socket,
    flags: c_int,
) -> *mut Socket {
    let state = (*so).so_state;
    let state = state & 0xf000;
    (*so).so_state = state | (0x004 | flags);
    so
}
}

