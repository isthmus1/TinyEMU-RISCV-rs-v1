//! From `ps2.c`.

// ── `if (q->count >= 256)` · ps2.c:119-120 · chunk 1618 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (q->count >= 256)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1618 {
// Port of the overflow guard from ps2_queue: report whether the queue is full.
// Returns 1 when count >= 256 (caller must return early), 0 otherwise.

#[no_mangle]
pub extern "C" fn ps2_queue__part1618(count: core::ffi::c_int) -> core::ffi::c_int {
    if count >= 256 {
        1
    } else {
        0
    }
}
}

// ── `q->data[q->wptr] = b;` · ps2.c:121-121 · chunk 1619 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     q->data[q->wptr] = b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1619 {
// Port of the queue-slot store from ps2_queue:  q->data[q->wptr] = b;

use core::ffi::c_int;

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_queue__part1619(q: *mut PS2Queue, b: c_int) {
    if q.is_null() {
        return;
    }
    let wptr = (*q).wptr;
    // Mirror the C store; wptr is kept in [0, 256) by the caller.
    let slot = (*q).data.as_mut_ptr().add(wptr as usize);
    *slot = b as u8;
}
}

// ── `if (++q->wptr == 256)` · ps2.c:122-123 · chunk 1620 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (++q->wptr == 256)
//             q->wptr = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1620 {
// Port of the ring-buffer write pointer wrap check from ps2_queue().
use core::ffi::c_int;

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[no_mangle]
pub extern "C" fn ps2_queue__part1620(q: *mut PS2Queue) {
    unsafe {
        let w = (*q).wptr.wrapping_add(1);
        (*q).wptr = w;
        if w == 256 {
            (*q).wptr = 0;
        }
    }
}
}

// ── `q->count++; s->update_irq(s->update_arg, 1);` · ps2.c:124-125 · chunk 1621 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     q->count++;
//         s->update_irq(s->update_arg, 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1621 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[no_mangle]
pub extern "C" fn ps2_queue__part1621(s: *mut PS2State) {
    unsafe {
        let q = core::ptr::addr_of_mut!((*s).queue);
        (*q).count = (*q).count.wrapping_add(1);
        let f = (*s).update_irq;
        let arg = (*s).update_arg;
        if let Some(f) = f {
            f(arg, 1);
        }
    }
}
}

// ── `if (keycode >= 96)` · ps2.c:142-149 · chunk 1624 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (keycode >= 96) {
//             if (keycode > 127)
//                 return;
//             keycode = linux_input_to_keycode_set1[keycode - 96];
//             if (keycode == 0)
//                 return;
//             ps2_queue(&s->common, 0xe0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1624 {
// Ported piece of ps2_put_keycode: the extended-key remapping block.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

static LINUX_INPUT_TO_KEYCODE_SET1: [u8; 32] = [
    0x1c, 0x1d, 0x35, 0x00, 0x38, 0x00, 0x47, 0x48,
    0x49, 0x4b, 0x4d, 0x4f, 0x50, 0x51, 0x52, 0x53,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x5b, 0x5c, 0x5d,
];

// Returns 1 when the caller must return immediately, 0 to keep going.
// On 0, *out_keycode holds the (possibly remapped) keycode.
#[no_mangle]
pub unsafe extern "C" fn ps2_put_keycode__part1624(
    s: *mut PS2KbdState,
    keycode: c_int,
    out_keycode: *mut c_int,
) -> c_int {
    let mut keycode = keycode;
    if keycode >= 96 {
        if keycode > 127 {
            return 1;
        }
        let idx = (keycode - 96) as usize;
        if idx >= LINUX_INPUT_TO_KEYCODE_SET1.len() {
            return 1;
        }
        keycode = LINUX_INPUT_TO_KEYCODE_SET1[idx] as c_int;
        if keycode == 0 {
            return 1;
        }
        let common = &mut (*s).common as *mut PS2State as *mut c_void;
        ps2_queue(common, 0xe0);
    }
    if !out_keycode.is_null() {
        *out_keycode = keycode;
    }
    0
}
}

// ── `ps2_queue(&s->common, keycode | ((!is_down) << 7));` · ps2.c:150-150 · chunk 1625 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ps2_queue(&s->common, keycode | ((!is_down) << 7));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1625 {
// Port of the final `ps2_queue` call in ps2_put_keycode.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: *mut c_void,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

#[no_mangle]
pub extern "C" fn ps2_put_keycode__part1625(
    s: *mut PS2KbdState,
    is_down: c_int,
    keycode: c_int,
) {
    let not_down: c_int = if is_down == 0 { 1 } else { 0 };
    let b: c_int = keycode | (not_down << 7);
    unsafe {
        let common: *mut PS2State = &mut (*s).common;
        ps2_queue(common as *mut c_void, b);
    }
}
}

