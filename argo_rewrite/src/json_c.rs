//! From `json.c`.

// ── `parse_string` · json.c:40-94 · chunk 1290 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static JSONValue parse_string(const char **pp)
//     {
//         char buf[4096], *q;
//         const char *p;
//         int c, h;
//     
//         q = buf;
//         p = *pp;
//         p++;
//         for(;;) {
//             c = *p++;
//             if (c == '\0' || c == '\n') {
//                 return json_error_new("unterminated string");
//             } else if (c == '\"') {
//                 break;
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
//                             return json_error_new("invalig hex digit");
//                         c = h << 4;
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return json_error_new("invalig hex digit");
//                         c |= h;
//                         goto add_char;
//                     default:
//                         return json_error_new("unknown escape code");
//                     }
//                 } else {
//                 add_char:
//                 if (q >= buf + sizeof(buf) - 1)
//                     return json_error_new("string too long");
//                 *q++ = c;
//             }
//         }
//         *q = '\0';
//         *pp = p;
//         return json_string_new(buf);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1290 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONUnion,
}

extern "C" {
    fn json_string_new(s: *const c_char) -> JSONValue;
    fn json_error_new(fmt: *const c_char, ...) -> JSONValue;
}

static ERR_UNTERMINATED: &[u8] = b"unterminated string\0";
static ERR_HEX: &[u8] = b"invalig hex digit\0";
static ERR_ESCAPE: &[u8] = b"unknown escape code\0";
static ERR_LONG: &[u8] = b"string too long\0";

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
pub unsafe extern "C" fn parse_string(pp: *const *const c_char) -> JSONValue {
    let mut buf = [0 as c_char; 4096];
    let mut q: usize = 0;
    let mut p: *const c_char = *pp;
    p = p.add(1);

    loop {
        let ch = *p;
        p = p.add(1);
        let c: c_int = ch as c_int;

        if c == 0 || c == b'\n' as c_int {
            return json_error_new(ERR_UNTERMINATED.as_ptr() as *const c_char);
        } else if c == b'"' as c_int {
            break;
        } else if c == b'\\' as c_int {
            let e = *p;
            p = p.add(1);
            let val: c_int = match e as u8 {
                b'\'' => e as c_int,
                b'"' => e as c_int,
                b'\\' => e as c_int,
                b'n' => b'\n' as c_int,
                b'r' => b'\r' as c_int,
                b't' => b'\t' as c_int,
                b'x' => {
                    let h = from_hex(*p as c_int);
                    p = p.add(1);
                    if h < 0 {
                        return json_error_new(ERR_HEX.as_ptr() as *const c_char);
                    }
                    let mut cc: c_int = h << 4;
                    let h2 = from_hex(*p as c_int);
                    p = p.add(1);
                    if h2 < 0 {
                        return json_error_new(ERR_HEX.as_ptr() as *const c_char);
                    }
                    cc |= h2;
                    cc
                }
                _ => {
                    return json_error_new(ERR_ESCAPE.as_ptr() as *const c_char);
                }
            };
            // add_char
            if q >= 4096 - 1 {
                return json_error_new(ERR_LONG.as_ptr() as *const c_char);
            }
            buf[q] = val as c_char;
            q += 1;
        } else {
            // add_char
            if q >= 4096 - 1 {
                return json_error_new(ERR_LONG.as_ptr() as *const c_char);
            }
            buf[q] = val_of(c);
            q += 1;
        }
    }

    buf[q] = 0;
    *(pp as *mut *const c_char) = p;
    json_string_new(buf.as_ptr())
}

#[inline]
fn val_of(c: c_int) -> c_char {
    c as c_char
}
}

// ── `char buf[4096], *q; const char *p; int c, h; q = buf; p = *pp; p++;` · json.c:42-48 · chunk 1291 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char buf[4096], *q;
//         const char *p;
//         int c, h;
//     
//         q = buf;
//         p = *pp;
//         p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1291 {
// Extracted initialization piece of parse_string().
//   char buf[4096], *q;
//   const char *p;
//   int c, h;
//   q = buf;
//   p = *pp;
//   p++;
// buf stays a C local (it is used by the rest of the function), so it is
// handed in as a pointer; q and p are written back through pointers.

use core::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn parse_string__part1291(
    pp: *mut *const c_char,
    buf: *mut c_char,
    q: *mut *mut c_char,
    p: *mut *const c_char,
) {
    // q = buf;
    *q = buf;
    // p = *pp; p++;
    let src: *const c_char = *pp;
    *p = src.add(1);
}
}

// ── `for (; ; )` · json.c:49-90 · chunk 1292 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             c = *p++;
//             if (c == '\0' || c == '\n') {
//                 return json_error_new("unterminated string");
//             } else if (c == '\"') {
//                 break;
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
//                             return json_error_new("invalig hex digit");
//                         c = h << 4;
//                         h = from_hex(*p++);
//                         if (h < 0)
//                             return json_error_new("invalig hex digit");
//                         c |= h;
//                         goto add_char;
//                     default:
//                         return json_error_new("unknown escape code");
//                     }
//                 } else {
//                 add_char:
//                 if (q >= buf + sizeof(buf) - 1)
//                     return json_error_new("string too long");
//                 *q++ = c;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1292 {
// Ported piece of json.c: parse_string's scanning loop.
// Returns 0 on success (string terminator found); nonzero error code otherwise.
// On success *pq and *pp are updated to the Rust-side cursor values.

use core::ffi::{c_char, c_int};

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
pub extern "C" fn parse_string__part1292(
    buf: *mut c_char,
    pq: *mut *mut c_char,
    pp: *mut *const c_char,
) -> c_int {
    unsafe {
        // buf + sizeof(buf) - 1, with sizeof(buf) == 4096
        let limit: usize = buf as usize + 4096 - 1;
        let mut p: *const c_char = *pp;
        let mut q: *mut c_char = *pq;
        let mut c: c_int;
        loop {
            c = *p as c_int; // char is signed: sign-extended into int
            p = p.add(1);
            if c == 0 || c == '\n' as c_int {
                return 1; // "unterminated string"
            } else if c == '"' as c_int {
                break;
            } else if c == '\\' as c_int {
                c = *p as c_int;
                p = p.add(1);
                match c {
                    0x27 | 0x22 | 0x5c => {} // '\''  '"'  '\\'
                    0x6e => c = '\n' as c_int, // 'n'
                    0x72 => c = '\r' as c_int, // 'r'
                    0x74 => c = '\t' as c_int, // 't'
                    0x78 => {
                        // 'x'
                        let mut h = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return 2; // "invalig hex digit"
                        }
                        c = h << 4;
                        h = from_hex(*p as c_int);
                        p = p.add(1);
                        if h < 0 {
                            return 2; // "invalig hex digit"
                        }
                        c |= h;
                    }
                    _ => return 3, // "unknown escape code"
                }
            }
            // add_char label (both the escape path and the plain-char path)
            if q as usize >= limit {
                return 4; // "string too long"
            }
            *q = c as c_char;
            q = q.add(1);
        }
        *pq = q;
        *pp = p;
        0
    }
}
}

// ── `*q = '\0'; *pp = p; return json_string_new(buf);` · json.c:91-93 · chunk 1293 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *q = '\0';
//         *pp = p;
//         return json_string_new(buf);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1293 {
// Port of the tail of parse_string(): terminates the buffer, advances the
// parse pointer and builds the JSON string value.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[]; flexible array member, not part of the struct size
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_string_new(str_: *const c_char) -> JSONValue;
}

#[no_mangle]
pub unsafe extern "C" fn parse_string__part1293(
    q: *mut c_char,
    pp: *mut *const c_char,
    p: *const c_char,
    buf: *const c_char,
) -> JSONValue {
    *q = 0;
    *pp = p;
    json_string_new(buf)
}
}

// ── `json_object_get2` · json.c:96-106 · chunk 1294 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static JSONProperty *json_object_get2(JSONObject *obj, const char *name)
//     {
//         JSONProperty *f;
//         int i;
//         for(i = 0; i < obj->len; i++) {
//             f = &obj->props[i];
//             if (!strcmp(f->name.u.str->data, name))
//                 return f;
//         }
//         return 
//               ((void *)0)
//                   ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1294 {
// Rust rewrite of json_object_get2 (json.c lines 96-106)
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn json_object_get2(
    obj: *mut JSONObject,
    name: *const c_char,
) -> *mut JSONProperty {
    let mut i: c_int = 0;
    while i < (*obj).len {
        let f: *mut JSONProperty = (*obj).props.offset(i as isize);
        let s: *mut JSONString = (*f).name.u.str_;
        let name_data = core::ptr::addr_of!((*s).data) as *const c_char;
        if strcmp(name_data, name) == 0 {
            return f;
        }
        i += 1;
    }
    core::ptr::null_mut()
}
}

// ── `for (i = 0; i < obj->len; i++)` · json.c:100-104 · chunk 1295 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < obj->len; i++) {
//             f = &obj->props[i];
//             if (!strcmp(f->name.u.str->data, name))
//                 return f;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1295 {
// Port of the linear property lookup loop from json_object_get2.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[no_mangle]
pub extern "C" fn json_object_get2__part1295(
    obj: *mut JSONObject,
    name: *const c_char,
    i: *mut c_int,
) -> *mut JSONProperty {
    let mut idx: c_int = 0;
    unsafe { *i = 0 };
    while idx < unsafe { (*obj).len } {
        let f: *mut JSONProperty = unsafe { (*obj).props.add(idx as usize) };
        let s: *mut JSONString = unsafe { (*f).name.u.str_ };
        let data: *const c_char = unsafe { core::ptr::addr_of!((*s).data) as *const c_char };
        if unsafe { strcmp(data, name) } == 0 {
            unsafe { *i = idx };
            return f;
        }
        idx += 1;
        unsafe { *i = idx };
    }
    unsafe { *i = idx };
    core::ptr::null_mut()
}
}

// ── `return ((void *)0) ;` · json.c:105-105 · chunk 1296 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1296 {
// Ported tail of json_object_get2: the not-found path returns a null
// JSONProperty pointer.

#[repr(C)]
pub struct JSONProperty {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn json_object_get2__part1296() -> *mut JSONProperty {
    core::ptr::null_mut()
}
}

// ── `json_object_get` · json.c:108-120 · chunk 1297 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_object_get(JSONValue val, const char *name)
//     {
//         JSONProperty *f;
//         JSONObject *obj;
//     
//         if (val.type != JSON_OBJ)
//             return json_undefined_new();
//         obj = val.u.obj;
//         f = json_object_get2(obj, name);
//         if (!f)
//             return json_undefined_new();
//         return f->value;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1297 {
// Rewrite of json_object_get from json.c (lines 108-120).
// The static helper json_object_get2 is inlined here as a private Rust fn.

use core::ffi::{c_char, c_int};

const JSON_OBJ: c_int = 2;
const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    // char data[0];
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[inline]
fn json_undefined_new() -> JSONValue {
    JSONValue {
        type_: JSON_UNDEFINED,
        u: JSONValueU { int32: 0 },
    }
}

