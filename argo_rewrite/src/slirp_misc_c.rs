//! From `slirp/misc.c`.

// ── `slirp_insque` · slirp/misc.c:19-29 · chunk 2440 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     inline void
//     slirp_insque(void *a, void *b)
//     {
//      register struct quehead *element = (struct quehead *) a;
//      register struct quehead *head = (struct quehead *) b;
//      element->qh_link = head->qh_link;
//      head->qh_link = (struct quehead *)element;
//      element->qh_rlink = (struct quehead *)head;
//      ((struct quehead *)(element->qh_link))->qh_rlink
//      = (struct quehead *)element;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2440 {
// Port of slirp_insque: insert element `a` into the doubly linked list after `b`.

#[repr(C)]
struct Quehead {
    qh_link: *mut Quehead,
    qh_rlink: *mut Quehead,
}

#[no_mangle]
pub extern "C" fn slirp_insque(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) {
    unsafe {
        let element = a as *mut Quehead;
        let head = b as *mut Quehead;
        (*element).qh_link = (*head).qh_link;
        (*head).qh_link = element;
        (*element).qh_rlink = head;
        (*(*element).qh_link).qh_rlink = element;
    }
}
}

// ── `register struct quehead *element = (struct quehead *) a; register struct quehead *head = (struc…` · slirp/misc.c:22-28 · chunk 2441 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct quehead *element = (struct quehead *) a;
//      register struct quehead *head = (struct quehead *) b;
//      element->qh_link = head->qh_link;
//      head->qh_link = (struct quehead *)element;
//      element->qh_rlink = (struct quehead *)head;
//      ((struct quehead *)(element->qh_link))->qh_rlink
//      = (struct quehead *)element;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2441 {
// Port of the body of slirp_insque: insert `a` after `b` in a doubly linked list.
#![allow(non_camel_case_types)]

#[repr(C)]
pub struct Quehead {
    pub qh_link: *mut Quehead,
    pub qh_rlink: *mut Quehead,
}

#[no_mangle]
pub unsafe extern "C" fn slirp_insque__part2441(
    a: *mut core::ffi::c_void,
    b: *mut core::ffi::c_void,
) {
    let element = a as *mut Quehead;
    let head = b as *mut Quehead;

    (*element).qh_link = (*head).qh_link;
    (*head).qh_link = element;
    (*element).qh_rlink = head;
    let next = (*element).qh_link;
    (*next).qh_rlink = element;
}
}

// ── `slirp_remque` · slirp/misc.c:31-38 · chunk 2442 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     inline void
//     slirp_remque(void *a)
//     {
//       register struct quehead *element = (struct quehead *) a;
//       ((struct quehead *)(element->qh_link))->qh_rlink = element->qh_rlink;
//       ((struct quehead *)(element->qh_rlink))->qh_link = element->qh_link;
//       element->qh_rlink = 
//                          ((void *)0)
//                              ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2442 {
// Rust rewrite of slirp_remque (originally remque() in slirp/misc.c).
// The queue head struct is file-local in C; mirror it here.

use core::ffi::{c_void, c_int};

#[repr(C)]
struct QueHead {
    qh_link: *mut QueHead,
    qh_rlink: *mut QueHead,
}

// Keep the type usable / avoid unused warnings for c_int (not needed but harmless).
#[allow(dead_code)]
type _Unused = c_int;

#[no_mangle]
pub extern "C" fn slirp_remque(a: *mut c_void) {
    unsafe {
        let element = a as *mut QueHead;
        let link = (*element).qh_link;
        let rlink = (*element).qh_rlink;
        (*link).qh_rlink = rlink;
        (*rlink).qh_link = link;
        (*element).qh_rlink = core::ptr::null_mut();
    }
}
}

// ── `register struct quehead *element = (struct quehead *) a; ((struct quehead *)(element->qh_link))…` · slirp/misc.c:34-37 · chunk 2443 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     register struct quehead *element = (struct quehead *) a;
//       ((struct quehead *)(element->qh_link))->qh_rlink = element->qh_rlink;
//       ((struct quehead *)(element->qh_rlink))->qh_link = element->qh_link;
//       element->qh_rlink = 
//                          ((void *)0)
//                              ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2443 {
// Port of the body of slirp_remque (slirp/misc.c lines 34-37).

use core::ffi::c_void;
use core::ptr;

#[repr(C)]
pub struct QueHead {
    pub qh_link: *mut QueHead,
    pub qh_rlink: *mut QueHead,
}

#[no_mangle]
pub extern "C" fn slirp_remque__part2443(a: *mut c_void) {
    unsafe {
        let element = a as *mut QueHead;
        let link = (*element).qh_link;
        let rlink = (*element).qh_rlink;

        (*link).qh_rlink = rlink;
        (*rlink).qh_link = link;
        (*element).qh_rlink = ptr::null_mut();
    }
}
}

