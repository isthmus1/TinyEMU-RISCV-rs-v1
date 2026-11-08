//! From `slirp/tcp_input.c`.

// ── `tcp_reass` · slirp/tcp_input.c:104-209 · chunk 2660 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int
//     tcp_reass(register struct tcpcb *tp, register struct tcpiphdr *ti,
//               struct mbuf *m)
//     {
//      register struct tcpiphdr *q;
//      struct socket *so = tp->t_socket;
//      int flags;
//     
//     
//     
//     
//     
//             if (ti == 
//                      ((void *)0)
//                          )
//       goto present;
//     
//     
//     
//     
//      for (q = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink))); !(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp));
//                 q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink))))
//       if (((int)((q->ti_t.th_seq)-(ti->ti_t.th_seq)) > 0))
//        break;
//     
//     
//     
//     
//     
//     
//      if (!(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       register int i;
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)));
//     
//       i = q->ti_t.th_seq + q->ti_i.ih_len - ti->ti_t.th_seq;
//       if (i > 0) {
//        if (i >= ti->ti_i.ih_len) {
//         m_free(m);
//     
//     
//     
//     
//     
//     
//         goto present;
//        }
//        m_adj(m, i);
//        ti->ti_i.ih_len -= i;
//        ti->ti_t.th_seq += i;
//       }
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//      }
//      ti->ti_i.ih_mbuf.mptr = m;
//     
//     
//     
//     
//     
//      while (!(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       register int i = (ti->ti_t.th_seq + ti->ti_i.ih_len) - q->ti_t.th_seq;
//       if (i <= 0)
//        break;
//       if (i < q->ti_i.ih_len) {
//        q->ti_t.th_seq += i;
//        q->ti_i.ih_len -= i;
//        m_adj(q->ti_i.ih_mbuf.mptr, i);
//        break;
//       }
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       m = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)))->ti_i.ih_mbuf.mptr;
//       slirp_remque(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
//       m_free(m);
//      }
//     
//     
//     
//     
//      slirp_insque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))), ((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
//     
//     present:
//     
//     
//     
//     
//      if (!((tp->t_state) >= 4))
//       return (0);
//      ti = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));
//      if ((((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))) == (struct qlink*)(tp)) || ti->ti_t.th_seq != tp->rcv_nxt)
//       return (0);
//      if (tp->t_state == 3 && ti->ti_i.ih_len)
//       return (0);
//      do {
//       tp->rcv_nxt += ti->ti_i.ih_len;
//       flags = ti->ti_t.th_flags & 0x01;
//       slirp_remque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))));
//       m = ti->ti_i.ih_mbuf.mptr;
//       ti = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       if (so->so_state & 0x010)
//        m_free(m);
//       else {
//        if (so->so_emu) {
//         if (tcp_emu(so,m)) sbappend(so, m);
//        } else
//         sbappend(so, m);
//       }
//      } while (ti != (struct tcpiphdr *)tp && ti->ti_t.th_seq == tp->rcv_nxt);
//      return (flags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2660 {
// Rust port of slirp's tcp_reass (slirp/tcp_input.c).
// Mirrors the C layout of the structures it walks through.

use core::ffi::c_void;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// opaque: only ever used through pointers
#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
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
    // `u_int th_x2:4, th_off:4;` (8 bits of a 4-byte aligned unit)
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
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: i32,
    pub slirp: *mut c_void,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
    pub so_urgc: i32,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: i32,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: u32,
    pub so_queued: i32,
    pub so_nqueued: i32,
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

extern "C" {
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: i32);
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> i32;
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    fn slirp_remque(a: *mut c_void);
}

#[inline]
unsafe fn tcpiphdr2qlink(t: *mut Tcpiphdr) -> *mut Qlink {
    (t as *mut u8).offset(-(core::mem::size_of::<Qlink>() as isize)) as *mut Qlink
}

#[inline]
unsafe fn qlink2tcpiphdr(q: *mut Qlink) -> *mut Tcpiphdr {
    (q as *mut u8).offset(core::mem::size_of::<Qlink>() as isize) as *mut Tcpiphdr
}

#[inline]
unsafe fn tcpiphdr_next(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).next as *mut Qlink)
}

#[inline]
unsafe fn tcpiphdr_prev(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).prev as *mut Qlink)
}

#[inline]
unsafe fn tcpfrag_list_first(tp: *mut Tcpcb) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tp).seg_next as *mut Qlink)
}

#[inline]
unsafe fn tcpfrag_list_end(f: *mut Tcpiphdr, tp: *mut Tcpcb) -> bool {
    tcpiphdr2qlink(f) == (tp as *mut Qlink)
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reass(
    tp: *mut Tcpcb,
    ti_arg: *mut Tcpiphdr,
    m_arg: *mut Mbuf,
) -> i32 {
    let so: *mut Socket = (*tp).t_socket;
    let mut m: *mut Mbuf = m_arg;
    let mut ti: *mut Tcpiphdr = ti_arg;
    let mut flags: i32 = 0;

    'present: {
        if ti.is_null() {
            break 'present;
        }

        // Find a segment which begins after this one does.
        let mut q: *mut Tcpiphdr = tcpfrag_list_first(tp);
        while !tcpfrag_list_end(q, tp) {
            let d = (*q).ti_t.th_seq.wrapping_sub((*ti).ti_t.th_seq) as i32;
            if d > 0 {
                break;
            }
            q = tcpiphdr_next(q);
        }

        // If there is a preceding segment, trim the incoming segment.
        if !tcpfrag_list_end(tcpiphdr_prev(q), tp) {
            q = tcpiphdr_prev(q);
            // conversion to int handles seq wraparound
            let i: i32 = (*q)
                .ti_t
                .th_seq
                .wrapping_add((*q).ti_i.ih_len as u32)
                .wrapping_sub((*ti).ti_t.th_seq) as i32;
            if i > 0 {
                if i >= (*ti).ti_i.ih_len as i32 {
                    m_free(m);
                    break 'present;
                }
                m_adj(m, i);
                (*ti).ti_i.ih_len = (*ti).ti_i.ih_len.wrapping_sub(i as u16);
                (*ti).ti_t.th_seq = (*ti).ti_t.th_seq.wrapping_add(i as u32);
            }
            q = tcpiphdr_next(q);
        }
        (*ti).ti_i.ih_mbuf.mptr = m;

        // Trim or dequeue completely covered succeeding segments.
        while !tcpfrag_list_end(q, tp) {
            let i: i32 = (*ti)
                .ti_t
                .th_seq
                .wrapping_add((*ti).ti_i.ih_len as u32)
                .wrapping_sub((*q).ti_t.th_seq) as i32;
            if i <= 0 {
                break;
            }
            if i < (*q).ti_i.ih_len as i32 {
                (*q).ti_t.th_seq = (*q).ti_t.th_seq.wrapping_add(i as u32);
                (*q).ti_i.ih_len = (*q).ti_i.ih_len.wrapping_sub(i as u16);
                m_adj((*q).ti_i.ih_mbuf.mptr, i);
                break;
            }
            q = tcpiphdr_next(q);
            m = (*tcpiphdr_prev(q)).ti_i.ih_mbuf.mptr;
            slirp_remque(tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void);
            m_free(m);
        }

        // Stick the new segment in its place.
        slirp_insque(
            tcpiphdr2qlink(ti) as *mut c_void,
            tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void,
        );
    }

    // Present data to user, advancing rcv_nxt through completed sequence space.
    if !((*tp).t_state >= 4) {
        return 0;
    }
    ti = tcpfrag_list_first(tp);
    if tcpfrag_list_end(ti, tp) || (*ti).ti_t.th_seq != (*tp).rcv_nxt {
        return 0;
    }
    if (*tp).t_state == 3 && (*ti).ti_i.ih_len != 0 {
        return 0;
    }
    loop {
        (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add((*ti).ti_i.ih_len as u32);
        flags = ((*ti).ti_t.th_flags & 0x01) as i32;
        slirp_remque(tcpiphdr2qlink(ti) as *mut c_void);
        m = (*ti).ti_i.ih_mbuf.mptr;
        ti = tcpiphdr_next(ti);
        if ((*so).so_state & 0x010) != 0 {
            m_free(m);
        } else {
            if (*so).so_emu != 0 {
                if tcp_emu(so, m) != 0 {
                    sbappend(so, m);
                }
            } else {
                sbappend(so, m);
            }
        }
        if !(ti != (tp as *mut Tcpiphdr) && (*ti).ti_t.th_seq == (*tp).rcv_nxt) {
            break;
        }
    }
    flags
}
}

// ── `if (ti == ((void *)0)` · slirp/tcp_input.c:116-117 · chunk 2661 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ti == 
//                      ((void *)0)
//                          )
//       goto present;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2661 {
// Helper extracted from tcp_reass: tests whether the segment pointer is NULL.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn tcp_reass__part2661(ti: *const c_void) -> c_int {
    if ti.is_null() {
        1
    } else {
        0
    }
}
}

// ── `for (q = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink); !(((struct qlink…` · slirp/tcp_input.c:122-125 · chunk 2662 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (q = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink))); !(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp));
//                 q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink))))
//       if (((int)((q->ti_t.th_seq)-(ti->ti_t.th_seq)) > 0))
//        break;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2662 {
// Port of the segment-loop searching for the insertion point in tcp_reass().
// Equivalent C:
//   for (q = first(tp); !(tcpiphdr2qlink(q) == (struct qlink*)tp);
//        q = tcpiphdr_next(q))
//     if ((int)(q->ti_t.th_seq - ti->ti_t.th_seq) > 0)
//       break;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut u8,
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
    // u_int th_x2:4, th_off:4;
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
pub struct Qlink {
    pub next: *mut u8,
    pub prev: *mut u8,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2662(
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
) -> *mut Tcpiphdr {
    let qlink_size: usize = core::mem::size_of::<Qlink>();

    let tp_qlink = (tp as *mut u8).wrapping_sub(qlink_size) as usize;

    let mut q: *mut Tcpiphdr =
        ((*tp).seg_next as *mut u8).wrapping_add(qlink_size) as *mut Tcpiphdr;

    while ((q as *mut u8).wrapping_sub(qlink_size) as usize) != tp_qlink {
        let q_seq = (*q).ti_t.th_seq;
        let ti_seq = (*ti).ti_t.th_seq;
        if (q_seq.wrapping_sub(ti_seq) as i32) > 0 {
            break;
        }
        let ql = (q as *mut u8).wrapping_sub(qlink_size) as *mut Qlink;
        let next = (*ql).next;
        q = next.wrapping_add(qlink_size) as *mut Tcpiphdr;
    }

    q
}
}

// ── `if (!(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - si…` · slirp/tcp_input.c:132-153 · chunk 2663 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       register int i;
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)));
//     
//       i = q->ti_t.th_seq + q->ti_i.ih_len - ti->ti_t.th_seq;
//       if (i > 0) {
//        if (i >= ti->ti_i.ih_len) {
//         m_free(m);
//     
//     
//     
//     
//     
//     
//         goto present;
//        }
//        m_adj(m, i);
//        ti->ti_i.ih_len -= i;
//        ti->ti_t.th_seq += i;
//       }
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2663 {
// Rust rewrite of the "merge/drop against the preceding segment" block of
// slirp's tcp_reass().  The C block both mutates the incoming segment (ti) and
// reassigns the list cursor q, and may jump to the `present:` label.

use core::ffi::{c_int, c_void};

// opaque, only ever handled through pointers
#[repr(C)]
pub struct Mbuf {
    _opaque: [u8; 0],
}

#[repr(C)]
struct Qlink {
    next: *mut c_void,
    prev: *mut c_void,
}

#[repr(C, packed)]
struct MbufPtr {
    mptr: *mut Mbuf,
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
    th_x2_off: u32, // u_int th_x2:4, th_off:4
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

extern "C" {
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: c_int);
}

const QLINK_SIZE: usize = core::mem::size_of::<Qlink>();

#[inline]
unsafe fn qlink_of(t: *mut Tcpiphdr) -> *mut Qlink {
    (t as *mut u8).sub(QLINK_SIZE) as *mut Qlink
}

#[inline]
unsafe fn tcpiphdr_of(q: *mut Qlink) -> *mut Tcpiphdr {
    (q as *mut u8).add(QLINK_SIZE) as *mut Tcpiphdr
}

#[inline]
unsafe fn get_ih_len(ti: *mut Tcpiphdr) -> u16 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*ti).ti_i.ih_len))
}

#[inline]
unsafe fn set_ih_len(ti: *mut Tcpiphdr, v: u16) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ti).ti_i.ih_len), v);
}

#[inline]
unsafe fn get_th_seq(ti: *mut Tcpiphdr) -> u32 {
    core::ptr::read_unaligned(core::ptr::addr_of!((*ti).ti_t.th_seq))
}

#[inline]
unsafe fn set_th_seq(ti: *mut Tcpiphdr, v: u32) {
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*ti).ti_t.th_seq), v);
}

/// Returns nonzero when the C code must `goto present`.
#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2663(
    tp: *mut c_void,
    ti: *mut Tcpiphdr,
    m: *mut Mbuf,
    q_out: *mut *mut Tcpiphdr,
) -> c_int {
    let mut q: *mut Tcpiphdr = *q_out;

    // tcpfrag_list_end(tcpiphdr_prev(q), tp)
    let prev_link = (*qlink_of(q)).prev;
    let prev = tcpiphdr_of(prev_link as *mut Qlink);

    if qlink_of(prev) as *mut c_void != tp {
        q = prev;
        // conversion to int (in i) handles seq wraparound
        let i: c_int =
            get_th_seq(q).wrapping_add(get_ih_len(q) as u32).wrapping_sub(get_th_seq(ti)) as c_int;
        if i > 0 {
            if i >= get_ih_len(ti) as c_int {
                m_free(m);
                *q_out = q;
                return 1;
            }
            m_adj(m, i);
            set_ih_len(ti, (get_ih_len(ti) as c_int).wrapping_sub(i) as u16);
            set_th_seq(ti, get_th_seq(ti).wrapping_add(i as u32));
        }
        q = tcpiphdr_of((*qlink_of(q)).next as *mut Qlink);
    }

    *q_out = q;
    0
}
}

// ── `ti->ti_i.ih_mbuf.mptr = m;` · slirp/tcp_input.c:154-154 · chunk 2664 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti->ti_i.ih_mbuf.mptr = m;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2664 {
// Port of the C statement: ti->ti_i.ih_mbuf.mptr = m;

#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
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
    pub ih_src: [u8; 4],
    pub ih_dst: [u8; 4],
}

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

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2664(ti: *mut Tcpiphdr, m: *mut Mbuf) {
    (*ti).ti_i.ih_mbuf.mptr = m;
}
}

// ── `while (!(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp)` · slirp/tcp_input.c:160-174 · chunk 2665 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (!(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp))) {
//       register int i = (ti->ti_t.th_seq + ti->ti_i.ih_len) - q->ti_t.th_seq;
//       if (i <= 0)
//        break;
//       if (i < q->ti_i.ih_len) {
//        q->ti_t.th_seq += i;
//        q->ti_i.ih_len -= i;
//        m_adj(q->ti_i.ih_mbuf.mptr, i);
//        break;
//       }
//       q = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       m = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink)))->ti_i.ih_mbuf.mptr;
//       slirp_remque(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
//       m_free(m);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2665 {
// Ported piece of slirp's tcp_reass(): the loop that trims/discards queued
// segments which are fully covered by the incoming segment (or trims the
// incoming one), removing them from the reassembly queue.

use core::ffi::{c_char, c_int, c_void};

const QLINK_SIZE: usize = 16; // sizeof(struct qlink)

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
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
    pub slirp: *mut c_void,
    pub m_dat: [u8; 8], // union M_dat { char m_dat_[1]; char *m_ext_; }
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
    pub th_x2_off: u32, // u_int th_x2:4, th_off:4
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

extern "C" {
    fn m_adj(m: *mut Mbuf, len: c_int);
    fn m_free(m: *mut Mbuf);
    fn slirp_remque(q: *mut c_void);
}

#[inline]
unsafe fn tcpiphdr_next(q: *mut Tcpiphdr) -> *mut Tcpiphdr {
    let ql = (q as *mut u8).sub(QLINK_SIZE) as *mut Qlink;
    ((*ql).next as *mut u8).add(QLINK_SIZE) as *mut Tcpiphdr
}

#[inline]
unsafe fn tcpiphdr_prev(q: *mut Tcpiphdr) -> *mut Tcpiphdr {
    let ql = (q as *mut u8).sub(QLINK_SIZE) as *mut Qlink;
    ((*ql).prev as *mut u8).add(QLINK_SIZE) as *mut Tcpiphdr
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2665(
    tp: *mut c_void,
    ti: *mut Tcpiphdr,
    q_ptr: *mut *mut Tcpiphdr,
    m_ptr: *mut *mut Mbuf,
) {
    let mut q: *mut Tcpiphdr = *q_ptr;

    while (q as *mut u8).sub(QLINK_SIZE) as *mut c_void != tp {
        let i: c_int = ((*ti)
            .ti_t
            .th_seq
            .wrapping_add((*ti).ti_i.ih_len as u32))
        .wrapping_sub((*q).ti_t.th_seq) as c_int;
        if i <= 0 {
            break;
        }
        let q_len: c_int = (*q).ti_i.ih_len as c_int;
        if i < q_len {
            (*q).ti_t.th_seq = (*q).ti_t.th_seq.wrapping_add(i as u32);
            (*q).ti_i.ih_len = q_len.wrapping_sub(i) as u16;
            let mb = (*q).ti_i.ih_mbuf.mptr;
            m_adj(mb, i);
            break;
        }
        q = tcpiphdr_next(q);
        let prev = tcpiphdr_prev(q);
        let mp = (*prev).ti_i.ih_mbuf.mptr;
        *m_ptr = mp;
        slirp_remque((prev as *mut u8).sub(QLINK_SIZE) as *mut c_void);
        m_free(mp);
    }

    *q_ptr = q;
}
}

// ── `slirp_insque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))), ((struct qlink*)(((char*)…` · slirp/tcp_input.c:179-179 · chunk 2666 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp_insque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))), ((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - sizeof(struct qlink)))->prev)) + sizeof(struct qlink))))) - sizeof(struct qlink))));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2666 {
// Port of the slirp_insque() call inside tcp_reass().
// The list links live in the mbuf header, immediately before the tcpiphdr:
//   qlink_of(x)  == (struct qlink *)((char *)(x) - sizeof(struct qlink))
// The C argument is
//   insque(qlink_of(ti), qlink_of(prev_tcpiphdr_of(q)))
// and qlink_of(prev_tcpiphdr_of(q)) is exactly qlink_of(q)->prev,
// because qlink_of(x) = (char *)x - sizeof(struct qlink).

use core::ffi::c_void;

#[repr(C)]
struct QLink {
    next: *mut c_void,
    prev: *mut c_void,
}

extern "C" {
    fn slirp_insque(link: *mut c_void, after: *mut c_void);
}

const QLINK_SIZE: usize = core::mem::size_of::<QLink>();

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2666(ti: *mut c_void, q: *mut c_void) {
    let ti_link = (ti as *mut u8).sub(QLINK_SIZE) as *mut QLink;
    let q_link = (q as *mut u8).sub(QLINK_SIZE) as *mut QLink;
    let after = (*q_link).prev;
    slirp_insque(ti_link as *mut c_void, after);
}
}

// ── `present: if (!((tp->t_state) >= 4)) return (0);` · slirp/tcp_input.c:181-187 · chunk 2667 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     present:
//     
//     
//     
//     
//      if (!((tp->t_state) >= 4))
//       return (0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2667 {
// Piece of tcp_reass: the `present:` label plus the state check that makes
// the surrounding C function return 0 when the connection is in a state
// earlier than ESTABLISHED (t_state < TCPS_ESTABLISHED, i.e. < 4).

use core::ffi::{c_int, c_short, c_void};

// Prefix of `struct tcpcb` (slirp/tcp_var.h). Only the fields up to
// `t_state` are needed here; they keep their natural C offsets.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
}

/// Returns 1 when the caller (`tcp_reass`) must `return (0)`, 0 otherwise.
#[no_mangle]
pub extern "C" fn tcp_reass__part2667(tp: *mut Tcpcb) -> c_int {
    let state = unsafe { (*tp).t_state };
    if state < 4 {
        1
    } else {
        0
    }
}
}

// ── `ti = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));` · slirp/tcp_input.c:188-188 · chunk 2668 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ti = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2668 {
use core::ffi::c_void;

// struct qlink { void *next, *prev; };
#[repr(C)]
struct Qlink {
    next: *mut c_void,
    prev: *mut c_void,
}

// Prefix of struct tcpcb: seg_next is its first field.
#[repr(C)]
struct Tcpcb {
    seg_next: *mut c_void,
}

#[no_mangle]
pub extern "C" fn tcp_reass__part2668(tp: *mut Tcpcb) -> *mut c_void {
    unsafe {
        let seg_next = (*tp).seg_next;
        (seg_next as *mut u8).wrapping_add(core::mem::size_of::<Qlink>()) as *mut c_void
    }
}
}

// ── `if ((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))) == (struct qlink*)(tp)) || ti->ti_t.…` · slirp/tcp_input.c:189-190 · chunk 2669 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))) == (struct qlink*)(tp)) || ti->ti_t.th_seq != tp->rcv_nxt)
//       return (0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2669 {
// Port of the TARGET from tcp_reass(): the "is the reassembly queue empty or
// out of order" test that makes the caller return 0.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
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

#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4;  (one full unsigned-int storage unit)
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

/// Returns nonzero when tcp_reass must `return (0)`, i.e. when the queue head
/// is the anchor itself or the first segment is not the expected one.
#[no_mangle]
pub extern "C" fn tcp_reass__part2669(
    ti: *mut Tcpiphdr,
    tp: *mut c_void,
    rcv_nxt: u32,
) -> c_int {
    unsafe {
        let qlink_sz = core::mem::size_of::<Qlink>();
        let head = (ti as *mut u8).wrapping_sub(qlink_sz) as *mut c_void;
        if head == tp {
            return 1;
        }
        if (*ti).ti_t.th_seq != rcv_nxt {
            return 1;
        }
        0
    }
}
}

// ── `if (tp->t_state == 3 && ti->ti_i.ih_len)` · slirp/tcp_input.c:191-192 · chunk 2670 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_state == 3 && ti->ti_i.ih_len)
//       return (0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2670 {
// Portion of tcp_reass: drop out if state is SYN_RECEIVED and the segment
// carries data.
use core::ffi::{c_int, c_short};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut core::ffi::c_void,
}

// struct ipovly is __attribute__((packed))
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// Only the leading ipovly is needed; ti_t follows it in the real struct.
#[repr(C, packed)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
}

#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: c_short,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2670(
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
) -> c_int {
    if tp.is_null() || ti.is_null() {
        return 0;
    }
    let state: c_short = (*tp).t_state;
    let ih_len: u16 = (*ti).ti_i.ih_len;
    if state == 3 && ih_len != 0 {
        1
    } else {
        0
    }
}
}

// ── `do … while (ti != (struct tcpiphdr *)tp && ti->ti_t.th_seq == tp->rcv_nxt)` · slirp/tcp_input.c:193-207 · chunk 2671 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do {
//       tp->rcv_nxt += ti->ti_i.ih_len;
//       flags = ti->ti_t.th_flags & 0x01;
//       slirp_remque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))));
//       m = ti->ti_i.ih_mbuf.mptr;
//       ti = ((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink)))->next)) + sizeof(struct qlink)));
//       if (so->so_state & 0x010)
//        m_free(m);
//       else {
//        if (so->so_emu) {
//         if (tcp_emu(so,m)) sbappend(so, m);
//        } else
//         sbappend(so, m);
//       }
//      } while (ti != (struct tcpiphdr *)tp && ti->ti_t.th_seq == tp->rcv_nxt);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2671 {
// Rust port of the "present data to user" do/while loop of tcp_reass() in
// slirp/tcp_input.c (lines 193-207). One Rust call runs the whole loop.