// Port of the static C helper json_object_get2().
unsafe fn json_object_get2(obj: *mut JSONObject, name: *const c_char) -> *mut JSONProperty {
    let len = (*obj).len;
    let mut i: c_int = 0;
    while i < len {
        let f: *mut JSONProperty = (*obj).props.offset(i as isize);
        let s: *mut JSONString = (*f).name.u.str_;
        // JSONString { int len; char data[0]; } -> data starts right after len.
        let data: *const c_char = (s as *const c_char).add(core::mem::size_of::<c_int>());
        if strcmp(data, name) == 0 {
            return f;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue {
    unsafe {
        if val.type_ != JSON_OBJ {
            return json_undefined_new();
        }
        let obj: *mut JSONObject = val.u.obj;
        let f = json_object_get2(obj, name);
        if f.is_null() {
            return json_undefined_new();
        }
        (*f).value
    }
}
}

// ── `if (val.type != JSON_OBJ)` · json.c:113-114 · chunk 1298 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_OBJ)
//             return json_undefined_new();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1298 {
// Port of the `if (val.type != JSON_OBJ) return json_undefined_new();`
// guard of json_object_get(). `json_undefined_new` is a static inline C
// function, so its logic is reproduced here.

use core::ffi::c_int;

pub type BOOL = c_int;

// Same enum values as the C JSONTypeEnum.
const JSON_OBJ: c_int = 2;
const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [i8; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueUnion,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

// Mirror of the C static inline json_undefined_new(): type = JSON_UNDEFINED,
// u.int32 = 0 (all bytes of the value fully defined here).
fn json_undefined_new() -> JSONValue {
    let mut v: JSONValue = unsafe { core::mem::zeroed() };
    v.type_ = JSON_UNDEFINED;
    v
}

// Returns nonzero when the caller must return *out from json_object_get,
// zero when the caller should continue with the object lookup.
#[no_mangle]
pub extern "C" fn json_object_get__part1298(val: JSONValue, out: *mut JSONValue) -> c_int {
    if val.type_ != JSON_OBJ {
        unsafe {
            core::ptr::write(out, json_undefined_new());
        }
        return 1;
    }
    0
}
}

// ── `obj = val.u.obj; f = json_object_get2(obj, name);` · json.c:115-116 · chunk 1299 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     obj = val.u.obj;
//         f = json_object_get2(obj, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1299 {
// Port of json.c lines 115-116 inside json_object_get():
//     obj = val.u.obj;
//     f = json_object_get2(obj, name);
// json_object_get2 is a static C function (no linker symbol), so its logic is
// reimplemented here as a private Rust fn.

use core::ffi::{c_char, c_int};

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

// Reimplementation of the static C helper json_object_get2().
unsafe fn json_object_get2(obj: *mut JSONObject, name: *const c_char) -> *mut JSONProperty {
    let mut i: c_int = 0;
    while i < (*obj).len {
        let f: *mut JSONProperty = (*obj).props.offset(i as isize);
        let s: *mut JSONString = (*f).name.u.str_;
        let sdata: *const c_char = core::ptr::addr_of!((*s).data) as *const c_char;
        if strcmp(sdata, name) == 0 {
            return f;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn json_object_get__part1299(
    val: JSONValue,
    name: *const c_char,
    obj_out: *mut *mut JSONObject,
    f_out: *mut *mut JSONProperty,
) {
    let obj: *mut JSONObject = val.u.obj;
    *obj_out = obj;
    let f: *mut JSONProperty = json_object_get2(obj, name);
    *f_out = f;
}
}

// ── `if (!f)` · json.c:117-118 · chunk 1300 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!f)
//             return json_undefined_new();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1300 {
// Port of the `if (!f) return json_undefined_new();` piece of json_object_get().
// json_undefined_new() is `static inline` in C (no linker symbol), so its logic
// is reproduced here privately.

use core::ffi::{c_char, c_int, c_uint};

pub type BOOL = c_int;
pub type JSONTypeEnum = c_uint;

const JSON_UNDEFINED: JSONTypeEnum = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: JSONTypeEnum,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

// Equivalent of the static inline json_undefined_new().
fn json_undefined_new() -> JSONValue {
    JSONValue {
        type_: JSON_UNDEFINED,
        u: JSONValueU { int32: 0 },
    }
}

/// Returns 1 (and stores the undefined value in *pret) when f is NULL, 0 otherwise.
#[no_mangle]
pub extern "C" fn json_object_get__part1300(
    f: *mut JSONProperty,
    pret: *mut JSONValue,
) -> c_int {
    if f.is_null() {
        let val = json_undefined_new();
        if !pret.is_null() {
            unsafe {
                core::ptr::write(pret, val);
            }
        }
        1
    } else {
        0
    }
}
}

// ── `return f->value;` · json.c:119-119 · chunk 1301 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return f->value;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1301 {
// Ported piece of json_object_get: return f->value;

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: core::ffi::c_int,
    pub b: core::ffi::c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: core::ffi::c_int, // JSONTypeEnum
    pub u: JSONValueU,
}

#[repr(C)]
pub struct JSONString {
    pub len: core::ffi::c_int,
    // char data[0] flexible array member
}

#[repr(C)]
pub struct JSONObject {
    pub len: core::ffi::c_int,
    pub size: core::ffi::c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: core::ffi::c_int,
    pub size: core::ffi::c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[no_mangle]
pub extern "C" fn json_object_get__part1301(f: *const JSONProperty) -> JSONValue {
    unsafe { (*f).value }
}
}

// ── `json_object_set` · json.c:122-146 · chunk 1302 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int json_object_set(JSONValue val, const char *name, JSONValue prop_val)
//     {
//         JSONObject *obj;
//         JSONProperty *f;
//         int new_size;
//     
//         if (val.type != JSON_OBJ)
//             return -1;
//         obj = val.u.obj;
//         f = json_object_get2(obj, name);
//         if (f) {
//             json_free(f->value);
//             f->value = prop_val;
//         } else {
//             if (obj->len >= obj->size) {
//                 new_size = max_int(obj->len + 1, obj->size * 3 / 2);
//                 obj->props = realloc(obj->props, new_size * sizeof(JSONProperty));
//                 obj->size = new_size;
//             }
//             f = &obj->props[obj->len++];
//             f->name = json_string_new(name);
//             f->value = prop_val;
//         }
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1302 {
// Rust rewrite of json_object_set (json.c) for the pseudo JSON parser.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

const JSON_OBJ: c_int = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn json_free(val: JSONValue);
    fn json_string_new(s: *const c_char) -> JSONValue;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// Ported from the static C helper json_object_get2().
unsafe fn json_object_get2(obj: *mut JSONObject, name: *const c_char) -> *mut JSONProperty {
    let mut i: c_int = 0;
    while i < (*obj).len {
        let f: *mut JSONProperty = (*obj).props.add(i as usize);
        let s: *mut JSONString = (*f).name.u.str_;
        let data: *const c_char = core::ptr::addr_of!((*s).data) as *const c_char;
        if strcmp(data, name) == 0 {
            return f;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn json_object_set(
    val: JSONValue,
    name: *const c_char,
    prop_val: JSONValue,
) -> c_int {
    if val.type_ != JSON_OBJ {
        return -1;
    }
    let obj: *mut JSONObject = val.u.obj;
    let mut f: *mut JSONProperty = json_object_get2(obj, name);
    if !f.is_null() {
        json_free((*f).value);
        (*f).value = prop_val;
    } else {
        if (*obj).len >= (*obj).size {
            let new_size: c_int = max_int(
                (*obj).len.wrapping_add(1),
                (*obj).size.wrapping_mul(3) / 2,
            );
            let bytes: usize =
                (new_size as usize).wrapping_mul(core::mem::size_of::<JSONProperty>());
            let p: *mut c_void = realloc((*obj).props as *mut c_void, bytes);
            (*obj).props = p as *mut JSONProperty;
            (*obj).size = new_size;
        }
        let idx: c_int = (*obj).len;
        (*obj).len = idx.wrapping_add(1);
        f = (*obj).props.add(idx as usize);
        (*f).name = json_string_new(name);
        (*f).value = prop_val;
    }
    0
}
}

// ── `if (val.type != JSON_OBJ)` · json.c:128-129 · chunk 1303 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_OBJ)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1303 {
// Ported piece of json_object_set: the early type check.
// Returns 1 when the value is not a JSON object (caller must `return -1`),
// 0 when the original code would have fallen through.

const JSON_OBJ: core::ffi::c_int = 2;

#[no_mangle]
pub extern "C" fn json_object_set__part1303(val_type: core::ffi::c_int) -> core::ffi::c_int {
    if val_type != JSON_OBJ {
        1
    } else {
        0
    }
}
}

// ── `obj = val.u.obj; f = json_object_get2(obj, name);` · json.c:130-131 · chunk 1304 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     obj = val.u.obj;
//         f = json_object_get2(obj, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1304 {
// Port of a slice of json_object_set(): computing `obj = val.u.obj` and
// `f = json_object_get2(obj, name)`.  json_object_get2 is `static` in the C
// translation unit, so its logic is reproduced here as a private Rust fn.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int, /* JSONTypeEnum */
    pub u: JSONValueU,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

// Mirror of the static C helper json_object_get2().
unsafe fn object_get2(obj: *mut JSONObject, name: *const c_char) -> *mut JSONProperty {
    let len = (*obj).len;
    let mut i: c_int = 0;
    while i < len {
        let f: *mut JSONProperty = (*obj).props.offset(i as isize);
        let s: *mut JSONString = (*f).name.u.str_;
        let data: *const c_char = core::ptr::addr_of!((*s).data) as *const c_char;
        if strcmp(data, name) == 0 {
            return f;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn json_object_set__part1304(
    val: JSONValue,
    name: *const c_char,
    out_obj: *mut *mut JSONObject,
    out_f: *mut *mut JSONProperty,
) {
    let obj: *mut JSONObject = val.u.obj;
    *out_obj = obj;
    *out_f = object_get2(obj, name);
}
}

// ── `if (f) … else` · json.c:132-144 · chunk 1305 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (f) {
//             json_free(f->value);
//             f->value = prop_val;
//         } else {
//             if (obj->len >= obj->size) {
//                 new_size = max_int(obj->len + 1, obj->size * 3 / 2);
//                 obj->props = realloc(obj->props, new_size * sizeof(JSONProperty));
//                 obj->size = new_size;
//             }
//             f = &obj->props[obj->len++];
//             f->name = json_string_new(name);
//             f->value = prop_val;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1305 {
// Port of the property update / insert block of json_object_set (json.c 132-144).
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

extern "C" {
    fn json_free(val: JSONValue);
    fn json_string_new(str_: *const c_char) -> JSONValue;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn json_object_set__part1305(
    f_p: *mut *mut JSONProperty,
    obj: *mut JSONObject,
    name: *const c_char,
    prop_val: JSONValue,
    new_size_p: *mut c_int,
) {
    let cur: *mut JSONProperty = *f_p;
    if !cur.is_null() {
        json_free((*cur).value);
        (*cur).value = prop_val;
        *f_p = cur;
    } else {
        if (*obj).len >= (*obj).size {
            let new_size: c_int = max_int(
                (*obj).len.wrapping_add(1),
                (*obj).size.wrapping_mul(3) / 2,
            );
            let p = realloc(
                (*obj).props as *mut c_void,
                (new_size as usize).wrapping_mul(core::mem::size_of::<JSONProperty>()),
            );
            (*obj).props = p as *mut JSONProperty;
            (*obj).size = new_size;
            *new_size_p = new_size;
        }
        let idx: c_int = (*obj).len;
        (*obj).len = idx.wrapping_add(1);
        let f: *mut JSONProperty = (*obj).props.add(idx as usize);
        (*f).name = json_string_new(name);
        (*f).value = prop_val;
        *f_p = f;
    }
}
}

// ── `return 0;` · json.c:145-145 · chunk 1306 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1306 {
// Piece of json_object_set: the trailing `return 0;` of the successful path.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn json_object_set__part1306() -> c_int {
    0
}
}

// ── `json_array_get` · json.c:148-160 · chunk 1307 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_array_get(JSONValue val, unsigned int idx)
//     {
//         JSONArray *array;
//     
//         if (val.type != JSON_ARRAY)
//             return json_undefined_new();
//         array = val.u.array;
//         if (idx < array->len) {
//             return array->tab[idx];
//         } else {
//             return json_undefined_new();
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1307 {
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_uint};

pub const JSON_ARRAY: c_int = 3;
pub const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0];
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[inline]
fn undefined_new() -> JSONValue {
    JSONValue {
        type_: JSON_UNDEFINED,
        u: JSONValueU { int32: 0 },
    }
}

#[no_mangle]
pub extern "C" fn json_array_get(val: JSONValue, idx: c_uint) -> JSONValue {
    if val.type_ != JSON_ARRAY {
        return undefined_new();
    }
    unsafe {
        let array = val.u.array;
        if array.is_null() {
            return undefined_new();
        }
        // C compares unsigned int idx against int len after the usual
        // arithmetic conversions (len reinterpreted as unsigned).
        if idx < (*array).len as c_uint {
            return core::ptr::read((*array).tab.add(idx as usize));
        }
        undefined_new()
    }
}
}

// ── `if (val.type != JSON_ARRAY)` · json.c:152-153 · chunk 1308 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_ARRAY)
//             return json_undefined_new();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1308 {
// Rust port of the guard inside json_array_get:
//   if (val.type != JSON_ARRAY) return json_undefined_new();
// Only the test is ported here; the C caller keeps the `return`.

use core::ffi::{c_char, c_int};

const JSON_ARRAY: c_int = 3;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

/// Returns nonzero when `val.type != JSON_ARRAY`.
#[no_mangle]
pub extern "C" fn json_array_get__part1308(val: *const JSONValue) -> c_int {
    if val.is_null() {
        return 1;
    }
    let t = unsafe { (*val).type_ };
    if t != JSON_ARRAY {
        1
    } else {
        0
    }
}
}

// ── `array = val.u.array;` · json.c:154-154 · chunk 1309 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     array = val.u.array;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1309 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[no_mangle]
pub extern "C" fn json_array_get__part1309(val: JSONValue) -> *mut JSONArray {
    unsafe { val.u.array }
}
}

