//! From `ide.c`.

// ── `padstr` · ide.c:236-247 · chunk 1167 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void padstr(char *str, const char *src, int len)
//     {
//         int i, v;
//         for(i = 0; i < len; i++) {
//             if (*src)
//                 v = *src++;
//             else
//                 v = ' ';
//             *(char *)((long)str ^ 1) = v;
//             str++;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1167 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn padstr(str_: *mut c_char, src: *const c_char, len: c_int) {
    let mut dst = str_;
    let mut p = src;
    let mut i: c_int = 0;
    while i < len {
        let v: c_char = unsafe {
            if *p != 0 {
                let b = *p;
                p = p.add(1);
                b
            } else {
                b' ' as c_char
            }
        };
        unsafe {
            let addr: isize = dst as isize ^ 1;
            *(addr as *mut c_char) = v;
            dst = dst.add(1);
        }
        i += 1;
    }
}
}

// ── `for (i = 0; i < len; i++)` · ide.c:239-246 · chunk 1168 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < len; i++) {
//             if (*src)
//                 v = *src++;
//             else
//                 v = ' ';
//             *(char *)((long)str ^ 1) = v;
//             str++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1168 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub unsafe extern "C" fn padstr__part1168(
    mut str_: *mut c_char,
    mut src: *const c_char,
    len: c_int,
) {
    let mut i: c_int = 0;
    while i < len {
        let v: c_int = if *src != 0 {
            let c: c_int = *src as c_int;
            src = src.offset(1);
            c
        } else {
            b' ' as c_int
        };
        let p: *mut c_char = ((str_ as isize) ^ 1) as *mut c_char;
        *p = v as c_char;
        str_ = str_.offset(1);
        i += 1;
    }
}
}

// ── `stw` · ide.c:250-253 · chunk 1169 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void stw(uint16_t *buf, int v)
//     {
//         *buf = v;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1169 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn stw(buf: *mut u16, v: c_int) {
    unsafe {
        *buf = v as u16;
    }
}
}

// ── `*buf = v;` · ide.c:252-252 · chunk 1170 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *buf = v;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1170 {
// Rust rewrite of the statement `*buf = v;` inside the static C function stw.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn stw__part1170(buf: *mut u16, v: c_int) {
    unsafe {
        *buf = v as u16;
    }
}
}

// ── `ide_identify` · ide.c:255-296 · chunk 1171 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_identify(IDEState *s)
//     {
//         uint16_t *tab;
//         uint32_t oldsize;
//     
//         tab = (uint16_t *)s->io_buffer;
//     
//         memset(tab, 0, 512 * 2);
//     
//         stw(tab + 0, 0x0040);
//         stw(tab + 1, s->cylinders);
//         stw(tab + 3, s->heads);
//         stw(tab + 4, 512 * s->sectors);
//         stw(tab + 5, 512);
//         stw(tab + 6, s->sectors);
//         stw(tab + 20, 3);
//         stw(tab + 21, 512);
//         stw(tab + 22, 4);
//         padstr((char *)(tab + 27), "RISCVEMU HARDDISK", 40);
//         stw(tab + 47, 0x8000 | 128);
//         stw(tab + 48, 0);
//         stw(tab + 49, 1 << 9);
//         stw(tab + 51, 0x200);
//         stw(tab + 52, 0x200);
//         stw(tab + 54, s->cylinders);
//         stw(tab + 55, s->heads);
//         stw(tab + 56, s->sectors);
//         oldsize = s->cylinders * s->heads * s->sectors;
//         stw(tab + 57, oldsize);
//         stw(tab + 58, oldsize >> 16);
//         if (s->mult_sectors)
//             stw(tab + 59, 0x100 | s->mult_sectors);
//         stw(tab + 60, s->nb_sectors);
//         stw(tab + 61, s->nb_sectors >> 16);
//         stw(tab + 80, (1 << 1) | (1 << 2));
//         stw(tab + 82, (1 << 14));
//         stw(tab + 83, (1 << 14));
//         stw(tab + 84, (1 << 14));
//         stw(tab + 85, (1 << 14));
//         stw(tab + 86, 0);
//         stw(tab + 87, (1 << 14));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1171 {
// Port of static void ide_identify(IDEState *s)
#![allow(non_camel_case_types, dead_code)]

use core::ffi::c_int;

#[repr(C)]
pub struct IDEIFState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BlockDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IDEState {
    ide_if: *mut IDEIFState,
    bs: *mut BlockDevice,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut core::ffi::c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

// Port of the static helper `stw`: *buf = v (v is the C `int` argument,
// truncated to 16 bits on store).
#[inline]
unsafe fn stw(tab: *mut u16, idx: usize, v: i32) {
    core::ptr::write_unaligned(tab.add(idx), v as u16);
}

#[no_mangle]
pub unsafe extern "C" fn ide_identify(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;

    let tab = s.io_buffer.as_mut_ptr() as *mut u16;

    core::ptr::write_bytes(tab as *mut u8, 0, 512 * 2);

    stw(tab, 0, 0x0040);
    stw(tab, 1, s.cylinders);
    stw(tab, 3, s.heads);
    stw(tab, 4, 512i32.wrapping_mul(s.sectors));
    stw(tab, 5, 512);
    stw(tab, 6, s.sectors);
    stw(tab, 20, 3);
    stw(tab, 21, 512);
    stw(tab, 22, 4);

    // padstr((char *)(tab + 27), "RISCVEMU HARDDISK", 40);
    {
        let base = tab as *mut u8;
        let strp = base.add(27 * 2);
        let src: &[u8] = b"RISCVEMU HARDDISK";
        let mut i: usize = 0;
        while i < 40 {
            let v: u8 = if i < src.len() { src[i] } else { b' ' };
            core::ptr::write_unaligned(strp.add(i ^ 1), v);
            i += 1;
        }
    }

    stw(tab, 47, 0x8000 | 128);
    stw(tab, 48, 0);
    stw(tab, 49, 1 << 9);
    stw(tab, 51, 0x200);
    stw(tab, 52, 0x200);
    stw(tab, 54, s.cylinders);
    stw(tab, 55, s.heads);
    stw(tab, 56, s.sectors);
    let oldsize: u32 = s
        .cylinders
        .wrapping_mul(s.heads)
        .wrapping_mul(s.sectors) as u32;
    stw(tab, 57, oldsize as i32);
    stw(tab, 58, (oldsize >> 16) as i32);
    if s.mult_sectors != 0 {
        stw(tab, 59, 0x100 | s.mult_sectors);
    }
    stw(tab, 60, s.nb_sectors as i32);
    stw(tab, 61, (s.nb_sectors >> 16) as i32);
    stw(tab, 80, (1 << 1) | (1 << 2));
    stw(tab, 82, 1 << 14);
    stw(tab, 83, 1 << 14);
    stw(tab, 84, 1 << 14);
    stw(tab, 85, 1 << 14);
    stw(tab, 86, 0);
    stw(tab, 87, 1 << 14);
}
}

// ── `uint16_t *tab; uint32_t oldsize; tab = (uint16_t *)s->io_buffer; memset(tab, 0, 512 * 2); stw(t…` · ide.c:257-284 · chunk 1172 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint16_t *tab;
//         uint32_t oldsize;
//     
//         tab = (uint16_t *)s->io_buffer;
//     
//         memset(tab, 0, 512 * 2);
//     
//         stw(tab + 0, 0x0040);
//         stw(tab + 1, s->cylinders);
//         stw(tab + 3, s->heads);
//         stw(tab + 4, 512 * s->sectors);
//         stw(tab + 5, 512);
//         stw(tab + 6, s->sectors);
//         stw(tab + 20, 3);
//         stw(tab + 21, 512);
//         stw(tab + 22, 4);
//         padstr((char *)(tab + 27), "RISCVEMU HARDDISK", 40);
//         stw(tab + 47, 0x8000 | 128);
//         stw(tab + 48, 0);
//         stw(tab + 49, 1 << 9);
//         stw(tab + 51, 0x200);
//         stw(tab + 52, 0x200);
//         stw(tab + 54, s->cylinders);
//         stw(tab + 55, s->heads);
//         stw(tab + 56, s->sectors);
//         oldsize = s->cylinders * s->heads * s->sectors;
//         stw(tab + 57, oldsize);
//         stw(tab + 58, oldsize >> 16);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1172 {
// Port of ide_identify lines 257-284 (ATA IDENTIFY buffer fill, first half).
// The static helpers `stw` and `padstr` of ide.c are private in the C
// translation unit and have no linker symbol, so their logic is mirrored here.

use core::ffi::{c_int, c_void};

// Prefix of the C struct IDEState (only the fields this piece touches).
#[repr(C)]
struct IDEState {
    ide_if: *mut c_void, // IDEIFState *
    bs: *mut c_void,     // BlockDevice *
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
}

// static void stw(uint16_t *buf, int v) { *buf = v; }
#[inline]
unsafe fn stw(buf: *mut u16, v: c_int) {
    core::ptr::write(buf, v as u16);
}

// static void padstr(char *str, const char *src, int len)
#[inline]
unsafe fn padstr(mut str_: *mut u8, src: &[u8], len: c_int) {
    let mut i: c_int = 0;
    let mut idx: usize = 0;
    while i < len {
        let v: u8;
        if idx < src.len() && src[idx] != 0 {
            v = src[idx];
            idx += 1;
        } else {
            v = b' ';
        }
        let addr = (str_ as usize) ^ 1;
        core::ptr::write(addr as *mut u8, v);
        str_ = str_.wrapping_add(1);
        i += 1;
    }
}

const IDENT_STR: [u8; 18] = *b"RISCVEMU HARDDISK\0";

#[no_mangle]
pub unsafe extern "C" fn ide_identify__part1172(s: *mut IDEState, tab: *mut u16) {
    // memset(tab, 0, 512 * 2);
    core::ptr::write_bytes(tab as *mut u8, 0, 512 * 2);

    stw(tab.add(0), 0x0040);
    stw(tab.add(1), (*s).cylinders);
    stw(tab.add(3), (*s).heads);
    stw(tab.add(4), 512 * (*s).sectors);
    stw(tab.add(5), 512);
    stw(tab.add(6), (*s).sectors);
    stw(tab.add(20), 3);
    stw(tab.add(21), 512);
    stw(tab.add(22), 4);
    padstr(tab.add(27) as *mut u8, &IDENT_STR, 40);
    stw(tab.add(47), 0x8000 | 128);
    stw(tab.add(48), 0);
    stw(tab.add(49), 1 << 9);
    stw(tab.add(51), 0x200);
    stw(tab.add(52), 0x200);
    stw(tab.add(54), (*s).cylinders);
    stw(tab.add(55), (*s).heads);
    stw(tab.add(56), (*s).sectors);

    // uint32_t oldsize = s->cylinders * s->heads * s->sectors;
    let cyl = (*s).cylinders;
    let heads = (*s).heads;
    let sectors = (*s).sectors;
    let oldsize: u32 = cyl.wrapping_mul(heads).wrapping_mul(sectors) as u32;

    stw(tab.add(57), oldsize as c_int);
    stw(tab.add(58), (oldsize >> 16) as c_int);
}
}

// ── `if (s->mult_sectors)` · ide.c:285-286 · chunk 1173 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->mult_sectors)
//             stw(tab + 59, 0x100 | s->mult_sectors);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1173 {
use core::ffi::c_int;

// Port of the static helper `stw`: *buf = (uint16_t)v;  (the index is applied by the caller)
#[no_mangle]
pub extern "C" fn ide_identify__part1173(tab: *mut u16, mult_sectors: c_int) {
    if mult_sectors != 0 {
        // stw(tab + 59, 0x100 | s->mult_sectors);
        unsafe {
            *tab.add(59) = (0x100 | mult_sectors) as u16;
        }
    }
}
}

// ── `stw(tab + 60, s->nb_sectors); stw(tab + 61, s->nb_sectors >> 16); stw(tab + 80, (1 << 1) | (1 <…` · ide.c:287-295 · chunk 1174 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     stw(tab + 60, s->nb_sectors);
//         stw(tab + 61, s->nb_sectors >> 16);
//         stw(tab + 80, (1 << 1) | (1 << 2));
//         stw(tab + 82, (1 << 14));
//         stw(tab + 83, (1 << 14));
//         stw(tab + 84, (1 << 14));
//         stw(tab + 85, (1 << 14));
//         stw(tab + 86, 0);
//         stw(tab + 87, (1 << 14));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1174 {
// Port of the tail of ide_identify: fills IDENTIFY words 60..87 of the
// identify buffer. `stw` is static in the C file (no linker symbol), so its
// logic is mirrored here: it stores `v` into *buf with uint16_t truncation.

#[inline]
unsafe fn stw(buf: *mut u16, v: i32) {
    if !buf.is_null() {
        core::ptr::write_volatile(buf, v as u16);
    }
}

#[no_mangle]
pub extern "C" fn ide_identify__part1174(tab: *mut u16, nb_sectors: i64) {
    unsafe {
        // stw(tab + 60, s->nb_sectors);
        stw(tab.add(60), nb_sectors as i32);
        // stw(tab + 61, s->nb_sectors >> 16);
        stw(tab.add(61), (nb_sectors >> 16) as i32);
        // stw(tab + 80, (1 << 1) | (1 << 2));
        stw(tab.add(80), (1i32 << 1) | (1i32 << 2));
        // stw(tab + 82, (1 << 14));
        stw(tab.add(82), 1i32 << 14);
        // stw(tab + 83, (1 << 14));
        stw(tab.add(83), 1i32 << 14);
        // stw(tab + 84, (1 << 14));
        stw(tab.add(84), 1i32 << 14);
        // stw(tab + 85, (1 << 14));
        stw(tab.add(85), 1i32 << 14);
        // stw(tab + 86, 0);
        stw(tab.add(86), 0);
        // stw(tab + 87, (1 << 14));
        stw(tab.add(87), 1i32 << 14);
    }
}
}

// ── `ide_set_signature` · ide.c:298-305 · chunk 1175 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_set_signature(IDEState *s)
//     {
//         s->select &= 0xf0;
//         s->nsector = 1;
//         s->sector = 1;
//         s->lcyl = 0;
//         s->hcyl = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1175 {
// Port of IDE drive signature reset.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_set_signature(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    unsafe {
        let st = &mut *s;
        st.select &= 0xf0;
        st.nsector = 1;
        st.sector = 1;
        st.lcyl = 0;
        st.hcyl = 0;
    }
}
}

// ── `s->select &= 0xf0; s->nsector = 1; s->sector = 1; s->lcyl = 0; s->hcyl = 0;` · ide.c:300-304 · chunk 1176 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->select &= 0xf0;
//         s->nsector = 1;
//         s->sector = 1;
//         s->lcyl = 0;
//         s->hcyl = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1176 {
// Port of the fragment of ide_set_signature that resets the IDE task-file
// signature registers.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_set_signature__part1176(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    s.select &= 0xf0;
    s.nsector = 1;
    s.sector = 1;
    s.lcyl = 0;
    s.hcyl = 0;
}
}

// ── `ide_abort_command` · ide.c:307-311 · chunk 1177 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_abort_command(IDEState *s)
//     {
//         s->status = 0x40 | 0x01;
//         s->error = 0x04;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1177 {
// Port of ide.c: ide_abort_command
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

// Opaque handles for types not touched by this function.
pub type EndTransferFunc = Option<unsafe extern "C" fn(*mut IDEState)>;

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: EndTransferFunc,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_abort_command(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}
}

// ── `s->status = 0x40 | 0x01; s->error = 0x04;` · ide.c:309-310 · chunk 1178 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->status = 0x40 | 0x01;
//         s->error = 0x04;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1178 {
// Port of the two field assignments of ide_abort_command.
use core::ffi::{c_int, c_void};

#[repr(C)]
struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_abort_command__part1178(s: *mut IDEState) {
    unsafe {
        (*s).status = (0x40 | 0x01) as u8;
        (*s).error = 0x04u8;
    }
}
}

// ── `ide_set_irq` · ide.c:313-319 · chunk 1179 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_set_irq(IDEState *s)
//     {
//         IDEIFState *ide_if = s->ide_if;
//         if (!(ide_if->cmd & 0x02)) {
//             set_irq(ide_if->irq, 1);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1179 {
// Rust rewrite of the C function ide_set_irq (ide.c lines 313-319).
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

/// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type EndTransferFunc = unsafe extern "C" fn(s: *mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void, // BlockDevice *
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ide_set_irq(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    let ide_if = (*s).ide_if;
    if ide_if.is_null() {
        return;
    }
    // if (!(ide_if->cmd & IDE_CMD_DISABLE_IRQ)) set_irq(ide_if->irq, 1);
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        if !irq.is_null() {
            // static inline void set_irq(IRQSignal *irq, int level)
            if let Some(f) = (*irq).set_irq {
                f((*irq).opaque, (*irq).irq_num, 1);
            }
        }
    }
}
}

// ── `if (!(ide_if->cmd & 0x02)` · ide.c:316-318 · chunk 1180 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(ide_if->cmd & 0x02)) {
//             set_irq(ide_if->irq, 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1180 {
// Port of the TARGET: conditional IRQ assertion in ide_set_irq.
// `set_irq()` is a C `static inline`, so its body (a call through the
// function pointer in IRQSignal) is reproduced here.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ide_set_irq__part1180(ide_if: *mut IDEIFState) {
    if ide_if.is_null() {
        return;
    }
    let cmd = (*ide_if).cmd;
    if (cmd as c_int) & 0x02 == 0 {
        let irq = (*ide_if).irq;
        if !irq.is_null() {
            if let Some(f) = (*irq).set_irq {
                f((*irq).opaque, (*irq).irq_num, 1);
            }
        }
    }
}
}

// ── `ide_transfer_start` · ide.c:322-328 · chunk 1181 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_transfer_start(IDEState *s, int size,
//                                    EndTransferFunc *end_transfer_func)
//     {
//         s->end_transfer_func = end_transfer_func;
//         s->data_index = 0;
//         s->data_end = size;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1181 {
// Port of ide_transfer_start() from ide.c (static function from the target TU).
#![allow(non_camel_case_types)]

use core::ffi::c_int;

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut core::ffi::c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut core::ffi::c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

/* prepare data transfer and tell what to do after */
#[no_mangle]
pub unsafe extern "C" fn ide_transfer_start(
    s: *mut IDEState,
    size: c_int,
    end_transfer_func: Option<EndTransferFunc>,
) {
    let s = &mut *s;
    s.end_transfer_func = end_transfer_func;
    s.data_index = 0;
    s.data_end = size;
}
}

// ── `s->end_transfer_func = end_transfer_func; s->data_index = 0; s->data_end = size;` · ide.c:325-327 · chunk 1182 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->end_transfer_func = end_transfer_func;
//         s->data_index = 0;
//         s->data_end = size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1182 {
// Ported piece of ide_transfer_start: init transfer bookkeeping fields.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_transfer_start__part1182(
    s: *mut IDEState,
    size: c_int,
    end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
) {
    (*s).end_transfer_func = end_transfer_func;
    (*s).data_index = 0;
    (*s).data_end = size;
}
}

// ── `ide_transfer_stop` · ide.c:330-335 · chunk 1183 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_transfer_stop(IDEState *s)
//     {
//         s->end_transfer_func = ide_transfer_stop;
//         s->data_index = 0;
//         s->data_end = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1183 {
use core::ffi::{c_int};

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut core::ffi::c_void,
    pub bs: *mut core::ffi::c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).end_transfer_func = Some(ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}
}

// ── `s->end_transfer_func = ide_transfer_stop; s->data_index = 0; s->data_end = 0;` · ide.c:332-334 · chunk 1184 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->end_transfer_func = ide_transfer_stop;
//         s->data_index = 0;
//         s->data_end = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1184 {
// Port of the body of ide_transfer_stop (ide.c lines 332-334).
//
// The target stores the address of the (static, TU-local) ide_transfer_stop
// function into s->end_transfer_func.  Since that function has internal
// linkage and therefore no linker symbol, the C caller passes its address in.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

// Mirror of `struct IDEState` from the preprocessed translation unit, field
// for field and in order, so that member offsets match the C struct.
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,          // IDEIFState *
    pub bs: *mut c_void,              // BlockDevice *
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    // EndTransferFunc *  (pointer-sized; opaque to the Rust side)
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_transfer_stop__part1184(
    s: *mut IDEState,
    end_transfer_func: *mut c_void,
) {
    let s = &mut *s;
    s.end_transfer_func = end_transfer_func;
    s.data_index = 0;
    s.data_end = 0;
}
}

// ── `ide_get_sector` · ide.c:337-350 · chunk 1185 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int64_t ide_get_sector(IDEState *s)
//     {
//         int64_t sector_num;
//         if (s->select & 0x40) {
//     
//             sector_num = ((s->select & 0x0f) << 24) | (s->hcyl << 16) |
//                 (s->lcyl << 8) | s->sector;
//         } else {
//             sector_num = ((s->hcyl << 8) | s->lcyl) *
//                 s->heads * s->sectors +
//                 (s->select & 0x0f) * s->sectors + (s->sector - 1);
//         }
//         return sector_num;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1185 {
// Rust rewrite of ide.c:ide_get_sector
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

#[repr(C)]
struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_get_sector(s: *mut IDEState) -> i64 {
    // Mirrors the C code: all arithmetic is performed in `int` (i32) and the
    // result is then converted to int64_t.
    unsafe {
        let s = &*s;
        if s.select & 0x40 != 0 {
            // lba
            let v: i32 = (((s.select & 0x0f) as i32) << 24)
                | ((s.hcyl as i32) << 16)
                | ((s.lcyl as i32) << 8)
                | (s.sector as i32);
            v as i64
        } else {
            let cyl: i32 = ((s.hcyl as i32) << 8) | (s.lcyl as i32);
            let v: i32 = cyl
                .wrapping_mul(s.heads)
                .wrapping_mul(s.sectors)
                .wrapping_add(((s.select & 0x0f) as i32).wrapping_mul(s.sectors))
                .wrapping_add((s.sector as i32).wrapping_sub(1));
            v as i64
        }
    }
}
}

// ── `if (s->select & 0x40) … else` · ide.c:340-348 · chunk 1186 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->select & 0x40) {
//     
//             sector_num = ((s->select & 0x0f) << 24) | (s->hcyl << 16) |
//                 (s->lcyl << 8) | s->sector;
//         } else {
//             sector_num = ((s->hcyl << 8) | s->lcyl) *
//                 s->heads * s->sectors +
//                 (s->select & 0x0f) * s->sectors + (s->sector - 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1186 {
// Rust port of the cylinder/head/sector decoding branch of ide_get_sector().
use core::ffi::{c_int, c_longlong, c_void};

// Mirror of the C `struct IDEState` fields, in order, with identical types.
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: c_longlong,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Computes the sector number for the current CHS/LBA register contents and
// stores it through `out` (C keeps `sector_num` as a local of the caller).
#[no_mangle]
pub unsafe extern "C" fn ide_get_sector__part1186(s: *mut IDEState, out: *mut c_longlong) {
    if s.is_null() || out.is_null() {
        return;
    }
    let s = &*s;

    let value: c_longlong = if (s.select & 0x40) != 0 {
        // LBA mode: all arithmetic stays in int, as in C.
        let v: c_int = (((s.select & 0x0f) as c_int) << 24)
            | ((s.hcyl as c_int) << 16)
            | ((s.lcyl as c_int) << 8)
            | (s.sector as c_int);
        v as c_longlong
    } else {
        // CHS mode: the product is evaluated with int wrap-around semantics.
        let cyl: c_int = ((s.hcyl as c_int) << 8) | (s.lcyl as c_int);
        let base: c_int = cyl.wrapping_mul(s.heads).wrapping_mul(s.sectors);
        let head_part: c_int = ((s.select & 0x0f) as c_int).wrapping_mul(s.sectors);
        let sect_part: c_int = (s.sector as c_int) - 1;
        base.wrapping_add(head_part).wrapping_add(sect_part) as c_longlong
    };

    *out = value;
}
}

