//! From `x86_machine.c`.

// ── `if (s->cmos_data[11] & 0x04) … else` · x86_machine.c:106-110 · chunk 3979 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->cmos_data[11] & 0x04) {
//             return a;
//         } else {
//             return ((a / 10) << 4) | (a % 10);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3979 {
// Ported piece of to_bcd: the BCD conversion branch.
use core::ffi::{c_int, c_uint, c_void};

// Mirror of the C struct CMOSState (same fields, same order, same types).
#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut c_void,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[no_mangle]
pub extern "C" fn to_bcd__part3979(s: *mut CMOSState, a: c_uint) -> c_int {
    let base = unsafe { core::ptr::addr_of!((*s).cmos_data) as *const u8 };
    let data11 = unsafe { core::ptr::read(base.add(11)) };
    if data11 & 0x04 != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}
}

// ── `struct timeval tv; struct tm tm; time_t ti; int val; gettimeofday(&tv, ((void *)0) ); ti = tv.t…` · x86_machine.c:115-121 · chunk 3981 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timeval tv;
//         struct tm tm;
//         time_t ti;
//         int val;
//     
//         gettimeofday(&tv, 
//                          ((void *)0)
//                              );
//         ti = tv.tv_sec;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3981 {
// Portion of cmos_update_time: fetch current wall-clock time and copy tv_sec.
use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3981(tv: *mut Timeval, ti: *mut c_long) {
    unsafe {
        gettimeofday(tv, core::ptr::null_mut());
        *ti = (*tv).tv_sec;
    }
}
}

// ── `if (s->use_local_time) … else` · x86_machine.c:122-126 · chunk 3982 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->use_local_time) {
//             localtime_r(&ti, &tm);
//         } else {
//             gmtime_r(&ti, &tm);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3982 {
// Port of the local/gm time conversion branch of cmos_update_time.

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

extern "C" {
    fn localtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn gmtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3982(
    use_local_time: c_int,
    ti: *mut c_long,
    tm: *mut Tm,
) {
    unsafe {
        if use_local_time != 0 {
            localtime_r(ti, tm);
        } else {
            gmtime_r(ti, tm);
        }
    }
}
}

// ── `s->cmos_data[0] = to_bcd(s, tm.tm_sec); s->cmos_data[2] = to_bcd(s, tm.tm_min);` · x86_machine.c:128-129 · chunk 3983 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cmos_data[0] = to_bcd(s, tm.tm_sec);
//         s->cmos_data[2] = to_bcd(s, tm.tm_min);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3983 {
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut core::ffi::c_void,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

// Ported logic of the C `static` helper to_bcd (no linker symbol of its own).
fn to_bcd(s: *mut CMOSState, a: c_uint) -> c_int {
    unsafe {
        if ((*s).cmos_data[11] & 0x04) != 0 {
            a as c_int
        } else {
            (((a / 10) << 4) | (a % 10)) as c_int
        }
    }
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3983(s: *mut CMOSState, tm_sec: c_int, tm_min: c_int) {
    unsafe {
        (*s).cmos_data[0] = to_bcd(s, tm_sec as c_uint) as u8;
        (*s).cmos_data[2] = to_bcd(s, tm_min as c_uint) as u8;
    }
}
}

// ── `if (s->cmos_data[11] & 0x02) … else` · x86_machine.c:130-136 · chunk 3984 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->cmos_data[11] & 0x02) {
//             s->cmos_data[4] = to_bcd(s, tm.tm_hour);
//         } else {
//             s->cmos_data[4] = to_bcd(s, tm.tm_hour % 12);
//             if (tm.tm_hour >= 12)
//                 s->cmos_data[4] |= 0x80;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3984 {
// Port of the hour-register update part of cmos_update_time.
use core::ffi::{c_int, c_void};

// Mirror of the C CMOSState struct (same fields, same order, same types).
#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut c_void, // IRQSignal *
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

// Ported logic of the file-static C helper to_bcd().
fn to_bcd(s: *mut CMOSState, a: u32) -> c_int {
    unsafe {
        let data = core::ptr::addr_of_mut!((*s).cmos_data) as *mut u8;
        if (*data.add(11)) & 0x04 != 0 {
            a as c_int
        } else {
            (((a / 10) << 4) | (a % 10)) as c_int
        }
    }
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3984(s: *mut CMOSState, tm_hour: c_int) {
    unsafe {
        let data = core::ptr::addr_of_mut!((*s).cmos_data) as *mut u8;
        if (*data.add(11)) & 0x02 != 0 {
            let v = to_bcd(s, tm_hour as u32);
            *data.add(4) = v as u8;
        } else {
            let v = to_bcd(s, (tm_hour % 12) as u32);
            *data.add(4) = v as u8;
            if tm_hour >= 12 {
                *data.add(4) |= 0x80;
            }
        }
    }
}
}

// ── `s->cmos_data[6] = to_bcd(s, tm.tm_wday); s->cmos_data[7] = to_bcd(s, tm.tm_mday); s->cmos_data[…` · x86_machine.c:137-140 · chunk 3985 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cmos_data[6] = to_bcd(s, tm.tm_wday);
//         s->cmos_data[7] = to_bcd(s, tm.tm_mday);
//         s->cmos_data[8] = to_bcd(s, tm.tm_mon + 1);
//         s->cmos_data[9] = to_bcd(s, tm.tm_year % 100);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3985 {
// Port of the weekday/day/month/year BCD update inside cmos_update_time.
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct IRQSignal {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut IRQSignal,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

// static in C: ported logic (no linker symbol).
fn to_bcd(s: &CMOSState, a: c_uint) -> c_int {
    if s.cmos_data[11] & 0x04 != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3985(
    s: *mut CMOSState,
    tm_wday: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
) {
    if s.is_null() {
        return;
    }
    unsafe {
        let st = &mut *s;
        let v6 = to_bcd(st, tm_wday as c_uint) as u8;
        let v7 = to_bcd(st, tm_mday as c_uint) as u8;
        let v8 = to_bcd(st, tm_mon.wrapping_add(1) as c_uint) as u8;
        let v9 = to_bcd(st, (tm_year % 100) as c_uint) as u8;
        st.cmos_data[6] = v6;
        st.cmos_data[7] = v7;
        st.cmos_data[8] = v8;
        st.cmos_data[9] = v9;
    }
}
}

// ── `if (set_century)` · x86_machine.c:142-147 · chunk 3986 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (set_century) {
//     
//             val = to_bcd(s, (tm.tm_year / 100) + 19);
//             s->cmos_data[0x32] = val;
//             s->cmos_data[0x37] = val;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3986 {
// Port of the set_century branch of cmos_update_time (x86_machine.c), together
// with the static helper to_bcd whose logic is inlined as a private fn.

use core::ffi::c_int;

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut core::ffi::c_void,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

// static int to_bcd(CMOSState *s, unsigned int a)
#[inline]
unsafe fn to_bcd(s: *mut CMOSState, a: u32) -> c_int {
    if ((*s).cmos_data[11] & 0x04) != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmos_update_time__part3986(
    s: *mut CMOSState,
    tm_year: c_int,
) -> c_int {
    // val = to_bcd(s, (tm.tm_year / 100) + 19);
    let arg: u32 = (tm_year / 100).wrapping_add(19) as u32;
    let val: c_int = to_bcd(s, arg);
    (*s).cmos_data[0x32] = val as u8;
    (*s).cmos_data[0x37] = val as u8;
    val
}
}

// ── `if (tv.tv_usec < 244) … else` · x86_machine.c:150-154 · chunk 3987 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tv.tv_usec < 244) {
//             s->cmos_data[10] |= 0x80;
//         } else {
//             s->cmos_data[10] &= ~0x80;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3987 {
#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut core::ffi::c_void,
    pub use_local_time: core::ffi::c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[no_mangle]
pub extern "C" fn cmos_update_time__part3987(s: *mut CMOSState, tv_usec: core::ffi::c_long) {
    unsafe {
        if s.is_null() {
            return;
        }
        if tv_usec < 244 as core::ffi::c_long {
            (*s).cmos_data[10] |= 0x80u8;
        } else {
            (*s).cmos_data[10] &= 0x7fu8;
        }
    }
}
}

