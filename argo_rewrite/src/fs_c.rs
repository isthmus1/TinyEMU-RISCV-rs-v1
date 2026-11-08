//! From `fs.c`.

// ── `fs_dup` · fs.c:34-39 · chunk 473 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FSFile *fs_dup(FSDevice *fs, FSFile *f)
//     {
//         FSQID qid;
//         fs->fs_walk(fs, &f, &qid, f, 0, 
//                                        ((void *)0)
//                                            );
//         return f;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk473 {
// Rust rewrite of `FSFile *fs_dup(FSDevice *fs, FSFile *f)` from fs.c.

use core::ffi::{c_char, c_int, c_void};

/// Opaque handle to a filesystem object on the C side.
#[repr(C)]
pub struct FSFile {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
pub struct FSStatFS {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
}

#[repr(C)]
pub struct FSStat {
    pub qid: FSQID,
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
pub struct FSLock {
    pub r#type: u8,
    pub flags: u32,
    pub start: u64,
    pub length: u64,
    pub proc_id: u32,
    pub client_id: *mut c_char,
}

pub type FSOpenCompletionFunc =
    Option<unsafe extern "C" fn(*mut FSDevice, *mut FSQID, c_int, *mut c_void)>;

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(*mut FSDevice)>,
    pub fs_delete: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_statfs: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSStatFS)>,
    pub fs_attach: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            u32,
            *const c_char,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_walk: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            *mut FSFile,
            c_int,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub fs_mkdir: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_open: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            u32,
            FSOpenCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub fs_create: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_stat:
        Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSStat) -> c_int>,
    pub fs_setattr: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSFile,
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
    pub fs_close: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_readdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_read: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_write: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *const u8, c_int) -> c_int,
    >,
    pub fs_link: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSFile, *const c_char) -> c_int,
    >,
    pub fs_symlink: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            *const c_char,
            u32,
        ) -> c_int,
    >,
    pub fs_mknod: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_readlink: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_char, c_int, *mut FSFile) -> c_int,
    >,
    pub fs_renameat: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSFile,
            *const c_char,
            *mut FSFile,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_unlinkat:
        Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const c_char) -> c_int>,
    pub fs_lock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const FSLock) -> c_int>,
    pub fs_getlock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSLock) -> c_int>,
}

#[no_mangle]
pub unsafe extern "C" fn fs_dup(fs: *mut FSDevice, f: *mut FSFile) -> *mut FSFile {
    let mut qid = FSQID {
        r#type: 0,
        version: 0,
        path: 0,
    };
    // The parameter `f` is a local copy in C; the walk updates it in place.
    let mut f = f;
    if let Some(walk) = (*fs).fs_walk {
        walk(fs, &mut f, &mut qid, f, 0, core::ptr::null_mut());
    }
    f
}
}

// ── `FSQID qid; fs->fs_walk(fs, &f, &qid, f, 0, ((void *)0) ); return f;` · fs.c:36-38 · chunk 474 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     FSQID qid;
//         fs->fs_walk(fs, &f, &qid, f, 0, 
//                                        ((void *)0)
//                                            );
//         return f;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk474 {
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ulonglong;
use core::ffi::c_int;
use std::ffi::c_char;
use std::ptr::null_mut;

#[repr(C)]
struct FSQID {
    r#type: c_uchar,
    version: c_uint,
    path: c_ulonglong,
}

#[repr(C)]
struct FSFile;

#[repr(C)]
struct FSDevice {
    fs_walk: unsafe extern "C" fn(*mut FSDevice, *mut *mut FSFile, *mut FSQID, *mut FSFile, c_int, *mut *mut c_char) -> c_int,
}

#[no_mangle]
pub extern "C" fn fs_dup__part474(fs: *mut FSDevice, mut f: *mut FSFile) -> *mut FSFile {
    let mut qid = FSQID { r#type: 0, version: 0, path: 0 };
    unsafe {
        let walk = (*fs).fs_walk;
        walk(fs, &mut f, &mut qid as *mut FSQID, f, 0, null_mut());
    }
    f
}
}