// ── `if (idx < array->len) … else` · json.c:155-159 · chunk 1310 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (idx < array->len) {
//             return array->tab[idx];
//         } else {
//             return json_undefined_new();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1310 {
// Port of the tail of json_array_get: index check and value fetch.
use core::ffi::c_int;

const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONValueUnion {
    pub ptr_or_int: u64,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueUnion,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[no_mangle]
pub extern "C" fn json_array_get__part1310(array: *mut JSONArray, idx: u32) -> JSONValue {
    unsafe {
        if !array.is_null() && (idx as c_int) < (*array).len {
            let tab = (*array).tab;
            if !tab.is_null() {
                let slot = tab.add(idx as usize);
                return JSONValue {
                    r#type: (*slot).r#type,
                    u: JSONValueUnion {
                        ptr_or_int: (*slot).u.ptr_or_int,
                    },
                };
            }
        }
    }
    JSONValue {
        r#type: JSON_UNDEFINED,
        u: JSONValueUnion { ptr_or_int: 0 },
    }
}
}

// ── `json_array_set` · json.c:162-184 · chunk 1311 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int json_array_set(JSONValue val, unsigned int idx, JSONValue prop_val)
//     {
//         JSONArray *array;
//         int new_size;
//     
//         if (val.type != JSON_ARRAY)
//             return -1;
//         array = val.u.array;
//         if (idx < array->len) {
//             json_free(array->tab[idx]);
//             array->tab[idx] = prop_val;
//         } else if (idx == array->len) {
//             if (array->len >= array->size) {
//                 new_size = max_int(array->len + 1, array->size * 3 / 2);
//                 array->tab = realloc(array->tab, new_size * sizeof(JSONValue));
//                 array->size = new_size;
//             }
//             array->tab[array->len++] = prop_val;
//         } else {
//             return -1;
//         }
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1311 {
// Rust rewrite of json_array_set
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

pub type BOOL = c_int;
pub type JSONTypeEnum = c_int;

pub const JSON_STR: JSONTypeEnum = 0;
pub const JSON_INT: JSONTypeEnum = 1;
pub const JSON_OBJ: JSONTypeEnum = 2;
pub const JSON_ARRAY: JSONTypeEnum = 3;
pub const JSON_BOOL: JSONTypeEnum = 4;
pub const JSON_NULL: JSONTypeEnum = 5;
pub const JSON_UNDEFINED: JSONTypeEnum = 6;
pub const JSON_EXCEPTION: JSONTypeEnum = 7;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0];  flexible array member, not addressable as a field
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub r#type: JSONTypeEnum,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

extern "C" {
    fn json_free(val: JSONValue);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

#[inline]
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn json_array_set(val: JSONValue, idx: c_uint, prop_val: JSONValue) -> c_int {
    if val.r#type != JSON_ARRAY {
        return -1;
    }
    let array: *mut JSONArray = val.u.array;
    let len: c_int = (*array).len;
    if idx < len as c_uint {
        let tab = (*array).tab;
        json_free(*tab.add(idx as usize));
        *tab.add(idx as usize) = prop_val;
    } else if idx == len as c_uint {
        if (*array).len >= (*array).size {
            let new_size: c_int = max_int(
                (*array).len.wrapping_add(1),
                (*array).size.wrapping_mul(3) / 2,
            );
            let p = realloc(
                (*array).tab as *mut c_void,
                (new_size as usize).wrapping_mul(size_of::<JSONValue>()),
            );
            (*array).tab = p as *mut JSONValue;
            (*array).size = new_size;
        }
        let l = (*array).len;
        *(*array).tab.add(l as usize) = prop_val;
        (*array).len = l.wrapping_add(1);
    } else {
        return -1;
    }
    0
}

// keep the unused-import lint quiet without changing anything
#[allow(dead_code)]
fn _unused(_p: *const c_char) {}
}

// ── `if (val.type != JSON_ARRAY)` · json.c:167-168 · chunk 1312 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_ARRAY)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1312 {
// Port of the JSON_ARRAY type guard at the head of json_array_set().
// Returns 1 when the caller must return -1 (val is not an array), 0 otherwise.

use core::ffi::{c_int, c_void};

// Mirror of the C union inside JSONValue. Only `type` is inspected here, but the
// whole struct is passed by value so its layout must match exactly:
// all members are pointer- or int-sized, align 8 on LP64.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of `typedef struct JSONValue { JSONTypeEnum type; union {...} u; }`.
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

const JSON_ARRAY: c_int = 3;

#[no_mangle]
pub extern "C" fn json_array_set__part1312(val: JSONValue) -> c_int {
    if val.type_ != JSON_ARRAY {
        1
    } else {
        0
    }
}
}

// ── `array = val.u.array;` · json.c:169-169 · chunk 1313 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     array = val.u.array;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1313 {
// Ported piece of json_array_set: `array = val.u.array;`

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_array_set__part1313(val: JSONValue) -> *mut JSONArray {
    unsafe { val.u.array }
}
}

// ── `if (idx < array->len) … else` · json.c:170-182 · chunk 1314 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (idx < array->len) {
//             json_free(array->tab[idx]);
//             array->tab[idx] = prop_val;
//         } else if (idx == array->len) {
//             if (array->len >= array->size) {
//                 new_size = max_int(array->len + 1, array->size * 3 / 2);
//                 array->tab = realloc(array->tab, new_size * sizeof(JSONValue));
//                 array->size = new_size;
//             }
//             array->tab[array->len++] = prop_val;
//         } else {
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1314 {
// Port of the body of json_array_set (indexing / growth branch).
use core::ffi::{c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_free(val: JSONValue);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// static inline in C: no linker symbol, so reimplemented here.
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// Returns 0 on success, -1 when idx > array->len (the C `return -1` path).
#[no_mangle]
pub unsafe extern "C" fn json_array_set__part1314(
    array: *mut JSONArray,
    idx: c_uint,
    prop_val: JSONValue,
    new_size: *mut c_int,
) -> c_int {
    let len = (*array).len;
    if idx < len as c_uint {
        let slot = (*array).tab.add(idx as usize);
        json_free(ptr::read(slot));
        ptr::write(slot, prop_val);
    } else if idx == len as c_uint {
        if (*array).len >= (*array).size {
            let ns = max_int(
                (*array).len.wrapping_add(1),
                (*array).size.wrapping_mul(3).wrapping_div(2),
            );
            *new_size = ns;
            let bytes = (ns as usize).wrapping_mul(core::mem::size_of::<JSONValue>());
            (*array).tab = realloc((*array).tab as *mut c_void, bytes) as *mut JSONValue;
            (*array).size = ns;
        }
        let l = (*array).len;
        ptr::write((*array).tab.add(l as usize), prop_val);
        (*array).len = l.wrapping_add(1);
    } else {
        return -1;
    }
    0
}
}

// ── `return 0;` · json.c:183-183 · chunk 1315 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1315 {
// Ported piece of json_array_set: the trailing `return 0;` (success path).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn json_array_set__part1315() -> c_int {
    0
}
}

// ── `json_get_str` · json.c:186-191 · chunk 1316 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *json_get_str(JSONValue val)
//     {
//         if (val.type != JSON_STR)
//             return 
//                   ((void *)0)
//                       ;
//         return val.u.str->data;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1316 {
// Port of `const char *json_get_str(JSONValue val)` from json.c.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C `JSONString` struct (flexible array member `data[0]`).
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

// Mirror of the anonymous union inside `JSONValue`.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of the C `JSONValue` struct.
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

const JSON_STR: c_int = 0;

#[no_mangle]
pub extern "C" fn json_get_str(val: JSONValue) -> *const c_char {
    unsafe {
        if val.type_ != JSON_STR {
            return core::ptr::null();
        }
        let s = val.u.str_;
        core::ptr::addr_of!((*s).data) as *const c_char
    }
}
}

// ── `if (val.type != JSON_STR)` · json.c:188-189 · chunk 1317 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_STR)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1317 {
// Port of the type check of json_get_str (json.c lines 188-189).
// Returns nonzero when the value is NOT a string, so the C caller returns NULL.

use core::ffi::{c_char, c_int};

pub const JSON_STR: c_int = 0;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0] -- flexible array member, never accessed here
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_get_str__part1317(val: JSONValue) -> c_int {
    let _ = core::mem::size_of::<c_char>();
    if val.type_ != JSON_STR {
        return 1;
    }
    0
}
}

// ── `return val.u.str->data;` · json.c:190-190 · chunk 1318 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val.u.str->data;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1318 {
// Ported piece of json_get_str: `return val.u.str->data;`
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_get_str__part1318(val: JSONValue) -> *const c_char {
    unsafe {
        let strp: *mut JSONString = val.u.str_;
        core::ptr::addr_of!((*strp).data) as *const c_char
    }
}
}

// ── `json_get_error` · json.c:193-198 · chunk 1319 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *json_get_error(JSONValue val)
//     {
//         if (val.type != JSON_EXCEPTION)
//             return 
//                   ((void *)0)
//                       ;
//         return val.u.str->data;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1319 {
// Rust port of json_get_error() from json.c
use core::ffi::{c_char, c_int};

const JSON_EXCEPTION: c_int = 7;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_get_error(val: JSONValue) -> *const c_char {
    if val.type_ != JSON_EXCEPTION {
        return core::ptr::null();
    }
    unsafe {
        let s = val.u.str_;
        if s.is_null() {
            return core::ptr::null();
        }
        core::ptr::addr_of!((*s).data) as *const c_char
    }
}
}

// ── `if (val.type != JSON_EXCEPTION)` · json.c:195-196 · chunk 1320 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_EXCEPTION)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1320 {
// Port of the TARGET inside json_get_error: the early "not an exception -> NULL"
// test. Returns nonzero when the caller must return NULL.
use core::ffi::{c_int, c_void};

// JSONValue.u mirror: union of a JSONString*, JSONObject*, JSONArray*, int, BOOL.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int, // JSONTypeEnum
    pub u: JSONValueU,
}

const JSON_EXCEPTION: c_int = 7;

#[no_mangle]
pub extern "C" fn json_get_error__part1320(val: JSONValue) -> c_int {
    if val.type_ != JSON_EXCEPTION {
        1
    } else {
        0
    }
}
}

// ── `return val.u.str->data;` · json.c:197-197 · chunk 1321 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val.u.str->data;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1321 {
// Port of `return val.u.str->data;` from json_get_error.
use core::ffi::{c_char, c_int};

// Mirrors the C `JSONString` (flexible array member `data` starts at offset 4).
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

// Mirrors the anonymous union inside `JSONValue`.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

// Mirrors `JSONValue` field-for-field (enum JSONTypeEnum has int size/alignment).
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_get_error__part1321(val: JSONValue) -> *const c_char {
    unsafe {
        let s = val.u.str_;
        // &s->data == (char *)s + offsetof(JSONString, data) == (char *)s + 4
        (s as *const u8).wrapping_add(4) as *const c_char
    }
}
}

// ── `json_string_new2` · json.c:200-211 · chunk 1322 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_string_new2(const char *str, int len)
//     {
//         JSONValue val;
//         JSONString *str1;
//     
//         str1 = malloc(sizeof(JSONString) + len + 1);
//         str1->len = len;
//         memcpy(str1->data, str, len + 1);
//         val.type = JSON_STR;
//         val.u.str = str1;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1322 {
// Rust port of json_string_new2() from json.c
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of C JSONString: { int len; char data[0]; }
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

// Mirror of the C union inside JSONValue.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of C JSONValue: { JSONTypeEnum type; union {...} u; }
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

const JSON_STR: c_int = 0;

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn json_string_new2(str_: *const c_char, len: c_int) -> JSONValue {
    // sizeof(JSONString) + len + 1, with C's modular size_t arithmetic.
    let size = core::mem::size_of::<JSONString>()
        .wrapping_add(len as usize)
        .wrapping_add(1);

    let str1 = malloc(size) as *mut JSONString;

    if !str1.is_null() {
        (*str1).len = len;
        // str1->data starts right after the int len field.
        let data = (str1 as *mut u8).add(core::mem::size_of::<c_int>()) as *mut c_char;
        core::ptr::copy_nonoverlapping(str_, data, (len as usize).wrapping_add(1));
    }

    let mut val: JSONValue = core::mem::zeroed();
    val.type_ = JSON_STR;
    val.u = JSONValueU { str_: str1 };
    val
}
}

