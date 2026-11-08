//! From `iomem.c`.

// ── `PhysMemoryMap *s; s = mallocz(sizeof(*s)); s->register_ram = default_register_ram; s->free_ram…` · iomem.c:43-49 · chunk 1249 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryMap *s;
//         s = mallocz(sizeof(*s));
//         s->register_ram = default_register_ram;
//         s->free_ram = default_free_ram;
//         s->get_dirty_bits = default_get_dirty_bits;
//         s->set_ram_addr = default_set_addr;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1249 {
// Rust port of the tail of phys_mem_map_init (iomem.c lines 43-49).
#![allow(non_camel_case_types)]

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

pub type RegisterRamFunc =
    Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>;
pub type FreeRamFunc = Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>;
pub type GetDirtyBitsFunc =
    Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>;
pub type SetRamAddrFunc =
    Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>;

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: RegisterRamFunc,
    pub free_ram: FreeRamFunc,
    pub get_dirty_bits: GetDirtyBitsFunc,
    pub set_ram_addr: SetRamAddrFunc,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn phys_mem_map_init__part1249(
    register_ram: RegisterRamFunc,
    free_ram: FreeRamFunc,
    get_dirty_bits: GetDirtyBitsFunc,
    set_ram_addr: SetRamAddrFunc,
) -> *mut PhysMemoryMap {
    let s = mallocz(core::mem::size_of::<PhysMemoryMap>()) as *mut PhysMemoryMap;
    (*s).register_ram = register_ram;
    (*s).free_ram = free_ram;
    (*s).get_dirty_bits = get_dirty_bits;
    (*s).set_ram_addr = set_ram_addr;
    s
}
}

// ── `for (i = 0; i < s->n_phys_mem_range; i++)` · iomem.c:57-62 · chunk 1251 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < s->n_phys_mem_range; i++) {
//             pr = &s->phys_mem_range[i];
//             if (pr->is_ram) {
//                 s->free_ram(s, pr);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1251 {
// Port of the loop in phys_mem_map_end() from iomem.c
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: *mut c_void,
}

type FreeRamFn = unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange);

#[no_mangle]
pub unsafe extern "C" fn phys_mem_map_end__part1251(s: *mut PhysMemoryMap) {
    if s.is_null() {
        return;
    }
    let base: *mut PhysMemoryRange = core::ptr::addr_of_mut!((*s).phys_mem_range) as *mut PhysMemoryRange;
    let mut i: c_int = 0;
    while i < (*s).n_phys_mem_range {
        let pr: *mut PhysMemoryRange = base.offset(i as isize);
        if (*pr).is_ram != 0 {
            let free_ram: FreeRamFn = core::mem::transmute((*s).free_ram);
            free_ram(s, pr);
        }
        i += 1;
    }
}
}

// ── `free(s);` · iomem.c:63-63 · chunk 1252 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1252 {
// Port of the `free(s);` statement at the end of phys_mem_map_end().
// The argument is the PhysMemoryMap pointer allocated by mallocz() in
// phys_mem_map_init(); it is simply released with libc free().

use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn phys_mem_map_end__part1252(s: *mut c_void) {
    unsafe {
        free(s);
    }
}
}

// ── `for (i = 0; i < s->n_phys_mem_range; i++)` · iomem.c:72-76 · chunk 1254 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < s->n_phys_mem_range; i++) {
//             pr = &s->phys_mem_range[i];
//             if (paddr >= pr->addr && paddr < pr->addr + pr->size)
//                 return pr;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1254 {
// Portion of get_phys_mem_range: search the physical memory ranges of the map
// for the first one containing paddr.

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
    pub read_func: Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

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

#[no_mangle]
pub unsafe extern "C" fn get_phys_mem_range__part1254(
    s: *mut PhysMemoryMap,
    paddr: u64,
) -> *mut PhysMemoryRange {
    let map: *mut PhysMemoryMap = s;
    let n: c_int = (*map).n_phys_mem_range;
    let base: *mut PhysMemoryRange = (*map).phys_mem_range.as_mut_ptr();
    let mut i: c_int = 0;
    while i < n {
        let pr: *mut PhysMemoryRange = base.add(i as usize);
        let addr: u64 = (*pr).addr;
        let size: u64 = (*pr).size;
        if paddr >= addr && paddr < addr.wrapping_add(size) {
            return pr;
        }
        i += 1;
    }
    core::ptr::null_mut()
}
}

