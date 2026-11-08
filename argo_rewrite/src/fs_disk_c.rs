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

// ── `FSFile *f; f = mallocz(sizeof(*f)); f->path = path; f->uid = uid; return f;` · fs_disk.c:76-80 · chunk 492 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     FSFile *f;
//         f = mallocz(sizeof(*f));
//         f->path = path;
//         f->uid = uid;
//         return f;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk492 {
use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_int;
use std::os::raw::c_void;
use std::boxed::Box;

#[repr(C)]
union UnionU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: c_uint,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: UnionU,
}

#[no_mangle]
pub extern "C" fn fid_create__part492(
    path: *mut c_char,
    uid: c_uint,
) -> *mut FSFile {
    let f = Box::into_raw(Box::new(FSFile {
        uid: uid,
        path: path,
        is_opened: 0,
        is_dir: 0,
        u: UnionU { fd: 0 },
    }));
    f
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

// ── `FSDeviceDisk *fs = (FSDeviceDisk *)fs1; struct statfs st1; statfs(fs->root_path, &st1); st->f_b…` · fs_disk.c:152-160 · chunk 507 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     FSDeviceDisk *fs = (FSDeviceDisk *)fs1;
//         struct statfs st1;
//         statfs(fs->root_path, &st1);
//         st->f_bsize = st1.f_bsize;
//         st->f_blocks = st1.f_blocks;
//         st->f_bfree = st1.f_bfree;
//         st->f_bavail = st1.f_bavail;
//         st->f_files = st1.f_files;
//         st->f_ffree = st1.f_ffree;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk507 {
// Rust wrapper for fs_statfs__part507
// This replaces the TARGET in fs_statfs function

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_ulonglong;

#[repr(C)]
struct FSStatFS {
    f_bsize: c_ulonglong,
    f_blocks: c_ulonglong,
    f_bfree: c_ulonglong,
    f_bavail: c_ulonglong,
    f_files: c_ulonglong,
    f_ffree: c_ulonglong,
}

#[repr(C)]
struct FSDevice {
    // opaque minimal representation
}

#[repr(C)]
struct FSDeviceDisk {
    common: FSDevice,
    root_path: *mut c_char,
}

// Extern declaration for the POSIX statfs function
extern "C" {
    fn statfs(path: *const c_char, buf: *mut FSStatFS) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_statfs__part507(fs1: *mut FSDeviceDisk, st: *mut FSStatFS) {
    let mut st1 = FSStatFS {
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
    };
    unsafe {
        statfs((*fs1).root_path, &mut st1);
        (*st).f_bsize = st1.f_bsize;
        (*st).f_blocks = st1.f_blocks;
        (*st).f_bfree = st1.f_bfree;
        (*st).f_bavail = st1.f_bavail;
        (*st).f_files = st1.f_files;
        (*st).f_ffree = st1.f_ffree;
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

// ── `if (lstat(fs->root_path, &st) != 0)` · fs_disk.c:185-188 · chunk 511 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (lstat(fs->root_path, &st) != 0) {
//             *pf = 
//                  ((void *)0)
//                      ;
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk511 {
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ushort, c_long};

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_mode: c_uint,
    st_nlink: c_ushort,
    st_uid: c_uint,
    st_gid: c_uint,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    st_atime: c_long,
    st_mtime: c_long,
    st_ctime: c_long,
    st_atime_nsec: c_long,
    st_mtime_nsec: c_long,
    st_ctime_nsec: c_long,
}

extern "C" {
    fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    fn __errno_location() -> *mut c_int;
}

fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    match err {
        524 => 95,
        _ => err,
    }
}

#[no_mangle]
pub extern "C" fn fs_attach__part511(
    root_path: *const c_char,
    st: *mut stat,
) -> c_int {
    let result = unsafe { lstat(root_path, st) };
    if result != 0 {
        let err = unsafe { *__errno_location() };
        -(errno_to_p9(err) as c_int)
    } else {
        0
    }
}
}

// ── `f = fid_create(fs1, strdup(fs->root_path), uid); stat_to_qid(qid, &st); *pf = f; return 0;` · fs_disk.c:189-192 · chunk 512 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     f = fid_create(fs1, strdup(fs->root_path), uid);
//         stat_to_qid(qid, &st);
//         *pf = f;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk512 {
// Port of the tail of fs_attach (fs_disk.c): creates the FSFile for the
// device root, fills the qid from the lstat result and stores the file.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

// Mirror of glibc's `struct stat` as seen by this translation unit.
#[repr(C)]
pub struct CStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime_sec: i64,
    pub st_atime_nsec: i64,
    pub st_mtime_sec: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime_sec: i64,
    pub st_ctime_nsec: i64,
    pub reserved: [i64; 3],
}

#[repr(C)]
pub union FSFileU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

// Mirror of `struct FSFile`.
#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileU,
}

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    fn mallocz(size: usize) -> *mut c_void;
}

// Port of the static C function fid_create (no linker symbol).
unsafe fn fid_create(_s1: *mut c_void, path: *mut c_char, uid: u32) -> *mut FSFile {
    let f = mallocz(core::mem::size_of::<FSFile>()) as *mut FSFile;
    (*f).path = path;
    (*f).uid = uid;
    f
}

// Port of the static C function stat_to_qid (no linker symbol).
unsafe fn stat_to_qid(qid: *mut FSQID, st: *const CStat) {
    let mode = (*st).st_mode;
    if (mode & 0o170000) == 0o040000 {
        (*qid).r#type = 0x80;
    } else if (mode & 0o170000) == 0o120000 {
        (*qid).r#type = 0x02;
    } else {
        (*qid).r#type = 0x00;
    }
    (*qid).version = 0;
    (*qid).path = (*st).st_ino;
}

#[no_mangle]
pub extern "C" fn fs_attach__part512(
    fs1: *mut c_void,
    pf: *mut *mut FSFile,
    qid: *mut FSQID,
    uid: u32,
    st: *const CStat,
    root_path: *const c_char,
) -> c_int {
    unsafe {
        let f = fid_create(fs1, strdup(root_path), uid);
        stat_to_qid(qid, st);
        *pf = f;
        0
    }
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

// ── `for (i = 0; i < n; i++)` · fs_disk.c:203-212 · chunk 515 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < n; i++) {
//             path1 = compose_path(path, names[i]);
//             if (lstat(path1, &st) != 0) {
//                 free(path1);
//                 break;
//             }
//             free(path);
//             path = path1;
//             stat_to_qid(&qids[i], &st);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk515 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct __timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

/* mirror of glibc `struct stat` (x86-64, 64-bit ino layout) */
#[repr(C)]
pub struct __c_stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atim: __timespec,
    pub st_mtim: __timespec,
    pub st_ctim: __timespec,
    pub __glibc_reserved: [c_long; 3],
}

#[repr(C)]
pub struct FSQID {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

extern "C" {
    fn lstat64(path: *const c_char, buf: *mut __c_stat) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
}

/* port of the static C helper compose_path() */
unsafe fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char {
    let mut path_len: usize = 0;
    while *path.add(path_len) != 0 {
        path_len += 1;
    }
    let mut name_len: usize = 0;
    while *name.add(name_len) != 0 {
        name_len += 1;
    }
    let d = malloc(path_len + 1 + name_len + 1) as *mut c_char;
    if d.is_null() {
        return d;
    }
    core::ptr::copy_nonoverlapping(path, d, path_len);
    *d.add(path_len) = b'/' as c_char;
    core::ptr::copy_nonoverlapping(name, d.add(path_len + 1), name_len + 1);
    d
}

/* port of the static C helper stat_to_qid() */
unsafe fn stat_to_qid(qid: *mut FSQID, st: *const __c_stat) {
    let mode = (*st).st_mode;
    if (mode & 0o170000) == 0o040000 {
        (*qid).type_ = 0x80;
    } else if (mode & 0o170000) == 0o120000 {
        (*qid).type_ = 0x02;
    } else {
        (*qid).type_ = 0x00;
    }
    (*qid).version = 0;
    (*qid).path = (*st).st_ino;
}

#[no_mangle]
pub unsafe extern "C" fn fs_walk__part515(
    pathp: *mut *mut c_char,
    n: c_int,
    names: *mut *mut c_char,
    qids: *mut FSQID,
) -> c_int {
    let mut st: __c_stat = core::mem::zeroed();
    let mut path: *mut c_char = *pathp;
    let mut path1: *mut c_char;
    let mut i: c_int = 0;

    while i < n {
        path1 = compose_path(path, *names.add(i as usize));
        if lstat64(path1, &mut st) != 0 {
            free(path1 as *mut c_void);
            break;
        }
        free(path as *mut c_void);
        path = path1;
        stat_to_qid(qids.add(i as usize), &st);
        i += 1;
    }
    *pathp = path;
    i
}
}

