//! From `slirp/tcp_subr.c`.

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

