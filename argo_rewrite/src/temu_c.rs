//! From `temu.c`.

// ── `tcsetattr (0, 0 , &oldtty); fcntl(0, 4 , old_fd0_flags);` · temu.c:75-76 · chunk 3511 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tcsetattr (0, 
//                      0
//                             , &oldtty);
//         fcntl(0, 
//                 4
//                        , old_fd0_flags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3511 {
// Port of the tail of term_exit: restore the terminal attributes and the
// original file status flags of fd 0.
use core::ffi::{c_int, c_void};

extern "C" {
    // int tcsetattr(int __fd, int __optional_actions, const struct termios *__termios_p);
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const c_void) -> c_int;
    // extern int fcntl(int __fd, int __cmd, ...) __asm__("fcntl64");
    #[link_name = "fcntl64"]
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn term_exit__part3511(tty: *const c_void, fd0_flags: c_int) {
    // tcsetattr(0, 0 /* TCSANOW */, &oldtty);
    let _ = tcsetattr(0, 0, tty);
    // fcntl(0, 4 /* F_SETFL */, old_fd0_flags);
    let _ = fcntl(0, 4, fd0_flags);
}
}

// ── `struct termios tty; memset(&tty, 0, sizeof(tty)); tcgetattr (0, &tty); oldtty = tty; old_fd0_fl…` · temu.c:81-91 · chunk 3513 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct termios tty;
//     
//         memset(&tty, 0, sizeof(tty));
//         tcgetattr (0, &tty);
//         oldtty = tty;
//         old_fd0_flags = fcntl(0, 
//                                 3
//                                        );
//     
//         tty.c_iflag &= ~(
//                         0000001
//                               |
//                                0000002
//                                      |
//                                       0000010
//                                             |
//                                              0000040
//                               
//                              |
//                               0000100
//                                    |
//                                     0000200
//                                          |
//                                           0000400
//                                                |
//                                                 0002000
//                                                     );
//         tty.c_oflag |= 
//                       0000001
//                            ;
//         tty.c_lflag &= ~(
//                         0000010
//                             |
//                              0000100
//                                    |
//                                     0000002
//                                           |
//                                            0100000
//                                                  );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3513 {
// Port of the first part of term_init (temu.c lines 81-91):
// zero/read the termios structure, save the old terminal state and fd0 flags,
// and adjust the raw-mode flags.

use core::ffi::{c_int, c_uchar, c_uint};
use core::mem::size_of;

// Mirror of glibc's `struct termios` (the anonymous speed unions hold a
// single `speed_t` each, so a plain c_uint field has the same layout).
#[repr(C)]
pub struct Termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_line: c_uchar,
    pub c_cc: [c_uchar; 32],
    pub c_ispeed: c_uint,
    pub c_ospeed: c_uint,
}

extern "C" {
    fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;
    // The C translation unit refers to fcntl as the "fcntl64" symbol.
    #[link_name = "fcntl64"]
    fn fcntl64(fd: c_int, cmd: c_int, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn term_init__part3513(
    tty: *mut Termios,
    oldtty_out: *mut Termios,
    old_fd0_flags_out: *mut c_int,
) {
    let n = size_of::<Termios>();

    /* memset(&tty, 0, sizeof(tty)); */
    core::ptr::write_bytes(tty as *mut u8, 0, n);

    /* tcgetattr(0, &tty); */
    tcgetattr(0, tty);

    /* oldtty = tty; */
    core::ptr::copy_nonoverlapping(tty as *const u8, oldtty_out as *mut u8, n);

    /* old_fd0_flags = fcntl(0, F_GETFD); */
    *old_fd0_flags_out = fcntl64(0, 3);

    let t = &mut *tty;

    /* BRKINT|ICRNL|INPCK|ISTRIP|INLCR|IGNCR|IXON are the raw-mode clears */
    let iflag_clear: c_uint = 0000001u32 | 0000002 | 0000010 | 0000040 | 0000100 | 0000200 | 0000400
        | 0002000;
    t.c_iflag &= !iflag_clear;

    /* tty.c_oflag |= OPOST; */
    t.c_oflag |= 0000001u32;

    let lflag_clear: c_uint = 0000010u32 | 0000100 | 0000002 | 0100000;
    t.c_lflag &= !lflag_clear;
}
}

// ── `if (!allow_ctrlc)` · temu.c:92-93 · chunk 3514 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!allow_ctrlc)
//             tty.c_lflag &= ~
//                            0000001
//                                ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3514 {
// Port of the `if (!allow_ctrlc) tty.c_lflag &= ~0000001;` statement of term_init.
// tcflag_t is `unsigned int`; mask 0000001 (octal) == 1 (ISIG).

#[no_mangle]
pub extern "C" fn term_init__part3514(c_lflag: *mut u32, allow_ctrlc: i32) {
    // C: if (!allow_ctrlc)  ->  true when allow_ctrlc == 0
    if allow_ctrlc == 0 {
        unsafe {
            if !c_lflag.is_null() {
                *c_lflag &= !1u32;
            }
        }
    }
}
}

// ── `tty.c_cflag &= ~( 0000060 | 0000400 ); tty.c_cflag |= 0000060 ; tty.c_cc[ 6 ] = 1; tty.c_cc[ 5…` · temu.c:94-101 · chunk 3515 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tty.c_cflag &= ~(
//                         0000060
//                              |
//                               0000400
//                                     );
//         tty.c_cflag |= 
//                       0000060
//                          ;
//         tty.c_cc[
//                 6
//                     ] = 1;
//         tty.c_cc[
//                 5
//                      ] = 0;
//     
//         tcsetattr (0, 
//                      0
//                             , &tty);
//     
//         atexit(term_exit);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3515 {
// Port of the tty setup tail of term_init (temu.c lines 94-101).
#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

extern "C" {
    fn tcsetattr(fd: c_int, optional_actions: c_int, t: *const Termios) -> c_int;
    fn atexit(cb: extern "C" fn()) -> c_int;
}

#[no_mangle]
pub extern "C" fn term_init__part3515(tty: *mut Termios, term_exit_fn: extern "C" fn()) {
    unsafe {
        if !tty.is_null() {
            (*tty).c_cflag &= !(0o60u32 | 0o400u32);
            (*tty).c_cflag |= 0o60u32;
            (*tty).c_cc[6] = 1;
            (*tty).c_cc[5] = 0;

            tcsetattr(0, 0, tty as *const Termios);
        }
        atexit(term_exit_fn);
    }
}
}

// ── `fwrite(buf, 1, len, stdout ); fflush( stdout );` · temu.c:106-107 · chunk 3517 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fwrite(buf, 1, len, 
//                            stdout
//                                  );
//         fflush(
//               stdout
//                     );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3517 {
// Portable rewrite of the stdio flush/write block of console_write().
use core::ffi::{c_int, c_void};

extern "C" {
    static mut stdout: *mut c_void;
    fn fwrite(ptr: *const c_void, size: usize, n: usize, s: *mut c_void) -> usize;
    fn fflush(s: *mut c_void) -> c_int;
}

#[no_mangle]
pub extern "C" fn console_write__part3517(buf: *const u8, len: c_int) {
    unsafe {
        let f = stdout;
        fwrite(buf as *const c_void, 1usize, len as usize, f);
        fflush(f);
    }
}
}

// ── `if (len <= 0)` · temu.c:116-117 · chunk 3519 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len <= 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3519 {
// Portion of console_read: the early-out guard `if (len <= 0) return 0;`.
// Returns nonzero when the caller must return 0 (i.e. len <= 0).
#[no_mangle]
pub extern "C" fn console_read__part3519(len: core::ffi::c_int) -> core::ffi::c_int {
    if len <= 0 {
        1
    } else {
        0
    }
}
}

// ── `ret = read(s->stdin_fd, buf, len);` · temu.c:119-119 · chunk 3520 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     ret = read(s->stdin_fd, buf, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3520 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[no_mangle]
pub extern "C" fn console_read__part3520(stdin_fd: c_int, buf: *mut u8, len: c_int) -> c_int {
    let n: isize = unsafe { read(stdin_fd, buf as *mut c_void, len as usize) };
    n as c_int
}
}

// ── `if (ret < 0)` · temu.c:120-121 · chunk 3521 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret < 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3521 {
// Portion of console_read: test whether the preceding read() failed.
// Returns 1 when the C code must `return 0;`, 0 otherwise.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn console_read__part3521(ret: c_int) -> c_int {
    if ret < 0 {
        1
    } else {
        0
    }
}
}

// ── `if (ret == 0)` · temu.c:122-125 · chunk 3522 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret == 0) {
//     
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3522 {
// Port of the `if (ret == 0) { exit(1); }` piece of console_read.
use core::ffi::c_int;

extern "C" {
    // libc exit(), declared noreturn like its C prototype.
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn console_read__part3522(ret: c_int) {
    if ret == 0 {
        unsafe {
            exit(1);
        }
    }
}
}

// ── `j = 0;` · temu.c:127-127 · chunk 3523 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     j = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3523 {
// Sets the running output index to zero before the compaction loop.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn console_read__part3523(j: *mut c_int) {
    unsafe {
        *j = 0;
    }
}
}

// ── `for (i = 0; i < ret; i++)` · temu.c:128-156 · chunk 3524 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < ret; i++) {
//             ch = buf[i];
//             if (s->console_esc_state) {
//                 s->console_esc_state = 0;
//                 switch(ch) {
//                 case 'x':
//                     printf("Terminated\n");
//                     exit(0);
//                 case 'h':
//                     printf("\n"
//                            "C-a h   print this help\n"
//                            "C-a x   exit emulator\n"
//                            "C-a C-a send C-a\n"
//                            );
//                     break;
//                 case 1:
//                     goto output_char;
//                 default:
//                     break;
//                 }
//             } else {
//                 if (ch == 1) {
//                     s->console_esc_state = 1;
//                 } else {
//                 output_char:
//                     buf[j++] = ch;
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3524 {
// Rust port of the character-processing loop of console_read().
use core::ffi::{c_char, c_int, c_void};

// BOOL is a typedef for int in the C program.
#[repr(C)]
struct STDIODevice {
    stdin_fd: c_int,
    console_esc_state: c_int,
    resize_pending: c_int,
}

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

const MSG_TERMINATED: &[u8] = b"Terminated\n\0";
const MSG_HELP: &[u8] = b"\nC-a h   print this help\nC-a x   exit emulator\nC-a C-a send C-a\n\0";

#[no_mangle]
pub extern "C" fn console_read__part3524(opaque: *mut c_void, buf: *mut u8, ret: c_int) -> c_int {
    let s = opaque as *mut STDIODevice;
    let mut j: c_int = 0;
    let mut i: c_int = 0;

    while i < ret {
        let ch: u8 = unsafe { *buf.offset(i as isize) };
        let mut output_char: bool = false;

        if unsafe { (*s).console_esc_state } != 0 {
            unsafe {
                (*s).console_esc_state = 0;
            }
            match ch {
                0x78 /* 'x' */ => {
                    unsafe {
                        printf(MSG_TERMINATED.as_ptr() as *const c_char);
                        exit(0);
                    }
                }
                0x68 /* 'h' */ => {
                    unsafe {
                        printf(MSG_HELP.as_ptr() as *const c_char);
                    }
                }
                1 => {
                    output_char = true;
                }
                _ => {}
            }
        } else if ch == 1 {
            unsafe {
                (*s).console_esc_state = 1;
            }
        } else {
            output_char = true;
        }

        if output_char {
            unsafe {
                *buf.offset(j as isize) = ch;
            }
            j += 1;
        }

        i += 1;
    }

    j
}
}

// ── `return j;` · temu.c:157-157 · chunk 3525 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return j;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3525 {
// Ported piece of console_read: the final `return j;` of the static C function.
// The target simply yields the accumulated output length, so `j` is passed by
// value and returned unchanged.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn console_read__part3525(j: c_int) -> c_int {
    j
}
}

// ── `if (global_stdio_device)` · temu.c:162-163 · chunk 3527 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (global_stdio_device)
//             global_stdio_device->resize_pending = TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3527 {
// Port of the TARGET from term_resize_handler: set the resize_pending flag
// on the global stdio device when one exists. The C static global has no
// linker symbol, so the pointer is passed in by value.

use core::ffi::c_int;

// BOOL is `typedef int BOOL;` in the C translation unit.
#[repr(C)]
pub struct STDIODevice {
    pub stdin_fd: c_int,
    pub console_esc_state: c_int,
    pub resize_pending: c_int,
}

#[no_mangle]
pub extern "C" fn term_resize_handler__part3527(dev: *mut STDIODevice) {
    unsafe {
        if !dev.is_null() {
            (*dev).resize_pending = 1; // TRUE == 1
        }
    }
}
}

// ── `struct winsize ws; int width, height; width = 80; height = 25;` · temu.c:171-175 · chunk 3529 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct winsize ws;
//         int width, height;
//     
//         width = 80;
//         height = 25;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3529 {
// Ported piece of console_get_size: local declarations plus the default
// width/height initialization (80x25) performed before the TIOCGWINSZ probe.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn console_get_size__part3529(pw: *mut c_int, ph: *mut c_int) {
    unsafe {
        if !pw.is_null() {
            *pw = 80;
        }
        if !ph.is_null() {
            *ph = 25;
        }
    }
}
}