// ── `return ((void *)0) ;` · iomem.c:77-77 · chunk 1255 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1255 {
// Port of the trailing `return ((void *)0);` of get_phys_mem_range().
// The target simply yields a null PhysMemoryRange pointer.

// Opaque mirror of the C `PhysMemoryRange` struct: only ever handled
// through a pointer here, so its layout need not be reproduced.
#[repr(C)]
pub struct PhysMemoryRange {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn get_phys_mem_range__part1255() -> *mut PhysMemoryRange {
    core::ptr::null_mut()
}
}

// ── `PhysMemoryRange *pr; ((void) sizeof (__assert_single_arg ( s->n_phys_mem_range < 32 )), __exten…` · iomem.c:83-92 · chunk 1257 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryRange *pr;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        s->n_phys_mem_range < 32
//        )), __extension__ ({ if (
//        s->n_phys_mem_range < 32
//        ) ; else __assert_fail (
//        "s->n_phys_mem_range < PHYS_MEM_RANGE_MAX"
//        , "iomem.c", 85, __extension__ __PRETTY_FUNCTION__); }))
//                                                        ;
//         
//        ((void) sizeof (__assert_single_arg (
//        (size & ((1 << 12) - 1)) == 0 && size != 0
//        )), __extension__ ({ if (
//        (size & ((1 << 12) - 1)) == 0 && size != 0
//        ) ; else __assert_fail (
//        "(size & (DEVRAM_PAGE_SIZE - 1)) == 0 && size != 0"
//        , "iomem.c", 86, __extension__ __PRETTY_FUNCTION__); }))
//                                                                 ;
//         pr = &s->phys_mem_range[s->n_phys_mem_range++];
//         pr->map = s;
//         pr->is_ram = TRUE;
//         pr->devram_flags = devram_flags & ~(1 << 2);
//         pr->addr = addr;
//         pr->org_size = size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1257 {
// Port of the first part of register_ram_entry (iomem.c lines 83-92).
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uint, c_void};

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
    pub read_func: Option<extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram:
        Option<extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<extern "C" fn(*mut c_void, *mut u8, usize)>,
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

#[no_mangle]
pub extern "C" fn register_ram_entry__part1257(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    unsafe {
        let idx: c_int = (*s).n_phys_mem_range;

        if !(idx < 32) {
            __assert_fail(
                b"s->n_phys_mem_range < PHYS_MEM_RANGE_MAX\0".as_ptr() as *const c_char,
                b"iomem.c\0".as_ptr() as *const c_char,
                85u32,
                b"register_ram_entry\0".as_ptr() as *const c_char,
            );
        }

        if !((size & ((1u64 << 12) - 1)) == 0 && size != 0) {
            __assert_fail(
                b"(size & (DEVRAM_PAGE_SIZE - 1)) == 0 && size != 0\0".as_ptr() as *const c_char,
                b"iomem.c\0".as_ptr() as *const c_char,
                86u32,
                b"register_ram_entry\0".as_ptr() as *const c_char,
            );
        }

        let pr: *mut PhysMemoryRange =
            (*s).phys_mem_range.as_mut_ptr().add(idx as usize);
        (*s).n_phys_mem_range = idx + 1;

        (*pr).map = s;
        (*pr).is_ram = 1; // TRUE
        (*pr).devram_flags = devram_flags & !(1i32 << 2);
        (*pr).addr = addr;
        (*pr).org_size = size;

        pr
    }
}
}

// ── `if (devram_flags & (1 << 2) … else` · iomem.c:93-96 · chunk 1258 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (devram_flags & (1 << 2))
//             pr->size = 0;
//         else
//             pr->size = pr->org_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1258 {
// Ported piece of register_ram_entry: set pr->size depending on the
// devram "size 0" flag (bit 2).
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
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

#[no_mangle]
pub extern "C" fn register_ram_entry__part1258(pr: *mut PhysMemoryRange, devram_flags: c_int) {
    unsafe {
        if (devram_flags & (1 << 2)) != 0 {
            (*pr).size = 0;
        } else {
            (*pr).size = (*pr).org_size;
        }
    }
}
}

