//! From `pckbd.c`.

// ── `printf("system_reset_request\n"); exit(1);` · pckbd.c:102-103 · chunk 1580 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     printf("system_reset_request\n");
//         exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1580 {
// Rust counterpart of the qemu_system_reset_request tail:
//   printf("system_reset_request\n");
//   exit(1);
use core::ffi::{c_char, c_int};

extern "C" {
    fn printf(__format: *const c_char, ...) -> c_int;
    fn exit(__status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn qemu_system_reset_request__part1580() {
    unsafe {
        printf(b"system_reset_request\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}
}

// ── `return 1;` · pckbd.c:113-113 · chunk 1583 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1583 {
// Ported piece of ioport_get_a20: returns the constant A20 state 1.
#[no_mangle]
pub extern "C" fn ioport_get_a20__part1583() -> core::ffi::c_int {
    1
}
}

// ── `int irq_kbd_level, irq_mouse_level; irq_kbd_level = 0; irq_mouse_level = 0; s->status &= ~(0x01…` · pckbd.c:121-125 · chunk 1585 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int irq_kbd_level, irq_mouse_level;
//     
//         irq_kbd_level = 0;
//         irq_mouse_level = 0;
//         s->status &= ~(0x01 | 0x20);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1585 {
// Piece of kbd_update_irq: initializes the two IRQ level locals to 0 and
// clears the kbd/mouse IRQ bits (0x01 | 0x20) in the status register.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut c_void,
    pub irq_mouse: *mut c_void,
}

#[no_mangle]
pub extern "C" fn kbd_update_irq__part1585(
    s: *mut KBDState,
    irq_kbd_level: *mut c_int,
    irq_mouse_level: *mut c_int,
) {
    unsafe {
        if !irq_kbd_level.is_null() {
            *irq_kbd_level = 0;
        }
        if !irq_mouse_level.is_null() {
            *irq_mouse_level = 0;
        }
        if !s.is_null() {
            (*s).status &= !(0x01u8 | 0x20u8);
        }
    }
}
}

// ── `if (s->pending)` · pckbd.c:126-138 · chunk 1586 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->pending) {
//             s->status |= 0x01;
//     
//             if (s->pending == 2) {
//                 s->status |= 0x20;
//                 if (s->mode & 0x02)
//                     irq_mouse_level = 1;
//             } else {
//                 if ((s->mode & 0x01) &&
//                     !(s->mode & 0x10))
//                     irq_kbd_level = 1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1586 {
// Ported piece of kbd_update_irq from pckbd.c lines 126-138.
// Reads s->pending and s->mode, updates s->status, and may set the
// irq level locals owned by the caller.

use core::ffi::c_int;

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut core::ffi::c_void,
    pub mouse: *mut core::ffi::c_void,
    pub irq_kbd: *mut core::ffi::c_void,
    pub irq_mouse: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn kbd_update_irq__part1586(
    s: *mut KBDState,
    irq_kbd_level: *mut c_int,
    irq_mouse_level: *mut c_int,
) {
    unsafe {
        let s = &mut *s;
        if s.pending != 0 {
            s.status |= 0x01;

            if s.pending == 2 {
                s.status |= 0x20;
                if (s.mode & 0x02) != 0 {
                    *irq_mouse_level = 1;
                }
            } else if (s.mode & 0x01) != 0 && (s.mode & 0x10) == 0 {
                *irq_kbd_level = 1;
            }
        }
    }
}
}

// ── `set_irq(s->irq_kbd, irq_kbd_level); set_irq(s->irq_mouse, irq_mouse_level);` · pckbd.c:139-140 · chunk 1587 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     set_irq(s->irq_kbd, irq_kbd_level);
//         set_irq(s->irq_mouse, irq_mouse_level);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1587 {
use core::ffi::{c_int, c_void};

// Mirror of the C `IRQSignal` struct (fields in declaration order).
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Mirror of the C `KBDState` struct (fields in declaration order).
// PS2KbdState / PS2MouseState are only ever touched as pointers here.
#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