// ── `return sector_num;` · ide.c:349-349 · chunk 1187 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return sector_num;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1187 {
// Returns the sector number computed by ide_get_sector.
#[no_mangle]
pub extern "C" fn ide_get_sector__part1187(sector_num: i64) -> i64 {
    sector_num
}
}

// ── `ide_set_sector` · ide.c:352-368 · chunk 1188 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_set_sector(IDEState *s, int64_t sector_num)
//     {
//         unsigned int cyl, r;
//         if (s->select & 0x40) {
//             s->select = (s->select & 0xf0) | ((sector_num >> 24) & 0x0f);
//             s->hcyl = (sector_num >> 16) & 0xff;
//             s->lcyl = (sector_num >> 8) & 0xff;
//             s->sector = sector_num & 0xff;
//         } else {
//             cyl = sector_num / (s->heads * s->sectors);
//             r = sector_num % (s->heads * s->sectors);
//             s->hcyl = (cyl >> 8) & 0xff;
//             s->lcyl = cyl & 0xff;
//             s->select = (s->select & 0xf0) | ((r / s->sectors) & 0x0f);
//             s->sector = (r % s->sectors) + 1;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1188 {
use core::ffi::{c_int, c_uint};

// Mirror of the fields of `struct IDEState` that this function touches.
// Only the prefix up to the accessed fields matters for offsets, so the full
// struct is mirrored exactly as declared in the C translation unit.
#[repr(C)]
pub struct IDEState {
    ide_if: *mut core::ffi::c_void,
    bs: *mut core::ffi::c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut core::ffi::c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    if (s.select & 0x40) != 0 {
        s.select = (s.select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        s.hcyl = ((sector_num >> 16) & 0xff) as u8;
        s.lcyl = ((sector_num >> 8) & 0xff) as u8;
        s.sector = (sector_num & 0xff) as u8;
    } else {
        // C computes `s->heads * s->sectors` in int arithmetic.
        let denom: c_int = s.heads.wrapping_mul(s.sectors);
        // C division/modulo by zero is undefined; keep it from panicking so no
        // unwinding can cross the FFI boundary.
        let cyl: c_uint;
        let r: c_uint;
        if denom == 0 {
            cyl = 0;
            r = 0;
        } else {
            let d = denom as i64;
            cyl = sector_num.wrapping_div(d) as c_uint;
            r = sector_num.wrapping_rem(d) as c_uint;
        }
        s.hcyl = ((cyl >> 8) & 0xff) as u8;
        s.lcyl = (cyl & 0xff) as u8;
        // `r / s->sectors` with int operands; guard against a zero divisor.
        let sec = s.sectors;
        if sec == 0 {
            s.select = (s.select & 0xf0) | ((r & 0x0f) as u8);
            s.sector = 1;
        } else {
            let ds = sec as i64;
            let q = (r as i64).wrapping_div(ds) as c_uint;
            let m = (r as i64).wrapping_rem(ds) as c_uint;
            s.select = (s.select & 0xf0) | ((q & 0x0f) as u8);
            s.sector = ((m & 0xff) as u8).wrapping_add(1);
        }
    }
}
}

// ── `if (s->select & 0x40) … else` · ide.c:355-367 · chunk 1189 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->select & 0x40) {
//             s->select = (s->select & 0xf0) | ((sector_num >> 24) & 0x0f);
//             s->hcyl = (sector_num >> 16) & 0xff;
//             s->lcyl = (sector_num >> 8) & 0xff;
//             s->sector = sector_num & 0xff;
//         } else {
//             cyl = sector_num / (s->heads * s->sectors);
//             r = sector_num % (s->heads * s->sectors);
//             s->hcyl = (cyl >> 8) & 0xff;
//             s->lcyl = cyl & 0xff;
//             s->select = (s->select & 0xf0) | ((r / s->sectors) & 0x0f);
//             s->sector = (r % s->sectors) + 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1189 {
// Port of the CHS/LBA register update branch of ide_set_sector.
use core::ffi::{c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
pub struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_set_sector__part1189(s: *mut IDEState, sector_num: i64) {
    unsafe {
        let s = &mut *s;
        if (s.select as c_int) & 0x40 != 0 {
            s.select = (s.select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
            s.hcyl = ((sector_num >> 16) & 0xff) as u8;
            s.lcyl = ((sector_num >> 8) & 0xff) as u8;
            s.sector = (sector_num & 0xff) as u8;
        } else {
            // C: `sector_num / (s->heads * s->sectors)`; the denominator is an
            // int*int computed with wrapping semantics, then widened to int64_t.
            let denom = s.heads.wrapping_mul(s.sectors) as i64;
            if denom != 0 {
                let cyl = sector_num.wrapping_div(denom) as u64 as u32;
                let r = sector_num.wrapping_rem(denom) as u64 as u32;
                s.hcyl = ((cyl >> 8) & 0xff) as u8;
                s.lcyl = (cyl & 0xff) as u8;
                let sec = s.sectors as u32;
                if sec != 0 {
                    s.select = (s.select & 0xf0) | (((r / sec) & 0x0f) as u8);
                    s.sector = ((r % sec) + 1) as u8;
                } else {
                    s.select &= 0xf0;
                }
            }
        }
    }
}
}

// ── `ide_sector_read` · ide.c:370-399 · chunk 1190 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_read(IDEState *s)
//     {
//         int64_t sector_num;
//         int ret, n;
//     
//         sector_num = ide_get_sector(s);
//         n = s->nsector;
//         if (n == 0)
//             n = 256;
//         if (n > s->req_nb_sectors)
//             n = s->req_nb_sectors;
//     
//     
//     
//         s->io_nb_sectors = n;
//         ret = s->bs->read_async(s->bs, sector_num, s->io_buffer, n,
//                                 ide_sector_read_cb, s);
//         if (ret < 0) {
//     
//             ide_abort_command(s);
//             ide_set_irq(s);
//         } else if (ret == 0) {
//     
//             ide_sector_read_cb(s, 0);
//         } else {
//     
//             s->status = 0x40 | 0x10 | 0x80;
//             s->error = 0;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1190 {
// Port of ide_sector_read (ide.c) together with the static helpers it needs
// (ide_get_sector, ide_set_sector, ide_abort_command, ide_set_irq,
// ide_transfer_start/stop and the ide_sector_read_cb/cb_end callbacks).
// Those C functions have internal linkage and therefore no linker symbol, so
// their logic is reproduced here faithfully.

use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// static int64_t ide_get_sector(IDEState *s)
unsafe fn rs_ide_get_sector(s: *mut IDEState) -> i64 {
    let st = &*s;
    if (st.select & 0x40) != 0 {
        // sector_num = ((select & 0x0f) << 24) | (hcyl << 16) |
        //             (lcyl << 8) | sector;   /* int arithmetic */
        let v: i32 = ((st.select as i32 & 0x0f) << 24)
            | ((st.hcyl as i32) << 16)
            | ((st.lcyl as i32) << 8)
            | (st.sector as i32);
        v as i64
    } else {
        let cyl_part: i32 = ((st.hcyl as i32) << 8) | (st.lcyl as i32);
        let v: i32 = cyl_part
            .wrapping_mul(st.heads)
            .wrapping_mul(st.sectors)
            .wrapping_add((st.select as i32 & 0x0f).wrapping_mul(st.sectors))
            .wrapping_add((st.sector as i32).wrapping_sub(1));
        v as i64
    }
}

// static void ide_set_sector(IDEState *s, int64_t sector_num)
unsafe fn rs_ide_set_sector(s: *mut IDEState, sector_num: i64) {
    let st = &mut *s;
    if (st.select & 0x40) != 0 {
        st.select = (st.select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        st.hcyl = ((sector_num >> 16) & 0xff) as u8;
        st.lcyl = ((sector_num >> 8) & 0xff) as u8;
        st.sector = (sector_num & 0xff) as u8;
    } else {
        let hsec: i64 = st.heads.wrapping_mul(st.sectors) as i64;
        let (cyl, r): (u32, u32) = if hsec != 0 {
            (
                sector_num.wrapping_div(hsec) as u32,
                sector_num.wrapping_rem(hsec) as u32,
            )
        } else {
            // Division by zero is undefined in C; stay panic-free.
            (0, 0)
        };
        st.hcyl = ((cyl >> 8) & 0xff) as u8;
        st.lcyl = (cyl & 0xff) as u8;
        let sec: u32 = st.sectors as u32;
        let (sel, snum): (u32, u32) = if sec != 0 {
            (r / sec, r % sec)
        } else {
            (0, 0)
        };
        st.select = (st.select & 0xf0) | ((sel & 0x0f) as u8);
        st.sector = snum.wrapping_add(1) as u8;
    }
}

// static void ide_abort_command(IDEState *s)
unsafe fn rs_ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

// static void ide_set_irq(IDEState *s) / set_irq()
unsafe fn rs_ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

// static void ide_transfer_stop(IDEState *s)
unsafe extern "C" fn rs_ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(rs_ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

// static void ide_transfer_start(IDEState *s, int size, EndTransferFunc *func)
unsafe fn rs_ide_transfer_start(s: *mut IDEState, size: c_int, func: Option<EndTransferFunc>) {
    (*s).end_transfer_func = func;
    (*s).data_index = 0;
    (*s).data_end = size;
}

// static void ide_sector_read_cb_end(IDEState *s)
unsafe extern "C" fn rs_ide_sector_read_cb_end(s: *mut IDEState) {
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    rs_ide_transfer_stop(s);
}

// static void ide_sector_read_cb(void *opaque, int ret)
unsafe extern "C" fn rs_ide_sector_read_cb(opaque: *mut c_void, ret: c_int) {
    let _ = ret; // ignored by the original code
    let s = opaque as *mut IDEState;
    let n = (*s).io_nb_sectors;
    rs_ide_set_sector(s, rs_ide_get_sector(s) + n as i64);
    (*s).nsector = (((*s).nsector as c_int - n) & 0xff) as u16;
    let func: Option<EndTransferFunc> = if (*s).nsector == 0 {
        Some(rs_ide_sector_read_cb_end)
    } else {
        Some(ide_sector_read)
    };
    rs_ide_transfer_start(s, 512i32.wrapping_mul(n), func);
    rs_ide_set_irq(s);
    (*s).status = 0x40 | 0x10 | 0x08;
    (*s).error = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read(s: *mut IDEState) {
    let sector_num: i64 = rs_ide_get_sector(s);
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }

    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    let buf: *mut u8 = (*s).io_buffer.as_mut_ptr();
    let ret: c_int = match (*bs).read_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            buf,
            n,
            Some(rs_ide_sector_read_cb),
            s as *mut c_void,
        ),
        // A null C function pointer would fault; report an error instead.
        None => -1,
    };
    if ret < 0 {
        // error
        rs_ide_abort_command(s);
        rs_ide_set_irq(s);
    } else if ret == 0 {
        // synchronous case (needed for performance)
        rs_ide_sector_read_cb(s as *mut c_void, 0);
    } else {
        // async case
        (*s).status = 0x40 | 0x10 | 0x80;
        (*s).error = 0; // not needed by IDE spec, but needed by Windows
    }
}
}

// ── `int64_t sector_num; int ret, n; sector_num = ide_get_sector(s); n = s->nsector;` · ide.c:372-376 · chunk 1191 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t sector_num;
//         int ret, n;
//     
//         sector_num = ide_get_sector(s);
//         n = s->nsector;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1191 {
use core::ffi::{c_int, c_void};

// Mirror of the C `struct IDEState` (fields in order, same types).
#[repr(C)]
pub struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

// Port of the C `static int64_t ide_get_sector(IDEState *s)` (no linker symbol).
unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let st = &*s;
    if st.select & 0x40 != 0 {
        let v: c_int = (((st.select & 0x0f) as c_int) << 24)
            | ((st.hcyl as c_int) << 16)
            | ((st.lcyl as c_int) << 8)
            | (st.sector as c_int);
        v as i64
    } else {
        let cyl: c_int = ((st.hcyl as c_int) << 8) | (st.lcyl as c_int);
        let a: c_int = cyl.wrapping_mul(st.heads).wrapping_mul(st.sectors);
        let b: c_int = ((st.select & 0x0f) as c_int).wrapping_mul(st.sectors);
        let c: c_int = (st.sector as c_int).wrapping_sub(1);
        a.wrapping_add(b).wrapping_add(c) as i64
    }
}

#[no_mangle]
pub extern "C" fn ide_sector_read__part1191(s: *mut IDEState, n_out: *mut c_int) -> i64 {
    unsafe {
        let sector_num = ide_get_sector(s);
        let n: c_int = (*s).nsector as c_int;
        if !n_out.is_null() {
            *n_out = n;
        }
        sector_num
    }
}
}

// ── `if (n == 0)` · ide.c:377-378 · chunk 1192 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n == 0)
//             n = 256;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1192 {
#[no_mangle]
pub extern "C" fn ide_sector_read__part1192(mut n: core::ffi::c_int) -> core::ffi::c_int {
    if n == 0 {
        n = 256;
    }
    n
}
}

// ── `if (n > s->req_nb_sectors)` · ide.c:379-380 · chunk 1193 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n > s->req_nb_sectors)
//             n = s->req_nb_sectors;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1193 {
// Port of the TARGET piece of ide_sector_read:
//   if (n > s->req_nb_sectors) n = s->req_nb_sectors;

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read__part1193(s: *mut IDEState, n: *mut c_int) {
    if s.is_null() || n.is_null() {
        return;
    }
    let cur = *n;
    let req = (*s).req_nb_sectors;
    if cur > req {
        *n = req;
    }
}
}

// ── `s->io_nb_sectors = n; ret = s->bs->read_async(s->bs, sector_num, s->io_buffer, n, ide_sector_re…` · ide.c:384-386 · chunk 1194 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     s->io_nb_sectors = n;
//         ret = s->bs->read_async(s->bs, sector_num, s->io_buffer, n,
//                                 ide_sector_read_cb, s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1194 {
// Ported piece of ide_sector_read: records io_nb_sectors and kicks off the
// asynchronous block-device read.
use core::ffi::{c_int, c_void};

// typedef void BlockDeviceCompletionFunc(void *opaque, int ret);
type BlockDeviceCompletionFunc = Option<unsafe extern "C" fn(*mut c_void, c_int)>;

#[repr(C)]
pub struct BlockDevice {
    // int64_t (*get_sector_count)(BlockDevice *bs);
    get_sector_count: *mut c_void,
    // int (*read_async)(BlockDevice *bs, uint64_t sector_num, uint8_t *buf,
    //                   int n, BlockDeviceCompletionFunc *cb, void *opaque);
    read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    // int (*write_async)(BlockDevice *bs, uint64_t sector_num,
    //                    const uint8_t *buf, int n,
    //                    BlockDeviceCompletionFunc *cb, void *opaque);
    write_async: *mut c_void,
    opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    ide_if: *mut c_void,
    bs: *mut BlockDevice,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read__part1194(
    s: *mut IDEState,
    sector_num: i64,
    n: c_int,
    cb: BlockDeviceCompletionFunc,
) -> c_int {
    if s.is_null() {
        return -1;
    }
    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    if bs.is_null() {
        return -1;
    }
    let buf: *mut u8 = (*s).io_buffer.as_mut_ptr();
    match (*bs).read_async {
        Some(f) => f(bs, sector_num as u64, buf, n, cb, s as *mut c_void),
        None => -1,
    }
}
}

// ── `if (ret < 0) … else` · ide.c:387-398 · chunk 1195 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0) {
//     
//             ide_abort_command(s);
//             ide_set_irq(s);
//         } else if (ret == 0) {
//     
//             ide_sector_read_cb(s, 0);
//         } else {
//     
//             s->status = 0x40 | 0x10 | 0x80;
//             s->error = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1195 {
// Rust port of the read-completion dispatch tail of ide_sector_read (ide.c).
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, irq_num: c_int, level: c_int);
pub type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);
pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            sector_num: u64,
            buf: *mut u8,
            n: c_int,
            cb: Option<BlockDeviceCompletionFunc>,
            opaque: *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            sector_num: u64,
            buf: *const u8,
            n: c_int,
            cb: Option<BlockDeviceCompletionFunc>,
            opaque: *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// static void ide_set_irq(IDEState *s)
#[inline]
unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if !(((*ide_if).cmd as c_int) & 0x02 != 0) {
        set_irq((*ide_if).irq, 1);
    }
}

// static void ide_abort_command(IDEState *s)
#[inline]
unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read__part1195(
    s: *mut IDEState,
    ret: c_int,
    cb: unsafe extern "C" fn(*mut c_void, c_int),
) {
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        cb(s as *mut c_void, 0);
    } else {
        (*s).status = (0x40 | 0x10 | 0x80) as u8;
        (*s).error = 0;
    }
}
}

// ── `ide_sector_read_cb` · ide.c:401-418 · chunk 1196 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_read_cb(void *opaque, int ret)
//     {
//         IDEState *s = opaque;
//         int n;
//         EndTransferFunc *func;
//     
//         n = s->io_nb_sectors;
//         ide_set_sector(s, ide_get_sector(s) + n);
//         s->nsector = (s->nsector - n) & 0xff;
//         if (s->nsector == 0)
//             func = ide_sector_read_cb_end;
//         else
//             func = ide_sector_read;
//         ide_transfer_start(s, 512 * n, func);
//         ide_set_irq(s);
//         s->status = 0x40 | 0x10 | 0x08;
//         s->error = 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1196 {
// Port of ide.c:ide_sector_read_cb() to Rust.
//
// In the C program this function is `static`, but it is used as an indirect
// callback (stored in IDEState.end_transfer_func and passed to
// BlockDevice.read_async), so it is exported here under the same C symbol
// name with an ABI-identical signature.
//
// The C helpers it calls (ide_set_sector, ide_get_sector, ide_set_irq,
// ide_transfer_start) and the two file-static end-transfer callbacks it names
// (ide_sector_read, ide_sector_read_cb_end, ide_transfer_stop) are all `static`
// in ide.c, hence they have no linker symbols: their logic is ported into this
// file as private Rust functions. Only libc/`core` facilities are used.

use core::ffi::{c_int, c_void};

// typedef void EndTransferFunc(IDEState *);
pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);

type GetSectorCountFunc = unsafe extern "C" fn(*mut BlockDevice) -> i64;

type ReadAsyncFunc = unsafe extern "C" fn(
    *mut BlockDevice,
    u64,
    *mut u8,
    c_int,
    BlockDeviceCompletionFunc,
    *mut c_void,
) -> c_int;

type WriteAsyncFunc = unsafe extern "C" fn(
    *mut BlockDevice,
    u64,
    *const u8,
    c_int,
    BlockDeviceCompletionFunc,
    *mut c_void,
) -> c_int;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<GetSectorCountFunc>,
    pub read_async: Option<ReadAsyncFunc>,
    pub write_async: Option<WriteAsyncFunc>,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Status/error bits from ide.c
const READY_STAT: u8 = 0x40;
const SEEK_STAT: u8 = 0x10;
const DRQ_STAT: u8 = 0x08;
const BUSY_STAT: u8 = 0x80;

// static void ide_transfer_start(IDEState *s, int size,
//                                EndTransferFunc *end_transfer_func)
#[inline]
unsafe fn ide_transfer_start(
    s: *mut IDEState,
    size: c_int,
    end_transfer_func: Option<EndTransferFunc>,
) {
    (*s).end_transfer_func = end_transfer_func;
    (*s).data_index = 0;
    (*s).data_end = size;
}

// static void ide_transfer_stop(IDEState *s)
unsafe extern "C" fn rust_ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(rust_ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

// static void ide_abort_command(IDEState *s)
#[inline]
unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = READY_STAT | 0x01; // ABRT_STAT
    (*s).error = 0x04;
}

// static void ide_set_irq(IDEState *s)
#[inline]
unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        // set_irq(ide_if->irq, 1);
        let irq = (*ide_if).irq;
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

// static int64_t ide_get_sector(IDEState *s)
#[inline]
unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    if ((*s).select & 0x40) != 0 {
        // LBA mode: ((s->select & 0x0f) << 24) | (s->hcyl << 16) |
        //           (s->lcyl << 8) | s->sector   (computed in int)
        let v: i32 = (((*s).select & 0x0f) as i32) << 24
            | ((*s).hcyl as i32) << 16
            | ((*s).lcyl as i32) << 8
            | ((*s).sector as i32);
        v as i64
    } else {
        // CHS mode:
        // ((s->hcyl << 8) | s->lcyl) * s->heads * s->sectors +
        // (s->select & 0x0f) * s->sectors + (s->sector - 1)
        let mut v: i32 = (((*s).hcyl as i32) << 8) | ((*s).lcyl as i32);
        v = v.wrapping_mul((*s).heads);
        v = v.wrapping_mul((*s).sectors);
        v = v.wrapping_add((((*s).select & 0x0f) as i32).wrapping_mul((*s).sectors));
        v = v.wrapping_add(((*s).sector as i32).wrapping_sub(1));
        v as i64
    }
}

// static void ide_set_sector(IDEState *s, int64_t sector_num)
#[inline]
unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if ((*s).select & 0x40) != 0 {
        (*s).select = ((*s).select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        // int divisor = s->heads * s->sectors  (evaluated as int, then widened
        // for the 64-bit division performed by C)
        let divisor: i32 = (*s).heads.wrapping_mul((*s).sectors);
        let d: i64 = divisor as i64;
        // C integer division / remainder truncate toward zero; guard the
        // (undefined in C) zero-divisor case so no Rust panic can occur.
        let (cyl, r): (u32, u32) = if d != 0 {
            (sector_num.wrapping_div(d) as u32, sector_num.wrapping_rem(d) as u32)
        } else {
            (0u32, 0u32)
        };
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        let secs: u32 = (*s).sectors as u32;
        let r_div: u32 = if secs != 0 { r / secs } else { 0 };
        let r_mod: u32 = if secs != 0 { r % secs } else { 0 };
        (*s).select = ((*s).select & 0xf0) | ((r_div & 0x0f) as u8);
        (*s).sector = (r_mod.wrapping_add(1)) as u8;
    }
}

// static void ide_sector_read_cb_end(IDEState *s)
unsafe extern "C" fn rust_ide_sector_read_cb_end(s: *mut IDEState) {
    // no more sector to read from disk
    (*s).status = READY_STAT | SEEK_STAT;
    (*s).error = 0;
    rust_ide_transfer_stop(s);
}

// static void ide_sector_read(IDEState *s)
unsafe extern "C" fn rust_ide_sector_read(s: *mut IDEState) {
    let ret: c_int;

    let sector_num: i64 = ide_get_sector(s);
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }

    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    ret = match (*bs).read_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_mut_ptr(),
            n,
            ide_sector_read_cb,
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        // error
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        // synchronous case (needed for performance)
        ide_sector_read_cb(s as *mut c_void, 0);
    } else {
        // async case
        (*s).status = READY_STAT | SEEK_STAT | BUSY_STAT;
        (*s).error = 0; // not needed by IDE spec, but needed by Windows
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let n: c_int;
    let func: EndTransferFunc;

    n = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s).wrapping_add(n as i64));
    (*s).nsector = (((*s).nsector as i32).wrapping_sub(n) & 0xff) as u16;
    if (*s).nsector == 0 {
        func = rust_ide_sector_read_cb_end;
    } else {
        func = rust_ide_sector_read;
    }
    ide_transfer_start(s, 512i32.wrapping_mul(n), Some(func));
    ide_set_irq(s);
    (*s).status = READY_STAT | SEEK_STAT | DRQ_STAT;
    (*s).error = 0; // not needed by IDE spec, but needed by Windows
}
}