// ── `JSONValue val; JSONString *str1; str1 = malloc(sizeof(JSONString) + len + 1); str1->len = len;…` · json.c:202-210 · chunk 1323 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         JSONString *str1;
//     
//         str1 = malloc(sizeof(JSONString) + len + 1);
//         str1->len = len;
//         memcpy(str1->data, str, len + 1);
//         val.type = JSON_STR;
//         val.u.str = str1;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1323 {
// Port of the body of json_string_new2 (json.c lines 202-210).
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

const JSON_STR: c_int = 0;

extern "C" {
    fn malloc(size: c_ulong) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: c_ulong) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn json_string_new2__part1323(
    str_: *const c_char,
    len: c_int,
) -> JSONValue {
    // size_t arithmetic mirrors the C: len is sign-extended to size_t.
    let size: usize = core::mem::size_of::<JSONString>()
        .wrapping_add(len as isize as usize)
        .wrapping_add(1);
    let n: c_ulong = (len.wrapping_add(1) as isize) as c_ulong;

    let str1: *mut JSONString = malloc(size as c_ulong) as *mut JSONString;
    (*str1).len = len;
    // `data` sits at offset size_of::<JSONString>() inside JSONString.
    let data: *mut c_char =
        (str1 as *mut c_char).wrapping_add(core::mem::size_of::<JSONString>());
    memcpy(data as *mut c_void, str_ as *const c_void, n);

    let mut val: JSONValue = JSONValue {
        type_: JSON_STR,
        u: JSONValueU { int32: 0 },
    };
    val.type_ = JSON_STR;
    val.u = JSONValueU { str_: str1 };
    val
}
}

// ── `json_string_new` · json.c:213-216 · chunk 1324 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_string_new(const char *str)
//     {
//         return json_string_new2(str, strlen(str));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1324 {
// Rust rewrite of json_string_new from json.c
use core::ffi::{c_char, c_int};

type BOOL = c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONObject {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct JSONArray {
    _priv: [u8; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_string_new2(str_: *const c_char, len: c_int) -> JSONValue;
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn json_string_new(str_: *const c_char) -> JSONValue {
    json_string_new2(str_, strlen(str_) as c_int)
}
}

// ── `return json_string_new2(str, strlen(str));` · json.c:215-215 · chunk 1325 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return json_string_new2(str, strlen(str));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1325 {
use core::ffi::{c_char, c_int};

pub type BOOL = c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut core::ffi::c_void,
}

#[repr(C)]
pub union JSONValueU {
    pub str: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_string_new2(str: *const c_char, len: c_int) -> JSONValue;
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub extern "C" fn json_string_new__part1325(str: *const c_char) -> JSONValue {
    unsafe {
        let len = strlen(str);
        json_string_new2(str, len as c_int)
    }
}
}

// ── `json_object_new` · json.c:232-240 · chunk 1327 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_object_new(void)
//     {
//         JSONValue val;
//         JSONObject *obj;
//         obj = mallocz(sizeof(JSONObject));
//         val.type = JSON_OBJ;
//         val.u.obj = obj;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1327 {
// Rust rewrite of json_object_new (json.c)
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

// C enum JSONTypeEnum is an int-typed enum.
pub type JSONTypeEnum = c_int;

const JSON_OBJ: JSONTypeEnum = 2;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // flexible array member `char data[0]` adds no size
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: JSONTypeEnum,
    pub u: JSONValueUnion,
}

extern "C" {
    fn mallocz(size: usize) -> *mut core::ffi::c_void;
}

#[no_mangle]
pub extern "C" fn json_object_new() -> JSONValue {
    let obj = unsafe { mallocz(core::mem::size_of::<JSONObject>()) } as *mut JSONObject;
    let val = JSONValue {
        r#type: JSON_OBJ,
        u: JSONValueUnion { obj },
    };
    val
}

// silence unused import warning for c_char (kept for parity with C declarations)
const _: Option<c_char> = None;
}

// ── `JSONValue val; JSONObject *obj; obj = mallocz(sizeof(JSONObject)); val.type = JSON_OBJ; val.u.o…` · json.c:234-239 · chunk 1328 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         JSONObject *obj;
//         obj = mallocz(sizeof(JSONObject));
//         val.type = JSON_OBJ;
//         val.u.obj = obj;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1328 {
// Port of the JSONValue/JSONObject construction piece of json_object_new.

use core::ffi::{c_int, c_void};

// C: typedef int BOOL;
pub type BOOL = c_int;

// C: typedef enum { JSON_STR, JSON_INT, JSON_OBJ, JSON_ARRAY, JSON_BOOL,
//                  JSON_NULL, JSON_UNDEFINED, JSON_EXCEPTION } JSONTypeEnum;
pub type JSONTypeEnum = c_int;

pub const JSON_OBJ: JSONTypeEnum = 2;

// C: typedef struct { int len; char data[0]; } JSONString;
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char_zero; 0],
}

#[repr(C)]
pub struct c_char_zero(pub core::ffi::c_char);

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: JSONTypeEnum,
    pub u: JSONValueU,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn json_object_new__part1328() -> JSONValue {
    let obj = mallocz(core::mem::size_of::<JSONObject>()) as *mut JSONObject;
    let mut val: JSONValue = core::mem::zeroed();
    val.type_ = JSON_OBJ;
    val.u.obj = obj;
    val
}
}

// ── `json_array_new` · json.c:242-250 · chunk 1329 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_array_new(void)
//     {
//         JSONValue val;
//         JSONArray *array;
//         array = mallocz(sizeof(JSONArray));
//         val.type = JSON_ARRAY;
//         val.u.array = array;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1329 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // flexible array member `char data[0]` contributes no size
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut c_void,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueUnion {
    pub str: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueUnion,
}

const JSON_ARRAY: c_int = 3;

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn json_array_new() -> JSONValue {
    unsafe {
        let array = mallocz(core::mem::size_of::<JSONArray>()) as *mut JSONArray;
        JSONValue {
            r#type: JSON_ARRAY,
            u: JSONValueUnion { array },
        }
    }
}
}

// ── `JSONValue val; JSONArray *array; array = mallocz(sizeof(JSONArray)); val.type = JSON_ARRAY; val…` · json.c:244-249 · chunk 1330 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         JSONArray *array;
//         array = mallocz(sizeof(JSONArray));
//         val.type = JSON_ARRAY;
//         val.u.array = array;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1330 {
// Port of json_array_new__part1330: the body of json_array_new().
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut c_void,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

// JSONTypeEnum: JSON_STR, JSON_INT, JSON_OBJ, JSON_ARRAY, ...
const JSON_ARRAY: c_int = 3;

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn json_array_new__part1330() -> JSONValue {
    unsafe {
        let array = mallocz(core::mem::size_of::<JSONArray>()) as *mut JSONArray;
        JSONValue {
            type_: JSON_ARRAY,
            u: JSONValueU { array },
        }
    }
}
}

// ── `json_free` · json.c:252-292 · chunk 1331 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void json_free(JSONValue val)
//     {
//         switch(val.type) {
//         case JSON_STR:
//         case JSON_EXCEPTION:
//             free(val.u.str);
//             break;
//         case JSON_INT:
//         case JSON_BOOL:
//         case JSON_NULL:
//         case JSON_UNDEFINED:
//             break;
//         case JSON_ARRAY:
//             {
//                 JSONArray *array = val.u.array;
//                 int i;
//     
//                 for(i = 0; i < array->len; i++) {
//                     json_free(array->tab[i]);
//                 }
//                 free(array);
//             }
//             break;
//         case JSON_OBJ:
//             {
//                 JSONObject *obj = val.u.obj;
//                 JSONProperty *f;
//                 int i;
//     
//                 for(i = 0; i < obj->len; i++) {
//                     f = &obj->props[i];
//                     json_free(f->name);
//                     json_free(f->value);
//                 }
//                 free(obj);
//             }
//             break;
//         default:
//             abort();
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1331 {
// Port of json_free() from json.c
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C structs (only the fields accessed here, exact order/types).
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

// JSONTypeEnum values from json.h
const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_OBJ: c_int = 2;
const JSON_ARRAY: c_int = 3;
const JSON_BOOL: c_int = 4;
const JSON_NULL: c_int = 5;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

extern "C" {
    fn free(ptr: *mut c_void);
    fn abort() -> !;
}

#[no_mangle]
pub extern "C" fn json_free(val: JSONValue) {
    unsafe {
        match val.type_ {
            JSON_STR | JSON_EXCEPTION => {
                free(val.u.str_ as *mut c_void);
            }
            JSON_INT | JSON_BOOL | JSON_NULL | JSON_UNDEFINED => {}
            JSON_ARRAY => {
                let array = val.u.array;
                let mut i: c_int = 0;
                while i < (*array).len {
                    json_free(*(*array).tab.offset(i as isize));
                    i = i.wrapping_add(1);
                }
                free(array as *mut c_void);
            }
            JSON_OBJ => {
                let obj = val.u.obj;
                let mut i: c_int = 0;
                while i < (*obj).len {
                    let f = (*obj).props.offset(i as isize);
                    json_free((*f).name);
                    json_free((*f).value);
                    i = i.wrapping_add(1);
                }
                free(obj as *mut c_void);
            }
            _ => abort(),
        }
    }
}
}

// ── `switch (val.type)` · json.c:254-291 · chunk 1332 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(val.type) {
//         case JSON_STR:
//         case JSON_EXCEPTION:
//             free(val.u.str);
//             break;
//         case JSON_INT:
//         case JSON_BOOL:
//         case JSON_NULL:
//         case JSON_UNDEFINED:
//             break;
//         case JSON_ARRAY:
//             {
//                 JSONArray *array = val.u.array;
//                 int i;
//     
//                 for(i = 0; i < array->len; i++) {
//                     json_free(array->tab[i]);
//                 }
//                 free(array);
//             }
//             break;
//         case JSON_OBJ:
//             {
//                 JSONObject *obj = val.u.obj;
//                 JSONProperty *f;
//                 int i;
//     
//                 for(i = 0; i < obj->len; i++) {
//                     f = &obj->props[i];
//                     json_free(f->name);
//                     json_free(f->value);
//                 }
//                 free(obj);
//             }
//             break;
//         default:
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1332 {
// Port of the json_free switch statement (json.c lines 254-291).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONUnion {
    pub str: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONUnion,
}

extern "C" {
    fn free(ptr: *mut c_void);
    fn abort() -> !;
    // Defined in the C translation unit; recursive call back into C.
    fn json_free(val: JSONValue);
}

const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_OBJ: c_int = 2;
const JSON_ARRAY: c_int = 3;
const JSON_BOOL: c_int = 4;
const JSON_NULL: c_int = 5;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

#[no_mangle]
pub unsafe extern "C" fn json_free__part1332(val: JSONValue) {
    let ty = val.r#type;
    if ty == JSON_STR || ty == JSON_EXCEPTION {
        free(val.u.str as *mut c_void);
    } else if ty == JSON_INT || ty == JSON_BOOL || ty == JSON_NULL || ty == JSON_UNDEFINED {
        // nothing to release
    } else if ty == JSON_ARRAY {
        let array: *mut JSONArray = val.u.array;
        let mut i: c_int = 0;
        while i < (*array).len {
            json_free(*(*array).tab.add(i as usize));
            i += 1;
        }
        free(array as *mut c_void);
    } else if ty == JSON_OBJ {
        let obj: *mut JSONObject = val.u.obj;
        let mut i: c_int = 0;
        while i < (*obj).len {
            let f: *mut JSONProperty = (*obj).props.add(i as usize);
            json_free((*f).name);
            json_free((*f).value);
            i += 1;
        }
        free(obj as *mut c_void);
    } else {
        abort();
    }
}
}

// ── `skip_spaces` · json.c:294-316 · chunk 1333 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void skip_spaces(const char **pp)
//     {
//         const char *p;
//         p = *pp;
//         for(;;) {
//             if (
//                ((*__ctype_b_loc ())[(int) ((
//                *p
//                ))] & (unsigned short int) _ISspace)
//                           ) {
//                 p++;
//             } else if (p[0] == '/' && p[1] == '/') {
//                 p += 2;
//                 while (*p != '\0' && *p != '\n')
//                     p++;
//             } else if (p[0] == '/' && p[1] == '*') {
//                 p += 2;
//                 while (*p != '\0' && (p[0] != '*' || p[1] != '/'))
//                     p++;
//                 if (*p != '\0')
//                     p += 2;
//             } else {
//                 break;
//             }
//         }
//         *pp = p;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1333 {
use core::ffi::{c_char, c_int};

// enum _ISspace from <ctype.h>: ((5) < 8 ? ((1 << 5) << 8) : ...)  == 8192
const IS_SPACE: u16 = (1u16 << 5) << 8;

