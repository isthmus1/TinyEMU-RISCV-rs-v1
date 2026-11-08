//! From `machine.c`.

// ── `vm_get_int` · machine.c:56-70 · chunk 1358 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int vm_get_int(JSONValue obj, const char *name, int *pval)
//     {
//         JSONValue val;
//         val = json_object_get(obj, name);
//         if (json_is_undefined(val)) {
//             vm_error("expecting '%s' property\n", name);
//             return -1;
//         }
//         if (val.type != JSON_INT) {
//             vm_error("%s: integer expected\n", name);
//             return -1;
//         }
//         *pval = val.u.int32;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1358 {
// Rust rewrite of int vm_get_int(JSONValue obj, const char *name, int *pval)
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int};

// Values of the C enum JSONTypeEnum (JSON_STR = 0, ...).
const JSON_INT: c_int = 1;
const JSON_UNDEFINED: c_int = 6;

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
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int, // BOOL is typedef int
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int, // JSONTypeEnum
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int {
    let val = unsafe { json_object_get(obj, name) };
    // json_is_undefined(val)
    if val.type_ == JSON_UNDEFINED {
        unsafe {
            vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }
    if val.type_ != JSON_INT {
        unsafe {
            vm_error(b"%s: integer expected\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }
    let int32 = unsafe { val.u.int32 };
    unsafe {
        *pval = int32;
    }
    0
}
}

// ── `JSONValue val; val = json_object_get(obj, name);` · machine.c:58-59 · chunk 1359 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val = json_object_get(obj, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1359 {
// Port of JSONValue lookup part of vm_get_int.

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueUnion {
    pub str_: *mut core::ffi::c_void,
    pub int32: core::ffi::c_int,
    pub b: core::ffi::c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: core::ffi::c_int,
    pub u: JSONValueUnion,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const core::ffi::c_char) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn vm_get_int__part1359(
    obj: JSONValue,
    name: *const core::ffi::c_char,
) -> JSONValue {
    unsafe { json_object_get(obj, name) }
}
}

// ── `if (json_is_undefined(val))` · machine.c:60-63 · chunk 1360 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (json_is_undefined(val)) {
//             vm_error("expecting '%s' property\n", name);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1360 {
// Port of the "undefined property" check of vm_get_int (machine.c).
use core::ffi::{c_char, c_int, c_uint, c_void};

// JSONTypeEnum: GCC gives an enum with no negative values the type unsigned int.
const JSON_UNDEFINED: c_uint = 6;

#[repr(C)]
pub union JsonValueUnion {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JsonValue {
    pub type_: c_uint,
    pub u: JsonValueUnion,
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn vm_get_int__part1360(val: JsonValue, name: *const c_char) -> c_int {
    unsafe {
        if val.type_ == JSON_UNDEFINED {
            vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
            return -1;
        }
    }
    0
}
}

// ── `if (val.type != JSON_INT)` · machine.c:64-67 · chunk 1361 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_INT) {
//             vm_error("%s: integer expected\n", name);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1361 {
// Port of the "integer type check" branch of vm_get_int().

use core::ffi::{c_char, c_int};

/// Mirror of the C union inside JSONValue (largest member: pointer / int).
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

/// Mirror of `typedef struct {} JSONString` (never dereferenced here).
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

/// Mirror of `typedef struct JSONValue`.
#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueU,
}

const JSON_INT: c_int = 1;

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

static FMT_INT_EXPECTED: &[u8] = b"%s: integer expected\n\0";

/// Returns 0 when `val` is a JSON integer, 1 when the C caller must
/// `return -1` (error message already printed).
#[no_mangle]
pub unsafe extern "C" fn vm_get_int__part1361(val: JSONValue, name: *const c_char) -> c_int {
    if val.r#type != JSON_INT {
        vm_error(FMT_INT_EXPECTED.as_ptr() as *const c_char, name);
        return 1;
    }
    0
}
}

// ── `*pval = val.u.int32; return 0;` · machine.c:68-69 · chunk 1362 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pval = val.u.int32;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1362 {
// Portion of vm_get_int: store the integer payload and report success.
use core::ffi::c_int;

// Mirror of C `JSONTypeEnum` (int-sized enum).
pub type JSONTypeEnum = c_int;

// Mirror of C `union { JSONString *str; int int32; BOOL b; ... }`.
#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut core::ffi::c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

// Mirror of C `struct JSONValue`.
#[repr(C)]
pub struct JSONValue {
    pub type_: JSONTypeEnum,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn vm_get_int__part1362(val: JSONValue, pval: *mut c_int) -> c_int {
    unsafe {
        *pval = val.u.int32;
    }
    0
}
}

// ── `vm_get_int_opt` · machine.c:72-86 · chunk 1363 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int vm_get_int_opt(JSONValue obj, const char *name, int *pval, int def_val)
//     {
//         JSONValue val;
//         val = json_object_get(obj, name);
//         if (json_is_undefined(val)) {
//             *pval = def_val;
//             return 0;
//         }
//         if (val.type != JSON_INT) {
//             vm_error("%s: integer expected\n", name);
//             return -1;
//         }
//         *pval = val.u.int32;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1363 {
// Rust rewrite of `vm_get_int_opt` from machine.c
// int vm_get_int_opt(JSONValue obj, const char *name, int *pval, int def_val)

use core::ffi::{c_char, c_int};

const JSON_INT: c_int = 1;
const JSON_UNDEFINED: c_int = 6;

// Mirrors `typedef int BOOL;`
type Bool = c_int;

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
pub union JSONValueUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: Bool,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
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

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

// Equivalent of the static inline `json_is_undefined`.
#[inline]
fn json_is_undefined(val: &JSONValue) -> bool {
    val.type_ == JSON_UNDEFINED
}

#[no_mangle]
pub unsafe extern "C" fn vm_get_int_opt(
    obj: JSONValue,
    name: *const c_char,
    pval: *mut c_int,
    def_val: c_int,
) -> c_int {
    let val = json_object_get(obj, name);
    if json_is_undefined(&val) {
        *pval = def_val;
        return 0;
    }
    if val.type_ != JSON_INT {
        vm_error(b"%s: integer expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    *pval = val.u.int32;
    0
}
}

// ── `JSONValue val; val = json_object_get(obj, name);` · machine.c:74-75 · chunk 1364 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val = json_object_get(obj, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1364 {
// Port of the JSONValue fetch step of vm_get_int_opt.
use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut JSONProperty,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut JSONValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
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
    pub r#type: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
}

// Silence unused-type warnings for fields we never touch directly.
const _: Option<*mut c_void> = None;

#[no_mangle]
pub extern "C" fn vm_get_int_opt__part1364(obj: JSONValue, name: *const c_char) -> JSONValue {
    unsafe { json_object_get(obj, name) }
}
}

// ── `if (json_is_undefined(val))` · machine.c:76-79 · chunk 1365 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (json_is_undefined(val)) {
//             *pval = def_val;
//             return 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1365 {
// Port of the `json_is_undefined(val)` early-out branch of vm_get_int_opt.
// Returns 1 when the branch was taken (caller must `return 0`), 0 otherwise.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

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

const JSON_UNDEFINED: c_int = 6;

#[no_mangle]
pub extern "C" fn vm_get_int_opt__part1365(
    val: *mut JSONValue,
    pval: *mut c_int,
    def_val: c_int,
) -> c_int {
    let is_undefined = unsafe { (*val).type_ } == JSON_UNDEFINED;
    if is_undefined {
        unsafe {
            *pval = def_val;
        }
        return 1;
    }
    0
}
}

// ── `if (val.type != JSON_INT)` · machine.c:80-83 · chunk 1366 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_INT) {
//             vm_error("%s: integer expected\n", name);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1366 {
// Ported piece of vm_get_int_opt: the "integer expected" error branch.
use core::ffi::{c_char, c_int};

// Value of JSON_INT in the C enum JSONTypeEnum (JSON_STR = 0, JSON_INT = 1).
const JSON_INT: c_int = 1;

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn vm_get_int_opt__part1366(val_type: c_int, name: *const c_char) -> c_int {
    if val_type != JSON_INT {
        unsafe {
            vm_error(b"%s: integer expected\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }
    0
}
}

// ── `*pval = val.u.int32; return 0;` · machine.c:84-85 · chunk 1367 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pval = val.u.int32;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1367 {
// Port of the tail of vm_get_int_opt: store the integer value and report success.
use core::ffi::{c_char, c_int};

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

#[no_mangle]
pub extern "C" fn vm_get_int_opt__part1367(val: JSONValue, pval: *mut c_int) -> c_int {
    let v = unsafe { val.u.int32 };
    unsafe {
        *pval = v;
    }
    0
}
}

// ── `vm_get_str2` · machine.c:88-108 · chunk 1368 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int vm_get_str2(JSONValue obj, const char *name, const char **pstr,
//                           BOOL is_opt)
//     {
//         JSONValue val;
//         val = json_object_get(obj, name);
//         if (json_is_undefined(val)) {
//             if (is_opt) {
//                 *pstr = 
//                        ((void *)0)
//                            ;
//                 return 0;
//             } else {
//                 vm_error("expecting '%s' property\n", name);
//                 return -1;
//             }
//         }
//         if (val.type != JSON_STR) {
//             vm_error("%s: string expected\n", name);
//             return -1;
//         }
//         *pstr = val.u.str->data;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1368 {
// Rewrite of static int vm_get_str2(JSONValue obj, const char *name,
//                                    const char **pstr, BOOL is_opt)
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

// JSONTypeEnum values (C enum is int)
const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    let val: JSONValue = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = core::ptr::null();
            return 0;
        } else {
            vm_error(
                b"expecting '%s' property\n\0".as_ptr() as *const c_char,
                name,
            );
            return -1;
        }
    }
    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    *pstr = (*val.u.str_).data.as_ptr();
    return 0;
}
}

// ── `JSONValue val; val = json_object_get(obj, name);` · machine.c:91-92 · chunk 1369 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val = json_object_get(obj, name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1369 {
// Portion of vm_get_str2: look up the named property in the JSON object.
use core::ffi::{c_char, c_int, c_void};

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
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn vm_get_str2__part1369(
    obj: JSONValue,
    name: *const c_char,
    _pstr: *mut *const c_char,
    _is_opt: c_int,
) -> JSONValue {
    unsafe { json_object_get(obj, name) }
}
}

// ── `if (json_is_undefined(val))` · machine.c:93-101 · chunk 1370 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (json_is_undefined(val)) {
//             if (is_opt) {
//                 *pstr = 
//                        ((void *)0)
//                            ;
//                 return 0;
//             } else {
//                 vm_error("expecting '%s' property\n", name);
//                 return -1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1370 {
// Portion of vm_get_str2(): handles the "property is undefined" case.
use core::ffi::{c_char, c_int, c_void};

// JSONTypeEnum is a C enum (int-sized). The C union holds an 8-byte pointer
// at offset 8, so the whole JSONValue is 16 bytes with 8-byte alignment.
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: *mut c_void,
}

const JSON_UNDEFINED: c_int = 6;

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

// Returns 0 or -1 when it fully handled the undefined case (caller must
// return that value), or 1 when the caller should continue normally.
#[no_mangle]
pub extern "C" fn vm_get_str2__part1370(
    val: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            unsafe {
                *pstr = core::ptr::null();
            }
            0
        } else {
            unsafe {
                vm_error(
                    b"expecting '%s' property\n\0".as_ptr() as *const c_char,
                    name,
                );
            }
            -1
        }
    } else {
        1
    }
}
}

// ── `if (val.type != JSON_STR)` · machine.c:102-105 · chunk 1371 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (val.type != JSON_STR) {
//             vm_error("%s: string expected\n", name);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1371 {
// Port of vm_get_str2's "not a string" check (machine.c lines 102-105).
use core::ffi::{c_char, c_int};

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0];
}

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int, // BOOL == int
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int, // JSONTypeEnum
    pub u: JSONValueUnion,
}

// JSON_STR == 0 (first enumerator of JSONTypeEnum)
const JSON_STR: c_int = 0;

/// Returns -1 (after reporting the error) when `val` is not a JSON string,
/// 0 when the caller may continue.
#[no_mangle]
pub unsafe extern "C" fn vm_get_str2__part1371(val: JSONValue, name: *const c_char) -> c_int {
    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    0
}
}

// ── `*pstr = val.u.str->data; return 0;` · machine.c:106-107 · chunk 1372 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pstr = val.u.str->data;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1372 {
use core::ffi::{c_char, c_int};

// Mirror of C's JSONString: int len; char data[0];
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
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

// Mirror of C's JSONValue: JSONTypeEnum type; union { ... } u;
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

/// Ported piece of vm_get_str2:
///     *pstr = val.u.str->data;
///     return 0;
#[no_mangle]
pub unsafe extern "C" fn vm_get_str2__part1372(
    val: JSONValue,
    pstr: *mut *const c_char,
) -> c_int {
    let s = val.u.str_;
    let data = core::ptr::addr_of!((*s).data) as *const c_char;
    *pstr = data;
    0
}
}

// ── `vm_get_str` · machine.c:110-113 · chunk 1373 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static int vm_get_str(JSONValue obj, const char *name, const char **pstr)
//     {
//         return vm_get_str2(obj, name, pstr, FALSE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1373 {
// Port of the static C helper vm_get_str (which forwards to vm_get_str2).
use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;
const FALSE: BOOL = 0;

type JSONTypeEnum = c_int;
const JSON_STR: JSONTypeEnum = 0;
const JSON_UNDEFINED: JSONTypeEnum = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: JSONTypeEnum,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: BOOL,
) -> c_int {
    let val: JSONValue = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = core::ptr::null();
            return 0;
        } else {
            vm_error(
                b"expecting '%s' property\n\0".as_ptr() as *const c_char,
                name,
            );
            return -1;
        }
    }
    if val.type_ != JSON_STR {
        vm_error(
            b"%s: string expected\n\0".as_ptr() as *const c_char,
            name,
        );
        return -1;
    }
    let s: *mut JSONString = val.u.str_;
    *pstr = (*s).data.as_ptr();
    0
}

#[no_mangle]
pub extern "C" fn vm_get_str(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    unsafe { vm_get_str2(obj, name, pstr, FALSE) }
}
}

// ── `return vm_get_str2(obj, name, pstr, FALSE);` · machine.c:112-112 · chunk 1374 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vm_get_str2(obj, name, pstr, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1374 {
// Port of the body of the static C function `vm_get_str2(obj, name, pstr, FALSE)`
// plus the target statement `return vm_get_str2(obj, name, pstr, FALSE);`
// of vm_get_str.

use core::ffi::{c_char, c_int};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn vm_get_str__part1374(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    let val = json_object_get(obj, name);

    // json_is_undefined(val)
    if val.type_ == JSON_UNDEFINED {
        // is_opt == FALSE branch
        vm_error(
            b"expecting '%s' property\n\0".as_ptr() as *const c_char,
            name,
        );
        return -1;
    }

    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }

    // *pstr = val.u.str->data;
    let s = val.u.str_;
    *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
    0
}
}

// ── `vm_get_str_opt` · machine.c:115-118 · chunk 1375 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int vm_get_str_opt(JSONValue obj, const char *name, const char **pstr)
//     {
//         return vm_get_str2(obj, name, pstr, TRUE);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1375 {
// Port of machine.c's static helper vm_get_str_opt.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueU {
    pub str: *mut JSONString,
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
const JSON_UNDEFINED: c_int = 6;

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn vm_get_str_opt(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    let val = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        // is_opt == TRUE
        *pstr = core::ptr::null();
        return 0;
    }
    if val.type_ != JSON_STR {
        vm_error(
            b"%s: string expected\n\0".as_ptr() as *const c_char,
            name,
        );
        return -1;
    }
    let s = val.u.str;
    if s.is_null() {
        *pstr = core::ptr::null();
    } else {
        *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
    }
    0
}
}

// ── `return vm_get_str2(obj, name, pstr, TRUE);` · machine.c:117-117 · chunk 1376 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return vm_get_str2(obj, name, pstr, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1376 {
// Port of the TARGET `return vm_get_str2(obj, name, pstr, TRUE);`
// vm_get_str2 is `static` in the C file (no linker symbol), so its body is
// inlined here with is_opt == TRUE.

use core::ffi::{c_char, c_int};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

static MSG_STRING_EXPECTED: &[u8] = b"%s: string expected\n\0";

#[no_mangle]
pub extern "C" fn vm_get_str_opt__part1376(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    unsafe {
        let val = json_object_get(obj, name);
        if val.type_ == JSON_UNDEFINED {
            // is_opt == TRUE
            *pstr = core::ptr::null();
            return 0;
        }
        if val.type_ != JSON_STR {
            vm_error(MSG_STRING_EXPECTED.as_ptr() as *const c_char, name);
            return -1;
        }
        let s = val.u.str_;
        *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
        0
    }
}
}

// ── `strdup_null` · machine.c:120-126 · chunk 1377 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static char *strdup_null(const char *str)
//     {
//         if (!str)
//             return 
//                   ((void *)0)
//                       ;
//         else
//             return strdup(str);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1377 {
use core::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn strdup_null(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { strdup(s) }
    }
}
}

// ── `if (!str) … else` · machine.c:122-125 · chunk 1378 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!str)
//             return 
//                   ((void *)0)
//                       ;
//         else
//             return strdup(str);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1378 {
// Port of the strdup_null body: return NULL for a NULL input, else strdup(str).
use core::ffi::c_char;

extern "C" {
    fn strdup(__s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn strdup_null__part1378(str: *const c_char) -> *mut c_char {
    if str.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { strdup(str) }
    }
}
}

// ── `cmdline_subst` · machine.c:129-172 · chunk 1379 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static char *cmdline_subst(const char *cmdline)
//     {
//         DynBuf dbuf;
//         const char *p;
//         char var_name[32], *q, buf[32];
//     
//         dbuf_init(&dbuf);
//         p = cmdline;
//         while (*p != '\0') {
//             if (p[0] == '$' && p[1] == '{') {
//                 p += 2;
//                 q = var_name;
//                 while (*p != '\0' && *p != '}') {
//                     if ((q - var_name) < sizeof(var_name) - 1)
//                         *q++ = *p;
//                     p++;
//                 }
//                 *q = '\0';
//                 if (*p == '}')
//                     p++;
//                 if (!strcmp(var_name, "TZ")) {
//                     time_t ti;
//                     struct tm tm;
//                     int n, sg;
//     
//                     time(&ti);
//                     localtime_r(&ti, &tm);
//                     n = tm.tm_gmtoff / 60;
//                     sg = '-';
//                     if (n < 0) {
//                         sg = '+';
//                         n = -n;
//                     }
//                     snprintf(buf, sizeof(buf), "UTC%c%02d:%02d",
//                              sg, n / 60, n % 60);
//                     dbuf_putstr(&dbuf, buf);
//                 }
//             } else {
//                 dbuf_putc(&dbuf, *p++);
//             }
//         }
//         dbuf_putc(&dbuf, 0);
//         return (char *)dbuf.buf;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1379 {
use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

extern "C" {
    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, string: *const c_char);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn time(t: *mut c_long) -> c_long;
    fn localtime_r(t: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn snprintf(buf: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    unsafe {
        let mut dbuf = DynBuf {
            buf: core::ptr::null_mut(),
            size: 0,
            allocated_size: 0,
        };

        dbuf_init(&mut dbuf);

        let mut p = cmdline;
        while *p != 0 {
            if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
                p = p.add(2);
                let mut var_name = [0 as c_char; 32];
                let mut q: usize = 0;
                while *p != 0 && *p != b'}' as c_char {
                    if q < 32 - 1 {
                        var_name[q] = *p;
                        q += 1;
                    }
                    p = p.add(1);
                }
                var_name[q] = 0;
                if *p == b'}' as c_char {
                    p = p.add(1);
                }
                if strcmp(var_name.as_ptr(), b"TZ\0".as_ptr() as *const c_char) == 0 {
                    let mut ti: c_long = 0;
                    let mut tm: Tm = core::mem::zeroed();
                    // get the offset to UTC
                    time(&mut ti);
                    localtime_r(&ti, &mut tm);
                    let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                    let mut sg: c_char = b'-' as c_char;
                    if n < 0 {
                        sg = b'+' as c_char;
                        n = n.wrapping_neg();
                    }
                    let mut buf = [0 as c_char; 32];
                    snprintf(
                        buf.as_mut_ptr(),
                        32,
                        b"UTC%c%02d:%02d\0".as_ptr() as *const c_char,
                        sg as c_int,
                        n / 60,
                        n % 60,
                    );
                    dbuf_putstr(&mut dbuf, buf.as_ptr());
                }
            } else {
                dbuf_putc(&mut dbuf, *p as u8);
                p = p.add(1);
            }
        }
        dbuf_putc(&mut dbuf, 0);
        dbuf.buf as *mut c_char
    }
}
}

// ── `DynBuf dbuf; const char *p; char var_name[32], *q, buf[32]; dbuf_init(&dbuf); p = cmdline;` · machine.c:131-136 · chunk 1380 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     DynBuf dbuf;
//         const char *p;
//         char var_name[32], *q, buf[32];
//     
//         dbuf_init(&dbuf);
//         p = cmdline;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1380 {
// Rust port of the declaration/initialisation prologue of cmdline_subst.
use core::ffi::c_char;

/// Mirror of cutils.h's DynBuf.
#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

extern "C" {
    // void dbuf_init(DynBuf *s);
    fn dbuf_init(s: *mut DynBuf);
}

#[no_mangle]
pub extern "C" fn cmdline_subst__part1380(
    dbuf: *mut DynBuf,
    cmdline: *const c_char,
) -> *const c_char {
    unsafe {
        dbuf_init(dbuf);
    }
    cmdline
}
}

// ── `while (*p != '\0')` · machine.c:137-169 · chunk 1381 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (*p != '\0') {
//             if (p[0] == '$' && p[1] == '{') {
//                 p += 2;
//                 q = var_name;
//                 while (*p != '\0' && *p != '}') {
//                     if ((q - var_name) < sizeof(var_name) - 1)
//                         *q++ = *p;
//                     p++;
//                 }
//                 *q = '\0';
//                 if (*p == '}')
//                     p++;
//                 if (!strcmp(var_name, "TZ")) {
//                     time_t ti;
//                     struct tm tm;
//                     int n, sg;
//     
//                     time(&ti);
//                     localtime_r(&ti, &tm);
//                     n = tm.tm_gmtoff / 60;
//                     sg = '-';
//                     if (n < 0) {
//                         sg = '+';
//                         n = -n;
//                     }
//                     snprintf(buf, sizeof(buf), "UTC%c%02d:%02d",
//                              sg, n / 60, n % 60);
//                     dbuf_putstr(&dbuf, buf);
//                 }
//             } else {
//                 dbuf_putc(&dbuf, *p++);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1381 {
// Rust port of the `${...}` substitution loop from cmdline_subst().

use core::ffi::{c_char, c_int, c_long};

// Mirrors the C DynBuf from util/cutils.h
#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

// Mirrors the glibc `struct tm` visible in this translation unit.
#[repr(C)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

extern "C" {
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, str_: *const c_char);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn time(timer: *mut c_long) -> c_long;
    fn localtime_r(timer: *const c_long, tp: *mut Tm) -> *mut Tm;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn cmdline_subst__part1381(mut p: *const c_char, dbuf: *mut DynBuf) {
    let mut var_name = [0i8; 32];
    let mut buf = [0i8; 32];

    unsafe {
        while *p != 0 {
            if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
                p = p.add(2);
                // q = var_name; (index form of the pointer)
                let mut qi: usize = 0;
                while *p != 0 && *p != b'}' as c_char {
                    if qi < 32 - 1 {
                        var_name[qi] = *p;
                        qi += 1;
                    }
                    p = p.add(1);
                }
                var_name[qi] = 0;
                if *p == b'}' as c_char {
                    p = p.add(1);
                }
                if strcmp(var_name.as_ptr(), b"TZ\0".as_ptr() as *const c_char) == 0 {
                    let mut ti: c_long = 0;
                    let mut tm: Tm = core::mem::zeroed();

                    time(&mut ti);
                    localtime_r(&ti, &mut tm);
                    let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                    let mut sg: c_int = b'-' as c_int;
                    if n < 0 {
                        sg = b'+' as c_int;
                        n = -n;
                    }
                    snprintf(
                        buf.as_mut_ptr(),
                        32,
                        b"UTC%c%02d:%02d\0".as_ptr() as *const c_char,
                        sg,
                        n / 60,
                        n % 60,
                    );
                    dbuf_putstr(dbuf, buf.as_ptr());
                }
            } else {
                dbuf_putc(dbuf, *p as u8);
                p = p.add(1);
            }
        }
    }
}
}

// ── `dbuf_putc(&dbuf, 0); return (char *)dbuf.buf;` · machine.c:170-171 · chunk 1382 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     dbuf_putc(&dbuf, 0);
//         return (char *)dbuf.buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1382 {
// Port of the tail of cmdline_subst: terminate the DynBuf with a NUL byte
// and return the underlying buffer.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

extern "C" {
    fn dbuf_putc(s: *mut DynBuf, c: u8);
}

#[no_mangle]
pub extern "C" fn cmdline_subst__part1382(dbuf: *mut DynBuf) -> *mut c_char {
    unsafe {
        dbuf_putc(dbuf, 0u8);
        (*dbuf).buf as *mut c_char
    }
}

// keep c_int referenced so the import is never considered unused by tools
#[allow(dead_code)]
const _UNUSED: Option<c_int> = None;
}

// ── `find_name` · machine.c:174-194 · chunk 1383 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static BOOL find_name(const char *name, const char *name_list)
//     {
//         size_t len;
//         const char *p, *r;
//     
//         p = name_list;
//         for(;;) {
//             r = 
//                _Generic (0 ? (
//                p
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                p
//                , 
//                ','
//                )), default: strchr (
//                p
//                , 
//                ','
//                ))
//                              ;
//             if (!r) {
//                 if (!strcmp(name, p))
//                     return TRUE;
//                 break;
//             } else {
//                 len = r - p;
//                 if (len == strlen(name) && !memcmp(name, p, len))
//                     return TRUE;
//                 p = r + 1;
//             }
//         }
//         return FALSE;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1383 {
use core::ffi::{c_char, c_int};

unsafe fn byte_at(p: *const c_char, i: usize) -> u8 {
    *p.add(i) as u8
}

unsafe fn c_str_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: usize = 0;
    loop {
        let ca = byte_at(a, i);
        let cb = byte_at(b, i);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}

unsafe fn c_str_len(s: *const c_char) -> usize {
    let mut i: usize = 0;
    while byte_at(s, i) != 0 {
        i += 1;
    }
    i
}