// ── `pr->phys_mem = ((void *)0) ; pr->dirty_bits = ((void *)0) ; return pr;` · iomem.c:97-99 · chunk 1259 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pr->phys_mem = 
//                       ((void *)0)
//                           ;
//         pr->dirty_bits = 
//                         ((void *)0)
//                             ;
//         return pr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1259 {
// Rust rewrite of the tail of register_ram_entry: clears the two RAM
// pointers of the freshly registered range and returns the range.

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: i32,
    pub devram_flags: i32,
    pub phys_mem: *mut u8,
    pub dirty_bits_size: i32,
    pub dirty_bits: *mut u32,
    pub dirty_bits_tab: [*mut u32; 2],
    pub dirty_bits_index: i32,
    pub opaque: *mut core::ffi::c_void,
    pub read_func: *mut core::ffi::c_void,
    pub write_func: *mut core::ffi::c_void,
    pub devio_flags: i32,
}

#[no_mangle]
pub extern "C" fn register_ram_entry__part1259(
    pr: *mut PhysMemoryRange,
) -> *mut PhysMemoryRange {
    unsafe {
        if !pr.is_null() {
            (*pr).phys_mem = core::ptr::null_mut();
            (*pr).dirty_bits = core::ptr::null_mut();
        }
    }
    pr
}
}

// ── `PhysMemoryRange *pr; pr = register_ram_entry(s, addr, size, devram_flags); pr->phys_mem = mallo…` · iomem.c:105-109 · chunk 1261 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryRange *pr;
//     
//         pr = register_ram_entry(s, addr, size, devram_flags);
//     
//         pr->phys_mem = mallocz(size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1261 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
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

extern "C" {
    fn register_ram_entry(
        s: *mut PhysMemoryMap,
        addr: u64,
        size: u64,
        devram_flags: c_int,
    ) -> *mut PhysMemoryRange;
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn default_register_ram__part1261(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
    devram_flags: c_int,
) -> *mut PhysMemoryRange {
    unsafe {
        let pr = register_ram_entry(s, addr, size, devram_flags);
        (*pr).phys_mem = mallocz(size as usize) as *mut u8;
        pr
    }
}
}

// ── `if (!pr->phys_mem)` · iomem.c:110-113 · chunk 1262 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr->phys_mem) {
//             fprintf(
//                    stderr
//                          , "Could not allocate VM memory\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1262 {
// Port of the allocation-failure check from default_register_ram (iomem.c).
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // extern FILE *stderr;
    static stderr: *mut c_void;
    // extern int fprintf (FILE *__restrict __stream, const char *__restrict __format, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    // extern void exit (int __status) __attribute__ ((__noreturn__));
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn default_register_ram__part1262(phys_mem: *mut u8) {
    if phys_mem.is_null() {
        unsafe {
            fprintf(
                stderr,
                b"Could not allocate VM memory\n\0".as_ptr() as *const c_char,
            );
            exit(1);
        }
    }
}
}

// ── `if (devram_flags & (1 << 1)` · iomem.c:115-125 · chunk 1263 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (devram_flags & (1 << 1)) {
//             size_t nb_pages;
//             int i;
//             nb_pages = size >> 12;
//             pr->dirty_bits_size = ((nb_pages + 31) / 32) * sizeof(uint32_t);
//             pr->dirty_bits_index = 0;
//             for(i = 0; i < 2; i++) {
//                 pr->dirty_bits_tab[i] = mallocz(pr->dirty_bits_size);
//             }
//             pr->dirty_bits = pr->dirty_bits_tab[pr->dirty_bits_index];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1263 {
// Port of the dirty-bits initialization block of default_register_ram.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
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

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn default_register_ram__part1263(
    pr: *mut PhysMemoryRange,
    size: u64,
    devram_flags: c_int,
) {
    if devram_flags & (1 << 1) != 0 {
        let nb_pages: usize = (size >> 12) as usize;
        let bits_size: usize = ((nb_pages + 31) / 32) * 4;
        (*pr).dirty_bits_size = bits_size as c_int;
        (*pr).dirty_bits_index = 0;
        let mut i: c_int = 0;
        while i < 2 {
            let p = mallocz((*pr).dirty_bits_size as usize);
            let slot = (*pr).dirty_bits_tab.as_mut_ptr().add(i as usize);
            *slot = p as *mut u32;
            i += 1;
        }
        let idx = (*pr).dirty_bits_index as usize;
        if idx < 2 {
            (*pr).dirty_bits = *(*pr).dirty_bits_tab.as_ptr().add(idx);
        }
    }
}
}

