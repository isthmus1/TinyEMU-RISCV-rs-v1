//! From `simplefb.c`.

// ── `const uint32_t *dirty_bits; uint32_t dirty_val; int y0, y1, page_y0, page_y1, byte_pos, page_in…` · simplefb.c:53-60 · chunk 2175 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const uint32_t *dirty_bits;
//         uint32_t dirty_val;
//         int y0, y1, page_y0, page_y1, byte_pos, page_index, bit_pos;
//     
//         dirty_bits = phys_mem_get_dirty_bits(mem_range);
//     
//         page_index = 0;
//         y0 = y1 = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2175 {
// Ported piece of simplefb_refresh: initializes the dirty-bit scan state.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut PhysMemoryMap,
    addr: u64,
    org_size: u64,
    size: u64,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *mut u8,
    dirty_bits_size: c_int,
    dirty_bits: *mut u32,
    dirty_bits_tab: [*mut u32; 2],
    dirty_bits_index: c_int,
    opaque: *mut c_void,
    read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    n_phys_mem_range: c_int,
    phys_mem_range: [PhysMemoryRange; 32],
    register_ram:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    get_dirty_bits: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32,
    >,
    set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    opaque: *mut c_void,
    flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

// Inlined equivalent of the C `phys_mem_get_dirty_bits` (static inline).
unsafe fn phys_mem_get_dirty_bits(pr: *mut PhysMemoryRange) -> *const u32 {
    if pr.is_null() {
        return core::ptr::null();
    }
    let map = (*pr).map;
    if map.is_null() {
        return core::ptr::null();
    }
    match (*map).get_dirty_bits {
        Some(f) => f(map, pr),
        None => core::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn simplefb_refresh__part2175(
    mem_range: *mut PhysMemoryRange,
    page_index: *mut c_int,
    y0: *mut c_int,
    y1: *mut c_int,
) -> *const u32 {
    let dirty_bits = phys_mem_get_dirty_bits(mem_range);

    if !page_index.is_null() {
        *page_index = 0;
    }
    if !y0.is_null() {
        *y0 = 0;
    }
    if !y1.is_null() {
        *y1 = 0;
    }

    dirty_bits
}
}

// ── `while (page_index < fb_page_count)` · simplefb.c:61-90 · chunk 2176 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (page_index < fb_page_count) {
//             dirty_val = dirty_bits[page_index >> 5];
//             if (dirty_val != 0) {
//                 bit_pos = 0;
//                 while (dirty_val != 0) {
//                     while (((dirty_val >> bit_pos) & 1) == 0)
//                         bit_pos++;
//                     dirty_val &= ~(1 << bit_pos);
//     
//                     byte_pos = (page_index + bit_pos) * (1 << 12);
//                     page_y0 = byte_pos / fb_dev->stride;
//                     page_y1 = ((byte_pos + (1 << 12) - 1) / fb_dev->stride) + 1;
//                     page_y1 = min_int(page_y1, fb_dev->height);
//                     if (y0 == y1) {
//                         y0 = page_y0;
//                         y1 = page_y1;
//                     } else if (page_y0 <= (y1 + 3)) {
//     
//                         y1 = page_y1;
//                     } else {
//     
//                         redraw_func(fb_dev, opaque,
//                                     0, y0, fb_dev->width, y1 - y0);
//                         y0 = page_y0;
//                         y1 = page_y1;
//                     }
//                 }
//             }
//             page_index += 32;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2176 {
// Rust port of the outer dirty-page scan loop of simplefb_refresh().
// min_int is a static inline C helper with no linker symbol: re-implemented here.

use core::ffi::{c_int, c_void};

pub type SimpleFBDrawFunc = unsafe extern "C" fn(*mut FBDevice, *mut c_void, c_int, c_int, c_int, c_int);

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

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// C integer division without the possibility of a Rust panic.
#[inline]
fn c_div(a: c_int, b: c_int) -> c_int {
    match a.checked_div(b) {
        Some(v) => v,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn simplefb_refresh__part2176(
    fb_dev: *mut FBDevice,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
    dirty_bits: *const u32,
    fb_page_count: c_int,
    page_index_start: c_int,
    y0_ptr: *mut c_int,
    y1_ptr: *mut c_int,
) {
    let mut page_index: c_int = page_index_start;
    let mut y0: c_int = *y0_ptr;
    let mut y1: c_int = *y1_ptr;

    while page_index < fb_page_count {
        let mut dirty_val: u32 = *dirty_bits.offset((page_index >> 5) as isize);
        if dirty_val != 0 {
            let mut bit_pos: c_int = 0;
            while dirty_val != 0 {
                while (dirty_val.wrapping_shr(bit_pos as u32)) & 1 == 0 {
                    bit_pos += 1;
                }
                dirty_val &= !(1u32.wrapping_shl(bit_pos as u32));

                let byte_pos: c_int = page_index.wrapping_add(bit_pos).wrapping_mul(1 << 12);
                let page_y0: c_int = c_div(byte_pos, (*fb_dev).stride);
                let mut page_y1: c_int = c_div(byte_pos.wrapping_add((1 << 12) - 1), (*fb_dev).stride)
                    .wrapping_add(1);
                page_y1 = min_int(page_y1, (*fb_dev).height);
                if y0 == y1 {
                    y0 = page_y0;
                    y1 = page_y1;
                } else if page_y0 <= y1.wrapping_add(3) {
                    y1 = page_y1;
                } else {
                    if let Some(f) = redraw_func {
                        f(fb_dev, opaque, 0, y0, (*fb_dev).width, y1.wrapping_sub(y0));
                    }
                    y0 = page_y0;
                    y1 = page_y1;
                }
            }
        }
        page_index += 32;
    }

    *y0_ptr = y0;
    *y1_ptr = y1;
}
}

// ── `if (y0 != y1)` · simplefb.c:92-95 · chunk 2177 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (y0 != y1) {
//             redraw_func(fb_dev, opaque,
//                         0, y0, fb_dev->width, y1 - y0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2177 {
// Rust port of the tail redraw of simplefb_refresh.

use core::ffi::{c_int, c_void};

pub type SimpleFBDrawFunc =
    unsafe extern "C" fn(fb_dev: *mut FBDevice, opaque: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int);

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

#[no_mangle]
pub extern "C" fn simplefb_refresh__part2177(
    fb_dev: *mut FBDevice,
    redraw_func: Option<SimpleFBDrawFunc>,
    opaque: *mut c_void,
    y0: c_int,
    y1: c_int,
) {
    if y0 != y1 {
        if let Some(f) = redraw_func {
            let width = unsafe { (*fb_dev).width };
            unsafe {
                f(fb_dev, opaque, 0, y0, width, y1 - y0);
            }
        }
    }
}
}

// ── `SimpleFBState *s = fb_dev->device_opaque; simplefb_refresh(fb_dev, redraw_func, opaque, s->mem_…` · simplefb.c:101-103 · chunk 2179 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     SimpleFBState *s = fb_dev->device_opaque;
//         simplefb_refresh(fb_dev, redraw_func, opaque, s->mem_range,
//                          s->fb_page_count);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2179 {
// Port of the body of simplefb_refresh1 (simplefb.c lines 101-103).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    // opaque from Rust's point of view: only pointers to it are used
    _private: [u8; 0],
}

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
pub struct SimpleFBState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
}