unsafe fn mem_eq(a: *const c_char, b: *const c_char, n: usize) -> bool {
    let mut i: usize = 0;
    while i < n {
        if byte_at(a, i) != byte_at(b, i) {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub extern "C" fn find_name(name: *const c_char, name_list: *const c_char) -> c_int {
    unsafe {
        let mut p: *const c_char = name_list;
        loop {
            // r = strchr(p, ',')
            let mut r: *const c_char = core::ptr::null();
            let mut q: *const c_char = p;
            while *q != 0 {
                if (*q as u8) == b',' {
                    r = q;
                    break;
                }
                q = q.add(1);
            }
            if r.is_null() {
                if c_str_eq(name, p) {
                    return 1;
                }
                return 0;
            } else {
                let len: usize = (r as usize).wrapping_sub(p as usize);
                if len == c_str_len(name) && mem_eq(name, p, len) {
                    return 1;
                }
                p = r.add(1);
            }
        }
    }
}
}

// ── `size_t len; const char *p, *r; p = name_list;` · machine.c:176-179 · chunk 1384 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     size_t len;
//         const char *p, *r;
//     
//         p = name_list;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1384 {
// Initialization part of find_name(): sets up the scan pointer from the list.
use core::ffi::c_char;

#[no_mangle]
pub extern "C" fn find_name__part1384(name_list: *const c_char) -> *const c_char {
    name_list
}
}

// ── `for (; ; )` · machine.c:180-192 · chunk 1385 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             r = 
//                _Generic (0 ? (
//                p
//                ) : (void *) 1, const void *: (const char *) (strchr (
//                p
//                , 
//                ','
//                )), default: strchr (
//                p
//                , 
//                ','
//                ))
//                              ;
//             if (!r) {
//                 if (!strcmp(name, p))
//                     return TRUE;
//                 break;
//             } else {
//                 len = r - p;
//                 if (len == strlen(name) && !memcmp(name, p, len))
//                     return TRUE;
//                 p = r + 1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1385 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

// Walks the comma separated name list starting at `p_start`.
// Returns 1 (TRUE) if `name` matches an entry, 0 if no match was found.
#[no_mangle]
pub extern "C" fn find_name__part1385(name: *const c_char, p_start: *const c_char) -> c_int {
    let mut p = p_start;
    loop {
        let r = unsafe { strchr(p, b',' as c_int) };
        if r.is_null() {
            if unsafe { strcmp(name, p) } == 0 {
                return 1;
            }
            return 0;
        } else {
            let len = (r as usize).wrapping_sub(p as usize);
            if len == unsafe { strlen(name) } && unsafe { memcmp(name, p, len) } == 0 {
                return 1;
            }
            p = r.wrapping_add(1);
        }
    }
}
}

// ── `return FALSE;` · machine.c:193-193 · chunk 1386 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return FALSE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1386 {
// Portion of find_name: the fall-through result when no entry in the
// comma separated name_list matches (original C: `return FALSE;`).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn find_name__part1386() -> c_int {
    // FALSE == 0 (BOOL is typedef int)
    0
}
}

// ── `virt_machine_find_class` · machine.c:211-221 · chunk 1388 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static const VirtMachineClass *virt_machine_find_class(const char *machine_name)
//     {
//         const VirtMachineClass *vmc, **pvmc;
//     
//         for(pvmc = virt_machine_list; *pvmc != 
//                                               ((void *)0)
//                                                   ; pvmc++) {
//             vmc = *pvmc;
//             if (find_name(machine_name, vmc->machine_names))
//                 return vmc;
//         }
//         return 
//               ((void *)0)
//                   ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1388 {
// Port of machine.c: virt_machine_find_class (with its static helper find_name inlined).

use core::ffi::{c_char, c_int};

type BOOL = c_int;

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: *const core::ffi::c_void,
    pub virt_machine_init: *const core::ffi::c_void,
    pub virt_machine_end: *const core::ffi::c_void,
    pub virt_machine_get_sleep_duration: *const core::ffi::c_void,
    pub virt_machine_interp: *const core::ffi::c_void,
    pub vm_mouse_is_absolute: *const core::ffi::c_void,
    pub vm_send_mouse_event: *const core::ffi::c_void,
    pub vm_send_key_event: *const core::ffi::c_void,
}

extern "C" {
    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;
}

// The entries of the file-static machine list depending on the build config,
// as seen in the preprocessed translation unit: riscv first, then pc, then NULL.
unsafe fn list_at(i: usize) -> *const VirtMachineClass {
    match i {
        0 => &riscv_machine_class as *const VirtMachineClass,
        1 => &pc_machine_class as *const VirtMachineClass,
        _ => core::ptr::null(),
    }
}

// strcmp(a, b) == 0
unsafe fn str_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: usize = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}

unsafe fn c_strlen(s: *const c_char) -> usize {
    let mut i: usize = 0;
    while *s.add(i) != 0 {
        i += 1;
    }
    i
}

// memcmp(a, b, len) == 0
unsafe fn mem_eq(a: *const c_char, b: *const c_char, len: usize) -> bool {
    let mut i: usize = 0;
    while i < len {
        if *a.add(i) != *b.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

// static BOOL find_name(const char *name, const char *name_list)
unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> BOOL {
    let mut p = name_list;
    loop {
        // r = strchr(p, ',')
        let mut r = p;
        while *r != 0 && *r != b',' as c_char {
            r = r.add(1);
        }
        if *r == 0 {
            if str_eq(name, p) {
                return 1; // TRUE
            }
            break;
        } else {
            let len = r.offset_from(p) as usize;
            if len == c_strlen(name) && mem_eq(name, p, len) {
                return 1; // TRUE
            }
            p = r.add(1);
        }
    }
    0 // FALSE
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_find_class(
    machine_name: *const c_char,
) -> *const VirtMachineClass {
    let mut i: usize = 0;
    loop {
        let vmc: *const VirtMachineClass = list_at(i);
        if vmc.is_null() {
            break;
        }
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
        i += 1;
    }
    core::ptr::null()
}
}

// ── `for (pvmc = virt_machine_list; *pvmc != ((void *)0; pvmc++)` · machine.c:215-219 · chunk 1389 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(pvmc = virt_machine_list; *pvmc != 
//                                               ((void *)0)
//                                                   ; pvmc++) {
//             vmc = *pvmc;
//             if (find_name(machine_name, vmc->machine_names))
//                 return vmc;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1389 {
// Ported piece of virt_machine_find_class: the search loop over the
// static virt_machine_list[] table. Returns the matching class or NULL.

use core::ffi::{c_char, c_int, c_uint};

// Opaque mirrors (only ever used through pointers).
#[repr(C)]
pub struct VirtMachineParams {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    _private: [u8; 0],
}

// Exactly the C layout of struct VirtMachineClass.
#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, c_int, u16)>,
}

const COMMA: c_char = b',' as c_char;

// Ported logic of the file-static C helper find_name().
unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> c_int {
    let mut p = name_list;
    loop {
        // r = strchr(p, ',')
        let mut q = p;
        let mut r: *const c_char = core::ptr::null();
        while *q != 0 {
            if *q == COMMA {
                r = q;
                break;
            }
            q = q.add(1);
        }
        if r.is_null() {
            if str_eq(name, p) {
                return 1;
            }
            break;
        } else {
            let len = r.offset_from(p) as usize;
            if len == str_len(name) && eq_prefix(name, p, len) {
                return 1;
            }
            p = r.add(1);
        }
    }
    0
}

unsafe fn str_len(mut s: *const c_char) -> usize {
    let mut n: usize = 0;
    while *s != 0 {
        n += 1;
        s = s.add(1);
    }
    n
}

unsafe fn eq_prefix(a: *const c_char, b: *const c_char, n: usize) -> bool {
    let mut i: usize = 0;
    while i < n {
        if *a.add(i) != *b.add(i) {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn str_eq(a: *const c_char, b: *const c_char) -> bool {
    let mut i: usize = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
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
pub unsafe extern "C" fn virt_machine_find_class__part1389(
    machine_name: *const c_char,
    virt_machine_list: *const *const VirtMachineClass,
) -> *const VirtMachineClass {
    let mut pvmc = virt_machine_list;
    while !(*pvmc).is_null() {
        let vmc: *const VirtMachineClass = *pvmc;
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
        pvmc = pvmc.add(1);
    }
    core::ptr::null()
}
}

// ── `return ((void *)0) ;` · machine.c:220-220 · chunk 1390 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1390 {
// Ported piece of virt_machine_find_class: the final `return NULL;` of the
// lookup loop.
//
// ABI note: only the pointer target is ever observed, so an opaque,
// zero-sized #[repr(C)] mirror is enough to describe `const VirtMachineClass *`.

#![allow(non_camel_case_types)]

#[repr(C)]
pub struct VirtMachineClass {
    // opaque: no field is ever accessed through this ported piece
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn virt_machine_find_class__part1390() -> *const VirtMachineClass {
    core::ptr::null()
}
}

// ── `virt_machine_parse_config` · machine.c:223-404 · chunk 1391 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static int virt_machine_parse_config(VirtMachineParams *p,
//                                          char *config_file_str, int len)
//     {
//         int version, val;
//         const char *tag_name, *str;
//         char buf1[256];
//         JSONValue cfg, obj, el;
//     
//         cfg = json_parse_value_len(config_file_str, len);
//         if (json_is_error(cfg)) {
//             vm_error("error: %s\n", json_get_error(cfg));
//             json_free(cfg);
//             return -1;
//         }
//     
//         if (vm_get_int(cfg, "version", &version) < 0)
//             goto tag_fail;
//         if (version != 1) {
//             if (version > 1) {
//                 vm_error("The emulator is too old to run this VM: please upgrade\n");
//                 return -1;
//             } else {
//                 vm_error("The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n");
//                 return -1;
//             }
//         }
//     
//         if (vm_get_str(cfg, "machine", &str) < 0)
//             goto tag_fail;
//         p->machine_name = strdup(str);
//         p->vmc = virt_machine_find_class(p->machine_name);
//         if (!p->vmc) {
//             vm_error("Unknown machine name: %s\n", p->machine_name);
//             goto tag_fail;
//         }
//         p->vmc->virt_machine_set_defaults(p);
//     
//         tag_name = "memory_size";
//         if (vm_get_int(cfg, tag_name, &val) < 0)
//             goto tag_fail;
//         p->ram_size = (uint64_t)val << 20;
//     
//         tag_name = "bios";
//         if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
//         if (str) {
//             p->files[VM_FILE_BIOS].filename = strdup(str);
//         }
//     
//         tag_name = "kernel";
//         if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
//         if (str) {
//             p->files[VM_FILE_KERNEL].filename = strdup(str);
//         }
//     
//         if (vm_get_str_opt(cfg, "cmdline", &str) < 0)
//             goto tag_fail;
//         if (str) {
//             p->cmdline = cmdline_subst(str);
//         }
//     
//         tag_name = "initrd";
//         if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
//         if (str) {
//             p->files[VM_FILE_INITRD].filename = strdup(str);
//         }
//     
//         for(;;) {
//             snprintf(buf1, sizeof(buf1), "drive%d", p->drive_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->drive_count >= 4) {
//                 vm_error("Too many drives\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "file", &str) < 0)
//                 goto tag_fail;
//             p->tab_drive[p->drive_count].filename = strdup(str);
//             if (vm_get_str_opt(obj, "device", &str) < 0)
//                 goto tag_fail;
//             p->tab_drive[p->drive_count].device = strdup_null(str);
//             p->drive_count++;
//         }
//     
//         for(;;) {
//             snprintf(buf1, sizeof(buf1), "fs%d", p->fs_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->fs_count >= 4) {
//                 vm_error("Too many filesystems\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "file", &str) < 0)
//                 goto tag_fail;
//             p->tab_fs[p->fs_count].filename = strdup(str);
//             if (vm_get_str_opt(obj, "tag", &str) < 0)
//                 goto tag_fail;
//             if (!str) {
//                 if (p->fs_count == 0)
//                     strcpy(buf1, "/dev/root");
//                 else
//                     snprintf(buf1, sizeof(buf1), "/dev/root%d", p->fs_count);
//                 str = buf1;
//             }
//             p->tab_fs[p->fs_count].tag = strdup(str);
//             p->fs_count++;
//         }
//     
//         for(;;) {
//             snprintf(buf1, sizeof(buf1), "eth%d", p->eth_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->eth_count >= 1) {
//                 vm_error("Too many ethernet interfaces\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "driver", &str) < 0)
//                 goto tag_fail;
//             p->tab_eth[p->eth_count].driver = strdup(str);
//             if (!strcmp(str, "tap")) {
//                 if (vm_get_str(obj, "ifname", &str) < 0)
//                     goto tag_fail;
//                 p->tab_eth[p->eth_count].ifname = strdup(str);
//             }
//             p->eth_count++;
//         }
//     
//         p->display_device = 
//                            ((void *)0)
//                                ;
//         obj = json_object_get(cfg, "display0");
//         if (!json_is_undefined(obj)) {
//             if (vm_get_str(obj, "device", &str) < 0)
//                 goto tag_fail;
//             p->display_device = strdup(str);
//             if (vm_get_int(obj, "width", &p->width) < 0)
//                 goto tag_fail;
//             if (vm_get_int(obj, "height", &p->height) < 0)
//                 goto tag_fail;
//             if (vm_get_str_opt(obj, "vga_bios", &str) < 0)
//                 goto tag_fail;
//             if (str) {
//                 p->files[VM_FILE_VGA_BIOS].filename = strdup(str);
//             }
//         }
//     
//         if (vm_get_str_opt(cfg, "input_device", &str) < 0)
//             goto tag_fail;
//         p->input_device = strdup_null(str);
//     
//         if (vm_get_str_opt(cfg, "accel", &str) < 0)
//             goto tag_fail;
//         if (str) {
//             if (!strcmp(str, "none")) {
//                 p->accel_enable = FALSE;
//             } else if (!strcmp(str, "auto")) {
//                 p->accel_enable = TRUE;
//             } else {
//                 vm_error("unsupported 'accel' config: %s\n", str);
//                 return -1;
//             }
//         }
//     
//         tag_name = "rtc_local_time";
//         el = json_object_get(cfg, tag_name);
//         if (!json_is_undefined(el)) {
//             if (el.type != JSON_BOOL) {
//                 vm_error("%s: boolean expected\n", tag_name);
//                 goto tag_fail;
//             }
//             p->rtc_local_time = el.u.b;
//         }
//     
//         json_free(cfg);
//         return 0;
//      tag_fail:
//         json_free(cfg);
//         return -1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1391 {
// Rust port of the static C function virt_machine_parse_config (machine.c).
// The static helpers it uses (find_name, virt_machine_find_class, vm_get_str,
// vm_get_str_opt, strdup_null, cmdline_subst) have no linker symbols, so their
// logic is ported here as private Rust functions.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

// JSONTypeEnum values (json.h)
const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_BOOL: c_int = 4;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

// VMFileTypeEnum values (machine.h)
const VM_FILE_BIOS: usize = 0;
const VM_FILE_VGA_BIOS: usize = 1;
const VM_FILE_KERNEL: usize = 2;
const VM_FILE_INITRD: usize = 3;
const VM_FILE_COUNT: usize = 4;

const MAX_DRIVE_DEVICE: c_int = 4;
const MAX_ETH_DEVICE: c_int = 1;

// typedef struct { int len; char data[0]; } JSONString;
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
pub union JSONValueUnion {
    pub str: *mut JSONString,
    pub int32: c_int,
    pub b: c_int, // BOOL
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueUnion,
}

// machine.h types touched here
#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; MAX_DRIVE_DEVICE as usize],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; MAX_DRIVE_DEVICE as usize],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; MAX_ETH_DEVICE as usize],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; VM_FILE_COUNT],
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(p: *mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(p: *const VirtMachineParams) -> *mut c_void>,
    pub virt_machine_end: Option<unsafe extern "C" fn(s: *mut c_void)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(s: *mut c_void, delay: c_int) -> c_int>,
    pub virt_machine_interp:
        Option<unsafe extern "C" fn(s: *mut c_void, max_exec_cycle: c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(s: *mut c_void) -> c_int>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(s1: *mut c_void, dx: c_int, dy: c_int, dz: c_int, buttons: u32)>,
    pub vm_send_key_event:
        Option<unsafe extern "C" fn(s1: *mut c_void, is_down: c_int, key_code: u16)>,
}

// DynBuf (cutils.h)
#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

// struct tm (glibc layout, used by cmdline_subst for ${TZ})
#[repr(C)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

extern "C" {
    // json.h
    fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue;
    fn json_free(val: JSONValue);
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn json_get_error(val: JSONValue) -> *const c_char;

    // machine.c / machine.h
    fn vm_error(fmt: *const c_char, ...);
    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;

    // C library
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;

    // cutils.c
    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, str_: *const c_char);

    // time
    fn time(t: *mut c_long) -> c_long;
    fn localtime_r(t: *const c_long, tm: *mut Tm) -> *mut Tm;

    // machine classes (defined in riscv_machine.c / x86_machine.c)
    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;
}

// small literal helper: b"..." -> *const c_char
macro_rules! cstr {
    ($s:expr) => {
        $s.as_ptr() as *const c_char
    };
}

// static BOOL find_name(const char *name, const char *name_list)
unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> c_int {
    let mut p = name_list;
    loop {
        let r = strchr(p, b',' as c_int);
        if r.is_null() {
            if strcmp(name, p) == 0 {
                return 1; // TRUE
            }
            break;
        } else {
            let len = r as usize - p as usize;
            if len == strlen(name) && memcmp(name as *const c_void, p as *const c_void, len) == 0 {
                return 1; // TRUE
            }
            p = r.add(1);
        }
    }
    0 // FALSE
}

// static const VirtMachineClass *virt_machine_list[] = {
//     &riscv_machine_class, &pc_machine_class, NULL };
unsafe fn virt_machine_find_class(machine_name: *const c_char) -> *const VirtMachineClass {
    let list: [*const VirtMachineClass; 3] = [
        &riscv_machine_class as *const VirtMachineClass,
        &pc_machine_class as *const VirtMachineClass,
        ptr::null(),
    ];
    let mut i: usize = 0;
    while let Some(slot) = list.get(i) {
        let vmc = *slot;
        if vmc.is_null() {
            break;
        }
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
        i += 1;
    }
    ptr::null()
}

// static int vm_get_str2(JSONValue obj, const char *name, const char **pstr, BOOL is_opt)
unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    let val = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = ptr::null();
            return 0;
        } else {
            vm_error(cstr!(b"expecting '%s' property\n\0"), name);
            return -1;
        }
    }
    if val.type_ != JSON_STR {
        vm_error(cstr!(b"%s: string expected\n\0"), name);
        return -1;
    }
    *pstr = (*val.u.str).data.as_ptr();
    0
}

unsafe fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 0)
}

unsafe fn vm_get_str_opt(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 1)
}

// static char *strdup_null(const char *str)
unsafe fn strdup_null(str_: *const c_char) -> *mut c_char {
    if str_.is_null() {
        ptr::null_mut()
    } else {
        strdup(str_)
    }
}

// static char *cmdline_subst(const char *cmdline)
unsafe fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    let mut dbuf = DynBuf {
        buf: ptr::null_mut(),
        size: 0,
        allocated_size: 0,
    };
    let mut var_name = [0i8; 32];
    let mut buf = [0i8; 32];
    let var_name_size = core::mem::size_of::<[i8; 32]>();
    let buf_size = core::mem::size_of::<[i8; 32]>();

    dbuf_init(&mut dbuf);
    let mut p = cmdline;
    while *p != 0 {
        if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
            p = p.add(2);
            let vn_base = var_name.as_mut_ptr();
            let mut q = vn_base;
            while *p != 0 && *p != b'}' as c_char {
                if (q as usize - vn_base as usize) < var_name_size - 1 {
                    *q = *p;
                    q = q.add(1);
                }
                p = p.add(1);
            }
            *q = 0;
            if *p == b'}' as c_char {
                p = p.add(1);
            }
            if strcmp(vn_base, cstr!(b"TZ\0")) == 0 {
                let mut ti: c_long = 0;
                let mut tm = core::mem::zeroed::<Tm>();

                time(&mut ti as *mut c_long);
                localtime_r(&ti as *const c_long, &mut tm as *mut Tm);
                let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                let mut sg: c_int = b'-' as c_int;
                if n < 0 {
                    sg = b'+' as c_int;
                    n = -n;
                }
                snprintf(
                    buf.as_mut_ptr(),
                    buf_size,
                    cstr!(b"UTC%c%02d:%02d\0"),
                    sg,
                    n / 60,
                    n % 60,
                );
                dbuf_putstr(&mut dbuf, buf.as_ptr());
            }
        } else {
            dbuf_putc(&mut dbuf, *p as u8);
            p = p.add(1);
        }
    }
    dbuf_putc(&mut dbuf, 0);
    dbuf.buf as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config(
    p_arg: *mut VirtMachineParams,
    config_file_str: *mut c_char,
    len: c_int,
) -> c_int {
    let p: &mut VirtMachineParams = &mut *p_arg;
    let mut version: c_int = 0;
    let mut val: c_int = 0;
    let mut str_: *const c_char;
    let mut buf1 = [0i8; 256];
    let buf1_size = core::mem::size_of::<[i8; 256]>();

    let cfg = json_parse_value_len(config_file_str, len);
    if cfg.type_ == JSON_EXCEPTION {
        vm_error(cstr!(b"error: %s\n\0"), json_get_error(cfg));
        json_free(cfg);
        return -1;
    }

    // tag_fail is emulated with this flag; the original "goto tag_fail" jumps
    // straight to the cleanup at the end of the function.
    let mut fail = false;

    // ---- version ----
    if vm_get_int(cfg, cstr!(b"version\0"), &mut version as *mut c_int) < 0 {
        fail = true;
    }
    if !fail && version != 1 {
        if version > 1 {
            vm_error(cstr!(
                b"The emulator is too old to run this VM: please upgrade\n\0"
            ));
            return -1;
        } else {
            vm_error(cstr!(b"The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n\0"));
            return -1;
        }
    }

    // ---- machine ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str(cfg, cstr!(b"machine\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else {
            p.machine_name = strdup(str_);
            p.vmc = virt_machine_find_class(p.machine_name);
            if p.vmc.is_null() {
                vm_error(cstr!(b"Unknown machine name: %s\n\0"), p.machine_name);
                fail = true;
            } else if let Some(set_defaults) = (*p.vmc).virt_machine_set_defaults {
                set_defaults(p as *mut VirtMachineParams);
            }
        }
    }

    // ---- memory_size ----
    if !fail {
        if vm_get_int(cfg, cstr!(b"memory_size\0"), &mut val as *mut c_int) < 0 {
            fail = true;
        } else {
            p.ram_size = ((val as u32) as u64) << 20;
        }
    }

    // ---- bios ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"bios\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else if !str_.is_null() {
            p.files[VM_FILE_BIOS].filename = strdup(str_);
        }
    }

    // ---- kernel ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"kernel\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else if !str_.is_null() {
            p.files[VM_FILE_KERNEL].filename = strdup(str_);
        }
    }

    // ---- cmdline ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"cmdline\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else if !str_.is_null() {
            p.cmdline = cmdline_subst(str_);
        }
    }

    // ---- initrd ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"initrd\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else if !str_.is_null() {
            p.files[VM_FILE_INITRD].filename = strdup(str_);
        }
    }

    // ---- drives ----
    if !fail {
        loop {
            snprintf(
                buf1.as_mut_ptr(),
                buf1_size,
                cstr!(b"drive%d\0"),
                p.drive_count,
            );
            let obj = json_object_get(cfg, buf1.as_ptr());
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if p.drive_count >= MAX_DRIVE_DEVICE {
                vm_error(cstr!(b"Too many drives\n\0"));
                return -1;
            }
            str_ = ptr::null();
            if vm_get_str(obj, cstr!(b"file\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
                break;
            }
            let idx = p.drive_count as usize;
            match p.tab_drive.get_mut(idx) {
                Some(entry) => entry.filename = strdup(str_),
                None => {
                    fail = true;
                    break;
                }
            }
            str_ = ptr::null();
            if vm_get_str_opt(obj, cstr!(b"device\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
                break;
            }
            match p.tab_drive.get_mut(idx) {
                Some(entry) => entry.device = strdup_null(str_),
                None => {
                    fail = true;
                    break;
                }
            }
            p.drive_count += 1;
        }
    }

    // ---- filesystems ----
    if !fail {
        loop {
            snprintf(buf1.as_mut_ptr(), buf1_size, cstr!(b"fs%d\0"), p.fs_count);
            let obj = json_object_get(cfg, buf1.as_ptr());
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if p.fs_count >= MAX_DRIVE_DEVICE {
                vm_error(cstr!(b"Too many filesystems\n\0"));
                return -1;
            }
            str_ = ptr::null();
            if vm_get_str(obj, cstr!(b"file\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
                break;
            }
            let idx = p.fs_count as usize;
            match p.tab_fs.get_mut(idx) {
                Some(entry) => entry.filename = strdup(str_),
                None => {
                    fail = true;
                    break;
                }
            }
            str_ = ptr::null();
            if vm_get_str_opt(obj, cstr!(b"tag\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
                break;
            }
            if str_.is_null() {
                if p.fs_count == 0 {
                    strcpy(buf1.as_mut_ptr(), cstr!(b"/dev/root\0"));
                } else {
                    snprintf(
                        buf1.as_mut_ptr(),
                        buf1_size,
                        cstr!(b"/dev/root%d\0"),
                        p.fs_count,
                    );
                }
                str_ = buf1.as_ptr();
            }
            match p.tab_fs.get_mut(idx) {
                Some(entry) => entry.tag = strdup(str_),
                None => {
                    fail = true;
                    break;
                }
            }
            p.fs_count += 1;
        }
    }

    // ---- ethernet ----
    if !fail {
        loop {
            snprintf(buf1.as_mut_ptr(), buf1_size, cstr!(b"eth%d\0"), p.eth_count);
            let obj = json_object_get(cfg, buf1.as_ptr());
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if p.eth_count >= MAX_ETH_DEVICE {
                vm_error(cstr!(b"Too many ethernet interfaces\n\0"));
                return -1;
            }
            str_ = ptr::null();
            if vm_get_str(obj, cstr!(b"driver\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
                break;
            }
            let idx = p.eth_count as usize;
            match p.tab_eth.get_mut(idx) {
                Some(entry) => entry.driver = strdup(str_),
                None => {
                    fail = true;
                    break;
                }
            }
            if strcmp(str_, cstr!(b"tap\0")) == 0 {
                str_ = ptr::null();
                if vm_get_str(obj, cstr!(b"ifname\0"), &mut str_ as *mut *const c_char) < 0 {
                    fail = true;
                    break;
                }
                match p.tab_eth.get_mut(idx) {
                    Some(entry) => entry.ifname = strdup(str_),
                    None => {
                        fail = true;
                        break;
                    }
                }
            }
            p.eth_count += 1;
        }
    }

    // ---- display ----
    if !fail {
        p.display_device = ptr::null_mut();
        let obj = json_object_get(cfg, cstr!(b"display0\0"));
        if obj.type_ != JSON_UNDEFINED {
            str_ = ptr::null();
            if vm_get_str(obj, cstr!(b"device\0"), &mut str_ as *mut *const c_char) < 0 {
                fail = true;
            } else {
                p.display_device = strdup(str_);
                if vm_get_int(obj, cstr!(b"width\0"), &mut p.width as *mut c_int) < 0 {
                    fail = true;
                } else if vm_get_int(obj, cstr!(b"height\0"), &mut p.height as *mut c_int) < 0 {
                    fail = true;
                } else {
                    str_ = ptr::null();
                    if vm_get_str_opt(
                        obj,
                        cstr!(b"vga_bios\0"),
                        &mut str_ as *mut *const c_char,
                    ) < 0
                    {
                        fail = true;
                    } else if !str_.is_null() {
                        p.files[VM_FILE_VGA_BIOS].filename = strdup(str_);
                    }
                }
            }
        }
    }

    // ---- input_device ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"input_device\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else {
            p.input_device = strdup_null(str_);
        }
    }

    // ---- accel ----
    if !fail {
        str_ = ptr::null();
        if vm_get_str_opt(cfg, cstr!(b"accel\0"), &mut str_ as *mut *const c_char) < 0 {
            fail = true;
        } else if !str_.is_null() {
            if strcmp(str_, cstr!(b"none\0")) == 0 {
                p.accel_enable = 0; // FALSE
            } else if strcmp(str_, cstr!(b"auto\0")) == 0 {
                p.accel_enable = 1; // TRUE
            } else {
                vm_error(cstr!(b"unsupported 'accel' config: %s\n\0"), str_);
                return -1;
            }
        }
    }

    // ---- rtc_local_time ----
    if !fail {
        let tag_name = cstr!(b"rtc_local_time\0");
        let el = json_object_get(cfg, tag_name);
        if el.type_ != JSON_UNDEFINED {
            if el.type_ != JSON_BOOL {
                vm_error(cstr!(b"%s: boolean expected\n\0"), tag_name);
                fail = true;
            } else {
                p.rtc_local_time = el.u.b;
            }
        }
    }

    // tag_fail / success
    json_free(cfg);
    if fail {
        return -1;
    }
    0
}
}

// ── `int version, val; const char *tag_name, *str; char buf1[256]; JSONValue cfg, obj, el; cfg = jso…` · machine.c:226-231 · chunk 1392 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int version, val;
//         const char *tag_name, *str;
//         char buf1[256];
//         JSONValue cfg, obj, el;
//     
//         cfg = json_parse_value_len(config_file_str, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1392 {
// Port of the parse step of virt_machine_parse_config: parse the config text
// into a JSONValue.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C JSONValue union (all members are pointer-sized or smaller;
// the largest alignment is that of a pointer).
#[repr(C)]
pub union JSONValueU {
    pub str: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of:
//   typedef struct JSONValue { JSONTypeEnum type; union {...} u; } JSONValue;
#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue;
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1392(
    config_file_str: *const c_char,
    len: c_int,
) -> JSONValue {
    unsafe { json_parse_value_len(config_file_str, len) }
}
}

// ── `if (json_is_error(cfg))` · machine.c:232-236 · chunk 1393 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (json_is_error(cfg)) {
//             vm_error("error: %s\n", json_get_error(cfg));
//             json_free(cfg);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1393 {
// Port of the JSON-error check at the start of virt_machine_parse_config.
// Returns -1 when the parsed value is a JSON exception (after reporting and
// freeing it), and 0 otherwise so the C caller can continue.

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONUnion {
    pub str_: *mut core::ffi::c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

const JSON_EXCEPTION: c_int = 7;

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
    fn json_free(val: JSONValue);
    fn json_get_error(val: JSONValue) -> *const c_char;
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1393(cfg: JSONValue) -> c_int {
    if cfg.r#type == JSON_EXCEPTION {
        vm_error(b"error: %s\n\0".as_ptr() as *const c_char, json_get_error(cfg));
        json_free(cfg);
        return -1;
    }
    0
}
}

