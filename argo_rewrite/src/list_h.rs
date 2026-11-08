//! From `list.h`.

// ── `init_list_head` · list.h:36-40 · chunk 208 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void init_list_head(struct list_head *head)
//     {
//         head->prev = head;
//         head->next = head;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk208 {
// Rewrite of `static inline void init_list_head(struct list_head *head)` from list.h.
// Initializes the list head so that it is an empty circular doubly linked list.

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

#[no_mangle]
pub unsafe extern "C" fn init_list_head(head: *mut list_head) {
    (*head).prev = head;
    (*head).next = head;
}
}

// ── `head->prev = head; head->next = head;` · list.h:38-39 · chunk 209 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     head->prev = head;
//         head->next = head;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk209 {
#[repr(C)]
struct list_head {
    prev: *mut list_head,
    next: *mut list_head,
}

#[no_mangle]
pub extern "C" fn init_list_head__part209(head: *mut list_head) {
    unsafe {
        (*head).prev = head;
        (*head).next = head;
    }
}
}

// ── `__list_add` · list.h:43-50 · chunk 210 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void __list_add(struct list_head *el,
//                                   struct list_head *prev, struct list_head *next)
//     {
//         prev->next = el;
//         el->prev = prev;
//         el->next = next;
//         next->prev = el;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk210 {
// Rewrite of list.h's __list_add:
//   prev->next = el;
//   el->prev = prev;
//   el->next = next;
//   next->prev = el;

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

#[no_mangle]
pub unsafe extern "C" fn __list_add(
    el: *mut list_head,
    prev: *mut list_head,
    next: *mut list_head,
) {
    (*prev).next = el;
    (*el).prev = prev;
    (*el).next = next;
    (*next).prev = el;
}
}

// ── `prev->next = el; el->prev = prev; el->next = next; next->prev = el;` · list.h:46-49 · chunk 211 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     prev->next = el;
//         el->prev = prev;
//         el->next = next;
//         next->prev = el;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk211 {
#[repr(C)]
struct ListHead {
    prev: *mut ListHead,
    next: *mut ListHead,
}

#[no_mangle]
pub unsafe extern "C" fn __list_add__part211(
    el: *mut ListHead,
    prev: *mut ListHead,
    next: *mut ListHead,
) {
    (*prev).next = el;
    (*el).prev = prev;
    (*el).next = next;
    (*next).prev = el;
}
}

// ── `list_add` · list.h:53-56 · chunk 212 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void list_add(struct list_head *el, struct list_head *head)
//     {
//         __list_add(el, head, head->next);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk212 {
// Port of the static inline C function:
//   static inline void list_add(struct list_head *el, struct list_head *head)
//   {
//       __list_add(el, head, head->next);
//   }
// with __list_add inlined (arguments are evaluated before any store, as in C).

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

#[no_mangle]
pub extern "C" fn list_add(el: *mut list_head, head: *mut list_head) {
    unsafe {
        // `next` is read before the stores, matching C argument evaluation.
        let next: *mut list_head = (*head).next;
        let prev: *mut list_head = head;
        (*prev).next = el;
        (*el).prev = prev;
        (*el).next = next;
        (*next).prev = el;
    }
}
}

// ── `__list_add(el, head, head->next);` · list.h:55-55 · chunk 213 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     __list_add(el, head, head->next);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk213 {
// Ported from C: __list_add
// Modifies three list_head structs in place: prev->next=el, el->prev=prev, el->next=next, next->prev=el

#[repr(C)]
struct ListHead {
    prev: *mut ListHead,
    next: *mut ListHead,
}

#[no_mangle]
pub extern "C" fn list_add__part213(el: *mut ListHead, prev: *mut ListHead, next: *mut ListHead) {
    unsafe {
        // prev->next = el;
        (*prev).next = el;
        // el->prev = prev;
        (*el).prev = prev;
        // el->next = next;
        (*el).next = next;
        // next->prev = el;
        (*next).prev = el;
    }
}
}

