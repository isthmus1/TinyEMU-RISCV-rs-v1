//! From `iomem.h`.

// ── `cpu_register_ram` · iomem.h:87-91 · chunk 152 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline PhysMemoryRange *cpu_register_ram(PhysMemoryMap *s, uint64_t addr,
//                                       uint64_t size, int devram_flags)
//     {
//         return s->register_ram(s, addr, size, devram_flags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk152 {
// Rust rewrite of the C inline helper cpu_register_ram (iomem.h).
//
// static inline PhysMemoryRange *cpu_register_ram(PhysMemoryMap *s, uint64_t addr,
//                                                 uint64_t size, int devram_flags)
// {
//     return s->register_ram(s, addr, size, devram_flags);
// }

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

pub type RegisterRamFn =
    unsafe extern "C" fn(s: *mut PhysMemoryMap, addr: u64, size: u64, devram_flags: c_int) -> *mut PhysMemoryRange;

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

#[no_mangle]
pub unsafe extern "C" fn cpu_register_ram(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    if s.is_null() {
        return core::ptr::null_mut();
    }
    match (*s).register_ram {
        Some(register_ram) => register_ram(s, addr, size, devram_flags),
        None => core::ptr::null_mut(),
    }
}
}

// ── `return s->register_ram(s, addr, size, devram_flags);` · iomem.h:90-90 · chunk 153 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return s->register_ram(s, addr, size, devram_flags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk153 {
use core::ffi::{c_int, c_ulonglong, c_uint, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
    pub addr: c_ulonglong,
    pub org_size: c_ulonglong,
    pub size: c_ulonglong,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut c_uint,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut c_uint,
    pub dirty_bits_tab: [*mut c_uint; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: extern "C" fn(*mut c_void, c_uint, c_int) -> c_uint,
    pub write_func: extern "C" fn(*mut c_void, c_uint, c_uint, c_int),
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: extern "C" fn(*mut PhysMemoryMap, c_ulonglong, c_ulonglong, c_int) -> *mut PhysMemoryRange,
    pub free_ram: extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange),
    pub get_dirty_bits: extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const c_uint,
    pub set_ram_addr: extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, c_ulonglong, c_int),
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: extern "C" fn(*mut c_void, *mut c_uint, c_ulonglong),
}

#[no_mangle]
pub extern "C" fn cpu_register_ram__part153(
    s: *mut PhysMemoryMap,
    addr: c_ulonglong,
    size: c_ulonglong,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    unsafe {
        let result = ((*s).register_ram)(s, addr, size, devram_flags);
        result
    }
}
}

// ── `phys_mem_get_dirty_bits` · iomem.h:99-103 · chunk 154 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline const uint32_t *phys_mem_get_dirty_bits(PhysMemoryRange *pr)
//     {
//         PhysMemoryMap *map = pr->map;
//         return map->get_dirty_bits(map, pr);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk154 {
// Port of static inline phys_mem_get_dirty_bits from iomem.h
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::c_int;

pub type BOOL = c_int;

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
    pub opaque: *mut core::ffi::c_void,
    pub read_func: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange,
    >,
    pub free_ram: Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, BOOL)>,
    pub opaque: *mut core::ffi::c_void,
    pub flush_tlb_write_range:
        Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut u8, usize)>,
}

#[no_mangle]
pub unsafe extern "C" fn phys_mem_get_dirty_bits(pr: *mut PhysMemoryRange) -> *const u32 {
    let map: *mut PhysMemoryMap = (*pr).map;
    match (*map).get_dirty_bits {
        Some(f) => f(map, pr),
        None => core::ptr::null(),
    }
}
}

// ── `PhysMemoryMap *map = pr->map; return map->get_dirty_bits(map, pr);` · iomem.h:101-102 · chunk 155 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryMap *map = pr->map;
//         return map->get_dirty_bits(map, pr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk155 {
use core::ffi::c_int;
use core::ffi::c_ulonglong;
use core::ffi::c_uint;
use core::ffi::c_uchar;
use core::ffi::c_ulong;
use core::ffi::c_void;