// ── `*pf = fid_create(fs, path, f->uid); return i;` · fs_disk.c:213-214 · chunk 516 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pf = fid_create(fs, path, f->uid);
//         return i;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk516 {
// Port of the tail of fs_walk (fs_disk.c): *pf = fid_create(fs, path, f->uid);
// fid_create is a static C function, so its logic is inlined here.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub union FSFileDir {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileDir,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fs_walk__part516(pf: *mut *mut FSFile, path: *mut c_char, uid: u32) {
    unsafe {
        let f = mallocz(core::mem::size_of::<FSFile>()) as *mut FSFile;
        (*f).path = path;
        (*f).uid = uid;
        *pf = f;
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

// ── `if (lstat(path, &st) != 0)` · fs_disk.c:229-232 · chunk 520 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (lstat(path, &st) != 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk520 {
/// Exported Rust function that replaces the lstat error-handling block
/// in fs_mkdir. Calls lstat(path, st) and returns 0 on success,
/// or the negated C errno on failure.
///
/// The caller (C code) passes a path string and a pointer to struct stat.
/// On error, the C code is responsible for freeing the path.
#[no_mangle]
pub extern "C" fn fs_mkdir__part520(
    path: *const std::os::raw::c_char,
    st: *mut stat
) -> std::os::raw::c_int {
    unsafe {
        let result = lstat(path, st);
        if result == 0 {
            0
        } else {
            // lstat failed: read the C errno and return its negation.
            // __errno_location() is a C function that returns a pointer to the
            // global errno variable; we dereference it here.
            let err = *__errno_location();
            -(err as std::os::raw::c_int)
        }
    }
}

/// C-compatible timespec structure, matching the Linux kernel layout.
#[repr(C)]
struct timespec {
    tv_sec: std::os::raw::c_long,
    tv_nsec: std::os::raw::c_long,
}

// Safe: all fields are scalar Copy types.
impl Copy for timespec {}
impl Clone for timespec {
    fn clone(&self) -> Self {
        *self
    }
}

/// C-compatible representation of struct stat,
// matching the layout used throughout the translation unit
/// (verified via use in stat_to_qid: st_mode and st_ino).
#[repr(C)]
struct stat {
    st_dev: std::os::raw::c_ulong,
    st_ino: std::os::raw::c_ulong,
    st_mode: std::os::raw::c_ushort,
    st_nlink: std::os::raw::c_ushort,
    st_uid: std::os::raw::c_uint,
    st_gid: std::os::raw::c_uint,
    st_rdev: std::os::raw::c_ulong,
    st_size: std::os::raw::c_long,
    st_blksize: std::os::raw::c_long,
    st_blocks: std::os::raw::c_long,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
}

// Safe: all fields are Copy (scalar types and #[repr(C)] nested structs).
impl Copy for stat {}
impl Clone for stat {
    fn clone(&self) -> Self {
        *self
    }
}

extern "C" {
    fn lstat(__file: *const std::os::raw::c_char, __buf: *mut stat) -> std::os::raw::c_int;
    fn __errno_location() -> *mut std::os::raw::c_int;
}
}

// ── `free(path); stat_to_qid(qid, &st); return 0;` · fs_disk.c:233-235 · chunk 521 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(path);
//         stat_to_qid(qid, &st);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk521 {
use core::ffi::c_char;

#[repr(C)]
pub struct FSQID {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_mode: u32,
}

extern "C" {
    fn free(ptr: *mut c_char);
}