// ── `fs_walk_path1` · fs.c:41-93 · chunk 475 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     FSFile *fs_walk_path1(FSDevice *fs, FSFile *f, const char *path,
//                           char **pname)
//     {
//         const char *p;
//         char *name;
//         FSFile *f1;
//         FSQID qid;
//         int len, ret;
//         BOOL is_last, is_first;
//     
//         if (path[0] == '/')
//             path++;
//     
//         is_first = TRUE;
//         for(;;) {
//             p = 
//                _Generic (0 ? (
//                path
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                path
//                , 
//                '/'
//                )), default: strchr (
//                path
//                , 
//                '/'
//                ))
//                                 ;
//             if (!p) {
//                 name = (char *)path;
//                 if (pname) {
//                     *pname = name;
//                     if (is_first) {
//                         ret = fs->fs_walk(fs, &f, &qid, f, 0, 
//                                                              ((void *)0)
//                                                                  );
//                         if (ret < 0)
//                             f = 
//                                ((void *)0)
//                                    ;
//                     }
//                     return f;
//                 }
//                 is_last = TRUE;
//             } else {
//                 len = p - path;
//                 name = malloc(len + 1);
//                 memcpy(name, path, len);
//                 name[len] = '\0';
//                 is_last = FALSE;
//             }
//             ret = fs->fs_walk(fs, &f1, &qid, f, 1, &name);
//             if (!is_last)
//                 free(name);
//             if (!is_first)
//                 fs->fs_delete(fs, f);
//             f = f1;
//             is_first = FALSE;
//             if (ret <= 0) {
//                 fs->fs_delete(fs, f);
//                 f = 
//                    ((void *)0)
//                        ;
//                 break;
//             } else if (is_last) {
//                 break;
//             }
//             path = p + 1;
//         }
//         return f;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk475 {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C types the target can see.

#[repr(C)]
pub struct FSStatFS {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
}

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
pub struct FSStat {
    pub qid: FSQID,
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
pub struct FSLock {
    pub r#type: u8,
    pub flags: u32,
    pub start: u64,
    pub length: u64,
    pub proc_id: u32,
    pub client_id: *mut c_char,
}

// Opaque struct: only ever handled through pointers.
#[repr(C)]
pub struct FSFile {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(*mut FSDevice)>,
    pub fs_delete: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_statfs: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSStatFS)>,
    pub fs_attach: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            u32,
            *const c_char,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_walk: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            *mut FSFile,
            c_int,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub fs_mkdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSQID, *mut FSFile, *const c_char, u32, u32)
            -> c_int,
    >,
    pub fs_open: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            u32,
            Option<unsafe extern "C" fn(*mut FSDevice, *mut FSQID, c_int, *mut c_void)>,
            *mut c_void,
        ) -> c_int,
    >,
    pub fs_create: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_stat: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSStat) -> c_int>,
    pub fs_setattr: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSFile,
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
    pub fs_close: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_readdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_read: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_write: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *const u8, c_int) -> c_int,
    >,
    pub fs_link: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSFile, *const c_char) -> c_int,
    >,
    pub fs_symlink: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            *const c_char,
            u32,
        ) -> c_int,
    >,
    pub fs_mknod: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_readlink: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_char, c_int, *mut FSFile) -> c_int,
    >,
    pub fs_renameat: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const c_char, *mut FSFile, *const c_char)
            -> c_int,
    >,
    pub fs_unlinkat:
        Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const c_char) -> c_int>,
    pub fs_lock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const FSLock) -> c_int>,
    pub fs_getlock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSLock) -> c_int>,
}

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn fs_walk_path1(
    fs: *mut FSDevice,
    mut f: *mut FSFile,
    mut path: *const c_char,
    pname: *mut *mut c_char,
) -> *mut FSFile {
    let mut p: *const c_char;
    let mut name: *mut c_char = core::ptr::null_mut();
    let mut f1: *mut FSFile = core::ptr::null_mut();
    let mut qid: FSQID = core::mem::zeroed();
    let mut len: isize = 0;
    let mut ret: c_int = 0;
    let mut is_last: bool;
    let mut is_first: bool;

    // The C code calls through the vtable unconditionally; bail out to NULL
    // rather than panic if a slot is unexpectedly empty.
    let walk = match (*fs).fs_walk {
        Some(w) => w,
        None => return core::ptr::null_mut(),
    };
    let delete = match (*fs).fs_delete {
        Some(d) => d,
        None => return core::ptr::null_mut(),
    };

    if *path == b'/' as c_char {
        path = path.add(1);
    }

    is_first = true;
    loop {
        p = strchr(path, b'/' as c_int);
        if p.is_null() {
            name = path as *mut c_char;
            if !pname.is_null() {
                *pname = name;
                if is_first {
                    let mut tmp = f;
                    ret = walk(fs, &mut tmp, &mut qid, f, 0, core::ptr::null_mut());
                    f = tmp;
                    if ret < 0 {
                        f = core::ptr::null_mut();
                    }
                }
                return f;
            }
            is_last = true;
        } else {
            len = (p as usize).wrapping_sub(path as usize) as isize;
            name = malloc((len as usize).wrapping_add(1)) as *mut c_char;
            memcpy(name as *mut c_void, path as *const c_void, len as usize);
            *name.offset(len) = 0;
            is_last = false;
        }
        ret = walk(fs, &mut f1, &mut qid, f, 1, &mut name);
        if !is_last {
            free(name as *mut c_void);
        }
        if !is_first {
            delete(fs, f);
        }
        f = f1;
        is_first = false;
        if ret <= 0 {
            delete(fs, f);
            f = core::ptr::null_mut();
            break;
        } else if is_last {
            break;
        }
        path = p.add(1);
    }
    f
}
}

