//! From `json.h`.

// ── `json_is_error` · json.h:84-87 · chunk 236 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL json_is_error(JSONValue val)
//     {
//         return val.type == JSON_EXCEPTION;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk236 {
// Port of `static inline BOOL json_is_error(JSONValue val)` from json.h.
//
//   static inline BOOL json_is_error(JSONValue val)
//   {
//       return val.type == JSON_EXCEPTION;
//   }

use core::ffi::c_int;

// BOOL is `typedef int BOOL;`
type BOOL = c_int;

// JSONTypeEnum: C enum with only non-negative enumerators -> 4 bytes.
const JSON_EXCEPTION: c_int = 7;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0]; flexible array member, never accessed here
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
    pub r#type: c_int,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_is_error(val: JSONValue) -> BOOL {
    // Read only the type tag; the union is left untouched.
    if val.r#type == JSON_EXCEPTION {
        1
    } else {
        0
    }
}
}

// ── `return val.type == JSON_EXCEPTION;` · json.h:86-86 · chunk 237 · nvidia/nemotron-3.5-lightning · 5 attempt(s) · Unvalidated
//
// Original C:
//     return val.type == JSON_EXCEPTION;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk237 {
use core::ffi::c_int;

#[repr(C)]
#[derive(PartialEq)]
enum JSONTypeEnum {
    JSON_STR,
    JSON_INT,
    JSON_OBJ,
    JSON_ARRAY,
    JSON_BOOL,
    JSON_NULL,
    JSON_UNDEFINED,
    JSON_EXCEPTION,
}

#[repr(C)]
union JSONValueUnion {
    str: *const (),
    int32: c_int,
    b: c_int,
    obj: *const (),
    array: *const (),
}

#[repr(C)]
struct JSONValue {
    r#type: JSONTypeEnum,
    u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_is_error__part237(val: JSONValue) -> c_int {
    (val.r#type == JSONTypeEnum::JSON_EXCEPTION) as c_int
}
}

// ── `json_is_undefined` · json.h:89-92 · chunk 238 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline BOOL json_is_undefined(JSONValue val)
//     {
//         return val.type == JSON_UNDEFINED;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk238 {
// Rust rewrite of json.h's static inline json_is_undefined.
use core::ffi::c_int;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // zero-length flexible array member; not accessed here
}

#[repr(C)]
pub union JSONValueU {
    pub str: *mut JSONString,
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

const JSON_UNDEFINED: c_int = 6;

#[no_mangle]
pub extern "C" fn json_is_undefined(val: JSONValue) -> c_int {
    if val.type_ == JSON_UNDEFINED {
        1
    } else {
        0
    }
}
}

// ── `return val.type == JSON_UNDEFINED;` · json.h:91-91 · chunk 239 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return val.type == JSON_UNDEFINED;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk239 {
use std::os::raw::c_void;
use std::ffi::c_int;

#[repr(C)]
pub enum JSONTypeEnum {
    JSON_STR,
    JSON_INT,
    JSON_OBJ,
    JSON_ARRAY,
    JSON_BOOL,
    JSON_NULL,
    JSON_UNDEFINED,
    JSON_EXCEPTION,
}

#[repr(C)]
pub union JSONValueUnion {
    str: *mut c_void,
    i32: c_int,
    b: c_int,
    obj: *mut c_void,
    arr: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub type_: JSONTypeEnum,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_is_undefined__part239(val: JSONValue) -> c_int {
    ((val.type_ as c_int) == (JSONTypeEnum::JSON_UNDEFINED as c_int)) as c_int
}
}