// ── `return pr;` · iomem.c:126-126 · chunk 1264 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1264 {
// Piece of default_register_ram: returns the registered range pointer.
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn default_register_ram__part1264(pr: *mut c_void) -> *mut c_void {
    pr
}
}

// ── `uint32_t *dirty_bits; BOOL has_dirty_bits; size_t n, i; dirty_bits = pr->dirty_bits; has_dirty_…` · iomem.c:133-140 · chunk 1266 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t *dirty_bits;
//         BOOL has_dirty_bits;
//         size_t n, i;
//     
//         dirty_bits = pr->dirty_bits;
//     
//         has_dirty_bits = FALSE;
//         n = pr->dirty_bits_size / sizeof(uint32_t);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1266 {
// Port of iomem.c lines 133-140 (part of default_get_dirty_bits):
// the declarations/initialisations of dirty_bits, has_dirty_bits and n.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
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

#[no_mangle]
pub unsafe extern "C" fn default_get_dirty_bits__part1266(
    pr: *mut PhysMemoryRange,
    dirty_bits_out: *mut *mut u32,
    has_dirty_bits_out: *mut c_int,
    n_out: *mut usize,
) {
    let dirty_bits: *mut u32 = (*pr).dirty_bits;
    let has_dirty_bits: c_int = 0; // FALSE
    // pr->dirty_bits_size / sizeof(uint32_t): unsigned division, as in C
    let n: usize = ((*pr).dirty_bits_size as usize) / core::mem::size_of::<u32>();

    if !dirty_bits_out.is_null() {
        *dirty_bits_out = dirty_bits;
    }
    if !has_dirty_bits_out.is_null() {
        *has_dirty_bits_out = has_dirty_bits;
    }
    if !n_out.is_null() {
        *n_out = n;
    }
}
}

// ── `for (i = 0; i < n; i++)` · iomem.c:141-146 · chunk 1267 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < n; i++) {
//             if (dirty_bits[i] != 0) {
//                 has_dirty_bits = TRUE;
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1267 {
use core::ffi::c_int;

// Port of the dirty-bits scan of default_get_dirty_bits():
// returns TRUE (1) as soon as any word of the bitmap is non-zero, else FALSE (0).
#[no_mangle]
pub extern "C" fn default_get_dirty_bits__part1267(dirty_bits: *const u32, n: usize) -> c_int {
    let mut i: usize = 0;
    while i < n {
        let v = unsafe { *dirty_bits.add(i) };
        if v != 0 {
            return 1;
        }
        i += 1;
    }
    0
}
}

// ── `if (has_dirty_bits && pr->size != 0)` · iomem.c:147-150 · chunk 1268 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (has_dirty_bits && pr->size != 0) {
//     
//             map->flush_tlb_write_range(map->opaque, pr->phys_mem, pr->org_size);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1268 {
// Rust port of the `default_get_dirty_bits` TLB flush branch.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

pub type FlushTlbWriteRange = unsafe extern "C" fn(*mut c_void, *mut u8, usize);

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: *mut c_void,
    pub free_ram: *mut c_void,
    pub get_dirty_bits: *mut c_void,
    pub set_ram_addr: *mut c_void,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: FlushTlbWriteRange,
}

#[no_mangle]
pub unsafe extern "C" fn default_get_dirty_bits__part1268(
    map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    has_dirty_bits: c_int,
) {
    if has_dirty_bits != 0 && (*pr).size != 0 {
        let flush = (*map).flush_tlb_write_range;
        flush((*map).opaque, (*pr).phys_mem, (*pr).org_size as usize);
    }
}
}

// ── `pr->dirty_bits_index ^= 1; pr->dirty_bits = pr->dirty_bits_tab[pr->dirty_bits_index]; memset(pr…` · iomem.c:152-155 · chunk 1269 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pr->dirty_bits_index ^= 1;
//         pr->dirty_bits = pr->dirty_bits_tab[pr->dirty_bits_index];
//         memset(pr->dirty_bits, 0, pr->dirty_bits_size);
//         return dirty_bits;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1269 {
// Port of default_get_dirty_bits lines 152-155 (iomem.c)
use core::ffi::{c_int, c_void};