// Ported from the C `static inline void set_irq(IRQSignal *irq, int level)`.
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[no_mangle]
pub extern "C" fn kbd_update_irq__part1587(
    s: *mut KBDState,
    irq_kbd_level: c_int,
    irq_mouse_level: c_int,
) {
    unsafe {
        set_irq((*s).irq_kbd, irq_kbd_level);
        set_irq((*s).irq_mouse, irq_mouse_level);
    }
}
}

// ── `if (level) … else` · pckbd.c:147-150 · chunk 1589 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (level)
//             s->pending |= 1;
//         else
//             s->pending &= ~1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1589 {
// Piece of kbd_update_kbd_irq: update the keyboard pending bit in KBDState.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut c_void,
    pub irq_mouse: *mut c_void,
}

#[no_mangle]
pub extern "C" fn kbd_update_kbd_irq__part1589(s: *mut KBDState, level: c_int) {
    if !s.is_null() {
        unsafe {
            if level != 0 {
                (*s).pending |= 1u8;
            } else {
                (*s).pending &= !1u8;
            }
        }
    }
}
}

// ── `kbd_update_irq(s);` · pckbd.c:151-151 · chunk 1590 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     kbd_update_irq(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1590 {
// Port of the TARGET `kbd_update_irq(s);` inside kbd_update_kbd_irq.
// kbd_update_irq is a static C function with no linker symbol, so its logic is
// reproduced here (together with the `set_irq` inline helper).

use core::ffi::{c_int, c_void};

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kbd_update_kbd_irq__part1590(s: *mut KBDState) {
    let mut irq_kbd_level: c_int = 0;
    let mut irq_mouse_level: c_int = 0;

    (*s).status &= !(0x01u8 | 0x20u8);
    if (*s).pending != 0 {
        (*s).status |= 0x01u8;

        if (*s).pending == 2 {
            (*s).status |= 0x20u8;
            if ((*s).mode & 0x02u8) != 0 {
                irq_mouse_level = 1;
            }
        } else {
            if ((*s).mode & 0x01u8) != 0 && ((*s).mode & 0x10u8) == 0 {
                irq_kbd_level = 1;
            }
        }
    }
    set_irq((*s).irq_kbd, irq_kbd_level);
    set_irq((*s).irq_mouse, irq_mouse_level);
}
}

// ── `if (level) … else` · pckbd.c:158-161 · chunk 1592 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (level)
//             s->pending |= 2;
//         else
//             s->pending &= ~2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1592 {
// Portion of kbd_update_aux_irq: update the mouse/aux pending bit.
use core::ffi::c_int;

#[repr(C)]
pub struct PS2KbdState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut PS2KbdState,
    pub mouse: *mut PS2MouseState,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

#[no_mangle]
pub extern "C" fn kbd_update_aux_irq__part1592(s: *mut KBDState, level: c_int) {
    if s.is_null() {
        return;
    }
    unsafe {
        if level != 0 {
            (*s).pending |= 2;
        } else {
            (*s).pending &= !2u8;
        }
    }
}
}

// ── `kbd_update_irq(s);` · pckbd.c:162-162 · chunk 1593 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     kbd_update_irq(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1593 {
// Port of kbd_update_irq(s) inlined: the C helper is `static`, so its logic
// lives here as a private Rust fn. `set_irq` is a `static inline` helper and is
// likewise reproduced here.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};

// Opaque types only ever used behind pointers.
#[repr(C)]
pub struct PS2KbdState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

