//! From `slirp/slirp.c`.

// ── `get_dns_addr` · slirp/slirp.c:158-224 · chunk 2497 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int get_dns_addr(struct in_addr *pdns_addr)
//     {
//         char buff[512];
//         char buff2[257];
//         FILE *f;
//         int found = 0;
//         struct in_addr tmp_addr;
//     
//         if (dns_addr.s_addr != 0) {
//             struct stat old_stat;
//             if ((curtime - dns_addr_time) < 1000) {
//                 *pdns_addr = dns_addr;
//                 return 0;
//             }
//             old_stat = dns_addr_stat;
//             if (stat("/etc/resolv.conf", &dns_addr_stat) != 0)
//                 return -1;
//             if ((dns_addr_stat.st_dev == old_stat.st_dev)
//                 && (dns_addr_stat.st_ino == old_stat.st_ino)
//                 && (dns_addr_stat.st_size == old_stat.st_size)
//                 && (dns_addr_stat.
//                                  st_mtim.tv_sec 
//                                           == old_stat.
//                                                       st_mtim.tv_sec
//                                                               )) {
//                 *pdns_addr = dns_addr;
//                 return 0;
//             }
//         }
//     
//         f = fopen("/etc/resolv.conf", "r");
//         if (!f)
//             return -1;
//     
//     
//     
//     
//         while (fgets(buff, 512, f) != 
//                                      ((void *)0)
//                                          ) {
//             if (sscanf(buff, "nameserver%*[ \t]%256s", buff2) == 1) {
//                 if (!inet_aton(buff2, &tmp_addr))
//                     continue;
//     
//                 if (!found) {
//                     *pdns_addr = tmp_addr;
//                     dns_addr = tmp_addr;
//                     dns_addr_time = curtime;
//                 }
//     
//     
//     
//     
//                 if (++found > 3) {
//     
//     
//     
//                     break;
//                 }
//     
//     
//     
//     
//             }
//         }
//         fclose(f);
//     
//     
//     
//         if (!found)
//             return -1;
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2497 {
// Port of get_dns_addr() from slirp/slirp.c (non-WIN32 / resolv.conf variant).
//
// The C statics dns_addr / dns_addr_time / dns_addr_stat have no linker symbol
// (and nothing else in the program references them), so they are mirrored here
// as private Rust statics.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

// Mirror of glibc's struct stat as seen by this translation unit.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub glibc_reserved: [c_long; 3],
}

const ZERO_TIMESPEC: Timespec = Timespec {
    tv_sec: 0,
    tv_nsec: 0,
};

const ZERO_STAT: Stat = Stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: ZERO_TIMESPEC,
    st_mtim: ZERO_TIMESPEC,
    st_ctim: ZERO_TIMESPEC,
    glibc_reserved: [0; 3],
};

static mut DNS_ADDR: InAddr = InAddr { s_addr: 0 };
static mut DNS_ADDR_TIME: c_uint = 0;
static mut DNS_ADDR_STAT: Stat = ZERO_STAT;

extern "C" {
    static mut curtime: c_uint;
    // glibc redirects stat() to stat64 in this configuration.
    fn stat64(path: *const c_char, buf: *mut Stat) -> c_int;
    // glibc redirects fopen() to fopen64 in this configuration.
    fn fopen64(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fgets(s: *mut c_char, n: c_int, stream: *mut c_void) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut c_void) -> c_int;
    fn inet_aton(cp: *const c_char, inp: *mut InAddr) -> c_int;
}

#[no_mangle]
pub extern "C" fn get_dns_addr(pdns_addr: *mut InAddr) -> c_int {
    let path: &[u8] = b"/etc/resolv.conf\0";
    let mode: &[u8] = b"r\0";
    let fmt: &[u8] = b"nameserver%*[ \t]%256s\0";

    let mut buff = [0 as c_char; 512];
    let mut buff2 = [0 as c_char; 257];
    let mut found: c_int = 0;
    let mut tmp_addr = InAddr { s_addr: 0 };

    unsafe {
        if (*core::ptr::addr_of!(DNS_ADDR)).s_addr != 0 {
            let now = *core::ptr::addr_of!(curtime);
            let last = *core::ptr::addr_of!(DNS_ADDR_TIME);
            if now.wrapping_sub(last) < 1000 {
                *pdns_addr = *core::ptr::addr_of!(DNS_ADDR);
                return 0;
            }
            let old_stat: Stat = *core::ptr::addr_of!(DNS_ADDR_STAT);
            if stat64(
                path.as_ptr() as *const c_char,
                core::ptr::addr_of_mut!(DNS_ADDR_STAT),
            ) != 0
            {
                return -1;
            }
            let new_stat: Stat = *core::ptr::addr_of!(DNS_ADDR_STAT);
            if new_stat.st_dev == old_stat.st_dev
                && new_stat.st_ino == old_stat.st_ino
                && new_stat.st_size == old_stat.st_size
                && new_stat.st_mtim.tv_sec == old_stat.st_mtim.tv_sec
            {
                *pdns_addr = *core::ptr::addr_of!(DNS_ADDR);
                return 0;
            }
        }

        let f = fopen64(
            path.as_ptr() as *const c_char,
            mode.as_ptr() as *const c_char,
        );
        if f.is_null() {
            return -1;
        }

        loop {
            if fgets(buff.as_mut_ptr(), 512, f).is_null() {
                break;
            }
            if sscanf(
                buff.as_ptr(),
                fmt.as_ptr() as *const c_char,
                buff2.as_mut_ptr(),
            ) == 1
            {
                if inet_aton(buff2.as_ptr(), core::ptr::addr_of_mut!(tmp_addr)) == 0 {
                    continue;
                }
                // If it's the first one, set it to dns_addr
                if found == 0 {
                    *pdns_addr = tmp_addr;
                    DNS_ADDR = tmp_addr;
                    DNS_ADDR_TIME = *core::ptr::addr_of!(curtime);
                }
                found += 1;
                if found > 3 {
                    break;
                }
            }
        }
        fclose(f);
    }

    if found == 0 {
        -1
    } else {
        0
    }
}
}

// ── `if (dns_addr.s_addr != 0)` · slirp/slirp.c:166-182 · chunk 2498 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dns_addr.s_addr != 0) {
//             struct stat old_stat;
//             if ((curtime - dns_addr_time) < 1000) {
//                 *pdns_addr = dns_addr;
//                 return 0;
//             }
//             old_stat = dns_addr_stat;
//             if (stat("/etc/resolv.conf", &dns_addr_stat) != 0)
//                 return -1;
//             if ((dns_addr_stat.st_dev == old_stat.st_dev)
//                 && (dns_addr_stat.st_ino == old_stat.st_ino)
//                 && (dns_addr_stat.st_size == old_stat.st_size)
//                 && (dns_addr_stat.
//                                  st_mtim.tv_sec 
//                                           == old_stat.
//                                                       st_mtim.tv_sec
//                                                               )) {
//                 *pdns_addr = dns_addr;
//                 return 0;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2498 {
// Port of the "cached dns address" prefix of get_dns_addr().
// Returns -1 -> C caller must return -1, 1 -> C caller must return 0,
// 0 -> C caller continues with the rest of get_dns_addr().

use core::ffi::{c_char, c_int};

/// struct in_addr { in_addr_t s_addr; }  (in_addr_t == uint32_t)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/// glibc `struct stat` as seen by the preprocessed translation unit.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

extern "C" {
    // C declaration is `stat` with __asm__("stat64"), so the linker
    // symbol is stat64.
    #[link_name = "stat64"]
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
}

static RESOLV_CONF: [u8; 17] = *b"/etc/resolv.conf\0";

#[no_mangle]
pub extern "C" fn get_dns_addr__part2498(
    pdns_addr: *mut InAddr,
    dns_addr: *const InAddr,
    curtime_val: u32,
    dns_addr_time_val: u32,
    dns_addr_stat: *mut Stat,
) -> c_int {
    unsafe {
        let addr = (*dns_addr).s_addr;
        if addr != 0 {
            if curtime_val.wrapping_sub(dns_addr_time_val) < 1000 {
                *pdns_addr = InAddr { s_addr: addr };
                return 1;
            }
            let old_stat: Stat = *dns_addr_stat;
            if stat(RESOLV_CONF.as_ptr() as *const c_char, dns_addr_stat) != 0 {
                return -1;
            }
            let new_stat: Stat = *dns_addr_stat;
            if new_stat.st_dev == old_stat.st_dev
                && new_stat.st_ino == old_stat.st_ino
                && new_stat.st_size == old_stat.st_size
                && new_stat.st_mtim.tv_sec == old_stat.st_mtim.tv_sec
            {
                *pdns_addr = InAddr { s_addr: addr };
                return 1;
            }
        }
        0
    }
}
}

// ── `f = fopen("/etc/resolv.conf", "r");` · slirp/slirp.c:184-184 · chunk 2499 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     f = fopen("/etc/resolv.conf", "r");
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2499 {
// Port of the `f = fopen("/etc/resolv.conf", "r");` statement from get_dns_addr.
// In the preprocessed translation unit, fopen is declared with `__asm__("fopen64")`,
// so the real symbol being called is fopen64.

use core::ffi::c_char;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    #[link_name = "fopen64"]
    fn fopen64(filename: *const c_char, modes: *const c_char) -> *mut FILE;
}

static RESOLV_CONF: [u8; 17] = *b"/etc/resolv.conf\0";
static MODE_R: [u8; 2] = *b"r\0";

#[no_mangle]
pub extern "C" fn get_dns_addr__part2499() -> *mut FILE {
    unsafe {
        fopen64(
            RESOLV_CONF.as_ptr() as *const c_char,
            MODE_R.as_ptr() as *const c_char,
        )
    }
}
}

// ── `if (!f)` · slirp/slirp.c:185-186 · chunk 2500 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2500 {
// Ported piece of get_dns_addr(): the guard `if (!f) return -1;` right after
// f = fopen("/etc/resolv.conf", "r");  Only the decision is moved to Rust; the
// actual `return -1;` stays in the C caller.
use core::ffi::{c_int, c_void};

/// Returns 1 when the stream pointer is NULL (caller must `return -1`), 0 otherwise.
#[no_mangle]
pub extern "C" fn get_dns_addr__part2500(f: *mut c_void) -> c_int {
    if f.is_null() {
        1
    } else {
        0
    }
}
}

// ── `while (fgets(buff, 512, f) != ((void *)0)` · slirp/slirp.c:191-216 · chunk 2501 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     while (fgets(buff, 512, f) != 
//                                      ((void *)0)
//                                          ) {
//             if (sscanf(buff, "nameserver%*[ \t]%256s", buff2) == 1) {
//                 if (!inet_aton(buff2, &tmp_addr))
//                     continue;
//     
//                 if (!found) {
//                     *pdns_addr = tmp_addr;
//                     dns_addr = tmp_addr;
//                     dns_addr_time = curtime;
//                 }
//     
//     
//     
//     
//                 if (++found > 3) {
//     
//     
//     
//                     break;
//                 }
//     
//     
//     
//     
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2501 {
// Port of the resolv.conf parsing loop from get_dns_addr (slirp/slirp.c).
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

extern "C" {
    fn fgets(s: *mut c_char, n: c_int, stream: *mut c_void) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int;
    static curtime: c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn get_dns_addr__part2501(
    buff: *mut c_char,
    buff2: *mut c_char,
    f: *mut c_void,
    found: *mut c_int,
    tmp_addr: *mut in_addr,
    pdns_addr: *mut in_addr,
    dns_addr: *mut in_addr,
    dns_addr_time: *mut c_uint,
) {
    let fmt: &[u8] = b"nameserver%*[ \t]%256s\0";
    loop {
        let line = fgets(buff, 512, f);
        if line.is_null() {
            break;
        }
        if sscanf(buff, fmt.as_ptr() as *const c_char, buff2) == 1 {
            if inet_aton(buff2, tmp_addr) == 0 {
                continue;
            }
            if *found == 0 {
                let a = (*tmp_addr).s_addr;
                (*pdns_addr).s_addr = a;
                (*dns_addr).s_addr = a;
                *dns_addr_time = curtime;
            }
            *found = (*found).wrapping_add(1);
            if *found > 3 {
                break;
            }
        }
    }
}
}

// ── `fclose(f);` · slirp/slirp.c:217-217 · chunk 2502 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     fclose(f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2502 {
use core::ffi::{c_int, c_void};

extern "C" {
    fn fclose(stream: *mut c_void) -> c_int;
}

// Port of the TARGET `fclose(f);` from get_dns_addr.
// The return value is discarded in C, but it is returned for completeness.
#[no_mangle]
pub extern "C" fn get_dns_addr__part2502(f: *mut c_void) -> c_int {
    unsafe { fclose(f) }
}
}

// ── `if (!found)` · slirp/slirp.c:221-222 · chunk 2503 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!found)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2503 {
// Port of the tail check of get_dns_addr:
//     if (!found) return -1;
// Returns 1 when the caller must perform `return -1`, 0 otherwise.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn get_dns_addr__part2503(found: c_int) -> c_int {
    if found == 0 {
        1
    } else {
        0
    }
}
}

// ── `return 0;` · slirp/slirp.c:223-223 · chunk 2504 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2504 {
// Piece of get_dns_addr(): the final "return 0;" after the resolv.conf scan.
// It reads nothing from the enclosing scope and simply reports success.
#[no_mangle]
pub extern "C" fn get_dns_addr__part2504() -> core::ffi::c_int {
    0
}
}

// ── `slirp_init_once` · slirp/slirp.c:228-246 · chunk 2505 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void slirp_init_once(void)
//     {
//         static int initialized;
//     
//     
//     
//     
//         if (initialized) {
//             return;
//         }
//         initialized = 1;
//     
//     
//     
//     
//     
//     
//         loopback_addr.s_addr = htonl(
//                                     ((in_addr_t) 0x7f000001)
//                                                    );
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2505 {
// Rewrite of slirp_init_once() from slirp/slirp.c
use core::ffi::c_int;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

extern "C" {
    static mut loopback_addr: InAddr;
    fn htonl(__hostlong: u32) -> u32;
}

static mut INITIALIZED: c_int = 0;

#[no_mangle]
pub extern "C" fn slirp_init_once() {
    unsafe {
        let initialized = core::ptr::addr_of_mut!(INITIALIZED);
        if *initialized != 0 {
            return;
        }
        *initialized = 1;

        let addr = core::ptr::addr_of_mut!(loopback_addr);
        (*addr).s_addr = htonl(0x7f00_0001u32);
    }
}
}

// ── `if (initialized)` · slirp/slirp.c:235-237 · chunk 2506 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (initialized) {
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2506 {
// Port of the early-return guard of slirp_init_once():
// if the one-time initialisation flag is already set, the caller must return.
#[no_mangle]
pub extern "C" fn slirp_init_once__part2506(initialized: core::ffi::c_int) -> core::ffi::c_int {
    if initialized != 0 {
        1
    } else {
        0
    }
}
}

// ── `initialized = 1; loopback_addr.s_addr = htonl( ((in_addr_t) 0x7f000001) );` · slirp/slirp.c:238-245 · chunk 2507 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     initialized = 1;
//     
//     
//     
//     
//     
//     
//         loopback_addr.s_addr = htonl(
//                                     ((in_addr_t) 0x7f000001)
//                                                    );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2507 {
// Ported piece of slirp_init_once: marks the one-time init as done and
// sets the loopback address.
#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

extern "C" {
    fn htonl(__hostlong: u32) -> u32;
    static mut loopback_addr: in_addr;
}

#[no_mangle]
pub extern "C" fn slirp_init_once__part2507(initialized: *mut c_int) {
    unsafe {
        if !initialized.is_null() {
            *initialized = 1;
        }
        let addr = core::ptr::addr_of_mut!(loopback_addr);
        (*addr).s_addr = htonl(0x7f000001u32);
    }
}
}

// ── `slirp_init` · slirp/slirp.c:248-285 · chunk 2508 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp_init(int restricted, struct in_addr vnetwork,
//                       struct in_addr vnetmask, struct in_addr vhost,
//                       const char *vhostname, const char *tftp_path,
//                       const char *bootfile, struct in_addr vdhcp_start,
//                       struct in_addr vnameserver, void *opaque)
//     {
//         Slirp *slirp = mallocz(sizeof(Slirp));
//     
//         slirp_init_once();
//     
//         slirp->restricted = restricted;
//     
//         if_init(slirp);
//         ip_init(slirp);
//     
//     
//         m_init(slirp);
//     
//         slirp->vnetwork_addr = vnetwork;
//         slirp->vnetwork_mask = vnetmask;
//         slirp->vhost_addr = vhost;
//         if (vhostname) {
//             pstrcpy(slirp->client_hostname, sizeof(slirp->client_hostname),
//                     vhostname);
//         }
//         if (tftp_path) {
//             slirp->tftp_prefix = strdup(tftp_path);
//         }
//         if (bootfile) {
//             slirp->bootp_filename = strdup(bootfile);
//         }
//         slirp->vdhcp_startaddr = vdhcp_start;
//         slirp->vnameserver_addr = vnameserver;
//     
//         slirp->opaque = opaque;
//     
//         return slirp;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2508 {
// Rust rewrite of slirp_init() from slirp/slirp.c

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::sync::atomic::{AtomicI32, Ordering};

