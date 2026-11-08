//! From `riscv_machine.c`.

// ── `rtc_get_real_time` · riscv_machine.c:83-89 · chunk 1961 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t rtc_get_real_time(RISCVMachine *s)
//     {
//         struct timespec ts;
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 10000000 +
//             (ts.tv_nsec / (1000000000 / 10000000));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1961 {
// Rewrite of rtc_get_real_time (riscv_machine.c) in Rust.

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,  // __time_t  (long int)
    pub tv_nsec: i64, // __syscall_slong_t (long int)
}

extern "C" {
    fn clock_gettime(clock_id: i32, tp: *mut Timespec) -> i32;
}

const CLOCK_MONOTONIC: i32 = 1;
const RTC_FREQ: u64 = 10000000;

#[no_mangle]
pub unsafe extern "C" fn rtc_get_real_time(_s: *mut core::ffi::c_void) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);
    (ts.tv_sec as u64)
        .wrapping_mul(RTC_FREQ)
        .wrapping_add((ts.tv_nsec / (1000000000i64 / RTC_FREQ as i64)) as u64)
}
}

// ── `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 10000000 + (ts.tv_nse…` · riscv_machine.c:85-88 · chunk 1962 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timespec ts;
//         clock_gettime(
//                      1
//                                     , &ts);
//         return (uint64_t)ts.tv_sec * 10000000 +
//             (ts.tv_nsec / (1000000000 / 10000000));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1962 {
// Port of rtc_get_real_time's body (CLOCK_MONOTONIC based real time in 10 MHz ticks).
use core::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;
const RTC_FREQ: u64 = 10000000;

#[no_mangle]
pub extern "C" fn rtc_get_real_time__part1962() -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }
    let nsec: c_long = ts.tv_nsec / (1000000000 / (RTC_FREQ as c_long));
    (ts.tv_sec as u64)
        .wrapping_mul(RTC_FREQ)
        .wrapping_add(nsec as u64)
}
}

// ── `rtc_get_time` · riscv_machine.c:91-101 · chunk 1963 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint64_t rtc_get_time(RISCVMachine *m)
//     {
//         uint64_t val;
//         if (m->rtc_real_time) {
//             val = rtc_get_real_time(m) - m->rtc_start_time;
//         } else {
//             val = riscv_cpu_get_cycles(m->cpu_state) / 16;
//         }
//     
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1963 {
// Port of the static C function rtc_get_time() from riscv_machine.c.
//
// The helpers it used in C (rtc_get_real_time, riscv_cpu_get_cycles) are
// `static`/`static inline` in the translation unit, so their logic is ported
// here as private Rust functions instead of being referenced by symbol.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;
const RTC_FREQ: u64 = 10000000;

// Prefix of `typedef struct VirtMachine { ... } VirtMachine;`
#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct RiscVCPUState {
    _opaque: [u8; 0],
}

#[repr(C)]
struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
struct RiscVCPUClass {
    riscv_cpu_init: Option<unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RiscVCPUState>,
    riscv_cpu_end: Option<unsafe extern "C" fn(*mut RiscVCPUState)>,
    riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RiscVCPUState, c_int)>,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RiscVCPUState) -> u64>,
}

#[repr(C)]
struct RiscVCPUCommonState {
    class_ptr: *const RiscVCPUClass,
}

// Prefix of `typedef struct RISCVMachine { ... } RISCVMachine;`
#[repr(C)]
struct RiscVMachine {
    common: VirtMachine,
    mem_map: *mut PhysMemoryMap,
    max_xlen: c_int,
    cpu_state: *mut RiscVCPUState,
    ram_size: u64,
    rtc_real_time: c_int, // BOOL == int
    rtc_start_time: u64,
}

unsafe fn rtc_get_real_time(_s: *mut RiscVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as u64).wrapping_mul(RTC_FREQ) + ((ts.tv_nsec as u64) / (1000000000u64 / RTC_FREQ))
}

unsafe fn riscv_cpu_get_cycles(s: *mut RiscVCPUState) -> u64 {
    let c = (*s.cast::<RiscVCPUCommonState>()).class_ptr;
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rtc_get_time(m: *mut RiscVMachine) -> u64 {
    let val: u64;
    if (*m).rtc_real_time != 0 {
        val = rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time);
    } else {
        val = riscv_cpu_get_cycles((*m).cpu_state) / 16;
    }
    val
}
}

// ── `if (m->rtc_real_time) … else` · riscv_machine.c:94-98 · chunk 1964 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->rtc_real_time) {
//             val = rtc_get_real_time(m) - m->rtc_start_time;
//         } else {
//             val = riscv_cpu_get_cycles(m->cpu_state) / 16;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1964 {
// Port of the RTC value computation inside rtc_get_time().

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long};

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
}

#[repr(C)]
struct RISCVCPUState {
    _private: [u8; 0],
}

type GetCyclesFn = unsafe extern "C" fn(s: *mut RISCVCPUState) -> u64;

#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: *const core::ffi::c_void,
    riscv_cpu_end: *const core::ffi::c_void,
    riscv_cpu_interp: *const core::ffi::c_void,
    riscv_cpu_get_cycles: Option<GetCyclesFn>,
    riscv_cpu_set_mip: *const core::ffi::c_void,
    riscv_cpu_reset_mip: *const core::ffi::c_void,
    riscv_cpu_get_mip: *const core::ffi::c_void,
    riscv_cpu_get_power_down: *const core::ffi::c_void,
    riscv_cpu_get_misa: *const core::ffi::c_void,
    riscv_cpu_flush_tlb_write_range_ram: *const core::ffi::c_void,
}

#[repr(C)]
struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

// Ported from the static C function rtc_get_real_time().
fn rtc_get_real_time_local() -> u64 {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(1 /* CLOCK_REALTIME */, &mut ts);
    }
    (ts.tv_sec as u64)
        .wrapping_mul(10000000)
        .wrapping_add((ts.tv_nsec / (1000000000 / 10000000)) as u64)
}

// Ported from the static inline C function riscv_cpu_get_cycles().
unsafe fn riscv_cpu_get_cycles_local(s: *mut RISCVCPUState) -> u64 {
    let c = (*(s as *const RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0u64,
    }
}

#[no_mangle]
pub extern "C" fn rtc_get_time__part1964(
    rtc_real_time: c_int,
    rtc_start_time: u64,
    cpu_state: *mut RISCVCPUState,
) -> u64 {
    if rtc_real_time != 0 {
        rtc_get_real_time_local().wrapping_sub(rtc_start_time)
    } else {
        unsafe { riscv_cpu_get_cycles_local(cpu_state) / 16 }
    }
}
}

// ── `return val;` · riscv_machine.c:100-100 · chunk 1965 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1965 {
// Port of the `return val;` statement of rtc_get_time.
// The TARGET simply returns the already computed 64-bit value.

#[no_mangle]
pub extern "C" fn rtc_get_time__part1965(val: u64) -> u64 {
    val
}
}

// ── `htif_read` · riscv_machine.c:103-128 · chunk 1966 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t htif_read(void *opaque, uint32_t offset,
//                               int size_log2)
//     {
//         RISCVMachine *s = opaque;
//         uint32_t val;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 109, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0:
//             val = s->htif_tohost;
//             break;
//         case 4:
//             val = s->htif_tohost >> 32;
//             break;
//         case 8:
//             val = s->htif_fromhost;
//             break;
//         case 12:
//             val = s->htif_fromhost >> 32;
//             break;
//         default:
//             val = 0;
//             break;
//         }
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1966 {
// Port of the static C function htif_read() from riscv_machine.c
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

// Layout mirrors the C `typedef struct` IRQSignal.
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Layout mirrors the C `typedef struct VirtMachine`.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Layout mirrors the C `typedef struct RISCVMachine` exactly.
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    fn abort() -> !;
}

#[no_mangle]
pub extern "C" fn htif_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    if size_log2 != 2 {
        unsafe { abort() }
    }
    let s = opaque as *const RISCVMachine;
    let tohost = unsafe { (*s).htif_tohost };
    let fromhost = unsafe { (*s).htif_fromhost };
    let val: u32 = match offset {
        0 => tohost as u32,
        4 => (tohost >> 32) as u32,
        8 => fromhost as u32,
        12 => (fromhost >> 32) as u32,
        _ => 0,
    };
    val
}
}

// ── `RISCVMachine *s = opaque; uint32_t val; ((void) sizeof (__assert_single_arg ( size_log2 == 2 ))…` · riscv_machine.c:106-109 · chunk 1967 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//         uint32_t val;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 109, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1967 {
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn htif_read__part1967(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                b"size_log2 == 2\0".as_ptr() as *const c_char,
                b"riscv_machine.c\0".as_ptr() as *const c_char,
                109u32,
                b"htif_read\0".as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:110-126 · chunk 1968 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0:
//             val = s->htif_tohost;
//             break;
//         case 4:
//             val = s->htif_tohost >> 32;
//             break;
//         case 8:
//             val = s->htif_fromhost;
//             break;
//         case 12:
//             val = s->htif_fromhost >> 32;
//             break;
//         default:
//             val = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1968 {
// Rust rewrite of the switch inside htif_read (riscv_machine.c).
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,          // const VirtMachineClass *
    pub net: *mut c_void,            // EthernetDevice *
    pub console_dev: *mut c_void,    // VIRTIODevice *
    pub console: *mut c_void,        // CharacterDevice *
    pub fb_dev: *mut c_void,         // FBDevice *
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,        // PhysMemoryMap *
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,      // RISCVCPUState *
    pub ram_size: u64,
    pub rtc_real_time: c_int,        // BOOL
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,   // VIRTIODevice *
    pub mouse_dev: *mut c_void,      // VIRTIODevice *
    pub virtio_count: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn htif_read__part1968(s: *mut RISCVMachine, offset: u32) -> u32 {
    let tohost = (*s).htif_tohost;
    let fromhost = (*s).htif_fromhost;
    match offset {
        0 => tohost as u32,
        4 => (tohost >> 32) as u32,
        8 => fromhost as u32,
        12 => (fromhost >> 32) as u32,
        _ => 0u32,
    }
}
}

// ── `return val;` · riscv_machine.c:127-127 · chunk 1969 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1969 {
// Tail of htif_read: return the computed register value.
#[no_mangle]
pub extern "C" fn htif_read__part1969(val: u32) -> u32 {
    val
}
}

// ── `htif_handle_cmd` · riscv_machine.c:130-152 · chunk 1970 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void htif_handle_cmd(RISCVMachine *s)
//     {
//         uint32_t device, cmd;
//     
//         device = s->htif_tohost >> 56;
//         cmd = (s->htif_tohost >> 48) & 0xff;
//         if (s->htif_tohost == 1) {
//     
//             printf("\nPower off.\n");
//             exit(0);
//         } else if (device == 1 && cmd == 1) {
//             uint8_t buf[1];
//             buf[0] = s->htif_tohost & 0xff;
//             s->common.console->write_data(s->common.console->opaque, buf, 1);
//             s->htif_tohost = 0;
//             s->htif_fromhost = ((uint64_t)device << 56) | ((uint64_t)cmd << 48);
//         } else if (device == 1 && cmd == 0) {
//     
//             s->htif_tohost = 0;
//         } else {
//             printf("HTIF: unsupported tohost=0x%016" 
//                                                     "l" "x" 
//                                                            "\n", s->htif_tohost);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1970 {
// Port of `htif_handle_cmd` from riscv_machine.c (originally `static`).
// The C declaration is `static void htif_handle_cmd(RISCVMachine *s)`; the
// kernel symbol keeps the same name with external linkage.

use core::ffi::c_void;

#[allow(non_camel_case_types)]
type BOOL = i32;

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<
        unsafe extern "C" fn(opaque: *mut c_void, buf: *const u8, len: i32),
    >,
    pub read_data: Option<
        unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, len: i32) -> i32,
    >,
}

#[repr(C)]
pub struct SetIRQFuncOpaque {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    // void (*set_irq)(void *opaque, int irq_num, int level);
    pub set_irq: Option<
        unsafe extern "C" fn(
            opaque: *mut c_void,
            irq_num: i32,
            level: i32,
        ),
    >,
    pub opaque: *mut c_void,
    pub irq_num: i32,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,        // const VirtMachineClass *
    pub net: *mut c_void,          // EthernetDevice *
    pub console_dev: *mut c_void,  // VIRTIODevice *
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,       // FBDevice *
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,      // PhysMemoryMap *
    pub max_xlen: i32,
    pub cpu_state: *mut c_void,    // RISCVCPUState *
    pub ram_size: u64,

    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,

    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],

    pub htif_tohost: u64,
    pub htif_fromhost: u64,

    pub keyboard_dev: *mut c_void, // VIRTIODevice *
    pub mouse_dev: *mut c_void,    // VIRTIODevice *

    pub virtio_count: i32,
}

extern "C" {
    fn printf(format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn exit(status: core::ffi::c_int) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn htif_handle_cmd(s: *mut RISCVMachine) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;

    let device: u32 = (s.htif_tohost >> 56) as u32;
    let cmd: u32 = ((s.htif_tohost >> 48) & 0xff) as u32;

    if s.htif_tohost == 1 {
        // shuthost
        printf(b"\nPower off.\n\0".as_ptr() as *const core::ffi::c_char);
        exit(0);
    } else if device == 1 && cmd == 1 {
        let mut buf: [u8; 1] = [0u8; 1];
        buf[0] = (s.htif_tohost & 0xff) as u8;
        let console = s.common.console;
        if !console.is_null() {
            let dev = &mut *console;
            if let Some(write_data) = dev.write_data {
                write_data(dev.opaque, buf.as_ptr(), 1);
            }
        }
        s.htif_tohost = 0;
        s.htif_fromhost =
            ((device as u64) << 56) | ((cmd as u64) << 48);
    } else if device == 1 && cmd == 0 {
        // request keyboard interrupt
        s.htif_tohost = 0;
    } else {
        printf(
            b"HTIF: unsupported tohost=0x%016lx\n\0".as_ptr() as *const core::ffi::c_char,
            s.htif_tohost,
        );
    }
}
}

// ── `uint32_t device, cmd; device = s->htif_tohost >> 56; cmd = (s->htif_tohost >> 48) & 0xff;` · riscv_machine.c:132-135 · chunk 1971 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t device, cmd;
//     
//         device = s->htif_tohost >> 56;
//         cmd = (s->htif_tohost >> 48) & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1971 {
// Ported piece of htif_handle_cmd: decode device and cmd from htif_tohost.
#[no_mangle]
pub extern "C" fn htif_handle_cmd__part1971(tohost: u64, device: *mut u32, cmd: *mut u32) {
    unsafe {
        if !device.is_null() {
            *device = (tohost >> 56) as u32;
        }
        if !cmd.is_null() {
            *cmd = ((tohost >> 48) & 0xff) as u32;
        }
    }
}
}

// ── `if (s->htif_tohost == 1) … else` · riscv_machine.c:136-151 · chunk 1972 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->htif_tohost == 1) {
//     
//             printf("\nPower off.\n");
//             exit(0);
//         } else if (device == 1 && cmd == 1) {
//             uint8_t buf[1];
//             buf[0] = s->htif_tohost & 0xff;
//             s->common.console->write_data(s->common.console->opaque, buf, 1);
//             s->htif_tohost = 0;
//             s->htif_fromhost = ((uint64_t)device << 56) | ((uint64_t)cmd << 48);
//         } else if (device == 1 && cmd == 0) {
//     
//             s->htif_tohost = 0;
//         } else {
//             printf("HTIF: unsupported tohost=0x%016" 
//                                                     "l" "x" 
//                                                            "\n", s->htif_tohost);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1972 {
// Port of htif_handle_cmd's command dispatch (riscv_machine.c lines 136-151).
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(code: c_int) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn htif_handle_cmd__part1972(
    s: *mut RISCVMachine,
    device: u32,
    cmd: u32,
) {
    if (*s).htif_tohost == 1 {
        printf(b"\nPower off.\n\0".as_ptr() as *const c_char);
        exit(0);
    } else if device == 1 && cmd == 1 {
        let buf: [u8; 1] = [((*s).htif_tohost & 0xff) as u8];
        let console = (*s).common.console;
        let write_data = (*console).write_data;
        if let Some(write) = write_data {
            write((*console).opaque, buf.as_ptr(), 1);
        }
        (*s).htif_tohost = 0;
        (*s).htif_fromhost =
            ((device as u64) << 56) | ((cmd as u64) << 48);
    } else if device == 1 && cmd == 0 {
        (*s).htif_tohost = 0;
    } else {
        printf(
            b"HTIF: unsupported tohost=0x%016lx\n\0".as_ptr() as *const c_char,
            (*s).htif_tohost,
        );
    }
}
}

// ── `htif_write` · riscv_machine.c:154-178 · chunk 1973 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void htif_write(void *opaque, uint32_t offset, uint32_t val,
//                            int size_log2)
//     {
//         RISCVMachine *s = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 159, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0:
//             s->htif_tohost = (s->htif_tohost & ~0xffffffff) | val;
//             break;
//         case 4:
//             s->htif_tohost = (s->htif_tohost & 0xffffffff) | ((uint64_t)val << 32);
//             htif_handle_cmd(s);
//             break;
//         case 8:
//             s->htif_fromhost = (s->htif_fromhost & ~0xffffffff) | val;
//             break;
//         case 12:
//             s->htif_fromhost = (s->htif_fromhost & 0xffffffff) |
//                 (uint64_t)val << 32;
//             break;
//         default:
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1973 {
// Rust rewrite of the static C function `htif_write` (riscv_machine.c).
// `htif_handle_cmd` is a static C function with no linker symbol, so its
// logic is ported here as a private Rust function.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    ) -> !;
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
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
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,

    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,

    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],

    pub htif_tohost: u64,
    pub htif_fromhost: u64,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,

    pub virtio_count: c_int,
}

// Ported from the static C helper htif_handle_cmd().
unsafe fn htif_handle_cmd(s: *mut RISCVMachine) {
    let tohost = (*s).htif_tohost;
    let device = (tohost >> 56) as u32;
    let cmd = ((tohost >> 48) & 0xff) as u32;

    if tohost == 1 {
        // shuthost
        printf(b"\nPower off.\n\0".as_ptr() as *const c_char);
        exit(0);
    } else if device == 1 && cmd == 1 {
        let mut buf = [0u8; 1];
        buf[0] = (tohost & 0xff) as u8;
        let console = (*s).common.console;
        if !console.is_null() {
            if let Some(write_data) = (*console).write_data {
                write_data((*console).opaque, buf.as_ptr(), 1);
            }
        }
        (*s).htif_tohost = 0;
        (*s).htif_fromhost = ((device as u64) << 56) | ((cmd as u64) << 48);
    } else if device == 1 && cmd == 0 {
        // request keyboard interrupt
        (*s).htif_tohost = 0;
    } else {
        printf(
            b"HTIF: unsupported tohost=0x%016lx\n\0".as_ptr() as *const c_char,
            tohost as c_ulong,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn htif_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    if size_log2 != 2 {
        __assert_fail(
            b"size_log2 == 2\0".as_ptr() as *const c_char,
            b"riscv_machine.c\0".as_ptr() as *const c_char,
            159,
            b"htif_write\0".as_ptr() as *const c_char,
        );
    }

    let s = opaque as *mut RISCVMachine;

    match offset {
        0 => {
            (*s).htif_tohost = ((*s).htif_tohost & !0xffffffffu64) | (val as u64);
        }
        4 => {
            (*s).htif_tohost = ((*s).htif_tohost & 0xffffffffu64) | ((val as u64) << 32);
            htif_handle_cmd(s);
        }
        8 => {
            (*s).htif_fromhost = ((*s).htif_fromhost & !0xffffffffu64) | (val as u64);
        }
        12 => {
            (*s).htif_fromhost =
                ((*s).htif_fromhost & 0xffffffffu64) | ((val as u64) << 32);
        }
        _ => {}
    }
}
}

// ── `RISCVMachine *s = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` · riscv_machine.c:157-159 · chunk 1974 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 159, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1974 {
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    // glibc assert failure hook; it does not return.
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

static ASSERTION: &[u8] = b"size_log2 == 2\0";
static FILE_NAME: &[u8] = b"riscv_machine.c\0";
static FUNC_NAME: &[u8] =
    b"void htif_write(void *opaque, uint32_t offset, uint32_t val, int size_log2)\0";

// Mirrors the C prologue of htif_write: the assert on size_log2 and the
// `RISCVMachine *s = opaque;` binding (pointer handed back to the caller).
#[no_mangle]
pub extern "C" fn htif_write__part1974(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                ASSERTION.as_ptr() as *const c_char,
                FILE_NAME.as_ptr() as *const c_char,
                159,
                FUNC_NAME.as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:160-177 · chunk 1975 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0:
//             s->htif_tohost = (s->htif_tohost & ~0xffffffff) | val;
//             break;
//         case 4:
//             s->htif_tohost = (s->htif_tohost & 0xffffffff) | ((uint64_t)val << 32);
//             htif_handle_cmd(s);
//             break;
//         case 8:
//             s->htif_fromhost = (s->htif_fromhost & ~0xffffffff) | val;
//             break;
//         case 12:
//             s->htif_fromhost = (s->htif_fromhost & 0xffffffff) |
//                 (uint64_t)val << 32;
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1975 {
// Port of the htif_write register switch (riscv_machine.c lines 160-177).
// htif_handle_cmd is a static C function, so its logic is mirrored here.

use core::ffi::{c_char, c_int, c_ulong, c_void};

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,

    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,

    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],

    pub htif_tohost: u64,
    pub htif_fromhost: u64,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,

    pub virtio_count: c_int,
}

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(code: c_int) -> !;
}

unsafe fn htif_handle_cmd(s: *mut RISCVMachine) {
    let tohost = (*s).htif_tohost;
    let device: u32 = (tohost >> 56) as u32;
    let cmd: u32 = ((tohost >> 48) & 0xff) as u32;
    if tohost == 1 {
        printf(b"\nPower off.\n\0".as_ptr() as *const c_char);
        exit(0);
    } else if device == 1 && cmd == 1 {
        let mut buf = [0u8; 1];
        buf[0] = (tohost & 0xff) as u8;
        let console = (*s).common.console;
        if !console.is_null() {
            if let Some(write_data) = (*console).write_data {
                write_data((*console).opaque, buf.as_mut_ptr(), 1);
            }
        }
        (*s).htif_tohost = 0;
        (*s).htif_fromhost = ((device as u64) << 56) | ((cmd as u64) << 48);
    } else if device == 1 && cmd == 0 {
        (*s).htif_tohost = 0;
    } else {
        printf(
            b"HTIF: unsupported tohost=0x%016lx\n\0".as_ptr() as *const c_char,
            (*s).htif_tohost as c_ulong,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn htif_write__part1975(s: *mut RISCVMachine, offset: u32, val: u32) {
    match offset {
        0 => {
            (*s).htif_tohost = ((*s).htif_tohost & !0xffffffffu64) | (val as u64);
        }
        4 => {
            (*s).htif_tohost = ((*s).htif_tohost & 0xffffffffu64) | ((val as u64) << 32);
            htif_handle_cmd(s);
        }
        8 => {
            (*s).htif_fromhost = ((*s).htif_fromhost & !0xffffffffu64) | (val as u64);
        }
        12 => {
            (*s).htif_fromhost = ((*s).htif_fromhost & 0xffffffffu64) | ((val as u64) << 32);
        }
        _ => {}
    }
}
}

// ── `clint_read` · riscv_machine.c:196-220 · chunk 1976 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t clint_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         RISCVMachine *m = opaque;
//         uint32_t val;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 201, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0xbff8:
//             val = rtc_get_time(m);
//             break;
//         case 0xbffc:
//             val = rtc_get_time(m) >> 32;
//             break;
//         case 0x4000:
//             val = m->timecmp;
//             break;
//         case 0x4004:
//             val = m->timecmp >> 32;
//             break;
//         default:
//             val = 0;
//             break;
//         }
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1976 {
// Rust rewrite of `clint_read` from riscv_machine.c
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_long, c_void};

// ---- Opaque/forward-declared types from the C translation unit ----

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int, // BOOL
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(*mut c_void) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

// CLOCK_MONOTONIC on Linux
const CLOCK_MONOTONIC: c_int = 1;
const RTC_FREQ: u64 = 10000000;
const RTC_FREQ_DIV: u64 = 16;

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

// ---- static inline helpers from the C translation unit ----

#[inline]
unsafe fn riscv_cpu_get_cycles(s: *mut RISCVCPUState) -> u64 {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        // The C code would call through a NULL pointer here (undefined
        // behaviour); abort instead of unwinding across the FFI boundary.
        None => std::process::abort(),
    }
}

// ---- static functions of riscv_machine.c ----

unsafe fn rtc_get_real_time(_s: *mut RISCVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);
    (ts.tv_sec as u64).wrapping_mul(RTC_FREQ) + ((ts.tv_nsec / (1000000000 / RTC_FREQ as c_long)) as u64)
}

unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    if (*m).rtc_real_time != 0 {
        rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time)
    } else {
        riscv_cpu_get_cycles((*m).cpu_state) / RTC_FREQ_DIV
    }
}

#[no_mangle]
pub unsafe extern "C" fn clint_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    let m = opaque as *mut RISCVMachine;
    let val: u32;

    // assert(size_log2 == 2);
    let _ = size_log2;

    match offset {
        0xbff8 => {
            val = rtc_get_time(m) as u32;
        }
        0xbffc => {
            val = (rtc_get_time(m) >> 32) as u32;
        }
        0x4000 => {
            val = (*m).timecmp as u32;
        }
        0x4004 => {
            val = ((*m).timecmp >> 32) as u32;
        }
        _ => {
            val = 0;
        }
    }
    val
}
}

// ── `RISCVMachine *m = opaque; uint32_t val; ((void) sizeof (__assert_single_arg ( size_log2 == 2 ))…` · riscv_machine.c:198-201 · chunk 1977 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *m = opaque;
//         uint32_t val;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 201, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1977 {
// Port of the prologue of clint_read: sets up the opaque pointer and checks
// that the access size is 32 bits (size_log2 == 2), matching assert().
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn clint_read__part1977(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                b"size_log2 == 2\0".as_ptr() as *const c_char,
                b"riscv_machine.c\0".as_ptr() as *const c_char,
                201u32,
                b"clint_read\0".as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:202-218 · chunk 1978 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0xbff8:
//             val = rtc_get_time(m);
//             break;
//         case 0xbffc:
//             val = rtc_get_time(m) >> 32;
//             break;
//         case 0x4000:
//             val = m->timecmp;
//             break;
//         case 0x4004:
//             val = m->timecmp >> 32;
//             break;
//         default:
//             val = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1978 {
// Rust port of the clint_read switch (TARGET) plus the static helpers
// rtc_get_time / rtc_get_real_time / riscv_cpu_get_cycles it depends on
// (they are `static` in C, so their logic is inlined here).

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
}

#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: *const c_void,
    riscv_cpu_end: *const c_void,
    riscv_cpu_interp: *const c_void,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clockid: c_int, tp: *mut Timespec) -> c_int;
}

unsafe fn rtc_get_real_time(_s: *mut RISCVMachine) -> u64 {
    let mut ts: Timespec = core::mem::zeroed();
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts as *mut Timespec);
    (ts.tv_sec as u64)
        .wrapping_mul(10000000)
        .wrapping_add((ts.tv_nsec / (1000000000 / 10000000)) as u64)
}

unsafe fn cpu_get_cycles(s: *mut c_void) -> u64 {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0,
    }
}

unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    let val: u64;
    if (*m).rtc_real_time != 0 {
        val = rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time);
    } else {
        val = cpu_get_cycles((*m).cpu_state) / 16;
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn clint_read__part1978(opaque: *mut c_void, offset: u32) -> u32 {
    let m = opaque as *mut RISCVMachine;
    let val: u32;
    match offset {
        0xbff8 => {
            val = rtc_get_time(m) as u32;
        }
        0xbffc => {
            val = (rtc_get_time(m) >> 32) as u32;
        }
        0x4000 => {
            val = (*m).timecmp as u32;
        }
        0x4004 => {
            val = ((*m).timecmp >> 32) as u32;
        }
        _ => {
            val = 0;
        }
    }
    val
}
}

// ── `return val;` · riscv_machine.c:219-219 · chunk 1979 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1979 {
// Ported piece of clint_read: returns the value read.
#[no_mangle]
pub extern "C" fn clint_read__part1979(val: u32) -> u32 {
    val
}
}

// ── `clint_write` · riscv_machine.c:222-240 · chunk 1980 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void clint_write(void *opaque, uint32_t offset, uint32_t val,
//                           int size_log2)
//     {
//         RISCVMachine *m = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 227, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0x4000:
//             m->timecmp = (m->timecmp & ~0xffffffff) | val;
//             riscv_cpu_reset_mip(m->cpu_state, (1 << 7));
//             break;
//         case 0x4004:
//             m->timecmp = (m->timecmp & 0xffffffff) | ((uint64_t)val << 32);
//             riscv_cpu_reset_mip(m->cpu_state, (1 << 7));
//             break;
//         default:
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1980 {
// Rust rewrite of `clint_write` from riscv_machine.c
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

// ---- Opaque / forward declared C types -----------------------------------

#[repr(C)]
pub struct PhysMemoryMap {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct VIRTIODevice {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    _priv: [u8; 0],
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

// typedef struct { SetIRQFunc *set_irq; void *opaque; int irq_num; } IRQSignal;
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int, // BOOL
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

// ---- RISCVCPUClass / RISCVCPUCommonState ---------------------------------

pub type riscv_cpu_init_fn = unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState;
pub type riscv_cpu_end_fn = unsafe extern "C" fn(*mut RISCVCPUState);
pub type riscv_cpu_interp_fn = unsafe extern "C" fn(*mut RISCVCPUState, c_int);
pub type riscv_cpu_get_cycles_fn = unsafe extern "C" fn(*mut RISCVCPUState) -> u64;
pub type riscv_cpu_set_mip_fn = unsafe extern "C" fn(*mut RISCVCPUState, u32);
pub type riscv_cpu_reset_mip_fn = unsafe extern "C" fn(*mut RISCVCPUState, u32);
pub type riscv_cpu_get_mip_fn = unsafe extern "C" fn(*mut RISCVCPUState) -> u32;
pub type riscv_cpu_get_power_down_fn = unsafe extern "C" fn(*mut RISCVCPUState) -> c_int;
pub type riscv_cpu_get_misa_fn = unsafe extern "C" fn(*mut RISCVCPUState) -> u32;
pub type riscv_cpu_flush_tlb_write_range_ram_fn =
    unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize);

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<riscv_cpu_init_fn>,
    pub riscv_cpu_end: Option<riscv_cpu_end_fn>,
    pub riscv_cpu_interp: Option<riscv_cpu_interp_fn>,
    pub riscv_cpu_get_cycles: Option<riscv_cpu_get_cycles_fn>,
    pub riscv_cpu_set_mip: Option<riscv_cpu_set_mip_fn>,
    pub riscv_cpu_reset_mip: Option<riscv_cpu_reset_mip_fn>,
    pub riscv_cpu_get_mip: Option<riscv_cpu_get_mip_fn>,
    pub riscv_cpu_get_power_down: Option<riscv_cpu_get_power_down_fn>,
    pub riscv_cpu_get_misa: Option<riscv_cpu_get_misa_fn>,
    pub riscv_cpu_flush_tlb_write_range_ram: Option<riscv_cpu_flush_tlb_write_range_ram_fn>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

// ---- Externs --------------------------------------------------------------

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: core::ffi::c_uint,
        __function: *const c_char,
    ) -> !;
}

// static inline void riscv_cpu_reset_mip(RISCVCPUState *s, uint32_t mask)
#[inline]
unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_reset_mip {
        f(s, mask);
    }
}

const MIP_MTIP: u32 = 1 << 7;

#[no_mangle]
pub unsafe extern "C" fn clint_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    let m = opaque as *mut RISCVMachine;

    if size_log2 == 2 {
        // nothing to do
    } else {
        __assert_fail(
            b"size_log2 == 2\0".as_ptr() as *const c_char,
            b"riscv_machine.c\0".as_ptr() as *const c_char,
            227,
            b"clint_write\0".as_ptr() as *const c_char,
        );
    }

    match offset {
        0x4000 => {
            // C: m->timecmp = (m->timecmp & ~0xffffffff) | val;
            // `~0xffffffff` is computed in `unsigned int` and is 0.
            let keep: u64 = (!0xffff_ffffu32) as u64;
            (*m).timecmp = ((*m).timecmp & keep) | (val as u64);
            riscv_cpu_reset_mip((*m).cpu_state, MIP_MTIP);
        }
        0x4004 => {
            // C: m->timecmp = (m->timecmp & 0xffffffff) | ((uint64_t)val << 32);
            (*m).timecmp = ((*m).timecmp & 0xffff_ffffu64) | ((val as u64) << 32);
            riscv_cpu_reset_mip((*m).cpu_state, MIP_MTIP);
        }
        _ => {}
    }
}
}

// ── `RISCVMachine *m = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` · riscv_machine.c:225-227 · chunk 1981 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *m = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 227, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1981 {
// Port of the prologue of clint_write: fetch the machine pointer and
// check the access size assertion.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn clint_write__part1981(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                b"size_log2 == 2\0".as_ptr() as *const c_char,
                b"riscv_machine.c\0".as_ptr() as *const c_char,
                227,
                b"clint_write\0".as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:228-239 · chunk 1982 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0x4000:
//             m->timecmp = (m->timecmp & ~0xffffffff) | val;
//             riscv_cpu_reset_mip(m->cpu_state, (1 << 7));
//             break;
//         case 0x4004:
//             m->timecmp = (m->timecmp & 0xffffffff) | ((uint64_t)val << 32);
//             riscv_cpu_reset_mip(m->cpu_state, (1 << 7));
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1982 {
// Port of the switch statement in clint_write (riscv_machine.c lines 228-239).

use core::ffi::{c_int, c_void};

// Vtable layout of RISCVCPUClass (only the entries needed for offset math).
#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    riscv_cpu_end: Option<unsafe extern "C" fn(*mut c_void)>,
    riscv_cpu_interp: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

// VirtMachine embedded as the first member of RISCVMachine: five pointers.
#[repr(C)]
pub struct VirtMachine {
    vmc: *mut c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
}

// Port of the static inline riscv_cpu_reset_mip(s, mask).
unsafe fn cpu_reset_mip(s: *mut c_void, mask: u32) {
    let class_ptr = *(s as *mut *const RISCVCPUClass);
    if !class_ptr.is_null() {
        if let Some(f) = (*class_ptr).riscv_cpu_reset_mip {
            f(s, mask);
        }
    }
}

#[no_mangle]
pub extern "C" fn clint_write__part1982(m: *mut RISCVMachine, offset: u32, val: u32) {
    unsafe {
        match offset {
            0x4000 => {
                // (m->timecmp & ~0xffffffff) | val  -- ~0xffffffffu == 0u in C
                let keep: u64 = u64::from(!0xffff_ffffu32);
                (*m).timecmp = ((*m).timecmp & keep) | u64::from(val);
                cpu_reset_mip((*m).cpu_state, 1u32 << 7);
            }
            0x4004 => {
                (*m).timecmp = ((*m).timecmp & 0xffff_ffffu64) | (u64::from(val) << 32);
                cpu_reset_mip((*m).cpu_state, 1u32 << 7);
            }
            _ => {}
        }
    }
}
}

// ── `plic_update_mip` · riscv_machine.c:242-252 · chunk 1983 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void plic_update_mip(RISCVMachine *s)
//     {
//         RISCVCPUState *cpu = s->cpu_state;
//         uint32_t mask;
//         mask = s->plic_pending_irq & ~s->plic_served_irq;
//         if (mask) {
//             riscv_cpu_set_mip(cpu, (1 << 11) | (1 << 9));
//         } else {
//             riscv_cpu_reset_mip(cpu, (1 << 11) | (1 << 9));
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1983 {
// Rewrite of plic_update_mip() from riscv_machine.c
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VIRTIODevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
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

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct RISCVCPUState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

pub type SetMipFunc = unsafe extern "C" fn(*mut RISCVCPUState, u32);
pub type ResetMipFunc = unsafe extern "C" fn(*mut RISCVCPUState, u32);

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: *mut c_void,
    pub riscv_cpu_end: *mut c_void,
    pub riscv_cpu_interp: *mut c_void,
    pub riscv_cpu_get_cycles: *mut c_void,
    pub riscv_cpu_set_mip: Option<SetMipFunc>,
    pub riscv_cpu_reset_mip: Option<ResetMipFunc>,
    pub riscv_cpu_get_mip: *mut c_void,
    pub riscv_cpu_get_power_down: *mut c_void,
    pub riscv_cpu_get_misa: *mut c_void,
    pub riscv_cpu_flush_tlb_write_range_ram: *mut c_void,
}

const MIP_SEIP: u32 = 1 << 9;
const MIP_MEIP: u32 = 1 << 11;

#[no_mangle]
pub unsafe extern "C" fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask: u32 = (*s).plic_pending_irq & !(*s).plic_served_irq;
    let class_ptr = (*(cpu as *mut RISCVCPUCommonState)).class_ptr;
    if mask != 0 {
        if let Some(f) = (*class_ptr).riscv_cpu_set_mip {
            f(cpu, MIP_MEIP | MIP_SEIP);
        }
    } else {
        if let Some(f) = (*class_ptr).riscv_cpu_reset_mip {
            f(cpu, MIP_MEIP | MIP_SEIP);
        }
    }
}
}

// ── `RISCVCPUState *cpu = s->cpu_state; uint32_t mask; mask = s->plic_pending_irq & ~s->plic_served_…` · riscv_machine.c:244-246 · chunk 1984 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     RISCVCPUState *cpu = s->cpu_state;
//         uint32_t mask;
//         mask = s->plic_pending_irq & ~s->plic_served_irq;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1984 {
// Piece of plic_update_mip: reads the machine state and computes the pending mask.

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const core::ffi::c_void,
    pub net: *mut core::ffi::c_void,
    pub console_dev: *mut core::ffi::c_void,
    pub console: *mut core::ffi::c_void,
    pub fb_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
    pub irq_num: core::ffi::c_int,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut core::ffi::c_void,
    pub max_xlen: core::ffi::c_int,
    pub cpu_state: *mut core::ffi::c_void,
    pub ram_size: u64,
    pub rtc_real_time: core::ffi::c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut core::ffi::c_void,
    pub mouse_dev: *mut core::ffi::c_void,
    pub virtio_count: core::ffi::c_int,
}

/// Yields `s->cpu_state` through `cpu_out` and returns
/// `s->plic_pending_irq & ~s->plic_served_irq`.
#[no_mangle]
pub unsafe extern "C" fn plic_update_mip__part1984(
    s: *mut RISCVMachine,
    cpu_out: *mut *mut core::ffi::c_void,
) -> u32 {
    let machine = &*s;
    if !cpu_out.is_null() {
        *cpu_out = machine.cpu_state;
    }
    machine.plic_pending_irq & !machine.plic_served_irq
}
}

// ── `if (mask) … else` · riscv_machine.c:247-251 · chunk 1985 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mask) {
//             riscv_cpu_set_mip(cpu, (1 << 11) | (1 << 9));
//         } else {
//             riscv_cpu_reset_mip(cpu, (1 << 11) | (1 << 9));
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1985 {
// Ported piece of plic_update_mip: the pending-IRQ branch that sets or clears
// the machine external + supervisor external MIP bits.
use core::ffi::{c_int, c_void};

// RISCVCPUState is opaque here (only the class pointer is read through it).
type PhysMemoryMap = c_void;

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(mem_map: *mut PhysMemoryMap) -> *mut c_void>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(s: *mut c_void)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(s: *mut c_void, n_cycles: c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(s: *mut c_void) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(s: *mut c_void, mask: u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(s: *mut c_void, mask: u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(s: *mut c_void) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(s: *mut c_void) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(s: *mut c_void) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(s: *mut c_void, ram_ptr: *mut u8, ram_size: usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[no_mangle]
pub unsafe extern "C" fn plic_update_mip__part1985(cpu: *mut c_void, mask: u32) {
    let common = cpu as *mut RISCVCPUCommonState;
    let class_ptr = (*common).class_ptr;
    let bits: u32 = (1u32 << 11) | (1u32 << 9);
    if mask != 0 {
        let set_mip = (*class_ptr).riscv_cpu_set_mip;
        if let Some(f) = set_mip {
            f(cpu, bits);
        }
    } else {
        let reset_mip = (*class_ptr).riscv_cpu_reset_mip;
        if let Some(f) = reset_mip {
            f(cpu, bits);
        }
    }
}
}

// ── `plic_read` · riscv_machine.c:257-283 · chunk 1986 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t plic_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         RISCVMachine *s = opaque;
//         uint32_t val, mask;
//         int i;
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 262, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0x200000:
//             val = 0;
//             break;
//         case 0x200000 + 4:
//             mask = s->plic_pending_irq & ~s->plic_served_irq;
//             if (mask != 0) {
//                 i = ctz32(mask);
//                 s->plic_served_irq |= 1 << i;
//                 plic_update_mip(s);
//                 val = i + 1;
//             } else {
//                 val = 0;
//             }
//             break;
//         default:
//             val = 0;
//             break;
//         }
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1986 {
// Port of static uint32_t plic_read(void *opaque, uint32_t offset, int size_log2)
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// ---------------------------------------------------------------- C types ---
pub type BOOL = c_int;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;

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
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _opaque: [u8; 0],
}

// typedef struct VirtMachineClass VirtMachineClass;  (only used through a pointer)
#[repr(C)]
pub struct VirtMachineClass {
    _opaque: [u8; 0],
}

// typedef struct VirtMachine { ... } VirtMachine;
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// typedef struct RISCVMachine { ... } RISCVMachine;
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: uint64_t,

    pub rtc_real_time: BOOL,
    pub rtc_start_time: uint64_t,
    pub timecmp: uint64_t,

    pub plic_pending_irq: uint32_t,
    pub plic_served_irq: uint32_t,
    pub plic_irq: [IRQSignal; 32],

    pub htif_tohost: uint64_t,
    pub htif_fromhost: uint64_t,

    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,

    pub virtio_count: c_int,
}

// typedef struct { ... } RISCVCPUClass;
type FnRiscvCpuInit = unsafe extern "C" fn(mem_map: *mut PhysMemoryMap) -> *mut RISCVCPUState;
type FnRiscvCpuEnd = unsafe extern "C" fn(s: *mut RISCVCPUState);
type FnRiscvCpuInterp = unsafe extern "C" fn(s: *mut RISCVCPUState, n_cycles: c_int);
type FnRiscvCpuGetCycles = unsafe extern "C" fn(s: *mut RISCVCPUState) -> uint64_t;
type FnRiscvCpuSetMip = unsafe extern "C" fn(s: *mut RISCVCPUState, mask: uint32_t);
type FnRiscvCpuResetMip = unsafe extern "C" fn(s: *mut RISCVCPUState, mask: uint32_t);
type FnRiscvCpuGetMip = unsafe extern "C" fn(s: *mut RISCVCPUState) -> uint32_t;
type FnRiscvCpuGetPowerDown = unsafe extern "C" fn(s: *mut RISCVCPUState) -> BOOL;
type FnRiscvCpuGetMisa = unsafe extern "C" fn(s: *mut RISCVCPUState) -> uint32_t;
type FnRiscvCpuFlushTlbWriteRangeRam =
    unsafe extern "C" fn(s: *mut RISCVCPUState, ram_ptr: *mut uint8_t, ram_size: size_t);

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<FnRiscvCpuInit>,
    pub riscv_cpu_end: Option<FnRiscvCpuEnd>,
    pub riscv_cpu_interp: Option<FnRiscvCpuInterp>,
    pub riscv_cpu_get_cycles: Option<FnRiscvCpuGetCycles>,
    pub riscv_cpu_set_mip: Option<FnRiscvCpuSetMip>,
    pub riscv_cpu_reset_mip: Option<FnRiscvCpuResetMip>,
    pub riscv_cpu_get_mip: Option<FnRiscvCpuGetMip>,
    pub riscv_cpu_get_power_down: Option<FnRiscvCpuGetPowerDown>,
    pub riscv_cpu_get_misa: Option<FnRiscvCpuGetMisa>,
    pub riscv_cpu_flush_tlb_write_range_ram: Option<FnRiscvCpuFlushTlbWriteRangeRam>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

// ------------------------------------------------- ported static helpers ---

const MIP_MEIP_SEIP: uint32_t = (1u32 << 11) | (1u32 << 9);
const PLIC_HART_BASE: uint32_t = 0x200000;

// static inline int ctz32(uint32_t val) { return val ? __builtin_ctz(val) : 32; }
#[inline]
fn ctz32(val: uint32_t) -> c_int {
    if val != 0 {
        val.trailing_zeros() as c_int
    } else {
        32
    }
}

#[inline]
unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: uint32_t) {
    let c: *const RISCVCPUClass = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_set_mip {
        f(s, mask);
    }
}

#[inline]
unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: uint32_t) {
    let c: *const RISCVCPUClass = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_reset_mip {
        f(s, mask);
    }
}

// static void plic_update_mip(RISCVMachine *s)
unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu: *mut RISCVCPUState = (*s).cpu_state;
    let mask: uint32_t = (*s).plic_pending_irq & !(*s).plic_served_irq;
    if mask != 0 {
        riscv_cpu_set_mip(cpu, MIP_MEIP_SEIP);
    } else {
        riscv_cpu_reset_mip(cpu, MIP_MEIP_SEIP);
    }
}

// ----------------------------------------------------------------- TARGET ---

#[no_mangle]
pub unsafe extern "C" fn plic_read(
    opaque: *mut c_void,
    offset: uint32_t,
    size_log2: c_int,
) -> uint32_t {
    let s: *mut RISCVMachine = opaque as *mut RISCVMachine;
    let val: uint32_t;

    if size_log2 != 2 {
        __assert_fail(
            b"size_log2 == 2\0".as_ptr() as *const c_char,
            b"riscv_machine.c\0".as_ptr() as *const c_char,
            262,
            b"plic_read\0".as_ptr() as *const c_char,
        );
    }

    if offset == PLIC_HART_BASE {
        val = 0;
    } else if offset == PLIC_HART_BASE + 4 {
        let mask: uint32_t = (*s).plic_pending_irq & !(*s).plic_served_irq;
        if mask != 0 {
            let i: c_int = ctz32(mask);
            (*s).plic_served_irq |= 1u32 << (i as u32);
            plic_update_mip(s);
            val = (i as uint32_t).wrapping_add(1);
        } else {
            val = 0;
        }
    } else {
        val = 0;
    }
    val
}
}

// ── `RISCVMachine *s = opaque; uint32_t val, mask; int i; ((void) sizeof (__assert_single_arg ( size…` · riscv_machine.c:259-262 · chunk 1987 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//         uint32_t val, mask;
//         int i;
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 262, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1987 {
use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

static ASSERTION: &[u8] = b"size_log2 == 2\0";
static FILE_NAME: &[u8] = b"riscv_machine.c\0";
static FUNCTION: &[u8] = b"plic_read\0";

// Prologue of plic_read: the size assertion, plus the `s = opaque` bind.
#[no_mangle]
pub extern "C" fn plic_read__part1987(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                ASSERTION.as_ptr() as *const c_char,
                FILE_NAME.as_ptr() as *const c_char,
                262 as c_uint,
                FUNCTION.as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:263-281 · chunk 1988 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0x200000:
//             val = 0;
//             break;
//         case 0x200000 + 4:
//             mask = s->plic_pending_irq & ~s->plic_served_irq;
//             if (mask != 0) {
//                 i = ctz32(mask);
//                 s->plic_served_irq |= 1 << i;
//                 plic_update_mip(s);
//                 val = i + 1;
//             } else {
//                 val = 0;
//             }
//             break;
//         default:
//             val = 0;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1988 {
// Port of the switch statement in plic_read (riscv_machine.c lines 263-281).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

// Mirror of the C RISCVMachine up to the fields this code touches.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
    plic_irq: [IRQSignal; 32],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: *const c_void,
    riscv_cpu_end: *const c_void,
    riscv_cpu_interp: *const c_void,
    riscv_cpu_get_cycles: *const c_void,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_get_mip: *const c_void,
    riscv_cpu_get_power_down: *const c_void,
    riscv_cpu_get_misa: *const c_void,
    riscv_cpu_flush_tlb_write_range_ram: *const c_void,
}

// static inline int ctz32(uint32_t val) { return val ? __builtin_ctz(val) : 32; }
fn ctz32(val: u32) -> c_int {
    if val != 0 {
        val.trailing_zeros() as c_int
    } else {
        32
    }
}

// static void plic_update_mip(RISCVMachine *s) -- file-static, so ported here.
unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
    let common = cpu as *const RISCVCPUCommonState;
    let class = (*common).class_ptr;
    if class.is_null() {
        return;
    }
    if mask != 0 {
        if let Some(f) = (*class).riscv_cpu_set_mip {
            f(cpu, (1u32 << 11) | (1u32 << 9));
        }
    } else {
        if let Some(f) = (*class).riscv_cpu_reset_mip {
            f(cpu, (1u32 << 11) | (1u32 << 9));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn plic_read__part1988(s: *mut RISCVMachine, offset: u32) -> u32 {
    let mut val: u32;
    match offset {
        0x200000 => {
            val = 0;
        }
        0x200004 => {
            let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
            if mask != 0 {
                let i: c_int = ctz32(mask);
                (*s).plic_served_irq |= 1u32.wrapping_shl(i as u32);
                plic_update_mip(s);
                val = (i as u32).wrapping_add(1);
            } else {
                val = 0;
            }
        }
        _ => {
            val = 0;
        }
    }
    val
}
}

// ── `return val;` · riscv_machine.c:282-282 · chunk 1989 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1989 {
// Extracted from plic_read: returns the accumulated read value.
#[no_mangle]
pub extern "C" fn plic_read__part1989(val: u32) -> u32 {
    val
}
}

// ── `plic_write` · riscv_machine.c:285-302 · chunk 1990 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void plic_write(void *opaque, uint32_t offset, uint32_t val,
//                            int size_log2)
//     {
//         RISCVMachine *s = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 290, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
//         switch(offset) {
//         case 0x200000 + 4:
//             val--;
//             if (val < 32) {
//                 s->plic_served_irq &= ~(1 << val);
//                 plic_update_mip(s);
//             }
//             break;
//         default:
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1990 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct RISCVCPUState {
    _private: [u8; 0],
}

#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: Option<unsafe extern "C" fn(*mut c_void) -> *mut RISCVCPUState>,
    riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
}

#[repr(C)]
struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut RISCVCPUState,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    ) -> !;
}

unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_set_mip {
        Some(f) => f(s, mask),
        None => {}
    }
}

unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_reset_mip {
        Some(f) => f(s, mask),
        None => {}
    }
}

unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
    if mask != 0 {
        riscv_cpu_set_mip(cpu, (1u32 << 11) | (1u32 << 9));
    } else {
        riscv_cpu_reset_mip(cpu, (1u32 << 11) | (1u32 << 9));
    }
}

#[no_mangle]
pub unsafe extern "C" fn plic_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    let s = opaque as *mut RISCVMachine;

    if size_log2 != 2 {
        __assert_fail(
            b"size_log2 == 2\0".as_ptr() as *const c_char,
            b"riscv_machine.c\0".as_ptr() as *const c_char,
            290,
            b"plic_write\0".as_ptr() as *const c_char,
        );
    }

    if offset == 0x200000u32.wrapping_add(4) {
        let v = val.wrapping_sub(1);
        if v < 32 {
            (*s).plic_served_irq &= !(1u32 << v);
            plic_update_mip(s);
        }
    }
}
}

// ── `RISCVMachine *s = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` · riscv_machine.c:288-290 · chunk 1991 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        size_log2 == 2
//        )), __extension__ ({ if (
//        size_log2 == 2
//        ) ; else __assert_fail (
//        "size_log2 == 2"
//        , "riscv_machine.c", 290, __extension__ __PRETTY_FUNCTION__); }))
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1991 {
// Port of the prologue of plic_write: binds the opaque state pointer and
// performs the debug assertion on size_log2.
#![allow(non_snake_case)]
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // libc assert failure handler; declared noreturn in C.
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn plic_write__part1991(opaque: *mut c_void, size_log2: c_int) -> *mut c_void {
    if size_log2 != 2 {
        unsafe {
            __assert_fail(
                b"size_log2 == 2\0".as_ptr() as *const c_char,
                b"riscv_machine.c\0".as_ptr() as *const c_char,
                290u32,
                b"plic_write\0".as_ptr() as *const c_char,
            );
        }
    }
    opaque
}
}

// ── `switch (offset)` · riscv_machine.c:291-301 · chunk 1992 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset) {
//         case 0x200000 + 4:
//             val--;
//             if (val < 32) {
//                 s->plic_served_irq &= ~(1 << val);
//                 plic_update_mip(s);
//             }
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1992 {
// Port of the switch statement in plic_write (riscv_machine.c)
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    set_irq: *mut c_void,
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
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
    plic_irq: [IRQSignal; 32],
    htif_tohost: u64,
    htif_fromhost: u64,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    virtio_count: c_int,
}

#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: *mut c_void,
    riscv_cpu_end: *mut c_void,
    riscv_cpu_interp: *mut c_void,
    riscv_cpu_get_cycles: *mut c_void,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_get_mip: *mut c_void,
    riscv_cpu_get_power_down: *mut c_void,
    riscv_cpu_get_misa: *mut c_void,
    riscv_cpu_flush_tlb_write_range_ram: *mut c_void,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

// Port of the static C function plic_update_mip.
unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
    let clas = (*(cpu as *const RISCVCPUCommonState)).class_ptr;
    if mask != 0 {
        if let Some(f) = (*clas).riscv_cpu_set_mip {
            f(cpu, (1u32 << 11) | (1u32 << 9));
        }
    } else if let Some(f) = (*clas).riscv_cpu_reset_mip {
        f(cpu, (1u32 << 11) | (1u32 << 9));
    }
}

#[no_mangle]
pub unsafe extern "C" fn plic_write__part1992(opaque: *mut c_void, offset: u32, mut val: u32) {
    let s = opaque as *mut RISCVMachine;
    if offset == 0x200000u32.wrapping_add(4) {
        val = val.wrapping_sub(1);
        if val < 32 {
            (*s).plic_served_irq &= !(1u32 << val);
            plic_update_mip(s);
        }
    }
}
}

// ── `plic_set_irq` · riscv_machine.c:304-315 · chunk 1993 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void plic_set_irq(void *opaque, int irq_num, int state)
//     {
//         RISCVMachine *s = opaque;
//         uint32_t mask;
//     
//         mask = 1 << (irq_num - 1);
//         if (state)
//             s->plic_pending_irq |= mask;
//         else
//             s->plic_pending_irq &= ~mask;
//         plic_update_mip(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1993 {
// Rust rewrite of the static C function plic_set_irq from riscv_machine.c
// (which in turn inlines the static plic_update_mip helper).

use core::ffi::{c_int, c_void};

/// Opaque C types that we only ever hold by pointer.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

// typedef struct VirtMachine { ... } VirtMachine;  (leading part of RISCVMachine)
#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// typedef struct RISCVMachine { ... } RISCVMachine;
#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut PhysMemoryMap,
    max_xlen: c_int,
    cpu_state: *mut RISCVCPUState,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
}

// typedef struct { ... } RISCVCPUClass;
#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: Option<unsafe extern "C" fn(mem_map: *mut PhysMemoryMap) -> *mut RISCVCPUState>,
    riscv_cpu_end: Option<unsafe extern "C" fn(s: *mut RISCVCPUState)>,
    riscv_cpu_interp: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, n_cycles: c_int)>,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u64>,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, mask: u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, mask: u32)>,
    riscv_cpu_get_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u32>,
    riscv_cpu_get_power_down: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> c_int>,
    riscv_cpu_get_misa: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u32>,
    riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(s: *mut RISCVCPUState, ram_ptr: *mut u8, ram_size: usize)>,
}

// typedef struct { const RISCVCPUClass *class_ptr; } RISCVCPUCommonState;
#[repr(C)]
pub struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

const MIP_MEIP_SEIP: u32 = (1u32 << 11) | (1u32 << 9);

// Ported static helper plic_update_mip() from the same translation unit.
unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
    let class_ptr = (*(cpu as *const RISCVCPUCommonState)).class_ptr;
    if mask != 0 {
        if let Some(f) = (*class_ptr).riscv_cpu_set_mip {
            f(cpu, MIP_MEIP_SEIP);
        }
    } else if let Some(f) = (*class_ptr).riscv_cpu_reset_mip {
        f(cpu, MIP_MEIP_SEIP);
    }
}

#[no_mangle]
pub unsafe extern "C" fn plic_set_irq(opaque: *mut c_void, irq_num: c_int, state: c_int) {
    let s = opaque as *mut RISCVMachine;

    // mask = 1 << (irq_num - 1);  (int shift, then converted to uint32_t)
    let mask = (1i32.wrapping_shl((irq_num - 1) as u32)) as u32;
    if state != 0 {
        (*s).plic_pending_irq |= mask;
    } else {
        (*s).plic_pending_irq &= !mask;
    }
    plic_update_mip(s);
}
}

// ── `RISCVMachine *s = opaque; uint32_t mask; mask = 1 << (irq_num - 1);` · riscv_machine.c:306-309 · chunk 1994 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//         uint32_t mask;
//     
//         mask = 1 << (irq_num - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1994 {
// Ported piece of plic_set_irq: compute the pending-irq mask for irq_num.
// The C side keeps the `s` / `mask` declarations and the pending-irq update.

use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn plic_set_irq__part1994(_opaque: *mut c_void, irq_num: c_int) -> u32 {
    // C: mask = 1 << (irq_num - 1);  (computed as int, then stored in uint32_t)
    let shift = (irq_num - 1) as u32;
    (1i32).wrapping_shl(shift) as u32
}
}

// ── `if (state) … else` · riscv_machine.c:310-313 · chunk 1995 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (state)
//             s->plic_pending_irq |= mask;
//         else
//             s->plic_pending_irq &= ~mask;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1995 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn plic_set_irq__part1995(state: c_int, plic_pending_irq: *mut u32, mask: u32) {
    unsafe {
        if state != 0 {
            *plic_pending_irq |= mask;
        } else {
            *plic_pending_irq &= !mask;
        }
    }
}
}

// ── `plic_update_mip(s);` · riscv_machine.c:314-314 · chunk 1996 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     plic_update_mip(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1996 {
// Port of the plic_update_mip() call made at the end of plic_set_irq().
// plic_update_mip() is a static C function (no linker symbol), so its logic is
// reproduced here; the riscv_cpu_{set,reset}_mip helpers are static inlines that
// dispatch through the RISCVCPUClass vtable of the CPU state, reproduced below.

use core::ffi::c_int;

pub enum PhysMemoryMap {}
pub enum RISCVCPUState {}

type Bool = c_int;

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init:
        Option<unsafe extern "C" fn(mem_map: *mut PhysMemoryMap) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(s: *mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, n_cycles: c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, mask: u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState, mask: u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> Bool>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(s: *mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(s: *mut RISCVCPUState, ram_ptr: *mut u8, ram_size: usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[no_mangle]
pub unsafe extern "C" fn plic_set_irq__part1996(
    cpu: *mut RISCVCPUState,
    plic_pending_irq: u32,
    plic_served_irq: u32,
) {
    if cpu.is_null() {
        return;
    }
    let common = cpu as *mut RISCVCPUCommonState;
    let class_ptr = (*common).class_ptr;
    if class_ptr.is_null() {
        return;
    }
    let class: &RISCVCPUClass = &*class_ptr;
    let mask: u32 = plic_pending_irq & !plic_served_irq;
    let mip_mask: u32 = (1u32 << 11) | (1u32 << 9);
    if mask != 0 {
        if let Some(f) = class.riscv_cpu_set_mip {
            f(cpu, mip_mask);
        }
    } else {
        if let Some(f) = class.riscv_cpu_reset_mip {
            f(cpu, mip_mask);
        }
    }
}
}

// ── `get_ram_ptr` · riscv_machine.c:317-320 · chunk 1997 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint8_t *get_ram_ptr(RISCVMachine *s, uint64_t paddr, BOOL is_rw)
//     {
//         return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1997 {
// Rust rewrite of the C function:
//   static uint8_t *get_ram_ptr(RISCVMachine *s, uint64_t paddr, BOOL is_rw)
//       { return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw); }

use core::ffi::{c_char, c_int, c_void};

// Opaque mirror of `struct PhysMemoryMap` (only pointed to here).
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

// Mirror of `typedef struct VirtMachine { const VirtMachineClass *vmc;
//   EthernetDevice *net; VIRTIODevice *console_dev; CharacterDevice *console;
//   FBDevice *fb_dev; } VirtMachine;`
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Mirror of `typedef struct RISCVMachine { VirtMachine common;
//   PhysMemoryMap *mem_map; int max_xlen; RISCVCPUState *cpu_state;
//   uint64_t ram_size; BOOL rtc_real_time; uint64_t rtc_start_time;
//   uint64_t timecmp; uint32_t plic_pending_irq, plic_served_irq;
//   IRQSignal plic_irq[32]; uint64_t htif_tohost, htif_fromhost;
//   VIRTIODevice *keyboard_dev; VIRTIODevice *mouse_dev; int virtio_count; }
// RISCVMachine;`
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

// Silence unused-import warnings without changing anything.
#[allow(dead_code)]
type UnusedChar = c_char;

extern "C" {
    fn phys_mem_get_ram_ptr(
        map: *mut PhysMemoryMap,
        paddr: u64,
        is_rw: c_int,
    ) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn get_ram_ptr(s: *mut RISCVMachine, paddr: u64, is_rw: c_int) -> *mut u8 {
    unsafe { phys_mem_get_ram_ptr((*s).mem_map, paddr, is_rw) }
}
}

// ── `return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);` · riscv_machine.c:319-319 · chunk 1998 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1998 {
// Port of the phys_mem_get_ram_ptr call inside get_ram_ptr (riscv_machine.c:319).
use core::ffi::c_int;

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

extern "C" {
    fn phys_mem_get_ram_ptr(
        map: *mut PhysMemoryMap,
        paddr: u64,
        is_rw: c_int,
    ) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn get_ram_ptr__part1998(
    map: *mut PhysMemoryMap,
    paddr: u64,
    is_rw: c_int,
) -> *mut u8 {
    unsafe { phys_mem_get_ram_ptr(map, paddr, is_rw) }
}
}

// ── `fdt_init` · riscv_machine.c:362-367 · chunk 2003 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static FDTState *fdt_init(void)
//     {
//         FDTState *s;
//         s = mallocz(sizeof(*s));
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2003 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut core::ffi::c_void;
}

#[no_mangle]
pub extern "C" fn fdt_init() -> *mut FDTState {
    let s: *mut FDTState = unsafe { mallocz(core::mem::size_of::<FDTState>()) as *mut FDTState };
    s
}
}

// ── `FDTState *s; s = mallocz(sizeof(*s)); return s;` · riscv_machine.c:364-366 · chunk 2004 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FDTState *s;
//         s = mallocz(sizeof(*s));
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2004 {
// Rewrite of the body of riscv_machine.c:fdt_init (lines 364-366).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_init__part2004() -> *mut FDTState {
    unsafe { mallocz(core::mem::size_of::<FDTState>()) as *mut FDTState }
}
}

// ── `fdt_alloc_len` · riscv_machine.c:369-377 · chunk 2005 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_alloc_len(FDTState *s, int len)
//     {
//         int new_size;
//         if (__builtin_expect(!!(len > s->tab_size), 0)) {
//             new_size = max_int(len, s->tab_size * 3 / 2);
//             s->tab = realloc(s->tab, new_size * sizeof(uint32_t));
//             s->tab_size = new_size;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2005 {
// Rust rewrite of fdt_alloc_len (riscv_machine.c)
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    let st: &mut FDTState = &mut *s;
    if len > st.tab_size {
        let new_size: c_int = max_int(len, st.tab_size.wrapping_mul(3) / 2);
        // (size_t)new_size * sizeof(uint32_t), with size_t = 64-bit unsigned
        let size: usize = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
        st.tab = realloc(st.tab as *mut c_void, size) as *mut u32;
        st.tab_size = new_size;
    }
}
}

// ── `if (__builtin_expect(!!(len > s->tab_size), 0))` · riscv_machine.c:372-376 · chunk 2006 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (__builtin_expect(!!(len > s->tab_size), 0)) {
//             new_size = max_int(len, s->tab_size * 3 / 2);
//             s->tab = realloc(s->tab, new_size * sizeof(uint32_t));
//             s->tab_size = new_size;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2006 {
// Port of the reallocation branch of fdt_alloc_len.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut i8,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// static inline int max_int(int a, int b) -- no linker symbol, ported here.
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_alloc_len__part2006(
    s: *mut FDTState,
    len: c_int,
    new_size: *mut c_int,
) {
    if !s.is_null() {
        let s = &mut *s;
        if len > s.tab_size {
            let ns = max_int(len, s.tab_size.wrapping_mul(3) / 2);
            if !new_size.is_null() {
                *new_size = ns;
            }
            s.tab = realloc(s.tab as *mut c_void, (ns as usize).wrapping_mul(4)) as *mut u32;
            s.tab_size = ns;
        }
    }
}
}

// ── `fdt_put32` · riscv_machine.c:379-383 · chunk 2007 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_put32(FDTState *s, int v)
//     {
//         fdt_alloc_len(s, s->tab_len + 1);
//         s->tab[s->tab_len++] = cpu_to_be32(v);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2007 {
// Rewrite of: static void fdt_put32(FDTState *s, int v)
// Ported to Rust; fdt_alloc_len (a static C function) is inlined here because
// it has no linker symbol. cpu_to_be32() is a byte swap.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
unsafe fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    if len > (*s).tab_size {
        // max_int(len, s->tab_size * 3 / 2) with C's (wrapping) int arithmetic
        let b = (*s).tab_size.wrapping_mul(3) / 2;
        let new_size: c_int = if len > b { len } else { b };
        // new_size * sizeof(uint32_t): int converted to size_t, wrapping multiply
        let bytes = (new_size as usize).wrapping_mul(4);
        let p = realloc((*s).tab as *mut c_void, bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_put32(s: *mut FdtState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len as usize;
    (*s).tab.add(idx).write((v as u32).swap_bytes());
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
}
}

// ── `fdt_alloc_len(s, s->tab_len + 1); s->tab[s->tab_len++] = cpu_to_be32(v);` · riscv_machine.c:381-382 · chunk 2008 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_alloc_len(s, s->tab_len + 1);
//         s->tab[s->tab_len++] = cpu_to_be32(v);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2008 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_put32__part2008(s: *mut FDTState, v: c_int) {
    unsafe {
        // fdt_alloc_len(s, s->tab_len + 1);
        let len: c_int = (*s).tab_len + 1;
        if len > (*s).tab_size {
            let b: c_int = (*s).tab_size.wrapping_mul(3) / 2;
            let new_size: c_int = if len > b { len } else { b };
            let bytes: usize = (new_size as isize as usize).wrapping_mul(4);
            let p = realloc((*s).tab as *mut c_void, bytes);
            (*s).tab = p as *mut u32;
            (*s).tab_size = new_size;
        }

        // s->tab[s->tab_len++] = cpu_to_be32(v);
        let idx: c_int = (*s).tab_len;
        (*s).tab_len = idx + 1;
        *(*s).tab.add(idx as usize) = (v as u32).swap_bytes();
    }
}
}

// ── `fdt_put_data` · riscv_machine.c:386-395 · chunk 2009 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_put_data(FDTState *s, const uint8_t *data, int len)
//     {
//         int len1;
//     
//         len1 = (len + 3) / 4;
//         fdt_alloc_len(s, s->tab_len + len1);
//         memcpy(s->tab + s->tab_len, data, len);
//         memset((uint8_t *)(s->tab + s->tab_len) + len, 0, -len & 3);
//         s->tab_len += len1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2009 {
use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// Ported from the static helper fdt_alloc_len (no linker symbol in C).
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let a = len;
        let b = (*s).tab_size.wrapping_mul(3) / 2;
        let new_size: c_int = if a > b { a } else { b };
        // C: realloc(s->tab, new_size * sizeof(uint32_t)) -- int converted to size_t
        let bytes = (new_size as isize as usize).wrapping_mul(4usize);
        let p = realloc((*s).tab as *mut c_void, bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

#[no_mangle]
pub extern "C" fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    unsafe {
        let len1: c_int = len.wrapping_add(3) / 4;
        fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
        let tab_len = (*s).tab_len;
        let dst: *mut u8 = ((*s).tab as *mut u8).wrapping_add((tab_len as isize as usize).wrapping_mul(4usize));
        ptr::copy_nonoverlapping(data, dst, len as usize);
        let pad: usize = (len.wrapping_neg() & 3) as usize;
        ptr::write_bytes(dst.wrapping_add(len as usize), 0u8, pad);
        (*s).tab_len = tab_len.wrapping_add(len1);
    }
}
}

// ── `int len1; len1 = (len + 3) / 4; fdt_alloc_len(s, s->tab_len + len1); memcpy(s->tab + s->tab_len…` · riscv_machine.c:388-394 · chunk 2010 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int len1;
//     
//         len1 = (len + 3) / 4;
//         fdt_alloc_len(s, s->tab_len + len1);
//         memcpy(s->tab + s->tab_len, data, len);
//         memset((uint8_t *)(s->tab + s->tab_len) + len, 0, -len & 3);
//         s->tab_len += len1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2010 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_put_data__part2010(
    s: *mut FDTState,
    data: *const u8,
    len: c_int,
) -> c_int {
    unsafe {
        let len1: c_int = len.wrapping_add(3) / 4;

        // Inlined static fdt_alloc_len(s, s->tab_len + len1)
        let want: c_int = (*s).tab_len.wrapping_add(len1);
        if want > (*s).tab_size {
            let a = want;
            let b = (*s).tab_size.wrapping_mul(3) / 2;
            let new_size: c_int = if a > b { a } else { b };
            let nbytes: usize = (new_size as usize).wrapping_mul(4);
            let p = realloc((*s).tab as *mut c_void, nbytes);
            (*s).tab = p as *mut u32;
            (*s).tab_size = new_size;
        }

        let base: usize = ((*s).tab as usize).wrapping_add(((*s).tab_len as usize).wrapping_mul(4));
        memcpy(base as *mut c_void, data as *const c_void, len as usize);
        let zbase: usize = base.wrapping_add(len as usize);
        memset(zbase as *mut c_void, 0, (len.wrapping_neg() & 3) as usize);
        (*s).tab_len += len1;

        len1
    }
}
}

// ── `fdt_begin_node` · riscv_machine.c:397-402 · chunk 2011 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_begin_node(FDTState *s, const char *name)
//     {
//         fdt_put32(s, 1);
//         fdt_put_data(s, (uint8_t *)name, strlen(name) + 1);
//         s->open_node_count++;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2011 {
// Rust rewrite of fdt_begin_node (static helper of riscv_machine.c).
// The C helpers fdt_put32 / fdt_put_data / fdt_alloc_len are `static` and thus
// have no linker symbol, so their logic is replicated here privately.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn c_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut n: usize = 0;
    while *s.add(n) != 0 {
        n += 1;
    }
    n
}

unsafe fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let bytes = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
        let p = realloc((*s).tab as *mut c_void, bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FdtState, v: c_int) {
    let tab_len = (*s).tab_len;
    fdt_alloc_len(s, tab_len.wrapping_add(1));
    let idx = (*s).tab_len as usize;
    // cpu_to_be32(v): __bswap_32 on the uint32_t value
    *(*s).tab.add(idx) = (v as u32).swap_bytes();
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
}

unsafe fn fdt_put_data(s: *mut FdtState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    let tab_len = (*s).tab_len;
    fdt_alloc_len(s, tab_len.wrapping_add(len1));

    let base = ((*s).tab as *mut u8).add(((*s).tab_len as usize).wrapping_mul(4));
    if len > 0 && !data.is_null() {
        core::ptr::copy_nonoverlapping(data, base, len as usize);
    }
    let pad = (len.wrapping_neg() & 3) as usize;
    if pad > 0 {
        core::ptr::write_bytes(base.add(len as usize), 0u8, pad);
    }
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

#[no_mangle]
pub extern "C" fn fdt_begin_node(s: *mut FdtState, name: *const c_char) {
    unsafe {
        if s.is_null() || name.is_null() {
            return;
        }
        fdt_put32(s, 1);
        let len = (c_strlen(name) as c_int).wrapping_add(1);
        fdt_put_data(s, name as *const u8, len);
        (*s).open_node_count = (*s).open_node_count.wrapping_add(1);
    }
}
}

// ── `fdt_put32(s, 1); fdt_put_data(s, (uint8_t *)name, strlen(name) + 1); s->open_node_count++;` · riscv_machine.c:399-401 · chunk 2012 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_put32(s, 1);
//         fdt_put_data(s, (uint8_t *)name, strlen(name) + 1);
//         s->open_node_count++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2012 {
use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
}

// ported from the static C helper max_int
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// ported from the static C helper fdt_alloc_len
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size * 3 / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize) * size_of::<u32>());
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

// ported from the static C helper fdt_put32
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len + 1);
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
    (*s).tab_len = idx + 1;
}

// ported from the static C helper fdt_put_data
unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = (len + 3) / 4;
    fdt_alloc_len(s, (*s).tab_len + len1);
    let dst = (*s).tab.offset((*s).tab_len as isize) as *mut u8;
    memcpy(dst as *mut c_void, data as *const c_void, len as usize);
    memset(
        dst.offset(len as isize) as *mut c_void,
        0,
        ((-len) & 3) as usize,
    );
    (*s).tab_len += len1;
}

#[no_mangle]
pub unsafe extern "C" fn fdt_begin_node__part2012(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, strlen(name) as c_int + 1);
    (*s).open_node_count += 1;
}
}

// ── `fdt_begin_node_num` · riscv_machine.c:404-409 · chunk 2013 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_begin_node_num(FDTState *s, const char *name, uint64_t n)
//     {
//         char buf[256];
//         snprintf(buf, sizeof(buf), "%s@%" 
//                                          "l" "x"
//                                                , name, n);
//         fdt_begin_node(s, buf);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2013 {
// Port of fdt_begin_node_num plus the static helpers it needs
// (fdt_begin_node, fdt_put_data, fdt_put32, fdt_alloc_len, max_int,
// cpu_to_be32), which have no linker symbols in the C program.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
    let base = (*s).tab.add((*s).tab_len as usize);
    memcpy(base as *mut c_void, data as *const c_void, len as usize);
    memset(
        (base as *mut u8).offset(len as isize) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as usize,
    );
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, (strlen(name) as c_int).wrapping_add(1));
    (*s).open_node_count = (*s).open_node_count.wrapping_add(1);
}

// Equivalent of snprintf(buf, sizeof(buf), "%s@%lx", name, n) into a
// 256-byte buffer (truncating to 255 chars + NUL), then fdt_begin_node.
#[no_mangle]
pub unsafe extern "C" fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0u8; 256];
    let mut len: usize = 0;

    let mut p = name as *const u8;
    while len < 255 && *p != 0 {
        buf[len] = *p;
        len += 1;
        p = p.add(1);
    }
    if len < 255 {
        buf[len] = b'@';
        len += 1;
    }

    let digits = b"0123456789abcdef";
    let mut tmp = [0u8; 16];
    let mut cnt: usize = 0;
    let mut v = n;
    if v == 0 {
        tmp[0] = b'0';
        cnt = 1;
    } else {
        while v != 0 && cnt < tmp.len() {
            tmp[cnt] = digits[(v & 0xf) as usize];
            cnt += 1;
            v >>= 4;
        }
    }
    let mut i = cnt;
    while i > 0 {
        i -= 1;
        if len < 255 {
            buf[len] = tmp[i];
            len += 1;
        }
    }
    buf[len] = 0;

    fdt_begin_node(s, buf.as_ptr() as *const c_char);
}
}

// ── `char buf[256]; snprintf(buf, sizeof(buf), "%s@%" "l" "x" , name, n); fdt_begin_node(s, buf);` · riscv_machine.c:406-408 · chunk 2014 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char buf[256];
//         snprintf(buf, sizeof(buf), "%s@%" 
//                                          "l" "x"
//                                                , name, n);
//         fdt_begin_node(s, buf);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2014 {
use core::ffi::c_char;

// Formats "<name>@<hex n>" into a fixed C buffer, mirroring
// snprintf(buf, buflen, "%s@%lx", name, n) byte for byte (including
// truncation and NUL termination).
#[no_mangle]
pub extern "C" fn fdt_begin_node_num__part2014(
    buf: *mut c_char,
    buflen: usize,
    name: *const c_char,
    n: u64,
) {
    unsafe {
        if buf.is_null() || buflen == 0 {
            return;
        }
        let cap = buflen - 1;
        let mut pos: usize = 0;

        if !name.is_null() {
            let mut i: usize = 0;
            loop {
                let raw = *name.add(i);
                if raw == 0 {
                    break;
                }
                let c = raw as u8;
                if pos < cap {
                    *buf.add(pos) = c as c_char;
                    pos += 1;
                } else {
                    break;
                }
                i += 1;
            }
        }

        if pos < cap {
            *buf.add(pos) = b'@' as c_char;
            pos += 1;
        }

        let mut digits = [0u8; 16];
        let mut nd: usize = 0;
        let mut v = n;
        if v == 0 {
            digits[0] = b'0';
            nd = 1;
        } else {
            while v != 0 {
                let d = (v & 0xf) as u8;
                digits[nd] = if d < 10 { b'0' + d } else { b'a' + (d - 10) };
                nd += 1;
                v >>= 4;
            }
        }
        while nd > 0 {
            nd -= 1;
            let ch = digits[nd];
            if pos < cap {
                *buf.add(pos) = ch as c_char;
                pos += 1;
            } else {
                nd = 0;
            }
        }

        *buf.add(pos) = 0;
    }
}
}

// ── `fdt_end_node` · riscv_machine.c:411-415 · chunk 2015 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_end_node(FDTState *s)
//     {
//         fdt_put32(s, 2);
//         s->open_node_count--;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2015 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// Ported from the static C helper fdt_alloc_len().
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let a = len;
        let b = (*s).tab_size.wrapping_mul(3) / 2;
        let new_size: c_int = if a > b { a } else { b };
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

// Ported from the static C helper fdt_put32().
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
    (*s).tab_len = idx.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count = (*s).open_node_count.wrapping_sub(1);
}
}

// ── `fdt_put32(s, 2); s->open_node_count--;` · riscv_machine.c:413-414 · chunk 2016 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     fdt_put32(s, 2);
//         s->open_node_count--;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2016 {
// Port of `fdt_put32(s, 2); s->open_node_count--;` from fdt_end_node.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// Inlined `fdt_put32` + `fdt_alloc_len` logic, preserving C integer semantics.
#[no_mangle]
pub extern "C" fn fdt_end_node__part2016(s: *mut FdtState) {
    unsafe {
        let st = &mut *s;

        // fdt_alloc_len(s, s->tab_len + 1)
        let len = st.tab_len.wrapping_add(1);
        if len > st.tab_size {
            let a = len;
            let b = st.tab_size.wrapping_mul(3) / 2;
            let new_size = if a > b { a } else { b };
            let bytes = (new_size as isize as usize).wrapping_mul(4);
            st.tab = realloc(st.tab as *mut c_void, bytes) as *mut u32;
            st.tab_size = new_size;
        }

        // s->tab[s->tab_len++] = cpu_to_be32(2);
        let idx = st.tab_len as usize;
        st.tab.add(idx).write(2u32.swap_bytes());
        st.tab_len = st.tab_len.wrapping_add(1);

        // s->open_node_count--;
        st.open_node_count = st.open_node_count.wrapping_sub(1);
    }
}
}

// ── `fdt_get_string_offset` · riscv_machine.c:417-439 · chunk 2017 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int fdt_get_string_offset(FDTState *s, const char *name)
//     {
//         int pos, new_size, name_size, new_len;
//     
//         pos = 0;
//         while (pos < s->string_table_len) {
//             if (!strcmp(s->string_table + pos, name))
//                 return pos;
//             pos += strlen(s->string_table + pos) + 1;
//         }
//     
//         name_size = strlen(name) + 1;
//         new_len = s->string_table_len + name_size;
//         if (new_len > s->string_table_size) {
//             new_size = max_int(new_len, s->string_table_size * 3 / 2);
//             s->string_table = realloc(s->string_table, new_size);
//             s->string_table_size = new_size;
//         }
//         pos = s->string_table_len;
//         memcpy(s->string_table + pos, name, name_size);
//         s->string_table_len = new_len;
//         return pos;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2017 {
// Rust rewrite of fdt_get_string_offset (riscv_machine.c)
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    unsafe {
        let mut pos: c_int = 0;
        while pos < (*s).string_table_len {
            let p = (*s).string_table.offset(pos as isize);
            if strcmp(p, name) == 0 {
                return pos;
            }
            pos += strlen(p) as c_int + 1;
        }

        // add a new string
        let name_size: c_int = strlen(name) as c_int + 1;
        let new_len: c_int = (*s).string_table_len.wrapping_add(name_size);
        if new_len > (*s).string_table_size {
            let a: c_int = new_len;
            let b: c_int = (*s).string_table_size.wrapping_mul(3) / 2;
            let new_size: c_int = if a > b { a } else { b };
            (*s).string_table =
                realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
            (*s).string_table_size = new_size;
        }
        pos = (*s).string_table_len;
        memcpy(
            (*s).string_table.offset(pos as isize) as *mut c_void,
            name as *const c_void,
            name_size as usize,
        );
        (*s).string_table_len = new_len;
        pos
    }
}
}

// ── `int pos, new_size, name_size, new_len; pos = 0;` · riscv_machine.c:419-421 · chunk 2018 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int pos, new_size, name_size, new_len;
//     
//         pos = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2018 {
// Port of the initialization part of fdt_get_string_offset:
// declares the locals and initializes the scan position.

#[no_mangle]
pub extern "C" fn fdt_get_string_offset__part2018() -> core::ffi::c_int {
    0
}
}

// ── `while (pos < s->string_table_len)` · riscv_machine.c:422-426 · chunk 2019 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (pos < s->string_table_len) {
//             if (!strcmp(s->string_table + pos, name))
//                 return pos;
//             pos += strlen(s->string_table + pos) + 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2019 {
// Port of the string-table search loop of fdt_get_string_offset.
// Returns 1 when the name was found (and stores its offset in *pos_out),
// 0 when the loop finished without a match (no early return in C).

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: core::ffi::c_int,
    pub tab_size: core::ffi::c_int,
    pub open_node_count: core::ffi::c_int,
    pub string_table: *mut core::ffi::c_char,
    pub string_table_len: core::ffi::c_int,
    pub string_table_size: core::ffi::c_int,
}

extern "C" {
    fn strcmp(__s1: *const core::ffi::c_char, __s2: *const core::ffi::c_char) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> usize;
}

#[no_mangle]
pub extern "C" fn fdt_get_string_offset__part2019(
    s: *mut FDTState,
    name: *const core::ffi::c_char,
    pos_out: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    unsafe {
        let len = (*s).string_table_len;
        let table = (*s).string_table;
        let mut pos: core::ffi::c_int = *pos_out;
        while pos < len {
            let p = table.offset(pos as isize);
            if strcmp(p, name) == 0 {
                *pos_out = pos;
                return 1;
            }
            // C: pos += strlen(...) + 1, with the usual truncating conversion back to int
            pos = pos
                .wrapping_add(strlen(p) as core::ffi::c_int)
                .wrapping_add(1);
        }
        *pos_out = pos;
        0
    }
}
}

// ── `name_size = strlen(name) + 1; new_len = s->string_table_len + name_size;` · riscv_machine.c:428-429 · chunk 2020 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     name_size = strlen(name) + 1;
//         new_len = s->string_table_len + name_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2020 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub extern "C" fn fdt_get_string_offset__part2020(
    s: *const FDTState,
    name: *const c_char,
    name_size: *mut c_int,
    new_len: *mut c_int,
) {
    unsafe {
        // name_size = strlen(name) + 1;  (size_t arithmetic, truncated to int)
        let ns: c_int = strlen(name).wrapping_add(1) as c_int;
        if !name_size.is_null() {
            *name_size = ns;
        }
        // new_len = s->string_table_len + name_size;
        if !new_len.is_null() {
            *new_len = (*s).string_table_len.wrapping_add(ns);
        }
    }
}
}

// ── `if (new_len > s->string_table_size)` · riscv_machine.c:430-434 · chunk 2021 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (new_len > s->string_table_size) {
//             new_size = max_int(new_len, s->string_table_size * 3 / 2);
//             s->string_table = realloc(s->string_table, new_size);
//             s->string_table_size = new_size;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2021 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_get_string_offset__part2021(s: *mut FDTState, new_len: c_int) {
    unsafe {
        let s = &mut *s;
        if new_len > s.string_table_size {
            // max_int(new_len, s->string_table_size * 3 / 2)
            let tmp = s.string_table_size.wrapping_mul(3) / 2;
            let new_size = if new_len > tmp { new_len } else { tmp };
            s.string_table = realloc(s.string_table as *mut c_void, new_size as usize) as *mut c_char;
            s.string_table_size = new_size;
        }
    }
}
}

// ── `pos = s->string_table_len; memcpy(s->string_table + pos, name, name_size); s->string_table_len…` · riscv_machine.c:435-438 · chunk 2022 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pos = s->string_table_len;
//         memcpy(s->string_table + pos, name, name_size);
//         s->string_table_len = new_len;
//         return pos;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2022 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fdt_get_string_offset__part2022(
    s: *mut FDTState,
    name: *const c_char,
    name_size: c_int,
    new_len: c_int,
) -> c_int {
    unsafe {
        let pos: c_int = (*s).string_table_len;
        memcpy(
            (*s).string_table.offset(pos as isize) as *mut c_void,
            name as *const c_void,
            name_size as usize,
        );
        (*s).string_table_len = new_len;
        pos
    }
}
}

// ── `fdt_prop` · riscv_machine.c:441-448 · chunk 2023 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop(FDTState *s, const char *prop_name,
//                          const void *data, int data_len)
//     {
//         fdt_put32(s, 3);
//         fdt_put32(s, data_len);
//         fdt_put32(s, fdt_get_string_offset(s, prop_name));
//         fdt_put_data(s, data, data_len);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2023 {
// Rust rewrite of `fdt_prop` from riscv_machine.c (FDT device-tree writer).
// The C helpers it calls (fdt_put32, fdt_put_data, fdt_get_string_offset) are
// `static` in the C translation unit, so they have no linker symbol: their
// logic is ported here as private Rust functions.

use core::ffi::{c_char, c_int, c_uint, c_void};

// Mirror of the C `FDTState` struct (exact fields, order and types).
#[repr(C)]
pub struct FDTState {
    pub tab: *mut c_uint,          // uint32_t *tab;
    pub tab_len: c_int,            // int tab_len;
    pub tab_size: c_int,           // int tab_size;
    pub open_node_count: c_int,    // int open_node_count;
    pub string_table: *mut c_char, // char *string_table;
    pub string_table_len: c_int,   // int string_table_len;
    pub string_table_size: c_int,  // int string_table_size;
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static inline uint32_t cpu_to_be32(uint32_t v) { return __bswap_32(v); }
#[inline]
fn cpu_to_be32(v: c_uint) -> c_uint {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<c_uint>()),
        ) as *mut c_uint;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = cpu_to_be32(v as c_uint);
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1: c_int = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
    let dst = (*s).tab.offset((*s).tab_len as isize) as *mut u8;
    memcpy(dst as *mut c_void, data as *const c_void, len as usize);
    memset(
        dst.offset(len as isize) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as usize,
    );
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let p = (*s).string_table.offset(pos as isize);
        if strcmp(p, name) == 0 {
            return pos;
        }
        pos = (pos as usize).wrapping_add(strlen(p).wrapping_add(1)) as c_int;
    }

    // add a new string
    let name_size: c_int = strlen(name).wrapping_add(1) as c_int;
    let new_len: c_int = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table =
            realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const c_void,
    data_len: c_int,
) {
    // FDT_PROP == 3
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data as *const u8, data_len);
}
}

// ── `fdt_put32(s, 3); fdt_put32(s, data_len); fdt_put32(s, fdt_get_string_offset(s, prop_name)); fdt…` · riscv_machine.c:444-447 · chunk 2024 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_put32(s, 3);
//         fdt_put32(s, data_len);
//         fdt_put32(s, fdt_get_string_offset(s, prop_name));
//         fdt_put_data(s, data, data_len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2024 {
// Port of riscv_machine.c fdt_prop body (lines 444-447, preprocessed 5924-5927).
// The C helpers fdt_put32 / fdt_get_string_offset / fdt_put_data and the
// static inline fdt_alloc_len / max_int are file-static, so their logic is
// mirrored here as private Rust functions.

use core::ffi::{c_char, c_int, c_void};

type SizeT = usize;

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: SizeT) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: SizeT) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: SizeT) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> SizeT;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc((*s).tab as *mut c_void, (new_size as SizeT).wrapping_mul(4)) as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
    *(*s).tab.add(idx as SizeT) = cpu_to_be32(v as u32);
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
    let dst = (*s).tab.add((*s).tab_len as SizeT);
    memcpy(dst as *mut c_void, data as *const c_void, len as SizeT);
    memset(
        (dst as *mut u8).add(len as SizeT) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as SizeT,
    );
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let cur = (*s).string_table.add(pos as SizeT);
        if strcmp(cur, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(cur) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table = realloc((*s).string_table as *mut c_void, new_size as SizeT) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.add(pos as SizeT) as *mut c_void,
        name as *const c_void,
        name_size as SizeT,
    );
    (*s).string_table_len = new_len;
    pos
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop__part2024(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const c_void,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data as *const u8, data_len);
}
}

// ── `fdt_prop_tab_u32` · riscv_machine.c:450-459 · chunk 2025 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop_tab_u32(FDTState *s, const char *prop_name,
//                                  uint32_t *tab, int tab_len)
//     {
//         int i;
//         fdt_put32(s, 3);
//         fdt_put32(s, tab_len * sizeof(uint32_t));
//         fdt_put32(s, fdt_get_string_offset(s, prop_name));
//         for(i = 0; i < tab_len; i++)
//             fdt_put32(s, tab[i]);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2025 {
// Port of the static C function fdt_prop_tab_u32() from riscv_machine.c.
// The helpers it used (fdt_put32, fdt_alloc_len, fdt_get_string_offset, max_int,
// cpu_to_be32) are static in the C translation unit and therefore have no linker
// symbols, so their logic is reproduced here as private Rust functions.

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C anonymous struct typedef FDTState.
#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<u32>()),
        ) as *mut u32;
        (*s).tab_size = new_size;
    }
}

// cpu_to_be32() is bswap32 on this target.
#[inline]
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if strcmp((*s).string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        let l = strlen((*s).string_table.offset(pos as isize));
        pos = (pos as usize).wrapping_add(l.wrapping_add(1)) as c_int;
    }

    let name_size: c_int = strlen(name).wrapping_add(1) as c_int;
    let new_len: c_int = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table = realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(
        s,
        (tab_len as usize).wrapping_mul(core::mem::size_of::<u32>()) as c_int,
    );
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i = i.wrapping_add(1);
    }
}
}

// ── `int i; fdt_put32(s, 3); fdt_put32(s, tab_len * sizeof(uint32_t)); fdt_put32(s, fdt_get_string_o…` · riscv_machine.c:453-456 · chunk 2026 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int i;
//         fdt_put32(s, 3);
//         fdt_put32(s, tab_len * sizeof(uint32_t));
//         fdt_put32(s, fdt_get_string_offset(s, prop_name));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2026 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if strcmp((*s).string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        pos += strlen((*s).string_table.offset(pos as isize)) as c_int + 1;
    }

    let name_size: c_int = strlen(name) as c_int + 1;
    let new_len: c_int = (*s).string_table_len + name_size;
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u32__part2026(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
}
}

// ── `for (i = 0; i < tab_len; i++)` · riscv_machine.c:457-458 · chunk 2027 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < tab_len; i++)
//             fdt_put32(s, tab[i]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2027 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// Ported from static fdt_alloc_len().
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let bytes = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
        let p = realloc((*s).tab as *mut c_void, bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

// Ported from static fdt_put32().
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    (*s).tab_len = idx.wrapping_add(1);
    // cpu_to_be32() is __bswap_32(): an unconditional byteswap.
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u32__part2027(
    s: *mut FDTState,
    tab: *mut u32,
    tab_len: c_int,
) {
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i += 1;
    }
}
}

// ── `fdt_prop_u32` · riscv_machine.c:461-464 · chunk 2028 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop_u32(FDTState *s, const char *prop_name, uint32_t val)
//     {
//         fdt_prop_tab_u32(s, prop_name, &val, 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2028 {
// Port of fdt_prop_u32 (and the static helpers it inlines) from riscv_machine.c
#![allow(non_camel_case_types, non_snake_case, unused)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// int -> size_t conversion in C sign-extends then reinterprets as unsigned.
#[inline]
fn int_to_usize(v: c_int) -> usize {
    (v as i64) as u64 as usize
}

// cpu_to_be32: unconditional byte swap (__builtin_bswap32).
#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    let st = &mut *s;
    if len > st.tab_size {
        let new_size = max_int(len, st.tab_size.wrapping_mul(3) / 2);
        let nbytes = int_to_usize(new_size).wrapping_mul(4);
        let p = realloc(st.tab as *mut c_void, nbytes);
        st.tab = p as *mut u32;
        st.tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    let st = &mut *s;
    fdt_alloc_len(s, st.tab_len.wrapping_add(1));
    let idx = st.tab_len as isize;
    *st.tab.offset(idx) = cpu_to_be32(v as u32);
    st.tab_len = st.tab_len.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let st = &mut *s;
    let mut pos: c_int = 0;
    while pos < st.string_table_len {
        let entry = st.string_table.offset(pos as isize);
        if strcmp(entry, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(entry) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = st.string_table_len.wrapping_add(name_size);
    if new_len > st.string_table_size {
        let new_size = max_int(new_len, st.string_table_size.wrapping_mul(3) / 2);
        let p = realloc(st.string_table as *mut c_void, int_to_usize(new_size));
        st.string_table = p as *mut c_char;
        st.string_table_size = new_size;
    }
    pos = st.string_table_len;
    memcpy(
        st.string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        int_to_usize(name_size),
    );
    st.string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_u32(s: *mut FDTState, prop_name: *const c_char, val: u32) {
    let mut v: u32 = val;
    fdt_prop_tab_u32(s, prop_name, &mut v as *mut u32, 1);
}
}

// ── `fdt_prop_tab_u32(s, prop_name, &val, 1);` · riscv_machine.c:463-463 · chunk 2029 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_prop_tab_u32(s, prop_name, &val, 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2029 {
// Port of the static helper chain used by fdt_prop_u32's call to
// fdt_prop_tab_u32 (fdt_prop_tab_u32, fdt_put32, fdt_alloc_len,
// fdt_get_string_offset).  All of those are `static` in C, so their logic is
// mirrored here as private Rust fns.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc((*s).tab as *mut c_void, (new_size as usize) * 4) as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let cur = (*s).string_table.offset(pos as isize);
        if strcmp(cur, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add(strlen(cur) as c_int + 1);
    }

    let name_size: c_int = (strlen(name) + 1) as c_int;
    let new_len: c_int = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table =
            realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *const u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn fdt_prop_u32__part2029(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *const u32,
    tab_len: c_int,
) {
    unsafe {
        fdt_prop_tab_u32(s, prop_name, tab, tab_len);
    }
}
}

// ── `fdt_prop_tab_u64` · riscv_machine.c:466-473 · chunk 2030 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop_tab_u64(FDTState *s, const char *prop_name,
//                                  uint64_t v0)
//     {
//         uint32_t tab[2];
//         tab[0] = v0 >> 32;
//         tab[1] = v0;
//         fdt_prop_tab_u32(s, prop_name, tab, 2);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2030 {
// Rust port of fdt_prop_tab_u64 (and the static helpers it needs:
// fdt_prop_tab_u32, fdt_put32, fdt_alloc_len, fdt_get_string_offset).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    let tab_size = (*s).tab_size;
    if len > tab_size {
        let new_size = max_int(len, tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: u32) {
    let len = (*s).tab_len.wrapping_add(1);
    fdt_alloc_len(s, len);
    let tl = (*s).tab_len;
    *(*s).tab.offset(tl as isize) = v.swap_bytes();
    (*s).tab_len = tl.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let entry = (*s).string_table.offset(pos as isize);
        if strcmp(entry, name) == 0 {
            return pos;
        }
        let add = strlen(entry).wrapping_add(1);
        pos = (pos as usize).wrapping_add(add) as c_int;
    }

    let name_size = (strlen(name) as usize).wrapping_add(1) as c_int;
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4) as u32);
    fdt_put32(s, fdt_get_string_offset(s, prop_name) as u32);
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize));
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u64(
    s: *mut FDTState,
    prop_name: *const c_char,
    v0: u64,
) {
    let mut tab: [u32; 2] = [0, 0];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 2);
}
}

// ── `uint32_t tab[2]; tab[0] = v0 >> 32; tab[1] = v0; fdt_prop_tab_u32(s, prop_name, tab, 2);` · riscv_machine.c:469-472 · chunk 2031 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t tab[2];
//         tab[0] = v0 >> 32;
//         tab[1] = v0;
//         fdt_prop_tab_u32(s, prop_name, tab, 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2031 {
// Port of fdt_prop_tab_u64's body (riscv_machine.c lines 469-472).
// fdt_prop_tab_u32 and its static helpers have no linker symbols, so their
// logic is reproduced here as private Rust functions.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

unsafe fn c_strlen(p: *const c_char) -> usize {
    let b = p as *const u8;
    let mut n: usize = 0;
    while *b.add(n) != 0 {
        n = n.wrapping_add(1);
    }
    n
}

unsafe fn c_strcmp(a: *const c_char, b: *const c_char) -> c_int {
    let a = a as *const u8;
    let b = b as *const u8;
    let mut i: usize = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb {
            return (ca as c_int) - (cb as c_int);
        }
        if ca == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
}

// static void fdt_alloc_len(FDTState *s, int len)
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let a = len;
        let b = (*s).tab_size.wrapping_mul(3) / 2;
        let new_size = if a > b { a } else { b };
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

// static void fdt_put32(FDTState *s, int v)
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
    *(*s).tab.offset(idx as isize) = (v as u32).swap_bytes();
}

// static int fdt_get_string_offset(FDTState *s, const char *name)
unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let p = (*s).string_table.offset(pos as isize);
        if c_strcmp(p, name) == 0 {
            return pos;
        }
        pos = (pos as usize).wrapping_add(c_strlen(p).wrapping_add(1)) as c_int;
    }

    let name_size = c_strlen(name).wrapping_add(1) as c_int;
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let b = (*s).string_table_size.wrapping_mul(3) / 2;
        let new_size = if new_len > b { new_len } else { b };
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    core::ptr::copy_nonoverlapping(
        name as *const u8,
        (*s).string_table.offset(pos as isize) as *mut u8,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

// static void fdt_prop_tab_u32(FDTState *s, const char *prop_name,
//                              uint32_t *tab, int tab_len)
unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u64__part2031(
    s: *mut FDTState,
    prop_name: *const c_char,
    v0: u64,
) {
    let mut tab: [u32; 2] = [0, 0];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 2);
}
}

// ── `fdt_prop_tab_u64_2` · riscv_machine.c:475-484 · chunk 2032 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop_tab_u64_2(FDTState *s, const char *prop_name,
//                                    uint64_t v0, uint64_t v1)
//     {
//         uint32_t tab[4];
//         tab[0] = v0 >> 32;
//         tab[1] = v0;
//         tab[2] = v1 >> 32;
//         tab[3] = v1;
//         fdt_prop_tab_u32(s, prop_name, tab, 4);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2032 {
// Port of fdt_prop_tab_u64_2 (riscv_machine.c) and the static helpers it needs.
// The helpers (fdt_prop_tab_u32, fdt_put32, fdt_get_string_offset,
// fdt_alloc_len, max_int, cpu_to_be32) are `static` in the C translation unit,
// so they have no linker symbol and are reimplemented here.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

#[inline]
unsafe fn cstr_len(p: *const c_char) -> c_int {
    let mut i: isize = 0;
    while *p.offset(i) != 0 {
        i += 1;
    }
    i as c_int
}

#[inline]
unsafe fn cstr_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: isize = 0;
    loop {
        let ca = *a.offset(i) as u8;
        let cb = *b.offset(i) as u8;
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let cur = (*s).string_table.offset(pos as isize);
        if cstr_eq(cur, name) {
            return pos;
        }
        pos = pos
            .wrapping_add(cstr_len(cur).wrapping_add(1));
    }

    let name_size = cstr_len(name).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    core::ptr::copy_nonoverlapping(
        name as *const u8,
        (*s).string_table.offset(pos as isize) as *mut u8,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, ((tab_len as usize).wrapping_mul(4)) as c_int);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u64_2(
    s: *mut FDTState,
    prop_name: *const c_char,
    v0: u64,
    v1: u64,
) {
    let mut tab: [u32; 4] = [0; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 4);
}
}

// ── `uint32_t tab[4]; tab[0] = v0 >> 32; tab[1] = v0; tab[2] = v1 >> 32; tab[3] = v1; fdt_prop_tab_u…` · riscv_machine.c:478-483 · chunk 2033 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t tab[4];
//         tab[0] = v0 >> 32;
//         tab[1] = v0;
//         tab[2] = v1 >> 32;
//         tab[3] = v1;
//         fdt_prop_tab_u32(s, prop_name, tab, 4);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2033 {
// Port of the body of fdt_prop_tab_u64_2 (riscv_machine.c) plus the
// static helpers it needs (fdt_prop_tab_u32, fdt_put32, fdt_alloc_len,
// fdt_get_string_offset), which have no linker symbols.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4)) as *mut u32;
        (*s).tab = p;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let entry = (*s).string_table.offset(pos as isize);
        if strcmp(entry, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(entry) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table = p;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_tab_u64_2__part2033(
    s: *mut FDTState,
    prop_name: *const c_char,
    v0: u64,
    v1: u64,
) {
    let mut tab: [u32; 4] = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 4);
}
}

// ── `fdt_prop_str` · riscv_machine.c:486-490 · chunk 2034 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void fdt_prop_str(FDTState *s, const char *prop_name,
//                              const char *str)
//     {
//         fdt_prop(s, prop_name, str, strlen(str) + 1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2034 {
// Rust rewrite of the static C function fdt_prop_str() from riscv_machine.c
//
// fdt_prop() (called by the target) is a `static` C function, so it has no
// linker symbol; its logic (plus the static helpers it uses: fdt_put32,
// fdt_get_string_offset, fdt_put_data, fdt_alloc_len, max_int, cpu_to_be32)
// is ported here as private Rust functions.
#![allow(non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static inline uint32_t cpu_to_be32(uint32_t v) { return __bswap_32(v); }
#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<u32>()),
        );
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len as usize;
    *(*s).tab.add(idx) = cpu_to_be32(v as u32);
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));

    let dst = (*s).tab.add((*s).tab_len as usize);
    memcpy(dst as *mut c_void, data as *const c_void, len as usize);
    memset(
        (dst as *mut u8).add(len as usize) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as usize,
    );
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if strcmp((*s).string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen((*s).string_table.offset(pos as isize)) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const c_void,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data as *const u8, data_len);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_str(
    s: *mut FDTState,
    prop_name: *const c_char,
    str_: *const c_char,
) {
    let data_len = (strlen(str_) as c_int).wrapping_add(1);
    fdt_prop(s, prop_name, str_ as *const c_void, data_len);
}
}

// ── `fdt_prop(s, prop_name, str, strlen(str) + 1);` · riscv_machine.c:489-489 · chunk 2035 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_prop(s, prop_name, str, strlen(str) + 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2035 {
// Port of the single statement in fdt_prop_str:
//     fdt_prop(s, prop_name, str, strlen(str) + 1);
// fdt_prop and the helpers it uses are static C functions (no linker symbol),
// so their logic is mirrored here as private Rust fns.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// static inline int max_int(int a, int b)
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static inline uint32_t cpu_to_be32(uint32_t v) -> __bswap_32(v)
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

// static void fdt_alloc_len(FDTState *s, int len)
unsafe fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    let st = &mut *s;
    if len > st.tab_size {
        let new_size = max_int(len, st.tab_size.wrapping_mul(3) / 2);
        st.tab = realloc(st.tab as *mut c_void, (new_size as usize).wrapping_mul(4)) as *mut u32;
        st.tab_size = new_size;
    }
}

// static void fdt_put32(FDTState *s, int v)
unsafe fn fdt_put32(s: *mut FdtState, v: c_int) {
    let st = &mut *s;
    fdt_alloc_len(s, st.tab_len.wrapping_add(1));
    let st = &mut *s;
    let idx = st.tab_len;
    *st.tab.offset(idx as isize) = cpu_to_be32(v as u32);
    st.tab_len = idx.wrapping_add(1);
}

// static void fdt_put_data(FDTState *s, const uint8_t *data, int len)
unsafe fn fdt_put_data(s: *mut FdtState, data: *const u8, len: c_int) {
    let st = &mut *s;
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, st.tab_len.wrapping_add(len1));
    let st = &mut *s;
    let base = st.tab.offset(st.tab_len as isize);
    memcpy(base as *mut c_void, data as *const c_void, len as usize);
    let tail = (base as *mut u8).offset(len as isize);
    memset(tail as *mut c_void, 0, (len.wrapping_neg() & 3) as usize);
    st.tab_len = st.tab_len.wrapping_add(len1);
}

// static int fdt_get_string_offset(FDTState *s, const char *name)
unsafe fn fdt_get_string_offset(s: *mut FdtState, name: *const c_char) -> c_int {
    let st = &mut *s;
    let mut pos: c_int = 0;
    while pos < st.string_table_len {
        if strcmp(st.string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        let l = strlen(st.string_table.offset(pos as isize));
        pos = (pos as usize).wrapping_add(l).wrapping_add(1) as c_int;
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = st.string_table_len.wrapping_add(name_size);
    if new_len > st.string_table_size {
        let new_size = max_int(new_len, st.string_table_size.wrapping_mul(3) / 2);
        st.string_table = realloc(st.string_table as *mut c_void, new_size as usize) as *mut c_char;
        st.string_table_size = new_size;
    }
    pos = st.string_table_len;
    memcpy(
        st.string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    st.string_table_len = new_len;
    pos
}

// static void fdt_prop(FDTState *s, const char *prop_name, const void *data, int data_len)
unsafe fn fdt_prop(s: *mut FdtState, prop_name: *const c_char, data: *const c_void, data_len: c_int) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
    fdt_put_data(s, data as *const u8, data_len);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_prop_str__part2035(
    s: *mut FdtState,
    prop_name: *const c_char,
    str_: *const c_char,
) {
    let data_len = (strlen(str_) as c_int).wrapping_add(1);
    fdt_prop(s, prop_name, str_ as *const c_void, data_len);
}
}

// ── `fdt_output` · riscv_machine.c:529-579 · chunk 2037 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int fdt_output(FDTState *s, uint8_t *dst)
//     {
//         struct fdt_header *h;
//         struct fdt_reserve_entry *re;
//         int dt_struct_size;
//         int dt_strings_size;
//         int pos;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        s->open_node_count == 0
//        )), __extension__ ({ if (
//        s->open_node_count == 0
//        ) ; else __assert_fail (
//        "s->open_node_count == 0"
//        , "riscv_machine.c", 537, __extension__ __PRETTY_FUNCTION__); }))
//                                       ;
//     
//         fdt_put32(s, 9);
//     
//         dt_struct_size = s->tab_len * sizeof(uint32_t);
//         dt_strings_size = s->string_table_len;
//     
//         h = (struct fdt_header *)dst;
//         h->magic = cpu_to_be32(0xd00dfeed);
//         h->version = cpu_to_be32(17);
//         h->last_comp_version = cpu_to_be32(16);
//         h->boot_cpuid_phys = cpu_to_be32(0);
//         h->size_dt_strings = cpu_to_be32(dt_strings_size);
//         h->size_dt_struct = cpu_to_be32(dt_struct_size);
//     
//         pos = sizeof(struct fdt_header);
//     
//         h->off_dt_struct = cpu_to_be32(pos);
//         memcpy(dst + pos, s->tab, dt_struct_size);
//         pos += dt_struct_size;
//     
//     
//         while ((pos & 7) != 0) {
//             dst[pos++] = 0;
//         }
//         h->off_mem_rsvmap = cpu_to_be32(pos);
//         re = (struct fdt_reserve_entry *)(dst + pos);
//         re->address = 0;
//         re->size = 0;
//         pos += sizeof(struct fdt_reserve_entry);
//     
//         h->off_dt_strings = cpu_to_be32(pos);
//         memcpy(dst + pos, s->string_table, dt_strings_size);
//         pos += dt_strings_size;
//     
//     
//         while ((pos & 7) != 0) {
//             dst[pos++] = 0;
//         }
//     
//         h->totalsize = cpu_to_be32(pos);
//         return pos;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2037 {
// Rust rewrite of fdt_output() from riscv_machine.c
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

#[repr(C)]
struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

#[repr(C)]
struct FdtReserveEntry {
    address: u64,
    size: u64,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: u32,
        function: *const c_char,
    ) -> !;
}

static ASSERT_MSG: &[u8] = b"s->open_node_count == 0\0";
static ASSERT_FILE: &[u8] = b"riscv_machine.c\0";
static ASSERT_FUNC: &[u8] = b"fdt_output\0";

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

// ported from the static fdt_alloc_len()
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let new_bytes = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
        let p = realloc((*s).tab as *mut c_void, new_bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// ported from the static fdt_put32()
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len as usize;
    let tab = (*s).tab;
    tab.add(idx).write_unaligned(cpu_to_be32(v as u32));
    (*s).tab_len = (*s).tab_len.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_output(s: *mut FDTState, dst: *mut u8) -> c_int {
    let mut h: *mut FdtHeader;
    let mut re: *mut FdtReserveEntry;
    let dt_struct_size: c_int;
    let dt_strings_size: c_int;
    let mut pos: c_int;

    if !((*s).open_node_count == 0) {
        __assert_fail(
            ASSERT_MSG.as_ptr() as *const c_char,
            ASSERT_FILE.as_ptr() as *const c_char,
            537,
            ASSERT_FUNC.as_ptr() as *const c_char,
        );
    }

    fdt_put32(s, 9);

    dt_struct_size = ((*s).tab_len as u32).wrapping_mul(core::mem::size_of::<u32>() as u32) as c_int;
    dt_strings_size = (*s).string_table_len;

    h = dst as *mut FdtHeader;
    (*h).magic = cpu_to_be32(0xd00d_feedu32);
    (*h).version = cpu_to_be32(17);
    (*h).last_comp_version = cpu_to_be32(16);
    (*h).boot_cpuid_phys = cpu_to_be32(0);
    (*h).size_dt_strings = cpu_to_be32(dt_strings_size as u32);
    (*h).size_dt_struct = cpu_to_be32(dt_struct_size as u32);

    pos = core::mem::size_of::<FdtHeader>() as c_int;

    (*h).off_dt_struct = cpu_to_be32(pos as u32);
    memcpy(
        dst.offset(pos as isize) as *mut c_void,
        (*s).tab as *const c_void,
        dt_struct_size as usize,
    );
    pos = pos.wrapping_add(dt_struct_size);

    // align to 8
    while (pos & 7) != 0 {
        *dst.offset(pos as isize) = 0;
        pos += 1;
    }
    (*h).off_mem_rsvmap = cpu_to_be32(pos as u32);
    re = dst.offset(pos as isize) as *mut FdtReserveEntry;
    (*re).address = 0; // no reserved entry
    (*re).size = 0;
    pos = pos.wrapping_add(core::mem::size_of::<FdtReserveEntry>() as c_int);

    (*h).off_dt_strings = cpu_to_be32(pos as u32);
    memcpy(
        dst.offset(pos as isize) as *mut c_void,
        (*s).string_table as *const c_void,
        dt_strings_size as usize,
    );
    pos = pos.wrapping_add(dt_strings_size);

    // align to 8, just in case
    while (pos & 7) != 0 {
        *dst.offset(pos as isize) = 0;
        pos += 1;
    }

    (*h).totalsize = cpu_to_be32(pos as u32);
    pos
}
}

// ── `struct fdt_header *h; struct fdt_reserve_entry *re; int dt_struct_size; int dt_strings_size; in…` · riscv_machine.c:531-556 · chunk 2038 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     struct fdt_header *h;
//         struct fdt_reserve_entry *re;
//         int dt_struct_size;
//         int dt_strings_size;
//         int pos;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        s->open_node_count == 0
//        )), __extension__ ({ if (
//        s->open_node_count == 0
//        ) ; else __assert_fail (
//        "s->open_node_count == 0"
//        , "riscv_machine.c", 537, __extension__ __PRETTY_FUNCTION__); }))
//                                       ;
//     
//         fdt_put32(s, 9);
//     
//         dt_struct_size = s->tab_len * sizeof(uint32_t);
//         dt_strings_size = s->string_table_len;
//     
//         h = (struct fdt_header *)dst;
//         h->magic = cpu_to_be32(0xd00dfeed);
//         h->version = cpu_to_be32(17);
//         h->last_comp_version = cpu_to_be32(16);
//         h->boot_cpuid_phys = cpu_to_be32(0);
//         h->size_dt_strings = cpu_to_be32(dt_strings_size);
//         h->size_dt_struct = cpu_to_be32(dt_struct_size);
//     
//         pos = sizeof(struct fdt_header);
//     
//         h->off_dt_struct = cpu_to_be32(pos);
//         memcpy(dst + pos, s->tab, dt_struct_size);
//         pos += dt_struct_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2038 {
// Port of the first part of fdt_output() from riscv_machine.c.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct FdtHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[repr(C)]
pub struct FdtReserveEntry {
    pub address: u64,
    pub size: u64,
}

#[repr(C)]
pub struct FdtState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// static inline uint32_t cpu_to_be32(uint32_t v) { return __bswap_32(v); }
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

// static inline int max_int(int a, int b)
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static void fdt_alloc_len(FDTState *s, int len)
fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    unsafe {
        if len > (*s).tab_size {
            let new_size: c_int = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
            let bytes: usize = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
            let p = realloc((*s).tab as *mut c_void, bytes);
            (*s).tab = p as *mut u32;
            (*s).tab_size = new_size;
        }
    }
}

// static void fdt_put32(FDTState *s, int v)
fn fdt_put32(s: *mut FdtState, v: c_int) {
    unsafe {
        fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
        let idx = (*s).tab_len;
        (*s).tab_len = idx.wrapping_add(1);
        *(*s).tab.add(idx as usize) = cpu_to_be32(v as u32);
    }
}

#[no_mangle]
pub extern "C" fn fdt_output__part2038(
    s: *mut FdtState,
    dst: *mut u8,
    h_out: *mut *mut FdtHeader,
    pos_out: *mut c_int,
    dt_strings_size_out: *mut c_int,
) {
    unsafe {
        if (*s).open_node_count != 0 {
            __assert_fail(
                b"s->open_node_count == 0\0".as_ptr() as *const c_char,
                b"riscv_machine.c\0".as_ptr() as *const c_char,
                537 as c_uint,
                b"fdt_output\0".as_ptr() as *const c_char,
            );
        }

        fdt_put32(s, 9);

        let dt_struct_size: c_int = (*s).tab_len.wrapping_mul(core::mem::size_of::<u32>() as c_int);
        let dt_strings_size: c_int = (*s).string_table_len;

        let h: *mut FdtHeader = dst as *mut FdtHeader;
        (*h).magic = cpu_to_be32(0xd00dfeed);
        (*h).version = cpu_to_be32(17);
        (*h).last_comp_version = cpu_to_be32(16);
        (*h).boot_cpuid_phys = cpu_to_be32(0);
        (*h).size_dt_strings = cpu_to_be32(dt_strings_size as u32);
        (*h).size_dt_struct = cpu_to_be32(dt_struct_size as u32);

        let mut pos: c_int = core::mem::size_of::<FdtHeader>() as c_int;

        (*h).off_dt_struct = cpu_to_be32(pos as u32);
        memcpy(
            dst.add(pos as usize) as *mut c_void,
            (*s).tab as *const c_void,
            dt_struct_size as usize,
        );
        pos = pos.wrapping_add(dt_struct_size);

        *h_out = h;
        *pos_out = pos;
        *dt_strings_size_out = dt_strings_size;
    }
}
}

// ── `while (pos & 7) != 0)` · riscv_machine.c:559-561 · chunk 2039 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((pos & 7) != 0) {
//             dst[pos++] = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2039 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fdt_output__part2039(dst: *mut u8, pos: c_int) -> c_int {
    let mut pos = pos;
    unsafe {
        while (pos & 7) != 0 {
            *dst.offset(pos as isize) = 0u8;
            pos += 1;
        }
    }
    pos
}
}

// ── `h->off_mem_rsvmap = cpu_to_be32(pos); re = (struct fdt_reserve_entry *)(dst + pos); re->address…` · riscv_machine.c:562-570 · chunk 2040 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     h->off_mem_rsvmap = cpu_to_be32(pos);
//         re = (struct fdt_reserve_entry *)(dst + pos);
//         re->address = 0;
//         re->size = 0;
//         pos += sizeof(struct fdt_reserve_entry);
//     
//         h->off_dt_strings = cpu_to_be32(pos);
//         memcpy(dst + pos, s->string_table, dt_strings_size);
//         pos += dt_strings_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2040 {
// Port of the mem-rsvmap + dt-strings emission part of fdt_output().
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct fdt_reserve_entry {
    pub address: u64,
    pub size: u64,
}

#[repr(C)]
pub struct fdt_header {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

// static inline uint32_t cpu_to_be32(uint32_t v) { return __bswap_32(v); }
#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

#[no_mangle]
pub unsafe extern "C" fn fdt_output__part2040(
    s: *mut FDTState,
    dst: *mut u8,
    dt_strings_size: c_int,
    pos: *mut c_int,
    re_out: *mut *mut fdt_reserve_entry,
) {
    let mut p: c_int = ptr::read(pos);

    let h = dst as *mut fdt_header;

    ptr::write_unaligned(ptr::addr_of_mut!((*h).off_mem_rsvmap), cpu_to_be32(p as u32));
    let re = dst.wrapping_offset(p as isize) as *mut fdt_reserve_entry;
    ptr::write_unaligned(ptr::addr_of_mut!((*re).address), 0u64);
    ptr::write_unaligned(ptr::addr_of_mut!((*re).size), 0u64);
    if !re_out.is_null() {
        ptr::write(re_out, re);
    }
    p = p.wrapping_add(size_of::<fdt_reserve_entry>() as c_int);

    ptr::write_unaligned(ptr::addr_of_mut!((*h).off_dt_strings), cpu_to_be32(p as u32));
    ptr::copy_nonoverlapping(
        (*s).string_table as *const u8,
        dst.wrapping_offset(p as isize),
        dt_strings_size as usize,
    );
    p = p.wrapping_add(dt_strings_size);

    ptr::write(pos, p);
}
}

// ── `while (pos & 7) != 0)` · riscv_machine.c:573-575 · chunk 2041 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while ((pos & 7) != 0) {
//             dst[pos++] = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2041 {
use core::ffi::c_int;

// Zero-pads dst at the current position up to the next 4/8-byte alignment,
// returning the updated position.
#[no_mangle]
pub extern "C" fn fdt_output__part2041(dst: *mut u8, pos: c_int) -> c_int {
    let mut pos = pos;
    unsafe {
        while (pos & 7) != 0 {
            *dst.offset(pos as isize) = 0u8;
            pos += 1;
        }
    }
    pos
}
}

// ── `h->totalsize = cpu_to_be32(pos); return pos;` · riscv_machine.c:577-578 · chunk 2042 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     h->totalsize = cpu_to_be32(pos);
//         return pos;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2042 {
// Port of the tail of fdt_output(): store the big-endian total size into the
// FDT header and yield the final size.

#[repr(C)]
pub struct FdtHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[no_mangle]
pub extern "C" fn fdt_output__part2042(h: *mut FdtHeader, pos: core::ffi::c_int) -> core::ffi::c_int {
    unsafe {
        if !h.is_null() {
            // h->totalsize = cpu_to_be32(pos);
            (*h).totalsize = (pos as u32).swap_bytes();
        }
    }
    pos
}
}

// ── `fdt_end` · riscv_machine.c:581-586 · chunk 2043 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void fdt_end(FDTState *s)
//     {
//         free(s->tab);
//         free(s->string_table);
//         free(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2043 {
// Port of `void fdt_end(FDTState *s)` from riscv_machine.c.

use core::ffi::{c_char, c_int};

/// Mirror of the C `FDTState` struct.
#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn free(ptr: *mut core::ffi::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_end(s: *mut FDTState) {
    if s.is_null() {
        return;
    }
    free((*s).tab as *mut core::ffi::c_void);
    free((*s).string_table as *mut core::ffi::c_void);
    free(s as *mut core::ffi::c_void);
}
}

// ── `free(s->tab); free(s->string_table); free(s);` · riscv_machine.c:583-585 · chunk 2044 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(s->tab);
//         free(s->string_table);
//         free(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2044 {
// Port of the tail of fdt_end: frees the tab, the string table and the state.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fdt_end__part2044(s: *mut FDTState) {
    let tab = (*s).tab;
    let string_table = (*s).string_table;
    free(tab as *mut c_void);
    free(string_table as *mut c_void);
    free(s as *mut c_void);
}
}

// ── `riscv_build_fdt` · riscv_machine.c:588-752 · chunk 2045 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int riscv_build_fdt(RISCVMachine *m, uint8_t *dst,
//                                uint64_t kernel_start, uint64_t kernel_size,
//                                const char *cmd_line,
//                                uint64_t initrd_start, uint64_t initrd_size)
//     {
//         FDTState *s;
//         int size, max_xlen, i, cur_phandle, intc_phandle, plic_phandle;
//         char isa_string[128], *q;
//         uint32_t misa;
//         uint32_t tab[4];
//         FBDevice *fb_dev;
//     
//         s = fdt_init();
//     
//         cur_phandle = 1;
//     
//         fdt_begin_node(s, "");
//         fdt_prop_u32(s, "#address-cells", 2);
//         fdt_prop_u32(s, "#size-cells", 2);
//         fdt_prop_str(s, "compatible", "ucbbar,riscvemu-bar_dev");
//         fdt_prop_str(s, "model", "ucbbar,riscvemu-bare");
//     
//     
//         fdt_begin_node(s, "cpus");
//         fdt_prop_u32(s, "#address-cells", 1);
//         fdt_prop_u32(s, "#size-cells", 0);
//         fdt_prop_u32(s, "timebase-frequency", 10000000);
//     
//     
//         fdt_begin_node_num(s, "cpu", 0);
//         fdt_prop_str(s, "device_type", "cpu");
//         fdt_prop_u32(s, "reg", 0);
//         fdt_prop_str(s, "status", "okay");
//         fdt_prop_str(s, "compatible", "riscv");
//     
//         max_xlen = m->max_xlen;
//         misa = riscv_cpu_get_misa(m->cpu_state);
//         q = isa_string;
//         q += snprintf(isa_string, sizeof(isa_string), "rv%d", max_xlen);
//         for(i = 0; i < 26; i++) {
//             if (misa & (1 << i))
//                 *q++ = 'a' + i;
//         }
//         *q = '\0';
//         fdt_prop_str(s, "riscv,isa", isa_string);
//     
//         fdt_prop_str(s, "mmu-type", max_xlen <= 32 ? "riscv,sv32" : "riscv,sv48");
//         fdt_prop_u32(s, "clock-frequency", 2000000000);
//     
//         fdt_begin_node(s, "interrupt-controller");
//         fdt_prop_u32(s, "#interrupt-cells", 1);
//         fdt_prop(s, "interrupt-controller", 
//                                            ((void *)0)
//                                                , 0);
//         fdt_prop_str(s, "compatible", "riscv,cpu-intc");
//         intc_phandle = cur_phandle++;
//         fdt_prop_u32(s, "phandle", intc_phandle);
//         fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node_num(s, "memory", 0x80000000);
//         fdt_prop_str(s, "device_type", "memory");
//         tab[0] = (uint64_t)0x80000000 >> 32;
//         tab[1] = 0x80000000;
//         tab[2] = m->ram_size >> 32;
//         tab[3] = m->ram_size;
//         fdt_prop_tab_u32(s, "reg", tab, 4);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node(s, "htif");
//         fdt_prop_str(s, "compatible", "ucb,htif0");
//         fdt_end_node(s);
//     
//         fdt_begin_node(s, "soc");
//         fdt_prop_u32(s, "#address-cells", 2);
//         fdt_prop_u32(s, "#size-cells", 2);
//         fdt_prop_tab_str(s, "compatible",
//                          "ucbbar,riscvemu-bar-soc", "simple-bus", 
//                                                                  ((void *)0)
//                                                                      );
//         fdt_prop(s, "ranges", 
//                              ((void *)0)
//                                  , 0);
//     
//         fdt_begin_node_num(s, "clint", 0x02000000);
//         fdt_prop_str(s, "compatible", "riscv,clint0");
//     
//         tab[0] = intc_phandle;
//         tab[1] = 3;
//         tab[2] = intc_phandle;
//         tab[3] = 7;
//         fdt_prop_tab_u32(s, "interrupts-extended", tab, 4);
//     
//         fdt_prop_tab_u64_2(s, "reg", 0x02000000, 0x000c0000);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node_num(s, "plic", 0x40100000);
//         fdt_prop_u32(s, "#interrupt-cells", 1);
//         fdt_prop(s, "interrupt-controller", 
//                                            ((void *)0)
//                                                , 0);
//         fdt_prop_str(s, "compatible", "riscv,plic0");
//         fdt_prop_u32(s, "riscv,ndev", 31);
//         fdt_prop_tab_u64_2(s, "reg", 0x40100000, 0x00400000);
//     
//         tab[0] = intc_phandle;
//         tab[1] = 9;
//         tab[2] = intc_phandle;
//         tab[3] = 11;
//         fdt_prop_tab_u32(s, "interrupts-extended", tab, 4);
//     
//         plic_phandle = cur_phandle++;
//         fdt_prop_u32(s, "phandle", plic_phandle);
//     
//         fdt_end_node(s);
//     
//         for(i = 0; i < m->virtio_count; i++) {
//             fdt_begin_node_num(s, "virtio", 0x40010000 + i * 0x1000);
//             fdt_prop_str(s, "compatible", "virtio,mmio");
//             fdt_prop_tab_u64_2(s, "reg", 0x40010000 + i * 0x1000,
//                                0x1000);
//             tab[0] = plic_phandle;
//             tab[1] = 1 + i;
//             fdt_prop_tab_u32(s, "interrupts-extended", tab, 2);
//             fdt_end_node(s);
//         }
//     
//         fb_dev = m->common.fb_dev;
//         if (fb_dev) {
//             fdt_begin_node_num(s, "framebuffer", 0x41000000);
//             fdt_prop_str(s, "compatible", "simple-framebuffer");
//             fdt_prop_tab_u64_2(s, "reg", 0x41000000, fb_dev->fb_size);
//             fdt_prop_u32(s, "width", fb_dev->width);
//             fdt_prop_u32(s, "height", fb_dev->height);
//             fdt_prop_u32(s, "stride", fb_dev->stride);
//             fdt_prop_str(s, "format", "a8r8g8b8");
//             fdt_end_node(s);
//         }
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node(s, "chosen");
//         fdt_prop_str(s, "bootargs", cmd_line ? cmd_line : "");
//         if (kernel_size > 0) {
//             fdt_prop_tab_u64(s, "riscv,kernel-start", kernel_start);
//             fdt_prop_tab_u64(s, "riscv,kernel-end", kernel_start + kernel_size);
//         }
//         if (initrd_size > 0) {
//             fdt_prop_tab_u64(s, "linux,initrd-start", initrd_start);
//             fdt_prop_tab_u64(s, "linux,initrd-end", initrd_start + initrd_size);
//         }
//     
//         fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         size = fdt_output(s, dst);
//         fdt_end(s);
//         return size;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2045 {
// Rust rewrite of riscv_build_fdt (riscv_machine.c)
// The FDT helper functions (fdt_init, fdt_put32, fdt_put_data, fdt_begin_node,
// fdt_begin_node_num, fdt_end_node, fdt_get_string_offset, fdt_prop*,
// fdt_output) are static in the C file, so their logic is ported here as
// private Rust functions.

use core::ffi::{c_char, c_int, c_void};
use std::ffi::CStr;

const FDT_MAGIC: u32 = 0xd00d_feed;
const FDT_VERSION: u32 = 17;

const RAM_BASE_ADDR: u32 = 0x8000_0000;
const CLINT_BASE_ADDR: u64 = 0x0200_0000;
const CLINT_SIZE: u64 = 0x000c_0000;
const VIRTIO_BASE_ADDR: i32 = 0x4001_0000;
const VIRTIO_SIZE: u64 = 0x1000;
const VIRTIO_IRQ: i32 = 1;
const PLIC_BASE_ADDR: u64 = 0x4010_0000;
const PLIC_SIZE: u64 = 0x0040_0000;
const FRAMEBUFFER_BASE_ADDR: u64 = 0x4100_0000;
const RTC_FREQ: u32 = 10_000_000;

// ---------------------------------------------------------------------------
// Mirrors of the C types accessed through pointers.
// ---------------------------------------------------------------------------

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct FBDevice {
    width: c_int,
    height: c_int,
    stride: c_int,
    fb_data: *mut u8,
    fb_size: c_int,
    device_opaque: *mut c_void,
    refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, *mut c_void)>,
}

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut FBDevice,
}

#[repr(C)]
struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
    plic_irq: [IRQSignal; 32],
    htif_tohost: u64,
    htif_fromhost: u64,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    virtio_count: c_int,
}

// RISCVCPUClass / RISCVCPUCommonState (riscv_cpu.h), used to emulate the
// static inline riscv_cpu_get_misa().
#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    riscv_cpu_end: Option<unsafe extern "C" fn(*mut c_void)>,
    riscv_cpu_interp: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut c_void) -> u64>,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut c_void, u32)>,
    riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

unsafe fn riscv_cpu_get_misa(s: *mut c_void) -> u32 {
    if s.is_null() {
        return 0;
    }
    let class_ptr = *(s as *const *const RISCVCPUClass);
    if class_ptr.is_null() {
        return 0;
    }
    match (*class_ptr).riscv_cpu_get_misa {
        Some(f) => f(s),
        None => 0,
    }
}

// ---------------------------------------------------------------------------
// FDT builder state (heap based equivalent of FDTState).
// ---------------------------------------------------------------------------

struct FdtState {
    // structure block, already in big-endian byte order
    tab: Vec<u8>,
    string_table: Vec<u8>,
    open_node_count: c_int,
}

impl FdtState {
    fn new() -> FdtState {
        FdtState {
            tab: Vec::new(),
            string_table: Vec::new(),
            open_node_count: 0,
        }
    }
}

fn fdt_put32(s: &mut FdtState, v: u32) {
    s.tab.extend_from_slice(&v.to_be_bytes());
}

fn fdt_put_data(s: &mut FdtState, data: &[u8]) {
    s.tab.extend_from_slice(data);
    let pad = (4 - (data.len() & 3)) & 3;
    for _ in 0..pad {
        s.tab.push(0);
    }
}

fn fdt_begin_node(s: &mut FdtState, name: &[u8]) {
    fdt_put32(s, 1);
    let mut buf: Vec<u8> = Vec::with_capacity(name.len() + 1);
    buf.extend_from_slice(name);
    buf.push(0);
    fdt_put_data(s, &buf);
    s.open_node_count = s.open_node_count.wrapping_add(1);
}

fn fdt_begin_node_num(s: &mut FdtState, name: &str, n: u64) {
    let buf = format!("{}@{:x}", name, n);
    fdt_begin_node(s, buf.as_bytes());
}

fn fdt_end_node(s: &mut FdtState) {
    fdt_put32(s, 2);
    s.open_node_count = s.open_node_count.wrapping_sub(1);
}

fn fdt_get_string_offset(s: &mut FdtState, name: &[u8]) -> u32 {
    let mut pos: usize = 0;
    while pos < s.string_table.len() {
        let rest = &s.string_table[pos..];
        let end = match rest.iter().position(|&c| c == 0) {
            Some(e) => e,
            None => rest.len(),
        };
        if let Some(prefix) = rest.get(..end) {
            if prefix == name {
                return pos as u32;
            }
        }
        pos += end + 1;
    }

    let pos = s.string_table.len();
    s.string_table.extend_from_slice(name);
    s.string_table.push(0);
    pos as u32
}

fn fdt_prop(s: &mut FdtState, prop_name: &str, data: &[u8]) {
    fdt_put32(s, 3);
    fdt_put32(s, data.len() as u32);
    let off = fdt_get_string_offset(s, prop_name.as_bytes());
    fdt_put32(s, off);
    fdt_put_data(s, data);
}

fn fdt_prop_tab_u32(s: &mut FdtState, prop_name: &str, tab: &[u32]) {
    fdt_put32(s, 3);
    fdt_put32(s, (tab.len() as u32).wrapping_mul(4));
    let off = fdt_get_string_offset(s, prop_name.as_bytes());
    fdt_put32(s, off);
    for &v in tab.iter() {
        fdt_put32(s, v);
    }
}

fn fdt_prop_u32(s: &mut FdtState, prop_name: &str, val: u32) {
    let tab = [val];
    fdt_prop_tab_u32(s, prop_name, &tab);
}

fn fdt_prop_tab_u64(s: &mut FdtState, prop_name: &str, v0: u64) {
    let tab = [(v0 >> 32) as u32, v0 as u32];
    fdt_prop_tab_u32(s, prop_name, &tab);
}

fn fdt_prop_tab_u64_2(s: &mut FdtState, prop_name: &str, v0: u64, v1: u64) {
    let tab = [
        (v0 >> 32) as u32,
        v0 as u32,
        (v1 >> 32) as u32,
        v1 as u32,
    ];
    fdt_prop_tab_u32(s, prop_name, &tab);
}

fn fdt_prop_str(s: &mut FdtState, prop_name: &str, val: &[u8]) {
    let mut buf: Vec<u8> = Vec::with_capacity(val.len() + 1);
    buf.extend_from_slice(val);
    buf.push(0);
    fdt_prop(s, prop_name, &buf);
}

fn fdt_prop_tab_str(s: &mut FdtState, prop_name: &str, vals: &[&[u8]]) {
    let mut tab: Vec<u8> = Vec::new();
    for v in vals.iter() {
        tab.extend_from_slice(v);
        tab.push(0);
    }
    fdt_prop(s, prop_name, &tab);
}

unsafe fn write_be32(dst: *mut u8, off: usize, v: u32) {
    let b = v.to_be_bytes();
    let mut i = 0usize;
    while i < 4 {
        *dst.add(off + i) = b[i];
        i += 1;
    }
}

// Mirrors fdt_output(): writes the FDT to 'dst' and returns its size in bytes.
unsafe fn fdt_output(s: &mut FdtState, dst: *mut u8) -> c_int {
    if s.open_node_count != 0 {
        std::process::abort();
    }

    fdt_put32(s, 9); // FDT_END

    let dt_struct_size = s.tab.len();
    let dt_strings_size = s.string_table.len();

    write_be32(dst, 0, FDT_MAGIC);
    write_be32(dst, 4, 0); // totalsize, set later
    write_be32(dst, 8, 40); // off_dt_struct = sizeof(struct fdt_header)
    write_be32(dst, 12, 0); // off_dt_strings, set later
    write_be32(dst, 16, 0); // off_mem_rsvmap, set later
    write_be32(dst, 20, FDT_VERSION);
    write_be32(dst, 24, 16); // last_comp_version
    write_be32(dst, 28, 0); // boot_cpuid_phys
    write_be32(dst, 32, dt_strings_size as u32);
    write_be32(dst, 36, dt_struct_size as u32);

    let mut pos: usize = 40; // sizeof(struct fdt_header)

    // off_dt_struct
    core::ptr::copy_nonoverlapping(s.tab.as_ptr(), dst.add(pos), dt_struct_size);
    pos += dt_struct_size;

    // align to 8
    while (pos & 7) != 0 {
        *dst.add(pos) = 0;
        pos += 1;
    }
    write_be32(dst, 16, pos as u32); // off_mem_rsvmap
    // one empty reserve entry (address = 0, size = 0)
    let mut i = 0usize;
    while i < 16 {
        *dst.add(pos + i) = 0;
        i += 1;
    }
    pos += 16; // sizeof(struct fdt_reserve_entry)

    write_be32(dst, 12, pos as u32); // off_dt_strings
    core::ptr::copy_nonoverlapping(s.string_table.as_ptr(), dst.add(pos), dt_strings_size);
    pos += dt_strings_size;

    // align to 8, just in case
    while (pos & 7) != 0 {
        *dst.add(pos) = 0;
        pos += 1;
    }

    write_be32(dst, 4, pos as u32); // totalsize

    pos as c_int
}

#[no_mangle]
pub unsafe extern "C" fn riscv_build_fdt(
    m: *mut RISCVMachine,
    dst: *mut u8,
    kernel_start: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_start: u64,
    initrd_size: u64,
) -> c_int {
    let mut s = FdtState::new();

    let mut cur_phandle: c_int = 1;
    let intc_phandle: c_int;
    let plic_phandle: c_int;

    fdt_begin_node(&mut s, b"");
    fdt_prop_u32(&mut s, "#address-cells", 2);
    fdt_prop_u32(&mut s, "#size-cells", 2);
    fdt_prop_str(&mut s, "compatible", b"ucbbar,riscvemu-bar_dev");
    fdt_prop_str(&mut s, "model", b"ucbbar,riscvemu-bare");

    // CPU list
    fdt_begin_node(&mut s, b"cpus");
    fdt_prop_u32(&mut s, "#address-cells", 1);
    fdt_prop_u32(&mut s, "#size-cells", 0);
    fdt_prop_u32(&mut s, "timebase-frequency", RTC_FREQ);

    // cpu
    fdt_begin_node_num(&mut s, "cpu", 0);
    fdt_prop_str(&mut s, "device_type", b"cpu");
    fdt_prop_u32(&mut s, "reg", 0);
    fdt_prop_str(&mut s, "status", b"okay");
    fdt_prop_str(&mut s, "compatible", b"riscv");

    let max_xlen: c_int = (*m).max_xlen;
    let misa: u32 = riscv_cpu_get_misa((*m).cpu_state);

    let mut isa_string: Vec<u8> = Vec::with_capacity(128);
    isa_string.extend_from_slice(format!("rv{}", max_xlen).as_bytes());
    let mut i: c_int = 0;
    while i < 26 {
        if (misa & (1u32 << (i as u32))) != 0 {
            isa_string.push(b'a'.wrapping_add(i as u8));
        }
        i += 1;
    }
    fdt_prop_str(&mut s, "riscv,isa", &isa_string);

    let mmu_type: &[u8] = if max_xlen <= 32 {
        b"riscv,sv32"
    } else {
        b"riscv,sv48"
    };
    fdt_prop_str(&mut s, "mmu-type", mmu_type);
    fdt_prop_u32(&mut s, "clock-frequency", 2_000_000_000);

    fdt_begin_node(&mut s, b"interrupt-controller");
    fdt_prop_u32(&mut s, "#interrupt-cells", 1);
    fdt_prop(&mut s, "interrupt-controller", &[]);
    fdt_prop_str(&mut s, "compatible", b"riscv,cpu-intc");
    intc_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    fdt_prop_u32(&mut s, "phandle", intc_phandle as u32);
    fdt_end_node(&mut s); // interrupt-controller

    fdt_end_node(&mut s); // cpu

    fdt_end_node(&mut s); // cpus

    fdt_begin_node_num(&mut s, "memory", RAM_BASE_ADDR as u64);
    fdt_prop_str(&mut s, "device_type", b"memory");
    let ram_size: u64 = (*m).ram_size;
    let tab = [
        (RAM_BASE_ADDR as u64 >> 32) as u32,
        RAM_BASE_ADDR,
        (ram_size >> 32) as u32,
        ram_size as u32,
    ];
    fdt_prop_tab_u32(&mut s, "reg", &tab);

    fdt_end_node(&mut s); // memory

    fdt_begin_node(&mut s, b"htif");
    fdt_prop_str(&mut s, "compatible", b"ucb,htif0");
    fdt_end_node(&mut s); // htif

    fdt_begin_node(&mut s, b"soc");
    fdt_prop_u32(&mut s, "#address-cells", 2);
    fdt_prop_u32(&mut s, "#size-cells", 2);
    fdt_prop_tab_str(
        &mut s,
        "compatible",
        &[b"ucbbar,riscvemu-bar-soc", b"simple-bus"],
    );
    fdt_prop(&mut s, "ranges", &[]);

    fdt_begin_node_num(&mut s, "clint", CLINT_BASE_ADDR);
    fdt_prop_str(&mut s, "compatible", b"riscv,clint0");

    let tab = [
        intc_phandle as u32,
        3, // M IPI irq
        intc_phandle as u32,
        7, // M timer irq
    ];
    fdt_prop_tab_u32(&mut s, "interrupts-extended", &tab);

    fdt_prop_tab_u64_2(&mut s, "reg", CLINT_BASE_ADDR, CLINT_SIZE);

    fdt_end_node(&mut s); // clint

    fdt_begin_node_num(&mut s, "plic", PLIC_BASE_ADDR);
    fdt_prop_u32(&mut s, "#interrupt-cells", 1);
    fdt_prop(&mut s, "interrupt-controller", &[]);
    fdt_prop_str(&mut s, "compatible", b"riscv,plic0");
    fdt_prop_u32(&mut s, "riscv,ndev", 31);
    fdt_prop_tab_u64_2(&mut s, "reg", PLIC_BASE_ADDR, PLIC_SIZE);

    let tab = [
        intc_phandle as u32,
        9, // S ext irq
        intc_phandle as u32,
        11, // M ext irq
    ];
    fdt_prop_tab_u32(&mut s, "interrupts-extended", &tab);

    plic_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    fdt_prop_u32(&mut s, "phandle", plic_phandle as u32);

    fdt_end_node(&mut s); // plic

    let virtio_count: c_int = (*m).virtio_count;
    let mut i: c_int = 0;
    while i < virtio_count {
        let addr = VIRTIO_BASE_ADDR.wrapping_add(i.wrapping_mul(VIRTIO_SIZE as i32));
        fdt_begin_node_num(&mut s, "virtio", addr as u64);
        fdt_prop_str(&mut s, "compatible", b"virtio,mmio");
        fdt_prop_tab_u64_2(&mut s, "reg", addr as u64, VIRTIO_SIZE);
        let tab = [
            plic_phandle as u32,
            VIRTIO_IRQ.wrapping_add(i) as u32,
        ];
        fdt_prop_tab_u32(&mut s, "interrupts-extended", &tab);
        fdt_end_node(&mut s); // virtio
        i += 1;
    }

    let fb_dev: *mut FBDevice = (*m).common.fb_dev;
    if !fb_dev.is_null() {
        fdt_begin_node_num(&mut s, "framebuffer", FRAMEBUFFER_BASE_ADDR);
        fdt_prop_str(&mut s, "compatible", b"simple-framebuffer");
        fdt_prop_tab_u64_2(
            &mut s,
            "reg",
            FRAMEBUFFER_BASE_ADDR,
            (*fb_dev).fb_size as i64 as u64,
        );
        fdt_prop_u32(&mut s, "width", (*fb_dev).width as u32);
        fdt_prop_u32(&mut s, "height", (*fb_dev).height as u32);
        fdt_prop_u32(&mut s, "stride", (*fb_dev).stride as u32);
        fdt_prop_str(&mut s, "format", b"a8r8g8b8");
        fdt_end_node(&mut s); // framebuffer
    }

    fdt_end_node(&mut s); // soc

    fdt_begin_node(&mut s, b"chosen");
    let cmd_line_bytes: &[u8] = if cmd_line.is_null() {
        b""
    } else {
        CStr::from_ptr(cmd_line).to_bytes()
    };
    fdt_prop_str(&mut s, "bootargs", cmd_line_bytes);
    if kernel_size > 0 {
        fdt_prop_tab_u64(&mut s, "riscv,kernel-start", kernel_start);
        fdt_prop_tab_u64(&mut s, "riscv,kernel-end", kernel_start.wrapping_add(kernel_size));
    }
    if initrd_size > 0 {
        fdt_prop_tab_u64(&mut s, "linux,initrd-start", initrd_start);
        fdt_prop_tab_u64(&mut s, "linux,initrd-end", initrd_start.wrapping_add(initrd_size));
    }

    fdt_end_node(&mut s); // chosen

    fdt_end_node(&mut s); // /

    let size = fdt_output(&mut s, dst);
    // fdt_end(s): the builder state is released automatically
    size
}
}

// ── `FDTState *s; int size, max_xlen, i, cur_phandle, intc_phandle, plic_phandle; char isa_string[12…` · riscv_machine.c:593-626 · chunk 2046 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FDTState *s;
//         int size, max_xlen, i, cur_phandle, intc_phandle, plic_phandle;
//         char isa_string[128], *q;
//         uint32_t misa;
//         uint32_t tab[4];
//         FBDevice *fb_dev;
//     
//         s = fdt_init();
//     
//         cur_phandle = 1;
//     
//         fdt_begin_node(s, "");
//         fdt_prop_u32(s, "#address-cells", 2);
//         fdt_prop_u32(s, "#size-cells", 2);
//         fdt_prop_str(s, "compatible", "ucbbar,riscvemu-bar_dev");
//         fdt_prop_str(s, "model", "ucbbar,riscvemu-bare");
//     
//     
//         fdt_begin_node(s, "cpus");
//         fdt_prop_u32(s, "#address-cells", 1);
//         fdt_prop_u32(s, "#size-cells", 0);
//         fdt_prop_u32(s, "timebase-frequency", 10000000);
//     
//     
//         fdt_begin_node_num(s, "cpu", 0);
//         fdt_prop_str(s, "device_type", "cpu");
//         fdt_prop_u32(s, "reg", 0);
//         fdt_prop_str(s, "status", "okay");
//         fdt_prop_str(s, "compatible", "riscv");
//     
//         max_xlen = m->max_xlen;
//         misa = riscv_cpu_get_misa(m->cpu_state);
//         q = isa_string;
//         q += snprintf(isa_string, sizeof(isa_string), "rv%d", max_xlen);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2046 {
// Rust port of riscv_build_fdt lines 593-626 (riscv_machine.c).
// Declares the FDT state, emits the root / cpus / cpu nodes and builds
// the "riscv,isa" string prefix.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

#[repr(C)]
struct VirtMachinePrefix {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[repr(C)]
struct RISCVMachine {
    common: VirtMachinePrefix,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut c_void,
}

#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: *const c_void,
    riscv_cpu_end: *const c_void,
    riscv_cpu_interp: *const c_void,
    riscv_cpu_get_cycles: *const c_void,
    riscv_cpu_set_mip: *const c_void,
    riscv_cpu_reset_mip: *const c_void,
    riscv_cpu_get_mip: *const c_void,
    riscv_cpu_get_power_down: *const c_void,
    riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
    riscv_cpu_flush_tlb_write_range_ram: *const c_void,
}

#[repr(C)]
struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static inline uint32_t riscv_cpu_get_misa(RISCVCPUState *s)
unsafe fn riscv_cpu_get_misa(s: *mut c_void) -> u32 {
    let c = (*(s as *const RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_get_misa {
        Some(f) => f(s),
        None => 0,
    }
}

// ---- ported static helpers from riscv_machine.c ----

unsafe fn fdt_init() -> *mut FDTState {
    mallocz(core::mem::size_of::<FDTState>()) as *mut FDTState
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<u32>()),
        ) as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    let idx = (*s).tab_len;
    fdt_alloc_len(s, idx.wrapping_add(1));
    *(*s).tab.wrapping_add(idx as usize) = (v as u32).swap_bytes();
    (*s).tab_len = idx.wrapping_add(1);
}

// the data is zero padded
unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    let idx = (*s).tab_len;
    fdt_alloc_len(s, idx.wrapping_add(len1));
    let dst = ((*s).tab as *mut u8).wrapping_add((idx as usize).wrapping_mul(4));
    if len > 0 {
        core::ptr::copy_nonoverlapping(data, dst, len as usize);
    }
    let pad = len.wrapping_neg() & 3;
    if pad > 0 {
        core::ptr::write_bytes(dst.wrapping_add(len as usize), 0, pad as usize);
    }
    (*s).tab_len = idx.wrapping_add(len1);
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, (strlen(name) + 1) as c_int);
    (*s).open_node_count = (*s).open_node_count.wrapping_add(1);
}

unsafe fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0 as c_char; 256];
    snprintf(
        buf.as_mut_ptr(),
        256usize,
        b"%s@%lx\0".as_ptr() as *const c_char,
        name,
        n,
    );
    fdt_begin_node(s, buf.as_mut_ptr());
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let p = (*s).string_table.wrapping_add(pos as usize);
        if strcmp(p, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(p) as c_int).wrapping_add(1));
    }
    // add a new string
    let name_size = (strlen(name) + 1) as c_int;
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table =
            realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    let pos = (*s).string_table_len;
    if name_size > 0 {
        core::ptr::copy_nonoverlapping(
            name as *const u8,
            (*s).string_table.wrapping_add(pos as usize) as *mut u8,
            name_size as usize,
        );
    }
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const u8,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data, data_len);
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(core::mem::size_of::<u32>() as c_int));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.wrapping_add(i as usize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_u32(s: *mut FDTState, prop_name: *const c_char, val: u32) {
    fdt_prop_tab_u32(s, prop_name, &val as *const u32 as *mut u32, 1);
}

unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, str_: *const c_char) {
    fdt_prop(s, prop_name, str_ as *const u8, (strlen(str_) + 1) as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_build_fdt__part2046(
    m: *mut RISCVMachine,
    s_out: *mut *mut FDTState,
    cur_phandle_out: *mut c_int,
    max_xlen_out: *mut c_int,
    misa_out: *mut u32,
    isa_string: *mut c_char,
    q_out: *mut *mut c_char,
) {
    let s = fdt_init();
    *s_out = s;

    *cur_phandle_out = 1;

    fdt_begin_node(s, b"\0".as_ptr() as *const c_char);
    fdt_prop_u32(s, b"#address-cells\0".as_ptr() as *const c_char, 2);
    fdt_prop_u32(s, b"#size-cells\0".as_ptr() as *const c_char, 2);
    fdt_prop_str(
        s,
        b"compatible\0".as_ptr() as *const c_char,
        b"ucbbar,riscvemu-bar_dev\0".as_ptr() as *const c_char,
    );
    fdt_prop_str(
        s,
        b"model\0".as_ptr() as *const c_char,
        b"ucbbar,riscvemu-bare\0".as_ptr() as *const c_char,
    );

    // CPU list
    fdt_begin_node(s, b"cpus\0".as_ptr() as *const c_char);
    fdt_prop_u32(s, b"#address-cells\0".as_ptr() as *const c_char, 1);
    fdt_prop_u32(s, b"#size-cells\0".as_ptr() as *const c_char, 0);
    fdt_prop_u32(
        s,
        b"timebase-frequency\0".as_ptr() as *const c_char,
        10000000,
    );

    // cpu
    fdt_begin_node_num(s, b"cpu\0".as_ptr() as *const c_char, 0);
    fdt_prop_str(
        s,
        b"device_type\0".as_ptr() as *const c_char,
        b"cpu\0".as_ptr() as *const c_char,
    );
    fdt_prop_u32(s, b"reg\0".as_ptr() as *const c_char, 0);
    fdt_prop_str(
        s,
        b"status\0".as_ptr() as *const c_char,
        b"okay\0".as_ptr() as *const c_char,
    );
    fdt_prop_str(
        s,
        b"compatible\0".as_ptr() as *const c_char,
        b"riscv\0".as_ptr() as *const c_char,
    );

    let max_xlen = (*m).max_xlen;
    *max_xlen_out = max_xlen;
    let misa = riscv_cpu_get_misa((*m).cpu_state);
    *misa_out = misa;

    let ret = snprintf(
        isa_string,
        128usize,
        b"rv%d\0".as_ptr() as *const c_char,
        max_xlen,
    );
    let q = (isa_string as usize).wrapping_add(ret as isize as usize) as *mut c_char;
    *q_out = q;
}
}

// ── `for (i = 0; i < 26; i++)` · riscv_machine.c:627-630 · chunk 2047 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 26; i++) {
//             if (misa & (1 << i))
//                 *q++ = 'a' + i;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2047 {
// Port of the ISA-letter loop from riscv_build_fdt: appends one letter per
// set bit of misa (bits 0..25) to the string buffer, advancing the caller's
// cursor `q`.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2047(misa: u32, q: *mut *mut c_char) {
    unsafe {
        // Load the caller's cursor; it is written back before returning.
        let mut p: *mut c_char = *q;
        let mut i: c_int = 0;
        while i < 26 {
            if misa & (1u32 << (i as u32)) != 0 {
                *p = (b'a' as c_int + i) as c_char;
                p = p.add(1);
            }
            i += 1;
        }
        *q = p;
    }
}
}

// ── `*q = '\0'; fdt_prop_str(s, "riscv,isa", isa_string); fdt_prop_str(s, "mmu-type", max_xlen <= 32…` · riscv_machine.c:631-699 · chunk 2048 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q = '\0';
//         fdt_prop_str(s, "riscv,isa", isa_string);
//     
//         fdt_prop_str(s, "mmu-type", max_xlen <= 32 ? "riscv,sv32" : "riscv,sv48");
//         fdt_prop_u32(s, "clock-frequency", 2000000000);
//     
//         fdt_begin_node(s, "interrupt-controller");
//         fdt_prop_u32(s, "#interrupt-cells", 1);
//         fdt_prop(s, "interrupt-controller", 
//                                            ((void *)0)
//                                                , 0);
//         fdt_prop_str(s, "compatible", "riscv,cpu-intc");
//         intc_phandle = cur_phandle++;
//         fdt_prop_u32(s, "phandle", intc_phandle);
//         fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node_num(s, "memory", 0x80000000);
//         fdt_prop_str(s, "device_type", "memory");
//         tab[0] = (uint64_t)0x80000000 >> 32;
//         tab[1] = 0x80000000;
//         tab[2] = m->ram_size >> 32;
//         tab[3] = m->ram_size;
//         fdt_prop_tab_u32(s, "reg", tab, 4);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node(s, "htif");
//         fdt_prop_str(s, "compatible", "ucb,htif0");
//         fdt_end_node(s);
//     
//         fdt_begin_node(s, "soc");
//         fdt_prop_u32(s, "#address-cells", 2);
//         fdt_prop_u32(s, "#size-cells", 2);
//         fdt_prop_tab_str(s, "compatible",
//                          "ucbbar,riscvemu-bar-soc", "simple-bus", 
//                                                                  ((void *)0)
//                                                                      );
//         fdt_prop(s, "ranges", 
//                              ((void *)0)
//                                  , 0);
//     
//         fdt_begin_node_num(s, "clint", 0x02000000);
//         fdt_prop_str(s, "compatible", "riscv,clint0");
//     
//         tab[0] = intc_phandle;
//         tab[1] = 3;
//         tab[2] = intc_phandle;
//         tab[3] = 7;
//         fdt_prop_tab_u32(s, "interrupts-extended", tab, 4);
//     
//         fdt_prop_tab_u64_2(s, "reg", 0x02000000, 0x000c0000);
//     
//         fdt_end_node(s);
//     
//         fdt_begin_node_num(s, "plic", 0x40100000);
//         fdt_prop_u32(s, "#interrupt-cells", 1);
//         fdt_prop(s, "interrupt-controller", 
//                                            ((void *)0)
//                                                , 0);
//         fdt_prop_str(s, "compatible", "riscv,plic0");
//         fdt_prop_u32(s, "riscv,ndev", 31);
//         fdt_prop_tab_u64_2(s, "reg", 0x40100000, 0x00400000);
//     
//         tab[0] = intc_phandle;
//         tab[1] = 9;
//         tab[2] = intc_phandle;
//         tab[3] = 11;
//         fdt_prop_tab_u32(s, "interrupts-extended", tab, 4);
//     
//         plic_phandle = cur_phandle++;
//         fdt_prop_u32(s, "phandle", plic_phandle);
//     
//         fdt_end_node(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2048 {
// Port of a slice of riscv_build_fdt (riscv_machine.c): emission of the
// riscv,isa / mmu-type properties plus the interrupt-controller, memory,
// htif, soc, clint and plic nodes.
//
// The fdt_* helpers are `static` in the C translation unit, so their logic
// is mirrored here as private functions operating on the very same FDTState
// instance that was created by fdt_init() in C and that fdt_output()/fdt_end()
// will consume later.

#![allow(dead_code, non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// Same layout as the C `FDTState` (allocation is done by the C fdt_init()).
#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

// Prefix of `VirtMachine` (fields are only touched opaquely from here).
#[repr(C)]
pub struct VirtMachinePrefix {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Prefix of the C `RISCVMachine` structure; only max_xlen and ram_size are
// read, but every preceding field must be mirrored to get the offsets right.
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachinePrefix,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// static void fdt_alloc_len(FDTState *s, int len)
unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc((*s).tab as *mut c_void, (new_size as usize) * 4) as *mut u32;
        (*s).tab_size = new_size;
    }
}

// static void fdt_put32(FDTState *s, int v)  (cpu_to_be32 == byte swap)
unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = (v as u32).swap_bytes();
    (*s).tab_len = idx.wrapping_add(1);
}

// static void fdt_put_data(FDTState *s, const uint8_t *data, int len)
unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
    let dst = (*s).tab.add((*s).tab_len as usize) as *mut u8;
    if len > 0 {
        core::ptr::copy_nonoverlapping(data, dst, len as usize);
    }
    // memset((uint8_t *)ptr + len, 0, -len & 3)
    let pad = len.wrapping_neg() & 3;
    if pad > 0 {
        core::ptr::write_bytes(dst.add(len as usize), 0, pad as usize);
    }
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

// static void fdt_begin_node(FDTState *s, const char *name)
unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, strlen(name) as c_int + 1);
    (*s).open_node_count = (*s).open_node_count.wrapping_add(1);
}

// static void fdt_begin_node_num(FDTState *s, const char *name, uint64_t n)
unsafe fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0 as c_char; 256];
    snprintf(
        buf.as_mut_ptr(),
        256,
        b"%s@%lx\0".as_ptr() as *const c_char,
        name,
        n,
    );
    fdt_begin_node(s, buf.as_ptr());
}

// static void fdt_end_node(FDTState *s)
unsafe fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count = (*s).open_node_count.wrapping_sub(1);
}

// static int fdt_get_string_offset(FDTState *s, const char *name)
unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let cur = (*s).string_table.add(pos as usize);
        if strcmp(cur, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add(strlen(cur) as c_int + 1);
    }

    let name_size = strlen(name) as c_int + 1;
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table =
            realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    core::ptr::copy_nonoverlapping(
        name,
        (*s).string_table.add(pos as usize),
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

// static void fdt_prop(FDTState *s, const char *prop_name,
//                      const void *data, int data_len)
unsafe fn fdt_prop(s: *mut FDTState, prop_name: *const c_char, data: *const u8, data_len: c_int) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data, data_len);
}

// static void fdt_prop_tab_u32(FDTState *s, const char *prop_name,
//                              uint32_t *tab, int tab_len)
unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *const u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i += 1;
    }
}

// static void fdt_prop_u32(FDTState *s, const char *prop_name, uint32_t val)
unsafe fn fdt_prop_u32(s: *mut FDTState, prop_name: *const c_char, val: u32) {
    fdt_prop_tab_u32(s, prop_name, &val, 1);
}

// static void fdt_prop_tab_u64_2(FDTState *s, const char *prop_name,
//                                uint64_t v0, uint64_t v1)
unsafe fn fdt_prop_tab_u64_2(s: *mut FDTState, prop_name: *const c_char, v0: u64, v1: u64) {
    let mut tab = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_ptr(), 4);
}

// static void fdt_prop_str(FDTState *s, const char *prop_name, const char *str)
unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, str_: *const c_char) {
    fdt_prop(s, prop_name, str_ as *const u8, strlen(str_) as c_int + 1);
}

// static void fdt_prop_tab_str(FDTState *s, const char *prop_name, ...)
// specialised to the two NULL-terminated strings used by this call site.
unsafe fn fdt_prop_tab_str2(
    s: *mut FDTState,
    prop_name: *const c_char,
    a: *const c_char,
    b: *const c_char,
) {
    let sa = strlen(a) + 1;
    let sb = strlen(b) + 1;
    let size = sa + sb;
    let mut tab: std::vec::Vec<u8> = std::vec::Vec::with_capacity(size);
    tab.resize(size, 0);
    core::ptr::copy_nonoverlapping(a as *const u8, tab.as_mut_ptr(), sa);
    core::ptr::copy_nonoverlapping(b as *const u8, tab.as_mut_ptr().add(sa), sb);
    fdt_prop(s, prop_name, tab.as_ptr(), size as c_int);
}

#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2048(
    m: *mut RISCVMachine,
    s: *mut FDTState,
    max_xlen: c_int,
    q: *mut c_char,
    isa_string: *mut c_char,
    cur_phandle: *mut c_int,
    intc_phandle: *mut c_int,
    plic_phandle: *mut c_int,
    tab: *mut u32,
) {
    unsafe {
        *q = 0;

        fdt_prop_str(s, b"riscv,isa\0".as_ptr() as *const c_char, isa_string);

        let mmu = if max_xlen <= 32 {
            b"riscv,sv32\0".as_ptr() as *const c_char
        } else {
            b"riscv,sv48\0".as_ptr() as *const c_char
        };
        fdt_prop_str(s, b"mmu-type\0".as_ptr() as *const c_char, mmu);
        fdt_prop_u32(s, b"clock-frequency\0".as_ptr() as *const c_char, 2000000000);

        fdt_begin_node(s, b"interrupt-controller\0".as_ptr() as *const c_char);
        fdt_prop_u32(s, b"#interrupt-cells\0".as_ptr() as *const c_char, 1);
        fdt_prop(
            s,
            b"interrupt-controller\0".as_ptr() as *const c_char,
            core::ptr::null(),
            0,
        );
        fdt_prop_str(
            s,
            b"compatible\0".as_ptr() as *const c_char,
            b"riscv,cpu-intc\0".as_ptr() as *const c_char,
        );
        // intc_phandle = cur_phandle++;
        let ph = *cur_phandle;
        *cur_phandle = ph.wrapping_add(1);
        *intc_phandle = ph;
        fdt_prop_u32(s, b"phandle\0".as_ptr() as *const c_char, ph as u32);
        fdt_end_node(s);

        fdt_end_node(s);

        fdt_end_node(s);

        fdt_begin_node_num(s, b"memory\0".as_ptr() as *const c_char, 0x80000000);
        fdt_prop_str(
            s,
            b"device_type\0".as_ptr() as *const c_char,
            b"memory\0".as_ptr() as *const c_char,
        );
        *tab = ((0x80000000u64) >> 32) as u32;
        *tab.add(1) = 0x80000000u32;
        *tab.add(2) = ((*m).ram_size >> 32) as u32;
        *tab.add(3) = (*m).ram_size as u32;
        fdt_prop_tab_u32(s, b"reg\0".as_ptr() as *const c_char, tab, 4);

        fdt_end_node(s);

        fdt_begin_node(s, b"htif\0".as_ptr() as *const c_char);
        fdt_prop_str(
            s,
            b"compatible\0".as_ptr() as *const c_char,
            b"ucb,htif0\0".as_ptr() as *const c_char,
        );
        fdt_end_node(s);

        fdt_begin_node(s, b"soc\0".as_ptr() as *const c_char);
        fdt_prop_u32(s, b"#address-cells\0".as_ptr() as *const c_char, 2);
        fdt_prop_u32(s, b"#size-cells\0".as_ptr() as *const c_char, 2);
        fdt_prop_tab_str2(
            s,
            b"compatible\0".as_ptr() as *const c_char,
            b"ucbbar,riscvemu-bar-soc\0".as_ptr() as *const c_char,
            b"simple-bus\0".as_ptr() as *const c_char,
        );
        fdt_prop(s, b"ranges\0".as_ptr() as *const c_char, core::ptr::null(), 0);

        fdt_begin_node_num(s, b"clint\0".as_ptr() as *const c_char, 0x02000000);
        fdt_prop_str(
            s,
            b"compatible\0".as_ptr() as *const c_char,
            b"riscv,clint0\0".as_ptr() as *const c_char,
        );

        *tab = *intc_phandle as u32;
        *tab.add(1) = 3;
        *tab.add(2) = *intc_phandle as u32;
        *tab.add(3) = 7;
        fdt_prop_tab_u32(
            s,
            b"interrupts-extended\0".as_ptr() as *const c_char,
            tab,
            4,
        );

        fdt_prop_tab_u64_2(
            s,
            b"reg\0".as_ptr() as *const c_char,
            0x02000000,
            0x000c0000,
        );

        fdt_end_node(s);

        fdt_begin_node_num(s, b"plic\0".as_ptr() as *const c_char, 0x40100000);
        fdt_prop_u32(s, b"#interrupt-cells\0".as_ptr() as *const c_char, 1);
        fdt_prop(
            s,
            b"interrupt-controller\0".as_ptr() as *const c_char,
            core::ptr::null(),
            0,
        );
        fdt_prop_str(
            s,
            b"compatible\0".as_ptr() as *const c_char,
            b"riscv,plic0\0".as_ptr() as *const c_char,
        );
        fdt_prop_u32(s, b"riscv,ndev\0".as_ptr() as *const c_char, 31);
        fdt_prop_tab_u64_2(
            s,
            b"reg\0".as_ptr() as *const c_char,
            0x40100000,
            0x00400000,
        );

        *tab = *intc_phandle as u32;
        *tab.add(1) = 9;
        *tab.add(2) = *intc_phandle as u32;
        *tab.add(3) = 11;
        fdt_prop_tab_u32(
            s,
            b"interrupts-extended\0".as_ptr() as *const c_char,
            tab,
            4,
        );

        // plic_phandle = cur_phandle++;
        let ph2 = *cur_phandle;
        *cur_phandle = ph2.wrapping_add(1);
        *plic_phandle = ph2;
        fdt_prop_u32(s, b"phandle\0".as_ptr() as *const c_char, ph2 as u32);

        fdt_end_node(s);
    }
}
}

// ── `for (i = 0; i < m->virtio_count; i++)` · riscv_machine.c:701-710 · chunk 2049 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < m->virtio_count; i++) {
//             fdt_begin_node_num(s, "virtio", 0x40010000 + i * 0x1000);
//             fdt_prop_str(s, "compatible", "virtio,mmio");
//             fdt_prop_tab_u64_2(s, "reg", 0x40010000 + i * 0x1000,
//                                0x1000);
//             tab[0] = plic_phandle;
//             tab[1] = 1 + i;
//             fdt_prop_tab_u32(s, "interrupts-extended", tab, 2);
//             fdt_end_node(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2049 {
// Port of the virtio-node emission loop from riscv_build_fdt in riscv_machine.c.
// The FDT helpers it uses are static in the C translation unit, so their logic
// is reproduced here as private Rust functions.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

const HEX: [u8; 16] = *b"0123456789abcdef";

unsafe fn c_strlen(p: *const c_char) -> usize {
    let mut n = 0usize;
    while *p.add(n) != 0 {
        n += 1;
    }
    n
}

unsafe fn c_strcmp(a: *const c_char, b: *const c_char) -> c_int {
    let mut i = 0isize;
    loop {
        let ca = *a.offset(i) as u8;
        let cb = *b.offset(i) as u8;
        if ca != cb {
            return ca as c_int - cb as c_int;
        }
        if ca == 0 {
            return 0;
        }
        i += 1;
    }
}

unsafe fn c_memcpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        *dst.add(i) = *src.add(i);
        i += 1;
    }
}

unsafe fn c_memset(dst: *mut u8, v: u8, n: usize) {
    let mut i = 0usize;
    while i < n {
        *dst.add(i) = v;
        i += 1;
    }
}

unsafe fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc((*s).tab as *mut c_void, new_size as usize * 4) as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    let idx = (*s).tab_len;
    fdt_alloc_len(s, idx + 1);
    *(*s).tab.add(idx as usize) = (v as u32).swap_bytes();
    (*s).tab_len = idx + 1;
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    let base = (*s).tab_len;
    fdt_alloc_len(s, base + len1);
    let dst = (*s).tab.add(base as usize) as *mut u8;
    c_memcpy(dst, data, len as usize);
    c_memset(dst.add(len as usize), 0, ((-len) & 3) as usize);
    (*s).tab_len = base + len1;
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    let l = c_strlen(name);
    fdt_put_data(s, name as *const u8, l as c_int + 1);
    (*s).open_node_count += 1;
}

unsafe fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    // snprintf(buf, sizeof(buf), "%s@%lx", name, n) into a 256 byte buffer
    let mut buf = [0u8; 256];
    let mut pos = 0usize;
    let mut idx = 0isize;
    loop {
        let c = *name.offset(idx) as u8;
        if c == 0 {
            break;
        }
        if pos < 255 {
            buf[pos] = c;
            pos += 1;
        }
        idx += 1;
    }
    if pos < 255 {
        buf[pos] = b'@';
        pos += 1;
    }
    if n == 0 {
        if pos < 255 {
            buf[pos] = b'0';
            pos += 1;
        }
    } else {
        let mut digits = [0u8; 16];
        let mut nd = 0usize;
        let mut v = n;
        while v != 0 {
            digits[nd] = HEX[(v & 0xf) as usize];
            nd += 1;
            v >>= 4;
        }
        let mut k = nd;
        while k > 0 {
            k -= 1;
            if pos < 255 {
                buf[pos] = digits[k];
                pos += 1;
            }
        }
    }
    buf[pos] = 0;
    fdt_begin_node(s, buf.as_ptr() as *const c_char);
}

unsafe fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count -= 1;
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if c_strcmp((*s).string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        pos += c_strlen((*s).string_table.offset(pos as isize)) as c_int + 1;
    }

    let name_size = c_strlen(name) as c_int + 1;
    let new_len = (*s).string_table_len + name_size;
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table =
            realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    c_memcpy(
        (*s).string_table.offset(pos as isize) as *mut u8,
        name as *const u8,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const u8,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
    fdt_put_data(s, data, data_len);
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *const u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len * 4);
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_tab_u64_2(
    s: *mut FDTState,
    prop_name: *const c_char,
    v0: u64,
    v1: u64,
) {
    let mut tab = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 4);
}

unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, str_: *const c_char) {
    let l = c_strlen(str_) as c_int + 1;
    fdt_prop(s, prop_name, str_ as *const u8, l);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_build_fdt__part2049(
    s: *mut FDTState,
    virtio_count: c_int,
    plic_phandle: c_int,
    tab: *mut u32,
) {
    let mut i: c_int = 0;
    while i < virtio_count {
        let addr_i32: c_int = 0x40010000i32.wrapping_add(i.wrapping_mul(0x1000));
        let addr: u64 = addr_i32 as i64 as u64;
        fdt_begin_node_num(s, b"virtio\0".as_ptr() as *const c_char, addr);
        fdt_prop_str(s, b"compatible\0".as_ptr() as *const c_char,
                     b"virtio,mmio\0".as_ptr() as *const c_char);
        fdt_prop_tab_u64_2(s, b"reg\0".as_ptr() as *const c_char, addr, 0x1000);
        *tab.add(0) = plic_phandle as u32;
        *tab.add(1) = 1i32.wrapping_add(i) as u32;
        fdt_prop_tab_u32(s, b"interrupts-extended\0".as_ptr() as *const c_char, tab, 2);
        fdt_end_node(s);
        i += 1;
    }
}
}

// ── `fb_dev = m->common.fb_dev;` · riscv_machine.c:712-712 · chunk 2050 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fb_dev = m->common.fb_dev;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2050 {
// Port of the C statement: fb_dev = m->common.fb_dev;
// Returns the FBDevice pointer stored in RISCVMachine.common.fb_dev.

use core::ffi::c_void;

// Opaque mirror of the C `FBDevice` (only ever used behind a pointer here).
#[repr(C)]
pub struct FBDevice {
    _opaque: [u8; 0],
}

// Mirror of C `VirtMachine` up to (and including) the `fb_dev` field.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut FBDevice,
}

// Mirror of C `RISCVMachine`; `common` is its first member.
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
}

#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2050(m: *mut RISCVMachine) -> *mut FBDevice {
    unsafe { (*m).common.fb_dev }
}
}

// ── `if (fb_dev)` · riscv_machine.c:713-722 · chunk 2051 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (fb_dev) {
//             fdt_begin_node_num(s, "framebuffer", 0x41000000);
//             fdt_prop_str(s, "compatible", "simple-framebuffer");
//             fdt_prop_tab_u64_2(s, "reg", 0x41000000, fb_dev->fb_size);
//             fdt_prop_u32(s, "width", fb_dev->width);
//             fdt_prop_u32(s, "height", fb_dev->height);
//             fdt_prop_u32(s, "stride", fb_dev->stride);
//             fdt_prop_str(s, "format", "a8r8g8b8");
//             fdt_end_node(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2051 {
// Port of the "framebuffer" node emission block of riscv_build_fdt.
// The fdt_* helpers used by the TARGET are `static` C functions without
// linker symbols, so their logic is ported here as private Rust fns.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
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
    redraw_func: *mut SimpleFBDrawFunc,
    opaque: *mut c_void,
);

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: *mut FBRefreshFunc,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

fn cs(b: &'static [u8]) -> *const c_char {
    b.as_ptr() as *const c_char
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc((*s).tab as *mut c_void, (new_size as usize).wrapping_mul(4));
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len + 1);
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = (v as u32).swap_bytes();
    (*s).tab_len = idx + 1;
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = (len + 3) / 4;
    fdt_alloc_len(s, (*s).tab_len + len1);
    let base = ((*s).tab as *mut u8).add((*s).tab_len as usize * 4);
    if len > 0 {
        memcpy(base as *mut c_void, data as *const c_void, len as usize);
    }
    let pad = (len.wrapping_neg() & 3) as usize;
    if pad > 0 {
        memset(base.add(len as usize) as *mut c_void, 0, pad);
    }
    (*s).tab_len += len1;
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, strlen(name) as c_int + 1);
    (*s).open_node_count += 1;
}

unsafe fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0 as c_char; 256];
    snprintf(
        buf.as_mut_ptr(),
        256,
        cs(b"%s@%lx\0"),
        name,
        n,
    );
    fdt_begin_node(s, buf.as_ptr());
}

unsafe fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count -= 1;
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let p = (*s).string_table.add(pos as usize);
        if strcmp(p, name) == 0 {
            return pos;
        }
        pos += strlen(p) as c_int + 1;
    }

    let name_size = strlen(name) as c_int + 1;
    let new_len = (*s).string_table_len + name_size;
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.add(pos as usize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const c_void,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data as *const u8, data_len);
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_u32(s: *mut FDTState, prop_name: *const c_char, val: u32) {
    let mut v: u32 = val;
    fdt_prop_tab_u32(s, prop_name, &mut v as *mut u32, 1);
}

unsafe fn fdt_prop_tab_u64_2(s: *mut FDTState, prop_name: *const c_char, v0: u64, v1: u64) {
    let mut tab = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 4);
}

unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, val: *const c_char) {
    fdt_prop(s, prop_name, val as *const c_void, strlen(val) as c_int + 1);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_build_fdt__part2051(s: *mut FDTState, fb_dev: *mut FBDevice) {
    if fb_dev.is_null() {
        return;
    }
    fdt_begin_node_num(s, cs(b"framebuffer\0"), 0x41000000);
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"simple-framebuffer\0"));
    fdt_prop_tab_u64_2(s, cs(b"reg\0"), 0x41000000, (*fb_dev).fb_size as u64);
    fdt_prop_u32(s, cs(b"width\0"), (*fb_dev).width as u32);
    fdt_prop_u32(s, cs(b"height\0"), (*fb_dev).height as u32);
    fdt_prop_u32(s, cs(b"stride\0"), (*fb_dev).stride as u32);
    fdt_prop_str(s, cs(b"format\0"), cs(b"a8r8g8b8\0"));
    fdt_end_node(s);
}
}

// ── `fdt_end_node(s); fdt_begin_node(s, "chosen"); fdt_prop_str(s, "bootargs", cmd_line ? cmd_line :…` · riscv_machine.c:724-727 · chunk 2052 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_end_node(s);
//     
//         fdt_begin_node(s, "chosen");
//         fdt_prop_str(s, "bootargs", cmd_line ? cmd_line : "");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2052 {
// Rust port of the FDT node helpers (ported from the static C helpers declared above
// riscv_build_fdt) plus the ported piece: end the "soc" node, open the "chosen" node
// and emit its "bootargs" property.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

const NAME_CHOSEN: &[u8] = b"chosen\0";
const NAME_BOOTARGS: &[u8] = b"bootargs\0";
const EMPTY_STR: &[u8] = b"\0";

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(4),
        );
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len + 1);
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx + 1;
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = (len + 3) / 4;
    fdt_alloc_len(s, (*s).tab_len + len1);
    let base = (*s).tab.add((*s).tab_len as usize) as *mut u8;
    memcpy(base as *mut c_void, data as *const c_void, len as usize);
    memset(
        base.add(len as usize) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as usize,
    );
    (*s).tab_len += len1;
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(
        s,
        name as *const u8,
        (strlen(name) as c_int).wrapping_add(1),
    );
    (*s).open_node_count += 1;
}

unsafe fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count -= 1;
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let cur = (*s).string_table.add(pos as usize);
        if strcmp(cur, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(cur) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.add(pos as usize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(s: *mut FDTState, prop_name: *const c_char, data: *const c_void, data_len: c_int) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data as *const u8, data_len);
}

unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, str_: *const c_char) {
    fdt_prop(
        s,
        prop_name,
        str_ as *const c_void,
        (strlen(str_) as c_int).wrapping_add(1),
    );
}

// The TARGET: close the enclosing "soc" node, open the "chosen" node and write bootargs.
#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2052(s: *mut FDTState, cmd_line: *const c_char) {
    unsafe {
        fdt_end_node(s);
        fdt_begin_node(s, NAME_CHOSEN.as_ptr() as *const c_char);
        let bootargs: *const c_char = if cmd_line.is_null() {
            EMPTY_STR.as_ptr() as *const c_char
        } else {
            cmd_line
        };
        fdt_prop_str(s, NAME_BOOTARGS.as_ptr() as *const c_char, bootargs);
    }
}
}

// ── `if (kernel_size > 0)` · riscv_machine.c:728-731 · chunk 2053 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (kernel_size > 0) {
//             fdt_prop_tab_u64(s, "riscv,kernel-start", kernel_start);
//             fdt_prop_tab_u64(s, "riscv,kernel-end", kernel_start + kernel_size);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2053 {
// Port of the `if (kernel_size > 0) { ... }` chunk of riscv_build_fdt().
// The helpers it needs (fdt_prop_tab_u64 / fdt_prop_tab_u32 / fdt_put32 /
// fdt_alloc_len / fdt_put_data / fdt_get_string_offset) are static in the C
// translation unit, so their logic is mirrored here as private functions.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(::core::mem::size_of::<u32>()),
        ) as *mut u32;
        (*s).tab = p;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    *(*s).tab.add(idx as usize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1 = len.wrapping_add(3) / 4;
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(len1));
    let dst = (*s).tab.add((*s).tab_len as usize);
    memcpy(dst as *mut c_void, data as *const c_void, len as usize);
    let tail = (dst as *mut u8).add(len as usize);
    memset(tail as *mut c_void, 0, ((-len) & 3) as usize);
    (*s).tab_len = (*s).tab_len.wrapping_add(len1);
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        let p = (*s).string_table.add(pos as usize);
        if strcmp(p, name) == 0 {
            return pos;
        }
        pos = pos.wrapping_add((strlen(p) as c_int).wrapping_add(1));
    }

    let name_size = (strlen(name) as c_int).wrapping_add(1);
    let new_len = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(
            new_len,
            (*s).string_table_size.wrapping_mul(3) / 2,
        );
        let p = realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table = p;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.add(pos as usize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(::core::mem::size_of::<u32>() as c_int));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i = i.wrapping_add(1);
    }
}

unsafe fn fdt_prop_tab_u64(s: *mut FDTState, prop_name: *const c_char, v0: u64) {
    let mut tab: [u32; 2] = [0; 2];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 2);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_build_fdt__part2053(
    s: *mut FDTState,
    kernel_start: u64,
    kernel_size: u64,
) {
    fdt_prop_tab_u64(
        s,
        b"riscv,kernel-start\0".as_ptr() as *const c_char,
        kernel_start,
    );
    fdt_prop_tab_u64(
        s,
        b"riscv,kernel-end\0".as_ptr() as *const c_char,
        kernel_start.wrapping_add(kernel_size),
    );
}
}

// ── `if (initrd_size > 0)` · riscv_machine.c:732-735 · chunk 2054 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (initrd_size > 0) {
//             fdt_prop_tab_u64(s, "linux,initrd-start", initrd_start);
//             fdt_prop_tab_u64(s, "linux,initrd-end", initrd_start + initrd_size);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2054 {
// Port of the initrd part of riscv_build_fdt (riscv_machine.c): the
// "linux,initrd-start"/"linux,initrd-end" property emission, including the
// static FDT helpers it depends on (fdt_prop_tab_u64 -> fdt_prop_tab_u32 ->
// fdt_put32/fdt_get_string_offset), ported privately since they have no
// linker symbols.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn strlen_c(p: *const c_char) -> usize {
    let mut n: usize = 0;
    while *p.add(n) != 0 {
        n += 1;
    }
    n
}

unsafe fn strcmp_c(a: *const c_char, b: *const c_char) -> c_int {
    let mut i: usize = 0;
    loop {
        let ca = *a.add(i) as u8;
        let cb = *b.add(i) as u8;
        if ca != cb {
            return ca as c_int - cb as c_int;
        }
        if ca == 0 {
            return 0;
        }
        i += 1;
    }
}

unsafe fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let p = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<u32>()),
        );
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FdtState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len.wrapping_add(1));
    let idx = (*s).tab_len;
    let slot = (*s).tab.add(idx as usize);
    *slot = (v as u32).swap_bytes();
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_get_string_offset(s: *mut FdtState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if strcmp_c((*s).string_table.offset(pos as isize), name) == 0 {
            return pos;
        }
        pos += strlen_c((*s).string_table.offset(pos as isize)) as c_int + 1;
    }

    let name_size: c_int = strlen_c(name) as c_int + 1;
    let new_len: c_int = (*s).string_table_len.wrapping_add(name_size);
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        let p = realloc((*s).string_table as *mut c_void, new_size as usize);
        (*s).string_table = p as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    memcpy(
        (*s).string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as usize,
    );
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FdtState,
    prop_name: *const c_char,
    tab: *mut u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(core::mem::size_of::<u32>() as c_int));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_tab_u64(s: *mut FdtState, prop_name: *const c_char, v0: u64) {
    let mut tab: [u32; 2] = [0; 2];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 2);
}

#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2054(
    s: *mut FdtState,
    initrd_start: u64,
    initrd_size: u64,
) {
    unsafe {
        fdt_prop_tab_u64(
            s,
            b"linux,initrd-start\0".as_ptr() as *const c_char,
            initrd_start,
        );
        fdt_prop_tab_u64(
            s,
            b"linux,initrd-end\0".as_ptr() as *const c_char,
            initrd_start.wrapping_add(initrd_size),
        );
    }
}
}

// ── `fdt_end_node(s); fdt_end_node(s); size = fdt_output(s, dst); fdt_end(s); return size;` · riscv_machine.c:737-751 · chunk 2055 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fdt_end_node(s);
//     
//         fdt_end_node(s);
//     
//         size = fdt_output(s, dst);
//         fdt_end(s);
//         return size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2055 {
// Port of the tail of riscv_build_fdt: closing the last two FDT nodes,
// emitting the blob and freeing the FDT state.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct FdtState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn fdt_output(s: *mut FdtState, dst: *mut u8) -> c_int;
    fn fdt_end(s: *mut FdtState);
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn fdt_alloc_len(s: *mut FdtState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        let bytes = (new_size as usize).wrapping_mul(core::mem::size_of::<u32>());
        let p = realloc((*s).tab as *mut c_void, bytes);
        (*s).tab = p as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FdtState, v: c_int) {
    let need = (*s).tab_len.wrapping_add(1);
    fdt_alloc_len(s, need);
    let idx = (*s).tab_len;
    *(*s).tab.offset(idx as isize) = cpu_to_be32(v as u32);
    (*s).tab_len = idx.wrapping_add(1);
}

unsafe fn fdt_end_node(s: *mut FdtState) {
    fdt_put32(s, 2);
    (*s).open_node_count = (*s).open_node_count.wrapping_sub(1);
}

#[no_mangle]
pub extern "C" fn riscv_build_fdt__part2055(s: *mut FdtState, dst: *mut u8) -> c_int {
    unsafe {
        fdt_end_node(s);
        fdt_end_node(s);
        let size = fdt_output(s, dst);
        fdt_end(s);
        size
    }
}
}

// ── `copy_bios` · riscv_machine.c:754-866 · chunk 2056 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void copy_bios(RISCVMachine *s, const uint8_t *buf, int buf_len,
//                           const uint8_t *kernel_buf, int kernel_buf_len,
//                           const char *cmd_line,
//                           const uint8_t *initrd_buf, int initrd_buf_len)
//     {
//         uint64_t fdt_addr, kernel_align, kernel_base, kernel_size;
//         uint64_t bios_base, bios_size, initrd_base, initrd_size;
//         uint64_t image_start, image_len;
//         uint8_t *ram_ptr;
//         uint32_t *q;
//         int res;
//     
//         ram_ptr = get_ram_ptr(s, 0x80000000, TRUE);
//     
//     
//         if (elf_detect_magic(buf, buf_len)) {
//             if (elf_load(buf, buf_len, ram_ptr, s->ram_size,
//                          &image_start, &image_len) == -1) {
//                 vm_error("Failed to load ELF BIOS\n");
//                 exit(1);
//             }
//             bios_base = image_start - 0x80000000;
//             bios_size = image_len;
//         } else {
//             if (buf_len > s->ram_size) {
//                 vm_error("BIOS too big\n");
//                 exit(1);
//             }
//             memcpy(ram_ptr, buf, buf_len);
//             bios_base = 0x80000000;
//             bios_size = buf_len;
//         }
//     
//     
//         if (kernel_buf_len > 0) {
//             if (s->max_xlen == 32)
//                 kernel_align = 4 << 20;
//             else
//                 kernel_align = 2 << 20;
//             kernel_base = (buf_len + kernel_align - 1) & ~(kernel_align - 1);
//             if (elf_detect_magic(kernel_buf, kernel_buf_len)) {
//                 if (elf_load(kernel_buf, kernel_buf_len,
//                              ram_ptr + kernel_base,
//                              s->ram_size - kernel_base,
//                              &image_start, &image_len) == -1) {
//                     vm_error("Failed to load ELF kernel\n");
//                     exit(1);
//                 }
//                 kernel_base += image_start;
//                 kernel_size = image_len;
//             } else {
//                 memcpy(ram_ptr + kernel_base, kernel_buf, kernel_buf_len);
//                 kernel_size = kernel_buf_len;
//             }
//         } else {
//             kernel_base = 0;
//             kernel_size = 0;
//         }
//     
//     
//         if (initrd_buf_len > 0) {
//             if (kernel_size > 0) {
//                 initrd_base = kernel_base + kernel_size;
//             } else {
//                 initrd_base = bios_base + bios_size;
//             }
//             initrd_base &= ~((1 << 12) - 1);
//             initrd_base += (1 << 12);
//             if (initrd_base + initrd_buf_len > s->ram_size) {
//                 vm_error("initrd too big\n");
//                 exit(1);
//             }
//     
//             if (compress_detect_magic(initrd_buf, initrd_buf_len)) {
//                 res = decompress(initrd_buf, initrd_buf_len,
//                                  ram_ptr + initrd_base,
//                                  s->ram_size - kernel_base);
//                 if (res == -1) {
//                     vm_error("Failed to decompress initramfs image\n");
//                     exit(1);
//                 }
//                 initrd_size = (uint64_t)res;
//             } else {
//     
//     
//     
//                 memcpy(ram_ptr + initrd_base, initrd_buf, initrd_buf_len);
//                 initrd_size = initrd_buf_len;
//             }
//         } else {
//             initrd_base = 0;
//             initrd_size = 0;
//         }
//     
//         ram_ptr = get_ram_ptr(s, 0, TRUE);
//     
//         fdt_addr = 0x1000 + 8 * 8;
//     
//         riscv_build_fdt(s, ram_ptr + fdt_addr,
//                         0x80000000 + kernel_base,
//                         kernel_size, cmd_line,
//                         0x80000000 + initrd_base,
//                         initrd_size);
//     
//     
//     
//         q = (uint32_t *)(ram_ptr + 0x1000);
//         q[0] = 0x297 + 0x80000000 - 0x1000;
//         q[1] = 0x597;
//         q[2] = 0x58593 + ((fdt_addr - 4) << 20);
//         q[3] = 0xf1402573;
//         q[4] = 0x00028067;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2056 {
// Rust port of `copy_bios` from riscv_machine.c.
//
// `get_ram_ptr`, the fdt_* helpers and `riscv_build_fdt` are `static` in the C
// file (no linker symbols), so their logic is ported here as private Rust
// functions.  Their output must be byte-for-byte identical to the C version.

use core::ffi::{c_char, c_int, c_void};

const TRUE: c_int = 1;
const RAM_BASE_ADDR: u64 = 0x8000_0000;

// ---------------------------------------------------------------------------
// opaque C types referenced only through pointers
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
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
pub struct EthernetDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    _private: [u8; 0],
}

// ---------------------------------------------------------------------------
// struct mirrors (exact C layout)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn()>,
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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn()>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

// ---------------------------------------------------------------------------
// external C symbols
// ---------------------------------------------------------------------------

extern "C" {
    fn phys_mem_get_ram_ptr(map: *mut PhysMemoryMap, paddr: u64, is_rw: c_int) -> *mut u8;

    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_load(
        buf: *const u8,
        buf_len: c_int,
        outbuf: *mut u8,
        outbuf_len: c_int,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;

    fn compress_detect_magic(buf: *const u8, len: c_int) -> c_int;
    fn decompress(in_buf: *const u8, in_len: c_int, out_buf: *mut u8, out_len: c_int) -> c_int;

    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
}

// ---------------------------------------------------------------------------
// helpers ported from the C file (they are `static` there)
// ---------------------------------------------------------------------------

unsafe fn get_ram_ptr(s: *mut RISCVMachine, paddr: u64, is_rw: c_int) -> *mut u8 {
    phys_mem_get_ram_ptr((*s).mem_map, paddr, is_rw)
}

// `riscv_cpu_get_misa()` is a static inline in riscv_cpu.h: walk the class
// pointer stored in the common part of the CPU state.
unsafe fn riscv_cpu_get_misa(s: *mut RISCVCPUState) -> u32 {
    if s.is_null() {
        return 0;
    }
    let common = s as *const RISCVCPUCommonState;
    let class_ptr = (*common).class_ptr;
    if class_ptr.is_null() {
        return 0;
    }
    match (*class_ptr).riscv_cpu_get_misa {
        Some(f) => f(s),
        None => 0,
    }
}

// --- flattened device tree builder -----------------------------------------

struct FdtState {
    // structure block, big endian bytes (equivalent to the C uint32_t array)
    tab: Vec<u8>,
    string_table: Vec<u8>,
    open_node_count: i32,
}

impl FdtState {
    fn new() -> FdtState {
        FdtState {
            tab: Vec::new(),
            string_table: Vec::new(),
            open_node_count: 0,
        }
    }

    fn put32(&mut self, v: u32) {
        self.tab.extend_from_slice(&v.to_be_bytes());
    }

    fn put_data(&mut self, data: &[u8]) {
        self.tab.extend_from_slice(data);
        let pad = (4 - (data.len() & 3)) & 3;
        for _ in 0..pad {
            self.tab.push(0);
        }
    }

    fn begin_node(&mut self, name: &[u8]) {
        self.put32(1);
        self.put_data(name);
        self.open_node_count += 1;
    }

    fn begin_node_num(&mut self, name: &[u8], n: u64) {
        // name is NUL terminated ("%s@%lx" with a 256 byte buffer)
        let mut s = String::new();
        for &b in name {
            if b == 0 {
                break;
            }
            s.push(b as char);
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.extend_from_slice(s.as_bytes());
        buf.push(b'@');
        buf.extend_from_slice(format!("{:x}", n).as_bytes());
        if buf.len() > 255 {
            buf.truncate(255);
        }
        buf.push(0);
        self.begin_node(&buf);
    }

    fn end_node(&mut self) {
        self.put32(2);
        self.open_node_count -= 1;
    }

    fn get_string_offset(&mut self, name: &[u8]) -> u32 {
        let mut pos = 0usize;
        while pos < self.string_table.len() {
            let rest = &self.string_table[pos..];
            let end = match rest.iter().position(|&b| b == 0) {
                Some(p) => p + 1,
                None => rest.len(),
            };
            if &rest[..end] == name {
                return pos as u32;
            }
            pos += end;
        }
        let pos = self.string_table.len();
        self.string_table.extend_from_slice(name);
        pos as u32
    }

    fn prop(&mut self, prop_name: &[u8], data: &[u8]) {
        self.put32(3);
        self.put32(data.len() as u32);
        let off = self.get_string_offset(prop_name);
        self.put32(off);
        self.put_data(data);
    }

    fn prop_tab_u32(&mut self, prop_name: &[u8], tab: &[u32]) {
        self.put32(3);
        self.put32((tab.len() * 4) as u32);
        let off = self.get_string_offset(prop_name);
        self.put32(off);
        for &v in tab {
            self.put32(v);
        }
    }

    fn prop_u32(&mut self, prop_name: &[u8], val: u32) {
        self.prop_tab_u32(prop_name, &[val]);
    }

    fn prop_tab_u64(&mut self, prop_name: &[u8], v0: u64) {
        let tab = [(v0 >> 32) as u32, v0 as u32];
        self.prop_tab_u32(prop_name, &tab);
    }

    fn prop_tab_u64_2(&mut self, prop_name: &[u8], v0: u64, v1: u64) {
        let tab = [(v0 >> 32) as u32, v0 as u32, (v1 >> 32) as u32, v1 as u32];
        self.prop_tab_u32(prop_name, &tab);
    }

    // `s` must include the terminating NUL (like fdt_prop_str with strlen + 1)
    fn prop_str(&mut self, prop_name: &[u8], s: &[u8]) {
        self.prop(prop_name, s);
    }

    // variadic fdt_prop_tab_str: concatenation of NUL terminated strings
    fn prop_tab_str(&mut self, prop_name: &[u8], strs: &[&[u8]]) {
        let mut tab: Vec<u8> = Vec::new();
        for s in strs {
            tab.extend_from_slice(s);
        }
        self.prop(prop_name, &tab);
    }

    fn output(&mut self, dst: *mut u8) -> c_int {
        self.put32(9);

        let dt_struct_size = self.tab.len();
        let dt_strings_size = self.string_table.len();

        let mut out: Vec<u8> = Vec::with_capacity(4096);
        // struct fdt_header, filled in at the end
        for _ in 0..10 {
            out.extend_from_slice(&0u32.to_be_bytes());
        }

        let off_dt_struct = out.len();
        out.extend_from_slice(&self.tab);

        while (out.len() & 7) != 0 {
            out.push(0);
        }
        let off_mem_rsvmap = out.len();
        // one empty reserve entry (address = 0, size = 0)
        for _ in 0..16 {
            out.push(0);
        }

        let off_dt_strings = out.len();
        out.extend_from_slice(&self.string_table);
        while (out.len() & 7) != 0 {
            out.push(0);
        }
        let totalsize = out.len();

        let fields: [(usize, u32); 10] = [
            (0, 0xd00dfeed),
            (4, totalsize as u32),
            (8, off_dt_struct as u32),
            (12, off_dt_strings as u32),
            (16, off_mem_rsvmap as u32),
            (20, 17),
            (24, 16),
            (28, 0),
            (32, dt_strings_size as u32),
            (36, dt_struct_size as u32),
        ];
        for &(idx, v) in fields.iter() {
            let b = v.to_be_bytes();
            if idx + 4 <= out.len() {
                out[idx] = b[0];
                out[idx + 1] = b[1];
                out[idx + 2] = b[2];
                out[idx + 3] = b[3];
            }
        }

        if !dst.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(out.as_ptr(), dst, out.len());
            }
        }

        totalsize as c_int
    }
}

unsafe fn riscv_build_fdt(
    m: *mut RISCVMachine,
    dst: *mut u8,
    kernel_start: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_start: u64,
    initrd_size: u64,
) -> c_int {
    let mut st = FdtState::new();
    let mut cur_phandle: u32 = 1;

    st.begin_node(b"\0");
    st.prop_u32(b"#address-cells\0", 2);
    st.prop_u32(b"#size-cells\0", 2);
    st.prop_str(b"compatible\0", b"ucbbar,riscvemu-bar_dev\0");
    st.prop_str(b"model\0", b"ucbbar,riscvemu-bare\0");

    st.begin_node(b"cpus\0");
    st.prop_u32(b"#address-cells\0", 1);
    st.prop_u32(b"#size-cells\0", 0);
    st.prop_u32(b"timebase-frequency\0", 10000000);

    st.begin_node_num(b"cpu\0", 0);
    st.prop_str(b"device_type\0", b"cpu\0");
    st.prop_u32(b"reg\0", 0);
    st.prop_str(b"status\0", b"okay\0");
    st.prop_str(b"compatible\0", b"riscv\0");

    let max_xlen = (*m).max_xlen;
    let misa = riscv_cpu_get_misa((*m).cpu_state);
    let mut isa: Vec<u8> = Vec::new();
    isa.extend_from_slice(format!("rv{}", max_xlen).as_bytes());
    let mut i: c_int = 0;
    while i < 26 {
        if (misa & (1u32 << i)) != 0 {
            isa.push(b'a' + (i as u8));
        }
        i += 1;
    }
    isa.push(0);
    st.prop_str(b"riscv,isa\0", &isa);

    if max_xlen <= 32 {
        st.prop_str(b"mmu-type\0", b"riscv,sv32\0");
    } else {
        st.prop_str(b"mmu-type\0", b"riscv,sv48\0");
    }
    st.prop_u32(b"clock-frequency\0", 2000000000);

    st.begin_node(b"interrupt-controller\0");
    st.prop_u32(b"#interrupt-cells\0", 1);
    st.prop(b"interrupt-controller\0", &[]);
    st.prop_str(b"compatible\0", b"riscv,cpu-intc\0");
    let intc_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    st.prop_u32(b"phandle\0", intc_phandle);
    st.end_node();

    st.end_node();

    st.end_node();

    st.begin_node_num(b"memory\0", 0x8000_0000);
    st.prop_str(b"device_type\0", b"memory\0");
    let ram_size = (*m).ram_size;
    let tab = [
        (0x8000_0000u64 >> 32) as u32,
        0x8000_0000u64 as u32,
        (ram_size >> 32) as u32,
        ram_size as u32,
    ];
    st.prop_tab_u32(b"reg\0", &tab);

    st.end_node();

    st.begin_node(b"htif\0");
    st.prop_str(b"compatible\0", b"ucb,htif0\0");
    st.end_node();

    st.begin_node(b"soc\0");
    st.prop_u32(b"#address-cells\0", 2);
    st.prop_u32(b"#size-cells\0", 2);
    st.prop_tab_str(
        b"compatible\0",
        &[b"ucbbar,riscvemu-bar-soc\0", b"simple-bus\0"],
    );
    st.prop(b"ranges\0", &[]);

    st.begin_node_num(b"clint\0", 0x02000000);
    st.prop_str(b"compatible\0", b"riscv,clint0\0");

    let tab = [intc_phandle, 3, intc_phandle, 7];
    st.prop_tab_u32(b"interrupts-extended\0", &tab);

    st.prop_tab_u64_2(b"reg\0", 0x02000000, 0x000c0000);

    st.end_node();

    st.begin_node_num(b"plic\0", 0x40100000);
    st.prop_u32(b"#interrupt-cells\0", 1);
    st.prop(b"interrupt-controller\0", &[]);
    st.prop_str(b"compatible\0", b"riscv,plic0\0");
    st.prop_u32(b"riscv,ndev\0", 31);
    st.prop_tab_u64_2(b"reg\0", 0x40100000, 0x00400000);

    let tab = [intc_phandle, 9, intc_phandle, 11];
    st.prop_tab_u32(b"interrupts-extended\0", &tab);

    let plic_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    st.prop_u32(b"phandle\0", plic_phandle);

    st.end_node();

    let virtio_count = (*m).virtio_count;
    let mut i: c_int = 0;
    while i < virtio_count {
        let addr = 0x40010000u64.wrapping_add((i as u64).wrapping_mul(0x1000));
        st.begin_node_num(b"virtio\0", addr);
        st.prop_str(b"compatible\0", b"virtio,mmio\0");
        st.prop_tab_u64_2(b"reg\0", addr, 0x1000);
        let tab = [plic_phandle, 1u32.wrapping_add(i as u32)];
        st.prop_tab_u32(b"interrupts-extended\0", &tab);
        st.end_node();
        i += 1;
    }

    let fb_dev = (*m).common.fb_dev;
    if !fb_dev.is_null() {
        st.begin_node_num(b"framebuffer\0", 0x41000000);
        st.prop_str(b"compatible\0", b"simple-framebuffer\0");
        st.prop_tab_u64_2(b"reg\0", 0x41000000, (*fb_dev).fb_size as i64 as u64);
        st.prop_u32(b"width\0", (*fb_dev).width as u32);
        st.prop_u32(b"height\0", (*fb_dev).height as u32);
        st.prop_u32(b"stride\0", (*fb_dev).stride as u32);
        st.prop_str(b"format\0", b"a8r8g8b8\0");
        st.end_node();
    }

    st.end_node();

    st.begin_node(b"chosen\0");
    let bootargs: Vec<u8> = if cmd_line.is_null() {
        let mut v = Vec::new();
        v.push(0u8);
        v
    } else {
        let mut v = Vec::new();
        let mut p = cmd_line;
        loop {
            let c = *p;
            v.push(c as u8);
            if c == 0 {
                break;
            }
            p = p.add(1);
        }
        v
    };
    st.prop_str(b"bootargs\0", &bootargs);
    if kernel_size > 0 {
        st.prop_tab_u64(b"riscv,kernel-start\0", kernel_start);
        st.prop_tab_u64(b"riscv,kernel-end\0", kernel_start.wrapping_add(kernel_size));
    }
    if initrd_size > 0 {
        st.prop_tab_u64(b"linux,initrd-start\0", initrd_start);
        st.prop_tab_u64(b"linux,initrd-end\0", initrd_start.wrapping_add(initrd_size));
    }

    st.end_node();

    st.end_node();

    st.output(dst)
}

// ---------------------------------------------------------------------------
// the target function
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn copy_bios(
    s: *mut RISCVMachine,
    buf: *const u8,
    buf_len: c_int,
    kernel_buf: *const u8,
    kernel_buf_len: c_int,
    cmd_line: *const c_char,
    initrd_buf: *const u8,
    initrd_buf_len: c_int,
) {
    let mut ram_ptr: *mut u8 = get_ram_ptr(s, RAM_BASE_ADDR, TRUE);
    let ram_size = (*s).ram_size;

    let mut image_start: u64 = 0;
    let mut image_len: u64 = 0;

    let bios_base: u64;
    let bios_size: u64;

    // copy the bios
    if elf_detect_magic(buf, buf_len) != 0 {
        if elf_load(
            buf,
            buf_len,
            ram_ptr,
            ram_size as c_int,
            &mut image_start as *mut u64,
            &mut image_len as *mut u64,
        ) == -1
        {
            vm_error(b"Failed to load ELF BIOS\n\0".as_ptr() as *const c_char);
            exit(1);
        }
        bios_base = image_start.wrapping_sub(RAM_BASE_ADDR);
        bios_size = image_len;
    } else {
        if (buf_len as i64 as u64) > ram_size {
            vm_error(b"BIOS too big\n\0".as_ptr() as *const c_char);
            exit(1);
        }
        core::ptr::copy_nonoverlapping(buf, ram_ptr, buf_len as usize);
        bios_base = RAM_BASE_ADDR;
        bios_size = buf_len as i64 as u64;
    }

    // copy the kernel if present
    let mut kernel_base: u64;
    let kernel_size: u64;

    if kernel_buf_len > 0 {
        let kernel_align: u64 = if (*s).max_xlen == 32 {
            4u64 << 20
        } else {
            2u64 << 20
        };
        kernel_base =
            (buf_len as i64 as u64).wrapping_add(kernel_align).wrapping_sub(1)
                & !(kernel_align - 1);
        if elf_detect_magic(kernel_buf, kernel_buf_len) != 0 {
            if elf_load(
                kernel_buf,
                kernel_buf_len,
                ram_ptr.add(kernel_base as usize),
                ram_size.wrapping_sub(kernel_base) as c_int,
                &mut image_start as *mut u64,
                &mut image_len as *mut u64,
            ) == -1
            {
                vm_error(b"Failed to load ELF kernel\n\0".as_ptr() as *const c_char);
                exit(1);
            }
            kernel_base = kernel_base.wrapping_add(image_start);
            kernel_size = image_len;
        } else {
            core::ptr::copy_nonoverlapping(
                kernel_buf,
                ram_ptr.add(kernel_base as usize),
                kernel_buf_len as usize,
            );
            kernel_size = kernel_buf_len as i64 as u64;
        }
    } else {
        kernel_base = 0;
        kernel_size = 0;
    }

    // copy the initrd if present
    let initrd_base: u64;
    let initrd_size: u64;

    if initrd_buf_len > 0 {
        let mut base: u64;
        if kernel_size > 0 {
            base = kernel_base.wrapping_add(kernel_size);
        } else {
            base = bios_base.wrapping_add(bios_size);
        }
        base &= !(0x1000u64 - 1);
        base = base.wrapping_add(0x1000);
        if base.wrapping_add(initrd_buf_len as i64 as u64) > ram_size {
            vm_error(b"initrd too big\n\0".as_ptr() as *const c_char);
            exit(1);
        }
        initrd_base = base;

        if compress_detect_magic(initrd_buf, initrd_buf_len) != 0 {
            let res = decompress(
                initrd_buf,
                initrd_buf_len,
                ram_ptr.add(initrd_base as usize),
                ram_size.wrapping_sub(kernel_base) as c_int,
            );
            if res == -1 {
                vm_error(b"Failed to decompress initramfs image\n\0".as_ptr() as *const c_char);
                exit(1);
            }
            initrd_size = res as i64 as u64;
        } else {
            core::ptr::copy_nonoverlapping(
                initrd_buf,
                ram_ptr.add(initrd_base as usize),
                initrd_buf_len as usize,
            );
            initrd_size = initrd_buf_len as i64 as u64;
        }
    } else {
        initrd_base = 0;
        initrd_size = 0;
    }

    ram_ptr = get_ram_ptr(s, 0, TRUE);

    let fdt_addr: u64 = 0x1000 + 8 * 8;

    riscv_build_fdt(
        s,
        ram_ptr.add(fdt_addr as usize),
        RAM_BASE_ADDR.wrapping_add(kernel_base),
        kernel_size,
        cmd_line,
        RAM_BASE_ADDR.wrapping_add(initrd_base),
        initrd_size,
    );

    // jump_addr = 0x80000000
    let q = ram_ptr.add(0x1000) as *mut u32;
    q.write_unaligned((0x297u64 + 0x8000_0000 - 0x1000) as u32);
    q.add(1).write_unaligned(0x597u32);
    q.add(2)
        .write_unaligned((0x58593u64 + (fdt_addr.wrapping_sub(4) << 20)) as u32);
    q.add(3).write_unaligned(0xf1402573u32);
    q.add(4).write_unaligned(0x00028067u32);
}
}

// ── `uint64_t fdt_addr, kernel_align, kernel_base, kernel_size; uint64_t bios_base, bios_size, initr…` · riscv_machine.c:759-766 · chunk 2057 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint64_t fdt_addr, kernel_align, kernel_base, kernel_size;
//         uint64_t bios_base, bios_size, initrd_base, initrd_size;
//         uint64_t image_start, image_len;
//         uint8_t *ram_ptr;
//         uint32_t *q;
//         int res;
//     
//         ram_ptr = get_ram_ptr(s, 0x80000000, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2057 {
// Port of the RAM pointer setup part of copy_bios().
use core::ffi::{c_int, c_void};

// Mirror of the leading fields of the C structures (only the ones reachable here).
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
}

extern "C" {
    fn phys_mem_get_ram_ptr(map: *mut c_void, paddr: u64, is_rw: c_int) -> *mut u8;
}

// Port of the C static function get_ram_ptr().
fn get_ram_ptr(s: *mut RISCVMachine, paddr: u64, is_rw: c_int) -> *mut u8 {
    unsafe { phys_mem_get_ram_ptr((*s).mem_map, paddr, is_rw) }
}

#[no_mangle]
pub extern "C" fn copy_bios__part2057(s: *mut RISCVMachine) -> *mut u8 {
    // TRUE is 1 in the C program.
    get_ram_ptr(s, 0x80000000u64, 1)
}
}

// ── `if (elf_detect_magic(buf, buf_len)) … else` · riscv_machine.c:769-785 · chunk 2058 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (elf_detect_magic(buf, buf_len)) {
//             if (elf_load(buf, buf_len, ram_ptr, s->ram_size,
//                          &image_start, &image_len) == -1) {
//                 vm_error("Failed to load ELF BIOS\n");
//                 exit(1);
//             }
//             bios_base = image_start - 0x80000000;
//             bios_size = image_len;
//         } else {
//             if (buf_len > s->ram_size) {
//                 vm_error("BIOS too big\n");
//                 exit(1);
//             }
//             memcpy(ram_ptr, buf, buf_len);
//             bios_base = 0x80000000;
//             bios_size = buf_len;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2058 {
// Rust rewrite of the BIOS loading part of copy_bios().
use core::ffi::{c_int, c_void};

extern "C" {
    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_load(
        buf: *const u8,
        buf_len: c_int,
        outbuf: *mut u8,
        outbuf_len: c_int,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/// Returns 0 on success, 1 for "Failed to load ELF BIOS",
/// 2 for "BIOS too big" (the caller prints the message and exits).
#[no_mangle]
pub unsafe extern "C" fn copy_bios__part2058(
    buf: *const u8,
    buf_len: c_int,
    ram_ptr: *mut u8,
    ram_size: u64,
    bios_base: *mut u64,
    bios_size: *mut u64,
) -> c_int {
    let mut image_start: u64 = 0;
    let mut image_len: u64 = 0;

    if elf_detect_magic(buf, buf_len) != 0 {
        if elf_load(
            buf,
            buf_len,
            ram_ptr,
            ram_size as c_int,
            &mut image_start,
            &mut image_len,
        ) == -1
        {
            return 1;
        }
        *bios_base = image_start.wrapping_sub(0x80000000u64);
        *bios_size = image_len;
    } else {
        if (buf_len as u64) > ram_size {
            return 2;
        }
        memcpy(
            ram_ptr as *mut c_void,
            buf as *const c_void,
            buf_len as usize,
        );
        *bios_base = 0x80000000u64;
        *bios_size = buf_len as u64;
    }

    0
}
}

// ── `if (kernel_buf_len > 0) … else` · riscv_machine.c:788-811 · chunk 2059 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (kernel_buf_len > 0) {
//             if (s->max_xlen == 32)
//                 kernel_align = 4 << 20;
//             else
//                 kernel_align = 2 << 20;
//             kernel_base = (buf_len + kernel_align - 1) & ~(kernel_align - 1);
//             if (elf_detect_magic(kernel_buf, kernel_buf_len)) {
//                 if (elf_load(kernel_buf, kernel_buf_len,
//                              ram_ptr + kernel_base,
//                              s->ram_size - kernel_base,
//                              &image_start, &image_len) == -1) {
//                     vm_error("Failed to load ELF kernel\n");
//                     exit(1);
//                 }
//                 kernel_base += image_start;
//                 kernel_size = image_len;
//             } else {
//                 memcpy(ram_ptr + kernel_base, kernel_buf, kernel_buf_len);
//                 kernel_size = kernel_buf_len;
//             }
//         } else {
//             kernel_base = 0;
//             kernel_size = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2059 {
// Port of the kernel-loading part of copy_bios (riscv_machine.c).
use core::ffi::{c_char, c_int};

extern "C" {
    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_load(
        buf: *const u8,
        buf_len: c_int,
        outbuf: *mut u8,
        outbuf_len: c_int,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn copy_bios__part2059(
    max_xlen: c_int,
    buf_len: c_int,
    kernel_buf: *const u8,
    kernel_buf_len: c_int,
    ram_ptr: *mut u8,
    ram_size: u64,
    kernel_base_out: *mut u64,
    kernel_size_out: *mut u64,
) {
    unsafe {
        let kernel_base: u64;
        let kernel_size: u64;

        if kernel_buf_len > 0 {
            let kernel_align: u64 = if max_xlen == 32 { 4u64 << 20 } else { 2u64 << 20 };
            let mut kbase: u64 =
                (buf_len as u64).wrapping_add(kernel_align).wrapping_sub(1)
                    & !(kernel_align.wrapping_sub(1));
            if elf_detect_magic(kernel_buf, kernel_buf_len) != 0 {
                let mut image_start: u64 = 0;
                let mut image_len: u64 = 0;
                if elf_load(
                    kernel_buf,
                    kernel_buf_len,
                    ram_ptr.wrapping_offset(kbase as isize),
                    ram_size.wrapping_sub(kbase) as c_int,
                    &mut image_start,
                    &mut image_len,
                ) == -1
                {
                    vm_error(b"Failed to load ELF kernel\n\0".as_ptr() as *const c_char);
                    exit(1);
                }
                kbase = kbase.wrapping_add(image_start);
                kernel_base = kbase;
                kernel_size = image_len;
            } else {
                memcpy(
                    ram_ptr.wrapping_offset(kbase as isize),
                    kernel_buf,
                    kernel_buf_len as usize,
                );
                kernel_base = kbase;
                kernel_size = kernel_buf_len as u64;
            }
        } else {
            kernel_base = 0;
            kernel_size = 0;
        }

        *kernel_base_out = kernel_base;
        *kernel_size_out = kernel_size;
    }
}
}

// ── `if (initrd_buf_len > 0) … else` · riscv_machine.c:814-846 · chunk 2060 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (initrd_buf_len > 0) {
//             if (kernel_size > 0) {
//                 initrd_base = kernel_base + kernel_size;
//             } else {
//                 initrd_base = bios_base + bios_size;
//             }
//             initrd_base &= ~((1 << 12) - 1);
//             initrd_base += (1 << 12);
//             if (initrd_base + initrd_buf_len > s->ram_size) {
//                 vm_error("initrd too big\n");
//                 exit(1);
//             }
//     
//             if (compress_detect_magic(initrd_buf, initrd_buf_len)) {
//                 res = decompress(initrd_buf, initrd_buf_len,
//                                  ram_ptr + initrd_base,
//                                  s->ram_size - kernel_base);
//                 if (res == -1) {
//                     vm_error("Failed to decompress initramfs image\n");
//                     exit(1);
//                 }
//                 initrd_size = (uint64_t)res;
//             } else {
//     
//     
//     
//                 memcpy(ram_ptr + initrd_base, initrd_buf, initrd_buf_len);
//                 initrd_size = initrd_buf_len;
//             }
//         } else {
//             initrd_base = 0;
//             initrd_size = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2060 {
// Port of the initrd placement/loading piece of copy_bios().
// Only ram_size is read from the RISCVMachine struct, so it is passed by value.
use core::ffi::{c_char, c_int};

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn compress_detect_magic(buf: *const u8, len: c_int) -> c_int;
    fn decompress(in_buf: *const u8, in_len: c_int, out_buf: *const u8, out_len: c_int) -> c_int;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn copy_bios__part2060(
    ram_ptr: *mut u8,
    bios_base: u64,
    bios_size: u64,
    kernel_base: u64,
    kernel_size: u64,
    ram_size: u64,
    initrd_buf: *const u8,
    initrd_buf_len: c_int,
    initrd_base_out: *mut u64,
    initrd_size_out: *mut u64,
) {
    unsafe {
        if initrd_buf_len > 0 {
            let mut initrd_base: u64;
            if kernel_size > 0 {
                initrd_base = kernel_base.wrapping_add(kernel_size);
            } else {
                initrd_base = bios_base.wrapping_add(bios_size);
            }
            // ~((1 << 12) - 1) computed as int then widened: 0xFFFFFFFFFFFFF000
            initrd_base &= !4095u64;
            initrd_base = initrd_base.wrapping_add(4096);
            if initrd_base.wrapping_add(initrd_buf_len as u64) > ram_size {
                vm_error(b"initrd too big\n\0".as_ptr() as *const c_char);
                exit(1);
            }

            let initrd_size: u64;
            if compress_detect_magic(initrd_buf, initrd_buf_len) != 0 {
                let res: c_int = decompress(
                    initrd_buf,
                    initrd_buf_len,
                    ram_ptr.add(initrd_base as usize) as *const u8,
                    ram_size.wrapping_sub(kernel_base) as c_int,
                );
                if res == -1 {
                    vm_error(b"Failed to decompress initramfs image\n\0".as_ptr() as *const c_char);
                    exit(1);
                }
                initrd_size = res as u64;
            } else {
                memcpy(
                    ram_ptr.add(initrd_base as usize),
                    initrd_buf,
                    initrd_buf_len as usize,
                );
                initrd_size = initrd_buf_len as u64;
            }

            *initrd_base_out = initrd_base;
            *initrd_size_out = initrd_size;
        } else {
            *initrd_base_out = 0;
            *initrd_size_out = 0;
        }
    }
}
}

// ── `ram_ptr = get_ram_ptr(s, 0, TRUE); fdt_addr = 0x1000 + 8 * 8; riscv_build_fdt(s, ram_ptr + fdt_…` · riscv_machine.c:848-865 · chunk 2061 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ram_ptr = get_ram_ptr(s, 0, TRUE);
//     
//         fdt_addr = 0x1000 + 8 * 8;
//     
//         riscv_build_fdt(s, ram_ptr + fdt_addr,
//                         0x80000000 + kernel_base,
//                         kernel_size, cmd_line,
//                         0x80000000 + initrd_base,
//                         initrd_size);
//     
//     
//     
//         q = (uint32_t *)(ram_ptr + 0x1000);
//         q[0] = 0x297 + 0x80000000 - 0x1000;
//         q[1] = 0x597;
//         q[2] = 0x58593 + ((fdt_addr - 4) << 20);
//         q[3] = 0xf1402573;
//         q[4] = 0x00028067;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2061 {
// Port of the FDT-building / boot-entry tail of copy_bios().
// get_ram_ptr() and riscv_build_fdt() are static C functions (no linker
// symbol), so they are supplied by the caller as function pointers.

use core::ffi::{c_char, c_int, c_void};

pub type GetRamPtrFn = unsafe extern "C" fn(s: *mut c_void, paddr: u64, is_rw: c_int) -> *mut u8;

pub type BuildFdtFn = unsafe extern "C" fn(
    m: *mut c_void,
    dst: *mut u8,
    kernel_start: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_start: u64,
    initrd_size: u64,
) -> c_int;

#[no_mangle]
pub extern "C" fn copy_bios__part2061(
    s: *mut c_void,
    p_fdt_addr: *mut u64,
    p_q: *mut *mut u32,
    kernel_base: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_base: u64,
    initrd_size: u64,
    get_ram_ptr: GetRamPtrFn,
    build_fdt: BuildFdtFn,
) -> *mut u8 {
    unsafe {
        // ram_ptr = get_ram_ptr(s, 0, TRUE);
        let ram_ptr = get_ram_ptr(s, 0, 1);

        // fdt_addr = 0x1000 + 8 * 8;
        let fdt_addr: u64 = 0x1000 + 8 * 8;
        if !p_fdt_addr.is_null() {
            *p_fdt_addr = fdt_addr;
        }

        // riscv_build_fdt(s, ram_ptr + fdt_addr, 0x80000000 + kernel_base,
        //                 kernel_size, cmd_line, 0x80000000 + initrd_base,
        //                 initrd_size);
        build_fdt(
            s,
            ram_ptr.add(fdt_addr as usize),
            0x80000000u64.wrapping_add(kernel_base),
            kernel_size,
            cmd_line,
            0x80000000u64.wrapping_add(initrd_base),
            initrd_size,
        );

        // q = (uint32_t *)(ram_ptr + 0x1000);
        let q = ram_ptr.add(0x1000) as *mut u32;
        if !p_q.is_null() {
            *p_q = q;
        }

        // q[0] = 0x297 + 0x80000000 - 0x1000;
        q.write(0x297u32.wrapping_add(0x80000000).wrapping_sub(0x1000));
        // q[1] = 0x597;
        q.add(1).write(0x597u32);
        // q[2] = 0x58593 + ((fdt_addr - 4) << 20);
        let mut v2 = (fdt_addr.wrapping_sub(4)) << 20;
        v2 = v2.wrapping_add(0x58593);
        q.add(2).write(v2 as u32);
        // q[3] = 0xf1402573;
        q.add(3).write(0xf1402573u32);
        // q[4] = 0x00028067;
        q.add(4).write(0x00028067u32);

        ram_ptr
    }
}
}

// ── `riscv_flush_tlb_write_range` · riscv_machine.c:868-873 · chunk 2062 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_flush_tlb_write_range(void *opaque, uint8_t *ram_addr,
//                                             size_t ram_size)
//     {
//         RISCVMachine *s = opaque;
//         riscv_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2062 {
use core::ffi::{c_int, c_void};

// Mirror of the C types needed to reach `s->cpu_state` and dispatch through
// the RISCVCPUClass vtable. Only the fields up to `cpu_state` are accessed,
// but the full preceding layout is reproduced so the offset is exact.

pub type BOOL = c_int;

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
}

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

pub type RISCVFlushTlbWriteRangeRamFn =
    unsafe extern "C" fn(s: *mut RISCVCPUState, ram_ptr: *mut u8, ram_size: usize);

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: *mut c_void,
    pub riscv_cpu_end: *mut c_void,
    pub riscv_cpu_interp: *mut c_void,
    pub riscv_cpu_get_cycles: *mut c_void,
    pub riscv_cpu_set_mip: *mut c_void,
    pub riscv_cpu_reset_mip: *mut c_void,
    pub riscv_cpu_get_mip: *mut c_void,
    pub riscv_cpu_get_power_down: *mut c_void,
    pub riscv_cpu_get_misa: *mut c_void,
    pub riscv_cpu_flush_tlb_write_range_ram: Option<RISCVFlushTlbWriteRangeRamFn>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[no_mangle]
pub unsafe extern "C" fn riscv_flush_tlb_write_range(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut RISCVMachine;
    let cpu_state = (*s).cpu_state;
    // riscv_cpu_flush_tlb_write_range_ram(): static inline vtable dispatch.
    let c = (*((cpu_state as *mut RISCVCPUCommonState))).class_ptr;
    if let Some(f) = (*c).riscv_cpu_flush_tlb_write_range_ram {
        f(cpu_state, ram_addr, ram_size);
    }
}
}

// ── `RISCVMachine *s = opaque; riscv_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);` · riscv_machine.c:871-872 · chunk 2063 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = opaque;
//         riscv_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2063 {
// Port of riscv_machine.c riscv_flush_tlb_write_range (lines 871-872):
// fetch the machine's CPU state and dispatch through the CPU class vtable
// to flush the TLB for the given RAM write range.
use core::ffi::{c_int, c_void};

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
pub struct RiskVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

pub type RiscvCpuFlushTlbWriteRangeRam =
    Option<unsafe extern "C" fn(s: *mut c_void, ram_ptr: *mut u8, ram_size: usize)>;

#[repr(C)]
pub struct RiscvCpuClass {
    pub riscv_cpu_init: *mut c_void,
    pub riscv_cpu_end: *mut c_void,
    pub riscv_cpu_interp: *mut c_void,
    pub riscv_cpu_get_cycles: *mut c_void,
    pub riscv_cpu_set_mip: *mut c_void,
    pub riscv_cpu_reset_mip: *mut c_void,
    pub riscv_cpu_get_mip: *mut c_void,
    pub riscv_cpu_get_power_down: *mut c_void,
    pub riscv_cpu_get_misa: *mut c_void,
    pub riscv_cpu_flush_tlb_write_range_ram: RiscvCpuFlushTlbWriteRangeRam,
}

#[repr(C)]
pub struct RiscvCpuCommonState {
    pub class_ptr: *const RiscvCpuClass,
}

#[no_mangle]
pub unsafe extern "C" fn riscv_flush_tlb_write_range__part2063(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    if opaque.is_null() {
        return;
    }
    let s = opaque as *mut RiskVMachine;
    let cpu_state = (*s).cpu_state;
    if cpu_state.is_null() {
        return;
    }
    let class_ptr = (*(cpu_state as *const RiscvCpuCommonState)).class_ptr;
    if class_ptr.is_null() {
        return;
    }
    if let Some(f) = (*class_ptr).riscv_cpu_flush_tlb_write_range_ram {
        f(cpu_state, ram_addr, ram_size);
    }
}
}

// ── `riscv_machine_set_defaults` · riscv_machine.c:875-877 · chunk 2064 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_machine_set_defaults(VirtMachineParams *p)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2064 {
// Port of riscv_machine.c:riscv_machine_set_defaults
// The C original has an empty body: it is a no-op placeholder.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn riscv_machine_set_defaults(_p: *mut c_void) {}
}

// ── `riscv_machine_init` · riscv_machine.c:879-1041 · chunk 2065 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static VirtMachine *riscv_machine_init(const VirtMachineParams *p)
//     {
//         RISCVMachine *s;
//         VIRTIODevice *blk_dev;
//         int irq_num, i, max_xlen, ram_flags;
//         VIRTIOBusDef vbus_s, *vbus = &vbus_s;
//     
//     
//         if (!strcmp(p->machine_name, "riscv32")) {
//             max_xlen = 32;
//         } else if (!strcmp(p->machine_name, "riscv64")) {
//             max_xlen = 64;
//         } else if (!strcmp(p->machine_name, "riscv128")) {
//             max_xlen = 128;
//         } else {
//             vm_error("unsupported machine: %s\n", p->machine_name);
//             return 
//                   ((void *)0)
//                       ;
//         }
//     
//         s = mallocz(sizeof(*s));
//         s->common.vmc = p->vmc;
//         s->ram_size = p->ram_size;
//         s->max_xlen = max_xlen;
//         s->mem_map = phys_mem_map_init();
//     
//         s->mem_map->opaque = s;
//         s->mem_map->flush_tlb_write_range = riscv_flush_tlb_write_range;
//     
//         s->cpu_state = riscv_cpu_init(s->mem_map, max_xlen);
//         if (!s->cpu_state) {
//             vm_error("unsupported max_xlen=%d\n", max_xlen);
//     
//             return 
//                   ((void *)0)
//                       ;
//         }
//     
//     
//         uint64_t htif_start = 0x40008000;
//         if (p->files[VM_FILE_BIOS].buf) {
//             const VMFileEntry *bios = &p->files[VM_FILE_BIOS];
//             if (elf_detect_magic(bios->buf, bios->len)) {
//                 uint64_t addr;
//                 if (elf_find_section(bios->buf, ".htif", &addr, 
//                                                                ((void *)0)
//                                                                    )) {
//                     htif_start = addr + 8;
//                 }
//             }
//         }
//         cpu_register_device(s->mem_map, htif_start, 16,
//                             s, htif_read, htif_write, (1 << 2));
//     
//     
//         ram_flags = 0;
//         cpu_register_ram(s->mem_map, 0x80000000, p->ram_size, ram_flags);
//         cpu_register_ram(s->mem_map, 0x00000000, 0x00010000, 0);
//         s->rtc_real_time = p->rtc_real_time;
//         if (p->rtc_real_time) {
//             s->rtc_start_time = rtc_get_real_time(s);
//         }
//     
//         cpu_register_device(s->mem_map, 0x02000000, 0x000c0000, s,
//                             clint_read, clint_write, (1 << 2));
//         cpu_register_device(s->mem_map, 0x40100000, 0x00400000, s,
//                             plic_read, plic_write, (1 << 2));
//         for(i = 1; i < 32; i++) {
//             irq_init(&s->plic_irq[i], plic_set_irq, s, i);
//         }
//     
//         s->common.console = p->console;
//     
//         memset(vbus, 0, sizeof(*vbus));
//         vbus->mem_map = s->mem_map;
//         vbus->addr = 0x40010000;
//         irq_num = 1;
//     
//     
//         if (p->console) {
//             vbus->irq = &s->plic_irq[irq_num];
//             s->common.console_dev = virtio_console_init(vbus, p->console);
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
//     
//     
//         for(i = 0; i < p->eth_count; i++) {
//             vbus->irq = &s->plic_irq[irq_num];
//             virtio_net_init(vbus, p->tab_eth[i].net);
//             s->common.net = p->tab_eth[i].net;
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
//     
//     
//         for(i = 0; i < p->drive_count; i++) {
//             vbus->irq = &s->plic_irq[irq_num];
//             blk_dev = virtio_block_init(vbus, p->tab_drive[i].block_dev);
//             (void)blk_dev;
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
//     
//     
//         for(i = 0; i < p->fs_count; i++) {
//             VIRTIODevice *fs_dev;
//             vbus->irq = &s->plic_irq[irq_num];
//             fs_dev = virtio_9p_init(vbus, p->tab_fs[i].fs_dev,
//                                     p->tab_fs[i].tag);
//             (void)fs_dev;
//     
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
//     
//         if (p->display_device) {
//             FBDevice *fb_dev;
//             fb_dev = mallocz(sizeof(*fb_dev));
//             s->common.fb_dev = fb_dev;
//             if (!strcmp(p->display_device, "simplefb")) {
//                 simplefb_init(s->mem_map,
//                               0x41000000,
//                               fb_dev,
//                               p->width, p->height);
//     
//             } else {
//                 vm_error("unsupported display device: %s\n", p->display_device);
//                 exit(1);
//             }
//         }
//     
//         if (p->input_device) {
//             if (!strcmp(p->input_device, "virtio")) {
//                 vbus->irq = &s->plic_irq[irq_num];
//                 s->keyboard_dev = virtio_input_init(vbus,
//                                                     VIRTIO_INPUT_TYPE_KEYBOARD);
//                 vbus->addr += 0x1000;
//                 irq_num++;
//                 s->virtio_count++;
//     
//                 vbus->irq = &s->plic_irq[irq_num];
//                 s->mouse_dev = virtio_input_init(vbus,
//                                                  VIRTIO_INPUT_TYPE_TABLET);
//                 vbus->addr += 0x1000;
//                 irq_num++;
//                 s->virtio_count++;
//             } else {
//                 vm_error("unsupported input device: %s\n", p->input_device);
//                 exit(1);
//             }
//         }
//     
//         if (!p->files[VM_FILE_BIOS].buf) {
//             vm_error("No bios found");
//         }
//     
//         copy_bios(s, p->files[VM_FILE_BIOS].buf, p->files[VM_FILE_BIOS].len,
//                   p->files[VM_FILE_KERNEL].buf, p->files[VM_FILE_KERNEL].len,
//                   p->cmdline,
//                   p->files[VM_FILE_INITRD].buf, p->files[VM_FILE_INITRD].len);
//     
//         return (VirtMachine *)s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2065 {
// Rust port of riscv_machine_init (riscv_machine.c) plus the static helpers it
// needs, which have no linker symbols in the C program.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_mut)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

/* ------------------------------------------------------------------ */
/* constants                                                           */
/* ------------------------------------------------------------------ */

const LOW_RAM_SIZE: u64 = 0x00010000;
const RAM_BASE_ADDR: u64 = 0x8000_0000;
const CLINT_BASE_ADDR: u64 = 0x0200_0000;
const CLINT_SIZE: u64 = 0x000c_0000;
const DEFAULT_HTIF_BASE_ADDR: u64 = 0x4000_8000;
const VIRTIO_BASE_ADDR: u64 = 0x4001_0000;
const VIRTIO_SIZE: u64 = 0x1000;
const VIRTIO_IRQ: c_int = 1;
const PLIC_BASE_ADDR: u64 = 0x4010_0000;
const PLIC_SIZE: u64 = 0x0040_0000;
const FRAMEBUFFER_BASE_ADDR: u64 = 0x4100_0000;

const RTC_FREQ: u64 = 10_000_000;
const RTC_FREQ_DIV: u64 = 16;

const VM_FILE_BIOS: usize = 0;
const VM_FILE_KERNEL: usize = 2;
const VM_FILE_INITRD: usize = 3;

const DEVIO_SIZE32: c_int = 1 << 2;
const DEVRAM_PAGE_SIZE: u64 = 0x1000;

const VIRTIO_INPUT_TYPE_KEYBOARD: c_int = 0;
const VIRTIO_INPUT_TYPE_TABLET: c_int = 2;

const MIP_MTIP: u32 = 1 << 7;
const MIP_SEIP: u32 = 1 << 9;
const MIP_MEIP: u32 = 1 << 11;

const FDT_MAGIC: u32 = 0xd00d_feed;
const FDT_VERSION: u32 = 17;

const SOC_COMPAT: &[u8] = b"ucbbar,riscvemu-bar-soc\0simple-bus\0";

/* ------------------------------------------------------------------ */
/* type mirrors                                                        */
/* ------------------------------------------------------------------ */

pub enum BlockDevice {}
pub enum EthernetDevice {}
pub enum FSDevice {}
pub enum CharacterDeviceOpaque {}
pub enum VIRTIODevice {}
pub enum RISCVCPUState {}
pub enum PCIBus {}
pub enum SimpleFBState {}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
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
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
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
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int)>,
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
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, c_int, u16)>,
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
pub struct VIRTIOBusDef {
    pub pci_bus: *mut PCIBus,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct FDTState {
    pub tab: *mut u32,
    pub tab_len: c_int,
    pub tab_size: c_int,
    pub open_node_count: c_int,
    pub string_table: *mut c_char,
    pub string_table_len: c_int,
    pub string_table_size: c_int,
}

#[repr(C)]
pub struct FdtHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[repr(C)]
pub struct FdtReserveEntry {
    pub address: u64,
    pub size: u64,
}

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/* ------------------------------------------------------------------ */
/* C symbols                                                           */
/* ------------------------------------------------------------------ */

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(p: *mut c_void, size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn exit(status: c_int);

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;

    fn phys_mem_map_init() -> *mut PhysMemoryMap;
    fn phys_mem_get_ram_ptr(map: *mut PhysMemoryMap, paddr: u64, is_rw: c_int) -> *mut u8;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
        write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
        opaque: *mut c_void,
        irq_num: c_int,
    );

    fn riscv_cpu_init(mem_map: *mut PhysMemoryMap, max_xlen: c_int) -> *mut RISCVCPUState;

    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_load(
        buf: *const u8,
        buf_len: c_int,
        outbuf: *mut u8,
        outbuf_len: c_int,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
    fn elf_find_section(
        buf: *const u8,
        name: *const c_char,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
    fn compress_detect_magic(buf: *const u8, len: c_int) -> c_int;
    fn decompress(in_buf: *const u8, in_len: c_int, out_buf: *mut u8, out_len: c_int) -> c_int;

    fn virtio_console_init(bus: *mut VIRTIOBusDef, cs: *mut CharacterDevice)
        -> *mut VIRTIODevice;
    fn virtio_net_init(bus: *mut VIRTIOBusDef, es: *mut EthernetDevice) -> *mut VIRTIODevice;
    fn virtio_block_init(bus: *mut VIRTIOBusDef, bs: *mut BlockDevice) -> *mut VIRTIODevice;
    fn virtio_9p_init(
        bus: *mut VIRTIOBusDef,
        fs: *mut FSDevice,
        mount_tag: *const c_char,
    ) -> *mut VIRTIODevice;
    fn virtio_input_init(bus: *mut VIRTIOBusDef, type_: c_int) -> *mut VIRTIODevice;

    fn simplefb_init(
        map: *mut PhysMemoryMap,
        phys_addr: u64,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
    ) -> *mut SimpleFBState;
}

/* ------------------------------------------------------------------ */
/* small helpers                                                       */
/* ------------------------------------------------------------------ */

#[inline]
fn cs(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn cpu_to_be32(v: u32) -> u32 {
    v.swap_bytes()
}

#[inline]
fn abort_now() -> ! {
    std::process::abort()
}

unsafe fn plic_irq_ptr(s: *mut RISCVMachine, idx: c_int) -> *mut IRQSignal {
    ptr::addr_of_mut!((*s).plic_irq)
        .cast::<IRQSignal>()
        .add(idx as usize)
}

/* ------------------------------------------------------------------ */
/* riscv_cpu inline accessors (static inline in the header)            */
/* ------------------------------------------------------------------ */

unsafe fn cpu_class(s: *mut RISCVCPUState) -> *const RISCVCPUClass {
    (*(s as *const RISCVCPUCommonState)).class_ptr
}

unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_set_mip {
        f(s, mask);
    }
}

unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: u32) {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_reset_mip {
        f(s, mask);
    }
}

unsafe fn riscv_cpu_get_cycles(s: *mut RISCVCPUState) -> u64 {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_get_cycles {
        f(s)
    } else {
        0
    }
}

unsafe fn riscv_cpu_get_misa(s: *mut RISCVCPUState) -> u32 {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_get_misa {
        f(s)
    } else {
        0
    }
}

unsafe fn riscv_cpu_flush_tlb_write_range_ram(
    s: *mut RISCVCPUState,
    ram_ptr: *mut u8,
    ram_size: usize,
) {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_flush_tlb_write_range_ram {
        f(s, ram_ptr, ram_size);
    }
}

/* ------------------------------------------------------------------ */
/* RTC                                                                 */
/* ------------------------------------------------------------------ */

unsafe fn rtc_get_real_time(_s: *mut RISCVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 /* CLOCK_MONOTONIC */, &mut ts);
    (ts.tv_sec as u64)
        .wrapping_mul(RTC_FREQ)
        .wrapping_add(ts.tv_nsec as u64 / (1_000_000_000 / RTC_FREQ))
}

unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    if (*m).rtc_real_time != 0 {
        rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time)
    } else {
        riscv_cpu_get_cycles((*m).cpu_state) / RTC_FREQ_DIV
    }
}

/* ------------------------------------------------------------------ */
/* HTIF device                                                         */
/* ------------------------------------------------------------------ */

unsafe fn htif_handle_cmd(s: *mut RISCVMachine) {
    let device: u32 = ((*s).htif_tohost >> 56) as u32;
    let cmd: u32 = (((*s).htif_tohost >> 48) & 0xff) as u32;

    if (*s).htif_tohost == 1 {
        printf(cs(b"\nPower off.\n\0"));
        exit(0);
    } else if device == 1 && cmd == 1 {
        let mut buf = [0u8; 1];
        buf[0] = ((*s).htif_tohost & 0xff) as u8;
        let console = (*s).common.console;
        if let Some(f) = (*console).write_data {
            f((*console).opaque, buf.as_ptr(), 1);
        }
        (*s).htif_tohost = 0;
        (*s).htif_fromhost = ((device as u64) << 56) | ((cmd as u64) << 48);
    } else if device == 1 && cmd == 0 {
        (*s).htif_tohost = 0;
    } else {
        printf(
            cs(b"HTIF: unsupported tohost=0x%016lx\n\0"),
            (*s).htif_tohost as c_ulong,
        );
    }
}

unsafe extern "C" fn htif_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0 => (*s).htif_tohost as u32,
        4 => ((*s).htif_tohost >> 32) as u32,
        8 => (*s).htif_fromhost as u32,
        12 => ((*s).htif_fromhost >> 32) as u32,
        _ => 0,
    }
}

unsafe extern "C" fn htif_write(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0 => {
            (*s).htif_tohost = ((*s).htif_tohost & !0xffff_ffffu64) | (val as u64);
        }
        4 => {
            (*s).htif_tohost = ((*s).htif_tohost & 0xffff_ffffu64) | ((val as u64) << 32);
            htif_handle_cmd(s);
        }
        8 => {
            (*s).htif_fromhost = ((*s).htif_fromhost & !0xffff_ffffu64) | (val as u64);
        }
        12 => {
            (*s).htif_fromhost =
                ((*s).htif_fromhost & 0xffff_ffffu64) | ((val as u64) << 32);
        }
        _ => {}
    }
}

/* ------------------------------------------------------------------ */
/* CLINT                                                               */
/* ------------------------------------------------------------------ */

unsafe extern "C" fn clint_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    let m = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0xbff8 => rtc_get_time(m) as u32,
        0xbffc => (rtc_get_time(m) >> 32) as u32,
        0x4000 => (*m).timecmp as u32,
        0x4004 => ((*m).timecmp >> 32) as u32,
        _ => 0,
    }
}

unsafe extern "C" fn clint_write(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int) {
    let m = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0x4000 => {
            (*m).timecmp = ((*m).timecmp & !0xffff_ffffu64) | (val as u64);
            riscv_cpu_reset_mip((*m).cpu_state, MIP_MTIP);
        }
        0x4004 => {
            (*m).timecmp = ((*m).timecmp & 0xffff_ffffu64) | ((val as u64) << 32);
            riscv_cpu_reset_mip((*m).cpu_state, MIP_MTIP);
        }
        _ => {}
    }
}

/* ------------------------------------------------------------------ */
/* PLIC                                                                */
/* ------------------------------------------------------------------ */

unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask: u32 = (*s).plic_pending_irq & !(*s).plic_served_irq;
    if mask != 0 {
        riscv_cpu_set_mip(cpu, MIP_MEIP | MIP_SEIP);
    } else {
        riscv_cpu_reset_mip(cpu, MIP_MEIP | MIP_SEIP);
    }
}

unsafe extern "C" fn plic_read(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32 {
    let s = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0x200000 => 0,
        0x200004 => {
            let mask: u32 = (*s).plic_pending_irq & !(*s).plic_served_irq;
            if mask != 0 {
                let i: c_int = if mask != 0 {
                    mask.trailing_zeros() as c_int
                } else {
                    32
                };
                (*s).plic_served_irq |= (1u32) << (i as u32);
                plic_update_mip(s);
                (i + 1) as u32
            } else {
                0
            }
        }
        _ => 0,
    }
}

unsafe extern "C" fn plic_write(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int) {
    let s = opaque as *mut RISCVMachine;
    if size_log2 != 2 {
        abort_now();
    }
    match offset {
        0x200004 => {
            let val = val.wrapping_sub(1);
            if val < 32 {
                (*s).plic_served_irq &= !((1u32) << val);
                plic_update_mip(s);
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn plic_set_irq(opaque: *mut c_void, irq_num: c_int, state: c_int) {
    let s = opaque as *mut RISCVMachine;
    let mask: u32 = (1u32).wrapping_shl((irq_num.wrapping_sub(1)) as u32);
    if state != 0 {
        (*s).plic_pending_irq |= mask;
    } else {
        (*s).plic_pending_irq &= !mask;
    }
    plic_update_mip(s);
}

/* ------------------------------------------------------------------ */
/* memory helpers                                                      */
/* ------------------------------------------------------------------ */

unsafe fn get_ram_ptr(s: *mut RISCVMachine, paddr: u64, is_rw: c_int) -> *mut u8 {
    phys_mem_get_ram_ptr((*s).mem_map, paddr, is_rw)
}

unsafe extern "C" fn riscv_flush_tlb_write_range(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut RISCVMachine;
    riscv_cpu_flush_tlb_write_range_ram((*s).cpu_state, ram_addr, ram_size);
}

/* ------------------------------------------------------------------ */
/* FDT builder                                                         */
/* ------------------------------------------------------------------ */

unsafe fn fdt_init() -> *mut FDTState {
    mallocz(core::mem::size_of::<FDTState>()) as *mut FDTState
}

unsafe fn fdt_alloc_len(s: *mut FDTState, len: c_int) {
    if len > (*s).tab_size {
        let new_size = max_int(len, (*s).tab_size.wrapping_mul(3) / 2);
        (*s).tab = realloc(
            (*s).tab as *mut c_void,
            (new_size as usize).wrapping_mul(core::mem::size_of::<u32>()),
        ) as *mut u32;
        (*s).tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: *mut FDTState, v: c_int) {
    fdt_alloc_len(s, (*s).tab_len + 1);
    let idx = (*s).tab_len as usize;
    *(*s).tab.add(idx) = cpu_to_be32(v as u32);
    (*s).tab_len = (*s).tab_len + 1;
}

unsafe fn fdt_put_data(s: *mut FDTState, data: *const u8, len: c_int) {
    let len1: c_int = (len + 3) / 4;
    fdt_alloc_len(s, (*s).tab_len + len1);
    let dst = (*s).tab.add((*s).tab_len as usize) as *mut u8;
    if len > 0 && !data.is_null() {
        ptr::copy_nonoverlapping(data, dst, len as usize);
    }
    let pad = ((len as i32).wrapping_neg() & 3) as usize;
    if pad > 0 {
        ptr::write_bytes(dst.add(len as usize), 0, pad);
    }
    (*s).tab_len = (*s).tab_len + len1;
}

unsafe fn fdt_begin_node(s: *mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const u8, strlen(name) as c_int + 1);
    (*s).open_node_count += 1;
}

unsafe fn fdt_begin_node_num(s: *mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0 as c_char; 256];
    snprintf(buf.as_mut_ptr(), 256, cs(b"%s@%lx\0"), name, n as c_ulong);
    fdt_begin_node(s, buf.as_ptr());
}

unsafe fn fdt_end_node(s: *mut FDTState) {
    fdt_put32(s, 2);
    (*s).open_node_count -= 1;
}

unsafe fn fdt_get_string_offset(s: *mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < (*s).string_table_len {
        if strcmp((*s).string_table.add(pos as usize), name) == 0 {
            return pos;
        }
        pos += strlen((*s).string_table.add(pos as usize)) as c_int + 1;
    }

    let name_size: c_int = strlen(name) as c_int + 1;
    let new_len: c_int = (*s).string_table_len + name_size;
    if new_len > (*s).string_table_size {
        let new_size = max_int(new_len, (*s).string_table_size.wrapping_mul(3) / 2);
        (*s).string_table = realloc((*s).string_table as *mut c_void, new_size as usize) as *mut c_char;
        (*s).string_table_size = new_size;
    }
    pos = (*s).string_table_len;
    if name_size > 0 {
        ptr::copy_nonoverlapping(
            name as *const u8,
            (*s).string_table.add(pos as usize) as *mut u8,
            name_size as usize,
        );
    }
    (*s).string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(
    s: *mut FDTState,
    prop_name: *const c_char,
    data: *const u8,
    data_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    fdt_put_data(s, data, data_len);
}

unsafe fn fdt_prop_tab_u32(
    s: *mut FDTState,
    prop_name: *const c_char,
    tab: *const u32,
    tab_len: c_int,
) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(core::mem::size_of::<u32>() as c_int));
    fdt_put32(s, fdt_get_string_offset(s, prop_name));
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.add(i as usize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_u32(s: *mut FDTState, prop_name: *const c_char, val: u32) {
    let v = val;
    fdt_prop_tab_u32(s, prop_name, &v as *const u32, 1);
}

unsafe fn fdt_prop_tab_u64(s: *mut FDTState, prop_name: *const c_char, v0: u64) {
    let mut tab = [0u32; 2];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_ptr(), 2);
}

unsafe fn fdt_prop_tab_u64_2(s: *mut FDTState, prop_name: *const c_char, v0: u64, v1: u64) {
    let mut tab = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_ptr(), 4);
}

unsafe fn fdt_prop_str(s: *mut FDTState, prop_name: *const c_char, str_: *const c_char) {
    fdt_prop(
        s,
        prop_name,
        str_ as *const u8,
        strlen(str_) as c_int + 1,
    );
}

unsafe fn fdt_output(s: *mut FDTState, dst: *mut u8) -> c_int {
    if (*s).open_node_count != 0 {
        abort_now();
    }

    fdt_put32(s, 9);

    let dt_struct_size: c_int = (*s).tab_len.wrapping_mul(core::mem::size_of::<u32>() as c_int);
    let dt_strings_size: c_int = (*s).string_table_len;

    let h = dst as *mut FdtHeader;
    (*h).magic = cpu_to_be32(FDT_MAGIC);
    (*h).version = cpu_to_be32(FDT_VERSION);
    (*h).last_comp_version = cpu_to_be32(16);
    (*h).boot_cpuid_phys = cpu_to_be32(0);
    (*h).size_dt_strings = cpu_to_be32(dt_strings_size as u32);
    (*h).size_dt_struct = cpu_to_be32(dt_struct_size as u32);

    let mut pos: c_int = core::mem::size_of::<FdtHeader>() as c_int;

    (*h).off_dt_struct = cpu_to_be32(pos as u32);
    if dt_struct_size > 0 {
        ptr::copy_nonoverlapping(
            (*s).tab as *const u8,
            dst.add(pos as usize),
            dt_struct_size as usize,
        );
    }
    pos += dt_struct_size;

    while (pos & 7) != 0 {
        *dst.add(pos as usize) = 0;
        pos += 1;
    }
    (*h).off_mem_rsvmap = cpu_to_be32(pos as u32);
    let re = dst.add(pos as usize) as *mut FdtReserveEntry;
    (*re).address = 0;
    (*re).size = 0;
    pos += core::mem::size_of::<FdtReserveEntry>() as c_int;

    (*h).off_dt_strings = cpu_to_be32(pos as u32);
    if dt_strings_size > 0 {
        ptr::copy_nonoverlapping(
            (*s).string_table as *const u8,
            dst.add(pos as usize),
            dt_strings_size as usize,
        );
    }
    pos += dt_strings_size;

    while (pos & 7) != 0 {
        *dst.add(pos as usize) = 0;
        pos += 1;
    }

    (*h).totalsize = cpu_to_be32(pos as u32);
    pos
}

unsafe fn fdt_end(s: *mut FDTState) {
    free((*s).tab as *mut c_void);
    free((*s).string_table as *mut c_void);
    free(s as *mut c_void);
}

unsafe fn riscv_build_fdt(
    m: *mut RISCVMachine,
    dst: *mut u8,
    kernel_start: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_start: u64,
    initrd_size: u64,
) -> c_int {
    let s = fdt_init();
    let mut cur_phandle: c_int = 1;

    fdt_begin_node(s, cs(b"\0"));
    fdt_prop_u32(s, cs(b"#address-cells\0"), 2);
    fdt_prop_u32(s, cs(b"#size-cells\0"), 2);
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"ucbbar,riscvemu-bar_dev\0"));
    fdt_prop_str(s, cs(b"model\0"), cs(b"ucbbar,riscvemu-bare\0"));

    fdt_begin_node(s, cs(b"cpus\0"));
    fdt_prop_u32(s, cs(b"#address-cells\0"), 1);
    fdt_prop_u32(s, cs(b"#size-cells\0"), 0);
    fdt_prop_u32(s, cs(b"timebase-frequency\0"), RTC_FREQ as u32);

    fdt_begin_node_num(s, cs(b"cpu\0"), 0);
    fdt_prop_str(s, cs(b"device_type\0"), cs(b"cpu\0"));
    fdt_prop_u32(s, cs(b"reg\0"), 0);
    fdt_prop_str(s, cs(b"status\0"), cs(b"okay\0"));
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"riscv\0"));

    let max_xlen = (*m).max_xlen;
    let misa = riscv_cpu_get_misa((*m).cpu_state);
    let mut isa_string = [0 as c_char; 128];
    let mut q: *mut c_char = isa_string.as_mut_ptr();
    let n = snprintf(isa_string.as_mut_ptr(), 128, cs(b"rv%d\0"), max_xlen);
    q = q.offset(n as isize);
    let mut i: c_int = 0;
    while i < 26 {
        if (misa & ((1u32) << (i as u32))) != 0 {
            *q = (b'a' as c_int + i) as c_char;
            q = q.add(1);
        }
        i += 1;
    }
    *q = 0;
    fdt_prop_str(s, cs(b"riscv,isa\0"), isa_string.as_ptr());

    fdt_prop_str(
        s,
        cs(b"mmu-type\0"),
        if max_xlen <= 32 {
            cs(b"riscv,sv32\0")
        } else {
            cs(b"riscv,sv48\0")
        },
    );
    fdt_prop_u32(s, cs(b"clock-frequency\0"), 2_000_000_000);

    fdt_begin_node(s, cs(b"interrupt-controller\0"));
    fdt_prop_u32(s, cs(b"#interrupt-cells\0"), 1);
    fdt_prop(s, cs(b"interrupt-controller\0"), ptr::null(), 0);
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"riscv,cpu-intc\0"));
    let intc_phandle = cur_phandle;
    cur_phandle += 1;
    fdt_prop_u32(s, cs(b"phandle\0"), intc_phandle as u32);
    fdt_end_node(s);

    fdt_end_node(s);

    fdt_end_node(s);

    fdt_begin_node_num(s, cs(b"memory\0"), RAM_BASE_ADDR);
    fdt_prop_str(s, cs(b"device_type\0"), cs(b"memory\0"));
    let mut tab = [0u32; 4];
    tab[0] = (RAM_BASE_ADDR >> 32) as u32;
    tab[1] = RAM_BASE_ADDR as u32;
    tab[2] = ((*m).ram_size >> 32) as u32;
    tab[3] = (*m).ram_size as u32;
    fdt_prop_tab_u32(s, cs(b"reg\0"), tab.as_ptr(), 4);

    fdt_end_node(s);

    fdt_begin_node(s, cs(b"htif\0"));
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"ucb,htif0\0"));
    fdt_end_node(s);

    fdt_begin_node(s, cs(b"soc\0"));
    fdt_prop_u32(s, cs(b"#address-cells\0"), 2);
    fdt_prop_u32(s, cs(b"#size-cells\0"), 2);
    fdt_prop(
        s,
        cs(b"compatible\0"),
        SOC_COMPAT.as_ptr(),
        SOC_COMPAT.len() as c_int,
    );
    fdt_prop(s, cs(b"ranges\0"), ptr::null(), 0);

    fdt_begin_node_num(s, cs(b"clint\0"), CLINT_BASE_ADDR);
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"riscv,clint0\0"));

    tab[0] = intc_phandle as u32;
    tab[1] = 3;
    tab[2] = intc_phandle as u32;
    tab[3] = 7;
    fdt_prop_tab_u32(s, cs(b"interrupts-extended\0"), tab.as_ptr(), 4);

    fdt_prop_tab_u64_2(s, cs(b"reg\0"), CLINT_BASE_ADDR, CLINT_SIZE);

    fdt_end_node(s);

    fdt_begin_node_num(s, cs(b"plic\0"), PLIC_BASE_ADDR);
    fdt_prop_u32(s, cs(b"#interrupt-cells\0"), 1);
    fdt_prop(s, cs(b"interrupt-controller\0"), ptr::null(), 0);
    fdt_prop_str(s, cs(b"compatible\0"), cs(b"riscv,plic0\0"));
    fdt_prop_u32(s, cs(b"riscv,ndev\0"), 31);
    fdt_prop_tab_u64_2(s, cs(b"reg\0"), PLIC_BASE_ADDR, PLIC_SIZE);

    tab[0] = intc_phandle as u32;
    tab[1] = 9;
    tab[2] = intc_phandle as u32;
    tab[3] = 11;
    fdt_prop_tab_u32(s, cs(b"interrupts-extended\0"), tab.as_ptr(), 4);

    let plic_phandle = cur_phandle;
    cur_phandle += 1;
    fdt_prop_u32(s, cs(b"phandle\0"), plic_phandle as u32);

    fdt_end_node(s);

    let mut i: c_int = 0;
    while i < (*m).virtio_count {
        let addr = VIRTIO_BASE_ADDR.wrapping_add((i as u64).wrapping_mul(VIRTIO_SIZE));
        fdt_begin_node_num(s, cs(b"virtio\0"), addr);
        fdt_prop_str(s, cs(b"compatible\0"), cs(b"virtio,mmio\0"));
        fdt_prop_tab_u64_2(s, cs(b"reg\0"), addr, VIRTIO_SIZE);
        tab[0] = plic_phandle as u32;
        tab[1] = (VIRTIO_IRQ as u32).wrapping_add(i as u32);
        fdt_prop_tab_u32(s, cs(b"interrupts-extended\0"), tab.as_ptr(), 2);
        fdt_end_node(s);
        i += 1;
    }

    let fb_dev = (*m).common.fb_dev;
    if !fb_dev.is_null() {
        fdt_begin_node_num(s, cs(b"framebuffer\0"), FRAMEBUFFER_BASE_ADDR);
        fdt_prop_str(s, cs(b"compatible\0"), cs(b"simple-framebuffer\0"));
        fdt_prop_tab_u64_2(
            s,
            cs(b"reg\0"),
            FRAMEBUFFER_BASE_ADDR,
            (*fb_dev).fb_size as u64,
        );
        fdt_prop_u32(s, cs(b"width\0"), (*fb_dev).width as u32);
        fdt_prop_u32(s, cs(b"height\0"), (*fb_dev).height as u32);
        fdt_prop_u32(s, cs(b"stride\0"), (*fb_dev).stride as u32);
        fdt_prop_str(s, cs(b"format\0"), cs(b"a8r8g8b8\0"));
        fdt_end_node(s);
    }

    fdt_end_node(s);

    fdt_begin_node(s, cs(b"chosen\0"));
    fdt_prop_str(
        s,
        cs(b"bootargs\0"),
        if cmd_line.is_null() { cs(b"\0") } else { cmd_line },
    );
    if kernel_size > 0 {
        fdt_prop_tab_u64(s, cs(b"riscv,kernel-start\0"), kernel_start);
        fdt_prop_tab_u64(
            s,
            cs(b"riscv,kernel-end\0"),
            kernel_start.wrapping_add(kernel_size),
        );
    }
    if initrd_size > 0 {
        fdt_prop_tab_u64(s, cs(b"linux,initrd-start\0"), initrd_start);
        fdt_prop_tab_u64(
            s,
            cs(b"linux,initrd-end\0"),
            initrd_start.wrapping_add(initrd_size),
        );
    }

    fdt_end_node(s);

    fdt_end_node(s);

    let size = fdt_output(s, dst);
    fdt_end(s);
    size
}

/* ------------------------------------------------------------------ */
/* copy_bios                                                           */
/* ------------------------------------------------------------------ */

unsafe fn copy_bios(
    s: *mut RISCVMachine,
    buf: *const u8,
    buf_len: c_int,
    kernel_buf: *const u8,
    kernel_buf_len: c_int,
    cmd_line: *const c_char,
    initrd_buf: *const u8,
    initrd_buf_len: c_int,
) {
    let fdt_addr: u64;
    let kernel_align: u64;
    let mut kernel_base: u64;
    let mut kernel_size: u64;
    let mut bios_base: u64 = 0;
    let mut bios_size: u64 = 0;
    let mut initrd_base: u64 = 0;
    let mut initrd_size: u64 = 0;
    let mut image_start: u64 = 0;
    let mut image_len: u64 = 0;
    let mut ram_ptr: *mut u8;

    ram_ptr = get_ram_ptr(s, RAM_BASE_ADDR, 1);

    if elf_detect_magic(buf, buf_len) != 0 {
        if elf_load(
            buf,
            buf_len,
            ram_ptr,
            (*s).ram_size as c_int,
            &mut image_start,
            &mut image_len,
        ) == -1
        {
            vm_error(cs(b"Failed to load ELF BIOS\n\0"));
            exit(1);
        }
        bios_base = image_start.wrapping_sub(RAM_BASE_ADDR);
        bios_size = image_len;
    } else {
        if (buf_len as u64) > (*s).ram_size {
            vm_error(cs(b"BIOS too big\n\0"));
            exit(1);
        }
        if buf_len > 0 && !buf.is_null() {
            ptr::copy_nonoverlapping(buf, ram_ptr, buf_len as usize);
        }
        bios_base = RAM_BASE_ADDR;
        bios_size = buf_len as u64;
    }

    if kernel_buf_len > 0 {
        if (*s).max_xlen == 32 {
            kernel_align = 4 << 20;
        } else {
            kernel_align = 2 << 20;
        }
        kernel_base = (buf_len as u64)
            .wrapping_add(kernel_align)
            .wrapping_sub(1)
            & !kernel_align.wrapping_sub(1);
        if elf_detect_magic(kernel_buf, kernel_buf_len) != 0 {
            if elf_load(
                kernel_buf,
                kernel_buf_len,
                ram_ptr.add(kernel_base as usize),
                (*s).ram_size.wrapping_sub(kernel_base) as c_int,
                &mut image_start,
                &mut image_len,
            ) == -1
            {
                vm_error(cs(b"Failed to load ELF kernel\n\0"));
                exit(1);
            }
            kernel_base = kernel_base.wrapping_add(image_start);
            kernel_size = image_len;
        } else {
            if kernel_buf_len > 0 && !kernel_buf.is_null() {
                ptr::copy_nonoverlapping(
                    kernel_buf,
                    ram_ptr.add(kernel_base as usize),
                    kernel_buf_len as usize,
                );
            }
            kernel_size = kernel_buf_len as u64;
        }
    } else {
        kernel_base = 0;
        kernel_size = 0;
    }

    if initrd_buf_len > 0 {
        if kernel_size > 0 {
            initrd_base = kernel_base.wrapping_add(kernel_size);
        } else {
            initrd_base = bios_base.wrapping_add(bios_size);
        }
        initrd_base &= !DEVRAM_PAGE_SIZE.wrapping_sub(1);
        initrd_base = initrd_base.wrapping_add(DEVRAM_PAGE_SIZE);
        if initrd_base.wrapping_add(initrd_buf_len as u64) > (*s).ram_size {
            vm_error(cs(b"initrd too big\n\0"));
            exit(1);
        }

        if compress_detect_magic(initrd_buf, initrd_buf_len) != 0 {
            let res = decompress(
                initrd_buf,
                initrd_buf_len,
                ram_ptr.add(initrd_base as usize),
                (*s).ram_size.wrapping_sub(kernel_base) as c_int,
            );
            if res == -1 {
                vm_error(cs(b"Failed to decompress initramfs image\n\0"));
                exit(1);
            }
            initrd_size = res as u64;
        } else {
            if initrd_buf_len > 0 && !initrd_buf.is_null() {
                ptr::copy_nonoverlapping(
                    initrd_buf,
                    ram_ptr.add(initrd_base as usize),
                    initrd_buf_len as usize,
                );
            }
            initrd_size = initrd_buf_len as u64;
        }
    } else {
        initrd_base = 0;
        initrd_size = 0;
    }

    ram_ptr = get_ram_ptr(s, 0, 1);

    fdt_addr = 0x1000 + 8 * 8;

    riscv_build_fdt(
        s,
        ram_ptr.add(fdt_addr as usize),
        RAM_BASE_ADDR.wrapping_add(kernel_base),
        kernel_size,
        cmd_line,
        RAM_BASE_ADDR.wrapping_add(initrd_base),
        initrd_size,
    );

    let q = ram_ptr.add(0x1000) as *mut u32;
    *q.add(0) = (0x297u32).wrapping_add(0x8000_0000).wrapping_sub(0x1000);
    *q.add(1) = 0x597;
    *q.add(2) = (0x58593u64).wrapping_add(fdt_addr.wrapping_sub(4) << 20) as u32;
    *q.add(3) = 0xf140_2573;
    *q.add(4) = 0x0002_8067;
}

/* ------------------------------------------------------------------ */
/* the target                                                          */
/* ------------------------------------------------------------------ */

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init(p: *const VirtMachineParams) -> *mut VirtMachine {
    let mut max_xlen: c_int = 0;
    let machine_name: *const c_char = (*p).machine_name;

    if strcmp(machine_name, cs(b"riscv32\0")) == 0 {
        max_xlen = 32;
    } else if strcmp(machine_name, cs(b"riscv64\0")) == 0 {
        max_xlen = 64;
    } else if strcmp(machine_name, cs(b"riscv128\0")) == 0 {
        max_xlen = 128;
    } else {
        vm_error(cs(b"unsupported machine: %s\n\0"), machine_name);
        return ptr::null_mut();
    }

    let s = mallocz(core::mem::size_of::<RISCVMachine>()) as *mut RISCVMachine;
    (*s).common.vmc = (*p).vmc;
    (*s).ram_size = (*p).ram_size;
    (*s).max_xlen = max_xlen;
    (*s).mem_map = phys_mem_map_init();

    (*(*s).mem_map).opaque = s as *mut c_void;
    (*(*s).mem_map).flush_tlb_write_range = Some(riscv_flush_tlb_write_range);

    (*s).cpu_state = riscv_cpu_init((*s).mem_map, max_xlen);
    if (*s).cpu_state.is_null() {
        vm_error(cs(b"unsupported max_xlen=%d\n\0"), max_xlen);
        return ptr::null_mut();
    }

    let files = ptr::addr_of!((*p).files).cast::<VMFileEntry>();
    let bios = files.add(VM_FILE_BIOS);

    let mut htif_start: u64 = DEFAULT_HTIF_BASE_ADDR;
    if !(*bios).buf.is_null() {
        if elf_detect_magic((*bios).buf, (*bios).len) != 0 {
            let mut addr: u64 = 0;
            if elf_find_section((*bios).buf, cs(b".htif\0"), &mut addr, ptr::null_mut()) != 0 {
                htif_start = addr.wrapping_add(8);
            }
        }
    }
    cpu_register_device(
        (*s).mem_map,
        htif_start,
        16,
        s as *mut c_void,
        Some(htif_read),
        Some(htif_write),
        DEVIO_SIZE32,
    );

    let ram_flags: c_int = 0;
    if let Some(f) = (*(*s).mem_map).register_ram {
        f((*s).mem_map, RAM_BASE_ADDR, (*p).ram_size, ram_flags);
        f((*s).mem_map, 0x0000_0000, LOW_RAM_SIZE, 0);
    }
    (*s).rtc_real_time = (*p).rtc_real_time;
    if (*p).rtc_real_time != 0 {
        (*s).rtc_start_time = rtc_get_real_time(s);
    }

    cpu_register_device(
        (*s).mem_map,
        CLINT_BASE_ADDR,
        CLINT_SIZE,
        s as *mut c_void,
        Some(clint_read),
        Some(clint_write),
        DEVIO_SIZE32,
    );
    cpu_register_device(
        (*s).mem_map,
        PLIC_BASE_ADDR,
        PLIC_SIZE,
        s as *mut c_void,
        Some(plic_read),
        Some(plic_write),
        DEVIO_SIZE32,
    );

    let mut i: c_int = 1;
    while i < 32 {
        irq_init(
            plic_irq_ptr(s, i),
            Some(plic_set_irq),
            s as *mut c_void,
            i,
        );
        i += 1;
    }

    (*s).common.console = (*p).console;

    let mut vbus_s: VIRTIOBusDef = core::mem::zeroed();
    let vbus: *mut VIRTIOBusDef = &mut vbus_s;
    (*vbus).mem_map = (*s).mem_map;
    (*vbus).addr = VIRTIO_BASE_ADDR;
    let mut irq_num: c_int = VIRTIO_IRQ;

    /* virtio console */
    if !(*p).console.is_null() {
        (*vbus).irq = plic_irq_ptr(s, irq_num);
        (*s).common.console_dev = virtio_console_init(vbus, (*p).console);
        (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
        irq_num += 1;
        (*s).virtio_count += 1;
    }

    /* virtio net device */
    let tab_eth = ptr::addr_of!((*p).tab_eth).cast::<VMEthEntry>();
    i = 0;
    while i < (*p).eth_count {
        let eth = tab_eth.add(i as usize);
        (*vbus).irq = plic_irq_ptr(s, irq_num);
        virtio_net_init(vbus, (*eth).net);
        (*s).common.net = (*eth).net;
        (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
        irq_num += 1;
        (*s).virtio_count += 1;
        i += 1;
    }

    /* virtio block device */
    let tab_drive = ptr::addr_of!((*p).tab_drive).cast::<VMDriveEntry>();
    i = 0;
    while i < (*p).drive_count {
        let drive = tab_drive.add(i as usize);
        (*vbus).irq = plic_irq_ptr(s, irq_num);
        let blk_dev = virtio_block_init(vbus, (*drive).block_dev);
        let _ = blk_dev;
        (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
        irq_num += 1;
        (*s).virtio_count += 1;
        i += 1;
    }

    /* virtio filesystem */
    let tab_fs = ptr::addr_of!((*p).tab_fs).cast::<VMFSEntry>();
    i = 0;
    while i < (*p).fs_count {
        let fs = tab_fs.add(i as usize);
        (*vbus).irq = plic_irq_ptr(s, irq_num);
        let fs_dev = virtio_9p_init(vbus, (*fs).fs_dev, (*fs).tag);
        let _ = fs_dev;
        (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
        irq_num += 1;
        (*s).virtio_count += 1;
        i += 1;
    }

    if !(*p).display_device.is_null() {
        let fb_dev = mallocz(core::mem::size_of::<FBDevice>()) as *mut FBDevice;
        (*s).common.fb_dev = fb_dev;
        if strcmp((*p).display_device, cs(b"simplefb\0")) == 0 {
            simplefb_init(
                (*s).mem_map,
                FRAMEBUFFER_BASE_ADDR,
                fb_dev,
                (*p).width,
                (*p).height,
            );
        } else {
            vm_error(
                cs(b"unsupported display device: %s\n\0"),
                (*p).display_device,
            );
            exit(1);
        }
    }

    if !(*p).input_device.is_null() {
        if strcmp((*p).input_device, cs(b"virtio\0")) == 0 {
            (*vbus).irq = plic_irq_ptr(s, irq_num);
            (*s).keyboard_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_KEYBOARD);
            (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
            irq_num += 1;
            (*s).virtio_count += 1;

            (*vbus).irq = plic_irq_ptr(s, irq_num);
            (*s).mouse_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_TABLET);
            (*vbus).addr = (*vbus).addr.wrapping_add(VIRTIO_SIZE);
            irq_num += 1;
            (*s).virtio_count += 1;
        } else {
            vm_error(cs(b"unsupported input device: %s\n\0"), (*p).input_device);
            exit(1);
        }
    }

    if (*bios).buf.is_null() {
        vm_error(cs(b"No bios found\0"));
    }

    let kernel = files.add(VM_FILE_KERNEL);
    let initrd = files.add(VM_FILE_INITRD);
    copy_bios(
        s,
        (*bios).buf,
        (*bios).len,
        (*kernel).buf,
        (*kernel).len,
        (*p).cmdline,
        (*initrd).buf,
        (*initrd).len,
    );

    s as *mut VirtMachine
}
}

// ── `if (!strcmp(p->machine_name, "riscv32")) … else` · riscv_machine.c:887-896 · chunk 2066 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!strcmp(p->machine_name, "riscv32")) {
//             max_xlen = 32;
//         } else if (!strcmp(p->machine_name, "riscv64")) {
//             max_xlen = 64;
//         } else if (!strcmp(p->machine_name, "riscv128")) {
//             max_xlen = 128;
//         } else {
//             vm_error("unsupported machine: %s\n", p->machine_name);
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2066 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
}

#[inline]
unsafe fn streq(s: *const c_char, lit: &[u8]) -> bool {
    strcmp(s, lit.as_ptr() as *const c_char) == 0
}

// Selects the max_xlen for the requested machine name. Writes 1 to *ok on
// success (returns the max_xlen), or 0 on failure after reporting the error.
#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2066(
    machine_name: *const c_char,
    ok: *mut c_int,
) -> c_int {
    if streq(machine_name, b"riscv32\0") {
        *ok = 1;
        32
    } else if streq(machine_name, b"riscv64\0") {
        *ok = 1;
        64
    } else if streq(machine_name, b"riscv128\0") {
        *ok = 1;
        128
    } else {
        vm_error(
            b"unsupported machine: %s\n\0".as_ptr() as *const c_char,
            machine_name,
        );
        *ok = 0;
        0
    }
}
}

// ── `s = mallocz(sizeof(*s)); s->common.vmc = p->vmc; s->ram_size = p->ram_size; s->max_xlen = max_x…` · riscv_machine.c:898-907 · chunk 2067 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     s = mallocz(sizeof(*s));
//         s->common.vmc = p->vmc;
//         s->ram_size = p->ram_size;
//         s->max_xlen = max_xlen;
//         s->mem_map = phys_mem_map_init();
//     
//         s->mem_map->opaque = s;
//         s->mem_map->flush_tlb_write_range = riscv_flush_tlb_write_range;
//     
//         s->cpu_state = riscv_cpu_init(s->mem_map, max_xlen);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2067 {
// Port of riscv_machine_init lines 898-907: allocate the machine, set up the
// memory map and the initial CPU state.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VirtMachineClass {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
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
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn phys_mem_map_init() -> *mut PhysMemoryMap;
    fn riscv_cpu_init(mem_map: *mut PhysMemoryMap, max_xlen: c_int) -> *mut RISCVCPUState;
}

// Port of static inline riscv_cpu_flush_tlb_write_range_ram()
unsafe fn riscv_cpu_flush_tlb_write_range_ram(
    s: *mut RISCVCPUState,
    ram_ptr: *mut u8,
    ram_size: usize,
) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_flush_tlb_write_range_ram {
        f(s, ram_ptr, ram_size);
    }
}

// Port of (the then-still-static) riscv_flush_tlb_write_range:
//   static void riscv_flush_tlb_write_range(void *opaque, uint8_t *ram_addr,
//                                           size_t ram_size)
// It is only reachable through the function pointer stored in the mem map, so
// the logic lives here as a private Rust fn.
unsafe extern "C" fn riscv_flush_tlb_write_range(
    opaque: *mut c_void,
    ram_addr: *mut u8,
    ram_size: usize,
) {
    let s = opaque as *mut RISCVMachine;
    riscv_cpu_flush_tlb_write_range_ram((*s).cpu_state, ram_addr, ram_size);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2067(
    vmc: *const VirtMachineClass,
    ram_size: u64,
    max_xlen: c_int,
) -> *mut RISCVMachine {
    let s = mallocz(core::mem::size_of::<RISCVMachine>()) as *mut RISCVMachine;
    (*s).common.vmc = vmc;
    (*s).ram_size = ram_size;
    (*s).max_xlen = max_xlen;

    let mem_map = phys_mem_map_init();
    (*s).mem_map = mem_map;

    (*mem_map).opaque = s as *mut c_void;
    (*mem_map).flush_tlb_write_range = Some(riscv_flush_tlb_write_range);

    (*s).cpu_state = riscv_cpu_init(mem_map, max_xlen);
    s
}
}

// ── `if (!s->cpu_state)` · riscv_machine.c:908-912 · chunk 2068 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s->cpu_state) {
//             vm_error("unsupported max_xlen=%d\n", max_xlen);
//     
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2068 {
// Port of the cpu_state check inside riscv_machine_init.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

/// Returns 1 when the cpu_state is valid, 0 when it is NULL (after reporting
/// the error), so the caller can perform the `return NULL;`.
#[no_mangle]
pub extern "C" fn riscv_machine_init__part2068(
    cpu_state: *mut RISCVCPUState,
    max_xlen: c_int,
) -> c_int {
    if cpu_state.is_null() {
        unsafe {
            vm_error(
                b"unsupported max_xlen=%d\n\0".as_ptr() as *const c_char,
                max_xlen,
            );
        }
        return 0;
    }
    1
}
}

// ── `if (p->files[VM_FILE_BIOS].buf)` · riscv_machine.c:916-924 · chunk 2069 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->files[VM_FILE_BIOS].buf) {
//             const VMFileEntry *bios = &p->files[VM_FILE_BIOS];
//             if (elf_detect_magic(bios->buf, bios->len)) {
//                 uint64_t addr;
//                 if (elf_find_section(bios->buf, ".htif", &addr, 
//                                                                ((void *)0)
//                                                                    )) {
//                     htif_start = addr + 8;
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2069 {
// Ported piece of riscv_machine_init: locate the .htif section of the BIOS ELF
// and derive the HTIF base address from it.

use core::ffi::{c_char, c_int};

// Mirror of the C VMFileEntry (only the fields following `filename` are read,
// but the layout must match exactly).
#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

extern "C" {
    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_find_section(
        buf: *const u8,
        name: *const c_char,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
}

static HTIF_NAME: [u8; 6] = [b'.', b'h', b't', b'i', b'f', 0];

#[no_mangle]
pub extern "C" fn riscv_machine_init__part2069(
    bios: *const VMFileEntry,
    htif_start_in: u64,
) -> u64 {
    let mut htif_start = htif_start_in;
    unsafe {
        if bios.is_null() {
            return htif_start;
        }
        let buf = (*bios).buf as *const u8;
        if buf.is_null() {
            return htif_start;
        }
        let len = (*bios).len;
        if elf_detect_magic(buf, len) != 0 {
            let mut addr: u64 = 0;
            if elf_find_section(
                buf,
                HTIF_NAME.as_ptr() as *const c_char,
                &mut addr as *mut u64,
                core::ptr::null_mut(),
            ) != 0
            {
                htif_start = addr.wrapping_add(8);
            }
        }
    }
    htif_start
}
}

// ── `cpu_register_device(s->mem_map, htif_start, 16, s, htif_read, htif_write, (1 << 2)); ram_flags…` · riscv_machine.c:925-932 · chunk 2070 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     cpu_register_device(s->mem_map, htif_start, 16,
//                             s, htif_read, htif_write, (1 << 2));
//     
//     
//         ram_flags = 0;
//         cpu_register_ram(s->mem_map, 0x80000000, p->ram_size, ram_flags);
//         cpu_register_ram(s->mem_map, 0x00000000, 0x00010000, 0);
//         s->rtc_real_time = p->rtc_real_time;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2070 {
// Rust port of the htif device registration / RAM registration / rtc flag
// setup part of riscv_machine_init().

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

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
    pub register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, BOOL)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

// Truncated mirror of RISCVMachine: only the leading part needed here
// (VirtMachine common is 40 bytes on the target ABI).
#[repr(C)]
pub struct RISCVMachine {
    pub common: [u8; 40],
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
}

// Truncated mirror of VirtMachineParams: fields up to rtc_real_time.
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
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

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2070(
    s: *mut RISCVMachine,
    htif_start: u64,
    p: *const VirtMachineParams,
    ram_flags: *mut c_int,
    htif_read: Option<DeviceReadFunc>,
    htif_write: Option<DeviceWriteFunc>,
) {
    let map: *mut PhysMemoryMap = (*s).mem_map;

    cpu_register_device(
        map,
        htif_start,
        16,
        s as *mut c_void,
        htif_read,
        htif_write,
        1 << 2,
    );

    *ram_flags = 0;
    let flags: c_int = *ram_flags;

    // cpu_register_ram(s->mem_map, 0x80000000, p->ram_size, ram_flags)
    if let Some(register_ram) = (*map).register_ram {
        register_ram(map, 0x8000_0000u64, (*p).ram_size, flags);
    }

    // cpu_register_ram(s->mem_map, 0x00000000, 0x00010000, 0)
    if let Some(register_ram) = (*map).register_ram {
        register_ram(map, 0x0000_0000u64, 0x0001_0000u64, 0);
    }

    (*s).rtc_real_time = (*p).rtc_real_time;
}
}

// ── `if (p->rtc_real_time)` · riscv_machine.c:933-935 · chunk 2071 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->rtc_real_time) {
//             s->rtc_start_time = rtc_get_real_time(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2071 {
// Port of the RTC initialization piece of riscv_machine_init.
// rtc_get_real_time is a `static` C function, so its logic is inlined here.

use core::ffi::c_int;

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    // int clock_gettime(clockid_t clk_id, struct timespec *tp);
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn riscv_machine_init__part2071(
    rtc_real_time: c_int,
    rtc_start_time: *mut u64,
) {
    if rtc_real_time != 0 {
        let mut ts = Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_gettime(1 /* CLOCK_REALTIME */, &mut ts);
        }
        let val = (ts.tv_sec as u64)
            .wrapping_mul(10000000u64)
            .wrapping_add((ts.tv_nsec / (1000000000i64 / 10000000i64)) as u64);
        unsafe {
            *rtc_start_time = val;
        }
    }
}
}

// ── `cpu_register_device(s->mem_map, 0x02000000, 0x000c0000, s, clint_read, clint_write, (1 << 2));…` · riscv_machine.c:937-940 · chunk 2072 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     cpu_register_device(s->mem_map, 0x02000000, 0x000c0000, s,
//                             clint_read, clint_write, (1 << 2));
//         cpu_register_device(s->mem_map, 0x40100000, 0x00400000, s,
//                             plic_read, plic_write, (1 << 2));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2072 {
// Port of the CLINT/PLIC device registration piece of riscv_machine_init.
// The two static C callbacks (clint_read/clint_write, plic_read/plic_write)
// have no linker symbol, so their logic is re-implemented here.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _opaque: [u8; 0],
}

type CpuInitFn = unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState;
type CpuEndFn = unsafe extern "C" fn(*mut RISCVCPUState);
type CpuInterpFn = unsafe extern "C" fn(*mut RISCVCPUState, c_int);
type CpuGetCyclesFn = unsafe extern "C" fn(*mut RISCVCPUState) -> u64;
type CpuSetMipFn = unsafe extern "C" fn(*mut RISCVCPUState, u32);
type CpuGetMipFn = unsafe extern "C" fn(*mut RISCVCPUState) -> u32;
type CpuGetPowerDownFn = unsafe extern "C" fn(*mut RISCVCPUState) -> c_int;
type CpuGetMisaFn = unsafe extern "C" fn(*mut RISCVCPUState) -> u32;
type CpuFlushTlbFn = unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize);

#[repr(C)]
pub struct RISCVCPUClass {
    riscv_cpu_init: Option<CpuInitFn>,
    riscv_cpu_end: Option<CpuEndFn>,
    riscv_cpu_interp: Option<CpuInterpFn>,
    riscv_cpu_get_cycles: Option<CpuGetCyclesFn>,
    riscv_cpu_set_mip: Option<CpuSetMipFn>,
    riscv_cpu_reset_mip: Option<CpuSetMipFn>,
    riscv_cpu_get_mip: Option<CpuGetMipFn>,
    riscv_cpu_get_power_down: Option<CpuGetPowerDownFn>,
    riscv_cpu_get_misa: Option<CpuGetMisaFn>,
    riscv_cpu_flush_tlb_write_range_ram: Option<CpuFlushTlbFn>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut PhysMemoryMap,
    max_xlen: c_int,
    cpu_state: *mut RISCVCPUState,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
    plic_pending_irq: u32,
    plic_served_irq: u32,
    plic_irq: [IRQSignal; 32],
    htif_tohost: u64,
    htif_fromhost: u64,
    keyboard_dev: *mut c_void,
    mouse_dev: *mut c_void,
    virtio_count: c_int,
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

extern "C" {
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut c_void;
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;

unsafe fn riscv_cpu_get_cycles(s: *mut RISCVCPUState) -> u64 {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0,
    }
}

unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_set_mip {
        f(s, mask);
    }
}

unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_reset_mip {
        f(s, mask);
    }
}

unsafe fn rtc_get_real_time(_s: *mut RISCVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    ((ts.tv_sec as u64).wrapping_mul(10_000_000))
        .wrapping_add((ts.tv_nsec / (1_000_000_000 / 10_000_000)) as u64)
}

unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    if (*m).rtc_real_time != 0 {
        rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time)
    } else {
        riscv_cpu_get_cycles((*m).cpu_state) / 16
    }
}

unsafe extern "C" fn clint_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let m = opaque as *mut RISCVMachine;
    match offset {
        0xbff8 => rtc_get_time(m) as u32,
        0xbffc => (rtc_get_time(m) >> 32) as u32,
        0x4000 => (*m).timecmp as u32,
        0x4004 => ((*m).timecmp >> 32) as u32,
        _ => 0,
    }
}

unsafe extern "C" fn clint_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let m = opaque as *mut RISCVMachine;
    match offset {
        0x4000 => {
            (*m).timecmp = ((*m).timecmp & !0xffff_ffffu64) | (val as u64);
            riscv_cpu_reset_mip((*m).cpu_state, 1 << 7);
        }
        0x4004 => {
            (*m).timecmp = ((*m).timecmp & 0xffff_ffffu64) | ((val as u64) << 32);
            riscv_cpu_reset_mip((*m).cpu_state, 1 << 7);
        }
        _ => {}
    }
}

unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu = (*s).cpu_state;
    let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
    if mask != 0 {
        riscv_cpu_set_mip(cpu, (1 << 11) | (1 << 9));
    } else {
        riscv_cpu_reset_mip(cpu, (1 << 11) | (1 << 9));
    }
}

unsafe extern "C" fn plic_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut RISCVMachine;
    match offset {
        0x200000 => 0,
        0x200004 => {
            let mask = (*s).plic_pending_irq & !(*s).plic_served_irq;
            if mask != 0 {
                let i = mask.trailing_zeros();
                (*s).plic_served_irq |= 1u32 << i;
                plic_update_mip(s);
                i.wrapping_add(1)
            } else {
                0
            }
        }
        _ => 0,
    }
}

unsafe extern "C" fn plic_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut RISCVMachine;
    match offset {
        0x200004 => {
            let val = val.wrapping_sub(1);
            if val < 32 {
                (*s).plic_served_irq &= !(1u32 << val);
                plic_update_mip(s);
            }
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2072(s: *mut RISCVMachine) {
    let mem_map = (*s).mem_map;
    cpu_register_device(
        mem_map,
        0x02000000,
        0x000c0000,
        s as *mut c_void,
        Some(clint_read),
        Some(clint_write),
        1 << 2,
    );
    cpu_register_device(
        mem_map,
        0x40100000,
        0x00400000,
        s as *mut c_void,
        Some(plic_read),
        Some(plic_write),
        1 << 2,
    );
}
}

// ── `for (i = 1; i < 32; i++)` · riscv_machine.c:941-943 · chunk 2073 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 1; i < 32; i++) {
//             irq_init(&s->plic_irq[i], plic_set_irq, s, i);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2073 {
// Rust port of the IRQ initialization loop of riscv_machine_init.
// Everything reachable from the loop is reimplemented here because
// plic_set_irq / plic_update_mip are static C functions without symbols.

use core::ffi::{c_int, c_void};

pub type BOOL = c_int;

/// Opaque RISC-V CPU state (only ever handled through pointers).
pub struct RISCVCPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: *const c_void,
    pub riscv_cpu_end: *const c_void,
    pub riscv_cpu_interp: *const c_void,
    pub riscv_cpu_get_cycles: *const c_void,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: *const c_void,
    pub riscv_cpu_get_power_down: *const c_void,
    pub riscv_cpu_get_misa: *const c_void,
    pub riscv_cpu_flush_tlb_write_range_ram: *const c_void,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    pub fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
}

unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    let c: *const RISCVCPUClass = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_set_mip {
        f(s, mask);
    }
}

unsafe fn riscv_cpu_reset_mip(s: *mut RISCVCPUState, mask: u32) {
    let c: *const RISCVCPUClass = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(f) = (*c).riscv_cpu_reset_mip {
        f(s, mask);
    }
}

unsafe fn plic_update_mip(s: *mut RISCVMachine) {
    let cpu: *mut RISCVCPUState = (*s).cpu_state;
    let mask: u32 = (*s).plic_pending_irq & !(*s).plic_served_irq;
    if mask != 0 {
        riscv_cpu_set_mip(cpu, (1u32 << 11) | (1u32 << 9));
    } else {
        riscv_cpu_reset_mip(cpu, (1u32 << 11) | (1u32 << 9));
    }
}

unsafe extern "C" fn plic_set_irq(opaque: *mut c_void, irq_num: c_int, state: c_int) {
    let s: *mut RISCVMachine = opaque as *mut RISCVMachine;
    let mask: u32 = (1i32).wrapping_shl((irq_num.wrapping_sub(1)) as u32) as u32;
    if state != 0 {
        (*s).plic_pending_irq |= mask;
    } else {
        (*s).plic_pending_irq &= !mask;
    }
    plic_update_mip(s);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2073(s: *mut RISCVMachine) {
    let base: *mut IRQSignal = (*s).plic_irq.as_mut_ptr();
    let mut i: c_int = 1;
    while i < 32 {
        let irq: *mut IRQSignal = base.add(i as usize);
        irq_init(irq, Some(plic_set_irq), s as *mut c_void, i);
        i += 1;
    }
}
}

// ── `s->common.console = p->console; memset(vbus, 0, sizeof(*vbus)); vbus->mem_map = s->mem_map; vbu…` · riscv_machine.c:945-950 · chunk 2074 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->common.console = p->console;
//     
//         memset(vbus, 0, sizeof(*vbus));
//         vbus->mem_map = s->mem_map;
//         vbus->addr = 0x40010000;
//         irq_num = 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2074 {
// Ported piece of riscv_machine_init: set the console device and initialise
// the local VIRTIOBusDef (memset + mem_map + addr) and irq_num.

use core::ffi::{c_char, c_int, c_void};

// Opaque C structures: only pointers to them are handled here.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _private: [u8; 0],
}

// struct VirtMachine
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

// struct RISCVMachine: only the leading fields that are used.
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
}

// VIRTIOBusDef
#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut c_void,
}

// Leading fields of VirtMachineParams, up to `console`.
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
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2074(
    s: *mut RISCVMachine,
    p: *const VirtMachineParams,
    vbus: *mut VIRTIOBusDef,
    irq_num: *mut c_int,
) {
    // s->common.console = p->console;
    (*s).common.console = (*p).console;

    // memset(vbus, 0, sizeof(*vbus));
    let vb = &mut *vbus;
    vb.pci_bus = core::ptr::null_mut();
    vb.mem_map = core::ptr::null_mut();
    vb.addr = 0;
    vb.irq = core::ptr::null_mut();

    // vbus->mem_map = s->mem_map;
    vb.mem_map = (*s).mem_map;
    // vbus->addr = 0x40010000;
    vb.addr = 0x40010000;
    // irq_num = 1;
    *irq_num = 1;
}
}

// ── `if (p->console)` · riscv_machine.c:953-959 · chunk 2075 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->console) {
//             vbus->irq = &s->plic_irq[irq_num];
//             s->common.console_dev = virtio_console_init(vbus, p->console);
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2075 {
// Rust port of the console virtio setup part of riscv_machine_init.
use core::ffi::{c_char, c_int, c_void};

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

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
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut c_void,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

extern "C" {
    fn virtio_console_init(bus: *mut VIRTIOBusDef, cs: *mut c_void) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2075(
    p: *const VirtMachineParams,
    s: *mut RISCVMachine,
    vbus: *mut VIRTIOBusDef,
    irq_num: *mut c_int,
) {
    if (*p).console.is_null() {
        return;
    }
    let irq = (*irq_num) as usize;
    (*vbus).irq = (*s).plic_irq.as_mut_ptr().add(irq);
    (*s).common.console_dev = virtio_console_init(vbus, (*p).console);
    (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
    *irq_num = (*irq_num).wrapping_add(1);
    (*s).virtio_count = (*s).virtio_count.wrapping_add(1);
}
}

// ── `for (i = 0; i < p->eth_count; i++)` · riscv_machine.c:962-969 · chunk 2076 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->eth_count; i++) {
//             vbus->irq = &s->plic_irq[irq_num];
//             virtio_net_init(vbus, p->tab_eth[i].net);
//             s->common.net = p->tab_eth[i].net;
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2076 {
// Port of the ethernet enumeration loop from riscv_machine_init.

use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct SetIRQFuncPtr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut SetIRQFuncPtr,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut PhysMemoryMap,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BlockDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FSDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: *mut c_void,
    pub read_data: *mut c_void,
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
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_local_time: BOOL,
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
    pub accel_enable: BOOL,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

extern "C" {
    fn virtio_net_init(bus: *mut VIRTIOBusDef, es: *mut EthernetDevice) -> *mut VIRTIODevice;
}

#[no_mangle]
pub extern "C" fn riscv_machine_init__part2076(
    p: *const VirtMachineParams,
    s: *mut RISCVMachine,
    vbus: *mut VIRTIOBusDef,
    irq_num: *mut c_int,
) {
    unsafe {
        let eth_count: c_int = (*p).eth_count;
        let mut i: c_int = 0;
        while i < eth_count {
            let entry: *const VMEthEntry = (*p).tab_eth.as_ptr().add(i as usize);
            let irq: *mut IRQSignal = (*s).plic_irq.as_mut_ptr().add((*irq_num) as usize);
            (*vbus).irq = irq;
            virtio_net_init(vbus, (*entry).net);
            (*s).common.net = (*entry).net;
            (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
            *irq_num = (*irq_num).wrapping_add(1);
            (*s).virtio_count = (*s).virtio_count.wrapping_add(1);
            i += 1;
        }
    }
}
}

// ── `for (i = 0; i < p->drive_count; i++)` · riscv_machine.c:972-979 · chunk 2077 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->drive_count; i++) {
//             vbus->irq = &s->plic_irq[irq_num];
//             blk_dev = virtio_block_init(vbus, p->tab_drive[i].block_dev);
//             (void)blk_dev;
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2077 {
// Ported piece of riscv_machine_init: the virtio block device registration loop.
use core::ffi::{c_char, c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut c_void,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

extern "C" {
    fn virtio_block_init(bus: *mut VIRTIOBusDef, bs: *mut c_void) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2077(
    s: *mut RISCVMachine,
    vbus: *mut VIRTIOBusDef,
    drive_count: c_int,
    tab_drive: *mut VMDriveEntry,
    irq_num: *mut c_int,
) {
    let mut i: c_int = 0;
    while i < drive_count {
        let base = (*s).plic_irq.as_mut_ptr();
        (*vbus).irq = base.add(*irq_num as usize);
        let blk_dev: *mut c_void =
            virtio_block_init(vbus, (*tab_drive.add(i as usize)).block_dev);
        let _ = blk_dev;
        (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
        *irq_num = (*irq_num).wrapping_add(1);
        (*s).virtio_count = (*s).virtio_count.wrapping_add(1);
        i += 1;
    }
}
}

// ── `for (i = 0; i < p->fs_count; i++)` · riscv_machine.c:982-992 · chunk 2078 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->fs_count; i++) {
//             VIRTIODevice *fs_dev;
//             vbus->irq = &s->plic_irq[irq_num];
//             fs_dev = virtio_9p_init(vbus, p->tab_fs[i].fs_dev,
//                                     p->tab_fs[i].tag);
//             (void)fs_dev;
//     
//             vbus->addr += 0x1000;
//             irq_num++;
//             s->virtio_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2078 {
// Rust port of the fs (virtio-9p) device registration loop of
// riscv_machine_init.

use core::ffi::{c_char, c_int, c_void};

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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct VIRTIOBusDef {
    pub pci_bus: *mut c_void,
    pub mem_map: *mut c_void,
    pub addr: u64,
    pub irq: *mut IRQSignal,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

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
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn virtio_9p_init(bus: *mut VIRTIOBusDef, fs: *mut c_void,
                      mount_tag: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn riscv_machine_init__part2078(s: *mut RISCVMachine,
                                               p: *const VirtMachineParams,
                                               vbus: *mut VIRTIOBusDef,
                                               irq_num: *mut c_int) {
    unsafe {
        let mut i: c_int = 0;
        let fs_count: c_int = (*p).fs_count;
        while i < fs_count {
            let irq: *mut IRQSignal =
                (*s).plic_irq.as_mut_ptr().wrapping_offset(*irq_num as isize);
            (*vbus).irq = irq;
            let entry: *const VMFSEntry =
                (*p).tab_fs.as_ptr().wrapping_offset(i as isize);
            let fs_dev: *mut c_void = virtio_9p_init(vbus, (*entry).fs_dev,
                                                     (*entry).tag);
            let _ = fs_dev;

            (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
            *irq_num += 1;
            (*s).virtio_count += 1;
            i += 1;
        }
    }
}
}

// ── `if (p->display_device)` · riscv_machine.c:994-1008 · chunk 2079 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->display_device) {
//             FBDevice *fb_dev;
//             fb_dev = mallocz(sizeof(*fb_dev));
//             s->common.fb_dev = fb_dev;
//             if (!strcmp(p->display_device, "simplefb")) {
//                 simplefb_init(s->mem_map,
//                               0x41000000,
//                               fb_dev,
//                               p->width, p->height);
//     
//             } else {
//                 vm_error("unsupported display device: %s\n", p->display_device);
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2079 {
#![allow(non_snake_case)]
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<
        unsafe extern "C" fn(fb_dev: *mut FBDevice, redraw_func: *mut c_void, opaque: *mut c_void),
    >,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn simplefb_init(
        map: *mut c_void,
        phys_addr: u64,
        fb_dev: *mut FBDevice,
        width: c_int,
        height: c_int,
    ) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(code: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2079(
    display_device: *const c_char,
    width: c_int,
    height: c_int,
    mem_map: *mut c_void,
    fb_dev_out: *mut *mut FBDevice,
) {
    if !display_device.is_null() {
        let fb_dev = mallocz(core::mem::size_of::<FBDevice>()) as *mut FBDevice;
        if !fb_dev_out.is_null() {
            *fb_dev_out = fb_dev;
        }
        if strcmp(display_device, b"simplefb\0".as_ptr() as *const c_char) == 0 {
            simplefb_init(mem_map, 0x41000000u64, fb_dev, width, height);
        } else {
            vm_error(
                b"unsupported display device: %s\n\0".as_ptr() as *const c_char,
                display_device,
            );
            exit(1);
        }
    }
}
}

// ── `if (p->input_device)` · riscv_machine.c:1010-1029 · chunk 2080 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->input_device) {
//             if (!strcmp(p->input_device, "virtio")) {
//                 vbus->irq = &s->plic_irq[irq_num];
//                 s->keyboard_dev = virtio_input_init(vbus,
//                                                     VIRTIO_INPUT_TYPE_KEYBOARD);
//                 vbus->addr += 0x1000;
//                 irq_num++;
//                 s->virtio_count++;
//     
//                 vbus->irq = &s->plic_irq[irq_num];
//                 s->mouse_dev = virtio_input_init(vbus,
//                                                  VIRTIO_INPUT_TYPE_TABLET);
//                 vbus->addr += 0x1000;
//                 irq_num++;
//                 s->virtio_count++;
//             } else {
//                 vm_error("unsupported input device: %s\n", p->input_device);
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2080 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *const c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
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
    fn vm_error(fmt: *const c_char, ...);
    fn exit(code: c_int) -> !;
    fn virtio_input_init(bus: *mut VIRTIOBusDef, type_: c_int) -> *mut c_void;
}

const VIRTIO_INPUT_TYPE_KEYBOARD: c_int = 0;
const VIRTIO_INPUT_TYPE_TABLET: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2080(
    input_device: *const c_char,
    vbus: *mut VIRTIOBusDef,
    plic_irq: *mut IRQSignal,
    mut irq_num: c_int,
    keyboard_dev: *mut *mut c_void,
    mouse_dev: *mut *mut c_void,
    virtio_count: *mut c_int,
) -> c_int {
    if input_device.is_null() {
        return irq_num;
    }
    if strcmp(input_device, b"virtio\0".as_ptr() as *const c_char) == 0 {
        (*vbus).irq = plic_irq.add(irq_num as usize);
        *keyboard_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_KEYBOARD);
        (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
        irq_num = irq_num.wrapping_add(1);
        *virtio_count = (*virtio_count).wrapping_add(1);

        (*vbus).irq = plic_irq.add(irq_num as usize);
        *mouse_dev = virtio_input_init(vbus, VIRTIO_INPUT_TYPE_TABLET);
        (*vbus).addr = (*vbus).addr.wrapping_add(0x1000);
        irq_num = irq_num.wrapping_add(1);
        *virtio_count = (*virtio_count).wrapping_add(1);
    } else {
        vm_error(
            b"unsupported input device: %s\n\0".as_ptr() as *const c_char,
            input_device,
        );
        exit(1);
    }
    irq_num
}
}

// ── `if (!p->files[VM_FILE_BIOS].buf)` · riscv_machine.c:1031-1033 · chunk 2081 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!p->files[VM_FILE_BIOS].buf) {
//             vm_error("No bios found");
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2081 {
// Port of the "no bios found" check inside riscv_machine_init.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct VMDriveEntry {
    device: *mut c_char,
    filename: *mut c_char,
    block_dev: *mut c_void,
}

#[repr(C)]
struct VMFSEntry {
    device: *mut c_char,
    tag: *mut c_char,
    filename: *mut c_char,
    fs_dev: *mut c_void,
}

#[repr(C)]
struct VMEthEntry {
    driver: *mut c_char,
    ifname: *mut c_char,
    net: *mut c_void,
}

#[repr(C)]
struct VMFileEntry {
    filename: *mut c_char,
    buf: *mut u8,
    len: c_int,
}

#[repr(C)]
struct VirtMachineParams {
    cfg_filename: *mut c_char,
    vmc: *const c_void,
    machine_name: *mut c_char,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_local_time: c_int,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    console: *mut c_void,
    tab_drive: [VMDriveEntry; 4],
    drive_count: c_int,
    tab_fs: [VMFSEntry; 4],
    fs_count: c_int,
    tab_eth: [VMEthEntry; 1],
    eth_count: c_int,
    cmdline: *mut c_char,
    accel_enable: c_int,
    input_device: *mut c_char,
    files: [VMFileEntry; 4],
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn riscv_machine_init__part2081(p: *const VirtMachineParams) {
    unsafe {
        let files = core::ptr::addr_of!((*p).files) as *const VMFileEntry;
        // VM_FILE_BIOS == 0
        let bios_buf = (*files).buf;
        if bios_buf.is_null() {
            vm_error(b"No bios found\0".as_ptr() as *const c_char);
        }
    }
}
}

// ── `copy_bios(s, p->files[VM_FILE_BIOS].buf, p->files[VM_FILE_BIOS].len, p->files[VM_FILE_KERNEL].b…` · riscv_machine.c:1035-1040 · chunk 2082 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     copy_bios(s, p->files[VM_FILE_BIOS].buf, p->files[VM_FILE_BIOS].len,
//                   p->files[VM_FILE_KERNEL].buf, p->files[VM_FILE_KERNEL].len,
//                   p->cmdline,
//                   p->files[VM_FILE_INITRD].buf, p->files[VM_FILE_INITRD].len);
//     
//         return (VirtMachine *)s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2082 {
// Rust port of the tail of riscv_machine_init(): the copy_bios() call.
// copy_bios() and riscv_build_fdt() are C `static` functions (no linker
// symbol), so their logic is ported here as private Rust functions.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

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
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut FBDevice,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: *mut c_void,
    riscv_cpu_end: *mut c_void,
    riscv_cpu_interp: *mut c_void,
    riscv_cpu_get_cycles: *mut c_void,
    riscv_cpu_set_mip: *mut c_void,
    riscv_cpu_reset_mip: *mut c_void,
    riscv_cpu_get_mip: *mut c_void,
    riscv_cpu_get_power_down: *mut c_void,
    riscv_cpu_get_misa: *mut c_void,
    riscv_cpu_flush_tlb_write_range_ram: *mut c_void,
}

#[repr(C)]
struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
struct FDTState {
    tab: *mut u32,
    tab_len: c_int,
    tab_size: c_int,
    open_node_count: c_int,
    string_table: *mut c_char,
    string_table_len: c_int,
    string_table_size: c_int,
}

#[repr(C)]
struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

#[repr(C)]
struct FdtReserveEntry {
    address: u64,
    size: u64,
}

extern "C" {
    fn phys_mem_get_ram_ptr(map: *mut c_void, paddr: u64, is_rw: c_int) -> *mut u8;
    fn elf_detect_magic(buf: *const u8, buf_len: c_int) -> c_int;
    fn elf_load(
        buf: *const u8,
        buf_len: c_int,
        outbuf: *mut u8,
        outbuf_len: c_int,
        start: *mut u64,
        len: *mut u64,
    ) -> c_int;
    fn compress_detect_magic(buf: *const u8, len: c_int) -> c_int;
    fn decompress(in_buf: *const u8, in_len: c_int, out_buf: *const u8, out_len: c_int) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn mallocz(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn abort() -> !;
}

fn cstr(b: &'static [u8]) -> *const c_char {
    b.as_ptr() as *const c_char
}

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

unsafe fn str_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: isize = 0;
    loop {
        let ca = *a.offset(i);
        let cb = *b.offset(i);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}

unsafe fn get_ram_ptr(s: *mut RISCVMachine, paddr: u64, is_rw: c_int) -> *mut u8 {
    phys_mem_get_ram_ptr((*s).mem_map, paddr, is_rw)
}

unsafe fn riscv_cpu_get_misa(cpu_state: *mut c_void) -> u32 {
    let c = (*(cpu_state as *mut RISCVCPUCommonState)).class_ptr;
    let f: unsafe extern "C" fn(*mut c_void) -> u32 = core::mem::transmute((*c).riscv_cpu_get_misa);
    f(cpu_state)
}

// ---------------------------------------------------------------------------
// FDT building helpers
// ---------------------------------------------------------------------------

unsafe fn fdt_init() -> *mut FDTState {
    let s = mallocz(core::mem::size_of::<FDTState>()) as *mut FDTState;
    s
}

unsafe fn fdt_alloc_len(s: &mut FDTState, len: c_int) {
    if len > s.tab_size {
        let new_size = max_int(len, s.tab_size.wrapping_mul(3) / 2);
        let bytes = (new_size as i64 as u64).wrapping_mul(4) as usize;
        s.tab = realloc(s.tab as *mut c_void, bytes) as *mut u32;
        s.tab_size = new_size;
    }
}

unsafe fn fdt_put32(s: &mut FDTState, v: c_int) {
    fdt_alloc_len(s, s.tab_len + 1);
    *s.tab.offset(s.tab_len as isize) = (v as u32).swap_bytes();
    s.tab_len += 1;
}

unsafe fn fdt_put_data(s: &mut FDTState, data: *const c_void, len: c_int) {
    let len1 = (len + 3) / 4;
    fdt_alloc_len(s, s.tab_len + len1);
    let base = s.tab.offset(s.tab_len as isize) as *mut u8;
    memcpy(base as *mut c_void, data, len as isize as usize);
    memset(
        base.wrapping_add(len as isize as usize) as *mut c_void,
        0,
        (len.wrapping_neg() & 3) as usize,
    );
    s.tab_len += len1;
}

unsafe fn fdt_begin_node(s: &mut FDTState, name: *const c_char) {
    fdt_put32(s, 1);
    fdt_put_data(s, name as *const c_void, strlen(name) as c_int + 1);
    s.open_node_count += 1;
}

// Format "%s@%lx" into buf (like snprintf with a 256 byte buffer).
unsafe fn fdt_begin_node_num(s: &mut FDTState, name: *const c_char, n: u64) {
    let mut buf = [0u8; 256];
    let mut len = 0usize;
    let mut i: isize = 0;
    loop {
        let c = *name.offset(i);
        if c == 0 {
            break;
        }
        if len < 255 {
            buf[len] = c as u8;
            len += 1;
        }
        i += 1;
    }
    if len < 255 {
        buf[len] = b'@';
        len += 1;
    }
    let mut digits = [0u8; 16];
    let mut d = 0usize;
    if n == 0 {
        digits[0] = b'0';
        d = 1;
    } else {
        let mut v = n;
        while v != 0 && d < 16 {
            digits[d] = b"0123456789abcdef"[(v & 0xf) as usize];
            d += 1;
            v >>= 4;
        }
    }
    while d > 0 {
        d -= 1;
        if len < 255 {
            buf[len] = digits[d];
            len += 1;
        }
    }
    buf[len] = 0;
    fdt_begin_node(s, buf.as_ptr() as *const c_char);
}

unsafe fn fdt_end_node(s: &mut FDTState) {
    fdt_put32(s, 2);
    s.open_node_count -= 1;
}

unsafe fn fdt_get_string_offset(s: &mut FDTState, name: *const c_char) -> c_int {
    let mut pos: c_int = 0;
    while pos < s.string_table_len {
        if str_eq(s.string_table.offset(pos as isize), name) {
            return pos;
        }
        pos += strlen(s.string_table.offset(pos as isize)) as c_int + 1;
    }

    let name_size = strlen(name) as c_int + 1;
    let new_len = s.string_table_len + name_size;
    if new_len > s.string_table_size {
        let new_size = max_int(new_len, s.string_table_size.wrapping_mul(3) / 2);
        s.string_table =
            realloc(s.string_table as *mut c_void, new_size as i64 as u64 as usize) as *mut c_char;
        s.string_table_size = new_size;
    }
    pos = s.string_table_len;
    memcpy(
        s.string_table.offset(pos as isize) as *mut c_void,
        name as *const c_void,
        name_size as i64 as u64 as usize,
    );
    s.string_table_len = new_len;
    pos
}

unsafe fn fdt_prop(s: &mut FDTState, prop_name: *const c_char, data: *const c_void, data_len: c_int) {
    fdt_put32(s, 3);
    fdt_put32(s, data_len);
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
    fdt_put_data(s, data, data_len);
}

unsafe fn fdt_prop_tab_u32(s: &mut FDTState, prop_name: *const c_char, tab: *mut u32, tab_len: c_int) {
    fdt_put32(s, 3);
    fdt_put32(s, tab_len.wrapping_mul(4));
    let off = fdt_get_string_offset(s, prop_name);
    fdt_put32(s, off);
    let mut i: c_int = 0;
    while i < tab_len {
        fdt_put32(s, *tab.offset(i as isize) as c_int);
        i += 1;
    }
}

unsafe fn fdt_prop_u32(s: &mut FDTState, prop_name: *const c_char, val: u32) {
    let mut v = val;
    fdt_prop_tab_u32(s, prop_name, &mut v as *mut u32, 1);
}

unsafe fn fdt_prop_tab_u64(s: &mut FDTState, prop_name: *const c_char, v0: u64) {
    let mut tab = [0u32; 2];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 2);
}

unsafe fn fdt_prop_tab_u64_2(s: &mut FDTState, prop_name: *const c_char, v0: u64, v1: u64) {
    let mut tab = [0u32; 4];
    tab[0] = (v0 >> 32) as u32;
    tab[1] = v0 as u32;
    tab[2] = (v1 >> 32) as u32;
    tab[3] = v1 as u32;
    fdt_prop_tab_u32(s, prop_name, tab.as_mut_ptr(), 4);
}

unsafe fn fdt_prop_str(s: &mut FDTState, prop_name: *const c_char, st: *const c_char) {
    fdt_prop(
        s,
        prop_name,
        st as *const c_void,
        strlen(st) as c_int + 1,
    );
}

// Varargs version specialised for the single two-string call made below.
unsafe fn fdt_prop_tab_str2(
    s: &mut FDTState,
    prop_name: *const c_char,
    a: *const c_char,
    b: *const c_char,
) {
    let sa = strlen(a) as c_int + 1;
    let sb = strlen(b) as c_int + 1;
    let size = sa + sb;
    let tab = malloc(size as i64 as u64 as usize) as *mut u8;
    memcpy(tab as *mut c_void, a as *const c_void, sa as usize);
    memcpy(tab.wrapping_add(sa as usize) as *mut c_void, b as *const c_void, sb as usize);
    fdt_prop(s, prop_name, tab as *const c_void, size);
    free(tab as *mut c_void);
}

unsafe fn fdt_output(s: &mut FDTState, dst: *mut u8) -> c_int {
    if s.open_node_count != 0 {
        abort();
    }

    fdt_put32(s, 9);

    let dt_struct_size = s.tab_len.wrapping_mul(4);
    let dt_strings_size = s.string_table_len;

    let h = dst as *mut FdtHeader;
    (*h).magic = 0xd00dfeedu32.swap_bytes();
    (*h).version = 17u32.swap_bytes();
    (*h).last_comp_version = 16u32.swap_bytes();
    (*h).boot_cpuid_phys = 0u32.swap_bytes();
    (*h).size_dt_strings = (dt_strings_size as u32).swap_bytes();
    (*h).size_dt_struct = (dt_struct_size as u32).swap_bytes();

    let mut pos: c_int = core::mem::size_of::<FdtHeader>() as c_int;

    (*h).off_dt_struct = (pos as u32).swap_bytes();
    memcpy(
        dst.wrapping_add(pos as isize as usize) as *mut c_void,
        s.tab as *const c_void,
        dt_struct_size as isize as usize,
    );
    pos = pos.wrapping_add(dt_struct_size);

    while (pos & 7) != 0 {
        *dst.wrapping_add(pos as isize as usize) = 0;
        pos += 1;
    }
    (*h).off_mem_rsvmap = (pos as u32).swap_bytes();
    let re = dst.wrapping_add(pos as isize as usize) as *mut FdtReserveEntry;
    (*re).address = 0;
    (*re).size = 0;
    pos = pos.wrapping_add(core::mem::size_of::<FdtReserveEntry>() as c_int);

    (*h).off_dt_strings = (pos as u32).swap_bytes();
    memcpy(
        dst.wrapping_add(pos as isize as usize) as *mut c_void,
        s.string_table as *const c_void,
        dt_strings_size as isize as usize,
    );
    pos = pos.wrapping_add(dt_strings_size);

    while (pos & 7) != 0 {
        *dst.wrapping_add(pos as isize as usize) = 0;
        pos += 1;
    }

    (*h).totalsize = (pos as u32).swap_bytes();
    pos
}

unsafe fn fdt_end(s: *mut FDTState) {
    free((*s).tab as *mut c_void);
    free((*s).string_table as *mut c_void);
    free(s as *mut c_void);
}

// snprintf(buf, 128, "rv%d", max_xlen); returns the number of chars written.
fn fmt_rv(buf: &mut [u8; 128], v: c_int) -> c_int {
    let mut n = 0usize;
    buf[n] = b'r';
    n += 1;
    buf[n] = b'v';
    n += 1;
    let neg = v < 0;
    let mut x: u32 = if neg {
        (v as i64).unsigned_abs() as u32
    } else {
        v as u32
    };
    let mut digits = [0u8; 10];
    let mut d = 0usize;
    if x == 0 {
        digits[0] = b'0';
        d = 1;
    }
    while x != 0 && d < 10 {
        digits[d] = b'0' + (x % 10) as u8;
        d += 1;
        x /= 10;
    }
    if neg {
        buf[n] = b'-';
        n += 1;
    }
    while d > 0 {
        d -= 1;
        buf[n] = digits[d];
        n += 1;
    }
    n as c_int
}

unsafe fn riscv_build_fdt(
    m: *mut RISCVMachine,
    dst: *mut u8,
    kernel_start: u64,
    kernel_size: u64,
    cmd_line: *const c_char,
    initrd_start: u64,
    initrd_size: u64,
) -> c_int {
    let s_ptr = fdt_init();
    let s: &mut FDTState = &mut *s_ptr;

    let mut cur_phandle: c_int = 1;

    fdt_begin_node(s, cstr(b"\0"));
    fdt_prop_u32(s, cstr(b"#address-cells\0"), 2);
    fdt_prop_u32(s, cstr(b"#size-cells\0"), 2);
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"ucbbar,riscvemu-bar_dev\0"));
    fdt_prop_str(s, cstr(b"model\0"), cstr(b"ucbbar,riscvemu-bare\0"));

    fdt_begin_node(s, cstr(b"cpus\0"));
    fdt_prop_u32(s, cstr(b"#address-cells\0"), 1);
    fdt_prop_u32(s, cstr(b"#size-cells\0"), 0);
    fdt_prop_u32(s, cstr(b"timebase-frequency\0"), 10000000);

    fdt_begin_node_num(s, cstr(b"cpu\0"), 0);
    fdt_prop_str(s, cstr(b"device_type\0"), cstr(b"cpu\0"));
    fdt_prop_u32(s, cstr(b"reg\0"), 0);
    fdt_prop_str(s, cstr(b"status\0"), cstr(b"okay\0"));
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"riscv\0"));

    let max_xlen = (*m).max_xlen;
    let misa = riscv_cpu_get_misa((*m).cpu_state);
    let mut isa_string = [0u8; 128];
    let mut q = fmt_rv(&mut isa_string, max_xlen) as usize;
    let mut i: c_int = 0;
    while i < 26 {
        if (misa & ((1u32) << (i as u32))) != 0 {
            if q < 127 {
                isa_string[q] = b'a' + i as u8;
                q += 1;
            }
        }
        i += 1;
    }
    isa_string[q] = 0;
    fdt_prop_str(s, cstr(b"riscv,isa\0"), isa_string.as_ptr() as *const c_char);

    let mmu: *const c_char = if max_xlen <= 32 {
        cstr(b"riscv,sv32\0")
    } else {
        cstr(b"riscv,sv48\0")
    };
    fdt_prop_str(s, cstr(b"mmu-type\0"), mmu);
    fdt_prop_u32(s, cstr(b"clock-frequency\0"), 2000000000);

    fdt_begin_node(s, cstr(b"interrupt-controller\0"));
    fdt_prop_u32(s, cstr(b"#interrupt-cells\0"), 1);
    fdt_prop(s, cstr(b"interrupt-controller\0"), core::ptr::null(), 0);
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"riscv,cpu-intc\0"));
    let intc_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    fdt_prop_u32(s, cstr(b"phandle\0"), intc_phandle as u32);
    fdt_end_node(s);

    fdt_end_node(s);

    fdt_end_node(s);

    fdt_begin_node_num(s, cstr(b"memory\0"), 0x80000000);
    fdt_prop_str(s, cstr(b"device_type\0"), cstr(b"memory\0"));
    let ram_size = (*m).ram_size;
    let mut tab = [0u32; 4];
    tab[0] = (0x80000000u64 >> 32) as u32;
    tab[1] = 0x80000000u64 as u32;
    tab[2] = (ram_size >> 32) as u32;
    tab[3] = ram_size as u32;
    fdt_prop_tab_u32(s, cstr(b"reg\0"), tab.as_mut_ptr(), 4);

    fdt_end_node(s);

    fdt_begin_node(s, cstr(b"htif\0"));
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"ucb,htif0\0"));
    fdt_end_node(s);

    fdt_begin_node(s, cstr(b"soc\0"));
    fdt_prop_u32(s, cstr(b"#address-cells\0"), 2);
    fdt_prop_u32(s, cstr(b"#size-cells\0"), 2);
    fdt_prop_tab_str2(
        s,
        cstr(b"compatible\0"),
        cstr(b"ucbbar,riscvemu-bar-soc\0"),
        cstr(b"simple-bus\0"),
    );
    fdt_prop(s, cstr(b"ranges\0"), core::ptr::null(), 0);

    fdt_begin_node_num(s, cstr(b"clint\0"), 0x02000000);
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"riscv,clint0\0"));

    tab[0] = intc_phandle as u32;
    tab[1] = 3;
    tab[2] = intc_phandle as u32;
    tab[3] = 7;
    fdt_prop_tab_u32(s, cstr(b"interrupts-extended\0"), tab.as_mut_ptr(), 4);

    fdt_prop_tab_u64_2(s, cstr(b"reg\0"), 0x02000000, 0x000c0000);

    fdt_end_node(s);

    fdt_begin_node_num(s, cstr(b"plic\0"), 0x40100000);
    fdt_prop_u32(s, cstr(b"#interrupt-cells\0"), 1);
    fdt_prop(s, cstr(b"interrupt-controller\0"), core::ptr::null(), 0);
    fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"riscv,plic0\0"));
    fdt_prop_u32(s, cstr(b"riscv,ndev\0"), 31);
    fdt_prop_tab_u64_2(s, cstr(b"reg\0"), 0x40100000, 0x00400000);

    tab[0] = intc_phandle as u32;
    tab[1] = 9;
    tab[2] = intc_phandle as u32;
    tab[3] = 11;
    fdt_prop_tab_u32(s, cstr(b"interrupts-extended\0"), tab.as_mut_ptr(), 4);

    let plic_phandle = cur_phandle;
    cur_phandle = cur_phandle.wrapping_add(1);
    fdt_prop_u32(s, cstr(b"phandle\0"), plic_phandle as u32);

    fdt_end_node(s);

    i = 0;
    while i < (*m).virtio_count {
        let addr: c_int = 0x40010000i32.wrapping_add(i.wrapping_mul(0x1000));
        fdt_begin_node_num(s, cstr(b"virtio\0"), addr as i64 as u64);
        fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"virtio,mmio\0"));
        fdt_prop_tab_u64_2(
            s,
            cstr(b"reg\0"),
            addr as i64 as u64,
            0x1000,
        );
        tab[0] = plic_phandle as u32;
        tab[1] = (1 + i) as u32;
        fdt_prop_tab_u32(s, cstr(b"interrupts-extended\0"), tab.as_mut_ptr(), 2);
        fdt_end_node(s);
        i += 1;
    }

    let fb_dev = (*m).common.fb_dev;
    if !fb_dev.is_null() {
        fdt_begin_node_num(s, cstr(b"framebuffer\0"), 0x41000000);
        fdt_prop_str(s, cstr(b"compatible\0"), cstr(b"simple-framebuffer\0"));
        fdt_prop_tab_u64_2(s, cstr(b"reg\0"), 0x41000000, (*fb_dev).fb_size as i64 as u64);
        fdt_prop_u32(s, cstr(b"width\0"), (*fb_dev).width as u32);
        fdt_prop_u32(s, cstr(b"height\0"), (*fb_dev).height as u32);
        fdt_prop_u32(s, cstr(b"stride\0"), (*fb_dev).stride as u32);
        fdt_prop_str(s, cstr(b"format\0"), cstr(b"a8r8g8b8\0"));
        fdt_end_node(s);
    }

    fdt_end_node(s);

    fdt_begin_node(s, cstr(b"chosen\0"));
    let args: *const c_char = if !cmd_line.is_null() {
        cmd_line
    } else {
        cstr(b"\0")
    };
    fdt_prop_str(s, cstr(b"bootargs\0"), args);
    if kernel_size > 0 {
        fdt_prop_tab_u64(s, cstr(b"riscv,kernel-start\0"), kernel_start);
        fdt_prop_tab_u64(
            s,
            cstr(b"riscv,kernel-end\0"),
            kernel_start.wrapping_add(kernel_size),
        );
    }
    if initrd_size > 0 {
        fdt_prop_tab_u64(s, cstr(b"linux,initrd-start\0"), initrd_start);
        fdt_prop_tab_u64(
            s,
            cstr(b"linux,initrd-end\0"),
            initrd_start.wrapping_add(initrd_size),
        );
    }

    fdt_end_node(s);

    fdt_end_node(s);

    let size = fdt_output(s, dst);
    fdt_end(s_ptr);
    size
}

// ---------------------------------------------------------------------------
// copy_bios
// ---------------------------------------------------------------------------

unsafe fn copy_bios(
    s: *mut RISCVMachine,
    buf: *const u8,
    buf_len: c_int,
    kernel_buf: *const u8,
    kernel_buf_len: c_int,
    cmd_line: *const c_char,
    initrd_buf: *const u8,
    initrd_buf_len: c_int,
) {
    let ram_size = (*s).ram_size;
    let max_xlen = (*s).max_xlen;

    let mut ram_ptr: *mut u8 = get_ram_ptr(s, 0x80000000, 1);

    let mut image_start: u64 = 0;
    let mut image_len: u64 = 0;

    let mut bios_base: u64;
    let mut bios_size: u64;

    if elf_detect_magic(buf, buf_len) != 0 {
        if elf_load(
            buf,
            buf_len,
            ram_ptr,
            ram_size as c_int,
            &mut image_start,
            &mut image_len,
        ) == -1
        {
            vm_error(cstr(b"Failed to load ELF BIOS\n\0"));
            exit(1);
        }
        bios_base = image_start.wrapping_sub(0x80000000);
        bios_size = image_len;
    } else {
        if (buf_len as i64 as u64) > ram_size {
            vm_error(cstr(b"BIOS too big\n\0"));
            exit(1);
        }
        memcpy(
            ram_ptr as *mut c_void,
            buf as *const c_void,
            buf_len as isize as usize,
        );
        bios_base = 0x80000000;
        bios_size = buf_len as i64 as u64;
    }

    let mut kernel_base: u64;
    let mut kernel_size: u64;

    if kernel_buf_len > 0 {
        let kernel_align: u64 = if max_xlen == 32 { 4u64 << 20 } else { 2u64 << 20 };
        kernel_base = (buf_len as i64 as u64)
            .wrapping_add(kernel_align)
            .wrapping_sub(1)
            & !(kernel_align.wrapping_sub(1));
        if elf_detect_magic(kernel_buf, kernel_buf_len) != 0 {
            if elf_load(
                kernel_buf,
                kernel_buf_len,
                ram_ptr.wrapping_add(kernel_base as usize),
                ram_size.wrapping_sub(kernel_base) as c_int,
                &mut image_start,
                &mut image_len,
            ) == -1
            {
                vm_error(cstr(b"Failed to load ELF kernel\n\0"));
                exit(1);
            }
            kernel_base = kernel_base.wrapping_add(image_start);
            kernel_size = image_len;
        } else {
            memcpy(
                ram_ptr.wrapping_add(kernel_base as usize) as *mut c_void,
                kernel_buf as *const c_void,
                kernel_buf_len as isize as usize,
            );
            kernel_size = kernel_buf_len as i64 as u64;
        }
    } else {
        kernel_base = 0;
        kernel_size = 0;
    }

    let mut initrd_base: u64;
    let mut initrd_size: u64;

    if initrd_buf_len > 0 {
        if kernel_size > 0 {
            initrd_base = kernel_base.wrapping_add(kernel_size);
        } else {
            initrd_base = bios_base.wrapping_add(bios_size);
        }
        initrd_base &= !(((1u64 << 12) - 1) as u64);
        initrd_base = initrd_base.wrapping_add(1u64 << 12);
        if initrd_base.wrapping_add(initrd_buf_len as i64 as u64) > ram_size {
            vm_error(cstr(b"initrd too big\n\0"));
            exit(1);
        }

        if compress_detect_magic(initrd_buf, initrd_buf_len) != 0 {
            let res = decompress(
                initrd_buf,
                initrd_buf_len,
                ram_ptr.wrapping_add(initrd_base as usize) as *const u8,
                ram_size.wrapping_sub(kernel_base) as c_int,
            );
            if res == -1 {
                vm_error(cstr(b"Failed to decompress initramfs image\n\0"));
                exit(1);
            }
            initrd_size = res as i64 as u64;
        } else {
            memcpy(
                ram_ptr.wrapping_add(initrd_base as usize) as *mut c_void,
                initrd_buf as *const c_void,
                initrd_buf_len as isize as usize,
            );
            initrd_size = initrd_buf_len as i64 as u64;
        }
    } else {
        initrd_base = 0;
        initrd_size = 0;
    }

    ram_ptr = get_ram_ptr(s, 0, 1);

    let fdt_addr: u64 = 0x1000 + 8 * 8;

    riscv_build_fdt(
        s,
        ram_ptr.wrapping_add(fdt_addr as usize),
        0x80000000u64.wrapping_add(kernel_base),
        kernel_size,
        cmd_line,
        0x80000000u64.wrapping_add(initrd_base),
        initrd_size,
    );

    let q = ram_ptr.wrapping_add(0x1000) as *mut u32;
    *q.offset(0) = 0x297u32.wrapping_add(0x80000000).wrapping_sub(0x1000);
    *q.offset(1) = 0x597;
    *q.offset(2) = (0x58593u64.wrapping_add(fdt_addr.wrapping_sub(4) << 20)) as u32;
    *q.offset(3) = 0xf1402573;
    *q.offset(4) = 0x00028067;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_init__part2082(
    s: *mut RISCVMachine,
    buf: *const u8,
    buf_len: c_int,
    kernel_buf: *const u8,
    kernel_buf_len: c_int,
    cmd_line: *const c_char,
    initrd_buf: *const u8,
    initrd_buf_len: c_int,
) {
    copy_bios(
        s,
        buf,
        buf_len,
        kernel_buf,
        kernel_buf_len,
        cmd_line,
        initrd_buf,
        initrd_buf_len,
    );
}
}

// ── `riscv_machine_end` · riscv_machine.c:1043-1050 · chunk 2083 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_machine_end(VirtMachine *s1)
//     {
//         RISCVMachine *s = (RISCVMachine *)s1;
//     
//         riscv_cpu_end(s->cpu_state);
//         phys_mem_map_end(s->mem_map);
//         free(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2083 {
// Rewrite of riscv_machine_end() from riscv_machine.c
use core::ffi::{c_int, c_void};

// Prefix mirror of `VirtMachine` (machine.h): 5 pointers.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

// Prefix mirror of `RISCVMachine`; only the leading fields are accessed here.
#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
}

// Prefix mirror of `RISCVCPUClass` (riscv_cpu.h).
#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<extern "C" fn(mem_map: *mut c_void) -> *mut c_void>,
    pub riscv_cpu_end: Option<extern "C" fn(s: *mut c_void)>,
}

// `RISCVCPUCommonState`: the CPU state starts with a class pointer.
#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

extern "C" {
    fn phys_mem_map_end(s: *mut c_void);
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_end(s1: *mut VirtMachine) {
    let s = s1 as *mut RISCVMachine;
    let s = &mut *s;

    // riscv_cpu_end(s->cpu_state): static inline dispatch through the class.
    let cpu = s.cpu_state;
    if !cpu.is_null() {
        let class_ptr = (*(cpu as *const RISCVCPUCommonState)).class_ptr;
        if !class_ptr.is_null() {
            if let Some(end_fn) = (*class_ptr).riscv_cpu_end {
                end_fn(cpu);
            }
        }
    }

    phys_mem_map_end(s.mem_map);
    free(s1 as *mut c_void);
}
}

// ── `RISCVMachine *s = (RISCVMachine *)s1; riscv_cpu_end(s->cpu_state); phys_mem_map_end(s->mem_map)…` · riscv_machine.c:1045-1049 · chunk 2084 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = (RISCVMachine *)s1;
//     
//         riscv_cpu_end(s->cpu_state);
//         phys_mem_map_end(s->mem_map);
//         free(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2084 {
// Port of the body of riscv_machine_end (riscv_machine.c lines 1045-1049).
#![allow(non_snake_case, non_camel_case_types, dead_code)]

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

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

// Only the first two members of RISCVCPUClass are needed.
#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    pub riscv_cpu_end: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

extern "C" {
    fn phys_mem_map_end(s: *mut c_void);
    fn free(p: *mut c_void);
}

// Ported logic of the C `static inline void riscv_cpu_end(RISCVCPUState *s)`.
unsafe fn riscv_cpu_end(s: *mut c_void) {
    let common = s as *mut RISCVCPUCommonState;
    let class_ptr = (*common).class_ptr;
    if !class_ptr.is_null() {
        let end_fn = (*class_ptr).riscv_cpu_end;
        if let Some(f) = end_fn {
            f(s);
        }
    }
}

#[no_mangle]
pub extern "C" fn riscv_machine_end__part2084(s1: *mut VirtMachine) {
    let s = s1 as *mut RISCVMachine;
    unsafe {
        let cpu_state = (*s).cpu_state;
        let mem_map = (*s).mem_map;
        riscv_cpu_end(cpu_state);
        phys_mem_map_end(mem_map);
        free(s as *mut c_void);
    }
}
}

// ── `riscv_machine_get_sleep_duration` · riscv_machine.c:1053-1075 · chunk 2085 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int riscv_machine_get_sleep_duration(VirtMachine *s1, int delay)
//     {
//         RISCVMachine *m = (RISCVMachine *)s1;
//         RISCVCPUState *s = m->cpu_state;
//         int64_t delay1;
//     
//     
//         if (!(riscv_cpu_get_mip(s) & (1 << 7))) {
//             delay1 = m->timecmp - rtc_get_time(m);
//             if (delay1 <= 0) {
//                 riscv_cpu_set_mip(s, (1 << 7));
//                 delay = 0;
//             } else {
//     
//                 delay1 = delay1 / (10000000 / 1000);
//                 if (delay1 < delay)
//                     delay = delay1;
//             }
//         }
//         if (!riscv_cpu_get_power_down(s))
//             delay = 0;
//         return delay;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2085 {
// Rust rewrite of static int riscv_machine_get_sleep_duration(VirtMachine *s1, int delay)
// from riscv_machine.c. The helpers rtc_get_time / rtc_get_real_time are `static`
// (no linker symbol), so their logic is ported here verbatim.

use core::ffi::{c_int, c_void};

type BOOL = c_int;

const CLOCK_MONOTONIC: c_int = 1;
const RTC_FREQ: u64 = 10000000;
const MIP_MTIP: u32 = 1 << 7;

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clockid: c_int, tp: *mut Timespec) -> c_int;
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Only the layout up to the first member (and none of its fields) is used.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

pub enum RISCVCPUState {}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(*mut c_void) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> BOOL>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,

    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,

    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],

    pub htif_tohost: u64,
    pub htif_fromhost: u64,

    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,

    pub virtio_count: c_int,
}

#[inline]
unsafe fn cpu_class(s: *mut RISCVCPUState) -> *const RISCVCPUClass {
    (*(s as *mut RISCVCPUCommonState)).class_ptr
}

unsafe fn riscv_cpu_get_cycles(s: *mut RISCVCPUState) -> u64 {
    match (*cpu_class(s)).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0,
    }
}

unsafe fn riscv_cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    if let Some(f) = (*cpu_class(s)).riscv_cpu_set_mip {
        f(s, mask);
    }
}

unsafe fn riscv_cpu_get_mip(s: *mut RISCVCPUState) -> u32 {
    match (*cpu_class(s)).riscv_cpu_get_mip {
        Some(f) => f(s),
        None => 0,
    }
}

unsafe fn riscv_cpu_get_power_down(s: *mut RISCVCPUState) -> BOOL {
    match (*cpu_class(s)).riscv_cpu_get_power_down {
        Some(f) => f(s),
        None => 0,
    }
}

unsafe fn rtc_get_real_time(_s: *mut RISCVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    (ts.tv_sec as u64).wrapping_mul(RTC_FREQ)
        .wrapping_add((ts.tv_nsec / (1000000000 / RTC_FREQ as i64)) as u64)
}

unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    if (*m).rtc_real_time != 0 {
        rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time)
    } else {
        riscv_cpu_get_cycles((*m).cpu_state) / 16
    }
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_get_sleep_duration(
    s1: *mut VirtMachine,
    mut delay: c_int,
) -> c_int {
    let m = s1 as *mut RISCVMachine;
    let s = (*m).cpu_state;
    let mut delay1: i64;

    // wait for an event: the only asynchronous event is the RTC timer
    if (riscv_cpu_get_mip(s) & MIP_MTIP) == 0 {
        delay1 = (*m).timecmp.wrapping_sub(rtc_get_time(m)) as i64;
        if delay1 <= 0 {
            riscv_cpu_set_mip(s, MIP_MTIP);
            delay = 0;
        } else {
            // convert delay to ms
            delay1 = delay1 / (RTC_FREQ as i64 / 1000);
            if delay1 < delay as i64 {
                delay = delay1 as c_int;
            }
        }
    }
    if riscv_cpu_get_power_down(s) == 0 {
        delay = 0;
    }
    delay
}
}

// ── `if (!(riscv_cpu_get_mip(s) & (1 << 7)` · riscv_machine.c:1060-1071 · chunk 2086 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(riscv_cpu_get_mip(s) & (1 << 7))) {
//             delay1 = m->timecmp - rtc_get_time(m);
//             if (delay1 <= 0) {
//                 riscv_cpu_set_mip(s, (1 << 7));
//                 delay = 0;
//             } else {
//     
//                 delay1 = delay1 / (10000000 / 1000);
//                 if (delay1 < delay)
//                     delay = delay1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2086 {
// Port of the timer/interrupt part of riscv_machine_get_sleep_duration().
use core::ffi::{c_int, c_void};

const CLOCK_MONOTONIC: c_int = 1;
const MIP_MTIP: u32 = 1 << 7;
const TICKS_PER_MS: i64 = 10000000 / 1000;

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

#[repr(C)]
pub struct RISCVCPUState {
    _priv: [u8; 0],
}

// Mirrors `RISCVCPUClass` (function pointer table); unused entries stay opaque.
#[repr(C)]
struct RISCVCPUClass {
    riscv_cpu_init: *mut c_void,
    riscv_cpu_end: *mut c_void,
    riscv_cpu_interp: *mut c_void,
    riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    riscv_cpu_reset_mip: *mut c_void,
    riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    riscv_cpu_get_power_down: *mut c_void,
    riscv_cpu_get_misa: *mut c_void,
    riscv_cpu_flush_tlb_write_range_ram: *mut c_void,
}

#[repr(C)]
struct RISCVCPUCommonState {
    class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
struct VirtMachine {
    vmc: *const c_void,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// Mirror of RISCVMachine up to (and including) the fields this piece touches.
#[repr(C)]
pub struct RISCVMachine {
    common: VirtMachine,
    mem_map: *mut c_void,
    max_xlen: c_int,
    cpu_state: *mut RISCVCPUState,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_start_time: u64,
    timecmp: u64,
}

#[inline]
unsafe fn cpu_class(s: *mut RISCVCPUState) -> *const RISCVCPUClass {
    (*(s as *mut RISCVCPUCommonState)).class_ptr
}

// C: static inline void riscv_cpu_set_mip(RISCVCPUState *s, uint32_t mask)
unsafe fn cpu_set_mip(s: *mut RISCVCPUState, mask: u32) {
    let c = cpu_class(s);
    if !c.is_null() {
        if let Some(f) = (*c).riscv_cpu_set_mip {
            f(s, mask);
        }
    }
}

// C: static inline uint32_t riscv_cpu_get_mip(RISCVCPUState *s)
unsafe fn cpu_get_mip(s: *mut RISCVCPUState) -> u32 {
    let c = cpu_class(s);
    if c.is_null() {
        return 0;
    }
    match (*c).riscv_cpu_get_mip {
        Some(f) => f(s),
        None => 0,
    }
}

// C: static inline uint64_t riscv_cpu_get_cycles(RISCVCPUState *s)
unsafe fn cpu_get_cycles(s: *mut RISCVCPUState) -> u64 {
    let c = cpu_class(s);
    if c.is_null() {
        return 0;
    }
    match (*c).riscv_cpu_get_cycles {
        Some(f) => f(s),
        None => 0,
    }
}

// C: static uint64_t rtc_get_real_time(RISCVMachine *s)
unsafe fn rtc_get_real_time(_m: *mut RISCVMachine) -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &mut ts as *mut Timespec);
    (ts.tv_sec as u64)
        .wrapping_mul(10000000)
        .wrapping_add((ts.tv_nsec / (1000000000 / 10000000)) as u64)
}

// C: static uint64_t rtc_get_time(RISCVMachine *m)
unsafe fn rtc_get_time(m: *mut RISCVMachine) -> u64 {
    if (*m).rtc_real_time != 0 {
        rtc_get_real_time(m).wrapping_sub((*m).rtc_start_time)
    } else {
        cpu_get_cycles((*m).cpu_state) / 16
    }
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_get_sleep_duration__part2086(
    m: *mut RISCVMachine,
    delay: c_int,
) -> c_int {
    let mut delay = delay;
    let s = (*m).cpu_state;

    if cpu_get_mip(s) & MIP_MTIP == 0 {
        let delay1 = (*m).timecmp.wrapping_sub(rtc_get_time(m)) as i64;
        if delay1 <= 0 {
            cpu_set_mip(s, MIP_MTIP);
            delay = 0;
        } else {
            let delay1 = delay1 / TICKS_PER_MS;
            if delay1 < delay as i64 {
                delay = delay1 as c_int;
            }
        }
    }
    delay
}
}

// ── `if (!riscv_cpu_get_power_down(s))` · riscv_machine.c:1072-1073 · chunk 2087 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!riscv_cpu_get_power_down(s))
//             delay = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2087 {
// Port of the TARGET inside riscv_machine_get_sleep_duration:
//     if (!riscv_cpu_get_power_down(s))
//         delay = 0;
// riscv_cpu_get_power_down() is a `static inline` C helper (no linker symbol),
// so its logic (dispatch through the class_ptr vtable) is reproduced here.

use core::ffi::c_int;

#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_get_sleep_duration__part2087(
    s: *mut RISCVCPUState,
    delay: c_int,
) -> c_int {
    let mut delay = delay;
    // static inline BOOL riscv_cpu_get_power_down(RISCVCPUState *s)
    let class_ptr = (*(s as *mut RISCVCPUCommonState)).class_ptr;
    let power_down = match (*class_ptr).riscv_cpu_get_power_down {
        Some(f) => f(s),
        None => 0,
    };
    if !(power_down != 0) {
        delay = 0;
    }
    delay
}
}

// ── `return delay;` · riscv_machine.c:1074-1074 · chunk 2088 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return delay;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2088 {
// Ported piece of riscv_machine_get_sleep_duration: just returns the delay.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn riscv_machine_get_sleep_duration__part2088(delay: c_int) -> c_int {
    delay
}
}

// ── `riscv_machine_interp` · riscv_machine.c:1077-1081 · chunk 2089 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_machine_interp(VirtMachine *s1, int max_exec_cycle)
//     {
//         RISCVMachine *s = (RISCVMachine *)s1;
//         riscv_cpu_interp(s->cpu_state, max_exec_cycle);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2089 {
// Rust port of riscv_machine_interp (riscv_machine.c).
// Calls through the CPU class vtable exactly as the C static inline
// riscv_cpu_interp() helper does.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;

// Opaque C types (only ever used behind pointers here).
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct RISCVCPUState {
    _private: [u8; 0],
}
#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}
#[repr(C)]
pub struct VirtMachineClass {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
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
pub struct SetIRQFunc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> BOOL>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

// Unused-but-referenced type kept for completeness of the mirrored header.
#[repr(C)]
pub struct VMFileEntry {
    _private: [u8; 0],
}

const _: () = {
    // The C name of the exported symbol has no conflicts; keep c_char used.
    let _ = core::mem::size_of::<c_char>();
};

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_interp(s1: *mut VirtMachine, max_exec_cycle: c_int) {
    if s1.is_null() {
        return;
    }
    let s = s1 as *mut RISCVMachine;
    let cpu = core::ptr::addr_of!((*s).cpu_state).read();
    if cpu.is_null() {
        return;
    }
    let class_ptr = core::ptr::addr_of!((*(cpu as *const RISCVCPUCommonState)).class_ptr).read();
    if class_ptr.is_null() {
        return;
    }
    if let Some(interp) = core::ptr::addr_of!((*class_ptr).riscv_cpu_interp).read() {
        interp(cpu, max_exec_cycle);
    }
}
}

// ── `RISCVMachine *s = (RISCVMachine *)s1; riscv_cpu_interp(s->cpu_state, max_exec_cycle);` · riscv_machine.c:1079-1080 · chunk 2090 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     RISCVMachine *s = (RISCVMachine *)s1;
//         riscv_cpu_interp(s->cpu_state, max_exec_cycle);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2090 {
// Port of the body of riscv_machine_interp (riscv_machine.c lines 1079-1080):
// cast the VirtMachine pointer to RISCVMachine and dispatch one interpreter
// slice through the CPU class vtable (logic of the static inline
// riscv_cpu_interp, which has no linker symbol).

#![allow(non_camel_case_types, dead_code)]

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
pub struct IRQSignal {
    pub set_irq: *const c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct RISCVCPUState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

#[repr(C)]
pub struct RISCVCPUClass {
    pub riscv_cpu_init:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap) -> *mut RISCVCPUState>,
    pub riscv_cpu_end: Option<unsafe extern "C" fn(*mut RISCVCPUState)>,
    pub riscv_cpu_interp: Option<unsafe extern "C" fn(*mut RISCVCPUState, c_int)>,
    pub riscv_cpu_get_cycles: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u64>,
    pub riscv_cpu_set_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_reset_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState, u32)>,
    pub riscv_cpu_get_mip: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_get_power_down: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> c_int>,
    pub riscv_cpu_get_misa: Option<unsafe extern "C" fn(*mut RISCVCPUState) -> u32>,
    pub riscv_cpu_flush_tlb_write_range_ram:
        Option<unsafe extern "C" fn(*mut RISCVCPUState, *mut u8, usize)>,
}

#[repr(C)]
pub struct RISCVCPUCommonState {
    pub class_ptr: *const RISCVCPUClass,
}

#[no_mangle]
pub unsafe extern "C" fn riscv_machine_interp__part2090(
    s1: *mut VirtMachine,
    max_exec_cycle: c_int,
) {
    let s = s1 as *mut RISCVMachine;
    let cpu = (*s).cpu_state;
    let c = (*(cpu as *mut RISCVCPUCommonState)).class_ptr;
    if let Some(interp) = (*c).riscv_cpu_interp {
        interp(cpu, max_exec_cycle);
    }
}
}

// ── `riscv_vm_send_key_event` · riscv_machine.c:1083-1090 · chunk 2091 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_vm_send_key_event(VirtMachine *s1, BOOL is_down,
//                                         uint16_t key_code)
//     {
//         RISCVMachine *s = (RISCVMachine *)s1;
//         if (s->keyboard_dev) {
//             virtio_input_send_key_event(s->keyboard_dev, is_down, key_code);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2091 {
// Port of riscv_machine.c: riscv_vm_send_key_event
use core::ffi::{c_char, c_int};

// Mirrors of the C types involved (only what this function touches).
pub type BOOL = c_int;

#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(*const core::ffi::c_void)>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
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
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut core::ffi::c_void, c_int, c_int)>,
    pub opaque: *mut core::ffi::c_void,
    pub irq_num: c_int,
}

pub type RISCVCPUState = core::ffi::c_void;
pub type PhysMemoryMap = core::ffi::c_void;

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut PhysMemoryMap,
    pub max_xlen: c_int,
    pub cpu_state: *mut RISCVCPUState,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

extern "C" {
    pub fn virtio_input_send_key_event(s: *mut VIRTIODevice, is_down: BOOL, key_code: u16)
        -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_vm_send_key_event(
    s1: *mut VirtMachine,
    is_down: BOOL,
    key_code: u16,
) {
    let s = s1 as *mut RISCVMachine;
    if s.is_null() {
        return;
    }
    let keyboard_dev = (*s).keyboard_dev;
    if !keyboard_dev.is_null() {
        virtio_input_send_key_event(keyboard_dev, is_down, key_code);
    }
}
}

// ── `if (s->keyboard_dev)` · riscv_machine.c:1087-1089 · chunk 2092 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->keyboard_dev) {
//             virtio_input_send_key_event(s->keyboard_dev, is_down, key_code);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2092 {
use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(p: *mut c_void)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(p: *const c_void) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(s: *mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(s: *mut VirtMachine, delay: c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(s: *mut VirtMachine, max_exec_cycle: c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(s: *mut VirtMachine) -> BOOL>,
    pub vm_send_mouse_event: Option<
        unsafe extern "C" fn(
            s1: *mut VirtMachine,
            dx: c_int,
            dy: c_int,
            dz: c_int,
            buttons: u32,
        ),
    >,
    pub vm_send_key_event:
        Option<unsafe extern "C" fn(s1: *mut VirtMachine, is_down: BOOL, key_code: u16)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    fn virtio_input_send_key_event(s: *mut c_void, is_down: BOOL, key_code: u16) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_vm_send_key_event__part2092(
    s: *mut RISCVMachine,
    is_down: BOOL,
    key_code: u16,
) {
    let s = &mut *s;
    if !s.keyboard_dev.is_null() {
        virtio_input_send_key_event(s.keyboard_dev, is_down, key_code);
    }
}
}

// ── `riscv_vm_mouse_is_absolute` · riscv_machine.c:1092-1095 · chunk 2093 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static BOOL riscv_vm_mouse_is_absolute(VirtMachine *s)
//     {
//         return TRUE;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2093 {
// riscv_vm_mouse_is_absolute: always reports absolute mouse positioning.
// BOOL is `typedef int`, TRUE is 1.
use core::ffi::c_int;

// The VirtMachine pointer is unused by this function; it is kept for ABI
// compatibility with the C declaration `BOOL riscv_vm_mouse_is_absolute(VirtMachine *s)`.
#[no_mangle]
pub extern "C" fn riscv_vm_mouse_is_absolute(_s: *mut core::ffi::c_void) -> c_int {
    1
}
}

// ── `return TRUE;` · riscv_machine.c:1094-1094 · chunk 2094 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2094 {
// Portion of riscv_vm_mouse_is_absolute: always reports an absolute mouse.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn riscv_vm_mouse_is_absolute__part2094(_s: *mut c_void) -> c_int {
    // TRUE
    1
}
}

// ── `riscv_vm_send_mouse_event` · riscv_machine.c:1097-1104 · chunk 2095 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void riscv_vm_send_mouse_event(VirtMachine *s1, int dx, int dy, int dz,
//                                           unsigned int buttons)
//     {
//         RISCVMachine *s = (RISCVMachine *)s1;
//         if (s->mouse_dev) {
//             virtio_input_send_mouse_event(s->mouse_dev, dx, dy, dz, buttons);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2095 {
// Rust rewrite of riscv_vm_send_mouse_event (riscv_machine.c)
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
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [IRQSignal; 32],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut c_void,
    pub mouse_dev: *mut c_void,
    pub virtio_count: c_int,
}

extern "C" {
    fn virtio_input_send_mouse_event(
        s: *mut c_void,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons: c_uint,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_vm_send_mouse_event(
    s1: *mut VirtMachine,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    let s = s1 as *mut RISCVMachine;
    if (*s).mouse_dev != core::ptr::null_mut() {
        virtio_input_send_mouse_event((*s).mouse_dev, dx, dy, dz, buttons);
    }
}
}

// ── `if (s->mouse_dev)` · riscv_machine.c:1101-1103 · chunk 2096 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->mouse_dev) {
//             virtio_input_send_mouse_event(s->mouse_dev, dx, dy, dz, buttons);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2096 {
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const core::ffi::c_void,
    pub net: *mut core::ffi::c_void,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut core::ffi::c_void,
    pub fb_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct RISCVMachine {
    pub common: VirtMachine,
    pub mem_map: *mut core::ffi::c_void,
    pub max_xlen: c_int,
    pub cpu_state: *mut core::ffi::c_void,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_start_time: u64,
    pub timecmp: u64,
    pub plic_pending_irq: u32,
    pub plic_served_irq: u32,
    pub plic_irq: [u8; 0],
    pub htif_tohost: u64,
    pub htif_fromhost: u64,
    pub keyboard_dev: *mut VIRTIODevice,
    pub mouse_dev: *mut VIRTIODevice,
    pub virtio_count: c_int,
}

extern "C" {
    fn virtio_input_send_mouse_event(
        s: *mut VIRTIODevice,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons: c_uint,
    ) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn riscv_vm_send_mouse_event__part2096(
    s: *mut RISCVMachine,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    if s.is_null() {
        return;
    }
    let dev = (*s).mouse_dev;
    if !dev.is_null() {
        virtio_input_send_mouse_event(dev, dx, dy, dz, buttons);
    }
}
}