// ── `if (ioctl(s->stdin_fd, 0x5413 , &ws) == 0 && ws.ws_col >= 4 && ws.ws_row >= 4)` · temu.c:176-180 · chunk 3530 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ioctl(s->stdin_fd, 
//                               0x5413
//                                         , &ws) == 0 &&
//             ws.ws_col >= 4 && ws.ws_row >= 4) {
//             width = ws.ws_col;
//             height = ws.ws_row;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3530 {
// Portion of console_get_size: query the terminal size via TIOCGWINSZ.
use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

const TIOCGWINSZ: c_ulong = 0x5413;

#[no_mangle]
pub unsafe extern "C" fn console_get_size__part3530(
    stdin_fd: c_int,
    ws: *mut Winsize,
    width: *mut c_int,
    height: *mut c_int,
) {
    if ioctl(stdin_fd, TIOCGWINSZ, ws) == 0 {
        let ws_ref = &*ws;
        if ws_ref.ws_col >= 4 && ws_ref.ws_row >= 4 {
            *width = ws_ref.ws_col as c_int;
            *height = ws_ref.ws_row as c_int;
        }
    }
}
}

// ── `*pw = width; *ph = height;` · temu.c:181-182 · chunk 3531 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pw = width;
//         *ph = height;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3531 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn console_get_size__part3531(
    pw: *mut c_int,
    ph: *mut c_int,
    width: c_int,
    height: c_int,
) {
    unsafe {
        if !pw.is_null() {
            *pw = width;
        }
        if !ph.is_null() {
            *ph = height;
        }
    }
}
}

// ── `CharacterDevice *dev; STDIODevice *s; struct sigaction sig; term_init(allow_ctrlc); dev = mallo…` · temu.c:188-213 · chunk 3533 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     CharacterDevice *dev;
//         STDIODevice *s;
//         struct sigaction sig;
//     
//         term_init(allow_ctrlc);
//     
//         dev = mallocz(sizeof(*dev));
//         s = mallocz(sizeof(*s));
//         s->stdin_fd = 0;
//     
//     
//         fcntl(s->stdin_fd, 
//                           4
//                                  , 
//                                    04000
//                                              );
//     
//         s->resize_pending = TRUE;
//         global_stdio_device = s;
//     
//     
//         sig.
//            __sigaction_handler.sa_handler 
//                       = term_resize_handler;
//         sigemptyset(&sig.sa_mask);
//         sig.sa_flags = 0;
//         sigaction(
//                  28
//                          , &sig, 
//                                  ((void *)0)
//                                      );
//     
//         dev->opaque = s;
//         dev->write_data = console_write;
//         dev->read_data = console_read;
//         return dev;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3533 {
// Rust port of the body of console_init (temu.c) as a standalone exported fn.
use core::ffi::{c_int, c_ulong, c_void};

// BOOL is `typedef int BOOL`, TRUE == 1
const TRUE: c_int = 1;
// fcntl command F_SETFL == 4
const F_SETFL: c_int = 4;
// O_NONBLOCK == 04000
const O_NONBLOCK: c_int = 0o4000;
// SIGWINCH == 28
const SIGWINCH: c_int = 28;

#[repr(C)]
pub struct StdioDevice {
    pub stdin_fd: c_int,
    pub console_esc_state: c_int,
    pub resize_pending: c_int, // BOOL
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int)>,
}

// __sigset_t: struct { unsigned long __val[1024 / (8 * sizeof(unsigned long))]; }
#[repr(C)]
pub struct SigSet {
    pub __val: [c_ulong; 16],
}

#[repr(C)]
pub union SigActionHandler {
    pub sa_handler: Option<unsafe extern "C" fn(c_int)>,
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_void)>,
}

#[repr(C)]
pub struct SigAction {
    pub handler: SigActionHandler,
    pub sa_mask: SigSet,
    pub sa_flags: c_int,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn console_init__part3533(
    allow_ctrlc: c_int,
    term_init_fp: Option<unsafe extern "C" fn(c_int)>,
    fcntl_fp: Option<unsafe extern "C" fn(c_int, c_int, ...) -> c_int>,
    term_resize_handler_fp: Option<unsafe extern "C" fn(c_int)>,
    sigemptyset_fp: Option<unsafe extern "C" fn(*mut SigSet) -> c_int>,
    sigaction_fp: Option<unsafe extern "C" fn(c_int, *const SigAction, *mut SigAction) -> c_int>,
    global_stdio_device_ptr: *mut *mut StdioDevice,
    console_write_fp: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    console_read_fp: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int)>,
) -> *mut CharacterDevice {
    if let Some(f) = term_init_fp {
        f(allow_ctrlc);
    }

    let dev = mallocz(core::mem::size_of::<CharacterDevice>()) as *mut CharacterDevice;
    let s = mallocz(core::mem::size_of::<StdioDevice>()) as *mut StdioDevice;
    (*s).stdin_fd = 0;

    if let Some(f) = fcntl_fp {
        f((*s).stdin_fd, F_SETFL, O_NONBLOCK);
    }

    (*s).resize_pending = TRUE;
    *global_stdio_device_ptr = s;

    let mut sig: SigAction = core::mem::zeroed();
    sig.handler.sa_handler = term_resize_handler_fp;
    if let Some(f) = sigemptyset_fp {
        f(&mut sig.sa_mask);
    }
    sig.sa_flags = 0;
    if let Some(f) = sigaction_fp {
        f(SIGWINCH, &sig, core::ptr::null_mut());
    }

    (*dev).opaque = s as *mut c_void;
    (*dev).write_data = console_write_fp;
    (*dev).read_data = console_read_fp;
    dev
}
}

// ── `BlockDeviceFile *bf = bs->opaque; return bf->nb_sectors;` · temu.c:235-236 · chunk 3539 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     BlockDeviceFile *bf = bs->opaque;
//         return bf->nb_sectors;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3539 {
// Port of the TARGET inside bf_get_sector_count: read bs->opaque and
// return its nb_sectors field.

use core::ffi::{c_int, c_void};

type BlockDeviceCompletionFunc = unsafe extern "C" fn(*mut c_void, c_int);

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            Option<BlockDeviceCompletionFunc>,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut c_void,
    pub nb_sectors: i64,
    pub mode: c_int,
    pub sector_table: *mut *mut u8,
}

#[no_mangle]
pub extern "C" fn bf_get_sector_count__part3539(bs: *mut BlockDevice) -> i64 {
    if bs.is_null() {
        return 0;
    }
    unsafe {
        let bf = (*bs).opaque as *mut BlockDeviceFile;
        if bf.is_null() {
            return 0;
        }
        (*bf).nb_sectors
    }
}
}

// ── `if (!bf->f)` · temu.c:255-256 · chunk 3541 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!bf->f)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3541 {
// Port of the `if (!bf->f) return -1;` check inside bf_read_async.
use core::ffi::{c_int, c_void};

// Mirror of the C struct BlockDeviceFile (only pointer access is performed).
#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut c_void,
    pub nb_sectors: i64,
    pub mode: c_int,
    pub sector_table: *mut *mut u8,
}

// Returns -1 when bf->f is NULL (caller must then `return -1;`), 0 otherwise.
#[no_mangle]
pub extern "C" fn bf_read_async__part3541(bf: *mut BlockDeviceFile) -> c_int {
    unsafe {
        if (*bf).f.is_null() {
            return -1;
        }
    }
    0
}
}

// ── `if (bf->mode == BF_MODE_SNAPSHOT) … else` · temu.c:257-272 · chunk 3542 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bf->mode == BF_MODE_SNAPSHOT) {
//             int i;
//             for(i = 0; i < n; i++) {
//                 if (!bf->sector_table[sector_num]) {
//                     fseek(bf->f, sector_num * 512, 
//                                                           0
//                                                                   );
//                     fread(buf, 1, 512, bf->f);
//                 } else {
//                     memcpy(buf, bf->sector_table[sector_num], 512);
//                 }
//                 sector_num++;
//                 buf += 512;
//             }
//         } else {
//             fseek(bf->f, sector_num * 512, 
//                                                   0
//                                                           );
//             fread(buf, 1, n * 512, bf->f);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3542 {
// Ported piece of bf_read_async: the snapshot / direct fread body.
use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut c_void,
    pub nb_sectors: i64,
    pub mode: c_int,
    pub sector_table: *mut *mut u8,
}

extern "C" {
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn bf_read_async__part3542(
    bf: *mut BlockDeviceFile,
    mut sector_num: u64,
    mut buf: *mut u8,
    n: c_int,
) {
    let bf = &*bf;
    if bf.mode == 2 {
        // BF_MODE_SNAPSHOT
        let mut i: c_int = 0;
        while i < n {
            let entry: *mut u8 = *bf.sector_table.add(sector_num as usize);
            if entry.is_null() {
                fseek(bf.f, (sector_num.wrapping_mul(512)) as c_long, 0);
                fread(buf as *mut c_void, 1, 512, bf.f);
            } else {
                memcpy(buf as *mut c_void, entry as *const c_void, 512);
            }
            sector_num = sector_num.wrapping_add(1);
            buf = buf.add(512);
            i += 1;
        }
    } else {
        fseek(bf.f, (sector_num.wrapping_mul(512)) as c_long, 0);
        fread(
            buf as *mut c_void,
            1,
            n.wrapping_mul(512) as usize,
            bf.f,
        );
    }
}
}

// ── `return 0;` · temu.c:274-274 · chunk 3543 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3543 {
// Ported tail of bf_read_async: unconditionally reports success.
#[no_mangle]
pub extern "C" fn bf_read_async__part3543() -> core::ffi::c_int {
    0
}
}

// ── `switch (bf->mode)` · temu.c:284-311 · chunk 3545 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(bf->mode) {
//         case BF_MODE_RO:
//             ret = -1;
//             break;
//         case BF_MODE_RW:
//             fseek(bf->f, sector_num * 512, 
//                                                   0
//                                                           );
//             fwrite(buf, 1, n * 512, bf->f);
//             ret = 0;
//             break;
//         case BF_MODE_SNAPSHOT:
//             {
//                 int i;
//                 if ((sector_num + n) > bf->nb_sectors)
//                     return -1;
//                 for(i = 0; i < n; i++) {
//                     if (!bf->sector_table[sector_num]) {
//                         bf->sector_table[sector_num] = malloc(512);
//                     }
//                     memcpy(bf->sector_table[sector_num], buf, 512);
//                     sector_num++;
//                     buf += 512;
//                 }
//                 ret = 0;
//             }
//             break;
//         default:
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3545 {
// Port of the write-mode dispatch switch from bf_write_async (temu.c:284-311).
// Returns 0 to let the C caller continue, 1 when the C code must `return -1`.

use core::ffi::{c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct BlockDeviceFile {
    // FILE *f
    pub f: *mut c_void,
    pub nb_sectors: i64,
    pub mode: c_uint,
    pub sector_table: *mut *mut u8,
}

extern "C" {
    fn fseek(stream: *mut c_void, off: c_long, whence: c_int) -> c_int;
    fn fwrite(ptr: *const c_void, size: usize, n: usize, s: *mut c_void) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn abort() -> !;
}

const BF_MODE_RO: c_uint = 0;
const BF_MODE_RW: c_uint = 1;
const BF_MODE_SNAPSHOT: c_uint = 2;
const SECTOR_SIZE: usize = 512;

#[no_mangle]
pub unsafe extern "C" fn bf_write_async__part3545(
    bf: *mut BlockDeviceFile,
    mut sector_num: u64,
    mut buf: *const u8,
    n: c_int,
    ret: *mut c_int,
) -> c_int {
    if bf.is_null() {
        abort();
    }
    let mode = (*bf).mode;
    if mode == BF_MODE_RO {
        *ret = -1;
    } else if mode == BF_MODE_RW {
        // fseek(bf->f, sector_num * 512, SEEK_SET);
        fseek(
            (*bf).f,
            sector_num.wrapping_mul(512) as c_long,
            0 as c_int,
        );
        // fwrite(buf, 1, n * 512, bf->f);
        fwrite(
            buf as *const c_void,
            1,
            (n.wrapping_mul(512) as i64) as usize,
            (*bf).f,
        );
        *ret = 0;
    } else if mode == BF_MODE_SNAPSHOT {
        // if ((sector_num + n) > bf->nb_sectors) return -1;
        if sector_num.wrapping_add(n as i64 as u64) > (*bf).nb_sectors as u64 {
            return 1;
        }
        let mut i: c_int = 0;
        while i < n {
            let slot = (*bf).sector_table.wrapping_add(sector_num as usize);
            if (*slot).is_null() {
                *slot = malloc(SECTOR_SIZE) as *mut u8;
            }
            memcpy(
                *slot as *mut c_void,
                buf as *const c_void,
                SECTOR_SIZE,
            );
            sector_num = sector_num.wrapping_add(1);
            buf = buf.wrapping_add(SECTOR_SIZE);
            i += 1;
        }
        *ret = 0;
    } else {
        abort();
    }
    0
}
}

// ── `return ret;` · temu.c:313-313 · chunk 3546 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3546 {
// Returns the value the enclosing C switch produced (the final `return ret;`).
#[no_mangle]
pub extern "C" fn bf_write_async__part3546(ret: core::ffi::c_int) -> core::ffi::c_int {
    ret
}
}

// ── `if (mode == BF_MODE_RW) … else` · temu.c:325-329 · chunk 3548 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mode == BF_MODE_RW) {
//             mode_str = "r+b";
//         } else {
//             mode_str = "rb";
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3548 {
use core::ffi::{c_char, c_int};

// BF_MODE_RW is the second enumerator of BlockDeviceModeEnum.
const BF_MODE_RW: c_int = 1;

static MODE_STR_RW: [u8; 4] = *b"r+b\0";
static MODE_STR_RO: [u8; 3] = *b"rb\0";

#[no_mangle]
pub extern "C" fn block_device_init__part3548(mode: c_int) -> *const c_char {
    if mode == BF_MODE_RW {
        MODE_STR_RW.as_ptr() as *const c_char
    } else {
        MODE_STR_RO.as_ptr() as *const c_char
    }
}
}

