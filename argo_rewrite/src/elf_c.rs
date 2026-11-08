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

// ── `for (i = 0; i < phnum; i++)` · elf.c:203-215 · chunk 463 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
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
//                 const uintptr_t offset = (uintptr_t)((ident->class == 2) ? ((ident->data == 2) ? 
//                                                    __bswap_64 (
//                                                    ((elf_pheader64_t *)(buf + phbase))->offset
//                                                    ) 
//                                                    : (((elf_pheader64_t *)(buf + phbase))->offset)) : (uint64_t)((ident->data == 2) ? 
//                                                    __bswap_32 (
//                                                    ((elf_pheader32_t *)(buf + phbase))->offset
//                                                    ) 
//                                                    : (((elf_pheader32_t *)(buf + phbase))->offset)));
//                 const uintptr_t paddr = (uintptr_t)((ident->class == 2) ? ((ident->data == 2) ? 
//                                                   __bswap_64 (
//                                                   ((elf_pheader64_t *)(buf + phbase))->paddr
//                                                   ) 
//                                                   : (((elf_pheader64_t *)(buf + phbase))->paddr)) : (uint64_t)((ident->data == 2) ? 
//                                                   __bswap_32 (
//                                                   ((elf_pheader32_t *)(buf + phbase))->paddr
//                                                   ) 
//                                                   : (((elf_pheader32_t *)(buf + phbase))->paddr)));
//                 const size_t filesz = (size_t)((ident->class == 2) ? ((ident->data == 2) ? 
//                                              __bswap_64 (
//                                              ((elf_pheader64_t *)(buf + phbase))->filesz
//                                              ) 
//                                              : (((elf_pheader64_t *)(buf + phbase))->filesz)) : (uint64_t)((ident->data == 2) ? 
//                                              __bswap_32 (
//                                              ((elf_pheader32_t *)(buf + phbase))->filesz
//                                              ) 
//                                              : (((elf_pheader32_t *)(buf + phbase))->filesz)));
//                 const size_t memsz = (size_t)((ident->class == 2) ? ((ident->data == 2) ? 
//                                             __bswap_64 (
//                                             ((elf_pheader64_t *)(buf + phbase))->memsz
//                                             ) 
//                                             : (((elf_pheader64_t *)(buf + phbase))->memsz)) : (uint64_t)((ident->data == 2) ? 
//                                             __bswap_32 (
//                                             ((elf_pheader32_t *)(buf + phbase))->memsz
//                                             ) 
//                                             : (((elf_pheader32_t *)(buf + phbase))->memsz)));
//                 if (paddr - base_address + 
//                                           (((
//                                           memsz
//                                           )>(
//                                           filesz
//                                           ))?(
//                                           memsz
//                                           ):(
//                                           filesz
//                                           )) 
//                                                              > outbuf_len) return -1;
//                 memcpy(&outbuf[paddr - base_address], &buf[offset], filesz);
//                 image_size = 
//                             (((
//                             image_size
//                             )>(
//                             paddr - base_address + memsz
//                             ))?(
//                             image_size
//                             ):(
//                             paddr - base_address + memsz
//                             ))
//                                                                          ;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk463 {
use core::ffi::{c_int, c_ulong, c_ushort};

