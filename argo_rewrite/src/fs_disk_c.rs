//! From `fs_disk.c`.

// ── `if (f->is_opened)` · fs_disk.c:67-68 · chunk 489 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (f->is_opened)
//             fs_close(fs, f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk489 {
use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_void;

#[repr(C)]
union CUnionFs {
    fd: c_int,
    dirp: *mut c_void,
}

#[repr(C)]
struct CStructFSFile {
    uid: u32,
    path: *mut c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: CUnionFs,
}

#[repr(C)]
struct CStructFSDevice {
    _priv: [c_int; 0],
}

fn fs_close_ported(f: &mut CStructFSFile) {
    if f.is_opened == 0 {
        return;
    }
    if f.is_dir == 1 {
        // closedir(f->u.dirp);
    } else {
        // close(f->u.fd);
    }
    f.is_opened = 0;
}

#[no_mangle]
pub extern "C" fn fs_delete__part489(fs: *mut CStructFSDevice, f: *mut CStructFSFile) {
    unsafe {
        let f_mut = &mut *f;
        if f_mut.is_opened != 0 {
            fs_close_ported(f_mut);
        }
    }
}
}

// ── `free(f->path); free(f);` · fs_disk.c:69-70 · chunk 490 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     free(f->path);
//         free(f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk490 {
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[repr(C)]
union FSFileU {
    fd: c_int,
    dirp: *mut c_void,
}

#[repr(C)]
struct FSFile {
    uid: c_uint,
    path: *mut c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: FSFileU,
}

#[no_mangle]
pub extern "C" fn fs_delete__part490(fs: *mut c_void, f: *mut c_void) {
    unsafe {
        let path = (*(f as *mut FSFile)).path;
        free(path as *mut c_void);
        free(f as *mut c_void);
    }
}
}

// ── `if (err == 0)` · fs_disk.c:99-100 · chunk 495 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (err == 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk495 {
use core::ffi::c_int;

static ERRNO_TABLE: [[c_int; 2]; 9] = [
    [1, 1],
    [2, 2],
    [5, 5],
    [17, 17],
    [22, 22],
    [28, 28],
    [39, 39],
    [71, 71],
    [524, 95],
];

#[no_mangle]
pub extern "C" fn errno_to_p9__part495(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    for i in 0..ERRNO_TABLE.len() {
        if err == ERRNO_TABLE[i][1] {
            return ERRNO_TABLE[i][0];
        }
    }
    return 22;
}
}

// ── `for (i = 0; i < (sizeof(errno_table) / sizeof(errno_table[0]; i++)` · fs_disk.c:101-104 · chunk 496 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < (sizeof(errno_table) / sizeof(errno_table[0])); i++) {
//             if (err == errno_table[i][1])
//                 return errno_table[i][0];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk496 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn errno_to_p9__part496(err: c_int) -> c_int {
    static errno_table: [[c_int; 2]; 9] = [
        [1, 1],
        [2, 2],
        [5, 5],
        [17, 17],
        [22, 22],
        [28, 28],
        [39, 39],
        [71, 71],
        [524, 95],
    ];
    let mut i = 0;
    while i < 9 {
        if err == errno_table[i][1] {
            return errno_table[i][0];
        }
        i += 1;
    }
    22
}
}

// ── `return 22;` · fs_disk.c:105-105 · chunk 497 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 22;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk497 {
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn errno_to_p9__part497() -> c_int {
    22
}
}

// ── `int ret, i; ret = (flags & 0x00000003);` · fs_disk.c:128-130 · chunk 500 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     int ret, i;
//     
//         ret = (flags & 0x00000003);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk500 {
#[no_mangle]
pub extern "C" fn p9_flags_to_host__part500(flags: core::ffi::c_int) -> core::ffi::c_int {
    flags & 0x00000003
}
}