// ── `f = fopen(filename, mode_str);` · temu.c:331-331 · chunk 3549 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     f = fopen(filename, mode_str);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3549 {
use core::ffi::{c_char, c_void};

extern "C" {
    fn fopen64(filename: *const c_char, modes: *const c_char) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn block_device_init__part3549(
    filename: *const c_char,
    mode_str: *const c_char,
) -> *mut c_void {
    fopen64(filename, mode_str)
}
}

// ── `if (!f)` · temu.c:332-335 · chunk 3550 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f) {
//             perror(filename);
//             exit(1);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3550 {
// Port of the fopen-failure check from block_device_init.
// Reads: filename, f. No locals written.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn block_device_init__part3550(filename: *const c_char, f: *mut c_void) {
    if f.is_null() {
        perror(filename);
        exit(1);
    }
}
}

// ── `fseek(f, 0, 2 ); file_size = ftello(f); bs = mallocz(sizeof(*bs)); bf = mallocz(sizeof(*bf)); b…` · temu.c:336-344 · chunk 3551 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fseek(f, 0, 
//                    2
//                            );
//         file_size = ftello(f);
//     
//         bs = mallocz(sizeof(*bs));
//         bf = mallocz(sizeof(*bf));
//     
//         bf->mode = mode;
//         bf->nb_sectors = file_size / 512;
//         bf->f = f;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3551 {
// Port of the middle piece of block_device_init (temu.c lines 336-344):
// seek to end of file, measure its size, allocate the BlockDevice and
// BlockDeviceFile headers and fill in the file-backed device fields.

use core::ffi::{c_char, c_int, c_long, c_void};

// FILE is an opaque libc type; only pointers to it are used here.
pub type FILE = c_void;

// Mirror of the C BlockDevice struct (only used for sizeof).
#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: *mut c_void,
    pub read_async: *mut c_void,
    pub write_async: *mut c_void,
    pub opaque: *mut c_void,
}

// Mirror of the C BlockDeviceFile struct, field for field, in order.
#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut FILE,
    pub nb_sectors: i64,
    pub mode: c_int,
    pub sector_table: *mut *mut u8,
}

extern "C" {
    fn fseek(stream: *mut FILE, off: c_long, whence: c_int) -> c_int;
    fn ftello(stream: *mut FILE) -> i64;
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn block_device_init__part3551(
    f: *mut FILE,
    mode: c_int,
    bs_out: *mut *mut BlockDevice,
    bf_out: *mut *mut BlockDeviceFile,
    file_size_out: *mut i64,
) {
    // fseek(f, 0, SEEK_END);  SEEK_END == 2
    fseek(f, 0, 2);
    let file_size: i64 = ftello(f);

    let bs: *mut BlockDevice = mallocz(core::mem::size_of::<BlockDevice>()) as *mut BlockDevice;
    let bf: *mut BlockDeviceFile =
        mallocz(core::mem::size_of::<BlockDeviceFile>()) as *mut BlockDeviceFile;

    (*bf).mode = mode;
    (*bf).nb_sectors = file_size / 512;
    (*bf).f = f;

    *bs_out = bs;
    *bf_out = bf;
    *file_size_out = file_size;
}
}

// ── `if (mode == BF_MODE_SNAPSHOT)` · temu.c:346-349 · chunk 3552 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (mode == BF_MODE_SNAPSHOT) {
//             bf->sector_table = mallocz(sizeof(bf->sector_table[0]) *
//                                        bf->nb_sectors);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3552 {
// Port of the snapshot sector-table allocation extracted from
// block_device_init() in temu.c.

use core::ffi::{c_int, c_void};

// Mirror of C's BlockDeviceFile (exact field order/types).
#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut c_void,          // FILE *f
    pub nb_sectors: i64,         // int64_t nb_sectors
    pub mode: c_int,             // BlockDeviceModeEnum mode
    pub sector_table: *mut *mut u8, // uint8_t **sector_table
}

#[allow(non_snake_case)]
extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

const BF_MODE_SNAPSHOT: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn block_device_init__part3552(
    bf: *mut BlockDeviceFile,
    mode: c_int,
) {
    if mode == BF_MODE_SNAPSHOT {
        if bf.is_null() {
            return;
        }
        let bf = &mut *bf;
        // sizeof(bf->sector_table[0]) == sizeof(uint8_t *), size_t arithmetic
        let elem_size = core::mem::size_of::<*mut u8>() as u64;
        let total = elem_size.wrapping_mul(bf.nb_sectors as u64);
        bf.sector_table = mallocz(total as usize) as *mut *mut u8;
    }
}
}

// ── `bs->opaque = bf; bs->get_sector_count = bf_get_sector_count; bs->read_async = bf_read_async; bs…` · temu.c:351-355 · chunk 3553 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     bs->opaque = bf;
//         bs->get_sector_count = bf_get_sector_count;
//         bs->read_async = bf_read_async;
//         bs->write_async = bf_write_async;
//         return bs;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3553 {
// Port of the tail of block_device_init: vtable wiring + returning the device.
use core::ffi::{c_int, c_void};

// typedef void BlockDeviceCompletionFunc(void *opaque, int ret);
type BlockDeviceCompletionFunc = Option<unsafe extern "C" fn(*mut c_void, c_int)>;

type GetSectorCountFn = Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>;
type ReadAsyncFn = Option<
    unsafe extern "C" fn(
        *mut BlockDevice,
        u64,
        *mut u8,
        c_int,
        BlockDeviceCompletionFunc,
        *mut c_void,
    ) -> c_int,
>;
type WriteAsyncFn = Option<
    unsafe extern "C" fn(
        *mut BlockDevice,
        u64,
        *const u8,
        c_int,
        BlockDeviceCompletionFunc,
        *mut c_void,
    ) -> c_int,
>;

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: GetSectorCountFn,
    pub read_async: ReadAsyncFn,
    pub write_async: WriteAsyncFn,
    pub opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn block_device_init__part3553(
    bs: *mut BlockDevice,
    bf: *mut c_void,
    get_sector_count: GetSectorCountFn,
    read_async: ReadAsyncFn,
    write_async: WriteAsyncFn,
) -> *mut BlockDevice {
    unsafe {
        (*bs).opaque = bf;
        (*bs).get_sector_count = get_sector_count;
        (*bs).read_async = read_async;
        (*bs).write_async = write_async;
    }
    bs
}
}

// ── `TunState *s = net->opaque; write(s->fd, buf, len);` · temu.c:368-369 · chunk 3557 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     TunState *s = net->opaque;
//         write(s->fd, buf, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3557 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            *mut EthernetDevice,
            *mut c_int,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(*mut EthernetDevice, *mut c_void, *mut c_void, *mut c_void, c_int),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice) -> c_int>,
    pub device_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(*mut EthernetDevice, c_int)>,
}

#[repr(C)]
pub struct TunState {
    pub fd: c_int,
    pub select_filled: c_int,
}

extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn tun_write_packet__part3557(
    net: *mut EthernetDevice,
    buf: *const u8,
    len: c_int,
) {
    let s = (*net).opaque as *mut TunState;
    write((*s).fd, buf as *const c_void, len as usize);
}
}

// ── `TunState *s = net->opaque; int net_fd = s->fd; s->select_filled = net->device_can_write_packet(…` · temu.c:376-379 · chunk 3559 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     TunState *s = net->opaque;
//         int net_fd = s->fd;
//     
//         s->select_filled = net->device_can_write_packet(net);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3559 {
// Port of the prologue of tun_select_fill: fetch TunState and its fd,
// query device_can_write_packet and store it into s->select_filled.
use core::ffi::{c_int, c_void};

type BOOL = c_int;

#[repr(C)]
pub struct TunState {
    pub fd: c_int,
    pub select_filled: BOOL,
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: *mut c_void,
    pub opaque: *mut c_void,
    pub select_fill: *mut c_void,
    pub select_poll: *mut c_void,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice) -> BOOL>,
    pub device_write_packet: *mut c_void,
    pub device_set_carrier: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn tun_select_fill__part3559(
    net: *mut EthernetDevice,
    pnet_fd: *mut c_int,
) -> *mut TunState {
    unsafe {
        let s = (*net).opaque as *mut TunState;
        let net_fd = (*s).fd;
        *pnet_fd = net_fd;

        let filled = match (*net).device_can_write_packet {
            Some(f) => f(net),
            None => 0,
        };
        (*s).select_filled = filled;

        s
    }
}
}

// ── `if (s->select_filled)` · temu.c:380-383 · chunk 3560 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->select_filled) {
//             
//            ((void) (((
//            rfds
//            )->fds_bits)[((
//            net_fd
//            ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//            net_fd
//            ) % (8 * (int) sizeof (__fd_mask)))))))
//                                ;
//             *pfd_max = max_int(*pfd_max, net_fd);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3560 {
// Piece of tun_select_fill: set the tun fd bit in rfds and raise *pfd_max.

use core::ffi::{c_int, c_long, c_ulong};

const FD_MASK_BITS: c_int = 8 * core::mem::size_of::<c_long>() as c_int;

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

// static inline int max_int(int a, int b) -- ported privately (no linker symbol).
fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub extern "C" fn tun_select_fill__part3560(
    net_fd: c_int,
    rfds: *mut FdSet,
    pfd_max: *mut c_int,
) {
    unsafe {
        let idx = (net_fd / FD_MASK_BITS) as isize;
        let shift = (net_fd % FD_MASK_BITS) as u32;
        let mask = (1 as c_ulong).wrapping_shl(shift) as c_long;
        let bits = core::ptr::addr_of_mut!((*rfds).fds_bits) as *mut c_long;
        let slot = bits.offset(idx);
        *slot |= mask;
        *pfd_max = max_int(*pfd_max, net_fd);
    }
}
}

// ── `if (select_ret <= 0)` · temu.c:395-396 · chunk 3562 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (select_ret <= 0)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3562 {
use core::ffi::c_int;

// Ported piece of tun_select_poll: the early-out guard.
// Returns 1 when the caller must return (select_ret <= 0), 0 otherwise.
#[no_mangle]
pub extern "C" fn tun_select_poll__part3562(select_ret: c_int) -> c_int {
    if select_ret <= 0 {
        1
    } else {
        0
    }
}
}

// ── `if (s->select_filled && (((( rfds )->fds_bits)[(( net_fd ) / (8 * (int) sizeof (__fd_mask)))] &…` · temu.c:397-401 · chunk 3563 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->select_filled && 
//                                ((((
//                                rfds
//                                )->fds_bits)[((
//                                net_fd
//                                ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                                net_fd
//                                ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                                      ) {
//             ret = read(net_fd, buf, sizeof(buf));
//             if (ret > 0)
//                 net->device_write_packet(net, buf, ret);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3563 {
// Port of the tun_select_poll read/forward block.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_long, c_void};

type BOOL = c_int;

const FDS_BITS_LEN: usize = 1024 / (8 * core::mem::size_of::<c_long>());

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [c_long; FDS_BITS_LEN],
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            *mut EthernetDevice,
            *mut c_int,
            *mut fd_set,
            *mut fd_set,
            *mut fd_set,
            *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(*mut EthernetDevice, *mut fd_set, *mut fd_set, *mut fd_set, c_int),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice) -> BOOL>,
    pub device_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(*mut EthernetDevice, BOOL)>,
}

extern "C" {
    fn read(fd: c_int, buf: *mut c_void, n: usize) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn tun_select_poll__part3563(
    net: *mut EthernetDevice,
    rfds: *mut fd_set,
    select_filled: c_int,
    net_fd: c_int,
    buf: *mut u8,
    buf_len: usize,
    ret_out: *mut c_int,
) {
    if select_filled == 0 {
        return;
    }
    let bits_per_word: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
    let idx: c_int = net_fd / bits_per_word;
    let mask: c_long = 1u64.wrapping_shl((net_fd % bits_per_word) as u32) as c_long;

    let base: *const c_long = core::ptr::addr_of!((*rfds).fds_bits) as *const c_long;
    let word: c_long = *base.offset(idx as isize);

    if (word & mask) != 0 {
        let ret = read(net_fd, buf as *mut c_void, buf_len);
        let ret_i = ret as c_int;
        if !ret_out.is_null() {
            *ret_out = ret_i;
        }
        if ret_i > 0 {
            let cb = (*net).device_write_packet;
            if let Some(f) = cb {
                f(net, buf as *const u8, ret_i);
            }
        }
    }
}
}

// ── `struct ifreq ifr; int fd, ret; EthernetDevice *net; TunState *s; fd = open("/dev/net/tun", 02 );` · temu.c:425-430 · chunk 3565 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct ifreq ifr;
//         int fd, ret;
//         EthernetDevice *net;
//         TunState *s;
//     
//         fd = open("/dev/net/tun", 
//                                  02
//                                        );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3565 {
use core::ffi::{c_char, c_int};

