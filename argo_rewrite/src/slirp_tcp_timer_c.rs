//! From `slirp/tcp_timer.c`.

// ── `tcp_fasttimo` · slirp/tcp_timer.c:40-57 · chunk 2852 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_fasttimo(Slirp *slirp)
//     {
//      register struct socket *so;
//      register struct tcpcb *tp;
//     
//      ;
//     
//      so = slirp->tcb.so_next;
//      if (so)
//      for (; so != &slirp->tcb; so = so->so_next)
//       if ((tp = (struct tcpcb *)so->so_tcpcb) &&
//           (tp->t_flags & 0x0002)) {
//        tp->t_flags &= ~0x0002;
//        tp->t_flags |= 0x0001;
//        (void) tcp_output(tp);
//       }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2852 {
// Rust port of slirp/tcp_timer.c : tcp_fasttimo()
//
// Fast timeout routine for processing delayed acks.
// Walks slirp->tcb's socket list and, for every tcb with TF_DELACK set,
// clears TF_DELACK, sets TF_ACKNOW and calls tcp_output().

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long};

pub const TF_ACKNOW: u16 = 0x0001;
pub const TF_DELACK: u16 = 0x0002;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// struct mbuf { struct m_hdr m_hdr; Slirp *slirp; union M_dat { ... } M_dat; }
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
    pub m_dat: Mdat,
}

// struct sbuf { u_int sb_cc; u_int sb_datalen; char *sb_wptr; char *sb_rptr;
//               char *sb_data; }
#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipvovly,
    pub ti_t: Tcphdr,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C, packed)]
pub struct Ipvovly {
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
    // u_int th_x2:4, th_off:4;
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
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
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub extra: *mut core::ffi::c_void, // ibufptr
    pub so_ti: *mut Tcpiphdr,
    pub ibuf_hiwat: c_int,
    pub ibuf_lowat: c_int,
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
    pub extra2: *mut core::ffi::c_void,
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

// typedef struct { uint16_t allocated; uint8_t macaddr[6]; } BOOTPClient;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
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

extern "C" {
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_fasttimo(slirp: *mut Slirp) {
    let head: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);
    let mut so: *mut Socket = (*slirp).tcb.so_next;

    if !so.is_null() {
        while so != head {
            let tp: *mut Tcpcb = (*so).so_tcpcb;
            if !tp.is_null() && ((*tp).t_flags & TF_DELACK) != 0 {
                (*tp).t_flags &= !TF_DELACK;
                (*tp).t_flags |= TF_ACKNOW;
                let _ = tcp_output(tp);
            }
            so = (*so).so_next;
        }
    }
}
}

// ── `register struct socket *so; register struct tcpcb *tp; ; so = slirp->tcb.so_next;` · slirp/tcp_timer.c:43-48 · chunk 2853 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct socket *so;
//      register struct tcpcb *tp;
//     
//      ;
//     
//      so = slirp->tcb.so_next;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2853 {
// Portion of tcp_fasttimo(): fetch the first socket on the TCP control block list.
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
pub extern "C" fn tcp_fasttimo__part2853(tcb: *mut Socket) -> *mut Socket {
    unsafe {
        if tcb.is_null() {
            return core::ptr::null_mut();
        }
        (*tcb).so_next
    }
}
}

