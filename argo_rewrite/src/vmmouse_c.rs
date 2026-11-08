//! From `vmmouse.c`.

// ── `put_queue` · vmmouse.c:53-61 · chunk 3928 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void put_queue(VMMouseState *s, uint32_t val)
//     {
//         if (s->fifo_count >= (4 * 16))
//             return;
//         s->fifo_buf[s->fifo_windex] = val;
//         if (++s->fifo_windex == (4 * 16))
//             s->fifo_windex = 0;
//         s->fifo_count++;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3928 {
use core::ffi::c_int;

const FIFO_SIZE: c_int = 4 * 16;

// Mirror of the C struct VMMouseState (PS2MouseState is only used as a pointer).
#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,  /* BOOL == int */
    pub absolute: c_int, /* BOOL == int */
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn put_queue(s: *mut VMMouseState, val: u32) {
    unsafe {
        let st = &mut *s;
        if st.fifo_count >= FIFO_SIZE {
            return;
        }
        *st.fifo_buf.as_mut_ptr().offset(st.fifo_windex as isize) = val;
        st.fifo_windex = st.fifo_windex.wrapping_add(1);
        if st.fifo_windex == FIFO_SIZE {
            st.fifo_windex = 0;
        }
        st.fifo_count = st.fifo_count.wrapping_add(1);
    }
}
}

// ── `if (s->fifo_count >= (4 * 16)` · vmmouse.c:55-56 · chunk 3929 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->fifo_count >= (4 * 16))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3929 {
// Ported piece of put_queue: the early-exit capacity check.
#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: core::ffi::c_int,
    pub fifo_rindex: core::ffi::c_int,
    pub fifo_windex: core::ffi::c_int,
    pub enabled: core::ffi::c_int,
    pub absolute: core::ffi::c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn put_queue__part3929(s: *mut VMMouseState) -> core::ffi::c_int {
    unsafe {
        if (*s).fifo_count >= 64 {
            return 1;
        }
    }
    0
}
}

// ── `s->fifo_buf[s->fifo_windex] = val;` · vmmouse.c:57-57 · chunk 3930 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->fifo_buf[s->fifo_windex] = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3930 {
// Port of the single statement `s->fifo_buf[s->fifo_windex] = val;`
// from put_queue() in vmmouse.c.

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut PS2MouseState,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,  /* BOOL */
    pub absolute: c_int, /* BOOL */
    pub fifo_buf: [c_uint; 4 * 16],
}

#[no_mangle]
pub unsafe extern "C" fn put_queue__part3930(s: *mut VMMouseState, val: c_uint) {
    if s.is_null() {
        return;
    }
    let st = &mut *s;
    let idx = st.fifo_windex as usize;
    let base: *mut c_uint = core::ptr::addr_of_mut!(st.fifo_buf) as *mut c_uint;
    core::ptr::write(base.add(idx), val);
}
}

// ── `if (++s->fifo_windex == (4 * 16)` · vmmouse.c:58-59 · chunk 3931 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (++s->fifo_windex == (4 * 16))
//             s->fifo_windex = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3931 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn put_queue__part3931(s: *mut VMMouseState) {
    unsafe {
        let s = &mut *s;
        s.fifo_windex = s.fifo_windex.wrapping_add(1);
        if s.fifo_windex == 64 {
            s.fifo_windex = 0;
        }
    }
}
}

// ── `s->fifo_count++;` · vmmouse.c:60-60 · chunk 3932 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->fifo_count++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3932 {
use core::ffi::c_int;

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn put_queue__part3932(s: *mut VMMouseState) {
    unsafe {
        let count = (*s).fifo_count;
        (*s).fifo_count = count.wrapping_add(1);
    }
}
}

