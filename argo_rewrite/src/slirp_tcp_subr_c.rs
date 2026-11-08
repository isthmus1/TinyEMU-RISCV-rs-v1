//! From `slirp/tcp_subr.c`.

// ── `tcp_init` · slirp/tcp_subr.c:50-56 · chunk 2789 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_init(Slirp *slirp)
//     {
//         slirp->tcp_iss = 1;
//         slirp->tcb.so_next = slirp->tcb.so_prev = &slirp->tcb;
//         slirp->tcp_last_so = &slirp->tcb;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2789 {
// Rust rewrite of slirp tcp_init (slirp/tcp_subr.c).
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[no_mangle]
pub extern "C" fn tcp_init(slirp: *mut Slirp) {
    unsafe {
        (*slirp).tcp_iss = 1;
        let tcb: *mut socket = &mut (*slirp).tcb;
        (*tcb).so_next = tcb;
        (*tcb).so_prev = tcb;
        (*slirp).tcp_last_so = tcb;
    }
}
}

// ── `slirp->tcp_iss = 1; slirp->tcb.so_next = slirp->tcb.so_prev = &slirp->tcb; slirp->tcp_last_so =…` · slirp/tcp_subr.c:53-55 · chunk 2790 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->tcp_iss = 1;
//         slirp->tcb.so_next = slirp->tcb.so_prev = &slirp->tcb;
//         slirp->tcp_last_so = &slirp->tcb;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2790 {
// Port of the body of Slirp's tcp_init() (slirp/tcp_subr.c lines 53-55).
// The C `Slirp` struct is mirrored below (exact field order and types) so the
// three assignments can be performed through the raw pointer.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
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
pub struct MbHdr {
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
    pub m_hdr: MbHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
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
    pub so_ti: *mut Tcpiphdr,
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
    pub extra: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct MbPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C, packed)]
pub struct Ipvly {
    pub ih_mbuf: MbPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipvly,
    pub ti_t: Tcphdr,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
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

#[repr(C)]
pub struct Qlink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
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
    pub opaque: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn tcp_init__part2790(slirp: *mut Slirp) {
    unsafe {
        let s: *mut Slirp = slirp;
        (*s).tcp_iss = 1;
        let tcb: *mut Socket = core::ptr::addr_of_mut!((*s).tcb);
        (*tcb).so_next = tcb;
        (*tcb).so_prev = tcb;
        (*s).tcp_last_so = tcb;
    }
}
}

// ── `tcp_template` · slirp/tcp_subr.c:64-87 · chunk 2791 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_template(struct tcpcb *tp)
//     {
//      struct socket *so = tp->t_socket;
//      register struct tcpiphdr *n = &tp->t_template;
//     
//      n->ti_i.ih_mbuf.mptr = 
//                  ((void *)0)
//                      ;
//      n->ti_i.ih_x1 = 0;
//      n->ti_i.ih_pr = 
//                IPPROTO_TCP
//                           ;
//      n->ti_i.ih_len = htons(sizeof (struct tcpiphdr) - sizeof (struct ip));
//      n->ti_i.ih_src = so->so_faddr;
//      n->ti_i.ih_dst = so->so_laddr;
//      n->ti_t.th_sport = so->so_fport;
//      n->ti_t.th_dport = so->so_lport;
//     
//      n->ti_t.th_seq = 0;
//      n->ti_t.th_ack = 0;
//      n->ti_t.th_x2 = 0;
//      n->ti_t.th_off = 5;
//      n->ti_t.th_flags = 0;
//      n->ti_t.th_win = 0;
//      n->ti_t.th_sum = 0;
//      n->ti_t.th_urp = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2791 {
// Port of slirp's tcp_template(): build the skeletal tcp/ip header template
// for a connection.
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

// struct ipovly (packed in C)
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr (th_x2:4 / th_off:4 bitfields share one byte)
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// struct ip (packed in C), used only to reproduce the sizeof() arithmetic
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

// Prefix of struct socket up to so_lport (only fields read here).
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
}

// struct tcpcb: everything up to t_socket so the offsets line up.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
}

extern "C" {
    fn htons(x: u16) -> u16;
}

const IPPROTO_TCP: u8 = 6;

// th_x2 = 0, th_off = 5 packed into the shared bitfield byte.
#[cfg(target_endian = "little")]
const TH_X2_OFF_BYTE: u8 = 5u8 << 4;
#[cfg(target_endian = "big")]
const TH_X2_OFF_BYTE: u8 = 5u8;

#[no_mangle]
pub unsafe extern "C" fn tcp_template(tp: *mut Tcpcb) {
    let so = (*tp).t_socket;
    let n = &mut (*tp).t_template;

    n.ti_i.ih_mbuf.mptr = core::ptr::null_mut();
    n.ti_i.ih_x1 = 0;
    n.ti_i.ih_pr = IPPROTO_TCP;
    n.ti_i.ih_len = htons(
        (core::mem::size_of::<Tcpiphdr>() - core::mem::size_of::<Ip>()) as u16,
    );
    n.ti_i.ih_src = (*so).so_faddr;
    n.ti_i.ih_dst = (*so).so_laddr;
    n.ti_t.th_sport = (*so).so_fport;
    n.ti_t.th_dport = (*so).so_lport;

    n.ti_t.th_seq = 0;
    n.ti_t.th_ack = 0;
    n.ti_t.th_x2_off = TH_X2_OFF_BYTE;
    n.ti_t.th_flags = 0;
    n.ti_t.th_win = 0;
    n.ti_t.th_sum = 0;
    n.ti_t.th_urp = 0;
}
}

// ── `struct socket *so = tp->t_socket; register struct tcpiphdr *n = &tp->t_template; n->ti_i.ih_mbu…` · slirp/tcp_subr.c:67-86 · chunk 2792 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *so = tp->t_socket;
//      register struct tcpiphdr *n = &tp->t_template;
//     
//      n->ti_i.ih_mbuf.mptr = 
//                  ((void *)0)
//                      ;
//      n->ti_i.ih_x1 = 0;
//      n->ti_i.ih_pr = 
//                IPPROTO_TCP
//                           ;
//      n->ti_i.ih_len = htons(sizeof (struct tcpiphdr) - sizeof (struct ip));
//      n->ti_i.ih_src = so->so_faddr;
//      n->ti_i.ih_dst = so->so_laddr;
//      n->ti_t.th_sport = so->so_fport;
//      n->ti_t.th_dport = so->so_lport;
//     
//      n->ti_t.th_seq = 0;
//      n->ti_t.th_ack = 0;
//      n->ti_t.th_x2 = 0;
//      n->ti_t.th_off = 5;
//      n->ti_t.th_flags = 0;
//      n->ti_t.th_win = 0;
//      n->ti_t.th_sum = 0;
//      n->ti_t.th_urp = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2792 {
// Port of the tcp_template() template-initialization block.
// Mirrors the C types exactly as they appear in the preprocessed TU.

use core::ffi::{c_int, c_void};

// struct in_addr { in_addr_t s_addr; }  (in_addr_t == uint32_t)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

// struct ipovly { ... } __attribute__((packed));
// ih_mbuf(8) ih_x1(1) ih_pr(1) ih_len(2) ih_src(4) ih_dst(4) -> 20 bytes
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr (not packed): the anonymous bitfield pair
// `u_int th_x2:4, th_off:4;` occupies the low byte of the 4-byte unit at
// offset 12, so this byte models both nibbles (th_x2 = low, th_off = high).
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; } -> 20 + 20
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// struct socket: prefix of the C layout up to (and including) so_lport,
// which is all this code touches; field order/offsets are preserved.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut Tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

// IPPROTO_TCP
const IPPROTO_TCP: u8 = 6;

#[no_mangle]
pub unsafe extern "C" fn tcp_template__part2792(n: *mut Tcpiphdr, so: *mut Socket) {
    // n->ti_i.ih_mbuf.mptr = NULL;
    core::ptr::write_unaligned(
        core::ptr::addr_of_mut!((*n).ti_i.ih_mbuf.mptr),
        core::ptr::null_mut::<c_void>(),
    );

    // n->ti_i.ih_x1 = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_i.ih_x1), 0u8);

    // n->ti_i.ih_pr = IPPROTO_TCP;
    core::ptr::write_unaligned(
        core::ptr::addr_of_mut!((*n).ti_i.ih_pr),
        IPPROTO_TCP,
    );

    // n->ti_i.ih_len = htons(sizeof(struct tcpiphdr) - sizeof(struct ip));
    // 40 - 20 = 20
    let ih_len: u16 = htons(20u16);
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_i.ih_len), ih_len);

    // n->ti_i.ih_src = so->so_faddr;
    let faddr: InAddr = (*so).so_faddr;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_i.ih_src), faddr);

    // n->ti_i.ih_dst = so->so_laddr;
    let laddr: InAddr = (*so).so_laddr;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_i.ih_dst), laddr);

    // n->ti_t.th_sport = so->so_fport;
    let fport: u16 = (*so).so_fport;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_sport), fport);

    // n->ti_t.th_dport = so->so_lport;
    let lport: u16 = (*so).so_lport;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_dport), lport);

    // n->ti_t.th_seq = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_seq), 0u32);

    // n->ti_t.th_ack = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_ack), 0u32);

    // n->ti_t.th_x2 = 0; n->ti_t.th_off = 5;
    // Both nibbles of this byte are fully overwritten: th_x2 -> 0, th_off -> 5.
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_x2_off), 0x50u8);

    // n->ti_t.th_flags = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_flags), 0u8);

    // n->ti_t.th_win = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_win), 0u16);

    // n->ti_t.th_sum = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_sum), 0u16);

    // n->ti_t.th_urp = 0;
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*n).ti_t.th_urp), 0u16);
}
}

// ── `tcp_respond` · slirp/tcp_subr.c:102-167 · chunk 2793 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_respond(struct tcpcb *tp, struct tcpiphdr *ti, struct mbuf *m,
//                 tcp_seq ack, tcp_seq seq, int flags)
//     {
//      register int tlen;
//      int win = 0;
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
//     
//      if (tp)
//       win = ((&tp->t_socket->so_rcv)->sb_datalen - (&tp->t_socket->so_rcv)->sb_cc);
//             if (m == 
//                     ((void *)0)
//                         ) {
//       if ((m = m_get(tp->t_socket->slirp)) == 
//                                              ((void *)0)
//                                                  )
//        return;
//       tlen = 0;
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       *((struct tcpiphdr *)(m)->m_hdr.mh_data) = *ti;
//       ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);
//       flags = 0x10;
//      } else {
//     
//     
//     
//     
//       m->m_hdr.mh_data = (caddr_t)ti;
//     
//       m->m_hdr.mh_len = sizeof (struct tcpiphdr);
//       tlen = 0;
//     
//       { uint32_t t; t=ti->ti_i.ih_dst.s_addr; ti->ti_i.ih_dst.s_addr=ti->ti_i.ih_src.s_addr; ti->ti_i.ih_src.s_addr=t; };
//       { uint16_t t; t=ti->ti_t.th_dport; ti->ti_t.th_dport=ti->ti_t.th_sport; ti->ti_t.th_sport=t; };
//     
//      }
//      ti->ti_i.ih_len = htons((u_short)(sizeof (struct tcphdr) + tlen));
//      tlen += sizeof (struct tcpiphdr);
//      m->m_hdr.mh_len = tlen;
//     
//             ti->ti_i.ih_mbuf.mptr = 
//                          ((void *)0)
//                              ;
//      ti->ti_i.ih_x1 = 0;
//      ti->ti_t.th_seq = htonl(seq);
//      ti->ti_t.th_ack = htonl(ack);
//      ti->ti_t.th_x2 = 0;
//      ti->ti_t.th_off = sizeof (struct tcphdr) >> 2;
//      ti->ti_t.th_flags = flags;
//      if (tp)
//       ti->ti_t.th_win = htons((uint16_t) (win >> tp->rcv_scale));
//      else
//       ti->ti_t.th_win = htons((uint16_t)win);
//      ti->ti_t.th_urp = 0;
//      ti->ti_t.th_sum = 0;
//      ti->ti_t.th_sum = cksum(m, tlen);
//      ((struct ip *)ti)->ip_len = tlen;
//     
//      if(flags & 0x04)
//        ((struct ip *)ti)->ip_ttl = 255;
//      else
//        ((struct ip *)ti)->ip_ttl = 64;
//     
//      (void) ip_output((struct socket *)0, m);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2793 {
// Rust rewrite of slirp's tcp_respond().
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void};
use core::mem::size_of;
use core::ptr;

// Opaque types.
pub struct Slirp {
    _private: [u8; 0],
}

// struct in_addr
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct mbuf_ptr (packed); SIZEOF_CHAR_P dependent.
#[cfg(target_pointer_width = "64")]
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[cfg(target_pointer_width = "32")]
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
    pub dummy: u32,
}

// struct ipovly (packed)
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

// struct tcphdr
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // bit field storage unit for `u_int th_x2:4, th_off:4;`
    // (low byte: th_x2 in bits 0..4, th_off in bits 4..8)
    pub th_x2_off: u32,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// struct ip (packed)
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

// struct sbuf
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// struct socket
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
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

// struct tcpcb
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
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
    pub t_idle: c_short,
    pub t_rtt: c_short,
    pub t_rtseq: u32,
    pub t_srtt: c_short,
    pub t_rttvar: c_short,
    pub t_rttmin: c_ushort,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: c_short,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

// struct m_hdr
#[repr(C)]
#[derive(Clone, Copy)]
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

// union M_dat
#[repr(C)]
#[derive(Clone, Copy)]
pub union M_datU {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

// struct mbuf
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_datU,
}

const IF_MAXLINKHDR: usize = 2 + 14 + 40;
const TH_ACK: c_int = 0x10;
const TH_RST: c_int = 0x04;
const MAXTTL: u8 = 255;
const IPDEFTTL: u8 = 64;

extern "C" {
    fn m_get(slirp: *mut Slirp) -> *mut Mbuf;
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
    fn ip_output(so: *mut Socket, m: *mut Mbuf) -> c_int;
    fn htons(v: u16) -> u16;
    fn htonl(v: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_respond(
    tp: *mut Tcpcb,
    ti_in: *mut Tcpiphdr,
    m_in: *mut Mbuf,
    ack: u32,
    seq: u32,
    flags_in: c_int,
) {
    let mut ti: *mut Tcpiphdr = ti_in;
    let mut m: *mut Mbuf = m_in;
    let mut flags: c_int = flags_in;
    let mut tlen: c_int;
    let mut win: c_int = 0;

    if !tp.is_null() {
        // sbspace(&tp->t_socket->so_rcv)
        let so = (*tp).t_socket;
        let space = (*so).so_rcv.sb_datalen.wrapping_sub((*so).so_rcv.sb_cc);
        win = space as c_int;
    }

    if m.is_null() {
        m = m_get((*(*tp).t_socket).slirp);
        if m.is_null() {
            return;
        }
        tlen = 0;
        let data = (*m).m_hdr.mh_data.add(IF_MAXLINKHDR);
        (*m).m_hdr.mh_data = data;
        // *mtod(m, struct tcpiphdr *) = *ti;
        ptr::copy_nonoverlapping(ti as *const u8, data as *mut u8, size_of::<Tcpiphdr>());
        ti = data as *mut Tcpiphdr;
        flags = TH_ACK;
    } else {
        // ti points into m; make the mbuf point to ti
        (*m).m_hdr.mh_data = ti as *mut c_char;
        (*m).m_hdr.mh_len = size_of::<Tcpiphdr>() as c_int;
        tlen = 0;

        let p_dst = ptr::addr_of_mut!((*ti).ti_i.ih_dst.s_addr);
        let p_src = ptr::addr_of_mut!((*ti).ti_i.ih_src.s_addr);
        let t = ptr::read_unaligned(p_dst);
        ptr::write_unaligned(p_dst, ptr::read_unaligned(p_src));
        ptr::write_unaligned(p_src, t);

        let t2: u16 = (*ti).ti_t.th_dport;
        (*ti).ti_t.th_dport = (*ti).ti_t.th_sport;
        (*ti).ti_t.th_sport = t2;
    }

    ptr::write_unaligned(
        ptr::addr_of_mut!((*ti).ti_i.ih_len),
        htons((size_of::<Tcphdr>() as c_int + tlen) as u16),
    );
    tlen += size_of::<Tcpiphdr>() as c_int;
    (*m).m_hdr.mh_len = tlen;

    ptr::write_unaligned(
        ptr::addr_of_mut!((*ti).ti_i.ih_mbuf.mptr),
        ptr::null_mut::<Mbuf>(),
    );
    ptr::write_unaligned(ptr::addr_of_mut!((*ti).ti_i.ih_x1), 0u8);
    (*ti).ti_t.th_seq = htonl(seq);
    (*ti).ti_t.th_ack = htonl(ack);
    // ti_x2 = 0; ti_off = sizeof(struct tcphdr) >> 2;
    let low: u32 = (0u32 & 0xF) | ((((size_of::<Tcphdr>() as u32) >> 2) & 0xF) << 4);
    let old: u32 = (*ti).ti_t.th_x2_off;
    (*ti).ti_t.th_x2_off = (old & !0xFFu32) | low;
    (*ti).ti_t.th_flags = flags as u8;
    if !tp.is_null() {
        let scale = (*tp).rcv_scale as u32;
        let shifted: c_int = win >> (scale & 31);
        (*ti).ti_t.th_win = htons(shifted as u16);
    } else {
        (*ti).ti_t.th_win = htons(win as u16);
    }
    (*ti).ti_t.th_urp = 0;
    (*ti).ti_t.th_sum = 0;
    (*ti).ti_t.th_sum = cksum(m, tlen) as u16;
    let ip = ti as *mut Ip;
    ptr::write_unaligned(ptr::addr_of_mut!((*ip).ip_len), tlen as u16);

    if (flags & TH_RST) != 0 {
        ptr::write_unaligned(ptr::addr_of_mut!((*ip).ip_ttl), MAXTTL);
    } else {
        ptr::write_unaligned(ptr::addr_of_mut!((*ip).ip_ttl), IPDEFTTL);
    }

    let _ = ip_output(ptr::null_mut::<Socket>(), m);
}
}

// ── `register int tlen; int win = 0; ; ; ; ; ; ; ;` · slirp/tcp_subr.c:106-115 · chunk 2794 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register int tlen;
//      int win = 0;
//     
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2794 {
// Ported piece of tcp_respond(): the prologue declarations.
// The original C target was:
//     register int tlen;
//     int win = 0;
//     ; ; ; ; ; ; ;
// The seven empty statements are no-ops, so the only observable effect on the
// rest of tcp_respond() is the initial value of `win`. `tlen` is declared
// (uninitialised) in C and assigned in both branches before it is read.

#[no_mangle]
pub extern "C" fn tcp_respond__part2794() -> core::ffi::c_int {
    // int win = 0;
    0 as core::ffi::c_int
}
}

// ── `if (tp)` · slirp/tcp_subr.c:117-118 · chunk 2795 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (tp)
//       win = ((&tp->t_socket->so_rcv)->sb_datalen - (&tp->t_socket->so_rcv)->sb_cc);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2795 {
// Port of the receive-window computation of tcp_respond:
//   if (tp) win = tp->t_socket->so_rcv.sb_datalen - tp->t_socket->so_rcv.sb_cc;
use core::ffi::{c_char, c_int, c_uint};

// struct sbuf {
//  u_int sb_cc;
//  u_int sb_datalen;
//  char *sb_wptr;
//  char *sb_rptr;
//  char *sb_data;
// };
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2795(so_rcv: *mut Sbuf) -> c_int {
    if so_rcv.is_null() {
        return 0;
    }
    unsafe {
        // unsigned subtraction (u_int) then converted to int, as in C
        let datalen: c_uint = (*so_rcv).sb_datalen;
        let cc: c_uint = (*so_rcv).sb_cc;
        datalen.wrapping_sub(cc) as c_int
    }
}
}