// typedef struct { SetIRQFunc *set_irq; void *opaque; int irq_num; } IRQSignal;
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// struct KBDState { uint8_t write_cmd, status, mode, pending;
//                   PS2KbdState *kbd; PS2MouseState *mouse;
//                   IRQSignal *irq_kbd, *irq_mouse; }
#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut PS2KbdState,
    pub mouse: *mut PS2MouseState,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let r: &IRQSignal = &*irq;
    if let Some(f) = r.set_irq {
        f(r.opaque, r.irq_num, level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kbd_update_aux_irq__part1593(s: *mut KBDState) {
    let mut irq_kbd_level: c_int = 0;
    let mut irq_mouse_level: c_int = 0;

    let st: &mut KBDState = &mut *s;

    st.status &= !(0x01u8 | 0x20u8);
    if st.pending != 0 {
        st.status |= 0x01u8;

        if st.pending == 2 {
            st.status |= 0x20u8;
            if (st.mode & 0x02u8) != 0 {
                irq_mouse_level = 1;
            }
        } else if (st.mode & 0x01u8) != 0 && (st.mode & 0x10u8) == 0 {
            irq_kbd_level = 1;
        }
    }

    set_irq(st.irq_kbd, irq_kbd_level);
    set_irq(st.irq_mouse, irq_mouse_level);
}
}

// ── `KBDState *s = opaque; int val; val = s->status; return val;` · pckbd.c:167-173 · chunk 1595 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     KBDState *s = opaque;
//         int val;
//         val = s->status;
//     
//     
//     
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1595 {
use core::ffi::c_void;

#[repr(C)]
pub struct KBDState {
    write_cmd: u8,
    status: u8,
    mode: u8,
    pending: u8,
    kbd: *mut c_void,
    mouse: *mut c_void,
    irq_kbd: *mut c_void,
    irq_mouse: *mut c_void,
}

#[no_mangle]
pub extern "C" fn kbd_read_status__part1595(opaque: *mut c_void) -> u32 {
    let val: i32 = unsafe { (*(opaque as *const KBDState)).status as i32 };
    val as u32
}
}

// ── `if (aux) … else` · pckbd.c:178-181 · chunk 1597 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (aux)
//             ps2_queue(s->mouse, b);
//         else
//             ps2_queue(s->kbd, b);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1597 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct KBDState {
    write_cmd: u8,
    status: u8,
    mode: u8,
    pending: u8,
    kbd: *mut c_void,
    mouse: *mut c_void,
    irq_kbd: *mut c_void,
    irq_mouse: *mut c_void,
}

extern "C" {
    fn ps2_queue(s: *mut c_void, b: c_int);
}

#[no_mangle]
pub extern "C" fn kbd_queue__part1597(s: *mut c_void, b: c_int, aux: c_int) {
    unsafe {
        let st = s as *mut KBDState;
        if aux != 0 {
            ps2_queue((*st).mouse, b);
        } else {
            ps2_queue((*st).kbd, b);
        }
    }
}
}

// ── `switch (val)` · pckbd.c:192-254 · chunk 1599 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(val) {
//         case 0x20:
//             kbd_queue(s, s->mode, 1);
//             break;
//         case 0x60:
//         case 0xD2:
//         case 0xD3:
//         case 0xD4:
//         case 0xD1:
//             s->write_cmd = val;
//             break;
//         case 0xA7:
//             s->mode |= 0x20;
//             break;
//         case 0xA8:
//             s->mode &= ~0x20;
//             break;
//         case 0xA9:
//             kbd_queue(s, 0x00, 0);
//             break;
//         case 0xAA:
//             s->status |= 0x04;
//             kbd_queue(s, 0x55, 0);
//             break;
//         case 0xAB:
//             kbd_queue(s, 0x00, 0);
//             break;
//         case 0xAD:
//             s->mode |= 0x10;
//             kbd_update_irq(s);
//             break;
//         case 0xAE:
//             s->mode &= ~0x10;
//             kbd_update_irq(s);
//             break;
//         case 0xC0:
//             kbd_queue(s, 0x00, 0);
//             break;
//         case 0xD0:
//     
//             val = 0x01 | (ioport_get_a20() << 1);
//             if (s->status & 0x01)
//                 val |= 0x10;
//             if (s->status & 0x20)
//                 val |= 0x20;
//             kbd_queue(s, val, 0);
//             break;
//         case 0xDF:
//             ioport_set_a20(1);
//             break;
//         case 0xDD:
//             ioport_set_a20(0);
//             break;
//         case 0xFE:
//             qemu_system_reset_request();
//             break;
//         case 0xff:
//     
//             break;
//         default:
//             fprintf(
//                    stderr
//                          , "qemu: unsupported keyboard cmd=0x%02x\n", val);
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1599 {
// Port of the switch statement inside kbd_write_command (pckbd.c).
use core::ffi::{c_char, c_int, c_uint, c_void};

// Mirror of the C `struct KBDState` (only fields that are touched here, but the
// layout must match the real struct exactly: offsets of kbd/mouse/irq_* matter).
#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut c_void,
    pub irq_mouse: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Opaque FILE: only ever used through pointers.
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

// static void set_irq(IRQSignal *irq, int level)  -- static inline in C
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let f = (*irq).set_irq.unwrap_unchecked();
    f((*irq).opaque, (*irq).irq_num, level);
}

