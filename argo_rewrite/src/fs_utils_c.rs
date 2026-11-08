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

