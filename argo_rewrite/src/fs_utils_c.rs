//! From `fs_utils.c`.

// ── `if (path[0] == '\0') … else` · fs_utils.c:49-61 · chunk 1085 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (path[0] == '\0') {
//             d = strdup(name);
//         } else {
//             path_len = strlen(path);
//             name_len = strlen(name);
//             d = malloc(path_len + 1 + name_len + 1);
//             q = d;
//             memcpy(q, path, path_len);
//             q += path_len;
//             if (path[path_len - 1] != '/')
//                 *q++ = '/';
//             memcpy(q, name, name_len + 1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1085 {
// Port of the compose_path body (fs_utils.c lines 49-61).
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn strdup(__s: *const c_char) -> *mut c_char;
    fn strlen(__s: *const c_char) -> usize;
    fn malloc(__size: usize) -> *mut c_void;
    fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn compose_path__part1085(path: *const c_char, name: *const c_char) -> *mut c_char {
    unsafe {
        if *path == 0 {
            strdup(name)
        } else {
            let path_len: c_int = strlen(path) as c_int;
            let name_len: c_int = strlen(name) as c_int;
            let d = malloc((path_len + 1 + name_len + 1) as usize) as *mut c_char;
            let mut q = d;
            memcpy(q as *mut c_void, path as *const c_void, path_len as usize);
            q = q.add(path_len as usize);
            if *path.offset((path_len - 1) as isize) != b'/' as c_char {
                *q = b'/' as c_char;
                q = q.add(1);
            }
            memcpy(q as *mut c_void, name as *const c_void, (name_len + 1) as usize);
            d
        }
    }
}
}

// ── `return d;` · fs_utils.c:62-62 · chunk 1086 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return d;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1086 {
// Port of the trivial `return d;` tail of compose_path: pass the pointer through.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn compose_path__part1086(d: *mut c_char) -> *mut c_char {
    d
}
}

// ── `compose_url` · fs_utils.c:65-72 · chunk 1087 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *compose_url(const char *base_url, const char *name)
//     {
//         if (
//            _Generic (0 ? (
//            name
//            ) : (void *) 1, const void *: (const char *) (strchr (
//            name
//            , 
//            ':'
//            )), default: strchr (
//            name
//            , 
//            ':'
//            ))
//                             ) {
//             return strdup(name);
//         } else {
//             return compose_path(base_url, name);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1087 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn compose_url(base_url: *const c_char, name: *const c_char) -> *mut c_char {
    unsafe {
        if !strchr(name, ':' as c_int).is_null() {
            strdup(name)
        } else {
            compose_path(base_url, name)
        }
    }
}
}

// ── `if ( _Generic (0 ? ( name ) : (void *) 1, const void *: (const char *) (strchr ( name , ':' )),…` · fs_utils.c:67-71 · chunk 1088 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            _Generic (0 ? (
//            name
//            ) : (void *) 1, const void *: (const char *) (strchr (
//            name
//            , 
//            ':'
//            )), default: strchr (
//            name
//            , 
//            ':'
//            ))
//                             ) {
//             return strdup(name);
//         } else {
//             return compose_path(base_url, name);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1088 {
// Port of the body of compose_url: if the name contains a ':',
// it is already an absolute URL (dup it), otherwise join it with the base.
use core::ffi::{c_char, c_int};

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn compose_path(path: *const c_char, name: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn compose_url__part1088(
    base_url: *const c_char,
    name: *const c_char,
) -> *mut c_char {
    unsafe {
        let colon = strchr(name, ':' as c_int);
        if !colon.is_null() {
            strdup(name)
        } else {
            compose_path(base_url, name)
        }
    }
}
}

// ── `skip_line` · fs_utils.c:74-83 · chunk 1089 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void skip_line(const char **pp)
//     {
//         const char *p;
//         p = *pp;
//         while (*p != '\n' && *p != '\0')
//             p++;
//         if (*p == '\n')
//             p++;
//         *pp = p;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1089 {
use core::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn skip_line(pp: *mut *const c_char) {
    let mut p: *const c_char = *pp;
    while (*p as u8) != b'\n' && (*p as u8) != 0 {
        p = p.add(1);
    }
    if (*p as u8) == b'\n' {
        p = p.add(1);
    }
    *pp = p;
}
}

// ── `const char *p; p = *pp;` · fs_utils.c:76-77 · chunk 1090 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p;
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1090 {
// Load the pointer stored in *pp (the TARGET: `const char *p; p = *pp;`).
use core::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn skip_line__part1090(pp: *const *const c_char) -> *const c_char {
    *pp
}
}

// ── `while (*p != '\n' && *p != '\0')` · fs_utils.c:78-79 · chunk 1091 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (*p != '\n' && *p != '\0')
//             p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1091 {
use core::ffi::c_char;

// Scans forward from `p` until a newline or the terminating NUL is found.
// Mirrors: while (*p != '\n' && *p != '\0') p++;
#[no_mangle]
pub extern "C" fn skip_line__part1091(mut p: *const c_char) -> *const c_char {
    unsafe {
        while *p != b'\n' as c_char && *p != 0 {
            p = p.add(1);
        }
    }
    p
}
}

// ── `if (*p == '\n')` · fs_utils.c:80-81 · chunk 1092 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p == '\n')
//             p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1092 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn skip_line__part1092(p: *const c_char) -> *const c_char {
    unsafe {
        if *p == b'\n' as c_char {
            p.wrapping_add(1)
        } else {
            p
        }
    }
}
}

// ── `*pp = p;` · fs_utils.c:82-82 · chunk 1093 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1093 {
// Assigns p to *pp: the tail of skip_line.
use core::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn skip_line__part1093(pp: *mut *const c_char, p: *const c_char) {
    if !pp.is_null() {
        *pp = p;
    }
}
}

// ── `quoted_str` · fs_utils.c:85-120 · chunk 1094 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *quoted_str(const char *str)
//     {
//         const char *s;
//         char *q;
//         int c;
//         char *buf;
//     
//         if (str[0] == '\0')
//             goto use_quote;
//         s = str;
//         while (*s != '\0') {
//             if (*s <= ' ' || *s > '~')
//                 goto use_quote;
//             s++;
//         }
//         return strdup(str);
//      use_quote:
//         buf = malloc(strlen(str) * 4 + 2 + 1);
//         q = buf;
//         s = str;
//         *q++ = '"';
//         while (*s != '\0') {
//             c = *(uint8_t *)s;
//             if (c < ' ' || c == 127) {
//                 q += sprintf(q, "\\x%02x", c);
//             } else if (c == '\\' || c == '\"') {
//                 q += sprintf(q, "\\%c", c);
//             } else {
//                 *q++ = c;
//             }
//             s++;
//         }
//         *q++ = '"';
//         *q = '\0';
//         return buf;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1094 {
// Rewrite of `char *quoted_str(const char *str)` from fs_utils.c
use core::ffi::{c_char, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[inline]
unsafe fn put(b: u8) -> u8 {
    b
}

#[inline]
fn hexdig(v: u8) -> u8 {
    // "%02x" digit
    if v < 10 {
        b'0' + v
    } else {
        b'a' + (v - 10)
    }
}

#[no_mangle]
pub unsafe extern "C" fn quoted_str(str_: *const c_char) -> *mut c_char {
    if str_.is_null() {
        return core::ptr::null_mut();
    }

    let mut need_quote = false;

    if *str_ == 0 {
        // empty string -> always quoted
        need_quote = true;
    } else {
        let mut s = str_ as *const u8;
        while *s != 0 {
            let c = *s;
            // C compares a (signed) char against ' ' and '~':
            //   *s <= ' '  is (c as i8) <= 32   -> c <= 0x20 || c >= 0x80
            //   *s >  '~'  is (c as i8) > 126   -> c == 0x7f
            if c <= 0x20 || c >= 0x7f {
                need_quote = true;
                break;
            }
            s = s.add(1);
        }
        if !need_quote {
            return strdup(str_);
        }
    }
    let _ = need_quote;

    let buf = malloc(strlen(str_) * 4 + 2 + 1) as *mut c_char;
    if buf.is_null() {
        return buf;
    }

    let mut q = buf as *mut u8;
    let mut s = str_ as *const u8;

    *q = b'"';
    q = q.add(1);

    while *s != 0 {
        let c = *s;
        if c < b' ' || c == 127 {
            // sprintf(q, "\\x%02x", c) -> 4 bytes, never any NUL written
            *q = b'\\';
            q = q.add(1);
            *q = b'x';
            q = q.add(1);
            *q = hexdig(c >> 4);
            q = q.add(1);
            *q = hexdig(c & 0xf);
            q = q.add(1);
        } else if c == b'\\' || c == b'"' {
            // sprintf(q, "\\%c", c) -> 2 bytes
            *q = b'\\';
            q = q.add(1);
            *q = put(c);
            q = q.add(1);
        } else {
            *q = c;
            q = q.add(1);
        }
        s = s.add(1);
    }

    *q = b'"';
    q = q.add(1);
    *q = 0;

    buf
}
}