extern "C" {
    fn simplefb_refresh(
        fb_dev: *mut FBDevice,
        redraw_func: *mut c_void,
        opaque: *mut c_void,
        mem_range: *mut PhysMemoryRange,
        fb_page_count: c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn simplefb_refresh1__part2179(
    fb_dev: *mut FBDevice,
    redraw_func: *mut c_void,
    opaque: *mut c_void,
) {
    let s = (*fb_dev).device_opaque as *mut SimpleFBState;
    simplefb_refresh(
        fb_dev,
        redraw_func,
        opaque,
        (*s).mem_range,
        (*s).fb_page_count,
    );
}
}

// ── `SimpleFBState *s; s = mallocz(sizeof(*s)); s->fb_dev = fb_dev; fb_dev->width = width; fb_dev->h…` · simplefb.c:109-126 · chunk 2181 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     SimpleFBState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->fb_dev = fb_dev;
//     
//         fb_dev->width = width;
//         fb_dev->height = height;
//         fb_dev->stride = width * 4;
//         fb_dev->fb_size = (height * fb_dev->stride + 65536 - 1) & ~(65536 - 1);
//         s->fb_page_count = fb_dev->fb_size >> 12;
//     
//         s->mem_range = cpu_register_ram(map, phys_addr, fb_dev->fb_size,
//                                         (1 << 1));
//     
//         fb_dev->fb_data = s->mem_range->phys_mem;
//         fb_dev->device_opaque = s;
//         fb_dev->refresh = simplefb_refresh1;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2181 {
// Port of the init part of simplefb_init: allocates the SimpleFBState, fills
// in the FBDevice geometry, registers the RAM range and hooks up refresh.
use core::ffi::{c_int, c_void};

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

type RegisterRamFn =
    extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange;

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<RegisterRamFn>,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

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
pub struct SimpleFBState {
    pub fb_dev: *mut FBDevice,
    pub fb_page_count: c_int,
    pub mem_range: *mut PhysMemoryRange,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn simplefb_refresh(
        fb_dev: *mut FBDevice,
        redraw_func: *mut c_void,
        opaque: *mut c_void,
        mem_range: *mut PhysMemoryRange,
        fb_page_count: c_int,
    );
}

// Equivalent of the file-static simplefb_refresh1() in simplefb.c, which has
// no linker symbol; it is only reachable through FBDevice::refresh.
extern "C" fn simplefb_refresh1_trampoline(
    fb_dev: *mut FBDevice,
    redraw_func: *mut c_void,
    opaque: *mut c_void,
) {
    unsafe {
        let s = (*fb_dev).device_opaque as *mut SimpleFBState;
        simplefb_refresh(
            fb_dev,
            redraw_func,
            opaque,
            (*s).mem_range,
            (*s).fb_page_count,
        );
    }
}

#[no_mangle]
pub extern "C" fn simplefb_init__part2181(
    map: *mut PhysMemoryMap,
    phys_addr: u64,
    fb_dev: *mut FBDevice,
    width: c_int,
    height: c_int,
) -> *mut SimpleFBState {
    unsafe {
        let s = mallocz(core::mem::size_of::<SimpleFBState>()) as *mut SimpleFBState;
        (*s).fb_dev = fb_dev;

        (*fb_dev).width = width;
        (*fb_dev).height = height;
        let stride = width.wrapping_mul(4);
        (*fb_dev).stride = stride;
        let fb_size = height
            .wrapping_mul(stride)
            .wrapping_add(65536 - 1)
            & !(65536 - 1);
        (*fb_dev).fb_size = fb_size;
        (*s).fb_page_count = fb_size >> 12;

        // cpu_register_ram(map, phys_addr, size, flags) ==
        //     map->register_ram(map, phys_addr, size, flags)
        let range = match (*map).register_ram {
            Some(register_ram) => register_ram(map, phys_addr, fb_size as u64, 1 << 1),
            None => core::ptr::null_mut(),
        };
        (*s).mem_range = range;

        (*fb_dev).fb_data = if range.is_null() {
            core::ptr::null_mut()
        } else {
            (*range).phys_mem
        };
        (*fb_dev).device_opaque = s as *mut c_void;
        (*fb_dev).refresh = simplefb_refresh1_trampoline as *mut c_void;
        s
    }
}
}