// ── `IDEState *s = opaque; int n; EndTransferFunc *func; n = s->io_nb_sectors; ide_set_sector(s, ide…` · ide.c:403-409 · chunk 1197 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     IDEState *s = opaque;
//         int n;
//         EndTransferFunc *func;
//     
//         n = s->io_nb_sectors;
//         ide_set_sector(s, ide_get_sector(s) + n);
//         s->nsector = (s->nsector - n) & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1197 {
// Port of the head of ide_sector_read_cb: reads io_nb_sectors, advances the
// CHS/LBA sector registers and decrements nsector.  The static helpers
// ide_get_sector / ide_set_sector have no linker symbol, so their logic is
// reproduced here as private Rust functions.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

fn div_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a.wrapping_div(b)
    }
}

fn rem_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a.wrapping_rem(b)
    }
}

fn div_u32(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

fn rem_u32(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a % b
    }
}

// static int64_t ide_get_sector(IDEState *s)
unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let select = (*s).select as i32;
    if select & 0x40 != 0 {
        let v = ((select & 0x0f) << 24)
            | ((*s).hcyl as i32) << 16
            | ((*s).lcyl as i32) << 8
            | ((*s).sector as i32);
        v as i64
    } else {
        let high = ((*s).hcyl as i32) << 8 | ((*s).lcyl as i32);
        let v = high
            .wrapping_mul((*s).heads)
            .wrapping_mul((*s).sectors)
            .wrapping_add((select & 0x0f).wrapping_mul((*s).sectors))
            .wrapping_add(((*s).sector as i32).wrapping_sub(1));
        v as i64
    }
}

// static void ide_set_sector(IDEState *s, int64_t sector_num)
unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if (*s).select & 0x40 != 0 {
        (*s).select = (((*s).select as i32 & 0xf0) | ((sector_num >> 24) & 0x0f) as i32) as u8;
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        let d = (*s).heads.wrapping_mul((*s).sectors) as i64;
        let cyl = div_i64(sector_num, d) as u32;
        let r = rem_i64(sector_num, d) as u32;
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        (*s).select = (((*s).select as i32 & 0xf0)
            | (div_u32(r, (*s).sectors as u32) & 0x0f) as i32) as u8;
        (*s).sector = (rem_u32(r, (*s).sectors as u32).wrapping_add(1) & 0xff) as u8;
    }
}

#[no_mangle]
pub extern "C" fn ide_sector_read_cb__part1197(opaque: *mut c_void) -> c_int {
    unsafe {
        let s = opaque as *mut IDEState;
        let n = (*s).io_nb_sectors;
        let sector = ide_get_sector(s).wrapping_add(n as i64);
        ide_set_sector(s, sector);
        (*s).nsector = (((*s).nsector as i32).wrapping_sub(n) & 0xff) as u16;
        n
    }
}
}

// ── `if (s->nsector == 0) … else` · ide.c:410-413 · chunk 1198 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->nsector == 0)
//             func = ide_sector_read_cb_end;
//         else
//             func = ide_sector_read;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1198 {
// Selection of the end-transfer callback for ide_sector_read_cb.
// Records whether the transfer is finished (nsector == 0) and picks the
// matching EndTransferFunc pointer accordingly.

#[no_mangle]
pub extern "C" fn ide_sector_read_cb__part1198(
    nsector: u16,
    end_transfer_func: *mut core::ffi::c_void,
    sector_read_func: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    if nsector == 0 {
        end_transfer_func
    } else {
        sector_read_func
    }
}
}

// ── `ide_transfer_start(s, 512 * n, func); ide_set_irq(s); s->status = 0x40 | 0x10 | 0x08; s->error…` · ide.c:414-417 · chunk 1199 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ide_transfer_start(s, 512 * n, func);
//         ide_set_irq(s);
//         s->status = 0x40 | 0x10 | 0x08;
//         s->error = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1199 {
// Ported piece of ide_sector_read_cb: ide_transfer_start + ide_set_irq +
// status/error update. The C helpers ide_transfer_start / ide_set_irq / set_irq
// are `static`/`static inline` in the translation unit, so their logic is
// reproduced here.

use core::ffi::{c_int, c_void};

type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct IDEIFState {
    irq: *mut IRQSignal,
    cur_drive: *mut IDEState,
    drives: [*mut IDEState; 2],
    cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    ide_if: *mut IDEIFState,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: Option<EndTransferFunc>,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read_cb__part1199(
    s: *mut IDEState,
    n: c_int,
    func: Option<EndTransferFunc>,
) {
    // static void ide_transfer_start(IDEState *s, int size, EndTransferFunc *f)
    (*s).end_transfer_func = func;
    (*s).data_index = 0;
    (*s).data_end = 512 * n;

    // static void ide_set_irq(IDEState *s) { set_irq(s->ide_if->irq, 1); }
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        match (*irq).set_irq {
            Some(f) => f((*irq).opaque, (*irq).irq_num, 1),
            None => {}
        }
    }

    (*s).status = 0x40 | 0x10 | 0x08;
    (*s).error = 0;
}
}

// ── `ide_sector_read_cb_end` · ide.c:420-426 · chunk 1200 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_read_cb_end(IDEState *s)
//     {
//     
//         s->status = 0x40 | 0x10;
//         s->error = 0;
//         ide_transfer_stop(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1200 {
// Rust rewrite of ide_sector_read_cb_end() from ide.c

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEIFState {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct BlockDevice {
    _opaque: [u8; 0],
}

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Ported logic of the C static function ide_transfer_stop().
unsafe fn ide_transfer_stop(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).end_transfer_func = ide_transfer_stop as *mut c_void;
    (*s).data_index = 0;
    (*s).data_end = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    ide_transfer_stop(s);
}
}

// ── `s->status = 0x40 | 0x10; s->error = 0; ide_transfer_stop(s);` · ide.c:423-425 · chunk 1201 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->status = 0x40 | 0x10;
//         s->error = 0;
//         ide_transfer_stop(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1201 {
// Ported target of ide_sector_read_cb_end: status/error update plus
// the (static) ide_transfer_stop logic, which is mirrored here because a
// C `static` function has no linker symbol.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

// Mirror of the static C helper ide_transfer_stop().
unsafe extern "C" fn transfer_stop(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).end_transfer_func = Some(transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_read_cb_end__part1201(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    transfer_stop(s);
}
}

// ── `ide_sector_write_cb1` · ide.c:428-452 · chunk 1202 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_write_cb1(IDEState *s)
//     {
//         int64_t sector_num;
//         int ret;
//     
//         ide_transfer_stop(s);
//         sector_num = ide_get_sector(s);
//     
//     
//     
//     
//         ret = s->bs->write_async(s->bs, sector_num, s->io_buffer, s->io_nb_sectors,
//                                  ide_sector_write_cb2, s);
//         if (ret < 0) {
//     
//             ide_abort_command(s);
//             ide_set_irq(s);
//         } else if (ret == 0) {
//     
//             ide_sector_write_cb2(s, 0);
//         } else {
//     
//             s->status = 0x40 | 0x10 | 0x80;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1202 {
// Port of ide_sector_write_cb1 (IDE sector write trigger).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};
use core::ptr::addr_of;

// ---------------------------------------------------------------------------
// Mirror of the C types touched through pointers (exact field order / types).
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: EndTransferFunc,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

pub type BlockDeviceCompletionFunc = Option<unsafe extern "C" fn(*mut c_void, c_int)>;

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

type EndTransferFunc = Option<unsafe extern "C" fn(*mut IDEState)>;

// ---------------------------------------------------------------------------
// Ported static helpers from ide.c (they have no linker symbol).
// ---------------------------------------------------------------------------

unsafe extern "C" fn ide_transfer_stop_local(s: *mut IDEState) {
    (*s).end_transfer_func = Some(ide_transfer_stop_local);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

unsafe fn ide_transfer_start_local(s: *mut IDEState, size: c_int, func: EndTransferFunc) {
    (*s).end_transfer_func = func;
    (*s).data_index = 0;
    (*s).data_end = size;
}

unsafe fn ide_abort_command_local(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

unsafe fn ide_set_irq_local(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

unsafe fn ide_get_sector_local(s: *mut IDEState) -> i64 {
    let sel = (*s).select as c_int;
    let sector = (*s).sector as c_int;
    let lcyl = (*s).lcyl as c_int;
    let hcyl = (*s).hcyl as c_int;
    if (sel & 0x40) != 0 {
        (((sel & 0x0f) << 24) | (hcyl << 16) | (lcyl << 8) | sector) as i64
    } else {
        let heads = (*s).heads;
        let sectors = (*s).sectors;
        let cyl_part = ((hcyl << 8) | lcyl)
            .wrapping_mul(heads)
            .wrapping_mul(sectors);
        let v = cyl_part
            .wrapping_add((sel & 0x0f).wrapping_mul(sectors))
            .wrapping_add(sector.wrapping_sub(1));
        v as i64
    }
}

unsafe fn ide_set_sector_local(s: *mut IDEState, sector_num: i64) {
    if ((*s).select & 0x40) != 0 {
        (*s).select = (((*s).select & 0xf0) as c_int | (((sector_num >> 24) & 0x0f) as c_int)) as u8;
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        let heads = (*s).heads;
        let sectors = (*s).sectors;
        let denom = heads.wrapping_mul(sectors);
        if denom == 0 {
            // C would divide by zero here; avoid a Rust panic across the FFI.
            return;
        }
        let cyl = sector_num / (denom as i64);
        let r = sector_num % (denom as i64);
        // C stores these into `unsigned int` locals before the shifts/divs.
        let cyl_u = cyl as u32;
        let r_u = r as u32;
        let sec_u = sectors as u32;
        (*s).hcyl = ((cyl_u >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl_u & 0xff) as u8;
        let selv = ((*s).select & 0xf0) as u32 | ((r_u / sec_u) & 0x0f);
        (*s).select = selv as u8;
        (*s).sector = (r_u % sec_u).wrapping_add(1) as u8;
    }
}

// Ported ide_sector_write_cb2 (static in ide.c, used as a completion callback).
unsafe extern "C" fn ide_sector_write_cb2_local(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let mut n = (*s).io_nb_sectors;

    ide_set_sector_local(s, ide_get_sector_local(s).wrapping_add(n as i64));
    (*s).nsector = (((*s).nsector as c_int).wrapping_sub(n) & 0xff) as u16;
    if (*s).nsector == 0 {
        // no more sectors to write
        (*s).status = 0x40 | 0x10;
    } else {
        n = (*s).nsector as c_int;
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start_local(s, 512i32.wrapping_mul(n), Some(ide_sector_write_cb1));
        (*s).status = 0x40 | 0x10 | 0x08;
    }
    ide_set_irq_local(s);
}

// ---------------------------------------------------------------------------
// TARGET
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    let mut sector_num: i64;
    let ret: c_int;

    ide_transfer_stop_local(s);
    sector_num = ide_get_sector_local(s);

    let bs = (*s).bs;
    let buf = addr_of!((*s).io_buffer) as *const u8;
    let nb = (*s).io_nb_sectors;
    ret = match (*bs).write_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            buf,
            nb,
            Some(ide_sector_write_cb2_local),
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        // error
        ide_abort_command_local(s);
        ide_set_irq_local(s);
    } else if ret == 0 {
        // synchronous case (needed for performance)
        ide_sector_write_cb2_local(s as *mut c_void, 0);
    } else {
        // async case
        (*s).status = 0x40 | 0x10 | 0x80;
    }
    let _ = sector_num;
    sector_num = 0;
}
}

// ── `int64_t sector_num; int ret; ide_transfer_stop(s); sector_num = ide_get_sector(s); ret = s->bs-…` · ide.c:430-440 · chunk 1203 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int64_t sector_num;
//         int ret;
//     
//         ide_transfer_stop(s);
//         sector_num = ide_get_sector(s);
//     
//     
//     
//     
//         ret = s->bs->write_async(s->bs, sector_num, s->io_buffer, s->io_nb_sectors,
//                                  ide_sector_write_cb2, s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1203 {
use core::ffi::{c_int, c_void};

type BlockDeviceCompletionFunc = Option<extern "C" fn(*mut c_void, c_int)>;
type EndTransferFunc = Option<extern "C" fn(*mut IDEState)>;

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: EndTransferFunc,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Ported from the static C helper `ide_get_sector`.
fn ide_get_sector(s: &IDEState) -> i64 {
    let select = s.select as c_int;
    if select & 0x40 != 0 {
        let v = ((select & 0x0f) << 24)
            | ((s.hcyl as c_int) << 16)
            | ((s.lcyl as c_int) << 8)
            | (s.sector as c_int);
        v as i64
    } else {
        let base = ((s.hcyl as c_int) << 8) | (s.lcyl as c_int);
        let v = base
            .wrapping_mul(s.heads)
            .wrapping_mul(s.sectors)
            .wrapping_add((select & 0x0f).wrapping_mul(s.sectors))
            .wrapping_add((s.sector as c_int).wrapping_sub(1));
        v as i64
    }
}

#[no_mangle]
pub extern "C" fn ide_sector_write_cb1__part1203(
    s: *mut IDEState,
    stop_func: EndTransferFunc,
    write_cb: BlockDeviceCompletionFunc,
) -> c_int {
    unsafe {
        // ide_transfer_stop(s)
        (*s).end_transfer_func = stop_func;
        (*s).data_index = 0;
        (*s).data_end = 0;

        let sector_num = ide_get_sector(&*s);

        let bs = (*s).bs;
        let io_nb_sectors = (*s).io_nb_sectors;
        let buf: *mut u8 = (*s).io_buffer.as_mut_ptr();

        // C would simply dereference the (non-null) slot; mirror that exactly.
        let f: extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            BlockDeviceCompletionFunc,
            *mut c_void,
        ) -> c_int = core::mem::transmute((*bs).write_async);

        f(
            bs,
            sector_num as u64,
            buf as *const u8,
            io_nb_sectors,
            write_cb,
            s as *mut c_void,
        )
    }
}
}

// ── `if (ret < 0) … else` · ide.c:441-451 · chunk 1204 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0) {
//     
//             ide_abort_command(s);
//             ide_set_irq(s);
//         } else if (ret == 0) {
//     
//             ide_sector_write_cb2(s, 0);
//         } else {
//     
//             s->status = 0x40 | 0x10 | 0x80;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1204 {
// Rust port of the write-completion branch of IDEState's ide_sector_write_cb1.
// The whole three-way dispatch on `ret` happens here; only the statically
// linked C callback ide_sector_write_cb2 is reached through a function pointer
// handed in from the C caller.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Port of the file-static `ide_abort_command` (no linker symbol).
unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

// Port of the file-static `ide_set_irq` (no linker symbol), which goes through
// the inlined `set_irq` helper.
unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write_cb1__part1204(
    s: *mut IDEState,
    ret: c_int,
    cb2: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
) {
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        if let Some(f) = cb2 {
            f(s as *mut c_void, 0);
        }
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
    }
}
}

// ── `ide_sector_write_cb2` · ide.c:454-474 · chunk 1205 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_write_cb2(void *opaque, int ret)
//     {
//         IDEState *s = opaque;
//         int n;
//     
//         n = s->io_nb_sectors;
//         ide_set_sector(s, ide_get_sector(s) + n);
//         s->nsector = (s->nsector - n) & 0xff;
//         if (s->nsector == 0) {
//     
//             s->status = 0x40 | 0x10;
//         } else {
//             n = s->nsector;
//             if (n > s->req_nb_sectors)
//                 n = s->req_nb_sectors;
//             s->io_nb_sectors = n;
//             ide_transfer_start(s, 512 * n, ide_sector_write_cb1);
//             s->status = 0x40 | 0x10 | 0x08;
//         }
//         ide_set_irq(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1205 {
// Rust rewrite of ide.c: ide_sector_write_cb2 (plus the file-static helpers it
// needs; those have no linker symbol so their logic is reproduced here).

use core::ffi::{c_int, c_void};

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
pub type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,

    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,

    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,

    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// --- panic-free substitutes for C division (C division by zero is UB) ---

fn div_i64(a: i64, b: i64) -> i64 {
    a.wrapping_div(b)
}

fn rem_i64(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else {
        a.wrapping_rem(b)
    }
}

fn div_u32(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a / b
    }
}

fn rem_u32(a: u32, b: u32) -> u32 {
    if b == 0 {
        0
    } else {
        a % b
    }
}

// --- ports of the static C helpers ---

unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let select = (*s).select;
    if select & 0x40 != 0 {
        let v: i32 = (((select & 0x0f) as i32) << 24)
            | (((*s).hcyl as i32) << 16)
            | (((*s).lcyl as i32) << 8)
            | ((*s).sector as i32);
        v as i64
    } else {
        let low = (((*s).hcyl as i32) << 8) | ((*s).lcyl as i32);
        let a = low.wrapping_mul((*s).heads).wrapping_mul((*s).sectors);
        let b = ((select & 0x0f) as i32).wrapping_mul((*s).sectors);
        let c = ((*s).sector as i32).wrapping_sub(1);
        a.wrapping_add(b).wrapping_add(c) as i64
    }
}

unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if (*s).select & 0x40 != 0 {
        (*s).select = ((*s).select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        // divisor is the int product (s->heads * s->sectors), widened to int64_t
        let denom = (*s).heads.wrapping_mul((*s).sectors) as i64;
        let cyl = div_i64(sector_num, denom) as u32;
        let r = rem_i64(sector_num, denom) as u32;
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        let sec = (*s).sectors as u32;
        (*s).select = ((*s).select & 0xf0) | ((div_u32(r, sec) & 0x0f) as u8);
        (*s).sector = (rem_u32(r, sec).wrapping_add(1) & 0xff) as u8;
    }
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ide_if.is_null() {
        return;
    }
    if (*ide_if).cmd & 0x02 == 0 {
        set_irq((*ide_if).irq, 1);
    }
}

unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

unsafe fn ide_transfer_start(s: *mut IDEState, size: c_int, end_transfer_func: EndTransferFunc) {
    (*s).end_transfer_func = Some(end_transfer_func);
    (*s).data_index = 0;
    (*s).data_end = size;
}

// Port of the static C function ide_sector_write_cb1 (its address is stored in
// end_transfer_func and called from C, hence extern "C").
unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    ide_transfer_stop(s);
    let sector_num = ide_get_sector(s);
    let bs = (*s).bs;
    let buf: *const u8 = (*s).io_buffer.as_ptr();
    let count = (*s).io_nb_sectors;
    let cb: BlockDeviceCompletionFunc = ide_sector_write_cb2;
    let ret: c_int = if bs.is_null() {
        -1
    } else {
        match (*bs).write_async {
            Some(f) => f(bs, sector_num as u64, buf, count, Some(cb), s as *mut c_void),
            None => -1,
        }
    };
    if ret < 0 {
        /* error */
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        /* synchronous case (needed for performance) */
        ide_sector_write_cb2(s as *mut c_void, 0);
    } else {
        /* async case */
        (*s).status = 0x40 | 0x10 | 0x80;
    }
}

// --- the target ---

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    if s.is_null() {
        return;
    }
    let mut n: c_int = (*s).io_nb_sectors;
    let sector_num = ide_get_sector(s);
    ide_set_sector(s, sector_num.wrapping_add(n as i64));
    (*s).nsector = ((((*s).nsector as i32).wrapping_sub(n)) & 0xff) as u16;
    if (*s).nsector == 0 {
        /* no more sectors to write */
        (*s).status = 0x40 | 0x10;
    } else {
        n = (*s).nsector as i32;
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
        (*s).status = 0x40 | 0x10 | 0x08;
    }
    ide_set_irq(s);
}
}

// ── `IDEState *s = opaque; int n; n = s->io_nb_sectors; ide_set_sector(s, ide_get_sector(s) + n); s-…` · ide.c:456-461 · chunk 1206 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     IDEState *s = opaque;
//         int n;
//     
//         n = s->io_nb_sectors;
//         ide_set_sector(s, ide_get_sector(s) + n);
//         s->nsector = (s->nsector - n) & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1206 {
// Port of the first statements of ide_sector_write_cb2:
//   n = s->io_nb_sectors;
//   ide_set_sector(s, ide_get_sector(s) + n);
//   s->nsector = (s->nsector - n) & 0xff;
// ide_get_sector / ide_set_sector are C `static` functions (no linker symbol),
// so their logic is ported here as private Rust helpers.

use core::ffi::{c_int, c_void};

#[repr(C)]
#[allow(dead_code)]
pub struct IDEState {
    ide_if: *mut c_void,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: *mut c_void,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let select = (*s).select;
    if (select & 0x40) != 0 {
        let v: i32 = (((select as i32) & 0x0f) << 24)
            | (((*s).hcyl as i32) << 16)
            | (((*s).lcyl as i32) << 8)
            | ((*s).sector as i32);
        v as i64
    } else {
        let base: i32 = (((*s).hcyl as i32) << 8) | ((*s).lcyl as i32);
        let heads = (*s).heads;
        let sectors = (*s).sectors;
        let v: i32 = base
            .wrapping_mul(heads)
            .wrapping_mul(sectors)
            .wrapping_add(((select as i32) & 0x0f).wrapping_mul(sectors))
            .wrapping_add((*s).sector as i32 - 1);
        v as i64
    }
}

unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    let select = (*s).select;
    if (select & 0x40) != 0 {
        let sel = ((select as u32) & 0xf0) | (((sector_num >> 24) & 0x0f) as u32);
        (*s).select = sel as u8;
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        // C: int64_t division/modulo by (int64_t)(s->heads * s->sectors)
        let div = ((*s).heads).wrapping_mul((*s).sectors) as i64;
        let cyl_q: i64 = if div != 0 { sector_num / div } else { 0 };
        let r_q: i64 = if div != 0 { sector_num % div } else { 0 };
        // conversion int64_t -> unsigned int truncates
        let cyl: u32 = cyl_q as u32;
        let r: u32 = r_q as u32;
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        // C: unsigned int arithmetic, s->sectors converted to unsigned int
        let sectors_u: u32 = (*s).sectors as u32;
        let q: u32 = if sectors_u != 0 { r / sectors_u } else { 0 };
        let m: u32 = if sectors_u != 0 { r % sectors_u } else { 0 };
        let sel = ((select as u32) & 0xf0) | (q & 0x0f);
        (*s).select = sel as u8;
        (*s).sector = m.wrapping_add(1) as u8;
    }
}

#[no_mangle]
pub extern "C" fn ide_sector_write_cb2__part1206(s: *mut IDEState) -> c_int {
    unsafe {
        let n: c_int = (*s).io_nb_sectors;
        let sector: i64 = ide_get_sector(s).wrapping_add(n as i64);
        ide_set_sector(s, sector);
        let ns: c_int = ((*s).nsector as c_int).wrapping_sub(n) & 0xff;
        (*s).nsector = ns as u16;
        n
    }
}
}