// ── `struct timespec ts; clock_gettime( 1 , &ts); return (uint32_t)ts.tv_sec * 32768 + ((uint64_t)ts…` · x86_machine.c:185-189 · chunk 3991 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint32_t)ts.tv_sec * 32768 +
//             ((uint64_t)ts.tv_nsec * 32768 / 1000000000);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3991 {
use core::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn cmos_get_timer__part3991() -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        clock_gettime(1, &mut ts);
    }

    // (uint32_t)ts.tv_sec * 32768  -- 32-bit wrapping arithmetic
    let sec_term: u32 = (ts.tv_sec as u32).wrapping_mul(32768);
    // (uint64_t)ts.tv_nsec * 32768 / 1000000000  -- 64-bit arithmetic
    let nsec_term: u64 = (ts.tv_nsec as u64).wrapping_mul(32768) / 1000000000;

    // uint32_t + uint64_t promotes to 64-bit, then truncates to uint32_t
    sec_term.wrapping_add(nsec_term as u32)
}
}

// ── `int period_code; period_code = s->cmos_data[10] & 0x0f;` · x86_machine.c:194-196 · chunk 3993 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int period_code;
//     
//         period_code = s->cmos_data[10] & 0x0f;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3993 {
// Extracted from cmos_update_timer: read the periodic-timer period code.
use core::ffi::c_int;

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut core::ffi::c_void,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[no_mangle]
pub extern "C" fn cmos_update_timer__part3993(s: *mut CMOSState) -> c_int {
    let data = unsafe { (*s).cmos_data[10] };
    (data & 0x0f) as c_int
}
}

// ── `if (s->cmos_data[11] & 0x40) && period_code != 0)` · x86_machine.c:197-204 · chunk 3994 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((s->cmos_data[11] & 0x40) &&
//             period_code != 0) {
//             if (period_code <= 2)
//                 period_code += 7;
//             s->irq_period = 1 << (period_code - 1);
//             s->irq_timeout = (cmos_get_timer(s) + s->irq_period) &
//                 ~(s->irq_period - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3994 {
use core::ffi::{c_int, c_long, c_void};

// Mirror of the C CMOSState struct (fields in order, same types).
#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut c_void,
    pub use_local_time: c_int,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

// Port of the static C helper cmos_get_timer (no linker symbol available).
unsafe fn cmos_get_timer(_s: *mut CMOSState) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_REALTIME */, &mut ts as *mut Timespec);
    let sec_part = (ts.tv_sec as u32).wrapping_mul(32768);
    let nsec_part = ((ts.tv_nsec as u64)
        .wrapping_mul(32768)
        .wrapping_div(1000000000)) as u32;
    sec_part.wrapping_add(nsec_part)
}

#[no_mangle]
pub extern "C" fn cmos_update_timer__part3994(s: *mut CMOSState, period_code_in: c_int) {
    let s = s;
    let mut period_code = period_code_in;
    unsafe {
        if ((*s).cmos_data[11] as c_int & 0x40) != 0 && period_code != 0 {
            if period_code <= 2 {
                period_code = period_code.wrapping_add(7);
            }
            let period: u32 = (1 as c_int).wrapping_shl((period_code - 1) as u32) as u32;
            (*s).irq_period = period;
            (*s).irq_timeout = cmos_get_timer(s)
                .wrapping_add((*s).irq_period)
                & !((*s).irq_period).wrapping_sub(1);
        }
    }
}
}

// ── `if (s->cmos_data[11] & 0x40)` · x86_machine.c:212-222 · chunk 3996 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->cmos_data[11] & 0x40) {
//             d = cmos_get_timer(s) - s->irq_timeout;
//             if ((int32_t)d >= 0) {
//     
//     
//                 s->cmos_data[12] |= 0xc0;
//                 set_irq(s->irq, 1);
//     
//                 s->irq_timeout += s->irq_period;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3996 {
// Port of the timing/IRQ-raising block of cmos_update_irq (x86_machine.c).
use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut IRQSignal,
    pub use_local_time: c_int, // BOOL == int
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,  // __time_t      == long int
    tv_nsec: i64, // __syscall_slong_t == long int
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

// CLOCK_REALTIME on Linux
const CLOCK_REALTIME: c_int = 1;

// Port of the static C function cmos_get_timer (it ignores its argument).
unsafe fn cmos_get_timer_impl() -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_REALTIME, &mut ts);
    let a = (ts.tv_sec as u32).wrapping_mul(32768u32);
    let b = ((ts.tv_nsec as u64).wrapping_mul(32768u64) / 1000000000u64) as u32;
    a.wrapping_add(b)
}

#[no_mangle]
pub unsafe extern "C" fn cmos_update_irq__part3996(s: *mut CMOSState) {
    if s.is_null() {
        return;
    }
    let st = &mut *s;
    if st.cmos_data[11] & 0x40 != 0 {
        let d: u32 = cmos_get_timer_impl().wrapping_sub(st.irq_timeout);
        if (d as i32) >= 0 {
            st.cmos_data[12] |= 0xc0;

            let irq = st.irq;
            if !irq.is_null() {
                let sig = &mut *irq;
                if let Some(f) = sig.set_irq {
                    f(sig.opaque, sig.irq_num, 1);
                }
            }

            st.irq_timeout = st.irq_timeout.wrapping_add(st.irq_period);
        }
    }
}
}

// ── `if (offset == 0) … else` · x86_machine.c:230-250 · chunk 3998 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset == 0) {
//             s->cmos_index = data & 0x7f;
//         } else {
//     
//     
//     
//             switch(s->cmos_index) {
//             case 10:
//                 s->cmos_data[10] = (data & ~0x80) |
//                     (s->cmos_data[10] & 0x80);
//                 cmos_update_timer(s);
//                 break;
//             case 11:
//                 s->cmos_data[s->cmos_index] = data;
//                 cmos_update_timer(s);
//                 break;
//             default:
//                 s->cmos_data[s->cmos_index] = data;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3998 {
// Port of the cmos_write switch/indexing logic into its own exported function.
use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
struct CmosState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut c_void, // IRQSignal *
    use_local_time: c_int,
    irq_timeout: u32,
    irq_period: u32,
}

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

unsafe fn cmos_get_timer(s: *mut CmosState) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1, &mut ts);
    let a = (ts.tv_sec as u32).wrapping_mul(32768) as u64;
    let b = (ts.tv_nsec as u64).wrapping_mul(32768) / 1000000000u64;
    (a.wrapping_add(b)) as u32
}

unsafe fn cmos_update_timer(s: *mut CmosState) {
    let data10 = *(*s).cmos_data.as_ptr().add(10);
    let data11 = *(*s).cmos_data.as_ptr().add(11);
    let mut period_code: c_int = (data10 & 0x0f) as c_int;
    if (data11 & 0x40) != 0 && period_code != 0 {
        if period_code <= 2 {
            period_code += 7;
        }
        let period: u32 = (1u32).wrapping_shl((period_code - 1) as u32);
        (*s).irq_period = period;
        (*s).irq_timeout = cmos_get_timer(s).wrapping_add(period) & !(period.wrapping_sub(1));
    }
}