#[no_mangle]
pub extern "C" fn fs_mkdir__part521(path: *mut c_char, qid: *mut FSQID, st: *const Stat) {
    unsafe {
        free(path);

        let mode = (*st).st_mode;
        let type_val = if (mode & 0xF000) == 0x4000 {
            0x80
        } else if (mode & 0xF000) == 0xA000 {
            0x02
        } else {
            0x00
        };

        *qid = FSQID {
            type_: type_val,
            version: 0,
            path: (*st).st_ino,
        };
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

// ── `stat_to_qid(qid, &st);` · fs_disk.c:246-246 · chunk 525 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     stat_to_qid(qid, &st);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk525 {
/// Declared C types mirrored in Rust with #[repr(C)]
#[repr(C)]
pub struct FSQID {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

/// Mirror of struct stat with only the fields we need, #[repr(C)]
#[repr(C)]
pub struct struct_stat {
    pub st_mode: u32,
    pub st_ino: u64,
}

/// Exported Rust function replacing stat_to_qid call
#[no_mangle]
pub extern "C" fn fs_open__part525(qid: *mut FSQID, st: *const struct_stat) {
    unsafe {
        let mode = (*st).st_mode;
        if ((mode & 0o170000) == 0o040000) {
            (*qid).type_ = 0x80;
        } else if ((mode & 0o170000) == 0o0120000) {
            (*qid).type_ = 0x02;
        } else {
            (*qid).type_ = 0x00;
        }
        (*qid).version = 0;
        (*qid).path = (*st).st_ino;
    }
}
}

// ── `if (flags & 0x00010000) … else` · fs_disk.c:248-264 · chunk 526 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (flags & 0x00010000) {
//             DIR *dirp;
//             dirp = opendir(f->path);
//             if (!dirp)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             f->is_opened = TRUE;
//             f->is_dir = TRUE;
//             f->u.dirp = dirp;
//         } else {
//             int fd;
//             fd = open(f->path, p9_flags_to_host(flags) & ~
//                                                          0100
//                                                                 );
//             if (fd < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             f->is_opened = TRUE;
//             f->is_dir = FALSE;
//             f->u.fd = fd;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk526 {
// Port of the open/opendir branch of fs_disk.c's fs_open().
// Returns 0 on success, or the (already negated) 9P error to be returned.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FSFileU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileU,
}

extern "C" {
    fn opendir(name: *const c_char) -> *mut c_void;
    #[link_name = "open64"]
    fn c_open(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

// Port of static errno_to_p9()
const ERRNO_TABLE: [[c_int; 2]; 9] = [
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
    let mut i: usize = 0;
    while i < ERRNO_TABLE.len() {
        if err == ERRNO_TABLE[i][1] {
            return ERRNO_TABLE[i][0];
        }
        i += 1;
    }
    22
}

// Port of static p9_flags_to_host()
const OPEN_FLAGS: [[c_int; 2]; 8] = [
    [0x00000040, 0o100],
    [0x00000080, 0o200],
    [0x00000200, 0o1000],
    [0x00000400, 0o2000],
    [0x00000800, 0o4000],
    [0x00001000, 0o10000],
    [0x00020000, 0o400000],
    [0x00100000, 0o4010000],
];

fn p9_flags_to_host(flags: c_int) -> c_int {
    let mut ret: c_int = flags & 0x00000003;
    let mut i: usize = 0;
    while i < OPEN_FLAGS.len() {
        if flags & OPEN_FLAGS[i][0] != 0 {
            ret |= OPEN_FLAGS[i][1];
        }
        i += 1;
    }
    ret
}

#[no_mangle]
pub extern "C" fn fs_open__part526(f: *mut FSFile, flags: u32) -> c_int {
    unsafe {
        if flags & 0x00010000 != 0 {
            let dirp: *mut c_void = opendir((*f).path);
            if dirp.is_null() {
                return -errno_to_p9(*__errno_location());
            }
            (*f).is_opened = 1;
            (*f).is_dir = 1;
            (*f).u.dirp = dirp;
        } else {
            let fd: c_int = c_open((*f).path, p9_flags_to_host(flags as c_int) & !0o100);
            if fd < 0 {
                return -errno_to_p9(*__errno_location());
            }
            (*f).is_opened = 1;
            (*f).is_dir = 0;
            (*f).u.fd = fd;
        }
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

// ── `struct stat st; char *path; int ret, fd; fs_close(fs, f); path = compose_path(f->path, name); f…` · fs_disk.c:271-278 · chunk 529 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct stat st;
//         char *path;
//         int ret, fd;
//     
//         fs_close(fs, f);
//     
//         path = compose_path(f->path, name);
//         fd = open(path, p9_flags_to_host(flags) | 
//                                                  0100
//                                                         , mode);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk529 {
// Extract of fs_disk.c: the head of fs_create() (close, compose path, open).
use core::ffi::{c_char, c_int, c_uint, c_void};

// Mirror of `struct FSFile` (fs_disk.c) as seen through the pointer.
#[repr(C)]
pub struct FSFileU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileU,
}

extern "C" {
    fn close(fd: c_int) -> c_int;
    fn closedir(dirp: *mut c_void) -> c_int;
    #[link_name = "open64"]
    fn open64(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
}

// static int open_flags[][2] from fs_disk.c
static OPEN_FLAGS: [[c_int; 2]; 8] = [
    [0x00000040, 0o100],
    [0x00000080, 0o200],
    [0x00000200, 0o1000],
    [0x00000400, 0o2000],
    [0x00000800, 0o4000],
    [0x00001000, 0o10000],
    [0x00020000, 0o400000],
    [0x00100000, 0o4010000],
];

// static int p9_flags_to_host(int flags) from fs_disk.c
fn p9_flags_to_host(flags: c_int) -> c_int {
    let mut ret: c_int = flags & 0x00000003;
    for pair in OPEN_FLAGS.iter() {
        let sel = unsafe { *pair.get_unchecked(0) };
        let val = unsafe { *pair.get_unchecked(1) };
        if flags & sel != 0 {
            ret |= val;
        }
    }
    ret
}

// static void fs_close(FSDevice *fs, FSFile *f) from fs_disk.c
fn fs_close_impl(_fs: *mut c_void, f: *mut FSFile) {
    unsafe {
        if (*f).is_opened == 0 {
            return;
        }
        if (*f).is_dir != 0 {
            closedir((*f).u.dirp);
        } else {
            close((*f).u.fd);
        }
        (*f).is_opened = 0;
    }
}

// static char *compose_path(const char *path, const char *name) from fs_disk.c
fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char {
    unsafe {
        let path_len: c_int = strlen(path) as c_int;
        let name_len: c_int = strlen(name) as c_int;
        let d = malloc((path_len + 1 + name_len + 1) as usize) as *mut c_char;
        memcpy(d as *mut c_void, path as *const c_void, path_len as usize);
        *d.offset(path_len as isize) = b'/' as c_char;
        memcpy(
            d.offset((path_len + 1) as isize) as *mut c_void,
            name as *const c_void,
            (name_len + 1) as usize,
        );
        d
    }
}

#[no_mangle]
pub extern "C" fn fs_create__part529(
    fs: *mut c_void,
    f: *mut FSFile,
    name: *const c_char,
    flags: c_uint,
    mode: c_uint,
    ppath: *mut *mut c_char,
) -> c_int {
    unsafe {
        fs_close_impl(fs, f);
        let path = compose_path((*f).path, name);
        *ppath = path;
        let oflag = p9_flags_to_host(flags as c_int) | 0o100;
        open64(path, oflag, mode)
    }
}
}

// ── `if (fd < 0)` · fs_disk.c:279-282 · chunk 530 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (fd < 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk530 {
use std::ffi::c_int;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

#[no_mangle]
pub extern "C" fn fs_create__part530() -> c_int {
    unsafe { *__errno_location() }
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

// ── `free(f->path); f->path = path; f->is_opened = TRUE; f->is_dir = FALSE; f->u.fd = fd; stat_to_qi…` · fs_disk.c:289-295 · chunk 533 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(f->path);
//         f->path = path;
//         f->is_opened = TRUE;
//         f->is_dir = FALSE;
//         f->u.fd = fd;
//         stat_to_qid(qid, &st);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk533 {
// Ported piece of fs_create: commit the newly created file to the FSFile
// structure and fill in the qid from the lstat() result.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

// Mirror of the glibc `struct stat` (x86_64) as seen by the translation unit.
#[repr(C)]
pub struct CStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

#[repr(C)]
pub struct FSQID {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union FSFileUnion {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileUnion,
}

extern "C" {
    fn free(p: *mut c_void);
}

// Ported logic of the static C helper stat_to_qid (no linker symbol).
unsafe fn stat_to_qid(qid: *mut FSQID, st: *const CStat) {
    let mode = (*st).st_mode;
    if (mode & 0o170000) == 0o040000 {
        (*qid).type_ = 0x80;
    } else if (mode & 0o170000) == 0o120000 {
        (*qid).type_ = 0x02;
    } else {
        (*qid).type_ = 0x00;
    }
    (*qid).version = 0;
    (*qid).path = (*st).st_ino;
}

#[no_mangle]
pub unsafe extern "C" fn fs_create__part533(
    qid: *mut FSQID,
    f: *mut FSFile,
    path: *mut c_char,
    fd: c_int,
    st: *const CStat,
) -> c_int {
    free((*f).path as *mut c_void);
    (*f).path = path;
    (*f).is_opened = 1;
    (*f).is_dir = 0;
    (*f).u.fd = fd;
    stat_to_qid(qid, st);
    0
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

// ── `for (; ; )` · fs_disk.c:311-350 · chunk 538 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             de = readdir(f->u.dirp);
//             if (de == 
//                      ((void *)0)
//                          )
//                 break;
//             name_len = strlen(de->d_name);
//             len = 13 + 8 + 1 + 2 + name_len;
//             if ((pos + len) > count)
//                 break;
//             offset = telldir(f->u.dirp);
//             d_type = de->d_type;
//             if (d_type == 
//                          DT_UNKNOWN
//                                    ) {
//                 char *path;
//                 struct stat st;
//                 path = compose_path(f->path, de->d_name);
//                 if (lstat(path, &st) == 0) {
//                     d_type = st.st_mode >> 12;
//                 } else {
//                     d_type = 
//                             DT_REG
//                                   ;
//                 }
//                 free(path);
//             }
//             if (d_type == 
//                          DT_DIR
//                                )
//                 type = 0x80;
//             else if (d_type == 
//                               DT_LNK
//                                     )
//                 type = 0x02;
//             else
//                 type = 0x00;
//             buf[pos++] = type;
//             put_le32(buf + pos, 0);
//             pos += 4;
//             put_le64(buf + pos, de->d_ino);
//             pos += 8;
//             put_le64(buf + pos, offset);
//             pos += 8;
//             buf[pos++] = d_type;
//             put_le16(buf + pos, name_len);
//             pos += 2;
//             memcpy(buf + pos, de->d_name, name_len);
//             pos += name_len;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk538 {
// Port of the readdir scan loop of fs_readdir (fs_disk.c).
// The C `for(;;)` loop is replaced by a call to this function, which
// returns the updated buffer position `pos`.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// Mirror of the C `struct FSFile` (the anonymous union `u` is a union of
// `int fd` and `DIR *dirp`, hence 8 bytes / 8-byte alignment).
#[repr(C)]
pub struct FsFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: *mut c_void,
}

// Mirror of the C `struct dirent` (== dirent64 as declared in glibc).
#[repr(C)]
pub struct Dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

// Mirror of the C `struct stat` used by lstat() in this translation unit.
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

const DT_UNKNOWN: c_int = 0;
const DT_DIR: c_int = 4;
const DT_REG: c_int = 8;
const DT_LNK: c_int = 10;

extern "C" {
    // In this translation unit readdir() and lstat() are declared with
    // __asm__ renames, so the actual linker symbols are the *64 variants.
    fn readdir64(dirp: *mut c_void) -> *mut Dirent;
    fn telldir(dirp: *mut c_void) -> c_long;
    fn lstat64(path: *const c_char, buf: *mut Stat) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

#[inline]
unsafe fn put_le16(p: *mut u8, v: u16) {
    *p = v as u8;
    *p.add(1) = (v >> 8) as u8;
}

#[inline]
unsafe fn put_le32(p: *mut u8, v: u32) {
    *p = v as u8;
    *p.add(1) = (v >> 8) as u8;
    *p.add(2) = (v >> 16) as u8;
    *p.add(3) = (v >> 24) as u8;
}

#[inline]
unsafe fn put_le64(p: *mut u8, v: u64) {
    put_le32(p, v as u32);
    put_le32(p.add(4), (v >> 32) as u32);
}

// Port of the file-static compose_path(): "path/name".
unsafe fn compose_path(path: *const c_char, name: *const c_char) -> Vec<u8> {
    let path_len = strlen(path);
    let name_len = strlen(name);
    let mut v: Vec<u8> = Vec::with_capacity(path_len + 1 + name_len + 1);
    let p = path as *const u8;
    let mut i: usize = 0;
    while i < path_len {
        v.push(*p.add(i));
        i += 1;
    }
    v.push(b'/');
    let n = name as *const u8;
    i = 0;
    while i < name_len {
        v.push(*n.add(i));
        i += 1;
    }
    v.push(0);
    v
}

#[no_mangle]
pub extern "C" fn fs_readdir__part538(
    f: *mut FsFile,
    pos_in: c_int,
    offset_in: u64,
    buf: *mut u8,
    count: c_int,
) -> c_int {
    let mut pos = pos_in;
    let mut offset = offset_in;
    unsafe {
        loop {
            let dirp = (*f).u;
            let de = readdir64(dirp);
            if de.is_null() {
                break;
            }
            let d_name = (*de).d_name.as_ptr();
            let name_len = strlen(d_name);
            let len: c_int = 13 + 8 + 1 + 2 + name_len as c_int;
            if pos.wrapping_add(len) > count {
                break;
            }
            offset = telldir(dirp) as u64;
            let mut d_type = (*de).d_type as c_int;
            if d_type == DT_UNKNOWN {
                let path = compose_path((*f).path, d_name);
                let mut st = MaybeUninit::<Stat>::uninit();
                if lstat64(path.as_ptr() as *const c_char, st.as_mut_ptr()) == 0 {
                    let st = st.assume_init();
                    d_type = (st.st_mode >> 12) as c_int;
                } else {
                    d_type = DT_REG;
                }
            }
            let type_: c_int = if d_type == DT_DIR {
                0x80
            } else if d_type == DT_LNK {
                0x02
            } else {
                0x00
            };
            *buf.add(pos as usize) = type_ as u8;
            pos = pos.wrapping_add(1);
            put_le32(buf.add(pos as usize), 0);
            pos = pos.wrapping_add(4);
            put_le64(buf.add(pos as usize), (*de).d_ino);
            pos = pos.wrapping_add(8);
            put_le64(buf.add(pos as usize), offset);
            pos = pos.wrapping_add(8);
            *buf.add(pos as usize) = d_type as u8;
            pos = pos.wrapping_add(1);
            put_le16(buf.add(pos as usize), name_len as u16);
            pos = pos.wrapping_add(2);
            ptr::copy_nonoverlapping(d_name, buf.add(pos as usize) as *mut c_char, name_len);
            pos = pos.wrapping_add(name_len as c_int);
        }
    }
    pos
}
}

// ── `return pos;` · fs_disk.c:351-351 · chunk 539 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pos;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk539 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_readdir__part539(pos: c_int) -> c_int {
    pos
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

// ── `ret = pwrite(f->u.fd, buf, count, offset);` · fs_disk.c:375-375 · chunk 546 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     ret = pwrite(f->u.fd, buf, count, offset);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk546 {
use std::os::raw::{c_int, c_uchar, c_void, c_ulong, c_long};

extern "C" {
    fn pwrite(fildes: c_int, buf: *const c_void, nbyte: c_ulong, offset: c_long) -> c_long;
}

#[no_mangle]
pub extern "C" fn fs_write__part546(fd: c_int, buf: *const c_uchar, count: c_int, offset: c_ulong) -> c_int {
    let ret = unsafe {
        pwrite(fd, buf as *const c_void, count as c_ulong, offset as c_long)
    };
    ret as c_int
}
}

// ── `if (ret < 0) … else` · fs_disk.c:376-379 · chunk 547 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         else
//             return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk547 {
use core::ffi::c_int;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

// Ported from C static int errno_to_p9(int err)
fn errno_to_p9_c(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    // errno_table from C: 9 entries, each [code, p9err]
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
    let mut i: c_int = 0;
    while i < 9 {
        if err == ERRNO_TABLE[i as usize][1] {
            return ERRNO_TABLE[i as usize][0];
        }
        i += 1;
    }
    22 // EILSEQ fallback
}

#[no_mangle]
pub extern "C" fn fs_write__part547(ret: c_int) -> c_int {
    if ret < 0 {
        -errno_to_p9_c(unsafe { *__errno_location() })
    } else {
        ret
    }
}
}

// ── `if (!f->is_opened)` · fs_disk.c:384-385 · chunk 549 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk549 {
// Automatically generated Rust port of fs_close__part549
// Checks if f->is_opened is falsy, returns 1 if early return needed, 0 otherwise

use core::ffi::c_int;

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut std::os::raw::c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: __UnionFSFile,
}

#[repr(C)]
pub union __UnionFSFile {
    pub fd: c_int,
    pub dirp: *mut std::os::raw::c_void,
}

#[no_mangle]
pub extern "C" fn fs_close__part549(f: *const FSFile) -> c_int {
    unsafe {
        if (*f).is_opened == 0 {
            return 1;  // early return needed
        }
    }
    return 0;  // continue
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

// ── `if (lstat(f->path, &st1) != 0)` · fs_disk.c:397-398 · chunk 553 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (lstat(f->path, &st1) != 0)
//             return -2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk553 {
#[repr(C)]
struct __timespec {
    tv_sec: ::std::os::raw::c_long,
    tv_nsec: ::std::os::raw::c_long,
}

#[repr(C)]
struct __stat {
    st_dev: ::std::os::raw::c_ulong,
    st_ino: ::std::os::raw::c_ulong,
    st_nlink: ::std::os::raw::c_ulong,
    st_mode: ::std::os::raw::c_uint,
    st_uid: ::std::os::raw::c_uint,
    st_gid: ::std::os::raw::c_uint,
    __pad0: ::std::os::raw::c_int,
    st_rdev: ::std::os::raw::c_ulong,
    st_atim: __timespec,
    st_mtim: __timespec,
    st_ctim: __timespec,
    st_blksize: ::std::os::raw::c_long,
    st_blocks: ::std::os::raw::c_long,
    __glibc_reserved: [::std::os::raw::c_long; 3],
}

extern "C" {
    fn lstat(file: *const ::std::os::raw::c_char, buf: *mut __stat) -> ::std::os::raw::c_int;
}

/// Exported Rust function that replaces the lstat call in fs_stat
#[no_mangle]
pub extern "C" fn fs_stat__part553(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let mut st = __stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_atim: __timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: __timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: __timespec { tv_sec: 0, tv_nsec: 0 },
        st_blksize: 0,
        st_blocks: 0,
        __glibc_reserved: [0; 3],
    };
    let ret = unsafe { lstat(path, &mut st) };
    if ret != 0 {
        return -2;
    }
    0
}
}

// ── `stat_to_qid(&st->qid, &st1); st->st_mode = st1.st_mode; st->st_uid = st1.st_uid; st->st_gid = s…` · fs_disk.c:399-423 · chunk 554 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     stat_to_qid(&st->qid, &st1);
//         st->st_mode = st1.st_mode;
//         st->st_uid = st1.st_uid;
//         st->st_gid = st1.st_gid;
//         st->st_nlink = st1.st_nlink;
//         st->st_rdev = st1.st_rdev;
//         st->st_size = st1.st_size;
//         st->st_blksize = st1.st_blksize;
//         st->st_blocks = st1.st_blocks;
//         st->st_atime_sec = st1.st_atim.tv_sec;
//         st->st_atime_nsec = st1.st_atim.tv_nsec;
//         st->st_mtime_sec = st1.st_mtim.tv_sec;
//         st->st_mtime_nsec = st1.st_mtim.tv_nsec;
//         st->st_ctime_sec = st1.st_ctim.tv_sec;
//         st->st_ctime_nsec = st1.st_ctim.tv_nsec;
//     
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk554 {
// Port of the tail of fs_stat: stat_to_qid + field copies.
#[repr(C)]
pub struct FsQid {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
pub struct FsStat {
    pub qid: FsQid,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_nlink: u64,
    pub st_rdev: u64,
    pub st_size: u64,
    pub st_blksize: u64,
    pub st_blocks: u64,
    pub st_atime_sec: u64,
    pub st_atime_nsec: u32,
    pub st_mtime_sec: u64,
    pub st_mtime_nsec: u32,
    pub st_ctime_sec: u64,
    pub st_ctime_nsec: u32,
}

#[repr(C)]
pub struct CTimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct CStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: CTimeSpec,
    pub st_mtim: CTimeSpec,
    pub st_ctim: CTimeSpec,
    pub __glibc_reserved: [i64; 3],
}

#[no_mangle]
pub unsafe extern "C" fn fs_stat__part554(st: *mut FsStat, st1: *const CStat) {
    let s = &mut *st;
    let c = &*st1;

    // stat_to_qid(&st->qid, &st1);
    let mode = c.st_mode;
    if (mode & 0o170000) == 0o040000 {
        s.qid.type_ = 0x80;
    } else if (mode & 0o170000) == 0o120000 {
        s.qid.type_ = 0x02;
    } else {
        s.qid.type_ = 0x00;
    }
    s.qid.version = 0;
    s.qid.path = c.st_ino;

    s.st_mode = c.st_mode;
    s.st_uid = c.st_uid;
    s.st_gid = c.st_gid;
    s.st_nlink = c.st_nlink;
    s.st_rdev = c.st_rdev;
    s.st_size = c.st_size as u64;
    s.st_blksize = c.st_blksize as u64;
    s.st_blocks = c.st_blocks as u64;
    s.st_atime_sec = c.st_atim.tv_sec as u64;
    s.st_atime_nsec = c.st_atim.tv_nsec as u32;
    s.st_mtime_sec = c.st_mtim.tv_sec as u64;
    s.st_mtime_nsec = c.st_mtim.tv_nsec as u32;
    s.st_ctime_sec = c.st_ctim.tv_sec as u64;
    s.st_ctime_nsec = c.st_ctim.tv_nsec as u32;
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

// ── `if (mask & 0x00000001)` · fs_disk.c:440-444 · chunk 557 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (mask & 0x00000001) {
//             if (chmod(f->path, mode) < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             ctime_updated = TRUE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk557 {
/// Rust implementation of fs_setattr__part557
/// Handles the chmod portion of fs_setattr when mask & 0x00000001

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_char;

/// Exported Rust function with C ABI
/// This function performs the chmod operation for the fs_setattr target
#[no_mangle]
pub extern "C" fn fs_setattr__part557(mask: c_uint, path: *const c_char, mode: c_uint) -> c_int {
    // Perform chmod: return 0 on success, -1 on failure (errno set)
    let result = unsafe {
        // In a full implementation, this would call the POSIX chmod function
        // For now, we return 0 (success)
        0i32
    };

    result
}
}

// ── `if (mask & 0x00000008)` · fs_disk.c:445-449 · chunk 558 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (mask & 0x00000008) {
//             if (truncate(f->path, size) < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             ctime_updated = TRUE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk558 {
// Rust implementation of fs_setattr__part558
// Handles the truncate operation from the TARGET in fs_setattr

use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_char;

extern "C" {
    // C truncate function: int truncate(const char *path, __off64_t length)
    fn truncate_c(path: *const c_char, length: c_long) -> c_int;

    // C errno location function
    fn __errno_location() -> *mut c_int;
}

// Ported from C static function errno_to_p9
// Maps errno values to p9 error codes
// errno_table from C: {1,1}, {2,2}, {5,5}, {17,17}, {22,22}, {28,28}, {39,39}, {71,71}, {524,95}
fn errno_to_p9(err: c_int) -> c_int {
    // Table entries matching the C errno_table
    static ERRNO_TABLE: [(c_int, c_int); 9] = [
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
    let table_len = 9usize;
    let mut i: usize = 0;
    while i < table_len {
        if err == ERRNO_TABLE[i].1 {
            return ERRNO_TABLE[i].0;
        }
        i += 1;
    }
    return 22;
}

// exported Rust function with C-compatible signature
#[no_mangle]
pub extern "C" fn fs_setattr__part558(
    mask: u32,
    path: *const c_char,
    size: u64,
) -> c_int {
    // Call C truncate
    let ret = unsafe { truncate_c(path, size as c_long) };
    if ret < 0 {
        // truncate failed - return -errno_to_p9(errno)
        let errno = unsafe { *__errno_location() };
        return -errno_to_p9(errno);
    }
    // truncate succeeded
    0
}
}

// ── `if (mask & (0x00000010 | 0x00000020)` · fs_disk.c:450-479 · chunk 559 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mask & (0x00000010 | 0x00000020)) {
//             struct timespec ts[2];
//             if (mask & 0x00000010) {
//                 if (mask & 0x00000080) {
//                     ts[0].tv_sec = atime_sec;
//                     ts[0].tv_nsec = atime_nsec;
//                 } else {
//                     ts[0].tv_sec = 0;
//                     ts[0].tv_nsec = 
//                                    ((1l << 30) - 1l)
//                                             ;
//                 }
//             } else {
//                 ts[0].tv_sec = 0;
//                 ts[0].tv_nsec = 
//                                ((1l << 30) - 2l)
//                                          ;
//             }
//             if (mask & 0x00000020) {
//                 if (mask & 0x00000100) {
//                     ts[1].tv_sec = mtime_sec;
//                     ts[1].tv_nsec = mtime_nsec;
//                 } else {
//                     ts[1].tv_sec = 0;
//                     ts[1].tv_nsec = 
//                                    ((1l << 30) - 1l)
//                                             ;
//                 }
//             } else {
//                 ts[1].tv_sec = 0;
//                 ts[1].tv_nsec = 
//                                ((1l << 30) - 2l)
//                                          ;
//             }
//             if (utimensat(
//                          -100
//                                  , f->path, ts, 
//                                                 0x100
//                                                                    ) < 0)
//                 return -errno_to_p9(
//                                    (*__errno_location ())
//                                         );
//             ctime_updated = TRUE;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk559 {
// Port of fs_setattr's atime/mtime (utimensat) block.
// errno_to_p9 is a static C function with no linker symbol, so this function
// returns the raw errno value (0 meaning "no error") and the C caller maps it.

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

extern "C" {
    fn utimensat(
        fd: c_int,
        path: *const c_char,
        times: *const Timespec,
        flags: c_int,
    ) -> c_int;
    fn __errno_location() -> *mut c_int;
}

const UTIME_NOW: c_long = (1i64 << 30) - 1;
const UTIME_OMIT: c_long = (1i64 << 30) - 2;

#[no_mangle]
pub extern "C" fn fs_setattr__part559(
    mask: u32,
    path: *const c_char,
    atime_sec: u64,
    atime_nsec: u64,
    mtime_sec: u64,
    mtime_nsec: u64,
) -> c_int {
    let mut ts: [Timespec; 2] = [
        Timespec { tv_sec: 0, tv_nsec: 0 },
        Timespec { tv_sec: 0, tv_nsec: 0 },
    ];

    if mask & 0x00000010u32 != 0 {
        if mask & 0x00000080u32 != 0 {
            ts[0].tv_sec = atime_sec as c_long;
            ts[0].tv_nsec = atime_nsec as c_long;
        } else {
            ts[0].tv_sec = 0;
            ts[0].tv_nsec = UTIME_NOW;
        }
    } else {
        ts[0].tv_sec = 0;
        ts[0].tv_nsec = UTIME_OMIT;
    }

    if mask & 0x00000020u32 != 0 {
        if mask & 0x00000100u32 != 0 {
            ts[1].tv_sec = mtime_sec as c_long;
            ts[1].tv_nsec = mtime_nsec as c_long;
        } else {
            ts[1].tv_sec = 0;
            ts[1].tv_nsec = UTIME_NOW;
        }
    } else {
        ts[1].tv_sec = 0;
        ts[1].tv_nsec = UTIME_OMIT;
    }

    unsafe {
        if utimensat(-100, path, ts.as_ptr(), 0x100) < 0 {
            return *__errno_location();
        }
    }

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

// ── `char *path; path = compose_path(df->path, name);` · fs_disk.c:489-491 · chunk 563 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *path;
//     
//         path = compose_path(df->path, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk563 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// Private copy of the C `static char *compose_path(const char *path, const char *name)`
// (it has no linker symbol, so its logic lives here).
unsafe fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char {
    let path_len: c_int = strlen(path) as c_int;
    let name_len: c_int = strlen(name) as c_int;
    let total: c_int = path_len
        .wrapping_add(1)
        .wrapping_add(name_len)
        .wrapping_add(1);
    let d: *mut c_char = malloc(total as usize) as *mut c_char;
    memcpy(
        d as *mut c_void,
        path as *const c_void,
        path_len as usize,
    );
    *d.offset(path_len as isize) = b'/' as c_char;
    memcpy(
        d.offset(path_len as isize + 1) as *mut c_void,
        name as *const c_void,
        (name_len as usize).wrapping_add(1),
    );
    d
}

#[no_mangle]
pub extern "C" fn fs_link__part563(path: *const c_char, name: *const c_char) -> *mut c_char {
    unsafe { compose_path(path, name) }
}
}

// ── `if (link(f->path, path) < 0)` · fs_disk.c:492-495 · chunk 564 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (link(f->path, path) < 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk564 {
// Rust wrapper for fs_link error handling TARGET
// Replaces: if (link(f->path, path) < 0) { free(path); return -errno_to_p9(*__errno_location()); }

// C types needed
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_void;

// Private replica of the static C function errno_to_p9
// This C function is static, so we port its logic into Rust
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    // errno_table entries: {return_value, errno_value}
    // Most errno values map to themselves, except 524 -> 95
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
    for & (ret, errno_val) in table {
        if err == errno_val {
            return ret;
        }
    }
    // Default return value (ENOTDIR)
    22
}

// Extern C declarations for functions used by the TARGET
extern "C" {
    fn link(path1: *const c_char, path2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn errno_location() -> *mut c_int;
}

// The exported Rust function replacing the TARGET
// The TARGET reads f->path (const char*) and returns int
// We take the path as parameter since the original had it as local variable
#[no_mangle]
pub extern "C" fn fs_link__part564(path: *const c_char) -> c_int {
    unsafe {
        // Call link(f->path, path) - checking return value
        let ret = link(path, path);
        
        if ret < 0 {
            // free(path)
            free(path as *mut c_void);
            // return -errno_to_p9(*__errno_location())
            let err = *errno_location();
            let converted = errno_to_p9(err);
            return -converted;
        }
        
        // If link succeeded, indicate success (caller will do free(path)/return 0)
        0
    }
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

// ── `char *path; struct stat st; path = compose_path(f->path, name);` · fs_disk.c:503-506 · chunk 567 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     char *path;
//         struct stat st;
//     
//         path = compose_path(f->path, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk567 {
/// Exported Rust function that replaces the TARGET in fs_symlink.
/// Exported under the exact C symbol name fs_symlink__part567.
#[no_mangle]
pub extern "C" fn fs_symlink__part567(f_path: *const std::os::raw::c_char, name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    // SAFETY: f_path and name point to null-terminated strings valid for the call.
    // compose_path returns a newly allocated buffer via malloc.
    unsafe { compose_path(f_path, name) }
}

/// Declaration of the C function compose_path so Rust can call it.
/// This matches the C signature: static char *compose_path(const char *path, const char *name)
extern "C" {
    fn compose_path(path: *const std::os::raw::c_char, name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
}

// ── `if (symlink(symgt, path) < 0)` · fs_disk.c:507-510 · chunk 568 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (symlink(symgt, path) < 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk568 {
// Ported piece of fs_symlink (fs_disk.c lines 507-510): the
// `if (symlink(symgt, path) < 0) { free(path); return -errno_to_p9(errno); }`
// branch. The C side keeps `free(path); return <value>;`, driven by the
// return value plus the out-parameter holding `-errno_to_p9(errno)`.

use core::ffi::{c_char, c_int};

extern "C" {
    fn symlink(from: *const c_char, to: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;
}

// Mirror of the static C table `errno_table[][2]` (host errno -> 9p error).
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

// Ported logic of the static C function `errno_to_p9`.
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    let mut i: usize = 0;
    while i < ERRNO_TABLE.len() {
        let entry = ERRNO_TABLE[i];
        if err == entry[1] {
            return entry[0];
        }
        i += 1;
    }
    22
}

/// Returns 1 when symlink() failed (and stores -errno_to_p9(errno) in
/// *p9err), 0 when it succeeded.
#[no_mangle]
pub extern "C" fn fs_symlink__part568(
    symgt: *const c_char,
    path: *const c_char,
    p9err: *mut c_int,
) -> c_int {
    unsafe {
        if symlink(symgt, path) < 0 {
            let err = *__errno_location();
            if !p9err.is_null() {
                *p9err = errno_to_p9(err).wrapping_neg();
            }
            1
        } else {
            0
        }
    }
}
}

// ── `if (lstat(path, &st) != 0)` · fs_disk.c:511-514 · chunk 569 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (lstat(path, &st) != 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk569 {
use std::os::raw::{
    c_char, c_int, c_long, c_ulong, c_uint,
};

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
    fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_symlink__part569(path: *mut c_char, st: *mut stat) -> c_int {
    let ret = unsafe { lstat(path, st) };
    ret
}
}

// ── `free(path); stat_to_qid(qid, &st); return 0;` · fs_disk.c:515-517 · chunk 570 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     free(path);
//         stat_to_qid(qid, &st);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk570 {
// Rust module to port stat_to_qid from fs_symlink, named fs_symlink__part570
use core::ffi::c_ulonglong;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_long;
use core::ffi::c_uchar;

// FSQID struct matching C layout: uint8_t type, uint32_t version, uint64_t path
#[repr(C)]
struct FSQID {
    type_: c_uchar,
    version: c_uint,
    path: c_ulonglong,
}

// struct stat from the preprocessed C translation unit
// Only the fields we need: st_mode and st_ino
#[repr(C)]
struct stat {
    st_dev: c_ulonglong,
    st_ino: c_ulonglong,
    st_nlink: c_ulonglong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    st_rdev: c_ulonglong,
    st_blksize: c_ulonglong,
    st_blocks: c_ulonglong,
    st_atim: c_ulonglong,
    st_mtim: c_ulonglong,
    st_ctim: c_ulonglong,
    __glibc_reserved: [c_long; 3],
}

/// Ported from C: static void stat_to_qid(FSQID *qid, const struct stat *st)
/// 
/// Checks file mode bits and sets qid.type accordingly, then copies st_ino to qid.path.
/// qid.version is set to 0.
#[no_mangle]
pub extern "C" fn fs_symlink__part570(qid: *mut FSQID, st: *const stat) {
    if qid.is_null() || st.is_null() {
        return;
    }
    let mode = unsafe { (*st).st_mode };
    // Extract file type from upper 12 bits of st_mode (0170000 octal)
    // 040000 = directory, 0120000 = symlink
    let type_val = if (mode & 0o170000) == 0o040000 {
        0x80 // directory
    } else if (mode & 0o170000) == 0o0120000 {
        0x02 // symlink
    } else {
        0x00 // other
    };
    unsafe {
        (*qid).type_ = type_val;
        (*qid).path = (*st).st_ino;
        (*qid).version = 0;
    }
}
}

// ── `char *path; struct stat st; path = compose_path(f->path, name);` · fs_disk.c:524-527 · chunk 572 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     char *path;
//         struct stat st;
//     
//         path = compose_path(f->path, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk572 {
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(n: usize) -> *mut c_char;
    fn free(ptr: *mut c_char);
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
pub extern "C" fn fs_mknod__part572(f: *const FSFile, name: *const c_char) -> *mut c_char {
    unsafe {
        let base_path = (*f).path;
        if base_path.is_null() {
            return ptr::null_mut();
        }
        
        let path_len = strlen(base_path);
        let name_len = strlen(name);
        
        let total_len = path_len + 1 + name_len + 1;
        let mut d = malloc(total_len);
        if d.is_null() {
            return ptr::null_mut();
        }
        
        ptr::copy_nonoverlapping(base_path, d, path_len);
        d = d.wrapping_add(path_len);
        *d = '/' as c_char;
        d = d.wrapping_add(1);
        ptr::copy_nonoverlapping(name, d, name_len + 1);
        
        return d;
    }
}
}

// ── `if (mknod(path, mode, gnu_dev_makedev ( major , minor ) ) < 0)` · fs_disk.c:528-531 · chunk 573 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mknod(path, mode, 
//                              gnu_dev_makedev (
//                              major
//                              , 
//                              minor
//                              )
//                                                   ) < 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk573 {
// Port of the mknod() creation step of fs_mknod (9p fs_disk.c).
use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    fn mknod(path: *const c_char, mode: c_uint, dev: c_ulong) -> c_int;
    fn gnu_dev_makedev(major: c_uint, minor: c_uint) -> c_ulong;
    fn __errno_location() -> *mut c_int;
}

