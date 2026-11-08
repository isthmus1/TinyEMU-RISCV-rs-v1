//! From `fs.c`.

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