// ── `PS2State *s = (PS2State *)opaque; PS2Queue *q; int val, index; q = &s->queue;` · ps2.c:155-159 · chunk 1627 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PS2State *s = (PS2State *)opaque;
//         PS2Queue *q;
//         int val, index;
//     
//         q = &s->queue;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1627 {
// Port of the initialisation block of ps2_read_data(): it only derives the
// PS2State / PS2Queue pointers from the opaque argument.  The caller keeps
// the declarations of s, q, val and index.

use core::ffi::{c_int, c_void};

const PS2_QUEUE_SIZE: usize = 256;

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; PS2_QUEUE_SIZE],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_read_data__part1627(
    opaque: *mut c_void,
    s_out: *mut *mut PS2State,
    q_out: *mut *mut PS2Queue,
) {
    let s = opaque as *mut PS2State;
    let q = core::ptr::addr_of_mut!((*s).queue);
    if !s_out.is_null() {
        *s_out = s;
    }
    if !q_out.is_null() {
        *q_out = q;
    }
}
}

// ── `if (q->count == 0) … else` · ps2.c:160-177 · chunk 1628 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (q->count == 0) {
//     
//     
//     
//             index = q->rptr - 1;
//             if (index < 0)
//                 index = 256 - 1;
//             val = q->data[index];
//         } else {
//             val = q->data[q->rptr];
//             if (++q->rptr == 256)
//                 q->rptr = 0;
//             q->count--;
//     
//             s->update_irq(s->update_arg, 0);
//     
//             s->update_irq(s->update_arg, q->count != 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1628 {
// Portion of ps2_read_data: queue read / deassert-reassert IRQ logic.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[no_mangle]
pub extern "C" fn ps2_read_data__part1628(opaque: *mut c_void) -> c_int {
    unsafe {
        let s = opaque as *mut PS2State;
        let q = core::ptr::addr_of_mut!((*s).queue);
        let val: c_int;
        if (*q).count == 0 {
            // NOTE: if no data left, return the last one (needed for EMM386)
            let mut index: c_int = (*q).rptr - 1;
            if index < 0 {
                index = 256 - 1;
            }
            val = *(*q).data.as_ptr().add(index as usize) as c_int;
        } else {
            val = *(*q).data.as_ptr().add((*q).rptr as usize) as c_int;
            (*q).rptr += 1;
            if (*q).rptr == 256 {
                (*q).rptr = 0;
            }
            (*q).count -= 1;

            // reading deasserts IRQ
            let f = (*s).update_irq;
            if let Some(func) = f {
                func((*s).update_arg, 0);
            }
            // reassert IRQs if data left
            let f2 = (*s).update_irq;
            if let Some(func) = f2 {
                func((*s).update_arg, if (*q).count != 0 { 1 } else { 0 });
            }
        }
        val
    }
}
}

// ── `return val;` · ps2.c:178-178 · chunk 1629 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1629 {
// Returns the value read from the PS/2 queue as an unsigned 32-bit quantity,
// mirroring the C `return val;` of ps2_read_data (int -> uint32_t conversion).
#[no_mangle]
pub extern "C" fn ps2_read_data__part1629(val: core::ffi::c_int) -> core::ffi::c_uint {
    val as core::ffi::c_uint
}
}

// ── `s->scan_enabled = 1;` · ps2.c:183-183 · chunk 1631 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->scan_enabled = 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1631 {
// Port of the single assignment performed by ps2_reset_keyboard:
//   s->scan_enabled = 1;
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_reset_keyboard__part1631(s: *mut PS2KbdState) {
    // Mirrors the C statement exactly: caller guarantees s is valid.
    core::ptr::addr_of_mut!((*s).scan_enabled).write(1);
}
}

// ── `switch (s->common.write_cmd)` · ps2.c:190-245 · chunk 1633 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->common.write_cmd) {
//         default:
//         case -1:
//             switch(val) {
//             case 0x00:
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0x05:
//                 ps2_queue(&s->common, 0xFE);
//                 break;
//             case 0xF2:
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_queue(&s->common, 0xab);
//                 ps2_queue(&s->common, 0x83);
//                 break;
//             case 0xEE:
//                 ps2_queue(&s->common, 0xEE);
//                 break;
//             case 0xF4:
//                 s->scan_enabled = 1;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xED:
//             case 0xF3:
//                 s->common.write_cmd = val;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF5:
//                 ps2_reset_keyboard(s);
//                 s->scan_enabled = 0;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF6:
//                 ps2_reset_keyboard(s);
//                 s->scan_enabled = 1;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xFF:
//                 ps2_reset_keyboard(s);
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_queue(&s->common, 0xAA);
//                 break;
//             default:
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             }
//             break;
//         case 0xED:
//             ps2_queue(&s->common, 0xFA);
//             s->common.write_cmd = -1;
//             break;
//         case 0xF3:
//             ps2_queue(&s->common, 0xFA);
//             s->common.write_cmd = -1;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1633 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