/* ---- mirrored C types (exact field order / types) ---- */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timeval {
    pub tv_sec: i64,  /* __time_t = long int */
    pub tv_usec: i64, /* __suseconds_t = long int */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpHdr {
    /* u_int ip_hl:4, ip_v:4; */
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MbufPtr {
    pub mptr: *mut Mbuf,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ipvly {
    pub ih_mbuf: MbufPtr,
    pub ih_x1: u8,
    pub ih_pr: u8,
    pub ih_len: u16,
    pub ih_src: InAddr,
    pub ih_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TcpHdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32, /* tcp_seq */
    pub th_ack: u32,
    /* u_int th_x2:4, th_off:4; */
    pub th_x2_off: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpiphdr {
    pub ti_i: Ipvly,
    pub ti_t: TcpHdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tcpcb {
    pub seg_next: *mut Tcpiphdr,
    pub seg_prev: *mut Tcpiphdr,
    pub t_state: i16,      /* short */
    pub t_timer: [i16; 4], /* short[4] */
    pub t_rxtshift: i16,
    pub t_rxtcur: i16,
    pub t_dupacks: i16,
    pub t_maxseg: u16, /* u_short */
    pub t_force: c_char,
    pub t_flags: u16, /* u_short */
    pub t_template: Tcpiphdr,
    pub t_socket: *mut Socket,
    pub snd_una: u32,
    pub snd_nxt: u32,
    pub snd_up: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub iss: u32,
    pub snd_wnd: u32,
    pub rcv_wnd: u32,
    pub rcv_nxt: u32,
    pub rcv_up: u32,
    pub irs: u32,
    pub rcv_adv: u32,
    pub snd_max: u32,
    pub snd_cwnd: u32,
    pub snd_ssthresh: u32,
    pub t_idle: i16,
    pub t_rtt: i16,
    pub t_rtseq: u32,
    pub t_srtt: i16,
    pub t_rttvar: i16,
    pub t_rttmin: u16,
    pub max_sndwnd: u32,
    pub t_oobflags: c_char,
    pub t_iobc: c_char,
    pub t_softerror: i16,
    pub snd_scale: u8, /* u_char */
    pub rcv_scale: u8,
    pub request_r_scale: u8,
    pub requested_s_scale: u8,
    pub ts_recent: u32,
    pub ts_recent_age: u32,
    pub last_ack_sent: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpQ {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct MHeader {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char, /* caddr_t */
    pub mh_len: c_int,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHeader,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sbuf {
    pub sb_cc: u32,        /* u_int */
    pub sb_datalen: u32,   /* u_int */
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut Tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8, /* u_char */
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: u32, /* u_int */
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,

    pub client_ethaddr: [u8; 6],

    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],

    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut ExList,

    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,

    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,

    pub ipq: IpQ,
    pub ip_id: u16,

    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,

    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32, /* tcp_seq */
    pub tcp_now: u32,

    pub udb: Socket,
    pub udp_last_so: *mut Socket,

    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],

    pub opaque: *mut c_void,
}

/* ---- C symbols used by this function ---- */

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn pstrcpy(buf: *mut c_char, buf_size: c_int, str_: *const c_char);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn if_init(slirp: *mut Slirp);
    fn ip_init(slirp: *mut Slirp);
    fn m_init(slirp: *mut Slirp);

    static mut loopback_addr: InAddr;
}

/* network byte order conversion (matches libc htonl, folded at compile time) */
fn htonl(v: u32) -> u32 {
    if cfg!(target_endian = "big") {
        v
    } else {
        v.swap_bytes()
    }
}

/* ported body of the C static slirp_init_once() */
unsafe fn slirp_init_once() {
    static INITIALIZED: AtomicI32 = AtomicI32::new(0);

    if INITIALIZED.load(Ordering::Relaxed) != 0 {
        return;
    }
    INITIALIZED.store(1, Ordering::Relaxed);

    let addr: *mut InAddr = core::ptr::addr_of_mut!(loopback_addr);
    (*addr).s_addr = htonl(0x7f00_0001u32);
}

#[no_mangle]
pub extern "C" fn slirp_init(
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
) -> *mut Slirp {
    unsafe {
        let slirp = mallocz(core::mem::size_of::<Slirp>()) as *mut Slirp;

        slirp_init_once();

        (*slirp).restricted = restricted;

        if_init(slirp);
        ip_init(slirp);

        /* Initialise mbufs *after* setting the MTU */
        m_init(slirp);

        (*slirp).vnetwork_addr = vnetwork;
        (*slirp).vnetwork_mask = vnetmask;
        (*slirp).vhost_addr = vhost;
        if !vhostname.is_null() {
            pstrcpy(
                core::ptr::addr_of_mut!((*slirp).client_hostname) as *mut c_char,
                core::mem::size_of::<[c_char; 33]>() as c_int,
                vhostname,
            );
        }
        if !tftp_path.is_null() {
            (*slirp).tftp_prefix = strdup(tftp_path);
        }
        if !bootfile.is_null() {
            (*slirp).bootp_filename = strdup(bootfile);
        }
        (*slirp).vdhcp_startaddr = vdhcp_start;
        (*slirp).vnameserver_addr = vnameserver;

        (*slirp).opaque = opaque;

        slirp
    }
}
}

// ── `Slirp *slirp = mallocz(sizeof(Slirp)); slirp_init_once(); slirp->restricted = restricted; if_in…` · slirp/slirp.c:254-268 · chunk 2509 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     Slirp *slirp = mallocz(sizeof(Slirp));
//     
//         slirp_init_once();
//     
//         slirp->restricted = restricted;
//     
//         if_init(slirp);
//         ip_init(slirp);
//     
//     
//         m_init(slirp);
//     
//         slirp->vnetwork_addr = vnetwork;
//         slirp->vnetwork_mask = vnetmask;
//         slirp->vhost_addr = vhost;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2509 {
// Portion of slirp_init: allocate the Slirp state, run the one-time init,
// set the basic fields and initialise the mbuf / if / ip subsystems.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// Leading part of `struct Slirp` (everything up to and including
// `restricted`), mirrored field for field so the offsets match the C layout.
#[repr(C)]
pub struct SlirpPrefix {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn if_init(slirp: *mut c_void);
    fn ip_init(slirp: *mut c_void);
    fn m_init(slirp: *mut c_void);
    static mut loopback_addr: InAddr;
}

static mut INITIALIZED: c_int = 0;

#[no_mangle]
pub extern "C" fn slirp_init__part2509(
    restricted: c_int,
    vnetwork: InAddr,
    vnetmask: InAddr,
    vhost: InAddr,
    slirp_size: usize,
) -> *mut c_void {
    unsafe {
        let slirp: *mut SlirpPrefix = mallocz(slirp_size) as *mut SlirpPrefix;

        // slirp_init_once(): C static function, logic ported here.
        let initialized = core::ptr::addr_of_mut!(INITIALIZED);
        if *initialized == 0 {
            *initialized = 1;
            let lp = core::ptr::addr_of_mut!(loopback_addr);
            // loopback_addr.s_addr = htonl(INADDR_LOOPBACK)
            (*lp).s_addr = 0x7f00_0001u32.to_be();
        }

        (*slirp).restricted = restricted;

        let raw = slirp as *mut c_void;
        if_init(raw);
        ip_init(raw);

        /* Initialise mbufs *after* setting the MTU */
        m_init(raw);

        (*slirp).vnetwork_addr = vnetwork;
        (*slirp).vnetwork_mask = vnetmask;
        (*slirp).vhost_addr = vhost;

        raw
    }
}
}

// ── `if (vhostname)` · slirp/slirp.c:269-272 · chunk 2510 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (vhostname) {
//             pstrcpy(slirp->client_hostname, sizeof(slirp->client_hostname),
//                     vhostname);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2510 {
// Port of the `if (vhostname) { pstrcpy(...); }` piece of slirp_init.
use core::ffi::{c_char, c_int};

extern "C" {
    // Non-static C function with a linker symbol in the program.
    fn pstrcpy(buf: *mut c_char, buf_size: c_int, str: *const c_char);
}

#[no_mangle]
pub extern "C" fn slirp_init__part2510(buf: *mut c_char, buf_size: c_int, str: *const c_char) {
    if !str.is_null() {
        unsafe {
            pstrcpy(buf, buf_size, str);
        }
    }
}
}

// ── `if (tftp_path)` · slirp/slirp.c:273-275 · chunk 2511 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (tftp_path) {
//             slirp->tftp_prefix = strdup(tftp_path);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2511 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn slirp_init__part2511(
    tftp_prefix: *mut *mut c_char,
    tftp_path: *const c_char,
) {
    // int dummy = 0; // silence unused import warnings is not needed for c_int
    let _ = core::mem::size_of::<c_int>();
    unsafe {
        if !tftp_path.is_null() && !tftp_prefix.is_null() {
            *tftp_prefix = strdup(tftp_path);
        }
    }
}
}

// ── `if (bootfile)` · slirp/slirp.c:276-278 · chunk 2512 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bootfile) {
//             slirp->bootp_filename = strdup(bootfile);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2512 {
use core::ffi::c_char;

extern "C" {
    fn strdup(s: *const c_char) -> *mut c_char;
}

#[no_mangle]
pub extern "C" fn slirp_init__part2512(slot: *mut *mut c_char, bootfile: *const c_char) {
    if !bootfile.is_null() && !slot.is_null() {
        unsafe {
            *slot = strdup(bootfile);
        }
    }
}
}

// ── `slirp->vdhcp_startaddr = vdhcp_start; slirp->vnameserver_addr = vnameserver; slirp->opaque = op…` · slirp/slirp.c:279-284 · chunk 2513 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     slirp->vdhcp_startaddr = vdhcp_start;
//         slirp->vnameserver_addr = vnameserver;
//     
//         slirp->opaque = opaque;
//     
//         return slirp;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2513 {
// Rust port of the tail of slirp_init(): store the DHCP/nameserver addresses
// and the opaque user pointer into the Slirp structure.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C)]
pub struct IpQ {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct MbufData {
    pub m_dat_: [c_char; 1],
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MbufData,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct TcpCb {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut MBuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut TcpCb,
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut ExList,
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn slirp_init__part2513(
    slirp: *mut Slirp,
    vdhcp_start: InAddr,
    vnameserver: InAddr,
    opaque: *mut c_void,
) -> *mut Slirp {
    (*slirp).vdhcp_startaddr = vdhcp_start;
    (*slirp).vnameserver_addr = vnameserver;
    (*slirp).opaque = opaque;
    slirp
}
}

// ── `slirp_cleanup` · slirp/slirp.c:287-292 · chunk 2514 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void slirp_cleanup(Slirp *slirp)
//     {
//         free(slirp->tftp_prefix);
//         free(slirp->bootp_filename);
//         free(slirp);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2514 {
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct MbufDat {
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MbufDat,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut ExList,
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn slirp_cleanup(slirp: *mut Slirp) {
    unsafe {
        if slirp.is_null() {
            return;
        }
        free((*slirp).tftp_prefix as *mut c_void);
        free((*slirp).bootp_filename as *mut c_void);
        free(slirp as *mut c_void);
    }
}
}

// ── `free(slirp->tftp_prefix); free(slirp->bootp_filename); free(slirp);` · slirp/slirp.c:289-291 · chunk 2515 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(slirp->tftp_prefix);
//         free(slirp->bootp_filename);
//         free(slirp);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2515 {
// Port of the tail of slirp_cleanup: free the tftp_prefix, bootp_filename and slirp itself.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long_t,
    pub tv_usec: c_long_t,
}
type c_long_t = core::ffi::c_long;

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut c_void,
    pub mh_prev: *mut c_void,
    pub mh_nextpkt: *mut c_void,
    pub mh_prevpkt: *mut c_void,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut c_void,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MbufDat {
    pub m_dat_: [u8; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut c_void,
    pub m_dat: MbufDat,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut c_void,
    pub so_prev: *mut c_void,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut c_void,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void,
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut c_void,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut c_void,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut c_void,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

#[no_mangle]
pub extern "C" fn slirp_cleanup__part2515(slirp: *mut Slirp) {
    unsafe {
        free((*slirp).tftp_prefix as *mut c_void);
        free((*slirp).bootp_filename as *mut c_void);
        free(slirp as *mut c_void);
    }
}
}

// ── `slirp_select_fill` · slirp/slirp.c:298-414 · chunk 2516 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void slirp_select_fill(Slirp *slirp, int *pnfds,
//                            fd_set *readfds, fd_set *writefds, fd_set *xfds)
//     {
//         struct socket *so, *so_next;
//         int nfds;
//     
//     
//         global_readfds = 
//                         ((void *)0)
//                             ;
//         global_writefds = 
//                          ((void *)0)
//                              ;
//         global_xfds = 
//                      ((void *)0)
//                          ;
//     
//         nfds = *pnfds;
//     
//     
//     
//      do_slowtimo = 0;
//     
//      {
//     
//     
//     
//     
//       do_slowtimo |= ((slirp->tcb.so_next != &slirp->tcb) ||
//           (&slirp->ipq.ip_link != slirp->ipq.ip_link.next));
//     
//       for (so = slirp->tcb.so_next; so != &slirp->tcb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//        if (time_fasttimo == 0 && so->so_tcpcb->t_flags & 0x0002)
//           time_fasttimo = curtime;
//     
//     
//     
//     
//     
//        if (so->so_state & 0x001 || so->s == -1)
//           continue;
//     
//     
//     
//     
//        if (so->so_state & 0x100) {
//                                     
//                                    ((void) (((
//                                    readfds
//                                    )->fds_bits)[((
//                                    so->s
//                                    ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//                                    so->s
//                                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                                          ;
//         if (nfds < (so->s)) nfds = (so->s);
//         continue;
//        }
//     
//     
//     
//     
//        if (so->so_state & 0x002) {
//         
//        ((void) (((
//        writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                               ;
//         if (nfds < (so->s)) nfds = (so->s);
//         continue;
//        }
//     
//     
//     
//     
//     
//        if ((((so)->so_state & (0x010|0x004)) == 0x004) && so->so_rcv.sb_cc) {
//         
//        ((void) (((
//        writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                               ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//     
//     
//     
//     
//     
//        if ((((so)->so_state & (0x008|0x004)) == 0x004) && (so->so_snd.sb_cc < (so->so_snd.sb_datalen/2))) {
//         
//        ((void) (((
//        readfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                              ;
//         
//        ((void) (((
//        xfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                           ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//       }
//     
//     
//     
//     
//       for (so = slirp->udb.so_next; so != &slirp->udb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//        if (so->so_expire) {
//         if (so->so_expire <= curtime) {
//          udp_detach(so);
//          continue;
//         } else
//          do_slowtimo = 1;
//        }
//        if ((so->so_state & 0x004) && so->so_queued <= 4) {
//         
//        ((void) (((
//        readfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                              ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//       }
//      }
//     
//             *pnfds = nfds;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2516 {
// Port of slirp_select_fill() from slirp/slirp.c
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ushort, c_void};

// ---------------------------------------------------------------- types ----

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ipq (packed in C)
#[repr(C, packed)]
pub struct Ipq {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    // union M_dat { char m_dat_[1]; char *m_ext_; }  (size/alignment of a pointer)
    pub m_ext: *mut c_char,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Prefix of struct tcpcb, up to and including t_flags (all that is accessed).
#[repr(C)]
pub struct Tcpcb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void, // struct ex_list *
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

// fd_set of the C library: __fd_mask == long int, NBITS == 1024
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * core::mem::size_of::<c_long>())],
}

// ----------------------------------------------------------- C externs ----

extern "C" {
    static mut curtime: c_uint;
    static mut global_readfds: *mut FdSet;
    static mut global_writefds: *mut FdSet;
    static mut global_xfds: *mut FdSet;
    fn udp_detach(so: *mut Socket);
}

// The C originals of these are file-scope statics of slirp.c and therefore
// have no linkage; keep the same state locally.
static mut TIME_FASTTIMO: c_uint = 0;
static mut DO_SLOWTIMO: c_int = 0;

// ------------------------------------------------------------- helpers ----

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_ISFCONNECTED: c_int = 0x004;
const SS_FCANTRCVMORE: c_int = 0x008;
const SS_FCANTSENDMORE: c_int = 0x010;
const SS_FACCEPTCONN: c_int = 0x100;
const TF_DELACK: c_ushort = 0x0002;

// FD_SET(fd, set)
#[inline]
unsafe fn fd_set_bit(set: *mut FdSet, fd: c_int) {
    let per: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
    let idx: isize = (fd / per) as isize;
    let sh: u32 = (fd % per) as u32;
    let base: *mut c_long = core::ptr::addr_of_mut!((*set).fds_bits) as *mut c_long;
    let slot: *mut c_long = base.offset(idx);
    let mask: c_long = (1 as c_long).wrapping_shl(sh);
    *slot |= mask;
}

// UPD_NFDS(x)
#[inline]
fn upd_nfds(nfds: &mut c_int, fd: c_int) {
    if *nfds < fd {
        *nfds = fd;
    }
}

// --------------------------------------------------------------- target ----

#[no_mangle]
pub extern "C" fn slirp_select_fill(
    slirp: *mut Slirp,
    pnfds: *mut c_int,
    readfds: *mut FdSet,
    writefds: *mut FdSet,
    xfds: *mut FdSet,
) {
    unsafe {
        // fail safe
        global_readfds = core::ptr::null_mut();
        global_writefds = core::ptr::null_mut();
        global_xfds = core::ptr::null_mut();

        let mut nfds: c_int = *pnfds;

        // First, TCP sockets
        DO_SLOWTIMO = 0;

        let tcb: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);
        {
            // *_slowtimo needs calling if there are IP fragments in the
            // fragment queue, or there are TCP connections active
            let ip_link: *const c_void =
                core::ptr::addr_of!((*slirp).ipq.ip_link) as *const c_void;
            let ip_next: *const c_void = core::ptr::read_unaligned(core::ptr::addr_of!(
                (*slirp).ipq.ip_link.next
            )) as *const c_void;
            if ((*tcb).so_next as *const c_void != tcb as *const c_void) || ip_link != ip_next {
                DO_SLOWTIMO = 1;
            }
        }

        let mut so: *mut Socket = (*tcb).so_next;
        while so != tcb {
            let so_next: *mut Socket = (*so).so_next;

            // See if we need a tcp_fasttimo
            let tp: *mut Tcpcb = (*so).so_tcpcb;
            if TIME_FASTTIMO == 0 && ((*tp).t_flags & TF_DELACK) != 0 {
                TIME_FASTTIMO = curtime;
            }

            let state: c_int = (*so).so_state;
            let s: c_int = (*so).s;

            // NOFDREF can include still connecting to local-host,
            // newly socreated() sockets etc. Don't want to select these.
            if (state & SS_NOFDREF) != 0 || s == -1 {
                so = so_next;
                continue;
            }

            // Set for reading sockets which are accepting
            if (state & SS_FACCEPTCONN) != 0 {
                fd_set_bit(readfds, s);
                upd_nfds(&mut nfds, s);
                so = so_next;
                continue;
            }

            // Set for writing sockets which are connecting
            if (state & SS_ISFCONNECTING) != 0 {
                fd_set_bit(writefds, s);
                upd_nfds(&mut nfds, s);
                so = so_next;
                continue;
            }

            // Set for writing if we are connected, can send more, and we
            // have something to send
            if (state & (SS_FCANTSENDMORE | SS_ISFCONNECTED)) == SS_ISFCONNECTED
                && (*so).so_rcv.sb_cc != 0
            {
                fd_set_bit(writefds, s);
                upd_nfds(&mut nfds, s);
            }

            // Set for reading (and urgent data) if we are connected, can
            // receive more, and we have room for it XXX /2 ?
            if (state & (SS_FCANTRCVMORE | SS_ISFCONNECTED)) == SS_ISFCONNECTED
                && (*so).so_snd.sb_cc < (*so).so_snd.sb_datalen / 2
            {
                fd_set_bit(readfds, s);
                fd_set_bit(xfds, s);
                upd_nfds(&mut nfds, s);
            }

            so = so_next;
        }

        // UDP sockets
        let udb: *mut Socket = core::ptr::addr_of_mut!((*slirp).udb);
        so = (*udb).so_next;
        while so != udb {
            let so_next: *mut Socket = (*so).so_next;

            // See if it's timed out
            if (*so).so_expire != 0 {
                if (*so).so_expire <= curtime {
                    udp_detach(so);
                    so = so_next;
                    continue;
                } else {
                    DO_SLOWTIMO = 1; // Let socket expire
                }
            }

            if ((*so).so_state & SS_ISFCONNECTED) != 0 && (*so).so_queued <= 4 {
                fd_set_bit(readfds, (*so).s);
                upd_nfds(&mut nfds, (*so).s);
            }

            so = so_next;
        }

        *pnfds = nfds;
    }
}
}

// ── `struct socket *so, *so_next; int nfds; global_readfds = ((void *)0) ; global_writefds = ((void…` · slirp/slirp.c:301-313 · chunk 2517 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *so, *so_next;
//         int nfds;
//     
//     
//         global_readfds = 
//                         ((void *)0)
//                             ;
//         global_writefds = 
//                          ((void *)0)
//                              ;
//         global_xfds = 
//                      ((void *)0)
//                          ;
//     
//         nfds = *pnfds;
//     
//     
//     
//      do_slowtimo = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2517 {
// Port of slirp_select_fill lines 301-313: clear the global fd_set pointers
// and initialise nfds from *pnfds.
use core::ffi::{c_int, c_void};

extern "C" {
    // fd_set *global_readfds, *global_writefds, *global_xfds; (non-static, exported)
    static mut global_readfds: *mut c_void;
    static mut global_writefds: *mut c_void;
    static mut global_xfds: *mut c_void;
}

#[no_mangle]
pub extern "C" fn slirp_select_fill__part2517(pnfds: c_int) -> c_int {
    unsafe {
        global_readfds = core::ptr::null_mut();
        global_writefds = core::ptr::null_mut();
        global_xfds = core::ptr::null_mut();
    }
    // nfds = *pnfds;  (returned to the caller, which stores it in its local)
    pnfds
}
}

// ── `{ … }` · slirp/slirp.c:315-411 · chunk 2518 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//     
//     
//     
//     
//       do_slowtimo |= ((slirp->tcb.so_next != &slirp->tcb) ||
//           (&slirp->ipq.ip_link != slirp->ipq.ip_link.next));
//     
//       for (so = slirp->tcb.so_next; so != &slirp->tcb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//        if (time_fasttimo == 0 && so->so_tcpcb->t_flags & 0x0002)
//           time_fasttimo = curtime;
//     
//     
//     
//     
//     
//        if (so->so_state & 0x001 || so->s == -1)
//           continue;
//     
//     
//     
//     
//        if (so->so_state & 0x100) {
//                                     
//                                    ((void) (((
//                                    readfds
//                                    )->fds_bits)[((
//                                    so->s
//                                    ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//                                    so->s
//                                    ) % (8 * (int) sizeof (__fd_mask)))))))
//                                                          ;
//         if (nfds < (so->s)) nfds = (so->s);
//         continue;
//        }
//     
//     
//     
//     
//        if (so->so_state & 0x002) {
//         
//        ((void) (((
//        writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                               ;
//         if (nfds < (so->s)) nfds = (so->s);
//         continue;
//        }
//     
//     
//     
//     
//     
//        if ((((so)->so_state & (0x010|0x004)) == 0x004) && so->so_rcv.sb_cc) {
//         
//        ((void) (((
//        writefds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                               ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//     
//     
//     
//     
//     
//        if ((((so)->so_state & (0x008|0x004)) == 0x004) && (so->so_snd.sb_cc < (so->so_snd.sb_datalen/2))) {
//         
//        ((void) (((
//        readfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                              ;
//         
//        ((void) (((
//        xfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                           ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//       }
//     
//     
//     
//     
//       for (so = slirp->udb.so_next; so != &slirp->udb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//        if (so->so_expire) {
//         if (so->so_expire <= curtime) {
//          udp_detach(so);
//          continue;
//         } else
//          do_slowtimo = 1;
//        }
//        if ((so->so_state & 0x004) && so->so_queued <= 4) {
//         
//        ((void) (((
//        readfds
//        )->fds_bits)[((
//        so->s
//        ) / (8 * (int) sizeof (__fd_mask)))] |= ((__fd_mask) (1UL << ((
//        so->s
//        ) % (8 * (int) sizeof (__fd_mask)))))))
//                              ;
//         if (nfds < (so->s)) nfds = (so->s);
//        }
//       }
//      }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2518 {
// Port of the body of slirp_select_fill() (slirp/slirp.c lines 315-411).
//
// The C structs are mirrored faithfully because the code walks intrusive
// linked lists and compares node addresses with list heads embedded in
// struct Slirp (struct ipq is __attribute__((packed)) in C, so it is packed
// here too, which keeps every following offset of struct Slirp identical).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ushort, c_void};