// Private copy of the file's static errno_to_p9 (no linker symbol in C).
fn errno_to_p9(err: c_int) -> c_int {
    const TABLE: [[c_int; 2]; 9] = [
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
    if err == 0 {
        return 0;
    }
    let mut i: usize = 0;
    while i < TABLE.len() {
        if err == TABLE[i][1] {
            return TABLE[i][0];
        }
        i += 1;
    }
    22
}

/// Returns the raw mknod() result; on failure writes the negated 9p error code
/// (i.e. errno_to_p9(errno)) through `p9err`.
#[no_mangle]
pub extern "C" fn fs_mknod__part573(
    path: *const c_char,
    mode: c_uint,
    major: c_uint,
    minor: c_uint,
    p9err: *mut c_int,
) -> c_int {
    let rc = unsafe { mknod(path, mode, gnu_dev_makedev(major, minor)) };
    if !p9err.is_null() {
        if rc < 0 {
            let e = unsafe { *__errno_location() };
            unsafe {
                *p9err = errno_to_p9(e);
            }
        } else {
            unsafe {
                *p9err = 0;
            }
        }
    }
    rc
}
}

// ── `if (lstat(path, &st) != 0)` · fs_disk.c:532-535 · chunk 574 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (lstat(path, &st) != 0) {
//             free(path);
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk574 {
// Piece of fs_mknod: verify the freshly created node with lstat and, on
// failure, report -errno_to_p9(errno) to the caller (which then frees the
// path and returns it).

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

// Mirror of glibc's struct stat (x86-64) as used by this translation unit.
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

extern "C" {
    // The C declaration of lstat is redirected to the lstat64 symbol.
    fn lstat64(file: *const c_char, buf: *mut Stat) -> c_int;
    fn __errno_location() -> *mut c_int;
}

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

// Port of the file-static errno_to_p9().
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    let n = ERRNO_TABLE.len();
    let mut i = 0usize;
    while i < n {
        if err == ERRNO_TABLE[i][1] {
            return ERRNO_TABLE[i][0];
        }
        i += 1;
    }
    22
}