// private port of the static C helper ps2_reset_keyboard
unsafe fn ps2_reset_keyboard(s: *mut PS2KbdState) {
    (*s).scan_enabled = 1;
}

#[no_mangle]
pub unsafe extern "C" fn ps2_write_keyboard__part1633(opaque: *mut PS2KbdState, val: c_int) {
    let s = opaque;
    let common = core::ptr::addr_of_mut!((*s).common) as *mut c_void;

    match (*s).common.write_cmd {
        -1 => {
            match val {
                0x00 => {
                    ps2_queue(common, 0xFA);
                }
                0x05 => {
                    ps2_queue(common, 0xFE);
                }
                0xF2 => {
                    ps2_queue(common, 0xFA);
                    ps2_queue(common, 0xab);
                    ps2_queue(common, 0x83);
                }
                0xEE => {
                    ps2_queue(common, 0xEE);
                }
                0xF4 => {
                    (*s).scan_enabled = 1;
                    ps2_queue(common, 0xFA);
                }
                0xED | 0xF3 => {
                    (*s).common.write_cmd = val;
                    ps2_queue(common, 0xFA);
                }
                0xF5 => {
                    ps2_reset_keyboard(s);
                    (*s).scan_enabled = 0;
                    ps2_queue(common, 0xFA);
                }
                0xF6 => {
                    ps2_reset_keyboard(s);
                    (*s).scan_enabled = 1;
                    ps2_queue(common, 0xFA);
                }
                0xFF => {
                    ps2_reset_keyboard(s);
                    ps2_queue(common, 0xFA);
                    ps2_queue(common, 0xAA);
                }
                _ => {
                    ps2_queue(common, 0xFA);
                }
            }
        }
        0xED => {
            ps2_queue(common, 0xFA);
            (*s).common.write_cmd = -1;
        }
        0xF3 => {
            ps2_queue(common, 0xFA);
            (*s).common.write_cmd = -1;
        }
        _ => {
            // default case is grouped with case -1 in the C switch
            match val {
                0x00 => {
                    ps2_queue(common, 0xFA);
                }
                0x05 => {
                    ps2_queue(common, 0xFE);
                }
                0xF2 => {
                    ps2_queue(common, 0xFA);
                    ps2_queue(common, 0xab);
                    ps2_queue(common, 0x83);
                }
                0xEE => {
                    ps2_queue(common, 0xEE);
                }
                0xF4 => {
                    (*s).scan_enabled = 1;
                    ps2_queue(common, 0xFA);
                }
                0xED | 0xF3 => {
                    (*s).common.write_cmd = val;
                    ps2_queue(common, 0xFA);
                }
                0xF5 => {
                    ps2_reset_keyboard(s);
                    (*s).scan_enabled = 0;
                    ps2_queue(common, 0xFA);
                }
                0xF6 => {
                    ps2_reset_keyboard(s);
                    (*s).scan_enabled = 1;
                    ps2_queue(common, 0xFA);
                }
                0xFF => {
                    ps2_reset_keyboard(s);
                    ps2_queue(common, 0xFA);
                    ps2_queue(common, 0xAA);
                }
                _ => {
                    ps2_queue(common, 0xFA);
                }
            }
        }
    }
}
}

// ── `PS2KbdState *s = (PS2KbdState *)opaque; s->translate = mode;` · ps2.c:254-255 · chunk 1635 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PS2KbdState *s = (PS2KbdState *)opaque;
//         s->translate = mode;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1635 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_set_translation__part1635(
    opaque: *mut c_void,
    mode: c_int,
) {
    let s = opaque as *mut PS2KbdState;
    (*s).translate = mode;
}
}

// ── `unsigned int b; int dx1, dy1, dz1; dx1 = s->mouse_dx; dy1 = s->mouse_dy; dz1 = s->mouse_dz;` · ps2.c:260-265 · chunk 1637 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     unsigned int b;
//         int dx1, dy1, dz1;
//     
//         dx1 = s->mouse_dx;
//         dy1 = s->mouse_dy;
//         dz1 = s->mouse_dz;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1637 {
// Port of the local-variable setup / delta fetch of ps2_mouse_send_packet.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: *mut c_void,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_send_packet__part1637(
    s: *mut PS2MouseState,
    dx1: *mut c_int,
    dy1: *mut c_int,
    dz1: *mut c_int,
) {
    let st = &*s;
    if !dx1.is_null() {
        *dx1 = st.mouse_dx;
    }
    if !dy1.is_null() {
        *dy1 = st.mouse_dy;
    }
    if !dz1.is_null() {
        *dz1 = st.mouse_dz;
    }
}
}

