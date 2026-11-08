//! From `slirp/tcp_output.c`.

// ── `tcp_output` · slirp/tcp_output.c:55-477 · chunk 2738 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int
//     tcp_output(struct tcpcb *tp)
//     {
//      register struct socket *so = tp->t_socket;
//      register long len, win;
//      int off, flags, error;
//      register struct mbuf *m;
//      register struct tcpiphdr *ti;
//      u_char opt[32];
//      unsigned optlen, hdrlen;
//      int idle, sendalot;
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//     
//     
//      idle = (tp->snd_max == tp->snd_una);
//      if (idle && tp->t_idle >= tp->t_rxtcur)
//     
//     
//     
//     
//     
//       tp->snd_cwnd = tp->t_maxseg;
//     again:
//      sendalot = 0;
//      off = tp->snd_nxt - tp->snd_una;
//      win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->snd_cwnd));
//     
//      flags = tcp_outflags[tp->t_state];
//     
//      ;
//     
//     
//     
//     
//     
//     
//     
//      if (tp->t_force) {
//       if (win == 0) {
//        if (off < so->so_snd.sb_cc)
//         flags &= ~0x01;
//        win = 1;
//       } else {
//        tp->t_timer[1] = 0;
//        tp->t_rxtshift = 0;
//       }
//      }
//     
//      len = ((so->so_snd.sb_cc) < (win) ? (so->so_snd.sb_cc) : (win)) - off;
//     
//      if (len < 0) {
//       len = 0;
//       if (win == 0) {
//        tp->t_timer[0] = 0;
//        tp->snd_nxt = tp->snd_una;
//       }
//      }
//     
//      if (len > tp->t_maxseg) {
//       len = tp->t_maxseg;
//       sendalot = 1;
//      }
//      if (((int)((tp->snd_nxt + len)-(tp->snd_una + so->so_snd.sb_cc)) < 0))
//       flags &= ~0x01;
//     
//      win = ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc);
//      if (len) {
//       if (len == tp->t_maxseg)
//        goto send;
//       if ((1 || idle || tp->t_flags & 0x0004) &&
//           len + off >= so->so_snd.sb_cc)
//        goto send;
//       if (tp->t_force)
//        goto send;
//       if (len >= tp->max_sndwnd / 2 && tp->max_sndwnd > 0)
//        goto send;
//       if (((int)((tp->snd_nxt)-(tp->snd_max)) < 0))
//        goto send;
//      }
//      if (win > 0) {
//     
//     
//     
//     
//     
//       long adv = ((win) < ((long)65535 << tp->rcv_scale) ? (win) : ((long)65535 << tp->rcv_scale)) -
//        (tp->rcv_adv - tp->rcv_nxt);
//     
//       if (adv >= (long) (2 * tp->t_maxseg))
//        goto send;
//       if (2 * adv >= (long) so->so_rcv.sb_datalen)
//        goto send;
//      }
//     
//     
//     
//     
//      if (tp->t_flags & 0x0001)
//       goto send;
//      if (flags & (0x02|0x04))
//       goto send;
//      if (((int)((tp->snd_up)-(tp->snd_una)) > 0))
//       goto send;
//     
//     
//     
//     
//     
//      if (flags & 0x01 &&
//          ((tp->t_flags & 0x0010) == 0 || tp->snd_nxt == tp->snd_una))
//       goto send;
//      if (so->so_snd.sb_cc && tp->t_timer[0] == 0 &&
//          tp->t_timer[1] == 0) {
//       tp->t_rxtshift = 0;
//       tcp_setpersist(tp);
//      }
//     
//     
//     
//     
//      return (0);
//     
//     send:
//      optlen = 0;
//      hdrlen = sizeof (struct tcpiphdr);
//      if (flags & 0x02) {
//       tp->snd_nxt = tp->iss;
//       if ((tp->t_flags & 0x0008) == 0) {
//        uint16_t mss;
//     
//        opt[0] = 2;
//        opt[1] = 4;
//        mss = htons((uint16_t) tcp_mss(tp, 0));
//        memcpy((caddr_t)(opt + 2), (caddr_t)&mss, sizeof(mss));
//        optlen = 4;
//       }
//       }
//     
//       hdrlen += optlen;
//     
//     
//     
//     
//     
//       if (len > tp->t_maxseg - optlen) {
//       len = tp->t_maxseg - optlen;
//       sendalot = 1;
//       }
//     
//     
//     
//     
//     
//     
//      if (len) {
//       m = m_get(so->slirp);
//       if (m == 
//               ((void *)0)
//                   ) {
//        error = 1;
//        goto out;
//       }
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       m->m_hdr.mh_len = hdrlen;
//     
//       sbcopy(&so->so_snd, off, (int) len, ((caddr_t)(m)->m_hdr.mh_data) + hdrlen);
//       m->m_hdr.mh_len += len;
//     
//     
//     
//     
//     
//     
//     
//       if (off + len == so->so_snd.sb_cc)
//        flags |= 0x08;
//      } else {
//       m = m_get(so->slirp);
//       if (m == 
//               ((void *)0)
//                   ) {
//        error = 1;
//        goto out;
//       }
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       m->m_hdr.mh_len = hdrlen;
//      }
//     
//      ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);
//     
//      memcpy((caddr_t)ti, &tp->t_template, sizeof (struct tcpiphdr));
//     
//     
//     
//     
//     
//     
//      if (flags & 0x01 && tp->t_flags & 0x0010 &&
//          tp->snd_nxt == tp->snd_max)
//       tp->snd_nxt--;
//      if (len || (flags & (0x02|0x01)) || tp->t_timer[1])
//       ti->ti_t.th_seq = htonl(tp->snd_nxt);
//      else
//       ti->ti_t.th_seq = htonl(tp->snd_max);
//      ti->ti_t.th_ack = htonl(tp->rcv_nxt);
//      if (optlen) {
//       memcpy((caddr_t)(ti + 1), (caddr_t)opt, optlen);
//       ti->ti_t.th_off = (sizeof (struct tcphdr) + optlen) >> 2;
//      }
//      ti->ti_t.th_flags = flags;
//     
//     
//     
//     
//      if (win < (long)(so->so_rcv.sb_datalen / 4) && win < (long)tp->t_maxseg)
//       win = 0;
//      if (win > (long)65535 << tp->rcv_scale)
//       win = (long)65535 << tp->rcv_scale;
//      if (win < (long)(tp->rcv_adv - tp->rcv_nxt))
//       win = (long)(tp->rcv_adv - tp->rcv_nxt);
//      ti->ti_t.th_win = htons((uint16_t) (win>>tp->rcv_scale));
//     
//      if (((int)((tp->snd_up)-(tp->snd_una)) > 0)) {
//       ti->ti_t.th_urp = htons((uint16_t)(tp->snd_up - ntohl(ti->ti_t.th_seq)));
//       ti->ti_t.th_flags |= 0x20;
//      } else
//     
//     
//     
//     
//     
//     
//       tp->snd_up = tp->snd_una;
//     
//     
//     
//     
//     
//      if (len + optlen)
//       ti->ti_i.ih_len = htons((uint16_t)(sizeof (struct tcphdr) +
//           optlen + len));
//      ti->ti_t.th_sum = cksum(m, (int)(hdrlen + len));
//     
//     
//     
//     
//     
//      if (tp->t_force == 0 || tp->t_timer[1] == 0) {
//       tcp_seq startseq = tp->snd_nxt;
//     
//     
//     
//     
//       if (flags & (0x02|0x01)) {
//        if (flags & 0x02)
//         tp->snd_nxt++;
//        if (flags & 0x01) {
//         tp->snd_nxt++;
//         tp->t_flags |= 0x0010;
//        }
//       }
//       tp->snd_nxt += len;
//       if (((int)((tp->snd_nxt)-(tp->snd_max)) > 0)) {
//        tp->snd_max = tp->snd_nxt;
//     
//     
//     
//     
//        if (tp->t_rtt == 0) {
//         tp->t_rtt = 1;
//         tp->t_rtseq = startseq;
//        }
//       }
//       if (tp->t_timer[0] == 0 &&
//           tp->snd_nxt != tp->snd_una) {
//        tp->t_timer[0] = tp->t_rxtcur;
//        if (tp->t_timer[1]) {
//         tp->t_timer[1] = 0;
//         tp->t_rxtshift = 0;
//        }
//       }
//      } else
//       if (((int)((tp->snd_nxt + len)-(tp->snd_max)) > 0))
//        tp->snd_max = tp->snd_nxt + len;
//     
//     
//     
//     
//     
//     
//     
//      m->m_hdr.mh_len = hdrlen + len;
//     
//         {
//     
//      ((struct ip *)ti)->ip_len = m->m_hdr.mh_len;
//     
//      ((struct ip *)ti)->ip_ttl = 64;
//      ((struct ip *)ti)->ip_tos = so->so_iptos;
//     
//      error = ip_output(so, m);
//         }
//      if (error) {
//     out:
//       return (error);
//      }
//     
//     
//     
//     
//     
//     
//     
//      if (win > 0 && ((int)((tp->rcv_nxt+win)-(tp->rcv_adv)) > 0))
//       tp->rcv_adv = tp->rcv_nxt + win;
//      tp->last_ack_sent = tp->rcv_nxt;
//      tp->t_flags &= ~(0x0001|0x0002);
//      if (sendalot)
//       goto again;
//     
//      return (0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2738 {
// Rust port of slirp's tcp_output() (slirp/tcp_output.c lines 55-477).
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_parens)]

use std::ffi::{c_char, c_int, c_uint};
use std::mem::size_of;
use std::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};

// TCP header flags
const TH_FIN: c_int = 0x01;
const TH_SYN: c_int = 0x02;
const TH_RST: c_int = 0x04;
const TH_PUSH: c_int = 0x08;
const TH_ACK: c_int = 0x10;
const TH_URG: c_int = 0x20;

// tcpcb flags
const TF_ACKNOW: u16 = 0x0001;
const TF_DELACK: u16 = 0x0002;
const TF_NODELAY: u16 = 0x0004;
const TF_NOOPT: u16 = 0x0008;
const TF_SENTFIN: u16 = 0x0010;

// timer indices
const TCPT_REXMT: usize = 0;
const TCPT_PERSIST: usize = 1;

const TCP_MAXWIN: i64 = 65535;
const IF_MAXLINKHDR: usize = 2 + 14 + 40;
const TCPOPT_MAXSEG: u8 = 2;
// sizeof(struct tcphdr) == 20
const TCPHDR_SIZE: i64 = 20;
const IPDEFTTL: u8 = 64;
// sizeof(struct tcpiphdr) == 40
const TCPIPHDR_SIZE: usize = 40;