extern "C" {
    // glibc declares open() with __asm__("open64")
    #[link_name = "open64"]
    fn open64(__file: *const c_char, __oflag: c_int, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn tun_open__part3565() -> c_int {
    let path: &[u8] = b"/dev/net/tun\0";
    unsafe { open64(path.as_ptr() as *const c_char, 02) }
}
}

// ── `if (fd < 0)` · temu.c:431-434 · chunk 3566 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (fd < 0) {
//             fprintf(
//                    stderr
//                          , "Error: could not open /dev/net/tun\n");
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3566 {
// Port of the fd < 0 error branch of tun_open (temu.c lines 431-434).
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

// Returns non-zero when the open() failed (message already printed and the
// caller must return NULL), zero when fd is usable.
#[no_mangle]
pub extern "C" fn tun_open__part3566(fd: c_int) -> c_int {
    if fd < 0 {
        unsafe {
            fprintf(
                stderr,
                b"Error: could not open /dev/net/tun\n\0".as_ptr() as *const c_char,
            );
        }
        return 1;
    }
    0
}
}

// ── `memset(&ifr, 0, sizeof(ifr)); ifr. ifr_ifru.ifru_flags = 0x0002 | 0x1000 ; pstrcpy(ifr. ifr_ifr…` · temu.c:435-438 · chunk 3567 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(&ifr, 0, sizeof(ifr));
//         ifr.
//            ifr_ifru.ifru_flags 
//                      = 
//                        0x0002 
//                                | 
//                                  0x1000
//                                           ;
//         pstrcpy(ifr.
//                    ifr_ifrn.ifrn_name
//                            , sizeof(ifr.
//                                         ifr_ifrn.ifrn_name
//                                                 ), ifname);
//         ret = ioctl(fd, 
//                        (((1U) << (((0 +8)+8)+14)) | ((('T')) << (0 +8)) | (((202)) << 0) | ((((sizeof(int)))) << ((0 +8)+8)))
//                                 , (void *) &ifr);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3567 {
// Port of the tun_open() setup piece: zero the ifreq, set the TUN flags,
// copy the interface name and issue TUNSETIFF.

use core::ffi::{c_char, c_int, c_short, c_ulong, c_void};
use core::ptr::addr_of_mut;

// Mirror of the parts of `struct ifreq` that are touched, with the same
// field offsets (ifrn_name at 0, ifru_flags at 16).
#[repr(C)]
struct TunIfreq {
    ifrn_name: [c_char; 16],
    ifru_flags: c_short,
    _pad: [u8; 14],
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn pstrcpy(buf: *mut c_char, buf_size: c_int, str_: *const c_char);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[no_mangle]
pub extern "C" fn tun_open__part3567(
    fd: c_int,
    ifname: *const c_char,
    ifr: *mut c_void,
    ifr_size: c_ulong,
    name_size: c_int,
    request: c_ulong,
) -> c_int {
    let p = ifr as *mut TunIfreq;
    unsafe {
        memset(ifr, 0, ifr_size as usize);
        (*p).ifru_flags = 0x1002i32 as c_short;
        let name = addr_of_mut!((*p).ifrn_name) as *mut c_char;
        pstrcpy(name, name_size, ifname);
        ioctl(fd, request, ifr)
    }
}
}

// ── `if (ret != 0)` · temu.c:439-443 · chunk 3568 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret != 0) {
//             fprintf(
//                    stderr
//                          , "Error: could not configure /dev/net/tun\n");
//             close(fd);
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3568 {
// Port of the ioctl-failure branch of tun_open (temu.c lines 439-443).
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn tun_open__part3568(fd: c_int, ret: c_int) -> c_int {
    if ret != 0 {
        unsafe {
            fprintf(
                stderr,
                b"Error: could not configure /dev/net/tun\n\0".as_ptr() as *const c_char,
            );
            close(fd);
        }
        return 1;
    }
    0
}
}

// ── `fcntl(fd, 4 , 04000 ); net = mallocz(sizeof(*net)); net->mac_addr[0] = 0x02; net->mac_addr[1] =…` · temu.c:444-459 · chunk 3569 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fcntl(fd, 
//                  4
//                         , 
//                           04000
//                                     );
//     
//         net = mallocz(sizeof(*net));
//         net->mac_addr[0] = 0x02;
//         net->mac_addr[1] = 0x00;
//         net->mac_addr[2] = 0x00;
//         net->mac_addr[3] = 0x00;
//         net->mac_addr[4] = 0x00;
//         net->mac_addr[5] = 0x01;
//         s = mallocz(sizeof(*s));
//         s->fd = fd;
//         net->opaque = s;
//         net->write_packet = tun_write_packet;
//         net->select_fill = tun_select_fill;
//         net->select_poll = tun_select_poll;
//         return net;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3569 {
// Port of the tail of tun_open(): set O_NONBLOCK on the tun fd, allocate the
// EthernetDevice and TunState, wire up the callbacks and return the device.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            pfd_max: *mut c_int,
            rfds: *mut fd_set,
            wfds: *mut fd_set,
            efds: *mut fd_set,
            pdelay: *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            rfds: *mut fd_set,
            wfds: *mut fd_set,
            efds: *mut fd_set,
            select_ret: c_int,
        ),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice) -> c_int>,
    pub device_write_packet:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(net: *mut EthernetDevice, carrier_state: c_int)>,
}

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [i64; 1024 / (8 * 8)],
}

#[repr(C)]
pub struct TunState {
    pub fd: c_int,
    pub select_filled: c_int,
}

extern "C" {
    // In the preprocessed TU `fcntl` is declared with __asm__("fcntl64").
    fn fcntl64(fd: c_int, cmd: c_int, ...) -> c_int;
    fn mallocz(size: usize) -> *mut c_void;
    fn read(fd: c_int, buf: *mut c_void, nbytes: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, n: usize) -> isize;
}

const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o4000;

static mut MAX_INT_DUMMY: c_int = 0;

unsafe fn max_int(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

// The three callbacks are static in the C translation unit, so their logic is
// replicated here as private Rust functions.

unsafe extern "C" fn tun_write_packet(net: *mut EthernetDevice, buf: *const u8, len: c_int) {
    let s = (*net).opaque as *mut TunState;
    write((*s).fd, buf as *const c_void, len as usize);
}

unsafe extern "C" fn tun_select_fill(
    net: *mut EthernetDevice,
    pfd_max: *mut c_int,
    rfds: *mut fd_set,
    _wfds: *mut fd_set,
    _efds: *mut fd_set,
    _pdelay: *mut c_int,
) {
    let s = (*net).opaque as *mut TunState;
    let net_fd = (*s).fd;

    let can_write = (*net).device_can_write_packet;
    (*s).select_filled = match can_write {
        Some(f) => f(net),
        None => 0,
    };
    if (*s).select_filled != 0 {
        // FD_SET(rfds, net_fd)
        let idx = net_fd / (8 * core::mem::size_of::<i64>() as c_int);
        let bit = 1i64.wrapping_shl((net_fd % (8 * core::mem::size_of::<i64>() as c_int)) as u32);
        let bits = &mut (*rfds).fds_bits;
        if idx >= 0 && (idx as usize) < bits.len() {
            bits[idx as usize] |= bit;
        }
        *pfd_max = max_int(*pfd_max, net_fd);
    }
}

unsafe extern "C" fn tun_select_poll(
    net: *mut EthernetDevice,
    rfds: *mut fd_set,
    _wfds: *mut fd_set,
    _efds: *mut fd_set,
    select_ret: c_int,
) {
    let s = (*net).opaque as *mut TunState;
    let net_fd = (*s).fd;
    let mut buf = [0u8; 2048];

    if select_ret <= 0 {
        return;
    }
    let idx = net_fd / (8 * core::mem::size_of::<i64>() as c_int);
    let bit = 1i64.wrapping_shl((net_fd % (8 * core::mem::size_of::<i64>() as c_int)) as u32);
    let bits = &(*rfds).fds_bits;
    let set = idx >= 0 && (idx as usize) < bits.len() && (bits[idx as usize] & bit) != 0;
    if (*s).select_filled != 0 && set {
        let ret = read(net_fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if ret > 0 {
            let wp = (*net).device_write_packet;
            if let Some(f) = wp {
                f(net, buf.as_ptr(), ret as c_int);
            }
        }
    }
    let _ = &mut MAX_INT_DUMMY;
}

#[no_mangle]
pub unsafe extern "C" fn tun_open__part3569(fd: c_int) -> *mut EthernetDevice {
    fcntl64(fd, F_SETFL, O_NONBLOCK);

    let net = mallocz(core::mem::size_of::<EthernetDevice>()) as *mut EthernetDevice;
    (*net).mac_addr = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
    let s = mallocz(core::mem::size_of::<TunState>()) as *mut TunState;
    (*s).fd = fd;
    (*net).opaque = s as *mut c_void;
    (*net).write_packet = Some(tun_write_packet);
    (*net).select_fill = Some(tun_select_fill);
    (*net).select_poll = Some(tun_select_poll);
    net
}
}

// ── `Slirp *slirp_state = net->opaque; slirp_input(slirp_state, buf, len);` · temu.c:474-475 · chunk 3572 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp_state = net->opaque;
//         slirp_input(slirp_state, buf, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3572 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            *mut EthernetDevice,
            *mut c_int,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(*mut EthernetDevice, *mut c_void, *mut c_void, *mut c_void, c_int),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice) -> c_int>,
    pub device_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(*mut EthernetDevice, c_int)>,
}

extern "C" {
    fn slirp_input(slirp: *mut c_void, pkt: *const u8, pkt_len: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn slirp_write_packet__part3572(
    net: *mut EthernetDevice,
    buf: *const u8,
    len: c_int,
) {
    let slirp_state = (*net).opaque;
    slirp_input(slirp_state, buf, len);
}
}

// ── `EthernetDevice *net = opaque; return net->device_can_write_packet(net);` · temu.c:480-481 · chunk 3574 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     EthernetDevice *net = opaque;
//         return net->device_can_write_packet(net);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3574 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<unsafe extern "C" fn(
        net: *mut EthernetDevice,
        pfd_max: *mut c_int,
        rfds: *mut c_void,
        wfds: *mut c_void,
        efds: *mut c_void,
        pdelay: *mut c_int,
    )>,
    pub select_poll: Option<unsafe extern "C" fn(
        net: *mut EthernetDevice,
        rfds: *mut c_void,
        wfds: *mut c_void,
        efds: *mut c_void,
        select_ret: c_int,
    )>,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice) -> c_int>,
    pub device_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(net: *mut EthernetDevice, carrier_state: c_int)>,
}

#[no_mangle]
pub unsafe extern "C" fn slirp_can_output__part3574(opaque: *mut c_void) -> c_int {
    let net = opaque as *mut EthernetDevice;
    match (*net).device_can_write_packet {
        Some(f) => f(net),
        None => 0,
    }
}
}

// ── `EthernetDevice *net = opaque; return net->device_write_packet(net, pkt, pkt_len);` · temu.c:486-487 · chunk 3576 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     EthernetDevice *net = opaque;
//         return net->device_write_packet(net, pkt, pkt_len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3576 {
// Port of the body of slirp_output: dispatch the packet through the
// EthernetDevice's device_write_packet callback.
use core::ffi::{c_char, c_int, c_long, c_void};

// Mirror of the C `fd_set` (glibc x86_64 layout) referenced by the
// EthernetDevice callback fields.
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * 8)],
}

pub type BOOL = c_int;

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            pfd_max: *mut c_int,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            pdelay: *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            select_ret: c_int,
        ),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice) -> BOOL>,
    pub device_write_packet:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, carrier_state: BOOL)>,
}

#[no_mangle]
pub unsafe extern "C" fn slirp_output__part3576(
    opaque: *mut c_void,
    pkt: *const u8,
    pkt_len: c_int,
) {
    let net = opaque as *mut EthernetDevice;
    if net.is_null() {
        return;
    }
    let cb = (*net).device_write_packet;
    if let Some(f) = cb {
        f(net, pkt, pkt_len);
    }
}

// Keep the c_char import meaningful for ABI documentation purposes.
#[allow(dead_code)]
type _CharAlias = c_char;
}

// ── `Slirp *slirp_state = net->opaque; slirp_select_fill(slirp_state, pfd_max, rfds, wfds, efds);` · temu.c:494-495 · chunk 3578 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp_state = net->opaque;
//         slirp_select_fill(slirp_state, pfd_max, rfds, wfds, efds);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3578 {
use core::ffi::{c_int, c_long, c_void};

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16],
}

// Mirrors `struct EthernetDevice` from temu.h (only the layout matters here).
#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: *mut c_void,
    pub opaque: *mut c_void,
    pub select_fill: *mut c_void,
    pub select_poll: *mut c_void,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: *mut c_void,
    pub device_write_packet: *mut c_void,
    pub device_set_carrier: *mut c_void,
}

extern "C" {
    fn slirp_select_fill(
        slirp: *mut c_void,
        pnfds: *mut c_int,
        readfds: *mut FdSet,
        writefds: *mut FdSet,
        xfds: *mut FdSet,
    );
}

#[no_mangle]
pub extern "C" fn slirp_select_fill1__part3578(
    net: *mut EthernetDevice,
    pfd_max: *mut c_int,
    rfds: *mut FdSet,
    wfds: *mut FdSet,
    efds: *mut FdSet,
) {
    unsafe {
        let slirp_state: *mut c_void = (*net).opaque;
        slirp_select_fill(slirp_state, pfd_max, rfds, wfds, efds);
    }
}
}

// ── `Slirp *slirp_state = net->opaque; slirp_select_poll(slirp_state, rfds, wfds, efds, (select_ret…` · temu.c:502-503 · chunk 3580 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp_state = net->opaque;
//         slirp_select_poll(slirp_state, rfds, wfds, efds, (select_ret <= 0));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3580 {
// Port of the head of slirp_select_poll1 from temu.c.
use core::ffi::{c_int, c_long};