// ── `if (str[0] == '\0')` · fs_utils.c:92-93 · chunk 1095 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str[0] == '\0')
//             goto use_quote;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1095 {
// Port of `quoted_str`'s empty-string check:
//   if (str[0] == '\0') goto use_quote;
// Returns 1 when the caller must take the `use_quote` path, 0 otherwise.

use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn quoted_str__part1095(str: *const c_char) -> c_int {
    unsafe {
        let b = *str;
        if b == 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `s = str;` · fs_utils.c:94-94 · chunk 1096 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s = str;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1096 {
// Port of the statement `s = str;` from quoted_str() in fs_utils.c.
// It simply yields the pointer passed in, so the C caller can assign it to `s`.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn quoted_str__part1096(str: *const c_char) -> *const c_char {
    str
}
}

// ── `while (*s != '\0')` · fs_utils.c:95-99 · chunk 1097 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (*s != '\0') {
//             if (*s <= ' ' || *s > '~')
//                 goto use_quote;
//             s++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1097 {
// Scan a NUL-terminated string; report 1 if any byte needs quoting
// (byte <= ' ' or byte > '~', with C's signed char semantics), else 0.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn quoted_str__part1097(str_: *const c_char) -> c_int {
    let mut p = str_;
    unsafe {
        while *p != 0 {
            let c = *p;
            if c <= b' ' as c_char || c > b'~' as c_char {
                return 1;
            }
            p = p.add(1);
        }
    }
    0
}
}

// ── `return strdup(str);` · fs_utils.c:100-100 · chunk 1098 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return strdup(str);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1098 {
use core::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn quoted_str__part1098(s: *const c_char) -> *mut c_char {
    unsafe { strdup(s) }
}
}

// ── `use_quote: buf = malloc(strlen(str) * 4 + 2 + 1);` · fs_utils.c:101-102 · chunk 1099 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     use_quote:
//         buf = malloc(strlen(str) * 4 + 2 + 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1099 {
// Allocation part of quoted_str: buf = malloc(strlen(str) * 4 + 2 + 1);
use core::ffi::{c_char, c_ulong, c_void};

extern "C" {
    fn malloc(size: c_ulong) -> *mut c_void;
    fn strlen(s: *const c_char) -> c_ulong;
}

#[no_mangle]
pub extern "C" fn quoted_str__part1099(str_: *const c_char) -> *mut c_char {
    unsafe {
        let len: c_ulong = strlen(str_);
        let size: c_ulong = len
            .wrapping_mul(4)
            .wrapping_add(2)
            .wrapping_add(1);
        malloc(size) as *mut c_char
    }
}
}

// ── `q = buf; s = str; *q++ = '"';` · fs_utils.c:103-105 · chunk 1100 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     q = buf;
//         s = str;
//         *q++ = '"';
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1100 {
// Portion of quoted_str: initialize the output pointer and write the leading quote.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn quoted_str__part1100(
    buf: *mut c_char,
    str_: *const c_char,
    q_out: *mut *mut c_char,
    s_out: *mut *const c_char,
) {
    let _ = core::mem::size_of::<c_int>();
    unsafe {
        let mut q: *mut c_char = buf;
        let s: *const c_char = str_;
        // *q++ = '"';
        if !q.is_null() {
            *q = b'"' as c_char;
            q = q.add(1);
        }
        if !q_out.is_null() {
            *q_out = q;
        }
        if !s_out.is_null() {
            *s_out = s;
        }
    }
}
}

// ── `while (*s != '\0')` · fs_utils.c:106-116 · chunk 1101 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (*s != '\0') {
//             c = *(uint8_t *)s;
//             if (c < ' ' || c == 127) {
//                 q += sprintf(q, "\\x%02x", c);
//             } else if (c == '\\' || c == '\"') {
//                 q += sprintf(q, "\\%c", c);
//             } else {
//                 *q++ = c;
//             }
//             s++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1101 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn sprintf(__s: *mut c_char, __format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn quoted_str__part1101(ps: *mut *const c_char, pq: *mut *mut c_char) {
    unsafe {
        let mut s: *const c_char = *ps;
        let mut q: *mut c_char = *pq;
        let space: c_int = 32; // ' '
        let backslash: c_int = 92; // '\\'
        let dquote: c_int = 34; // '"'

        while *s != 0 {
            let c: c_int = (*s as u8) as c_int;
            if c < space || c == 127 {
                let n = sprintf(q, b"\\x%02x\0".as_ptr() as *const c_char, c);
                q = q.offset(n as isize);
            } else if c == backslash || c == dquote {
                let n = sprintf(q, b"\\%c\0".as_ptr() as *const c_char, c);
                q = q.offset(n as isize);
            } else {
                *q = (c as u8) as c_char;
                q = q.offset(1);
            }
            s = s.offset(1);
        }

        *ps = s;
        *pq = q;
    }
}
}

// ── `*q++ = '"'; *q = '\0'; return buf;` · fs_utils.c:117-119 · chunk 1102 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q++ = '"';
//         *q = '\0';
//         return buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1102 {
use core::ffi::{c_char, c_int};

// Tail of quoted_str: close the quote and NUL-terminate, then return buf.
#[no_mangle]
pub extern "C" fn quoted_str__part1102(q: *mut c_char, buf: *mut c_char) -> *mut c_char {
    unsafe {
        *q = b'"' as c_char;
        *q.add(1) = 0 as c_char;
    }
    buf
}

// Silence unused-import lint in case c_int is not needed on some targets.
const _: Option<c_int> = None;
}

// ── `parse_fname` · fs_utils.c:122-188 · chunk 1103 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_fname(char *buf, int buf_size, const char **pp)
//     {
//         const char *p;
//         char *q;
//         int c, h;
//     
//         p = *pp;
//         while (isspace_nolf(*p))
//             p++;
//         if (*p == '\0')
//             return -1;
//         q = buf;
//         if (*p == '"') {
//             p++;
//             for(;;) {
//                 c = *p++;
//                 if (c == '\0' || c == '\n') {
//                     return -1;
//                 } else if (c == '\"') {
//                     break;
//                 } else if (c == '\\') {
//                     c = *p++;
//                     switch(c) {
//                     case '\'':
//                     case '\"':
//                     case '\\':
//                         goto add_char;
//                     case 'n':
//                         c = '\n';
//                         goto add_char;
//                     case 'r':
//                         c = '\r';
//                         goto add_char;
//                     case 't':
//                         c = '\t';
//                         goto add_char;
//                     case 'x':
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return -1;
//                         c = h << 4;
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return -1;
//                         c |= h;
//                         goto add_char;
//                     default:
//                         return -1;
//                     }
//                 } else {
//                 add_char:
//                     if (q >= buf + buf_size - 1)
//                         return -1;
//                     *q++ = c;
//                 }
//             }
//         } else {
//             while (!isspace_nolf(*p) && *p != '\0' && *p != '\n') {
//                 if (q >= buf + buf_size - 1)
//                     return -1;
//                 *q++ = *p++;
//             }
//         }
//         *q = '\0';
//         *pp = p;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1103 {
// Port of parse_fname (fs_utils.c). The helpers isspace_nolf and from_hex are
// `static inline` in the C file, so their logic is inlined here as private fns.

use core::ffi::{c_char, c_int};

#[inline]
fn isspace_nolf(c: c_int) -> bool {
    c == i32::from(b' ') || c == i32::from(b'\t')
}