// static const u_char tcp_outflags[TCP_NSTATES] (a file-scope static in the C,
// therefore not a linker symbol: replicated here).
const TCP_OUTFLAGS: [u8; 11] = [
    (TH_RST | TH_ACK) as u8, // 0x14
    0,
    TH_SYN as u8,            // 0x02
    (TH_SYN | TH_ACK) as u8, // 0x12
    TH_ACK as u8,            // 0x10
    TH_ACK as u8,            // 0x10
    (TH_FIN | TH_ACK) as u8, // 0x11
    (TH_FIN | TH_ACK) as u8, // 0x11
    (TH_FIN | TH_ACK) as u8, // 0x11
    TH_ACK as u8,            // 0x10
    TH_ACK as u8,            // 0x10
];

#[inline]
fn tcp_outflags_at(state: i16) -> c_int {
    let i = state as i32;
    if i >= 0 && i < TCP_OUTFLAGS.len() as i32 {
        TCP_OUTFLAGS[i as usize] as c_int
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Mirrored C structures
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

// struct ip (packed).  The `u_int ip_hl:4, ip_v:4;` bitfield occupies the
// first byte on little-endian hosts, immediately followed by ip_tos.
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

// struct mbuf_ptr (packed): a single pointer.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
}

// struct ipovly (packed)
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

// struct tcphdr.  The `u_int th_x2:4, th_off:4;` bitfield lives in the byte at
// offset 12 (th_x2 in the low nibble, th_off in the high nibble on
// little-endian hosts); th_flags follows at offset 13.  Total size 20.
#[repr(C)]
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

// struct tcpiphdr
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
#[derive(Clone, Copy)]
pub union M_dat {
    pub m_dat_: [u8; 1],
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
    pub extra: *mut std::ffi::c_void,
}

unsafe extern "C" {
    fn m_get(slirp: *mut Slirp) -> *mut mbuf;
    fn sbcopy(sb: *mut sbuf, off: c_int, len: c_int, to: *mut c_char);
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
    fn ip_output(so: *mut socket, m: *mut mbuf) -> c_int;
    fn tcp_mss(tp: *mut tcpcb, offer: c_uint) -> c_int;
    fn tcp_setpersist(tp: *mut tcpcb);
    fn htons(x: u16) -> u16;
    fn htonl(x: u32) -> u32;
    fn ntohl(x: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_output(tp: *mut tcpcb) -> c_int {
    unsafe {
        let so: *mut socket = (*tp).t_socket;
        let mut len: i64;
        let mut win: i64;
        let mut off: c_int;
        let mut flags: c_int;
        let mut m: *mut mbuf;
        let mut ti: *mut tcpiphdr;
        let mut opt = [0u8; 32];
        let mut optlen: u32;
        let mut hdrlen: u32;
        let mut sendalot: c_int;

        /*
         * Determine length of data that should be transmitted,
         * and flags that will be used.
         */
        let idle: c_int = if (*tp).snd_max == (*tp).snd_una { 1 } else { 0 };
        if idle != 0 && (*tp).t_idle >= (*tp).t_rxtcur {
            /*
             * We have been idle for "a while" and no acks are
             * expected to clock out any data we send --
             * slow start to get ack "clock" running again.
             */
            (*tp).snd_cwnd = (*tp).t_maxseg as u32;
        }

        'again: loop {
            sendalot = 0;
            off = (*tp).snd_nxt.wrapping_sub((*tp).snd_una) as c_int;
            {
                let w = (*tp).snd_wnd;
                let c = (*tp).snd_cwnd;
                win = (if w < c { w } else { c }) as i64;
            }

            flags = tcp_outflags_at((*tp).t_state);

            /*
             * If in persist timeout with window of 0, send 1 byte.
             * Otherwise, if window is small but nonzero and the timer
             * expired, we will send what we can and go to transmit state.
             */
            if (*tp).t_force != 0 {
                if win == 0 {
                    /*
                     * If we still have some data to send, then clear the FIN
                     * bit.  We can't just blindly clear the FIN bit, because
                     * if we don't have any more data to send then the probe
                     * will be the FIN itself.
                     */
                    if (off as u32) < (*so).so_snd.sb_cc {
                        flags &= !TH_FIN;
                    }
                    win = 1;
                } else {
                    (*tp).t_timer[TCPT_PERSIST] = 0;
                    (*tp).t_rxtshift = 0;
                }
            }

            let sb_cc_snd: i64 = (*so).so_snd.sb_cc as i64;
            len = (if sb_cc_snd < win { sb_cc_snd } else { win }) - (off as i64);

            if len < 0 {
                /*
                 * If FIN has been sent but not acked, but we haven't been
                 * called to retransmit, len will be -1.  If window shrank to
                 * 0, cancel pending retransmit and pull snd_nxt back to the
                 * (closed) window.
                 */
                len = 0;
                if win == 0 {
                    (*tp).t_timer[TCPT_REXMT] = 0;
                    (*tp).snd_nxt = (*tp).snd_una;
                }
            }

            if len > (*tp).t_maxseg as i64 {
                len = (*tp).t_maxseg as i64;
                sendalot = 1;
            }
            // SEQ_LT(tp->snd_nxt + len, tp->snd_una + so->so_snd.sb_cc)
            {
                let a: i64 = ((*tp).snd_nxt as i64).wrapping_add(len);
                let b: u32 = (*tp).snd_una.wrapping_add((*so).so_snd.sb_cc);
                if (a.wrapping_sub(b as i64) as c_int) < 0 {
                    flags &= !TH_FIN;
                }
            }

            // win = sbspace(&so->so_rcv);
            win = (*so).so_rcv.sb_datalen.wrapping_sub((*so).so_rcv.sb_cc) as i64;

            /*
             * Sender silly window avoidance.
             */
            let mut go: bool = false;
            if len != 0 {
                if len == (*tp).t_maxseg as i64 {
                    go = true;
                }
                if !go && len + (off as i64) >= (*so).so_snd.sb_cc as i64 {
                    go = true;
                }
                if !go && (*tp).t_force != 0 {
                    go = true;
                }
                if !go && len >= ((*tp).max_sndwnd / 2) as i64 && (*tp).max_sndwnd > 0 {
                    go = true;
                }
                if !go && ((*tp).snd_nxt.wrapping_sub((*tp).snd_max) as c_int) < 0 {
                    go = true;
                }
            }

            /*
             * Compare available window to amount of window known to peer.
             * If the difference is at least two max size segments, or at
             * least 50% of the maximum possible window, then we want to
             * send a window update to peer.
             */
            if !go && win > 0 {
                let maxwin: i64 = TCP_MAXWIN.wrapping_shl((*tp).rcv_scale as u32);
                let basis: i64 = if win < maxwin { win } else { maxwin };
                let adv: i64 = basis
                    .wrapping_sub((*tp).rcv_adv.wrapping_sub((*tp).rcv_nxt) as i64);

                if adv >= (2i32.wrapping_mul((*tp).t_maxseg as i32)) as i64 {
                    go = true;
                }
                if !go && 2i64.wrapping_mul(adv) >= (*so).so_rcv.sb_datalen as i64 {
                    go = true;
                }
            }

            /*
             * Send if we owe peer an ACK.
             */
            if !go && ((*tp).t_flags & TF_ACKNOW) != 0 {
                go = true;
            }
            if !go && (flags & (TH_SYN | TH_RST)) != 0 {
                go = true;
            }
            if !go && ((*tp).snd_up.wrapping_sub((*tp).snd_una) as c_int) > 0 {
                go = true;
            }
            /*
             * If our state indicates that FIN should be sent and we have not
             * yet done so, or we're retransmitting the FIN, then we need to
             * send.
             */
            if !go
                && (flags & TH_FIN) != 0
                && (((*tp).t_flags & TF_SENTFIN) == 0 || (*tp).snd_nxt == (*tp).snd_una)
            {
                go = true;
            }

            if !go {
                /*
                 * If send window is too small, there is data to transmit, and
                 * no retransmit or persist is pending, then go to persist
                 * state.
                 */
                if (*so).so_snd.sb_cc != 0
                    && (*tp).t_timer[TCPT_REXMT] == 0
                    && (*tp).t_timer[TCPT_PERSIST] == 0
                {
                    (*tp).t_rxtshift = 0;
                    tcp_setpersist(tp);
                }

                /*
                 * No reason to send a segment, just return.
                 */
                return 0;
            }

            // -------------------------------------------------------------
            // send:
            // -------------------------------------------------------------
            optlen = 0;
            hdrlen = TCPIPHDR_SIZE as u32;
            if (flags & TH_SYN) != 0 {
                (*tp).snd_nxt = (*tp).iss;
                if ((*tp).t_flags & TF_NOOPT) == 0 {
                    let mss: u16 = htons(tcp_mss(tp, 0) as u16);
                    opt[0] = TCPOPT_MAXSEG;
                    opt[1] = 4;
                    copy_nonoverlapping(addr_of!(mss) as *const u8, opt.as_mut_ptr().add(2), 2);
                    optlen = 4;
                }
            }

            hdrlen = hdrlen.wrapping_add(optlen);

            /*
             * Adjust data length if insertion of options will bump the packet
             * length beyond the t_maxseg length.
             */
            {
                let maxseg: u32 = (*tp).t_maxseg as u32;
                let limit: i64 = maxseg.wrapping_sub(optlen) as i64;
                if len > limit {
                    len = limit;
                    sendalot = 1;
                }
            }

            /*
             * Grab a header mbuf, attaching a copy of data to be transmitted.
             */
            if len != 0 {
                m = m_get((*so).slirp);
                if m.is_null() {
                    return 1;
                }
                (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(IF_MAXLINKHDR);
                (*m).m_hdr.mh_len = hdrlen as c_int;

                sbcopy(
                    addr_of_mut!((*so).so_snd),
                    off,
                    len as c_int,
                    (*m).m_hdr.mh_data.add(hdrlen as usize),
                );
                (*m).m_hdr.mh_len = ((*m).m_hdr.mh_len as i64).wrapping_add(len) as c_int;

                /*
                 * If we're sending everything we've got, set PUSH.
                 */
                if (off as i64).wrapping_add(len) == (*so).so_snd.sb_cc as i64 {
                    flags |= TH_PUSH;
                }
            } else {
                m = m_get((*so).slirp);
                if m.is_null() {
                    return 1;
                }
                (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.add(IF_MAXLINKHDR);
                (*m).m_hdr.mh_len = hdrlen as c_int;
            }

            ti = (*m).m_hdr.mh_data as *mut tcpiphdr;

            copy_nonoverlapping(
                addr_of!((*tp).t_template) as *const u8,
                ti as *mut u8,
                size_of::<tcpiphdr>(),
            );

            /*
             * Fill in fields, remembering maximum advertised window for use in
             * delaying messages about window sizes.  If resending a FIN, be
             * sure not to use a new sequence number.
             */
            if (flags & TH_FIN) != 0
                && ((*tp).t_flags & TF_SENTFIN) != 0
                && (*tp).snd_nxt == (*tp).snd_max
            {
                (*tp).snd_nxt = (*tp).snd_nxt.wrapping_sub(1);
            }
            /*
             * For ACK only packets, we do not want the sequence number of the
             * retransmitted packet, we want the sequence number of the next
             * unsent octet.  But if we are in persist state, use snd_nxt, since
             * we know we aren't doing a retransmission.
             */
            if len != 0
                || (flags & (TH_SYN | TH_FIN)) != 0
                || (*tp).t_timer[TCPT_PERSIST] != 0
            {
                (*ti).ti_t.th_seq = htonl((*tp).snd_nxt);
            } else {
                (*ti).ti_t.th_seq = htonl((*tp).snd_max);
            }
            (*ti).ti_t.th_ack = htonl((*tp).rcv_nxt);
            if optlen != 0 {
                copy_nonoverlapping(
                    opt.as_ptr(),
                    (ti as *mut u8).add(TCPIPHDR_SIZE),
                    optlen as usize,
                );
                // ti->ti_off = (sizeof (struct tcphdr) + optlen) >> 2;
                let thoff: u8 = (TCPHDR_SIZE.wrapping_add(optlen as i64) >> 2) as u8;
                let p = addr_of_mut!((*ti).ti_t.th_x2_off);
                *p = (*p & 0x0f) | ((thoff & 0x0f) << 4);
            }
            (*ti).ti_t.th_flags = flags as u8;

            /*
             * Calculate receive window.  Don't shrink window, but avoid silly
             * window syndrome.
             */
            if win < ((*so).so_rcv.sb_datalen / 4) as i64 && win < (*tp).t_maxseg as i64 {
                win = 0;
            }
            {
                let maxwin: i64 = TCP_MAXWIN.wrapping_shl((*tp).rcv_scale as u32);
                if win > maxwin {
                    win = maxwin;
                }
            }
            {
                let advwin: i64 = (*tp).rcv_adv.wrapping_sub((*tp).rcv_nxt) as i64;
                if win < advwin {
                    win = advwin;
                }
            }
            (*ti).ti_t.th_win = htons(win.wrapping_shr((*tp).rcv_scale as u32) as u16);

            if ((*tp).snd_up.wrapping_sub((*tp).snd_una) as c_int) > 0 {
                (*ti).ti_t.th_urp =
                    htons((*tp).snd_up.wrapping_sub(ntohl((*ti).ti_t.th_seq)) as u16);
                (*ti).ti_t.th_flags |= TH_URG as u8;
            } else {
                /*
                 * If no urgent pointer to send, then we pull the urgent pointer
                 * to the left edge of the send window so that it doesn't drift
                 * into the send window on sequence number wraparound.
                 */
                (*tp).snd_up = (*tp).snd_una;
            }

            /*
             * Put TCP length in extended header, and then checksum extended
             * header and data.
             */
            if len.wrapping_add(optlen as i64) != 0 {
                let tl: i64 = TCPHDR_SIZE.wrapping_add(optlen as i64).wrapping_add(len);
                addr_of_mut!((*ti).ti_i.ih_len).write_unaligned(htons(tl as u16));
            }
            // th_sum is uint16_t, the int result is truncated by the assignment
            (*ti).ti_t.th_sum = cksum(m, (hdrlen as i64).wrapping_add(len) as c_int) as u16;

            /*
             * In transmit state, time the transmission and arrange for the
             * retransmit.  In persist state, just set snd_max.
             */
            if (*tp).t_force == 0 || (*tp).t_timer[TCPT_PERSIST] == 0 {
                let startseq: u32 = (*tp).snd_nxt;

                /*
                 * Advance snd_nxt over sequence space of this segment.
                 */
                if (flags & (TH_SYN | TH_FIN)) != 0 {
                    if (flags & TH_SYN) != 0 {
                        (*tp).snd_nxt = (*tp).snd_nxt.wrapping_add(1);
                    }
                    if (flags & TH_FIN) != 0 {
                        (*tp).snd_nxt = (*tp).snd_nxt.wrapping_add(1);
                        (*tp).t_flags |= TF_SENTFIN;
                    }
                }
                (*tp).snd_nxt = (*tp).snd_nxt.wrapping_add(len as u32);
                if ((*tp).snd_nxt.wrapping_sub((*tp).snd_max) as c_int) > 0 {
                    (*tp).snd_max = (*tp).snd_nxt;
                    /*
                     * Time this transmission if not a retransmission and not
                     * currently timing anything.
                     */
                    if (*tp).t_rtt == 0 {
                        (*tp).t_rtt = 1;
                        (*tp).t_rtseq = startseq;
                    }
                }

                /*
                 * Set retransmit timer if not currently set, and not doing an
                 * ack or a keep-alive probe.
                 */
                if (*tp).t_timer[TCPT_REXMT] == 0 && (*tp).snd_nxt != (*tp).snd_una {
                    (*tp).t_timer[TCPT_REXMT] = (*tp).t_rxtcur;
                    if (*tp).t_timer[TCPT_PERSIST] != 0 {
                        (*tp).t_timer[TCPT_PERSIST] = 0;
                        (*tp).t_rxtshift = 0;
                    }
                }
            } else if ((*tp).snd_nxt as i64)
                .wrapping_add(len)
                .wrapping_sub((*tp).snd_max as i64) as c_int
                > 0
            {
                (*tp).snd_max = ((*tp).snd_nxt as i64).wrapping_add(len) as u32;
            }

            /*
             * Fill in IP length and desired time to live and send to IP level.
             */
            (*m).m_hdr.mh_len = (hdrlen as i64).wrapping_add(len) as c_int;

            {
                let ipp = ti as *mut ip;
                addr_of_mut!((*ipp).ip_len).write_unaligned((*m).m_hdr.mh_len as u16);
                addr_of_mut!((*ipp).ip_ttl).write_unaligned(IPDEFTTL);
                addr_of_mut!((*ipp).ip_tos).write_unaligned((*so).so_iptos);
            }

            let error: c_int = ip_output(so, m);
            if error != 0 {
                return error;
            }

            /*
             * Data sent (as far as we can tell).  If this advertises a larger
             * window than any other segment, then remember the size of the
             * advertised window.  Any pending ACK has now been sent.
             */
            {
                let a: i64 = ((*tp).rcv_nxt as i64).wrapping_add(win);
                if win > 0 && (a.wrapping_sub((*tp).rcv_adv as i64) as c_int) > 0 {
                    (*tp).rcv_adv = a as u32;
                }
            }
            (*tp).last_ack_sent = (*tp).rcv_nxt;
            (*tp).t_flags &= !(TF_ACKNOW | TF_DELACK);
            if sendalot != 0 {
                continue 'again;
            }

            return 0;
        }
    }
}
}

// ── `register struct socket *so = tp->t_socket; register long len, win; int off, flags, error; regis…` · slirp/tcp_output.c:58-76 · chunk 2739 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct socket *so = tp->t_socket;
//      register long len, win;
//      int off, flags, error;
//      register struct mbuf *m;
//      register struct tcpiphdr *ti;
//      u_char opt[32];
//      unsigned optlen, hdrlen;
//      int idle, sendalot;
//     
//      ;
//      ;
//     
//     
//     
//     
//     
//     
//     
//      idle = (tp->snd_max == tp->snd_una);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2739 {
// Port of the declaration / initialization prologue of tcp_output():
//   so   = tp->t_socket
//   idle = (tp->snd_max == tp->snd_una)
// The remaining locals are still declared in the C replacement.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

// struct ipovly (packed)
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr; th_x2:4 and th_off:4 share one unsigned int bit-field unit.
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
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

// struct socket: opaque here, never dereferenced.
#[repr(C)]
pub struct Socket {
    pub _opaque: [u8; 0],
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

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2739(
    tp: *mut Tcpcb,
    so_out: *mut *mut Socket,
) -> c_int {
    let so = (*tp).t_socket;
    *so_out = so;
    let idle = ((*tp).snd_max == (*tp).snd_una) as c_int;
    idle
}
}

// ── `if (idle && tp->t_idle >= tp->t_rxtcur)` · slirp/tcp_output.c:77-83 · chunk 2740 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (idle && tp->t_idle >= tp->t_rxtcur)
//     
//     
//     
//     
//     
//       tp->snd_cwnd = tp->t_maxseg;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2740 {
// Rust port of tcp_output's idle-cwnd-reset piece.

use core::ffi::{c_char, c_int, c_short, c_ushort};

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut core::ffi::c_void,
}

#[repr(C, packed)]
pub struct IpOvly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: u32,
    pub ih_dst: u32,
}

#[repr(C)]
pub struct TcpHdr {
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
pub struct TcpIpHdr {
    pub ti_i: IpOvly,
    pub ti_t: TcpHdr,
}

#[repr(C)]
pub struct TcpCb {
    pub seg_next: *mut TcpIpHdr,
    pub seg_prev: *mut TcpIpHdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
    pub t_template: TcpIpHdr,
    pub t_socket: *mut core::ffi::c_void,
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

#[no_mangle]
pub extern "C" fn tcp_output__part2740(idle: c_int, tp: *mut TcpCb) {
    unsafe {
        if tp.is_null() {
            return;
        }
        if idle != 0 && (*tp).t_idle >= (*tp).t_rxtcur {
            (*tp).snd_cwnd = (*tp).t_maxseg as u32;
        }
    }
}
}

// ── `again: sendalot = 0;` · slirp/tcp_output.c:84-85 · chunk 2741 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     again:
//      sendalot = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2741 {
// Portion of tcp_output(): the `again:` label target that resets `sendalot`.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2741(sendalot: *mut c_int) {
    if !sendalot.is_null() {
        unsafe {
            *sendalot = 0;
        }
    }
}
}