// ── `add_exec` · slirp/misc.c:40-60 · chunk 2444 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int add_exec(struct ex_list **ex_ptr, int do_pty, char *exec,
//                  struct in_addr addr, int port)
//     {
//      struct ex_list *tmp_ptr;
//     
//     
//      for (tmp_ptr = *ex_ptr; tmp_ptr; tmp_ptr = tmp_ptr->ex_next) {
//       if (port == tmp_ptr->ex_fport &&
//           addr.s_addr == tmp_ptr->ex_addr.s_addr)
//        return -1;
//      }
//     
//      tmp_ptr = *ex_ptr;
//      *ex_ptr = (struct ex_list *)malloc(sizeof(struct ex_list));
//      (*ex_ptr)->ex_fport = port;
//      (*ex_ptr)->ex_addr = addr;
//      (*ex_ptr)->ex_pty = do_pty;
//      (*ex_ptr)->ex_exec = (do_pty == 3) ? exec : strdup(exec);
//      (*ex_ptr)->ex_next = tmp_ptr;
//      return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2444 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn add_exec(
    ex_ptr: *mut *mut ExList,
    do_pty: c_int,
    exec: *mut c_char,
    addr: InAddr,
    port: c_int,
) -> c_int {
    unsafe {
        let mut tmp_ptr: *mut ExList = *ex_ptr;
        while !tmp_ptr.is_null() {
            if port == (*tmp_ptr).ex_fport && addr.s_addr == (*tmp_ptr).ex_addr.s_addr {
                return -1;
            }
            tmp_ptr = (*tmp_ptr).ex_next;
        }

        tmp_ptr = *ex_ptr;
        *ex_ptr = malloc(core::mem::size_of::<ExList>()) as *mut ExList;
        let new_ptr = *ex_ptr;
        (*new_ptr).ex_fport = port;
        (*new_ptr).ex_addr = addr;
        (*new_ptr).ex_pty = do_pty;
        (*new_ptr).ex_exec = if do_pty == 3 {
            exec as *const c_char
        } else {
            strdup(exec as *const c_char) as *const c_char
        };
        (*new_ptr).ex_next = tmp_ptr;
        0
    }
}
}

// ── `for (tmp_ptr = *ex_ptr; tmp_ptr; tmp_ptr = tmp_ptr->ex_next)` · slirp/misc.c:46-50 · chunk 2445 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (tmp_ptr = *ex_ptr; tmp_ptr; tmp_ptr = tmp_ptr->ex_next) {
//       if (port == tmp_ptr->ex_fport &&
//           addr.s_addr == tmp_ptr->ex_addr.s_addr)
//        return -1;
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2445 {
// Port of the duplicate-exec check loop from add_exec (slirp/misc.c).
use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const core::ffi::c_char,
    pub ex_next: *mut ExList,
}

#[no_mangle]
pub extern "C" fn add_exec__part2445(
    ex_ptr: *mut *mut ExList,
    addr: InAddr,
    port: c_int,
) -> c_int {
    let mut tmp_ptr: *mut ExList = unsafe { *ex_ptr };
    while !tmp_ptr.is_null() {
        unsafe {
            if port == (*tmp_ptr).ex_fport && addr.s_addr == (*tmp_ptr).ex_addr.s_addr {
                return -1;
            }
            tmp_ptr = (*tmp_ptr).ex_next;
        }
    }
    0
}
}

// ── `tmp_ptr = *ex_ptr; *ex_ptr = (struct ex_list *)malloc(sizeof(struct ex_list)); (*ex_ptr)->ex_fp…` · slirp/misc.c:52-59 · chunk 2446 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     tmp_ptr = *ex_ptr;
//      *ex_ptr = (struct ex_list *)malloc(sizeof(struct ex_list));
//      (*ex_ptr)->ex_fport = port;
//      (*ex_ptr)->ex_addr = addr;
//      (*ex_ptr)->ex_pty = do_pty;
//      (*ex_ptr)->ex_exec = (do_pty == 3) ? exec : strdup(exec);
//      (*ex_ptr)->ex_next = tmp_ptr;
//      return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2446 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn add_exec__part2446(
    ex_ptr: *mut *mut ExList,
    do_pty: c_int,
    exec: *mut c_char,
    addr: InAddr,
    port: c_int,
) -> c_int {
    unsafe {
        let tmp_ptr: *mut ExList = *ex_ptr;
        let new_ptr: *mut ExList = malloc(core::mem::size_of::<ExList>()) as *mut ExList;
        *ex_ptr = new_ptr;
        (*new_ptr).ex_fport = port;
        (*new_ptr).ex_addr = addr;
        (*new_ptr).ex_pty = do_pty;
        (*new_ptr).ex_exec = if do_pty == 3 {
            exec as *const c_char
        } else {
            strdup(exec) as *const c_char
        };
        (*new_ptr).ex_next = tmp_ptr;
        0
    }
}
}

// ── `os_socket` · slirp/misc.c:83-86 · chunk 2447 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int os_socket(int domain, int type, int protocol)
//     {
//         return socket(domain, type, protocol);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2447 {
// Port of slirp's os_socket(): thin wrapper over libc socket(2).
use core::ffi::c_int;

extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn os_socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    unsafe { socket(domain, type_, protocol) }
}
}

// ── `return socket(domain, type, protocol);` · slirp/misc.c:85-85 · chunk 2448 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return socket(domain, type, protocol);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2448 {
use core::ffi::c_int;

extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn os_socket__part2448(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    unsafe { socket(domain, type_, protocol) }
}
}

// ── `os_get_time_ms` · slirp/misc.c:88-95 · chunk 2449 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     uint32_t os_get_time_ms(void)
//     {
//         struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return ts.tv_sec * 1000 +
//             (ts.tv_nsec / 1000000);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2449 {
use core::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;

#[no_mangle]
pub extern "C" fn os_get_time_ms() -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
    }

    (ts.tv_sec.wrapping_mul(1000).wrapping_add(ts.tv_nsec / 1000000)) as u32
}
}

// ── `struct timespec ts; clock_gettime( 1 , &ts); return ts.tv_sec * 1000 + (ts.tv_nsec / 1000000);` · slirp/misc.c:90-94 · chunk 2450 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct timespec ts;
//     
//         clock_gettime(
//                      1
//                                     , &ts);
//         return ts.tv_sec * 1000 +
//             (ts.tv_nsec / 1000000);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2450 {
// Port of the timing body of os_get_time_ms (slirp/misc.c).
use core::ffi::{c_int, c_long};

#[repr(C)]
struct Timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

extern "C" {
    fn clock_gettime(clk_id: c_int, tp: *mut Timespec) -> c_int;
}

#[no_mangle]
pub extern "C" fn os_get_time_ms__part2450() -> u32 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        clock_gettime(1, &mut ts);
    }
    let ms = ts.tv_sec.wrapping_mul(1000).wrapping_add(ts.tv_nsec / 1000000);
    ms as u32
}
}

// ── `fork_exec` · slirp/misc.c:100-105 · chunk 2451 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int
//     fork_exec(struct socket *so, const char *ex, int do_pty)
//     {
//     
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2451 {
// Rewrite of slirp/misc.c fork_exec (not implemented variant).
// The C definition simply returns 0 and ignores its arguments.

use core::ffi::{c_char, c_int, c_void};

/// Opaque stand-in for `struct socket`; the ported function never dereferences it.
#[repr(C)]
pub struct Socket {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn fork_exec(
    _so: *mut Socket,
    _ex: *const c_char,
    _do_pty: c_int,
) -> c_int {
    // Suppress any "unused" concerns and keep exact C behavior: return 0.
    let _ = core::mem::size_of::<*mut c_void>();
    0
}
}

// ── `return 0;` · slirp/misc.c:104-104 · chunk 2452 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2452 {
// Port of the `return 0;` statement inside slirp's fork_exec (stubbed out).
use core::ffi::{c_char, c_int, c_void};

#[no_mangle]
pub extern "C" fn fork_exec__part2452(
    _so: *mut c_void,
    _ex: *const c_char,
    _do_pty: c_int,
) -> c_int {
    0
}
}

// ── `fd_nonblock` · slirp/misc.c:280-298 · chunk 2454 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     fd_nonblock(int fd)
//     {
//     
//     
//     
//     
//             int opt = 1;
//     
//     
//      ioctl(fd, 
//                     0x5421
//                            , &opt);
//     
//     
//     
//     
//     
//     
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2454 {
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

const FIONBIO: c_ulong = 0x5421;

#[no_mangle]
pub extern "C" fn fd_nonblock(fd: c_int) {
    let mut opt: c_int = 1;
    unsafe {
        ioctl(fd, FIONBIO, &mut opt as *mut c_int);
    }
}
}

// ── `int opt = 1; ioctl(fd, 0x5421 , &opt);` · slirp/misc.c:287-290 · chunk 2455 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int opt = 1;
//     
//     
//      ioctl(fd, 
//                     0x5421
//                            , &opt);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2455 {
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn fd_nonblock__part2455(fd: c_int) {
    let opt: c_int = 1;
    unsafe {
        ioctl(fd, 0x5421u32 as c_ulong, &opt as *const c_int);
    }
}
}

// ── `fd_block` · slirp/misc.c:300-318 · chunk 2456 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void
//     fd_block(int fd)
//     {
//     
//     
//     
//     
//      int opt = 0;
//     
//     
//      ioctl(fd, 
//                     0x5421
//                            , &opt);
//     
//     
//     
//     
//     
//     
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2456 {
use core::ffi::{c_int, c_ulong};

extern "C" {
    // extern int ioctl (int __fd, unsigned long int __request, ...);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

const FIONBIO: c_ulong = 0x5421;

#[no_mangle]
pub extern "C" fn fd_block(fd: c_int) {
    let mut opt: c_int = 0;
    unsafe {
        ioctl(fd, FIONBIO, &mut opt as *mut c_int);
    }
}
}

// ── `int opt = 0; ioctl(fd, 0x5421 , &opt);` · slirp/misc.c:307-310 · chunk 2457 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int opt = 0;
//     
//     
//      ioctl(fd, 
//                     0x5421
//                            , &opt);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2457 {
use core::ffi::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn fd_block__part2457(fd: c_int) {
    let opt: c_int = 0;
    unsafe {
        ioctl(fd, 0x5421 as c_ulong, &opt as *const c_int);
    }
}
}