// ── `if (vm_get_int(cfg, "version", &version) < 0)` · machine.c:238-239 · chunk 1394 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_int(cfg, "version", &version) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1394 {
// Portion of virt_machine_parse_config: read the mandatory "version" tag.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

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
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;
}

static VERSION_TAG: &[u8; 8] = b"version\0";

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1394(
    cfg: JSONValue,
    version: *mut c_int,
) -> c_int {
    unsafe { vm_get_int(cfg, VERSION_TAG.as_ptr() as *const c_char, version) }
}
}

// ── `if (version != 1)` · machine.c:240-248 · chunk 1395 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (version != 1) {
//             if (version > 1) {
//                 vm_error("The emulator is too old to run this VM: please upgrade\n");
//                 return -1;
//             } else {
//                 vm_error("The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n");
//                 return -1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1395 {
// Portion of virt_machine_parse_config: validates the config "version" tag.
use core::ffi::{c_char, c_int};

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1395(version: c_int) -> c_int {
    if version == 1 {
        return 0;
    }
    if version > 1 {
        unsafe {
            vm_error(
                b"The emulator is too old to run this VM: please upgrade\n\0".as_ptr() as *const c_char,
            );
        }
    } else {
        unsafe {
            vm_error(
                b"The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n\0"
                    .as_ptr() as *const c_char,
            );
        }
    }
    -1
}
}

// ── `if (vm_get_str(cfg, "machine", &str) < 0)` · machine.c:250-251 · chunk 1396 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str(cfg, "machine", &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1396 {
// Port of the `vm_get_str(cfg, "machine", &str)` check from
// virt_machine_parse_config. Mirrors the inlined logic of vm_get_str2 with
// is_opt = FALSE.

use core::ffi::{c_char, c_int};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    pub r#type: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1396(
    cfg: JSONValue,
    pstr: *mut *const c_char,
) -> c_int {
    let name: *const c_char = b"machine\0".as_ptr() as *const c_char;

    let val = unsafe { json_object_get(cfg, name) };

    if val.r#type == JSON_UNDEFINED {
        unsafe {
            vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }
    if val.r#type != JSON_STR {
        unsafe {
            vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }
    unsafe {
        let s = val.u.str_;
        let data: *const c_char = core::ptr::addr_of!((*s).data) as *const c_char;
        *pstr = data;
    }
    0
}
}

// ── `p->machine_name = strdup(str); p->vmc = virt_machine_find_class(p->machine_name);` · machine.c:252-253 · chunk 1397 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->machine_name = strdup(str);
//         p->vmc = virt_machine_find_class(p->machine_name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1397 {
// Port of machine.c lines 252-253:
//     p->machine_name = strdup(str);
//     p->vmc = virt_machine_find_class(p->machine_name);
// virt_machine_find_class (and its helper find_name / table virt_machine_list)
// are static in machine.c, so their logic is ported here as private Rust fns.

use core::ffi::{c_char, c_int, c_uint};

// Opaque C types only ever handled through pointers here.
#[repr(C)]
pub struct BlockDevice {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct FSDevice {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct EthernetDevice {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct CharacterDevice {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct VirtMachine {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut FSDevice,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, c_int, u16)>,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;
}

// Equivalent of C strlen().
unsafe fn c_strlen(mut s: *const c_char) -> usize {
    let mut n: usize = 0;
    while *s != 0 {
        n += 1;
        s = s.add(1);
    }
    n
}

// Equivalent of C strcmp(a, b) == 0.
unsafe fn c_streq(mut a: *const c_char, mut b: *const c_char) -> bool {
    loop {
        let ca = *a;
        let cb = *b;
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        a = a.add(1);
        b = b.add(1);
    }
}

// Equivalent of C memcmp(a, b, len) == 0.
unsafe fn c_memeq(mut a: *const c_char, mut b: *const c_char, len: usize) -> bool {
    let mut i: usize = 0;
    while i < len {
        if *a != *b {
            return false;
        }
        a = a.add(1);
        b = b.add(1);
        i += 1;
    }
    true
}

// static BOOL find_name(const char *name, const char *name_list)
unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> c_int {
    let name_len = c_strlen(name);
    let mut p = name_list;
    loop {
        let mut r = p;
        while *r != 0 && (*r as u8) != b',' {
            r = r.add(1);
        }
        if *r == 0 {
            if c_streq(name, p) {
                return 1; // TRUE
            }
            break;
        } else {
            let len = (r as usize).wrapping_sub(p as usize);
            if len == name_len && c_memeq(name, p, len) {
                return 1; // TRUE
            }
            p = r.add(1);
        }
    }
    0 // FALSE
}

// static const VirtMachineClass *virt_machine_find_class(const char *machine_name)
unsafe fn virt_machine_find_class(machine_name: *const c_char) -> *const VirtMachineClass {
    // virt_machine_list[] as built by the preprocessor for this target.
    let list: [*const VirtMachineClass; 3] = [
        &riscv_machine_class,
        &pc_machine_class,
        core::ptr::null(),
    ];
    let mut i: usize = 0;
    while i < list.len() {
        let vmc = *list.get_unchecked(i);
        if vmc.is_null() {
            break;
        }
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
        i += 1;
    }
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1397(
    p: *mut VirtMachineParams,
    str_: *const c_char,
) {
    if p.is_null() {
        return;
    }
    let machine_name = strdup(str_);
    (*p).machine_name = machine_name;
    (*p).vmc = virt_machine_find_class(machine_name);
}
}

// ── `if (!p->vmc)` · machine.c:254-257 · chunk 1398 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!p->vmc) {
//             vm_error("Unknown machine name: %s\n", p->machine_name);
//             goto tag_fail;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1398 {
// Port of the "unknown machine name" check from virt_machine_parse_config.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

/// Returns 1 when the error path (goto tag_fail) must be taken, 0 otherwise.
#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1398(
    vmc: *const c_void,
    machine_name: *const c_char,
) -> c_int {
    if vmc.is_null() {
        unsafe {
            vm_error(
                b"Unknown machine name: %s\n\0".as_ptr() as *const c_char,
                machine_name,
            );
        }
        return 1;
    }
    0
}
}

// ── `p->vmc->virt_machine_set_defaults(p); tag_name = "memory_size";` · machine.c:258-260 · chunk 1399 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->vmc->virt_machine_set_defaults(p);
//     
//         tag_name = "memory_size";
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1399 {
// Ported piece of virt_machine_parse_config: dispatch to the machine class'
// default setter and pick the next config tag name.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init: *mut c_void,
    pub virt_machine_end: *mut c_void,
    pub virt_machine_get_sleep_duration: *mut c_void,
    pub virt_machine_interp: *mut c_void,
    pub vm_mouse_is_absolute: *mut c_void,
    pub vm_send_mouse_event: *mut c_void,
    pub vm_send_key_event: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

static MEMORY_SIZE: [u8; 12] = *b"memory_size\0";

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1399(
    p: *mut VirtMachineParams,
) -> *const c_char {
    let vmc = (*p).vmc;
    if !vmc.is_null() {
        if let Some(f) = (*vmc).virt_machine_set_defaults {
            f(p);
        }
    }
    MEMORY_SIZE.as_ptr() as *const c_char
}
}

// ── `if (vm_get_int(cfg, tag_name, &val) < 0)` · machine.c:261-262 · chunk 1400 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_int(cfg, tag_name, &val) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1400 {
// Port of machine.c lines 261-262: required-integer property fetch.
use core::ffi::{c_char, c_int, c_void};

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
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1400(
    cfg: JSONValue,
    tag_name: *const c_char,
    val: *mut c_int,
) -> c_int {
    unsafe { vm_get_int(cfg, tag_name, val) }
}
}

// ── `p->ram_size = (uint64_t)val << 20; tag_name = "bios";` · machine.c:263-265 · chunk 1401 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->ram_size = (uint64_t)val << 20;
//     
//         tag_name = "bios";
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1401 {
// Port of virt_machine_parse_config__part1401:
//   p->ram_size = (uint64_t)val << 20;
//   tag_name = "bios";
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

static BIOS_TAG: [u8; 5] = *b"bios\0";

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1401(
    p: *mut VirtMachineParams,
    val: c_int,
) -> *const c_char {
    if !p.is_null() {
        unsafe {
            (*p).ram_size = (val as u64) << 20;
        }
    }
    BIOS_TAG.as_ptr() as *const c_char
}
}

// ── `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` · machine.c:266-267 · chunk 1402 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1402 {
// Port of the `vm_get_str_opt(cfg, tag_name, &str)` check used by
// virt_machine_parse_config.  vm_get_str_opt/vm_get_str2 are static in the C
// translation unit, so their (is_opt == TRUE) logic is reproduced here.

#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// typedef struct { int len; char data[0]; } JSONString;
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

// The anonymous union inside JSONValue.  Pointer members are all plain
// pointers, so mirroring them as *mut c_void keeps the layout identical.
#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// typedef struct JSONValue { JSONTypeEnum type; union {...} u; } JSONValue;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1402(
    cfg: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    // val = json_object_get(obj, name);
    let val = unsafe { json_object_get(cfg, name) };

    // if (json_is_undefined(val)) { *pstr = NULL; return 0; }
    if val.type_ == JSON_UNDEFINED {
        if !pstr.is_null() {
            unsafe { *pstr = core::ptr::null() };
        }
        return 0;
    }

    // if (val.type != JSON_STR) { vm_error("%s: string expected\n", name); return -1; }
    if val.type_ != JSON_STR {
        unsafe {
            vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        }
        return -1;
    }

    // *pstr = val.u.str->data;
    let s: *const c_char = unsafe { (*val.u.str_).data.as_ptr() };
    if !pstr.is_null() {
        unsafe { *pstr = s };
    }
    0
}
}

// ── `if (str)` · machine.c:268-270 · chunk 1403 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str) {
//             p->files[VM_FILE_BIOS].filename = strdup(str);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1403 {
// Port of machine.c lines 268-270 inside virt_machine_parse_config:
//     if (str) { p->files[VM_FILE_BIOS].filename = strdup(str); }
use core::ffi::{c_char, c_int};

// Mirror of the C `VMFileEntry` (machine.c): only the fields of the struct
// are needed; the caller hands us a pointer to the wanted array element.
#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

extern "C" {
    // char *strdup(const char *s);
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1403(
    entry: *mut VMFileEntry,
    str_: *const c_char,
) {
    if !str_.is_null() && !entry.is_null() {
        unsafe {
            (*entry).filename = strdup(str_);
        }
    }
}
}

// ── `tag_name = "kernel";` · machine.c:272-272 · chunk 1404 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tag_name = "kernel";
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1404 {
use core::ffi::c_char;

// "kernel" as a NUL-terminated C string literal.
static KERNEL_TAG: [u8; 7] = *b"kernel\0";

// Returns the address of the static "kernel" tag name used to look up the
// kernel image entry in the VM configuration object.
#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1404() -> *const c_char {
    KERNEL_TAG.as_ptr() as *const c_char
}
}

// ── `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` · machine.c:273-274 · chunk 1405 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1405 {
// Port of the static helper vm_get_str_opt() (i.e. vm_get_str2 with is_opt = TRUE)
// as used by virt_machine_parse_config() for the "kernel" tag.

use core::ffi::{c_char, c_int};

type BOOL = c_int;

type JSONTypeEnum = c_int;

const JSON_STR: JSONTypeEnum = 0;
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
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
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
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1405(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    unsafe {
        let val = json_object_get(obj, name);
        if val.type_ == JSON_UNDEFINED {
            // optional property: *pstr = NULL; return 0;
            *pstr = core::ptr::null();
            return 0;
        }
        if val.type_ != JSON_STR {
            vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
            return -1;
        }
        let s = val.u.str_;
        *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
        0
    }
}
}

// ── `if (str)` · machine.c:275-277 · chunk 1406 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str) {
//             p->files[VM_FILE_KERNEL].filename = strdup(str);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1406 {
// Port of the TARGET inside virt_machine_parse_config:
//   if (str) { p->files[VM_FILE_KERNEL].filename = strdup(str); }
use core::ffi::{c_char, c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_local_time: BOOL,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: BOOL,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

const VM_FILE_KERNEL: usize = 2;

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1406(
    p: *mut VirtMachineParams,
    str_: *const c_char,
) {
    if !str_.is_null() && !p.is_null() {
        let files = &mut (*p).files;
        let entry = &mut files[VM_FILE_KERNEL];
        entry.filename = strdup(str_);
    }
}
}

// ── `if (vm_get_str_opt(cfg, "cmdline", &str) < 0)` · machine.c:279-280 · chunk 1407 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, "cmdline", &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1407 {
// Port of the "cmdline" lookup from virt_machine_parse_config.
// Mirrors the static helper vm_get_str_opt() (is_opt = TRUE) and json_is_undefined().

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub value_type: c_int,
    pub u: JSONValueUnion,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1407(
    cfg: JSONValue,
    pstr: *mut *const c_char,
) -> c_int {
    let name: *const c_char = b"cmdline\0".as_ptr() as *const c_char;

    let val = json_object_get(cfg, name);

    if val.value_type == JSON_UNDEFINED {
        // optional tag missing: *pstr = NULL, success
        if !pstr.is_null() {
            *pstr = core::ptr::null();
        }
        return 0;
    }

    if val.value_type != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }

    let strp = val.u.str_;
    let data: *const c_char = if strp.is_null() {
        core::ptr::null()
    } else {
        (*strp).data.as_ptr()
    };
    if !pstr.is_null() {
        *pstr = data;
    }
    0
}
}

// ── `if (str)` · machine.c:281-283 · chunk 1408 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str) {
//             p->cmdline = cmdline_subst(str);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1408 {
// Port of the `if (str) { p->cmdline = cmdline_subst(str); }` piece of
// virt_machine_parse_config, plus the static helper cmdline_subst it calls.

use core::ffi::{c_char, c_int, c_long};

#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

// struct tm as laid out by the C library on this target.
#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

extern "C" {
    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, str: *const c_char);
    fn time(timer: *mut c_long) -> c_long;
    fn localtime_r(timer: *const c_long, result: *mut Tm) -> *mut Tm;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
}

// static in C: no linker symbol, so the logic lives here as a private fn.
unsafe fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    let mut dbuf: DynBuf = DynBuf {
        buf: core::ptr::null_mut(),
        size: 0,
        allocated_size: 0,
    };
    let mut var_name: [u8; 32] = [0; 32];
    let mut buf: [u8; 32] = [0; 32];

    dbuf_init(&mut dbuf);
    let mut p = cmdline;
    while *p != 0 {
        if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
            p = p.add(2);
            let mut q: usize = 0;
            while *p != 0 && *p != b'}' as c_char {
                if q < var_name.len() - 1 {
                    var_name[q] = *p as u8;
                    q += 1;
                }
                p = p.add(1);
            }
            var_name[q] = 0;
            if *p == b'}' as c_char {
                p = p.add(1);
            }
            if strcmp(
                var_name.as_ptr() as *const c_char,
                b"TZ\0".as_ptr() as *const c_char,
            ) == 0
            {
                let mut ti: c_long = 0;
                let mut tm: Tm = core::mem::zeroed();

                time(&mut ti);
                localtime_r(&ti, &mut tm);
                let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                let mut sg: c_char = b'-' as c_char;
                if n < 0 {
                    sg = b'+' as c_char;
                    n = -n;
                }
                snprintf(
                    buf.as_mut_ptr() as *mut c_char,
                    buf.len(),
                    b"UTC%c%02d:%02d\0".as_ptr() as *const c_char,
                    sg as c_int,
                    n / 60,
                    n % 60,
                );
                dbuf_putstr(&mut dbuf, buf.as_ptr() as *const c_char);
            }
        } else {
            let ch = *p as u8;
            dbuf_putc(&mut dbuf, ch);
            p = p.add(1);
        }
    }
    dbuf_putc(&mut dbuf, 0);
    dbuf.buf as *mut c_char
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1408(str: *const c_char) -> *mut c_char {
    unsafe { cmdline_subst(str) }
}
}

// ── `tag_name = "initrd";` · machine.c:285-285 · chunk 1409 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tag_name = "initrd";
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1409 {
use core::ffi::c_char;

// "initrd" as a NUL-terminated C string literal.
static INITRD: [u8; 7] = *b"initrd\0";

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1409(tag_name: *mut *const c_char) {
    unsafe {
        if !tag_name.is_null() {
            *tag_name = INITRD.as_ptr() as *const c_char;
        }
    }
}
}

// ── `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` · machine.c:286-287 · chunk 1410 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, tag_name, &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1410 {
// Ported piece of virt_machine_parse_config: the "tag_name" optional string
// lookup (originally vm_get_str_opt, a static helper of machine.c).

use core::ffi::{c_char, c_int, c_void};

pub const JSON_STR: c_int = 0;
pub const JSON_UNDEFINED: c_int = 6;

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
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

// Port of the file-static C helper vm_get_str2(..., is_opt).
unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    let val: JSONValue = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = core::ptr::null();
            return 0;
        } else {
            vm_error(
                b"expecting '%s' property\n\0".as_ptr() as *const c_char,
                name,
            );
            return -1;
        }
    }
    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    let s = val.u.str_;
    *pstr = (s as *const u8).add(4) as *const c_char;
    0
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1410(
    cfg: JSONValue,
    tag_name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    vm_get_str2(cfg, tag_name, pstr, 1)
}
}

// ── `if (str)` · machine.c:288-290 · chunk 1411 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str) {
//             p->files[VM_FILE_INITRD].filename = strdup(str);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1411 {
use core::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

// if (str) { p->files[VM_FILE_INITRD].filename = strdup(str); }
#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1411(
    filename_slot: *mut *mut c_char,
    str: *const c_char,
) {
    if !str.is_null() {
        *filename_slot = strdup(str);
    }
}
}

// ── `for (; ; )` · machine.c:292-308 · chunk 1412 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             snprintf(buf1, sizeof(buf1), "drive%d", p->drive_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->drive_count >= 4) {
//                 vm_error("Too many drives\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "file", &str) < 0)
//                 goto tag_fail;
//             p->tab_drive[p->drive_count].filename = strdup(str);
//             if (vm_get_str_opt(obj, "device", &str) < 0)
//                 goto tag_fail;
//             p->tab_drive[p->drive_count].device = strdup_null(str);
//             p->drive_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1412 {
// Piece of virt_machine_parse_config: the "drive%d" parsing loop.
use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type BOOL = c_int;

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirrors `typedef struct JSONValue { JSONTypeEnum type; union {...} u; }`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

// Mirrors VirtMachineParams with VM_FILE_COUNT == 4.
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_local_time: BOOL,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: BOOL,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn vm_error(fmt: *const c_char, ...);
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
}

#[inline]
fn json_is_undefined(val: JSONValue) -> bool {
    val.type_ == JSON_UNDEFINED
}

// Port of the static C helper vm_get_str2.
unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: BOOL,
) -> c_int {
    let val = json_object_get(obj, name);
    if json_is_undefined(val) {
        if is_opt != 0 {
            *pstr = ptr::null();
            0
        } else {
            vm_error(
                b"expecting '%s' property\n\0".as_ptr() as *const c_char,
                name,
            );
            -1
        }
    } else if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        -1
    } else {
        *pstr = (*val.u.str_).data.as_ptr();
        0
    }
}

// Port of the static C helper vm_get_str.
unsafe fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 0)
}

// Port of the static C helper vm_get_str_opt.
unsafe fn vm_get_str_opt(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 1)
}

// Port of the static C helper strdup_null.
unsafe fn strdup_null(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        ptr::null_mut()
    } else {
        strdup(s)
    }
}

// Status codes: 0 = loop finished (C `break`), 1 = "Too many drives"
// (C `return -1`), 2 = tag_fail (C `goto tag_fail`).
#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1412(
    p: *mut VirtMachineParams,
    cfg: JSONValue,
) -> c_int {
    let mut buf1 = [0 as c_char; 256];
    unsafe {
        loop {
            snprintf(
                buf1.as_mut_ptr(),
                256usize,
                b"drive%d\0".as_ptr() as *const c_char,
                (*p).drive_count,
            );
            let obj = json_object_get(cfg, buf1.as_ptr());
            if json_is_undefined(obj) {
                break;
            }
            if (*p).drive_count >= 4 {
                vm_error(b"Too many drives\n\0".as_ptr() as *const c_char);
                return 1;
            }
            let idx = (*p).drive_count as usize;
            let mut s: *const c_char = ptr::null();
            if vm_get_str(obj, b"file\0".as_ptr() as *const c_char, &mut s) < 0 {
                return 2;
            }
            let drive = (*p).tab_drive.as_mut_ptr().add(idx);
            (*drive).filename = strdup(s);
            if vm_get_str_opt(obj, b"device\0".as_ptr() as *const c_char, &mut s) < 0 {
                return 2;
            }
            (*drive).device = strdup_null(s);
            (*p).drive_count += 1;
        }
    }
    0
}
}

// ── `for (; ; )` · machine.c:310-333 · chunk 1413 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             snprintf(buf1, sizeof(buf1), "fs%d", p->fs_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->fs_count >= 4) {
//                 vm_error("Too many filesystems\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "file", &str) < 0)
//                 goto tag_fail;
//             p->tab_fs[p->fs_count].filename = strdup(str);
//             if (vm_get_str_opt(obj, "tag", &str) < 0)
//                 goto tag_fail;
//             if (!str) {
//                 if (p->fs_count == 0)
//                     strcpy(buf1, "/dev/root");
//                 else
//                     snprintf(buf1, sizeof(buf1), "/dev/root%d", p->fs_count);
//                 str = buf1;
//             }
//             p->tab_fs[p->fs_count].tag = strdup(str);
//             p->fs_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1413 {
// Port of the "fs%d" filesystem configuration loop of virt_machine_parse_config
// (machine.c lines 310-333).  The loop walks the JSON config entries fs0, fs1,
// ... until one is undefined, recording the filename and tag of each filesystem.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn vm_error(fmt: *const c_char, ...);
}

fn cs(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

// Port of the static C helper vm_get_str2 (vm_get_str / vm_get_str_opt).
fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    unsafe {
        let val = json_object_get(obj, name);
        if val.type_ == JSON_UNDEFINED {
            if is_opt != 0 {
                *pstr = core::ptr::null();
                return 0;
            } else {
                vm_error(cs(b"expecting '%s' property\n\0"), name);
                return -1;
            }
        }
        if val.type_ != JSON_STR {
            vm_error(cs(b"%s: string expected\n\0"), name);
            return -1;
        }
        let s = val.u.str_;
        let data = if s.is_null() {
            core::ptr::null()
        } else {
            core::ptr::addr_of!((*s).data).cast::<c_char>()
        };
        *pstr = data;
        0
    }
}

fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 0)
}

fn vm_get_str_opt(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 1)
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1413(
    p: *mut VirtMachineParams,
    cfg: JSONValue,
    buf1: *mut c_char,
    obj_out: *mut JSONValue,
    str_out: *mut *const c_char,
) -> c_int {
    unsafe {
        loop {
            snprintf(buf1, 256, cs(b"fs%d\0"), (*p).fs_count);
            let obj = json_object_get(cfg, buf1);
            *obj_out = obj;
            if obj.type_ == JSON_UNDEFINED {
                return 0;
            }
            let fs_count = (*p).fs_count;
            if fs_count >= 4 {
                vm_error(cs(b"Too many filesystems\n\0"));
                return -1;
            }
            if fs_count < 0 {
                return -1;
            }
            if vm_get_str(obj, cs(b"file\0"), str_out) < 0 {
                return -1;
            }
            let ent: *mut VMFSEntry = core::ptr::addr_of_mut!((*p).tab_fs)
                .cast::<VMFSEntry>()
                .add(fs_count as usize);
            (*ent).filename = strdup(*str_out);
            if vm_get_str_opt(obj, cs(b"tag\0"), str_out) < 0 {
                return -1;
            }
            if (*str_out).is_null() {
                if (*p).fs_count == 0 {
                    strcpy(buf1, cs(b"/dev/root\0"));
                } else {
                    snprintf(buf1, 256, cs(b"/dev/root%d\0"), (*p).fs_count);
                }
                *str_out = buf1;
            }
            (*ent).tag = strdup(*str_out);
            (*p).fs_count = fs_count + 1;
        }
    }
}
}

// ── `for (; ; )` · machine.c:335-353 · chunk 1414 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             snprintf(buf1, sizeof(buf1), "eth%d", p->eth_count);
//             obj = json_object_get(cfg, buf1);
//             if (json_is_undefined(obj))
//                 break;
//             if (p->eth_count >= 1) {
//                 vm_error("Too many ethernet interfaces\n");
//                 return -1;
//             }
//             if (vm_get_str(obj, "driver", &str) < 0)
//                 goto tag_fail;
//             p->tab_eth[p->eth_count].driver = strdup(str);
//             if (!strcmp(str, "tap")) {
//                 if (vm_get_str(obj, "ifname", &str) < 0)
//                     goto tag_fail;
//                 p->tab_eth[p->eth_count].ifname = strdup(str);
//             }
//             p->eth_count++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1414 {
// Port of the "eth%d" loop of virt_machine_parse_config().
//
// Return protocol (drives the C control flow in the replacement):
//   0 -> loop finished normally (C `break`), caller continues
//   1 -> C `return -1;`  (too many ethernet interfaces)
//   2 -> C `goto tag_fail;`
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const core::ffi::c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut core::ffi::c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn vm_error(fmt: *const c_char, ...);
}

