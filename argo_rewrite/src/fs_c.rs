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