#[no_mangle]
pub extern "C" fn cmos_write__part3998(
    opaque: *mut c_void,
    offset: u32,
    data: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut CmosState;
    unsafe {
        if offset == 0 {
            (*s).cmos_index = (data & 0x7f) as u8;
        } else {
            let idx = (*s).cmos_index;
            match idx {
                10 => {
                    let base = (*s).cmos_data.as_mut_ptr();
                    let old = *base.add(10);
                    let val = ((data & !0x80u32) | ((old & 0x80) as u32)) & 0xff;
                    *base.add(10) = val as u8;
                    cmos_update_timer(s);
                }
                11 => {
                    let base = (*s).cmos_data.as_mut_ptr();
                    *base.add(idx as usize) = data as u8;
                    cmos_update_timer(s);
                }
                _ => {
                    let base = (*s).cmos_data.as_mut_ptr();
                    *base.add(idx as usize) = data as u8;
                }
            }
        }
    }
}
}

// ── `s->last_irr = 0; s->irr = 0; s->imr = 0; s->isr = 0; s->priority_add = 0; s->irq_base = 0; s->r…` · x86_machine.c:343-354 · chunk 4007 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->last_irr = 0;
//         s->irr = 0;
//         s->imr = 0;
//         s->isr = 0;
//         s->priority_add = 0;
//         s->irq_base = 0;
//         s->read_reg_select = 0;
//         s->special_mask = 0;
//         s->init_state = 0;
//         s->auto_eoi = 0;
//         s->rotate_on_autoeoi = 0;
//         s->init4 = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4007 {
// Reset the PIC's internal registers to zero.
#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_reset__part4007(s: *mut PICState) {
    if s.is_null() {
        return;
    }
    let p = &mut *s;
    p.last_irr = 0;
    p.irr = 0;
    p.imr = 0;
    p.isr = 0;
    p.priority_add = 0;
    p.irq_base = 0;
    p.read_reg_select = 0;
    p.special_mask = 0;
    p.init_state = 0;
    p.auto_eoi = 0;
    p.rotate_on_autoeoi = 0;
    p.init4 = 0;
}
}

// ── `int mask; mask = 1 << irq;` · x86_machine.c:360-361 · chunk 4009 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int mask;
//         mask = 1 << irq;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4009 {
// Computes the IRQ bit mask: mask = 1 << irq  (as in the original C code).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pic_set_irq1__part4009(irq: c_int) -> c_int {
    // C `1 << irq` on a 32-bit int; mirror x86 shift semantics (count masked to 5 bits).
    (1i32).wrapping_shl(irq as u32) as c_int
}
}

// ── `if (s->elcr & mask) … else` · x86_machine.c:362-380 · chunk 4010 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->elcr & mask) {
//     
//             if (level) {
//                 s->irr |= mask;
//                 s->last_irr |= mask;
//             } else {
//                 s->irr &= ~mask;
//                 s->last_irr &= ~mask;
//             }
//         } else {
//     
//             if (level) {
//                 if ((s->last_irr & mask) == 0)
//                     s->irr |= mask;
//                 s->last_irr |= mask;
//             } else {
//                 s->last_irr &= ~mask;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4010 {
// Port of the IRQ-level update branch of pic_set_irq1 from x86_machine.c
use core::ffi::{c_int, c_void};

pub type PICUpdateIRQFunc = unsafe extern "C" fn(opaque: *mut c_void);

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: Option<PICUpdateIRQFunc>,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_set_irq1__part4010(s: *mut PICState, mask: c_int, level: c_int) {
    if s.is_null() {
        return;
    }
    let st = &mut *s;
    if (st.elcr as c_int & mask) != 0 {
        if level != 0 {
            st.irr |= mask as u8;
            st.last_irr |= mask as u8;
        } else {
            st.irr &= !(mask as u8);
            st.last_irr &= !(mask as u8);
        }
    } else {
        if level != 0 {
            if (st.last_irr as c_int & mask) == 0 {
                st.irr |= mask as u8;
            }
            st.last_irr |= mask as u8;
        } else {
            st.last_irr &= !(mask as u8);
        }
    }
}
}

// ── `if (mask == 0)` · x86_machine.c:386-387 · chunk 4012 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mask == 0)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4012 {
// Piece of pic_get_priority: early return when the interrupt mask is empty.
use core::ffi::c_int;

/// Returns 1 when `mask` is zero (the C code then returns -1), 0 otherwise.
#[no_mangle]
pub extern "C" fn pic_get_priority__part4012(mask: c_int) -> c_int {
    if mask == 0 {
        1
    } else {
        0
    }
}
}

// ── `priority = 7;` · x86_machine.c:388-388 · chunk 4013 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     priority = 7;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4013 {
// Piece of pic_get_priority: initialise the running priority to 7.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pic_get_priority__part4013() -> c_int {
    7
}
}

// ── `while (mask & (1 << ((priority + s->priority_add) & 7))) == 0)` · x86_machine.c:389-390 · chunk 4014 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((mask & (1 << ((priority + s->priority_add) & 7))) == 0)
//             priority--;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4014 {
// Ported piece of pic_get_priority: the priority scan loop.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_get_priority__part4014(
    s: *const PICState,
    mask: c_int,
    mut priority: c_int,
) -> c_int {
    loop {
        let add = (*s).priority_add as c_int;
        let bit = (priority.wrapping_add(add)) & 7;
        if (mask & (1 << bit)) != 0 {
            break;
        }
        priority = priority.wrapping_sub(1);
    }
    priority
}
}

// ── `return priority;` · x86_machine.c:391-391 · chunk 4015 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return priority;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4015 {
// Port of the `return priority;` statement of pic_get_priority.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pic_get_priority__part4015(priority: c_int) -> c_int {
    priority
}
}

// ── `int mask, cur_priority, priority; mask = s->irr & ~s->imr; priority = pic_get_priority(s, mask);` · x86_machine.c:397-400 · chunk 4017 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int mask, cur_priority, priority;
//     
//         mask = s->irr & ~s->imr;
//         priority = pic_get_priority(s, mask);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4017 {
// Port of the first part of pic_get_irq: compute mask and priority.

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

// The C function pic_get_priority is static (no linker symbol), so its
// logic is ported here as a private Rust function.
unsafe fn pic_get_priority(s: *const PICState, mask: core::ffi::c_int) -> core::ffi::c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: core::ffi::c_int = 7;
    loop {
        let shift: core::ffi::c_int =
            (priority + ((*s).priority_add as core::ffi::c_int)) & 7;
        let bit: core::ffi::c_int = (1 as core::ffi::c_int).wrapping_shl(shift as u32);
        if (mask & bit) != 0 {
            break;
        }
        priority -= 1;
    }
    priority
}

#[no_mangle]
pub unsafe extern "C" fn pic_get_irq__part4017(
    s: *mut PICState,
    mask: *mut core::ffi::c_int,
    priority: *mut core::ffi::c_int,
) {
    let m: core::ffi::c_int =
        ((*s).irr as core::ffi::c_int) & !((*s).imr as core::ffi::c_int);
    *mask = m;
    *priority = pic_get_priority(s, m);
}
}

// ── `if (priority < 0)` · x86_machine.c:401-402 · chunk 4018 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (priority < 0)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4018 {
// Portion of pic_get_irq: decide whether an invalid (negative) priority was computed.
// Returns nonzero when the caller must take its early `return -1;` path, else 0.

#[no_mangle]
pub extern "C" fn pic_get_irq__part4018(priority: core::ffi::c_int) -> core::ffi::c_int {
    if priority < 0 {
        1
    } else {
        0
    }
}
}

// ── `cur_priority = pic_get_priority(s, s->isr);` · x86_machine.c:404-404 · chunk 4019 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     cur_priority = pic_get_priority(s, s->isr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4019 {
// Portion of pic_get_irq: compute current priority of the in-service register.
// pic_get_priority is a static C function (no linker symbol), so its logic is
// reproduced here as a private Rust function.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

// Mirrors the static C function pic_get_priority(PICState *s, int mask).
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let priority_add = (*s).priority_add as c_int;
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + priority_add) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

#[no_mangle]
pub unsafe extern "C" fn pic_get_irq__part4019(s: *mut PICState) -> c_int {
    let isr = (*s).isr as c_int;
    pic_get_priority(s, isr)
}
}

