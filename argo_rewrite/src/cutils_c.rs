//! From `cutils.c`.

// ── `void *ptr; ptr = malloc(size);` · cutils.c:37-38 · chunk 414 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     void *ptr;
//         ptr = malloc(size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk414 {
extern "C" {
    fn malloc(size: core::ffi::c_ulong) -> *mut core::ffi::c_void;
}

#[no_mangle]
pub extern "C" fn mallocz__part414(size: core::ffi::c_ulong) -> *mut core::ffi::c_void {
    unsafe { malloc(size) }
}
}

// ── `if (!ptr)` · cutils.c:39-40 · chunk 415 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!ptr)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk415 {
use core::ffi::c_void;

#[no_mangle]
pub extern "C" fn mallocz__part415(ptr: *mut c_void) -> *mut c_void {
    ptr
}
}

// ── `memset(ptr, 0, size); return ptr;` · cutils.c:41-42 · chunk 416 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     memset(ptr, 0, size);
//         return ptr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk416 {
use core::ffi::{c_int, c_void, c_ulong};

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn mallocz__part416(ptr: *mut c_void, size: c_ulong) -> *mut c_void {
    unsafe {
        memset(ptr, 0, size);
        ptr
    }
}
}

// ── `if (buf_size <= 0)` · cutils.c:50-51 · chunk 418 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (buf_size <= 0)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk418 {
#[no_mangle]
pub extern "C" fn pstrcpy__part418(_buf: *mut core::ffi::c_char, _buf_size: core::ffi::c_int, _str: *const core::ffi::c_char) -> core::ffi::c_int {
    if _buf_size <= 0 {
        return 1;
    }
    0
}
}

// ── `for (; ; )` · cutils.c:53-58 · chunk 419 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             c = *str++;
//             if (c == 0 || q >= buf + buf_size - 1)
//                 break;
//             *q++ = c;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk419 {
// Rust port of pstrcpy__part419 TARGET
use core::ffi::c_char;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pstrcpy__part419(buf: *mut c_char, buf_size: c_int, mut str: *const c_char) -> c_int {
    if buf_size <= 0 {
        return 0;
    }
    
    let mut q = buf;
    let mut n: c_int = 0;
    let mut c: c_int;
    
    loop {
        unsafe {
            c = *str as c_int;
            str = str.offset(1);
        }
        
        if c == 0 {
            break;
        }
        
        if n >= buf_size - 1 {
            break;
        }
        
        unsafe {
            *q = c as c_char;
            q = q.offset(1);
        }
        n += 1;
    }
    
    if c == 0 {
        0
    } else {
        1
    }
}
}

// ── `*q = '\0';` · cutils.c:59-59 · chunk 420 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q = '\0';
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk420 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn pstrcpy__part420(q: *mut c_char) {
    unsafe {
        *q = 0;
    }
}
}

// ── `int len; len = strlen(buf);` · cutils.c:64-65 · chunk 422 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     int len;
//         len = strlen(buf);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk422 {
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_ulong;

extern "C" {
    fn strlen(s: *const c_char) -> c_ulong;
}

#[no_mangle]
pub extern "C" fn pstrcat__part422(buf: *const c_char) -> c_int {
    unsafe {
        strlen(buf) as c_int
    }
}
}

// ── `if (len < buf_size)` · cutils.c:66-67 · chunk 423 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (len < buf_size)
//             pstrcpy(buf + len, buf_size - len, s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk423 {
use core::ffi::c_char;
use core::ffi::c_int;

extern "C" {
    fn pstrcpy(buf: *mut c_char, buf_size: c_int, str: *const c_char);
}

#[no_mangle]
pub extern "C" fn pstrcat__part423(buf: *mut c_char, buf_size: c_int, s: *const c_char, len: c_int) -> *mut c_char {
    if len < buf_size {
        unsafe {
            pstrcpy(buf.add(len as usize), buf_size - len, s);
        }
    }
    buf
}
}

// ── `return buf;` · cutils.c:68-68 · chunk 424 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk424 {
use std::ffi::c_char;

#[no_mangle]
pub extern "C" fn pstrcat__part424(buf: *mut c_char) -> *mut c_char {
    buf
}
}

// ── `const char *p, *q; p = str; q = val;` · cutils.c:73-75 · chunk 426 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p, *q;
//         p = str;
//         q = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk426 {
// Port of strstart__part426 TARGET
// Initializes output pointers out_p and out_q from str and val

use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn strstart__part426(
    str: *const c_char,
    val: *const c_char,
    out_p: *mut *const c_char,
    out_q: *mut *const c_char,
) {
    unsafe {
        *out_p = str;
        *out_q = val;
    }
}
}

// ── `while (*q != '\0')` · cutils.c:76-81 · chunk 427 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     while (*q != '\0') {
//             if (*p != *q)
//                 return 0;
//             p++;
//             q++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk427 {
use core::ffi::c_char;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn strstart__part427(str: *const c_char, val: *const c_char) -> c_int {
    let mut p = str;
    let mut q = val;
    unsafe {
        while *q != 0 {
            if *p != *q {
                return 0;
            }
            p = p.offset(1);
            q = q.offset(1);
        }
    }
    1
}
}

// ── `if (ptr)` · cutils.c:82-83 · chunk 428 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (ptr)
//             *ptr = p;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk428 {
use core::ffi::c_int;
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn strstart__part428(
    ptr: *mut *const c_char,
    p: *const c_char,
) -> c_int {
    if !ptr.is_null() {
        unsafe {
            *ptr = p;
        }
    }
    1i32 as c_int
}
}