// ── `if (s->nsector == 0) … else` · ide.c:462-472 · chunk 1207 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (s->nsector == 0) {
//     
//             s->status = 0x40 | 0x10;
//         } else {
//             n = s->nsector;
//             if (n > s->req_nb_sectors)
//                 n = s->req_nb_sectors;
//             s->io_nb_sectors = n;
//             ide_transfer_start(s, 512 * n, ide_sector_write_cb1);
//             s->status = 0x40 | 0x10 | 0x08;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1207 {
// Rust port of the completion branch of ide_sector_write_cb2():
// it either finishes the transfer or starts the next sector write.
//
// The C statics reachable from here (ide_transfer_start) have no linker
// symbol, so their bodies are mirrored as private Rust fns.  The address of
// the static `ide_sector_write_cb1` cannot be named from Rust either, so the
// C caller passes it in.

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut core::ffi::c_void,
    pub bs: *mut core::ffi::c_void,
    pub cylinders: core::ffi::c_int,
    pub heads: core::ffi::c_int,
    pub sectors: core::ffi::c_int,
    pub mult_sectors: core::ffi::c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: core::ffi::c_int,
    pub req_nb_sectors: core::ffi::c_int,
    pub end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    pub data_index: core::ffi::c_int,
    pub data_end: core::ffi::c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// static void ide_transfer_start(IDEState *s, int size, EndTransferFunc *f)
fn ide_transfer_start(
    s: &mut IDEState,
    size: core::ffi::c_int,
    end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
) {
    s.end_transfer_func = end_transfer_func;
    s.data_index = 0;
    s.data_end = size;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write_cb2__part1207(
    s: *mut IDEState,
    n: core::ffi::c_int,
    write_cb1: Option<unsafe extern "C" fn(*mut IDEState)>,
) {
    let s: &mut IDEState = &mut *s;
    let mut n: core::ffi::c_int = n;
    if s.nsector == 0 {
        s.status = 0x40 | 0x10;
    } else {
        n = s.nsector as core::ffi::c_int;
        if n > s.req_nb_sectors {
            n = s.req_nb_sectors;
        }
        s.io_nb_sectors = n;
        ide_transfer_start(s, 512u32.wrapping_mul(n as u32) as core::ffi::c_int, write_cb1);
        s.status = 0x40 | 0x10 | 0x08;
    }
}
}

// ── `ide_set_irq(s);` · ide.c:473-473 · chunk 1208 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ide_set_irq(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1208 {
// Port of the ide_set_irq() call (and the static inline set_irq chain) at the
// tail of ide_sector_write_cb2().
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write_cb2__part1208(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if (*ide_if).cmd & 0x02 == 0 {
        let irq = (*ide_if).irq;
        let set_irq = (*irq).set_irq;
        if let Some(f) = set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}
}

// ── `ide_sector_write` · ide.c:476-487 · chunk 1209 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_sector_write(IDEState *s)
//     {
//         int n;
//         n = s->nsector;
//         if (n == 0)
//             n = 256;
//         if (n > s->req_nb_sectors)
//             n = s->req_nb_sectors;
//         s->io_nb_sectors = n;
//         ide_transfer_start(s, 512 * n, ide_sector_write_cb1);
//         s->status = 0x40 | 0x10 | 0x08;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1209 {
// Rust port of ide_sector_write (ide.c) plus the write-side transfer chain
// it depends on (static C helpers without linker symbols).

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<unsafe extern "C" fn(*mut c_void, c_int)>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<unsafe extern "C" fn(*mut c_void, c_int)>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
pub type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

const READY_STAT: u8 = 0x40;
const SEEK_STAT: u8 = 0x10;
const DRQ_STAT: u8 = 0x08;
const BUSY_STAT: u8 = 0x80;

unsafe extern "C" fn transfer_stop(s: *mut IDEState) {
    unsafe {
        if s.is_null() {
            return;
        }
        (*s).end_transfer_func = Some(transfer_stop);
        (*s).data_index = 0;
        (*s).data_end = 0;
    }
}

unsafe fn transfer_start(s: *mut IDEState, size: c_int, func: EndTransferFunc) {
    unsafe {
        (*s).end_transfer_func = Some(func);
        (*s).data_index = 0;
        (*s).data_end = size;
    }
}

unsafe fn get_sector(s: *mut IDEState) -> i64 {
    unsafe {
        let st = &*s;
        if st.select & 0x40 != 0 {
            let v = (((st.select & 0x0f) as i32) << 24)
                | ((st.hcyl as i32) << 16)
                | ((st.lcyl as i32) << 8)
                | (st.sector as i32);
            v as i64
        } else {
            let a = (((st.hcyl as i32) << 8) | (st.lcyl as i32))
                .wrapping_mul(st.heads)
                .wrapping_mul(st.sectors);
            let b = ((st.select & 0x0f) as i32).wrapping_mul(st.sectors);
            a.wrapping_add(b)
                .wrapping_add((st.sector as i32).wrapping_sub(1)) as i64
        }
    }
}

unsafe fn set_sector(s: *mut IDEState, sector_num: i64) {
    unsafe {
        let st = &mut *s;
        if st.select & 0x40 != 0 {
            st.select = (st.select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
            st.hcyl = ((sector_num >> 16) & 0xff) as u8;
            st.lcyl = ((sector_num >> 8) & 0xff) as u8;
            st.sector = (sector_num & 0xff) as u8;
        } else {
            let d = st.heads.wrapping_mul(st.sectors) as i64;
            let (cyl, r) = if d == 0 {
                (0i64, 0i64)
            } else {
                (sector_num / d, sector_num % d)
            };
            let cyl_u = cyl as u32;
            st.hcyl = ((cyl_u >> 8) & 0xff) as u8;
            st.lcyl = (cyl_u & 0xff) as u8;
            let r_u = r as u32;
            let secs = st.sectors as u32;
            let (quo, rem) = if secs == 0 {
                (0u32, 0u32)
            } else {
                (r_u / secs, r_u % secs)
            };
            st.select = (st.select & 0xf0) | ((quo & 0x0f) as u8);
            st.sector = (rem.wrapping_add(1) & 0xff) as u8;
        }
    }
}

unsafe fn abort_command(s: *mut IDEState) {
    unsafe {
        (*s).status = READY_STAT | 0x01;
        (*s).error = 0x04;
    }
}

unsafe fn set_irq(s: *mut IDEState) {
    unsafe {
        let ide_if = (*s).ide_if;
        if ide_if.is_null() {
            return;
        }
        if (*ide_if).cmd & 0x02 == 0 {
            let irq = (*ide_if).irq;
            if irq.is_null() {
                return;
            }
            if let Some(f) = (*irq).set_irq {
                f((*irq).opaque, (*irq).irq_num, 1);
            }
        }
    }
}

unsafe extern "C" fn write_cb1(s: *mut IDEState) {
    unsafe {
        transfer_stop(s);
        let sector_num = get_sector(s);

        let bs = (*s).bs;
        if bs.is_null() {
            return;
        }
        let wf: BlockDeviceCompletionFunc = write_cb2;
        let f = match (*bs).write_async {
            Some(f) => f,
            None => return,
        };
        let ret = f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_ptr(),
            (*s).io_nb_sectors,
            Some(wf),
            s as *mut c_void,
        );
        if ret < 0 {
            abort_command(s);
            set_irq(s);
        } else if ret == 0 {
            write_cb2(s as *mut c_void, 0);
        } else {
            (*s).status = READY_STAT | SEEK_STAT | BUSY_STAT;
        }
    }
}

unsafe extern "C" fn write_cb2(opaque: *mut c_void, _ret: c_int) {
    unsafe {
        let s = opaque as *mut IDEState;
        if s.is_null() {
            return;
        }
        let mut n: c_int = (*s).io_nb_sectors;
        set_sector(s, get_sector(s).wrapping_add(n as i64));
        (*s).nsector = (((*s).nsector as c_int).wrapping_sub(n) & 0xff) as u16;
        if (*s).nsector == 0 {
            // no more sectors to write
            (*s).status = READY_STAT | SEEK_STAT;
        } else {
            n = (*s).nsector as c_int;
            if n > (*s).req_nb_sectors {
                n = (*s).req_nb_sectors;
            }
            (*s).io_nb_sectors = n;
            transfer_start(s, 512i32.wrapping_mul(n), write_cb1);
            (*s).status = READY_STAT | SEEK_STAT | DRQ_STAT;
        }
        set_irq(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write(s: *mut IDEState) {
    unsafe {
        if s.is_null() {
            return;
        }
        let mut n: c_int = (*s).nsector as c_int;
        if n == 0 {
            n = 256;
        }
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        transfer_start(s, 512i32.wrapping_mul(n), write_cb1);
        (*s).status = READY_STAT | SEEK_STAT | DRQ_STAT;
    }
}
}

// ── `int n; n = s->nsector;` · ide.c:478-479 · chunk 1210 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int n;
//         n = s->nsector;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1210 {
// Fragment of ide_sector_write: reads s->nsector and yields the sector count.
#[no_mangle]
pub extern "C" fn ide_sector_write__part1210(nsector: u16) -> core::ffi::c_int {
    nsector as core::ffi::c_int
}
}

// ── `if (n == 0)` · ide.c:480-481 · chunk 1211 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n == 0)
//             n = 256;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1211 {
// Ported piece of ide_sector_write: sector count fixup.
// if (n == 0) n = 256;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ide_sector_write__part1211(n: c_int) -> c_int {
    if n == 0 {
        256
    } else {
        n
    }
}
}

// ── `if (n > s->req_nb_sectors)` · ide.c:482-483 · chunk 1212 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (n > s->req_nb_sectors)
//             n = s->req_nb_sectors;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1212 {
// Port of ide_sector_write lines 482-483: clamp the requested sector count
// against the drive's max sectors-per-request.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn ide_sector_write__part1212(n: c_int, req_nb_sectors: c_int) -> c_int {
    let mut n = n;
    if n > req_nb_sectors {
        n = req_nb_sectors;
    }
    n
}
}

// ── `s->io_nb_sectors = n; ide_transfer_start(s, 512 * n, ide_sector_write_cb1); s->status = 0x40 |…` · ide.c:484-486 · chunk 1213 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->io_nb_sectors = n;
//         ide_transfer_start(s, 512 * n, ide_sector_write_cb1);
//         s->status = 0x40 | 0x10 | 0x08;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1213 {
// Ported piece of ide_sector_write: sets up the write transfer for the
// IDE sector write command.
use core::ffi::{c_int, c_uchar, c_ushort};

// typedef void EndTransferFunc(IDEState *);
pub type EndTransferFunc = Option<unsafe extern "C" fn(*mut IDEState)>;

// Mirror of the C struct IDEState (fields in order, same types).
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut core::ffi::c_void,
    pub bs: *mut core::ffi::c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: c_uchar,
    pub error: c_uchar,
    pub nsector: c_ushort,
    pub sector: c_uchar,
    pub lcyl: c_uchar,
    pub hcyl: c_uchar,
    pub select: c_uchar,
    pub status: c_uchar,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: EndTransferFunc,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [c_uchar; 128 * 512 + 4],
}

// Ported logic of the static C function ide_transfer_start (no linker symbol).
unsafe fn ide_transfer_start(s: &mut IDEState, size: c_int, end_transfer_func: EndTransferFunc) {
    s.end_transfer_func = end_transfer_func;
    s.data_index = 0;
    s.data_end = size;
}

#[no_mangle]
pub unsafe extern "C" fn ide_sector_write__part1213(
    s: *mut IDEState,
    n: c_int,
    cb: EndTransferFunc,
) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    s.io_nb_sectors = n;
    ide_transfer_start(s, 512i32.wrapping_mul(n), cb);
    s.status = (0x40 | 0x10 | 0x08) as c_uchar;
}
}

// ── `ide_identify_cb` · ide.c:489-493 · chunk 1214 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_identify_cb(IDEState *s)
//     {
//         ide_transfer_stop(s);
//         s->status = 0x40;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1214 {
// Rust rewrite of ide_identify_cb (ide.c)
use core::ffi::c_int;

// Mirror of the C struct IDEState (fields in order, same types).
#[repr(C)]
pub struct IDEState {
    ide_if: *mut core::ffi::c_void,
    bs: *mut core::ffi::c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

// Body of the C static helper ide_transfer_stop(), which has no linker symbol.
unsafe extern "C" fn ide_transfer_stop_impl(s: *mut IDEState) {
    let s = &mut *s;
    s.end_transfer_func = Some(ide_transfer_stop_impl);
    s.data_index = 0;
    s.data_end = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_identify_cb(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    ide_transfer_stop_impl(s);
    (*s).status = 0x40;
}
}

// ── `ide_transfer_stop(s); s->status = 0x40;` · ide.c:491-492 · chunk 1215 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ide_transfer_stop(s);
//         s->status = 0x40;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1215 {
// Port of the TARGET statements inside ide_identify_cb:
//     ide_transfer_stop(s);
//     s->status = 0x40;
// ide_transfer_stop() is a static C function (no linker symbol), so its body is
// inlined here. It also stores its own address into s->end_transfer_func, which
// cannot be expressed from Rust; the C replacement finishes that part.

use core::ffi::c_int;

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut core::ffi::c_void,
    pub bs: *mut core::ffi::c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut core::ffi::c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_identify_cb__part1215(s: *mut IDEState) {
    unsafe {
        // ide_transfer_stop(s): s->data_index = 0; s->data_end = 0;
        (*s).data_index = 0;
        (*s).data_end = 0;
        (*s).status = 0x40;
    }
}
}

// ── `ide_exec_cmd` · ide.c:495-564 · chunk 1216 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_exec_cmd(IDEState *s, int val)
//     {
//     
//     
//     
//         switch(val) {
//         case 0xEC:
//             ide_identify(s);
//             s->status = 0x40 | 0x10 | 0x08;
//             ide_transfer_start(s, 512, ide_identify_cb);
//             ide_set_irq(s);
//             break;
//         case 0x91:
//         case 0x10:
//             s->error = 0;
//             s->status = 0x40 | 0x10;
//             ide_set_irq(s);
//             break;
//         case 0xC6:
//             if (s->nsector > 128 ||
//                 (s->nsector & (s->nsector - 1)) != 0) {
//                 ide_abort_command(s);
//             } else {
//                 s->mult_sectors = s->nsector;
//     
//     
//     
//                 s->status = 0x40;
//             }
//             ide_set_irq(s);
//             break;
//         case 0x20:
//         case 0x21:
//             s->req_nb_sectors = 1;
//             ide_sector_read(s);
//             break;
//         case 0x30:
//         case 0x31:
//             s->req_nb_sectors = 1;
//             ide_sector_write(s);
//             break;
//         case 0xC4:
//             if (!s->mult_sectors) {
//                 ide_abort_command(s);
//                 ide_set_irq(s);
//             } else {
//                 s->req_nb_sectors = s->mult_sectors;
//                 ide_sector_read(s);
//             }
//             break;
//         case 0xC5:
//             if (!s->mult_sectors) {
//                 ide_abort_command(s);
//                 ide_set_irq(s);
//             } else {
//                 s->req_nb_sectors = s->mult_sectors;
//                 ide_sector_write(s);
//             }
//             break;
//         case 0xF8:
//             ide_set_sector(s, s->nb_sectors - 1);
//             s->status = 0x40;
//             ide_set_irq(s);
//             break;
//         default:
//             ide_abort_command(s);
//             ide_set_irq(s);
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1216 {
// Rust reimplementation of ide_exec_cmd (ide.c) together with the static
// helpers of that file which the target calls (directly or as callbacks).

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
pub type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

const IO_BUFFER_LEN: usize = 128 * 512 + 4;

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; IO_BUFFER_LEN],
}

// ---------------------------------------------------------------- helpers ---

#[inline]
unsafe fn io_buf(s: *mut IDEState) -> *mut u8 {
    unsafe { ptr::addr_of_mut!((*s).io_buffer) as *mut u8 }
}

// static void stw(uint16_t *buf, int v) { *buf = v; }  (tab + index)
#[inline]
unsafe fn stw(tab: *mut u16, index: usize, v: c_int) {
    unsafe {
        ptr::write_unaligned(tab.add(index), v as u16);
    }
}

// static void padstr(char *str, const char *src, int len)
unsafe fn padstr(str_: *mut c_char, src: *const c_char, len: c_int) {
    unsafe {
        let mut p = str_;
        let mut s = src;
        let mut i: c_int = 0;
        while i < len {
            let v: c_int = if *s != 0 {
                let c = *s;
                s = s.add(1);
                c as c_int
            } else {
                32 // ' '
            };
            *((p as isize ^ 1) as *mut c_char) = v as c_char;
            p = p.add(1);
            i = i.wrapping_add(1);
        }
    }
}

// static inline void set_irq(IRQSignal *irq, int level)
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    unsafe {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

unsafe fn ide_set_irq(s: *mut IDEState) {
    unsafe {
        let ide_if = (*s).ide_if;
        if ((*ide_if).cmd & 0x02) == 0 {
            set_irq((*ide_if).irq, 1);
        }
    }
}

unsafe fn ide_abort_command(s: *mut IDEState) {
    unsafe {
        (*s).status = 0x40 | 0x01;
        (*s).error = 0x04;
    }
}

unsafe fn ide_transfer_start(s: *mut IDEState, size: c_int, func: EndTransferFunc) {
    unsafe {
        (*s).end_transfer_func = Some(func);
        (*s).data_index = 0;
        (*s).data_end = size;
    }
}

unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    unsafe {
        (*s).end_transfer_func = Some(ide_transfer_stop);
        (*s).data_index = 0;
        (*s).data_end = 0;
    }
}

unsafe fn ide_identify(s: *mut IDEState) {
    unsafe {
        let tab = io_buf(s) as *mut u16;

        ptr::write_bytes(tab as *mut u8, 0u8, 512 * 2);

        stw(tab, 0, 0x0040);
        stw(tab, 1, (*s).cylinders);
        stw(tab, 3, (*s).heads);
        stw(tab, 4, 512i32.wrapping_mul((*s).sectors));
        stw(tab, 5, 512);
        stw(tab, 6, (*s).sectors);
        stw(tab, 20, 3);
        stw(tab, 21, 512);
        stw(tab, 22, 4);
        padstr(
            tab.add(27) as *mut c_char,
            b"RISCVEMU HARDDISK\0".as_ptr() as *const c_char,
            40,
        );
        stw(tab, 47, 0x8000 | 128);
        stw(tab, 48, 0);
        stw(tab, 49, 1 << 9);
        stw(tab, 51, 0x200);
        stw(tab, 52, 0x200);
        stw(tab, 54, (*s).cylinders);
        stw(tab, 55, (*s).heads);
        stw(tab, 56, (*s).sectors);
        let oldsize: u32 = (*s)
            .cylinders
            .wrapping_mul((*s).heads)
            .wrapping_mul((*s).sectors) as u32;
        stw(tab, 57, oldsize as c_int);
        stw(tab, 58, (oldsize >> 16) as c_int);
        if (*s).mult_sectors != 0 {
            stw(tab, 59, 0x100 | (*s).mult_sectors);
        }
        stw(tab, 60, (*s).nb_sectors as c_int);
        stw(tab, 61, ((*s).nb_sectors >> 16) as c_int);
        stw(tab, 80, (1 << 1) | (1 << 2));
        stw(tab, 82, 1 << 14);
        stw(tab, 83, 1 << 14);
        stw(tab, 84, 1 << 14);
        stw(tab, 85, 1 << 14);
        stw(tab, 86, 0);
        stw(tab, 87, 1 << 14);
    }
}

unsafe extern "C" fn ide_identify_cb(s: *mut IDEState) {
    unsafe {
        ide_transfer_stop(s);
        (*s).status = 0x40;
    }
}

unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    unsafe {
        let sector_num: c_int;
        if ((*s).select & 0x40) != 0 {
            sector_num = (((*s).select & 0x0f) as c_int) << 24
                | ((*s).hcyl as c_int) << 16
                | ((*s).lcyl as c_int) << 8
                | ((*s).sector as c_int);
        } else {
            let head_cyl: c_int = ((*s).hcyl as c_int) << 8 | ((*s).lcyl as c_int);
            sector_num = head_cyl
                .wrapping_mul((*s).heads)
                .wrapping_mul((*s).sectors)
                .wrapping_add((((*s).select & 0x0f) as c_int).wrapping_mul((*s).sectors))
                .wrapping_add((*s).sector as c_int - 1);
        }
        sector_num as i64
    }
}

unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    unsafe {
        if ((*s).select & 0x40) != 0 {
            (*s).select = (((*s).select & 0xf0) as i64 | ((sector_num >> 24) & 0x0f)) as u8;
            (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
            (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
            (*s).sector = (sector_num & 0xff) as u8;
        } else {
            let denom: i64 = (*s).heads.wrapping_mul((*s).sectors) as i64;
            let cyl: u32 = sector_num.checked_div(denom).unwrap_or(0) as u32;
            let r: u32 = sector_num.checked_rem(denom).unwrap_or(0) as u32;
            let secs: u32 = (*s).sectors as u32;
            (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
            (*s).lcyl = (cyl & 0xff) as u8;
            (*s).select =
                (((*s).select & 0xf0) as u32 | (r.checked_div(secs).unwrap_or(0) & 0x0f)) as u8;
            (*s).sector = r.checked_rem(secs).unwrap_or(0).wrapping_add(1) as u8;
        }
    }
}

// ide_sector_read is also used as an EndTransferFunc callback.
unsafe extern "C" fn ide_sector_read(s: *mut IDEState) {
    unsafe {
        let sector_num = ide_get_sector(s);
        let mut n: c_int = (*s).nsector as c_int;
        if n == 0 {
            n = 256;
        }
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }

        (*s).io_nb_sectors = n;

        let bs = (*s).bs;
        let read_async = match (*bs).read_async {
            Some(f) => f,
            None => return,
        };
        let ret = read_async(
            bs,
            sector_num as u64,
            io_buf(s),
            n,
            Some(ide_sector_read_cb),
            s as *mut c_void,
        );

        if ret < 0 {
            ide_abort_command(s);
            ide_set_irq(s);
        } else if ret == 0 {
            ide_sector_read_cb(s as *mut c_void, 0);
        } else {
            (*s).status = 0x40 | 0x10 | 0x80;
            (*s).error = 0;
        }
    }
}

unsafe extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    unsafe {
        let s = opaque as *mut IDEState;

        let n: c_int = (*s).io_nb_sectors;
        ide_set_sector(s, ide_get_sector(s).wrapping_add(n as i64));
        (*s).nsector = (((*s).nsector as c_int).wrapping_sub(n) & 0xff) as u16;
        let func: EndTransferFunc = if (*s).nsector == 0 {
            ide_sector_read_cb_end
        } else {
            ide_sector_read
        };
        ide_transfer_start(s, 512i32.wrapping_mul(n), func);
        ide_set_irq(s);
        (*s).status = 0x40 | 0x10 | 0x08;
        (*s).error = 0;
    }
}

unsafe extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    unsafe {
        (*s).status = 0x40 | 0x10;
        (*s).error = 0;
        ide_transfer_stop(s);
    }
}

unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    unsafe {
        ide_transfer_stop(s);
        let sector_num = ide_get_sector(s);

        let bs = (*s).bs;
        let write_async = match (*bs).write_async {
            Some(f) => f,
            None => return,
        };
        let ret = write_async(
            bs,
            sector_num as u64,
            io_buf(s) as *const u8,
            (*s).io_nb_sectors,
            Some(ide_sector_write_cb2),
            s as *mut c_void,
        );

        if ret < 0 {
            ide_abort_command(s);
            ide_set_irq(s);
        } else if ret == 0 {
            ide_sector_write_cb2(s as *mut c_void, 0);
        } else {
            (*s).status = 0x40 | 0x10 | 0x80;
        }
    }
}