#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::ptr::{addr_of, read_unaligned};

// Opaque C types: only ever handled through pointers.
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
}

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

// struct tcphdr; the leading "u_int th_x2:4, th_off:4;" pair occupies one
// 4-byte unsigned int unit, which is mirrored here as a plain u32.
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

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

extern "C" {
    fn slirp_remque(a: *mut c_void);
    fn m_free(m: *mut Mbuf);
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_reass__part2671(
    ti_arg: *mut Tcpiphdr,
    m_arg: *mut Mbuf,
    so: *mut Socket,
    rcv_nxt: *mut u32,
    so_state: *mut c_int,
    so_emu: *mut u8,
    seg_head: *mut Tcpiphdr,
) -> c_int {
    let qlink_size = core::mem::size_of::<Qlink>();
    let mut ti = ti_arg;
    let mut m = m_arg;
    let mut flags: c_int = 0;

    loop {
        // tp->rcv_nxt += ti->ti_i.ih_len;
        let ih_len = read_unaligned(addr_of!((*ti).ti_i.ih_len));
        *rcv_nxt = (*rcv_nxt).wrapping_add(ih_len as u32);

        // flags = ti->ti_t.th_flags & TH_FIN;
        flags = (read_unaligned(addr_of!((*ti).ti_t.th_flags)) & 0x01) as c_int;

        // remque(tcpiphdr2qlink(ti));
        let q: *mut Qlink = (ti as *mut u8).sub(qlink_size) as *mut Qlink;
        slirp_remque(q as *mut c_void);

        // m = ti->ti_mbuf;
        m = read_unaligned(addr_of!((*ti).ti_i.ih_mbuf.mptr));

        // ti = tcpiphdr_next(ti) = qlink2tcpiphdr(tcpiphdr2qlink(ti)->next);
        let next = read_unaligned(addr_of!((*q).next));
        ti = (next as *mut u8).add(qlink_size) as *mut Tcpiphdr;

        if ((*so_state) & 0x010) != 0 {
            m_free(m);
        } else if *so_emu != 0 {
            if tcp_emu(so, m) != 0 {
                sbappend(so, m);
            }
        } else {
            sbappend(so, m);
        }

        // } while (ti != (struct tcpiphdr *)tp && ti->ti_t.th_seq == tp->rcv_nxt);
        let cont = ti != seg_head && read_unaligned(addr_of!((*ti).ti_t.th_seq)) == *rcv_nxt;
        if !cont {
            break;
        }
    }

    flags
}
}

// ── `return (flags);` · slirp/tcp_input.c:208-208 · chunk 2672 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (flags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2672 {
// Port of the tail `return (flags);` of tcp_reass.
// The TARGET only returns the local `flags` value (TH_FIN mask of the last
// dequeued segment), so this helper simply passes it through.
#[no_mangle]
pub extern "C" fn tcp_reass__part2672(flags: core::ffi::c_int) -> core::ffi::c_int {
    flags
}
}

// ── `struct ip save_ip, *ip; register struct tcpiphdr *ti; caddr_t optp = ((void *)0) ; int optlen =…` · slirp/tcp_input.c:218-235 · chunk 2674 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct ip save_ip, *ip;
//      register struct tcpiphdr *ti;
//      caddr_t optp = 
//                    ((void *)0)
//                        ;
//      int optlen = 0;
//      int len, tlen, off;
//             register struct tcpcb *tp = 
//                                        ((void *)0)
//                                            ;
//      register int tiflags;
//             struct socket *so = 
//                                ((void *)0)
//                                    ;
//      int todrop, acked, ourfinisacked, needoutput = 0;
//      int iss = 0;
//      u_long tiwin;
//      int ret;
//         struct ex_list *ex_ptr;
//         Slirp *slirp;
//     
//      ;
//     
//                                           ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2674 {
// Port of the prologue chunk of tcp_input(): the local declarations stay in the
// C translation unit (the rest of the function uses them), and the only
// statements in the chunk are DEBUG_CALL/DEBUG_ARGS, which expand to nothing
// because DEBUG is not defined in the preprocessed program.  Hence the ported
// piece has no observable effect.

#[no_mangle]
pub extern "C" fn tcp_input__part2674() {}
}

// ── `if (m == ((void *)0)` · slirp/tcp_input.c:240-253 · chunk 2675 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (m == 
//              ((void *)0)
//                  ) {
//       so = inso;
//       slirp = so->slirp;
//     
//     
//       tp = ((so)->so_tcpcb);
//       m = so->so_m;
//                     so->so_m = 
//                               ((void *)0)
//                                   ;
//       ti = so->so_ti;
//       tiwin = ti->ti_t.th_win;
//       tiflags = ti->ti_t.th_flags;
//     
//       goto cont_conn;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2675 {
// Port of the `m == NULL` (connection-completion) prologue of slirp tcp_input.
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
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

// Opaque types: only ever handled through pointers here.
#[repr(C)]
pub struct mbuf {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct tcpcb {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct Slirp {
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
}

// Mirrors `struct tcp_input__part2675_out` declared in the C caller.
#[repr(C)]
pub struct PartOut {
    pub tp: *mut tcpcb,
    pub ti: *mut tcpiphdr,
    pub tiwin: c_ulong,
    pub tiflags: c_int,
}

#[no_mangle]
pub extern "C" fn tcp_input__part2675(
    m: *mut mbuf,
    inso: *mut socket,
    so_out: *mut *mut socket,
    slirp_out: *mut *mut Slirp,
    m_out: *mut *mut mbuf,
    out: *mut PartOut,
) -> c_int {
    if m.is_null() {
        unsafe {
            let so = inso;
            *so_out = so;
            *slirp_out = (*so).slirp;
            let mm = (*so).so_m;
            *m_out = mm;
            (*so).so_m = core::ptr::null_mut();
            let ti = (*so).so_ti;
            *out = PartOut {
                tp: (*so).so_tcpcb,
                ti,
                tiwin: (*ti).ti_t.th_win as c_ulong,
                tiflags: (*ti).ti_t.th_flags as c_int,
            };
            1
        }
    } else {
        0
    }
}
}

// ── `slirp = m->slirp; ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);` · slirp/tcp_input.c:254-260 · chunk 2676 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     slirp = m->slirp;
//     
//     
//     
//     
//     
//      ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2676 {
// Port of the piece of tcp_input that fetches the Slirp pointer and the
// tcpiphdr pointer from the mbuf.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

// Opaque: only ever used behind a pointer.
pub type Slirp = c_void;

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void, // struct socket *
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
pub struct tcp_input__part2676_res {
    pub slirp: *mut Slirp,
    pub ti: *mut c_void, // struct tcpiphdr *
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2676(m: *mut mbuf) -> tcp_input__part2676_res {
    tcp_input__part2676_res {
        slirp: (*m).slirp,
        ti: (*m).m_hdr.mh_data as *mut c_void,
    }
}
}

// ── `if (iphlen > sizeof(struct ip ))` · slirp/tcp_input.c:261-264 · chunk 2677 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (iphlen > sizeof(struct ip )) {
//        ip_stripoptions(m, (struct mbuf *)0);
//        iphlen=sizeof(struct ip );
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2677 {
// Port of the ip-option stripping branch from tcp_input.
use core::ffi::{c_int, c_void};

// Mirror of `struct ip` (packed) as declared in the translation unit; only its
// size is needed here (sizeof(struct ip)).
#[repr(C, packed)]
struct Ip {
    ip_hl_ip_v: u8,
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

extern "C" {
    fn ip_stripoptions(m: *mut c_void, o: *mut c_void);
}

#[no_mangle]
pub extern "C" fn tcp_input__part2677(m: *mut c_void, iphlen: c_int) -> c_int {
    // C compares `int iphlen > sizeof(struct ip)`: iphlen is converted to
    // size_t (unsigned), which `as usize` reproduces (sign-extended then
    // reinterpreted).
    if (iphlen as usize) > core::mem::size_of::<Ip>() {
        unsafe {
            ip_stripoptions(m, core::ptr::null_mut());
        }
        core::mem::size_of::<Ip>() as c_int
    } else {
        iphlen
    }
}
}

// ── `ip=((struct ip *)(m)->m_hdr.mh_data); save_ip = *ip; save_ip.ip_len+= iphlen; tlen = ((struct i…` · slirp/tcp_input.c:272-284 · chunk 2678 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     ip=((struct ip *)(m)->m_hdr.mh_data);
//      save_ip = *ip;
//      save_ip.ip_len+= iphlen;
//     
//     
//     
//     
//      tlen = ((struct ip *)ti)->ip_len;
//             ((struct qlink*)(((char*)(ti)) - sizeof(struct qlink)))->next = ((struct qlink*)(((char*)(ti)) - sizeof(struct qlink)))->prev = 
//                                                                  ((void *)0)
//                                                                      ;
//             memset(&ti->ti_i.ih_mbuf, 0 , sizeof(struct mbuf_ptr));
//      ti->ti_i.ih_x1 = 0;
//      ti->ti_i.ih_len = htons((uint16_t)tlen);
//      len = sizeof(struct ip ) + tlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2678 {
// Port of the tcp_input prologue piece: IP header save, tcpiphdr init.
use core::ffi::{c_char, c_int, c_void};

#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// packed in C
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
    // bitfields th_x2:4, th_off:4
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C, packed)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// packed in C
#[repr(C, packed)]
pub struct Ip {
    // bitfields ip_hl:4, ip_v:4
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
pub struct M_hdr {
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
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: M_hdr,
    pub slirp: *mut c_void,
    pub m_dat: M_dat,
}

extern "C" {
    fn htons(x: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2678(
    m: *mut Mbuf,
    ti: *mut Tcpiphdr,
    iphlen: c_int,
    ipp: *mut *mut Ip,
    save_ip: *mut Ip,
    tlenp: *mut c_int,
    lenp: *mut c_int,
) {
    // ip = (struct ip *)m->m_hdr.mh_data;
    let ip_ptr: *mut Ip = (*m).m_hdr.mh_data as *mut Ip;
    *ipp = ip_ptr;

    // save_ip = *ip; save_ip.ip_len += iphlen;
    let mut sv: Ip = core::ptr::read_unaligned(ip_ptr);
    sv.ip_len = sv.ip_len.wrapping_add(iphlen as u16);
    core::ptr::write_unaligned(save_ip, sv);

    // tlen = ((struct ip *)ti)->ip_len;  (ip_len sits at byte offset 2 of packed struct ip)
    let tlen: c_int = core::ptr::read_unaligned((ti as *const u8).add(2) as *const u16) as c_int;
    *tlenp = tlen;

    let base: *mut u8 = ti as *mut u8;

    // ((struct qlink *)((char *)ti - sizeof(struct qlink)))->next = ...->prev = NULL;
    let q: *mut Qlink = base.sub(core::mem::size_of::<Qlink>()) as *mut Qlink;
    core::ptr::write_unaligned(
        q,
        Qlink {
            next: core::ptr::null_mut(),
            prev: core::ptr::null_mut(),
        },
    );

    // memset(&ti->ti_i.ih_mbuf, 0, sizeof(struct mbuf_ptr));
    core::ptr::write_bytes(base, 0u8, core::mem::size_of::<MbufPtr>());
    // ti->ti_i.ih_x1 = 0;
    core::ptr::write_unaligned(base.add(8), 0u8);
    // ti->ti_i.ih_len = htons((uint16_t)tlen);
    core::ptr::write_unaligned(base.add(10) as *mut u16, htons(tlen as u16));

    // len = sizeof(struct ip) + tlen;
    *lenp = 20 + tlen;
}
}

// ── `if (cksum(m, len))` · slirp/tcp_input.c:285-287 · chunk 2679 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if(cksum(m, len)) {
//        goto drop;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2679 {
// Ported piece of tcp_input: the checksum test that jumps to `drop`.
use core::ffi::c_int;

// Opaque mirror of `struct mbuf`; only pointers to it cross this boundary.
#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

extern "C" {
    fn cksum(m: *mut mbuf, len: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_input__part2679(m: *mut mbuf, len: c_int) -> c_int {
    unsafe { cksum(m, len) }
}
}

// ── `off = ti->ti_t.th_off << 2;` · slirp/tcp_input.c:293-293 · chunk 2680 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     off = ti->ti_t.th_off << 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2680 {
// Extract the TCP data offset (th_off) from the TCP header of a tcpiphdr
// and convert it to a byte offset (off = ti->ti_t.th_off << 2).

use core::ffi::{c_int, c_void};

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
struct MbufPtr {
    mptr: *mut c_void,
}

// struct in_addr { in_addr_t s_addr; };
#[repr(C)]
#[derive(Clone, Copy)]
struct InAddr {
    s_addr: u32,
}

// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
struct Ipovly {
    ih_mbuf: MbufPtr,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: InAddr,
    ih_dst: InAddr,
}

// struct tcphdr { ... u_int th_x2:4, th_off:4; ... };
#[repr(C)]
struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    // storage byte holding the two 4-bit fields: th_x2 in the low nibble,
    // th_off in the high nibble (little-endian bit allocation).
    th_x2_off: u8,
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

#[no_mangle]
pub extern "C" fn tcp_input__part2680(ti: *const Tcpiphdr) -> c_int {
    if ti.is_null() {
        return 0;
    }
    let bits = unsafe { core::ptr::addr_of!((*ti).ti_t.th_x2_off).read() };
    let th_off = ((bits >> 4) & 0x0f) as c_int;
    th_off << 2
}
}

// ── `if (off < sizeof (struct tcphdr) || off > tlen)` · slirp/tcp_input.c:294-296 · chunk 2681 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (off < sizeof (struct tcphdr) || off > tlen) {
//        goto drop;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2681 {
// Rust rewrite of the TCP header length sanity check inside tcp_input().
// C original:
//   off = ti->ti_t.th_off << 2;
//   if (off < sizeof (struct tcphdr) || off > tlen) {
//     goto drop;
//   }
// The goto stays in the caller; this function reports whether the drop
// branch must be taken.

use core::ffi::c_int;

// Mirror of `struct tcphdr` (the bitfield pair th_x2:4 / th_off:4 is packed
// into a single `u_int`, which is 4 bytes here), so that `size_of` matches
// the C `sizeof (struct tcphdr)`.
#[repr(C)]
#[allow(dead_code)]
struct Tcphdr {
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

#[no_mangle]
pub extern "C" fn tcp_input__part2681(off: c_int, tlen: c_int) -> c_int {
    let hdr_len: u64 = core::mem::size_of::<Tcphdr>() as u64;
    // `off < sizeof (struct tcphdr)`: the int is converted to size_t
    // (unsigned long), i.e. sign-extended modulo 2^64.
    let off_unsigned: u64 = (off as i64) as u64;
    if off_unsigned < hdr_len {
        return 1;
    }
    // `off > tlen`: plain signed int comparison.
    if off > tlen {
        return 1;
    }
    0
}
}

// ── `tlen -= off; ti->ti_i.ih_len = tlen;` · slirp/tcp_input.c:297-298 · chunk 2682 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tlen -= off;
//      ti->ti_i.ih_len = tlen;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2682 {
// Port of tcp_input.c lines 297-298 (the payload-length fixup after the
// TCP option header has been skipped).
//
// C original:
//     tlen -= off;
//     ti->ti_i.ih_len = tlen;
//
// `off` and `tlen` are read by value; the field `ti->ti_i.ih_len`
// (uint16_t inside struct ipovly) is written through a pointer, and the new
// value of `tlen` is returned so the caller can store it back.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_input__part2682(off: c_int, tlen: c_int, ih_len: *mut u16) -> c_int {
    // int arithmetic: keep wrapping semantics instead of trapping.
    let new_tlen: c_int = tlen.wrapping_sub(off);
    // Assignment to a uint16_t field truncates modulo 2^16.
    unsafe {
        *ih_len = new_tlen as u16;
    }
    new_tlen
}
}

// ── `if (off > sizeof (struct tcphdr))` · slirp/tcp_input.c:299-302 · chunk 2683 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (off > sizeof (struct tcphdr)) {
//        optlen = off - sizeof (struct tcphdr);
//        optp = ((caddr_t)(m)->m_hdr.mh_data) + sizeof (struct tcpiphdr);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2683 {
// Port of the IP-options extraction snippet of tcp_input.
// Computes, when the TCP header carries options, the option length and a
// pointer to the option bytes inside the mbuf.

use core::ffi::{c_char, c_int};

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

// struct ipovly is __attribute__((packed))
#[repr(C, packed)]
struct Ipvoly {
    ih_mbuf: *mut u8, // struct mbuf_ptr { struct mbuf *mptr; } (packed)
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
    th_x2_off: u32, // u_int th_x2:4, th_off:4;
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
struct Tcpiphdr {
    ti_i: Ipvoly,
    ti_t: Tcphdr,
}

#[repr(C)]
struct MHdr {
    mh_next: *mut u8,
    mh_prev: *mut u8,
    mh_nextpkt: *mut u8,
    mh_prevpkt: *mut u8,
    mh_flags: c_int,
    mh_size: c_int,
    mh_so: *mut u8,
    mh_data: *mut c_char,
    mh_len: c_int,
}

#[repr(C)]
struct Mbuf {
    m_hdr: MHdr,
}

#[no_mangle]
pub extern "C" fn tcp_input__part2683(
    m: *mut Mbuf,
    off: c_int,
    optlen: *mut c_int,
    optp: *mut *mut c_char,
) {
    let th_size = core::mem::size_of::<Tcphdr>() as c_int;
    if off > th_size {
        let newlen = off - th_size;
        unsafe {
            *optlen = newlen;
            let data = (*m).m_hdr.mh_data;
            *optp = data.add(core::mem::size_of::<Tcpiphdr>());
        }
    }
}
}

// ── `tiflags = ti->ti_t.th_flags; ((ti->ti_t.th_seq) = ntohl((ti->ti_t.th_seq))); ((ti->ti_t.th_ack)…` · slirp/tcp_input.c:303-317 · chunk 2684 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tiflags = ti->ti_t.th_flags;
//     
//     
//     
//     
//      ((ti->ti_t.th_seq) = ntohl((ti->ti_t.th_seq)));
//      ((ti->ti_t.th_ack) = ntohl((ti->ti_t.th_ack)));
//      ((ti->ti_t.th_win) = ntohs((uint16_t)(ti->ti_t.th_win)));
//      ((ti->ti_t.th_urp) = ntohs((uint16_t)(ti->ti_t.th_urp)));
//     
//     
//     
//     
//      m->m_hdr.mh_data += sizeof(struct tcpiphdr)+off-sizeof(struct tcphdr);
//      m->m_hdr.mh_len -= sizeof(struct tcpiphdr)+off-sizeof(struct tcphdr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2684 {
// Part of tcp_input: capture flags, convert TCP header fields to host order and
// advance the mbuf data pointer past the IP/TCP headers (and options).
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uchar, c_uint};
use core::ptr;