#[repr(C)]
pub struct fd_set {
    pub fds_bits: [c_long; 1024 / (8 * 8)],
}

#[repr(C)]
pub struct Slirp {
    _private: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
    pub select_fill: *mut core::ffi::c_void,
    pub select_poll: *mut core::ffi::c_void,
    pub device_opaque: *mut core::ffi::c_void,
    pub device_can_write_packet: *mut core::ffi::c_void,
    pub device_write_packet: *mut core::ffi::c_void,
    pub device_set_carrier: *mut core::ffi::c_void,
}

extern "C" {
    fn slirp_select_poll(
        slirp: *mut Slirp,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        xfds: *mut fd_set,
        select_error: c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn slirp_select_poll1__part3580(
    net: *mut EthernetDevice,
    rfds: *mut fd_set,
    wfds: *mut fd_set,
    efds: *mut fd_set,
    select_ret: c_int,
) {
    let slirp_state: *mut Slirp = (*net).opaque as *mut Slirp;
    slirp_select_poll(
        slirp_state,
        rfds,
        wfds,
        efds,
        if select_ret <= 0 { 1 } else { 0 },
    );
}
}

// ── `if (slirp_state)` · temu.c:518-521 · chunk 3582 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (slirp_state) {
//             fprintf(
//                    stderr
//                          , "Only a single slirp instance is allowed\n");
//             return 
//                   ((void *)0)
//                       ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3582 {
// Ported piece of slirp_open: single-instance guard + error message.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    // FILE *stderr;  (FILE is opaque here, so use void* which is ABI-identical)
    static mut stderr: *mut c_void;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

/// Returns nonzero when a slirp instance already exists (caller must return NULL).
#[no_mangle]
pub extern "C" fn slirp_open__part3582(slirp_state: *mut c_void) -> c_int {
    if !slirp_state.is_null() {
        unsafe {
            fprintf(
                stderr,
                b"Only a single slirp instance is allowed\n\0".as_ptr() as *const c_char,
            );
        }
        return 1;
    }
    0
}
}

// ── `net = mallocz(sizeof(*net)); slirp_state = slirp_init(restricted, net_addr, mask, host, vhostna…` · temu.c:522-538 · chunk 3583 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     net = mallocz(sizeof(*net));
//     
//         slirp_state = slirp_init(restricted, net_addr, mask, host, vhostname,
//                                  "", bootfile, dhcp, dns, net);
//     
//         net->mac_addr[0] = 0x02;
//         net->mac_addr[1] = 0x00;
//         net->mac_addr[2] = 0x00;
//         net->mac_addr[3] = 0x00;
//         net->mac_addr[4] = 0x00;
//         net->mac_addr[5] = 0x01;
//         net->opaque = slirp_state;
//         net->write_packet = slirp_write_packet;
//         net->select_fill = slirp_select_fill1;
//         net->select_poll = slirp_select_poll1;
//     
//         return net;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3583 {
// Port of the body of slirp_open (temu.c lines 522-538).
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        extern "C" fn(
            net: *mut EthernetDevice,
            pfd_max: *mut c_int,
            rfds: *mut c_void,
            wfds: *mut c_void,
            efds: *mut c_void,
            pdelay: *mut c_int,
        ),
    >,
    pub select_poll: Option<
        extern "C" fn(
            net: *mut EthernetDevice,
            rfds: *mut c_void,
            wfds: *mut c_void,
            efds: *mut c_void,
            select_ret: c_int,
        ),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<extern "C" fn(net: *mut EthernetDevice) -> c_int>,
    pub device_write_packet:
        Option<extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier: Option<extern "C" fn(net: *mut EthernetDevice, carrier_state: c_int)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn slirp_init(
        restricted: c_int,
        vnetwork: InAddr,
        vnetmask: InAddr,
        vhost: InAddr,
        vhostname: *const c_char,
        tftp_path: *const c_char,
        bootfile: *const c_char,
        vdhcp_start: InAddr,
        vnameserver: InAddr,
        opaque: *mut c_void,
    ) -> *mut c_void;
    fn slirp_input(slirp: *mut c_void, pkt: *const u8, pkt_len: c_int);
    fn slirp_select_fill(
        slirp: *mut c_void,
        pnfds: *mut c_int,
        readfds: *mut c_void,
        writefds: *mut c_void,
        xfds: *mut c_void,
    );
    fn slirp_select_poll(
        slirp: *mut c_void,
        readfds: *mut c_void,
        writefds: *mut c_void,
        xfds: *mut c_void,
        select_error: c_int,
    );
}

// The C statics slirp_write_packet / slirp_select_fill1 / slirp_select_poll1
// have no linker symbols and are not reachable from here, so their (trivial)
// bodies are reproduced below.
extern "C" fn rs_slirp_write_packet(net: *mut EthernetDevice, buf: *const u8, len: c_int) {
    unsafe {
        let slirp_state = (*net).opaque;
        slirp_input(slirp_state, buf, len);
    }
}

extern "C" fn rs_slirp_select_fill1(
    net: *mut EthernetDevice,
    pfd_max: *mut c_int,
    rfds: *mut c_void,
    wfds: *mut c_void,
    efds: *mut c_void,
    _pdelay: *mut c_int,
) {
    unsafe {
        let slirp_state = (*net).opaque;
        slirp_select_fill(slirp_state, pfd_max, rfds, wfds, efds);
    }
}

extern "C" fn rs_slirp_select_poll1(
    net: *mut EthernetDevice,
    rfds: *mut c_void,
    wfds: *mut c_void,
    efds: *mut c_void,
    select_ret: c_int,
) {
    unsafe {
        let slirp_state = (*net).opaque;
        slirp_select_poll(
            slirp_state,
            rfds,
            wfds,
            efds,
            if select_ret <= 0 { 1 } else { 0 },
        );
    }
}

// The "" literal passed as tftp_path: a single NUL byte in read-only data.
static TFTP_PATH: [u8; 1] = [0];

#[no_mangle]
pub extern "C" fn slirp_open__part3583(
    slirp_state_ptr: *mut *mut c_void,
    restricted: c_int,
    net_addr: InAddr,
    mask: InAddr,
    host: InAddr,
    dhcp: InAddr,
    dns: InAddr,
    vhostname: *const c_char,
    bootfile: *const c_char,
) -> *mut EthernetDevice {
    unsafe {
        let net = mallocz(core::mem::size_of::<EthernetDevice>() as usize) as *mut EthernetDevice;

        let state = slirp_init(
            restricted,
            net_addr,
            mask,
            host,
            vhostname,
            TFTP_PATH.as_ptr() as *const c_char,
            bootfile,
            dhcp,
            dns,
            net as *mut c_void,
        );
        *slirp_state_ptr = state;

        (*net).mac_addr[0] = 0x02;
        (*net).mac_addr[1] = 0x00;
        (*net).mac_addr[2] = 0x00;
        (*net).mac_addr[3] = 0x00;
        (*net).mac_addr[4] = 0x00;
        (*net).mac_addr[5] = 0x01;
        (*net).opaque = state;
        (*net).write_packet = Some(rs_slirp_write_packet);
        (*net).select_fill = Some(rs_slirp_select_fill1);
        (*net).select_poll = Some(rs_slirp_select_poll1);

        net
    }
}
}

// ── `fd_set rfds, wfds, efds; int fd_max, ret, delay; struct timeval tv; int stdin_fd; delay = virt_…` · temu.c:548-555 · chunk 3585 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fd_set rfds, wfds, efds;
//         int fd_max, ret, delay;
//         struct timeval tv;
//     
//         int stdin_fd;
//     
//     
//         delay = virt_machine_get_sleep_duration(m, 10);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3585 {
// Port of the delay computation in virt_machine_run (temu.c).
// virt_machine_get_sleep_duration is a `static inline` C function with no
// linker symbol, so its logic is replicated here as a private Rust fn.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
struct VirtMachine {
    vmc: *const VirtMachineClass,
    net: *mut c_void,
    console_dev: *mut c_void,
    console: *mut c_void,
    fb_dev: *mut c_void,
}

// Only the leading pointer-sized members are needed; the trailing function
// pointers all have pointer size, so the layout up to
// `virt_machine_get_sleep_duration` is exactly the C layout.
#[repr(C)]
struct VirtMachineClass {
    machine_names: *const c_char,
    virt_machine_set_defaults: Option<unsafe extern "C" fn()>,
    virt_machine_init: Option<unsafe extern "C" fn() -> *mut VirtMachine>,
    virt_machine_end: Option<unsafe extern "C" fn()>,
    virt_machine_get_sleep_duration: Option<unsafe extern "C" fn(*mut VirtMachine, c_int) -> c_int>,
}

unsafe fn virt_machine_get_sleep_duration(s: *mut VirtMachine, delay: c_int) -> c_int {
    let vmc = (*s).vmc;
    match (*vmc).virt_machine_get_sleep_duration {
        Some(f) => f(s, delay),
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3585(m: *mut VirtMachine) -> c_int {
    unsafe { virt_machine_get_sleep_duration(m, 10) }
}
}

// ── `do … while (0)` · temu.c:558-558 · chunk 3586 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do { unsigned int __i; fd_set *__arr = (
//        &rfds
//        ); for (__i = 0; __i < sizeof (fd_set) / sizeof (__fd_mask); ++__i) ((__arr)->fds_bits)[__i] = 0; } while (0)
//                      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3586 {
// Zero-fill an fd_set, mirroring the C FD_ZERO loop:
//   do { unsigned int __i; fd_set *__arr = (&set);
//        for (__i = 0; __i < sizeof(fd_set)/sizeof(__fd_mask); ++__i)
//            ((__arr)->fds_bits)[__i] = 0;
//   } while (0)
use core::ffi::c_long;

// __fd_mask is `long int`; fds_bits has 1024 / (8 * sizeof(__fd_mask)) = 16 elements.
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3586(set: *mut FdSet) {
    if set.is_null() {
        return;
    }
    let count: usize = core::mem::size_of::<FdSet>() / core::mem::size_of::<c_long>();
    unsafe {
        let base = core::ptr::addr_of_mut!((*set).fds_bits) as *mut c_long;
        let mut i: usize = 0;
        while i < count {
            core::ptr::write(base.add(i), 0 as c_long);
            i += 1;
        }
    }
}
}

// ── `do … while (0)` · temu.c:559-559 · chunk 3587 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do { unsigned int __i; fd_set *__arr = (
//        &wfds
//        ); for (__i = 0; __i < sizeof (fd_set) / sizeof (__fd_mask); ++__i) ((__arr)->fds_bits)[__i] = 0; } while (0)
//                      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3587 {
// Zeroes an fd_set (the FD_ZERO equivalent macro expansion on `wfds`).
// struct fd_set { __fd_mask fds_bits[1024 / (8 * (int) sizeof(__fd_mask))]; }
// with __fd_mask == long int (8 bytes) => 16 elements.

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [i64; 16],
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3587(arr: *mut FdSet) {
    if arr.is_null() {
        return;
    }
    unsafe {
        let bits = core::ptr::addr_of_mut!((*arr).fds_bits) as *mut i64;
        let mut i: u32 = 0;
        while (i as usize) < 16 {
            core::ptr::write(bits.add(i as usize), 0 as i64);
            i = i.wrapping_add(1);
        }
    }
}
}

// ── `do … while (0)` · temu.c:560-560 · chunk 3588 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     do { unsigned int __i; fd_set *__arr = (
//        &efds
//        ); for (__i = 0; __i < sizeof (fd_set) / sizeof (__fd_mask); ++__i) ((__arr)->fds_bits)[__i] = 0; } while (0)
//                      ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3588 {
use core::ffi::{c_long, c_uint};

// Mirror of glibc fd_set: 1024 / (8 * sizeof(long)) == 16 longs.
const FD_SET_LONGS: usize = 1024 / (8 * core::mem::size_of::<c_long>());

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; FD_SET_LONGS],
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3588(arr: *mut FdSet) {
    let mut i: c_uint = 0;
    unsafe {
        if arr.is_null() {
            return;
        }
        while (i as usize) < FD_SET_LONGS {
            let slot = core::ptr::addr_of_mut!((*arr).fds_bits).cast::<c_long>();
            core::ptr::write(slot.add(i as usize), 0);
            i += 1;
        }
    }
}
}

// ── `fd_max = -1;` · temu.c:561-561 · chunk 3589 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fd_max = -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3589 {
// Port of the `fd_max = -1;` statement inside virt_machine_run (temu.c).
// The statement only writes the enclosing function's local `fd_max`, so the
// variable is passed by pointer and set to -1.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virt_machine_run__part3589(fd_max: *mut c_int) {
    if !fd_max.is_null() {
        unsafe {
            *fd_max = -1;
        }
    }
}
}

// ── `if (m->console_dev && virtio_console_can_write_data(m->console_dev))` · temu.c:563-575 · chunk 3590 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->console_dev && virtio_console_can_write_data(m->console_dev)) {
//             STDIODevice *s = m->console->opaque;
//             stdin_fd = s->stdin_fd;
//             
//            ((void) (((
//            &rfds
//            )->fds_bits)[((
//            stdin_fd
//            ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//            stdin_fd
//            ) % (8 * (int) sizeof (__fd_mask)))))))
//                                   ;
//             fd_max = stdin_fd;
//     
//             if (s->resize_pending) {
//                 int width, height;
//                 console_get_size(s, &width, &height);
//                 virtio_console_resize_event(m->console_dev, width, height);
//                 s->resize_pending = FALSE;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3590 {
// Portion of virt_machine_run(): the console/stdin select-setup branch.