// ── `if (so)` · slirp/tcp_timer.c:49-56 · chunk 2854 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (so)
//      for (; so != &slirp->tcb; so = so->so_next)
//       if ((tp = (struct tcpcb *)so->so_tcpcb) &&
//           (tp->t_flags & 0x0002)) {
//        tp->t_flags &= ~0x0002;
//        tp->t_flags |= 0x0001;
//        (void) tcp_output(tp);
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2854 {
// Port of tcp_fasttimo's socket-list walk (slirp/tcp_timer.c lines 49-56):
//
//   if (so)
//    for (; so != &slirp->tcb; so = so->so_next)
//     if ((tp = (struct tcpcb *)so->so_tcpcb) && (tp->t_flags & TF_DELACK)) {
//      tp->t_flags &= ~TF_DELACK;
//      tp->t_flags |= TF_ACKNOW;
//      (void) tcp_output(tp);
//     }
//
// `so` is the first socket (slirp->tcb.so_next) and `tcb` is the list head
// (&slirp->tcb).  Only so_next, so_tcpcb, t_flags and tcp_output() are used.

use core::ffi::{c_char, c_int, c_short, c_uint, c_ushort, c_void};

// struct in_addr { in_addr_t s_addr; } with in_addr_t == uint32_t
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct tcpiphdr is { struct ipovly ti_i; struct tcphdr ti_t; }: 20 + 20
// bytes, 4-byte aligned.  No field of it is touched here, so it is mirrored as
// a block of identical size and alignment.
#[repr(C, align(4))]
pub struct Tcpiphdr {
    _opaque: [u8; 40],
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

// struct socket (fields after so_state are never touched here, but the layout
// is mirrored exactly so offsets of the touched fields are correct).
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

extern "C" {
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

const TF_ACKNOW: c_ushort = 0x0001;
const TF_DELACK: c_ushort = 0x0002;

#[no_mangle]
pub extern "C" fn tcp_fasttimo__part2854(so: *mut Socket, tcb: *mut Socket) {
    // if (so)
    if so.is_null() {
        return;
    }
    unsafe {
        let mut so: *mut Socket = so;
        // for (; so != &slirp->tcb; so = so->so_next)
        while so != tcb {
            let tp: *mut Tcpcb = (*so).so_tcpcb;
            if !tp.is_null() && ((*tp).t_flags & TF_DELACK) != 0 {
                (*tp).t_flags &= !TF_DELACK;
                (*tp).t_flags |= TF_ACKNOW;
                let _ = tcp_output(tp);
            }
            so = (*so).so_next;
        }
    }
}
}

// ── `tcp_slowtimo` · slirp/tcp_timer.c:64-101 · chunk 2855 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_slowtimo(Slirp *slirp)
//     {
//      register struct socket *ip, *ipnxt;
//      register struct tcpcb *tp;
//      register int i;
//     
//      ;
//     
//     
//     
//     
//      ip = slirp->tcb.so_next;
//             if (ip == 
//                      ((void *)0)
//                          ) {
//                 return;
//             }
//      for (; ip != &slirp->tcb; ip = ipnxt) {
//       ipnxt = ip->so_next;
//       tp = ((ip)->so_tcpcb);
//                     if (tp == 
//                              ((void *)0)
//                                  ) {
//                             continue;
//                     }
//       for (i = 0; i < 4; i++) {
//        if (tp->t_timer[i] && --tp->t_timer[i] == 0) {
//         tcp_timers(tp,i);
//         if (ipnxt->so_prev != ip)
//          goto tpgone;
//        }
//       }
//       tp->t_idle++;
//       if (tp->t_rtt)
//          tp->t_rtt++;
//     tpgone:
//       ;
//      }
//      slirp->tcp_iss += (125*1024)/2;
//      slirp->tcp_now++;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2855 {
// Port of slirp/tcp_timer.c : tcp_slowtimo()
//
// The file-static helper tcp_timers() has no linker symbol, so its logic is
// ported here as a private Rust function.  All struct layouts mirror the C
// definitions (see slirp/socket.h, slirp/tcp_var.h, slirp/tcp.h, slirp/mbuf.h,
// slirp/sbuf.h, slirp/ip.h, slirp/libslirp.h) so that fields accessed through
// pointers land at the same offsets as in the C program.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

/* ------------------------------------------------------------------ */
/* types                                                              */
/* ------------------------------------------------------------------ */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
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
    /* u_int th_x2:4, th_off:4 */
    pub th_x2_off: c_uint,
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
    /* send sequence variables */
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub snd_up: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub iss: u32,
    pub snd_wnd: u32,
    /* receive sequence variables */
    pub rcv_wnd: u32,
    pub rcv_nxt: u32,
    pub rcv_up: u32,
    pub irs: u32,
    /* additional variables */
    pub rcv_adv: u32,
    pub snd_max: u32,
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    /* transmit timing */
    pub t_idle: i16,
    pub t_rtt: i16,
    pub t_rtseq: u32,
    pub t_srtt: i16,
    pub t_rttvar: i16,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    /* out-of-band data */
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: i16,
    /* RFC 1323 variables */
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
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
    pub m_dat: M_dat,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
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

/* ------------------------------------------------------------------ */
/* external C functions                                              */
/* ------------------------------------------------------------------ */

extern "C" {
    fn tcp_output(tp: *mut tcpcb) -> c_int;
    fn tcp_setpersist(tp: *mut tcpcb);
    fn tcp_close(tp: *mut tcpcb) -> *mut tcpcb;
    fn tcp_drop(tp: *mut tcpcb, err: c_int) -> *mut tcpcb;
}

/* const int tcp_backoff[TCP_MAXRXTSHIFT + 1] (values are compile-time constant) */
const TCP_BACKOFF: [c_int; 13] = [1, 2, 4, 8, 16, 32, 64, 64, 64, 64, 64, 64, 64];

const TCPT_NTIMERS: usize = 4;
const TCP_MAXRXTSHIFT: c_int = 12;
const TCPTV_REXMTMAX: c_int = 12 * 2;
const TCP_MAXIDLE: c_int = 8 * (75 * 2);
const TCPS_TIME_WAIT: i16 = 10;
const TCPS_ESTABLISHED: i16 = 4;
const TCPTV_KEEP_IDLE: i16 = 120 * 60 * 2;
const TCPTV_KEEPINTVL: i16 = 75 * 2;

/*
 * TCP timer processing.  Private port of the file-static C helper:
 *   static struct tcpcb *tcp_timers(register struct tcpcb *tp, int timer)
 */
unsafe fn tcp_timers(tp_in: *mut tcpcb, timer: c_int) -> *mut tcpcb {
    let mut tp = tp_in;

    match timer {
        /* case TCPT_2MSL */
        3 => {
            if (*tp).t_state != TCPS_TIME_WAIT && (*tp).t_idle as c_int <= TCP_MAXIDLE {
                (*tp).t_timer[3] = TCPTV_KEEPINTVL;
            } else {
                tp = tcp_close(tp);
            }
        }
        /* case TCPT_REXMT */
        0 => {
            let shift = (*tp).t_rxtshift.wrapping_add(1);
            (*tp).t_rxtshift = shift;
            if shift as c_int > TCP_MAXRXTSHIFT {
                (*tp).t_maxseg >>= 1;
                if (*tp).t_maxseg < 32 {
                    (*tp).t_rxtshift = TCP_MAXRXTSHIFT as i16;
                    tp = tcp_drop(tp, (*tp).t_softerror as c_int);
                    return tp;
                }
                (*tp).t_rxtshift = 6;
            }
            let shift = (*tp).t_rxtshift as c_int;
            let backoff = if (0..TCP_BACKOFF.len() as c_int).contains(&shift) {
                TCP_BACKOFF[shift as usize]
            } else {
                /* out of range would be UB in C; keep a harmless value */
                TCP_BACKOFF[0]
            };
            let rexmt: c_int =
                (((*tp).t_srtt >> 3) as c_int + (*tp).t_rttvar as c_int).wrapping_mul(backoff);
            /* TCPT_RANGESET(tp->t_rxtcur, rexmt, (short)tp->t_rttmin, TCPTV_REXMTMAX) */
            (*tp).t_rxtcur = rexmt as i16;
            let tvmin = (*tp).t_rttmin as i16;
            if (*tp).t_rxtcur < tvmin {
                (*tp).t_rxtcur = tvmin;
            } else if (*tp).t_rxtcur as c_int > TCPTV_REXMTMAX {
                (*tp).t_rxtcur = TCPTV_REXMTMAX as i16;
            }
            (*tp).t_timer[0] = (*tp).t_rxtcur;
            if (*tp).t_rxtshift as c_int > TCP_MAXRXTSHIFT / 4 {
                (*tp).t_rttvar =
                    ((*tp).t_rttvar as c_int + ((*tp).t_srtt >> 3) as c_int) as i16;
                (*tp).t_srtt = 0;
            }
            (*tp).snd_nxt = (*tp).snd_una;
            (*tp).t_rtt = 0;
            {
                let mss = (*tp).t_maxseg as c_uint;
                let mut win: c_uint = if mss == 0 {
                    0
                } else {
                    let w = if (*tp).snd_wnd < (*tp).snd_cwnd {
                        (*tp).snd_wnd
                    } else {
                        (*tp).snd_cwnd
                    };
                    (w / 2) / mss
                };
                if win < 2 {
                    win = 2;
                }
                (*tp).snd_cwnd = (*tp).t_maxseg as c_uint;
                (*tp).snd_ssthresh = win.wrapping_mul((*tp).t_maxseg as c_uint);
                (*tp).t_dupacks = 0;
            }
            tcp_output(tp);
        }
        /* case TCPT_PERSIST */
        1 => {
            tcp_setpersist(tp);
            (*tp).t_force = 1;
            tcp_output(tp);
            (*tp).t_force = 0;
        }
        /* case TCPT_KEEP */
        2 => {
            if (*tp).t_state < TCPS_ESTABLISHED {
                tp = tcp_drop(tp, 0);
            } else {
                /* The SO_OPTIONS probe branch is compiled out in this build. */
                (*tp).t_timer[2] = TCPTV_KEEP_IDLE;
            }
        }
        _ => {}
    }

    tp
}

/*
 * Tcp protocol timeout routine called every 500 ms.
 * Updates the timers in all active tcb's and
 * causes finite state machine actions if timers expire.
 */
#[no_mangle]
pub extern "C" fn tcp_slowtimo(slirp: *mut Slirp) {
    unsafe {
        let mut ip: *mut socket = (*slirp).tcb.so_next;
        if ip.is_null() {
            return;
        }
        let head: *mut socket = core::ptr::addr_of_mut!((*slirp).tcb);

        /* Search through tcb's and update active timers. */
        while ip != head {
            let ipnxt: *mut socket = (*ip).so_next;
            let tp: *mut tcpcb = (*ip).so_tcpcb;

            if tp.is_null() {
                ip = ipnxt;
                continue;
            }

            let mut tpgone = false;
            let mut i: usize = 0;
            while i < TCPT_NTIMERS {
                if (*tp).t_timer[i] != 0 {
                    let left = (*tp).t_timer[i].wrapping_sub(1);
                    (*tp).t_timer[i] = left;
                    if left == 0 {
                        tcp_timers(tp, i as c_int);
                        if (*ipnxt).so_prev != ip {
                            tpgone = true;
                            break;
                        }
                    }
                }
                i += 1;
            }

            if !tpgone {
                (*tp).t_idle = (*tp).t_idle.wrapping_add(1);
                if (*tp).t_rtt != 0 {
                    (*tp).t_rtt = (*tp).t_rtt.wrapping_add(1);
                }
            }

            ip = ipnxt;
        }

        /* increment iss */
        (*slirp).tcp_iss = (*slirp).tcp_iss.wrapping_add((125 * 1024) / 2);
        /* for timestamps */
        (*slirp).tcp_now = (*slirp).tcp_now.wrapping_add(1);
    }
}
}

// ── `register struct socket *ip, *ipnxt; register struct tcpcb *tp; register int i; ; ip = slirp->tc…` · slirp/tcp_timer.c:67-76 · chunk 2856 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct socket *ip, *ipnxt;
//      register struct tcpcb *tp;
//      register int i;
//     
//      ;
//     
//     
//     
//     
//      ip = slirp->tcb.so_next;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2856 {
// Port of the head of slirp's tcp_slowtimo(): fetch slirp->tcb.so_next.
// The C caller passes the address of the embedded `struct socket tcb`
// instead of the whole Slirp, so only the leading fields of
// `struct socket` need to be mirrored here.

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
}