// (8 * (int) sizeof(__fd_mask)) from the FD_SET/FD_ISSET macros
const FD_WORD_BITS: c_int = 8 * core::mem::size_of::<c_long>() as c_int;
const FD_SET_WORDS: usize = 1024 / (8 * core::mem::size_of::<c_long>());

// offsetof(struct ipq, ip_link) == sizeof(struct qlink) == 2 pointers
const IP_LINK_OFFSET: usize = 2 * core::mem::size_of::<*mut c_void>();

#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; FD_SET_WORDS],
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

// Prefix of struct tcpcb: only up to t_flags is ever touched here.
#[repr(C)]
pub struct TcpCb {
    pub seg_next: *mut c_void,
    pub seg_prev: *mut c_void,
    pub t_state: c_short,
    pub t_timer: [c_short; 4],
    pub t_rxtshift: c_short,
    pub t_rxtcur: c_short,
    pub t_dupacks: c_short,
    pub t_maxseg: c_ushort,
    pub t_force: c_char,
    pub t_flags: c_ushort,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut MBuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: c_char,
    pub so_state: c_int,
    pub so_tcpcb: *mut TcpCb,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut c_void,
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn udp_detach(so: *mut Socket);
    static mut curtime: c_uint;
}

// ((fd_set *)fds)->fds_bits[fd / (8 * sizeof(__fd_mask))] |= (__fd_mask)(1UL << (fd % (8 * sizeof(__fd_mask))))
unsafe fn fd_set_bit(fds: *mut FdSet, fd: c_int) {
    let idx = fd / FD_WORD_BITS;
    let bit = fd % FD_WORD_BITS;
    let word = (fds as *mut u8)
        .wrapping_offset((idx as isize) * core::mem::size_of::<c_long>() as isize)
        as *mut c_long;
    let mask: c_long = (1u64 << ((bit as u32) & (FD_WORD_BITS as u32 - 1))) as c_long;
    *word |= mask;
}

#[no_mangle]
pub unsafe extern "C" fn slirp_select_fill__part2518(
    slirp: *mut Slirp,
    nfds: *mut c_int,
    readfds: *mut FdSet,
    writefds: *mut FdSet,
    xfds: *mut FdSet,
    do_slowtimo: *mut c_int,
    time_fasttimo: *mut c_uint,
) -> c_uint {
    let mut nfds_v: c_int = *nfds;

    let tcb: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);
    let udb: *mut Socket = core::ptr::addr_of_mut!((*slirp).udb);
    let ipq: *mut IpQ = core::ptr::addr_of_mut!((*slirp).ipq);

    // &slirp->ipq.ip_link  and  slirp->ipq.ip_link.next
    let ip_link_addr: *const c_void = (ipq as *const u8).add(IP_LINK_OFFSET) as *const c_void;
    let ip_link_next: *mut c_void = core::ptr::read_unaligned(ip_link_addr as *const *mut c_void);

    let mut slow: c_int = 0;
    if (*tcb).so_next != tcb || ip_link_addr != ip_link_next as *const c_void {
        slow = 1;
    }
    *do_slowtimo |= slow;

    // ---- TCP socket list -------------------------------------------------
    let mut so: *mut Socket = (*tcb).so_next;
    while so != tcb {
        let so_next: *mut Socket = (*so).so_next;

        if *time_fasttimo == 0 && ((*(*so).so_tcpcb).t_flags & 0x0002) != 0 {
            *time_fasttimo = curtime;
        }

        if ((*so).so_state & 0x001) != 0 || (*so).s == -1 {
            so = so_next;
            continue;
        }

        if ((*so).so_state & 0x100) != 0 {
            fd_set_bit(readfds, (*so).s);
            if nfds_v < (*so).s {
                nfds_v = (*so).s;
            }
            so = so_next;
            continue;
        }

        if ((*so).so_state & 0x002) != 0 {
            fd_set_bit(writefds, (*so).s);
            if nfds_v < (*so).s {
                nfds_v = (*so).s;
            }
            so = so_next;
            continue;
        }

        if ((*so).so_state & (0x010 | 0x004)) == 0x004 && (*so).so_rcv.sb_cc != 0 {
            fd_set_bit(writefds, (*so).s);
            if nfds_v < (*so).s {
                nfds_v = (*so).s;
            }
        }

        if ((*so).so_state & (0x008 | 0x004)) == 0x004
            && (*so).so_snd.sb_cc < (*so).so_snd.sb_datalen / 2
        {
            fd_set_bit(readfds, (*so).s);
            fd_set_bit(xfds, (*so).s);
            if nfds_v < (*so).s {
                nfds_v = (*so).s;
            }
        }

        so = so_next;
    }

    // ---- UDP socket list -------------------------------------------------
    let mut so: *mut Socket = (*udb).so_next;
    while so != udb {
        let so_next: *mut Socket = (*so).so_next;

        if (*so).so_expire != 0 {
            if (*so).so_expire <= curtime {
                udp_detach(so);
                so = so_next;
                continue;
            } else {
                *do_slowtimo = 1;
            }
        }

        if ((*so).so_state & 0x004) != 0 && (*so).so_queued <= 4 {
            fd_set_bit(readfds, (*so).s);
            if nfds_v < (*so).s {
                nfds_v = (*so).s;
            }
        }

        so = so_next;
    }

    *nfds = nfds_v;
    0
}
}

// ── `*pnfds = nfds;` · slirp/slirp.c:413-413 · chunk 2519 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *pnfds = nfds;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2519 {
// Stores the final nfds value back through the caller's pointer.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn slirp_select_fill__part2519(pnfds: *mut c_int, nfds: c_int) {
    unsafe {
        *pnfds = nfds;
    }
}
}

// ── `slirp_select_poll` · slirp/slirp.c:416-594 · chunk 2520 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void slirp_select_poll(Slirp *slirp,
//                            fd_set *readfds, fd_set *writefds, fd_set *xfds,
//                            int select_error)
//     {
//         struct socket *so, *so_next;
//         int ret;
//     
//         global_readfds = readfds;
//         global_writefds = writefds;
//         global_xfds = xfds;
//     
//         curtime = os_get_time_ms();
//     
//         {
//     
//     
//     
//       if (time_fasttimo && ((curtime - time_fasttimo) >= 2)) {
//        tcp_fasttimo(slirp);
//        time_fasttimo = 0;
//       }
//       if (do_slowtimo && ((curtime - last_slowtimo) >= 499)) {
//        ip_slowtimo(slirp);
//        tcp_slowtimo(slirp);
//        last_slowtimo = curtime;
//       }
//     
//     
//     
//     
//      if (!select_error) {
//     
//     
//     
//       for (so = slirp->tcb.so_next; so != &slirp->tcb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//     
//        if (so->so_state & 0x001 || so->s == -1)
//           continue;
//     
//     
//     
//     
//     
//     
//        if (
//           ((((
//           xfds
//           )->fds_bits)[((
//           so->s
//           ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//           so->s
//           ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                )
//           sorecvoob(so);
//     
//     
//     
//        else if (
//                ((((
//                readfds
//                )->fds_bits)[((
//                so->s
//                ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                so->s
//                ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                        ) {
//     
//     
//     
//         if (so->so_state & 0x100) {
//          tcp_connect(so);
//          continue;
//         }
//         ret = soread(so);
//     
//     
//         if (ret > 0)
//            tcp_output(((so)->so_tcpcb));
//        }
//     
//     
//     
//     
//        if (
//           ((((
//           writefds
//           )->fds_bits)[((
//           so->s
//           ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//           so->s
//           ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                    ) {
//     
//     
//     
//          if (so->so_state & 0x002) {
//     
//            so->so_state &= ~0x002;
//     
//            ret = send(so->s, (const void *) &ret, 0, 0);
//            if (ret < 0) {
//     
//              if (
//                 (*__errno_location ()) 
//                       == 
//                          11 
//                                 || 
//                                    (*__errno_location ()) 
//                                          == 
//                                             11 
//                                                         ||
//           
//          (*__errno_location ()) 
//                == 
//                   115 
//                               || 
//                                  (*__errno_location ()) 
//                                        == 
//                                           107
//                                                   )
//         continue;
//     
//     
//              so->so_state &= 0xf000;
//              so->so_state |= 0x001;
//            }
//     
//     
//     
//     
//     
//            tcp_input((struct mbuf *)
//                                    ((void *)0)
//                                        , sizeof(struct ip), so);
//     
//          } else
//            ret = sowrite(so);
//     
//     
//     
//     
//     
//     
//        }
//       }
//     
//     
//     
//     
//     
//     
//       for (so = slirp->udb.so_next; so != &slirp->udb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//        if (so->s != -1 && 
//                          ((((
//                          readfds
//                          )->fds_bits)[((
//                          so->s
//                          ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                          so->s
//                          ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                                  ) {
//                                 sorecvfrom(so);
//                             }
//       }
//      }
//     
//     
//     
//     
//      if (slirp->if_queued) {
//          if_start(slirp);
//      }
//         }
//     
//     
//     
//     
//     
//     
//       global_readfds = 
//                       ((void *)0)
//                           ;
//       global_writefds = 
//                        ((void *)0)
//                            ;
//       global_xfds = 
//                    ((void *)0)
//                        ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2520 {
// Rust port of slirp_select_poll() from slirp/slirp.c
//
// Notes on the port:
//  * fd_set is a glibc fd_set: struct { long int fds_bits[1024 / (8*sizeof(long))] }
//  * struct socket / struct Slirp are mirrored with #[repr(C)] and the exact
//    C field order/types so that &slirp->tcb and &slirp->udb compare equal to
//    the list-head addresses produced by the C side.
//  * time_fasttimo / last_slowtimo / do_slowtimo are file-static in slirp.c and
//    therefore have no linker symbol; they are mirrored here as private statics.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const SS_NOFDREF: c_int = 0x001;
const SS_ISFCONNECTING: c_int = 0x002;
const SS_FACCEPTCONN: c_int = 0x100;
const SS_PERSISTENT_MASK: c_int = 0xf000;

const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = 11;
const EINPROGRESS: c_int = 115;
const ENOTCONN: c_int = 107;

// sizeof(struct ip) for the (packed) ip header used by slirp
const SIZEOF_STRUCT_IP: c_int = 20;

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub _pad0: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub _pad1: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

// glibc fd_set
#[repr(C)]
pub struct FdSet {
    pub fds_bits: [c_long; 1024 / (8 * 8)],
}

// Opaque mirrors of the remaining Slirp fields (layout only, never accessed
// through these types other than the fields named in Slirp below).
#[repr(C, align(8))]
struct Mbuf {
    _opaque: [u8; 80],
}

#[repr(C, align(1))]
struct IpQ {
    _opaque: [u8; 44],
}

#[repr(C, align(2))]
struct BootpClient {
    _opaque: [u8; 8],
}

#[repr(C, align(8))]
struct TftpSession {
    _opaque: [u8; 32],
}

#[repr(C)]
pub struct Slirp {
    // vnetwork_addr .. tt, exec_list
    _pre1: [u8; 96],
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    // globals defined in slirp.c
    static mut curtime: c_uint;
    static mut global_readfds: *mut FdSet;
    static mut global_writefds: *mut FdSet;
    static mut global_xfds: *mut FdSet;

    fn os_get_time_ms() -> u32;

    fn tcp_fasttimo(slirp: *mut Slirp);
    fn tcp_slowtimo(slirp: *mut Slirp);
    fn ip_slowtimo(slirp: *mut Slirp);

    fn sorecvoob(so: *mut Socket);
    fn soread(so: *mut Socket) -> c_int;
    fn sowrite(so: *mut Socket) -> c_int;
    fn sorecvfrom(so: *mut Socket);
    fn tcp_connect(so: *mut Socket);
    fn tcp_output(tp: *mut c_void) -> c_int;
    fn tcp_input(m: *mut c_void, iphlen: c_int, so: *mut Socket);
    fn if_start(slirp: *mut Slirp);

    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn __errno_location() -> *mut c_int;
}

// file-static state of slirp.c (no linker symbol is available for those)
static mut TIME_FASTTIMO: c_uint = 0;
static mut LAST_SLOWTIMO: c_uint = 0;
static mut DO_SLOWTIMO: c_int = 0;

// FD_ISSET(fd, set)
#[inline]
unsafe fn fd_isset(set: *mut FdSet, fd: c_int) -> bool {
    let bits_per_word: c_int = (8 * core::mem::size_of::<c_long>()) as c_int;
    let idx = (fd / bits_per_word) as isize;
    let bit = fd % bits_per_word;
    let base: *mut c_long = core::ptr::addr_of_mut!((*set).fds_bits) as *mut c_long;
    let word: c_long = *base.offset(idx);
    (word & (1 as c_long).wrapping_shl(bit as u32)) != 0
}

#[no_mangle]
pub unsafe extern "C" fn slirp_select_poll(
    slirp: *mut Slirp,
    readfds: *mut FdSet,
    writefds: *mut FdSet,
    xfds: *mut FdSet,
    select_error: c_int,
) {
    global_readfds = readfds;
    global_writefds = writefds;
    global_xfds = xfds;

    curtime = os_get_time_ms();

    /*
     * See if anything has timed out
     */
    if TIME_FASTTIMO != 0 && curtime.wrapping_sub(TIME_FASTTIMO) >= 2 {
        tcp_fasttimo(slirp);
        TIME_FASTTIMO = 0;
    }
    if DO_SLOWTIMO != 0 && curtime.wrapping_sub(LAST_SLOWTIMO) >= 499 {
        ip_slowtimo(slirp);
        tcp_slowtimo(slirp);
        LAST_SLOWTIMO = curtime;
    }

    /*
     * Check sockets
     */
    if select_error == 0 {
        /*
         * Check TCP sockets
         */
        let tcb: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);
        let mut so: *mut Socket = (*tcb).so_next;
        while so != tcb {
            let so_next: *mut Socket = (*so).so_next;

            'next: {
                /*
                 * FD_ISSET is meaningless on these sockets
                 * (and they can crash the program)
                 */
                if ((*so).so_state & SS_NOFDREF) != 0 || (*so).s == -1 {
                    break 'next;
                }

                /*
                 * Check for URG data
                 * This will soread as well, so no need to
                 * test for readfds below if this succeeds
                 */
                if fd_isset(xfds, (*so).s) {
                    sorecvoob(so);
                } else if fd_isset(readfds, (*so).s) {
                    /*
                     * Check for incoming connections
                     */
                    if ((*so).so_state & SS_FACCEPTCONN) != 0 {
                        tcp_connect(so);
                        break 'next;
                    }

                    let ret: c_int = soread(so);

                    /* Output it if we read something */
                    if ret > 0 {
                        tcp_output((*so).so_tcpcb);
                    }
                }

                /*
                 * Check sockets for writing
                 */
                if fd_isset(writefds, (*so).s) {
                    /*
                     * Check for non-blocking, still-connecting sockets
                     */
                    if ((*so).so_state & SS_ISFCONNECTING) != 0 {
                        /* Connected */
                        (*so).so_state &= !SS_ISFCONNECTING;

                        let mut ret: c_int = 0;
                        ret = send(
                            (*so).s,
                            core::ptr::addr_of!(ret) as *const c_void,
                            0,
                            0,
                        ) as c_int;
                        if ret < 0 {
                            /* XXXXX Must fix, zero bytes is a NOP */
                            let err: c_int = *__errno_location();
                            if err == EAGAIN
                                || err == EWOULDBLOCK
                                || err == EINPROGRESS
                                || err == ENOTCONN
                            {
                                break 'next;
                            }

                            /* else failed */
                            (*so).so_state &= SS_PERSISTENT_MASK;
                            (*so).so_state |= SS_NOFDREF;
                        }

                        /*
                         * Continue tcp_input
                         */
                        tcp_input(
                            core::ptr::null_mut(),
                            SIZEOF_STRUCT_IP,
                            so,
                        );
                    } else {
                        sowrite(so);
                    }
                }
            }

            so = so_next;
        }

        /*
         * Now UDP sockets.
         * Incoming packets are sent straight away, they're not buffered.
         * Incoming UDP data isn't buffered either.
         */
        let udb: *mut Socket = core::ptr::addr_of_mut!((*slirp).udb);
        let mut so: *mut Socket = (*udb).so_next;
        while so != udb {
            let so_next: *mut Socket = (*so).so_next;

            if (*so).s != -1 && fd_isset(readfds, (*so).s) {
                sorecvfrom(so);
            }

            so = so_next;
        }
    }

    /*
     * See if we can start outputting
     */
    if (*slirp).if_queued != 0 {
        if_start(slirp);
    }

    /* clear global file descriptor sets.
     * these reside on the stack in vl.c
     * so they're unusable if we're not in
     * slirp_select_fill or slirp_select_poll.
     */
    global_readfds = core::ptr::null_mut();
    global_writefds = core::ptr::null_mut();
    global_xfds = core::ptr::null_mut();
}
}

// ── `struct socket *so, *so_next; int ret; global_readfds = readfds; global_writefds = writefds; glo…` · slirp/slirp.c:420-427 · chunk 2521 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct socket *so, *so_next;
//         int ret;
//     
//         global_readfds = readfds;
//         global_writefds = writefds;
//         global_xfds = xfds;
//     
//         curtime = os_get_time_ms();
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2521 {
use core::ffi::{c_uint, c_void};

extern "C" {
    static mut global_readfds: *mut c_void;
    static mut global_writefds: *mut c_void;
    static mut global_xfds: *mut c_void;
    static mut curtime: c_uint;
    fn os_get_time_ms() -> u32;
}

#[no_mangle]
pub extern "C" fn slirp_select_poll__part2521(
    readfds: *mut c_void,
    writefds: *mut c_void,
    xfds: *mut c_void,
) {
    unsafe {
        global_readfds = readfds;
        global_writefds = writefds;
        global_xfds = xfds;

        curtime = os_get_time_ms() as c_uint;
    }
}
}

// ── `{ … }` · slirp/slirp.c:429-584 · chunk 2522 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     {
//     
//     
//     
//       if (time_fasttimo && ((curtime - time_fasttimo) >= 2)) {
//        tcp_fasttimo(slirp);
//        time_fasttimo = 0;
//       }
//       if (do_slowtimo && ((curtime - last_slowtimo) >= 499)) {
//        ip_slowtimo(slirp);
//        tcp_slowtimo(slirp);
//        last_slowtimo = curtime;
//       }
//     
//     
//     
//     
//      if (!select_error) {
//     
//     
//     
//       for (so = slirp->tcb.so_next; so != &slirp->tcb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//     
//     
//     
//     
//        if (so->so_state & 0x001 || so->s == -1)
//           continue;
//     
//     
//     
//     
//     
//     
//        if (
//           ((((
//           xfds
//           )->fds_bits)[((
//           so->s
//           ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//           so->s
//           ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                )
//           sorecvoob(so);
//     
//     
//     
//        else if (
//                ((((
//                readfds
//                )->fds_bits)[((
//                so->s
//                ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                so->s
//                ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                        ) {
//     
//     
//     
//         if (so->so_state & 0x100) {
//          tcp_connect(so);
//          continue;
//         }
//         ret = soread(so);
//     
//     
//         if (ret > 0)
//            tcp_output(((so)->so_tcpcb));
//        }
//     
//     
//     
//     
//        if (
//           ((((
//           writefds
//           )->fds_bits)[((
//           so->s
//           ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//           so->s
//           ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                    ) {
//     
//     
//     
//          if (so->so_state & 0x002) {
//     
//            so->so_state &= ~0x002;
//     
//            ret = send(so->s, (const void *) &ret, 0, 0);
//            if (ret < 0) {
//     
//              if (
//                 (*__errno_location ()) 
//                       == 
//                          11 
//                                 || 
//                                    (*__errno_location ()) 
//                                          == 
//                                             11 
//                                                         ||
//           
//          (*__errno_location ()) 
//                == 
//                   115 
//                               || 
//                                  (*__errno_location ()) 
//                                        == 
//                                           107
//                                                   )
//         continue;
//     
//     
//              so->so_state &= 0xf000;
//              so->so_state |= 0x001;
//            }
//     
//     
//     
//     
//     
//            tcp_input((struct mbuf *)
//                                    ((void *)0)
//                                        , sizeof(struct ip), so);
//     
//          } else
//            ret = sowrite(so);
//     
//     
//     
//     
//     
//     
//        }
//       }
//     
//     
//     
//     
//     
//     
//       for (so = slirp->udb.so_next; so != &slirp->udb;
//            so = so_next) {
//        so_next = so->so_next;
//     
//        if (so->s != -1 && 
//                          ((((
//                          readfds
//                          )->fds_bits)[((
//                          so->s
//                          ) / (8 * (int) sizeof (__fd_mask)))] & ((__fd_mask) (1UL << ((
//                          so->s
//                          ) % (8 * (int) sizeof (__fd_mask)))))) != 0)
//                                                  ) {
//                                 sorecvfrom(so);
//                             }
//       }
//      }
//     
//     
//     
//     
//      if (slirp->if_queued) {
//          if_start(slirp);
//      }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2522 {
// Port of the polling block of slirp_select_poll (slirp/slirp.c lines 429-584).
// Only the pieces of the C types that are actually touched are mirrored here.