// Port of the file-local `static int vm_get_str2(obj, name, pstr, FALSE)`
// (no linker symbol in C, so it lives here).
unsafe fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    let val: JSONValue = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        // is_opt == FALSE
        vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    let s: *mut JSONString = val.u.str_;
    *pstr = (*s).data.as_ptr();
    0
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1414(
    cfg: JSONValue,
    p: *mut VirtMachineParams,
) -> c_int {
    unsafe {
        let mut buf1 = [0 as c_char; 256];
        loop {
            snprintf(
                buf1.as_mut_ptr(),
                256,
                b"eth%d\0".as_ptr() as *const c_char,
                (*p).eth_count,
            );
            let obj: JSONValue = json_object_get(cfg, buf1.as_ptr());
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if (*p).eth_count >= 1 {
                vm_error(b"Too many ethernet interfaces\n\0".as_ptr() as *const c_char);
                return 1;
            }
            let mut s: *const c_char = core::ptr::null();
            if vm_get_str(
                obj,
                b"driver\0".as_ptr() as *const c_char,
                &mut s as *mut *const c_char,
            ) < 0
            {
                return 2;
            }
            let entry: *mut VMEthEntry = (*p).tab_eth.as_mut_ptr().offset((*p).eth_count as isize);
            (*entry).driver = strdup(s);
            if strcmp(s, b"tap\0".as_ptr() as *const c_char) == 0 {
                let mut s2: *const c_char = core::ptr::null();
                if vm_get_str(
                    obj,
                    b"ifname\0".as_ptr() as *const c_char,
                    &mut s2 as *mut *const c_char,
                ) < 0
                {
                    return 2;
                }
                let entry2: *mut VMEthEntry = (*p).tab_eth.as_mut_ptr().offset((*p).eth_count as isize);
                (*entry2).ifname = strdup(s2);
            }
            (*p).eth_count += 1;
        }
        0
    }
}
}

// ── `p->display_device = ((void *)0) ; obj = json_object_get(cfg, "display0");` · machine.c:355-356 · chunk 1415 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->display_device = 
//                            ((void *)0)
//                                ;
//         obj = json_object_get(cfg, "display0");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1415 {
// Port of machine.c lines 355-356 (inside virt_machine_parse_config):
//   p->display_device = NULL;
//   obj = json_object_get(cfg, "display0");

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int};

// C: typedef int BOOL;
pub type BOOL = c_int;

// C: typedef enum { ... } JSONTypeEnum;
pub type JSONTypeEnum = c_int;

#[repr(C)]
pub struct JSONString {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct JSONObject {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct JSONArray {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueU {
    pub str: *mut JSONString,
    pub int32: c_int,
    pub b: BOOL,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub r#type: JSONTypeEnum,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1415(
    display_device: *mut *mut c_char,
    cfg: JSONValue,
) -> JSONValue {
    // p->display_device = NULL;
    if !display_device.is_null() {
        *display_device = core::ptr::null_mut();
    }
    // obj = json_object_get(cfg, "display0");
    let name: &[u8] = b"display0\0";
    json_object_get(cfg, name.as_ptr() as *const c_char)
}
}

// ── `if (!json_is_undefined(obj))` · machine.c:357-370 · chunk 1416 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!json_is_undefined(obj)) {
//             if (vm_get_str(obj, "device", &str) < 0)
//                 goto tag_fail;
//             p->display_device = strdup(str);
//             if (vm_get_int(obj, "width", &p->width) < 0)
//                 goto tag_fail;
//             if (vm_get_int(obj, "height", &p->height) < 0)
//                 goto tag_fail;
//             if (vm_get_str_opt(obj, "vga_bios", &str) < 0)
//                 goto tag_fail;
//             if (str) {
//                 p->files[VM_FILE_VGA_BIOS].filename = strdup(str);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1416 {
// Port of the `display0` config block of virt_machine_parse_config.
use core::ffi::{c_char, c_int};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const core::ffi::c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut core::ffi::c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn vm_error(fmt: *const c_char, ...);
}

const FMT_EXPECTING: &[u8] = b"expecting '%s' property\n\0";
const FMT_STR_EXPECTED: &[u8] = b"%s: string expected\n\0";
const TAG_DEVICE: &[u8] = b"device\0";
const TAG_WIDTH: &[u8] = b"width\0";
const TAG_HEIGHT: &[u8] = b"height\0";
const TAG_VGA_BIOS: &[u8] = b"vga_bios\0";

// Port of the static C helper vm_get_str2 (with is_opt as a flag).
unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    let val = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = core::ptr::null();
            0
        } else {
            vm_error(FMT_EXPECTING.as_ptr() as *const c_char, name);
            -1
        }
    } else if val.type_ != JSON_STR {
        vm_error(FMT_STR_EXPECTED.as_ptr() as *const c_char, name);
        -1
    } else {
        let s = val.u.str_;
        if s.is_null() {
            *pstr = core::ptr::null();
            return 0;
        }
        *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1416(
    p: *mut VirtMachineParams,
    obj: JSONValue,
    pstr: *mut *const c_char,
) -> c_int {
    if obj.type_ == JSON_UNDEFINED {
        return 0;
    }
    if vm_get_str2(obj, TAG_DEVICE.as_ptr() as *const c_char, pstr, 0) < 0 {
        return -1;
    }
    (*p).display_device = strdup(*pstr);
    if vm_get_int(
        obj,
        TAG_WIDTH.as_ptr() as *const c_char,
        core::ptr::addr_of_mut!((*p).width),
    ) < 0
    {
        return -1;
    }
    if vm_get_int(
        obj,
        TAG_HEIGHT.as_ptr() as *const c_char,
        core::ptr::addr_of_mut!((*p).height),
    ) < 0
    {
        return -1;
    }
    if vm_get_str2(obj, TAG_VGA_BIOS.as_ptr() as *const c_char, pstr, 1) < 0 {
        return -1;
    }
    let str_val = *pstr;
    if !str_val.is_null() {
        (*p).files[1].filename = strdup(str_val);
    }
    0
}
}

// ── `if (vm_get_str_opt(cfg, "input_device", &str) < 0)` · machine.c:372-373 · chunk 1417 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, "input_device", &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1417 {
// Port of the "input_device" lookup step of virt_machine_parse_config.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int};

// C: typedef enum { JSON_STR, JSON_INT, JSON_OBJ, JSON_ARRAY, JSON_BOOL,
//                  JSON_NULL, JSON_UNDEFINED, JSON_EXCEPTION } JSONTypeEnum;
pub const JSON_STR: c_int = 0;
pub const JSON_UNDEFINED: c_int = 6;

// C: typedef struct { int len; char data[0]; } JSONString;
#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

// C: union { JSONString *str; int int32; BOOL b; JSONObject *obj;
//            JSONArray *array; } u;
#[repr(C)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

// C: typedef struct JSONValue { JSONTypeEnum type; union {...} u; } JSONValue;
#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

static INPUT_DEVICE: [u8; 13] = *b"input_device\0";
static EXPECTING_PROPERTY: [u8; 25] = *b"expecting '%s' property\n\0";
static STRING_EXPECTED: [u8; 21] = *b"%s: string expected\n\0";

// Port of the file-static C helper vm_get_str2 (no linker symbol exists).
fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    unsafe {
        let val = json_object_get(obj, name);
        if val.type_ == JSON_UNDEFINED {
            if is_opt != 0 {
                *pstr = core::ptr::null();
                return 0;
            } else {
                vm_error(EXPECTING_PROPERTY.as_ptr() as *const c_char, name);
                return -1;
            }
        }
        if val.type_ != JSON_STR {
            vm_error(STRING_EXPECTED.as_ptr() as *const c_char, name);
            return -1;
        }
        *pstr = (*val.u.str_).data.as_ptr();
        return 0;
    }
}

// Original target:
//   if (vm_get_str_opt(cfg, "input_device", &str) < 0)
//           goto tag_fail;
#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1417(
    pcfg: *const JSONValue,
    pstr: *mut *const c_char,
) -> c_int {
    unsafe {
        if pcfg.is_null() {
            return -1;
        }
        vm_get_str2(
            core::ptr::read(pcfg),
            INPUT_DEVICE.as_ptr() as *const c_char,
            pstr,
            1,
        )
    }
}
}

// ── `p->input_device = strdup_null(str);` · machine.c:374-374 · chunk 1418 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->input_device = strdup_null(str);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1418 {
// Port of the C helper `strdup_null` applied to `p->input_device = strdup_null(str);`
// from virt_machine_parse_config().
use core::ffi::c_char;
use core::ptr::null_mut;

extern "C" {
    // libc strdup (declared in string.h in the preprocessed translation unit)
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1418(s: *const c_char) -> *mut c_char {
    // static char *strdup_null(const char *str) { if (!str) return NULL; else return strdup(str); }
    if s.is_null() {
        null_mut()
    } else {
        unsafe { strdup(s) }
    }
}
}

// ── `if (vm_get_str_opt(cfg, "accel", &str) < 0)` · machine.c:376-377 · chunk 1419 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vm_get_str_opt(cfg, "accel", &str) < 0)
//             goto tag_fail;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1419 {
// Port of the "accel" property lookup from virt_machine_parse_config.
// vm_get_str_opt is a static C function, so its logic (vm_get_str2 with
// is_opt = TRUE) is reproduced here as a private Rust function.

use core::ffi::{c_char, c_int, c_void};

const JSON_STR: c_int = 0;
const JSON_UNDEFINED: c_int = 6;

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
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONUnion,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn vm_error(fmt: *const c_char, ...);
}

// Equivalent of vm_get_str2(obj, name, pstr, is_opt = 1).
unsafe fn vm_get_str2(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char,
                      is_opt: c_int) -> c_int {
    let val = json_object_get(obj, name);
    if val.type_ == JSON_UNDEFINED {
        if is_opt != 0 {
            *pstr = core::ptr::null();
            return 0;
        } else {
            vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
            return -1;
        }
    }
    if val.type_ != JSON_STR {
        vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
        return -1;
    }
    let s = val.u.str_;
    *pstr = core::ptr::addr_of!((*s).data) as *const c_char;
    return 0;
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1419(cfg: JSONValue, name: *const c_char,
                                                      pstr: *mut *const c_char) -> c_int {
    unsafe { vm_get_str2(cfg, name, pstr, 1) }
}
}

// ── `if (str)` · machine.c:378-387 · chunk 1420 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (str) {
//             if (!strcmp(str, "none")) {
//                 p->accel_enable = FALSE;
//             } else if (!strcmp(str, "auto")) {
//                 p->accel_enable = TRUE;
//             } else {
//                 vm_error("unsupported 'accel' config: %s\n", str);
//                 return -1;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1420 {
use core::ffi::{c_char, c_int};

extern "C" {
    // void vm_error(const char *fmt, ...);
    fn vm_error(fmt: *const c_char, ...);
}

const FALSE: c_int = 0;
const TRUE: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1420(
    accel_enable: *mut c_int,
    str_: *const c_char,
) -> c_int {
    if str_.is_null() {
        return 0;
    }
    let bytes = core::ffi::CStr::from_ptr(str_).to_bytes();
    if bytes == b"none" {
        if !accel_enable.is_null() {
            *accel_enable = FALSE;
        }
    } else if bytes == b"auto" {
        if !accel_enable.is_null() {
            *accel_enable = TRUE;
        }
    } else {
        vm_error(
            b"unsupported 'accel' config: %s\n\0".as_ptr() as *const c_char,
            str_,
        );
        return -1;
    }
    0
}
}

// ── `tag_name = "rtc_local_time"; el = json_object_get(cfg, tag_name);` · machine.c:389-390 · chunk 1421 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tag_name = "rtc_local_time";
//         el = json_object_get(cfg, tag_name);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1421 {
// Port of machine.c lines 389-390 inside virt_machine_parse_config():
//   tag_name = "rtc_local_time";
//   el = json_object_get(cfg, tag_name);

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C union inside JSONValue.
#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONUnion {
    pub str_: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

// Mirror of the C JSONValue struct.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONUnion,
}

extern "C" {
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
}

static TAG_RTC_LOCAL_TIME: [u8; 15] = *b"rtc_local_time\0";

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1421(
    tag_name: *mut *const c_char,
    cfg: JSONValue,
) -> JSONValue {
    let name = TAG_RTC_LOCAL_TIME.as_ptr() as *const c_char;
    unsafe {
        if !tag_name.is_null() {
            *tag_name = name;
        }
        json_object_get(cfg, name)
    }
}
}

// ── `if (!json_is_undefined(el))` · machine.c:391-397 · chunk 1422 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!json_is_undefined(el)) {
//             if (el.type != JSON_BOOL) {
//                 vm_error("%s: boolean expected\n", tag_name);
//                 goto tag_fail;
//             }
//             p->rtc_local_time = el.u.b;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1422 {
// Port of the "rtc_local_time" handling inside virt_machine_parse_config.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

const JSON_BOOL: c_int = 4;
const JSON_UNDEFINED: c_int = 6;

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
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
}

/// Returns 0 on success; -1 means the caller must `goto tag_fail`.
#[no_mangle]
pub unsafe extern "C" fn virt_machine_parse_config__part1422(
    el: *const JSONValue,
    tag_name: *const c_char,
    p: *mut VirtMachineParams,
) -> c_int {
    if el.is_null() {
        return 0;
    }
    let ev = &*el;
    // if (!json_is_undefined(el)) { ... }
    if ev.type_ != JSON_UNDEFINED {
        if ev.type_ != JSON_BOOL {
            vm_error(
                b"%s: boolean expected\n\0".as_ptr() as *const c_char,
                tag_name,
            );
            return -1;
        }
        if !p.is_null() {
            (*p).rtc_local_time = ev.u.b;
        }
    }
    0
}
}

// ── `json_free(cfg); return 0;` · machine.c:399-400 · chunk 1423 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     json_free(cfg);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1423 {
// Port of the tail of virt_machine_parse_config: free the config JSON value
// and report success to the caller.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONObject {
    pub len: c_int,
    pub size: c_int,
    pub props: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub len: c_int,
    pub size: c_int,
    pub tab: *mut c_void,
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

extern "C" {
    fn json_free(val: JSONValue);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1423(cfg: JSONValue) -> c_int {
    unsafe {
        json_free(cfg);
    }
    0
}
}

// ── `tag_fail: json_free(cfg);` · machine.c:401-402 · chunk 1424 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tag_fail:
//         json_free(cfg);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1424 {
// Port of `virt_machine_parse_config`'s tag_fail epilogue:
//     tag_fail:
//         json_free(cfg);
// The enclosing C function keeps the `return -1;` that follows.

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    // char data[0] flexible array member: zero-sized in the layout
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONProperty {
    pub name: JSONValue,
    pub value: JSONValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

extern "C" {
    fn json_free(val: JSONValue);
}

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1424(cfg: JSONValue) {
    unsafe {
        json_free(cfg);
    }
}
}

// ── `return -1;` · machine.c:403-403 · chunk 1425 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1425 {
// Ported piece of virt_machine_parse_config: the return -1 of the tag_fail path.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virt_machine_parse_config__part1425() -> c_int {
    -1
}
}

// ── `get_file_path` · machine.c:424-446 · chunk 1429 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     char *get_file_path(const char *base_filename, const char *filename)
//     {
//         int len, len1;
//         char *fname, *p;
//     
//         if (!base_filename)
//             goto done;
//         if (
//            _Generic (0 ? (
//            filename
//            ) : (void *) 1, const void *: (const char *) (strchr (
//            filename
//            , 
//            ':'
//            )), default: strchr (
//            filename
//            , 
//            ':'
//            ))
//                                 )
//             goto done;
//         if (filename[0] == '/')
//             goto done;
//         p = 
//            _Generic (0 ? (
//            base_filename
//            ) : (void *) 1, const void *: (const char *) (strrchr (
//            base_filename
//            , 
//            '/'
//            )), default: strrchr (
//            base_filename
//            , 
//            '/'
//            ))
//                                       ;
//         if (!p) {
//         done:
//             return strdup(filename);
//         }
//         len = p + 1 - base_filename;
//         len1 = strlen(filename);
//         fname = malloc(len + len1 + 1);
//         memcpy(fname, base_filename, len);
//         memcpy(fname + len, filename, len1 + 1);
//         return fname;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1429 {
// Rust rewrite of get_file_path from machine.c
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn get_file_path(
    base_filename: *const c_char,
    filename: *const c_char,
) -> *mut c_char {
    unsafe {
        if base_filename.is_null() {
            return strdup(filename);
        }
        if !strchr(filename, ':' as c_int).is_null() {
            // full URL
            return strdup(filename);
        }
        if *filename == b'/' as c_char {
            return strdup(filename);
        }
        let p: *mut c_char = strrchr(base_filename, '/' as c_int);
        if p.is_null() {
            return strdup(filename);
        }
        let len: c_int = (p as isize + 1 - base_filename as isize) as c_int;
        let len1: c_int = strlen(filename) as c_int;
        let fname = malloc((len + len1 + 1) as usize) as *mut c_char;
        memcpy(
            fname as *mut c_void,
            base_filename as *const c_void,
            len as usize,
        );
        memcpy(
            fname.offset(len as isize) as *mut c_void,
            filename as *const c_void,
            (len1 + 1) as usize,
        );
        fname
    }
}
}

// ── `if (!base_filename)` · machine.c:429-430 · chunk 1430 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!base_filename)
//             goto done;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1430 {
// Portion of get_file_path: the NULL check on base_filename.
// Returns nonzero when the C code must `goto done`.

use core::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn get_file_path__part1430(base_filename: *const c_char) -> c_int {
    if base_filename.is_null() {
        1
    } else {
        0
    }
}
}

// ── `if ( _Generic (0 ? ( filename ) : (void *) 1, const void *: (const char *) (strchr ( filename ,…` · machine.c:431-432 · chunk 1431 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            _Generic (0 ? (
//            filename
//            ) : (void *) 1, const void *: (const char *) (strchr (
//            filename
//            , 
//            ':'
//            )), default: strchr (
//            filename
//            , 
//            ':'
//            ))
//                                 )
//             goto done;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1431 {
// Port of the TARGET from get_file_path: does the filename contain a ':' ?
use core::ffi::{c_char, c_int};

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn get_file_path__part1431(filename: *const c_char) -> c_int {
    unsafe {
        if strchr(filename, ':' as c_int).is_null() {
            0
        } else {
            1
        }
    }
}
}

// ── `if (filename[0] == '/')` · machine.c:433-434 · chunk 1432 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (filename[0] == '/')
//             goto done;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1432 {
use core::ffi::{c_char, c_int};

// Returns nonzero when `filename` starts with '/', mirroring
// `if (filename[0] == '/')`.
#[no_mangle]
pub extern "C" fn get_file_path__part1432(filename: *const c_char) -> c_int {
    if filename.is_null() {
        return 0;
    }
    let first = unsafe { *filename };
    if first == b'/' as c_char {
        1
    } else {
        0
    }
}
}

// ── `p = _Generic (0 ? ( base_filename ) : (void *) 1, const void *: (const char *) (strrchr ( base_…` · machine.c:435-435 · chunk 1433 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     p = 
//            _Generic (0 ? (
//            base_filename
//            ) : (void *) 1, const void *: (const char *) (strrchr (
//            base_filename
//            , 
//            '/'
//            )), default: strrchr (
//            base_filename
//            , 
//            '/'
//            ))
//                                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1433 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn get_file_path__part1433(base_filename: *const c_char) -> *mut c_char {
    unsafe { strrchr(base_filename, '/' as c_int) }
}
}

// ── `if (!p)` · machine.c:436-439 · chunk 1434 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!p) {
//         done:
//             return strdup(filename);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1434 {
// Port of the `done:` exit of get_file_path: duplicate the filename.
use core::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn get_file_path__part1434(filename: *const c_char) -> *mut c_char {
    unsafe { strdup(filename) }
}
}

// ── `len = p + 1 - base_filename; len1 = strlen(filename); fname = malloc(len + len1 + 1); memcpy(fn…` · machine.c:440-445 · chunk 1435 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     len = p + 1 - base_filename;
//         len1 = strlen(filename);
//         fname = malloc(len + len1 + 1);
//         memcpy(fname, base_filename, len);
//         memcpy(fname + len, filename, len1 + 1);
//         return fname;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1435 {
// Port of the tail of get_file_path: build "dir/" + filename.
use core::ffi::{c_char, c_int};

extern "C" {
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn strlen(s: *const c_char) -> usize;
}

#[no_mangle]
pub extern "C" fn get_file_path__part1435(
    base_filename: *const c_char,
    filename: *const c_char,
    p: *const c_char,
    len_out: *mut c_int,
    len1_out: *mut c_int,
) -> *mut c_char {
    unsafe {
        // len = p + 1 - base_filename;  (pointer difference in chars, truncated to int)
        let len: c_int = ((p as isize).wrapping_add(1).wrapping_sub(base_filename as isize)) as c_int;
        // len1 = strlen(filename);
        let len1: c_int = strlen(filename) as c_int;

        if !len_out.is_null() {
            *len_out = len;
        }
        if !len1_out.is_null() {
            *len1_out = len1;
        }

        // fname = malloc(len + len1 + 1);
        let total = len.wrapping_add(len1).wrapping_add(1);
        let fname = malloc(total as usize) as *mut c_char;

        // memcpy(fname, base_filename, len);
        memcpy(
            fname as *mut core::ffi::c_void,
            base_filename as *const core::ffi::c_void,
            len as usize,
        );
        // memcpy(fname + len, filename, len1 + 1);
        memcpy(
            fname.offset(len as isize) as *mut core::ffi::c_void,
            filename as *const core::ffi::c_void,
            len1.wrapping_add(1) as usize,
        );

        fname
    }
}
}

// ── `load_file` · machine.c:456-478 · chunk 1436 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static int load_file(uint8_t **pbuf, const char *filename)
//     {
//         FILE *f;
//         int size;
//         uint8_t *buf;
//     
//         f = fopen(filename, "rb");
//         if (!f) {
//             perror(filename);
//             exit(1);
//         }
//         fseek(f, 0, 
//                    2
//                            );
//         size = ftell(f);
//         fseek(f, 0, 
//                    0
//                            );
//         buf = malloc(size);
//         if (fread(buf, 1, size, f) != size) {
//             fprintf(
//                    stderr
//                          , "%s: read error\n", filename);
//             exit(1);
//         }
//         fclose(f);
//         *pbuf = buf;
//         return size;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1436 {
use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn malloc(size: usize) -> *mut c_void;
    static stderr: *mut FILE;
}

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;

/* return -1 if error. */
#[no_mangle]
pub unsafe extern "C" fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f: *mut FILE = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    // C assigns the long returned by ftell() to an int: same truncation here.
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, SEEK_SET);
    let buf: *mut u8 = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        fprintf(
            stderr,
            b"%s: read error\n\0".as_ptr() as *const c_char,
            filename,
        );
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}
}

// ── `FILE *f; int size; uint8_t *buf; f = fopen(filename, "rb");` · machine.c:458-462 · chunk 1437 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FILE *f;
//         int size;
//         uint8_t *buf;
//     
//         f = fopen(filename, "rb");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1437 {
// Ported part of load_file(): the declarations plus the initial fopen() call.
// glibc maps fopen() to fopen64() with file-offset-bits enabled, which is the
// symbol the rest of the translation unit already references, so use it too.
use core::ffi::{c_char, c_void};

extern "C" {
    fn fopen64(filename: *const c_char, modes: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn load_file__part1437(filename: *const c_char) -> *mut c_void {
    // "rb" as a NUL-terminated byte string; the pointer is valid for the call.
    static MODE: [u8; 3] = [b'r', b'b', 0];
    unsafe { fopen64(filename, MODE.as_ptr() as *const c_char) }
}
}

// ── `if (!f)` · machine.c:463-466 · chunk 1438 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1438 {
// Port of the "file open failed" check inside load_file().
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn load_file__part1438(f: *mut c_void, filename: *const c_char) {
    if f.is_null() {
        unsafe {
            perror(filename);
            exit(1);
        }
    }
}
}

// ── `fseek(f, 0, 2 ); size = ftell(f); fseek(f, 0, 0 ); buf = malloc(size);` · machine.c:467-470 · chunk 1439 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fseek(f, 0, 
//                    2
//                            );
//         size = ftell(f);
//         fseek(f, 0, 
//                    0
//                            );
//         buf = malloc(size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1439 {
// Port of the file-size / buffer-allocation part of load_file().
use core::ffi::{c_char, c_int, c_long, c_void};

extern "C" {
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut c_void) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn load_file__part1439(
    f: *mut c_void,
    size_out: *mut c_int,
    buf_out: *mut *mut u8,
) {
    let _ = core::mem::size_of::<c_char>();
    unsafe {
        // fseek(f, 0, SEEK_END)
        fseek(f, 0, 2);
        // size = ftell(f);   (long -> int conversion, same as C truncation)
        let size: c_int = ftell(f) as c_int;
        // fseek(f, 0, SEEK_SET)
        fseek(f, 0, 0);
        // buf = malloc(size);
        let buf: *mut u8 = malloc(size as usize) as *mut u8;

        if !size_out.is_null() {
            *size_out = size;
        }
        if !buf_out.is_null() {
            *buf_out = buf;
        }
    }
}
}

// ── `if (fread(buf, 1, size, f) != size)` · machine.c:471-474 · chunk 1440 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (fread(buf, 1, size, f) != size) {
//             fprintf(
//                    stderr
//                          , "%s: read error\n", filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1440 {
// Port of the fread/error-reporting piece of load_file.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn fread(ptr: *mut c_void, size: usize, n: usize, stream: *mut c_void) -> usize;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(code: c_int);
    static mut stderr: *mut c_void;
}

#[no_mangle]
pub extern "C" fn load_file__part1440(
    buf: *mut u8,
    size: c_int,
    f: *mut c_void,
    filename: *const c_char,
) {
    unsafe {
        let n: usize = size as usize;
        if fread(buf as *mut c_void, 1, n, f) != n {
            fprintf(
                stderr,
                b"%s: read error\n\0".as_ptr() as *const c_char,
                filename,
            );
            exit(1);
        }
    }
}
}

// ── `fclose(f); *pbuf = buf; return size;` · machine.c:475-477 · chunk 1441 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fclose(f);
//         *pbuf = buf;
//         return size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1441 {
// Portion of load_file: close the stream, publish the buffer, return the size.
use core::ffi::{c_int, c_void};

extern "C" {
    // int fclose(FILE *stream);
    fn fclose(stream: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn load_file__part1441(
    f: *mut c_void,
    pbuf: *mut *mut u8,
    buf: *mut u8,
    size: c_int,
) -> c_int {
    unsafe {
        fclose(f);
        *pbuf = buf;
    }
    size
}
}

// ── `config_load_file_cb` · machine.c:482-492 · chunk 1442 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void config_load_file_cb(void *opaque, int err, void *data, size_t size)
//     {
//         VMConfigLoadState *s = opaque;
//     
//     
//         if (err < 0) {
//             vm_error("Error %d while loading file\n", -err);
//             exit(1);
//         }
//         s->file_load_cb(s->file_load_opaque, data, size);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1442 {
// Rust rewrite of config_load_file_cb (machine.c)
use core::ffi::{c_char, c_int, c_void};

// FSLoadFileCB: void (*)(void *opaque, uint8_t *buf, int buf_len)
type FSLoadFileCB = Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int)>;
type StartCB = Option<unsafe extern "C" fn(*mut c_void)>;

// Mirror of the C VMConfigLoadState (same field order and types).
#[repr(C)]
struct VMConfigLoadState {
    vm_params: *mut c_void,
    start_cb: StartCB,
    opaque: *mut c_void,
    file_load_cb: FSLoadFileCB,
    file_load_opaque: *mut c_void,
    file_index: c_int,
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
}

static ERR_FMT: &[u8] = b"Error %d while loading file\n\0";