#[no_mangle]
pub extern "C" fn tcp_slowtimo__part2856(tcb: *mut Socket) -> *mut Socket {
    unsafe {
        if tcb.is_null() {
            return core::ptr::null_mut();
        }
        (*tcb).so_next
    }
}
}

// ── `if (ip == ((void *)0)` · slirp/tcp_timer.c:77-79 · chunk 2857 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip == 
//                      ((void *)0)
//                          ) {
//                 return;
//             }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2857 {
// Port of the null check of the tcb list head pointer in tcp_slowtimo.
// Returns 1 when the pointer is NULL (the caller must then return), else 0.
#[no_mangle]
pub extern "C" fn tcp_slowtimo__part2857(ip: *mut core::ffi::c_void) -> core::ffi::c_int {
    if ip.is_null() {
        1
    } else {
        0
    }
}
}

// ── `for (; ip != &slirp->tcb; ip = ipnxt)` · slirp/tcp_timer.c:80-98 · chunk 2858 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (; ip != &slirp->tcb; ip = ipnxt) {
//       ipnxt = ip->so_next;
//       tp = ((ip)->so_tcpcb);
//                     if (tp == 
//                              ((void *)0)
//                                  ) {
//                             continue;
//                     }
//       for (i = 0; i < 4; i++) {
//        if (tp->t_timer[i] && --tp->t_timer[i] == 0) {
//         tcp_timers(tp,i);
//         if (ipnxt->so_prev != ip)
//          goto tpgone;
//        }
//       }
//       tp->t_idle++;
//       if (tp->t_rtt)
//          tp->t_rtt++;
//     tpgone:
//       ;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2858 {
// Rust port of the timer-scan loop of tcp_slowtimo() (slirp/tcp_timer.c).
// The loop walks the socket list hanging off slirp->tcb, calls the (static, C-only)
// tcp_timers() through a function pointer, and ages t_idle / t_rtt.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_short, c_uint, c_ushort};

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

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut SocketlessMbuf,
}

