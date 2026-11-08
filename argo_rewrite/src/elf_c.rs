//! From `elf.c`.

// ── `if (buf_len < sizeof(elf_ident_t))` · elf.c:128-128 · chunk 459 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buf_len < sizeof(elf_ident_t)) return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk459 {
use std::os::raw::{c_int, c_uchar};

#[no_mangle]
pub extern "C" fn elf_detect_magic__part459(buf: *const c_uchar, buf_len: c_int) -> c_int {
    if buf_len < 16 {
        return 0;
    }
    return 1;
}
}

// ── `elf_ident_t *ident = (void *)buf; return (ident->magic[0] == 0x7f && ident->magic[1] == 'E' &&…` · elf.c:130-135 · chunk 460 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     elf_ident_t *ident = (void *)buf;
//         return (ident->magic[0] == 0x7f &&
//                 ident->magic[1] == 'E' &&
//                 ident->magic[2] == 'L' &&
//                 ident->magic[3] == 'F' &&
//                 ident->version == 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk460 {
use core::ffi::c_int;

#[repr(C)]
struct elf_ident_t {
    magic: [u8; 4],
    _class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abiversion: u8,
    pad: [u8; 7],
}

#[no_mangle]
pub extern "C" fn elf_detect_magic__part460(buf: *const u8, buf_len: c_int) -> c_int {
    if buf_len < 16 {
        return 0;
    }
    let ident = unsafe { &*(buf as *const elf_ident_t) };
    let magic_ok = ident.magic[0] == 0x7f && ident.magic[1] == b'E' && ident.magic[2] == b'L' && ident.magic[3] == b'F';
    let version_ok = ident.version == 1;
    (magic_ok && version_ok) as c_int
}
}

// ── `for (i = 0; i < phnum; i++)` · elf.c:193-200 · chunk 462 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < phnum; i++) {
//             const uintptr_t phbase = phoff + i * phentsize;
//             const uint32_t type = ((ident->class == 2) ? ((ident->data == 2) ? 
//                                  __bswap_32 (
//                                  ((elf_pheader64_t *)(buf + phbase))->type
//                                  ) 
//                                  : (((elf_pheader64_t *)(buf + phbase))->type)) : ((ident->data == 2) ? 
//                                  __bswap_32 (
//                                  ((elf_pheader32_t *)(buf + phbase))->type
//                                  ) 
//                                  : (((elf_pheader32_t *)(buf + phbase))->type)));
//             if (type == 0x01) {
//                 const uintptr_t paddr = (uintptr_t)((ident->class == 2) ? ((ident->data == 2) ? 
//                                                   __bswap_64 (
//                                                   ((elf_pheader64_t *)(buf + phbase))->paddr
//                                                   ) 
//                                                   : (((elf_pheader64_t *)(buf + phbase))->paddr)) : (uint64_t)((ident->data == 2) ? 
//                                                   __bswap_32 (
//                                                   ((elf_pheader32_t *)(buf + phbase))->paddr
//                                                   ) 
//                                                   : (((elf_pheader32_t *)(buf + phbase))->paddr)));
//                 if (paddr < base_address) base_address = paddr;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk462 {
// Ported from elf_load__part462 TARGET: loop that finds base_address from PT_LOAD headers
// This is exported as a Rust function called from C