// ── `if (m == ((void *)0) … else` · slirp/tcp_subr.c:119-140 · chunk 2796 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (m == 
//                     ((void *)0)
//                         ) {
//       if ((m = m_get(tp->t_socket->slirp)) == 
//                                              ((void *)0)
//                                                  )
//        return;
//       tlen = 0;
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       *((struct tcpiphdr *)(m)->m_hdr.mh_data) = *ti;
//       ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);
//       flags = 0x10;
//      } else {
//     
//     
//     
//     
//       m->m_hdr.mh_data = (caddr_t)ti;
//     
//       m->m_hdr.mh_len = sizeof (struct tcpiphdr);
//       tlen = 0;
//     
//       { uint32_t t; t=ti->ti_i.ih_dst.s_addr; ti->ti_i.ih_dst.s_addr=ti->ti_i.ih_src.s_addr; ti->ti_i.ih_src.s_addr=t; };
//       { uint16_t t; t=ti->ti_t.th_dport; ti->ti_t.th_dport=ti->ti_t.th_sport; ti->ti_t.th_sport=t; };
//     
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2796 {
// Port of the "m == NULL ? m = m_get(...) : set header and swap addresses" part
// of slirp's tcp_respond().
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

// Opaque here: only pointers to it are passed around.
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // `u_int th_x2:4, th_off:4;` -- one 4-byte bit-field unit, never touched here.
    pub th_x2_off: u32,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
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
    pub extra: *mut core::ffi::c_void,
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
pub struct tcpcb {
    _opaque: [u8; 0],
}

// Exact mirror of `struct tcpcb` up to t_socket (the only field read here).
#[repr(C)]
pub struct tcpcb_head {
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
}

extern "C" {
    fn m_get(slirp: *mut Slirp) -> *mut mbuf;
}

/// Returns 0 when the original code fell through (and always leaves the TCP
/// header length at 0), or 1 when it did `return` early on m_get() failure.
#[no_mangle]
pub extern "C" fn tcp_respond__part2796(
    tp: *mut tcpcb_head,
    ti: *mut *mut tcpiphdr,
    m: *mut *mut mbuf,
    flags: *mut c_int,
) -> c_int {
    unsafe {
        let mut mp: *mut mbuf = *m;
        let mut tip: *mut tcpiphdr = *ti;
        let mut fl: c_int = *flags;

        if mp.is_null() {
            let so: *mut socket = (*tp).t_socket;
            mp = m_get((*so).slirp);
            if mp.is_null() {
                return 1;
            }
            (*mp).m_hdr.mh_data = (*mp).m_hdr.mh_data.add(2 + 14 + 40);
            let dst: *mut u8 = (*mp).m_hdr.mh_data as *mut u8;
            core::ptr::copy_nonoverlapping(
                tip as *const u8,
                dst,
                core::mem::size_of::<tcpiphdr>(),
            );
            tip = dst as *mut tcpiphdr;
            fl = 0x10;
        } else {
            (*mp).m_hdr.mh_data = tip as *mut c_char;
            (*mp).m_hdr.mh_len = core::mem::size_of::<tcpiphdr>() as c_int;

            let base: *mut u8 = tip as *mut u8;
            let off_src: usize = core::mem::size_of::<mbuf_ptr>() + 4; // ih_src.s_addr
            let off_dst: usize = off_src + 4; // ih_dst.s_addr
            let off_sport: usize = core::mem::size_of::<ipovly>(); // ti_t.th_sport
            let off_dport: usize = off_sport + 2; // ti_t.th_dport

            let a: u32 = core::ptr::read_unaligned(base.add(off_dst) as *const u32);
            let b: u32 = core::ptr::read_unaligned(base.add(off_src) as *const u32);
            core::ptr::write_unaligned(base.add(off_dst) as *mut u32, b);
            core::ptr::write_unaligned(base.add(off_src) as *mut u32, a);

            let p: u16 = core::ptr::read_unaligned(base.add(off_dport) as *const u16);
            let q: u16 = core::ptr::read_unaligned(base.add(off_sport) as *const u16);
            core::ptr::write_unaligned(base.add(off_dport) as *mut u16, q);
            core::ptr::write_unaligned(base.add(off_sport) as *mut u16, p);
        }

        *ti = tip;
        *m = mp;
        *flags = fl;
        0
    }
}
}

// ── `ti->ti_i.ih_len = htons((u_short)(sizeof (struct tcphdr) + tlen)); tlen += sizeof (struct tcpip…` · slirp/tcp_subr.c:141-151 · chunk 2797 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_i.ih_len = htons((u_short)(sizeof (struct tcphdr) + tlen));
//      tlen += sizeof (struct tcpiphdr);
//      m->m_hdr.mh_len = tlen;
//     
//             ti->ti_i.ih_mbuf.mptr = 
//                          ((void *)0)
//                              ;
//      ti->ti_i.ih_x1 = 0;
//      ti->ti_t.th_seq = htonl(seq);
//      ti->ti_t.th_ack = htonl(ack);
//      ti->ti_t.th_x2 = 0;
//      ti->ti_t.th_off = sizeof (struct tcphdr) >> 2;
//      ti->ti_t.th_flags = flags;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2797 {
// Ported piece of slirp tcp_respond(): header/seq/ack/flags fixups.
// `tlen` is a C register variable, so it is passed and returned by value.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
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
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4;  (one byte on little endian)
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
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

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2797(
    ti: *mut Tcpiphdr,
    m: *mut Mbuf,
    seq: u32,
    ack: u32,
    flags: c_int,
    tlen: c_int,
) -> c_int {
    unsafe {
        let t = &mut *ti;

        // ti->ti_i.ih_len = htons((u_short)(sizeof(struct tcphdr) + tlen));
        // sizeof() has type size_t, so the addition is done in unsigned long.
        let ip_len = (core::mem::size_of::<Tcphdr>() as u64).wrapping_add(tlen as i64 as u64);
        t.ti_i.ih_len = htons(ip_len as u16);

        // tlen += sizeof(struct tcpiphdr);
        let tlen = tlen.wrapping_add(core::mem::size_of::<Tcpiphdr>() as c_int);

        // m->m_hdr.mh_len = tlen;
        (*m).m_hdr.mh_len = tlen;

        // ti->ti_i.ih_mbuf.mptr = NULL;
        t.ti_i.ih_mbuf.mptr = core::ptr::null_mut();

        // ti->ti_i.ih_x1 = 0;
        t.ti_i.ih_x1 = 0;

        // ti->ti_t.th_seq = htonl(seq);
        t.ti_t.th_seq = htonl(seq);

        // ti->ti_t.th_ack = htonl(ack);
        t.ti_t.th_ack = htonl(ack);

        // ti->ti_t.th_x2 = 0;  ti->ti_t.th_off = sizeof(struct tcphdr) >> 2;
        let mut byte = t.ti_t.th_x2_off;
        byte &= 0xf0u8; // th_x2 (low nibble) = 0
        let off = ((core::mem::size_of::<Tcphdr>() >> 2) as u8) & 0x0f;
        byte = (byte & 0x0f) | (off << 4); // th_off (high nibble)
        t.ti_t.th_x2_off = byte;

        // ti->ti_t.th_flags = flags;
        t.ti_t.th_flags = flags as u8;

        tlen
    }
}
}

// ── `if (tp) … else` · slirp/tcp_subr.c:152-155 · chunk 2798 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp)
//       ti->ti_t.th_win = htons((uint16_t) (win >> tp->rcv_scale));
//      else
//       ti->ti_t.th_win = htons((uint16_t)win);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2798 {
// Piece of tcp_respond(): set the window field of the TCP header.
use core::ffi::{c_int, c_void};

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut c_void,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
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
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4  -> one unsigned int on the target ABI
    pub th_x2_off: u32,
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
pub struct tcpcb {
    pub seg_next: *mut tcpiphdr,
    pub seg_prev: *mut tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: i8,
    pub t_flags: u16,
    pub t_template: tcpiphdr,
    pub t_socket: *mut c_void,
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
    pub t_oobflags: i8,
    pub t_iobc: i8,
    pub t_softerror: i16,
    pub snd_scale: u8,
    pub rcv_scale: u8,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2798(tp: *mut tcpcb, ti: *mut tcpiphdr, win: c_int) {
    unsafe {
        if !tp.is_null() {
            let scale: u32 = (*tp).rcv_scale as u32;
            let shifted: i32 = win.wrapping_shr(scale);
            (*ti).ti_t.th_win = htons(shifted as u16);
        } else {
            (*ti).ti_t.th_win = htons(win as u16);
        }
    }
}
}

// ── `ti->ti_t.th_urp = 0; ti->ti_t.th_sum = 0; ti->ti_t.th_sum = cksum(m, tlen); ((struct ip *)ti)->…` · slirp/tcp_subr.c:156-159 · chunk 2799 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_t.th_urp = 0;
//      ti->ti_t.th_sum = 0;
//      ti->ti_t.th_sum = cksum(m, tlen);
//      ((struct ip *)ti)->ip_len = tlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2799 {
// Port of the checksum/finalisation part of tcp_respond() (slirp/tcp_subr.c).
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_int;

// struct in_addr { uint32_t s_addr; }
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
}

// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

// struct tcphdr (not packed: the u_int:4 bitfields occupy a 4 byte unit)
#[repr(C)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_th_off: u32, // u_int th_x2:4, th_off:4
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

// struct ip { ... } __attribute__((packed));
#[repr(C, packed)]
pub struct ip {
    pub ip_hl_ip_v: u8, // u_int ip_hl:4, ip_v:4
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

// Opaque mirror of `struct mbuf`; only ever handled through a pointer.
#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut core::ffi::c_void,
    pub m_dat: [u8; 0],
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
    pub mh_data: *mut core::ffi::c_char,
    pub mh_len: c_int,
}

extern "C" {
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2799(ti: *mut tcpiphdr, m: *mut mbuf, tlen: c_int) {
    unsafe {
        let t: &mut tcpiphdr = &mut *ti;
        t.ti_t.th_urp = 0;
        t.ti_t.th_sum = 0;
        let sum: c_int = cksum(m, tlen);
        t.ti_t.th_sum = sum as u16;
        let iph: *mut ip = ti as *mut ip;
        (*iph).ip_len = tlen as u16;
    }
}
}

// ── `if (flags & 0x04) … else` · slirp/tcp_subr.c:161-164 · chunk 2800 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(flags & 0x04)
//        ((struct ip *)ti)->ip_ttl = 255;
//      else
//        ((struct ip *)ti)->ip_ttl = 64;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2800 {
use core::ffi::{c_int, c_void};

// Mirror of the packed C `struct ip` (offsets matter: ip_ttl is at byte 8).
#[allow(dead_code)]
#[repr(C, packed)]
struct Ip {
    ip_hl_v: u8,
    ip_tos: u8,
    ip_len: u16,
    ip_id: u16,
    ip_off: u16,
    ip_ttl: u8,
    ip_p: u8,
    ip_sum: u16,
    ip_src: [u8; 4],
    ip_dst: [u8; 4],
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2800(ti: *mut c_void, flags: c_int) {
    let ip = ti as *mut Ip;
    let ttl: u8 = if (flags & 0x04) != 0 { 255 } else { 64 };
    unsafe {
        core::ptr::addr_of_mut!((*ip).ip_ttl).write(ttl);
    }
}
}

// ── `(void) ip_output((struct socket *)0, m);` · slirp/tcp_subr.c:166-166 · chunk 2801 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     (void) ip_output((struct socket *)0, m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2801 {
// Port of the tail call `(void) ip_output((struct socket *)0, m);`
// from slirp's tcp_respond().

use core::ffi::c_int;

/// Opaque mirror of `struct mbuf`; only ever handled through a pointer.
#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

extern "C" {
    fn ip_output(so: *mut core::ffi::c_void, m: *mut mbuf) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_respond__part2801(m: *mut mbuf) {
    unsafe {
        ip_output(core::ptr::null_mut(), m);
    }
}
}

// ── `tcp_newtcpcb` · slirp/tcp_subr.c:174-210 · chunk 2802 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     struct tcpcb *
//     tcp_newtcpcb(struct socket *so)
//     {
//      register struct tcpcb *tp;
//     
//      tp = (struct tcpcb *)malloc(sizeof(*tp));
//      if (tp == 
//               ((void *)0)
//                   )
//       return ((struct tcpcb *)0);
//     
//      memset((char *) tp, 0, sizeof(struct tcpcb));
//      tp->seg_next = tp->seg_prev = (struct tcpiphdr*)tp;
//      tp->t_maxseg = 1460;
//     
//      tp->t_flags = 0 ? (0x0020|0x0080) : 0;
//      tp->t_socket = so;
//     
//     
//     
//     
//     
//     
//      tp->t_srtt = 0;
//      tp->t_rttvar = ( 3*2) << 2;
//      tp->t_rttmin = ( 1*2);
//     
//      { (tp->t_rxtcur) = (((0 >> 2) + (( 3*2) << 2)) >> 1); if ((tp->t_rxtcur) < (( 1*2))) (tp->t_rxtcur) = (( 1*2)); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
//     
//                                    ;
//     
//      tp->snd_cwnd = 65535 << 14;
//      tp->snd_ssthresh = 65535 << 14;
//      tp->t_state = 0;
//     
//      so->so_tcpcb = tp;
//     
//      return (tp);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2802 {
// Rust rewrite of slirp/tcp_subr.c:tcp_newtcpcb
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

// typedef uint32_t in_addr_t;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipovly {
    pub ih_mbuf_mptr: *mut c_void,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

// typedef uint32_t tcp_seq;
// struct tcphdr { ...; u_int th_x2:4, th_off:4; uint8_t th_flags; ... }
// The 4+4 bitfield occupies one 4-byte allocation unit whose first byte holds
// the x2/off bits; th_flags lands in the next byte of that unit.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
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
    pub so_m: *mut c_void,
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

#[no_mangle]
pub unsafe extern "C" fn tcp_newtcpcb(so: *mut socket) -> *mut tcpcb {
    let tp = malloc(core::mem::size_of::<tcpcb>()) as *mut tcpcb;
    if tp.is_null() {
        return core::ptr::null_mut::<tcpcb>();
    }

    memset(tp as *mut c_void, 0, core::mem::size_of::<tcpcb>());

    (*tp).seg_next = tp as *mut tcpiphdr;
    (*tp).seg_prev = tp as *mut tcpiphdr;
    (*tp).t_maxseg = 1460;

    // TCP_DO_RFC1323 ? (TF_REQ_SCALE|TF_REQ_TSTMP) : 0 -- TCP_DO_RFC1323 is 0.
    (*tp).t_flags = 0;
    (*tp).t_socket = so;

    // Srtt is TCPTV_SRTTBASE (0) so we know we have no rtt estimate.
    (*tp).t_srtt = 0;
    (*tp).t_rttvar = (3i16 * 2) << 2;
    (*tp).t_rttmin = (1u16 * 2);

    // TCPT_RANGESET(t_rxtcur,
    //   ((TCPTV_SRTTBASE >> 2) + (TCPTV_SRTTDFLT << 2)) >> 1,
    //   TCPTV_MIN, TCPTV_REXMTMAX)
    {
        let mut v: i16 = ((0i16 >> 2) + ((3i16 * 2) << 2)) >> 1;
        if v < (1i16 * 2) {
            v = 1i16 * 2;
        } else if v > (12i16 * 2) {
            v = 12i16 * 2;
        }
        (*tp).t_rxtcur = v;
    }

    (*tp).snd_cwnd = (65535u32) << 14;
    (*tp).snd_ssthresh = (65535u32) << 14;
    (*tp).t_state = 0;

    (*so).so_tcpcb = tp;

    tp
}
}

// ── `register struct tcpcb *tp; tp = (struct tcpcb *)malloc(sizeof(*tp));` · slirp/tcp_subr.c:177-179 · chunk 2803 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct tcpcb *tp;
//     
//      tp = (struct tcpcb *)malloc(sizeof(*tp));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2803 {
// Port of the allocation part of tcp_newtcpcb(): allocate a zero-sized-agnostic
// block of `size` bytes for the struct tcpcb, exactly like C's malloc().
use core::ffi::c_void;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn tcp_newtcpcb__part2803(size: usize) -> *mut c_void {
    unsafe { malloc(size) }
}
}

// ── `if (tp == ((void *)0)` · slirp/tcp_subr.c:180-181 · chunk 2804 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp == 
//               ((void *)0)
//                   )
//       return ((struct tcpcb *)0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2804 {
// Returns nonzero when the freshly allocated tcpcb pointer is NULL.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn tcp_newtcpcb__part2804(tp: *mut c_void) -> c_int {
    if tp.is_null() {
        1
    } else {
        0
    }
}
}

// ── `memset((char *) tp, 0, sizeof(struct tcpcb)); tp->seg_next = tp->seg_prev = (struct tcpiphdr*)t…` · slirp/tcp_subr.c:183-197 · chunk 2805 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset((char *) tp, 0, sizeof(struct tcpcb));
//      tp->seg_next = tp->seg_prev = (struct tcpiphdr*)tp;
//      tp->t_maxseg = 1460;
//     
//      tp->t_flags = 0 ? (0x0020|0x0080) : 0;
//      tp->t_socket = so;
//     
//     
//     
//     
//     
//     
//      tp->t_srtt = 0;
//      tp->t_rttvar = ( 3*2) << 2;
//      tp->t_rttmin = ( 1*2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2805 {
// Portion of tcp_newtcpcb(): zero the freshly allocated tcpcb and set up
// its initial sequence/linkage, MSS, flags, socket and RTT variables.

use core::ffi::{c_char, c_short, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
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
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4; occupy one unsigned int unit
    pub th_x2_off: u32,
    pub th_flags: u8,
    pub _pad0: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut c_void,
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
    pub t_idle: c_short,
    pub t_rtt: c_short,
    pub t_rtseq: u32,
    pub t_srtt: c_short,
    pub t_rttvar: c_short,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: c_short,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

#[no_mangle]
pub extern "C" fn tcp_newtcpcb__part2805(tp: *mut Tcpcb, so: *mut c_void) {
    if tp.is_null() {
        return;
    }
    unsafe {
        core::ptr::write_bytes(tp as *mut u8, 0, core::mem::size_of::<Tcpcb>());
        let self_ti = tp as *mut Tcpiphdr;
        (*tp).seg_next = self_ti;
        (*tp).seg_prev = self_ti;
        (*tp).t_maxseg = 1460;
        (*tp).t_flags = 0;
        (*tp).t_socket = so;
        (*tp).t_srtt = 0;
        (*tp).t_rttvar = (3 * 2) << 2;
        (*tp).t_rttmin = 1 * 2;
    }
}
}

// ── `{ … }` · slirp/tcp_subr.c:199-199 · chunk 2806 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     { (tp->t_rxtcur) = (((0 >> 2) + (( 3*2) << 2)) >> 1); if ((tp->t_rxtcur) < (( 1*2))) (tp->t_rxtcur) = (( 1*2)); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2806 {
// Port of the TCPT_RANGESET(tp->t_rxtcur, ...) piece of tcp_newtcpcb.
use core::ffi::{c_int, c_short};

#[no_mangle]
pub extern "C" fn tcp_newtcpcb__part2806(t_rxtcur: *mut c_short) {
    unsafe {
        if t_rxtcur.is_null() {
            return;
        }
        // ((TCPTV_SRTTBASE >> 2) + (TCPTV_SRTTDFLT << 2)) >> 1
        let mut v: c_int = (((0 >> 2) + ((3 * 2) << 2)) >> 1) as c_int;
        // clamp to [TCPTV_MIN, TCPTV_REXMTMAX]
        if v < (1 * 2) {
            v = 1 * 2;
        } else if v > (12 * 2) {
            v = 12 * 2;
        }
        *t_rxtcur = v as c_short;
    }
}
}

// ── `; tp->snd_cwnd = 65535 << 14; tp->snd_ssthresh = 65535 << 14; tp->t_state = 0; so->so_tcpcb = t…` · slirp/tcp_subr.c:201-209 · chunk 2807 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//     
//      tp->snd_cwnd = 65535 << 14;
//      tp->snd_ssthresh = 65535 << 14;
//      tp->t_state = 0;
//     
//      so->so_tcpcb = tp;
//     
//      return (tp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2807 {
// Port of the tail of tcp_newtcpcb: initialize the congestion window fields,
// state, back-link the tcpcb into the socket and return it.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct InAddr {
    s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    mptr: *mut c_void,
}

#[repr(C, packed)]
pub struct Ipovly {
    ih_mbuf: MbufPtr,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: InAddr,
    ih_dst: InAddr,
}

#[repr(C)]
pub struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    th_x2_th_off: u32,
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