// ── `read_data` · vmmouse.c:63-77 · chunk 3933 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void read_data(VMMouseState *s, uint32_t *regs, int size)
//     {
//         int i;
//         if (size > 6 || size > s->fifo_count) {
//     
//             s->enabled = FALSE;
//             return;
//         }
//         for(i = 0; i < size; i++) {
//             regs[i] = s->fifo_buf[s->fifo_rindex];
//             if (++s->fifo_rindex == (4 * 16))
//                 s->fifo_rindex = 0;
//         }
//         s->fifo_count -= size;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3933 {
// Port of vmmouse.c:read_data
use core::ffi::c_int;

const FIFO_SIZE: c_int = 4 * 16;

// Mirror of `struct VMMouseState` (BOOL is `typedef int BOOL`).
#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; FIFO_SIZE as usize],
}

#[no_mangle]
pub unsafe extern "C" fn read_data(s: *mut VMMouseState, regs: *mut u32, size: c_int) {
    if size > 6 || size > (*s).fifo_count {
        (*s).enabled = 0; // FALSE
        return;
    }

    let mut i: c_int = 0;
    while i < size {
        let idx = (*s).fifo_rindex as usize;
        let val = *(*s).fifo_buf.as_ptr().add(idx);
        *regs.offset(i as isize) = val;

        let next = (*s).fifo_rindex.wrapping_add(1);
        (*s).fifo_rindex = next;
        if next == FIFO_SIZE {
            (*s).fifo_rindex = 0;
        }
        i += 1;
    }
    (*s).fifo_count = (*s).fifo_count.wrapping_sub(size);
}
}

// ── `if (size > 6 || size > s->fifo_count)` · vmmouse.c:66-70 · chunk 3934 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (size > 6 || size > s->fifo_count) {
//     
//             s->enabled = FALSE;
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3934 {
// Port of the size/fifo validation branch of read_data().
// Returns nonzero when the original C branch took the early `return`.

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: core::ffi::c_int,
    pub fifo_rindex: core::ffi::c_int,
    pub fifo_windex: core::ffi::c_int,
    pub enabled: core::ffi::c_int,
    pub absolute: core::ffi::c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn read_data__part3934(s: *mut VMMouseState, size: core::ffi::c_int) -> core::ffi::c_int {
    unsafe {
        if size > 6 || size > (*s).fifo_count {
            (*s).enabled = 0;
            return 1;
        }
    }
    0
}
}

// ── `for (i = 0; i < size; i++)` · vmmouse.c:71-75 · chunk 3935 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < size; i++) {
//             regs[i] = s->fifo_buf[s->fifo_rindex];
//             if (++s->fifo_rindex == (4 * 16))
//                 s->fifo_rindex = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3935 {
// Rust replacement for the FIFO read loop inside read_data() (vmmouse.c)
use core::ffi::{c_int, c_uint};

/// Mirror of the C struct VMMouseState (opaque ps2_mouse pointer).
#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [c_uint; 64],
}

#[no_mangle]
pub extern "C" fn read_data__part3935(s: *mut VMMouseState, regs: *mut c_uint, size: c_int) {
    let mut i: c_int = 0;
    unsafe {
        while i < size {
            let idx = (*s).fifo_rindex;
            // raw pointer arithmetic: no bounds checks / panics on C data
            let src = (*s).fifo_buf.as_mut_ptr().offset(idx as isize);
            *regs.offset(i as isize) = *src;
            (*s).fifo_rindex = idx + 1;
            if (*s).fifo_rindex == 64 {
                (*s).fifo_rindex = 0;
            }
            i += 1;
        }
    }
}
}

// ── `s->fifo_count -= size;` · vmmouse.c:76-76 · chunk 3936 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->fifo_count -= size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3936 {
// Port of `s->fifo_count -= size;` from vmmouse.c read_data().
// VMMouseState mirror (exact C field order / types).

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: core::ffi::c_int,
    pub fifo_rindex: core::ffi::c_int,
    pub fifo_windex: core::ffi::c_int,
    pub enabled: core::ffi::c_int,  /* BOOL */
    pub absolute: core::ffi::c_int, /* BOOL */
    pub fifo_buf: [u32; 4 * 16],
}

