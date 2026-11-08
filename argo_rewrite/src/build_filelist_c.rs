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

// ── `for (; ; )` · build_filelist.c:136-207 · chunk 380 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             de = readdir(dirp);
//             if (!de)
//                 break;
//             name = de->d_name;
//             if (!strcmp(name, ".") || !strcmp(name, ".."))
//                 continue;
//             path1 = compose_path(path, name);
//             if (lstat(path1, &st) < 0) {
//                 perror(path1);
//                 exit(1);
//             }
//     
//             mode = st.st_mode & 0xffff;
//             fprintf(f, "%06o %u %u",
//                     mode,
//                     (int)st.st_uid,
//                     (int)st.st_gid);
//             if (
//                ((((
//                mode
//                )) & 0170000) == (0020000)) 
//                              || 
//                                 ((((
//                                 mode
//                                 )) & 0170000) == (0060000))
//                                              ) {
//                 fprintf(f, " %u %u",
//                         (int)
//                             gnu_dev_major (
//                             st.st_rdev
//                             )
//                                              ,
//                         (int)
//                             gnu_dev_minor (
//                             st.st_rdev
//                             )
//                                              );
//             }
//             if (
//                ((((
//                mode
//                )) & 0170000) == (0100000))
//                             ) {
//                 fprintf(f, " %" 
//                                "l" "u"
//                                      , st.st_size);
//             }
//     
//     
//     
//     
//     
//             fprintf(f, " %u", (int)st.st_mtim.tv_sec);
//             v = st.st_mtim.tv_nsec;
//     
//             if (v != 0) {
//                 fprintf(f, ".");
//                 while (v != 0) {
//                     fprintf(f, "%u", v / 100000000);
//                     v = (v % 100000000) * 10;
//                 }
//             }
//     
//             fprintf(f, " ");
//             print_str(f, name);
//             if (
//                ((((
//                mode
//                )) & 0170000) == (0120000))
//                             ) {
//                 char buf[1024];
//                 int len;
//                 len = readlink(path1, buf, sizeof(buf) - 1);
//                 if (len < 0) {
//                     perror("readlink");
//                     exit(1);
//                 }
//                 buf[len] = '\0';
//                 fprintf(f, " ");
//                 print_str(f, buf);
//             } else if (
//                       ((((
//                       mode
//                       )) & 0170000) == (0100000)) 
//                                     && st.st_size > 0) {
//                 char buf1[32], *fname;
//                 FSFileID file_id;
//                 file_id = s->next_inode_num++;
//                 fprintf(f, " %" 
//                                "l" "x"
//                                      , file_id);
//                 file_id_to_filename(buf1, file_id);
//                 fname = compose_path(s->files_path, buf1);
//                 copy_file(path1, fname);
//                 add_file_size(s, st.st_size);
//             }
//     
//             fprintf(f, "\n");
//             if (
//                ((((
//                mode
//                )) & 0170000) == (0040000))
//                             ) {
//                 scan_dir(s, path1);
//             }
//             free(path1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk380 {
// Rust port of one iteration of the directory scan loop of scan_dir().
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct ScanState {
    pub files_path: *mut c_char,
    pub next_inode_num: u64,
    pub fs_size: u64,
    pub fs_max_size: u64,
    pub f: *mut c_void,
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: i32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [i64; 3],
}

#[repr(C)]
struct Dirent {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 256],
}