// Forward handle for `struct mbuf` (never dereferenced here).
#[repr(C)]
pub struct SocketlessMbuf {
    pub opaque: [u8; 0],
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

#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4;
    pub th_x2_off: c_uint,
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

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut SlirpOpaque,
    pub so_m: *mut SocketlessMbuf,
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
    pub extra: *mut core::ffi::c_void,
}

// Forward handle for `struct Slirp` (never dereferenced here).
#[repr(C)]
pub struct SlirpOpaque {
    pub opaque: [u8; 0],
}

type TcpTimersFn = unsafe extern "C" fn(*mut Tcpcb, c_int) -> *mut Tcpcb;

#[no_mangle]
pub extern "C" fn tcp_slowtimo__part2858(
    head: *mut Socket,
    timers_fn: Option<unsafe extern "C" fn(*mut Tcpcb, c_int) -> *mut Tcpcb>,
) {
    if head.is_null() {
        return;
    }
    unsafe {
        let mut ip: *mut Socket = (*head).so_next;
        while ip != head {
            if ip.is_null() {
                break;
            }
            let ipnxt: *mut Socket = (*ip).so_next;
            let tp: *mut Tcpcb = (*ip).so_tcpcb;
            if tp.is_null() {
                ip = ipnxt;
                continue;
            }
            let mut i: c_int = 0;
            let mut tpgone = false;
            while i < 4 {
                let idx = i as usize;
                let timer = (*tp).t_timer[idx];
                if timer != 0 {
                    let nv = timer.wrapping_sub(1);
                    (*tp).t_timer[idx] = nv;
                    if nv == 0 {
                        if let Some(f) = timers_fn {
                            let f: TcpTimersFn = f;
                            let _ = f(tp, i);
                        }
                        if (*ipnxt).so_prev != ip {
                            tpgone = true;
                            break;
                        }
                    }
                }
                i += 1;
            }
            if !tpgone {
                (*tp).t_idle = (*tp).t_idle.wrapping_add(1);
                if (*tp).t_rtt != 0 {
                    (*tp).t_rtt = (*tp).t_rtt.wrapping_add(1);
                }
            }
            ip = ipnxt;
        }
    }
}
}

// ── `slirp->tcp_iss += (125*1024)/2; slirp->tcp_now++;` · slirp/tcp_timer.c:99-100 · chunk 2859 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->tcp_iss += (125*1024)/2;
//      slirp->tcp_now++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2859 {
// Ported piece of tcp_slowtimo: advance the ISS and the slow-timer tick.
// tcp_iss is tcp_seq (uint32_t), tcp_now is uint32_t; both wrap modulo 2^32.

#[no_mangle]
pub extern "C" fn tcp_slowtimo__part2859(tcp_iss: *mut u32, tcp_now: *mut u32) {
    unsafe {
        if !tcp_iss.is_null() {
            *tcp_iss = (*tcp_iss).wrapping_add((125u32 * 1024u32) / 2u32);
        }
        if !tcp_now.is_null() {
            *tcp_now = (*tcp_now).wrapping_add(1u32);
        }
    }
}
}

// ── `tcp_canceltimers` · slirp/tcp_timer.c:106-113 · chunk 2860 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_canceltimers(struct tcpcb *tp)
//     {
//      register int i;
//     
//      for (i = 0; i < 4; i++)
//       tp->t_timer[i] = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2860 {
// Rewrite of slirp/tcp_timer.c:tcp_canceltimers
// void tcp_canceltimers(struct tcpcb *tp) { int i; for (i = 0; i < 4; i++) tp->t_timer[i] = 0; }
#![allow(non_camel_case_types)]