#[no_mangle]
pub unsafe extern "C" fn config_load_file_cb(opaque: *mut c_void, err: c_int, data: *mut c_void, size: usize) {
    let s = opaque as *mut VMConfigLoadState;

    if err < 0 {
        vm_error(ERR_FMT.as_ptr() as *const c_char, err.wrapping_neg());
        exit(1);
    }

    // s->file_load_cb(s->file_load_opaque, data, size);
    // size is size_t in the call, converted to int in the callee's prototype.
    let st = &mut *s;
    if let Some(cb) = st.file_load_cb {
        cb(st.file_load_opaque, data as *mut u8, size as c_int);
    }
}
}

// ── `if (err < 0)` · machine.c:487-490 · chunk 1443 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (err < 0) {
//             vm_error("Error %d while loading file\n", -err);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1443 {
// Ported piece of config_load_file_cb: the error check around the file load.
use core::ffi::{c_char, c_int};

extern "C" {
    // void vm_error(const char *fmt, ...);
    fn vm_error(fmt: *const c_char, ...);
    // void exit(int status) __attribute__((noreturn));
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn config_load_file_cb__part1443(err: c_int) {
    if err < 0 {
        // "-err" on a negative int: wrapping negation matches C semantics.
        let nerr: c_int = (0 as c_int).wrapping_sub(err);
        unsafe {
            vm_error(
                b"Error %d while loading file\n\0".as_ptr() as *const c_char,
                nerr,
            );
            exit(1);
        }
    }
}
}

// ── `s->file_load_cb(s->file_load_opaque, data, size);` · machine.c:491-491 · chunk 1444 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->file_load_cb(s->file_load_opaque, data, size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1444 {
// Port of the config_load_file_cb forwarding call.
use core::ffi::{c_int, c_void};

type FSLoadFileCB = unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_len: c_int);

#[repr(C)]
struct VMConfigLoadState {
    vm_params: *mut c_void,
    start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
    file_load_cb: Option<FSLoadFileCB>,
    file_load_opaque: *mut c_void,
    file_index: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn config_load_file_cb__part1444(
    opaque: *mut c_void,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *const VMConfigLoadState;
    if s.is_null() {
        return;
    }
    let cb = (*s).file_load_cb;
    if let Some(f) = cb {
        f((*s).file_load_opaque, data as *mut u8, size as c_int);
    }
}
}

// ── `config_load_file` · machine.c:495-513 · chunk 1445 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void config_load_file(VMConfigLoadState *s, const char *filename,
//                                  FSLoadFileCB *cb, void *opaque)
//     {
//     
//     
//         if (is_url(filename)) {
//             s->file_load_cb = cb;
//             s->file_load_opaque = opaque;
//             fs_wget(filename, 
//                              ((void *)0)
//                                  , 
//                                    ((void *)0)
//                                        , s, config_load_file_cb, TRUE);
//         } else
//     
//         {
//             uint8_t *buf;
//             int size;
//             size = load_file(&buf, filename);
//             cb(opaque, buf, size);
//             free(buf);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1445 {
// Rust port of `config_load_file` (machine.c) and the static helpers it uses
// (`load_file` and `config_load_file_cb`), which have no linker symbol.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

// Opaque mirror of C `FILE`.
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

// WGetWriteCallback: void (*)(void *opaque, int err, void *data, size_t size)
type WGetWriteCallback =
    Option<unsafe extern "C" fn(opaque: *mut c_void, err: c_int, data: *mut c_void, size: usize)>;

// FSLoadFileCB: void (*)(void *opaque, uint8_t *buf, int buf_len)
type FSLoadFileCB =
    Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_len: c_int)>;

// Mirror of the C struct VMConfigLoadState (field order and types matter).
#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut c_void,
    pub start_cb: Option<unsafe extern "C" fn(opaque: *mut c_void)>,
    pub opaque: *mut c_void,
    pub file_load_cb: FSLoadFileCB,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;
const TRUE: c_int = 1;

extern "C" {
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: WGetWriteCallback,
        single_write: c_int,
    ) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, n: usize, stream: *mut FILE) -> usize;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    static mut stderr: *mut FILE;
}

// Port of the static C function `load_file`: returns the file size.
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let mode: *const c_char = b"rb\0".as_ptr() as *const c_char;

    let f = fopen(filename, mode);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, SEEK_SET);
    let buf = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        let fmt: *const c_char = b"%s: read error\n\0".as_ptr() as *const c_char;
        fprintf(stderr, fmt, filename);
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

// Port of the static C function `config_load_file_cb`.
unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *mut VMConfigLoadState;

    if err < 0 {
        let fmt: *const c_char = b"Error %d while loading file\n\0".as_ptr() as *const c_char;
        vm_error(fmt, err.wrapping_neg());
        exit(1);
    }
    let cb = (*s).file_load_cb;
    let cb_opaque = (*s).file_load_opaque;
    if let Some(f) = cb {
        f(cb_opaque, data as *mut u8, size as c_int);
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: FSLoadFileCB,
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = cb;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            Some(config_load_file_cb),
            TRUE,
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size: c_int = load_file(&mut buf, filename);
        if let Some(f) = cb {
            f(opaque, buf, size);
        }
        free(buf as *mut c_void);
    }
}
}

// ── `if (is_url(filename)) … else` · machine.c:500-512 · chunk 1446 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_url(filename)) {
//             s->file_load_cb = cb;
//             s->file_load_opaque = opaque;
//             fs_wget(filename, 
//                              ((void *)0)
//                                  , 
//                                    ((void *)0)
//                                        , s, config_load_file_cb, TRUE);
//         } else
//     
//         {
//             uint8_t *buf;
//             int size;
//             size = load_file(&buf, filename);
//             cb(opaque, buf, size);
//             free(buf);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1446 {
// Ported piece: the if/else body of config_load_file (machine.c)
use core::ffi::{c_char, c_int, c_void};

type FSLoadFileCB = Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int)>;
type WGetWriteCallback = Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, usize)>;
type LoadFileFn = Option<unsafe extern "C" fn(*mut *mut u8, *const c_char) -> c_int>;

#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut c_void,
    pub start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub opaque: *mut c_void,
    pub file_load_cb: FSLoadFileCB,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: WGetWriteCallback,
        single_write: c_int,
    ) -> *mut c_void;
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn config_load_file__part1446(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: FSLoadFileCB,
    opaque: *mut c_void,
    load_file_fn: LoadFileFn,
    wget_cb: WGetWriteCallback,
) {
    if is_url(filename) != 0 {
        // asynchronously loaded (URL): record the callback and start the transfer
        (*s).file_load_cb = cb;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            wget_cb,
            1, // TRUE
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size: c_int = match load_file_fn {
            Some(f) => f(&mut buf, filename),
            None => 0,
        };
        if let Some(f) = cb {
            f(opaque, buf, size);
        }
        free(buf as *mut c_void);
    }
}
}

// ── `virt_machine_load_config_file` · machine.c:515-529 · chunk 1447 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     void virt_machine_load_config_file(VirtMachineParams *p,
//                                        const char *filename,
//                                        void (*start_cb)(void *opaque),
//                                        void *opaque)
//     {
//         VMConfigLoadState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->vm_params = p;
//         s->start_cb = start_cb;
//         s->opaque = opaque;
//         p->cfg_filename = strdup(filename);
//     
//         config_load_file(s, filename, config_file_loaded, s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1447 {
// Rust port of virt_machine_load_config_file (machine.c lines 515-529).
//
// The TARGET calls the file-static helper config_load_file(), which in turn
// calls the file-static config_file_loaded()/config_additional_file_load()/
// config_additional_file_load_cb()/virt_machine_parse_config()/load_file()
// chain.  None of those have a linker symbol, so their logic is ported here.
// Everything else (mallocz, strdup, free, is_url, fs_wget, get_file_path,
// vm_error, vm_get_int, json_*, dbuf_*, libc stdio) is called through extern.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_assignments)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};
use std::ffi::CString;

// ---------------------------------------------------------------------------
// constants
// ---------------------------------------------------------------------------

pub type BOOL = c_int;

const VM_FILE_BIOS: usize = 0;
const VM_FILE_VGA_BIOS: usize = 1;
const VM_FILE_KERNEL: usize = 2;
const VM_FILE_INITRD: usize = 3;
const VM_FILE_COUNT: c_int = 4;

const JSON_STR: c_int = 0;
const JSON_BOOL: c_int = 4;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

const VM_CONFIG_VERSION: c_int = 1;
const MAX_DRIVE_DEVICE: c_int = 4;
const MAX_ETH_DEVICE: c_int = 1;

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;

const TRUE_VAL: BOOL = 1;

// ---------------------------------------------------------------------------
// C type mirrors (exact field order)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut c_void>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut c_void)>,
    pub virt_machine_get_sleep_duration: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut c_void) -> BOOL>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut c_void, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut c_void, BOOL, u16)>,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_local_time: BOOL,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: BOOL,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

type FSLoadFileCB = unsafe extern "C" fn(*mut c_void, *mut u8, c_int);
type WGetWriteCallback = unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, usize);

#[repr(C)]
struct VMConfigLoadState {
    vm_params: *mut VirtMachineParams,
    start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
    file_load_cb: Option<FSLoadFileCB>,
    file_load_opaque: *mut c_void,
    file_index: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    // char data[0]; flexible array member
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONUnion {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub type_: c_int, // JSONTypeEnum
    pub u: JSONUnion,
}

#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

#[repr(C)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

// ---------------------------------------------------------------------------
// extern declarations
// ---------------------------------------------------------------------------

extern "C" {
    // libc
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn fopen64(filename: *const c_char, modes: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut c_void) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn perror(s: *const c_char);
    fn time(t: *mut c_long) -> c_long;
    fn localtime_r(t: *const c_long, tm: *mut Tm) -> *mut Tm;

    // tinyemu
    fn mallocz(size: usize) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn is_url(path: *const c_char) -> BOOL;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<WGetWriteCallback>,
        single_write: BOOL,
    ) -> *mut c_void;
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;

    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;

    fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue;
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn json_free(val: JSONValue);
    fn json_get_error(val: JSONValue) -> *const c_char;

    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, str_: *const c_char);

    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;
}

// ---------------------------------------------------------------------------
// small C string helpers (no panics)
// ---------------------------------------------------------------------------

unsafe fn c_strlen(s: *const u8) -> usize {
    unsafe {
        let mut n = 0usize;
        while *s.add(n) != 0 {
            n += 1;
        }
        n
    }
}

unsafe fn bytes_eq(a: *const u8, b: *const u8, len: usize) -> bool {
    unsafe {
        let mut i = 0usize;
        while i < len {
            if *a.add(i) != *b.add(i) {
                return false;
            }
            i += 1;
        }
        true
    }
}

unsafe fn cstr_eq_cstr(a: *const u8, b: *const u8) -> bool {
    unsafe {
        let mut i = 0usize;
        loop {
            let ca = *a.add(i);
            let cb = *b.add(i);
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

unsafe fn cstr_eq_lit(a: *const c_char, lit: &[u8]) -> bool {
    unsafe {
        if a.is_null() {
            return false;
        }
        let p = a as *const u8;
        let mut i = 0usize;
        while i < lit.len() {
            if *p.add(i) != lit[i] {
                return false;
            }
            i += 1;
        }
        *p.add(lit.len()) == 0
    }
}

// snprintf(buf, sizeof(buf), "<prefix>%d", n)
fn build_indexed_name(prefix: &str, n: c_int) -> [u8; 256] {
    let mut out = [0u8; 256];
    let s = format!("{}{}", prefix, n);
    let bytes = s.as_bytes();
    let len = if bytes.len() > 255 { 255 } else { bytes.len() };
    let mut i = 0usize;
    while i < len {
        out[i] = bytes[i];
        i += 1;
    }
    out
}

fn build_dev_root(n: c_int) -> [u8; 256] {
    let mut out = [0u8; 256];
    let s = if n == 0 {
        String::from("/dev/root")
    } else {
        format!("/dev/root{}", n)
    };
    let bytes = s.as_bytes();
    let len = if bytes.len() > 255 { 255 } else { bytes.len() };
    let mut i = 0usize;
    while i < len {
        out[i] = bytes[i];
        i += 1;
    }
    out
}

unsafe fn json_string_data(val: JSONValue) -> *const c_char {
    unsafe {
        let s = val.u.str_;
        (s as *const u8).add(size_of::<JSONString>()) as *const c_char
    }
}

// ---------------------------------------------------------------------------
// ported static helpers of machine.c
// ---------------------------------------------------------------------------

unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: BOOL,
) -> c_int {
    unsafe {
        let val = json_object_get(obj, name);
        if val.type_ == JSON_UNDEFINED {
            if is_opt != 0 {
                *pstr = null::<c_char>();
                return 0;
            } else {
                vm_error(b"expecting '%s' property\n\0".as_ptr() as *const c_char, name);
                return -1;
            }
        }
        if val.type_ != JSON_STR {
            vm_error(b"%s: string expected\n\0".as_ptr() as *const c_char, name);
            return -1;
        }
        *pstr = json_string_data(val);
        0
    }
}

unsafe fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    unsafe { vm_get_str2(obj, name, pstr, 0) }
}

unsafe fn vm_get_str_opt(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    unsafe { vm_get_str2(obj, name, pstr, TRUE_VAL) }
}

unsafe fn strdup_null(s: *const c_char) -> *mut c_char {
    unsafe {
        if s.is_null() {
            null_mut::<c_char>()
        } else {
            strdup(s)
        }
    }
}

unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> BOOL {
    unsafe {
        let mut p = name_list as *const u8;
        loop {
            let mut q = p;
            while *q != 0 && *q != b',' {
                q = q.add(1);
            }
            if *q == 0 {
                if cstr_eq_cstr(name as *const u8, p) {
                    return 1;
                }
                return 0;
            } else {
                let len = q as usize - p as usize;
                let namelen = c_strlen(name as *const u8);
                if len == namelen && bytes_eq(name as *const u8, p, len) {
                    return 1;
                }
                p = q.add(1);
            }
        }
    }
}

unsafe fn virt_machine_find_class(machine_name: *const c_char) -> *const VirtMachineClass {
    unsafe {
        let list: [*const VirtMachineClass; 3] = [
            &riscv_machine_class as *const VirtMachineClass,
            &pc_machine_class as *const VirtMachineClass,
            null::<VirtMachineClass>(),
        ];
        let mut i = 0usize;
        while i < list.len() {
            let vmc = *list.get_unchecked(i);
            if !vmc.is_null() {
                let names = (*vmc).machine_names;
                if find_name(machine_name, names) != 0 {
                    return vmc;
                }
            }
            i += 1;
        }
        null::<VirtMachineClass>()
    }
}

unsafe fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    unsafe {
        let mut dbuf: DynBuf = zeroed();
        dbuf_init(addr_of_mut!(dbuf));

        let mut p = cmdline as *const u8;
        while *p != 0 {
            if *p == b'$' && *p.add(1) == b'{' {
                p = p.add(2);
                let mut var_name = [0u8; 32];
                let mut q = 0usize;
                while *p != 0 && *p != b'}' {
                    if q < 31 {
                        var_name[q] = *p;
                        q += 1;
                    }
                    p = p.add(1);
                }
                var_name[q] = 0;
                if *p == b'}' {
                    p = p.add(1);
                }
                if cstr_eq_lit(var_name.as_ptr() as *const c_char, b"TZ") {
                    let mut ti: c_long = 0;
                    time(addr_of_mut!(ti));
                    let mut tm: Tm = zeroed();
                    localtime_r(addr_of_mut!(ti) as *const c_long, addr_of_mut!(tm));
                    let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                    let mut sg: u8 = b'-';
                    if n < 0 {
                        sg = b'+';
                        n = -n;
                    }
                    let s = format!("UTC{}{:02}:{:02}", sg as char, n / 60, n % 60);
                    if let Ok(cs) = CString::new(s) {
                        dbuf_putstr(addr_of_mut!(dbuf), cs.as_ptr());
                    }
                }
            } else {
                dbuf_putc(addr_of_mut!(dbuf), *p);
                p = p.add(1);
            }
        }
        dbuf_putc(addr_of_mut!(dbuf), 0);
        dbuf.buf as *mut c_char
    }
}

unsafe fn tag_fail(cfg: JSONValue) -> c_int {
    unsafe {
        json_free(cfg);
    }
    -1
}

unsafe fn virt_machine_parse_config(
    p: *mut VirtMachineParams,
    config_file_str: *mut c_char,
    len: c_int,
) -> c_int {
    unsafe {
        let mut version: c_int = 0;
        let mut val: c_int = 0;
        let mut strp: *const c_char = null::<c_char>();
        let mut tag_name: *const c_char;

        let cfg = json_parse_value_len(config_file_str, len);
        if cfg.type_ == JSON_EXCEPTION {
            vm_error(
                b"error: %s\n\0".as_ptr() as *const c_char,
                json_get_error(cfg),
            );
            json_free(cfg);
            return -1;
        }

        if vm_get_int(
            cfg,
            b"version\0".as_ptr() as *const c_char,
            addr_of_mut!(version),
        ) < 0
        {
            return tag_fail(cfg);
        }
        if version != VM_CONFIG_VERSION {
            if version > VM_CONFIG_VERSION {
                vm_error(
                    b"The emulator is too old to run this VM: please upgrade\n\0".as_ptr()
                        as *const c_char,
                );
            } else {
                vm_error(
                    b"The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n\0"
                        .as_ptr() as *const c_char,
                );
            }
            return -1;
        }

        if vm_get_str(
            cfg,
            b"machine\0".as_ptr() as *const c_char,
            addr_of_mut!(strp),
        ) < 0
        {
            return tag_fail(cfg);
        }
        (*p).machine_name = strdup(strp);
        (*p).vmc = virt_machine_find_class((*p).machine_name);
        if (*p).vmc.is_null() {
            vm_error(
                b"Unknown machine name: %s\n\0".as_ptr() as *const c_char,
                (*p).machine_name,
            );
            return tag_fail(cfg);
        }
        if let Some(vmc) = (*p).vmc.as_ref() {
            if let Some(f) = vmc.virt_machine_set_defaults {
                f(p);
            }
        }

        tag_name = b"memory_size\0".as_ptr() as *const c_char;
        if vm_get_int(cfg, tag_name, addr_of_mut!(val)) < 0 {
            return tag_fail(cfg);
        }
        (*p).ram_size = (val as u64) << 20;

        tag_name = b"bios\0".as_ptr() as *const c_char;
        if vm_get_str_opt(cfg, tag_name, addr_of_mut!(strp)) < 0 {
            return tag_fail(cfg);
        }
        if !strp.is_null() {
            (*p).files[VM_FILE_BIOS].filename = strdup(strp);
        }

        tag_name = b"kernel\0".as_ptr() as *const c_char;
        if vm_get_str_opt(cfg, tag_name, addr_of_mut!(strp)) < 0 {
            return tag_fail(cfg);
        }
        if !strp.is_null() {
            (*p).files[VM_FILE_KERNEL].filename = strdup(strp);
        }

        if vm_get_str_opt(
            cfg,
            b"cmdline\0".as_ptr() as *const c_char,
            addr_of_mut!(strp),
        ) < 0
        {
            return tag_fail(cfg);
        }
        if !strp.is_null() {
            (*p).cmdline = cmdline_subst(strp);
        }

        tag_name = b"initrd\0".as_ptr() as *const c_char;
        if vm_get_str_opt(cfg, tag_name, addr_of_mut!(strp)) < 0 {
            return tag_fail(cfg);
        }
        if !strp.is_null() {
            (*p).files[VM_FILE_INITRD].filename = strdup(strp);
        }

        // drives
        loop {
            let namebuf = build_indexed_name("drive", (*p).drive_count);
            let obj = json_object_get(cfg, namebuf.as_ptr() as *const c_char);
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if (*p).drive_count >= MAX_DRIVE_DEVICE {
                vm_error(b"Too many drives\n\0".as_ptr() as *const c_char);
                return -1;
            }
            if vm_get_str(
                obj,
                b"file\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            let idx = (*p).drive_count as usize;
            if let Some(e) = (*p).tab_drive.get_mut(idx) {
                e.filename = strdup(strp);
            }
            if vm_get_str_opt(
                obj,
                b"device\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            if let Some(e) = (*p).tab_drive.get_mut(idx) {
                e.device = strdup_null(strp);
            }
            (*p).drive_count += 1;
        }

        // filesystems
        loop {
            let namebuf = build_indexed_name("fs", (*p).fs_count);
            let obj = json_object_get(cfg, namebuf.as_ptr() as *const c_char);
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if (*p).fs_count >= MAX_DRIVE_DEVICE {
                vm_error(b"Too many filesystems\n\0".as_ptr() as *const c_char);
                return -1;
            }
            if vm_get_str(
                obj,
                b"file\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            let idx = (*p).fs_count as usize;
            if let Some(e) = (*p).tab_fs.get_mut(idx) {
                e.filename = strdup(strp);
            }
            if vm_get_str_opt(
                obj,
                b"tag\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            let tagbuf: [u8; 256] = if strp.is_null() {
                build_dev_root((*p).fs_count)
            } else {
                [0u8; 256]
            };
            let tagptr: *const c_char = if strp.is_null() {
                tagbuf.as_ptr() as *const c_char
            } else {
                strp
            };
            if let Some(e) = (*p).tab_fs.get_mut(idx) {
                e.tag = strdup(tagptr);
            }
            (*p).fs_count += 1;
        }

        // ethernet interfaces
        loop {
            let namebuf = build_indexed_name("eth", (*p).eth_count);
            let obj = json_object_get(cfg, namebuf.as_ptr() as *const c_char);
            if obj.type_ == JSON_UNDEFINED {
                break;
            }
            if (*p).eth_count >= MAX_ETH_DEVICE {
                vm_error(
                    b"Too many ethernet interfaces\n\0".as_ptr() as *const c_char,
                );
                return -1;
            }
            if vm_get_str(
                obj,
                b"driver\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            let idx = (*p).eth_count as usize;
            if let Some(e) = (*p).tab_eth.get_mut(idx) {
                e.driver = strdup(strp);
            }
            if cstr_eq_lit(strp, b"tap") {
                if vm_get_str(
                    obj,
                    b"ifname\0".as_ptr() as *const c_char,
                    addr_of_mut!(strp),
                ) < 0
                {
                    return tag_fail(cfg);
                }
                if let Some(e) = (*p).tab_eth.get_mut(idx) {
                    e.ifname = strdup(strp);
                }
            }
            (*p).eth_count += 1;
        }

        (*p).display_device = null_mut::<c_char>();
        let obj = json_object_get(cfg, b"display0\0".as_ptr() as *const c_char);
        if obj.type_ != JSON_UNDEFINED {
            if vm_get_str(
                obj,
                b"device\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            (*p).display_device = strdup(strp);
            if vm_get_int(
                obj,
                b"width\0".as_ptr() as *const c_char,
                addr_of_mut!((*p).width),
            ) < 0
            {
                return tag_fail(cfg);
            }
            if vm_get_int(
                obj,
                b"height\0".as_ptr() as *const c_char,
                addr_of_mut!((*p).height),
            ) < 0
            {
                return tag_fail(cfg);
            }
            if vm_get_str_opt(
                obj,
                b"vga_bios\0".as_ptr() as *const c_char,
                addr_of_mut!(strp),
            ) < 0
            {
                return tag_fail(cfg);
            }
            if !strp.is_null() {
                (*p).files[VM_FILE_VGA_BIOS].filename = strdup(strp);
            }
        }

        if vm_get_str_opt(
            cfg,
            b"input_device\0".as_ptr() as *const c_char,
            addr_of_mut!(strp),
        ) < 0
        {
            return tag_fail(cfg);
        }
        (*p).input_device = strdup_null(strp);

        if vm_get_str_opt(
            cfg,
            b"accel\0".as_ptr() as *const c_char,
            addr_of_mut!(strp),
        ) < 0
        {
            return tag_fail(cfg);
        }
        if !strp.is_null() {
            if cstr_eq_lit(strp, b"none") {
                (*p).accel_enable = 0;
            } else if cstr_eq_lit(strp, b"auto") {
                (*p).accel_enable = 1;
            } else {
                vm_error(
                    b"unsupported 'accel' config: %s\n\0".as_ptr() as *const c_char,
                    strp,
                );
                return -1;
            }
        }

        tag_name = b"rtc_local_time\0".as_ptr() as *const c_char;
        let el = json_object_get(cfg, tag_name);
        if el.type_ != JSON_UNDEFINED {
            if el.type_ != JSON_BOOL {
                vm_error(
                    b"%s: boolean expected\n\0".as_ptr() as *const c_char,
                    tag_name,
                );
                return tag_fail(cfg);
            }
            (*p).rtc_local_time = el.u.b;
        }

        json_free(cfg);
        0
    }
}

// return -1 if error.
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    unsafe {
        let f = fopen64(filename, b"rb\0".as_ptr() as *const c_char);
        if f.is_null() {
            perror(filename);
            exit(1);
        }
        fseek(f, 0, SEEK_END);
        let size: c_int = ftell(f) as c_int;
        fseek(f, 0, SEEK_SET);
        let buf = malloc(size as usize) as *mut u8;
        if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
            vm_error(b"%s: read error\n\0".as_ptr() as *const c_char, filename);
            exit(1);
        }
        fclose(f);
        *pbuf = buf;
        size
    }
}

unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    unsafe {
        let s = opaque as *mut VMConfigLoadState;
        if err < 0 {
            vm_error(
                b"Error %d while loading file\n\0".as_ptr() as *const c_char,
                -err,
            );
            exit(1);
        }
        if let Some(f) = (*s).file_load_cb {
            f((*s).file_load_opaque, data as *mut u8, size as c_int);
        }
    }
}

unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: Option<FSLoadFileCB>,
    opaque: *mut c_void,
) {
    unsafe {
        if is_url(filename) != 0 {
            (*s).file_load_cb = cb;
            (*s).file_load_opaque = opaque;
            fs_wget(
                filename,
                null::<c_char>(),
                null::<c_char>(),
                s as *mut c_void,
                Some(config_load_file_cb),
                TRUE_VAL,
            );
        } else {
            let mut buf: *mut u8 = null_mut::<u8>();
            let size = load_file(addr_of_mut!(buf), filename);
            if let Some(f) = cb {
                f(opaque, buf, size);
            }
            free(buf as *mut c_void);
        }
    }
}

unsafe extern "C" fn config_file_loaded(opaque: *mut c_void, buf: *mut u8, buf_len: c_int) {
    unsafe {
        let s = opaque as *mut VMConfigLoadState;
        let p = (*s).vm_params;
        if virt_machine_parse_config(p, buf as *mut c_char, buf_len) < 0 {
            exit(1);
        }
        (*s).file_index = 0;
        config_additional_file_load(s);
    }
}

unsafe fn config_additional_file_load(s: *mut VMConfigLoadState) {
    unsafe {
        let p = (*s).vm_params;
        loop {
            let idx = (*s).file_index;
            if idx >= VM_FILE_COUNT {
                break;
            }
            let fname_ptr = match (*p).files.get(idx as usize) {
                Some(e) => e.filename,
                None => break,
            };
            if !fname_ptr.is_null() {
                break;
            }
            (*s).file_index = idx + 1;
        }
        if (*s).file_index == VM_FILE_COUNT {
            if let Some(cb) = (*s).start_cb {
                cb((*s).opaque);
            }
            free(s as *mut c_void);
        } else {
            let idx = (*s).file_index as usize;
            let file_name = match (*p).files.get(idx) {
                Some(e) => e.filename,
                None => return,
            };
            let fname = get_file_path((*p).cfg_filename, file_name);
            config_load_file(
                s,
                fname,
                Some(config_additional_file_load_cb),
                s as *mut c_void,
            );
            free(fname as *mut c_void);
        }
    }
}