#[inline]
fn from_hex(c: c_int) -> c_int {
    if c >= i32::from(b'0') && c <= i32::from(b'9') {
        c - i32::from(b'0')
    } else if c >= i32::from(b'A') && c <= i32::from(b'F') {
        c - i32::from(b'A') + 10
    } else if c >= i32::from(b'a') && c <= i32::from(b'f') {
        c - i32::from(b'a') + 10
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn parse_fname(buf: *mut c_char, buf_size: c_int, pp: *mut *const c_char) -> c_int {
    unsafe {
        let mut p: *const c_char = *pp;
        while isspace_nolf(*p as c_int) {
            p = p.add(1);
        }
        if *p == 0 {
            return -1;
        }
        let mut q: *mut c_char = buf;
        let limit: *const c_char = buf.offset(buf_size as isize - 1);

        if *p == b'"' as c_char {
            p = p.add(1);
            loop {
                let mut c: c_int = *p as c_int;
                p = p.add(1);
                if c == 0 || c == i32::from(b'\n') {
                    return -1;
                } else if c == i32::from(b'"') {
                    break;
                } else if c == i32::from(b'\\') {
                    c = *p as c_int;
                    p = p.add(1);
                    if c == i32::from(b'\'') || c == i32::from(b'"') || c == i32::from(b'\\') {
                        // goto add_char
                    } else if c == i32::from(b'n') {
                        c = i32::from(b'\n');
                    } else if c == i32::from(b'r') {
                        c = i32::from(b'\r');
                    } else if c == i32::from(b't') {
                        c = i32::from(b'\t');
                    } else if c == i32::from(b'x') {
                        let mut h: c_int = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return -1;
                        }
                        c = h << 4;
                        h = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return -1;
                        }
                        c |= h;
                    } else {
                        return -1;
                    }
                }
                // add_char:
                if (q as *const c_char) >= limit {
                    return -1;
                }
                *q = c as c_char;
                q = q.add(1);
            }
        } else {
            while !isspace_nolf(*p as c_int) && *p != 0 && *p != b'\n' as c_char {
                if (q as *const c_char) >= limit {
                    return -1;
                }
                *q = *p;
                q = q.add(1);
                p = p.add(1);
            }
        }
        *q = 0;
        *pp = p;
        0
    }
}
}

// ── `const char *p; char *q; int c, h; p = *pp;` · fs_utils.c:124-128 · chunk 1104 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p;
//         char *q;
//         int c, h;
//     
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1104 {
// Ported piece of parse_fname: the local declarations plus `p = *pp;`
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_fname__part1104(pp: *const *const c_char) -> *const c_char {
    unsafe {
        if pp.is_null() {
            return core::ptr::null();
        }
        *pp
    }
}
}

// ── `while (isspace_nolf(*p))` · fs_utils.c:129-130 · chunk 1105 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (isspace_nolf(*p))
//             p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1105 {
// Port of the leading-whitespace skip loop from parse_fname.
use core::ffi::c_char;

// static inline in fs_utils.h: BOOL isspace_nolf(int c) { return (c == ' ' || c == '\t'); }
#[inline]
fn isspace_nolf(c: c_char) -> bool {
    c == b' ' as c_char || c == b'\t' as c_char
}

#[no_mangle]
pub extern "C" fn parse_fname__part1105(mut p: *const c_char) -> *const c_char {
    unsafe {
        loop {
            let stop = !isspace_nolf(*p);
            if stop {
                break;
            }
            p = p.add(1);
        }
    }
    p
}
}

// ── `if (*p == '\0')` · fs_utils.c:131-132 · chunk 1106 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p == '\0')
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1106 {
// Ported piece of parse_fname: check for end of string.
use core::ffi::{c_char, c_int};

// Returns 1 if the string at `p` is empty (first char is NUL), else 0.
#[no_mangle]
pub extern "C" fn parse_fname__part1106(p: *const c_char) -> c_int {
    unsafe {
        if *p == 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `q = buf;` · fs_utils.c:133-133 · chunk 1107 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     q = buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1107 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_fname__part1107(buf: *mut c_char) -> *mut c_char {
    buf
}
}

// ── `if (*p == '"') … else` · fs_utils.c:134-184 · chunk 1108 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p == '"') {
//             p++;
//             for(;;) {
//                 c = *p++;
//                 if (c == '\0' || c == '\n') {
//                     return -1;
//                 } else if (c == '\"') {
//                     break;
//                 } else if (c == '\\') {
//                     c = *p++;
//                     switch(c) {
//                     case '\'':
//                     case '\"':
//                     case '\\':
//                         goto add_char;
//                     case 'n':
//                         c = '\n';
//                         goto add_char;
//                     case 'r':
//                         c = '\r';
//                         goto add_char;
//                     case 't':
//                         c = '\t';
//                         goto add_char;
//                     case 'x':
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return -1;
//                         c = h << 4;
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return -1;
//                         c |= h;
//                         goto add_char;
//                     default:
//                         return -1;
//                     }
//                 } else {
//                 add_char:
//                     if (q >= buf + buf_size - 1)
//                         return -1;
//                     *q++ = c;
//                 }
//             }
//         } else {
//             while (!isspace_nolf(*p) && *p != '\0' && *p != '\n') {
//                 if (q >= buf + buf_size - 1)
//                     return -1;
//                 *q++ = *p++;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1108 {
// Port of the quoted/unquoted filename parsing block of parse_fname.
use core::ffi::{c_char, c_int};

#[inline]
fn isspace_nolf(c: c_int) -> bool {
    c == ' ' as c_int || c == '\t' as c_int
}

#[inline]
fn from_hex(c: c_int) -> c_int {
    if c >= '0' as c_int && c <= '9' as c_int {
        c - '0' as c_int
    } else if c >= 'A' as c_int && c <= 'F' as c_int {
        c - 'A' as c_int + 10
    } else if c >= 'a' as c_int && c <= 'f' as c_int {
        c - 'a' as c_int + 10
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_fname__part1108(
    buf: *mut c_char,
    buf_size: c_int,
    pp: *mut *const c_char,
    qq: *mut *mut c_char,
) -> c_int {
    let mut p: *const c_char = *pp;
    let mut q: *mut c_char = *qq;
    let limit: usize = (buf as usize).wrapping_add((buf_size as isize - 1) as usize);

    if *p as c_int == '"' as c_int {
        p = p.add(1);
        loop {
            let mut c: c_int = *p as c_int;
            p = p.add(1);
            if c == 0 || c == '\n' as c_int {
                return -1;
            } else if c == '"' as c_int {
                break;
            } else if c == '\\' as c_int {
                c = *p as c_int;
                p = p.add(1);
                match c {
                    // '\'' | '"' | '\\' : character used as-is
                    39 | 34 | 92 => {}
                    110 => c = '\n' as c_int, // 'n'
                    114 => c = '\r' as c_int, // 'r'
                    116 => c = '\t' as c_int, // 't'
                    120 => {
                        // 'x'
                        let mut h: c_int = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return -1;
                        }
                        c = h << 4;
                        h = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return -1;
                        }
                        c |= h;
                    }
                    _ => return -1,
                }
                // add_char:
                if q as usize >= limit {
                    return -1;
                }
                *q = c as c_char;
                q = q.add(1);
            } else {
                // add_char:
                if q as usize >= limit {
                    return -1;
                }
                *q = c as c_char;
                q = q.add(1);
            }
        }
    } else {
        while !isspace_nolf(*p as c_int) && *p != 0 && *p != '\n' as c_char {
            if q as usize >= limit {
                return -1;
            }
            *q = *p;
            q = q.add(1);
            p = p.add(1);
        }
    }

    *qq = q;
    *pp = p;
    0
}
}

// ── `*q = '\0'; *pp = p; return 0;` · fs_utils.c:185-187 · chunk 1109 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q = '\0';
//         *pp = p;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1109 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_fname__part1109(
    q: *mut c_char,
    pp: *mut *const c_char,
    p: *const c_char,
) -> c_int {
    unsafe {
        *q = 0;
        *pp = p;
    }
    0
}
}

// ── `parse_uint32_base` · fs_utils.c:190-201 · chunk 1110 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_uint32_base(uint32_t *pval, const char **pp, int base)
//     {
//         const char *p, *p1;
//         p = *pp;
//         while (isspace_nolf(*p))
//             p++;
//         *pval = strtoul(p, (char **)&p1, base);
//         if (p1 == p)
//             return -1;
//         *pp = p1;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1110 {
// Port of fs_utils.c: parse_uint32_base
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_ulong};
use core::ptr;