// ── `list_add_tail` · list.h:59-62 · chunk 214 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline void list_add_tail(struct list_head *el, struct list_head *head)
//     {
//         __list_add(el, head->prev, head);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk214 {
// Rewrite of list_add_tail from list.h

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

// Ported from the static inline __list_add in list.h:
// insert `el` between `prev` and `next`.
#[inline]
unsafe fn __list_add(el: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*prev).next = el;
    (*el).prev = prev;
    (*el).next = next;
    (*next).prev = el;
}

#[no_mangle]
pub unsafe extern "C" fn list_add_tail(el: *mut list_head, head: *mut list_head) {
    __list_add(el, (*head).prev, head);
}
}

// ── `__list_add(el, head->prev, head);` · list.h:61-61 · chunk 215 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     __list_add(el, head->prev, head);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk215 {
// Port of __list_add from list.h
// This function inserts a node (el) into a doubly-linked list before another node (prev)

#[repr(C)]
struct list_head {
    prev: *mut list_head,
    next: *mut list_head,
}

#[no_mangle]
pub extern "C" fn list_add_tail__part215(
    el: *mut list_head,
    prev: *mut list_head,
    next: *mut list_head,
) {
    unsafe {
        // Mirror the C __list_add implementation exactly:
        // prev->next = el;
        // el->prev = prev;
        // el->next = next;
        // next->prev = el;
        (*prev).next = el;
        (*el).prev = prev;
        (*el).next = next;
        (*next).prev = el;
    }
}
}

// ── `list_del` · list.h:64-73 · chunk 216 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static inline void list_del(struct list_head *el)
//     {
//         struct list_head *prev, *next;
//         prev = el->prev;
//         next = el->next;
//         prev->next = next;
//         next->prev = prev;
//         el->prev = 
//                   ((void *)0)
//                       ;
//         el->next = 
//                   ((void *)0)
//                       ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk216 {
// Rust rewrite of `list_del` from list.h
#![allow(non_camel_case_types)]

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

#[no_mangle]
pub unsafe extern "C" fn list_del(el: *mut list_head) {
    let prev = (*el).prev;
    let next = (*el).next;
    (*prev).next = next;
    (*next).prev = prev;
    (*el).prev = core::ptr::null_mut();
    (*el).next = core::ptr::null_mut();
}
}

// ── `struct list_head *prev, *next; prev = el->prev; next = el->next; prev->next = next; next->prev…` · list.h:66-72 · chunk 217 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct list_head *prev, *next;
//         prev = el->prev;
//         next = el->next;
//         prev->next = next;
//         next->prev = prev;
//         el->prev = 
//                   ((void *)0)
//                       ;
//         el->next = 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk217 {
#[repr(C)]
struct list_head {
    prev: *mut list_head,
    next: *mut list_head,
}

#[no_mangle]
pub extern "C" fn list_del__part217(el: *mut list_head) {
    unsafe {
        let prev = (*el).prev;
        let next = (*el).next;
        (*prev).next = next;
        (*next).prev = prev;
        (*el).prev = core::ptr::null_mut();
        (*el).next = core::ptr::null_mut();
    }
}
}

// ── `list_empty` · list.h:75-78 · chunk 218 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static inline int list_empty(struct list_head *el)
//     {
//         return el->next == el;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk218 {
use core::ffi::c_int;

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[no_mangle]
pub extern "C" fn list_empty(el: *mut ListHead) -> c_int {
    unsafe {
        if (*el).next == el {
            1
        } else {
            0
        }
    }
}
}

// ── `return el->next == el;` · list.h:77-77 · chunk 219 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     return el->next == el;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk219 {
use core::ffi::c_int;

#[repr(C)]
struct list_head {
    prev: *const list_head,
    next: *const list_head,
}

#[no_mangle]
pub extern "C" fn list_empty__part219(el: *const list_head) -> c_int {
    unsafe {
        ((*el).next == el) as c_int
    }
}
}