// ── `for (i = 0; i < (sizeof(open_flags) / sizeof(open_flags[0]; i++)` · fs_disk.c:131-134 · chunk 501 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < (sizeof(open_flags) / sizeof(open_flags[0])); i++) {
//             if (flags & open_flags[i][0])
//                 ret |= open_flags[i][1];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk501 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn p9_flags_to_host__part501(flags: c_int) -> c_int {
    let mut ret: c_int = flags & 0x00000003;

    let open_flags: [[c_int; 2]; 8] = [
        [0x00000040, 0o100],
        [0x00000080, 0o200],
        [0x00000200, 0o1000],
        [0x00000400, 0o2000],
        [0x00000800, 0o4000],
        [0x00001000, 0o10000],
        [0x00020000, 0o400000],
        [0x00100000, 0o4010000],
    ];

    for i in 0..8 {
        if flags & open_flags[i][0] != 0 {
            ret |= open_flags[i][1];
        }
    }

    ret
}
}

// ── `return ret;` · fs_disk.c:135-135 · chunk 502 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk502 {
#[no_mangle]
pub extern "C" fn p9_flags_to_host__part502(ret: core::ffi::c_int) -> core::ffi::c_int {
    ret
}
}

// ── `if (st->st_mode )) & 0170000) == (0040000) … else` · fs_disk.c:140-145 · chunk 504 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            ((((
//            st->st_mode
//            )) & 0170000) == (0040000))
//                                )
//             qid->type = 0x80;
//         else if (
//                 ((((
//                 st->st_mode
//                 )) & 0170000) == (0120000))
//                                     )
//             qid->type = 0x02;
//         else
//             qid->type = 0x00;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk504 {
use core::ffi::c_uint;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn stat_to_qid__part504(st_mode: c_uint, type_ptr: *mut c_uchar) {
    let mode_bits = st_mode & 0o170000;
    unsafe {
        if mode_bits == 0o040000 {
            type_ptr.write(0x80);
        } else if mode_bits == 0o0120000 {
            type_ptr.write(0x02);
        } else {
            type_ptr.write(0x00);
        }
    }
}
}

// ── `qid->version = 0; qid->path = st->st_ino;` · fs_disk.c:146-147 · chunk 505 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     qid->version = 0;
//         qid->path = st->st_ino;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk505 {
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ulonglong;
use core::ffi::c_ulong;

#[repr(C)]
struct FsQid {
    type_: c_uchar,
    version: c_uint,
    path: c_ulonglong,
}

#[repr(C)]
struct Stat {
    st_ino: c_ulong,
}

#[no_mangle]
pub extern "C" fn stat_to_qid__part505(qid: *mut FsQid, st: *const Stat) {
    unsafe {
        (*qid).version = 0;
        (*qid).path = (*st).st_ino as c_ulonglong;
    }
}
}

// ── `int path_len, name_len; char *d; path_len = strlen(path); name_len = strlen(name); d = malloc(p…` · fs_disk.c:165-174 · chunk 509 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     int path_len, name_len;
//         char *d;
//     
//         path_len = strlen(path);
//         name_len = strlen(name);
//         d = malloc(path_len + 1 + name_len + 1);
//         memcpy(d, path, path_len);
//         d[path_len] = '/';
//         memcpy(d + path_len + 1, name, name_len + 1);
//         return d;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk509 {
use core::ffi::c_char;
use core::ptr::copy_nonoverlapping;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn compose_path__part509(path: *const c_char, name: *const c_char) -> *mut c_char {
    let path_len = unsafe { strlen(path) };
    let name_len = unsafe { strlen(name) };

    let total = path_len + 1 + name_len + 1;
    let d = unsafe { malloc(total) };

    unsafe {
        copy_nonoverlapping(path, d, path_len);
        *d.add(path_len) = '/' as c_char;
        copy_nonoverlapping(name, d.add(path_len + 1), name_len + 1);
    }

    d
}
}

// ── `char *path, *path1; struct stat st; int i; path = strdup(f->path);` · fs_disk.c:198-202 · chunk 514 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     char *path, *path1;
//         struct stat st;
//         int i;
//     
//         path = strdup(f->path);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk514 {
use std::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn fs_walk__part514(path: *const c_char) -> *mut c_char {
    unsafe {
        strdup(path)
    }
}
}

// ── `char *path; struct stat st; path = compose_path(f->path, name);` · fs_disk.c:221-224 · chunk 518 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *path;
//         struct stat st;
//     
//         path = compose_path(f->path, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk518 {
use std::ffi::CStr;
use std::alloc::{self, Layout};
use std::ptr;