extern "C" {
    // glibc redirects strtoul to the C23 variant in this program.
    #[link_name = "__isoc23_strtoul"]
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
}

// static inline BOOL isspace_nolf(int c) { return (c == ' ' || c == '\t'); }
#[inline]
fn isspace_nolf(c: c_int) -> bool {
    c == b' ' as c_int || c == b'\t' as c_int
}

#[no_mangle]
pub unsafe extern "C" fn parse_uint32_base(
    pval: *mut u32,
    pp: *mut *const c_char,
    base: c_int,
) -> c_int {
    let mut p: *const c_char = *pp;
    while isspace_nolf(*p as c_int) {
        p = p.add(1);
    }
    let mut p1: *mut c_char = ptr::null_mut();
    let v = strtoul(p, &mut p1 as *mut *mut c_char, base);
    *pval = v as u32;
    if p1 as *const c_char == p {
        return -1;
    }
    *pp = p1 as *const c_char;
    0
}
}

// ── `const char *p, *p1; p = *pp;` · fs_utils.c:192-193 · chunk 1111 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p, *p1;
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1111 {
use core::ffi::c_char;

// Reads the current parse position out of the caller's cursor.
#[no_mangle]
pub extern "C" fn parse_uint32_base__part1111(pp: *const *const c_char) -> *const c_char {
    unsafe { *pp }
}
}

// ── `while (isspace_nolf(*p))` · fs_utils.c:194-195 · chunk 1112 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (isspace_nolf(*p))
//             p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1112 {
// Port of the whitespace-skipping loop of parse_uint32_base.
// isspace_nolf had no linker symbol (static inline), so its logic
// (c == ' ' || c == '\t') is reproduced here.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_uint32_base__part1112(mut p: *const c_char) -> *const c_char {
    unsafe {
        while *p == b' ' as c_char || *p == b'\t' as c_char {
            p = p.add(1);
        }
    }
    p
}
}

// ── `*pval = strtoul(p, (char **)&p1, base);` · fs_utils.c:196-196 · chunk 1113 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pval = strtoul(p, (char **)&p1, base);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1113 {
use core::ffi::{c_char, c_int, c_uint, c_ulong};

extern "C" {
    // The translation unit's <stdlib.h> redirects strtoul to __isoc23_strtoul.
    #[link_name = "__isoc23_strtoul"]
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
}

#[no_mangle]
pub extern "C" fn parse_uint32_base__part1113(
    pval: *mut c_uint,
    p1_out: *mut *const c_char,
    p: *const c_char,
    base: c_int,
) {
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let v: c_ulong = unsafe { strtoul(p, &mut endptr, base) };
    unsafe {
        *pval = v as c_uint;
        *p1_out = endptr as *const c_char;
    }
}
}

// ── `if (p1 == p)` · fs_utils.c:197-198 · chunk 1114 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p1 == p)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1114 {
// Port of the `if (p1 == p) return -1;` piece of parse_uint32_base.
// Returns 1 when the two pointers are equal (caller then returns -1), else 0.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_uint32_base__part1114(p1: *const c_char, p: *const c_char) -> core::ffi::c_int {
    if p1 == p { 1 } else { 0 }
}
}

// ── `*pp = p1; return 0;` · fs_utils.c:199-200 · chunk 1115 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p1;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1115 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_uint32_base__part1115(pp: *mut *const c_char, p1: *const c_char) -> c_int {
    unsafe {
        *pp = p1;
    }
    0
}
}

// ── `parse_uint64_base` · fs_utils.c:203-214 · chunk 1116 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_uint64_base(uint64_t *pval, const char **pp, int base)
//     {
//         const char *p, *p1;
//         p = *pp;
//         while (isspace_nolf(*p))
//             p++;
//         *pval = strtoull(p, (char **)&p1, base);
//         if (p1 == p)
//             return -1;
//         *pp = p1;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1116 {
use core::ffi::{c_char, c_int};

extern "C" {
    // glibc's C23 strtoull (the C translation unit refers to it via asm("__isoc23_strtoull")).
    #[link_name = "__isoc23_strtoull"]
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
}

// Port of the static inline helper isspace_nolf() from fs_utils.c.
#[inline]
fn isspace_nolf(c: c_int) -> bool {
    c == b' ' as c_int || c == b'\t' as c_int
}

#[no_mangle]
pub extern "C" fn parse_uint64_base(
    pval: *mut u64,
    pp: *mut *const c_char,
    base: c_int,
) -> c_int {
    unsafe {
        let mut p: *const c_char = *pp;
        while isspace_nolf(*p as c_int) {
            p = p.add(1);
        }
        let mut p1: *mut c_char = core::ptr::null_mut();
        let v = strtoull(p, &mut p1 as *mut *mut c_char, base);
        *pval = v;
        if p1 as *const c_char == p {
            return -1;
        }
        *pp = p1 as *const c_char;
        0
    }
}
}

// ── `const char *p, *p1; p = *pp;` · fs_utils.c:205-206 · chunk 1117 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p, *p1;
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1117 {
use core::ffi::c_char;

// Port of the TARGET in parse_uint64_base:
//     const char *p, *p1;
//     p = *pp;
// Returns the pointer stored at *pp (which the C side assigns to `p`).
#[no_mangle]
pub extern "C" fn parse_uint64_base__part1117(pp: *const *const c_char) -> *const c_char {
    unsafe { *pp }
}
}

// ── `while (isspace_nolf(*p))` · fs_utils.c:207-208 · chunk 1118 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (isspace_nolf(*p))
//             p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1118 {
use core::ffi::c_char;

// Port of the file-static `isspace_nolf` helper (no linker symbol in C).
#[inline]
fn isspace_nolf(c: i32) -> bool {
    c == 0x20 || c == 0x09
}

#[no_mangle]
pub extern "C" fn parse_uint64_base__part1118(mut p: *const c_char) -> *const c_char {
    unsafe {
        while isspace_nolf(*p as i32) {
            p = p.add(1);
        }
    }
    p
}
}

// ── `*pval = strtoull(p, (char **)&p1, base);` · fs_utils.c:209-209 · chunk 1119 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pval = strtoull(p, (char **)&p1, base);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1119 {
use core::ffi::{c_char, c_int, c_ulong, c_ulonglong};

extern "C" {
    // In this translation unit `strtoull` is redirected by an asm label to
    // __isoc23_strtoull, so reference that exact symbol.
    #[link_name = "__isoc23_strtoull"]
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
}

#[no_mangle]
pub extern "C" fn parse_uint64_base__part1119(
    pval: *mut c_ulong,
    p: *const c_char,
    p1: *mut *const c_char,
    base: c_int,
) {
    let mut end: *mut c_char = core::ptr::null_mut();
    let v = unsafe { strtoull(p, &mut end, base) };
    unsafe {
        *pval = v as c_ulong;
        *p1 = end as *const c_char;
    }
}
}

// ── `if (p1 == p)` · fs_utils.c:210-211 · chunk 1120 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p1 == p)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1120 {
// Port of the `if (p1 == p) return -1;` check inside parse_uint64_base.
// Returns 1 when the two `const char *` values are the same pointer
// (i.e. strtoull consumed no input), 0 otherwise.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_uint64_base__part1120(p1: *const c_char, p: *const c_char) -> c_int {
    if p1 == p {
        1
    } else {
        0
    }
}
}

// ── `*pp = p1; return 0;` · fs_utils.c:212-213 · chunk 1121 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p1;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1121 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_uint64_base__part1121(pp: *mut *const c_char, p1: *const c_char) -> c_int {
    unsafe {
        *pp = p1;
    }
    0
}
}

// ── `parse_uint64` · fs_utils.c:216-219 · chunk 1122 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_uint64(uint64_t *pval, const char **pp)
//     {
//         return parse_uint64_base(pval, pp, 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1122 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_uint64(pval: *mut u64, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint64_base(pval, pp, 0) }
}
}

// ── `return parse_uint64_base(pval, pp, 0);` · fs_utils.c:218-218 · chunk 1123 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return parse_uint64_base(pval, pp, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1123 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_uint64__part1123(pval: *mut u64, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint64_base(pval, pp, 0) }
}
}