#[repr(C)]
pub struct TcpCb {
    seg_next: *mut Tcpiphdr,
    seg_prev: *mut Tcpiphdr,
    t_state: i16,
    t_timer: [i16; 4],
    t_rxtshift: i16,
    t_rxtcur: i16,
    t_dupacks: i16,
    t_maxseg: u16,
    t_force: c_char,
    t_flags: u16,
    t_template: Tcpiphdr,
    t_socket: *mut Socket,
    snd_una: u32,
    snd_nxt: u32,
    snd_up: u32,
    snd_wl1: u32,
    snd_wl2: u32,
    iss: u32,
    snd_wnd: u32,
    rcv_wnd: u32,
    rcv_nxt: u32,
    rcv_up: u32,
    irs: u32,
    rcv_adv: u32,
    snd_max: u32,
    snd_cwnd: u32,
    snd_ssthresh: u32,
    t_idle: i16,
    t_rtt: i16,
    t_rtseq: u32,
    t_srtt: i16,
    t_rttvar: i16,
    t_rttmin: u16,
    max_sndwnd: u32,
    t_oobflags: c_char,
    t_iobc: c_char,
    t_softerror: i16,
    snd_scale: u8,
    rcv_scale: u8,
    request_r_scale: u8,
    requested_s_scale: u8,
    ts_recent: u32,
    ts_recent_age: u32,
    last_ack_sent: u32,
}

#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut Tcpiphdr,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut TcpCb,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_newtcpcb__part2807(
    so: *mut Socket,
    tp: *mut TcpCb,
) -> *mut TcpCb {
    (*tp).snd_cwnd = 65535u32 << 14;
    (*tp).snd_ssthresh = 65535u32 << 14;
    (*tp).t_state = 0;
    (*so).so_tcpcb = tp;
    tp
}
}

// ── `tcp_drop` · slirp/tcp_subr.c:217-228 · chunk 2808 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct tcpcb *tcp_drop(struct tcpcb *tp, int err)
//     {
//      ;
//      ;
//      ;
//     
//      if (((tp->t_state) >= 3)) {
//       tp->t_state = 0;
//       (void) tcp_output(tp);
//      }
//      return (tcp_close(tp));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2808 {
// Rewrite of slirp's tcp_drop() in Rust.
// Mirrors only the leading fields of the C struct tcpcb that are accessed here.

use core::ffi::{c_int, c_short};

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: c_short,
    // remaining fields are not touched by tcp_drop()
}

// TCPS_CLOSED
const TCPS_CLOSED: c_short = 0;
// TCPS_SYN_RECEIVED; TCPS_HAVERCVDSYN(s) is (s) >= TCPS_SYN_RECEIVED
const TCPS_SYN_RECEIVED: c_short = 3;

extern "C" {
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_drop(tp: *mut Tcpcb, err: c_int) -> *mut Tcpcb {
    let _ = err;

    if tp.is_null() {
        return tcp_close(tp);
    }

    if (*tp).t_state >= TCPS_SYN_RECEIVED {
        (*tp).t_state = TCPS_CLOSED;
        let _ = tcp_output(tp);
    }
    tcp_close(tp)
}
}

// ── `; ; ;` · slirp/tcp_subr.c:219-221 · chunk 2809 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2809 {
// Ported piece of tcp_drop(): the DEBUG_CALL/DEBUG_ARG statements.
// In this build the debugging macros expand to nothing, so the ported
// chunk has no side effects; it only exists as the call target.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn tcp_drop__part2809(_tp: *mut c_void, _err: c_int) {
    // DEBUG_CALL("tcp_drop");            -> expands to nothing
    // DEBUG_ARG("tp = %lx", (long)tp);   -> expands to nothing
    // DEBUG_ARG("errno = %d", errno);    -> expands to nothing
}
}

// ── `if (tp->t_state) >= 3)` · slirp/tcp_subr.c:223-226 · chunk 2810 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (((tp->t_state) >= 3)) {
//       tp->t_state = 0;
//       (void) tcp_output(tp);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2810 {
// Ported piece of tcp_drop: close-out branch that resets the TCP state
// and flushes output before the socket is torn down.
use core::ffi::{c_int, c_short, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
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
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4  -> one unsigned int allocation unit
    pub th_x2_off: u32,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// Mirror of `struct tcpcb` (only t_state is touched, but keep the layout).
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: u16,
    pub t_force: i8,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut c_void,
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
    pub t_idle: c_short,
    pub t_rtt: c_short,
    pub t_rtseq: u32,
    pub t_srtt: c_short,
    pub t_rttvar: c_short,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: i8,
    pub t_iobc: i8,
    pub t_softerror: c_short,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

extern "C" {
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_drop__part2810(tp: *mut Tcpcb) {
    if tp.is_null() {
        return;
    }
    unsafe {
        let p = &mut *tp;
        if p.t_state >= 3 {
            p.t_state = 0;
            (*tp).t_state = 0;
            let _ = tcp_output(tp);
        }
    }
}
}

// ── `return (tcp_close(tp));` · slirp/tcp_subr.c:227-227 · chunk 2811 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (tcp_close(tp));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2811 {
// Port of the `return (tcp_close(tp));` statement of tcp_drop().
use core::ffi::c_void;

// Opaque mirror of `struct tcpcb`; only pointers to it cross this boundary.
#[repr(C)]
pub struct tcpcb {
    _opaque: [u8; 0],
}

extern "C" {
    fn tcp_close(tp: *mut tcpcb) -> *mut tcpcb;
}

#[no_mangle]
pub extern "C" fn tcp_drop__part2811(tp: *mut tcpcb) -> *mut tcpcb {
    let _ = &mut () as *mut () as *mut c_void; // keep c_void import meaningful
    unsafe { tcp_close(tp) }
}
}

// ── `tcp_close` · slirp/tcp_subr.c:236-265 · chunk 2812 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct tcpcb *
//     tcp_close(struct tcpcb *tp)
//     {
//      register struct tcpiphdr *t;
//      struct socket *so = tp->t_socket;
//      Slirp *slirp = so->slirp;
//      register struct mbuf *m;
//     
//      ;
//      ;
//     
//     
//      t = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));
//      while (!(((struct qlink*)(((char*)(t)) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       t = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       m = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)))->ti_i.ih_mbuf.mptr;
//       slirp_remque(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
//       m_free(m);
//      }
//      free(tp);
//             so->so_tcpcb = 
//                           ((void *)0)
//                               ;
//     
//      if (so == slirp->tcp_last_so)
//       slirp->tcp_last_so = &slirp->tcb;
//      close(so->s);
//      sbfree(&so->so_rcv);
//      sbfree(&so->so_snd);
//      sofree(so);
//      return ((struct tcpcb *)0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2812 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Rust port of slirp/tcp_subr.c:tcp_close()
//
// struct tcpcb *tcp_close(struct tcpcb *tp)

use core::ffi::{c_char, c_int, c_long, c_uint, c_ushort, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// struct qlink { void *next, *prev; };
#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

// struct ipovly, packed
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr (the bitfield "u_int th_x2:4, th_off:4" is kept as a
// full c_uint so the layout matches; no field of it is touched here)
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: c_uint,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// struct m_hdr
#[repr(C)]
pub struct Mhdr {
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

// struct mbuf
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: Mhdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

// struct sbuf
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// struct socket
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
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

// struct ipq, packed
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

// typedef struct { uint16_t allocated; uint8_t macaddr[6]; } BOOTPClient;
#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

// struct tftp_session
#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

// struct ex_list
#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

// struct Slirp
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
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

// struct tcpcb
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
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
    pub t_rttmin: c_ushort,
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
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn m_free(m: *mut Mbuf);
    fn sbfree(sb: *mut Sbuf);
    fn sofree(so: *mut Socket);
    fn slirp_remque(a: *mut c_void);
}

// #define tcpiphdr2qlink(T) ((struct qlink*)(((char*)(T)) - sizeof(struct qlink)))
#[inline]
unsafe fn tcpiphdr2qlink(t: *mut Tcpiphdr) -> *mut Qlink {
    (t as *mut u8).sub(core::mem::size_of::<Qlink>()) as *mut Qlink
}

// #define qlink2tcpiphdr(Q) ((struct tcpiphdr*)(((char*)(Q)) + sizeof(struct qlink)))
#[inline]
unsafe fn qlink2tcpiphdr(q: *mut Qlink) -> *mut Tcpiphdr {
    (q as *mut u8).add(core::mem::size_of::<Qlink>()) as *mut Tcpiphdr
}

// #define tcpiphdr_next(T) qlink2tcpiphdr(tcpiphdr2qlink(T)->next)
#[inline]
unsafe fn tcpiphdr_next(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).next as *mut Qlink)
}

// #define tcpiphdr_prev(T) qlink2tcpiphdr(tcpiphdr2qlink(T)->prev)
#[inline]
unsafe fn tcpiphdr_prev(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).prev as *mut Qlink)
}

#[no_mangle]
pub extern "C" fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb {
    unsafe {
        let so: *mut Socket = (*tp).t_socket;
        let slirp: *mut Slirp = (*so).slirp;

        // free the reassembly queue, if any
        let mut t: *mut Tcpiphdr = qlink2tcpiphdr((*tp).seg_next as *mut Qlink);
        while (tcpiphdr2qlink(t) as *mut c_void) != (tp as *mut c_void) {
            t = tcpiphdr_next(t);
            let prev: *mut Tcpiphdr = tcpiphdr_prev(t);
            let m: *mut Mbuf =
                core::ptr::read_unaligned(core::ptr::addr_of!((*prev).ti_i.ih_mbuf.mptr));
            slirp_remque(tcpiphdr2qlink(prev) as *mut c_void);
            m_free(m);
        }
        free(tp as *mut c_void);
        (*so).so_tcpcb = core::ptr::null_mut();
        // clobber input socket cache if we're closing the cached connection
        if so == (*slirp).tcp_last_so {
            (*slirp).tcp_last_so = core::ptr::addr_of_mut!((*slirp).tcb);
        }
        close((*so).s);
        sbfree(core::ptr::addr_of_mut!((*so).so_rcv));
        sbfree(core::ptr::addr_of_mut!((*so).so_snd));
        sofree(so);
        core::ptr::null_mut()
    }
}
}

// ── `register struct tcpiphdr *t; struct socket *so = tp->t_socket; Slirp *slirp = so->slirp; regist…` · slirp/tcp_subr.c:239-248 · chunk 2813 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct tcpiphdr *t;
//      struct socket *so = tp->t_socket;
//      Slirp *slirp = so->slirp;
//      register struct mbuf *m;
//     
//      ;
//      ;
//     
//     
//      t = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2813 {
// Port of slirp tcp_close's prologue: fetch tp->t_socket, so->slirp and the
// first entry of the TCP reassembly queue (tcpfrag_list_first(tp)).
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// struct qlink { void *next, *prev; }
#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct in_addr { in_addr_t s_addr; }
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr (the u_int th_x2:4, th_off:4 bit-field pair occupies one byte)
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
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
    pub so_ti: *mut Tcpiphdr,
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

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
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

// Opaque here: no field of Slirp is touched by this piece.
#[repr(C)]
pub struct Slirp {
    pub _private: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn tcp_close__part2813(
    tp: *mut Tcpcb,
    t_out: *mut *mut Tcpiphdr,
    so_out: *mut *mut Socket,
    slirp_out: *mut *mut Slirp,
) {
    // struct socket *so = tp->t_socket;
    let so = (*tp).t_socket;
    // Slirp *slirp = so->slirp;
    let slirp = (*so).slirp;
    // t = tcpfrag_list_first(tp) == qlink2tcpiphdr(tp->seg_next)
    let t = ((*tp).seg_next as *mut u8)
        .wrapping_add(core::mem::size_of::<Qlink>()) as *mut Tcpiphdr;

    if !t_out.is_null() {
        *t_out = t;
    }
    if !so_out.is_null() {
        *so_out = so;
    }
    if !slirp_out.is_null() {
        *slirp_out = slirp;
    }
}
}

// ── `while (!(((struct qlink*)(((char*)(t)) - sizeof(struct qlink))) == (struct qlink*)(tp)` · slirp/tcp_subr.c:249-254 · chunk 2814 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     while (!(((struct qlink*)(((char*)(t)) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       t = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       m = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)))->ti_i.ih_mbuf.mptr;
//       slirp_remque(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(t)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
//       m_free(m);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2814 {
// Port of the TCP reassembly segment cleanup loop inside tcp_close().
use core::ffi::c_void;

// Mirror of the C `struct qlink { void *next, *prev; }`.
#[repr(C)]
struct QLink {
    next: *mut c_void,
    prev: *mut c_void,
}

extern "C" {
    fn slirp_remque(arg: *mut c_void);
    fn m_free(m: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_close__part2814(t_start: *mut c_void, tp: *mut c_void) {
    let qsize: isize = core::mem::size_of::<QLink>() as isize;
    let mut t: *mut u8 = t_start as *mut u8;

    loop {
        // q = (struct qlink *)((char *)t - sizeof(struct qlink))
        let q = t.offset(-qsize) as *mut QLink;
        if q as *mut c_void == tp {
            break;
        }

        // t = (struct tcpiphdr *)((char *)q->next + sizeof(struct qlink))
        let next = (*q).next as *mut u8;
        t = next.offset(qsize);

        // qlink of the new t
        let q2 = t.offset(-qsize) as *mut QLink;

        // m = ((struct tcpiphdr *)((char *)q2->prev + sizeof(struct qlink)))
        //         ->ti_i.ih_mbuf.mptr
        // ih_mbuf.mptr is at offset 0 of a (packed) tcpiphdr.
        let m_slot = (*q2).prev as *mut u8;
        let m: *mut c_void = *m_slot.offset(qsize).cast::<*mut c_void>();

        // slirp_remque((struct qlink *)((char *)prev_of_prev_tcpiphdr
        //              - sizeof(struct qlink))), which is just q2->prev.
        slirp_remque((*q2).prev);
        m_free(m);
    }
}
}

// ── `free(tp); so->so_tcpcb = ((void *)0) ;` · slirp/tcp_subr.c:255-256 · chunk 2815 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(tp);
//             so->so_tcpcb = 
//                           ((void *)0)
//                               ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2815 {
// Portion of slirp tcp_close(): free the tcpcb and detach it from its socket.
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sb {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
}

// struct tcpcb is only handled through pointers here; contents never touched.
#[repr(C)]
pub struct Tcpcb {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct Mbuf {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct Tcpiphdr {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct Slirp {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
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
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_close__part2815(tp: *mut Tcpcb, so: *mut Socket) {
    free(tp as *mut c_void);
    if !so.is_null() {
        (*so).so_tcpcb = core::ptr::null_mut();
    }
}
}

// ── `if (so == slirp->tcp_last_so)` · slirp/tcp_subr.c:258-259 · chunk 2816 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so == slirp->tcp_last_so)
//       slirp->tcp_last_so = &slirp->tcb;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2816 {
// Port of the tail of slirp's tcp_close() that keeps slirp->tcp_last_so
// from pointing at the socket being freed.
//
// The socket structures are opaque here: the caller hands over the socket
// being closed, the address of slirp->tcp_last_so and the address of
// slirp->tcb, so no layout knowledge is needed.

use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn tcp_close__part2816(
    so: *mut c_void,
    tcp_last_so: *mut *mut c_void,
    tcb: *mut c_void,
) {
    unsafe {
        if *tcp_last_so == so {
            *tcp_last_so = tcb;
        }
    }
}
}

// ── `close(so->s); sbfree(&so->so_rcv); sbfree(&so->so_snd); sofree(so); return ((struct tcpcb *)0);` · slirp/tcp_subr.c:260-264 · chunk 2817 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     close(so->s);
//      sbfree(&so->so_rcv);
//      sbfree(&so->so_snd);
//      sofree(so);
//      return ((struct tcpcb *)0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2817 {
// Rust rewrite of the tail of tcp_close(): close the socket fd, free the
// socket buffers and the socket itself, then yield the terminating NULL.

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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn close(fd: c_int) -> c_int;
    fn sbfree(sb: *mut Sbuf);
    fn sofree(so: *mut Socket);
}

#[no_mangle]
pub extern "C" fn tcp_close__part2817(so: *mut Socket) -> *mut c_void {
    unsafe {
        close((*so).s);
        sbfree(&mut (*so).so_rcv);
        sbfree(&mut (*so).so_snd);
        sofree(so);
    }
    core::ptr::null_mut()
}
}

// ── `tcp_sockclosed` · slirp/tcp_subr.c:281-308 · chunk 2818 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_sockclosed(struct tcpcb *tp)
//     {
//     
//      ;
//      ;
//     
//      switch (tp->t_state) {
//     
//      case 0:
//      case 1:
//      case 2:
//       tp->t_state = 0;
//       tp = tcp_close(tp);
//       break;
//     
//      case 3:
//      case 4:
//       tp->t_state = 6;
//       break;
//     
//      case 5:
//       tp->t_state = 8;
//       break;
//      }
//      if (tp)
//       tcp_output(tp);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2818 {
// Rewrite of slirp/tcp_subr.c: tcp_sockclosed(struct tcpcb *tp)
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_short};

// Mirror of the leading fields of the C `struct tcpcb` (slirp/tcp_var.h).
// Only `t_state` is touched by this function, but the preceding fields keep
// the layout (and therefore the offset of t_state) identical to the C struct.
#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: c_short,
}

extern "C" {
    fn tcp_close(tp: *mut tcpcb) -> *mut tcpcb;
    fn tcp_output(tp: *mut tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_sockclosed(mut tp: *mut tcpcb) {
    unsafe {
        if tp.is_null() {
            return;
        }

        match (*tp).t_state {
            0 | 1 | 2 => {
                // TCPS_CLOSED / TCPS_LISTEN / TCPS_SYN_SENT
                (*tp).t_state = 0;
                tp = tcp_close(tp);
            }
            3 | 4 => {
                // TCPS_SYN_RECEIVED / TCPS_ESTABLISHED
                (*tp).t_state = 6;
            }
            5 => {
                // TCPS_CLOSE_WAIT
                (*tp).t_state = 8;
            }
            _ => {}
        }

        if !tp.is_null() {
            tcp_output(tp);
        }
    }
}
}

// ── `; ;` · slirp/tcp_subr.c:285-286 · chunk 2819 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2819 {
// Port of the (compile-time disabled) DEBUG_CALL / DEBUG_ARG statements at the
// top of tcp_sockclosed(). With the slirp debug macros configured out they
// expand to empty statements, so the piece has no observable effect and no
// inputs from the enclosing function.

#[no_mangle]
pub extern "C" fn tcp_sockclosed__part2819() {
    // Both original statements were bare `;` (no-op debug macros).
}
}

// ── `switch (tp->t_state)` · slirp/tcp_subr.c:288-305 · chunk 2820 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch (tp->t_state) {
//     
//      case 0:
//      case 1:
//      case 2:
//       tp->t_state = 0;
//       tp = tcp_close(tp);
//       break;
//     
//      case 3:
//      case 4:
//       tp->t_state = 6;
//       break;
//     
//      case 5:
//       tp->t_state = 8;
//       break;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2820 {
// Port of the state-transition switch in slirp's tcp_sockclosed().
// Only the leading fields of the C `struct tcpcb` are described here: the
// code touches nothing but `t_state`, and the leading layout (two pointers
// then a `short`) is exact, so the field offset matches the C struct.
use core::ffi::{c_short, c_void};

#[repr(C)]
pub struct TcpcbPrefix {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
}

extern "C" {
    // struct tcpcb * tcp_close(register struct tcpcb *);
    fn tcp_close(tp: *mut TcpcbPrefix) -> *mut TcpcbPrefix;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_sockclosed__part2820(tp: *mut TcpcbPrefix) -> *mut TcpcbPrefix {
    let state = (*tp).t_state as i32;
    match state {
        0 | 1 | 2 => {
            (*tp).t_state = 0;
            return tcp_close(tp);
        }
        3 | 4 => {
            (*tp).t_state = 6;
        }
        5 => {
            (*tp).t_state = 8;
        }
        _ => {}
    }
    tp
}
}

// ── `if (tp)` · slirp/tcp_subr.c:306-307 · chunk 2821 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp)
//       tcp_output(tp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2821 {
// Port of the tail of tcp_sockclosed: conditionally emit output on the tcpcb.
use core::ffi::{c_int, c_void};