// static void qemu_system_reset_request(void)  -- static in C, no linker symbol
unsafe fn qemu_system_reset_request() {
    printf(b"system_reset_request\n\0".as_ptr() as *const c_char);
    exit(1);
}

// static void ioport_set_a20(int val)  -- static in C
unsafe fn ioport_set_a20(_val: c_int) {}

// static int ioport_get_a20(void)  -- static in C
unsafe fn ioport_get_a20() -> c_int {
    1
}

// static void kbd_update_irq(KBDState *s)  -- static in C
unsafe fn kbd_update_irq(s: *mut KBDState) {
    let mut irq_kbd_level: c_int = 0;
    let mut irq_mouse_level: c_int = 0;

    (*s).status &= !(0x01u8 | 0x20u8);
    if (*s).pending != 0 {
        (*s).status |= 0x01;

        if (*s).pending == 2 {
            (*s).status |= 0x20;
            if ((*s).mode & 0x02) != 0 {
                irq_mouse_level = 1;
            }
        } else {
            if ((*s).mode & 0x01) != 0 && ((*s).mode & 0x10) == 0 {
                irq_kbd_level = 1;
            }
        }
    }
    set_irq((*s).irq_kbd as *mut IRQSignal, irq_kbd_level);
    set_irq((*s).irq_mouse as *mut IRQSignal, irq_mouse_level);
}

// static void kbd_queue(KBDState *s, int b, int aux)  -- static in C
unsafe fn kbd_queue(s: *mut KBDState, b: c_int, aux: c_int) {
    if aux != 0 {
        ps2_queue((*s).mouse, b);
    } else {
        ps2_queue((*s).kbd, b);
    }
}

#[no_mangle]
pub unsafe extern "C" fn kbd_write_command__part1599(s: *mut KBDState, val_in: u32) -> u32 {
    // `val` is reassigned in the 0xD0 case and used by the default case; the
    // final value is handed back so the C caller keeps it.
    let mut val: u32 = val_in;

    match val {
        0x20 => {
            kbd_queue(s, (*s).mode as c_int, 1);
        }
        0x60 | 0xD2 | 0xD3 | 0xD4 | 0xD1 => {
            (*s).write_cmd = val as u8;
        }
        0xA7 => {
            (*s).mode |= 0x20;
        }
        0xA8 => {
            (*s).mode &= !0x20u8;
        }
        0xA9 => {
            kbd_queue(s, 0x00, 0);
        }
        0xAA => {
            (*s).status |= 0x04;
            kbd_queue(s, 0x55, 0);
        }
        0xAB => {
            kbd_queue(s, 0x00, 0);
        }
        0xAD => {
            (*s).mode |= 0x10;
            kbd_update_irq(s);
        }
        0xAE => {
            (*s).mode &= !0x10u8;
            kbd_update_irq(s);
        }
        0xC0 => {
            kbd_queue(s, 0x00, 0);
        }
        0xD0 => {
            val = 0x01u32 | ((ioport_get_a20() as c_uint) << 1);
            if ((*s).status & 0x01) != 0 {
                val |= 0x10;
            }
            if ((*s).status & 0x20) != 0 {
                val |= 0x20;
            }
            kbd_queue(s, val as c_int, 0);
        }
        0xDF => {
            ioport_set_a20(1);
        }
        0xDD => {
            ioport_set_a20(0);
        }
        0xFE => {
            qemu_system_reset_request();
        }
        0xFF => {}
        _ => {
            fprintf(
                stderr,
                b"qemu: unsupported keyboard cmd=0x%02x\n\0".as_ptr() as *const c_char,
                val,
            );
        }
    }

    val
}
}