#[no_mangle]
pub unsafe extern "C" fn read_data__part3936(s: *mut VMMouseState, size: core::ffi::c_int) {
    if s.is_null() {
        return;
    }
    let count = (*s).fifo_count;
    (*s).fifo_count = count.wrapping_sub(size);
}
}

// ── `vmmouse_send_mouse_event` · vmmouse.c:79-112 · chunk 3937 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void vmmouse_send_mouse_event(VMMouseState *s, int x, int y, int dz,
//                                   int buttons)
//     {
//         int state;
//     
//         if (!s->enabled) {
//             ps2_mouse_event(s->ps2_mouse, x, y, dz, buttons);
//             return;
//         }
//     
//         if ((s->fifo_count + 4) > (4 * 16))
//             return;
//     
//         state = 0;
//         if (buttons & 1)
//             state |= 0x20;
//         if (buttons & 2)
//             state |= 0x10;
//         if (buttons & 4)
//             state |= 0x08;
//         if (s->absolute) {
//     
//             x *= 2;
//             y *= 2;
//         }
//     
//         put_queue(s, state);
//         put_queue(s, x);
//         put_queue(s, y);
//         put_queue(s, -dz);
//     
//     
//         ps2_mouse_event(s->ps2_mouse, 1, 0, 0, 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3937 {
use core::ffi::{c_int, c_uint};

const FIFO_SIZE: usize = 4 * 16;

#[repr(C)]
pub struct PS2MouseState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut PS2MouseState,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [c_uint; FIFO_SIZE],
}

extern "C" {
    fn ps2_mouse_event(
        s: *mut PS2MouseState,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons_state: c_int,
    );
}

// Port of the static C helper put_queue().
unsafe fn put_queue(s: *mut VMMouseState, val: c_uint) {
    if (*s).fifo_count >= FIFO_SIZE as c_int {
        return;
    }
    let windex = (*s).fifo_windex;
    if windex >= 0 && (windex as usize) < FIFO_SIZE {
        (*s).fifo_buf[windex as usize] = val;
    } else {
        return;
    }
    let mut w = (*s).fifo_windex.wrapping_add(1);
    if w == FIFO_SIZE as c_int {
        w = 0;
    }
    (*s).fifo_windex = w;
    (*s).fifo_count = (*s).fifo_count.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn vmmouse_send_mouse_event(
    s: *mut VMMouseState,
    x: c_int,
    y: c_int,
    dz: c_int,
    buttons: c_int,
) {
    let mut x = x;
    let mut y = y;

    if (*s).enabled == 0 {
        ps2_mouse_event((*s).ps2_mouse, x, y, dz, buttons);
        return;
    }

    if (*s).fifo_count.wrapping_add(4) > FIFO_SIZE as c_int {
        return;
    }

    let mut state: c_int = 0;
    if buttons & 1 != 0 {
        state |= 0x20;
    }
    if buttons & 2 != 0 {
        state |= 0x10;
    }
    if buttons & 4 != 0 {
        state |= 0x08;
    }
    if (*s).absolute != 0 {
        // range = 0 ... 65535
        x = x.wrapping_mul(2);
        y = y.wrapping_mul(2);
    }

    put_queue(s, state as c_uint);
    put_queue(s, x as c_uint);
    put_queue(s, y as c_uint);
    put_queue(s, dz.wrapping_neg() as c_uint);

    // send PS/2 mouse event
    ps2_mouse_event((*s).ps2_mouse, 1, 0, 0, 0);
}
}