// ── `if (dx1 > 127) … else` · ps2.c:267-270 · chunk 1638 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dx1 > 127)
//             dx1 = 127;
//         else if (dx1 < -127)
//             dx1 = -127;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1638 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ps2_mouse_send_packet__part1638(dx1: *mut c_int) {
    unsafe {
        let v = *dx1;
        if v > 127 {
            *dx1 = 127;
        } else if v < -127 {
            *dx1 = -127;
        }
    }
}
}

// ── `if (dy1 > 127) … else` · ps2.c:271-274 · chunk 1639 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dy1 > 127)
//             dy1 = 127;
//         else if (dy1 < -127)
//             dy1 = -127;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1639 {
// Clamp dy1 into the signed 7-bit range a PS/2 mouse packet can carry.
#[no_mangle]
pub extern "C" fn ps2_mouse_send_packet__part1639(dy1: *mut core::ffi::c_int) {
    if dy1.is_null() {
        return;
    }
    unsafe {
        if *dy1 > 127 {
            *dy1 = 127;
        } else if *dy1 < -127 {
            *dy1 = -127;
        }
    }
}
}

// ── `b = 0x08 | ((dx1 < 0) << 4) | ((dy1 < 0) << 5) | (s->mouse_buttons & 0x07); ps2_queue(&s->commo…` · ps2.c:275-278 · chunk 1640 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     b = 0x08 | ((dx1 < 0) << 4) | ((dy1 < 0) << 5) | (s->mouse_buttons & 0x07);
//         ps2_queue(&s->common, b);
//         ps2_queue(&s->common, dx1 & 0xff);
//         ps2_queue(&s->common, dy1 & 0xff);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1640 {
// Port of the header/first two data bytes queued by ps2_mouse_send_packet.
use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

#[no_mangle]
pub extern "C" fn ps2_mouse_send_packet__part1640(
    common: *mut c_void,
    mouse_buttons: c_uint,
    dx1: c_int,
    dy1: c_int,
) -> c_uint {
    let b: c_uint = 0x08u32
        | (((dx1 < 0) as u32) << 4)
        | (((dy1 < 0) as u32) << 5)
        | (mouse_buttons & 0x07);
    unsafe {
        ps2_queue(common, b as c_int);
        ps2_queue(common, (dx1 & 0xff) as c_int);
        ps2_queue(common, (dy1 & 0xff) as c_int);
    }
    b
}
}

// ── `switch (s->mouse_type)` · ps2.c:280-298 · chunk 1641 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->mouse_type) {
//         default:
//             break;
//         case 3:
//             if (dz1 > 127)
//                 dz1 = 127;
//             else if (dz1 < -127)
//                     dz1 = -127;
//             ps2_queue(&s->common, dz1 & 0xff);
//             break;
//         case 4:
//             if (dz1 > 7)
//                 dz1 = 7;
//             else if (dz1 < -7)
//                 dz1 = -7;
//             b = (dz1 & 0x0f) | ((s->mouse_buttons & 0x18) << 1);
//             ps2_queue(&s->common, b);
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1641 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

#[no_mangle]
pub extern "C" fn ps2_mouse_send_packet__part1641(s: *mut PS2MouseState, dz1: *mut c_int) {
    unsafe {
        let common: *mut c_void = core::ptr::addr_of_mut!((*s).common) as *mut c_void;
        let mouse_type = (*s).mouse_type as c_int;
        let mut dz: c_int = *dz1;

        if mouse_type == 3 {
            if dz > 127 {
                dz = 127;
            } else if dz < -127 {
                dz = -127;
            }
            ps2_queue(common, dz & 0xff);
        } else if mouse_type == 4 {
            if dz > 7 {
                dz = 7;
            } else if dz < -7 {
                dz = -7;
            }
            let b: c_int = (dz & 0x0f) | (((*s).mouse_buttons as c_int & 0x18) << 1);
            ps2_queue(common, b);
        }

        *dz1 = dz;
    }
}
}

// ── `s->mouse_dx -= dx1; s->mouse_dy -= dy1; s->mouse_dz -= dz1;` · ps2.c:301-303 · chunk 1642 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->mouse_dx -= dx1;
//         s->mouse_dy -= dy1;
//         s->mouse_dz -= dz1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1642 {
// Port of the tail of ps2_mouse_send_packet: consume the clamped deltas
// from the accumulated mouse movement fields.
#[no_mangle]
pub extern "C" fn ps2_mouse_send_packet__part1642(
    pdx: *mut core::ffi::c_int,
    pdy: *mut core::ffi::c_int,
    pdz: *mut core::ffi::c_int,
    dx1: core::ffi::c_int,
    dy1: core::ffi::c_int,
    dz1: core::ffi::c_int,
) {
    unsafe {
        if !pdx.is_null() {
            *pdx = (*pdx).wrapping_sub(dx1);
        }
        if !pdy.is_null() {
            *pdy = (*pdy).wrapping_sub(dy1);
        }
        if !pdz.is_null() {
            *pdz = (*pdz).wrapping_sub(dz1);
        }
    }
}
}