// ── `json_undefined_new` · json.h:94-100 · chunk 240 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline JSONValue json_undefined_new(void)
//     {
//         JSONValue val;
//         val.type = JSON_UNDEFINED;
//         val.u.int32 = 0;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk240 {
// Port of the static inline C helper `json_undefined_new` from json.h.
// JSONValue is:
//   typedef struct JSONValue {
//       JSONTypeEnum type;      /* int */
//       union { JSONString *str; int int32; BOOL b;
//               struct JSONObject *obj; struct JSONArray *array; } u;
//   } JSONValue;
// On SysV x86-64 this is a 16-byte struct (4-byte enum + 4 padding + 8-byte union).

use core::ffi::c_int;

const JSON_UNDEFINED: c_int = 6;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0] flexible array member; never accessed here.
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
#[derive(Copy, Clone)]
pub union JSONValueU {
    pub str_: *mut JSONString,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut JSONObject,
    pub array: *mut JSONArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSONValue {
    pub type_: c_int,
    pub u: JSONValueU,
}

#[no_mangle]
pub extern "C" fn json_undefined_new() -> JSONValue {
    JSONValue {
        type_: JSON_UNDEFINED,
        u: JSONValueU { int32: 0 },
    }
}
}

// ── `JSONValue val; val.type = JSON_UNDEFINED; val.u.int32 = 0; return val;` · json.h:96-99 · chunk 241 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val.type = JSON_UNDEFINED;
//         val.u.int32 = 0;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk241 {
use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
union JSONValueUnion {
    int32: c_int,
    b: c_int,
    str: *mut c_void,
    obj: *mut c_void,
    array: *mut c_void,
}

#[repr(C)]
struct JSONValue {
    r#type: c_int,
    u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_undefined_new__part241() -> JSONValue {
    let mut val = JSONValue {
        r#type: 6 as c_int,
        u: JSONValueUnion {
            int32: 0,
        },
    };
    val
}
}

// ── `json_null_new` · json.h:102-108 · chunk 242 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline JSONValue json_null_new(void)
//     {
//         JSONValue val;
//         val.type = JSON_NULL;
//         val.u.int32 = 0;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk242 {
// Port of `static inline JSONValue json_null_new(void)` from json.h
// (json.h lines 102-108). Exported as an external symbol so it can replace
// the C inline definition.

use core::ffi::{c_int, c_void};

/// Mirror of C `JSONTypeEnum` (an `int`-sized enum).
pub type JSONTypeEnum = c_int;

/// The value `JSON_NULL` of the C enum `JSONTypeEnum`.
pub const JSON_NULL: JSONTypeEnum = 5;

/// Mirror of the anonymous union inside C `JSONValue`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union JSONValueUnion {
    pub str: *mut c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut c_void,
    pub array: *mut c_void,
}

/// Mirror of C `JSONValue`:
///     typedef struct JSONValue {
///         JSONTypeEnum type;
///         union { ... } u;
///     } JSONValue;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JSONValue {
    pub r#type: JSONTypeEnum,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_null_new() -> JSONValue {
    JSONValue {
        r#type: JSON_NULL,
        u: JSONValueUnion { int32: 0 },
    }
}
}

// ── `JSONValue val; val.type = JSON_NULL; val.u.int32 = 0; return val;` · json.h:104-107 · chunk 243 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val.type = JSON_NULL;
//         val.u.int32 = 0;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk243 {
use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
union JSONValueUnion {
    int32: c_int,
    b: c_int,
    str: *mut c_void,
    obj: *mut c_void,
    array: *mut c_void,
}

#[repr(C)]
struct JSONValue {
    type_: c_int,
    u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_null_new__part243() -> JSONValue {
    JSONValue {
        type_: 5,
        u: JSONValueUnion {
            int32: 0,
        },
    }
}
}

// ── `json_int32_new` · json.h:110-116 · chunk 244 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline JSONValue json_int32_new(int v)
//     {
//         JSONValue val;
//         val.type = JSON_INT;
//         val.u.int32 = v;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk244 {
// Rust rewrite of the C function:
//   static inline JSONValue json_int32_new(int v)
//   {
//       JSONValue val;
//       val.type = JSON_INT;
//       val.u.int32 = v;
//       return val;
//   }