// Opaque: only ever handled through a pointer here.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
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

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn default_get_dirty_bits__part1269(
    pr: *mut PhysMemoryRange,
    dirty_bits: *mut u32,
) -> *const u32 {
    let idx: c_int = (*pr).dirty_bits_index ^ 1;
    (*pr).dirty_bits_index = idx;
    let new_bits: *mut u32 = *(*pr).dirty_bits_tab.as_ptr().add(idx as usize);
    (*pr).dirty_bits = new_bits;
    // int -> size_t conversion as in C (sign extend, then reinterpret)
    let n: usize = (*pr).dirty_bits_size as i64 as usize;
    memset((*pr).dirty_bits as *mut c_void, 0, n);
    dirty_bits as *const u32
}
}

// ── `if (pr->dirty_bits)` · iomem.c:164-176 · chunk 1271 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pr->dirty_bits) {
//             page_index = offset >> 12;
//             mask = 1 << (page_index & 0x1f);
//             dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
//             if (*dirty_bits_ptr & mask) {
//                 *dirty_bits_ptr &= ~mask;
//     
//                 map = pr->map;
//                 map->flush_tlb_write_range(map->opaque,
//                                            pr->phys_mem + (offset & ~((1 << 12) - 1)),
//                                            (1 << 12));
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1271 {
// Rust port of the body of phys_mem_reset_dirty_bit (iomem.c lines 164-176).

use core::ffi::{c_int, c_void};

type FlushTlbWriteRange = Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>;
type RegisterRamFn = Option<unsafe extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>;
type FreeRamFn = Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>;
type GetDirtyBitsFn = Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>;
type SetRamAddrFn = Option<unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>;
type DeviceReadFn = Option<unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32>;
type DeviceWriteFn = Option<unsafe extern "C" fn(*mut c_void, u32, u32, c_int)>;

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
    pub read_func: DeviceReadFn,
    pub write_func: DeviceWriteFn,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram: RegisterRamFn,
    pub free_ram: FreeRamFn,
    pub get_dirty_bits: GetDirtyBitsFn,
    pub set_ram_addr: SetRamAddrFn,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: FlushTlbWriteRange,
}

#[no_mangle]
pub extern "C" fn phys_mem_reset_dirty_bit__part1271(pr: *mut PhysMemoryRange, offset: usize) {
    unsafe {
        if pr.is_null() {
            return;
        }
        let pr = &mut *pr;
        let bits = pr.dirty_bits;
        if !bits.is_null() {
            let page_index: usize = offset >> 12;
            let mask: u32 = 1u32 << ((page_index & 0x1f) as u32);
            let dirty_bits_ptr: *mut u32 = bits.add(page_index >> 5);
            if (*dirty_bits_ptr) & mask != 0 {
                *dirty_bits_ptr &= !mask;

                let map: *mut PhysMemoryMap = pr.map;
                if !map.is_null() {
                    let flush = (*map).flush_tlb_write_range;
                    if let Some(flush) = flush {
                        let ram_addr: *mut u8 = pr.phys_mem.add(offset & !4095usize);
                        flush((*map).opaque, ram_addr, 4096usize);
                    }
                }
            }
        }
    }
}
}

// ── `free(pr->phys_mem);` · iomem.c:181-181 · chunk 1273 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(pr->phys_mem);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1273 {
// Ported piece of default_free_ram: free(pr->phys_mem)
use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn default_free_ram__part1273(phys_mem: *mut u8) {
    unsafe {
        free(phys_mem as *mut c_void);
    }
}
}

// ── `PhysMemoryRange *pr; ((void) sizeof (__assert_single_arg ( s->n_phys_mem_range < 32 )), __exten…` · iomem.c:189-195 · chunk 1275 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PhysMemoryRange *pr;
//         
//        ((void) sizeof (__assert_single_arg (
//        s->n_phys_mem_range < 32
//        )), __extension__ ({ if (
//        s->n_phys_mem_range < 32
//        ) ; else __assert_fail (
//        "s->n_phys_mem_range < PHYS_MEM_RANGE_MAX"
//        , "iomem.c", 190, __extension__ __PRETTY_FUNCTION__); }))
//                                                        ;
//         
//        ((void) sizeof (__assert_single_arg (
//        size <= 0xffffffff
//        )), __extension__ ({ if (
//        size <= 0xffffffff
//        ) ; else __assert_fail (
//        "size <= 0xffffffff"
//        , "iomem.c", 191, __extension__ __PRETTY_FUNCTION__); }))
//                                  ;
//         pr = &s->phys_mem_range[s->n_phys_mem_range++];
//         pr->map = s;
//         pr->addr = addr;
//         pr->org_size = size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1275 {
// Portion of cpu_register_device(): the assertions and the allocation of the
// next PhysMemoryRange entry, including the fields set before the branch.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

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
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
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
    pub get_dirty_bits: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32,
    >,
    pub set_ram_addr: Option<
        unsafe extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int),
    >,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize)>,
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