// ── `if (priority > cur_priority) … else` · x86_machine.c:405-410 · chunk 4020 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (priority > cur_priority) {
//     
//             return priority;
//         } else {
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4020 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pic_get_irq__part4020(priority: c_int, cur_priority: c_int) -> c_int {
    if priority > cur_priority {
        priority
    } else {
        -1
    }
}
}

// ── `if (s->auto_eoi) … else` · x86_machine.c:416-421 · chunk 4022 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->auto_eoi) {
//             if (s->rotate_on_autoeoi)
//                 s->priority_add = (irq + 1) & 7;
//         } else {
//             s->isr |= (1 << irq);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4022 {
use core::ffi::{c_int, c_void};

// Mirror of the C PICState struct (fields in order, same types).
#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_intack__part4022(s: *mut PICState, irq: c_int) {
    if s.is_null() {
        return;
    }
    let p = &mut *s;
    if p.auto_eoi != 0 {
        if p.rotate_on_autoeoi != 0 {
            p.priority_add = (((irq as i32).wrapping_add(1)) & 7) as u8;
        }
    } else {
        p.isr |= (1u32 << (irq as u32 & 31)) as u8;
    }
}
}

// ── `if (!(s->elcr & (1 << irq)` · x86_machine.c:423-424 · chunk 4023 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->elcr & (1 << irq)))
//             s->irr &= ~(1 << irq);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4023 {
use core::ffi::{c_int, c_void};

pub type PICUpdateIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void)>;

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: PICUpdateIRQFunc,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_intack__part4023(s: *mut PICState, irq: c_int) {
    let s = &mut *s;
    let mask: c_int = 1 << irq;
    if (s.elcr as c_int) & mask == 0 {
        s.irr = (s.irr as c_int & !mask) as u8;
    }
}
}

// ── `PICState *s = opaque; int priority, addr; addr = offset & 1;` · x86_machine.c:430-433 · chunk 4025 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PICState *s = opaque;
//         int priority, addr;
//     
//         addr = offset & 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4025 {
// Rust port of the prologue of pic_write():
//   PICState *s = opaque;
//   int priority, addr;
//   addr = offset & 1;
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn pic_write__part4025(
    opaque: *mut c_void,
    offset: u32,
    s_out: *mut *mut c_void,
    addr_out: *mut c_int,
) {
    unsafe {
        if !s_out.is_null() {
            *s_out = opaque;
        }
        if !addr_out.is_null() {
            *addr_out = (offset & 1u32) as c_int;
        }
    }
}
}

// ── `PICState *s = opaque; int addr, ret; addr = offset & 1;` · x86_machine.c:530-533 · chunk 4028 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PICState *s = opaque;
//         int addr, ret;
//     
//         addr = offset & 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4028 {
// Ported piece of pic_read: local setup and register address decoding.
use core::ffi::{c_int, c_void};

pub type PICUpdateIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void)>;

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: PICUpdateIRQFunc,
    pub opaque: *mut c_void,
}

/// Equivalent of:
///     PICState *s = opaque;
///     int addr = offset & 1;
/// `s` is returned through `s_out`, `addr` as the return value.
#[no_mangle]
pub extern "C" fn pic_read__part4028(
    opaque: *mut c_void,
    offset: u32,
    s_out: *mut *mut PICState,
) -> c_int {
    let s: *mut PICState = opaque as *mut PICState;
    if !s_out.is_null() {
        unsafe {
            *s_out = s;
        }
    }
    (offset & 1) as c_int
}
}

// ── `if (addr == 0) … else` · x86_machine.c:534-541 · chunk 4029 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 0) {
//             if (s->read_reg_select)
//                 ret = s->isr;
//             else
//                 ret = s->irr;
//         } else {
//             ret = s->imr;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4029 {
// Port of pic_read's register-select branch.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn pic_read__part4029(s: *mut PICState, addr: c_int) -> u32 {
    unsafe {
        if addr == 0 {
            if (*s).read_reg_select != 0 {
                (*s).isr as u32
            } else {
                (*s).irr as u32
            }
        } else {
            (*s).imr as u32
        }
    }
}
}

// ── `return ret;` · x86_machine.c:545-545 · chunk 4030 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4030 {
// Ported piece of pic_read: convert the int result to the uint32_t return value.
#[no_mangle]
pub extern "C" fn pic_read__part4030(ret: core::ffi::c_int) -> u32 {
    ret as u32
}
}

// ── `PICState *s = opaque; s->elcr = val & s->elcr_mask;` · x86_machine.c:551-552 · chunk 4032 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PICState *s = opaque;
//         s->elcr = val & s->elcr_mask;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4032 {
#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_elcr_write__part4032(s: *mut PICState, val: u32) {
    let s = &mut *s;
    s.elcr = (val & (s.elcr_mask as u32)) as u8;
}
}

// ── `PICState *s = opaque; return s->elcr;` · x86_machine.c:557-558 · chunk 4034 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PICState *s = opaque;
//         return s->elcr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4034 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_elcr_read__part4034(
    opaque: *mut c_void,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    let _ = offset;
    let _ = size_log2;
    let s = opaque as *const PICState;
    (*s).elcr as u32
}
}

// ── `PIC2State *s; int i; s = mallocz(sizeof(*s));` · x86_machine.c:580-583 · chunk 4038 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PIC2State *s;
//         int i;
//     
//         s = mallocz(sizeof(*s));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4038 {
// Port of the allocation part of pic2_init(): allocates and zeroes a PIC2State.
use core::ffi::{c_int, c_void};

#[repr(C)]
struct Pic2State {
    pics: [*mut c_void; 2],
    irq_requested: c_int,
    cpu_set_irq: *mut c_void,
    opaque: *mut c_void,
    irqs: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn pic2_init__part4038() -> *mut c_void {
    unsafe { mallocz(core::mem::size_of::<Pic2State>()) }
}
}

// ── `for (i = 0; i < 2; i++)` · x86_machine.c:599-601 · chunk 4042 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 2; i++) {
//             s->pics[i]->elcr = elcr[i] & s->pics[i]->elcr_mask;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4042 {
// Port of the loop inside pic2_set_elcr (x86_machine.c lines 599-601).

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: core::ffi::c_int,
    pub cpu_set_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
    pub irqs: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic2_set_elcr__part4042(s: *mut PIC2State, elcr: *const u8) {
    let mut i: core::ffi::c_int = 0;
    while i < 2 {
        let pic: *mut PICState = *(*s).pics.as_ptr().add(i as usize);
        let val: u8 = *elcr.add(i as usize) & (*pic).elcr_mask;
        (*pic).elcr = val;
        i += 1;
    }
}
}

// ── `if (irq2 >= 0)` · x86_machine.c:613-617 · chunk 4045 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (irq2 >= 0) {
//     
//             pic_set_irq1(s->pics[0], 2, 1);
//             pic_set_irq1(s->pics[0], 2, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4045 {
// Port of the irq2 >= 0 branch of pic2_update_irq: raises and clears
// the cascade IRQ (line 2) on the master PIC.
use core::ffi::c_int;
use core::ffi::c_void;

pub type PICUpdateIRQFunc = unsafe extern "C" fn(opaque: *mut c_void);

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