use core::ffi::{c_int, c_long, c_uint, c_void};

const FD_WORD_BITS: usize = core::mem::size_of::<c_long>() * 8;

// glibc fd_set on this target holds only fds_bits, so the pointer cast in
// fd_isset() stays valid.
#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
struct FdSet {
    fds_bits: [c_long; 1024 / FD_WORD_BITS],
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
struct InAddr {
    s_addr: u32,
}

// struct socket, mirrored through so_tcpcb (the last field this code needs).
#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut c_void,
}

extern "C" {
    // u_int curtime;
    static mut curtime: c_uint;

    fn tcp_fasttimo(slirp: *mut c_void);
    fn tcp_slowtimo(slirp: *mut c_void);
    fn ip_slowtimo(slirp: *mut c_void);
    fn if_start(slirp: *mut c_void);

    fn sorecvoob(so: *mut Socket);
    fn sorecvfrom(so: *mut Socket);
    fn soread(so: *mut Socket) -> c_int;
    fn sowrite(so: *mut Socket) -> c_int;
    fn tcp_connect(so: *mut Socket);
    fn tcp_output(tp: *mut c_void) -> c_int;
    fn tcp_input(m: *mut c_void, len: c_int, so: *mut Socket);

    // ssize_t send(int, const void *, size_t, int);
    fn send(fd: c_int, buf: *const c_void, n: usize, flags: c_int) -> c_long;
    fn __errno_location() -> *mut c_int;
}

// FD_ISSET(fd, set): (((set)->fds_bits)[fd / (8 * sizeof(__fd_mask))] &
//                     ((__fd_mask)(1UL << (fd % (8 * sizeof(__fd_mask))))))
#[inline]
unsafe fn fd_isset(fds: *mut FdSet, fd: c_int) -> bool {
    let bits = fds as *mut c_long;
    let idx = (fd as usize) / FD_WORD_BITS;
    let word = *bits.add(idx);
    let shift = (fd as usize) % FD_WORD_BITS;
    (word & ((1 as c_long) << shift)) != 0
}

#[no_mangle]
pub extern "C" fn slirp_select_poll__part2522(
    slirp: *mut c_void,
    tcb: *mut Socket,
    udb: *mut Socket,
    if_queued: *mut c_int,
    readfds: *mut FdSet,
    writefds: *mut FdSet,
    xfds: *mut FdSet,
    select_error: c_int,
    ip_size: c_int,
    time_fasttimo: *mut c_uint,
    last_slowtimo: *mut c_uint,
    do_slowtimo: *mut c_int,
    ret: *mut c_int,
) {
    unsafe {
        let now: c_uint = curtime;

        if *time_fasttimo != 0 && now.wrapping_sub(*time_fasttimo) >= 2 {
            tcp_fasttimo(slirp);
            *time_fasttimo = 0;
        }
        if *do_slowtimo != 0 && now.wrapping_sub(*last_slowtimo) >= 499 {
            ip_slowtimo(slirp);
            tcp_slowtimo(slirp);
            *last_slowtimo = now;
        }

        if select_error == 0 {
            let mut r: c_int = 0;

            let mut so: *mut Socket = (*tcb).so_next;
            while so != tcb {
                let so_next: *mut Socket = (*so).so_next;

                if (*so).so_state & 0x001 != 0 || (*so).s == -1 {
                    so = so_next;
                    continue;
                }

                if fd_isset(xfds, (*so).s) {
                    sorecvoob(so);
                } else if fd_isset(readfds, (*so).s) {
                    if (*so).so_state & 0x100 != 0 {
                        tcp_connect(so);
                        so = so_next;
                        continue;
                    }
                    r = soread(so);
                    *ret = r;
                    if r > 0 {
                        tcp_output((*so).so_tcpcb);
                    }
                }

                if fd_isset(writefds, (*so).s) {
                    if (*so).so_state & 0x002 != 0 {
                        (*so).so_state &= !0x002i32;

                        // ret = send(so->s, (const void *)&ret, 0, 0);
                        r = send(
                            (*so).s,
                            &mut r as *mut c_int as *const c_void,
                            0,
                            0,
                        ) as c_int;
                        *ret = r;
                        if r < 0 {
                            let e: c_int = *__errno_location();
                            if e == 11 || e == 11 || e == 115 || e == 107 {
                                so = so_next;
                                continue;
                            }
                            (*so).so_state &= 0xf000;
                            (*so).so_state |= 0x001;
                        }

                        // tcp_input((struct mbuf *)0, sizeof(struct ip), so);
                        tcp_input(core::ptr::null_mut(), ip_size, so);
                    } else {
                        r = sowrite(so);
                        *ret = r;
                    }
                }

                so = so_next;
            }

            let mut so: *mut Socket = (*udb).so_next;
            while so != udb {
                let so_next: *mut Socket = (*so).so_next;

                if (*so).s != -1 && fd_isset(readfds, (*so).s) {
                    sorecvfrom(so);
                }

                so = so_next;
            }
        }

        if *if_queued != 0 {
            if_start(slirp);
        }
    }
}
}

// ── `global_readfds = ((void *)0) ; global_writefds = ((void *)0) ; global_xfds = ((void *)0) ;` · slirp/slirp.c:591-593 · chunk 2523 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     global_readfds = 
//                       ((void *)0)
//                           ;
//       global_writefds = 
//                        ((void *)0)
//                            ;
//       global_xfds = 
//                    ((void *)0)
//                        ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2523 {
use core::ffi::c_void;

extern "C" {
    static mut global_readfds: *mut c_void;
    static mut global_writefds: *mut c_void;
    static mut global_xfds: *mut c_void;
}

#[no_mangle]
pub extern "C" fn slirp_select_poll__part2523() {
    unsafe {
        core::ptr::addr_of_mut!(global_readfds).write(core::ptr::null_mut());
        core::ptr::addr_of_mut!(global_writefds).write(core::ptr::null_mut());
        core::ptr::addr_of_mut!(global_xfds).write(core::ptr::null_mut());
    }
}
}

// ── `arp_input` · slirp/slirp.c:629-685 · chunk 2526 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void arp_input(Slirp *slirp, const uint8_t *pkt, int pkt_len)
//     {
//         struct ethhdr *eh = (struct ethhdr *)pkt;
//         struct arphdr *ah = (struct arphdr *)(pkt + 14);
//         uint8_t arp_reply[((14 + sizeof(struct arphdr)) > (64) ? (14 + sizeof(struct arphdr)) : (64))];
//         struct ethhdr *reh = (struct ethhdr *)arp_reply;
//         struct arphdr *rah = (struct arphdr *)(arp_reply + 14);
//         int ar_op;
//         struct ex_list *ex_ptr;
//     
//         ar_op = ntohs(ah->ar_op);
//         switch(ar_op) {
//         case 1:
//             if ((ah->ar_tip & slirp->vnetwork_mask.s_addr) ==
//                 slirp->vnetwork_addr.s_addr) {
//                 if (ah->ar_tip == slirp->vnameserver_addr.s_addr ||
//                     ah->ar_tip == slirp->vhost_addr.s_addr)
//                     goto arp_ok;
//                 for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next) {
//                     if (ex_ptr->ex_addr.s_addr == ah->ar_tip)
//                         goto arp_ok;
//                 }
//                 return;
//             arp_ok:
//                 memset(arp_reply, 0, sizeof(arp_reply));
//     
//                 memcpy(slirp->client_ethaddr, eh->h_source, 6);
//     
//     
//                 memcpy(reh->h_dest, pkt + 6, 6);
//                 memcpy(reh->h_source, special_ethaddr, 6 - 4);
//                 memcpy(&reh->h_source[2], &ah->ar_tip, 4);
//                 reh->h_proto = htons(0x0806);
//     
//                 rah->ar_hrd = htons(1);
//                 rah->ar_pro = htons(0x0800);
//                 rah->ar_hln = 6;
//                 rah->ar_pln = 4;
//                 rah->ar_op = htons(2);
//                 memcpy(rah->ar_sha, reh->h_source, 6);
//                 rah->ar_sip = ah->ar_tip;
//                 memcpy(rah->ar_tha, ah->ar_sha, 6);
//                 rah->ar_tip = ah->ar_sip;
//                 slirp_output(slirp->opaque, arp_reply, sizeof(arp_reply));
//             }
//             break;
//         case 2:
//     
//             if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6) &&
//                 ah->ar_sip == slirp->client_ipaddr.s_addr) {
//                 memcpy(slirp->client_ethaddr, ah->ar_sha, 6);
//             }
//             break;
//         default:
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2526 {
// Rust rewrite of slirp's static arp_input().
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const ETH_ALEN: usize = 6;
const ETH_HLEN: usize = 14;
const ARP_REPLY_LEN: usize = 64;

// static const uint8_t special_ethaddr[6] = {0x52, 0x55, 0, 0, 0, 0};
const SPECIAL_ETHADDR: [u8; 6] = [0x52, 0x55, 0x00, 0x00, 0x00, 0x00];

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: u16,
}

#[repr(C, packed)]
pub struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sha: [u8; ETH_ALEN],
    pub ar_sip: u32,
    pub ar_tha: [u8; ETH_ALEN],
    pub ar_tip: u32,
}

#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: in_addr,
    pub vnetwork_mask: in_addr,
    pub vhost_addr: in_addr,
    pub vdhcp_startaddr: in_addr,
    pub vnameserver_addr: in_addr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: in_addr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: timeval,
    pub exec_list: *mut ex_list,
    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    pub ipq: ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: socket,
    pub tcp_last_so: *mut socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: socket,
    pub udp_last_so: *mut socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
}

#[no_mangle]
pub extern "C" fn arp_input(slirp: *mut Slirp, pkt: *const u8, _pkt_len: c_int) {
    unsafe {
        let eh = pkt as *const ethhdr;
        let ah = pkt.add(ETH_HLEN) as *const arphdr;

        // uint8_t arp_reply[max(ETH_HLEN + sizeof(struct arphdr), 64)];
        // 64 bytes, kept 8-byte aligned so the struct casts stay aligned.
        let mut arp_reply = [0u64; 8];
        let reply_ptr = arp_reply.as_mut_ptr() as *mut u8;
        let reh = reply_ptr as *mut ethhdr;
        let rah = reply_ptr.add(ETH_HLEN) as *mut arphdr;

        let ar_op: c_int = u16::from_be(core::ptr::addr_of!((*ah).ar_op).read_unaligned()) as c_int;

        match ar_op {
            1 => {
                let ar_tip = core::ptr::addr_of!((*ah).ar_tip).read_unaligned();
                let vnetwork_mask = (*slirp).vnetwork_mask.s_addr;
                let vnetwork_addr = (*slirp).vnetwork_addr.s_addr;

                if (ar_tip & vnetwork_mask) == vnetwork_addr {
                    let mut arp_ok = false;

                    if ar_tip == (*slirp).vnameserver_addr.s_addr
                        || ar_tip == (*slirp).vhost_addr.s_addr
                    {
                        arp_ok = true;
                    } else {
                        let mut ex_ptr = (*slirp).exec_list;
                        while !ex_ptr.is_null() {
                            if (*ex_ptr).ex_addr.s_addr == ar_tip {
                                arp_ok = true;
                                break;
                            }
                            ex_ptr = (*ex_ptr).ex_next;
                        }
                    }

                    if arp_ok {
                        core::ptr::write_bytes(reply_ptr, 0u8, ARP_REPLY_LEN);

                        // memcpy(slirp->client_ethaddr, eh->h_source, ETH_ALEN);
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*eh).h_source) as *const u8,
                            core::ptr::addr_of_mut!((*slirp).client_ethaddr) as *mut u8,
                            ETH_ALEN,
                        );

                        // memcpy(reh->h_dest, pkt + ETH_ALEN, ETH_ALEN);
                        core::ptr::copy_nonoverlapping(
                            pkt.add(ETH_ALEN),
                            core::ptr::addr_of_mut!((*reh).h_dest) as *mut u8,
                            ETH_ALEN,
                        );

                        // memcpy(reh->h_source, special_ethaddr, ETH_ALEN - 4);
                        core::ptr::copy_nonoverlapping(
                            SPECIAL_ETHADDR.as_ptr(),
                            core::ptr::addr_of_mut!((*reh).h_source) as *mut u8,
                            ETH_ALEN - 4,
                        );

                        // memcpy(&reh->h_source[2], &ah->ar_tip, 4);
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*ah).ar_tip) as *const u8,
                            (core::ptr::addr_of_mut!((*reh).h_source) as *mut u8).add(2),
                            4,
                        );

                        // reh->h_proto = htons(ETH_P_ARP);
                        (*reh).h_proto = 0x0806u16.to_be();

                        // rah->ar_hrd = htons(1);
                        core::ptr::addr_of_mut!((*rah).ar_hrd).write_unaligned(1u16.to_be());
                        // rah->ar_pro = htons(ETH_P_IP);
                        core::ptr::addr_of_mut!((*rah).ar_pro).write_unaligned(0x0800u16.to_be());
                        // rah->ar_hln = ETH_ALEN;
                        core::ptr::addr_of_mut!((*rah).ar_hln).write_unaligned(ETH_ALEN as u8);
                        // rah->ar_pln = 4;
                        core::ptr::addr_of_mut!((*rah).ar_pln).write_unaligned(4u8);
                        // rah->ar_op = htons(ARPOP_REPLY);
                        core::ptr::addr_of_mut!((*rah).ar_op).write_unaligned(2u16.to_be());

                        // memcpy(rah->ar_sha, reh->h_source, ETH_ALEN);
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*reh).h_source) as *const u8,
                            core::ptr::addr_of_mut!((*rah).ar_sha) as *mut u8,
                            ETH_ALEN,
                        );

                        // rah->ar_sip = ah->ar_tip;
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*ah).ar_tip) as *const u8,
                            core::ptr::addr_of_mut!((*rah).ar_sip) as *mut u8,
                            4,
                        );

                        // memcpy(rah->ar_tha, ah->ar_sha, ETH_ALEN);
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*ah).ar_sha) as *const u8,
                            core::ptr::addr_of_mut!((*rah).ar_tha) as *mut u8,
                            ETH_ALEN,
                        );

                        // rah->ar_tip = ah->ar_sip;
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*ah).ar_sip) as *const u8,
                            core::ptr::addr_of_mut!((*rah).ar_tip) as *mut u8,
                            4,
                        );

                        slirp_output((*slirp).opaque, reply_ptr, ARP_REPLY_LEN as c_int);
                    }
                }
            }
            2 => {
                // reply to request of client mac address?
                let mut client_mac = [0u8; ETH_ALEN];
                core::ptr::copy_nonoverlapping(
                    core::ptr::addr_of!((*slirp).client_ethaddr) as *const u8,
                    client_mac.as_mut_ptr(),
                    ETH_ALEN,
                );
                // !memcmp(slirp->client_ethaddr, zero_ethaddr, ETH_ALEN)
                if client_mac == [0u8; ETH_ALEN] {
                    let ar_sip = core::ptr::addr_of!((*ah).ar_sip).read_unaligned();
                    if ar_sip == (*slirp).client_ipaddr.s_addr {
                        core::ptr::copy_nonoverlapping(
                            core::ptr::addr_of!((*ah).ar_sha) as *const u8,
                            core::ptr::addr_of_mut!((*slirp).client_ethaddr) as *mut u8,
                            ETH_ALEN,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
}

// ── `struct ethhdr *eh = (struct ethhdr *)pkt; struct arphdr *ah = (struct arphdr *)(pkt + 14); uint…` · slirp/slirp.c:631-639 · chunk 2527 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct ethhdr *eh = (struct ethhdr *)pkt;
//         struct arphdr *ah = (struct arphdr *)(pkt + 14);
//         uint8_t arp_reply[((14 + sizeof(struct arphdr)) > (64) ? (14 + sizeof(struct arphdr)) : (64))];
//         struct ethhdr *reh = (struct ethhdr *)arp_reply;
//         struct arphdr *rah = (struct arphdr *)(arp_reply + 14);
//         int ar_op;
//         struct ex_list *ex_ptr;
//     
//         ar_op = ntohs(ah->ar_op);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2527 {
// Port of the declaration/initialisation prologue of arp_input:
// the ARP header is located at pkt + 14 and its ar_op field is read
// and converted from network to host byte order.

use core::ffi::{c_int, c_ushort};

// Mirror of the packed C struct arphdr (see the translation unit).
#[repr(C, packed)]
pub struct Arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sha: [u8; 6],
    pub ar_sip: u32,
    pub ar_tha: [u8; 6],
    pub ar_tip: u32,
}

extern "C" {
    fn ntohs(__netshort: c_ushort) -> c_ushort;
}

#[no_mangle]
pub extern "C" fn arp_input__part2527(pkt: *const u8) -> c_int {
    unsafe {
        let ah = pkt.add(14) as *const Arphdr;
        let raw_op = core::ptr::addr_of!((*ah).ar_op).read_unaligned();
        ntohs(raw_op) as c_int
    }
}
}

// ── `switch (ar_op)` · slirp/slirp.c:640-684 · chunk 2528 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(ar_op) {
//         case 1:
//             if ((ah->ar_tip & slirp->vnetwork_mask.s_addr) ==
//                 slirp->vnetwork_addr.s_addr) {
//                 if (ah->ar_tip == slirp->vnameserver_addr.s_addr ||
//                     ah->ar_tip == slirp->vhost_addr.s_addr)
//                     goto arp_ok;
//                 for (ex_ptr = slirp->exec_list; ex_ptr; ex_ptr = ex_ptr->ex_next) {
//                     if (ex_ptr->ex_addr.s_addr == ah->ar_tip)
//                         goto arp_ok;
//                 }
//                 return;
//             arp_ok:
//                 memset(arp_reply, 0, sizeof(arp_reply));
//     
//                 memcpy(slirp->client_ethaddr, eh->h_source, 6);
//     
//     
//                 memcpy(reh->h_dest, pkt + 6, 6);
//                 memcpy(reh->h_source, special_ethaddr, 6 - 4);
//                 memcpy(&reh->h_source[2], &ah->ar_tip, 4);
//                 reh->h_proto = htons(0x0806);
//     
//                 rah->ar_hrd = htons(1);
//                 rah->ar_pro = htons(0x0800);
//                 rah->ar_hln = 6;
//                 rah->ar_pln = 4;
//                 rah->ar_op = htons(2);
//                 memcpy(rah->ar_sha, reh->h_source, 6);
//                 rah->ar_sip = ah->ar_tip;
//                 memcpy(rah->ar_tha, ah->ar_sha, 6);
//                 rah->ar_tip = ah->ar_sip;
//                 slirp_output(slirp->opaque, arp_reply, sizeof(arp_reply));
//             }
//             break;
//         case 2:
//     
//             if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6) &&
//                 ah->ar_sip == slirp->client_ipaddr.s_addr) {
//                 memcpy(slirp->client_ethaddr, ah->ar_sha, 6);
//             }
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2528 {
// Port of the ARP request/reply handling switch from slirp's arp_input().
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C, packed)]
pub struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sha: [u8; 6],
    pub ar_sip: u32,
    pub ar_tha: [u8; 6],
    pub ar_tip: u32,
}

#[repr(C)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

extern "C" {
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
    fn htons(hostshort: u16) -> u16;
}