static ASSERT_N_RANGE: &[u8] = b"s->n_phys_mem_range < PHYS_MEM_RANGE_MAX\0";
static ASSERT_SIZE: &[u8] = b"size <= 0xffffffff\0";
static FILE_NAME: &[u8] = b"iomem.c\0";
static FUNC_NAME: &[u8] = b"cpu_register_device\0";

#[no_mangle]
pub unsafe extern "C" fn cpu_register_device__part1275(
    s: *mut PhysMemoryMap,
    addr: u64,
    size: u64,
) -> *mut PhysMemoryRange {
    if !((*s).n_phys_mem_range < 32) {
        __assert_fail(
            ASSERT_N_RANGE.as_ptr() as *const c_char,
            FILE_NAME.as_ptr() as *const c_char,
            190,
            FUNC_NAME.as_ptr() as *const c_char,
        );
    }
    if !(size <= 0xffffffffu64) {
        __assert_fail(
            ASSERT_SIZE.as_ptr() as *const c_char,
            FILE_NAME.as_ptr() as *const c_char,
            191,
            FUNC_NAME.as_ptr() as *const c_char,
        );
    }

    let index: c_int = (*s).n_phys_mem_range;
    (*s).n_phys_mem_range = index.wrapping_add(1);

    let base: *mut PhysMemoryRange = core::ptr::addr_of_mut!((*s).phys_mem_range) as *mut _;
    let pr: *mut PhysMemoryRange = base.offset(index as isize);

    (*pr).map = s;
    (*pr).addr = addr;
    (*pr).org_size = size;

    pr
}
}

// ── `if (devio_flags & (1 << 4) … else` · iomem.c:196-199 · chunk 1276 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (devio_flags & (1 << 4))
//             pr->size = 0;
//         else
//             pr->size = pr->org_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1276 {
// Port of iomem.c lines 196-199: set the size of the newly registered
// device memory range according to the DEVIO flags.

use core::ffi::{c_int, c_uint, c_uchar, c_void};

pub type BOOL = c_int;

pub type DeviceWriteFunc =
    extern "C" fn(opaque: *mut c_void, offset: u32, val: u32, size_log2: c_int);
pub type DeviceReadFunc = extern "C" fn(opaque: *mut c_void, offset: u32, size_log2: c_int) -> u32;

#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void, // PhysMemoryMap *
    pub addr: u64,
    pub org_size: u64,
    pub size: u64,
    pub is_ram: BOOL,
    pub devram_flags: c_int,
    pub phys_mem: *mut c_uchar,
    pub dirty_bits_size: c_int,
    pub dirty_bits: *mut c_uint,
    pub dirty_bits_tab: [*mut c_uint; 2],
    pub dirty_bits_index: c_int,
    pub opaque: *mut c_void,
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn cpu_register_device__part1276(pr: *mut PhysMemoryRange, devio_flags: c_int) {
    if pr.is_null() {
        return;
    }
    unsafe {
        if (devio_flags & (1 << 4)) != 0 {
            (*pr).size = 0;
        } else {
            (*pr).size = (*pr).org_size;
        }
    }
}
}

// ── `pr->is_ram = FALSE; pr->opaque = opaque; pr->read_func = read_func; pr->write_func = write_func…` · iomem.c:200-205 · chunk 1277 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pr->is_ram = FALSE;
//         pr->opaque = opaque;
//         pr->read_func = read_func;
//         pr->write_func = write_func;
//         pr->devio_flags = devio_flags;
//         return pr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1277 {
// Portion of cpu_register_device: fills in the device (non-RAM) fields of a
// PhysMemoryRange and returns the range pointer.

use core::ffi::{c_int, c_void};

pub type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
pub type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);

