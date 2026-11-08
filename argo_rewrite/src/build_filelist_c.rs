//! From `build_filelist.c`.

// ── `const char *s; int c; s = str;` · build_filelist.c:48-50 · chunk 358 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     const char *s;
//         int c;
//         s = str;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk358 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn print_str__part358(s: *const c_char) -> *const c_char {
    s
}
}

// ── `while (*s != '\0')` · build_filelist.c:51-55 · chunk 359 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     while (*s != '\0') {
//             if (*s <= ' ' || *s > '~')
//                 goto use_quote;
//             s++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk359 {
use core::ffi::c_char;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn print_str__part359(s: *const c_char) -> c_int {
    let mut p = s;
    while !p.is_null() {
        let c = unsafe { *p };
        if c == 0 {
            return 0;
        }
        if c as i32 <= 32 || c as i32 > 126 {
            return 1;
        }
        unsafe { p = p.add(1); }
    }
    0
}
}

// ── `fputs(str, f); return;` · build_filelist.c:56-57 · chunk 360 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fputs(str, f);
//         return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk360 {
use core::ffi::c_char;
use core::ffi::c_int;

extern "C" {
    fn fputs(str: *const c_char, stream: *mut core::ffi::c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn print_str__part360(f: *mut core::ffi::c_void, str: *const c_char) {
    unsafe {
        fputs(str, f);
    }
}
}

// ── `use_quote: s = str;` · build_filelist.c:58-59 · chunk 361 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     use_quote:
//         s = str;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk361 {
use std::ffi::c_char;

#[no_mangle]
pub extern "C" fn print_str__part361(str: *const c_char) {}
}

// ── `fputc('"', f);` · build_filelist.c:60-60 · chunk 362 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fputc('"', f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk362 {
use std::os::raw::{c_int, c_void};

extern "C" {
    fn fputc(c: c_int, f: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn print_str__part362(f: *mut c_void, c: c_int) {
    unsafe {
        let _ = fputc(c, f);
    }
}
}

// ── `while (*s != '\0')` · build_filelist.c:61-71 · chunk 363 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     while (*s != '\0') {
//             c = *(uint8_t *)s;
//             if (c < ' ' || c == 127) {
//                 fprintf(f, "\\x%02x", c);
//             } else if (c == '\\' || c == '\"') {
//                 fprintf(f, "\\%c", c);
//             } else {
//                 fputc(c, f);
//             }
//             s++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk363 {
use std::os::raw::{c_char, c_int, c_uchar, c_void};

extern "C" {
    fn fputc(c: c_int, f: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn print_str__part363(s: *const c_char, f: *mut c_void) {
    let mut addr: usize = s as usize;
    while addr < usize::MAX {
        let p = addr as *const c_char;
        let c;
        unsafe {
            c = *p as c_uchar as c_int;
        }
        if c < ' ' as c_int || c == 127 {
            unsafe {
                fputc('\\' as c_int, f);
                fputc('x' as c_int, f);
            }
            let high = (c >> 4) & 0xf;
            let low = c & 0xf;
            unsafe {
                fputc(if high < 10 { '0' as c_int + high } else { 'A' as c_int + high - 10 }, f);
                fputc(if low < 10 { '0' as c_int + low } else { 'A' as c_int + low - 10 }, f);
            }
        } else if c == '\\' as c_int || c == '"' as c_int {
            unsafe {
                fputc('\\' as c_int, f);
                fputc(c as c_int, f);
            }
        } else {
            unsafe {
                fputc(c, f);
            }
        }
        addr = addr.wrapping_add(1);
    }
}
}

// ── `fputc('"', f);` · build_filelist.c:72-72 · chunk 364 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fputc('"', f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk364 {
extern "C" {
    fn fputc(c: i32, stream: *mut std::os::raw::c_void) -> i32;
}

#[no_mangle]
pub extern "C" fn print_str__part364(f: *mut std::os::raw::c_void) {
    unsafe {
        fputc('"' as i32, f);
    }
}
}

// ── `uint8_t *buf; FILE *fi, *fo; int len; buf = malloc((1024 * 1024)); fi = fopen(src_filename, "rb…` · build_filelist.c:79-84 · chunk 366 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint8_t *buf;
//         FILE *fi, *fo;
//         int len;
//     
//         buf = malloc((1024 * 1024));
//         fi = fopen(src_filename, "rb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk366 {
use core::ffi::c_uchar;
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn copy_file__part366(
    buf: *mut *mut c_uchar,
    fi: *mut *mut c_void,
    len: *mut c_int,
    src_filename: *const c_char,
) {
    unsafe {
        *buf = malloc((1024 * 1024) as usize) as *mut c_uchar;
        let mode = "rb\0".as_ptr() as *const c_char;
        *fi = fopen(src_filename, mode);
        *len = 0;
    }
}
}