unsafe extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    unsafe {
        let s = opaque as *mut IDEState;

        let mut n: c_int = (*s).io_nb_sectors;
        ide_set_sector(s, ide_get_sector(s).wrapping_add(n as i64));
        (*s).nsector = (((*s).nsector as c_int).wrapping_sub(n) & 0xff) as u16;
        if (*s).nsector == 0 {
            (*s).status = 0x40 | 0x10;
        } else {
            n = (*s).nsector as c_int;
            if n > (*s).req_nb_sectors {
                n = (*s).req_nb_sectors;
            }
            (*s).io_nb_sectors = n;
            ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
            (*s).status = 0x40 | 0x10 | 0x08;
        }
        ide_set_irq(s);
    }
}

unsafe fn ide_sector_write(s: *mut IDEState) {
    unsafe {
        let mut n: c_int = (*s).nsector as c_int;
        if n == 0 {
            n = 256;
        }
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
        (*s).status = 0x40 | 0x10 | 0x08;
    }
}

// ------------------------------------------------------------------ target ---

#[no_mangle]
pub extern "C" fn ide_exec_cmd(s: *mut IDEState, val: c_int) {
    unsafe {
        match val {
            0xEC => {
                ide_identify(s);
                (*s).status = 0x40 | 0x10 | 0x08;
                ide_transfer_start(s, 512, ide_identify_cb);
                ide_set_irq(s);
            }
            0x91 | 0x10 => {
                (*s).error = 0;
                (*s).status = 0x40 | 0x10;
                ide_set_irq(s);
            }
            0xC6 => {
                let n: c_int = (*s).nsector as c_int;
                if n > 128 || (n & n.wrapping_sub(1)) != 0 {
                    ide_abort_command(s);
                } else {
                    (*s).mult_sectors = (*s).nsector as c_int;
                    (*s).status = 0x40;
                }
                ide_set_irq(s);
            }
            0x20 | 0x21 => {
                (*s).req_nb_sectors = 1;
                ide_sector_read(s);
            }
            0x30 | 0x31 => {
                (*s).req_nb_sectors = 1;
                ide_sector_write(s);
            }
            0xC4 => {
                if (*s).mult_sectors == 0 {
                    ide_abort_command(s);
                    ide_set_irq(s);
                } else {
                    (*s).req_nb_sectors = (*s).mult_sectors;
                    ide_sector_read(s);
                }
            }
            0xC5 => {
                if (*s).mult_sectors == 0 {
                    ide_abort_command(s);
                    ide_set_irq(s);
                } else {
                    (*s).req_nb_sectors = (*s).mult_sectors;
                    ide_sector_write(s);
                }
            }
            0xF8 => {
                ide_set_sector(s, (*s).nb_sectors.wrapping_sub(1));
                (*s).status = 0x40;
                ide_set_irq(s);
            }
            _ => {
                ide_abort_command(s);
                ide_set_irq(s);
            }
        }
    }
}
}

// ── `switch (val)` · ide.c:500-563 · chunk 1217 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(val) {
//         case 0xEC:
//             ide_identify(s);
//             s->status = 0x40 | 0x10 | 0x08;
//             ide_transfer_start(s, 512, ide_identify_cb);
//             ide_set_irq(s);
//             break;
//         case 0x91:
//         case 0x10:
//             s->error = 0;
//             s->status = 0x40 | 0x10;
//             ide_set_irq(s);
//             break;
//         case 0xC6:
//             if (s->nsector > 128 ||
//                 (s->nsector & (s->nsector - 1)) != 0) {
//                 ide_abort_command(s);
//             } else {
//                 s->mult_sectors = s->nsector;
//     
//     
//     
//                 s->status = 0x40;
//             }
//             ide_set_irq(s);
//             break;
//         case 0x20:
//         case 0x21:
//             s->req_nb_sectors = 1;
//             ide_sector_read(s);
//             break;
//         case 0x30:
//         case 0x31:
//             s->req_nb_sectors = 1;
//             ide_sector_write(s);
//             break;
//         case 0xC4:
//             if (!s->mult_sectors) {
//                 ide_abort_command(s);
//                 ide_set_irq(s);
//             } else {
//                 s->req_nb_sectors = s->mult_sectors;
//                 ide_sector_read(s);
//             }
//             break;
//         case 0xC5:
//             if (!s->mult_sectors) {
//                 ide_abort_command(s);
//                 ide_set_irq(s);
//             } else {
//                 s->req_nb_sectors = s->mult_sectors;
//                 ide_sector_write(s);
//             }
//             break;
//         case 0xF8:
//             ide_set_sector(s, s->nb_sectors - 1);
//             s->status = 0x40;
//             ide_set_irq(s);
//             break;
//         default:
//             ide_abort_command(s);
//             ide_set_irq(s);
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1217 {
// Rust port of the command dispatch switch of ide_exec_cmd() (ide.c).
// The C helpers reached from the switch are static, so their logic is
// reproduced here as private Rust functions.

use core::ffi::{c_int, c_void};

type EndTransferFunc = extern "C" fn(*mut IDEState);
type BlockDeviceCompletionFunc = extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// C signed division / remainder, without panicking on the C-UB cases.
fn c_div(a: i64, b: i64) -> i64 {
    match a.checked_div(b) {
        Some(v) => v,
        None => {
            if b == 0 {
                0
            } else {
                a
            }
        }
    }
}

fn c_rem(a: i64, b: i64) -> i64 {
    match a.checked_rem(b) {
        Some(v) => v,
        None => 0,
    }
}

fn set_irq(irq: *mut IRQSignal, level: c_int) {
    unsafe {
        if irq.is_null() {
            return;
        }
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

fn ide_set_irq(s: *mut IDEState) {
    unsafe {
        if s.is_null() {
            return;
        }
        let ide_if = (*s).ide_if;
        if ide_if.is_null() {
            return;
        }
        if ((*ide_if).cmd as c_int) & 0x02 == 0 {
            set_irq((*ide_if).irq, 1);
        }
    }
}

fn ide_abort_command(s: *mut IDEState) {
    unsafe {
        (*s).status = (0x40 | 0x01) as u8;
        (*s).error = 0x04;
    }
}

fn ide_transfer_start(s: *mut IDEState, size: c_int, f: EndTransferFunc) {
    unsafe {
        (*s).end_transfer_func = Some(f);
        (*s).data_index = 0;
        (*s).data_end = size;
    }
}

extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    unsafe {
        (*s).end_transfer_func = Some(ide_transfer_stop as EndTransferFunc);
        (*s).data_index = 0;
        (*s).data_end = 0;
    }
}

fn ide_get_sector(s: *mut IDEState) -> i64 {
    unsafe {
        let sector_num: i64;
        if (*s).select as c_int & 0x40 != 0 {
            // sector_num = ((s->select & 0x0f) << 24) | (s->hcyl << 16) |
            //              (s->lcyl << 8) | s->sector;
            let v: c_int = (((*s).select as c_int & 0x0f) << 24)
                | ((*s).hcyl as c_int) << 16
                | ((*s).lcyl as c_int) << 8
                | ((*s).sector as c_int);
            sector_num = v as i64;
        } else {
            // sector_num = ((s->hcyl << 8) | s->lcyl) * s->heads * s->sectors +
            //              (s->select & 0x0f) * s->sectors + (s->sector - 1);
            let v: c_int = ((((*s).hcyl as c_int) << 8) | ((*s).lcyl as c_int))
                .wrapping_mul((*s).heads)
                .wrapping_mul((*s).sectors)
                .wrapping_add(((*s).select as c_int & 0x0f).wrapping_mul((*s).sectors))
                .wrapping_add((*s).sector as c_int - 1);
            sector_num = v as i64;
        }
        sector_num
    }
}

fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    unsafe {
        if (*s).select as c_int & 0x40 != 0 {
            (*s).select =
                ((((*s).select as c_int & 0xf0) | (((sector_num >> 24) & 0x0f) as c_int)) & 0xff) as u8;
            (*s).hcyl = (((sector_num >> 16) & 0xff) as u8) as u8;
            (*s).lcyl = (((sector_num >> 8) & 0xff) as u8) as u8;
            (*s).sector = ((sector_num & 0xff) as u8) as u8;
        } else {
            // unsigned int cyl, r;
            let d: i64 = (*s).heads.wrapping_mul((*s).sectors) as i64;
            let cyl: u32 = c_div(sector_num, d) as u32;
            let r: u32 = c_rem(sector_num, d) as u32;
            (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
            (*s).lcyl = (cyl & 0xff) as u8;
            let sect: u32 = (*s).sectors as u32;
            let q: u32 = if sect == 0 { 0 } else { r / sect };
            let m: u32 = if sect == 0 { 0 } else { r % sect };
            (*s).select = ((((*s).select as u32) & 0xf0) | (q & 0x0f)) as u8;
            (*s).sector = m.wrapping_add(1) as u8;
        }
    }
}

// static void stw(uint16_t *buf, int v) { *buf = v; }
fn stw(tab: *mut u8, idx: usize, v: c_int) {
    unsafe {
        core::ptr::write_unaligned(tab.add(idx * 2) as *mut u16, v as u16);
    }
}

// static void padstr(char *str, const char *src, int len)
fn padstr(str_: *mut u8, src: *const u8, len: c_int) {
    unsafe {
        let mut sp = str_;
        let mut sc = src;
        let mut i: c_int = 0;
        while i < len {
            let v: u8 = if *sc != 0 {
                let t = *sc;
                sc = sc.add(1);
                t
            } else {
                b' '
            };
            core::ptr::write((sp as usize ^ 1) as *mut u8, v);
            sp = sp.add(1);
            i += 1;
        }
    }
}

fn ide_identify(s: *mut IDEState) {
    unsafe {
        let tab: *mut u8 = (*s).io_buffer.as_mut_ptr();

        // memset(tab, 0, 512 * 2);
        core::ptr::write_bytes(tab, 0, 512 * 2);

        stw(tab, 0, 0x0040);
        stw(tab, 1, (*s).cylinders);
        stw(tab, 3, (*s).heads);
        stw(tab, 4, 512i32.wrapping_mul((*s).sectors));
        stw(tab, 5, 512);
        stw(tab, 6, (*s).sectors);
        stw(tab, 20, 3);
        stw(tab, 21, 512);
        stw(tab, 22, 4);
        padstr(tab.add(27 * 2), b"RISCVEMU HARDDISK\0".as_ptr(), 40);
        stw(tab, 47, 0x8000 | 128);
        stw(tab, 48, 0);
        stw(tab, 49, 1 << 9);
        stw(tab, 51, 0x200);
        stw(tab, 52, 0x200);
        stw(tab, 54, (*s).cylinders);
        stw(tab, 55, (*s).heads);
        stw(tab, 56, (*s).sectors);
        let oldsize: u32 = ((*s).cylinders as u32)
            .wrapping_mul((*s).heads as u32)
            .wrapping_mul((*s).sectors as u32);
        stw(tab, 57, oldsize as c_int);
        stw(tab, 58, (oldsize >> 16) as c_int);
        if (*s).mult_sectors != 0 {
            stw(tab, 59, 0x100 | (*s).mult_sectors);
        }
        stw(tab, 60, (*s).nb_sectors as c_int);
        stw(tab, 61, ((*s).nb_sectors >> 16) as c_int);
        stw(tab, 80, (1 << 1) | (1 << 2));
        stw(tab, 82, 1 << 14);
        stw(tab, 83, 1 << 14);
        stw(tab, 84, 1 << 14);
        stw(tab, 85, 1 << 14);
        stw(tab, 86, 0);
        stw(tab, 87, 1 << 14);
    }
}

extern "C" fn ide_identify_cb(s: *mut IDEState) {
    ide_transfer_stop(s);
    unsafe {
        (*s).status = 0x40;
    }
}

extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    unsafe {
        (*s).status = (0x40 | 0x10) as u8;
        (*s).error = 0;
    }
    ide_transfer_stop(s);
}

extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    unsafe {
        let s = opaque as *mut IDEState;
        if s.is_null() {
            return;
        }
        let n: c_int = (*s).io_nb_sectors;
        ide_set_sector(s, ide_get_sector(s) + n as i64);
        (*s).nsector = ((((*s).nsector as c_int) - n) & 0xff) as u16;
        let func: EndTransferFunc = if (*s).nsector == 0 {
            ide_sector_read_cb_end
        } else {
            ide_sector_read
        };
        ide_transfer_start(s, 512i32.wrapping_mul(n), func);
        ide_set_irq(s);
        (*s).status = (0x40 | 0x10 | 0x08) as u8;
        (*s).error = 0;
    }
}

extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    unsafe {
        let s = opaque as *mut IDEState;
        if s.is_null() {
            return;
        }
        let mut n: c_int = (*s).io_nb_sectors;
        ide_set_sector(s, ide_get_sector(s) + n as i64);
        (*s).nsector = ((((*s).nsector as c_int) - n) & 0xff) as u16;
        if (*s).nsector == 0 {
            (*s).status = (0x40 | 0x10) as u8;
        } else {
            n = (*s).nsector as c_int;
            if n > (*s).req_nb_sectors {
                n = (*s).req_nb_sectors;
            }
            (*s).io_nb_sectors = n;
            ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
            (*s).status = (0x40 | 0x10 | 0x08) as u8;
        }
        ide_set_irq(s);
    }
}

extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    unsafe {
        ide_transfer_stop(s);
        let sector_num: i64 = ide_get_sector(s);
        let bs = (*s).bs;
        let ret: c_int = if bs.is_null() {
            -1
        } else {
            match (*bs).write_async {
                Some(f) => f(
                    bs,
                    sector_num as u64,
                    (*s).io_buffer.as_ptr(),
                    (*s).io_nb_sectors,
                    Some(ide_sector_write_cb2 as BlockDeviceCompletionFunc),
                    s as *mut c_void,
                ),
                None => -1,
            }
        };
        if ret < 0 {
            ide_abort_command(s);
            ide_set_irq(s);
        } else if ret == 0 {
            ide_sector_write_cb2(s as *mut c_void, 0);
        } else {
            (*s).status = (0x40 | 0x10 | 0x80) as u8;
        }
    }
}

extern "C" fn ide_sector_read(s: *mut IDEState) {
    unsafe {
        let sector_num: i64 = ide_get_sector(s);
        let mut n: c_int = (*s).nsector as c_int;
        if n == 0 {
            n = 256;
        }
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }

        (*s).io_nb_sectors = n;
        let bs = (*s).bs;
        let ret: c_int = if bs.is_null() {
            -1
        } else {
            match (*bs).read_async {
                Some(f) => f(
                    bs,
                    sector_num as u64,
                    (*s).io_buffer.as_mut_ptr(),
                    n,
                    Some(ide_sector_read_cb as BlockDeviceCompletionFunc),
                    s as *mut c_void,
                ),
                None => -1,
            }
        };
        if ret < 0 {
            ide_abort_command(s);
            ide_set_irq(s);
        } else if ret == 0 {
            ide_sector_read_cb(s as *mut c_void, 0);
        } else {
            (*s).status = (0x40 | 0x10 | 0x80) as u8;
            (*s).error = 0;
        }
    }
}

extern "C" fn ide_sector_write(s: *mut IDEState) {
    unsafe {
        let mut n: c_int = (*s).nsector as c_int;
        if n == 0 {
            n = 256;
        }
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
        (*s).status = (0x40 | 0x10 | 0x08) as u8;
    }
}

#[no_mangle]
pub extern "C" fn ide_exec_cmd__part1217(s: *mut IDEState, val: c_int) {
    unsafe {
        if s.is_null() {
            return;
        }
        match val {
            0xEC => {
                ide_identify(s);
                (*s).status = (0x40 | 0x10 | 0x08) as u8;
                ide_transfer_start(s, 512, ide_identify_cb);
                ide_set_irq(s);
            }
            0x91 | 0x10 => {
                (*s).error = 0;
                (*s).status = (0x40 | 0x10) as u8;
                ide_set_irq(s);
            }
            0xC6 => {
                if (*s).nsector > 128 || ((*s).nsector & (*s).nsector.wrapping_sub(1)) != 0 {
                    ide_abort_command(s);
                } else {
                    (*s).mult_sectors = (*s).nsector as c_int;

                    (*s).status = 0x40;
                }
                ide_set_irq(s);
            }
            0x20 | 0x21 => {
                (*s).req_nb_sectors = 1;
                ide_sector_read(s);
            }
            0x30 | 0x31 => {
                (*s).req_nb_sectors = 1;
                ide_sector_write(s);
            }
            0xC4 => {
                if (*s).mult_sectors == 0 {
                    ide_abort_command(s);
                    ide_set_irq(s);
                } else {
                    (*s).req_nb_sectors = (*s).mult_sectors;
                    ide_sector_read(s);
                }
            }
            0xC5 => {
                if (*s).mult_sectors == 0 {
                    ide_abort_command(s);
                    ide_set_irq(s);
                } else {
                    (*s).req_nb_sectors = (*s).mult_sectors;
                    ide_sector_write(s);
                }
            }
            0xF8 => {
                ide_set_sector(s, (*s).nb_sectors - 1);
                (*s).status = 0x40;
                ide_set_irq(s);
            }
            _ => {
                ide_abort_command(s);
                ide_set_irq(s);
            }
        }
    }
}
}

// ── `ide_ioport_write` · ide.c:566-619 · chunk 1218 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_ioport_write(void *opaque, uint32_t offset,
//                                  uint32_t val, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s = s1->cur_drive;
//         int addr = offset + 1;
//     
//     
//     
//     
//         switch(addr) {
//         case 0:
//             break;
//         case 1:
//             if (s) {
//                 s->feature = val;
//             }
//             break;
//         case 2:
//             if (s) {
//                 s->nsector = val;
//             }
//             break;
//         case 3:
//             if (s) {
//                 s->sector = val;
//             }
//             break;
//         case 4:
//             if (s) {
//                 s->lcyl = val;
//             }
//             break;
//         case 5:
//             if (s) {
//                 s->hcyl = val;
//             }
//             break;
//         case 6:
//     
//             s = s1->cur_drive = s1->drives[(val >> 4) & 1];
//             if (s) {
//                 s->select = val;
//             }
//             break;
//         default:
//         case 7:
//     
//             if (s) {
//                 ide_exec_cmd(s, val);
//             }
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1218 {
// Port of ide.c: static ide_ioport_write and the static helper chain it uses
// (ide_exec_cmd and friends have internal linkage in C, so they are
// re-implemented here as private Rust functions).
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<unsafe extern "C" fn(*mut c_void, c_int)>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<unsafe extern "C" fn(*mut c_void, c_int)>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        set_irq((*ide_if).irq, 1);
    }
}

unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

unsafe fn ide_transfer_start(s: *mut IDEState, size: c_int, end_transfer_func: EndTransferFunc) {
    (*s).end_transfer_func = Some(end_transfer_func);
    (*s).data_index = 0;
    (*s).data_end = size;
}

unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let sector_num: i64;
    if ((*s).select & 0x40) != 0 {
        sector_num = ((((*s).select & 0x0f) as i64) << 24)
            | (((*s).hcyl as i64) << 16)
            | (((*s).lcyl as i64) << 8)
            | ((*s).sector as i64);
    } else {
        let a: i64 = (((*s).hcyl as i32) << 8) as i64 | ((*s).lcyl as i64);
        sector_num = a * ((*s).heads as i64) * ((*s).sectors as i64)
            + (((*s).select & 0x0f) as i64) * ((*s).sectors as i64)
            + ((*s).sector as i64 - 1);
    }
    sector_num
}

unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if ((*s).select & 0x40) != 0 {
        (*s).select = ((*s).select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        let d: i64 = (*s).heads as i64 * (*s).sectors as i64;
        let cyl: u32 = match sector_num.checked_div(d) {
            Some(v) => v as u32,
            None => 0,
        };
        let r: u32 = match sector_num.checked_rem(d) {
            Some(v) => v as u32,
            None => 0,
        };
        let sec: u32 = (*s).sectors as u32;
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        let rr = match r.checked_div(sec) {
            Some(v) => v & 0x0f,
            None => 0,
        };
        (*s).select = ((*s).select & 0xf0) | (rr as u8);
        let rs = match r.checked_rem(sec) {
            Some(v) => v,
            None => 0,
        };
        (*s).sector = ((rs + 1) & 0xff) as u8;
    }
}

unsafe extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    ide_transfer_stop(s);
}

unsafe extern "C" fn ide_sector_read(s: *mut IDEState) {
    let sector_num: i64 = ide_get_sector(s);
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }
    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    let ret: c_int = match (*bs).read_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_mut_ptr(),
            n,
            Some(ide_sector_read_cb),
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_read_cb(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
        (*s).error = 0;
    }
}

unsafe extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let n: c_int = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s) + n as i64);
    (*s).nsector = (((*s).nsector as c_int) - n) as u16 & 0xff;
    let func: EndTransferFunc = if (*s).nsector == 0 {
        ide_sector_read_cb_end
    } else {
        ide_sector_read
    };
    ide_transfer_start(s, 512i32.wrapping_mul(n), func);
    ide_set_irq(s);
    (*s).status = 0x40 | 0x10 | 0x08;
    (*s).error = 0;
}

unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    ide_transfer_stop(s);
    let sector_num: i64 = ide_get_sector(s);
    let bs = (*s).bs;
    let ret: c_int = match (*bs).write_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_ptr(),
            (*s).io_nb_sectors,
            Some(ide_sector_write_cb2),
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_write_cb2(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
    }
}

unsafe extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let mut n: c_int = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s) + n as i64);
    (*s).nsector = (((*s).nsector as c_int) - n) as u16 & 0xff;
    if (*s).nsector == 0 {
        (*s).status = 0x40 | 0x10;
    } else {
        n = (*s).nsector as c_int;
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
        (*s).status = 0x40 | 0x10 | 0x08;
    }
    ide_set_irq(s);
}

unsafe fn ide_sector_write(s: *mut IDEState) {
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }
    (*s).io_nb_sectors = n;
    ide_transfer_start(s, 512i32.wrapping_mul(n), ide_sector_write_cb1);
    (*s).status = 0x40 | 0x10 | 0x08;
}

unsafe fn padstr(str_: *mut c_char, src: *const c_char, len: c_int) {
    let mut str_ = str_;
    let mut src = src;
    let mut i: c_int = 0;
    while i < len {
        let v: c_int = if *src != 0 {
            let c = *src;
            src = src.add(1);
            c as c_int & 0xff
        } else {
            b' ' as c_int
        };
        let p = ((str_ as isize) ^ 1) as *mut c_char;
        *p = v as c_char;
        str_ = str_.add(1);
        i += 1;
    }
}

unsafe fn stw(buf: *mut u16, v: c_int) {
    core::ptr::write(buf, v as u16);
}