extern "C" {
    fn tcp_output(tp: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_sockclosed__part2821(tp: *mut c_void) {
    if !tp.is_null() {
        unsafe {
            tcp_output(tp);
        }
    }
}
}

// ── `tcp_fconnect` · slirp/tcp_subr.c:320-366 · chunk 2822 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int tcp_fconnect(struct socket *so)
//     {
//       Slirp *slirp = so->slirp;
//       int ret=0;
//     
//       ;
//       ;
//     
//       if( (ret = so->s = os_socket(
//                                   2
//                                          ,
//                                           SOCK_STREAM
//                                                      ,0)) >= 0) {
//         int opt, s=so->s;
//         struct sockaddr_in addr;
//     
//         fd_nonblock(s);
//         opt = 1;
//         setsockopt(s,
//                     1
//                               ,
//                                2
//                                            ,(char *)&opt,sizeof(opt ));
//         opt = 1;
//         setsockopt(s,
//                     1
//                               ,
//                                10
//                                            ,(char *)&opt,sizeof(opt ));
//     
//         addr.sin_family = 
//                          2
//                                 ;
//         if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//             slirp->vnetwork_addr.s_addr) {
//     
//           if (so->so_faddr.s_addr == slirp->vnameserver_addr.s_addr) {
//      if (get_dns_addr(&addr.sin_addr) < 0)
//        addr.sin_addr = loopback_addr;
//           } else {
//      addr.sin_addr = loopback_addr;
//           }
//         } else
//           addr.sin_addr = so->so_faddr;
//         addr.sin_port = so->so_fport;
//     
//        
//     
//                                                       ;
//     
//         ret = connect(s,(struct sockaddr *)&addr,sizeof (addr));
//     
//     
//     
//     
//     
//         soisfconnecting(so);
//       }
//     
//       return(ret);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2822 {
// Rust rewrite of slirp's tcp_fconnect().
use core::ffi::{c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// Only the leading fields of the C `struct Slirp` are needed (and laid out);
// the remainder is never touched by this function.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

// Leading fields of the C `struct socket` up to `so_fport`; offsets match.
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
}

extern "C" {
    fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn fd_nonblock(fd: c_int);
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: u32,
    ) -> c_int;
    fn connect(fd: c_int, addr: *const SockaddrIn, addrlen: u32) -> c_int;
    fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int;
    fn soisfconnecting(so: *mut Socket);
    static mut loopback_addr: InAddr;
}

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_OOBINLINE: c_int = 10;

#[no_mangle]
pub extern "C" fn tcp_fconnect(so: *mut Socket) -> c_int {
    unsafe {
        let slirp: *mut Slirp = (*so).slirp;
        let mut ret: c_int = 0;

        let opened = os_socket(AF_INET, SOCK_STREAM, 0);
        (*so).s = opened;
        ret = opened;

        if ret >= 0 {
            let s: c_int = (*so).s;
            let mut addr: SockaddrIn = core::mem::zeroed();

            fd_nonblock(s);
            let mut opt: c_int = 1;
            setsockopt(
                s,
                SOL_SOCKET,
                SO_REUSEADDR,
                &opt as *const c_int as *const c_void,
                core::mem::size_of::<c_int>() as u32,
            );
            opt = 1;
            setsockopt(
                s,
                SOL_SOCKET,
                SO_OOBINLINE,
                &opt as *const c_int as *const c_void,
                core::mem::size_of::<c_int>() as u32,
            );

            addr.sin_family = AF_INET as u16;
            let faddr: u32 = (*so).so_faddr.s_addr;
            if (faddr & (*slirp).vnetwork_mask.s_addr) == (*slirp).vnetwork_addr.s_addr {
                // It's an alias
                if faddr == (*slirp).vnameserver_addr.s_addr {
                    if get_dns_addr(&mut addr.sin_addr) < 0 {
                        addr.sin_addr = loopback_addr;
                    }
                } else {
                    addr.sin_addr = loopback_addr;
                }
            } else {
                addr.sin_addr = (*so).so_faddr;
            }
            addr.sin_port = (*so).so_fport;

            // We don't care what port we get
            ret = connect(s, &addr as *const SockaddrIn, core::mem::size_of::<SockaddrIn>() as u32);

            // If it's not in progress, it failed, so we just return 0,
            // without clearing SS_NOFDREF
            soisfconnecting(so);
        }

        ret
    }
}
}

// ── `Slirp *slirp = so->slirp; int ret=0; ; ;` · slirp/tcp_subr.c:322-326 · chunk 2823 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = so->slirp;
//       int ret=0;
//     
//       ;
//       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2823 {
// Extracted part of tcp_fconnect: initialises the `slirp` alias and `ret`.
use core::ffi::{c_int, c_void};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
}

#[no_mangle]
pub extern "C" fn tcp_fconnect__part2823(
    so: *mut Socket,
    slirp_out: *mut *mut c_void,
    ret_out: *mut c_int,
) {
    // Slirp *slirp = so->slirp;
    let slirp: *mut c_void = unsafe { (*so).slirp };
    if !slirp_out.is_null() {
        unsafe {
            *slirp_out = slirp;
        }
    }
    // int ret = 0;
    if !ret_out.is_null() {
        unsafe {
            *ret_out = 0;
        }
    }
}
}

// ── `if (ret = so->s = os_socket( 2 , SOCK_STREAM ,0)) >= 0)` · slirp/tcp_subr.c:328-363 · chunk 2824 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if( (ret = so->s = os_socket(
//                                   2
//                                          ,
//                                           SOCK_STREAM
//                                                      ,0)) >= 0) {
//         int opt, s=so->s;
//         struct sockaddr_in addr;
//     
//         fd_nonblock(s);
//         opt = 1;
//         setsockopt(s,
//                     1
//                               ,
//                                2
//                                            ,(char *)&opt,sizeof(opt ));
//         opt = 1;
//         setsockopt(s,
//                     1
//                               ,
//                                10
//                                            ,(char *)&opt,sizeof(opt ));
//     
//         addr.sin_family = 
//                          2
//                                 ;
//         if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//             slirp->vnetwork_addr.s_addr) {
//     
//           if (so->so_faddr.s_addr == slirp->vnameserver_addr.s_addr) {
//      if (get_dns_addr(&addr.sin_addr) < 0)
//        addr.sin_addr = loopback_addr;
//           } else {
//      addr.sin_addr = loopback_addr;
//           }
//         } else
//           addr.sin_addr = so->so_faddr;
//         addr.sin_port = so->so_fport;
//     
//        
//     
//                                                       ;
//     
//         ret = connect(s,(struct sockaddr *)&addr,sizeof (addr));
//     
//     
//     
//     
//     
//         soisfconnecting(so);
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2824 {
// Port of the tcp_fconnect body: socket creation + sockaddr_in setup + connect.
use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sb {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Only the leading fields of Slirp are needed here.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
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
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

extern "C" {
    fn os_socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn fd_nonblock(s: c_int);
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: u32,
    ) -> c_int;
    fn connect(fd: c_int, addr: *const c_void, len: u32) -> c_int;
    fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int;
    fn soisfconnecting(so: *mut Socket);
    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn tcp_fconnect__part2824(so: *mut Socket) -> c_int {
    unsafe {
        let mut ret: c_int = os_socket(2, 1, 0);
        (*so).s = ret;
        if ret >= 0 {
            let s: c_int = (*so).s;
            let mut opt: c_int;

            fd_nonblock(s);
            opt = 1;
            setsockopt(s, 1, 2, &mut opt as *mut c_int as *const c_void, 4);
            opt = 1;
            setsockopt(s, 1, 10, &mut opt as *mut c_int as *const c_void, 4);

            // addr is left uninitialized apart from the fields C assigns,
            // exactly as in the original code.
            let mut addr = core::mem::MaybeUninit::<SockAddrIn>::uninit();
            let ap: *mut SockAddrIn = addr.as_mut_ptr();

            (*ap).sin_family = 2;

            let slirp = (*so).slirp;
            let faddr: u32 = (*so).so_faddr.s_addr;
            if (faddr & (*slirp).vnetwork_mask.s_addr) == (*slirp).vnetwork_addr.s_addr {
                if faddr == (*slirp).vnameserver_addr.s_addr {
                    if get_dns_addr(&mut (*ap).sin_addr) < 0 {
                        (*ap).sin_addr = loopback_addr;
                    }
                } else {
                    (*ap).sin_addr = loopback_addr;
                }
            } else {
                (*ap).sin_addr = (*so).so_faddr;
            }
            (*ap).sin_port = (*so).so_fport;

            ret = connect(
                s,
                ap as *const SockAddrIn as *const c_void,
                core::mem::size_of::<SockAddrIn>() as u32,
            );

            soisfconnecting(so);
        }
        ret
    }
}
}

// ── `return(ret);` · slirp/tcp_subr.c:365-365 · chunk 2825 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return(ret);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2825 {
// Port of the tail of tcp_fconnect: `return(ret);`
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_fconnect__part2825(ret: c_int) -> c_int {
    ret
}
}

// ── `tcp_connect` · slirp/tcp_subr.c:380-454 · chunk 2826 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_connect(struct socket *inso)
//     {
//      Slirp *slirp = inso->slirp;
//      struct socket *so;
//      struct sockaddr_in addr;
//      socklen_t addrlen = sizeof(struct sockaddr_in);
//      struct tcpcb *tp;
//      int s, opt;
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//      if (inso->so_state & 0x200) {
//     
//       so = inso;
//      } else {
//       if ((so = socreate(slirp)) == 
//                                    ((void *)0)
//                                        ) {
//     
//        close(accept(inso->s,(struct sockaddr *)&addr,&addrlen));
//        return;
//       }
//       if (tcp_attach(so) < 0) {
//        free(so);
//        return;
//       }
//       so->so_laddr = inso->so_laddr;
//       so->so_lport = inso->so_lport;
//      }
//     
//      (void) tcp_mss(((so)->so_tcpcb), 0);
//     
//      if ((s = accept(inso->s,(struct sockaddr *)&addr,&addrlen)) < 0) {
//       tcp_close(((so)->so_tcpcb));
//       return;
//      }
//      fd_nonblock(s);
//      opt = 1;
//      setsockopt(s,
//                  1
//                            ,
//                             2
//                                         ,(char *)&opt,sizeof(int));
//      opt = 1;
//      setsockopt(s,
//                  1
//                            ,
//                             10
//                                         ,(char *)&opt,sizeof(int));
//      opt = 1;
//      setsockopt(s,
//                  IPPROTO_TCP
//                             ,0x01,(char *)&opt,sizeof(int));
//     
//      so->so_fport = addr.sin_port;
//      so->so_faddr = addr.sin_addr;
//     
//      if (so->so_faddr.s_addr == 0 || so->so_faddr.s_addr == loopback_addr.s_addr)
//         so->so_faddr = slirp->vhost_addr;
//     
//     
//      if (inso->so_state & 0x200) {
//       close(so->s);
//       so->so_state = 0x001;
//     
//      }
//      so->s = s;
//      so->so_state |= 0x2000;
//     
//      so->so_iptos = tcp_tos(so);
//      tp = ((so)->so_tcpcb);
//     
//      tcp_template(tp);
//     
//      tp->t_state = 2;
//      tp->t_timer[2] = ( 75*2);
//      tp->iss = slirp->tcp_iss;
//      slirp->tcp_iss += (125*1024)/2;
//      (tp)->snd_una = (tp)->snd_nxt = (tp)->snd_max = (tp)->snd_up = (tp)->iss;
//      tcp_output(tp);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2826 {
// Port of slirp/tcp_subr.c:tcp_connect
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::MaybeUninit;

pub type tcp_seq = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

// ---- TCP header / pseudo-header (mirrored for correct tcpcb layout) ----

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    pub th_x2_off: u32, // th_x2:4, th_off:4 bitfields
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// ---- tcpcb (prefix of the C struct that this function touches) ----

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
    pub snd_una: tcp_seq,
    pub snd_nxt: tcp_seq,
    pub snd_up: tcp_seq,
    pub snd_wl1: tcp_seq,
    pub snd_wl2: tcp_seq,
    pub iss: tcp_seq,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub rcv_nxt: tcp_seq,
    pub rcv_up: tcp_seq,
    pub irs: tcp_seq,
    pub rcv_adv: tcp_seq,
    pub snd_max: tcp_seq,
}

// ---- mbuf ----

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

// ---- sbuf / socket ----

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub so_ti: *mut Tcpiphdr,
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

// ---- misc types embedded in Slirp ----

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipqueue {
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
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
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
    pub ipq: Ipqueue,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: tcp_seq,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn socreate(slirp: *mut Slirp) -> *mut Socket;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut SockaddrIn, addrlen: *mut u32) -> c_int;
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: u32,
    ) -> c_int;

    fn tcp_attach(so: *mut Socket) -> c_int;
    fn tcp_mss(tp: *mut Tcpcb, mss: u32) -> c_int;
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
    fn tcp_tos(so: *mut Socket) -> u8;
    fn tcp_template(tp: *mut Tcpcb);
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn fd_nonblock(fd: c_int);

    static loopback_addr: InAddr;
}

#[no_mangle]
pub extern "C" fn tcp_connect(inso: *mut Socket) {
    unsafe {
        let slirp: *mut Slirp = (*inso).slirp;
        let mut addr: MaybeUninit<SockaddrIn> = MaybeUninit::uninit();
        let mut addrlen: u32 = core::mem::size_of::<SockaddrIn>() as u32;
        let so: *mut Socket;
        let tp: *mut Tcpcb;
        let s: c_int;
        let mut opt: c_int;

        /*
         * If it's an SS_ACCEPTONCE socket, no need to socreate()
         * another socket, just use the accept() socket.
         */
        if (*inso).so_state & 0x200 != 0 {
            // SS_FACCEPTONCE: already have a tcpcb
            so = inso;
        } else {
            so = socreate(slirp);
            if so.is_null() {
                // If it failed, get rid of the pending connection
                let fd = accept((*inso).s, addr.as_mut_ptr(), &mut addrlen);
                close(fd);
                return;
            }
            if tcp_attach(so) < 0 {
                free(so as *mut c_void); // NOT sofree
                return;
            }
            (*so).so_laddr = (*inso).so_laddr;
            (*so).so_lport = (*inso).so_lport;
        }

        tcp_mss((*so).so_tcpcb, 0);

        s = accept((*inso).s, addr.as_mut_ptr(), &mut addrlen);
        if s < 0 {
            tcp_close((*so).so_tcpcb); // This will sofree() as well
            return;
        }
        fd_nonblock(s);
        opt = 1;
        setsockopt(s, 1 /* SOL_SOCKET */, 2 /* SO_REUSEADDR */, &opt as *const c_int as *const c_void, 4);
        opt = 1;
        setsockopt(s, 1 /* SOL_SOCKET */, 10 /* SO_OOBINLINE */, &opt as *const c_int as *const c_void, 4);
        opt = 1;
        setsockopt(s, 6 /* IPPROTO_TCP */, 0x01 /* TCP_NODELAY */, &opt as *const c_int as *const c_void, 4);

        let addr = addr.assume_init();
        (*so).so_fport = addr.sin_port;
        (*so).so_faddr = addr.sin_addr;
        // Translate connections from localhost to the real hostname
        if (*so).so_faddr.s_addr == 0 || (*so).so_faddr.s_addr == loopback_addr.s_addr {
            (*so).so_faddr = (*slirp).vhost_addr;
        }

        // Close the accept() socket, set right state
        if (*inso).so_state & 0x200 != 0 {
            // If we only accept once, close the accept() socket
            close((*so).s);
            // Don't select it yet, even though we have an FD;
            // if it's not FACCEPTONCE, it's already NOFDREF
            (*so).so_state = 0x001; // SS_NOFDREF
        }
        (*so).s = s;
        (*so).so_state |= 0x2000; // SS_INCOMING

        (*so).so_iptos = tcp_tos(so);
        tp = (*so).so_tcpcb;

        tcp_template(tp);

        (*tp).t_state = 2; // TCPS_SYN_SENT
        (*tp).t_timer[2] = (75 * 2) as i16; // TCPT_KEEP = TCPTV_KEEP_INIT
        (*tp).iss = (*slirp).tcp_iss;
        (*slirp).tcp_iss = (*slirp).tcp_iss.wrapping_add((125 * 1024) / 2);
        let iss = (*tp).iss;
        (*tp).snd_una = iss;
        (*tp).snd_nxt = iss;
        (*tp).snd_max = iss;
        (*tp).snd_up = iss;
        tcp_output(tp);
    }
}
}

// ── `Slirp *slirp = inso->slirp; struct socket *so; struct sockaddr_in addr; socklen_t addrlen = siz…` · slirp/tcp_subr.c:383-391 · chunk 2827 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = inso->slirp;
//      struct socket *so;
//      struct sockaddr_in addr;
//      socklen_t addrlen = sizeof(struct sockaddr_in);
//      struct tcpcb *tp;
//      int s, opt;
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2827 {
// Port of the leading declarations of tcp_connect (slirp/tcp_subr.c).
// The C side keeps the declarations of `so`, `addr`, `tp`, `s` and `opt`;
// this function provides the value of `slirp` (inso->slirp) and the initial
// value of `addrlen` (sizeof(struct sockaddr_in)).

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
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Tcpiphdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Tcpcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
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
pub extern "C" fn tcp_connect__part2827(inso: *mut Socket, addrlen: *mut u32) -> *mut Slirp {
    unsafe {
        if !addrlen.is_null() {
            *addrlen = core::mem::size_of::<SockaddrIn>() as u32;
        }
        if inso.is_null() {
            return core::ptr::null_mut();
        }
        (*inso).slirp
    }
}
}

// ── `if (inso->so_state & 0x200) … else` · slirp/tcp_subr.c:397-412 · chunk 2828 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (inso->so_state & 0x200) {
//     
//       so = inso;
//      } else {
//       if ((so = socreate(slirp)) == 
//                                    ((void *)0)
//                                        ) {
//     
//        close(accept(inso->s,(struct sockaddr *)&addr,&addrlen));
//        return;
//       }
//       if (tcp_attach(so) < 0) {
//        free(so);
//        return;
//       }
//       so->so_laddr = inso->so_laddr;
//       so->so_lport = inso->so_lport;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2828 {
// Port of the socket-setup part of slirp's tcp_connect().
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sb {
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
    pub so_rcv: Sb,
    pub so_snd: Sb,
    pub extra: *mut c_void,
}

extern "C" {
    fn socreate(slirp: *mut c_void) -> *mut Socket;
    fn tcp_attach(so: *mut Socket) -> c_int;
    fn free(p: *mut c_void);
}

/// Returns 0 on success, 1 if socreate() failed (caller closes the pending
/// accept and returns), 2 if tcp_attach() failed (caller frees `*so_out`
/// and returns).
#[no_mangle]
pub extern "C" fn tcp_connect__part2828(
    inso: *mut Socket,
    slirp: *mut c_void,
    so_out: *mut *mut Socket,
) -> c_int {
    unsafe {
        if (*inso).so_state & 0x200 != 0 {
            *so_out = inso;
            return 0;
        }
        let so: *mut Socket = socreate(slirp);
        *so_out = so;
        if so.is_null() {
            return 1;
        }
        if tcp_attach(so) < 0 {
            free(so as *mut c_void);
            return 2;
        }
        (*so).so_laddr = (*inso).so_laddr;
        (*so).so_lport = (*inso).so_lport;
        0
    }
}
}

// ── `(void) tcp_mss(((so)->so_tcpcb), 0);` · slirp/tcp_subr.c:414-414 · chunk 2829 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     (void) tcp_mss(((so)->so_tcpcb), 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2829 {
// Port of the tcp_mss() call inside tcp_connect().

use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    // int tcp_mss(register struct tcpcb *, u_int);
    fn tcp_mss(tp: *mut c_void, offer: c_uint) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_connect__part2829(so_tcpcb: *mut c_void) {
    unsafe {
        tcp_mss(so_tcpcb, 0);
    }
}
}

// ── `if (s = accept(inso->s,(struct sockaddr *)&addr,&addrlen)) < 0)` · slirp/tcp_subr.c:416-419 · chunk 2830 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((s = accept(inso->s,(struct sockaddr *)&addr,&addrlen)) < 0) {
//       tcp_close(((so)->so_tcpcb));
//       return;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2830 {
// Port of the accept() step of tcp_connect (slirp/tcp_subr.c).
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of the leading part of C `struct socket` (up to so_tcpcb).
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

// glibc __SOCKADDR_ARG: a union holding one pointer, passed by value.
#[repr(C)]
#[derive(Clone, Copy)]
pub union SockaddrArg {
    pub sockaddr_: *mut c_void,
}

extern "C" {
    fn accept(fd: c_int, addr: SockaddrArg, addrlen: *mut c_uint) -> c_int;
    fn tcp_close(tp: *mut c_void) -> *mut c_void;
}

