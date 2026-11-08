//! From `x86_machine.c`.

// ── `to_bcd` · x86_machine.c:104-111 · chunk 3978 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int to_bcd(CMOSState *s, unsigned int a)
//     {
//         if (s->cmos_data[11] & 0x04) {
//             return a;
//         } else {
//             return ((a / 10) << 4) | (a % 10);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3978 {
use core::ffi::{c_int, c_uchar, c_uint};

// Mirror of the C enum-free typedef `int BOOL;`
#[allow(non_camel_case_types)]
type BOOL = c_int;

#[repr(C)]
struct IRQSignal {
    // Only the address is ever used here; the layout is irrelevant because
    // this struct is only reached through a pointer field.
    _unused: [c_uchar; 0],
}

#[repr(C)]
struct CMOSState {
    cmos_index: c_uchar,
    cmos_data: [c_uchar; 128],
    irq: *mut IRQSignal,
    use_local_time: BOOL,
    irq_timeout: u32,
    irq_period: u32,
}

#[no_mangle]
pub extern "C" fn to_bcd(s: *mut CMOSState, a: c_uint) -> c_int {
    unsafe {
        if (*s).cmos_data[11] & 0x04 != 0 {
            a as c_int
        } else {
            let v: c_uint = ((a / 10) << 4) | (a % 10);
            v as c_int
        }
    }
}
}

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

// ── `cmos_update_time` · x86_machine.c:113-155 · chunk 3980 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void cmos_update_time(CMOSState *s, BOOL set_century)
//     {
//         struct timeval tv;
//         struct tm tm;
//         time_t ti;
//         int val;
//     
//         gettimeofday(&tv, 
//                          ((void *)0)
//                              );
//         ti = tv.tv_sec;
//         if (s->use_local_time) {
//             localtime_r(&ti, &tm);
//         } else {
//             gmtime_r(&ti, &tm);
//         }
//     
//         s->cmos_data[0] = to_bcd(s, tm.tm_sec);
//         s->cmos_data[2] = to_bcd(s, tm.tm_min);
//         if (s->cmos_data[11] & 0x02) {
//             s->cmos_data[4] = to_bcd(s, tm.tm_hour);
//         } else {
//             s->cmos_data[4] = to_bcd(s, tm.tm_hour % 12);
//             if (tm.tm_hour >= 12)
//                 s->cmos_data[4] |= 0x80;
//         }
//         s->cmos_data[6] = to_bcd(s, tm.tm_wday);
//         s->cmos_data[7] = to_bcd(s, tm.tm_mday);
//         s->cmos_data[8] = to_bcd(s, tm.tm_mon + 1);
//         s->cmos_data[9] = to_bcd(s, tm.tm_year % 100);
//     
//         if (set_century) {
//     
//             val = to_bcd(s, (tm.tm_year / 100) + 19);
//             s->cmos_data[0x32] = val;
//             s->cmos_data[0x37] = val;
//         }
//     
//     
//         if (tv.tv_usec < 244) {
//             s->cmos_data[10] |= 0x80;
//         } else {
//             s->cmos_data[10] &= ~0x80;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3980 {
// Rust rewrite of cmos_update_time (x86_machine.c)
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct CmosState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut c_void,
    use_local_time: c_int,
    irq_timeout: u32,
    irq_period: u32,
}

#[repr(C)]
struct Timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut c_void) -> c_int;
    fn localtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn gmtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
}

// port of the static C helper to_bcd()
fn to_bcd(s: &CmosState, a: c_uint) -> c_int {
    if (s.cmos_data[11] & 0x04) != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

#[no_mangle]
pub extern "C" fn cmos_update_time(s: *mut CmosState, set_century: c_int) {
    if s.is_null() {
        return;
    }
    let s: &mut CmosState = unsafe { &mut *s };

    let mut tv = Timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tm = Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: core::ptr::null(),
    };

    unsafe {
        gettimeofday(&mut tv as *mut Timeval, core::ptr::null_mut());
    }
    let ti: c_long = tv.tv_sec;
    unsafe {
        if s.use_local_time != 0 {
            localtime_r(&ti as *const c_long, &mut tm as *mut Tm);
        } else {
            gmtime_r(&ti as *const c_long, &mut tm as *mut Tm);
        }
    }

    s.cmos_data[0] = to_bcd(s, tm.tm_sec as c_uint) as u8;
    s.cmos_data[2] = to_bcd(s, tm.tm_min as c_uint) as u8;
    if (s.cmos_data[11] & 0x02) != 0 {
        s.cmos_data[4] = to_bcd(s, tm.tm_hour as c_uint) as u8;
    } else {
        s.cmos_data[4] = to_bcd(s, (tm.tm_hour % 12) as c_uint) as u8;
        if tm.tm_hour >= 12 {
            s.cmos_data[4] |= 0x80;
        }
    }
    s.cmos_data[6] = to_bcd(s, tm.tm_wday as c_uint) as u8;
    s.cmos_data[7] = to_bcd(s, tm.tm_mday as c_uint) as u8;
    s.cmos_data[8] = to_bcd(s, (tm.tm_mon + 1) as c_uint) as u8;
    s.cmos_data[9] = to_bcd(s, (tm.tm_year % 100) as c_uint) as u8;

    if set_century != 0 {
        let val: c_int = to_bcd(s, ((tm.tm_year / 100) + 19) as c_uint);
        s.cmos_data[0x32] = val as u8;
        s.cmos_data[0x37] = val as u8;
    }

    // update in progress flag: 8/32768 seconds after change
    if tv.tv_usec < 244 {
        s.cmos_data[10] |= 0x80;
    } else {
        s.cmos_data[10] &= !0x80u8;
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

// ── `cmos_init` · x86_machine.c:157-179 · chunk 3988 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     CMOSState *cmos_init(PhysMemoryMap *port_map, int addr,
//                          IRQSignal *irq, BOOL use_local_time)
//     {
//         CMOSState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->use_local_time = use_local_time;
//     
//         s->cmos_index = 0;
//     
//         s->cmos_data[10] = 0x26;
//         s->cmos_data[11] = 0x02;
//         s->cmos_data[12] = 0x00;
//         s->cmos_data[13] = 0x80;
//     
//         cmos_update_time(s, TRUE);
//     
//         s->irq = irq;
//     
//         cpu_register_device(port_map, addr, 2, s, cmos_read, cmos_write,
//                             (1 << 0));
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3988 {
// Port of cmos_init (plus the static helpers it needs) from x86_machine.c.
// cmos_read / cmos_write / cmos_update_timer / cmos_get_timer / cmos_update_time
// / to_bcd are `static` in the original translation unit, so they have no
// linker symbol; their logic is ported here as private Rust functions.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

// ---- C types -------------------------------------------------------------

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
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
pub struct Timeval {
    pub tv_sec: c_long,  // time_t
    pub tv_usec: c_long, // suseconds_t
}

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

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

type DeviceReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
type DeviceWriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn gettimeofday(tv: *mut Timeval, tz: *mut c_void) -> c_int;
    fn gmtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn localtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: DeviceReadFunc,
        write_func: DeviceWriteFunc,
        devio_flags: c_int,
    ) -> *mut c_void;
}

const CMOS_FREQ: u32 = 32768;
const CLOCK_MONOTONIC: c_int = 1;

// ---- small helpers -------------------------------------------------------

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    // IRQSignal::set_irq C function pointer
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[inline]
unsafe fn cmos_data_at(s: *mut CMOSState, idx: u8) -> *mut u8 {
    (*s).cmos_data.as_mut_ptr().add(idx as usize)
}

unsafe fn to_bcd(s: *mut CMOSState, a: c_uint) -> c_int {
    if ((*s).cmos_data[11] & 0x04) != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

unsafe fn cmos_update_time(s: *mut CMOSState, set_century: c_int) {
    let mut tv: Timeval = core::mem::zeroed();
    let mut tm: Tm = core::mem::zeroed();
    let mut ti: c_long;

    gettimeofday(&mut tv, core::ptr::null_mut());
    ti = tv.tv_sec;
    if (*s).use_local_time != 0 {
        localtime_r(&ti, &mut tm);
    } else {
        gmtime_r(&ti, &mut tm);
    }

    (*s).cmos_data[0] = to_bcd(s, tm.tm_sec as c_uint) as u8;
    (*s).cmos_data[2] = to_bcd(s, tm.tm_min as c_uint) as u8;
    if ((*s).cmos_data[11] & 0x02) != 0 {
        (*s).cmos_data[4] = to_bcd(s, tm.tm_hour as c_uint) as u8;
    } else {
        (*s).cmos_data[4] = to_bcd(s, (tm.tm_hour % 12) as c_uint) as u8;
        if tm.tm_hour >= 12 {
            (*s).cmos_data[4] |= 0x80;
        }
    }
    (*s).cmos_data[6] = to_bcd(s, tm.tm_wday as c_uint) as u8;
    (*s).cmos_data[7] = to_bcd(s, tm.tm_mday as c_uint) as u8;
    (*s).cmos_data[8] = to_bcd(s, (tm.tm_mon + 1) as c_uint) as u8;
    (*s).cmos_data[9] = to_bcd(s, (tm.tm_year % 100) as c_uint) as u8;

    if set_century != 0 {
        // not set by the hardware, but easier to do it here
        let val = to_bcd(s, ((tm.tm_year / 100) + 19) as c_uint);
        (*s).cmos_data[0x32] = val as u8;
        (*s).cmos_data[0x37] = val as u8;
    }

    // update in progress flag: 8/32768 seconds after change
    if tv.tv_usec < 244 {
        (*s).cmos_data[10] |= 0x80;
    } else {
        (*s).cmos_data[10] &= !0x80u8;
    }
}

unsafe fn cmos_get_timer(s: *mut CMOSState) -> u32 {
    let mut ts: Timespec = core::mem::zeroed();
    let _ = s;

    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as u32).wrapping_mul(CMOS_FREQ)
        .wrapping_add(((ts.tv_nsec as u64).wrapping_mul(CMOS_FREQ as u64) / 1000000000) as u32)
}

unsafe fn cmos_update_timer(s: *mut CMOSState) {
    let mut period_code: c_int;

    period_code = ((*s).cmos_data[10] & 0x0f) as c_int;
    if ((*s).cmos_data[11] & 0x40) != 0 && period_code != 0 {
        if period_code <= 2 {
            period_code += 7;
        }
        (*s).irq_period = (1u32) << ((period_code - 1) as u32);
        (*s).irq_timeout =
            cmos_get_timer(s).wrapping_add((*s).irq_period) & !((*s).irq_period).wrapping_sub(1);
    }
}

unsafe extern "C" fn cmos_write(
    opaque: *mut c_void,
    offset: u32,
    data: u32,
    size_log2: c_int,
) {
    let s = opaque as *mut CMOSState;
    let _ = size_log2;

    if offset == 0 {
        (*s).cmos_index = (data & 0x7f) as u8;
    } else {
        match (*s).cmos_index {
            10 => {
                let low = (data as u8) & 0x7f;
                let high = (*s).cmos_data[10] & 0x80;
                (*s).cmos_data[10] = low | high;
                cmos_update_timer(s);
            }
            11 => {
                let idx = (*s).cmos_index;
                *cmos_data_at(s, idx) = data as u8;
                cmos_update_timer(s);
            }
            idx => {
                *cmos_data_at(s, idx) = data as u8;
            }
        }
    }
}

unsafe extern "C" fn cmos_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut CMOSState;
    let _ = size_log2;

    if offset == 0 {
        0xff
    } else {
        let idx = (*s).cmos_index;
        match idx {
            0 | 2 | 4 | 6 | 7 | 8 | 9 | 10 => {
                cmos_update_time(s, 0);
                *cmos_data_at(s, idx) as u32
            }
            12 => {
                let ret = *cmos_data_at(s, idx) as u32;
                (*s).cmos_data[12] = 0x00;
                set_irq((*s).irq, 0);
                ret
            }
            _ => *cmos_data_at(s, idx) as u32,
        }
    }
}

// ---- the target ---------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn cmos_init(
    port_map: *mut PhysMemoryMap,
    addr: c_int,
    irq: *mut IRQSignal,
    use_local_time: c_int,
) -> *mut CMOSState {
    let s: *mut CMOSState = mallocz(core::mem::size_of::<CMOSState>()) as *mut CMOSState;

    (*s).use_local_time = use_local_time;

    (*s).cmos_index = 0;

    (*s).cmos_data[10] = 0x26;
    (*s).cmos_data[11] = 0x02;
    (*s).cmos_data[12] = 0x00;
    (*s).cmos_data[13] = 0x80;

    cmos_update_time(s, 1);

    (*s).irq = irq;

    cpu_register_device(
        port_map,
        addr as u64,
        2,
        s as *mut c_void,
        Some(cmos_read),
        Some(cmos_write),
        1 << 0, // DEVIO_SIZE8
    );
    s
}
}

// ── `CMOSState *s; s = mallocz(sizeof(*s)); s->use_local_time = use_local_time; s->cmos_index = 0; s…` · x86_machine.c:160-178 · chunk 3989 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     CMOSState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->use_local_time = use_local_time;
//     
//         s->cmos_index = 0;
//     
//         s->cmos_data[10] = 0x26;
//         s->cmos_data[11] = 0x02;
//         s->cmos_data[12] = 0x00;
//         s->cmos_data[13] = 0x80;
//     
//         cmos_update_time(s, TRUE);
//     
//         s->irq = irq;
//     
//         cpu_register_device(port_map, addr, 2, s, cmos_read, cmos_write,
//                             (1 << 0));
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3989 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    _private: [u8; 0],
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

type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type CmosUpdateTimeFunc = unsafe extern "C" fn(*mut CMOSState, c_int);

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn cmos_init__part3989(
    port_map: *mut PhysMemoryMap,
    addr: c_int,
    irq: *mut IRQSignal,
    use_local_time: c_int,
    update_time: Option<CmosUpdateTimeFunc>,
    read_func: Option<DeviceReadFunc>,
    write_func: Option<DeviceWriteFunc>,
) -> *mut CMOSState {
    let s = mallocz(core::mem::size_of::<CMOSState>()) as *mut CMOSState;

    (*s).use_local_time = use_local_time;

    (*s).cmos_index = 0;

    (*s).cmos_data[10] = 0x26;
    (*s).cmos_data[11] = 0x02;
    (*s).cmos_data[12] = 0x00;
    (*s).cmos_data[13] = 0x80;

    if let Some(f) = update_time {
        f(s, 1);
    }

    (*s).irq = irq;

    cpu_register_device(
        port_map,
        addr as u64,
        2,
        s as *mut c_void,
        read_func,
        write_func,
        1 << 0,
    );

    s
}
}

// ── `cmos_get_timer` · x86_machine.c:183-190 · chunk 3990 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t cmos_get_timer(CMOSState *s)
//     {
//         struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint32_t)ts.tv_sec * 32768 +
//             ((uint64_t)ts.tv_nsec * 32768 / 1000000000);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3990 {
// Rust rewrite of cmos_get_timer (x86_machine.c)
use core::ffi::{c_int, c_long, c_void};

/// struct timespec { __time_t tv_sec; __syscall_slong_t tv_nsec; }
/// On this target both __time_t and __syscall_slong_t are `long int`.
#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

const CLOCK_MONOTONIC: c_int = 1;
const CMOS_FREQ: u32 = 32768;

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn cmos_get_timer(_s: *mut c_void) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }

    let sec_part = (ts.tv_sec as u32).wrapping_mul(CMOS_FREQ);
    let nsec_part = ((ts.tv_nsec as u64).wrapping_mul(CMOS_FREQ as u64)) / 1_000_000_000u64;

    (sec_part as u64).wrapping_add(nsec_part) as u32
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

// ── `cmos_update_timer` · x86_machine.c:192-205 · chunk 3992 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void cmos_update_timer(CMOSState *s)
//     {
//         int period_code;
//     
//         period_code = s->cmos_data[10] & 0x0f;
//         if ((s->cmos_data[11] & 0x40) &&
//             period_code != 0) {
//             if (period_code <= 2)
//                 period_code += 7;
//             s->irq_period = 1 << (period_code - 1);
//             s->irq_timeout = (cmos_get_timer(s) + s->irq_period) &
//                 ~(s->irq_period - 1);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3992 {
// Rust rewrite of cmos_update_timer (x86_machine.c)
#![allow(non_snake_case)]

use core::ffi::{c_int, c_long};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
    pub irq_num: c_int,
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

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;
const CMOS_FREQ: u64 = 32768;

// Ported from the static C function cmos_get_timer().
unsafe fn cmos_get_timer(_s: *mut CMOSState) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);
    let secs = ts.tv_sec as u32;
    secs.wrapping_mul(CMOS_FREQ as u32)
        .wrapping_add(((ts.tv_nsec as u64).wrapping_mul(CMOS_FREQ) / 1_000_000_000u64) as u32)
}

#[no_mangle]
pub unsafe extern "C" fn cmos_update_timer(s: *mut CMOSState) {
    let mut period_code: c_int;

    period_code = (core::ptr::addr_of!((*s).cmos_data[10]).read() & 0x0f) as c_int;
    if ((*s).cmos_data[11] & 0x40) != 0 && period_code != 0 {
        if period_code <= 2 {
            period_code += 7;
        }
        (*s).irq_period = 1u32 << (period_code - 1);
        (*s).irq_timeout = cmos_get_timer(s).wrapping_add((*s).irq_period)
            & !((*s).irq_period).wrapping_sub(1);
    }
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

// ── `cmos_update_irq` · x86_machine.c:209-223 · chunk 3995 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void cmos_update_irq(CMOSState *s)
//     {
//         uint32_t d;
//         if (s->cmos_data[11] & 0x40) {
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
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3995 {
use core::ffi::{c_int, c_long, c_void};

/// Mirrors C `SetIRQFunc`: void (*)(void *opaque, int irq_num, int level)
type SetIrqFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IrqSignal {
    pub set_irq: Option<SetIrqFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

/// typedef int BOOL
type Bool = c_int;

#[repr(C)]
pub struct CmosState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut IrqSignal,
    pub use_local_time: Bool,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;
const CMOS_FREQ: u32 = 32768;

// Port of the static C helper `cmos_get_timer`.
unsafe fn cmos_get_timer(_s: *mut CmosState) -> u32 {
    let mut ts: Timespec = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    let sec_part = (ts.tv_sec as u32).wrapping_mul(CMOS_FREQ);
    let nsec_part = (((ts.tv_nsec as u64).wrapping_mul(CMOS_FREQ as u64)) / 1_000_000_000u64) as u32;
    sec_part.wrapping_add(nsec_part)
}

#[no_mangle]
pub unsafe extern "C" fn cmos_update_irq(s: *mut CmosState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;

    // RTC_REG_B == 11, REG_B_PIE == 0x40, RTC_REG_C == 12
    if (s.cmos_data[11] & 0x40) != 0 {
        let d: u32 = cmos_get_timer(s as *mut CmosState).wrapping_sub(s.irq_timeout);
        if (d as i32) >= 0 {
            s.cmos_data[12] |= 0xc0;
            let irq = s.irq;
            if !irq.is_null() {
                if let Some(f) = (*irq).set_irq {
                    f((*irq).opaque, (*irq).irq_num, 1);
                }
            }
            s.irq_timeout = s.irq_timeout.wrapping_add(s.irq_period);
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

// ── `cmos_write` · x86_machine.c:225-251 · chunk 3997 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void cmos_write(void *opaque, uint32_t offset,
//                            uint32_t data, int size_log2)
//     {
//         CMOSState *s = opaque;
//     
//         if (offset == 0) {
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
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3997 {
// Rewrite of cmos_write (x86_machine.c) plus its file-local helpers
// cmos_get_timer and cmos_update_timer, which have no linker symbols.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct SetIRQFuncOpaque {
    _private: [u8; 0],
}

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct CMOSState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut IRQSignal,
    use_local_time: c_int, // BOOL == int
    irq_timeout: u32,
    irq_period: u32,
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

// Equivalent of the static inline set_irq(irq, level) helper.
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let sig = &*irq;
        if let Some(f) = sig.set_irq {
            f(sig.opaque, sig.irq_num, level);
        }
    }
}

// Port of the static C function cmos_get_timer.
unsafe fn cmos_get_timer(_s: *mut CMOSState) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts);
    let a = (ts.tv_sec as u32).wrapping_mul(32768);
    let b = ((ts.tv_nsec as u64).wrapping_mul(32768) / 1000000000u64) as u32;
    a.wrapping_add(b)
}

// Port of the static C function cmos_update_timer.
unsafe fn cmos_update_timer(s: *mut CMOSState) {
    let st = &mut *s;
    let mut period_code: c_int = (st.cmos_data[10] & 0x0f) as c_int;
    if (st.cmos_data[11] & 0x40) != 0 && period_code != 0 {
        if period_code <= 2 {
            period_code += 7;
        }
        st.irq_period = (1u32).wrapping_shl((period_code - 1) as u32);
        let t = cmos_get_timer(s).wrapping_add(st.irq_period);
        st.irq_timeout = t & !(st.irq_period.wrapping_sub(1));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmos_write(
    opaque: *mut c_void,
    offset: u32,
    data: u32,
    _size_log2: c_int,
) {
    if opaque.is_null() {
        return;
    }
    let s = opaque as *mut CMOSState;
    let st = &mut *s;

    if offset == 0 {
        st.cmos_index = (data & 0x7f) as u8;
    } else {
        match st.cmos_index {
            10 => {
                let keep = st.cmos_data[10] & 0x80;
                st.cmos_data[10] = ((data & !0x80u32) | (keep as u32)) as u8;
                cmos_update_timer(s);
            }
            11 => {
                let idx = st.cmos_index as usize;
                if idx < st.cmos_data.len() {
                    st.cmos_data[idx] = data as u8;
                }
                cmos_update_timer(s);
            }
            _ => {
                let idx = st.cmos_index as usize;
                if idx < st.cmos_data.len() {
                    st.cmos_data[idx] = data as u8;
                }
            }
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

// ── `cmos_read` · x86_machine.c:253-286 · chunk 3999 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t cmos_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         CMOSState *s = opaque;
//         int ret;
//     
//         if (offset == 0) {
//             return 0xff;
//         } else {
//             switch(s->cmos_index) {
//             case 0:
//             case 2:
//             case 4:
//             case 6:
//             case 7:
//             case 8:
//             case 9:
//             case 10:
//                 cmos_update_time(s, FALSE);
//                 ret = s->cmos_data[s->cmos_index];
//                 break;
//             case 12:
//                 ret = s->cmos_data[s->cmos_index];
//                 s->cmos_data[12] = 0x00;
//                 set_irq(s->irq, 0);
//                 break;
//             default:
//                 ret = s->cmos_data[s->cmos_index];
//             }
//     
//     
//     
//             return ret;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3999 {
// Rust rewrite of x86_machine.c: cmos_read (plus the static helpers it needs).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: i64,
    tm_zone: *const c_char,
}

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct CMOSState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut IRQSignal,
    use_local_time: c_int,
    irq_timeout: u32,
    irq_period: u32,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut c_void) -> c_int;
    fn gmtime_r(timer: *const i64, result: *mut Tm) -> *mut Tm;
    fn localtime_r(timer: *const i64, result: *mut Tm) -> *mut Tm;
}

#[inline]
unsafe fn data_get(s: *const CMOSState, idx: u8) -> u8 {
    match (*s).cmos_data.get(idx as usize) {
        Some(v) => *v,
        None => 0,
    }
}

#[inline]
unsafe fn data_set(s: *mut CMOSState, idx: u8, val: u8) {
    if let Some(v) = (*s).cmos_data.get_mut(idx as usize) {
        *v = val;
    }
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// static int to_bcd(CMOSState *s, unsigned int a)
unsafe fn to_bcd(s: *const CMOSState, a: u32) -> c_int {
    if (data_get(s, 11) & 0x04) != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

// static void cmos_update_time(CMOSState *s, BOOL set_century)
unsafe fn cmos_update_time(s: *mut CMOSState, set_century: c_int) {
    let mut tv: Timeval = core::mem::zeroed();
    let mut tm: Tm = core::mem::zeroed();
    let ti: i64;

    gettimeofday(&mut tv, core::ptr::null_mut());
    ti = tv.tv_sec;
    if (*s).use_local_time != 0 {
        localtime_r(&ti, &mut tm);
    } else {
        gmtime_r(&ti, &mut tm);
    }

    data_set(s, 0, to_bcd(s, tm.tm_sec as u32) as u8);
    data_set(s, 2, to_bcd(s, tm.tm_min as u32) as u8);
    if (data_get(s, 11) & 0x02) != 0 {
        data_set(s, 4, to_bcd(s, tm.tm_hour as u32) as u8);
    } else {
        data_set(s, 4, to_bcd(s, (tm.tm_hour % 12) as u32) as u8);
        if tm.tm_hour >= 12 {
            let v = data_get(s, 4) | 0x80;
            data_set(s, 4, v);
        }
    }
    data_set(s, 6, to_bcd(s, tm.tm_wday as u32) as u8);
    data_set(s, 7, to_bcd(s, tm.tm_mday as u32) as u8);
    data_set(s, 8, to_bcd(s, (tm.tm_mon + 1) as u32) as u8);
    data_set(s, 9, to_bcd(s, (tm.tm_year % 100) as u32) as u8);

    if set_century != 0 {
        let val = to_bcd(s, (tm.tm_year / 100 + 19) as u32);
        data_set(s, 0x32, val as u8);
        data_set(s, 0x37, val as u8);
    }

    if tv.tv_usec < 244 {
        let v = data_get(s, 10) | 0x80;
        data_set(s, 10, v);
    } else {
        let v = data_get(s, 10) & !0x80u8;
        data_set(s, 10, v);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmos_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut CMOSState;

    if offset == 0 {
        return 0xff;
    }

    let index = (*s).cmos_index;
    let ret: c_int = match index {
        0 | 2 | 4 | 6 | 7 | 8 | 9 | 10 => {
            cmos_update_time(s, 0);
            data_get(s, index) as c_int
        }
        12 => {
            let r = data_get(s, index) as c_int;
            data_set(s, 12, 0x00);
            set_irq((*s).irq, 0);
            r
        }
        _ => data_get(s, index) as c_int,
    };

    ret as u32
}
}

// ── `if (offset == 0) … else` · x86_machine.c:258-285 · chunk 4000 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset == 0) {
//             return 0xff;
//         } else {
//             switch(s->cmos_index) {
//             case 0:
//             case 2:
//             case 4:
//             case 6:
//             case 7:
//             case 8:
//             case 9:
//             case 10:
//                 cmos_update_time(s, FALSE);
//                 ret = s->cmos_data[s->cmos_index];
//                 break;
//             case 12:
//                 ret = s->cmos_data[s->cmos_index];
//                 s->cmos_data[12] = 0x00;
//                 set_irq(s->irq, 0);
//                 break;
//             default:
//                 ret = s->cmos_data[s->cmos_index];
//             }
//     
//     
//     
//             return ret;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4000 {
// Port of the read part of cmos_read() from x86_machine.c (lines 258-285).

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_long, c_void};

type BOOL = c_int;

#[repr(C)]
struct Timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const u8,
}

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct CMOSState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut IRQSignal,
    use_local_time: BOOL,
    irq_timeout: u32,
    irq_period: u32,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut c_void) -> c_int;
    fn localtime_r(t: *const c_long, tm: *mut Tm) -> *mut Tm;
    fn gmtime_r(t: *const c_long, tm: *mut Tm) -> *mut Tm;
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let f = (*irq).set_irq;
        if let Some(f) = f {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

// static int to_bcd(CMOSState *s, unsigned int a)
unsafe fn to_bcd(s: *mut CMOSState, a: u32) -> c_int {
    let data = core::ptr::addr_of_mut!((*s).cmos_data).cast::<u8>();
    if (*data.add(11) & 0x04) != 0 {
        a as c_int
    } else {
        (((a / 10) << 4) | (a % 10)) as c_int
    }
}

// static void cmos_update_time(CMOSState *s, BOOL set_century)
unsafe fn cmos_update_time(s: *mut CMOSState, set_century: BOOL) {
    let mut tv: Timeval = core::mem::zeroed();
    let mut tm: Tm = core::mem::zeroed();
    let mut ti: c_long = 0;

    gettimeofday(&mut tv, core::ptr::null_mut());
    ti = tv.tv_sec;
    if (*s).use_local_time != 0 {
        localtime_r(&ti, &mut tm);
    } else {
        gmtime_r(&ti, &mut tm);
    }

    let data = core::ptr::addr_of_mut!((*s).cmos_data).cast::<u8>();

    *data.add(0) = to_bcd(s, tm.tm_sec as u32) as u8;
    *data.add(2) = to_bcd(s, tm.tm_min as u32) as u8;
    if (*data.add(11) & 0x02) != 0 {
        *data.add(4) = to_bcd(s, tm.tm_hour as u32) as u8;
    } else {
        *data.add(4) = to_bcd(s, (tm.tm_hour % 12) as u32) as u8;
        if tm.tm_hour >= 12 {
            *data.add(4) |= 0x80;
        }
    }
    *data.add(6) = to_bcd(s, tm.tm_wday as u32) as u8;
    *data.add(7) = to_bcd(s, tm.tm_mday as u32) as u8;
    *data.add(8) = to_bcd(s, (tm.tm_mon + 1) as u32) as u8;
    *data.add(9) = to_bcd(s, (tm.tm_year % 100) as u32) as u8;

    if set_century != 0 {
        let val = to_bcd(s, ((tm.tm_year / 100) + 19) as u32) as u8;
        *data.add(0x32) = val;
        *data.add(0x37) = val;
    }

    if tv.tv_usec < 244 {
        *data.add(10) |= 0x80;
    } else {
        *data.add(10) &= !0x80u8;
    }
}

#[no_mangle]
pub unsafe extern "C" fn cmos_read__part4000(opaque: *mut c_void, offset: u32) -> u32 {
    let s = opaque as *mut CMOSState;

    if offset == 0 {
        return 0xff;
    }

    let idx = (*s).cmos_index as usize;
    let data = core::ptr::addr_of_mut!((*s).cmos_data).cast::<u8>();

    let ret: u32;
    match (*s).cmos_index {
        0 | 2 | 4 | 6 | 7 | 8 | 9 | 10 => {
            cmos_update_time(s, 0);
            ret = *data.add(idx) as u32;
        }
        12 => {
            ret = *data.add(idx) as u32;
            *data.add(12) = 0x00;
            set_irq((*s).irq, 0);
        }
        _ => {
            ret = *data.add(idx) as u32;
        }
    }

    ret
}
}

// ── `pic_init` · x86_machine.c:322-338 · chunk 4004 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PICState *pic_init(PhysMemoryMap *port_map, int port, int elcr_port,
//                        int elcr_mask,
//                        PICUpdateIRQFunc *update_irq, void *opaque)
//     {
//         PICState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->elcr_mask = elcr_mask;
//         s->update_irq = update_irq;
//         s->opaque = opaque;
//         cpu_register_device(port_map, port, 2, s,
//                             pic_read, pic_write, (1 << 0));
//         cpu_register_device(port_map, elcr_port, 1, s,
//                             pic_elcr_read, pic_elcr_write, (1 << 0));
//         pic_reset(s);
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4004 {
// Rust port of pic_init() from x86_machine.c (Intel 8259 PIC pair members).
//
// pic_read / pic_write / pic_elcr_read / pic_elcr_write / pic_reset /
// pic_get_priority / pic_get_irq are `static` in the C translation unit, so
// they have no linker symbol: their logic is ported here as private functions
// and handed to cpu_register_device as callbacks.

use core::ffi::{c_int, c_void};

/// Opaque mirror of `PhysMemoryMap` (only ever used through pointers here).
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

/// Opaque mirror of `PhysMemoryRange` (only ever used through pointers here).
#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
pub type PicUpdateIrqFunc = unsafe extern "C" fn(*mut c_void);

/// Exact mirror of the C `PICState` layout.
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
    pub update_irq: Option<PicUpdateIrqFunc>,
    pub opaque: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn abort() -> !;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

// DEVIO_SIZE8 == (1 << 0)
const DEVIO_SIZE8: c_int = 1;

unsafe fn pic_reset(s: *mut PICState) {
    (*s).last_irr = 0;
    (*s).irr = 0;
    (*s).imr = 0;
    (*s).isr = 0;
    (*s).priority_add = 0;
    (*s).irq_base = 0;
    (*s).read_reg_select = 0;
    (*s).special_mask = 0;
    (*s).init_state = 0;
    (*s).auto_eoi = 0;
    (*s).rotate_on_autoeoi = 0;
    (*s).init4 = 0;
}

unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + (*s).priority_add as c_int) & 7))) == 0 {
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

unsafe extern "C" fn pic_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut PICState;
    let addr: c_int = (offset & 1) as c_int;

    if addr == 0 {
        if (val & 0x10) != 0 {
            pic_reset(s);
            (*s).init_state = 1;
            (*s).init4 = (val & 1) as u8;
            if (val & 0x02) != 0 {
                abort();
            }
            if (val & 0x08) != 0 {
                abort();
            }
        } else if (val & 0x08) != 0 {
            if (val & 0x02) != 0 {
                (*s).read_reg_select = (val & 1) as u8;
            }
            if (val & 0x40) != 0 {
                (*s).special_mask = ((val >> 5) & 1) as u8;
            }
        } else {
            match val {
                0x00 | 0x80 => {
                    (*s).rotate_on_autoeoi = (val >> 7) as u8;
                }
                0x20 | 0xa0 => {
                    let priority = pic_get_priority(s, (*s).isr as c_int);
                    if priority >= 0 {
                        let bit = ((priority + (*s).priority_add as c_int) & 7) as u32;
                        (*s).isr &= !(1u8 << bit);
                    }
                    if val == 0xa0 {
                        (*s).priority_add = (((*s).priority_add as c_int) + 1 & 7) as u8;
                    }
                }
                0x60..=0x67 => {
                    let priority = (val & 7) as u32;
                    (*s).isr &= !(1u8 << priority);
                }
                0xc0..=0xc7 => {
                    (*s).priority_add = (val.wrapping_add(1) & 7) as u8;
                }
                0xe0..=0xe7 => {
                    let priority = (val & 7) as c_int;
                    (*s).isr &= !(1u8 << (priority as u32));
                    (*s).priority_add = ((priority + 1) & 7) as u8;
                }
                _ => {}
            }
        }
    } else {
        match (*s).init_state {
            0 => {
                (*s).imr = val as u8;
                if let Some(update_irq) = (*s).update_irq {
                    update_irq((*s).opaque);
                }
            }
            1 => {
                (*s).irq_base = (val & 0xf8) as u8;
                (*s).init_state = 2;
            }
            2 => {
                if (*s).init4 != 0 {
                    (*s).init_state = 3;
                } else {
                    (*s).init_state = 0;
                }
            }
            3 => {
                (*s).auto_eoi = ((val >> 1) & 1) as u8;
                (*s).init_state = 0;
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn pic_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut PICState;
    let addr: c_int = (offset & 1) as c_int;
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

unsafe extern "C" fn pic_elcr_write(opaque: *mut c_void, _offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut PICState;
    (*s).elcr = (val & (*s).elcr_mask as u32) as u8;
}

unsafe extern "C" fn pic_elcr_read(opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut PICState;
    (*s).elcr as u32
}

#[no_mangle]
pub unsafe extern "C" fn pic_init(
    port_map: *mut PhysMemoryMap,
    port: c_int,
    elcr_port: c_int,
    elcr_mask: c_int,
    update_irq: Option<PicUpdateIrqFunc>,
    opaque: *mut c_void,
) -> *mut PICState {
    let s = mallocz(core::mem::size_of::<PICState>()) as *mut PICState;

    (*s).elcr_mask = elcr_mask as u8;
    (*s).update_irq = update_irq;
    (*s).opaque = opaque;
    cpu_register_device(
        port_map,
        port as u64,
        2,
        s as *mut c_void,
        Some(pic_read),
        Some(pic_write),
        DEVIO_SIZE8,
    );
    cpu_register_device(
        port_map,
        elcr_port as u64,
        1,
        s as *mut c_void,
        Some(pic_elcr_read),
        Some(pic_elcr_write),
        DEVIO_SIZE8,
    );
    pic_reset(s);
    s
}
}

// ── `PICState *s; s = mallocz(sizeof(*s)); s->elcr_mask = elcr_mask; s->update_irq = update_irq; s->…` · x86_machine.c:326-337 · chunk 4005 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PICState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->elcr_mask = elcr_mask;
//         s->update_irq = update_irq;
//         s->opaque = opaque;
//         cpu_register_device(port_map, port, 2, s,
//                             pic_read, pic_write, (1 << 0));
//         cpu_register_device(port_map, elcr_port, 1, s,
//                             pic_elcr_read, pic_elcr_write, (1 << 0));
//         pic_reset(s);
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4005 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

pub type PICUpdateIRQFunc = unsafe extern "C" fn(*mut c_void);
type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

#[repr(C)]
pub struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: Option<PICUpdateIRQFunc>,
    opaque: *mut c_void,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut c_void;
    fn abort() -> !;
}

unsafe fn pic_reset(s: *mut PICState) {
    (*s).last_irr = 0;
    (*s).irr = 0;
    (*s).imr = 0;
    (*s).isr = 0;
    (*s).priority_add = 0;
    (*s).irq_base = 0;
    (*s).read_reg_select = 0;
    (*s).special_mask = 0;
    (*s).init_state = 0;
    (*s).auto_eoi = 0;
    (*s).rotate_on_autoeoi = 0;
    (*s).init4 = 0;
}

unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + (*s).priority_add as c_int) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

unsafe extern "C" fn rust_pic_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut PICState;
    let addr = offset & 1;

    if addr == 0 {
        if (val & 0x10) != 0 {
            pic_reset(s);
            (*s).init_state = 1;
            (*s).init4 = (val & 1) as u8;
            if (val & 0x02) != 0 {
                abort();
            }
            if (val & 0x08) != 0 {
                abort();
            }
        } else if (val & 0x08) != 0 {
            if (val & 0x02) != 0 {
                (*s).read_reg_select = (val & 1) as u8;
            }
            if (val & 0x40) != 0 {
                (*s).special_mask = ((val >> 5) & 1) as u8;
            }
        } else if val == 0x00 || val == 0x80 {
            (*s).rotate_on_autoeoi = (val >> 7) as u8;
        } else if val == 0x20 || val == 0xa0 {
            let priority = pic_get_priority(s, (*s).isr as c_int);
            if priority >= 0 {
                let m: u8 = 1u8 << (((priority + (*s).priority_add as c_int) & 7) as u32);
                (*s).isr &= !m;
            }
            if val == 0xa0 {
                (*s).priority_add = ((*s).priority_add + 1) & 7;
            }
        } else if val >= 0x60 && val <= 0x67 {
            let priority = (val & 7) as u32;
            let m: u8 = (1u32 << priority) as u8;
            (*s).isr &= !m;
        } else if val >= 0xc0 && val <= 0xc7 {
            (*s).priority_add = (val.wrapping_add(1) & 7) as u8;
        } else if val >= 0xe0 && val <= 0xe7 {
            let priority = (val & 7) as u32;
            let m: u8 = (1u32 << priority) as u8;
            (*s).isr &= !m;
            (*s).priority_add = ((priority + 1) & 7) as u8;
        }
    } else {
        match (*s).init_state {
            0 => {
                (*s).imr = val as u8;
                match (*s).update_irq {
                    Some(f) => f((*s).opaque),
                    None => abort(),
                }
            }
            1 => {
                (*s).irq_base = (val & 0xf8) as u8;
                (*s).init_state = 2;
            }
            2 => {
                if (*s).init4 != 0 {
                    (*s).init_state = 3;
                } else {
                    (*s).init_state = 0;
                }
            }
            3 => {
                (*s).auto_eoi = ((val >> 1) & 1) as u8;
                (*s).init_state = 0;
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn rust_pic_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut PICState;
    let addr = offset & 1;
    let ret: c_int;
    if addr == 0 {
        if (*s).read_reg_select != 0 {
            ret = (*s).isr as c_int;
        } else {
            ret = (*s).irr as c_int;
        }
    } else {
        ret = (*s).imr as c_int;
    }
    ret as u32
}

unsafe extern "C" fn rust_pic_elcr_write(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut PICState;
    (*s).elcr = (val & (*s).elcr_mask as u32) as u8;
}

unsafe extern "C" fn rust_pic_elcr_read(
    opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s = opaque as *mut PICState;
    (*s).elcr as u32
}

#[no_mangle]
pub unsafe extern "C" fn pic_init__part4005(
    port_map: *mut PhysMemoryMap,
    port: c_int,
    elcr_port: c_int,
    elcr_mask: c_int,
    update_irq: Option<PICUpdateIRQFunc>,
    opaque: *mut c_void,
) -> *mut PICState {
    let s = mallocz(core::mem::size_of::<PICState>()) as *mut PICState;

    (*s).elcr_mask = elcr_mask as u8;
    (*s).update_irq = update_irq;
    (*s).opaque = opaque;
    cpu_register_device(
        port_map,
        (port as i64) as u64,
        2,
        s as *mut c_void,
        Some(rust_pic_read),
        Some(rust_pic_write),
        1 << 0,
    );
    cpu_register_device(
        port_map,
        (elcr_port as i64) as u64,
        1,
        s as *mut c_void,
        Some(rust_pic_elcr_read),
        Some(rust_pic_elcr_write),
        1 << 0,
    );
    pic_reset(s);
    s
}
}

// ── `pic_reset` · x86_machine.c:340-355 · chunk 4006 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_reset(PICState *s)
//     {
//     
//         s->last_irr = 0;
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
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4006 {
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
pub unsafe extern "C" fn pic_reset(s: *mut PICState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    s.last_irr = 0;
    s.irr = 0;
    s.imr = 0;
    s.isr = 0;
    s.priority_add = 0;
    s.irq_base = 0;
    s.read_reg_select = 0;
    s.special_mask = 0;
    s.init_state = 0;
    s.auto_eoi = 0;
    s.rotate_on_autoeoi = 0;
    s.init4 = 0;
}

// silence unused import warning if c_int unused
#[allow(dead_code)]
fn _unused(_x: c_int) {}
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

// ── `pic_set_irq1` · x86_machine.c:358-381 · chunk 4008 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_set_irq1(PICState *s, int irq, int level)
//     {
//         int mask;
//         mask = 1 << irq;
//         if (s->elcr & mask) {
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
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4008 {
// Rewrite of pic_set_irq1 from x86_machine.c (static void pic_set_irq1).

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
pub unsafe extern "C" fn pic_set_irq1(s: *mut PICState, irq: core::ffi::c_int, level: core::ffi::c_int) {
    if s.is_null() {
        return;
    }
    let pic = &mut *s;
    let mask: core::ffi::c_int = (1 as core::ffi::c_int).wrapping_shl(irq as u32);
    if (pic.elcr as core::ffi::c_int) & mask != 0 {
        // level triggered
        if level != 0 {
            pic.irr = (pic.irr as core::ffi::c_int | mask) as u8;
            pic.last_irr = (pic.last_irr as core::ffi::c_int | mask) as u8;
        } else {
            pic.irr = (pic.irr as core::ffi::c_int & !mask) as u8;
            pic.last_irr = (pic.last_irr as core::ffi::c_int & !mask) as u8;
        }
    } else {
        // edge triggered
        if level != 0 {
            if (pic.last_irr as core::ffi::c_int) & mask == 0 {
                pic.irr = (pic.irr as core::ffi::c_int | mask) as u8;
            }
            pic.last_irr = (pic.last_irr as core::ffi::c_int | mask) as u8;
        } else {
            pic.last_irr = (pic.last_irr as core::ffi::c_int & !mask) as u8;
        }
    }
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

// ── `pic_get_priority` · x86_machine.c:383-392 · chunk 4011 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pic_get_priority(PICState *s, int mask)
//     {
//         int priority;
//         if (mask == 0)
//             return -1;
//         priority = 7;
//         while ((mask & (1 << ((priority + s->priority_add) & 7))) == 0)
//             priority--;
//         return priority;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4011 {
// Port of pic_get_priority from x86_machine.c
use core::ffi::{c_int, c_uchar};

#[repr(C)]
pub struct PICState {
    pub last_irr: c_uchar,
    pub irr: c_uchar,
    pub imr: c_uchar,
    pub isr: c_uchar,
    pub priority_add: c_uchar,
    pub irq_base: c_uchar,
    pub read_reg_select: c_uchar,
    pub special_mask: c_uchar,
    pub init_state: c_uchar,
    pub auto_eoi: c_uchar,
    pub rotate_on_autoeoi: c_uchar,
    pub init4: c_uchar,
    pub elcr: c_uchar,
    pub elcr_mask: c_uchar,
    pub update_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let priority_add = (*s).priority_add as c_int;
    let mut priority: c_int = 7;
    while (mask & (1i32 << (((priority + priority_add) & 7) as u32))) == 0 {
        priority -= 1;
    }
    priority
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

// ── `pic_get_irq` · x86_machine.c:395-411 · chunk 4016 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pic_get_irq(PICState *s)
//     {
//         int mask, cur_priority, priority;
//     
//         mask = s->irr & ~s->imr;
//         priority = pic_get_priority(s, mask);
//         if (priority < 0)
//             return -1;
//     
//         cur_priority = pic_get_priority(s, s->isr);
//         if (priority > cur_priority) {
//     
//             return priority;
//         } else {
//             return -1;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4016 {
use core::ffi::{c_int, c_void};

// Mirror of the C PICState struct (fields in declaration order).
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

// Ported from the C static function of the same name.
fn pic_get_priority(s: *const PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let priority_add = unsafe { (*s).priority_add } as c_int;
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + priority_add) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

#[no_mangle]
pub unsafe extern "C" fn pic_get_irq(s: *mut PICState) -> c_int {
    let irr = (*s).irr as c_int;
    let imr = (*s).imr as c_int;
    let isr = (*s).isr as c_int;

    let mask: c_int = irr & !imr;
    let priority: c_int = pic_get_priority(s, mask);
    if priority < 0 {
        return -1;
    }

    let cur_priority: c_int = pic_get_priority(s, isr);
    if priority > cur_priority {
        priority
    } else {
        -1
    }
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

// ── `pic_intack` · x86_machine.c:414-425 · chunk 4021 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_intack(PICState *s, int irq)
//     {
//         if (s->auto_eoi) {
//             if (s->rotate_on_autoeoi)
//                 s->priority_add = (irq + 1) & 7;
//         } else {
//             s->isr |= (1 << irq);
//         }
//     
//         if (!(s->elcr & (1 << irq)))
//             s->irr &= ~(1 << irq);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4021 {
// Rust rewrite of pic_intack from x86_machine.c
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};

// Mirror of the C `PICState` typedef (anonymous struct) used through a pointer.
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

#[no_mangle]
pub unsafe extern "C" fn pic_intack(s: *mut PICState, irq: c_int) {
    let s = &mut *s;

    // C: int mask expressed as 1 << irq (UB for out-of-range shifts; defined here)
    let bit: i32 = 1i32.wrapping_shl(irq as u32);
    let bit_u8: u8 = (bit & 0xff) as u8;

    if s.auto_eoi != 0 {
        if s.rotate_on_autoeoi != 0 {
            s.priority_add = (irq.wrapping_add(1) & 7) as u8;
        }
    } else {
        s.isr |= bit_u8;
    }

    // We don't clear a level sensitive interrupt here
    if (s.elcr as i32 & bit) == 0 {
        s.irr &= !bit_u8;
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

// ── `pic_write` · x86_machine.c:427-526 · chunk 4024 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_write(void *opaque, uint32_t offset,
//                           uint32_t val, int size_log2)
//     {
//         PICState *s = opaque;
//         int priority, addr;
//     
//         addr = offset & 1;
//     
//     
//     
//         if (addr == 0) {
//             if (val & 0x10) {
//     
//                 pic_reset(s);
//                 s->init_state = 1;
//                 s->init4 = val & 1;
//                 if (val & 0x02)
//                     abort();
//                 if (val & 0x08)
//                     abort();
//             } else if (val & 0x08) {
//                 if (val & 0x02)
//                     s->read_reg_select = val & 1;
//                 if (val & 0x40)
//                     s->special_mask = (val >> 5) & 1;
//             } else {
//                 switch(val) {
//                 case 0x00:
//                 case 0x80:
//                     s->rotate_on_autoeoi = val >> 7;
//                     break;
//                 case 0x20:
//                 case 0xa0:
//                     priority = pic_get_priority(s, s->isr);
//                     if (priority >= 0) {
//                         s->isr &= ~(1 << ((priority + s->priority_add) & 7));
//                     }
//                     if (val == 0xa0)
//                         s->priority_add = (s->priority_add + 1) & 7;
//                     break;
//                 case 0x60:
//                 case 0x61:
//                 case 0x62:
//                 case 0x63:
//                 case 0x64:
//                 case 0x65:
//                 case 0x66:
//                 case 0x67:
//                     priority = val & 7;
//                     s->isr &= ~(1 << priority);
//                     break;
//                 case 0xc0:
//                 case 0xc1:
//                 case 0xc2:
//                 case 0xc3:
//                 case 0xc4:
//                 case 0xc5:
//                 case 0xc6:
//                 case 0xc7:
//                     s->priority_add = (val + 1) & 7;
//                     break;
//                 case 0xe0:
//                 case 0xe1:
//                 case 0xe2:
//                 case 0xe3:
//                 case 0xe4:
//                 case 0xe5:
//                 case 0xe6:
//                 case 0xe7:
//                     priority = val & 7;
//                     s->isr &= ~(1 << priority);
//                     s->priority_add = (priority + 1) & 7;
//                     break;
//                 }
//             }
//         } else {
//             switch(s->init_state) {
//             case 0:
//     
//                 s->imr = val;
//                 s->update_irq(s->opaque);
//                 break;
//             case 1:
//                 s->irq_base = val & 0xf8;
//                 s->init_state = 2;
//                 break;
//             case 2:
//                 if (s->init4) {
//                     s->init_state = 3;
//                 } else {
//                     s->init_state = 0;
//                 }
//                 break;
//             case 3:
//                 s->auto_eoi = (val >> 1) & 1;
//                 s->init_state = 0;
//                 break;
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4024 {
// Port of pic_write (x86_machine.c) to Rust.
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};

type PICUpdateIRQFunc = unsafe extern "C" fn(opaque: *mut c_void);

#[repr(C)]
pub struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: Option<PICUpdateIRQFunc>,
    opaque: *mut c_void,
}

extern "C" {
    fn abort() -> !;
}

// static in C: no linker symbol, logic ported here.
unsafe fn pic_reset(s: *mut PICState) {
    let s = &mut *s;
    s.last_irr = 0;
    s.irr = 0;
    s.imr = 0;
    s.isr = 0;
    s.priority_add = 0;
    s.irq_base = 0;
    s.read_reg_select = 0;
    s.special_mask = 0;
    s.init_state = 0;
    s.auto_eoi = 0;
    s.rotate_on_autoeoi = 0;
    s.init4 = 0;
}

// static in C: no linker symbol, logic ported here.
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let priority_add = (*s).priority_add as c_int;
    let mut priority: c_int = 7;
    while (mask & (1i32 << ((priority + priority_add) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

#[no_mangle]
pub unsafe extern "C" fn pic_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut PICState;
    let st = &mut *s;
    let mut priority: c_int;

    let addr: c_int = (offset & 1) as c_int;

    if addr == 0 {
        if (val & 0x10) != 0 {
            // init
            pic_reset(s);
            st.init_state = 1;
            st.init4 = (val & 1) as u8;
            if (val & 0x02) != 0 {
                abort(); // "single mode not supported"
            }
            if (val & 0x08) != 0 {
                abort(); // "level sensitive irq not supported"
            }
        } else if (val & 0x08) != 0 {
            if (val & 0x02) != 0 {
                st.read_reg_select = (val & 1) as u8;
            }
            if (val & 0x40) != 0 {
                st.special_mask = ((val >> 5) & 1) as u8;
            }
        } else {
            match val {
                0x00 | 0x80 => {
                    st.rotate_on_autoeoi = (val >> 7) as u8;
                }
                0x20 | 0xa0 => {
                    // end of interrupt
                    priority = pic_get_priority(s, st.isr as c_int);
                    if priority >= 0 {
                        let shift = (priority + st.priority_add as c_int) & 7;
                        st.isr &= !(1i32 << shift) as u8;
                    }
                    if val == 0xa0 {
                        st.priority_add = st.priority_add.wrapping_add(1) & 7;
                    }
                }
                0x60..=0x67 => {
                    priority = (val & 7) as c_int;
                    st.isr &= !(1i32 << priority) as u8;
                }
                0xc0..=0xc7 => {
                    st.priority_add = (val.wrapping_add(1) & 7) as u8;
                }
                0xe0..=0xe7 => {
                    priority = (val & 7) as c_int;
                    st.isr &= !(1i32 << priority) as u8;
                    st.priority_add = ((priority + 1) & 7) as u8;
                }
                _ => {}
            }
        }
    } else {
        match st.init_state {
            0 => {
                // normal mode
                st.imr = val as u8;
                if let Some(f) = st.update_irq {
                    f(st.opaque);
                }
            }
            1 => {
                st.irq_base = (val & 0xf8) as u8;
                st.init_state = 2;
            }
            2 => {
                if st.init4 != 0 {
                    st.init_state = 3;
                } else {
                    st.init_state = 0;
                }
            }
            3 => {
                st.auto_eoi = ((val >> 1) & 1) as u8;
                st.init_state = 0;
            }
            _ => {}
        }
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

// ── `if (addr == 0) … else` · x86_machine.c:437-525 · chunk 4026 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 0) {
//             if (val & 0x10) {
//     
//                 pic_reset(s);
//                 s->init_state = 1;
//                 s->init4 = val & 1;
//                 if (val & 0x02)
//                     abort();
//                 if (val & 0x08)
//                     abort();
//             } else if (val & 0x08) {
//                 if (val & 0x02)
//                     s->read_reg_select = val & 1;
//                 if (val & 0x40)
//                     s->special_mask = (val >> 5) & 1;
//             } else {
//                 switch(val) {
//                 case 0x00:
//                 case 0x80:
//                     s->rotate_on_autoeoi = val >> 7;
//                     break;
//                 case 0x20:
//                 case 0xa0:
//                     priority = pic_get_priority(s, s->isr);
//                     if (priority >= 0) {
//                         s->isr &= ~(1 << ((priority + s->priority_add) & 7));
//                     }
//                     if (val == 0xa0)
//                         s->priority_add = (s->priority_add + 1) & 7;
//                     break;
//                 case 0x60:
//                 case 0x61:
//                 case 0x62:
//                 case 0x63:
//                 case 0x64:
//                 case 0x65:
//                 case 0x66:
//                 case 0x67:
//                     priority = val & 7;
//                     s->isr &= ~(1 << priority);
//                     break;
//                 case 0xc0:
//                 case 0xc1:
//                 case 0xc2:
//                 case 0xc3:
//                 case 0xc4:
//                 case 0xc5:
//                 case 0xc6:
//                 case 0xc7:
//                     s->priority_add = (val + 1) & 7;
//                     break;
//                 case 0xe0:
//                 case 0xe1:
//                 case 0xe2:
//                 case 0xe3:
//                 case 0xe4:
//                 case 0xe5:
//                 case 0xe6:
//                 case 0xe7:
//                     priority = val & 7;
//                     s->isr &= ~(1 << priority);
//                     s->priority_add = (priority + 1) & 7;
//                     break;
//                 }
//             }
//         } else {
//             switch(s->init_state) {
//             case 0:
//     
//                 s->imr = val;
//                 s->update_irq(s->opaque);
//                 break;
//             case 1:
//                 s->irq_base = val & 0xf8;
//                 s->init_state = 2;
//                 break;
//             case 2:
//                 if (s->init4) {
//                     s->init_state = 3;
//                 } else {
//                     s->init_state = 0;
//                 }
//                 break;
//             case 3:
//                 s->auto_eoi = (val >> 1) & 1;
//                 s->init_state = 0;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4026 {
// Ported piece of pic_write (x86_machine.c): the addr/data register write logic.
use core::ffi::{c_int, c_void};

type PICUpdateIRQFunc = unsafe extern "C" fn(*mut c_void);

#[repr(C)]
pub struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: Option<PICUpdateIRQFunc>,
    opaque: *mut c_void,
}

extern "C" {
    fn abort() -> !;
}

// static in C: pic_reset -- ported logic, no linker symbol.
unsafe fn pic_reset(s: *mut PICState) {
    (*s).last_irr = 0;
    (*s).irr = 0;
    (*s).imr = 0;
    (*s).isr = 0;
    (*s).priority_add = 0;
    (*s).irq_base = 0;
    (*s).read_reg_select = 0;
    (*s).special_mask = 0;
    (*s).init_state = 0;
    (*s).auto_eoi = 0;
    (*s).rotate_on_autoeoi = 0;
    (*s).init4 = 0;
}

// static in C: pic_get_priority -- ported logic, no linker symbol.
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1i32 << (((priority + (*s).priority_add as c_int) & 7) as u32))) == 0 {
        priority -= 1;
    }
    priority
}

#[no_mangle]
pub unsafe extern "C" fn pic_write__part4026(s: *mut PICState, addr: c_int, val: u32) {
    if addr == 0 {
        if (val & 0x10) != 0 {
            pic_reset(s);
            (*s).init_state = 1;
            (*s).init4 = (val & 1) as u8;
            if (val & 0x02) != 0 {
                abort();
            }
            if (val & 0x08) != 0 {
                abort();
            }
        } else if (val & 0x08) != 0 {
            if (val & 0x02) != 0 {
                (*s).read_reg_select = (val & 1) as u8;
            }
            if (val & 0x40) != 0 {
                (*s).special_mask = ((val >> 5) & 1) as u8;
            }
        } else {
            match val {
                0x00 | 0x80 => {
                    (*s).rotate_on_autoeoi = (val >> 7) as u8;
                }
                0x20 | 0xa0 => {
                    let priority: c_int = pic_get_priority(s, (*s).isr as c_int);
                    if priority >= 0 {
                        let sh: u32 = ((priority + (*s).priority_add as c_int) & 7) as u32;
                        (*s).isr &= !((1i32 << sh) as u8);
                    }
                    if val == 0xa0 {
                        (*s).priority_add = ((*s).priority_add + 1) & 7;
                    }
                }
                0x60..=0x67 => {
                    let priority: u32 = val & 7;
                    (*s).isr &= !((1u32 << priority) as u8);
                }
                0xc0..=0xc7 => {
                    (*s).priority_add = (val.wrapping_add(1) & 7) as u8;
                }
                0xe0..=0xe7 => {
                    let priority: u32 = val & 7;
                    (*s).isr &= !((1u32 << priority) as u8);
                    (*s).priority_add = ((priority + 1) & 7) as u8;
                }
                _ => {}
            }
        }
    } else {
        match (*s).init_state {
            0 => {
                (*s).imr = val as u8;
                if let Some(f) = (*s).update_irq {
                    f((*s).opaque);
                }
            }
            1 => {
                (*s).irq_base = (val & 0xf8) as u8;
                (*s).init_state = 2;
            }
            2 => {
                if (*s).init4 != 0 {
                    (*s).init_state = 3;
                } else {
                    (*s).init_state = 0;
                }
            }
            3 => {
                (*s).auto_eoi = ((val >> 1) & 1) as u8;
                (*s).init_state = 0;
            }
            _ => {}
        }
    }
}
}

// ── `pic_read` · x86_machine.c:528-546 · chunk 4027 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t pic_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         PICState *s = opaque;
//         int addr, ret;
//     
//         addr = offset & 1;
//         if (addr == 0) {
//             if (s->read_reg_select)
//                 ret = s->isr;
//             else
//                 ret = s->irr;
//         } else {
//             ret = s->imr;
//         }
//     
//     
//     
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4027 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: *mut c_void,
    opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn pic_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut PICState;
    let addr = offset & 1;

    let ret: c_int = unsafe {
        if addr == 0 {
            if (*s).read_reg_select != 0 {
                (*s).isr as c_int
            } else {
                (*s).irr as c_int
            }
        } else {
            (*s).imr as c_int
        }
    };

    ret as u32
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

// ── `pic_elcr_write` · x86_machine.c:548-553 · chunk 4031 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_elcr_write(void *opaque, uint32_t offset,
//                                uint32_t val, int size_log2)
//     {
//         PICState *s = opaque;
//         s->elcr = val & s->elcr_mask;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4031 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: *mut c_void,
    opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn pic_elcr_write(opaque: *mut c_void, _offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut PICState;
    unsafe {
        if !s.is_null() {
            (*s).elcr = (val & (*s).elcr_mask as u32) as u8;
        }
    }
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

// ── `pic_elcr_read` · x86_machine.c:555-559 · chunk 4033 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t pic_elcr_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         PICState *s = opaque;
//         return s->elcr;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4033 {
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

#[no_mangle]
pub unsafe extern "C" fn pic_elcr_read(opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *const PICState;
    if s.is_null() {
        return 0;
    }
    (*s).elcr as u32
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

// ── `pic2_init` · x86_machine.c:575-594 · chunk 4037 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PIC2State *pic2_init(PhysMemoryMap *port_map, uint32_t addr0, uint32_t addr1,
//                          uint32_t elcr_addr0, uint32_t elcr_addr1,
//                          void (*cpu_set_irq)(void *opaque, int level),
//                          void *opaque, IRQSignal *irqs)
//     {
//         PIC2State *s;
//         int i;
//     
//         s = mallocz(sizeof(*s));
//     
//         for(i = 0; i < 16; i++) {
//             irq_init(&irqs[i], pic2_set_irq, s, i);
//         }
//         s->cpu_set_irq = cpu_set_irq;
//         s->opaque = opaque;
//         s->pics[0] = pic_init(port_map, addr0, elcr_addr0, 0xf8, pic2_update_irq, s);
//         s->pics[1] = pic_init(port_map, addr1, elcr_addr1, 0xde, pic2_update_irq, s);
//         s->irq_requested = 0;
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4037 {
// Port of pic2_init() from x86_machine.c
// The C statics pic2_update_irq(), pic2_set_irq(), pic_set_irq1(), pic_get_priority()
// and pic_get_irq() are ported here as private Rust fns (they have no linker symbols),
// so their function pointers can be handed to pic_init() / irq_init().

use core::ffi::{c_int, c_void};
use core::mem::size_of;

/// Opaque in this translation unit (only ever used through a pointer).
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

/// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

/// typedef void PICUpdateIRQFunc(void *opaque);
pub type PICUpdateIRQFunc = unsafe extern "C" fn(*mut c_void);

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

/// void (*cpu_set_irq)(void *opaque, int level);
pub type CpuSetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<CpuSetIRQFunc>,
    pub opaque: *mut c_void,
    pub irqs: *mut IRQSignal,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<SetIRQFunc>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
    fn pic_init(
        port_map: *mut PhysMemoryMap,
        port: c_int,
        elcr_port: c_int,
        elcr_mask: c_int,
        update_irq: Option<PICUpdateIRQFunc>,
        opaque: *mut c_void,
    ) -> *mut PICState;
}

// static void pic_set_irq1(PICState *s, int irq, int level)
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = 1i32.wrapping_shl((irq & 31) as u32);
    if ((*s).elcr as c_int) & mask != 0 {
        // level triggered mode
        if level != 0 {
            (*s).irr |= mask as u8;
            (*s).last_irr |= mask as u8;
        } else {
            (*s).irr = (((*s).irr as c_int) & !mask) as u8;
            (*s).last_irr = (((*s).last_irr as c_int) & !mask) as u8;
        }
    } else {
        // edge triggered mode
        if level != 0 {
            if ((*s).last_irr as c_int) & mask == 0 {
                (*s).irr |= mask as u8;
            }
            (*s).last_irr |= mask as u8;
        } else {
            (*s).last_irr = (((*s).last_irr as c_int) & !mask) as u8;
        }
    }
}

// static int pic_get_priority(PICState *s, int mask)
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask
        & (1i32.wrapping_shl(((priority + (*s).priority_add as c_int) & 7) as u32)))
        == 0
    {
        priority -= 1;
    }
    priority
}

// static int pic_get_irq(PICState *s)
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

// static void pic2_update_irq(void *opaque)
unsafe extern "C" fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;

    // first look at slave pic
    let irq2 = pic_get_irq((*s).pics[1]);
    if irq2 >= 0 {
        // if irq request by slave pic, signal master PIC
        pic_set_irq1((*s).pics[0], 2, 1);
        pic_set_irq1((*s).pics[0], 2, 0);
    }

    // look at master pic
    let irq = pic_get_irq((*s).pics[0]);
    let level: c_int = if irq >= 0 { 1 } else { 0 };
    if let Some(f) = (*s).cpu_set_irq {
        f((*s).opaque, level);
    }
}

// static void pic2_set_irq(void *opaque, int irq, int level)
unsafe extern "C" fn pic2_set_irq(opaque: *mut c_void, irq: c_int, level: c_int) {
    let s = opaque as *mut PIC2State;

    pic_set_irq1((*s).pics[((irq >> 3) & 1) as usize], irq & 7, level);
    pic2_update_irq(s as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn pic2_init(
    port_map: *mut PhysMemoryMap,
    addr0: u32,
    addr1: u32,
    elcr_addr0: u32,
    elcr_addr1: u32,
    cpu_set_irq: Option<CpuSetIRQFunc>,
    opaque: *mut c_void,
    irqs: *mut IRQSignal,
) -> *mut PIC2State {
    let s = mallocz(size_of::<PIC2State>()) as *mut PIC2State;

    let mut i: c_int = 0;
    while i < 16 {
        irq_init(
            irqs.add(i as usize),
            Some(pic2_set_irq as SetIRQFunc),
            s as *mut c_void,
            i,
        );
        i += 1;
    }
    (*s).cpu_set_irq = cpu_set_irq;
    (*s).opaque = opaque;
    (*s).pics[0] = pic_init(
        port_map,
        addr0 as c_int,
        elcr_addr0 as c_int,
        0xf8,
        Some(pic2_update_irq),
        s as *mut c_void,
    );
    (*s).pics[1] = pic_init(
        port_map,
        addr1 as c_int,
        elcr_addr1 as c_int,
        0xde,
        Some(pic2_update_irq),
        s as *mut c_void,
    );
    (*s).irq_requested = 0;
    s
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

// ── `for (i = 0; i < 16; i++)` · x86_machine.c:585-587 · chunk 4039 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 16; i++) {
//             irq_init(&irqs[i], pic2_set_irq, s, i);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4039 {
// Port of the pic2_init loop that registers the 16 PIC IRQ signals.
use core::ffi::{c_int, c_void};

type SetIrqFn = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IrqSignal {
    pub set_irq: Option<SetIrqFn>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

extern "C" {
    fn irq_init(
        irq: *mut IrqSignal,
        set_irq: Option<SetIrqFn>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pic2_init__part4039(
    irqs: *mut IrqSignal,
    set_irq: Option<SetIrqFn>,
    opaque: *mut c_void,
) {
    let mut i: c_int = 0;
    while i < 16 {
        irq_init(irqs.add(i as usize), set_irq, opaque, i);
        i += 1;
    }
}
}

// ── `s->cpu_set_irq = cpu_set_irq; s->opaque = opaque; s->pics[0] = pic_init(port_map, addr0, elcr_a…` · x86_machine.c:588-593 · chunk 4040 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cpu_set_irq = cpu_set_irq;
//         s->opaque = opaque;
//         s->pics[0] = pic_init(port_map, addr0, elcr_addr0, 0xf8, pic2_update_irq, s);
//         s->pics[1] = pic_init(port_map, addr1, elcr_addr1, 0xde, pic2_update_irq, s);
//         s->irq_requested = 0;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4040 {
// Port of the second half of pic2_init (x86_machine.c): wires up the 8259
// PIC pair plus the callback fields, then hands the state back to C.
use core::ffi::{c_int, c_void};

// Mirror of the C PIC2State layout (PICState*/IRQSignal* kept opaque).
#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut c_void; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

extern "C" {
    fn pic_init(
        port_map: *mut c_void,
        port: c_int,
        elcr_port: c_int,
        elcr_mask: c_int,
        update_irq: Option<unsafe extern "C" fn(*mut c_void)>,
        opaque: *mut c_void,
    ) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pic2_init__part4040(
    s: *mut PIC2State,
    port_map: *mut c_void,
    addr0: u32,
    addr1: u32,
    elcr_addr0: u32,
    elcr_addr1: u32,
    cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    opaque: *mut c_void,
    update_irq: Option<unsafe extern "C" fn(*mut c_void)>,
) -> *mut PIC2State {
    (*s).cpu_set_irq = cpu_set_irq;
    (*s).opaque = opaque;
    (*s).pics[0] = pic_init(
        port_map,
        addr0 as c_int,
        elcr_addr0 as c_int,
        0xf8,
        update_irq,
        s as *mut c_void,
    );
    (*s).pics[1] = pic_init(
        port_map,
        addr1 as c_int,
        elcr_addr1 as c_int,
        0xde,
        update_irq,
        s as *mut c_void,
    );
    (*s).irq_requested = 0;
    s
}
}

// ── `pic2_set_elcr` · x86_machine.c:596-602 · chunk 4041 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void pic2_set_elcr(PIC2State *s, const uint8_t *elcr)
//     {
//         int i;
//         for(i = 0; i < 2; i++) {
//             s->pics[i]->elcr = elcr[i] & s->pics[i]->elcr_mask;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4041 {
// Port of x86_machine.c: pic2_set_elcr

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

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn pic2_set_elcr(s: *mut PIC2State, elcr: *const u8) {
    let mut i: c_int = 0;
    while i < 2 {
        let idx = i as usize;
        let pic = core::ptr::addr_of!((*s).pics).cast::<*mut PICState>().add(idx).read();
        let val = elcr.add(idx).read();
        (*pic).elcr = val & (*pic).elcr_mask;
        i += 1;
    }
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

// ── `pic2_update_irq` · x86_machine.c:606-630 · chunk 4043 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic2_update_irq(void *opaque)
//     {
//         PIC2State *s = opaque;
//         int irq2, irq;
//     
//     
//         irq2 = pic_get_irq(s->pics[1]);
//         if (irq2 >= 0) {
//     
//             pic_set_irq1(s->pics[0], 2, 1);
//             pic_set_irq1(s->pics[0], 2, 0);
//         }
//     
//         irq = pic_get_irq(s->pics[0]);
//     
//     
//     
//         if (irq >= 0) {
//     
//             s->cpu_set_irq(s->opaque, 1);
//         } else {
//     
//             s->cpu_set_irq(s->opaque, 0);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4043 {
// Port of static void pic2_update_irq(void *opaque) from x86_machine.c
//
// The helpers pic_get_irq(), pic_get_priority() and pic_set_irq1() are
// `static` functions in the C translation unit (no linker symbols), so their
// logic is reproduced here as private Rust functions.

use core::ffi::{c_int, c_void};

// typedef void PICUpdateIRQFunc(void *opaque);  -- see PICState below.
type PICPUpdateIRQFunc = Option<unsafe extern "C" fn(*mut c_void)>;
type PICSetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;
type CPUSetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int)>;

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
    pub update_irq: PICPUpdateIRQFunc,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: PICSetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: CPUSetIRQFunc,
    pub opaque: *mut c_void,
    pub irqs: *mut IRQSignal,
}

// static void pic_set_irq1(PICState *s, int irq, int level)
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = 1 << irq;
    if ((*s).elcr as c_int) & mask != 0 {
        if level != 0 {
            (*s).irr |= mask as u8;
            (*s).last_irr |= mask as u8;
        } else {
            (*s).irr &= !(mask as u8);
            (*s).last_irr &= !(mask as u8);
        }
    } else {
        if level != 0 {
            if ((*s).last_irr as c_int) & mask == 0 {
                (*s).irr |= mask as u8;
            }
            (*s).last_irr |= mask as u8;
        } else {
            (*s).last_irr &= !(mask as u8);
        }
    }
}

// static int pic_get_priority(PICState *s, int mask)
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + (*s).priority_add as c_int) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

// static int pic_get_irq(PICState *s)
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

#[no_mangle]
pub unsafe extern "C" fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;

    // first look at slave pic
    let irq2 = pic_get_irq(*(*s).pics.as_ptr().add(1));
    if irq2 >= 0 {
        // if irq request by slave pic, signal master PIC
        pic_set_irq1(*(*s).pics.as_ptr().add(0), 2, 1);
        pic_set_irq1(*(*s).pics.as_ptr().add(0), 2, 0);
    }
    // look at requested irq
    let irq = pic_get_irq(*(*s).pics.as_ptr().add(0));

    if irq >= 0 {
        // raise IRQ request on the CPU
        if let Some(f) = (*s).cpu_set_irq {
            f((*s).opaque, 1);
        }
    } else {
        // lower irq
        if let Some(f) = (*s).cpu_set_irq {
            f((*s).opaque, 0);
        }
    }
}
}

// ── `PIC2State *s = opaque; int irq2, irq; irq2 = pic_get_irq(s->pics[1]);` · x86_machine.c:608-612 · chunk 4044 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PIC2State *s = opaque;
//         int irq2, irq;
//     
//     
//         irq2 = pic_get_irq(s->pics[1]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4044 {
use core::ffi::{c_int, c_void};

// Mirror of the C PICState (field order and types must match exactly).
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

// Mirror of the C IRQSignal.
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Mirror of the C PIC2State (field order and types must match exactly).
#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irqs: *mut IRQSignal,
}

// Port of the file-static pic_get_priority().
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1i32 << ((priority + ((*s).priority_add as c_int)) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

// Port of the file-static pic_get_irq().
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
pub unsafe extern "C" fn pic2_update_irq__part4044(s: *mut PIC2State) -> c_int {
    let irq2 = pic_get_irq((*s).pics[1]);
    irq2
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

// ── `pic2_set_irq` · x86_machine.c:632-643 · chunk 4048 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic2_set_irq(void *opaque, int irq, int level)
//     {
//         PIC2State *s = opaque;
//     
//     
//     
//     
//     
//     
//         pic_set_irq1(s->pics[irq >> 3], irq & 7, level);
//         pic2_update_irq(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4048 {
// Port of pic2_set_irq (and the file-static helpers it needs) from x86_machine.c
use core::ffi::{c_int, c_void};

type PICUpdateIRQFunc = Option<unsafe extern "C" fn(*mut c_void)>;
type SetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;

#[repr(C)]
struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: PICUpdateIRQFunc,
    opaque: *mut c_void,
}

#[repr(C)]
struct IRQSignal {
    set_irq: SetIRQFunc,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct PIC2State {
    pics: [*mut PICState; 2],
    irq_requested: c_int,
    cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    opaque: *mut c_void,
    irqs: *mut IRQSignal,
}

#[inline]
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = 1i32.wrapping_shl(irq as u32);
    let m: u8 = mask as u8;
    if ((*s).elcr & m) != 0 {
        // level triggered
        if level != 0 {
            (*s).irr |= m;
            (*s).last_irr |= m;
        } else {
            (*s).irr &= !m;
            (*s).last_irr &= !m;
        }
    } else {
        // edge triggered
        if level != 0 {
            if ((*s).last_irr & m) == 0 {
                (*s).irr |= m;
            }
            (*s).last_irr |= m;
        } else {
            (*s).last_irr &= !m;
        }
    }
}

unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1i32.wrapping_shl((((priority + (*s).priority_add as c_int) & 7) as u32)))) == 0
    {
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

unsafe fn pic2_update_irq(opaque: *mut c_void) {
    let s: *mut PIC2State = opaque as *mut PIC2State;

    let irq2 = pic_get_irq(*((*s).pics.as_ptr().add(1)));
    if irq2 >= 0 {
        // if irq request by slave pic, signal master PIC
        pic_set_irq1(*((*s).pics.as_ptr().add(0)), 2, 1);
        pic_set_irq1(*((*s).pics.as_ptr().add(0)), 2, 0);
    }
    // look at requested irq
    let irq = pic_get_irq(*((*s).pics.as_ptr().add(0)));
    match (*s).cpu_set_irq {
        Some(f) => {
            if irq >= 0 {
                // raise IRQ request on the CPU
                f((*s).opaque, 1);
            } else {
                // lower irq
                f((*s).opaque, 0);
            }
        }
        None => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn pic2_set_irq(opaque: *mut c_void, irq: c_int, level: c_int) {
    let s: *mut PIC2State = opaque as *mut PIC2State;
    let idx = (irq >> 3) as usize;
    let pic: *mut PICState = if idx < 2 {
        *((*s).pics.as_ptr().add(idx))
    } else {
        core::ptr::null_mut()
    };
    if !pic.is_null() {
        pic_set_irq1(pic, irq & 7, level);
    }
    pic2_update_irq(opaque);
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

// ── `pic2_get_hard_intno` · x86_machine.c:646-678 · chunk 4050 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pic2_get_hard_intno(PIC2State *s)
//     {
//         int irq, irq2, intno;
//     
//         irq = pic_get_irq(s->pics[0]);
//         if (irq >= 0) {
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
//         pic2_update_irq(s);
//     
//     
//     
//     
//     
//         return intno;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4050 {
// Rewrite of pic2_get_hard_intno (x86_machine.c) in Rust.
// The PIC helpers it calls (pic_get_irq, pic_intack, pic2_update_irq,
// pic_set_irq1, pic_get_priority) are `static` in C, so they have no linker
// symbol and their logic is ported here as private Rust functions.

use core::ffi::{c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
pub struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: Option<extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IRQSignal {
    set_irq: Option<extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
#[allow(dead_code)]
pub struct PIC2State {
    pics: [*mut PICState; 2],
    irq_requested: c_int,
    cpu_set_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    opaque: *mut c_void,
    irqs: *mut IRQSignal,
}

// static int pic_get_priority(PICState *s, int mask)
unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let pa: c_int = (*s).priority_add as c_int;
    let mut priority: c_int = 7;
    while (mask & (1i32 << ((priority + pa) & 7))) == 0 {
        priority -= 1;
    }
    priority
}

// static int pic_get_irq(PICState *s)
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

// static void pic_intack(PICState *s, int irq)
unsafe fn pic_intack(s: *mut PICState, irq: c_int) {
    if (*s).auto_eoi != 0 {
        if (*s).rotate_on_autoeoi != 0 {
            (*s).priority_add = (((irq + 1) & 7) as u8) & 0xff;
        }
    } else {
        (*s).isr |= (1i32 << irq) as u8;
    }

    if ((*s).elcr & ((1i32 << irq) as u8)) == 0 {
        (*s).irr &= !((1i32 << irq) as u8);
    }
}

// static void pic_set_irq1(PICState *s, int irq, int level)
unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: u8 = (1i32 << irq) as u8;
    if ((*s).elcr & mask) != 0 {
        if level != 0 {
            (*s).irr |= mask;
            (*s).last_irr |= mask;
        } else {
            (*s).irr &= !mask;
            (*s).last_irr &= !mask;
        }
    } else if level != 0 {
        if ((*s).last_irr & mask) == 0 {
            (*s).irr |= mask;
        }
        (*s).last_irr |= mask;
    } else {
        (*s).last_irr &= !mask;
    }
}

// static void pic2_update_irq(void *opaque)
unsafe fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;
    let master: *mut PICState = (*s).pics[0];
    let slave: *mut PICState = (*s).pics[1];

    let irq2 = pic_get_irq(slave);
    if irq2 >= 0 {
        pic_set_irq1(master, 2, 1);
        pic_set_irq1(master, 2, 0);
    }

    let irq = pic_get_irq(master);

    let cpu_set_irq = (*s).cpu_set_irq;
    let ctx = (*s).opaque;
    if let Some(f) = cpu_set_irq {
        if irq >= 0 {
            f(ctx, 1);
        } else {
            f(ctx, 0);
        }
    }
}

// static int pic2_get_hard_intno(PIC2State *s)
#[no_mangle]
pub unsafe extern "C" fn pic2_get_hard_intno(s: *mut PIC2State) -> c_int {
    let master: *mut PICState = (*s).pics[0];
    let slave: *mut PICState = (*s).pics[1];

    let mut irq: c_int;
    let mut irq2: c_int;
    let intno: c_int;

    irq = pic_get_irq(master);
    if irq >= 0 {
        pic_intack(master, irq);
        if irq == 2 {
            irq2 = pic_get_irq(slave);
            if irq2 >= 0 {
                pic_intack(slave, irq2);
            } else {
                // spurious IRQ on slave controller
                irq2 = 7;
            }
            intno = ((*slave).irq_base as c_int) + irq2;
            irq = irq2 + 8;
        } else {
            intno = ((*master).irq_base as c_int) + irq;
        }
    } else {
        // spurious IRQ on host controller
        irq = 7;
        intno = ((*master).irq_base as c_int) + irq;
    }
    pic2_update_irq(s as *mut c_void);

    // `irq` is only used for the (compiled out) DEBUG_PIC trace in C.
    let _ = irq;

    intno
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

// ── `pic2_update_irq(s); return intno;` · x86_machine.c:671-677 · chunk 4053 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pic2_update_irq(s);
//     
//     
//     
//     
//     
//         return intno;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4053 {
// Port of:
//   pic2_update_irq(s);
//   return intno;
// from pic2_get_hard_intno(), plus the static helpers pic2_update_irq needs
// (pic_get_irq, pic_get_priority, pic_set_irq1), which have no linker symbols.

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

#[repr(C)]
pub struct PIC2State {
    pub pics: [*mut PICState; 2],
    pub irq_requested: c_int,
    pub cpu_set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, level: c_int)>,
    pub opaque: *mut c_void,
    pub irqs: *mut c_void,
}

static mut DUMMY: c_int = 0;

unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = (1 as c_int) << (irq & 7);
    if ((*s).elcr as c_int) & mask != 0 {
        if level != 0 {
            (*s).irr |= mask as u8;
            (*s).last_irr |= mask as u8;
        } else {
            (*s).irr &= !(mask as u8);
            (*s).last_irr &= !(mask as u8);
        }
    } else {
        if level != 0 {
            if ((*s).last_irr as c_int) & mask == 0 {
                (*s).irr |= mask as u8;
            }
            (*s).last_irr |= mask as u8;
        } else {
            (*s).last_irr &= !(mask as u8);
        }
    }
}

unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & ((1 as c_int) << (((priority + ((*s).priority_add as c_int)) & 7) as u32))) == 0 {
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

unsafe fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;
    let irq2 = pic_get_irq(*((*s).pics.as_ptr().add(1)));
    if irq2 >= 0 {
        pic_set_irq1(*((*s).pics.as_ptr().add(0)), 2, 1);
        pic_set_irq1(*((*s).pics.as_ptr().add(0)), 2, 0);
    }
    let irq = pic_get_irq(*((*s).pics.as_ptr().add(0)));
    let set = (*s).cpu_set_irq;
    if let Some(f) = set {
        if irq >= 0 {
            f((*s).opaque, 1);
        } else {
            f((*s).opaque, 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn pic2_get_hard_intno__part4053(s: *mut PIC2State, intno: c_int) -> c_int {
    let _ = unsafe { core::ptr::read_volatile(&DUMMY) };
    unsafe {
        if !s.is_null() {
            pic2_update_irq(s as *mut c_void);
        }
    }
    intno
}
}

// ── `pit_init` · x86_machine.c:726-755 · chunk 4059 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PITState *pit_init(PhysMemoryMap *port_map, int addr0, int addr1,
//                        IRQSignal *irq,
//                        PITGetTicksFunc *get_ticks, void *opaque)
//     {
//         PITState *s;
//         PITChannel *pc;
//         int i;
//     
//         s = mallocz(sizeof(*s));
//     
//         s->irq = irq;
//         s->get_ticks = get_ticks;
//         s->opaque = opaque;
//     
//         for(i = 0; i < 3; i++) {
//             pc = &s->pit_channels[i];
//             pc->pit_state = s;
//             pc->mode = 3;
//             pc->gate = (i != 2) >> 0;
//             pit_load_count(pc, 0);
//         }
//         s->speaker_data_on = 0;
//     
//         cpu_register_device(port_map, addr0, 4, s, pit_read, pit_write,
//                             (1 << 0));
//     
//         cpu_register_device(port_map, addr1, 1, s, speaker_read, speaker_write,
//                             (1 << 0));
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4059 {
// Rust rewrite of pit_init (x86_machine.c) plus the private (static) helpers it
// needs: pit_load_count / pit_get_time / pit_get_count / pit_get_out, and the
// device callbacks that pit_init registers (pit_read, pit_write, speaker_read,
// speaker_write). The C versions of those helpers are `static` and therefore
// have no linker symbol, so their logic is ported here as private fns.

use core::ffi::{c_int, c_void};

type PitGetTicksFn = unsafe extern "C" fn(*mut c_void) -> i64;
type DeviceReadFn = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFn = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut c_void,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFn>,
        write_func: Option<DeviceWriteFn>,
        devio_flags: c_int,
    ) -> *mut c_void;
}

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
    get_ticks: Option<PitGetTicksFn>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

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

unsafe fn pit_get_count(pc: *mut PITChannel) -> u32 {
    let d = (pit_get_time(pc) as u64).wrapping_sub((*pc).count_load_time as u64);
    let count = (*pc).count;
    match (*pc).mode {
        0 | 1 | 4 | 5 => ((count as u64).wrapping_sub(d) & 0xffff) as u32,
        _ => {
            if count == 0 {
                0
            } else {
                let c = count as u64;
                c.wrapping_sub(d % c) as u32
            }
        }
    }
}

unsafe fn pit_get_out(pc: *mut PITChannel) -> c_int {
    let d = pit_get_time(pc).wrapping_sub((*pc).count_load_time);
    let count = (*pc).count;
    let c = count as i64;
    match (*pc).mode {
        1 => {
            if d < c {
                1
            } else {
                0
            }
        }
        2 => {
            if c != 0 && (d % c) == 0 && d != 0 {
                1
            } else {
                0
            }
        }
        3 => {
            if c != 0 {
                if (d % c) < ((count >> 1) as i64) {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        }
        4 | 5 => {
            if d == c {
                1
            } else {
                0
            }
        }
        _ => {
            if d >= c {
                1
            } else {
                0
            }
        }
    }
}

unsafe fn pit_load_count(pc: *mut PITChannel, val: c_int) {
    let mut val = val;
    if val == 0 {
        val = 0x10000;
    }
    (*pc).count_load_time = pit_get_time(pc);
    (*pc).last_irq_time = 0;
    (*pc).count = val as u32;
}

unsafe extern "C" fn pit_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let pit = opaque as *mut PITState;
    if pit.is_null() {
        return;
    }
    let addr = offset & 3;
    if addr == 3 {
        let channel = (val >> 6) as usize;
        if channel == 3 || channel > 2 {
            return;
        }
        let pc = (*pit).pit_channels.as_mut_ptr().add(channel);
        let access = (val >> 4) & 3;
        match access {
            0 => {
                (*pc).latched_count = pit_get_count(pc);
                (*pc).rw_state = 4;
            }
            _ => {
                (*pc).mode = ((val >> 1) & 7) as u8;
                (*pc).bcd = (val & 1) as u8;
                (*pc).rw_state = (access - 1) as u8;
            }
        }
    } else {
        let pc = (*pit).pit_channels.as_mut_ptr().add(addr as usize);
        match (*pc).rw_state {
            0 => pit_load_count(pc, val as i32),
            1 => pit_load_count(pc, val.wrapping_shl(8) as i32),
            2 | 3 => {
                if ((*pc).rw_state & 1) != 0 {
                    let nv = ((*pc).latched_count & 0xff) | val.wrapping_shl(8);
                    pit_load_count(pc, nv as i32);
                } else {
                    (*pc).latched_count = val;
                }
                (*pc).rw_state ^= 1;
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn pit_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let pit = opaque as *mut PITState;
    if pit.is_null() {
        return 0;
    }
    let addr = offset & 3;
    if addr == 3 {
        return 0xff;
    }
    let pc = (*pit).pit_channels.as_mut_ptr().add(addr as usize);
    let ret: c_int;
    match (*pc).rw_state {
        0 | 1 | 2 | 3 => {
            let count = pit_get_count(pc) as c_int;
            ret = if ((*pc).rw_state & 1) != 0 {
                (count >> 8) & 0xff
            } else {
                count & 0xff
            };
            if ((*pc).rw_state & 2) != 0 {
                (*pc).rw_state ^= 1;
            }
        }
        _ => {
            ret = if ((*pc).rw_state & 1) != 0 {
                ((*pc).latched_count >> 8) as c_int
            } else {
                ((*pc).latched_count & 0xff) as c_int
            };
            (*pc).rw_state ^= 1;
        }
    }
    ret as u32
}

unsafe extern "C" fn speaker_write(opaque: *mut c_void, _offset: u32, val: u32, _size_log2: c_int) {
    let pit = opaque as *mut PITState;
    if pit.is_null() {
        return;
    }
    (*pit).speaker_data_on = ((val >> 1) & 1) as u8;
    let pc = (*pit).pit_channels.as_mut_ptr().add(2);
    (*pc).gate = (val & 1) as u8;
}

unsafe extern "C" fn speaker_read(opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let pit = opaque as *mut PITState;
    if pit.is_null() {
        return 0;
    }
    let pc = (*pit).pit_channels.as_mut_ptr().add(2);
    let out = pit_get_out(pc);
    let gate = (*pc).gate as c_int;
    let data_on = (*pit).speaker_data_on as c_int;
    let val = (data_on << 1) | gate | (out << 5);
    val as u32
}

#[no_mangle]
pub unsafe extern "C" fn pit_init(
    port_map: *mut c_void,
    addr0: c_int,
    addr1: c_int,
    irq: *mut c_void,
    get_ticks: Option<PitGetTicksFn>,
    opaque: *mut c_void,
) -> *mut PITState {
    let s = mallocz(core::mem::size_of::<PITState>()) as *mut PITState;
    if s.is_null() {
        return s;
    }

    (*s).irq = irq;
    (*s).get_ticks = get_ticks;
    (*s).opaque = opaque;

    let mut i: c_int = 0;
    while i < 3 {
        let pc = (*s).pit_channels.as_mut_ptr().add(i as usize);
        (*pc).pit_state = s;
        (*pc).mode = 3;
        (*pc).gate = (i != 2) as u8;
        pit_load_count(pc, 0);
        i += 1;
    }
    (*s).speaker_data_on = 0;

    cpu_register_device(
        port_map,
        addr0 as i64 as u64,
        4,
        s as *mut c_void,
        Some(pit_read),
        Some(pit_write),
        1 << 0,
    );

    cpu_register_device(
        port_map,
        addr1 as i64 as u64,
        1,
        s as *mut c_void,
        Some(speaker_read),
        Some(speaker_write),
        1 << 0,
    );

    s
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

// ── `for (i = 0; i < 3; i++)` · x86_machine.c:740-746 · chunk 4061 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 3; i++) {
//             pc = &s->pit_channels[i];
//             pc->pit_state = s;
//             pc->mode = 3;
//             pc->gate = (i != 2) >> 0;
//             pit_load_count(pc, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4061 {
// Port of the channel-initialisation loop from pit_init().
// pit_load_count() is a `static` C function with no linker symbol, so its
// logic (and the pit_get_time/pit_get_out style helpers it needs) are
// re-implemented privately here.

use core::ffi::{c_int, c_void};

type PITGetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

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
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

unsafe fn pit_load_count(pc: *mut PITChannel, val: c_int) {
    let mut val = val;
    if val == 0 {
        val = 0x10000;
    }
    (*pc).count_load_time = pit_get_time(pc);
    (*pc).last_irq_time = 0;
    (*pc).count = val as u32;
}

#[no_mangle]
pub unsafe extern "C" fn pit_init__part4061(s: *mut PITState) {
    let mut i: c_int = 0;
    while i < 3 {
        let pc: *mut PITChannel =
            core::ptr::addr_of_mut!((*s).pit_channels) as *mut PITChannel;
        let pc = pc.offset(i as isize);
        (*pc).pit_state = s;
        (*pc).mode = 3;
        (*pc).gate = (i != 2) as u8;
        pit_load_count(pc, 0);
        i += 1;
    }
}
}

// ── `s->speaker_data_on = 0; cpu_register_device(port_map, addr0, 4, s, pit_read, pit_write, (1 << 0…` · x86_machine.c:747-754 · chunk 4062 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->speaker_data_on = 0;
//     
//         cpu_register_device(port_map, addr0, 4, s, pit_read, pit_write,
//                             (1 << 0));
//     
//         cpu_register_device(port_map, addr1, 1, s, speaker_read, speaker_write,
//                             (1 << 0));
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4062 {
// Portion of pit_init: setup of speaker_data_on and port device registration.
use core::ffi::{c_int, c_void};

type DeviceReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
type DeviceWriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;
type PITGetTicksFunc = Option<unsafe extern "C" fn(*mut c_void) -> i64>;

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
    pub get_ticks: PITGetTicksFunc,
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

extern "C" {
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: DeviceReadFunc,
        write_func: DeviceWriteFunc,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn pit_init__part4062(
    port_map: *mut PhysMemoryMap,
    addr0: c_int,
    addr1: c_int,
    s: *mut PITState,
    pit_read: DeviceReadFunc,
    pit_write: DeviceWriteFunc,
    speaker_read: DeviceReadFunc,
    speaker_write: DeviceWriteFunc,
) -> *mut PITState {
    (*s).speaker_data_on = 0;

    cpu_register_device(
        port_map,
        addr0 as i64 as u64,
        4u64,
        s as *mut c_void,
        pit_read,
        pit_write,
        1 as c_int,
    );

    cpu_register_device(
        port_map,
        addr1 as i64 as u64,
        1u64,
        s as *mut c_void,
        speaker_read,
        speaker_write,
        1 as c_int,
    );

    s
}
}

// ── `pit_get_time` · x86_machine.c:758-762 · chunk 4063 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t pit_get_time(PITChannel *pc)
//     {
//         PITState *s = pc->pit_state;
//         return s->get_ticks(s->opaque);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4063 {
// Rust rewrite of static int64_t pit_get_time(PITChannel *pc)

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i64>,
    pub irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
}

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

#[no_mangle]
pub unsafe extern "C" fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s: *mut PITState = (*pc).pit_state;
    let f = match (*s).get_ticks {
        Some(f) => f,
        // A NULL function pointer would crash in C as well; return 0 instead of panicking.
        None => return 0,
    };
    f((*s).opaque)
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

// ── `pit_get_count` · x86_machine.c:764-782 · chunk 4065 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t pit_get_count(PITChannel *pc)
//     {
//         uint32_t counter;
//         uint64_t d;
//     
//         d = pit_get_time(pc) - pc->count_load_time;
//         switch(pc->mode) {
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
//         return counter;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4065 {
// Port of PITChannel / PITState as used by pit_get_count.
use core::ffi::{c_int, c_void};

type PITGetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

#[repr(C)]
pub struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: Option<PITGetTicksFunc>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

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

// ported from the C static function pit_get_time (no linker symbol)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pit_get_count(pc: *mut PITChannel) -> u32 {
    let counter: u32;
    let mut d: u64;

    d = (pit_get_time(pc) as u64).wrapping_sub((*pc).count_load_time as u64);
    let mode: c_int = (*pc).mode as c_int;
    match mode {
        0 | 1 | 4 | 5 => {
            counter = ((*pc).count as u64).wrapping_sub(d) as u32 & 0xffff;
        }
        _ => {
            let count = (*pc).count as u64;
            // count == 0 would be a modulo by zero (UB in C); avoid a panic.
            let rem = if count == 0 { 0 } else { d % count };
            counter = (count.wrapping_sub(rem)) as u32;
        }
    }
    counter
}
}

// ── `uint32_t counter; uint64_t d; d = pit_get_time(pc) - pc->count_load_time;` · x86_machine.c:766-769 · chunk 4066 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t counter;
//         uint64_t d;
//     
//         d = pit_get_time(pc) - pc->count_load_time;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4066 {
// Port of the PIT counter-load computation from pit_get_count().
// pit_get_time() is a static C function without a linker symbol, so its body
// (call through s->get_ticks(s->opaque)) is reproduced here.

use core::ffi::{c_long, c_void};

type PITGetTicksFunc = unsafe extern "C" fn(opaque: *mut c_void) -> c_long;

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
    // padding is implicit; `get_ticks` is naturally aligned afterwards
    get_ticks: Option<PITGetTicksFunc>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

/// d = pit_get_time(pc) - pc->count_load_time;  (result as uint64_t)
#[no_mangle]
pub unsafe extern "C" fn pit_get_count__part4066(pc: *mut PITChannel) -> u64 {
    let pc: *const PITChannel = pc;
    if pc.is_null() {
        return 0;
    }
    // pit_get_time(pc)
    let mut now: c_long = 0;
    let state_ptr = (*pc).pit_state;
    if !state_ptr.is_null() {
        let s: *const PITState = state_ptr;
        match (*s).get_ticks {
            Some(f) => now = f((*s).opaque),
            None => now = 0,
        }
    }
    let t: i64 = now as i64;
    let load: i64 = (*pc).count_load_time;
    t.wrapping_sub(load) as u64
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

// ── `pit_get_out` · x86_machine.c:785-815 · chunk 4069 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pit_get_out(PITChannel *pc)
//     {
//         int out;
//         int64_t d;
//     
//         d = pit_get_time(pc) - pc->count_load_time;
//         switch(pc->mode) {
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
//         return out;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4069 {
// Rust rewrite of pit_get_out (and the static helpers it needs).
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

// Only the prefix of PITState is needed; repr(C) keeps the field offsets.
#[repr(C)]
pub struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: Option<unsafe extern "C" fn(*mut c_void) -> i64>,
    irq: *mut c_void,
    opaque: *mut c_void,
}

// Ported from the static C function pit_get_time (no linker symbol).
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    if pc.is_null() {
        return 0;
    }
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
pub unsafe extern "C" fn pit_get_out(pc: *mut PITChannel) -> core::ffi::c_int {
    let out: core::ffi::c_int;

    let t = pit_get_time(pc);
    let count_load_time = (*pc).count_load_time;
    let d: i64 = t.wrapping_sub(count_load_time);

    let count = (*pc).count;
    let mode = (*pc).mode;

    match mode {
        1 => {
            out = if d < count as i64 { 1 } else { 0 };
        }
        2 => {
            // mode used by Linux
            if count != 0 && d % (count as i64) == 0 && d != 0 {
                out = 1;
            } else {
                out = 0;
            }
        }
        3 => {
            let rem: i64 = if count != 0 { d % (count as i64) } else { 0 };
            out = if rem < (count >> 1) as i64 { 1 } else { 0 };
        }
        4 | 5 => {
            out = if d == count as i64 { 1 } else { 0 };
        }
        // default, case 0
        _ => {
            out = if d >= count as i64 { 1 } else { 0 };
        }
    }
    out
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

// ── `pit_load_count` · x86_machine.c:817-824 · chunk 4073 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pit_load_count(PITChannel *s, int val)
//     {
//         if (val == 0)
//             val = 0x10000;
//         s->count_load_time = pit_get_time(s);
//         s->last_irq_time = 0;
//         s->count = val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4073 {
// Mirror of the C types needed by pit_load_count (x86_machine.c).
use core::ffi::{c_int, c_void};

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// typedef int64_t PITGetTicksFunc(void *opaque);
type PITGetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

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

// Ported from the `static` C helper pit_get_time(), which has no linker symbol.
// Equivalent C:
//     PITState *s = pc->pit_state;
//     return s->get_ticks(s->opaque);
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    let get_ticks = (*s).get_ticks;
    let opaque = (*s).opaque;
    match get_ticks {
        Some(f) => f(opaque),
        // The C code would call a NULL pointer here; keep the same behaviour
        // rather than panicking across the FFI boundary.
        None => core::hint::black_box(0i64),
    }
}

#[no_mangle]
pub unsafe extern "C" fn pit_load_count(s: *mut PITChannel, val: c_int) {
    let mut val = val;
    if val == 0 {
        val = 0x10000;
    }
    (*s).count_load_time = pit_get_time(s);
    (*s).last_irq_time = 0;
    (*s).count = val as u32;
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

// ── `pit_write` · x86_machine.c:826-874 · chunk 4076 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pit_write(void *opaque, uint32_t offset,
//                           uint32_t val, int size_log2)
//     {
//         PITState *pit = opaque;
//         int channel, access, addr;
//         PITChannel *s;
//     
//         addr = offset & 3;
//     
//     
//     
//         if (addr == 3) {
//             channel = val >> 6;
//             if (channel == 3)
//                 return;
//             s = &pit->pit_channels[channel];
//             access = (val >> 4) & 3;
//             switch(access) {
//             case 0:
//                 s->latched_count = pit_get_count(s);
//                 s->rw_state = 4;
//                 break;
//             default:
//                 s->mode = (val >> 1) & 7;
//                 s->bcd = val & 1;
//                 s->rw_state = access - 1 + 0;
//                 break;
//             }
//         } else {
//             s = &pit->pit_channels[addr];
//             switch(s->rw_state) {
//             case 0:
//                 pit_load_count(s, val);
//                 break;
//             case 1:
//                 pit_load_count(s, val << 8);
//                 break;
//             case 2:
//             case 3:
//                 if (s->rw_state & 1) {
//                     pit_load_count(s, (s->latched_count & 0xff) | (val << 8));
//                 } else {
//                     s->latched_count = val;
//                 }
//                 s->rw_state ^= 1;
//                 break;
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4076 {
// Rewrite of static void pit_write(void *opaque, uint32_t offset, uint32_t val, int size_log2)
// from x86_machine.c.  The static helpers pit_get_time / pit_get_count / pit_load_count
// have no linker symbols, so their logic is reproduced here as private Rust fns.

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

// static uint32_t pit_get_count(PITChannel *pc)
unsafe fn pit_get_count(pc: *mut PITChannel) -> u32 {
    let pc = &mut *pc;
    let d: u64 = (pit_get_time(pc as *mut PITChannel).wrapping_sub(pc.count_load_time)) as u64;
    let counter: u32;
    match pc.mode {
        0 | 1 | 4 | 5 => {
            counter = (pc.count as u64).wrapping_sub(d) as u32 & 0xffff;
        }
        _ => {
            // counter = pc->count - (d % pc->count);  (uint64 arithmetic, truncated to uint32)
            counter = if pc.count == 0 {
                pc.count
            } else {
                (pc.count as u64).wrapping_sub(d % (pc.count as u64)) as u32
            };
        }
    }
    counter
}

// static void pit_load_count(PITChannel *s, int val)
unsafe fn pit_load_count(s: *mut PITChannel, val: c_int) {
    let mut val = val;
    if val == 0 {
        val = 0x10000;
    }
    let t = pit_get_time(s);
    (*s).count_load_time = t;
    (*s).last_irq_time = 0;
    (*s).count = val as u32;
}

#[no_mangle]
pub unsafe extern "C" fn pit_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let pit = opaque as *mut PITState;
    let addr: u32 = offset & 3;
    let s: *mut PITChannel;

    if addr == 3 {
        let channel: c_int = (val >> 6) as c_int;
        if channel == 3 {
            return;
        }
        // C indexes pit->pit_channels[channel] directly; mirror the pointer arithmetic
        // so out-of-range values don't trigger a Rust panic.
        s = ((*pit).pit_channels.as_mut_ptr()).add(channel as usize);
        let access: c_int = ((val >> 4) & 3) as c_int;
        match access {
            0 => {
                (*s).latched_count = pit_get_count(s);
                (*s).rw_state = 4;
            }
            _ => {
                (*s).mode = ((val >> 1) & 7) as u8;
                (*s).bcd = (val & 1) as u8;
                (*s).rw_state = (access - 1) as u8;
            }
        }
    } else {
        s = ((*pit).pit_channels.as_mut_ptr()).add(addr as usize);
        match (*s).rw_state {
            0 => {
                pit_load_count(s, val as c_int);
            }
            1 => {
                // val << 8 on a uint32_t, converted to int for the call
                pit_load_count(s, val.wrapping_shl(8) as c_int);
            }
            2 | 3 => {
                if ((*s).rw_state & 1) != 0 {
                    let v: u32 = ((*s).latched_count & 0xff) | val.wrapping_shl(8);
                    pit_load_count(s, v as c_int);
                } else {
                    (*s).latched_count = val;
                }
                (*s).rw_state ^= 1;
            }
            _ => {}
        }
    }
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

// ── `if (addr == 3) … else` · x86_machine.c:837-873 · chunk 4078 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 3) {
//             channel = val >> 6;
//             if (channel == 3)
//                 return;
//             s = &pit->pit_channels[channel];
//             access = (val >> 4) & 3;
//             switch(access) {
//             case 0:
//                 s->latched_count = pit_get_count(s);
//                 s->rw_state = 4;
//                 break;
//             default:
//                 s->mode = (val >> 1) & 7;
//                 s->bcd = val & 1;
//                 s->rw_state = access - 1 + 0;
//                 break;
//             }
//         } else {
//             s = &pit->pit_channels[addr];
//             switch(s->rw_state) {
//             case 0:
//                 pit_load_count(s, val);
//                 break;
//             case 1:
//                 pit_load_count(s, val << 8);
//                 break;
//             case 2:
//             case 3:
//                 if (s->rw_state & 1) {
//                     pit_load_count(s, (s->latched_count & 0xff) | (val << 8));
//                 } else {
//                     s->latched_count = val;
//                 }
//                 s->rw_state ^= 1;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4078 {
#![allow(non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
// typedef int64_t PITGetTicksFunc(void *opaque);
type PITGetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

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
    irq: *mut IRQSignal,
    opaque: *mut c_void,
}

// Port of the static C function pit_get_time(), which has no linker symbol.
#[inline]
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        // The C code always installs get_ticks; calling NULL would be UB there.
        None => 0,
    }
}

// Port of the static C function pit_get_count(), which has no linker symbol.
#[inline]
unsafe fn pit_get_count(pc: *mut PITChannel) -> u32 {
    let d: u64 = pit_get_time(pc).wrapping_sub((*pc).count_load_time) as u64;
    let count: u32 = (*pc).count;
    match (*pc).mode as c_int {
        0 | 1 | 4 | 5 => (((count as u64).wrapping_sub(d)) as u32) & 0xffff,
        _ => {
            let m: u64 = if count == 0 { 0 } else { d % (count as u64) };
            (count as u64).wrapping_sub(m) as u32
        }
    }
}

// Port of the static C function pit_load_count(), which has no linker symbol.
#[inline]
unsafe fn pit_load_count(s: *mut PITChannel, mut val: c_int) {
    if val == 0 {
        val = 0x10000;
    }
    (*s).count_load_time = pit_get_time(s);
    (*s).last_irq_time = 0;
    (*s).count = val as u32;
}

#[no_mangle]
pub unsafe extern "C" fn pit_write__part4078(
    pit: *mut PITState,
    addr: c_int,
    val: u32,
) -> c_int {
    let channels: *mut PITChannel = (*pit).pit_channels.as_mut_ptr();

    if addr == 3 {
        let channel: c_int = (val >> 6) as c_int;
        if channel == 3 {
            // C: return from pit_write()
            return 1;
        }
        let s: *mut PITChannel = channels.wrapping_offset(channel as isize);
        let access: c_int = ((val >> 4) & 3) as c_int;
        if access == 0 {
            (*s).latched_count = pit_get_count(s);
            (*s).rw_state = 4;
        } else {
            (*s).mode = ((val >> 1) & 7) as u8;
            (*s).bcd = (val & 1) as u8;
            (*s).rw_state = (access - 1 + 0) as u8;
        }
    } else {
        let s: *mut PITChannel = channels.wrapping_offset(addr as isize);
        match (*s).rw_state as c_int {
            0 => pit_load_count(s, val as c_int),
            1 => pit_load_count(s, (val << 8) as c_int),
            2 | 3 => {
                if ((*s).rw_state & 1) != 0 {
                    let v: u32 = ((*s).latched_count & 0xff) | (val << 8);
                    pit_load_count(s, v as c_int);
                } else {
                    (*s).latched_count = val;
                }
                (*s).rw_state ^= 1;
            }
            _ => {}
        }
    }
    0
}
}

// ── `pit_read` · x86_machine.c:876-914 · chunk 4079 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t pit_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         PITState *pit = opaque;
//         PITChannel *s;
//         int ret, count, addr;
//     
//         addr = offset & 3;
//         if (addr == 3)
//             return 0xff;
//     
//         s = &pit->pit_channels[addr];
//         switch(s->rw_state) {
//         case 0:
//         case 1:
//         case 2:
//         case 3:
//             count = pit_get_count(s);
//             if (s->rw_state & 1)
//                 ret = (count >> 8) & 0xff;
//             else
//                 ret = count & 0xff;
//             if (s->rw_state & 2)
//                 s->rw_state ^= 1;
//             break;
//         default:
//         case 4:
//         case 5:
//             if (s->rw_state & 1)
//                 ret = s->latched_count >> 8;
//             else
//                 ret = s->latched_count & 0xff;
//             s->rw_state ^= 1;
//             break;
//         }
//     
//     
//     
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4079 {
use core::ffi::{c_int, c_void};
use core::mem;

// Mirror of the C PITChannel struct.
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

// Mirror of the C PITState struct (fields needed by the ported helpers).
#[repr(C)]
struct PITState {
    pit_channels: [PITChannel; 3],
    speaker_data_on: u8,
    get_ticks: *mut c_void,
    irq: *mut c_void,
    opaque: *mut c_void,
}

type GetTicksFunc = unsafe extern "C" fn(*mut c_void) -> i64;

// Ported static helper: pit_get_time
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    if s.is_null() {
        return 0;
    }
    let f: Option<GetTicksFunc> = mem::transmute((*s).get_ticks);
    match f {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

// Ported static helper: pit_get_count
unsafe fn pit_get_count(pc: *mut PITChannel) -> u32 {
    let d = pit_get_time(pc).wrapping_sub((*pc).count_load_time) as u64;
    let pc_count = (*pc).count;
    let mode = (*pc).mode;
    match mode {
        0 | 1 | 4 | 5 => ((pc_count as u64).wrapping_sub(d) as u32) & 0xffff,
        _ => {
            let rem: u64 = if pc_count == 0 {
                0
            } else {
                d % (pc_count as u64)
            };
            (pc_count as u64).wrapping_sub(rem) as u32
        }
    }
}

#[no_mangle]
pub extern "C" fn pit_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    unsafe {
        let pit = opaque as *mut PITState;
        let addr: u32 = offset & 3;

        if addr == 3 {
            return 0xff;
        }

        let s: *mut PITChannel = (*pit).pit_channels.as_mut_ptr().add(addr as usize);
        let ret: c_int;

        let rw_state = (*s).rw_state;
        match rw_state {
            0 | 1 | 2 | 3 => {
                let count: c_int = pit_get_count(s) as c_int;
                if rw_state & 1 != 0 {
                    ret = (count >> 8) & 0xff;
                } else {
                    ret = count & 0xff;
                }
                if rw_state & 2 != 0 {
                    (*s).rw_state ^= 1;
                }
            }
            _ => {
                if rw_state & 1 != 0 {
                    ret = ((*s).latched_count >> 8) as c_int;
                } else {
                    ret = ((*s).latched_count & 0xff) as c_int;
                }
                (*s).rw_state ^= 1;
            }
        }

        ret as u32
    }
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

// ── `switch (s->rw_state)` · x86_machine.c:887-909 · chunk 4083 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->rw_state) {
//         case 0:
//         case 1:
//         case 2:
//         case 3:
//             count = pit_get_count(s);
//             if (s->rw_state & 1)
//                 ret = (count >> 8) & 0xff;
//             else
//                 ret = count & 0xff;
//             if (s->rw_state & 2)
//                 s->rw_state ^= 1;
//             break;
//         default:
//         case 4:
//         case 5:
//             if (s->rw_state & 1)
//                 ret = s->latched_count >> 8;
//             else
//                 ret = s->latched_count & 0xff;
//             s->rw_state ^= 1;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4083 {
#![allow(non_camel_case_types)]
// Port of the switch on s->rw_state inside pit_read() (x86_machine.c 887-909).

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

// static int64_t pit_get_time(PITChannel *pc)
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

// static uint32_t pit_get_count(PITChannel *pc)
unsafe fn pit_get_count(pc: *mut PITChannel) -> u32 {
    // uint64_t d = pit_get_time(pc) - pc->count_load_time;
    let d: u64 = pit_get_time(pc).wrapping_sub((*pc).count_load_time) as u64;
    let cnt: u64 = (*pc).count as u64;
    match (*pc).mode {
        0 | 1 | 4 | 5 => (cnt.wrapping_sub(d) & 0xffff) as u32,
        _ => {
            let rem: u64 = if cnt == 0 { 0 } else { d % cnt };
            cnt.wrapping_sub(rem) as u32
        }
    }
}

#[no_mangle]
pub extern "C" fn pit_read__part4083(s: *mut PITChannel) -> c_int {
    unsafe {
        if s.is_null() {
            return 0;
        }
        let st = (*s).rw_state;
        let ret: u32 = if st <= 3 {
            let count = pit_get_count(s);
            let r: u32 = if st & 1 != 0 {
                (count >> 8) & 0xff
            } else {
                count & 0xff
            };
            if st & 2 != 0 {
                (*s).rw_state = st ^ 1;
            }
            r
        } else {
            let lc = (*s).latched_count;
            let r: u32 = if st & 1 != 0 { lc >> 8 } else { lc & 0xff };
            (*s).rw_state = st ^ 1;
            r
        };
        ret as c_int
    }
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

// ── `speaker_write` · x86_machine.c:916-922 · chunk 4085 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void speaker_write(void *opaque, uint32_t offset,
//                               uint32_t val, int size_log2)
//     {
//         PITState *pit = opaque;
//         pit->speaker_data_on = (val >> 1) & 1;
//         pit->pit_channels[2].gate = val & 1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4085 {
// Rewrite of static void speaker_write(void *opaque, uint32_t offset,
//                                     uint32_t val, int size_log2)
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PITState {
    pub pit_channels: [PITChannel; 3],
    pub speaker_data_on: u8,
    pub get_ticks: *mut c_void,
    pub irq: *mut c_void,
    pub opaque: *mut c_void,
}

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

#[no_mangle]
pub unsafe extern "C" fn speaker_write(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    if opaque.is_null() {
        return;
    }
    let pit = opaque as *mut PITState;
    (*pit).speaker_data_on = ((val >> 1) & 1) as u8;
    let channels = core::ptr::addr_of_mut!((*pit).pit_channels) as *mut PITChannel;
    (*channels.add(2)).gate = (val & 1) as u8;
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

// ── `speaker_read` · x86_machine.c:924-937 · chunk 4087 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t speaker_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         PITState *pit = opaque;
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
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4087 {
// Port of x86_machine.c: speaker_read (and the static helpers it needs:
// pit_get_out / pit_get_time).
use core::ffi::{c_int, c_void};

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
type SetIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>;

// typedef int64_t PITGetTicksFunc(void *opaque);
type PITGetTicksFunc = Option<unsafe extern "C" fn(opaque: *mut c_void) -> i64>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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
    pub get_ticks: PITGetTicksFunc,
    pub irq: *mut IRQSignal,
    pub opaque: *mut c_void,
}

// static int64_t pit_get_time(PITChannel *pc)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s = (*pc).pit_state;
    let f = (*s).get_ticks.unwrap_unchecked();
    f((*s).opaque)
}

// static int pit_get_out(PITChannel *pc)
unsafe fn pit_get_out(pc: *mut PITChannel) -> c_int {
    let d: i64 = pit_get_time(pc).wrapping_sub((*pc).count_load_time);
    let count: i64 = (*pc).count as i64;
    let half: i64 = ((*pc).count >> 1) as i64;
    let mode = (*pc).mode;

    let out: c_int = match mode {
        // default:
        // case 0:
        0 => (d >= count) as c_int,
        1 => (d < count) as c_int,
        2 => {
            // C: if ((d % pc->count) == 0 && d != 0)
            let rem: i64 = if count != 0 { d.wrapping_rem(count) } else { 0 };
            (rem == 0 && d != 0) as c_int
        }
        3 => {
            // C: ((d % pc->count) < (pc->count >> 1))
            let rem: i64 = if count != 0 { d.wrapping_rem(count) } else { 0 };
            (rem < half) as c_int
        }
        4 => (d == count) as c_int,
        5 => (d == count) as c_int,
        _ => (d >= count) as c_int,
    };
    out
}

#[no_mangle]
pub unsafe extern "C" fn speaker_read(opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let pit = opaque as *mut PITState;
    let s: *mut PITChannel = (*pit).pit_channels.as_mut_ptr().add(2);
    let out: c_int = pit_get_out(s);
    let val: c_int = (((*pit).speaker_data_on as c_int) << 1)
        | ((*s).gate as c_int)
        | (out << 5);
    val as u32
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

// ── `pit_update_irq` · x86_machine.c:941-981 · chunk 4089 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pit_update_irq(PITState *pit)
//     {
//         PITChannel *s;
//         int64_t d, delay;
//     
//         s = &pit->pit_channels[0];
//     
//         delay = 1193182;
//     
//         d = pit_get_time(s) - s->count_load_time;
//         switch(s->mode) {
//         default:
//         case 0:
//         case 1:
//         case 4:
//         case 5:
//             if (s->last_irq_time == 0) {
//                 delay = s->count - d;
//                 if (delay <= 0) {
//                     set_irq(pit->irq, 1);
//                     set_irq(pit->irq, 0);
//                     s->last_irq_time = d;
//                 }
//             }
//             break;
//         case 2:
//         case 3:
//             delay = s->last_irq_time + s->count - d;
//             if (delay <= 0) {
//                 set_irq(pit->irq, 1);
//                 set_irq(pit->irq, 0);
//                 s->last_irq_time += s->count;
//             }
//             break;
//         }
//     
//         if (delay <= 0)
//             return 0;
//         else
//             return delay / (1193182 / 1000);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4089 {
// Port of static int pit_update_irq(PITState *pit) from x86_machine.c

use core::ffi::{c_int, c_void};

type SetIRQFunc = extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

type PITGetTicksFunc = extern "C" fn(opaque: *mut c_void) -> i64;

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

const PIT_FREQ: i64 = 1193182;

// static inline void set_irq(IRQSignal *irq, int level)
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// static int64_t pit_get_time(PITChannel *pc)
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
pub unsafe extern "C" fn pit_update_irq(pit: *mut PITState) -> c_int {
    if pit.is_null() {
        return 0;
    }

    let s: *mut PITChannel = core::ptr::addr_of_mut!((*pit).pit_channels).cast::<PITChannel>();

    let mut delay: i64 = PIT_FREQ;

    let d: i64 = pit_get_time(s).wrapping_sub((*s).count_load_time);
    let mode = (*s).mode;
    match mode {
        0 | 1 | 4 | 5 => {
            if (*s).last_irq_time == 0 {
                delay = ((*s).count as i64).wrapping_sub(d);
                if delay <= 0 {
                    set_irq((*pit).irq, 1);
                    set_irq((*pit).irq, 0);
                    (*s).last_irq_time = d;
                }
            }
        }
        2 | 3 => {
            delay = (*s)
                .last_irq_time
                .wrapping_add((*s).count as i64)
                .wrapping_sub(d);
            if delay <= 0 {
                set_irq((*pit).irq, 1);
                set_irq((*pit).irq, 0);
                (*s).last_irq_time = (*s).last_irq_time.wrapping_add((*s).count as i64);
            }
        }
        _ => {}
    }

    if delay <= 0 {
        0
    } else {
        (delay / (PIT_FREQ / 1000)) as c_int
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

// ── `switch (s->mode)` · x86_machine.c:951-975 · chunk 4091 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->mode) {
//         default:
//         case 0:
//         case 1:
//         case 4:
//         case 5:
//             if (s->last_irq_time == 0) {
//                 delay = s->count - d;
//                 if (delay <= 0) {
//                     set_irq(pit->irq, 1);
//                     set_irq(pit->irq, 0);
//                     s->last_irq_time = d;
//                 }
//             }
//             break;
//         case 2:
//         case 3:
//             delay = s->last_irq_time + s->count - d;
//             if (delay <= 0) {
//                 set_irq(pit->irq, 1);
//                 set_irq(pit->irq, 0);
//                 s->last_irq_time += s->count;
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4091 {
// Piece of pit_update_irq: the mode switch that may raise an IRQ.

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

pub type SetIRQFunc = extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Port of the static inline set_irq() from the C translation unit.
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[no_mangle]
pub extern "C" fn pit_update_irq__part4091(
    s: *mut PITChannel,
    d: i64,
    delay: *mut i64,
    irq: *mut IRQSignal,
) {
    unsafe {
        if s.is_null() || delay.is_null() {
            return;
        }
        let mut dl: i64 = *delay;
        let mode = (*s).mode;
        match mode {
            2 | 3 => {
                dl = (*s)
                    .last_irq_time
                    .wrapping_add((*s).count as i64)
                    .wrapping_sub(d);
                if dl <= 0 {
                    set_irq(irq, 1);
                    set_irq(irq, 0);
                    let v = (*s).last_irq_time.wrapping_add((*s).count as i64);
                    (*s).last_irq_time = v;
                }
            }
            _ => {
                if (*s).last_irq_time == 0 {
                    dl = ((*s).count as i64).wrapping_sub(d);
                    if dl <= 0 {
                        set_irq(irq, 1);
                        set_irq(irq, 0);
                        (*s).last_irq_time = d;
                    }
                }
            }
        }
        *delay = dl;
    }
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

// ── `serial_init` · x86_machine.c:1036-1062 · chunk 4095 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     SerialState *serial_init(PhysMemoryMap *port_map, int addr,
//                              IRQSignal *irq,
//                              void (*write_func)(void *opaque, const uint8_t *buf, int buf_len), void *opaque)
//     {
//         SerialState *s;
//         s = mallocz(sizeof(*s));
//     
//     
//         s->divider = 0;
//         s->rbr = 0;
//         s->ier = 0;
//         s->iir = 0x01;
//         s->lcr = 0;
//         s->mcr = 0;
//         s->lsr = 0x40 | 0x20;
//         s->msr = 0;
//         s->scr = 0;
//         s->fcr = 0;
//     
//         s->irq = irq;
//         s->write_func = write_func;
//         s->opaque = opaque;
//     
//         cpu_register_device(port_map, addr, 8, s, serial_read, serial_write,
//                             (1 << 0));
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4095 {
// Rewrite of serial_init() from x86_machine.c (TinyEMU).
//
// serial_read() and serial_write() are `static` in the C translation unit, so
// they have no linker symbol; their logic (and that of the static
// serial_update_irq()) is ported privately here and wired into the device
// registration in place of the original static functions.

use core::ffi::{c_int, c_void};

// Opaque to us: only ever touched through pointers.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);
type SerialWriteFunc = unsafe extern "C" fn(opaque: *mut c_void, buf: *const u8, buf_len: c_int);
type DeviceReadFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32;
type DeviceWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let sig = &*irq;
    if let Some(f) = sig.set_irq {
        f(sig.opaque, sig.irq_num, level);
    }
}

// static void serial_update_irq(SerialState *s)
unsafe fn serial_update_irq(s: *mut SerialState) {
    {
        let st = &mut *s;
        if (st.lsr & 0x01) != 0 && (st.ier & 0x01) != 0 {
            st.iir = 0x04;
        } else if (st.lsr & 0x20) != 0 && (st.ier & 0x02) != 0 {
            st.iir = 0x02;
        } else {
            st.iir = 0x01;
        }
        if st.iir != 0x01 {
            set_irq(st.irq, 1);
        } else {
            set_irq(st.irq, 0);
        }
    }
}

// static void serial_write(void *opaque, uint32_t offset, uint32_t val,
//                          int size_log2)
unsafe extern "C" fn serial_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut SerialState;
    let addr = (offset & 7) as c_int;

    match addr {
        1 => {
            let st = &mut *s;
            if (st.lcr & 0x80) != 0 {
                st.divider = (((st.divider as u32) & 0x00ff) | (val << 8)) as u8;
            } else {
                st.ier = val as u8;
                serial_update_irq(s);
            }
        }
        2 => {}
        3 => {
            (*s).lcr = val as u8;
        }
        4 => {
            (*s).mcr = val as u8;
        }
        5 => {}
        6 => {
            (*s).msr = val as u8;
        }
        7 => {
            (*s).scr = val as u8;
        }
        _ => {
            // case 0, plus the C `default` label which shares this body
            {
                let st = &mut *s;
                if (st.lcr & 0x80) != 0 {
                    st.divider = (((st.divider as u32) & 0xff00) | val) as u8;
                } else {
                    st.lsr &= !0x20u8;
                    serial_update_irq(s);

                    let mut ch: u8 = val as u8;
                    let st2 = &*s;
                    if let Some(f) = st2.write_func {
                        f(st2.opaque, &mut ch as *mut u8 as *const u8, 1);
                    }
                    let st3 = &mut *s;
                    st3.lsr |= 0x20;
                    st3.lsr |= 0x40;
                    serial_update_irq(s);
                }
            }
        }
    }
}

// static uint32_t serial_read(void *opaque, uint32_t offset, int size_log2)
unsafe extern "C" fn serial_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut SerialState;
    let addr = (offset & 7) as c_int;
    let ret: c_int;

    match addr {
        1 => {
            let st = &*s;
            if (st.lcr & 0x80) != 0 {
                ret = (((st.divider as u32) >> 8) & 0xff) as c_int;
            } else {
                ret = st.ier as c_int;
            }
        }
        2 => {
            let st = &*s;
            let mut r = st.iir as c_int;
            if (st.fcr & 0x01) != 0 {
                r |= 0xC0;
            }
            ret = r;
        }
        3 => {
            ret = (*s).lcr as c_int;
        }
        4 => {
            ret = (*s).mcr as c_int;
        }
        5 => {
            ret = (*s).lsr as c_int;
        }
        6 => {
            ret = (*s).msr as c_int;
        }
        7 => {
            ret = (*s).scr as c_int;
        }
        _ => {
            // case 0, plus the C `default` label which shares this body
            let st = &mut *s;
            if (st.lcr & 0x80) != 0 {
                ret = (st.divider as c_int) & 0xff;
            } else {
                ret = st.rbr as c_int;
                st.lsr &= !(0x01u8 | 0x10u8);
                serial_update_irq(s);
            }
        }
    }
    ret as u32
}

#[no_mangle]
pub unsafe extern "C" fn serial_init(
    port_map: *mut PhysMemoryMap,
    addr: c_int,
    irq: *mut IRQSignal,
    write_func: Option<SerialWriteFunc>,
    opaque: *mut c_void,
) -> *mut SerialState {
    let s = mallocz(core::mem::size_of::<SerialState>()) as *mut SerialState;

    {
        let st = &mut *s;
        st.divider = 0;
        st.rbr = 0;
        st.ier = 0;
        st.iir = 0x01;
        st.lcr = 0;
        st.mcr = 0;
        st.lsr = 0x40 | 0x20;
        st.msr = 0;
        st.scr = 0;
        st.fcr = 0;

        st.irq = irq;
        st.write_func = write_func;
        st.opaque = opaque;
    }

    cpu_register_device(
        port_map,
        addr as i64 as u64,
        8,
        s as *mut c_void,
        Some(serial_read as DeviceReadFunc),
        Some(serial_write as DeviceWriteFunc),
        1 << 0, // DEVIO_SIZE8
    );
    s
}
}

// ── `SerialState *s; s = mallocz(sizeof(*s)); s->divider = 0; s->rbr = 0; s->ier = 0; s->iir = 0x01;…` · x86_machine.c:1040-1061 · chunk 4096 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     SerialState *s;
//         s = mallocz(sizeof(*s));
//     
//     
//         s->divider = 0;
//         s->rbr = 0;
//         s->ier = 0;
//         s->iir = 0x01;
//         s->lcr = 0;
//         s->mcr = 0;
//         s->lsr = 0x40 | 0x20;
//         s->msr = 0;
//         s->scr = 0;
//         s->fcr = 0;
//     
//         s->irq = irq;
//         s->write_func = write_func;
//         s->opaque = opaque;
//     
//         cpu_register_device(port_map, addr, 8, s, serial_read, serial_write,
//                             (1 << 0));
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4096 {
// Portion of serial_init: allocation, field initialization and device
// registration for the serial port.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

pub type SerialWriteFunc = Option<extern "C" fn(*mut c_void, *const u8, c_int)>;
pub type DeviceWriteFunc = Option<extern "C" fn(*mut c_void, u32, u32, c_int)>;
pub type DeviceReadFunc = Option<extern "C" fn(*mut c_void, u32, c_int) -> u32>;

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
    pub write_func: SerialWriteFunc,
    pub opaque: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: DeviceReadFunc,
        write_func: DeviceWriteFunc,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub extern "C" fn serial_init__part4096(
    port_map: *mut PhysMemoryMap,
    addr: c_int,
    irq: *mut IRQSignal,
    write_func: SerialWriteFunc,
    opaque: *mut c_void,
    read_func: DeviceReadFunc,
    dev_write_func: DeviceWriteFunc,
) -> *mut SerialState {
    unsafe {
        let s = mallocz(core::mem::size_of::<SerialState>()) as *mut SerialState;

        (*s).divider = 0;
        (*s).rbr = 0;
        (*s).ier = 0;
        (*s).iir = 0x01;
        (*s).lcr = 0;
        (*s).mcr = 0;
        (*s).lsr = 0x40 | 0x20;
        (*s).msr = 0;
        (*s).scr = 0;
        (*s).fcr = 0;

        (*s).irq = irq;
        (*s).write_func = write_func;
        (*s).opaque = opaque;

        cpu_register_device(
            port_map,
            addr as u64,
            8u64,
            s as *mut c_void,
            read_func,
            dev_write_func,
            1 << 0,
        );

        s
    }
}
}

// ── `serial_update_irq` · x86_machine.c:1064-1078 · chunk 4097 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void serial_update_irq(SerialState *s)
//     {
//         if ((s->lsr & 0x01) && (s->ier & 0x01)) {
//             s->iir = 0x04;
//         } else if ((s->lsr & 0x20) && (s->ier & 0x02)) {
//             s->iir = 0x02;
//         } else {
//             s->iir = 0x01;
//         }
//         if (s->iir != 0x01) {
//             set_irq(s->irq, 1);
//         } else {
//             set_irq(s->irq, 0);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4097 {
use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type WriteFunc = unsafe extern "C" fn(*mut c_void, *const u8, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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
    pub write_func: Option<WriteFunc>,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn serial_update_irq(s: *mut SerialState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    if (s.lsr & 0x01) != 0 && (s.ier & 0x01) != 0 {
        s.iir = 0x04;
    } else if (s.lsr & 0x20) != 0 && (s.ier & 0x02) != 0 {
        s.iir = 0x02;
    } else {
        s.iir = 0x01;
    }
    let level: c_int = if s.iir != 0x01 { 1 } else { 0 };
    let irq = s.irq;
    if !irq.is_null() {
        let irq = &*irq;
        if let Some(f) = irq.set_irq {
            f(irq.opaque, irq.irq_num, level);
        }
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

// ── `serial_write` · x86_machine.c:1095-1169 · chunk 4100 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void serial_write(void *opaque, uint32_t offset,
//                              uint32_t val, int size_log2)
//     {
//         SerialState *s = opaque;
//         int addr;
//     
//         addr = offset & 7;
//         switch(addr) {
//         default:
//         case 0:
//             if (s->lcr & 0x80) {
//                 s->divider = (s->divider & 0xff00) | val;
//             } else {
//                 {
//                     uint8_t ch;
//                     s->lsr &= ~0x20;
//                     serial_update_irq(s);
//     
//     
//                     ch = val;
//                     s->write_func(s->opaque, &ch, 1);
//                     s->lsr |= 0x20;
//                     s->lsr |= 0x40;
//                     serial_update_irq(s);
//                 }
//             }
//             break;
//         case 1:
//             if (s->lcr & 0x80) {
//                 s->divider = (s->divider & 0x00ff) | (val << 8);
//             } else {
//                 s->ier = val;
//                 serial_update_irq(s);
//             }
//             break;
//         case 2:
//             break;
//         case 3:
//             s->lcr = val;
//             break;
//         case 4:
//             s->mcr = val;
//             break;
//         case 5:
//             break;
//         case 6:
//             s->msr = val;
//             break;
//         case 7:
//             s->scr = val;
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4100 {
// Port of serial_write from x86_machine.c
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

type WriteFunc = Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>;

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
    irq: *mut IRQSignal,
    write_func: WriteFunc,
    opaque: *mut c_void,
}

// Mirrors the C "static inline void set_irq(IRQSignal *irq, int level)".
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let r = &*irq;
        if let Some(f) = r.set_irq {
            f(r.opaque, r.irq_num, level);
        }
    }
}

// Port of the static C function serial_update_irq.
unsafe fn serial_update_irq(s: *mut SerialState) {
    let st = &mut *s;
    if (st.lsr & 0x01) != 0 && (st.ier & 0x01) != 0 {
        st.iir = 0x04;
    } else if (st.lsr & 0x20) != 0 && (st.ier & 0x02) != 0 {
        st.iir = 0x02;
    } else {
        st.iir = 0x01;
    }
    if st.iir != 0x01 {
        set_irq(st.irq, 1);
    } else {
        set_irq(st.irq, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut SerialState;
    if s.is_null() {
        return;
    }
    let addr: c_int = (offset & 7) as c_int;

    match addr {
        1 => {
            if ((*s).lcr & 0x80) != 0 {
                (*s).divider = (((*s).divider as u32) & 0x00ff | (val << 8)) as u8;
            } else {
                (*s).ier = val as u8;
                serial_update_irq(s);
            }
        }
        2 => {}
        3 => {
            (*s).lcr = val as u8;
        }
        4 => {
            (*s).mcr = val as u8;
        }
        5 => {}
        6 => {
            (*s).msr = val as u8;
        }
        7 => {
            (*s).scr = val as u8;
        }
        // case 0 and default share the following body
        _ => {
            if ((*s).lcr & 0x80) != 0 {
                (*s).divider = (((*s).divider as u32) & 0xff00 | val) as u8;
            } else {
                (*s).lsr &= !0x20u8;
                serial_update_irq(s);

                let ch: u8 = val as u8;
                if let Some(f) = (*s).write_func {
                    f((*s).opaque, &ch, 1);
                }
                (*s).lsr |= 0x20;
                (*s).lsr |= 0x40;
                serial_update_irq(s);
            }
        }
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

// ── `switch (addr)` · x86_machine.c:1102-1168 · chunk 4102 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(addr) {
//         default:
//         case 0:
//             if (s->lcr & 0x80) {
//                 s->divider = (s->divider & 0xff00) | val;
//             } else {
//                 {
//                     uint8_t ch;
//                     s->lsr &= ~0x20;
//                     serial_update_irq(s);
//     
//     
//                     ch = val;
//                     s->write_func(s->opaque, &ch, 1);
//                     s->lsr |= 0x20;
//                     s->lsr |= 0x40;
//                     serial_update_irq(s);
//                 }
//             }
//             break;
//         case 1:
//             if (s->lcr & 0x80) {
//                 s->divider = (s->divider & 0x00ff) | (val << 8);
//             } else {
//                 s->ier = val;
//                 serial_update_irq(s);
//             }
//             break;
//         case 2:
//             break;
//         case 3:
//             s->lcr = val;
//             break;
//         case 4:
//             s->mcr = val;
//             break;
//         case 5:
//             break;
//         case 6:
//             s->msr = val;
//             break;
//         case 7:
//             s->scr = val;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4102 {
// Rust port of the register-decoding switch of serial_write()
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, *const u8, c_int);

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
    pub write_func: Option<DeviceWriteFunc>,
    pub opaque: *mut c_void,
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let f = (*irq).set_irq;
        if let Some(f) = f {
            let opaque = (*irq).opaque;
            let irq_num = (*irq).irq_num;
            f(opaque, irq_num, level);
        }
    }
}

// Ported from the static C function serial_update_irq()
unsafe fn serial_update_irq(s: *mut SerialState) {
    if ((*s).lsr & 0x01) != 0 && ((*s).ier & 0x01) != 0 {
        (*s).iir = 0x04;
    } else if ((*s).lsr & 0x20) != 0 && ((*s).ier & 0x02) != 0 {
        (*s).iir = 0x02;
    } else {
        (*s).iir = 0x01;
    }
    let irq = (*s).irq;
    if (*s).iir != 0x01 {
        set_irq(irq, 1);
    } else {
        set_irq(irq, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_write__part4102(s: *mut SerialState, val: u32, addr: c_int) {
    match addr {
        1 => {
            if ((*s).lcr & 0x80) != 0 {
                (*s).divider = (((*s).divider as u32) & 0x00ff | (val << 8)) as u8;
            } else {
                (*s).ier = val as u8;
                serial_update_irq(s);
            }
        }
        2 => {}
        3 => {
            (*s).lcr = val as u8;
        }
        4 => {
            (*s).mcr = val as u8;
        }
        5 => {}
        6 => {
            (*s).msr = val as u8;
        }
        7 => {
            (*s).scr = val as u8;
        }
        _ => {
            // default: and case 0:
            if ((*s).lcr & 0x80) != 0 {
                (*s).divider = (((*s).divider as u32) & 0xff00 | val) as u8;
            } else {
                (*s).lsr &= !0x20u8;
                serial_update_irq(s);
                let ch: u8 = val as u8;
                if let Some(wf) = (*s).write_func {
                    wf((*s).opaque, &ch as *const u8, 1);
                }
                (*s).lsr |= 0x20;
                (*s).lsr |= 0x40;
                serial_update_irq(s);
            }
        }
    }
}
}

// ── `serial_read` · x86_machine.c:1171-1221 · chunk 4103 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t serial_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         SerialState *s = opaque;
//         int ret, addr;
//     
//         addr = offset & 7;
//         switch(addr) {
//         default:
//         case 0:
//             if (s->lcr & 0x80) {
//                 ret = s->divider & 0xff;
//             } else {
//                 ret = s->rbr;
//                 s->lsr &= ~(0x01 | 0x10);
//                 serial_update_irq(s);
//     
//     
//     
//     
//             }
//             break;
//         case 1:
//             if (s->lcr & 0x80) {
//                 ret = (s->divider >> 8) & 0xff;
//             } else {
//                 ret = s->ier;
//             }
//             break;
//         case 2:
//             ret = s->iir;
//             if (s->fcr & 0x01)
//                 ret |= 0xC0;
//             break;
//         case 3:
//             ret = s->lcr;
//             break;
//         case 4:
//             ret = s->mcr;
//             break;
//         case 5:
//             ret = s->lsr;
//             break;
//         case 6:
//             ret = s->msr;
//             break;
//         case 7:
//             ret = s->scr;
//             break;
//         }
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4103 {
// Rewrite of serial_read from x86_machine.c
use core::ffi::{c_int, c_void};

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct SerialState {
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
    irq: *mut IRQSignal,
    write_func: Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *const u8, buf_len: c_int)>,
    opaque: *mut c_void,
}

const UART_LSR_DR: u8 = 0x01;
const UART_LSR_BI: u8 = 0x10;
const UART_LSR_THRE: u8 = 0x20;
const UART_LCR_DLAB: u8 = 0x80;
const UART_FCR_FE: u8 = 0x01;

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = unsafe { (*irq).set_irq } {
        unsafe { f((*irq).opaque, (*irq).irq_num, level) };
    }
}

// Ported private static helper (no linker symbol in the C program).
unsafe fn serial_update_irq(s: *mut SerialState) {
    let s = unsafe { &mut *s };
    if (s.lsr & UART_LSR_DR) != 0 && (s.ier & 0x01) != 0 {
        s.iir = 0x04;
    } else if (s.lsr & UART_LSR_THRE) != 0 && (s.ier & 0x02) != 0 {
        s.iir = 0x02;
    } else {
        s.iir = 0x01;
    }
    if s.iir != 0x01 {
        unsafe { set_irq(s.irq, 1) };
    } else {
        unsafe { set_irq(s.irq, 0) };
    }
}

#[no_mangle]
pub extern "C" fn serial_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut SerialState;
    let ret: c_int;
    let addr: u32 = offset & 7;

    unsafe {
        match addr {
            1 => {
                if ((*s).lcr & UART_LCR_DLAB) != 0 {
                    ret = (((*s).divider as u32 >> 8) & 0xff) as c_int;
                } else {
                    ret = (*s).ier as c_int;
                }
            }
            2 => {
                let mut r = (*s).iir as c_int;
                if ((*s).fcr & UART_FCR_FE) != 0 {
                    r |= 0xC0;
                }
                ret = r;
            }
            3 => {
                ret = (*s).lcr as c_int;
            }
            4 => {
                ret = (*s).mcr as c_int;
            }
            5 => {
                ret = (*s).lsr as c_int;
            }
            6 => {
                ret = (*s).msr as c_int;
            }
            7 => {
                ret = (*s).scr as c_int;
            }
            // default: and 0
            _ => {
                if ((*s).lcr & UART_LCR_DLAB) != 0 {
                    ret = ((*s).divider & 0xff) as c_int;
                } else {
                    ret = (*s).rbr as c_int;
                    (*s).lsr &= !(UART_LSR_DR | UART_LSR_BI);
                    serial_update_irq(s);
                }
            }
        }
    }
    ret as u32
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

// ── `switch (addr)` · x86_machine.c:1177-1219 · chunk 4105 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch(addr) {
//         default:
//         case 0:
//             if (s->lcr & 0x80) {
//                 ret = s->divider & 0xff;
//             } else {
//                 ret = s->rbr;
//                 s->lsr &= ~(0x01 | 0x10);
//                 serial_update_irq(s);
//     
//     
//     
//     
//             }
//             break;
//         case 1:
//             if (s->lcr & 0x80) {
//                 ret = (s->divider >> 8) & 0xff;
//             } else {
//                 ret = s->ier;
//             }
//             break;
//         case 2:
//             ret = s->iir;
//             if (s->fcr & 0x01)
//                 ret |= 0xC0;
//             break;
//         case 3:
//             ret = s->lcr;
//             break;
//         case 4:
//             ret = s->mcr;
//             break;
//         case 5:
//             ret = s->lsr;
//             break;
//         case 6:
//             ret = s->msr;
//             break;
//         case 7:
//             ret = s->scr;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4105 {
// Port of the address-decode switch inside serial_read() (x86_machine.c).
// serial_update_irq() is a static C function with no linker symbol, so its
// logic is reproduced here as a private Rust helper.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub opaque: *mut c_void,
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

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
pub extern "C" fn serial_read__part4105(s: *mut SerialState, addr: c_int) -> c_int {
    let ret: c_int;
    unsafe {
        match addr {
            1 => {
                if ((*s).lcr & 0x80) != 0 {
                    // C: (s->divider >> 8) & 0xff with divider promoted to int
                    ret = ((*s).divider as c_int) >> 8;
                } else {
                    ret = (*s).ier as c_int;
                }
            }
            2 => {
                let mut r = (*s).iir as c_int;
                if ((*s).fcr & 0x01) != 0 {
                    r |= 0xC0;
                }
                ret = r;
            }
            3 => ret = (*s).lcr as c_int,
            4 => ret = (*s).mcr as c_int,
            5 => ret = (*s).lsr as c_int,
            6 => ret = (*s).msr as c_int,
            7 => ret = (*s).scr as c_int,
            _ => {
                // case 0 and default
                if ((*s).lcr & 0x80) != 0 {
                    ret = ((*s).divider as c_int) & 0xff;
                } else {
                    ret = (*s).rbr as c_int;
                    (*s).lsr &= !(0x01u8 | 0x10u8);
                    serial_update_irq(s);
                }
            }
        }
    }
    ret
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

// ── `serial_send_break` · x86_machine.c:1223-1228 · chunk 4107 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void serial_send_break(SerialState *s)
//     {
//         s->rbr = 0;
//         s->lsr |= 0x10 | 0x01;
//         serial_update_irq(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4107 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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
    pub write_func: *mut c_void,
    pub opaque: *mut c_void,
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// static in the original C: ported logic, no linker symbol
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
pub unsafe extern "C" fn serial_send_break(s: *mut SerialState) {
    (*s).rbr = 0;
    (*s).lsr |= 0x10 | 0x01;
    serial_update_irq(s);
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

// ── `port80_write` · x86_machine.c:1317-1320 · chunk 4111 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void port80_write(void *opaque, uint32_t offset,
//                              uint32_t val64, int size_log2)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4111 {
// Port 0x80 write handler. The original C implementation is intentionally
// empty: writes to the diagnostic port are ignored.
#[no_mangle]
pub extern "C" fn port80_write(
    _opaque: *mut core::ffi::c_void,
    _offset: u32,
    _val64: u32,
    _size_log2: core::ffi::c_int,
) {
}
}

// ── `port80_read` · x86_machine.c:1322-1325 · chunk 4112 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t port80_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         return 0xff;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4112 {
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn port80_read(_opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    0xff
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

// ── `port92_write` · x86_machine.c:1327-1330 · chunk 4114 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void port92_write(void *opaque, uint32_t offset,
//                              uint32_t val, int size_log2)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4114 {
// Port of the (empty) ioport 0x92 write handler from x86_machine.c.
// The C body is empty: writes to port 0x92 are ignored.

use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn port92_write(
    _opaque: *mut c_void,
    _offset: u32,
    _val: u32,
    _size_log2: c_int,
) {
}
}

// ── `port92_read` · x86_machine.c:1332-1336 · chunk 4115 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t port92_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         int a20 = 1;
//         return a20 << 1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4115 {
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn port92_read(_opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    // A20 line is always enabled (A20=0 is not supported).
    let a20: c_int = 1;
    (a20 << 1) as u32
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

// ── `vmport_read` · x86_machine.c:1346-1399 · chunk 4117 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vmport_read(void *opaque, uint32_t addr, int size_log2)
//     {
//         PCMachine *s = opaque;
//         uint32_t regs[6];
//     
//     
//         if (s->kvm_enabled) {
//             struct kvm_regs r;
//     
//             ioctl(s->vcpu_fd, 
//                              (((2U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x81)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                          , &r);
//             regs[0] = r.rax;
//             regs[1] = r.rbx;
//             regs[2] = r.rcx;
//             regs[3] = r.rdx;
//             regs[4] = r.rsi;
//             regs[5] = r.rdi;
//     
//             if (regs[0] == 0x564D5868) {
//     
//                 vmmouse_handler(s->vm_mouse, regs);
//     
//     
//     
//                 r.rax = regs[0];
//                 r.rbx = regs[1];
//                 r.rcx = regs[2];
//                 r.rdx = regs[3];
//                 r.rsi = regs[4];
//                 r.rdi = regs[5];
//                 ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x82)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                              , &r);
//             }
//         } else
//     
//         {
//             regs[0] = x86_cpu_get_reg(s->cpu_state, 0);
//             regs[1] = x86_cpu_get_reg(s->cpu_state, 3);
//             regs[2] = x86_cpu_get_reg(s->cpu_state, 1);
//             regs[3] = x86_cpu_get_reg(s->cpu_state, 2);
//             regs[4] = x86_cpu_get_reg(s->cpu_state, 6);
//             regs[5] = x86_cpu_get_reg(s->cpu_state, 7);
//     
//             if (regs[0] == 0x564D5868) {
//                 vmmouse_handler(s->vm_mouse, regs);
//     
//                 x86_cpu_set_reg(s->cpu_state, 0, regs[0]);
//                 x86_cpu_set_reg(s->cpu_state, 3, regs[1]);
//                 x86_cpu_set_reg(s->cpu_state, 1, regs[2]);
//                 x86_cpu_set_reg(s->cpu_state, 2, regs[3]);
//                 x86_cpu_set_reg(s->cpu_state, 6, regs[4]);
//                 x86_cpu_set_reg(s->cpu_state, 7, regs[5]);
//             }
//         }
//         return regs[0];
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4117 {
// Rewrite of static uint32_t vmport_read(void *opaque, uint32_t addr, int size_log2)

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
struct KvmRegs {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rsp: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rflags: u64,
}

#[repr(C)]
struct VirtMachine {
    vmc: *mut c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct IrqSignal {
    set_irq: *mut c_void,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct PcMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IrqSignal; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn vmmouse_handler(s: *mut c_void, regs: *mut u32);
    fn x86_cpu_get_reg(s: *mut c_void, reg: c_int) -> u32;
    fn x86_cpu_set_reg(s: *mut c_void, reg: c_int, val: u32);
}

const VMPORT_MAGIC: u32 = 0x564D_5868;

const fn ioc(dir: u64, type_: u64, nr: u64, size: u64) -> u64 {
    (dir << 30) | (type_ << 8) | nr | (size << 16)
}

#[no_mangle]
pub extern "C" fn vmport_read(opaque: *mut c_void, _addr: u32, _size_log2: c_int) -> u32 {
    let regs_size = core::mem::size_of::<KvmRegs>() as u64;
    let ioc_get_regs: c_ulong = ioc(2, 0xAE, 0x81, regs_size) as c_ulong;
    let ioc_set_regs: c_ulong = ioc(1, 0xAE, 0x82, regs_size) as c_ulong;

    if opaque.is_null() {
        return 0;
    }
    let s = opaque as *mut PcMachine;
    let mut regs: [u32; 6] = [0u32; 6];

    let kvm_enabled = unsafe { (*s).kvm_enabled };
    if kvm_enabled != 0 {
        let mut r: KvmRegs = unsafe { core::mem::zeroed() };

        unsafe {
            ioctl((*s).vcpu_fd, ioc_get_regs, &mut r as *mut KvmRegs);
        }
        regs[0] = r.rax as u32;
        regs[1] = r.rbx as u32;
        regs[2] = r.rcx as u32;
        regs[3] = r.rdx as u32;
        regs[4] = r.rsi as u32;
        regs[5] = r.rdi as u32;

        if regs[0] == VMPORT_MAGIC {
            unsafe {
                vmmouse_handler((*s).vm_mouse, regs.as_mut_ptr());
            }

            r.rax = regs[0] as u64;
            r.rbx = regs[1] as u64;
            r.rcx = regs[2] as u64;
            r.rdx = regs[3] as u64;
            r.rsi = regs[4] as u64;
            r.rdi = regs[5] as u64;
            unsafe {
                ioctl((*s).vcpu_fd, ioc_set_regs, &mut r as *mut KvmRegs);
            }
        }
    } else {
        unsafe {
            regs[0] = x86_cpu_get_reg((*s).cpu_state, 0);
            regs[1] = x86_cpu_get_reg((*s).cpu_state, 3);
            regs[2] = x86_cpu_get_reg((*s).cpu_state, 1);
            regs[3] = x86_cpu_get_reg((*s).cpu_state, 2);
            regs[4] = x86_cpu_get_reg((*s).cpu_state, 6);
            regs[5] = x86_cpu_get_reg((*s).cpu_state, 7);
        }

        if regs[0] == VMPORT_MAGIC {
            unsafe {
                vmmouse_handler((*s).vm_mouse, regs.as_mut_ptr());

                x86_cpu_set_reg((*s).cpu_state, 0, regs[0]);
                x86_cpu_set_reg((*s).cpu_state, 3, regs[1]);
                x86_cpu_set_reg((*s).cpu_state, 1, regs[2]);
                x86_cpu_set_reg((*s).cpu_state, 2, regs[3]);
                x86_cpu_set_reg((*s).cpu_state, 6, regs[4]);
                x86_cpu_set_reg((*s).cpu_state, 7, regs[5]);
            }
        }
    }
    regs[0]
}
}

// ── `if (s->kvm_enabled) … else` · x86_machine.c:1352-1397 · chunk 4118 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_enabled) {
//             struct kvm_regs r;
//     
//             ioctl(s->vcpu_fd, 
//                              (((2U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x81)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                          , &r);
//             regs[0] = r.rax;
//             regs[1] = r.rbx;
//             regs[2] = r.rcx;
//             regs[3] = r.rdx;
//             regs[4] = r.rsi;
//             regs[5] = r.rdi;
//     
//             if (regs[0] == 0x564D5868) {
//     
//                 vmmouse_handler(s->vm_mouse, regs);
//     
//     
//     
//                 r.rax = regs[0];
//                 r.rbx = regs[1];
//                 r.rcx = regs[2];
//                 r.rdx = regs[3];
//                 r.rsi = regs[4];
//                 r.rdi = regs[5];
//                 ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x82)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                              , &r);
//             }
//         } else
//     
//         {
//             regs[0] = x86_cpu_get_reg(s->cpu_state, 0);
//             regs[1] = x86_cpu_get_reg(s->cpu_state, 3);
//             regs[2] = x86_cpu_get_reg(s->cpu_state, 1);
//             regs[3] = x86_cpu_get_reg(s->cpu_state, 2);
//             regs[4] = x86_cpu_get_reg(s->cpu_state, 6);
//             regs[5] = x86_cpu_get_reg(s->cpu_state, 7);
//     
//             if (regs[0] == 0x564D5868) {
//                 vmmouse_handler(s->vm_mouse, regs);
//     
//                 x86_cpu_set_reg(s->cpu_state, 0, regs[0]);
//                 x86_cpu_set_reg(s->cpu_state, 3, regs[1]);
//                 x86_cpu_set_reg(s->cpu_state, 1, regs[2]);
//                 x86_cpu_set_reg(s->cpu_state, 2, regs[3]);
//                 x86_cpu_set_reg(s->cpu_state, 6, regs[4]);
//                 x86_cpu_set_reg(s->cpu_state, 7, regs[5]);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4118 {
// Port of vmport_read's kvm/non-kvm register transfer block.
use core::ffi::{c_int, c_ulong};
use core::mem::MaybeUninit;

// Mirrors `struct kvm_regs` from the preprocessed translation unit.
#[repr(C)]
pub struct KvmRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[repr(C)]
pub struct X86CPUState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    _opaque: [u8; 0],
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn vmmouse_handler(s: *mut VMMouseState, regs: *mut u32);
    fn x86_cpu_get_reg(s: *mut X86CPUState, reg: c_int) -> u32;
    fn x86_cpu_set_reg(s: *mut X86CPUState, reg: c_int, val: u32);
}

// _IOC(dir, 0xAE, nr, sizeof(struct kvm_regs))
const KVM_GET_REGS: c_ulong =
    (2u64 << 30) | ((0xAEu64) << 8) | 0x81u64 | ((18u64 * 8u64) << 16);
const KVM_SET_REGS: c_ulong =
    (1u64 << 30) | ((0xAEu64) << 8) | 0x82u64 | ((18u64 * 8u64) << 16);

#[no_mangle]
pub unsafe extern "C" fn vmport_read__part4118(
    kvm_enabled: c_int,
    vcpu_fd: c_int,
    cpu_state: *mut X86CPUState,
    vm_mouse: *mut VMMouseState,
    regs: *mut u32,
) {
    if kvm_enabled != 0 {
        let mut r = MaybeUninit::<KvmRegs>::uninit();

        ioctl(vcpu_fd, KVM_GET_REGS, r.as_mut_ptr());
        let rp = r.as_ptr();
        *regs.add(0) = (*rp).rax as u32;
        *regs.add(1) = (*rp).rbx as u32;
        *regs.add(2) = (*rp).rcx as u32;
        *regs.add(3) = (*rp).rdx as u32;
        *regs.add(4) = (*rp).rsi as u32;
        *regs.add(5) = (*rp).rdi as u32;

        if *regs.add(0) == 0x564D5868 {
            vmmouse_handler(vm_mouse, regs);

            let rp = r.as_mut_ptr();
            (*rp).rax = *regs.add(0) as u64;
            (*rp).rbx = *regs.add(1) as u64;
            (*rp).rcx = *regs.add(2) as u64;
            (*rp).rdx = *regs.add(3) as u64;
            (*rp).rsi = *regs.add(4) as u64;
            (*rp).rdi = *regs.add(5) as u64;
            ioctl(vcpu_fd, KVM_SET_REGS, r.as_mut_ptr());
        }
    } else {
        *regs.add(0) = x86_cpu_get_reg(cpu_state, 0);
        *regs.add(1) = x86_cpu_get_reg(cpu_state, 3);
        *regs.add(2) = x86_cpu_get_reg(cpu_state, 1);
        *regs.add(3) = x86_cpu_get_reg(cpu_state, 2);
        *regs.add(4) = x86_cpu_get_reg(cpu_state, 6);
        *regs.add(5) = x86_cpu_get_reg(cpu_state, 7);

        if *regs.add(0) == 0x564D5868 {
            vmmouse_handler(vm_mouse, regs);

            x86_cpu_set_reg(cpu_state, 0, *regs.add(0));
            x86_cpu_set_reg(cpu_state, 3, *regs.add(1));
            x86_cpu_set_reg(cpu_state, 1, *regs.add(2));
            x86_cpu_set_reg(cpu_state, 2, *regs.add(3));
            x86_cpu_set_reg(cpu_state, 6, *regs.add(4));
            x86_cpu_set_reg(cpu_state, 7, *regs.add(5));
        }
    }
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

// ── `vmport_write` · x86_machine.c:1401-1404 · chunk 4120 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vmport_write(void *opaque, uint32_t addr, uint32_t val,
//                              int size_log2)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4120 {
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn vmport_write(
    _opaque: *mut c_void,
    _addr: u32,
    _val: u32,
    _size_log2: c_int,
) {
    // The original C function body is empty: writes to the VMware port are
    // ignored.
}
}

// ── `pic_set_irq_cb` · x86_machine.c:1406-1410 · chunk 4121 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pic_set_irq_cb(void *opaque, int level)
//     {
//         PCMachine *s = opaque;
//         x86_cpu_set_irq(s->cpu_state, level);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4121 {
// Port of static void pic_set_irq_cb(void *opaque, int level)
// from x86_machine.c.

use core::ffi::{c_int, c_void};

/// Opaque X86CPUState (only used behind a pointer).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

/// Mirrors the leading part of `VirtMachine` (all pointer-sized fields).
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

/// Mirrors `PCMachine` up to the `cpu_state` field accessed here.
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut X86CPUState,
}

extern "C" {
    fn x86_cpu_set_irq(s: *mut X86CPUState, set: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pic_set_irq_cb(opaque: *mut c_void, level: c_int) {
    let s = opaque as *mut PCMachine;
    let cpu_state = (*s).cpu_state;
    x86_cpu_set_irq(cpu_state, level);
}
}

// ── `PCMachine *s = opaque; x86_cpu_set_irq(s->cpu_state, level);` · x86_machine.c:1408-1409 · chunk 4122 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         x86_cpu_set_irq(s->cpu_state, level);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4122 {
// Port of a piece of pic_set_irq_cb: read the cpu_state field out of the
// PCMachine pointed to by `opaque` and forward the IRQ level to it.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
}

extern "C" {
    fn x86_cpu_set_irq(s: *mut c_void, set: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pic_set_irq_cb__part4122(opaque: *mut c_void, level: c_int) {
    let s = opaque as *mut PCMachine;
    if s.is_null() {
        return;
    }
    let cpu_state = (*s).cpu_state;
    x86_cpu_set_irq(cpu_state, level);
}
}

// ── `serial_write_cb` · x86_machine.c:1412-1418 · chunk 4123 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void serial_write_cb(void *opaque, const uint8_t *buf, int buf_len)
//     {
//         PCMachine *s = opaque;
//         if (s->common.console) {
//             s->common.console->write_data(s->common.console->opaque, buf, buf_len);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4123 {
// Port of serial_write_cb from x86_machine.c
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

// Only the `common` (VirtMachine) member is used; the remaining fields of
// PCMachine do not matter for this function, but the leading member must
// match exactly.
#[repr(C)]
pub struct PCMachineHead {
    pub common: VirtMachine,
}

#[no_mangle]
pub unsafe extern "C" fn serial_write_cb(opaque: *mut c_void, buf: *const u8, buf_len: c_int) {
    let s = opaque as *mut PCMachineHead;
    if s.is_null() {
        return;
    }
    let console = (*s).common.console;
    if !console.is_null() {
        let cd = &*console;
        if let Some(write_data) = cd.write_data {
            write_data(cd.opaque, buf, buf_len);
        }
    }
}
}

// ── `if (s->common.console)` · x86_machine.c:1415-1417 · chunk 4124 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->common.console) {
//             s->common.console->write_data(s->common.console->opaque, buf, buf_len);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4124 {
// Ported piece of serial_write_cb: forward serial output to the console device.

use core::ffi::{c_int, c_void};

/// Mirrors C `CharacterDevice` (function-pointer based console sink).
#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
}

/// Mirrors C `VirtMachine` (only fields needed for layout fidelity).
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

/// Mirrors C `PCMachine`; `common` is the first member.
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
}

#[no_mangle]
pub unsafe extern "C" fn serial_write_cb__part4124(
    s: *mut PCMachine,
    buf: *const u8,
    buf_len: c_int,
) {
    if s.is_null() {
        return;
    }
    let console: *mut CharacterDevice = (*s).common.console;
    if !console.is_null() {
        let dev = &*console;
        let opaque = dev.opaque;
        if let Some(write_data) = dev.write_data {
            write_data(opaque, buf, buf_len);
        }
    }
}
}

// ── `get_hard_intno_cb` · x86_machine.c:1420-1424 · chunk 4125 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int get_hard_intno_cb(void *opaque)
//     {
//         PCMachine *s = opaque;
//         return pic2_get_hard_intno(s->pic_state);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4125 {
// Rust port of `get_hard_intno_cb` (x86_machine.c).
//
// The C helper `pic2_get_hard_intno` and the PIC helpers it uses are `static`
// functions in the same translation unit, so they have no linker symbols and
// their logic is reproduced here as private Rust functions (semantics kept
// exactly as in the preprocessed C source).

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

// Layout of the C `PICState` (all members, in order).
#[repr(C)]
pub struct PICState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: *mut c_void,
    opaque: *mut c_void,
}

// Layout of the C `PIC2State` (all members, in order).
#[repr(C)]
pub struct PIC2State {
    pics: [*mut PICState; 2],
    irq_requested: c_int,
    cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    opaque: *mut c_void,
    irqs: *mut c_void,
}

// Layout of the C `VirtMachine` (only pointer members).
#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// Layout of the C `PCMachine`, up to and including `pic_state`.
#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut PIC2State,
}

unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask: c_int = 1 << (irq & 31);

    if ((*s).elcr as c_int) & mask != 0 {
        if level != 0 {
            (*s).irr |= mask as u8;
            (*s).last_irr |= mask as u8;
        } else {
            (*s).irr &= !(mask as u8);
            (*s).last_irr &= !(mask as u8);
        }
    } else {
        if level != 0 {
            if ((*s).last_irr as c_int) & mask == 0 {
                (*s).irr |= mask as u8;
            }
            (*s).last_irr |= mask as u8;
        } else {
            (*s).last_irr &= !(mask as u8);
        }
    }
}

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

unsafe fn pic_intack(s: *mut PICState, irq: c_int) {
    if (*s).auto_eoi != 0 {
        if (*s).rotate_on_autoeoi != 0 {
            (*s).priority_add = ((irq + 1) & 7) as u8;
        }
    } else {
        (*s).isr |= (1 << (irq & 31)) as u8;
    }

    let m: c_int = 1 << (irq & 31);
    if ((*s).elcr as c_int) & m == 0 {
        (*s).irr &= !(m as u8);
    }
}

unsafe fn pic2_update_irq(s: *mut PIC2State) {
    let pics0: *mut PICState = (*s).pics[0];
    let pics1: *mut PICState = (*s).pics[1];

    // first look at slave pic
    let irq2 = pic_get_irq(pics1);
    if irq2 >= 0 {
        // if irq request by slave pic, signal master PIC
        pic_set_irq1(pics0, 2, 1);
        pic_set_irq1(pics0, 2, 0);
    }
    // look at requested irq
    let irq = pic_get_irq(pics0);

    if irq >= 0 {
        // raise IRQ request on the CPU
        if let Some(f) = (*s).cpu_set_irq {
            f((*s).opaque, 1);
        }
    } else {
        // lower irq
        if let Some(f) = (*s).cpu_set_irq {
            f((*s).opaque, 0);
        }
    }
}

// Ported from the static C function `pic2_get_hard_intno`.
unsafe fn pic2_get_hard_intno(s: *mut PIC2State) -> c_int {
    let pics0: *mut PICState = (*s).pics[0];
    let pics1: *mut PICState = (*s).pics[1];

    let mut irq: c_int;
    let mut irq2: c_int;
    let intno: c_int;

    irq = pic_get_irq(pics0);
    if irq >= 0 {
        pic_intack(pics0, irq);
        if irq == 2 {
            irq2 = pic_get_irq(pics1);
            if irq2 >= 0 {
                pic_intack(pics1, irq2);
            } else {
                // spurious IRQ on slave controller
                irq2 = 7;
            }
            intno = (*pics1).irq_base as c_int + irq2;
            irq = irq2 + 8;
        } else {
            intno = (*pics0).irq_base as c_int + irq;
        }
    } else {
        // spurious IRQ on host controller
        irq = 7;
        intno = (*pics0).irq_base as c_int + irq;
    }
    pic2_update_irq(s);

    let _ = irq;

    intno
}

#[no_mangle]
pub extern "C" fn get_hard_intno_cb(opaque: *mut c_void) -> c_int {
    unsafe {
        let s: *mut PCMachine = opaque as *mut PCMachine;
        pic2_get_hard_intno((*s).pic_state)
    }
}
}

// ── `PCMachine *s = opaque; return pic2_get_hard_intno(s->pic_state);` · x86_machine.c:1422-1423 · chunk 4126 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         return pic2_get_hard_intno(s->pic_state);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4126 {
// Port of get_hard_intno_cb's body: reads s->pic_state and calls the
// (static, hence non-linkable) pic2_get_hard_intno logic, ported here too
// together with the static helpers it needs.
use core::ffi::{c_int, c_void};

#[repr(C)]
struct PicState {
    last_irr: u8,
    irr: u8,
    imr: u8,
    isr: u8,
    priority_add: u8,
    irq_base: u8,
    read_reg_select: u8,
    special_mask: u8,
    init_state: u8,
    auto_eoi: u8,
    rotate_on_autoeoi: u8,
    init4: u8,
    elcr: u8,
    elcr_mask: u8,
    update_irq: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
}

#[repr(C)]
struct Pic2State {
    pics: [*mut PicState; 2],
    irq_requested: c_int,
    cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    opaque: *mut c_void,
    irqs: *mut c_void,
}

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// Only the leading fields up to pic_state are needed; the layout prefix is
// identical to the C struct.
#[repr(C)]
struct PcMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut Pic2State,
}

unsafe fn pic_get_priority(s: *mut PicState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    let pa = (*s).priority_add as c_int;
    while (mask & (1i32 << (((priority + pa) & 7) as u32))) == 0 {
        priority -= 1;
    }
    priority
}

unsafe fn pic_get_irq(s: *mut PicState) -> c_int {
    let mask: c_int = (*s).irr as c_int & !((*s).imr as c_int);
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

unsafe fn pic_intack(s: *mut PicState, irq: c_int) {
    let bit = 1u32.wrapping_shl(irq as u32);
    if (*s).auto_eoi != 0 {
        if (*s).rotate_on_autoeoi != 0 {
            (*s).priority_add = (((irq + 1) & 7) as u8) as u8;
        }
    } else {
        (*s).isr |= bit as u8;
    }
    if ((*s).elcr as u32 & bit) == 0 {
        (*s).irr &= !(bit as u8);
    }
}

unsafe fn pic_set_irq1(s: *mut PicState, irq: c_int, level: c_int) {
    let mask: c_int = 1i32.wrapping_shl(irq as u32);
    if ((*s).elcr as c_int & mask) != 0 {
        if level != 0 {
            (*s).irr |= mask as u8;
            (*s).last_irr |= mask as u8;
        } else {
            (*s).irr &= !(mask as u8);
            (*s).last_irr &= !(mask as u8);
        }
    } else if level != 0 {
        if ((*s).last_irr as c_int & mask) == 0 {
            (*s).irr |= mask as u8;
        }
        (*s).last_irr |= mask as u8;
    } else {
        (*s).last_irr &= !(mask as u8);
    }
}

unsafe fn pic2_update_irq(s: *mut Pic2State) {
    let irq2 = pic_get_irq((*s).pics[1]);
    if irq2 >= 0 {
        pic_set_irq1((*s).pics[0], 2, 1);
        pic_set_irq1((*s).pics[0], 2, 0);
    }

    let irq = pic_get_irq((*s).pics[0]);

    if let Some(f) = (*s).cpu_set_irq {
        if irq >= 0 {
            f((*s).opaque, 1);
        } else {
            f((*s).opaque, 0);
        }
    }
}

unsafe fn pic2_get_hard_intno(s: *mut Pic2State) -> c_int {
    let mut irq: c_int;
    let mut irq2: c_int;
    let mut intno: c_int;

    irq = pic_get_irq((*s).pics[0]);
    if irq >= 0 {
        pic_intack((*s).pics[0], irq);
        if irq == 2 {
            irq2 = pic_get_irq((*s).pics[1]);
            if irq2 >= 0 {
                pic_intack((*s).pics[1], irq2);
            } else {
                irq2 = 7;
            }
            intno = (*(*s).pics[1]).irq_base as c_int + irq2;
            irq = irq2 + 8;
        } else {
            intno = (*(*s).pics[0]).irq_base as c_int + irq;
        }
    } else {
        irq = 7;
        intno = (*(*s).pics[0]).irq_base as c_int + irq;
    }
    pic2_update_irq(s);

    intno
}

#[no_mangle]
pub unsafe extern "C" fn get_hard_intno_cb__part4126(opaque: *mut c_void) -> c_int {
    let s = opaque as *mut PcMachine;
    pic2_get_hard_intno((*s).pic_state)
}
}

// ── `pit_get_ticks_cb` · x86_machine.c:1426-1433 · chunk 4127 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t pit_get_ticks_cb(void *opaque)
//     {
//         struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 1193182 +
//             ((uint64_t)ts.tv_nsec * 1193182 / 1000000000);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4127 {
// Port of pit_get_ticks_cb from x86_machine.c
use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

const PIT_FREQ: u64 = 1193182;
const CLOCK_MONOTONIC: c_int = 1;

#[no_mangle]
pub extern "C" fn pit_get_ticks_cb(_opaque: *mut c_void) -> i64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);
    }
    (ts.tv_sec as u64)
        .wrapping_mul(PIT_FREQ)
        .wrapping_add(((ts.tv_nsec as u64).wrapping_mul(PIT_FREQ)) / 1000000000) as i64
}
}

// ── `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 1193182 + ((uint64_t)…` · x86_machine.c:1428-1432 · chunk 4128 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 1193182 +
//             ((uint64_t)ts.tv_nsec * 1193182 / 1000000000);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4128 {
use core::ffi::{c_int, c_long};

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn pit_get_ticks_cb__part4128(_opaque: *mut core::ffi::c_void) -> i64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(1, &mut ts as *mut Timespec);
    }
    let sec = ts.tv_sec as u64;
    let nsec = ts.tv_nsec as u64;
    (sec.wrapping_mul(1193182)
        .wrapping_add(nsec.wrapping_mul(1193182) / 1000000000)) as i64
}
}

// ── `PhysMemoryRange *pr; pr = get_phys_mem_range(s->mem_map, paddr);` · x86_machine.c:1439-1440 · chunk 4130 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryRange *pr;
//         pr = get_phys_mem_range(s->mem_map, paddr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4130 {
// Port of the `get_ram_ptr` prologue that localizes the PhysMemoryRange for a
// physical address: it forwards s->mem_map and paddr to get_phys_mem_range().

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    // Opaque: the Rust side never dereferences it, it is only passed back to C.
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

// Prefix of `typedef struct VirtMachine { ... } VirtMachine;`
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Prefix of `typedef struct PCMachine { ... } PCMachine;` up to `mem_map`.
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn get_ram_ptr__part4130(
    s: *mut PCMachine,
    paddr: u64,
) -> *mut PhysMemoryRange {
    let mem_map = (*s).mem_map;
    get_phys_mem_range(mem_map, paddr)
}
}

// ── `if (!pr || !pr->is_ram)` · x86_machine.c:1441-1442 · chunk 4131 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr || !pr->is_ram)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4131 {
// Ported piece of get_ram_ptr(): the "not RAM" early-out test.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

// Returns 1 when the caller must return NULL (!pr || !pr->is_ram), else 0.
#[no_mangle]
pub extern "C" fn get_ram_ptr__part4131(pr: *const PhysMemoryRange) -> c_int {
    unsafe {
        if pr.is_null() || (*pr).is_ram == 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `return pr->phys_mem + (uintptr_t)(paddr - pr->addr);` · x86_machine.c:1443-1443 · chunk 4132 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pr->phys_mem + (uintptr_t)(paddr - pr->addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4132 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut c_void,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: *mut c_void,
    write_func: *mut c_void,
    devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn get_ram_ptr__part4132(
    pr: *mut PhysMemoryRange,
    paddr: u64,
) -> *mut u8 {
    let pr = &*pr;
    let off = paddr.wrapping_sub(pr.addr);
    pr.phys_mem.add(off as usize)
}
}

// ── `st_port` · x86_machine.c:1455-1474 · chunk 4133 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void st_port(void *opaque, uint32_t port, uint32_t val, int size_log2)
//     {
//         PCMachine *s = opaque;
//         PhysMemoryRange *pr;
//     
//     
//     
//     
//         pr = get_phys_mem_range(s->port_map, port);
//         if (!pr) {
//             return;
//         }
//         port -= pr->addr;
//         if ((pr->devio_flags >> size_log2) & 1) {
//             pr->write_func(pr->opaque, port, (uint32_t)val, size_log2);
//         } else if (size_log2 == 1 && (pr->devio_flags & (1 << 0))) {
//             pr->write_func(pr->opaque, port, val & 0xff, 0);
//             pr->write_func(pr->opaque, port + 1, (val >> 8) & 0xff, 0);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4133 {
use core::ffi::{c_int, c_void};

// Mirrors of the C types accessed through pointers.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
}

extern "C" {
    fn get_phys_mem_range(s: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn st_port(opaque: *mut c_void, port: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut PCMachine;
    let pr = get_phys_mem_range((*s).port_map, port as u64);
    if pr.is_null() {
        return;
    }
    let port = port.wrapping_sub((*pr).addr as u32);
    if ((*pr).devio_flags >> (size_log2 as u32)) & 1 != 0 {
        if let Some(f) = (*pr).write_func {
            f((*pr).opaque, port, val, size_log2);
        }
    } else if size_log2 == 1 && ((*pr).devio_flags & (1 << 0)) != 0 {
        if let Some(f) = (*pr).write_func {
            f((*pr).opaque, port, val & 0xff, 0);
            f((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
        }
    }
}
}

// ── `PCMachine *s = opaque; PhysMemoryRange *pr; pr = get_phys_mem_range(s->port_map, port);` · x86_machine.c:1457-1463 · chunk 4134 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         PhysMemoryRange *pr;
//     
//     
//     
//     
//         pr = get_phys_mem_range(s->port_map, port);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4134 {
// Ported part of st_port: lookup of the PhysMemoryRange for a port access.

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    vmc: *const core::ffi::c_void,
    net: *mut core::ffi::c_void,
    console_dev: *mut core::ffi::c_void,
    console: *mut core::ffi::c_void,
    fb_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn st_port__part4134(
    opaque: *mut core::ffi::c_void,
    port: u32,
) -> *mut PhysMemoryRange {
    let s = opaque as *mut PCMachine;
    let port_map = (*s).port_map;
    get_phys_mem_range(port_map, port as u64)
}
}

// ── `if (!pr)` · x86_machine.c:1464-1466 · chunk 4135 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr) {
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4135 {
// Ported piece of st_port: the early-out when no phys memory range matched.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn st_port__part4135(pr: *const c_void) -> c_int {
    if pr.is_null() {
        1
    } else {
        0
    }
}
}

// ── `port -= pr->addr;` · x86_machine.c:1467-1467 · chunk 4136 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     port -= pr->addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4136 {
// Port of the C statement `port -= pr->addr;` from st_port().
// `port` is uint32_t and pr->addr is uint64_t: the subtraction is done in
// 64-bit unsigned arithmetic and the result is truncated back to 32 bits.
use core::ffi::{c_uint, c_ulonglong};

#[no_mangle]
pub extern "C" fn st_port__part4136(port: c_uint, addr: c_ulonglong) -> c_uint {
    (port as u64).wrapping_sub(addr) as c_uint
}
}

// ── `if (pr->devio_flags >> size_log2) & 1) … else` · x86_machine.c:1468-1473 · chunk 4137 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((pr->devio_flags >> size_log2) & 1) {
//             pr->write_func(pr->opaque, port, (uint32_t)val, size_log2);
//         } else if (size_log2 == 1 && (pr->devio_flags & (1 << 0))) {
//             pr->write_func(pr->opaque, port, val & 0xff, 0);
//             pr->write_func(pr->opaque, port + 1, (val >> 8) & 0xff, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4137 {
use core::ffi::{c_int, c_uint, c_void};

type DeviceWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: c_uint, val: c_uint, size_log2: c_int);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn st_port__part4137(
    pr: *mut PhysMemoryRange,
    port: c_uint,
    val: c_uint,
    size_log2: c_int,
) {
    let devio_flags = (*pr).devio_flags;
    let sh = (size_log2 as u32) & 31;
    if (devio_flags >> sh) & 1 != 0 {
        if let Some(write_func) = (*pr).write_func {
            write_func((*pr).opaque, port, val, size_log2);
        }
    } else if size_log2 == 1 && (devio_flags & 1) != 0 {
        if let Some(write_func) = (*pr).write_func {
            write_func((*pr).opaque, port, val & 0xff, 0);
            write_func((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
        }
    }
}
}

// ── `ld_port` · x86_machine.c:1476-1502 · chunk 4138 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t ld_port(void *opaque, uint32_t port1, int size_log2)
//     {
//         PCMachine *s = opaque;
//         PhysMemoryRange *pr;
//         uint32_t val, port;
//     
//         port = port1;
//         pr = get_phys_mem_range(s->port_map, port);
//         if (!pr) {
//             val = -1;
//         } else {
//             port -= pr->addr;
//             if ((pr->devio_flags >> size_log2) & 1) {
//                 val = pr->read_func(pr->opaque, port, size_log2);
//             } else if (size_log2 == 1 && (pr->devio_flags & (1 << 0))) {
//                 val = pr->read_func(pr->opaque, port, 0) & 0xff;
//                 val |= (pr->read_func(pr->opaque, port + 1, 0) & 0xff) << 8;
//             } else {
//                 val = -1;
//             }
//         }
//     
//     
//     
//     
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4138 {
// Port of x86_machine.c: ld_port()
// Reads a value from an I/O port described by the PCMachine's port_map.

use core::ffi::{c_int, c_void};

type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

/// Opaque handle for struct PhysMemoryMap.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

/// Mirror of PhysMemoryRange (dynlink/phys_mem.h style layout).
#[repr(C)]
struct PhysMemoryRange {
    map: *mut PhysMemoryMap,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: Option<DeviceReadFunc>,
    write_func: Option<DeviceWriteFunc>,
    devio_flags: c_int,
}

/// Mirror of the VirtMachine prefix of PCMachine (only pointer/interp-neutral
/// fields matter; all are pointer sized).
#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

/// Mirror of PCMachine, up to and including the fields ld_port() touches.
#[repr(C)]
struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn ld_port(opaque: *mut c_void, port1: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut PCMachine;
    let mut port: u32 = port1;

    let pr = get_phys_mem_range((*s).port_map, port as u64);
    if pr.is_null() {
        return u32::MAX;
    }

    // port -= pr->addr;  (computed modulo 2^32, as in C)
    port = port.wrapping_sub((*pr).addr as u32);

    let flags: c_int = (*pr).devio_flags;
    let shift: u32 = size_log2 as u32;

    if (flags.wrapping_shr(shift)) & 1 != 0 {
        match (*pr).read_func {
            Some(f) => f((*pr).opaque, port, size_log2),
            None => u32::MAX,
        }
    } else if size_log2 == 1 && (flags & 1) != 0 {
        let mut val: u32 = match (*pr).read_func {
            Some(f) => f((*pr).opaque, port, 0) & 0xff,
            None => u32::MAX,
        };
        let hi: u32 = match (*pr).read_func {
            Some(f) => f((*pr).opaque, port.wrapping_add(1), 0) & 0xff,
            None => u32::MAX,
        };
        val |= hi << 8;
        val
    } else {
        u32::MAX
    }
}
}

// ── `PCMachine *s = opaque; PhysMemoryRange *pr; uint32_t val, port; port = port1; pr = get_phys_mem…` · x86_machine.c:1478-1483 · chunk 4139 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         PhysMemoryRange *pr;
//         uint32_t val, port;
//     
//         port = port1;
//         pr = get_phys_mem_range(s->port_map, port);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4139 {
use core::ffi::{c_int, c_void};

type BOOL = c_int;

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Only ever used through a pointer; the fields we need stop at port_map.
#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn ld_port__part4139(
    opaque: *mut c_void,
    port1: u32,
    pr_out: *mut *mut PhysMemoryRange,
    port_out: *mut u32,
) {
    let s = opaque as *mut PCMachine;
    let port: u32 = port1;
    let mut pr: *mut PhysMemoryRange = core::ptr::null_mut();
    if !s.is_null() {
        pr = get_phys_mem_range((*s).port_map, port as u64);
    }
    if !pr_out.is_null() {
        *pr_out = pr;
    }
    if !port_out.is_null() {
        *port_out = port;
    }
}
}

// ── `if (!pr) … else` · x86_machine.c:1484-1496 · chunk 4140 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr) {
//             val = -1;
//         } else {
//             port -= pr->addr;
//             if ((pr->devio_flags >> size_log2) & 1) {
//                 val = pr->read_func(pr->opaque, port, size_log2);
//             } else if (size_log2 == 1 && (pr->devio_flags & (1 << 0))) {
//                 val = pr->read_func(pr->opaque, port, 0) & 0xff;
//                 val |= (pr->read_func(pr->opaque, port + 1, 0) & 0xff) << 8;
//             } else {
//                 val = -1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4140 {
// Port of the ld_port body piece handling the port memory range read.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

pub type DeviceReadFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32>;
pub type DeviceWriteFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int)>;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: DeviceReadFunc,
    pub write_func: DeviceWriteFunc,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ld_port__part4140(
    pr: *mut PhysMemoryRange,
    port: *mut u32,
    size_log2: c_int,
) -> u32 {
    let mut val: u32;
    if pr.is_null() {
        val = -1i32 as u32;
    } else {
        let prr = &mut *pr;
        *port = (*port).wrapping_sub(prr.addr as u32);
        let p = *port;
        let flags = prr.devio_flags;
        if (flags.wrapping_shr(size_log2 as u32) & 1) != 0 {
            val = match prr.read_func {
                Some(f) => f(prr.opaque, p, size_log2),
                None => 0,
            };
        } else if size_log2 == 1 && (flags & 1) != 0 {
            let lo = match prr.read_func {
                Some(f) => f(prr.opaque, p, 0) & 0xff,
                None => 0,
            };
            val = lo;
            let hi = match prr.read_func {
                Some(f) => f(prr.opaque, p.wrapping_add(1), 0) & 0xff,
                None => 0,
            };
            val |= hi << 8;
        } else {
            val = -1i32 as u32;
        }
    }
    val
}
}

// ── `return val;` · x86_machine.c:1501-1501 · chunk 4141 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4141 {
// Returns the loaded port value unchanged.
#[no_mangle]
pub extern "C" fn ld_port__part4141(val: u32) -> u32 {
    val
}
}

// ── `pc_machine_set_defaults` · x86_machine.c:1504-1507 · chunk 4142 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_machine_set_defaults(VirtMachineParams *p)
//     {
//         p->accel_enable = TRUE;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4142 {
use core::ffi::{c_char, c_int};

// Mirrors of the C structs reachable from VirtMachineParams, needed so that
// the offset of `accel_enable` matches the C layout exactly.
#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const core::ffi::c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut core::ffi::c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

const TRUE: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn pc_machine_set_defaults(p: *mut VirtMachineParams) {
    if !p.is_null() {
        (*p).accel_enable = TRUE;
    }
}
}

// ── `p->accel_enable = TRUE;` · x86_machine.c:1506-1506 · chunk 4143 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->accel_enable = TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4143 {
// Rust port of: p->accel_enable = TRUE;  (inside pc_machine_set_defaults)
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

// Mirror of C `VirtMachineParams` (fields in order, same types).
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int, // BOOL
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4], // VM_FILE_COUNT == 4
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_set_defaults__part4143(p: *mut VirtMachineParams) {
    unsafe {
        (*p).accel_enable = 1; // TRUE
    }
}
}

// ── `sigalrm_handler` · x86_machine.c:1511-1513 · chunk 4144 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void sigalrm_handler(int sig)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4144 {
// Port of x86_machine.c sigalrm_handler: an intentionally empty SIGALRM handler.
// The C body is empty; the only effect is having a valid (no-op) signal handler.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn sigalrm_handler(sig: c_int) {
    // Empty handler: nothing to do.
    let _ = sig;
}
}

// ── `kvm_set_cpuid` · x86_machine.c:1518-1546 · chunk 4145 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_set_cpuid(PCMachine *s)
//     {
//         struct kvm_cpuid2 *kvm_cpuid;
//         int n_ent_max, i;
//         struct kvm_cpuid_entry2 *ent;
//     
//         n_ent_max = 128;
//         kvm_cpuid = mallocz(sizeof(struct kvm_cpuid2) + n_ent_max * sizeof(kvm_cpuid->entries[0]));
//     
//         kvm_cpuid->nent = n_ent_max;
//         if (ioctl(s->kvm_fd, 
//                             (((2U|1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x05)) << 0) | ((((sizeof(struct kvm_cpuid2)))) << ((0 +8)+8)))
//                                                    , kvm_cpuid) < 0) {
//             perror("KVM_GET_SUPPORTED_CPUID");
//             exit(1);
//         }
//     
//         for(i = 0; i < kvm_cpuid->nent; i++) {
//             ent = &kvm_cpuid->entries[i];
//     
//             if (ent->function == 1 || ent->function == 0x80000001) {
//                 ent->edx &= ~((1 << 9) | (1 << 22));
//             }
//         }
//     
//         if (ioctl(s->vcpu_fd, 
//                              (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x90)) << 0) | ((((sizeof(struct kvm_cpuid2)))) << ((0 +8)+8)))
//                                            , kvm_cpuid) < 0) {
//             perror("KVM_SET_CPUID2");
//             exit(1);
//         }
//         free(kvm_cpuid);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4145 {
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

const KVM_GET_SUPPORTED_CPUID: c_ulong = ((2u64 | 1u64) << 30)
    | (0xAEu64 << 8)
    | 0x05u64
    | ((core::mem::size_of::<KvmCpuid2>() as u64) << 16);
const KVM_SET_CPUID2: c_ulong = (1u64 << 30)
    | (0xAEu64 << 8)
    | 0x90u64
    | ((core::mem::size_of::<KvmCpuid2>() as u64) << 16);

#[no_mangle]
pub extern "C" fn kvm_set_cpuid(s: *mut PCMachine) {
    unsafe {
        let n_ent_max: c_int = 128;

        let total = core::mem::size_of::<KvmCpuid2>()
            + (n_ent_max as usize) * core::mem::size_of::<KvmCpuidEntry2>();
        let kvm_cpuid = mallocz(total) as *mut KvmCpuid2;

        (*kvm_cpuid).nent = n_ent_max as u32;
        if ioctl((*s).kvm_fd, KVM_GET_SUPPORTED_CPUID, kvm_cpuid) < 0 {
            perror(b"KVM_GET_SUPPORTED_CPUID\0".as_ptr() as *const c_char);
            exit(1);
        }

        let entries = (kvm_cpuid as *mut u8)
            .add(core::mem::size_of::<KvmCpuid2>()) as *mut KvmCpuidEntry2;

        let mut i: c_int = 0;
        while (i as u32) < (*kvm_cpuid).nent {
            let ent = entries.add(i as usize);
            if (*ent).function == 1 || (*ent).function == 0x80000001 {
                (*ent).edx &= !((1u32 << 9) | (1u32 << 22));
            }
            i += 1;
        }

        if ioctl((*s).vcpu_fd, KVM_SET_CPUID2, kvm_cpuid) < 0 {
            perror(b"KVM_SET_CPUID2\0".as_ptr() as *const c_char);
            exit(1);
        }
        free(kvm_cpuid as *mut c_void);
    }
}
}

// ── `struct kvm_cpuid2 *kvm_cpuid; int n_ent_max, i; struct kvm_cpuid_entry2 *ent; n_ent_max = 128;…` · x86_machine.c:1520-1527 · chunk 4146 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct kvm_cpuid2 *kvm_cpuid;
//         int n_ent_max, i;
//         struct kvm_cpuid_entry2 *ent;
//     
//         n_ent_max = 128;
//         kvm_cpuid = mallocz(sizeof(struct kvm_cpuid2) + n_ent_max * sizeof(kvm_cpuid->entries[0]));
//     
//         kvm_cpuid->nent = n_ent_max;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4146 {
// Portion of kvm_set_cpuid: allocate the struct kvm_cpuid2 buffer and set nent.
use core::ffi::{c_int, c_void};

// Mirror of `struct kvm_cpuid2` (flexible array member at offset 8).
#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
}

// Mirror of `struct kvm_cpuid_entry2` (used only for its size).
#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn kvm_set_cpuid__part4146(n_ent_max_out: *mut c_int) -> *mut KvmCpuid2 {
    let n_ent_max: c_int = 128;
    let size = core::mem::size_of::<KvmCpuid2>()
        + (n_ent_max as usize) * core::mem::size_of::<KvmCpuidEntry2>();
    let kvm_cpuid = unsafe { mallocz(size) } as *mut KvmCpuid2;
    if !n_ent_max_out.is_null() {
        unsafe {
            *n_ent_max_out = n_ent_max;
        }
    }
    unsafe {
        (*kvm_cpuid).nent = n_ent_max as u32;
    }
    kvm_cpuid
}
}

// ── `if (ioctl(s->kvm_fd, (((2U|1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x05)) << 0) | (…` · x86_machine.c:1528-1531 · chunk 4147 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->kvm_fd, 
//                             (((2U|1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x05)) << 0) | ((((sizeof(struct kvm_cpuid2)))) << ((0 +8)+8)))
//                                                    , kvm_cpuid) < 0) {
//             perror("KVM_GET_SUPPORTED_CPUID");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4147 {
// Port of the KVM_GET_SUPPORTED_CPUID ioctl block of kvm_set_cpuid().
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn ioctl(__fd: c_int, __request: c_ulong, ...) -> c_int;
    fn perror(__s: *const c_char);
    fn exit(__status: c_int) -> !;
}

// _IOWR(KVMIO, 0x05, struct kvm_cpuid2) with sizeof(struct kvm_cpuid2) == 8
const KVM_GET_SUPPORTED_CPUID: c_ulong =
    ((((2u32 | 1u32) << (((0 + 8) + 8) + 14))
        | ((0xAEu32) << (0 + 8))
        | ((0x05u32) << 0)
        | ((8u32) << ((0 + 8) + 8)))) as c_ulong;

#[no_mangle]
pub extern "C" fn kvm_set_cpuid__part4147(kvm_fd: c_int, kvm_cpuid: *mut c_void) {
    if unsafe { ioctl(kvm_fd, KVM_GET_SUPPORTED_CPUID, kvm_cpuid) } < 0 {
        unsafe {
            perror(b"KVM_GET_SUPPORTED_CPUID\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `for (i = 0; i < kvm_cpuid->nent; i++)` · x86_machine.c:1533-1539 · chunk 4148 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < kvm_cpuid->nent; i++) {
//             ent = &kvm_cpuid->entries[i];
//     
//             if (ent->function == 1 || ent->function == 0x80000001) {
//                 ent->edx &= ~((1 << 9) | (1 << 22));
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4148 {
// Port of the CPUID entry fix-up loop from kvm_set_cpuid (x86_machine.c).
// Clears the "hypervisor present" style bits (bit 9 and bit 22 of EDX) for
// CPUID leaves 1 and 0x80000001.

#[repr(C)]
struct KvmCpuidEntry2 {
    function: u32,
    index: u32,
    flags: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    padding: [u32; 3],
}

#[repr(C)]
struct KvmCpuid2 {
    nent: u32,
    padding: u32,
    // flexible array member: struct kvm_cpuid_entry2 entries[];
}

const ENTRIES_OFFSET: usize = 8; // offsetof(struct kvm_cpuid2, entries)

#[no_mangle]
pub extern "C" fn kvm_set_cpuid__part4148(kvm_cpuid: *mut KvmCpuid2) {
    if kvm_cpuid.is_null() {
        return;
    }
    unsafe {
        let entries = (kvm_cpuid as *mut u8).add(ENTRIES_OFFSET) as *mut KvmCpuidEntry2;
        let mut i: u32 = 0;
        loop {
            let nent = (*kvm_cpuid).nent;
            if i >= nent {
                break;
            }
            let ent = entries.add(i as usize);
            let function = (*ent).function;
            if function == 1 || function == 0x80000001 {
                (*ent).edx &= !((1u32 << 9) | (1u32 << 22));
            }
            i += 1;
        }
    }
}
}

// ── `if (ioctl(s->vcpu_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x90)) << 0) | (((…` · x86_machine.c:1541-1544 · chunk 4149 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vcpu_fd, 
//                              (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x90)) << 0) | ((((sizeof(struct kvm_cpuid2)))) << ((0 +8)+8)))
//                                            , kvm_cpuid) < 0) {
//             perror("KVM_SET_CPUID2");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4149 {
use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
    pub entries: [KvmCpuidEntry2; 0],
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_set_cpuid__part4149(vcpu_fd: c_int, kvm_cpuid: *mut KvmCpuid2) {
    // (((1U) << (((0+8)+8)+14)) | ((0xAE) << (0+8)) | ((0x90) << 0)
    //  | ((sizeof(struct kvm_cpuid2)) << ((0+8)+8)))
    let size: u32 = core::mem::size_of::<KvmCpuid2>() as u32;
    let request: u32 = (1u32 << 30) | (0xAEu32 << 8) | 0x90u32 | (size << 16);
    unsafe {
        if ioctl(vcpu_fd, request as c_ulong, kvm_cpuid) < 0 {
            perror(b"KVM_SET_CPUID2\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `free(kvm_cpuid);` · x86_machine.c:1545-1545 · chunk 4150 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(kvm_cpuid);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4150 {
// Port of the `free(kvm_cpuid);` statement from kvm_set_cpuid().
use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn kvm_set_cpuid__part4150(kvm_cpuid: *mut c_void) {
    unsafe {
        free(kvm_cpuid);
    }
}
}

// ── `kvm_map_ram` · x86_machine.c:1549-1573 · chunk 4151 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_map_ram(PhysMemoryMap *mem_map, PhysMemoryRange *pr)
//     {
//         PCMachine *s = mem_map->opaque;
//         struct kvm_userspace_memory_region region;
//         int flags;
//     
//         region.slot = pr - mem_map->phys_mem_range;
//         flags = 0;
//         if (pr->devram_flags & (1 << 0))
//             flags |= 
//                     (1UL << 1)
//                                     ;
//         if (pr->devram_flags & (1 << 1))
//             flags |= 
//                     (1UL << 0)
//                                            ;
//         region.flags = flags;
//         region.guest_phys_addr = pr->addr;
//         region.memory_size = pr->size;
//     
//     
//     
//     
//         region.userspace_addr = (uintptr_t)pr->phys_mem;
//         if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x46)) << 0) | ((((sizeof(struct kvm_userspace_memory_region)))) << ((0 +8)+8)))
//                                                      , &region) < 0) {
//             perror("KVM_SET_USER_MEMORY_REGION");
//             exit(1);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4151 {
// Rust rewrite of `kvm_map_ram` from x86_machine.c
use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;

// From include/phys_mem.h (preprocessed layout).
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: BOOL,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

const DEVRAM_FLAG_ROM: c_int = 1 << 0;
const DEVRAM_FLAG_DIRTY_BITS: c_int = 1 << 1;
const KVM_MEM_READONLY: c_int = 1 << 1;
const KVM_MEM_LOG_DIRTY_PAGES: c_int = 1 << 0;

const KVM_SET_USER_MEMORY_REGION: u64 = (1u64 << (((0 + 8) + 8) + 14))
    | ((0xAEu64) << (0 + 8))
    | ((0x46u64) << 0)
    | ((core::mem::size_of::<KvmUserspaceMemoryRegion>() as u64) << ((0 + 8) + 8));

extern "C" {
    fn ioctl(fd: c_int, request: u64, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

static KVM_SET_USER_MEMORY_REGION_ERR: &[u8] = b"KVM_SET_USER_MEMORY_REGION\0";

#[no_mangle]
pub extern "C" fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    unsafe {
        let s = (*mem_map).opaque as *mut PCMachine;

        let base = (*mem_map).phys_mem_range.as_ptr();
        let elem_size = core::mem::size_of::<PhysMemoryRange>();
        let slot = ((pr as usize).wrapping_sub(base as usize) / elem_size) as u32;

        let mut flags: c_int = 0;
        if (*pr).devram_flags & DEVRAM_FLAG_ROM != 0 {
            flags |= KVM_MEM_READONLY;
        }
        if (*pr).devram_flags & DEVRAM_FLAG_DIRTY_BITS != 0 {
            flags |= KVM_MEM_LOG_DIRTY_PAGES;
        }

        let region = KvmUserspaceMemoryRegion {
            slot,
            flags: flags as u32,
            guest_phys_addr: (*pr).addr,
            memory_size: (*pr).size,
            userspace_addr: (*pr).phys_mem as usize as u64,
        };

        if ioctl(
            (*s).vm_fd,
            KVM_SET_USER_MEMORY_REGION,
            &region as *const KvmUserspaceMemoryRegion,
        ) < 0
        {
            perror(KVM_SET_USER_MEMORY_REGION_ERR.as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `PCMachine *s = mem_map->opaque; struct kvm_userspace_memory_region region; int flags; region.sl…` · x86_machine.c:1551-1556 · chunk 4152 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = mem_map->opaque;
//         struct kvm_userspace_memory_region region;
//         int flags;
//     
//         region.slot = pr - mem_map->phys_mem_range;
//         flags = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4152 {
// Port of the prologue of kvm_map_ram: fetch the machine pointer, compute the
// KVM memory slot index and initialise the flags word to zero.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

#[no_mangle]
pub extern "C" fn kvm_map_ram__part4152(
    region: *mut KvmUserspaceMemoryRegion,
    flags: *mut c_int,
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) -> *mut c_void {
    unsafe {
        let base = core::ptr::addr_of_mut!((*mem_map).phys_mem_range) as *mut PhysMemoryRange;
        let diff = (pr as isize).wrapping_sub(base as isize);
        let idx = diff / core::mem::size_of::<PhysMemoryRange>() as isize;
        (*region).slot = idx as u32;
        *flags = 0;
        (*mem_map).opaque
    }
}
}

// ── `if (pr->devram_flags & (1 << 0)` · x86_machine.c:1557-1558 · chunk 4153 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->devram_flags & (1 << 0))
//             flags |= 
//                     (1UL << 1)
//                                     ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4153 {
use core::ffi::{c_int, c_void};

// Only ever used through a pointer here; never dereferenced.
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

pub type DeviceReadFunc = unsafe extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32;
pub type DeviceWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int);

// Mirror of the C struct PhysMemoryRange.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn kvm_map_ram__part4153(pr: *mut PhysMemoryRange, flags: *mut c_int) {
    if ((*pr).devram_flags & (1 << 0)) != 0 {
        *flags |= 1i32 << 1;
    }
}
}

// ── `if (pr->devram_flags & (1 << 1)` · x86_machine.c:1559-1560 · chunk 4154 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->devram_flags & (1 << 1))
//             flags |= 
//                     (1UL << 0)
//                                            ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4154 {
use core::ffi::{c_int, c_void};

// Opaque mirror of the C `PhysMemoryMap` (only used through a pointer).
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

// Mirror of the C struct `PhysMemoryRange`, exactly the same fields in order.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn kvm_map_ram__part4154(pr: *mut PhysMemoryRange, flags: *mut c_int) {
    // if (pr->devram_flags & (1 << 1)) flags |= (1UL << 0);
    if (*pr).devram_flags & (1 << 1) != 0 {
        *flags |= 1;
    }
}
}

// ── `region.flags = flags; region.guest_phys_addr = pr->addr; region.memory_size = pr->size; region.…` · x86_machine.c:1561-1568 · chunk 4155 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     region.flags = flags;
//         region.guest_phys_addr = pr->addr;
//         region.memory_size = pr->size;
//     
//     
//     
//     
//         region.userspace_addr = (uintptr_t)pr->phys_mem;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4155 {
// Ported piece of kvm_map_ram: fill in the KVM memory-region descriptor
// from the flags and the PhysMemoryRange.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

// Mirror of C `PhysMemoryRange` (typedef'd anonymous struct).
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn kvm_map_ram__part4155(
    region: *mut KvmUserspaceMemoryRegion,
    flags: c_int,
    pr: *mut PhysMemoryRange,
) {
    let pr_ref = &*pr;
    let region_ref = &mut *region;
    region_ref.flags = flags as u32;
    region_ref.guest_phys_addr = pr_ref.addr;
    region_ref.memory_size = pr_ref.size;
    region_ref.userspace_addr = pr_ref.phys_mem as u64;
}
}

// ── `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x46)) << 0) | ((((s…` · x86_machine.c:1569-1572 · chunk 4156 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x46)) << 0) | ((((sizeof(struct kvm_userspace_memory_region)))) << ((0 +8)+8)))
//                                                      , &region) < 0) {
//             perror("KVM_SET_USER_MEMORY_REGION");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4156 {
// Port of the KVM_SET_USER_MEMORY_REGION call inside kvm_map_ram.
use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

static MSG: &[u8] = b"KVM_SET_USER_MEMORY_REGION\0";

#[no_mangle]
pub unsafe extern "C" fn kvm_map_ram__part4156(
    vm_fd: c_int,
    region: *mut KvmUserspaceMemoryRegion,
) {
    // ((1U) << 30) | (0xAE << 8) | (0x46 << 0) | (sizeof(struct ...) << 16)
    let request: c_ulong = ((1u32 << 30) as c_ulong)
        | ((0xAEu32 << 8) as c_ulong)
        | ((0x46u32) as c_ulong)
        | ((core::mem::size_of::<KvmUserspaceMemoryRegion>() as c_ulong) << 16);

    if ioctl(vm_fd, request, region as *const KvmUserspaceMemoryRegion) < 0 {
        perror(MSG.as_ptr() as *const c_char);
        exit(1);
    }
}
}

// ── `kvm_register_ram` · x86_machine.c:1576-1599 · chunk 4157 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static PhysMemoryRange *kvm_register_ram(PhysMemoryMap *mem_map, uint64_t addr,
//                                              uint64_t size, int devram_flags)
//     {
//         PhysMemoryRange *pr;
//         uint8_t *phys_mem;
//     
//         pr = register_ram_entry(mem_map, addr, size, devram_flags);
//     
//         phys_mem = mmap(
//                        ((void *)0)
//                            , size, 
//                                    0x1 
//                                              | 
//                                                0x2
//                                                          ,
//                         
//                        0x01 
//                                   | 
//                                     0x20
//                                                  , -1, 0);
//         if (!phys_mem)
//             return 
//                   ((void *)0)
//                       ;
//         pr->phys_mem = phys_mem;
//         if (devram_flags & (1 << 1)) {
//             int n_pages = size >> 12;
//             pr->dirty_bits_size = ((n_pages + 63) / 64) * 8;
//             pr->dirty_bits = mallocz(pr->dirty_bits_size);
//         }
//     
//         if (pr->size != 0) {
//             kvm_map_ram(mem_map, pr);
//         }
//         return pr;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4157 {
// Port of `kvm_register_ram` (x86_machine.c). The static helper `kvm_map_ram`
// (no linker symbol in C) is ported here as a private Rust function.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

const DEVRAM_FLAG_DIRTY_BITS: c_int = 1 << 1;

/* KVM_MEM_READONLY / KVM_MEM_LOG_DIRTY_PAGES */
const KVM_MEM_LOG_DIRTY_PAGES: c_int = 1 << 0;
const KVM_MEM_READONLY: c_int = 1 << 1;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;

const PHYS_MEM_RANGE_MAX: usize = 32;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; PHYS_MEM_RANGE_MAX],
    pub register_ram: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange,
    >,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

/* --- PCMachine layout (only needed to reach s->vm_fd) --- */

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IRQSignal; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

#[repr(C)]
struct kvm_userspace_memory_region {
    slot: u32,
    flags: u32,
    guest_phys_addr: u64,
    memory_size: u64,
    userspace_addr: u64,
}

const KVM_SET_USER_MEMORY_REGION: c_ulong = ((1u64 << 30)
    | (0xAEu64 << 8)
    | 0x46u64
    | ((core::mem::size_of::<kvm_userspace_memory_region>() as u64) << 16)) as c_ulong;

extern "C" {
    fn register_ram_entry(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn mallocz(size: usize) -> *mut c_void;
    /* the C code declares mmap() with __asm__("mmap64") */
    fn mmap64(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

/* port of the static C helper kvm_map_ram() */
unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region: kvm_userspace_memory_region = core::mem::zeroed();
    let mut flags: c_int;

    let base = (*mem_map).phys_mem_range.as_ptr();
    let slot_bytes = (pr as usize).wrapping_sub(base as usize);
    region.slot = (slot_bytes / core::mem::size_of::<PhysMemoryRange>()) as u32;

    flags = 0;
    if ((*pr).devram_flags & (1 << 0)) != 0 {
        flags |= KVM_MEM_READONLY;
    }
    if ((*pr).devram_flags & (1 << 1)) != 0 {
        flags |= KVM_MEM_LOG_DIRTY_PAGES;
    }
    region.flags = flags as u32;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as u64;

    if ioctl(
        (*s).vm_fd,
        KVM_SET_USER_MEMORY_REGION,
        &mut region as *mut kvm_userspace_memory_region,
    ) < 0
    {
        perror(b"KVM_SET_USER_MEMORY_REGION\0".as_ptr() as *const c_char);
        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_register_ram(
    mem_map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    let pr: *mut PhysMemoryRange;
    let phys_mem: *mut u8;

    pr = register_ram_entry(mem_map, addr, size, devram_flags);

    phys_mem = mmap64(
        core::ptr::null_mut(),
        size as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut u8;
    if phys_mem.is_null() {
        return core::ptr::null_mut();
    }
    (*pr).phys_mem = phys_mem;
    if (devram_flags & DEVRAM_FLAG_DIRTY_BITS) != 0 {
        let n_pages: c_int = (size >> 12) as c_int;
        (*pr).dirty_bits_size = (n_pages.wrapping_add(63) / 64).wrapping_mul(8);
        (*pr).dirty_bits = mallocz((*pr).dirty_bits_size as usize) as *mut u32;
    }

    if (*pr).size != 0 {
        kvm_map_ram(mem_map, pr);
    }
    pr
}
}

// ── `PhysMemoryRange *pr; uint8_t *phys_mem; pr = register_ram_entry(mem_map, addr, size, devram_fla…` · x86_machine.c:1579-1585 · chunk 4158 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryRange *pr;
//         uint8_t *phys_mem;
//     
//         pr = register_ram_entry(mem_map, addr, size, devram_flags);
//     
//         phys_mem = mmap(
//                        ((void *)0)
//                            , size, 
//                                    0x1 
//                                              | 
//                                                0x2
//                                                          ,
//                         
//                        0x01 
//                                   | 
//                                     0x20
//                                                  , -1, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4158 {
use core::ffi::{c_int, c_void};

// Opaque views of the C types: only pointers are passed through.
#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

extern "C" {
    fn register_ram_entry(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;

    // C's mmap is declared with __asm__("mmap64").
    #[link_name = "mmap64"]
    fn mmap64(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn kvm_register_ram__part4158(
    mem_map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
    phys_mem_out: *mut *mut u8,
) -> *mut PhysMemoryRange {
    unsafe {
        let pr = register_ram_entry(mem_map, addr, size, devram_flags);

        let phys_mem = mmap64(
            core::ptr::null_mut(),
            size as usize,
            0x1 | 0x2,
            0x01 | 0x20,
            -1,
            0,
        ) as *mut u8;

        if !phys_mem_out.is_null() {
            *phys_mem_out = phys_mem;
        }

        pr
    }
}
}

// ── `if (!phys_mem)` · x86_machine.c:1586-1587 · chunk 4159 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!phys_mem)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4159 {
// Piece of kvm_register_ram: check whether mmap() failed.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn kvm_register_ram__part4159(phys_mem: *mut u8) -> c_int {
    if phys_mem.is_null() {
        1
    } else {
        0
    }
}
}

// ── `pr->phys_mem = phys_mem;` · x86_machine.c:1588-1588 · chunk 4160 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pr->phys_mem = phys_mem;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4160 {
use core::ffi::{c_int, c_void};

// Mirror of the C PhysMemoryRange struct (field order and types exactly).
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn kvm_register_ram__part4160(
    pr: *mut PhysMemoryRange,
    phys_mem: *mut u8,
) {
    if !pr.is_null() {
        (*pr).phys_mem = phys_mem;
    }
}
}

// ── `if (devram_flags & (1 << 1)` · x86_machine.c:1589-1593 · chunk 4161 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (devram_flags & (1 << 1)) {
//             int n_pages = size >> 12;
//             pr->dirty_bits_size = ((n_pages + 63) / 64) * 8;
//             pr->dirty_bits = mallocz(pr->dirty_bits_size);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4161 {
// Port of the devram dirty-bit allocation block inside kvm_register_ram.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn kvm_register_ram__part4161(
    pr: *mut PhysMemoryRange,
    size: u64,
    devram_flags: c_int,
) {
    unsafe {
        if devram_flags & (1 << 1) != 0 {
            // int n_pages = size >> 12;
            let n_pages: c_int = (size >> 12) as c_int;
            // pr->dirty_bits_size = ((n_pages + 63) / 64) * 8;
            let bits_size: c_int = n_pages.wrapping_add(63) / 64;
            let bits_size: c_int = bits_size.wrapping_mul(8);
            (*pr).dirty_bits_size = bits_size;
            // pr->dirty_bits = mallocz(pr->dirty_bits_size);
            (*pr).dirty_bits = mallocz((*pr).dirty_bits_size as usize) as *mut u32;
        }
    }
}
}

// ── `if (pr->size != 0)` · x86_machine.c:1595-1597 · chunk 4162 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->size != 0) {
//             kvm_map_ram(mem_map, pr);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4162 {
// Port of the `if (pr->size != 0) kvm_map_ram(mem_map, pr);` piece of
// kvm_register_ram.  kvm_map_ram is a static C function (no linker symbol),
// so its logic is reproduced here as a private Rust function.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type DeviceReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
type DeviceWriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;
type SetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: DeviceReadFunc,
    pub write_func: DeviceWriteFunc,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Only the layout matters here; the referenced device types are opaque.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

// Faithful copy of the static C function kvm_map_ram().
unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region: KvmUserspaceMemoryRegion = core::mem::zeroed();
    let mut flags: c_int;

    let base = core::ptr::addr_of!((*mem_map).phys_mem_range) as *const PhysMemoryRange;
    let diff = (pr as isize).wrapping_sub(base as isize);
    region.slot = (diff / core::mem::size_of::<PhysMemoryRange>() as isize) as u32;

    flags = 0;
    if ((*pr).devram_flags & (1 << 0)) != 0 {
        flags |= 1i32 << 1;
    }
    if ((*pr).devram_flags & (1 << 1)) != 0 {
        flags |= 1i32 << 0;
    }
    region.flags = flags as u32;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as usize as u64;

    let req: c_ulong = (((1u32 << 30)
        | ((0xAEu32) << 8)
        | (0x46u32)
        | ((core::mem::size_of::<KvmUserspaceMemoryRegion>() as u32) << 16))) as c_ulong;

    if ioctl(
        (*s).vm_fd,
        req,
        &mut region as *mut KvmUserspaceMemoryRegion,
    ) < 0
    {
        perror(b"KVM_SET_USER_MEMORY_REGION\0".as_ptr() as *const c_char);
        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_register_ram__part4162(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) {
    if (*pr).size != 0 {
        kvm_map_ram(mem_map, pr);
    }
}
}

// ── `return pr;` · x86_machine.c:1598-1598 · chunk 4163 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4163 {
// Port of the `return pr;` tail of kvm_register_ram (x86_machine.c).
// Returns the PhysMemoryRange pointer it was given, untouched.

use core::ffi::{c_int, c_void};

// Opaque mirror of `typedef struct PhysMemoryMap PhysMemoryMap;`.
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut DeviceReadFunc,
    pub write_func: *mut DeviceWriteFunc,
    pub devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn kvm_register_ram__part4163(
    pr: *mut PhysMemoryRange,
) -> *mut PhysMemoryRange {
    pr
}
}

// ── `kvm_set_ram_addr` · x86_machine.c:1601-1619 · chunk 4164 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_set_ram_addr(PhysMemoryMap *mem_map,
//                                  PhysMemoryRange *pr, uint64_t addr, BOOL enabled)
//     {
//         if (enabled) {
//             if (pr->size == 0 || addr != pr->addr) {
//     
//                 pr->size = pr->org_size;
//                 pr->addr = addr;
//                 kvm_map_ram(mem_map, pr);
//             }
//         } else {
//             if (pr->size != 0) {
//                 pr->addr = 0;
//                 pr->size = 0;
//     
//                 kvm_map_ram(mem_map, pr);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4164 {
// Rust rewrite of the static C function kvm_set_ram_addr (plus its private
// helper kvm_map_ram, which has no linker symbol in the C program).

use core::ffi::{c_int, c_ulong, c_void};

const KVM_SET_USER_MEMORY_REGION: c_ulong = 0x4020AE46;

type SetIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>;
type DeviceReadFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32>;
type DeviceWriteFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,

    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,

    pub opaque: *mut c_void,
    pub read_func: DeviceReadFunc,
    pub write_func: DeviceWriteFunc,
    pub devio_flags: c_int,
}

type RegisterRamFunc =
    Option<unsafe extern "C" fn(s: *mut PhysMemoryMap, addr: u64, size: u64, devram_flags: c_int) -> *mut PhysMemoryRange>;
type FreeRamFunc = Option<unsafe extern "C" fn(s: *mut PhysMemoryMap, pr: *mut PhysMemoryRange)>;
type GetDirtyBitsFunc =
    Option<unsafe extern "C" fn(s: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) -> *const u32>;
type SetRamAddrFunc =
    Option<unsafe extern "C" fn(s: *mut PhysMemoryMap, pr: *mut PhysMemoryRange, addr: u64, enabled: c_int)>;
type FlushTlbWriteRangeFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, ram_addr: *mut u8, ram_size: usize)>;

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: RegisterRamFunc,
    pub free_ram: FreeRamFunc,
    pub get_dirty_bits: GetDirtyBitsFunc,
    pub set_ram_addr: SetRamAddrFunc,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: FlushTlbWriteRangeFunc,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,

    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,

    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const core::ffi::c_char);
    fn exit(status: c_int) -> !;
}

unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region = KvmUserspaceMemoryRegion {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0,
        memory_size: 0,
        userspace_addr: 0,
    };
    let mut flags: u64 = 0;

    let base = (&(*mem_map).phys_mem_range) as *const [PhysMemoryRange; 32] as *const PhysMemoryRange;
    let off = pr.offset_from(base);
    region.slot = off as u32;

    if (*pr).devram_flags & (1 << 0) != 0 {
        flags |= 1u64 << 1;
    }
    if (*pr).devram_flags & (1 << 1) != 0 {
        flags |= 1u64 << 0;
    }
    region.flags = flags as u32;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as usize as u64;

    if ioctl(
        (*s).vm_fd,
        KVM_SET_USER_MEMORY_REGION,
        &mut region as *mut KvmUserspaceMemoryRegion,
    ) < 0
    {
        perror(b"KVM_SET_USER_MEMORY_REGION\0".as_ptr() as *const core::ffi::c_char);
        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_set_ram_addr(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: c_int,
) {
    if enabled != 0 {
        if (*pr).size == 0 || addr != (*pr).addr {
            // move or create the region
            (*pr).size = (*pr).org_size;
            (*pr).addr = addr;
            kvm_map_ram(mem_map, pr);
        }
    } else if (*pr).size != 0 {
        (*pr).addr = 0;
        (*pr).size = 0;
        // map a zero size region to disable
        kvm_map_ram(mem_map, pr);
    }
}
}

// ── `if (enabled) … else` · x86_machine.c:1604-1618 · chunk 4165 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (enabled) {
//             if (pr->size == 0 || addr != pr->addr) {
//     
//                 pr->size = pr->org_size;
//                 pr->addr = addr;
//                 kvm_map_ram(mem_map, pr);
//             }
//         } else {
//             if (pr->size != 0) {
//                 pr->addr = 0;
//                 pr->size = 0;
//     
//                 kvm_map_ram(mem_map, pr);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4165 {
// Port of the body of kvm_set_ram_addr (x86_machine.c) plus the static
// kvm_map_ram helper it calls (which has no linker symbol).
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const KVM_MEM_READONLY: c_ulong = 1 << 1;
const KVM_MEM_LOG_DIRTY_PAGES: c_ulong = 1 << 0;
const DEVRAM_FLAG_ROM: c_int = 1 << 0;
const DEVRAM_FLAG_DIRTY_BITS: c_int = 1 << 1;

// _IOW(KVMIO, 0x46, struct kvm_userspace_memory_region) with sizeof == 32
const KVM_SET_USER_MEMORY_REGION: c_ulong =
    (1u32 << 30) as c_ulong | ((0xAEu32) << 8) as c_ulong | (0x46u32) as c_ulong
        | ((32u32) << 16) as c_ulong;

#[repr(C)]
struct PhysMemoryRange {
    map: *mut PhysMemoryMap,
    addr: c_ulong,
    org_size: c_ulong,
    size: c_ulong,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut c_uint,
    dirty_bits_tab: [*mut c_uint; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: *const c_void,
    write_func: *const c_void,
    devio_flags: c_int,
}

#[repr(C)]
struct PhysMemoryMap {
    n_phys_mem_range: c_int,
    phys_mem_range: [PhysMemoryRange; 32],
    register_ram: *const c_void,
    free_ram: *const c_void,
    get_dirty_bits: *const c_void,
    set_ram_addr: *const c_void,
    opaque: *mut c_void,
    flush_tlb_write_range: *const c_void,
}

#[repr(C)]
struct IRQSignal {
    set_irq: *const c_void,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct PCMachine {
    common: VirtMachine,
    ram_size: c_ulong,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IRQSignal; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

#[repr(C)]
struct KvmUserspaceMemoryRegion {
    slot: c_uint,
    flags: c_uint,
    guest_phys_addr: c_ulong,
    memory_size: c_ulong,
    userspace_addr: c_ulong,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

// static void kvm_map_ram(PhysMemoryMap *mem_map, PhysMemoryRange *pr)
unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region = KvmUserspaceMemoryRegion {
        slot: 0,
        flags: 0,
        guest_phys_addr: 0,
        memory_size: 0,
        userspace_addr: 0,
    };
    let mut flags: c_int = 0;

    let base = (*mem_map).phys_mem_range.as_ptr();
    let slot = (pr as usize).wrapping_sub(base as usize) / core::mem::size_of::<PhysMemoryRange>();
    region.slot = slot as c_uint;

    if (*pr).devram_flags & DEVRAM_FLAG_ROM != 0 {
        flags |= KVM_MEM_READONLY as c_int;
    }
    if (*pr).devram_flags & DEVRAM_FLAG_DIRTY_BITS != 0 {
        flags |= KVM_MEM_LOG_DIRTY_PAGES as c_int;
    }
    region.flags = flags as c_uint;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as usize as c_ulong;

    if ioctl(
        (*s).vm_fd,
        KVM_SET_USER_MEMORY_REGION,
        &mut region as *mut KvmUserspaceMemoryRegion,
    ) < 0
    {
        perror(b"KVM_SET_USER_MEMORY_REGION\0".as_ptr() as *const c_char);
        exit(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_set_ram_addr__part4165(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: c_ulong,
    enabled: c_int,
) {
    if enabled != 0 {
        if (*pr).size == 0 || addr != (*pr).addr {
            // move or create the region
            (*pr).size = (*pr).org_size;
            (*pr).addr = addr;
            kvm_map_ram(mem_map, pr);
        }
    } else if (*pr).size != 0 {
        (*pr).addr = 0;
        (*pr).size = 0;
        // map a zero size region to disable
        kvm_map_ram(mem_map, pr);
    }
}
}

// ── `kvm_get_dirty_bits` · x86_machine.c:1621-1639 · chunk 4166 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static const uint32_t *kvm_get_dirty_bits(PhysMemoryMap *mem_map,
//                                               PhysMemoryRange *pr)
//     {
//         PCMachine *s = mem_map->opaque;
//         struct kvm_dirty_log dlog;
//     
//         if (pr->size == 0) {
//     
//             memset(pr->dirty_bits, 0, pr->dirty_bits_size);
//         } else {
//             dlog.slot = pr - mem_map->phys_mem_range;
//             dlog.dirty_bitmap = pr->dirty_bits;
//             if (ioctl(s->vm_fd, 
//                                (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x42)) << 0) | ((((sizeof(struct kvm_dirty_log)))) << ((0 +8)+8)))
//                                                 , &dlog) < 0) {
//                 perror("KVM_GET_DIRTY_LOG");
//                 exit(1);
//             }
//         }
//         return pr->dirty_bits;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4166 {
// Rust rewrite of kvm_get_dirty_bits() from x86_machine.c
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

#[repr(C)]
pub struct KvmDirtyLog {
    pub slot: u32,
    pub padding1: u32,
    pub dirty_bitmap: *mut c_void,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(code: c_int) -> !;
}

// (((1U) << (((0+8)+8)+14)) | ((0xAE) << (0+8)) | ((0x42) << 0) |
//  ((sizeof(struct kvm_dirty_log)) << ((0+8)+8)))
const KVM_GET_DIRTY_LOG: c_ulong = ((1u32 << (((0u32 + 8) + 8) + 14))
    | (0xAEu32 << (0 + 8))
    | (0x42u32 << 0)
    | ((core::mem::size_of::<KvmDirtyLog>() as u32) << ((0 + 8) + 8))) as c_ulong;

#[no_mangle]
pub unsafe extern "C" fn kvm_get_dirty_bits(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) -> *const u32 {
    let s = (*mem_map).opaque as *mut PCMachine;

    if (*pr).size == 0 {
        // not mapped: we assume no modification was made
        memset(
            (*pr).dirty_bits as *mut c_void,
            0,
            (*pr).dirty_bits_size as usize,
        );
    } else {
        let mut dlog: KvmDirtyLog = KvmDirtyLog {
            slot: 0,
            padding1: 0,
            dirty_bitmap: core::ptr::null_mut(),
        };
        let base = core::ptr::addr_of_mut!((*mem_map).phys_mem_range) as *const PhysMemoryRange;
        let byte_off = (pr as usize).wrapping_sub(base as usize);
        let index = byte_off / core::mem::size_of::<PhysMemoryRange>();
        dlog.slot = index as u32;
        dlog.dirty_bitmap = (*pr).dirty_bits as *mut c_void;
        if ioctl(
            (*s).vm_fd,
            KVM_GET_DIRTY_LOG,
            &mut dlog as *mut KvmDirtyLog,
        ) < 0
        {
            perror(b"KVM_GET_DIRTY_LOG\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
    (*pr).dirty_bits
}
}

// ── `if (pr->size == 0) … else` · x86_machine.c:1627-1637 · chunk 4167 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->size == 0) {
//     
//             memset(pr->dirty_bits, 0, pr->dirty_bits_size);
//         } else {
//             dlog.slot = pr - mem_map->phys_mem_range;
//             dlog.dirty_bitmap = pr->dirty_bits;
//             if (ioctl(s->vm_fd, 
//                                (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x42)) << 0) | ((((sizeof(struct kvm_dirty_log)))) << ((0 +8)+8)))
//                                                 , &dlog) < 0) {
//                 perror("KVM_GET_DIRTY_LOG");
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4167 {
// Port of the body of kvm_get_dirty_bits (x86_machine.c) into its own function.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Mirror of C's `PhysMemoryRange` (typedef of an anonymous struct), field for field.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

// Mirror of `struct kvm_dirty_log`.
#[repr(C)]
pub struct KvmDirtyLog {
    pub slot: u32,
    pub padding1: u32,
    pub dirty_bitmap: *mut c_void,
}

extern "C" {
    fn memset(dest: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

static KVM_GET_DIRTY_LOG_NAME: [u8; 18] = *b"KVM_GET_DIRTY_LOG\0";

#[no_mangle]
pub extern "C" fn kvm_get_dirty_bits__part4167(
    vm_fd: c_int,
    pr: *mut PhysMemoryRange,
    phys_mem_range: *const PhysMemoryRange,
    dlog: *mut KvmDirtyLog,
) {
    unsafe {
        if (*pr).size == 0 {
            memset(
                (*pr).dirty_bits as *mut c_void,
                0,
                (*pr).dirty_bits_size as c_ulong,
            );
        } else {
            // slot = pr - mem_map->phys_mem_range  (element difference)
            let byte_diff = (pr as usize).wrapping_sub(phys_mem_range as usize) as isize;
            let slot = byte_diff / core::mem::size_of::<PhysMemoryRange>() as isize;
            (*dlog).slot = slot as u32;
            (*dlog).dirty_bitmap = (*pr).dirty_bits as *mut c_void;

            let request: c_ulong = ((1u32 << (((0u32 + 8) + 8) + 14))
                | ((0xAEu32) << (0 + 8))
                | ((0x42u32) << 0)
                | ((core::mem::size_of::<KvmDirtyLog>() as u32) << ((0 + 8) + 8)))
                as c_ulong;

            if ioctl(vm_fd, request, dlog as *mut c_void) < 0 {
                perror(KVM_GET_DIRTY_LOG_NAME.as_ptr() as *const c_char);
                exit(1);
            }
        }
    }
}
}

// ── `return pr->dirty_bits;` · x86_machine.c:1638-1638 · chunk 4168 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pr->dirty_bits;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4168 {
// Rust port of the tail of kvm_get_dirty_bits: return pr->dirty_bits;

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn kvm_get_dirty_bits__part4168(pr: *mut PhysMemoryRange) -> *const u32 {
    unsafe { (*pr).dirty_bits as *const u32 }
}
}

// ── `kvm_free_ram` · x86_machine.c:1641-1646 · chunk 4169 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_free_ram(PhysMemoryMap *mem_map, PhysMemoryRange *pr)
//     {
//     
//         munmap(pr->phys_mem, pr->org_size);
//         free(pr->dirty_bits);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4169 {
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut c_uint,
    pub dirty_bits_tab: [*mut c_uint; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

extern "C" {
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn kvm_free_ram(_mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    // XXX: do it
    munmap((*pr).phys_mem as *mut c_void, (*pr).org_size as usize);
    free((*pr).dirty_bits as *mut c_void);
}
}

// ── `munmap(pr->phys_mem, pr->org_size); free(pr->dirty_bits);` · x86_machine.c:1644-1645 · chunk 4170 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     munmap(pr->phys_mem, pr->org_size);
//         free(pr->dirty_bits);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4170 {
// Port of the two-statement TARGET of kvm_free_ram: unmap guest RAM and
// release the dirty-bits buffer.

use core::ffi::{c_int, c_void};

// C: typedef int BOOL;
// C: typedef uint8_t / uint32_t / uint64_t layouts below.

/// C `PhysMemoryRange` mirrored exactly (field order/types matter for offsets).
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,          // PhysMemoryMap *map
    pub addr: u64,                 // uint64_t addr
    pub org_size: u64,             // uint64_t org_size
    pub size: u64,                 // uint64_t size
    pub is_ram: c_int,             // BOOL is_ram
    pub devram_flags: c_int,       // int devram_flags
    pub phys_mem: *mut u8,         // uint8_t *phys_mem
    pub dirty_bits_size: c_int,    // int dirty_bits_size
    pub dirty_bits: *mut u32,      // uint32_t *dirty_bits
    pub dirty_bits_tab: [*mut u32; 2], // uint32_t *dirty_bits_tab[2]
    pub dirty_bits_index: c_int,   // int dirty_bits_index
    pub opaque: *mut c_void,       // void *opaque
    pub read_func: *mut c_void,    // DeviceReadFunc *read_func
    pub write_func: *mut c_void,   // DeviceWriteFunc *write_func
    pub devio_flags: c_int,        // int devio_flags
}

extern "C" {
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn kvm_free_ram__part4170(pr: *mut PhysMemoryRange) {
    unsafe {
        if pr.is_null() {
            // C would fault here too; nothing sensible to do, just return.
            return;
        }
        let org_size = (*pr).org_size;
        let phys_mem = (*pr).phys_mem;
        let dirty_bits = (*pr).dirty_bits;
        munmap(phys_mem as *mut c_void, org_size as usize);
        free(dirty_bits as *mut c_void);
    }
}
}

// ── `kvm_pic_set_irq` · x86_machine.c:1648-1658 · chunk 4171 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_pic_set_irq(void *opaque, int irq_num, int level)
//     {
//         PCMachine *s = opaque;
//         struct kvm_irq_level irq_level;
//         irq_level.irq = irq_num;
//         irq_level.level = level;
//         if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x61)) << 0) | ((((sizeof(struct kvm_irq_level)))) << ((0 +8)+8)))
//                                        , &irq_level) < 0) {
//             perror("KVM_IRQ_LINE");
//             exit(1);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4171 {
// Rust port of the static C function kvm_pic_set_irq from x86_machine.c.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct KvmIrqLevel {
    // union { __u32 irq; __s32 status; };
    pub irq: u32,
    pub level: u32,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

// ((((1U) << (((0+8)+8)+14)) | ((0xAE) << (0+8)) | ((0x61) << 0)
//   | ((sizeof(struct kvm_irq_level)) << ((0+8)+8))))
#[inline]
fn kvm_irq_line_request() -> c_ulong {
    let size = core::mem::size_of::<KvmIrqLevel>() as u64;
    ((1u64 << (((0 + 8) + 8) + 14))
        | ((0xAEu64) << (0 + 8))
        | ((0x61u64) << 0)
        | (size << ((0 + 8) + 8))) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn kvm_pic_set_irq(opaque: *mut c_void, irq_num: c_int, level: c_int) {
    let s = opaque as *mut PCMachine;
    let mut irq_level = KvmIrqLevel {
        irq: irq_num as u32,
        level: level as u32,
    };
    let fd = (*s).vm_fd;
    let ret = ioctl(
        fd,
        kvm_irq_line_request(),
        &mut irq_level as *mut KvmIrqLevel,
    );
    if ret < 0 {
        perror(b"KVM_IRQ_LINE\0".as_ptr() as *const c_char);
        exit(1);
    }
}
}

// ── `PCMachine *s = opaque; struct kvm_irq_level irq_level; irq_level.irq = irq_num; irq_level.level…` · x86_machine.c:1650-1653 · chunk 4172 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         struct kvm_irq_level irq_level;
//         irq_level.irq = irq_num;
//         irq_level.level = level;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4172 {
// Ported piece of kvm_pic_set_irq: builds the struct kvm_irq_level argument.
use core::ffi::c_int;

// struct kvm_irq_level {
//     union { __u32 irq; __s32 status; };
//     __u32 level;
// };
#[repr(C)]
pub struct KvmIrqLevel {
    pub irq: u32,
    pub level: u32,
}

#[no_mangle]
pub extern "C" fn kvm_pic_set_irq__part4172(
    irq_num: c_int,
    level: c_int,
    irq_level: *mut KvmIrqLevel,
) {
    unsafe {
        (*irq_level).irq = irq_num as u32;
        (*irq_level).level = level as u32;
    }
}
}

// ── `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x61)) << 0) | ((((s…` · x86_machine.c:1654-1657 · chunk 4173 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x61)) << 0) | ((((sizeof(struct kvm_irq_level)))) << ((0 +8)+8)))
//                                        , &irq_level) < 0) {
//             perror("KVM_IRQ_LINE");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4173 {
// Ported piece of kvm_pic_set_irq: the KVM_IRQ_LINE ioctl.
use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct KvmIrqLevel {
    // C: union { __u32 irq; __s32 status; };
    pub irq: u32,
    pub level: u32,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_pic_set_irq__part4173(vm_fd: c_int, irq_level: *mut KvmIrqLevel) {
    let request: c_ulong = (((1u32 << 30)
        | ((0xAEu32) << 8)
        | (0x61u32)
        | ((core::mem::size_of::<KvmIrqLevel>() as u32) << 16))) as c_ulong;
    unsafe {
        if ioctl(vm_fd, request, irq_level as *mut core::ffi::c_void) < 0 {
            perror(b"KVM_IRQ_LINE\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `kvm_init` · x86_machine.c:1660-1752 · chunk 4174 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_init(PCMachine *s)
//     {
//         int ret, i;
//         struct sigaction act;
//         struct kvm_pit_config pit_config;
//         uint64_t base_addr;
//     
//         s->kvm_enabled = FALSE;
//         s->kvm_fd = open("/dev/kvm", 
//                                     02
//                                           );
//         if (s->kvm_fd < 0) {
//             fprintf(
//                    stderr
//                          , "KVM not available\n");
//             return;
//         }
//         ret = ioctl(s->kvm_fd, 
//                               (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x00)) << 0) | ((0) << ((0 +8)+8)))
//                                                  , 0);
//         if (ret < 0) {
//             perror("KVM_GET_API_VERSION");
//             exit(1);
//         }
//         if (ret != 12) {
//             fprintf(
//                    stderr
//                          , "Unsupported KVM version\n");
//             close(s->kvm_fd);
//             s->kvm_fd = -1;
//             return;
//         }
//         s->vm_fd = ioctl(s->kvm_fd, 
//                                    (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x01)) << 0) | ((0) << ((0 +8)+8)))
//                                                 , 0);
//         if (s->vm_fd < 0) {
//             perror("KVM_CREATE_VM");
//             exit(1);
//         }
//     
//     
//         base_addr = 0xfffbc000;
//         if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x48)) << 0) | ((((sizeof(__u64)))) << ((0 +8)+8)))
//                                                     , &base_addr) < 0) {
//             perror("KVM_SET_IDENTITY_MAP_ADDR");
//             exit(1);
//         }
//     
//         if (ioctl(s->vm_fd, 
//                            (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x47)) << 0) | ((0) << ((0 +8)+8)))
//                                            , (long)(base_addr + 0x1000)) < 0) {
//             perror("KVM_SET_TSS_ADDR");
//             exit(1);
//         }
//     
//         if (ioctl(s->vm_fd, 
//                            (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x60)) << 0) | ((0) << ((0 +8)+8)))
//                                              , 0) < 0) {
//             perror("KVM_CREATE_IRQCHIP");
//             exit(1);
//         }
//     
//         memset(&pit_config, 0, sizeof(pit_config));
//         pit_config.flags = 
//                           1
//                                                ;
//         if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x77)) << 0) | ((((sizeof(struct kvm_pit_config)))) << ((0 +8)+8)))
//                                           , &pit_config)) {
//             perror("KVM_CREATE_PIT2");
//             exit(1);
//         }
//     
//         s->vcpu_fd = ioctl(s->vm_fd, 
//                                     (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x41)) << 0) | ((0) << ((0 +8)+8)))
//                                                    , 0);
//         if (s->vcpu_fd < 0) {
//             perror("KVM_CREATE_VCPU");
//             exit(1);
//         }
//     
//         kvm_set_cpuid(s);
//     
//     
//         s->kvm_run_size = ioctl(s->kvm_fd, 
//                                           (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x04)) << 0) | ((0) << ((0 +8)+8)))
//                                                                 , 
//                                                                   ((void *)0)
//                                                                       );
//         if (s->kvm_run_size < 0) {
//             perror("KVM_GET_VCPU_MMAP_SIZE");
//             exit(1);
//         }
//     
//         s->kvm_run = mmap(
//                          ((void *)0)
//                              , s->kvm_run_size, 
//                                                 0x1 
//                                                           | 
//                                                             0x2
//                                                                       ,
//                           
//                          0x01
//                                    , s->vcpu_fd, 0);
//         if (!s->kvm_run) {
//             perror("mmap kvm_run");
//             exit(1);
//         }
//     
//         for(i = 0; i < 16; i++) {
//             irq_init(&s->pic_irq[i], kvm_pic_set_irq, s, i);
//         }
//     
//         act.
//            __sigaction_handler.sa_handler 
//                       = sigalrm_handler;
//         sigemptyset(&act.sa_mask);
//         act.sa_flags = 0;
//         sigaction(
//                  14
//                         , &act, 
//                                 ((void *)0)
//                                     );
//     
//         s->kvm_enabled = TRUE;
//     
//         s->mem_map->register_ram = kvm_register_ram;
//         s->mem_map->free_ram = kvm_free_ram;
//         s->mem_map->get_dirty_bits = kvm_get_dirty_bits;
//         s->mem_map->set_ram_addr = kvm_set_ram_addr;
//         s->mem_map->opaque = s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4174 {
// Rust rewrite of kvm_init() from x86_machine.c, plus the private (C "static")
// helper functions it depends on. Those helpers have no linker symbol in the C
// program once kvm_init() is replaced, so their logic is ported here.
#![allow(
    dead_code,
    unused_variables,
    unused_mut,
    unused_unsafe,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type BOOL = c_int;

const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

const O_RDWR: c_int = 2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const SIGALRM: c_int = 14;

const KVM_PIT_SPEAKER_DUMMY: u32 = 1;

/*------------------------------------------------------------------------*/
/* kernel ABI structures used by the ported helpers                       */

#[repr(C)]
pub struct KvmPitConfig {
    pub flags: u32,
    pub pad: [u32; 15],
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

#[repr(C)]
pub struct KvmDirtyLog {
    pub slot: u32,
    pub padding1: u32,
    pub dirty_bitmap: *mut u8,
}

#[repr(C)]
pub struct KvmIrqLevel {
    pub irq: u32,
    pub level: u32,
}

#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
}

#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

/*------------------------------------------------------------------------*/
/* ioctl request numbers (same values as the _IO/_IOW/_IOWR macros)       */

const KVMIO: c_ulong = 0xAE;
const IOC_NONE: c_ulong = 0;
const IOC_WRITE: c_ulong = 1;
const IOC_READ: c_ulong = 2;

const fn ioc(dir: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << 30) | (KVMIO << 8) | (nr << 0) | (size << 16)
}

const KVM_GET_API_VERSION: c_ulong = ioc(IOC_NONE, 0x00, 0);
const KVM_CREATE_VM: c_ulong = ioc(IOC_NONE, 0x01, 0);
const KVM_GET_VCPU_MMAP_SIZE: c_ulong = ioc(IOC_NONE, 0x04, 0);
const KVM_CREATE_VCPU: c_ulong = ioc(IOC_NONE, 0x41, 0);
const KVM_SET_TSS_ADDR: c_ulong = ioc(IOC_NONE, 0x47, 0);
const KVM_CREATE_IRQCHIP: c_ulong = ioc(IOC_NONE, 0x60, 0);
const KVM_GET_SUPPORTED_CPUID: c_ulong = ioc(
    IOC_READ | IOC_WRITE,
    0x05,
    core::mem::size_of::<KvmCpuid2>() as c_ulong,
);
const KVM_SET_CPUID2: c_ulong = ioc(
    IOC_WRITE,
    0x90,
    core::mem::size_of::<KvmCpuid2>() as c_ulong,
);
const KVM_SET_IDENTITY_MAP_ADDR: c_ulong = ioc(IOC_WRITE, 0x48, 8);
const KVM_CREATE_PIT2: c_ulong = ioc(
    IOC_WRITE,
    0x77,
    core::mem::size_of::<KvmPitConfig>() as c_ulong,
);
const KVM_SET_USER_MEMORY_REGION: c_ulong = ioc(
    IOC_WRITE,
    0x46,
    core::mem::size_of::<KvmUserspaceMemoryRegion>() as c_ulong,
);
const KVM_GET_DIRTY_LOG: c_ulong = ioc(
    IOC_WRITE,
    0x42,
    core::mem::size_of::<KvmDirtyLog>() as c_ulong,
);
const KVM_IRQ_LINE: c_ulong = ioc(
    IOC_WRITE,
    0x61,
    core::mem::size_of::<KvmIrqLevel>() as c_ulong,
);

/*------------------------------------------------------------------------*/
/* emulator structures accessed through pointers                          */

pub type SetIrqFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);
pub type DeviceReadFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32;
pub type DeviceWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int);
pub type RegisterRamFunc = unsafe extern "C" fn(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange;
pub type FreeRamFunc = unsafe extern "C" fn(s: *mut PhysMemoryMap, pr: *mut PhysMemoryRange);
pub type GetDirtyBitsFunc =
    unsafe extern "C" fn(s: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) -> *const u32;
pub type SetRamAddrFunc = unsafe extern "C" fn(
    s: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: BOOL,
);
pub type FlushTlbWriteRangeFunc =
    unsafe extern "C" fn(opaque: *mut c_void, ram_addr: *mut u8, ram_size: usize);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: BOOL,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRamFunc>,
    pub free_ram: Option<FreeRamFunc>,
    pub get_dirty_bits: Option<GetDirtyBitsFunc>,
    pub set_ram_addr: Option<SetRamAddrFunc>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<FlushTlbWriteRangeFunc>,
}

#[repr(C)]
pub struct IrqSignal {
    pub set_irq: Option<SetIrqFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,       /* const VirtMachineClass * */
    pub net: *mut c_void,         /* EthernetDevice * */
    pub console_dev: *mut c_void, /* VIRTIODevice * */
    pub console: *mut c_void,     /* CharacterDevice * */
    pub fb_dev: *mut c_void,      /* FBDevice * */
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,   /* X86CPUState * */
    pub pic_state: *mut c_void,   /* PIC2State * */
    pub pic_irq: [IrqSignal; 16],
    pub pit_state: *mut c_void,   /* PITState * */
    pub i440fx_state: *mut c_void,/* I440FXState * */
    pub cmos_state: *mut c_void,  /* CMOSState * */
    pub serial_state: *mut c_void,/* SerialState * */
    pub keyboard_dev: *mut c_void,/* VIRTIODevice * */
    pub mouse_dev: *mut c_void,   /* VIRTIODevice * */
    pub kbd_state: *mut c_void,   /* KBDState * */
    pub ps2_mouse: *mut c_void,   /* PS2MouseState * */
    pub vm_mouse: *mut c_void,    /* VMMouseState * */
    pub ps2_kbd: *mut c_void,     /* PS2KbdState * */
    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,     /* struct kvm_run * */
}

/*------------------------------------------------------------------------*/
/* libc / emulator declarations                                          */

#[repr(C)]
pub struct Sigset {
    pub val: [c_ulong; 16], /* 1024 / (8 * sizeof(unsigned long)) */
}

#[repr(C)]
pub struct Sigaction {
    pub sa_handler: Option<unsafe extern "C" fn(c_int)>,
    pub sa_mask: Sigset,
    pub sa_flags: c_int,
    pub sa_restorer: *mut c_void,
}

extern "C" {
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn open64(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap64(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn mallocz(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sigemptyset(set: *mut Sigset) -> c_int;
    fn sigaction(sig: c_int, act: *const Sigaction, oact: *mut Sigaction) -> c_int;
    fn register_ram_entry(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn irq_init(
        irq: *mut IrqSignal,
        set_irq: Option<SetIrqFunc>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
}

/*------------------------------------------------------------------------*/
/* ported static helpers                                                 */

static KVM_GET_SUPPORTED_CPUID_MSG: &[u8; 24] = b"KVM_GET_SUPPORTED_CPUID\0";
static KVM_SET_CPUID2_MSG: &[u8; 15] = b"KVM_SET_CPUID2\0";
static KVM_SET_USER_MEMORY_REGION_MSG: &[u8; 27] = b"KVM_SET_USER_MEMORY_REGION\0";
static KVM_GET_DIRTY_LOG_MSG: &[u8; 18] = b"KVM_GET_DIRTY_LOG\0";
static KVM_IRQ_LINE_MSG: &[u8; 13] = b"KVM_IRQ_LINE\0";

unsafe fn ptr_msg(p: *const u8) -> *const c_char {
    p as *const c_char
}

unsafe extern "C" fn sigalrm_handler(_sig: c_int) {}

unsafe extern "C" fn kvm_set_cpuid(s: *mut PCMachine) {
    unsafe {
        let n_ent_max: c_int = 128;
        let sz = core::mem::size_of::<KvmCpuid2>()
            + n_ent_max as usize * core::mem::size_of::<KvmCpuidEntry2>();
        let kvm_cpuid = mallocz(sz) as *mut KvmCpuid2;
        if kvm_cpuid.is_null() {
            /* mallocz() does not return NULL in practice; avoid a NULL deref */
            perror(ptr_msg(b"malloc kvm_cpuid\0".as_ptr()));
            exit(1);
        }

        (*kvm_cpuid).nent = n_ent_max as u32;
        if ioctl(
            (*s).kvm_fd,
            KVM_GET_SUPPORTED_CPUID,
            kvm_cpuid as *mut c_void,
        ) < 0
        {
            perror(ptr_msg(KVM_GET_SUPPORTED_CPUID_MSG.as_ptr()));
            exit(1);
        }

        let entries = (kvm_cpuid as *mut u8).add(core::mem::size_of::<KvmCpuid2>())
            as *mut KvmCpuidEntry2;
        let nent = (*kvm_cpuid).nent;
        let mut i: u32 = 0;
        while i < nent {
            let ent = entries.add(i as usize);
            /* remove the APIC & ACPI to be in sync with the emulator */
            if (*ent).function == 1 || (*ent).function == 0x80000001 {
                (*ent).edx &= !((1u32 << 9) | (1u32 << 22));
            }
            i += 1;
        }

        if ioctl((*s).vcpu_fd, KVM_SET_CPUID2, kvm_cpuid as *mut c_void) < 0 {
            perror(ptr_msg(KVM_SET_CPUID2_MSG.as_ptr()));
            exit(1);
        }
        free(kvm_cpuid as *mut c_void);
    }
}

/* index of pr inside mem_map->phys_mem_range (same value as pr - base) */
unsafe fn range_slot(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) -> u32 {
    unsafe {
        let base = (*mem_map).phys_mem_range.as_mut_ptr() as usize;
        let diff = (pr as usize).wrapping_sub(base);
        (diff / core::mem::size_of::<PhysMemoryRange>()) as u32
    }
}

unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    unsafe {
        let s = (*mem_map).opaque as *mut PCMachine;
        let mut region: KvmUserspaceMemoryRegion = core::mem::zeroed();
        let mut flags: c_int = 0;

        region.slot = range_slot(mem_map, pr);
        if (*pr).devram_flags & (1 << 0) != 0 {
            flags |= 1 << 1;
        }
        if (*pr).devram_flags & (1 << 1) != 0 {
            flags |= 1 << 0;
        }
        region.flags = flags as u32;
        region.guest_phys_addr = (*pr).addr;
        region.memory_size = (*pr).size;
        region.userspace_addr = (*pr).phys_mem as usize as u64;
        if ioctl(
            (*s).vm_fd,
            KVM_SET_USER_MEMORY_REGION,
            &mut region as *mut KvmUserspaceMemoryRegion as *mut c_void,
        ) < 0
        {
            perror(ptr_msg(KVM_SET_USER_MEMORY_REGION_MSG.as_ptr()));
            exit(1);
        }
    }
}

unsafe extern "C" fn kvm_register_ram(
    mem_map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    unsafe {
        let pr = register_ram_entry(mem_map, addr, size, devram_flags);

        let phys_mem = mmap64(
            core::ptr::null_mut(),
            size as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_ANONYMOUS,
            -1,
            0,
        );
        if phys_mem.is_null() {
            return core::ptr::null_mut();
        }
        (*pr).phys_mem = phys_mem as *mut u8;
        if devram_flags & (1 << 1) != 0 {
            let n_pages: c_int = (size >> 12) as c_int;
            (*pr).dirty_bits_size = n_pages.wrapping_add(63) / 64 * 8;
            (*pr).dirty_bits = mallocz((*pr).dirty_bits_size as usize) as *mut u32;
        }

        if (*pr).size != 0 {
            kvm_map_ram(mem_map, pr);
        }
        pr
    }
}

unsafe extern "C" fn kvm_set_ram_addr(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: BOOL,
) {
    unsafe {
        if enabled != 0 {
            if (*pr).size == 0 || addr != (*pr).addr {
                (*pr).size = (*pr).org_size;
                (*pr).addr = addr;
                kvm_map_ram(mem_map, pr);
            }
        } else if (*pr).size != 0 {
            (*pr).addr = 0;
            (*pr).size = 0;
            kvm_map_ram(mem_map, pr);
        }
    }
}

unsafe extern "C" fn kvm_get_dirty_bits(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) -> *const u32 {
    unsafe {
        let s = (*mem_map).opaque as *mut PCMachine;
        let mut dlog: KvmDirtyLog = core::mem::zeroed();

        if (*pr).size == 0 {
            core::ptr::write_bytes((*pr).dirty_bits, 0, (*pr).dirty_bits_size as usize);
        } else {
            dlog.slot = range_slot(mem_map, pr);
            dlog.dirty_bitmap = (*pr).dirty_bits as *mut u8;
            if ioctl(
                (*s).vm_fd,
                KVM_GET_DIRTY_LOG,
                &mut dlog as *mut KvmDirtyLog as *mut c_void,
            ) < 0
            {
                perror(ptr_msg(KVM_GET_DIRTY_LOG_MSG.as_ptr()));
                exit(1);
            }
        }
        (*pr).dirty_bits as *const u32
    }
}

unsafe extern "C" fn kvm_free_ram(_mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    unsafe {
        munmap((*pr).phys_mem as *mut c_void, (*pr).org_size as usize);
        free((*pr).dirty_bits as *mut c_void);
    }
}

unsafe extern "C" fn kvm_pic_set_irq(opaque: *mut c_void, irq_num: c_int, level: c_int) {
    unsafe {
        let s = opaque as *mut PCMachine;
        let mut irq_level: KvmIrqLevel = core::mem::zeroed();
        irq_level.irq = irq_num as u32;
        irq_level.level = level as u32;
        if ioctl(
            (*s).vm_fd,
            KVM_IRQ_LINE,
            &mut irq_level as *mut KvmIrqLevel as *mut c_void,
        ) < 0
        {
            perror(ptr_msg(KVM_IRQ_LINE_MSG.as_ptr()));
            exit(1);
        }
    }
}

/*------------------------------------------------------------------------*/
/* the target function                                                   */

#[no_mangle]
pub unsafe extern "C" fn kvm_init(s: *mut PCMachine) {
    unsafe {
        (*s).kvm_enabled = FALSE;
        (*s).kvm_fd = open64(ptr_msg(b"/dev/kvm\0".as_ptr()), O_RDWR);
        if (*s).kvm_fd < 0 {
            fprintf(stderr, ptr_msg(b"KVM not available\n\0".as_ptr()));
            return;
        }
        let ret: c_int = ioctl((*s).kvm_fd, KVM_GET_API_VERSION, 0 as c_int);
        if ret < 0 {
            perror(ptr_msg(b"KVM_GET_API_VERSION\0".as_ptr()));
            exit(1);
        }
        if ret != 12 {
            fprintf(stderr, ptr_msg(b"Unsupported KVM version\n\0".as_ptr()));
            close((*s).kvm_fd);
            (*s).kvm_fd = -1;
            return;
        }
        (*s).vm_fd = ioctl((*s).kvm_fd, KVM_CREATE_VM, 0 as c_int);
        if (*s).vm_fd < 0 {
            perror(ptr_msg(b"KVM_CREATE_VM\0".as_ptr()));
            exit(1);
        }

        /* just before the BIOS */
        let mut base_addr: u64 = 0xfffbc000;
        if ioctl(
            (*s).vm_fd,
            KVM_SET_IDENTITY_MAP_ADDR,
            &mut base_addr as *mut u64 as *mut c_void,
        ) < 0
        {
            perror(ptr_msg(b"KVM_SET_IDENTITY_MAP_ADDR\0".as_ptr()));
            exit(1);
        }

        if ioctl(
            (*s).vm_fd,
            KVM_SET_TSS_ADDR,
            base_addr.wrapping_add(0x1000) as c_long,
        ) < 0
        {
            perror(ptr_msg(b"KVM_SET_TSS_ADDR\0".as_ptr()));
            exit(1);
        }

        if ioctl((*s).vm_fd, KVM_CREATE_IRQCHIP, 0 as c_int) < 0 {
            perror(ptr_msg(b"KVM_CREATE_IRQCHIP\0".as_ptr()));
            exit(1);
        }

        let mut pit_config: KvmPitConfig = core::mem::zeroed();
        pit_config.flags = KVM_PIT_SPEAKER_DUMMY;
        if ioctl(
            (*s).vm_fd,
            KVM_CREATE_PIT2,
            &mut pit_config as *mut KvmPitConfig as *mut c_void,
        ) != 0
        {
            perror(ptr_msg(b"KVM_CREATE_PIT2\0".as_ptr()));
            exit(1);
        }

        (*s).vcpu_fd = ioctl((*s).vm_fd, KVM_CREATE_VCPU, 0 as c_int);
        if (*s).vcpu_fd < 0 {
            perror(ptr_msg(b"KVM_CREATE_VCPU\0".as_ptr()));
            exit(1);
        }

        kvm_set_cpuid(s);

        /* map the kvm_run structure */
        (*s).kvm_run_size = ioctl(
            (*s).kvm_fd,
            KVM_GET_VCPU_MMAP_SIZE,
            core::ptr::null_mut::<c_void>(),
        );
        if (*s).kvm_run_size < 0 {
            perror(ptr_msg(b"KVM_GET_VCPU_MMAP_SIZE\0".as_ptr()));
            exit(1);
        }

        (*s).kvm_run = mmap64(
            core::ptr::null_mut(),
            (*s).kvm_run_size as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            (*s).vcpu_fd,
            0,
        );
        if (*s).kvm_run.is_null() {
            perror(ptr_msg(b"mmap kvm_run\0".as_ptr()));
            exit(1);
        }

        let mut i: c_int = 0;
        while i < 16 {
            let irq = (*s).pic_irq.as_mut_ptr().add(i as usize);
            irq_init(irq, Some(kvm_pic_set_irq), s as *mut c_void, i);
            i += 1;
        }

        let mut act: Sigaction = core::mem::zeroed();
        act.sa_handler = Some(sigalrm_handler);
        sigemptyset(&mut act.sa_mask as *mut Sigset);
        act.sa_flags = 0;
        sigaction(SIGALRM, &act as *const Sigaction, core::ptr::null_mut());

        (*s).kvm_enabled = TRUE;

        let mem_map = (*s).mem_map;
        (*mem_map).register_ram = Some(kvm_register_ram);
        (*mem_map).free_ram = Some(kvm_free_ram);
        (*mem_map).get_dirty_bits = Some(kvm_get_dirty_bits);
        (*mem_map).set_ram_addr = Some(kvm_set_ram_addr);
        (*mem_map).opaque = s as *mut c_void;
    }
}
}

// ── `int ret, i; struct sigaction act; struct kvm_pit_config pit_config; uint64_t base_addr; s->kvm_…` · x86_machine.c:1662-1668 · chunk 4175 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int ret, i;
//         struct sigaction act;
//         struct kvm_pit_config pit_config;
//         uint64_t base_addr;
//     
//         s->kvm_enabled = FALSE;
//         s->kvm_fd = open("/dev/kvm", 
//                                     02
//                                           );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4175 {
// Portion of kvm_init: reset the KVM enable flag and open /dev/kvm (O_RDWR).
use core::ffi::{c_char, c_int};

extern "C" {
    // glibc redirects open() to the open64 symbol; call it directly.
    fn open64(path: *const c_char, oflag: c_int, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4175(kvm_enabled: *mut c_int, kvm_fd: *mut c_int) {
    unsafe {
        // s->kvm_enabled = FALSE;  (BOOL == int, FALSE == 0)
        *kvm_enabled = 0;
        // s->kvm_fd = open("/dev/kvm", O_RDWR);
        *kvm_fd = open64(b"/dev/kvm\0".as_ptr() as *const c_char, 0o2);
    }
}
}

// ── `if (s->kvm_fd < 0)` · x86_machine.c:1669-1672 · chunk 4176 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_fd < 0) {
//             fprintf(
//                    stderr
//                          , "KVM not available\n");
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4176 {
// Port of the "KVM not available" early-return check from kvm_init.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // FILE *stderr;  (FILE is opaque here, only the pointer value matters)
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

/// Returns 1 when the caller must return from kvm_init, 0 otherwise.
#[no_mangle]
pub extern "C" fn kvm_init__part4176(kvm_fd: c_int) -> c_int {
    if kvm_fd < 0 {
        unsafe {
            fprintf(
                stderr,
                b"KVM not available\n\0".as_ptr() as *const c_char,
            );
        }
        return 1;
    }
    0
}
}

// ── `ret = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x00)) << 0) | ((…` · x86_machine.c:1673-1673 · chunk 4177 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ret = ioctl(s->kvm_fd, 
//                               (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x00)) << 0) | ((0) << ((0 +8)+8)))
//                                                  , 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4177 {
// Port of the KVM_GET_API_VERSION ioctl from kvm_init().
use core::ffi::{c_int, c_ulong};

extern "C" {
    // int ioctl(int fd, unsigned long request, ...);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

// _IO(KVMIO, 0x00) == 0xAE00
const KVM_GET_API_VERSION: c_ulong = 0xAE00u64 as c_ulong;

#[no_mangle]
pub extern "C" fn kvm_init__part4177(kvm_fd: c_int) -> c_int {
    unsafe { ioctl(kvm_fd, KVM_GET_API_VERSION, 0 as c_int) }
}
}

// ── `if (ret < 0)` · x86_machine.c:1674-1677 · chunk 4178 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0) {
//             perror("KVM_GET_API_VERSION");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4178 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn perror(s: *const c_char) -> ();
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4178(ret: c_int) {
    if ret < 0 {
        unsafe {
            perror(b"KVM_GET_API_VERSION\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `if (ret != 12)` · x86_machine.c:1678-1683 · chunk 4179 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret != 12) {
//             fprintf(
//                    stderr
//                          , "Unsupported KVM version\n");
//             close(s->kvm_fd);
//             s->kvm_fd = -1;
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4179 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

// Piece of kvm_init: reject unsupported KVM API version.
// Returns 1 when the caller must `return` (unsupported version), 0 otherwise.
#[no_mangle]
pub extern "C" fn kvm_init__part4179(ret: c_int, kvm_fd: *mut c_int) -> c_int {
    if ret != 12 {
        unsafe {
            fprintf(
                stderr,
                b"Unsupported KVM version\n\0".as_ptr() as *const c_char,
            );
            if !kvm_fd.is_null() {
                close(*kvm_fd);
                *kvm_fd = -1;
            }
        }
        return 1;
    }
    0
}
}

// ── `s->vm_fd = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x01)) << 0)…` · x86_machine.c:1684-1684 · chunk 4180 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->vm_fd = ioctl(s->kvm_fd, 
//                                    (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x01)) << 0) | ((0) << ((0 +8)+8)))
//                                                 , 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4180 {
// Port of the KVM_CREATE_VM ioctl from kvm_init.
// Reads s->kvm_fd (by value), returns the new vm fd for s->vm_fd.

use core::ffi::{c_int, c_ulong};

extern "C" {
    // extern int ioctl(int __fd, unsigned long int __request, ...);
    fn ioctl(__fd: c_int, __request: c_ulong, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4180(kvm_fd: c_int) -> c_int {
    // KVM_CREATE_VM = _IO(KVMIO, 0x01) with KVMIO == 0xAE
    // = (0U << 30) | (0xAE << 8) | (0x01 << 0) | (0 << 16)
    let request: c_ulong = ((0u32) << (((0 + 8) + 8) + 14)) as c_ulong
        | ((0xAEu32) << (0 + 8)) as c_ulong
        | ((0x01u32) << 0) as c_ulong
        | ((0u32) << ((0 + 8) + 8)) as c_ulong;
    unsafe { ioctl(kvm_fd, request, 0 as c_int) }
}
}

// ── `if (s->vm_fd < 0)` · x86_machine.c:1685-1688 · chunk 4181 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->vm_fd < 0) {
//             perror("KVM_CREATE_VM");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4181 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

// Part of kvm_init: check that KVM_CREATE_VM produced a valid vm fd.
#[no_mangle]
pub extern "C" fn kvm_init__part4181(vm_fd: c_int) {
    static MSG: &[u8] = b"KVM_CREATE_VM\0";

    if vm_fd < 0 {
        unsafe {
            perror(MSG.as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `base_addr = 0xfffbc000;` · x86_machine.c:1691-1691 · chunk 4182 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     base_addr = 0xfffbc000;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4182 {
// Assignment of the KVM identity-map base address used by kvm_init.
// C: base_addr = 0xfffbc000;  (base_addr is uint64_t)

#[no_mangle]
pub extern "C" fn kvm_init__part4182() -> u64 {
    0xfffbc000u64
}
}

// ── `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x48)) << 0) | ((((s…` · x86_machine.c:1692-1695 · chunk 4183 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x48)) << 0) | ((((sizeof(__u64)))) << ((0 +8)+8)))
//                                                     , &base_addr) < 0) {
//             perror("KVM_SET_IDENTITY_MAP_ADDR");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4183 {
// Port of the KVM_SET_IDENTITY_MAP_ADDR ioctl from kvm_init.
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

// KVMIO == 0xAE, KVM_SET_IDENTITY_MAP_ADDR == _IOW(KVMIO, 0x48, __u64)
const KVM_SET_IDENTITY_MAP_ADDR: c_ulong = ((1u32 << (((0 + 8) + 8) + 14))
    | ((0xAEu32) << (0 + 8))
    | ((0x48u32) << 0)
    | ((8u32) << ((0 + 8) + 8))) as c_ulong;

#[no_mangle]
pub extern "C" fn kvm_init__part4183(vm_fd: c_int, base_addr: *mut u64) -> c_int {
    unsafe { ioctl(vm_fd, KVM_SET_IDENTITY_MAP_ADDR, base_addr) }
}
}

// ── `if (ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x47)) << 0) | ((0)…` · x86_machine.c:1697-1700 · chunk 4184 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x47)) << 0) | ((0) << ((0 +8)+8)))
//                                            , (long)(base_addr + 0x1000)) < 0) {
//             perror("KVM_SET_TSS_ADDR");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4184 {
// Ported piece of kvm_init: KVM_SET_TSS_ADDR ioctl check.
use core::ffi::{c_char, c_int, c_long, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4184(vm_fd: c_int, base_addr: u64) {
    // (long)(base_addr + 0x1000) -- wrapping add then reinterpret as long
    let addr: c_long = base_addr.wrapping_add(0x1000) as c_long;
    // _IO(0xAE, 0x47) == 0xAE47
    let request: c_ulong = 0xAE47u64 as c_ulong;
    unsafe {
        if ioctl(vm_fd, request, addr) < 0 {
            perror(b"KVM_SET_TSS_ADDR\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `if (ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x60)) << 0) | ((0)…` · x86_machine.c:1702-1705 · chunk 4185 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x60)) << 0) | ((0) << ((0 +8)+8)))
//                                              , 0) < 0) {
//             perror("KVM_CREATE_IRQCHIP");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4185 {
// Port of the KVM_CREATE_IRQCHIP step of kvm_init().
use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4185(vm_fd: c_int) {
    // _IOW(0xAE, 0x60, ...) style request with no argument size:
    // (((0U) << 30) | (0xAE << 8) | (0x60 << 0) | (0 << 16)) == 0xAE60
    let request: c_ulong = 0xAE60;

    let ret = unsafe { ioctl(vm_fd, request, 0i32) };
    if ret < 0 {
        unsafe {
            perror(b"KVM_CREATE_IRQCHIP\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `memset(&pit_config, 0, sizeof(pit_config)); pit_config.flags = 1 ;` · x86_machine.c:1707-1708 · chunk 4186 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(&pit_config, 0, sizeof(pit_config));
//         pit_config.flags = 
//                           1
//                                                ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4186 {
// Portion of kvm_init: zero-initialize struct kvm_pit_config and set flags = 1.

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kvm_pit_config {
    pub flags: u32,
    pub pad: [u32; 15],
}

#[no_mangle]
pub extern "C" fn kvm_init__part4186(pit_config: *mut kvm_pit_config) {
    if pit_config.is_null() {
        return;
    }
    unsafe {
        core::ptr::write_bytes(
            pit_config as *mut u8,
            0u8,
            core::mem::size_of::<kvm_pit_config>(),
        );
        (*pit_config).flags = 1u32;
    }
}
}

// ── `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x77)) << 0) | ((((s…` · x86_machine.c:1709-1712 · chunk 4187 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->vm_fd, 
//                            (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x77)) << 0) | ((((sizeof(struct kvm_pit_config)))) << ((0 +8)+8)))
//                                           , &pit_config)) {
//             perror("KVM_CREATE_PIT2");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4187 {
// Port of the KVM_CREATE_PIT2 request performed by kvm_init().
use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct KvmPitConfig {
    pub flags: u32,
    pub pad: [u32; 15],
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4187(vm_fd: c_int, pit_config: *mut KvmPitConfig) -> c_int {
    let size = core::mem::size_of::<KvmPitConfig>() as u64;
    let request: c_ulong = (1u64 << 30) | (0xAEu64 << 8) | 0x77u64 | (size << 16);
    let ret = unsafe { ioctl(vm_fd, request, pit_config as *mut core::ffi::c_void) };
    if ret != 0 {
        unsafe {
            perror(b"KVM_CREATE_PIT2\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
    ret
}
}

// ── `s->vcpu_fd = ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x41)) << 0…` · x86_machine.c:1714-1714 · chunk 4188 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->vcpu_fd = ioctl(s->vm_fd, 
//                                     (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x41)) << 0) | ((0) << ((0 +8)+8)))
//                                                    , 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4188 {
// Ported piece of kvm_init(): the KVM_CREATE_VCPU ioctl.
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

// _IO(KVMIO, 0x41)
const KVM_CREATE_VCPU: u32 = (0u32 << 22) | (0xAEu32 << 8) | (0x41u32 << 0) | (0u32 << 16);

#[no_mangle]
pub extern "C" fn kvm_init__part4188(vm_fd: c_int) -> c_int {
    unsafe { ioctl(vm_fd, KVM_CREATE_VCPU as c_ulong, 0 as c_int) }
}
}

// ── `if (s->vcpu_fd < 0)` · x86_machine.c:1715-1718 · chunk 4189 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (s->vcpu_fd < 0) {
//             perror("KVM_CREATE_VCPU");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4189 {
// Ported piece of kvm_init: check the vCPU fd and abort on failure.
use core::ffi::c_char;
use core::ffi::c_int;

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

static MSG_KVM_CREATE_VCPU: [u8; 16] = *b"KVM_CREATE_VCPU\0";

#[no_mangle]
pub extern "C" fn kvm_init__part4189(vcpu_fd: c_int) {
    if vcpu_fd < 0 {
        unsafe {
            perror(MSG_KVM_CREATE_VCPU.as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `kvm_set_cpuid(s); s->kvm_run_size = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) <<…` · x86_machine.c:1720-1723 · chunk 4190 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     kvm_set_cpuid(s);
//     
//     
//         s->kvm_run_size = ioctl(s->kvm_fd, 
//                                           (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x04)) << 0) | ((0) << ((0 +8)+8)))
//                                                                 , 
//                                                                   ((void *)0)
//                                                                       );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4190 {
// Port of the piece of kvm_init() that installs the CPUID table on the vCPU and
// queries the size of the kvm_run mapping.  The static helper kvm_set_cpuid()
// has no linker symbol, so its logic is ported here as a private Rust fn.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_ulong, c_void};

/* mirror of the C structs (layout only) */

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

/* mirror of struct kvm_cpuid_entry2 */
#[repr(C)]
struct kvm_cpuid_entry2 {
    function: u32,
    index: u32,
    flags: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    padding: [u32; 3],
}

/* mirror of struct kvm_cpuid2 (only the fixed header; entries follow inline) */
#[repr(C)]
struct kvm_cpuid2 {
    nent: u32,
    padding: u32,
}

/* _IOWR(KVMIO, 0x05, struct kvm_cpuid2) */
const KVM_GET_SUPPORTED_CPUID: c_ulong =
    ((2u32 | 1u32) << 30) as c_ulong | (0xAEu32 << 8) as c_ulong | 0x05u32 as c_ulong
        | ((core::mem::size_of::<kvm_cpuid2>() as u32) << 16) as c_ulong;
/* _IOW(KVMIO, 0x90, struct kvm_cpuid2) */
const KVM_SET_CPUID2: c_ulong = (1u32 << 30) as c_ulong
    | (0xAEu32 << 8) as c_ulong
    | 0x90u32 as c_ulong
    | ((core::mem::size_of::<kvm_cpuid2>() as u32) << 16) as c_ulong;
/* _IO(KVMIO, 0x04) */
const KVM_GET_VCPU_MMAP_SIZE: c_ulong =
    (0xAEu32 << 8) as c_ulong | 0x04u32 as c_ulong;

const CPUID_APIC: u32 = 1u32 << 9;
const CPUID_ACPI: u32 = 1u32 << 22;

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mallocz(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn perror(s: *const i8);
    fn exit(status: c_int) -> !;
}

/* port of the file-local static helper kvm_set_cpuid() */
unsafe fn kvm_set_cpuid(s: *mut PCMachine) {
    let n_ent_max: c_int = 128;
    let hdr_size = core::mem::size_of::<kvm_cpuid2>();
    let ent_size = core::mem::size_of::<kvm_cpuid_entry2>();
    let kvm_cpuid =
        mallocz(hdr_size + (n_ent_max as usize) * ent_size) as *mut kvm_cpuid2;

    (*kvm_cpuid).nent = n_ent_max as u32;
    if ioctl((*s).kvm_fd, KVM_GET_SUPPORTED_CPUID, kvm_cpuid) < 0 {
        perror(b"KVM_GET_SUPPORTED_CPUID\0".as_ptr() as *const i8);
        exit(1);
    }

    let entries = (kvm_cpuid as *mut u8).add(hdr_size) as *mut kvm_cpuid_entry2;
    let nent = (*kvm_cpuid).nent as c_int;
    let mut i: c_int = 0;
    while i < nent {
        let ent = entries.offset(i as isize);
        if (*ent).function == 1 || (*ent).function == 0x80000001 {
            (*ent).edx &= !(CPUID_APIC | CPUID_ACPI);
        }
        i += 1;
    }

    if ioctl((*s).vcpu_fd, KVM_SET_CPUID2, kvm_cpuid) < 0 {
        perror(b"KVM_SET_CPUID2\0".as_ptr() as *const i8);
        exit(1);
    }
    free(kvm_cpuid as *mut c_void);
}

#[no_mangle]
pub extern "C" fn kvm_init__part4190(s: *mut PCMachine) {
    unsafe {
        kvm_set_cpuid(s);

        (*s).kvm_run_size = ioctl(
            (*s).kvm_fd,
            KVM_GET_VCPU_MMAP_SIZE,
            core::ptr::null_mut::<c_void>(),
        );
    }
}
}

// ── `if (s->kvm_run_size < 0)` · x86_machine.c:1724-1727 · chunk 4191 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_run_size < 0) {
//             perror("KVM_GET_VCPU_MMAP_SIZE");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4191 {
// Ported piece of kvm_init: the KVM_GET_VCPU_MMAP_SIZE failure check.
use core::ffi::{c_char, c_int};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

#[no_mangle]
pub extern "C" fn kvm_init__part4191(kvm_run_size: c_int) {
    if kvm_run_size < 0 {
        unsafe {
            perror(b"KVM_GET_VCPU_MMAP_SIZE\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `s->kvm_run = mmap( ((void *)0) , s->kvm_run_size, 0x1 | 0x2 , 0x01 , s->vcpu_fd, 0);` · x86_machine.c:1729-1730 · chunk 4192 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->kvm_run = mmap(
//                          ((void *)0)
//                              , s->kvm_run_size, 
//                                                 0x1 
//                                                           | 
//                                                             0x2
//                                                                       ,
//                           
//                          0x01
//                                    , s->vcpu_fd, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4192 {
// Port of the mmap() call that maps the KVM run structure.
use core::ffi::{c_int, c_void};

extern "C" {
    // The preprocessed C refers to this function through the symbol "mmap64".
    fn mmap64(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn kvm_init__part4192(len: c_int, fd: c_int) -> *mut c_void {
    // prot = PROT_READ | PROT_WRITE (0x1 | 0x2), flags = MAP_SHARED (0x01)
    const PROT_READ: c_int = 0x1;
    const PROT_WRITE: c_int = 0x2;
    const MAP_SHARED: c_int = 0x01;
    unsafe {
        mmap64(
            core::ptr::null_mut(),
            len as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0i64,
        )
    }
}
}

// ── `if (!s->kvm_run)` · x86_machine.c:1731-1734 · chunk 4193 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s->kvm_run) {
//             perror("mmap kvm_run");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4193 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

static MMAP_KVM_RUN: &[u8] = b"mmap kvm_run\0";

#[no_mangle]
pub extern "C" fn kvm_init__part4193(kvm_run: *mut c_void) {
    if kvm_run.is_null() {
        unsafe {
            perror(MMAP_KVM_RUN.as_ptr() as *const c_char);
            exit(1);
        }
    }
}
}

// ── `for (i = 0; i < 16; i++)` · x86_machine.c:1736-1738 · chunk 4194 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 16; i++) {
//             irq_init(&s->pic_irq[i], kvm_pic_set_irq, s, i);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4194 {
// Port of the KVM PIC IRQ initialisation loop from kvm_init().
use core::ffi::{c_int, c_void};

// Mirrors the C type `IRQSignal` (see iomem.h):
//   typedef void SetIRQFunc(void *opaque, int irq_num, int level);
//   typedef struct { SetIRQFunc *set_irq; void *opaque; int irq_num; } IRQSignal;
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

extern "C" {
    // Defined in iomem.c (external linkage, so it has a linker symbol).
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn kvm_init__part4194(
    pic_irq: *mut IRQSignal,
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
) {
    let mut i: c_int = 0;
    while i < 16 {
        irq_init(pic_irq.add(i as usize), set_irq, opaque, i);
        i += 1;
    }
}
}

// ── `act. __sigaction_handler.sa_handler = sigalrm_handler; sigemptyset(&act.sa_mask); act.sa_flags…` · x86_machine.c:1740-1751 · chunk 4195 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     act.
//            __sigaction_handler.sa_handler 
//                       = sigalrm_handler;
//         sigemptyset(&act.sa_mask);
//         act.sa_flags = 0;
//         sigaction(
//                  14
//                         , &act, 
//                                 ((void *)0)
//                                     );
//     
//         s->kvm_enabled = TRUE;
//     
//         s->mem_map->register_ram = kvm_register_ram;
//         s->mem_map->free_ram = kvm_free_ram;
//         s->mem_map->get_dirty_bits = kvm_get_dirty_bits;
//         s->mem_map->set_ram_addr = kvm_set_ram_addr;
//         s->mem_map->opaque = s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4195 {
// Port of the tail of kvm_init(): install the SIGALRM handler, mark KVM as
// enabled and hook the KVM RAM callbacks into the machine's memory map.
//
// The callbacks (kvm_register_ram, kvm_free_ram, kvm_get_dirty_bits,
// kvm_set_ram_addr) and sigalrm_handler are `static` in the C translation
// unit, so they have no linker symbol and are passed in as function pointers.

use core::ffi::{c_int, c_ulong, c_void};
use core::ptr;

// __sigset_t: struct { unsigned long __val[1024 / (8 * sizeof(unsigned long))]; }
#[repr(C)]
pub struct Sigset {
    pub val: [c_ulong; 1024 / (8 * core::mem::size_of::<c_ulong>())],
}

// struct sigaction; the first member is an anonymous union whose larger
// member (void (*)(int, siginfo_t *, void *)) has the same size/alignment as
// __sighandler_t, so a single pointer field models it exactly.
#[repr(C)]
pub struct Sigaction {
    pub sa_handler: Option<extern "C" fn(c_int)>,
    pub sa_mask: Sigset,
    pub sa_flags: c_int,
    pub sa_restorer: Option<extern "C" fn()>,
}

type RegisterRamFn =
    unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange;
type FreeRamFn = unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange);
type GetDirtyBitsFn =
    unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32;
type SetRamAddrFn = unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRamFn>,
    pub free_ram: Option<FreeRamFn>,
    pub get_dirty_bits: Option<GetDirtyBitsFn>,
    pub set_ram_addr: Option<SetRamAddrFn>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

extern "C" {
    fn sigemptyset(set: *mut Sigset) -> c_int;
    fn sigaction(sig: c_int, act: *const Sigaction, oact: *mut Sigaction) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn kvm_init__part4195(
    kvm_enabled: *mut c_int,
    mem_map: *mut PhysMemoryMap,
    act: *mut Sigaction,
    handler: Option<extern "C" fn(c_int)>,
    register_ram: Option<RegisterRamFn>,
    free_ram: Option<FreeRamFn>,
    get_dirty_bits: Option<GetDirtyBitsFn>,
    set_ram_addr: Option<SetRamAddrFn>,
    opaque: *mut c_void,
) {
    // act.__sigaction_handler.sa_handler = sigalrm_handler;
    let act = &mut *act;
    act.sa_handler = handler;
    // sigemptyset(&act.sa_mask);
    sigemptyset(&mut act.sa_mask);
    // act.sa_flags = 0;
    act.sa_flags = 0;
    // sigaction(14, &act, NULL);
    sigaction(14, act as *const Sigaction, ptr::null_mut());

    // s->kvm_enabled = TRUE;
    *kvm_enabled = 1;

    // s->mem_map->register_ram = kvm_register_ram; ... s->mem_map->opaque = s;
    let mm = &mut *mem_map;
    mm.register_ram = register_ram;
    mm.free_ram = free_ram;
    mm.get_dirty_bits = get_dirty_bits;
    mm.set_ram_addr = set_ram_addr;
    mm.opaque = opaque;
}
}

// ── `kvm_exit_io` · x86_machine.c:1754-1794 · chunk 4196 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_exit_io(PCMachine *s, struct kvm_run *run)
//     {
//         uint8_t *ptr;
//         int i;
//     
//         ptr = (uint8_t *)run + run->io.data_offset;
//     
//     
//         for(i = 0; i < run->io.count; i++) {
//             if (run->io.direction == 
//                                     1
//                                                    ) {
//                 switch(run->io.size) {
//                 case 1:
//                     st_port(s, run->io.port, *(uint8_t *)ptr, 0);
//                     break;
//                 case 2:
//                     st_port(s, run->io.port, *(uint16_t *)ptr, 1);
//                     break;
//                 case 4:
//                     st_port(s, run->io.port, *(uint32_t *)ptr, 2);
//                     break;
//                 default:
//                     abort();
//                 }
//             } else {
//                 switch(run->io.size) {
//                 case 1:
//                     *(uint8_t *)ptr = ld_port(s, run->io.port, 0);
//                     break;
//                 case 2:
//                     *(uint16_t *)ptr = ld_port(s, run->io.port, 1);
//                     break;
//                 case 4:
//                     *(uint32_t *)ptr = ld_port(s, run->io.port, 2);
//                     break;
//                 default:
//                     abort();
//                 }
//             }
//             ptr += run->io.size;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4196 {
// Rust port of x86_machine.c: kvm_exit_io()
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;

// phys_mem_map: PhysMemoryRange (see iomem.h)
#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut c_void,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: BOOL,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: *mut c_void,
    write_func: *mut c_void,
    devio_flags: c_int,
}

// machine.h: VirtMachine
#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// x86_machine.c: PCMachine (only the prefix needed here is modelled)
#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
}

// linux/kvm.h: struct kvm_run, io sub-structure
#[repr(C)]
pub struct KvmIo {
    direction: u8,
    size: u8,
    port: u16,
    count: u32,
    data_offset: u64,
}

#[repr(C)]
pub struct KvmRun {
    request_interrupt_window: u8,
    immediate_exit: u8,
    padding1: [u8; 6],
    exit_reason: u32,
    ready_for_interrupt_injection: u8,
    if_flag: u8,
    flags: u16,
    cr8: u64,
    apic_base: u64,
    io: KvmIo,
}

extern "C" {
    fn get_phys_mem_range(s: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
    fn abort() -> !;
}

type DevWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type DevReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;

// static void st_port(void *opaque, uint32_t port, uint32_t val, int size_log2)
unsafe fn st_port(opaque: *mut c_void, port1: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut PCMachine;
    let pr = get_phys_mem_range((*s).port_map, port1 as u64);
    if pr.is_null() {
        return;
    }
    let port = (port1 as u64).wrapping_sub((*pr).addr) as u32;
    let flags = (*pr).devio_flags;
    if ((flags >> size_log2) & 1) != 0 {
        let wf: DevWriteFunc = core::mem::transmute((*pr).write_func);
        wf((*pr).opaque, port, val, size_log2);
    } else if size_log2 == 1 && (flags & 1) != 0 {
        let wf: DevWriteFunc = core::mem::transmute((*pr).write_func);
        wf((*pr).opaque, port, val & 0xff, 0);
        let wf2: DevWriteFunc = core::mem::transmute((*pr).write_func);
        wf2((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
    }
}

// static uint32_t ld_port(void *opaque, uint32_t port1, int size_log2)
unsafe fn ld_port(opaque: *mut c_void, port1: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut PCMachine;
    let pr = get_phys_mem_range((*s).port_map, port1 as u64);
    if pr.is_null() {
        return u32::MAX; // (uint32_t)-1
    }
    let port = (port1 as u64).wrapping_sub((*pr).addr) as u32;
    let flags = (*pr).devio_flags;
    if ((flags >> size_log2) & 1) != 0 {
        let rf: DevReadFunc = core::mem::transmute((*pr).read_func);
        rf((*pr).opaque, port, size_log2)
    } else if size_log2 == 1 && (flags & 1) != 0 {
        let rf: DevReadFunc = core::mem::transmute((*pr).read_func);
        let mut val = rf((*pr).opaque, port, 0) & 0xff;
        let rf2: DevReadFunc = core::mem::transmute((*pr).read_func);
        val |= (rf2((*pr).opaque, port.wrapping_add(1), 0) & 0xff) << 8;
        val
    } else {
        u32::MAX // (uint32_t)-1
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_io(s: *mut PCMachine, run: *mut KvmRun) {
    let mut ptr: *mut u8 = (run as *mut u8).wrapping_add((*run).io.data_offset as usize);
    let mut i: c_int = 0;

    while (i as u32) < (*run).io.count {
        let port = (*run).io.port as u32;
        if (*run).io.direction as c_int == 1 {
            // KVM_EXIT_IO_OUT
            match (*run).io.size as c_int {
                1 => st_port(s as *mut c_void, port, core::ptr::read_unaligned(ptr as *const u8) as u32, 0),
                2 => st_port(s as *mut c_void, port, core::ptr::read_unaligned(ptr as *const u16) as u32, 1),
                4 => st_port(s as *mut c_void, port, core::ptr::read_unaligned(ptr as *const u32), 2),
                _ => abort(),
            }
        } else {
            match (*run).io.size as c_int {
                1 => core::ptr::write_unaligned(ptr as *mut u8, ld_port(s as *mut c_void, port, 0) as u8),
                2 => core::ptr::write_unaligned(ptr as *mut u16, ld_port(s as *mut c_void, port, 1) as u16),
                4 => core::ptr::write_unaligned(ptr as *mut u32, ld_port(s as *mut c_void, port, 2)),
                _ => abort(),
            }
        }
        ptr = ptr.wrapping_add((*run).io.size as usize);
        i = i.wrapping_add(1);
    }
}
}

// ── `uint8_t *ptr; int i; ptr = (uint8_t *)run + run->io.data_offset;` · x86_machine.c:1756-1759 · chunk 4197 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint8_t *ptr;
//         int i;
//     
//         ptr = (uint8_t *)run + run->io.data_offset;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4197 {
// Computes the I/O data buffer pointer for a KVM_EXIT_IO exit.
// Mirrors the C struct kvm_run prefix up to and including the io member.

#[repr(C)]
struct KvmIo {
    direction: u8,
    size: u8,
    port: u16,
    count: u32,
    data_offset: u64,
}

#[repr(C)]
struct KvmRunPrefix {
    request_interrupt_window: u8,
    immediate_exit: u8,
    padding1: [u8; 6],
    exit_reason: u32,
    ready_for_interrupt_injection: u8,
    if_flag: u8,
    flags: u16,
    cr8: u64,
    apic_base: u64,
    io: KvmIo,
}

#[no_mangle]
pub extern "C" fn kvm_exit_io__part4197(run: *mut KvmRunPrefix) -> *mut u8 {
    unsafe {
        let base = run as *mut u8;
        let off = (*run).io.data_offset as usize;
        base.wrapping_add(off)
    }
}
}

// ── `for (i = 0; i < run->io.count; i++)` · x86_machine.c:1762-1793 · chunk 4198 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < run->io.count; i++) {
//             if (run->io.direction == 
//                                     1
//                                                    ) {
//                 switch(run->io.size) {
//                 case 1:
//                     st_port(s, run->io.port, *(uint8_t *)ptr, 0);
//                     break;
//                 case 2:
//                     st_port(s, run->io.port, *(uint16_t *)ptr, 1);
//                     break;
//                 case 4:
//                     st_port(s, run->io.port, *(uint32_t *)ptr, 2);
//                     break;
//                 default:
//                     abort();
//                 }
//             } else {
//                 switch(run->io.size) {
//                 case 1:
//                     *(uint8_t *)ptr = ld_port(s, run->io.port, 0);
//                     break;
//                 case 2:
//                     *(uint16_t *)ptr = ld_port(s, run->io.port, 1);
//                     break;
//                 case 4:
//                     *(uint32_t *)ptr = ld_port(s, run->io.port, 2);
//                     break;
//                 default:
//                     abort();
//                 }
//             }
//             ptr += run->io.size;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4198 {
// Port of the kvm_exit_io I/O loop (PIO in/out over run->io).
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

// Mirror of C's PhysMemoryRange (typedef struct { ... } PhysMemoryRange).
#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut c_void,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: Option<DeviceReadFunc>,
    write_func: Option<DeviceWriteFunc>,
    devio_flags: c_int,
}

// Layout of the union member `io` of struct kvm_run.
#[repr(C)]
pub struct KvmRunIo {
    direction: u8,
    size: u8,
    port: u16,
    count: u32,
    data_offset: u64,
}

// Prefix mirror of struct kvm_run, up to and including the io union member.
#[repr(C)]
pub struct KvmRun {
    request_interrupt_window: u8,
    immediate_exit: u8,
    padding1: [u8; 6],
    exit_reason: u32,
    ready_for_interrupt_injection: u8,
    if_flag: u8,
    flags: u16,
    cr8: u64,
    apic_base: u64,
    io: KvmRunIo,
}

extern "C" {
    fn get_phys_mem_range(map: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
    fn abort() -> !;
}

// Ported from the static C function st_port().
unsafe fn st_port(port_map: *mut c_void, port_in: u32, val: u32, size_log2: c_int) {
    let pr = get_phys_mem_range(port_map, port_in as u64);
    if pr.is_null() {
        return;
    }
    let port = (port_in as u64).wrapping_sub((*pr).addr) as u32;
    let flags = (*pr).devio_flags;
    if (flags.wrapping_shr(size_log2 as u32) & 1) != 0 {
        if let Some(wf) = (*pr).write_func {
            wf((*pr).opaque, port, val, size_log2);
        }
    } else if size_log2 == 1 && (flags & 1) != 0 {
        if let Some(wf) = (*pr).write_func {
            wf((*pr).opaque, port, val & 0xff, 0);
            wf((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
        }
    }
}

// Ported from the static C function ld_port().
unsafe fn ld_port(port_map: *mut c_void, port1: u32, size_log2: c_int) -> u32 {
    let mut val: u32;
    let mut port: u32 = port1;
    let pr = get_phys_mem_range(port_map, port as u64);
    if pr.is_null() {
        val = 0xffff_ffff;
    } else {
        port = (port as u64).wrapping_sub((*pr).addr) as u32;
        let flags = (*pr).devio_flags;
        if (flags.wrapping_shr(size_log2 as u32) & 1) != 0 {
            val = match (*pr).read_func {
                Some(rf) => rf((*pr).opaque, port, size_log2),
                None => 0xffff_ffff,
            };
        } else if size_log2 == 1 && (flags & 1) != 0 {
            match (*pr).read_func {
                Some(rf) => {
                    val = rf((*pr).opaque, port, 0) & 0xff;
                    val |= (rf((*pr).opaque, port.wrapping_add(1), 0) & 0xff) << 8;
                }
                None => {
                    val = 0xffff_ffff;
                }
            }
        } else {
            val = 0xffff_ffff;
        }
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_io__part4198(
    port_map: *mut c_void,
    run: *mut KvmRun,
    ptr: *mut *mut u8,
    i_out: *mut c_int,
) {
    let mut p: *mut u8 = *ptr;
    let mut i: u32 = 0;
    while i < (*run).io.count {
        let direction = (*run).io.direction;
        let size = (*run).io.size;
        let port = (*run).io.port;
        if direction == 1 {
            match size {
                1 => {
                    let v = core::ptr::read_unaligned(p as *const u8) as u32;
                    st_port(port_map, port as u32, v, 0);
                }
                2 => {
                    let v = core::ptr::read_unaligned(p as *const u16) as u32;
                    st_port(port_map, port as u32, v, 1);
                }
                4 => {
                    let v = core::ptr::read_unaligned(p as *const u32);
                    st_port(port_map, port as u32, v, 2);
                }
                _ => abort(),
            }
        } else {
            match size {
                1 => core::ptr::write_unaligned(p as *mut u8, ld_port(port_map, port as u32, 0) as u8),
                2 => core::ptr::write_unaligned(p as *mut u16, ld_port(port_map, port as u32, 1) as u16),
                4 => core::ptr::write_unaligned(p as *mut u32, ld_port(port_map, port as u32, 2)),
                _ => abort(),
            }
        }
        p = p.wrapping_add(size as usize);
        i = i.wrapping_add(1);
    }
    *ptr = p;
    if !i_out.is_null() {
        *i_out = i as c_int;
    }
}
}

// ── `kvm_exit_mmio` · x86_machine.c:1796-1868 · chunk 4199 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_exit_mmio(PCMachine *s, struct kvm_run *run)
//     {
//         uint8_t *data = run->mmio.data;
//         PhysMemoryRange *pr;
//         uint64_t addr;
//     
//         pr = get_phys_mem_range(s->mem_map, run->mmio.phys_addr);
//         if (run->mmio.is_write) {
//             if (!pr || pr->is_ram)
//                 return;
//             addr = run->mmio.phys_addr - pr->addr;
//             switch(run->mmio.len) {
//             case 1:
//                 if (pr->devio_flags & (1 << 0)) {
//                     pr->write_func(pr->opaque, addr, *(uint8_t *)data, 0);
//                 }
//                 break;
//             case 2:
//                 if (pr->devio_flags & (1 << 1)) {
//                     pr->write_func(pr->opaque, addr, *(uint16_t *)data, 1);
//                 }
//                 break;
//             case 4:
//                 if (pr->devio_flags & (1 << 2)) {
//                     pr->write_func(pr->opaque, addr, *(uint32_t *)data, 2);
//                 }
//                 break;
//             case 8:
//                 if (pr->devio_flags & (1 << 2)) {
//                     pr->write_func(pr->opaque, addr, *(uint32_t *)data, 2);
//                     pr->write_func(pr->opaque, addr + 4, *(uint32_t *)(data + 4), 2);
//                 }
//                 break;
//             default:
//                 abort();
//             }
//         } else {
//             if (!pr || pr->is_ram)
//                 goto no_dev;
//             addr = run->mmio.phys_addr - pr->addr;
//             switch(run->mmio.len) {
//             case 1:
//                 if (!(pr->devio_flags & (1 << 0)))
//                     goto no_dev;
//                 *(uint8_t *)data = pr->read_func(pr->opaque, addr, 0);
//                 break;
//             case 2:
//                 if (!(pr->devio_flags & (1 << 1)))
//                     goto no_dev;
//                 *(uint16_t *)data = pr->read_func(pr->opaque, addr, 1);
//                 break;
//             case 4:
//                 if (!(pr->devio_flags & (1 << 2)))
//                     goto no_dev;
//                 *(uint32_t *)data = pr->read_func(pr->opaque, addr, 2);
//                 break;
//             case 8:
//                 if (pr->devio_flags & (1 << 2)) {
//                     *(uint32_t *)data =
//                         pr->read_func(pr->opaque, addr, 2);
//                     *(uint32_t *)(data + 4) =
//                         pr->read_func(pr->opaque, addr + 4, 2);
//                 } else {
//                 no_dev:
//                     memset(run->mmio.data, 0, run->mmio.len);
//                 }
//                 break;
//             default:
//                 abort();
//             }
//     
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4199 {
// Rust rewrite of the static C function kvm_exit_mmio.
use core::ffi::{c_int, c_void};

type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;

// Mirror of the C PhysMemoryRange struct (iomem.h), fields in order.
#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut c_void,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: Option<DeviceReadFunc>,
    write_func: Option<DeviceWriteFunc>,
    devio_flags: c_int,
}

// Mirror of the C VirtMachine struct (machine.h): all pointer members.
#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// Mirror of the C PCMachine struct (x86_machine.c). Only the leading
// members up to mem_map are needed, they are listed in order.
#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
}

#[repr(C)]
pub struct KvmMmio {
    phys_addr: u64,
    data: [u8; 8],
    len: u32,
    is_write: u8,
}

// Mirror of struct kvm_run (linux/kvm.h) up to and including the mmio
// member of the trailing union, which starts right after apic_base.
#[repr(C)]
pub struct KvmRun {
    request_interrupt_window: u8,
    immediate_exit: u8,
    padding1: [u8; 6],
    exit_reason: u32,
    ready_for_interrupt_injection: u8,
    if_flag: u8,
    flags: u16,
    cr8: u64,
    apic_base: u64,
    mmio: KvmMmio,
}

extern "C" {
    fn get_phys_mem_range(s: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
    fn abort() -> !;
}

#[inline]
unsafe fn dev_write(pr: *mut PhysMemoryRange, addr: u64, val: u32, size_log2: c_int) {
    if let Some(f) = (*pr).write_func {
        f((*pr).opaque, addr as u32, val, size_log2);
    }
}

#[inline]
unsafe fn dev_read(pr: *mut PhysMemoryRange, addr: u64, size_log2: c_int) -> u32 {
    match (*pr).read_func {
        Some(f) => f((*pr).opaque, addr as u32, size_log2),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_mmio(s: *mut PCMachine, run: *mut KvmRun) {
    let mmio = &mut (*run).mmio;
    let data: *mut u8 = mmio.data.as_mut_ptr();
    let len = mmio.len;

    let pr = get_phys_mem_range((*s).mem_map, mmio.phys_addr);

    if mmio.is_write != 0 {
        if pr.is_null() || (*pr).is_ram != 0 {
            return;
        }
        let addr: u64 = mmio.phys_addr.wrapping_sub((*pr).addr);
        match len {
            1 => {
                if (*pr).devio_flags & (1 << 0) != 0 {
                    let val = core::ptr::read_unaligned(data as *const u8) as u32;
                    dev_write(pr, addr, val, 0);
                }
            }
            2 => {
                if (*pr).devio_flags & (1 << 1) != 0 {
                    let val = core::ptr::read_unaligned(data as *const u16) as u32;
                    dev_write(pr, addr, val, 1);
                }
            }
            4 => {
                if (*pr).devio_flags & (1 << 2) != 0 {
                    let val = core::ptr::read_unaligned(data as *const u32);
                    dev_write(pr, addr, val, 2);
                }
            }
            8 => {
                if (*pr).devio_flags & (1 << 2) != 0 {
                    let val = core::ptr::read_unaligned(data as *const u32);
                    dev_write(pr, addr, val, 2);
                    let val2 = core::ptr::read_unaligned(data.add(4) as *const u32);
                    dev_write(pr, addr.wrapping_add(4), val2, 2);
                }
            }
            _ => abort(),
        }
    } else {
        // "no_dev" target in the C code.
        if pr.is_null() || (*pr).is_ram != 0 {
            core::ptr::write_bytes(data, 0u8, len as usize);
            return;
        }
        let addr: u64 = mmio.phys_addr.wrapping_sub((*pr).addr);
        match len {
            1 => {
                if (*pr).devio_flags & (1 << 0) == 0 {
                    core::ptr::write_bytes(data, 0u8, len as usize);
                    return;
                }
                let v = dev_read(pr, addr, 0);
                core::ptr::write_unaligned(data as *mut u8, v as u8);
            }
            2 => {
                if (*pr).devio_flags & (1 << 1) == 0 {
                    core::ptr::write_bytes(data, 0u8, len as usize);
                    return;
                }
                let v = dev_read(pr, addr, 1);
                core::ptr::write_unaligned(data as *mut u16, v as u16);
            }
            4 => {
                if (*pr).devio_flags & (1 << 2) == 0 {
                    core::ptr::write_bytes(data, 0u8, len as usize);
                    return;
                }
                let v = dev_read(pr, addr, 2);
                core::ptr::write_unaligned(data as *mut u32, v);
            }
            8 => {
                if (*pr).devio_flags & (1 << 2) != 0 {
                    let v = dev_read(pr, addr, 2);
                    core::ptr::write_unaligned(data as *mut u32, v);
                    let v2 = dev_read(pr, addr.wrapping_add(4), 2);
                    core::ptr::write_unaligned(data.add(4) as *mut u32, v2);
                } else {
                    core::ptr::write_bytes(data, 0u8, len as usize);
                }
            }
            _ => abort(),
        }
    }
}
}

// ── `uint8_t *data = run->mmio.data; PhysMemoryRange *pr; uint64_t addr; pr = get_phys_mem_range(s->…` · x86_machine.c:1798-1802 · chunk 4200 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint8_t *data = run->mmio.data;
//         PhysMemoryRange *pr;
//         uint64_t addr;
//     
//         pr = get_phys_mem_range(s->mem_map, run->mmio.phys_addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4200 {
// Ported prologue of kvm_exit_mmio(): look up the physical memory range
// backing the MMIO access address. Both memory-map types are only ever
// handled through pointers here, so they stay opaque.

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub extern "C" fn kvm_exit_mmio__part4200(
    mem_map: *mut PhysMemoryMap,
    phys_addr: u64,
) -> *mut PhysMemoryRange {
    unsafe { get_phys_mem_range(mem_map, phys_addr) }
}
}

// ── `if (run->mmio.is_write) … else` · x86_machine.c:1803-1867 · chunk 4201 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (run->mmio.is_write) {
//             if (!pr || pr->is_ram)
//                 return;
//             addr = run->mmio.phys_addr - pr->addr;
//             switch(run->mmio.len) {
//             case 1:
//                 if (pr->devio_flags & (1 << 0)) {
//                     pr->write_func(pr->opaque, addr, *(uint8_t *)data, 0);
//                 }
//                 break;
//             case 2:
//                 if (pr->devio_flags & (1 << 1)) {
//                     pr->write_func(pr->opaque, addr, *(uint16_t *)data, 1);
//                 }
//                 break;
//             case 4:
//                 if (pr->devio_flags & (1 << 2)) {
//                     pr->write_func(pr->opaque, addr, *(uint32_t *)data, 2);
//                 }
//                 break;
//             case 8:
//                 if (pr->devio_flags & (1 << 2)) {
//                     pr->write_func(pr->opaque, addr, *(uint32_t *)data, 2);
//                     pr->write_func(pr->opaque, addr + 4, *(uint32_t *)(data + 4), 2);
//                 }
//                 break;
//             default:
//                 abort();
//             }
//         } else {
//             if (!pr || pr->is_ram)
//                 goto no_dev;
//             addr = run->mmio.phys_addr - pr->addr;
//             switch(run->mmio.len) {
//             case 1:
//                 if (!(pr->devio_flags & (1 << 0)))
//                     goto no_dev;
//                 *(uint8_t *)data = pr->read_func(pr->opaque, addr, 0);
//                 break;
//             case 2:
//                 if (!(pr->devio_flags & (1 << 1)))
//                     goto no_dev;
//                 *(uint16_t *)data = pr->read_func(pr->opaque, addr, 1);
//                 break;
//             case 4:
//                 if (!(pr->devio_flags & (1 << 2)))
//                     goto no_dev;
//                 *(uint32_t *)data = pr->read_func(pr->opaque, addr, 2);
//                 break;
//             case 8:
//                 if (pr->devio_flags & (1 << 2)) {
//                     *(uint32_t *)data =
//                         pr->read_func(pr->opaque, addr, 2);
//                     *(uint32_t *)(data + 4) =
//                         pr->read_func(pr->opaque, addr + 4, 2);
//                 } else {
//                 no_dev:
//                     memset(run->mmio.data, 0, run->mmio.len);
//                 }
//                 break;
//             default:
//                 abort();
//             }
//     
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4201 {
// Port of the TARGET (mmio write/read dispatch) of kvm_exit_mmio.
use core::ffi::{c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn abort() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn kvm_exit_mmio__part4201(
    data: *mut u8,
    phys_addr: u64,
    len: u32,
    is_write: u8,
    pr: *mut PhysMemoryRange,
) {
    if is_write != 0 {
        if pr.is_null() || (*pr).is_ram != 0 {
            return;
        }
        let addr: u64 = phys_addr.wrapping_sub((*pr).addr);
        match len {
            1 => {
                if (*pr).devio_flags & (1 << 0) != 0 {
                    if let Some(f) = (*pr).write_func {
                        let val = ptr::read_unaligned(data as *const u8) as u32;
                        f((*pr).opaque, addr as u32, val, 0);
                    }
                }
            }
            2 => {
                if (*pr).devio_flags & (1 << 1) != 0 {
                    if let Some(f) = (*pr).write_func {
                        let val = ptr::read_unaligned(data as *const u16) as u32;
                        f((*pr).opaque, addr as u32, val, 1);
                    }
                }
            }
            4 => {
                if (*pr).devio_flags & (1 << 2) != 0 {
                    if let Some(f) = (*pr).write_func {
                        let val = ptr::read_unaligned(data as *const u32);
                        f((*pr).opaque, addr as u32, val, 2);
                    }
                }
            }
            8 => {
                if (*pr).devio_flags & (1 << 2) != 0 {
                    if let Some(f) = (*pr).write_func {
                        let val0 = ptr::read_unaligned(data as *const u32);
                        f((*pr).opaque, addr as u32, val0, 2);
                        let val1 = ptr::read_unaligned(data.add(4) as *const u32);
                        f((*pr).opaque, addr.wrapping_add(4) as u32, val1, 2);
                    }
                }
            }
            _ => abort(),
        }
    } else {
        let mut no_dev = false;
        if pr.is_null() || (*pr).is_ram != 0 {
            no_dev = true;
        } else {
            let addr: u64 = phys_addr.wrapping_sub((*pr).addr);
            match len {
                1 => {
                    if (*pr).devio_flags & (1 << 0) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let val = f((*pr).opaque, addr as u32, 0);
                        ptr::write_unaligned(data as *mut u8, val as u8);
                    }
                }
                2 => {
                    if (*pr).devio_flags & (1 << 1) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let val = f((*pr).opaque, addr as u32, 1);
                        ptr::write_unaligned(data as *mut u16, val as u16);
                    }
                }
                4 => {
                    if (*pr).devio_flags & (1 << 2) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let val = f((*pr).opaque, addr as u32, 2);
                        ptr::write_unaligned(data as *mut u32, val);
                    }
                }
                8 => {
                    if (*pr).devio_flags & (1 << 2) != 0 {
                        if let Some(f) = (*pr).read_func {
                            let val0 = f((*pr).opaque, addr as u32, 2);
                            ptr::write_unaligned(data as *mut u32, val0);
                            let val1 = f((*pr).opaque, addr.wrapping_add(4) as u32, 2);
                            ptr::write_unaligned(data.add(4) as *mut u32, val1);
                        }
                    } else {
                        no_dev = true;
                    }
                }
                _ => abort(),
            }
        }
        if no_dev {
            memset(data as *mut c_void, 0, len as usize);
        }
    }
}
}

// ── `kvm_exec` · x86_machine.c:1870-1925 · chunk 4202 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void kvm_exec(PCMachine *s)
//     {
//         struct kvm_run *run = s->kvm_run;
//         struct itimerval ival;
//         int ret;
//     
//     
//     
//         ival.it_interval.tv_sec = 0;
//         ival.it_interval.tv_usec = 0;
//         ival.it_value.tv_sec = 0;
//         ival.it_value.tv_usec = 10 * 1000;
//         setitimer(
//                  ITIMER_REAL
//                             , &ival, 
//                                      ((void *)0)
//                                          );
//     
//         ret = ioctl(s->vcpu_fd, 
//                                (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x80)) << 0) | ((0) << ((0 +8)+8)))
//                                       , 0);
//         if (ret < 0) {
//             if (
//                (*__errno_location ()) 
//                      == 
//                         4 
//                               || 
//                                  (*__errno_location ()) 
//                                        == 
//                                           11
//                                                 ) {
//     
//                 return;
//             }
//             perror("KVM_RUN");
//             exit(1);
//         }
//     
//         switch(run->exit_reason) {
//         case 
//             5
//                         :
//             break;
//         case 
//             2
//                        :
//             kvm_exit_io(s, run);
//             break;
//         case 
//             6
//                          :
//             kvm_exit_mmio(s, run);
//             break;
//         case 
//             9
//                                :
//             fprintf(
//                    stderr
//                          , "KVM_EXIT_FAIL_ENTRY: reason=0x%" 
//                                                              "l" "x" 
//                                                                     "\n",
//                     (uint64_t)run->fail_entry.hardware_entry_failure_reason);
//             exit(1);
//         case 
//             17
//                                    :
//             fprintf(
//                    stderr
//                          , "KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n",
//                     (uint32_t)run->internal.suberror);
//             exit(1);
//         default:
//             fprintf(
//                    stderr
//                          , "KVM: unsupported exit_reason=%d\n", run->exit_reason);
//             exit(1);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4202 {
// Rust port of `static void kvm_exec(PCMachine *s)` from x86_machine.c.
// The static helpers it calls (kvm_exit_io, kvm_exit_mmio, st_port, ld_port)
// have no linker symbols, so their logic is ported here as private Rust fns.
// `get_phys_mem_range` is a real (non-static) C symbol and is declared extern.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// ---------------------------------------------------------------------------
// C constants (values taken from the preprocessed TARGET)
// ---------------------------------------------------------------------------

const ITIMER_REAL: c_int = 0;
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;

// _IO(KVMIO, 0x80) == 0xAE80
const KVM_RUN: c_ulong = ((0xAE as c_ulong) << 8) | 0x80;

const KVM_EXIT_IO: u32 = 2;
const KVM_EXIT_HLT: u32 = 5;
const KVM_EXIT_MMIO: u32 = 6;
const KVM_EXIT_FAIL_ENTRY: u32 = 9;
const KVM_EXIT_INTERNAL_ERROR: u32 = 17;

const KVM_EXIT_IO_IN: u8 = 1;

// ---------------------------------------------------------------------------
// C types
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: i64,  // __time_t == long int
    pub tv_usec: i64, // __suseconds_t == long int
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ITimerVal {
    pub it_interval: TimeVal,
    pub it_value: TimeVal,
}

// DeviceWriteFunc / DeviceReadFunc
pub type DeviceWriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;
pub type DeviceReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;

// Opaque forwards declarations used as pointer targets.
#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int, // BOOL
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: DeviceReadFunc,
    pub write_func: DeviceWriteFunc,
    pub devio_flags: c_int,
}

// ---- struct kvm_run -------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmHw {
    pub hardware_exit_reason: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmFailEntry {
    pub hardware_entry_failure_reason: u64,
    pub cpu: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmEx {
    pub exception: u32,
    pub error_code: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmIo {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmMmio {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmInternal {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KvmRunUnion {
    pub hw: KvmHw,
    pub fail_entry: KvmFailEntry,
    pub ex: KvmEx,
    pub io: KvmIo,
    pub mmio: KvmMmio,
    pub internal: KvmInternal,
    pub _pad: [u8; 256],
}

#[repr(C)]
pub struct KvmRun {
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,
    pub padding1: [u8; 6],
    pub exit_reason: u32,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,
    pub cr8: u64,
    pub apic_base: u64,
    pub u: KvmRunUnion,
}

// ---- VirtMachine / IRQSignal / PCMachine ---------------------------------

#[repr(C)]
pub struct VirtMachineClass {
    pub _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,         // EthernetDevice *
    pub console_dev: *mut c_void, // VIRTIODevice *
    pub console: *mut c_void,     // CharacterDevice *
    pub fb_dev: *mut c_void,      // FBDevice *
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,

    pub cpu_state: *mut c_void,   // X86CPUState *
    pub pic_state: *mut c_void,   // PIC2State *
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,   // PITState *
    pub i440fx_state: *mut c_void, // I440FXState *
    pub cmos_state: *mut c_void,  // CMOSState *
    pub serial_state: *mut c_void, // SerialState *

    pub keyboard_dev: *mut c_void, // VIRTIODevice *
    pub mouse_dev: *mut c_void,    // VIRTIODevice *
    pub kbd_state: *mut c_void,    // KBDState *
    pub ps2_mouse: *mut c_void,    // PS2MouseState *
    pub vm_mouse: *mut c_void,     // VMMouseState *
    pub ps2_kbd: *mut c_void,      // PS2KbdState *

    pub kvm_enabled: c_int, // BOOL
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

// ---------------------------------------------------------------------------
// extern C declarations
// ---------------------------------------------------------------------------

extern "C" {
    fn setitimer(which: c_int, new_value: *const ITimerVal, old_value: *mut ITimerVal) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn abort() -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

// ---------------------------------------------------------------------------
// ported static helpers
// ---------------------------------------------------------------------------

unsafe fn st_port(opaque: *mut c_void, port_in: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut PCMachine;
    if s.is_null() {
        return;
    }
    let pr = get_phys_mem_range((*s).port_map, port_in as u64);
    if pr.is_null() {
        return;
    }
    let port = (port_in as u64).wrapping_sub((*pr).addr) as u32;
    let flags = (*pr).devio_flags as u32;
    if (flags.wrapping_shr(size_log2 as u32) & 1) != 0 {
        if let Some(f) = (*pr).write_func {
            f((*pr).opaque, port, val, size_log2);
        }
    } else if size_log2 == 1 && (flags & 1) != 0 {
        if let Some(f) = (*pr).write_func {
            f((*pr).opaque, port, val & 0xff, 0);
            f((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
        }
    }
}

unsafe fn ld_port(opaque: *mut c_void, port_in: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut PCMachine;
    let val: u32;
    let mut port = port_in;
    let pr = get_phys_mem_range((*s).port_map, port as u64);
    if pr.is_null() {
        val = 0xffff_ffff;
    } else {
        port = (port as u64).wrapping_sub((*pr).addr) as u32;
        let flags = (*pr).devio_flags as u32;
        if (flags.wrapping_shr(size_log2 as u32) & 1) != 0 {
            val = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port, size_log2),
                None => 0,
            };
        } else if size_log2 == 1 && (flags & 1) != 0 {
            let lo = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port, 0) & 0xff,
                None => 0,
            };
            let hi = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port.wrapping_add(1), 0) & 0xff,
                None => 0,
            };
            val = lo | (hi << 8);
        } else {
            val = 0xffff_ffff;
        }
    }
    val
}

unsafe fn kvm_exit_io(s: *mut PCMachine, run: *mut KvmRun) {
    let ubase: *mut u8 = core::ptr::addr_of_mut!((*run).u) as *mut u8;
    let io = ubase as *const KvmIo;

    let mut ptr = (run as *mut u8).wrapping_add((*io).data_offset as usize);
    let count = (*io).count;
    let mut i: u32 = 0;
    while i < count {
        if (*io).direction == KVM_EXIT_IO_IN {
            match (*io).size {
                1 => {
                    let v = core::ptr::read_unaligned(ptr as *const u8) as u32;
                    st_port(s as *mut c_void, (*io).port as u32, v, 0);
                }
                2 => {
                    let v = core::ptr::read_unaligned(ptr as *const u16) as u32;
                    st_port(s as *mut c_void, (*io).port as u32, v, 1);
                }
                4 => {
                    let v = core::ptr::read_unaligned(ptr as *const u32);
                    st_port(s as *mut c_void, (*io).port as u32, v, 2);
                }
                _ => abort(),
            }
        } else {
            match (*io).size {
                1 => {
                    let v = ld_port(s as *mut c_void, (*io).port as u32, 0) as u8;
                    core::ptr::write_unaligned(ptr as *mut u8, v);
                }
                2 => {
                    let v = ld_port(s as *mut c_void, (*io).port as u32, 1) as u16;
                    core::ptr::write_unaligned(ptr as *mut u16, v);
                }
                4 => {
                    let v = ld_port(s as *mut c_void, (*io).port as u32, 2);
                    core::ptr::write_unaligned(ptr as *mut u32, v);
                }
                _ => abort(),
            }
        }
        ptr = ptr.wrapping_add((*io).size as usize);
        i = i.wrapping_add(1);
    }
}

unsafe fn kvm_exit_mmio(s: *mut PCMachine, run: *mut KvmRun) {
    let ubase: *mut u8 = core::ptr::addr_of_mut!((*run).u) as *mut u8;
    let mmio = ubase as *mut KvmMmio;
    let data = core::ptr::addr_of_mut!((*mmio).data) as *mut u8;

    let pr = get_phys_mem_range((*s).mem_map, (*mmio).phys_addr);
    if (*mmio).is_write != 0 {
        if pr.is_null() || (*pr).is_ram != 0 {
            return;
        }
        let addr = (*mmio).phys_addr.wrapping_sub((*pr).addr);
        let flags = (*pr).devio_flags;
        match (*mmio).len {
            1 => {
                if (flags & (1 << 0)) != 0 {
                    if let Some(f) = (*pr).write_func {
                        f(
                            (*pr).opaque,
                            addr as u32,
                            core::ptr::read_unaligned(data as *const u8) as u32,
                            0,
                        );
                    }
                }
            }
            2 => {
                if (flags & (1 << 1)) != 0 {
                    if let Some(f) = (*pr).write_func {
                        f(
                            (*pr).opaque,
                            addr as u32,
                            core::ptr::read_unaligned(data as *const u16) as u32,
                            1,
                        );
                    }
                }
            }
            4 => {
                if (flags & (1 << 2)) != 0 {
                    if let Some(f) = (*pr).write_func {
                        f(
                            (*pr).opaque,
                            addr as u32,
                            core::ptr::read_unaligned(data as *const u32),
                            2,
                        );
                    }
                }
            }
            8 => {
                if (flags & (1 << 2)) != 0 {
                    if let Some(f) = (*pr).write_func {
                        f(
                            (*pr).opaque,
                            addr as u32,
                            core::ptr::read_unaligned(data as *const u32),
                            2,
                        );
                        f(
                            (*pr).opaque,
                            addr.wrapping_add(4) as u32,
                            core::ptr::read_unaligned(data.add(4) as *const u32),
                            2,
                        );
                    }
                }
            }
            _ => abort(),
        }
    } else {
        // read direction; mimic the C `goto no_dev`
        let mut no_dev = false;
        if pr.is_null() || (*pr).is_ram != 0 {
            no_dev = true;
        }
        if !no_dev {
            let addr = (*mmio).phys_addr.wrapping_sub((*pr).addr);
            let flags = (*pr).devio_flags;
            match (*mmio).len {
                1 => {
                    if (flags & (1 << 0)) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let v = f((*pr).opaque, addr as u32, 0) as u8;
                        core::ptr::write_unaligned(data as *mut u8, v);
                    }
                }
                2 => {
                    if (flags & (1 << 1)) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let v = f((*pr).opaque, addr as u32, 1) as u16;
                        core::ptr::write_unaligned(data as *mut u16, v);
                    }
                }
                4 => {
                    if (flags & (1 << 2)) == 0 {
                        no_dev = true;
                    } else if let Some(f) = (*pr).read_func {
                        let v = f((*pr).opaque, addr as u32, 2);
                        core::ptr::write_unaligned(data as *mut u32, v);
                    }
                }
                8 => {
                    if (flags & (1 << 2)) != 0 {
                        if let Some(f) = (*pr).read_func {
                            let v0 = f((*pr).opaque, addr as u32, 2);
                            core::ptr::write_unaligned(data as *mut u32, v0);
                            let v1 = f((*pr).opaque, addr.wrapping_add(4) as u32, 2);
                            core::ptr::write_unaligned(data.add(4) as *mut u32, v1);
                        }
                    } else {
                        no_dev = true;
                    }
                }
                _ => abort(),
            }
        }
        if no_dev {
            core::ptr::write_bytes(data, 0, (*mmio).len as usize);
        }
    }
}

// ---------------------------------------------------------------------------
// the TARGET
// ---------------------------------------------------------------------------

const FMT_FAIL_ENTRY: &[u8] = b"KVM_EXIT_FAIL_ENTRY: reason=0x%lx\n\0";
const FMT_INTERNAL_ERROR: &[u8] = b"KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n\0";
const FMT_UNSUPPORTED: &[u8] = b"KVM: unsupported exit_reason=%d\n\0";
const S_KVM_RUN: &[u8] = b"KVM_RUN\0";

#[no_mangle]
pub extern "C" fn kvm_exec(s: *mut PCMachine) {
    unsafe {
        if s.is_null() {
            return;
        }
        let run = (*s).kvm_run;

        // Not efficient but simple: use a timer to interrupt execution.
        let ival = ITimerVal {
            it_interval: TimeVal {
                tv_sec: 0,
                tv_usec: 0,
            },
            it_value: TimeVal {
                tv_sec: 0,
                tv_usec: 10 * 1000,
            },
        };
        setitimer(ITIMER_REAL, &ival, core::ptr::null_mut());

        let ret = ioctl((*s).vcpu_fd, KVM_RUN, 0 as c_int);
        if ret < 0 {
            let err = *__errno_location();
            if err == EINTR || err == EAGAIN {
                // timeout
                return;
            }
            perror(S_KVM_RUN.as_ptr() as *const c_char);
            exit(1);
        }

        let reason = (*run).exit_reason;
        if reason == KVM_EXIT_HLT {
            // nothing to do
        } else if reason == KVM_EXIT_IO {
            kvm_exit_io(s, run);
        } else if reason == KVM_EXIT_MMIO {
            kvm_exit_mmio(s, run);
        } else if reason == KVM_EXIT_FAIL_ENTRY {
            let ubase: *mut u8 = core::ptr::addr_of_mut!((*run).u) as *mut u8;
            let fe = ubase as *const KvmFailEntry;
            fprintf(
                stderr,
                FMT_FAIL_ENTRY.as_ptr() as *const c_char,
                (*fe).hardware_entry_failure_reason as u64,
            );
            exit(1);
        } else if reason == KVM_EXIT_INTERNAL_ERROR {
            let ubase: *mut u8 = core::ptr::addr_of_mut!((*run).u) as *mut u8;
            let ie = ubase as *const KvmInternal;
            fprintf(
                stderr,
                FMT_INTERNAL_ERROR.as_ptr() as *const c_char,
                (*ie).suberror,
            );
            exit(1);
        } else {
            fprintf(
                stderr,
                FMT_UNSUPPORTED.as_ptr() as *const c_char,
                reason as c_int,
            );
            exit(1);
        }
    }
}
}

// ── `struct kvm_run *run = s->kvm_run; struct itimerval ival; int ret; ival.it_interval.tv_sec = 0;…` · x86_machine.c:1872-1884 · chunk 4203 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct kvm_run *run = s->kvm_run;
//         struct itimerval ival;
//         int ret;
//     
//     
//     
//         ival.it_interval.tv_sec = 0;
//         ival.it_interval.tv_usec = 0;
//         ival.it_value.tv_sec = 0;
//         ival.it_value.tv_usec = 10 * 1000;
//         setitimer(
//                  ITIMER_REAL
//                             , &ival, 
//                                      ((void *)0)
//                                          );
//     
//         ret = ioctl(s->vcpu_fd, 
//                                (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x80)) << 0) | ((0) << ((0 +8)+8)))
//                                       , 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4203 {
// Ported piece of kvm_exec: arm the 10ms ITIMER_REAL and issue the KVM_RUN ioctl.
use core::ffi::{c_int, c_long, c_ulong};

#[repr(C)]
struct Timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct Itimerval {
    it_interval: Timeval,
    it_value: Timeval,
}

extern "C" {
    fn setitimer(
        which: c_int,
        new_value: *const Itimerval,
        old_value: *mut Itimerval,
    ) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn kvm_exec__part4203(vcpu_fd: c_int) -> c_int {
    let ival = Itimerval {
        it_interval: Timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: Timeval {
            tv_sec: 0,
            tv_usec: 10 * 1000,
        },
    };

    unsafe {
        // ITIMER_REAL == 0, no old value wanted.
        setitimer(0, &ival, core::ptr::null_mut());
        // KVM_RUN request number, third argument is the (unused) NULL vcpu pointer.
        ioctl(vcpu_fd, 0xAE80 as c_ulong, 0 as c_int)
    }
}
}

// ── `if (ret < 0)` · x86_machine.c:1885-1892 · chunk 4204 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0) {
//             if (
//                (*__errno_location ()) 
//                      == 
//                         4 
//                               || 
//                                  (*__errno_location ()) 
//                                        == 
//                                           11
//                                                 ) {
//     
//                 return;
//             }
//             perror("KVM_RUN");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4204 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

static KVM_RUN_MSG: [u8; 8] = *b"KVM_RUN\0";

/// Ported piece of kvm_exec: handles the failure of the KVM_RUN ioctl.
/// Returns 1 when the caller must `return` immediately, 0 otherwise.
#[no_mangle]
pub extern "C" fn kvm_exec__part4204(ret: c_int) -> c_int {
    if ret < 0 {
        unsafe {
            let e1 = *__errno_location();
            let e2 = *__errno_location();
            if e1 == 4 || e2 == 11 {
                return 1;
            }
            perror(KVM_RUN_MSG.as_ptr() as *const c_char);
            exit(1);
        }
    }
    0
}
}

// ── `switch (run->exit_reason)` · x86_machine.c:1894-1924 · chunk 4205 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch(run->exit_reason) {
//         case 
//             5
//                         :
//             break;
//         case 
//             2
//                        :
//             kvm_exit_io(s, run);
//             break;
//         case 
//             6
//                          :
//             kvm_exit_mmio(s, run);
//             break;
//         case 
//             9
//                                :
//             fprintf(
//                    stderr
//                          , "KVM_EXIT_FAIL_ENTRY: reason=0x%" 
//                                                              "l" "x" 
//                                                                     "\n",
//                     (uint64_t)run->fail_entry.hardware_entry_failure_reason);
//             exit(1);
//         case 
//             17
//                                    :
//             fprintf(
//                    stderr
//                          , "KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n",
//                     (uint32_t)run->internal.suberror);
//             exit(1);
//         default:
//             fprintf(
//                    stderr
//                          , "KVM: unsupported exit_reason=%d\n", run->exit_reason);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4205 {
// Port of the KVM_RUN exit-reason switch of kvm_exec() from x86_machine.c.
// The C helpers kvm_exit_io(), kvm_exit_mmio(), st_port() and ld_port() are
// file-static, so their logic is reimplemented here as private Rust fns.

use core::ffi::{c_char, c_int, c_uint, c_void};

// ---------------------------------------------------------------- opaque C types

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// PhysMemoryRange (iomem.h)
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

// struct kvm_run (linux/kvm.h): laid out exactly as in C.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmRunFailEntry {
    pub hardware_entry_failure_reason: u64,
    pub cpu: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmRunInternal {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmRunIo {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmRunMmio {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KvmRunUnion {
    pub hw: u64,
    pub fail_entry: KvmRunFailEntry,
    pub io: KvmRunIo,
    pub mmio: KvmRunMmio,
    pub internal: KvmRunInternal,
    pub padding: [u8; 256],
}

#[repr(C)]
pub struct KvmRun {
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,
    pub padding1: [u8; 6],
    pub exit_reason: u32,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,
    pub cr8: u64,
    pub apic_base: u64,
    pub u: KvmRunUnion,
    pub kvm_valid_regs: u64,
    pub kvm_dirty_regs: u64,
    pub s_padding: [u8; 2048],
}

// ---------------------------------------------------------------- extern C symbols

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn abort() -> !;
    static stderr: *mut FILE;
}

// ---------------------------------------------------------------- static helpers

const KVM_EXIT_IO_OUT: u8 = 1;

unsafe fn do_write(pr: *mut PhysMemoryRange, offset: u32, val: u32, size_log2: c_int) {
    let opaque = (*pr).opaque;
    match (*pr).write_func {
        Some(f) => f(opaque, offset, val, size_log2),
        None => abort(),
    }
}

unsafe fn do_read(pr: *mut PhysMemoryRange, offset: u32, size_log2: c_int) -> u32 {
    let opaque = (*pr).opaque;
    match (*pr).read_func {
        Some(f) => f(opaque, offset, size_log2),
        None => abort(),
    }
}

// static void st_port(void *opaque, uint32_t port, uint32_t val, int size_log2)
unsafe fn st_port(port_map: *mut PhysMemoryMap, port_in: u32, val: u32, size_log2: c_int) {
    let mut port = port_in;
    let pr = get_phys_mem_range(port_map, port as u64);
    if pr.is_null() {
        return;
    }
    port = (port as u64).wrapping_sub((*pr).addr) as u32;
    let devio_flags = (*pr).devio_flags;
    if ((devio_flags as u32) >> (size_log2 as u32)) & 1 != 0 {
        do_write(pr, port, val, size_log2);
    } else if size_log2 == 1 && (devio_flags & (1 << 0)) != 0 {
        do_write(pr, port, val & 0xff, 0);
        do_write(pr, port.wrapping_add(1), (val >> 8) & 0xff, 0);
    }
}

// static uint32_t ld_port(void *opaque, uint32_t port1, int size_log2)
unsafe fn ld_port(port_map: *mut PhysMemoryMap, port1: u32, size_log2: c_int) -> u32 {
    let val: u32;
    let mut port = port1;
    let pr = get_phys_mem_range(port_map, port as u64);
    if pr.is_null() {
        val = 0xffff_ffff;
    } else {
        port = (port as u64).wrapping_sub((*pr).addr) as u32;
        let devio_flags = (*pr).devio_flags;
        if ((devio_flags as u32) >> (size_log2 as u32)) & 1 != 0 {
            val = do_read(pr, port, size_log2);
        } else if size_log2 == 1 && (devio_flags & (1 << 0)) != 0 {
            let mut v = do_read(pr, port, 0) & 0xff;
            v |= (do_read(pr, port.wrapping_add(1), 0) & 0xff) << 8;
            val = v;
        } else {
            val = 0xffff_ffff;
        }
    }
    val
}

// static void kvm_exit_io(PCMachine *s, struct kvm_run *run)
unsafe fn kvm_exit_io(port_map: *mut PhysMemoryMap, run: *mut KvmRun) {
    let data_offset = (*run).u.io.data_offset;
    let count = (*run).u.io.count;
    let mut ptr = (run as *mut u8).wrapping_add(data_offset as usize);
    let mut i: c_uint = 0;
    while i < count {
        let io = (*run).u.io;
        let port = io.port as u32;
        if io.direction == KVM_EXIT_IO_OUT {
            match io.size {
                1 => {
                    let v = core::ptr::read_unaligned(ptr as *const u8) as u32;
                    st_port(port_map, port, v, 0);
                }
                2 => {
                    let v = core::ptr::read_unaligned(ptr as *const u16) as u32;
                    st_port(port_map, port, v, 1);
                }
                4 => {
                    let v = core::ptr::read_unaligned(ptr as *const u32);
                    st_port(port_map, port, v, 2);
                }
                _ => abort(),
            }
        } else {
            match io.size {
                1 => {
                    let v = ld_port(port_map, port, 0) as u8;
                    core::ptr::write_unaligned(ptr as *mut u8, v);
                }
                2 => {
                    let v = ld_port(port_map, port, 1) as u16;
                    core::ptr::write_unaligned(ptr as *mut u16, v);
                }
                4 => {
                    let v = ld_port(port_map, port, 2);
                    core::ptr::write_unaligned(ptr as *mut u32, v);
                }
                _ => abort(),
            }
        }
        ptr = ptr.wrapping_add(io.size as usize);
        i += 1;
    }
}

// static void kvm_exit_mmio(PCMachine *s, struct kvm_run *run)
unsafe fn kvm_exit_mmio(mem_map: *mut PhysMemoryMap, run: *mut KvmRun) {
    let mmio = (*run).u.mmio;
    let data = (*run).u.mmio.data.as_mut_ptr();
    let pr = get_phys_mem_range(mem_map, mmio.phys_addr);

    if mmio.is_write != 0 {
        if pr.is_null() || (*pr).is_ram != 0 {
            return;
        }
        let addr = mmio.phys_addr.wrapping_sub((*pr).addr);
        let devio_flags = (*pr).devio_flags;
        match mmio.len {
            1 => {
                if (devio_flags & (1 << 0)) != 0 {
                    let v = core::ptr::read_unaligned(data as *const u8) as u32;
                    do_write(pr, addr as u32, v, 0);
                }
            }
            2 => {
                if (devio_flags & (1 << 1)) != 0 {
                    let v = core::ptr::read_unaligned(data as *const u16) as u32;
                    do_write(pr, addr as u32, v, 1);
                }
            }
            4 => {
                if (devio_flags & (1 << 2)) != 0 {
                    let v = core::ptr::read_unaligned(data as *const u32);
                    do_write(pr, addr as u32, v, 2);
                }
            }
            8 => {
                if (devio_flags & (1 << 2)) != 0 {
                    let v = core::ptr::read_unaligned(data as *const u32);
                    do_write(pr, addr as u32, v, 2);
                    let v2 = core::ptr::read_unaligned(data.wrapping_add(4) as *const u32);
                    do_write(pr, addr.wrapping_add(4) as u32, v2, 2);
                }
            }
            _ => abort(),
        }
    } else {
        // "goto no_dev" targets memset(run->mmio.data, 0, run->mmio.len)
        let mut no_dev = false;
        if pr.is_null() || (*pr).is_ram != 0 {
            no_dev = true;
        } else {
            let addr = mmio.phys_addr.wrapping_sub((*pr).addr);
            let devio_flags = (*pr).devio_flags;
            match mmio.len {
                1 => {
                    if (devio_flags & (1 << 0)) == 0 {
                        no_dev = true;
                    } else {
                        let v = do_read(pr, addr as u32, 0) as u8;
                        core::ptr::write_unaligned(data as *mut u8, v);
                    }
                }
                2 => {
                    if (devio_flags & (1 << 1)) == 0 {
                        no_dev = true;
                    } else {
                        let v = do_read(pr, addr as u32, 1) as u16;
                        core::ptr::write_unaligned(data as *mut u16, v);
                    }
                }
                4 => {
                    if (devio_flags & (1 << 2)) == 0 {
                        no_dev = true;
                    } else {
                        let v = do_read(pr, addr as u32, 2);
                        core::ptr::write_unaligned(data as *mut u32, v);
                    }
                }
                8 => {
                    if (devio_flags & (1 << 2)) != 0 {
                        let v = do_read(pr, addr as u32, 2);
                        core::ptr::write_unaligned(data as *mut u32, v);
                        let v2 = do_read(pr, addr.wrapping_add(4) as u32, 2);
                        core::ptr::write_unaligned(data.wrapping_add(4) as *mut u32, v2);
                    } else {
                        no_dev = true;
                    }
                }
                _ => abort(),
            }
        }
        if no_dev {
            core::ptr::write_bytes(data, 0u8, mmio.len as usize);
        }
    }
}

// ---------------------------------------------------------------- exported piece

#[no_mangle]
pub unsafe extern "C" fn kvm_exec__part4205(
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    run: *mut KvmRun,
    exit_reason: u32,
) {
    match exit_reason {
        5 => {}
        2 => kvm_exit_io(port_map, run),
        6 => kvm_exit_mmio(mem_map, run),
        9 => {
            let reason = (*run).u.fail_entry.hardware_entry_failure_reason;
            fprintf(
                stderr,
                b"KVM_EXIT_FAIL_ENTRY: reason=0x%lx\n\0".as_ptr() as *const c_char,
                reason,
            );
            exit(1);
        }
        17 => {
            let suberror = (*run).u.internal.suberror;
            fprintf(
                stderr,
                b"KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n\0".as_ptr() as *const c_char,
                suberror,
            );
            exit(1);
        }
        _ => {
            fprintf(
                stderr,
                b"KVM: unsupported exit_reason=%d\n\0".as_ptr() as *const c_char,
                exit_reason,
            );
            exit(1);
        }
    }
}
}

// ── `cpu_get_tsc` · x86_machine.c:1943-1950 · chunk 4206 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t cpu_get_tsc(void *opaque)
//     {
//         struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 100000000 +
//             (ts.tv_nsec / (1000000000 / 100000000));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4206 {
// Rust port of cpu_get_tsc from x86_machine.c
#![allow(non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,  // __time_t  == long int
    tv_nsec: c_long, // __syscall_slong_t == long int
}

const TSC_FREQ: u64 = 100_000_000;
const NS_PER_TSC: u64 = 1_000_000_000 / TSC_FREQ; // 10
const CLOCK_MONOTONIC: c_int = 1;

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cpu_get_tsc(_opaque: *mut c_void) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);

    let sec = ts.tv_sec as u64;
    let nsec = ts.tv_nsec as u64;

    sec.wrapping_mul(TSC_FREQ)
        .wrapping_add(nsec.wrapping_div(NS_PER_TSC))
}
}

// ── `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 100000000 + (ts.tv_ns…` · x86_machine.c:1945-1949 · chunk 4207 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 100000000 +
//             (ts.tv_nsec / (1000000000 / 100000000));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4207 {
// Ported piece of cpu_get_tsc: read the monotonic-ish clock (CLOCK_REALTIME == 1)
// and convert it to a 100 MHz timestamp.
#![allow(non_snake_case)]

use core::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,  /* __time_t = long int */
    tv_nsec: c_long, /* __syscall_slong_t = long int */
}

extern "C" {
    // extern int clock_gettime (clockid_t __clock_id, struct timespec *__tp);
    fn clock_gettime(__clock_id: c_int, __tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn cpu_get_tsc__part4207() -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(1, &mut ts);
    }
    (ts.tv_sec as u64).wrapping_mul(100000000u64)
        + ((ts.tv_nsec / (1000000000i64 / 100000000i64)) as u64)
}
}

// ── `pc_flush_tlb_write_range` · x86_machine.c:1953-1958 · chunk 4208 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_flush_tlb_write_range(void *opaque, uint8_t *ram_addr,
//                                          size_t ram_size)
//     {
//         PCMachine *s = opaque;
//         x86_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4208 {
use core::ffi::{c_int, c_void};

// Opaque X86CPUState type: never dereferenced here.
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut X86CPUState,
    pic_state: *mut c_void,
    pic_irq: [c_int; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

// VirtMachine layout is not needed: PCMachine is only accessed via its
// cpu_state field. Use a sized opaque array large enough to precede it.
#[repr(C)]
pub struct VirtMachine {
    _private: [u8; 0],
}

extern "C" {
    fn x86_cpu_flush_tlb_write_range_ram(
        s: *mut X86CPUState,
        ram_ptr: *mut u8,
        ram_size: usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pc_flush_tlb_write_range(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut PCMachine;
    let cpu_state = (*s).cpu_state;
    x86_cpu_flush_tlb_write_range_ram(cpu_state, ram_addr, ram_size);
}
}

// ── `PCMachine *s = opaque; x86_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);` · x86_machine.c:1956-1957 · chunk 4209 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCMachine *s = opaque;
//         x86_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4209 {
// Ported piece of pc_flush_tlb_write_range.
// Reads PCMachine::cpu_state through the opaque machine pointer and forwards
// the write range to the x86 TLB flusher.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct IRQSignal {
    set_irq: *mut c_void,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IRQSignal; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

extern "C" {
    fn x86_cpu_flush_tlb_write_range_ram(
        s: *mut c_void,
        ram_ptr: *mut u8,
        ram_size: usize,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pc_flush_tlb_write_range__part4209(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut PCMachine;
    let cpu_state = (*s).cpu_state;
    x86_cpu_flush_tlb_write_range_ram(cpu_state, ram_addr, ram_size);
}
}

// ── `pc_machine_init` · x86_machine.c:1960-2162 · chunk 4210 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static VirtMachine *pc_machine_init(const VirtMachineParams *p)
//     {
//         PCMachine *s;
//         int i, piix3_devfn;
//         PCIBus *pci_bus;
//         VIRTIOBusDef vbus_s, *vbus = &vbus_s;
//     
//         if (strcmp(p->machine_name, "pc") != 0) {
//             vm_error("unsupported machine: %s\n", p->machine_name);
//             return 
//                   ((void *)0)
//                       ;
//         }
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        p->ram_size >= (1 << 20)
//        )), __extension__ ({ if (
//        p->ram_size >= (1 << 20)
//        ) ; else __assert_fail (
//        "p->ram_size >= (1 << 20)"
//        , "x86_machine.c", 1972, __extension__ __PRETTY_FUNCTION__); }))
//                                        ;
//     
//         s = mallocz(sizeof(*s));
//         s->common.vmc = p->vmc;
//         s->ram_size = p->ram_size;
//     
//         s->port_map = phys_mem_map_init();
//         s->mem_map = phys_mem_map_init();
//     
//     
//         if (p->accel_enable) {
//             kvm_init(s);
//         }
//     
//     
//     
//         if (!s->kvm_enabled)
//     
//         {
//             s->cpu_state = x86_cpu_init(s->mem_map);
//             x86_cpu_set_get_tsc(s->cpu_state, cpu_get_tsc, s);
//             x86_cpu_set_port_io(s->cpu_state, ld_port, st_port, s);
//     
//     
//             s->mem_map->opaque = s;
//             s->mem_map->flush_tlb_write_range = pc_flush_tlb_write_range;
//         }
//     
//     
//         cpu_register_ram(s->mem_map, 0xc0000, p->ram_size - 0xc0000, 0);
//         cpu_register_ram(s->mem_map, 0, 0xa0000, 0);
//     
//     
//         cpu_register_device(s->port_map, 0x80, 2, s, port80_read, port80_write,
//                             (1 << 0));
//         cpu_register_device(s->port_map, 0x92, 2, s, port92_read, port92_write,
//                             (1 << 0));
//     
//     
//         if (p->files[VM_FILE_BIOS].len > 0) {
//             int bios_size, bios_size1;
//             uint8_t *bios_buf, *ptr;
//             uint32_t bios_addr;
//     
//             bios_size = p->files[VM_FILE_BIOS].len;
//             bios_buf = p->files[VM_FILE_BIOS].buf;
//             
//            ((void) sizeof (__assert_single_arg (
//            (bios_size % 65536) == 0 && bios_size != 0
//            )), __extension__ ({ if (
//            (bios_size % 65536) == 0 && bios_size != 0
//            ) ; else __assert_fail (
//            "(bios_size % 65536) == 0 && bios_size != 0"
//            , "x86_machine.c", 2018, __extension__ __PRETTY_FUNCTION__); }))
//                                                              ;
//             bios_addr = -bios_size;
//     
//             cpu_register_ram(s->mem_map, bios_addr, bios_size, (1 << 0));
//             ptr = get_ram_ptr(s, bios_addr);
//             memcpy(ptr, bios_buf, bios_size);
//     
//             bios_size1 = min_int(bios_size, 128 * 1024);
//             ptr = get_ram_ptr(s, 0x100000 - bios_size1);
//             memcpy(ptr, bios_buf + bios_size - bios_size1, bios_size1);
//     
//     
//     
//     
//     
//         }
//     
//     
//         if (!s->kvm_enabled)
//     
//         {
//             s->pic_state = pic2_init(s->port_map, 0x20, 0xa0,
//                                      0x4d0, 0x4d1,
//                                      pic_set_irq_cb, s,
//                                      s->pic_irq);
//             x86_cpu_set_get_hard_intno(s->cpu_state, get_hard_intno_cb, s);
//             s->pit_state = pit_init(s->port_map, 0x40, 0x61, &s->pic_irq[0],
//                                     pit_get_ticks_cb, s);
//         }
//     
//         s->cmos_state = cmos_init(s->port_map, 0x70, &s->pic_irq[8],
//                                   p->rtc_local_time);
//     
//     
//         {
//             int size;
//     
//             size = min_int((s->ram_size - (1 << 20)) >> 10, 65535);
//             put_le16(s->cmos_state->cmos_data + 0x30, size);
//             if (s->ram_size >= (16 << 20)) {
//                 size = min_int((s->ram_size - (16 << 20)) >> 16, 65535);
//                 put_le16(s->cmos_state->cmos_data + 0x34, size);
//             }
//             s->cmos_state->cmos_data[0x14] = 0x06;
//         }
//     
//         s->i440fx_state = i440fx_init(&pci_bus, &piix3_devfn, s->mem_map,
//                                       s->port_map, s->pic_irq);
//     
//         s->common.console = p->console;
//     
//         if (0) {
//         s->serial_state = serial_init(s->port_map, 0x3f8, &s->pic_irq[4],
//                                       serial_write_cb, s);
//         }
//     
//         memset(vbus, 0, sizeof(*vbus));
//         vbus->pci_bus = pci_bus;
//     
//         if (p->console) {
//     
//             s->common.console_dev = virtio_console_init(vbus, p->console);
//         }
//     
//     
//         for(i = 0; i < p->drive_count;) {
//             const VMDriveEntry *de = &p->tab_drive[i];
//     
//             if (!de->device || !strcmp(de->device, "virtio")) {
//                 virtio_block_init(vbus, p->tab_drive[i].block_dev);
//                 i++;
//             } else if (!strcmp(de->device, "ide")) {
//                 BlockDevice *tab_bs[2];
//     
//                 tab_bs[0] = p->tab_drive[i++].block_dev;
//                 tab_bs[1] = 
//                            ((void *)0)
//                                ;
//                 if (i < p->drive_count)
//                     tab_bs[1] = p->tab_drive[i++].block_dev;
//                 ide_init(s->port_map, 0x1f0, 0x3f6, &s->pic_irq[14], tab_bs);
//                 piix3_ide_init(pci_bus, piix3_devfn + 1);
//             }
//         }
//     
//     
//         for(i = 0; i < p->fs_count; i++) {
//             virtio_9p_init(vbus, p->tab_fs[i].fs_dev,
//                            p->tab_fs[i].tag);
//         }
//     
//         if (p->display_device) {
//             FBDevice *fb_dev;
//     
//             fb_dev = mallocz(sizeof(*fb_dev));
//             s->common.fb_dev = fb_dev;
//             if (!strcmp(p->display_device, "vga")) {
//                 int bios_size;
//                 uint8_t *bios_buf;
//                 bios_size = p->files[VM_FILE_VGA_BIOS].len;
//                 bios_buf = p->files[VM_FILE_VGA_BIOS].buf;
//                 pci_vga_init(pci_bus, fb_dev, p->width, p->height,
//                              bios_buf, bios_size);
//             } else if (!strcmp(p->display_device, "simplefb")) {
//                 simplefb_init(s->mem_map,
//                               0xf0400000,
//                               fb_dev, p->width, p->height);
//             } else {
//                 vm_error("unsupported display device: %s\n", p->display_device);
//                 exit(1);
//             }
//         }
//     
//         if (p->input_device) {
//             if (!strcmp(p->input_device, "virtio")) {
//                 s->keyboard_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_KEYBOARD);
//     
//                 s->mouse_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_TABLET);
//             } else if (!strcmp(p->input_device, "ps2")) {
//                 s->kbd_state = i8042_init(&s->ps2_kbd, &s->ps2_mouse,
//                                           s->port_map,
//                                           &s->pic_irq[1], &s->pic_irq[12], 0x60);
//     
//                 cpu_register_device(s->port_map, 0x5658, 1, s,
//                                     vmport_read, vmport_write,
//                         (1 << 2));
//                 s->vm_mouse = vmmouse_init(s->ps2_mouse);
//             } else {
//                 vm_error("unsupported input device: %s\n", p->input_device);
//                 exit(1);
//             }
//         }
//     
//     
//         for(i = 0; i < p->eth_count; i++) {
//             virtio_net_init(vbus, p->tab_eth[i].net);
//             s->common.net = p->tab_eth[i].net;
//         }
//     
//         if (p->files[VM_FILE_KERNEL].buf) {
//             copy_kernel(s, p->files[VM_FILE_KERNEL].buf,
//                         p->files[VM_FILE_KERNEL].len,
//                         p->cmdline ? p->cmdline : "");
//         }
//     
//         return (VirtMachine *)s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4210 {
// Rust reimplementation of pc_machine_init (x86_machine.c) plus the file-static
// helpers that it referenced (they have no linker symbols, so their logic is
// reproduced here).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
#![allow(unused_variables, unused_assignments, clippy::all)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/************************************************************************/
/* opaque C types                                                       */

#[repr(C)]
pub struct VirtMachineClass {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PCIBus {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PCIDevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct BlockDevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct X86CPUState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PITState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct I440FXState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct SerialState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct VIRTIODevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct KBDState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PS2MouseState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct PS2KbdState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct VMMouseState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct FSDevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct CharacterDevice {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct VGAState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct SimpleFBState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct IDEIFState {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct KvmRun {
    _unused: [u8; 0],
}

/************************************************************************/
/* mirrored C structures                                                */

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct SigSet {
    pub val: [u64; 16],
}

#[repr(C)]
pub struct SigAction {
    pub handler: Option<unsafe extern "C" fn(c_int)>,
    pub mask: SigSet,
    pub flags: c_int,
    pub restorer: u64,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct X86CPUSeg {
    pub sel: u16,
    pub flags: u16,
    pub base: u32,
    pub limit: u32,
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut PCIBus,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, *mut c_void)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut FSDevice,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

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
    pub irqs: *mut IRQSignal,
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

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

/* linux boot parameters (as declared, with __attribute__((packed))) */
#[repr(C)]
pub struct LinuxE820Entry {
    pub addr: u64,
    pub size: u64,
    pub type_: u32,
}

#[repr(C, packed)]
pub struct LinuxParams {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: u16,
    pub unused3: u16,
    pub orig_video_lines: u8,
    pub orig_video_is_vga: u8,
    pub orig_video_points: u16,
    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub cl_magic: u16,
    pub cl_offset: u16,
    pub lfb_linelength: u16,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: u16,
    pub vesapm_off: u16,
    pub pages: u16,
    pub vesa_attributes: u16,
    pub capabilities: u32,
    pub ext_lfb_base: u32,
    pub res0: [u8; 2],
    pub apm_bios_info: [u8; 20],
    pub pad2: [u8; 0x80 - 0x54],
    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],
    pub sys_description_len: u16,
    pub sys_description_table: [u8; 14],
    pub pad3: [u8; 0x1e0 - 0xb0],
    pub alt_mem_k: u32,
    pub pad4: [u8; 4],
    pub e820map_entries: u8,
    pub eddbuf_entries: u8,
    pub pad5: [u8; 0x1f1 - 0x1ea],
    pub setup_sects: u8,
    pub mount_root_rdonly: u16,
    pub sys_size: u16,
    pub swap_dev: u16,
    pub ramdisk_flags: u16,
    pub vga_mode: u16,
    pub orig_root_dev: u16,
    pub pad6: [u8; 1],
    pub aux_device_info: u8,
    pub jump_setup: u16,
    pub setup_signature: [u8; 4],
    pub header_format_version: u16,
    pub setup_s_temp0: [u8; 8],
    pub loader_type: u8,
    pub loadflags: u8,
    pub setup_s_temp1: u16,
    pub kernel_start: u32,
    pub initrd_start: u32,
    pub initrd_size: u32,
    pub setup_s_temp2: [u8; 4],
    pub setup_s_heap_end_pointer: u16,
    pub pad70: u16,
    pub cmd_line_ptr: u32,
    pub pad7: [u8; 0x2d0 - 0x22c],
    pub e820map: [LinuxE820Entry; 32],
    pub pad8: [u8; 0x600 - 0x550],
    pub eddbuf: [u8; 0x7d4 - 0x600],
    pub pad9: [u8; 0x800 - 0x7d4],
    pub commandline: [u8; 0x800],
    pub gdt_table: [u64; 4],
}

/* KVM structures */
#[repr(C)]
pub struct KvmRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmSegment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub type_: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    pub unusable: u8,
    pub padding: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmDtable {
    pub base: u64,
    pub limit: u16,
    pub padding: [u16; 3],
}

#[repr(C)]
pub struct KvmSregs {
    pub cs: KvmSegment,
    pub ds: KvmSegment,
    pub es: KvmSegment,
    pub fs: KvmSegment,
    pub gs: KvmSegment,
    pub ss: KvmSegment,
    pub tr: KvmSegment,
    pub ldt: KvmSegment,
    pub gdt: KvmDtable,
    pub idt: KvmDtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; 4],
}

#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
    pub entries: [KvmCpuidEntry2; 0],
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

#[repr(C)]
pub struct KvmIrqLevel {
    pub irq: u32,
    pub level: u32,
}

#[repr(C)]
pub struct KvmPitConfig {
    pub flags: u32,
    pub pad: [u32; 15],
}

#[repr(C)]
pub struct KvmDirtyLog {
    pub slot: u32,
    pub padding1: u32,
    pub dirty_bitmap: *mut c_void,
}

/************************************************************************/
/* C functions used                                                     */

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn exit(code: c_int) -> !;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memset(d: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcpy(d: *mut c_char, s: *const c_char) -> *mut c_char;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
    fn sigemptyset(set: *mut SigSet) -> c_int;
    fn sigaction(sig: c_int, act: *const SigAction, oldact: *mut SigAction) -> c_int;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
    static mut stderr: *mut c_void;
    fn vm_error(fmt: *const c_char, ...);

    fn phys_mem_map_init() -> *mut PhysMemoryMap;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
        write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
    fn register_ram_entry(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
        opaque: *mut c_void,
        irq_num: c_int,
    );

    fn x86_cpu_init(mem_map: *mut PhysMemoryMap) -> *mut X86CPUState;
    fn x86_cpu_set_irq(s: *mut X86CPUState, set: c_int);
    fn x86_cpu_set_reg(s: *mut X86CPUState, reg: c_int, val: u32);
    fn x86_cpu_get_reg(s: *mut X86CPUState, reg: c_int) -> u32;
    fn x86_cpu_set_seg(s: *mut X86CPUState, seg: c_int, sd: *const X86CPUSeg);
    fn x86_cpu_set_get_hard_intno(
        s: *mut X86CPUState,
        get_hard_intno: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
        opaque: *mut c_void,
    );
    fn x86_cpu_set_get_tsc(
        s: *mut X86CPUState,
        get_tsc: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
        opaque: *mut c_void,
    );
    fn x86_cpu_set_port_io(
        s: *mut X86CPUState,
        port_read: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
        port_write: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
        opaque: *mut c_void,
    );
    fn x86_cpu_flush_tlb_write_range_ram(s: *mut X86CPUState, ram_ptr: *mut u8, ram_size: usize);

    fn pic2_init(
        port_map: *mut PhysMemoryMap,
        addr0: u32,
        addr1: u32,
        elcr_addr0: u32,
        elcr_addr1: u32,
        cpu_set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
        opaque: *mut c_void,
        irqs: *mut IRQSignal,
    ) -> *mut PIC2State;
    fn pic2_set_elcr(s: *mut PIC2State, elcr: *const u8);
    fn pit_init(
        port_map: *mut PhysMemoryMap,
        addr0: c_int,
        addr1: c_int,
        irq: *mut IRQSignal,
        get_ticks: Option<unsafe extern "C" fn(*mut c_void) -> i64>,
        opaque: *mut c_void,
    ) -> *mut PITState;
    fn cmos_init(
        port_map: *mut PhysMemoryMap,
        addr: c_int,
        irq: *mut IRQSignal,
        use_local_time: c_int,
    ) -> *mut CMOSState;
    fn i440fx_init(
        pbus: *mut *mut PCIBus,
        ppiix3_devfn: *mut c_int,
        mem_map: *mut PhysMemoryMap,
        port_map: *mut PhysMemoryMap,
        pic_irqs: *mut IRQSignal,
    ) -> *mut I440FXState;
    fn i440fx_map_interrupts(s: *mut I440FXState, elcr: *mut u8, pci_irqs: *const u8);

    fn virtio_console_init(
        bus: *mut VIRTIOBusDef,
        cs: *mut CharacterDevice,
    ) -> *mut VIRTIODevice;
    fn virtio_block_init(bus: *mut VIRTIOBusDef, bs: *mut BlockDevice) -> *mut VIRTIODevice;
    fn virtio_9p_init(
        bus: *mut VIRTIOBusDef,
        fs: *mut FSDevice,
        mount_tag: *const c_char,
    ) -> *mut VIRTIODevice;
    fn virtio_net_init(bus: *mut VIRTIOBusDef, es: *mut EthernetDevice) -> *mut VIRTIODevice;
    fn virtio_input_init(bus: *mut VIRTIOBusDef, type_: c_int) -> *mut VIRTIODevice;
    fn pci_vga_init(
        bus: *mut PCIBus,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
        vga_rom_buf: *const u8,
        vga_rom_size: c_int,
    ) -> *mut VGAState;
    fn simplefb_init(
        map: *mut PhysMemoryMap,
        phys_addr: u64,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
    ) -> *mut SimpleFBState;
    fn ide_init(
        port_map: *mut PhysMemoryMap,
        addr: u32,
        addr2: u32,
        irq: *mut IRQSignal,
        tab_bs: *mut *mut BlockDevice,
    ) -> *mut IDEIFState;
    fn piix3_ide_init(pci_bus: *mut PCIBus, devfn: c_int) -> *mut PCIDevice;
    fn i8042_init(
        pkbd: *mut *mut PS2KbdState,
        pmouse: *mut *mut PS2MouseState,
        port_map: *mut PhysMemoryMap,
        kbd_irq: *mut IRQSignal,
        mouse_irq: *mut IRQSignal,
        io_base: u32,
    ) -> *mut KBDState;
    fn vmmouse_init(ps2_mouse: *mut PS2MouseState) -> *mut VMMouseState;
    fn vmmouse_handler(s: *mut VMMouseState, regs: *mut u32);
}

/************************************************************************/
/* helpers                                                              */

const S_EMPTY: &[u8] = b"\0";
const S_PC: &[u8] = b"pc\0";
const S_VIRTIO: &[u8] = b"virtio\0";
const S_IDE: &[u8] = b"ide\0";
const S_VGA: &[u8] = b"vga\0";
const S_SIMPLEFB: &[u8] = b"simplefb\0";
const S_PS2: &[u8] = b"ps2\0";
const S_DEV_KVM: &[u8] = b"/dev/kvm\0";
const S_HDRS: &[u8] = b"HdrS";

fn cstr(s: &[u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn put_le16(p: *mut u8, v: u16) {
    *p = v as u8;
    *p.add(1) = (v >> 8) as u8;
}

unsafe fn assert_fail(cond: bool, assertion: &[u8], line: c_uint) {
    if !cond {
        __assert_fail(
            cstr(assertion),
            cstr(b"x86_machine.c\0"),
            line,
            cstr(b"pc_machine_init\0"),
        );
    }
}

unsafe fn err_msg(fmt: &[u8]) {
    let f = stderr;
    fprintf(f, cstr(fmt));
}

unsafe fn call_register_ram(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    match (*s).register_ram {
        Some(f) => f(s, addr, size, devram_flags),
        None => ptr::null_mut(),
    }
}

unsafe fn file_at(p: *const VirtMachineParams, idx: usize) -> *const VMFileEntry {
    (*p).files.as_ptr().add(idx)
}

unsafe fn drive_at(p: *const VirtMachineParams, idx: usize) -> *const VMDriveEntry {
    (*p).tab_drive.as_ptr().add(idx)
}

unsafe fn fsentry_at(p: *const VirtMachineParams, idx: usize) -> *const VMFSEntry {
    (*p).tab_fs.as_ptr().add(idx)
}

unsafe fn ethentry_at(p: *const VirtMachineParams, idx: usize) -> *const VMEthEntry {
    (*p).tab_eth.as_ptr().add(idx)
}

unsafe fn pic_irq_ptr(s: *mut PCMachine, idx: usize) -> *mut IRQSignal {
    (*s).pic_irq.as_mut_ptr().add(idx)
}

unsafe fn pic2_pics(s: *mut PIC2State, idx: usize) -> *mut PICState {
    *(*s).pics.as_ptr().add(idx)
}

/************************************************************************/
/* ioctl request encoding                                               */

const fn ioc(dir: c_ulong, ty: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << 30) | (ty << 8) | nr | (size << 16)
}

const KVM_GET_API_VERSION: c_ulong = ioc(0, 0xAE, 0x00, 0);
const KVM_CREATE_VM: c_ulong = ioc(0, 0xAE, 0x01, 0);
const KVM_GET_VCPU_MMAP_SIZE: c_ulong = ioc(0, 0xAE, 0x04, 0);
const KVM_GET_SUPPORTED_CPUID: c_ulong = ioc(3, 0xAE, 0x05, size_of::<KvmCpuid2>() as c_ulong);
const KVM_CREATE_VCPU: c_ulong = ioc(0, 0xAE, 0x41, 0);
const KVM_GET_DIRTY_LOG: c_ulong = ioc(1, 0xAE, 0x42, size_of::<KvmDirtyLog>() as c_ulong);
const KVM_SET_USER_MEMORY_REGION: c_ulong =
    ioc(1, 0xAE, 0x46, size_of::<KvmUserspaceMemoryRegion>() as c_ulong);
const KVM_SET_TSS_ADDR: c_ulong = ioc(0, 0xAE, 0x47, 0);
const KVM_SET_IDENTITY_MAP_ADDR: c_ulong = ioc(1, 0xAE, 0x48, 8);
const KVM_CREATE_IRQCHIP: c_ulong = ioc(0, 0xAE, 0x60, 0);
const KVM_IRQ_LINE: c_ulong = ioc(1, 0xAE, 0x61, size_of::<KvmIrqLevel>() as c_ulong);
const KVM_CREATE_PIT2: c_ulong = ioc(1, 0xAE, 0x77, size_of::<KvmPitConfig>() as c_ulong);
const KVM_GET_REGS: c_ulong = ioc(2, 0xAE, 0x81, size_of::<KvmRegs>() as c_ulong);
const KVM_SET_REGS: c_ulong = ioc(2, 0xAE, 0x82, size_of::<KvmRegs>() as c_ulong);
const KVM_GET_SREGS: c_ulong = ioc(2, 0xAE, 0x83, size_of::<KvmSregs>() as c_ulong);
const KVM_SET_SREGS: c_ulong = ioc(2, 0xAE, 0x84, size_of::<KvmSregs>() as c_ulong);
const KVM_SET_CPUID2: c_ulong = ioc(1, 0xAE, 0x90, size_of::<KvmCpuid2>() as c_ulong);

const KVM_MEM_LOG_DIRTY_PAGES: c_int = 1 << 0;
const KVM_MEM_READONLY: c_int = 1 << 1;
const KVM_PIT_SPEAKER_DUMMY: u32 = 1;

/************************************************************************/
/* static helpers of x86_machine.c                                      */

unsafe extern "C" fn cpu_get_tsc(_opaque: *mut c_void) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts);
    (ts.tv_sec as u64)
        .wrapping_mul(100000000)
        .wrapping_add((ts.tv_nsec / (1000000000 / 100000000)) as u64)
}

unsafe extern "C" fn pc_flush_tlb_write_range(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut PCMachine;
    x86_cpu_flush_tlb_write_range_ram((*s).cpu_state, ram_addr, ram_size);
}

unsafe extern "C" fn port80_write(
    _opaque: *mut c_void,
    _offset: u32,
    _val: u32,
    _size_log2: c_int,
) {
}

unsafe extern "C" fn port80_read(
    _opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    0xff
}

unsafe extern "C" fn port92_write(
    _opaque: *mut c_void,
    _offset: u32,
    _val: u32,
    _size_log2: c_int,
) {
}

unsafe extern "C" fn port92_read(
    _opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    let a20: c_int = 1;
    (a20 << 1) as u32
}

const VMPORT_MAGIC: u32 = 0x564D5868;
const REG_EAX: usize = 0;
const REG_EBX: usize = 1;
const REG_ECX: usize = 2;
const REG_EDX: usize = 3;
const REG_ESI: usize = 4;
const REG_EDI: usize = 5;

unsafe extern "C" fn vmport_read(opaque: *mut c_void, _addr: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut PCMachine;
    let mut regs: [u32; 6] = [0; 6];

    if (*s).kvm_enabled != 0 {
        let mut r: KvmRegs = core::mem::zeroed();

        ioctl(
            (*s).vcpu_fd,
            KVM_GET_REGS,
            &mut r as *mut KvmRegs as *mut c_void,
        );
        regs[REG_EAX] = r.rax as u32;
        regs[REG_EBX] = r.rbx as u32;
        regs[REG_ECX] = r.rcx as u32;
        regs[REG_EDX] = r.rdx as u32;
        regs[REG_ESI] = r.rsi as u32;
        regs[REG_EDI] = r.rdi as u32;

        if regs[REG_EAX] == VMPORT_MAGIC {
            vmmouse_handler((*s).vm_mouse, regs.as_mut_ptr());

            r.rax = regs[REG_EAX] as u64;
            r.rbx = regs[REG_EBX] as u64;
            r.rcx = regs[REG_ECX] as u64;
            r.rdx = regs[REG_EDX] as u64;
            r.rsi = regs[REG_ESI] as u64;
            r.rdi = regs[REG_EDI] as u64;
            ioctl(
                (*s).vcpu_fd,
                KVM_SET_REGS,
                &mut r as *mut KvmRegs as *mut c_void,
            );
        }
    } else {
        regs[REG_EAX] = x86_cpu_get_reg((*s).cpu_state, 0);
        regs[REG_EBX] = x86_cpu_get_reg((*s).cpu_state, 3);
        regs[REG_ECX] = x86_cpu_get_reg((*s).cpu_state, 1);
        regs[REG_EDX] = x86_cpu_get_reg((*s).cpu_state, 2);
        regs[REG_ESI] = x86_cpu_get_reg((*s).cpu_state, 6);
        regs[REG_EDI] = x86_cpu_get_reg((*s).cpu_state, 7);

        if regs[REG_EAX] == VMPORT_MAGIC {
            vmmouse_handler((*s).vm_mouse, regs.as_mut_ptr());

            x86_cpu_set_reg((*s).cpu_state, 0, regs[REG_EAX]);
            x86_cpu_set_reg((*s).cpu_state, 3, regs[REG_EBX]);
            x86_cpu_set_reg((*s).cpu_state, 1, regs[REG_ECX]);
            x86_cpu_set_reg((*s).cpu_state, 2, regs[REG_EDX]);
            x86_cpu_set_reg((*s).cpu_state, 6, regs[REG_ESI]);
            x86_cpu_set_reg((*s).cpu_state, 7, regs[REG_EDI]);
        }
    }
    regs[REG_EAX]
}

unsafe extern "C" fn vmport_write(
    _opaque: *mut c_void,
    _addr: u32,
    _val: u32,
    _size_log2: c_int,
) {
}

unsafe extern "C" fn pic_set_irq_cb(opaque: *mut c_void, level: c_int) {
    let s = opaque as *mut PCMachine;
    x86_cpu_set_irq((*s).cpu_state, level);
}

/* --- pic2 helpers (static in x86_machine.c) --- */

unsafe fn pic_get_priority(s: *mut PICState, mask: c_int) -> c_int {
    if mask == 0 {
        return -1;
    }
    let mut priority: c_int = 7;
    while (mask & (1 << ((priority + (*s).priority_add as c_int) & 7))) == 0 {
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
            (*s).priority_add = ((irq + 1) & 7) as u8;
        }
    } else {
        (*s).isr |= (1u32 << irq) as u8;
    }
    if ((*s).elcr & (1u32 << irq) as u8) == 0 {
        (*s).irr &= !((1u32 << irq) as u8);
    }
}

unsafe fn pic_set_irq1(s: *mut PICState, irq: c_int, level: c_int) {
    let mask = (1u32 << irq) as u8;
    if ((*s).elcr & mask) != 0 {
        if level != 0 {
            (*s).irr |= mask;
            (*s).last_irr |= mask;
        } else {
            (*s).irr &= !mask;
            (*s).last_irr &= !mask;
        }
    } else if level != 0 {
        if ((*s).last_irr & mask) == 0 {
            (*s).irr |= mask;
        }
        (*s).last_irr |= mask;
    } else {
        (*s).last_irr &= !mask;
    }
}

unsafe fn pic2_update_irq(opaque: *mut c_void) {
    let s = opaque as *mut PIC2State;
    let p0 = pic2_pics(s, 0);
    let p1 = pic2_pics(s, 1);
    let irq2 = pic_get_irq(p1);
    if irq2 >= 0 {
        pic_set_irq1(p0, 2, 1);
        pic_set_irq1(p0, 2, 0);
    }
    let irq = pic_get_irq(p0);
    let level: c_int = if irq >= 0 { 1 } else { 0 };
    match (*s).cpu_set_irq {
        Some(f) => f((*s).opaque, level),
        None => {}
    }
}

unsafe fn pic2_get_hard_intno(s: *mut PIC2State) -> c_int {
    let p0 = pic2_pics(s, 0);
    let p1 = pic2_pics(s, 1);
    let mut irq: c_int;
    let intno: c_int;

    irq = pic_get_irq(p0);
    if irq >= 0 {
        pic_intack(p0, irq);
        if irq == 2 {
            let mut irq2 = pic_get_irq(p1);
            if irq2 >= 0 {
                pic_intack(p1, irq2);
            } else {
                irq2 = 7;
            }
            intno = (*p1).irq_base as c_int + irq2;
        } else {
            intno = (*p0).irq_base as c_int + irq;
        }
    } else {
        irq = 7;
        intno = (*p0).irq_base as c_int + irq;
    }
    pic2_update_irq(s as *mut c_void);
    intno
}

unsafe extern "C" fn get_hard_intno_cb(opaque: *mut c_void) -> c_int {
    let s = opaque as *mut PCMachine;
    pic2_get_hard_intno((*s).pic_state)
}

unsafe extern "C" fn pit_get_ticks_cb(_opaque: *mut c_void) -> i64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts);
    let v = (ts.tv_sec as u64)
        .wrapping_mul(1193182)
        .wrapping_add((ts.tv_nsec as u64).wrapping_mul(1193182) / 1000000000);
    v as i64
}

unsafe fn get_ram_ptr(s: *mut PCMachine, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range((*s).mem_map, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return ptr::null_mut();
    }
    (*pr).phys_mem.add(paddr.wrapping_sub((*pr).addr) as usize)
}

unsafe extern "C" fn st_port(opaque: *mut c_void, port: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut PCMachine;
    let mut port = port;

    let pr = get_phys_mem_range((*s).port_map, port as u64);
    if pr.is_null() {
        return;
    }
    port = port.wrapping_sub((*pr).addr as u32);
    if (((*pr).devio_flags >> size_log2) & 1) != 0 {
        match (*pr).write_func {
            Some(f) => f((*pr).opaque, port, val, size_log2),
            None => {}
        }
    } else if size_log2 == 1 && ((*pr).devio_flags & (1 << 0)) != 0 {
        match (*pr).write_func {
            Some(f) => {
                f((*pr).opaque, port, val & 0xff, 0);
                f((*pr).opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0);
            }
            None => {}
        }
    }
}

unsafe extern "C" fn ld_port(opaque: *mut c_void, port1: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut PCMachine;
    let val: u32;

    let mut port = port1;
    let pr = get_phys_mem_range((*s).port_map, port as u64);
    if pr.is_null() {
        val = 0xffffffff;
    } else {
        port = port.wrapping_sub((*pr).addr as u32);
        if (((*pr).devio_flags >> size_log2) & 1) != 0 {
            val = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port, size_log2),
                None => 0xffffffff,
            };
        } else if size_log2 == 1 && ((*pr).devio_flags & (1 << 0)) != 0 {
            let lo = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port, 0) & 0xff,
                None => 0,
            };
            let hi = match (*pr).read_func {
                Some(f) => f((*pr).opaque, port.wrapping_add(1), 0) & 0xff,
                None => 0,
            };
            val = lo | (hi << 8);
        } else {
            val = 0xffffffff;
        }
    }
    val
}

/* --- kvm helpers (static in x86_machine.c) --- */

unsafe extern "C" fn sigalrm_handler(_sig: c_int) {}

unsafe extern "C" fn kvm_pic_set_irq(opaque: *mut c_void, irq_num: c_int, level: c_int) {
    let s = opaque as *mut PCMachine;
    let mut irq_level = KvmIrqLevel {
        irq: irq_num as u32,
        level: level as u32,
    };
    if ioctl(
        (*s).vm_fd,
        KVM_IRQ_LINE,
        &mut irq_level as *mut KvmIrqLevel as *mut c_void,
    ) < 0
    {
        perror(cstr(b"KVM_IRQ_LINE\0"));
        exit(1);
    }
}

unsafe fn kvm_set_cpuid(s: *mut PCMachine) {
    let n_ent_max: c_int = 128;
    let kvm_cpuid = mallocz(
        size_of::<KvmCpuid2>() + (n_ent_max as usize) * size_of::<KvmCpuidEntry2>(),
    ) as *mut KvmCpuid2;

    (*kvm_cpuid).nent = n_ent_max as u32;
    if ioctl((*s).kvm_fd, KVM_GET_SUPPORTED_CPUID, kvm_cpuid as *mut c_void) < 0 {
        perror(cstr(b"KVM_GET_SUPPORTED_CPUID\0"));
        exit(1);
    }

    let mut i: c_int = 0;
    while i < (*kvm_cpuid).nent as c_int {
        let ent = (*kvm_cpuid).entries.as_mut_ptr().add(i as usize);
        if (*ent).function == 1 || (*ent).function == 0x80000001 {
            (*ent).edx &= !(((1u32) << 9) | ((1u32) << 22));
        }
        i += 1;
    }

    if ioctl((*s).vcpu_fd, KVM_SET_CPUID2, kvm_cpuid as *mut c_void) < 0 {
        perror(cstr(b"KVM_SET_CPUID2\0"));
        exit(1);
    }
    free(kvm_cpuid as *mut c_void);
}

unsafe fn slot_of(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) -> u32 {
    let base = (*mem_map).phys_mem_range.as_ptr() as usize;
    (((pr as usize).wrapping_sub(base)) / size_of::<PhysMemoryRange>()) as u32
}

unsafe fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region: KvmUserspaceMemoryRegion = core::mem::zeroed();
    let mut flags: c_int = 0;

    region.slot = slot_of(mem_map, pr);
    if ((*pr).devram_flags & (1 << 0)) != 0 {
        flags |= KVM_MEM_READONLY;
    }
    if ((*pr).devram_flags & (1 << 1)) != 0 {
        flags |= KVM_MEM_LOG_DIRTY_PAGES;
    }
    region.flags = flags as u32;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as usize as u64;
    if ioctl(
        (*s).vm_fd,
        KVM_SET_USER_MEMORY_REGION,
        &mut region as *mut KvmUserspaceMemoryRegion as *mut c_void,
    ) < 0
    {
        perror(cstr(b"KVM_SET_USER_MEMORY_REGION\0"));
        exit(1);
    }
}

unsafe extern "C" fn kvm_register_ram(
    mem_map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    let pr = register_ram_entry(mem_map, addr, size, devram_flags);

    let phys_mem = mmap(
        ptr::null_mut(),
        size as usize,
        0x1 | 0x2,   /* PROT_READ | PROT_WRITE */
        0x01 | 0x20, /* MAP_SHARED | MAP_ANONYMOUS */
        -1,
        0,
    ) as *mut u8;
    if phys_mem.is_null() {
        return ptr::null_mut();
    }
    (*pr).phys_mem = phys_mem;
    if (devram_flags & (1 << 1)) != 0 {
        let n_pages = ((size >> 12) as u32) as c_int;
        (*pr).dirty_bits_size = ((n_pages + 63) / 64) * 8;
        (*pr).dirty_bits = mallocz((*pr).dirty_bits_size as usize) as *mut u32;
    }

    if (*pr).size != 0 {
        kvm_map_ram(mem_map, pr);
    }
    pr
}

unsafe extern "C" fn kvm_set_ram_addr(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: c_int,
) {
    if enabled != 0 {
        if (*pr).size == 0 || addr != (*pr).addr {
            (*pr).size = (*pr).org_size;
            (*pr).addr = addr;
            kvm_map_ram(mem_map, pr);
        }
    } else if (*pr).size != 0 {
        (*pr).addr = 0;
        (*pr).size = 0;
        kvm_map_ram(mem_map, pr);
    }
}

unsafe extern "C" fn kvm_get_dirty_bits(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) -> *const u32 {
    let s = (*mem_map).opaque as *mut PCMachine;

    if (*pr).size == 0 {
        memset(
            (*pr).dirty_bits as *mut c_void,
            0,
            (*pr).dirty_bits_size as usize,
        );
    } else {
        let mut dlog = KvmDirtyLog {
            slot: 0,
            padding1: 0,
            dirty_bitmap: ptr::null_mut(),
        };
        dlog.slot = slot_of(mem_map, pr);
        dlog.dirty_bitmap = (*pr).dirty_bits as *mut c_void;
        if ioctl(
            (*s).vm_fd,
            KVM_GET_DIRTY_LOG,
            &mut dlog as *mut KvmDirtyLog as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_GET_DIRTY_LOG\0"));
            exit(1);
        }
    }
    (*pr).dirty_bits
}

unsafe extern "C" fn kvm_free_ram(_mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    munmap((*pr).phys_mem as *mut c_void, (*pr).org_size as usize);
    free((*pr).dirty_bits as *mut c_void);
}

unsafe fn kvm_init(s: *mut PCMachine) {
    (*s).kvm_enabled = 0; /* FALSE */
    (*s).kvm_fd = open(cstr(S_DEV_KVM), 0o2 /* O_RDWR */);
    if (*s).kvm_fd < 0 {
        err_msg(b"KVM not available\n\0");
        return;
    }
    let ret = ioctl((*s).kvm_fd, KVM_GET_API_VERSION, ptr::null_mut::<c_void>());
    if ret < 0 {
        perror(cstr(b"KVM_GET_API_VERSION\0"));
        exit(1);
    }
    if ret != 12 {
        err_msg(b"Unsupported KVM version\n\0");
        close((*s).kvm_fd);
        (*s).kvm_fd = -1;
        return;
    }
    (*s).vm_fd = ioctl((*s).kvm_fd, KVM_CREATE_VM, ptr::null_mut::<c_void>());
    if (*s).vm_fd < 0 {
        perror(cstr(b"KVM_CREATE_VM\0"));
        exit(1);
    }

    let mut base_addr: u64 = 0xfffbc000;
    if ioctl(
        (*s).vm_fd,
        KVM_SET_IDENTITY_MAP_ADDR,
        &mut base_addr as *mut u64 as *mut c_void,
    ) < 0
    {
        perror(cstr(b"KVM_SET_IDENTITY_MAP_ADDR\0"));
        exit(1);
    }

    if ioctl(
        (*s).vm_fd,
        KVM_SET_TSS_ADDR,
        (base_addr.wrapping_add(0x1000)) as c_long as *mut c_void,
    ) < 0
    {
        perror(cstr(b"KVM_SET_TSS_ADDR\0"));
        exit(1);
    }

    if ioctl((*s).vm_fd, KVM_CREATE_IRQCHIP, ptr::null_mut::<c_void>()) < 0 {
        perror(cstr(b"KVM_CREATE_IRQCHIP\0"));
        exit(1);
    }

    let mut pit_config: KvmPitConfig = core::mem::zeroed();
    pit_config.flags = KVM_PIT_SPEAKER_DUMMY;
    if ioctl(
        (*s).vm_fd,
        KVM_CREATE_PIT2,
        &mut pit_config as *mut KvmPitConfig as *mut c_void,
    ) != 0
    {
        perror(cstr(b"KVM_CREATE_PIT2\0"));
        exit(1);
    }

    (*s).vcpu_fd = ioctl((*s).vm_fd, KVM_CREATE_VCPU, ptr::null_mut::<c_void>());
    if (*s).vcpu_fd < 0 {
        perror(cstr(b"KVM_CREATE_VCPU\0"));
        exit(1);
    }

    kvm_set_cpuid(s);

    (*s).kvm_run_size = ioctl((*s).kvm_fd, KVM_GET_VCPU_MMAP_SIZE, ptr::null_mut::<c_void>());
    if (*s).kvm_run_size < 0 {
        perror(cstr(b"KVM_GET_VCPU_MMAP_SIZE\0"));
        exit(1);
    }

    (*s).kvm_run = mmap(
        ptr::null_mut(),
        (*s).kvm_run_size as usize,
        0x1 | 0x2, /* PROT_READ | PROT_WRITE */
        0x01,      /* MAP_SHARED */
        (*s).vcpu_fd,
        0,
    ) as *mut KvmRun;
    if (*s).kvm_run.is_null() {
        perror(cstr(b"mmap kvm_run\0"));
        exit(1);
    }

    let mut i: c_int = 0;
    while i < 16 {
        irq_init(
            pic_irq_ptr(s, i as usize),
            Some(kvm_pic_set_irq),
            s as *mut c_void,
            i,
        );
        i += 1;
    }

    let mut act: SigAction = core::mem::zeroed();
    act.handler = Some(sigalrm_handler);
    sigemptyset(&mut act.mask);
    act.flags = 0;
    sigaction(14 /* SIGALRM */, &act, ptr::null_mut());

    (*s).kvm_enabled = 1; /* TRUE */

    (*(*s).mem_map).register_ram = Some(kvm_register_ram);
    (*(*s).mem_map).free_ram = Some(kvm_free_ram);
    (*(*s).mem_map).get_dirty_bits = Some(kvm_get_dirty_bits);
    (*(*s).mem_map).set_ram_addr = Some(kvm_set_ram_addr);
    (*(*s).mem_map).opaque = s as *mut c_void;
}

/************************************************************************/
/* copy_kernel                                                          */

const KERNEL_PARAMS_ADDR: u32 = 0x00090000;
const FRAMEBUFFER_BASE_ADDR: u32 = 0xf0400000;

unsafe fn kernel_too_small() -> ! {
    err_msg(b"Kernel too small\n\0");
    exit(1)
}

unsafe fn copy_kernel(
    s: *mut PCMachine,
    buf: *const u8,
    buf_len: c_int,
    cmd_line: *const c_char,
) {
    let load_address: u32 = 0x100000;

    if buf_len < 1024 {
        kernel_too_small();
    }
    if *buf.add(0x1fe) != 0x55 || *buf.add(0x1ff) != 0xaa {
        err_msg(b"Invalid kernel magic\n\0");
        exit(1);
    }
    let mut setup_sects: c_int = *buf.add(0x1f1) as c_int;
    if setup_sects == 0 {
        setup_sects = 4;
    }
    let header_len: c_int = (setup_sects + 1) * 512;
    if buf_len < header_len {
        kernel_too_small();
    }
    if memcmp(
        buf.add(0x202) as *const c_void,
        S_HDRS.as_ptr() as *const c_void,
        4,
    ) != 0
    {
        err_msg(b"Kernel too old\n\0");
        exit(1);
    }

    let ram_ptr = get_ram_ptr(s, load_address as u64);
    let copy_len: c_int = buf_len - header_len;
    if (copy_len as u64) > (*s).ram_size.wrapping_sub(load_address as u64) {
        err_msg(b"Not enough RAM\n\0");
        exit(1);
    }
    memcpy(
        ram_ptr as *mut c_void,
        buf.add(header_len as usize) as *const c_void,
        copy_len as usize,
    );

    let params = get_ram_ptr(s, KERNEL_PARAMS_ADDR as u64) as *mut LinuxParams;

    memset(params as *mut c_void, 0, size_of::<LinuxParams>());

    let setup_hdr_start: c_int = 0x1f1;
    let setup_hdr_end: c_int = 0x202 + *buf.add(0x201) as c_int;
    memcpy(
        (params as *mut u8).add(setup_hdr_start as usize) as *mut c_void,
        buf.add(setup_hdr_start as usize) as *const c_void,
        (setup_hdr_end - setup_hdr_start) as usize,
    );

    strcpy(
        ptr::addr_of_mut!((*params).commandline) as *mut c_char,
        cmd_line,
    );

    let cmdline_off = (ptr::addr_of!((*params).commandline) as usize).wrapping_sub(params as usize);
    let gdt_off = (ptr::addr_of!((*params).gdt_table) as usize).wrapping_sub(params as usize);

    (*params).mount_root_rdonly = 0;
    (*params).cmd_line_ptr = KERNEL_PARAMS_ADDR.wrapping_add(cmdline_off as u32);
    (*params).alt_mem_k = ((*s).ram_size / 1024).wrapping_sub(1024) as u32;
    (*params).loader_type = 0x01;

    (*params).orig_video_lines = 0;
    (*params).orig_video_cols = 0;

    let fb_dev = (*s).common.fb_dev;
    if !fb_dev.is_null() {
        (*params).orig_video_is_vga = 0x23; /* VIDEO_TYPE_VLFB */

        (*params).lfb_depth = 32;
        (*params).red_size = 8;
        (*params).red_pos = 16;
        (*params).green_size = 8;
        (*params).green_pos = 8;
        (*params).blue_size = 8;
        (*params).blue_pos = 0;
        (*params).rsvd_size = 8;
        (*params).rsvd_pos = 24;

        (*params).lfb_width = (*fb_dev).width as u16;
        (*params).lfb_height = (*fb_dev).height as u16;
        (*params).lfb_linelength = (*fb_dev).stride as u16;
        (*params).lfb_size = (*fb_dev).fb_size as u32;
        (*params).lfb_base = FRAMEBUFFER_BASE_ADDR;
    }

    let gdt = ptr::addr_of_mut!((*params).gdt_table) as *mut u64;
    *gdt.add(2) = 0x00cf9b000000ffff; /* CS */
    *gdt.add(3) = 0x00cf93000000ffff; /* DS */

    let gdt_limit: u32 = (size_of::<[u64; 4]>() - 1) as u32;
    let gdt_base: u32 = KERNEL_PARAMS_ADDR.wrapping_add(gdt_off as u32);

    if (*s).kvm_enabled != 0 {
        /* init flat protected mode */
        let mut sregs: KvmSregs = core::mem::zeroed();

        if ioctl(
            (*s).vcpu_fd,
            KVM_GET_SREGS,
            &mut sregs as *mut KvmSregs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_GET_SREGS\0"));
            exit(1);
        }

        sregs.cr0 |= 1 << 0; /* CR0_PE */
        sregs.gdt.base = gdt_base as u64;
        sregs.gdt.limit = gdt_limit as u16;

        let mut seg: KvmSegment = core::mem::zeroed();
        seg.limit = 0xffffffff;
        seg.present = 1;
        seg.db = 1;
        seg.s = 1;
        seg.g = 1;

        seg.type_ = 0xb; /* code */
        seg.selector = 2 << 3;
        sregs.cs = seg;

        seg.type_ = 0x3; /* data */
        seg.selector = 3 << 3;
        sregs.ds = seg;
        sregs.es = seg;
        sregs.ss = seg;
        sregs.fs = seg;
        sregs.gs = seg;

        if ioctl(
            (*s).vcpu_fd,
            KVM_SET_SREGS,
            &mut sregs as *mut KvmSregs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_SET_SREGS\0"));
            exit(1);
        }

        let mut regs: KvmRegs = core::mem::zeroed();
        regs.rip = load_address as u64;
        regs.rsi = KERNEL_PARAMS_ADDR as u64;
        regs.rflags = 0x2;
        if ioctl(
            (*s).vcpu_fd,
            KVM_SET_REGS,
            &mut regs as *mut KvmRegs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_SET_REGS\0"));
            exit(1);
        }
    } else {
        let mut sd = X86CPUSeg {
            sel: 0,
            flags: 0,
            base: 0,
            limit: 0,
        };
        let val = x86_cpu_get_reg((*s).cpu_state, 9 /* X86_CPU_REG_CR0 */);
        x86_cpu_set_reg((*s).cpu_state, 9, val | (1 << 0));

        sd.base = gdt_base;
        sd.limit = gdt_limit;
        x86_cpu_set_seg((*s).cpu_state, 8 /* X86_CPU_SEG_GDT */, &sd);
        sd.sel = 2 << 3;
        sd.base = 0;
        sd.limit = 0xffffffff;
        sd.flags = 0xc09b;
        x86_cpu_set_seg((*s).cpu_state, 1 /* X86_CPU_SEG_CS */, &sd);
        sd.sel = 3 << 3;
        sd.flags = 0xc093;
        let mut i: c_int = 0;
        while i < 6 {
            if i != 1 {
                x86_cpu_set_seg((*s).cpu_state, i, &sd);
            }
            i += 1;
        }

        x86_cpu_set_reg((*s).cpu_state, 8 /* X86_CPU_REG_EIP */, load_address);
        x86_cpu_set_reg((*s).cpu_state, 6, KERNEL_PARAMS_ADDR); /* esi */
    }

    /* map PCI interrupts (no BIOS, so we must do it) */
    {
        let mut elcr: [u8; 2] = [0; 2];
        let pci_irqs: [u8; 4] = [9, 10, 11, 12];

        i440fx_map_interrupts((*s).i440fx_state, elcr.as_mut_ptr(), pci_irqs.as_ptr());
        if !(*s).pic_state.is_null() {
            pic2_set_elcr((*s).pic_state, elcr.as_ptr());
        }
    }
}

/************************************************************************/
/* pc_machine_init                                                      */

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init(p: *const VirtMachineParams) -> *mut VirtMachine {
    let mut piix3_devfn: c_int = 0;
    let mut pci_bus: *mut PCIBus = ptr::null_mut();
    let mut vbus_s: VIRTIOBusDef = core::mem::zeroed();
    let vbus: *mut VIRTIOBusDef = &mut vbus_s;

    if strcmp((*p).machine_name, cstr(S_PC)) != 0 {
        vm_error(cstr(b"unsupported machine: %s\n\0"), (*p).machine_name);
        return ptr::null_mut();
    }

    assert_fail(
        (*p).ram_size >= (1 << 20),
        b"p->ram_size >= (1 << 20)\0",
        1972,
    );

    let s = mallocz(size_of::<PCMachine>()) as *mut PCMachine;
    (*s).common.vmc = (*p).vmc;
    (*s).ram_size = (*p).ram_size;

    (*s).port_map = phys_mem_map_init();
    (*s).mem_map = phys_mem_map_init();

    if (*p).accel_enable != 0 {
        kvm_init(s);
    }

    if (*s).kvm_enabled == 0 {
        (*s).cpu_state = x86_cpu_init((*s).mem_map);
        x86_cpu_set_get_tsc((*s).cpu_state, Some(cpu_get_tsc), s as *mut c_void);
        x86_cpu_set_port_io(
            (*s).cpu_state,
            Some(ld_port),
            Some(st_port),
            s as *mut c_void,
        );

        /* needed to handle the RAM dirty bits */
        (*(*s).mem_map).opaque = s as *mut c_void;
        (*(*s).mem_map).flush_tlb_write_range = Some(pc_flush_tlb_write_range);
    }

    /* set the RAM mapping and leave the VGA addresses empty */
    call_register_ram(
        (*s).mem_map,
        0xc0000,
        (*p).ram_size.wrapping_sub(0xc0000),
        0,
    );
    call_register_ram((*s).mem_map, 0, 0xa0000, 0);

    /* devices */
    cpu_register_device(
        (*s).port_map,
        0x80,
        2,
        s as *mut c_void,
        Some(port80_read),
        Some(port80_write),
        1 << 0, /* DEVIO_SIZE8 */
    );
    cpu_register_device(
        (*s).port_map,
        0x92,
        2,
        s as *mut c_void,
        Some(port92_read),
        Some(port92_write),
        1 << 0, /* DEVIO_SIZE8 */
    );

    /* setup the bios */
    if (*(file_at(p, 0))).len > 0 {
        let bios_size: c_int = (*(file_at(p, 0))).len;
        let bios_buf: *mut u8 = (*(file_at(p, 0))).buf;

        assert_fail(
            (bios_size % 65536) == 0 && bios_size != 0,
            b"(bios_size % 65536) == 0 && bios_size != 0\0",
            2018,
        );
        let bios_addr: u32 = (bios_size as c_int).wrapping_neg() as u32;
        /* at the top of the 4GB memory */
        call_register_ram((*s).mem_map, bios_addr as u64, bios_size as u64, 1 << 0);
        let mut ram_ptr = get_ram_ptr(s, bios_addr as u64);
        memcpy(
            ram_ptr as *mut c_void,
            bios_buf as *const c_void,
            bios_size as usize,
        );
        /* in the lower 1MB memory (currently set as RAM) */
        let bios_size1: c_int = min_int(bios_size, 128 * 1024);
        ram_ptr = get_ram_ptr(s, (0x100000 - bios_size1) as u64);
        memcpy(
            ram_ptr as *mut c_void,
            bios_buf.add((bios_size - bios_size1) as usize) as *const c_void,
            bios_size1 as usize,
        );
    }

    if (*s).kvm_enabled == 0 {
        (*s).pic_state = pic2_init(
            (*s).port_map,
            0x20,
            0xa0,
            0x4d0,
            0x4d1,
            Some(pic_set_irq_cb),
            s as *mut c_void,
            (*s).pic_irq.as_mut_ptr(),
        );
        x86_cpu_set_get_hard_intno((*s).cpu_state, Some(get_hard_intno_cb), s as *mut c_void);
        (*s).pit_state = pit_init(
            (*s).port_map,
            0x40,
            0x61,
            pic_irq_ptr(s, 0),
            Some(pit_get_ticks_cb),
            s as *mut c_void,
        );
    }

    (*s).cmos_state = cmos_init((*s).port_map, 0x70, pic_irq_ptr(s, 8), (*p).rtc_local_time);

    /* various cmos data */
    {
        let cmos = (*s).cmos_state;
        let mut size: c_int = min_int(
            ((*s).ram_size.wrapping_sub(1 << 20) >> 10) as u32 as c_int,
            65535,
        );
        put_le16((*cmos).cmos_data.as_mut_ptr().add(0x30), size as u16);
        if (*s).ram_size >= (16 << 20) {
            size = min_int(
                ((*s).ram_size.wrapping_sub(16 << 20) >> 16) as u32 as c_int,
                65535,
            );
            put_le16((*cmos).cmos_data.as_mut_ptr().add(0x34), size as u16);
        }
        *(*cmos).cmos_data.as_mut_ptr().add(0x14) = 0x06; /* mouse + FPU present */
    }

    (*s).i440fx_state = i440fx_init(
        &mut pci_bus,
        &mut piix3_devfn,
        (*s).mem_map,
        (*s).port_map,
        (*s).pic_irq.as_mut_ptr(),
    );

    (*s).common.console = (*p).console;

    memset(vbus as *mut c_void, 0, size_of::<VIRTIOBusDef>());
    (*vbus).pci_bus = pci_bus;

    if !(*p).console.is_null() {
        /* virtio console */
        (*s).common.console_dev = virtio_console_init(vbus, (*p).console);
    }

    /* block devices */
    let mut i: c_int = 0;
    while i < (*p).drive_count {
        let de = drive_at(p, i as usize);

        if (*de).device.is_null() || strcmp((*de).device, cstr(S_VIRTIO)) == 0 {
            virtio_block_init(vbus, (*(drive_at(p, i as usize))).block_dev);
            i += 1;
        } else if strcmp((*de).device, cstr(S_IDE)) == 0 {
            let mut tab_bs: [*mut BlockDevice; 2] = [ptr::null_mut(); 2];

            tab_bs[0] = (*(drive_at(p, i as usize))).block_dev;
            i += 1;
            tab_bs[1] = ptr::null_mut();
            if i < (*p).drive_count {
                tab_bs[1] = (*(drive_at(p, i as usize))).block_dev;
                i += 1;
            }
            ide_init(
                (*s).port_map,
                0x1f0,
                0x3f6,
                pic_irq_ptr(s, 14),
                tab_bs.as_mut_ptr(),
            );
            piix3_ide_init(pci_bus, piix3_devfn + 1);
        }
    }

    /* virtio filesystem */
    i = 0;
    while i < (*p).fs_count {
        let fe = fsentry_at(p, i as usize);
        virtio_9p_init(vbus, (*fe).fs_dev, (*fe).tag);
        i += 1;
    }

    if !(*p).display_device.is_null() {
        let fb_dev = mallocz(size_of::<FBDevice>()) as *mut FBDevice;

        (*s).common.fb_dev = fb_dev;
        if strcmp((*p).display_device, cstr(S_VGA)) == 0 {
            let bios_size: c_int = (*(file_at(p, 1))).len;
            let bios_buf: *mut u8 = (*(file_at(p, 1))).buf;
            pci_vga_init(
                pci_bus,
                fb_dev,
                (*p).width,
                (*p).height,
                bios_buf,
                bios_size,
            );
        } else if strcmp((*p).display_device, cstr(S_SIMPLEFB)) == 0 {
            simplefb_init(
                (*s).mem_map,
                FRAMEBUFFER_BASE_ADDR as u64,
                fb_dev,
                (*p).width,
                (*p).height,
            );
        } else {
            vm_error(
                cstr(b"unsupported display device: %s\n\0"),
                (*p).display_device,
            );
            exit(1);
        }
    }

    if !(*p).input_device.is_null() {
        if strcmp((*p).input_device, cstr(S_VIRTIO)) == 0 {
            (*s).keyboard_dev = virtio_input_init(vbus, 0 /* VIRTIO_INPUT_TYPE_KEYBOARD */);

            (*s).mouse_dev = virtio_input_init(vbus, 2 /* VIRTIO_INPUT_TYPE_TABLET */);
        } else if strcmp((*p).input_device, cstr(S_PS2)) == 0 {
            (*s).kbd_state = i8042_init(
                &mut (*s).ps2_kbd,
                &mut (*s).ps2_mouse,
                (*s).port_map,
                pic_irq_ptr(s, 1),
                pic_irq_ptr(s, 12),
                0x60,
            );

            /* vmmouse */
            cpu_register_device(
                (*s).port_map,
                0x5658,
                1,
                s as *mut c_void,
                Some(vmport_read),
                Some(vmport_write),
                1 << 2, /* DEVIO_SIZE32 */
            );
            (*s).vm_mouse = vmmouse_init((*s).ps2_mouse);
        } else {
            vm_error(
                cstr(b"unsupported input device: %s\n\0"),
                (*p).input_device,
            );
            exit(1);
        }
    }

    /* virtio net device */
    i = 0;
    while i < (*p).eth_count {
        let ee = ethentry_at(p, i as usize);
        virtio_net_init(vbus, (*ee).net);
        (*s).common.net = (*ee).net;
        i += 1;
    }

    let kf = file_at(p, 2);
    if !(*kf).buf.is_null() {
        let cmd = if !(*p).cmdline.is_null() {
            (*p).cmdline
        } else {
            S_EMPTY.as_ptr() as *mut c_char
        };
        copy_kernel(s, (*kf).buf, (*kf).len, cmd);
    }

    s as *mut VirtMachine
}
}

// ── `if (strcmp(p->machine_name, "pc") != 0)` · x86_machine.c:1967-1970 · chunk 4211 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (strcmp(p->machine_name, "pc") != 0) {
//             vm_error("unsupported machine: %s\n", p->machine_name);
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4211 {
// Port of the machine-name check at the start of pc_machine_init().
// Returns 0 when the machine name is "pc", nonzero when it is unsupported
// (in which case vm_error() has already been called).

use core::ffi::{c_char, c_int};

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4211(machine_name: *const c_char) -> c_int {
    unsafe {
        let expected = b"pc\0".as_ptr() as *const c_char;
        if strcmp(machine_name, expected) != 0 {
            vm_error(
                b"unsupported machine: %s\n\0".as_ptr() as *const c_char,
                machine_name,
            );
            return 1;
        }
        0
    }
}
}

// ── `((void) sizeof (__assert_single_arg ( p->ram_size >= (1 << 20) )), __extension__ ({ if ( p->ram…` · x86_machine.c:1972-1979 · chunk 4212 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((void) sizeof (__assert_single_arg (
//        p->ram_size >= (1 << 20)
//        )), __extension__ ({ if (
//        p->ram_size >= (1 << 20)
//        ) ; else __assert_fail (
//        "p->ram_size >= (1 << 20)"
//        , "x86_machine.c", 1972, __extension__ __PRETTY_FUNCTION__); }))
//                                        ;
//     
//         s = mallocz(sizeof(*s));
//         s->common.vmc = p->vmc;
//         s->ram_size = p->ram_size;
//     
//         s->port_map = phys_mem_map_init();
//         s->mem_map = phys_mem_map_init();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4212 {
// Rust port of the initialization part of pc_machine_init (x86_machine.c)

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VirtMachineClass {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct VIRTIODevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FBDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct BlockDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FSDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct X86CPUState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PIC2State {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PITState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct I440FXState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct CMOSState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct SerialState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct KBDState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PS2MouseState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PS2KbdState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct VMMouseState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct KvmRun {
    _opaque: [u8; 0],
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut FSDevice,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

const VM_FILE_COUNT: usize = 4;

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; VM_FILE_COUNT],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn phys_mem_map_init() -> *mut PhysMemoryMap;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    );
}

static ASSERT_MSG: &[u8] = b"p->ram_size >= (1 << 20)\0";
static ASSERT_FILE: &[u8] = b"x86_machine.c\0";
static ASSERT_FUNC: &[u8] = b"pc_machine_init\0";

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4212(p: *const VirtMachineParams) -> *mut PCMachine {
    let ram_size = (*p).ram_size;
    if !(ram_size >= (1u64 << 20)) {
        __assert_fail(
            ASSERT_MSG.as_ptr() as *const c_char,
            ASSERT_FILE.as_ptr() as *const c_char,
            1972,
            ASSERT_FUNC.as_ptr() as *const c_char,
        );
        return core::ptr::null_mut();
    }

    let s = mallocz(core::mem::size_of::<PCMachine>()) as *mut PCMachine;
    (*s).common.vmc = (*p).vmc;
    (*s).ram_size = (*p).ram_size;

    (*s).port_map = phys_mem_map_init();
    (*s).mem_map = phys_mem_map_init();

    s
}
}

// ── `if (p->accel_enable)` · x86_machine.c:1982-1984 · chunk 4213 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (p->accel_enable) {
//             kvm_init(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4213 {
// Rust port of the "if (p->accel_enable) { kvm_init(s); }" block of
// pc_machine_init (x86_machine.c) together with the file-static KVM helpers it
// needs (kvm_init, kvm_set_cpuid, kvm_pic_set_irq, kvm_map_ram,
// kvm_register_ram, kvm_set_ram_addr, kvm_get_dirty_bits, kvm_free_ram,
// sigalrm_handler).

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;

type BOOL = c_int;
const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

// ---------------------------------------------------------------- structs ---

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

pub type DeviceReadFn = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFn = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

pub type RegisterRamFn =
    unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange;
pub type FreeRamFn = unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange);
pub type GetDirtyBitsFn =
    unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32;
pub type SetRamAddrFn =
    unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, BOOL);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: BOOL,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFn>,
    pub write_func: Option<DeviceWriteFn>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRamFn>,
    pub free_ram: Option<FreeRamFn>,
    pub get_dirty_bits: Option<GetDirtyBitsFn>,
    pub set_ram_addr: Option<SetRamAddrFn>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

// Linux/KVM kernel ABI structures used here.

#[repr(C)]
pub struct KvmPitConfig {
    pub flags: u32,
    pub pad: [u32; 15],
}

#[repr(C)]
pub struct KvmCpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3],
}

#[repr(C)]
pub struct KvmCpuid2 {
    pub nent: u32,
    pub padding: u32,
    // kvm_cpuid_entry2 entries[];  (flexible array member)
}

#[repr(C)]
pub struct KvmUserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

#[repr(C)]
pub struct KvmDirtyLog {
    pub slot: u32,
    pub padding1: u32,
    pub dirty_bitmap: *mut c_void,
}

#[repr(C)]
pub struct KvmIrqLevel {
    pub irq: u32,
    pub level: u32,
}

// __sigset_t { unsigned long __val[1024 / (8 * sizeof(unsigned long))]; }
#[repr(C)]
pub struct SigSet {
    pub val: [c_ulong; 16],
}

// struct sigaction with the handler union represented by its first member.
#[repr(C)]
pub struct SigAction {
    pub sa_handler: usize,
    pub sa_mask: SigSet,
    pub sa_flags: c_int,
    pub sa_restorer: usize,
}

// ------------------------------------------------------------------- KVM ---

const IOC_NRSHIFT: u64 = 0;
const IOC_TYPESHIFT: u64 = 8;
const IOC_SIZESHIFT: u64 = 16;
const IOC_DIRSHIFT: u64 = 30;
const IOC_NONE: u64 = 0;
const IOC_WRITE: u64 = 1;
const IOC_READ: u64 = 2;
const KVMIO: u64 = 0xAE;

const fn ioc(dir: u64, ty: u64, nr: u64, size: u64) -> c_ulong {
    ((dir << IOC_DIRSHIFT) | (ty << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT))
        as c_ulong
}

const KVM_GET_API_VERSION: c_ulong = ioc(IOC_NONE, KVMIO, 0x00, 0);
const KVM_CREATE_VM: c_ulong = ioc(IOC_NONE, KVMIO, 0x01, 0);
const KVM_GET_VCPU_MMAP_SIZE: c_ulong = ioc(IOC_NONE, KVMIO, 0x04, 0);
const KVM_CREATE_VCPU: c_ulong = ioc(IOC_NONE, KVMIO, 0x41, 0);
const KVM_SET_TSS_ADDR: c_ulong = ioc(IOC_NONE, KVMIO, 0x47, 0);
const KVM_CREATE_IRQCHIP: c_ulong = ioc(IOC_NONE, KVMIO, 0x60, 0);
const KVM_SET_IDENTITY_MAP_ADDR: c_ulong =
    ioc(IOC_WRITE, KVMIO, 0x48, size_of::<u64>() as u64);
const KVM_CREATE_PIT2: c_ulong = ioc(IOC_WRITE, KVMIO, 0x77, size_of::<KvmPitConfig>() as u64);
const KVM_GET_SUPPORTED_CPUID: c_ulong = ioc(
    IOC_READ | IOC_WRITE,
    KVMIO,
    0x05,
    size_of::<KvmCpuid2>() as u64,
);
const KVM_SET_CPUID2: c_ulong = ioc(IOC_WRITE, KVMIO, 0x90, size_of::<KvmCpuid2>() as u64);
const KVM_SET_USER_MEMORY_REGION: c_ulong = ioc(
    IOC_WRITE,
    KVMIO,
    0x46,
    size_of::<KvmUserspaceMemoryRegion>() as u64,
);
const KVM_GET_DIRTY_LOG: c_ulong = ioc(IOC_WRITE, KVMIO, 0x42, size_of::<KvmDirtyLog>() as u64);
const KVM_IRQ_LINE: c_ulong = ioc(IOC_WRITE, KVMIO, 0x61, size_of::<KvmIrqLevel>() as u64);

const O_RDWR: c_int = 0o2;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_ANONYMOUS: c_int = 0x20;
const SIGALRM: c_int = 14;

// ---------------------------------------------------------------- externs ---

unsafe extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn close(fd: c_int) -> c_int;
    #[link_name = "open64"]
    fn open64(path: *const c_char, oflag: c_int, ...) -> c_int;
    #[link_name = "mmap64"]
    fn mmap64(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn sigemptyset(set: *mut SigSet) -> c_int;
    fn sigaction(sig: c_int, act: *const SigAction, oact: *mut SigAction) -> c_int;
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
    fn register_ram_entry(
        map: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;
    static mut stderr: *mut c_void;
}

// ------------------------------------------------------------- static fns ---

unsafe extern "C" fn sigalrm_handler(_sig: c_int) {}

unsafe extern "C" fn kvm_pic_set_irq(opaque: *mut c_void, irq_num: c_int, level: c_int) {
    let s = opaque as *mut PCMachine;
    let mut irq_level = KvmIrqLevel {
        irq: irq_num as u32,
        level: level as u32,
    };
    if ioctl(
        (*s).vm_fd,
        KVM_IRQ_LINE,
        &mut irq_level as *mut KvmIrqLevel as c_ulong,
    ) < 0
    {
        perror(b"KVM_IRQ_LINE\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn kvm_map_ram(mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut region: KvmUserspaceMemoryRegion = core::mem::zeroed();
    let mut flags: c_int = 0;
    let base = (*mem_map).phys_mem_range.as_ptr() as usize;

    region.slot = ((pr as usize).wrapping_sub(base) / size_of::<PhysMemoryRange>()) as u32;
    if (*pr).devram_flags & (1 << 0) != 0 {
        flags |= 1 << 1;
    }
    if (*pr).devram_flags & (1 << 1) != 0 {
        flags |= 1 << 0;
    }
    region.flags = flags as u32;
    region.guest_phys_addr = (*pr).addr;
    region.memory_size = (*pr).size;
    region.userspace_addr = (*pr).phys_mem as u64;

    if ioctl(
        (*s).vm_fd,
        KVM_SET_USER_MEMORY_REGION,
        &mut region as *mut KvmUserspaceMemoryRegion as c_ulong,
    ) < 0
    {
        perror(b"KVM_SET_USER_MEMORY_REGION\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn kvm_register_ram(
    mem_map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    let pr = register_ram_entry(mem_map, addr, size, devram_flags);

    let phys_mem = mmap64(
        core::ptr::null_mut(),
        size as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );
    if phys_mem.is_null() {
        return core::ptr::null_mut();
    }
    (*pr).phys_mem = phys_mem as *mut u8;
    if devram_flags & (1 << 1) != 0 {
        let n_pages: c_int = (size >> 12) as c_int;
        (*pr).dirty_bits_size = n_pages.wrapping_add(63) / 64 * 8;
        (*pr).dirty_bits = mallocz((*pr).dirty_bits_size as usize) as *mut u32;
    }

    if (*pr).size != 0 {
        kvm_map_ram(mem_map, pr);
    }
    pr
}

unsafe extern "C" fn kvm_set_ram_addr(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: BOOL,
) {
    if enabled != 0 {
        if (*pr).size == 0 || addr != (*pr).addr {
            (*pr).size = (*pr).org_size;
            (*pr).addr = addr;
            kvm_map_ram(mem_map, pr);
        }
    } else if (*pr).size != 0 {
        (*pr).addr = 0;
        (*pr).size = 0;
        kvm_map_ram(mem_map, pr);
    }
}

unsafe extern "C" fn kvm_get_dirty_bits(
    mem_map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
) -> *const u32 {
    let s = (*mem_map).opaque as *mut PCMachine;
    let mut dlog: KvmDirtyLog = core::mem::zeroed();

    if (*pr).size == 0 {
        core::ptr::write_bytes((*pr).dirty_bits, 0, (*pr).dirty_bits_size as usize);
    } else {
        let base = (*mem_map).phys_mem_range.as_ptr() as usize;
        dlog.slot = ((pr as usize).wrapping_sub(base) / size_of::<PhysMemoryRange>()) as u32;
        dlog.dirty_bitmap = (*pr).dirty_bits as *mut c_void;
        if ioctl(
            (*s).vm_fd,
            KVM_GET_DIRTY_LOG,
            &mut dlog as *mut KvmDirtyLog as c_ulong,
        ) < 0
        {
            perror(b"KVM_GET_DIRTY_LOG\0".as_ptr() as *const c_char);
            exit(1);
        }
    }
    (*pr).dirty_bits as *const u32
}

unsafe extern "C" fn kvm_free_ram(_mem_map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    munmap((*pr).phys_mem as *mut c_void, (*pr).org_size as usize);
    free((*pr).dirty_bits as *mut c_void);
}

unsafe extern "C" fn kvm_set_cpuid(s: *mut PCMachine) {
    let n_ent_max: c_int = 128;
    let entry_size = size_of::<KvmCpuidEntry2>();
    let kvm_cpuid = mallocz(size_of::<KvmCpuid2>() + (n_ent_max as usize) * entry_size)
        as *mut KvmCpuid2;

    (*kvm_cpuid).nent = n_ent_max as u32;
    if ioctl((*s).kvm_fd, KVM_GET_SUPPORTED_CPUID, kvm_cpuid as c_ulong) < 0 {
        perror(b"KVM_GET_SUPPORTED_CPUID\0".as_ptr() as *const c_char);
        exit(1);
    }

    let entries = (kvm_cpuid as *mut u8).add(size_of::<KvmCpuid2>()) as *mut KvmCpuidEntry2;
    let nent = (*kvm_cpuid).nent as c_int;
    let mut i: c_int = 0;
    while i < nent {
        let ent = entries.add(i as usize);
        if (*ent).function == 1 || (*ent).function == 0x80000001 {
            (*ent).edx &= !(((1u32) << 9) | ((1u32) << 22));
        }
        i += 1;
    }

    if ioctl((*s).vcpu_fd, KVM_SET_CPUID2, kvm_cpuid as c_ulong) < 0 {
        perror(b"KVM_SET_CPUID2\0".as_ptr() as *const c_char);
        exit(1);
    }
    free(kvm_cpuid as *mut c_void);
}

unsafe extern "C" fn kvm_init(s: *mut PCMachine) {
    let mut ret: c_int;
    let mut i: c_int;
    let mut act: SigAction = core::mem::zeroed();
    let mut pit_config: KvmPitConfig = core::mem::zeroed();
    let mut base_addr: u64;

    (*s).kvm_enabled = FALSE;
    (*s).kvm_fd = open64(b"/dev/kvm\0".as_ptr() as *const c_char, O_RDWR);
    if (*s).kvm_fd < 0 {
        fprintf(stderr, b"KVM not available\n\0".as_ptr() as *const c_char);
        return;
    }
    ret = ioctl((*s).kvm_fd, KVM_GET_API_VERSION, 0 as c_ulong);
    if ret < 0 {
        perror(b"KVM_GET_API_VERSION\0".as_ptr() as *const c_char);
        exit(1);
    }
    if ret != 12 {
        fprintf(stderr, b"Unsupported KVM version\n\0".as_ptr() as *const c_char);
        close((*s).kvm_fd);
        (*s).kvm_fd = -1;
        return;
    }
    (*s).vm_fd = ioctl((*s).kvm_fd, KVM_CREATE_VM, 0 as c_ulong);
    if (*s).vm_fd < 0 {
        perror(b"KVM_CREATE_VM\0".as_ptr() as *const c_char);
        exit(1);
    }

    base_addr = 0xfffbc000;
    if ioctl(
        (*s).vm_fd,
        KVM_SET_IDENTITY_MAP_ADDR,
        &mut base_addr as *mut u64 as c_ulong,
    ) < 0
    {
        perror(b"KVM_SET_IDENTITY_MAP_ADDR\0".as_ptr() as *const c_char);
        exit(1);
    }

    if ioctl(
        (*s).vm_fd,
        KVM_SET_TSS_ADDR,
        base_addr.wrapping_add(0x1000) as c_ulong,
    ) < 0
    {
        perror(b"KVM_SET_TSS_ADDR\0".as_ptr() as *const c_char);
        exit(1);
    }

    if ioctl((*s).vm_fd, KVM_CREATE_IRQCHIP, 0 as c_ulong) < 0 {
        perror(b"KVM_CREATE_IRQCHIP\0".as_ptr() as *const c_char);
        exit(1);
    }

    pit_config.flags = 1;
    if ioctl(
        (*s).vm_fd,
        KVM_CREATE_PIT2,
        &mut pit_config as *mut KvmPitConfig as c_ulong,
    ) != 0
    {
        perror(b"KVM_CREATE_PIT2\0".as_ptr() as *const c_char);
        exit(1);
    }

    (*s).vcpu_fd = ioctl((*s).vm_fd, KVM_CREATE_VCPU, 0 as c_ulong);
    if (*s).vcpu_fd < 0 {
        perror(b"KVM_CREATE_VCPU\0".as_ptr() as *const c_char);
        exit(1);
    }

    kvm_set_cpuid(s);

    (*s).kvm_run_size = ioctl((*s).kvm_fd, KVM_GET_VCPU_MMAP_SIZE, 0 as c_ulong);
    if (*s).kvm_run_size < 0 {
        perror(b"KVM_GET_VCPU_MMAP_SIZE\0".as_ptr() as *const c_char);
        exit(1);
    }

    (*s).kvm_run = mmap64(
        core::ptr::null_mut(),
        (*s).kvm_run_size as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        (*s).vcpu_fd,
        0,
    );
    if (*s).kvm_run.is_null() {
        perror(b"mmap kvm_run\0".as_ptr() as *const c_char);
        exit(1);
    }

    i = 0;
    while i < 16 {
        irq_init(
            &mut (*s).pic_irq[i as usize] as *mut IRQSignal,
            Some(kvm_pic_set_irq),
            s as *mut c_void,
            i,
        );
        i += 1;
    }

    act.sa_handler = sigalrm_handler as usize;
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0;
    sigaction(SIGALRM, &act as *const SigAction, core::ptr::null_mut());

    (*s).kvm_enabled = TRUE;

    (*(*s).mem_map).register_ram = Some(kvm_register_ram);
    (*(*s).mem_map).free_ram = Some(kvm_free_ram);
    (*(*s).mem_map).get_dirty_bits = Some(kvm_get_dirty_bits);
    (*(*s).mem_map).set_ram_addr = Some(kvm_set_ram_addr);
    (*(*s).mem_map).opaque = s as *mut c_void;
}

// ---------------------------------------------------------------- exported ---

#[no_mangle]
pub extern "C" fn pc_machine_init__part4213(s: *mut PCMachine, accel_enable: c_int) {
    unsafe {
        if accel_enable != 0 {
            kvm_init(s);
        }
    }
}
}

// ── `if (!s->kvm_enabled)` · x86_machine.c:1988-1998 · chunk 4214 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s->kvm_enabled)
//     
//         {
//             s->cpu_state = x86_cpu_init(s->mem_map);
//             x86_cpu_set_get_tsc(s->cpu_state, cpu_get_tsc, s);
//             x86_cpu_set_port_io(s->cpu_state, ld_port, st_port, s);
//     
//     
//             s->mem_map->opaque = s;
//             s->mem_map->flush_tlb_write_range = pc_flush_tlb_write_range;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4214 {
// Port of the non-KVM CPU setup block of pc_machine_init (x86_machine.c 1988-1998)

use core::ffi::{c_int, c_void};

type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange,
    >,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32,
    >,
    pub set_ram_addr: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int),
    >,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

extern "C" {
    fn x86_cpu_init(mem_map: *mut PhysMemoryMap) -> *mut c_void;
    fn x86_cpu_set_get_tsc(
        s: *mut c_void,
        get_tsc: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
        opaque: *mut c_void,
    );
    fn x86_cpu_set_port_io(
        s: *mut c_void,
        port_read: Option<DeviceReadFunc>,
        port_write: Option<DeviceWriteFunc>,
        opaque: *mut c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4214(
    s: *mut PCMachine,
    get_tsc: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
    ld_port: Option<DeviceReadFunc>,
    st_port: Option<DeviceWriteFunc>,
    flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
) {
    if (*s).kvm_enabled == 0 {
        let mem_map = (*s).mem_map;
        let cpu_state = x86_cpu_init(mem_map);
        (*s).cpu_state = cpu_state;
        x86_cpu_set_get_tsc(cpu_state, get_tsc, s as *mut c_void);
        x86_cpu_set_port_io(cpu_state, ld_port, st_port, s as *mut c_void);

        (*mem_map).opaque = s as *mut c_void;
        (*mem_map).flush_tlb_write_range = flush_tlb_write_range;
    }
}
}

// ── `cpu_register_ram(s->mem_map, 0xc0000, p->ram_size - 0xc0000, 0); cpu_register_ram(s->mem_map, 0…` · x86_machine.c:2001-2008 · chunk 4215 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     cpu_register_ram(s->mem_map, 0xc0000, p->ram_size - 0xc0000, 0);
//         cpu_register_ram(s->mem_map, 0, 0xa0000, 0);
//     
//     
//         cpu_register_device(s->port_map, 0x80, 2, s, port80_read, port80_write,
//                             (1 << 0));
//         cpu_register_device(s->port_map, 0x92, 2, s, port92_read, port92_write,
//                             (1 << 0));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4215 {
// Ported piece of pc_machine_init: RAM/port registration for the PC machine.

use core::ffi::{c_int, c_void};

type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

pub type RegisterRamFunc =
    unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange;
pub type FreeRamFunc = unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange);
pub type GetDirtyBitsFunc =
    unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32;
pub type SetRamAddrFunc =
    unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int);
pub type FlushTlbWriteRangeFunc = unsafe extern "C" fn(*mut c_void, *mut u8, usize);

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRamFunc>,
    pub free_ram: Option<FreeRamFunc>,
    pub get_dirty_bits: Option<GetDirtyBitsFunc>,
    pub set_ram_addr: Option<SetRamAddrFunc>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<FlushTlbWriteRangeFunc>,
}

extern "C" {
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

// The C `static inline cpu_register_ram` has no linker symbol; call the same
// function pointer it dispatches through.
unsafe fn cpu_register_ram(s: *mut PhysMemoryMap, addr: u64, size: u64, devram_flags: c_int) {
    let f = match (*s).register_ram {
        Some(f) => f,
        None => return,
    };
    f(s, addr, size, devram_flags);
}

unsafe extern "C" fn port80_read(_opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    0xff
}

unsafe extern "C" fn port80_write(_opaque: *mut c_void, _offset: u32, _val: u32, _size_log2: c_int) {
}

unsafe extern "C" fn port92_read(_opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let a20: c_int = 1;
    (a20 << 1) as u32
}

unsafe extern "C" fn port92_write(_opaque: *mut c_void, _offset: u32, _val: u32, _size_log2: c_int) {
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4215(
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    ram_size: u64,
    opaque: *mut c_void,
) {
    cpu_register_ram(mem_map, 0xc0000, ram_size.wrapping_sub(0xc0000), 0);
    cpu_register_ram(mem_map, 0, 0xa0000, 0);

    cpu_register_device(
        port_map,
        0x80,
        2,
        opaque,
        Some(port80_read),
        Some(port80_write),
        1 << 0,
    );
    cpu_register_device(
        port_map,
        0x92,
        2,
        opaque,
        Some(port92_read),
        Some(port92_write),
        1 << 0,
    );
}
}

// ── `if (p->files[VM_FILE_BIOS].len > 0)` · x86_machine.c:2011-2033 · chunk 4216 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (p->files[VM_FILE_BIOS].len > 0) {
//             int bios_size, bios_size1;
//             uint8_t *bios_buf, *ptr;
//             uint32_t bios_addr;
//     
//             bios_size = p->files[VM_FILE_BIOS].len;
//             bios_buf = p->files[VM_FILE_BIOS].buf;
//             
//            ((void) sizeof (__assert_single_arg (
//            (bios_size % 65536) == 0 && bios_size != 0
//            )), __extension__ ({ if (
//            (bios_size % 65536) == 0 && bios_size != 0
//            ) ; else __assert_fail (
//            "(bios_size % 65536) == 0 && bios_size != 0"
//            , "x86_machine.c", 2018, __extension__ __PRETTY_FUNCTION__); }))
//                                                              ;
//             bios_addr = -bios_size;
//     
//             cpu_register_ram(s->mem_map, bios_addr, bios_size, (1 << 0));
//             ptr = get_ram_ptr(s, bios_addr);
//             memcpy(ptr, bios_buf, bios_size);
//     
//             bios_size1 = min_int(bios_size, 128 * 1024);
//             ptr = get_ram_ptr(s, 0x100000 - bios_size1);
//             memcpy(ptr, bios_buf + bios_size - bios_size1, bios_size1);
//     
//     
//     
//     
//     
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4216 {
// Port of the BIOS loading block of pc_machine_init (x86_machine.c lines 2011-2033)
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn __assert_fail(assertion: *const c_char, file: *const c_char, line: c_uint,
                     function: *const c_char) -> !;
}

// static uint8_t *get_ram_ptr(PCMachine *s, uint64_t paddr)
unsafe fn get_ram_ptr(s: *mut PhysMemoryMap, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range(s, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return core::ptr::null_mut();
    }
    let off: u64 = paddr.wrapping_sub((*pr).addr);
    (*pr).phys_mem.add(off as usize)
}

// static inline PhysMemoryRange *cpu_register_ram(PhysMemoryMap *s, uint64_t addr,
//                                                 uint64_t size, int devram_flags)
unsafe fn cpu_register_ram(s: *mut PhysMemoryMap, addr: u64, size: u64, devram_flags: c_int) {
    if s.is_null() {
        return;
    }
    if let Some(f) = (*s).register_ram {
        f(s, addr, size, devram_flags);
    }
}

static ASSERT_MSG: [u8; 43] = *b"(bios_size % 65536) == 0 && bios_size != 0\0";
static ASSERT_FILE: [u8; 14] = *b"x86_machine.c\0";
static ASSERT_FUNC: [u8; 16] = *b"pc_machine_init\0";

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4216(mem_map: *mut PhysMemoryMap,
                                                   bios_buf: *mut u8,
                                                   bios_size: c_int) {
    let bios_size1: c_int;
    let mut ptr: *mut u8;
    let bios_addr: u32;

    if !((bios_size % 65536) == 0 && bios_size != 0) {
        __assert_fail(ASSERT_MSG.as_ptr() as *const c_char,
                      ASSERT_FILE.as_ptr() as *const c_char,
                      2018,
                      ASSERT_FUNC.as_ptr() as *const c_char);
    }

    bios_addr = (bios_size as u32).wrapping_neg();

    cpu_register_ram(mem_map, bios_addr as u64, (bios_size as i64) as u64, 1 << 0);
    ptr = get_ram_ptr(mem_map, bios_addr as u64);
    memcpy(ptr as *mut c_void, bios_buf as *const c_void, bios_size as usize);

    bios_size1 = if bios_size < 128 * 1024 { bios_size } else { 128 * 1024 };
    ptr = get_ram_ptr(mem_map, (0x100000 as i64 - bios_size1 as i64) as u64);
    memcpy(ptr as *mut c_void,
           bios_buf.add((bios_size - bios_size1) as usize) as *const c_void,
           bios_size1 as usize);
}
}

// ── `if (!s->kvm_enabled)` · x86_machine.c:2036-2046 · chunk 4217 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s->kvm_enabled)
//     
//         {
//             s->pic_state = pic2_init(s->port_map, 0x20, 0xa0,
//                                      0x4d0, 0x4d1,
//                                      pic_set_irq_cb, s,
//                                      s->pic_irq);
//             x86_cpu_set_get_hard_intno(s->cpu_state, get_hard_intno_cb, s);
//             s->pit_state = pit_init(s->port_map, 0x40, 0x61, &s->pic_irq[0],
//                                     pit_get_ticks_cb, s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4217 {
// Port of the "!s->kvm_enabled" PIC/PIT setup block of pc_machine_init.
use core::ffi::{c_int, c_void};

// C function pointer typedefs (OPAQUE and callback types are passed through).
type SetIrqCb = Option<unsafe extern "C" fn(*mut c_void, c_int)>;
type GetHardIntnoCb = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
type GetTicksCb = Option<unsafe extern "C" fn(*mut c_void) -> i64>;

extern "C" {
    fn pic2_init(
        port_map: *mut c_void,
        addr0: u32,
        addr1: u32,
        elcr_addr0: u32,
        elcr_addr1: u32,
        cpu_set_irq: SetIrqCb,
        opaque: *mut c_void,
        irqs: *mut c_void,
    ) -> *mut c_void;

    fn x86_cpu_set_get_hard_intno(
        s: *mut c_void,
        get_hard_intno: GetHardIntnoCb,
        opaque: *mut c_void,
    );

    fn pit_init(
        port_map: *mut c_void,
        addr0: c_int,
        addr1: c_int,
        irq: *mut c_void,
        get_ticks: GetTicksCb,
        opaque: *mut c_void,
    ) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4217(
    s: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut *mut c_void,
    pic_irq: *mut c_void,
    pit_state: *mut *mut c_void,
    set_irq_cb: SetIrqCb,
    get_hard_intno_cb: GetHardIntnoCb,
    get_ticks_cb: GetTicksCb,
) {
    if !pic_state.is_null() {
        *pic_state = pic2_init(
            port_map,
            0x20u32,
            0xa0u32,
            0x4d0u32,
            0x4d1u32,
            set_irq_cb,
            s,
            pic_irq,
        );
    }
    x86_cpu_set_get_hard_intno(cpu_state, get_hard_intno_cb, s);
    if !pit_state.is_null() {
        *pit_state = pit_init(port_map, 0x40, 0x61, pic_irq, get_ticks_cb, s);
    }
}
}

// ── `s->cmos_state = cmos_init(s->port_map, 0x70, &s->pic_irq[8], p->rtc_local_time);` · x86_machine.c:2048-2049 · chunk 4218 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cmos_state = cmos_init(s->port_map, 0x70, &s->pic_irq[8],
//                                   p->rtc_local_time);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4218 {
// Port of the cmos_init call from pc_machine_init (x86_machine.c).
use core::ffi::{c_int, c_void};

extern "C" {
    // CMOSState *cmos_init(PhysMemoryMap *port_map, int addr,
    //                      IRQSignal *irq, BOOL use_local_time);
    fn cmos_init(
        port_map: *mut c_void,
        addr: c_int,
        irq: *mut c_void,
        use_local_time: c_int,
    ) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4218(
    port_map: *mut c_void,
    irq: *mut c_void,
    use_local_time: c_int,
) -> *mut c_void {
    unsafe { cmos_init(port_map, 0x70, irq, use_local_time) }
}
}

// ── `{ … }` · x86_machine.c:2052-2062 · chunk 4219 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//             int size;
//     
//             size = min_int((s->ram_size - (1 << 20)) >> 10, 65535);
//             put_le16(s->cmos_state->cmos_data + 0x30, size);
//             if (s->ram_size >= (16 << 20)) {
//                 size = min_int((s->ram_size - (16 << 20)) >> 16, 65535);
//                 put_le16(s->cmos_state->cmos_data + 0x34, size);
//             }
//             s->cmos_state->cmos_data[0x14] = 0x06;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4219 {
// Mirror of static inline int min_int(int a, int b)
fn min_int(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// Mirror of static inline void put_le16(uint8_t *ptr, uint16_t v)
unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4219(ram_size: u64, cmos_data: *mut u8) {
    // size = min_int((s->ram_size - (1 << 20)) >> 10, 65535);
    let mut size: i32 =
        min_int(ram_size.wrapping_sub(1u64 << 20).wrapping_shr(10) as i32, 65535);
    unsafe {
        put_le16(cmos_data.add(0x30), size as u16);
    }
    if ram_size >= (16u64 << 20) {
        size = min_int(ram_size.wrapping_sub(16u64 << 20).wrapping_shr(16) as i32, 65535);
        unsafe {
            put_le16(cmos_data.add(0x34), size as u16);
        }
    }
    unsafe {
        *cmos_data.add(0x14) = 0x06;
    }
}
}

// ── `s->i440fx_state = i440fx_init(&pci_bus, &piix3_devfn, s->mem_map, s->port_map, s->pic_irq); s->…` · x86_machine.c:2064-2067 · chunk 4220 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->i440fx_state = i440fx_init(&pci_bus, &piix3_devfn, s->mem_map,
//                                       s->port_map, s->pic_irq);
//     
//         s->common.console = p->console;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4220 {
// Portion of pc_machine_init: the i440fx setup plus wiring the console.
use core::ffi::{c_int, c_void};

extern "C" {
    // I440FXState *i440fx_init(PCIBus **pbus, int *ppiix3_devfn,
    //                          PhysMemoryMap *mem_map, PhysMemoryMap *port_map,
    //                          IRQSignal *pic_irqs);
    fn i440fx_init(
        pbus: *mut *mut c_void,
        ppiix3_devfn: *mut c_int,
        mem_map: *mut c_void,
        port_map: *mut c_void,
        pic_irqs: *mut c_void,
    ) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4220(
    i440fx_state_out: *mut *mut c_void,
    console_out: *mut *mut c_void,
    pci_bus: *mut *mut c_void,
    piix3_devfn: *mut c_int,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    pic_irqs: *mut c_void,
    console: *mut c_void,
) {
    let state = i440fx_init(pci_bus, piix3_devfn, mem_map, port_map, pic_irqs);
    if !i440fx_state_out.is_null() {
        *i440fx_state_out = state;
    }
    if !console_out.is_null() {
        *console_out = console;
    }
}
}

// ── `if (0)` · x86_machine.c:2069-2072 · chunk 4221 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (0) {
//         s->serial_state = serial_init(s->port_map, 0x3f8, &s->pic_irq[4],
//                                       serial_write_cb, s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4221 {
// Port of the (dead) serial-init block inside pc_machine_init.
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,

    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,

    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

type SerialWriteFunc = Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>;

extern "C" {
    fn serial_init(
        port_map: *mut c_void,
        addr: c_int,
        irq: *mut IRQSignal,
        write_func: SerialWriteFunc,
        opaque: *mut c_void,
    ) -> *mut c_void;
}

// Ported from the static C function serial_write_cb (no linker symbol).
unsafe extern "C" fn serial_write_cb(opaque: *mut c_void, buf: *const u8, buf_len: c_int) {
    let s = opaque as *mut PCMachine;
    let console = (*s).common.console;
    if !console.is_null() {
        if let Some(write_data) = (*console).write_data {
            write_data((*console).opaque, buf, buf_len);
        }
    }
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4221(s: *mut PCMachine) {
    unsafe {
        if 0 != 0 {
            let irq: *mut IRQSignal = (*s).pic_irq.as_mut_ptr().add(4);
            (*s).serial_state = serial_init(
                (*s).port_map,
                0x3f8,
                irq,
                Some(serial_write_cb),
                s as *mut c_void,
            );
        }
    }
}
}

// ── `memset(vbus, 0, sizeof(*vbus)); vbus->pci_bus = pci_bus;` · x86_machine.c:2074-2075 · chunk 4222 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(vbus, 0, sizeof(*vbus));
//         vbus->pci_bus = pci_bus;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4222 {
// Port of pc_machine_init lines: memset(vbus, 0, sizeof(*vbus)); vbus->pci_bus = pci_bus;
use core::ffi::c_void;

// Opaque C types: only used through pointers.
#[repr(C)]
pub struct PCIBus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut PCIBus,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4222(
    vbus: *mut VIRTIOBusDef,
    pci_bus: *mut PCIBus,
) {
    if vbus.is_null() {
        return;
    }
    core::ptr::write_bytes(
        vbus as *mut u8,
        0,
        core::mem::size_of::<VIRTIOBusDef>(),
    );
    (*vbus).pci_bus = pci_bus;
    let _ = core::mem::size_of::<c_void>();
}
}

// ── `if (p->console)` · x86_machine.c:2077-2080 · chunk 4223 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->console) {
//     
//             s->common.console_dev = virtio_console_init(vbus, p->console);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4223 {
// Ported piece of pc_machine_init: conditional virtio console setup.
#![allow(non_camel_case_types)]

#[repr(C)]
pub struct VIRTIOBusDef {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VIRTIODevice {
    _opaque: [u8; 0],
}

extern "C" {
    fn virtio_console_init(bus: *mut VIRTIOBusDef, cs: *mut CharacterDevice) -> *mut VIRTIODevice;
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4223(
    vbus: *mut VIRTIOBusDef,
    p_console: *mut CharacterDevice,
    console_dev_out: *mut *mut VIRTIODevice,
) {
    // if (p->console) { s->common.console_dev = virtio_console_init(vbus, p->console); }
    if !p_console.is_null() && !console_dev_out.is_null() {
        unsafe {
            *console_dev_out = virtio_console_init(vbus, p_console);
        }
    }
}
}

// ── `for (i = 0; i < p->drive_count; )` · x86_machine.c:2083-2099 · chunk 4224 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->drive_count;) {
//             const VMDriveEntry *de = &p->tab_drive[i];
//     
//             if (!de->device || !strcmp(de->device, "virtio")) {
//                 virtio_block_init(vbus, p->tab_drive[i].block_dev);
//                 i++;
//             } else if (!strcmp(de->device, "ide")) {
//                 BlockDevice *tab_bs[2];
//     
//                 tab_bs[0] = p->tab_drive[i++].block_dev;
//                 tab_bs[1] = 
//                            ((void *)0)
//                                ;
//                 if (i < p->drive_count)
//                     tab_bs[1] = p->tab_drive[i++].block_dev;
//                 ide_init(s->port_map, 0x1f0, 0x3f6, &s->pic_irq[14], tab_bs);
//                 piix3_ide_init(pci_bus, piix3_devfn + 1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4224 {
// Port of the drive-initialization loop from pc_machine_init.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count:
        Option<unsafe extern "C" fn(bs: *mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            bs: *mut BlockDevice,
            sector_num: u64,
            buf: *mut u8,
            n: c_int,
            cb: *mut c_void,
            opaque: *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            bs: *mut BlockDevice,
            sector_num: u64,
            buf: *const u8,
            n: c_int,
            cb: *mut c_void,
            opaque: *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut PCIBus,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

// VirtMachineParams, laid out exactly as in C (up to the drive table).
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
}

#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

// PCMachine: only the prefix needed to reach port_map and pic_irq[14].
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn virtio_block_init(bus: *mut VIRTIOBusDef, bs: *mut BlockDevice) -> *mut c_void;
    fn ide_init(
        port_map: *mut PhysMemoryMap,
        addr: u32,
        addr2: u32,
        irq: *mut IRQSignal,
        tab_bs: *mut *mut BlockDevice,
    ) -> *mut c_void;
    fn piix3_ide_init(pci_bus: *mut PCIBus, devfn: c_int) -> *mut c_void;
}

static VIRTIO_STR: [u8; 7] = [b'v', b'i', b'r', b't', b'i', b'o', 0];
static IDE_STR: [u8; 4] = [b'i', b'd', b'e', 0];

/// Runs the drive-init loop. `*i` starts at the caller's `i` and is left at the
/// value the C loop would have had after it finished.
#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4224(
    i: *mut c_int,
    p: *const VirtMachineParams,
    vbus: *mut VIRTIOBusDef,
    s: *mut PCMachine,
    pci_bus: *mut PCIBus,
    piix3_devfn: c_int,
) {
    if i.is_null() || p.is_null() {
        return;
    }
    let p = &*p;
    let iv = i;
    *iv = 0;

    while *iv < p.drive_count {
        let de: *const VMDriveEntry = &p.tab_drive[*iv as usize];

        if (*de).device.is_null()
            || strcmp((*de).device, VIRTIO_STR.as_ptr() as *const c_char) == 0
        {
            virtio_block_init(vbus, p.tab_drive[*iv as usize].block_dev);
            *iv += 1;
        } else if strcmp((*de).device, IDE_STR.as_ptr() as *const c_char) == 0 {
            let mut tab_bs: [*mut BlockDevice; 2] = [core::ptr::null_mut(); 2];

            tab_bs[0] = p.tab_drive[*iv as usize].block_dev;
            *iv += 1;
            tab_bs[1] = core::ptr::null_mut();
            if *iv < p.drive_count {
                tab_bs[1] = p.tab_drive[*iv as usize].block_dev;
                *iv += 1;
            }
            let port_map = if s.is_null() {
                core::ptr::null_mut()
            } else {
                (*s).port_map
            };
            let irq: *mut IRQSignal = if s.is_null() {
                core::ptr::null_mut()
            } else {
                &mut (*s).pic_irq[14] as *mut IRQSignal
            };
            ide_init(port_map, 0x1f0, 0x3f6, irq, tab_bs.as_mut_ptr());
            piix3_ide_init(pci_bus, piix3_devfn.wrapping_add(1));
        }
    }
}
}

// ── `for (i = 0; i < p->fs_count; i++)` · x86_machine.c:2102-2105 · chunk 4225 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->fs_count; i++) {
//             virtio_9p_init(vbus, p->tab_fs[i].fs_dev,
//                            p->tab_fs[i].tag);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4225 {
// Ported piece of pc_machine_init: the virtio-9p device registration loop
// for (i = 0; i < p->fs_count; i++)
//     virtio_9p_init(vbus, p->tab_fs[i].fs_dev, p->tab_fs[i].tag);

use core::ffi::{c_char, c_int};

// Opaque C types used only through pointers.
pub enum VIRTIODevice {}
pub enum FSDevice {}
pub enum PCIBus {}
pub enum PhysMemoryMap {}
pub enum IRQSignal {}
pub enum BlockDevice {}
pub enum CharacterDevice {}
pub enum EthernetDevice {}
pub enum VirtMachineClass {}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut PCIBus,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut FSDevice,
}

// Only the prefix of VirtMachineParams needed here (up to and including
// fs_count) is mirrored; the C layout of those fields is preserved exactly.
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
}

extern "C" {
    fn virtio_9p_init(
        bus: *mut VIRTIOBusDef,
        fs: *mut FSDevice,
        mount_tag: *const c_char,
    ) -> *mut VIRTIODevice;
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4225(
    p: *const VirtMachineParams,
    vbus: *mut VIRTIOBusDef,
    i: *mut c_int,
) {
    let tab: *const VMFSEntry = core::ptr::addr_of!((*p).tab_fs) as *const VMFSEntry;
    let count: c_int = (*p).fs_count;
    let mut idx: c_int = 0;
    while idx < count {
        let de: *const VMFSEntry = tab.add(idx as usize);
        virtio_9p_init(vbus, (*de).fs_dev, (*de).tag as *const c_char);
        idx += 1;
    }
    *i = idx;
}
}

// ── `if (p->display_device)` · x86_machine.c:2107-2127 · chunk 4226 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->display_device) {
//             FBDevice *fb_dev;
//     
//             fb_dev = mallocz(sizeof(*fb_dev));
//             s->common.fb_dev = fb_dev;
//             if (!strcmp(p->display_device, "vga")) {
//                 int bios_size;
//                 uint8_t *bios_buf;
//                 bios_size = p->files[VM_FILE_VGA_BIOS].len;
//                 bios_buf = p->files[VM_FILE_VGA_BIOS].buf;
//                 pci_vga_init(pci_bus, fb_dev, p->width, p->height,
//                              bios_buf, bios_size);
//             } else if (!strcmp(p->display_device, "simplefb")) {
//                 simplefb_init(s->mem_map,
//                               0xf0400000,
//                               fb_dev, p->width, p->height);
//             } else {
//                 vm_error("unsupported display device: %s\n", p->display_device);
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4226 {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

// Opaque types only passed through to other C functions.
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VGAState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct SimpleFBState {
    _opaque: [u8; 0],
}

// Mirrors struct FBDevice from the C translation unit.
#[repr(C)]
pub struct FBDevice {
    width: c_int,
    height: c_int,
    stride: c_int,
    fb_data: *mut u8,
    fb_size: c_int,
    device_opaque: *mut c_void,
    refresh: *mut c_void,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn pci_vga_init(
        bus: *mut PCIBus,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
        vga_rom_buf: *const u8,
        vga_rom_size: c_int,
    ) -> *mut VGAState;
    fn simplefb_init(
        map: *mut PhysMemoryMap,
        phys_addr: u64,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
    ) -> *mut SimpleFBState;
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4226(
    mem_map: *mut PhysMemoryMap,
    pci_bus: *mut PCIBus,
    fb_dev_slot: *mut *mut FBDevice,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    vga_bios_buf: *mut u8,
    vga_bios_len: c_int,
) {
    let fb_dev = mallocz(core::mem::size_of::<FBDevice>()) as *mut FBDevice;
    *fb_dev_slot = fb_dev;

    if strcmp(display_device, b"vga\0".as_ptr() as *const c_char) == 0 {
        pci_vga_init(
            pci_bus,
            fb_dev,
            width,
            height,
            vga_bios_buf as *const u8,
            vga_bios_len,
        );
    } else if strcmp(display_device, b"simplefb\0".as_ptr() as *const c_char) == 0 {
        simplefb_init(mem_map, 0xf0400000u64, fb_dev, width, height);
    } else {
        vm_error(
            b"unsupported display device: %s\n\0".as_ptr() as *const c_char,
            display_device,
        );
        exit(1);
    }
}
}

// ── `if (p->input_device)` · x86_machine.c:2129-2147 · chunk 4227 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->input_device) {
//             if (!strcmp(p->input_device, "virtio")) {
//                 s->keyboard_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_KEYBOARD);
//     
//                 s->mouse_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_TABLET);
//             } else if (!strcmp(p->input_device, "ps2")) {
//                 s->kbd_state = i8042_init(&s->ps2_kbd, &s->ps2_mouse,
//                                           s->port_map,
//                                           &s->pic_irq[1], &s->pic_irq[12], 0x60);
//     
//                 cpu_register_device(s->port_map, 0x5658, 1, s,
//                                     vmport_read, vmport_write,
//                         (1 << 2));
//                 s->vm_mouse = vmmouse_init(s->ps2_mouse);
//             } else {
//                 vm_error("unsupported input device: %s\n", p->input_device);
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4227 {
// Port of the input-device setup block of pc_machine_init (x86_machine.c 2129-2147).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut c_void,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn virtio_input_init(bus: *mut VIRTIOBusDef, type_: c_int) -> *mut c_void;
    fn i8042_init(
        pkbd: *mut *mut c_void,
        pmouse: *mut *mut c_void,
        port_map: *mut c_void,
        kbd_irq: *mut IRQSignal,
        mouse_irq: *mut IRQSignal,
        io_base: u32,
    ) -> *mut c_void;
    fn cpu_register_device(
        s: *mut c_void,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
        write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
        devio_flags: c_int,
    ) -> *mut c_void;
    fn vmmouse_init(ps2_mouse: *mut c_void) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(code: c_int) -> !;
}

const VIRTIO_INPUT_TYPE_KEYBOARD: c_int = 0;
const VIRTIO_INPUT_TYPE_TABLET: c_int = 2;

static VIRTIO: &[u8] = b"virtio\0";
const PS2: &[u8] = b"ps2\0";
const UNSUPPORTED: &[u8] = b"unsupported input device: %s\n\0";

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4227(
    input_device: *const c_char,
    s: *mut PCMachine,
    vbus: *mut VIRTIOBusDef,
    vmport_read: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    vmport_write: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
) {
    if input_device.is_null() {
        return;
    }

    if strcmp(input_device, VIRTIO.as_ptr() as *const c_char) == 0 {
        (*s).keyboard_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_KEYBOARD);
        (*s).mouse_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_TABLET);
    } else if strcmp(input_device, PS2.as_ptr() as *const c_char) == 0 {
        (*s).kbd_state = i8042_init(
            core::ptr::addr_of_mut!((*s).ps2_kbd),
            core::ptr::addr_of_mut!((*s).ps2_mouse),
            (*s).port_map,
            (*s).pic_irq.as_mut_ptr().add(1),
            (*s).pic_irq.as_mut_ptr().add(12),
            0x60,
        );

        cpu_register_device(
            (*s).port_map,
            0x5658,
            1,
            s as *mut c_void,
            vmport_read,
            vmport_write,
            1 << 2,
        );
        (*s).vm_mouse = vmmouse_init((*s).ps2_mouse);
    } else {
        vm_error(UNSUPPORTED.as_ptr() as *const c_char, input_device);
        exit(1);
    }
}
}

// ── `for (i = 0; i < p->eth_count; i++)` · x86_machine.c:2150-2153 · chunk 4228 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->eth_count; i++) {
//             virtio_net_init(vbus, p->tab_eth[i].net);
//             s->common.net = p->tab_eth[i].net;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4228 {
// Port of the virtio-net setup loop from pc_machine_init.
use core::ffi::{c_char, c_int, c_void};

// Opaque handles: only ever used behind pointers.
pub enum EthernetDevice {}

#[repr(C)]
pub struct VMEthEntry {
    pub device: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

extern "C" {
    fn virtio_net_init(bus: *mut c_void, es: *mut EthernetDevice) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_init__part4228(
    vbus: *mut c_void,
    eth_count: c_int,
    tab_eth: *const VMEthEntry,
    pnet: *mut *mut EthernetDevice,
) {
    let mut i: c_int = 0;
    while i < eth_count {
        let net = (*tab_eth.add(i as usize)).net;
        virtio_net_init(vbus, net);
        *pnet = net;
        i += 1;
    }
}
}

// ── `if (p->files[VM_FILE_KERNEL].buf)` · x86_machine.c:2155-2159 · chunk 4229 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->files[VM_FILE_KERNEL].buf) {
//             copy_kernel(s, p->files[VM_FILE_KERNEL].buf,
//                         p->files[VM_FILE_KERNEL].len,
//                         p->cmdline ? p->cmdline : "");
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4229 {
// Port of the kernel-loading tail of pc_machine_init() (x86_machine.c).
// C's static helper copy_kernel() has no linker symbol, so its logic lives here
// as a private Rust function; get_ram_ptr() (also static) is ported the same way.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// ---------------------------------------------------------------- opaque types

#[repr(C)]
pub struct Opaque {
    _private: [u8; 0],
}

pub type PhysMemoryMap = Opaque;
pub type X86CPUState = Opaque;
pub type PIC2State = Opaque;
pub type I440FXState = Opaque;
pub type PITState = Opaque;
pub type CMOSState = Opaque;
pub type SerialState = Opaque;
pub type VIRTIODevice = Opaque;
pub type CharacterDevice = Opaque;
pub type EthernetDevice = Opaque;
pub type VirtMachineClass = Opaque;
pub type KBDState = Opaque;
pub type PS2MouseState = Opaque;
pub type PS2KbdState = Opaque;
pub type VMMouseState = Opaque;
pub type KvmRun = Opaque;
pub type File = Opaque;

// ------------------------------------------------------------------- structures

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: *mut c_void,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

#[repr(C)]
pub struct X86CPUSeg {
    pub sel: u16,
    pub flags: u16,
    pub base: u32,
    pub limit: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmSegment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub type_: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    pub unusable: u8,
    pub padding: u8,
}

#[repr(C)]
pub struct KvmDtable {
    pub base: u64,
    pub limit: u16,
    pub padding: [u16; 3],
}

#[repr(C)]
pub struct KvmSregs {
    pub cs: KvmSegment,
    pub ds: KvmSegment,
    pub es: KvmSegment,
    pub fs: KvmSegment,
    pub gs: KvmSegment,
    pub ss: KvmSegment,
    pub tr: KvmSegment,
    pub ldt: KvmSegment,
    pub gdt: KvmDtable,
    pub idt: KvmDtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; 4],
}

#[repr(C)]
pub struct KvmRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

pub const VM_FILE_COUNT: usize = 4;
pub const VM_FILE_KERNEL: usize = 2;

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; VM_FILE_COUNT],
}

#[repr(C)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub type_: u32,
}

#[repr(C, packed)]
pub struct LinuxParams {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: u16,
    pub unused3: u16,
    pub orig_video_lines: u8,
    pub orig_video_isVGA: u8,
    pub orig_video_points: u16,

    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub cl_magic: u16,
    pub cl_offset: u16,
    pub lfb_linelength: u16,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: u16,
    pub vesapm_off: u16,
    pub pages: u16,
    pub vesa_attributes: u16,
    pub capabilities: u32,
    pub ext_lfb_base: u32,
    pub _reserved: [u8; 2],

    pub apm_bios_info: [u8; 20],
    pub pad2: [u8; 0x80 - 0x54],

    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],

    pub sys_description_len: u16,
    pub sys_description_table: [u8; 14],

    pub pad3: [u8; 0x1e0 - 0xb0],
    pub alt_mem_k: u32,
    pub pad4: [u8; 4],
    pub e820map_entries: u8,
    pub eddbuf_entries: u8,
    pub pad5: [u8; 0x1f1 - 0x1ea],
    pub setup_sects: u8,
    pub mount_root_rdonly: u16,
    pub sys_size: u16,

    pub swap_dev: u16,
    pub ramdisk_flags: u16,
    pub vga_mode: u16,
    pub orig_root_dev: u16,
    pub pad6: [u8; 1],
    pub aux_device_info: u8,
    pub jump_setup: u16,

    pub setup_signature: [u8; 4],
    pub header_format_version: u16,
    pub setup_S_temp0: [u8; 8],

    pub loader_type: u8,
    pub loadflags: u8,

    pub setup_S_temp1: u16,
    pub kernel_start: u32,
    pub initrd_start: u32,
    pub initrd_size: u32,
    pub setup_S_temp2: [u8; 4],
    pub setup_S_heap_end_pointer: u16,
    pub pad70: u16,
    pub cmd_line_ptr: u32,
    pub pad7: [u8; 0x2d0 - 0x22c],
    pub e820map: [E820Entry; 32],

    pub pad8: [u8; 0x600 - 0x550],

    pub eddbuf: [u8; 0x7d4 - 0x600],

    pub pad9: [u8; 0x800 - 0x7d4],
    pub commandline: [u8; 0x800],

    pub gdt_table: [u64; 4],
}

// ------------------------------------------------------------------ extern C API

extern "C" {
    static mut stderr: *mut File;

    fn fprintf(stream: *mut File, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;

    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
    fn x86_cpu_get_reg(s: *mut X86CPUState, reg: c_int) -> u32;
    fn x86_cpu_set_reg(s: *mut X86CPUState, reg: c_int, val: u32);
    fn x86_cpu_set_seg(s: *mut X86CPUState, seg: c_int, sd: *const X86CPUSeg);
    fn i440fx_map_interrupts(s: *mut I440FXState, elcr: *mut u8, pci_irqs: *const u8);
    fn pic2_set_elcr(s: *mut PIC2State, elcr: *const u8);
}

// --------------------------------------------------------------------- helpers

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> c_ulong {
    ((dir << 30) | (ty << 8) | nr | ((size as u32) << 16)) as c_ulong
}

#[inline]
fn kvm_get_sregs_request() -> c_ulong {
    ioc(2, 0xAE, 0x83, core::mem::size_of::<KvmSregs>())
}

#[inline]
fn kvm_set_sregs_request() -> c_ulong {
    ioc(1, 0xAE, 0x84, core::mem::size_of::<KvmSregs>())
}

#[inline]
fn kvm_set_regs_request() -> c_ulong {
    ioc(1, 0xAE, 0x82, core::mem::size_of::<KvmRegs>())
}

// static uint8_t *get_ram_ptr(PCMachine *s, uint64_t paddr)
unsafe fn get_ram_ptr(s: *mut PCMachine, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range((*s).mem_map, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return core::ptr::null_mut();
    }
    (*pr).phys_mem.add(paddr.wrapping_sub((*pr).addr) as usize)
}

unsafe fn kernel_too_small() -> ! {
    fprintf(stderr, cstr(b"Kernel too small\n\0"));
    exit(1);
}

// static void copy_kernel(PCMachine *s, const uint8_t *buf, int buf_len,
//                         const char *cmd_line)
unsafe fn copy_kernel(s: *mut PCMachine, buf: *const u8, buf_len: c_int, cmd_line: *const c_char) {
    if buf_len < 1024 {
        kernel_too_small();
    }
    if *buf.add(0x1fe) != 0x55 || *buf.add(0x1ff) != 0xaa {
        fprintf(stderr, cstr(b"Invalid kernel magic\n\0"));
        exit(1);
    }
    let mut setup_sects: c_int = *buf.add(0x1f1) as c_int;
    if setup_sects == 0 {
        setup_sects = 4;
    }
    let header_len: c_int = (setup_sects + 1) * 512;
    if buf_len < header_len {
        kernel_too_small();
    }
    if memcmp(
        buf.add(0x202) as *const c_void,
        cstr(b"HdrS\0") as *const c_void,
        4,
    ) != 0
    {
        fprintf(stderr, cstr(b"Kernel too old\n\0"));
        exit(1);
    }
    let load_address: u32 = 0x100000;

    let ram_ptr = get_ram_ptr(s, load_address as u64);
    let copy_len: c_int = buf_len - header_len;
    if (copy_len as u64) > (*s).ram_size.wrapping_sub(load_address as u64) {
        fprintf(stderr, cstr(b"Not enough RAM\n\0"));
        exit(1);
    }
    memcpy(
        ram_ptr as *mut c_void,
        buf.add(header_len as usize) as *const c_void,
        copy_len as usize,
    );

    let params = get_ram_ptr(s, 0x00090000);

    memset(params as *mut c_void, 0, core::mem::size_of::<LinuxParams>());

    let lp = params as *mut LinuxParams;

    let setup_hdr_start: c_int = 0x1f1;
    let setup_hdr_end: c_int = 0x202 + (*buf.add(0x201) as c_int);
    memcpy(
        (params).add(setup_hdr_start as usize) as *mut c_void,
        buf.add(setup_hdr_start as usize) as *const c_void,
        (setup_hdr_end - setup_hdr_start) as usize,
    );

    let off_commandline = core::ptr::addr_of!((*lp).commandline) as usize;
    let off_gdt_table = core::ptr::addr_of!((*lp).gdt_table) as usize;

    strcpy(
        core::ptr::addr_of_mut!((*lp).commandline) as *mut c_char,
        cmd_line,
    );

    (*lp).mount_root_rdonly = 0;
    (*lp).cmd_line_ptr = (0x00090000u64.wrapping_add(off_commandline as u64)) as u32;
    (*lp).alt_mem_k = ((*s).ram_size / 1024).wrapping_sub(1024) as u32;
    (*lp).loader_type = 0x01;

    (*lp).orig_video_lines = 0;
    (*lp).orig_video_cols = 0;

    let fb_dev = (*s).common.fb_dev;
    if !fb_dev.is_null() {
        (*lp).orig_video_isVGA = 0x23;

        (*lp).lfb_depth = 32;
        (*lp).red_size = 8;
        (*lp).red_pos = 16;
        (*lp).green_size = 8;
        (*lp).green_pos = 8;
        (*lp).blue_size = 8;
        (*lp).blue_pos = 0;
        (*lp).rsvd_size = 8;
        (*lp).rsvd_pos = 24;

        (*lp).lfb_width = (*fb_dev).width as u16;
        (*lp).lfb_height = (*fb_dev).height as u16;
        (*lp).lfb_linelength = (*fb_dev).stride as u16;
        (*lp).lfb_size = (*fb_dev).fb_size as u32;
        (*lp).lfb_base = 0xf0400000;
    }

    let gdt_ptr = core::ptr::addr_of_mut!((*lp).gdt_table) as *mut u64;
    gdt_ptr.add(2).write(0x00cf9b000000ffffu64);
    gdt_ptr.add(3).write(0x00cf93000000ffffu64);

    if (*s).kvm_enabled != 0 {
        let mut sregs: KvmSregs = core::mem::zeroed();

        if ioctl(
            (*s).vcpu_fd,
            kvm_get_sregs_request(),
            &mut sregs as *mut KvmSregs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_GET_SREGS\0"));
            exit(1);
        }

        sregs.cr0 |= 1 << 0;
        sregs.gdt.base = 0x00090000u64.wrapping_add(off_gdt_table as u64);
        sregs.gdt.limit = (core::mem::size_of::<[u64; 4]>() - 1) as u16;

        let mut seg: KvmSegment = core::mem::zeroed();
        seg.limit = 0xffffffff;
        seg.present = 1;
        seg.db = 1;
        seg.s = 1;
        seg.g = 1;

        seg.type_ = 0xb;
        seg.selector = 2 << 3;
        sregs.cs = seg;

        seg.type_ = 0x3;
        seg.selector = 3 << 3;
        sregs.ds = seg;
        sregs.es = seg;
        sregs.ss = seg;
        sregs.fs = seg;
        sregs.gs = seg;

        if ioctl(
            (*s).vcpu_fd,
            kvm_set_sregs_request(),
            &mut sregs as *mut KvmSregs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_SET_SREGS\0"));
            exit(1);
        }

        let mut regs: KvmRegs = core::mem::zeroed();
        regs.rip = load_address as u64;
        regs.rsi = 0x00090000;
        regs.rflags = 0x2;
        if ioctl(
            (*s).vcpu_fd,
            kvm_set_regs_request(),
            &mut regs as *mut KvmRegs as *mut c_void,
        ) < 0
        {
            perror(cstr(b"KVM_SET_REGS\0"));
            exit(1);
        }
    } else {
        let mut sd: X86CPUSeg = core::mem::zeroed();
        let val = x86_cpu_get_reg((*s).cpu_state, 9);
        x86_cpu_set_reg((*s).cpu_state, 9, val | (1 << 0));

        sd.base = (0x00090000u64.wrapping_add(off_gdt_table as u64)) as u32;
        sd.limit = (core::mem::size_of::<[u64; 4]>() - 1) as u32;
        x86_cpu_set_seg((*s).cpu_state, 8, &sd);
        sd.sel = 2 << 3;
        sd.base = 0;
        sd.limit = 0xffffffff;
        sd.flags = 0xc09b;
        x86_cpu_set_seg((*s).cpu_state, 1, &sd);
        sd.sel = 3 << 3;
        sd.flags = 0xc093;
        let mut i: c_int = 0;
        while i < 6 {
            if i != 1 {
                x86_cpu_set_seg((*s).cpu_state, i, &sd);
            }
            i += 1;
        }

        x86_cpu_set_reg((*s).cpu_state, 8, load_address);
        x86_cpu_set_reg((*s).cpu_state, 6, 0x00090000);
    }

    {
        let mut elcr: [u8; 2] = [0, 0];
        let pci_irqs: [u8; 4] = [9, 10, 11, 12];

        i440fx_map_interrupts((*s).i440fx_state, elcr.as_mut_ptr(), pci_irqs.as_ptr());

        if !(*s).pic_state.is_null() {
            pic2_set_elcr((*s).pic_state, elcr.as_ptr());
        }
    }
}

// ------------------------------------------------------------------ entry point

#[no_mangle]
pub extern "C" fn pc_machine_init__part4229(
    s: *mut PCMachine,
    p: *const VirtMachineParams,
) {
    unsafe {
        let buf = (*p).files[VM_FILE_KERNEL].buf;
        if !buf.is_null() {
            let len = (*p).files[VM_FILE_KERNEL].len;
            let mut cmdline = (*p).cmdline;
            if cmdline.is_null() {
                cmdline = cstr(b"\0") as *mut c_char;
            }
            copy_kernel(s, buf, len, cmdline);
        }
    }
}
}

// ── `return (VirtMachine *)s;` · x86_machine.c:2161-2161 · chunk 4230 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (VirtMachine *)s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4230 {
// Port of the final `return (VirtMachine *)s;` of pc_machine_init.
// The pointed-to layouts are never accessed, so opaque mirrors are enough.

#[repr(C)]
pub struct PCMachine {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn pc_machine_init__part4230(s: *mut PCMachine) -> *mut VirtMachine {
    // C cast of the pointer value: identical bits, no dereference.
    s as *mut VirtMachine
}
}

// ── `pc_machine_end` · x86_machine.c:2164-2174 · chunk 4231 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_machine_end(VirtMachine *s1)
//     {
//         PCMachine *s = (PCMachine *)s1;
//     
//         if (s->cpu_state) {
//             x86_cpu_end(s->cpu_state);
//         }
//         phys_mem_map_end(s->mem_map);
//         phys_mem_map_end(s->port_map);
//         free(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4231 {
// Port of the C function:
//   static void pc_machine_end(VirtMachine *s1)
//   {
//       PCMachine *s = (PCMachine *)s1;
//       if (s->cpu_state) {
//           x86_cpu_end(s->cpu_state);
//       }
//       phys_mem_map_end(s->mem_map);
//       phys_mem_map_end(s->port_map);
//       free(s);
//   }
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

// Opaque C types.
pub enum VirtMachineClass {}
pub enum PhysMemoryMap {}
pub enum X86CPUState {}

// Mirror of the C `VirtMachine` header (only the prefix needed is declared;
// all members are pointers, so offsets are exact).
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct EthernетDevicePlaceholder {
    _private: [u8; 0],
}

// Mirror of the C `PCMachine` header, up to and including `cpu_state`.
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut X86CPUState,
}

extern "C" {
    fn x86_cpu_end(s: *mut X86CPUState);
    fn phys_mem_map_end(s: *mut PhysMemoryMap);
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_end(s1: *mut VirtMachine) {
    let s = s1 as *mut PCMachine;

    if s.is_null() {
        // C would also dereference NULL here; keep the same undefined behaviour
        // by simply proceeding would be unsound in Rust, so use a raw read path.
        return;
    }

    let cpu_state = core::ptr::addr_of!((*s).cpu_state).read();
    if !cpu_state.is_null() {
        x86_cpu_end(cpu_state);
    }

    let mem_map = core::ptr::addr_of!((*s).mem_map).read();
    phys_mem_map_end(mem_map);

    let port_map = core::ptr::addr_of!((*s).port_map).read();
    phys_mem_map_end(port_map);

    free(s as *mut c_void);
}

// Silence unused-import warning for c_int (kept for documentation parity).
#[allow(dead_code)]
const _UNUSED: c_int = 0;
}

// ── `if (s->cpu_state)` · x86_machine.c:2168-2170 · chunk 4232 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->cpu_state) {
//             x86_cpu_end(s->cpu_state);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4232 {
// Port of the CPU-teardown branch of pc_machine_end (x86_machine.c).
// The null check on s->cpu_state is kept here, so the caller just passes the pointer.

use core::ffi::c_void;

extern "C" {
    // void x86_cpu_end(X86CPUState *s);
    fn x86_cpu_end(s: *mut c_void);
}

#[no_mangle]
pub extern "C" fn pc_machine_end__part4232(cpu_state: *mut c_void) {
    if !cpu_state.is_null() {
        unsafe {
            x86_cpu_end(cpu_state);
        }
    }
}
}

// ── `phys_mem_map_end(s->mem_map); phys_mem_map_end(s->port_map); free(s);` · x86_machine.c:2171-2173 · chunk 4233 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     phys_mem_map_end(s->mem_map);
//         phys_mem_map_end(s->port_map);
//         free(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4233 {
// Port of the tail of pc_machine_end:
//   phys_mem_map_end(s->mem_map);
//   phys_mem_map_end(s->port_map);
//   free(s);
use core::ffi::c_void;

// Prefix of the C `VirtMachine` struct (only used through a pointer).
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Prefix of the C `PCMachine` struct up to `port_map`.
#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
}

extern "C" {
    // void phys_mem_map_end(PhysMemoryMap *s);
    fn phys_mem_map_end(s: *mut c_void);
    // void free(void *ptr);
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_end__part4233(s: *mut PCMachine) {
    let s = s as *mut u8 as *mut PCMachine;
    if s.is_null() {
        return;
    }
    let mem_map = (*s).mem_map;
    phys_mem_map_end(mem_map);
    let port_map = (*s).port_map;
    phys_mem_map_end(port_map);
    free(s as *mut c_void);
}
}

// ── `pc_vm_send_key_event` · x86_machine.c:2176-2184 · chunk 4234 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_vm_send_key_event(VirtMachine *s1, BOOL is_down, uint16_t key_code)
//     {
//         PCMachine *s = (PCMachine *)s1;
//         if (s->keyboard_dev) {
//             virtio_input_send_key_event(s->keyboard_dev, is_down, key_code);
//         } else if (s->ps2_kbd) {
//             ps2_put_keycode(s->ps2_kbd, is_down, key_code);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4234 {
// Rewrite of pc_vm_send_key_event from x86_machine.c
use core::ffi::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
pub type BOOL = c_int;

// Opaque handle types referenced through pointers only.
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct X86CPUState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PIC2State {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PITState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct I440FXState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct CMOSState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct SerialState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct VIRTIODevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct KBDState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PS2MouseState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct VMMouseState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct PS2KbdState {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FBDevice {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct VirtMachineClass {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct kvm_run {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

pub type SetIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,
    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut kvm_run,
}

extern "C" {
    fn virtio_input_send_key_event(s: *mut VIRTIODevice, is_down: BOOL, key_code: u16) -> c_int;
    fn ps2_put_keycode(s: *mut PS2KbdState, is_down: BOOL, keycode: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pc_vm_send_key_event(s1: *mut VirtMachine, is_down: BOOL, key_code: u16) {
    let s = s1 as *mut PCMachine;
    if s.is_null() {
        return;
    }
    let keyboard_dev = (*s).keyboard_dev;
    if !keyboard_dev.is_null() {
        virtio_input_send_key_event(keyboard_dev, is_down, key_code);
    } else {
        let ps2_kbd = (*s).ps2_kbd;
        if !ps2_kbd.is_null() {
            ps2_put_keycode(ps2_kbd, is_down, key_code as c_int);
        }
    }
}
}

// ── `if (s->keyboard_dev) … else` · x86_machine.c:2179-2183 · chunk 4235 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->keyboard_dev) {
//             virtio_input_send_key_event(s->keyboard_dev, is_down, key_code);
//         } else if (s->ps2_kbd) {
//             ps2_put_keycode(s->ps2_kbd, is_down, key_code);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4235 {
// Ported piece of pc_vm_send_key_event: route the key event to the virtio
// keyboard if present, otherwise to the PS/2 keyboard if present.
use core::ffi::{c_int, c_void};

extern "C" {
    // int virtio_input_send_key_event(VIRTIODevice *s, BOOL is_down, uint16_t key_code);
    fn virtio_input_send_key_event(s: *mut c_void, is_down: c_int, key_code: u16) -> c_int;
    // void ps2_put_keycode(PS2KbdState *s, BOOL is_down, int keycode);
    fn ps2_put_keycode(s: *mut c_void, is_down: c_int, keycode: c_int);
}

#[no_mangle]
pub extern "C" fn pc_vm_send_key_event__part4235(
    keyboard_dev: *mut c_void,
    ps2_kbd: *mut c_void,
    is_down: c_int,
    key_code: u16,
) {
    unsafe {
        if !keyboard_dev.is_null() {
            virtio_input_send_key_event(keyboard_dev, is_down, key_code);
        } else if !ps2_kbd.is_null() {
            ps2_put_keycode(ps2_kbd, is_down, key_code as c_int);
        }
    }
}
}

// ── `pc_vm_mouse_is_absolute` · x86_machine.c:2186-2196 · chunk 4236 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static BOOL pc_vm_mouse_is_absolute(VirtMachine *s1)
//     {
//         PCMachine *s = (PCMachine *)s1;
//         if (s->mouse_dev) {
//             return TRUE;
//         } else if (s->vm_mouse) {
//             return vmmouse_is_absolute(s->vm_mouse);
//         } else {
//             return FALSE;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4236 {
use core::ffi::{c_int, c_void};

type BOOL = c_int;
const TRUE: BOOL = 1;
const FALSE: BOOL = 0;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,

    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,

    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

extern "C" {
    fn vmmouse_is_absolute(s: *mut c_void) -> BOOL;
}

#[no_mangle]
pub unsafe extern "C" fn pc_vm_mouse_is_absolute(s1: *mut VirtMachine) -> BOOL {
    let s = s1 as *mut PCMachine;
    if (*s).mouse_dev != core::ptr::null_mut() {
        TRUE
    } else if (*s).vm_mouse != core::ptr::null_mut() {
        vmmouse_is_absolute((*s).vm_mouse)
    } else {
        FALSE
    }
}
}

// ── `if (s->mouse_dev) … else` · x86_machine.c:2189-2195 · chunk 4237 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->mouse_dev) {
//             return TRUE;
//         } else if (s->vm_mouse) {
//             return vmmouse_is_absolute(s->vm_mouse);
//         } else {
//             return FALSE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4237 {
// Ported piece of pc_vm_mouse_is_absolute: decides whether the VM mouse
// reports absolute coordinates.
use core::ffi::{c_int, c_void};

extern "C" {
    // BOOL vmmouse_is_absolute(VMMouseState *s);
    fn vmmouse_is_absolute(s: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn pc_vm_mouse_is_absolute__part4237(
    mouse_dev: *mut c_void,
    vm_mouse: *mut c_void,
) -> c_int {
    if !mouse_dev.is_null() {
        1
    } else if !vm_mouse.is_null() {
        unsafe { vmmouse_is_absolute(vm_mouse) }
    } else {
        0
    }
}
}

// ── `pc_vm_send_mouse_event` · x86_machine.c:2198-2207 · chunk 4238 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_vm_send_mouse_event(VirtMachine *s1, int dx, int dy, int dz,
//                                        unsigned int buttons)
//     {
//         PCMachine *s = (PCMachine *)s1;
//         if (s->mouse_dev) {
//             virtio_input_send_mouse_event(s->mouse_dev, dx, dy, dz, buttons);
//         } else if (s->vm_mouse) {
//             vmmouse_send_mouse_event(s->vm_mouse, dx, dy, dz, buttons);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4238 {
use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,

    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,

    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

extern "C" {
    fn virtio_input_send_mouse_event(
        s: *mut c_void,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons: c_uint,
    ) -> c_int;
    fn vmmouse_send_mouse_event(s: *mut c_void, x: c_int, y: c_int, dz: c_int, buttons: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pc_vm_send_mouse_event(
    s1: *mut VirtMachine,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    let s = s1 as *mut PCMachine;
    if (*s).mouse_dev != core::ptr::null_mut() {
        virtio_input_send_mouse_event((*s).mouse_dev, dx, dy, dz, buttons);
    } else if (*s).vm_mouse != core::ptr::null_mut() {
        vmmouse_send_mouse_event((*s).vm_mouse, dx, dy, dz, buttons as c_int);
    }
}
}

// ── `if (s->mouse_dev) … else` · x86_machine.c:2202-2206 · chunk 4239 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (s->mouse_dev) {
//             virtio_input_send_mouse_event(s->mouse_dev, dx, dy, dz, buttons);
//         } else if (s->vm_mouse) {
//             vmmouse_send_mouse_event(s->vm_mouse, dx, dy, dz, buttons);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4239 {
// Ported piece: forwarding of a mouse event to either the virtio input
// device or the vmware mouse device.
use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    fn virtio_input_send_mouse_event(
        s: *mut c_void,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons: c_uint,
    ) -> c_int;
    fn vmmouse_send_mouse_event(s: *mut c_void, x: c_int, y: c_int, dz: c_int, buttons: c_int);
}

#[no_mangle]
pub extern "C" fn pc_vm_send_mouse_event__part4239(
    mouse_dev: *mut c_void,
    vm_mouse: *mut c_void,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    unsafe {
        if !mouse_dev.is_null() {
            virtio_input_send_mouse_event(mouse_dev, dx, dy, dz, buttons);
        } else if !vm_mouse.is_null() {
            // implicit unsigned -> int conversion at the C call site
            vmmouse_send_mouse_event(vm_mouse, dx, dy, dz, buttons as c_int);
        }
    }
}
}

// ── `copy_kernel` · x86_machine.c:2343-2523 · chunk 4242 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void copy_kernel(PCMachine *s, const uint8_t *buf, int buf_len,
//                             const char *cmd_line)
//     {
//         uint8_t *ram_ptr;
//         int setup_sects, header_len, copy_len, setup_hdr_start, setup_hdr_end;
//         uint32_t load_address;
//         struct linux_params *params;
//         FBDevice *fb_dev;
//     
//         if (buf_len < 1024) {
//         too_small:
//             fprintf(
//                    stderr
//                          , "Kernel too small\n");
//             exit(1);
//         }
//         if (buf[0x1fe] != 0x55 || buf[0x1ff] != 0xaa) {
//             fprintf(
//                    stderr
//                          , "Invalid kernel magic\n");
//             exit(1);
//         }
//         setup_sects = buf[0x1f1];
//         if (setup_sects == 0)
//             setup_sects = 4;
//         header_len = (setup_sects + 1) * 512;
//         if (buf_len < header_len)
//             goto too_small;
//         if (memcmp(buf + 0x202, "HdrS", 4) != 0) {
//             fprintf(
//                    stderr
//                          , "Kernel too old\n");
//             exit(1);
//         }
//         load_address = 0x100000;
//     
//         ram_ptr = get_ram_ptr(s, load_address);
//         copy_len = buf_len - header_len;
//         if (copy_len > (s->ram_size - load_address)) {
//             fprintf(
//                    stderr
//                          , "Not enough RAM\n");
//             exit(1);
//         }
//         memcpy(ram_ptr, buf + header_len, copy_len);
//     
//         params = (void *)get_ram_ptr(s, 0x00090000);
//     
//         memset(params, 0, sizeof(struct linux_params));
//     
//     
//         setup_hdr_start = 0x1f1;
//         setup_hdr_end = 0x202 + buf[0x201];
//         memcpy((uint8_t *)params + setup_hdr_start, buf + setup_hdr_start,
//                setup_hdr_end - setup_hdr_start);
//     
//         strcpy((char *)params->commandline, cmd_line);
//     
//         params->mount_root_rdonly = 0;
//         params->cmd_line_ptr = 0x00090000 +
//             ((size_t) &((struct linux_params *)0)->commandline);
//         params->alt_mem_k = (s->ram_size / 1024) - 1024;
//         params->loader_type = 0x01;
//     
//     
//     
//     
//     
//     
//         params->orig_video_lines = 0;
//         params->orig_video_cols = 0;
//     
//         fb_dev = s->common.fb_dev;
//         if (fb_dev) {
//     
//             params->orig_video_isVGA = 0x23;
//     
//             params->lfb_depth = 32;
//             params->red_size = 8;
//             params->red_pos = 16;
//             params->green_size = 8;
//             params->green_pos = 8;
//             params->blue_size = 8;
//             params->blue_pos = 0;
//             params->rsvd_size = 8;
//             params->rsvd_pos = 24;
//     
//             params->lfb_width = fb_dev->width;
//             params->lfb_height = fb_dev->height;
//             params->lfb_linelength = fb_dev->stride;
//             params->lfb_size = fb_dev->fb_size;
//             params->lfb_base = 0xf0400000;
//         }
//     
//         params->gdt_table[2] = 0x00cf9b000000ffffLL;
//         params->gdt_table[3] = 0x00cf93000000ffffLL;
//     
//     
//         if (s->kvm_enabled) {
//             struct kvm_sregs sregs;
//             struct kvm_segment seg;
//             struct kvm_regs regs;
//     
//     
//     
//             if (ioctl(s->vcpu_fd, 
//                                  (((2U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x83)) << 0) | ((((sizeof(struct kvm_sregs)))) << ((0 +8)+8)))
//                                               , &sregs) < 0) {
//                 perror("KVM_GET_SREGS");
//                 exit(1);
//             }
//     
//             sregs.cr0 |= (1 << 0);
//             sregs.gdt.base = 0x00090000 +
//                 ((size_t) &((struct linux_params *)0)->gdt_table);
//             sregs.gdt.limit = sizeof(params->gdt_table) - 1;
//     
//             memset(&seg, 0, sizeof(seg));
//             seg.limit = 0xffffffff;
//             seg.present = 1;
//             seg.db = 1;
//             seg.s = 1;
//             seg.g = 1;
//     
//             seg.type = 0xb;
//             seg.selector = 2 << 3;
//             sregs.cs = seg;
//     
//             seg.type = 0x3;
//             seg.selector = 3 << 3;
//             sregs.ds = seg;
//             sregs.es = seg;
//             sregs.ss = seg;
//             sregs.fs = seg;
//             sregs.gs = seg;
//     
//             if (ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x84)) << 0) | ((((sizeof(struct kvm_sregs)))) << ((0 +8)+8)))
//                                               , &sregs) < 0) {
//                 perror("KVM_SET_SREGS");
//                 exit(1);
//             }
//     
//             memset(&regs, 0, sizeof(regs));
//             regs.rip = load_address;
//             regs.rsi = 0x00090000;
//             regs.rflags = 0x2;
//             if (ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x82)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                              , &regs) < 0) {
//                 perror("KVM_SET_REGS");
//                 exit(1);
//             }
//         } else
//     
//         {
//             int i;
//             X86CPUSeg sd;
//             uint32_t val;
//             val = x86_cpu_get_reg(s->cpu_state, 9);
//             x86_cpu_set_reg(s->cpu_state, 9, val | (1 << 0));
//     
//             sd.base = 0x00090000 +
//                 ((size_t) &((struct linux_params *)0)->gdt_table);
//             sd.limit = sizeof(params->gdt_table) - 1;
//             x86_cpu_set_seg(s->cpu_state, 8, &sd);
//             sd.sel = 2 << 3;
//             sd.base = 0;
//             sd.limit = 0xffffffff;
//             sd.flags = 0xc09b;
//             x86_cpu_set_seg(s->cpu_state, 1, &sd);
//             sd.sel = 3 << 3;
//             sd.flags = 0xc093;
//             for(i = 0; i < 6; i++) {
//                 if (i != 1) {
//                     x86_cpu_set_seg(s->cpu_state, i, &sd);
//                 }
//             }
//     
//             x86_cpu_set_reg(s->cpu_state, 8, load_address);
//             x86_cpu_set_reg(s->cpu_state, 6, 0x00090000);
//         }
//     
//     
//         {
//             uint8_t elcr[2];
//             static const uint8_t pci_irqs[4] = { 9, 10, 11, 12 };
//     
//             i440fx_map_interrupts(s->i440fx_state, elcr, pci_irqs);
//     
//             if (s->pic_state) {
//                 pic2_set_elcr(s->pic_state, elcr);
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4242 {
// Port of the static C function copy_kernel() from x86_machine.c.
// The kernel image is loaded at 1MB, a linux_params (boot params) structure is
// filled in at KERNEL_PARAMS_ADDR (0x00090000) and the CPU (or KVM vcpu) is put
// into flat protected mode pointing at the kernel entry point.
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* ------------------------------------------------------------------ */
/* opaque C types (only ever used through pointers)                    */
/* ------------------------------------------------------------------ */

#[repr(C)]
pub struct PhysMemoryMap {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct X86CPUState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct PIC2State {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct PITState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct I440FXState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct CMOSState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct SerialState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VIRTIODevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct KBDState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct PS2MouseState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct PS2KbdState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct KvmRun {
    pub _opaque: [u8; 0],
}

/* ------------------------------------------------------------------ */
/* mirrored C structures                                               */
/* ------------------------------------------------------------------ */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, *mut c_void)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct X86CPUSeg {
    pub sel: u16,
    pub flags: u16,
    pub base: u32,
    pub limit: u32,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int, /* BOOL */
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,

    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,

    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,

    pub kvm_enabled: c_int, /* BOOL */
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

/* struct e820entry (linux/include/asm-i386/e820.h) as used inside
   struct linux_params; not itself packed. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub r#type: u32,
}

/* Same field sequence and nesting as
   'struct __attribute__ ((packed)) linux_params'.  Mirroring the nested
   anonymous e820 struct as a separate #[repr(C)] type reproduces exactly the
   same member offsets/sizes the C compiler computes. */
#[repr(C, packed)]
pub struct LinuxParams {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: u16,
    pub unused3: u16,
    pub orig_video_lines: u8,
    pub orig_video_isVGA: u8,
    pub orig_video_points: u16,

    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub cl_magic: u16,
    pub cl_offset: u16,
    pub lfb_linelength: u16,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: u16,
    pub vesapm_off: u16,
    pub pages: u16,
    pub vesa_attributes: u16,
    pub capabilities: u32,
    pub ext_lfb_base: u32,
    pub _reserved: [u8; 2],

    pub apm_bios_info: [u8; 20],
    pub pad2: [u8; 0x80 - 0x54],

    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],

    pub sys_description_len: u16,
    pub sys_description_table: [u8; 14],

    pub pad3: [u8; 0x1e0 - 0xb0],
    pub alt_mem_k: u32,
    pub pad4: [u8; 4],
    pub e820map_entries: u8,
    pub eddbuf_entries: u8,
    pub pad5: [u8; 0x1f1 - 0x1ea],
    pub setup_sects: u8,
    pub mount_root_rdonly: u16,
    pub sys_size: u16,
    pub swap_dev: u16,
    pub ramdisk_flags: u16,
    pub vga_mode: u16,
    pub orig_root_dev: u16,
    pub pad6: [u8; 1],
    pub aux_device_info: u8,
    pub jump_setup: u16,

    pub setup_signature: [u8; 4],
    pub header_format_version: u16,
    pub setup_S_temp0: [u8; 8],

    pub loader_type: u8,
    pub loadflags: u8,

    pub setup_S_temp1: u16,
    pub kernel_start: u32,
    pub initrd_start: u32,
    pub initrd_size: u32,
    pub setup_S_temp2: [u8; 4],
    pub setup_S_heap_end_pointer: u16,
    pub pad70: u16,
    pub cmd_line_ptr: u32,
    pub pad7: [u8; 0x2d0 - 0x22c],

    pub e820map: [E820Entry; 32],

    pub pad8: [u8; 0x600 - 0x550],

    pub eddbuf: [u8; 0x7d4 - 0x600],

    pub pad9: [u8; 0x800 - 0x7d4],
    pub commandline: [u8; 0x800],

    pub gdt_table: [u64; 4],
}

/* struct kvm_segment */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmSegment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub r#type: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    pub unusable: u8,
    pub padding: u8,
}

/* struct kvm_dtable */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmDtable {
    pub base: u64,
    pub limit: u16,
    pub padding: [u16; 3],
}

/* struct kvm_sregs */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmSregs {
    pub cs: KvmSegment,
    pub ds: KvmSegment,
    pub es: KvmSegment,
    pub fs: KvmSegment,
    pub gs: KvmSegment,
    pub ss: KvmSegment,
    pub tr: KvmSegment,
    pub ldt: KvmSegment,
    pub gdt: KvmDtable,
    pub idt: KvmDtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; (256 + 63) / 64],
}

/* struct kvm_regs */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KvmRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

/* ------------------------------------------------------------------ */
/* libc / emulator entry points                                        */
/* ------------------------------------------------------------------ */

extern "C" {
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;

    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
    fn i440fx_map_interrupts(s: *mut I440FXState, elcr: *mut u8, pci_irqs: *const u8);
    fn pic2_set_elcr(s: *mut PIC2State, elcr: *const u8);
    fn x86_cpu_get_reg(s: *mut X86CPUState, reg: c_int) -> u32;
    fn x86_cpu_set_reg(s: *mut X86CPUState, reg: c_int, val: u32);
    fn x86_cpu_set_seg(s: *mut X86CPUState, seg: c_int, sd: *const X86CPUSeg);
}

/* ------------------------------------------------------------------ */
/* KVM ioctl numbers (linux/ioctl.h _IOC encoding)                     */
/* ------------------------------------------------------------------ */

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> c_ulong {
    ((dir << 30) | ((ty & 0xff) << 8) | (nr & 0xff) | ((size as u32) << 16)) as c_ulong
}

const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;
const KVMIO: u32 = 0xAE;

const KVM_GET_SREGS: c_ulong = ioc(IOC_READ, KVMIO, 0x83, size_of::<KvmSregs>());
const KVM_SET_SREGS: c_ulong = ioc(IOC_WRITE, KVMIO, 0x84, size_of::<KvmSregs>());
const KVM_SET_REGS: c_ulong = ioc(IOC_WRITE, KVMIO, 0x82, size_of::<KvmRegs>());

/* ------------------------------------------------------------------ */
/* helpers                                                             */
/* ------------------------------------------------------------------ */

const KERNEL_PARAMS_ADDR: u32 = 0x00090000;
const FRAMEBUFFER_BASE_ADDR: u32 = 0xf0400000;

#[inline]
unsafe fn err_stream() -> *mut c_void {
    ptr::addr_of_mut!(stderr).read()
}

#[inline]
unsafe fn die(msg: &[u8]) -> ! {
    fprintf(err_stream(), msg.as_ptr() as *const c_char);
    exit(1)
}

#[inline]
unsafe fn die_perror(msg: &[u8]) -> ! {
    perror(msg.as_ptr() as *const c_char);
    exit(1)
}

/* ported from the static C helper get_ram_ptr() */
unsafe fn get_ram_ptr(s: *mut PCMachine, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range((*s).mem_map, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return ptr::null_mut();
    }
    (*pr)
        .phys_mem
        .add(paddr.wrapping_sub((*pr).addr) as usize)
}

/* ------------------------------------------------------------------ */
/* copy_kernel                                                         */
/* ------------------------------------------------------------------ */

#[no_mangle]
pub unsafe extern "C" fn copy_kernel(
    s: *mut PCMachine,
    buf: *const u8,
    buf_len: c_int,
    cmd_line: *const c_char,
) {
    if buf_len < 1024 {
        die(b"Kernel too small\n\0");
    }
    if *buf.add(0x1fe) != 0x55 || *buf.add(0x1ff) != 0xaa {
        die(b"Invalid kernel magic\n\0");
    }
    let mut setup_sects: c_int = *buf.add(0x1f1) as c_int;
    if setup_sects == 0 {
        setup_sects = 4;
    }
    let header_len: c_int = (setup_sects + 1) * 512;
    if buf_len < header_len {
        die(b"Kernel too small\n\0");
    }
    if memcmp(
        buf.add(0x202) as *const c_void,
        b"HdrS".as_ptr() as *const c_void,
        4,
    ) != 0
    {
        die(b"Kernel too old\n\0");
    }
    /* we don't support older protocols */
    let load_address: u32 = 0x100000;

    let ram_ptr = get_ram_ptr(s, load_address as u64);
    let copy_len: c_int = buf_len - header_len;
    if (copy_len as u64) > (*s).ram_size.wrapping_sub(load_address as u64) {
        die(b"Not enough RAM\n\0");
    }
    memcpy(
        ram_ptr as *mut c_void,
        buf.add(header_len as usize) as *const c_void,
        copy_len as usize,
    );

    let params = get_ram_ptr(s, KERNEL_PARAMS_ADDR as u64) as *mut LinuxParams;

    memset(params as *mut c_void, 0, size_of::<LinuxParams>());

    /* copy the setup header */
    let setup_hdr_start: c_int = 0x1f1;
    let setup_hdr_end: c_int = 0x202 + (*buf.add(0x201) as c_int);
    memcpy(
        (params as *mut u8).add(setup_hdr_start as usize) as *mut c_void,
        buf.add(setup_hdr_start as usize) as *const c_void,
        (setup_hdr_end - setup_hdr_start) as usize,
    );

    let cmdline_field = ptr::addr_of_mut!((*params).commandline);
    strcpy(cmdline_field as *mut c_char, cmd_line);

    let params_addr = params as usize;
    let cmdline_off = cmdline_field as usize - params_addr;

    (*params).mount_root_rdonly = 0;
    (*params).cmd_line_ptr = KERNEL_PARAMS_ADDR.wrapping_add(cmdline_off as u32);
    (*params).alt_mem_k = ((*s).ram_size / 1024).wrapping_sub(1024) as u32;
    (*params).loader_type = 0x01;

    (*params).orig_video_lines = 0;
    (*params).orig_video_cols = 0;

    let fb_dev = (*s).common.fb_dev;
    if !fb_dev.is_null() {
        (*params).orig_video_isVGA = 0x23; /* VIDEO_TYPE_VLFB */

        (*params).lfb_depth = 32;
        (*params).red_size = 8;
        (*params).red_pos = 16;
        (*params).green_size = 8;
        (*params).green_pos = 8;
        (*params).blue_size = 8;
        (*params).blue_pos = 0;
        (*params).rsvd_size = 8;
        (*params).rsvd_pos = 24;

        (*params).lfb_width = (*fb_dev).width as u16;
        (*params).lfb_height = (*fb_dev).height as u16;
        (*params).lfb_linelength = (*fb_dev).stride as u16;
        (*params).lfb_size = (*fb_dev).fb_size as u32;
        (*params).lfb_base = FRAMEBUFFER_BASE_ADDR;
    }

    let gdt_field = ptr::addr_of_mut!((*params).gdt_table) as *mut u64;
    let gdt_off = gdt_field as usize - params_addr;

    gdt_field.add(2).write(0x00cf9b000000ffffu64); /* CS */
    gdt_field.add(3).write(0x00cf93000000ffffu64); /* DS */

    if (*s).kvm_enabled != 0 {
        let mut sregs: KvmSregs = core::mem::zeroed();

        /* init flat protected mode */

        if ioctl(
            (*s).vcpu_fd,
            KVM_GET_SREGS,
            core::ptr::addr_of_mut!(sregs),
        ) < 0
        {
            die_perror(b"KVM_GET_SREGS\0");
        }

        sregs.cr0 |= 1 << 0; /* CR0_PE */
        sregs.gdt.base = (KERNEL_PARAMS_ADDR as u64).wrapping_add(gdt_off as u64);
        sregs.gdt.limit = (size_of::<[u64; 4]>() - 1) as u16;

        let mut seg: KvmSegment = core::mem::zeroed();
        seg.limit = 0xffffffff;
        seg.present = 1;
        seg.db = 1;
        seg.s = 1; /* code/data */
        seg.g = 1; /* 4KB granularity */

        seg.r#type = 0xb; /* code */
        seg.selector = 2 << 3;
        sregs.cs = seg;

        seg.r#type = 0x3; /* data */
        seg.selector = 3 << 3;
        sregs.ds = seg;
        sregs.es = seg;
        sregs.ss = seg;
        sregs.fs = seg;
        sregs.gs = seg;

        if ioctl(
            (*s).vcpu_fd,
            KVM_SET_SREGS,
            core::ptr::addr_of_mut!(sregs),
        ) < 0
        {
            die_perror(b"KVM_SET_SREGS\0");
        }

        let mut regs: KvmRegs = core::mem::zeroed();
        regs.rip = load_address as u64;
        regs.rsi = KERNEL_PARAMS_ADDR as u64;
        regs.rflags = 0x2;
        if ioctl(
            (*s).vcpu_fd,
            KVM_SET_REGS,
            core::ptr::addr_of_mut!(regs),
        ) < 0
        {
            die_perror(b"KVM_SET_REGS\0");
        }
    } else {
        let val: u32 = x86_cpu_get_reg((*s).cpu_state, 9); /* X86_CPU_REG_CR0 */
        x86_cpu_set_reg((*s).cpu_state, 9, val | (1 << 0));

        /* The C original leaves sel/flags uninitialised for the GDT update;
           zeroing them is behaviourally equivalent (both fields are fully
           initialised before the segment registers are programmed). */
        let mut sd: X86CPUSeg = core::mem::zeroed();
        sd.base = KERNEL_PARAMS_ADDR.wrapping_add(gdt_off as u32);
        sd.limit = (size_of::<[u64; 4]>() - 1) as u32;
        x86_cpu_set_seg((*s).cpu_state, 8 /* GDT */, core::ptr::addr_of!(sd));
        sd.sel = 2 << 3;
        sd.base = 0;
        sd.limit = 0xffffffff;
        sd.flags = 0xc09b;
        x86_cpu_set_seg((*s).cpu_state, 1 /* CS */, core::ptr::addr_of!(sd));
        sd.sel = 3 << 3;
        sd.flags = 0xc093;
        let mut i: c_int = 0;
        while i < 6 {
            if i != 1 {
                x86_cpu_set_seg((*s).cpu_state, i, core::ptr::addr_of!(sd));
            }
            i += 1;
        }

        x86_cpu_set_reg((*s).cpu_state, 8 /* EIP */, load_address);
        x86_cpu_set_reg((*s).cpu_state, 6 /* esi */, KERNEL_PARAMS_ADDR);
    }

    /* map PCI interrupts (no BIOS, so we must do it) */
    {
        let mut elcr: [u8; 2] = [0, 0];
        let pci_irqs: [u8; 4] = [9, 10, 11, 12];

        i440fx_map_interrupts((*s).i440fx_state, elcr.as_mut_ptr(), pci_irqs.as_ptr());
        /* XXX: KVM support */
        let pic = (*s).pic_state;
        if !pic.is_null() {
            pic2_set_elcr(pic, elcr.as_ptr());
        }
    }
}
}

// ── `if (buf_len < 1024)` · x86_machine.c:2352-2356 · chunk 4243 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buf_len < 1024) {
//         too_small:
//             fprintf(
//                    stderr
//                          , "Kernel too small\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4243 {
// Port of the "Kernel too small" error path of copy_kernel().
// The C side keeps the `too_small:` label so the later `goto too_small;`
// still lands on the call to this function.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn copy_kernel__part4243() {
    unsafe {
        fprintf(
            stderr,
            b"Kernel too small\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}
}

// ── `if (buf[0x1fe] != 0x55 || buf[0x1ff] != 0xaa)` · x86_machine.c:2357-2360 · chunk 4244 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buf[0x1fe] != 0x55 || buf[0x1ff] != 0xaa) {
//             fprintf(
//                    stderr
//                          , "Invalid kernel magic\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4244 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn copy_kernel__part4244(buf: *const u8) {
    unsafe {
        if *buf.add(0x1fe) != 0x55 || *buf.add(0x1ff) != 0xaa {
            fprintf(
                stderr,
                b"Invalid kernel magic\n\0".as_ptr() as *const c_char,
            );
            exit(1);
        }
    }
}
}

// ── `setup_sects = buf[0x1f1];` · x86_machine.c:2361-2361 · chunk 4245 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     setup_sects = buf[0x1f1];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4245 {
// Extract the setup_sects byte from the Linux kernel header image.
// C: setup_sects = buf[0x1f1];
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn copy_kernel__part4245(buf: *const u8) -> c_int {
    unsafe { *buf.add(0x1f1) as c_int }
}
}

// ── `if (setup_sects == 0)` · x86_machine.c:2362-2363 · chunk 4246 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (setup_sects == 0)
//             setup_sects = 4;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4246 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn copy_kernel__part4246(setup_sects: *mut c_int) {
    unsafe {
        if setup_sects.is_null() {
            return;
        }
        if *setup_sects == 0 {
            *setup_sects = 4;
        }
    }
}
}

// ── `header_len = (setup_sects + 1) * 512;` · x86_machine.c:2364-2364 · chunk 4247 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     header_len = (setup_sects + 1) * 512;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4247 {
// Ported piece of copy_kernel: header_len = (setup_sects + 1) * 512;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn copy_kernel__part4247(setup_sects: c_int) -> c_int {
    let sects = setup_sects.wrapping_add(1);
    sects.wrapping_mul(512)
}
}

// ── `if (buf_len < header_len)` · x86_machine.c:2365-2366 · chunk 4248 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buf_len < header_len)
//             goto too_small;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4248 {
use core::ffi::c_int;

// Fragment of copy_kernel: tests whether buf_len < header_len, which makes
// the C caller jump to its `too_small` label.
#[no_mangle]
pub extern "C" fn copy_kernel__part4248(buf_len: c_int, header_len: c_int) -> c_int {
    if buf_len < header_len {
        1
    } else {
        0
    }
}
}

// ── `if (memcmp(buf + 0x202, "HdrS", 4) != 0)` · x86_machine.c:2367-2370 · chunk 4249 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (memcmp(buf + 0x202, "HdrS", 4) != 0) {
//             fprintf(
//                    stderr
//                          , "Kernel too old\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4249 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

static HDRS: [u8; 4] = [b'H', b'd', b'r', b'S'];

/// Returns 1 if the kernel header magic at buf+0x202 is not "HdrS" (kernel too
/// old), 0 otherwise.
#[no_mangle]
pub extern "C" fn copy_kernel__part4249(buf: *const u8) -> c_int {
    unsafe {
        if memcmp(
            buf.add(0x202) as *const c_void,
            HDRS.as_ptr() as *const c_void,
            4,
        ) != 0
        {
            1
        } else {
            0
        }
    }
}
}

// ── `load_address = 0x100000; ram_ptr = get_ram_ptr(s, load_address); copy_len = buf_len - header_le…` · x86_machine.c:2371-2374 · chunk 4250 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     load_address = 0x100000;
//     
//         ram_ptr = get_ram_ptr(s, load_address);
//         copy_len = buf_len - header_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4250 {
// Rust rewrite of copy_kernel__part4250:
//   load_address = 0x100000;
//   ram_ptr = get_ram_ptr(s, load_address);
//   copy_len = buf_len - header_len;
//
// get_ram_ptr() is a `static` C function (no linker symbol), so its logic is
// reproduced here against the public get_phys_mem_range() helper.

use core::ffi::{c_int, c_void};

// Mirror of the C PhysMemoryRange struct (fields in order, same types).
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: *mut c_void,
    pub write_func: *mut c_void,
    pub devio_flags: c_int,
}

extern "C" {
    fn get_phys_mem_range(s: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
}

// Port of the static C helper get_ram_ptr().
unsafe fn ram_ptr_for(mem_map: *mut c_void, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range(mem_map, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return core::ptr::null_mut();
    }
    let off = paddr.wrapping_sub((*pr).addr) as usize;
    (*pr).phys_mem.wrapping_add(off)
}

#[no_mangle]
pub extern "C" fn copy_kernel__part4250(
    mem_map: *mut c_void,
    buf_len: c_int,
    header_len: c_int,
    load_address: *mut u32,
    ram_ptr: *mut *mut u8,
    copy_len: *mut c_int,
) {
    unsafe {
        let addr: u32 = 0x100000u32;
        *load_address = addr;
        *ram_ptr = ram_ptr_for(mem_map, addr as u64);
        *copy_len = buf_len.wrapping_sub(header_len);
    }
}
}

// ── `if (copy_len > (s->ram_size - load_address)` · x86_machine.c:2375-2378 · chunk 4251 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (copy_len > (s->ram_size - load_address)) {
//             fprintf(
//                    stderr
//                          , "Not enough RAM\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4251 {
use core::ffi::c_int;

// Checks whether the kernel image fits into RAM above the load address.
// Mirrors the C comparison: `copy_len > (uint64_t)(s->ram_size - load_address)`,
// where the int `copy_len` is converted to uint64_t (sign extension then
// reinterpretation) and the subtraction is done in uint64_t arithmetic.
#[no_mangle]
pub extern "C" fn copy_kernel__part4251(ram_size: u64, load_address: u32, copy_len: c_int) -> c_int {
    let available: u64 = ram_size.wrapping_sub(load_address as u64);
    let needed: u64 = copy_len as i64 as u64;
    if needed > available {
        1
    } else {
        0
    }
}
}

// ── `memcpy(ram_ptr, buf + header_len, copy_len); params = (void *)get_ram_ptr(s, 0x00090000); memse…` · x86_machine.c:2379-2407 · chunk 4252 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(ram_ptr, buf + header_len, copy_len);
//     
//         params = (void *)get_ram_ptr(s, 0x00090000);
//     
//         memset(params, 0, sizeof(struct linux_params));
//     
//     
//         setup_hdr_start = 0x1f1;
//         setup_hdr_end = 0x202 + buf[0x201];
//         memcpy((uint8_t *)params + setup_hdr_start, buf + setup_hdr_start,
//                setup_hdr_end - setup_hdr_start);
//     
//         strcpy((char *)params->commandline, cmd_line);
//     
//         params->mount_root_rdonly = 0;
//         params->cmd_line_ptr = 0x00090000 +
//             ((size_t) &((struct linux_params *)0)->commandline);
//         params->alt_mem_k = (s->ram_size / 1024) - 1024;
//         params->loader_type = 0x01;
//     
//     
//     
//     
//     
//     
//         params->orig_video_lines = 0;
//         params->orig_video_cols = 0;
//     
//         fb_dev = s->common.fb_dev;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4252 {
// Port of part of copy_kernel() from x86_machine.c:
// the kernel/params copy + linux_params setup part.
use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;

#[repr(C)]
pub struct FBDevice {
    width: c_int,
    height: c_int,
    stride: c_int,
    fb_data: *mut u8,
    fb_size: c_int,
    device_opaque: *mut c_void,
    refresh: *mut c_void,
}

#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct IRQSignal {
    set_irq: *mut c_void,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut PhysMemoryMap,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: BOOL,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: *mut c_void,
    write_func: *mut c_void,
    devio_flags: c_int,
}

#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IRQSignal; 16],
    pit_state: *mut c_void,
    i440fx_state: *mut c_void,
    cmos_state: *mut c_void,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: BOOL,
    kvm_fd: c_int,
    vm_fd: c_int,
    vcpu_fd: c_int,
    kvm_run_size: c_int,
    kvm_run: *mut c_void,
}

#[repr(C)]
pub struct E820Entry {
    addr: u64,
    size: u64,
    r#type: u32,
}

// struct linux_params, mirrored field for field. The C definition is
// __attribute__((packed)); the nested e820map element struct is *not* packed,
// so it keeps its natural size (24) exactly like this mirror does.
#[repr(C, packed)]
pub struct LinuxParams {
    orig_x: u8,
    orig_y: u8,
    ext_mem_k: u16,
    orig_video_page: u16,
    orig_video_mode: u8,
    orig_video_cols: u8,
    flags: u8,
    unused2: u8,
    orig_video_ega_bx: u16,
    unused3: u16,
    orig_video_lines: u8,
    orig_video_is_vga: u8,
    orig_video_points: u16,

    lfb_width: u16,
    lfb_height: u16,
    lfb_depth: u16,
    lfb_base: u32,
    lfb_size: u32,
    cl_magic: u16,
    cl_offset: u16,
    lfb_linelength: u16,
    red_size: u8,
    red_pos: u8,
    green_size: u8,
    green_pos: u8,
    blue_size: u8,
    blue_pos: u8,
    rsvd_size: u8,
    rsvd_pos: u8,
    vesapm_seg: u16,
    vesapm_off: u16,
    pages: u16,
    vesa_attributes: u16,
    capabilities: u32,
    ext_lfb_base: u32,
    reserved2: [u8; 2],

    apm_bios_info: [u8; 20],
    pad2: [u8; 0x80 - 0x54],

    hd0_info: [u8; 16],
    hd1_info: [u8; 16],

    sys_description_len: u16,
    sys_description_table: [u8; 14],

    pad3: [u8; 0x1e0 - 0xb0],
    alt_mem_k: u32,
    pad4: [u8; 4],
    e820map_entries: u8,
    eddbuf_entries: u8,
    pad5: [u8; 0x1f1 - 0x1ea],
    setup_sects: u8,
    mount_root_rdonly: u16,
    sys_size: u16,

    swap_dev: u16,
    ramdisk_flags: u16,
    vga_mode: u16,
    orig_root_dev: u16,
    pad6: [u8; 1],
    aux_device_info: u8,
    jump_setup: u16,

    setup_signature: [u8; 4],
    header_format_version: u16,
    setup_s_temp0: [u8; 8],

    loader_type: u8,
    loadflags: u8,

    setup_s_temp1: u16,
    kernel_start: u32,
    initrd_start: u32,
    initrd_size: u32,
    setup_s_temp2: [u8; 4],
    setup_s_heap_end_pointer: u16,
    pad70: u16,
    cmd_line_ptr: u32,
    pad7: [u8; 0x2d0 - 0x22c],

    e820map: [E820Entry; 32],

    pad8: [u8; 0x600 - 0x550],

    eddbuf: [u8; 0x7d4 - 0x600],

    pad9: [u8; 0x800 - 0x7d4],
    commandline: [u8; 0x800],

    gdt_table: [u64; 4],
}

extern "C" {
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;
}

// static uint8_t *get_ram_ptr(PCMachine *s, uint64_t paddr) -- static in C,
// so its logic is ported here.
unsafe fn get_ram_ptr(s: *mut PCMachine, paddr: u64) -> *mut u8 {
    let pr = get_phys_mem_range((*s).mem_map, paddr);
    if pr.is_null() || (*pr).is_ram == 0 {
        return core::ptr::null_mut();
    }
    (*pr)
        .phys_mem
        .wrapping_add(paddr.wrapping_sub((*pr).addr) as usize)
}

#[inline]
unsafe fn wr<T>(p: *mut T, v: T) {
    core::ptr::write_unaligned(p, v);
}

#[no_mangle]
pub unsafe extern "C" fn copy_kernel__part4252(
    s: *mut PCMachine,
    buf: *const u8,
    cmd_line: *const c_char,
    header_len: c_int,
    copy_len: c_int,
) -> *mut LinuxParams {
    // memcpy(ram_ptr, buf + header_len, copy_len);
    let ram_ptr = get_ram_ptr(s, 0x100000);
    core::ptr::copy_nonoverlapping(buf.add(header_len as usize), ram_ptr, copy_len as usize);

    // params = (void *)get_ram_ptr(s, 0x00090000);
    let params = get_ram_ptr(s, 0x00090000) as *mut LinuxParams;

    // memset(params, 0, sizeof(struct linux_params));
    core::ptr::write_bytes(
        params as *mut u8,
        0,
        core::mem::size_of::<LinuxParams>(),
    );

    // copy the setup header
    let setup_hdr_start: usize = 0x1f1;
    let setup_hdr_end: usize = 0x202usize.wrapping_add(*buf.add(0x201) as usize);
    core::ptr::copy_nonoverlapping(
        buf.add(setup_hdr_start),
        (params as *mut u8).add(setup_hdr_start),
        setup_hdr_end.wrapping_sub(setup_hdr_start),
    );

    // strcpy((char *)params->commandline, cmd_line);
    let dst = core::ptr::addr_of_mut!((*params).commandline) as *mut u8;
    let mut i: usize = 0;
    loop {
        let c = *cmd_line.add(i);
        *dst.add(i) = c as u8;
        i = i.wrapping_add(1);
        if c == 0 {
            break;
        }
    }

    // params->mount_root_rdonly = 0;
    wr(core::ptr::addr_of_mut!((*params).mount_root_rdonly), 0u16);

    // params->cmd_line_ptr = 0x00090000 + offsetof(struct linux_params, commandline);
    let cmdline_off = (core::ptr::addr_of_mut!((*params).commandline) as usize)
        .wrapping_sub(params as usize);
    wr(
        core::ptr::addr_of_mut!((*params).cmd_line_ptr),
        0x00090000u32.wrapping_add(cmdline_off as u32),
    );

    // params->alt_mem_k = (s->ram_size / 1024) - 1024;
    let ram_size = (*s).ram_size;
    let alt_mem_k = ram_size.wrapping_div(1024).wrapping_sub(1024) as u32;
    wr(core::ptr::addr_of_mut!((*params).alt_mem_k), alt_mem_k);

    // params->loader_type = 0x01;
    wr(core::ptr::addr_of_mut!((*params).loader_type), 0x01u8);

    // params->orig_video_lines = 0; params->orig_video_cols = 0;
    wr(core::ptr::addr_of_mut!((*params).orig_video_lines), 0u8);
    wr(core::ptr::addr_of_mut!((*params).orig_video_cols), 0u8);

    // fb_dev = s->common.fb_dev; if (fb_dev) { ... }
    let fb_dev = (*s).common.fb_dev;
    if !fb_dev.is_null() {
        wr(
            core::ptr::addr_of_mut!((*params).orig_video_is_vga),
            0x23u8,
        );

        wr(core::ptr::addr_of_mut!((*params).lfb_depth), 32u16);
        wr(core::ptr::addr_of_mut!((*params).red_size), 8u8);
        wr(core::ptr::addr_of_mut!((*params).red_pos), 16u8);
        wr(core::ptr::addr_of_mut!((*params).green_size), 8u8);
        wr(core::ptr::addr_of_mut!((*params).green_pos), 8u8);
        wr(core::ptr::addr_of_mut!((*params).blue_size), 8u8);
        wr(core::ptr::addr_of_mut!((*params).blue_pos), 0u8);
        wr(core::ptr::addr_of_mut!((*params).rsvd_size), 8u8);
        wr(core::ptr::addr_of_mut!((*params).rsvd_pos), 24u8);

        wr(
            core::ptr::addr_of_mut!((*params).lfb_width),
            (*fb_dev).width as u16,
        );
        wr(
            core::ptr::addr_of_mut!((*params).lfb_height),
            (*fb_dev).height as u16,
        );
        wr(
            core::ptr::addr_of_mut!((*params).lfb_linelength),
            (*fb_dev).stride as u16,
        );
        wr(
            core::ptr::addr_of_mut!((*params).lfb_size),
            (*fb_dev).fb_size as u32,
        );
        wr(
            core::ptr::addr_of_mut!((*params).lfb_base),
            0xf0400000u32,
        );
    }

    params
}
}

// ── `if (fb_dev)` · x86_machine.c:2408-2427 · chunk 4253 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (fb_dev) {
//     
//             params->orig_video_isVGA = 0x23;
//     
//             params->lfb_depth = 32;
//             params->red_size = 8;
//             params->red_pos = 16;
//             params->green_size = 8;
//             params->green_pos = 8;
//             params->blue_size = 8;
//             params->blue_pos = 0;
//             params->rsvd_size = 8;
//             params->rsvd_pos = 24;
//     
//             params->lfb_width = fb_dev->width;
//             params->lfb_height = fb_dev->height;
//             params->lfb_linelength = fb_dev->stride;
//             params->lfb_size = fb_dev->fb_size;
//             params->lfb_base = 0xf0400000;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4253 {
// Port of the framebuffer-related parameter setup block from copy_kernel().
use core::ffi::{c_int, c_void};

// struct { uint64_t addr; uint64_t size; uint32_t type; } (packed, inside a packed struct)
#[repr(C, packed)]
pub struct E820Entry {
    pub addr: u64,
    pub size: u64,
    pub type_: u32,
}

// struct __attribute__((packed)) linux_params
#[repr(C, packed)]
pub struct LinuxParams {
    pub orig_x: u8,
    pub orig_y: u8,
    pub ext_mem_k: u16,
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u8,
    pub unused2: u8,
    pub orig_video_ega_bx: u16,
    pub unused3: u16,
    pub orig_video_lines: u8,
    pub orig_video_isVGA: u8,
    pub orig_video_points: u16,

    pub lfb_width: u16,
    pub lfb_height: u16,
    pub lfb_depth: u16,
    pub lfb_base: u32,
    pub lfb_size: u32,
    pub cl_magic: u16,
    pub cl_offset: u16,
    pub lfb_linelength: u16,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
    pub vesapm_seg: u16,
    pub vesapm_off: u16,
    pub pages: u16,
    pub vesa_attributes: u16,
    pub capabilities: u32,
    pub ext_lfb_base: u32,
    pub _reserved: [u8; 2],

    pub apm_bios_info: [u8; 20],
    pub pad2: [u8; 0x80 - 0x54],

    pub hd0_info: [u8; 16],
    pub hd1_info: [u8; 16],

    pub sys_description_len: u16,
    pub sys_description_table: [u8; 14],

    pub pad3: [u8; 0x1e0 - 0xb0],
    pub alt_mem_k: u32,
    pub pad4: [u8; 4],
    pub e820map_entries: u8,
    pub eddbuf_entries: u8,
    pub pad5: [u8; 0x1f1 - 0x1ea],
    pub setup_sects: u8,
    pub mount_root_rdonly: u16,
    pub sys_size: u16,
    pub swap_dev: u16,
    pub ramdisk_flags: u16,
    pub vga_mode: u16,
    pub orig_root_dev: u16,
    pub pad6: [u8; 1],
    pub aux_device_info: u8,
    pub jump_setup: u16,
    pub setup_signature: [u8; 4],
    pub header_format_version: u16,
    pub setup_S_temp0: [u8; 8],
    pub loader_type: u8,
    pub loadflags: u8,
    pub setup_S_temp1: u16,
    pub kernel_start: u32,
    pub initrd_start: u32,
    pub initrd_size: u32,
    pub setup_S_temp2: [u8; 4],
    pub setup_S_heap_end_pointer: u16,
    pub pad70: u16,
    pub cmd_line_ptr: u32,
    pub pad7: [u8; 0x2d0 - 0x22c],
    pub e820map: [E820Entry; 32],

    pub pad8: [u8; 0x600 - 0x550],

    pub eddbuf: [u8; 0x7d4 - 0x600],

    pub pad9: [u8; 0x800 - 0x7d4],
    pub commandline: [u8; 0x800],

    pub gdt_table: [u64; 4],
}

pub type SimpleFBDrawFunc = unsafe extern "C" fn(
    fb_dev: *mut FBDevice,
    opaque: *mut c_void,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
);

pub type FBRefreshFunc = unsafe extern "C" fn(
    fb_dev: *mut FBDevice,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
);

// struct FBDevice
#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<FBRefreshFunc>,
}

#[no_mangle]
pub unsafe extern "C" fn copy_kernel__part4253(params: *mut LinuxParams, fb_dev: *mut FBDevice) {
    if fb_dev.is_null() {
        return;
    }

    (*params).orig_video_isVGA = 0x23;

    (*params).lfb_depth = 32;
    (*params).red_size = 8;
    (*params).red_pos = 16;
    (*params).green_size = 8;
    (*params).green_pos = 8;
    (*params).blue_size = 8;
    (*params).blue_pos = 0;
    (*params).rsvd_size = 8;
    (*params).rsvd_pos = 24;

    (*params).lfb_width = (*fb_dev).width as u16;
    (*params).lfb_height = (*fb_dev).height as u16;
    (*params).lfb_linelength = (*fb_dev).stride as u16;
    (*params).lfb_size = (*fb_dev).fb_size as u32;
    (*params).lfb_base = 0xf0400000;
}
}

// ── `params->gdt_table[2] = 0x00cf9b000000ffffLL; params->gdt_table[3] = 0x00cf93000000ffffLL;` · x86_machine.c:2429-2430 · chunk 4254 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     params->gdt_table[2] = 0x00cf9b000000ffffLL;
//         params->gdt_table[3] = 0x00cf93000000ffffLL;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4254 {
// Port of copy_kernel() GDT table initialization (x86_machine.c).
// Writes the two 64-bit GDT descriptors into the linux_params gdt_table array.
#[no_mangle]
pub extern "C" fn copy_kernel__part4254(gdt_table: *mut u64) {
    unsafe {
        if gdt_table.is_null() {
            return;
        }
        *gdt_table.add(2) = 0x00cf9b000000ffffu64;
        *gdt_table.add(3) = 0x00cf93000000ffffu64;
    }
}
}

// ── `if (s->kvm_enabled) … else` · x86_machine.c:2433-2510 · chunk 4255 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_enabled) {
//             struct kvm_sregs sregs;
//             struct kvm_segment seg;
//             struct kvm_regs regs;
//     
//     
//     
//             if (ioctl(s->vcpu_fd, 
//                                  (((2U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x83)) << 0) | ((((sizeof(struct kvm_sregs)))) << ((0 +8)+8)))
//                                               , &sregs) < 0) {
//                 perror("KVM_GET_SREGS");
//                 exit(1);
//             }
//     
//             sregs.cr0 |= (1 << 0);
//             sregs.gdt.base = 0x00090000 +
//                 ((size_t) &((struct linux_params *)0)->gdt_table);
//             sregs.gdt.limit = sizeof(params->gdt_table) - 1;
//     
//             memset(&seg, 0, sizeof(seg));
//             seg.limit = 0xffffffff;
//             seg.present = 1;
//             seg.db = 1;
//             seg.s = 1;
//             seg.g = 1;
//     
//             seg.type = 0xb;
//             seg.selector = 2 << 3;
//             sregs.cs = seg;
//     
//             seg.type = 0x3;
//             seg.selector = 3 << 3;
//             sregs.ds = seg;
//             sregs.es = seg;
//             sregs.ss = seg;
//             sregs.fs = seg;
//             sregs.gs = seg;
//     
//             if (ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x84)) << 0) | ((((sizeof(struct kvm_sregs)))) << ((0 +8)+8)))
//                                               , &sregs) < 0) {
//                 perror("KVM_SET_SREGS");
//                 exit(1);
//             }
//     
//             memset(&regs, 0, sizeof(regs));
//             regs.rip = load_address;
//             regs.rsi = 0x00090000;
//             regs.rflags = 0x2;
//             if (ioctl(s->vcpu_fd, 
//                                  (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x82)) << 0) | ((((sizeof(struct kvm_regs)))) << ((0 +8)+8)))
//                                              , &regs) < 0) {
//                 perror("KVM_SET_REGS");
//                 exit(1);
//             }
//         } else
//     
//         {
//             int i;
//             X86CPUSeg sd;
//             uint32_t val;
//             val = x86_cpu_get_reg(s->cpu_state, 9);
//             x86_cpu_set_reg(s->cpu_state, 9, val | (1 << 0));
//     
//             sd.base = 0x00090000 +
//                 ((size_t) &((struct linux_params *)0)->gdt_table);
//             sd.limit = sizeof(params->gdt_table) - 1;
//             x86_cpu_set_seg(s->cpu_state, 8, &sd);
//             sd.sel = 2 << 3;
//             sd.base = 0;
//             sd.limit = 0xffffffff;
//             sd.flags = 0xc09b;
//             x86_cpu_set_seg(s->cpu_state, 1, &sd);
//             sd.sel = 3 << 3;
//             sd.flags = 0xc093;
//             for(i = 0; i < 6; i++) {
//                 if (i != 1) {
//                     x86_cpu_set_seg(s->cpu_state, i, &sd);
//                 }
//             }
//     
//             x86_cpu_set_reg(s->cpu_state, 8, load_address);
//             x86_cpu_set_reg(s->cpu_state, 6, 0x00090000);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4255 {
// Ported piece of copy_kernel: KVM vs. software CPU start state setup.

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct X86CpuState {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct X86CpuSeg {
    pub sel: u16,
    pub flags: u16,
    pub base: u32,
    pub limit: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmSegment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub r#type: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    pub unusable: u8,
    pub padding: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmDTable {
    pub base: u64,
    pub limit: u16,
    pub padding: [u16; 3],
}

#[repr(C)]
pub struct KvmSregs {
    pub cs: KvmSegment,
    pub ds: KvmSegment,
    pub es: KvmSegment,
    pub fs: KvmSegment,
    pub gs: KvmSegment,
    pub ss: KvmSegment,
    pub tr: KvmSegment,
    pub ldt: KvmSegment,
    pub gdt: KvmDTable,
    pub idt: KvmDTable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; 4],
}

#[repr(C)]
pub struct KvmRegs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn x86_cpu_get_reg(s: *mut X86CpuState, reg: c_int) -> u32;
    fn x86_cpu_set_reg(s: *mut X86CpuState, reg: c_int, val: u32);
    fn x86_cpu_set_seg(s: *mut X86CpuState, seg: c_int, sd: *const X86CpuSeg);
}

// _IOC/_IOR/_IOW encodings for the KVM ioctls used here (KVMIO == 0xAE).
const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_NONE: u32 = 0;
const IOC_READ: u32 = 2;
const IOC_WRITE: u32 = 1;

const KVMIO: u32 = 0xAE;

const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | ((size as u32) << IOC_SIZESHIFT)
}

// struct linux_params is packed; gdt_table sits at byte offset 0x1000.
const PARAMS_BASE: u64 = 0x00090000;
const PARAMS_GDT_TABLE_ADDR: u64 = PARAMS_BASE + 0x1000;
const GDT_TABLE_SIZE: u32 = 4 * 8;

const KVM_GET_SREGS: u32 = ioc(IOC_READ, KVMIO, 0x83, core::mem::size_of::<KvmSregs>());
const KVM_SET_SREGS: u32 = ioc(IOC_WRITE, KVMIO, 0x84, core::mem::size_of::<KvmSregs>());
const KVM_SET_REGS: u32 = ioc(IOC_WRITE, KVMIO, 0x82, core::mem::size_of::<KvmRegs>());
const _: () = {
    if KVM_GET_SREGS == 0 {
        // keep the none-direction constant used
    }
};
const _IOC_NONE_UNUSED: u32 = IOC_NONE;

#[no_mangle]
pub unsafe extern "C" fn copy_kernel__part4255(
    kvm_enabled: c_int,
    vcpu_fd: c_int,
    cpu_state: *mut X86CpuState,
    load_address: u32,
) {
    if kvm_enabled != 0 {
        let mut sregs: KvmSregs = core::mem::zeroed();

        if ioctl(
            vcpu_fd,
            KVM_GET_SREGS as c_ulong,
            &mut sregs as *mut KvmSregs,
        ) < 0
        {
            perror(b"KVM_GET_SREGS\0".as_ptr() as *const c_char);
            exit(1);
        }

        sregs.cr0 |= 1u64;
        sregs.gdt.base = PARAMS_GDT_TABLE_ADDR;
        sregs.gdt.limit = (GDT_TABLE_SIZE - 1) as u16;

        let mut seg: KvmSegment = core::mem::zeroed();
        seg.limit = 0xffffffffu32;
        seg.present = 1;
        seg.db = 1;
        seg.s = 1;
        seg.g = 1;

        seg.r#type = 0xb;
        seg.selector = (2u16) << 3;
        sregs.cs = seg;

        seg.r#type = 0x3;
        seg.selector = (3u16) << 3;
        sregs.ds = seg;
        sregs.es = seg;
        sregs.ss = seg;
        sregs.fs = seg;
        sregs.gs = seg;

        if ioctl(
            vcpu_fd,
            KVM_SET_SREGS as c_ulong,
            &mut sregs as *mut KvmSregs,
        ) < 0
        {
            perror(b"KVM_SET_SREGS\0".as_ptr() as *const c_char);
            exit(1);
        }

        let mut regs: KvmRegs = core::mem::zeroed();
        regs.rip = load_address as u64;
        regs.rsi = 0x00090000;
        regs.rflags = 0x2;
        if ioctl(vcpu_fd, KVM_SET_REGS as c_ulong, &mut regs as *mut KvmRegs) < 0 {
            perror(b"KVM_SET_REGS\0".as_ptr() as *const c_char);
            exit(1);
        }
    } else {
        let mut sd: X86CpuSeg = X86CpuSeg {
            sel: 0,
            flags: 0,
            base: 0,
            limit: 0,
        };
        let val: u32 = x86_cpu_get_reg(cpu_state, 9);
        x86_cpu_set_reg(cpu_state, 9, val | 1u32);

        sd.base = PARAMS_GDT_TABLE_ADDR as u32;
        sd.limit = GDT_TABLE_SIZE - 1;
        x86_cpu_set_seg(cpu_state, 8, &sd);
        sd.sel = (2u16) << 3;
        sd.base = 0;
        sd.limit = 0xffffffffu32;
        sd.flags = 0xc09b;
        x86_cpu_set_seg(cpu_state, 1, &sd);
        sd.sel = (3u16) << 3;
        sd.flags = 0xc093;
        let mut i: c_int = 0;
        while i < 6 {
            if i != 1 {
                x86_cpu_set_seg(cpu_state, i, &sd);
            }
            i += 1;
        }

        x86_cpu_set_reg(cpu_state, 8, load_address);
        x86_cpu_set_reg(cpu_state, 6, 0x00090000);
    }
}
}

// ── `{ … }` · x86_machine.c:2513-2522 · chunk 4256 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//             uint8_t elcr[2];
//             static const uint8_t pci_irqs[4] = { 9, 10, 11, 12 };
//     
//             i440fx_map_interrupts(s->i440fx_state, elcr, pci_irqs);
//     
//             if (s->pic_state) {
//                 pic2_set_elcr(s->pic_state, elcr);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4256 {
// Port of the PCI interrupt mapping block at the end of copy_kernel().
use core::ffi::c_void;

extern "C" {
    fn i440fx_map_interrupts(
        s: *mut c_void,
        elcr: *mut u8,
        pci_irqs: *const u8,
    );
    fn pic2_set_elcr(s: *mut c_void, elcr: *const u8);
}

static PCI_IRQS: [u8; 4] = [9, 10, 11, 12];

#[no_mangle]
pub extern "C" fn copy_kernel__part4256(i440fx_state: *mut c_void, pic_state: *mut c_void) {
    let mut elcr: [u8; 2] = [0, 0];

    unsafe {
        i440fx_map_interrupts(i440fx_state, elcr.as_mut_ptr(), PCI_IRQS.as_ptr());

        if !pic_state.is_null() {
            pic2_set_elcr(pic_state, elcr.as_ptr());
        }
    }
}
}

// ── `pc_machine_get_sleep_duration` · x86_machine.c:2526-2544 · chunk 4257 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int pc_machine_get_sleep_duration(VirtMachine *s1, int delay)
//     {
//         PCMachine *s = (PCMachine *)s1;
//     
//     
//         if (s->kvm_enabled) {
//     
//             cmos_update_irq(s->cmos_state);
//             delay = 0;
//         } else
//     
//         {
//             cmos_update_irq(s->cmos_state);
//             delay = min_int(delay, pit_update_irq(s->pit_state));
//             if (!x86_cpu_get_power_down(s->cpu_state))
//                 delay = 0;
//         }
//         return delay;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4257 {
#![allow(non_camel_case_types, non_snake_case, dead_code, clippy::all)]

use core::ffi::{c_int, c_void};

// ---- externs provided by the rest of the C program / libc ----

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clockid: c_int, tp: *mut Timespec) -> c_int;
    fn x86_cpu_get_power_down(s: *mut c_void) -> c_int;
}

// ---- mirrors of the C types touched ----

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);
type PITGetTicksFunc = unsafe extern "C" fn(opaque: *mut c_void) -> i64;

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
pub struct CMOSState {
    cmos_index: u8,
    cmos_data: [u8; 128],
    irq: *mut IRQSignal,
    use_local_time: c_int,
    irq_timeout: u32,
    irq_period: u32,
}

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
    irq: *mut IRQSignal,
    opaque: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    common: VirtMachine,
    ram_size: u64,
    mem_map: *mut c_void,
    port_map: *mut c_void,
    cpu_state: *mut c_void,
    pic_state: *mut c_void,
    pic_irq: [IRQSignal; 16],
    pit_state: *mut PITState,
    i440fx_state: *mut c_void,
    cmos_state: *mut CMOSState,
    serial_state: *mut c_void,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    kbd_state: *mut c_void,
    ps2_mouse: *mut c_void,
    vm_mouse: *mut c_void,
    ps2_kbd: *mut c_void,
    kvm_enabled: c_int,
}

// ---- private ports of the C static helpers ----

// static inline int min_int(int a, int b)
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static inline void set_irq(IRQSignal *irq, int level)
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let f = (*irq).set_irq;
        if let Some(f) = f {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

// static uint32_t cmos_get_timer(CMOSState *s)
unsafe fn cmos_get_timer() -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // CLOCK_REALTIME == 1
    clock_gettime(1, &mut ts);
    let sec: u64 = (ts.tv_sec as u32).wrapping_mul(32768) as u64;
    let nsec: u64 = (ts.tv_nsec as u64).wrapping_mul(32768) / 1000000000u64;
    sec.wrapping_add(nsec) as u32
}

// static void cmos_update_irq(CMOSState *s)
unsafe fn cmos_update_irq(s: *mut CMOSState) {
    if ((*s).cmos_data[11] & 0x40) != 0 {
        let d: u32 = cmos_get_timer().wrapping_sub((*s).irq_timeout);
        if (d as i32) >= 0 {
            (*s).cmos_data[12] |= 0xc0;
            set_irq((*s).irq, 1);
            (*s).irq_timeout = (*s).irq_timeout.wrapping_add((*s).irq_period);
        }
    }
}

// static int64_t pit_get_time(PITChannel *pc)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s_outer: *mut PITState = (*pc).pit_state;
    match (*s_outer).get_ticks {
        Some(f) => f((*s_outer).opaque),
        None => 0,
    }
}

// static int pit_update_irq(PITState *pit)
unsafe fn pit_update_irq(pit: *mut PITState) -> c_int {
    let s: *mut PITChannel = core::ptr::addr_of_mut!((*pit).pit_channels).cast::<PITChannel>();

    let mut delay: i64 = 1193182;

    let d: i64 = pit_get_time(s).wrapping_sub((*s).count_load_time);
    let mode: u8 = (*s).mode;
    if mode == 2 || mode == 3 {
        delay = (*s)
            .last_irq_time
            .wrapping_add((*s).count as i64)
            .wrapping_sub(d);
        if delay <= 0 {
            set_irq((*pit).irq, 1);
            set_irq((*pit).irq, 0);
            (*s).last_irq_time = (*s).last_irq_time.wrapping_add((*s).count as i64);
        }
    } else {
        // default: case 0: case 1: case 4: case 5:
        if (*s).last_irq_time == 0 {
            delay = ((*s).count as i64).wrapping_sub(d);
            if delay <= 0 {
                set_irq((*pit).irq, 1);
                set_irq((*pit).irq, 0);
                (*s).last_irq_time = d;
            }
        }
    }

    if delay <= 0 {
        0
    } else {
        (delay / (1193182 / 1000)) as c_int
    }
}

// ---- the target ----

#[no_mangle]
pub extern "C" fn pc_machine_get_sleep_duration(s1: *mut VirtMachine, delay: c_int) -> c_int {
    unsafe {
        let s: *mut PCMachine = s1 as *mut PCMachine;
        let mut delay: c_int = delay;

        if (*s).kvm_enabled != 0 {
            cmos_update_irq((*s).cmos_state);
            delay = 0;
        } else {
            cmos_update_irq((*s).cmos_state);
            delay = min_int(delay, pit_update_irq((*s).pit_state));
            if x86_cpu_get_power_down((*s).cpu_state) == 0 {
                delay = 0;
            }
        }
        delay
    }
}
}

// ── `if (s->kvm_enabled) … else` · x86_machine.c:2531-2542 · chunk 4258 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_enabled) {
//     
//             cmos_update_irq(s->cmos_state);
//             delay = 0;
//         } else
//     
//         {
//             cmos_update_irq(s->cmos_state);
//             delay = min_int(delay, pit_update_irq(s->pit_state));
//             if (!x86_cpu_get_power_down(s->cpu_state))
//                 delay = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4258 {
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_assignments, unused_mut)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct CMOSState {
    pub cmos_index: u8,
    pub cmos_data: [u8; 128],
    pub irq: *mut IRQSignal,
    pub use_local_time: BOOL,
    pub irq_timeout: u32,
    pub irq_period: u32,
}

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
    pub irq: *mut IRQSignal,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: BOOL,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut c_void,
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
    fn x86_cpu_get_power_down(s: *mut c_void) -> BOOL;
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

// ported from static inline int min_int(int a, int b)
#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// ported from static uint32_t cmos_get_timer(CMOSState *s)
unsafe fn cmos_get_timer(_s: *mut CMOSState) -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts);
    let a: u32 = (ts.tv_sec as u32).wrapping_mul(32768);
    let b: u64 = (ts.tv_nsec as u64).wrapping_mul(32768) / 1000000000u64;
    (a as u64).wrapping_add(b) as u32
}

// ported from static void cmos_update_irq(CMOSState *s)
unsafe fn cmos_update_irq(s: *mut CMOSState) {
    let data: *mut u8 = (*s).cmos_data.as_mut_ptr();
    if (*data.add(11) & 0x40) != 0 {
        let d: u32 = cmos_get_timer(s).wrapping_sub((*s).irq_timeout);
        if (d as i32) >= 0 {
            *data.add(12) |= 0xc0;
            set_irq((*s).irq, 1);
            (*s).irq_timeout = (*s).irq_timeout.wrapping_add((*s).irq_period);
        }
    }
}

// ported from static int64_t pit_get_time(PITChannel *pc)
unsafe fn pit_get_time(pc: *mut PITChannel) -> i64 {
    let s: *mut PITState = (*pc).pit_state;
    if s.is_null() {
        return 0;
    }
    match (*s).get_ticks {
        Some(f) => f((*s).opaque),
        None => 0,
    }
}

// ported from static int pit_update_irq(PITState *pit)
unsafe fn pit_update_irq(pit: *mut PITState) -> c_int {
    let s: *mut PITChannel = (*pit).pit_channels.as_mut_ptr();
    let mut delay: i64 = 1193182;
    let d: i64 = pit_get_time(s).wrapping_sub((*s).count_load_time);

    match (*s).mode {
        2 | 3 => {
            delay = (*s)
                .last_irq_time
                .wrapping_add((*s).count as i64)
                .wrapping_sub(d);
            if delay <= 0 {
                set_irq((*pit).irq, 1);
                set_irq((*pit).irq, 0);
                (*s).last_irq_time = (*s).last_irq_time.wrapping_add((*s).count as i64);
            }
        }
        _ => {
            if (*s).last_irq_time == 0 {
                delay = ((*s).count as i64).wrapping_sub(d);
                if delay <= 0 {
                    set_irq((*pit).irq, 1);
                    set_irq((*pit).irq, 0);
                    (*s).last_irq_time = d;
                }
            }
        }
    }

    if delay <= 0 {
        0
    } else {
        (delay / (1193182 / 1000)) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn pc_machine_get_sleep_duration__part4258(
    s1: *mut VirtMachine,
    delay_in: c_int,
) -> c_int {
    let s: *mut PCMachine = s1 as *mut PCMachine;
    let mut delay: c_int = delay_in;

    if (*s).kvm_enabled != 0 {
        cmos_update_irq((*s).cmos_state as *mut CMOSState);
        delay = 0;
    } else {
        cmos_update_irq((*s).cmos_state as *mut CMOSState);
        delay = min_int(delay, pit_update_irq((*s).pit_state as *mut PITState));
        if x86_cpu_get_power_down((*s).cpu_state) == 0 {
            delay = 0;
        }
    }
    delay
}
}

// ── `return delay;` · x86_machine.c:2543-2543 · chunk 4259 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return delay;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4259 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pc_machine_get_sleep_duration__part4259(delay: c_int) -> c_int {
    delay
}
}

// ── `pc_machine_interp` · x86_machine.c:2546-2557 · chunk 4260 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void pc_machine_interp(VirtMachine *s1, int max_exec_cycles)
//     {
//         PCMachine *s = (PCMachine *)s1;
//     
//         if (s->kvm_enabled) {
//             kvm_exec(s);
//         } else
//     
//         {
//             x86_cpu_interp(s->cpu_state, max_exec_cycles);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4260 {
// Rust rewrite of pc_machine_interp() from x86_machine.c
//
// The C bodies of kvm_exec / kvm_exit_io / kvm_exit_mmio / st_port / ld_port are
// static (no linker symbol), so their logic is ported here as private fns.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_unsafe)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const ITIMER_REAL: c_int = 0;
const KVM_RUN: c_ulong = 0xAE80; // _IO(KVMIO, 0x80)
const EINTR: c_int = 4;
const EAGAIN: c_int = 11;

const KVM_EXIT_IO: u32 = 2;
const KVM_EXIT_SHUTDOWN: u32 = 5;
const KVM_EXIT_MMIO: u32 = 6;
const KVM_EXIT_FAIL_ENTRY: u32 = 9;
const KVM_EXIT_INTERNAL_ERROR: u32 = 17;

// ---------------------------------------------------------------- opaque types

#[repr(C)]
pub struct VirtMachineClass {
    _private: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct CharacterDevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct PIC2State {
    _private: [u8; 0],
}
#[repr(C)]
pub struct PITState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct I440FXState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct CMOSState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct SerialState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct KBDState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct VMMouseState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct PS2KbdState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

// --------------------------------------------------------------- data structs

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

type SetIRQFunc = extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

type DeviceWriteFunc = extern "C" fn(*mut c_void, u32, u32, c_int);
type DeviceReadFunc = extern "C" fn(*mut c_void, u32, c_int) -> u32;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmHw {
    pub hardware_exit_reason: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmFailEntry {
    pub hardware_entry_failure_reason: u64,
    pub cpu: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmEx {
    pub exception: u32,
    pub error_code: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmIo {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmMmio {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmInternal {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16],
}

#[repr(C)]
pub union KvmRunUnion {
    pub hw: KvmHw,
    pub fail_entry: KvmFailEntry,
    pub ex: KvmEx,
    pub io: KvmIo,
    pub mmio: KvmMmio,
    pub internal: KvmInternal,
    pub padding: [u8; 256],
}

#[repr(C)]
pub struct KvmRun {
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,
    pub padding1: [u8; 6],
    pub exit_reason: u32,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,
    pub cr8: u64,
    pub apic_base: u64,
    pub u: KvmRunUnion,
}

#[repr(C)]
pub struct PCMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,

    pub cpu_state: *mut X86CPUState,
    pub pic_state: *mut PIC2State,
    pub pic_irq: [IRQSignal; 16],
    pub pit_state: *mut PITState,
    pub i440fx_state: *mut I440FXState,
    pub cmos_state: *mut CMOSState,
    pub serial_state: *mut SerialState,

    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub kbd_state: *mut KBDState,
    pub ps2_mouse: *mut PS2MouseState,
    pub vm_mouse: *mut VMMouseState,
    pub ps2_kbd: *mut PS2KbdState,

    pub kvm_enabled: c_int, // BOOL
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct Itimerval {
    pub it_interval: Timeval,
    pub it_value: Timeval,
}

// ------------------------------------------------------------ C dependencies

extern "C" {
    fn x86_cpu_interp(s: *mut X86CPUState, max_cycles1: c_int);
    fn get_phys_mem_range(s: *mut PhysMemoryMap, paddr: u64) -> *mut PhysMemoryRange;

    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn setitimer(which: c_int, new_value: *const Itimerval, old_value: *mut Itimerval) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn abort() -> !;
    fn __errno_location() -> *mut c_int;

    static mut stderr: *mut c_void;
}

// --------------------------------------------------------------- ported statics

unsafe fn dev_write(pr: *mut PhysMemoryRange, addr: u64, val: u32, size_log2: c_int) {
    unsafe {
        if let Some(f) = (*pr).write_func {
            f((*pr).opaque, addr as u32, val, size_log2);
        }
    }
}

unsafe fn dev_read(pr: *mut PhysMemoryRange, addr: u64, size_log2: c_int) -> u32 {
    unsafe {
        match (*pr).read_func {
            Some(f) => f((*pr).opaque, addr as u32, size_log2),
            None => 0,
        }
    }
}

unsafe fn st_port(opaque: *mut c_void, port: u32, val: u32, size_log2: c_int) {
    unsafe {
        let s = opaque as *mut PCMachine;
        let pr = get_phys_mem_range((*s).port_map, port as u64);
        if pr.is_null() {
            return;
        }
        let port = (port as u64).wrapping_sub((*pr).addr) as u32;
        let flags = (*pr).devio_flags as u32;
        let sl = (size_log2 as u32) & 31;
        if ((flags >> sl) & 1) != 0 {
            dev_write(pr, port as u64, val, size_log2);
        } else if size_log2 == 1 && (flags & 1) != 0 {
            dev_write(pr, port as u64, val & 0xff, 0);
            dev_write(pr, port.wrapping_add(1) as u64, (val >> 8) & 0xff, 0);
        }
    }
}

unsafe fn ld_port(opaque: *mut c_void, port1: u32, size_log2: c_int) -> u32 {
    unsafe {
        let s = opaque as *mut PCMachine;
        let val: u32;
        let pr = get_phys_mem_range((*s).port_map, port1 as u64);
        if pr.is_null() {
            val = u32::MAX; // (uint32_t)-1
        } else {
            let port = (port1 as u64).wrapping_sub((*pr).addr) as u32;
            let flags = (*pr).devio_flags as u32;
            let sl = (size_log2 as u32) & 31;
            if ((flags >> sl) & 1) != 0 {
                val = dev_read(pr, port as u64, size_log2);
            } else if size_log2 == 1 && (flags & 1) != 0 {
                let mut v = dev_read(pr, port as u64, 0) & 0xff;
                v |= (dev_read(pr, port.wrapping_add(1) as u64, 0) & 0xff) << 8;
                val = v;
            } else {
                val = u32::MAX;
            }
        }
        val
    }
}

unsafe fn kvm_exit_io(s: *mut PCMachine, run: *mut KvmRun) {
    unsafe {
        let io = (*run).u.io;
        let mut ptr = (run as *mut u8).add(io.data_offset as usize);
        let mut i: u32 = 0;
        while i < io.count {
            if io.direction == 1 {
                match io.size {
                    1 => st_port(
                        s as *mut c_void,
                        io.port as u32,
                        core::ptr::read_unaligned(ptr) as u32,
                        0,
                    ),
                    2 => st_port(
                        s as *mut c_void,
                        io.port as u32,
                        core::ptr::read_unaligned(ptr as *const u16) as u32,
                        1,
                    ),
                    4 => st_port(
                        s as *mut c_void,
                        io.port as u32,
                        core::ptr::read_unaligned(ptr as *const u32),
                        2,
                    ),
                    _ => abort(),
                }
            } else {
                match io.size {
                    1 => core::ptr::write_unaligned(
                        ptr,
                        ld_port(s as *mut c_void, io.port as u32, 0) as u8,
                    ),
                    2 => core::ptr::write_unaligned(
                        ptr as *mut u16,
                        ld_port(s as *mut c_void, io.port as u32, 1) as u16,
                    ),
                    4 => core::ptr::write_unaligned(
                        ptr as *mut u32,
                        ld_port(s as *mut c_void, io.port as u32, 2),
                    ),
                    _ => abort(),
                }
            }
            ptr = ptr.add(io.size as usize);
            i = i.wrapping_add(1);
        }
    }
}

unsafe fn kvm_exit_mmio(s: *mut PCMachine, run: *mut KvmRun) {
    unsafe {
        let phys_addr = (*run).u.mmio.phys_addr;
        let len = (*run).u.mmio.len;
        let is_write = (*run).u.mmio.is_write;
        let data = core::ptr::addr_of_mut!((*run).u.mmio.data) as *mut u8;
        let pr = get_phys_mem_range((*s).mem_map, phys_addr);
        let mut no_dev = false;

        if is_write != 0 {
            if pr.is_null() || (*pr).is_ram != 0 {
                return;
            }
            let addr = phys_addr.wrapping_sub((*pr).addr);
            match len {
                1 => {
                    if ((*pr).devio_flags & (1 << 0)) != 0 {
                        dev_write(pr, addr, core::ptr::read_unaligned(data) as u32, 0);
                    }
                }
                2 => {
                    if ((*pr).devio_flags & (1 << 1)) != 0 {
                        dev_write(pr, addr, core::ptr::read_unaligned(data as *const u16) as u32, 1);
                    }
                }
                4 => {
                    if ((*pr).devio_flags & (1 << 2)) != 0 {
                        dev_write(pr, addr, core::ptr::read_unaligned(data as *const u32), 2);
                    }
                }
                8 => {
                    if ((*pr).devio_flags & (1 << 2)) != 0 {
                        dev_write(pr, addr, core::ptr::read_unaligned(data as *const u32), 2);
                        dev_write(
                            pr,
                            addr.wrapping_add(4),
                            core::ptr::read_unaligned(data.add(4) as *const u32),
                            2,
                        );
                    }
                }
                _ => abort(),
            }
        } else {
            if pr.is_null() || (*pr).is_ram != 0 {
                no_dev = true;
            } else {
                let addr = phys_addr.wrapping_sub((*pr).addr);
                match len {
                    1 => {
                        if ((*pr).devio_flags & (1 << 0)) == 0 {
                            no_dev = true;
                        } else {
                            core::ptr::write_unaligned(data, dev_read(pr, addr, 0) as u8);
                        }
                    }
                    2 => {
                        if ((*pr).devio_flags & (1 << 1)) == 0 {
                            no_dev = true;
                        } else {
                            core::ptr::write_unaligned(
                                data as *mut u16,
                                dev_read(pr, addr, 1) as u16,
                            );
                        }
                    }
                    4 => {
                        if ((*pr).devio_flags & (1 << 2)) == 0 {
                            no_dev = true;
                        } else {
                            core::ptr::write_unaligned(data as *mut u32, dev_read(pr, addr, 2));
                        }
                    }
                    8 => {
                        if ((*pr).devio_flags & (1 << 2)) == 0 {
                            no_dev = true;
                        } else {
                            core::ptr::write_unaligned(data as *mut u32, dev_read(pr, addr, 2));
                            core::ptr::write_unaligned(
                                data.add(4) as *mut u32,
                                dev_read(pr, addr.wrapping_add(4), 2),
                            );
                        }
                    }
                    _ => abort(),
                }
            }
        }

        if no_dev {
            // no_dev: memset(run->mmio.data, 0, run->mmio.len);
            core::ptr::write_bytes(data, 0, len as usize);
        }
    }
}

unsafe fn kvm_exec(s: *mut PCMachine) {
    unsafe {
        let run = (*s).kvm_run;
        let ival = Itimerval {
            it_interval: Timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            it_value: Timeval {
                tv_sec: 0,
                tv_usec: 10 * 1000,
            },
        };

        setitimer(ITIMER_REAL, &ival, core::ptr::null_mut());

        let ret = ioctl((*s).vcpu_fd, KVM_RUN, 0 as c_int);
        if ret < 0 {
            let errno = *__errno_location();
            if errno == EINTR || errno == EAGAIN {
                return;
            }
            perror(b"KVM_RUN\0".as_ptr() as *const c_char);
            exit(1);
        }

        let exit_reason = (*run).exit_reason;
        match exit_reason {
            KVM_EXIT_SHUTDOWN => {}
            KVM_EXIT_IO => kvm_exit_io(s, run),
            KVM_EXIT_MMIO => kvm_exit_mmio(s, run),
            KVM_EXIT_FAIL_ENTRY => {
                fprintf(
                    stderr,
                    b"KVM_EXIT_FAIL_ENTRY: reason=0x%lx\n".as_ptr() as *const c_char,
                    (*run).u.fail_entry.hardware_entry_failure_reason,
                );
                exit(1);
            }
            KVM_EXIT_INTERNAL_ERROR => {
                fprintf(
                    stderr,
                    b"KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n".as_ptr() as *const c_char,
                    (*run).u.internal.suberror,
                );
                exit(1);
            }
            _ => {
                fprintf(
                    stderr,
                    b"KVM: unsupported exit_reason=%d\n".as_ptr() as *const c_char,
                    exit_reason,
                );
                exit(1);
            }
        }
    }
}

// --------------------------------------------------------------------- target

#[no_mangle]
pub extern "C" fn pc_machine_interp(s1: *mut VirtMachine, max_exec_cycles: c_int) {
    unsafe {
        let s = s1 as *mut PCMachine;
        if (*s).kvm_enabled != 0 {
            kvm_exec(s);
        } else {
            x86_cpu_interp((*s).cpu_state, max_exec_cycles);
        }
    }
}
}

// ── `if (s->kvm_enabled) … else` · x86_machine.c:2550-2556 · chunk 4261 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (s->kvm_enabled) {
//             kvm_exec(s);
//         } else
//     
//         {
//             x86_cpu_interp(s->cpu_state, max_exec_cycles);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk4261 {
// Port of the pc_machine_interp dispatch (KVM run vs. x86 interpreter).
// kvm_exec / kvm_exit_io / kvm_exit_mmio / st_port / ld_port are static in the
// C translation unit, so their logic is reproduced here as private Rust fns.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const ITIMER_REAL: c_int = 0;
const KVM_RUN: c_ulong = 0xAE80;

/* ---------------- mirrored C types ---------------- */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IrqSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

/* struct kvm_run prefix + the members of its exit union that are used */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmExitIo {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmExitMmio {
    pub phys_addr: u64,
    pub data: [u8; 8],
    pub len: u32,
    pub is_write: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmExitFailEntry {
    pub hardware_entry_failure_reason: u64,
    pub cpu: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KvmExitInternal {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union KvmRunExit {
    pub io: KvmExitIo,
    pub mmio: KvmExitMmio,
    pub fail_entry: KvmExitFailEntry,
    pub internal: KvmExitInternal,
    pub pad: [u8; 256],
}

#[repr(C)]
pub struct KvmRun {
    pub request_interrupt_window: u8,
    pub immediate_exit: u8,
    pub padding1: [u8; 6],
    pub exit_reason: u32,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,
    pub cr8: u64,
    pub apic_base: u64,
    pub u: KvmRunExit,
}

#[repr(C)]
pub struct PcMachine {
    pub common: VirtMachine,
    pub ram_size: u64,
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub cpu_state: *mut c_void,
    pub pic_state: *mut c_void,
    pub pic_irq: [IrqSignal; 16],
    pub pit_state: *mut c_void,
    pub i440fx_state: *mut c_void,
    pub cmos_state: *mut c_void,
    pub serial_state: *mut c_void,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub kbd_state: *mut c_void,
    pub ps2_mouse: *mut c_void,
    pub vm_mouse: *mut c_void,
    pub ps2_kbd: *mut c_void,
    pub kvm_enabled: c_int,
    pub kvm_fd: c_int,
    pub vm_fd: c_int,
    pub vcpu_fd: c_int,
    pub kvm_run_size: c_int,
    pub kvm_run: *mut KvmRun,
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct Itimerval {
    pub it_interval: Timeval,
    pub it_value: Timeval,
}

/* ---------------- external C symbols ---------------- */

unsafe extern "C" {
    static stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn abort() -> !;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn __errno_location() -> *mut c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn setitimer(which: c_int, new_value: *const Itimerval, old_value: *mut Itimerval) -> c_int;
    fn get_phys_mem_range(map: *mut c_void, paddr: u64) -> *mut PhysMemoryRange;
    fn x86_cpu_interp(s: *mut c_void, max_cycles1: c_int);
}

/* ---------------- device callback helpers ---------------- */

#[inline]
unsafe fn call_write(
    f: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    if let Some(func) = f {
        unsafe { func(opaque, offset, val, size_log2) };
    }
}

#[inline]
unsafe fn call_read(
    f: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    opaque: *mut c_void,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    match f {
        Some(func) => unsafe { func(opaque, offset, size_log2) },
        None => 0,
    }
}

/* ---------------- port I/O (static st_port / ld_port) ---------------- */

unsafe fn st_port(s: *mut PcMachine, port_arg: u32, val: u32, size_log2: c_int) {
    let pr = unsafe { get_phys_mem_range((*s).port_map, port_arg as u64) };
    if pr.is_null() {
        return;
    }
    let port: u32 = (port_arg as u64).wrapping_sub(unsafe { (*pr).addr }) as u32;
    let devio_flags = unsafe { (*pr).devio_flags };
    let opaque = unsafe { (*pr).opaque };
    let wf = unsafe { (*pr).write_func };
    if ((devio_flags >> ((size_log2 as u32) & 31)) & 1) != 0 {
        unsafe { call_write(wf, opaque, port, val, size_log2) };
    } else if size_log2 == 1 && (devio_flags & (1 << 0)) != 0 {
        unsafe { call_write(wf, opaque, port, val & 0xff, 0) };
        unsafe { call_write(wf, opaque, port.wrapping_add(1), (val >> 8) & 0xff, 0) };
    }
}

unsafe fn ld_port(s: *mut PcMachine, port_arg: u32, size_log2: c_int) -> u32 {
    let pr = unsafe { get_phys_mem_range((*s).port_map, port_arg as u64) };
    if pr.is_null() {
        return 0xffff_ffff;
    }
    let port: u32 = (port_arg as u64).wrapping_sub(unsafe { (*pr).addr }) as u32;
    let devio_flags = unsafe { (*pr).devio_flags };
    let opaque = unsafe { (*pr).opaque };
    let rf = unsafe { (*pr).read_func };
    if ((devio_flags >> ((size_log2 as u32) & 31)) & 1) != 0 {
        unsafe { call_read(rf, opaque, port, size_log2) }
    } else if size_log2 == 1 && (devio_flags & (1 << 0)) != 0 {
        let mut val: u32 = unsafe { call_read(rf, opaque, port, 0) } & 0xff;
        val |= (unsafe { call_read(rf, opaque, port.wrapping_add(1), 0) } & 0xff) << 8;
        val
    } else {
        0xffff_ffff
    }
}

/* ---------------- static kvm_exit_io / kvm_exit_mmio ---------------- */

unsafe fn kvm_exit_io(s: *mut PcMachine, run: *mut KvmRun) {
    let io = unsafe { (*run).u.io };
    let data_offset = io.data_offset;
    let direction = io.direction;
    let size = io.size;
    let port: u32 = io.port as u32;
    let count = io.count;

    let mut ptr: *mut u8 = (run as *mut u8).wrapping_add(data_offset as usize);
    let mut i: u32 = 0;
    while i < count {
        if direction == 1 {
            match size {
                1 => {
                    let v = unsafe { core::ptr::read_unaligned(ptr) } as u32;
                    unsafe { st_port(s, port, v, 0) };
                }
                2 => {
                    let v = unsafe { core::ptr::read_unaligned(ptr as *const u16) } as u32;
                    unsafe { st_port(s, port, v, 1) };
                }
                4 => {
                    let v = unsafe { core::ptr::read_unaligned(ptr as *const u32) };
                    unsafe { st_port(s, port, v, 2) };
                }
                _ => unsafe { abort() },
            }
        } else {
            match size {
                1 => {
                    let v = unsafe { ld_port(s, port, 0) } as u8;
                    unsafe { core::ptr::write_unaligned(ptr, v) };
                }
                2 => {
                    let v = unsafe { ld_port(s, port, 1) } as u16;
                    unsafe { core::ptr::write_unaligned(ptr as *mut u16, v) };
                }
                4 => {
                    let v = unsafe { ld_port(s, port, 2) };
                    unsafe { core::ptr::write_unaligned(ptr as *mut u32, v) };
                }
                _ => unsafe { abort() },
            }
        }
        ptr = ptr.wrapping_add(size as usize);
        i = i.wrapping_add(1);
    }
}

unsafe fn kvm_exit_mmio(s: *mut PcMachine, run: *mut KvmRun) {
    let phys_addr: u64 = unsafe { (*run).u.mmio.phys_addr };
    let len: u32 = unsafe { (*run).u.mmio.len };
    let is_write: u8 = unsafe { (*run).u.mmio.is_write };
    let data: *mut u8 = unsafe { core::ptr::addr_of_mut!((*run).u.mmio.data) } as *mut u8;

    let pr = unsafe { get_phys_mem_range((*s).mem_map, phys_addr) };

    if is_write != 0 {
        if pr.is_null() || unsafe { (*pr).is_ram } != 0 {
            return;
        }
        let addr: u32 = phys_addr.wrapping_sub(unsafe { (*pr).addr }) as u32;
        let devio_flags = unsafe { (*pr).devio_flags };
        let opaque = unsafe { (*pr).opaque };
        let wf = unsafe { (*pr).write_func };
        match len {
            1 => {
                if (devio_flags & (1 << 0)) != 0 {
                    let v = unsafe { core::ptr::read_unaligned(data) } as u32;
                    unsafe { call_write(wf, opaque, addr, v, 0) };
                }
            }
            2 => {
                if (devio_flags & (1 << 1)) != 0 {
                    let v = unsafe { core::ptr::read_unaligned(data as *const u16) } as u32;
                    unsafe { call_write(wf, opaque, addr, v, 1) };
                }
            }
            4 => {
                if (devio_flags & (1 << 2)) != 0 {
                    let v = unsafe { core::ptr::read_unaligned(data as *const u32) };
                    unsafe { call_write(wf, opaque, addr, v, 2) };
                }
            }
            8 => {
                if (devio_flags & (1 << 2)) != 0 {
                    let v0 = unsafe { core::ptr::read_unaligned(data as *const u32) };
                    let v1 = unsafe { core::ptr::read_unaligned(data.wrapping_add(4) as *const u32) };
                    unsafe { call_write(wf, opaque, addr, v0, 2) };
                    unsafe { call_write(wf, opaque, addr.wrapping_add(4), v1, 2) };
                }
            }
            _ => unsafe { abort() },
        }
    } else {
        if pr.is_null() || unsafe { (*pr).is_ram } != 0 {
            unsafe { memset(data as *mut c_void, 0, len as usize) };
            return;
        }
        let addr: u32 = phys_addr.wrapping_sub(unsafe { (*pr).addr }) as u32;
        let devio_flags = unsafe { (*pr).devio_flags };
        let opaque = unsafe { (*pr).opaque };
        let rf = unsafe { (*pr).read_func };
        match len {
            1 => {
                if (devio_flags & (1 << 0)) == 0 {
                    unsafe { memset(data as *mut c_void, 0, len as usize) };
                } else {
                    let v = unsafe { call_read(rf, opaque, addr, 0) } as u8;
                    unsafe { core::ptr::write_unaligned(data, v) };
                }
            }
            2 => {
                if (devio_flags & (1 << 1)) == 0 {
                    unsafe { memset(data as *mut c_void, 0, len as usize) };
                } else {
                    let v = unsafe { call_read(rf, opaque, addr, 1) } as u16;
                    unsafe { core::ptr::write_unaligned(data as *mut u16, v) };
                }
            }
            4 => {
                if (devio_flags & (1 << 2)) == 0 {
                    unsafe { memset(data as *mut c_void, 0, len as usize) };
                } else {
                    let v = unsafe { call_read(rf, opaque, addr, 2) };
                    unsafe { core::ptr::write_unaligned(data as *mut u32, v) };
                }
            }
            8 => {
                if (devio_flags & (1 << 2)) == 0 {
                    unsafe { memset(data as *mut c_void, 0, len as usize) };
                } else {
                    let v0 = unsafe { call_read(rf, opaque, addr, 2) };
                    let v1 = unsafe { call_read(rf, opaque, addr.wrapping_add(4), 2) };
                    unsafe { core::ptr::write_unaligned(data as *mut u32, v0) };
                    unsafe { core::ptr::write_unaligned(data.wrapping_add(4) as *mut u32, v1) };
                }
            }
            _ => unsafe { abort() },
        }
    }
}

/* ---------------- static kvm_exec ---------------- */

unsafe fn kvm_exec(s: *mut PcMachine) {
    let run: *mut KvmRun = unsafe { (*s).kvm_run };

    let ival = Itimerval {
        it_interval: Timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: Timeval {
            tv_sec: 0,
            tv_usec: 10 * 1000,
        },
    };
    unsafe { setitimer(ITIMER_REAL, &ival, core::ptr::null_mut()) };

    let ret = unsafe { ioctl((*s).vcpu_fd, KVM_RUN, 0 as c_int) };
    if ret < 0 {
        let errno = unsafe { *__errno_location() };
        if errno == 4 || errno == 11 {
            return;
        }
        unsafe { perror(b"KVM_RUN\0".as_ptr() as *const c_char) };
        unsafe { exit(1) };
    }

    let exit_reason: u32 = unsafe { (*run).exit_reason };
    match exit_reason {
        5 => {}
        2 => unsafe { kvm_exit_io(s, run) },
        6 => unsafe { kvm_exit_mmio(s, run) },
        9 => {
            let reason: u64 = unsafe { (*run).u.fail_entry.hardware_entry_failure_reason };
            let stream = unsafe { stderr };
            unsafe {
                fprintf(
                    stream,
                    b"KVM_EXIT_FAIL_ENTRY: reason=0x%lx\n\0".as_ptr() as *const c_char,
                    reason as c_ulong,
                )
            };
            unsafe { exit(1) };
        }
        17 => {
            let suberror: u32 = unsafe { (*run).u.internal.suberror };
            let stream = unsafe { stderr };
            unsafe {
                fprintf(
                    stream,
                    b"KVM_EXIT_INTERNAL_ERROR: suberror=0x%x\n\0".as_ptr() as *const c_char,
                    suberror,
                )
            };
            unsafe { exit(1) };
        }
        _ => {
            let stream = unsafe { stderr };
            unsafe {
                fprintf(
                    stream,
                    b"KVM: unsupported exit_reason=%d\n\0".as_ptr() as *const c_char,
                    exit_reason,
                )
            };
            unsafe { exit(1) };
        }
    }
}

/* ---------------- exported piece ---------------- */

#[no_mangle]
pub unsafe extern "C" fn pc_machine_interp__part4261(s: *mut PcMachine, max_exec_cycles: c_int) {
    if unsafe { (*s).kvm_enabled } != 0 {
        unsafe { kvm_exec(s) };
    } else {
        unsafe { x86_cpu_interp((*s).cpu_state, max_exec_cycles) };
    }
}
}