// Ported from the static C helper pic_set_irq1.
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let s = &mut *s;
    let mask: c_int = 1 << irq;
    if (s.elcr as c_int) & mask != 0 {
        if level != 0 {
            s.irr = (s.irr as c_int | mask) as u8;
            s.last_irr = (s.last_irr as c_int | mask) as u8;
        } else {
            s.irr = (s.irr as c_int & !mask) as u8;
            s.last_irr = (s.last_irr as c_int & !mask) as u8;
        }
    } else {
        if level != 0 {
            if (s.last_irr as c_int) & mask == 0 {
                s.irr = (s.irr as c_int | mask) as u8;
            }
            s.last_irr = (s.last_irr as c_int | mask) as u8;
        } else {
            s.last_irr = (s.last_irr as c_int & !mask) as u8;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pic2_update_irq__part4045(s: *mut PIC2State, irq2: c_int) {
    if irq2 >= 0 {
        let master = (*s).pics[0];
        pic_set_irq1(master, 2, 1);
        pic_set_irq1(master, 2, 0);
    }
}
}

// ── `irq = pic_get_irq(s->pics[0]);` · x86_machine.c:619-619 · chunk 4046 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     irq = pic_get_irq(s->pics[0]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4046 {
// Ported piece of pic2_update_irq: irq = pic_get_irq(s->pics[0]);
use core::ffi::{c_int, c_void};

type PICUpdateIRQFunc = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: Option<PICUpdateIRQFunc>,
    pub opaque: *mut c_void,
}

// Port of the static C function pic_get_priority (no linker symbol available).
fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    let priority_add: c_int = unsafe { (*s).priority_add } as c_int;
    while (mask & (1i32 << ((priority + priority_add) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

// Port of the static C function pic_get_irq (no linker symbol available).
fn pic_get_irq(s: *mut PICState) -> c_int {
    let mask: c_int = unsafe { ((*s).irr & !(*s).imr) } as c_int;
    let priority: c_int = pic_get_priority(s, mask);
    if priority < 0 {
        return -1;
    }
    let cur_priority: c_int = pic_get_priority(s, unsafe { (*s).isr } as c_int);
    if priority > cur_priority {
        priority
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn pic2_update_irq__part4046(pics0: *mut PICState) -> c_int {
    pic_get_irq(pics0)
}
}

// ── `if (irq >= 0) … else` · x86_machine.c:623-629 · chunk 4047 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (irq >= 0) {
//     
//             s->cpu_set_irq(s->opaque, 1);
//         } else {
//     
//             s->cpu_set_irq(s->opaque, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4047 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut c_void; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic2_update_irq__part4047(s: *mut PIC2State, irq: c_int) {
    if s.is_null() {
        return;
    }
    let level: c_int = if irq >= 0 { 1 } else { 0 };
    let st = &mut *s;
    if let Some(f) = st.cpu_set_irq {
        f(st.opaque, level);
    }
}
}

// ── `PIC2State *s = opaque; pic_set_irq1(s->pics[irq >> 3], irq & 7, level); pic2_update_irq(s);` · x86_machine.c:634-642 · chunk 4049 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PIC2State *s = opaque;
//     
//     
//     
//     
//     
//     
//         pic_set_irq1(s->pics[irq >> 3], irq & 7, level);
//         pic2_update_irq(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4049 {
// Port of the body of pic2_set_irq (x86_machine.c lines 634-642).
// pic_set_irq1, pic_get_priority, pic_get_irq and pic2_update_irq are all
// `static` in the C translation unit, so their logic is mirrored here.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void)>,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

#[inline]
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = 1 << irq;
    let m8 = mask as u8;
    if ((*s).elcr as c_int) & mask != 0 {
        if level != 0 {
            (*s).irr |= m8;
            (*s).last_irr |= m8;
        } else {
            (*s).irr &= !m8;
            (*s).last_irr &= !m8;
        }
    } else {
        if level != 0 {
            if ((*s).last_irr as c_int) & mask == 0 {
                (*s).irr |= m8;
            }
            (*s).last_irr |= m8;
        } else {
            (*s).last_irr &= !m8;
        }
    }
}

#[inline]
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    let mut priority: c_int;
    if mask == 0 {
        return -1;
    }
    priority = 7;
    while (mask & (1 << ((priority + (*s).priority_add as c_int) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

#[inline]
unsafe fn pic_get_irq(s: *mut PICState) -> c_int {
    let mask: c_int = ((*s).irr & !(*s).imr) as c_int;
    let priority: c_int = pic_get_priority(s, mask);
    if priority < 0 {
        return -1;
    }
    let cur_priority: c_int = pic_get_priority(s, (*s).isr as c_int);
    if priority > cur_priority {
        priority
    } else {
        -1
    }
}

#[inline]
unsafe fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;
    let irq2: c_int = pic_get_irq(*(*s).pics.as_ptr().add(1));
    if irq2 >= 0 {
        pic_set_irq1(*(*s).pics.as_ptr(), 2, 1);
        pic_set_irq1(*(*s).pics.as_ptr(), 2, 0);
    }
    let irq: c_int = pic_get_irq(*(*s).pics.as_ptr());
    let level: c_int = if irq >= 0 { 1 } else { 0 };
    if let Some(f) = (*s).cpu_set_irq {
        f((*s).opaque, level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pic2_set_irq__part4049(opaque: *mut c_void, irq: c_int, level: c_int) {
    let s = opaque as *mut PIC2State;
    let pic_index: usize = (irq >> 3) as usize;
    let pic: *mut PICState = *(*s).pics.as_ptr().add(pic_index);
    pic_set_irq1(pic, irq & 7, level);
    pic2_update_irq(s as *mut c_void);
}
}

// ── `int irq, irq2, intno; irq = pic_get_irq(s->pics[0]);` · x86_machine.c:648-650 · chunk 4051 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int irq, irq2, intno;
//     
//         irq = pic_get_irq(s->pics[0]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4051 {
use core::ffi::c_int;

// Mirror of the C PICState struct (only fields accessed are declared, but the
// layout up to those fields must match exactly).
#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

// static in C: ported logic, private here.
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + ((*s).priority_add as c_int)) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

unsafe fn pic_get_irq(s: *mut PICState) -> c_int {
    let mask: c_int = ((*s).irr as c_int) & !((*s).imr as c_int);
    let priority = pic_get_priority(s, mask);
    if priority < 0 {
        return -1;
    }
    let cur_priority = pic_get_priority(s, (*s).isr as c_int);
    if priority > cur_priority {
        priority
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn pic2_get_hard_intno__part4051(
    pic0: *mut PICState,
    irq_out: *mut c_int,
) -> c_int {
    let irq = pic_get_irq(pic0);
    *irq_out = irq;
    irq
}
}

// ── `if (irq >= 0) … else` · x86_machine.c:651-670 · chunk 4052 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (irq >= 0) {
//             pic_intack(s->pics[0], irq);
//             if (irq == 2) {
//                 irq2 = pic_get_irq(s->pics[1]);
//                 if (irq2 >= 0) {
//                     pic_intack(s->pics[1], irq2);
//                 } else {
//     
//                     irq2 = 7;
//                 }
//                 intno = s->pics[1]->irq_base + irq2;
//                 irq = irq2 + 8;
//             } else {
//                 intno = s->pics[0]->irq_base + irq;
//             }
//         } else {
//     
//             irq = 7;
//             intno = s->pics[0]->irq_base + irq;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4052 {
// Port of the IRQ-acknowledge block of pic2_get_hard_intno (i8259 pair logic).
// pic_get_irq / pic_get_priority / pic_intack are `static` in the C file, so their
// logic is mirrored here as private Rust functions.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PICState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_autoeoi: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void)>,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub opaque: *mut c_void,
    pub irqs: *mut IRQSignal,
}

#[inline]
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + ((*s).priority_add as c_int)) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

unsafe fn pic_get_irq(s: *mut PICState) -> c_int {
    let mask: c_int = ((*s).irr & !(*s).imr) as c_int;
    let priority = pic_get_priority(s, mask);
    if priority < 0 {
        return -1;
    }
    let cur_priority = pic_get_priority(s, (*s).isr as c_int);
    if priority > cur_priority {
        priority
    } else {
        -1
    }
}

unsafe fn pic_intack(s: *mut PICState, irq: c_int) {
    if (*s).auto_eoi != 0 {
        if (*s).rotate_on_autoeoi != 0 {
            (*s).priority_add = (((irq + 1) & 7) as u8) & 0xff;
        }
    } else {
        (*s).isr |= (1 << irq) as u8;
    }
    if ((*s).elcr & ((1 << irq) as u8)) == 0 {
        (*s).irr &= !((1 << irq) as u8);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pic2_get_hard_intno__part4052(
    s: *mut PIC2State,
    irq: *mut c_int,
    irq2: *mut c_int,
    intno: *mut c_int,
) {
    let mut irq_v: c_int = *irq;
    let mut irq2_v: c_int = *irq2;
    let mut intno_v: c_int = *intno;

    if irq_v >= 0 {
        pic_intack((*s).pics[0], irq_v);
        if irq_v == 2 {
            irq2_v = pic_get_irq((*s).pics[1]);
            if irq2_v >= 0 {
                pic_intack((*s).pics[1], irq2_v);
            } else {
                irq2_v = 7;
            }
            intno_v = (*(*s).pics[1]).irq_base as c_int + irq2_v;
            irq_v = irq2_v + 8;
        } else {
            intno_v = (*(*s).pics[0]).irq_base as c_int + irq_v;
        }
    } else {
        irq_v = 7;
        intno_v = (*(*s).pics[0]).irq_base as c_int + irq_v;
    }

    *irq = irq_v;
    *irq2 = irq2_v;
    *intno = intno_v;
}
}

// ── `PITState *s; PITChannel *pc; int i; s = mallocz(sizeof(*s)); s->irq = irq; s->get_ticks = get_t…` · x86_machine.c:730-738 · chunk 4060 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *s;
//         PITChannel *pc;
//         int i;
//     
//         s = mallocz(sizeof(*s));
//     
//         s->irq = irq;
//         s->get_ticks = get_ticks;
//         s->opaque = opaque;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4060 {
// Portion of pit_init: allocate the PITState and store its back-pointers.
use core::ffi::c_void;

#[repr(C)]
struct PITChannel {
    pit_state: *mut PITState,
    count: u32,
    latched_count: u32,
    rw_state: u8,
    mode: u8,
    bcd: u8,
    gate: u8,
    count_load_time: i64,
    last_irq_time: i64,
}

#[repr(C)]
struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: *mut c_void,
    irq: *mut c_void,
    opaque: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn pit_init__part4060(
    irq: *mut c_void,
    get_ticks: *mut c_void,
    opaque: *mut c_void,
) -> *mut PITState {
    unsafe {
        let s = mallocz(core::mem::size_of::<PITState>()) as *mut PITState;
        (*s).irq = irq;
        (*s).get_ticks = get_ticks;
        (*s).opaque = opaque;
        s
    }
}
}

// ── `PITState *s = pc->pit_state; return s->get_ticks(s->opaque);` · x86_machine.c:760-761 · chunk 4064 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *s = pc->pit_state;
//         return s->get_ticks(s->opaque);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4064 {
// Rust rewrite of pit_get_time (part).
use core::ffi::{c_int, c_void};

// Opaque mirror of the C `IRQSignal` type (only pointers to it appear here).
#[repr(C)]
pub struct IRQSignal {
    _opaque: [u8; 0],
}

pub type PITGetTicksFunc = unsafe extern "C" fn(opaque: *mut c_void) -> i64;

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut PITState,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: Option<PITGetTicksFunc>,
    pub irq: *mut IRQSignal,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pit_get_time__part4064(pc: *mut PITChannel) -> i64 {
    let s: *mut PITState = (*pc).pit_state;
    let get_ticks = match (*s).get_ticks {
        Some(f) => f,
        None => return 0,
    };
    get_ticks((*s).opaque)
}
}

// ── `switch (pc->mode)` · x86_machine.c:770-780 · chunk 4067 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(pc->mode) {
//         case 0:
//         case 1:
//         case 4:
//         case 5:
//             counter = (pc->count - d) & 0xffff;
//             break;
//         default:
//             counter = pc->count - (d % pc->count);
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4067 {
// Port of the mode switch inside pit_get_count (x86_machine.c).
// counter is computed from pc->mode, pc->count and d.

#[no_mangle]
pub extern "C" fn pit_get_count__part4067(mode: u8, count: u32, d: u64) -> u32 {
    // pc->count is promoted to uint64_t in the original C expressions.
    let c = count as u64;
    match mode {
        0 | 1 | 4 | 5 => (c.wrapping_sub(d) & 0xffff) as u32,
        _ => match d.checked_rem(c) {
            // (count - (d % count)), computed in uint64_t then converted.
            Some(r) => c.wrapping_sub(r) as u32,
            // Division by zero is undefined in the original C; avoid a panic.
            None => 0,
        },
    }
}
}

// ── `return counter;` · x86_machine.c:781-781 · chunk 4068 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return counter;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4068 {
// Ported piece of pit_get_count: the final `return counter;`.
#[no_mangle]
pub extern "C" fn pit_get_count__part4068(counter: core::ffi::c_uint) -> core::ffi::c_uint {
    counter
}
}

// ── `int out; int64_t d; d = pit_get_time(pc) - pc->count_load_time;` · x86_machine.c:787-790 · chunk 4070 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int out;
//         int64_t d;
//     
//         d = pit_get_time(pc) - pc->count_load_time;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4070 {
// Port of the prefix of pit_get_out(): computes d = pit_get_time(pc) - pc->count_load_time.
// pit_get_time is a static C function, so its logic is inlined here as a private fn.

use core::ffi::c_void;

#[repr(C)]
pub struct PITChannel {
    pit_state: *mut PITState,
    count: u32,
    latched_count: u32,
    rw_state: u8,
    mode: u8,
    bcd: u8,
    gate: u8,
    count_load_time: i64,
    last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: Option<unsafe extern "C" fn(*mut c_void) -> i64>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

// static int64_t pit_get_time(PITChannel *pc)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pit_get_out__part4070(pc: *mut PITChannel) -> i64 {
    let t = pit_get_time(pc);
    t.wrapping_sub((*pc).count_load_time)
}
}