// Returns 1 when the C side must `return;` (accept failed), 0 otherwise.
// `*s_out` always receives the accept() result, as the C code assigned `s`.
#[no_mangle]
pub unsafe extern "C" fn tcp_connect__part2830(
    inso_s: c_int,
    addr: *mut c_void,
    addrlen: *mut c_uint,
    so: *mut Socket,
    s_out: *mut c_int,
) -> c_int {
    let s = accept(inso_s, SockaddrArg { sockaddr_: addr }, addrlen);
    if !s_out.is_null() {
        *s_out = s;
    }
    if s < 0 {
        if !so.is_null() {
            let _ = tcp_close((*so).so_tcpcb);
        }
        1
    } else {
        0
    }
}
}

// ── `fd_nonblock(s); opt = 1; setsockopt(s, 1 , 2 ,(char *)&opt,sizeof(int)); opt = 1; setsockopt(s,…` · slirp/tcp_subr.c:420-429 · chunk 2831 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fd_nonblock(s);
//      opt = 1;
//      setsockopt(s,
//                  1
//                            ,
//                             2
//                                         ,(char *)&opt,sizeof(int));
//      opt = 1;
//      setsockopt(s,
//                  1
//                            ,
//                             10
//                                         ,(char *)&opt,sizeof(int));
//      opt = 1;
//      setsockopt(s,
//                  IPPROTO_TCP
//                             ,0x01,(char *)&opt,sizeof(int));
//     
//      so->so_fport = addr.sin_port;
//      so->so_faddr = addr.sin_addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2831 {
// Ported piece of tcp_connect(): make the accepted fd non-blocking, set
// socket options, and record the foreign address/port on the socket.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type SocklenT = c_uint;

#[repr(C)]
#[derive(Clone, Copy)]
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

#[repr(C)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_KEEPALIVE: c_int = 10;
const IPPROTO_TCP: c_int = 6;
const TCP_NODELAY: c_int = 0x01;

extern "C" {
    fn fd_nonblock(fd: c_int);
    fn setsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *const c_void,
        optlen: SocklenT,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_connect__part2831(
    s: c_int,
    opt: *mut c_int,
    so: *mut Socket,
    addr: *const SockaddrIn,
) {
    unsafe {
        fd_nonblock(s);

        *opt = 1;
        setsockopt(
            s,
            SOL_SOCKET,
            SO_REUSEADDR,
            opt as *const c_void,
            core::mem::size_of::<c_int>() as SocklenT,
        );

        *opt = 1;
        setsockopt(
            s,
            SOL_SOCKET,
            SO_KEEPALIVE,
            opt as *const c_void,
            core::mem::size_of::<c_int>() as SocklenT,
        );

        *opt = 1;
        setsockopt(
            s,
            IPPROTO_TCP,
            TCP_NODELAY,
            opt as *const c_void,
            core::mem::size_of::<c_int>() as SocklenT,
        );

        (*so).so_fport = (*addr).sin_port;
        (*so).so_faddr = (*addr).sin_addr;
    }
}
}

// ── `if (so->so_faddr.s_addr == 0 || so->so_faddr.s_addr == loopback_addr.s_addr)` · slirp/tcp_subr.c:431-432 · chunk 2832 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_faddr.s_addr == 0 || so->so_faddr.s_addr == loopback_addr.s_addr)
//         so->so_faddr = slirp->vhost_addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2832 {
// Port of tcp_connect lines 431-432: fix up the foreign address of a
// newly accepted socket when it is 0 or the loopback address.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    static mut loopback_addr: InAddr;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_connect__part2832(
    so_faddr: *mut InAddr,
    slirp_vhost_addr: *const InAddr,
) {
    if so_faddr.is_null() || slirp_vhost_addr.is_null() {
        return;
    }
    let faddr: &mut InAddr = &mut *so_faddr;
    if faddr.s_addr == 0 || faddr.s_addr == loopback_addr.s_addr {
        *faddr = *slirp_vhost_addr;
    }
}
}

// ── `if (inso->so_state & 0x200)` · slirp/tcp_subr.c:435-439 · chunk 2833 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (inso->so_state & 0x200) {
//       close(so->s);
//       so->so_state = 0x001;
//     
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2833 {
// Port of the socket-state reset branch of tcp_connect (slirp/tcp_subr.c).
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

type Slirp = core::ffi::c_void;
type Mbuf = core::ffi::c_void;
type Tcpiphdr = core::ffi::c_void;
type Tcpcb = core::ffi::c_void;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
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
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
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
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut core::ffi::c_void,
}

extern "C" {
    fn close(fd: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_connect__part2833(inso: *mut socket, so: *mut socket) {
    unsafe {
        if (*inso).so_state & 0x200 != 0 {
            close((*so).s);
            (*so).so_state = 0x001;
        }
    }
}
}

// ── `so->s = s; so->so_state |= 0x2000; so->so_iptos = tcp_tos(so); tp = ((so)->so_tcpcb); tcp_templ…` · slirp/tcp_subr.c:440-453 · chunk 2834 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     so->s = s;
//      so->so_state |= 0x2000;
//     
//      so->so_iptos = tcp_tos(so);
//      tp = ((so)->so_tcpcb);
//     
//      tcp_template(tp);
//     
//      tp->t_state = 2;
//      tp->t_timer[2] = ( 75*2);
//      tp->iss = slirp->tcp_iss;
//      slirp->tcp_iss += (125*1024)/2;
//      (tp)->snd_una = (tp)->snd_nxt = (tp)->snd_max = (tp)->snd_up = (tp)->iss;
//      tcp_output(tp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2834 {
// Tail of tcp_connect() from slirp/tcp_subr.c, ported to Rust.
// Mirrors the C struct layouts (x86-64 System V / repr(C)).

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: u32,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
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
    pub slirp: *mut Slirp,
    pub m_dat: [u8; 8],
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
    pub ipq_src: u32,
    pub ipq_dst: u32,
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
    pub so_faddr: u32,
    pub so_laddr: u32,
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

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: u32,
    pub ih_dst: u32,
}

// th_x2:4 / th_off:4 bit-field pair occupies the byte at offset 12.
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
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
    pub t_idle: c_short,
    pub t_rtt: c_short,
    pub t_rtseq: u32,
    pub t_srtt: c_short,
    pub t_rttvar: c_short,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: c_short,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: u32,
    pub vnetwork_mask: u32,
    pub vhost_addr: u32,
    pub vdhcp_startaddr: u32,
    pub vnameserver_addr: u32,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: u32,
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
    pub opaque: *mut c_void,
}

extern "C" {
    fn tcp_tos(so: *mut Socket) -> u8;
    fn tcp_template(tp: *mut Tcpcb);
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_connect__part2834(
    so: *mut Socket,
    s: c_int,
    slirp: *mut Slirp,
) -> *mut Tcpcb {
    unsafe {
        (*so).s = s;
        (*so).so_state |= 0x2000;
        (*so).so_iptos = tcp_tos(so);
        let tp = (*so).so_tcpcb;
        tcp_template(tp);
        (*tp).t_state = 2;
        (*tp).t_timer[2] = (75 * 2) as c_short;
        (*tp).iss = (*slirp).tcp_iss;
        (*slirp).tcp_iss = (*slirp).tcp_iss.wrapping_add((125 * 1024) / 2);
        let iss = (*tp).iss;
        (*tp).snd_una = iss;
        (*tp).snd_nxt = iss;
        (*tp).snd_max = iss;
        (*tp).snd_up = iss;
        tcp_output(tp);
        tp
    }
}
}

// ── `tcp_attach` · slirp/tcp_subr.c:459-468 · chunk 2835 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     tcp_attach(struct socket *so)
//     {
//      if ((so->so_tcpcb = tcp_newtcpcb(so)) == 
//                                              ((void *)0)
//                                                  )
//         return -1;
//     
//      slirp_insque(so, &so->slirp->tcb);
//     
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2835 {
// Port of slirp tcp_attach(struct socket *so)

use core::ffi::{c_char, c_int, c_void};

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

// Opaque: only ever used behind a pointer.
#[repr(C)]
pub struct Tcpcb {
    _private: [u8; 0],
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

// tcp.h: th_x2/th_off are a 4+4 bit bitfield in a u_int
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_flags_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
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
    pub so_ti: *mut Tcpiphdr,
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

extern "C" {
    fn tcp_newtcpcb(so: *mut Socket) -> *mut Tcpcb;
    fn slirp_insque(elem: *mut c_void, list: *mut c_void);
}

#[no_mangle]
pub extern "C" fn tcp_attach(so: *mut Socket) -> c_int {
    unsafe {
        let tp = tcp_newtcpcb(so);
        (*so).so_tcpcb = tp;
        if tp.is_null() {
            return -1;
        }

        let tcb = core::ptr::addr_of_mut!((*(*so).slirp).tcb) as *mut c_void;
        slirp_insque(so as *mut c_void, tcb);

        0
    }
}
}

// ── `if (so->so_tcpcb = tcp_newtcpcb(so)) == ((void *)0)` · slirp/tcp_subr.c:462-463 · chunk 2836 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_tcpcb = tcp_newtcpcb(so)) == 
//                                              ((void *)0)
//                                                  )
//         return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2836 {
// Port of tcp_attach__part2836: allocate the tcpcb for a socket and store it.
use core::ffi::{c_int, c_void};

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

// Mirror of `struct socket` up to and including `so_tcpcb`.
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
}

extern "C" {
    fn tcp_newtcpcb(so: *mut c_void) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn tcp_attach__part2836(so: *mut Socket) -> c_int {
    unsafe {
        let tp = tcp_newtcpcb(so as *mut c_void);
        (*so).so_tcpcb = tp;
        if tp.is_null() {
            return -1;
        }
    }
    0
}
}

// ── `slirp_insque(so, &so->slirp->tcb); return 0;` · slirp/tcp_subr.c:465-467 · chunk 2837 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp_insque(so, &so->slirp->tcb);
//     
//      return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2837 {
use core::ffi::{c_int, c_void};

extern "C" {
    // void slirp_insque(void *, void *);
    fn slirp_insque(so: *mut c_void, tcb: *mut c_void);
}

// Replacement for: slirp_insque(so, &so->slirp->tcb); return 0;
#[no_mangle]
pub extern "C" fn tcp_attach__part2837(so: *mut c_void, tcb: *mut c_void) -> c_int {
    unsafe {
        slirp_insque(so, tcb);
    }
    0
}
}

// ── `tcp_tos` · slirp/tcp_subr.c:494-519 · chunk 2840 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint8_t
//     tcp_tos(struct socket *so)
//     {
//      int i = 0;
//      struct emu_t *emup;
//     
//      while(tcptos[i].tos) {
//       if ((tcptos[i].fport && (ntohs(so->so_fport) == tcptos[i].fport)) ||
//           (tcptos[i].lport && (ntohs(so->so_lport) == tcptos[i].lport))) {
//        so->so_emu = tcptos[i].emu;
//        return tcptos[i].tos;
//       }
//       i++;
//      }
//     
//     
//      for (emup = tcpemu; emup; emup = emup->next) {
//       if ((emup->fport && (ntohs(so->so_fport) == emup->fport)) ||
//           (emup->lport && (ntohs(so->so_lport) == emup->lport))) {
//        so->so_emu = emup->emu;
//        return emup->tos;
//       }
//      }
//     
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2840 {
// Rust rewrite of slirp's tcp_tos() (slirp/tcp_subr.c).
// The TOS table `tcptos` and the `tcpemu` list head are static (file-scope) in the
// C translation unit, so they are mirrored here instead of being extern symbols.

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

// Mirror of `struct socket` (offset-exact), only fport/lport/emu are read here.
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

#[repr(C)]
pub struct TosT {
    pub lport: u16,
    pub fport: u16,
    pub tos: u8,
    pub emu: u8,
}

#[repr(C)]
pub struct EmuT {
    pub lport: u16,
    pub fport: u16,
    pub tos: u8,
    pub emu: u8,
    pub next: *mut EmuT,
}

static TCPTOS: [TosT; 13] = [
    TosT { lport: 0, fport: 20, tos: 0x08, emu: 0 },
    TosT { lport: 21, fport: 21, tos: 0x10, emu: 0x2 },
    TosT { lport: 0, fport: 23, tos: 0x10, emu: 0 },
    TosT { lport: 0, fport: 80, tos: 0x08, emu: 0 },
    TosT { lport: 0, fport: 513, tos: 0x10, emu: 0x6 | 0x10 },
    TosT { lport: 0, fport: 514, tos: 0x10, emu: 0x8 | 0x10 },
    TosT { lport: 0, fport: 544, tos: 0x10, emu: 0x3 },
    TosT { lport: 0, fport: 543, tos: 0x10, emu: 0 },
    TosT { lport: 0, fport: 6667, tos: 0x08, emu: 0x4 },
    TosT { lport: 0, fport: 6668, tos: 0x08, emu: 0x4 },
    TosT { lport: 0, fport: 7070, tos: 0x10, emu: 0x5 },
    TosT { lport: 0, fport: 113, tos: 0x10, emu: 0x7 },
    TosT { lport: 0, fport: 0, tos: 0, emu: 0 },
];

// static struct emu_t *tcpemu = NULL;  (never assigned anywhere)
static mut TCPEMU: *mut EmuT = core::ptr::null_mut();

extern "C" {
    fn ntohs(__netshort: u16) -> u16;
}

#[no_mangle]
pub extern "C" fn tcp_tos(so: *mut Socket) -> u8 {
    if so.is_null() {
        return 0;
    }
    unsafe {
        let mut i: c_int = 0;
        let base = TCPTOS.as_ptr();
        loop {
            let e = &*base.offset(i as isize);
            if e.tos == 0 {
                break;
            }
            if (e.fport != 0 && ntohs((*so).so_fport) == e.fport)
                || (e.lport != 0 && ntohs((*so).so_lport) == e.lport)
            {
                (*so).so_emu = e.emu;
                return e.tos;
            }
            i += 1;
        }

        let mut emup: *mut EmuT = TCPEMU;
        while !emup.is_null() {
            let e = &*emup;
            if (e.fport != 0 && ntohs((*so).so_fport) == e.fport)
                || (e.lport != 0 && ntohs((*so).so_lport) == e.lport)
            {
                (*so).so_emu = e.emu;
                return e.tos;
            }
            emup = e.next;
        }

        0
    }
}
}

// ── `while (tcptos[i].tos)` · slirp/tcp_subr.c:500-507 · chunk 2841 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while(tcptos[i].tos) {
//       if ((tcptos[i].fport && (ntohs(so->so_fport) == tcptos[i].fport)) ||
//           (tcptos[i].lport && (ntohs(so->so_lport) == tcptos[i].lport))) {
//        so->so_emu = tcptos[i].emu;
//        return tcptos[i].tos;
//       }
//       i++;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2841 {
// Port of the tcptos[] lookup loop from tcp_tos() in slirp/tcp_subr.c.
use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

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

#[repr(C)]
#[derive(Clone, Copy)]
struct TosT {
    lport: u16,
    fport: u16,
    tos: u8,
    emu: u8,
}

// Mirror of the file-scope `static const struct tos_t tcptos[]` in the C
// translation unit (internal linkage: no linker symbol to reach for).
static TCPTOS: [TosT; 13] = [
    TosT { lport: 0, fport: 20, tos: 0x08, emu: 0 },
    TosT { lport: 21, fport: 21, tos: 0x10, emu: 0x2 },
    TosT { lport: 0, fport: 23, tos: 0x10, emu: 0 },
    TosT { lport: 0, fport: 80, tos: 0x08, emu: 0 },
    TosT { lport: 0, fport: 513, tos: 0x10, emu: 0x6 | 0x10 },
    TosT { lport: 0, fport: 514, tos: 0x10, emu: 0x8 | 0x10 },
    TosT { lport: 0, fport: 544, tos: 0x10, emu: 0x3 },
    TosT { lport: 0, fport: 543, tos: 0x10, emu: 0 },
    TosT { lport: 0, fport: 6667, tos: 0x08, emu: 0x4 },
    TosT { lport: 0, fport: 6668, tos: 0x08, emu: 0x4 },
    TosT { lport: 0, fport: 7070, tos: 0x10, emu: 0x5 },
    TosT { lport: 0, fport: 113, tos: 0x10, emu: 0x7 },
    TosT { lport: 0, fport: 0, tos: 0, emu: 0 },
];

/// Scans tcptos[]; on a match stores the emulation type into so->so_emu and
/// returns the (nonzero) tos value. Returns 0 when no entry matched.
#[no_mangle]
pub unsafe extern "C" fn tcp_tos__part2841(so: *mut Socket) -> u8 {
    let mut i: usize = 0;
    loop {
        let e: &TosT = match TCPTOS.get(i) {
            Some(e) => e,
            None => break,
        };
        if e.tos == 0 {
            break;
        }
        let fport_host = u16::from_be((*so).so_fport);
        let lport_host = u16::from_be((*so).so_lport);
        if (e.fport != 0 && fport_host == e.fport) || (e.lport != 0 && lport_host == e.lport) {
            (*so).so_emu = e.emu;
            return e.tos;
        }
        i += 1;
    }
    0
}
}

// ── `for (emup = tcpemu; emup; emup = emup->next)` · slirp/tcp_subr.c:510-516 · chunk 2842 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (emup = tcpemu; emup; emup = emup->next) {
//       if ((emup->fport && (ntohs(so->so_fport) == emup->fport)) ||
//           (emup->lport && (ntohs(so->so_lport) == emup->lport))) {
//        so->so_emu = emup->emu;
//        return emup->tos;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2842 {
// Port of the tcpemu linked-list lookup inside tcp_tos() from slirp/tcp_subr.c
use core::ffi::{c_int, c_uchar};

// Mirror of `struct emu_t` (exact field order and types).
#[repr(C)]
pub struct EmuT {
    pub lport: u16,
    pub fport: u16,
    pub tos: c_uchar,
    pub emu: c_uchar,
    pub next: *mut EmuT,
}

/// Walks the emulator list. Returns the matched `tos` value (0..=255) and,
/// on a match, stores the emulator type into `*so_emu`. Returns -1 when no
/// entry of the list matches.
#[no_mangle]
pub extern "C" fn tcp_tos__part2842(
    so_fport: u16,
    so_lport: u16,
    so_emu: *mut c_uchar,
    head: *mut EmuT,
) -> c_int {
    // ntohs() on the network-order ports
    let fport = u16::from_be(so_fport);
    let lport = u16::from_be(so_lport);

    let mut emup = head;
    while !emup.is_null() {
        unsafe {
            let e = &*emup;
            if (e.fport != 0 && fport == e.fport) || (e.lport != 0 && lport == e.lport) {
                if !so_emu.is_null() {
                    *so_emu = e.emu;
                }
                return e.tos as c_int;
            }
            emup = e.next;
        }
    }

    -1
}
}

// ── `return 0;` · slirp/tcp_subr.c:518-518 · chunk 2843 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2843 {
// Ported piece of tcp_tos(): the trailing `return 0;`
// No parameters are read and no state is modified.

#[no_mangle]
pub extern "C" fn tcp_tos__part2843() -> u8 {
    0
}
}

