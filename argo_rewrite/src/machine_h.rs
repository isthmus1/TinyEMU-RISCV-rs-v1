//! From `machine.h`.

// ── `virt_machine_get_sleep_duration` · machine.h:151-154 · chunk 268 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int virt_machine_get_sleep_duration(VirtMachine *s, int delay)
//     {
//         return s->vmc->virt_machine_get_sleep_duration(s, delay);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk268 {
// Rust rewrite of the preprocessed C target:
//   static inline int virt_machine_get_sleep_duration(VirtMachine *s, int delay)
//   { return s->vmc->virt_machine_get_sleep_duration(s, delay); }
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Opaque forward declaration; only ever handled through pointers.
#[repr(C)]
pub struct VirtMachineParams {
    _opaque: [u8; 0],
}

// The C struct VirtMachine begins with `const VirtMachineClass *vmc;`
// which is the only field this function touches.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
}

// Layout of the leading members of struct VirtMachineClass up to the
// `virt_machine_get_sleep_duration` callback slot.  Unused members are
// pointer-sized placeholders; only the sizes/offsets matter here.
#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: *const c_void,
    pub virt_machine_init: *const c_void,
    pub virt_machine_end: *const c_void,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(s: *mut VirtMachine, delay: c_int) -> c_int>,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_get_sleep_duration(
    s: *mut VirtMachine,
    delay: c_int,
) -> c_int {
    let vmc: *const VirtMachineClass = (*s).vmc;
    match (*vmc).virt_machine_get_sleep_duration {
        Some(f) => f(s, delay),
        // Mirrors a NULL call through the class vtable being fatal; the C
        // code would fault, so report no sleep time rather than panic.
        None => 0,
    }
}
}

// ── `return s->vmc->virt_machine_get_sleep_duration(s, delay);` · machine.h:153-153 · chunk 269 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return s->vmc->virt_machine_get_sleep_duration(s, delay);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk269 {
use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
struct VirtMachineClass {
    machine_names: *const i8,
    virt_machine_set_defaults: extern "C" fn(*const c_void),
    virt_machine_init: extern "C" fn(*const c_void) -> *mut c_void,
    virt_machine_end: extern "C" fn(*mut c_void),
    virt_machine_get_sleep_duration: extern "C" fn(*mut c_void, c_int) -> c_int,
}

#[repr(C)]
struct VirtMachine {
    vmc: *const VirtMachineClass,
}

#[no_mangle]
pub extern "C" fn virt_machine_get_sleep_duration__part269(s: *mut VirtMachine, delay: c_int) -> c_int {
    unsafe {
        let vmc = (*s).vmc;
        let func = (*vmc).virt_machine_get_sleep_duration;
        func(s as *mut c_void, delay)
    }
}
}

// ── `virt_machine_interp` · machine.h:155-158 · chunk 270 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void virt_machine_interp(VirtMachine *s, int max_exec_cycle)
//     {
//         s->vmc->virt_machine_interp(s, max_exec_cycle);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk270 {
// Port of static inline void virt_machine_interp(VirtMachine *s, int max_exec_cycle)

use core::ffi::{c_char, c_int, c_void};

// BOOL is `typedef int BOOL;`
type Bool = c_int;

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut c_void)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(*const c_void) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> Bool>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, u32)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, Bool, u16)>,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_interp(s: *mut VirtMachine, max_exec_cycle: c_int) {
    if s.is_null() {
        return;
    }
    let vmc = (*s).vmc;
    if vmc.is_null() {
        return;
    }
    let f = (*vmc).virt_machine_interp;
    if let Some(interp) = f {
        interp(s, max_exec_cycle);
    }
}
}

// ── `s->vmc->virt_machine_interp(s, max_exec_cycle);` · machine.h:157-157 · chunk 271 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->vmc->virt_machine_interp(s, max_exec_cycle);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk271 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virt_machine_interp__part271(s: *mut core::ffi::c_void, max_exec_cycle: c_int) {
}
}

// ── `vm_mouse_is_absolute` · machine.h:159-162 · chunk 272 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL vm_mouse_is_absolute(VirtMachine *s)
//     {
//         return s->vmc->vm_mouse_is_absolute(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk272 {
// Rust rewrite of the C static inline helper:
//   static inline BOOL vm_mouse_is_absolute(VirtMachine *s)
//   {
//       return s->vmc->vm_mouse_is_absolute(s);
//   }
// BOOL is `typedef int BOOL;` in the C program.

use core::ffi::{c_char, c_int, c_void};