// ── `switch (pc->mode)` · x86_machine.c:791-813 · chunk 4071 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(pc->mode) {
//         default:
//         case 0:
//             out = (d >= pc->count) >> 0;
//             break;
//         case 1:
//             out = (d < pc->count) >> 0;
//             break;
//         case 2:
//     
//             if ((d % pc->count) == 0 && d != 0)
//                 out = 1;
//             else
//                 out = 0;
//             break;
//         case 3:
//             out = ((d % pc->count) < (pc->count >> 1)) >> 0;
//             break;
//         case 4:
//         case 5:
//             out = (d == pc->count) >> 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4071 {
// Port of the mode-decoding switch of pit_get_out.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut c_void,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

// C's `d % pc->count` would be undefined for count == 0; never panic across FFI.
#[inline]
fn smod(d: i64, count: u32) -> i64 {
    match d.checked_rem(count as i64) {
        Some(r) => r,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn pit_get_out__part4071(pc: *mut PITChannel, d: i64) -> c_int {
    if pc.is_null() {
        return 0;
    }
    let pc = unsafe { &*pc };
    let count = pc.count;
    let out: c_int = match pc.mode {
        1 => (d < count as i64) as c_int,
        2 => ((smod(d, count) == 0 && d != 0) as c_int),
        3 => (smod(d, count) < ((count >> 1) as i64)) as c_int,
        4 | 5 => (d == count as i64) as c_int,
        // default: and case 0
        _ => (d >= count as i64) as c_int,
    };
    out
}
}

// ── `return out;` · x86_machine.c:814-814 · chunk 4072 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return out;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4072 {
use core::ffi::c_int;

// Ported piece of pit_get_out(): the function's `return out;` statement.
#[no_mangle]
pub extern "C" fn pit_get_out__part4072(out: c_int) -> c_int {
    out
}
}

// ── `if (val == 0)` · x86_machine.c:819-820 · chunk 4074 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val == 0)
//             val = 0x10000;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4074 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pit_load_count__part4074(val: c_int) -> c_int {
    if val == 0 {
        0x10000
    } else {
        val
    }
}
}