// ── `if (!(s->mouse_status & 0x20)` · ps2.c:310-311 · chunk 1644 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->mouse_status & 0x20))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1644 {
// Port of the early-out check at the top of ps2_mouse_event:
//   if (!(s->mouse_status & 0x20)) return;
// Returns nonzero when the caller must return immediately.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: *mut c_void,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

#[no_mangle]
pub extern "C" fn ps2_mouse_event__part1644(s: *mut PS2MouseState) -> c_int {
    unsafe {
        if ((*s).mouse_status & 0x20) == 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `s->mouse_dx += dx; s->mouse_dy -= dy; s->mouse_dz += dz;` · ps2.c:313-315 · chunk 1645 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->mouse_dx += dx;
//         s->mouse_dy -= dy;
//         s->mouse_dz += dz;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1645 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_event__part1645(
    s: *mut PS2MouseState,
    dx: c_int,
    dy: c_int,
    dz: c_int,
) {
    let m = &mut *s;
    m.mouse_dx = m.mouse_dx.wrapping_add(dx);
    m.mouse_dy = m.mouse_dy.wrapping_sub(dy);
    m.mouse_dz = m.mouse_dz.wrapping_add(dz);
}
}

// ── `if (s->mouse_dx == 0 && s->mouse_dy == 0 && s->mouse_dz == 0 && s->mouse_buttons == buttons_sta…` · ps2.c:317-319 · chunk 1646 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->mouse_dx == 0 && s->mouse_dy == 0 && s->mouse_dz == 0 &&
//             s->mouse_buttons == buttons_state)
//      return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1646 {
use core::ffi::{c_int, c_void};

// Mirror of the C structures accessed through the PS2MouseState pointer.
#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

/// Returns nonzero when the caller must `return` from ps2_mouse_event.
#[no_mangle]
pub extern "C" fn ps2_mouse_event__part1646(
    s: *mut PS2MouseState,
    buttons_state: c_int,
) -> c_int {
    if s.is_null() {
        return 0;
    }
    unsafe {
        let sp = &*s;
        if sp.mouse_dx == 0
            && sp.mouse_dy == 0
            && sp.mouse_dz == 0
            && (sp.mouse_buttons as c_int) == buttons_state
        {
            return 1;
        }
    }
    0
}
}

// ── `s->mouse_buttons = buttons_state;` · ps2.c:320-320 · chunk 1647 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->mouse_buttons = buttons_state;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1647 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_event__part1647(s: *mut PS2MouseState, buttons_state: c_int) {
    if s.is_null() {
        return;
    }
    (*s).mouse_buttons = buttons_state as u8;
}
}

// ── `if (!(s->mouse_status & 0x40) && (s->common.queue.count < (256 - 16)` · ps2.c:322-331 · chunk 1648 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->mouse_status & 0x40) &&
//             (s->common.queue.count < (256 - 16))) {
//             for(;;) {
//     
//     
//                 ps2_mouse_send_packet(s);
//                 if (s->mouse_dx == 0 && s->mouse_dy == 0 && s->mouse_dz == 0)
//                     break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1648 {
// Port of the TARGET block inside ps2_mouse_event (ps2.c lines 322-331):
// the guarded "flush pending mouse movement by sending packets" loop.
// The static helper ps2_mouse_send_packet() has no linker symbol, so its
// logic is ported here as a private Rust function.

use core::ffi::{c_int, c_void};
use core::ptr::addr_of_mut;

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

pub type UpdateIrqFn = Option<unsafe extern "C" fn(*mut c_void, c_int)>;

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: UpdateIrqFn,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

// Private port of the static C function ps2_mouse_send_packet().
unsafe fn ps2_mouse_send_packet(s: *mut PS2MouseState) {
    let mut dx1: c_int = (*s).mouse_dx;
    let mut dy1: c_int = (*s).mouse_dy;
    let mut dz1: c_int = (*s).mouse_dz;

    if dx1 > 127 {
        dx1 = 127;
    } else if dx1 < -127 {
        dx1 = -127;
    }
    if dy1 > 127 {
        dy1 = 127;
    } else if dy1 < -127 {
        dy1 = -127;
    }

    let mut b: u32 = 0x08u32
        | ((dx1 < 0) as u32) << 4
        | ((dy1 < 0) as u32) << 5
        | (((*s).mouse_buttons as u32) & 0x07);
    ps2_queue(addr_of_mut!((*s).common) as *mut c_void, b as c_int);
    ps2_queue(addr_of_mut!((*s).common) as *mut c_void, dx1 & 0xff);
    ps2_queue(addr_of_mut!((*s).common) as *mut c_void, dy1 & 0xff);

    match (*s).mouse_type {
        3 => {
            if dz1 > 127 {
                dz1 = 127;
            } else if dz1 < -127 {
                dz1 = -127;
            }
            ps2_queue(addr_of_mut!((*s).common) as *mut c_void, dz1 & 0xff);
        }
        4 => {
            if dz1 > 7 {
                dz1 = 7;
            } else if dz1 < -7 {
                dz1 = -7;
            }
            b = ((dz1 & 0x0f) as u32) | ((((*s).mouse_buttons as u32) & 0x18) << 1);
            ps2_queue(addr_of_mut!((*s).common) as *mut c_void, b as c_int);
        }
        _ => {}
    }

    (*s).mouse_dx -= dx1;
    (*s).mouse_dy -= dy1;
    (*s).mouse_dz -= dz1;
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_event__part1648(s: *mut PS2MouseState) {
    if ((*s).mouse_status & 0x40) == 0 && (*s).common.queue.count < (256 - 16) {
        loop {
            ps2_mouse_send_packet(s);
            if (*s).mouse_dx == 0 && (*s).mouse_dy == 0 && (*s).mouse_dz == 0 {
                break;
            }
        }
    }
}
}