// ── `parse_uint32` · fs_utils.c:221-224 · chunk 1124 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_uint32(uint32_t *pval, const char **pp)
//     {
//         return parse_uint32_base(pval, pp, 0);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1124 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint32_base(pval: *mut u32, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_uint32(pval: *mut u32, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint32_base(pval, pp, 0) }
}
}

// ── `return parse_uint32_base(pval, pp, 0);` · fs_utils.c:223-223 · chunk 1125 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return parse_uint32_base(pval, pp, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1125 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint32_base(pval: *mut u32, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_uint32__part1125(pval: *mut u32, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint32_base(pval, pp, 0) }
}
}

// ── `parse_time` · fs_utils.c:226-248 · chunk 1126 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_time(uint32_t *psec, uint32_t *pnsec, const char **pp)
//     {
//         const char *p;
//         uint32_t v, m;
//         p = *pp;
//         if (parse_uint32(psec, &p) < 0)
//             return -1;
//         v = 0;
//         if (*p == '.') {
//             p++;
//     
//             m = 1000000000;
//             v = 0;
//             while (*p >= '0' && *p <= '9') {
//                 m /= 10;
//                 v += (*p - '0') * m;
//                 p++;
//             }
//         }
//         *pnsec = v;
//         *pp = p;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1126 {
use core::ffi::{c_char, c_int};

extern "C" {
    // Provided by the C program (parse_uint32 -> parse_uint32_base).
    fn parse_uint32(pval: *mut u32, pp: *mut *const c_char) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn parse_time(
    psec: *mut u32,
    pnsec: *mut u32,
    pp: *mut *const c_char,
) -> c_int {
    let mut p: *const c_char = *pp;

    if parse_uint32(psec, &mut p) < 0 {
        return -1;
    }

    let mut v: u32 = 0;
    if *p == b'.' as c_char {
        p = p.add(1);

        let mut m: u32 = 1_000_000_000;
        v = 0;
        while *p >= b'0' as c_char && *p <= b'9' as c_char {
            m /= 10;
            // (*p - '0') is an int in C, then converted to unsigned for the
            // multiplication with the uint32_t m; keep wrapping semantics.
            let d: c_int = *p as c_int - b'0' as c_int;
            v = v.wrapping_add((d as u32).wrapping_mul(m));
            p = p.add(1);
        }
    }

    *pnsec = v;
    *pp = p;
    0
}
}

// ── `const char *p; uint32_t v, m; p = *pp;` · fs_utils.c:228-230 · chunk 1127 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p;
//         uint32_t v, m;
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1127 {
// Ported piece of parse_time: initialises the local `p` from *pp.
// The other locals (v, m) are left uninitialised exactly as in the C original;
// the surrounding C code assigns them before any use.

use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_time__part1127(
    pp: *mut *const c_char,
    p_out: *mut *const c_char,
) {
    unsafe {
        let p: *const c_char = *pp;
        *p_out = p;
    }
}
}

// ── `if (parse_uint32(psec, &p) < 0)` · fs_utils.c:231-232 · chunk 1128 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (parse_uint32(psec, &p) < 0)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1128 {
// Port of the parse_uint32() probe inside parse_time().
use core::ffi::{c_char, c_int};

extern "C" {
    // Defined elsewhere in the same translation unit (fs_utils.c).
    fn parse_uint32(pval: *mut u32, pp: *mut *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_time__part1128(psec: *mut u32, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint32(psec, pp) }
}
}

// ── `v = 0;` · fs_utils.c:233-233 · chunk 1129 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     v = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1129 {
// Ported piece of parse_time: `v = 0;`
// The C local `v` is uint32_t; the value is returned so the C side keeps the
// declaration `uint32_t v` and its assignment.

#[no_mangle]
pub extern "C" fn parse_time__part1129() -> core::ffi::c_uint {
    0
}
}

// ── `if (*p == '.')` · fs_utils.c:234-244 · chunk 1130 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p == '.') {
//             p++;
//     
//             m = 1000000000;
//             v = 0;
//             while (*p >= '0' && *p <= '9') {
//                 m /= 10;
//                 v += (*p - '0') * m;
//                 p++;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1130 {
use core::ffi::c_char;

// Ports the fractional-seconds parsing part of parse_time.
// *pp holds the cursor `p`; returns the new value of `v`.
#[no_mangle]
pub extern "C" fn parse_time__part1130(pp: *mut *const c_char, v_in: u32) -> u32 {
    let mut v: u32 = v_in;
    unsafe {
        let mut p: *const c_char = *pp;
        if *p == b'.' as c_char {
            p = p.offset(1);
            let mut m: u32 = 1000000000u32;
            v = 0;
            loop {
                let c: c_char = *p;
                if !(c >= b'0' as c_char && c <= b'9' as c_char) {
                    break;
                }
                m /= 10;
                let digit: u32 = (c as i32 - b'0' as i32) as u32;
                v = v.wrapping_add(digit.wrapping_mul(m));
                p = p.offset(1);
            }
        }
        *pp = p;
    }
    v
}
}

// ── `*pnsec = v; *pp = p; return 0;` · fs_utils.c:245-247 · chunk 1131 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pnsec = v;
//         *pp = p;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1131 {
// Tail of parse_time: store the fractional seconds and the advanced pointer.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_time__part1131(
    pnsec: *mut u32,
    v: u32,
    pp: *mut *const c_char,
    p: *const c_char,
) -> c_int {
    unsafe {
        if !pnsec.is_null() {
            *pnsec = v;
        }
        if !pp.is_null() {
            *pp = p;
        }
    }
    0
}
}

// ── `parse_file_id` · fs_utils.c:250-253 · chunk 1132 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_file_id(FSFileID *pval, const char **pp)
//     {
//         return parse_uint64_base(pval, pp, 16);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1132 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_file_id(pval: *mut u64, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint64_base(pval, pp, 16) }
}
}

// ── `return parse_uint64_base(pval, pp, 16);` · fs_utils.c:252-252 · chunk 1133 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return parse_uint64_base(pval, pp, 16);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1133 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_file_id__part1133(pval: *mut u64, pp: *mut *const c_char) -> c_int {
    unsafe { parse_uint64_base(pval, pp, 16) }
}
}

// ── `file_id_to_filename` · fs_utils.c:255-259 · chunk 1134 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *file_id_to_filename(char *buf, FSFileID file_id)
//     {
//         sprintf(buf, "%016" 
//                            "l" "x"
//                                  , file_id);
//         return buf;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1134 {
// Port of: char *file_id_to_filename(char *buf, FSFileID file_id)
// FSFileID is typedef uint64_t.
// Equivalent to sprintf(buf, "%016lx", file_id).

use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn file_id_to_filename(buf: *mut c_char, file_id: u64) -> *mut c_char {
    if !buf.is_null() {
        const HEX: [u8; 16] = *b"0123456789abcdef";
        let mut v = file_id;
        // Write 16 hex digits, most significant first, at buf[0..16].
        let mut i: usize = 16;
        unsafe {
            while i > 0 {
                i -= 1;
                let digit = (v & 0xf) as usize;
                // HEX index is always in range (digit <= 15).
                let c = *HEX.get_unchecked(digit);
                core::ptr::write(buf.add(i) as *mut u8, c);
                v >>= 4;
            }
            // NUL terminate
            core::ptr::write(buf.add(16) as *mut u8, 0u8);
        }
    }
    buf
}
}

// ── `sprintf(buf, "%016" "l" "x" , file_id); return buf;` · fs_utils.c:257-258 · chunk 1135 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sprintf(buf, "%016" 
//                            "l" "x"
//                                  , file_id);
//         return buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1135 {
// Port of file_id_to_filename's sprintf(body) + return.
// C: sprintf(buf, "%016lx", file_id); return buf;
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn file_id_to_filename__part1135(buf: *mut c_char, file_id: u64) -> *mut c_char {
    unsafe {
        let mut i: usize = 0;
        let mut shift: u32 = 60;
        loop {
            let nib: u32 = ((file_id >> shift) & 0xf) as u32;
            let ch: u8 = if nib < 10 {
                b'0' + (nib as u8)
            } else {
                b'a' + ((nib - 10) as u8)
            };
            *buf.add(i) = ch as c_char;
            i += 1;
            if shift == 0 {
                break;
            }
            shift -= 4;
        }
        // sprintf always writes the terminating NUL
        *buf.add(i) = 0;
    }
    buf
}
}