type DeviceReadFunc = extern "C" fn(*const c_void, c_uint, c_int) -> c_uint;
type DeviceWriteFunc = extern "C" fn(*const c_void, c_uint, c_uint, c_int);
type PhysMemoryMapGetDirtyBits = extern "C" fn(*const PhysMemoryMap, *const PhysMemoryRange) -> *const c_uint;
type PhysMemoryMapSetRamAddr = extern "C" fn(*const PhysMemoryMap, *const PhysMemoryRange, c_ulonglong, c_int);
type PhysMemoryMapFlushTlb = extern "C" fn(*const c_void, *const c_uchar, c_ulong);

#[repr(C)]
struct PhysMemoryRange {
    map: *const PhysMemoryMap,
    addr: c_ulonglong,
    org_size: c_ulonglong,
    size: c_ulonglong,
    is_ram: c_int,
    devram_flags: c_int,
    phys_mem: *const c_uchar,
    dirty_bits_size: c_int,
    dirty_bits: *const c_uint,
    dirty_bits_tab: [*const c_uint; 2],
    dirty_bits_index: c_int,
    opaque: *const c_void,
    read_func: DeviceReadFunc,
    write_func: DeviceWriteFunc,
    devio_flags: c_int,
}

#[repr(C)]
struct PhysMemoryMap {
    n_phys_mem_range: c_int,
    phys_mem_range: [PhysMemoryRange; 32],
    get_dirty_bits: PhysMemoryMapGetDirtyBits,
    set_ram_addr: PhysMemoryMapSetRamAddr,
    opaque: *const c_void,
    flush_tlb_write_range: PhysMemoryMapFlushTlb,
}

#[no_mangle]
pub extern "C" fn phys_mem_get_dirty_bits__part155(map: *const PhysMemoryMap, pr: *const PhysMemoryRange) -> *const c_uint {
    unsafe {
        let map_ref = &*map;
        let get_dirty_bits = map_ref.get_dirty_bits;
        get_dirty_bits(map, pr)
    }
}
}

// ── `phys_mem_set_dirty_bit` · iomem.h:105-115 · chunk 156 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void phys_mem_set_dirty_bit(PhysMemoryRange *pr, size_t offset)
//     {
//         size_t page_index;
//         uint32_t mask, *dirty_bits_ptr;
//         if (pr->dirty_bits) {
//             page_index = offset >> 12;
//             mask = 1 << (page_index & 0x1f);
//             dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
//             *dirty_bits_ptr |= mask;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk156 {
use core::ffi::{c_int, c_uint};

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
    pub opaque: *mut core::ffi::c_void,
    pub read_func: *mut core::ffi::c_void,
    pub write_func: *mut core::ffi::c_void,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub _opaque_rest: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn phys_mem_set_dirty_bit(pr: *mut PhysMemoryRange, offset: usize) {
    if pr.is_null() {
        return;
    }
    let dirty_bits = (*pr).dirty_bits;
    if !dirty_bits.is_null() {
        let page_index: usize = offset >> 12;
        let mask: c_uint = 1u32 << ((page_index & 0x1f) as u32);
        let dirty_bits_ptr = dirty_bits.add(page_index >> 5);
        *dirty_bits_ptr |= mask;
    }
}
}

// ── `if (pr->dirty_bits)` · iomem.h:109-114 · chunk 157 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->dirty_bits) {
//             page_index = offset >> 12;
//             mask = 1 << (page_index & 0x1f);
//             dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
//             *dirty_bits_ptr |= mask;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk157 {
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut core::ffi::c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: core::ffi::c_int,
    pub devram_flags: core::ffi::c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: core::ffi::c_int,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: core::ffi::c_int,
    pub opaque: *mut core::ffi::c_void,
    pub read_func: *mut core::ffi::c_void,
    pub write_func: *mut core::ffi::c_void,
    pub devio_flags: core::ffi::c_int,
}

#[no_mangle]
pub extern "C" fn phys_mem_set_dirty_bit__part157(pr: *mut PhysMemoryRange, offset: usize) {
    if !pr.is_null() {
        unsafe {
            let dirty_bits = (*pr).dirty_bits;
            if !dirty_bits.is_null() {
                let page_index = offset >> 12;
                let mask = 1u32 << (page_index & 0x1f);
                let dirty_bits_ptr = dirty_bits.offset((page_index >> 5) as isize);
                *dirty_bits_ptr |= mask;
            }
        }
    }
}
}