// ── `off = tp->snd_nxt - tp->snd_una; win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->s…` · slirp/tcp_output.c:86-91 · chunk 2742 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     off = tp->snd_nxt - tp->snd_una;
//      win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->snd_cwnd));
//     
//      flags = tcp_outflags[tp->t_state];
//     
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2742 {
// Rust port of a piece of slirp's tcp_output():
//   off   = tp->snd_nxt - tp->snd_una;
//   win   = min(tp->snd_wnd, tp->snd_cwnd);
//   flags = tcp_outflags[tp->t_state];
// `win` is a C `register` variable in the caller, so the three results are
// returned by value in a small repr(C) struct instead of through pointers.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_short, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut c_void,
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

// u_int th_x2:4, th_off:4 lives in one unsigned int allocation unit.
#[repr(C)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
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

// Only reached through a pointer; `struct socket` is opaque here.
#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut tcpiphdr,
    pub seg_prev: *mut tcpiphdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: u16,
    pub t_force: c_char,
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
pub struct tcp_output__part2742_ret {
    pub off: c_int,
    pub win: c_long,
    pub flags: c_int,
}

// `static const u_char tcp_outflags[11]` has no linker symbol: keep the copy here.
static TCP_OUTFLAGS: [u8; 11] = [
    0x04 | 0x10,
    0,
    0x02,
    0x02 | 0x10,
    0x10,
    0x10,
    0x01 | 0x10,
    0x01 | 0x10,
    0x01 | 0x10,
    0x10,
    0x10,
];

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2742(tp: *mut tcpcb) -> tcp_output__part2742_ret {
    let mut ret = tcp_output__part2742_ret {
        off: 0,
        win: 0,
        flags: 0,
    };

    if tp.is_null() {
        return ret;
    }
    let t: &tcpcb = &*tp;

    // off = tp->snd_nxt - tp->snd_una;  (uint32 arithmetic, stored into int)
    ret.off = t.snd_nxt.wrapping_sub(t.snd_una) as c_int;

    // win = min(tp->snd_wnd, tp->snd_cwnd);  (unsigned -> long keeps value)
    let w: u32 = if t.snd_wnd < t.snd_cwnd {
        t.snd_wnd
    } else {
        t.snd_cwnd
    };
    ret.win = w as c_long;

    // flags = tcp_outflags[tp->t_state];
    let idx = t.t_state as c_int;
    let fl: u8 = if idx >= 0 && (idx as usize) < TCP_OUTFLAGS.len() {
        match TCP_OUTFLAGS.get(idx as usize) {
            Some(v) => *v,
            None => 0,
        }
    } else {
        0
    };
    ret.flags = fl as c_int;

    ret
}
}