// static const arrays of the C translation unit: no linker symbol, so inline them.
static SPECIAL_ETHADDR: [u8; 6] = [0x52, 0x55, 0x00, 0x00, 0x00, 0x00];
static ZERO_ETHADDR: [u8; 6] = [0, 0, 0, 0, 0, 0];

// ((14 + sizeof(struct arphdr)) > 64 ? (14 + sizeof(struct arphdr)) : 64)
const ARP_REPLY_SIZE: usize = if 14 + 28 > 64 { 14 + 28 } else { 64 };

// Returns 1 when the caller must return from arp_input(), 0 otherwise.
#[no_mangle]
pub extern "C" fn arp_input__part2528(
    opaque: *mut c_void,
    client_ethaddr: *mut u8,
    eh: *const ethhdr,
    ah: *const arphdr,
    arp_reply: *mut u8,
    reh: *mut ethhdr,
    rah: *mut arphdr,
    ar_op: c_int,
    vnetwork_mask: u32,
    vnetwork_addr: u32,
    vnameserver_addr: u32,
    vhost_addr: u32,
    client_ipaddr: u32,
    exec_list: *mut ex_list,
    ex_ptr_out: *mut *mut ex_list,
) -> c_int {
    unsafe {
        match ar_op {
            1 => {
                let ar_tip: u32 = ptr::read_unaligned(ptr::addr_of!((*ah).ar_tip));
                if (ar_tip & vnetwork_mask) == vnetwork_addr {
                    let mut ok: bool = ar_tip == vnameserver_addr || ar_tip == vhost_addr;
                    let mut ex_ptr: *mut ex_list = exec_list;
                    while !ok && !ex_ptr.is_null() {
                        if (*ex_ptr).ex_addr.s_addr == ar_tip {
                            ok = true;
                            break;
                        }
                        ex_ptr = (*ex_ptr).ex_next;
                    }
                    if !ex_ptr_out.is_null() {
                        *ex_ptr_out = ex_ptr;
                    }
                    if !ok {
                        // C: return;
                        return 1;
                    }

                    // arp_ok:
                    ptr::write_bytes(arp_reply, 0, ARP_REPLY_SIZE);

                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*eh).h_source) as *const u8,
                        client_ethaddr,
                        6,
                    );

                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*eh).h_source) as *const u8,
                        ptr::addr_of_mut!((*reh).h_dest) as *mut u8,
                        6,
                    );
                    ptr::copy_nonoverlapping(
                        SPECIAL_ETHADDR.as_ptr(),
                        ptr::addr_of_mut!((*reh).h_source) as *mut u8,
                        6 - 4,
                    );
                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*ah).ar_tip) as *const u8,
                        (ptr::addr_of_mut!((*reh).h_source) as *mut u8).add(2),
                        4,
                    );
                    ptr::write_unaligned(ptr::addr_of_mut!((*reh).h_proto), htons(0x0806));

                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_hrd), htons(1));
                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_pro), htons(0x0800));
                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_hln), 6u8);
                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_pln), 4u8);
                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_op), htons(2));
                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*reh).h_source) as *const u8,
                        ptr::addr_of_mut!((*rah).ar_sha) as *mut u8,
                        6,
                    );
                    ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_sip), ar_tip);
                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*ah).ar_sha) as *const u8,
                        ptr::addr_of_mut!((*rah).ar_tha) as *mut u8,
                        6,
                    );
                    ptr::write_unaligned(
                        ptr::addr_of_mut!((*rah).ar_tip),
                        ptr::read_unaligned(ptr::addr_of!((*ah).ar_sip)),
                    );
                    slirp_output(opaque, arp_reply, ARP_REPLY_SIZE as c_int);
                }
                0
            }
            2 => {
                let mut is_zero: bool = true;
                let mut i: usize = 0;
                while i < 6 {
                    if *client_ethaddr.add(i) != ZERO_ETHADDR[i] {
                        is_zero = false;
                        break;
                    }
                    i += 1;
                }
                if is_zero
                    && ptr::read_unaligned(ptr::addr_of!((*ah).ar_sip)) == client_ipaddr
                {
                    ptr::copy_nonoverlapping(
                        ptr::addr_of!((*ah).ar_sha) as *const u8,
                        client_ethaddr,
                        6,
                    );
                }
                0
            }
            _ => 0,
        }
    }
}
}

// ── `slirp_input` · slirp/slirp.c:687-719 · chunk 2529 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     void slirp_input(Slirp *slirp, const uint8_t *pkt, int pkt_len)
//     {
//         struct mbuf *m;
//         int proto;
//     
//         if (pkt_len < 14)
//             return;
//     
//         proto = ntohs(*(uint16_t *)(pkt + 12));
//         switch(proto) {
//         case 0x0806:
//             arp_input(slirp, pkt, pkt_len);
//             break;
//         case 0x0800:
//             m = m_get(slirp);
//             if (!m)
//                 return;
//     
//             if ((((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len) < pkt_len + 2) {
//                 m_inc(m, pkt_len + 2);
//             }
//             m->m_hdr.mh_len = pkt_len + 2;
//             memcpy(m->m_hdr.mh_data + 2, pkt, pkt_len);
//     
//             m->m_hdr.mh_data += 2 + 14;
//             m->m_hdr.mh_len -= 2 + 14;
//     
//             ip_input(m);
//             break;
//         default:
//             break;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2529 {
// Port of slirp_input() from slirp/slirp.c (includes the static arp_input()
// helper ported as a private Rust fn, since it has no linker symbol).
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

// ---------------------------------------------------------------- C types ---

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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut MBuf,
    pub so_ti: *mut c_void,   // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// union M_dat { char m_dat_[1]; char *m_ext_; }
// Both members live at the same address; the pointer governs alignment/size.
#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_ext: *mut c_char,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut ExList,
    pub m_freelist: MBuf,
    pub m_usedlist: MBuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: MBuf,
    pub if_batchq: MBuf,
    pub next_m: *mut MBuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

#[repr(C)]
pub struct EthHdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}

#[repr(C, packed)]
pub struct ArpHdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sha: [u8; 6],
    pub ar_sip: u32,
    pub ar_tha: [u8; 6],
    pub ar_tip: u32,
}

// -------------------------------------------------------------- constants ---

const ETH_HLEN: c_int = 14;
const ETH_ALEN: usize = 6;
const ETH_P_ARP: c_int = 0x0806;
const ETH_P_IP: c_int = 0x0800;

// sizeof(struct arphdr) == 28, so max(14 + 28, 64) == 64
const ARP_REPLY_LEN: usize = 64;

// static const uint8_t special_ethaddr[6] (static in C, mirrored here)
const SPECIAL_ETHADDR: [u8; 6] = [0x52, 0x55, 0x00, 0x00, 0x00, 0x00];

// ---------------------------------------------------------- C externals -----

extern "C" {
    fn ntohs(netshort: u16) -> u16;
    fn htons(hostshort: u16) -> u16;
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
    fn m_get(slirp: *mut Slirp) -> *mut MBuf;
    fn m_inc(m: *mut MBuf, size: c_int);
    fn ip_input(m: *mut MBuf);
}

// --------------------------------------- ported static C fn: arp_input() ---

unsafe fn arp_input(slirp: *mut Slirp, pkt: *const u8, pkt_len: c_int) {
    let _ = pkt_len;
    let eh: *const EthHdr = pkt as *const EthHdr;
    let ah: *const ArpHdr = pkt.add(ETH_HLEN as usize) as *const ArpHdr;
    let mut arp_reply = [0u8; ARP_REPLY_LEN];
    let reh: *mut EthHdr = arp_reply.as_mut_ptr() as *mut EthHdr;
    let rah: *mut ArpHdr = arp_reply.as_mut_ptr().add(ETH_HLEN as usize) as *mut ArpHdr;

    let ar_op = ntohs(ptr::read_unaligned(ptr::addr_of!((*ah).ar_op))) as c_int;

    if ar_op == 1 {
        let ar_tip = ptr::read_unaligned(ptr::addr_of!((*ah).ar_tip));
        if (ar_tip & (*slirp).vnetwork_mask.s_addr) != (*slirp).vnetwork_addr.s_addr {
            return;
        }
        if ar_tip != (*slirp).vnameserver_addr.s_addr && ar_tip != (*slirp).vhost_addr.s_addr {
            let mut found = false;
            let mut ex_ptr = (*slirp).exec_list;
            while !ex_ptr.is_null() {
                if (*ex_ptr).ex_addr.s_addr == ar_tip {
                    found = true;
                    break;
                }
                ex_ptr = (*ex_ptr).ex_next;
            }
            if !found {
                return;
            }
        }

        // arp_ok:
        ptr::write_bytes(arp_reply.as_mut_ptr(), 0, ARP_REPLY_LEN);

        ptr::copy_nonoverlapping(
            ptr::addr_of!((*eh).h_source) as *const u8,
            (*slirp).client_ethaddr.as_mut_ptr(),
            ETH_ALEN,
        );

        ptr::copy_nonoverlapping(
            pkt.add(6),
            ptr::addr_of_mut!((*reh).h_dest) as *mut u8,
            ETH_ALEN,
        );
        ptr::copy_nonoverlapping(
            SPECIAL_ETHADDR.as_ptr(),
            ptr::addr_of_mut!((*reh).h_source) as *mut u8,
            2,
        );
        ptr::copy_nonoverlapping(
            ptr::addr_of!((*ah).ar_tip) as *const u8,
            (ptr::addr_of_mut!((*reh).h_source) as *mut u8).add(2),
            4,
        );
        ptr::write_unaligned(ptr::addr_of_mut!((*reh).h_proto), htons(0x0806));

        ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_hrd), htons(1));
        ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_pro), htons(0x0800));
        ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_hln), 6);
        ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_pln), 4);
        ptr::write_unaligned(ptr::addr_of_mut!((*rah).ar_op), htons(2));
        ptr::copy_nonoverlapping(
            ptr::addr_of!((*reh).h_source) as *const u8,
            ptr::addr_of_mut!((*rah).ar_sha) as *mut u8,
            ETH_ALEN,
        );
        ptr::write_unaligned(
            ptr::addr_of_mut!((*rah).ar_sip),
            ptr::read_unaligned(ptr::addr_of!((*ah).ar_tip)),
        );
        ptr::copy_nonoverlapping(
            ptr::addr_of!((*ah).ar_sha) as *const u8,
            ptr::addr_of_mut!((*rah).ar_tha) as *mut u8,
            ETH_ALEN,
        );
        ptr::write_unaligned(
            ptr::addr_of_mut!((*rah).ar_tip),
            ptr::read_unaligned(ptr::addr_of!((*ah).ar_sip)),
        );

        slirp_output((*slirp).opaque, arp_reply.as_ptr(), ARP_REPLY_LEN as c_int);
    } else if ar_op == 2 {
        // reply to request of client mac address ?
        let client_mac: [u8; 6] = (*slirp).client_ethaddr;
        if client_mac == [0u8; 6]
            && ptr::read_unaligned(ptr::addr_of!((*ah).ar_sip)) == (*slirp).client_ipaddr.s_addr
        {
            ptr::copy_nonoverlapping(
                ptr::addr_of!((*ah).ar_sha) as *const u8,
                (*slirp).client_ethaddr.as_mut_ptr(),
                ETH_ALEN,
            );
        }
    }
}

// ---------------------------------------------------------------- TARGET ----

#[no_mangle]
pub unsafe extern "C" fn slirp_input(slirp: *mut Slirp, pkt: *const u8, pkt_len: c_int) {
    let m: *mut MBuf;
    let proto: c_int;

    if pkt_len < ETH_HLEN {
        return;
    }

    // proto = ntohs(*(uint16_t *)(pkt + 12));
    let raw_proto: u16 = ptr::read_unaligned(pkt.add(12) as *const u16);
    proto = ntohs(raw_proto) as c_int;

    match proto {
        ETH_P_ARP => {
            arp_input(slirp, pkt, pkt_len);
        }
        ETH_P_IP => {
            m = m_get(slirp);
            if m.is_null() {
                return;
            }

            // M_FREEROOM(m): (mh_flags & M_EXT ? m_ext_ : m_dat_ + mh_size)
            //                - mh_data - mh_len
            let base: *mut c_char = if ((*m).m_hdr.mh_flags & 0x01) != 0 {
                (*m).m_ext
            } else {
                ptr::addr_of!((*m).m_ext) as *mut c_char
            };
            let freeroom: isize = ((base as isize).wrapping_add((*m).m_hdr.mh_size as isize)
                - ((*m).m_hdr.mh_data as isize))
            .wrapping_sub((*m).m_hdr.mh_len as isize);

            if freeroom < pkt_len.wrapping_add(2) as isize {
                m_inc(m, pkt_len.wrapping_add(2));
            }
            (*m).m_hdr.mh_len = pkt_len.wrapping_add(2);
            ptr::copy_nonoverlapping(
                pkt,
                (*m).m_hdr.mh_data.add(2) as *mut u8,
                pkt_len as usize,
            );

            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(2 + ETH_HLEN as isize);
            (*m).m_hdr.mh_len = (*m).m_hdr.mh_len.wrapping_sub(2 + ETH_HLEN);

            ip_input(m);
        }
        _ => {}
    }
}
}

// ── `if (pkt_len < 14)` · slirp/slirp.c:692-693 · chunk 2530 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pkt_len < 14)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2530 {
use core::ffi::c_int;

// Port of slirp_input's length guard: returns nonzero when the caller
// should return early (packet shorter than an Ethernet header).
#[no_mangle]
pub extern "C" fn slirp_input__part2530(pkt_len: c_int) -> c_int {
    if pkt_len < 14 {
        1
    } else {
        0
    }
}
}

// ── `proto = ntohs(*(uint16_t *)(pkt + 12));` · slirp/slirp.c:695-695 · chunk 2531 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     proto = ntohs(*(uint16_t *)(pkt + 12));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2531 {
// Port of: proto = ntohs(*(uint16_t *)(pkt + 12));
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn slirp_input__part2531(pkt: *const u8) -> c_int {
    // Mirror C's unaligned/possibly-aliasing 16-bit load followed by ntohs (big-endian
    // network order to host order, i.e. a byte swap on little-endian hosts).
    let raw: u16 = unsafe { core::ptr::read_unaligned(pkt.add(12) as *const u16) };
    raw.swap_bytes() as c_int
}
}

// ── `switch (proto)` · slirp/slirp.c:696-718 · chunk 2532 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch(proto) {
//         case 0x0806:
//             arp_input(slirp, pkt, pkt_len);
//             break;
//         case 0x0800:
//             m = m_get(slirp);
//             if (!m)
//                 return;
//     
//             if ((((m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data) : (((m)->M_dat.m_dat_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)) - (m)->m_hdr.mh_len) < pkt_len + 2) {
//                 m_inc(m, pkt_len + 2);
//             }
//             m->m_hdr.mh_len = pkt_len + 2;
//             memcpy(m->m_hdr.mh_data + 2, pkt, pkt_len);
//     
//             m->m_hdr.mh_data += 2 + 14;
//             m->m_hdr.mh_len -= 2 + 14;
//     
//             ip_input(m);
//             break;
//         default:
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2532 {
// Port of the switch statement inside slirp_input() (dispatch on the
// ethernet protocol field), plus the file-static C function arp_input(),
// which has no linker symbol and is therefore ported here as a private fn.
use core::ffi::{c_char, c_int, c_void};

// Mirror of `struct socket` (opaque, only pointed to).
#[repr(C)]
pub struct Socket {
    _opaque: [u8; 0],
}

// Mirror of `struct m_hdr`.
#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut MBuf,
    pub mh_prev: *mut MBuf,
    pub mh_nextpkt: *mut MBuf,
    pub mh_prevpkt: *mut MBuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

// Mirror of `union M_dat`.
#[repr(C)]
pub union MDat {
    pub m_dat_: *mut c_char,
    pub m_ext_: *mut c_char,
}

// Mirror of `struct mbuf`.
#[repr(C)]
pub struct MBuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

// Mirror of `typedef struct Slirp Slirp` (opaque; only handed back to C).
#[repr(C)]
pub struct Slirp {
    _opaque: [u8; 0],
}

// Mirror of `struct ex_list`.
#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: u32,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

extern "C" {
    fn m_get(slirp: *mut Slirp) -> *mut MBuf;
    fn m_inc(m: *mut MBuf, size: c_int);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn ip_input(m: *mut MBuf);
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
}

// ---- arp_input() port -------------------------------------------------

static SPECIAL_ETHADDR: [u8; 6] = [0x52, 0x55, 0x00, 0x00, 0x00, 0x00];
static ZERO_ETHADDR: [u8; 6] = [0, 0, 0, 0, 0, 0];

#[repr(C)]
struct EthHdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: u16,
}

#[repr(C, packed)]
struct ArpHdr {
    ar_hrd: u16,
    ar_pro: u16,
    ar_hln: u8,
    ar_pln: u8,
    ar_op: u16,
    ar_sha: [u8; 6],
    ar_sip: u32,
    ar_tha: [u8; 6],
    ar_tip: u32,
}

const ARP_HDR_SIZE: usize = 28; // sizeof(struct arphdr), packed
const ARP_REPLY_SIZE: usize = if 14 + ARP_HDR_SIZE > 64 {
    14 + ARP_HDR_SIZE
} else {
    64
};

#[inline]
fn ntohs(x: u16) -> u16 {
    x.to_be()
}

#[inline]
fn htons(x: u16) -> u16 {
    x.to_be()
}

unsafe fn c_memcpy(dst: *mut u8, src: *const u8, n: usize) {
    memcpy(dst as *mut c_void, src as *const c_void, n);
}

unsafe fn c_memcmp(a: *const u8, b: *const u8, n: usize) -> c_int {
    let mut i = 0usize;
    while i < n {
        let x = *a.add(i);
        let y = *b.add(i);
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        i += 1;
    }
    0
}

// `slirp`'s fields this port needs are passed explicitly (see signature).
unsafe fn arp_input(
    pkt: *const u8,
    client_ethaddr: *mut u8,
    client_ipaddr: u32,
    vnetwork_addr: u32,
    vnetwork_mask: u32,
    vhost_addr: u32,
    vnameserver_addr: u32,
    exec_list: *mut ExList,
    opaque: *mut c_void,
) {
    let eh = pkt as *const EthHdr;
    let ah = (pkt as *mut u8).add(14) as *mut ArpHdr;
    let mut arp_reply: [u8; ARP_REPLY_SIZE] = [0u8; ARP_REPLY_SIZE];
    let reh = arp_reply.as_mut_ptr() as *mut EthHdr;
    let rah = (arp_reply.as_mut_ptr() as *mut u8).add(14) as *mut ArpHdr;

    let ar_op = ntohs(core::ptr::addr_of!((*ah).ar_op).read_unaligned());
    let ar_tip = core::ptr::addr_of!((*ah).ar_tip).read_unaligned();

    match ar_op {
        1 => {
            if (ar_tip & vnetwork_mask) != vnetwork_addr {
                return;
            }
            let mut arp_ok = ar_tip == vnameserver_addr || ar_tip == vhost_addr;
            if !arp_ok {
                let mut ex_ptr = exec_list;
                while !ex_ptr.is_null() {
                    if (*ex_ptr).ex_addr == ar_tip {
                        arp_ok = true;
                        break;
                    }
                    ex_ptr = (*ex_ptr).ex_next;
                }
            }
            if !arp_ok {
                return;
            }

            // memset(arp_reply, 0, sizeof(arp_reply));
            let mut i = 0usize;
            while i < ARP_REPLY_SIZE {
                arp_reply[i] = 0;
                i += 1;
            }

            // memcpy(slirp->client_ethaddr, eh->h_source, 6);
            c_memcpy(
                client_ethaddr,
                core::ptr::addr_of!((*eh).h_source) as *const u8,
                6,
            );

            // memcpy(reh->h_dest, pkt + 6, 6);
            c_memcpy(
                core::ptr::addr_of_mut!((*reh).h_dest) as *mut u8,
                (pkt as *const u8).add(6),
                6,
            );
            // memcpy(reh->h_source, special_ethaddr, 6 - 4);
            c_memcpy(
                core::ptr::addr_of_mut!((*reh).h_source) as *mut u8,
                SPECIAL_ETHADDR.as_ptr(),
                6 - 4,
            );
            // memcpy(&reh->h_source[2], &ah->ar_tip, 4);
            c_memcpy(
                (core::ptr::addr_of_mut!((*reh).h_source) as *mut u8).add(2),
                core::ptr::addr_of!((*ah).ar_tip) as *const u8,
                4,
            );
            core::ptr::addr_of_mut!((*reh).h_proto).write_unaligned(htons(0x0806));

            core::ptr::addr_of_mut!((*rah).ar_hrd).write_unaligned(htons(1));
            core::ptr::addr_of_mut!((*rah).ar_pro).write_unaligned(htons(0x0800));
            core::ptr::addr_of_mut!((*rah).ar_hln).write_unaligned(6);
            core::ptr::addr_of_mut!((*rah).ar_pln).write_unaligned(4);
            core::ptr::addr_of_mut!((*rah).ar_op).write_unaligned(htons(2));
            // memcpy(rah->ar_sha, reh->h_source, 6);
            c_memcpy(
                core::ptr::addr_of_mut!((*rah).ar_sha) as *mut u8,
                core::ptr::addr_of!((*reh).h_source) as *const u8,
                6,
            );
            core::ptr::addr_of_mut!((*rah).ar_sip).write_unaligned(ar_tip);
            // memcpy(rah->ar_tha, ah->ar_sha, 6);
            c_memcpy(
                core::ptr::addr_of_mut!((*rah).ar_tha) as *mut u8,
                core::ptr::addr_of!((*ah).ar_sha) as *const u8,
                6,
            );
            // rah->ar_tip = ah->ar_sip;
            let ar_sip = core::ptr::addr_of!((*ah).ar_sip).read_unaligned();
            core::ptr::addr_of_mut!((*rah).ar_tip).write_unaligned(ar_sip);

            slirp_output(opaque, arp_reply.as_ptr(), ARP_REPLY_SIZE as c_int);
        }
        2 => {
            if c_memcmp(client_ethaddr, ZERO_ETHADDR.as_ptr(), 6) == 0
                && core::ptr::addr_of!((*ah).ar_sip).read_unaligned() == client_ipaddr
            {
                c_memcpy(
                    client_ethaddr,
                    core::ptr::addr_of!((*ah).ar_sha) as *const u8,
                    6,
                );
            }
        }
        _ => {}
    }
}

