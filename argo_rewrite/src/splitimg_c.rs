//! From `splitimg.c`.

// ── `if (optind + 1) >= argc)` · splitimg.c:39-47 · chunk 3494 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((optind + 1) >= argc) {
//             printf("splitimg version " "2019-02-10" ", Copyright (c) 2011-2016 Fabrice Bellard\n"
//                    "usage: splitimg infile outpath [blocksize]\n"
//                    "Create a multi-file disk image for the RISCVEMU HTTP block device\n"
//                    "\n"
//                    "outpath must be a directory\n"
//                    "blocksize is the block size in KB\n");
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3494 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

static USAGE: &[u8] = b"splitimg version 2019-02-10, Copyright (c) 2011-2016 Fabrice Bellard\nusage: splitimg infile outpath [blocksize]\nCreate a multi-file disk image for the RISCVEMU HTTP block device\n\noutpath must be a directory\nblocksize is the block size in KB\n\0";

// Returns 1 when the usage message was printed (caller must exit(1)), 0 otherwise.
#[no_mangle]
pub extern "C" fn main__part3494(argc: c_int, optind: c_int) -> c_int {
    if optind.wrapping_add(1) >= argc {
        unsafe {
            printf(USAGE.as_ptr() as *const c_char);
        }
        return 1;
    }
    0
}
}

// ── `infilename = argv[optind++]; outpath = argv[optind++]; blocksize = 256;` · splitimg.c:49-51 · chunk 3495 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     infilename = argv[optind++];
//         outpath = argv[optind++];
//         blocksize = 256;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3495 {
use core::ffi::{c_char, c_int};

extern "C" {
    static mut optind: c_int;
}

#[no_mangle]
pub extern "C" fn main__part3495(
    argv: *mut *mut c_char,
    infilename: *mut *const c_char,
    outpath: *mut *const c_char,
    blocksize: *mut c_int,
) {
    unsafe {
        let idx0 = optind;
        optind = idx0 + 1;
        *infilename = *argv.add(idx0 as usize) as *const c_char;

        let idx1 = optind;
        optind = idx1 + 1;
        *outpath = *argv.add(idx1 as usize) as *const c_char;

        *blocksize = 256;
    }
}
}

// ── `if (optind < argc)` · splitimg.c:52-53 · chunk 3496 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (optind < argc)
//             blocksize = strtol(argv[optind++], 
//                                               ((void *)0)
//                                                   , 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3496 {
// Port of splitimg.c lines 52-53: optional blocksize argument parsing.
use core::ffi::{c_char, c_int, c_long};

extern "C" {
    // The C translation unit sees strtol redirected to __isoc23_strtol.
    fn __isoc23_strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
}

#[no_mangle]
pub extern "C" fn main__part3496(
    argc: c_int,
    argv: *mut *mut c_char,
    blocksize: *mut c_int,
    optind: *mut c_int,
) {
    unsafe {
        let idx = *optind;
        if idx < argc {
            // post-increment of optind
            *optind = idx.wrapping_add(1);
            let arg: *mut c_char = *argv.offset(idx as isize);
            let v: c_long = __isoc23_strtol(arg as *const c_char, core::ptr::null_mut(), 0);
            *blocksize = v as c_int;
        }
    }
}
}

// ── `blocksize *= 1024; buf = malloc(blocksize); f = fopen(infilename, "rb");` · splitimg.c:55-59 · chunk 3497 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     blocksize *= 1024;
//     
//         buf = malloc(blocksize);
//     
//         f = fopen(infilename, "rb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3497 {
// Port of splitimg.c main() lines 55-59:
//   blocksize *= 1024; buf = malloc(blocksize); f = fopen(infilename, "rb");
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    #[link_name = "fopen64"]
    fn fopen64(filename: *const c_char, modes: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn main__part3497(
    blocksize: *mut c_int,
    buf: *mut *mut u8,
    f: *mut *mut c_void,
    infilename: *const c_char,
) {
    let bs: c_int = (*blocksize).wrapping_mul(1024);
    *blocksize = bs;
    *buf = malloc(bs as usize) as *mut u8;
    let mode: &[u8; 3] = b"rb\0";
    *f = fopen64(infilename, mode.as_ptr() as *const c_char);
}
}

// ── `if (!f)` · splitimg.c:60-63 · chunk 3498 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(infilename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3498 {
// Port of the fopen-failure check of splitimg.c's main().
// Reports the error with perror() and tells the caller that exit(1) is due.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn perror(s: *const c_char);
}

#[no_mangle]
pub extern "C" fn main__part3498(f: *mut c_void, infilename: *const c_char) -> c_int {
    unsafe {
        if f.is_null() {
            perror(infilename);
            1
        } else {
            0
        }
    }
}
}

// ── `i = 0;` · splitimg.c:64-64 · chunk 3499 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     i = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3499 {
// Ported piece of main(): the loop counter initialisation `i = 0;`.
#[no_mangle]
pub extern "C" fn main__part3499(i: *mut core::ffi::c_int) {
    // Mirrors C's `i = 0;` through the pointer to the caller's local.
    unsafe {
        if !i.is_null() {
            *i = 0;
        }
    }
}
}