unsafe extern "C" fn config_additional_file_load_cb(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    unsafe {
        let s = opaque as *mut VMConfigLoadState;
        let p = (*s).vm_params;
        let idx = (*s).file_index as usize;
        let new_buf = malloc(buf_len as usize) as *mut u8;
        if buf_len > 0 && !new_buf.is_null() && !buf.is_null() {
            core::ptr::copy_nonoverlapping(buf as *const u8, new_buf, buf_len as usize);
        }
        if let Some(e) = (*p).files.get_mut(idx) {
            e.buf = new_buf;
            e.len = buf_len;
        }
        (*s).file_index += 1;
        config_additional_file_load(s);
    }
}

// ---------------------------------------------------------------------------
// TARGET
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn virt_machine_load_config_file(
    p: *mut VirtMachineParams,
    filename: *const c_char,
    start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
) {
    unsafe {
        let s = mallocz(size_of::<VMConfigLoadState>()) as *mut VMConfigLoadState;
        (*s).vm_params = p;
        (*s).start_cb = start_cb;
        (*s).opaque = opaque;
        (*p).cfg_filename = strdup(filename);

        config_load_file(s, filename, Some(config_file_loaded), s as *mut c_void);
    }
}
}

// ── `VMConfigLoadState *s; s = mallocz(sizeof(*s)); s->vm_params = p; s->start_cb = start_cb; s->opa…` · machine.c:520-528 · chunk 1448 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VMConfigLoadState *s;
//     
//         s = mallocz(sizeof(*s));
//         s->vm_params = p;
//         s->start_cb = start_cb;
//         s->opaque = opaque;
//         p->cfg_filename = strdup(filename);
//     
//         config_load_file(s, filename, config_file_loaded, s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1448 {
// Port of the VMConfigLoadState setup part of virt_machine_load_config_file.
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

// Mirror of the C `VMConfigLoadState` (anonymous typedef in machine.c), same
// field order and types.
#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut c_void,
    pub start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    pub opaque: *mut c_void,
    pub file_load_cb: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int)>,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

// Allocates and fills the VMConfigLoadState, duplicates the config file name
// into VirtMachineParams.cfg_filename (first member of VirtMachineParams, so it
// is written through a pointer cast) and returns the new state.
#[no_mangle]
pub unsafe extern "C" fn virt_machine_load_config_file__part1448(
    p: *mut c_void,
    filename: *const c_char,
    start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
) -> *mut VMConfigLoadState {
    let s = mallocz(core::mem::size_of::<VMConfigLoadState>()) as *mut VMConfigLoadState;
    (*s).vm_params = p;
    (*s).start_cb = start_cb;
    (*s).opaque = opaque;
    // p->cfg_filename = strdup(filename);  (cfg_filename is VirtMachineParams' first field)
    *(p as *mut *mut c_char) = strdup(filename);
    s
}
}

// ── `config_file_loaded` · machine.c:531-542 · chunk 1449 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void config_file_loaded(void *opaque, uint8_t *buf, int buf_len)
//     {
//         VMConfigLoadState *s = opaque;
//         VirtMachineParams *p = s->vm_params;
//     
//         if (virt_machine_parse_config(p, (char *)buf, buf_len) < 0)
//             exit(1);
//     
//     
//         s->file_index = 0;
//         config_additional_file_load(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1449 {
// Rust port of machine.c: config_file_loaded() plus the static helpers it
// reaches (virt_machine_parse_config, find_name, virt_machine_find_class,
// vm_get_str*, strdup_null, cmdline_subst, config_additional_file_load,
// config_load_file, load_file). All of those are static in C, so they have no
// linker symbol and are re-implemented here as private Rust functions.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

/* ------------------------------------------------------------------ */
/* constants                                                          */
/* ------------------------------------------------------------------ */

const VM_FILE_COUNT: c_int = 4;

const JSON_STR: c_int = 0;
const JSON_BOOL: c_int = 4;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

/* ------------------------------------------------------------------ */
/* small helper for literal C strings                                 */
/* ------------------------------------------------------------------ */

#[inline]
fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

/* ------------------------------------------------------------------ */
/* mirrored C types                                                   */
/* ------------------------------------------------------------------ */

#[repr(C)]
struct JSONString {
    len: c_int,
    data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
union JSONValueU {
    str_: *mut JSONString,
    int32: c_int,
    b: c_int,
    obj: *mut c_void,
    array: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct JSONValue {
    type_: c_int,
    u: JSONValueU,
}

#[repr(C)]
struct DynBuf {
    buf: *mut u8,
    size: usize,
    allocated_size: usize,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: c_long,
    tm_zone: *const c_char,
}

type time_t = c_long;

#[repr(C)]
struct VMFileEntry {
    filename: *mut c_char,
    buf: *mut u8,
    len: c_int,
}

#[repr(C)]
struct VMDriveEntry {
    device: *mut c_char,
    filename: *mut c_char,
    block_dev: *mut c_void,
}

#[repr(C)]
struct VMFSEntry {
    device: *mut c_char,
    tag: *mut c_char,
    filename: *mut c_char,
    fs_dev: *mut c_void,
}

#[repr(C)]
struct VMEthEntry {
    driver: *mut c_char,
    ifname: *mut c_char,
    net: *mut c_void,
}

#[repr(C)]
struct VirtMachineClass {
    machine_names: *const c_char,
    virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    virt_machine_init: Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut c_void>,
    virt_machine_end: Option<unsafe extern "C" fn(*mut c_void)>,
    virt_machine_get_sleep_duration: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    virt_machine_interp: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
    vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut c_void, c_int, c_int, c_int, c_int)>,
    vm_send_key_event: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
}

#[repr(C)]
struct VirtMachineParams {
    cfg_filename: *mut c_char,
    vmc: *const VirtMachineClass,
    machine_name: *mut c_char,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_local_time: c_int,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    console: *mut c_void,
    tab_drive: [VMDriveEntry; 4],
    drive_count: c_int,
    tab_fs: [VMFSEntry; 4],
    fs_count: c_int,
    tab_eth: [VMEthEntry; 1],
    eth_count: c_int,
    cmdline: *mut c_char,
    accel_enable: c_int,
    input_device: *mut c_char,
    files: [VMFileEntry; VM_FILE_COUNT as usize],
}

type FSLoadFileCB = unsafe extern "C" fn(*mut c_void, *mut u8, c_int);
type WGetWriteCallback = unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, usize);

#[repr(C)]
struct VMConfigLoadState {
    vm_params: *mut VirtMachineParams,
    start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
    opaque: *mut c_void,
    file_load_cb: Option<FSLoadFileCB>,
    file_load_opaque: *mut c_void,
    file_index: c_int,
}

/* ------------------------------------------------------------------ */
/* extern declarations                                                */
/* ------------------------------------------------------------------ */

extern "C" {
    /* machine.c / cutils.c / json.c / fs_utils.c / fs_wget.c */
    fn vm_error(fmt: *const c_char, ...);
    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;

    fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue;
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;
    fn json_get_error(val: JSONValue) -> *const c_char;
    fn json_free(val: JSONValue);

    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<WGetWriteCallback>,
        single_write: c_int,
    ) -> *mut c_void;

    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, string: *const c_char);

    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;

    /* libc */
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcpy(d: *mut c_char, s: *const c_char) -> *mut c_char;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn exit(code: c_int) -> !;

    fn time(t: *mut time_t) -> time_t;
    fn localtime_r(t: *const time_t, tm: *mut Tm) -> *mut Tm;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(f: *mut c_void) -> c_int;
    fn fseek(f: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(f: *mut c_void) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, n: usize, f: *mut c_void) -> usize;
    fn fprintf(f: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    static mut stderr: *mut c_void;
}

/* ------------------------------------------------------------------ */
/* json_is_error / json_is_undefined                                  */
/* ------------------------------------------------------------------ */

#[inline]
fn json_is_error(val: JSONValue) -> bool {
    val.type_ == JSON_EXCEPTION
}

#[inline]
fn json_is_undefined(val: JSONValue) -> bool {
    val.type_ == JSON_UNDEFINED
}

/* ------------------------------------------------------------------ */
/* vm_get_str2 / vm_get_str / vm_get_str_opt / strdup_null            */
/* ------------------------------------------------------------------ */

unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: c_int,
) -> c_int {
    let val = json_object_get(obj, name);
    if json_is_undefined(val) {
        if is_opt != 0 {
            *pstr = ptr::null();
            0
        } else {
            vm_error(cstr(b"expecting '%s' property\n\0"), name);
            -1
        }
    } else if val.type_ != JSON_STR {
        vm_error(cstr(b"%s: string expected\n\0"), name);
        -1
    } else {
        let s = val.u.str_;
        *pstr = ptr::addr_of!((*s).data) as *const c_char;
        0
    }
}

unsafe fn vm_get_str(obj: JSONValue, name: *const c_char, pstr: *mut *const c_char) -> c_int {
    vm_get_str2(obj, name, pstr, 0)
}

unsafe fn vm_get_str_opt(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    vm_get_str2(obj, name, pstr, 1)
}

unsafe fn strdup_null(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        ptr::null_mut()
    } else {
        strdup(s)
    }
}

/* ------------------------------------------------------------------ */
/* cmdline_subst (only ${TZ} is substituted)                          */
/* ------------------------------------------------------------------ */

unsafe fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    let mut dbuf: DynBuf = core::mem::zeroed();
    let mut var_name = [0u8; 32];
    let mut buf = [0u8; 32];

    dbuf_init(ptr::addr_of_mut!(dbuf));
    let mut p = cmdline;
    while *p != 0 {
        if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
            p = p.add(2);
            let mut q: usize = 0;
            while *p != 0 && *p != b'}' as c_char {
                if q < 32 - 1 {
                    var_name[q] = *p as u8;
                    q += 1;
                }
                p = p.add(1);
            }
            var_name[q] = 0;
            if *p == b'}' as c_char {
                p = p.add(1);
            }
            if strcmp(var_name.as_ptr() as *const c_char, cstr(b"TZ\0")) == 0 {
                let mut ti: time_t = 0;
                let mut tm: Tm = core::mem::zeroed();
                time(ptr::addr_of_mut!(ti));
                localtime_r(ptr::addr_of!(ti), ptr::addr_of_mut!(tm));
                let mut n: c_int = (tm.tm_gmtoff / 60) as c_int;
                let mut sg: c_int = b'-' as c_int;
                if n < 0 {
                    sg = b'+' as c_int;
                    n = -n;
                }
                snprintf(
                    buf.as_mut_ptr() as *mut c_char,
                    buf.len(),
                    cstr(b"UTC%c%02d:%02d\0"),
                    sg,
                    n / 60,
                    n % 60,
                );
                dbuf_putstr(ptr::addr_of_mut!(dbuf), buf.as_ptr() as *const c_char);
            }
        } else {
            dbuf_putc(ptr::addr_of_mut!(dbuf), *p as u8);
            p = p.add(1);
        }
    }
    dbuf_putc(ptr::addr_of_mut!(dbuf), 0);
    dbuf.buf as *mut c_char
}

/* ------------------------------------------------------------------ */
/* find_name / virt_machine_find_class                                */
/* ------------------------------------------------------------------ */

unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> c_int {
    let mut p = name_list;
    loop {
        let r = strchr(p, b',' as c_int);
        if r.is_null() {
            if strcmp(name, p) == 0 {
                return 1;
            }
            break;
        } else {
            let len = r as usize - p as usize;
            if len == strlen(name) && memcmp(name as *const c_void, p as *const c_void, len) == 0 {
                return 1;
            }
            p = (r as *const c_char).add(1);
        }
    }
    0
}

unsafe fn virt_machine_find_class(machine_name: *const c_char) -> *const VirtMachineClass {
    let mut i: usize = 0;
    loop {
        let vmc: *const VirtMachineClass = match i {
            0 => ptr::addr_of!(riscv_machine_class),
            1 => ptr::addr_of!(pc_machine_class),
            _ => break,
        };
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
        i += 1;
    }
    ptr::null()
}

/* ------------------------------------------------------------------ */
/* virt_machine_parse_config                                          */
/* ------------------------------------------------------------------ */

enum ParseRes {
    Ok,
    TagFail, /* goto tag_fail: json_free() then return -1 */
    Fail,    /* return -1 without freeing (as in the C original) */
}

#[inline]
unsafe fn file_at(p: *mut VirtMachineParams, i: c_int) -> *mut VMFileEntry {
    (*p).files.as_mut_ptr().add(i as usize)
}

unsafe fn parse_config_inner(cfg: JSONValue, p: *mut VirtMachineParams) -> ParseRes {
    let mut str_: *const c_char;
    let mut val: c_int = 0;
    let mut buf1 = [0u8; 256];

    str_ = ptr::null();
    if vm_get_str(cfg, cstr(b"machine\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    (*p).machine_name = strdup(str_);
    (*p).vmc = virt_machine_find_class((*p).machine_name);
    if (*p).vmc.is_null() {
        vm_error(
            cstr(b"Unknown machine name: %s\n\0"),
            (*p).machine_name,
        );
        return ParseRes::TagFail;
    }
    if let Some(f) = (*(*p).vmc).virt_machine_set_defaults {
        f(p);
    }

    /* memory_size */
    if vm_get_int(cfg, cstr(b"memory_size\0"), ptr::addr_of_mut!(val)) < 0 {
        return ParseRes::TagFail;
    }
    (*p).ram_size = (val as u64) << 20;

    /* bios */
    if vm_get_str_opt(cfg, cstr(b"bios\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    if !str_.is_null() {
        (*file_at(p, 0)).filename = strdup(str_);
    }

    /* kernel */
    if vm_get_str_opt(cfg, cstr(b"kernel\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    if !str_.is_null() {
        (*file_at(p, 2)).filename = strdup(str_);
    }

    /* cmdline */
    if vm_get_str_opt(cfg, cstr(b"cmdline\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    if !str_.is_null() {
        (*p).cmdline = cmdline_subst(str_);
    }

    /* initrd */
    if vm_get_str_opt(cfg, cstr(b"initrd\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    if !str_.is_null() {
        (*file_at(p, 3)).filename = strdup(str_);
    }

    /* drives */
    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            buf1.len(),
            cstr(b"drive%d\0"),
            (*p).drive_count,
        );
        let obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) {
            break;
        }
        if (*p).drive_count >= 4 {
            vm_error(cstr(b"Too many drives\n\0"));
            return ParseRes::Fail;
        }
        if vm_get_str(obj, cstr(b"file\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        let idx = (*p).drive_count as usize;
        (*(*p).tab_drive.as_mut_ptr().add(idx)).filename = strdup(str_);
        if vm_get_str_opt(obj, cstr(b"device\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        (*(*p).tab_drive.as_mut_ptr().add(idx)).device = strdup_null(str_);
        (*p).drive_count += 1;
    }

    /* filesystems */
    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            buf1.len(),
            cstr(b"fs%d\0"),
            (*p).fs_count,
        );
        let obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) {
            break;
        }
        if (*p).fs_count >= 4 {
            vm_error(cstr(b"Too many filesystems\n\0"));
            return ParseRes::Fail;
        }
        if vm_get_str(obj, cstr(b"file\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        let idx = (*p).fs_count as usize;
        (*(*p).tab_fs.as_mut_ptr().add(idx)).filename = strdup(str_);
        if vm_get_str_opt(obj, cstr(b"tag\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        if str_.is_null() {
            if (*p).fs_count == 0 {
                strcpy(buf1.as_mut_ptr() as *mut c_char, cstr(b"/dev/root\0"));
            } else {
                snprintf(
                    buf1.as_mut_ptr() as *mut c_char,
                    buf1.len(),
                    cstr(b"/dev/root%d\0"),
                    (*p).fs_count,
                );
            }
            str_ = buf1.as_ptr() as *const c_char;
        }
        (*(*p).tab_fs.as_mut_ptr().add(idx)).tag = strdup(str_);
        (*p).fs_count += 1;
    }

    /* ethernet interfaces */
    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            buf1.len(),
            cstr(b"eth%d\0"),
            (*p).eth_count,
        );
        let obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) {
            break;
        }
        if (*p).eth_count >= 1 {
            vm_error(cstr(b"Too many ethernet interfaces\n\0"));
            return ParseRes::Fail;
        }
        if vm_get_str(obj, cstr(b"driver\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        let idx = (*p).eth_count as usize;
        (*(*p).tab_eth.as_mut_ptr().add(idx)).driver = strdup(str_);
        if strcmp(str_, cstr(b"tap\0")) == 0 {
            if vm_get_str(obj, cstr(b"ifname\0"), ptr::addr_of_mut!(str_)) < 0 {
                return ParseRes::TagFail;
            }
            (*(*p).tab_eth.as_mut_ptr().add(idx)).ifname = strdup(str_);
        }
        (*p).eth_count += 1;
    }

    /* display */
    (*p).display_device = ptr::null_mut();
    let obj = json_object_get(cfg, cstr(b"display0\0"));
    if !json_is_undefined(obj) {
        if vm_get_str(obj, cstr(b"device\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        (*p).display_device = strdup(str_);
        if vm_get_int(obj, cstr(b"width\0"), ptr::addr_of_mut!((*p).width)) < 0 {
            return ParseRes::TagFail;
        }
        if vm_get_int(obj, cstr(b"height\0"), ptr::addr_of_mut!((*p).height)) < 0 {
            return ParseRes::TagFail;
        }
        if vm_get_str_opt(obj, cstr(b"vga_bios\0"), ptr::addr_of_mut!(str_)) < 0 {
            return ParseRes::TagFail;
        }
        if !str_.is_null() {
            (*file_at(p, 1)).filename = strdup(str_);
        }
    }

    /* input_device */
    if vm_get_str_opt(cfg, cstr(b"input_device\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    (*p).input_device = strdup_null(str_);

    /* accel */
    if vm_get_str_opt(cfg, cstr(b"accel\0"), ptr::addr_of_mut!(str_)) < 0 {
        return ParseRes::TagFail;
    }
    if !str_.is_null() {
        if strcmp(str_, cstr(b"none\0")) == 0 {
            (*p).accel_enable = 0;
        } else if strcmp(str_, cstr(b"auto\0")) == 0 {
            (*p).accel_enable = 1;
        } else {
            vm_error(cstr(b"unsupported 'accel' config: %s\n\0"), str_);
            return ParseRes::Fail;
        }
    }

    /* rtc_local_time */
    let tag_name = cstr(b"rtc_local_time\0");
    let el = json_object_get(cfg, tag_name);
    if !json_is_undefined(el) {
        if el.type_ != JSON_BOOL {
            vm_error(cstr(b"%s: boolean expected\n\0"), tag_name);
            return ParseRes::TagFail;
        }
        (*p).rtc_local_time = el.u.b;
    }

    ParseRes::Ok
}

unsafe fn virt_machine_parse_config(
    p: *mut VirtMachineParams,
    config_file_str: *mut c_char,
    len: c_int,
) -> c_int {
    let cfg = json_parse_value_len(config_file_str, len);
    if json_is_error(cfg) {
        vm_error(cstr(b"error: %s\n\0"), json_get_error(cfg));
        json_free(cfg);
        return -1;
    }

    let mut version: c_int = 0;
    if vm_get_int(cfg, cstr(b"version\0"), ptr::addr_of_mut!(version)) < 0 {
        json_free(cfg);
        return -1;
    }
    if version != 1 {
        if version > 1 {
            vm_error(cstr(
                b"The emulator is too old to run this VM: please upgrade\n\0",
            ));
        } else {
            vm_error(cstr(b"The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n\0"));
        }
        return -1;
    }

    match parse_config_inner(cfg, p) {
        ParseRes::Ok => {
            json_free(cfg);
            0
        }
        ParseRes::TagFail => {
            json_free(cfg);
            -1
        }
        ParseRes::Fail => -1,
    }
}

/* ------------------------------------------------------------------ */
/* load_file                                                          */
/* ------------------------------------------------------------------ */

unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, cstr(b"rb\0"));
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, 2 /* SEEK_END */);
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, 0 /* SEEK_SET */);
    let buf = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        let err_out = stderr;
        fprintf(err_out, cstr(b"%s: read error\n\0"), filename);
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

/* ------------------------------------------------------------------ */
/* config_load_file / callbacks                                        */
/* ------------------------------------------------------------------ */

unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *mut VMConfigLoadState;

    if err < 0 {
        vm_error(cstr(b"Error %d while loading file\n\0"), -err);
        exit(1);
    }
    if let Some(f) = (*s).file_load_cb {
        f((*s).file_load_opaque, data as *mut u8, size as c_int);
    }
}

unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: Option<FSLoadFileCB>,
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = cb;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            ptr::null(),
            ptr::null(),
            s as *mut c_void,
            Some(config_load_file_cb),
            1,
        );
    } else {
        let mut buf: *mut u8 = ptr::null_mut();
        let size = load_file(ptr::addr_of_mut!(buf), filename);
        if let Some(f) = cb {
            f(opaque, buf, size);
        }
        free(buf as *mut c_void);
    }
}

/* ------------------------------------------------------------------ */
/* config_additional_file_load (+ callback)                           */
/* ------------------------------------------------------------------ */

unsafe extern "C" fn config_additional_file_load_cb(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    let s = opaque as *mut VMConfigLoadState;
    let p = (*s).vm_params;

    let e = file_at(p, (*s).file_index);
    (*e).buf = malloc(buf_len as usize) as *mut u8;
    memcpy((*e).buf as *mut c_void, buf as *const c_void, buf_len as usize);
    (*e).len = buf_len;

    (*s).file_index += 1;
    config_additional_file_load(s);
}

unsafe fn config_additional_file_load(s: *mut VMConfigLoadState) {
    let p = (*s).vm_params;
    while (*s).file_index < VM_FILE_COUNT && (*(file_at(p, (*s).file_index))).filename.is_null() {
        (*s).file_index += 1;
    }
    if (*s).file_index == VM_FILE_COUNT {
        if let Some(cb) = (*s).start_cb {
            cb((*s).opaque);
        }
        free(s as *mut c_void);
    } else {
        let fname = get_file_path(
            (*p).cfg_filename,
            (*(file_at(p, (*s).file_index))).filename,
        );
        config_load_file(
            s,
            fname,
            Some(config_additional_file_load_cb),
            s as *mut c_void,
        );
        free(fname as *mut c_void);
    }
}

/* ------------------------------------------------------------------ */
/* the TARGET                                                         */
/* ------------------------------------------------------------------ */

#[no_mangle]
pub unsafe extern "C" fn config_file_loaded(opaque: *mut c_void, buf: *mut u8, buf_len: c_int) {
    let s = opaque as *mut VMConfigLoadState;
    let p = (*s).vm_params;

    if virt_machine_parse_config(p, buf as *mut c_char, buf_len) < 0 {
        exit(1);
    }

    (*s).file_index = 0;
    config_additional_file_load(s);
}
}

// ── `if (virt_machine_parse_config(p, (char *)buf, buf_len) < 0)` · machine.c:536-537 · chunk 1450 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (virt_machine_parse_config(p, (char *)buf, buf_len) < 0)
//             exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1450 {
// Port of the static helper `virt_machine_parse_config()` (machine.c) that the
// C function config_file_loaded() calls. Exported as
// config_file_loaded__part1450 for the C caller; returns 0 on success and -1 on
// failure (the C side then calls exit(1)).

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

macro_rules! cs {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

pub type BOOL = c_int;
pub type TimeT = c_long;

const JSON_STR: c_int = 0;
const JSON_INT: c_int = 1;
const JSON_BOOL: c_int = 4;
const JSON_UNDEFINED: c_int = 6;
const JSON_EXCEPTION: c_int = 7;

const VM_FILE_BIOS: usize = 0;
const VM_FILE_VGA_BIOS: usize = 1;
const VM_FILE_KERNEL: usize = 2;
const VM_FILE_INITRD: usize = 3;

/* ------------------------------------------------------------------ types */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONString {
    pub len: c_int,
    pub data: [c_char; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONObject {
    pub _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONArray {
    pub _opaque: [u8; 0],
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
    pub type_: c_int,
    pub u: JSONValueU,
}

#[repr(C)]
pub struct DynBuf {
    pub buf: *mut u8,
    pub size: usize,
    pub allocated_size: usize,
}

#[repr(C)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

#[repr(C)]
pub struct BlockDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct FSDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub _opaque: [u8; 0],
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut BlockDevice,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut FSDevice,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut EthernetDevice,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> BOOL>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, BOOL, u16)>,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: BOOL,
    pub rtc_local_time: BOOL,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut CharacterDevice,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: BOOL,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

/* ------------------------------------------------------- C declarations */

extern "C" {
    fn json_parse_value_len(p: *const c_char, len: c_int) -> JSONValue;
    fn json_free(val: JSONValue);
    fn json_get_error(val: JSONValue) -> *const c_char;
    fn json_object_get(val: JSONValue, name: *const c_char) -> JSONValue;

    fn vm_get_int(obj: JSONValue, name: *const c_char, pval: *mut c_int) -> c_int;
    fn vm_error(fmt: *const c_char, ...);

    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;

    fn time(t: *mut TimeT) -> TimeT;
    fn localtime_r(t: *const TimeT, tp: *mut Tm) -> *mut Tm;

    fn dbuf_init(s: *mut DynBuf);
    fn dbuf_putc(s: *mut DynBuf, c: u8);
    fn dbuf_putstr(s: *mut DynBuf, str_: *const c_char);

    static riscv_machine_class: VirtMachineClass;
    static pc_machine_class: VirtMachineClass;
}

/* --------------------------------------------------- inlined C helpers */

#[inline]
fn json_is_error(val: JSONValue) -> BOOL {
    (val.type_ == JSON_EXCEPTION) as BOOL
}

#[inline]
fn json_is_undefined(val: JSONValue) -> BOOL {
    (val.type_ == JSON_UNDEFINED) as BOOL
}

/* port of: static int vm_get_str2(JSONValue obj, const char *name,
                                   const char **pstr, BOOL is_opt) */
unsafe fn vm_get_str2(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
    is_opt: BOOL,
) -> c_int {
    let val = json_object_get(obj, name);
    if json_is_undefined(val) != 0 {
        if is_opt != 0 {
            *pstr = ptr::null();
            0
        } else {
            vm_error(cs!("expecting '%s' property\n"), name);
            -1
        }
    } else if val.type_ != JSON_STR {
        vm_error(cs!("%s: string expected\n"), name);
        -1
    } else {
        let s = val.u.str_;
        *pstr = ptr::addr_of!((*s).data) as *const c_char;
        0
    }
}

unsafe fn vm_get_str(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    vm_get_str2(obj, name, pstr, 0)
}

unsafe fn vm_get_str_opt(
    obj: JSONValue,
    name: *const c_char,
    pstr: *mut *const c_char,
) -> c_int {
    vm_get_str2(obj, name, pstr, 1)
}

/* port of: static char *strdup_null(const char *str) */
unsafe fn strdup_null(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        ptr::null_mut()
    } else {
        strdup(s)
    }
}

/* port of: static BOOL find_name(const char *name, const char *name_list) */
unsafe fn find_name(name: *const c_char, name_list: *const c_char) -> BOOL {
    let mut p = name_list;
    loop {
        let r = strchr(p, ',' as c_int) as *const c_char;
        if r.is_null() {
            if strcmp(name, p) == 0 {
                return 1;
            }
            break;
        } else {
            let len = (r as usize).wrapping_sub(p as usize);
            if len == strlen(name) && memcmp(name as *const c_void, p as *const c_void, len) == 0
            {
                return 1;
            }
            p = r.add(1);
        }
    }
    0
}

/* port of: static const VirtMachineClass *virt_machine_find_class(const char *name)
   (virt_machine_list[] is `static`, so it is rebuilt here) */
unsafe fn virt_machine_find_class(machine_name: *const c_char) -> *const VirtMachineClass {
    let list: [*const VirtMachineClass; 3] = [
        ptr::addr_of!(riscv_machine_class),
        ptr::addr_of!(pc_machine_class),
        ptr::null(),
    ];
    let mut it = list.iter();
    loop {
        let vmc = match it.next() {
            Some(&v) => v,
            None => break,
        };
        if vmc.is_null() {
            break;
        }
        if find_name(machine_name, (*vmc).machine_names) != 0 {
            return vmc;
        }
    }
    ptr::null()
}

/* port of: static char *cmdline_subst(const char *cmdline) */
unsafe fn cmdline_subst(cmdline: *const c_char) -> *mut c_char {
    const VAR_NAME_LEN: usize = 32;

    let mut dbuf = DynBuf {
        buf: ptr::null_mut(),
        size: 0,
        allocated_size: 0,
    };
    dbuf_init(&mut dbuf);
    let mut p = cmdline;
    let mut var_name = [0u8; VAR_NAME_LEN];
    let mut buf = [0u8; 32];

    while *p != 0 {
        if *p == b'$' as c_char && *p.add(1) == b'{' as c_char {
            p = p.add(2);
            let mut qi: usize = 0;
            while *p != 0 && *p != b'}' as c_char {
                if qi < VAR_NAME_LEN - 1 {
                    var_name[qi] = *p as u8;
                    qi += 1;
                }
                p = p.add(1);
            }
            var_name[qi] = 0;
            if *p == b'}' as c_char {
                p = p.add(1);
            }
            if strcmp(var_name.as_ptr() as *const c_char, cs!("TZ")) == 0 {
                let mut ti: TimeT = 0;
                let mut tm: Tm = core::mem::zeroed();
                let mut n: c_int;
                let mut sg: c_int;

                time(&mut ti);
                localtime_r(&ti, &mut tm);
                n = (tm.tm_gmtoff / 60) as c_int;
                sg = b'-' as c_int;
                if n < 0 {
                    sg = b'+' as c_int;
                    n = -n;
                }
                snprintf(
                    buf.as_mut_ptr() as *mut c_char,
                    32usize,
                    cs!("UTC%c%02d:%02d"),
                    sg,
                    n / 60,
                    n % 60,
                );
                dbuf_putstr(&mut dbuf, buf.as_ptr() as *const c_char);
            }
        } else {
            dbuf_putc(&mut dbuf, *p as u8);
            p = p.add(1);
        }
    }
    dbuf_putc(&mut dbuf, 0);
    dbuf.buf as *mut c_char
}

/* ---------------------------------------------- port of the parse body */

unsafe fn parse_cfg_body(p: *mut VirtMachineParams, cfg: JSONValue) -> c_int {
    let mut version: c_int = 0;
    let mut val: c_int = 0;
    let mut tag_name: *const c_char = ptr::null();
    let mut str_: *const c_char = ptr::null();
    let mut buf1 = [0u8; 256];
    let mut obj: JSONValue = core::mem::zeroed();
    let mut el: JSONValue = core::mem::zeroed();

    if vm_get_int(cfg, cs!("version"), &mut version) < 0 {
        return -1;
    }
    if version != 1 {
        if version > 1 {
            vm_error(cs!(
                "The emulator is too old to run this VM: please upgrade\n"
            ));
            return -1;
        } else {
            vm_error(cs!("The VM configuration file is too old for this emulator version: please upgrade the VM configuration file\n"));
            return -1;
        }
    }

    if vm_get_str(cfg, cs!("machine"), &mut str_) < 0 {
        return -1;
    }
    (*p).machine_name = strdup(str_);
    (*p).vmc = virt_machine_find_class((*p).machine_name);
    if (*p).vmc.is_null() {
        vm_error(cs!("Unknown machine name: %s\n"), (*p).machine_name);
        return -1;
    }
    let set_defaults = (*(*p).vmc).virt_machine_set_defaults;
    if let Some(f) = set_defaults {
        f(p);
    }

    tag_name = cs!("memory_size");
    if vm_get_int(cfg, tag_name, &mut val) < 0 {
        return -1;
    }
    (*p).ram_size = ((val as i64) as u64) << 20;

    tag_name = cs!("bios");
    if vm_get_str_opt(cfg, tag_name, &mut str_) < 0 {
        return -1;
    }
    if !str_.is_null() {
        (*p).files[VM_FILE_BIOS].filename = strdup(str_);
    }

    tag_name = cs!("kernel");
    if vm_get_str_opt(cfg, tag_name, &mut str_) < 0 {
        return -1;
    }
    if !str_.is_null() {
        (*p).files[VM_FILE_KERNEL].filename = strdup(str_);
    }

    if vm_get_str_opt(cfg, cs!("cmdline"), &mut str_) < 0 {
        return -1;
    }
    if !str_.is_null() {
        (*p).cmdline = cmdline_subst(str_);
    }

    tag_name = cs!("initrd");
    if vm_get_str_opt(cfg, tag_name, &mut str_) < 0 {
        return -1;
    }
    if !str_.is_null() {
        (*p).files[VM_FILE_INITRD].filename = strdup(str_);
    }

    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            256usize,
            cs!("drive%d"),
            (*p).drive_count,
        );
        obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) != 0 {
            break;
        }
        if (*p).drive_count >= 4 {
            vm_error(cs!("Too many drives\n"));
            return -1;
        }
        if vm_get_str(obj, cs!("file"), &mut str_) < 0 {
            return -1;
        }
        let idx = (*p).drive_count as usize;
        (*p).tab_drive[idx].filename = strdup(str_);
        if vm_get_str_opt(obj, cs!("device"), &mut str_) < 0 {
            return -1;
        }
        let idx = (*p).drive_count as usize;
        (*p).tab_drive[idx].device = strdup_null(str_);
        (*p).drive_count += 1;
    }

    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            256usize,
            cs!("fs%d"),
            (*p).fs_count,
        );
        obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) != 0 {
            break;
        }
        if (*p).fs_count >= 4 {
            vm_error(cs!("Too many filesystems\n"));
            return -1;
        }
        if vm_get_str(obj, cs!("file"), &mut str_) < 0 {
            return -1;
        }
        let idx = (*p).fs_count as usize;
        (*p).tab_fs[idx].filename = strdup(str_);
        if vm_get_str_opt(obj, cs!("tag"), &mut str_) < 0 {
            return -1;
        }
        if str_.is_null() {
            if (*p).fs_count == 0 {
                strcpy(buf1.as_mut_ptr() as *mut c_char, cs!("/dev/root"));
            } else {
                snprintf(
                    buf1.as_mut_ptr() as *mut c_char,
                    256usize,
                    cs!("/dev/root%d"),
                    (*p).fs_count,
                );
            }
            str_ = buf1.as_ptr() as *const c_char;
        }
        let idx = (*p).fs_count as usize;
        (*p).tab_fs[idx].tag = strdup(str_);
        (*p).fs_count += 1;
    }

    loop {
        snprintf(
            buf1.as_mut_ptr() as *mut c_char,
            256usize,
            cs!("eth%d"),
            (*p).eth_count,
        );
        obj = json_object_get(cfg, buf1.as_ptr() as *const c_char);
        if json_is_undefined(obj) != 0 {
            break;
        }
        if (*p).eth_count >= 1 {
            vm_error(cs!("Too many ethernet interfaces\n"));
            return -1;
        }
        if vm_get_str(obj, cs!("driver"), &mut str_) < 0 {
            return -1;
        }
        let idx = (*p).eth_count as usize;
        (*p).tab_eth[idx].driver = strdup(str_);
        if strcmp(str_, cs!("tap")) == 0 {
            if vm_get_str(obj, cs!("ifname"), &mut str_) < 0 {
                return -1;
            }
            let idx = (*p).eth_count as usize;
            (*p).tab_eth[idx].ifname = strdup(str_);
        }
        (*p).eth_count += 1;
    }

    (*p).display_device = ptr::null_mut();
    obj = json_object_get(cfg, cs!("display0"));
    if json_is_undefined(obj) == 0 {
        if vm_get_str(obj, cs!("device"), &mut str_) < 0 {
            return -1;
        }
        (*p).display_device = strdup(str_);
        if vm_get_int(obj, cs!("width"), &mut (*p).width) < 0 {
            return -1;
        }
        if vm_get_int(obj, cs!("height"), &mut (*p).height) < 0 {
            return -1;
        }
        if vm_get_str_opt(obj, cs!("vga_bios"), &mut str_) < 0 {
            return -1;
        }
        if !str_.is_null() {
            (*p).files[VM_FILE_VGA_BIOS].filename = strdup(str_);
        }
    }

    if vm_get_str_opt(cfg, cs!("input_device"), &mut str_) < 0 {
        return -1;
    }
    (*p).input_device = strdup_null(str_);

    if vm_get_str_opt(cfg, cs!("accel"), &mut str_) < 0 {
        return -1;
    }
    if !str_.is_null() {
        if strcmp(str_, cs!("none")) == 0 {
            (*p).accel_enable = 0;
        } else if strcmp(str_, cs!("auto")) == 0 {
            (*p).accel_enable = 1;
        } else {
            vm_error(cs!("unsupported 'accel' config: %s\n"), str_);
            return -1;
        }
    }

    tag_name = cs!("rtc_local_time");
    el = json_object_get(cfg, tag_name);
    if json_is_undefined(el) == 0 {
        if el.type_ != JSON_BOOL {
            vm_error(cs!("%s: boolean expected\n"), tag_name);
            return -1;
        }
        (*p).rtc_local_time = el.u.b;
    }

    0
}