extern "C" {
    fn __ctype_b_loc() -> *const *const u16;
}

#[no_mangle]
pub unsafe extern "C" fn skip_spaces(pp: *const *const c_char) {
    let mut p: *const c_char = *pp;
    loop {
        // C: (*__ctype_b_loc())[(int)(*p)] & _ISspace ; char is signed here
        let c: c_int = (*p as u8) as i8 as c_int;
        let table: *const u16 = *__ctype_b_loc();
        if (*table.offset(c as isize) & IS_SPACE) != 0 {
            p = p.add(1);
        } else if *p == b'/' as c_char && *p.add(1) == b'/' as c_char {
            p = p.add(2);
            while *p != 0 && *p != b'\n' as c_char {
                p = p.add(1);
            }
        } else if *p == b'/' as c_char && *p.add(1) == b'*' as c_char {
            p = p.add(2);
            while *p != 0 && (*p != b'*' as c_char || *p.add(1) != b'/' as c_char) {
                p = p.add(1);
            }
            if *p != 0 {
                p = p.add(2);
            }
        } else {
            break;
        }
    }
    *(pp as *mut *const c_char) = p;
}
}

// ── `const char *p; p = *pp;` · json.c:296-297 · chunk 1334 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const char *p;
//         p = *pp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1334 {
// Ported piece of skip_spaces: initialize the local pointer p from *pp.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn skip_spaces__part1334(pp: *const *const c_char) -> *const c_char {
    if pp.is_null() {
        return core::ptr::null();
    }
    unsafe { *pp }
}
}

// ── `for (; ; )` · json.c:298-314 · chunk 1335 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             if (
//                ((*__ctype_b_loc ())[(int) ((
//                *p
//                ))] & (unsigned short int) _ISspace)
//                           ) {
//                 p++;
//             } else if (p[0] == '/' && p[1] == '/') {
//                 p += 2;
//                 while (*p != '\0' && *p != '\n')
//                     p++;
//             } else if (p[0] == '/' && p[1] == '*') {
//                 p += 2;
//                 while (*p != '\0' && (p[0] != '*' || p[1] != '/'))
//                     p++;
//                 if (*p != '\0')
//                     p += 2;
//             } else {
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1335 {
use core::ffi::{c_char, c_int, c_ushort};

extern "C" {
    fn __ctype_b_loc() -> *mut *const c_ushort;
}

const ISSPACE: c_ushort = 0x2000; // _ISspace == (1 << 5) << 8

#[no_mangle]
pub extern "C" fn skip_spaces__part1335(pp: *mut *const c_char) {
    unsafe {
        let mut p: *const c_char = *pp;
        loop {
            let table: *const c_ushort = *__ctype_b_loc();
            let idx = *p as c_int as isize;
            let flags: c_ushort = *table.offset(idx);
            if (flags & ISSPACE) != 0 {
                p = p.offset(1);
            } else if *p == b'/' as c_char && *p.offset(1) == b'/' as c_char {
                p = p.offset(2);
                while *p != 0 && *p != b'\n' as c_char {
                    p = p.offset(1);
                }
            } else if *p == b'/' as c_char && *p.offset(1) == b'*' as c_char {
                p = p.offset(2);
                while *p != 0 && !(*p == b'*' as c_char && *p.offset(1) == b'/' as c_char) {
                    p = p.offset(1);
                }
                if *p != 0 {
                    p = p.offset(2);
                }
            } else {
                break;
            }
        }
        *pp = p;
    }
}
}

// ── `*pp = p;` · json.c:315-315 · chunk 1336 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1336 {
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn skip_spaces__part1336(pp: *mut *const c_char, p: *const c_char) {
    unsafe {
        *pp = p;
    }
}
}

// ── `is_ident_first` · json.c:318-323 · chunk 1337 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL is_ident_first(int c)
//     {
//         return (c >= 'a' && c <= 'z') ||
//             (c >= 'A' && c <= 'Z') ||
//             c == '_' || c == '$';
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1337 {
// Port of: static inline BOOL is_ident_first(int c)
// BOOL is typedef'd as int in the C program.

#[no_mangle]
pub extern "C" fn is_ident_first(c: core::ffi::c_int) -> core::ffi::c_int {
    let lc = c as u32;
    // 'a'..='z' or 'A'..='Z' or '_' or '$'
    let ok = (lc >= b'a' as u32 && lc <= b'z' as u32)
        || (lc >= b'A' as u32 && lc <= b'Z' as u32)
        || c == b'_' as core::ffi::c_int
        || c == b'$' as core::ffi::c_int;
    if ok {
        1
    } else {
        0
    }
}
}

// ── `return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' || c == '$';` · json.c:320-322 · chunk 1338 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return (c >= 'a' && c <= 'z') ||
//             (c >= 'A' && c <= 'Z') ||
//             c == '_' || c == '$';
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1338 {
// Port of the TARGET expression in is_ident_first (json.c).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn is_ident_first__part1338(c: c_int) -> c_int {
    let lower = c >= 0x61 && c <= 0x7a; // 'a' .. 'z'
    let upper = c >= 0x41 && c <= 0x5a; // 'A' .. 'Z'
    let res = lower || upper || c == 0x5f /* '_' */ || c == 0x24 /* '$' */;
    res as c_int
}
}

// ── `parse_ident` · json.c:325-340 · chunk 1339 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int parse_ident(char *buf, int buf_size, const char **pp)
//     {
//         char *q;
//         const char *p;
//         p = *pp;
//         q = buf;
//         *q++ = *p++;
//         while (is_ident_first(*p) || 
//                                     ((*__ctype_b_loc ())[(int) ((
//                                     *p
//                                     ))] & (unsigned short int) _ISdigit)
//                                                ) {
//             if ((q - buf) >= buf_size - 1)
//                 return -1;
//             *q++ = *p++;
//         }
//         *pp = p;
//         *q = '\0';
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1339 {
use core::ffi::{c_char, c_int};

#[inline]
fn is_ident_first(c: c_int) -> bool {
    let c = c as i32;
    (c >= 'a' as i32 && c <= 'z' as i32)
        || (c >= 'A' as i32 && c <= 'Z' as i32)
        || c == '_' as i32
        || c == '$' as i32
}

// Equivalent to glibc's isdigit() on the ctype table for a char promoted to int
// (entries for negative values are all zero in glibc's table).
#[inline]
fn is_digit(c: c_int) -> bool {
    let c = c as i32;
    c >= '0' as i32 && c <= '9' as i32
}

#[no_mangle]
pub unsafe extern "C" fn parse_ident(
    buf: *mut c_char,
    buf_size: c_int,
    pp: *mut *const c_char,
) -> c_int {
    let mut p: *const c_char = *pp;
    let mut q: *mut c_char = buf;

    // first char is already tested
    *q = *p;
    q = q.add(1);
    p = p.add(1);

    loop {
        let c = *p as c_int;
        if !(is_ident_first(c) || is_digit(c)) {
            break;
        }
        if (q as isize - buf as isize) >= (buf_size as isize - 1) {
            return -1;
        }
        *q = *p;
        q = q.add(1);
        p = p.add(1);
    }

    *pp = p;
    *q = 0;
    0
}
}

// ── `char *q; const char *p; p = *pp; q = buf; *q++ = *p++;` · json.c:327-331 · chunk 1340 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *q;
//         const char *p;
//         p = *pp;
//         q = buf;
//         *q++ = *p++;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1340 {
// Extracted piece of parse_ident: initializes the cursors and copies the
// first character of the identifier into the buffer.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn parse_ident__part1340(
    buf: *mut c_char,
    pp: *const *const c_char,
    p_out: *mut *const c_char,
    q_out: *mut *mut c_char,
) {
    unsafe {
        let mut p: *const c_char = *pp;
        let mut q: *mut c_char = buf;
        // *q++ = *p++;
        *q = *p;
        q = q.wrapping_add(1);
        p = p.wrapping_add(1);
        *p_out = p;
        *q_out = q;
    }
}
}

// ── `while (is_ident_first(*p) || ((*__ctype_b_loc ())[(int) (( *p ))] & (unsigned short int) _ISdig…` · json.c:332-336 · chunk 1341 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (is_ident_first(*p) || 
//                                     ((*__ctype_b_loc ())[(int) ((
//                                     *p
//                                     ))] & (unsigned short int) _ISdigit)
//                                                ) {
//             if ((q - buf) >= buf_size - 1)
//                 return -1;
//             *q++ = *p++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1341 {
// Port of the ident-body scanning loop of parse_ident (json.c lines 332-336).
// Ported private logic: the static inline is_ident_first has no linker symbol.
use core::ffi::{c_char, c_int};

extern "C" {
    // glibc char classification table accessor (declared `const unsigned short int **`).
    fn __ctype_b_loc() -> *mut *const u16;
}

// _ISdigit == ((3) < 8 ? ((1 << (3)) << 8) : ((1 << (3)) >> 8)) == 0x0800
const IS_DIGIT: c_int = 0x0800;

#[inline]
fn is_ident_first(c: c_int) -> bool {
    (c >= ('a' as c_int) && c <= ('z' as c_int))
        || (c >= ('A' as c_int) && c <= ('Z' as c_int))
        || c == ('_' as c_int)
        || c == ('$' as c_int)
}

// Replicates the preprocessed isdigit() macro:
// ((*__ctype_b_loc ())[(int)((*p))] & (unsigned short int) _ISdigit)
#[inline]
unsafe fn is_digit(c: c_int) -> bool {
    let tbl_ptr = __ctype_b_loc();
    if tbl_ptr.is_null() {
        return false;
    }
    let tbl = *tbl_ptr;
    if tbl.is_null() {
        return false;
    }
    let v = *tbl.offset(c as isize);
    ((v as c_int) & IS_DIGIT) != 0
}

#[no_mangle]
pub unsafe extern "C" fn parse_ident__part1341(
    buf: *mut c_char,
    buf_size: c_int,
    pp: *mut *const c_char,
    pq: *mut *mut c_char,
) -> c_int {
    let mut p: *const c_char = *pp;
    let mut q: *mut c_char = *pq;

    loop {
        let c: c_int = *p as c_int;
        if !(is_ident_first(c) || is_digit(c)) {
            break;
        }
        if (q as isize - buf as isize) >= (buf_size as isize - 1) {
            return -1;
        }
        *q = *p;
        q = q.add(1);
        p = p.add(1);
    }

    *pp = p;
    *pq = q;
    0
}
}

// ── `*pp = p; *q = '\0'; return 0;` · json.c:337-339 · chunk 1342 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p;
//         *q = '\0';
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1342 {
use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn parse_ident__part1342(
    pp: *mut *const c_char,
    p: *const c_char,
    q: *mut c_char,
) -> c_int {
    unsafe {
        *pp = p;
        *q = 0;
    }
    0
}
}

// ── `json_parse_value2` · json.c:342-438 · chunk 1343 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_parse_value2(const char **pp)
//     {
//         char buf[128];
//         const char *p;
//         JSONValue val, val1, tag;
//     
//         p = *pp;
//         skip_spaces(&p);
//         if (*p == '\0') {
//             return json_error_new("unexpected end of file");
//         }
//         if (
//            ((*__ctype_b_loc ())[(int) ((
//            *p
//            ))] & (unsigned short int) _ISdigit)
//                       ) {
//             val = json_int32_new(strtol(p, (char **)&p, 0));
//         } else if (*p == '"') {
//             val = parse_string(&p);
//         } else if (*p == '{') {
//             p++;
//             val = json_object_new();
//             for(;;) {
//                 skip_spaces(&p);
//                 if (*p == '}') {
//                     p++;
//                     break;
//                 }
//                 if (*p == '"') {
//                     tag = parse_string(&p);
//                     if (json_is_error(tag))
//                         return tag;
//                 } else if (is_ident_first(*p)) {
//                     if (parse_ident(buf, sizeof(buf), &p) < 0)
//                         goto invalid_prop;
//                     tag = json_string_new(buf);
//                 } else {
//                     goto invalid_prop;
//                 }
//     
//                 if (tag.u.str->len == 0) {
//                 invalid_prop:
//                     return json_error_new("Invalid property name");
//                 }
//                 skip_spaces(&p);
//                 if (*p != ':') {
//                     return json_error_new("':' expected");
//                 }
//                 p++;
//     
//                 val1 = json_parse_value2(&p);
//                 json_object_set(val, tag.u.str->data, val1);
//     
//                 skip_spaces(&p);
//                 if (*p == ',') {
//                     p++;
//                 } else if (*p != '}') {
//                     return json_error_new("expecting ',' or '}'");
//                 }
//             }
//         } else if (*p == '[') {
//             int idx;
//     
//             p++;
//             val = json_array_new();
//             idx = 0;
//             for(;;) {
//                 skip_spaces(&p);
//                 if (*p == ']') {
//                     p++;
//                     break;
//                 }
//                 val1 = json_parse_value2(&p);
//                 json_array_set(val, idx++, val1);
//     
//                 skip_spaces(&p);
//                 if (*p == ',') {
//                     p++;
//                 } else if (*p != ']') {
//                     return json_error_new("expecting ',' or ']'");
//                 }
//             }
//         } else if (is_ident_first(*p)) {
//             if (parse_ident(buf, sizeof(buf), &p) < 0)
//                 goto unknown_id;
//             if (!strcmp(buf, "null")) {
//                 val = json_null_new();
//             } else if (!strcmp(buf, "true")) {
//                 val = json_bool_new(TRUE);
//             } else if (!strcmp(buf, "false")) {
//                 val = json_bool_new(FALSE);
//             } else {
//             unknown_id:
//                 return json_error_new("unknown identifier: '%s'", buf);
//             }
//         } else {
//             return json_error_new("unexpected character");
//         }
//         *pp = p;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1343 {
// Port of json.c: json_parse_value2 (with its static helpers skip_spaces,
// is_ident_first, parse_ident, parse_string and the json.h static inline
// constructors ported in as private Rust functions).