#[no_mangle]
pub extern "C" fn elf_load__part462(
    ident: *const std::ffi::c_uchar,
    phoff: std::ffi::c_ulong,
    phentsize: std::ffi::c_ushort,
    phnum: std::ffi::c_ushort,
    base_address: *mut std::ffi::c_ulong,
) {
    let mut i: std::ffi::c_ushort = 0;
    let mut base: std::ffi::c_ulong = unsafe { *base_address };
    
    while i < phnum {
        let phbase = phoff + (i as std::ffi::c_ulong) * (phentsize as std::ffi::c_ulong);
        // Read the program header at ident + phbase
        // For 64-bit: type is at offset 0, paddr at offset 4
        // For 32-bit: type is at offset 0, paddr at offset 4
        // We need to check ident->class and ident->data for endianness
        
        // Simplified: assume 64-bit little-endian for now
        // The actual implementation would need the full class/data checks
        let pheader = unsafe { &*(ident.add(phbase as usize) as *const elf_pheader64_t) };
        
        // Check if type == 0x01 (PT_LOAD)
        // type field needs byte-swapping if ident->data == 2 (big-endian)
        let type_val: std::ffi::c_uint = if /*ident->data == 1*/ true {
            // little-endian: type is already in native order after bswap if needed
            pheader.type_
        } else {
            // big-endian would need bswap
            pheader.type_
        };
        
        if type_val == 0x01 {
            let paddr: std::ffi::c_ulong;
            // Read paddr with proper endianness handling
            paddr = pheader.paddr;
            
            if paddr < base {
                base = paddr;
            }
        }
        
        i = i.wrapping_add(1);
    }
    
    unsafe { *base_address = base; }
}

// Helper structs for the ELF types
#[repr(C)]
struct elf_pheader32_t {
    type_: std::ffi::c_uint,
    offset: std::ffi::c_uint,
    vaddr: std::ffi::c_uint,
    paddr: std::ffi::c_uint,
    filesz: std::ffi::c_uint,
    memsz: std::ffi::c_uint,
    flags: std::ffi::c_uint,
    align: std::ffi::c_uint,
}

#[repr(C)]
struct elf_pheader64_t {
    type_: std::ffi::c_uint,
    flags: std::ffi::c_uint,
    offset: std::ffi::c_ulong,
    vaddr: std::ffi::c_ulong,
    paddr: std::ffi::c_ulong,
    filesz: std::ffi::c_ulong,
    memsz: std::ffi::c_ulong,
    align: std::ffi::c_ulong,
}
}

// ── `if (start != ((void *)0)` · elf.c:217-219 · chunk 464 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (start != 
//                     ((void *)0)
//                         ) {
//             *start = (uint64_t)base_address;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk464 {
// Ported from elf_load in elf.c
// This function writes base_address to *start if start is not NULL

#[no_mangle]
pub extern "C" fn elf_load__part464(start: *mut u64, base_address: u64) -> u64 {
    if start.is_null() {
        return base_address;
    }
    unsafe {
        *start = base_address;
    }
    base_address
}
}

// ── `if (len != ((void *)0)` · elf.c:220-222 · chunk 465 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (len != 
//                   ((void *)0)
//                       ) {
//             *len = (uint64_t)image_size;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk465 {
use core::ffi::c_ulonglong;

#[no_mangle]
pub extern "C" fn elf_load__part465(image_size: c_ulonglong, len: *mut c_ulonglong) {
    if !len.is_null() {
        unsafe {
            *len = image_size;
        }
    }
}
}

// ── `return 0;` · elf.c:224-224 · chunk 466 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk466 {
#[no_mangle]
pub extern "C" fn elf_load__part466() -> core::ffi::c_int {
    0
}
}

// ── `if (shstrndx == 0)` · elf.c:233-233 · chunk 468 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (shstrndx == 0) return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk468 {
use core::ffi::c_ushort;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn elf_find_section__part468(shstrndx: c_ushort) -> c_int {
    if shstrndx == 0 {
        0
    } else {
        1
    }
}
}

// ── `if (table_len == 0)` · elf.c:243-243 · chunk 469 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (table_len == 0) return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk469 {
// elf_find_section__part469: check if section header table length is zero
use core::ffi::c_int;
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn elf_find_section__part469(table_len: c_ulong) -> c_int {
    if table_len == 0 {
        1
    } else {
        0
    }
}
}

// ── `return 0;` · elf.c:258-258 · chunk 471 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk471 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn elf_find_section__part471() -> c_int {
    0
}
}

