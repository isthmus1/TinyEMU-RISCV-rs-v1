//! From `slirp/tcp_timer.c`.

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