/* ------------------------------------------------------- entry point */

#[no_mangle]
pub extern "C" fn config_file_loaded__part1450(
    p: *mut VirtMachineParams,
    config_file_str: *mut c_char,
    len: c_int,
) -> c_int {
    unsafe {
        let cfg = json_parse_value_len(config_file_str, len);
        let ret;
        if json_is_error(cfg) != 0 {
            vm_error(cs!("error: %s\n"), json_get_error(cfg));
            ret = -1;
        } else {
            ret = parse_cfg_body(p, cfg);
        }
        json_free(cfg);
        ret
    }
}
}

// ── `s->file_index = 0; config_additional_file_load(s);` · machine.c:540-541 · chunk 1451 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     s->file_index = 0;
//         config_additional_file_load(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1451 {
// Port of the tail of config_file_loaded: reset file_index and start
// loading the additional configuration files.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

const VM_FILE_COUNT: c_int = 4;

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

pub type FSLoadFileCB =
    Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_len: c_int)>;
pub type StartCB = Option<unsafe extern "C" fn(opaque: *mut c_void)>;
pub type WGetWriteCallback =
    Option<unsafe extern "C" fn(opaque: *mut c_void, err: c_int, data: *mut c_void, size: usize)>;

#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut VirtMachineParams,
    pub start_cb: StartCB,
    pub opaque: *mut c_void,
    pub file_load_cb: FSLoadFileCB,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn vm_error(fmt: *const c_char, ...);
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: WGetWriteCallback,
        single_write: c_int,
    ) -> *mut c_void;
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn free(p: *mut c_void);
    fn exit(code: c_int) -> !;

    // libc bits used by the ported load_file
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(f: *mut c_void) -> c_int;
    fn fseek(f: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(f: *mut c_void) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, f: *mut c_void) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
}

// Ported from the static C function load_file().
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, 2);
    let size = ftell(f) as c_int;
    fseek(f, 0, 0);
    let buf = malloc(size as usize) as *mut u8;
    let got = fread(buf as *mut c_void, 1, size as usize, f);
    if got != size as usize {
        fprintf(
            stderr,
            b"%s: read error\n\0".as_ptr() as *const c_char,
            filename,
        );
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

// Ported from the static C function config_load_file_cb().
unsafe extern "C" fn config_load_file_cb_local(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *mut VMConfigLoadState;
    if err < 0 {
        vm_error(
            b"Error %d while loading file\n\0".as_ptr() as *const c_char,
            err.wrapping_neg(),
        );
        exit(1);
    }
    match (*s).file_load_cb {
        Some(f) => f((*s).file_load_opaque, data as *mut u8, size as c_int),
        None => {}
    }
}

// Ported from the static C function config_load_file().
unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: unsafe extern "C" fn(*mut c_void, *mut u8, c_int),
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = Some(cb);
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            Some(config_load_file_cb_local),
            1,
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size = load_file(&mut buf, filename);
        cb(opaque, buf, size);
        free(buf as *mut c_void);
    }
}

// Ported from the static C function config_additional_file_load_cb().
unsafe extern "C" fn config_additional_file_load_cb(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    let s = opaque as *mut VMConfigLoadState;
    let p = (*s).vm_params;
    let idx = (*s).file_index;
    if idx < 0 || idx >= VM_FILE_COUNT {
        return;
    }
    let alloc = malloc(buf_len as usize) as *mut u8;
    let entry = &mut (*p).files[idx as usize];
    entry.buf = alloc;
    if !alloc.is_null() && !buf.is_null() && buf_len > 0 {
        core::ptr::copy_nonoverlapping(buf, alloc, buf_len as usize);
    }
    entry.len = buf_len;

    (*s).file_index = (*s).file_index.wrapping_add(1);
    config_additional_file_load(s);
}

// Ported from the static C function config_additional_file_load().
unsafe fn config_additional_file_load(s: *mut VMConfigLoadState) {
    let p = (*s).vm_params;
    while (*s).file_index < VM_FILE_COUNT {
        let idx = (*s).file_index as usize;
        if !(*p).files[idx].filename.is_null() {
            break;
        }
        (*s).file_index = (*s).file_index.wrapping_add(1);
    }
    if (*s).file_index == VM_FILE_COUNT {
        match (*s).start_cb {
            Some(f) => f((*s).opaque),
            None => {}
        }
        free(s as *mut c_void);
    } else {
        let idx = (*s).file_index as usize;
        let fname = get_file_path((*p).cfg_filename, (*p).files[idx].filename);
        config_load_file(s, fname, config_additional_file_load_cb, s as *mut c_void);
        free(fname as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_file_loaded__part1451(s: *mut VMConfigLoadState) {
    if s.is_null() {
        return;
    }
    (*s).file_index = 0;
    config_additional_file_load(s);
}
}

// ── `config_additional_file_load` · machine.c:544-564 · chunk 1452 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void config_additional_file_load(VMConfigLoadState *s)
//     {
//         VirtMachineParams *p = s->vm_params;
//         while (s->file_index < VM_FILE_COUNT &&
//                p->files[s->file_index].filename == 
//                                                   ((void *)0)
//                                                       ) {
//             s->file_index++;
//         }
//         if (s->file_index == VM_FILE_COUNT) {
//             if (s->start_cb)
//                 s->start_cb(s->opaque);
//             free(s);
//         } else {
//             char *fname;
//     
//             fname = get_file_path(p->cfg_filename,
//                                   p->files[s->file_index].filename);
//             config_load_file(s, fname,
//                              config_additional_file_load_cb, s);
//             free(fname);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1452 {
// Port of machine.c: config_additional_file_load()
// All helper routines it reaches (config_load_file, config_load_file_cb,
// load_file, config_additional_file_load_cb) are `static` in C, so their
// logic is reproduced here as private Rust functions.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr::{null, null_mut};

const VM_FILE_COUNT: c_int = 4;
const TRUE: c_int = 1;

#[repr(C)]
struct VmFileEntry {
    filename: *mut c_char,
    buf: *mut u8,
    len: c_int,
}

#[repr(C)]
struct VmDriveEntry {
    device: *mut c_char,
    filename: *mut c_char,
    block_dev: *mut c_void,
}

#[repr(C)]
struct VmFsEntry {
    device: *mut c_char,
    tag: *mut c_char,
    filename: *mut c_char,
    fs_dev: *mut c_void,
}

#[repr(C)]
struct VmEthEntry {
    driver: *mut c_char,
    ifname: *mut c_char,
    net: *mut c_void,
}

#[repr(C)]
struct VirtMachineParams {
    cfg_filename: *mut c_char,
    vmc: *const c_void,
    machine_name: *mut c_char,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_local_time: c_int,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    console: *mut c_void,
    tab_drive: [VmDriveEntry; 4],
    drive_count: c_int,
    tab_fs: [VmFsEntry; 4],
    fs_count: c_int,
    tab_eth: [VmEthEntry; 1],
    eth_count: c_int,
    cmdline: *mut c_char,
    accel_enable: c_int,
    input_device: *mut c_char,
    files: [VmFileEntry; VM_FILE_COUNT as usize],
}

type FileLoadCb = unsafe extern "C" fn(*mut c_void, *mut u8, c_int);
type StartCb = unsafe extern "C" fn(*mut c_void);
type WGetWriteCallback = unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, usize);

#[repr(C)]
struct VmConfigLoadState {
    vm_params: *mut VirtMachineParams,
    start_cb: Option<StartCb>,
    opaque: *mut c_void,
    file_load_cb: Option<FileLoadCb>,
    file_load_opaque: *mut c_void,
    file_index: c_int,
}

extern "C" {
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<WGetWriteCallback>,
        single_write: c_int,
    ) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn perror(s: *const c_char);
    #[link_name = "fopen64"]
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> c_int;
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut c_void) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// static int load_file(uint8_t **pbuf, const char *filename)
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, 2 /* SEEK_END */);
    let size_long = ftell(f);
    let size = size_long as c_int;
    fseek(f, 0, 0 /* SEEK_SET */);
    let buf = malloc(size as usize);
    if fread(buf, 1, size as usize, f) != size as usize {
        fprintf(
            stderr,
            b"%s: read error\n\0".as_ptr() as *const c_char,
            filename,
        );
        exit(1);
    }
    fclose(f);
    *pbuf = buf as *mut u8;
    size
}

// static void config_load_file_cb(void *opaque, int err, void *data, size_t size)
unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *mut VmConfigLoadState;

    if err < 0 {
        vm_error(
            b"Error %d while loading file\n\0".as_ptr() as *const c_char,
            -err,
        );
        exit(1);
    }
    if let Some(cb) = (*s).file_load_cb {
        cb((*s).file_load_opaque, data as *mut u8, size as c_int);
    }
}

// static void config_load_file(VMConfigLoadState *s, const char *filename,
//                              FSLoadFileCB *cb, void *opaque)
unsafe fn config_load_file(
    s: *mut VmConfigLoadState,
    filename: *const c_char,
    cb: FileLoadCb,
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = Some(cb);
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            null(),
            null(),
            s as *mut c_void,
            Some(config_load_file_cb),
            TRUE,
        );
    } else {
        let mut buf: *mut u8 = null_mut();
        let size = load_file(&mut buf, filename);
        cb(opaque, buf, size);
        free(buf as *mut c_void);
    }
}

// static void config_additional_file_load_cb(void *opaque,
//                                            uint8_t *buf, int buf_len)
unsafe extern "C" fn config_additional_file_load_cb(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    let s = opaque as *mut VmConfigLoadState;
    let p = (*s).vm_params;

    let entry = (*p).files.as_mut_ptr().add((*s).file_index as usize);
    (*entry).buf = malloc(buf_len as usize) as *mut u8;
    memcpy((*entry).buf as *mut c_void, buf as *const c_void, buf_len as usize);
    (*entry).len = buf_len;

    // load the next files
    (*s).file_index += 1;
    config_additional_file_load(s);
}

#[no_mangle]
pub unsafe extern "C" fn config_additional_file_load(s: *mut VmConfigLoadState) {
    let p = (*s).vm_params;
    while (*s).file_index < VM_FILE_COUNT
        && (*(*p).files.as_mut_ptr().add((*s).file_index as usize))
            .filename
            .is_null()
    {
        (*s).file_index += 1;
    }
    if (*s).file_index == VM_FILE_COUNT {
        if let Some(cb) = (*s).start_cb {
            cb((*s).opaque);
        }
        free(s as *mut c_void);
    } else {
        let entry = (*p).files.as_mut_ptr().add((*s).file_index as usize);
        let fname = get_file_path((*p).cfg_filename, (*entry).filename);
        config_load_file(s, fname, config_additional_file_load_cb, s as *mut c_void);
        free(fname as *mut c_void);
    }
}
}

// ── `while (s->file_index < VM_FILE_COUNT && p->files[s->file_index].filename == ((void *)0)` · machine.c:547-550 · chunk 1453 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (s->file_index < VM_FILE_COUNT &&
//                p->files[s->file_index].filename == 
//                                                   ((void *)0)
//                                                       ) {
//             s->file_index++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1453 {
// Port of the file_index skip loop from config_additional_file_load.
// Mirrors VMFileEntry (char *filename; uint8_t *buf; int len;) and walks the
// params->files[] array until a non-NULL filename or VM_FILE_COUNT is reached.

use core::ffi::{c_char, c_int};

const VM_FILE_COUNT: c_int = 4;

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[no_mangle]
pub extern "C" fn config_additional_file_load__part1453(
    file_index: *mut c_int,
    files: *mut VMFileEntry,
) {
    if file_index.is_null() {
        return;
    }
    unsafe {
        let mut fi = *file_index;
        while fi < VM_FILE_COUNT && (*files.add(fi as usize)).filename.is_null() {
            fi += 1;
        }
        *file_index = fi;
    }
}
}

// ── `if (s->file_index == VM_FILE_COUNT) … else` · machine.c:551-563 · chunk 1454 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (s->file_index == VM_FILE_COUNT) {
//             if (s->start_cb)
//                 s->start_cb(s->opaque);
//             free(s);
//         } else {
//             char *fname;
//     
//             fname = get_file_path(p->cfg_filename,
//                                   p->files[s->file_index].filename);
//             config_load_file(s, fname,
//                              config_additional_file_load_cb, s);
//             free(fname);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1454 {
// Portion of config_additional_file_load(): start loading the current
// additional config file, or finish the load sequence when all entries of
// the file list have been consumed.
//
// config_load_file() and load_file() are `static` in the C translation unit,
// so their logic is ported here as private functions. The two `static`
// callbacks (config_additional_file_load_cb and config_load_file_cb) are
// passed in by the C caller as function pointers.

use core::ffi::{c_char, c_int, c_long, c_void};

// enum { VM_FILE_BIOS, VM_FILE_VGA_BIOS, VM_FILE_KERNEL, VM_FILE_INITRD, VM_FILE_COUNT }
const VM_FILE_COUNT: c_int = 4;

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;

#[allow(dead_code)]
#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[allow(dead_code)]
#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[allow(dead_code)]
#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[allow(dead_code)]
#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[allow(dead_code)]
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; VM_FILE_COUNT as usize],
}

// typedef void FSLoadFileCB(void *opaque, uint8_t *buf, int buf_len);
type FSLoadFileCBFn = unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_len: c_int);

// typedef void WGetWriteCallback(void *opaque, int err, void *data, size_t size);
type WGetWriteCBFn =
    unsafe extern "C" fn(opaque: *mut c_void, err: c_int, data: *mut c_void, size: usize);

#[allow(dead_code)]
#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut VirtMachineParams,
    pub start_cb: Option<unsafe extern "C" fn(opaque: *mut c_void)>,
    pub opaque: *mut c_void,
    pub file_load_cb: Option<FSLoadFileCBFn>,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;

    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<WGetWriteCBFn>,
        single_write: c_int,
    ) -> *mut c_void;

    #[link_name = "fopen64"]
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut c_void;
    static stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut c_void) -> c_long;
    fn fclose(stream: *mut c_void) -> c_int;
    fn perror(s: *const c_char);

    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
}

// Ported from the static C function load_file().
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, SEEK_SET);
    let buf = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        fprintf(
            stderr,
            b"%s: read error\n\0".as_ptr() as *const c_char,
            filename,
        );
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

// Ported from the static C function config_load_file(); `load_cb` is the
// static C function config_load_file_cb.
unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: Option<FSLoadFileCBFn>,
    opaque: *mut c_void,
    load_cb: Option<WGetWriteCBFn>,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = cb;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            load_cb,
            1,
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size = load_file(&mut buf, filename);
        if let Some(cbf) = cb {
            cbf(opaque, buf, size);
        }
        free(buf as *mut c_void);
    }
}

/// Returns 1 when the whole load sequence is finished (the state has already
/// been freed, so the C caller must return), 0 when another file load was
/// started.
#[no_mangle]
pub unsafe extern "C" fn config_additional_file_load__part1454(
    s: *mut VMConfigLoadState,
    cb: Option<FSLoadFileCBFn>,
    load_cb: Option<WGetWriteCBFn>,
) -> c_int {
    let state = &mut *s;
    if state.file_index == VM_FILE_COUNT {
        if let Some(start_cb) = state.start_cb {
            start_cb(state.opaque);
        }
        free(s as *mut c_void);
        1
    } else {
        let p = state.vm_params;
        let entry = (*p).files.as_ptr().add(state.file_index as usize);
        let fname = get_file_path((*p).cfg_filename, (*entry).filename);
        config_load_file(s, fname, cb, s as *mut c_void, load_cb);
        free(fname as *mut c_void);
        0
    }
}
}

// ── `config_additional_file_load_cb` · machine.c:566-579 · chunk 1455 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void config_additional_file_load_cb(void *opaque,
//                                                uint8_t *buf, int buf_len)
//     {
//         VMConfigLoadState *s = opaque;
//         VirtMachineParams *p = s->vm_params;
//     
//         p->files[s->file_index].buf = malloc(buf_len);
//         memcpy(p->files[s->file_index].buf, buf, buf_len);
//         p->files[s->file_index].len = buf_len;
//     
//     
//         s->file_index++;
//         config_additional_file_load(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1455 {
// Rust rewrite of machine.c: config_additional_file_load_cb()
//
// The C static helpers it depends on (config_additional_file_load,
// config_load_file, config_load_file_cb, load_file) have no linker symbols,
// so their logic is ported here as private Rust fns.

use core::ffi::{c_char, c_int, c_long, c_void};

const VM_FILE_COUNT: c_int = 4;

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;

const TRUE: c_int = 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XHRState {
    _private: [u8; 0],
}

// typedef void FSLoadFileCB(void *opaque, uint8_t *buf, int buf_len);
type FSLoadFileCB = unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, buf_len: c_int);

// typedef void WGetWriteCallback(void *opaque, int err, void *data, size_t size);
type WGetWriteCallback =
    unsafe extern "C" fn(opaque: *mut c_void, err: c_int, data: *mut c_void, size: usize);

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

// Mirror of VirtMachineParams (machine.h). Pointer-only fields of opaque
// device/class types are represented as raw void pointers (same layout).
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut VirtMachineParams,
    pub start_cb: Option<unsafe extern "C" fn(opaque: *mut c_void)>,
    pub opaque: *mut c_void,
    pub file_load_cb: Option<FSLoadFileCB>,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;

    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<WGetWriteCallback>,
        single_write: c_int,
    ) -> *mut XHRState;

    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn perror(s: *const c_char);

    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, n: usize, stream: *mut FILE) -> usize;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut FILE;
}

// static int load_file(uint8_t **pbuf, const char *filename)
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, SEEK_SET);
    let buf = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        fprintf(
            stderr,
            b"%s: read error\n\0".as_ptr() as *const c_char,
            filename,
        );
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

// static void config_load_file_cb(void *opaque, int err, void *data, size_t size)
unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    let s = opaque as *mut VMConfigLoadState;

    if err < 0 {
        vm_error(
            b"Error %d while loading file\n\0".as_ptr() as *const c_char,
            -err,
        );
        exit(1);
    }
    let cb = (*s).file_load_cb;
    if let Some(f) = cb {
        f((*s).file_load_opaque, data as *mut u8, size as c_int);
    }
}

// static void config_load_file(VMConfigLoadState *s, const char *filename,
//                              FSLoadFileCB *cb, void *opaque)
unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: Option<FSLoadFileCB>,
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = cb;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            Some(config_load_file_cb),
            TRUE,
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size = load_file(&mut buf, filename);
        if let Some(f) = cb {
            f(opaque, buf, size);
        }
        free(buf as *mut c_void);
    }
}

