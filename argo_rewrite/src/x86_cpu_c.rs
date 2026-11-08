//! From `x86_cpu.c`.

// ── `x86_cpu_init` · x86_cpu.c:34-38 · chunk 3958 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     X86CPUState *x86_cpu_init(PhysMemoryMap *mem_map)
//     {
//         fprintf(
//                stderr
//                      , "x86 emulator is not supported\n");
//         exit(1);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3958 {
// Port of x86_cpu_init from x86_cpu.c
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn x86_cpu_init(_mem_map: *mut PhysMemoryMap) -> *mut X86CPUState {
    fprintf(
        stderr,
        b"x86 emulator is not supported\n\0".as_ptr() as *const c_char,
    );
    exit(1);
}
}

// ── `fprintf( stderr , "x86 emulator is not supported\n"); exit(1);` · x86_cpu.c:36-37 · chunk 3959 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fprintf(
//                stderr
//                      , "x86 emulator is not supported\n");
//         exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3959 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn x86_cpu_init__part3959() {
    unsafe {
        fprintf(
            stderr,
            b"x86 emulator is not supported\n\0".as_ptr() as *const c_char,
        );
        exit(1);
    }
}
}

// ── `x86_cpu_end` · x86_cpu.c:40-42 · chunk 3960 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_end(X86CPUState *s)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3960 {
// Port of the no-op x86_cpu_end stub. The body is empty in C and touches no
// state, so nothing needs to be declared beyond the ABI-identical signature.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn x86_cpu_end(_s: *mut c_void) {}
}

// ── `x86_cpu_interp` · x86_cpu.c:44-46 · chunk 3961 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_interp(X86CPUState *s, int max_cycles1)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3961 {
// x86_cpu_interp: stub implementation (the x86 emulator is not supported).
// The C definition is an empty body, so this rewrite does nothing with its
// arguments. X86CPUState is only ever used through an opaque pointer here.

use core::ffi::{c_char, c_int};

/// Opaque handle to the (unused) x86 CPU state.
#[repr(C)]
pub struct X86CPUState {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_interp(_s: *mut X86CPUState, _max_cycles1: c_int) {
    // Intentionally empty: matches the empty C function body.
    let _ = c_char::default();
}
}

// ── `x86_cpu_set_irq` · x86_cpu.c:48-50 · chunk 3962 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_irq(X86CPUState *s, BOOL set)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3962 {
// Empty stub: the C implementation of x86_cpu_set_irq is a no-op
// (the x86 emulator is not supported by this build).
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn x86_cpu_set_irq(_s: *mut c_void, _set: c_int) {}
}

// ── `x86_cpu_set_reg` · x86_cpu.c:52-54 · chunk 3963 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_reg(X86CPUState *s, int reg, uint32_t val)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3963 {
// Port of the stub x86_cpu_set_reg from x86_cpu.c (unsupported emulator build).
// The original C body is empty, so the Rust version does nothing; it only keeps
// the exported symbol and ABI identical.

use core::ffi::c_int;

/// Opaque mirror of the C `struct X86CPUState` (only ever used behind a pointer).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_set_reg(_s: *mut X86CPUState, _reg: c_int, _val: u32) {}
}

// ── `x86_cpu_get_reg` · x86_cpu.c:56-59 · chunk 3964 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t x86_cpu_get_reg(X86CPUState *s, int reg)
//     {
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3964 {
use core::ffi::{c_int, c_uint};

// Opaque x86 CPU state; the emulator stub never dereferences it.
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_get_reg(_s: *mut X86CPUState, _reg: c_int) -> c_uint {
    0
}
}

// ── `return 0;` · x86_cpu.c:58-58 · chunk 3965 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3965 {
// Ported piece of x86_cpu_get_reg: the `return 0;` statement.
use core::ffi::c_int;

/// Opaque mirror of the C `X86CPUState` type (never dereferenced here).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_get_reg__part3965(
    _s: *mut X86CPUState,
    _reg: c_int,
) -> u32 {
    0
}
}

// ── `x86_cpu_set_seg` · x86_cpu.c:61-63 · chunk 3966 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_seg(X86CPUState *s, int seg, const X86CPUSeg *sd)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3966 {
// Port of x86_cpu_set_seg from x86_cpu.c (lines 61-63).
// The original C definition is an empty stub (the x86 emulator is not
// supported in this build), so the Rust version performs no work.

use core::ffi::{c_int, c_void};

/// Mirror of the C `X86CPUSeg` struct (x86_cpu.h lines 44-49).
#[repr(C)]
pub struct X86CPUSeg {
    pub sel: u16,
    pub flags: u16,
    pub base: u32,
    pub limit: u32,
}

/// `void x86_cpu_set_seg(X86CPUState *s, int seg, const X86CPUSeg *sd)`
///
/// `X86CPUState` is an incomplete type in C, so it is represented as an
/// opaque pointer here. The body is intentionally empty, matching the
/// original definition.
#[no_mangle]
pub extern "C" fn x86_cpu_set_seg(
    _s: *mut c_void,
    _seg: c_int,
    _sd: *const X86CPUSeg,
) {
}
}

// ── `x86_cpu_set_get_hard_intno` · x86_cpu.c:65-69 · chunk 3967 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_get_hard_intno(X86CPUState *s,
//                                     int (*get_hard_intno)(void *opaque),
//                                     void *opaque)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3967 {
// Port of x86_cpu_set_get_hard_intno() from x86_cpu.c (stub, empty body).

