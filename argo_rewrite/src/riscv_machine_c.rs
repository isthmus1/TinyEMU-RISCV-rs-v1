//! From `riscv_machine.c`.

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