// ── `if (tp->t_force)` · slirp/tcp_output.c:99-124 · chunk 2743 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_force) {
//       if (win == 0) {
//        if (off < so->so_snd.sb_cc)
//         flags &= ~0x01;
//        win = 1;
//       } else {
//        tp->t_timer[1] = 0;
//        tp->t_rxtshift = 0;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2743 {
// Port of a slice of slirp's tcp_output: the "t_force" window handling.
// win is a C `register` local, so it is passed by value and returned.

#[no_mangle]
pub extern "C" fn tcp_output__part2743(
    t_force: core::ffi::c_char,
    off: core::ffi::c_int,
    sb_cc: core::ffi::c_uint,
    timer1: *mut core::ffi::c_short,
    rxtshift: *mut core::ffi::c_short,
    win_in: core::ffi::c_long,
    flags: *mut core::ffi::c_int,
) -> core::ffi::c_long {
    let mut win = win_in;
    unsafe {
        if t_force != 0 {
            if win == 0 {
                // C computes `off < so->so_snd.sb_cc` as an unsigned comparison.
                if (off as core::ffi::c_uint) < sb_cc {
                    *flags &= !0x01;
                }
                win = 1;
            } else {
                *timer1 = 0;
                *rxtshift = 0;
            }
        }
    }
    win
}
}

// ── `len = ((so->so_snd.sb_cc) < (win) ? (so->so_snd.sb_cc) : (win)) - off;` · slirp/tcp_output.c:126-126 · chunk 2744 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     len = ((so->so_snd.sb_cc) < (win) ? (so->so_snd.sb_cc) : (win)) - off;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2744 {
// Piece of tcp_output: computes len = min(so->so_snd.sb_cc, win) - off.
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct Sb_uf {
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
    pub so_rcv: Sb_uf,
    pub so_snd: Sb_uf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2744(so: *mut Socket, win: c_long, off: c_int) -> c_long {
    let cc = unsafe { (*so).so_snd.sb_cc } as c_long;
    let m = if cc < win { cc } else { win };
    m - off as c_long
}
}

// ── `if (len < 0)` · slirp/tcp_output.c:128-144 · chunk 2745 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len < 0) {
//       len = 0;
//       if (win == 0) {
//        tp->t_timer[0] = 0;
//        tp->snd_nxt = tp->snd_una;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2745 {
// Ported piece of tcp_output(): the "len < 0" adjustment after
//     len = min(so->so_snd.sb_cc, win) - off;
// Original C:
//   if (len < 0) {
//     len = 0;
//     if (win == 0) {
//       tp->t_timer[TCPT_REXMT] = 0;
//       tp->snd_nxt = tp->snd_una;
//     }
//   }

#![allow(dead_code)]

use core::ffi::{c_char, c_long, c_short, c_ushort, c_void};

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
    // u_int th_x2:4, th_off:4;  -> storage unit, size/alignment preserved
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

// Mirror of struct tcpcb (fields up to the ones touched here).
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
    pub t_socket: *mut c_void,
    pub snd_una: u32,
    pub snd_nxt: u32,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2745(tp: *mut Tcpcb, win: c_long, len: c_long) -> c_long {
    if len < 0 {
        // len = 0; and, if the window closed completely, cancel the
        // pending retransmit timer and pull snd_nxt back to the window.
        if win == 0 {
            unsafe {
                (*tp).t_timer[0] = 0;
                (*tp).snd_nxt = (*tp).snd_una;
            }
        }
        0
    } else {
        len
    }
}
}

// ── `if (len > tp->t_maxseg)` · slirp/tcp_output.c:146-149 · chunk 2746 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len > tp->t_maxseg) {
//       len = tp->t_maxseg;
//       sendalot = 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2746 {
use core::ffi::{c_int, c_long, c_ushort};

#[no_mangle]
pub extern "C" fn tcp_output__part2746(mut len: c_long, t_maxseg: c_ushort, sendalot: *mut c_int) -> c_long {
    if len > t_maxseg as c_long {
        len = t_maxseg as c_long;
        unsafe {
            *sendalot = 1;
        }
    }
    len
}
}

// ── `if ((int)((tp->snd_nxt + len)-(tp->snd_una + so->so_snd.sb_cc)) < 0)` · slirp/tcp_output.c:150-151 · chunk 2747 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (((int)((tp->snd_nxt + len)-(tp->snd_una + so->so_snd.sb_cc)) < 0))
//       flags &= ~0x01;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2747 {
// Port of tcp_output's "flags &= ~TH_FIN" guard:
//   if (((int)((tp->snd_nxt + len) - (tp->snd_una + so->so_snd.sb_cc)) < 0))
//       flags &= ~0x01;
// Note the C usual arithmetic conversions:
//  - snd_nxt (uint32_t / unsigned int) + len (long)  -> long (no overflow)
//  - snd_una + so_snd.sb_cc (both 32-bit unsigned)   -> wraps modulo 2^32
//  - the subtraction is done in long, the (int) cast truncates to 32 bits,
//    and only then the sign is tested.

use core::ffi::{c_int, c_long, c_uint};

#[no_mangle]
pub extern "C" fn tcp_output__part2747(
    snd_nxt: c_uint,
    len: c_long,
    snd_una: c_uint,
    sb_cc: c_uint,
    flags: *mut c_int,
) {
    let left: i64 = snd_nxt as i64 + len as i64;
    let right: i64 = (snd_una.wrapping_add(sb_cc)) as i64;
    let diff: i32 = (left - right) as i32;

    if diff < 0 {
        unsafe {
            if !flags.is_null() {
                *flags &= !1;
            }
        }
    }
}
}

// ── `win = ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc);` · slirp/tcp_output.c:153-153 · chunk 2748 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     win = ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2748 {
use core::ffi::{c_char, c_long, c_uint};

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// win = (long)(&so->so_rcv)->sb_datalen - (long)((unsigned int subtraction))
#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2748(sb: *const Sbuf) -> c_long {
    let datalen = (*sb).sb_datalen;
    let cc = (*sb).sb_cc;
    datalen.wrapping_sub(cc) as c_long
}
}

// ── `if (len)` · slirp/tcp_output.c:165-177 · chunk 2749 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len) {
//       if (len == tp->t_maxseg)
//        goto send;
//       if ((1 || idle || tp->t_flags & 0x0004) &&
//           len + off >= so->so_snd.sb_cc)
//        goto send;
//       if (tp->t_force)
//        goto send;
//       if (len >= tp->max_sndwnd / 2 && tp->max_sndwnd > 0)
//        goto send;
//       if (((int)((tp->snd_nxt)-(tp->snd_max)) < 0))
//        goto send;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2749 {
// Ported piece of slirp tcp_output: sender silly-window-avoidance decision.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ushort, c_void};

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

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,  /* struct mbuf * */
    pub so_ti: *mut c_void, /* struct tcpiphdr * */
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

/* struct tcpiphdr: struct ipovly (packed, 20 bytes) + struct tcphdr
 * (bitfields, 20 bytes); total 40 bytes with 4-byte alignment. */
#[repr(C, align(4))]
pub struct tcpiphdr {
    pub bytes: [u8; 40],
}

#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut c_void, /* struct tcpiphdr * */
    pub seg_prev: *mut c_void, /* struct tcpiphdr * */
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
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

/// Returns 1 when tcp_output should `goto send`, 0 when it should fall through.
#[no_mangle]
pub extern "C" fn tcp_output__part2749(
    tp: *mut tcpcb,
    so: *mut socket,
    len: c_long,
    off: c_int,
    idle: c_int,
) -> c_int {
    let _ = idle;
    let tp: &tcpcb = unsafe { &*tp };
    let so: &socket = unsafe { &*so };

    if len != 0 {
        if len == tp.t_maxseg as c_long {
            return 1;
        }
        // (1 || idle || tp->t_flags & TF_NODELAY) && len + off >= so->so_snd.sb_cc
        if len + off as c_long >= so.so_snd.sb_cc as c_long {
            return 1;
        }
        if tp.t_force != 0 {
            return 1;
        }
        if len >= (tp.max_sndwnd / 2) as c_long && tp.max_sndwnd > 0 {
            return 1;
        }
        // SEQ_LT(tp->snd_nxt, tp->snd_max)
        if (tp.snd_nxt.wrapping_sub(tp.snd_max) as i32) < 0 {
            return 1;
        }
    }
    0
}
}

// ── `if (win > 0)` · slirp/tcp_output.c:186-199 · chunk 2750 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (win > 0) {
//     
//     
//     
//     
//     
//       long adv = ((win) < ((long)65535 << tp->rcv_scale) ? (win) : ((long)65535 << tp->rcv_scale)) -
//        (tp->rcv_adv - tp->rcv_nxt);
//     
//       if (adv >= (long) (2 * tp->t_maxseg))
//        goto send;
//       if (2 * adv >= (long) so->so_rcv.sb_datalen)
//        goto send;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2750 {
// Port of the "window advertisement / send-now?" test inside tcp_output().
// Returns 1 when the C code would `goto send`, 0 otherwise.
use core::ffi::{c_int, c_long, c_uchar, c_uint, c_ushort};