// ── `if (!s->enabled)` · vmmouse.c:84-87 · chunk 3938 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s->enabled) {
//             ps2_mouse_event(s->ps2_mouse, x, y, dz, buttons);
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3938 {
use core::ffi::c_int;

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut PS2MouseState,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

extern "C" {
    fn ps2_mouse_event(
        s: *mut PS2MouseState,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons_state: c_int,
    );
}

// Returns 1 when the C caller must return from vmmouse_send_mouse_event,
// 0 otherwise.
#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3938(
    s: *mut VMMouseState,
    x: c_int,
    y: c_int,
    dz: c_int,
    buttons: c_int,
) -> c_int {
    unsafe {
        if (*s).enabled == 0 {
            ps2_mouse_event((*s).ps2_mouse, x, y, dz, buttons);
            return 1;
        }
    }
    0
}
}

// ── `if (s->fifo_count + 4) > (4 * 16)` · vmmouse.c:89-90 · chunk 3939 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if ((s->fifo_count + 4) > (4 * 16))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3939 {
use core::ffi::c_int;

#[repr(C)]
pub struct PS2MouseState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut PS2MouseState,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3939(s: *mut VMMouseState) -> c_int {
    unsafe {
        if s.is_null() {
            return 0;
        }
        let fifo_count = (*s).fifo_count;
        if fifo_count.wrapping_add(4) > 4 * 16 {
            1
        } else {
            0
        }
    }
}
}

// ── `state = 0;` · vmmouse.c:92-92 · chunk 3940 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     state = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3940 {
// Port of the assignment `state = 0;` from vmmouse_send_mouse_event.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3940(state: *mut c_int) {
    if !state.is_null() {
        unsafe {
            *state = 0;
        }
    }
}
}

// ── `if (buttons & 1)` · vmmouse.c:93-94 · chunk 3941 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buttons & 1)
//             state |= 0x20;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3941 {
// Port of the button-bit test that sets bit 0x20 of the mouse event state.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3941(buttons: c_int, state: *mut c_int) {
    if buttons & 1 != 0 {
        unsafe {
            *state |= 0x20;
        }
    }
}
}

// ── `if (buttons & 2)` · vmmouse.c:95-96 · chunk 3942 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buttons & 2)
//             state |= 0x10;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3942 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3942(buttons: c_int, state: *mut c_int) {
    unsafe {
        if buttons & 2 != 0 {
            *state |= 0x10;
        }
    }
}
}

// ── `if (buttons & 4)` · vmmouse.c:97-98 · chunk 3943 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buttons & 4)
//             state |= 0x08;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3943 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vmmouse_send_mouse_event__part3943(state: *mut c_int, buttons: c_int) {
    unsafe {
        if buttons & 4 != 0 {
            *state |= 0x08;
        }
    }
}
}

// ── `if (s->absolute)` · vmmouse.c:99-103 · chunk 3944 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->absolute) {
//     
//             x *= 2;
//             y *= 2;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3944 {
use core::ffi::c_int;

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

#[no_mangle]
pub unsafe extern "C" fn vmmouse_send_mouse_event__part3944(
    s: *mut VMMouseState,
    x: *mut c_int,
    y: *mut c_int,
) {
    if s.is_null() || x.is_null() || y.is_null() {
        return;
    }
    if (*s).absolute != 0 {
        // range = 0 ... 65535
        *x = (*x).wrapping_mul(2);
        *y = (*y).wrapping_mul(2);
    }
}
}

// ── `put_queue(s, state); put_queue(s, x); put_queue(s, y); put_queue(s, -dz); ps2_mouse_event(s->ps…` · vmmouse.c:105-111 · chunk 3945 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_queue(s, state);
//         put_queue(s, x);
//         put_queue(s, y);
//         put_queue(s, -dz);
//     
//     
//         ps2_mouse_event(s->ps2_mouse, 1, 0, 0, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3945 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 64],
}

extern "C" {
    fn ps2_mouse_event(
        s: *mut c_void,
        dx: c_int,
        dy: c_int,
        dz: c_int,
        buttons_state: c_int,
    );
}

