//! From `machine.h`.

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