use core::ffi::{c_short, c_void};

// Prefix of the C `struct tcpcb` (only the fields reached here are mirrored).
#[repr(C)]
pub struct tcpcb {
    seg_next: *mut c_void,
    seg_prev: *mut c_void,
    t_state: c_short,
    t_timer: [c_short; 4],
}

#[no_mangle]
pub extern "C" fn tcp_canceltimers(tp: *mut tcpcb) {
    unsafe {
        if tp.is_null() {
            return;
        }
        let mut i: i32 = 0;
        while i < 4 {
            (*tp).t_timer[i as usize] = 0;
            i += 1;
        }
    }
}
}

// ── `for (i = 0; i < 4; i++)` · slirp/tcp_timer.c:111-112 · chunk 2861 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < 4; i++)
//       tp->t_timer[i] = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2861 {
// Port of the timer-clearing loop from tcp_canceltimers().
// Clears the four tcp timer slots of a tcpcb.

#[no_mangle]
pub extern "C" fn tcp_canceltimers__part2861(t_timer: *mut core::ffi::c_short) {
    let mut i: core::ffi::c_int = 0;
    while i < 4 {
        if !t_timer.is_null() {
            unsafe {
                *t_timer.add(i as usize) = 0;
            }
        }
        i += 1;
    }
}
}

// ── `tcp_timers` · slirp/tcp_timer.c:121-292 · chunk 2863 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static struct tcpcb *
//     tcp_timers(register struct tcpcb *tp, int timer)
//     {
//      register int rexmt;
//     
//      ;
//     
//      switch (timer) {
//     
//     
//     
//     
//     
//     
//     
//      case 3:
//       if (tp->t_state != 10 &&
//           tp->t_idle <= (8 * ( 75*2)))
//        tp->t_timer[3] = ( 75*2);
//       else
//        tp = tcp_close(tp);
//       break;
//     
//     
//     
//     
//     
//     
//      case 0:
//     
//     
//     
//     
//     
//     
//       if (++tp->t_rxtshift > 12) {
//        tp->t_maxseg >>= 1;
//        if (tp->t_maxseg < 32) {
//     
//     
//     
//         tp->t_rxtshift = 12;
//         tp = tcp_drop(tp, tp->t_softerror);
//     
//         return (tp);
//        }
//     
//     
//     
//     
//     
//        tp->t_rxtshift = 6;
//       }
//       rexmt = (((tp)->t_srtt >> 3) + (tp)->t_rttvar) * tcp_backoff[tp->t_rxtshift];
//       { (tp->t_rxtcur) = (rexmt); if ((tp->t_rxtcur) < ((short)tp->t_rttmin)) (tp->t_rxtcur) = ((short)tp->t_rttmin); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
//                                               ;
//       tp->t_timer[0] = tp->t_rxtcur;
//       if (tp->t_rxtshift > 12 / 4) {
//        tp->t_rttvar += (tp->t_srtt >> 3);
//        tp->t_srtt = 0;
//       }
//       tp->snd_nxt = tp->snd_una;
//     
//     
//     
//       tp->t_rtt = 0;
//       {
//       u_int win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->snd_cwnd)) / 2 / tp->t_maxseg;
//       if (win < 2)
//        win = 2;
//       tp->snd_cwnd = tp->t_maxseg;
//       tp->snd_ssthresh = win * tp->t_maxseg;
//       tp->t_dupacks = 0;
//       }
//       (void) tcp_output(tp);
//       break;
//     
//     
//     
//     
//     
//      case 1:
//       tcp_setpersist(tp);
//       tp->t_force = 1;
//       (void) tcp_output(tp);
//       tp->t_force = 0;
//       break;
//     
//     
//     
//     
//     
//      case 2:
//       if (tp->t_state < 4)
//        goto dropit;
//     
//       if ((0) && tp->t_state <= 5) {
//            if (tp->t_idle >= (120*60*2) + (8 * ( 75*2)))
//         goto dropit;
//        tcp_respond(tp, &tp->t_template, (struct mbuf *)
//                                                       ((void *)0)
//                                                           ,
//            tp->rcv_nxt, tp->snd_una - 1, 0);
//        tp->t_timer[2] = ( 75*2);
//       } else
//        tp->t_timer[2] = (120*60*2);
//       break;
//     
//      dropit:
//       tp = tcp_drop(tp, 0);
//       break;
//      }
//     
//      return (tp);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2863 {
// Rust port of slirp's static tcp_timers() (slirp/tcp_timer.c).
// Exported under the C symbol name so the C program links against it.

use core::ffi::{c_int, c_uint};

// ---- constants from tcp_timer.h / tcp.h ----
const TCPT_REXMT: c_int = 0;
const TCPT_PERSIST: c_int = 1;
const TCPT_KEEP: c_int = 2;
const TCPT_2MSL: c_int = 3;

const TCPS_ESTABLISHED: i16 = 4;
const TCPS_TIME_WAIT: i16 = 10;

const TCPTV_KEEPINTVL: i16 = 75 * 2; /* 150 */
const TCPTV_KEEP_IDLE: i16 = 120 * 60 * 2; /* 14400 */
const TCP_MAXIDLE: c_int = 8 * (75 * 2); /* TCPTV_KEEPCNT * TCPTV_KEEPINTVL */
const TCPTV_REXMTMAX: c_int = 12 * 2; /* 24 */
const TCP_MAXRXTSHIFT: i16 = 12;