unsafe fn put_queue(s: *mut VMMouseState, val: u32) {
    if (*s).fifo_count >= 64 {
        return;
    }
    let idx = (*s).fifo_windex as usize;
    match (*s).fifo_buf.get_mut(idx) {
        Some(slot) => *slot = val,
        None => return,
    }
    (*s).fifo_windex += 1;
    if (*s).fifo_windex == 64 {
        (*s).fifo_windex = 0;
    }
    (*s).fifo_count += 1;
}

#[no_mangle]
pub unsafe extern "C" fn vmmouse_send_mouse_event__part3945(
    s: *mut VMMouseState,
    state: c_int,
    x: c_int,
    y: c_int,
    dz: c_int,
) {
    put_queue(s, state as u32);
    put_queue(s, x as u32);
    put_queue(s, y as u32);
    put_queue(s, dz.wrapping_neg() as u32);

    ps2_mouse_event((*s).ps2_mouse, 1, 0, 0, 0);
}
}

// ── `vmmouse_handler` · vmmouse.c:114-149 · chunk 3946 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void vmmouse_handler(VMMouseState *s, uint32_t *regs)
//     {
//         uint32_t cmd;
//     
//         cmd = regs[2] & 0xff;
//         switch(cmd) {
//         case 10:
//             regs[1] = 0x564D5868;
//             break;
//         case 39:
//             read_data(s, regs, regs[1]);
//             break;
//         case 40:
//             regs[0] = ((s->enabled ? 0 : 0xffff) << 16) | s->fifo_count;
//             break;
//         case 41:
//             switch(regs[1]) {
//             case 0x45414552:
//                 if (s->fifo_count < (4 * 16)) {
//                     put_queue(s, 0x3442554a);
//                     s->enabled = TRUE;
//                 }
//                 break;
//             case 0x000000f5:
//                 s->enabled = FALSE;
//                 break;
//             case 0x4c455252:
//                 s->absolute = 0;
//                 break;
//             case 0x53424152:
//                 s->absolute = 1;
//                 break;
//             }
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3946 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 4 * 16],
}

#[inline]
unsafe fn put_queue(s: *mut VMMouseState, val: u32) {
    if (*s).fifo_count >= (4 * 16) {
        return;
    }
    let idx = (*s).fifo_windex;
    if idx >= 0 && idx < (4 * 16) {
        *(*s).fifo_buf.as_mut_ptr().offset(idx as isize) = val;
    }
    (*s).fifo_windex += 1;
    if (*s).fifo_windex == (4 * 16) {
        (*s).fifo_windex = 0;
    }
    (*s).fifo_count += 1;
}

unsafe fn read_data(s: *mut VMMouseState, regs: *mut u32, size: c_int) {
    if size > 6 || size > (*s).fifo_count {
        (*s).enabled = 0;
        return;
    }
    let mut i: c_int = 0;
    while i < size {
        let ridx = (*s).fifo_rindex;
        let v = if ridx >= 0 && ridx < (4 * 16) {
            *(*s).fifo_buf.as_ptr().offset(ridx as isize)
        } else {
            // Out-of-range index cannot occur with the C logic above; keep it safe.
            0
        };
        *regs.offset(i as isize) = v;
        (*s).fifo_rindex += 1;
        if (*s).fifo_rindex == (4 * 16) {
            (*s).fifo_rindex = 0;
        }
        i += 1;
    }
    (*s).fifo_count -= size;
}