#[no_mangle]
pub extern "C" fn tcp_output__part2750(
    win: c_long,
    rcv_scale: c_uchar,
    rcv_adv: u32,
    rcv_nxt: u32,
    t_maxseg: c_ushort,
    sb_datalen: c_uint,
) -> c_int {
    // if (win > 0) { ... }
    if win <= 0 {
        return 0;
    }

    // ((win) < ((long)65535 << tp->rcv_scale) ? (win) : ((long)65535 << tp->rcv_scale))
    let maxwin: c_long = (65535 as c_long).wrapping_shl(rcv_scale as u32);
    let m: c_long = if win < maxwin { win } else { maxwin };

    // (tp->rcv_adv - tp->rcv_nxt) is computed in uint32_t arithmetic, then widened to long
    let diff: c_long = rcv_adv.wrapping_sub(rcv_nxt) as c_long;

    let adv: c_long = m.wrapping_sub(diff);

    // if (adv >= (long) (2 * tp->t_maxseg)) goto send;
    if adv >= (2 * (t_maxseg as c_int)) as c_long {
        return 1;
    }
    // if (2 * adv >= (long) so->so_rcv.sb_datalen) goto send;
    if (2 as c_long).wrapping_mul(adv) >= (sb_datalen as c_long) {
        return 1;
    }

    0
}
}

// ── `if (tp->t_flags & 0x0001)` · slirp/tcp_output.c:204-205 · chunk 2751 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_flags & 0x0001)
//       goto send;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2751 {
// Port of the "if (tp->t_flags & TF_NODELAY) goto send;" decision from
// tcp_output().  Returns nonzero when the C caller must jump to `send`.
use core::ffi::{c_char, c_int, c_short, c_ushort, c_void};

// Mirror of the leading fields of C `struct tcpcb` that are needed to reach
// `t_flags` at the correct offset.
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
    pub t_force: c_char,
    pub t_flags: c_ushort,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2751(tp: *mut Tcpcb) -> c_int {
    if tp.is_null() {
        return 0;
    }
    let t_flags = unsafe { (*tp).t_flags };
    if (t_flags & 0x0001u16) != 0 {
        1
    } else {
        0
    }
}
}

// ── `if (flags & (0x02|0x04)` · slirp/tcp_output.c:206-207 · chunk 2752 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & (0x02|0x04))
//       goto send;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2752 {
// Piece of slirp tcp_output: test whether the flags include SYN or RST.
#[no_mangle]
pub extern "C" fn tcp_output__part2752(flags: core::ffi::c_int) -> core::ffi::c_int {
    if (flags & (0x02 | 0x04)) != 0 {
        1
    } else {
        0
    }
}
}

// ── `if ((int)((tp->snd_up)-(tp->snd_una)) > 0)` · slirp/tcp_output.c:208-209 · chunk 2753 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (((int)((tp->snd_up)-(tp->snd_una)) > 0))
//       goto send;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2753 {
// Portion of tcp_output: decide whether an urgent-pointer update forces a send.
// C original:  if (((int)((tp->snd_up)-(tp->snd_una)) > 0)) goto send;
// tcp_seq is uint32_t, so the subtraction is performed modulo 2^32 and the
// 32-bit result is reinterpreted as a signed int before the comparison.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2753(snd_up: u32, snd_una: u32) -> c_int {
    let diff: i32 = snd_up.wrapping_sub(snd_una) as i32;
    if diff > 0 {
        1
    } else {
        0
    }
}
}

// ── `if (flags & 0x01 && ((tp->t_flags & 0x0010) == 0 || tp->snd_nxt == tp->snd_una)` · slirp/tcp_output.c:215-217 · chunk 2754 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & 0x01 &&
//          ((tp->t_flags & 0x0010) == 0 || tp->snd_nxt == tp->snd_una))
//       goto send;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2754 {
// Portion of tcp_output: decide whether to jump to `send`.
use core::ffi::c_int;

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut core::ffi::c_void,
}

#[repr(C, packed)]
pub struct InAddr {
    pub s_addr: u32,
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
    // u_int th_x2:4, th_off:4  -- single 4-byte storage unit
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
    pub t_socket: *mut core::ffi::c_void,
    pub snd_una: u32,
    pub snd_nxt: u32,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2754(tp: *mut Tcpcb, flags: c_int) -> c_int {
    unsafe {
        let tp = &*tp;
        if (flags & 0x01) != 0 && ((tp.t_flags & 0x0010) == 0 || tp.snd_nxt == tp.snd_una) {
            1
        } else {
            0
        }
    }
}
}

// ── `if (so->so_snd.sb_cc && tp->t_timer[0] == 0 && tp->t_timer[1] == 0)` · slirp/tcp_output.c:241-245 · chunk 2755 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_snd.sb_cc && tp->t_timer[0] == 0 &&
//          tp->t_timer[1] == 0) {
//       tp->t_rxtshift = 0;
//       tcp_setpersist(tp);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2755 {
// Portion of tcp_output: the persist-timer setup branch.
// Only fields up to the ones touched are mirrored; leading layout matches C.

use core::ffi::{c_char, c_int, c_short, c_uint};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
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
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

extern "C" {
    fn tcp_setpersist(tp: *mut Tcpcb);
}

#[no_mangle]
pub extern "C" fn tcp_output__part2755(tp: *mut Tcpcb, so: *mut Socket) {
    unsafe {
        if (*so).so_snd.sb_cc != 0 && (*tp).t_timer[0] == 0 && (*tp).t_timer[1] == 0 {
            (*tp).t_rxtshift = 0;
            tcp_setpersist(tp);
        }
    }
}
}

// ── `return (0);` · slirp/tcp_output.c:250-250 · chunk 2756 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2756 {
// Port of the `return (0);` exit path of tcp_output (slirp/tcp_output.c).
// The original statement is a plain "no data to send" return of 0; it reads
// no state of the translation unit and has no side effects.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2756() -> c_int {
    0
}
}

// ── `send: optlen = 0;` · slirp/tcp_output.c:252-261 · chunk 2757 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     send:
//      optlen = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2757 {
// Ported piece of tcp_output: the `optlen = 0;` initialisation at the `send:` label.
#[no_mangle]
pub extern "C" fn tcp_output__part2757() -> core::ffi::c_uint {
    0u32
}
}

// ── `hdrlen = sizeof (struct tcpiphdr);` · slirp/tcp_output.c:262-262 · chunk 2758 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     hdrlen = sizeof (struct tcpiphdr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2758 {
// Port of the `hdrlen = sizeof (struct tcpiphdr);` statement from
// slirp/tcp_output.c. The C layout of struct tcpiphdr is mirrored here so
// that the size matches the value the C compiler computes.

use core::ffi::{c_uint, c_void};

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
struct MbufPtr {
    mptr: *mut c_void,
}

// struct ipovly (packed): struct mbuf_ptr ih_mbuf; u8 ih_x1; u8 ih_pr;
// u16 ih_len; struct in_addr ih_src, ih_dst;
#[repr(C, packed)]
struct Ipovly {
    ih_mbuf: MbufPtr,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: u32,
    ih_dst: u32,
}

// struct tcphdr (unpacked). The two u_int bit-fields th_x2:4 / th_off:4 occupy
// one byte at offset 12, followed by uint8_t th_flags at offset 13.
#[repr(C)]
struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    th_x2_th_off: u8,
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

// Returns sizeof(struct tcpiphdr) as an `unsigned` (C `unsigned int`).
#[no_mangle]
pub extern "C" fn tcp_output__part2758() -> c_uint {
    core::mem::size_of::<Tcpiphdr>() as c_uint
}
}

// ── `if (flags & 0x02)` · slirp/tcp_output.c:263-274 · chunk 2759 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & 0x02) {
//       tp->snd_nxt = tp->iss;
//       if ((tp->t_flags & 0x0008) == 0) {
//        uint16_t mss;
//     
//        opt[0] = 2;
//        opt[1] = 4;
//        mss = htons((uint16_t) tcp_mss(tp, 0));
//        memcpy((caddr_t)(opt + 2), (caddr_t)&mss, sizeof(mss));
//        optlen = 4;
//       }
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2759 {
// Ported piece of tcp_output(): the SYN option (MSS) generation block.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint, c_void};

// struct mbuf_ptr { struct mbuf *mptr; } (packed) -> pointer sized
#[repr(C)]
pub struct mbuf_ptr {
    pub mptr: *mut c_void,
}

// struct ipovly, __attribute__((packed)): 20 bytes
#[repr(C)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: u32, // struct in_addr { uint32_t s_addr; }
    pub ih_dst: u32,
}

// struct tcphdr: 20 bytes
#[repr(C)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u8, // u_int th_x2:4, th_off:4
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr: 40 bytes
#[repr(C)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

// struct tcpcb, exactly as declared in the preprocessed translation unit
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
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

extern "C" {
    fn tcp_mss(tp: *mut tcpcb, offer: c_uint) -> c_int;
    fn htons(hostshort: u16) -> u16;
}

#[no_mangle]
pub extern "C" fn tcp_output__part2759(
    tp: *mut tcpcb,
    opt: *mut u8,
    optlen: *mut c_uint,
    flags: c_int,
) {
    unsafe {
        if flags & 0x02 != 0 {
            (*tp).snd_nxt = (*tp).iss;
            if (*tp).t_flags & 0x0008 == 0 {
                *opt.offset(0) = 2;
                *opt.offset(1) = 4;
                let mss: u16 = htons(tcp_mss(tp, 0) as u16);
                core::ptr::copy_nonoverlapping(
                    &mss as *const u16 as *const u8,
                    opt.offset(2),
                    2,
                );
                *optlen = 4;
            }
        }
    }
}
}

// ── `hdrlen += optlen;` · slirp/tcp_output.c:276-276 · chunk 2760 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     hdrlen += optlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2760 {
// Computes `hdrlen += optlen` for the two `unsigned` locals of tcp_output.
#[no_mangle]
pub extern "C" fn tcp_output__part2760(hdrlen: u32, optlen: u32) -> u32 {
    hdrlen.wrapping_add(optlen)
}
}

// ── `if (len > tp->t_maxseg - optlen)` · slirp/tcp_output.c:282-285 · chunk 2761 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (len > tp->t_maxseg - optlen) {
//       len = tp->t_maxseg - optlen;
//       sendalot = 1;
//       }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2761 {
use core::ffi::{c_int, c_long, c_uint, c_ushort};

// Port of the length-clamping tail of tcp_output:
//   if (len > tp->t_maxseg - optlen) { len = tp->t_maxseg - optlen; sendalot = 1; }
// The subtraction is done in `unsigned int` (usual arithmetic conversions),
// then converted to `long` for the comparison/assignment.  `len` is a
// register variable in the C caller, so it is passed by value and the
// (possibly unchanged) result is returned.
#[no_mangle]
pub extern "C" fn tcp_output__part2761(
    len: c_long,
    maxseg: c_ushort,
    optlen: c_uint,
    sendalot: *mut c_int,
) -> c_long {
    unsafe {
        let diff_u32: u32 = (maxseg as u32).wrapping_sub(optlen);
        // Zero-extend u32 -> 64-bit signed long (all values representable).
        let diff: c_long = diff_u32 as c_long;
        if len > diff {
            if !sendalot.is_null() {
                *sendalot = 1;
            }
            return diff;
        }
        len
    }
}
}