// static void config_additional_file_load(VMConfigLoadState *s)
unsafe fn config_additional_file_load(s: *mut VMConfigLoadState) {
    let p = (*s).vm_params;

    while (*s).file_index < VM_FILE_COUNT
        && (*(*p).files.as_mut_ptr().add((*s).file_index as usize))
            .filename
            .is_null()
    {
        (*s).file_index += 1;
    }
    if (*s).file_index == VM_FILE_COUNT {
        let start_cb = (*s).start_cb;
        if let Some(f) = start_cb {
            f((*s).opaque);
        }
        free(s as *mut c_void);
    } else {
        let fname = get_file_path(
            (*p).cfg_filename,
            (*(*p).files.as_mut_ptr().add((*s).file_index as usize)).filename,
        );
        config_load_file(
            s,
            fname,
            Some(config_additional_file_load_cb),
            s as *mut c_void,
        );
        free(fname as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_additional_file_load_cb(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    let s = opaque as *mut VMConfigLoadState;
    let p = (*s).vm_params;
    let file_index = (*s).file_index;
    let entry = (*p).files.as_mut_ptr().add(file_index as usize);

    let new_buf = malloc(buf_len as usize);
    (*entry).buf = new_buf as *mut u8;
    memcpy(new_buf, buf as *const c_void, buf_len as usize);
    (*entry).len = buf_len;

    (*s).file_index = file_index + 1;
    config_additional_file_load(s);
}
}

// ── `VMConfigLoadState *s = opaque; VirtMachineParams *p = s->vm_params; p->files[s->file_index].buf…` · machine.c:569-578 · chunk 1456 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     VMConfigLoadState *s = opaque;
//         VirtMachineParams *p = s->vm_params;
//     
//         p->files[s->file_index].buf = malloc(buf_len);
//         memcpy(p->files[s->file_index].buf, buf, buf_len);
//         p->files[s->file_index].len = buf_len;
//     
//     
//         s->file_index++;
//         config_additional_file_load(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1456 {
// Port of the body of config_additional_file_load_cb (machine.c lines 569-578),
// exported as config_additional_file_load_cb__part1456.
//
// The tail call config_additional_file_load(s) targets a `static` C function,
// which has no linker symbol, so its logic (and the static helpers it needs:
// config_load_file, config_load_file_cb, load_file) is mirrored privately here.

use core::ffi::{c_char, c_int, c_long, c_void};

const VM_FILE_COUNT: c_int = 4;

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; VM_FILE_COUNT as usize],
}

#[repr(C)]
pub struct VMConfigLoadState {
    pub vm_params: *mut VirtMachineParams,
    pub start_cb: *mut c_void,
    pub opaque: *mut c_void,
    pub file_load_cb: *mut c_void,
    pub file_load_opaque: *mut c_void,
    pub file_index: c_int,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn free(p: *mut c_void);
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn is_url(path: *const c_char) -> c_int;
    fn fs_wget(
        url: *const c_char,
        user: *const c_char,
        password: *const c_char,
        opaque: *mut c_void,
        cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, usize)>,
        single_write: c_int,
    ) -> *mut c_void;
    fn vm_error(fmt: *const c_char, ...);
    fn exit(status: c_int) -> !;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fseek(stream: *mut c_void, offset: c_long, whence: c_int) -> c_int;
    fn ftell(stream: *mut c_void) -> c_long;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn fclose(stream: *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static stderr: *mut c_void;
}

#[inline]
unsafe fn file_at(p: *mut VirtMachineParams, i: c_int) -> *mut VMFileEntry {
    (core::ptr::addr_of_mut!((*p).files) as *mut VMFileEntry).offset(i as isize)
}

// --- private mirror of the C `static int load_file(uint8_t **pbuf, const char *)`
unsafe fn load_file(pbuf: *mut *mut u8, filename: *const c_char) -> c_int {
    let f = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, 2);
    let size: c_int = ftell(f) as c_int;
    fseek(f, 0, 0);
    let buf = malloc(size as usize) as *mut u8;
    if fread(buf as *mut c_void, 1, size as usize, f) != size as usize {
        fprintf(stderr, b"%s: read error\n\0".as_ptr() as *const c_char, filename);
        exit(1);
    }
    fclose(f);
    *pbuf = buf;
    size
}

type FSLoadFileCB = unsafe extern "C" fn(*mut c_void, *mut u8, c_int);

// --- private mirror of the C `static void config_load_file_cb(...)`
unsafe extern "C" fn config_load_file_cb(
    opaque: *mut c_void,
    err: c_int,
    data: *mut c_void,
    size: usize,
) {
    if err < 0 {
        vm_error(b"Error %d while loading file\n\0".as_ptr() as *const c_char, -err);
        exit(1);
    }
    let s = opaque as *mut VMConfigLoadState;
    let cb = (*s).file_load_cb;
    if !cb.is_null() {
        let f: FSLoadFileCB = core::mem::transmute(cb);
        f((*s).file_load_opaque, data as *mut u8, size as c_int);
    }
}

// --- private mirror of the C `static void config_load_file(...)`
unsafe fn config_load_file(
    s: *mut VMConfigLoadState,
    filename: *const c_char,
    cb: FSLoadFileCB,
    opaque: *mut c_void,
) {
    if is_url(filename) != 0 {
        (*s).file_load_cb = cb as *mut c_void;
        (*s).file_load_opaque = opaque;
        fs_wget(
            filename,
            core::ptr::null(),
            core::ptr::null(),
            s as *mut c_void,
            Some(config_load_file_cb),
            1,
        );
    } else {
        let mut buf: *mut u8 = core::ptr::null_mut();
        let size = load_file(&mut buf, filename);
        cb(opaque, buf, size);
        free(buf as *mut c_void);
    }
}

// Forward declaration: this is the exported function below, used as the
// FSLoadFileCB the C code would have passed.
unsafe extern "C" fn config_additional_file_load_cb_trampoline(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    config_additional_file_load_cb__part1456(opaque, buf, buf_len);
}

// --- private mirror of the C `static void config_additional_file_load(...)`
unsafe fn config_additional_file_load(s: *mut VMConfigLoadState) {
    let p = (*s).vm_params;
    while (*s).file_index < VM_FILE_COUNT && (*file_at(p, (*s).file_index)).filename.is_null() {
        (*s).file_index += 1;
    }
    if (*s).file_index == VM_FILE_COUNT {
        let start_cb = (*s).start_cb;
        if !start_cb.is_null() {
            let f: unsafe extern "C" fn(*mut c_void) = core::mem::transmute(start_cb);
            f((*s).opaque);
        }
        free(s as *mut c_void);
    } else {
        let fname = get_file_path((*p).cfg_filename, (*file_at(p, (*s).file_index)).filename);
        config_load_file(
            s,
            fname,
            config_additional_file_load_cb_trampoline,
            s as *mut c_void,
        );
        free(fname as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn config_additional_file_load_cb__part1456(
    opaque: *mut c_void,
    buf: *mut u8,
    buf_len: c_int,
) {
    let s = opaque as *mut VMConfigLoadState;
    let p = (*s).vm_params;

    let entry = file_at(p, (*s).file_index);

    (*entry).buf = malloc(buf_len as usize) as *mut u8;
    memcpy(
        (*entry).buf as *mut c_void,
        buf as *const c_void,
        buf_len as usize,
    );
    (*entry).len = buf_len;

    (*s).file_index += 1;
    config_additional_file_load(s);
}
}

// ── `vm_add_cmdline` · machine.c:581-597 · chunk 1457 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void vm_add_cmdline(VirtMachineParams *p, const char *cmdline)
//     {
//         char *new_cmdline, *old_cmdline;
//         if (cmdline[0] == '!') {
//             new_cmdline = strdup(cmdline + 1);
//         } else {
//             old_cmdline = p->cmdline;
//             if (!old_cmdline)
//                 old_cmdline = "";
//             new_cmdline = malloc(strlen(old_cmdline) + 1 + strlen(cmdline) + 1);
//             strcpy(new_cmdline, old_cmdline);
//             strcat(new_cmdline, " ");
//             strcat(new_cmdline, cmdline);
//         }
//         free(p->cmdline);
//         p->cmdline = new_cmdline;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1457 {
// Rust rewrite of vm_add_cmdline (machine.c) for the C program.
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void, /* const VirtMachineClass * */
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,  /* BOOL */
    pub rtc_local_time: c_int, /* BOOL */
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void, /* CharacterDevice * */
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int, /* BOOL */
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4], /* VM_FILE_COUNT == 4 */
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
}

static EMPTY: [c_char; 1] = [0];
static SPACE: [c_char; 2] = [b' ' as c_char, 0];

#[no_mangle]
pub unsafe extern "C" fn vm_add_cmdline(p: *mut VirtMachineParams, cmdline: *const c_char) {
    let new_cmdline: *mut c_char;
    if *cmdline == b'!' as c_char {
        new_cmdline = strdup(cmdline.add(1));
    } else {
        let mut old_cmdline: *mut c_char = (*p).cmdline;
        if old_cmdline.is_null() {
            old_cmdline = EMPTY.as_ptr() as *mut c_char;
        }
        let size: usize = strlen(old_cmdline) + 1 + strlen(cmdline) + 1;
        new_cmdline = malloc(size) as *mut c_char;
        strcpy(new_cmdline, old_cmdline);
        strcat(new_cmdline, SPACE.as_ptr());
        strcat(new_cmdline, cmdline);
    }
    free((*p).cmdline as *mut c_void);
    (*p).cmdline = new_cmdline;
}
}

// ── `if (cmdline[0] == '!') … else` · machine.c:584-594 · chunk 1458 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cmdline[0] == '!') {
//             new_cmdline = strdup(cmdline + 1);
//         } else {
//             old_cmdline = p->cmdline;
//             if (!old_cmdline)
//                 old_cmdline = "";
//             new_cmdline = malloc(strlen(old_cmdline) + 1 + strlen(cmdline) + 1);
//             strcpy(new_cmdline, old_cmdline);
//             strcat(new_cmdline, " ");
//             strcat(new_cmdline, cmdline);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1458 {
use core::ffi::{c_char, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

static EMPTY: [c_char; 1] = [0];

#[no_mangle]
pub extern "C" fn vm_add_cmdline__part1458(
    cmdline: *const c_char,
    p_cmdline: *const c_char,
) -> *mut c_char {
    unsafe {
        if *cmdline == b'!' as c_char {
            // strdup(cmdline + 1)
            let src = cmdline.add(1);
            let len = strlen(src);
            let buf = malloc(len + 1) as *mut c_char;
            if buf.is_null() {
                return buf;
            }
            memcpy(buf as *mut c_void, src as *const c_void, len + 1);
            buf
        } else {
            let mut old_cmdline = p_cmdline;
            if old_cmdline.is_null() {
                old_cmdline = EMPTY.as_ptr();
            }
            let total = strlen(old_cmdline) + 1 + strlen(cmdline) + 1;
            let buf = malloc(total) as *mut c_char;
            if buf.is_null() {
                return buf;
            }
            // strcpy(buf, old_cmdline)
            let old_len_with_nul = strlen(old_cmdline) + 1;
            memcpy(
                buf as *mut c_void,
                old_cmdline as *const c_void,
                old_len_with_nul,
            );
            // strcat(buf, " ")
            let mut pos = old_len_with_nul - 1;
            *buf.add(pos) = b' ' as c_char;
            pos += 1;
            // strcat(buf, cmdline)
            let clen_with_nul = strlen(cmdline) + 1;
            memcpy(
                buf.add(pos) as *mut c_void,
                cmdline as *const c_void,
                clen_with_nul,
            );
            buf
        }
    }
}
}

// ── `free(p->cmdline); p->cmdline = new_cmdline;` · machine.c:595-596 · chunk 1459 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(p->cmdline);
//         p->cmdline = new_cmdline;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1459 {
// Port of the tail of vm_add_cmdline: replace the stored command line.
use core::ffi::{c_char, c_void};

extern "C" {
    fn free(p: *mut c_void);
}

#[no_mangle]
pub extern "C" fn vm_add_cmdline__part1459(slot: *mut *mut c_char, new_cmdline: *mut c_char) {
    unsafe {
        free(*slot as *mut c_void);
        *slot = new_cmdline;
    }
}
}

// ── `virt_machine_free_config` · machine.c:599-624 · chunk 1460 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void virt_machine_free_config(VirtMachineParams *p)
//     {
//         int i;
//     
//         free(p->machine_name);
//         free(p->cmdline);
//         for(i = 0; i < VM_FILE_COUNT; i++) {
//             free(p->files[i].filename);
//             free(p->files[i].buf);
//         }
//         for(i = 0; i < p->drive_count; i++) {
//             free(p->tab_drive[i].filename);
//             free(p->tab_drive[i].device);
//         }
//         for(i = 0; i < p->fs_count; i++) {
//             free(p->tab_fs[i].filename);
//             free(p->tab_fs[i].tag);
//         }
//         for(i = 0; i < p->eth_count; i++) {
//             free(p->tab_eth[i].driver);
//             free(p->tab_eth[i].ifname);
//         }
//         free(p->input_device);
//         free(p->display_device);
//         free(p->cfg_filename);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1460 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct VMFileEntry {
    filename: *mut c_char,
    buf: *mut u8,
    len: c_int,
}

#[repr(C)]
struct VMDriveEntry {
    device: *mut c_char,
    filename: *mut c_char,
    block_dev: *mut c_void,
}

#[repr(C)]
struct VMFSEntry {
    device: *mut c_char,
    tag: *mut c_char,
    filename: *mut c_char,
    fs_dev: *mut c_void,
}

#[repr(C)]
struct VMEthEntry {
    driver: *mut c_char,
    ifname: *mut c_char,
    net: *mut c_void,
}

const VM_FILE_COUNT: usize = 4;

#[repr(C)]
struct VirtMachineParams {
    cfg_filename: *mut c_char,
    vmc: *const c_void,
    machine_name: *mut c_char,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_local_time: c_int,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    console: *mut c_void,
    tab_drive: [VMDriveEntry; 4],
    drive_count: c_int,
    tab_fs: [VMFSEntry; 4],
    fs_count: c_int,
    tab_eth: [VMEthEntry; 1],
    eth_count: c_int,
    cmdline: *mut c_char,
    accel_enable: c_int,
    input_device: *mut c_char,
    files: [VMFileEntry; VM_FILE_COUNT],
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn virt_machine_free_config(p: *mut VirtMachineParams) {
    unsafe {
        if p.is_null() {
            return;
        }
        let p = &mut *p;

        free(p.machine_name as *mut c_void);
        free(p.cmdline as *mut c_void);

        let mut i: c_int = 0;
        while i < VM_FILE_COUNT as c_int {
            free(p.files[i as usize].filename as *mut c_void);
            free(p.files[i as usize].buf as *mut c_void);
            i += 1;
        }

        i = 0;
        while i < p.drive_count {
            free(p.tab_drive[i as usize].filename as *mut c_void);
            free(p.tab_drive[i as usize].device as *mut c_void);
            i += 1;
        }

        i = 0;
        while i < p.fs_count {
            free(p.tab_fs[i as usize].filename as *mut c_void);
            free(p.tab_fs[i as usize].tag as *mut c_void);
            i += 1;
        }

        i = 0;
        while i < p.eth_count {
            free(p.tab_eth[i as usize].driver as *mut c_void);
            free(p.tab_eth[i as usize].ifname as *mut c_void);
            i += 1;
        }

        free(p.input_device as *mut c_void);
        free(p.display_device as *mut c_void);
        free(p.cfg_filename as *mut c_void);
    }
}
}

// ── `int i; free(p->machine_name); free(p->cmdline);` · machine.c:601-604 · chunk 1461 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int i;
//     
//         free(p->machine_name);
//         free(p->cmdline);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1461 {
// Port of the first part of virt_machine_free_config:
//   free(p->machine_name);
//   free(p->cmdline);
use core::ffi::{c_char, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn virt_machine_free_config__part1461(
    machine_name: *mut c_char,
    cmdline: *mut c_char,
) {
    unsafe {
        free(machine_name as *mut c_void);
        free(cmdline as *mut c_void);
    }
}
}

// ── `for (i = 0; i < VM_FILE_COUNT; i++)` · machine.c:605-608 · chunk 1462 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < VM_FILE_COUNT; i++) {
//             free(p->files[i].filename);
//             free(p->files[i].buf);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1462 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

/// Port of `for(i = 0; i < VM_FILE_COUNT; i++) { free(p->files[i].filename); free(p->files[i].buf); }`
#[no_mangle]
pub extern "C" fn virt_machine_free_config__part1462(files: *mut VMFileEntry) {
    // VM_FILE_COUNT == 4 (VM_FILE_BIOS, VM_FILE_VGA_BIOS, VM_FILE_KERNEL, VM_FILE_INITRD)
    const VM_FILE_COUNT: usize = 4;
    let mut i: usize = 0;
    while i < VM_FILE_COUNT {
        unsafe {
            let entry = files.add(i);
            free((*entry).filename as *mut c_void);
            free((*entry).buf as *mut c_void);
        }
        i += 1;
    }
}
}

// ── `for (i = 0; i < p->drive_count; i++)` · machine.c:609-612 · chunk 1463 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->drive_count; i++) {
//             free(p->tab_drive[i].filename);
//             free(p->tab_drive[i].device);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1463 {
// Portion of virt_machine_free_config: frees the drive table entries.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

// Mirror of the C VMDriveEntry (device, filename, block_dev) -- same field
// order and pointer widths, so the layout matches exactly.
#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[no_mangle]
pub extern "C" fn virt_machine_free_config__part1463(
    tab_drive: *mut VMDriveEntry,
    drive_count: c_int,
    i_out: *mut c_int,
) {
    let mut i: c_int = 0;
    while i < drive_count {
        unsafe {
            let e = tab_drive.add(i as usize);
            free((*e).filename as *mut c_void);
            free((*e).device as *mut c_void);
        }
        i += 1;
    }
    if !i_out.is_null() {
        unsafe {
            *i_out = i;
        }
    }
}
}

// ── `for (i = 0; i < p->fs_count; i++)` · machine.c:613-616 · chunk 1464 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->fs_count; i++) {
//             free(p->tab_fs[i].filename);
//             free(p->tab_fs[i].tag);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1464 {
// Free the file-system entries of a VirtMachineParams config.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_free_config__part1464(
    i: *mut c_int,
    fs_count: c_int,
    tab_fs: *mut VMFSEntry,
) {
    let mut idx: c_int = 0;
    while idx < fs_count {
        let entry = tab_fs.add(idx as usize);
        free((*entry).filename as *mut c_void);
        free((*entry).tag as *mut c_void);
        idx += 1;
    }
    if !i.is_null() {
        *i = idx;
    }
}
}

// ── `for (i = 0; i < p->eth_count; i++)` · machine.c:617-620 · chunk 1465 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->eth_count; i++) {
//             free(p->tab_eth[i].driver);
//             free(p->tab_eth[i].ifname);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1465 {
// Free the ethernet entries of a VirtMachineParams config.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

extern "C" {
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_free_config__part1465(
    tab_eth: *mut VMEthEntry,
    eth_count: c_int,
) {
    let mut i: c_int = 0;
    while i < eth_count {
        let e: *mut VMEthEntry = tab_eth.offset(i as isize);
        free((*e).driver as *mut c_void);
        free((*e).ifname as *mut c_void);
        i += 1;
    }
}
}

// ── `free(p->input_device); free(p->display_device); free(p->cfg_filename);` · machine.c:621-623 · chunk 1466 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(p->input_device);
//         free(p->display_device);
//         free(p->cfg_filename);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1466 {
// Freeing of the scalar string members of VirtMachineParams.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn virt_machine_free_config__part1466(p: *mut VirtMachineParams) {
    if p.is_null() {
        return;
    }
    unsafe {
        free((*p).input_device as *mut c_void);
        free((*p).display_device as *mut c_void);
        free((*p).cfg_filename as *mut c_void);
    }
}
}

// ── `virt_machine_init` · machine.c:626-630 · chunk 1467 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VirtMachine *virt_machine_init(const VirtMachineParams *p)
//     {
//         const VirtMachineClass *vmc = p->vmc;
//         return vmc->virt_machine_init(p);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1467 {
// Rewrite of virt_machine_init from machine.c
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int};

// Opaque to Rust; never dereferenced here (only a pointer is passed through).
#[repr(C)]
pub struct VirtMachine {
    _opaque: [u8; 0],
}

// Mirror of the leading part of struct VirtMachineClass (machine.h).
// Only the fields up to and including virt_machine_init are needed, and their
// offsets are preserved exactly.
#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(p: *mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(p: *const VirtMachineParams) -> *mut VirtMachine>,
    // remaining members (virt_machine_end, ...) are not accessed
}

// Mirror of the leading part of VirtMachineParams (machine.h): only
// cfg_filename and vmc are needed, in that order, to read p->vmc correctly.
#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
    // remaining members are not accessed
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_init(p: *const VirtMachineParams) -> *mut VirtMachine {
    let vmc: *const VirtMachineClass = (*p).vmc;
    match (*vmc).virt_machine_init {
        Some(f) => f(p),
        None => core::ptr::null_mut(),
    }
}
}

// ── `const VirtMachineClass *vmc = p->vmc; return vmc->virt_machine_init(p);` · machine.c:628-629 · chunk 1468 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     const VirtMachineClass *vmc = p->vmc;
//         return vmc->virt_machine_init(p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1468 {
use core::ffi::c_char;

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const VirtMachineClass,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_init__part1468(
    p: *const VirtMachineParams,
) -> *mut VirtMachine {
    let vmc: *const VirtMachineClass = (*p).vmc;
    let init = (*vmc).virt_machine_init;
    match init {
        Some(f) => f(p),
        None => core::ptr::null_mut(),
    }
}
}

// ── `virt_machine_set_defaults` · machine.c:632-635 · chunk 1469 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void virt_machine_set_defaults(VirtMachineParams *p)
//     {
//         memset(p, 0, sizeof(*p));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1469 {
// Port of: void virt_machine_set_defaults(VirtMachineParams *p)
// Original C body: memset(p, 0, sizeof(*p));

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

// POINTER_SIZE_ASSUMPTION: all pointer fields keep the C layout below.

#[repr(C)]
struct VMFileEntry {
    filename: *mut c_char,
    buf: *mut u8,
    len: c_int,
}

#[repr(C)]
struct VMDriveEntry {
    device: *mut c_char,
    filename: *mut c_char,
    block_dev: *mut c_void,
}

#[repr(C)]
struct VMFSEntry {
    device: *mut c_char,
    tag: *mut c_char,
    filename: *mut c_char,
    fs_dev: *mut c_void,
}

#[repr(C)]
struct VMEthEntry {
    driver: *mut c_char,
    ifname: *mut c_char,
    net: *mut c_void,
}

// Opaque handles used only behind pointers.
#[repr(C)]
struct VirtMachineClass {
    _opaque: [u8; 0],
}

#[repr(C)]
struct CharacterDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
struct VirtMachineParams {
    cfg_filename: *mut c_char,
    vmc: *const VirtMachineClass,
    machine_name: *mut c_char,
    ram_size: u64,
    rtc_real_time: c_int,
    rtc_local_time: c_int,
    display_device: *mut c_char,
    width: c_int,
    height: c_int,
    console: *mut CharacterDevice,
    tab_drive: [VMDriveEntry; 4],
    drive_count: c_int,
    tab_fs: [VMFSEntry; 4],
    fs_count: c_int,
    tab_eth: [VMEthEntry; 1],
    eth_count: c_int,
    cmdline: *mut c_char,
    accel_enable: c_int,
    input_device: *mut c_char,
    files: [VMFileEntry; 4],
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_set_defaults(p: *mut VirtMachineParams) {
    // memset(p, 0, sizeof(*p)) with p == NULL being undefined behaviour in C too.
    if p.is_null() {
        return;
    }
    core::ptr::write_bytes(p, 0u8, 1);
}
}

// ── `memset(p, 0, sizeof(*p));` · machine.c:634-634 · chunk 1470 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(p, 0, sizeof(*p));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1470 {
// Portion of virt_machine_set_defaults: zero-initialise the VirtMachineParams.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

#[repr(C)]
pub struct VMDriveEntry {
    pub device: *mut c_char,
    pub filename: *mut c_char,
    pub block_dev: *mut c_void,
}

#[repr(C)]
pub struct VMFSEntry {
    pub device: *mut c_char,
    pub tag: *mut c_char,
    pub filename: *mut c_char,
    pub fs_dev: *mut c_void,
}

#[repr(C)]
pub struct VMEthEntry {
    pub driver: *mut c_char,
    pub ifname: *mut c_char,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const c_void,
    pub machine_name: *mut c_char,
    pub ram_size: u64,
    pub rtc_real_time: c_int,
    pub rtc_local_time: c_int,
    pub display_device: *mut c_char,
    pub width: c_int,
    pub height: c_int,
    pub console: *mut c_void,
    pub tab_drive: [VMDriveEntry; 4],
    pub drive_count: c_int,
    pub tab_fs: [VMFSEntry; 4],
    pub fs_count: c_int,
    pub tab_eth: [VMEthEntry; 1],
    pub eth_count: c_int,
    pub cmdline: *mut c_char,
    pub accel_enable: c_int,
    pub input_device: *mut c_char,
    pub files: [VMFileEntry; 4],
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_set_defaults__part1470(p: *mut VirtMachineParams) {
    if p.is_null() {
        return;
    }
    core::ptr::write_bytes(
        p as *mut u8,
        0u8,
        core::mem::size_of::<VirtMachineParams>(),
    );
}
}

// ── `virt_machine_end` · machine.c:637-640 · chunk 1471 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void virt_machine_end(VirtMachine *s)
//     {
//         s->vmc->virt_machine_end(s);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1471 {
use core::ffi::c_void;

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const core::ffi::c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut c_void)>,
    pub virt_machine_init: Option<unsafe extern "C" fn(*const c_void) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration: Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int) -> core::ffi::c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> core::ffi::c_int>,
    pub vm_send_mouse_event: Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int, core::ffi::c_int, core::ffi::c_int, core::ffi::c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, core::ffi::c_int, u16)>,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_end(s: *mut VirtMachine) {
    let vmc = (*s).vmc;
    if let Some(f) = (*vmc).virt_machine_end {
        f(s);
    }
}
}

// ── `s->vmc->virt_machine_end(s);` · machine.c:639-639 · chunk 1472 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->vmc->virt_machine_end(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1472 {
// Port of the TARGET statement inside virt_machine_end():
//   s->vmc->virt_machine_end(s);
use core::ffi::{c_char, c_int, c_uint, c_void};

// Opaque placeholders: only ever used behind pointers here.
#[repr(C)]
pub struct VirtMachineParams {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: Option<unsafe extern "C" fn(*mut VirtMachineParams)>,
    pub virt_machine_init:
        Option<unsafe extern "C" fn(*const VirtMachineParams) -> *mut VirtMachine>,
    pub virt_machine_end: Option<unsafe extern "C" fn(*mut VirtMachine)>,
    pub virt_machine_get_sleep_duration:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: Option<unsafe extern "C" fn(*mut VirtMachine) -> c_int>,
    pub vm_send_mouse_event:
        Option<unsafe extern "C" fn(*mut VirtMachine, c_int, c_int, c_int, c_uint)>,
    pub vm_send_key_event: Option<unsafe extern "C" fn(*mut VirtMachine, c_int, u16)>,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_end__part1472(s: *mut VirtMachine) {
    let vmc = (*s).vmc;
    if vmc.is_null() {
        return;
    }
    let f = (*vmc).virt_machine_end;
    if let Some(f) = f {
        f(s);
    }
}
}