#[no_mangle]
pub unsafe extern "C" fn vmmouse_handler(s: *mut VMMouseState, regs: *mut u32) {
    let cmd: u32 = *regs.offset(2) & 0xff;
    match cmd {
        10 => {
            *regs.offset(1) = 0x564D5868;
        }
        39 => {
            let size = *regs.offset(1) as c_int;
            read_data(s, regs, size);
        }
        40 => {
            let hi: u32 = if (*s).enabled != 0 { 0u32 } else { 0xffffu32 };
            *regs.offset(0) = (hi << 16) | ((*s).fifo_count as u32);
        }
        41 => match *regs.offset(1) {
            0x45414552 => {
                if (*s).fifo_count < (4 * 16) {
                    put_queue(s, 0x3442554a);
                    (*s).enabled = 1;
                }
            }
            0x000000f5 => {
                (*s).enabled = 0;
            }
            0x4c455252 => {
                (*s).absolute = 0;
            }
            0x53424152 => {
                (*s).absolute = 1;
            }
            _ => {}
        },
        _ => {}
    }
}
}

// ── `uint32_t cmd; cmd = regs[2] & 0xff;` · vmmouse.c:116-118 · chunk 3947 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t cmd;
//     
//         cmd = regs[2] & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3947 {
// Extract the VM port command number from the register block.
// TARGET: uint32_t cmd; cmd = regs[2] & 0xff;

#[no_mangle]
pub extern "C" fn vmmouse_handler__part3947(regs: *const u32) -> u32 {
    unsafe { core::ptr::read_volatile(regs.add(2)) & 0xff }
}
}

// ── `switch (cmd)` · vmmouse.c:119-148 · chunk 3948 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(cmd) {
//         case 10:
//             regs[1] = 0x564D5868;
//             break;
//         case 39:
//             read_data(s, regs, regs[1]);
//             break;
//         case 40:
//             regs[0] = ((s->enabled ? 0 : 0xffff) << 16) | s->fifo_count;
//             break;
//         case 41:
//             switch(regs[1]) {
//             case 0x45414552:
//                 if (s->fifo_count < (4 * 16)) {
//                     put_queue(s, 0x3442554a);
//                     s->enabled = TRUE;
//                 }
//                 break;
//             case 0x000000f5:
//                 s->enabled = FALSE;
//                 break;
//             case 0x4c455252:
//                 s->absolute = 0;
//                 break;
//             case 0x53424152:
//                 s->absolute = 1;
//                 break;
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3948 {
// Port of the command-dispatch switch of vmmouse_handler.

use core::ffi::{c_int, c_void};

const FIFO_SIZE: c_int = 4 * 16;

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; FIFO_SIZE as usize],
}

// Ported from the static C helper put_queue().
unsafe fn put_queue(s: *mut VMMouseState, val: u32) {
    let st = &mut *s;
    if st.fifo_count >= FIFO_SIZE {
        return;
    }
    let widx = st.fifo_windex as usize;
    if widx < st.fifo_buf.len() {
        st.fifo_buf[widx] = val;
    }
    st.fifo_windex += 1;
    if st.fifo_windex == FIFO_SIZE {
        st.fifo_windex = 0;
    }
    st.fifo_count += 1;
}

// Ported from the static C helper read_data().
unsafe fn read_data(s: *mut VMMouseState, regs: *mut u32, size: c_int) {
    let st = &mut *s;
    if size > 6 || size > st.fifo_count {
        st.enabled = 0;
        return;
    }
    let mut i: c_int = 0;
    while i < size {
        let ridx = st.fifo_rindex as usize;
        let v = if ridx < st.fifo_buf.len() {
            st.fifo_buf[ridx]
        } else {
            0
        };
        *regs.offset(i as isize) = v;
        st.fifo_rindex += 1;
        if st.fifo_rindex == FIFO_SIZE {
            st.fifo_rindex = 0;
        }
        i += 1;
    }
    st.fifo_count -= size;
}