// ---- the switch itself ------------------------------------------------

/// Returns 0 normally, or 1 when the C caller must `return` from
/// slirp_input() (m_get() failed). Writes the allocated mbuf to *mp.
#[no_mangle]
pub unsafe extern "C" fn slirp_input__part2532(
    slirp: *mut Slirp,
    pkt: *const u8,
    pkt_len: c_int,
    proto: c_int,
    mp: *mut *mut MBuf,
    client_ethaddr: *mut u8,
    client_ipaddr: u32,
    vnetwork_addr: u32,
    vnetwork_mask: u32,
    vhost_addr: u32,
    vnameserver_addr: u32,
    exec_list: *mut ExList,
    opaque: *mut c_void,
) -> c_int {
    match proto {
        0x0806 => {
            arp_input(
                pkt,
                client_ethaddr,
                client_ipaddr,
                vnetwork_addr,
                vnetwork_mask,
                vhost_addr,
                vnameserver_addr,
                exec_list,
                opaque,
            );
        }
        0x0800 => {
            let m = m_get(slirp);
            *mp = m;
            if m.is_null() {
                return 1;
            }

            // Space left in the mbuf: (base + mh_size) - mh_data - mh_len.
            let base: *mut c_char = if ((*m).m_hdr.mh_flags & 0x01) != 0 {
                (*m).M_dat.m_ext_
            } else {
                (*m).M_dat.m_dat_
            };
            let avail: isize = base
                .offset((*m).m_hdr.mh_size as isize)
                .offset_from((*m).m_hdr.mh_data)
                - (*m).m_hdr.mh_len as isize;

            if avail < (pkt_len + 2) as isize {
                m_inc(m, pkt_len + 2);
            }
            (*m).m_hdr.mh_len = pkt_len + 2;
            memcpy(
                (*m).m_hdr.mh_data.add(2) as *mut c_void,
                pkt as *const c_void,
                pkt_len as usize,
            );

            (*m).m_hdr.mh_data = (*m).m_hdr.mh_data.offset(2 + 14);
            (*m).m_hdr.mh_len -= 2 + 14;

            ip_input(m);
        }
        _ => {}
    }
    0
}
}

// ── `if_encap` · slirp/slirp.c:722-770 · chunk 2533 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void if_encap(Slirp *slirp, const uint8_t *ip_data, int ip_data_len)
//     {
//         uint8_t buf[1600];
//         struct ethhdr *eh = (struct ethhdr *)buf;
//     
//         if (ip_data_len + 14 > sizeof(buf))
//             return;
//     
//         if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6)) {
//             uint8_t arp_req[14 + sizeof(struct arphdr)];
//             struct ethhdr *reh = (struct ethhdr *)arp_req;
//             struct arphdr *rah = (struct arphdr *)(arp_req + 14);
//             const struct ip *iph = (const struct ip *)ip_data;
//     
//     
//     
//     
//     
//     
//             memset(reh->h_dest, 0xff, 6);
//             memcpy(reh->h_source, special_ethaddr, 6 - 4);
//             memcpy(&reh->h_source[2], &slirp->vhost_addr, 4);
//             reh->h_proto = htons(0x0806);
//             rah->ar_hrd = htons(1);
//             rah->ar_pro = htons(0x0800);
//             rah->ar_hln = 6;
//             rah->ar_pln = 4;
//             rah->ar_op = htons(1);
//     
//             memcpy(rah->ar_sha, special_ethaddr, 6 - 4);
//             memcpy(&rah->ar_sha[2], &slirp->vhost_addr, 4);
//     
//             rah->ar_sip = slirp->vhost_addr.s_addr;
//     
//             memset(rah->ar_tha, 0, 6);
//     
//             rah->ar_tip = iph->ip_dst.s_addr;
//             slirp->client_ipaddr = iph->ip_dst;
//             slirp_output(slirp->opaque, arp_req, sizeof(arp_req));
//         } else {
//             memcpy(eh->h_dest, slirp->client_ethaddr, 6);
//             memcpy(eh->h_source, special_ethaddr, 6 - 4);
//     
//             memcpy(&eh->h_source[2], &slirp->vhost_addr, 4);
//             eh->h_proto = htons(0x0800);
//             memcpy(buf + sizeof(struct ethhdr), ip_data, ip_data_len);
//             slirp_output(slirp->opaque, buf, ip_data_len + 14);
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2533 {
// Rust rewrite of slirp's if_encap(): output an IP packet to the ethernet device.
use core::ffi::{c_int, c_void};

const ETH_ALEN: usize = 6;
const ETH_HLEN: usize = 14;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_ARP: u16 = 0x0806;
const ARPOP_REQUEST: u16 = 1;
const BUF_SIZE: usize = 1600;

extern "C" {
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
}

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// packed IPv4 header; only ip_dst is used here (offset 16)
#[repr(C)]
pub struct Ip {
    pub ip_hl_v: u8,
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: InAddr,
    pub ip_dst: InAddr,
}

#[repr(C)]
struct EthHdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}

#[repr(C, packed)]
struct ArpHdr {
    ar_hrd: u16,
    ar_pro: u16,
    ar_hln: u8,
    ar_pln: u8,
    ar_op: u16,
    ar_sha: [u8; ETH_ALEN],
    ar_sip: u32,
    ar_tha: [u8; ETH_ALEN],
    ar_tip: u32,
}

#[repr(C)]
struct SbUf {
    sb_cc: u32,
    sb_datalen: u32,
    sb_wptr: *mut i8,
    sb_rptr: *mut i8,
    sb_data: *mut i8,
}

#[repr(C)]
struct BootpClient {
    allocated: u16,
    macaddr: [u8; 6],
}

#[repr(C)]
struct TftpSession {
    slirp: *mut c_void,
    filename: *mut i8,
    client_ip: InAddr,
    client_port: u16,
    timestamp: c_int,
}

// Mirror of `struct Slirp`; only fields up to `opaque` are needed.
#[repr(C)]
struct Slirp {
    vnetwork_addr: InAddr,
    vnetwork_mask: InAddr,
    vhost_addr: InAddr,
    vdhcp_startaddr: InAddr,
    vnameserver_addr: InAddr,
    client_ethaddr: [u8; ETH_ALEN],
    client_ipaddr: InAddr,
    client_hostname: [i8; 33],
    restricted: c_int,
    tt: [u8; 16], // struct timeval
    exec_list: *mut c_void,
    m_freelist: [u8; 80], // struct mbuf
    m_usedlist: [u8; 80], // struct mbuf
    mbuf_alloced: c_int,
    if_queued: c_int,
    if_fastq: [u8; 80],  // struct mbuf
    if_batchq: [u8; 80], // struct mbuf
    next_m: *mut c_void,
    ipq: [u8; 32], // struct ipq (packed)
    ip_id: u16,
    bootp_clients: [BootpClient; 16],
    bootp_filename: *mut i8,
    tcb: [u8; 168], // struct socket
    tcp_last_so: *mut c_void,
    tcp_iss: u32,
    tcp_now: u32,
    udb: [u8; 168], // struct socket
    udp_last_so: *mut c_void,
    tftp_prefix: *mut i8,
    tftp_sessions: [TftpSession; 3],
    opaque: *mut c_void,
}

const SPECIAL_ETHADDR: [u8; ETH_ALEN] = [0x52, 0x55, 0x00, 0x00, 0x00, 0x00];
const ZERO_ETHADDR: [u8; ETH_ALEN] = [0, 0, 0, 0, 0, 0];

#[inline]
fn htons(v: u16) -> u16 {
    v.to_be()
}

#[no_mangle]
pub extern "C" fn if_encap(slirp: *mut Slirp, ip_data: *const u8, ip_data_len: c_int) {
    let mut buf = [0u8; BUF_SIZE];

    unsafe {
        if ip_data_len as i64 + ETH_HLEN as i64 > BUF_SIZE as i64 {
            return;
        }
        if slirp.is_null() || ip_data.is_null() {
            return;
        }

        let s = &mut *slirp;
        let eh = buf.as_mut_ptr() as *mut EthHdr;

        if memcmp(
            s.client_ethaddr.as_ptr() as *const c_void,
            ZERO_ETHADDR.as_ptr() as *const c_void,
            ETH_ALEN,
        ) == 0
        {
            // client MAC unknown: send an ARP request instead of the packet
            let mut arp_req = [0u8; ETH_HLEN + 28];
            let reh = arp_req.as_mut_ptr() as *mut EthHdr;
            let rah = arp_req.as_mut_ptr().add(ETH_HLEN) as *mut ArpHdr;
            // packed IPv4 header, ip_dst is at offset 16
            let iph_dst = ip_data.add(16) as *const u32;

            memset((*reh).h_dest.as_mut_ptr() as *mut c_void, 0xff, ETH_ALEN);
            memcpy(
                (*reh).h_source.as_mut_ptr() as *mut c_void,
                SPECIAL_ETHADDR.as_ptr() as *const c_void,
                ETH_ALEN - 4,
            );
            memcpy(
                (*reh).h_source.as_mut_ptr().add(2) as *mut c_void,
                &s.vhost_addr as *const InAddr as *const c_void,
                4,
            );
            (*reh).h_proto = htons(ETH_P_ARP);

            (*rah).ar_hrd = htons(1);
            (*rah).ar_pro = htons(ETH_P_IP);
            (*rah).ar_hln = ETH_ALEN as u8;
            (*rah).ar_pln = 4;
            (*rah).ar_op = htons(ARPOP_REQUEST);
            // source hw addr
            memcpy(
                (*rah).ar_sha.as_mut_ptr() as *mut c_void,
                SPECIAL_ETHADDR.as_ptr() as *const c_void,
                ETH_ALEN - 4,
            );
            memcpy(
                (*rah).ar_sha.as_mut_ptr().add(2) as *mut c_void,
                &s.vhost_addr as *const InAddr as *const c_void,
                4,
            );
            // source IP
            (*rah).ar_sip = s.vhost_addr.s_addr;
            // target hw addr (none)
            memset((*rah).ar_tha.as_mut_ptr() as *mut c_void, 0, ETH_ALEN);
            // target IP
            let dst = core::ptr::read_unaligned(iph_dst);
            (*rah).ar_tip = dst;
            s.client_ipaddr.s_addr = dst;
            slirp_output(
                s.opaque,
                arp_req.as_ptr(),
                arp_req.len() as c_int,
            );
        } else {
            memcpy(
                (*eh).h_dest.as_mut_ptr() as *mut c_void,
                s.client_ethaddr.as_ptr() as *const c_void,
                ETH_ALEN,
            );
            memcpy(
                (*eh).h_source.as_mut_ptr() as *mut c_void,
                SPECIAL_ETHADDR.as_ptr() as *const c_void,
                ETH_ALEN - 4,
            );
            // XXX: not correct
            memcpy(
                (*eh).h_source.as_mut_ptr().add(2) as *mut c_void,
                &s.vhost_addr as *const InAddr as *const c_void,
                4,
            );
            (*eh).h_proto = htons(ETH_P_IP);
            memcpy(
                buf.as_mut_ptr().add(core::mem::size_of::<EthHdr>()) as *mut c_void,
                ip_data as *const c_void,
                ip_data_len as usize,
            );
            slirp_output(s.opaque, buf.as_ptr(), ip_data_len + ETH_HLEN as c_int);
        }
    }
}
}

// ── `if (ip_data_len + 14 > sizeof(buf)` · slirp/slirp.c:727-728 · chunk 2534 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ip_data_len + 14 > sizeof(buf))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2534 {
// Port of the early-exit guard of if_encap():
//     uint8_t buf[1600];
//     if (ip_data_len + 14 > sizeof(buf)) return;
//
// Returns nonzero when the caller must return immediately.
// C semantics: `ip_data_len + 14` is computed as an `int` (wrapping),
// then converted to `size_t` for the comparison against sizeof(buf) == 1600,
// so a negative value becomes a huge unsigned value.

use core::ffi::c_int;

const BUF_SIZE: usize = 1600;

#[no_mangle]
pub extern "C" fn if_encap__part2534(ip_data_len: c_int) -> c_int {
    let sum: c_int = ip_data_len.wrapping_add(14);
    // int -> size_t conversion, matching C's sign extension.
    let as_size: usize = sum as usize;
    if as_size > BUF_SIZE {
        1
    } else {
        0
    }
}
}

// ── `if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6)) … else` · slirp/slirp.c:730-769 · chunk 2535 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6)) {
//             uint8_t arp_req[14 + sizeof(struct arphdr)];
//             struct ethhdr *reh = (struct ethhdr *)arp_req;
//             struct arphdr *rah = (struct arphdr *)(arp_req + 14);
//             const struct ip *iph = (const struct ip *)ip_data;
//     
//     
//     
//     
//     
//     
//             memset(reh->h_dest, 0xff, 6);
//             memcpy(reh->h_source, special_ethaddr, 6 - 4);
//             memcpy(&reh->h_source[2], &slirp->vhost_addr, 4);
//             reh->h_proto = htons(0x0806);
//             rah->ar_hrd = htons(1);
//             rah->ar_pro = htons(0x0800);
//             rah->ar_hln = 6;
//             rah->ar_pln = 4;
//             rah->ar_op = htons(1);
//     
//             memcpy(rah->ar_sha, special_ethaddr, 6 - 4);
//             memcpy(&rah->ar_sha[2], &slirp->vhost_addr, 4);
//     
//             rah->ar_sip = slirp->vhost_addr.s_addr;
//     
//             memset(rah->ar_tha, 0, 6);
//     
//             rah->ar_tip = iph->ip_dst.s_addr;
//             slirp->client_ipaddr = iph->ip_dst;
//             slirp_output(slirp->opaque, arp_req, sizeof(arp_req));
//         } else {
//             memcpy(eh->h_dest, slirp->client_ethaddr, 6);
//             memcpy(eh->h_source, special_ethaddr, 6 - 4);
//     
//             memcpy(&eh->h_source[2], &slirp->vhost_addr, 4);
//             eh->h_proto = htons(0x0800);
//             memcpy(buf + sizeof(struct ethhdr), ip_data, ip_data_len);
//             slirp_output(slirp->opaque, buf, ip_data_len + 14);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2535 {
// Port of the body of if_encap: sends an ARP request when the client MAC is
// still unknown, otherwise wraps the IP packet into an Ethernet frame and
// hands it to slirp_output.

use core::ffi::{c_int, c_void};

extern "C" {
    fn slirp_output(opaque: *mut c_void, pkt: *const u8, pkt_len: c_int);
}

// First two bytes of the static special_ethaddr[] (6 - 4 == 2).
const SPECIAL_ETHADDR: [u8; 2] = [0x52, 0x55];

const ARPHDR_SIZE: usize = 28;
const ARP_REQ_SIZE: usize = 14 + ARPHDR_SIZE;

#[inline]
fn htons(x: u16) -> u16 {
    if cfg!(target_endian = "little") {
        x.swap_bytes()
    } else {
        x
    }
}

#[inline]
fn put_u16(dst: *mut u8, off: usize, v: u16) {
    unsafe {
        core::ptr::copy_nonoverlapping(v.to_ne_bytes().as_ptr(), dst.add(off), 2);
    }
}

#[no_mangle]
pub extern "C" fn if_encap__part2535(
    client_ethaddr: *mut u8,
    vhost_addr: *const u8,
    client_ipaddr: *mut u8,
    opaque: *mut c_void,
    ip_data: *const u8,
    ip_data_len: c_int,
    buf: *mut u8,
    _eh: *mut u8,
) {
    unsafe {
        // !memcmp(slirp->client_ethaddr, zero_ethaddr, 6)
        let mut is_zero = true;
        let mut i: usize = 0;
        while i < 6 {
            if *client_ethaddr.add(i) != 0 {
                is_zero = false;
            }
            i += 1;
        }

        if is_zero {
            let mut arp_req = [0u8; ARP_REQ_SIZE];
            let a: *mut u8 = arp_req.as_mut_ptr();

            // struct ethhdr at offset 0
            // memset(reh->h_dest, 0xff, 6);
            let mut k: usize = 0;
            while k < 6 {
                *a.add(k) = 0xff;
                k += 1;
            }
            // memcpy(reh->h_source, special_ethaddr, 6 - 4);
            core::ptr::copy_nonoverlapping(SPECIAL_ETHADDR.as_ptr(), a.add(6), 2);
            // memcpy(&reh->h_source[2], &slirp->vhost_addr, 4);
            core::ptr::copy_nonoverlapping(vhost_addr, a.add(8), 4);
            // reh->h_proto = htons(0x0806);
            put_u16(a, 12, htons(0x0806));

            // struct arphdr at offset 14 (packed)
            put_u16(a, 14, htons(1)); // ar_hrd
            put_u16(a, 16, htons(0x0800)); // ar_pro
            *a.add(18) = 6; // ar_hln
            *a.add(19) = 4; // ar_pln
            put_u16(a, 20, htons(1)); // ar_op

            // memcpy(rah->ar_sha, special_ethaddr, 6 - 4);
            core::ptr::copy_nonoverlapping(SPECIAL_ETHADDR.as_ptr(), a.add(22), 2);
            // memcpy(&rah->ar_sha[2], &slirp->vhost_addr, 4);
            core::ptr::copy_nonoverlapping(vhost_addr, a.add(24), 4);

            // rah->ar_sip = slirp->vhost_addr.s_addr;
            core::ptr::copy_nonoverlapping(vhost_addr, a.add(28), 4);

            // memset(rah->ar_tha, 0, 6); -- already zeroed

            // rah->ar_tip = iph->ip_dst.s_addr; (ip_dst is at offset 16)
            core::ptr::copy_nonoverlapping(ip_data.add(16), a.add(38), 4);
            // slirp->client_ipaddr = iph->ip_dst;
            core::ptr::copy_nonoverlapping(ip_data.add(16), client_ipaddr, 4);

            slirp_output(opaque, a as *const u8, ARP_REQ_SIZE as c_int);
        } else {
            // struct ethhdr over buf
            core::ptr::copy_nonoverlapping(client_ethaddr, buf, 6);
            core::ptr::copy_nonoverlapping(SPECIAL_ETHADDR.as_ptr(), buf.add(6), 2);
            core::ptr::copy_nonoverlapping(vhost_addr, buf.add(8), 4);
            put_u16(buf, 12, htons(0x0800));
            core::ptr::copy_nonoverlapping(ip_data, buf.add(14), ip_data_len as usize);
            slirp_output(opaque, buf as *const u8, ip_data_len + 14);
        }
    }
}
}