// ── `if (s->pending == 2) … else` · pckbd.c:261-264 · chunk 1601 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->pending == 2)
//             val = ps2_read_data(s->mouse);
//         else
//             val = ps2_read_data(s->kbd);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1601 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut c_void,
    pub mouse: *mut c_void,
    pub irq_kbd: *mut c_void,
    pub irq_mouse: *mut c_void,
}

extern "C" {
    fn ps2_read_data(arg: *mut c_void) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn kbd_read_data__part1601(s: *mut KBDState, val: *mut u32) {
    if (*s).pending == 2 {
        *val = ps2_read_data((*s).mouse);
    } else {
        *val = ps2_read_data((*s).kbd);
    }
}
}

// ── `return val;` · pckbd.c:268-268 · chunk 1602 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1602 {
// Ported piece of kbd_read_data: simply returns the value read from the
// PS/2 mouse or keyboard data register.

#[no_mangle]
pub extern "C" fn kbd_read_data__part1602(val: u32) -> u32 {
    val
}
}

// ── `switch (s->write_cmd)` · pckbd.c:279-306 · chunk 1604 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->write_cmd) {
//         case 0:
//             ps2_write_keyboard(s->kbd, val);
//             break;
//         case 0x60:
//             s->mode = val;
//             ps2_keyboard_set_translation(s->kbd, (s->mode & 0x40) != 0);
//     
//             kbd_update_irq(s);
//             break;
//         case 0xD2:
//             kbd_queue(s, val, 0);
//             break;
//         case 0xD3:
//             kbd_queue(s, val, 1);
//             break;
//         case 0xD1:
//             ioport_set_a20((val >> 1) & 1);
//             if (!(val & 1)) {
//                 qemu_system_reset_request();
//             }
//             break;
//         case 0xD4:
//             ps2_write_mouse(s->mouse, val);
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1604 {
// Rust port of the switch(s->write_cmd) part of kbd_write_data.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct KBDState {
    write_cmd: u8,
    status: u8,
    mode: u8,
    pending: u8,
    kbd: *mut c_void,
    mouse: *mut c_void,
    irq_kbd: *mut IRQSignal,
    irq_mouse: *mut IRQSignal,
}

extern "C" {
    fn ps2_write_mouse(opaque: *mut c_void, val: c_int);
    fn ps2_write_keyboard(opaque: *mut c_void, val: c_int);
    fn ps2_queue(opaque: *mut c_void, b: c_int);
    fn ps2_keyboard_set_translation(opaque: *mut c_void, mode: c_int);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

// port of the static inline set_irq() from the translation unit
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let irq = &*irq;
    if let Some(f) = irq.set_irq {
        f(irq.opaque, irq.irq_num, level);
    }
}

// port of static void qemu_system_reset_request(void)
unsafe fn qemu_system_reset_request() {
    printf(b"system_reset_request\n\0".as_ptr() as *const c_char);
    exit(1);
}

// port of static void ioport_set_a20(int val) (empty in the original)
unsafe fn ioport_set_a20(_val: c_int) {}

// port of static void kbd_queue(KBDState *s, int b, int aux)
unsafe fn kbd_queue(s: *mut KBDState, b: c_int, aux: c_int) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    if aux != 0 {
        ps2_queue(s.mouse, b);
    } else {
        ps2_queue(s.kbd, b);
    }
}

// port of static void kbd_update_irq(KBDState *s)
unsafe fn kbd_update_irq(s: *mut KBDState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    let mut irq_kbd_level: c_int = 0;
    let mut irq_mouse_level: c_int = 0;
    s.status &= !(0x01u8 | 0x20u8);
    if s.pending != 0 {
        s.status |= 0x01u8;
        if s.pending == 2 {
            s.status |= 0x20u8;
            if (s.mode & 0x02) != 0 {
                irq_mouse_level = 1;
            }
        } else if (s.mode & 0x01) != 0 && (s.mode & 0x10) == 0 {
            irq_kbd_level = 1;
        }
    }
    set_irq(s.irq_kbd, irq_kbd_level);
    set_irq(s.irq_mouse, irq_mouse_level);
}