// Mirror of `struct VirtMachineClass` (machine.h), exact field order/types.
#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: *mut c_void,
    pub virt_machine_init: *mut c_void,
    pub virt_machine_end: *mut c_void,
    pub virt_machine_get_sleep_duration: *mut c_void,
    pub virt_machine_interp: *mut c_void,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(s: *mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event: *mut c_void,
    pub vm_send_key_event: *mut c_void,
}

// Mirror of `typedef struct VirtMachine { ... } VirtMachine;`, exact field order/types.
#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn vm_mouse_is_absolute(s: *mut VirtMachine) -> c_int {
    if s.is_null() {
        return 0;
    }
    let vmc = (*s).vmc;
    if vmc.is_null() {
        return 0;
    }
    match (*vmc).vm_mouse_is_absolute {
        Some(f) => f(s),
        None => 0,
    }
}
}

// ── `return s->vmc->vm_mouse_is_absolute(s);` · machine.h:161-161 · chunk 273 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     return s->vmc->vm_mouse_is_absolute(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk273 {
use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_uint;
use core::ffi::c_void;

#[repr(C)]
struct VirtMachine {
    vmc: *const VirtMachineClass,
}

#[repr(C)]
struct VirtMachineClass {
    _machine_names: *const c_char,
    _virt_machine_set_defaults: extern "C" fn(*const c_void) -> (),
    _virt_machine_init: extern "C" fn(*const c_void) -> *const VirtMachine,
    _virt_machine_end: extern "C" fn(*const VirtMachine) -> (),
    _virt_machine_get_sleep_duration: extern "C" fn(*const VirtMachine, c_int) -> c_int,
    _virt_machine_interp: extern "C" fn(*const VirtMachine, c_int) -> (),
    vm_mouse_is_absolute: extern "C" fn(*const VirtMachine) -> c_int,
    _vm_send_mouse_event: extern "C" fn(*const VirtMachine, c_int, c_int, c_int, c_uint) -> (),
    _vm_send_key_event: extern "C" fn(*const VirtMachine, c_int, c_uint) -> (),
}

#[no_mangle]
pub extern "C" fn vm_mouse_is_absolute__part273(s: *mut VirtMachine) -> c_int {
    unsafe {
        let s_const = s as *const VirtMachine;
        let vmc = (*s_const).vmc;
        let vmc_ref = &*vmc;
        (vmc_ref.vm_mouse_is_absolute)(s_const)
    }
}
}

// ── `vm_send_mouse_event` · machine.h:163-167 · chunk 274 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void vm_send_mouse_event(VirtMachine *s1, int dx, int dy, int dz,
//                                            unsigned int buttons)
//     {
//         s1->vmc->vm_send_mouse_event(s1, dx, dy, dz, buttons);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk274 {
// Rust rewrite of the static inline C helper `vm_send_mouse_event`
// (machine.h): forwards the mouse event to the machine class vtable entry.

use core::ffi::{c_char, c_int, c_uint, c_void};