extern "C" {
    fn ntohl(__netlong: u32) -> u32;
    fn ntohs(__netshort: u16) -> u16;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct mbuf_ptr {
    pub mptr: *mut mbuf,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipovly {
    pub ih_mbuf: mbuf_ptr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: in_addr,
    pub ih_dst: in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    // u_int th_x2:4, th_off:4  (one storage byte)
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpiphdr {
    pub ti_i: ipovly,
    pub ti_t: tcphdr,
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
    pub slirp: *mut core::ffi::c_void,
    // union M_dat { char m_dat_[1]; char *m_ext_; }
    pub m_ext: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2684(
    m: *mut mbuf,
    ti: *mut tcpiphdr,
    off: c_int,
) -> c_int {
    let ti_t = ptr::addr_of_mut!((*ti).ti_t);

    // tiflags = ti->ti_t.th_flags;
    let flags_ptr = ptr::addr_of_mut!((*ti_t).th_flags);
    let tiflags = flags_ptr.read_unaligned() as c_int;

    // th_seq = ntohl(th_seq)
    let seq_ptr = ptr::addr_of_mut!((*ti_t).th_seq);
    let seq = seq_ptr.read_unaligned();
    seq_ptr.write_unaligned(ntohl(seq));

    // th_ack = ntohl(th_ack)
    let ack_ptr = ptr::addr_of_mut!((*ti_t).th_ack);
    let ack = ack_ptr.read_unaligned();
    ack_ptr.write_unaligned(ntohl(ack));

    // th_win = ntohs((uint16_t)th_win)
    let win_ptr = ptr::addr_of_mut!((*ti_t).th_win);
    let win = win_ptr.read_unaligned();
    win_ptr.write_unaligned(ntohs(win));

    // th_urp = ntohs((uint16_t)th_urp)
    let urp_ptr = ptr::addr_of_mut!((*ti_t).th_urp);
    let urp = urp_ptr.read_unaligned();
    urp_ptr.write_unaligned(ntohs(urp));

    // m->m_hdr.mh_data += sizeof(struct tcpiphdr) + off - sizeof(struct tcphdr)
    // m->m_hdr.mh_len  -= sizeof(struct tcpiphdr) + off - sizeof(struct tcphdr)
    let delta: c_int = core::mem::size_of::<tcpiphdr>() as c_int + off
        - core::mem::size_of::<tcphdr>() as c_int;

    let data_ptr = ptr::addr_of_mut!((*m).m_hdr.mh_data);
    let data = data_ptr.read_unaligned();
    data_ptr.write_unaligned(data.offset(delta as isize));

    let len_ptr = ptr::addr_of_mut!((*m).m_hdr.mh_len);
    let len = len_ptr.read_unaligned();
    len_ptr.write_unaligned(len - delta);

    tiflags
}

// keep the imported C scalar aliases referenced (no-op, avoids unused warnings)
const _: () = {
    let _ = core::mem::size_of::<c_uchar>();
    let _ = core::mem::size_of::<c_uint>();
};
}

// ── `if (slirp->restricted)` · slirp/tcp_input.c:319-328 · chunk 2685 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp->restricted) {
//             for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next) {
//                 if (ex_ptr->ex_fport == ti->ti_t.th_dport &&
//                     ti->ti_i.ih_dst.s_addr == ex_ptr->ex_addr.s_addr) {
//                     break;
//                 }
//             }
//             if (!ex_ptr)
//                 goto drop;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2685 {
// Ported piece of slirp's tcp_input: the "restricted" exec-list filter.
// Returns 1 when the packet must be dropped (C's `goto drop`), 0 otherwise.

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TimeVal {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct ExList {
    ex_pty: c_int,
    ex_addr: InAddr,
    ex_fport: c_int,
    ex_exec: *const c_char,
    ex_next: *mut ExList,
}

// Only the prefix of Slirp up to (and including) exec_list is needed;
// the layout mirrors the C declaration exactly.
#[repr(C)]
struct Slirp {
    vnetwork_addr: InAddr,
    vnetwork_mask: InAddr,
    vhost_addr: InAddr,
    vdhcp_startaddr: InAddr,
    vnameserver_addr: InAddr,
    client_ethaddr: [u8; 6],
    client_ipaddr: InAddr,
    client_hostname: [c_char; 33],
    restricted: c_int,
    tt: TimeVal,
    exec_list: *mut ExList,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Ipovly {
    ih_mbuf: [u8; 8],
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: InAddr,
    ih_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    // th_x2:4, th_off:4 packed into one unsigned int
    th_x2off: u32,
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

#[no_mangle]
pub extern "C" fn tcp_input__part2685(slirp: *mut Slirp, ti: *mut Tcpiphdr) -> c_int {
    unsafe {
        if slirp.is_null() {
            return 0;
        }
        if (*slirp).restricted == 0 {
            return 0;
        }
        if ti.is_null() {
            // C would fault here; treat "no matching entry" as a drop.
            return 1;
        }

        let th_dport: u16 = (*ti).ti_t.th_dport;
        let ih_dst_s_addr: u32 = (*ti).ti_i.ih_dst.s_addr;

        let mut ex_ptr: *mut ExList = (*slirp).exec_list;
        while !ex_ptr.is_null() {
            let e_fport: c_int = (*ex_ptr).ex_fport;
            let e_addr: u32 = (*ex_ptr).ex_addr.s_addr;
            if e_fport == (th_dport as c_int) && ih_dst_s_addr == e_addr {
                break;
            }
            ex_ptr = (*ex_ptr).ex_next;
        }

        if ex_ptr.is_null() {
            1
        } else {
            0
        }
    }
}
}

// ── `findso: so = slirp->tcp_last_so;` · slirp/tcp_input.c:332-333 · chunk 2686 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     findso:
//      so = slirp->tcp_last_so;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2686 {
// Piece of tcp_input():  "findso: so = slirp->tcp_last_so;"

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
    pub ex_pty: i32,
    pub ex_addr: InAddr,
    pub ex_fport: i32,
    pub ex_exec: *const u8,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut u8,
    pub mh_prev: *mut u8,
    pub mh_nextpkt: *mut u8,
    pub mh_prevpkt: *mut u8,
    pub mh_flags: i32,
    pub mh_size: i32,
    pub mh_so: *mut u8,
    pub mh_data: *mut u8,
    pub mh_len: i32,
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut u8,
    // union M_dat { char m_dat_[1]; char *m_ext_; }
    pub m_dat: [u8; 8],
}

#[repr(C)]
pub struct QLink {
    pub next: *mut u8,
    pub prev: *mut u8,
}

#[repr(C, packed)]
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
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut u8,
    pub sb_rptr: *mut u8,
    pub sb_data: *mut u8,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut u8,
    pub so_prev: *mut u8,
    pub s: i32,
    pub slirp: *mut u8,
    pub so_m: *mut u8,
    pub so_ti: *mut u8,
    pub so_urgc: i32,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: i32,
    pub so_tcpcb: *mut u8,
    pub so_expire: u32,
    pub so_queued: i32,
    pub so_nqueued: i32,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut u8,
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
    pub restricted: i32,
    pub tt: Timeval,
    pub exec_list: *mut ExList,
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: i32,
    pub if_queued: i32,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut u8,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2686(slirp: *mut Slirp) -> *mut Socket {
    (*slirp).tcp_last_so
}
}

// ── `if (so->so_fport != ti->ti_t.th_dport || so->so_lport != ti->ti_t.th_sport || so->so_laddr.s_ad…` · slirp/tcp_input.c:334-342 · chunk 2687 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_fport != ti->ti_t.th_dport ||
//          so->so_lport != ti->ti_t.th_sport ||
//          so->so_laddr.s_addr != ti->ti_i.ih_src.s_addr ||
//          so->so_faddr.s_addr != ti->ti_i.ih_dst.s_addr) {
//       so = solookup(&slirp->tcb, ti->ti_i.ih_src, ti->ti_t.th_sport,
//               ti->ti_i.ih_dst, ti->ti_t.th_dport);
//       if (so)
//        slirp->tcp_last_so = so;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2687 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_uint};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut core::ffi::c_void,
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
    pub th_x2_off: u32, // u_int th_x2:4, th_off:4
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

#[repr(C)]
pub struct Sbuff {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut i8,
    pub sb_rptr: *mut i8,
    pub sb_data: *mut i8,
}

// Mirror of `struct socket` (field offsets up to `so_lport` are what matters here;
// the remaining fields keep the layout exact).
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
    pub so_rcv: Sbuff,
    pub so_snd: Sbuff,
    pub extra: *mut core::ffi::c_void,
}

extern "C" {
    fn solookup(
        tcb: *mut Socket,
        laddr: InAddr,
        lport: c_uint,
        faddr: InAddr,
        fport: c_uint,
    ) -> *mut Socket;
}

// Ported TARGET: look up the socket for this segment, caching it in
// slirp->tcp_last_so when a match is found.
#[no_mangle]
pub extern "C" fn tcp_input__part2687(
    mut so: *mut Socket,
    tcb: *mut Socket,
    tcp_last_so: *mut *mut Socket,
    ti: *mut Tcpiphdr,
) -> *mut Socket {
    unsafe {
        let dport: u16 = (*ti).ti_t.th_dport;
        let sport: u16 = (*ti).ti_t.th_sport;
        let src: u32 = (*ti).ti_i.ih_src.s_addr;
        let dst: u32 = (*ti).ti_i.ih_dst.s_addr;

        if (*so).so_fport != dport
            || (*so).so_lport != sport
            || (*so).so_laddr.s_addr != src
            || (*so).so_faddr.s_addr != dst
        {
            so = solookup(
                tcb,
                InAddr { s_addr: src },
                sport as c_uint,
                InAddr { s_addr: dst },
                dport as c_uint,
            );
            if !so.is_null() {
                *tcp_last_so = so;
            }
        }
        so
    }
}
}

// ── `if (so == ((void *)0)` · slirp/tcp_input.c:357-381 · chunk 2688 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so == 
//                      ((void *)0)
//                          ) {
//        if ((tiflags & (0x02|0x01|0x04|0x20|0x10)) != 0x02)
//          goto dropwithreset;
//     
//        if ((so = socreate(slirp)) == 
//                                     ((void *)0)
//                                         )
//          goto dropwithreset;
//        if (tcp_attach(so) < 0) {
//          free(so);
//          goto dropwithreset;
//        }
//     
//        sbreserve(&so->so_snd, 8192);
//        sbreserve(&so->so_rcv, 8192);
//     
//        so->so_laddr = ti->ti_i.ih_src;
//        so->so_lport = ti->ti_t.th_sport;
//        so->so_faddr = ti->ti_i.ih_dst;
//        so->so_fport = ti->ti_t.th_dport;
//     
//        if ((so->so_iptos = tcp_tos(so)) == 0)
//          so->so_iptos = ((struct ip *)ti)->ip_tos;
//     
//        tp = ((so)->so_tcpcb);
//        tp->t_state = 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2688 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub type in_addr_t = u32;
pub type tcp_seq = u32;
pub type u_int = u32;
pub type u_short = u16;
pub type u_char = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
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
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    // u_int th_x2:4, th_off:4;  (one 32-bit bitfield storage unit)
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

// struct ip, __attribute__((packed))
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ip {
    pub ip_hl_v: u32,
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
pub struct sbuf {
    pub sb_cc: u_int,
    pub sb_datalen: u_int,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
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
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub t_idle: i16,
    pub t_rtt: i16,
    pub t_rtseq: tcp_seq,
    pub t_srtt: i16,
    pub t_rttvar: i16,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: i16,
    pub snd_scale: u_char,
    pub rcv_scale: u_char,
    pub request_r_scale: u_char,
    pub requested_s_scale: u_char,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: tcp_seq,
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
    pub so_type: u_char,
    pub so_state: c_int,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: u_int,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn socreate(slirp: *mut Slirp) -> *mut socket;
    fn tcp_attach(so: *mut socket) -> c_int;
    fn tcp_tos(so: *mut socket) -> u8;
    fn sbreserve(sb: *mut sbuf, n: c_int);
    fn free(p: *mut c_void);
}

// Returns 0 to fall through, 1 for "goto dropwithreset".
#[no_mangle]
pub extern "C" fn tcp_input__part2688(
    so_p: *mut *mut socket,
    tiflags: c_int,
    slirp: *mut Slirp,
    ti: *mut tcpiphdr,
    tp_p: *mut *mut tcpcb,
) -> c_int {
    unsafe {
        if (*so_p).is_null() {
            if (tiflags & (0x02 | 0x01 | 0x04 | 0x20 | 0x10)) != 0x02 {
                return 1;
            }

            let nso = socreate(slirp);
            *so_p = nso;
            if nso.is_null() {
                return 1;
            }
            if tcp_attach(nso) < 0 {
                free(nso as *mut c_void);
                return 1;
            }

            sbreserve(&mut (*nso).so_snd as *mut sbuf, 8192);
            sbreserve(&mut (*nso).so_rcv as *mut sbuf, 8192);

            let laddr: in_addr = (*ti).ti_i.ih_src;
            (*nso).so_laddr = laddr;
            let lport: u16 = (*ti).ti_t.th_sport;
            (*nso).so_lport = lport;
            let faddr: in_addr = (*ti).ti_i.ih_dst;
            (*nso).so_faddr = faddr;
            let fport: u16 = (*ti).ti_t.th_dport;
            (*nso).so_fport = fport;

            let tos: u8 = tcp_tos(nso);
            (*nso).so_iptos = tos;
            if tos == 0 {
                let ipp = ti as *mut Ip;
                let iptos: u8 = (*ipp).ip_tos;
                (*nso).so_iptos = iptos;
            }

            let ntpcb = (*nso).so_tcpcb;
            (*ntpcb).t_state = 1;
            *tp_p = ntpcb;
        }
        0
    }
}
}

// ── `if (so->so_state & 0x002)` · slirp/tcp_input.c:388-389 · chunk 2689 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (so->so_state & 0x002)
//                     goto drop;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2689 {
// Port of the `if (so->so_state & 0x002) goto drop;` check in tcp_input.
use core::ffi::c_int;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut core::ffi::c_char,
    pub sb_rptr: *mut core::ffi::c_char,
    pub sb_data: *mut core::ffi::c_char,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut core::ffi::c_void,
    pub so_m: *mut core::ffi::c_void,
    pub so_ti: *mut core::ffi::c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
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
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut core::ffi::c_void,
}

/// Returns nonzero when the socket is in the SS_ISFCONNECTING-ish state
/// flagged by bit 0x002, i.e. when tcp_input must go to `drop`.
#[no_mangle]
pub extern "C" fn tcp_input__part2689(so: *mut socket) -> c_int {
    unsafe {
        if ((*so).so_state & 0x002) != 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `tp = ((so)->so_tcpcb);` · slirp/tcp_input.c:391-391 · chunk 2690 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tp = ((so)->so_tcpcb);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2690 {
// Port of `tp = sototcpcb(so);` from tcp_input (slirp/tcp_input.c:391)
#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcpiphdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct tcpcb {
    _private: [u8; 0],
}

// Mirror of the leading fields of `struct socket` up to `so_tcpcb`.
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
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2690(so: *mut socket) -> *mut tcpcb {
    (*so).so_tcpcb
}
}

// ── `if (tp == ((void *)0)` · slirp/tcp_input.c:394-395 · chunk 2691 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp == 
//                      ((void *)0)
//                          )
//       goto dropwithreset;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2691 {
// Port of the `if (tp == NULL) goto dropwithreset;` check from tcp_input().
// Only the pointer's null-ness is observed here, so the pointee is opaque.

use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn tcp_input__part2691(tp: *const c_void) -> c_int {
    if tp.is_null() {
        1
    } else {
        0
    }
}
}

// ── `if (tp->t_state == 0)` · slirp/tcp_input.c:396-397 · chunk 2692 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_state == 0)
//       goto drop;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2692 {
// Port of the state check inside slirp's tcp_input:
//   if (tp->t_state == 0) goto drop;
// Returns 1 when the caller must jump to `drop`, 0 otherwise.

use core::ffi::{c_int, c_short};

// Prefix of the C `struct tcpcb` (slirp/tcp_var.h) up to the t_state member.
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: c_short,
}

#[no_mangle]
pub extern "C" fn tcp_input__part2692(tp: *mut Tcpcb) -> c_int {
    unsafe {
        if (*tp).t_state == 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `tiwin = ti->ti_t.th_win; tp->t_idle = 0;` · slirp/tcp_input.c:399-405 · chunk 2693 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tiwin = ti->ti_t.th_win;
//     
//     
//     
//     
//     
//      tp->t_idle = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2693 {
// Ported piece of tcp_input: tiwin = ti->ti_t.th_win; tp->t_idle = 0;
use core::ffi::{c_short, c_ulong, c_ushort};

#[no_mangle]
pub extern "C" fn tcp_input__part2693(th_win: *const c_ushort, t_idle: *mut c_short) -> c_ulong {
    unsafe {
        let win = *th_win as c_ulong;
        *t_idle = 0;
        win
    }
}
}

// ── `if (0) … else` · slirp/tcp_input.c:406-409 · chunk 2694 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (0)
//         tp->t_timer[2] = ( 75*2);
//      else
//         tp->t_timer[2] = (120*60*2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2694 {
// Portion of slirp's tcp_input(): the (constant-condition) assignment of the
// keepalive/2MSL timer, tp->t_timer[2].
//
// The original C is:
//     if (0)
//         tp->t_timer[2] = ( 75*2);
//     else
//         tp->t_timer[2] = (120*60*2);
//
// `tp` is only read (never assigned) by this piece, so it is passed by
// pointer, exactly as C holds it.

use core::ffi::{c_short, c_void};

// Layout-identical prefix of `struct tcpcb` from slirp/tcp_var.h; the fields
// up to and including t_timer are enough to reach tp->t_timer[2].
#[repr(C)]
pub struct TcpcbPrefix {
    seg_next: *mut c_void, // struct tcpiphdr *
    seg_prev: *mut c_void, // struct tcpiphdr *
    t_state: c_short,
    t_timer: [c_short; 4],
}

#[no_mangle]
pub extern "C" fn tcp_input__part2694(tp: *mut TcpcbPrefix) {
    // The condition is the literal 0, so the else branch always runs.
    if false {
        unsafe {
            (*tp).t_timer[2] = 75 * 2;
        }
    } else {
        unsafe {
            (*tp).t_timer[2] = 120 * 60 * 2;
        }
    }
}
}

// ── `if (optp && tp->t_state != 1)` · slirp/tcp_input.c:415-416 · chunk 2695 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (optp && tp->t_state != 1)
//       tcp_dooptions(tp, (u_char *)optp, optlen, ti);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2695 {
// Port of the `tcp_dooptions(tp, (u_char *)optp, optlen, ti)` invocation in
// tcp_input (original.c lines 415-416) together with the static helper
// tcp_dooptions() it calls, which has no linker symbol and is therefore
// re-implemented here as a private Rust function.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct Socket {
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

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed))
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

// struct ipovly { ... } __attribute__((packed))
#[repr(C, packed)]
pub struct Ipvovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr; the "u_int th_x2:4, th_off:4;" bitfield pair is mirrored as
// the single byte it occupies on the target ABI.
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

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; }
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipvovly,
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

extern "C" {
    fn tcp_mss(tp: *mut Tcpcb, offer: c_uint) -> c_int;
}

// static void tcp_dooptions(struct tcpcb *tp, u_char *cp, int cnt,
//                           struct tcpiphdr *ti)
unsafe fn tcp_dooptions(tp: *mut Tcpcb, mut cp: *mut u8, mut cnt: c_int, ti: *mut Tcpiphdr) {
    while cnt > 0 {
        let opt = *cp as c_int;
        if opt == 0 {
            break;
        }
        let optlen: c_int;
        if opt == 1 {
            optlen = 1;
        } else {
            optlen = *cp.offset(1) as c_int;
            if optlen <= 0 {
                break;
            }
        }
        // switch (opt) { default: continue; case 2: ... }
        if opt == 2 {
            // TH_SYN
            if optlen == 4 && ((*ti).ti_t.th_flags & 0x02) != 0 {
                let b0 = *cp.offset(2);
                let b1 = *cp.offset(3);
                let mss = u16::from_be_bytes([b0, b1]);
                tcp_mss(tp, mss as c_uint);
            }
        }
        cnt -= optlen;
        cp = cp.offset(optlen as isize);
    }
}

#[no_mangle]
pub extern "C" fn tcp_input__part2695(
    optp: *mut c_char,
    tp: *mut Tcpcb,
    optlen: c_int,
    ti: *mut Tcpiphdr,
) {
    unsafe {
        if !optp.is_null() && (*tp).t_state != 1 {
            tcp_dooptions(tp, optp as *mut u8, optlen, ti);
        }
    }
}
}

// ── `if (tp->t_state == 4 && (tiflags & (0x02|0x01|0x04|0x20|0x10)) == 0x10 && ti->ti_t.th_seq == tp…` · slirp/tcp_input.c:436-509 · chunk 2696 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_state == 4 &&
//          (tiflags & (0x02|0x01|0x04|0x20|0x10)) == 0x10 &&
//          ti->ti_t.th_seq == tp->rcv_nxt &&
//          tiwin && tiwin == tp->snd_wnd &&
//          tp->snd_nxt == tp->snd_max) {
//       if (ti->ti_i.ih_len == 0) {
//        if (((int)((ti->ti_t.th_ack)-(tp->snd_una)) > 0) &&
//            ((int)((ti->ti_t.th_ack)-(tp->snd_max)) <= 0) &&
//            tp->snd_cwnd >= tp->snd_wnd) {
//     
//     
//     
//         if (tp->t_rtt &&
//             ((int)((ti->ti_t.th_ack)-(tp->t_rtseq)) > 0))
//          tcp_xmit_timer(tp, tp->t_rtt);
//         acked = ti->ti_t.th_ack - tp->snd_una;
//         sbdrop(&so->so_snd, acked);
//         tp->snd_una = ti->ti_t.th_ack;
//         m_free(m);
//         if (tp->snd_una == tp->snd_max)
//          tp->t_timer[0] = 0;
//         else if (tp->t_timer[1] == 0)
//          tp->t_timer[0] = tp->t_rxtcur;
//     
//     
//     
//     
//     
//     
//         if (so->so_snd.sb_cc)
//          (void) tcp_output(tp);
//     
//         return;
//        }
//       } else if (ti->ti_t.th_ack == tp->snd_una &&
//           ((tp)->seg_next == (struct tcpiphdr*)(tp)) &&
//           ti->ti_i.ih_len <= ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc)) {
//     
//     
//     
//     
//     
//        tp->rcv_nxt += ti->ti_i.ih_len;
//     
//     
//     
//        if (so->so_emu) {
//         if (tcp_emu(so,m)) sbappend(so, m);
//        } else
//         sbappend(so, m);
//        tp->t_flags |= 0x0001;
//        tcp_output(tp);
//        return;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2696 {
// Port of the "fast path" ACK/data-in-order block of tcp_input.
// Returns 1 when the original code took its early `return;`.