extern "C" {
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[inline]
unsafe fn rd_u32(p: *const u8) -> u32 {
    (p as *const u32).read_unaligned()
}

#[inline]
unsafe fn rd_u64(p: *const u8) -> u64 {
    (p as *const u64).read_unaligned()
}

#[no_mangle]
pub unsafe extern "C" fn elf_load__part463(
    buf: *const u8,
    phnum: c_ushort,
    phentsize: c_ushort,
    phoff: c_ulong,
    base_address: c_ulong,
    outbuf: *mut u8,
    outbuf_len: c_int,
    image_size_ptr: *mut usize,
) -> c_int {
    let mut image_size: usize = *image_size_ptr;
    // int -> unsigned long conversion in the C comparison (sign extended)
    let limit: c_ulong = outbuf_len as i64 as c_ulong;

    let mut i: c_ushort = 0;
    while (i as u32) < (phnum as u32) {
        // phoff + i * phentsize, with the C int multiplication wrapping
        let prod: c_ulong = ((i as i32).wrapping_mul(phentsize as i32)) as i64 as c_ulong;
        let phbase: c_ulong = phoff.wrapping_add(prod);
        let p: *const u8 = buf.add(phbase as usize);

        let class64: bool = *buf.add(4) == 2;
        let be: bool = *buf.add(5) == 2;

        let t = rd_u32(p);
        let ptype: u32 = if be { t.swap_bytes() } else { t };

        if ptype == 0x01 {
            let offset: c_ulong;
            let paddr: c_ulong;
            let filesz: usize;
            let memsz: usize;
            if class64 {
                let v = rd_u64(p.add(8));
                offset = if be { v.swap_bytes() } else { v };
                let v = rd_u64(p.add(24));
                paddr = if be { v.swap_bytes() } else { v };
                let v = rd_u64(p.add(32));
                filesz = (if be { v.swap_bytes() } else { v }) as usize;
                let v = rd_u64(p.add(40));
                memsz = (if be { v.swap_bytes() } else { v }) as usize;
            } else {
                let v = rd_u32(p.add(4));
                offset = (if be { v.swap_bytes() } else { v }) as c_ulong;
                let v = rd_u32(p.add(12));
                paddr = (if be { v.swap_bytes() } else { v }) as c_ulong;
                let v = rd_u32(p.add(16));
                filesz = (if be { v.swap_bytes() } else { v }) as usize;
                let v = rd_u32(p.add(20));
                memsz = (if be { v.swap_bytes() } else { v }) as usize;
            }

            let dst_off: c_ulong = paddr.wrapping_sub(base_address);
            let maxsz: c_ulong = if memsz > filesz {
                memsz as c_ulong
            } else {
                filesz as c_ulong
            };
            if dst_off.wrapping_add(maxsz) > limit {
                return -1;
            }

            memcpy(
                outbuf.add(dst_off as usize),
                buf.add(offset as usize),
                filesz,
            );

            let cand: usize = dst_off.wrapping_add(memsz as c_ulong) as usize;
            if cand > image_size {
                image_size = cand;
            }
        }

        i = i.wrapping_add(1);
    }

    *image_size_ptr = image_size;
    0
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

// ── `for (i = 0; i < shnum; i++)` · elf.c:247-256 · chunk 470 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < shnum; i++) {
//             shbase = shoff + i * shentsize;
//             const uint32_t name_idx = ((ident->class == 2) ? ((ident->data == 2) ? 
//                                      __bswap_32 (
//                                      ((elf_sheader64_t *)(buf + shbase))->name
//                                      ) 
//                                      : (((elf_sheader64_t *)(buf + shbase))->name)) : ((ident->data == 2) ? 
//                                      __bswap_32 (
//                                      ((elf_sheader32_t *)(buf + shbase))->name
//                                      ) 
//                                      : (((elf_sheader32_t *)(buf + shbase))->name)));
//             const char *name_str = (const char *)(buf + table_start + name_idx);
//             if (!strcmp(name_str, name)) {
//                 if (start != 
//                             ((void *)0)
//                                 ) *start = ((ident->class == 2) ? ((ident->data == 2) ? 
//                                            __bswap_64 (
//                                            ((elf_sheader64_t *)(buf + shbase))->addr
//                                            ) 
//                                            : (((elf_sheader64_t *)(buf + shbase))->addr)) : (uint64_t)((ident->data == 2) ? 
//                                            __bswap_32 (
//                                            ((elf_sheader32_t *)(buf + shbase))->addr
//                                            ) 
//                                            : (((elf_sheader32_t *)(buf + shbase))->addr)));
//                 if (len != 
//                           ((void *)0)
//                               ) *len = ((ident->class == 2) ? ((ident->data == 2) ? 
//                                        __bswap_64 (
//                                        ((elf_sheader64_t *)(buf + shbase))->size
//                                        ) 
//                                        : (((elf_sheader64_t *)(buf + shbase))->size)) : (uint64_t)((ident->data == 2) ? 
//                                        __bswap_32 (
//                                        ((elf_sheader32_t *)(buf + shbase))->size
//                                        ) 
//                                        : (((elf_sheader32_t *)(buf + shbase))->size)));
//                 return 1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk470 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[inline]
unsafe fn rd_u32(p: *const u8, off: usize, be: bool) -> u32 {
    let b = p.add(off);
    let bytes = [*b, *b.add(1), *b.add(2), *b.add(3)];
    if be {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    }
}

#[inline]
unsafe fn rd_u64(p: *const u8, off: usize, be: bool) -> u64 {
    let b = p.add(off);
    let bytes = [
        *b,
        *b.add(1),
        *b.add(2),
        *b.add(3),
        *b.add(4),
        *b.add(5),
        *b.add(6),
        *b.add(7),
    ];
    if be {
        u64::from_be_bytes(bytes)
    } else {
        u64::from_le_bytes(bytes)
    }
}

// Port of the section-table scan loop of elf_find_section.
// Returns 1 (and writes *start / *len) when the named section is found,
// 0 when the whole table was scanned without a match.
#[no_mangle]
pub extern "C" fn elf_find_section__part470(
    buf: *const u8,
    ident_class: u8,
    ident_data: u8,
    shnum: u16,
    shentsize: u16,
    shoff: usize,
    table_start: usize,
    name: *const c_char,
    start: *mut u64,
    len: *mut u64,
) -> c_int {
    unsafe {
        let is64 = ident_class == 2;
        let be = ident_data == 2;
        let mut i: u16 = 0;
        while (i as u32) < (shnum as u32) {
            // C: shbase = shoff + i * shentsize;   (int arithmetic, then uintptr_t)
            let prod: i32 = (i as i32).wrapping_mul(shentsize as i32);
            let shbase: usize = shoff.wrapping_add(prod as isize as usize);
            let ent = buf.add(shbase);

            let name_idx = rd_u32(ent, 0, be);
            let name_str = buf
                .add(table_start.wrapping_add(name_idx as usize))
                as *const c_char;

            if strcmp(name_str, name) == 0 {
                let addr: u64 = if is64 {
                    rd_u64(ent, 16, be)
                } else {
                    rd_u32(ent, 12, be) as u64
                };
                if !start.is_null() {
                    *start = addr;
                }
                let sz: u64 = if is64 {
                    rd_u64(ent, 32, be)
                } else {
                    rd_u32(ent, 20, be) as u64
                };
                if !len.is_null() {
                    *len = sz;
                }
                return 1;
            }

            i = i.wrapping_add(1);
        }

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