use core::ffi::{c_int, c_void};

/// Opaque mirror of the C `X86CPUState` (never dereferenced here).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

/// C type: `int (*get_hard_intno)(void *opaque)`
pub type GetHardIntnoFn = unsafe extern "C" fn(opaque: *mut c_void) -> c_int;

#[no_mangle]
pub extern "C" fn x86_cpu_set_get_hard_intno(
    s: *mut X86CPUState,
    get_hard_intno: Option<GetHardIntnoFn>,
    opaque: *mut c_void,
) {
    // Original C body is empty.
    let _ = (s, get_hard_intno, opaque);
}
}

// ── `x86_cpu_set_get_tsc` · x86_cpu.c:71-75 · chunk 3968 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_get_tsc(X86CPUState *s,
//                              uint64_t (*get_tsc)(void *opaque),
//                              void *opaque)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3968 {
use core::ffi::c_void;

// Opaque: struct X86CPUState is only declared (incomplete) in this TU.
pub type X86CPUState = c_void;

// typedef uint64_t (*get_tsc_t)(void *opaque)
pub type GetTscFn = Option<unsafe extern "C" fn(opaque: *mut c_void) -> u64>;

#[no_mangle]
pub extern "C" fn x86_cpu_set_get_tsc(
    s: *mut X86CPUState,
    get_tsc: GetTscFn,
    opaque: *mut c_void,
) {
    let _ = s;
    let _ = get_tsc;
    let _ = opaque;
}
}

// ── `x86_cpu_set_port_io` · x86_cpu.c:77-81 · chunk 3969 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_set_port_io(X86CPUState *s,
//                              DeviceReadFunc *port_read, DeviceWriteFunc *port_write,
//                              void *opaque)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3969 {
// Port of x86_cpu_set_port_io from x86_cpu.c (lines 77-81).
// The C definition has an empty body, so this is a no-op stub.

use core::ffi::{c_int, c_uint, c_void};

/// Opaque X86CPUState; only ever used through a pointer here.
pub enum X86CPUState {}

/// DeviceWriteFunc(void *opaque, uint32_t offset, uint32_t val, int size_log2)
pub type DeviceWriteFunc =
    unsafe extern "C" fn(opaque: *mut c_void, offset: c_uint, val: c_uint, size_log2: c_int);

/// DeviceReadFunc(void *opaque, uint32_t offset, int size_log2)
pub type DeviceReadFunc = unsafe extern "C" fn(opaque: *mut c_void, offset: c_uint, size_log2: c_int) -> c_uint;

#[no_mangle]
pub unsafe extern "C" fn x86_cpu_set_port_io(
    _s: *mut X86CPUState,
    _port_read: Option<DeviceReadFunc>,
    _port_write: Option<DeviceWriteFunc>,
    _opaque: *mut c_void,
) {
}
}

// ── `x86_cpu_get_cycles` · x86_cpu.c:83-86 · chunk 3970 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t x86_cpu_get_cycles(X86CPUState *s)
//     {
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3970 {
// Port of x86_cpu_get_cycles from x86_cpu.c.
// The x86 emulator stub ignores its state argument and reports zero cycles.

use core::ffi::c_long;

/// Opaque forward declaration of `struct X86CPUState` (never dereferenced).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_get_cycles(s: *mut X86CPUState) -> c_long {
    let _ = s;
    0
}
}

// ── `return 0;` · x86_cpu.c:85-85 · chunk 3971 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3971 {
// Piece of x86_cpu_get_cycles: placeholder implementation that reports zero cycles.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn x86_cpu_get_cycles__part3971(_s: *mut c_void) -> i64 {
    0
}
}

// ── `x86_cpu_get_power_down` · x86_cpu.c:88-91 · chunk 3972 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL x86_cpu_get_power_down(X86CPUState *s)
//     {
//         return FALSE;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3972 {
// Rewrite of x86_cpu_get_power_down from x86_cpu.c
#![allow(non_camel_case_types)]

/// Opaque mirror of the C `X86CPUState` (incomplete type in the header).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

/// `typedef int BOOL;`
pub type BOOL = core::ffi::c_int;

const FALSE: BOOL = 0;

#[no_mangle]
pub extern "C" fn x86_cpu_get_power_down(_s: *mut X86CPUState) -> BOOL {
    FALSE
}
}

// ── `return FALSE;` · x86_cpu.c:90-90 · chunk 3973 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return FALSE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3973 {
// Ported piece: `return FALSE;` from x86_cpu_get_power_down.
// FALSE is the enum constant 0; BOOL is `int`.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn x86_cpu_get_power_down__part3973() -> c_int {
    // FALSE == 0
    0
}
}

// ── `x86_cpu_flush_tlb_write_range_ram` · x86_cpu.c:93-96 · chunk 3974 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void x86_cpu_flush_tlb_write_range_ram(X86CPUState *s,
//                                            uint8_t *ram_ptr, size_t ram_size)
//     {
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3974 {
// Port of x86_cpu_flush_tlb_write_range_ram from x86_cpu.c.
// The original C body is empty; it only needs an ABI-identical exported wrapper.

use core::ffi::c_void;

/// Opaque mirror of the C `X86CPUState` (never dereferenced here).
#[repr(C)]
pub struct X86CPUState {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn x86_cpu_flush_tlb_write_range_ram(
    _s: *mut X86CPUState,
    _ram_ptr: *mut u8,
    _ram_size: usize,
) {
    let _ = core::mem::size_of::<*mut c_void>();
}
}