// ── `switch (s->common.write_cmd)` · ps2.c:340-457 · chunk 1650 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(s->common.write_cmd) {
//         default:
//         case -1:
//     
//             if (s->mouse_wrap) {
//                 if (val == 0xEC) {
//                     s->mouse_wrap = 0;
//                     ps2_queue(&s->common, 0xFA);
//                     return;
//                 } else if (val != 0xFF) {
//                     ps2_queue(&s->common, val);
//                     return;
//                 }
//             }
//             switch(val) {
//             case 0xE6:
//                 s->mouse_status &= ~0x10;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xE7:
//                 s->mouse_status |= 0x10;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xEA:
//                 s->mouse_status &= ~0x40;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xEE:
//                 s->mouse_wrap = 1;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF0:
//                 s->mouse_status |= 0x40;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF2:
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_queue(&s->common, s->mouse_type);
//                 break;
//             case 0xE8:
//             case 0xF3:
//                 s->common.write_cmd = val;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xE9:
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_queue(&s->common, s->mouse_status);
//                 ps2_queue(&s->common, s->mouse_resolution);
//                 ps2_queue(&s->common, s->mouse_sample_rate);
//                 break;
//             case 0xEB:
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_mouse_send_packet(s);
//                 break;
//             case 0xF4:
//                 s->mouse_status |= 0x20;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF5:
//                 s->mouse_status &= ~0x20;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xF6:
//                 s->mouse_sample_rate = 100;
//                 s->mouse_resolution = 2;
//                 s->mouse_status = 0;
//                 ps2_queue(&s->common, 0xFA);
//                 break;
//             case 0xFF:
//                 s->mouse_sample_rate = 100;
//                 s->mouse_resolution = 2;
//                 s->mouse_status = 0;
//                 s->mouse_type = 0;
//                 ps2_queue(&s->common, 0xFA);
//                 ps2_queue(&s->common, 0xaa);
//                 ps2_queue(&s->common, s->mouse_type);
//                 break;
//             default:
//                 break;
//             }
//             break;
//         case 0xF3:
//             s->mouse_sample_rate = val;
//     
//             switch(s->mouse_detect_state) {
//             default:
//             case 0:
//                 if (val == 200)
//                     s->mouse_detect_state = 1;
//                 break;
//             case 1:
//                 if (val == 100)
//                     s->mouse_detect_state = 2;
//                 else if (val == 200)
//                     s->mouse_detect_state = 3;
//                 else
//                     s->mouse_detect_state = 0;
//                 break;
//             case 2:
//                 if (val == 80)
//                     s->mouse_type = 3;
//                 s->mouse_detect_state = 0;
//                 break;
//             case 3:
//                 if (val == 80)
//                     s->mouse_type = 4;
//                 s->mouse_detect_state = 0;
//                 break;
//             }
//             ps2_queue(&s->common, 0xFA);
//             s->common.write_cmd = -1;
//             break;
//         case 0xE8:
//             s->mouse_resolution = val;
//             ps2_queue(&s->common, 0xFA);
//             s->common.write_cmd = -1;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1650 {
// Port of the write_cmd switch of ps2_write_mouse (ps2.c lines 340-457).
use core::ffi::{c_int, c_uint, c_void};
use core::ptr::addr_of_mut;

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

extern "C" {
    fn ps2_queue(opaque: *mut c_void, b: c_int);
}

#[inline]
unsafe fn queue_common(s: *mut PS2MouseState, b: c_int) {
    ps2_queue(addr_of_mut!((*s).common) as *mut c_void, b);
}