// ── `if (len) … else` · slirp/tcp_output.c:292-320 · chunk 2762 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len) {
//       m = m_get(so->slirp);
//       if (m == 
//               ((void *)0)
//                   ) {
//        error = 1;
//        goto out;
//       }
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       m->m_hdr.mh_len = hdrlen;
//     
//       sbcopy(&so->so_snd, off, (int) len, ((caddr_t)(m)->m_hdr.mh_data) + hdrlen);
//       m->m_hdr.mh_len += len;
//     
//     
//     
//     
//     
//     
//     
//       if (off + len == so->so_snd.sb_cc)
//        flags |= 0x08;
//      } else {
//       m = m_get(so->slirp);
//       if (m == 
//               ((void *)0)
//                   ) {
//        error = 1;
//        goto out;
//       }
//       m->m_hdr.mh_data += (2 + 14 + 40);
//       m->m_hdr.mh_len = hdrlen;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2762 {
// Port of the mbuf-grabbing piece of tcp_output (slirp/tcp_output.c lines 292-320).

use core::ffi::{c_char, c_int, c_long, c_uint};

#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
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
pub union Mdat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub m_dat: Mdat,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut u8,
    pub so_urgc: c_int,
    pub so_faddr: u32,
    pub so_laddr: u32,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut u8,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

extern "C" {
    fn m_get(slirp: *mut Slirp) -> *mut Mbuf;
    fn sbcopy(sb: *mut Sbuf, off: c_int, len: c_int, to: *mut c_char);
}

// Returns 0 on success. Returns nonzero when m_get() failed; in that case
// *error has been set to 1 and the caller must take the "goto out" path.
#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2762(
    len: c_long,
    m_out: *mut *mut Mbuf,
    so: *mut Socket,
    error: *mut c_int,
    hdrlen: c_uint,
    off: c_int,
    flags: *mut c_int,
) -> c_int {
    if len != 0 {
        let m = m_get((*so).slirp);
        *m_out = m;
        if m.is_null() {
            *error = 1;
            return 1;
        }
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_add((2 + 14 + 40) as usize);
        (*m).m_hdr.mh_len = hdrlen as c_int;

        sbcopy(
            &mut (*so).so_snd,
            off,
            len as c_int,
            (*m).m_hdr.mh_data.wrapping_add(hdrlen as usize),
        );
        (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_add(len as c_int);

        if (off as c_long).wrapping_add(len) == (*so).so_snd.sb_cc as c_long {
            *flags |= 0x08;
        }
    } else {
        let m = m_get((*so).slirp);
        *m_out = m;
        if m.is_null() {
            *error = 1;
            return 1;
        }
        (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.wrapping_add((2 + 14 + 40) as usize);
        (*m).m_hdr.mh_len = hdrlen as c_int;
    }
    0
}
}

// ── `ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data); memcpy((caddr_t)ti, &tp->t_template, sizeof (stru…` · slirp/tcp_output.c:322-324 · chunk 2763 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);
//     
//      memcpy((caddr_t)ti, &tp->t_template, sizeof (struct tcpiphdr));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2763 {
// Port of the TARGET from tcp_output(): fill in the TCP/IP header of the
// outgoing packet by copying tp->t_template into the mbuf data area.
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
pub struct IpOvly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
pub struct TcpHdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
pub struct TcpIpHdr {
    pub ti_i: IpOvly,
    pub ti_t: TcpHdr,
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
pub union Mdat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut c_void,
    pub m_dat: Mdat,
}

// The part of struct tcpcb this piece touches: the t_template field.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut TcpIpHdr,
    pub seg_prev: *mut TcpIpHdr,
    pub t_state: i16,
    pub t_timer: [i16; 4],
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16,
    pub t_force: c_char,
    pub t_flags: u16,
    pub t_template: TcpIpHdr,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2763(
    tp: *mut Tcpcb,
    m: *mut Mbuf,
) -> *mut TcpIpHdr {
    let ti: *mut TcpIpHdr = (*(m)).m_hdr.mh_data as *mut TcpIpHdr;
    let src: *const u8 = &(*(tp)).t_template as *const TcpIpHdr as *const u8;
    let dst: *mut u8 = ti as *mut u8;
    let mut i: usize = 0;
    let n: usize = core::mem::size_of::<TcpIpHdr>();
    while i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
    ti
}
}

// ── `if (flags & 0x01 && tp->t_flags & 0x0010 && tp->snd_nxt == tp->snd_max)` · slirp/tcp_output.c:331-333 · chunk 2764 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & 0x01 && tp->t_flags & 0x0010 &&
//          tp->snd_nxt == tp->snd_max)
//       tp->snd_nxt--;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2764 {
// Fragment of slirp tcp_output(): rewind snd_nxt when resending a FIN.
#![allow(dead_code)]
use core::ffi::{c_int, c_void};

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

#[repr(C, packed)]
pub struct InAddr {
    pub s_addr: u32,
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
    pub th_off: u8, // bitfields th_x2:4, th_off:4
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
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

const TH_FIN: c_int = 0x01;
const TF_SENTFIN: c_int = 0x0010;

#[no_mangle]
pub extern "C" fn tcp_output__part2764(tp: *mut Tcpcb, flags: c_int) {
    unsafe {
        if flags & TH_FIN != 0
            && ((*tp).t_flags as c_int) & TF_SENTFIN != 0
            && (*tp).snd_nxt == (*tp).snd_max
        {
            let cur = (*tp).snd_nxt;
            (*tp).snd_nxt = cur.wrapping_sub(1);
        }
    }
}
}

// ── `if (len || (flags & (0x02|0x01)) || tp->t_timer[1]) … else` · slirp/tcp_output.c:347-350 · chunk 2765 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len || (flags & (0x02|0x01)) || tp->t_timer[1])
//       ti->ti_t.th_seq = htonl(tp->snd_nxt);
//      else
//       ti->ti_t.th_seq = htonl(tp->snd_max);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2765 {
use core::ffi::{c_int, c_long, c_short};

extern "C" {
    fn htonl(hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn tcp_output__part2765(
    th_seq: *mut u32,
    len: c_long,
    flags: c_int,
    snd_nxt: u32,
    snd_max: u32,
    t_timer1: c_short,
) {
    let seq = if len != 0 || (flags & (0x02 | 0x01)) != 0 || t_timer1 != 0 {
        snd_nxt
    } else {
        snd_max
    };
    unsafe {
        *th_seq = htonl(seq);
    }
}
}

// ── `ti->ti_t.th_ack = htonl(tp->rcv_nxt);` · slirp/tcp_output.c:351-351 · chunk 2766 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_t.th_ack = htonl(tp->rcv_nxt);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2766 {
// Port of: ti->ti_t.th_ack = htonl(tp->rcv_nxt);
// The C caller passes the sequence number by value and the destination
// field by address, so no C struct layout has to be mirrored here.

extern "C" {
    // uint32_t htonl(uint32_t __hostlong);
    fn htonl(__hostlong: u32) -> u32;
}

#[no_mangle]
pub extern "C" fn tcp_output__part2766(seq: u32, ack_field: *mut u32) {
    if ack_field.is_null() {
        return;
    }
    unsafe {
        *ack_field = htonl(seq);
    }
}
}

// ── `if (optlen)` · slirp/tcp_output.c:352-355 · chunk 2767 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (optlen) {
//       memcpy((caddr_t)(ti + 1), (caddr_t)opt, optlen);
//       ti->ti_t.th_off = (sizeof (struct tcphdr) + optlen) >> 2;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2767 {
// Port of the TCP option copy in slirp's tcp_output():
//   if (optlen) {
//       memcpy((caddr_t)(ti + 1), (caddr_t)opt, optlen);
//       ti->ti_t.th_off = (sizeof (struct tcphdr) + optlen) >> 2;
//   }
#![allow(non_camel_case_types)]

use core::ffi::{c_uint, c_void};

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

// u_int th_x2:4, th_off:4;  (little-endian storage layout)
#[repr(C)]
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

#[repr(C)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2767(
    ti: *mut tcpiphdr,
    optlen: c_uint,
    opt: *mut u8,
) {
    if optlen == 0 {
        return;
    }
    // (caddr_t)(ti + 1)
    let dst = (ti as *mut u8).add(core::mem::size_of::<tcpiphdr>());
    core::ptr::copy_nonoverlapping(opt as *const u8, dst, optlen as usize);
    // ti->ti_t.th_off = (sizeof (struct tcphdr) + optlen) >> 2;
    let off_ptr = core::ptr::addr_of_mut!((*ti).ti_t.th_x2_off);
    let val = ((core::mem::size_of::<tcphdr>() as c_uint).wrapping_add(optlen) >> 2) as u8;
    let cur = off_ptr.read();
    off_ptr.write((cur & 0x0f) | ((val & 0x0f) << 4));
}
}

// ── `ti->ti_t.th_flags = flags;` · slirp/tcp_output.c:356-356 · chunk 2768 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_t.th_flags = flags;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2768 {
// Portion of tcp_output: store the TCP flags byte into the header.
#[no_mangle]
pub extern "C" fn tcp_output__part2768(th_flags: *mut u8, flags: core::ffi::c_int) {
    unsafe {
        *th_flags = flags as u8;
    }
}
}

// ── `if (win < (long)(so->so_rcv.sb_datalen / 4) && win < (long)tp->t_maxseg)` · slirp/tcp_output.c:361-362 · chunk 2769 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (win < (long)(so->so_rcv.sb_datalen / 4) && win < (long)tp->t_maxseg)
//       win = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2769 {
// Rewrite of slirp/tcp_output.c lines 361-362
//   if (win < (long)(so->so_rcv.sb_datalen / 4) && win < (long)tp->t_maxseg)
//     win = 0;
use core::ffi::{c_long, c_uint, c_ushort};

#[no_mangle]
pub extern "C" fn tcp_output__part2769(
    win: c_long,
    sb_datalen: c_uint,
    t_maxseg: c_ushort,
) -> c_long {
    // sb_datalen / 4 is unsigned int arithmetic, then converted (zero extended)
    // to long, exactly like the C cast.
    let quarter: c_long = (sb_datalen / 4) as c_long;
    if win < quarter && win < t_maxseg as c_long {
        0
    } else {
        win
    }
}
}

// ── `if (win > (long)65535 << tp->rcv_scale)` · slirp/tcp_output.c:363-364 · chunk 2770 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (win > (long)65535 << tp->rcv_scale)
//       win = (long)65535 << tp->rcv_scale;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2770 {
// Ported piece of tcp_output: clamp the window to (long)65535 << tp->rcv_scale.
use core::ffi::{c_long, c_uchar};

#[no_mangle]
pub extern "C" fn tcp_output__part2770(win: c_long, rcv_scale: c_uchar) -> c_long {
    // (long)65535 << tp->rcv_scale  (shift count semantics of C on the target:
    // only the low 6 bits of the count are used for a 64-bit operand).
    let limit: c_long = 65535i64.wrapping_shl(rcv_scale as u32);
    if win > limit {
        limit
    } else {
        win
    }
}
}