unsafe fn ide_identify(s: *mut IDEState) {
    let tab: *mut u16 = (*s).io_buffer.as_mut_ptr() as *mut u16;

    core::ptr::write_bytes(tab as *mut u8, 0u8, 512 * 2);

    stw(tab.add(0), 0x0040);
    stw(tab.add(1), (*s).cylinders);
    stw(tab.add(3), (*s).heads);
    stw(tab.add(4), 512i32.wrapping_mul((*s).sectors));
    stw(tab.add(5), 512);
    stw(tab.add(6), (*s).sectors);
    stw(tab.add(20), 3);
    stw(tab.add(21), 512);
    stw(tab.add(22), 4);
    padstr(
        tab.add(27) as *mut c_char,
        b"RISCVEMU HARDDISK\0".as_ptr() as *const c_char,
        40,
    );
    stw(tab.add(47), 0x8000 | 128);
    stw(tab.add(48), 0);
    stw(tab.add(49), 1 << 9);
    stw(tab.add(51), 0x200);
    stw(tab.add(52), 0x200);
    stw(tab.add(54), (*s).cylinders);
    stw(tab.add(55), (*s).heads);
    stw(tab.add(56), (*s).sectors);
    let oldsize: u32 = (*s)
        .cylinders
        .wrapping_mul((*s).heads)
        .wrapping_mul((*s).sectors) as u32;
    stw(tab.add(57), oldsize as c_int);
    stw(tab.add(58), (oldsize >> 16) as c_int);
    if (*s).mult_sectors != 0 {
        stw(tab.add(59), 0x100 | (*s).mult_sectors);
    }
    stw(tab.add(60), (*s).nb_sectors as c_int);
    stw(tab.add(61), ((*s).nb_sectors >> 16) as c_int);
    stw(tab.add(80), (1 << 1) | (1 << 2));
    stw(tab.add(82), 1 << 14);
    stw(tab.add(83), 1 << 14);
    stw(tab.add(84), 1 << 14);
    stw(tab.add(85), 1 << 14);
    stw(tab.add(86), 0);
    stw(tab.add(87), 1 << 14);
}

unsafe extern "C" fn ide_identify_cb(s: *mut IDEState) {
    ide_transfer_stop(s);
    (*s).status = 0x40;
}

unsafe fn ide_exec_cmd(s: *mut IDEState, val: c_int) {
    match val {
        0xEC => {
            ide_identify(s);
            (*s).status = 0x40 | 0x10 | 0x08;
            ide_transfer_start(s, 512, ide_identify_cb);
            ide_set_irq(s);
        }
        0x91 | 0x10 => {
            (*s).error = 0;
            (*s).status = 0x40 | 0x10;
            ide_set_irq(s);
        }
        0xC6 => {
            let ns: c_int = (*s).nsector as c_int;
            if ns > 128 || (ns & (ns - 1)) != 0 {
                ide_abort_command(s);
            } else {
                (*s).mult_sectors = ns;
                (*s).status = 0x40;
            }
            ide_set_irq(s);
        }
        0x20 | 0x21 => {
            (*s).req_nb_sectors = 1;
            ide_sector_read(s);
        }
        0x30 | 0x31 => {
            (*s).req_nb_sectors = 1;
            ide_sector_write(s);
        }
        0xC4 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_read(s);
            }
        }
        0xC5 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_write(s);
            }
        }
        0xF8 => {
            ide_set_sector(s, (*s).nb_sectors - 1);
            (*s).status = 0x40;
            ide_set_irq(s);
        }
        _ => {
            ide_abort_command(s);
            ide_set_irq(s);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_ioport_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;
    let mut s: *mut IDEState = (*s1).cur_drive;
    let addr: c_int = offset.wrapping_add(1) as c_int;

    match addr {
        0 => {}
        1 => {
            if !s.is_null() {
                (*s).feature = val as u8;
            }
        }
        2 => {
            if !s.is_null() {
                (*s).nsector = val as u16;
            }
        }
        3 => {
            if !s.is_null() {
                (*s).sector = val as u8;
            }
        }
        4 => {
            if !s.is_null() {
                (*s).lcyl = val as u8;
            }
        }
        5 => {
            if !s.is_null() {
                (*s).hcyl = val as u8;
            }
        }
        6 => {
            // select drive
            let idx = ((val >> 4) & 1) as usize;
            s = *(*s1).drives.as_ptr().add(idx);
            (*s1).cur_drive = s;
            if !s.is_null() {
                (*s).select = val as u8;
            }
        }
        _ => {
            // command (case 7 and the default label)
            if !s.is_null() {
                ide_exec_cmd(s, val as c_int);
            }
        }
    }
}
}

// ── `switch (addr)` · ide.c:576-618 · chunk 1219 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(addr) {
//         case 0:
//             break;
//         case 1:
//             if (s) {
//                 s->feature = val;
//             }
//             break;
//         case 2:
//             if (s) {
//                 s->nsector = val;
//             }
//             break;
//         case 3:
//             if (s) {
//                 s->sector = val;
//             }
//             break;
//         case 4:
//             if (s) {
//                 s->lcyl = val;
//             }
//             break;
//         case 5:
//             if (s) {
//                 s->hcyl = val;
//             }
//             break;
//         case 6:
//     
//             s = s1->cur_drive = s1->drives[(val >> 4) & 1];
//             if (s) {
//                 s->select = val;
//             }
//             break;
//         default:
//         case 7:
//     
//             if (s) {
//                 ide_exec_cmd(s, val);
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1219 {
// Port of the ide_ioport_write register-select switch (ide.c lines 576-618).
// The switch hands commands to ide_exec_cmd(), which is a `static` C function
// with no linker symbol, so its logic (and everything below it that is also
// static, plus the two block-device completion callbacks it hands out) is
// reproduced here as private Rust functions.

use core::ffi::{c_int, c_void};

type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);
type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

const IO_BUFFER_SIZE: usize = 128 * 512 + 4;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; IO_BUFFER_SIZE],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

unsafe fn stw(buf: *mut u16, v: c_int) {
    core::ptr::write_unaligned(buf, v as u16);
}

// static void padstr(char *str, const char *src, int len)
unsafe fn padstr(mut str_: *mut u8, src: &[u8], len: c_int) {
    let mut si: usize = 0;
    let mut i: c_int = 0;
    while i < len {
        let c: u8 = match src.get(si) {
            Some(&b) => b,
            None => 0,
        };
        let v: u8;
        if c != 0 {
            v = c;
            si += 1;
        } else {
            v = b' ';
        }
        *(((str_ as usize) ^ 1) as *mut u8) = v;
        str_ = str_.add(1);
        i += 1;
    }
}

unsafe fn ide_identify(s: *mut IDEState) {
    let tab = core::ptr::addr_of_mut!((*s).io_buffer) as *mut u16;

    core::ptr::write_bytes(tab as *mut u8, 0, 512 * 2);

    stw(tab, 0x0040);
    stw(tab.add(1), (*s).cylinders);
    stw(tab.add(3), (*s).heads);
    stw(tab.add(4), 512i32.wrapping_mul((*s).sectors));
    stw(tab.add(5), 512);
    stw(tab.add(6), (*s).sectors);
    stw(tab.add(20), 3);
    stw(tab.add(21), 512);
    stw(tab.add(22), 4);
    padstr(
        (tab as *mut u8).add(27 * 2),
        b"RISCVEMU HARDDISK\0",
        40,
    );
    stw(tab.add(47), 0x8000 | 128);
    stw(tab.add(48), 0);
    stw(tab.add(49), 1 << 9);
    stw(tab.add(51), 0x200);
    stw(tab.add(52), 0x200);
    stw(tab.add(54), (*s).cylinders);
    stw(tab.add(55), (*s).heads);
    stw(tab.add(56), (*s).sectors);
    let oldsize: u32 =
        (*s).cylinders.wrapping_mul((*s).heads).wrapping_mul((*s).sectors) as u32;
    stw(tab.add(57), oldsize as c_int);
    stw(tab.add(58), (oldsize >> 16) as c_int);
    if (*s).mult_sectors != 0 {
        stw(tab.add(59), 0x100 | (*s).mult_sectors);
    }
    stw(tab.add(60), (*s).nb_sectors as c_int);
    stw(tab.add(61), ((*s).nb_sectors >> 16) as c_int);
    stw(tab.add(80), (1 << 1) | (1 << 2));
    stw(tab.add(82), 1 << 14);
    stw(tab.add(83), 1 << 14);
    stw(tab.add(84), 1 << 14);
    stw(tab.add(85), 1 << 14);
    stw(tab.add(86), 0);
    stw(tab.add(87), 1 << 14);
}

unsafe extern "C" fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

unsafe extern "C" fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        let irq = (*ide_if).irq;
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

unsafe extern "C" fn ide_transfer_start(s: *mut IDEState, size: c_int, f: Option<EndTransferFunc>) {
    (*s).end_transfer_func = f;
    (*s).data_index = 0;
    (*s).data_end = size;
}

unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

unsafe extern "C" fn ide_get_sector(s: *mut IDEState) -> i64 {
    let sector_num: i64;
    if ((*s).select & 0x40) != 0 {
        let x: u32 = (((*s).select & 0x0f) as u32) << 24
            | ((*s).hcyl as u32) << 16
            | ((*s).lcyl as u32) << 8
            | ((*s).sector as u32);
        sector_num = x as i32 as i64;
    } else {
        let v: i32 = ((*s).hcyl as i32) << 8
            | ((*s).lcyl as i32);
        let v = v
            .wrapping_mul((*s).heads)
            .wrapping_mul((*s).sectors)
            .wrapping_add((((*s).select & 0x0f) as i32).wrapping_mul((*s).sectors))
            .wrapping_add((*s).sector as i32 - 1);
        sector_num = v as i64;
    }
    sector_num
}

unsafe extern "C" fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if ((*s).select & 0x40) != 0 {
        (*s).select = ((*s).select & 0xf0) | (((sector_num >> 24) & 0x0f) as u8);
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        let d: i64 = (*s).heads.wrapping_mul((*s).sectors) as i64;
        if d != 0 {
            let cyl: u32 = (sector_num / d) as u32;
            let r: u32 = (sector_num % d) as u32;
            let sec: u32 = (*s).sectors as u32;
            (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
            (*s).lcyl = (cyl & 0xff) as u8;
            (*s).select = ((*s).select & 0xf0) | (((r / sec) & 0x0f) as u8);
            (*s).sector = ((r % sec).wrapping_add(1)) as u8;
        }
    }
}

unsafe extern "C" fn ide_sector_read(s: *mut IDEState) {
    let sector_num: i64 = ide_get_sector(s);
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }

    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    let buf = core::ptr::addr_of_mut!((*s).io_buffer) as *mut u8;
    let ret: c_int = match (*bs).read_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            buf,
            n,
            Some(ide_sector_read_cb),
            s as *mut c_void,
        ),
        None => 0,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_read_cb(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
        (*s).error = 0;
    }
}

unsafe extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let n: c_int = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s) + n as i64);
    (*s).nsector = (((*s).nsector as c_int - n) & 0xff) as u16;
    let func: EndTransferFunc = if (*s).nsector == 0 {
        ide_sector_read_cb_end
    } else {
        ide_sector_read
    };
    ide_transfer_start(s, 512i32.wrapping_mul(n), Some(func));
    ide_set_irq(s);
    (*s).status = 0x40 | 0x10 | 0x08;
    (*s).error = 0;
}

unsafe extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    ide_transfer_stop(s);
}

unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    ide_transfer_stop(s);
    let sector_num: i64 = ide_get_sector(s);

    let bs = (*s).bs;
    let buf = core::ptr::addr_of!((*s).io_buffer) as *const u8;
    let ret: c_int = match (*bs).write_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            buf,
            (*s).io_nb_sectors,
            Some(ide_sector_write_cb2),
            s as *mut c_void,
        ),
        None => 0,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_write_cb2(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
    }
}

unsafe extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let mut n: c_int = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s) + n as i64);
    (*s).nsector = (((*s).nsector as c_int - n) & 0xff) as u16;
    if (*s).nsector == 0 {
        (*s).status = 0x40 | 0x10;
    } else {
        n = (*s).nsector as c_int;
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), Some(ide_sector_write_cb1));
        (*s).status = 0x40 | 0x10 | 0x08;
    }
    ide_set_irq(s);
}

unsafe extern "C" fn ide_sector_write(s: *mut IDEState) {
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }
    (*s).io_nb_sectors = n;
    ide_transfer_start(s, 512i32.wrapping_mul(n), Some(ide_sector_write_cb1));
    (*s).status = 0x40 | 0x10 | 0x08;
}

unsafe extern "C" fn ide_identify_cb(s: *mut IDEState) {
    ide_transfer_stop(s);
    (*s).status = 0x40;
}

unsafe extern "C" fn ide_exec_cmd(s: *mut IDEState, val: c_int) {
    match val {
        0xEC => {
            ide_identify(s);
            (*s).status = 0x40 | 0x10 | 0x08;
            ide_transfer_start(s, 512, Some(ide_identify_cb));
            ide_set_irq(s);
        }
        0x91 | 0x10 => {
            (*s).error = 0;
            (*s).status = 0x40 | 0x10;
            ide_set_irq(s);
        }
        0xC6 => {
            let ns: c_int = (*s).nsector as c_int;
            if ns > 128 || (ns & (ns.wrapping_sub(1))) != 0 {
                ide_abort_command(s);
            } else {
                (*s).mult_sectors = ns;
                (*s).status = 0x40;
            }
            ide_set_irq(s);
        }
        0x20 | 0x21 => {
            (*s).req_nb_sectors = 1;
            ide_sector_read(s);
        }
        0x30 | 0x31 => {
            (*s).req_nb_sectors = 1;
            ide_sector_write(s);
        }
        0xC4 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_read(s);
            }
        }
        0xC5 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_write(s);
            }
        }
        0xF8 => {
            ide_set_sector(s, (*s).nb_sectors.wrapping_sub(1));
            (*s).status = 0x40;
            ide_set_irq(s);
        }
        _ => {
            ide_abort_command(s);
            ide_set_irq(s);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_ioport_write__part1219(
    s1: *mut IDEIFState,
    mut s: *mut IDEState,
    addr: c_int,
    val: u32,
) {
    match addr {
        0 => {}
        1 => {
            if !s.is_null() {
                (*s).feature = val as u8;
            }
        }
        2 => {
            if !s.is_null() {
                (*s).nsector = val as u16;
            }
        }
        3 => {
            if !s.is_null() {
                (*s).sector = val as u8;
            }
        }
        4 => {
            if !s.is_null() {
                (*s).lcyl = val as u8;
            }
        }
        5 => {
            if !s.is_null() {
                (*s).hcyl = val as u8;
            }
        }
        6 => {
            let d = if ((val >> 4) & 1) != 0 {
                (*s1).drives[1]
            } else {
                (*s1).drives[0]
            };
            (*s1).cur_drive = d;
            s = d;
            if !s.is_null() {
                (*s).select = val as u8;
            }
        }
        _ => {
            if !s.is_null() {
                ide_exec_cmd(s, val as c_int);
            }
        }
    }
}
}

// ── `ide_ioport_read` · ide.c:621-663 · chunk 1220 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t ide_ioport_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s = s1->cur_drive;
//         int ret, addr = offset + 1;
//     
//         if (!s) {
//             ret = 0x00;
//         } else {
//             switch(addr) {
//             case 0:
//                ret = 0xff;
//                break;
//             case 1:
//                 ret = s->error;
//                 break;
//             case 2:
//                 ret = s->nsector;
//                 break;
//             case 3:
//                 ret = s->sector;
//                 break;
//             case 4:
//                 ret = s->lcyl;
//                 break;
//             case 5:
//                 ret = s->hcyl;
//                 break;
//             case 6:
//                 ret = s->select;
//                 break;
//             default:
//             case 7:
//                 ret = s->status;
//                 set_irq(s1->irq, 0);
//                 break;
//             }
//         }
//     
//     
//     
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1220 {
// Port of the static C function `ide_ioport_read` from ide.c.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

/// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// BlockDevice and EndTransferFunc are only ever held as pointers here.
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if !irq.is_null() {
        let handler = (*irq).set_irq;
        if let Some(f) = handler {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ide_ioport_read(
    opaque: *mut c_void,
    offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s1 = opaque as *mut IDEIFState;
    let s = (*s1).cur_drive;
    let addr: c_int = offset.wrapping_add(1) as c_int;
    let ret: c_int;

    if s.is_null() {
        ret = 0x00;
    } else {
        match addr {
            0 => {
                ret = 0xff;
            }
            1 => {
                ret = (*s).error as c_int;
            }
            2 => {
                ret = (*s).nsector as c_int;
            }
            3 => {
                ret = (*s).sector as c_int;
            }
            4 => {
                ret = (*s).lcyl as c_int;
            }
            5 => {
                ret = (*s).hcyl as c_int;
            }
            6 => {
                ret = (*s).select as c_int;
            }
            _ => {
                // default: case 7
                ret = (*s).status as c_int;
                set_irq((*s1).irq, 0);
            }
        }
    }

    ret as u32
}
}

// ── `if (!s) … else` · ide.c:627-658 · chunk 1221 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s) {
//             ret = 0x00;
//         } else {
//             switch(addr) {
//             case 0:
//                ret = 0xff;
//                break;
//             case 1:
//                 ret = s->error;
//                 break;
//             case 2:
//                 ret = s->nsector;
//                 break;
//             case 3:
//                 ret = s->sector;
//                 break;
//             case 4:
//                 ret = s->lcyl;
//                 break;
//             case 5:
//                 ret = s->hcyl;
//                 break;
//             case 6:
//                 ret = s->select;
//                 break;
//             default:
//             case 7:
//                 ret = s->status;
//                 set_irq(s1->irq, 0);
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1221 {
// Rust port of the read-side ioport decode of ide_ioport_read().
// set_irq() is a `static inline` C helper with no linker symbol, so its logic
// (irq->set_irq(irq->opaque, irq->irq_num, level)) is inlined here.

use core::ffi::{c_int, c_void};

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// struct IDEState (only layout matters here)
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_ioport_read__part1221(
    s: *mut IDEState,
    irq: *mut IRQSignal,
    addr: c_int,
) -> c_int {
    unsafe {
        if s.is_null() {
            return 0x00;
        }
        match addr {
            0 => 0xff,
            1 => (*s).error as c_int,
            2 => (*s).nsector as c_int,
            3 => (*s).sector as c_int,
            4 => (*s).lcyl as c_int,
            5 => (*s).hcyl as c_int,
            6 => (*s).select as c_int,
            // default: / case 7:
            _ => {
                let ret = (*s).status as c_int;
                if !irq.is_null() {
                    if let Some(f) = (*irq).set_irq {
                        f((*irq).opaque, (*irq).irq_num, 0);
                    }
                }
                ret
            }
        }
    }
}
}

// ── `return ret;` · ide.c:662-662 · chunk 1222 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1222 {
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn ide_ioport_read__part1222(ret: c_int) -> c_uint {
    // C: `return ret;` from a function returning uint32_t.
    // Conversion from int to unsigned int is modular, as `as` does in Rust.
    ret as c_uint
}
}

// ── `ide_status_read` · ide.c:665-680 · chunk 1223 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t ide_status_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s = s1->cur_drive;
//         int ret;
//     
//         if (s) {
//             ret = s->status;
//         } else {
//             ret = 0;
//         }
//     
//     
//     
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1223 {
// Port of ide_status_read from QEMU's ide.c (preprocessed/old version).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[no_mangle]
pub extern "C" fn ide_status_read(opaque: *mut c_void, _offset: u32, _size_log2: c_int) -> u32 {
    let s1 = opaque as *mut IDEIFState;
    let s = unsafe { (*s1).cur_drive };
    let ret: c_int = if !s.is_null() {
        unsafe { (*s).status as c_int }
    } else {
        0
    };
    ret as u32
}
}

// ── `if (s) … else` · ide.c:671-675 · chunk 1224 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s) {
//             ret = s->status;
//         } else {
//             ret = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1224 {
// Rust port of the status-read piece of ide_status_read().
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_status_read__part1224(s: *mut IDEState) -> c_int {
    if !s.is_null() {
        (*s).status as c_int
    } else {
        0
    }
}
}

// ── `return ret;` · ide.c:679-679 · chunk 1225 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1225 {
// Returns the IDE status value read from the current drive, converted to
// uint32_t exactly as the C `return ret;` statement does.
#[no_mangle]
pub extern "C" fn ide_status_read__part1225(ret: core::ffi::c_int) -> u32 {
    ret as u32
}
}

// ── `ide_cmd_write` · ide.c:682-712 · chunk 1226 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_cmd_write(void *opaque, uint32_t offset,
//                               uint32_t val, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s;
//         int i;
//     
//     
//     
//     
//         if (!(s1->cmd & 0x04) && (val & 0x04)) {
//     
//             for(i = 0; i < 2; i++) {
//                 s = s1->drives[i];
//                 if (s) {
//                     s->status = 0x80 | 0x10;
//                     s->error = 0x01;
//                 }
//             }
//         } else if ((s1->cmd & 0x04) && !(val & 0x04)) {
//     
//             for(i = 0; i < 2; i++) {
//                 s = s1->drives[i];
//                 if (s) {
//                     s->status = 0x40 | 0x10;
//                     ide_set_signature(s);
//                 }
//             }
//         }
//         s1->cmd = val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1226 {
// Rust rewrite of ide.c: ide_cmd_write
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// static void ide_set_signature(IDEState *s)
#[inline]
unsafe fn ide_set_signature(s: *mut IDEState) {
    (*s).select &= 0xf0;
    (*s).nsector = 1;
    (*s).sector = 1;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_cmd_write(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;
    let mut i: c_int;
    let mut s: *mut IDEState;

    if ((*s1).cmd & 0x04) == 0 && (val & 0x04) != 0 {
        // low to high
        i = 0;
        while i < 2 {
            s = (*s1).drives[i as usize];
            if !s.is_null() {
                (*s).status = 0x80 | 0x10;
                (*s).error = 0x01;
            }
            i += 1;
        }
    } else if ((*s1).cmd & 0x04) != 0 && (val & 0x04) == 0 {
        // high to low
        i = 0;
        while i < 2 {
            s = (*s1).drives[i as usize];
            if !s.is_null() {
                (*s).status = 0x40 | 0x10;
                ide_set_signature(s);
            }
            i += 1;
        }
    }
    (*s1).cmd = val as u8;
}
}

// ── `if (!(s1->cmd & 0x04) && (val & 0x04) … else` · ide.c:692-710 · chunk 1227 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s1->cmd & 0x04) && (val & 0x04)) {
//     
//             for(i = 0; i < 2; i++) {
//                 s = s1->drives[i];
//                 if (s) {
//                     s->status = 0x80 | 0x10;
//                     s->error = 0x01;
//                 }
//             }
//         } else if ((s1->cmd & 0x04) && !(val & 0x04)) {
//     
//             for(i = 0; i < 2; i++) {
//                 s = s1->drives[i];
//                 if (s) {
//                     s->status = 0x40 | 0x10;
//                     ide_set_signature(s);
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1227 {
// Rust port of the device-reset/signature part of ide_cmd_write.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

// Ported logic of the static C helper ide_set_signature().
unsafe fn ide_set_signature(s: *mut IDEState) {
    (*s).select &= 0xf0;
    (*s).nsector = 1;
    (*s).sector = 1;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_cmd_write__part1227(s1: *mut IDEIFState, val: u32) {
    let cmd = (*s1).cmd;

    if (cmd & 0x04) == 0 && (val & 0x04) != 0 {
        let mut i: usize = 0;
        while i < 2 {
            let s = (*s1).drives[i];
            if !s.is_null() {
                (*s).status = 0x80 | 0x10;
                (*s).error = 0x01;
            }
            i += 1;
        }
    } else if (cmd & 0x04) != 0 && (val & 0x04) == 0 {
        let mut i: usize = 0;
        while i < 2 {
            let s = (*s1).drives[i];
            if !s.is_null() {
                (*s).status = 0x40 | 0x10;
                ide_set_signature(s);
            }
            i += 1;
        }
    }
}
}

// ── `s1->cmd = val;` · ide.c:711-711 · chunk 1228 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s1->cmd = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1228 {
// Port of the `s1->cmd = val;` statement from ide_cmd_write().
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ide_cmd_write__part1228(s1: *mut IDEIFState, val: u32) {
    if !s1.is_null() {
        (*s1).cmd = val as u8;
    }
}
}