use core::ffi::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SbUF {
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
    pub so_fport: c_ushort,
    pub so_lport: c_ushort,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: c_uchar,
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SbUF,
    pub so_snd: SbUF,
    pub extra: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct MbHdr {
    pub mh_next: *mut core::ffi::c_void,
    pub mh_prev: *mut core::ffi::c_void,
    pub mh_nextpkt: *mut core::ffi::c_void,
    pub mh_prevpkt: *mut core::ffi::c_void,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbHdr,
    pub slirp: *mut core::ffi::c_void,
}

// struct ipovly (packed) — 24 bytes when ti_i.ih_mbuf is 8 bytes (mptr stored
// starting at offset 0 with 8 bytes of the field plus 4 bytes padding so the
// following packed fields land on their natural offsets).
#[repr(C)]
pub struct Ipovly {
    pub ih_mbuf: [u8; 8],
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
    pub snd_scale: c_uchar,
    pub rcv_scale: c_uchar,
    pub request_r_scale: c_uchar,
    pub requested_s_scale: c_uchar,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

extern "C" {
    fn sbdrop(sb: *mut SbUF, cnt: c_int);
    fn m_free(m: *mut Mbuf);
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> c_int;
}

// Port of the file-static C helper tcp_xmit_timer().
fn tcp_xmit_timer(tp: *mut Tcpcb, rtt: c_int) {
    unsafe {
        let tp = &mut *tp;
        if tp.t_srtt != 0 {
            let mut delta: c_short = (rtt - 1 - ((tp.t_srtt as c_int) >> 3)) as c_short;
            let v = (tp.t_srtt as c_int) + (delta as c_int);
            tp.t_srtt = v as c_short;
            if tp.t_srtt <= 0 {
                tp.t_srtt = 1;
            }
            if delta < 0 {
                delta = (-(delta as c_int)) as c_short;
            }
            let vs = (delta as c_int) - ((tp.t_rttvar as c_int) >> 2);
            delta = vs as c_short;
            let v2 = (tp.t_rttvar as c_int) + (delta as c_int);
            tp.t_rttvar = v2 as c_short;
            if tp.t_rttvar <= 0 {
                tp.t_rttvar = 1;
            }
        } else {
            tp.t_srtt = ((rtt as c_uint) << 3) as c_short;
            tp.t_rttvar = ((rtt as c_uint) << 1) as c_short;
        }
        tp.t_rtt = 0;
        tp.t_rxtshift = 0;
        {
            let mut rc = ((tp.t_srtt as c_int) >> 3) + (tp.t_rttvar as c_int);
            if rc < (tp.t_rttmin as c_int) {
                rc = tp.t_rttmin as c_int;
            } else if rc > 24 {
                rc = 24;
            }
            tp.t_rxtcur = rc as c_short;
        }
        tp.t_softerror = 0;
    }
}

#[no_mangle]
pub extern "C" fn tcp_input__part2696(
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
    so: *mut Socket,
    m: *mut Mbuf,
    tiflags: c_int,
    tiwin: core::ffi::c_ulong,
    acked_out: *mut c_int,
) -> c_int {
    unsafe {
        let tp_ = &mut *tp;
        let ti_ = &mut *ti;
        let so_ = &mut *so;

        let mut acked: c_int = 0;

        if tp_.t_state == 4
            && (tiflags & (0x02 | 0x01 | 0x04 | 0x20 | 0x10)) == 0x10
            && ti_.ti_t.th_seq == tp_.rcv_nxt
            && tiwin != 0
            && tiwin == (tp_.snd_wnd as core::ffi::c_ulong)
            && tp_.snd_nxt == tp_.snd_max
        {
            if ti_.ti_i.ih_len == 0 {
                if ((ti_.ti_t.th_ack.wrapping_sub(tp_.snd_una)) as c_int) > 0
                    && ((ti_.ti_t.th_ack.wrapping_sub(tp_.snd_max)) as c_int) <= 0
                    && tp_.snd_cwnd >= tp_.snd_wnd
                {
                    if tp_.t_rtt != 0
                        && ((ti_.ti_t.th_ack.wrapping_sub(tp_.t_rtseq)) as c_int) > 0
                    {
                        tcp_xmit_timer(tp, tp_.t_rtt as c_int);
                    }
                    acked = ti_.ti_t.th_ack.wrapping_sub(tp_.snd_una) as c_int;
                    sbdrop(&mut so_.so_snd as *mut SbUF, acked);
                    tp_.snd_una = ti_.ti_t.th_ack;
                    m_free(m);
                    if tp_.snd_una == tp_.snd_max {
                        tp_.t_timer[0] = 0;
                    } else if tp_.t_timer[1] == 0 {
                        tp_.t_timer[0] = tp_.t_rxtcur;
                    }

                    if so_.so_snd.sb_cc != 0 {
                        let _ = tcp_output(tp);
                    }

                    *acked_out = acked;
                    return 1;
                }
            } else if ti_.ti_t.th_ack == tp_.snd_una
                && tp_.seg_next as *mut u8 == tp as *mut u8
                && (ti_.ti_i.ih_len as u32)
                    <= (so_.so_rcv.sb_datalen.wrapping_sub(so_.so_rcv.sb_cc))
            {
                tp_.rcv_nxt = tp_.rcv_nxt.wrapping_add(ti_.ti_i.ih_len as u32);

                if so_.so_emu != 0 {
                    if tcp_emu(so, m) != 0 {
                        sbappend(so, m);
                    }
                } else {
                    sbappend(so, m);
                }
                tp_.t_flags |= 0x0001;
                let _ = tcp_output(tp);
                *acked_out = acked;
                return 1;
            }
        }

        *acked_out = acked;
        0
    }
}
}

// ── `{ … }` · slirp/tcp_input.c:516-521 · chunk 2697 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     { int win;
//               win = ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc);
//        if (win < 0)
//          win = 0;
//        tp->rcv_wnd = ((win) > ((int)(tp->rcv_adv - tp->rcv_nxt)) ? (win) : ((int)(tp->rcv_adv - tp->rcv_nxt)));
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2697 {
// Rewrite of tcp_input lines 516-521: rcv_wnd update.
// Receives pointers/values instead of whole C structs where possible,
// so no layout mirroring of struct tcpcb / struct socket is needed.

#[no_mangle]
pub extern "C" fn tcp_input__part2697(
    rcv_wnd: *mut u32,
    sb_datalen: ::core::ffi::c_uint,
    sb_cc: ::core::ffi::c_uint,
    rcv_adv: u32,
    rcv_nxt: u32,
) {
    // win = (int)(sb_datalen - sb_cc);   (unsigned subtraction, then to int)
    let mut win: i32 = sb_datalen.wrapping_sub(sb_cc) as i32;
    if win < 0 {
        win = 0;
    }
    // (int)(tp->rcv_adv - tp->rcv_nxt)
    let adv: i32 = rcv_adv.wrapping_sub(rcv_nxt) as i32;
    let val: i32 = if win > adv { win } else { adv };
    unsafe {
        if !rcv_wnd.is_null() {
            *rcv_wnd = val as u32;
        }
    }
}
}

// ── `switch (tp->t_state)` · slirp/tcp_input.c:523-712 · chunk 2698 · deepseek/deepseek-v4.1-flash-1017 · 4 attempt(s) · Unvalidated
//
// Original C:
//     switch (tp->t_state) {
//      case 1: {
//     
//        if (tiflags & 0x04)
//          goto drop;
//        if (tiflags & 0x10)
//          goto dropwithreset;
//        if ((tiflags & 0x02) == 0)
//          goto drop;
//        if ((so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) ==
//            slirp->vnetwork_addr.s_addr) {
//          if (so->so_faddr.s_addr != slirp->vhost_addr.s_addr &&
//       so->so_faddr.s_addr != slirp->vnameserver_addr.s_addr) {
//     
//       for (ex_ptr = slirp->exec_list; ex_ptr;
//            ex_ptr = ex_ptr->ex_next) {
//         if(ex_ptr->ex_fport == so->so_fport &&
//            so->so_faddr.s_addr == ex_ptr->ex_addr.s_addr) {
//           so->so_state |= 0x080;
//           break;
//         }
//       }
//       if (so->so_state & 0x080) {
//           goto cont_input;
//       }
//          }
//     
//        }
//     
//        if (so->so_emu & 0x10) {
//          so->so_emu &= ~0x10;
//          goto cont_input;
//        }
//     
//        if((tcp_fconnect(so) == -1) && (
//                                       (*__errno_location ()) 
//                                             != 
//                                                115
//                                                           ) && (
//                                                                 (*__errno_location ()) 
//                                                                       != 
//                                                                          11
//                                                                                     )) {
//          u_char code=0;
//         
//                               ;
//          if(
//            (*__errno_location ()) 
//                  == 
//                     111
//                                 ) {
//     
//            tcp_respond(tp, ti, m, ti->ti_t.th_seq+1, (tcp_seq)0,
//          0x04|0x10);
//          } else {
//            if(
//              (*__errno_location ()) 
//                    == 
//                       113
//                                   ) code=1;
//            ((ti->ti_t.th_seq) = htonl((ti->ti_t.th_seq)));
//            ((ti->ti_t.th_ack) = htonl((ti->ti_t.th_ack)));
//            ((ti->ti_t.th_win) = htons((uint16_t)(ti->ti_t.th_win)));
//            ((ti->ti_t.th_urp) = htons((uint16_t)(ti->ti_t.th_urp)));
//            m->m_hdr.mh_data -= sizeof(struct tcpiphdr)+off-sizeof(struct tcphdr);
//            m->m_hdr.mh_len += sizeof(struct tcpiphdr)+off-sizeof(struct tcphdr);
//            *ip=save_ip;
//            icmp_error(m, 3,code, 0,strerror(
//                                                       (*__errno_location ())
//                                                            ));
//          }
//                 tcp_close(tp);
//          m_free(m);
//        } else {
//     
//     
//     
//     
//     
//     
//          so->so_m = m;
//          so->so_ti = ti;
//          tp->t_timer[2] = ( 75*2);
//          tp->t_state = 3;
//        }
//        return;
//     
//      cont_conn:
//     
//     
//     
//        if (so->so_state & 0x001) {
//          tp = tcp_close(tp);
//          goto dropwithreset;
//        }
//      cont_input:
//        tcp_template(tp);
//     
//        if (optp)
//          tcp_dooptions(tp, (u_char *)optp, optlen, ti);
//     
//        if (iss)
//          tp->iss = iss;
//        else
//          tp->iss = slirp->tcp_iss;
//        slirp->tcp_iss += (125*1024)/2;
//        tp->irs = ti->ti_t.th_seq;
//        (tp)->snd_una = (tp)->snd_nxt = (tp)->snd_max = (tp)->snd_up = (tp)->iss;
//        (tp)->rcv_adv = (tp)->rcv_nxt = (tp)->irs + 1;
//        tp->t_flags |= 0x0001;
//        tp->t_state = 3;
//        tp->t_timer[2] = ( 75*2);
//        goto trimthenstep6;
//      }
//      case 2:
//       if ((tiflags & 0x10) &&
//           (((int)((ti->ti_t.th_ack)-(tp->iss)) <= 0) ||
//            ((int)((ti->ti_t.th_ack)-(tp->snd_max)) > 0)))
//        goto dropwithreset;
//     
//       if (tiflags & 0x04) {
//                             if (tiflags & 0x10) {
//                                     tcp_drop(tp, 0);
//                             }
//        goto drop;
//       }
//     
//       if ((tiflags & 0x02) == 0)
//        goto drop;
//       if (tiflags & 0x10) {
//        tp->snd_una = ti->ti_t.th_ack;
//        if (((int)((tp->snd_nxt)-(tp->snd_una)) < 0))
//         tp->snd_nxt = tp->snd_una;
//       }
//     
//       tp->t_timer[0] = 0;
//       tp->irs = ti->ti_t.th_seq;
//       (tp)->rcv_adv = (tp)->rcv_nxt = (tp)->irs + 1;
//       tp->t_flags |= 0x0001;
//       if (tiflags & 0x10 && ((int)((tp->snd_una)-(tp->iss)) > 0)) {
//        soisfconnected(so);
//        tp->t_state = 4;
//     
//        (void) tcp_reass(tp, (struct tcpiphdr *)0,
//         (struct mbuf *)0);
//     
//     
//     
//     
//        if (tp->t_rtt)
//         tcp_xmit_timer(tp, tp->t_rtt);
//       } else
//        tp->t_state = 3;
//     
//     trimthenstep6:
//     
//     
//     
//     
//     
//       ti->ti_t.th_seq++;
//       if (ti->ti_i.ih_len > tp->rcv_wnd) {
//        todrop = ti->ti_i.ih_len - tp->rcv_wnd;
//        m_adj(m, -todrop);
//        ti->ti_i.ih_len = tp->rcv_wnd;
//        tiflags &= ~0x01;
//       }
//       tp->snd_wl1 = ti->ti_t.th_seq - 1;
//       tp->rcv_up = ti->ti_t.th_seq;
//       goto step6;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2698 {
// Port of the `switch (tp->t_state) { case 1: ... case 2: ... }` block of
// slirp/tcp_input.c (tcp_input).  Exits that leave the block (return, drop,
// dropwithreset) are reported to the C caller through the return value; the
// gotos remaining inside the block are implemented with helper "label"
// functions.  The `goto step6` target of this region always falls into a
// plain `return;` of the enclosing function, so it yields RES_RETURN.
//
// `tcp_dooptions`, `tcp_reass` and `tcp_xmit_timer` are `static` in the C
// program, so their logic is ported here as private functions.

#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uchar, c_void};

type TcpSeq = u32;

// how the ported region terminates, interpreted by the C caller
const RES_FALLTHROUGH: c_int = 0; // continue after the replaced block
const RES_RETURN: c_int = 1; // `return;`
const RES_DROP: c_int = 2; // `goto drop;`
const RES_DROPWITHRESET: c_int = 3; // `goto dropwithreset;`
const RES_CLOSE_DROPWITHRESET: c_int = 4; // `tp = tcp_close(tp); goto dropwithreset;`

const NONE: c_int = -1; // no exit chosen yet

/* ---------------- mirrored C structs (exact field order/types) ---------- */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr, // struct mbuf_ptr is packed
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
    pub th_seq: TcpSeq,
    pub th_ack: TcpSeq,
    pub th_x2off: u8, // u_int th_x2:4, th_off:4;
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mbuf {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
    pub slirp: *mut Slirp,
    pub m_dat: [u8; 8], // union M_dat { char m_dat_[1]; char *m_ext_; }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
    pub snd_una: TcpSeq,
    pub snd_nxt: TcpSeq,
    pub snd_up: TcpSeq,
    pub snd_wl1: TcpSeq,
    pub snd_wl2: TcpSeq,
    pub iss: TcpSeq,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub rcv_nxt: TcpSeq,
    pub rcv_up: TcpSeq,
    pub irs: TcpSeq,
    pub rcv_adv: TcpSeq,
    pub snd_max: TcpSeq,
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub t_idle: i16,
    pub t_rtt: i16,
    pub t_rtseq: TcpSeq,
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
    pub last_ack_sent: TcpSeq,
}

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
    pub so_type: c_uchar,
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
#[derive(Clone, Copy)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ip {
    pub ip_x1: u8, // u_int ip_hl:4, ip_v:4;
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
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
    // remaining members are never accessed by this code; their layouts are
    // kept complete so that offsets stay exact.
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: [u8; 40],           // struct ipq (packed)
    pub ip_id: u16,
    pub bootp_clients: [u8; 96], // BOOTPClient bootp_clients[16]
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: TcpSeq,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [u8; 144], // struct tftp_session tftp_sessions[3]
    pub opaque: *mut c_void,
}

/* ----------------------------- extern symbols -------------------------- */

extern "C" {
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
    fn tcp_drop(tp: *mut Tcpcb, err: c_int) -> *mut Tcpcb;
    fn tcp_fconnect(so: *mut Socket) -> c_int;
    fn tcp_template(tp: *mut Tcpcb);
    fn tcp_mss(tp: *mut Tcpcb, offer: u32) -> c_int;
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> c_int;
    fn tcp_respond(
        tp: *mut Tcpcb,
        ti: *mut Tcpiphdr,
        m: *mut Mbuf,
        ack: TcpSeq,
        seq: TcpSeq,
        flags: c_int,
    );
    fn soisfconnected(so: *mut Socket);
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
    fn slirp_insque(q: *mut c_void, after: *mut c_void);
    fn slirp_remque(q: *mut c_void);
    fn m_adj(m: *mut Mbuf, diff: c_int);
    fn m_free(m: *mut Mbuf);
    fn htonl(x: u32) -> u32;
    fn htons(x: u16) -> u16;
    fn ntohs(x: u16) -> u16;
    fn icmp_error(
        msrc: *mut Mbuf,
        typ: c_uchar,
        code: c_uchar,
        minsize: c_int,
        message: *const c_char,
    );
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

/* ---------------- private ports of the C `static` helpers -------------- */

const QLINK_SIZE: isize = 16; // sizeof(struct qlink)

#[inline]
unsafe fn tcpiphdr2qlink(t: *mut Tcpiphdr) -> *mut Qlink {
    (t as *mut u8).offset(-QLINK_SIZE) as *mut Qlink
}

#[inline]
unsafe fn qlink2tcpiphdr(q: *mut Qlink) -> *mut Tcpiphdr {
    (q as *mut u8).offset(QLINK_SIZE) as *mut Tcpiphdr
}

#[inline]
unsafe fn tcpiphdr_next(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).next as *mut Qlink)
}

#[inline]
unsafe fn tcpiphdr_prev(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).prev as *mut Qlink)
}

#[inline]
unsafe fn tcpfrag_list_first(tp: *mut Tcpcb) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tp).seg_next as *mut Qlink)
}

#[inline]
unsafe fn tcpfrag_list_end(f: *mut Tcpiphdr, tp: *mut Tcpcb) -> bool {
    tcpiphdr2qlink(f) == tp as *mut Qlink
}

// port of `static int tcp_reass(struct tcpcb *, struct tcpiphdr *, struct mbuf *)`
unsafe fn tcp_reass(tp: *mut Tcpcb, ti_in: *mut Tcpiphdr, m_in: *mut Mbuf) -> c_int {
    let mut ti: *mut Tcpiphdr = ti_in;
    let mut m: *mut Mbuf = m_in;

    if ti.is_null() {
        return tcp_reass_present(tp);
    }

    let mut q: *mut Tcpiphdr = tcpfrag_list_first(tp);
    while !tcpfrag_list_end(q, tp) {
        if ((*q).ti_t.th_seq.wrapping_sub((*ti).ti_t.th_seq) as c_int) > 0 {
            break;
        }
        q = tcpiphdr_next(q);
    }

    if !tcpfrag_list_end(tcpiphdr_prev(q), tp) {
        q = tcpiphdr_prev(q);
        let i: c_int = (*q)
            .ti_t
            .th_seq
            .wrapping_add((*q).ti_i.ih_len as u32)
            .wrapping_sub((*ti).ti_t.th_seq) as c_int;
        if i > 0 {
            if i >= (*ti).ti_i.ih_len as c_int {
                m_free(m);
                return tcp_reass_present(tp);
            }
            m_adj(m, i);
            (*ti).ti_i.ih_len = ((*ti).ti_i.ih_len as c_int - i) as u16;
            (*ti).ti_t.th_seq = (*ti).ti_t.th_seq.wrapping_add(i as u32);
        }
        q = tcpiphdr_next(q);
    }
    (*ti).ti_i.ih_mbuf.mptr = m;

    while !tcpfrag_list_end(q, tp) {
        let i: c_int = (*ti)
            .ti_t
            .th_seq
            .wrapping_add((*ti).ti_i.ih_len as u32)
            .wrapping_sub((*q).ti_t.th_seq) as c_int;
        if i <= 0 {
            break;
        }
        if i < (*q).ti_i.ih_len as c_int {
            (*q).ti_t.th_seq = (*q).ti_t.th_seq.wrapping_add(i as u32);
            (*q).ti_i.ih_len = ((*q).ti_i.ih_len as c_int - i) as u16;
            m_adj((*q).ti_i.ih_mbuf.mptr, i);
            break;
        }
        q = tcpiphdr_next(q);
        m = (*tcpiphdr_prev(q)).ti_i.ih_mbuf.mptr;
        slirp_remque(tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void);
        m_free(m);
    }

    slirp_insque(
        tcpiphdr2qlink(ti) as *mut c_void,
        tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void,
    );

    tcp_reass_present(tp)
}

// port of the `present:` part of tcp_reass
unsafe fn tcp_reass_present(tp: *mut Tcpcb) -> c_int {
    let so: *mut Socket = (*tp).t_socket;

    if !((*tp).t_state >= 4) {
        return 0;
    }
    let mut ti: *mut Tcpiphdr = tcpfrag_list_first(tp);
    if tcpfrag_list_end(ti, tp) || (*ti).ti_t.th_seq != (*tp).rcv_nxt {
        return 0;
    }
    if (*tp).t_state == 3 && (*ti).ti_i.ih_len != 0 {
        return 0;
    }
    let mut flags: c_int;
    loop {
        (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add((*ti).ti_i.ih_len as u32);
        flags = ((*ti).ti_t.th_flags & 0x01) as c_int;
        slirp_remque(tcpiphdr2qlink(ti) as *mut c_void);
        let m: *mut Mbuf = (*ti).ti_i.ih_mbuf.mptr;
        ti = tcpiphdr_next(ti);
        if (*so).so_state & 0x010 != 0 {
            m_free(m);
        } else if (*so).so_emu != 0 {
            if tcp_emu(so, m) != 0 {
                sbappend(so, m);
            }
        } else {
            sbappend(so, m);
        }
        if !(ti != tp as *mut Tcpiphdr && (*ti).ti_t.th_seq == (*tp).rcv_nxt) {
            break;
        }
    }
    flags
}

// port of `static void tcp_dooptions(struct tcpcb *, u_char *, int, struct tcpiphdr *)`
unsafe fn tcp_dooptions(tp: *mut Tcpcb, cp_in: *mut c_uchar, cnt_in: c_int, ti: *mut Tcpiphdr) {
    let mut cp: *mut c_uchar = cp_in;
    let mut cnt: c_int = cnt_in;

    while cnt > 0 {
        let opt: c_int = *cp as c_int;
        if opt == 0 {
            break;
        }
        let optlen: c_int;
        if opt == 1 {
            optlen = 1;
        } else {
            optlen = *cp.offset(1) as c_int;
            if optlen <= 0 {
                break;
            }
        }
        if opt == 2 && optlen == 4 && ((*ti).ti_t.th_flags & 0x02) != 0 {
            let mut mss: u16 = 0;
            // memcpy((char *)&mss, (char *)cp + 2, sizeof(mss));
            core::ptr::copy_nonoverlapping(cp.offset(2), (&mut mss as *mut u16) as *mut u8, 2);
            let mss = ntohs(mss);
            tcp_mss(tp, mss as u32);
        }
        cnt -= optlen;
        cp = cp.offset(optlen as isize);
    }
}

// port of `static void tcp_xmit_timer(register struct tcpcb *, int)`
unsafe fn tcp_xmit_timer(tp: *mut Tcpcb, rtt: c_int) {
    if (*tp).t_srtt != 0 {
        let mut delta: i16 = (rtt - 1 - (((*tp).t_srtt as i32) >> 3)) as i16;
        (*tp).t_srtt = (*tp).t_srtt.wrapping_add(delta);
        if (*tp).t_srtt <= 0 {
            (*tp).t_srtt = 1;
        }
        if delta < 0 {
            delta = delta.wrapping_neg();
        }
        delta = (delta as i32 - (((*tp).t_rttvar as i32) >> 2)) as i16;
        (*tp).t_rttvar = (*tp).t_rttvar.wrapping_add(delta);
        if (*tp).t_rttvar <= 0 {
            (*tp).t_rttvar = 1;
        }
    } else {
        (*tp).t_srtt = (rtt << 3) as i16;
        (*tp).t_rttvar = (rtt << (2 - 1)) as i16;
    }
    (*tp).t_rtt = 0;
    (*tp).t_rxtshift = 0;
    {
        let mut rxtcur: i16 = ((((*tp).t_srtt as i32) >> 3) + (*tp).t_rttvar as i32) as i16;
        if rxtcur < (*tp).t_rttmin as i16 {
            rxtcur = (*tp).t_rttmin as i16;
        } else if rxtcur > 12 * 2 {
            rxtcur = 12 * 2;
        }
        (*tp).t_rxtcur = rxtcur;
    }
    (*tp).t_softerror = 0;
}

/* ------------------------------ helpers -------------------------------- */

// `goto cont_input:` -- (re)initialize the tcpcb and fall into
// `goto trimthenstep6; ... goto step6;` which ends in `return;`
unsafe fn cont_input(
    tp: *mut Tcpcb,
    slirp: *mut Slirp,
    ti: *mut Tcpiphdr,
    optp: *mut c_char,
    optlen: c_int,
    iss: c_int,
) -> c_int {
    tcp_template(tp);
    if !optp.is_null() {
        tcp_dooptions(tp, optp as *mut c_uchar, optlen, ti);
    }
    if iss != 0 {
        (*tp).iss = iss as u32;
    } else {
        (*tp).iss = (*slirp).tcp_iss;
    }
    (*slirp).tcp_iss = (*slirp).tcp_iss.wrapping_add((125 * 1024) / 2);
    (*tp).irs = (*ti).ti_t.th_seq;
    (*tp).snd_una = (*tp).iss;
    (*tp).snd_nxt = (*tp).iss;
    (*tp).snd_max = (*tp).iss;
    (*tp).snd_up = (*tp).iss;
    (*tp).rcv_adv = (*tp).irs.wrapping_add(1);
    (*tp).rcv_nxt = (*tp).irs.wrapping_add(1);
    (*tp).t_flags |= 0x0001;
    (*tp).t_state = 3;
    (*tp).t_timer[2] = 75 * 2;
    trimthenstep6(tp, ti);
    RES_RETURN
}

// `goto trimthenstep6:`
unsafe fn trimthenstep6(tp: *mut Tcpcb, ti: *mut Tcpiphdr) {
    (*ti).ti_t.th_seq = (*ti).ti_t.th_seq.wrapping_add(1);
    // the C comparison is unsigned here (u_short vs uint32_t)
    let wnd: u32 = (*tp).rcv_wnd;
    if (*ti).ti_i.ih_len as u32 > wnd {
        let todrop: c_int = ((*ti).ti_i.ih_len as u32).wrapping_sub(wnd) as c_int;
        m_adj((*ti).ti_i.ih_mbuf.mptr, todrop.wrapping_neg());
        (*ti).ti_i.ih_len = wnd as u16;
    }
    (*tp).snd_wl1 = (*ti).ti_t.th_seq.wrapping_sub(1);
    (*tp).rcv_up = (*ti).ti_t.th_seq;
}

/* ------------------------------ the port ------------------------------- */

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2698(
    m: *mut Mbuf,
    iphlen: c_int,
    inso: *mut Socket,
    ti: *mut Tcpiphdr,
    tp: *mut Tcpcb,
    so: *mut Socket,
    slirp: *mut Slirp,
    tiflags: c_int,
    tiwin: u64,
    sav: *mut Ip,
    optp: *mut c_char,
    optlen: c_int,
    off: c_int,
    iss: c_int,
    ex_ptr: *mut ExList,
) -> c_int {
    let _ = (iphlen, inso, tiwin, ex_ptr);
    let mut res: c_int;

    match (*tp).t_state {
        1 => {
            if tiflags & 0x04 != 0 {
                res = RES_DROP;
            } else if tiflags & 0x10 != 0 {
                res = RES_DROPWITHRESET;
            } else if (tiflags & 0x02) == 0 {
                res = RES_DROP;
            } else {
                res = NONE;
            }

            if res == NONE {
                if ((*so).so_faddr.s_addr & (*slirp).vnetwork_mask.s_addr)
                    == (*slirp).vnetwork_addr.s_addr
                {
                    if (*so).so_faddr.s_addr != (*slirp).vhost_addr.s_addr
                        && (*so).so_faddr.s_addr != (*slirp).vnameserver_addr.s_addr
                    {
                        let mut p: *mut ExList = (*slirp).exec_list;
                        while !p.is_null() {
                            if (*p).ex_fport == (*so).so_fport as c_int
                                && (*so).so_faddr.s_addr == (*p).ex_addr.s_addr
                            {
                                (*so).so_state |= 0x080;
                                break;
                            }
                            p = (*p).ex_next;
                        }
                        if (*so).so_state & 0x080 != 0 {
                            res = cont_input(tp, slirp, ti, optp, optlen, iss);
                        }
                    }
                }
            }

            if res == NONE && (*so).so_emu & 0x10 != 0 {
                (*so).so_emu &= !0x10u8;
                res = cont_input(tp, slirp, ti, optp, optlen, iss);
            }

            if res == NONE {
                if tcp_fconnect(so) == -1
                    && *__errno_location() != 115
                    && *__errno_location() != 11
                {
                    let mut code: c_uchar = 0;
                    if *__errno_location() == 111 {
                        tcp_respond(
                            tp,
                            ti,
                            m,
                            (*ti).ti_t.th_seq.wrapping_add(1),
                            0,
                            0x04 | 0x10,
                        );
                    } else {
                        if *__errno_location() == 113 {
                            code = 1;
                        }
                        (*ti).ti_t.th_seq = htonl((*ti).ti_t.th_seq);
                        (*ti).ti_t.th_ack = htonl((*ti).ti_t.th_ack);
                        (*ti).ti_t.th_win = htons((*ti).ti_t.th_win);
                        (*ti).ti_t.th_urp = htons((*ti).ti_t.th_urp);
                        // sizeof(struct tcpiphdr) + off - sizeof(struct tcphdr)
                        let delta: c_int = 40 + off - 20;
                        (*m).mh_data = (*m).mh_data.offset(-(delta as isize));
                        (*m).mh_len += delta;
                        *sav = *(*m).mh_data.cast::<Ip>();
                        icmp_error(m, 3, code, 0, strerror(*__errno_location()));
                    }
                    tcp_close(tp);
                    m_free(m);
                } else {
                    (*so).so_m = m;
                    (*so).so_ti = ti;
                    (*tp).t_timer[2] = 75 * 2;
                    (*tp).t_state = 3;
                }
                res = RES_RETURN;
            }

            if res == NONE {
                // goto cont_conn;
                if (*so).so_state & 0x001 != 0 {
                    // tp = tcp_close(tp); goto dropwithreset;
                    res = RES_CLOSE_DROPWITHRESET;
                } else {
                    res = cont_input(tp, slirp, ti, optp, optlen, iss);
                }
            }
        }
        2 => {
            if (tiflags & 0x10) != 0
                && ((*ti).ti_t.th_ack.wrapping_sub((*tp).iss) as c_int <= 0
                    || (*ti).ti_t.th_ack.wrapping_sub((*tp).snd_max) as c_int > 0)
            {
                res = RES_DROPWITHRESET;
            } else if tiflags & 0x04 != 0 {
                if tiflags & 0x10 != 0 {
                    tcp_drop(tp, 0);
                }
                res = RES_DROP;
            } else if (tiflags & 0x02) == 0 {
                res = RES_DROP;
            } else {
                if tiflags & 0x10 != 0 {
                    (*tp).snd_una = (*ti).ti_t.th_ack;
                    if ((*tp).snd_nxt.wrapping_sub((*tp).snd_una) as c_int) < 0 {
                        (*tp).snd_nxt = (*tp).snd_una;
                    }
                }

                (*tp).t_timer[0] = 0;
                (*tp).irs = (*ti).ti_t.th_seq;
                (*tp).rcv_adv = (*tp).irs.wrapping_add(1);
                (*tp).rcv_nxt = (*tp).irs.wrapping_add(1);
                (*tp).t_flags |= 0x0001;
                if tiflags & 0x10 != 0 && ((*tp).snd_una.wrapping_sub((*tp).iss) as c_int) > 0 {
                    soisfconnected(so);
                    (*tp).t_state = 4;
                    tcp_reass(tp, core::ptr::null_mut(), core::ptr::null_mut());
                    if (*tp).t_rtt != 0 {
                        tcp_xmit_timer(tp, (*tp).t_rtt as c_int);
                    }
                } else {
                    (*tp).t_state = 3;
                }

                // trimthenstep6: ... goto step6; -> return
                trimthenstep6(tp, ti);
                res = RES_RETURN;
            }
        }
        _ => {
            res = RES_FALLTHROUGH;
        }
    }

    res
}
}