// PhysMemoryMap is only ever held by pointer here; it is never dereferenced.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut c_void,
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
    pub read_func: Option<DeviceReadFunc>,
    pub write_func: Option<DeviceWriteFunc>,
    pub devio_flags: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn cpu_register_device__part1277(
    pr: *mut PhysMemoryRange,
    opaque: *mut c_void,
    read_func: Option<DeviceReadFunc>,
    write_func: Option<DeviceWriteFunc>,
    devio_flags: c_int,
) -> *mut PhysMemoryRange {
    let p = pr;
    (*p).is_ram = 0;
    (*p).opaque = opaque;
    (*p).read_func = read_func;
    (*p).write_func = write_func;
    (*p).devio_flags = devio_flags;
    p
}
}

// ── `if (enabled) … else` · iomem.c:211-231 · chunk 1279 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (enabled) {
//             if (pr->size == 0 || pr->addr != addr) {
//     
//                 if (pr->is_ram) {
//                     map->flush_tlb_write_range(map->opaque,
//                                                pr->phys_mem, pr->org_size);
//                 }
//                 pr->addr = addr;
//                 pr->size = pr->org_size;
//             }
//         } else {
//             if (pr->size != 0) {
//     
//                 if (pr->is_ram) {
//                     map->flush_tlb_write_range(map->opaque,
//                                                pr->phys_mem, pr->org_size);
//                 }
//                 pr->addr = 0;
//                 pr->size = 0;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1279 {
// Port of the set/clear-address body of default_set_addr (iomem.c).
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
    pub read_func: Option<extern "C" fn(*mut c_void, u32, c_int) -> u32>,
    pub write_func: Option<extern "C" fn(*mut c_void, u32, u32, c_int)>,
    pub devio_flags: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
    pub register_ram:
        Option<extern "C" fn(*mut PhysMemoryMap, u64, u64, c_int) -> *mut PhysMemoryRange>,
    pub free_ram: Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange)>,
    pub get_dirty_bits:
        Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange) -> *const u32>,
    pub set_ram_addr:
        Option<extern "C" fn(*mut PhysMemoryMap, *mut PhysMemoryRange, u64, c_int)>,
    pub opaque: *mut c_void,
    pub flush_tlb_write_range: Option<extern "C" fn(*mut c_void, *mut u8, usize)>,
}

#[inline]
unsafe fn flush_tlb(map: *mut PhysMemoryMap, pr: *mut PhysMemoryRange) {
    if let Some(mapref) = map.as_ref() {
        if let Some(flush) = mapref.flush_tlb_write_range {
            flush(
                mapref.opaque,
                (*pr).phys_mem,
                (*pr).org_size as usize,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn default_set_addr__part1279(
    map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: c_int,
) {
    if enabled != 0 {
        if (*pr).size == 0 || (*pr).addr != addr {
            if (*pr).is_ram != 0 {
                flush_tlb(map, pr);
            }
            (*pr).addr = addr;
            (*pr).size = (*pr).org_size;
        }
    } else {
        if (*pr).size != 0 {
            if (*pr).is_ram != 0 {
                flush_tlb(map, pr);
            }
            (*pr).addr = 0;
            (*pr).size = 0;
        }
    }
}
}

// ── `if (!pr->is_ram) … else` · iomem.c:237-241 · chunk 1281 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr->is_ram) {
//             default_set_addr(map, pr, addr, enabled);
//         } else {
//             return map->set_ram_addr(map, pr, addr, enabled);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1281 {
// Rust port of the tail of phys_mem_set_addr (originally iomem.c lines 237-241).
// `default_set_addr` is a C `static` function (no linker symbol), so its logic is
// mirrored here as a private Rust function.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

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

// Private port of the C `static void default_set_addr(...)`.
unsafe fn default_set_addr(
    map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: c_int,
) {
    if enabled != 0 {
        if (*pr).size == 0 || (*pr).addr != addr {
            if (*pr).is_ram != 0 {
                if let Some(f) = (*map).flush_tlb_write_range {
                    f((*map).opaque, (*pr).phys_mem, (*pr).org_size as usize);
                }
            }
            (*pr).addr = addr;
            (*pr).size = (*pr).org_size;
        }
    } else if (*pr).size != 0 {
        if (*pr).is_ram != 0 {
            if let Some(f) = (*map).flush_tlb_write_range {
                f((*map).opaque, (*pr).phys_mem, (*pr).org_size as usize);
            }
        }
        (*pr).addr = 0;
        (*pr).size = 0;
    }
}

/// Returns nonzero when the caller must `return;` (the is_ram branch).
#[no_mangle]
pub unsafe extern "C" fn phys_mem_set_addr__part1281(
    map: *mut PhysMemoryMap,
    pr: *mut PhysMemoryRange,
    addr: u64,
    enabled: c_int,
) -> c_int {
    if (*pr).is_ram == 0 {
        default_set_addr(map, pr, addr, enabled);
        0
    } else {
        if let Some(f) = (*map).set_ram_addr {
            f(map, pr, addr, enabled);
        }
        1
    }
}
}

// ── `if (!pr || !pr->is_ram)` · iomem.c:249-250 · chunk 1283 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!pr || !pr->is_ram)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1283 {
// Mirrors the C PhysMemoryRange struct layout.
#[repr(C)]
pub struct PhysMemoryRange {
    pub map: *mut PhysMemoryMap,
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

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: core::ffi::c_int,
    pub phys_mem_range: [PhysMemoryRange; 32],
}

// Returns nonzero when the caller must return NULL: pr is NULL or not RAM.
#[no_mangle]
pub extern "C" fn phys_mem_get_ram_ptr__part1283(pr: *const PhysMemoryRange) -> core::ffi::c_int {
    if pr.is_null() {
        return 1;
    }
    let is_ram = unsafe { (*pr).is_ram };
    if is_ram == 0 {
        1
    } else {
        0
    }
}
}

// ── `offset = paddr - pr->addr;` · iomem.c:251-251 · chunk 1284 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     offset = paddr - pr->addr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1284 {
// Compute the offset of paddr inside a physical memory range.

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
pub extern "C" fn phys_mem_get_ram_ptr__part1284(
    paddr: u64,
    pr: *const PhysMemoryRange,
) -> usize {
    let addr = unsafe { (*pr).addr };
    paddr.wrapping_sub(addr) as usize
}
}