extern "C" {
    #[link_name = "readdir64"]
    fn readdir(dirp: *mut c_void) -> *mut Dirent;
    #[link_name = "lstat64"]
    fn lstat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char;
    fn perror(s: *const c_char);
    fn exit(code: c_int);
    fn free(p: *mut c_void);
    fn fprintf(f: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn print_str(f: *mut c_void, s: *const c_char);
    fn gnu_dev_major(dev: u64) -> c_uint;
    fn gnu_dev_minor(dev: u64) -> c_uint;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn file_id_to_filename(buf: *mut c_char, file_id: u64) -> *mut c_char;
    fn scan_dir(s: *mut ScanState, path: *const c_char);
    fn malloc(size: usize) -> *mut u8;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fclose(stream: *mut c_void) -> c_int;
    static mut stderr: *mut c_void;
}

const COPY_BUF_LEN: usize = 1024 * 1024;

unsafe fn die(code: c_int) -> ! {
    exit(code);
    loop {}
}

// port of the static C helper copy_file()
unsafe fn copy_file_rs(src_filename: *const c_char, dst_filename: *const c_char) {
    let buf = malloc(COPY_BUF_LEN);
    let fi = fopen(src_filename, b"rb\0".as_ptr() as *const c_char);
    if fi.is_null() {
        perror(src_filename);
        die(1);
    }
    let fo = fopen(dst_filename, b"wb\0".as_ptr() as *const c_char);
    if fo.is_null() {
        perror(dst_filename);
        die(1);
    }
    loop {
        let len = fread(buf as *mut c_void, 1, COPY_BUF_LEN, fi) as c_int;
        if len == 0 {
            break;
        }
        fwrite(buf as *const c_void, 1, len as usize, fo);
    }
    fclose(fo);
    fclose(fi);
}

// port of the static C helper block_align()
fn block_align(val: u64, align: u64) -> u64 {
    val.wrapping_add(align - 1) & !(align - 1)
}

// port of the static C helper add_file_size()
unsafe fn add_file_size_rs(s: *mut ScanState, size: u64) {
    (*s).fs_size = (*s).fs_size.wrapping_add(block_align(size, 1u64 << 12));
    if (*s).fs_size > (*s).fs_max_size {
        let se: *mut c_void = core::ptr::addr_of!(stderr).read();
        fprintf(
            se,
            b"Filesystem Quota exceeded (%ld bytes)\n\0".as_ptr() as *const c_char,
            (*s).fs_max_size,
        );
        die(1);
    }
}

// returns 1 when the readdir() loop must stop, 0 otherwise
#[no_mangle]
pub extern "C" fn scan_dir__part380(
    s: *mut ScanState,
    path: *const c_char,
    dirp: *mut c_void,
) -> c_int {
    unsafe {
        let de = readdir(dirp);
        if de.is_null() {
            return 1;
        }
        let name: *const c_char = (*de).d_name.as_ptr();
        if strcmp(name, b".\0".as_ptr() as *const c_char) == 0
            || strcmp(name, b"..\0".as_ptr() as *const c_char) == 0
        {
            return 0;
        }
        let path1 = compose_path(path, name);
        let mut st: Stat = core::mem::zeroed();
        if lstat(path1, &mut st) < 0 {
            perror(path1);
            die(1);
        }

        let mode: u32 = st.st_mode & 0xffff;
        let f: *mut c_void = (*s).f;
        fprintf(
            f,
            b"%06o %u %u\0".as_ptr() as *const c_char,
            mode,
            st.st_uid as c_int,
            st.st_gid as c_int,
        );
        let fmt: u32 = mode & 0o170000;
        if fmt == 0o020000 || fmt == 0o060000 {
            fprintf(
                f,
                b" %u %u\0".as_ptr() as *const c_char,
                gnu_dev_major(st.st_rdev) as c_int,
                gnu_dev_minor(st.st_rdev) as c_int,
            );
        }
        if fmt == 0o100000 {
            fprintf(f, b" %lu\0".as_ptr() as *const c_char, st.st_size);
        }

        fprintf(
            f,
            b" %u\0".as_ptr() as *const c_char,
            st.st_mtim.tv_sec as c_int,
        );
        let mut v: u32 = st.st_mtim.tv_nsec as u32;

        if v != 0 {
            fprintf(f, b".\0".as_ptr() as *const c_char);
            while v != 0 {
                fprintf(f, b"%u\0".as_ptr() as *const c_char, v / 100000000);
                v = (v % 100000000).wrapping_mul(10);
            }
        }

        fprintf(f, b" \0".as_ptr() as *const c_char);
        print_str(f, name);
        if fmt == 0o120000 {
            let mut buf: [c_char; 1024] = [0; 1024];
            let len = readlink(path1, buf.as_mut_ptr(), 1024 - 1) as c_int;
            if len < 0 {
                perror(b"readlink\0".as_ptr() as *const c_char);
                die(1);
            }
            *buf.as_mut_ptr().offset(len as isize) = 0;
            fprintf(f, b" \0".as_ptr() as *const c_char);
            print_str(f, buf.as_ptr());
        } else if fmt == 0o100000 && st.st_size > 0 {
            let mut buf1: [c_char; 32] = [0; 32];
            let file_id: u64 = (*s).next_inode_num;
            (*s).next_inode_num = file_id.wrapping_add(1);
            fprintf(f, b" %lx\0".as_ptr() as *const c_char, file_id);
            file_id_to_filename(buf1.as_mut_ptr(), file_id);
            let fname = compose_path((*s).files_path, buf1.as_ptr());
            copy_file_rs(path1, fname);
            add_file_size_rs(s, st.st_size as u64);
        }

        fprintf(f, b"\n\0".as_ptr() as *const c_char);
        if fmt == 0o040000 {
            scan_dir(s, path1);
        }
        free(path1 as *mut c_void);
        0
    }
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

// ── `src_path = argv[optind]; dst_path = argv[optind + 1]; mkdir(dst_path, 0755); s->files_path = co…` · build_filelist.c:258-273 · chunk 388 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     src_path = argv[optind];
//         dst_path = argv[optind + 1];
//     
//         mkdir(dst_path, 0755);
//     
//         s->files_path = compose_path(dst_path, "files");
//         s->next_inode_num = first_inode;
//         s->fs_size = 0;
//         s->fs_max_size = fs_max_size;
//     
//         mkdir(s->files_path, 0755);
//     
//         root_id = s->next_inode_num++;
//         file_id_to_filename(fname, root_id);
//         filename = compose_path(s->files_path, fname);
//         f = fopen(filename, "wb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk388 {
// Ported piece of main__part388 from build_filelist.c
// SYMBOL: main__part388
// This replaces the TARGET in main(): from src_path = argv[optind] through f = fopen(filename, "wb")

use std::os::raw::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn compose_path(dst: *const c_char, suffix: *const c_char) -> *mut c_char;
    fn file_id_to_filename(buf: *mut c_char, file_id: c_ulong);
    fn mkdir(path: *const c_char, mode: c_int);
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int);
}