// ── `todrop = tp->rcv_nxt - ti->ti_t.th_seq;` · slirp/tcp_input.c:719-719 · chunk 2699 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     todrop = tp->rcv_nxt - ti->ti_t.th_seq;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2699 {
// Computes `tp->rcv_nxt - ti->ti_t.th_seq` as an int (the C assignment to `todrop`).
// Both operands are tcp_seq == uint32_t; the subtraction wraps and the result is
// converted to int, which we mirror with a wrapping subtract and an `as` cast.

#[no_mangle]
pub extern "C" fn tcp_input__part2699(rcv_nxt: u32, th_seq: u32) -> core::ffi::c_int {
    rcv_nxt.wrapping_sub(th_seq) as core::ffi::c_int
}
}

// ── `if (todrop > 0)` · slirp/tcp_input.c:720-758 · chunk 2700 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (todrop > 0) {
//       if (tiflags & 0x02) {
//        tiflags &= ~0x02;
//        ti->ti_t.th_seq++;
//        if (ti->ti_t.th_urp > 1)
//         ti->ti_t.th_urp--;
//        else
//         tiflags &= ~0x20;
//        todrop--;
//       }
//     
//     
//     
//       if (todrop > ti->ti_i.ih_len
//           || (todrop == ti->ti_i.ih_len && (tiflags & 0x01) == 0)) {
//     
//     
//     
//     
//     
//        tiflags &= ~0x01;
//     
//     
//     
//     
//     
//        tp->t_flags |= 0x0001;
//        todrop = ti->ti_i.ih_len;
//       }
//       m_adj(m, todrop);
//       ti->ti_t.th_seq += todrop;
//       ti->ti_i.ih_len -= todrop;
//       if (ti->ti_t.th_urp > todrop)
//        ti->ti_t.th_urp -= todrop;
//       else {
//        tiflags &= ~0x20;
//        ti->ti_t.th_urp = 0;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2700 {
// Port of the "drop already-received data" block of slirp tcp_input().
// Mirrors slirp/ip.h (ipovly), slirp/tcp.h (tcphdr), slirp/tcpip.h (tcpiphdr).
use core::ffi::{c_int, c_ushort};
use core::ptr::{addr_of_mut, read_unaligned, write_unaligned};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

// struct ipovly (packed): 8 + 1 + 1 + 2 + 4 + 4 = 20 bytes
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr; the th_x2:4 / th_off:4 bitfield pair occupies one u_int.
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

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; }
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// Opaque; only ever passed through to m_adj().
#[repr(C)]
pub struct Mbuf {
    _private: [u8; 0],
}

extern "C" {
    fn m_adj(m: *mut Mbuf, len: c_int);
}

/// Returns the (possibly modified) tiflags value; `*todrop` is updated in place.
#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2700(
    m: *mut Mbuf,
    ti: *mut Tcpiphdr,
    t_flags: *mut c_ushort,
    tiflags: c_int,
    todrop: *mut c_int,
) -> c_int {
    let mut flags: c_int = tiflags;
    let mut td: c_int = *todrop;
    if td > 0 {
        let th_seq_p = addr_of_mut!((*ti).ti_t.th_seq);
        let th_urp_p = addr_of_mut!((*ti).ti_t.th_urp);
        let ih_len_p = addr_of_mut!((*ti).ti_i.ih_len);

        if (flags & 0x02) != 0 {
            flags &= !0x02;
            write_unaligned(th_seq_p, read_unaligned(th_seq_p).wrapping_add(1));
            let urp = read_unaligned(th_urp_p);
            if (urp as c_int) > 1 {
                write_unaligned(th_urp_p, urp.wrapping_sub(1));
            } else {
                flags &= !0x20;
            }
            td -= 1;
        }

        let ih_len: c_int = read_unaligned(ih_len_p) as c_int;
        if td > ih_len || (td == ih_len && (flags & 0x01) == 0) {
            flags &= !0x01;
            *t_flags |= 0x0001;
            td = ih_len;
        }

        m_adj(m, td);
        write_unaligned(th_seq_p, read_unaligned(th_seq_p).wrapping_add(td as u32));
        write_unaligned(
            ih_len_p,
            read_unaligned(ih_len_p).wrapping_sub(td as u16),
        );
        let urp = read_unaligned(th_urp_p);
        if (urp as c_int) > td {
            write_unaligned(th_urp_p, urp.wrapping_sub(td as u16));
        } else {
            flags &= !0x20;
            write_unaligned(th_urp_p, 0);
        }
    }
    *todrop = td;
    flags
}
}

// ── `if (so->so_state & 0x001) && tp->t_state > 5 && ti->ti_i.ih_len)` · slirp/tcp_input.c:763-767 · chunk 2701 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((so->so_state & 0x001) &&
//          tp->t_state > 5 && ti->ti_i.ih_len) {
//       tp = tcp_close(tp);
//       goto dropwithreset;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2701 {
// Ported piece of slirp tcp_input(): the "no fd reference with data" check
// that closes the tcpcb and jumps to dropwithreset.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_short};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct socket (leading part; only so_state is touched here).
#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut core::ffi::c_void,
    so_m: *mut core::ffi::c_void,
    so_ti: *mut core::ffi::c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
}

// struct tcpcb (leading part; only t_state is touched here).
#[repr(C)]
pub struct Tcpcb {
    seg_next: *mut core::ffi::c_void,
    seg_prev: *mut core::ffi::c_void,
    t_state: c_short,
}

// struct ipovly is __attribute__((packed)) in C.
#[repr(C, packed)]
pub struct Ipovly {
    ih_mbuf: *mut core::ffi::c_void,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: InAddr,
    ih_dst: InAddr,
}

// struct tcpiphdr { struct ipovly ti_i; struct tcphdr ti_t; };
#[repr(C)]
pub struct Tcpiphdr {
    ti_i: Ipovly,
}

extern "C" {
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
}

/// Returns the (possibly NULL) tcpcb; `*drop_out` is set to 1 when the
/// caller must `goto dropwithreset`.
#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2701(
    so: *mut Socket,
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
    drop_out: *mut c_int,
) -> *mut Tcpcb {
    if !drop_out.is_null() {
        *drop_out = 0;
    }
    if so.is_null() || tp.is_null() || ti.is_null() {
        return tp;
    }
    if ((*so).so_state & 0x001) != 0
        && (*tp).t_state > 5
        && (*ti).ti_i.ih_len != 0
    {
        if !drop_out.is_null() {
            *drop_out = 1;
        }
        return tcp_close(tp);
    }
    tp
}
}

// ── `todrop = (ti->ti_t.th_seq+ti->ti_i.ih_len) - (tp->rcv_nxt+tp->rcv_wnd);` · slirp/tcp_input.c:773-773 · chunk 2702 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     todrop = (ti->ti_t.th_seq+ti->ti_i.ih_len) - (tp->rcv_nxt+tp->rcv_wnd);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2702 {
// Compute: todrop = (ti->ti_t.th_seq + ti->ti_i.ih_len) - (tp->rcv_nxt + tp->rcv_wnd)
// All arithmetic is done in unsigned 32-bit (uint16_t is promoted to int, then
// added to a uint32_t, giving uint32_t), the result being stored into an int.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_input__part2702(
    th_seq: u32,
    ih_len: u16,
    rcv_nxt: u32,
    rcv_wnd: u32,
) -> c_int {
    let left = th_seq.wrapping_add(ih_len as u32);
    let right = rcv_nxt.wrapping_add(rcv_wnd);
    left.wrapping_sub(right) as c_int
}
}

// ── `if (todrop > 0)` · slirp/tcp_input.c:774-805 · chunk 2703 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (todrop > 0) {
//       if (todrop >= ti->ti_i.ih_len) {
//     
//     
//     
//     
//     
//     
//        if (tiflags & 0x02 &&
//            tp->t_state == 10 &&
//            ((int)((ti->ti_t.th_seq)-(tp->rcv_nxt)) > 0)) {
//         iss = tp->rcv_nxt + (125*1024);
//         tp = tcp_close(tp);
//         goto findso;
//        }
//     
//     
//     
//     
//     
//     
//     
//        if (tp->rcv_wnd == 0 && ti->ti_t.th_seq == tp->rcv_nxt) {
//         tp->t_flags |= 0x0001;
//        } else {
//         goto dropafterack;
//        }
//       }
//       m_adj(m, -todrop);
//       ti->ti_i.ih_len -= todrop;
//       tiflags &= ~(0x08|0x01);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2703 {
// Port of the "drop trailing data after window" block of slirp's tcp_input().
#![allow(non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_short, c_ushort};
use core::ptr::{addr_of, addr_of_mut};

pub type tcp_seq = u32;

// Opaque types only ever handled through pointers.
#[repr(C)]
pub struct mbuf {
    pub _opaque: u8,
}

#[repr(C)]
pub struct socket {
    pub _opaque: u8,
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
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
pub struct tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: tcp_seq,
    pub th_ack: tcp_seq,
    // u_int th_x2:4, th_off:4;  (one byte on little endian)
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
    pub t_force: i8,
    pub t_flags: c_ushort,
    pub t_template: tcpiphdr,
    pub t_socket: *mut socket,
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
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub t_idle: c_short,
    pub t_rtt: c_short,
    pub t_rtseq: tcp_seq,
    pub t_srtt: c_short,
    pub t_rttvar: c_short,
    pub t_rttmin: c_ushort,
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
    pub last_ack_sent: tcp_seq,
}

// In/out parameters (tp, tiflags and iss are `register` locals in the caller,
// so they are handed over through this small structure).
#[repr(C)]
pub struct tcp_input_part2703_io {
    pub tp: *mut tcpcb,
    pub tiflags: c_int,
    pub iss: c_int,
}

extern "C" {
    fn tcp_close(tp: *mut tcpcb) -> *mut tcpcb;
    fn m_adj(m: *mut mbuf, diff: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2703(
    m: *mut mbuf,
    ti: *mut tcpiphdr,
    io: *mut tcp_input_part2703_io,
    todrop: c_int,
) -> c_int {
    if todrop > 0 {
        let ih_len: c_int = addr_of!((*ti).ti_i.ih_len).read_unaligned() as c_int;
        if todrop >= ih_len {
            let tp: *mut tcpcb = (*io).tp;
            let tiflags: c_int = (*io).tiflags;
            let t_state: c_int = addr_of!((*tp).t_state).read_unaligned() as c_int;
            let rcv_nxt: tcp_seq = addr_of!((*tp).rcv_nxt).read_unaligned();
            let th_seq: tcp_seq = addr_of!((*ti).ti_t.th_seq).read_unaligned();

            if (tiflags & 0x02) != 0
                && t_state == 10
                && (th_seq.wrapping_sub(rcv_nxt) as c_int) > 0
            {
                (*io).iss = rcv_nxt.wrapping_add(125 * 1024) as c_int;
                (*io).tp = tcp_close(tp);
                return 1; // goto findso
            }

            let rcv_wnd: u32 = addr_of!((*tp).rcv_wnd).read_unaligned();
            if rcv_wnd == 0 && th_seq == rcv_nxt {
                let flags: c_ushort = addr_of!((*tp).t_flags).read_unaligned();
                addr_of_mut!((*tp).t_flags).write_unaligned(flags | 0x0001);
            } else {
                return 2; // goto dropafterack
            }
        }

        m_adj(m, todrop.wrapping_neg());

        let cur_len: c_int = addr_of!((*ti).ti_i.ih_len).read_unaligned() as c_int;
        addr_of_mut!((*ti).ti_i.ih_len)
            .write_unaligned(cur_len.wrapping_sub(todrop) as u16);
        (*io).tiflags &= !(0x08 | 0x01);
    }
    0
}
}

// ── `if (tiflags&0x04)` · slirp/tcp_input.c:817-833 · chunk 2704 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tiflags&0x04) switch (tp->t_state) {
//     
//      case 3:
//      case 4:
//      case 6:
//      case 9:
//      case 5:
//       tp->t_state = 0;
//                     tcp_close(tp);
//       goto drop;
//     
//      case 7:
//      case 8:
//      case 10:
//                     tcp_close(tp);
//       goto drop;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2704 {
#![allow(dead_code, non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_short, c_void};

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
    /* u_int th_x2:4, th_off:4 -> one 4-byte bitfield unit */
    pub th_bits: u32,
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

extern "C" {
    fn tcp_close(tp: *mut tcpcb) -> *mut tcpcb;
}

/// Handles the RST-bit state switch inside tcp_input.
/// Returns 1 when the C code must `goto drop`, 0 when it falls through.
#[no_mangle]
pub extern "C" fn tcp_input__part2704(tiflags: c_int, tp: *mut tcpcb) -> c_int {
    unsafe {
        if tiflags & 0x04 == 0 {
            return 0;
        }
        if tp.is_null() {
            return 0;
        }
        let state = (*tp).t_state;
        match state {
            3 | 4 | 6 | 9 | 5 => {
                (*tp).t_state = 0;
                tcp_close(tp);
                1
            }
            7 | 8 | 10 => {
                tcp_close(tp);
                1
            }
            _ => 0,
        }
    }
}
}

// ── `if (tiflags & 0x02)` · slirp/tcp_input.c:839-842 · chunk 2705 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (tiflags & 0x02) {
//       tp = tcp_drop(tp,0);
//       goto dropwithreset;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2705 {
// Ported piece of tcp_input: dropping the connection when a SYN arrives
// (handled here by returning the new tcpcb pointer; the caller keeps the
// `goto dropwithreset` control flow driven by tiflags).

#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: core::ffi::c_short,
}

extern "C" {
    fn tcp_drop(tp: *mut tcpcb, err: core::ffi::c_int) -> *mut tcpcb;
}

#[no_mangle]
pub extern "C" fn tcp_input__part2705(
    tiflags: core::ffi::c_int,
    tp: *mut tcpcb,
) -> *mut tcpcb {
    unsafe {
        if tiflags & 0x02 != 0 {
            return tcp_drop(tp, 0);
        }
        tp
    }
}
}

// ── `if (tiflags & 0x10) == 0)` · slirp/tcp_input.c:847-847 · chunk 2706 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((tiflags & 0x10) == 0) goto drop;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2706 {
// Portion of tcp_input: check that the ACK flag is set.
// Returns 1 when (tiflags & TH_ACK) != 0 (continue), 0 when the packet
// must be dropped (C: `goto drop`).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_input__part2706(tiflags: c_int) -> c_int {
    if (tiflags & 0x10) == 0 {
        0
    } else {
        1
    }
}
}

// ── `switch (tp->t_state)` · slirp/tcp_input.c:852-1092 · chunk 2707 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (tp->t_state) {
//     
//     
//     
//     
//     
//      case 3:
//     
//       if (((int)((tp->snd_una)-(ti->ti_t.th_ack)) > 0) ||
//           ((int)((ti->ti_t.th_ack)-(tp->snd_max)) > 0))
//        goto dropwithreset;
//       tp->t_state = 4;
//     
//     
//     
//     
//     
//     
//     
//       tp->snd_una=ti->ti_t.th_ack;
//       if (so->so_state & 0x080) {
//     
//         ret = tcp_ctl(so);
//         if (ret == 1) {
//           soisfconnected(so);
//           so->so_state &= ~0x080;
//         } else if (ret == 2) {
//           so->so_state &= 0xf000;
//           so->so_state |= 0x001;
//         } else {
//           needoutput = 1;
//           tp->t_state = 6;
//         }
//       } else {
//         soisfconnected(so);
//       }
//     
//       (void) tcp_reass(tp, (struct tcpiphdr *)0, (struct mbuf *)0);
//       tp->snd_wl1 = ti->ti_t.th_seq - 1;
//     
//       goto synrx_to_est;
//      case 4:
//      case 6:
//      case 9:
//      case 5:
//      case 7:
//      case 8:
//      case 10:
//     
//       if (((int)((ti->ti_t.th_ack)-(tp->snd_una)) <= 0)) {
//        if (ti->ti_i.ih_len == 0 && tiwin == tp->snd_wnd) {
//         
//                                    ;
//         if (tp->t_timer[0] == 0 ||
//             ti->ti_t.th_ack != tp->snd_una)
//          tp->t_dupacks = 0;
//         else if (++tp->t_dupacks == 3) {
//          tcp_seq onxt = tp->snd_nxt;
//          u_int win =
//              ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->snd_cwnd)) / 2 /
//           tp->t_maxseg;
//     
//          if (win < 2)
//           win = 2;
//          tp->snd_ssthresh = win * tp->t_maxseg;
//          tp->t_timer[0] = 0;
//          tp->t_rtt = 0;
//          tp->snd_nxt = ti->ti_t.th_ack;
//          tp->snd_cwnd = tp->t_maxseg;
//          (void) tcp_output(tp);
//          tp->snd_cwnd = tp->snd_ssthresh +
//                 tp->t_maxseg * tp->t_dupacks;
//          if (((int)((onxt)-(tp->snd_nxt)) > 0))
//           tp->snd_nxt = onxt;
//          goto drop;
//         } else if (tp->t_dupacks > 3) {
//          tp->snd_cwnd += tp->t_maxseg;
//          (void) tcp_output(tp);
//          goto drop;
//         }
//        } else
//         tp->t_dupacks = 0;
//        break;
//       }
//      synrx_to_est:
//     
//     
//     
//     
//       if (tp->t_dupacks > 3 &&
//           tp->snd_cwnd > tp->snd_ssthresh)
//        tp->snd_cwnd = tp->snd_ssthresh;
//       tp->t_dupacks = 0;
//       if (((int)((ti->ti_t.th_ack)-(tp->snd_max)) > 0)) {
//        goto dropafterack;
//       }
//       acked = ti->ti_t.th_ack - tp->snd_una;
//       if (tp->t_rtt && ((int)((ti->ti_t.th_ack)-(tp->t_rtseq)) > 0))
//        tcp_xmit_timer(tp,tp->t_rtt);
//     
//     
//     
//     
//     
//     
//     
//       if (ti->ti_t.th_ack == tp->snd_max) {
//        tp->t_timer[0] = 0;
//        needoutput = 1;
//       } else if (tp->t_timer[1] == 0)
//        tp->t_timer[0] = tp->t_rxtcur;
//     
//     
//     
//     
//     
//     
//     
//       {
//         register u_int cw = tp->snd_cwnd;
//         register u_int incr = tp->t_maxseg;
//     
//         if (cw > tp->snd_ssthresh)
//           incr = incr * incr / cw;
//         tp->snd_cwnd = ((cw + incr) < (65535<<tp->snd_scale) ? (cw + incr) : (65535<<tp->snd_scale));
//       }
//       if (acked > so->so_snd.sb_cc) {
//        tp->snd_wnd -= so->so_snd.sb_cc;
//        sbdrop(&so->so_snd, (int )so->so_snd.sb_cc);
//        ourfinisacked = 1;
//       } else {
//        sbdrop(&so->so_snd, acked);
//        tp->snd_wnd -= acked;
//        ourfinisacked = 0;
//       }
//       tp->snd_una = ti->ti_t.th_ack;
//       if (((int)((tp->snd_nxt)-(tp->snd_una)) < 0))
//        tp->snd_nxt = tp->snd_una;
//     
//       switch (tp->t_state) {
//     
//     
//     
//     
//     
//     
//       case 6:
//        if (ourfinisacked) {
//     
//     
//     
//     
//     
//     
//     
//         if (so->so_state & 0x008) {
//          tp->t_timer[3] = (8 * ( 75*2));
//         }
//         tp->t_state = 9;
//        }
//        break;
//     
//     
//     
//     
//     
//     
//     
//       case 7:
//        if (ourfinisacked) {
//         tp->t_state = 10;
//         tcp_canceltimers(tp);
//         tp->t_timer[3] = 2 * ( 5*2);
//        }
//        break;
//     
//     
//     
//     
//     
//     
//     
//       case 8:
//        if (ourfinisacked) {
//                                     tcp_close(tp);
//         goto drop;
//        }
//        break;
//     
//     
//     
//     
//     
//     
//       case 10:
//        tp->t_timer[3] = 2 * ( 5*2);
//        goto dropafterack;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2707 {
// Port of a piece of slirp's tcp_input(): the ACK-processing switch.
// The C side keeps the goto labels; this function reports where control
// has to go: 0 = fall through to step6, 1 = dropwithreset, 2 = drop,
// 3 = dropafterack.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    _priv: [u8; 0],
}

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
// struct ipovly { ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Ipovly {
    pub ih_mbuf: *mut Mbuf,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// struct tcphdr: u_int th_x2:4, th_off:4; -> the bitfields share one byte
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