// ── `if (!fi)` · build_filelist.c:85-88 · chunk 367 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!fi) {
//             perror(src_filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk367 {
use std::os::raw::c_char;
use std::os::raw::c_int;

extern "C" {
    pub fn perror(s: *const c_char);
    pub fn exit(code: c_int);
}

#[no_mangle]
pub extern "C" fn copy_file__part367(src_filename: *const c_char) {
    unsafe {
        perror(src_filename);
        exit(1);
    }
}
}

// ── `fo = fopen(dst_filename, "wb");` · build_filelist.c:89-89 · chunk 368 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     fo = fopen(dst_filename, "wb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk368 {
use std::ffi::c_char;
use std::os::raw::c_void;

extern "C" {
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn copy_file__part368(dst_filename: *const c_char) -> *mut c_void {
    unsafe {
        let mode: *const c_char = b"wb\0".as_ptr() as *const c_char;
        fopen(dst_filename, mode)
    }
}
}

// ── `if (!fo)` · build_filelist.c:90-93 · chunk 369 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (!fo) {
//             perror(dst_filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk369 {
use std::ffi::c_char;
use std::ffi::c_void;
use std::os::raw::c_int;

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

#[no_mangle]
pub extern "C" fn copy_file__part369(fo: *mut c_void, dst_filename: *const c_char) {
    if fo.is_null() {
        unsafe {
            perror(dst_filename);
            exit(1);
        }
    }
}
}

// ── `for (; ; )` · build_filelist.c:94-99 · chunk 370 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             len = fread(buf, 1, (1024 * 1024), fi);
//             if (len == 0)
//                 break;
//             fwrite(buf, 1, len, fo);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk370 {
use core::ffi::{c_uchar, c_ulong, c_void};

extern "C" {
    fn fread(ptr: *mut c_void, size: c_ulong, nmemb: c_ulong, stream: *mut c_void) -> c_ulong;
    fn fwrite(ptr: *const c_void, size: c_ulong, nmemb: c_ulong, stream: *mut c_void) -> c_ulong;
}

#[no_mangle]
pub extern "C" fn copy_file__part370(buf: *mut c_uchar, fi: *mut c_void, fo: *mut c_void) {
    unsafe {
        loop {
            let len = fread(buf as *mut c_void, 1 as c_ulong, (1024 * 1024) as c_ulong, fi);
            if len == 0 {
                break;
            }
            fwrite(buf as *const c_void, 1 as c_ulong, len as c_ulong, fo);
        }
    }
}
}

// ── `fclose(fo); fclose(fi);` · build_filelist.c:100-101 · chunk 371 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fclose(fo);
//         fclose(fi);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk371 {
use core::ffi::c_int;
use core::ffi::c_void;