// ── `s->count_load_time = pit_get_time(s); s->last_irq_time = 0; s->count = val;` · x86_machine.c:821-823 · chunk 4075 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->count_load_time = pit_get_time(s);
//         s->last_irq_time = 0;
//         s->count = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4075 {
// Port of pit_load_count lines 821-823: load timer count and reset irq time.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut PITState,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: Option<unsafe extern "C" fn(*mut c_void) -> i64>,
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

// Private port of the static C helper pit_get_time().
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    if s.is_null() {
        return 0;
    }
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pit_load_count__part4075(s: *mut PITChannel, val: c_int) {
    if s.is_null() {
        return;
    }
    (*s).count_load_time = pit_get_time(s);
    (*s).last_irq_time = 0;
    (*s).count = val as u32;
}
}

// ── `PITState *pit = opaque; int channel, access, addr; PITChannel *s; addr = offset & 3;` · x86_machine.c:829-833 · chunk 4077 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PITState *pit = opaque;
//         int channel, access, addr;
//         PITChannel *s;
//     
//         addr = offset & 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4077 {
// Part of pit_write: computes the register address from the port offset.
// The remaining C code keeps the declarations of pit, channel, access, s.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn pit_write__part4077(opaque: *mut c_void, offset: u32) -> c_int {
    // `pit` is assigned from opaque in C; nothing in this piece reads it.
    let _ = opaque;
    (offset & 3) as c_int
}
}

// ── `PITState *pit = opaque; PITChannel *s; int ret, count, addr; addr = offset & 3;` · x86_machine.c:878-882 · chunk 4080 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *pit = opaque;
//         PITChannel *s;
//         int ret, count, addr;
//     
//         addr = offset & 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4080 {
// Ported piece of pit_read: computes the channel address from the port offset.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn pit_read__part4080(_opaque: *mut c_void, offset: u32) -> c_int {
    (offset & 3) as c_int
}
}

// ── `if (addr == 3)` · x86_machine.c:883-884 · chunk 4081 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 3)
//             return 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4081 {
// Ported piece of pit_read: the "addr == 3" early-return guard.
// Returns nonzero when the PIT read targets the control register (addr 3),
// in which case the caller returns 0xff.

#[no_mangle]
pub extern "C" fn pit_read__part4081(addr: core::ffi::c_int) -> core::ffi::c_int {
    if addr == 3 {
        1
    } else {
        0
    }
}
}

// ── `s = &pit->pit_channels[addr];` · x86_machine.c:886-886 · chunk 4082 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s = &pit->pit_channels[addr];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4082 {
// Computes `&pit->pit_channels[addr]` from the PITState pointer.
use core::ffi::{c_int, c_void};

pub type PITGetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut PITState,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: *mut PITGetTicksFunc,
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pit_read__part4082(pit: *mut c_void, addr: c_int) -> *mut PITChannel {
    if pit.is_null() {
        return core::ptr::null_mut();
    }
    let base: *mut PITChannel = (*(pit as *mut PITState)).pit_channels.as_mut_ptr();
    base.offset(addr as isize)
}
}

// ── `return ret;` · x86_machine.c:913-913 · chunk 4084 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4084 {
// Ported piece of pit_read: the final `return ret;`
// Converts the `int` accumulator to the function's uint32_t return type.

#[no_mangle]
pub extern "C" fn pit_read__part4084(ret: core::ffi::c_int) -> core::ffi::c_uint {
    ret as core::ffi::c_uint
}
}

// ── `PITState *pit = opaque; pit->speaker_data_on = (val >> 1) & 1; pit->pit_channels[2].gate = val…` · x86_machine.c:919-921 · chunk 4086 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *pit = opaque;
//         pit->speaker_data_on = (val >> 1) & 1;
//         pit->pit_channels[2].gate = val & 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4086 {
// Rewrite of the speaker_write body (portion __part4086) of x86_machine.c
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut PITState,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: *mut c_void,
    pub irq: *mut IRQSignal,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn speaker_write__part4086(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    let _ = offset;
    let _ = size_log2;
    let pit = opaque as *mut PITState;
    (*pit).speaker_data_on = ((val >> 1) & 1) as u8;
    (*pit).pit_channels[2].gate = (val & 1) as u8;
}
}

// ── `PITState *pit = opaque; PITChannel *s; int out, val; s = &pit->pit_channels[2]; out = pit_get_o…` · x86_machine.c:926-936 · chunk 4088 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *pit = opaque;
//         PITChannel *s;
//         int out, val;
//     
//         s = &pit->pit_channels[2];
//         out = pit_get_out(s);
//         val = (pit->speaker_data_on << 1) | s->gate | (out << 5);
//     
//     
//     
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4088 {
// Port of speaker_read body (plus the static helpers pit_get_out / pit_get_time
// which have no linker symbols, so their logic is mirrored here).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PITChannel {
    pub pit_state: *mut PITState,
    pub count: u32,
    pub latched_count: u32,
    pub rw_state: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
    pub last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: Option<unsafe extern "C" fn(*mut c_void) -> i64>,
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

// C `%` on signed values truncates toward zero, same as Rust; guard against a
// zero divisor (undefined in C) so nothing can panic across the FFI boundary.
fn c_rem(a: i64, b: i64) -> i64 {
    match a.checked_rem(b) {
        Some(r) => r,
        None => 0,
    }
}

// static int64_t pit_get_time(PITChannel *pc)
fn pit_get_time(pc: *mut PITChannel) -> i64 {
    unsafe {
        let s = (*pc).pit_state;
        if s.is_null() {
            return 0;
        }
        match (*s).get_ticks {
            Some(f) => f((*s).opaque),
            None => 0,
        }
    }
}

// static int pit_get_out(PITChannel *pc)
fn pit_get_out(pc: *mut PITChannel) -> c_int {
    unsafe {
        let d: i64 = pit_get_time(pc).wrapping_sub((*pc).count_load_time);
        let count: i64 = (*pc).count as i64;
        let mode: u8 = (*pc).mode;
        let out: c_int = match mode {
            1 => {
                if d < count {
                    1
                } else {
                    0
                }
            }
            2 => {
                if c_rem(d, count) == 0 && d != 0 {
                    1
                } else {
                    0
                }
            }
            3 => {
                if c_rem(d, count) < (count >> 1) {
                    1
                } else {
                    0
                }
            }
            4 | 5 => {
                if d == count {
                    1
                } else {
                    0
                }
            }
            // default: case 0
            _ => {
                if d >= count {
                    1
                } else {
                    0
                }
            }
        };
        out
    }
}

#[no_mangle]
pub extern "C" fn speaker_read__part4088(
    opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    if opaque.is_null() {
        return 0;
    }
    unsafe {
        let pit = opaque as *mut PITState;
        let s: *mut PITChannel = &mut (*pit).pit_channels[2] as *mut PITChannel;
        let out: c_int = pit_get_out(s);
        let val: c_int =
            (((*pit).speaker_data_on as c_int) << 1) | ((*s).gate as c_int) | (out << 5);
        val as u32
    }
}
}

// ── `PITChannel *s; int64_t d, delay; s = &pit->pit_channels[0]; delay = 1193182; d = pit_get_time(s…` · x86_machine.c:943-950 · chunk 4090 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITChannel *s;
//         int64_t d, delay;
//     
//         s = &pit->pit_channels[0];
//     
//         delay = 1193182;
//     
//         d = pit_get_time(s) - s->count_load_time;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4090 {
// Ported piece of pit_update_irq: local setup (s, delay, d) for channel 0.
use core::ffi::{c_int, c_void};

type PITGetTicksFunc = unsafe extern "C" fn(opaque: *mut c_void) -> i64;

#[repr(C)]
pub struct PITChannel {
    pit_state: *mut PITState,
    count: u32,
    latched_count: u32,
    rw_state: u8,
    mode: u8,
    bcd: u8,
    gate: u8,
    count_load_time: i64,
    last_irq_time: i64,
}