use core::ffi::c_int;

// typedef int BOOL;
pub type BOOL = c_int;

// typedef enum { JSON_STR, JSON_INT, JSON_OBJ, JSON_ARRAY, JSON_BOOL,
//                JSON_NULL, JSON_UNDEFINED, JSON_EXCEPTION } JSONTypeEnum;
pub type JSONTypeEnum = c_int;
const JSON_INT: JSONTypeEnum = 1;

#[repr(C)]
pub struct JSONString {
    pub len: c_int,
    // char data[0];  (flexible array member)
}

pub enum JSONObject {}
pub enum JSONArray {}

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

#[no_mangle]
pub extern "C" fn json_int32_new(v: c_int) -> JSONValue {
    // Zero the whole value (matching a fully initialized 16-byte struct).
    let mut val: JSONValue = unsafe { core::mem::zeroed() };
    val.type_ = JSON_INT;
    val.u.int32 = v;
    val
}
}

// ── `JSONValue val; val.type = JSON_INT; val.u.int32 = v; return val;` · json.h:112-115 · chunk 245 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val.type = JSON_INT;
//         val.u.int32 = v;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk245 {
use std::os::raw::c_int;
use std::os::raw::c_void;

#[repr(C)]
union JSONValueUnion {
    str: *mut c_void,
    int32: c_int,
    b: c_int,
    obj: *mut c_void,
    array: *mut c_void,
}

#[repr(C)]
pub struct JSONValue {
    type_: c_int,
    u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_int32_new__part245(v: c_int) -> JSONValue {
    JSONValue {
        type_: 1,
        u: JSONValueUnion {
            int32: v,
        },
    }
}
}

// ── `json_bool_new` · json.h:118-124 · chunk 246 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline JSONValue json_bool_new(BOOL v)
//     {
//         JSONValue val;
//         val.type = JSON_BOOL;
//         val.u.b = v;
//         return val;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk246 {
// Rewrite of json_bool_new from json.h
// BOOL is `typedef int BOOL;` and JSONTypeEnum is a C enum (int-sized).
// JSONValue is { JSONTypeEnum type; union { ... } u; }

use core::ffi::c_int;

const JSON_BOOL: c_int = 4;

#[repr(C)]
pub union JSONValueUnion {
    pub str_: *mut core::ffi::c_void,
    pub int32: c_int,
    pub b: c_int,
    pub obj: *mut core::ffi::c_void,
    pub array: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct JSONValue {
    pub r#type: c_int,
    pub u: JSONValueUnion,
}

#[no_mangle]
pub extern "C" fn json_bool_new(v: c_int) -> JSONValue {
    JSONValue {
        r#type: JSON_BOOL,
        u: JSONValueUnion { b: v },
    }
}
}

// ── `JSONValue val; val.type = JSON_BOOL; val.u.b = v; return val;` · json.h:120-123 · chunk 247 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     JSONValue val;
//         val.type = JSON_BOOL;
//         val.u.b = v;
//         return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk247 {
use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
enum JsonTypeEnum {
    JSON_STR = 0,
    JSON_INT,
    JSON_OBJ,
    JSON_ARRAY,
    JSON_BOOL,
    JSON_NULL,
    JSON_UNDEFINED,
    JSON_EXCEPTION,
}

#[repr(C)]
union JsonValueUnion {
    _str: *mut c_void,
    _int32: c_int,
    _b: c_int,
    _obj: *mut c_void,
    _array: *mut c_void,
}

#[repr(C)]
struct JSONValue {
    type_: JsonTypeEnum,
    u: JsonValueUnion,
}

#[no_mangle]
pub extern "C" fn json_bool_new__part247(v: c_int) -> JSONValue {
    let val = JSONValue {
        type_: JsonTypeEnum::JSON_BOOL,
        u: JsonValueUnion { _b: v },
    };
    val
}
}