#[repr(C)]
struct ScanState {
    files_path: *mut c_char,
    next_inode_num: c_ulong,
    fs_size: c_ulong,
    fs_max_size: c_ulong,
    f: *mut c_void,
}

#[no_mangle]
pub extern "C" fn main__part388(
    argv: *const *const c_char,
    optind: c_int,
    // Output pointers for the C code to read after the call
    src_path: *mut *const c_char,
    dst_path: *mut *const c_char,
    // ScanState pointer (already dereferenced by caller if needed)
    s: *mut ScanState,
    // Values from main's local scope
    first_inode: c_ulong,
    fs_max_size: c_ulong,
    // Output pointers for derived values
    root_id: *mut c_ulong,
    fname: *mut c_char,
    filename: *mut *mut c_char,
    // FILE pointer output
    f: *mut *mut c_void,
) {
    // 1. src_path = argv[optind]
    unsafe {
        let src = *argv.offset(optind as isize);
        *src_path = src;
    }

    // 2. dst_path = argv[optind + 1]
    unsafe {
        let dst = *argv.offset((optind + 1) as isize);
        *dst_path = dst;
    }

    // 3. mkdir(dst_path, 0755)
    unsafe {
        let dst_for_mkdir = *dst_path;
        mkdir(dst_for_mkdir, 0o755);
    }

    // 4. s->files_path = compose_path(dst_path, "files")
    unsafe {
        let files_path = compose_path(*dst_path, b"files\0".as_ptr() as *const c_char);
        (*s).files_path = files_path;
    }

    // 5. s->next_inode_num = first_inode
    unsafe {
        (*s).next_inode_num = first_inode;
    }

    // 6. s->fs_size = 0
    unsafe {
        (*s).fs_size = 0;
    }

    // 7. s->fs_max_size = fs_max_size
    unsafe {
        (*s).fs_max_size = fs_max_size;
    }

    // 8. mkdir(s->files_path, 0755)
    unsafe {
        mkdir((*s).files_path, 0o755);
    }

    // 9. root_id = s->next_inode_num++
    unsafe {
        let current_next = (*s).next_inode_num;
        (*s).next_inode_num = current_next + 1;
        *root_id = current_next;
    }

    // 10. file_id_to_filename(fname, root_id)
    unsafe {
        file_id_to_filename(fname, *root_id);
    }

    // 11. filename = compose_path(s->files_path, fname)
    unsafe {
        let fname_val = fname as *const c_char;
        let filename_result = compose_path((*s).files_path, fname_val);
        *filename = filename_result;
    }

    // 12. f = fopen(filename, "wb")
    unsafe {
        let filename_c = *filename;
        let mode = b"wb\0".as_ptr() as *const c_char;
        *f = fopen(filename_c, mode);
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

// ── `fprintf(f, "Version: 1\n"); fprintf(f, "Revision: 1\n"); fprintf(f, "NextFileID: %" "l" "x" "\n…` · build_filelist.c:300-312 · chunk 394 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fprintf(f, "Version: 1\n");
//         fprintf(f, "Revision: 1\n");
//         fprintf(f, "NextFileID: %" 
//                                   "l" "x" 
//                                          "\n", s->next_inode_num);
//         fprintf(f, "FSFileCount: %" 
//                                    "l" "u" 
//                                           "\n", s->next_inode_num - 1);
//         fprintf(f, "FSSize: %" 
//                               "l" "u" 
//                                      "\n", s->fs_size);
//         fprintf(f, "FSMaxSize: %" 
//                                  "l" "u" 
//                                         "\n", s->fs_max_size);
//         fprintf(f, "Key:\n");
//         fprintf(f, "RootID: %" 
//                               "l" "x" 
//                                      "\n", root_id);
//         fclose(f);
//         free(filename);
//     
//         filename = compose_path(dst_path, "lock");
//         f = fopen(filename, "wb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk394 {
// Port of the head-file writing block from main().
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ScanState {
    pub files_path: *mut c_char,
    pub next_inode_num: u64,
    pub fs_size: u64,
    pub fs_max_size: u64,
    pub f: *mut FILE,
}

extern "C" {
    fn fprintf(f: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fclose(f: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn main__part394(
    pf: *mut *mut FILE,
    dst_path: *const c_char,
    pfilename: *mut *mut c_char,
    s: *mut ScanState,
    root_id: u64,
) {
    let f: *mut FILE = *pf;

    fprintf(f, b"Version: 1\n\0".as_ptr() as *const c_char);
    fprintf(f, b"Revision: 1\n\0".as_ptr() as *const c_char);
    fprintf(
        f,
        b"NextFileID: %lx\n\0".as_ptr() as *const c_char,
        (*s).next_inode_num,
    );
    fprintf(
        f,
        b"FSFileCount: %lu\n\0".as_ptr() as *const c_char,
        (*s).next_inode_num.wrapping_sub(1),
    );
    fprintf(
        f,
        b"FSSize: %lu\n\0".as_ptr() as *const c_char,
        (*s).fs_size,
    );
    fprintf(
        f,
        b"FSMaxSize: %lu\n\0".as_ptr() as *const c_char,
        (*s).fs_max_size,
    );
    fprintf(f, b"Key:\n\0".as_ptr() as *const c_char);
    fprintf(
        f,
        b"RootID: %lx\n\0".as_ptr() as *const c_char,
        root_id,
    );

    fclose(f);
    free(*pfilename as *mut c_void);

    let filename: *mut c_char = compose_path(dst_path, b"lock\0".as_ptr() as *const c_char);
    *pfilename = filename;
    *pf = fopen(filename, b"wb\0".as_ptr() as *const c_char);
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

