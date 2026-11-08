//! From `vga.c`.

// ── `vga_draw_glyph8` · vga.c:122-142 · chunk 3624 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_draw_glyph8(uint8_t *d, int linesize,
//                                 const uint8_t *font_ptr, int h,
//                                 uint32_t fgcol, uint32_t bgcol)
//     {
//         uint32_t font_data, xorcol;
//     
//         xorcol = bgcol ^ fgcol;
//         do {
//             font_data = font_ptr[0];
//             ((uint32_t *)d)[0] = (-((font_data >> 7)) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[1] = (-((font_data >> 6) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[2] = (-((font_data >> 5) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[3] = (-((font_data >> 4) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[4] = (-((font_data >> 3) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[5] = (-((font_data >> 2) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[6] = (-((font_data >> 1) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[7] = (-((font_data >> 0) & 1) & xorcol) ^ bgcol;
//             font_ptr++;
//             d += linesize;
//         } while (--h);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3624 {
// Port of vga_draw_glyph8 from vga.c
#![allow(clippy::all)]

use core::ffi::{c_int, c_uchar};

#[no_mangle]
pub unsafe extern "C" fn vga_draw_glyph8(
    d: *mut c_uchar,
    linesize: c_int,
    font_ptr: *const c_uchar,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let xorcol: u32 = bgcol ^ fgcol;
    let mut d = d;
    let mut font_ptr = font_ptr;
    let mut h = h;

    loop {
        let font_data: u32 = *font_ptr as u32;

        *d.add(0) as *mut u32;
        // Each pixel is written as a 32-bit value; use unaligned writes so we
        // do not rely on destination alignment.
        let px = d as *mut u32;
        core::ptr::write_unaligned(
            px.add(0),
            (font_data >> 7).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(1),
            ((font_data >> 6) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(2),
            ((font_data >> 5) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(3),
            ((font_data >> 4) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(4),
            ((font_data >> 3) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(5),
            ((font_data >> 2) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(6),
            ((font_data >> 1) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        core::ptr::write_unaligned(
            px.add(7),
            (font_data & 1).wrapping_neg() & xorcol ^ bgcol,
        );

        font_ptr = font_ptr.add(1);
        d = d.offset(linesize as isize);

        h = h.wrapping_sub(1);
        if h == 0 {
            break;
        }
    }
}
}

// ── `uint32_t font_data, xorcol; xorcol = bgcol ^ fgcol;` · vga.c:126-128 · chunk 3625 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t font_data, xorcol;
//     
//         xorcol = bgcol ^ fgcol;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3625 {
#[no_mangle]
pub extern "C" fn vga_draw_glyph8__part3625(fgcol: u32, bgcol: u32) -> u32 {
    bgcol ^ fgcol
}
}

// ── `do … while (--h)` · vga.c:129-141 · chunk 3626 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do {
//             font_data = font_ptr[0];
//             ((uint32_t *)d)[0] = (-((font_data >> 7)) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[1] = (-((font_data >> 6) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[2] = (-((font_data >> 5) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[3] = (-((font_data >> 4) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[4] = (-((font_data >> 3) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[5] = (-((font_data >> 2) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[6] = (-((font_data >> 1) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[7] = (-((font_data >> 0) & 1) & xorcol) ^ bgcol;
//             font_ptr++;
//             d += linesize;
//         } while (--h);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3626 {
use core::ffi::c_int;

// Ports the do/while loop of vga_draw_glyph8 (8 pixels per glyph row,
// unpacking one font byte into 8 32-bit pixels).
#[no_mangle]
pub unsafe extern "C" fn vga_draw_glyph8__part3626(
    d: *mut *mut u8,
    linesize: c_int,
    font_ptr: *mut *const u8,
    h: *mut c_int,
    xorcol: u32,
    bgcol: u32,
) {
    let mut dp: *mut u8 = *d;
    let mut fp: *const u8 = *font_ptr;
    let mut hh: c_int = *h;

    loop {
        let font_data: u32 = *fp as u32;
        let dst: *mut u32 = dp as *mut u32;

        core::ptr::write_unaligned(
            dst.add(0),
            ((font_data >> 7).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(1),
            (((font_data >> 6) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(2),
            (((font_data >> 5) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(3),
            (((font_data >> 4) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(4),
            (((font_data >> 3) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(5),
            (((font_data >> 2) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(6),
            (((font_data >> 1) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            dst.add(7),
            (((font_data >> 0) & 1u32).wrapping_neg() & xorcol) ^ bgcol,
        );

        fp = fp.add(1);
        dp = dp.offset(linesize as isize);
        hh -= 1;
        if hh == 0 {
            break;
        }
    }

    *d = dp;
    *font_ptr = fp;
    *h = hh;
}
}

// ── `vga_draw_glyph9` · vga.c:144-170 · chunk 3627 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_draw_glyph9(uint8_t *d, int linesize,
//                                 const uint8_t *font_ptr, int h,
//                                 uint32_t fgcol, uint32_t bgcol,
//                                 int dup9)
//     {
//         uint32_t font_data, xorcol, v;
//     
//         xorcol = bgcol ^ fgcol;
//         do {
//             font_data = font_ptr[0];
//             ((uint32_t *)d)[0] = (-((font_data >> 7)) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[1] = (-((font_data >> 6) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[2] = (-((font_data >> 5) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[3] = (-((font_data >> 4) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[4] = (-((font_data >> 3) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[5] = (-((font_data >> 2) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[6] = (-((font_data >> 1) & 1) & xorcol) ^ bgcol;
//             v = (-((font_data >> 0) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[7] = v;
//             if (dup9)
//                 ((uint32_t *)d)[8] = v;
//             else
//                 ((uint32_t *)d)[8] = bgcol;
//             font_ptr++;
//             d += linesize;
//         } while (--h);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3627 {
use core::ffi::c_int;
use core::ptr;

// Rewrite of the static C helper vga_draw_glyph9() from vga.c.
#[no_mangle]
pub unsafe extern "C" fn vga_draw_glyph9(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let xorcol: u32 = bgcol ^ fgcol;
    let mut d: *mut u8 = d;
    let mut font_ptr: *const u8 = font_ptr;
    let mut h: c_int = h;

    loop {
        let font_data: u32 = *font_ptr as u32;
        let p = d as *mut u32;
        ptr::write_unaligned(p.add(0), ((font_data >> 7).wrapping_neg() & xorcol) ^ bgcol);
        ptr::write_unaligned(
            p.add(1),
            (((font_data >> 6) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        ptr::write_unaligned(
            p.add(2),
            (((font_data >> 5) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        ptr::write_unaligned(
            p.add(3),
            (((font_data >> 4) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        ptr::write_unaligned(
            p.add(4),
            (((font_data >> 3) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        ptr::write_unaligned(
            p.add(5),
            (((font_data >> 2) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        ptr::write_unaligned(
            p.add(6),
            (((font_data >> 1) & 1).wrapping_neg() & xorcol) ^ bgcol,
        );
        let v: u32 = (((font_data >> 0) & 1).wrapping_neg() & xorcol) ^ bgcol;
        ptr::write_unaligned(p.add(7), v);
        if dup9 != 0 {
            ptr::write_unaligned(p.add(8), v);
        } else {
            ptr::write_unaligned(p.add(8), bgcol);
        }
        font_ptr = font_ptr.add(1);
        d = d.offset(linesize as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}
}

// ── `uint32_t font_data, xorcol, v; xorcol = bgcol ^ fgcol;` · vga.c:149-151 · chunk 3628 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t font_data, xorcol, v;
//     
//         xorcol = bgcol ^ fgcol;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3628 {
// Computes the xor colour mask used by vga_draw_glyph9.
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn vga_draw_glyph9__part3628(fgcol: c_uint, bgcol: c_uint) -> c_uint {
    bgcol ^ fgcol
}
}

// ── `do … while (--h)` · vga.c:152-169 · chunk 3629 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do {
//             font_data = font_ptr[0];
//             ((uint32_t *)d)[0] = (-((font_data >> 7)) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[1] = (-((font_data >> 6) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[2] = (-((font_data >> 5) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[3] = (-((font_data >> 4) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[4] = (-((font_data >> 3) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[5] = (-((font_data >> 2) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[6] = (-((font_data >> 1) & 1) & xorcol) ^ bgcol;
//             v = (-((font_data >> 0) & 1) & xorcol) ^ bgcol;
//             ((uint32_t *)d)[7] = v;
//             if (dup9)
//                 ((uint32_t *)d)[8] = v;
//             else
//                 ((uint32_t *)d)[8] = bgcol;
//             font_ptr++;
//             d += linesize;
//         } while (--h);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3629 {
// Port of the glyph9 pixel-writing loop of vga_draw_glyph9.

#[no_mangle]
pub extern "C" fn vga_draw_glyph9__part3629(
    mut d: *mut u8,
    linesize: core::ffi::c_int,
    mut font_ptr: *const u8,
    mut h: core::ffi::c_int,
    xorcol: u32,
    bgcol: u32,
    dup9: core::ffi::c_int,
) {
    loop {
        let font_data: u32 = unsafe { *font_ptr } as u32;
        let v: u32 = (font_data & 1).wrapping_neg() & xorcol ^ bgcol;
        unsafe {
            core::ptr::write_unaligned(
                d as *mut u32,
                (font_data >> 7).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(1),
                ((font_data >> 6) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(2),
                ((font_data >> 5) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(3),
                ((font_data >> 4) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(4),
                ((font_data >> 3) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(5),
                ((font_data >> 2) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned(
                (d as *mut u32).add(6),
                ((font_data >> 1) & 1).wrapping_neg() & xorcol ^ bgcol,
            );
            core::ptr::write_unaligned((d as *mut u32).add(7), v);
            if dup9 != 0 {
                core::ptr::write_unaligned((d as *mut u32).add(8), v);
            } else {
                core::ptr::write_unaligned((d as *mut u32).add(8), bgcol);
            }
        }
        font_ptr = unsafe { font_ptr.add(1) };
        d = unsafe { d.offset(linesize as isize) };
        h -= 1;
        if h == 0 {
            break;
        }
    }
}
}

// ── `c6_to_8` · vga.c:179-185 · chunk 3631 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int c6_to_8(int v)
//     {
//         int b;
//         v &= 0x3f;
//         b = v & 1;
//         return (v << 2) | (b << 1) | b;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3631 {
// Port of vga.c's static inline helper c6_to_8.
// Expands a 6-bit color component to 8 bits by replicating the low two bits.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn c6_to_8(v: c_int) -> c_int {
    let mut v = v;
    v &= 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}
}

// ── `int b; v &= 0x3f; b = v & 1; return (v << 2) | (b << 1) | b;` · vga.c:181-184 · chunk 3632 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int b;
//         v &= 0x3f;
//         b = v & 1;
//         return (v << 2) | (b << 1) | b;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3632 {
// Ported piece of c6_to_8: mask to 6 bits and expand to 8 bits.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn c6_to_8__part3632(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}
}

// ── `update_palette16` · vga.c:187-209 · chunk 3633 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int update_palette16(VGAState *s, uint32_t *palette)
//     {
//         int full_update, i;
//         uint32_t v, col;
//     
//         full_update = 0;
//         for(i = 0; i < 16; i++) {
//             v = s->ar[i];
//             if (s->ar[0x10] & 0x80)
//                 v = ((s->ar[0x14] & 0xf) << 4) | (v & 0xf);
//             else
//                 v = ((s->ar[0x14] & 0xc) << 4) | (v & 0x3f);
//             v = v * 3;
//             col = (c6_to_8(s->palette[v]) << 16) |
//                 (c6_to_8(s->palette[v + 1]) << 8) |
//                 c6_to_8(s->palette[v + 2]);
//             if (col != palette[i]) {
//                 full_update = 1;
//                 palette[i] = col;
//             }
//         }
//         return full_update;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3633 {
// Rust rewrite of update_palette16() from vga.c
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_int;

// Opaque pointer-only dependencies of VGAState.
// They are only ever stored as pointers, so mirror them as opaque types.
#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

// Mirror of the C `struct VGAState` layout (vga.c).
#[repr(C)]
pub struct VGAState {
    fb_dev: *mut FBDevice,
    fb_page_count: c_int,
    mem_range: *mut PhysMemoryRange,
    mem_range2: *mut PhysMemoryRange,
    pci_dev: *mut PCIDevice,
    rom_range: *mut PhysMemoryRange,

    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// Port of the static inline helper `c6_to_8` (no linker symbol).
#[inline]
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

#[no_mangle]
pub extern "C" fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    let mut full_update: c_int = 0;

    unsafe {
        let ar = (*s).ar.as_ptr();
        let raw_palette = (*s).palette.as_ptr();

        let mut i: c_int = 0;
        while i < 16 {
            let mut v: u32 = *ar.add(i as usize) as u32;
            if (*ar.add(0x10) & 0x80) != 0 {
                v = (((*ar.add(0x14) & 0xf) as u32) << 4) | (v & 0xf);
            } else {
                v = (((*ar.add(0x14) & 0xc) as u32) << 4) | (v & 0x3f);
            }
            v = v.wrapping_mul(3);

            let col: u32 = ((c6_to_8(*raw_palette.add(v as usize) as c_int) << 16)
                | (c6_to_8(*raw_palette.add(v as usize + 1) as c_int) << 8)
                | c6_to_8(*raw_palette.add(v as usize + 2) as c_int)) as u32;

            if col != *palette.add(i as usize) {
                full_update = 1;
                *palette.add(i as usize) = col;
            }

            i += 1;
        }
    }

    full_update
}
}

// ── `int full_update, i; uint32_t v, col; full_update = 0;` · vga.c:189-192 · chunk 3634 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int full_update, i;
//         uint32_t v, col;
//     
//         full_update = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3634 {
// Ported piece of update_palette16: the local declarations plus the
// initialisation `full_update = 0;`. The locals themselves stay declared
// in the C caller, which assigns them before any use.

#[no_mangle]
pub extern "C" fn update_palette16__part3634() -> core::ffi::c_int {
    // full_update = 0;
    0
}
}

// ── `for (i = 0; i < 16; i++)` · vga.c:193-207 · chunk 3635 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 16; i++) {
//             v = s->ar[i];
//             if (s->ar[0x10] & 0x80)
//                 v = ((s->ar[0x14] & 0xf) << 4) | (v & 0xf);
//             else
//                 v = ((s->ar[0x14] & 0xc) << 4) | (v & 0x3f);
//             v = v * 3;
//             col = (c6_to_8(s->palette[v]) << 16) |
//                 (c6_to_8(s->palette[v + 1]) << 8) |
//                 c6_to_8(s->palette[v + 2]);
//             if (col != palette[i]) {
//                 full_update = 1;
//                 palette[i] = col;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3635 {
// Ported piece of update_palette16: the 16-entry palette update loop.
use core::ffi::{c_int, c_uint};

// Mirrors the C struct VGAState layout exactly (pointers to opaque types
// modelled as *mut core::ffi::c_void, which has identical size/alignment).
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut core::ffi::c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut core::ffi::c_void,
    pub mem_range2: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub rom_range: *mut core::ffi::c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

// static inline int c6_to_8(int v) -- static in C, so ported as a private fn.
#[inline]
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

#[no_mangle]
pub unsafe extern "C" fn update_palette16__part3635(
    s: *mut VGAState,
    palette: *mut u32,
    full_update: *mut c_int,
    i_out: *mut c_int,
    v_out: *mut u32,
    col_out: *mut u32,
) {
    let ar = core::ptr::addr_of!((*s).ar) as *const u8;
    let pal6 = core::ptr::addr_of!((*s).palette) as *const u8;

    let mut i: c_int = 0;
    let mut v: u32 = 0;
    let mut col: u32 = 0;

    while i < 16 {
        v = *ar.add(i as usize) as u32;
        if (*ar.add(0x10) as c_int & 0x80) != 0 {
            v = (((*ar.add(0x14) as u32) & 0xf) << 4) | (v & 0xf);
        } else {
            v = (((*ar.add(0x14) as u32) & 0xc) << 4) | (v & 0x3f);
        }
        v = v.wrapping_mul(3);
        col = ((c6_to_8(*pal6.add(v as usize) as c_int) << 16) as c_uint
            | (c6_to_8(*pal6.add(v as usize + 1) as c_int) << 8) as c_uint
            | c6_to_8(*pal6.add(v as usize + 2) as c_int) as c_uint) as u32;
        if col != *palette.add(i as usize) {
            *full_update = 1;
            *palette.add(i as usize) = col;
        }
        i += 1;
    }

    // Loop is finished: hand the last loop-carried values back to the caller.
    *i_out = i;
    *v_out = v;
    *col_out = col;
}
}

// ── `return full_update;` · vga.c:208-208 · chunk 3636 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return full_update;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3636 {
// Tail of update_palette16: return the accumulated full_update flag.
#[no_mangle]
pub extern "C" fn update_palette16__part3636(full_update: core::ffi::c_int) -> core::ffi::c_int {
    full_update
}
}

// ── `vga_text_refresh` · vga.c:213-346 · chunk 3637 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_text_refresh(VGAState *s,
//                                  SimpleFBDrawFunc *redraw_func, void *opaque)
//     {
//         FBDevice *fb_dev = s->fb_dev;
//         int width, height, cwidth, cheight, cy, cx, x1, y1, width1, height1;
//         int cx_min, cx_max, dup9;
//         uint32_t ch_attr, line_offset, start_addr, ch_addr, ch_addr1, ch, cattr;
//         uint8_t *vga_ram, *font_ptr, *dst;
//         uint32_t fgcol, bgcol, cursor_offset, cursor_start, cursor_end;
//         BOOL full_update;
//     
//         full_update = update_palette16(s, s->last_palette);
//     
//         vga_ram = s->vga_ram;
//     
//         line_offset = s->cr[0x13];
//         line_offset <<= 3;
//         line_offset >>= 1;
//     
//         start_addr = s->cr[0x0d] | (s->cr[0x0c] << 8);
//     
//         cheight = (s->cr[9] & 0x1f) + 1;
//         cwidth = 8;
//         if (!(s->sr[1] & 0x01))
//             cwidth++;
//     
//         width = (s->cr[0x01] + 1);
//         height = s->cr[0x12] |
//             ((s->cr[0x07] & 0x02) << 7) |
//             ((s->cr[0x07] & 0x40) << 3);
//         height = (height + 1) / cheight;
//     
//         width1 = width * cwidth;
//         height1 = height * cheight;
//         if (fb_dev->width < width1 || fb_dev->height < height1 ||
//             width > 132 || height > 60)
//             return;
//         if (s->last_line_offset != line_offset ||
//             s->last_start_addr != start_addr ||
//             s->last_width != width ||
//             s->last_height != height) {
//             s->last_line_offset = line_offset;
//             s->last_start_addr = start_addr;
//             s->last_width = width;
//             s->last_height = height;
//             full_update = TRUE;
//         }
//     
//     
//         cursor_offset = ((s->cr[0x0e] << 8) | s->cr[0x0f]) - start_addr;
//         cursor_start = s->cr[0xa];
//         cursor_end = s->cr[0xb];
//         if (cursor_offset != s->last_cursor_offset ||
//             cursor_start != s->last_cursor_start ||
//             cursor_end != s->last_cursor_end) {
//     
//             if (s->last_cursor_offset < 132 * 60)
//                 s->last_ch_attr[s->last_cursor_offset] = -1;
//             if (cursor_offset < 132 * 60)
//                 s->last_ch_attr[cursor_offset] = -1;
//             s->last_cursor_offset = cursor_offset;
//             s->last_cursor_start = cursor_start;
//             s->last_cursor_end = cursor_end;
//         }
//     
//         ch_addr1 = 0x18000 + (start_addr * 2);
//         cursor_offset = 0x18000 + (start_addr + cursor_offset) * 2;
//     
//         x1 = (fb_dev->width - width1) / 2;
//         y1 = (fb_dev->height - height1) / 2;
//     
//     
//     
//     
//         for(cy = 0; cy < height; cy++) {
//             ch_addr = ch_addr1;
//             dst = fb_dev->fb_data + (y1 + cy * cheight) * fb_dev->stride + x1 * 4;
//             cx_min = width;
//             cx_max = -1;
//             for(cx = 0; cx < width; cx++) {
//                 ch_attr = *(uint16_t *)(vga_ram + (ch_addr & 0x1fffe));
//                 if (full_update || ch_attr != s->last_ch_attr[cy * width + cx]) {
//                     s->last_ch_attr[cy * width + cx] = ch_attr;
//                     cx_min = min_int(cx_min, cx);
//                     cx_max = max_int(cx_max, cx);
//                     ch = ch_attr & 0xff;
//                     cattr = ch_attr >> 8;
//                     font_ptr = vga_ram + 32 * ch;
//                     bgcol = s->last_palette[cattr >> 4];
//                     fgcol = s->last_palette[cattr & 0x0f];
//                     if (cwidth == 8) {
//                         vga_draw_glyph8(dst, fb_dev->stride, font_ptr, cheight,
//                                         fgcol, bgcol);
//                     } else {
//                         dup9 = 0;
//                         if (ch >= 0xb0 && ch <= 0xdf && (s->ar[0x10] & 0x04))
//                             dup9 = 1;
//                         vga_draw_glyph9(dst, fb_dev->stride, font_ptr, cheight,
//                                         fgcol, bgcol, dup9);
//                     }
//     
//                     if (cursor_offset == ch_addr && !(cursor_start & 0x20)) {
//                         int line_start, line_last, h;
//                         uint8_t *dst1;
//                         line_start = cursor_start & 0x1f;
//                         line_last = min_int(cursor_end & 0x1f, cheight - 1);
//     
//                         if (line_last >= line_start && line_start < cheight) {
//                             h = line_last - line_start + 1;
//                             dst1 = dst + fb_dev->stride * line_start;
//                             if (cwidth == 8) {
//                                 vga_draw_glyph8(dst1, fb_dev->stride,
//                                                 cursor_glyph,
//                                                 h, fgcol, bgcol);
//                             } else {
//                                 vga_draw_glyph9(dst1, fb_dev->stride,
//                                                 cursor_glyph,
//                                                 h, fgcol, bgcol, 1);
//                             }
//                         }
//                     }
//                 }
//                 ch_addr += 2;
//                 dst += 4 * cwidth;
//             }
//             if (cx_max >= cx_min) {
//     
//                 redraw_func(fb_dev, opaque,
//                             x1 + cx_min * cwidth, y1 + cy * cheight,
//                             (cx_max - cx_min + 1) * cwidth, cheight);
//             }
//             ch_addr1 += line_offset;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3637 {
// Port of static void vga_text_refresh(VGAState *s, SimpleFBDrawFunc *redraw_func, void *opaque)
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

pub type SimpleFBDrawFunc =
    unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int);

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, Option<SimpleFBDrawFunc>, *mut c_void)>,
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

const MAX_TEXT_WIDTH: c_int = 132;
const MAX_TEXT_HEIGHT: c_int = 60;

static CURSOR_GLYPH: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b { a } else { b }
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b { a } else { b }
}

#[inline]
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

unsafe fn vga_draw_glyph8(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let xorcol: u32 = bgcol ^ fgcol;
    let mut d = d;
    let mut fp = font_ptr;
    let mut h = h;
    loop {
        let font_data: u32 = *fp as u32;
        let p = d as *mut u32;
        core::ptr::write_unaligned(p, (0u32.wrapping_sub(font_data >> 7) & xorcol) ^ bgcol);
        core::ptr::write_unaligned(
            p.add(1),
            (0u32.wrapping_sub((font_data >> 6) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(2),
            (0u32.wrapping_sub((font_data >> 5) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(3),
            (0u32.wrapping_sub((font_data >> 4) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(4),
            (0u32.wrapping_sub((font_data >> 3) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(5),
            (0u32.wrapping_sub((font_data >> 2) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(6),
            (0u32.wrapping_sub((font_data >> 1) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(7),
            (0u32.wrapping_sub(font_data & 1) & xorcol) ^ bgcol,
        );
        fp = fp.add(1);
        d = d.offset(linesize as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

unsafe fn vga_draw_glyph9(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let xorcol: u32 = bgcol ^ fgcol;
    let mut d = d;
    let mut fp = font_ptr;
    let mut h = h;
    loop {
        let font_data: u32 = *fp as u32;
        let p = d as *mut u32;
        core::ptr::write_unaligned(p, (0u32.wrapping_sub(font_data >> 7) & xorcol) ^ bgcol);
        core::ptr::write_unaligned(
            p.add(1),
            (0u32.wrapping_sub((font_data >> 6) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(2),
            (0u32.wrapping_sub((font_data >> 5) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(3),
            (0u32.wrapping_sub((font_data >> 4) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(4),
            (0u32.wrapping_sub((font_data >> 3) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(5),
            (0u32.wrapping_sub((font_data >> 2) & 1) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            p.add(6),
            (0u32.wrapping_sub((font_data >> 1) & 1) & xorcol) ^ bgcol,
        );
        let v: u32 = (0u32.wrapping_sub(font_data & 1) & xorcol) ^ bgcol;
        core::ptr::write_unaligned(p.add(7), v);
        if dup9 != 0 {
            core::ptr::write_unaligned(p.add(8), v);
        } else {
            core::ptr::write_unaligned(p.add(8), bgcol);
        }
        fp = fp.add(1);
        d = d.offset(linesize as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

unsafe fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    let mut full_update: c_int = 0;
    let mut i: c_int = 0;
    while i < 16 {
        let mut v: u32 = (*s).ar[i as usize] as u32;
        if ((*s).ar[0x10] & 0x80) != 0 {
            v = ((((*s).ar[0x14] & 0x0f) as u32) << 4) | (v & 0xf);
        } else {
            v = ((((*s).ar[0x14] & 0x0c) as u32) << 4) | (v & 0x3f);
        }
        v = v.wrapping_mul(3);
        let vi = v as usize;
        let col: u32 = ((c6_to_8((*s).palette[vi] as c_int) as u32) << 16)
            | ((c6_to_8((*s).palette[vi + 1] as c_int) as u32) << 8)
            | (c6_to_8((*s).palette[vi + 2] as c_int) as u32);
        let slot = palette.add(i as usize);
        if col != *slot {
            full_update = 1;
            *slot = col;
        }
        i += 1;
    }
    full_update
}

#[no_mangle]
pub unsafe extern "C" fn vga_text_refresh(
    s: *mut VGAState,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
) {
    let sp: *mut VGAState = s;
    let fb_dev: *mut FBDevice = (*sp).fb_dev;

    let mut full_update: c_int = update_palette16(sp, (*sp).last_palette.as_mut_ptr());

    let vga_ram: *mut u8 = (*sp).vga_ram;

    let mut line_offset: u32 = (*sp).cr[0x13] as u32;
    line_offset <<= 3;
    line_offset >>= 1;

    let start_addr: u32 = ((*sp).cr[0x0d] as u32) | (((*sp).cr[0x0c] as u32) << 8);

    let cheight: c_int = (((*sp).cr[9] & 0x1f) as c_int) + 1;
    let mut cwidth: c_int = 8;
    if ((*sp).sr[1] & 0x01) == 0 {
        cwidth += 1;
    }

    let width: c_int = ((*sp).cr[0x01] as c_int) + 1;
    let mut height: c_int = ((*sp).cr[0x12] as c_int)
        | ((((*sp).cr[0x07] & 0x02) as c_int) << 7)
        | ((((*sp).cr[0x07] & 0x40) as c_int) << 3);
    height = (height + 1) / cheight;

    let width1: c_int = width * cwidth;
    let height1: c_int = height * cheight;
    if (*fb_dev).width < width1
        || (*fb_dev).height < height1
        || width > MAX_TEXT_WIDTH
        || height > MAX_TEXT_HEIGHT
    {
        return;
    }
    if (*sp).last_line_offset != line_offset as u16
        || (*sp).last_start_addr != start_addr as u16
        || (*sp).last_width != width as u32
        || (*sp).last_height != height as u32
    {
        (*sp).last_line_offset = line_offset as u16;
        (*sp).last_start_addr = start_addr as u16;
        (*sp).last_width = width as u32;
        (*sp).last_height = height as u32;
        full_update = 1;
    }

    /* update cursor position */
    let mut cursor_offset: u32 =
        ((((*sp).cr[0x0e] as u32) << 8) | ((*sp).cr[0x0f] as u32)).wrapping_sub(start_addr);
    let cursor_start: u32 = (*sp).cr[0xa] as u32;
    let cursor_end: u32 = (*sp).cr[0xb] as u32;
    if cursor_offset != (*sp).last_cursor_offset as u32
        || cursor_start != (*sp).last_cursor_start as u32
        || cursor_end != (*sp).last_cursor_end as u32
    {
        /* force refresh of characters with the cursor */
        let last_off = (*sp).last_cursor_offset as usize;
        if last_off < (MAX_TEXT_WIDTH as usize) * (MAX_TEXT_HEIGHT as usize) {
            (*sp).last_ch_attr[last_off] = 0xffffu16;
        }
        if cursor_offset < (MAX_TEXT_WIDTH as u32) * (MAX_TEXT_HEIGHT as u32) {
            (*sp).last_ch_attr[cursor_offset as usize] = 0xffffu16;
        }
        (*sp).last_cursor_offset = cursor_offset as u16;
        (*sp).last_cursor_start = cursor_start as u8;
        (*sp).last_cursor_end = cursor_end as u8;
    }

    let mut ch_addr1: u32 = 0x18000u32.wrapping_add(start_addr.wrapping_mul(2));
    cursor_offset =
        0x18000u32.wrapping_add(start_addr.wrapping_add(cursor_offset).wrapping_mul(2));

    let x1: c_int = ((*fb_dev).width - width1) / 2;
    let y1: c_int = ((*fb_dev).height - height1) / 2;

    let stride: c_int = (*fb_dev).stride;
    let fb_data: *mut u8 = (*fb_dev).fb_data;

    let mut cy: c_int = 0;
    while cy < height {
        let mut ch_addr: u32 = ch_addr1;
        let mut dst: *mut u8 = fb_data.offset(
            (y1 + cy * cheight) as isize * stride as isize + (x1 as isize) * 4,
        );
        let mut cx_min: c_int = width;
        let mut cx_max: c_int = -1;
        let mut cx: c_int = 0;
        while cx < width {
            let src = vga_ram.offset((ch_addr & 0x1fffe) as isize) as *const u16;
            let ch_attr: u32 = core::ptr::read_unaligned(src) as u32;
            let idx = (cy * width + cx) as usize;
            if full_update != 0 || ch_attr != (*sp).last_ch_attr[idx] as u32 {
                (*sp).last_ch_attr[idx] = ch_attr as u16;
                cx_min = min_int(cx_min, cx);
                cx_max = max_int(cx_max, cx);
                let ch: u32 = ch_attr & 0xff;
                let cattr: u32 = ch_attr >> 8;
                let font_ptr: *const u8 = vga_ram.offset((32u32.wrapping_mul(ch)) as isize);
                let bgcol: u32 = (*sp).last_palette[(cattr >> 4) as usize];
                let fgcol: u32 = (*sp).last_palette[(cattr & 0x0f) as usize];
                if cwidth == 8 {
                    vga_draw_glyph8(dst, stride, font_ptr, cheight, fgcol, bgcol);
                } else {
                    let mut dup9: c_int = 0;
                    if ch >= 0xb0 && ch <= 0xdf && ((*sp).ar[0x10] & 0x04) != 0 {
                        dup9 = 1;
                    }
                    vga_draw_glyph9(dst, stride, font_ptr, cheight, fgcol, bgcol, dup9);
                }
                /* cursor display */
                if cursor_offset == ch_addr && (cursor_start & 0x20) == 0 {
                    let line_start: c_int = (cursor_start & 0x1f) as c_int;
                    let line_last: c_int = min_int((cursor_end & 0x1f) as c_int, cheight - 1);

                    if line_last >= line_start && line_start < cheight {
                        let h: c_int = line_last - line_start + 1;
                        let dst1: *mut u8 =
                            dst.offset(stride as isize * line_start as isize);
                        if cwidth == 8 {
                            vga_draw_glyph8(
                                dst1,
                                stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                            );
                        } else {
                            vga_draw_glyph9(
                                dst1,
                                stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                                1,
                            );
                        }
                    }
                }
            }
            ch_addr = ch_addr.wrapping_add(2);
            dst = dst.offset((4 * cwidth) as isize);
            cx += 1;
        }
        if cx_max >= cx_min {
            if let Some(f) = redraw_func {
                f(
                    fb_dev,
                    opaque,
                    x1 + cx_min * cwidth,
                    y1 + cy * cheight,
                    (cx_max - cx_min + 1) * cwidth,
                    cheight,
                );
            }
        }
        ch_addr1 = ch_addr1.wrapping_add(line_offset);
        cy += 1;
    }
}
}

// ── `FBDevice *fb_dev = s->fb_dev; int width, height, cwidth, cheight, cy, cx, x1, y1, width1, heigh…` · vga.c:216-235 · chunk 3638 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FBDevice *fb_dev = s->fb_dev;
//         int width, height, cwidth, cheight, cy, cx, x1, y1, width1, height1;
//         int cx_min, cx_max, dup9;
//         uint32_t ch_attr, line_offset, start_addr, ch_addr, ch_addr1, ch, cattr;
//         uint8_t *vga_ram, *font_ptr, *dst;
//         uint32_t fgcol, bgcol, cursor_offset, cursor_start, cursor_end;
//         BOOL full_update;
//     
//         full_update = update_palette16(s, s->last_palette);
//     
//         vga_ram = s->vga_ram;
//     
//         line_offset = s->cr[0x13];
//         line_offset <<= 3;
//         line_offset >>= 1;
//     
//         start_addr = s->cr[0x0d] | (s->cr[0x0c] << 8);
//     
//         cheight = (s->cr[9] & 0x1f) + 1;
//         cwidth = 8;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3638 {
// Ported piece of vga_text_refresh: initialization of the text-mode refresh
// parameters (palette update, VRAM pointer, line offset, start address and
// character cell geometry).

use core::ffi::{c_int, c_void};

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
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

// Ported from the static C helper update_palette16 (no linker symbol).
fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    unsafe {
        let mut full_update: c_int = 0;
        let ar = (*s).ar.as_ptr();
        let pal = (*s).palette.as_ptr();
        let mut i: usize = 0;
        while i < 16 {
            let mut v = *ar.add(i) as c_int;
            if (*ar.add(0x10) & 0x80) != 0 {
                v = (((*ar.add(0x14) & 0xf) as c_int) << 4) | (v & 0xf);
            } else {
                v = (((*ar.add(0x14) & 0xc) as c_int) << 4) | (v & 0x3f);
            }
            v = v * 3;
            let col: u32 = ((c6_to_8(*pal.add(v as usize) as c_int) << 16)
                | (c6_to_8(*pal.add((v + 1) as usize) as c_int) << 8)
                | c6_to_8(*pal.add((v + 2) as usize) as c_int)) as u32;
            if col != *palette.add(i) {
                full_update = 1;
                *palette.add(i) = col;
            }
            i += 1;
        }
        full_update
    }
}

#[no_mangle]
pub extern "C" fn vga_text_refresh__part3638(
    s: *mut VGAState,
    p_fb_dev: *mut *mut FBDevice,
    p_vga_ram: *mut *mut u8,
    p_line_offset: *mut u32,
    p_start_addr: *mut u32,
    p_cheight: *mut c_int,
    p_cwidth: *mut c_int,
    p_full_update: *mut c_int,
) {
    unsafe {
        let full_update: c_int = update_palette16(s, (*s).last_palette.as_mut_ptr());

        let vga_ram: *mut u8 = (*s).vga_ram;

        let cr = (*s).cr.as_ptr();
        let sr = (*s).sr.as_ptr();

        let mut line_offset: u32 = *cr.add(0x13) as u32;
        line_offset <<= 3;
        line_offset >>= 1;

        let start_addr: u32 = (*cr.add(0x0d) as u32) | ((*cr.add(0x0c) as u32) << 8);

        let cheight: c_int = ((*cr.add(9) & 0x1f) as c_int) + 1;
        let cwidth: c_int = {
            let mut cw: c_int = 8;
            if (*sr.add(1) & 0x01) == 0 {
                cw += 1;
            }
            cw
        };
        // The C target stops at `cwidth = 8;`; keep the exact assignment here.
        let cwidth_after: c_int = 8;

        *p_fb_dev = (*s).fb_dev;
        *p_vga_ram = vga_ram;
        *p_line_offset = line_offset;
        *p_start_addr = start_addr;
        *p_cheight = cheight;
        *p_cwidth = cwidth_after;
        *p_full_update = full_update;

        // silence unused-variable style lints while retaining the width check
        let _ = cwidth;
    }
}
}

// ── `if (!(s->sr[1] & 0x01)` · vga.c:236-237 · chunk 3639 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->sr[1] & 0x01))
//             cwidth++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3639 {
// Ported piece of vga_text_refresh: 9-dot character width adjustment.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vga_text_refresh__part3639(sr1: u8, cwidth: c_int) -> c_int {
    let mut cwidth = cwidth;
    if (sr1 as c_int) & 0x01 == 0 {
        cwidth = cwidth.wrapping_add(1);
    }
    cwidth
}
}

// ── `width = (s->cr[0x01] + 1); height = s->cr[0x12] | ((s->cr[0x07] & 0x02) << 7) | ((s->cr[0x07] &…` · vga.c:239-246 · chunk 3640 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     width = (s->cr[0x01] + 1);
//         height = s->cr[0x12] |
//             ((s->cr[0x07] & 0x02) << 7) |
//             ((s->cr[0x07] & 0x40) << 3);
//         height = (height + 1) / cheight;
//     
//         width1 = width * cwidth;
//         height1 = height * cheight;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3640 {
// Port of the geometry computation from vga_text_refresh (vga.c lines 239-246).
use core::ffi::c_int;

#[no_mangle]
pub unsafe extern "C" fn vga_text_refresh__part3640(
    cr: *const u8,
    cheight: c_int,
    cwidth: c_int,
    width: *mut c_int,
    height: *mut c_int,
    width1: *mut c_int,
    height1: *mut c_int,
) {
    // width = (s->cr[0x01] + 1);
    let w: c_int = *cr.add(0x01) as c_int + 1;

    // height = s->cr[0x12] | ((s->cr[0x07] & 0x02) << 7) | ((s->cr[0x07] & 0x40) << 3);
    let cr07: c_int = *cr.add(0x07) as c_int;
    let h_raw: c_int = (*cr.add(0x12) as c_int) | ((cr07 & 0x02) << 7) | ((cr07 & 0x40) << 3);

    // height = (height + 1) / cheight;
    let h: c_int = (h_raw + 1) / cheight;

    *width = w;
    *height = h;
    // width1 = width * cwidth;
    *width1 = w.wrapping_mul(cwidth);
    // height1 = height * cheight;
    *height1 = h.wrapping_mul(cheight);
}
}

// ── `if (fb_dev->width < width1 || fb_dev->height < height1 || width > 132 || height > 60)` · vga.c:247-249 · chunk 3641 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (fb_dev->width < width1 || fb_dev->height < height1 ||
//             width > 132 || height > 60)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3641 {
// Port of the geometry/fit check from vga_text_refresh.
// Returns 1 when the refresh must be aborted (early `return` in C), 0 otherwise.

#[repr(C)]
pub struct FBDevice {
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
    pub fb_data: *mut u8,
    pub fb_size: core::ffi::c_int,
    pub device_opaque: *mut core::ffi::c_void,
    pub refresh: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn vga_text_refresh__part3641(
    fb_dev: *mut FBDevice,
    width1: core::ffi::c_int,
    height1: core::ffi::c_int,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
) -> core::ffi::c_int {
    let dev = &*fb_dev;
    if dev.width < width1 || dev.height < height1 || width > 132 || height > 60 {
        1
    } else {
        0
    }
}
}

// ── `if (s->last_line_offset != line_offset || s->last_start_addr != start_addr || s->last_width !=…` · vga.c:250-259 · chunk 3642 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->last_line_offset != line_offset ||
//             s->last_start_addr != start_addr ||
//             s->last_width != width ||
//             s->last_height != height) {
//             s->last_line_offset = line_offset;
//             s->last_start_addr = start_addr;
//             s->last_width = width;
//             s->last_height = height;
//             full_update = TRUE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3642 {
// Port of vga_text_refresh lines 250-259: geometry-change detection.
// The VGAState geometry registers are passed by pointer so no struct mirror
// is needed; `full_update` is a BOOL (int) passed by pointer too.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vga_text_refresh__part3642(
    last_line_offset: *mut u16,
    last_start_addr: *mut u16,
    last_width: *mut u32,
    last_height: *mut u32,
    line_offset: u32,
    start_addr: u32,
    width: c_int,
    height: c_int,
    full_update: *mut c_int,
) {
    unsafe {
        // Usual arithmetic conversions: uint16_t fields promote to int and are
        // then converted to unsigned int to match the uint32_t values.
        let changed = (*last_line_offset as u32) != line_offset
            || (*last_start_addr as u32) != start_addr
            || (*last_width) != (width as u32)
            || (*last_height) != (height as u32);

        if changed {
            *last_line_offset = line_offset as u16;
            *last_start_addr = start_addr as u16;
            *last_width = width as u32;
            *last_height = height as u32;
            *full_update = 1; // TRUE
        }
    }
}
}

// ── `cursor_offset = ((s->cr[0x0e] << 8) | s->cr[0x0f]) - start_addr; cursor_start = s->cr[0xa]; cur…` · vga.c:262-264 · chunk 3643 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     cursor_offset = ((s->cr[0x0e] << 8) | s->cr[0x0f]) - start_addr;
//         cursor_start = s->cr[0xa];
//         cursor_end = s->cr[0xb];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3643 {
use core::ffi::{c_int, c_void};

// Mirror of the C struct VGAState (fields in order, exact C types).
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub unsafe extern "C" fn vga_text_refresh__part3643(
    s: *mut VGAState,
    start_addr: u32,
    cursor_offset: *mut u32,
    cursor_start: *mut u32,
    cursor_end: *mut u32,
) {
    if s.is_null() {
        return;
    }
    let cr: *const u8 = core::ptr::addr_of!((*s).cr) as *const u8;
    // ((s->cr[0x0e] << 8) | s->cr[0x0f]) - start_addr, computed in unsigned int
    let raw: u32 = ((*cr.add(0x0e) as u32) << 8) | (*cr.add(0x0f) as u32);
    if !cursor_offset.is_null() {
        *cursor_offset = raw.wrapping_sub(start_addr);
    }
    if !cursor_start.is_null() {
        *cursor_start = *cr.add(0x0a) as u32;
    }
    if !cursor_end.is_null() {
        *cursor_end = *cr.add(0x0b) as u32;
    }
}
}

// ── `if (cursor_offset != s->last_cursor_offset || cursor_start != s->last_cursor_start || cursor_en…` · vga.c:265-276 · chunk 3644 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cursor_offset != s->last_cursor_offset ||
//             cursor_start != s->last_cursor_start ||
//             cursor_end != s->last_cursor_end) {
//     
//             if (s->last_cursor_offset < 132 * 60)
//                 s->last_ch_attr[s->last_cursor_offset] = -1;
//             if (cursor_offset < 132 * 60)
//                 s->last_ch_attr[cursor_offset] = -1;
//             s->last_cursor_offset = cursor_offset;
//             s->last_cursor_start = cursor_start;
//             s->last_cursor_end = cursor_end;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3644 {
// Rust rewrite of the cursor-attribute invalidation block of vga_text_refresh().

#[repr(C)]
pub struct FBDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
}

// Mirror of `struct VGAState` (fields in exact C order).
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: core::ffi::c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut PCIDevice,
    pub rom_range: *mut PhysMemoryRange,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: core::ffi::c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub extern "C" fn vga_text_refresh__part3644(
    s: *mut VGAState,
    cursor_offset: u32,
    cursor_start: u32,
    cursor_end: u32,
) {
    if s.is_null() {
        return;
    }
    let st = unsafe { &mut *s };
    if cursor_offset != st.last_cursor_offset as u32
        || cursor_start != st.last_cursor_start as u32
        || cursor_end != st.last_cursor_end as u32
    {
        let base = st.last_ch_attr.as_mut_ptr();
        let old = st.last_cursor_offset as u32;
        if old < 132 * 60 {
            unsafe {
                *base.add(old as usize) = 0xffff_u16;
            }
        }
        if cursor_offset < 132 * 60 {
            unsafe {
                *base.add(cursor_offset as usize) = 0xffff_u16;
            }
        }
        st.last_cursor_offset = cursor_offset as u16;
        st.last_cursor_start = cursor_start as u8;
        st.last_cursor_end = cursor_end as u8;
    }
}
}

// ── `ch_addr1 = 0x18000 + (start_addr * 2); cursor_offset = 0x18000 + (start_addr + cursor_offset) *…` · vga.c:278-282 · chunk 3645 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ch_addr1 = 0x18000 + (start_addr * 2);
//         cursor_offset = 0x18000 + (start_addr + cursor_offset) * 2;
//     
//         x1 = (fb_dev->width - width1) / 2;
//         y1 = (fb_dev->height - height1) / 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3645 {
// Port of the address/position computation of vga_text_refresh (vga.c lines 278-282).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn vga_text_refresh__part3645(
    start_addr: u32,
    cursor_offset: u32,
    width1: c_int,
    height1: c_int,
    fb_width: c_int,
    fb_height: c_int,
    out_ch_addr1: *mut u32,
    out_cursor_offset: *mut u32,
    out_x1: *mut c_int,
    out_y1: *mut c_int,
) {
    unsafe {
        // ch_addr1 = 0x18000 + (start_addr * 2);
        *out_ch_addr1 = 0x18000u32.wrapping_add(start_addr.wrapping_mul(2));
        // cursor_offset = 0x18000 + (start_addr + cursor_offset) * 2;
        *out_cursor_offset =
            0x18000u32.wrapping_add(start_addr.wrapping_add(cursor_offset).wrapping_mul(2));
        // x1 = (fb_dev->width - width1) / 2;
        *out_x1 = fb_width.wrapping_sub(width1) / 2;
        // y1 = (fb_dev->height - height1) / 2;
        *out_y1 = fb_height.wrapping_sub(height1) / 2;
    }
}
}

// ── `for (cy = 0; cy < height; cy++)` · vga.c:287-345 · chunk 3646 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(cy = 0; cy < height; cy++) {
//             ch_addr = ch_addr1;
//             dst = fb_dev->fb_data + (y1 + cy * cheight) * fb_dev->stride + x1 * 4;
//             cx_min = width;
//             cx_max = -1;
//             for(cx = 0; cx < width; cx++) {
//                 ch_attr = *(uint16_t *)(vga_ram + (ch_addr & 0x1fffe));
//                 if (full_update || ch_attr != s->last_ch_attr[cy * width + cx]) {
//                     s->last_ch_attr[cy * width + cx] = ch_attr;
//                     cx_min = min_int(cx_min, cx);
//                     cx_max = max_int(cx_max, cx);
//                     ch = ch_attr & 0xff;
//                     cattr = ch_attr >> 8;
//                     font_ptr = vga_ram + 32 * ch;
//                     bgcol = s->last_palette[cattr >> 4];
//                     fgcol = s->last_palette[cattr & 0x0f];
//                     if (cwidth == 8) {
//                         vga_draw_glyph8(dst, fb_dev->stride, font_ptr, cheight,
//                                         fgcol, bgcol);
//                     } else {
//                         dup9 = 0;
//                         if (ch >= 0xb0 && ch <= 0xdf && (s->ar[0x10] & 0x04))
//                             dup9 = 1;
//                         vga_draw_glyph9(dst, fb_dev->stride, font_ptr, cheight,
//                                         fgcol, bgcol, dup9);
//                     }
//     
//                     if (cursor_offset == ch_addr && !(cursor_start & 0x20)) {
//                         int line_start, line_last, h;
//                         uint8_t *dst1;
//                         line_start = cursor_start & 0x1f;
//                         line_last = min_int(cursor_end & 0x1f, cheight - 1);
//     
//                         if (line_last >= line_start && line_start < cheight) {
//                             h = line_last - line_start + 1;
//                             dst1 = dst + fb_dev->stride * line_start;
//                             if (cwidth == 8) {
//                                 vga_draw_glyph8(dst1, fb_dev->stride,
//                                                 cursor_glyph,
//                                                 h, fgcol, bgcol);
//                             } else {
//                                 vga_draw_glyph9(dst1, fb_dev->stride,
//                                                 cursor_glyph,
//                                                 h, fgcol, bgcol, 1);
//                             }
//                         }
//                     }
//                 }
//                 ch_addr += 2;
//                 dst += 4 * cwidth;
//             }
//             if (cx_max >= cx_min) {
//     
//                 redraw_func(fb_dev, opaque,
//                             x1 + cx_min * cwidth, y1 + cy * cheight,
//                             (cx_max - cx_min + 1) * cwidth, cheight);
//             }
//             ch_addr1 += line_offset;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3646 {
// Rust port of the VGA text-mode refresh render loop.
// Private ports of the C file's static helpers (no linker symbols):
//   vga_draw_glyph8 / vga_draw_glyph9 / min_int / max_int / cursor_glyph

use core::ffi::{c_int, c_void};

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
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

static CURSOR_GLYPH: [u8; 32] = [0xffu8; 32];

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
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
unsafe fn put_px(d: *mut u8, idx: usize, v: u32) {
    core::ptr::write_unaligned(d.wrapping_add(4 * idx) as *mut u32, v);
}

unsafe fn vga_draw_glyph8(
    d0: *mut u8,
    linesize: c_int,
    font_ptr0: *const u8,
    h0: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let xorcol = bgcol ^ fgcol;
    let mut d = d0;
    let mut font_ptr = font_ptr0;
    let mut h = h0;
    loop {
        let font_data = *font_ptr as u32;
        put_px(d, 0, ((font_data >> 7).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 1, (((font_data >> 6) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 2, (((font_data >> 5) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 3, (((font_data >> 4) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 4, (((font_data >> 3) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 5, (((font_data >> 2) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 6, (((font_data >> 1) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 7, (((font_data >> 0) & 1).wrapping_neg() & xorcol) ^ bgcol);
        font_ptr = font_ptr.wrapping_add(1);
        d = d.wrapping_offset(linesize as isize);
        h = h.wrapping_sub(1);
        if h == 0 {
            break;
        }
    }
}

unsafe fn vga_draw_glyph9(
    d0: *mut u8,
    linesize: c_int,
    font_ptr0: *const u8,
    h0: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let xorcol = bgcol ^ fgcol;
    let mut d = d0;
    let mut font_ptr = font_ptr0;
    let mut h = h0;
    loop {
        let font_data = *font_ptr as u32;
        put_px(d, 0, ((font_data >> 7).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 1, (((font_data >> 6) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 2, (((font_data >> 5) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 3, (((font_data >> 4) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 4, (((font_data >> 3) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 5, (((font_data >> 2) & 1).wrapping_neg() & xorcol) ^ bgcol);
        put_px(d, 6, (((font_data >> 1) & 1).wrapping_neg() & xorcol) ^ bgcol);
        let v = (((font_data >> 0) & 1).wrapping_neg() & xorcol) ^ bgcol;
        put_px(d, 7, v);
        if dup9 != 0 {
            put_px(d, 8, v);
        } else {
            put_px(d, 8, bgcol);
        }
        font_ptr = font_ptr.wrapping_add(1);
        d = d.wrapping_offset(linesize as isize);
        h = h.wrapping_sub(1);
        if h == 0 {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn vga_text_refresh__part3646(
    s: *mut VGAState,
    fb_dev: *mut FBDevice,
    redraw_func: unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int),
    opaque: *mut c_void,
    vga_ram: *mut u8,
    width: c_int,
    height: c_int,
    cwidth: c_int,
    cheight: c_int,
    x1: c_int,
    y1: c_int,
    ch_addr1_in: u32,
    line_offset: u32,
    cursor_offset: u32,
    cursor_start: u32,
    cursor_end: u32,
    full_update: c_int,
) {
    let stride = (*fb_dev).stride;
    let fb_data = (*fb_dev).fb_data;
    let lca = (*s).last_ch_attr.as_mut_ptr();
    let pal = (*s).last_palette.as_ptr();
    let ar10 = (*s).ar[0x10];

    let mut ch_addr1 = ch_addr1_in;
    let mut cy: c_int = 0;
    while cy < height {
        let mut ch_addr = ch_addr1;
        let row_off = y1
            .wrapping_add(cy.wrapping_mul(cheight))
            .wrapping_mul(stride)
            .wrapping_add(x1.wrapping_mul(4));
        let mut dst = fb_data.wrapping_offset(row_off as isize);
        let mut cx_min: c_int = width;
        let mut cx_max: c_int = -1;
        let mut cx: c_int = 0;
        while cx < width {
            let ch_attr = core::ptr::read_unaligned(
                vga_ram.wrapping_add((ch_addr & 0x1fffe) as usize) as *const u16,
            ) as u32;
            let idx = cy.wrapping_mul(width).wrapping_add(cx) as usize;
            if full_update != 0 || ch_attr != *lca.add(idx) as u32 {
                *lca.add(idx) = ch_attr as u16;
                cx_min = min_int(cx_min, cx);
                cx_max = max_int(cx_max, cx);
                let ch = ch_attr & 0xff;
                let cattr = ch_attr >> 8;
                let font_ptr = vga_ram.wrapping_add(32u32.wrapping_mul(ch) as usize);
                let bgcol = *pal.add((cattr >> 4) as usize);
                let fgcol = *pal.add((cattr & 0x0f) as usize);
                if cwidth == 8 {
                    vga_draw_glyph8(dst, stride, font_ptr, cheight, fgcol, bgcol);
                } else {
                    let mut dup9: c_int = 0;
                    if ch >= 0xb0 && ch <= 0xdf && (ar10 & 0x04) != 0 {
                        dup9 = 1;
                    }
                    vga_draw_glyph9(dst, stride, font_ptr, cheight, fgcol, bgcol, dup9);
                }

                if cursor_offset == ch_addr && (cursor_start & 0x20) == 0 {
                    let line_start = (cursor_start & 0x1f) as c_int;
                    let line_last = min_int((cursor_end & 0x1f) as c_int, cheight.wrapping_sub(1));

                    if line_last >= line_start && line_start < cheight {
                        let h = line_last.wrapping_sub(line_start).wrapping_add(1);
                        let dst1 = dst.wrapping_offset(stride.wrapping_mul(line_start) as isize);
                        if cwidth == 8 {
                            vga_draw_glyph8(
                                dst1,
                                stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                            );
                        } else {
                            vga_draw_glyph9(
                                dst1,
                                stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                                1,
                            );
                        }
                    }
                }
            }
            ch_addr = ch_addr.wrapping_add(2);
            dst = dst.wrapping_offset((4 * cwidth) as isize);
            cx += 1;
        }
        if cx_max >= cx_min {
            redraw_func(
                fb_dev,
                opaque,
                x1.wrapping_add(cx_min.wrapping_mul(cwidth)),
                y1.wrapping_add(cy.wrapping_mul(cheight)),
                cx_max
                    .wrapping_sub(cx_min)
                    .wrapping_add(1)
                    .wrapping_mul(cwidth),
                cheight,
            );
        }
        ch_addr1 = ch_addr1.wrapping_add(line_offset);
        cy += 1;
    }
}
}

// ── `vga_refresh` · vga.c:348-362 · chunk 3647 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_refresh(FBDevice *fb_dev,
//                             SimpleFBDrawFunc *redraw_func, void *opaque)
//     {
//         VGAState *s = fb_dev->device_opaque;
//     
//         if (!(s->ar_index & 0x20)) {
//     
//         } else if (s->gr[0x06] & 1) {
//     
//             simplefb_refresh(fb_dev, redraw_func, opaque, s->mem_range, s->fb_page_count);
//         } else {
//     
//             vga_text_refresh(s, redraw_func, opaque);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3647 {
// Rust reimplementation of vga.c's vga_refresh plus the static helpers it
// reaches (vga_text_refresh, update_palette16, vga_draw_glyph8/9, c6_to_8,
// min_int/max_int). Those helpers are `static` in C, so they live here as
// private Rust functions.

use core::ffi::{c_int, c_void};
use core::ptr::{read_unaligned, write_unaligned};

pub type SimpleFBDrawFunc =
    unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int);

pub type RedrawFunc = Option<SimpleFBDrawFunc>;

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, RedrawFunc, *mut c_void)>,
}

/// Opaque mirror of C's PhysMemoryRange; only pointers to it are handled here.
#[repr(C)]
pub struct PhysMemoryRange {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut PhysMemoryRange,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

extern "C" {
    fn simplefb_refresh(
        fb_dev: *mut FBDevice,
        redraw_func: RedrawFunc,
        opaque: *mut c_void,
        mem_range: *mut PhysMemoryRange,
        fb_page_count: c_int,
    );
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

static CURSOR_GLYPH: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

#[inline]
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

unsafe fn vga_draw_glyph8(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let xorcol = bgcol ^ fgcol;
    let mut dp = d;
    let mut fp = font_ptr;
    let mut hh = h;
    loop {
        let font_data = *fp as u32;
        let q = dp as *mut u32;
        write_unaligned(q, (font_data >> 7).wrapping_neg() & xorcol ^ bgcol);
        write_unaligned(
            q.add(1),
            ((font_data >> 6) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(2),
            ((font_data >> 5) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(3),
            ((font_data >> 4) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(4),
            ((font_data >> 3) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(5),
            ((font_data >> 2) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(6),
            ((font_data >> 1) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(q.add(7), (font_data & 1).wrapping_neg() & xorcol ^ bgcol);
        fp = fp.add(1);
        dp = dp.offset(linesize as isize);
        hh = hh.wrapping_sub(1);
        if hh == 0 {
            break;
        }
    }
}

unsafe fn vga_draw_glyph9(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let xorcol = bgcol ^ fgcol;
    let mut dp = d;
    let mut fp = font_ptr;
    let mut hh = h;
    loop {
        let font_data = *fp as u32;
        let q = dp as *mut u32;
        write_unaligned(q, (font_data >> 7).wrapping_neg() & xorcol ^ bgcol);
        write_unaligned(
            q.add(1),
            ((font_data >> 6) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(2),
            ((font_data >> 5) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(3),
            ((font_data >> 4) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(4),
            ((font_data >> 3) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(5),
            ((font_data >> 2) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        write_unaligned(
            q.add(6),
            ((font_data >> 1) & 1).wrapping_neg() & xorcol ^ bgcol,
        );
        let v = (font_data & 1).wrapping_neg() & xorcol ^ bgcol;
        write_unaligned(q.add(7), v);
        if dup9 != 0 {
            write_unaligned(q.add(8), v);
        } else {
            write_unaligned(q.add(8), bgcol);
        }
        fp = fp.add(1);
        dp = dp.offset(linesize as isize);
        hh = hh.wrapping_sub(1);
        if hh == 0 {
            break;
        }
    }
}

unsafe fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    let mut full_update: c_int = 0;
    let ar = (*s).ar.as_ptr();
    let pal = (*s).palette.as_ptr();
    let mut i: usize = 0;
    while i < 16 {
        let mut v: u32 = *ar.add(i) as u32;
        if (*ar.add(0x10) & 0x80) != 0 {
            v = (((*ar.add(0x14) & 0xf) as u32) << 4) | (v & 0xf);
        } else {
            v = (((*ar.add(0x14) & 0xc) as u32) << 4) | (v & 0x3f);
        }
        v = v.wrapping_mul(3);
        let vi = v as usize;
        let col: u32 = ((c6_to_8(*pal.add(vi) as c_int) as u32) << 16)
            | ((c6_to_8(*pal.add(vi + 1) as c_int) as u32) << 8)
            | (c6_to_8(*pal.add(vi + 2) as c_int) as u32);
        let p = palette.add(i);
        if col != *p {
            full_update = 1;
            *p = col;
        }
        i += 1;
    }
    full_update
}

unsafe fn vga_text_refresh(s: *mut VGAState, redraw_func: RedrawFunc, opaque: *mut c_void) {
    let fb_dev = (*s).fb_dev;

    let mut cx_min: c_int;
    let mut cx_max: c_int;
    let mut ch_attr: u32;
    let mut ch_addr: u32;
    let mut ch_addr1: u32;
    let mut ch: u32;
    let mut cattr: u32;
    let mut font_ptr: *const u8;
    let mut dst: *mut u8;
    let mut fgcol: u32 = 0;
    let mut bgcol: u32 = 0;
    let mut dup9: c_int;

    let mut full_update: c_int =
        update_palette16(s, (*s).last_palette.as_mut_ptr());

    let vga_ram: *mut u8 = (*s).vga_ram;

    let cr = (*s).cr.as_ptr();

    let mut line_offset: u32 = *cr.add(0x13) as u32;
    line_offset <<= 3;
    line_offset >>= 1;

    let start_addr: u32 = (*cr.add(0x0d) as u32) | ((*cr.add(0x0c) as u32) << 8);

    let cheight: c_int = (*cr.add(9) & 0x1f) as c_int + 1;
    let mut cwidth: c_int = 8;
    if ((*s).sr[1] & 0x01) == 0 {
        cwidth += 1;
    }

    let width: c_int = *cr.add(0x01) as c_int + 1;
    let mut height: c_int = (*cr.add(0x12) as c_int)
        | (((*cr.add(0x07) & 0x02) as c_int) << 7)
        | (((*cr.add(0x07) & 0x40) as c_int) << 3);
    height = (height.wrapping_add(1)) / cheight;

    let width1: c_int = width.wrapping_mul(cwidth);
    let height1: c_int = height.wrapping_mul(cheight);
    if (*fb_dev).width < width1 || (*fb_dev).height < height1 || width > 132 || height > 60 {
        return;
    }
    if (*s).last_line_offset as u32 != line_offset
        || (*s).last_start_addr as u32 != start_addr
        || (*s).last_width != width as u32
        || (*s).last_height != height as u32
    {
        (*s).last_line_offset = line_offset as u16;
        (*s).last_start_addr = start_addr as u16;
        (*s).last_width = width as u32;
        (*s).last_height = height as u32;
        full_update = 1;
    }

    let mut cursor_offset: u32 =
        (((*cr.add(0x0e) as u32) << 8) | (*cr.add(0x0f) as u32)).wrapping_sub(start_addr);
    let cursor_start: u32 = *cr.add(0xa) as u32;
    let cursor_end: u32 = *cr.add(0xb) as u32;
    if cursor_offset != (*s).last_cursor_offset as u32
        || cursor_start != (*s).last_cursor_start as u32
        || cursor_end != (*s).last_cursor_end as u32
    {
        let lca = (*s).last_ch_attr.as_mut_ptr();
        let last_off = (*s).last_cursor_offset as u32;
        if last_off < 132 * 60 {
            *lca.add(last_off as usize) = 0xffffu16;
        }
        if cursor_offset < 132 * 60 {
            *lca.add(cursor_offset as usize) = 0xffffu16;
        }
        (*s).last_cursor_offset = cursor_offset as u16;
        (*s).last_cursor_start = cursor_start as u8;
        (*s).last_cursor_end = cursor_end as u8;
    }

    ch_addr1 = 0x18000u32.wrapping_add(start_addr.wrapping_mul(2));
    cursor_offset =
        0x18000u32.wrapping_add(start_addr.wrapping_add(cursor_offset).wrapping_mul(2));

    let x1: c_int = ((*fb_dev).width - width1) / 2;
    let y1: c_int = ((*fb_dev).height - height1) / 2;

    let stride: c_int = (*fb_dev).stride;
    let fb_data: *mut u8 = (*fb_dev).fb_data;

    let mut cy: c_int = 0;
    while cy < height {
        ch_addr = ch_addr1;
        let off: c_int = y1
            .wrapping_add(cy.wrapping_mul(cheight))
            .wrapping_mul(stride)
            .wrapping_add(x1.wrapping_mul(4));
        dst = fb_data.offset(off as isize);
        cx_min = width;
        cx_max = -1;
        let mut cx: c_int = 0;
        while cx < width {
            let idx: usize = cy.wrapping_mul(width).wrapping_add(cx) as usize;
            ch_attr = read_unaligned(
                vga_ram.offset((ch_addr & 0x1fffe) as isize) as *const u16
            ) as u32;
            {
                let lca = (*s).last_ch_attr.as_mut_ptr();
                if full_update != 0 || ch_attr != *lca.add(idx) as u32 {
                    *lca.add(idx) = ch_attr as u16;
                    cx_min = min_int(cx_min, cx);
                    cx_max = max_int(cx_max, cx);
                    ch = ch_attr & 0xff;
                    cattr = ch_attr >> 8;
                    font_ptr = vga_ram.offset(ch.wrapping_mul(32) as isize);
                    let lp = (*s).last_palette.as_ptr();
                    bgcol = *lp.add((cattr >> 4) as usize);
                    fgcol = *lp.add((cattr & 0x0f) as usize);
                    if cwidth == 8 {
                        vga_draw_glyph8(dst, stride, font_ptr, cheight, fgcol, bgcol);
                    } else {
                        dup9 = 0;
                        if ch >= 0xb0 && ch <= 0xdf && ((*s).ar[0x10] & 0x04) != 0 {
                            dup9 = 1;
                        }
                        vga_draw_glyph9(dst, stride, font_ptr, cheight, fgcol, bgcol, dup9);
                    }

                    if cursor_offset == ch_addr && (cursor_start & 0x20) == 0 {
                        let line_start: c_int = (cursor_start & 0x1f) as c_int;
                        let line_last: c_int =
                            min_int((cursor_end & 0x1f) as c_int, cheight - 1);

                        if line_last >= line_start && line_start < cheight {
                            let h: c_int = line_last - line_start + 1;
                            let dst1 = dst.offset(stride.wrapping_mul(line_start) as isize);
                            if cwidth == 8 {
                                vga_draw_glyph8(
                                    dst1,
                                    stride,
                                    CURSOR_GLYPH.as_ptr(),
                                    h,
                                    fgcol,
                                    bgcol,
                                );
                            } else {
                                vga_draw_glyph9(
                                    dst1,
                                    stride,
                                    CURSOR_GLYPH.as_ptr(),
                                    h,
                                    fgcol,
                                    bgcol,
                                    1,
                                );
                            }
                        }
                    }
                }
            }
            ch_addr = ch_addr.wrapping_add(2);
            dst = dst.offset((4 * cwidth) as isize);
            cx += 1;
        }
        if cx_max >= cx_min {
            if let Some(f) = redraw_func {
                f(
                    fb_dev,
                    opaque,
                    x1.wrapping_add(cx_min.wrapping_mul(cwidth)),
                    y1.wrapping_add(cy.wrapping_mul(cheight)),
                    (cx_max - cx_min + 1).wrapping_mul(cwidth),
                    cheight,
                );
            }
        }
        ch_addr1 = ch_addr1.wrapping_add(line_offset);
        cy += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vga_refresh(
    fb_dev: *mut FBDevice,
    redraw_func: RedrawFunc,
    opaque: *mut c_void,
) {
    let s = (*fb_dev).device_opaque as *mut VGAState;

    if ((*s).ar_index & 0x20) == 0 {
        /* blank */
    } else if ((*s).gr[0x06] & 1) != 0 {
        /* graphic mode (VBE) */
        simplefb_refresh(
            fb_dev,
            redraw_func,
            opaque,
            (*s).mem_range,
            (*s).fb_page_count,
        );
    } else {
        /* text mode */
        vga_text_refresh(s, redraw_func, opaque);
    }
}
}

// ── `if (!(s->ar_index & 0x20) … else` · vga.c:353-361 · chunk 3648 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->ar_index & 0x20)) {
//     
//         } else if (s->gr[0x06] & 1) {
//     
//             simplefb_refresh(fb_dev, redraw_func, opaque, s->mem_range, s->fb_page_count);
//         } else {
//     
//             vga_text_refresh(s, redraw_func, opaque);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3648 {
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::{c_int, c_void};

pub type SimpleFBDrawFunc = Option<
    unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int),
>;

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, SimpleFBDrawFunc, *mut c_void)>,
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

extern "C" {
    fn simplefb_refresh(
        fb_dev: *mut FBDevice,
        redraw_func: SimpleFBDrawFunc,
        opaque: *mut c_void,
        mem_range: *mut c_void,
        fb_page_count: c_int,
    );
}

static CURSOR_GLYPH: [u8; 32] = [0xff; 32];

#[inline]
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

unsafe fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    let mut full_update: c_int = 0;
    let ar = (*s).ar.as_ptr();
    let pal = (*s).palette.as_ptr();
    let mut i: usize = 0;
    while i < 16 {
        let mut v: u32 = *ar.add(i) as u32;
        if (*ar.add(0x10) & 0x80) != 0 {
            v = (((*ar.add(0x14) & 0xf) as u32) << 4) | (v & 0xf);
        } else {
            v = (((*ar.add(0x14) & 0xc) as u32) << 4) | (v & 0x3f);
        }
        v = v.wrapping_mul(3);
        let vi = v as usize;
        let col: u32 = ((c6_to_8(*pal.add(vi) as c_int) << 16) as u32)
            | ((c6_to_8(*pal.add(vi + 1) as c_int) << 8) as u32)
            | (c6_to_8(*pal.add(vi + 2) as c_int) as u32);
        if col != *palette.add(i) {
            full_update = 1;
            *palette.add(i) = col;
        }
        i += 1;
    }
    full_update
}

unsafe fn vga_draw_glyph8(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    mut h: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let mut d = d;
    let mut fp = font_ptr;
    let xorcol = bgcol ^ fgcol;
    loop {
        let font_data = *fp as u32;
        let out = d as *mut u32;
        core::ptr::write_unaligned(out.add(0), (((font_data >> 7).wrapping_neg()) & xorcol) ^ bgcol);
        core::ptr::write_unaligned(
            out.add(1),
            ((((font_data >> 6) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(2),
            ((((font_data >> 5) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(3),
            ((((font_data >> 4) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(4),
            ((((font_data >> 3) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(5),
            ((((font_data >> 2) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(6),
            ((((font_data >> 1) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(7),
            ((((font_data >> 0) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        fp = fp.add(1);
        d = d.offset(linesize as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

unsafe fn vga_draw_glyph9(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    mut h: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let mut d = d;
    let mut fp = font_ptr;
    let xorcol = bgcol ^ fgcol;
    loop {
        let font_data = *fp as u32;
        let out = d as *mut u32;
        core::ptr::write_unaligned(out.add(0), (((font_data >> 7).wrapping_neg()) & xorcol) ^ bgcol);
        core::ptr::write_unaligned(
            out.add(1),
            ((((font_data >> 6) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(2),
            ((((font_data >> 5) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(3),
            ((((font_data >> 4) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(4),
            ((((font_data >> 3) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(5),
            ((((font_data >> 2) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            out.add(6),
            ((((font_data >> 1) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        let v = ((((font_data >> 0) & 1).wrapping_neg()) & xorcol) ^ bgcol;
        core::ptr::write_unaligned(out.add(7), v);
        if dup9 != 0 {
            core::ptr::write_unaligned(out.add(8), v);
        } else {
            core::ptr::write_unaligned(out.add(8), bgcol);
        }
        fp = fp.add(1);
        d = d.offset(linesize as isize);
        h -= 1;
        if h == 0 {
            break;
        }
    }
}

unsafe fn vga_text_refresh(s: *mut VGAState, redraw_func: SimpleFBDrawFunc, opaque: *mut c_void) {
    let fb_dev = (*s).fb_dev;
    let mut full_update: c_int = update_palette16(s, (*s).last_palette.as_mut_ptr());

    let vga_ram = (*s).vga_ram;

    let mut line_offset: u32 = (*s).cr[0x13] as u32;
    line_offset <<= 3;
    line_offset >>= 1;

    let start_addr: u32 = ((*s).cr[0x0d] as u32) | (((*s).cr[0x0c] as u32) << 8);

    let cheight: c_int = (((*s).cr[9] & 0x1f) as c_int) + 1;
    let mut cwidth: c_int = 8;
    if ((*s).sr[1] & 0x01) == 0 {
        cwidth += 1;
    }

    let width: c_int = ((*s).cr[0x01] as c_int) + 1;
    let mut height: c_int = ((*s).cr[0x12] as c_int)
        | ((((*s).cr[0x07] & 0x02) as c_int) << 7)
        | ((((*s).cr[0x07] & 0x40) as c_int) << 3);
    height = (height + 1) / cheight;

    let width1: c_int = width * cwidth;
    let height1: c_int = height * cheight;
    if (*fb_dev).width < width1
        || (*fb_dev).height < height1
        || width > 132
        || height > 60
    {
        return;
    }
    if ((*s).last_line_offset as u32) != line_offset
        || ((*s).last_start_addr as u32) != start_addr
        || (*s).last_width != width as u32
        || (*s).last_height != height as u32
    {
        (*s).last_line_offset = line_offset as u16;
        (*s).last_start_addr = start_addr as u16;
        (*s).last_width = width as u32;
        (*s).last_height = height as u32;
        full_update = 1;
    }

    let mut cursor_offset: u32 = ((((*s).cr[0x0e] as u32) << 8) | ((*s).cr[0x0f] as u32))
        .wrapping_sub(start_addr);
    let cursor_start: u32 = (*s).cr[0xa] as u32;
    let cursor_end: u32 = (*s).cr[0xb] as u32;
    if ((*s).last_cursor_offset as u32) != cursor_offset
        || ((*s).last_cursor_start as u32) != cursor_start
        || ((*s).last_cursor_end as u32) != cursor_end
    {
        let lca = (*s).last_ch_attr.as_mut_ptr();
        if ((*s).last_cursor_offset as u32) < 132 * 60 {
            *lca.add((*s).last_cursor_offset as usize) = 0xffffu16;
        }
        if cursor_offset < 132 * 60 {
            *lca.add(cursor_offset as usize) = 0xffffu16;
        }
        (*s).last_cursor_offset = cursor_offset as u16;
        (*s).last_cursor_start = cursor_start as u8;
        (*s).last_cursor_end = cursor_end as u8;
    }

    let mut ch_addr1: u32 = 0x18000u32.wrapping_add(start_addr.wrapping_mul(2));
    cursor_offset = 0x18000u32.wrapping_add(start_addr.wrapping_add(cursor_offset).wrapping_mul(2));

    let x1: c_int = ((*fb_dev).width - width1) / 2;
    let y1: c_int = ((*fb_dev).height - height1) / 2;

    let mut cy: c_int = 0;
    while cy < height {
        let mut ch_addr: u32 = ch_addr1;
        let mut dst: *mut u8 = ((*fb_dev).fb_data)
            .offset(((y1 + cy * cheight) * (*fb_dev).stride + x1 * 4) as isize);
        let mut cx_min: c_int = width;
        let mut cx_max: c_int = -1;
        let mut cx: c_int = 0;
        while cx < width {
            let ch_attr: u32 = core::ptr::read_unaligned(
                vga_ram.offset((ch_addr & 0x1fffe) as isize) as *const u16
            ) as u32;
            let idx = (cy * width + cx) as usize;
            let lca = (*s).last_ch_attr.as_mut_ptr();
            if full_update != 0 || ch_attr != (*lca.add(idx)) as u32 {
                *lca.add(idx) = ch_attr as u16;
                if cx < cx_min {
                    cx_min = cx;
                }
                if cx > cx_max {
                    cx_max = cx;
                }
                let ch: u32 = ch_attr & 0xff;
                let cattr: u32 = ch_attr >> 8;
                let font_ptr = vga_ram.offset((32 * ch) as isize);
                let bgcol = *(*s).last_palette.as_ptr().add((cattr >> 4) as usize);
                let fgcol = *(*s).last_palette.as_ptr().add((cattr & 0x0f) as usize);
                if cwidth == 8 {
                    vga_draw_glyph8(dst, (*fb_dev).stride, font_ptr, cheight, fgcol, bgcol);
                } else {
                    let mut dup9: c_int = 0;
                    if ch >= 0xb0 && ch <= 0xdf && ((*s).ar[0x10] & 0x04) != 0 {
                        dup9 = 1;
                    }
                    vga_draw_glyph9(dst, (*fb_dev).stride, font_ptr, cheight, fgcol, bgcol, dup9);
                }

                if cursor_offset == ch_addr && (cursor_start & 0x20) == 0 {
                    let line_start: c_int = (cursor_start & 0x1f) as c_int;
                    let ce = (cursor_end & 0x1f) as c_int;
                    let cm = cheight - 1;
                    let line_last: c_int = if ce < cm { ce } else { cm };

                    if line_last >= line_start && line_start < cheight {
                        let h = line_last - line_start + 1;
                        let dst1 = dst.offset(((*fb_dev).stride * line_start) as isize);
                        if cwidth == 8 {
                            vga_draw_glyph8(
                                dst1,
                                (*fb_dev).stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                            );
                        } else {
                            vga_draw_glyph9(
                                dst1,
                                (*fb_dev).stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                                1,
                            );
                        }
                    }
                }
            }
            ch_addr = ch_addr.wrapping_add(2);
            dst = dst.offset((4 * cwidth) as isize);
            cx += 1;
        }
        if cx_max >= cx_min {
            if let Some(f) = redraw_func {
                f(
                    fb_dev,
                    opaque,
                    x1 + cx_min * cwidth,
                    y1 + cy * cheight,
                    (cx_max - cx_min + 1) * cwidth,
                    cheight,
                );
            }
        }
        ch_addr1 = ch_addr1.wrapping_add(line_offset);
        cy += 1;
    }
}

#[no_mangle]
pub extern "C" fn vga_refresh__part3648(
    fb_dev: *mut FBDevice,
    redraw_func: SimpleFBDrawFunc,
    opaque: *mut c_void,
    s: *mut VGAState,
) {
    unsafe {
        if ((*s).ar_index & 0x20) == 0 {
            // nothing to do
        } else if ((*s).gr[0x06] & 1) != 0 {
            simplefb_refresh(
                fb_dev,
                redraw_func,
                opaque,
                (*s).mem_range,
                (*s).fb_page_count,
            );
        } else {
            vga_text_refresh(s, redraw_func, opaque);
        }
    }
}
}

// ── `vga_ioport_read` · vga.c:395-486 · chunk 3651 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_ioport_read(VGAState *s, uint32_t addr)
//     {
//         int val, index;
//     
//     
//         if ((addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
//             (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01))) {
//             val = 0xff;
//         } else {
//             switch(addr) {
//             case 0x3c0:
//                 if (s->ar_flip_flop == 0) {
//                     val = s->ar_index;
//                 } else {
//                     val = 0;
//                 }
//                 break;
//             case 0x3c1:
//                 index = s->ar_index & 0x1f;
//                 if (index < 21)
//                     val = s->ar[index];
//                 else
//                     val = 0;
//                 break;
//             case 0x3c2:
//                 val = s->st00;
//                 break;
//             case 0x3c4:
//                 val = s->sr_index;
//                 break;
//             case 0x3c5:
//                 val = s->sr[s->sr_index];
//     
//     
//     
//                 break;
//             case 0x3c7:
//                 val = s->dac_state;
//                 break;
//      case 0x3c8:
//          val = s->dac_write_index;
//          break;
//             case 0x3c9:
//                 val = s->palette[s->dac_read_index * 3 + s->dac_sub_index];
//                 if (++s->dac_sub_index == 3) {
//                     s->dac_sub_index = 0;
//                     s->dac_read_index++;
//                 }
//                 break;
//             case 0x3ca:
//                 val = s->fcr;
//                 break;
//             case 0x3cc:
//                 val = s->msr;
//                 break;
//             case 0x3ce:
//                 val = s->gr_index;
//                 break;
//             case 0x3cf:
//                 val = s->gr[s->gr_index];
//     
//     
//     
//                 break;
//             case 0x3b4:
//             case 0x3d4:
//                 val = s->cr_index;
//                 break;
//             case 0x3b5:
//             case 0x3d5:
//                 val = s->cr[s->cr_index];
//     
//     
//     
//                 break;
//             case 0x3ba:
//             case 0x3da:
//     
//                 s->st01 ^= 0x08 | 0x01;
//                 val = s->st01;
//                 s->ar_flip_flop = 0;
//                 break;
//             default:
//                 val = 0x00;
//                 break;
//             }
//         }
//     
//     
//     
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3651 {
use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut core::ffi::c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut core::ffi::c_void,
    pub mem_range2: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub rom_range: *mut core::ffi::c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub unsafe extern "C" fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let s = &mut *s;
    let val: c_int;

    // check port range access depending on color/monochrome mode
    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if s.ar_flip_flop == 0 {
                    val = s.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (s.ar_index & 0x1f) as usize;
                if index < 21 {
                    val = s.ar[index] as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = s.st00 as c_int;
            }
            0x3c4 => {
                val = s.sr_index as c_int;
            }
            0x3c5 => {
                val = s.sr[s.sr_index as usize] as c_int;
            }
            0x3c7 => {
                val = s.dac_state as c_int;
            }
            0x3c8 => {
                val = s.dac_write_index as c_int;
            }
            0x3c9 => {
                let pidx = (s.dac_read_index as usize)
                    .wrapping_mul(3)
                    .wrapping_add(s.dac_sub_index as usize);
                val = if pidx < 768 { s.palette[pidx] as c_int } else { 0 };
                s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
                if s.dac_sub_index == 3 {
                    s.dac_sub_index = 0;
                    s.dac_read_index = s.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = s.fcr as c_int;
            }
            0x3cc => {
                val = s.msr as c_int;
            }
            0x3ce => {
                val = s.gr_index as c_int;
            }
            0x3cf => {
                val = s.gr[s.gr_index as usize] as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = s.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = s.cr[s.cr_index as usize] as c_int;
            }
            0x3ba | 0x3da => {
                // just toggle to fool polling
                s.st01 ^= 0x08 | 0x01;
                val = s.st01 as c_int;
                s.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }
    let _ = c_uint::default();
    val as u32
}
}

// ── `if (addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) || (addr >= 0x3d0 && addr <= 0x3df && !(… … else` · vga.c:400-481 · chunk 3652 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
//             (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01))) {
//             val = 0xff;
//         } else {
//             switch(addr) {
//             case 0x3c0:
//                 if (s->ar_flip_flop == 0) {
//                     val = s->ar_index;
//                 } else {
//                     val = 0;
//                 }
//                 break;
//             case 0x3c1:
//                 index = s->ar_index & 0x1f;
//                 if (index < 21)
//                     val = s->ar[index];
//                 else
//                     val = 0;
//                 break;
//             case 0x3c2:
//                 val = s->st00;
//                 break;
//             case 0x3c4:
//                 val = s->sr_index;
//                 break;
//             case 0x3c5:
//                 val = s->sr[s->sr_index];
//     
//     
//     
//                 break;
//             case 0x3c7:
//                 val = s->dac_state;
//                 break;
//      case 0x3c8:
//          val = s->dac_write_index;
//          break;
//             case 0x3c9:
//                 val = s->palette[s->dac_read_index * 3 + s->dac_sub_index];
//                 if (++s->dac_sub_index == 3) {
//                     s->dac_sub_index = 0;
//                     s->dac_read_index++;
//                 }
//                 break;
//             case 0x3ca:
//                 val = s->fcr;
//                 break;
//             case 0x3cc:
//                 val = s->msr;
//                 break;
//             case 0x3ce:
//                 val = s->gr_index;
//                 break;
//             case 0x3cf:
//                 val = s->gr[s->gr_index];
//     
//     
//     
//                 break;
//             case 0x3b4:
//             case 0x3d4:
//                 val = s->cr_index;
//                 break;
//             case 0x3b5:
//             case 0x3d5:
//                 val = s->cr[s->cr_index];
//     
//     
//     
//                 break;
//             case 0x3ba:
//             case 0x3da:
//     
//                 s->st01 ^= 0x08 | 0x01;
//                 val = s->st01;
//                 s->ar_flip_flop = 0;
//                 break;
//             default:
//                 val = 0x00;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3652 {
// Rust port of the read-side switch of vga_ioport_read.

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut core::ffi::c_void,
    pub fb_page_count: core::ffi::c_int,
    pub mem_range: *mut core::ffi::c_void,
    pub mem_range2: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub rom_range: *mut core::ffi::c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: core::ffi::c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub extern "C" fn vga_ioport_read__part3652(s: *mut VGAState, addr: u32) -> core::ffi::c_int {
    use core::ffi::c_int;
    unsafe {
        let val: c_int;
        let msr = (*s).msr;
        if (addr >= 0x3b0 && addr <= 0x3bf && (msr & 0x01) != 0)
            || (addr >= 0x3d0 && addr <= 0x3df && (msr & 0x01) == 0)
        {
            val = 0xff;
        } else {
            match addr {
                0x3c0 => {
                    val = if (*s).ar_flip_flop == 0 {
                        (*s).ar_index as c_int
                    } else {
                        0
                    };
                }
                0x3c1 => {
                    let index = ((*s).ar_index & 0x1f) as c_int;
                    val = if index < 21 {
                        let p = core::ptr::addr_of!((*s).ar) as *const u8;
                        *p.add(index as usize) as c_int
                    } else {
                        0
                    };
                }
                0x3c2 => val = (*s).st00 as c_int,
                0x3c4 => val = (*s).sr_index as c_int,
                0x3c5 => {
                    let p = core::ptr::addr_of!((*s).sr) as *const u8;
                    val = *p.add((*s).sr_index as usize) as c_int;
                }
                0x3c7 => val = (*s).dac_state as c_int,
                0x3c8 => val = (*s).dac_write_index as c_int,
                0x3c9 => {
                    let idx = ((*s).dac_read_index as usize) * 3 + (*s).dac_sub_index as usize;
                    let p = core::ptr::addr_of!((*s).palette) as *const u8;
                    val = *p.add(idx) as c_int;
                    let t = (*s).dac_sub_index.wrapping_add(1);
                    (*s).dac_sub_index = t;
                    if t == 3 {
                        (*s).dac_sub_index = 0;
                        (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
                    }
                }
                0x3ca => val = (*s).fcr as c_int,
                0x3cc => val = (*s).msr as c_int,
                0x3ce => val = (*s).gr_index as c_int,
                0x3cf => {
                    let p = core::ptr::addr_of!((*s).gr) as *const u8;
                    val = *p.add((*s).gr_index as usize) as c_int;
                }
                0x3b4 | 0x3d4 => val = (*s).cr_index as c_int,
                0x3b5 | 0x3d5 => {
                    let p = core::ptr::addr_of!((*s).cr) as *const u8;
                    val = *p.add((*s).cr_index as usize) as c_int;
                }
                0x3ba | 0x3da => {
                    (*s).st01 ^= 0x08 | 0x01;
                    val = (*s).st01 as c_int;
                    (*s).ar_flip_flop = 0;
                }
                _ => val = 0x00,
            }
        }
        val
    }
}
}

// ── `return val;` · vga.c:485-485 · chunk 3653 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3653 {
// Port of `return val;` from vga_ioport_read (vga.c:485).
// `val` is a C `int`; the enclosing function returns `uint32_t`,
// so the conversion is a plain unsigned modulo-2^32 reinterpretation.

use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn vga_ioport_read__part3653(val: c_int) -> c_uint {
    val as c_uint
}
}

// ── `vga_ioport_write` · vga.c:488-607 · chunk 3654 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_ioport_write(VGAState *s, uint32_t addr, uint32_t val)
//     {
//         int index;
//     
//     
//         if ((addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
//             (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01)))
//             return;
//     
//     
//     
//     
//     
//         switch(addr) {
//         case 0x3c0:
//             if (s->ar_flip_flop == 0) {
//                 val &= 0x3f;
//                 s->ar_index = val;
//             } else {
//                 index = s->ar_index & 0x1f;
//                 switch(index) {
//                 case 0x00 ... 0x0f:
//                     s->ar[index] = val & 0x3f;
//                     break;
//                 case 0x10:
//                     s->ar[index] = val & ~0x10;
//                     break;
//                 case 0x11:
//                     s->ar[index] = val;
//                     break;
//                 case 0x12:
//                     s->ar[index] = val & ~0xc0;
//                     break;
//                 case 0x13:
//                     s->ar[index] = val & ~0xf0;
//                     break;
//                 case 0x14:
//                     s->ar[index] = val & ~0xf0;
//                     break;
//                 default:
//                     break;
//                 }
//             }
//             s->ar_flip_flop ^= 1;
//             break;
//         case 0x3c2:
//             s->msr = val & ~0x10;
//             break;
//         case 0x3c4:
//             s->sr_index = val & 7;
//             break;
//         case 0x3c5:
//     
//     
//     
//             s->sr[s->sr_index] = val & sr_mask[s->sr_index];
//             break;
//         case 0x3c7:
//             s->dac_read_index = val;
//             s->dac_sub_index = 0;
//             s->dac_state = 3;
//             break;
//         case 0x3c8:
//             s->dac_write_index = val;
//             s->dac_sub_index = 0;
//             s->dac_state = 0;
//             break;
//         case 0x3c9:
//             s->dac_cache[s->dac_sub_index] = val;
//             if (++s->dac_sub_index == 3) {
//                 memcpy(&s->palette[s->dac_write_index * 3], s->dac_cache, 3);
//                 s->dac_sub_index = 0;
//                 s->dac_write_index++;
//             }
//             break;
//         case 0x3ce:
//             s->gr_index = val & 0x0f;
//             break;
//         case 0x3cf:
//     
//     
//     
//             s->gr[s->gr_index] = val & gr_mask[s->gr_index];
//             break;
//         case 0x3b4:
//         case 0x3d4:
//             s->cr_index = val;
//             break;
//         case 0x3b5:
//         case 0x3d5:
//     
//     
//     
//     
//             if ((s->cr[0x11] & 0x80) && s->cr_index <= 7) {
//     
//                 if (s->cr_index == 7)
//                     s->cr[7] = (s->cr[7] & ~0x10) | (val & 0x10);
//                 return;
//             }
//             switch(s->cr_index) {
//             case 0x01:
//             case 0x07:
//             case 0x09:
//             case 0x0c:
//             case 0x0d:
//             case 0x12:
//                 s->cr[s->cr_index] = val;
//                 break;
//             default:
//                 s->cr[s->cr_index] = val;
//                 break;
//             }
//             break;
//         case 0x3ba:
//         case 0x3da:
//             s->fcr = val & 0x10;
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3654 {
// Rust rewrite of vga_ioport_write (ported from vga.c)

use core::ffi::c_int;

#[repr(C)]
pub struct VgaState {
    fb_dev: *mut u8,
    fb_page_count: c_int,
    mem_range: *mut u8,
    mem_range2: *mut u8,
    pci_dev: *mut u8,
    rom_range: *mut u8,
    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8,
    !0xc2u8,
    !0xf0u8,
    !0xc0u8,
    !0xf1u8,
    !0xffu8,
    !0xffu8,
    !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8,
    !0xf0u8,
    !0xf0u8,
    !0xe0u8,
    !0xfcu8,
    !0x84u8,
    !0xf0u8,
    !0xf0u8,
    !0x00u8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
];

#[no_mangle]
pub unsafe extern "C" fn vga_ioport_write(s: *mut VgaState, addr: u32, val: u32) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;

    // check port range access depending on color/monochrome mode
    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                s.ar_index = (val & 0x3f) as u8;
            } else {
                let index = (s.ar_index & 0x1f) as usize;
                let p = s.ar.as_mut_ptr();
                match index {
                    0x00..=0x0f => *p.add(index) = (val & 0x3f) as u8,
                    0x10 => *p.add(index) = (val & !0x10u32) as u8,
                    0x11 => *p.add(index) = val as u8,
                    0x12 => *p.add(index) = (val & !0xc0u32) as u8,
                    0x13 => *p.add(index) = (val & !0xf0u32) as u8,
                    0x14 => *p.add(index) = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let index = s.sr_index as usize;
            let mask = *SR_MASK.get_unchecked(index) as u32;
            *s.sr.as_mut_ptr().add(index) = (val & mask) as u8;
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let index = s.dac_sub_index as usize;
            *s.dac_cache.as_mut_ptr().add(index) = val as u8;
            s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
            if s.dac_sub_index == 3 {
                let dst = s
                    .palette
                    .as_mut_ptr()
                    .add((s.dac_write_index as u32 * 3) as usize);
                let src = s.dac_cache.as_ptr();
                core::ptr::copy_nonoverlapping(src, dst, 3);
                s.dac_sub_index = 0;
                s.dac_write_index = s.dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let index = s.gr_index as usize;
            let mask = *GR_MASK.get_unchecked(index) as u32;
            *s.gr.as_mut_ptr().add(index) = (val & mask) as u8;
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            // handle CR0-7 protection
            if (s.cr[0x11] & 0x80) != 0 && s.cr_index <= 7 {
                // can always write bit 4 of CR7
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | (val as u8 & 0x10);
                }
                return;
            }
            let index = s.cr_index as usize;
            *s.cr.as_mut_ptr().add(index) = val as u8;
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}
}

// ── `if (addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) || (addr >= 0x3d0 && addr <= 0x3df && !(…` · vga.c:493-495 · chunk 3655 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
//             (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01)))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3655 {
use core::ffi::c_int;

// Ported piece of vga_ioport_write: the early-out check on the VGA
// address decode vs. the master status register bit 0.
// Returns nonzero when the write must be ignored.
#[no_mangle]
pub extern "C" fn vga_ioport_write__part3655(addr: u32, msr: u8) -> c_int {
    let ignore = (addr >= 0x3b0 && addr <= 0x3bf && (msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (msr & 0x01) == 0);
    ignore as c_int
}
}

// ── `switch (addr)` · vga.c:501-606 · chunk 3656 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(addr) {
//         case 0x3c0:
//             if (s->ar_flip_flop == 0) {
//                 val &= 0x3f;
//                 s->ar_index = val;
//             } else {
//                 index = s->ar_index & 0x1f;
//                 switch(index) {
//                 case 0x00 ... 0x0f:
//                     s->ar[index] = val & 0x3f;
//                     break;
//                 case 0x10:
//                     s->ar[index] = val & ~0x10;
//                     break;
//                 case 0x11:
//                     s->ar[index] = val;
//                     break;
//                 case 0x12:
//                     s->ar[index] = val & ~0xc0;
//                     break;
//                 case 0x13:
//                     s->ar[index] = val & ~0xf0;
//                     break;
//                 case 0x14:
//                     s->ar[index] = val & ~0xf0;
//                     break;
//                 default:
//                     break;
//                 }
//             }
//             s->ar_flip_flop ^= 1;
//             break;
//         case 0x3c2:
//             s->msr = val & ~0x10;
//             break;
//         case 0x3c4:
//             s->sr_index = val & 7;
//             break;
//         case 0x3c5:
//     
//     
//     
//             s->sr[s->sr_index] = val & sr_mask[s->sr_index];
//             break;
//         case 0x3c7:
//             s->dac_read_index = val;
//             s->dac_sub_index = 0;
//             s->dac_state = 3;
//             break;
//         case 0x3c8:
//             s->dac_write_index = val;
//             s->dac_sub_index = 0;
//             s->dac_state = 0;
//             break;
//         case 0x3c9:
//             s->dac_cache[s->dac_sub_index] = val;
//             if (++s->dac_sub_index == 3) {
//                 memcpy(&s->palette[s->dac_write_index * 3], s->dac_cache, 3);
//                 s->dac_sub_index = 0;
//                 s->dac_write_index++;
//             }
//             break;
//         case 0x3ce:
//             s->gr_index = val & 0x0f;
//             break;
//         case 0x3cf:
//     
//     
//     
//             s->gr[s->gr_index] = val & gr_mask[s->gr_index];
//             break;
//         case 0x3b4:
//         case 0x3d4:
//             s->cr_index = val;
//             break;
//         case 0x3b5:
//         case 0x3d5:
//     
//     
//     
//     
//             if ((s->cr[0x11] & 0x80) && s->cr_index <= 7) {
//     
//                 if (s->cr_index == 7)
//                     s->cr[7] = (s->cr[7] & ~0x10) | (val & 0x10);
//                 return;
//             }
//             switch(s->cr_index) {
//             case 0x01:
//             case 0x07:
//             case 0x09:
//             case 0x0c:
//             case 0x0d:
//             case 0x12:
//                 s->cr[s->cr_index] = val;
//                 break;
//             default:
//                 s->cr[s->cr_index] = val;
//                 break;
//             }
//             break;
//         case 0x3ba:
//         case 0x3da:
//             s->fcr = val & 0x10;
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3656 {
// Port of the ioport write switch (vga.c lines 501-606) from vga_ioport_write().

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

// Copies of the C file's static const mask tables:
//   sr_mask[i] == (uint8_t)~(expr below), gr_mask likewise.
static SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8,
    !0x00u8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

/// Returns 1 when the original C switch executed `return;` (i.e. the caller
/// must return from vga_ioport_write), 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn vga_ioport_write__part3656(
    s: *mut VGAState,
    addr: u32,
    val: u32,
) -> c_int {
    let s = &mut *s;
    let mut val = val;

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                val &= 0x3f;
                s.ar_index = val as u8;
            } else {
                let index = (s.ar_index & 0x1f) as usize;
                let slot = s.ar.as_mut_ptr().add(index);
                match index {
                    0x00..=0x0f => *slot = (val & 0x3f) as u8,
                    0x10 => *slot = (val & !0x10u32) as u8,
                    0x11 => *slot = val as u8,
                    0x12 => *slot = (val & !0xc0u32) as u8,
                    0x13 => *slot = (val & !0xf0u32) as u8,
                    0x14 => *slot = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let index = s.sr_index as usize;
            let mask = *SR_MASK.as_ptr().add(index);
            *s.sr.as_mut_ptr().add(index) = (val & (mask as u32)) as u8;
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let sub = s.dac_sub_index as usize;
            *s.dac_cache.as_mut_ptr().add(sub) = val as u8;
            s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
            if s.dac_sub_index == 3 {
                let dst = s
                    .palette
                    .as_mut_ptr()
                    .add((s.dac_write_index as usize) * 3);
                core::ptr::copy_nonoverlapping(s.dac_cache.as_ptr(), dst, 3);
                s.dac_sub_index = 0;
                s.dac_write_index = s.dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let index = s.gr_index as usize;
            let mask = *GR_MASK.as_ptr().add(index);
            *s.gr.as_mut_ptr().add(index) = (val & (mask as u32)) as u8;
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if (s.cr[0x11] & 0x80) != 0 && (s.cr_index as c_int) <= 7 {
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | ((val & 0x10) as u8);
                }
                return 1;
            }
            let index = s.cr_index as usize;
            // Both switch branches in C store the same thing.
            *s.cr.as_mut_ptr().add(index) = val as u8;
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }

    0
}
}

// ── `vga_read_0x3c0` · vga.c:619-619 · chunk 3657 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_read_0x3c0(void *opaque, uint32_t addr, int size_log2){ return vga_ioport_read(opaque, 0x3c0 + addr);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3657 {
// Port of vga.c: vga_read_0x3c0 (an I/O port read wrapper) together with the
// static helper vga_ioport_read, which has no linker symbol and is therefore
// re-implemented here as a private Rust function.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// Direct byte access into fixed C arrays, mirroring C's unchecked indexing
// (never panics, even for out-of-range indices coming from device state).
unsafe fn arr_get<const N: usize>(a: &[u8; N], idx: usize) -> u8 {
    *(a.as_ptr().add(idx))
}

// static uint32_t vga_ioport_read(VGAState *s, uint32_t addr)
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let s = &mut *s;
    let mut val: c_int;
    let index: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if s.ar_flip_flop == 0 {
                    val = s.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                index = (s.ar_index & 0x1f) as c_int;
                if index < 21 {
                    val = arr_get(&s.ar, index as usize) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = s.st00 as c_int;
            }
            0x3c4 => {
                val = s.sr_index as c_int;
            }
            0x3c5 => {
                val = arr_get(&s.sr, s.sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = s.dac_state as c_int;
            }
            0x3c8 => {
                val = s.dac_write_index as c_int;
            }
            0x3c9 => {
                let pidx = (s.dac_read_index as usize) * 3 + (s.dac_sub_index as usize);
                val = arr_get(&s.palette, pidx) as c_int;
                s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
                if s.dac_sub_index == 3 {
                    s.dac_sub_index = 0;
                    s.dac_read_index = s.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = s.fcr as c_int;
            }
            0x3cc => {
                val = s.msr as c_int;
            }
            0x3ce => {
                val = s.gr_index as c_int;
            }
            0x3cf => {
                val = arr_get(&s.gr, s.gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = s.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = arr_get(&s.cr, s.cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                s.st01 ^= 0x08 | 0x01;
                val = s.st01 as c_int;
                s.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub unsafe extern "C" fn vga_read_0x3c0(
    opaque: *mut c_void,
    addr: u32,
    _size_log2: c_int,
) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3c0u32.wrapping_add(addr))
}
}

// ── `return vga_ioport_read(opaque, 0x3c0 + addr);` · vga.c:619-619 · chunk 3658 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_read(opaque, 0x3c0 + addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3658 {
// Port of the TARGET `return vga_ioport_read(opaque, 0x3c0 + addr);`
// plus the logic of the file-static C helper `vga_ioport_read`.
use core::ffi::{c_int, c_void};

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// Mirrors the static C function vga_ioport_read(VGAState *s, uint32_t addr).
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let msr_color = ((*s).msr & 0x01) != 0;
    if (addr >= 0x3b0 && addr <= 0x3bf && msr_color)
        || (addr >= 0x3d0 && addr <= 0x3df && !msr_color)
    {
        return 0xff;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index as u32
            } else {
                0
            }
        }
        0x3c1 => {
            let index = ((*s).ar_index & 0x1f) as usize;
            if index < 21 {
                *(*s).ar.as_ptr().add(index) as u32
            } else {
                0
            }
        }
        0x3c2 => (*s).st00 as u32,
        0x3c4 => (*s).sr_index as u32,
        0x3c5 => {
            let index = (*s).sr_index as usize;
            *(*s).sr.as_ptr().add(index) as u32
        }
        0x3c7 => (*s).dac_state as u32,
        0x3c8 => (*s).dac_write_index as u32,
        0x3c9 => {
            let index = ((*s).dac_read_index as usize) * 3 + ((*s).dac_sub_index as usize);
            let val = *(*s).palette.as_ptr().add(index) as u32;
            let next = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = next;
            if next == 3 {
                (*s).dac_sub_index = 0;
                (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
            }
            val
        }
        0x3ca => (*s).fcr as u32,
        0x3cc => (*s).msr as u32,
        0x3ce => (*s).gr_index as u32,
        0x3cf => {
            let index = (*s).gr_index as usize;
            *(*s).gr.as_ptr().add(index) as u32
        }
        0x3b4 | 0x3d4 => (*s).cr_index as u32,
        0x3b5 | 0x3d5 => {
            let index = (*s).cr_index as usize;
            *(*s).cr.as_ptr().add(index) as u32
        }
        0x3ba | 0x3da => {
            (*s).st01 ^= 0x08 | 0x01;
            let val = (*s).st01 as u32;
            (*s).ar_flip_flop = 0;
            val
        }
        _ => 0x00,
    }
}

#[no_mangle]
pub extern "C" fn vga_read_0x3c0__part3658(opaque: *mut c_void, addr: u32) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, addr.wrapping_add(0x3c0)) }
}
}

// ── `vga_write_0x3c0` · vga.c:619-619 · chunk 3659 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_write_0x3c0(void *opaque, uint32_t addr, uint32_t val, int size_log2){ return vga_ioport_write(opaque, 0x3c0 + addr, val);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3659 {
// Port of the VGA 0x3c0 ioport write wrapper (plus the file-static helper it calls).

use core::ffi::{c_int, c_void};

/// Mirror of the C `struct VGAState`, field-for-field, in order, with the same
/// types. Trailing fields that the ported code never touches are omitted.
#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8,
    !0xc2u8,
    !0xf0u8,
    !0xc0u8,
    !0xf1u8,
    !0xffu8,
    !0xffu8,
    !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8,
    !0xf0u8,
    !0xf0u8,
    !0xe0u8,
    !0xfcu8,
    !0x84u8,
    !0xf0u8,
    !0xf0u8,
    !0x00u8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
];

// Port of the C file-static helper `vga_ioport_write`.
unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    let msr_set = ((*s).msr & 0x01) != 0;
    if (addr >= 0x3b0 && addr <= 0x3bf && msr_set)
        || (addr >= 0x3d0 && addr <= 0x3df && !msr_set)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index = (val & 0x3f) as u8;
            } else {
                let index = ((*s).ar_index & 0x1f) as usize;
                match index {
                    0x00..=0x0f => *(*s).ar.as_mut_ptr().add(index) = (val & 0x3f) as u8,
                    0x10 => *(*s).ar.as_mut_ptr().add(index) = (val & !0x10u32) as u8,
                    0x11 => *(*s).ar.as_mut_ptr().add(index) = val as u8,
                    0x12 => *(*s).ar.as_mut_ptr().add(index) = (val & !0xc0u32) as u8,
                    0x13 => *(*s).ar.as_mut_ptr().add(index) = (val & !0xf0u32) as u8,
                    0x14 => *(*s).ar.as_mut_ptr().add(index) = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let idx = ((*s).sr_index & 7) as usize;
            *(*s).sr.as_mut_ptr().add(idx) = (val as u8) & *SR_MASK.as_ptr().add(idx);
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            *(*s).dac_cache.as_mut_ptr().add(sub) = val as u8;
            let next = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = next;
            if next == 3 {
                let off = ((*s).dac_write_index as usize).wrapping_mul(3);
                let dst = (*s).palette.as_mut_ptr().add(off);
                *dst = *(*s).dac_cache.as_ptr().add(0);
                *dst.add(1) = *(*s).dac_cache.as_ptr().add(1);
                *dst.add(2) = *(*s).dac_cache.as_ptr().add(2);
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let idx = ((*s).gr_index & 0x0f) as usize;
            *(*s).gr.as_mut_ptr().add(idx) = (val as u8) & *GR_MASK.as_ptr().add(idx);
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            let cr17 = *(*s).cr.as_ptr().add(0x11);
            if (cr17 & 0x80) != 0 && (*s).cr_index <= 7 {
                if (*s).cr_index == 7 {
                    let cr7 = *(*s).cr.as_ptr().add(7);
                    *(*s).cr.as_mut_ptr().add(7) = (cr7 & !0x10u8) | ((val & 0x10) as u8);
                }
                return;
            }
            let idx = (*s).cr_index as usize;
            *(*s).cr.as_mut_ptr().add(idx) = val as u8;
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3c0(opaque: *mut c_void, addr: u32, val: u32, _size_log2: c_int) {
    unsafe {
        vga_ioport_write(
            opaque as *mut VGAState,
            0x3c0u32.wrapping_add(addr),
            val,
        );
    }
}
}

// ── `return vga_ioport_write(opaque, 0x3c0 + addr, val);` · vga.c:619-619 · chunk 3660 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_write(opaque, 0x3c0 + addr, val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3660 {
// Port of the TARGET line inside vga_write_0x3c0:
//   return vga_ioport_write(opaque, 0x3c0 + addr, val);
// vga_ioport_write is `static` in the C program, so its body is ported here as a
// private Rust function. The static const masks sr_mask/gr_mask have no linker
// symbols either, so their values are mirrored below.
use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// static const uint8_t sr_mask[8]
const SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

// static const uint8_t gr_mask[16]
const GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8,
    !0x00u8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

// Ported body of: static void vga_ioport_write(VGAState *s, uint32_t addr, uint32_t val)
unsafe fn vga_ioport_write_impl(s: *mut VGAState, addr: u32, val: u32) {
    if ((addr >= 0x3b0 && addr <= 0x3bf) && ((*s).msr & 0x01) != 0)
        || ((addr >= 0x3d0 && addr <= 0x3df) && ((*s).msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index = (val & 0x3f) as u8;
            } else {
                let index = ((*s).ar_index & 0x1f) as usize;
                let arp = (*s).ar.as_mut_ptr();
                match index {
                    0x00..=0x0f => *arp.add(index) = (val & 0x3f) as u8,
                    0x10 => *arp.add(index) = (val & !0x10u32) as u8,
                    0x11 => *arp.add(index) = val as u8,
                    0x12 => *arp.add(index) = (val & !0xc0u32) as u8,
                    0x13 => *arp.add(index) = (val & !0xf0u32) as u8,
                    0x14 => *arp.add(index) = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let index = (*s).sr_index as usize;
            let mask = *SR_MASK.as_ptr().add(index);
            *(*s).sr.as_mut_ptr().add(index) = (val & (mask as u32)) as u8;
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            *(*s).dac_cache.as_mut_ptr().add(sub) = val as u8;
            let next = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = next;
            if next == 3 {
                let off = ((*s).dac_write_index as usize).wrapping_mul(3);
                let dst = (*s).palette.as_mut_ptr().add(off);
                core::ptr::copy_nonoverlapping((*s).dac_cache.as_ptr(), dst, 3);
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let index = (*s).gr_index as usize;
            let mask = *GR_MASK.as_ptr().add(index);
            *(*s).gr.as_mut_ptr().add(index) = (val & (mask as u32)) as u8;
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if ((*s).cr[0x11] & 0x80) != 0 && (*s).cr_index <= 7 {
                if (*s).cr_index == 7 {
                    (*s).cr[7] = ((*s).cr[7] & !0x10u8) | ((val as u8) & 0x10);
                }
                return;
            }
            let index = (*s).cr_index as usize;
            *(*s).cr.as_mut_ptr().add(index) = val as u8;
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3c0__part3660(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    size_log2: c_int,
) {
    let _ = size_log2;
    unsafe {
        // uint32_t arithmetic wraps on overflow, exactly like the C `0x3c0 + addr`.
        vga_ioport_write_impl(opaque as *mut VGAState, 0x3c0u32.wrapping_add(addr), val);
    }
}
}

// ── `vga_read_0x3b4` · vga.c:620-620 · chunk 3661 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_read_0x3b4(void *opaque, uint32_t addr, int size_log2){ return vga_ioport_read(opaque, 0x3b4 + addr);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3661 {
// Rust rewrite of vga_read_0x3b4 (from vga.c, VGA_IO(0x3b4)).
// The C helper vga_ioport_read is `static`, so it has no linker symbol:
// its logic is ported here as a private Rust function.

use core::ffi::{c_int, c_void};

// Prefix of the C `struct VGAState` (fields accessed by vga_ioport_read),
// in exact declaration order so the offsets match the C layout.
#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
}

#[inline]
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let s = &mut *s;
    let val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if s.ar_flip_flop == 0 {
                    val = s.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (s.ar_index & 0x1f) as usize;
                if index < 21 {
                    val = s.ar[index] as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = s.st00 as c_int;
            }
            0x3c4 => {
                val = s.sr_index as c_int;
            }
            0x3c5 => {
                let p = core::ptr::addr_of!(s.sr) as *const u8;
                val = *p.add(s.sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = s.dac_state as c_int;
            }
            0x3c8 => {
                val = s.dac_write_index as c_int;
            }
            0x3c9 => {
                let p = core::ptr::addr_of!(s.palette) as *const u8;
                let idx = s.dac_read_index as usize * 3 + s.dac_sub_index as usize;
                val = *p.add(idx) as c_int;
                s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
                if s.dac_sub_index == 3 {
                    s.dac_sub_index = 0;
                    s.dac_read_index = s.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = s.fcr as c_int;
            }
            0x3cc => {
                val = s.msr as c_int;
            }
            0x3ce => {
                val = s.gr_index as c_int;
            }
            0x3cf => {
                let p = core::ptr::addr_of!(s.gr) as *const u8;
                val = *p.add(s.gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = s.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                let p = core::ptr::addr_of!(s.cr) as *const u8;
                val = *p.add(s.cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                s.st01 ^= 0x08 | 0x01;
                val = s.st01 as c_int;
                s.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3b4(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3b4u32.wrapping_add(addr)) }
}
}

// ── `return vga_ioport_read(opaque, 0x3b4 + addr);` · vga.c:620-620 · chunk 3662 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_read(opaque, 0x3b4 + addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3662 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// Private port of the static C helper vga_ioport_read (it has no linker symbol).
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let st = &mut *s;
    let val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && (st.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (st.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if st.ar_flip_flop == 0 {
                    val = st.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (st.ar_index & 0x1f) as c_int;
                if index < 21 {
                    val = *st.ar.as_ptr().add(index as usize) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = st.st00 as c_int;
            }
            0x3c4 => {
                val = st.sr_index as c_int;
            }
            0x3c5 => {
                val = *st.sr.as_ptr().add(st.sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = st.dac_state as c_int;
            }
            0x3c8 => {
                val = st.dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = st.dac_read_index as usize * 3 + st.dac_sub_index as usize;
                val = *st.palette.as_ptr().add(idx) as c_int;
                st.dac_sub_index = st.dac_sub_index.wrapping_add(1);
                if st.dac_sub_index == 3 {
                    st.dac_sub_index = 0;
                    st.dac_read_index = st.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = st.fcr as c_int;
            }
            0x3cc => {
                val = st.msr as c_int;
            }
            0x3ce => {
                val = st.gr_index as c_int;
            }
            0x3cf => {
                val = *st.gr.as_ptr().add(st.gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = st.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = *st.cr.as_ptr().add(st.cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                st.st01 ^= 0x08 | 0x01;
                val = st.st01 as c_int;
                st.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3b4__part3662(opaque: *mut c_void, addr: u32) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3b4u32.wrapping_add(addr)) }
}
}

// ── `vga_write_0x3b4` · vga.c:620-620 · chunk 3663 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_write_0x3b4(void *opaque, uint32_t addr, uint32_t val, int size_log2){ return vga_ioport_write(opaque, 0x3b4 + addr, val);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3663 {
// Port of vga_write_0x3b4 and the static helper vga_ioport_write it calls.
#![allow(non_snake_case)]
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,          // FBDevice *
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,       // PhysMemoryRange *
    pub mem_range2: *mut c_void,      // PhysMemoryRange *
    pub pci_dev: *mut c_void,         // PCIDevice *
    pub rom_range: *mut c_void,       // PhysMemoryRange *
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8,
    !0x00u8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    if s.is_null() {
        return;
    }
    let s = &mut *s;
    let mut val = val;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                val &= 0x3f;
                s.ar_index = val as u8;
            } else {
                let index = (s.ar_index & 0x1f) as usize;
                if index < s.ar.len() {
                    match index {
                        0x00..=0x0f => {
                            s.ar[index] = (val & 0x3f) as u8;
                        }
                        0x10 => {
                            s.ar[index] = (val & !0x10u32) as u8;
                        }
                        0x11 => {
                            s.ar[index] = val as u8;
                        }
                        0x12 => {
                            s.ar[index] = (val & !0xc0u32) as u8;
                        }
                        0x13 => {
                            s.ar[index] = (val & !0xf0u32) as u8;
                        }
                        0x14 => {
                            s.ar[index] = (val & !0xf0u32) as u8;
                        }
                        _ => {}
                    }
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let index = s.sr_index as usize;
            if index < 8 {
                s.sr[index] = (val & SR_MASK[index] as u32) as u8;
            }
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let sub = s.dac_sub_index as usize;
            if sub < 3 {
                s.dac_cache[sub] = val as u8;
                let next = s.dac_sub_index.wrapping_add(1);
                s.dac_sub_index = next;
                if next == 3 {
                    let base = (s.dac_write_index as usize).wrapping_mul(3);
                    if base + 3 <= s.palette.len() {
                        core::ptr::copy_nonoverlapping(
                            s.dac_cache.as_ptr(),
                            s.palette.as_mut_ptr().add(base),
                            3,
                        );
                    }
                    s.dac_sub_index = 0;
                    s.dac_write_index = s.dac_write_index.wrapping_add(1);
                }
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let index = s.gr_index as usize;
            if index < 16 {
                s.gr[index] = (val & GR_MASK[index] as u32) as u8;
            }
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if (s.cr[0x11] & 0x80) != 0 && s.cr_index <= 7 {
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | (val as u8 & 0x10);
                }
                return;
            }
            s.cr[s.cr_index as usize] = val as u8;
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3b4(opaque: *mut c_void, addr: u32, val: u32, size_log2: c_int) {
    let _ = size_log2;
    unsafe {
        vga_ioport_write(opaque as *mut VGAState, 0x3b4u32.wrapping_add(addr), val);
    }
}
}

// ── `return vga_ioport_write(opaque, 0x3b4 + addr, val);` · vga.c:620-620 · chunk 3664 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_write(opaque, 0x3b4 + addr, val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3664 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8,
    !0xc2u8,
    !0xf0u8,
    !0xc0u8,
    !0xf1u8,
    !0xffu8,
    !0xffu8,
    !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8,
    !0xf0u8,
    !0xf0u8,
    !0xe0u8,
    !0xfcu8,
    !0x84u8,
    !0xf0u8,
    !0xf0u8,
    !0x00u8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
    !0xffu8,
];

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn vga_write_0x3b4__part3664(s: *mut VGAState, addr: u32, val: u32) {
    let s = &mut *s;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                let v = val & 0x3f;
                s.ar_index = v as u8;
            } else {
                let index = (s.ar_index & 0x1f) as u32;
                match index {
                    0x00..=0x0f => {
                        s.ar[index as usize] = (val & 0x3f) as u8;
                    }
                    0x10 => {
                        s.ar[index as usize] = (val & !0x10u32) as u8;
                    }
                    0x11 => {
                        s.ar[index as usize] = val as u8;
                    }
                    0x12 => {
                        s.ar[index as usize] = (val & !0xc0u32) as u8;
                    }
                    0x13 => {
                        s.ar[index as usize] = (val & !0xf0u32) as u8;
                    }
                    0x14 => {
                        s.ar[index as usize] = (val & !0xf0u32) as u8;
                    }
                    _ => {}
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let i = s.sr_index as usize;
            s.sr[i] = (val & SR_MASK[i] as u32) as u8;
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let sub = s.dac_sub_index as usize;
            s.dac_cache[sub] = val as u8;
            let n = s.dac_sub_index.wrapping_add(1);
            s.dac_sub_index = n;
            if n == 3 {
                let off = (s.dac_write_index as usize).wrapping_mul(3);
                memcpy(
                    s.palette.as_mut_ptr().add(off) as *mut c_void,
                    s.dac_cache.as_ptr() as *const c_void,
                    3,
                );
                s.dac_sub_index = 0;
                s.dac_write_index = s.dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let i = s.gr_index as usize;
            s.gr[i] = (val & GR_MASK[i] as u32) as u8;
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if (s.cr[0x11] & 0x80) != 0 && s.cr_index <= 7 {
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | (val as u8 & 0x10);
                }
                return;
            }
            let i = s.cr_index as usize;
            s.cr[i] = val as u8;
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}
}

// ── `vga_read_0x3d4` · vga.c:621-621 · chunk 3665 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_read_0x3d4(void *opaque, uint32_t addr, int size_log2){ return vga_ioport_read(opaque, 0x3d4 + addr);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3665 {
// Port of the `VGA_IO(0x3d4)` generated read helper from vga.c:
//     static uint32_t vga_read_0x3d4(void *opaque, uint32_t addr, int size_log2)
//     { return vga_ioport_read(opaque, 0x3d4 + addr); }
// `vga_ioport_read` is a static (local) C function, so its logic is mirrored here.

use core::ffi::{c_int, c_void};

// Mirror of `struct VGAState` (vga.c). Only the fields touched by
// vga_ioport_read() are ever accessed, but the full layout is reproduced.
#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// Mirror of the static C function `vga_ioport_read`.
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let st = &mut *s;
    let val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && (st.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (st.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if st.ar_flip_flop == 0 {
                    val = st.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (st.ar_index & 0x1f) as usize;
                if index < 21 {
                    val = *st.ar.as_ptr().add(index) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => val = st.st00 as c_int,
            0x3c4 => val = st.sr_index as c_int,
            0x3c5 => val = *st.sr.as_ptr().add(st.sr_index as usize) as c_int,
            0x3c7 => val = st.dac_state as c_int,
            0x3c8 => val = st.dac_write_index as c_int,
            0x3c9 => {
                let idx = (st.dac_read_index as usize) * 3 + st.dac_sub_index as usize;
                val = *st.palette.as_ptr().add(idx) as c_int;
                st.dac_sub_index = st.dac_sub_index.wrapping_add(1);
                if st.dac_sub_index == 3 {
                    st.dac_sub_index = 0;
                    st.dac_read_index = st.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => val = st.fcr as c_int,
            0x3cc => val = st.msr as c_int,
            0x3ce => val = st.gr_index as c_int,
            0x3cf => val = *st.gr.as_ptr().add(st.gr_index as usize) as c_int,
            0x3b4 | 0x3d4 => val = st.cr_index as c_int,
            0x3b5 | 0x3d5 => val = *st.cr.as_ptr().add(st.cr_index as usize) as c_int,
            0x3ba | 0x3da => {
                st.st01 ^= 0x08 | 0x01;
                val = st.st01 as c_int;
                st.ar_flip_flop = 0;
            }
            _ => val = 0x00,
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3d4(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3d4u32.wrapping_add(addr)) }
}
}

// ── `return vga_ioport_read(opaque, 0x3d4 + addr);` · vga.c:621-621 · chunk 3666 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_read(opaque, 0x3d4 + addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3666 {
// Port of the static C helper `vga_ioport_read` plus the exported wrapper for
// the `vga_read_0x3d4` read handler body.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

/// Private re-implementation of the C `static uint32_t vga_ioport_read(VGAState *, uint32_t)`.
#[allow(dead_code)]
unsafe fn vga_ioport_read_impl(s: *mut VGAState, addr: u32) -> u32 {
    let mut val: c_int;
    let index: c_int;

    let msr = (*s).msr;
    if (addr >= 0x3b0 && addr <= 0x3bf && (msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if (*s).ar_flip_flop == 0 {
                    val = (*s).ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                index = ((*s).ar_index & 0x1f) as c_int;
                if index < 21 {
                    val = *(*s).ar.as_ptr().add(index as usize) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = (*s).st00 as c_int;
            }
            0x3c4 => {
                val = (*s).sr_index as c_int;
            }
            0x3c5 => {
                let idx = (*s).sr_index as usize;
                val = *(*s).sr.as_ptr().add(idx) as c_int;
            }
            0x3c7 => {
                val = (*s).dac_state as c_int;
            }
            0x3c8 => {
                val = (*s).dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = (*s).dac_read_index as usize * 3 + (*s).dac_sub_index as usize;
                val = *(*s).palette.as_ptr().add(idx) as c_int;
                let sub = (*s).dac_sub_index.wrapping_add(1);
                (*s).dac_sub_index = sub;
                if sub == 3 {
                    (*s).dac_sub_index = 0;
                    (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = (*s).fcr as c_int;
            }
            0x3cc => {
                val = (*s).msr as c_int;
            }
            0x3ce => {
                val = (*s).gr_index as c_int;
            }
            0x3cf => {
                let idx = (*s).gr_index as usize;
                val = *(*s).gr.as_ptr().add(idx) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = (*s).cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                let idx = (*s).cr_index as usize;
                val = *(*s).cr.as_ptr().add(idx) as c_int;
            }
            0x3ba | 0x3da => {
                (*s).st01 ^= 0x08 | 0x01;
                val = (*s).st01 as c_int;
                (*s).ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3d4__part3666(opaque: *mut c_void, addr: u32) -> u32 {
    unsafe { vga_ioport_read_impl(opaque as *mut VGAState, 0x3d4u32.wrapping_add(addr)) }
}
}

// ── `vga_write_0x3d4` · vga.c:621-621 · chunk 3667 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_write_0x3d4(void *opaque, uint32_t addr, uint32_t val, int size_log2){ return vga_ioport_write(opaque, 0x3d4 + addr, val);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3667 {
// Rust port of vga.c's VGA_IO(0x3d4) write wrapper.
// The C helper vga_ioport_write() is `static` (no linker symbol), so its
// logic is reimplemented here as a private Rust function.

use core::ffi::{c_int, c_void};

// Mirror of the leading part of struct VGAState that this code touches.
// Pointer fields are kept opaque: they are never dereferenced here.
#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8,
    !0x00u8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    if s.is_null() {
        return;
    }
    let s: &mut VGAState = &mut *s;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                let v = val & 0x3f;
                s.ar_index = v as u8;
            } else {
                let index = (s.ar_index & 0x1f) as u32;
                match index {
                    0x00..=0x0f => {
                        if let Some(slot) = s.ar.get_mut(index as usize) {
                            *slot = (val & 0x3f) as u8;
                        }
                    }
                    0x10 => {
                        let v = val & !0x10u32;
                        if let Some(slot) = s.ar.get_mut(index as usize) {
                            *slot = v as u8;
                        }
                    }
                    0x11 => {
                        if let Some(slot) = s.ar.get_mut(index as usize) {
                            *slot = val as u8;
                        }
                    }
                    0x12 => {
                        let v = val & !0xc0u32;
                        if let Some(slot) = s.ar.get_mut(index as usize) {
                            *slot = v as u8;
                        }
                    }
                    0x13 | 0x14 => {
                        let v = val & !0xf0u32;
                        if let Some(slot) = s.ar.get_mut(index as usize) {
                            *slot = v as u8;
                        }
                    }
                    _ => {}
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let i = s.sr_index as usize;
            let mask = SR_MASK[i & 7];
            if let Some(slot) = s.sr.get_mut(i) {
                *slot = (val as u8) & mask;
            }
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let i = s.dac_sub_index as usize;
            if let Some(slot) = s.dac_cache.get_mut(i) {
                *slot = val as u8;
            }
            s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
            if s.dac_sub_index == 3 {
                let base = (s.dac_write_index as usize).wrapping_mul(3);
                let mut k = 0usize;
                while k < 3 {
                    if let Some(src) = s.dac_cache.get(k).copied() {
                        if let Some(dst) = s.palette.get_mut(base.wrapping_add(k)) {
                            *dst = src;
                        }
                    }
                    k += 1;
                }
                s.dac_sub_index = 0;
                s.dac_write_index = s.dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let i = s.gr_index as usize;
            let mask = GR_MASK[i & 15];
            if let Some(slot) = s.gr.get_mut(i) {
                *slot = (val as u8) & mask;
            }
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if (s.cr[0x11] & 0x80) != 0 && s.cr_index <= 7 {
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | ((val as u8) & 0x10);
                }
                return;
            }
            let i = s.cr_index as usize;
            if let Some(slot) = s.cr.get_mut(i) {
                *slot = val as u8;
            }
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3d4(opaque: *mut c_void, addr: u32, val: u32, _size_log2: c_int) {
    unsafe {
        vga_ioport_write(opaque as *mut VGAState, 0x3d4u32.wrapping_add(addr), val);
    }
}
}

// ── `return vga_ioport_write(opaque, 0x3d4 + addr, val);` · vga.c:621-621 · chunk 3668 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_write(opaque, 0x3d4 + addr, val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3668 {
// Port of the `return vga_ioport_write(opaque, 0x3d4 + addr, val);` statement
// of vga_write_0x3d4.  vga_ioport_write is a C static function (no linker
// symbol), so its logic is ported in here as a private Rust function, together
// with the file-static sr_mask / gr_mask tables it uses.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VgaState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8, !0x00u8, !0xffu8,
    !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

fn vga_ioport_write_impl(s: &mut VgaState, addr: u32, val: u32) {
    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if s.ar_flip_flop == 0 {
                s.ar_index = (val & 0x3f) as u8;
            } else {
                let index = (s.ar_index & 0x1f) as usize;
                match index {
                    0x00..=0x0f => s.ar[index] = (val & 0x3f) as u8,
                    0x10 => s.ar[index] = (val & !0x10u32) as u8,
                    0x11 => s.ar[index] = val as u8,
                    0x12 => s.ar[index] = (val & !0xc0u32) as u8,
                    0x13 => s.ar[index] = (val & !0xf0u32) as u8,
                    0x14 => s.ar[index] = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            s.ar_flip_flop ^= 1;
        }
        0x3c2 => {
            s.msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            s.sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let index = (s.sr_index & 7) as usize;
            s.sr[index] = (val as u8) & SR_MASK[index];
        }
        0x3c7 => {
            s.dac_read_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 3;
        }
        0x3c8 => {
            s.dac_write_index = val as u8;
            s.dac_sub_index = 0;
            s.dac_state = 0;
        }
        0x3c9 => {
            let sub = (s.dac_sub_index % 3) as usize;
            s.dac_cache[sub] = val as u8;
            s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
            if s.dac_sub_index == 3 {
                let base = s.dac_write_index as usize * 3;
                s.palette[base] = s.dac_cache[0];
                s.palette[base + 1] = s.dac_cache[1];
                s.palette[base + 2] = s.dac_cache[2];
                s.dac_sub_index = 0;
                s.dac_write_index = s.dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            s.gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let index = (s.gr_index & 0x0f) as usize;
            s.gr[index] = (val as u8) & GR_MASK[index];
        }
        0x3b4 | 0x3d4 => {
            s.cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if (s.cr[0x11] & 0x80) != 0 && s.cr_index <= 7 {
                if s.cr_index == 7 {
                    s.cr[7] = (s.cr[7] & !0x10u8) | ((val as u8) & 0x10);
                }
                return;
            }
            let index = s.cr_index as usize;
            s.cr[index] = val as u8;
        }
        0x3ba | 0x3da => {
            s.fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

fn vga_ioport_write(s: *mut VgaState, addr: u32, val: u32) {
    if s.is_null() {
        return;
    }
    let st: &mut VgaState = unsafe { &mut *s };
    vga_ioport_write_impl(st, addr, val);
}

#[no_mangle]
pub extern "C" fn vga_write_0x3d4__part3668(opaque: *mut c_void, addr: u32, val: u32) {
    vga_ioport_write(opaque as *mut VgaState, 0x3d4u32.wrapping_add(addr), val);
}
}

// ── `vga_read_0x3ba` · vga.c:622-622 · chunk 3669 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_read_0x3ba(void *opaque, uint32_t addr, int size_log2){ return vga_ioport_read(opaque, 0x3ba + addr);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3669 {
// Port of vga_read_0x3ba (a VGA_IO(0x3ba) instance) plus the static helper
// vga_ioport_read it depends on, which has no linker symbol in C.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,

    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let s = &mut *s;
    let val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && (s.msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (s.msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if s.ar_flip_flop == 0 {
                    val = s.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (s.ar_index & 0x1f) as c_int;
                if index < 21 {
                    val = *s.ar.as_ptr().add(index as usize) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = s.st00 as c_int;
            }
            0x3c4 => {
                val = s.sr_index as c_int;
            }
            0x3c5 => {
                val = *s.sr.as_ptr().add(s.sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = s.dac_state as c_int;
            }
            0x3c8 => {
                val = s.dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = s.dac_read_index as usize * 3 + s.dac_sub_index as usize;
                val = *s.palette.as_ptr().add(idx) as c_int;
                s.dac_sub_index = s.dac_sub_index.wrapping_add(1);
                if s.dac_sub_index == 3 {
                    s.dac_sub_index = 0;
                    s.dac_read_index = s.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = s.fcr as c_int;
            }
            0x3cc => {
                val = s.msr as c_int;
            }
            0x3ce => {
                val = s.gr_index as c_int;
            }
            0x3cf => {
                val = *s.gr.as_ptr().add(s.gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = s.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = *s.cr.as_ptr().add(s.cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                s.st01 ^= 0x08 | 0x01;
                val = s.st01 as c_int;
                s.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3ba(opaque: *mut c_void, addr: u32, size_log2: c_int) -> u32 {
    let _ = size_log2;
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3bau32.wrapping_add(addr)) }
}
}

// ── `return vga_ioport_read(opaque, 0x3ba + addr);` · vga.c:622-622 · chunk 3670 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_read(opaque, 0x3ba + addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3670 {
// Port of the TARGET statement `return vga_ioport_read(opaque, 0x3ba + addr);`
// from vga_read_0x3ba.  vga_ioport_read is a `static` C function without a
// linker symbol, so its body is mirrored here as a private Rust fn.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VgaState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
}

#[inline]
unsafe fn ld_u8(base: *const u8, idx: usize) -> u8 {
    *base.add(idx)
}

unsafe fn vga_ioport_read(s: *mut VgaState, addr: u32) -> u32 {
    let val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if (*s).ar_flip_flop == 0 {
                    val = (*s).ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index: c_int = ((*s).ar_index & 0x1f) as c_int;
                if index < 21 {
                    val = ld_u8((*s).ar.as_ptr(), index as usize) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = (*s).st00 as c_int;
            }
            0x3c4 => {
                val = (*s).sr_index as c_int;
            }
            0x3c5 => {
                val = ld_u8((*s).sr.as_ptr(), (*s).sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = (*s).dac_state as c_int;
            }
            0x3c8 => {
                val = (*s).dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = (*s).dac_read_index as usize * 3 + (*s).dac_sub_index as usize;
                val = ld_u8((*s).palette.as_ptr(), idx) as c_int;
                let sub = (*s).dac_sub_index.wrapping_add(1);
                (*s).dac_sub_index = sub;
                if sub == 3 {
                    (*s).dac_sub_index = 0;
                    (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = (*s).fcr as c_int;
            }
            0x3cc => {
                val = (*s).msr as c_int;
            }
            0x3ce => {
                val = (*s).gr_index as c_int;
            }
            0x3cf => {
                val = ld_u8((*s).gr.as_ptr(), (*s).gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = (*s).cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = ld_u8((*s).cr.as_ptr(), (*s).cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                (*s).st01 ^= 0x08 | 0x01;
                val = (*s).st01 as c_int;
                (*s).ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3ba__part3670(
    opaque: *mut c_void,
    addr: u32,
) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VgaState, addr.wrapping_add(0x3ba)) }
}
}

// ── `vga_write_0x3ba` · vga.c:622-622 · chunk 3671 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_write_0x3ba(void *opaque, uint32_t addr, uint32_t val, int size_log2){ return vga_ioport_write(opaque, 0x3ba + addr, val);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3671 {
// Port of vga_write_0x3ba (macro VGA_IO(0x3ba) in vga.c), together with the
// static helper vga_ioport_write it forwards to.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

static SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2u8, !0xf0u8, !0xc0u8, !0xf1u8, !0xffu8, !0xffu8, !0x00u8,
];

static GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0u8, !0xf0u8, !0xe0u8, !0xfcu8, !0x84u8, !0xf0u8, !0xf0u8, !0x00u8, !0xffu8,
    !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8, !0xffu8,
];

unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    // (addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
    // (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01))
    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                let v = val & 0x3f;
                (*s).ar_index = v as u8;
            } else {
                let index: c_int = ((*s).ar_index & 0x1f) as c_int;
                match index {
                    0x00..=0x0f => {
                        (*s).ar[index as usize] = (val & 0x3f) as u8;
                    }
                    0x10 => {
                        (*s).ar[index as usize] = (val & !0x10u32) as u8;
                    }
                    0x11 => {
                        (*s).ar[index as usize] = val as u8;
                    }
                    0x12 => {
                        (*s).ar[index as usize] = (val & !0xc0u32) as u8;
                    }
                    0x13 => {
                        (*s).ar[index as usize] = (val & !0xf0u32) as u8;
                    }
                    0x14 => {
                        (*s).ar[index as usize] = (val & !0xf0u32) as u8;
                    }
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let idx = (*s).sr_index as usize;
            let m = SR_MASK[idx];
            (*s).sr[idx] = (val & (m as u32)) as u8;
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            (*s).dac_cache[sub] = val as u8;
            (*s).dac_sub_index = (*s).dac_sub_index.wrapping_add(1);
            if (*s).dac_sub_index == 3 {
                let off = ((*s).dac_write_index as usize) * 3;
                let dst = (&mut (*s).palette[off]) as *mut u8 as *mut c_void;
                let src = (&(*s).dac_cache[0]) as *const u8 as *const c_void;
                memcpy(dst, src, 3);
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let idx = (*s).gr_index as usize;
            let m = GR_MASK[idx];
            (*s).gr[idx] = (val & (m as u32)) as u8;
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if ((*s).cr[0x11] & 0x80) != 0 && (*s).cr_index <= 7 {
                if (*s).cr_index == 7 {
                    (*s).cr[7] = ((*s).cr[7] & !0x10u8) | ((val as u8) & 0x10);
                }
                return;
            }
            let idx = (*s).cr_index as usize;
            match (*s).cr_index {
                0x01 | 0x07 | 0x09 | 0x0c | 0x0d | 0x12 => {
                    (*s).cr[idx] = val as u8;
                }
                _ => {
                    (*s).cr[idx] = val as u8;
                }
            }
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn vga_write_0x3ba(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(
        opaque as *mut VGAState,
        0x3bau32.wrapping_add(addr),
        val,
    );
}
}

// ── `return vga_ioport_write(opaque, 0x3ba + addr, val);` · vga.c:622-622 · chunk 3672 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_write(opaque, 0x3ba + addr, val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3672 {
// Port of the body of vga_write_0x3ba's tail call: forwards a port write to
// vga_ioport_write with the 0x3ba base address added.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

// Mirror of the C struct VGAState layout (fields only accessed by layout here).
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

static SR_MASK: [u8; 8] = [0x03, 0x3d, 0x0f, 0x3f, 0x0e, 0x00, 0x00, 0xff];

static GR_MASK: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x1f, 0x03, 0x7b, 0x0f, 0x0f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

#[no_mangle]
pub extern "C" fn vga_write_0x3ba__part3672(opaque: *mut c_void, addr: u32, val: u32) {
    let s = opaque as *mut VGAState;
    unsafe {
        // if ((addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) ||
        //     (addr >= 0x3d0 && addr <= 0x3df && !(s->msr & 0x01)))
        //     return;
        let msr_bit = ((*s).msr & 0x01) != 0;
        if (addr >= 0x3b0 && addr <= 0x3bf && msr_bit)
            || (addr >= 0x3d0 && addr <= 0x3df && !msr_bit)
        {
            return;
        }

        match addr {
            0x3c0 => {
                if (*s).ar_flip_flop == 0 {
                    (*s).ar_index = (val & 0x3f) as u8;
                } else {
                    let index: c_int = ((*s).ar_index & 0x1f) as c_int;
                    let ar = (*s).ar.as_mut_ptr();
                    match index {
                        0x00..=0x0f => {
                            *ar.add(index as usize) = (val & 0x3f) as u8;
                        }
                        0x10 => {
                            *ar.add(index as usize) = (val & !0x10u32) as u8;
                        }
                        0x11 => {
                            *ar.add(index as usize) = val as u8;
                        }
                        0x12 => {
                            *ar.add(index as usize) = (val & !0xc0u32) as u8;
                        }
                        0x13 | 0x14 => {
                            *ar.add(index as usize) = (val & !0xf0u32) as u8;
                        }
                        _ => {}
                    }
                }
                (*s).ar_flip_flop ^= 1;
            }
            0x3c2 => {
                (*s).msr = (val & !0x10u32) as u8;
            }
            0x3c4 => {
                (*s).sr_index = (val & 7) as u8;
            }
            0x3c5 => {
                let i = (*s).sr_index as usize;
                let mask = *SR_MASK.as_ptr().add(i) as u32;
                *(*s).sr.as_mut_ptr().add(i) = (val & mask) as u8;
            }
            0x3c7 => {
                (*s).dac_read_index = val as u8;
                (*s).dac_sub_index = 0;
                (*s).dac_state = 3;
            }
            0x3c8 => {
                (*s).dac_write_index = val as u8;
                (*s).dac_sub_index = 0;
                (*s).dac_state = 0;
            }
            0x3c9 => {
                let i = (*s).dac_sub_index as usize;
                *(*s).dac_cache.as_mut_ptr().add(i) = val as u8;
                let n = (*s).dac_sub_index.wrapping_add(1);
                (*s).dac_sub_index = n;
                if n == 3 {
                    // memcpy(&s->palette[s->dac_write_index * 3], s->dac_cache, 3);
                    let off = ((*s).dac_write_index as usize).wrapping_mul(3);
                    core::ptr::copy_nonoverlapping(
                        (*s).dac_cache.as_ptr(),
                        (*s).palette.as_mut_ptr().add(off),
                        3,
                    );
                    (*s).dac_sub_index = 0;
                    (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
                }
            }
            0x3ce => {
                (*s).gr_index = (val & 0x0f) as u8;
            }
            0x3cf => {
                let i = (*s).gr_index as usize;
                let mask = *GR_MASK.as_ptr().add(i) as u32;
                *(*s).gr.as_mut_ptr().add(i) = (val & mask) as u8;
            }
            0x3b4 | 0x3d4 => {
                (*s).cr_index = val as u8;
            }
            0x3b5 | 0x3d5 => {
                let cr11 = *(*s).cr.as_ptr().add(0x11);
                if (cr11 & 0x80) != 0 && (*s).cr_index <= 7 {
                    if (*s).cr_index == 7 {
                        let c7 = *(*s).cr.as_ptr().add(7);
                        *(*s).cr.as_mut_ptr().add(7) = (c7 & 0xef) | ((val & 0x10) as u8);
                    }
                    return;
                }
                let i = (*s).cr_index as usize;
                *(*s).cr.as_mut_ptr().add(i) = val as u8;
            }
            0x3ba | 0x3da => {
                (*s).fcr = (val & 0x10) as u8;
            }
            _ => {}
        }
    }
}
}

// ── `vga_read_0x3da` · vga.c:623-623 · chunk 3673 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vga_read_0x3da(void *opaque, uint32_t addr, int size_log2){ return vga_ioport_read(opaque, 0x3da + addr);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3673 {
// Port of the VGA ioport read path used by vga_read_0x3da.
// The C helper `vga_ioport_read` is a static function (no linker symbol),
// so its logic is reproduced here as a private Rust function.

use core::ffi::{c_int, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

const MSR_COLOR_EMULATION: u8 = 0x01;
const ST01_V_RETRACE: u8 = 0x08;
const ST01_DISP_ENABLE: u8 = 0x01;

// Equivalent of the C static helper `vga_ioport_read`.
unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let st = &mut *s;
    let val: c_int;

    // check port range access depending on color/monochrome mode
    if (addr >= 0x3b0 && addr <= 0x3bf && (st.msr & MSR_COLOR_EMULATION) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (st.msr & MSR_COLOR_EMULATION) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if st.ar_flip_flop == 0 {
                    val = st.ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = (st.ar_index & 0x1f) as usize;
                if index < 21 {
                    val = *st.ar.as_ptr().add(index) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = st.st00 as c_int;
            }
            0x3c4 => {
                val = st.sr_index as c_int;
            }
            0x3c5 => {
                val = *st.sr.as_ptr().add(st.sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = st.dac_state as c_int;
            }
            0x3c8 => {
                val = st.dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = st.dac_read_index as usize * 3 + st.dac_sub_index as usize;
                val = *st.palette.as_ptr().add(idx) as c_int;
                st.dac_sub_index = st.dac_sub_index.wrapping_add(1);
                if st.dac_sub_index == 3 {
                    st.dac_sub_index = 0;
                    st.dac_read_index = st.dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = st.fcr as c_int;
            }
            0x3cc => {
                val = st.msr as c_int;
            }
            0x3ce => {
                val = st.gr_index as c_int;
            }
            0x3cf => {
                val = *st.gr.as_ptr().add(st.gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = st.cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = *st.cr.as_ptr().add(st.cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                st.st01 ^= ST01_V_RETRACE | ST01_DISP_ENABLE;
                val = st.st01 as c_int;
                st.ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3da(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3da_u32.wrapping_add(addr)) }
}
}

// ── `return vga_ioport_read(opaque, 0x3da + addr);` · vga.c:623-623 · chunk 3674 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_read(opaque, 0x3da + addr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3674 {
// Port of the TARGET `return vga_ioport_read(opaque, 0x3da + addr);`
// vga_ioport_read is a static C function (no linker symbol) so its logic is
// reproduced here as a private Rust fn.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,
    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],
    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,
    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let mut val: c_int;

    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & 0x01) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if (*s).ar_flip_flop == 0 {
                    val = (*s).ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = ((*s).ar_index & 0x1f) as usize;
                if index < 21 {
                    val = *(*s).ar.as_ptr().add(index) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => {
                val = (*s).st00 as c_int;
            }
            0x3c4 => {
                val = (*s).sr_index as c_int;
            }
            0x3c5 => {
                val = *(*s).sr.as_ptr().add((*s).sr_index as usize) as c_int;
            }
            0x3c7 => {
                val = (*s).dac_state as c_int;
            }
            0x3c8 => {
                val = (*s).dac_write_index as c_int;
            }
            0x3c9 => {
                let idx = (*s).dac_read_index as usize * 3 + (*s).dac_sub_index as usize;
                val = *(*s).palette.as_ptr().add(idx) as c_int;
                let sub = (*s).dac_sub_index.wrapping_add(1);
                (*s).dac_sub_index = sub;
                if sub == 3 {
                    (*s).dac_sub_index = 0;
                    (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => {
                val = (*s).fcr as c_int;
            }
            0x3cc => {
                val = (*s).msr as c_int;
            }
            0x3ce => {
                val = (*s).gr_index as c_int;
            }
            0x3cf => {
                val = *(*s).gr.as_ptr().add((*s).gr_index as usize) as c_int;
            }
            0x3b4 | 0x3d4 => {
                val = (*s).cr_index as c_int;
            }
            0x3b5 | 0x3d5 => {
                val = *(*s).cr.as_ptr().add((*s).cr_index as usize) as c_int;
            }
            0x3ba | 0x3da => {
                (*s).st01 ^= 0x08 | 0x01;
                val = (*s).st01 as c_int;
                (*s).ar_flip_flop = 0;
            }
            _ => {
                val = 0x00;
            }
        }
    }

    val as u32
}

#[no_mangle]
pub extern "C" fn vga_read_0x3da__part3674(opaque: *mut c_void, addr: u32) -> u32 {
    unsafe { vga_ioport_read(opaque as *mut VGAState, 0x3dau32.wrapping_add(addr)) }
}
}

// ── `vga_write_0x3da` · vga.c:623-623 · chunk 3675 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static void vga_write_0x3da(void *opaque, uint32_t addr, uint32_t val, int size_log2){ return vga_ioport_write(opaque, 0x3da + addr, val);}
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3675 {
#![allow(dead_code)]
// Port of the static VGA_IO(0x3da) write wrapper plus the static helper
// vga_ioport_write() it calls (a static C function has no linker symbol,
// so its logic is carried here).

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,
    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// static const uint8_t sr_mask[8] = { (uint8_t)~0xfc, ... };
const SR_MASK: [u8; 8] = [0x03, 0x3d, 0x0f, 0x3f, 0x0e, 0x00, 0x00, 0xff];

// static const uint8_t gr_mask[16] = { (uint8_t)~0xf0, ... };
const GR_MASK: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x1f, 0x03, 0x7b, 0x0f, 0x0f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    let msr = (*s).msr;
    if (addr >= 0x3b0 && addr <= 0x3bf && (msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && (msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index = (val & 0x3f) as u8;
            } else {
                let index = ((*s).ar_index & 0x1f) as usize;
                let arp = (*s).ar.as_mut_ptr();
                match index {
                    0x00..=0x0f => *arp.add(index) = (val & 0x3f) as u8,
                    0x10 => *arp.add(index) = (val & !0x10u32) as u8,
                    0x11 => *arp.add(index) = val as u8,
                    0x12 => *arp.add(index) = (val & !0xc0u32) as u8,
                    0x13 | 0x14 => *arp.add(index) = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let i = (*s).sr_index as usize;
            let m = *SR_MASK.as_ptr().add(i) as u32;
            *(*s).sr.as_mut_ptr().add(i) = (val & m) as u8;
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            *(*s).dac_cache.as_mut_ptr().add(sub) = val as u8;
            let new_sub = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = new_sub;
            if new_sub == 3 {
                let widx = (*s).dac_write_index as usize;
                let src = (*s).dac_cache.as_ptr();
                let dst = (*s).palette.as_mut_ptr().add(widx.wrapping_mul(3));
                core::ptr::copy_nonoverlapping(src, dst, 3);
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let i = (*s).gr_index as usize;
            let m = *GR_MASK.as_ptr().add(i) as u32;
            *(*s).gr.as_mut_ptr().add(i) = (val & m) as u8;
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if ((*s).cr[0x11] & 0x80) != 0 && (*s).cr_index <= 7 {
                if (*s).cr_index == 7 {
                    let c7 = (*s).cr.as_mut_ptr().add(7);
                    *c7 = ((*c7) & !0x10u8) | ((val & 0x10) as u8);
                }
                return;
            }
            let i = (*s).cr_index as usize;
            *(*s).cr.as_mut_ptr().add(i) = val as u8;
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3da(opaque: *mut c_void, addr: u32, val: u32, _size_log2: c_int) {
    unsafe {
        vga_ioport_write(opaque as *mut VGAState, 0x3dau32.wrapping_add(addr), val);
    }
}
}

// ── `return vga_ioport_write(opaque, 0x3da + addr, val);` · vga.c:623-623 · chunk 3676 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vga_ioport_write(opaque, 0x3da + addr, val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3676 {
// Port of the `return vga_ioport_write(opaque, 0x3da + addr, val);` statement
// inside vga_write_0x3da().  vga_ioport_write() is a static C function with no
// linker symbol, so its logic is re-implemented here as a private Rust fn.

use core::ffi::{c_int, c_void};

// Mirror of the C `struct VGAState` layout (exact fields, in order).  Pointer
// fields that are never dereferenced here are kept as opaque raw pointers,
// which have the identical size and alignment.
#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,

    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

// static const uint8_t sr_mask[8] / gr_mask[16] from the C translation unit.
static SR_MASK: [u8; 8] = [
    0x03, 0x3d, 0x0f, 0x3f, 0x0e, 0x00, 0x00, 0xff,
];

static GR_MASK: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x1f, 0x03, 0x7b, 0x0f, 0x0f,
    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// static void vga_ioport_write(VGAState *s, uint32_t addr, uint32_t val)
unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & 0x01) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & 0x01) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index = (val & 0x3f) as u8;
            } else {
                let index = ((*s).ar_index & 0x1f) as usize;
                let ar = (*s).ar.as_mut_ptr();
                match index {
                    0x00..=0x0f => *ar.add(index) = (val & 0x3f) as u8,
                    0x10 => *ar.add(index) = (val & !0x10u32) as u8,
                    0x11 => *ar.add(index) = val as u8,
                    0x12 => *ar.add(index) = (val & !0xc0u32) as u8,
                    0x13 => *ar.add(index) = (val & !0xf0u32) as u8,
                    0x14 => *ar.add(index) = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let idx = (*s).sr_index as usize;
            let mask = *core::ptr::addr_of!(SR_MASK).cast::<u8>().add(idx) as u32;
            let sr = (*s).sr.as_mut_ptr();
            *sr.add(idx) = (val & mask) as u8;
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            let cache = (*s).dac_cache.as_mut_ptr();
            *cache.add(sub) = val as u8;
            let nsub = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = nsub;
            if nsub == 3 {
                // memcpy(&s->palette[s->dac_write_index * 3], s->dac_cache, 3);
                let off = (*s).dac_write_index as usize * 3;
                let src = core::ptr::addr_of!((*s).dac_cache) as *const u8;
                let dst = core::ptr::addr_of_mut!((*s).palette) as *mut u8;
                core::ptr::copy_nonoverlapping(src, dst.add(off), 3);
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let idx = (*s).gr_index as usize;
            let mask = *core::ptr::addr_of!(GR_MASK).cast::<u8>().add(idx) as u32;
            let gr = (*s).gr.as_mut_ptr();
            *gr.add(idx) = (val & mask) as u8;
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            if ((*s).cr[0x11] & 0x80) != 0 && (*s).cr_index <= 7 {
                if (*s).cr_index == 7 {
                    (*s).cr[7] = (((*s).cr[7] as u32) & !0x10u32 | (val & 0x10)) as u8;
                }
                return;
            }
            let idx = (*s).cr_index as usize;
            let cr = (*s).cr.as_mut_ptr();
            *cr.add(idx) = val as u8;
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn vga_write_0x3da__part3676(opaque: *mut c_void, addr: u32, val: u32) {
    unsafe {
        vga_ioport_write(opaque as *mut VGAState, addr, val);
    }
}
}

// ── `vbe_write` · vga.c:625-676 · chunk 3677 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void vbe_write(void *opaque, uint32_t offset,
//                           uint32_t val, int size_log2)
//     {
//         VGAState *s = opaque;
//         FBDevice *fb_dev = s->fb_dev;
//     
//         if (offset == 0) {
//             s->vbe_index = val;
//         } else {
//     
//     
//     
//             switch(s->vbe_index) {
//             case 0x0:
//                 if (val >= 0xB0C0 && val <= 0xB0C5)
//                     s->vbe_regs[s->vbe_index] = val;
//                 break;
//             case 0x4:
//                 if ((val & 0x01) &&
//                     !(s->vbe_regs[0x4] & 0x01)) {
//     
//     
//                     if (s->vbe_regs[0x1] <= 4096 &&
//                         s->vbe_regs[0x2] <= 4096 &&
//                         (s->vbe_regs[0x1] * 4 *
//                          s->vbe_regs[0x2]) <= fb_dev->fb_size) {
//                         fb_dev->width = s->vbe_regs[0x1];
//                         fb_dev->height = s->vbe_regs[0x2];
//                         fb_dev->stride = fb_dev->width * 4;
//                     }
//                     s->vbe_regs[0x6] =
//                         s->vbe_regs[0x1];
//                     s->vbe_regs[0x7] =
//                         s->vbe_regs[0x2];
//                     s->vbe_regs[0x8] = 0;
//                     s->vbe_regs[0x9] = 0;
//                 }
//                 s->vbe_regs[s->vbe_index] = val;
//                 break;
//             case 0x1:
//             case 0x2:
//             case 0x3:
//             case 0x5:
//             case 0x6:
//             case 0x7:
//             case 0x8:
//             case 0x9:
//                 s->vbe_regs[s->vbe_index] = val;
//                 break;
//             }
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3677 {
// Rust rewrite of vbe_write (from vga.c)

use std::os::raw::{c_int, c_void};

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
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub unsafe extern "C" fn vbe_write(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut VGAState;
    let fb_dev = (*s).fb_dev;

    if offset == 0 {
        (*s).vbe_index = val as u16;
    } else {
        match (*s).vbe_index {
            0x0 => {
                if val >= 0xB0C0 && val <= 0xB0C5 {
                    (*s).vbe_regs[0x0] = val as u16;
                }
            }
            0x4 => {
                if (val & 0x01) != 0 && ((*s).vbe_regs[0x4] & 0x01) == 0 {
                    let xres = (*s).vbe_regs[0x1] as c_int;
                    let yres = (*s).vbe_regs[0x2] as c_int;
                    if xres <= 4096
                        && yres <= 4096
                        && xres.wrapping_mul(4).wrapping_mul(yres) <= (*fb_dev).fb_size
                    {
                        (*fb_dev).width = xres;
                        (*fb_dev).height = yres;
                        (*fb_dev).stride = (*fb_dev).width.wrapping_mul(4);
                    }
                    (*s).vbe_regs[0x6] = (*s).vbe_regs[0x1];
                    (*s).vbe_regs[0x7] = (*s).vbe_regs[0x2];
                    (*s).vbe_regs[0x8] = 0;
                    (*s).vbe_regs[0x9] = 0;
                }
                (*s).vbe_regs[0x4] = val as u16;
            }
            0x1 | 0x2 | 0x3 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 => {
                let idx = (*s).vbe_index as usize;
                if idx < 0xb {
                    (*s).vbe_regs[idx] = val as u16;
                }
            }
            _ => {}
        }
    }
}
}

// ── `if (offset == 0) … else` · vga.c:631-675 · chunk 3678 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset == 0) {
//             s->vbe_index = val;
//         } else {
//     
//     
//     
//             switch(s->vbe_index) {
//             case 0x0:
//                 if (val >= 0xB0C0 && val <= 0xB0C5)
//                     s->vbe_regs[s->vbe_index] = val;
//                 break;
//             case 0x4:
//                 if ((val & 0x01) &&
//                     !(s->vbe_regs[0x4] & 0x01)) {
//     
//     
//                     if (s->vbe_regs[0x1] <= 4096 &&
//                         s->vbe_regs[0x2] <= 4096 &&
//                         (s->vbe_regs[0x1] * 4 *
//                          s->vbe_regs[0x2]) <= fb_dev->fb_size) {
//                         fb_dev->width = s->vbe_regs[0x1];
//                         fb_dev->height = s->vbe_regs[0x2];
//                         fb_dev->stride = fb_dev->width * 4;
//                     }
//                     s->vbe_regs[0x6] =
//                         s->vbe_regs[0x1];
//                     s->vbe_regs[0x7] =
//                         s->vbe_regs[0x2];
//                     s->vbe_regs[0x8] = 0;
//                     s->vbe_regs[0x9] = 0;
//                 }
//                 s->vbe_regs[s->vbe_index] = val;
//                 break;
//             case 0x1:
//             case 0x2:
//             case 0x3:
//             case 0x5:
//             case 0x6:
//             case 0x7:
//             case 0x8:
//             case 0x9:
//                 s->vbe_regs[s->vbe_index] = val;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3678 {
// Port of the register-write part of vbe_write() from vga.c.
use core::ffi::{c_int, c_void};

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
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[no_mangle]
pub unsafe extern "C" fn vbe_write__part3678(
    s: *mut VGAState,
    fb_dev: *mut FBDevice,
    offset: u32,
    val: u32,
) {
    if offset == 0 {
        (*s).vbe_index = val as u16;
        return;
    }

    let st = &mut *s;
    let regs = (&mut st.vbe_regs) as *mut [u16; 0xb] as *mut u16;
    let idx = st.vbe_index;

    match idx {
        0x0 => {
            if val >= 0xB0C0u32 && val <= 0xB0C5u32 {
                *regs.add(idx as usize) = val as u16;
            }
        }
        0x4 => {
            if (val & 0x01) != 0 && (*regs.add(0x4) & 0x01) == 0 {
                let w = *regs.add(0x1);
                let h = *regs.add(0x2);
                if (w as c_int) <= 4096
                    && (h as c_int) <= 4096
                    && ((w as c_int) * 4 * (h as c_int)) <= (*fb_dev).fb_size
                {
                    (*fb_dev).width = w as c_int;
                    (*fb_dev).height = h as c_int;
                    (*fb_dev).stride = (*fb_dev).width * 4;
                }
                *regs.add(0x6) = w;
                *regs.add(0x7) = h;
                *regs.add(0x8) = 0;
                *regs.add(0x9) = 0;
            }
            *regs.add(idx as usize) = val as u16;
        }
        0x1 | 0x2 | 0x3 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 => {
            *regs.add(idx as usize) = val as u16;
        }
        _ => {}
    }
}
}

// ── `vbe_read` · vga.c:678-712 · chunk 3679 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static uint32_t vbe_read(void *opaque, uint32_t offset, int size_log2)
//     {
//         VGAState *s = opaque;
//         uint32_t val;
//     
//         if (offset == 0) {
//             val = s->vbe_index;
//         } else {
//             if (s->vbe_regs[0x4] & 0x02) {
//                 switch(s->vbe_index) {
//                 case 0x1:
//                     val = s->fb_dev->width;
//                     break;
//                 case 0x2:
//                     val = s->fb_dev->height;
//                     break;
//                 case 0x3:
//                     val = 32;
//                     break;
//                 default:
//                     goto read_reg;
//                 }
//             } else {
//             read_reg:
//                 if (s->vbe_index < 0xb)
//                     val = s->vbe_regs[s->vbe_index];
//                 else
//                     val = 0;
//             }
//     
//     
//     
//         }
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3679 {
// Port of static uint32_t vbe_read(void *opaque, uint32_t offset, int size_log2)
// Mirrors the C structs exactly (field order / types) so pointer arithmetic matches.

use std::os::raw::c_void;

#[repr(C)]
pub struct FBDevice {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub fb_data: *mut u8,
    pub fb_size: i32,
    pub device_opaque: *mut c_void,
    pub refresh: *mut c_void,
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: i32,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: i32,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

// The C `read_reg:` label target.
unsafe fn read_reg(s: *mut VGAState) -> u32 {
    let idx = (*s).vbe_index;
    if (idx as u32) < 0xb {
        (*s).vbe_regs[idx as usize] as u32
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn vbe_read(opaque: *mut c_void, offset: u32, _size_log2: i32) -> u32 {
    let s = opaque as *mut VGAState;
    let val: u32;

    if offset == 0 {
        val = (*s).vbe_index as u32;
    } else {
        if ((*s).vbe_regs[0x4] & 0x02) != 0 {
            match (*s).vbe_index {
                0x1 => {
                    let fb = (*s).fb_dev;
                    val = if fb.is_null() { 0 } else { (*fb).width as u32 };
                }
                0x2 => {
                    let fb = (*s).fb_dev;
                    val = if fb.is_null() { 0 } else { (*fb).height as u32 };
                }
                0x3 => {
                    val = 32;
                }
                _ => {
                    val = read_reg(s);
                }
            }
        } else {
            val = read_reg(s);
        }
    }
    val
}
}

// ── `if (offset == 0) … else` · vga.c:683-710 · chunk 3680 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset == 0) {
//             val = s->vbe_index;
//         } else {
//             if (s->vbe_regs[0x4] & 0x02) {
//                 switch(s->vbe_index) {
//                 case 0x1:
//                     val = s->fb_dev->width;
//                     break;
//                 case 0x2:
//                     val = s->fb_dev->height;
//                     break;
//                 case 0x3:
//                     val = 32;
//                     break;
//                 default:
//                     goto read_reg;
//                 }
//             } else {
//             read_reg:
//                 if (s->vbe_index < 0xb)
//                     val = s->vbe_regs[s->vbe_index];
//                 else
//                     val = 0;
//             }
//     
//     
//     
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3680 {
// Rust port of a piece of vbe_read() from vga.c

use core::ffi::{c_int, c_void};

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
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut c_void,
    pub mem_range2: *mut c_void,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut c_void,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

#[inline]
unsafe fn read_reg(s: *mut VGAState) -> u32 {
    let idx = (*s).vbe_index;
    if (idx as c_int) < 0xb {
        let p = (*s).vbe_regs.as_ptr();
        *p.add(idx as usize) as u32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn vbe_read__part3680(s: *mut VGAState, offset: u32) -> u32 {
    unsafe {
        if offset == 0 {
            (*s).vbe_index as u32
        } else if ((*s).vbe_regs[4] & 0x02) != 0 {
            match (*s).vbe_index {
                0x1 => (*(*s).fb_dev).width as u32,
                0x2 => (*(*s).fb_dev).height as u32,
                0x3 => 32,
                _ => read_reg(s),
            }
        } else {
            read_reg(s)
        }
    }
}
}

// ── `return val;` · vga.c:711-711 · chunk 3681 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3681 {
// Ported piece of vbe_read: returns the computed register value (uint32_t -> u32).
#[no_mangle]
pub extern "C" fn vbe_read__part3681(val: u32) -> u32 {
    val
}
}

// ── `simplefb_bar_set` · vga.c:715-723 · chunk 3682 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void simplefb_bar_set(void *opaque, int bar_num,
//                                   uint32_t addr, BOOL enabled)
//     {
//         VGAState *s = opaque;
//         if (bar_num == 0)
//             phys_mem_set_addr(s->mem_range, addr, enabled);
//         else
//             phys_mem_set_addr(s->rom_range, addr, enabled);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3682 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut c_void,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut c_void,
    pub rom_range: *mut PhysMemoryRange,
}

extern "C" {
    fn phys_mem_set_addr(pr: *mut PhysMemoryRange, addr: u64, enabled: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn simplefb_bar_set(
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: c_int,
) {
    let s = opaque as *mut VGAState;
    if bar_num == 0 {
        phys_mem_set_addr((*s).mem_range, addr as u64, enabled);
    } else {
        phys_mem_set_addr((*s).rom_range, addr as u64, enabled);
    }
}
}

// ── `if (bar_num == 0) … else` · vga.c:719-722 · chunk 3683 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bar_num == 0)
//             phys_mem_set_addr(s->mem_range, addr, enabled);
//         else
//             phys_mem_set_addr(s->rom_range, addr, enabled);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3683 {
// Rust port of the BAR-address dispatch inside the C function simplefb_bar_set.
use core::ffi::c_int;

pub type BOOL = c_int;

// PhysMemoryRange is only ever handled behind a pointer here.
#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

// Layout-compatible prefix of the C struct VGAState (fields up to rom_range).
#[repr(C)]
pub struct VGAState {
    fb_dev: *mut core::ffi::c_void,
    fb_page_count: c_int,
    mem_range: *mut PhysMemoryRange,
    mem_range2: *mut PhysMemoryRange,
    pci_dev: *mut core::ffi::c_void,
    rom_range: *mut PhysMemoryRange,
}

extern "C" {
    fn phys_mem_set_addr(pr: *mut PhysMemoryRange, addr: u64, enabled: BOOL);
}

#[no_mangle]
pub extern "C" fn simplefb_bar_set__part3683(
    s: *mut VGAState,
    bar_num: c_int,
    addr: u32,
    enabled: BOOL,
) {
    unsafe {
        let pr: *mut PhysMemoryRange = if bar_num == 0 {
            (*s).mem_range
        } else {
            (*s).rom_range
        };
        phys_mem_set_addr(pr, addr as u64, enabled);
    }
}
}

// ── `pci_vga_init` · vga.c:725-804 · chunk 3684 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VGAState *pci_vga_init(PCIBus *bus, FBDevice *fb_dev,
//                            int width, int height,
//                            const uint8_t *vga_rom_buf, int vga_rom_size)
//     {
//         VGAState *s;
//         PCIDevice *d;
//         uint32_t bar_size;
//         PhysMemoryMap *mem_map, *port_map;
//     
//         d = pci_register_device(bus, "VGA", -1, 0x1234, 0x1111, 0x00, 0x0300);
//     
//         mem_map = pci_device_get_mem_map(d);
//         port_map = pci_device_get_port_map(d);
//     
//         s = mallocz(sizeof(*s));
//         s->fb_dev = fb_dev;
//     
//         fb_dev->width = width;
//         fb_dev->height = height;
//         fb_dev->stride = width * 4;
//     
//         fb_dev->fb_size = (height * fb_dev->stride + (1 << 20) - 1) & ~((1 << 20) - 1);
//         s->fb_page_count = fb_dev->fb_size >> 12;
//     
//         s->mem_range =
//             cpu_register_ram(mem_map, 0, fb_dev->fb_size,
//                              (1 << 1) | (1 << 2));
//     
//         fb_dev->fb_data = s->mem_range->phys_mem;
//     
//         s->pci_dev = d;
//         bar_size = 1;
//         while (bar_size < fb_dev->fb_size)
//             bar_size <<= 1;
//         pci_register_bar(d, 0, bar_size, 0x00, s,
//                          simplefb_bar_set);
//     
//         if (vga_rom_size > 0) {
//             int rom_size;
//     
//             rom_size = (vga_rom_size + (1 << 12) - 1) & ~((1 << 12) - 1);
//             s->rom_range = cpu_register_ram(mem_map, 0, rom_size,
//                                             (1 << 0) | (1 << 2));
//             memcpy(s->rom_range->phys_mem, vga_rom_buf, vga_rom_size);
//     
//             bar_size = 1;
//             while (bar_size < rom_size)
//                 bar_size <<= 1;
//             pci_register_bar(d, 6, bar_size, 0x00, s,
//                              simplefb_bar_set);
//         }
//     
//     
//         s->mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0);
//         s->vga_ram = s->mem_range2->phys_mem;
//     
//     
//     
//         cpu_register_device(port_map, 0x3c0, 16, s, vga_read_0x3c0, vga_write_0x3c0,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3b4, 2, s, vga_read_0x3b4, vga_write_0x3b4,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3d4, 2, s, vga_read_0x3d4, vga_write_0x3d4,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3ba, 1, s, vga_read_0x3ba, vga_write_0x3ba,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3da, 1, s, vga_read_0x3da, vga_write_0x3da,
//                             (1 << 0));
//     
//     
//         cpu_register_device(port_map, 0x1ce, 2, s, vbe_read, vbe_write,
//                             (1 << 1));
//     
//         s->vbe_regs[0x0] = 0xB0C5;
//         s->vbe_regs[0xa] = fb_dev->fb_size >> 16;
//     
//         fb_dev->device_opaque = s;
//         fb_dev->refresh = vga_refresh;
//         return s;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3684 {
// Rust rewrite of pci_vga_init() from vga.c (TinyEMU).
// The static helper functions of vga.c (simplefb_bar_set, vga_refresh,
// vga_text_refresh, vga_ioport_read/write, vbe_read/write, ...) have internal
// linkage in C, so their logic is ported here as private Rust functions.
#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type BOOL = c_int;

// cutils.h
const TRUE: BOOL = 1;

// iomem.h
const DEVIO_SIZE8: c_int = 1 << 0;
const DEVIO_SIZE16: c_int = 1 << 1;
const DEVRAM_FLAG_ROM: c_int = 1 << 0;
const DEVRAM_FLAG_DIRTY_BITS: c_int = 1 << 1;
const DEVRAM_FLAG_DISABLED: c_int = 1 << 2;
const DEVRAM_PAGE_SIZE: c_int = 1 << 12;

// vga.c
const MSR_COLOR_EMULATION: u8 = 0x01;
const ST01_V_RETRACE: u8 = 0x08;
const ST01_DISP_ENABLE: u8 = 0x01;
const VBE_DISPI_ENABLED: u32 = 0x01;
const VBE_DISPI_GETCAPS: u16 = 0x02;
const FB_ALLOC_ALIGN: c_int = 1 << 20;
const MAX_TEXT_WIDTH: c_int = 132;
const MAX_TEXT_HEIGHT: c_int = 60;
const VBE_DISPI_INDEX_NB: usize = 0xb;

const SR_MASK: [u8; 8] = [
    !0xfcu8, !0xc2, !0xf0, !0xc0, !0xf1, !0xff, !0xff, !0x00,
];

const GR_MASK: [u8; 16] = [
    !0xf0u8, !0xf0, !0xf0, !0xe0, !0xfc, !0x84, !0xf0, !0xf0, !0x00, !0xff, !0xff, !0xff, !0xff,
    !0xff, !0xff, !0xff,
];

static CURSOR_GLYPH: [u8; 32] = [0xff; 32];

#[repr(C)]
pub struct PCIBus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
pub type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, BOOL);
pub type SimpleFBDrawFunc =
    unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int);

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

pub type RegisterRAMFunc =
    unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange;

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRAMFunc>,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, BOOL)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

// machine.h
#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh:
        Option<unsafe extern "C" fn(*mut FBDevice, Option<SimpleFBDrawFunc>, *mut c_void)>,
}

// vga.c
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut PCIDevice,
    pub rom_range: *mut PhysMemoryRange,

    pub vga_ram: *mut u8,

    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],

    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,

    pub vbe_index: u16,
    pub vbe_regs: [u16; VBE_DISPI_INDEX_NB],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;

    // iomem.c
    fn cpu_register_device(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn phys_mem_set_addr(pr: *mut PhysMemoryRange, addr: u64, enabled: BOOL);

    // pci.c
    fn pci_register_device(
        b: *mut PCIBus,
        name: *const c_char,
        devfn: c_int,
        vendor_id: u16,
        device_id: u16,
        revision: u8,
        class_id: u16,
    ) -> *mut PCIDevice;
    fn pci_device_get_mem_map(d: *mut PCIDevice) -> *mut PhysMemoryMap;
    fn pci_device_get_port_map(d: *mut PCIDevice) -> *mut PhysMemoryMap;
    fn pci_register_bar(
        d: *mut PCIDevice,
        bar_num: c_uint,
        size: u32,
        bar_type: c_int,
        opaque: *mut c_void,
        bar_set: Option<PCIBarSetFunc>,
    );

    // simplefb.c
    fn simplefb_refresh(
        fb_dev: *mut FBDevice,
        redraw_func: Option<SimpleFBDrawFunc>,
        opaque: *mut c_void,
        mem_range: *mut PhysMemoryRange,
        fb_page_count: c_int,
    );
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
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
fn c6_to_8(v: c_int) -> c_int {
    let v = v & 0x3f;
    let b = v & 1;
    (v << 2) | (b << 1) | b
}

// static inline in iomem.h: s->register_ram(s, addr, size, devram_flags)
#[inline]
unsafe fn cpu_register_ram(
    map: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    match (*map).register_ram {
        Some(f) => f(map, addr, size, devram_flags),
        None => core::ptr::null_mut(),
    }
}

unsafe fn vga_draw_glyph8(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
) {
    let xorcol = bgcol ^ fgcol;
    let mut d = d;
    let mut font_ptr = font_ptr;
    let mut h = h;
    loop {
        let font_data: u32 = *font_ptr as u32;
        core::ptr::write_unaligned(
            d as *mut u32,
            (((font_data >> 7).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(1),
            ((((font_data >> 6) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(2),
            ((((font_data >> 5) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(3),
            ((((font_data >> 4) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(4),
            ((((font_data >> 3) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(5),
            ((((font_data >> 2) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(6),
            ((((font_data >> 1) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(7),
            (((font_data & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        font_ptr = font_ptr.add(1);
        d = d.offset(linesize as isize);
        h = h.wrapping_sub(1);
        if h == 0 {
            break;
        }
    }
}

unsafe fn vga_draw_glyph9(
    d: *mut u8,
    linesize: c_int,
    font_ptr: *const u8,
    h: c_int,
    fgcol: u32,
    bgcol: u32,
    dup9: c_int,
) {
    let xorcol = bgcol ^ fgcol;
    let mut d = d;
    let mut font_ptr = font_ptr;
    let mut h = h;
    loop {
        let font_data: u32 = *font_ptr as u32;
        core::ptr::write_unaligned(
            d as *mut u32,
            (((font_data >> 7).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(1),
            ((((font_data >> 6) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(2),
            ((((font_data >> 5) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(3),
            ((((font_data >> 4) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(4),
            ((((font_data >> 3) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(5),
            ((((font_data >> 2) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        core::ptr::write_unaligned(
            (d as *mut u32).add(6),
            ((((font_data >> 1) & 1).wrapping_neg()) & xorcol) ^ bgcol,
        );
        let v = (((font_data & 1).wrapping_neg()) & xorcol) ^ bgcol;
        core::ptr::write_unaligned((d as *mut u32).add(7), v);
        if dup9 != 0 {
            core::ptr::write_unaligned((d as *mut u32).add(8), v);
        } else {
            core::ptr::write_unaligned((d as *mut u32).add(8), bgcol);
        }
        font_ptr = font_ptr.add(1);
        d = d.offset(linesize as isize);
        h = h.wrapping_sub(1);
        if h == 0 {
            break;
        }
    }
}

unsafe fn update_palette16(s: *mut VGAState, palette: *mut u32) -> c_int {
    let mut full_update: c_int = 0;
    let mut i: usize = 0;
    while i < 16 {
        let mut v: u32 = *(*s).ar.as_ptr().add(i) as u32;
        if ((*s).ar[0x10] & 0x80) != 0 {
            v = (((*s).ar[0x14] & 0xf) as u32) << 4 | (v & 0xf);
        } else {
            v = (((*s).ar[0x14] & 0xc) as u32) << 4 | (v & 0x3f);
        }
        v = v.wrapping_mul(3);
        let pal = (*s).palette.as_ptr();
        let vi = v as usize;
        let col = ((c6_to_8(*pal.add(vi) as c_int) as u32) << 16)
            | ((c6_to_8(*pal.add(vi + 1) as c_int) as u32) << 8)
            | (c6_to_8(*pal.add(vi + 2) as c_int) as u32);
        if col != *palette.add(i) {
            full_update = 1;
            *palette.add(i) = col;
        }
        i += 1;
    }
    full_update
}

unsafe fn vga_text_refresh(
    s: *mut VGAState,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
) {
    let fb_dev = (*s).fb_dev;
    let mut full_update: c_int = update_palette16(s, (*s).last_palette.as_mut_ptr());

    let vga_ram = (*s).vga_ram;

    let mut line_offset: u32 = (*s).cr[0x13] as u32;
    line_offset <<= 3;
    line_offset >>= 1;

    let start_addr: u32 = ((*s).cr[0x0d] as u32) | (((*s).cr[0x0c] as u32) << 8);

    let cheight: c_int = ((*s).cr[9] & 0x1f) as c_int + 1;
    let mut cwidth: c_int = 8;
    if ((*s).sr[1] & 0x01) == 0 {
        cwidth += 1;
    }

    let width: c_int = (*s).cr[0x01] as c_int + 1;
    let mut height: c_int = ((*s).cr[0x12] as c_int)
        | ((((*s).cr[0x07] & 0x02) as c_int) << 7)
        | ((((*s).cr[0x07] & 0x40) as c_int) << 3);
    height = (height + 1) / cheight;

    let width1: c_int = width.wrapping_mul(cwidth);
    let height1: c_int = height.wrapping_mul(cheight);
    if (*fb_dev).width < width1
        || (*fb_dev).height < height1
        || width > MAX_TEXT_WIDTH
        || height > MAX_TEXT_HEIGHT
    {
        return;
    }
    if ((*s).last_line_offset as u32) != line_offset
        || ((*s).last_start_addr as u32) != start_addr
        || (*s).last_width != width as u32
        || (*s).last_height != height as u32
    {
        (*s).last_line_offset = line_offset as u16;
        (*s).last_start_addr = start_addr as u16;
        (*s).last_width = width as u32;
        (*s).last_height = height as u32;
        full_update = TRUE;
    }

    // update cursor position
    let mut cursor_offset: u32 = ((((*s).cr[0x0e] as u32) << 8) | ((*s).cr[0x0f] as u32))
        .wrapping_sub(start_addr);
    let cursor_start: u32 = (*s).cr[0xa] as u32;
    let cursor_end: u32 = (*s).cr[0xb] as u32;
    if cursor_offset != (*s).last_cursor_offset as u32
        || cursor_start != (*s).last_cursor_start as u32
        || cursor_end != (*s).last_cursor_end as u32
    {
        if ((*s).last_cursor_offset as u32) < (MAX_TEXT_WIDTH * MAX_TEXT_HEIGHT) as u32 {
            *(*s)
                .last_ch_attr
                .as_mut_ptr()
                .add((*s).last_cursor_offset as usize) = 0xffff;
        }
        if cursor_offset < (MAX_TEXT_WIDTH * MAX_TEXT_HEIGHT) as u32 {
            *(*s).last_ch_attr.as_mut_ptr().add(cursor_offset as usize) = 0xffff;
        }
        (*s).last_cursor_offset = cursor_offset as u16;
        (*s).last_cursor_start = cursor_start as u8;
        (*s).last_cursor_end = cursor_end as u8;
    }

    let mut ch_addr1: u32 = 0x18000u32.wrapping_add(start_addr.wrapping_mul(2));
    cursor_offset = 0x18000u32.wrapping_add(start_addr.wrapping_add(cursor_offset).wrapping_mul(2));

    let x1: c_int = (*fb_dev).width.wrapping_sub(width1) / 2;
    let y1: c_int = (*fb_dev).height.wrapping_sub(height1) / 2;

    let mut cy: c_int = 0;
    while cy < height {
        let mut ch_addr: u32 = ch_addr1;
        let mut dst: *mut u8 = (*fb_dev).fb_data.offset(
            y1.wrapping_add(cy.wrapping_mul(cheight))
                .wrapping_mul((*fb_dev).stride)
                .wrapping_add(x1.wrapping_mul(4)) as isize,
        );
        let mut cx_min: c_int = width;
        let mut cx_max: c_int = -1;
        let mut cx: c_int = 0;
        while cx < width {
            let ch_attr: u32 = core::ptr::read_unaligned(
                vga_ram.add((ch_addr & 0x1fffe) as usize) as *const u16
            ) as u32;
            let idx = cy.wrapping_mul(width).wrapping_add(cx) as usize;
            if full_update != 0 || ch_attr != *(*s).last_ch_attr.as_ptr().add(idx) as u32 {
                *(*s).last_ch_attr.as_mut_ptr().add(idx) = ch_attr as u16;
                cx_min = min_int(cx_min, cx);
                cx_max = max_int(cx_max, cx);
                let ch: u32 = ch_attr & 0xff;
                let cattr: u32 = ch_attr >> 8;
                let font_ptr: *const u8 = vga_ram.add(32usize.wrapping_mul(ch as usize));
                let bgcol: u32 = *(*s).last_palette.as_ptr().add((cattr >> 4) as usize);
                let fgcol: u32 = *(*s).last_palette.as_ptr().add((cattr & 0x0f) as usize);
                if cwidth == 8 {
                    vga_draw_glyph8(dst, (*fb_dev).stride, font_ptr, cheight, fgcol, bgcol);
                } else {
                    let mut dup9: c_int = 0;
                    if ch >= 0xb0 && ch <= 0xdf && ((*s).ar[0x10] & 0x04) != 0 {
                        dup9 = 1;
                    }
                    vga_draw_glyph9(dst, (*fb_dev).stride, font_ptr, cheight, fgcol, bgcol, dup9);
                }

                // cursor display
                if cursor_offset == ch_addr && (cursor_start & 0x20) == 0 {
                    let line_start: c_int = (cursor_start & 0x1f) as c_int;
                    let line_last: c_int = min_int((cursor_end & 0x1f) as c_int, cheight - 1);
                    if line_last >= line_start && line_start < cheight {
                        let h: c_int = line_last.wrapping_sub(line_start).wrapping_add(1);
                        let dst1: *mut u8 =
                            dst.offset((*fb_dev).stride.wrapping_mul(line_start) as isize);
                        if cwidth == 8 {
                            vga_draw_glyph8(
                                dst1,
                                (*fb_dev).stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                            );
                        } else {
                            vga_draw_glyph9(
                                dst1,
                                (*fb_dev).stride,
                                CURSOR_GLYPH.as_ptr(),
                                h,
                                fgcol,
                                bgcol,
                                1,
                            );
                        }
                    }
                }
            }
            ch_addr = ch_addr.wrapping_add(2);
            dst = dst.offset((4 * cwidth) as isize);
            cx += 1;
        }
        if cx_max >= cx_min {
            if let Some(f) = redraw_func {
                f(
                    fb_dev,
                    opaque,
                    x1.wrapping_add(cx_min.wrapping_mul(cwidth)),
                    y1.wrapping_add(cy.wrapping_mul(cheight)),
                    cx_max
                        .wrapping_sub(cx_min)
                        .wrapping_add(1)
                        .wrapping_mul(cwidth),
                    cheight,
                );
            }
        }
        ch_addr1 = ch_addr1.wrapping_add(line_offset);
        cy += 1;
    }
}

unsafe extern "C" fn vga_refresh(
    fb_dev: *mut FBDevice,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
) {
    let s = (*fb_dev).device_opaque as *mut VGAState;

    if ((*s).ar_index & 0x20) == 0 {
        // blank
    } else if ((*s).gr[0x06] & 1) != 0 {
        // graphic mode (VBE)
        simplefb_refresh(
            fb_dev,
            redraw_func,
            opaque,
            (*s).mem_range,
            (*s).fb_page_count,
        );
    } else {
        // text mode
        vga_text_refresh(s, redraw_func, opaque);
    }
}

unsafe fn vga_ioport_read(s: *mut VGAState, addr: u32) -> u32 {
    let val: c_int;

    // check port range access depending on color/monochrome mode
    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & MSR_COLOR_EMULATION) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & MSR_COLOR_EMULATION) == 0)
    {
        val = 0xff;
    } else {
        match addr {
            0x3c0 => {
                if (*s).ar_flip_flop == 0 {
                    val = (*s).ar_index as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c1 => {
                let index = ((*s).ar_index & 0x1f) as usize;
                if index < 21 {
                    val = *(*s).ar.as_ptr().add(index) as c_int;
                } else {
                    val = 0;
                }
            }
            0x3c2 => val = (*s).st00 as c_int,
            0x3c4 => val = (*s).sr_index as c_int,
            0x3c5 => val = *(*s).sr.as_ptr().add((*s).sr_index as usize) as c_int,
            0x3c7 => val = (*s).dac_state as c_int,
            0x3c8 => val = (*s).dac_write_index as c_int,
            0x3c9 => {
                val = *(*s)
                    .palette
                    .as_ptr()
                    .add((*s).dac_read_index as usize * 3 + (*s).dac_sub_index as usize)
                    as c_int;
                let sub = (*s).dac_sub_index.wrapping_add(1);
                (*s).dac_sub_index = sub;
                if sub == 3 {
                    (*s).dac_sub_index = 0;
                    (*s).dac_read_index = (*s).dac_read_index.wrapping_add(1);
                }
            }
            0x3ca => val = (*s).fcr as c_int,
            0x3cc => val = (*s).msr as c_int,
            0x3ce => val = (*s).gr_index as c_int,
            0x3cf => val = *(*s).gr.as_ptr().add((*s).gr_index as usize) as c_int,
            0x3b4 | 0x3d4 => val = (*s).cr_index as c_int,
            0x3b5 | 0x3d5 => val = *(*s).cr.as_ptr().add((*s).cr_index as usize) as c_int,
            0x3ba | 0x3da => {
                // just toggle to fool polling
                (*s).st01 ^= ST01_V_RETRACE | ST01_DISP_ENABLE;
                val = (*s).st01 as c_int;
                (*s).ar_flip_flop = 0;
            }
            _ => val = 0x00,
        }
    }
    val as u32
}

unsafe fn vga_ioport_write(s: *mut VGAState, addr: u32, val: u32) {
    // check port range access depending on color/monochrome mode
    if (addr >= 0x3b0 && addr <= 0x3bf && ((*s).msr & MSR_COLOR_EMULATION) != 0)
        || (addr >= 0x3d0 && addr <= 0x3df && ((*s).msr & MSR_COLOR_EMULATION) == 0)
    {
        return;
    }

    match addr {
        0x3c0 => {
            if (*s).ar_flip_flop == 0 {
                (*s).ar_index = (val & 0x3f) as u8;
            } else {
                let index = ((*s).ar_index & 0x1f) as usize;
                let dst = (*s).ar.as_mut_ptr().add(index);
                match index {
                    0x00..=0x0f => *dst = (val & 0x3f) as u8,
                    0x10 => *dst = (val & !0x10u32) as u8,
                    0x11 => *dst = val as u8,
                    0x12 => *dst = (val & !0xc0u32) as u8,
                    0x13 => *dst = (val & !0xf0u32) as u8,
                    0x14 => *dst = (val & !0xf0u32) as u8,
                    _ => {}
                }
            }
            (*s).ar_flip_flop ^= 1;
        }
        0x3c2 => {
            (*s).msr = (val & !0x10u32) as u8;
        }
        0x3c4 => {
            (*s).sr_index = (val & 7) as u8;
        }
        0x3c5 => {
            let i = (*s).sr_index as usize;
            let m = *SR_MASK.as_ptr().add(i) as u32;
            *(*s).sr.as_mut_ptr().add(i) = (val & m) as u8;
        }
        0x3c7 => {
            (*s).dac_read_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 3;
        }
        0x3c8 => {
            (*s).dac_write_index = val as u8;
            (*s).dac_sub_index = 0;
            (*s).dac_state = 0;
        }
        0x3c9 => {
            let sub = (*s).dac_sub_index as usize;
            *(*s).dac_cache.as_mut_ptr().add(sub) = val as u8;
            let nsub = (*s).dac_sub_index.wrapping_add(1);
            (*s).dac_sub_index = nsub;
            if nsub == 3 {
                let off = (*s).dac_write_index as usize * 3;
                core::ptr::copy_nonoverlapping(
                    (*s).dac_cache.as_ptr(),
                    (*s).palette.as_mut_ptr().add(off),
                    3,
                );
                (*s).dac_sub_index = 0;
                (*s).dac_write_index = (*s).dac_write_index.wrapping_add(1);
            }
        }
        0x3ce => {
            (*s).gr_index = (val & 0x0f) as u8;
        }
        0x3cf => {
            let i = (*s).gr_index as usize;
            let m = *GR_MASK.as_ptr().add(i) as u32;
            *(*s).gr.as_mut_ptr().add(i) = (val & m) as u8;
        }
        0x3b4 | 0x3d4 => {
            (*s).cr_index = val as u8;
        }
        0x3b5 | 0x3d5 => {
            // handle CR0-7 protection
            if (*(*s).cr.as_ptr().add(0x11) & 0x80) != 0 && (*s).cr_index <= 7 {
                // can always write bit 4 of CR7
                if (*s).cr_index == 7 {
                    let c7 = *(*s).cr.as_ptr().add(7);
                    *(*s).cr.as_mut_ptr().add(7) = (c7 & !0x10u8) | ((val & 0x10) as u8);
                }
                return;
            }
            *(*s).cr.as_mut_ptr().add((*s).cr_index as usize) = val as u8;
        }
        0x3ba | 0x3da => {
            (*s).fcr = (val & 0x10) as u8;
        }
        _ => {}
    }
}

unsafe extern "C" fn vga_read_0x3c0(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3c0u32.wrapping_add(addr))
}

unsafe extern "C" fn vga_write_0x3c0(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(opaque as *mut VGAState, 0x3c0u32.wrapping_add(addr), val)
}

unsafe extern "C" fn vga_read_0x3b4(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3b4u32.wrapping_add(addr))
}

unsafe extern "C" fn vga_write_0x3b4(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(opaque as *mut VGAState, 0x3b4u32.wrapping_add(addr), val)
}

unsafe extern "C" fn vga_read_0x3d4(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3d4u32.wrapping_add(addr))
}

unsafe extern "C" fn vga_write_0x3d4(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(opaque as *mut VGAState, 0x3d4u32.wrapping_add(addr), val)
}

unsafe extern "C" fn vga_read_0x3ba(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3bau32.wrapping_add(addr))
}

unsafe extern "C" fn vga_write_0x3ba(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(opaque as *mut VGAState, 0x3bau32.wrapping_add(addr), val)
}

unsafe extern "C" fn vga_read_0x3da(opaque: *mut c_void, addr: u32, _size_log2: c_int) -> u32 {
    vga_ioport_read(opaque as *mut VGAState, 0x3dau32.wrapping_add(addr))
}

unsafe extern "C" fn vga_write_0x3da(
    opaque: *mut c_void,
    addr: u32,
    val: u32,
    _size_log2: c_int,
) {
    vga_ioport_write(opaque as *mut VGAState, 0x3dau32.wrapping_add(addr), val)
}

unsafe extern "C" fn vbe_write(opaque: *mut c_void, offset: u32, val: u32, _size_log2: c_int) {
    let s = opaque as *mut VGAState;
    let fb_dev = (*s).fb_dev;

    if offset == 0 {
        (*s).vbe_index = val as u16;
    } else {
        let index = (*s).vbe_index as usize;
        match index {
            0x0 => {
                if val >= 0xB0C0 && val <= 0xB0C5 {
                    *(*s).vbe_regs.as_mut_ptr().add(index) = val as u16;
                }
            }
            0x4 => {
                if (val & VBE_DISPI_ENABLED) != 0
                    && ((*s).vbe_regs[0x4] & 1) == 0
                {
                    // set graphic mode (resolution change not really supported)
                    let xres: c_int = (*s).vbe_regs[0x1] as c_int;
                    let yres: c_int = (*s).vbe_regs[0x2] as c_int;
                    if xres <= 4096
                        && yres <= 4096
                        && xres.wrapping_mul(4).wrapping_mul(yres) <= (*fb_dev).fb_size
                    {
                        (*fb_dev).width = xres;
                        (*fb_dev).height = yres;
                        (*fb_dev).stride = (*fb_dev).width.wrapping_mul(4);
                    }
                    (*s).vbe_regs[0x6] = (*s).vbe_regs[0x1];
                    (*s).vbe_regs[0x7] = (*s).vbe_regs[0x2];
                    (*s).vbe_regs[0x8] = 0;
                    (*s).vbe_regs[0x9] = 0;
                }
                (*s).vbe_regs[0x4] = val as u16;
            }
            0x1 | 0x2 | 0x3 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 => {
                *(*s).vbe_regs.as_mut_ptr().add(index) = val as u16;
            }
            _ => {}
        }
    }
}

unsafe extern "C" fn vbe_read(opaque: *mut c_void, offset: u32, _size_log2: c_int) -> u32 {
    let s = opaque as *mut VGAState;

    if offset == 0 {
        return (*s).vbe_index as u32;
    }
    if ((*s).vbe_regs[0x4] & VBE_DISPI_GETCAPS) != 0 {
        match (*s).vbe_index {
            0x1 => return (*fb_width(s)) as u32,
            0x2 => return (*fb_height(s)) as u32,
            0x3 => return 32,
            _ => {}
        }
    }
    // read_reg:
    let index = (*s).vbe_index as usize;
    if index < VBE_DISPI_INDEX_NB {
        *(*s).vbe_regs.as_ptr().add(index) as u32
    } else {
        0
    }
}

#[inline]
unsafe fn fb_width(s: *mut VGAState) -> *const c_int {
    core::ptr::addr_of!((*s).fb_dev) as *const c_int
}

#[inline]
unsafe fn fb_height(s: *mut VGAState) -> *const c_int {
    (core::ptr::addr_of!((*s).fb_dev) as *const c_int).add(1)
}

unsafe extern "C" fn simplefb_bar_set(
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: BOOL,
) {
    let s = opaque as *mut VGAState;
    if bar_num == 0 {
        phys_mem_set_addr((*s).mem_range, addr as u64, enabled);
    } else {
        phys_mem_set_addr((*s).rom_range, addr as u64, enabled);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pci_vga_init(
    bus: *mut PCIBus,
    fb_dev: *mut FBDevice,
    width: c_int,
    height: c_int,
    vga_rom_buf: *const u8,
    vga_rom_size: c_int,
) -> *mut VGAState {
    let d: *mut PCIDevice = pci_register_device(
        bus,
        b"VGA\0".as_ptr() as *const c_char,
        -1,
        0x1234,
        0x1111,
        0x00,
        0x0300,
    );

    let mem_map: *mut PhysMemoryMap = pci_device_get_mem_map(d);
    let port_map: *mut PhysMemoryMap = pci_device_get_port_map(d);

    let s: *mut VGAState = mallocz(core::mem::size_of::<VGAState>()) as *mut VGAState;
    (*s).fb_dev = fb_dev;

    (*fb_dev).width = width;
    (*fb_dev).height = height;
    (*fb_dev).stride = width.wrapping_mul(4);

    (*fb_dev).fb_size = height
        .wrapping_mul((*fb_dev).stride)
        .wrapping_add(FB_ALLOC_ALIGN - 1)
        & !(FB_ALLOC_ALIGN - 1);
    (*s).fb_page_count = (*fb_dev).fb_size >> 12;

    (*s).mem_range = cpu_register_ram(
        mem_map,
        0,
        (*fb_dev).fb_size as u64,
        DEVRAM_FLAG_DIRTY_BITS | DEVRAM_FLAG_DISABLED,
    );

    (*fb_dev).fb_data = (*(*s).mem_range).phys_mem;

    (*s).pci_dev = d;
    let mut bar_size: u32 = 1;
    while bar_size < (*fb_dev).fb_size as u32 {
        bar_size <<= 1;
    }
    pci_register_bar(
        d,
        0,
        bar_size,
        0x00,
        s as *mut c_void,
        Some(simplefb_bar_set),
    );

    if vga_rom_size > 0 {
        let rom_size: c_int =
            vga_rom_size.wrapping_add(DEVRAM_PAGE_SIZE - 1) & !(DEVRAM_PAGE_SIZE - 1);
        (*s).rom_range = cpu_register_ram(
            mem_map,
            0,
            rom_size as u64,
            DEVRAM_FLAG_ROM | DEVRAM_FLAG_DISABLED,
        );
        core::ptr::copy_nonoverlapping(
            vga_rom_buf,
            (*(*s).rom_range).phys_mem,
            vga_rom_size as usize,
        );

        bar_size = 1;
        while bar_size < rom_size as u32 {
            bar_size <<= 1;
        }
        pci_register_bar(
            d,
            6,
            bar_size,
            0x00,
            s as *mut c_void,
            Some(simplefb_bar_set),
        );
    }

    // VGA memory (for simple text mode no need for callbacks)
    (*s).mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0);
    (*s).vga_ram = (*(*s).mem_range2).phys_mem;

    // standard VGA ports
    cpu_register_device(
        port_map,
        0x3c0,
        16,
        s as *mut c_void,
        Some(vga_read_0x3c0),
        Some(vga_write_0x3c0),
        DEVIO_SIZE8,
    );
    cpu_register_device(
        port_map,
        0x3b4,
        2,
        s as *mut c_void,
        Some(vga_read_0x3b4),
        Some(vga_write_0x3b4),
        DEVIO_SIZE8,
    );
    cpu_register_device(
        port_map,
        0x3d4,
        2,
        s as *mut c_void,
        Some(vga_read_0x3d4),
        Some(vga_write_0x3d4),
        DEVIO_SIZE8,
    );
    cpu_register_device(
        port_map,
        0x3ba,
        1,
        s as *mut c_void,
        Some(vga_read_0x3ba),
        Some(vga_write_0x3ba),
        DEVIO_SIZE8,
    );
    cpu_register_device(
        port_map,
        0x3da,
        1,
        s as *mut c_void,
        Some(vga_read_0x3da),
        Some(vga_write_0x3da),
        DEVIO_SIZE8,
    );

    // VBE extension
    cpu_register_device(
        port_map,
        0x1ce,
        2,
        s as *mut c_void,
        Some(vbe_read),
        Some(vbe_write),
        DEVIO_SIZE16,
    );

    (*s).vbe_regs[0x0] = 0xB0C5;
    (*s).vbe_regs[0xa] = ((*fb_dev).fb_size >> 16) as u16;

    (*fb_dev).device_opaque = s as *mut c_void;
    (*fb_dev).refresh = Some(vga_refresh);
    s
}
}

// ── `VGAState *s; PCIDevice *d; uint32_t bar_size; PhysMemoryMap *mem_map, *port_map; d = pci_regist…` · vga.c:729-756 · chunk 3685 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VGAState *s;
//         PCIDevice *d;
//         uint32_t bar_size;
//         PhysMemoryMap *mem_map, *port_map;
//     
//         d = pci_register_device(bus, "VGA", -1, 0x1234, 0x1111, 0x00, 0x0300);
//     
//         mem_map = pci_device_get_mem_map(d);
//         port_map = pci_device_get_port_map(d);
//     
//         s = mallocz(sizeof(*s));
//         s->fb_dev = fb_dev;
//     
//         fb_dev->width = width;
//         fb_dev->height = height;
//         fb_dev->stride = width * 4;
//     
//         fb_dev->fb_size = (height * fb_dev->stride + (1 << 20) - 1) & ~((1 << 20) - 1);
//         s->fb_page_count = fb_dev->fb_size >> 12;
//     
//         s->mem_range =
//             cpu_register_ram(mem_map, 0, fb_dev->fb_size,
//                              (1 << 1) | (1 << 2));
//     
//         fb_dev->fb_data = s->mem_range->phys_mem;
//     
//         s->pci_dev = d;
//         bar_size = 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3685 {
// Partial port of pci_vga_init: device setup up to (but excluding) the
// pci_register_bar() call for BAR 0.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
}

// Mirror of C PhysMemoryRange (typedef struct {...} PhysMemoryRange).
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

// Mirror of C struct PhysMemoryMap.
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

// Mirror of C struct FBDevice.
#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, *mut c_void)>,
}

// Mirror of C struct VGAState (needed in full because sizeof(*s) is used).
#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut PCIDevice,
    pub rom_range: *mut PhysMemoryRange,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
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
    fn pci_device_get_mem_map(d: *mut PCIDevice) -> *mut PhysMemoryMap;
    fn pci_device_get_port_map(d: *mut PCIDevice) -> *mut PhysMemoryMap;
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn pci_vga_init__part3685(
    s_out: *mut *mut VGAState,
    d_out: *mut *mut PCIDevice,
    bar_size_out: *mut u32,
    mem_map_out: *mut *mut PhysMemoryMap,
    port_map_out: *mut *mut PhysMemoryMap,
    bus: *mut PCIBus,
    fb_dev: *mut FBDevice,
    width: c_int,
    height: c_int,
) {
    let d = pci_register_device(
        bus,
        b"VGA\0".as_ptr() as *const c_char,
        -1,
        0x1234,
        0x1111,
        0x00,
        0x0300,
    );
    *d_out = d;

    let mem_map = pci_device_get_mem_map(d);
    *mem_map_out = mem_map;
    let port_map = pci_device_get_port_map(d);
    *port_map_out = port_map;
    let _ = port_map;

    let s = mallocz(core::mem::size_of::<VGAState>()) as *mut VGAState;
    *s_out = s;
    (*s).fb_dev = fb_dev;

    (*fb_dev).width = width;
    (*fb_dev).height = height;
    let stride = width.wrapping_mul(4);
    (*fb_dev).stride = stride;

    let fb_size = height
        .wrapping_mul(stride)
        .wrapping_add((1 << 20) - 1)
        & !((1 << 20) - 1);
    (*fb_dev).fb_size = fb_size;
    (*s).fb_page_count = fb_size >> 12;

    // cpu_register_ram(mem_map, 0, fb_dev->fb_size, (1 << 1) | (1 << 2))
    let size = fb_size as i64 as u64;
    let mem_range = match (*mem_map).register_ram {
        Some(register_ram) => register_ram(mem_map, 0, size, (1 << 1) | (1 << 2)),
        None => core::ptr::null_mut(),
    };
    (*s).mem_range = mem_range;

    if !mem_range.is_null() {
        (*fb_dev).fb_data = (*mem_range).phys_mem;
    }

    (*s).pci_dev = d;

    *bar_size_out = 1;
}
}

// ── `while (bar_size < fb_dev->fb_size)` · vga.c:757-758 · chunk 3686 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (bar_size < fb_dev->fb_size)
//             bar_size <<= 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3686 {
// Port of the bar_size rounding loop from pci_vga_init.

#[repr(C)]
pub struct FBDevice {
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
    pub fb_data: *mut u8,
    pub fb_size: core::ffi::c_int,
    pub device_opaque: *mut core::ffi::c_void,
    pub refresh: Option<
        unsafe extern "C" fn(
            fb_dev: *mut FBDevice,
            redraw_func: Option<
                unsafe extern "C" fn(
                    fb_dev: *mut FBDevice,
                    opaque: *mut core::ffi::c_void,
                    x: core::ffi::c_int,
                    y: core::ffi::c_int,
                    w: core::ffi::c_int,
                    h: core::ffi::c_int,
                ),
            >,
            opaque: *mut core::ffi::c_void,
        ),
    >,
}

#[no_mangle]
pub unsafe extern "C" fn pci_vga_init__part3686(
    fb_dev: *mut FBDevice,
    mut bar_size: u32,
) -> u32 {
    let fb_size = (*fb_dev).fb_size as u32;
    while bar_size < fb_size {
        bar_size <<= 1;
    }
    bar_size
}
}

// ── `pci_register_bar(d, 0, bar_size, 0x00, s, simplefb_bar_set);` · vga.c:759-760 · chunk 3687 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pci_register_bar(d, 0, bar_size, 0x00, s,
//                          simplefb_bar_set);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3687 {
// Port of the pci_register_bar(...) call for BAR 0 in pci_vga_init (vga.c).
// The C bar-set callback (simplefb_bar_set) is a static function without a
// linker symbol, so its logic is ported here as a private Rust fn and its
// address is handed to pci_register_bar instead.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
struct VGAState {
    fb_dev: *mut c_void,
    fb_page_count: c_int,
    mem_range: *mut c_void,
    mem_range2: *mut c_void,
    pci_dev: *mut c_void,
    rom_range: *mut c_void,

    vga_ram: *mut u8,

    sr_index: u8,
    sr: [u8; 8],
    gr_index: u8,
    gr: [u8; 16],
    ar_index: u8,
    ar: [u8; 21],
    ar_flip_flop: c_int,
    cr_index: u8,
    cr: [u8; 256],
    msr: u8,
    fcr: u8,
    st00: u8,
    st01: u8,
    dac_state: u8,
    dac_sub_index: u8,
    dac_read_index: u8,
    dac_write_index: u8,
    dac_cache: [u8; 3],
    palette: [u8; 768],

    last_palette: [u32; 16],
    last_ch_attr: [u16; 132 * 60],
    last_width: u32,
    last_height: u32,
    last_line_offset: u16,
    last_start_addr: u16,
    last_cursor_offset: u16,
    last_cursor_start: u8,
    last_cursor_end: u8,

    vbe_index: u16,
    vbe_regs: [u16; 0xb],
}

type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

extern "C" {
    fn pci_register_bar(
        d: *mut c_void,
        bar_num: c_uint,
        size: u32,
        type_: c_int,
        opaque: *mut c_void,
        bar_set: Option<PCIBarSetFunc>,
    );
    fn phys_mem_set_addr(pr: *mut c_void, addr: u64, enabled: c_int);
}

// Ported logic of the static C function simplefb_bar_set.
unsafe extern "C" fn pci_vga_init__part3687_bar_set(
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: c_int,
) {
    let s = opaque as *mut VGAState;
    if bar_num == 0 {
        phys_mem_set_addr((*s).mem_range, addr as u64, enabled);
    } else {
        phys_mem_set_addr((*s).rom_range, addr as u64, enabled);
    }
}

#[no_mangle]
pub extern "C" fn pci_vga_init__part3687(d: *mut c_void, bar_size: u32, s: *mut c_void) {
    unsafe {
        pci_register_bar(
            d,
            0,
            bar_size,
            0x00,
            s,
            Some(pci_vga_init__part3687_bar_set),
        );
    }
}
}

// ── `if (vga_rom_size > 0)` · vga.c:762-775 · chunk 3688 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vga_rom_size > 0) {
//             int rom_size;
//     
//             rom_size = (vga_rom_size + (1 << 12) - 1) & ~((1 << 12) - 1);
//             s->rom_range = cpu_register_ram(mem_map, 0, rom_size,
//                                             (1 << 0) | (1 << 2));
//             memcpy(s->rom_range->phys_mem, vga_rom_buf, vga_rom_size);
//     
//             bar_size = 1;
//             while (bar_size < rom_size)
//                 bar_size <<= 1;
//             pci_register_bar(d, 6, bar_size, 0x00, s,
//                              simplefb_bar_set);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3688 {
// Ported piece of pci_vga_init(): optional VGA ROM BAR registration.

use core::ffi::{c_int, c_uint, c_void};

pub type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct PCIDevice {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: Option<unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int)>,
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
    pub register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut PCIDevice,
    pub rom_range: *mut PhysMemoryRange,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

unsafe extern "C" {
    fn pci_register_bar(
        d: *mut PCIDevice,
        bar_num: c_uint,
        size: u32,
        r#type: c_int,
        opaque: *mut c_void,
        bar_set: Option<PCIBarSetFunc>,
    );
    fn phys_mem_set_addr(pr: *mut PhysMemoryRange, addr: u64, enabled: c_int);
}

// Ported body of the static C function simplefb_bar_set (no linker symbol).
unsafe extern "C" fn simplefb_bar_set(
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: c_int,
) {
    let s = opaque as *mut VGAState;
    if bar_num == 0 {
        phys_mem_set_addr((*s).mem_range, addr as u64, enabled);
    } else {
        phys_mem_set_addr((*s).rom_range, addr as u64, enabled);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pci_vga_init__part3688(
    vga_rom_size: c_int,
    s: *mut VGAState,
    mem_map: *mut PhysMemoryMap,
    vga_rom_buf: *const u8,
    d: *mut PCIDevice,
) {
    if vga_rom_size > 0 {
        let rom_size: c_int = (vga_rom_size + ((1 << 12) - 1)) & !((1 << 12) - 1);

        // cpu_register_ram(mem_map, 0, rom_size, (1 << 0) | (1 << 2))
        let register_ram = match (*mem_map).register_ram {
            Some(f) => f,
            None => return,
        };
        let rom_range = register_ram(mem_map, 0, rom_size as i64 as u64, (1 << 0) | (1 << 2));
        (*s).rom_range = rom_range;

        // memcpy(s->rom_range->phys_mem, vga_rom_buf, vga_rom_size)
        core::ptr::copy_nonoverlapping(
            vga_rom_buf,
            (*rom_range).phys_mem,
            vga_rom_size as usize,
        );

        let mut bar_size: u32 = 1;
        let rom_size_u: u32 = rom_size as u32;
        while bar_size < rom_size_u {
            bar_size = bar_size.wrapping_shl(1);
        }
        pci_register_bar(d, 6, bar_size, 0x00, s as *mut c_void, Some(simplefb_bar_set));
    }
}
}

// ── `s->mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0); s->vga_ram = s->mem_range2->phy…` · vga.c:778-803 · chunk 3689 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0);
//         s->vga_ram = s->mem_range2->phys_mem;
//     
//     
//     
//         cpu_register_device(port_map, 0x3c0, 16, s, vga_read_0x3c0, vga_write_0x3c0,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3b4, 2, s, vga_read_0x3b4, vga_write_0x3b4,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3d4, 2, s, vga_read_0x3d4, vga_write_0x3d4,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3ba, 1, s, vga_read_0x3ba, vga_write_0x3ba,
//                             (1 << 0));
//         cpu_register_device(port_map, 0x3da, 1, s, vga_read_0x3da, vga_write_0x3da,
//                             (1 << 0));
//     
//     
//         cpu_register_device(port_map, 0x1ce, 2, s, vbe_read, vbe_write,
//                             (1 << 1));
//     
//         s->vbe_regs[0x0] = 0xB0C5;
//         s->vbe_regs[0xa] = fb_dev->fb_size >> 16;
//     
//         fb_dev->device_opaque = s;
//         fb_dev->refresh = vga_refresh;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3689 {
// Port of the tail of pci_vga_init (vga.c lines 778-803).
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

// void DeviceWriteFunc(void *opaque, uint32_t offset, uint32_t val, int size_log2);
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
// uint32_t DeviceReadFunc(void *opaque, uint32_t offset, int size_log2);
pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;

// typedef void SimpleFBDrawFunc(FBDevice *fb_dev, void *opaque,
//                               int x, int y, int w, int h);
pub type SimpleFBDrawFunc =
    unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int);

// void (*refresh)(struct FBDevice *fb_dev, SimpleFBDrawFunc *redraw_func, void *opaque);
pub type RefreshFunc = unsafe extern "C" fn(*mut FBDevice, SimpleFBDrawFunc, *mut c_void);

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
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
pub struct FBDevice {
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub fb_data: *mut u8,
    pub fb_size: c_int,
    pub device_opaque: *mut c_void,
    pub refresh: RefreshFunc,
}

#[repr(C)]
pub struct VGAState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
    pub mem_range2: *mut PhysMemoryRange,
    pub pci_dev: *mut PCIDevice,
    pub rom_range: *mut PhysMemoryRange,
    pub vga_ram: *mut u8,
    pub sr_index: u8,
    pub sr: [u8; 8],
    pub gr_index: u8,
    pub gr: [u8; 16],
    pub ar_index: u8,
    pub ar: [u8; 21],
    pub ar_flip_flop: c_int,
    pub cr_index: u8,
    pub cr: [u8; 256],
    pub msr: u8,
    pub fcr: u8,
    pub st00: u8,
    pub st01: u8,
    pub dac_state: u8,
    pub dac_sub_index: u8,
    pub dac_read_index: u8,
    pub dac_write_index: u8,
    pub dac_cache: [u8; 3],
    pub palette: [u8; 768],
    pub last_palette: [u32; 16],
    pub last_ch_attr: [u16; 132 * 60],
    pub last_width: u32,
    pub last_height: u32,
    pub last_line_offset: u16,
    pub last_start_addr: u16,
    pub last_cursor_offset: u16,
    pub last_cursor_start: u8,
    pub last_cursor_end: u8,
    pub vbe_index: u16,
    pub vbe_regs: [u16; 0xb],
}

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
pub unsafe extern "C" fn pci_vga_init__part3689(
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    s: *mut VGAState,
    fb_dev: *mut FBDevice,
    register_ram: unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange,
    read_3c0: DeviceReadFunc,
    write_3c0: DeviceWriteFunc,
    read_3b4: DeviceReadFunc,
    write_3b4: DeviceWriteFunc,
    read_3d4: DeviceReadFunc,
    write_3d4: DeviceWriteFunc,
    read_3ba: DeviceReadFunc,
    write_3ba: DeviceWriteFunc,
    read_3da: DeviceReadFunc,
    write_3da: DeviceWriteFunc,
    vbe_read_fn: DeviceReadFunc,
    vbe_write_fn: DeviceWriteFunc,
    refresh_fn: RefreshFunc,
) -> *mut VGAState {
    // s->mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0);
    let mem_range2 = register_ram(mem_map, 0xa0000, 0x20000, 0);
    (*s).mem_range2 = mem_range2;
    // s->vga_ram = s->mem_range2->phys_mem;
    (*s).vga_ram = (*mem_range2).phys_mem;

    let opaque = s as *mut c_void;

    cpu_register_device(port_map, 0x3c0, 16, opaque, read_3c0, write_3c0, 1);
    cpu_register_device(port_map, 0x3b4, 2, opaque, read_3b4, write_3b4, 1);
    cpu_register_device(port_map, 0x3d4, 2, opaque, read_3d4, write_3d4, 1);
    cpu_register_device(port_map, 0x3ba, 1, opaque, read_3ba, write_3ba, 1);
    cpu_register_device(port_map, 0x3da, 1, opaque, read_3da, write_3da, 1);

    cpu_register_device(port_map, 0x1ce, 2, opaque, vbe_read_fn, vbe_write_fn, (1 << 1));

    // s->vbe_regs[0x0] = 0xB0C5;
    (*s).vbe_regs[0x0] = 0xB0C5;
    // s->vbe_regs[0xa] = fb_dev->fb_size >> 16;
    (*s).vbe_regs[0xa] = ((*fb_dev).fb_size >> 16) as u16;

    // fb_dev->device_opaque = s;
    (*fb_dev).device_opaque = opaque;
    // fb_dev->refresh = vga_refresh;
    (*fb_dev).refresh = refresh_fn;

    s
}
}