// struct qlink { void *next, *prev; }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
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
    pub extra: *mut c_void,
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
    pub snd_scale: c_uchar,
    pub rcv_scale: c_uchar,
    pub request_r_scale: c_uchar,
    pub requested_s_scale: c_uchar,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

extern "C" {
    fn tcp_ctl(so: *mut Socket) -> c_int;
    fn soisfconnected(so: *mut Socket);
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn tcp_canceltimers(tp: *mut Tcpcb);
    fn tcp_close(tp: *mut Tcpcb) -> *mut Tcpcb;
    fn sbdrop(sb: *mut Sbuf, num: c_int);
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: c_int);
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> c_int;
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
}

#[inline]
fn seq_lt(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) < 0
}

#[inline]
fn seq_leq(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) <= 0
}

#[inline]
fn seq_gt(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) > 0
}

const QLINK_SIZE: usize = core::mem::size_of::<Qlink>();

// tcpiphdr2qlink(), as a byte address
#[inline]
unsafe fn q_addr_of(t: *mut Tcpiphdr) -> *mut u8 {
    (t as *mut u8).offset(-(QLINK_SIZE as isize))
}

// qlink2tcpiphdr()
#[inline]
unsafe fn ti_of_q(q: *mut u8) -> *mut Tcpiphdr {
    (q).offset(QLINK_SIZE as isize) as *mut Tcpiphdr
}

#[inline]
unsafe fn q_of_ti(t: *mut Tcpiphdr) -> *mut Qlink {
    q_addr_of(t) as *mut Qlink
}

// struct quehead { struct quehead *qh_link, *qh_rlink; } -- slirp_insque()
unsafe fn rs_insque(element: *mut Qlink, head: *mut Qlink) {
    (*element).next = (*head).next;
    (*head).next = element as *mut c_void;
    (*element).prev = head as *mut c_void;
    let link: *mut Qlink = (*element).next.cast::<Qlink>();
    (*link).prev = element as *mut c_void;
}

// slirp_remque()
unsafe fn rs_remque(element: *mut Qlink) {
    let link: *mut Qlink = (*element).next.cast::<Qlink>();
    let rlink: *mut Qlink = (*element).prev.cast::<Qlink>();
    (*link).prev = (*element).prev;
    (*rlink).next = (*element).next;
    (*element).prev = core::ptr::null_mut();
}

// static int tcp_reass(register struct tcpcb *tp, register struct tcpiphdr *ti,
//                      struct mbuf *m) -- ported, it has no linker symbol
unsafe fn rs_tcp_reass(tp: *mut Tcpcb, ti_in: *mut Tcpiphdr, m_in: *mut Mbuf) -> c_int {
    let so = (*tp).t_socket;
    let mut m = m_in;
    let ti = ti_in;
    let sentinel: *mut u8 = tp as *mut u8;

    if ti.is_null() {
        return rs_tcp_reass_present(tp, so);
    }

    // for (q = tcpfrag_list_first(tp->seg_next); !tcpfrag_list_end(q, tp);
    //      q = tcpiphdr_next(q))
    //     if (SEQ_GT(q->ti_seq, ti->ti_seq)) break;
    let mut q: *mut Tcpiphdr = ti_of_q((*tp).seg_next as *mut u8);
    while q_addr_of(q) != sentinel {
        if ((*q).ti_t.th_seq.wrapping_sub((*ti).ti_t.th_seq) as i32) > 0 {
            break;
        }
        let nxt: *mut Qlink = (*q_of_ti(q)).next.cast::<Qlink>();
        q = ti_of_q(nxt as *mut u8);
    }

    // if (!tcpfrag_list_end(tcpiphdr_prev(q), tp)) { ... }
    let qprev: *mut Qlink = (*q_of_ti(q)).prev.cast::<Qlink>();
    let qprev_ti: *mut Tcpiphdr = ti_of_q(qprev as *mut u8);
    if q_addr_of(qprev_ti) != sentinel {
        let i: c_int = (*qprev_ti)
            .ti_t
            .th_seq
            .wrapping_add((*qprev_ti).ti_i.ih_len as u32)
            .wrapping_sub((*ti).ti_t.th_seq) as c_int;
        if i > 0 {
            if i >= (*ti).ti_i.ih_len as c_int {
                m_free(m);
                return rs_tcp_reass_present(tp, so);
            }
            m_adj(m, i);
            (*ti).ti_i.ih_len = ((*ti).ti_i.ih_len as i32 - i) as u16;
            (*ti).ti_t.th_seq = (*ti).ti_t.th_seq.wrapping_add(i as u32);
        }
        let nxt: *mut Qlink = (*q_of_ti(qprev_ti)).next.cast::<Qlink>();
        q = ti_of_q(nxt as *mut u8);
    }

    (*ti).ti_i.ih_mbuf = m;

    while q_addr_of(q) != sentinel {
        let i: c_int = (*ti)
            .ti_t
            .th_seq
            .wrapping_add((*ti).ti_i.ih_len as u32)
            .wrapping_sub((*q).ti_t.th_seq) as c_int;
        if i <= 0 {
            break;
        }
        if i < (*q).ti_i.ih_len as c_int {
            (*q).ti_t.th_seq = (*q).ti_t.th_seq.wrapping_add(i as u32);
            (*q).ti_i.ih_len = ((*q).ti_i.ih_len as i32 - i) as u16;
            m_adj((*q).ti_i.ih_mbuf, i);
            break;
        }
        let nxt: *mut Qlink = (*q_of_ti(q)).next.cast::<Qlink>();
        q = ti_of_q(nxt as *mut u8);
        let prv: *mut Qlink = (*q_of_ti(q)).prev.cast::<Qlink>();
        let prv_ti: *mut Tcpiphdr = ti_of_q(prv as *mut u8);
        m = (*prv_ti).ti_i.ih_mbuf;
        rs_remque(prv);
        m_free(m);
    }

    {
        let prv: *mut Qlink = (*q_of_ti(q)).prev.cast::<Qlink>();
        rs_insque(q_of_ti(ti), prv);
    }

    rs_tcp_reass_present(tp, so)
}

unsafe fn rs_tcp_reass_present(tp: *mut Tcpcb, so: *mut Socket) -> c_int {
    let sentinel: *mut u8 = tp as *mut u8;
    let mut m: *mut Mbuf;

    // present:
    if !((*tp).t_state >= 4) {
        return 0;
    }
    let mut ti: *mut Tcpiphdr = ti_of_q((*tp).seg_next as *mut u8);
    if q_addr_of(ti) == sentinel || (*ti).ti_t.th_seq != (*tp).rcv_nxt {
        return 0;
    }
    if (*tp).t_state == 3 && (*ti).ti_i.ih_len != 0 {
        return 0;
    }
    let mut flags: c_int;
    loop {
        (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add((*ti).ti_i.ih_len as u32);
        flags = ((*ti).ti_t.th_flags & 0x01) as c_int;
        rs_remque(q_of_ti(ti));
        m = (*ti).ti_i.ih_mbuf;
        let nxt: *mut Qlink = (*q_of_ti(ti)).next.cast::<Qlink>();
        ti = ti_of_q(nxt as *mut u8);
        if (*so).so_state & 0x010 != 0 {
            m_free(m);
        } else if (*so).so_emu != 0 {
            if tcp_emu(so, m) != 0 {
                sbappend(so, m);
            }
        } else {
            sbappend(so, m);
        }
        if ti == tp as *mut Tcpiphdr || (*ti).ti_t.th_seq != (*tp).rcv_nxt {
            break;
        }
    }
    flags
}

// static void tcp_xmit_timer(register struct tcpcb *tp, int rtt)
unsafe fn rs_tcp_xmit_timer(tp: *mut Tcpcb, rtt: c_int) {
    if (*tp).t_srtt != 0 {
        let mut delta: i16 = (rtt - 1 - (((*tp).t_srtt as i32) >> 3)) as i16;
        let srtt = (*tp).t_srtt.wrapping_add(delta);
        (*tp).t_srtt = srtt;
        if srtt <= 0 {
            (*tp).t_srtt = 1;
        }
        if delta < 0 {
            delta = delta.wrapping_neg();
        }
        delta = (delta as i32 - (((*tp).t_rttvar as i32) >> 2)) as i16;
        let rttvar = (*tp).t_rttvar.wrapping_add(delta);
        (*tp).t_rttvar = rttvar;
        if rttvar <= 0 {
            (*tp).t_rttvar = 1;
        }
    } else {
        (*tp).t_srtt = (rtt << 3) as i16;
        (*tp).t_rttvar = (rtt << 1) as i16;
    }
    (*tp).t_rtt = 0;
    (*tp).t_rxtshift = 0;

    // TCPT_RANGESET(tp->t_rxtcur, TCP_REXMTVAL(tp), (short)tp->t_rttmin,
    //               TCPTV_REXMTMAX)
    let mut tv: i16 = ((((*tp).t_srtt as i32) >> 3) + (*tp).t_rttvar as i32) as i16;
    let tvmin: i16 = (*tp).t_rttmin as i16;
    if tv < tvmin {
        tv = tvmin;
    } else if tv > (12 * 2) {
        tv = 12 * 2;
    }
    (*tp).t_rxtcur = tv;
    (*tp).t_softerror = 0;
}

#[no_mangle]
pub extern "C" fn tcp_input__part2707(
    tp: *mut Tcpcb,
    so: *mut Socket,
    ti: *mut Tcpiphdr,
    tiwin: c_ulong,
    needoutput: *mut c_int,
    acked_out: *mut c_int,
    ourfinisacked_out: *mut c_int,
) -> c_int {
    unsafe {
        let ack: u32 = (*ti).ti_t.th_ack;
        let seglen: u16 = (*ti).ti_i.ih_len;
        let state: c_short = (*tp).t_state;
        let mut reach_synrx = false;

        match state {
            3 => {
                // TCPS_SYN_RECEIVED
                if seq_gt((*tp).snd_una, ack) || seq_gt(ack, (*tp).snd_max) {
                    return 1; // goto dropwithreset
                }
                (*tp).t_state = 4; // TCPS_ESTABLISHED

                (*tp).snd_una = ack;
                if (*so).so_state & 0x080 != 0 {
                    // SS_CTL
                    let ret = tcp_ctl(so);
                    if ret == 1 {
                        soisfconnected(so);
                        (*so).so_state &= !0x080;
                    } else if ret == 2 {
                        (*so).so_state &= 0xf000;
                        (*so).so_state |= 0x001;
                    } else {
                        *needoutput = 1;
                        (*tp).t_state = 6; // TCPS_FIN_WAIT_1
                    }
                } else {
                    soisfconnected(so);
                }

                rs_tcp_reass(tp, core::ptr::null_mut(), core::ptr::null_mut());
                (*tp).snd_wl1 = (*ti).ti_t.th_seq.wrapping_sub(1);

                reach_synrx = true; // falls into "goto synrx_to_est"
            }
            4 | 5 | 6 | 7 | 8 | 9 | 10 => {
                if seq_leq(ack, (*tp).snd_una) {
                    if seglen == 0 && tiwin as u32 == (*tp).snd_wnd {
                        if (*tp).t_timer[0] == 0 || ack != (*tp).snd_una {
                            (*tp).t_dupacks = 0;
                        } else {
                            (*tp).t_dupacks = (*tp).t_dupacks.wrapping_add(1);
                            if (*tp).t_dupacks == 3 {
                                let onxt: u32 = (*tp).snd_nxt;
                                let mseg: u32 = (*tp).t_maxseg as u32;
                                let lo: u32 = if (*tp).snd_wnd < (*tp).snd_cwnd {
                                    (*tp).snd_wnd
                                } else {
                                    (*tp).snd_cwnd
                                };
                                let mut win: u32 = if mseg != 0 { lo / 2 / mseg } else { 0 };
                                if win < 2 {
                                    win = 2;
                                }
                                (*tp).snd_ssthresh = win.wrapping_mul(mseg);
                                (*tp).t_timer[0] = 0;
                                (*tp).t_rtt = 0;
                                (*tp).snd_nxt = ack;
                                (*tp).snd_cwnd = mseg;
                                tcp_output(tp);
                                (*tp).snd_cwnd = (*tp)
                                    .snd_ssthresh
                                    .wrapping_add(mseg.wrapping_mul((*tp).t_dupacks as u32));
                                if seq_gt(onxt, (*tp).snd_nxt) {
                                    (*tp).snd_nxt = onxt;
                                }
                                return 2; // goto drop
                            } else if (*tp).t_dupacks > 3 {
                                let mseg: u32 = (*tp).t_maxseg as u32;
                                (*tp).snd_cwnd = (*tp).snd_cwnd.wrapping_add(mseg);
                                tcp_output(tp);
                                return 2; // goto drop
                            }
                        }
                    } else {
                        (*tp).t_dupacks = 0;
                    }
                    // break out of the switch: fall to step6
                } else {
                    reach_synrx = true;
                }
            }
            _ => {}
        }

        if !reach_synrx {
            return 0; // step6
        }

        // synrx_to_est:
        if (*tp).t_dupacks > 3 && (*tp).snd_cwnd > (*tp).snd_ssthresh {
            (*tp).snd_cwnd = (*tp).snd_ssthresh;
        }
        (*tp).t_dupacks = 0;
        if seq_gt(ack, (*tp).snd_max) {
            return 3; // goto dropafterack
        }
        let acked: c_int = ack.wrapping_sub((*tp).snd_una) as c_int;
        *acked_out = acked;
        if (*tp).t_rtt != 0 && seq_gt(ack, (*tp).t_rtseq) {
            rs_tcp_xmit_timer(tp, (*tp).t_rtt as c_int);
        }

        if ack == (*tp).snd_max {
            (*tp).t_timer[0] = 0;
            *needoutput = 1;
        } else if (*tp).t_timer[1] == 0 {
            (*tp).t_timer[0] = (*tp).t_rxtcur;
        }

        {
            let cw: u32 = (*tp).snd_cwnd;
            let mut incr: u32 = (*tp).t_maxseg as u32;
            if cw > (*tp).snd_ssthresh {
                incr = if cw != 0 {
                    incr.wrapping_mul(incr) / cw
                } else {
                    incr
                };
            }
            let maxwin: u32 = (65535i32).wrapping_shl((*tp).snd_scale as u32) as u32;
            let cand: u32 = cw.wrapping_add(incr);
            (*tp).snd_cwnd = if cand < maxwin { cand } else { maxwin };
        }

        let ourfinisacked: c_int;
        if (acked as u32) > (*so).so_snd.sb_cc {
            (*tp).snd_wnd = (*tp).snd_wnd.wrapping_sub((*so).so_snd.sb_cc);
            sbdrop(
                core::ptr::addr_of_mut!((*so).so_snd),
                (*so).so_snd.sb_cc as c_int,
            );
            ourfinisacked = 1;
        } else {
            sbdrop(core::ptr::addr_of_mut!((*so).so_snd), acked);
            (*tp).snd_wnd = (*tp).snd_wnd.wrapping_sub(acked as u32);
            ourfinisacked = 0;
        }
        *ourfinisacked_out = ourfinisacked;
        (*tp).snd_una = ack;
        if seq_lt((*tp).snd_nxt, (*tp).snd_una) {
            (*tp).snd_nxt = (*tp).snd_una;
        }

        match (*tp).t_state {
            6 => {
                // TCPS_FIN_WAIT_1
                if ourfinisacked != 0 {
                    if (*so).so_state & 0x008 != 0 {
                        // SS_FCANTRCVMORE
                        (*tp).t_timer[3] = 8 * (75 * 2);
                    }
                    (*tp).t_state = 9; // TCPS_FIN_WAIT_2
                }
            }
            7 => {
                // TCPS_CLOSING
                if ourfinisacked != 0 {
                    (*tp).t_state = 10; // TCPS_TIME_WAIT
                    tcp_canceltimers(tp);
                    (*tp).t_timer[3] = 2 * (5 * 2);
                }
            }
            8 => {
                // TCPS_LAST_ACK
                if ourfinisacked != 0 {
                    tcp_close(tp);
                    return 2; // goto drop
                }
            }
            10 => {
                // TCPS_TIME_WAIT
                (*tp).t_timer[3] = 2 * (5 * 2);
                return 3; // goto dropafterack
            }
            _ => {}
        }

        0
    }
}
}

// ── `step6: if ((tiflags & 0x10) && (((int)((tp->snd_wl1)-(ti->ti_t.th_seq)) < 0) || (tp->snd_wl1 ==…` · slirp/tcp_input.c:1094-1109 · chunk 2708 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     step6:
//     
//     
//     
//     
//      if ((tiflags & 0x10) &&
//          (((int)((tp->snd_wl1)-(ti->ti_t.th_seq)) < 0) ||
//          (tp->snd_wl1 == ti->ti_t.th_seq && (((int)((tp->snd_wl2)-(ti->ti_t.th_ack)) < 0) ||
//          (tp->snd_wl2 == ti->ti_t.th_ack && tiwin > tp->snd_wnd))))) {
//       tp->snd_wnd = tiwin;
//       tp->snd_wl1 = ti->ti_t.th_seq;
//       tp->snd_wl2 = ti->ti_t.th_ack;
//       if (tp->snd_wnd > tp->max_sndwnd)
//        tp->max_sndwnd = tp->snd_wnd;
//       needoutput = 1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2708 {
// Port of the "step6" block of tcp_input (slirp/tcp_input.c).
// Updates the send window from the incoming segment when the segment is
// newer than what we have seen so far, and flags that output is needed.

use core::ffi::{c_char, c_int, c_ulong};

// struct mbuf_ptr { struct mbuf *mptr; } __attribute__((packed));
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
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

// struct tcphdr; th_x2:4 / th_off:4 bitfields live in one u_int unit at
// byte offset 12, mirrored here as a single byte.
#[repr(C)]
pub struct Tcphdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32, // tcp_seq
    pub th_ack: u32, // tcp_seq
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

// struct tcpcb, field for field in order.
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
    pub t_socket: *mut core::ffi::c_void,
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub snd_up: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
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
pub extern "C" fn tcp_input__part2708(
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
    tiwin: c_ulong,
    tiflags: c_int,
    needoutput: *mut c_int,
) {
    unsafe {
        let th_seq: u32 = (*ti).ti_t.th_seq;
        let th_ack: u32 = (*ti).ti_t.th_ack;

        let newer = ((*tp).snd_wl1.wrapping_sub(th_seq) as c_int) < 0
            || ((*tp).snd_wl1 == th_seq
                && (((*tp).snd_wl2.wrapping_sub(th_ack) as c_int) < 0
                    || ((*tp).snd_wl2 == th_ack
                        && tiwin > (*tp).snd_wnd as c_ulong)));

        if (tiflags & 0x10) != 0 && newer {
            (*tp).snd_wnd = tiwin as u32;
            (*tp).snd_wl1 = th_seq;
            (*tp).snd_wl2 = th_ack;
            if (*tp).snd_wnd > (*tp).max_sndwnd {
                (*tp).max_sndwnd = (*tp).snd_wnd;
            }
            *needoutput = 1;
        }
    }
}
}

// ── `if (tiflags & 0x20) && ti->ti_t.th_urp && ((tp->t_state) >= 10) == 0) … else` · slirp/tcp_input.c:1114-1155 · chunk 2709 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((tiflags & 0x20) && ti->ti_t.th_urp &&
//          ((tp->t_state) >= 10) == 0) {
//     
//     
//     
//     
//     
//     
//       if (ti->ti_t.th_urp + so->so_rcv.sb_cc > so->so_rcv.sb_datalen) {
//        ti->ti_t.th_urp = 0;
//        tiflags &= ~0x20;
//        goto dodata;
//       }
//       if (((int)((ti->ti_t.th_seq+ti->ti_t.th_urp)-(tp->rcv_up)) > 0)) {
//        tp->rcv_up = ti->ti_t.th_seq + ti->ti_t.th_urp;
//        so->so_urgc = so->so_rcv.sb_cc +
//         (tp->rcv_up - tp->rcv_nxt);
//        tp->rcv_up = ti->ti_t.th_seq + ti->ti_t.th_urp;
//     
//       }
//      } else
//     
//     
//     
//     
//     
//       if (((int)((tp->rcv_nxt)-(tp->rcv_up)) > 0))
//        tp->rcv_up = tp->rcv_nxt;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2709 {
// Port of the URG-processing / idle-rcv_up block of tcp_input().
// Returns 1 when the C side must `goto dodata;` (and *tiflags holds the
// updated flag word), 0 otherwise.
use core::ffi::{c_char, c_int, c_short, c_ushort, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct ipovly is __attribute__((packed))
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ipovly {
    pub ih_mbuf: [u8; 8], // struct mbuf_ptr { struct mbuf *mptr; } packed
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
    // u_int th_x2:4, th_off:4;
    pub th_x2_off: u32,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn tcp_input__part2709(
    tiflags: *mut c_int,
    ti: *mut Tcpiphdr,
    tp: *mut Tcpcb,
    so: *mut Socket,
) -> c_int {
    unsafe {
        let flags: c_int = *tiflags;

        if (flags & 0x20) != 0
            && (*ti).ti_t.th_urp != 0
            && !((*tp).t_state as c_int >= 10)
        {
            let urp: u32 = (*ti).ti_t.th_urp as u32;
            let cc: u32 = (*so).so_rcv.sb_cc;
            if urp.wrapping_add(cc) > (*so).so_rcv.sb_datalen {
                (*ti).ti_t.th_urp = 0;
                *tiflags = flags & !0x20;
                return 1;
            }
            let seq: u32 = (*ti).ti_t.th_seq;
            let newup: u32 = seq.wrapping_add(urp);
            if (newup.wrapping_sub((*tp).rcv_up) as i32) > 0 {
                (*tp).rcv_up = newup;
                (*so).so_urgc = cc.wrapping_add((*tp).rcv_up.wrapping_sub((*tp).rcv_nxt)) as i32;
                (*tp).rcv_up = seq.wrapping_add((*ti).ti_t.th_urp as u32);
            }
        } else {
            if ((*tp).rcv_nxt.wrapping_sub((*tp).rcv_up) as i32) > 0 {
                (*tp).rcv_up = (*tp).rcv_nxt;
            }
        }
        0
    }
}
}

// ── `dodata: if ((ti->ti_i.ih_len || (tiflags&0x01)) && ((tp->t_state) >= 10) == 0) { { if ((ti)->ti…` · slirp/tcp_input.c:1156-1172 · chunk 2710 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     dodata:
//      if ((ti->ti_i.ih_len || (tiflags&0x01)) &&
//          ((tp->t_state) >= 10) == 0) {
//       { if ((ti)->ti_t.th_seq == (tp)->rcv_nxt && ((tp)->seg_next == (struct tcpiphdr*)(tp)) && (tp)->t_state == 4) { tp->t_flags |= 0x0002; (tp)->rcv_nxt += (ti)->ti_i.ih_len; tiflags = (ti)->ti_t.th_flags & 0x01; if (so->so_emu) { if (tcp_emu((so),(m))) sbappend(so, (m)); } else sbappend((so), (m)); } else { (tiflags) = tcp_reass((tp), (ti), (m)); tp->t_flags |= 0x0001; } };
//      } else {
//       m_free(m);
//       tiflags &= ~0x01;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2710 {
#![allow(dead_code, non_snake_case)]

// Port of the "dodata:" block of tcp_input() (data reassembly) and of the
// static C helper tcp_reass() it calls (a static function has no linker
// symbol, so its logic is ported here as a private Rust function).

use core::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct MbufPtr {
    mptr: *mut c_void,
}

#[repr(C)]
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
    th_x2_off: u32, /* u_int th_x2:4, th_off:4 */
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
struct Tcpiphdr {
    ti_i: Ipovly,
    ti_t: Tcphdr,
}

#[repr(C)]
struct Qlink {
    next: *mut c_void,
    prev: *mut c_void,
}

#[repr(C)]
struct Sbuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[repr(C)]
struct Mbuf {
    _opaque: u8,
}

#[repr(C)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut Mbuf,
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
    so_tcpcb: *mut Tcpcb,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: Sbuf,
    so_snd: Sbuf,
    extra: *mut c_void,
}