// ── `if (path[0] == '/')` · fs.c:51-52 · chunk 476 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (path[0] == '/')
//             path++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk476 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn fs_walk_path1__part476(path: *const c_char) -> *const c_char {
    if !path.is_null() {
        unsafe {
            if *path == '/' as c_char {
                return path.offset(1);
            }
        }
    }
    path
}
}

// ── `is_first = TRUE;` · fs.c:54-54 · chunk 477 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     is_first = TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk477 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn fs_walk_path1__part477(is_first: *mut c_int) {
    unsafe {
        *is_first = 1;
    }
}
}

// ── `for (; ; )` · fs.c:55-91 · chunk 478 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             p = 
//                _Generic (0 ? (
//                path
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                path
//                , 
//                '/'
//                )), default: strchr (
//                path
//                , 
//                '/'
//                ))
//                                 ;
//             if (!p) {
//                 name = (char *)path;
//                 if (pname) {
//                     *pname = name;
//                     if (is_first) {
//                         ret = fs->fs_walk(fs, &f, &qid, f, 0, 
//                                                              ((void *)0)
//                                                                  );
//                         if (ret < 0)
//                             f = 
//                                ((void *)0)
//                                    ;
//                     }
//                     return f;
//                 }
//                 is_last = TRUE;
//             } else {
//                 len = p - path;
//                 name = malloc(len + 1);
//                 memcpy(name, path, len);
//                 name[len] = '\0';
//                 is_last = FALSE;
//             }
//             ret = fs->fs_walk(fs, &f1, &qid, f, 1, &name);
//             if (!is_last)
//                 free(name);
//             if (!is_first)
//                 fs->fs_delete(fs, f);
//             f = f1;
//             is_first = FALSE;
//             if (ret <= 0) {
//                 fs->fs_delete(fs, f);
//                 f = 
//                    ((void *)0)
//                        ;
//                 break;
//             } else if (is_last) {
//                 break;
//             }
//             path = p + 1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk478 {
// Port of the walk loop of fs_walk_path1 (fs.c lines 55-91).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// Opaque handle types from the C program.
#[repr(C)]
pub struct FSFile {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FSQID {
    pub type_: u8,
    pub version: u32,
    pub path: u64,
}

// Mirrors the leading fields of struct FSDevice, in order.  Only the fields
// reached by this code are needed; the offsets of a #[repr(C)] prefix match
// the C struct exactly.
#[repr(C)]
pub struct FSDevice {
    pub fs_end: unsafe extern "C" fn(*mut FSDevice),
    pub fs_delete: unsafe extern "C" fn(*mut FSDevice, *mut FSFile),
    pub fs_statfs: *mut c_void,
    pub fs_attach: *mut c_void,
    pub fs_walk: unsafe extern "C" fn(
        *mut FSDevice,
        *mut *mut FSFile,
        *mut FSQID,
        *mut FSFile,
        c_int,
        *mut *mut c_char,
    ) -> c_int,
}

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn fs_walk_path1__part478(
    fs: *mut FSDevice,
    f_in: *mut FSFile,
    mut path: *const c_char,
    pname: *mut *mut c_char,
    qid: *mut FSQID,
    mut is_first: c_int,
) -> *mut FSFile {
    let mut f: *mut FSFile = f_in;
    let mut p: *mut c_char;
    let mut name: *mut c_char;
    let mut f1: *mut FSFile;
    let mut ret: c_int;
    let mut len: isize;
    let mut is_last: bool;

    loop {
        p = strchr(path, '/' as c_int);
        if p.is_null() {
            name = path as *mut c_char;
            if !pname.is_null() {
                *pname = name;
                if is_first != 0 {
                    ret = ((*fs).fs_walk)(fs, core::ptr::addr_of_mut!(f), qid, f, 0, core::ptr::null_mut());
                    if ret < 0 {
                        f = core::ptr::null_mut();
                    }
                }
                return f;
            }
            is_last = true;
        } else {
            len = (p as isize).wrapping_sub(path as isize);
            name = malloc((len + 1) as usize) as *mut c_char;
            memcpy(name as *mut c_void, path as *const c_void, len as usize);
            *name.offset(len) = 0;
            is_last = false;
        }
        f1 = core::ptr::null_mut();
        ret = ((*fs).fs_walk)(fs, core::ptr::addr_of_mut!(f1), qid, f, 1, core::ptr::addr_of_mut!(name));
        if !is_last {
            free(name as *mut c_void);
        }
        if is_first == 0 {
            ((*fs).fs_delete)(fs, f);
        }
        f = f1;
        is_first = 0;
        if ret <= 0 {
            ((*fs).fs_delete)(fs, f);
            f = core::ptr::null_mut();
            break;
        } else if is_last {
            break;
        }
        path = p.offset(1);
    }
    f
}
}