// ── `slirp_remove_hostfwd` · slirp/slirp.c:773-795 · chunk 2536 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int slirp_remove_hostfwd(Slirp *slirp, int is_udp, struct in_addr host_addr,
//                              int host_port)
//     {
//         struct socket *so;
//         struct socket *head = (is_udp ? &slirp->udb : &slirp->tcb);
//         struct sockaddr_in addr;
//         int port = htons(host_port);
//         socklen_t addr_len;
//     
//         for (so = head->so_next; so != head; so = so->so_next) {
//             addr_len = sizeof(addr);
//             if ((so->so_state & 0x1000) &&
//                 getsockname(so->s, (struct sockaddr *)&addr, &addr_len) == 0 &&
//                 addr.sin_addr.s_addr == host_addr.s_addr &&
//                 addr.sin_port == port) {
//                 close(so->s);
//                 sofree(so);
//                 return 0;
//             }
//         }
//     
//         return -1;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2536 {
// Rust rewrite of slirp_remove_hostfwd() from slirp/slirp.c
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};

/* ---------- basic networking types (mirror of <netinet/in.h>) ---------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

pub type socklen_t = u32;

/* ---------- slirp structures ---------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

/* opaque types only ever used through pointers here */
#[repr(C)]
pub struct tcpiphdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcpcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ex_list {
    _private: [u8; 0],
}

/* struct mbuf; (mbuf.h) */
#[repr(C)]
pub union M_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: M_dat,
}

/* struct sbuf; (sbuf.h) */
#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,      /* u_int */
    pub sb_datalen: u32, /* u_int */
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

/* struct socket; (socket.h) */
#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut tcpiphdr,
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8, /* u_char */
    pub so_state: c_int,
    pub so_tcpcb: *mut tcpcb,
    pub so_expire: u32, /* u_int */
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

/* struct qlink { void *next, *prev; }; */
#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

/* struct ipq (packed) */
#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

/* BOOTPClient */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

/* struct tftp_session */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

/* struct Slirp; (slirp.h) */
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: in_addr,
    pub vnetwork_mask: in_addr,
    pub vhost_addr: in_addr,
    pub vdhcp_startaddr: in_addr,
    pub vnameserver_addr: in_addr,

    pub client_ethaddr: [u8; 6],

    pub client_ipaddr: in_addr,
    pub client_hostname: [c_char; 33],

    pub restricted: c_int,
    pub tt: timeval,
    pub exec_list: *mut ex_list,

    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
    pub mbuf_alloced: c_int,

    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,

    pub ipq: ipq,
    pub ip_id: u16,

    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,

    pub tcb: socket,
    pub tcp_last_so: *mut socket,
    pub tcp_iss: u32, /* tcp_seq */
    pub tcp_now: u32,

    pub udb: socket,
    pub udp_last_so: *mut socket,

    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],

    pub opaque: *mut c_void,
}

/* ---------- C functions used ---------- */

extern "C" {
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn sofree(so: *mut socket);
}

/* ---------- the ported function ---------- */

/* Drop host forwarding rule, return 0 if found. */
#[no_mangle]
pub extern "C" fn slirp_remove_hostfwd(
    slirp: *mut Slirp,
    is_udp: c_int,
    host_addr: in_addr,
    host_port: c_int,
) -> c_int {
    unsafe {
        let head: *mut socket = if is_udp != 0 {
            core::ptr::addr_of_mut!((*slirp).udb)
        } else {
            core::ptr::addr_of_mut!((*slirp).tcb)
        };
        let port: c_int = htons(host_port as u16) as c_int;

        let mut so: *mut socket = (*head).so_next;
        while so != head {
            let mut addr: sockaddr_in = core::mem::zeroed();
            let mut addr_len: socklen_t = core::mem::size_of::<sockaddr_in>() as socklen_t;

            if ((*so).so_state & 0x1000) != 0
                && getsockname(
                    (*so).s,
                    &mut addr as *mut sockaddr_in as *mut sockaddr,
                    &mut addr_len,
                ) == 0
                && addr.sin_addr.s_addr == host_addr.s_addr
                && (addr.sin_port as c_int) == port
            {
                close((*so).s);
                sofree(so);
                return 0;
            }

            so = (*so).so_next;
        }

        -1
    }
}
}

// ── `for (so = head->so_next; so != head; so = so->so_next)` · slirp/slirp.c:782-792 · chunk 2537 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (so = head->so_next; so != head; so = so->so_next) {
//             addr_len = sizeof(addr);
//             if ((so->so_state & 0x1000) &&
//                 getsockname(so->s, (struct sockaddr *)&addr, &addr_len) == 0 &&
//                 addr.sin_addr.s_addr == host_addr.s_addr &&
//                 addr.sin_port == port) {
//                 close(so->s);
//                 sofree(so);
//                 return 0;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2537 {
// Port of the socket-search loop from slirp_remove_hostfwd().

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

extern "C" {
    fn getsockname(fd: c_int, addr: *mut SockaddrIn, len: *mut u32) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sofree(so: *mut Socket);
}

/// Returns 0 when the matching host-forward socket was found, freed and
/// closed; returns -1 when no socket matched.
#[no_mangle]
pub extern "C" fn slirp_remove_hostfwd__part2537(
    head: *mut Socket,
    host_addr_s_addr: u32,
    port: c_int,
) -> c_int {
    if head.is_null() {
        return -1;
    }

    let mut so: *mut Socket = unsafe { (*head).so_next };

    while so != head {
        let mut addr: SockaddrIn = SockaddrIn {
            sin_family: 0,
            sin_port: 0,
            sin_addr: InAddr { s_addr: 0 },
            sin_zero: [0u8; 8],
        };
        let mut addr_len: u32 = core::mem::size_of::<SockaddrIn>() as u32;

        let sock = unsafe { (*so).s };
        let state = unsafe { (*so).so_state };

        if (state & 0x1000) != 0
            && unsafe { getsockname(sock, &mut addr, &mut addr_len) } == 0
            && addr.sin_addr.s_addr == host_addr_s_addr
            && (addr.sin_port as c_int) == port
        {
            unsafe {
                close(sock);
                sofree(so);
            }
            return 0;
        }

        so = unsafe { (*so).so_next };
    }

    -1
}
}

// ── `return -1;` · slirp/slirp.c:794-794 · chunk 2538 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2538 {
// Port of the fall-through "return -1;" of slirp_remove_hostfwd: the search
// over the socket list found no matching host forwarding entry.
#[no_mangle]
pub extern "C" fn slirp_remove_hostfwd__part2538() -> core::ffi::c_int {
    -1
}
}

// ── `slirp_add_hostfwd` · slirp/slirp.c:797-813 · chunk 2539 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int slirp_add_hostfwd(Slirp *slirp, int is_udp, struct in_addr host_addr,
//                           int host_port, struct in_addr guest_addr, int guest_port)
//     {
//         if (!guest_addr.s_addr) {
//             guest_addr = slirp->vdhcp_startaddr;
//         }
//         if (is_udp) {
//             if (!udp_listen(slirp, host_addr.s_addr, htons(host_port),
//                             guest_addr.s_addr, htons(guest_port), 0x1000))
//                 return -1;
//         } else {
//             if (!tcp_listen(slirp, host_addr.s_addr, htons(host_port),
//                             guest_addr.s_addr, htons(guest_port), 0x1000))
//                 return -1;
//         }
//         return 0;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2539 {
// Rust rewrite of slirp_add_hostfwd from slirp/slirp.c
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

// Prefix of the C `struct Slirp`: only the leading fields needed here.
// Offsets match the C layout, so pointer casts are valid.
#[repr(C)]
pub struct SlirpPrefix {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
}

extern "C" {
    fn udp_listen(
        slirp: *mut SlirpPrefix,
        haddr: u32,
        hport: u32,
        gaddr: u32,
        gport: u32,
        flags: c_int,
    ) -> *mut c_void;
    fn tcp_listen(
        slirp: *mut SlirpPrefix,
        haddr: u32,
        hport: u32,
        gaddr: u32,
        gport: u32,
        flags: c_int,
    ) -> *mut c_void;
}

const SS_HOSTFWD: c_int = 0x1000;

#[inline]
fn htons(x: c_int) -> u32 {
    // C: htons(int) takes the low 16 bits, returns uint16_t, then widened to u_int.
    (x as u16).to_be() as u32
}

#[no_mangle]
pub extern "C" fn slirp_add_hostfwd(
    slirp: *mut SlirpPrefix,
    is_udp: c_int,
    host_addr: InAddr,
    host_port: c_int,
    mut guest_addr: InAddr,
    guest_port: c_int,
) -> c_int {
    if guest_addr.s_addr == 0 {
        guest_addr = unsafe { (*slirp).vdhcp_startaddr };
    }
    if is_udp != 0 {
        let so = unsafe {
            udp_listen(
                slirp,
                host_addr.s_addr,
                htons(host_port),
                guest_addr.s_addr,
                htons(guest_port),
                SS_HOSTFWD,
            )
        };
        if so.is_null() {
            return -1;
        }
    } else {
        let so = unsafe {
            tcp_listen(
                slirp,
                host_addr.s_addr,
                htons(host_port),
                guest_addr.s_addr,
                htons(guest_port),
                SS_HOSTFWD,
            )
        };
        if so.is_null() {
            return -1;
        }
    }
    0
}
}

// ── `if (!guest_addr.s_addr)` · slirp/slirp.c:800-802 · chunk 2540 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!guest_addr.s_addr) {
//             guest_addr = slirp->vdhcp_startaddr;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2540 {
// Port of the guest_addr defaulting part of slirp_add_hostfwd (slirp/slirp.c).
// If no guest address was given, fall back to the DHCP start address.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

// Prefix of `struct Slirp` (all fields are 4-byte aligned `struct in_addr` up to
// and including vdhcp_startaddr, which therefore lives at offset 12).
#[repr(C)]
pub struct SlirpPrefix {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
}

#[no_mangle]
pub unsafe extern "C" fn slirp_add_hostfwd__part2540(
    slirp: *mut SlirpPrefix,
    guest_addr: InAddr,
) -> InAddr {
    let mut guest_addr = guest_addr;
    if guest_addr.s_addr == 0 {
        guest_addr = (*slirp).vdhcp_startaddr;
    }
    guest_addr
}
}

// ── `if (is_udp) … else` · slirp/slirp.c:803-811 · chunk 2541 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (is_udp) {
//             if (!udp_listen(slirp, host_addr.s_addr, htons(host_port),
//                             guest_addr.s_addr, htons(guest_port), 0x1000))
//                 return -1;
//         } else {
//             if (!tcp_listen(slirp, host_addr.s_addr, htons(host_port),
//                             guest_addr.s_addr, htons(guest_port), 0x1000))
//                 return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2541 {
use core::ffi::{c_int, c_uint, c_void};

extern "C" {
    fn htons(__hostshort: u16) -> u16;
    fn udp_listen(
        slirp: *mut c_void,
        src: u32,
        sport: c_uint,
        dst: u32,
        dport: c_uint,
        flags: c_int,
    ) -> *mut c_void;
    fn tcp_listen(
        slirp: *mut c_void,
        src: u32,
        sport: c_uint,
        dst: u32,
        dport: c_uint,
        flags: c_int,
    ) -> *mut c_void;
}

// Mirrors the tail of slirp_add_hostfwd: picks udp_listen or tcp_listen,
// returns -1 when the listen call fails, 0 otherwise.
#[no_mangle]
pub extern "C" fn slirp_add_hostfwd__part2541(
    slirp: *mut c_void,
    is_udp: c_int,
    host_s_addr: u32,
    host_port: c_int,
    guest_s_addr: u32,
    guest_port: c_int,
) -> c_int {
    unsafe {
        let sport: c_uint = htons(host_port as u16) as c_uint;
        let dport: c_uint = htons(guest_port as u16) as c_uint;
        let ok = if is_udp != 0 {
            !udp_listen(slirp, host_s_addr, sport, guest_s_addr, dport, 0x1000).is_null()
        } else {
            !tcp_listen(slirp, host_s_addr, sport, guest_s_addr, dport, 0x1000).is_null()
        };
        if ok {
            0
        } else {
            -1
        }
    }
}
}

// ── `return 0;` · slirp/slirp.c:812-812 · chunk 2542 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2542 {
// Ported piece of slirp_add_hostfwd: the trailing `return 0;`.
// The function reports success unconditionally at this point.

#[no_mangle]
pub extern "C" fn slirp_add_hostfwd__part2542() -> core::ffi::c_int {
    0
}
}

// ── `slirp_add_exec` · slirp/slirp.c:815-830 · chunk 2543 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int slirp_add_exec(Slirp *slirp, int do_pty, const void *args,
//                        struct in_addr *guest_addr, int guest_port)
//     {
//         if (!guest_addr->s_addr) {
//             guest_addr->s_addr = slirp->vnetwork_addr.s_addr |
//                 (htonl(0x0204) & ~slirp->vnetwork_mask.s_addr);
//         }
//         if ((guest_addr->s_addr & slirp->vnetwork_mask.s_addr) !=
//             slirp->vnetwork_addr.s_addr ||
//             guest_addr->s_addr == slirp->vhost_addr.s_addr ||
//             guest_addr->s_addr == slirp->vnameserver_addr.s_addr) {
//             return -1;
//         }
//         return add_exec(&slirp->exec_list, do_pty, (char *)args, *guest_addr,
//                         htons(guest_port));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2543 {
// Rust rewrite of slirp_add_exec (slirp/slirp.c).
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: core::ffi::c_long,
    pub tv_usec: core::ffi::c_long,
}

// Mirror of `struct Slirp` (slirp/slirp.h) up to the fields this function uses.
// Fields after `exec_list` are not accessed, so the struct is only required to
// match in the accessed prefix; all preceding fields keep exact layout.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut c_void,
}

extern "C" {
    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn add_exec(
        ex_ptr: *mut *mut c_void,
        do_pty: c_int,
        exec: *mut c_char,
        addr: InAddr,
        port: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn slirp_add_exec(
    slirp: *mut Slirp,
    do_pty: c_int,
    args: *const c_void,
    guest_addr: *mut InAddr,
    guest_port: c_int,
) -> c_int {
    let guest = guest_addr;
    unsafe {
        if (*guest).s_addr == 0 {
            (*guest).s_addr = (*slirp).vnetwork_addr.s_addr
                | (htonl(0x0204) & !(*slirp).vnetwork_mask.s_addr);
        }
        if ((*guest).s_addr & (*slirp).vnetwork_mask.s_addr) != (*slirp).vnetwork_addr.s_addr
            || (*guest).s_addr == (*slirp).vhost_addr.s_addr
            || (*guest).s_addr == (*slirp).vnameserver_addr.s_addr
        {
            return -1;
        }
        add_exec(
            &mut (*slirp).exec_list as *mut *mut c_void,
            do_pty,
            args as *mut c_char,
            *guest,
            htons(guest_port as u16) as c_int,
        )
    }
}
}

// ── `if (!guest_addr->s_addr)` · slirp/slirp.c:818-821 · chunk 2544 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!guest_addr->s_addr) {
//             guest_addr->s_addr = slirp->vnetwork_addr.s_addr |
//                 (htonl(0x0204) & ~slirp->vnetwork_mask.s_addr);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2544 {
// Port of the guest address defaulting step of slirp_add_exec.
use core::ffi::c_uint;

#[repr(C)]
pub struct InAddr {
    pub s_addr: c_uint,
}

extern "C" {
    fn htonl(__hostlong: c_uint) -> c_uint;
}

#[no_mangle]
pub extern "C" fn slirp_add_exec__part2544(
    guest_addr: *mut InAddr,
    vnetwork_addr: c_uint,
    vnetwork_mask: c_uint,
) {
    unsafe {
        if (*guest_addr).s_addr == 0 {
            (*guest_addr).s_addr = vnetwork_addr | (htonl(0x0204) & !vnetwork_mask);
        }
    }
}
}

// ── `if (guest_addr->s_addr & slirp->vnetwork_mask.s_addr) != slirp->vnetwork_addr.s_addr || guest_a…` · slirp/slirp.c:822-827 · chunk 2545 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((guest_addr->s_addr & slirp->vnetwork_mask.s_addr) !=
//             slirp->vnetwork_addr.s_addr ||
//             guest_addr->s_addr == slirp->vhost_addr.s_addr ||
//             guest_addr->s_addr == slirp->vnameserver_addr.s_addr) {
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2545 {
// Port of the guest address validation part of slirp_add_exec.

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

// Mirrors the prefix of `struct Slirp` needed here; field order and types match
// the C definition so offsets are identical.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
}

/// Returns 0 when the guest address is valid, -1 when it must be rejected.
#[no_mangle]
pub unsafe extern "C" fn slirp_add_exec__part2545(
    slirp: *const Slirp,
    guest_addr: *const InAddr,
) -> core::ffi::c_int {
    if slirp.is_null() || guest_addr.is_null() {
        return -1;
    }
    let s = &*slirp;
    let g = (*guest_addr).s_addr;

    if (g & s.vnetwork_mask.s_addr) != s.vnetwork_addr.s_addr
        || g == s.vhost_addr.s_addr
        || g == s.vnameserver_addr.s_addr
    {
        return -1;
    }
    0
}
}

// ── `return add_exec(&slirp->exec_list, do_pty, (char *)args, *guest_addr, htons(guest_port));` · slirp/slirp.c:828-829 · chunk 2546 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     return add_exec(&slirp->exec_list, do_pty, (char *)args, *guest_addr,
//                         htons(guest_port));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2546 {
// Port of the tail of slirp_add_exec: delegate to add_exec().
use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct ExList {
    pub ex_pty: c_int,
    pub ex_addr: InAddr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ExList,
}

// Prefix of `struct Slirp` up to and including `exec_list`.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut ExList,
}

extern "C" {
    fn add_exec(
        list: *mut *mut ExList,
        do_pty: c_int,
        args: *mut c_char,
        addr: InAddr,
        port: c_int,
    ) -> c_int;
    fn htons(hostshort: u16) -> u16;
}

#[no_mangle]
pub extern "C" fn slirp_add_exec__part2546(
    slirp: *mut Slirp,
    do_pty: c_int,
    args: *const c_void,
    guest_addr: *const InAddr,
    guest_port: c_int,
) -> c_int {
    unsafe {
        let addr: InAddr = *guest_addr;
        let port: c_int = htons(guest_port as u16) as c_int;
        add_exec(&mut (*slirp).exec_list, do_pty, args as *mut c_char, addr, port)
    }
}
}

// ── `slirp_send` · slirp/slirp.c:832-841 · chunk 2547 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ssize_t slirp_send(struct socket *so, const void *buf, size_t len, int flags)
//     {
//     
//     
//     
//     
//     
//     
//      return send(so->s, buf, len, flags);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2547 {
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
struct InAddr {
    s_addr: u32,
}

#[repr(C)]
struct SBuf {
    sb_cc: c_uint,
    sb_datalen: c_uint,
    sb_wptr: *mut c_char,
    sb_rptr: *mut c_char,
    sb_data: *mut c_char,
}

#[repr(C)]
struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
    slirp: *mut c_void,
    so_m: *mut c_void,
    so_ti: *mut c_void,
    so_urgc: c_int,
    so_faddr: InAddr,
    so_laddr: InAddr,
    so_fport: u16,
    so_lport: u16,
    so_iptos: u8,
    so_emu: u8,
    so_type: u8,
    so_state: c_int,
    so_tcpcb: *mut c_void,
    so_expire: c_uint,
    so_queued: c_int,
    so_nqueued: c_int,
    so_rcv: SBuf,
    so_snd: SBuf,
    extra: *mut c_void,
}

extern "C" {
    fn send(fd: c_int, buf: *const c_void, n: usize, flags: c_int) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn slirp_send(
    so: *mut Socket,
    buf: *const c_void,
    len: usize,
    flags: c_int,
) -> isize {
    send((*so).s, buf, len, flags)
}
}

// ── `return send(so->s, buf, len, flags);` · slirp/slirp.c:840-840 · chunk 2548 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return send(so->s, buf, len, flags);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2548 {
use core::ffi::{c_int, c_long, c_void};

// Mirror of the prefix of `struct socket` needed to read the `s` field.
#[repr(C)]
pub struct Socket {
    so_next: *mut Socket,
    so_prev: *mut Socket,
    s: c_int,
}

extern "C" {
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> c_long;
}

#[no_mangle]
pub extern "C" fn slirp_send__part2548(
    so: *mut Socket,
    buf: *const c_void,
    len: usize,
    flags: c_int,
) -> c_long {
    let fd = unsafe { (*so).s };
    unsafe { send(fd, buf, len, flags) }
}
}