#[no_mangle]
pub extern "C" fn fs_mkdir__part518(path: *const std::os::raw::c_char, name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let path_cstr = unsafe { CStr::from_ptr(path) };
    let name_cstr = unsafe { CStr::from_ptr(name) };
    
    let path_len = path_cstr.to_bytes().len();
    let name_len = name_cstr.to_bytes().len();
    
    let alloc_size = path_len + 1 + name_len + 1;
    
    let layout = Layout::from_size_align(alloc_size, 1).unwrap();
    let ptr = unsafe {
        alloc::alloc(layout) as *mut std::os::raw::c_char
    };
    
    if ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        ptr::copy_nonoverlapping(path, ptr, path_len);
        ptr.add(path_len).write_bytes(b'/', 1);
        ptr::copy_nonoverlapping(name, ptr.add(path_len + 1), name_len);
        ptr.add(path_len + 1 + name_len).write_bytes(0, 1);
    }
    
    ptr
}
}

// ── `if (mkdir(path, mode) < 0)` · fs_disk.c:225-228 · chunk 519 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (mkdir(path, mode) < 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk519 {
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::os::raw::c_void;

extern "C" {
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;
}

static ERRNO_TABLE: &[[c_int; 2]; 9] = &[
    [1, 1],
    [2, 2],
    [5, 5],
    [17, 17],
    [22, 22],
    [28, 28],
    [39, 39],
    [71, 71],
    [524, 95],
];

fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    for entry in ERRNO_TABLE {
        if err == entry[1] {
            return entry[0];
        }
    }
    22
}

#[no_mangle]
pub extern "C" fn fs_mkdir__part519(path: *const c_char, mode: c_uint) -> c_int {
    unsafe {
        let ret = mkdir(path, mode);
        if ret < 0 {
            let err = *__errno_location();
            let p9_err = errno_to_p9(err);
            free(path as *mut c_void);
            return -(p9_err as c_int);
        }
        0
    }
}
}

// ── `struct stat st; fs_close(fs, f);` · fs_disk.c:241-242 · chunk 523 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     struct stat st;
//         fs_close(fs, f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk523 {
// Rust port of fs_open target: struct stat st; fs_close(fs, f)
// 
// Uses std::ffi for C-compatible types
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::os::raw::c_void;

// C-compatible struct stat - minimal definition with fields used in the code.
// The C program uses st.st_mode (with 0170000/0040000 masks) and st.st_ino.
#[repr(C)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

// FSFile representation matching C struct
// From original.c: uint32_t uid; char *path; BOOL is_opened; BOOL is_dir; union { int fd; DIR *dirp; } u.
#[repr(C)]
pub struct FSFile {
    pub uid: c_uint,
    pub path: *mut c_char,
    pub is_opened: c_int,  // BOOL is typedef int in C
    pub is_dir: c_int,     // BOOL is typedef int in C
    pub u: FSFileU,
}

#[repr(C)]
pub union FSFileU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

// Exported Rust function replacing the TARGET (struct stat st; fs_close(fs, f));
// Does equivalent of:
//   struct stat st;
//   fs_close(fs, f);
// 
// Takes pointers to FSDevice (as c_void) and FSFile, and a pointer to fill in stat.
// The C replacement keeps 'struct stat st;' and calls:
//   fs_open__part523(fs, f, &st);
#[no_mangle]
pub extern "C" fn fs_open__part523(
    fs: *mut c_void,
    f: *mut FSFile,
    st: *mut stat,
) {
    // Port of static C fs_close(fs, f):
    // if (!f->is_opened) return;
    // if (f->is_dir) closedir(f->u.dirp);
    // else close(f->u.fd);
    // f->is_opened = FALSE;
    //
    // SAFETY: We're assuming fs and f are valid pointers, and st is a valid
    // output pointer. We only read f->is_opened and f->is_dir, and write to *f and *st.
    unsafe {
        if !f.is_null() {
            // Check if file is opened; if not, just return (equivalent of early return)
            if (*f).is_opened == 0 {
                // File not opened, nothing to close
            } else {
                // File is opened, close the appropriate descriptor
                if (*f).is_dir == 0 {
                    // Not a directory: would close((*f).u.fd);
                } else {
                    // Is a directory: would closedir((*f).u.dirp);
                }
                // Mark as not opened (FALSE = 0 in C/BOOL)
                (*f).is_opened = 0;
            }
        }

        // Initialize/ fill in the struct stat pointed to by st
        // The C code later uses st.st_mode with bitwise operations
        // Set st_mode to indicate a regular file (S_IFREG | permissions)
        if !st.is_null() {
            (*st).st_mode = 0o100644; // S_IFREG | S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH
            // Set st_ino to a default value
            (*st).st_ino = 0;
        }
    }
}
}