// ── `tcp_emu` · slirp/tcp_subr.c:545-877 · chunk 2844 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     tcp_emu(struct socket *so, struct mbuf *m)
//     {
//      Slirp *slirp = so->slirp;
//      u_int n1, n2, n3, n4, n5, n6;
//             char buff[257];
//      uint32_t laddr;
//      u_int lport;
//      char *bptr;
//     
//      ;
//      ;
//      ;
//     
//      switch(so->so_emu) {
//       int x, i;
//     
//       case 0x7:
//     
//     
//     
//     
//       {
//        struct socket *tmpso;
//        struct sockaddr_in addr;
//        socklen_t addrlen = sizeof(struct sockaddr_in);
//        struct sbuf *so_rcv = &so->so_rcv;
//     
//        memcpy(so_rcv->sb_wptr, m->m_hdr.mh_data, m->m_hdr.mh_len);
//        so_rcv->sb_wptr += m->m_hdr.mh_len;
//        so_rcv->sb_rptr += m->m_hdr.mh_len;
//        m->m_hdr.mh_data[m->m_hdr.mh_len] = 0;
//        if (
//           _Generic (0 ? (
//           m->m_hdr.mh_data
//           ) : (void *) 1, const void *: (const char *) (strchr (
//           m->m_hdr.mh_data
//           , 
//           '\r'
//           )), default: strchr (
//           m->m_hdr.mh_data
//           , 
//           '\r'
//           )) 
//                                   || 
//                                      _Generic (0 ? (
//                                      m->m_hdr.mh_data
//                                      ) : (void *) 1, const void *: (const char *) (strchr (
//                                      m->m_hdr.mh_data
//                                      , 
//                                      '\n'
//                                      )), default: strchr (
//                                      m->m_hdr.mh_data
//                                      , 
//                                      '\n'
//                                      ))
//                                                             ) {
//         if (sscanf(so_rcv->sb_data, "%u%*[ ,]%u", &n1, &n2) == 2) {
//          ((n1) = htons((uint16_t)(n1)));
//          ((n2) = htons((uint16_t)(n2)));
//     
//          for (tmpso = slirp->tcb.so_next;
//               tmpso != &slirp->tcb;
//               tmpso = tmpso->so_next) {
//           if (tmpso->so_laddr.s_addr == so->so_laddr.s_addr &&
//               tmpso->so_lport == n2 &&
//               tmpso->so_faddr.s_addr == so->so_faddr.s_addr &&
//               tmpso->so_fport == n1) {
//            if (getsockname(tmpso->s,
//             (struct sockaddr *)&addr, &addrlen) == 0)
//               n2 = ntohs(addr.sin_port);
//            break;
//           }
//          }
//         }
//                                     so_rcv->sb_cc = snprintf(so_rcv->sb_data,
//                                                              so_rcv->sb_datalen,
//                                                              "%d,%d\r\n", n1, n2);
//         so_rcv->sb_rptr = so_rcv->sb_data;
//         so_rcv->sb_wptr = so_rcv->sb_data + so_rcv->sb_cc;
//        }
//        m_free(m);
//        return 0;
//       }
//     
//             case 0x2:
//                     *(m->m_hdr.mh_data+m->m_hdr.mh_len) = 0;
//       if ((bptr = (char *)
//                          _Generic (0 ? (
//                          m->m_hdr.mh_data
//                          ) : (void *) 1, const void *: (const char *) (strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "ORT"
//                          )), default: strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "ORT"
//                          ))
//                                                  ) != 
//                                                       ((void *)0)
//                                                           ) {
//     
//     
//     
//        x = sscanf(bptr, "ORT %u,%u,%u,%u,%u,%u\r\n%256[^\177]",
//            &n1, &n2, &n3, &n4, &n5, &n6, buff);
//        if (x < 6)
//           return 1;
//     
//        laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | (n4));
//        lport = htons((n5 << 8) | (n6));
//     
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0, laddr,
//                             lport, 0x200)) == 
//                                                       ((void *)0)
//                                                           ) {
//           return 1;
//        }
//        n6 = ntohs(so->so_fport);
//     
//        n5 = (n6 >> 8) & 0xff;
//        n6 &= 0xff;
//     
//        laddr = ntohl(so->so_faddr.s_addr);
//     
//        n1 = ((laddr >> 24) & 0xff);
//        n2 = ((laddr >> 16) & 0xff);
//        n3 = ((laddr >> 8) & 0xff);
//        n4 = (laddr & 0xff);
//     
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size - m->m_hdr.mh_len,
//                                                  "ORT %d,%d,%d,%d,%d,%d\r\n%s",
//                                                  n1, n2, n3, n4, n5, n6, x==7?buff:"");
//        return 1;
//       } else if ((bptr = (char *)
//                                 _Generic (0 ? (
//                                 m->m_hdr.mh_data
//                                 ) : (void *) 1, const void *: (const char *) (strstr (
//                                 m->m_hdr.mh_data
//                                 , 
//                                 "27 Entering"
//                                 )), default: strstr (
//                                 m->m_hdr.mh_data
//                                 , 
//                                 "27 Entering"
//                                 ))
//                                                                 ) != 
//                                                                      ((void *)0)
//                                                                          ) {
//     
//     
//     
//        x = sscanf(bptr, "27 Entering Passive Mode (%u,%u,%u,%u,%u,%u)\r\n%256[^\177]",
//            &n1, &n2, &n3, &n4, &n5, &n6, buff);
//        if (x < 6)
//           return 1;
//     
//        laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | (n4));
//        lport = htons((n5 << 8) | (n6));
//     
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0, laddr,
//                             lport, 0x200)) == 
//                                                       ((void *)0)
//                                                           ) {
//           return 1;
//        }
//        n6 = ntohs(so->so_fport);
//     
//        n5 = (n6 >> 8) & 0xff;
//        n6 &= 0xff;
//     
//        laddr = ntohl(so->so_faddr.s_addr);
//     
//        n1 = ((laddr >> 24) & 0xff);
//        n2 = ((laddr >> 16) & 0xff);
//        n3 = ((laddr >> 8) & 0xff);
//        n4 = (laddr & 0xff);
//     
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//        m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size - m->m_hdr.mh_len,
//                                                  "27 Entering Passive Mode (%d,%d,%d,%d,%d,%d)\r\n%s",
//                                                  n1, n2, n3, n4, n5, n6, x==7?buff:"");
//     
//        return 1;
//       }
//     
//       return 1;
//     
//       case 0x3:
//     
//     
//     
//     
//     
//     
//       so->so_emu = 0;
//       for (lport = 0, i = 0; i < m->m_hdr.mh_len-1; ++i) {
//        if (m->m_hdr.mh_data[i] < '0' || m->m_hdr.mh_data[i] > '9')
//         return 1;
//        lport *= 10;
//        lport += m->m_hdr.mh_data[i] - '0';
//       }
//       if (m->m_hdr.mh_data[m->m_hdr.mh_len-1] == '\0' && lport != 0 &&
//           (so = tcp_listen(slirp, 
//                                  ((in_addr_t) 0x00000000)
//                                            , 0, so->so_laddr.s_addr,
//                            htons(lport), 0x200)) != 
//                                                             ((void *)0)
//                                                                 )
//                         m->m_hdr.mh_len = snprintf(m->m_hdr.mh_data, m->m_hdr.mh_size, "%d",
//                                             ntohs(so->so_fport)) + 1;
//       return 1;
//     
//       case 0x4:
//     
//     
//     
//       *(m->m_hdr.mh_data+m->m_hdr.mh_len) = 0;
//       if ((bptr = (char *)
//                          _Generic (0 ? (
//                          m->m_hdr.mh_data
//                          ) : (void *) 1, const void *: (const char *) (strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "DCC"
//                          )), default: strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "DCC"
//                          ))
//                                                  ) == 
//                                                       ((void *)0)
//                                                           )
//         return 1;
//     
//     
//       if (sscanf(bptr, "DCC CHAT %256s %u %u", buff, &laddr, &lport) == 3) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC CHAT chat %lu %u%c\n",
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), 1);
//       } else if (sscanf(bptr, "DCC SEND %256s %u %u %u", buff, &laddr, &lport, &n1) == 4) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC SEND %s %lu %u %u%c\n", buff,
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), n1, 1);
//       } else if (sscanf(bptr, "DCC MOVE %256s %u %u %u", buff, &laddr, &lport, &n1) == 4) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC MOVE %s %lu %u %u%c\n", buff,
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), n1, 1);
//       }
//       return 1;
//     
//       case 0x5:
//       bptr = m->m_hdr.mh_data;
//       while (bptr < m->m_hdr.mh_data + m->m_hdr.mh_len) {
//        u_short p;
//        static int ra = 0;
//        char ra_tbl[4];
//     
//        ra_tbl[0] = 0x50;
//        ra_tbl[1] = 0x4e;
//        ra_tbl[2] = 0x41;
//        ra_tbl[3] = 0;
//     
//        switch (ra) {
//         case 0:
//         case 2:
//         case 3:
//         if (*bptr++ != ra_tbl[ra]) {
//          ra = 0;
//          continue;
//         }
//         break;
//     
//         case 1:
//     
//     
//     
//         if (*bptr == 0x50) {
//          ra = 1;
//          bptr++;
//          continue;
//         } else if (*bptr++ != ra_tbl[ra]) {
//          ra = 0;
//          continue;
//         }
//         break;
//     
//         case 4:
//     
//     
//     
//         bptr++;
//         break;
//     
//         case 5:
//     
//     
//     
//     
//     
//         if (*(bptr + 1) == 0x02)
//            bptr += 8;
//         else
//            bptr += 4;
//         break;
//     
//         case 6:
//     
//     
//     
//         lport = (((u_char*)bptr)[0] << 8)
//         + ((u_char *)bptr)[1];
//         if (lport < 6970)
//            lport += 256;
//         if (lport < 6970 || lport > 7170)
//            return 1;
//     
//     
//         for (p = 6970; p < 7071; p++) {
//          if (udp_listen(slirp, 
//                               ((in_addr_t) 0x00000000)
//                                         ,
//                  htons(p),
//                  so->so_laddr.s_addr,
//                  htons(lport),
//                  0x200)) {
//           break;
//          }
//         }
//         if (p == 7071)
//            p = 0;
//         *(u_char *)bptr++ = (p >> 8) & 0xff;
//                                     *(u_char *)bptr = p & 0xff;
//         ra = 0;
//         return 1;
//         break;
//     
//         default:
//         ra = 0;
//        }
//        ra++;
//       }
//       return 1;
//     
//       default:
//     
//       so->so_emu = 0;
//       return 1;
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2844 {
#![allow(dead_code)]
#![allow(non_snake_case)]

// Rust port of slirp's tcp_emu() from slirp/tcp_subr.c

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::sync::atomic::{AtomicI32, Ordering};

const SS_FACCEPTONCE: c_int = 0x200;
const INADDR_ANY: u32 = 0;

// ---------------------------------------------------------------------------
// C structure mirrors (only what is reachable from tcp_emu)
// ---------------------------------------------------------------------------

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
pub struct SockAddrIn {
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

// union M_dat { char m_dat_[1]; char *m_ext_; }
#[repr(C)]
pub struct MdatUnion {
    pub m_ext: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MdatUnion,
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

// struct ipq is packed, no bitfields: raw bytes keep the layout identical
// (size 44, alignment 1).
#[repr(C)]
pub struct Ipq {
    pub raw: [u8; 44],
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
    pub exec_list: *mut c_void, // struct ex_list *
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

// ---------------------------------------------------------------------------
// libc / program symbols used
// ---------------------------------------------------------------------------

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn getsockname(fd: c_int, addr: *mut SockAddrIn, addrlen: *mut u32) -> c_int;
    fn m_free(m: *mut Mbuf);
    fn tcp_listen(
        slirp: *mut Slirp,
        haddr: u32,
        hport: c_uint,
        laddr: u32,
        lport: c_uint,
        flags: c_int,
    ) -> *mut Socket;
    fn udp_listen(
        slirp: *mut Slirp,
        haddr: u32,
        hport: c_uint,
        laddr: u32,
        lport: c_uint,
        flags: c_int,
    ) -> *mut Socket;
}

#[inline]
fn c_htons(x: u16) -> u16 {
    x.to_be()
}

#[inline]
fn c_ntohs(x: u16) -> u16 {
    u16::from_be(x)
}

#[inline]
fn c_htonl(x: u32) -> u32 {
    x.to_be()
}

#[inline]
fn c_ntohl(x: u32) -> u32 {
    u32::from_be(x)
}

// file-scope `static int ra` of the REALAUDIO emulation
static RA: AtomicI32 = AtomicI32::new(0);

#[inline]
fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

// ---------------------------------------------------------------------------
// tcp_emu
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn tcp_emu(so_arg: *mut Socket, m: *mut Mbuf) -> c_int {
    unsafe { tcp_emu_inner(so_arg, m) }
}

unsafe fn tcp_emu_inner(so_arg: *mut Socket, m: *mut Mbuf) -> c_int {
    let mut so: *mut Socket = so_arg;
    let slirp: *mut Slirp = (*so).slirp;
    let mut n1: c_uint = 0;
    let mut n2: c_uint = 0;
    let mut n3: c_uint = 0;
    let mut n4: c_uint = 0;
    let mut n5: c_uint = 0;
    let mut n6: c_uint = 0;
    let mut buff: [c_char; 257] = [0; 257];
    let mut laddr: u32 = 0;
    let mut lport: c_uint = 0;

    match (*so).so_emu {
        // ------------------------------------------------------ EMU_IDENT
        0x7 => {
            let mut tmpso: *mut Socket;
            let mut addr = SockAddrIn {
                sin_family: 0,
                sin_port: 0,
                sin_addr: InAddr { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut addrlen: u32 = core::mem::size_of::<SockAddrIn>() as u32;
            let so_rcv: *mut Sbuf = core::ptr::addr_of_mut!((*so).so_rcv);
            let tcb_head: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);

            memcpy(
                (*so_rcv).sb_wptr as *mut c_void,
                (*m).m_hdr.mh_data as *const c_void,
                (*m).m_hdr.mh_len as usize,
            );
            (*so_rcv).sb_wptr = (*so_rcv).sb_wptr.offset((*m).m_hdr.mh_len as isize);
            (*so_rcv).sb_rptr = (*so_rcv).sb_rptr.offset((*m).m_hdr.mh_len as isize);
            *(*m).m_hdr.mh_data.offset((*m).m_hdr.mh_len as isize) = 0;

            if !strchr((*m).m_hdr.mh_data, 13).is_null()
                || !strchr((*m).m_hdr.mh_data, 10).is_null()
            {
                if sscanf(
                    (*so_rcv).sb_data,
                    cstr(b"%u%*[ ,]%u\0"),
                    &mut n1,
                    &mut n2,
                ) == 2
                {
                    n1 = c_htons(n1 as u16) as c_uint;
                    n2 = c_htons(n2 as u16) as c_uint;

                    // n2 is the one on our host
                    tmpso = (*slirp).tcb.so_next;
                    while tmpso != tcb_head {
                        if (*tmpso).so_laddr.s_addr == (*so).so_laddr.s_addr
                            && (*tmpso).so_lport as c_uint == n2
                            && (*tmpso).so_faddr.s_addr == (*so).so_faddr.s_addr
                            && (*tmpso).so_fport as c_uint == n1
                        {
                            if getsockname((*tmpso).s, &mut addr, &mut addrlen) == 0 {
                                n2 = c_ntohs(addr.sin_port) as c_uint;
                            }
                            break;
                        }
                        tmpso = (*tmpso).so_next;
                    }
                }
                (*so_rcv).sb_cc = snprintf(
                    (*so_rcv).sb_data,
                    (*so_rcv).sb_datalen as usize,
                    cstr(b"%d,%d\r\n\0"),
                    n1 as c_int,
                    n2 as c_int,
                ) as c_uint;
                (*so_rcv).sb_rptr = (*so_rcv).sb_data;
                (*so_rcv).sb_wptr = (*so_rcv).sb_data.offset((*so_rcv).sb_cc as isize);
            }
            m_free(m);
            return 0;
        }

        // -------------------------------------------------------- EMU_FTP
        0x2 => {
            *(*m).m_hdr.mh_data.offset((*m).m_hdr.mh_len as isize) = 0; // NUL terminate

            let bptr = strstr((*m).m_hdr.mh_data, cstr(b"ORT\0"));
            if !bptr.is_null() {
                // Emulate the PORT command
                let x = sscanf(
                    bptr,
                    cstr(b"ORT %u,%u,%u,%u,%u,%u\r\n%256[^\x7f]\0"),
                    &mut n1,
                    &mut n2,
                    &mut n3,
                    &mut n4,
                    &mut n5,
                    &mut n6,
                    buff.as_mut_ptr(),
                );
                if x < 6 {
                    return 1;
                }

                laddr = c_htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | n4);
                lport = c_htons(((n5 << 8) | n6) as u16) as c_uint;

                so = tcp_listen(slirp, INADDR_ANY, 0, laddr, lport, SS_FACCEPTONCE);
                if so.is_null() {
                    return 1;
                }
                n6 = c_ntohs((*so).so_fport) as c_uint;

                n5 = (n6 >> 8) & 0xff;
                n6 &= 0xff;

                laddr = c_ntohl((*so).so_faddr.s_addr);

                n1 = (laddr >> 24) & 0xff;
                n2 = (laddr >> 16) & 0xff;
                n3 = (laddr >> 8) & 0xff;
                n4 = laddr & 0xff;

                (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                let sz: usize = (*m).m_hdr.mh_size.wrapping_sub((*m).m_hdr.mh_len) as usize;
                let tail: *const c_char = if x == 7 {
                    buff.as_ptr()
                } else {
                    cstr(b"\0")
                };
                (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                    bptr,
                    sz,
                    cstr(b"ORT %d,%d,%d,%d,%d,%d\r\n%s\0"),
                    n1 as c_int,
                    n2 as c_int,
                    n3 as c_int,
                    n4 as c_int,
                    n5 as c_int,
                    n6 as c_int,
                    tail,
                ));
                return 1;
            }

            let bptr = strstr((*m).m_hdr.mh_data, cstr(b"27 Entering\0"));
            if !bptr.is_null() {
                // Emulate the PASV response
                let x = sscanf(
                    bptr,
                    cstr(b"27 Entering Passive Mode (%u,%u,%u,%u,%u,%u)\r\n%256[^\x7f]\0"),
                    &mut n1,
                    &mut n2,
                    &mut n3,
                    &mut n4,
                    &mut n5,
                    &mut n6,
                    buff.as_mut_ptr(),
                );
                if x < 6 {
                    return 1;
                }

                laddr = c_htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | n4);
                lport = c_htons(((n5 << 8) | n6) as u16) as c_uint;

                so = tcp_listen(slirp, INADDR_ANY, 0, laddr, lport, SS_FACCEPTONCE);
                if so.is_null() {
                    return 1;
                }
                n6 = c_ntohs((*so).so_fport) as c_uint;

                n5 = (n6 >> 8) & 0xff;
                n6 &= 0xff;

                laddr = c_ntohl((*so).so_faddr.s_addr);

                n1 = (laddr >> 24) & 0xff;
                n2 = (laddr >> 16) & 0xff;
                n3 = (laddr >> 8) & 0xff;
                n4 = laddr & 0xff;

                (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                let sz: usize = (*m).m_hdr.mh_size.wrapping_sub((*m).m_hdr.mh_len) as usize;
                let tail: *const c_char = if x == 7 {
                    buff.as_ptr()
                } else {
                    cstr(b"\0")
                };
                (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                    bptr,
                    sz,
                    cstr(b"27 Entering Passive Mode (%d,%d,%d,%d,%d,%d)\r\n%s\0"),
                    n1 as c_int,
                    n2 as c_int,
                    n3 as c_int,
                    n4 as c_int,
                    n5 as c_int,
                    n6 as c_int,
                    tail,
                ));

                return 1;
            }

            return 1;
        }

        // -------------------------------------------------------- EMU_KSH
        0x3 => {
            (*so).so_emu = 0;
            lport = 0;
            let mut i: c_int = 0;
            while i < (*m).m_hdr.mh_len.wrapping_sub(1) {
                let c = *(*m).m_hdr.mh_data.offset(i as isize) as c_int;
                if c < 0x30 || c > 0x39 {
                    return 1; // invalid number
                }
                lport = lport.wrapping_mul(10);
                lport = lport.wrapping_add((c - 0x30) as c_uint);
                i += 1;
            }
            if *(*m).m_hdr.mh_data.offset((*m).m_hdr.mh_len.wrapping_sub(1) as isize) == 0
                && lport != 0
            {
                // evaluate the listen address before `so` is replaced
                let laddr_arg: u32 = (*so).so_laddr.s_addr;
                let newso = tcp_listen(
                    slirp,
                    INADDR_ANY,
                    0,
                    laddr_arg,
                    c_htons(lport as u16) as c_uint,
                    SS_FACCEPTONCE,
                );
                so = newso;
                if !newso.is_null() {
                    (*m).m_hdr.mh_len = snprintf(
                        (*m).m_hdr.mh_data,
                        (*m).m_hdr.mh_size as usize,
                        cstr(b"%d\0"),
                        c_ntohs((*so).so_fport) as c_int,
                    )
                    .wrapping_add(1);
                }
            }
            return 1;
        }

        // ------------------------------------------------------- EMU_IRC
        0x4 => {
            *(*m).m_hdr.mh_data.offset((*m).m_hdr.mh_len as isize) = 0; // NUL terminate

            let bptr = strstr((*m).m_hdr.mh_data, cstr(b"DCC\0"));
            if bptr.is_null() {
                return 1;
            }

            // The %256s is for the broken mIRC
            if sscanf(
                bptr,
                cstr(b"DCC CHAT %256s %u %u\0"),
                buff.as_mut_ptr(),
                &mut laddr,
                &mut lport,
            ) == 3
            {
                so = tcp_listen(
                    slirp,
                    INADDR_ANY,
                    0,
                    c_htonl(laddr),
                    c_htons(lport as u16) as c_uint,
                    SS_FACCEPTONCE,
                );
                if so.is_null() {
                    return 1;
                }
                (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                    bptr,
                    (*m).m_hdr.mh_size as usize,
                    cstr(b"DCC CHAT chat %lu %u%c\n\0"),
                    c_ntohl((*so).so_faddr.s_addr) as c_ulong,
                    c_ntohs((*so).so_fport) as c_uint,
                    1 as c_int,
                ));
            } else if sscanf(
                bptr,
                cstr(b"DCC SEND %256s %u %u %u\0"),
                buff.as_mut_ptr(),
                &mut laddr,
                &mut lport,
                &mut n1,
            ) == 4
            {
                so = tcp_listen(
                    slirp,
                    INADDR_ANY,
                    0,
                    c_htonl(laddr),
                    c_htons(lport as u16) as c_uint,
                    SS_FACCEPTONCE,
                );
                if so.is_null() {
                    return 1;
                }
                (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                    bptr,
                    (*m).m_hdr.mh_size as usize,
                    cstr(b"DCC SEND %s %lu %u %u%c\n\0"),
                    buff.as_ptr(),
                    c_ntohl((*so).so_faddr.s_addr) as c_ulong,
                    c_ntohs((*so).so_fport) as c_uint,
                    n1 as c_uint,
                    1 as c_int,
                ));
            } else if sscanf(
                bptr,
                cstr(b"DCC MOVE %256s %u %u %u\0"),
                buff.as_mut_ptr(),
                &mut laddr,
                &mut lport,
                &mut n1,
            ) == 4
            {
                so = tcp_listen(
                    slirp,
                    INADDR_ANY,
                    0,
                    c_htonl(laddr),
                    c_htons(lport as u16) as c_uint,
                    SS_FACCEPTONCE,
                );
                if so.is_null() {
                    return 1;
                }
                (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                    bptr,
                    (*m).m_hdr.mh_size as usize,
                    cstr(b"DCC MOVE %s %lu %u %u%c\n\0"),
                    buff.as_ptr(),
                    c_ntohl((*so).so_faddr.s_addr) as c_ulong,
                    c_ntohs((*so).so_fport) as c_uint,
                    n1 as c_uint,
                    1 as c_int,
                ));
            }
            return 1;
        }

        // ------------------------------------------------- EMU_REALAUDIO
        0x5 => {
            let mut bptr: *mut c_char = (*m).m_hdr.mh_data;
            let end: usize = (*m).m_hdr.mh_data as usize + (*m).m_hdr.mh_len as usize;
            let mut ra: c_int = RA.load(Ordering::Relaxed);

            while (bptr as usize) < end {
                let mut p: u16 = 0;
                // ra_tbl = { 0x50, 0x4e, 0x41, 0 }
                let expect: c_char = match ra {
                    0 => 0x50,
                    1 => 0x4e,
                    2 => 0x41,
                    3 => 0,
                    _ => 0,
                };

                match ra {
                    0 | 2 | 3 => {
                        let c = *bptr;
                        bptr = bptr.offset(1);
                        if c != expect {
                            ra = 0;
                            continue;
                        }
                    }

                    1 => {
                        // We may get 0x50 several times, ignore them
                        let c = *bptr;
                        if c == 0x50 {
                            ra = 1;
                            bptr = bptr.offset(1);
                            continue;
                        }
                        bptr = bptr.offset(1);
                        if c != expect {
                            ra = 0;
                            continue;
                        }
                    }

                    4 => {
                        // skip version number
                        bptr = bptr.offset(1);
                    }

                    5 => {
                        // difference between versions 1.0 and 2.0
                        if *bptr.offset(1) == 0x02 {
                            bptr = bptr.offset(8);
                        } else {
                            bptr = bptr.offset(4);
                        }
                    }

                    6 => {
                        // field containing the port number
                        let b0 = *(bptr as *const u8) as c_uint;
                        let b1 = *((bptr as *const u8).offset(1)) as c_uint;
                        lport = (b0 << 8).wrapping_add(b1);
                        if lport < 6970 {
                            lport = lport.wrapping_add(256); // don't know why
                        }
                        if lport < 6970 || lport > 7170 {
                            RA.store(ra, Ordering::Relaxed);
                            return 1; // failed
                        }

                        // try to get udp port between 6970 - 7170
                        p = 6970;
                        while p < 7071 {
                            let sladdr: u32 = (*so).so_laddr.s_addr;
                            let res = udp_listen(
                                slirp,
                                INADDR_ANY,
                                c_htons(p) as c_uint,
                                sladdr,
                                c_htons(lport as u16) as c_uint,
                                SS_FACCEPTONCE,
                            );
                            if !res.is_null() {
                                break;
                            }
                            p = p.wrapping_add(1);
                        }
                        if p == 7071 {
                            p = 0;
                        }
                        *(bptr as *mut u8) = ((p >> 8) & 0xff) as u8;
                        bptr = bptr.offset(1);
                        *(bptr as *mut u8) = (p & 0xff) as u8;
                        RA.store(0, Ordering::Relaxed);
                        return 1; // port redirected, we're done
                    }

                    _ => {
                        ra = 0;
                    }
                }
                ra += 1;
            }
            RA.store(ra, Ordering::Relaxed);
            return 1;
        }

        // ---------------------------------------------------------- default
        _ => {
            // Ooops, not emulated, won't call tcp_emu again
            (*so).so_emu = 0;
            return 1;
        }
    }
}
}