#[no_mangle]
pub extern "C" fn vmmouse_handler__part3948(s: *mut VMMouseState, regs: *mut u32, cmd: u32) {
    unsafe {
        match cmd {
            10 => {
                *regs.offset(1) = 0x564D5868u32;
            }
            39 => {
                let size = *regs.offset(1) as c_int;
                read_data(s, regs, size);
            }
            40 => {
                let enabled = (*s).enabled;
                let hi: u32 = if enabled != 0 { 0 } else { 0xffffu32 << 16 };
                *regs.offset(0) = hi | ((*s).fifo_count as u32);
            }
            41 => {
                match *regs.offset(1) {
                    0x45414552 => {
                        if (*s).fifo_count < FIFO_SIZE {
                            put_queue(s, 0x3442554a);
                            (*s).enabled = 1;
                        }
                    }
                    0x000000f5 => {
                        (*s).enabled = 0;
                    }
                    0x4c455252 => {
                        (*s).absolute = 0;
                    }
                    0x53424152 => {
                        (*s).absolute = 1;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
}

// ── `vmmouse_is_absolute` · vmmouse.c:151-154 · chunk 3949 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL vmmouse_is_absolute(VMMouseState *s)
//     {
//         return s->absolute;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3949 {
// Rewrite of vmmouse_is_absolute from vmmouse.c
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut core::ffi::c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [c_uint; 4 * 16],
}

#[no_mangle]
pub extern "C" fn vmmouse_is_absolute(s: *mut VMMouseState) -> c_int {
    unsafe { (*s).absolute }
}
}

// ── `return s->absolute;` · vmmouse.c:153-153 · chunk 3950 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return s->absolute;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3950 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut c_void,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 4 * 16],
}

#[no_mangle]
pub extern "C" fn vmmouse_is_absolute__part3950(s: *mut VMMouseState) -> c_int {
    unsafe { (*s).absolute }
}
}

// ── `vmmouse_init` · vmmouse.c:156-162 · chunk 3951 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VMMouseState *vmmouse_init(PS2MouseState *ps2_mouse)
//     {
//         VMMouseState *s;
//         s = mallocz(sizeof(*s));
//         s->ps2_mouse = ps2_mouse;
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3951 {
// Rewrite of vmmouse_init() from vmmouse.c
use core::ffi::{c_int, c_void};

// Mirror of `struct VMMouseState` from vmmouse.c (PS2MouseState is opaque here).
#[repr(C)]
pub struct VMMouseState {
    ps2_mouse: *mut c_void,
    fifo_count: c_int,
    fifo_rindex: c_int,
    fifo_windex: c_int,
    enabled: c_int,
    absolute: c_int,
    fifo_buf: [u32; 4 * 16],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn vmmouse_init(ps2_mouse: *mut c_void) -> *mut VMMouseState {
    unsafe {
        let s = mallocz(core::mem::size_of::<VMMouseState>()) as *mut VMMouseState;
        if !s.is_null() {
            (*s).ps2_mouse = ps2_mouse;
        }
        s
    }
}
}

// ── `VMMouseState *s; s = mallocz(sizeof(*s)); s->ps2_mouse = ps2_mouse; return s;` · vmmouse.c:158-161 · chunk 3952 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VMMouseState *s;
//         s = mallocz(sizeof(*s));
//         s->ps2_mouse = ps2_mouse;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3952 {
use core::ffi::{c_int, c_void};

// Opaque: only ever handled through a pointer here.
#[repr(C)]
pub struct PS2MouseState {
    _opaque: [u8; 0],
}

// Mirror of `struct VMMouseState` in vmmouse.c.
#[repr(C)]
pub struct VMMouseState {
    pub ps2_mouse: *mut PS2MouseState,
    pub fifo_count: c_int,
    pub fifo_rindex: c_int,
    pub fifo_windex: c_int,
    pub enabled: c_int,
    pub absolute: c_int,
    pub fifo_buf: [u32; 4 * 16],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn vmmouse_init__part3952(
    ps2_mouse: *mut PS2MouseState,
) -> *mut VMMouseState {
    let s = mallocz(core::mem::size_of::<VMMouseState>()) as *mut VMMouseState;
    if !s.is_null() {
        (*s).ps2_mouse = ps2_mouse;
    }
    s
}
}