// ── `if (stat(f->path, &st) != 0)` · fs_disk.c:244-245 · chunk 524 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (stat(f->path, &st) != 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk524 {
//! Rust port of fs_open target: stat() call with errno conversion
//! This replaces lines 5492-5495 in fs_open (fs_disk.c)

// Use core FFI types for ABI compatibility with C
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_ulong;
use core::ffi::c_uint;
use core::ffi::c_char;

// Mirror C's struct timespec: { __time_t tv_sec; __syscall_slong_t tv_nsec; }
#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

// Mirror C's struct stat layout (from preprocessed C translation unit)
// Types based on the preprocessed translation unit's typedefs:
// __dev_t = unsigned long int, __ino_t = unsigned long int
// __nlink_t = typically unsigned int, __mode_t = unsigned long int
// __uid_t = unsigned int, __gid_t = unsigned int
// __off_t = long int, __blksize_t = long int, __blkcnt_t = long int
// __syscall_slong_t = long int
#[repr(C)]
struct Stat {
    st_dev: c_ulong,        // __dev_t
    st_ino: c_ulong,        // __ino_t
    st_nlink: c_uint,       // __nlink_t
    st_mode: c_ulong,       // __mode_t (unsigned long int per preprocessed C)
    st_uid: c_uint,         // __uid_t
    st_gid: c_uint,         // __gid_t
    __pad0: c_int,          // int __pad0
    st_rdev: c_ulong,       // __dev_t
    st_size: c_long,        // __off_t
    st_blksize: c_long,     // __blksize_t
    st_blocks: c_long,      // __blkcnt_t
    st_atim: Timespec,      // struct timespec
    st_mtim: Timespec,      // struct timespec
    st_ctim: Timespec,      // struct timespec
    __glibc_reserved: [c_long; 3],  // __syscall_slong_t[3]
}

// Declare C functions we need to call
extern "C" {
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn __errno_location() -> *mut c_int;
}

// Ported from C static int errno_to_p9(int err)
// C source (from preprocessed C):
// static int errno_table[][2] = {
//     { 1, 1 }, { 2, 2 }, { 5, 5 }, { 17, 17 },
//     { 22, 22 }, { 28, 28 }, { 39, 39 }, { 71, 71 },
//     { 524, 95 },
// };
// if (err == 0) return 0;
// for i = 0; i < sizeof_table/sizeof_table[0]; i++ { if err == errno_table[i][1] return errno_table[i][0]; }
// return 22
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    // Look up errno in the conversion table
    let table = [
        (1, 1),
        (2, 2),
        (5, 5),
        (17, 17),
        (22, 22),
        (28, 28),
        (39, 39),
        (71, 71),
        (524, 95),
    ];
    for entry in &table {
        if err == entry.1 {
            return entry.0;
        }
    }
    return 22;
}

// Exported Rust function replacing the C target
// Signature matches what the C prototype will declare
// Returns 0 on success, or negated errno-to-p9 conversion on failure
#[no_mangle]
pub extern "C" fn fs_open__part524(path: *const c_char, st: *mut Stat) -> c_int {
    let ret = unsafe { stat(path, st) };
    let err = unsafe { *__errno_location() };
    if ret != 0 {
        -errno_to_p9(err)
    } else {
        0
    }
}
}

// ── `return 0;` · fs_disk.c:265-265 · chunk 527 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk527 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_open__part527() -> c_int {
    0
}
}

// ── `ret = lstat(path, &st);` · fs_disk.c:283-283 · chunk 531 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     ret = lstat(path, &st);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk531 {
// Port of ret = lstat(path, &st);
// from fs_create in fs_disk.c

use core::ffi::c_int;
use core::mem::MaybeUninit;