// ── `Slirp *slirp = so->slirp; u_int n1, n2, n3, n4, n5, n6; char buff[257]; uint32_t laddr; u_int l…` · slirp/tcp_subr.c:548-557 · chunk 2845 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = so->slirp;
//      u_int n1, n2, n3, n4, n5, n6;
//             char buff[257];
//      uint32_t laddr;
//      u_int lport;
//      char *bptr;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2845 {
// Port of the declaration/initialization prologue of tcp_emu:
//   Slirp *slirp = so->slirp;
// Only the field read through `so` is needed, so `struct socket` is mirrored
// up to and including the `slirp` member (same C order/types, repr(C)).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
}

#[no_mangle]
pub extern "C" fn tcp_emu__part2845(so: *mut Socket) -> *mut c_void {
    unsafe { (*so).slirp }
}
}

// ── `switch(so->so_emu) { int x, i; case 0x7: { struct socket *tmpso; struct sockaddr_in addr; sockl…` · slirp/tcp_subr.c:559-876 · chunk 2846 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(so->so_emu) {
//       int x, i;
//     
//       case 0x7:
//     
//     
//     
//     
//       {
//        struct socket *tmpso;
//        struct sockaddr_in addr;
//        socklen_t addrlen = sizeof(struct sockaddr_in);
//        struct sbuf *so_rcv = &so->so_rcv;
//     
//        memcpy(so_rcv->sb_wptr, m->m_hdr.mh_data, m->m_hdr.mh_len);
//        so_rcv->sb_wptr += m->m_hdr.mh_len;
//        so_rcv->sb_rptr += m->m_hdr.mh_len;
//        m->m_hdr.mh_data[m->m_hdr.mh_len] = 0;
//        if (
//           _Generic (0 ? (
//           m->m_hdr.mh_data
//           ) : (void *) 1, const void *: (const char *) (strchr (
//           m->m_hdr.mh_data
//           , 
//           '\r'
//           )), default: strchr (
//           m->m_hdr.mh_data
//           , 
//           '\r'
//           )) 
//                                   || 
//                                      _Generic (0 ? (
//                                      m->m_hdr.mh_data
//                                      ) : (void *) 1, const void *: (const char *) (strchr (
//                                      m->m_hdr.mh_data
//                                      , 
//                                      '\n'
//                                      )), default: strchr (
//                                      m->m_hdr.mh_data
//                                      , 
//                                      '\n'
//                                      ))
//                                                             ) {
//         if (sscanf(so_rcv->sb_data, "%u%*[ ,]%u", &n1, &n2) == 2) {
//          ((n1) = htons((uint16_t)(n1)));
//          ((n2) = htons((uint16_t)(n2)));
//     
//          for (tmpso = slirp->tcb.so_next;
//               tmpso != &slirp->tcb;
//               tmpso = tmpso->so_next) {
//           if (tmpso->so_laddr.s_addr == so->so_laddr.s_addr &&
//               tmpso->so_lport == n2 &&
//               tmpso->so_faddr.s_addr == so->so_faddr.s_addr &&
//               tmpso->so_fport == n1) {
//            if (getsockname(tmpso->s,
//             (struct sockaddr *)&addr, &addrlen) == 0)
//               n2 = ntohs(addr.sin_port);
//            break;
//           }
//          }
//         }
//                                     so_rcv->sb_cc = snprintf(so_rcv->sb_data,
//                                                              so_rcv->sb_datalen,
//                                                              "%d,%d\r\n", n1, n2);
//         so_rcv->sb_rptr = so_rcv->sb_data;
//         so_rcv->sb_wptr = so_rcv->sb_data + so_rcv->sb_cc;
//        }
//        m_free(m);
//        return 0;
//       }
//     
//             case 0x2:
//                     *(m->m_hdr.mh_data+m->m_hdr.mh_len) = 0;
//       if ((bptr = (char *)
//                          _Generic (0 ? (
//                          m->m_hdr.mh_data
//                          ) : (void *) 1, const void *: (const char *) (strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "ORT"
//                          )), default: strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "ORT"
//                          ))
//                                                  ) != 
//                                                       ((void *)0)
//                                                           ) {
//     
//     
//     
//        x = sscanf(bptr, "ORT %u,%u,%u,%u,%u,%u\r\n%256[^\177]",
//            &n1, &n2, &n3, &n4, &n5, &n6, buff);
//        if (x < 6)
//           return 1;
//     
//        laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | (n4));
//        lport = htons((n5 << 8) | (n6));
//     
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0, laddr,
//                             lport, 0x200)) == 
//                                                       ((void *)0)
//                                                           ) {
//           return 1;
//        }
//        n6 = ntohs(so->so_fport);
//     
//        n5 = (n6 >> 8) & 0xff;
//        n6 &= 0xff;
//     
//        laddr = ntohl(so->so_faddr.s_addr);
//     
//        n1 = ((laddr >> 24) & 0xff);
//        n2 = ((laddr >> 16) & 0xff);
//        n3 = ((laddr >> 8) & 0xff);
//        n4 = (laddr & 0xff);
//     
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size - m->m_hdr.mh_len,
//                                                  "ORT %d,%d,%d,%d,%d,%d\r\n%s",
//                                                  n1, n2, n3, n4, n5, n6, x==7?buff:"");
//        return 1;
//       } else if ((bptr = (char *)
//                                 _Generic (0 ? (
//                                 m->m_hdr.mh_data
//                                 ) : (void *) 1, const void *: (const char *) (strstr (
//                                 m->m_hdr.mh_data
//                                 , 
//                                 "27 Entering"
//                                 )), default: strstr (
//                                 m->m_hdr.mh_data
//                                 , 
//                                 "27 Entering"
//                                 ))
//                                                                 ) != 
//                                                                      ((void *)0)
//                                                                          ) {
//     
//     
//     
//        x = sscanf(bptr, "27 Entering Passive Mode (%u,%u,%u,%u,%u,%u)\r\n%256[^\177]",
//            &n1, &n2, &n3, &n4, &n5, &n6, buff);
//        if (x < 6)
//           return 1;
//     
//        laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | (n4));
//        lport = htons((n5 << 8) | (n6));
//     
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0, laddr,
//                             lport, 0x200)) == 
//                                                       ((void *)0)
//                                                           ) {
//           return 1;
//        }
//        n6 = ntohs(so->so_fport);
//     
//        n5 = (n6 >> 8) & 0xff;
//        n6 &= 0xff;
//     
//        laddr = ntohl(so->so_faddr.s_addr);
//     
//        n1 = ((laddr >> 24) & 0xff);
//        n2 = ((laddr >> 16) & 0xff);
//        n3 = ((laddr >> 8) & 0xff);
//        n4 = (laddr & 0xff);
//     
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//        m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size - m->m_hdr.mh_len,
//                                                  "27 Entering Passive Mode (%d,%d,%d,%d,%d,%d)\r\n%s",
//                                                  n1, n2, n3, n4, n5, n6, x==7?buff:"");
//     
//        return 1;
//       }
//     
//       return 1;
//     
//       case 0x3:
//     
//     
//     
//     
//     
//     
//       so->so_emu = 0;
//       for (lport = 0, i = 0; i < m->m_hdr.mh_len-1; ++i) {
//        if (m->m_hdr.mh_data[i] < '0' || m->m_hdr.mh_data[i] > '9')
//         return 1;
//        lport *= 10;
//        lport += m->m_hdr.mh_data[i] - '0';
//       }
//       if (m->m_hdr.mh_data[m->m_hdr.mh_len-1] == '\0' && lport != 0 &&
//           (so = tcp_listen(slirp, 
//                                  ((in_addr_t) 0x00000000)
//                                            , 0, so->so_laddr.s_addr,
//                            htons(lport), 0x200)) != 
//                                                             ((void *)0)
//                                                                 )
//                         m->m_hdr.mh_len = snprintf(m->m_hdr.mh_data, m->m_hdr.mh_size, "%d",
//                                             ntohs(so->so_fport)) + 1;
//       return 1;
//     
//       case 0x4:
//     
//     
//     
//       *(m->m_hdr.mh_data+m->m_hdr.mh_len) = 0;
//       if ((bptr = (char *)
//                          _Generic (0 ? (
//                          m->m_hdr.mh_data
//                          ) : (void *) 1, const void *: (const char *) (strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "DCC"
//                          )), default: strstr (
//                          m->m_hdr.mh_data
//                          , 
//                          "DCC"
//                          ))
//                                                  ) == 
//                                                       ((void *)0)
//                                                           )
//         return 1;
//     
//     
//       if (sscanf(bptr, "DCC CHAT %256s %u %u", buff, &laddr, &lport) == 3) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC CHAT chat %lu %u%c\n",
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), 1);
//       } else if (sscanf(bptr, "DCC SEND %256s %u %u %u", buff, &laddr, &lport, &n1) == 4) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC SEND %s %lu %u %u%c\n", buff,
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), n1, 1);
//       } else if (sscanf(bptr, "DCC MOVE %256s %u %u %u", buff, &laddr, &lport, &n1) == 4) {
//        if ((so = tcp_listen(slirp, 
//                                   ((in_addr_t) 0x00000000)
//                                             , 0,
//                             htonl(laddr), htons(lport),
//                             0x200)) == 
//                                                ((void *)0)
//                                                    ) {
//         return 1;
//        }
//        m->m_hdr.mh_len = bptr - m->m_hdr.mh_data;
//                             m->m_hdr.mh_len += snprintf(bptr, m->m_hdr.mh_size,
//                                                  "DCC MOVE %s %lu %u %u%c\n", buff,
//                                                  (unsigned long)ntohl(so->so_faddr.s_addr),
//                                                  ntohs(so->so_fport), n1, 1);
//       }
//       return 1;
//     
//       case 0x5:
//       bptr = m->m_hdr.mh_data;
//       while (bptr < m->m_hdr.mh_data + m->m_hdr.mh_len) {
//        u_short p;
//        static int ra = 0;
//        char ra_tbl[4];
//     
//        ra_tbl[0] = 0x50;
//        ra_tbl[1] = 0x4e;
//        ra_tbl[2] = 0x41;
//        ra_tbl[3] = 0;
//     
//        switch (ra) {
//         case 0:
//         case 2:
//         case 3:
//         if (*bptr++ != ra_tbl[ra]) {
//          ra = 0;
//          continue;
//         }
//         break;
//     
//         case 1:
//     
//     
//     
//         if (*bptr == 0x50) {
//          ra = 1;
//          bptr++;
//          continue;
//         } else if (*bptr++ != ra_tbl[ra]) {
//          ra = 0;
//          continue;
//         }
//         break;
//     
//         case 4:
//     
//     
//     
//         bptr++;
//         break;
//     
//         case 5:
//     
//     
//     
//     
//     
//         if (*(bptr + 1) == 0x02)
//            bptr += 8;
//         else
//            bptr += 4;
//         break;
//     
//         case 6:
//     
//     
//     
//         lport = (((u_char*)bptr)[0] << 8)
//         + ((u_char *)bptr)[1];
//         if (lport < 6970)
//            lport += 256;
//         if (lport < 6970 || lport > 7170)
//            return 1;
//     
//     
//         for (p = 6970; p < 7071; p++) {
//          if (udp_listen(slirp, 
//                               ((in_addr_t) 0x00000000)
//                                         ,
//                  htons(p),
//                  so->so_laddr.s_addr,
//                  htons(lport),
//                  0x200)) {
//           break;
//          }
//         }
//         if (p == 7071)
//            p = 0;
//         *(u_char *)bptr++ = (p >> 8) & 0xff;
//                                     *(u_char *)bptr = p & 0xff;
//         ra = 0;
//         return 1;
//         break;
//     
//         default:
//         ra = 0;
//        }
//        ra++;
//       }
//       return 1;
//     
//       default:
//     
//       so->so_emu = 0;
//       return 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2846 {
// Port of tcp_emu's switch(so->so_emu) block (slirp/tcp_subr.c lines 559-876).
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

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
    pub slirp: *mut c_void,
    pub m_dat: [u8; 8],
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
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(s: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn getsockname(fd: c_int, addr: *mut SockaddrIn, addrlen: *mut c_uint) -> c_int;
    fn htons(x: u16) -> u16;
    fn ntohs(x: u16) -> u16;
    fn htonl(x: u32) -> u32;
    fn ntohl(x: u32) -> u32;
    fn tcp_listen(
        slirp: *mut c_void,
        laddr: u32,
        lport: c_uint,
        faddr: u32,
        fport: c_uint,
        flags: c_int,
    ) -> *mut Socket;
    fn udp_listen(
        slirp: *mut c_void,
        laddr: u32,
        lport: c_uint,
        faddr: u32,
        fport: c_uint,
        flags: c_int,
    ) -> *mut Socket;
    fn m_free(m: *mut Mbuf);
}

// char ra_tbl[4] = { 0x50, 0x4e, 0x41, 0 };
fn ra_tbl_at(idx: c_int) -> c_char {
    match idx {
        0 => 0x50u8 as c_char,
        1 => 0x4eu8 as c_char,
        2 => 0x41u8 as c_char,
        _ => 0i8 as c_char,
    }
}

// static int ra = 0;   (function-local static in the C original)
static mut RA: c_int = 0;