// ── `if (is_rw)` · iomem.c:252-253 · chunk 1285 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_rw)
//             phys_mem_set_dirty_bit(pr, offset);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1285 {
// Portion of phys_mem_get_ram_ptr: mark the dirty bit for the accessed page.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryRange {
    map: *mut c_void,
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
    read_func: *mut c_void,
    write_func: *mut c_void,
    devio_flags: c_int,
}

#[no_mangle]
pub extern "C" fn phys_mem_get_ram_ptr__part1285(pr: *mut PhysMemoryRange, offset: usize) {
    unsafe {
        if pr.is_null() {
            return;
        }
        let pr = &mut *pr;
        let dirty_bits = pr.dirty_bits;
        if !dirty_bits.is_null() {
            let page_index: usize = offset >> 12;
            let mask: u32 = 1u32 << ((page_index & 0x1f) as u32);
            let dirty_bits_ptr: *mut u32 = dirty_bits.add(page_index >> 5);
            *dirty_bits_ptr |= mask;
        }
    }
}
}

// ── `return pr->phys_mem + (uintptr_t)offset;` · iomem.c:254-254 · chunk 1286 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pr->phys_mem + (uintptr_t)offset;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1286 {
// Port of: return pr->phys_mem + (uintptr_t)offset;
// Returns the host pointer to the RAM page at the given byte offset.

#[no_mangle]
pub extern "C" fn phys_mem_get_ram_ptr__part1286(phys_mem: *mut u8, offset: usize) -> *mut u8 {
    // C: pr->phys_mem + (uintptr_t)offset  (wrapping pointer arithmetic)
    (phys_mem as usize).wrapping_add(offset) as *mut u8
}
}

// ── `irq->set_irq = set_irq; irq->opaque = opaque; irq->irq_num = irq_num;` · iomem.c:261-263 · chunk 1288 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     irq->set_irq = set_irq;
//         irq->opaque = opaque;
//         irq->irq_num = irq_num;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1288 {
// Port of the IRQSignal initialisation body of irq_init().
use core::ffi::{c_int, c_void};

/// Mirrors the C `IRQSignal` struct (iomem.c).
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn irq_init__part1288(
    irq: *mut IRQSignal,
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
) {
    if irq.is_null() {
        return;
    }
    let irq = irq;
    core::ptr::addr_of_mut!((*irq).set_irq).write(set_irq);
    core::ptr::addr_of_mut!((*irq).opaque).write(opaque);
    core::ptr::addr_of_mut!((*irq).irq_num).write(irq_num);
}
}