#[no_mangle]
pub unsafe extern "C" fn kbd_write_data__part1604(s: *mut KBDState, val: u32) {
    if s.is_null() {
        return;
    }
    let st = &mut *s;
    match st.write_cmd {
        0 => {
            ps2_write_keyboard(st.kbd, val as c_int);
        }
        0x60 => {
            st.mode = val as u8;
            ps2_keyboard_set_translation(st.kbd, ((st.mode & 0x40) != 0) as c_int);
            kbd_update_irq(s);
        }
        0xD2 => {
            kbd_queue(s, val as c_int, 0);
        }
        0xD3 => {
            kbd_queue(s, val as c_int, 1);
        }
        0xD1 => {
            ioport_set_a20(((val >> 1) & 1) as c_int);
            if (val & 1) == 0 {
                qemu_system_reset_request();
            }
        }
        0xD4 => {
            ps2_write_mouse(st.mouse, val as c_int);
        }
        _ => {}
    }
}
}

// ── `s->write_cmd = 0;` · pckbd.c:307-307 · chunk 1605 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->write_cmd = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1605 {
// Port of the write_cmd reset at the end of kbd_write_data.
// Mirrors struct KBDState from pckbd.c (only the layout matters here).

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut core::ffi::c_void,
    pub mouse: *mut core::ffi::c_void,
    pub irq_kbd: *mut core::ffi::c_void,
    pub irq_mouse: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn kbd_write_data__part1605(s: *mut KBDState) {
    unsafe {
        if !s.is_null() {
            (*s).write_cmd = 0;
        }
    }
}
}

// ── `KBDState *s = opaque; s->mode = 0x01 | 0x02; s->status = 0x08 | 0x10;` · pckbd.c:312-315 · chunk 1607 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     KBDState *s = opaque;
//     
//         s->mode = 0x01 | 0x02;
//         s->status = 0x08 | 0x10;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1607 {
// Port of the body of kbd_reset(): stores the default mode/status of the i8042.
#[repr(C)]
pub struct PS2KbdState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PS2MouseState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut PS2KbdState,
    pub mouse: *mut PS2MouseState,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

#[no_mangle]
pub unsafe extern "C" fn kbd_reset__part1607(opaque: *mut core::ffi::c_void) {
    let s = opaque as *mut KBDState;
    (*s).mode = (0x01u8) | (0x02u8);
    (*s).status = (0x08u8) | (0x10u8);
}
}

// ── `KBDState *s; s = mallocz(sizeof(*s)); s->irq_kbd = kbd_irq; s->irq_mouse = mouse_irq; kbd_reset…` · pckbd.c:323-341 · chunk 1609 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     KBDState *s;
//     
//         s = mallocz(sizeof(*s));
//     
//         s->irq_kbd = kbd_irq;
//         s->irq_mouse = mouse_irq;
//     
//         kbd_reset(s);
//         cpu_register_device(port_map, io_base, 1, s, kbd_read_data, kbd_write_data,
//                             (1 << 0));
//         cpu_register_device(port_map, io_base + 4, 1, s, kbd_read_status, kbd_write_command,
//                             (1 << 0));
//     
//         s->kbd = ps2_kbd_init(kbd_update_kbd_irq, s);
//         s->mouse = ps2_mouse_init(kbd_update_aux_irq, s);
//     
//         *pkbd = s->kbd;
//         *pmouse = s->mouse;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1609 {
// Rust port of the i8042_init body (pckbd.c lines 323-341) and of the
// static helpers it needs (they have no linker symbols in C).

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PS2KbdState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct KBDState {
    pub write_cmd: u8,
    pub status: u8,
    pub mode: u8,
    pub pending: u8,
    pub kbd: *mut PS2KbdState,
    pub mouse: *mut PS2MouseState,
    pub irq_kbd: *mut IRQSignal,
    pub irq_mouse: *mut IRQSignal,
}