#[repr(C)]
pub struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: Option<PITGetTicksFunc>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

// static int64_t pit_get_time(PITChannel *pc)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pit_update_irq__part4090(
    pit: *mut PITState,
    s_out: *mut *mut PITChannel,
    d_out: *mut i64,
    delay_out: *mut i64,
) {
    let s: *mut PITChannel = core::ptr::addr_of_mut!((*pit).pit_channels)
        .cast::<PITChannel>();

    *s_out = s;

    *delay_out = 1193182;

    let d: i64 = pit_get_time(s).wrapping_sub((*s).count_load_time);
    *d_out = d;

    let _ = core::mem::size_of::<c_int>();
}
}

// ── `if (delay <= 0) … else` · x86_machine.c:977-980 · chunk 4092 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (delay <= 0)
//             return 0;
//         else
//             return delay / (1193182 / 1000);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4092 {
// Tail of pit_update_irq: convert the remaining delay (in PIT ticks) to
// milliseconds, returning 0 when no delay remains.
// int64_t is `long int` on this target, so c_long matches the C ABI.
use core::ffi::{c_int, c_long};

#[no_mangle]
pub extern "C" fn pit_update_irq__part4092(delay: c_long) -> c_int {
    if delay <= 0 {
        0
    } else {
        // C integer division truncates toward zero; 1193182 / 1000 == 1193.
        (delay / (1193182 as c_long / 1000 as c_long)) as c_int
    }
}
}

// ── `if (s->lsr & 0x01) && (s->ier & 0x01) … else` · x86_machine.c:1066-1072 · chunk 4098 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((s->lsr & 0x01) && (s->ier & 0x01)) {
//             s->iir = 0x04;
//         } else if ((s->lsr & 0x20) && (s->ier & 0x02)) {
//             s->iir = 0x02;
//         } else {
//             s->iir = 0x01;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4098 {
use core::ffi::c_void;

#[repr(C)]
pub struct SerialState {
    pub divider: u8,
    pub rbr: u8,
    pub ier: u8,
    pub iir: u8,
    pub lcr: u8,
    pub mcr: u8,
    pub lsr: u8,
    pub msr: u8,
    pub scr: u8,
    pub fcr: u8,
    pub irq: *mut c_void,
    pub write_func: Option<extern "C" fn(opaque: *mut c_void, buf: *const u8, buf_len: i32)>,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn serial_update_irq__part4098(s: *mut SerialState) {
    unsafe {
        let s = &mut *s;
        if (s.lsr & 0x01) != 0 && (s.ier & 0x01) != 0 {
            s.iir = 0x04;
        } else if (s.lsr & 0x20) != 0 && (s.ier & 0x02) != 0 {
            s.iir = 0x02;
        } else {
            s.iir = 0x01;
        }
    }
}
}

// ── `if (s->iir != 0x01) … else` · x86_machine.c:1073-1077 · chunk 4099 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->iir != 0x01) {
//             set_irq(s->irq, 1);
//         } else {
//             set_irq(s->irq, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4099 {
// Port of serial_update_irq's IRQ update tail.
use core::ffi::{c_int, c_void};

type SetIrqFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IrqSignal {
    set_irq: Option<SetIrqFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct SerialState {
    divider: u8,
    rbr: u8,
    ier: u8,
    iir: u8,
    lcr: u8,
    mcr: u8,
    lsr: u8,
    msr: u8,
    scr: u8,
    fcr: u8,
    irq: *mut IrqSignal,
    write_func: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn serial_update_irq__part4099(s: *mut SerialState) {
    let iir = (*s).iir;
    let level: c_int = if iir != 0x01 { 1 } else { 0 };
    let irq = (*s).irq;
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}
}

// ── `SerialState *s = opaque; int addr; addr = offset & 7;` · x86_machine.c:1098-1101 · chunk 4101 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     SerialState *s = opaque;
//         int addr;
//     
//         addr = offset & 7;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4101 {
// Port of the beginning of the C function `serial_write`:
//   SerialState *s = opaque;   /* kept in C */
//   int addr; addr = offset & 7;
// The Rust function computes the register address (offset & 7).

#[no_mangle]
pub extern "C" fn serial_write__part4101(offset: u32) -> core::ffi::c_int {
    (offset & 7) as core::ffi::c_int
}
}

// ── `SerialState *s = opaque; int ret, addr; addr = offset & 7;` · x86_machine.c:1173-1176 · chunk 4104 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     SerialState *s = opaque;
//         int ret, addr;
//     
//         addr = offset & 7;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4104 {
// Computes the serial port register address from the byte offset.
#[no_mangle]
pub extern "C" fn serial_read__part4104(offset: u32) -> core::ffi::c_int {
    (offset & 7) as core::ffi::c_int
}
}

// ── `return ret;` · x86_machine.c:1220-1220 · chunk 4106 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4106 {
// Extracted tail of serial_read(): converts the collected register value
// (an `int`) to the function's `uint32_t` return type.
#[no_mangle]
pub extern "C" fn serial_read__part4106(ret: core::ffi::c_int) -> u32 {
    ret as u32
}
}

// ── `s->rbr = 0; s->lsr |= 0x10 | 0x01; serial_update_irq(s);` · x86_machine.c:1225-1227 · chunk 4108 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->rbr = 0;
//         s->lsr |= 0x10 | 0x01;
//         serial_update_irq(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4108 {
// Rust rewrite of serial_send_break's body (x86_machine.c lines 1225-1227).

use core::ffi::{c_int, c_void};

type SetIRQFunc =
    unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

type SerialWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, buf: *const u8, buf_len: c_int);

#[repr(C)]
pub struct SerialState {
    pub divider: u8,
    pub rbr: u8,
    pub ier: u8,
    pub iir: u8,
    pub lcr: u8,
    pub mcr: u8,
    pub lsr: u8,
    pub msr: u8,
    pub scr: u8,
    pub fcr: u8,
    pub irq: *mut IRQSignal,
    pub write_func: Option<SerialWriteFunc>,
    pub opaque: *mut c_void,
}

// Port of the static inline set_irq().
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// Port of the static function serial_update_irq().
unsafe fn serial_update_irq(s: *mut SerialState) {
    if ((*s).lsr & 0x01) != 0 && ((*s).ier & 0x01) != 0 {
        (*s).iir = 0x04;
    } else if ((*s).lsr & 0x20) != 0 && ((*s).ier & 0x02) != 0 {
        (*s).iir = 0x02;
    } else {
        (*s).iir = 0x01;
    }
    if (*s).iir != 0x01 {
        set_irq((*s).irq, 1);
    } else {
        set_irq((*s).irq, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_send_break__part4108(s: *mut SerialState) {
    (*s).rbr = 0;
    (*s).lsr |= 0x10 | 0x01;
    serial_update_irq(s);
}
}

// ── `return 0xff;` · x86_machine.c:1324-1324 · chunk 4113 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4113 {
// Rust port of the TARGET: `return 0xff;` from port80_read (x86_machine.c line 1324).
// The port-0x80 read handler reports 0x80 as the "no device / bus idle" value.

#[no_mangle]
pub extern "C" fn port80_read__part4113() -> u32 {
    0xff
}
}

// ── `int a20 = 1; return a20 << 1;` · x86_machine.c:1334-1335 · chunk 4116 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int a20 = 1;
//         return a20 << 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4116 {
#[no_mangle]
pub extern "C" fn port92_read__part4116() -> u32 {
    // int a20 = 1; return a20 << 1;
    let a20: i32 = 1;
    (a20 << 1) as u32
}
}

// ── `return regs[0];` · x86_machine.c:1398-1398 · chunk 4119 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return regs[0];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4119 {
// Extracted from vmport_read: returns the first element of the regs array.
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn vmport_read__part4119(regs: *const c_uint) -> c_uint {
    unsafe { *regs }
}
}