// struct tcpiphdr: struct ipovly (packed, 20 bytes) + struct tcphdr (24 bytes),
// 4-byte aligned, 44 bytes total. Only its address/size/alignment matter here.
#[repr(C, align(4))]
pub struct Tcpiphdr {
    _opaque: [u8; 44],
}

// Mirror of `struct tcpcb` (slirp/tcp_var.h); field order and types exactly as C.
#[repr(C)]
pub struct Tcpcb {
    seg_next: *mut Tcpiphdr,
    seg_prev: *mut Tcpiphdr,
    t_state: i16,
    t_timer: [i16; 4],
    t_rxtshift: i16,
    t_rxtcur: i16,
    t_dupacks: i16,
    t_maxseg: u16,
    t_force: u8,
    t_flags: u16,
    t_template: Tcpiphdr,
    t_socket: *mut core::ffi::c_void,
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
    t_oobflags: u8,
    t_iobc: u8,
    t_softerror: i16,
    snd_scale: u8,
    rcv_scale: u8,
    request_r_scale: u8,
    requested_s_scale: u8,
    ts_recent: u32,
    ts_recent_age: u32,
    last_ack_sent: u32,
}

extern "C" {
    static tcp_backoff: [c_int; 13];
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
    fn tcp_drop(tp: *mut Tcpcb, err: c_int) -> *mut Tcpcb;
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn tcp_setpersist(tp: *mut Tcpcb);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_timers(tp: *mut Tcpcb, timer: c_int) -> *mut Tcpcb {
    let mut tp = tp;

    match timer {
        /*
         * 2 MSL timeout in shutdown went off.  If we're closed but
         * still waiting for peer to close and connection has been idle
         * too long, or if 2MSL time is up from TIME_WAIT, delete
         * connection control block.  Otherwise, check again in a bit.
         */
        TCPT_2MSL => {
            if (*tp).t_state != TCPS_TIME_WAIT && ((*tp).t_idle as c_int) <= TCP_MAXIDLE {
                (*tp).t_timer[TCPT_2MSL as usize] = TCPTV_KEEPINTVL;
            } else {
                tp = tcp_close(tp);
            }
        }

        /*
         * Retransmission timer went off: back off to a longer interval and
         * retransmit one segment.
         */
        TCPT_REXMT => {
            let shift = (*tp).t_rxtshift.wrapping_add(1);
            (*tp).t_rxtshift = shift;

            if (shift as c_int) > (TCP_MAXRXTSHIFT as c_int) {
                (*tp).t_maxseg >>= 1;
                if (*tp).t_maxseg < 32 {
                    /* We tried our best, now the connection must die! */
                    (*tp).t_rxtshift = TCP_MAXRXTSHIFT;
                    tp = tcp_drop(tp, (*tp).t_softerror as c_int);
                    return tp;
                }
                /* Set rxtshift to 6, which is still at the maximum backoff time */
                (*tp).t_rxtshift = 6;
            }

            // rexmt = TCP_REXMTVAL(tp) * tcp_backoff[tp->t_rxtshift];
            let table = core::ptr::addr_of!(tcp_backoff) as *const c_int;
            let backoff = *table.offset((*tp).t_rxtshift as isize);
            let rexmtval = (((*tp).t_srtt as c_int) >> 3) + ((*tp).t_rttvar as c_int);
            let rexmt = rexmtval.wrapping_mul(backoff);

            // TCPT_RANGESET(tp->t_rxtcur, rexmt, (short)tp->t_rttmin, TCPTV_REXMTMAX);
            let mut rxtcur = rexmt as i16;
            let tvmin = (*tp).t_rttmin as i16;
            if rxtcur < tvmin {
                rxtcur = tvmin;
            } else if (rxtcur as c_int) > TCPTV_REXMTMAX {
                rxtcur = TCPTV_REXMTMAX as i16;
            }
            (*tp).t_rxtcur = rxtcur;
            (*tp).t_timer[TCPT_REXMT as usize] = (*tp).t_rxtcur;

            /*
             * If losing, let the lower level know... clobber srtt so the next
             * rtt measurement is taken as srtt, keeping the current value in
             * rttvar.
             */
            if ((*tp).t_rxtshift as c_int) > (TCP_MAXRXTSHIFT as c_int) / 4 {
                let add = (((*tp).t_srtt as c_int) >> 3) as i16;
                (*tp).t_rttvar = (*tp).t_rttvar.wrapping_add(add);
                (*tp).t_srtt = 0;
            }
            (*tp).snd_nxt = (*tp).snd_una;
            /* If timing a segment in this window, stop the timer. */
            (*tp).t_rtt = 0;

            /*
             * Close the congestion window down to one segment.
             */
            {
                let mss = (*tp).t_maxseg as c_uint;
                let mut win: c_uint =
                    core::cmp::min((*tp).snd_wnd, (*tp).snd_cwnd) / 2;
                win = if mss == 0 { 0 } else { win / mss };
                if win < 2 {
                    win = 2;
                }
                (*tp).snd_cwnd = (*tp).t_maxseg as c_uint;
                (*tp).snd_ssthresh = win.wrapping_mul((*tp).t_maxseg as c_uint);
                (*tp).t_dupacks = 0;
            }
            let _ = tcp_output(tp);
        }

        /*
         * Persistence timer into zero window.  Force a byte to be output.
         */
        TCPT_PERSIST => {
            tcp_setpersist(tp);
            (*tp).t_force = 1;
            let _ = tcp_output(tp);
            (*tp).t_force = 0;
        }

        /*
         * Keep-alive timer went off; send something or drop connection if
         * idle for too long.
         */
        TCPT_KEEP => {
            if (*tp).t_state < TCPS_ESTABLISHED {
                /* goto dropit */
                tp = tcp_drop(tp, 0);
            } else {
                /*
                 * SO_OPTIONS is 0 here, so the keep-alive probe branch is dead
                 * code; fall through to the "else" arm.
                 */
                (*tp).t_timer[TCPT_KEEP as usize] = TCPTV_KEEP_IDLE;
            }
        }

        _ => {}
    }

    tp
}
}