type DevReadFunc = extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DevWriteFunc = extern "C" fn(*mut c_void, u32, u32, c_int);
type UpdateIrqFunc = extern "C" fn(*mut c_void, c_int);

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;

    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DevReadFunc>,
        write_func: Option<DevWriteFunc>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;

    fn ps2_kbd_init(update_irq: Option<UpdateIrqFunc>, update_arg: *mut c_void)
        -> *mut PS2KbdState;
    fn ps2_mouse_init(
        update_irq: Option<UpdateIrqFunc>,
        update_arg: *mut c_void,
    ) -> *mut PS2MouseState;

    fn ps2_queue(s: *mut c_void, b: c_int);
    fn ps2_write_mouse(s: *mut c_void, val: c_int);
    fn ps2_write_keyboard(s: *mut c_void, val: c_int);
    fn ps2_read_data(s: *mut c_void) -> u32;
    fn ps2_keyboard_set_translation(opaque: *mut c_void, mode: c_int);

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn exit(code: c_int) -> !;

    static mut stderr: *mut c_void;
}

fn get_stderr() -> *mut c_void {
    unsafe { core::ptr::addr_of!(stderr).read() }
}

fn set_irq(irq: *mut IRQSignal, level: c_int) {
    unsafe {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

fn qemu_system_reset_request() {
    unsafe {
        printf(b"system_reset_request\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

fn ioport_set_a20(_val: c_int) {}

fn ioport_get_a20() -> c_int {
    1
}

fn kbd_reset(s: *mut KBDState) {
    unsafe {
        (*s).mode = 0x01 | 0x02;
        (*s).status = 0x08 | 0x10;
    }
}

fn kbd_update_irq(s: *mut KBDState) {
    unsafe {
        let mut irq_kbd_level: c_int = 0;
        let mut irq_mouse_level: c_int = 0;
        (*s).status &= !(0x01u8 | 0x20u8);
        if (*s).pending != 0 {
            (*s).status |= 0x01;

            if (*s).pending == 2 {
                (*s).status |= 0x20;
                if (*s).mode & 0x02 != 0 {
                    irq_mouse_level = 1;
                }
            } else if (*s).mode & 0x01 != 0 && (*s).mode & 0x10 == 0 {
                irq_kbd_level = 1;
            }
        }
        set_irq((*s).irq_kbd, irq_kbd_level);
        set_irq((*s).irq_mouse, irq_mouse_level);
    }
}

extern "C" fn kbd_update_kbd_irq(opaque: *mut c_void, level: c_int) {
    unsafe {
        let s = opaque as *mut KBDState;
        if level != 0 {
            (*s).pending |= 1;
        } else {
            (*s).pending &= !1u8;
        }
        kbd_update_irq(s);
    }
}

extern "C" fn kbd_update_aux_irq(opaque: *mut c_void, level: c_int) {
    unsafe {
        let s = opaque as *mut KBDState;
        if level != 0 {
            (*s).pending |= 2;
        } else {
            (*s).pending &= !2u8;
        }
        kbd_update_irq(s);
    }
}

extern "C" fn kbd_read_status(opaque: *mut c_void, _addr: u32, _size_log2: c_int) -> u32 {
    unsafe {
        let s = opaque as *mut KBDState;
        let val: c_int = (*s).status as c_int;
        val as u32
    }
}

fn kbd_queue(s: *mut KBDState, b: c_int, aux: c_int) {
    unsafe {
        if aux != 0 {
            ps2_queue((*s).mouse as *mut c_void, b);
        } else {
            ps2_queue((*s).kbd as *mut c_void, b);
        }
    }
}

extern "C" fn kbd_write_command(opaque: *mut c_void, _addr: u32, val: u32, _size_log2: c_int) {
    unsafe {
        let s = opaque as *mut KBDState;

        match val {
            0x20 => kbd_queue(s, (*s).mode as c_int, 1),
            0x60 | 0xD2 | 0xD3 | 0xD4 | 0xD1 => {
                (*s).write_cmd = val as u8;
            }
            0xA7 => (*s).mode |= 0x20,
            0xA8 => (*s).mode &= !0x20u8,
            0xA9 => kbd_queue(s, 0x00, 0),
            0xAA => {
                (*s).status |= 0x04;
                kbd_queue(s, 0x55, 0);
            }
            0xAB => kbd_queue(s, 0x00, 0),
            0xAD => {
                (*s).mode |= 0x10;
                kbd_update_irq(s);
            }
            0xAE => {
                (*s).mode &= !0x10u8;
                kbd_update_irq(s);
            }
            0xC0 => kbd_queue(s, 0x00, 0),
            0xD0 => {
                let mut v: c_int = 0x01 | (ioport_get_a20() << 1);
                if (*s).status & 0x01 != 0 {
                    v |= 0x10;
                }
                if (*s).status & 0x20 != 0 {
                    v |= 0x20;
                }
                kbd_queue(s, v, 0);
            }
            0xDF => ioport_set_a20(1),
            0xDD => ioport_set_a20(0),
            0xFE => qemu_system_reset_request(),
            0xff => {}
            _ => {
                fprintf(
                    get_stderr(),
                    b"qemu: unsupported keyboard cmd=0x%02x\n\0".as_ptr() as *const c_char,
                    val,
                );
            }
        }
    }
}

extern "C" fn kbd_read_data(opaque: *mut c_void, _addr: u32, _size_log2: c_int) -> u32 {
    unsafe {
        let s = opaque as *mut KBDState;
        if (*s).pending == 2 {
            ps2_read_data((*s).mouse as *mut c_void)
        } else {
            ps2_read_data((*s).kbd as *mut c_void)
        }
    }
}

extern "C" fn kbd_write_data(opaque: *mut c_void, _addr: u32, val: u32, _size_log2: c_int) {
    unsafe {
        let s = opaque as *mut KBDState;

        match (*s).write_cmd {
            0 => ps2_write_keyboard((*s).kbd as *mut c_void, val as c_int),
            0x60 => {
                (*s).mode = val as u8;
                let translation: c_int = if (*s).mode & 0x40 != 0 { 1 } else { 0 };
                ps2_keyboard_set_translation((*s).kbd as *mut c_void, translation);

                kbd_update_irq(s);
            }
            0xD2 => kbd_queue(s, val as c_int, 0),
            0xD3 => kbd_queue(s, val as c_int, 1),
            0xD1 => {
                ioport_set_a20(((val >> 1) & 1) as c_int);
                if val & 1 == 0 {
                    qemu_system_reset_request();
                }
            }
            0xD4 => ps2_write_mouse((*s).mouse as *mut c_void, val as c_int),
            _ => {}
        }
        (*s).write_cmd = 0;
    }
}

#[no_mangle]
pub extern "C" fn i8042_init__part1609(
    pkbd: *mut *mut PS2KbdState,
    pmouse: *mut *mut PS2MouseState,
    port_map: *mut PhysMemoryMap,
    kbd_irq: *mut IRQSignal,
    mouse_irq: *mut IRQSignal,
    io_base: u32,
) -> *mut KBDState {
    unsafe {
        let s = mallocz(core::mem::size_of::<KBDState>()) as *mut KBDState;

        (*s).irq_kbd = kbd_irq;
        (*s).irq_mouse = mouse_irq;

        kbd_reset(s);
        cpu_register_device(
            port_map,
            io_base as u64,
            1,
            s as *mut c_void,
            Some(kbd_read_data),
            Some(kbd_write_data),
            1 << 0,
        );
        cpu_register_device(
            port_map,
            (io_base as u64).wrapping_add(4),
            1,
            s as *mut c_void,
            Some(kbd_read_status),
            Some(kbd_write_command),
            1 << 0,
        );

        (*s).kbd = ps2_kbd_init(Some(kbd_update_kbd_irq), s as *mut c_void);
        (*s).mouse = ps2_mouse_init(Some(kbd_update_aux_irq), s as *mut c_void);

        *pkbd = (*s).kbd;
        *pmouse = (*s).mouse;
        s
    }
}
}