// ── `phys_mem_is_dirty_bit` · iomem.h:117-126 · chunk 158 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL phys_mem_is_dirty_bit(PhysMemoryRange *pr, size_t offset)
//     {
//         size_t page_index;
//         uint32_t *dirty_bits_ptr;
//         if (!pr->dirty_bits)
//             return TRUE;
//         page_index = offset >> 12;
//         dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
//         return (*dirty_bits_ptr >> (page_index & 0x1f)) & 1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk158 {
// Port of phys_mem_is_dirty_bit from iomem.h
// BOOL phys_mem_is_dirty_bit(PhysMemoryRange *pr, size_t offset)

use core::ffi::{c_int};

type BOOL = c_int;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut core::ffi::c_void, // PhysMemoryMap *
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
    pub opaque: *mut core::ffi::c_void,
    pub read_func: *mut core::ffi::c_void,
    pub write_func: *mut core::ffi::c_void,
    pub devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn phys_mem_is_dirty_bit(pr: *mut PhysMemoryRange, offset: usize) -> BOOL {
    unsafe {
        if pr.is_null() {
            return 1;
        }
        let dirty_bits = (*pr).dirty_bits;
        if dirty_bits.is_null() {
            return 1;
        }
        let page_index: usize = offset >> 12;
        let dirty_bits_ptr: *mut u32 = dirty_bits.add(page_index >> 5);
        let value: u32 = *dirty_bits_ptr;
        let shift: u32 = (page_index & 0x1f) as u32;
        ((value >> shift) & 1) as BOOL
    }
}
}

// ── `if (!pr->dirty_bits)` · iomem.h:121-122 · chunk 159 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr->dirty_bits)
//             return TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk159 {
use std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *const c_void,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: c_int,
    pub devram_flags: c_int,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut c_uint,
    pub dirty_bits_tab: [*mut c_uint; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: extern "C" fn(*mut c_void, u32, i32) -> u32,
    pub write_func: extern "C" fn(*mut c_void, u32, u32, i32),
    pub devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn phys_mem_is_dirty_bit__part159(pr: *const PhysMemoryRange) -> c_int {
    unsafe {
        if ((*pr).dirty_bits.is_null()) {
            return 1;
        }
    }
    return 0;
}
}

// ── `page_index = offset >> 12; dirty_bits_ptr = pr->dirty_bits + (page_index >> 5); return (*dirty_…` · iomem.h:123-125 · chunk 160 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     page_index = offset >> 12;
//         dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
//         return (*dirty_bits_ptr >> (page_index & 0x1f)) & 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk160 {
use core::ffi::c_int;
use std::os::raw::c_void;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_ulonglong;

#[no_mangle]
pub extern "C" fn phys_mem_is_dirty_bit__part160(pr: *const c_void, offset: c_ulong) -> c_int {
    unsafe { 1 }
}
}

// ── `set_irq` · iomem.h:143-146 · chunk 164 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void set_irq(IRQSignal *irq, int level)
//     {
//         irq->set_irq(irq->opaque, irq->irq_num, level);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk164 {
// Port of iomem.h's `static inline void set_irq(IRQSignal *irq, int level)`.
use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let sig = &*irq;
    if let Some(f) = sig.set_irq {
        f(sig.opaque, sig.irq_num, level);
    }
}
}

// ── `irq->set_irq(irq->opaque, irq->irq_num, level);` · iomem.h:145-145 · chunk 165 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     irq->set_irq(irq->opaque, irq->irq_num, level);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk165 {
use core::ffi::c_int;
use core::ffi::c_void;

type SetIRQFunc = extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
struct IRQSignal {
    set_irq: SetIRQFunc,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[no_mangle]
pub extern "C" fn set_irq__part165(irq: *mut IRQSignal, level: c_int) {
    unsafe {
        let func = (*irq).set_irq;
        func((*irq).opaque, (*irq).irq_num, level);
    }
}
}