// ── `return f;` · fs.c:92-92 · chunk 479 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return f;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk479 {
use core::ffi::c_char;

#[repr(C)]
pub struct FSDevice;

#[repr(C)]
pub struct FSFile;

#[no_mangle]
pub unsafe extern "C" fn fs_walk_path1__part479(
    fs: *mut FSDevice,
    f: *mut FSFile,
    path: *const c_char,
    pname: *mut *mut c_char,
) -> *mut FSFile {
    f
}
}

// ── `fs_walk_path` · fs.c:95-98 · chunk 480 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FSFile *fs_walk_path(FSDevice *fs, FSFile *f, const char *path)
//     {
//         return fs_walk_path1(fs, f, path, 
//                                          ((void *)0)
//                                              );
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk480 {
use core::ffi::{c_char, c_void};

// Opaque handles mirrored from the C side; only ever used behind pointers.
#[repr(C)]
pub struct FSDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FSFile {
    _private: [u8; 0],
}

extern "C" {
    fn fs_walk_path1(
        fs: *mut FSDevice,
        f: *mut FSFile,
        path: *const c_char,
        pname: *mut *mut c_char,
    ) -> *mut FSFile;
}

#[no_mangle]
pub extern "C" fn fs_walk_path(
    fs: *mut FSDevice,
    f: *mut FSFile,
    path: *const c_char,
) -> *mut FSFile {
    // NULL pname: the walk must fully resolve the path (no leftover name).
    let null_pname: *mut *mut c_char = core::ptr::null_mut::<*mut c_char>() as *mut *mut c_char;
    let _ = core::mem::size_of::<c_void>();
    unsafe { fs_walk_path1(fs, f, path, null_pname) }
}
}

// ── `return fs_walk_path1(fs, f, path, ((void *)0) );` · fs.c:97-97 · chunk 481 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     return fs_walk_path1(fs, f, path, 
//                                          ((void *)0)
//                                              );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk481 {
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ptr::null_mut;

extern "C" {
    fn fs_walk_path1(fs: *mut c_void, f: *mut c_void, path: *const c_char, pname: *mut *mut c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn fs_walk_path__part481(fs: *mut c_void, f: *mut c_void, path: *const c_char) -> *mut c_void {
    unsafe { fs_walk_path1(fs, f, path, null_mut()) }
}
}

// ── `fs_end` · fs.c:100-104 · chunk 482 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void fs_end(FSDevice *fs)
//     {
//         fs->fs_end(fs);
//         free(fs);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk482 {
// Rust rewrite of fs_end (fs.c lines 100-104)
use core::ffi::{c_void};

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(s: *mut FSDevice)>,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fs_end(fs: *mut FSDevice) {
    if let Some(f) = (*fs).fs_end {
        f(fs);
    }
    free(fs as *mut c_void);
}
}

// ── `fs->fs_end(fs); free(fs);` · fs.c:102-103 · chunk 483 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     fs->fs_end(fs);
//         free(fs);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk483 {
use std::os::raw::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[repr(C)]
struct FSDevice {
    fs_end: extern "C" fn(*mut FSDevice),
}

#[no_mangle]
pub extern "C" fn fs_end__part483(fs: *mut FSDevice) {
    unsafe {
        let func = (*fs).fs_end;
        func(fs);
        free(fs as *mut c_void);
    }
}
}

