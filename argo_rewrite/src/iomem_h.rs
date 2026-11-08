//! From `iomem.h`.

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