// ── `if (win < (long)(tp->rcv_adv - tp->rcv_nxt)` · slirp/tcp_output.c:365-366 · chunk 2771 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (win < (long)(tp->rcv_adv - tp->rcv_nxt))
//       win = (long)(tp->rcv_adv - tp->rcv_nxt);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2771 {
// Port of slirp tcp_output.c lines 365-366:
//   if (win < (long)(tp->rcv_adv - tp->rcv_nxt))
//     win = (long)(tp->rcv_adv - tp->rcv_nxt);
use core::ffi::c_long;

#[no_mangle]
pub extern "C" fn tcp_output__part2771(win: c_long, tp_rcv_adv: u32, tp_rcv_nxt: u32) -> c_long {
    // tcp_seq is uint32_t; the subtraction wraps in 32 bits and is then
    // converted to long.
    let adv: c_long = tp_rcv_adv.wrapping_sub(tp_rcv_nxt) as c_long;
    if win < adv {
        adv
    } else {
        win
    }
}
}

// ── `ti->ti_t.th_win = htons((uint16_t) (win>>tp->rcv_scale));` · slirp/tcp_output.c:367-367 · chunk 2772 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_t.th_win = htons((uint16_t) (win>>tp->rcv_scale));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2772 {
// Port of the single statement:
//   ti->ti_t.th_win = htons((uint16_t)(win >> tp->rcv_scale));

use core::ffi::{c_long, c_void};

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ipovly (packed in C)
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: *mut c_void, // struct mbuf_ptr { struct mbuf *mptr; }
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub th_x2_off: u32, // u_int th_x2:4, th_off:4;
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

// struct tcpiphdr
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2772(ti: *mut Tcpiphdr, win: c_long, rcv_scale: u8) {
    unsafe {
        // (uint16_t)(win >> tp->rcv_scale), arithmetic shift on signed long
        let shifted = win.wrapping_shr(rcv_scale as u32);
        (*ti).ti_t.th_win = htons(shifted as u16);
    }
}
}

// ── `if ((int)((tp->snd_up)-(tp->snd_una)) > 0) … else` · slirp/tcp_output.c:369-379 · chunk 2773 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (((int)((tp->snd_up)-(tp->snd_una)) > 0)) {
//       ti->ti_t.th_urp = htons((uint16_t)(tp->snd_up - ntohl(ti->ti_t.th_seq)));
//       ti->ti_t.th_flags |= 0x20;
//      } else
//     
//     
//     
//     
//     
//     
//       tp->snd_up = tp->snd_una;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2773 {
// Port of the urgent-pointer handling fragment of tcp_output().

extern "C" {
    fn htons(x: u16) -> u16;
    fn ntohl(x: u32) -> u32;
}

#[repr(C, packed)]
struct MbufPtr {
    mptr: *mut core::ffi::c_void,
}

#[repr(C, packed)]
struct InAddr {
    s_addr: u32,
}

#[repr(C, packed)]
struct Ipovly {
    ih_mbuf: MbufPtr,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: InAddr,
    ih_dst: InAddr,
}

#[repr(C)]
struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    th_x2_off: u32, // u_int th_x2:4, th_off:4; storage unit of unsigned int
    th_flags: u8,
    _pad: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2773(
    ti: *mut core::ffi::c_void,
    snd_up: *mut u32,
    snd_una: u32,
) {
    let ti = ti as *mut Tcpiphdr;
    let up = core::ptr::read(snd_up);
    if up.wrapping_sub(snd_una) as i32 > 0 {
        let seq = ntohl(core::ptr::read(&(*ti).ti_t.th_seq));
        let urp = up.wrapping_sub(seq) as u16;
        core::ptr::write(&mut (*ti).ti_t.th_urp, htons(urp));
        let flags = core::ptr::read(&(*ti).ti_t.th_flags);
        core::ptr::write(&mut (*ti).ti_t.th_flags, flags | 0x20);
    } else {
        core::ptr::write(snd_up, snd_una);
    }
}
}

// ── `if (len + optlen)` · slirp/tcp_output.c:385-387 · chunk 2774 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len + optlen)
//       ti->ti_i.ih_len = htons((uint16_t)(sizeof (struct tcphdr) +
//           optlen + len));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2774 {
// Ported piece of tcp_output(): fill in the extended-header TCP length
// (ti->ti_i.ih_len) when the outgoing segment has payload or options.
use core::ffi::{c_long, c_uint, c_void};

// struct ipovly { struct mbuf_ptr ih_mbuf; uint8_t ih_x1; uint8_t ih_pr;
//                 uint16_t ih_len; struct in_addr ih_src, ih_dst; } __attribute__((packed));
#[repr(C, packed)]
pub struct Ipovly {
    ih_mbuf: *mut c_void,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: u32,
    ih_dst: u32,
}

// struct tcphdr (unpacked): bitfields th_x2:4 / th_off:4 share one u_int slot.
#[repr(C)]
pub struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    th_x2_off: u32,
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; }
#[repr(C)]
pub struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

// sizeof (struct tcphdr) as seen by the C translation unit.
const TCPHDR_SIZE: u64 = 24;

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2774(ti: *mut Tcpiphdr, len: c_long, optlen: c_uint) {
    // C: if (len + optlen)   -- computed in long arithmetic
    if len.wrapping_add(optlen as c_long) != 0 {
        // C: (uint16_t)(sizeof (struct tcphdr) + optlen + len)
        // computed in unsigned long arithmetic, then truncated to 16 bits,
        // and passed through htons().
        let total: u64 = TCPHDR_SIZE
            .wrapping_add(optlen as u64)
            .wrapping_add(len as u64);
        let value: u16 = (total as u16).to_be();
        (*ti).ti_i.ih_len = value;
    }
}
}

// ── `ti->ti_t.th_sum = cksum(m, (int)(hdrlen + len));` · slirp/tcp_output.c:388-388 · chunk 2775 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_t.th_sum = cksum(m, (int)(hdrlen + len));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2775 {
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

/* struct ipovly is __attribute__((packed)) in C */
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: *mut c_void,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

/* struct tcphdr; the `u_int th_x2:4, th_off:4;` bitfield pair lives in one
 * 4-byte storage unit. */
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
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

/* opaque: only ever handed back to C */
#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
}

extern "C" {
    fn cksum(m: *mut Mbuf, len: c_int) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_output__part2775(
    ti: *mut Tcpiphdr,
    m: *mut Mbuf,
    hdrlen: c_uint,
    len: c_long,
) {
    /* C: (int)(hdrlen + len) -- unsigned int promoted to long, then truncated */
    let total = (hdrlen as c_long).wrapping_add(len) as c_int;
    let sum = cksum(m, total);
    (*ti).ti_t.th_sum = sum as u16;
}
}

// ── `if (tp->t_force == 0 || tp->t_timer[1] == 0) … else` · slirp/tcp_output.c:394-439 · chunk 2776 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_force == 0 || tp->t_timer[1] == 0) {
//       tcp_seq startseq = tp->snd_nxt;
//     
//     
//     
//     
//       if (flags & (0x02|0x01)) {
//        if (flags & 0x02)
//         tp->snd_nxt++;
//        if (flags & 0x01) {
//         tp->snd_nxt++;
//         tp->t_flags |= 0x0010;
//        }
//       }
//       tp->snd_nxt += len;
//       if (((int)((tp->snd_nxt)-(tp->snd_max)) > 0)) {
//        tp->snd_max = tp->snd_nxt;
//     
//     
//     
//     
//        if (tp->t_rtt == 0) {
//         tp->t_rtt = 1;
//         tp->t_rtseq = startseq;
//        }
//       }
//       if (tp->t_timer[0] == 0 &&
//           tp->snd_nxt != tp->snd_una) {
//        tp->t_timer[0] = tp->t_rxtcur;
//        if (tp->t_timer[1]) {
//         tp->t_timer[1] = 0;
//         tp->t_rxtshift = 0;
//        }
//       }
//      } else
//       if (((int)((tp->snd_nxt + len)-(tp->snd_max)) > 0))
//        tp->snd_max = tp->snd_nxt + len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2776 {
// Port of part of tcp_output() from slirp/tcp_output.c (sequence-advance block).

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C, align(4))]
#[derive(Copy, Clone)]
struct Tcpiphdr {
    // struct tcpiphdr is 44 bytes (packed struct ipovly + struct tcphdr), align 4
    _bytes: [u8; 44],
}

#[repr(C)]
struct Tcpcb {
    seg_next: *mut c_void,
    seg_prev: *mut c_void,
    t_state: i16,
    t_timer: [i16; 4],
    t_rxtshift: i16,
    t_rxtcur: i16,
    t_dupacks: i16,
    t_maxseg: u16,
    t_force: c_char,
    t_flags: u16,
    t_template: Tcpiphdr,
    t_socket: *mut c_void,
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

#[no_mangle]
pub extern "C" fn tcp_output__part2776(tp: *mut Tcpcb, flags: c_int, len: c_long) {
    if tp.is_null() {
        return;
    }
    let tp: &mut Tcpcb = unsafe { &mut *tp };

    if tp.t_force == 0 || tp.t_timer[1] == 0 {
        let startseq: u32 = tp.snd_nxt;

        if (flags & (0x02 | 0x01)) != 0 {
            if (flags & 0x02) != 0 {
                tp.snd_nxt = tp.snd_nxt.wrapping_add(1);
            }
            if (flags & 0x01) != 0 {
                tp.snd_nxt = tp.snd_nxt.wrapping_add(1);
                tp.t_flags |= 0x0010;
            }
        }
        tp.snd_nxt = ((tp.snd_nxt as i64).wrapping_add(len)) as u32;
        if (tp.snd_nxt.wrapping_sub(tp.snd_max) as i32) > 0 {
            tp.snd_max = tp.snd_nxt;

            if tp.t_rtt == 0 {
                tp.t_rtt = 1;
                tp.t_rtseq = startseq;
            }
        }
        if tp.t_timer[0] == 0 && tp.snd_nxt != tp.snd_una {
            tp.t_timer[0] = tp.t_rxtcur;
            if tp.t_timer[1] != 0 {
                tp.t_timer[1] = 0;
                tp.t_rxtshift = 0;
            }
        }
    } else {
        let v: i64 = (tp.snd_nxt as i64).wrapping_add(len);
        let diff: i64 = v.wrapping_sub(tp.snd_max as i64);
        if (diff as i32) > 0 {
            tp.snd_max = v as u32;
        }
    }
}
}

// ── `m->m_hdr.mh_len = hdrlen + len;` · slirp/tcp_output.c:447-447 · chunk 2777 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m->m_hdr.mh_len = hdrlen + len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2777 {
// Rust rewrite of: m->m_hdr.mh_len = hdrlen + len;
// hdrlen is `unsigned` (unsigned int), len is `long`; the sum is computed in
// `long` (usual arithmetic conversions) and then truncated into the int field
// mh_len.

use core::ffi::{c_char, c_int, c_long, c_uint};

#[repr(C)]
pub struct M_hdr {
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
    pub m_hdr: M_hdr,
}