extern "C" {
    fn fclose(stream: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn copy_file__part371(fo: *mut c_void, fi: *mut c_void) {
    unsafe {
        fclose(fo);
        fclose(fi);
    }
}
}

// ── `s->fs_size += block_align(size, (1 << 12));` · build_filelist.c:114-114 · chunk 375 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     s->fs_size += block_align(size, (1 << 12));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk375 {
use core::ffi::c_ulonglong;
use core::ffi::c_char;
use core::ffi::c_void;

#[repr(C)]
struct ScanState {
    files_path: *mut c_char,
    next_inode_num: c_ulonglong,
    fs_size: c_ulonglong,
    fs_max_size: c_ulonglong,
    f: *mut c_void,
}

#[no_mangle]
pub extern "C" fn add_file_size__part375(s: *mut ScanState, size: c_ulonglong) {
    unsafe {
        let align: c_ulonglong = 4096;
        let mask = ! (align - 1);
        (*s).fs_size = (size + align - 1) & mask;
    }
}
}

// ── `if (s->fs_size > s->fs_max_size)` · build_filelist.c:115-118 · chunk 376 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->fs_size > s->fs_max_size) {
//             fprintf(
//                    stderr
//                          , "Filesystem Quota exceeded (%" 
//                                                           "l" "d" 
//                                                                  " bytes)\n", s->fs_max_size);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk376 {
use std::os::raw::{c_int, c_ulong};

#[no_mangle]
pub extern "C" fn add_file_size__part376(fs_size: c_ulong, fs_max_size: c_ulong) -> c_int {
    if fs_size > fs_max_size {
        1
    } else {
        0
    }
}
}

// ── `FILE *f = s->f; DIR *dirp; struct dirent *de; const char *name; struct stat st; char *path1; ui…` · build_filelist.c:123-131 · chunk 378 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     FILE *f = s->f;
//         DIR *dirp;
//         struct dirent *de;
//         const char *name;
//         struct stat st;
//         char *path1;
//         uint32_t mode, v;
//     
//         dirp = opendir(path);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk378 {
use std::os::raw::c_char;
use std::os::raw::c_void;

extern "C" {
    fn opendir(path: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn scan_dir__part378(path: *const c_char) -> *mut c_void {
    unsafe {
        opendir(path)
    }
}
}

// ── `if (!dirp)` · build_filelist.c:132-135 · chunk 379 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (!dirp) {
//             perror(path);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk379 {
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

extern "C" {
    fn opendir(name: *const c_char) -> *mut c_void;
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

#[no_mangle]
pub extern "C" fn scan_dir__part379(path: *const c_char) -> *mut c_void {
    let dirp;
    unsafe {
        dirp = opendir(path);
        if dirp.is_null() {
            perror(path);
            exit(1);
        }
    }
    dirp
}
}

// ── `closedir(dirp); fprintf(f, ".\n");` · build_filelist.c:209-210 · chunk 381 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     closedir(dirp);
//         fprintf(f, ".\n");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk381 {
use core::ffi::c_int;
use core::ffi::c_void;

extern "C" {
    fn closedir(dirp: *mut c_void) -> c_int;
    fn fputc(c: c_int, stream: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn scan_dir__part381(dirp: *mut c_void, f: *mut c_void) -> c_int {
    unsafe {
        let _ = closedir(dirp);
        let _ = fputc(0x2e, f);  // print '.'
        let _ = fputc(0x0a, f);  // print '\n'
    }
    0i32 as c_int
}
}

// ── `printf("usage: build_filelist [options] source_path dest_path\n" "\n" "Options:\n" "-m size_mb…` · build_filelist.c:215-219 · chunk 383 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     printf("usage: build_filelist [options] source_path dest_path\n"
//                "\n"
//                "Options:\n"
//                "-m size_mb  set the max filesystem size in MiB\n");
//         exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk383 {
#[no_mangle]
pub extern "C" fn help__part383() {
    println!("usage: build_filelist [options] source_path dest_path");
    println!();
    println!("Options:");
    println!("-m size_mb  set the max filesystem size in MiB");
    std::process::exit(1);
}
}

// ── `const char *dst_path, *src_path; ScanState s_s, *s = &s_s; FILE *f; char *filename; FSFileID ro…` · build_filelist.c:226-237 · chunk 385 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *dst_path, *src_path;
//         ScanState s_s, *s = &s_s;
//         FILE *f;
//         char *filename;
//         FSFileID root_id;
//         char fname[32];
//         struct stat st;
//         uint64_t first_inode, fs_max_size;
//         int c;
//     
//         first_inode = 1;
//         fs_max_size = (uint64_t)1 << 30;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk385 {
use core::ffi::c_char;
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn main__part385(
    dst_path: *const c_char,
    src_path: *const c_char,
    first_inode: *mut c_ulong,
    fs_max_size: *mut c_ulong,
) {
    unsafe {
        *first_inode = 1;
        *fs_max_size = 1 << 30;
    }
}
}