// Port of the static C function ps2_mouse_send_packet (no linker symbol).
unsafe fn ps2_mouse_send_packet(s: *mut PS2MouseState) {
    let mut dx1: c_int = (*s).mouse_dx;
    let mut dy1: c_int = (*s).mouse_dy;
    let mut dz1: c_int = (*s).mouse_dz;

    if dx1 > 127 {
        dx1 = 127;
    } else if dx1 < -127 {
        dx1 = -127;
    }
    if dy1 > 127 {
        dy1 = 127;
    } else if dy1 < -127 {
        dy1 = -127;
    }
    let mut b: c_uint = 0x08u32
        | (((dx1 < 0) as c_uint) << 4)
        | (((dy1 < 0) as c_uint) << 5)
        | (((*s).mouse_buttons & 0x07) as c_uint);
    queue_common(s, b as c_int);
    queue_common(s, dx1 & 0xff);
    queue_common(s, dy1 & 0xff);

    match (*s).mouse_type {
        3 => {
            if dz1 > 127 {
                dz1 = 127;
            } else if dz1 < -127 {
                dz1 = -127;
            }
            queue_common(s, dz1 & 0xff);
        }
        4 => {
            if dz1 > 7 {
                dz1 = 7;
            } else if dz1 < -7 {
                dz1 = -7;
            }
            b = ((dz1 & 0x0f) as c_uint) | ((((*s).mouse_buttons & 0x18) as c_uint) << 1);
            queue_common(s, b as c_int);
        }
        _ => {}
    }

    (*s).mouse_dx -= dx1;
    (*s).mouse_dy -= dy1;
    (*s).mouse_dz -= dz1;
}

// Returns 1 when the original C code would have executed `return;`
// (early exit from ps2_write_mouse), 0 otherwise.
#[no_mangle]
pub extern "C" fn ps2_write_mouse__part1650(opaque: *mut c_void, val: c_int) -> c_int {
    unsafe {
        let s = opaque as *mut PS2MouseState;
        match (*s).common.write_cmd {
            0xF3 => {
                (*s).mouse_sample_rate = val as u8;

                match (*s).mouse_detect_state {
                    1 => {
                        if val == 100 {
                            (*s).mouse_detect_state = 2;
                        } else if val == 200 {
                            (*s).mouse_detect_state = 3;
                        } else {
                            (*s).mouse_detect_state = 0;
                        }
                    }
                    2 => {
                        if val == 80 {
                            (*s).mouse_type = 3;
                        }
                        (*s).mouse_detect_state = 0;
                    }
                    3 => {
                        if val == 80 {
                            (*s).mouse_type = 4;
                        }
                        (*s).mouse_detect_state = 0;
                    }
                    _ => {
                        // default: and case 0:
                        if val == 200 {
                            (*s).mouse_detect_state = 1;
                        }
                    }
                }
                queue_common(s, 0xFA);
                (*s).common.write_cmd = -1;
            }
            0xE8 => {
                (*s).mouse_resolution = val as u8;
                queue_common(s, 0xFA);
                (*s).common.write_cmd = -1;
            }
            _ => {
                // default: and case -1:
                if (*s).mouse_wrap != 0 {
                    if val == 0xEC {
                        (*s).mouse_wrap = 0;
                        queue_common(s, 0xFA);
                        return 1;
                    } else if val != 0xFF {
                        queue_common(s, val);
                        return 1;
                    }
                }
                match val {
                    0xE6 => {
                        (*s).mouse_status &= !0x10u8;
                        queue_common(s, 0xFA);
                    }
                    0xE7 => {
                        (*s).mouse_status |= 0x10u8;
                        queue_common(s, 0xFA);
                    }
                    0xEA => {
                        (*s).mouse_status &= !0x40u8;
                        queue_common(s, 0xFA);
                    }
                    0xEE => {
                        (*s).mouse_wrap = 1;
                        queue_common(s, 0xFA);
                    }
                    0xF0 => {
                        (*s).mouse_status |= 0x40u8;
                        queue_common(s, 0xFA);
                    }
                    0xF2 => {
                        queue_common(s, 0xFA);
                        queue_common(s, (*s).mouse_type as c_int);
                    }
                    0xE8 | 0xF3 => {
                        (*s).common.write_cmd = val;
                        queue_common(s, 0xFA);
                    }
                    0xE9 => {
                        queue_common(s, 0xFA);
                        queue_common(s, (*s).mouse_status as c_int);
                        queue_common(s, (*s).mouse_resolution as c_int);
                        queue_common(s, (*s).mouse_sample_rate as c_int);
                    }
                    0xEB => {
                        queue_common(s, 0xFA);
                        ps2_mouse_send_packet(s);
                    }
                    0xF4 => {
                        (*s).mouse_status |= 0x20u8;
                        queue_common(s, 0xFA);
                    }
                    0xF5 => {
                        (*s).mouse_status &= !0x20u8;
                        queue_common(s, 0xFA);
                    }
                    0xF6 => {
                        (*s).mouse_sample_rate = 100;
                        (*s).mouse_resolution = 2;
                        (*s).mouse_status = 0;
                        queue_common(s, 0xFA);
                    }
                    0xFF => {
                        (*s).mouse_sample_rate = 100;
                        (*s).mouse_resolution = 2;
                        (*s).mouse_status = 0;
                        (*s).mouse_type = 0;
                        queue_common(s, 0xFA);
                        queue_common(s, 0xaa);
                        queue_common(s, (*s).mouse_type as c_int);
                    }
                    _ => {}
                }
            }
        }
        0
    }
}
}