#[no_mangle]
pub extern "C" fn tcp_output__part2777(m: *mut Mbuf, hdrlen: c_uint, len: c_long) {
    unsafe {
        if m.is_null() {
            return;
        }
        let total: c_long = (hdrlen as c_long).wrapping_add(len);
        (*m).m_hdr.mh_len = total as c_int;
    }
}
}

// ── `{ … }` · slirp/tcp_output.c:449-457 · chunk 2778 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//     
//      ((struct ip *)ti)->ip_len = m->m_hdr.mh_len;
//     
//      ((struct ip *)ti)->ip_ttl = 64;
//      ((struct ip *)ti)->ip_tos = so->so_iptos;
//     
//      error = ip_output(so, m);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2778 {
// Port of the "fill in IP header fields and hand the mbuf to ip_output"
// block of tcp_output().  Everything else stays in C.

use core::ffi::{c_char, c_int, c_uint, c_void};

// struct in_addr { in_addr_t s_addr; }   (in_addr_t == uint32_t)
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ip { u_int ip_hl:4, ip_v:4; uint8_t ip_tos; uint16_t ip_len; ... }
// __attribute__((packed)): the two 4-bit fields share the first byte, all
// further members follow on byte boundaries, total size 20.
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

// struct m_hdr
#[repr(C)]
pub struct MbufHeader {
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

// struct mbuf { struct m_hdr m_hdr; Slirp *slirp; union M_dat M_dat; }
#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHeader,
    pub slirp: *mut c_void,
    pub m_dat: [u8; 8], // union { char m_dat_[1]; char *m_ext_; }
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

// struct socket (only so_iptos is touched; the links to Slirp/tcpcb are
// opaque pointers, which keeps every offset identical to the C layout).
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
    fn ip_output(so: *mut Socket, m: *mut Mbuf) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_output__part2778(
    ti: *mut c_void,
    m: *mut Mbuf,
    so: *mut Socket,
) -> c_int {
    unsafe {
        let ip = ti as *mut Ip;

        let len = (*m).m_hdr.mh_len; // int -> uint16_t truncation below
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_len), len as u16);
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_ttl), 64u8);

        let tos = (*so).so_iptos;
        core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ip).ip_tos), tos);

        ip_output(so, m)
    }
}
}

// ── `if (error)` · slirp/tcp_output.c:458-461 · chunk 2779 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (error) {
//     out:
//       return (error);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2779 {
// Port of the tail error-check / `out:` label block of tcp_output().
// Reads the local `error`, returns it unchanged (the C side keeps the
// `if (error)` test and the `out:` label so the existing gotos still work).

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2779(error: c_int) -> c_int {
    error
}
}

// ── `if (win > 0 && ((int)((tp->rcv_nxt+win)-(tp->rcv_adv)) > 0)` · slirp/tcp_output.c:469-470 · chunk 2780 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (win > 0 && ((int)((tp->rcv_nxt+win)-(tp->rcv_adv)) > 0))
//       tp->rcv_adv = tp->rcv_nxt + win;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2780 {
// Port of the advance-receive-window check from tcp_output().

#[no_mangle]
pub extern "C" fn tcp_output__part2780(
    win: core::ffi::c_long,
    rcv_nxt: *mut u32,
    rcv_adv: *mut u32,
) {
    unsafe {
        let nxt = *rcv_nxt;
        let adv = *rcv_adv;
        // (int)((tp->rcv_nxt + win) - tp->rcv_adv)
        let diff = (nxt as i64).wrapping_add(win as i64).wrapping_sub(adv as i64) as i32;
        if win > 0 && diff > 0 {
            *rcv_adv = (nxt as i64).wrapping_add(win as i64) as u32;
        }
    }
}
}

// ── `tp->last_ack_sent = tp->rcv_nxt; tp->t_flags &= ~(0x0001|0x0002);` · slirp/tcp_output.c:471-472 · chunk 2781 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tp->last_ack_sent = tp->rcv_nxt;
//      tp->t_flags &= ~(0x0001|0x0002);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2781 {
// Port of tcp_output fragment: update last_ack_sent and clear TF_ACKNOW|TF_DELACK.

use core::ffi::{c_char, c_short, c_ushort, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut c_void,
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
    // u_int th_x2:4, th_off:4;
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
    pub s: i32,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut tcpiphdr,
    pub so_urgc: i32,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: i32,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: u32,
    pub so_queued: i32,
    pub so_nqueued: i32,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut tcpiphdr,
    pub seg_prev: *mut tcpiphdr,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
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

#[no_mangle]
pub extern "C" fn tcp_output__part2781(tp: *mut tcpcb) {
    unsafe {
        if tp.is_null() {
            return;
        }
        let tp = &mut *tp;
        tp.last_ack_sent = tp.rcv_nxt;
        tp.t_flags &= !(0x0001u16 | 0x0002u16);
    }
}
}

// ── `if (sendalot)` · slirp/tcp_output.c:473-474 · chunk 2782 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (sendalot)
//       goto again;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2782 {
// Portion of tcp_output(): decide whether another segment must be sent
// (i.e. whether the C code should `goto again`).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2782(sendalot: c_int) -> c_int {
    // C semantics: `if (sendalot) goto again;`
    if sendalot != 0 {
        1
    } else {
        0
    }
}
}

// ── `return (0);` · slirp/tcp_output.c:476-476 · chunk 2783 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2783 {
// Tail of tcp_output: the trailing unconditional `return (0);`.
// It reads no locals and produces the function's success value.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_output__part2783() -> c_int {
    0
}
}

// ── `tcp_setpersist` · slirp/tcp_output.c:479-492 · chunk 2784 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     tcp_setpersist(struct tcpcb *tp)
//     {
//         int t = ((tp->t_srtt >> 2) + tp->t_rttvar) >> 1;
//     
//     
//     
//     
//      { (tp->t_timer[1]) = (t * tcp_backoff[tp->t_rxtshift]); if ((tp->t_timer[1]) < (( 5*2))) (tp->t_timer[1]) = (( 5*2)); else if ((tp->t_timer[1]) > (( 60*2))) (tp->t_timer[1]) = (( 60*2)); }
//     
//                                       ;
//      if (tp->t_rxtshift < 12)
//       tp->t_rxtshift++;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2784 {
// Rust rewrite of slirp's tcp_setpersist(struct tcpcb *tp)

use core::ffi::{c_char, c_int, c_short, c_ushort};

#[repr(C)]
pub struct socket {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

#[repr(C, packed)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
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
    // u_int th_x2:4, th_off:4;
    pub th_x2_off: u8,
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
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
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
    static tcp_backoff: c_int;
}

const TCPT_PERSIST: usize = 1;
const TCPTV_PERSMIN: c_int = 5 * 2;
const TCPTV_PERSMAX: c_int = 60 * 2;
const TCP_MAXRXTSHIFT: c_short = 12;

#[no_mangle]
pub unsafe extern "C" fn tcp_setpersist(tp: *mut tcpcb) {
    let srtt = (*tp).t_srtt as c_int;
    let rttvar = (*tp).t_rttvar as c_int;
    let t: c_int = ((srtt >> 2).wrapping_add(rttvar)) >> 1;

    let shift = (*tp).t_rxtshift as c_int;
    let backoff = *core::ptr::addr_of!(tcp_backoff).offset(shift as isize);

    // TCPT_RANGESET(tp->t_timer[TCPT_PERSIST], t * tcp_backoff[...], MIN, MAX)
    let mut val: c_short = t.wrapping_mul(backoff) as c_short;
    if (val as c_int) < TCPTV_PERSMIN {
        val = TCPTV_PERSMIN as c_short;
    } else if (val as c_int) > TCPTV_PERSMAX {
        val = TCPTV_PERSMAX as c_short;
    }
    (*tp).t_timer[TCPT_PERSIST] = val;

    if (*tp).t_rxtshift < TCP_MAXRXTSHIFT {
        (*tp).t_rxtshift = (*tp).t_rxtshift.wrapping_add(1);
    }
}
}

// ── `{ … }` · slirp/tcp_output.c:487-487 · chunk 2785 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     { (tp->t_timer[1]) = (t * tcp_backoff[tp->t_rxtshift]); if ((tp->t_timer[1]) < (( 5*2))) (tp->t_timer[1]) = (( 5*2)); else if ((tp->t_timer[1]) > (( 60*2))) (tp->t_timer[1]) = (( 60*2)); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2785 {
// Port of a piece of tcp_setpersist: set tp->t_timer[1] from the backoff table
// and clamp it to [5*2, 60*2] (in TCP timer ticks).
use core::ffi::{c_int, c_short, c_void};

// Partial mirror of `struct tcpcb`: only the fields this piece touches, in
// declaration order, so the offsets of t_timer/t_rxtshift match the C layout.
#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
}

extern "C" {
    static tcp_backoff: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_setpersist__part2785(tp: *mut tcpcb, t: c_int) {
    let shift = (*tp).t_rxtshift as isize;
    let backoff: c_int = (core::ptr::addr_of!(tcp_backoff) as *const c_int)
        .offset(shift)
        .read();
    let prod: c_int = t.wrapping_mul(backoff);
    // Assignment to a `short` field truncates before the comparisons.
    let mut val: c_short = prod as c_short;
    if (val as c_int) < 5 * 2 {
        val = (5 * 2) as c_short;
    } else if (val as c_int) > 60 * 2 {
        val = (60 * 2) as c_short;
    }
    core::ptr::addr_of_mut!((*tp).t_timer)
        .cast::<c_short>()
        .add(1)
        .write(val);
}
}

// ── `;` · slirp/tcp_output.c:489-489 · chunk 2786 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2786 {
// Placeholder piece: the original C target was an empty statement (`;`).
// The exported function keeps the same (empty) behaviour.

#[no_mangle]
pub extern "C" fn tcp_setpersist__part2786() {}
}

// ── `if (tp->t_rxtshift < 12)` · slirp/tcp_output.c:490-491 · chunk 2787 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_rxtshift < 12)
//       tp->t_rxtshift++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2787 {
// Rust port of the tail of tcp_setpersist(): advance the retransmit backoff shift.
use core::ffi::{c_short, c_void};

// Mirror of the leading fields of `struct tcpcb` up to `t_rxtshift`, in order,
// so that the field offsets match the C layout.
#[repr(C)]
struct TcpcbPrefix {
    seg_next: *mut c_void,
    seg_prev: *mut c_void,
    t_state: c_short,
    t_timer: [c_short; 4],
    t_rxtshift: c_short,
}

#[no_mangle]
pub extern "C" fn tcp_setpersist__part2787(tp: *mut c_void) {
    if tp.is_null() {
        return;
    }
    let tp = tp as *mut TcpcbPrefix;
    unsafe {
        let shift = (*tp).t_rxtshift;
        if (shift as i32) < 12 {
            (*tp).t_rxtshift = (shift as i32 + 1) as c_short;
        }
    }
}
}