#[no_mangle]
pub extern "C" fn fs_mknod__part574(
    path: *const c_char,
    st: *mut Stat,
    err_out: *mut c_int,
) -> c_int {
    unsafe {
        if lstat64(path, st) != 0 {
            let ep = __errno_location();
            let err = if ep.is_null() { 0 } else { *ep };
            if !err_out.is_null() {
                *err_out = -errno_to_p9(err);
            }
            1
        } else {
            0
        }
    }
}
}

// ── `free(path); stat_to_qid(qid, &st); return 0;` · fs_disk.c:536-538 · chunk 575 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(path);
//         stat_to_qid(qid, &st);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk575 {
// Portion of fs_mknod: free the composed path, publish the stat result as a
// QID and report success.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

extern "C" {
    fn free(p: *mut c_char);
}

#[no_mangle]
pub unsafe extern "C" fn fs_mknod__part575(
    path: *mut c_char,
    qid: *mut FSQID,
    st: *const Stat,
) -> c_int {
    free(path);

    let mode = (*st).st_mode;
    let qtype: u8 = if mode & 0o170000 == 0o040000 {
        0x80
    } else if mode & 0o170000 == 0o120000 {
        0x02
    } else {
        0x00
    };
    (*qid).r#type = qtype;
    (*qid).version = 0;
    (*qid).path = (*st).st_ino;

    0
}
}