use core::ffi::{c_char, c_int, c_long, c_uint, c_ushort};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JsonString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JsonValueU {
    pub str_: *mut JsonString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JsonValue {
    pub type_: c_int,
    pub u: JsonValueU,
}

const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_OBJ: c_int = 2;
const JSON_ARRAY: c_int = 3;
const JSON_BOOL: c_int = 4;
const JSON_NULL: c_int = 5;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

const ISDIGIT_MASK: c_ushort = 0x0800; /* _ISbit(3)  => _ISdigit */
const ISSPACE_MASK: c_ushort = 0x2000; /* _ISbit(5)  => _ISspace */

unsafe extern "C" {
    fn json_string_new(str_: *const c_char) -> JsonValue;
    fn json_error_new(fmt: *const c_char, ...) -> JsonValue;
    fn json_object_new() -> JsonValue;
    fn json_array_new() -> JsonValue;
    fn json_object_set(val: JsonValue, name: *const c_char, prop_val: JsonValue) -> c_int;
    fn json_array_set(val: JsonValue, idx: c_uint, prop_val: JsonValue) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn __ctype_b_loc() -> *const *const c_ushort;
}

static MSG_EOF: &[u8] = b"unexpected end of file\0";
static MSG_INVALID_PROP: &[u8] = b"Invalid property name\0";
static MSG_COLON: &[u8] = b"':' expected\0";
static MSG_COMMA_BRACE: &[u8] = b"expecting ',' or '}'\0";
static MSG_COMMA_BRACKET: &[u8] = b"expecting ',' or ']'\0";
static MSG_UNKNOWN_ID: &[u8] = b"unknown identifier: '%s'\0";
static MSG_UNEXPECTED_CHAR: &[u8] = b"unexpected character\0";
static MSG_UNTERMINATED: &[u8] = b"unterminated string\0";
static MSG_BAD_HEX: &[u8] = b"invalig hex digit\0";
static MSG_UNKNOWN_ESCAPE: &[u8] = b"unknown escape code\0";
static MSG_STRING_TOO_LONG: &[u8] = b"string too long\0";

#[inline]
unsafe fn ctype_test(c: c_int, mask: c_ushort) -> bool {
    let tbl_ptr = __ctype_b_loc();
    let tbl = *tbl_ptr;
    (*tbl.offset(c as isize) & mask) != 0
}

#[inline]
unsafe fn c_isdigit(c: c_int) -> bool {
    ctype_test(c, ISDIGIT_MASK)
}

#[inline]
unsafe fn c_isspace(c: c_int) -> bool {
    ctype_test(c, ISSPACE_MASK)
}

#[inline]
fn is_ident_first(c: c_int) -> bool {
    (c >= b'a' as c_int && c <= b'z' as c_int)
        || (c >= b'A' as c_int && c <= b'Z' as c_int)
        || c == b'_' as c_int
        || c == b'$' as c_int
}

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

/* ---- json.h static inline helpers ---- */

fn json_is_error(val: JsonValue) -> bool {
    val.type_ == JSON_EXCEPTION
}

fn json_int32_new(v: c_int) -> JsonValue {
    JsonValue {
        type_: JSON_INT,
        u: JsonValueU { int32: v },
    }
}

fn json_null_new() -> JsonValue {
    JsonValue {
        type_: JSON_NULL,
        u: JsonValueU { int32: 0 },
    }
}

fn json_bool_new(v: c_int) -> JsonValue {
    JsonValue {
        type_: JSON_BOOL,
        u: JsonValueU { b: v },
    }
}

#[inline]
unsafe fn json_string_len(s: *mut JsonString) -> c_int {
    (*s).len
}

#[inline]
unsafe fn json_string_data(s: *mut JsonString) -> *const c_char {
    core::ptr::addr_of!((*s).data) as *const c_char
}

/* ---- static void skip_spaces(const char **pp) ---- */

unsafe fn skip_spaces(pp: *mut *const c_char) {
    let mut p: *const c_char = *pp;
    loop {
        if c_isspace(*p as c_int) {
            p = p.add(1);
        } else if *p == b'/' as c_char && *p.add(1) == b'/' as c_char {
            p = p.add(2);
            while *p != 0 && *p != b'\n' as c_char {
                p = p.add(1);
            }
        } else if *p == b'/' as c_char && *p.add(1) == b'*' as c_char {
            p = p.add(2);
            while *p != 0 && !(*p == b'*' as c_char && *p.add(1) == b'/' as c_char) {
                p = p.add(1);
            }
            if *p != 0 {
                p = p.add(2);
            }
        } else {
            break;
        }
    }
    *pp = p;
}

/* ---- static int parse_ident(char *buf, int buf_size, const char **pp) ---- */

unsafe fn parse_ident(buf: *mut c_char, buf_size: c_int, pp: *mut *const c_char) -> c_int {
    let mut p: *const c_char = *pp;
    let mut q: isize = 0;
    *buf = *p; /* first char is already tested */
    p = p.add(1);
    q += 1;
    while is_ident_first(*p as c_int) || c_isdigit(*p as c_int) {
        if q >= (buf_size as isize) - 1 {
            return -1;
        }
        *buf.offset(q) = *p;
        q += 1;
        p = p.add(1);
    }
    *pp = p;
    *buf.offset(q) = 0;
    0
}

/* ---- static JSONValue parse_string(const char **pp) ---- */

unsafe fn parse_string(pp: *mut *const c_char) -> JsonValue {
    let mut buf = [0 as c_char; 4096];
    let mut q: usize = 0;
    let mut p: *const c_char = *pp;
    p = p.add(1);
    loop {
        let mut c: c_int = *p as c_int;
        p = p.add(1);
        if c == 0 || c == b'\n' as c_int {
            return json_error_new(MSG_UNTERMINATED.as_ptr() as *const c_char);
        } else if c == b'"' as c_int {
            break;
        } else {
            if c == b'\\' as c_int {
                c = *p as c_int;
                p = p.add(1);
                if c == b'\'' as c_int || c == b'"' as c_int || c == b'\\' as c_int {
                    /* add_char */
                } else if c == b'n' as c_int {
                    c = b'\n' as c_int;
                } else if c == b'r' as c_int {
                    c = b'\r' as c_int;
                } else if c == b't' as c_int {
                    c = b'\t' as c_int;
                } else if c == b'x' as c_int {
                    let h = from_hex(*p as c_int);
                    p = p.add(1);
                    if h < 0 {
                        return json_error_new(MSG_BAD_HEX.as_ptr() as *const c_char);
                    }
                    c = h << 4;
                    let h2 = from_hex(*p as c_int);
                    p = p.add(1);
                    if h2 < 0 {
                        return json_error_new(MSG_BAD_HEX.as_ptr() as *const c_char);
                    }
                    c |= h2;
                } else {
                    return json_error_new(MSG_UNKNOWN_ESCAPE.as_ptr() as *const c_char);
                }
            }
            /* add_char: */
            if q >= 4096 - 1 {
                return json_error_new(MSG_STRING_TOO_LONG.as_ptr() as *const c_char);
            }
            buf[q] = c as c_char;
            q += 1;
        }
    }
    buf[q] = 0;
    *pp = p;
    json_string_new(buf.as_ptr())
}

/* ---- JSONValue json_parse_value2(const char **pp) ---- */

#[no_mangle]
pub unsafe extern "C" fn json_parse_value2(pp: *mut *const c_char) -> JsonValue {
    let mut buf = [0 as c_char; 128];
    let mut p: *const c_char = *pp;

    skip_spaces(&mut p);
    if *p == 0 {
        return json_error_new(MSG_EOF.as_ptr() as *const c_char);
    }

    let val: JsonValue;
    if c_isdigit(*p as c_int) {
        let mut end: *mut c_char = core::ptr::null_mut();
        let v = strtol(p, &mut end, 0);
        p = end as *const c_char;
        val = json_int32_new(v as c_int);
    } else if *p == b'"' as c_char {
        val = parse_string(&mut p);
    } else if *p == b'{' as c_char {
        p = p.add(1);
        let obj = json_object_new();
        loop {
            skip_spaces(&mut p);
            if *p == b'}' as c_char {
                p = p.add(1);
                break;
            }
            let tag: JsonValue;
            if *p == b'"' as c_char {
                tag = parse_string(&mut p);
                if json_is_error(tag) {
                    return tag;
                }
            } else if is_ident_first(*p as c_int) {
                if parse_ident(buf.as_mut_ptr(), 128, &mut p) < 0 {
                    /* invalid_prop: */
                    return json_error_new(MSG_INVALID_PROP.as_ptr() as *const c_char);
                }
                tag = json_string_new(buf.as_ptr());
            } else {
                /* invalid_prop: */
                return json_error_new(MSG_INVALID_PROP.as_ptr() as *const c_char);
            }

            let tag_str: *mut JsonString = tag.u.str_;
            if json_string_len(tag_str) == 0 {
                return json_error_new(MSG_INVALID_PROP.as_ptr() as *const c_char);
            }
            skip_spaces(&mut p);
            if *p != b':' as c_char {
                return json_error_new(MSG_COLON.as_ptr() as *const c_char);
            }
            p = p.add(1);

            let val1: JsonValue = json_parse_value2(&mut p);
            json_object_set(obj, json_string_data(tag_str), val1);

            skip_spaces(&mut p);
            if *p == b',' as c_char {
                p = p.add(1);
            } else if *p != b'}' as c_char {
                return json_error_new(MSG_COMMA_BRACE.as_ptr() as *const c_char);
            }
        }
        val = obj;
    } else if *p == b'[' as c_char {
        p = p.add(1);
        let arr = json_array_new();
        let mut idx: c_int = 0;
        loop {
            skip_spaces(&mut p);
            if *p == b']' as c_char {
                p = p.add(1);
                break;
            }
            let val1: JsonValue = json_parse_value2(&mut p);
            let cur = idx;
            idx = idx.wrapping_add(1);
            json_array_set(arr, cur as c_uint, val1);

            skip_spaces(&mut p);
            if *p == b',' as c_char {
                p = p.add(1);
            } else if *p != b']' as c_char {
                return json_error_new(MSG_COMMA_BRACKET.as_ptr() as *const c_char);
            }
        }
        val = arr;
    } else if is_ident_first(*p as c_int) {
        if parse_ident(buf.as_mut_ptr(), 128, &mut p) < 0 {
            /* unknown_id: */
            return json_error_new(
                MSG_UNKNOWN_ID.as_ptr() as *const c_char,
                buf.as_ptr() as *const c_char,
            );
        }
        if strcmp(buf.as_ptr(), b"null\0".as_ptr() as *const c_char) == 0 {
            val = json_null_new();
        } else if strcmp(buf.as_ptr(), b"true\0".as_ptr() as *const c_char) == 0 {
            val = json_bool_new(1);
        } else if strcmp(buf.as_ptr(), b"false\0".as_ptr() as *const c_char) == 0 {
            val = json_bool_new(0);
        } else {
            /* unknown_id: */
            return json_error_new(
                MSG_UNKNOWN_ID.as_ptr() as *const c_char,
                buf.as_ptr() as *const c_char,
            );
        }
    } else {
        return json_error_new(MSG_UNEXPECTED_CHAR.as_ptr() as *const c_char);
    }
    *pp = p;
    val
}
}