// ── `PS2State *s = (PS2State *)opaque; PS2Queue *q; s->write_cmd = -1; q = &s->queue; q->rptr = 0; q…` · ps2.c:462-468 · chunk 1652 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PS2State *s = (PS2State *)opaque;
//         PS2Queue *q;
//         s->write_cmd = -1;
//         q = &s->queue;
//         q->rptr = 0;
//         q->wptr = 0;
//         q->count = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1652 {
// Ported piece of ps2_reset: reset of the PS2 queue and write_cmd.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[no_mangle]
pub extern "C" fn ps2_reset__part1652(opaque: *mut c_void) {
    let s = opaque as *mut PS2State;
    unsafe {
        (*s).write_cmd = -1;
        let q = &mut (*s).queue;
        q.rptr = 0;
        q.wptr = 0;
        q.count = 0;
    }
}
}

// ── `PS2KbdState *s = (PS2KbdState *)mallocz(sizeof(PS2KbdState)); s->common.update_irq = update_irq…` · ps2.c:473-478 · chunk 1654 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PS2KbdState *s = (PS2KbdState *)mallocz(sizeof(PS2KbdState));
//     
//         s->common.update_irq = update_irq;
//         s->common.update_arg = update_arg;
//         ps2_reset(&s->common);
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1654 {
// Port of ps2_kbd_init (ps2.c lines 473-478): allocate a PS2KbdState,
// install the IRQ callback, reset the common PS/2 state, return it.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2KbdState {
    pub common: PS2State,
    pub scan_enabled: c_int,
    pub translate: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

// static void ps2_reset(void *opaque) -- file-local in C, so ported here.
unsafe fn ps2_reset(state: *mut PS2State) {
    let s = &mut *state;
    s.write_cmd = -1;
    let q = &mut s.queue;
    q.rptr = 0;
    q.wptr = 0;
    q.count = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ps2_kbd_init__part1654(
    update_irq: Option<extern "C" fn(*mut c_void, c_int)>,
    update_arg: *mut c_void,
) -> *mut PS2KbdState {
    let s = mallocz(core::mem::size_of::<PS2KbdState>()) as *mut PS2KbdState;

    (*s).common.update_irq = update_irq;
    (*s).common.update_arg = update_arg;
    ps2_reset(&mut (*s).common);
    s
}
}

// ── `PS2MouseState *s = (PS2MouseState *)mallocz(sizeof(PS2MouseState)); s->common.update_irq = upda…` · ps2.c:483-488 · chunk 1656 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PS2MouseState *s = (PS2MouseState *)mallocz(sizeof(PS2MouseState));
//     
//         s->common.update_irq = update_irq;
//         s->common.update_arg = update_arg;
//         ps2_reset(&s->common);
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1656 {
// Port of the body of ps2_mouse_init (ps2.c lines 483-488).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PS2Queue {
    pub data: [u8; 256],
    pub rptr: c_int,
    pub wptr: c_int,
    pub count: c_int,
}

#[repr(C)]
pub struct PS2State {
    pub queue: PS2Queue,
    pub write_cmd: i32,
    pub update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub update_arg: *mut c_void,
}

#[repr(C)]
pub struct PS2MouseState {
    pub common: PS2State,
    pub mouse_status: u8,
    pub mouse_resolution: u8,
    pub mouse_sample_rate: u8,
    pub mouse_wrap: u8,
    pub mouse_type: u8,
    pub mouse_detect_state: u8,
    pub mouse_dx: c_int,
    pub mouse_dy: c_int,
    pub mouse_dz: c_int,
    pub mouse_buttons: u8,
}

extern "C" {
    // void *mallocz(size_t size);
    fn mallocz(size: usize) -> *mut c_void;
}

// Ported from the static C function ps2_reset (no linker symbol exists for it).
unsafe fn ps2_reset(opaque: *mut c_void) {
    let s = opaque as *mut PS2State;
    (*s).write_cmd = -1;
    let q = core::ptr::addr_of_mut!((*s).queue);
    (*q).rptr = 0;
    (*q).wptr = 0;
    (*q).count = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_init__part1656(
    update_irq: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    update_arg: *mut c_void,
) -> *mut PS2MouseState {
    let s = mallocz(core::mem::size_of::<PS2MouseState>()) as *mut PS2MouseState;

    (*s).common.update_irq = update_irq;
    (*s).common.update_arg = update_arg;
    ps2_reset(core::ptr::addr_of_mut!((*s).common) as *mut c_void);
    s
}
}