// ── `register int rexmt; ;` · slirp/tcp_timer.c:124-126 · chunk 2864 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     register int rexmt;
//     
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2864 {
// Ported piece of tcp_timers: the local `rexmt` declaration and the
// DEBUG_CALL("tcp_timers") statement, which preprocesses to a bare `;`.
// Nothing outside the function is touched, so the Rust side is a no-op.
#[no_mangle]
pub extern "C" fn tcp_timers__part2864() {}
}

// ── `switch (timer)` · slirp/tcp_timer.c:128-289 · chunk 2865 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (timer) {
//     
//     
//     
//     
//     
//     
//     
//      case 3:
//       if (tp->t_state != 10 &&
//           tp->t_idle <= (8 * ( 75*2)))
//        tp->t_timer[3] = ( 75*2);
//       else
//        tp = tcp_close(tp);
//       break;
//     
//     
//     
//     
//     
//     
//      case 0:
//     
//     
//     
//     
//     
//     
//       if (++tp->t_rxtshift > 12) {
//        tp->t_maxseg >>= 1;
//        if (tp->t_maxseg < 32) {
//     
//     
//     
//         tp->t_rxtshift = 12;
//         tp = tcp_drop(tp, tp->t_softerror);
//     
//         return (tp);
//        }
//     
//     
//     
//     
//     
//        tp->t_rxtshift = 6;
//       }
//       rexmt = (((tp)->t_srtt >> 3) + (tp)->t_rttvar) * tcp_backoff[tp->t_rxtshift];
//       { (tp->t_rxtcur) = (rexmt); if ((tp->t_rxtcur) < ((short)tp->t_rttmin)) (tp->t_rxtcur) = ((short)tp->t_rttmin); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
//                                               ;
//       tp->t_timer[0] = tp->t_rxtcur;
//       if (tp->t_rxtshift > 12 / 4) {
//        tp->t_rttvar += (tp->t_srtt >> 3);
//        tp->t_srtt = 0;
//       }
//       tp->snd_nxt = tp->snd_una;
//     
//     
//     
//       tp->t_rtt = 0;
//       {
//       u_int win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->snd_cwnd)) / 2 / tp->t_maxseg;
//       if (win < 2)
//        win = 2;
//       tp->snd_cwnd = tp->t_maxseg;
//       tp->snd_ssthresh = win * tp->t_maxseg;
//       tp->t_dupacks = 0;
//       }
//       (void) tcp_output(tp);
//       break;
//     
//     
//     
//     
//     
//      case 1:
//       tcp_setpersist(tp);
//       tp->t_force = 1;
//       (void) tcp_output(tp);
//       tp->t_force = 0;
//       break;
//     
//     
//     
//     
//     
//      case 2:
//       if (tp->t_state < 4)
//        goto dropit;
//     
//       if ((0) && tp->t_state <= 5) {
//            if (tp->t_idle >= (120*60*2) + (8 * ( 75*2)))
//         goto dropit;
//        tcp_respond(tp, &tp->t_template, (struct mbuf *)
//                                                       ((void *)0)
//                                                           ,
//            tp->rcv_nxt, tp->snd_una - 1, 0);
//        tp->t_timer[2] = ( 75*2);
//       } else
//        tp->t_timer[2] = (120*60*2);
//       break;
//     
//      dropit:
//       tp = tcp_drop(tp, 0);
//       break;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2865 {
// Port of the timer switch of tcp_timers() from slirp/tcp_timer.c (lines 128-289).
// The C caller does:
//     tp = tcp_timers__part2865(tp, timer);
//     return (tp);
// which is exactly equivalent to the original code: both the early "return (tp)"
// of case 0 and the fall-through return hand back the (possibly replaced) tp.

// ---- C type mirrors -------------------------------------------------------

#[repr(C)]
pub struct Mbuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    pub _opaque: [u8; 0],
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
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4;  (packed into one unsigned int)
    pub th_x2_th_off: u32,
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
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: i8,
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
    pub t_oobflags: i8,
    pub t_iobc: i8,
    pub t_softerror: i16,
    pub snd_scale: u8,
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

// ---- C symbols used -------------------------------------------------------

extern "C" {
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
    fn tcp_drop(tp: *mut Tcpcb, err: i32) -> *mut Tcpcb;
    fn tcp_setpersist(tp: *mut Tcpcb);
    fn tcp_output(tp: *mut Tcpcb) -> i32;
    fn tcp_respond(
        tp: *mut Tcpcb,
        ti: *mut Tcpiphdr,
        m: *mut Mbuf,
        ack: u32,
        seq: u32,
        len: i32,
    );
    // const int tcp_backoff[TCP_MAXRXTSHIFT + 1]
    static tcp_backoff: [i32; 13];
}