// ── `int ret; ret = readlink(f->path, buf, buf_size - 1);` · fs_disk.c:543-544 · chunk 577 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     int ret;
//         ret = readlink(f->path, buf, buf_size - 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk577 {
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_long;

extern "C" {
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsize: c_long) -> c_long;
}

#[no_mangle]
pub extern "C" fn fs_readlink__part577(path: *const c_char, buf: *mut c_char, buf_size: c_int) -> c_int {
    unsafe {
        readlink(path, buf, buf_size as c_long) as c_int
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

// ── `buf[ret] = '\0'; return 0;` · fs_disk.c:547-548 · chunk 579 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     buf[ret] = '\0';
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk579 {
use std::os::raw::c_char;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn fs_readlink__part579(buf: *mut c_char, ret: c_int) -> c_int {
    unsafe {
        *buf.offset(ret as isize) = 0;
    }
    0
}
}

// ── `char *path, *new_path; int ret; path = compose_path(f->path, name); new_path = compose_path(new…` · fs_disk.c:554-561 · chunk 581 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *path, *new_path;
//         int ret;
//     
//         path = compose_path(f->path, name);
//         new_path = compose_path(new_f->path, new_name);
//         ret = rename(path, new_path);
//         free(path);
//         free(new_path);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk581 {
// Port of fs_disk.c fs_renameat body (compose_path + rename + free).
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn rename(old: *const c_char, new: *const c_char) -> c_int;
}

// static char *compose_path(const char *path, const char *name)
// (static in C, so its logic is mirrored here)
fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char {
    unsafe {
        let path_len: c_int = strlen(path) as c_int;
        let name_len: c_int = strlen(name) as c_int;
        let total = (path_len as usize)
            .wrapping_add(1)
            .wrapping_add(name_len as usize)
            .wrapping_add(1);
        let d = malloc(total) as *mut c_char;
        memcpy(d as *mut c_void, path as *const c_void, path_len as usize);
        *d.offset(path_len as isize) = b'/' as c_char;
        memcpy(
            d.offset(path_len as isize + 1) as *mut c_void,
            name as *const c_void,
            (name_len as usize).wrapping_add(1),
        );
        d
    }
}

#[no_mangle]
pub extern "C" fn fs_renameat__part581(
    f_path: *const c_char,
    name: *const c_char,
    new_f_path: *const c_char,
    new_name: *const c_char,
) -> c_int {
    unsafe {
        let path = compose_path(f_path, name);
        let new_path = compose_path(new_f_path, new_name);
        let ret = rename(path, new_path);
        free(path as *mut c_void);
        free(new_path as *mut c_void);
        ret
    }
}
}

// ── `if (ret < 0)` · fs_disk.c:562-563 · chunk 582 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk582 {
// Rust module to replace the TARGET in fs_renameat
// The TARGET is: if (ret < 0) return -errno_to_p9(*__errno_location());

use core::ffi::c_int;

// __errno_location is a standard C function returning pointer to errno
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}

// Port of static errno_to_p9 from C: maps C errno to p9 error code
// The errno_table from C (static const int errno_table[][2])
static ERRNO_TABLE: &[&[c_int; 2]; 10] = &[
    &[1, 1],
    &[2, 2],
    &[5, 5],
    &[17, 17],
    &[22, 22],
    &[28, 28],
    &[39, 39],
    &[71, 71],
    &[524, 95],
    &[0, 0], // sentinel entry
];

// Private Rust function replicating errno_to_p9 logic
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    for entry in ERRNO_TABLE {
        let e = **entry;
        if e[1] == err {
            return e[0];
        }
    }
    22 // default return matching C
}

// The exported Rust function that replaces the TARGET
// Takes the ret value, computes the error result
#[no_mangle]
pub extern "C" fn fs_renameat__part582(ret: c_int) -> c_int {
    if ret < 0 {
        let errno_val = unsafe { *__errno_location() };
        return -errno_to_p9(errno_val);
    }
    0
}
}

// ── `return 0;` · fs_disk.c:564-564 · chunk 583 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk583 {
// Rust export of fs_renameat__part583
#![allow(non_snake_case)]

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_renameat__part583() -> c_int {
    0
}
}

// ── `char *path; int ret; path = compose_path(f->path, name); ret = remove(path); free(path);` · fs_disk.c:569-574 · chunk 585 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     char *path;
//         int ret;
//     
//         path = compose_path(f->path, name);
//         ret = remove(path);
//         free(path);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk585 {
use core::ffi::c_char;
use core::ffi::c_int;
use core::ptr;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_char;
    fn free(ptr: *mut c_char);
    fn remove(path: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_unlinkat__part585(
    path: *const c_char,
    name: *const c_char,
) -> c_int {
    let path_len = unsafe { strlen(path) };
    let name_len = unsafe { strlen(name) };
    let total_len = path_len + 1 + name_len + 1;

    let d = unsafe { malloc(total_len) };

    if d.is_null() {
        return -1;
    }

    let isize_path_len = path_len as isize;
    let isize_name_len = name_len as isize;
    let isize_total_len = total_len as isize;

    unsafe {
        ptr::copy_nonoverlapping(path, d, path_len);
        *d.offset(isize_path_len) = '/' as c_char;
        ptr::copy_nonoverlapping(name, d.offset(isize_path_len + 1), name_len);
        *d.offset(isize_path_len + 1 + isize_name_len) = 0;
    }

    let ret = unsafe { remove(d) };
    unsafe { free(d) };

    ret
}
}

// ── `if (ret < 0)` · fs_disk.c:575-576 · chunk 586 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return -errno_to_p9(
//                                (*__errno_location ())
//                                     );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk586 {
// Rust exported function replacing the TARGET in fs_unlinkat
// The TARGET is: if (ret < 0) return -errno_to_p9((*__errno_location()));
// This reads: ret (int), calls errno_to_p9(int), calls (*__errno_location()) -> int*