// ── `encode_hex` · fs_utils.c:261-266 · chunk 1136 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void encode_hex(char *str, const uint8_t *buf, int len)
//     {
//         int i;
//         for(i = 0; i < len; i++)
//             sprintf(str + 2 * i, "%02x", buf[i]);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1136 {
// Port of void encode_hex(char *str, const uint8_t *buf, int len)
// Mirrors the original: for each byte, sprintf(str + 2 * i, "%02x", buf[i]),
// i.e. two lowercase hex digits plus the NUL terminator written at 2 * i + 2.

use core::ffi::{c_char, c_int};

static HEX: [u8; 16] = *b"0123456789abcdef";

#[no_mangle]
pub extern "C" fn encode_hex(str: *mut c_char, buf: *const u8, len: c_int) {
    let mut i: c_int = 0;
    unsafe {
        while i < len {
            let b = *buf.offset(i as isize);
            let hi = *HEX.get_unchecked((b >> 4) as usize);
            let lo = *HEX.get_unchecked((b & 0x0f) as usize);
            let p = str.offset(i.wrapping_mul(2) as isize);
            *p = hi as c_char;
            *p.offset(1) = lo as c_char;
            *p.offset(2) = 0;
            i += 1;
        }
    }
}
}

// ── `for (i = 0; i < len; i++)` · fs_utils.c:264-265 · chunk 1137 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < len; i++)
//             sprintf(str + 2 * i, "%02x", buf[i]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1137 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn encode_hex__part1137(out: *mut c_char, buf: *const u8, len: c_int) {
    let mut i: c_int = 0;
    while i < len {
        unsafe {
            sprintf(
                out.offset((2 * i) as isize),
                b"%02x\0".as_ptr() as *const c_char,
                *buf.offset(i as isize) as c_int,
            );
        }
        i += 1;
    }
}
}

// ── `decode_hex` · fs_utils.c:268-282 · chunk 1138 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int decode_hex(uint8_t *buf, const char *str, int len)
//     {
//         int h0, h1, i;
//     
//         for(i = 0; i < len; i++) {
//             h0 = from_hex(str[2 * i]);
//             if (h0 < 0)
//                 return -1;
//             h1 = from_hex(str[2 * i + 1]);
//             if (h1 < 0)
//                 return -1;
//             buf[i] = (h0 << 4) | h1;
//         }
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1138 {
use core::ffi::{c_char, c_int};

#[inline]
fn from_hex(c: c_int) -> c_int {
    if c >= b'0' as c_int && c <= b'9' as c_int {
        c - b'0' as c_int
    } else if c >= b'A' as c_int && c <= b'F' as c_int {
        c - b'A' as c_int + 10
    } else if c >= b'a' as c_int && c <= b'f' as c_int {
        c - b'a' as c_int + 10
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn decode_hex(buf: *mut u8, str_: *const c_char, len: c_int) -> c_int {
    let mut i: c_int = 0;
    while i < len {
        let h0 = from_hex(*str_.offset((2 * i) as isize) as c_int);
        if h0 < 0 {
            return -1;
        }
        let h1 = from_hex(*str_.offset((2 * i + 1) as isize) as c_int);
        if h1 < 0 {
            return -1;
        }
        *buf.offset(i as isize) = ((h0 << 4) | h1) as u8;
        i += 1;
    }
    0
}
}

// ── `for (i = 0; i < len; i++)` · fs_utils.c:272-280 · chunk 1139 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < len; i++) {
//             h0 = from_hex(str[2 * i]);
//             if (h0 < 0)
//                 return -1;
//             h1 = from_hex(str[2 * i + 1]);
//             if (h1 < 0)
//                 return -1;
//             buf[i] = (h0 << 4) | h1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1139 {
use core::ffi::{c_char, c_int};

// static inline in C: no linker symbol, so the logic is ported here.
#[inline]
fn from_hex(c: c_int) -> c_int {
    if c >= b'0' as c_int && c <= b'9' as c_int {
        c - b'0' as c_int
    } else if c >= b'A' as c_int && c <= b'F' as c_int {
        c - b'A' as c_int + 10
    } else if c >= b'a' as c_int && c <= b'f' as c_int {
        c - b'a' as c_int + 10
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn decode_hex__part1139(buf: *mut u8, str_: *const c_char, len: c_int) -> c_int {
    let mut i: c_int = 0;
    while i < len {
        let h0 = from_hex(unsafe { *str_.offset((2 * i) as isize) } as c_int);
        if h0 < 0 {
            return -1;
        }
        let h1 = from_hex(unsafe { *str_.offset((2 * i + 1) as isize) } as c_int);
        if h1 < 0 {
            return -1;
        }
        unsafe {
            *buf.offset(i as isize) = (((h0 << 4) | h1) as u8);
        }
        i += 1;
    }
    0
}
}

// ── `return 0;` · fs_utils.c:281-281 · chunk 1140 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1140 {
// Tail of decode_hex: success return value.
#[no_mangle]
pub extern "C" fn decode_hex__part1140() -> core::ffi::c_int {
    0
}
}

// ── `skip_header` · fs_utils.c:285-291 · chunk 1141 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *skip_header(const char *p)
//     {
//         p = 
//            _Generic (0 ? (
//            p
//            ) : (void *) 1, const void *: (const char *) (strstr (
//            p
//            , 
//            "\n\n"
//            )), default: strstr (
//            p
//            , 
//            "\n\n"
//            ))
//                             ;
//         if (!p)
//             return 
//                   ((void *)0)
//                       ;
//         return p + 2;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1141 {
use core::ffi::c_char;

extern "C" {
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

static NEEDLE: [c_char; 3] = [b'\n' as c_char, b'\n' as c_char, 0];

#[no_mangle]
pub unsafe extern "C" fn skip_header(p: *const c_char) -> *const c_char {
    let found = strstr(p, NEEDLE.as_ptr());
    if found.is_null() {
        return core::ptr::null();
    }
    (found as *const c_char).add(2)
}
}

// ── `p = _Generic (0 ? ( p ) : (void *) 1, const void *: (const char *) (strstr ( p , "\n\n" )), def…` · fs_utils.c:287-287 · chunk 1142 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p = 
//            _Generic (0 ? (
//            p
//            ) : (void *) 1, const void *: (const char *) (strstr (
//            p
//            , 
//            "\n\n"
//            )), default: strstr (
//            p
//            , 
//            "\n\n"
//            ))
//                             ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1142 {
use core::ffi::{c_char, c_int};

extern "C" {
    // char *strstr(const char *haystack, const char *needle)
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

static NEEDLE: [u8; 3] = [b'\n' as u8, b'\n' as u8, 0u8];

#[no_mangle]
pub extern "C" fn skip_header__part1142(p: *const c_char) -> *const c_char {
    let _ = core::mem::size_of::<c_int>();
    unsafe { strstr(p, NEEDLE.as_ptr() as *const c_char) as *const c_char }
}
}

// ── `if (!p)` · fs_utils.c:288-289 · chunk 1143 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!p)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1143 {
// Ported piece of skip_header: test whether the header end pointer is NULL.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn skip_header__part1143(p: *const c_char) -> c_int {
    if p.is_null() {
        1
    } else {
        0
    }
}
}

// ── `return p + 2;` · fs_utils.c:290-290 · chunk 1144 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return p + 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1144 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn skip_header__part1144(p: *const c_char) -> *const c_char {
    // p is guaranteed non-NULL here (checked by the caller); C's p + 2.
    unsafe { p.add(2) }
}
}