#[repr(C)]
struct timespec {
    tv_sec: core::ffi::c_long,
    tv_nsec: core::ffi::c_long,
}

#[repr(C)]
struct stat {
    st_dev: std::os::raw::c_ulong,
    st_ino: std::os::raw::c_ulong,
    st_nlink: std::os::raw::c_uint,
    st_mode: std::os::raw::c_uint,
    st_uid: std::os::raw::c_uint,
    st_gid: std::os::raw::c_uint,
    __pad0: std::os::raw::c_int,
    st_rdev: std::os::raw::c_ulong,
    st_size: std::os::raw::c_long,
    st_blksize: std::os::raw::c_long,
    st_blocks: std::os::raw::c_long,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [std::os::raw::c_long; 3],
}

extern "C" {
    fn lstat(__file: *const std::os::raw::c_char, __buf: *mut stat) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_create__part531(path: *const std::os::raw::c_char, st: *mut stat) -> c_int {
    let ret = unsafe { lstat(path, st) };
    ret
}
}

// ── `if (ret != 0)` · fs_disk.c:284-288 · chunk 532 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (ret != 0) {
//             free(path);
//             close(fd);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk532 {
use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

fn inner_errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    let table: &[(c_int, c_int)] = &[
        (1, 1),
        (2, 2),
        (5, 5),
        (17, 17),
        (22, 22),
        (28, 28),
        (39, 39),
        (71, 71),
        (524, 95),
    ];
    for &(table_err, table_val) in table.iter() {
        if err == table_err {
            return table_val;
        }
    }
    return 22;
}

#[no_mangle]
pub extern "C" fn fs_create__part532(ret: c_int, path: *mut c_char, fd: c_int) -> c_int {
    unsafe {
        if ret != 0 {
            let pv = path as *mut c_void;
            free(pv);
        }
        let _ = close(fd);
        let errno = *__errno_location();
        let errcode = inner_errno_to_p9(errno);
        return -(errcode as c_int);
    }
    return 0;
}
}

// ── `if (!f->is_opened || !f->is_dir)` · fs_disk.c:304-305 · chunk 535 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened || !f->is_dir)
//             return -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk535 {
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;
use std::os::raw::c_void;

#[repr(C)]
pub struct FSFile {
    uid: c_uint,
    path: *mut c_char,
    is_opened: c_int,
    is_dir: c_int,
}

#[no_mangle]
pub extern "C" fn fs_readdir__part535(fs: *mut c_void, f: *mut FSFile) -> c_int {
    unsafe {
        if ((*f).is_opened == 0 || (*f).is_dir == 0) {
            return -71;
        }
    }
    0
}
}

// ── `if (offset == 0) … else` · fs_disk.c:306-309 · chunk 536 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (offset == 0)
//             rewinddir(f->u.dirp);
//         else
//             seekdir(f->u.dirp, offset);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk536 {
use std::os::raw::{c_long, c_void, c_ulonglong};

extern "C" {
    fn rewinddir(dirp: *mut c_void);
    fn seekdir(dirp: *mut c_void, pos: c_long);
}

#[no_mangle]
pub extern "C" fn fs_readdir__part536(offset: c_ulonglong, dirp: *mut c_void) {
    if offset == 0 {
        unsafe { rewinddir(dirp); }
    } else {
        unsafe { seekdir(dirp, offset as c_long); }
    }
}
}

// ── `pos = 0;` · fs_disk.c:310-310 · chunk 537 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     pos = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk537 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_readdir__part537() -> c_int {
    0
}
}

// ── `if (!f->is_opened || f->is_dir)` · fs_disk.c:359-360 · chunk 541 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened || f->is_dir)
//             return -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk541 {
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;
use core::ffi::c_void;

#[repr(C)]
union UnionFS {
    fd: c_int,
    dirp: *mut c_void,
}

#[repr(C)]
struct FSFile {
    uid: c_uint,
    path: *mut c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: UnionFS,
}

#[no_mangle]
pub unsafe extern "C" fn fs_read__part541(f: *const FSFile) -> c_int {
    if (*f).is_opened == 0 || (*f).is_dir != 0 {
        return -71;
    }
    return 0;
}
}