use core::ffi::{c_int, c_long, c_ulong, c_void};

// Opaque VIRTIODevice (only ever passed around by pointer).
#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: *mut c_void,
    pub read_data: *mut c_void,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

#[repr(C)]
pub struct STDIODevice {
    pub stdin_fd: c_int,
    pub console_esc_state: c_int,
    pub resize_pending: c_int, // BOOL
}

#[repr(C)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * 8)],
}

extern "C" {
    fn virtio_console_can_write_data(s: *mut VIRTIODevice) -> c_int;
    fn virtio_console_resize_event(s: *mut VIRTIODevice, width: c_int, height: c_int);
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

// Ported from the static C helper console_get_size().
unsafe fn console_get_size(s: *mut STDIODevice, pw: *mut c_int, ph: *mut c_int) {
    let mut ws: Winsize = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut width: c_int = 80;
    let mut height: c_int = 25;
    if ioctl(
        (*s).stdin_fd,
        0x5413 as c_ulong,
        &mut ws as *mut Winsize,
    ) == 0
        && (ws.ws_col as c_int) >= 4
        && (ws.ws_row as c_int) >= 4
    {
        width = ws.ws_col as c_int;
        height = ws.ws_row as c_int;
    }
    *pw = width;
    *ph = height;
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_run__part3590(
    m: *mut VirtMachine,
    stdin_fd: *mut c_int,
    fd_max: *mut c_int,
    rfds: *mut FdSet,
) {
    let console_dev = (*m).console_dev;
    if console_dev.is_null() {
        return;
    }
    if virtio_console_can_write_data(console_dev) == 0 {
        return;
    }

    let s = (*((*m).console)).opaque as *mut STDIODevice;
    let fd = (*s).stdin_fd;
    *stdin_fd = fd;

    // FD_SET(stdin_fd, &rfds)
    let idx: c_int = fd / (8 * 8);
    let shift: u32 = (fd % (8 * 8)) as u32;
    let bit: u64 = 1u64.wrapping_shl(shift);
    let bits = (*rfds).fds_bits.as_mut_ptr();
    let slot = bits.offset(idx as isize);
    let cur = *slot as u64;
    *slot = (cur | bit) as c_long;

    *fd_max = fd;

    if (*s).resize_pending != 0 {
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        console_get_size(s, &mut width, &mut height);
        virtio_console_resize_event(console_dev, width, height);
        (*s).resize_pending = 0;
    }
}
}

// ── `if (m->net)` · temu.c:577-579 · chunk 3591 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->net) {
//             m->net->select_fill(m->net, &fd_max, &rfds, &wfds, &efds, &delay);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3591 {
// Piece of virt_machine_run: net select_fill hook.
use core::ffi::{c_int, c_long};

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut core::ffi::c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            pfd_max: *mut c_int,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            pdelay: *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            select_ret: c_int,
        ),
    >,
    pub device_opaque: *mut core::ffi::c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice) -> c_int>,
    pub device_write_packet:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, carrier_state: c_int)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const core::ffi::c_void,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut core::ffi::c_void,
    pub console: *mut core::ffi::c_void,
    pub fb_dev: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3591(
    m: *mut VirtMachine,
    pfd_max: *mut c_int,
    rfds: *mut FdSet,
    wfds: *mut FdSet,
    efds: *mut FdSet,
    pdelay: *mut c_int,
) {
    unsafe {
        if m.is_null() {
            return;
        }
        let net = (*m).net;
        if net.is_null() {
            return;
        }
        if let Some(f) = (*net).select_fill {
            f(net, pfd_max, rfds, wfds, efds, pdelay);
        }
    }
}
}

// ── `fs_net_set_fdset(&fd_max, &rfds, &wfds, &efds, &delay); tv.tv_sec = delay / 1000; tv.tv_usec =…` · temu.c:581-585 · chunk 3592 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fs_net_set_fdset(&fd_max, &rfds, &wfds, &efds, &delay);
//     
//         tv.tv_sec = delay / 1000;
//         tv.tv_usec = (delay % 1000) * 1000;
//         ret = select(fd_max + 1, &rfds, &wfds, &efds, &tv);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3592 {
use core::ffi::{c_int, c_long};

// struct timeval { __time_t tv_sec; __suseconds_t tv_usec; }  (both long int)
#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// typedef struct { __fd_mask fds_bits[1024 / (8 * sizeof(__fd_mask))]; } fd_set;
// __fd_mask is `long int`, so 1024 / (8 * 8) == 16 entries.
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 16],
}

extern "C" {
    fn fs_net_set_fdset(
        pfd_max: *mut c_int,
        rfds: *mut FdSet,
        wfds: *mut FdSet,
        efds: *mut FdSet,
        ptimeout: *mut c_int,
    );
    fn select(
        nfds: c_int,
        readfds: *mut FdSet,
        writefds: *mut FdSet,
        exceptfds: *mut FdSet,
        timeout: *mut Timeval,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn virt_machine_run__part3592(
    fd_max: *mut c_int,
    rfds: *mut FdSet,
    wfds: *mut FdSet,
    efds: *mut FdSet,
    delay: *mut c_int,
    tv: *mut Timeval,
) -> c_int {
    unsafe {
        fs_net_set_fdset(fd_max, rfds, wfds, efds, delay);

        let d: c_int = *delay;
        (*tv).tv_sec = (d / 1000) as c_long;
        (*tv).tv_usec = ((d % 1000) * 1000) as c_long;

        select(*fd_max + 1, rfds, wfds, efds, tv)
    }
}
}

// ── `if (m->net)` · temu.c:586-588 · chunk 3593 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (m->net) {
//             m->net->select_poll(m->net, &rfds, &wfds, &efds, ret);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3593 {
// Port of the `if (m->net) { m->net->select_poll(...); }` piece of virt_machine_run.
use core::ffi::{c_int, c_long, c_void};

// Mirror of glibc's fd_set (__fd_mask == long, 1024 bits).
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            pfd_max: *mut c_int,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            pdelay: *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(
            net: *mut EthernetDevice,
            rfds: *mut FdSet,
            wfds: *mut FdSet,
            efds: *mut FdSet,
            select_ret: c_int,
        ),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(net: *mut EthernetDevice) -> c_int>,
    pub device_write_packet:
        Option<unsafe extern "C" fn(net: *mut EthernetDevice, buf: *const u8, len: c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(net: *mut EthernetDevice, carrier: c_int)>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_run__part3593(
    m: *mut VirtMachine,
    rfds: *mut FdSet,
    wfds: *mut FdSet,
    efds: *mut FdSet,
    ret: c_int,
) {
    if m.is_null() {
        return;
    }
    let net = (*m).net;
    if net.is_null() {
        return;
    }
    let net_ref = &mut *net;
    if let Some(f) = net_ref.select_poll {
        f(net, rfds, wfds, efds, ret);
    }
}
}

// ── `if (ret > 0)` · temu.c:589-602 · chunk 3594 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret > 0) {
//     
//             if (m->console_dev && 
//                                  ((((
//                                  &rfds
//                                  )->fds_bits)[((
//                                  stdin_fd
//                                  ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                                  stdin_fd
//                                  ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                                           ) {
//                 uint8_t buf[128];
//                 int ret, len;
//                 len = virtio_console_get_write_len(m->console_dev);
//                 len = min_int(len, sizeof(buf));
//                 ret = m->console->read_data(m->console->opaque, buf, len);
//                 if (ret > 0) {
//                     virtio_console_write_data(m->console_dev, buf, ret);
//                 }
//             }
//     
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3594 {
// Port of the `if (ret > 0) { ... }` console-read block of virt_machine_run.
use core::ffi::{c_int, c_long, c_void};

const FD_SET_N: usize = 1024 / (8 * core::mem::size_of::<c_long>());

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; FD_SET_N],
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(*mut c_void, *const u8, c_int)>,
    pub read_data: Option<unsafe extern "C" fn(*mut c_void, *mut u8, c_int) -> c_int>,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const c_void,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut c_void,
}

extern "C" {
    fn virtio_console_get_write_len(s: *mut c_void) -> c_int;
    fn virtio_console_write_data(s: *mut c_void, buf: *const u8, buf_len: c_int) -> c_int;
}

// static inline in C: ported logic
#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_run__part3594(
    m: *mut VirtMachine,
    ret: c_int,
    rfds: *mut FdSet,
    stdin_fd: *const c_int,
) {
    if ret <= 0 {
        return;
    }
    if m.is_null() || rfds.is_null() || stdin_fd.is_null() {
        return;
    }
    let mref = &*m;
    if mref.console_dev.is_null() {
        return;
    }

    let fd = *stdin_fd;
    let word_bits = 8 * core::mem::size_of::<c_long>() as c_int;
    let idx_i = fd / word_bits;
    let shift_i = fd % word_bits;
    if idx_i < 0 || shift_i < 0 || shift_i >= word_bits {
        return;
    }
    let idx = idx_i as usize;
    let shift = shift_i as u32;

    let word = match (*rfds).fds_bits.get(idx) {
        Some(w) => *w,
        None => return,
    };
    let mask = (1u64 << shift) as c_long;
    if word & mask == 0 {
        return;
    }

    if mref.console.is_null() {
        return;
    }
    let cdev = &*mref.console;

    let mut buf: [u8; 128] = [0u8; 128];
    let mut len = virtio_console_get_write_len(mref.console_dev);
    len = min_int(len, 128);
    let r = match cdev.read_data {
        Some(f) => f(cdev.opaque, buf.as_mut_ptr(), len),
        None => return,
    };
    if r > 0 {
        virtio_console_write_data(mref.console_dev, buf.as_ptr(), r);
    }
}
}

// ── `sdl_refresh(m); virt_machine_interp(m, 500000);` · temu.c:605-608 · chunk 3595 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     sdl_refresh(m);
//     
//     
//         virt_machine_interp(m, 500000);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3595 {
// Port of the tail of virt_machine_run: sdl_refresh(m); virt_machine_interp(m, 500000);
// virt_machine_interp is a C `static inline` wrapper (s->vmc->virt_machine_interp(s, n)),
// so its logic is reproduced here instead of being linked to.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct VirtMachineClass {
    pub machine_names: *const c_char,
    pub virt_machine_set_defaults: *const c_void,
    pub virt_machine_init: *const c_void,
    pub virt_machine_end: *const c_void,
    pub virt_machine_get_sleep_duration: *const c_void,
    pub virt_machine_interp: Option<unsafe extern "C" fn(*mut VirtMachine, c_int)>,
    pub vm_mouse_is_absolute: *const c_void,
    pub vm_send_mouse_event: *const c_void,
    pub vm_send_key_event: *const c_void,
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut c_void,
    pub console_dev: *mut c_void,
    pub console: *mut c_void,
    pub fb_dev: *mut c_void,
}

extern "C" {
    fn sdl_refresh(m: *mut VirtMachine);
}

#[no_mangle]
pub unsafe extern "C" fn virt_machine_run__part3595(m: *mut VirtMachine) {
    sdl_refresh(m);

    if m.is_null() {
        return;
    }
    let vmc = (*m).vmc;
    if vmc.is_null() {
        return;
    }
    if let Some(f) = (*vmc).virt_machine_interp {
        f(m, 500000 as c_int);
    }
}
}

// ── `printf("temu version " "2019-02-10" ", Copyright (c) 2016-2018 Fabrice Bellard\n" "usage: temu…` · temu.c:626-638 · chunk 3598 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     printf("temu version " "2019-02-10" ", Copyright (c) 2016-2018 Fabrice Bellard\n"
//                "usage: temu [options] config_file\n"
//                "options are:\n"
//                "-m ram_size       set the RAM size in MB\n"
//                "-rw               allow write access to the disk image (default=snapshot)\n"
//                "-ctrlc            the C-c key stops the emulator instead of being sent to the\n"
//                "                  emulated software\n"
//                "-append cmdline   append cmdline to the kernel command line\n"
//                "-no-accel         disable VM acceleration (KVM, x86 machine only)\n"
//                "\n"
//                "Console keys:\n"
//                "Press C-a x to exit the emulator, C-a h to get some help.\n");
//         exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3598 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
}

static USAGE: &[u8] = b"temu version 2019-02-10, Copyright (c) 2016-2018 Fabrice Bellard\n\
usage: temu [options] config_file\n\
options are:\n\
-m ram_size       set the RAM size in MB\n\
-rw               allow write access to the disk image (default=snapshot)\n\
-ctrlc            the C-c key stops the emulator instead of being sent to the\n\
                  emulated software\n\
-append cmdline   append cmdline to the kernel command line\n\
-no-accel         disable VM acceleration (KVM, x86 machine only)\n\
\n\
Console keys:\n\
Press C-a x to exit the emulator, C-a h to get some help.\n\
\0";

#[no_mangle]
pub extern "C" fn help__part3598() {
    unsafe {
        printf(USAGE.as_ptr() as *const c_char);
        exit(1);
    }
}
}

// ── `net_completed = TRUE;` · temu.c:646-646 · chunk 3601 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     net_completed = TRUE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3601 {
// Set the net_completed flag through a pointer supplied by the C caller
// (the flag has internal linkage in C, so it is passed by address).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn net_start_cb__part3601(net_completed: *mut c_int) {
    unsafe {
        if !net_completed.is_null() {
            *net_completed = 1;
        }
    }
}
}

