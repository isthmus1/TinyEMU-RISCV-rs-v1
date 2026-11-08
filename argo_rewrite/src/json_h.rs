//! From `json.h`.

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