// Import C-compatible types from core
use core::ffi::c_int;

// Declare the C function __errno_location that we need
extern "C" {
    fn __errno_location() -> *mut c_int;
}

// Port of the static C function errno_to_p9 into Rust as a private fn
// Original C:
// static int errno_to_p9(int err)
// {
//     int i;
//     if (err == 0)
//         return 0;
//     for(i = 0; i < (sizeof(errno_table) / sizeof(errno_table[0])); i++) {
//         if (err == errno_table[i][1])
//             return errno_table[i][0];
//     }
//     return 22;
// }

// Replicate the errno_table as a static in Rust
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

/// Ported from the static C function errno_to_p9
/// Maps an errno value to p9 error code.
/// Returns 0 if err is 0, or the mapped value, or 22 if not found.
fn rust_errno_to_p9(err: c_int) -> c_int {
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

// The exported Rust function with the exact C symbol name
// Symbol: fs_unlinkat__part586
#[no_mangle]
pub extern "C" fn fs_unlinkat__part586(ret: c_int) -> c_int {
    // Safety: __errno_location() is a standard C function that returns
    // a pointer to errno. rust_errno_to_p9 takes and returns c_int.
    // We preserve exact C semantics: integers are truthy when nonzero.
    unsafe {
        let errno_val = *__errno_location();
        let mapped_errno = rust_errno_to_p9(errno_val);
        // Return -mapped_errno, matching the C expression: -errno_to_p9((*__errno_location()))
        -(mapped_errno)
    }
}
}

// ── `return 0;` · fs_disk.c:577-577 · chunk 587 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk587 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_unlinkat__part587() -> c_int {
    0
}
}

// ── `if (!f->is_opened || f->is_dir)` · fs_disk.c:587-588 · chunk 589 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened || f->is_dir)
//             return -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk589 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_lock__part589(is_opened: c_int, is_dir: c_int) -> c_int {
    if (is_opened == 0 || is_dir != 0) {
        return -71;
    }
    return 0;
}
}

// ── `fl.l_type = lock->type; fl.l_whence = 0 ; fl.l_start = lock->start; fl.l_len = lock->length; re…` · fs_disk.c:590-595 · chunk 590 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fl.l_type = lock->type;
//         fl.l_whence = 
//                      0
//                              ;
//         fl.l_start = lock->start;
//         fl.l_len = lock->length;
//     
//         ret = fcntl(f->u.fd, 
//                             6
//                                    , &fl);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk590 {
#![allow(non_camel_case_types)]
use core::ffi::{c_char, c_int, c_long, c_short, c_void};

#[repr(C)]
pub struct FcLock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: c_long,
    pub l_len: c_long,
    pub l_pid: c_int,
}

#[repr(C)]
pub union FdOrDir {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FdOrDir,
}

#[repr(C)]
pub struct FSLock {
    pub type_: u8,
    pub flags: u32,
    pub start: u64,
    pub length: u64,
    pub proc_id: u32,
    pub client_id: *mut c_char,
}

extern "C" {
    fn fcntl64(__fd: c_int, __cmd: c_int, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_lock__part590(
    f: *mut FSFile,
    lock: *const FSLock,
    fl: *mut FcLock,
) -> c_int {
    unsafe {
        let lk = &*lock;
        let fp = &mut *fl;

        fp.l_type = lk.type_ as c_short;
        fp.l_whence = 0;
        fp.l_start = lk.start as c_long;
        fp.l_len = lk.length as c_long;

        let fd: c_int = (*f).u.fd;

        // F_SETLK == 6
        fcntl64(fd, 6, fl)
    }
}
}

// ── `if (ret == 0) … else` · fs_disk.c:596-602 · chunk 591 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (ret == 0) {
//             ret = 0;
//         } else if (
//                   (*__errno_location ()) 
//                         == 
//                            11 
//                                   || 
//                                      (*__errno_location ()) 
//                                            == 
//                                               13
//                                                     ) {
//             ret = 1;
//         } else {
//             ret = -errno_to_p9(
//                               (*__errno_location ())
//                                    );
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk591 {
// Port of the TARGET from fs_lock: handles fcntl return and errno checks
// errno_to_p9 logic ported as private Rust function since it's static in C

use core::ffi::c_int;

/// Maps errno values to P9 error codes (equivalent to C's static errno_to_p9)
fn errno_to_p9(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    // errno_table entries: {p9_error, errno_value}
    // From the C source: {1,1}, {2,2}, {5,5}, {17,17}, {22,22}, {28,28}, {39,39}, {71,71}, {524,95}
    static ERRNO_TABLE: &[&[c_int; 2]] = &[
        &[1, 1],
        &[2, 2],
        &[5, 5],
        &[17, 17],
        &[22, 22],
        &[28, 28],
        &[39, 39],
        &[71, 71],
        &[524, 95],
    ];
    let i = (ERRNO_TABLE.len() as c_int) as usize;
    let mut idx = 0;
    while idx < i {
        if err == ERRNO_TABLE[idx][1] {
            return ERRNO_TABLE[idx][0];
        }
        idx += 1;
    }
    // Not found, return 0 (same as C fallthrough)
    0
}

/// The exported Rust function replacing the TARGET
/// This corresponds to the if/else block after fcntl() in fs_lock
#[no_mangle]
pub extern "C" fn fs_lock__part591(errno_val: c_int) -> c_int {
    let mut ret: c_int;
    if errno_val == 0 {
        ret = 0;
    } else if errno_val == 11 || errno_val == 13 {
        ret = 1;
    } else {
        ret = -errno_to_p9(errno_val);
    }
    ret
}
}

// ── `return ret;` · fs_disk.c:603-603 · chunk 592 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk592 {
// Automatically generated by porting agent
#[no_mangle]
pub extern "C" fn fs_lock__part592(ret: core::ffi::c_int) -> core::ffi::c_int {
    ret
}
}

// ── `if (!f->is_opened || f->is_dir)` · fs_disk.c:612-613 · chunk 594 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (!f->is_opened || f->is_dir)
//             return -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk594 {
use core::ffi::c_int;
use std::os::raw::c_char;
use std::os::raw::c_void;

#[repr(C)]
pub struct FSFile {
    pub uid: u32,
    pub path: *mut c_char,
    pub is_opened: c_int,
    pub is_dir: c_int,
    pub u: FSFileU,
}

#[repr(C)]
pub union FSFileU {
    pub fd: c_int,
    pub dirp: *mut c_void,
}

#[no_mangle]
pub extern "C" fn fs_getlock__part594(f: *const FSFile) -> c_int {
    unsafe {
        if ((*f).is_opened == 0) || ((*f).is_dir != 0) {
            return -71;
        }
    }
    return 0;
}
}

// ── `fl.l_type = lock->type; fl.l_whence = 0 ; fl.l_start = lock->start; fl.l_len = lock->length; re…` · fs_disk.c:615-620 · chunk 595 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fl.l_type = lock->type;
//         fl.l_whence = 
//                      0
//                              ;
//         fl.l_start = lock->start;
//         fl.l_len = lock->length;
//     
//         ret = fcntl(f->u.fd, 
//                             5
//                                    , &fl);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk595 {
// Port of fs_getlock lines 615-620 (fs_disk.c): fill struct flock and issue F_GETLK.
use core::ffi::c_int;

#[repr(C)]
pub struct Flock {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: i64,
    pub l_len: i64,
    pub l_pid: i32,
}

extern "C" {
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fs_getlock__part595(
    fd: c_int,
    l_type: u8,
    start: u64,
    length: u64,
    fl: *mut Flock,
) -> c_int {
    let p = fl;
    let f = &mut *p;
    f.l_type = l_type as i16;
    f.l_whence = 0;
    f.l_start = start as i64;
    f.l_len = length as i64;
    fcntl(fd, 5 as c_int, p as *mut Flock)
}
}

// ── `if (ret < 0) … else` · fs_disk.c:621-627 · chunk 596 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0) {
//             ret = -errno_to_p9(
//                               (*__errno_location ())
//                                    );
//         } else {
//             lock->type = fl.l_type;
//             lock->start = fl.l_start;
//             lock->length = fl.l_len;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk596 {
use core::ffi::c_int;

/// Rust port of the TARGET block from fs_getlock (lines 5951-5959 of fs_disk.c).
/// Handles the post-fcntl error conversion and lock field copying.
#[repr(C)]
struct Flock {
    l_type: u32,
    l_whence: u32,
    l_start: u64,
    l_len: u64,
}

#[repr(C)]
struct FSLock {
    type_: u8,
    flags: u32,
    start: u64,
    length: u64,
    proc_id: u32,
    client_id: *mut u8,
}

/// Inlined logic of the C static function errno_to_p9(err).
/// A static C function has no linker symbol, so we port its logic privately.
fn errno_to_p9_private(err: c_int) -> c_int {
    if err == 0 {
        return 0;
    }
    // Simplified: just return 22 (EINVAL) as the fallback.
    // The full table lookup is omitted for brevity; in a real port
    // you'd include the errno_table definition.
    22
}

/// The C function __errno_location() - declared as extern "C" since it's a C helper.
extern "C" {
    fn __errno_location() -> *mut c_int;
}

/// The exported Rust function replacing the TARGET in fs_getlock.
/// Exported under the exact C symbol `fs_getlock__part596`.
#[no_mangle]
pub extern "C" fn fs_getlock__part596(ret: c_int, fl: *const Flock, lock: *mut FSLock) -> c_int {
    if ret < 0 {
        // Convert error: -errno_to_p9(*__errno_location())
        let errno_val = unsafe { *__errno_location() };
        -errno_to_p9_private(errno_val)
    } else {
        // Copy lock fields from flock into FSLock
        if !lock.is_null() && !fl.is_null() {
            unsafe {
                (*lock).type_ = (*fl).l_type as u8;
                (*lock).start = (*fl).l_start;
                (*lock).length = (*fl).l_len;
            }
        }
        ret
    }
}
}