// ── `char buf[128]; const char *p; JSONValue val, val1, tag; p = *pp; skip_spaces(&p);` · json.c:344-349 · chunk 1344 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char buf[128];
//         const char *p;
//         JSONValue val, val1, tag;
//     
//         p = *pp;
//         skip_spaces(&p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1344 {
use core::ffi::{c_char, c_int, c_ushort};

// glibc: _ISspace == (1 << 5) << 8
const IS_SPACE: c_ushort = (1u16 << 5) << 8;

extern "C" {
    fn __ctype_b_loc() -> *const *const c_ushort;
}

// Reads *pp and skips whitespace/comments, returning the resulting pointer
// (the caller still owns the pointer variable). Mirrors the C static
// `skip_spaces` helper, which has no linker symbol.
#[no_mangle]
pub unsafe extern "C" fn json_parse_value2__part1344(pp: *const *const c_char) -> *const c_char {
    let mut p: *const c_char = *pp;

    loop {
        let c: c_char = *p;
        let table: *const c_ushort = *__ctype_b_loc();
        let class: c_ushort = *table.offset(c as c_int as isize);
        if (class & IS_SPACE) != 0 {
            p = p.add(1);
        } else if *p == b'/' as c_char && *p.add(1) == b'/' as c_char {
            p = p.add(2);
            while *p != 0 && *p != b'\n' as c_char {
                p = p.add(1);
            }
        } else if *p == b'/' as c_char && *p.add(1) == b'*' as c_char {
            p = p.add(2);
            while *p != 0 && !(*p == b'*' as c_char && *p.add(1) == b'/' as c_char) {
                p = p.add(1);
            }
            if *p != 0 {
                p = p.add(2);
            }
        } else {
            break;
        }
    }

    p
}
}

// ── `if (*p == '\0')` · json.c:350-352 · chunk 1345 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p == '\0') {
//             return json_error_new("unexpected end of file");
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1345 {
// Port of the "unexpected end of file" check from json_parse_value2.
use core::ffi::{c_char, c_int, c_void};

// Mirror of the C union inside JSONValue.  Only size/alignment and the
// pointer/int members matter for the ABI.
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of the C type:
//   typedef struct JSONValue { JSONTypeEnum type; union {...} u; } JSONValue;
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_error_new(fmt: *const c_char, ...) -> JSONValue;
}

static MSG_UNEXPECTED_EOF: [u8; 23] = *b"unexpected end of file\0";

// Returns nonzero (and stores the error value into *out) when the current
// character is the terminating NUL, i.e. when the C code must `return`.
#[no_mangle]
pub extern "C" fn json_parse_value2__part1345(
    p: *const c_char,
    out: *mut JSONValue,
) -> c_int {
    unsafe {
        if p.is_null() || *p == 0 {
            let val = json_error_new(MSG_UNEXPECTED_EOF.as_ptr() as *const c_char);
            if !out.is_null() {
                *out = val;
            }
            return 1;
        }
    }
    0
}
}

// ── `if (*__ctype_b_loc ())[(int) (( *p ))] & (unsigned short int) _ISdigit) … else` · json.c:353-435 · chunk 1346 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            ((*__ctype_b_loc ())[(int) ((
//            *p
//            ))] & (unsigned short int) _ISdigit)
//                       ) {
//             val = json_int32_new(strtol(p, (char **)&p, 0));
//         } else if (*p == '"') {
//             val = parse_string(&p);
//         } else if (*p == '{') {
//             p++;
//             val = json_object_new();
//             for(;;) {
//                 skip_spaces(&p);
//                 if (*p == '}') {
//                     p++;
//                     break;
//                 }
//                 if (*p == '"') {
//                     tag = parse_string(&p);
//                     if (json_is_error(tag))
//                         return tag;
//                 } else if (is_ident_first(*p)) {
//                     if (parse_ident(buf, sizeof(buf), &p) < 0)
//                         goto invalid_prop;
//                     tag = json_string_new(buf);
//                 } else {
//                     goto invalid_prop;
//                 }
//     
//                 if (tag.u.str->len == 0) {
//                 invalid_prop:
//                     return json_error_new("Invalid property name");
//                 }
//                 skip_spaces(&p);
//                 if (*p != ':') {
//                     return json_error_new("':' expected");
//                 }
//                 p++;
//     
//                 val1 = json_parse_value2(&p);
//                 json_object_set(val, tag.u.str->data, val1);
//     
//                 skip_spaces(&p);
//                 if (*p == ',') {
//                     p++;
//                 } else if (*p != '}') {
//                     return json_error_new("expecting ',' or '}'");
//                 }
//             }
//         } else if (*p == '[') {
//             int idx;
//     
//             p++;
//             val = json_array_new();
//             idx = 0;
//             for(;;) {
//                 skip_spaces(&p);
//                 if (*p == ']') {
//                     p++;
//                     break;
//                 }
//                 val1 = json_parse_value2(&p);
//                 json_array_set(val, idx++, val1);
//     
//                 skip_spaces(&p);
//                 if (*p == ',') {
//                     p++;
//                 } else if (*p != ']') {
//                     return json_error_new("expecting ',' or ']'");
//                 }
//             }
//         } else if (is_ident_first(*p)) {
//             if (parse_ident(buf, sizeof(buf), &p) < 0)
//                 goto unknown_id;
//             if (!strcmp(buf, "null")) {
//                 val = json_null_new();
//             } else if (!strcmp(buf, "true")) {
//                 val = json_bool_new(TRUE);
//             } else if (!strcmp(buf, "false")) {
//                 val = json_bool_new(FALSE);
//             } else {
//             unknown_id:
//                 return json_error_new("unknown identifier: '%s'", buf);
//             }
//         } else {
//             return json_error_new("unexpected character");
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1346 {
// Port of part of json_parse_value2 (json.c): the top-level value dispatch
// (number / string / object / array / identifier / error).
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ushort};

// JSONTypeEnum values (typedef enum { JSON_STR, ... })
const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_OBJ: c_int = 2;
const JSON_ARRAY: c_int = 3;
const JSON_BOOL: c_int = 4;
const JSON_NULL: c_int = 5;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

// glibc ctype bit masks (<< 8)
const C_ISDIGIT: c_ushort = 0x0800;
const C_ISSPACE: c_ushort = 0x2000;

const ERR_UNTERMINATED: &[u8] = b"unterminated string\0";
const ERR_HEX: &[u8] = b"invalig hex digit\0";
const ERR_ESCAPE: &[u8] = b"unknown escape code\0";
const ERR_TOO_LONG: &[u8] = b"string too long\0";
const ERR_INVALID_PROP: &[u8] = b"Invalid property name\0";
const ERR_COLON: &[u8] = b"':' expected\0";
const ERR_OBJ_COMMA: &[u8] = b"expecting ',' or '}'\0";
const ERR_ARR_COMMA: &[u8] = b"expecting ',' or ']'\0";
const ERR_UNKNOWN_ID: &[u8] = b"unknown identifier: '%s'\0";
const ERR_UNEXPECTED: &[u8] = b"unexpected character\0";
const LIT_NULL: &[u8] = b"null\0";
const LIT_TRUE: &[u8] = b"true\0";
const LIT_FALSE: &[u8] = b"false\0";

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn __ctype_b_loc() -> *const *const c_ushort;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;

    fn json_string_new(str_: *const c_char) -> JSONValue;
    fn json_error_new(fmt: *const c_char, ...) -> JSONValue;
    fn json_object_new() -> JSONValue;
    fn json_object_set(val: JSONValue, name: *const c_char, prop_val: JSONValue) -> c_int;
    fn json_array_new() -> JSONValue;
    fn json_array_set(val: JSONValue, idx: c_uint, prop_val: JSONValue) -> c_int;
    fn json_parse_value2(pp: *mut *const c_char) -> JSONValue;
}

fn json_int32_new(v: c_int) -> JSONValue {
    JSONValue {
        type_: JSON_INT,
        u: JSONValueU { int32: v },
    }
}

fn json_bool_new(v: c_int) -> JSONValue {
    JSONValue {
        type_: JSON_BOOL,
        u: JSONValueU { b: v },
    }
}

fn json_null_new() -> JSONValue {
    JSONValue {
        type_: JSON_NULL,
        u: JSONValueU { int32: 0 },
    }
}

fn json_is_error(v: JSONValue) -> bool {
    v.type_ == JSON_EXCEPTION
}

unsafe fn ctype_flag(c: c_int, mask: c_ushort) -> bool {
    let tbl = __ctype_b_loc();
    let row = *tbl;
    ((*row.offset(c as isize)) & mask) != 0
}

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

fn is_ident_first(c: c_int) -> bool {
    (c >= b'a' as c_int && c <= b'z' as c_int)
        || (c >= b'A' as c_int && c <= b'Z' as c_int)
        || c == b'_' as c_int
        || c == b'$' as c_int
}

// static void skip_spaces(const char **pp)
unsafe fn skip_spaces(pp: *mut *const c_char) {
    let mut p: *const c_char = *pp;
    loop {
        if ctype_flag(*p as c_int, C_ISSPACE) {
            p = p.offset(1);
        } else if *p as c_int == b'/' as c_int && *p.offset(1) as c_int == b'/' as c_int {
            p = p.offset(2);
            while *p as c_int != 0 && *p as c_int != b'\n' as c_int {
                p = p.offset(1);
            }
        } else if *p as c_int == b'/' as c_int && *p.offset(1) as c_int == b'*' as c_int {
            p = p.offset(2);
            while *p as c_int != 0
                && !(*p as c_int == b'*' as c_int && *p.offset(1) as c_int == b'/' as c_int)
            {
                p = p.offset(1);
            }
            if *p as c_int != 0 {
                p = p.offset(2);
            }
        } else {
            break;
        }
    }
    *pp = p;
}

// static int parse_ident(char *buf, int buf_size, const char **pp)
unsafe fn parse_ident(buf: *mut c_char, buf_size: c_int, pp: *mut *const c_char) -> c_int {
    let mut p: *const c_char = *pp;
    let mut q: *mut c_char = buf;
    *q = *p;
    q = q.offset(1);
    p = p.offset(1);
    while is_ident_first(*p as c_int) || ctype_flag(*p as c_int, C_ISDIGIT) {
        if (q as isize - buf as isize) >= (buf_size as isize - 1) {
            return -1;
        }
        *q = *p;
        q = q.offset(1);
        p = p.offset(1);
    }
    *pp = p;
    *q = 0;
    0
}

// static JSONValue parse_string(const char **pp)
unsafe fn parse_string(pp: *mut *const c_char) -> JSONValue {
    let mut buf = [0 as c_char; 4096];
    let bptr = buf.as_mut_ptr();
    let mut q: usize = 0;
    let mut p: *const c_char = *pp;
    p = p.offset(1);
    loop {
        let mut c: c_int = *p as c_int;
        p = p.offset(1);
        if c == 0 || c == b'\n' as c_int {
            return json_error_new(ERR_UNTERMINATED.as_ptr() as *const c_char);
        }
        if c == b'"' as c_int {
            break;
        }
        if c == b'\\' as c_int {
            c = *p as c_int;
            p = p.offset(1);
            if c == b'\'' as c_int || c == b'"' as c_int || c == b'\\' as c_int {
                // character is kept as is
            } else if c == b'n' as c_int {
                c = b'\n' as c_int;
            } else if c == b'r' as c_int {
                c = b'\r' as c_int;
            } else if c == b't' as c_int {
                c = b'\t' as c_int;
            } else if c == b'x' as c_int {
                let mut h = from_hex(*p as c_int);
                p = p.offset(1);
                if h < 0 {
                    return json_error_new(ERR_HEX.as_ptr() as *const c_char);
                }
                c = h << 4;
                h = from_hex(*p as c_int);
                p = p.offset(1);
                if h < 0 {
                    return json_error_new(ERR_HEX.as_ptr() as *const c_char);
                }
                c |= h;
            } else {
                return json_error_new(ERR_ESCAPE.as_ptr() as *const c_char);
            }
        }
        if q >= 4095 {
            return json_error_new(ERR_TOO_LONG.as_ptr() as *const c_char);
        }
        *bptr.add(q) = c as c_char;
        q += 1;
    }
    *bptr.add(q) = 0;
    *pp = p;
    json_string_new(buf.as_ptr() as *const c_char)
}