// ── `parse_tag` · fs_utils.c:294-335 · chunk 1145 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_tag(char *buf, int buf_size, const char *str, const char *tag)
//     {
//         char tagname[128], *q;
//         const char *p, *p1;
//         int len;
//     
//         p = str;
//         for(;;) {
//             if (*p == '\0' || *p == '\n')
//                 break;
//             q = tagname;
//             while (*p != ':' && *p != '\n' && *p != '\0') {
//                 if ((q - tagname) < sizeof(tagname) - 1)
//                     *q++ = *p;
//                 p++;
//             }
//             *q = '\0';
//             if (*p != ':')
//                 return -1;
//             p++;
//             while (isspace_nolf(*p))
//                 p++;
//             p1 = p;
//             p = 
//                _Generic (0 ? (
//                p
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                p
//                , 
//                '\n'
//                )), default: strchr (
//                p
//                , 
//                '\n'
//                ))
//                               ;
//             if (!p)
//                 len = strlen(p1);
//             else
//                 len = p - p1;
//             if (!strcmp(tagname, tag)) {
//                 if (len > buf_size - 1)
//                     len = buf_size - 1;
//                 memcpy(buf, p1, len);
//                 buf[len] = '\0';
//                 return 0;
//             }
//             if (!p)
//                 break;
//             else
//                 p++;
//         }
//         return -1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1145 {
// Rust rewrite of parse_tag from fs_utils.c
use core::ffi::{c_char, c_int};

#[inline]
fn isspace_nolf(c: c_int) -> bool {
    c == b' ' as c_int || c == b'\t' as c_int
}

#[inline]
unsafe fn load(p: *const c_char) -> c_int {
    *p as c_int
}

// equivalent of strchr(p, '\n'), returning the pointer or null
unsafe fn strchr_nl(mut p: *const c_char) -> *const c_char {
    loop {
        let c = *p as u8;
        if c == b'\n' {
            return p;
        }
        if c == 0 {
            return core::ptr::null();
        }
        p = p.offset(1);
    }
}

unsafe fn strlen_rs(p: *const c_char) -> usize {
    let mut n: usize = 0;
    while *p.offset(n as isize) as u8 != 0 {
        n += 1;
    }
    n
}

// strcmp(tagname, tag) == 0 ?
unsafe fn str_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: isize = 0;
    loop {
        let ca = *a.offset(i) as u8;
        let cb = *b.offset(i) as u8;
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn parse_tag(
    buf: *mut c_char,
    buf_size: c_int,
    str_: *const c_char,
    tag: *const c_char,
) -> c_int {
    let mut tagname = [0u8; 128];
    let mut p: *const c_char = str_;

    loop {
        unsafe {
            let c0 = load(p);
            if c0 == 0 || c0 == b'\n' as c_int {
                break;
            }
            let mut qi: usize = 0;
            loop {
                let c = load(p);
                if c == b':' as c_int || c == b'\n' as c_int || c == 0 {
                    break;
                }
                if qi < 127 {
                    tagname[qi] = c as u8;
                    qi += 1;
                }
                p = p.offset(1);
            }
            tagname[qi] = 0;
            if load(p) != b':' as c_int {
                return -1;
            }
            p = p.offset(1);
            while isspace_nolf(load(p)) {
                p = p.offset(1);
            }
            let p1 = p;
            let nl = strchr_nl(p);
            let mut len: c_int = if nl.is_null() {
                strlen_rs(p1) as c_int
            } else {
                nl.offset_from(p1) as c_int
            };
            if str_eq(tagname.as_ptr() as *const c_char, tag) {
                let limit = buf_size.wrapping_sub(1);
                if len > limit {
                    len = limit;
                }
                if len > 0 {
                    core::ptr::copy_nonoverlapping(
                        p1 as *const u8,
                        buf as *mut u8,
                        len as usize,
                    );
                }
                *buf.offset(len as isize) = 0;
                return 0;
            }
            if nl.is_null() {
                break;
            } else {
                p = nl.offset(1);
            }
        }
    }
    -1
}
}

// ── `char tagname[128], *q; const char *p, *p1; int len; p = str;` · fs_utils.c:296-300 · chunk 1146 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char tagname[128], *q;
//         const char *p, *p1;
//         int len;
//     
//         p = str;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1146 {
// Portion of parse_tag: the local-declaration block plus `p = str;`.
// The C side keeps the actual declarations (so `sizeof(tagname)` and the
// loop body keep working) and this function performs the initialisation.
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_tag__part1146(
    _tagname: *mut c_char,
    _q: *mut *mut c_char,
    p: *mut *const c_char,
    _p1: *mut *const c_char,
    _len: *mut c_int,
    str_ptr: *const c_char,
) {
    // p = str;
    unsafe {
        *p = str_ptr;
    }
}
}

// ── `for (; ; )` · fs_utils.c:301-333 · chunk 1147 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             if (*p == '\0' || *p == '\n')
//                 break;
//             q = tagname;
//             while (*p != ':' && *p != '\n' && *p != '\0') {
//                 if ((q - tagname) < sizeof(tagname) - 1)
//                     *q++ = *p;
//                 p++;
//             }
//             *q = '\0';
//             if (*p != ':')
//                 return -1;
//             p++;
//             while (isspace_nolf(*p))
//                 p++;
//             p1 = p;
//             p = 
//                _Generic (0 ? (
//                p
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                p
//                , 
//                '\n'
//                )), default: strchr (
//                p
//                , 
//                '\n'
//                ))
//                               ;
//             if (!p)
//                 len = strlen(p1);
//             else
//                 len = p - p1;
//             if (!strcmp(tagname, tag)) {
//                 if (len > buf_size - 1)
//                     len = buf_size - 1;
//                 memcpy(buf, p1, len);
//                 buf[len] = '\0';
//                 return 0;
//             }
//             if (!p)
//                 break;
//             else
//                 p++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1147 {
// Port of the tag-scanning loop of parse_tag (fs_utils.c lines 301-333).
// Returns 0 when `tag` is found (and copied into buf), -1 otherwise.
use core::ffi::{c_char, c_int};

// strcmp(a, b) == 0
fn cstr_eq(a: *const c_char, b: *const c_char) -> bool {
    unsafe {
        let mut i: isize = 0;
        loop {
            let ca = *a.offset(i) as u8;
            let cb = *b.offset(i) as u8;
            if ca != cb {
                return false;
            }
            if ca == 0 {
                return true;
            }
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn parse_tag__part1147(
    buf: *mut c_char,
    buf_size: c_int,
    str_: *const c_char,
    tag: *const c_char,
) -> c_int {
    unsafe {
        let mut tagname = [0 as c_char; 128];
        let tname = tagname.as_mut_ptr();
        let mut p: *const c_char = str_;

        loop {
            let c0 = *p as u8;
            // if (*p == '\0' || *p == '\n') break;
            if c0 == 0 || c0 == b'\n' {
                break;
            }

            // q = tagname;
            let mut qi: usize = 0;
            // while (*p != ':' && *p != '\n' && *p != '\0')
            loop {
                let c = *p as u8;
                if c == b':' || c == b'\n' || c == 0 {
                    break;
                }
                if qi < 128 - 1 {
                    *tname.add(qi) = *p;
                    qi += 1;
                }
                p = p.add(1);
            }
            *tname.add(qi) = 0;

            if (*p as u8) != b':' {
                return -1;
            }
            p = p.add(1);

            // while (isspace_nolf(*p)) p++;
            loop {
                let c = *p as c_int;
                if !(c == b' ' as c_int || c == b'\t' as c_int) {
                    break;
                }
                p = p.add(1);
            }

            let p1 = p;

            // p = strchr(p, '\n');
            let mut r = p;
            loop {
                if *r == 0 {
                    break;
                }
                if (*r as u8) == b'\n' {
                    break;
                }
                r = r.add(1);
            }
            p = if *r == 0 { core::ptr::null() } else { r };

            // if (!p) len = strlen(p1); else len = p - p1;
            let mut len: c_int = if p.is_null() {
                let mut n: c_int = 0;
                let mut s = p1;
                while *s != 0 {
                    n += 1;
                    s = s.add(1);
                }
                n
            } else {
                p.offset_from(p1) as c_int
            };

            // if (!strcmp(tagname, tag)) { ... return 0; }
            if cstr_eq(tname as *const c_char, tag) {
                if len > buf_size - 1 {
                    len = buf_size - 1;
                }
                let mut i: isize = 0;
                while i < len as isize {
                    *buf.offset(i) = *p1.offset(i);
                    i += 1;
                }
                *buf.offset(len as isize) = 0;
                return 0;
            }

            if p.is_null() {
                break;
            } else {
                p = p.add(1);
            }
        }

        -1
    }
}
}

// ── `return -1;` · fs_utils.c:334-334 · chunk 1148 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1148 {
// Ported piece of parse_tag: the trailing "not found" return value.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn parse_tag__part1148() -> c_int {
    -1
}
}