#[inline]
fn backoff(idx: usize) -> i32 {
    // idx is always <= 12 here (t_rxtshift was clamped by the code above/below)
    unsafe {
        if idx < 13 {
            *(std::ptr::addr_of!(tcp_backoff) as *const i32).add(idx)
        } else {
            64
        }
    }
}

// ---- ported switch --------------------------------------------------------

#[no_mangle]
pub extern "C" fn tcp_timers__part2865(tp_arg: *mut Tcpcb, timer: i32) -> *mut Tcpcb {
    let mut tp = tp_arg;

    unsafe {
        match timer {
            // case TCPT_KEEP:
            3 => {
                if ((*tp).t_state as i32) != 10 && ((*tp).t_idle as i32) <= (8 * (75 * 2)) {
                    (*tp).t_timer[3] = 75 * 2;
                } else {
                    tp = tcp_close(tp);
                }
            }

            // case TCPT_REXMT:
            0 => {
                // if (++tp->t_rxtshift > TCP_MAXRXTSHIFT)
                (*tp).t_rxtshift = ((*tp).t_rxtshift as i32).wrapping_add(1) as i16;
                if ((*tp).t_rxtshift as i32) > 12 {
                    (*tp).t_maxseg >>= 1;
                    if (*tp).t_maxseg < 32 {
                        (*tp).t_rxtshift = 12;
                        let err = (*tp).t_softerror as i32;
                        tp = tcp_drop(tp, err);
                        return tp;
                    }
                    (*tp).t_rxtshift = 6;
                }

                // rexmt = TCP_REXMTVAL(tp) * tcp_backoff[tp->t_rxtshift]
                let rexmt: i32 = (((*tp).t_srtt as i32) >> 3)
                    .wrapping_add((*tp).t_rttvar as i32)
                    .wrapping_mul(backoff((*tp).t_rxtshift as usize));

                // TCP_RANGESET(tp, rexmt, TCP_RXT_MIN(tp), TCP_RXT_MAX)
                (*tp).t_rxtcur = rexmt as i16;
                let minv = (*tp).t_rttmin as i16;
                if ((*tp).t_rxtcur as i32) < (minv as i32) {
                    (*tp).t_rxtcur = minv;
                } else if ((*tp).t_rxtcur as i32) > (12 * 2) {
                    (*tp).t_rxtcur = 12 * 2;
                }

                (*tp).t_timer[0] = (*tp).t_rxtcur;

                if ((*tp).t_rxtshift as i32) > 12 / 4 {
                    (*tp).t_rttvar =
                        ((*tp).t_rttvar as i32).wrapping_add(((*tp).t_srtt as i32) >> 3) as i16;
                    (*tp).t_srtt = 0;
                }
                (*tp).snd_nxt = (*tp).snd_una;

                (*tp).t_rtt = 0;

                {
                    let a = (*tp).snd_wnd;
                    let b = (*tp).snd_cwnd;
                    let mn: u32 = if a < b { a } else { b };
                    let maxseg: u32 = (*tp).t_maxseg as u32;
                    let mut win: u32 = if maxseg == 0 { 0 } else { (mn / 2) / maxseg };
                    if win < 2 {
                        win = 2;
                    }
                    (*tp).snd_cwnd = maxseg;
                    (*tp).snd_ssthresh = win.wrapping_mul(maxseg);
                    (*tp).t_dupacks = 0;
                }
                let _ = tcp_output(tp);
            }

            // case TCPT_PERSIST:
            1 => {
                tcp_setpersist(tp);
                (*tp).t_force = 1;
                let _ = tcp_output(tp);
                (*tp).t_force = 0;
            }

            // case TCPT_RANGESET:
            2 => {
                let mut dropit = false;

                if ((*tp).t_state as i32) < 4 {
                    dropit = true;
                } else if false && ((*tp).t_state as i32) <= 5 {
                    // Dead branch in the original: the condition starts with (0) &&
                    if ((*tp).t_idle as i32) >= (120 * 60 * 2) + (8 * (75 * 2)) {
                        dropit = true;
                    } else {
                        let rcv_nxt = (*tp).rcv_nxt;
                        let snd_una = (*tp).snd_una;
                        tcp_respond(
                            tp,
                            std::ptr::addr_of_mut!((*tp).t_template),
                            std::ptr::null_mut(),
                            rcv_nxt,
                            snd_una.wrapping_sub(1),
                            0,
                        );
                        (*tp).t_timer[2] = 75 * 2;
                    }
                } else {
                    (*tp).t_timer[2] = 120 * 60 * 2;
                }

                // dropit:
                if dropit {
                    tp = tcp_drop(tp, 0);
                }
            }

            _ => {}
        }
    }

    tp
}
}

// ── `return (tp);` · slirp/tcp_timer.c:291-291 · chunk 2866 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (tp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2866 {
// Port of the final `return (tp);` of tcp_timers(). The pointer is simply
// handed back to the C caller, so the struct stays opaque here.

#[repr(C)]
pub struct tcpcb {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn tcp_timers__part2866(tp: *mut tcpcb) -> *mut tcpcb {
    tp
}
}