// ── `slirp_find_ctl_socket` · slirp/slirp.c:843-855 · chunk 2549 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static struct socket *
//     slirp_find_ctl_socket(Slirp *slirp, struct in_addr guest_addr, int guest_port)
//     {
//         struct socket *so;
//     
//         for (so = slirp->tcb.so_next; so != &slirp->tcb; so = so->so_next) {
//             if (so->so_faddr.s_addr == guest_addr.s_addr &&
//                 htons(so->so_fport) == guest_port) {
//                 return so;
//             }
//         }
//         return 
//               ((void *)0)
//                   ;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2549 {
// Rust rewrite of static struct socket *slirp_find_ctl_socket(Slirp *, struct in_addr, int)
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// struct in_addr { in_addr_t s_addr; } with in_addr_t = uint32_t
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct ex_list {
    pub ex_pty: c_int,
    pub ex_addr: in_addr,
    pub ex_fport: c_int,
    pub ex_exec: *const c_char,
    pub ex_next: *mut ex_list,
}

// struct m_hdr (field order/type sizes as in slirp's mbuf.h)
#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct mbuf_dat {
    pub m_dat_: [c_char; 1],
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: mbuf_dat,
}

#[repr(C)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

#[repr(C, packed)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

#[repr(C)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: in_addr,
    pub vnetwork_mask: in_addr,
    pub vhost_addr: in_addr,
    pub vdhcp_startaddr: in_addr,
    pub vnameserver_addr: in_addr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: in_addr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: timeval,
    pub exec_list: *mut ex_list,
    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    pub ipq: ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: socket,
    pub tcp_last_so: *mut socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: socket,
    pub udp_last_so: *mut socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn slirp_find_ctl_socket(
    slirp: *mut Slirp,
    guest_addr: in_addr,
    guest_port: c_int,
) -> *mut socket {
    // Non-null assumption: the C caller always passes a valid Slirp.
    let head: *mut socket = core::ptr::addr_of_mut!((*slirp).tcb);
    let mut so: *mut socket = (*slirp).tcb.so_next;

    while so != head {
        if (*so).so_faddr.s_addr == guest_addr.s_addr
            && htons((*so).so_fport) as c_int == guest_port
        {
            return so;
        }
        so = (*so).so_next;
    }
    core::ptr::null_mut()
}
}

// ── `for (so = slirp->tcb.so_next; so != &slirp->tcb; so = so->so_next)` · slirp/slirp.c:848-853 · chunk 2550 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (so = slirp->tcb.so_next; so != &slirp->tcb; so = so->so_next) {
//             if (so->so_faddr.s_addr == guest_addr.s_addr &&
//                 htons(so->so_fport) == guest_port) {
//                 return so;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2550 {
// Port of the TCP control-socket lookup loop from slirp_find_ctl_socket().
// The loop walks the socket list anchored at the tcb head node and returns the
// socket whose foreign address/port match, or NULL when the list is exhausted.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

#[no_mangle]
pub unsafe extern "C" fn slirp_find_ctl_socket__part2550(
    head: *mut Socket,
    guest_addr: InAddr,
    guest_port: c_int,
) -> *mut Socket {
    let mut so: *mut Socket = (*head).so_next;
    while so != head {
        if (*so).so_faddr.s_addr == guest_addr.s_addr
            && htons((*so).so_fport) as c_int == guest_port
        {
            return so;
        }
        so = (*so).so_next;
    }
    core::ptr::null_mut()
}
}

// ── `return ((void *)0) ;` · slirp/slirp.c:854-854 · chunk 2551 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2551 {
// Rust rewrite of the `return NULL;` tail of slirp_find_ctl_socket().
// Returned pointer type mirrors the C `struct socket *`; only null is produced.

#[repr(C)]
pub struct socket {
    _opaque: [u8; 0],
}

#[no_mangle]
pub extern "C" fn slirp_find_ctl_socket__part2551() -> *mut socket {
    core::ptr::null_mut()
}
}

// ── `slirp_socket_can_recv` · slirp/slirp.c:857-872 · chunk 2552 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     size_t slirp_socket_can_recv(Slirp *slirp, struct in_addr guest_addr,
//                                  int guest_port)
//     {
//      struct iovec iov[2];
//      struct socket *so;
//     
//      so = slirp_find_ctl_socket(slirp, guest_addr, guest_port);
//     
//      if (!so || so->so_state & 0x001)
//       return 0;
//     
//      if (!(((so)->so_state & (0x008|0x004)) == 0x004) || so->so_snd.sb_cc >= (so->so_snd.sb_datalen/2))
//       return 0;
//     
//      return sopreprbuf(so, iov, 
//                                ((void *)0)
//                                    );
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2552 {
#![allow(non_snake_case, non_camel_case_types, dead_code, non_upper_case_globals)]

// Rewrite of slirp/slirp.c: slirp_socket_can_recv (with its static helper
// slirp_find_ctl_socket folded in as a private Rust function).

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QLink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// struct ipq is declared __attribute__((packed))
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ipq {
    pub frag_link: QLink,
    pub ip_link: QLink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
pub struct MbufHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MbufHdr,
    pub slirp: *mut Slirp,
    pub M_dat: MDat,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct TftpSession {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: InAddr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: TimeVal,
    pub exec_list: *mut c_void, // struct ex_list *
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: Ipq,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
    pub tcp_last_so: *mut Socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: Socket,
    pub udp_last_so: *mut Socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [TftpSession; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn sopreprbuf(so: *mut Socket, iov: *mut Iovec, np: *mut c_int) -> usize;
}

// uint16_t htons(uint16_t) -- no dependency on the C library symbol.
#[inline]
fn htons(x: u16) -> u16 {
    if cfg!(target_endian = "big") {
        x
    } else {
        x.swap_bytes()
    }
}

// static struct socket *slirp_find_ctl_socket(Slirp *, struct in_addr, int)
fn slirp_find_ctl_socket(
    slirp: *mut Slirp,
    guest_addr: InAddr,
    guest_port: c_int,
) -> *mut Socket {
    unsafe {
        let head: *mut Socket = core::ptr::addr_of_mut!((*slirp).tcb);
        let mut so: *mut Socket = (*slirp).tcb.so_next;
        while so != head {
            if (*so).so_faddr.s_addr == guest_addr.s_addr
                && htons((*so).so_fport) as c_int == guest_port
            {
                return so;
            }
            so = (*so).so_next;
        }
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn slirp_socket_can_recv(
    slirp: *mut Slirp,
    guest_addr: InAddr,
    guest_port: c_int,
) -> usize {
    let mut iov: [Iovec; 2] = [
        Iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
        Iovec {
            iov_base: core::ptr::null_mut(),
            iov_len: 0,
        },
    ];

    let so = slirp_find_ctl_socket(slirp, guest_addr, guest_port);

    unsafe {
        if so.is_null() || ((*so).so_state & 0x001) != 0 {
            return 0;
        }

        // CONN_CANFRCV(so): (so_state & (SS_ISCONNECTED|SS_ISCONNECTING)) == SS_ISCONNECTED
        if ((*so).so_state & (0x008 | 0x004)) != 0x004 {
            return 0;
        }

        let snd = &(*so).so_snd;
        if snd.sb_cc >= snd.sb_datalen / 2 {
            return 0;
        }

        sopreprbuf(so, iov.as_mut_ptr(), core::ptr::null_mut())
    }
}
}

// ── `struct iovec iov[2]; struct socket *so; so = slirp_find_ctl_socket(slirp, guest_addr, guest_por…` · slirp/slirp.c:860-863 · chunk 2553 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     struct iovec iov[2];
//      struct socket *so;
//     
//      so = slirp_find_ctl_socket(slirp, guest_addr, guest_port);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2553 {
// Port of the "find control socket" part of slirp_socket_can_recv.
// The C static helper slirp_find_ctl_socket has no linker symbol, so its
// logic is reproduced here as private Rust code inside the exported function.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct qlink {
    pub next: *mut c_void,
    pub prev: *mut c_void,
}

// C: struct ipq { ... } __attribute__((packed));
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct ipq {
    pub frag_link: qlink,
    pub ip_link: qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: in_addr,
    pub ipq_dst: in_addr,
}

#[repr(C)]
pub struct sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub union m_dat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct m_hdr {
    pub mh_next: *mut mbuf,
    pub mh_prev: *mut mbuf,
    pub mh_nextpkt: *mut mbuf,
    pub mh_prevpkt: *mut mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub struct mbuf {
    pub m_hdr: m_hdr,
    pub slirp: *mut Slirp,
    pub M_dat: m_dat,
}

#[repr(C)]
pub struct socket {
    pub so_next: *mut socket,
    pub so_prev: *mut socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut mbuf,
    pub so_ti: *mut c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: in_addr,
    pub so_laddr: in_addr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void, // struct tcpcb *
    pub so_expire: u32,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: sbuf,
    pub so_snd: sbuf,
    pub extra: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTPClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct tftp_session {
    pub slirp: *mut Slirp,
    pub filename: *mut c_char,
    pub client_ip: in_addr,
    pub client_port: u16,
    pub timestamp: c_int,
}

#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: in_addr,
    pub vnetwork_mask: in_addr,
    pub vhost_addr: in_addr,
    pub vdhcp_startaddr: in_addr,
    pub vnameserver_addr: in_addr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: in_addr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: timeval,
    pub exec_list: *mut c_void, // struct ex_list *
    pub m_freelist: mbuf,
    pub m_usedlist: mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: mbuf,
    pub if_batchq: mbuf,
    pub next_m: *mut mbuf,
    pub ipq: ipq,
    pub ip_id: u16,
    pub bootp_clients: [BOOTPClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: socket,
    pub tcp_last_so: *mut socket,
    pub tcp_iss: u32,
    pub tcp_now: u32,
    pub udb: socket,
    pub udp_last_so: *mut socket,
    pub tftp_prefix: *mut c_char,
    pub tftp_sessions: [tftp_session; 3],
    pub opaque: *mut c_void,
}

extern "C" {
    fn htons(__hostshort: u16) -> u16;
}

// Reproduces the C static helper slirp_find_ctl_socket().
unsafe fn find_ctl_socket(
    slirp: *mut Slirp,
    guest_addr: in_addr,
    guest_port: c_int,
) -> *mut socket {
    let tcb: *mut socket = core::ptr::addr_of_mut!((*slirp).tcb);
    let mut so: *mut socket = (*tcb).so_next;
    while so != tcb {
        if so.is_null() {
            break;
        }
        if (*so).so_faddr.s_addr == guest_addr.s_addr
            && htons((*so).so_fport) as c_int == guest_port
        {
            return so;
        }
        so = (*so).so_next;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn slirp_socket_can_recv__part2553(
    slirp: *mut Slirp,
    guest_addr: in_addr,
    guest_port: c_int,
) -> *mut socket {
    unsafe {
        if slirp.is_null() {
            return core::ptr::null_mut();
        }
        find_ctl_socket(slirp, guest_addr, guest_port)
    }
}
}

// ── `if (!so || so->so_state & 0x001)` · slirp/slirp.c:865-866 · chunk 2554 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!so || so->so_state & 0x001)
//       return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2554 {
// Rewrite of the null/state check at the top of slirp_socket_can_recv().
// Returns 1 when the caller must return 0, 0 otherwise.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct Sbuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut c_void,
}

#[no_mangle]
pub extern "C" fn slirp_socket_can_recv__part2554(so: *mut Socket) -> c_int {
    if so.is_null() {
        return 1;
    }
    unsafe {
        if (*so).so_state & 0x001 != 0 {
            return 1;
        }
    }
    0
}
}

// ── `if (!(((so)->so_state & (0x008|0x004)) == 0x004) || so->so_snd.sb_cc >= (so->so_snd.sb_datalen/…` · slirp/slirp.c:868-869 · chunk 2555 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(((so)->so_state & (0x008|0x004)) == 0x004) || so->so_snd.sb_cc >= (so->so_snd.sb_datalen/2))
//       return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2555 {
// Ported piece of slirp_socket_can_recv: the state/buffer check that
// decides whether the caller must return 0.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct InAddr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct SBuf {
    pub sb_cc: c_uint,
    pub sb_datalen: c_uint,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut c_void,
    pub so_expire: c_uint,
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: SBuf,
    pub so_snd: SBuf,
    pub extra: *mut c_void,
}

/// Returns nonzero when the caller must `return 0`, zero otherwise.
#[no_mangle]
pub extern "C" fn slirp_socket_can_recv__part2555(so: *mut Socket) -> c_int {
    if so.is_null() {
        return 1;
    }
    let so = unsafe { &*so };
    if (so.so_state & (0x008 | 0x004)) != 0x004 {
        return 1;
    }
    if so.so_snd.sb_cc >= so.so_snd.sb_datalen / 2 {
        return 1;
    }
    0
}
}

// ── `return sopreprbuf(so, iov, ((void *)0) );` · slirp/slirp.c:871-871 · chunk 2556 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return sopreprbuf(so, iov, 
//                                ((void *)0)
//                                    );
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2556 {
use core::ffi::{c_int, c_void};

extern "C" {
    // size_t sopreprbuf(struct socket *so, struct iovec *iov, int *np);
    fn sopreprbuf(so: *mut c_void, iov: *mut c_void, np: *mut c_int) -> usize;
}

#[no_mangle]
pub extern "C" fn slirp_socket_can_recv__part2556(so: *mut c_void, iov: *mut c_void) -> usize {
    unsafe { sopreprbuf(so, iov, core::ptr::null_mut()) }
}
}

// ── `slirp_socket_recv` · slirp/slirp.c:874-887 · chunk 2557 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void slirp_socket_recv(Slirp *slirp, struct in_addr guest_addr, int guest_port,
//                            const uint8_t *buf, int size)
//     {
//         int ret;
//         struct socket *so = slirp_find_ctl_socket(slirp, guest_addr, guest_port);
//     
//         if (!so)
//             return;
//     
//         ret = soreadbuf(so, (const char *)buf, size);
//     
//         if (ret > 0)
//             tcp_output(((so)->so_tcpcb));
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2557 {
// Rust rewrite of slirp_socket_recv (slirp/slirp.c)
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InAddr {
    pub s_addr: u32,
}

// struct tcpcb is only ever used through a pointer here.
#[repr(C)]
pub struct Tcpcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sbuf {
    pub sb_cc: u32,
    pub sb_datalen: u32,
    pub sb_wptr: *mut c_char,
    pub sb_rptr: *mut c_char,
    pub sb_data: *mut c_char,
}

#[repr(C)]
pub struct MHdr {
    pub mh_next: *mut Mbuf,
    pub mh_prev: *mut Mbuf,
    pub mh_nextpkt: *mut Mbuf,
    pub mh_prevpkt: *mut Mbuf,
    pub mh_flags: c_int,
    pub mh_size: c_int,
    pub mh_so: *mut Socket,
    pub mh_data: *mut c_char,
    pub mh_len: c_int,
}

#[repr(C)]
pub union MDat {
    pub m_dat_: [c_char; 1],
    pub m_ext_: *mut c_char,
}

#[repr(C)]
pub struct Mbuf {
    pub m_hdr: MHdr,
    pub slirp: *mut Slirp,
    pub m_dat: MDat,
}

#[repr(C)]
pub struct Qlink {
    pub next: *mut core::ffi::c_void,
    pub prev: *mut core::ffi::c_void,
}

// struct ipq is declared __attribute__((packed))
#[repr(C, packed)]
pub struct IpQ {
    pub frag_link: Qlink,
    pub ip_link: Qlink,
    pub ipq_ttl: u8,
    pub ipq_p: u8,
    pub ipq_id: u16,
    pub ipq_src: InAddr,
    pub ipq_dst: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootpClient {
    pub allocated: u16,
    pub macaddr: [u8; 6],
}

#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut Slirp,
    pub so_m: *mut Mbuf,
    pub so_ti: *mut core::ffi::c_void, // struct tcpiphdr *
    pub so_urgc: c_int,
    pub so_faddr: InAddr,
    pub so_laddr: InAddr,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8, // u_char
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
    pub so_expire: u32, // u_int
    pub so_queued: c_int,
    pub so_nqueued: c_int,
    pub so_rcv: Sbuf,
    pub so_snd: Sbuf,
    pub extra: *mut core::ffi::c_void,
}

// Mirrors struct Slirp up to (and including) the `tcb` field; the fields that
// follow are never touched by this function.
#[repr(C)]
pub struct Slirp {
    pub vnetwork_addr: InAddr,
    pub vnetwork_mask: InAddr,
    pub vhost_addr: InAddr,
    pub vdhcp_startaddr: InAddr,
    pub vnameserver_addr: InAddr,
    pub client_ethaddr: [u8; 6],
    pub client_ipaddr: InAddr,
    pub client_hostname: [c_char; 33],
    pub restricted: c_int,
    pub tt: Timeval,
    pub exec_list: *mut core::ffi::c_void, // struct ex_list *
    pub m_freelist: Mbuf,
    pub m_usedlist: Mbuf,
    pub mbuf_alloced: c_int,
    pub if_queued: c_int,
    pub if_fastq: Mbuf,
    pub if_batchq: Mbuf,
    pub next_m: *mut Mbuf,
    pub ipq: IpQ,
    pub ip_id: u16,
    pub bootp_clients: [BootpClient; 16],
    pub bootp_filename: *mut c_char,
    pub tcb: Socket,
}

extern "C" {
    fn soreadbuf(so: *mut Socket, buf: *const c_char, size: c_int) -> c_int;
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
    fn htons(x: u16) -> u16;
}

// Ported from the (static) C helper slirp_find_ctl_socket().
unsafe fn slirp_find_ctl_socket(
    slirp: *mut Slirp,
    guest_addr: InAddr,
    guest_port: c_int,
) -> *mut Socket {
    let tcb = core::ptr::addr_of_mut!((*slirp).tcb);
    let mut so = (*slirp).tcb.so_next;
    while so != tcb {
        if (*so).so_faddr.s_addr == guest_addr.s_addr
            && (htons((*so).so_fport) as c_int) == guest_port
        {
            return so;
        }
        so = (*so).so_next;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn slirp_socket_recv(
    slirp: *mut Slirp,
    guest_addr: InAddr,
    guest_port: c_int,
    buf: *const u8,
    size: c_int,
) {
    let so = slirp_find_ctl_socket(slirp, guest_addr, guest_port);

    if so.is_null() {
        return;
    }

    let ret = soreadbuf(so, buf as *const c_char, size);

    if ret > 0 {
        tcp_output((*so).so_tcpcb);
    }
}
}

// ── `if (!so)` · slirp/slirp.c:880-881 · chunk 2558 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!so)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2558 {
// Port of `if (!so) return;` from slirp_socket_recv.
// Returns nonzero when the caller should return early (i.e. `so` is NULL).

#[no_mangle]
pub extern "C" fn slirp_socket_recv__part2558(so: *mut core::ffi::c_void) -> core::ffi::c_int {
    if so.is_null() {
        1
    } else {
        0
    }
}
}

// ── `ret = soreadbuf(so, (const char *)buf, size);` · slirp/slirp.c:883-883 · chunk 2559 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ret = soreadbuf(so, (const char *)buf, size);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2559 {
use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn soreadbuf(so: *mut c_void, buf: *const c_char, size: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn slirp_socket_recv__part2559(
    so: *mut c_void,
    buf: *const u8,
    size: c_int,
) -> c_int {
    unsafe { soreadbuf(so, buf as *const c_char, size) }
}
}

// ── `if (ret > 0)` · slirp/slirp.c:885-886 · chunk 2560 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret > 0)
//             tcp_output(((so)->so_tcpcb));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2560 {
// Port of the tail of slirp_socket_recv: flush pending TCP output when
// the read produced data.

use core::ffi::{c_int, c_void};

// Opaque mirror of `struct tcpcb` (only ever used behind a pointer).
#[repr(C)]
pub struct Tcpcb {
    _private: [u8; 0],
}

// Mirror of `struct socket` up to and including `so_tcpcb`;
// the trailing fields (so_expire .. extra) are never touched here.
#[repr(C)]
pub struct Socket {
    pub so_next: *mut Socket,
    pub so_prev: *mut Socket,
    pub s: c_int,
    pub slirp: *mut c_void,
    pub so_m: *mut c_void,
    pub so_ti: *mut c_void,
    pub so_urgc: c_int,
    pub so_faddr: u32,
    pub so_laddr: u32,
    pub so_fport: u16,
    pub so_lport: u16,
    pub so_iptos: u8,
    pub so_emu: u8,
    pub so_type: u8,
    pub _pad0: u8,
    pub so_state: c_int,
    pub so_tcpcb: *mut Tcpcb,
}

extern "C" {
    fn tcp_output(tp: *mut Tcpcb) -> c_int;
}

#[no_mangle]
pub extern "C" fn slirp_socket_recv__part2560(ret: c_int, so: *mut Socket) {
    if ret > 0 {
        unsafe {
            tcp_output((*so).so_tcpcb);
        }
    }
}
}