// ── `ide_data_writew` · ide.c:714-732 · chunk 1229 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void ide_data_writew(void *opaque, uint32_t offset,
//                                 uint32_t val, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s = s1->cur_drive;
//         int p;
//         uint8_t *tab;
//     
//         if (!s)
//             return;
//         p = s->data_index;
//         tab = s->io_buffer;
//         tab[p] = val & 0xff;
//         tab[p + 1] = (val >> 8) & 0xff;
//         p += 2;
//         s->data_index = p;
//         if (p >= s->data_end)
//             s->end_transfer_func(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1229 {
// Port of ide_data_writew from ide.c
use core::ffi::{c_int, c_void};

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ide_data_writew(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;
    if s1.is_null() {
        return;
    }
    let s = (*s1).cur_drive;
    if s.is_null() {
        return;
    }
    let mut p = (*s).data_index;
    let tab = (*s).io_buffer.as_mut_ptr();
    *tab.offset(p as isize) = (val & 0xff) as u8;
    *tab.offset(p as isize + 1) = ((val >> 8) & 0xff) as u8;
    p += 2;
    (*s).data_index = p;
    if p >= (*s).data_end {
        if let Some(f) = (*s).end_transfer_func {
            f(s);
        }
    }
}
}

// ── `if (!s)` · ide.c:722-723 · chunk 1230 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1230 {
// Null check extracted from ide_data_writew: reports whether the current
// drive pointer is NULL, in which case the caller must return early.
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn ide_data_writew__part1230(s: *mut c_void) -> c_int {
    if s.is_null() {
        1
    } else {
        0
    }
}
}

// ── `p = s->data_index; tab = s->io_buffer; tab[p] = val & 0xff; tab[p + 1] = (val >> 8) & 0xff; p +…` · ide.c:724-729 · chunk 1231 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p = s->data_index;
//         tab = s->io_buffer;
//         tab[p] = val & 0xff;
//         tab[p + 1] = (val >> 8) & 0xff;
//         p += 2;
//         s->data_index = p;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1231 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: *mut c_void,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_data_writew__part1231(s: *mut IDEState, val: u32) -> c_int {
    unsafe {
        let mut p: c_int = (*s).data_index;
        let tab: *mut u8 = (*s).io_buffer.as_mut_ptr();
        *tab.offset(p as isize) = (val & 0xff) as u8;
        *tab.offset(p as isize + 1) = ((val >> 8) & 0xff) as u8;
        p = p.wrapping_add(2);
        (*s).data_index = p;
        p
    }
}
}

// ── `if (p >= s->data_end)` · ide.c:730-731 · chunk 1232 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p >= s->data_end)
//             s->end_transfer_func(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1232 {
use core::ffi::{c_int, c_void};

// Mirror of the C `struct IDEState` (fields in exact declaration order).
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<unsafe extern "C" fn(*mut IDEState)>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub unsafe extern "C" fn ide_data_writew__part1232(p: c_int, s: *mut IDEState) {
    if p >= (*s).data_end {
        let f = (*s).end_transfer_func;
        if let Some(f) = f {
            f(s);
        }
    }
}
}

// ── `ide_data_readw` · ide.c:734-753 · chunk 1233 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t ide_data_readw(void *opaque, uint32_t offset, int size_log2)
//     {
//         IDEIFState *s1 = opaque;
//         IDEState *s = s1->cur_drive;
//         int p, ret;
//         uint8_t *tab;
//     
//         if (!s) {
//             ret = 0;
//         } else {
//             p = s->data_index;
//             tab = s->io_buffer;
//             ret = tab[p] | (tab[p + 1] << 8);
//             p += 2;
//             s->data_index = p;
//             if (p >= s->data_end)
//                 s->end_transfer_func(s);
//         }
//         return ret;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1233 {
// Rust rewrite of ide_data_readw (static uint32_t ide_data_readw(void *, uint32_t, int))

use core::ffi::{c_int, c_uint, c_void};

// Mirror of struct IDEIFState (ide.c)
#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

pub type EndTransferFunc = Option<unsafe extern "C" fn(s: *mut IDEState)>;

// Mirror of struct IDEState (ide.c)
#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: EndTransferFunc,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[no_mangle]
pub extern "C" fn ide_data_readw(opaque: *mut c_void, _offset: c_uint, _size_log2: c_int) -> c_uint {
    let ret: c_int;
    unsafe {
        let s1 = opaque as *mut IDEIFState;
        let s: *mut IDEState = if s1.is_null() {
            core::ptr::null_mut()
        } else {
            (*s1).cur_drive
        };
        if s.is_null() {
            ret = 0;
        } else {
            let mut p: c_int = (*s).data_index;
            let tab: *mut u8 = (*s).io_buffer.as_mut_ptr();
            let lo = *tab.offset(p as isize) as c_int;
            let hi = *tab.offset(p as isize + 1) as c_int;
            ret = lo | (hi << 8);
            p += 2;
            (*s).data_index = p;
            if p >= (*s).data_end {
                if let Some(f) = (*s).end_transfer_func {
                    f(s);
                }
            }
        }
    }
    ret as c_uint
}
}

// ── `if (!s) … else` · ide.c:741-751 · chunk 1234 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s) {
//             ret = 0;
//         } else {
//             p = s->data_index;
//             tab = s->io_buffer;
//             ret = tab[p] | (tab[p + 1] << 8);
//             p += 2;
//             s->data_index = p;
//             if (p >= s->data_end)
//                 s->end_transfer_func(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1234 {
// Ported piece of ide_data_readw: the "if (!s) ... else ..." body that
// produces the returned word and advances/terminates the transfer.
use core::ffi::{c_int, c_void};

// typedef void EndTransferFunc(IDEState *);
type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
#[allow(dead_code)]
pub struct IDEState {
    ide_if: *mut IDEIFState,
    bs: *mut c_void,
    cylinders: c_int,
    heads: c_int,
    sectors: c_int,
    mult_sectors: c_int,
    nb_sectors: i64,
    feature: u8,
    error: u8,
    nsector: u16,
    sector: u8,
    lcyl: u8,
    hcyl: u8,
    select: u8,
    status: u8,
    io_nb_sectors: c_int,
    req_nb_sectors: c_int,
    end_transfer_func: Option<EndTransferFunc>,
    data_index: c_int,
    data_end: c_int,
    io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
#[allow(dead_code)]
pub struct IDEIFState {
    irq: *mut c_void,
    cur_drive: *mut IDEState,
    drives: [*mut IDEState; 2],
    cmd: u8,
}

#[no_mangle]
pub unsafe extern "C" fn ide_data_readw__part1234(s1: *mut IDEIFState) -> u32 {
    let s: *mut IDEState = (*s1).cur_drive;
    let ret: c_int;
    if s.is_null() {
        ret = 0;
    } else {
        let mut p: c_int = (*s).data_index;
        let tab: *mut u8 = core::ptr::addr_of_mut!((*s).io_buffer) as *mut u8;
        ret = (*tab.offset(p as isize) as c_int) | ((*tab.offset(p as isize + 1) as c_int) << 8);
        p += 2;
        (*s).data_index = p;
        if p >= (*s).data_end {
            let f = (*s).end_transfer_func;
            match f {
                Some(end_transfer_func) => end_transfer_func(s),
                None => {}
            }
        }
    }
    ret as u32
}
}

// ── `return ret;` · ide.c:752-752 · chunk 1235 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1235 {
// Portion of ide_data_readw: return the value assembled in `ret`.
#[no_mangle]
pub extern "C" fn ide_data_readw__part1235(ret: core::ffi::c_int) -> core::ffi::c_uint {
    ret as core::ffi::c_uint
}
}

// ── `ide_drive_init` · ide.c:755-797 · chunk 1236 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static IDEState *ide_drive_init(IDEIFState *ide_if, BlockDevice *bs)
//     {
//         IDEState *s;
//         uint32_t cylinders;
//         uint64_t nb_sectors;
//     
//         s = malloc(sizeof(*s));
//         memset(s, 0, sizeof(*s));
//     
//         s->ide_if = ide_if;
//         s->bs = bs;
//     
//         nb_sectors = s->bs->get_sector_count(s->bs);
//         cylinders = nb_sectors / (16 * 63);
//         if (cylinders > 16383)
//             cylinders = 16383;
//         else if (cylinders < 2)
//             cylinders = 2;
//         s->cylinders = cylinders;
//         s->heads = 16;
//         s->sectors = 63;
//         s->nb_sectors = nb_sectors;
//     
//         s->mult_sectors = 128;
//     
//         s->feature = 0;
//         s->error = 0;
//         s->nsector = 0;
//         s->sector = 0;
//         s->lcyl = 0;
//         s->hcyl = 0;
//         s->select = 0xa0;
//         s->status = 0x40 | 0x10;
//     
//     
//         s->data_index = 0;
//         s->data_end = 0;
//         s->end_transfer_func = ide_transfer_stop;
//     
//         s->req_nb_sectors = 0;
//         s->io_nb_sectors = 0;
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1236 {
// Rust rewrite of static IDEState *ide_drive_init(IDEIFState *ide_if, BlockDevice *bs)

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IDEIFState {
    _opaque: [u8; 0],
}

// void (*)(void *opaque, int ret)
type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

type EndTransferFunc = unsafe extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

// Private port of the C static function ide_transfer_stop (no linker symbol).
unsafe extern "C" fn ide_transfer_stop_rs(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    (*s).end_transfer_func = Some(ide_transfer_stop_rs);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

#[no_mangle]
pub unsafe extern "C" fn ide_drive_init(
    ide_if: *mut IDEIFState,
    bs: *mut BlockDevice,
) -> *mut IDEState {
    let size = core::mem::size_of::<IDEState>();
    let s = malloc(size) as *mut IDEState;
    core::ptr::write_bytes(s as *mut u8, 0, size);

    (*s).ide_if = ide_if;
    (*s).bs = bs;

    let mut nb_sectors: u64 = 0;
    let bsd = (*s).bs;
    if let Some(f) = (*bsd).get_sector_count {
        nb_sectors = f(bsd) as u64;
    }

    // cylinders = nb_sectors / (16 * 63); truncated into a uint32_t
    let mut cylinders: u32 = (nb_sectors / (16u64 * 63u64)) as u32;
    if cylinders > 16383 {
        cylinders = 16383;
    } else if cylinders < 2 {
        cylinders = 2;
    }
    (*s).cylinders = cylinders as c_int;
    (*s).heads = 16;
    (*s).sectors = 63;
    (*s).nb_sectors = nb_sectors as i64;

    (*s).mult_sectors = 128;

    // ide regs
    (*s).feature = 0;
    (*s).error = 0;
    (*s).nsector = 0;
    (*s).sector = 0;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
    (*s).select = 0xa0;
    (*s).status = (0x40 | 0x10) as u8;

    // init I/O buffer
    (*s).data_index = 0;
    (*s).data_end = 0;
    (*s).end_transfer_func = Some(ide_transfer_stop_rs);

    (*s).req_nb_sectors = 0;
    (*s).io_nb_sectors = 0;
    s
}
}

// ── `IDEState *s; uint32_t cylinders; uint64_t nb_sectors; s = malloc(sizeof(*s)); memset(s, 0, size…` · ide.c:757-768 · chunk 1237 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     IDEState *s;
//         uint32_t cylinders;
//         uint64_t nb_sectors;
//     
//         s = malloc(sizeof(*s));
//         memset(s, 0, sizeof(*s));
//     
//         s->ide_if = ide_if;
//         s->bs = bs;
//     
//         nb_sectors = s->bs->get_sector_count(s->bs);
//         cylinders = nb_sectors / (16 * 63);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1237 {
// Portion of ide_drive_init: allocate and zero the IDEState, bind it to the
// interface and block device, and derive the CHS cylinder count.
use core::ffi::{c_int, c_void};

type BsGetSectorCount = unsafe extern "C" fn(*mut BlockDevice) -> i64;
type BsReadAsync = unsafe extern "C" fn(
    *mut BlockDevice,
    u64,
    *mut u8,
    c_int,
    Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    *mut c_void,
) -> c_int;
type BsWriteAsync = unsafe extern "C" fn(
    *mut BlockDevice,
    u64,
    *const u8,
    c_int,
    Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    *mut c_void,
) -> c_int;

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<BsGetSectorCount>,
    pub read_async: Option<BsReadAsync>,
    pub write_async: Option<BsWriteAsync>,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IdeIfState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IdeState,
    pub drives: [*mut IdeState; 2],
    pub cmd: u8,
}

#[repr(C)]
pub struct IdeState {
    pub ide_if: *mut IdeIfState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<unsafe extern "C" fn(*mut IdeState)>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn ide_drive_init__part1237(
    ide_if: *mut IdeIfState,
    bs: *mut BlockDevice,
    out_cylinders: *mut u32,
    out_nb_sectors: *mut u64,
) -> *mut IdeState {
    let sz = core::mem::size_of::<IdeState>();
    let s = malloc(sz) as *mut IdeState;
    memset(s as *mut c_void, 0, sz);

    (*s).ide_if = ide_if;
    (*s).bs = bs;

    let mut nb_sectors: u64 = 0;
    if !bs.is_null() {
        match (*bs).get_sector_count {
            Some(get_sector_count) => {
                nb_sectors = get_sector_count(bs) as u64;
            }
            None => {}
        }
    }
    let cylinders: u32 = (nb_sectors / (16 * 63)) as u32;

    if !out_cylinders.is_null() {
        *out_cylinders = cylinders;
    }
    if !out_nb_sectors.is_null() {
        *out_nb_sectors = nb_sectors;
    }
    s
}
}

// ── `if (cylinders > 16383) … else` · ide.c:769-772 · chunk 1238 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cylinders > 16383)
//             cylinders = 16383;
//         else if (cylinders < 2)
//             cylinders = 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1238 {
// Clamp the cylinder count computed from the sector count.
// C: if (cylinders > 16383) cylinders = 16383;
//    else if (cylinders < 2) cylinders = 2;
#[no_mangle]
pub extern "C" fn ide_drive_init__part1238(cylinders: *mut u32) {
    unsafe {
        if !cylinders.is_null() {
            let mut c: u32 = *cylinders;
            if c > 16383 {
                c = 16383;
            } else if c < 2 {
                c = 2;
            }
            *cylinders = c;
        }
    }
}
}

// ── `s->cylinders = cylinders; s->heads = 16; s->sectors = 63; s->nb_sectors = nb_sectors; s->mult_s…` · ide.c:773-796 · chunk 1239 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cylinders = cylinders;
//         s->heads = 16;
//         s->sectors = 63;
//         s->nb_sectors = nb_sectors;
//     
//         s->mult_sectors = 128;
//     
//         s->feature = 0;
//         s->error = 0;
//         s->nsector = 0;
//         s->sector = 0;
//         s->lcyl = 0;
//         s->hcyl = 0;
//         s->select = 0xa0;
//         s->status = 0x40 | 0x10;
//     
//     
//         s->data_index = 0;
//         s->data_end = 0;
//         s->end_transfer_func = ide_transfer_stop;
//     
//         s->req_nb_sectors = 0;
//         s->io_nb_sectors = 0;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1239 {
// Portion of ide_drive_init: initializes the IDEState register/transfer fields
// and returns the state pointer.

use core::ffi::{c_int, c_void};

type EndTransferFunc = extern "C" fn(*mut IDEState);

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut c_void,
    pub bs: *mut c_void,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransferFunc>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

// Replacement for the static C function ide_transfer_stop (no linker symbol).
// Stores itself as the end-transfer callback and clears the transfer window.
extern "C" fn ide_transfer_stop_impl(s: *mut IDEState) {
    if s.is_null() {
        return;
    }
    unsafe {
        let st = &mut *s;
        st.end_transfer_func = Some(ide_transfer_stop_impl);
        st.data_index = 0;
        st.data_end = 0;
    }
}

#[no_mangle]
pub extern "C" fn ide_drive_init__part1239(
    s: *mut IDEState,
    cylinders: u32,
    nb_sectors: u64,
) -> *mut IDEState {
    if !s.is_null() {
        unsafe {
            let st = &mut *s;
            st.cylinders = cylinders as c_int;
            st.heads = 16;
            st.sectors = 63;
            st.nb_sectors = nb_sectors as i64;

            st.mult_sectors = 128;

            st.feature = 0;
            st.error = 0;
            st.nsector = 0;
            st.sector = 0;
            st.lcyl = 0;
            st.hcyl = 0;
            st.select = 0xa0;
            st.status = 0x40 | 0x10;

            st.data_index = 0;
            st.data_end = 0;
            st.end_transfer_func = Some(ide_transfer_stop_impl);

            st.req_nb_sectors = 0;
            st.io_nb_sectors = 0;
        }
    }
    s
}
}

// ── `ide_init` · ide.c:799-826 · chunk 1240 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     IDEIFState *ide_init(PhysMemoryMap *port_map, uint32_t addr, uint32_t addr2,
//                          IRQSignal *irq, BlockDevice **tab_bs)
//     {
//         int i;
//         IDEIFState *s;
//     
//         s = malloc(sizeof(IDEIFState));
//         memset(s, 0, sizeof(*s));
//     
//         s->irq = irq;
//         s->cmd = 0;
//     
//         cpu_register_device(port_map, addr, 1, s, ide_data_readw, ide_data_writew,
//                             (1 << 1));
//         cpu_register_device(port_map, addr + 1, 7, s, ide_ioport_read, ide_ioport_write,
//                             (1 << 0));
//         if (addr2) {
//             cpu_register_device(port_map, addr2, 1, s, ide_status_read, ide_cmd_write,
//                                 (1 << 0));
//         }
//     
//         for(i = 0; i < 2; i++) {
//             if (tab_bs[i])
//                 s->drives[i] = ide_drive_init(s, tab_bs[i]);
//         }
//         s->cur_drive = s->drives[0];
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1240 {
// Port of ide_init() from ide.c, plus the file-static IDE helpers it
// references (they have no linker symbols, so their logic lives here).

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

type DevRead = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type DevWrite = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type EndTransfer = unsafe extern "C" fn(*mut IDEState);
type BlockCb = unsafe extern "C" fn(*mut c_void, c_int);
type SetIrq = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIrq>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockCb>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockCb>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,

    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,

    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<EndTransfer>,

    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut c_void,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DevRead>,
        write_func: Option<DevWrite>,
        devio_flags: c_int,
    ) -> *mut c_void;
}

/* ---------------------------------------------------------------- */
/* small helpers                                                     */
/* ---------------------------------------------------------------- */

unsafe fn do_set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn padstr(mut s: *mut u8, mut src: *const u8, len: c_int) {
    let mut i: c_int = 0;
    while i < len {
        let v: c_int;
        if *src != 0 {
            v = *src as c_int;
            src = src.add(1);
        } else {
            v = b' ' as c_int;
        }
        *((s as usize ^ 1) as *mut u8) = v as u8;
        s = s.add(1);
        i += 1;
    }
}

unsafe fn stw(buf: *mut u16, v: u16) {
    *buf = v;
}

/* ---------------------------------------------------------------- */
/* drive level helpers                                               */
/* ---------------------------------------------------------------- */

unsafe fn ide_set_signature(s: *mut IDEState) {
    (*s).select &= 0xf0;
    (*s).nsector = 1;
    (*s).sector = 1;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
}

unsafe fn ide_abort_command(s: *mut IDEState) {
    (*s).status = 0x40 | 0x01;
    (*s).error = 0x04;
}

unsafe fn ide_set_irq(s: *mut IDEState) {
    let ide_if = (*s).ide_if;
    if ((*ide_if).cmd & 0x02) == 0 {
        do_set_irq((*ide_if).irq, 1);
    }
}

unsafe fn ide_transfer_start(
    s: *mut IDEState,
    size: c_int,
    end_transfer_func: Option<EndTransfer>,
) {
    (*s).end_transfer_func = end_transfer_func;
    (*s).data_index = 0;
    (*s).data_end = size;
}

unsafe extern "C" fn ide_transfer_stop(s: *mut IDEState) {
    (*s).end_transfer_func = Some(ide_transfer_stop);
    (*s).data_index = 0;
    (*s).data_end = 0;
}

unsafe fn ide_get_sector(s: *mut IDEState) -> i64 {
    let sector_num: i64;
    if ((*s).select & 0x40) != 0 {
        let v = (((*s).select & 0x0f) as i32).wrapping_shl(24)
            | ((*s).hcyl as i32).wrapping_shl(16)
            | ((*s).lcyl as i32).wrapping_shl(8)
            | ((*s).sector as i32);
        sector_num = v as i64;
    } else {
        let a = (((*s).hcyl as i32).wrapping_shl(8) | ((*s).lcyl as i32))
            .wrapping_mul((*s).heads)
            .wrapping_mul((*s).sectors)
            .wrapping_add((((*s).select & 0x0f) as i32).wrapping_mul((*s).sectors))
            .wrapping_add(((*s).sector as i32).wrapping_sub(1));
        sector_num = a as i64;
    }
    sector_num
}

unsafe fn ide_set_sector(s: *mut IDEState, sector_num: i64) {
    if ((*s).select & 0x40) != 0 {
        (*s).select = (((*s).select & 0xf0) as i32 | ((sector_num >> 24) & 0x0f) as i32) as u8;
        (*s).hcyl = ((sector_num >> 16) & 0xff) as u8;
        (*s).lcyl = ((sector_num >> 8) & 0xff) as u8;
        (*s).sector = (sector_num & 0xff) as u8;
    } else {
        let d = ((*s).heads.wrapping_mul((*s).sectors)) as i64;
        let cyl = sector_num.wrapping_div(d) as u32;
        let r = sector_num.wrapping_rem(d) as u32;
        (*s).hcyl = ((cyl >> 8) & 0xff) as u8;
        (*s).lcyl = (cyl & 0xff) as u8;
        let secs = (*s).sectors as u32;
        (*s).select = (((*s).select & 0xf0) as u32 | (r.wrapping_div(secs) & 0x0f)) as u8;
        (*s).sector = r.wrapping_rem(secs).wrapping_add(1) as u8;
    }
}

unsafe fn ide_identify(s: *mut IDEState) {
    let tab = (*s).io_buffer.as_mut_ptr() as *mut u16;

    core::ptr::write_bytes(tab as *mut u8, 0, 512 * 2);

    stw(tab.add(0), 0x0040);
    stw(tab.add(1), (*s).cylinders as u16);
    stw(tab.add(3), (*s).heads as u16);
    stw(tab.add(4), (512i32.wrapping_mul((*s).sectors)) as u16);
    stw(tab.add(5), 512);
    stw(tab.add(6), (*s).sectors as u16);
    stw(tab.add(20), 3);
    stw(tab.add(21), 512);
    stw(tab.add(22), 4);
    padstr(
        tab.add(27) as *mut u8,
        b"RISCVEMU HARDDISK\0".as_ptr(),
        40,
    );
    stw(tab.add(47), 0x8000 | 128);
    stw(tab.add(48), 0);
    stw(tab.add(49), 1 << 9);
    stw(tab.add(51), 0x200);
    stw(tab.add(52), 0x200);
    stw(tab.add(54), (*s).cylinders as u16);
    stw(tab.add(55), (*s).heads as u16);
    stw(tab.add(56), (*s).sectors as u16);
    let oldsize: u32 = (*s)
        .cylinders
        .wrapping_mul((*s).heads)
        .wrapping_mul((*s).sectors) as u32;
    stw(tab.add(57), oldsize as u16);
    stw(tab.add(58), (oldsize >> 16) as u16);
    if (*s).mult_sectors != 0 {
        stw(tab.add(59), 0x100 | ((*s).mult_sectors as u16 & 0xff));
    }
    stw(tab.add(60), (*s).nb_sectors as u16);
    stw(tab.add(61), ((*s).nb_sectors >> 16) as u16);
    stw(tab.add(80), (1 << 1) | (1 << 2));
    stw(tab.add(82), 1 << 14);
    stw(tab.add(83), 1 << 14);
    stw(tab.add(84), 1 << 14);
    stw(tab.add(85), 1 << 14);
    stw(tab.add(86), 0);
    stw(tab.add(87), 1 << 14);
}