#[repr(C)]
struct Tcpcb {
    seg_next: *mut Tcpiphdr,
    seg_prev: *mut Tcpiphdr,
    t_state: i16,
    t_timer: [i16; 4],
    t_rxtshift: i16,
    t_rxtcur: i16,
    t_dupacks: i16,
    t_maxseg: c_ushort,
    t_force: c_char,
    t_flags: c_ushort,
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
    t_rttmin: c_ushort,
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

extern "C" {
    fn tcp_emu(so: *mut Socket, m: *mut Mbuf) -> c_int;
    fn sbappend(so: *mut Socket, m: *mut Mbuf);
    fn m_free(m: *mut Mbuf);
    fn m_adj(m: *mut Mbuf, len: c_int);
    fn slirp_insque(a: *mut c_void, b: *mut c_void);
    fn slirp_remque(a: *mut c_void);
}

const QLINK_SZ: isize = core::mem::size_of::<Qlink>() as isize;

#[inline]
unsafe fn tcpiphdr2qlink(t: *mut Tcpiphdr) -> *mut Qlink {
    ((t as isize) - QLINK_SZ) as *mut Qlink
}

#[inline]
unsafe fn qlink2tcpiphdr(q: *mut Qlink) -> *mut Tcpiphdr {
    ((q as isize) + QLINK_SZ) as *mut Tcpiphdr
}

#[inline]
unsafe fn tcpfrag_list_first(tp: *mut Tcpcb) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tp).seg_next as *mut Qlink)
}

#[inline]
unsafe fn tcpfrag_list_end(f: *mut Tcpiphdr, tp: *mut Tcpcb) -> bool {
    (tcpiphdr2qlink(f) as *mut c_void) == (tp as *mut c_void)
}

#[inline]
unsafe fn tcpiphdr_next(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).next as *mut Qlink)
}

#[inline]
unsafe fn tcpiphdr_prev(t: *mut Tcpiphdr) -> *mut Tcpiphdr {
    qlink2tcpiphdr((*tcpiphdr2qlink(t)).prev as *mut Qlink)
}

/* TCPS_HAVEESTABLISHED and friends */
const TCPS_HAVEESTABLISHED: c_int = 4;
const TF_ACKNOW: c_ushort = 0x0001;
const TF_DELACK: c_ushort = 0x0002;
const TH_FIN: c_int = 0x01;
const SS_FCANTSENDMORE: c_int = 0x010;

/* ---- ported static helper: tcp_reass() ---- */

unsafe fn tcp_reass_present(tp: *mut Tcpcb, so: *mut Socket) -> c_int {
    let mut flags: c_int = 0;

    if !((*tp).t_state as c_int >= TCPS_HAVEESTABLISHED) {
        return 0;
    }
    let mut ti: *mut Tcpiphdr = tcpfrag_list_first(tp);
    if tcpfrag_list_end(ti, tp) || (*ti).ti_t.th_seq != (*tp).rcv_nxt {
        return 0;
    }
    if (*tp).t_state == 3 && (*ti).ti_i.ih_len != 0 {
        return 0;
    }
    loop {
        (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add((*ti).ti_i.ih_len as u32);
        flags = ((*ti).ti_t.th_flags as c_int) & TH_FIN;
        slirp_remque(tcpiphdr2qlink(ti) as *mut c_void);
        let m: *mut Mbuf = (*ti).ti_i.ih_mbuf.mptr as *mut Mbuf;
        ti = tcpiphdr_next(ti);
        if ((*so).so_state & SS_FCANTSENDMORE) != 0 {
            m_free(m);
        } else {
            if (*so).so_emu != 0 {
                if tcp_emu(so, m) != 0 {
                    sbappend(so, m);
                }
            } else {
                sbappend(so, m);
            }
        }
        if !((ti as *mut c_void) != (tp as *mut c_void) && (*ti).ti_t.th_seq == (*tp).rcv_nxt) {
            break;
        }
    }
    flags
}

unsafe fn tcp_reass(tp: *mut Tcpcb, ti_in: *mut Tcpiphdr, m_in: *mut Mbuf) -> c_int {
    let so: *mut Socket = (*tp).t_socket;
    let mut m: *mut Mbuf = m_in;
    let mut ti: *mut Tcpiphdr = ti_in;

    if !ti.is_null() {
        /* find a segment which begins after this one does */
        let mut q: *mut Tcpiphdr = tcpfrag_list_first(tp);
        while !tcpfrag_list_end(q, tp) {
            if ((*q).ti_t.th_seq.wrapping_sub((*ti).ti_t.th_seq) as c_int) > 0 {
                break;
            }
            q = tcpiphdr_next(q);
        }

        /* if there is a preceding segment, trim our data against it */
        let qprev: *mut Tcpiphdr = tcpiphdr_prev(q);
        if !tcpfrag_list_end(qprev, tp) {
            q = qprev;
            let i: c_int = (*q)
                .ti_t
                .th_seq
                .wrapping_add((*q).ti_i.ih_len as u32)
                .wrapping_sub((*ti).ti_t.th_seq) as c_int;
            if i > 0 {
                if i >= (*ti).ti_i.ih_len as c_int {
                    m_free(m);
                    return tcp_reass_present(tp, so);
                }
                m_adj(m, i);
                (*ti).ti_i.ih_len = ((*ti).ti_i.ih_len as c_int - i) as u16;
                (*ti).ti_t.th_seq = (*ti).ti_t.th_seq.wrapping_add(i as u32);
            }
            q = tcpiphdr_next(q);
        }
        (*ti).ti_i.ih_mbuf.mptr = m as *mut c_void;

        /* trim or dequeue succeeding segments covered by this one */
        while !tcpfrag_list_end(q, tp) {
            let i: c_int = (*ti)
                .ti_t
                .th_seq
                .wrapping_add((*ti).ti_i.ih_len as u32)
                .wrapping_sub((*q).ti_t.th_seq) as c_int;
            if i <= 0 {
                break;
            }
            if i < (*q).ti_i.ih_len as c_int {
                (*q).ti_t.th_seq = (*q).ti_t.th_seq.wrapping_add(i as u32);
                (*q).ti_i.ih_len = ((*q).ti_i.ih_len as c_int - i) as u16;
                m_adj((*q).ti_i.ih_mbuf.mptr as *mut Mbuf, i);
                break;
            }
            q = tcpiphdr_next(q);
            m = (*tcpiphdr_prev(q)).ti_i.ih_mbuf.mptr as *mut Mbuf;
            slirp_remque(tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void);
            m_free(m);
        }

        /* stick the new segment in its place */
        slirp_insque(
            tcpiphdr2qlink(ti) as *mut c_void,
            tcpiphdr2qlink(tcpiphdr_prev(q)) as *mut c_void,
        );
    }

    tcp_reass_present(tp, so)
}

/* ---- the ported TARGET: the dodata: block ---- */

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2710(
    tp: *mut Tcpcb,
    ti: *mut Tcpiphdr,
    m: *mut Mbuf,
    so: *mut Socket,
    tiflags_in: c_int,
) -> c_int {
    let mut tiflags: c_int = tiflags_in;
    let ih_len: u16 = (*ti).ti_i.ih_len;
    let t_state: i16 = (*tp).t_state;

    if (ih_len != 0 || (tiflags & 0x01) != 0) && !((t_state as c_int) >= 10) {
        /* common case: segment is the next one expected, queue is empty */
        if (*ti).ti_t.th_seq == (*tp).rcv_nxt
            && ((*tp).seg_next as *mut c_void) == (tp as *mut c_void)
            && t_state == 4
        {
            (*tp).t_flags |= TF_DELACK;
            (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add(ih_len as u32);
            tiflags = ((*ti).ti_t.th_flags as c_int) & TH_FIN;
            if (*so).so_emu != 0 {
                if tcp_emu(so, m) != 0 {
                    sbappend(so, m);
                }
            } else {
                sbappend(so, m);
            }
        } else {
            tiflags = tcp_reass(tp, ti, m);
            (*tp).t_flags |= TF_ACKNOW;
        }
    } else {
        m_free(m);
        tiflags &= !0x01;
    }

    tiflags
}
}

// ── `if (tiflags & 0x01)` · slirp/tcp_input.c:1178-1234 · chunk 2711 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tiflags & 0x01) {
//       if (((tp->t_state) >= 10) == 0) {
//        sofwdrain(so);
//     
//        tp->t_flags |= 0x0001;
//        tp->rcv_nxt++;
//       }
//       switch (tp->t_state) {
//     
//     
//     
//     
//     
//       case 3:
//       case 4:
//         if(so->so_emu == 0x1)
//           tp->t_state = 8;
//         else
//           tp->t_state = 5;
//         break;
//     
//     
//     
//     
//     
//       case 6:
//        tp->t_state = 7;
//        break;
//     
//     
//     
//     
//     
//     
//       case 9:
//        tp->t_state = 10;
//        tcp_canceltimers(tp);
//        tp->t_timer[3] = 2 * ( 5*2);
//        break;
//     
//     
//     
//     
//       case 10:
//        tp->t_timer[3] = 2 * ( 5*2);
//        break;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2711 {
// Ported piece of tcp_input: the FIN/flags handling block (tiflags & TH_FIN).
use core::ffi::{c_int, c_short};

// Mirror of the C `struct socket` fields needed (offsets preserved).
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub _pad0: [u8; 4],
    pub slirp: *mut u8,
    pub so_m: *mut u8,
    pub so_ti: *mut u8,
    pub so_urgc: c_int,
    pub so_faddr: u32,
    pub so_laddr: u32,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
}

// Mirror of the C `struct tcpcb` fields needed (offsets preserved).
#[repr(C)]
pub struct TcpCb {
    pub seg_next: *mut u8,
    pub seg_prev: *mut u8,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: u16,
    pub t_force: i8,
    pub _pad1: i8,
    pub t_flags: u16,
    pub _pad2: [u8; 2],
    pub t_template: [u8; 40],
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
}

extern "C" {
    fn sofwdrain(so: *mut Socket);
    fn tcp_canceltimers(tp: *mut TcpCb);
}

#[no_mangle]
pub extern "C" fn tcp_input__part2711(tiflags: c_int, tp: *mut TcpCb, so: *mut Socket) {
    unsafe {
        if tiflags & 0x01 != 0 {
            if (*tp).t_state < 10 {
                sofwdrain(so);
                (*tp).t_flags |= 0x0001;
                (*tp).rcv_nxt = (*tp).rcv_nxt.wrapping_add(1);
            }
            match (*tp).t_state {
                3 | 4 => {
                    if (*so).so_emu == 0x1 {
                        (*tp).t_state = 8;
                    } else {
                        (*tp).t_state = 5;
                    }
                }
                6 => {
                    (*tp).t_state = 7;
                }
                9 => {
                    (*tp).t_state = 10;
                    tcp_canceltimers(tp);
                    (*tp).t_timer[3] = 2 * (5 * 2);
                }
                10 => {
                    (*tp).t_timer[3] = 2 * (5 * 2);
                }
                _ => {}
            }
        }
    }
}
}

// ── `if (ti->ti_i.ih_len && (unsigned)ti->ti_i.ih_len <= 5 && ((struct tcpiphdr_2 *)ti)->first_char…` · slirp/tcp_input.c:1243-1246 · chunk 2712 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ti->ti_i.ih_len && (unsigned)ti->ti_i.ih_len <= 5 &&
//          ((struct tcpiphdr_2 *)ti)->first_char == (char)27) {
//       tp->t_flags |= 0x0001;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2712 {
// Port of the telnet-escape check at the end of tcp_input():
//   if (ti->ti_i.ih_len && (unsigned)ti->ti_i.ih_len <= 5 &&
//       ((struct tcpiphdr_2 *)ti)->first_char == (char)27)
//     tp->t_flags |= TF_ACKNOW;
use core::ffi::{c_char, c_void};

// struct ipovly (packed): struct mbuf_ptr ih_mbuf; uint8_t ih_x1, ih_pr;
// uint16_t ih_len; struct in_addr ih_src, ih_dst;
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: *mut c_void,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: u32,
    pub ih_dst: u32,
}

// struct tcphdr (bitfields collapsed into one byte for th_x2/th_off)
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
pub struct Tcpiphdr2 {
    pub dummy: Tcpiphdr,
    pub first_char: c_char,
}

// Leading part of struct tcpcb, up to t_flags.
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
}

#[no_mangle]
pub unsafe extern "C" fn tcp_input__part2712(ti: *mut Tcpiphdr, tp: *mut Tcpcb) {
    if ti.is_null() || tp.is_null() {
        return;
    }
    let ih_len = core::ptr::addr_of!((*ti).ti_i.ih_len).read_unaligned();
    if ih_len != 0 && (ih_len as u32) <= 5 {
        let first_char =
            core::ptr::addr_of!((*(ti as *const Tcpiphdr2)).first_char).read_unaligned();
        if first_char == 27 {
            (*tp).t_flags |= 0x0001;
        }
    }
}
}

// ── `if (needoutput || (tp->t_flags & 0x0001)` · slirp/tcp_input.c:1251-1253 · chunk 2713 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (needoutput || (tp->t_flags & 0x0001)) {
//       (void) tcp_output(tp);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2713 {
// Port of the tcp_input tail: if (needoutput || (tp->t_flags & TF_ACKNOW)) tcp_output(tp);
#![allow(non_camel_case_types)]

// Prefix of the C `struct tcpcb` (fields in declaration order), enough to reach
// `t_flags` with the exact same offset and type (u_short).
#[repr(C)]
pub struct tcpcb_prefix {
    pub seg_next: *mut core::ffi::c_void,
    pub seg_prev: *mut core::ffi::c_void,
    pub t_state: core::ffi::c_short,
    pub t_timer: [core::ffi::c_short; 4],
    pub t_rxtshift: core::ffi::c_short,
    pub t_rxtcur: core::ffi::c_short,
    pub t_dupacks: core::ffi::c_short,
    pub t_maxseg: core::ffi::c_ushort,
    pub t_force: core::ffi::c_char,
    pub t_flags: core::ffi::c_ushort,
}

extern "C" {
    fn tcp_output(tp: *mut tcpcb_prefix) -> core::ffi::c_int;
}

#[no_mangle]
pub extern "C" fn tcp_input__part2713(tp: *mut tcpcb_prefix, needoutput: core::ffi::c_int) {
    unsafe {
        let flags_nonzero = if tp.is_null() {
            false
        } else {
            ((*tp).t_flags as core::ffi::c_int) & 0x0001 != 0
        };
        if needoutput != 0 || flags_nonzero {
            let _ = tcp_output(tp);
        }
    }
}
}

// ── `return;` · slirp/tcp_input.c:1254-1254 · chunk 2714 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2714 {
// Ported piece of tcp_input: the normal-path `return;` at the end of the
// data-processing section (slirp/tcp_input.c:1254). The statement itself has
// no effect; the control flow stays in C.

#[no_mangle]
pub extern "C" fn tcp_input__part2714() {
    // no-op: the C counterpart performs the actual `return`
}
}

// ── `dropafterack: if (tiflags & 0x04) goto drop;` · slirp/tcp_input.c:1256-1262 · chunk 2715 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     dropafterack:
//     
//     
//     
//     
//      if (tiflags & 0x04)
//       goto drop;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2715 {
// Port of the "dropafterack:" entry test from tcp_input().
// Returns non-zero when the caller must jump to the `drop` label
// (i.e. the RST flag is set), zero otherwise.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_input__part2715(tiflags: c_int) -> c_int {
    ((tiflags & 0x04) != 0) as c_int
}
}

// ── `m_free(m); tp->t_flags |= 0x0001; (void) tcp_output(tp); return;` · slirp/tcp_input.c:1263-1266 · chunk 2716 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     m_free(m);
//      tp->t_flags |= 0x0001;
//      (void) tcp_output(tp);
//      return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2716 {
use core::ffi::{c_char, c_int, c_short, c_ushort, c_void};

// Prefix of the C `struct tcpcb` (slirp/tcp_var.h) up to and including t_flags.
// Fields are in the same order with the same types so offsets match the C layout.
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

extern "C" {
    fn m_free(m: *mut c_void);
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn tcp_input__part2716(m: *mut c_void, tp: *mut Tcpcb) {
    unsafe {
        m_free(m);
        (*tp).t_flags |= 0x0001;
        tcp_output(tp);
    }
}
}

// ── `dropwithreset: if (tiflags & 0x10) tcp_respond(tp, ti, m, (tcp_seq)0, ti->ti_t.th_ack, 0x04); e…` · slirp/tcp_input.c:1268-1276 · chunk 2717 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     dropwithreset:
//     
//      if (tiflags & 0x10)
//       tcp_respond(tp, ti, m, (tcp_seq)0, ti->ti_t.th_ack, 0x04);
//      else {
//       if (tiflags & 0x02) ti->ti_i.ih_len++;
//       tcp_respond(tp, ti, m, ti->ti_t.th_seq+ti->ti_i.ih_len, (tcp_seq)0,
//           0x04|0x10);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2717 {
// Port of the `dropwithreset:` tail of slirp's tcp_input().
#![allow(dead_code)]

use core::ffi::c_int;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct tcpcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mbuf {
    _private: [u8; 0],
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

extern "C" {
    fn tcp_respond(
        tp: *mut tcpcb,
        ti: *mut tcpiphdr,
        m: *mut mbuf,
        seq: u32,
        ack: u32,
        flags: c_int,
    );
}

#[no_mangle]
pub extern "C" fn tcp_input__part2717(
    tp: *mut tcpcb,
    ti: *mut tcpiphdr,
    m: *mut mbuf,
    tiflags: c_int,
) {
    unsafe {
        if tiflags & 0x10 != 0 {
            let ack: u32 = (*ti).ti_t.th_ack;
            tcp_respond(tp, ti, m, 0u32, ack, 0x04);
        } else {
            if tiflags & 0x02 != 0 {
                let ih: &mut ipovly = &mut (*ti).ti_i;
                let cur: u16 = ih.ih_len;
                ih.ih_len = cur.wrapping_add(1);
            }
            let seq: u32 = (*ti).ti_t.th_seq;
            let len: u16 = (*ti).ti_i.ih_len;
            tcp_respond(tp, ti, m, seq.wrapping_add(len as u32), 0u32, 0x04 | 0x10);
        }
    }
}
}

// ── `return;` · slirp/tcp_input.c:1278-1278 · chunk 2718 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2718 {
// Port of the `dropwithreset` tail of tcp_input(): a plain void return.
// No parameters and no results: the enclosing C function's control flow
// (the `return;` statement itself) stays in C right after this call.

#[no_mangle]
pub extern "C" fn tcp_input__part2718() {
    // nothing to do: the C `return;` follows the call
}
}

// ── `drop: m_free(m);` · slirp/tcp_input.c:1280-1284 · chunk 2719 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     drop:
//     
//     
//     
//      m_free(m);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2719 {
// Port of the `drop:` tail of tcp_input: free the mbuf holding the segment.
// `struct mbuf` is only ever handled through a pointer here, so it is mirrored
// as an opaque #[repr(C)] type.

#[repr(C)]
pub struct mbuf {
    _opaque: [u8; 0],
}

extern "C" {
    // void m_free(struct mbuf *);
    fn m_free(m: *mut mbuf);
}

#[no_mangle]
pub extern "C" fn tcp_input__part2719(m: *mut mbuf) {
    unsafe {
        m_free(m);
    }
}
}

// ── `return;` · slirp/tcp_input.c:1286-1286 · chunk 2720 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2720 {
// Ported piece of tcp_input(): the plain `return;` at the `drop:` label.
// It takes no arguments, mutates nothing visible, and just returns to the
// caller; the C side keeps the actual `return;` statement.
#[no_mangle]
pub extern "C" fn tcp_input__part2720() {}
}

// ── `tcp_dooptions` · slirp/tcp_input.c:1289-1325 · chunk 2721 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     tcp_dooptions(struct tcpcb *tp, u_char *cp, int cnt, struct tcpiphdr *ti)
//     {
//      uint16_t mss;
//      int opt, optlen;
//     
//      ;
//      ;
//     
//      for (; cnt > 0; cnt -= optlen, cp += optlen) {
//       opt = cp[0];
//       if (opt == 0)
//        break;
//       if (opt == 1)
//        optlen = 1;
//       else {
//        optlen = cp[1];
//        if (optlen <= 0)
//         break;
//       }
//       switch (opt) {
//     
//       default:
//        continue;
//     
//       case 2:
//        if (optlen != 4)
//         continue;
//        if (!(ti->ti_t.th_flags & 0x02))
//         continue;
//        memcpy((char *) &mss, (char *) cp + 2, sizeof(mss));
//        ((mss) = ntohs((uint16_t)(mss)));
//        (void) tcp_mss(tp, mss);
//        break;
//       }
//      }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2721 {
// Port of slirp/tcp_input.c:tcp_dooptions
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uchar, c_uint, c_void};