// ── `for (; ; )` · build_filelist.c:238-254 · chunk 386 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             c = getopt(argc, argv, "hi:m:");
//             if (c == -1)
//                 break;
//             switch(c) {
//             case 'h':
//                 help();
//             case 'i':
//                 first_inode = strtoul(optarg, 
//                                              ((void *)0)
//                                                  , 0);
//                 break;
//             case 'm':
//                 fs_max_size = (uint64_t)strtoul(optarg, 
//                                                        ((void *)0)
//                                                            , 0) << 20;
//                 break;
//             default:
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk386 {
use core::ffi::c_int;
use core::ffi::c_ulong;
use core::ffi::c_char;
use std::ffi::CString;
use core::ptr::null_mut;

extern "C" {
    static optarg: *mut c_char;
    fn getopt(___argc: c_int, ___argv: *const *const c_char, __shortopts: *const c_char) -> c_int;
    fn help();
    fn exit(__status: c_int);
    fn strtoul(__nptr: *const c_char, __endptr: *mut *const c_char, __base: c_int) -> c_ulong;
}

#[no_mangle]
pub extern "C" fn main__part386(
    argc: c_int,
    argv: *const *const c_char,
    first_inode: *mut c_ulong,
    fs_max_size: *mut c_ulong
) -> c_int {
    loop {
        let optstring = CString::new("hi:m:").unwrap().as_ptr();
        let c = unsafe { getopt(argc, argv, optstring) };
        if c == -1 {
            return 0;
        }
        unsafe {
            match c {
                _ if c == 'h' as c_int => {
                    help();
                    let val = strtoul(optarg, null_mut(), 0);
                    *first_inode = val as c_ulong;
                }
                _ if c == 'i' as c_int => {
                    let val = strtoul(optarg, null_mut(), 0);
                    *first_inode = val as c_ulong;
                }
                _ if c == 'm' as c_int => {
                    let val = strtoul(optarg, null_mut(), 0);
                    *fs_max_size = (val as c_ulong) << 20;
                }
                _ => {
                    return 1;
                }
            }
        }
    }
}
}

// ── `if (optind + 1 >= argc)` · build_filelist.c:256-257 · chunk 387 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (optind + 1 >= argc)
//             help();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk387 {
use core::ffi::c_int;

extern "C" {
    fn help();
}

#[no_mangle]
pub extern "C" fn main__part387(optind: c_int, argc: c_int) {
    if optind + 1 >= argc {
        unsafe { help() }
    }
}
}

// ── `if (!f)` · build_filelist.c:274-277 · chunk 389 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk389 {
use core::ffi::c_char;
use core::ffi::c_int;

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

#[no_mangle]
pub extern "C" fn main__part389(filename: *const c_char) -> c_int {
    unsafe {
        perror(filename);
        exit(1);
    }
    unreachable!()
}
}