// Opaque mirrors: only ever used behind a pointer here.
#[repr(C)]
pub struct VirtMachineParams {
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
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(p: *mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(p: *const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(s: *mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(s: *mut VirtMachine, delay: c_int) -> c_int>,
    pub virt_machine_interp:
        Option<unsafe extern "C" fn(s: *mut VirtMachine, max_exec_cycle: c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(s: *mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event: Option<
        unsafe extern "C" fn(s1: *mut VirtMachine, dx: c_int, dy: c_int, dz: c_int, buttons: c_uint),
    >,
    pub vm_send_key_event:
        Option<unsafe extern "C" fn(s1: *mut VirtMachine, is_down: c_int, key_code: u16)>,
}

#[no_mangle]
pub unsafe extern "C" fn vm_send_mouse_event(
    s1: *mut VirtMachine,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    unsafe {
        if s1.is_null() {
            return;
        }
        let vmc = (*s1).vmc;
        if vmc.is_null() {
            return;
        }
        let handler = (*vmc).vm_send_mouse_event;
        if let Some(f) = handler {
            f(s1, dx, dy, dz, buttons);
        }
    }
}

// Silence unused-import warnings if any field type is optimized away.
#[allow(dead_code)]
fn _unused(_p: *mut c_void) {}
}

// ── `s1->vmc->vm_send_mouse_event(s1, dx, dy, dz, buttons);` · machine.h:166-166 · chunk 275 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     s1->vmc->vm_send_mouse_event(s1, dx, dy, dz, buttons);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk275 {
use std::ffi::c_int;
use std::ffi::c_uint;
use std::os::raw::c_ushort;
use std::os::raw::c_char;
use std::os::raw::c_void;

// C struct definitions mirrored with #[repr(C)]
#[repr(C)]
struct VirtMachineClass {
    machine_names: *const c_char,
    virt_machine_set_defaults: extern "C" fn(*mut c_void),
    virt_machine_init: extern "C" fn(*mut c_void) -> *mut VirtMachine,
    virt_machine_end: extern "C" fn(*mut VirtMachine),
    virt_machine_get_sleep_duration: extern "C" fn(*mut VirtMachine, c_int) -> c_int,
    virt_machine_interp: extern "C" fn(*mut VirtMachine, c_int),
    vm_mouse_is_absolute: extern "C" fn(*mut VirtMachine) -> c_int,
    vm_send_mouse_event: extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint),
    vm_send_key_event: extern "C" fn(*mut VirtMachine, c_int, c_ushort),
}

#[repr(C)]
struct VirtMachine {
    vmc: *const VirtMachineClass,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

#[no_mangle]
pub extern "C" fn vm_send_mouse_event__part275(
    s1: *mut VirtMachine,
    dx: c_int,
    dy: c_int,
    dz: c_int,
    buttons: c_uint,
) {
    unsafe {
        let class = (*s1).vmc;
        let fn_ptr = (*class).vm_send_mouse_event;
        fn_ptr(s1, dx, dy, dz, buttons);
    }
}
}

// ── `vm_send_key_event` · machine.h:168-171 · chunk 276 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void vm_send_key_event(VirtMachine *s1, BOOL is_down, uint16_t key_code)
//     {
//         s1->vmc->vm_send_key_event(s1, is_down, key_code);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk276 {
// Port of: static inline void vm_send_key_event(VirtMachine *s1, BOOL is_down, uint16_t key_code)
// which dispatches to s1->vmc->vm_send_key_event(s1, is_down, key_code).

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut core::ffi::c_void,
    pub console_dev: *mut core::ffi::c_void,
    pub console: *mut core::ffi::c_void,
    pub fb_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const core::ffi::c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(*const core::ffi::c_void) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int) -> core::ffi::c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int)>,
    pub vm_mouse_is_absolute:
        Option<unsafe extern "C" fn(*mut VirtMachine) -> core::ffi::c_int>,
    pub vm_send_mouse_event: Option<
        unsafe extern "C" fn(
            *mut VirtMachine,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_uint,
        ),
    >,
    pub vm_send_key_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int, u16)>,
}

#[no_mangle]
pub unsafe extern "C" fn vm_send_key_event(
    s1: *mut VirtMachine,
    is_down: core::ffi::c_int,
    key_code: u16,
) {
    if s1.is_null() {
        return;
    }
    let vmc = (*s1).vmc;
    if vmc.is_null() {
        return;
    }
    if let Some(f) = (*vmc).vm_send_key_event {
        f(s1, is_down, key_code);
    }
}
}

// ── `s1->vmc->vm_send_key_event(s1, is_down, key_code);` · machine.h:170-170 · chunk 277 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     s1->vmc->vm_send_key_event(s1, is_down, key_code);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk277 {
use core::ffi::c_int;
use core::ffi::c_char;
use std::os::raw::c_void;
use core::ffi::c_ushort;
use std::os::raw::c_uint;

#[repr(C)]
struct VirtMachineClass {
    machine_names: *const c_char,
    virt_machine_set_defaults: extern "C" fn(*mut c_void),
    virt_machine_init: extern "C" fn(*const c_void) -> *mut VirtMachine,
    virt_machine_end: extern "C" fn(*mut VirtMachine),
    virt_machine_get_sleep_duration: extern "C" fn(*mut VirtMachine, c_int) -> c_int,
    virt_machine_interp: extern "C" fn(*mut VirtMachine, c_int),
    vm_mouse_is_absolute: extern "C" fn(*mut VirtMachine) -> c_int,
    vm_send_mouse_event: extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint),
    vm_send_key_event: extern "C" fn(*mut VirtMachine, c_int, c_ushort),
}

#[repr(C)]
struct VirtMachine {
    vmc: *const VirtMachineClass,
}

#[no_mangle]
pub extern "C" fn vm_send_key_event__part277(s1: *mut VirtMachine, is_down: c_int, key_code: c_ushort) {
    unsafe {
        let vmc_ptr = (*s1).vmc;
        let class = &*vmc_ptr;
        (class.vm_send_key_event)(s1, is_down, key_code);
    }
}
}