// ── `parse_tag_uint64` · fs_utils.c:337-345 · chunk 1149 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_tag_uint64(uint64_t *pval, const char *str, const char *tag)
//     {
//         char buf[64];
//         const char *p;
//         if (parse_tag(buf, sizeof(buf), str, tag))
//             return -1;
//         p = buf;
//         return parse_uint64(pval, &p);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1149 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_tag(buf: *mut c_char, buf_size: c_int, str: *const c_char, tag: *const c_char) -> c_int;
    fn parse_uint64(pval: *mut u64, pp: *mut *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_uint64(
    pval: *mut u64,
    str: *const c_char,
    tag: *const c_char,
) -> c_int {
    let mut buf = [0 as c_char; 64];
    let mut p: *const c_char;

    unsafe {
        if parse_tag(buf.as_mut_ptr(), 64 as c_int, str, tag) != 0 {
            return -1;
        }
        p = buf.as_ptr();
        parse_uint64(pval, &mut p as *mut *const c_char)
    }
}
}

// ── `if (parse_tag(buf, sizeof(buf), str, tag))` · fs_utils.c:341-342 · chunk 1150 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (parse_tag(buf, sizeof(buf), str, tag))
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1150 {
// Port of the parse_tag() guard inside parse_tag_uint64().
// Returns 0 when the tag was parsed successfully, -1 otherwise (like the
// original `if (parse_tag(...)) return -1;`).

use core::ffi::{c_char, c_int};

extern "C" {
    // int parse_tag(char *buf, int buf_size, const char *str, const char *tag);
    fn parse_tag(buf: *mut c_char, buf_size: c_int, str_: *const c_char, tag: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_uint64__part1150(
    buf: *mut c_char,
    buf_size: c_int,
    str_: *const c_char,
    tag: *const c_char,
) -> c_int {
    unsafe {
        if parse_tag(buf, buf_size, str_, tag) != 0 {
            -1
        } else {
            0
        }
    }
}
}

// ── `p = buf; return parse_uint64(pval, &p);` · fs_utils.c:343-344 · chunk 1151 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p = buf;
//         return parse_uint64(pval, &p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1151 {
// Portion of parse_tag_uint64: parse buf with parse_uint64.
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64(pval: *mut u64, pp: *mut *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_uint64__part1151(pval: *mut u64, buf: *const c_char) -> c_int {
    let mut p: *const c_char = buf;
    unsafe { parse_uint64(pval, &mut p) }
}
}

// ── `parse_tag_file_id` · fs_utils.c:347-355 · chunk 1152 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_tag_file_id(FSFileID *pval, const char *str, const char *tag)
//     {
//         char buf[64];
//         const char *p;
//         if (parse_tag(buf, sizeof(buf), str, tag))
//             return -1;
//         p = buf;
//         return parse_uint64_base(pval, &p, 16);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1152 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_tag(buf: *mut c_char, buf_size: c_int, str: *const c_char, tag: *const c_char) -> c_int;
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_file_id(pval: *mut u64, str: *const c_char, tag: *const c_char) -> c_int {
    let mut buf = [0 as c_char; 64];
    let mut p: *const c_char;
    unsafe {
        if parse_tag(buf.as_mut_ptr(), 64, str, tag) != 0 {
            return -1;
        }
        p = buf.as_ptr();
        parse_uint64_base(pval, &mut p as *mut *const c_char, 16)
    }
}
}

// ── `if (parse_tag(buf, sizeof(buf), str, tag))` · fs_utils.c:351-352 · chunk 1153 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (parse_tag(buf, sizeof(buf), str, tag))
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1153 {
// Port of the parse_tag check inside parse_tag_file_id.
// Calls the C function parse_tag (external linkage) and returns its status.

use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_tag(buf: *mut c_char, buf_size: c_int, s: *const c_char, tag: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_file_id__part1153(
    buf: *mut c_char,
    buf_size: c_int,
    s: *const c_char,
    tag: *const c_char,
) -> c_int {
    unsafe { parse_tag(buf, buf_size, s, tag) }
}
}

// ── `p = buf; return parse_uint64_base(pval, &p, 16);` · fs_utils.c:353-354 · chunk 1154 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p = buf;
//         return parse_uint64_base(pval, &p, 16);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1154 {
// Port of parse_tag_file_id lines 353-354: parse the tag buffer as a hex file id.
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_uint64_base(pval: *mut u64, pp: *mut *const c_char, base: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_file_id__part1154(pval: *mut u64, buf: *const c_char) -> c_int {
    let mut p: *const c_char = buf;
    unsafe { parse_uint64_base(pval, &mut p as *mut *const c_char, 16) }
}
}

// ── `parse_tag_version` · fs_utils.c:357-363 · chunk 1155 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int parse_tag_version(const char *str)
//     {
//         uint64_t version;
//         if (parse_tag_uint64(&version, str, "Version"))
//             return -1;
//         return version;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1155 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_tag_uint64(pval: *mut u64, str: *const c_char, tag: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_version(str: *const c_char) -> c_int {
    let mut version: u64 = 0;
    unsafe {
        if parse_tag_uint64(&mut version, str, b"Version\0".as_ptr() as *const c_char) != 0 {
            return -1;
        }
    }
    version as c_int
}
}

// ── `if (parse_tag_uint64(&version, str, "Version"))` · fs_utils.c:360-361 · chunk 1156 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (parse_tag_uint64(&version, str, "Version"))
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1156 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn parse_tag_uint64(pval: *mut u64, str: *const c_char, tag: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn parse_tag_version__part1156(version: *mut u64, str: *const c_char) -> c_int {
    static VERSION: &[u8; 8] = b"Version\0";
    unsafe { parse_tag_uint64(version, str, VERSION.as_ptr() as *const c_char) }
}
}

// ── `return version;` · fs_utils.c:362-362 · chunk 1157 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return version;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1157 {
// Ported piece: `return version;` from parse_tag_version in fs_utils.c.
// `version` is a uint64_t (unsigned long int) local; the C return converts it
// to the function's int result type, i.e. truncation to the low 32 bits.

#[no_mangle]
pub extern "C" fn parse_tag_version__part1157(version: u64) -> i32 {
    version as i32
}
}

// ── `is_url` · fs_utils.c:365-370 · chunk 1158 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BOOL is_url(const char *path)
//     {
//         return (strstart(path, "http:", 
//                                        ((void *)0)
//                                            ) ||
//                 strstart(path, "https:", 
//                                         ((void *)0)
//                                             ) ||
//                 strstart(path, "file:", 
//                                        ((void *)0)
//                                            ));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1158 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strstart(str_: *const c_char, val: *const c_char, ptr: *mut *const c_char) -> c_int;
}

static HTTP: &[u8] = b"http:\0";
static HTTPS: &[u8] = b"https:\0";
static FILE_: &[u8] = b"file:\0";

#[no_mangle]
pub extern "C" fn is_url(path: *const c_char) -> c_int {
    unsafe {
        if strstart(path, HTTP.as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
            return 1;
        }
        if strstart(path, HTTPS.as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
            return 1;
        }
        if strstart(path, FILE_.as_ptr() as *const c_char, core::ptr::null_mut()) != 0 {
            return 1;
        }
        0
    }
}
}

// ── `return (strstart(path, "http:", ((void *)0) ) || strstart(path, "https:", ((void *)0) ) || strs…` · fs_utils.c:367-369 · chunk 1159 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (strstart(path, "http:", 
//                                        ((void *)0)
//                                            ) ||
//                 strstart(path, "https:", 
//                                         ((void *)0)
//                                             ) ||
//                 strstart(path, "file:", 
//                                        ((void *)0)
//                                            ));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1159 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strstart(str_: *const c_char, val: *const c_char, ptr: *const *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn is_url__part1159(path: *const c_char) -> c_int {
    unsafe {
        if strstart(path, b"http:\0".as_ptr() as *const c_char, core::ptr::null()) != 0 {
            return 1;
        }
        if strstart(path, b"https:\0".as_ptr() as *const c_char, core::ptr::null()) != 0 {
            return 1;
        }
        if strstart(path, b"file:\0".as_ptr() as *const c_char, core::ptr::null()) != 0 {
            return 1;
        }
        0
    }
}
}