#[no_mangle]
pub extern "C" fn json_parse_value2__part1346(
    pp: *mut *const c_char,
    buf: *mut c_char,
    pret: *mut c_int,
) -> JSONValue {
    unsafe {
        *pret = 0;
        let mut p: *const c_char = *pp;
        let val: JSONValue;

        if ctype_flag(*p as c_int, C_ISDIGIT) {
            let endp: *mut *const c_char = &mut p as *mut *const c_char;
            let n = strtol(p, endp as *mut *mut c_char, 0);
            val = json_int32_new(n as c_int);
        } else if *p as c_int == b'"' as c_int {
            val = parse_string(&mut p as *mut *const c_char);
        } else if *p as c_int == b'{' as c_int {
            p = p.offset(1);
            let obj: JSONValue = json_object_new();
            loop {
                skip_spaces(&mut p as *mut *const c_char);
                if *p as c_int == b'}' as c_int {
                    p = p.offset(1);
                    break;
                }
                let tag: JSONValue;
                if *p as c_int == b'"' as c_int {
                    let t = parse_string(&mut p as *mut *const c_char);
                    if json_is_error(t) {
                        *pret = 1;
                        return t;
                    }
                    tag = t;
                } else if is_ident_first(*p as c_int) {
                    if parse_ident(buf, 128, &mut p as *mut *const c_char) < 0 {
                        *pret = 1;
                        return json_error_new(ERR_INVALID_PROP.as_ptr() as *const c_char);
                    }
                    tag = json_string_new(buf as *const c_char);
                } else {
                    *pret = 1;
                    return json_error_new(ERR_INVALID_PROP.as_ptr() as *const c_char);
                }

                let s = tag.u.str_;
                if (*s).len == 0 {
                    *pret = 1;
                    return json_error_new(ERR_INVALID_PROP.as_ptr() as *const c_char);
                }
                skip_spaces(&mut p as *mut *const c_char);
                if *p as c_int != b':' as c_int {
                    *pret = 1;
                    return json_error_new(ERR_COLON.as_ptr() as *const c_char);
                }
                p = p.offset(1);

                let val1 = json_parse_value2(&mut p as *mut *const c_char);
                json_object_set(
                    obj,
                    core::ptr::addr_of!((*s).data) as *const c_char,
                    val1,
                );

                skip_spaces(&mut p as *mut *const c_char);
                if *p as c_int == b',' as c_int {
                    p = p.offset(1);
                } else if *p as c_int != b'}' as c_int {
                    *pret = 1;
                    return json_error_new(ERR_OBJ_COMMA.as_ptr() as *const c_char);
                }
            }
            val = obj;
        } else if *p as c_int == b'[' as c_int {
            p = p.offset(1);
            let arr: JSONValue = json_array_new();
            let mut idx: c_int = 0;
            loop {
                skip_spaces(&mut p as *mut *const c_char);
                if *p as c_int == b']' as c_int {
                    p = p.offset(1);
                    break;
                }
                let val1 = json_parse_value2(&mut p as *mut *const c_char);
                json_array_set(arr, idx as c_uint, val1);
                idx = idx.wrapping_add(1);

                skip_spaces(&mut p as *mut *const c_char);
                if *p as c_int == b',' as c_int {
                    p = p.offset(1);
                } else if *p as c_int != b']' as c_int {
                    *pret = 1;
                    return json_error_new(ERR_ARR_COMMA.as_ptr() as *const c_char);
                }
            }
            val = arr;
        } else if is_ident_first(*p as c_int) {
            if parse_ident(buf, 128, &mut p as *mut *const c_char) < 0 {
                *pret = 1;
                return json_error_new(
                    ERR_UNKNOWN_ID.as_ptr() as *const c_char,
                    buf as *const c_char,
                );
            }
            if strcmp(buf, LIT_NULL.as_ptr() as *const c_char) == 0 {
                val = json_null_new();
            } else if strcmp(buf, LIT_TRUE.as_ptr() as *const c_char) == 0 {
                val = json_bool_new(1);
            } else if strcmp(buf, LIT_FALSE.as_ptr() as *const c_char) == 0 {
                val = json_bool_new(0);
            } else {
                *pret = 1;
                return json_error_new(
                    ERR_UNKNOWN_ID.as_ptr() as *const c_char,
                    buf as *const c_char,
                );
            }
        } else {
            *pret = 1;
            return json_error_new(ERR_UNEXPECTED.as_ptr() as *const c_char);
        }

        *pp = p;
        val
    }
}
}

// ── `*pp = p; return val;` · json.c:436-437 · chunk 1347 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pp = p;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1347 {
// Port of the tail of json_parse_value2: store the advanced pointer back to
// *pp and return the parsed value.
use core::ffi::{c_char, c_int};

type BOOL = c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_parse_value2__part1347(
    pp: *mut *const c_char,
    p: *const c_char,
    val: JSONValue,
) -> JSONValue {
    unsafe {
        *pp = p;
    }
    val
}
}

// ── `json_parse_value` · json.c:440-452 · chunk 1348 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_parse_value(const char *p)
//     {
//         JSONValue val;
//         val = json_parse_value2(&p);
//         if (json_is_error(val))
//             return val;
//         skip_spaces(&p);
//         if (*p != '\0') {
//             json_free(val);
//             return json_error_new("unexpected characters at the end");
//         }
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1348 {
// Port of: JSONValue json_parse_value(const char *p)  (json.c)
use core::ffi::{c_char, c_int, c_void};

const JSON_EXCEPTION: c_int = 7;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int, /* BOOL == int */
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int, /* JSONTypeEnum */
    pub u: JSONValueU,
}

extern "C" {
    fn json_parse_value2(pp: *mut *const c_char) -> JSONValue;
    fn json_free(val: JSONValue);
    fn json_error_new(fmt: *const c_char, ...) -> JSONValue;
    fn isspace(c: c_int) -> c_int;
}

// static void skip_spaces(const char **pp)
unsafe fn skip_spaces(pp: *mut *const c_char) {
    let mut p = *pp;
    loop {
        if isspace(*p as c_int) != 0 {
            p = p.add(1);
        } else if *p == b'/' as c_char && *p.add(1) == b'/' as c_char {
            p = p.add(2);
            while *p != 0 && *p != b'\n' as c_char {
                p = p.add(1);
            }
        } else if *p == b'/' as c_char && *p.add(1) == b'*' as c_char {
            p = p.add(2);
            while *p != 0 && !(*p == b'*' as c_char && *p.add(1) == b'/' as c_char) {
                p = p.add(1);
            }
            if *p != 0 {
                p = p.add(2);
            }
        } else {
            break;
        }
    }
    *pp = p;
}

#[no_mangle]
pub extern "C" fn json_parse_value(p: *const c_char) -> JSONValue {
    let mut cur: *const c_char = p;
    let val: JSONValue = unsafe { json_parse_value2(&mut cur as *mut *const c_char) };
    if val.type_ == JSON_EXCEPTION {
        return val;
    }
    unsafe {
        skip_spaces(&mut cur as *mut *const c_char);
        if *cur != 0 {
            json_free(val);
            return json_error_new(b"unexpected characters at the end\0".as_ptr() as *const c_char);
        }
    }
    val
}
}

// ── `JSONValue val; val = json_parse_value2(&p);` · json.c:442-443 · chunk 1349 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val = json_parse_value2(&p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1349 {
// Portion of json_parse_value: parse the value from the string cursor.
use core::ffi::{c_char, c_int, c_void};

// Layout of JSONString is irrelevant here (only pointers are stored).
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_parse_value2(pp: *mut *const c_char) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn json_parse_value__part1349(p: *mut *const c_char) -> JSONValue {
    unsafe { json_parse_value2(p) }
}
}

// ── `if (json_is_error(val))` · json.c:444-445 · chunk 1350 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (json_is_error(val))
//             return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1350 {
// Port of the `if (json_is_error(val)) return val;` piece of json_parse_value.
// The Rust side only evaluates json_is_error(val) (val.type == JSON_EXCEPTION);
// the early return itself stays in the C caller.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::os::raw::c_int;

const JSON_EXCEPTION: c_int = 7;

// union u of struct JSONValue (all pointer / int members, 8 bytes, align 8)
#[repr(C)]
pub union JSONValue_u {
    pub str_: *mut core::ffi::c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

// typedef struct JSONValue { JSONTypeEnum type; union { ... } u; } JSONValue;
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValue_u,
}

#[no_mangle]
pub extern "C" fn json_parse_value__part1350(val: JSONValue) -> c_int {
    if val.type_ == JSON_EXCEPTION {
        1
    } else {
        0
    }
}
}

// ── `skip_spaces(&p);` · json.c:446-446 · chunk 1351 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     skip_spaces(&p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1351 {
// Port of the static C helper skip_spaces() logic, called from json_parse_value.
// Mirrors: isspace(*p) || "//" comment || "/* */" comment, skipping all of them.
use core::ffi::{c_char, c_int};

// glibc: _ISspace = (5 < 8 ? (1 << 5) << 8 : (1 << 5) >> 8)
const IS_SPACE: u16 = (1u16 << 5) << 8;

extern "C" {
    fn __ctype_b_loc() -> *mut *const u16;
}

#[no_mangle]
pub extern "C" fn json_parse_value__part1351(pp: *mut *const c_char) {
    unsafe {
        let mut p: *const c_char = *pp;
        let tbl: *const u16 = *__ctype_b_loc();
        loop {
            let c: c_int = *p as c_int;
            if *tbl.offset(c as isize) & IS_SPACE != 0 {
                p = p.offset(1);
            } else if *p == b'/' as c_char && *p.offset(1) == b'/' as c_char {
                p = p.offset(2);
                while *p != 0 && *p != b'\n' as c_char {
                    p = p.offset(1);
                }
            } else if *p == b'/' as c_char && *p.offset(1) == b'*' as c_char {
                p = p.offset(2);
                while *p != 0 && (*p != b'*' as c_char || *p.offset(1) != b'/' as c_char) {
                    p = p.offset(1);
                }
                if *p != 0 {
                    p = p.offset(2);
                }
            } else {
                break;
            }
        }
        *pp = p;
    }
}
}

// ── `if (*p != '\0')` · json.c:447-450 · chunk 1352 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (*p != '\0') {
//             json_free(val);
//             return json_error_new("unexpected characters at the end");
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1352 {
use core::ffi::{c_char, c_int, c_void};

// Mirror of the C `JSONValue` struct (enum JSONTypeEnum + union of pointers/int).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: *mut c_void,
}

extern "C" {
    fn json_free(val: JSONValue);
    fn json_error_new(fmt: *const c_char, ...) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn json_parse_value__part1352(
    p: *const c_char,
    val: JSONValue,
    ret: *mut JSONValue,
) -> c_int {
    unsafe {
        if *p != 0 {
            json_free(val);
            if !ret.is_null() {
                let msg = b"unexpected characters at the end\0";
                *ret = json_error_new(msg.as_ptr() as *const c_char);
            }
            return 1;
        }
    }
    0
}
}

// ── `return val;` · json.c:451-451 · chunk 1353 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1353 {
// Port of the tail `return val;` of json_parse_value (json.c line 451).
// Returns the JSONValue by value unchanged.

use core::ffi::{c_char, c_int};

type BOOL = c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    // CJSONTypeEnum in C is an enum; it has the representation of int.
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_parse_value__part1353(val: JSONValue) -> JSONValue {
    val
}
}

// ── `json_parse_value_len` · json.c:454-464 · chunk 1354 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue json_parse_value_len(const char *p, int len)
//     {
//         char *str;
//         JSONValue val;
//         str = malloc(len + 1);
//         memcpy(str, p, len);
//         str[len] = '\0';
//         val = json_parse_value(str);
//         free(str);
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1354 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn json_parse_value(p: *const c_char) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue {
    let str_: *mut c_char;
    let val: JSONValue;
    unsafe {
        str_ = malloc(len.wrapping_add(1) as usize) as *mut c_char;
        memcpy(str_ as *mut c_void, p as *const c_void, len as usize);
        *str_.offset(len as isize) = 0;
        val = json_parse_value(str_);
        free(str_ as *mut c_void);
    }
    val
}
}

// ── `char *str; JSONValue val; str = malloc(len + 1); memcpy(str, p, len); str[len] = '\0'; val = js…` · json.c:456-463 · chunk 1355 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *str;
//         JSONValue val;
//         str = malloc(len + 1);
//         memcpy(str, p, len);
//         str[len] = '\0';
//         val = json_parse_value(str);
//         free(str);
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1355 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn json_parse_value(p: *const c_char) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn json_parse_value_len__part1355(p: *const c_char, len: c_int) -> JSONValue {
    unsafe {
        let size = (len as usize).wrapping_add(1);
        let str_: *mut c_char = malloc(size) as *mut c_char;
        memcpy(str_ as *mut c_void, p as *const c_void, len as usize);
        *str_.add(len as usize) = 0;
        let val: JSONValue = json_parse_value(str_);
        free(str_ as *mut c_void);
        val
    }
}
}