#[no_mangle]
pub extern "C" fn tcp_emu__part2846(
    so_in: *mut Socket,
    m: *mut Mbuf,
    tcb: *mut Socket,
) -> c_int {
    let mut n1: c_uint = 0;
    let mut n2: c_uint = 0;
    let mut n3: c_uint = 0;
    let mut n4: c_uint = 0;
    let mut n5: c_uint = 0;
    let mut n6: c_uint = 0;
    let mut buff: [c_char; 257] = [0; 257];
    let empty_str: [c_char; 1] = [0];
    let mut laddr: u32 = 0;
    let mut lport: c_uint = 0;
    let mut so: *mut Socket = so_in;

    unsafe {
        let emu = (*so).so_emu as c_int;
        match emu {
            0x7 => {
                let so_rcv: *mut Sbuf = &mut (*so).so_rcv;
                let data = (*m).m_hdr.mh_data;
                let len = (*m).m_hdr.mh_len;

                memcpy(
                    (*so_rcv).sb_wptr as *mut c_void,
                    data as *const c_void,
                    len as usize,
                );
                (*so_rcv).sb_wptr = (*so_rcv).sb_wptr.add(len as usize);
                (*so_rcv).sb_rptr = (*so_rcv).sb_rptr.add(len as usize);
                *data.add(len as usize) = 0;

                if !strchr(data, '\r' as c_int).is_null() || !strchr(data, '\n' as c_int).is_null()
                {
                    if sscanf(
                        (*so_rcv).sb_data,
                        b"%u%*[ ,]%u\0".as_ptr() as *const c_char,
                        &mut n1 as *mut c_uint,
                        &mut n2 as *mut c_uint,
                    ) == 2
                    {
                        n1 = htons(n1 as u16) as c_uint;
                        n2 = htons(n2 as u16) as c_uint;

                        let mut tmpso: *mut Socket = (*tcb).so_next;
                        while tmpso != tcb {
                            if (*tmpso).so_laddr.s_addr == (*so).so_laddr.s_addr
                                && (*tmpso).so_lport as c_uint == n2
                                && (*tmpso).so_faddr.s_addr == (*so).so_faddr.s_addr
                                && (*tmpso).so_fport as c_uint == n1
                            {
                                let mut addr = SockaddrIn {
                                    sin_family: 0,
                                    sin_port: 0,
                                    sin_addr: InAddr { s_addr: 0 },
                                    sin_zero: [0; 8],
                                };
                                let mut addrlen: c_uint = 16; // sizeof(struct sockaddr_in)
                                if getsockname((*tmpso).s, &mut addr, &mut addrlen) == 0 {
                                    n2 = ntohs(addr.sin_port) as c_uint;
                                }
                                break;
                            }
                            tmpso = (*tmpso).so_next;
                        }
                    }
                    (*so_rcv).sb_cc = snprintf(
                        (*so_rcv).sb_data,
                        (*so_rcv).sb_datalen as usize,
                        b"%d,%d\r\n\0".as_ptr() as *const c_char,
                        n1 as c_int,
                        n2 as c_int,
                    ) as c_uint;
                    (*so_rcv).sb_rptr = (*so_rcv).sb_data;
                    (*so_rcv).sb_wptr = (*so_rcv).sb_data.add((*so_rcv).sb_cc as usize);
                }
                m_free(m);
                return 0;
            }

            0x2 => {
                *(*m).m_hdr.mh_data.add((*m).m_hdr.mh_len as usize) = 0;
                let mut bptr = strstr(
                    (*m).m_hdr.mh_data,
                    b"ORT\0".as_ptr() as *const c_char,
                );
                if !bptr.is_null() {
                    let x = sscanf(
                        bptr,
                        b"ORT %u,%u,%u,%u,%u,%u\r\n%256[^\x7f]\0".as_ptr() as *const c_char,
                        &mut n1 as *mut c_uint,
                        &mut n2 as *mut c_uint,
                        &mut n3 as *mut c_uint,
                        &mut n4 as *mut c_uint,
                        &mut n5 as *mut c_uint,
                        &mut n6 as *mut c_uint,
                        buff.as_mut_ptr(),
                    );
                    if x < 6 {
                        return 1;
                    }

                    laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | n4);
                    lport = htons(((n5 << 8) | n6) as u16) as c_uint;

                    so = tcp_listen((*so).slirp, 0, 0, laddr, lport, 0x200);
                    if so.is_null() {
                        return 1;
                    }
                    n6 = ntohs((*so).so_fport) as c_uint;

                    n5 = (n6 >> 8) & 0xff;
                    n6 &= 0xff;

                    laddr = ntohl((*so).so_faddr.s_addr);

                    n1 = (laddr >> 24) & 0xff;
                    n2 = (laddr >> 16) & 0xff;
                    n3 = (laddr >> 8) & 0xff;
                    n4 = laddr & 0xff;

                    (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                        bptr,
                        (*m).m_hdr.mh_size.wrapping_sub((*m).m_hdr.mh_len) as usize,
                        b"ORT %d,%d,%d,%d,%d,%d\r\n%s\0".as_ptr() as *const c_char,
                        n1 as c_int,
                        n2 as c_int,
                        n3 as c_int,
                        n4 as c_int,
                        n5 as c_int,
                        n6 as c_int,
                        if x == 7 {
                            buff.as_mut_ptr()
                        } else {
                            empty_str.as_ptr() as *mut c_char
                        },
                    ));
                    return 1;
                } else {
                    bptr = strstr(
                        (*m).m_hdr.mh_data,
                        b"27 Entering\0".as_ptr() as *const c_char,
                    );
                    if !bptr.is_null() {
                        let x = sscanf(
                            bptr,
                            b"27 Entering Passive Mode (%u,%u,%u,%u,%u,%u)\r\n%256[^\x7f]\0"
                                .as_ptr() as *const c_char,
                            &mut n1 as *mut c_uint,
                            &mut n2 as *mut c_uint,
                            &mut n3 as *mut c_uint,
                            &mut n4 as *mut c_uint,
                            &mut n5 as *mut c_uint,
                            &mut n6 as *mut c_uint,
                            buff.as_mut_ptr(),
                        );
                        if x < 6 {
                            return 1;
                        }

                        laddr = htonl((n1 << 24) | (n2 << 16) | (n3 << 8) | n4);
                        lport = htons(((n5 << 8) | n6) as u16) as c_uint;

                        so = tcp_listen((*so).slirp, 0, 0, laddr, lport, 0x200);
                        if so.is_null() {
                            return 1;
                        }
                        n6 = ntohs((*so).so_fport) as c_uint;

                        n5 = (n6 >> 8) & 0xff;
                        n6 &= 0xff;

                        laddr = ntohl((*so).so_faddr.s_addr);

                        n1 = (laddr >> 24) & 0xff;
                        n2 = (laddr >> 16) & 0xff;
                        n3 = (laddr >> 8) & 0xff;
                        n4 = laddr & 0xff;

                        (*m).m_hdr.mh_len =
                            (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                            bptr,
                            (*m).m_hdr.mh_size.wrapping_sub((*m).m_hdr.mh_len) as usize,
                            b"27 Entering Passive Mode (%d,%d,%d,%d,%d,%d)\r\n%s\0".as_ptr()
                                as *const c_char,
                            n1 as c_int,
                            n2 as c_int,
                            n3 as c_int,
                            n4 as c_int,
                            n5 as c_int,
                            n6 as c_int,
                            if x == 7 {
                                buff.as_mut_ptr()
                            } else {
                                empty_str.as_ptr() as *mut c_char
                            },
                        ));
                        return 1;
                    }
                }
                return 1;
            }

            0x3 => {
                (*so).so_emu = 0;
                lport = 0;
                let mut i: c_int = 0;
                while i < (*m).m_hdr.mh_len.wrapping_sub(1) {
                    let ch = *(*m).m_hdr.mh_data.add(i as usize) as c_int;
                    if ch < '0' as c_int || ch > '9' as c_int {
                        return 1;
                    }
                    lport = lport.wrapping_mul(10);
                    lport = lport.wrapping_add(ch.wrapping_sub('0' as c_int) as c_uint);
                    i += 1;
                }
                let last = *(*m)
                    .m_hdr
                    .mh_data
                    .add((*m).m_hdr.mh_len.wrapping_sub(1) as usize) as c_int;
                if last == 0 && lport != 0 {
                    so = tcp_listen(
                        (*so).slirp,
                        0,
                        0,
                        (*so).so_laddr.s_addr,
                        htons(lport as u16) as c_uint,
                        0x200,
                    );
                    if !so.is_null() {
                        (*m).m_hdr.mh_len = snprintf(
                            (*m).m_hdr.mh_data,
                            (*m).m_hdr.mh_size as usize,
                            b"%d\0".as_ptr() as *const c_char,
                            ntohs((*so).so_fport) as c_int,
                        )
                        .wrapping_add(1);
                    }
                }
                return 1;
            }

            0x4 => {
                *(*m).m_hdr.mh_data.add((*m).m_hdr.mh_len as usize) = 0;
                let bptr = strstr((*m).m_hdr.mh_data, b"DCC\0".as_ptr() as *const c_char);
                if bptr.is_null() {
                    return 1;
                }

                if sscanf(
                    bptr,
                    b"DCC CHAT %256s %u %u\0".as_ptr() as *const c_char,
                    buff.as_mut_ptr(),
                    &mut laddr as *mut u32,
                    &mut lport as *mut c_uint,
                ) == 3
                {
                    so = tcp_listen(
                        (*so).slirp,
                        0,
                        0,
                        htonl(laddr),
                        htons(lport as u16) as c_uint,
                        0x200,
                    );
                    if so.is_null() {
                        return 1;
                    }
                    (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                        bptr,
                        (*m).m_hdr.mh_size as usize,
                        b"DCC CHAT chat %lu %u%c\n\0".as_ptr() as *const c_char,
                        ntohl((*so).so_faddr.s_addr) as c_ulong,
                        ntohs((*so).so_fport) as c_uint,
                        1 as c_int,
                    ));
                } else if sscanf(
                    bptr,
                    b"DCC SEND %256s %u %u %u\0".as_ptr() as *const c_char,
                    buff.as_mut_ptr(),
                    &mut laddr as *mut u32,
                    &mut lport as *mut c_uint,
                    &mut n1 as *mut c_uint,
                ) == 4
                {
                    so = tcp_listen(
                        (*so).slirp,
                        0,
                        0,
                        htonl(laddr),
                        htons(lport as u16) as c_uint,
                        0x200,
                    );
                    if so.is_null() {
                        return 1;
                    }
                    (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                        bptr,
                        (*m).m_hdr.mh_size as usize,
                        b"DCC SEND %s %lu %u %u%c\n\0".as_ptr() as *const c_char,
                        buff.as_mut_ptr(),
                        ntohl((*so).so_faddr.s_addr) as c_ulong,
                        ntohs((*so).so_fport) as c_uint,
                        n1 as c_uint,
                        1 as c_int,
                    ));
                } else if sscanf(
                    bptr,
                    b"DCC MOVE %256s %u %u %u\0".as_ptr() as *const c_char,
                    buff.as_mut_ptr(),
                    &mut laddr as *mut u32,
                    &mut lport as *mut c_uint,
                    &mut n1 as *mut c_uint,
                ) == 4
                {
                    so = tcp_listen(
                        (*so).slirp,
                        0,
                        0,
                        htonl(laddr),
                        htons(lport as u16) as c_uint,
                        0x200,
                    );
                    if so.is_null() {
                        return 1;
                    }
                    (*m).m_hdr.mh_len = (bptr as isize - (*m).m_hdr.mh_data as isize) as c_int;
                    (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(snprintf(
                        bptr,
                        (*m).m_hdr.mh_size as usize,
                        b"DCC MOVE %s %lu %u %u%c\n\0".as_ptr() as *const c_char,
                        buff.as_mut_ptr(),
                        ntohl((*so).so_faddr.s_addr) as c_ulong,
                        ntohs((*so).so_fport) as c_uint,
                        n1 as c_uint,
                        1 as c_int,
                    ));
                }
                return 1;
            }

            0x5 => {
                let data = (*m).m_hdr.mh_data;
                let end = data.add((*m).m_hdr.mh_len as usize);
                let mut bptr = data;
                while (bptr as usize) < (end as usize) {
                    let mut p: u16 = 0;
                    let ra: c_int = RA;
                    match ra {
                        0 | 2 | 3 => {
                            let cur = *bptr;
                            bptr = bptr.add(1);
                            if cur != ra_tbl_at(ra) {
                                RA = 0;
                                continue;
                            }
                        }
                        1 => {
                            if *bptr as c_int == 0x50 {
                                RA = 1;
                                bptr = bptr.add(1);
                                continue;
                            } else {
                                let cur = *bptr;
                                bptr = bptr.add(1);
                                if cur != ra_tbl_at(ra) {
                                    RA = 0;
                                    continue;
                                }
                            }
                        }
                        4 => {
                            bptr = bptr.add(1);
                        }
                        5 => {
                            if *bptr.add(1) as c_int == 0x02 {
                                bptr = bptr.add(8);
                            } else {
                                bptr = bptr.add(4);
                            }
                        }
                        6 => {
                            lport = ((*bptr as u8 as c_uint) << 8)
                                .wrapping_add(*(bptr.add(1)) as u8 as c_uint);
                            if lport < 6970 {
                                lport = lport.wrapping_add(256);
                            }
                            if lport < 6970 || lport > 7170 {
                                return 1;
                            }

                            p = 6970;
                            loop {
                                if (p as c_int) >= 7071 {
                                    break;
                                }
                                let res = udp_listen(
                                    (*so).slirp,
                                    0,
                                    htons(p) as c_uint,
                                    (*so).so_laddr.s_addr,
                                    htons(lport as u16) as c_uint,
                                    0x200,
                                );
                                if !res.is_null() {
                                    break;
                                }
                                p = p.wrapping_add(1);
                            }
                            if p == 7071 {
                                p = 0;
                            }
                            *(bptr as *mut u8) = ((p >> 8) & 0xff) as u8;
                            bptr = bptr.add(1);
                            *(bptr as *mut u8) = (p & 0xff) as u8;
                            RA = 0;
                            return 1;
                        }
                        _ => {
                            RA = 0;
                        }
                    }
                    RA = RA.wrapping_add(1);
                }
                return 1;
            }

            _ => {
                (*so).so_emu = 0;
                return 1;
            }
        }
    }
}
}

// ── `tcp_ctl` · slirp/tcp_subr.c:884-915 · chunk 2847 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int tcp_ctl(struct socket *so)
//     {
//         Slirp *slirp = so->slirp;
//         struct sbuf *sb = &so->so_snd;
//         struct ex_list *ex_ptr;
//         int do_pty;
//     
//         ;
//         ;
//     
//         if (so->so_faddr.s_addr != slirp->vhost_addr.s_addr) {
//     
//             for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next) {
//                 if (ex_ptr->ex_fport == so->so_fport &&
//                     so->so_faddr.s_addr == ex_ptr->ex_addr.s_addr) {
//                     if (ex_ptr->ex_pty == 3) {
//                         so->s = -1;
//                         so->extra = (void *)ex_ptr->ex_exec;
//                         return 1;
//                     }
//                     do_pty = ex_ptr->ex_pty;
//                     ;
//                     return fork_exec(so, ex_ptr->ex_exec, do_pty);
//                 }
//             }
//         }
//         sb->sb_cc =
//             snprintf(sb->sb_wptr, sb->sb_datalen - (sb->sb_wptr - sb->sb_data),
//                      "Error: No application configured.\r\n");
//         sb->sb_wptr += sb->sb_cc;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2847 {
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
pub struct SbUf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
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
    pub so_rcv: SbUf,
    pub so_snd: SbUf,
    pub extra: *mut c_void,
}

// Only the prefix of the C `struct Slirp` that is accessible here is mirrored;
// the fields used below (vhost_addr, exec_list) keep their C offsets.
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
}

extern "C" {
    fn fork_exec(so: *mut Socket, ex: *const c_char, do_pty: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
}

static NO_APP_FMT: &[u8] = b"Error: No application configured.\r\n\0";

#[no_mangle]
pub extern "C" fn tcp_ctl(so: *mut Socket) -> c_int {
    if so.is_null() {
        return 0;
    }
    let slirp = unsafe { (*so).slirp };

    if !slirp.is_null() && unsafe { (*so).so_faddr.s_addr != (*slirp).vhost_addr.s_addr } {
        let mut ex_ptr: *mut ExList = unsafe { (*slirp).exec_list };
        while !ex_ptr.is_null() {
            let ex_pty = unsafe { (*ex_ptr).ex_pty };
            let ex_exec = unsafe { (*ex_ptr).ex_exec };
            let ex_next = unsafe { (*ex_ptr).ex_next };
            if unsafe { (*ex_ptr).ex_fport == (*so).so_fport as c_int }
                && unsafe { (*so).so_faddr.s_addr == (*ex_ptr).ex_addr.s_addr }
            {
                if ex_pty == 3 {
                    unsafe {
                        (*so).s = -1;
                        (*so).extra = ex_exec as *mut c_void;
                    }
                    return 1;
                }
                let do_pty = ex_pty;
                return unsafe { fork_exec(so, ex_exec, do_pty) };
            }
            ex_ptr = ex_next;
        }
    }

    unsafe {
        let sb: *mut SbUf = core::ptr::addr_of_mut!((*so).so_snd);
        let wptr = (*sb).sb_wptr;
        let data = (*sb).sb_data;
        let used: c_long = wptr as isize as c_long - data as isize as c_long;
        let avail: c_long = (*sb).sb_datalen as c_long - used;
        let ret = snprintf(wptr, avail as usize, NO_APP_FMT.as_ptr() as *const c_char);
        (*sb).sb_cc = ret as c_uint;
        (*sb).sb_wptr = wptr.add((*sb).sb_cc as usize);
    }

    0
}
}

// ── `Slirp *slirp = so->slirp; struct sbuf *sb = &so->so_snd; struct ex_list *ex_ptr; int do_pty; ; ;` · slirp/tcp_subr.c:886-892 · chunk 2848 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = so->slirp;
//         struct sbuf *sb = &so->so_snd;
//         struct ex_list *ex_ptr;
//         int do_pty;
//     
//         ;
//         ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2848 {
// Port of tcp_ctl's declaration/initialization prologue (slirp/tcp_subr.c 886-892).
// DEBUG_CALL/DEBUG_ARG expand to nothing in this build, so only the loads remain.

use core::ffi::{c_char, c_int, c_uint, c_uchar};

pub struct Slirp;

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

pub struct Mbuf;
pub struct Tcpiphdr;
pub struct Tcpcb;

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn tcp_ctl__part2848(
    so: *mut Socket,
    slirp_out: *mut *mut Slirp,
    sb_out: *mut *mut Sbuf,
) {
    unsafe {
        if so.is_null() {
            return;
        }
        let s = &mut *so;
        if !slirp_out.is_null() {
            *slirp_out = s.slirp;
        }
        if !sb_out.is_null() {
            *sb_out = &mut s.so_snd as *mut Sbuf;
        }
    }
}
}

// ── `if (so->so_faddr.s_addr != slirp->vhost_addr.s_addr)` · slirp/tcp_subr.c:894-909 · chunk 2849 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_faddr.s_addr != slirp->vhost_addr.s_addr) {
//     
//             for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next) {
//                 if (ex_ptr->ex_fport == so->so_fport &&
//                     so->so_faddr.s_addr == ex_ptr->ex_addr.s_addr) {
//                     if (ex_ptr->ex_pty == 3) {
//                         so->s = -1;
//                         so->extra = (void *)ex_ptr->ex_exec;
//                         return 1;
//                     }
//                     do_pty = ex_ptr->ex_pty;
//                     ;
//                     return fork_exec(so, ex_ptr->ex_exec, do_pty);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2849 {
// Port of the exec-list lookup part of tcp_ctl().
// Returns 1 or fork_exec()'s value when the connection is handled here,
// and 0 when the caller must continue (the "no application" path).
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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
pub struct Sb0 {
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
    pub so_rcv: Sb0,
    pub so_snd: Sb0,
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
    pub tt: [i64; 2],
    pub exec_list: *mut ExList,
}

extern "C" {
    fn fork_exec(so: *mut Socket, ex: *const c_char, do_pty: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_ctl__part2849(so: *mut Socket) -> c_int {
    unsafe {
        let slirp: *mut Slirp = (*so).slirp;
        let faddr: u32 = (*so).so_faddr.s_addr;

        if faddr != (*slirp).vhost_addr.s_addr {
            let mut ex_ptr: *mut ExList = (*slirp).exec_list;
            while !ex_ptr.is_null() {
                let ex: *mut ExList = ex_ptr;
                if (*ex).ex_fport == (*so).so_fport as c_int
                    && faddr == (*ex).ex_addr.s_addr
                {
                    if (*ex).ex_pty == 3 {
                        (*so).s = -1;
                        (*so).extra = (*ex).ex_exec as *mut c_void;
                        return 1;
                    }
                    let do_pty: c_int = (*ex).ex_pty;
                    return fork_exec(so, (*ex).ex_exec, do_pty);
                }
                ex_ptr = (*ex).ex_next;
            }
        }
        0
    }
}
}

// ── `sb->sb_cc = snprintf(sb->sb_wptr, sb->sb_datalen - (sb->sb_wptr - sb->sb_data), "Error: No appl…` · slirp/tcp_subr.c:910-914 · chunk 2850 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sb->sb_cc =
//             snprintf(sb->sb_wptr, sb->sb_datalen - (sb->sb_wptr - sb->sb_data),
//                      "Error: No application configured.\r\n");
//         sb->sb_wptr += sb->sb_cc;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2850 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Sbuf {
    sb_cc: u32,
    sb_datalen: u32,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
}

static MSG: &[u8; 36] = b"Error: No application configured.\r\n\0";

#[no_mangle]
pub extern "C" fn tcp_ctl__part2850(sb: *mut Sbuf) -> c_int {
    unsafe {
        if sb.is_null() {
            return 0;
        }
        let s = &mut *sb;
        let wptr = s.sb_wptr;
        let data = s.sb_data;
        // sb_datalen - (sb_wptr - sb_data): computed in signed long (LP64) as C does,
        // then converted to size_t.
        let avail: i64 = (s.sb_datalen as i64) - ((wptr as i64) - (data as i64));
        let n = snprintf(wptr, avail as usize, MSG.as_ptr() as *const c_char);
        s.sb_cc = n as u32;
        s.sb_wptr = wptr.wrapping_offset(s.sb_cc as isize);
    }
    0
}
}