// ── `for (; ; )` · splitimg.c:65-86 · chunk 3500 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             ret = fread(buf, 1, blocksize, f);
//             if (ret < 0) {
//                 perror("fread");
//                 exit(1);
//             }
//             if (ret == 0)
//                 break;
//             if (ret < blocksize) {
//                 printf("warning: last block is not full\n");
//                 memset(buf + ret, 0, blocksize - ret);
//             }
//             snprintf(buf1, sizeof(buf1), "%s/blk%09u.bin", outpath, i);
//             fo = fopen(buf1, "wb");
//             if (!fo) {
//                 perror(buf1);
//                 exit(1);
//             }
//             fwrite(buf, 1, blocksize, fo);
//             fclose(fo);
//             i++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3500 {
// Port of the read/split loop of splitimg's main().
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn fread(ptr: *mut c_void, size: usize, n: usize, stream: *mut FILE) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, n: usize, stream: *mut FILE) -> usize;
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn main__part3500(
    buf: *mut u8,
    blocksize: c_int,
    f: *mut FILE,
    outpath: *const c_char,
    buf1: *mut c_char,
    i: *mut c_int,
) {
    unsafe {
        loop {
            let ret: c_int = fread(buf as *mut c_void, 1, blocksize as usize, f) as c_int;
            if ret < 0 {
                perror(b"fread\0".as_ptr() as *const c_char);
                exit(1);
            }
            if ret == 0 {
                break;
            }
            if ret < blocksize {
                printf(b"warning: last block is not full\n\0".as_ptr() as *const c_char);
                memset(
                    buf.offset(ret as isize) as *mut c_void,
                    0,
                    blocksize.wrapping_sub(ret) as usize,
                );
            }
            snprintf(
                buf1,
                1024usize,
                b"%s/blk%09u.bin\0".as_ptr() as *const c_char,
                outpath,
                *i,
            );
            let fo = fopen(buf1, b"wb\0".as_ptr() as *const c_char);
            if fo.is_null() {
                perror(buf1);
                exit(1);
            }
            fwrite(buf as *const c_void, 1, blocksize as usize, fo);
            fclose(fo);
            *i = (*i).wrapping_add(1);
        }
    }
}
}

// ── `fclose(f); printf("%d blocks\n", i); snprintf(buf1, sizeof(buf1), "%s/blk.txt", outpath); fo =…` · splitimg.c:87-91 · chunk 3501 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fclose(f);
//         printf("%d blocks\n", i);
//     
//         snprintf(buf1, sizeof(buf1), "%s/blk.txt", outpath);
//         fo = fopen(buf1, "wb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3501 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn fclose(stream: *mut c_void) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, fmt: *const c_char, ...) -> c_int;
    // glibc's <stdio.h> renames fopen to fopen64 through an asm label.
    #[link_name = "fopen64"]
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn main__part3501(
    f: *mut c_void,
    i: c_int,
    outpath: *const c_char,
    buf1: *mut c_char,
    fo_out: *mut *mut c_void,
) {
    unsafe {
        fclose(f);
        printf(b"%d blocks\n\0".as_ptr() as *const c_char, i);
        // sizeof(buf1) == 1024 in the caller
        snprintf(buf1, 1024, b"%s/blk.txt\0".as_ptr() as *const c_char, outpath);
        let fo = fopen(buf1, b"wb\0".as_ptr() as *const c_char);
        *fo_out = fo;
    }
}
}

// ── `if (!fo)` · splitimg.c:92-95 · chunk 3502 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!fo) {
//             perror(buf1);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3502 {
// Portion of main(): the "failed to open output file" check.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(code: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn main__part3502(fo: *mut c_void, buf1: *const c_char) {
    if fo.is_null() {
        unsafe {
            perror(buf1);
            exit(1);
        }
    }
}
}

// ── `fprintf(fo, "{\n"); fprintf(fo, " block_size: %d,\n", blocksize / 1024); fprintf(fo, " n_block:…` · splitimg.c:96-101 · chunk 3503 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fprintf(fo, "{\n");
//         fprintf(fo, "  block_size: %d,\n", blocksize / 1024);
//         fprintf(fo, "  n_block: %d,\n", i);
//         fprintf(fo, "}\n");
//         fclose(fo);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3503 {
// Port of splitimg.c lines 96-101: write the blk.txt descriptor and close it.
use core::ffi::{c_char, c_int};

// Opaque C `FILE`; only ever used behind a pointer.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
}

#[no_mangle]
pub extern "C" fn main__part3503(fo: *mut FILE, blocksize: c_int, i: c_int) -> c_int {
    unsafe {
        fprintf(fo, b"{\n\0".as_ptr() as *const c_char);
        fprintf(
            fo,
            b"  block_size: %d,\n\0".as_ptr() as *const c_char,
            blocksize / 1024,
        );
        fprintf(
            fo,
            b"  n_block: %d,\n\0".as_ptr() as *const c_char,
            i,
        );
        fprintf(fo, b"}\n\0".as_ptr() as *const c_char);
        fclose(fo);
    }
    0
}
}