// ── `return net_completed;` · temu.c:651-651 · chunk 3603 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return net_completed;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3603 {
// Port of the tail of net_poll_cb: returns the current value of the
// file-static flag `net_completed` (a C `BOOL`, i.e. `int`).  The flag has
// internal linkage, so its value is handed to us by value at the call site.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn net_poll_cb__part3603(net_completed: c_int) -> c_int {
    net_completed
}
}

// ── `VirtMachine *s; const char *path, *cmdline, *build_preload_file; int c, option_index, i, ram_si…` · temu.c:662-675 · chunk 3605 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VirtMachine *s;
//         const char *path, *cmdline, *build_preload_file;
//         int c, option_index, i, ram_size, accel_enable;
//         BOOL allow_ctrlc;
//         BlockDeviceModeEnum drive_mode;
//         VirtMachineParams p_s, *p = &p_s;
//     
//         ram_size = -1;
//         allow_ctrlc = FALSE;
//         (void)allow_ctrlc;
//         drive_mode = BF_MODE_SNAPSHOT;
//         accel_enable = -1;
//         cmdline = 
//                  ((void *)0)
//                      ;
//         build_preload_file = 
//                             ((void *)0)
//                                 ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3605 {
// Initialization of main's locals, moved out of C.
use core::ffi::{c_char, c_int};

// BF_MODE_SNAPSHOT from BlockDeviceModeEnum (BF_MODE_RO=0, BF_MODE_RW=1, BF_MODE_SNAPSHOT=2)
const BF_MODE_SNAPSHOT: c_int = 2;

#[no_mangle]
pub extern "C" fn main__part3605(
    ram_size: *mut c_int,
    allow_ctrlc: *mut c_int,
    drive_mode: *mut c_int,
    accel_enable: *mut c_int,
    cmdline: *mut *const c_char,
    build_preload_file: *mut *const c_char,
) {
    unsafe {
        if !ram_size.is_null() {
            *ram_size = -1;
        }
        if !allow_ctrlc.is_null() {
            *allow_ctrlc = 0; // FALSE
        }
        if !drive_mode.is_null() {
            *drive_mode = BF_MODE_SNAPSHOT;
        }
        if !accel_enable.is_null() {
            *accel_enable = -1;
        }
        if !cmdline.is_null() {
            *cmdline = core::ptr::null();
        }
        if !build_preload_file.is_null() {
            *build_preload_file = core::ptr::null();
        }
    }
}
}

// ── `for (; ; )` · temu.c:676-715 · chunk 3606 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             c = getopt_long_only(argc, argv, "hm:", options, &option_index);
//             if (c == -1)
//                 break;
//             switch(c) {
//             case 0:
//                 switch(option_index) {
//                 case 1:
//                     allow_ctrlc = TRUE;
//                     break;
//                 case 2:
//                     drive_mode = BF_MODE_RW;
//                     break;
//                 case 3:
//                     drive_mode = BF_MODE_RO;
//                     break;
//                 case 4:
//                     cmdline = optarg;
//                     break;
//                 case 5:
//                     accel_enable = FALSE;
//                     break;
//                 case 6:
//                     build_preload_file = optarg;
//                     break;
//                 default:
//                     fprintf(
//                            stderr
//                                  , "unknown option index: %d\n", option_index);
//                     exit(1);
//                 }
//                 break;
//             case 'h':
//                 help();
//                 break;
//             case 'm':
//                 ram_size = strtoul(optarg, 
//                                           ((void *)0)
//                                               , 0);
//                 break;
//             default:
//                 exit(1);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3606 {
// Rust port of the getopt_long_only option-parsing loop of main() (temu.c).

use core::ffi::{c_char, c_int, c_ulong, c_void};

// mirror of `struct option`
#[repr(C)]
pub struct COption {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

const FALSE: c_int = 0;
const TRUE: c_int = 1;

// BlockDeviceModeEnum
const BF_MODE_RO: c_int = 0;
const BF_MODE_RW: c_int = 1;

extern "C" {
    fn getopt_long_only(
        argc: c_int,
        argv: *const *mut c_char,
        shortopts: *const c_char,
        longopts: *const COption,
        longind: *mut c_int,
    ) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn help();
    static mut stderr: *mut c_void;
    static mut optarg: *mut c_char;
}

#[no_mangle]
pub extern "C" fn main__part3606(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *const COption,
    allow_ctrlc: *mut c_int,
    drive_mode: *mut c_int,
    cmdline: *mut *const c_char,
    accel_enable: *mut c_int,
    build_preload_file: *mut *const c_char,
    ram_size: *mut c_int,
) {
    unsafe {
        let mut option_index: c_int = 0;
        loop {
            let c = getopt_long_only(
                argc,
                argv,
                b"hm:\0".as_ptr() as *const c_char,
                options,
                &mut option_index,
            );
            if c == -1 {
                break;
            }
            match c {
                0 => match option_index {
                    1 => *allow_ctrlc = TRUE,
                    2 => *drive_mode = BF_MODE_RW,
                    3 => *drive_mode = BF_MODE_RO,
                    4 => *cmdline = optarg as *const c_char,
                    5 => *accel_enable = FALSE,
                    6 => *build_preload_file = optarg as *const c_char,
                    _ => {
                        fprintf(
                            stderr,
                            b"unknown option index: %d\n\0".as_ptr() as *const c_char,
                            option_index,
                        );
                        exit(1);
                    }
                },
                // case 'h'
                104 => help(),
                // case 'm'
                109 => {
                    *ram_size = strtoul(optarg, core::ptr::null_mut(), 0) as c_int;
                }
                _ => exit(1),
            }
        }
    }
}
}

// ── `if (optind >= argc)` · temu.c:717-719 · chunk 3607 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (optind >= argc) {
//             help();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3607 {
// Port of the TARGET: "if (optind >= argc) { help(); }" from main().
use core::ffi::c_int;

extern "C" {
    static mut optind: c_int;
    fn help();
}

#[no_mangle]
pub extern "C" fn main__part3607(argc: c_int) {
    unsafe {
        if optind >= argc {
            help();
        }
    }
}
}

// ── `path = argv[optind++]; virt_machine_set_defaults(p); fs_wget_init(); virt_machine_load_config_f…` · temu.c:721-729 · chunk 3608 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     path = argv[optind++];
//     
//         virt_machine_set_defaults(p);
//     
//         fs_wget_init();
//     
//         virt_machine_load_config_file(p, path, 
//                                               ((void *)0)
//                                                   , 
//                                                     ((void *)0)
//                                                         );
//     
//         fs_net_event_loop(
//                          ((void *)0)
//                              , 
//                                ((void *)0)
//                                    );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3608 {
// Ported piece of main(): path = argv[optind++]; then setup calls.
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn virt_machine_set_defaults(p: *mut c_void);
    fn virt_machine_load_config_file(
        p: *mut c_void,
        filename: *const c_char,
        start_cb: *mut c_void,
        opaque: *mut c_void,
    );
    fn fs_wget_init();
    fn fs_net_event_loop(cb: *mut c_void, opaque: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn main__part3608(
    argv: *mut *mut c_char,
    poptind: *mut c_int,
    p: *mut c_void,
) -> *const c_char {
    // path = argv[optind++];
    let idx = *poptind;
    *poptind = idx.wrapping_add(1);
    let path = *argv.offset(idx as isize) as *const c_char;

    virt_machine_set_defaults(p);

    fs_wget_init();

    virt_machine_load_config_file(
        p,
        path,
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    fs_net_event_loop(
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );

    path
}
}

// ── `if (ram_size > 0)` · temu.c:734-736 · chunk 3609 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ram_size > 0) {
//             p->ram_size = (uint64_t)ram_size << 20;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3609 {
// Port of the RAM-size override block from main().
#[no_mangle]
pub extern "C" fn main__part3609(ram_size: core::ffi::c_int, p_ram_size: *mut u64) {
    if ram_size > 0 {
        unsafe {
            *p_ram_size = (ram_size as u64) << 20;
        }
    }
}
}

// ── `if (accel_enable != -1)` · temu.c:737-738 · chunk 3610 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (accel_enable != -1)
//             p->accel_enable = accel_enable;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3610 {
// Port of the accel_enable override in main: if accel_enable != -1,
// store it into p->accel_enable (BOOL == int in the C code).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn main__part3610(accel_field: *mut c_int, accel_enable: c_int) {
    if accel_enable != -1 {
        unsafe {
            *accel_field = accel_enable;
        }
    }
}
}

// ── `if (cmdline)` · temu.c:739-741 · chunk 3611 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cmdline) {
//             vm_add_cmdline(p, cmdline);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3611 {
use core::ffi::{c_char, c_void};

extern "C" {
    // void vm_add_cmdline(VirtMachineParams *p, const char *cmdline);
    fn vm_add_cmdline(p: *mut c_void, cmdline: *const c_char);
}

#[no_mangle]
pub extern "C" fn main__part3611(p: *mut c_void, cmdline: *const c_char) {
    if !cmdline.is_null() {
        unsafe {
            vm_add_cmdline(p, cmdline);
        }
    }
}
}

// ── `for (i = 0; i < p->drive_count; i++)` · temu.c:744-762 · chunk 3612 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->drive_count; i++) {
//             BlockDevice *drive;
//             char *fname;
//             fname = get_file_path(p->cfg_filename, p->tab_drive[i].filename);
//     
//             if (is_url(fname)) {
//                 net_completed = FALSE;
//                 drive = block_device_init_http(fname, 128 * 1024,
//                                                net_start_cb, 
//                                                             ((void *)0)
//                                                                 );
//     
//                 fs_net_event_loop(net_poll_cb, 
//                                               ((void *)0)
//                                                   );
//             } else
//     
//             {
//                 drive = block_device_init(fname, drive_mode);
//             }
//             free(fname);
//             p->tab_drive[i].block_dev = drive;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3612 {
// Port of the drive-initialization loop of main() in temu.c.
// `block_device_init()` and the bf_* helpers are `static` in temu.c, so they
// have no linker symbol: their logic is ported here as private Rust fns.
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::ffi::{c_char, c_int, c_void};

const BF_MODE_RO: c_int = 0;
const BF_MODE_RW: c_int = 1;
const BF_MODE_SNAPSHOT: c_int = 2;

const SEEK_SET: c_int = 0;
const SEEK_END: c_int = 2;
const SECTOR_SIZE: usize = 512;

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

#[repr(C)]
pub struct BlockDevice {
    pub get_sector_count: Option<unsafe extern "C" fn(*mut BlockDevice) -> i64>,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *mut u8,
            c_int,
            *mut c_void,
            *mut c_void,
        ) -> c_int,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut BlockDevice,
            u64,
            *const u8,
            c_int,
            *mut c_void,
            *mut c_void,
        ) -> c_int,
    >,
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct BlockDeviceFile {
    pub f: *mut c_void,
    pub nb_sectors: i64,
    pub mode: c_int,
    pub sector_table: *mut *mut u8,
}

// static BOOL net_completed;  (static in temu.c: no linker symbol)
static mut NET_COMPLETED: c_int = 0;

// static void net_start_cb(void *arg) { net_completed = TRUE; }
unsafe extern "C" fn net_start_cb(_arg: *mut c_void) {
    NET_COMPLETED = 1;
}

// static BOOL net_poll_cb(void *arg) { return net_completed; }
unsafe extern "C" fn net_poll_cb(_arg: *mut c_void) -> c_int {
    NET_COMPLETED
}