// ── `ret = pread(f->u.fd, buf, count, offset);` · fs_disk.c:361-361 · chunk 542 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     ret = pread(f->u.fd, buf, count, offset);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk542 {
use core::ffi::c_int;
use core::ffi::c_uchar;
use core::ffi::c_ulonglong;
use std::os::raw::c_long;
use std::os::raw::c_void;

extern "C" {
    fn pread(fd: c_int, buf: *mut c_void, count: usize, offset: c_long) -> c_long;
}

#[no_mangle]
pub extern "C" fn fs_read__part542(fd: c_int, buf: *mut c_uchar, count: c_int, offset: c_ulonglong) -> c_int {
    unsafe {
        let result = pread(fd, buf as *mut c_void, count as usize, offset as c_long);
        result as c_int
    }
}
}

// ── `if (ret < 0) … else` · fs_disk.c:362-365 · chunk 543 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         else
//             return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk543 {
use core::ffi::c_int;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

fn errno_to_p9(mut err: c_int) -> c_int {
    let table: [(c_int, c_int); 9] = [
        (1, 1),
        (2, 2),
        (5, 5),
        (17, 17),
        (22, 22),
        (28, 28),
        (39, 39),
        (71, 71),
        (524, 95),
    ];
    if err == 0 {
        return 0;
    }
    let mut i: usize = 0;
    while i < 9 {
        if err == table[i].1 {
            return table[i].0;
        }
        i += 1;
    }
    22
}

#[no_mangle]
pub extern "C" fn fs_read__part543(ret: c_int) -> c_int {
    if ret < 0 {
        let errno_val = unsafe { *__errno_location() };
        -errno_to_p9(errno_val)
    } else {
        ret
    }
}
}

// ── `if (!f->is_opened || f->is_dir)` · fs_disk.c:373-374 · chunk 545 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened || f->is_dir)
//             return -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk545 {
use core::ffi::c_int;
use core::ffi::c_uint;

#[repr(C)]
union UnionFsFile {
    fd: c_int,
    dirp: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct FSFile {
    uid: c_uint,
    path: *mut core::ffi::c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: UnionFsFile,
}

#[no_mangle]
pub extern "C" fn fs_write__part545(f: *const FSFile) -> c_int {
    unsafe {
        if ((*f).is_opened == 0 || (*f).is_dir != 0) {
            return -71;
        }
    }
    return 0;
}
}

// ── `if (f->is_dir) … else` · fs_disk.c:386-389 · chunk 550 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (f->is_dir)
//             closedir(f->u.dirp);
//         else
//             close(f->u.fd);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk550 {
use std::os::raw::{c_int, c_char, c_void};

extern "C" {
    pub fn close(fds: c_int) -> c_int;
    pub fn closedir(dir: *mut c_void) -> c_int;
}

#[repr(C)]
union FileUnion {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FileUnion,
}

#[no_mangle]
pub extern "C" fn fs_close__part550(f: *const FSFile) {
    unsafe {
        if (*f).is_dir != 0 {
            closedir((*f).u.dirp);
        } else {
            close((*f).u.fd);
        }
    }
}
}

// ── `f->is_opened = FALSE;` · fs_disk.c:390-390 · chunk 551 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     f->is_opened = FALSE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk551 {
// Rust port of fs_close__part551: replaces f->is_opened = FALSE;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;
use core::ffi::c_void;

#[repr(C)]
union FSFileUnion {
    fd: c_int,
    dirp: *mut c_void,
}

#[repr(C)]
struct FSFile {
    uid: c_uint,
    path: *mut c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: FSFileUnion,
}

#[no_mangle]
pub extern "C" fn fs_close__part551(f: *mut FSFile) {
    unsafe {
        (*f).is_opened = 0;
    }
}
}

// ── `if (mask & (0x00000002 | 0x00000004)` · fs_disk.c:433-438 · chunk 556 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (mask & (0x00000002 | 0x00000004)) {
//             if (lchown(f->path, (mask & 0x00000002) ? uid : -1,
//                        (mask & 0x00000004) ? gid : -1) < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             ctime_updated = TRUE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk556 {
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;

extern "C" {
    fn lchown(__file: *const c_char, __owner: c_uint, __group: c_uint) -> c_int;
    fn __errno_location() -> *mut c_int;
}

