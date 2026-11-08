//! From `x86_cpu.c`.

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