extern "C" {
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn is_url(path: *const c_char) -> c_int;
    fn block_device_init_http(
        url: *const c_char,
        max_cache_size_kb: c_int,
        start_cb: Option<unsafe extern "C" fn(*mut c_void)>,
        start_opaque: *mut c_void,
    ) -> *mut c_void;
    fn fs_net_event_loop(
        cb: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
        opaque: *mut c_void,
    );
    fn free(ptr: *mut c_void);

    // libc / cutils
    fn mallocz(size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    #[link_name = "fopen64"]
    fn fopen(filename: *const c_char, modes: *const c_char) -> *mut c_void;
    fn fseek(stream: *mut c_void, off: i64, whence: c_int) -> c_int;
    #[link_name = "ftello64"]
    fn ftello64(stream: *mut c_void) -> i64;
    fn fread(ptr: *mut c_void, size: usize, n: usize, stream: *mut c_void) -> usize;
    fn fwrite(ptr: *const c_void, size: usize, n: usize, stream: *mut c_void) -> usize;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn abort() -> !;
}

// static int64_t bf_get_sector_count(BlockDevice *bs)
unsafe extern "C" fn bf_get_sector_count(bs: *mut BlockDevice) -> i64 {
    let bf = (*bs).opaque as *mut BlockDeviceFile;
    (*bf).nb_sectors
}

// static int bf_read_async(BlockDevice *bs, uint64_t sector_num, uint8_t *buf,
//                          int n, BlockDeviceCompletionFunc *cb, void *opaque)
unsafe extern "C" fn bf_read_async(
    bs: *mut BlockDevice,
    mut sector_num: u64,
    mut buf: *mut u8,
    n: c_int,
    _cb: *mut c_void,
    _opaque: *mut c_void,
) -> c_int {
    let bf = (*bs).opaque as *mut BlockDeviceFile;
    if (*bf).f.is_null() {
        return -1;
    }
    if (*bf).mode == BF_MODE_SNAPSHOT {
        let mut i: c_int = 0;
        while i < n {
            let slot = *(*bf).sector_table.add(sector_num as usize);
            if slot.is_null() {
                fseek((*bf).f, sector_num.wrapping_mul(512) as i64, SEEK_SET);
                fread(buf as *mut c_void, 1, SECTOR_SIZE, (*bf).f);
            } else {
                core::ptr::copy_nonoverlapping(slot, buf, SECTOR_SIZE);
            }
            sector_num = sector_num.wrapping_add(1);
            buf = buf.add(SECTOR_SIZE);
            i += 1;
        }
    } else {
        fseek((*bf).f, sector_num.wrapping_mul(512) as i64, SEEK_SET);
        fread(
            buf as *mut c_void,
            1,
            n.wrapping_mul(512) as usize,
            (*bf).f,
        );
    }
    // synchronous read
    0
}

// static int bf_write_async(BlockDevice *bs, uint64_t sector_num,
//                           const uint8_t *buf, int n,
//                           BlockDeviceCompletionFunc *cb, void *opaque)
unsafe extern "C" fn bf_write_async(
    bs: *mut BlockDevice,
    mut sector_num: u64,
    mut buf: *const u8,
    n: c_int,
    _cb: *mut c_void,
    _opaque: *mut c_void,
) -> c_int {
    let bf = (*bs).opaque as *mut BlockDeviceFile;
    let ret: c_int;

    let mode = (*bf).mode;
    if mode == BF_MODE_RO {
        ret = -1;
    } else if mode == BF_MODE_RW {
        fseek((*bf).f, sector_num.wrapping_mul(512) as i64, SEEK_SET);
        fwrite(
            buf as *const c_void,
            1,
            n.wrapping_mul(512) as usize,
            (*bf).f,
        );
        ret = 0;
    } else if mode == BF_MODE_SNAPSHOT {
        if sector_num.wrapping_add(n as u64) > (*bf).nb_sectors as u64 {
            return -1;
        }
        let mut i: c_int = 0;
        while i < n {
            let slot = (*bf).sector_table.add(sector_num as usize);
            if (*slot).is_null() {
                *slot = malloc(SECTOR_SIZE) as *mut u8;
            }
            core::ptr::copy_nonoverlapping(buf, *slot, SECTOR_SIZE);
            sector_num = sector_num.wrapping_add(1);
            buf = buf.add(SECTOR_SIZE);
            i += 1;
        }
        ret = 0;
    } else {
        abort();
    }

    ret
}

// static BlockDevice *block_device_init(const char *filename,
//                                       BlockDeviceModeEnum mode)
unsafe fn block_device_init_impl(filename: *const c_char, mode: c_int) -> *mut c_void {
    let mode_str: *const c_char = if mode == BF_MODE_RW {
        b"r+b\0".as_ptr() as *const c_char
    } else {
        b"rb\0".as_ptr() as *const c_char
    };

    let f = fopen(filename, mode_str);
    if f.is_null() {
        perror(filename);
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    let file_size = ftello64(f);

    let bs = mallocz(core::mem::size_of::<BlockDevice>()) as *mut BlockDevice;
    let bf = mallocz(core::mem::size_of::<BlockDeviceFile>()) as *mut BlockDeviceFile;

    (*bf).mode = mode;
    (*bf).nb_sectors = file_size / 512;
    (*bf).f = f;

    if mode == BF_MODE_SNAPSHOT {
        let ent_size = core::mem::size_of::<*mut u8>() as u64;
        let sz = ((*bf).nb_sectors as u64).wrapping_mul(ent_size) as usize;
        (*bf).sector_table = mallocz(sz) as *mut *mut u8;
    }

    (*bs).opaque = bf as *mut c_void;
    (*bs).get_sector_count = Some(bf_get_sector_count);
    (*bs).read_async = Some(bf_read_async);
    (*bs).write_async = Some(bf_write_async);
    bs as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn main__part3612(pv: *mut c_void, drive_mode: c_int) {
    let p = pv as *mut VirtMachineParams;
    let mut i: c_int = 0;
    while i < (*p).drive_count {
        let entry = (*p).tab_drive.as_mut_ptr().add(i as usize);
        let fname: *mut c_char = get_file_path((*p).cfg_filename, (*entry).filename);

        let drive: *mut c_void;
        if is_url(fname) != 0 {
            NET_COMPLETED = 0;
            drive = block_device_init_http(
                fname,
                128 * 1024,
                Some(net_start_cb),
                core::ptr::null_mut(),
            );

            fs_net_event_loop(Some(net_poll_cb), core::ptr::null_mut());
        } else {
            drive = block_device_init_impl(fname, drive_mode);
        }
        free(fname as *mut c_void);
        (*entry).block_dev = drive;
        i += 1;
    }
}
}

// ── `for (i = 0; i < p->fs_count; i++)` · temu.c:764-794 · chunk 3613 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < p->fs_count; i++) {
//             FSDevice *fs;
//             const char *path;
//             path = p->tab_fs[i].filename;
//     
//             if (is_url(path)) {
//                 fs = fs_net_init(path, 
//                                       ((void *)0)
//                                           , 
//                                             ((void *)0)
//                                                 );
//                 if (!fs)
//                     exit(1);
//                 if (build_preload_file)
//                     fs_dump_cache_load(fs, build_preload_file);
//                 fs_net_event_loop(
//                                  ((void *)0)
//                                      , 
//                                        ((void *)0)
//                                            );
//             } else
//     
//             {
//     
//     
//     
//     
//                 char *fname;
//                 fname = get_file_path(p->cfg_filename, path);
//                 fs = fs_disk_init(fname);
//                 if (!fs) {
//                     fprintf(
//                            stderr
//                                  , "%s: must be a directory\n", fname);
//                     exit(1);
//                 }
//                 free(fname);
//     
//             }
//             p->tab_fs[i].fs_dev = fs;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3613 {
// Port of the fs-device setup loop of main() (temu.c lines 764-794).
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr::null_mut;

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

// VM_FILE_COUNT == 4
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
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int);
    fn free(ptr: *mut c_void);

    fn is_url(path: *const c_char) -> c_int;
    fn get_file_path(base_filename: *const c_char, filename: *const c_char) -> *mut c_char;
    fn fs_net_init(
        url: *const c_char,
        start: Option<extern "C" fn(*mut c_void)>,
        opaque: *mut c_void,
    ) -> *mut c_void;
    fn fs_disk_init(root_path: *const c_char) -> *mut c_void;
    fn fs_dump_cache_load(fs: *mut c_void, filename: *const c_char);
    fn fs_net_event_loop(
        cb: Option<extern "C" fn(*mut c_void) -> c_int>,
        opaque: *mut c_void,
    );
}

#[no_mangle]
pub extern "C" fn main__part3613(p: *mut VirtMachineParams, build_preload_file: *const c_char) {
    let mut i: c_int = 0;
    unsafe {
        while i < (*p).fs_count {
            let entry: *mut VMFSEntry = (*p).tab_fs.as_mut_ptr().add(i as usize);
            let path: *const c_char = (*entry).filename;
            let mut fs: *mut c_void;

            if is_url(path) != 0 {
                fs = fs_net_init(path, None, null_mut());
                if fs.is_null() {
                    exit(1);
                }
                if !build_preload_file.is_null() {
                    fs_dump_cache_load(fs, build_preload_file);
                }
                fs_net_event_loop(None, null_mut());
            } else {
                let fname: *mut c_char = get_file_path((*p).cfg_filename, path);
                fs = fs_disk_init(fname);
                if fs.is_null() {
                    fprintf(
                        stderr,
                        b"%s: must be a directory\n\0".as_ptr() as *const c_char,
                        fname,
                    );
                    exit(1);
                }
                free(fname as *mut c_void);
            }
            (*entry).fs_dev = fs;
            i += 1;
        }
    }
}
}

// ── `if (p->display_device) … else` · temu.c:819-831 · chunk 3615 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (p->display_device) {
//             sdl_init(p->width, p->height);
//             p->console = console_init(TRUE);
//         } else
//     
//         {
//     
//     
//     
//     
//             p->console = console_init(allow_ctrlc);
//     
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3615 {
// Port of the console/display initialization block of main().

use core::ffi::{c_char, c_int};

type BOOL = c_int;

#[repr(C)]
pub struct CharacterDevice {
    _private: [u8; 0],
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
pub struct VMFileEntry {
    pub filename: *mut c_char,
    pub buf: *mut u8,
    pub len: c_int,
}

pub const VM_FILE_COUNT: usize = 4;

#[repr(C)]
pub struct VirtMachineParams {
    pub cfg_filename: *mut c_char,
    pub vmc: *const core::ffi::c_void,
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
    pub files: [VMFileEntry; VM_FILE_COUNT],
}

extern "C" {
    fn sdl_init(width: c_int, height: c_int);
    fn console_init(allow_ctrlc: BOOL) -> *mut CharacterDevice;
}

#[no_mangle]
pub extern "C" fn main__part3615(p: *mut VirtMachineParams, allow_ctrlc: BOOL) {
    unsafe {
        let display = (*p).display_device;
        if !display.is_null() {
            sdl_init((*p).width, (*p).height);
            (*p).console = console_init(1);
        } else {
            (*p).console = console_init(allow_ctrlc);
        }
    }
}
}

// ── `p->rtc_real_time = TRUE; s = virt_machine_init(p);` · temu.c:832-834 · chunk 3616 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     p->rtc_real_time = TRUE;
//     
//         s = virt_machine_init(p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3616 {
// Port of main()'s tail piece:
//     p->rtc_real_time = TRUE;
//     s = virt_machine_init(p);
// Exported as main__part3616.

use core::ffi::{c_char, c_int};

// Opaque C types (only used behind pointers here).
#[repr(C)]
pub struct BlockDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct FSDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct EthernetDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    _opaque: [u8; 0],
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

// VM_FILE_COUNT == 4
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
    fn virt_machine_init(p: *const VirtMachineParams) -> *mut VirtMachine;
}

#[no_mangle]
pub extern "C" fn main__part3616(p: *mut VirtMachineParams) -> *mut VirtMachine {
    unsafe {
        (*p).rtc_real_time = 1; // TRUE
        virt_machine_init(p as *const VirtMachineParams)
    }
}
}

// ── `if (!s)` · temu.c:835-836 · chunk 3617 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!s)
//             exit(1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3617 {
// Port of the null check on the freshly created VirtMachine.
use core::ffi::{c_int, c_void};

extern "C" {
    // C: extern void exit(int __status) __attribute__((__noreturn__));
    fn exit(status: c_int) -> !;
}

#[no_mangle]
pub extern "C" fn main__part3617(s: *mut c_void) {
    if s.is_null() {
        unsafe {
            exit(1);
        }
    }
}
}

// ── `virt_machine_free_config(p);` · temu.c:838-838 · chunk 3618 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     virt_machine_free_config(p);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3618 {
// Wrapper for virt_machine_free_config(p) from main().
// VirtMachineParams is opaque here: only a pointer is passed through.

extern "C" {
    fn virt_machine_free_config(p: *mut core::ffi::c_void);
}

#[no_mangle]
pub extern "C" fn main__part3618(p: *mut core::ffi::c_void) {
    unsafe {
        virt_machine_free_config(p);
    }
}
}

// ── `if (s->net)` · temu.c:840-842 · chunk 3619 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->net) {
//             s->net->device_set_carrier(s->net, TRUE);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3619 {
// Port of the carrier-enable step from main().
use core::ffi::{c_int, c_void};

type BOOL = c_int;

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: Option<
        unsafe extern "C" fn(
            *mut EthernetDevice,
            *mut c_int,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_int,
        ),
    >,
    pub select_poll: Option<
        unsafe extern "C" fn(*mut EthernetDevice, *mut c_void, *mut c_void, *mut c_void, c_int),
    >,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice) -> BOOL>,
    pub device_write_packet:
        Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub device_set_carrier: Option<unsafe extern "C" fn(*mut EthernetDevice, BOOL)>,
}

#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CharacterDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FBDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachineClass {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VirtMachine {
    pub vmc: *const VirtMachineClass,
    pub net: *mut EthernetDevice,
    pub console_dev: *mut VIRTIODevice,
    pub console: *mut CharacterDevice,
    pub fb_dev: *mut FBDevice,
}

#[no_mangle]
pub unsafe extern "C" fn main__part3619(s: *mut VirtMachine) {
    if s.is_null() {
        return;
    }
    let net = (*s).net;
    if !net.is_null() {
        if let Some(f) = (*net).device_set_carrier {
            f(net, 1 as BOOL);
        }
    }
}
}

// ── `for (; ; )` · temu.c:844-846 · chunk 3620 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             virt_machine_run(s);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3620 {
// Ported piece of main(): the infinite virt_machine_run loop.
use core::ffi::c_void;

extern "C" {
    fn virt_machine_run(m: *mut c_void);
}

#[no_mangle]
pub extern "C" fn main__part3620(s: *mut c_void) {
    loop {
        unsafe {
            virt_machine_run(s);
        }
    }
}
}

// ── `virt_machine_end(s); return 0;` · temu.c:847-848 · chunk 3621 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     virt_machine_end(s);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3621 {
// Port of the tail of main(): shut the virtual machine down and return 0.

#[repr(C)]
pub struct VirtMachine {
    _private: [u8; 0],
}

extern "C" {
    fn virt_machine_end(s: *mut VirtMachine);
}

#[no_mangle]
pub extern "C" fn main__part3621(s: *mut VirtMachine) -> core::ffi::c_int {
    unsafe {
        virt_machine_end(s);
    }
    0
}
}