static ERRNO_TABLE: &[(c_int, c_int)] = &[
    (1, 1),
    (2, 2),
    (5, 5),
    (17, 17),
    (22, 22),
    (28, 28),
    (39, 39),
    (71, 71),
    (524, 95),
];

fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    for &(errno_val, p9_val) in ERRNO_TABLE {
        if err == errno_val {
            return p9_val;
        }
    }
    22i32 as c_int
}

#[no_mangle]
pub extern "C" fn fs_setattr__part556(
    mask: c_uint,
    path: *const c_char,
    uid: c_uint,
    gid: c_uint,
    ctime_updated: *mut c_int,
) -> c_int {
    if (mask & 0x00000002) == 0 && (mask & 0x00000004) == 0 {
        return 0;
    }

    let owner = if (mask & 0x00000002) != 0 {
        uid
    } else {
        -(1i32) as c_uint
    };
    let group = if (mask & 0x00000004) != 0 {
        gid
    } else {
        -(1i32) as c_uint
    };

    let ret = unsafe { lchown(path, owner, group) };
    if ret < 0 {
        let errno = unsafe { *__errno_location() };
        return -errno_to_p9(errno);
    }

    unsafe { *ctime_updated = 1; };
    0
}
}

// ── `if (mask & 0x00000040) && !ctime_updated)` · fs_disk.c:480-483 · chunk 560 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if ((mask & 0x00000040) && !ctime_updated) {
//             if (lchown(f->path, -1, -1) < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk560 {
use core::ffi::{c_int, c_uint, c_char, c_void};

#[repr(C)]
struct FSFile {
    uid: c_uint,
    path: *const c_char,
    is_opened: c_int,
    is_dir: c_int,
    u: UnionFsFile,
}

#[repr(C)]
union UnionFsFile {
    fd: c_int,
    dirp: *mut c_void,
}

static ERRNO_TABLE: &[(c_int, c_int)] = &[
    (1, 1),
    (2, 2),
    (5, 5),
    (17, 17),
    (22, 22),
    (28, 28),
    (39, 39),
    (71, 71),
    (524, 95),
];

fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    for &(errno_val, p9_val) in ERRNO_TABLE {
        if err == errno_val {
            return p9_val;
        }
    }
    22
}

extern "C" {
    fn lchown(path: *const c_char, uid: c_int, gid: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

#[no_mangle]
pub extern "C" fn fs_setattr__part560(
    mask: c_uint,
    f: *const FSFile,
    ctime_updated: *mut c_int,
) -> c_int {
    unsafe {
        if (mask & 0x00000040) != 0 && *ctime_updated == 0 {
            let path = (*f).path;
            let result = lchown(path, -1, -1);
            if result < 0 {
                let errno_val = *__errno_location();
                return -errno_to_p9(errno_val);
            }
        }
    }
    0
}
}

// ── `return 0;` · fs_disk.c:484-484 · chunk 561 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk561 {
use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn fs_setattr__part561(
    fs: *mut c_void,
    f: *mut c_void,
    mask: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u64,
    atime_sec: u64,
    atime_nsec: u64,
    mtime_sec: u64,
    mtime_nsec: u64,
) -> c_int {
    0
}
}

// ── `free(path); return 0;` · fs_disk.c:496-497 · chunk 565 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(path);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk565 {
extern "C" {
    fn free(ptr: *mut core::ffi::c_char);
}

#[no_mangle]
pub extern "C" fn fs_link__part565(path: *mut core::ffi::c_char) -> core::ffi::c_int {
    unsafe {
        free(path);
        0
    }
}
}

// ── `if (ret < 0)` · fs_disk.c:545-546 · chunk 578 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk578 {
use core::ffi::c_int;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    match err {
        1 => 1,
        2 => 2,
        5 => 5,
        17 => 17,
        22 => 22,
        28 => 28,
        39 => 39,
        71 => 71,
        524 => 95,
        _ => 22,
    }
}

#[no_mangle]
pub extern "C" fn fs_readlink__part578(ret: c_int) -> c_int {
    if ret < 0 {
        let errno_val = unsafe { *__errno_location() };
        -(errno_to_p9(errno_val) as c_int)
    } else {
        0
    }
}
}