extern "C" {
    fn ntohs(__netshort: u16) -> u16;
    fn tcp_mss(tp: *mut c_void, offer: c_uint) -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct in_addr {
    s_addr: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct mbuf_ptr {
    mptr: *mut c_void,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct ipovly {
    ih_mbuf: mbuf_ptr,
    ih_x1: u8,
    ih_pr: u8,
    ih_len: u16,
    ih_src: in_addr,
    ih_dst: in_addr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tcphdr {
    th_sport: u16,
    th_dport: u16,
    th_seq: u32,
    th_ack: u32,
    // u_int th_x2:4, th_off:4;
    th_x2_off: c_uint,
    th_flags: u8,
    th_win: u16,
    th_sum: u16,
    th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct tcpiphdr {
    ti_i: ipovly,
    ti_t: tcphdr,
}

const TCPOPT_EOL: c_int = 0;
const TCPOPT_NOP: c_int = 1;
const TCPOPT_MAXSEG: c_int = 2;
const TCPOLEN_MAXSEG: c_int = 4;
const TH_SYN: u8 = 0x02;

#[no_mangle]
pub extern "C" fn tcp_dooptions(
    tp: *mut c_void,
    cp: *mut c_uchar,
    cnt: c_int,
    ti: *mut tcpiphdr,
) {
    let mut mss: u16;
    let mut opt: c_int;
    let mut optlen: c_int = 0;

    let mut cnt = cnt;
    let mut cp = cp;

    loop {
        if cnt <= 0 {
            break;
        }
        unsafe {
            opt = *cp as c_int;
        }
        if opt == TCPOPT_EOL {
            break;
        }
        if opt == TCPOPT_NOP {
            optlen = 1;
        } else {
            unsafe {
                optlen = *cp.add(1) as c_int;
            }
            if optlen <= 0 {
                break;
            }
        }
        if opt == TCPOPT_MAXSEG {
            let flags = unsafe { (*ti).ti_t.th_flags };
            if optlen == TCPOLEN_MAXSEG && (flags & TH_SYN) != 0 {
                mss = 0;
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        cp.add(2),
                        &mut mss as *mut u16 as *mut u8,
                        core::mem::size_of::<u16>(),
                    );
                    mss = ntohs(mss);
                    let _ = tcp_mss(tp, mss as c_uint);
                }
            }
        }
        // both `break` out of the switch and `continue` reach the for-increment
        cnt -= optlen;
        unsafe {
            cp = cp.offset(optlen as isize);
        }
    }
}
}

// ── `uint16_t mss; int opt, optlen; ; ;` · slirp/tcp_input.c:1292-1296 · chunk 2722 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint16_t mss;
//      int opt, optlen;
//     
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2722 {
// Port of the local-declaration head of tcp_dooptions().
//
// In the original C these are plain declarations of the loop's locals:
//     uint16_t mss;
//     int opt, optlen;
// followed by two empty statements.  Every read of `opt`, `optlen` and `mss`
// in the rest of tcp_dooptions() happens only after an assignment, so the
// declarations merely have to bring the locals into existence (with an
// addressable home).  This function receives their addresses and leaves them
// in a defined state.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_dooptions__part2722(
    mss: *mut u16,
    opt: *mut c_int,
    optlen: *mut c_int,
) {
    unsafe {
        if !mss.is_null() {
            core::ptr::write(mss, 0);
        }
        if !opt.is_null() {
            core::ptr::write(opt, 0);
        }
        if !optlen.is_null() {
            core::ptr::write(optlen, 0);
        }
    }
}
}

// ── `for (; cnt > 0; cnt -= optlen, cp += optlen)` · slirp/tcp_input.c:1298-1324 · chunk 2723 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (; cnt > 0; cnt -= optlen, cp += optlen) {
//       opt = cp[0];
//       if (opt == 0)
//        break;
//       if (opt == 1)
//        optlen = 1;
//       else {
//        optlen = cp[1];
//        if (optlen <= 0)
//         break;
//       }
//       switch (opt) {
//     
//       default:
//        continue;
//     
//       case 2:
//        if (optlen != 4)
//         continue;
//        if (!(ti->ti_t.th_flags & 0x02))
//         continue;
//        memcpy((char *) &mss, (char *) cp + 2, sizeof(mss));
//        ((mss) = ntohs((uint16_t)(mss)));
//        (void) tcp_mss(tp, mss);
//        break;
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2723 {
#![allow(non_camel_case_types)]
#![allow(unsafe_op_in_unsafe_fn)]
use core::ffi::{c_int, c_uint, c_void};

// Mirror of C struct in_addr (only the field layout matters here).
#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirror of C struct mbuf_ptr (packed pointer wrapper).
#[repr(C, packed)]
pub struct MbufPtr {
    pub mptr: *mut c_void,
}

// Mirror of C struct ipovly (packed).
#[repr(C, packed)]
pub struct Ipovly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

// Mirror of C struct tcphdr: the bitfields th_x2:4/th_off:4 share one u_int unit.
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

// Mirror of C struct tcpiphdr.
#[repr(C)]
pub struct Tcpiphdr {
    pub ti_i: Ipovly,
    pub ti_t: Tcphdr,
}

extern "C" {
    fn ntohs(__netshort: u16) -> u16;
    fn tcp_mss(tp: *mut c_void, offer: c_uint) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tcp_dooptions__part2723(
    tp: *mut c_void,
    mut cp: *mut u8,
    mut cnt: c_int,
    ti: *const Tcpiphdr,
) {
    let mut mss: u16 = 0;
    let mut opt: c_int;
    let mut optlen: c_int;

    while cnt > 0 {
        opt = *cp as c_int;
        if opt == 0 {
            break;
        }
        if opt == 1 {
            optlen = 1;
        } else {
            optlen = *cp.add(1) as c_int;
            if optlen <= 0 {
                break;
            }
        }
        // switch (opt) { default: continue; case 2: ... break; }
        if opt == 2 {
            if optlen == 4 && ((*ti).ti_t.th_flags & 0x02) != 0 {
                core::ptr::copy_nonoverlapping(cp.add(2), &mut mss as *mut u16 as *mut u8, 2);
                mss = ntohs(mss);
                tcp_mss(tp, mss as c_uint);
            }
        }
        cnt -= optlen;
        cp = cp.add(optlen as usize);
    }
}
}

// ── `tcp_xmit_timer` · slirp/tcp_input.c:1371-1440 · chunk 2724 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void
//     tcp_xmit_timer(register struct tcpcb *tp, int rtt)
//     {
//      register short delta;
//     
//      ;
//      ;
//      ;
//     
//      if (tp->t_srtt != 0) {
//     
//     
//     
//     
//     
//     
//     
//       delta = rtt - 1 - (tp->t_srtt >> 3);
//       if ((tp->t_srtt += delta) <= 0)
//        tp->t_srtt = 1;
//       if (delta < 0)
//        delta = -delta;
//       delta -= (tp->t_rttvar >> 2);
//       if ((tp->t_rttvar += delta) <= 0)
//        tp->t_rttvar = 1;
//      } else {
//     
//     
//     
//     
//     
//       tp->t_srtt = rtt << 3;
//       tp->t_rttvar = rtt << (2 - 1);
//      }
//      tp->t_rtt = 0;
//      tp->t_rxtshift = 0;
//      { (tp->t_rxtcur) = ((((tp)->t_srtt >> 3) + (tp)->t_rttvar)); if ((tp->t_rxtcur) < ((short)tp->t_rttmin)) (tp->t_rxtcur) = ((short)tp->t_rttmin); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
//                                              ;
//      tp->t_softerror = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2724 {
// Rust rewrite of slirp's static tcp_xmit_timer() (slirp/tcp_input.c).
//
// Collect new round-trip time estimate and update averages and current
// timeout.  The C original works with "short" arithmetic; every store into a
// short field is a truncating conversion, and the temporaries are ints, so the
// Rust code below mirrors that with explicit i32 math and `as i16` stores.

#![allow(dead_code)]

use core::ffi::{c_int, c_void};

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
    pub th_seq: u32, /* tcp_seq */
    pub th_ack: u32, /* tcp_seq */
    /* u_int th_x2:4, th_off:4;  -- one 4 byte bit-field unit */
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
    pub t_socket: *mut c_void, /* struct socket * */
    pub snd_una: u32,          /* tcp_seq */
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
    pub t_rtseq: u32, /* tcp_seq */
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
    pub last_ack_sent: u32, /* tcp_seq */
}

const TCP_RTT_SHIFT: i32 = 3;
const TCP_RTTVAR_SHIFT: i32 = 2;
const TCPTV_REXMTMAX: i16 = 12 * 2;

#[no_mangle]
pub extern "C" fn tcp_xmit_timer(tp: *mut Tcpcb, rtt: c_int) {
    if tp.is_null() {
        return;
    }
    unsafe {
        let srtt = (*tp).t_srtt;
        if srtt != 0 {
            /* srtt is fixed point with 3 bits after the binary point. */
            let mut delta: i16 =
                rtt.wrapping_sub(1)
                    .wrapping_sub((srtt as i32) >> TCP_RTT_SHIFT) as i16;

            let new_srtt = srtt.wrapping_add(delta);
            (*tp).t_srtt = new_srtt;
            if new_srtt <= 0 {
                (*tp).t_srtt = 1;
            }

            /* rttvar is fixed point with 2 bits after the binary point. */
            if delta < 0 {
                delta = delta.wrapping_neg();
            }
            delta = (delta as i32)
                .wrapping_sub(((*tp).t_rttvar as i32) >> TCP_RTTVAR_SHIFT)
                as i16;

            let new_rttvar = (*tp).t_rttvar.wrapping_add(delta);
            (*tp).t_rttvar = new_rttvar;
            if new_rttvar <= 0 {
                (*tp).t_rttvar = 1;
            }
        } else {
            /* No rtt measurement yet - use the unsmoothed rtt. */
            (*tp).t_srtt = rtt.wrapping_shl(TCP_RTT_SHIFT as u32) as i16;
            (*tp).t_rttvar = rtt.wrapping_shl((TCP_RTTVAR_SHIFT - 1) as u32) as i16;
        }
        (*tp).t_rtt = 0;
        (*tp).t_rxtshift = 0;

        /* TCPT_RANGESET(tp->t_rxtcur, TCP_REXMTVAL(tp),
         *               (short)tp->t_rttmin, TCPTV_REXMTMAX) */
        let val: i32 = (((*tp).t_srtt as i32) >> TCP_RTT_SHIFT)
            .wrapping_add((*tp).t_rttvar as i32);
        let mut rxtcur: i16 = val as i16;
        let rttmin: i16 = (*tp).t_rttmin as i16;
        if rxtcur < rttmin {
            rxtcur = rttmin;
        } else if rxtcur > TCPTV_REXMTMAX {
            rxtcur = TCPTV_REXMTMAX;
        }
        (*tp).t_rxtcur = rxtcur;

        (*tp).t_softerror = 0;
    }
}
}

// ── `register short delta; ; ; ;` · slirp/tcp_input.c:1374-1378 · chunk 2725 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register short delta;
//     
//      ;
//      ;
//      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2725 {
// Ported from tcp_xmit_timer() in slirp/tcp_input.c.
//
// The TARGET is the declaration of the local `register short delta;` together
// with the three DEBUG_* statements, which are compiled out to empty
// statements (`;`) in this configuration.  The only observable part the rest
// of the function needs is a value for `delta`; C leaves it uninitialized and
// the function assigns it before any use, so the ported helper just supplies
// the initial (dummy) value.

use core::ffi::c_short;

#[no_mangle]
pub extern "C" fn tcp_xmit_timer__part2725() -> c_short {
    0
}
}

// ── `if (tp->t_srtt != 0) … else` · slirp/tcp_input.c:1380-1414 · chunk 2726 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tp->t_srtt != 0) {
//     
//     
//     
//     
//     
//     
//     
//       delta = rtt - 1 - (tp->t_srtt >> 3);
//       if ((tp->t_srtt += delta) <= 0)
//        tp->t_srtt = 1;
//       if (delta < 0)
//        delta = -delta;
//       delta -= (tp->t_rttvar >> 2);
//       if ((tp->t_rttvar += delta) <= 0)
//        tp->t_rttvar = 1;
//      } else {
//     
//     
//     
//     
//     
//       tp->t_srtt = rtt << 3;
//       tp->t_rttvar = rtt << (2 - 1);
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2726 {
// Port of the srtt/rttvar smoothing step of tcp_xmit_timer (slirp/tcp_input.c).
// The C target only reads and updates tp->t_srtt and tp->t_rttvar, so those
// two fields are passed by pointer; the temporary `delta` stays in Rust.

use core::ffi::{c_int, c_short};

#[no_mangle]
pub extern "C" fn tcp_xmit_timer__part2726(
    t_srtt: *mut c_short,
    t_rttvar: *mut c_short,
    rtt: c_int,
) {
    unsafe {
        if *t_srtt != 0 {
            // delta = rtt - 1 - (tp->t_srtt >> 3);  (int -> short truncation)
            let delta: c_short = rtt.wrapping_sub(1).wrapping_sub((*t_srtt as c_int) >> 3) as c_short;

            // if ((tp->t_srtt += delta) <= 0) tp->t_srtt = 1;
            let srtt = (*t_srtt as c_int).wrapping_add(delta as c_int) as c_short;
            *t_srtt = srtt;
            if srtt <= 0 {
                *t_srtt = 1;
            }

            // if (delta < 0) delta = -delta;
            let mut d: c_short = delta;
            if d < 0 {
                d = d.wrapping_neg();
            }

            // delta -= (tp->t_rttvar >> 2);
            d = (d as c_int).wrapping_sub((*t_rttvar as c_int) >> 2) as c_short;

            // if ((tp->t_rttvar += delta) <= 0) tp->t_rttvar = 1;
            let rttvar = (*t_rttvar as c_int).wrapping_add(d as c_int) as c_short;
            *t_rttvar = rttvar;
            if rttvar <= 0 {
                *t_rttvar = 1;
            }
        } else {
            // tp->t_srtt = rtt << 3;
            *t_srtt = rtt.wrapping_shl(3) as c_short;
            // tp->t_rttvar = rtt << (2 - 1);
            *t_rttvar = rtt.wrapping_shl(1) as c_short;
        }
    }
}
}

// ── `tp->t_rtt = 0; tp->t_rxtshift = 0;` · slirp/tcp_input.c:1415-1416 · chunk 2727 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tp->t_rtt = 0;
//      tp->t_rxtshift = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2727 {
// Ported piece of tcp_xmit_timer: reset the RTT sample and backoff shift.
use core::ffi::c_short;

#[no_mangle]
pub extern "C" fn tcp_xmit_timer__part2727(t_rtt: *mut c_short, t_rxtshift: *mut c_short) {
    unsafe {
        if !t_rtt.is_null() {
            *t_rtt = 0;
        }
        if !t_rxtshift.is_null() {
            *t_rxtshift = 0;
        }
    }
}
}

// ── `{ … }` · slirp/tcp_input.c:1429-1429 · chunk 2728 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     { (tp->t_rxtcur) = ((((tp)->t_srtt >> 3) + (tp)->t_rttvar)); if ((tp->t_rxtcur) < ((short)tp->t_rttmin)) (tp->t_rxtcur) = ((short)tp->t_rttmin); else if ((tp->t_rxtcur) > (( 12*2))) (tp->t_rxtcur) = (( 12*2)); }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2728 {
// Rust port of the TCPT_RANGESET piece of tcp_xmit_timer.

#[repr(C, align(4))]
pub struct Tcpiphdr {
    bytes: [u8; 44],
}

#[repr(C)]
pub struct Tcpcb {
    seg_next: *mut u8,
    seg_prev: *mut u8,
    t_state: i16,
    t_timer: [i16; 4],
    t_rxtshift: i16,
    t_rxtcur: i16,
    t_dupacks: i16,
    t_maxseg: u16,
    t_force: i8,
    t_flags: u16,
    t_template: Tcpiphdr,
    t_socket: *mut u8,
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
}

// TCPT_RANGESET(tp->t_rxtcur, TCP_REXMTVAL(tp), (short)tp->t_rttmin, TCPTV_REXMTMAX)
#[no_mangle]
pub extern "C" fn tcp_xmit_timer__part2728(tp: *mut Tcpcb) {
    unsafe {
        let srtt = (*tp).t_srtt;
        let rttvar = (*tp).t_rttvar;
        let rttmin = (*tp).t_rttmin;

        // ((tp)->t_srtt >> TCP_RTT_SHIFT) + (tp)->t_rttvar, in int, then
        // stored into the short field t_rxtcur.
        let mut val: i16 = (srtt as i32 >> 3).wrapping_add(rttvar as i32) as i16;

        if val < rttmin as i16 {
            val = rttmin as i16;
        } else if val > 12 * 2 {
            val = 12 * 2;
        }

        (*tp).t_rxtcur = val;
    }
}
}

// ── `; tp->t_softerror = 0;` · slirp/tcp_input.c:1430-1439 · chunk 2729 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ;
//      tp->t_softerror = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2729 {
// Portion of tcp_xmit_timer: clear tp->t_softerror.
// The field is passed by address so the C side keeps ownership of the
// struct layout.

use core::ffi::c_short;

#[no_mangle]
pub extern "C" fn tcp_xmit_timer__part2729(t_softerror: *mut c_short) {
    unsafe {
        *t_softerror = 0;
    }
}
}

// ── `tcp_mss` · slirp/tcp_input.c:1458-1487 · chunk 2730 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     tcp_mss(struct tcpcb *tp, u_int offer)
//     {
//      struct socket *so = tp->t_socket;
//      int mss;
//     
//      ;
//      ;
//      ;
//     
//      mss = ((1500) < (1500) ? (1500) : (1500)) - sizeof(struct tcpiphdr);
//      if (offer)
//       mss = ((mss) < (offer) ? (mss) : (offer));
//      mss = ((mss) > (32) ? (mss) : (32));
//      if (mss < tp->t_maxseg || offer != 0)
//         tp->t_maxseg = mss;
//     
//      tp->snd_cwnd = mss;
//     
//      sbreserve(&so->so_snd, 8192 + ((8192 % mss) ?
//                                                    (mss - (8192 % mss)) :
//                                                    0));
//      sbreserve(&so->so_rcv, 8192 + ((8192 % mss) ?
//                                                    (mss - (8192 % mss)) :
//                                                    0));
//     
//      ;
//     
//      return mss;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2730 {
// Rust rewrite of slirp's tcp_mss()
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

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
    // u_int th_x2:4, th_off:4;  (one 4-byte storage unit)
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

extern "C" {
    fn sbreserve(sb: *mut Sbuf, size: c_int);
}

// TCP_SNDSPACE / TCP_RCVSPACE in the preprocessed source
const TCP_SNDSPACE: c_int = 8192;
const TCP_RCVSPACE: c_int = 8192;
// min(IF_MTU, IF_MRU) : (1500) < (1500) ? (1500) : (1500)
const IF_MTU_MRU: c_int = 1500;

#[no_mangle]
pub extern "C" fn tcp_mss(tp: *mut Tcpcb, offer: c_uint) -> c_int {
    unsafe {
        if tp.is_null() {
            return 0;
        }
        let so: *mut Socket = (*tp).t_socket;
        let mut mss: c_int;

        mss = IF_MTU_MRU - core::mem::size_of::<Tcpiphdr>() as c_int;
        if offer != 0 {
            // C: ((mss) < (offer) ? (mss) : (offer)) with int/unsigned int
            // operands -> comparison done in unsigned int.
            mss = if (mss as c_uint) < offer {
                mss
            } else {
                offer as c_int
            };
        }
        mss = if mss > 32 { mss } else { 32 };
        if mss < (*tp).t_maxseg as c_int || offer != 0 {
            (*tp).t_maxseg = mss as u16;
        }

        (*tp).snd_cwnd = mss as u32;

        if so.is_null() {
            return mss;
        }

        let rem_snd = TCP_SNDSPACE % mss;
        sbreserve(
            core::ptr::addr_of_mut!((*so).so_snd),
            TCP_SNDSPACE + if rem_snd != 0 { mss - rem_snd } else { 0 },
        );

        let rem_rcv = TCP_RCVSPACE % mss;
        sbreserve(
            core::ptr::addr_of_mut!((*so).so_rcv),
            TCP_RCVSPACE + if rem_rcv != 0 { mss - rem_rcv } else { 0 },
        );

        mss
    }
}
}

// ── `struct socket *so = tp->t_socket; int mss; ; ; ; mss = ((1500) < (1500) ? (1500) : (1500)) - si…` · slirp/tcp_input.c:1461-1468 · chunk 2731 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *so = tp->t_socket;
//      int mss;
//     
//      ;
//      ;
//      ;
//     
//      mss = ((1500) < (1500) ? (1500) : (1500)) - sizeof(struct tcpiphdr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2731 {
// Part of tcp_mss: initial MSS computation from the interface MTU and
// the size of the tcp/ip header block.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

// size of the (packed) struct tcpiphdr in the C program
const TCPIPHDR_SIZE: core::ffi::c_int = 40;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct mbuf_ptr {
    pub mptr: *mut u8,
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
    // u_int th_x2:4, th_off:4; packed into one 32-bit unit
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
    pub t_socket: *mut u8,
    // remaining fields are not accessed here
}

#[no_mangle]
pub extern "C" fn tcp_mss__part2731(tp: *mut tcpcb) -> core::ffi::c_int {
    let _ = tp;
    // mss = ((1500) < (1500) ? (1500) : (1500)) - sizeof(struct tcpiphdr);
    let mtu: core::ffi::c_int = 1500;
    let other: core::ffi::c_int = 1500;
    let base: core::ffi::c_int = if mtu < other { mtu } else { other };
    base - TCPIPHDR_SIZE
}
}

// ── `if (offer)` · slirp/tcp_input.c:1469-1470 · chunk 2732 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offer)
//       mss = ((mss) < (offer) ? (mss) : (offer));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2732 {
use core::ffi::{c_int, c_uint};

// Part of tcp_mss(): apply the offered MSS if one was given.
// C: if (offer) mss = ((mss) < (offer) ? (mss) : (offer));
// The comparison is done in the common type of int and unsigned int,
// i.e. unsigned int, and the result is converted back to int.
#[no_mangle]
pub extern "C" fn tcp_mss__part2732(mss: c_int, offer: c_uint) -> c_int {
    if offer != 0 {
        if (mss as c_uint) < offer {
            mss
        } else {
            offer as c_int
        }
    } else {
        mss
    }
}
}

// ── `mss = ((mss) > (32) ? (mss) : (32));` · slirp/tcp_input.c:1471-1471 · chunk 2733 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     mss = ((mss) > (32) ? (mss) : (32));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2733 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn tcp_mss__part2733(mss: c_int) -> c_int {
    if mss > 32 {
        mss
    } else {
        32
    }
}
}

// ── `if (mss < tp->t_maxseg || offer != 0)` · slirp/tcp_input.c:1472-1473 · chunk 2734 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mss < tp->t_maxseg || offer != 0)
//         tp->t_maxseg = mss;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2734 {
// Ported piece of tcp_mss: update tp->t_maxseg when the computed mss is
// smaller than the current one, or when an MSS was offered by the peer.
use core::ffi::{c_int, c_short, c_uint, c_ushort, c_void};

// Mirror of the prefix of `struct tcpcb` (slirp) up to and including
// `t_maxseg`; the fields before it are needed to place `t_maxseg` correctly.
#[repr(C)]
pub struct Tcpcb {
    seg_next: *mut c_void,
    seg_prev: *mut c_void,
    t_state: c_short,
    t_timer: [c_short; 4],
    t_rxtshift: c_short,
    t_rxtcur: c_short,
    t_dupacks: c_short,
    t_maxseg: c_ushort,
}

#[no_mangle]
pub extern "C" fn tcp_mss__part2734(tp: *mut Tcpcb, mss: c_int, offer: c_uint) {
    unsafe {
        let t_maxseg = (*tp).t_maxseg as c_int;
        if mss < t_maxseg || offer != 0 {
            (*tp).t_maxseg = mss as c_ushort;
        }
    }
}
}

// ── `tp->snd_cwnd = mss; sbreserve(&so->so_snd, 8192 + ((8192 % mss) ? (mss - (8192 % mss)) : 0)); s…` · slirp/tcp_input.c:1475-1486 · chunk 2735 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tp->snd_cwnd = mss;
//     
//      sbreserve(&so->so_snd, 8192 + ((8192 % mss) ?
//                                                    (mss - (8192 % mss)) :
//                                                    0));
//      sbreserve(&so->so_rcv, 8192 + ((8192 % mss) ?
//                                                    (mss - (8192 % mss)) :
//                                                    0));
//     
//      ;
//     
//      return mss;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2735 {
use core::ffi::{c_char, c_int, c_uint, c_void};

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
    pub th_x2_off: c_uint,
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
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
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

#[repr(C)]
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

extern "C" {
    fn sbreserve(sb: *mut sbuf, size: c_int);
}

#[no_mangle]
pub extern "C" fn tcp_mss__part2735(tp: *mut tcpcb, mss: c_int) -> c_int {
    unsafe {
        let so = (*tp).t_socket;
        (*tp).snd_cwnd = mss as u32;

        let rem = if mss != 0 { 8192i32 % mss } else { 0 };
        let val = 8192i32.wrapping_add(if rem != 0 { mss.wrapping_sub(rem) } else { 0 });

        sbreserve(&mut (*so).so_snd as *mut sbuf, val);
        sbreserve(&mut (*so).so_rcv as *mut sbuf, val);

        mss
    }
}
}