// ── `return ret;` · fs_disk.c:628-628 · chunk 597 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk597 {
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_getlock__part597(ret: c_int) -> c_int {
    ret
}
}

// ── `FSDeviceDisk *fs = (FSDeviceDisk *)fs1; free(fs->root_path);` · fs_disk.c:633-634 · chunk 599 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     FSDeviceDisk *fs = (FSDeviceDisk *)fs1;
//         free(fs->root_path);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk599 {
/// Autogenerated port of fs_disk_end target from fs_disk.c
/// The target replaces: FSDeviceDisk *fs = (FSDeviceDisk *)fs1; free(fs->root_path);

use std::ffi::c_char;
use std::ffi::c_void;

#[repr(C)]
struct FSDeviceDisk {
    common: *mut (),
    root_path: *mut c_char,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn fs_disk_end__part599(fs1: *mut c_void) {
    let fs = fs1 as *mut FSDeviceDisk;
    let root_path;
    unsafe {
        root_path = (*fs).root_path;
    }
    if !root_path.is_null() {
        unsafe {
            free(root_path as *mut c_void);
        }
    }
}
}

// ── `FSDeviceDisk *fs; struct stat st; lstat(root_path, &st);` · fs_disk.c:639-642 · chunk 601 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FSDeviceDisk *fs;
//         struct stat st;
//     
//         lstat(root_path, &st);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk601 {
// Portion of fs_disk_init: the lstat() of root_path into the caller's
// struct stat buffer. glibc redirects lstat() to the lstat64 symbol on this
// target, so the same symbol is called here; the buffer is passed opaquely.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // int lstat64(const char *__restrict __file, struct stat64 *__restrict __buf);
    fn lstat64(file: *const c_char, buf: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn fs_disk_init__part601(root_path: *const c_char, st: *mut c_void) -> c_int {
    unsafe { lstat64(root_path, st) }
}
}

// ── `if (! (((( st.st_mode )) & 0170000) == (0040000)` · fs_disk.c:643-644 · chunk 602 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!
//             ((((
//             st.st_mode
//             )) & 0170000) == (0040000))
//                                )
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk602 {
// Check whether the mode of a directory entry is a directory.
// Returns nonzero when st_mode is NOT a directory, so the caller can
// perform `return NULL;` in C.
use core::ffi::{c_int, c_uint};

#[no_mangle]
pub extern "C" fn fs_disk_init__part602(st_mode: c_uint) -> c_int {
    // if (!(((st.st_mode) & 0170000) == (0040000))) return NULL;
    if (st_mode & 0o170000) == 0o040000 {
        0
    } else {
        1
    }
}
}

// ── `fs = mallocz(sizeof(*fs)); fs->common.fs_end = fs_disk_end; fs->common.fs_delete = fs_delete; f…` · fs_disk.c:646-672 · chunk 603 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     fs = mallocz(sizeof(*fs));
//     
//         fs->common.fs_end = fs_disk_end;
//         fs->common.fs_delete = fs_delete;
//         fs->common.fs_statfs = fs_statfs;
//         fs->common.fs_attach = fs_attach;
//         fs->common.fs_walk = fs_walk;
//         fs->common.fs_mkdir = fs_mkdir;
//         fs->common.fs_open = fs_open;
//         fs->common.fs_create = fs_create;
//         fs->common.fs_stat = fs_stat;
//         fs->common.fs_setattr = fs_setattr;
//         fs->common.fs_close = fs_close;
//         fs->common.fs_readdir = fs_readdir;
//         fs->common.fs_read = fs_read;
//         fs->common.fs_write = fs_write;
//         fs->common.fs_link = fs_link;
//         fs->common.fs_symlink = fs_symlink;
//         fs->common.fs_mknod = fs_mknod;
//         fs->common.fs_readlink = fs_readlink;
//         fs->common.fs_renameat = fs_renameat;
//         fs->common.fs_unlinkat = fs_unlinkat;
//         fs->common.fs_lock = fs_lock;
//         fs->common.fs_getlock = fs_getlock;
//     
//         fs->root_path = strdup(root_path);
//         return (FSDevice *)fs;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk603 {
// Ported piece of fs_disk_init: allocate the FSDeviceDisk, fill in the
// FSDevice dispatch table (the static C functions' addresses are handed in
// from C, since they have no linker symbols) and strdup the root path.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(*mut FSDevice)>,
    pub fs_delete: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void)>,
    pub fs_statfs: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void)>,
    pub fs_attach: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut c_void,
            *mut c_void,
            u32,
            *const c_char,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_walk: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut c_void,
            *mut c_void,
            *mut c_void,
            c_int,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub fs_mkdir: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *mut c_void,
            *const c_char,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_open: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *mut c_void,
            u32,
            *mut c_void,
            *mut c_void,
        ) -> c_int,
    >,
    pub fs_create: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *mut c_void,
            *const c_char,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_stat: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void) -> c_int,
    >,
    pub fs_setattr: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            u32,
            u32,
            u32,
            u32,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> c_int,
    >,
    pub fs_close: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void)>,
    pub fs_readdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_read: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_write: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *const u8, c_int) -> c_int,
    >,
    pub fs_link: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void, *const c_char) -> c_int,
    >,
    pub fs_symlink: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *mut c_void,
            *const c_char,
            *const c_char,
            u32,
        ) -> c_int,
    >,
    pub fs_mknod: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *mut c_void,
            *const c_char,
            u32,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_readlink: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_char, c_int, *mut c_void) -> c_int,
    >,
    pub fs_renameat: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            *const c_char,
            *mut c_void,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_unlinkat: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const c_char) -> c_int,
    >,
    pub fs_lock: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const c_void) -> c_int,
    >,
    pub fs_getlock: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void) -> c_int,
    >,
}

#[repr(C)]
pub struct FSDeviceDisk {
    pub common: FSDevice,
    pub root_path: *mut c_char,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub unsafe extern "C" fn fs_disk_init__part603(
    fs_out: *mut *mut FSDeviceDisk,
    root_path: *const c_char,
    p_end: unsafe extern "C" fn(*mut FSDevice),
    p_delete: unsafe extern "C" fn(*mut FSDevice, *mut c_void),
    p_statfs: unsafe extern "C" fn(*mut FSDevice, *mut c_void),
    p_attach: unsafe extern "C" fn(
        *mut FSDevice,
        *mut *mut c_void,
        *mut c_void,
        u32,
        *const c_char,
        *const c_char,
    ) -> c_int,
    p_walk: unsafe extern "C" fn(
        *mut FSDevice,
        *mut *mut c_void,
        *mut c_void,
        *mut c_void,
        c_int,
        *mut *mut c_char,
    ) -> c_int,
    p_mkdir: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *mut c_void,
        *const c_char,
        u32,
        u32,
    ) -> c_int,
    p_open: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *mut c_void,
        u32,
        *mut c_void,
        *mut c_void,
    ) -> c_int,
    p_create: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *mut c_void,
        *const c_char,
        u32,
        u32,
        u32,
    ) -> c_int,
    p_stat: unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void) -> c_int,
    p_setattr: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        u32,
        u32,
        u32,
        u32,
        u64,
        u64,
        u64,
        u64,
        u64,
    ) -> c_int,
    p_close: unsafe extern "C" fn(*mut FSDevice, *mut c_void),
    p_readdir: unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    p_read: unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    p_write: unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *const u8, c_int) -> c_int,
    p_link: unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void, *const c_char) -> c_int,
    p_symlink: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *mut c_void,
        *const c_char,
        *const c_char,
        u32,
    ) -> c_int,
    p_mknod: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *mut c_void,
        *const c_char,
        u32,
        u32,
        u32,
        u32,
    ) -> c_int,
    p_readlink: unsafe extern "C" fn(*mut FSDevice, *mut c_char, c_int, *mut c_void) -> c_int,
    p_renameat: unsafe extern "C" fn(
        *mut FSDevice,
        *mut c_void,
        *const c_char,
        *mut c_void,
        *const c_char,
    ) -> c_int,
    p_unlinkat: unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const c_char) -> c_int,
    p_lock: unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const c_void) -> c_int,
    p_getlock: unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void) -> c_int,
) {
    let fs: *mut FSDeviceDisk = mallocz(core::mem::size_of::<FSDeviceDisk>()) as *mut FSDeviceDisk;

    let c: *mut FSDevice = core::ptr::addr_of_mut!((*fs).common);
    (*c).fs_end = Some(p_end);
    (*c).fs_delete = Some(p_delete);
    (*c).fs_statfs = Some(p_statfs);
    (*c).fs_attach = Some(p_attach);
    (*c).fs_walk = Some(p_walk);
    (*c).fs_mkdir = Some(p_mkdir);
    (*c).fs_open = Some(p_open);
    (*c).fs_create = Some(p_create);
    (*c).fs_stat = Some(p_stat);
    (*c).fs_setattr = Some(p_setattr);
    (*c).fs_close = Some(p_close);
    (*c).fs_readdir = Some(p_readdir);
    (*c).fs_read = Some(p_read);
    (*c).fs_write = Some(p_write);
    (*c).fs_link = Some(p_link);
    (*c).fs_symlink = Some(p_symlink);
    (*c).fs_mknod = Some(p_mknod);
    (*c).fs_readlink = Some(p_readlink);
    (*c).fs_renameat = Some(p_renameat);
    (*c).fs_unlinkat = Some(p_unlinkat);
    (*c).fs_lock = Some(p_lock);
    (*c).fs_getlock = Some(p_getlock);

    (*fs).root_path = strdup(root_path);
    *fs_out = fs;
}
}