// ── `fprintf(f, "Version: 1\n"); fprintf(f, "Revision: 1\n"); fprintf(f, "\n"); s->f = f; scan_dir(s…` · build_filelist.c:278-283 · chunk 390 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fprintf(f, "Version: 1\n");
//         fprintf(f, "Revision: 1\n");
//         fprintf(f, "\n");
//         s->f = f;
//         scan_dir(s, src_path);
//         fclose(f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk390 {
use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
struct ScanState {
    files_path: *mut c_char,
    next_inode_num: u64,
    fs_size: u64,
    fs_max_size: u64,
    f: *mut c_void,
}

extern "C" {
    fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
    fn fclose(stream: *mut c_void) -> c_int;
    fn scan_dir(s: *mut ScanState, path: *const c_char);
}

#[no_mangle]
pub extern "C" fn main__part390(f: *mut c_void, s: *mut ScanState, src_path: *const c_char) {
    unsafe {
        fputs(b"Version: 1\n\0".as_ptr() as *const c_char, f);
        fputs(b"Revision: 1\n\0".as_ptr() as *const c_char, f);
        fputs(b"\n\0".as_ptr() as *const c_char, f);

        if !s.is_null() {
            let state = &mut *s;
            state.f = f;
        }

        if !s.is_null() {
            scan_dir(s, src_path);
        }

        fclose(f);
    }
}
}

// ── `if (stat(filename, &st) < 0)` · build_filelist.c:286-289 · chunk 391 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (stat(filename, &st) < 0) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk391 {
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_ulong;
use core::ffi::c_uint;

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [c_long; 3],
}

extern "C" {
    fn stat(filename: *const c_char, buf: *mut stat) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int);
}

#[no_mangle]
pub extern "C" fn main__part391(filename: *const c_char, st: *mut stat) -> c_int {
    unsafe {
        if stat(filename, st) < 0 {
            perror(filename);
            exit(1);
        }
        0
    }
}
}

// ── `add_file_size(s, st.st_size); free(filename); filename = compose_path(dst_path, "head"); f = fo…` · build_filelist.c:290-295 · chunk 392 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     add_file_size(s, st.st_size);
//     
//         free(filename);
//     
//         filename = compose_path(dst_path, "head");
//         f = fopen(filename, "wb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk392 {
use std::ffi::{c_char, c_void};
use std::process;

#[repr(C)]
struct ScanState {
    files_path: *mut c_char,
    next_inode_num: u64,
    fs_size: u64,
    fs_max_size: u64,
    f: *mut c_void,
}

#[no_mangle]
pub extern "C" fn main__part392(s: *mut ScanState, size: u64) {
    let s = unsafe { &mut *s };
    let aligned = (size + 4095) & !4095;
    s.fs_size = s.fs_size + aligned;
    if s.fs_size > s.fs_max_size {
        eprintln!("Filesystem Quota exceeded ({}) bytes", s.fs_max_size);
        process::exit(1);
    }
}
}

// ── `if (!f)` · build_filelist.c:296-299 · chunk 393 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk393 {
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    pub fn perror(s: *const c_char);
    pub fn exit(code: c_int);
}

#[no_mangle]
pub extern "C" fn main__part393(f: *mut c_void, filename: *const c_char) {
    if f.is_null() {
        unsafe {
            perror(filename);
            exit(1);
        }
    }
}
}

// ── `if (!f)` · build_filelist.c:313-316 · chunk 395 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk395 {
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn main__part395(f: *const std::os::raw::c_void) -> c_int {
    if f.is_null() {
        1i32
    } else {
        0i32
    }
}
}

// ── `fclose(f); free(filename); return 0;` · build_filelist.c:317-320 · chunk 396 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     fclose(f);
//         free(filename);
//     
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk396 {
use core::ffi::{c_int, c_char, c_void};

extern "C" {
    fn fclose(fp: *mut c_void) -> c_int;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn main__part396(f: *mut c_void, filename: *mut c_char) -> c_int {
    unsafe {
        fclose(f);
        free(filename as *mut c_void);
    }
    0
}
}