unsafe extern "C" fn ide_identify_cb(s: *mut IDEState) {
    ide_transfer_stop(s);
    (*s).status = 0x40;
}

unsafe extern "C" fn ide_sector_read_cb(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let n: c_int = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s).wrapping_add(n as i64));
    (*s).nsector = ((((*s).nsector as i32).wrapping_sub(n)) & 0xff) as u16;
    let func: Option<EndTransfer> = if (*s).nsector == 0 {
        Some(ide_sector_read_cb_end)
    } else {
        Some(ide_sector_read as EndTransfer)
    };
    ide_transfer_start(s, 512i32.wrapping_mul(n), func);
    ide_set_irq(s);
    (*s).status = 0x40 | 0x10 | 0x08;
    (*s).error = 0;
}

unsafe extern "C" fn ide_sector_read_cb_end(s: *mut IDEState) {
    (*s).status = 0x40 | 0x10;
    (*s).error = 0;
    ide_transfer_stop(s);
}

unsafe extern "C" fn ide_sector_read(s: *mut IDEState) {
    let sector_num = ide_get_sector(s);
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }

    (*s).io_nb_sectors = n;
    let bs = (*s).bs;
    let ret: c_int = match (*bs).read_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_mut_ptr(),
            n,
            Some(ide_sector_read_cb),
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_read_cb(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
        (*s).error = 0;
    }
}

unsafe extern "C" fn ide_sector_write_cb1(s: *mut IDEState) {
    ide_transfer_stop(s);
    let sector_num = ide_get_sector(s);

    let bs = (*s).bs;
    let ret: c_int = match (*bs).write_async {
        Some(f) => f(
            bs,
            sector_num as u64,
            (*s).io_buffer.as_ptr(),
            (*s).io_nb_sectors,
            Some(ide_sector_write_cb2),
            s as *mut c_void,
        ),
        None => -1,
    };
    if ret < 0 {
        ide_abort_command(s);
        ide_set_irq(s);
    } else if ret == 0 {
        ide_sector_write_cb2(s as *mut c_void, 0);
    } else {
        (*s).status = 0x40 | 0x10 | 0x80;
    }
}

unsafe extern "C" fn ide_sector_write_cb2(opaque: *mut c_void, _ret: c_int) {
    let s = opaque as *mut IDEState;
    let mut n: c_int = (*s).io_nb_sectors;

    n = (*s).io_nb_sectors;
    ide_set_sector(s, ide_get_sector(s).wrapping_add(n as i64));
    (*s).nsector = ((((*s).nsector as i32).wrapping_sub(n)) & 0xff) as u16;
    if (*s).nsector == 0 {
        (*s).status = 0x40 | 0x10;
    } else {
        n = (*s).nsector as c_int;
        if n > (*s).req_nb_sectors {
            n = (*s).req_nb_sectors;
        }
        (*s).io_nb_sectors = n;
        ide_transfer_start(s, 512i32.wrapping_mul(n), Some(ide_sector_write_cb1));
        (*s).status = 0x40 | 0x10 | 0x08;
    }
    ide_set_irq(s);
}

unsafe fn ide_sector_write(s: *mut IDEState) {
    let mut n: c_int = (*s).nsector as c_int;
    if n == 0 {
        n = 256;
    }
    if n > (*s).req_nb_sectors {
        n = (*s).req_nb_sectors;
    }
    (*s).io_nb_sectors = n;
    ide_transfer_start(s, 512i32.wrapping_mul(n), Some(ide_sector_write_cb1));
    (*s).status = 0x40 | 0x10 | 0x08;
}

unsafe fn ide_exec_cmd(s: *mut IDEState, val: c_int) {
    match val {
        0xEC => {
            ide_identify(s);
            (*s).status = 0x40 | 0x10 | 0x08;
            ide_transfer_start(s, 512, Some(ide_identify_cb));
            ide_set_irq(s);
        }
        0x91 | 0x10 => {
            (*s).error = 0;
            (*s).status = 0x40 | 0x10;
            ide_set_irq(s);
        }
        0xC6 => {
            let ns = (*s).nsector as c_int;
            if ns > 128 || (ns & (ns.wrapping_sub(1))) != 0 {
                ide_abort_command(s);
            } else {
                (*s).mult_sectors = (*s).nsector as c_int;
                (*s).status = 0x40;
            }
            ide_set_irq(s);
        }
        0x20 | 0x21 => {
            (*s).req_nb_sectors = 1;
            ide_sector_read(s);
        }
        0x30 | 0x31 => {
            (*s).req_nb_sectors = 1;
            ide_sector_write(s);
        }
        0xC4 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_read(s);
            }
        }
        0xC5 => {
            if (*s).mult_sectors == 0 {
                ide_abort_command(s);
                ide_set_irq(s);
            } else {
                (*s).req_nb_sectors = (*s).mult_sectors;
                ide_sector_write(s);
            }
        }
        0xF8 => {
            ide_set_sector(s, (*s).nb_sectors.wrapping_sub(1));
            (*s).status = 0x40;
            ide_set_irq(s);
        }
        _ => {
            ide_abort_command(s);
            ide_set_irq(s);
        }
    }
}

/* ---------------------------------------------------------------- */
/* interface level handlers                                          */
/* ---------------------------------------------------------------- */

unsafe extern "C" fn ide_ioport_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;
    let mut s: *mut IDEState = (*s1).cur_drive;
    let addr: c_int = offset.wrapping_add(1) as c_int;

    match addr {
        0 => {}
        1 => {
            if !s.is_null() {
                (*s).feature = val as u8;
            }
        }
        2 => {
            if !s.is_null() {
                (*s).nsector = val as u16;
            }
        }
        3 => {
            if !s.is_null() {
                (*s).sector = val as u8;
            }
        }
        4 => {
            if !s.is_null() {
                (*s).lcyl = val as u8;
            }
        }
        5 => {
            if !s.is_null() {
                (*s).hcyl = val as u8;
            }
        }
        6 => {
            s = (*s1).drives[((val >> 4) & 1) as usize];
            (*s1).cur_drive = s;
            if !s.is_null() {
                (*s).select = val as u8;
            }
        }
        _ => {
            if !s.is_null() {
                ide_exec_cmd(s, val as c_int);
            }
        }
    }
}

unsafe extern "C" fn ide_ioport_read(
    opaque: *mut c_void,
    offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s1 = opaque as *mut IDEIFState;
    let s: *mut IDEState = (*s1).cur_drive;
    let addr: c_int = offset.wrapping_add(1) as c_int;
    let ret: c_int;

    if s.is_null() {
        ret = 0x00;
    } else {
        ret = match addr {
            0 => 0xff,
            1 => (*s).error as c_int,
            2 => (*s).nsector as c_int,
            3 => (*s).sector as c_int,
            4 => (*s).lcyl as c_int,
            5 => (*s).hcyl as c_int,
            6 => (*s).select as c_int,
            _ => {
                do_set_irq((*s1).irq, 0);
                (*s).status as c_int
            }
        };
    }

    ret as u32
}

unsafe extern "C" fn ide_status_read(
    opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s1 = opaque as *mut IDEIFState;
    let s: *mut IDEState = (*s1).cur_drive;
    let ret: c_int;

    if !s.is_null() {
        ret = (*s).status as c_int;
    } else {
        ret = 0;
    }

    ret as u32
}

unsafe extern "C" fn ide_cmd_write(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;

    if ((*s1).cmd & 0x04) == 0 && (val & 0x04) != 0 {
        let mut i: c_int = 0;
        while i < 2 {
            let s = (*s1).drives[i as usize];
            if !s.is_null() {
                (*s).status = 0x80 | 0x10;
                (*s).error = 0x01;
            }
            i += 1;
        }
    } else if ((*s1).cmd & 0x04) != 0 && (val & 0x04) == 0 {
        let mut i: c_int = 0;
        while i < 2 {
            let s = (*s1).drives[i as usize];
            if !s.is_null() {
                (*s).status = 0x40 | 0x10;
                ide_set_signature(s);
            }
            i += 1;
        }
    }
    (*s1).cmd = val as u8;
}

unsafe extern "C" fn ide_data_writew(
    opaque: *mut c_void,
    _offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s1 = opaque as *mut IDEIFState;
    let s: *mut IDEState = (*s1).cur_drive;

    if s.is_null() {
        return;
    }
    let mut p: c_int = (*s).data_index;
    let tab: *mut u8 = (*s).io_buffer.as_mut_ptr();
    *tab.add(p as usize) = (val & 0xff) as u8;
    *tab.add(p.wrapping_add(1) as usize) = ((val >> 8) & 0xff) as u8;
    p = p.wrapping_add(2);
    (*s).data_index = p;
    if p >= (*s).data_end {
        if let Some(f) = (*s).end_transfer_func {
            f(s);
        }
    }
}

unsafe extern "C" fn ide_data_readw(
    opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s1 = opaque as *mut IDEIFState;
    let s: *mut IDEState = (*s1).cur_drive;
    let ret: c_int;

    if s.is_null() {
        ret = 0;
    } else {
        let mut p: c_int = (*s).data_index;
        let tab: *const u8 = (*s).io_buffer.as_ptr();
        ret = (*tab.add(p as usize) as c_int)
            | ((*tab.add(p.wrapping_add(1) as usize) as c_int) << 8);
        p = p.wrapping_add(2);
        (*s).data_index = p;
        if p >= (*s).data_end {
            if let Some(f) = (*s).end_transfer_func {
                f(s);
            }
        }
    }
    ret as u32
}

/* ---------------------------------------------------------------- */
/* drive creation                                                    */
/* ---------------------------------------------------------------- */

unsafe fn ide_drive_init(ide_if: *mut IDEIFState, bs: *mut BlockDevice) -> *mut IDEState {
    let s = malloc(core::mem::size_of::<IDEState>()) as *mut IDEState;
    core::ptr::write_bytes(s as *mut u8, 0, core::mem::size_of::<IDEState>());

    (*s).ide_if = ide_if;
    (*s).bs = bs;

    let nb_sectors: u64 = match (*bs).get_sector_count {
        Some(f) => f(bs) as u64,
        None => 0,
    };
    let mut cylinders: u32 = nb_sectors.wrapping_div(16 * 63) as u32;
    if cylinders > 16383 {
        cylinders = 16383;
    } else if cylinders < 2 {
        cylinders = 2;
    }
    (*s).cylinders = cylinders as c_int;
    (*s).heads = 16;
    (*s).sectors = 63;
    (*s).nb_sectors = nb_sectors as i64;

    (*s).mult_sectors = 128;

    (*s).feature = 0;
    (*s).error = 0;
    (*s).nsector = 0;
    (*s).sector = 0;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
    (*s).select = 0xa0;
    (*s).status = 0x40 | 0x10;

    (*s).data_index = 0;
    (*s).data_end = 0;
    (*s).end_transfer_func = Some(ide_transfer_stop);

    (*s).req_nb_sectors = 0;
    (*s).io_nb_sectors = 0;
    s
}

/* ---------------------------------------------------------------- */
/* the target                                                        */
/* ---------------------------------------------------------------- */

#[no_mangle]
pub unsafe extern "C" fn ide_init(
    port_map: *mut c_void,
    addr: u32,
    addr2: u32,
    irq: *mut IRQSignal,
    tab_bs: *mut *mut BlockDevice,
) -> *mut IDEIFState {
    let s = malloc(core::mem::size_of::<IDEIFState>()) as *mut IDEIFState;
    core::ptr::write_bytes(s as *mut u8, 0, core::mem::size_of::<IDEIFState>());

    (*s).irq = irq;
    (*s).cmd = 0;

    cpu_register_device(
        port_map,
        addr as u64,
        1,
        s as *mut c_void,
        Some(ide_data_readw),
        Some(ide_data_writew),
        1 << 1,
    );
    cpu_register_device(
        port_map,
        addr.wrapping_add(1) as u64,
        7,
        s as *mut c_void,
        Some(ide_ioport_read),
        Some(ide_ioport_write),
        1 << 0,
    );
    if addr2 != 0 {
        cpu_register_device(
            port_map,
            addr2 as u64,
            1,
            s as *mut c_void,
            Some(ide_status_read),
            Some(ide_cmd_write),
            1 << 0,
        );
    }

    let mut i: c_int = 0;
    while i < 2 {
        let bs = *tab_bs.add(i as usize);
        if !bs.is_null() {
            (*s).drives[i as usize] = ide_drive_init(s, bs);
        }
        i += 1;
    }
    (*s).cur_drive = (*s).drives[0];
    s
}
}

// ── `int i; IDEIFState *s; s = malloc(sizeof(IDEIFState)); memset(s, 0, sizeof(*s)); s->irq = irq; s…` · ide.c:802-814 · chunk 1241 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int i;
//         IDEIFState *s;
//     
//         s = malloc(sizeof(IDEIFState));
//         memset(s, 0, sizeof(*s));
//     
//         s->irq = irq;
//         s->cmd = 0;
//     
//         cpu_register_device(port_map, addr, 1, s, ide_data_readw, ide_data_writew,
//                             (1 << 1));
//         cpu_register_device(port_map, addr + 1, 7, s, ide_ioport_read, ide_ioport_write,
//                             (1 << 0));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1241 {
// Portion of ide_init(): allocate and zero the IDEIFState, set irq/cmd and
// register the two I/O port device ranges.

use core::ffi::{c_int, c_void};

type ReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
type WriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;

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
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct IDEState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut IRQSignal,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: ReadFunc,
        write_func: WriteFunc,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
}

#[no_mangle]
pub unsafe extern "C" fn ide_init__part1241(
    port_map: *mut PhysMemoryMap,
    addr: u32,
    irq: *mut IRQSignal,
    data_readw: ReadFunc,
    data_writew: WriteFunc,
    ioport_read: ReadFunc,
    ioport_write: WriteFunc,
) -> *mut IDEIFState {
    let size = core::mem::size_of::<IDEIFState>();
    let s = malloc(size) as *mut IDEIFState;
    memset(s as *mut c_void, 0, size);

    (*s).irq = irq;
    (*s).cmd = 0;

    cpu_register_device(
        port_map,
        addr as u64,
        1,
        s as *mut c_void,
        data_readw,
        data_writew,
        1 << 1,
    );
    cpu_register_device(
        port_map,
        addr.wrapping_add(1) as u64,
        7,
        s as *mut c_void,
        ioport_read,
        ioport_write,
        1 << 0,
    );

    s
}
}

// ── `if (addr2)` · ide.c:815-818 · chunk 1242 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr2) {
//             cpu_register_device(port_map, addr2, 1, s, ide_status_read, ide_cmd_write,
//                                 (1 << 0));
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1242 {
// Ported piece of ide_init(): the conditional registration of the IDE
// control/status port range when a second address (addr2) is given.
//
// The device callbacks are `static` C functions of the same translation unit
// (they have no linker symbol), so they are handed to this function as
// pointers from the caller, preserving the exact functions the original code
// registered.

use core::ffi::{c_int, c_void};

// Opaque handles: only ever used behind a pointer.
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

pub type DeviceReadFunc = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
pub type DeviceWriteFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;

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
pub extern "C" fn ide_init__part1242(
    port_map: *mut PhysMemoryMap,
    addr2: u32,
    opaque: *mut c_void,
    read_func: DeviceReadFunc,
    write_func: DeviceWriteFunc,
) {
    if addr2 != 0 {
        unsafe {
            cpu_register_device(
                port_map,
                addr2 as u64,
                1 as u64,
                opaque,
                read_func,
                write_func,
                1 as c_int,
            );
        }
    }
}
}

// ── `for (i = 0; i < 2; i++)` · ide.c:820-823 · chunk 1243 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 2; i++) {
//             if (tab_bs[i])
//                 s->drives[i] = ide_drive_init(s, tab_bs[i]);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1243 {
// Rust port of the IDE drive initialization loop from ide_init(), together with
// the static helper ide_drive_init() which has no linker symbol of its own.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            *mut c_void,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            *mut c_void,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct IDEState {
    pub ide_if: *mut IDEIFState,
    pub bs: *mut BlockDevice,
    pub cylinders: c_int,
    pub heads: c_int,
    pub sectors: c_int,
    pub mult_sectors: c_int,
    pub nb_sectors: i64,
    pub feature: u8,
    pub error: u8,
    pub nsector: u16,
    pub sector: u8,
    pub lcyl: u8,
    pub hcyl: u8,
    pub select: u8,
    pub status: u8,
    pub io_nb_sectors: c_int,
    pub req_nb_sectors: c_int,
    pub end_transfer_func: Option<extern "C" fn(*mut IDEState)>,
    pub data_index: c_int,
    pub data_end: c_int,
    pub io_buffer: [u8; 128 * 512 + 4],
}

#[repr(C)]
pub struct IDEIFState {
    pub irq: *mut c_void,
    pub cur_drive: *mut IDEState,
    pub drives: [*mut IDEState; 2],
    pub cmd: u8,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

// Private port of the C `static void ide_transfer_stop(IDEState *s)`.
extern "C" fn ide_transfer_stop_rs(s: *mut IDEState) {
    unsafe {
        (*s).end_transfer_func = Some(ide_transfer_stop_rs);
        (*s).data_index = 0;
        (*s).data_end = 0;
    }
}

// Private port of the C `static IDEState *ide_drive_init(IDEIFState *, BlockDevice *)`.
unsafe fn ide_drive_init(ide_if: *mut IDEIFState, bs: *mut BlockDevice) -> *mut IDEState {
    let s = malloc(core::mem::size_of::<IDEState>()) as *mut IDEState;
    memset(s as *mut c_void, 0, core::mem::size_of::<IDEState>());

    (*s).ide_if = ide_if;
    (*s).bs = bs;

    let mut nb_sectors: u64 = 0;
    if let Some(f) = (*bs).get_sector_count {
        nb_sectors = f(bs) as u64;
    }
    let mut cylinders: u32 = (nb_sectors / (16 * 63)) as u32;
    if cylinders > 16383 {
        cylinders = 16383;
    } else if cylinders < 2 {
        cylinders = 2;
    }
    (*s).cylinders = cylinders as c_int;
    (*s).heads = 16;
    (*s).sectors = 63;
    (*s).nb_sectors = nb_sectors as i64;

    (*s).mult_sectors = 128;

    (*s).feature = 0;
    (*s).error = 0;
    (*s).nsector = 0;
    (*s).sector = 0;
    (*s).lcyl = 0;
    (*s).hcyl = 0;
    (*s).select = 0xa0;
    (*s).status = 0x40 | 0x10;

    (*s).data_index = 0;
    (*s).data_end = 0;
    (*s).end_transfer_func = Some(ide_transfer_stop_rs);

    (*s).req_nb_sectors = 0;
    (*s).io_nb_sectors = 0;
    s
}

#[no_mangle]
pub extern "C" fn ide_init__part1243(s: *mut IDEIFState, tab_bs: *mut *mut BlockDevice) {
    unsafe {
        let mut i: c_int = 0;
        while i < 2 {
            let bs: *mut BlockDevice = *tab_bs.offset(i as isize);
            if !bs.is_null() {
                (*s).drives[i as usize] = ide_drive_init(s, bs);
            }
            i += 1;
        }
    }
}
}

// ── `s->cur_drive = s->drives[0]; return s;` · ide.c:824-825 · chunk 1244 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->cur_drive = s->drives[0];
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1244 {
// Tail of ide_init: set the current drive to the first drive and hand the
// state pointer back to the C caller.

#[repr(C)]
pub struct IDEIFState {
    irq: *mut core::ffi::c_void,
    cur_drive: *mut core::ffi::c_void,
    drives: [*mut core::ffi::c_void; 2],
    cmd: u8,
}

#[no_mangle]
pub extern "C" fn ide_init__part1244(s: *mut IDEIFState) -> *mut IDEIFState {
    unsafe {
        (*s).cur_drive = (*s).drives[0];
    }
    s
}
}

// ── `piix3_ide_init` · ide.c:829-835 · chunk 1245 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIDevice *piix3_ide_init(PCIBus *pci_bus, int devfn)
//     {
//         PCIDevice *d;
//         d = pci_register_device(pci_bus, "PIIX3 IDE", devfn, 0x8086, 0x7010, 0x00, 0x0101);
//         pci_device_set_config8(d, 0x09, 0x00);
//         return d;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1245 {
// Port of piix3_ide_init from ide.c

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct PCIBus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

extern "C" {
    fn pci_register_device(
        b: *mut PCIBus,
        name: *const c_char,
        devfn: c_int,
        vendor_id: u16,
        device_id: u16,
        revision: u8,
        class_id: u16,
    ) -> *mut PCIDevice;
    fn pci_device_set_config8(d: *mut PCIDevice, addr: u8, val: u8);
}

static PIIX3_IDE_NAME: &[u8; 10] = b"PIIX3 IDE\0";

#[no_mangle]
pub extern "C" fn piix3_ide_init(pci_bus: *mut PCIBus, devfn: c_int) -> *mut PCIDevice {
    let d = unsafe {
        pci_register_device(
            pci_bus,
            PIIX3_IDE_NAME.as_ptr() as *const c_char,
            devfn,
            0x8086u16,
            0x7010u16,
            0x00u8,
            0x0101u16,
        )
    };
    unsafe {
        // ISA IDE ports, no DMA
        pci_device_set_config8(d, 0x09, 0x00);
    }
    d
}
}

// ── `PCIDevice *d; d = pci_register_device(pci_bus, "PIIX3 IDE", devfn, 0x8086, 0x7010, 0x00, 0x0101…` · ide.c:831-834 · chunk 1246 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIDevice *d;
//         d = pci_register_device(pci_bus, "PIIX3 IDE", devfn, 0x8086, 0x7010, 0x00, 0x0101);
//         pci_device_set_config8(d, 0x09, 0x00);
//         return d;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1246 {
// Port of ide.c lines 831-834 from piix3_ide_init.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // PCIDevice *pci_register_device(PCIBus *b, const char *name, int devfn,
    //                                uint16_t vendor_id, uint16_t device_id,
    //                                uint8_t revision, uint16_t class_id);
    fn pci_register_device(
        b: *mut c_void,
        name: *const c_char,
        devfn: c_int,
        vendor_id: u16,
        device_id: u16,
        revision: u8,
        class_id: u16,
    ) -> *mut c_void;

    // void pci_device_set_config8(PCIDevice *d, uint8_t addr, uint8_t val);
    fn pci_device_set_config8(d: *mut c_void, addr: u8, val: u8);
}

// "PIIX3 IDE" including its NUL terminator.
static DEV_NAME: [u8; 10] = *b"PIIX3 IDE\0";

#[no_mangle]
pub extern "C" fn piix3_ide_init__part1246(pci_bus: *mut c_void, devfn: c_int) -> *mut c_void {
    unsafe {
        let d = pci_register_device(
            pci_bus,
            DEV_NAME.as_ptr() as *const c_char,
            devfn,
            0x8086u16,
            0x7010u16,
            0x00u8,
            0x0101u16,
        );
        pci_device_set_config8(d, 0x09, 0x00);
        d
    }
}
}