// ── `return 1;` · cutils.c:84-84 · chunk 429 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk429 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn strstart__part429() -> c_int {
    1
}
}

// ── `memset(s, 0, sizeof(*s));` · cutils.c:89-89 · chunk 431 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(s, 0, sizeof(*s));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk431 {
extern "C" {
    fn memset(s: *mut std::ffi::c_void, c: i32, n: usize);
}

#[no_mangle]
pub extern "C" fn dbuf_init__part431(s: *mut std::ffi::c_void, n: usize) {
    unsafe {
        memset(s, 0, n);
    }
}
}

// ── `size_t end, new_size; new_size = end = offset + len;` · cutils.c:94-95 · chunk 433 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     size_t end, new_size;
//         new_size = end = offset + len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk433 {
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn dbuf_write__part433(offset: c_ulong, len: c_ulong) -> c_ulong {
    offset + len
}
}

// ── `if (new_size > s->allocated_size)` · cutils.c:96-100 · chunk 434 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     if (new_size > s->allocated_size) {
//             new_size = max_int(new_size, s->allocated_size * 3 / 2);
//             s->buf = realloc(s->buf, new_size);
//             s->allocated_size = new_size;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk434 {
#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

extern "C" {
    fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn dbuf_write__part434(s: *mut DynBuf, new_size: *mut usize) {
    unsafe {
        let allocated_size = (*s).allocated_size;
        if *new_size > allocated_size {
            let new_alloc = allocated_size * 3 / 2;
            let new_size_val = if *new_size > new_alloc { *new_size } else { new_alloc };
            (*s).buf = realloc((*s).buf, new_size_val);
            (*s).allocated_size = new_size_val;
            *new_size = new_size_val;
        }
    }
}
}

// ── `memcpy(s->buf + offset, data, len);` · cutils.c:101-101 · chunk 435 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     memcpy(s->buf + offset, data, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk435 {
#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

#[no_mangle]
pub extern "C" fn dbuf_write__part435(
    s: *mut DynBuf,
    offset: usize,
    data: *const u8,
    len: usize,
) {
    unsafe {
        core::ptr::copy_nonoverlapping(data, (*s).buf.add(offset), len);
    }
}
}

// ── `if (end > s->size)` · cutils.c:102-103 · chunk 436 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (end > s->size)
//             s->size = end;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk436 {
#[no_mangle]
pub extern "C" fn dbuf_write__part436(end: core::ffi::c_ulong, s_size: *mut core::ffi::c_ulong) {
    unsafe {
        if end > *s_size {
            *s_size = end;
        }
    }
}
}

// ── `dbuf_write(s, s->size, &c, 1);` · cutils.c:108-108 · chunk 438 · nvidia/nemotron-3.5-lightning · 4 attempt(s) · Unvalidated
//
// Original C:
//     dbuf_write(s, s->size, &c, 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk438 {
// Port of dbuf_write from cutils.c
use std::alloc;

#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

#[no_mangle]
pub extern "C" fn dbuf_putc__part438(s: *mut DynBuf, offset: usize, data: *const u8, len: usize) -> () {
    let end = offset + len;
    if end > unsafe { (*s).allocated_size } {
        let new_alloc = end.max(unsafe { (*s).allocated_size * 3 / 2 });
        if new_alloc > 0 {
            let layout = alloc::Layout::from_size_align(new_alloc, 1).unwrap();
            let new_buf = unsafe {
                alloc::realloc((*s).buf, layout, new_alloc)
            };
            if new_buf.is_null() {
                return;
            }
            unsafe { (*s).buf = new_buf; };
            unsafe { (*s).allocated_size = new_alloc; };
        }
    }
    let dst = unsafe { (*s).buf.add(offset) };
    let mut i = 0;
    while i < len {
        unsafe {
            *dst.add(i) = *data.add(i);
        }
        i += 1;
    }
    if end > unsafe { (*s).size } {
        unsafe { (*s).size = end; };
    }
}
}

// ── `dbuf_write(s, s->size, (const uint8_t *)str, strlen(str));` · cutils.c:113-113 · chunk 440 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     dbuf_write(s, s->size, (const uint8_t *)str, strlen(str));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk440 {
use std::ffi::c_char;
use std::ffi::CStr;

#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

extern "C" {
    fn dbuf_write(s: *mut DynBuf, offset: usize, data: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn dbuf_putstr__part440(s: *mut DynBuf, str: *const c_char) {
    unsafe {
        let len = CStr::from_ptr(str).to_bytes().len();
        let s_size = (*s).size;
        dbuf_write(s, s_size, str as *const u8, len);
    }
}
}

// ── `free(s->buf); memset(s, 0, sizeof(*s));` · cutils.c:118-119 · chunk 442 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     free(s->buf);
//         memset(s, 0, sizeof(*s));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk442 {
use core::ffi::{c_void, c_int, c_ulong, c_uchar};
use core::mem;

#[repr(C)]
struct DynBuf {
    buf: *mut c_uchar,
    size: c_ulong,
    allocated_size: c_ulong,
}

extern "C" {
    fn free(ptr: *mut c_void);
    fn memset(dest: *mut c_void, fill: c_int, len: c_ulong);
}

#[no_mangle]
pub extern "C" fn dbuf_free__part442(s: *mut DynBuf) {
    unsafe {
        free((*s).buf as *mut c_void);
        memset(s as *mut c_void, 0, mem::size_of::<DynBuf>() as c_ulong);
    }
}
}

