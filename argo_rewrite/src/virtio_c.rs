//! From `virtio.c`.

// ── `int i; s->status = 0; s->queue_sel = 0; s->device_features_sel = 0; s->int_status = 0;` · virtio.c:164-169 · chunk 3699 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int i;
//     
//         s->status = 0;
//         s->queue_sel = 0;
//         s->device_features_sel = 0;
//         s->int_status = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3699 {
// Port of the state-clearing prologue of virtio_reset().

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
}

#[no_mangle]
pub unsafe extern "C" fn virtio_reset__part3699(s: *mut VIRTIODevice) {
    let s = &mut *s;
    s.status = 0;
    s.queue_sel = 0;
    s.device_features_sel = 0;
    s.int_status = 0;
}
}

// ── `for (i = 0; i < 8; i++)` · virtio.c:170-178 · chunk 3700 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 8; i++) {
//             QueueState *qs = &s->queue[i];
//             qs->ready = 0;
//             qs->num = 16;
//             qs->desc_addr = 0;
//             qs->avail_addr = 0;
//             qs->used_addr = 0;
//             qs->last_avail_idx = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3700 {
// Reset the 8 virtio queue states of a device.
// Ported piece of virtio_reset().

use core::ffi::c_int;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn virtio_reset__part3700(qs: *mut QueueState) {
    let mut i: c_int = 0;
    while i < 8 {
        let q = qs.add(i as usize);
        (*q).ready = 0;
        (*q).num = 16;
        (*q).desc_addr = 0;
        (*q).avail_addr = 0;
        (*q).used_addr = 0;
        (*q).last_avail_idx = 0;
        i += 1;
    }
}
}

// ── `return pci_device_get_dma_ptr(s->pci_dev, paddr, is_rw);` · virtio.c:183-183 · chunk 3702 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return pci_device_get_dma_ptr(s->pci_dev, paddr, is_rw);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3702 {
use core::ffi::{c_int, c_void};

// Prefix of the C `struct VIRTIODevice`; only `pci_dev` is accessed here.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
}

extern "C" {
    fn pci_device_get_dma_ptr(d: *mut c_void, addr: u64, is_rw: c_int) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn virtio_pci_get_ram_ptr__part3702(
    s: *mut VIRTIODevice,
    paddr: u64,
    is_rw: c_int,
) -> *mut u8 {
    unsafe {
        let pci_dev = (*s).pci_dev;
        pci_device_get_dma_ptr(pci_dev, paddr, is_rw)
    }
}
}

// ── `return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);` · virtio.c:188-188 · chunk 3704 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3704 {
use core::ffi::{c_int, c_void};

// Mirror of the leading portion of C's struct VIRTIODevice that this code
// touches. `mem_map` is the first field, so the offset is identical.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
}

extern "C" {
    fn phys_mem_get_ram_ptr(map: *mut c_void, paddr: u64, is_rw: c_int) -> *mut u8;
}

#[no_mangle]
pub extern "C" fn virtio_mmio_get_ram_ptr__part3704(
    s: *mut VIRTIODevice,
    paddr: u64,
    is_rw: c_int,
) -> *mut u8 {
    unsafe {
        let mem_map = (*s).mem_map;
        phys_mem_get_ram_ptr(mem_map, paddr, is_rw)
    }
}
}

// ── `if (cfg_type == 2) … else` · virtio.c:197-200 · chunk 3706 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cfg_type == 2)
//             cap_len = 20;
//         else
//             cap_len = 16;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3706 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_add_pci_capability__part3706(cfg_type: c_int) -> c_int {
    if cfg_type == 2 {
        20
    } else {
        16
    }
}
}

// ── `memset(cap, 0, cap_len); cap[0] = 0x09; cap[2] = cap_len; cap[3] = cfg_type; cap[4] = bar; put_…` · virtio.c:201-207 · chunk 3707 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(cap, 0, cap_len);
//         cap[0] = 0x09;
//         cap[2] = cap_len;
//         cap[3] = cfg_type;
//         cap[4] = bar;
//         put_le32(cap + 8, offset);
//         put_le32(cap + 12, len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3707 {
use core::ffi::c_int;

unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr.add(0) = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

#[no_mangle]
pub extern "C" fn virtio_add_pci_capability__part3707(
    cap: *mut u8,
    cap_len: c_int,
    cfg_type: c_int,
    bar: c_int,
    offset: u32,
    len: u32,
) {
    unsafe {
        // memset(cap, 0, cap_len)
        let n = cap_len as usize;
        let mut i: usize = 0;
        while i < n {
            *cap.add(i) = 0;
            i += 1;
        }
        *cap.add(0) = 0x09u8;
        *cap.add(2) = cap_len as u8;
        *cap.add(3) = cfg_type as u8;
        *cap.add(4) = bar as u8;
        put_le32(cap.add(8), offset);
        put_le32(cap.add(12), len);
    }
}
}

// ── `if (cfg_type == 2)` · virtio.c:208-209 · chunk 3708 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (cfg_type == 2)
//             put_le32(cap + 16, mult);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3708 {
// Port of the mult-field store from virtio_add_pci_capability.
// put_le32 is a static inline helper, so its logic lives here.
use core::ffi::{c_int};

#[inline]
fn put_le32(ptr: *mut u8, v: u32) {
    unsafe {
        core::ptr::write(ptr.add(0), (v & 0xff) as u8);
        core::ptr::write(ptr.add(1), ((v >> 8) & 0xff) as u8);
        core::ptr::write(ptr.add(2), ((v >> 16) & 0xff) as u8);
        core::ptr::write(ptr.add(3), ((v >> 24) & 0xff) as u8);
    }
}

#[no_mangle]
pub extern "C" fn virtio_add_pci_capability__part3708(
    cfg_type: c_int,
    cap: *mut u8,
    mult: u32,
) {
    if cfg_type == 2 {
        if !cap.is_null() {
            put_le32(unsafe { cap.add(16) }, mult);
        }
    }
}
}

// ── `pci_add_capability(s->pci_dev, cap, cap_len);` · virtio.c:210-210 · chunk 3709 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     pci_add_capability(s->pci_dev, cap, cap_len);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3709 {
// Part of virtio_add_pci_capability: forward the built capability buffer to
// the PCI layer.
use core::ffi::{c_int, c_void};

// Prefix of `struct VIRTIODevice` up to and including the `pci_dev` field.
#[repr(C)]
pub struct VirtioDevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
}

extern "C" {
    fn pci_add_capability(d: *mut c_void, buf: *const u8, size: c_int) -> c_int;
}

#[no_mangle]
pub extern "C" fn virtio_add_pci_capability__part3709(
    s: *mut VirtioDevice,
    cap: *const u8,
    cap_len: c_int,
) {
    unsafe {
        let pci_dev = (*s).pci_dev;
        pci_add_capability(pci_dev, cap, cap_len);
    }
}
}

// ── `VIRTIODevice *s = opaque; phys_mem_set_addr(s->mem_range, addr, enabled);` · virtio.c:216-217 · chunk 3711 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VIRTIODevice *s = opaque;
//         phys_mem_set_addr(s->mem_range, addr, enabled);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3711 {
// Rust rewrite of virtio.c lines 216-217 (inside virtio_pci_bar_set).
use core::ffi::{c_int, c_void};

// Mirror of the C structs touched through the opaque pointer. VIRTIODevice is
// only read at its `mem_range` field, but the full layout is mirrored so that
// the field offsets match the C definition exactly.
#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VirtioDevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

extern "C" {
    // void phys_mem_set_addr(PhysMemoryRange *pr, uint64_t addr, BOOL enabled);
    fn phys_mem_set_addr(pr: *mut c_void, addr: u64, enabled: c_int);
}

#[no_mangle]
pub extern "C" fn virtio_pci_bar_set__part3711(
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: c_int,
) {
    let _ = bar_num; // unused in the original code
    unsafe {
        let s = opaque as *mut VirtioDevice;
        let mem_range = (*s).mem_range;
        phys_mem_set_addr(mem_range, addr as u64, enabled);
    }
}
}

// ── `memset(s, 0, sizeof(*s));` · virtio.c:224-224 · chunk 3713 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     memset(s, 0, sizeof(*s));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3713 {
// Port of the `memset(s, 0, sizeof(*s));` statement from virtio_init().
use core::ffi::{c_int, c_void};

extern "C" {
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn virtio_init__part3713(s: *mut c_void, size: usize) {
    if !s.is_null() {
        memset(s, 0, size);
    }
}
}

// ── `s->device_id = device_id; s->vendor_id = 0xffff; s->config_space_size = config_space_size; s->d…` · virtio.c:291-295 · chunk 3715 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->device_id = device_id;
//         s->vendor_id = 0xffff;
//         s->config_space_size = config_space_size;
//         s->device_recv = device_recv;
//         virtio_reset(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3715 {
// Port of the tail of virtio_init(): device identity fields plus virtio_reset().
use core::ffi::{c_int, c_void};

pub type VirtioPhysAddrT = u64;
pub type Bool = c_int;

pub type VirtioRecvFunc = unsafe extern "C" fn(
    s1: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    read_size: c_int,
    write_size: c_int,
) -> c_int;

pub type VirtioGetRamPtrFunc =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: VirtioPhysAddrT, is_rw: Bool) -> *mut u8;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddrT,
    pub avail_addr: VirtioPhysAddrT,
    pub used_addr: VirtioPhysAddrT,
    pub manual_recv: Bool,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<VirtioGetRamPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: Option<VirtioRecvFunc>,
    pub config_write: Option<unsafe extern "C" fn(s: *mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

// C: static void virtio_reset(VIRTIODevice *s)  (static -> ported inline)
unsafe fn virtio_reset(s: *mut VIRTIODevice) {
    let sp = &mut *s;
    sp.status = 0;
    sp.queue_sel = 0;
    sp.device_features_sel = 0;
    sp.int_status = 0;
    let mut i: c_int = 0;
    while i < 8 {
        let qs = &mut sp.queue[i as usize];
        qs.ready = 0;
        qs.num = 16;
        qs.desc_addr = 0;
        qs.avail_addr = 0;
        qs.used_addr = 0;
        qs.last_avail_idx = 0;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_init__part3715(
    s: *mut VIRTIODevice,
    device_id: u32,
    config_space_size: c_int,
    device_recv: Option<VirtioRecvFunc>,
) {
    let sp = &mut *s;
    sp.device_id = device_id;
    sp.vendor_id = 0xffff;
    sp.config_space_size = config_space_size as u32;
    sp.device_recv = device_recv;
    virtio_reset(s);
}
}

// ── `if (addr & 1)` · virtio.c:301-302 · chunk 3717 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr & 1)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3717 {
// Port of the odd-address guard of virtio_read16:
// returns 1 when the address is odd (caller must return 0), else 0.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_read16__part3717(addr: u64) -> c_int {
    if addr & 1 != 0 { 1 } else { 0 }
}
}

// ── `ptr = s->get_ram_ptr(s, addr, FALSE);` · virtio.c:303-303 · chunk 3718 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ptr = s->get_ram_ptr(s, addr, FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3718 {
// Ported piece of virtio_read16: fetching the RAM pointer for `addr`.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};
use core::ptr::null_mut;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

pub type GetRAMPtrFn =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8;

pub type DeviceRecvFn =
    unsafe extern "C" fn(s1: *mut c_void, queue_idx: c_int, desc_idx: c_int,
                         read_size: c_int, write_size: c_int) -> c_int;

pub type ConfigWriteFn = unsafe extern "C" fn(s: *mut VIRTIODevice);

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<GetRAMPtrFn>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: Option<DeviceRecvFn>,
    pub config_write: Option<ConfigWriteFn>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[no_mangle]
pub unsafe extern "C" fn virtio_read16__part3718(
    s: *mut VIRTIODevice,
    addr: u64,
) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, 0),
        None => null_mut(),
    }
}
}

// ── `if (!ptr)` · virtio.c:304-305 · chunk 3719 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!ptr)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3719 {
// Port of the `if (!ptr) return 0;` guard inside virtio_read16().
// Takes the RAM pointer obtained from s->get_ram_ptr() and reports whether the
// caller must bail out with 0 (i.e. the pointer was NULL).

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_read16__part3719(ptr: *const u8) -> c_int {
    if ptr.is_null() {
        1
    } else {
        0
    }
}
}

// ── `return *(uint16_t *)ptr;` · virtio.c:306-306 · chunk 3720 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return *(uint16_t *)ptr;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3720 {
// Reads a 16-bit little-endian value from guest RAM at ptr (unaligned-safe).
#[no_mangle]
pub extern "C" fn virtio_read16__part3720(ptr: *const u8) -> core::ffi::c_ushort {
    unsafe { core::ptr::read_unaligned(ptr as *const u16) }
}
}

// ── `if (addr & 1)` · virtio.c:313-314 · chunk 3722 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr & 1)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3722 {
// Ported piece of virtio_write16: the unaligned-address early return check.
// Returns nonzero when the target address is odd (i.e. not 16-bit aligned),
// which makes the C caller return early.

#[no_mangle]
pub extern "C" fn virtio_write16__part3722(addr: u64) -> core::ffi::c_int {
    if (addr & 1) != 0 {
        1
    } else {
        0
    }
}
}

// ── `ptr = s->get_ram_ptr(s, addr, TRUE);` · virtio.c:315-315 · chunk 3723 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ptr = s->get_ram_ptr(s, addr, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3723 {
// Port of the `ptr = s->get_ram_ptr(s, addr, TRUE);` statement of virtio_write16.
use core::ffi::{c_int, c_void};

pub type BOOL = c_int;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, BOOL) -> *mut u8>,
}

#[no_mangle]
pub unsafe extern "C" fn virtio_write16__part3723(s: *mut VIRTIODevice, addr: u64) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, 1),
        None => core::ptr::null_mut(),
    }
}
}

// ── `if (!ptr)` · virtio.c:316-317 · chunk 3724 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (!ptr)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3724 {
use core::ffi::c_int;

// Mirrors the C check `if (!ptr) return;` inside virtio_write16.
// Returns 1 when the caller must return early (ptr is NULL), 0 otherwise.
#[no_mangle]
pub extern "C" fn virtio_write16__part3724(ptr: *mut u8) -> c_int {
    if ptr.is_null() {
        1
    } else {
        0
    }
}
}

// ── `*(uint16_t *)ptr = val;` · virtio.c:318-318 · chunk 3725 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *(uint16_t *)ptr = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3725 {
// Store `val` through the byte pointer `ptr`, exactly like the C statement
// `*(uint16_t *)ptr = val;`.
#[no_mangle]
pub extern "C" fn virtio_write16__part3725(ptr: *mut u8, val: u16) {
    unsafe {
        core::ptr::write_unaligned(ptr as *mut u16, val);
    }
}
}

// ── `if (addr & 3)` · virtio.c:325-326 · chunk 3727 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr & 3)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3727 {
// Ported piece of virtio_write32: alignment check for 32-bit accesses.
// Returns nonzero when the address is not 4-byte aligned (C's `if (addr & 3)`).

#[no_mangle]
pub extern "C" fn virtio_write32__part3727(addr: u64) -> core::ffi::c_int {
    // (addr & 3) as a truthiness test, keeping the same unsigned arithmetic.
    if (addr & 3) != 0 {
        1
    } else {
        0
    }
}
}

// ── `ptr = s->get_ram_ptr(s, addr, TRUE);` · virtio.c:327-327 · chunk 3728 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ptr = s->get_ram_ptr(s, addr, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3728 {
// Port of the get_ram_ptr call inside virtio_write32.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;
const TRUE: BOOL = 1;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: BOOL,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

pub type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, BOOL) -> *mut u8>;
pub type VIRTIODeviceRecvFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, c_int, c_int, c_int, c_int) -> c_int>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: VIRTIOGetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: VIRTIODeviceRecvFunc,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[no_mangle]
pub unsafe extern "C" fn virtio_write32__part3728(
    s: *mut VIRTIODevice,
    addr: u64,
) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, TRUE),
        None => core::ptr::null_mut(),
    }
}
}

// ── `if (!ptr)` · virtio.c:328-329 · chunk 3729 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!ptr)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3729 {
// Port of the `if (!ptr) return;` guard inside virtio_write32.
// Returns nonzero when the caller must return early (ptr is NULL).

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_write32__part3729(ptr: *mut u8) -> c_int {
    if ptr.is_null() {
        1
    } else {
        0
    }
}
}

// ── `*(uint32_t *)ptr = val;` · virtio.c:330-330 · chunk 3730 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *(uint32_t *)ptr = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3730 {
// Stores a 32-bit little-host-endian value through a raw RAM pointer.
// Mirrors the C statement `*(uint32_t *)ptr = val;` in virtio_write32.
#[no_mangle]
pub extern "C" fn virtio_write32__part3730(ptr: *mut u8, val: u32) {
    unsafe {
        // ptr is non-NULL and 4-byte aligned as guaranteed by the caller.
        *(ptr as *mut u32) = val;
    }
}
}

// ── `while (count > 0)` · virtio.c:339-348 · chunk 3732 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (count > 0) {
//             l = min_int(count, 4096 - (addr & (4096 - 1)));
//             ptr = s->get_ram_ptr(s, addr, FALSE);
//             if (!ptr)
//                 return -1;
//             memcpy(buf, ptr, l);
//             addr += l;
//             buf += l;
//             count -= l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3732 {
// Port of the copy loop of virtio_memcpy_from_ram (virtio.c lines 339-348).
use core::ffi::{c_int, c_void};

#[repr(C)]
struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: c_int,
}

type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8>;

#[repr(C)]
struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut c_void,
    get_ram_ptr: VIRTIOGetRAMPtrFunc,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn virtio_memcpy_from_ram__part3732(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    addr: u64,
    count: c_int,
) -> c_int {
    let mut addr: u64 = addr;
    let mut buf: *mut u8 = buf;
    let mut count: c_int = count;

    while count > 0 {
        let room: u64 = 4096u64.wrapping_sub(addr & (4096u64 - 1));
        let room_i: c_int = room as c_int;
        // min_int(count, 4096 - (addr & (4096 - 1)))
        let l: c_int = if count < room_i { count } else { room_i };
        let get_ram_ptr = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr: *mut u8 = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        memcpy(buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.wrapping_offset(l as isize);
        count -= l;
    }
    0
}
}

// ── `return 0;` · virtio.c:349-349 · chunk 3733 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3733 {
// Portion of virtio_memcpy_from_ram: the trailing `return 0;`.
#[no_mangle]
pub extern "C" fn virtio_memcpy_from_ram__part3733() -> core::ffi::c_int {
    0
}
}

// ── `while (count > 0)` · virtio.c:358-367 · chunk 3735 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (count > 0) {
//             l = min_int(count, 4096 - (addr & (4096 - 1)));
//             ptr = s->get_ram_ptr(s, addr, TRUE);
//             if (!ptr)
//                 return -1;
//             memcpy(ptr, buf, l);
//             addr += l;
//             buf += l;
//             count -= l;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3735 {
// Port of the page-splitting copy loop of virtio_memcpy_to_ram().
// Returns 0 on success, -1 if a RAM pointer could not be obtained.

use core::ffi::{c_int, c_void};

type Bool = c_int;
type VirtioPhysAddr = u64;

// uint8_t *VIRTIOGetRAMPtrFunc(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw)
type VirtioGetRamPtrFn = extern "C" fn(*mut VirtIODevice, VirtioPhysAddr, Bool) -> *mut u8;

#[repr(C)]
pub struct VirtIODevice {
    pub mem_map: *mut c_void,        // PhysMemoryMap *
    pub mem_range: *mut c_void,      // PhysMemoryRange *
    pub pci_dev: *mut c_void,        // PCIDevice *
    pub irq: *mut c_void,            // IRQSignal *
    pub get_ram_ptr: Option<VirtioGetRamPtrFn>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    // QueueState queue[8]
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,    // VIRTIODeviceRecvFunc *
    pub config_write: *mut c_void,   // void (*)(VIRTIODevice *)
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: Bool,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// static inline int min_int(int a, int b)
#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_memcpy_to_ram__part3735(
    s: *mut VirtIODevice,
    mut addr: VirtioPhysAddr,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let b = (4096u64 - (addr & (4096u64 - 1))) as c_int;
        let l = min_int(count, b);
        let get_ram_ptr = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = get_ram_ptr(s, addr, 1 /* TRUE */);
        if ptr.is_null() {
            return -1;
        }
        memcpy(ptr as *mut c_void, buf as *const c_void, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.wrapping_add(l as usize);
        count -= l;
    }
    0
}
}

// ── `return 0;` · virtio.c:368-368 · chunk 3736 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3736 {
// Ported piece of virtio_memcpy_to_ram: the trailing "return 0;" statement.
#[no_mangle]
pub extern "C" fn virtio_memcpy_to_ram__part3736() -> core::ffi::c_int {
    0
}
}

// ── `QueueState *qs = &s->queue[queue_idx]; return virtio_memcpy_from_ram(s, (void *)desc, qs->desc_…` · virtio.c:374-377 · chunk 3738 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     QueueState *qs = &s->queue[queue_idx];
//         return virtio_memcpy_from_ram(s, (void *)desc, qs->desc_addr +
//                                       desc_idx * sizeof(VIRTIODesc),
//                                       sizeof(VIRTIODesc));
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3738 {
// Rust port of the get_desc tail piece: locate the queue state and copy the
// descriptor out of guest RAM.
use core::ffi::{c_int, c_void};

type VirtioPhysAddr = u64;
type BOOL = c_int;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: BOOL,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

const DESC_SIZE: usize = 16; // sizeof(VIRTIODesc)

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// Port of the static C helper virtio_memcpy_from_ram (no linker symbol).
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: VirtioPhysAddr,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & (4096 - 1))) as c_int);
        let get_ram_ptr: unsafe extern "C" fn(*mut VIRTIODevice, VirtioPhysAddr, BOOL) -> *mut u8 =
            core::mem::transmute((*s).get_ram_ptr);
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        memcpy(buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.wrapping_add(l as usize);
        count -= l;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_desc__part3738(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let queue_base = core::ptr::addr_of!((*s).queue) as *const QueueState;
    let qs = queue_base.wrapping_offset(queue_idx as isize);
    let desc_addr = core::ptr::addr_of!((*qs).desc_addr).read();
    let offset = (desc_idx as i64 as u64).wrapping_mul(DESC_SIZE as u64);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        desc_addr.wrapping_add(offset),
        DESC_SIZE as c_int,
    )
}
}

// ── `if (count == 0)` · virtio.c:387-388 · chunk 3740 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (count == 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3740 {
// Port of the early-exit check at the top of memcpy_to_from_queue:
//     if (count == 0)
//         return 0;
// Returns 1 when the caller must return 0 immediately (count == 0),
// 0 when the caller should continue with the transfer.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn memcpy_to_from_queue__part3740(count: c_int) -> c_int {
    if count == 0 {
        1
    } else {
        0
    }
}
}

// ── `get_desc(s, &desc, queue_idx, desc_idx);` · virtio.c:390-390 · chunk 3741 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     get_desc(s, &desc, queue_idx, desc_idx);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3741 {
// Port of the static C helper chain `get_desc` -> `virtio_memcpy_from_ram`
// used by the TARGET statement inside memcpy_to_from_queue.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

pub type BOOL = c_int;
pub type virtio_phys_addr_t = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: virtio_phys_addr_t,
    pub avail_addr: virtio_phys_addr_t,
    pub used_addr: virtio_phys_addr_t,
    pub manual_recv: BOOL,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

pub type VIRTIOGetRAMPtrFunc =
    extern "C" fn(s: *mut VIRTIODevice, paddr: virtio_phys_addr_t, is_rw: BOOL) -> *mut u8;
pub type VIRTIODeviceRecvFunc =
    extern "C" fn(s: *mut VIRTIODevice, queue_idx: c_int, desc_idx: c_int, read_size: c_int, write_size: c_int) -> c_int;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: Option<VIRTIODeviceRecvFunc>,
    pub config_write: Option<extern "C" fn(s: *mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
//                                   virtio_phys_addr_t addr, int count)
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    mut addr: virtio_phys_addr_t,
    mut count: c_int,
) -> c_int {
    let mut cur_buf = buf;
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & (4096u64 - 1))) as c_int);
        let get_ram_ptr = (*s).get_ram_ptr;
        let ptr = match get_ram_ptr {
            Some(f) => f(s, addr, 0),
            None => core::ptr::null_mut::<u8>(),
        };
        if ptr.is_null() {
            return -1;
        }
        memcpy(cur_buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as u64);
        cur_buf = cur_buf.wrapping_offset(l as isize);
        count -= l;
    }
    0
}

// static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
//                     int queue_idx, int desc_idx)
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let queue_base = core::ptr::addr_of_mut!((*s).queue) as *mut QueueState;
    let qs = queue_base.offset(queue_idx as isize);
    let off = (desc_idx as usize as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    let addr = (*qs).desc_addr.wrapping_add(off);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        addr,
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

#[no_mangle]
pub extern "C" fn memcpy_to_from_queue__part3741(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    unsafe { get_desc(s, desc, queue_idx, desc_idx) }
}
}

// ── `if (to_queue) … else` · virtio.c:392-405 · chunk 3742 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (to_queue) {
//             f_write_flag = 2;
//     
//             for(;;) {
//                 if ((desc.flags & 2) == f_write_flag)
//                     break;
//                 if (!(desc.flags & 1))
//                     return -1;
//                 desc_idx = desc.next;
//                 get_desc(s, &desc, queue_idx, desc_idx);
//             }
//         } else {
//             f_write_flag = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3742 {
// Rust rewrite of the target piece of memcpy_to_from_queue (virtio.c).
// Static helpers get_desc / virtio_memcpy_from_ram have no linker symbol,
// so their logic is mirrored here as private Rust functions.

use core::ffi::c_int;

type VirtioPhysAddr = u64;

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: c_int,
}

pub type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, VirtioPhysAddr, c_int) -> *mut u8>;

pub type VIRTIODeviceRecvFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, c_int, c_int, c_int, c_int) -> c_int>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut u8,
    pub mem_range: *mut u8,
    pub pci_dev: *mut u8,
    pub irq: *mut u8,
    pub get_ram_ptr: VIRTIOGetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: VIRTIODeviceRecvFunc,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// Mirror of the static C function virtio_memcpy_from_ram().
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: VirtioPhysAddr,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & (4096 - 1))) as c_int);
        let ptr = match (*s).get_ram_ptr {
            Some(f) => f(s, addr, 0),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

// Mirror of the static C function get_desc().
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_ptr().offset(queue_idx as isize);
    let desc_addr = (*qs).desc_addr;
    let off = (desc_idx as u64).wrapping_mul(16); // sizeof(VIRTIODesc)
    virtio_memcpy_from_ram(s, desc as *mut u8, desc_addr.wrapping_add(off), 16)
}

#[no_mangle]
pub extern "C" fn memcpy_to_from_queue__part3742(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: *mut c_int,
    desc: *mut VIRTIODesc,
    to_queue: c_int,
    f_write_flag: *mut c_int,
) -> c_int {
    unsafe {
        if to_queue != 0 {
            *f_write_flag = 2;

            loop {
                if ((*desc).flags & 2) as c_int == *f_write_flag {
                    break;
                }
                if (*desc).flags & 1 == 0 {
                    return -1;
                }
                *desc_idx = (*desc).next as c_int;
                get_desc(s, desc, queue_idx, *desc_idx);
            }
        } else {
            *f_write_flag = 0;
        }
    }
    0
}
}

// ── `for (; ; )` · virtio.c:408-418 · chunk 3743 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             if ((desc.flags & 2) != f_write_flag)
//                 return -1;
//             if (offset < desc.len)
//                 break;
//             if (!(desc.flags & 1))
//                 return -1;
//             desc_idx = desc.next;
//             offset -= desc.len;
//             get_desc(s, &desc, queue_idx, desc_idx);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3743 {
// Port of the descriptor-advance loop of memcpy_to_from_queue() (virtio.c 408-418).
#![allow(non_camel_case_types)]

use core::ffi::c_int;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut core::ffi::c_void,
    pub mem_range: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub irq: *mut core::ffi::c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

// Port of the static C helper virtio_memcpy_from_ram().
unsafe fn virtio_memcpy_from_ram_impl(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let span = 4096u64.wrapping_sub(addr & 4095) as c_int;
        let l = if count < span { count } else { span };
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        let mut i: c_int = 0;
        while i < l {
            *buf.offset(i as isize) = *ptr.offset(i as isize);
            i += 1;
        }
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

// Port of the static C helper get_desc().
unsafe fn get_desc_impl(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let addr = (*qs)
        .desc_addr
        .wrapping_add((desc_idx as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64));
    virtio_memcpy_from_ram_impl(
        s,
        desc as *mut u8,
        addr,
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

// Returns 0 to continue, -1 when the C code must `return -1`.
#[no_mangle]
pub unsafe extern "C" fn memcpy_to_from_queue__part3743(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    offset: *mut c_int,
    desc_idx: *mut c_int,
    queue_idx: c_int,
    f_write_flag: c_int,
) -> c_int {
    loop {
        if (((*desc).flags as c_int) & 2) != f_write_flag {
            return -1;
        }
        if (*offset as u32) < (*desc).len {
            break;
        }
        if ((*desc).flags & 1) == 0 {
            return -1;
        }
        *desc_idx = (*desc).next as c_int;
        *offset = (*offset as u32).wrapping_sub((*desc).len) as c_int;
        get_desc_impl(s, desc, queue_idx, *desc_idx);
    }
    0
}
}

// ── `return 0;` · virtio.c:441-441 · chunk 3745 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3745 {
// Ported piece of memcpy_to_from_queue: the final "return 0;" success path.
// It reads no caller state, so it takes no parameters and just yields 0.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn memcpy_to_from_queue__part3745() -> c_int {
    0
}
}

// ── `return memcpy_to_from_queue(s, buf, queue_idx, desc_idx, offset, count, FALSE);` · virtio.c:448-449 · chunk 3747 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return memcpy_to_from_queue(s, buf, queue_idx, desc_idx, offset, count,
//                                     FALSE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3747 {
// Port of the TARGET statement in memcpy_from_queue: the call to the static
// helper memcpy_to_from_queue(s, buf, queue_idx, desc_idx, offset, count, FALSE).
// Because memcpy_to_from_queue (and its static helpers) have no linker symbol,
// their logic is ported here as private Rust functions.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

type GetRAMPtrFunc = unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<GetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn call_get_ram_ptr(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, paddr, is_rw),
        None => core::ptr::null_mut(),
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let ptr = call_get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: u64,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let ptr = call_get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let off = (desc_idx as i64 as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    virtio_memcpy_from_ram(s, desc as *mut u8, (*qs).desc_addr.wrapping_add(off),
                           core::mem::size_of::<VIRTIODesc>() as c_int)
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = VIRTIODesc {
        addr: 0,
        len: 0,
        flags: 0,
        next: 0,
    };
    let f_write_flag: c_int;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if ((desc.flags as c_int) & 2) == f_write_flag {
                break;
            }
            if ((desc.flags as c_int) & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if ((desc.flags as c_int) & 2) != f_write_flag {
            return -1;
        }
        if offset < desc.len as c_int {
            break;
        }
        if ((desc.flags as c_int) & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset -= desc.len as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(count, desc.len.wrapping_sub(offset as u32) as c_int);
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, desc.addr.wrapping_add(offset as i64 as u64), buf, l);
        } else {
            virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as i64 as u64), l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset += l;
        buf = buf.add(l as usize);
        if offset == desc.len as c_int {
            if ((desc.flags as c_int) & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
            if ((desc.flags as c_int) & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_from_queue__part3747(
    s: *mut VIRTIODevice,
    buf: *mut c_void,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 0)
}
}

// ── `return memcpy_to_from_queue(s, (void *)buf, queue_idx, desc_idx, offset, count, TRUE);` · virtio.c:456-457 · chunk 3749 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return memcpy_to_from_queue(s, (void *)buf, queue_idx, desc_idx, offset,
//                                     count, TRUE);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3749 {
// Port of virtio.c: memcpy_to_queue (TARGET: the call to memcpy_to_from_queue).
// memcpy_to_from_queue, get_desc, virtio_memcpy_to_ram/from_ram and min_int are
// static in the C TU, so their logic is reproduced here as private Rust fns.

use core::ffi::{c_int, c_void};
use core::ptr;

type VirtioPhysAddrT = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddrT,
    pub avail_addr: VirtioPhysAddrT,
    pub used_addr: VirtioPhysAddrT,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VirtioDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

type GetRamPtrFunc =
    unsafe extern "C" fn(s: *mut VirtioDevice, paddr: VirtioPhysAddrT, is_rw: c_int) -> *mut u8;

#[repr(C)]
pub struct VirtioDevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<GetRamPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
unsafe fn call_get_ram_ptr(
    s: *mut VirtioDevice,
    addr: VirtioPhysAddrT,
    is_rw: c_int,
) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, is_rw),
        None => ptr::null_mut(),
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VirtioDevice,
    buf: *mut u8,
    addr_in: VirtioPhysAddrT,
    count_in: c_int,
) -> c_int {
    let mut addr = addr_in;
    let mut count = count_in;
    let mut buf = buf;

    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & (4096u64 - 1))) as c_int);
        let p = call_get_ram_ptr(s, addr, 0);
        if p.is_null() {
            return -1;
        }
        ptr::copy_nonoverlapping(p, buf, l as usize);
        addr = addr.wrapping_add((l as i64) as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VirtioDevice,
    addr_in: VirtioPhysAddrT,
    buf_in: *const u8,
    count_in: c_int,
) -> c_int {
    let mut addr = addr_in;
    let mut count = count_in;
    let mut buf = buf_in;

    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & (4096u64 - 1))) as c_int);
        let p = call_get_ram_ptr(s, addr, 1);
        if p.is_null() {
            return -1;
        }
        ptr::copy_nonoverlapping(buf, p, l as usize);
        addr = addr.wrapping_add((l as i64) as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VirtioDevice,
    desc: *mut VirtioDesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let base = ptr::addr_of_mut!((*s).queue) as *mut QueueState;
    let qs = base.offset(queue_idx as isize);
    let addr = (*qs)
        .desc_addr
        .wrapping_add(((desc_idx as i64) as u64).wrapping_mul(16u64));
    virtio_memcpy_from_ram(s, desc as *mut u8, addr, 16)
}

#[no_mangle]
pub unsafe extern "C" fn memcpy_to_queue__part3749(
    s: *mut VirtioDevice,
    queue_idx: c_int,
    desc_idx_in: c_int,
    offset_in: c_int,
    buf_in: *const c_void,
    count_in: c_int,
) -> c_int {
    let to_queue: c_int = 1; // TRUE

    let mut desc: VirtioDesc = core::mem::zeroed();
    let mut desc_idx = desc_idx_in;
    let mut offset = offset_in;
    let mut count = count_in;
    let mut buf = buf_in as *mut u8;
    let f_write_flag: c_int;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc as *mut VirtioDesc, queue_idx, desc_idx);

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if ((desc.flags as c_int) & 2) == f_write_flag {
                break;
            }
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc as *mut VirtioDesc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if ((desc.flags as c_int) & 2) != f_write_flag {
            return -1;
        }
        if (offset as u32) < desc.len {
            break;
        }
        if (desc.flags & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = (offset as u32).wrapping_sub(desc.len) as c_int;
        get_desc(s, &mut desc as *mut VirtioDesc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(count, desc.len.wrapping_sub(offset as u32) as c_int);
        if to_queue != 0 {
            virtio_memcpy_to_ram(
                s,
                desc.addr.wrapping_add((offset as i64) as u64),
                buf,
                l,
            );
        } else {
            virtio_memcpy_from_ram(
                s,
                buf,
                desc.addr.wrapping_add((offset as i64) as u64),
                l,
            );
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset += l;
        buf = buf.offset(l as isize);
        if (offset as u32) == desc.len {
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc as *mut VirtioDesc, queue_idx, desc_idx);
            if ((desc.flags as c_int) & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}
}

// ── `QueueState *qs = &s->queue[queue_idx]; virtio_phys_addr_t addr; uint32_t index; addr = qs->used…` · virtio.c:464-477 · chunk 3751 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     QueueState *qs = &s->queue[queue_idx];
//         virtio_phys_addr_t addr;
//         uint32_t index;
//     
//         addr = qs->used_addr + 2;
//         index = virtio_read16(s, addr);
//         virtio_write16(s, addr, index + 1);
//     
//         addr = qs->used_addr + 4 + (index & (qs->num - 1)) * 8;
//         virtio_write32(s, addr, desc_idx);
//         virtio_write32(s, addr + 4, desc_len);
//     
//         s->int_status |= 1;
//         set_irq(s->irq, 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3751 {
// Port of the tail of virtio_consume_desc: updates the used ring and raises the IRQ.
use core::ffi::{c_int, c_void};

// typedef int BOOL;
const FALSE: c_int = 0;
const TRUE: c_int = 1;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
}

unsafe fn get_ram_ptr(s: *mut VIRTIODevice, addr: u64, is_rw: c_int) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, is_rw),
        None => core::ptr::null_mut(),
    }
}

// static uint16_t virtio_read16(VIRTIODevice *s, virtio_phys_addr_t addr)
unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if (addr & 1) != 0 {
        return 0; // unaligned accesses are not supported
    }
    let ptr = get_ram_ptr(s, addr, FALSE);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

// static void virtio_write16(VIRTIODevice *s, virtio_phys_addr_t addr, uint16_t val)
unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: u64, val: u16) {
    if (addr & 1) != 0 {
        return; // unaligned accesses are not supported
    }
    let ptr = get_ram_ptr(s, addr, TRUE);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

// static void virtio_write32(VIRTIODevice *s, virtio_phys_addr_t addr, uint32_t val)
unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: u64, val: u32) {
    if (addr & 3) != 0 {
        return; // unaligned accesses are not supported
    }
    let ptr = get_ram_ptr(s, addr, TRUE);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

// static inline void set_irq(IRQSignal *irq, int level)
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_consume_desc__part3751(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().wrapping_offset(queue_idx as isize);

    let mut addr: u64 = (*qs).used_addr.wrapping_add(2);
    let index: u32 = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, (index.wrapping_add(1)) as u16);

    addr = (*qs).used_addr
        .wrapping_add(4)
        .wrapping_add((index & ((*qs).num).wrapping_sub(1)).wrapping_mul(8) as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    set_irq((*s).irq, 1);
}
}

// ── `VIRTIODesc desc; int read_size, write_size; read_size = 0; write_size = 0; get_desc(s, &desc, q…` · virtio.c:484-489 · chunk 3753 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     VIRTIODesc desc;
//         int read_size, write_size;
//     
//         read_size = 0;
//         write_size = 0;
//         get_desc(s, &desc, queue_idx, desc_idx);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3753 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

pub type GetRAMPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: GetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// Port of the static C function virtio_memcpy_from_ram (no linker symbol).
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    let mut buf = buf;
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & 4095)) as c_int);
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

// Port of the static C function get_desc (no linker symbol of its own).
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = (core::ptr::addr_of_mut!((*s).queue) as *mut QueueState)
        .offset(queue_idx as isize);
    let off = (desc_idx as i64 as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn get_desc_rw_size__part3753(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    pdesc: *mut VIRTIODesc,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
) {
    let desc: *mut VIRTIODesc = pdesc;
    core::ptr::write_bytes(desc as *mut u8, 0, core::mem::size_of::<VIRTIODesc>());
    let read_size: c_int = 0;
    let write_size: c_int = 0;
    get_desc(s, desc, queue_idx, desc_idx);

    *pread_size = read_size;
    *pwrite_size = write_size;
}
}

// ── `for (; ; )` · virtio.c:491-499 · chunk 3754 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             if (desc.flags & 2)
//                 break;
//             read_size += desc.len;
//             if (!(desc.flags & 1))
//                 goto done;
//             desc_idx = desc.next;
//             get_desc(s, &desc, queue_idx, desc_idx);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3754 {
// Port of the read-descriptor chain loop of get_desc_rw_size (virtio.c).
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
//                                   virtio_phys_addr_t addr, int count)
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & 4095u64)) as c_int);
        let get_ram_ptr = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        // FALSE == 0
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        memcpy(buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

// static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
//                     int queue_idx, int desc_idx)
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let off = (desc_idx as i64 as u64).wrapping_mul(16);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        16,
    )
}

// Returns 1 when the C code must `goto done`, 0 when it must fall through
// past the read loop (the `break`).
#[no_mangle]
pub unsafe extern "C" fn get_desc_rw_size__part3754(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    pread_size: *mut c_int,
    queue_idx: c_int,
    pdesc_idx: *mut c_int,
) -> c_int {
    let mut read_size: c_int = *pread_size;
    let mut desc_idx: c_int = *pdesc_idx;

    loop {
        if (*desc).flags & 2 != 0 {
            break;
        }
        read_size = read_size.wrapping_add((*desc).len as c_int);
        if (*desc).flags & 1 == 0 {
            *pread_size = read_size;
            *pdesc_idx = desc_idx;
            return 1;
        }
        desc_idx = (*desc).next as c_int;
        get_desc(s, desc, queue_idx, desc_idx);
    }

    *pread_size = read_size;
    *pdesc_idx = desc_idx;
    0
}
}

// ── `for (; ; )` · virtio.c:501-509 · chunk 3755 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(;;) {
//             if (!(desc.flags & 2))
//                 return -1;
//             write_size += desc.len;
//             if (!(desc.flags & 1))
//                 break;
//             desc_idx = desc.next;
//             get_desc(s, &desc, queue_idx, desc_idx);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3755 {
// Port of the second descriptor-walk loop of get_desc_rw_size() from virtio.c.
// The loop may abort the whole function with `return -1`, so the Rust function
// returns -1 in that case (the C caller returns it) and 0 when the loop breaks
// normally (the C caller falls through to the `done:` label).

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

type virtio_phys_addr_t = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: virtio_phys_addr_t,
    pub avail_addr: virtio_phys_addr_t,
    pub used_addr: virtio_phys_addr_t,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

type VIRTIOGetRAMPtrFunc =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: virtio_phys_addr_t, is_rw: c_int) -> *mut u8;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

// static inline int min_int(int a, int b)
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
//                                   virtio_phys_addr_t addr, int count)
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: virtio_phys_addr_t,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & 4095u64)) as c_int);
        let f = (*s).get_ram_ptr;
        let ptr = match f {
            Some(func) => func(s, addr, 0),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        memcpy(buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

// static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
//                     int queue_idx, int desc_idx)
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let addr = (*qs).desc_addr.wrapping_add(
        (desc_idx as i64 as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64),
    );
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        addr,
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

#[no_mangle]
pub unsafe extern "C" fn get_desc_rw_size__part3755(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    write_size: *mut c_int,
    desc_idx: *mut c_int,
    queue_idx: c_int,
) -> c_int {
    loop {
        if ((*desc).flags & 2) == 0 {
            return -1;
        }
        *write_size = ((*write_size) as u32).wrapping_add((*desc).len) as c_int;
        if ((*desc).flags & 1) == 0 {
            break;
        }
        *desc_idx = (*desc).next as c_int;
        get_desc(s, desc, queue_idx, *desc_idx);
    }
    0
}
}

// ── `done: *pread_size = read_size;` · virtio.c:511-512 · chunk 3756 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     done:
//         *pread_size = read_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3756 {
// Port of the `done:` block statement of get_desc_rw_size:
//   *pread_size = read_size;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn get_desc_rw_size__part3756(pread_size: *mut c_int, read_size: c_int) {
    unsafe {
        *pread_size = read_size;
    }
}
}

// ── `*pwrite_size = write_size; return 0;` · virtio.c:513-514 · chunk 3757 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     *pwrite_size = write_size;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3757 {
// Writes the computed write size through the pointer and reports success (0).

#[no_mangle]
pub extern "C" fn get_desc_rw_size__part3757(
    pwrite_size: *mut core::ffi::c_int,
    write_size: core::ffi::c_int,
) -> core::ffi::c_int {
    unsafe {
        *pwrite_size = write_size;
    }
    0
}
}

// ── `if (qs->manual_recv)` · virtio.c:524-525 · chunk 3759 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (qs->manual_recv)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3759 {
// Port of the `if (qs->manual_recv) return;` early-exit from queue_notify().
// Returns 1 when the caller must return immediately, 0 otherwise.

use core::ffi::c_int;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn queue_notify__part3759(qs: *mut QueueState) -> c_int {
    if qs.is_null() {
        return 0;
    }
    if (*qs).manual_recv != 0 {
        1
    } else {
        0
    }
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2);` · virtio.c:527-527 · chunk 3760 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3760 {
// Port of the statement:
//   avail_idx = virtio_read16(s, qs->avail_addr + 2);
// from queue_notify(). virtio_read16 is a `static` C function (no linker
// symbol), so its logic is reproduced here as a private Rust function.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

type GetRAMPtrFunc = Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: GetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, 0 /* FALSE */);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub extern "C" fn queue_notify__part3760(s: *mut VIRTIODevice, addr: u64) -> u16 {
    unsafe { virtio_read16(s, addr) }
}
}

// ── `while (qs->last_avail_idx != avail_idx)` · virtio.c:528-543 · chunk 3761 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (qs->last_avail_idx != avail_idx) {
//             desc_idx = virtio_read16(s, qs->avail_addr + 4 +
//                                      (qs->last_avail_idx & (qs->num - 1)) * 2);
//             if (!get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx)) {
//     
//     
//     
//     
//     
//     
//                 if (s->device_recv(s, queue_idx, desc_idx,
//                                    read_size, write_size) < 0)
//                     break;
//             }
//             qs->last_avail_idx++;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3761 {
// Rust port of the avail-ring processing loop of virtio's queue_notify().
// Static C helpers used by the loop (virtio_read16, get_desc_rw_size,
// get_desc, virtio_memcpy_from_ram, min_int) have no linker symbols, so
// their logic is reproduced here as private Rust functions.

use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
pub struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: c_int,
}

type GetRAMPtrFunc = unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8;
type DeviceRecvFunc = unsafe extern "C" fn(*mut VIRTIODevice, c_int, c_int, c_int, c_int) -> c_int;
type ConfigWriteFunc = unsafe extern "C" fn(*mut VIRTIODevice);

#[repr(C)]
pub struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut c_void,
    get_ram_ptr: Option<GetRAMPtrFunc>,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: Option<DeviceRecvFunc>,
    config_write: Option<ConfigWriteFunc>,
    config_space_size: u32,
    config_space: [u8; 256],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VIRTIODesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

const VIRTIO_DESC_SIZE: u64 = 16; // sizeof(VIRTIODesc)

#[inline]
unsafe fn queue_ptr(s: *mut VIRTIODevice, queue_idx: c_int) -> *mut QueueState {
    ((*s).queue.as_mut_ptr()).add(queue_idx as usize)
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    (ptr as *const u16).read_unaligned()
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    addr: u64,
    count: c_int,
) -> c_int {
    let mut addr = addr;
    let mut buf = buf;
    let mut count = count;

    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(s: *mut VIRTIODevice, queue_idx: c_int, desc_idx: c_int) -> VIRTIODesc {
    let qs = queue_ptr(s, queue_idx);
    let mut desc = VIRTIODesc {
        addr: 0,
        len: 0,
        flags: 0,
        next: 0,
    };
    let addr = (*qs)
        .desc_addr
        .wrapping_add((desc_idx as u64).wrapping_mul(VIRTIO_DESC_SIZE));
    virtio_memcpy_from_ram(
        s,
        &mut desc as *mut VIRTIODesc as *mut u8,
        addr,
        VIRTIO_DESC_SIZE as c_int,
    );
    desc
}

unsafe fn get_desc_rw_size(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;
    let mut desc_idx = desc_idx;

    let mut desc = get_desc(s, queue_idx, desc_idx);

    loop {
        if desc.flags & 2 != 0 {
            break;
        }
        read_size = (read_size as u32).wrapping_add(desc.len) as c_int;
        if desc.flags & 1 == 0 {
            // goto done
            *pread_size = read_size;
            *pwrite_size = write_size;
            return 0;
        }
        desc_idx = desc.next as c_int;
        desc = get_desc(s, queue_idx, desc_idx);
    }

    loop {
        if desc.flags & 2 == 0 {
            return -1;
        }
        write_size = (write_size as u32).wrapping_add(desc.len) as c_int;
        if desc.flags & 1 == 0 {
            break;
        }
        desc_idx = desc.next as c_int;
        desc = get_desc(s, queue_idx, desc_idx);
    }

    *pread_size = read_size;
    *pwrite_size = write_size;
    0
}

#[no_mangle]
pub unsafe extern "C" fn queue_notify__part3761(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    avail_idx: u16,
) {
    let qs = queue_ptr(s, queue_idx);

    while (*qs).last_avail_idx != avail_idx {
        let ring = (*qs).last_avail_idx as u32 & (*qs).num.wrapping_sub(1);
        let addr = (*qs)
            .avail_addr
            .wrapping_add(4)
            .wrapping_add(ring.wrapping_mul(2) as u64);
        let desc_idx: c_int = virtio_read16(s, addr) as c_int;

        let mut read_size: c_int = 0;
        let mut write_size: c_int = 0;
        if get_desc_rw_size(s, &mut read_size, &mut write_size, queue_idx, desc_idx) == 0 {
            let f = match (*s).device_recv {
                Some(f) => f,
                None => break,
            };
            if f(s, queue_idx, desc_idx, read_size, write_size) < 0 {
                break;
            }
        }
        (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    }
}
}

// ── `switch (size_log2)` · virtio.c:550-574 · chunk 3763 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(size_log2) {
//         case 0:
//             if (offset < s->config_space_size) {
//                 val = s->config_space[offset];
//             } else {
//                 val = 0;
//             }
//             break;
//         case 1:
//             if (offset < (s->config_space_size - 1)) {
//                 val = get_le16(&s->config_space[offset]);
//             } else {
//                 val = 0;
//             }
//             break;
//         case 2:
//             if (offset < (s->config_space_size - 3)) {
//                 val = get_le32(s->config_space + offset);
//             } else {
//                 val = 0;
//             }
//             break;
//         default:
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3763 {
// Port of the config-space read switch from virtio_config_read().
use core::ffi::{c_int, c_uint};

extern "C" {
    // C's abort() is noreturn.
    fn abort() -> !;
}

// Equivalent of the static inline get_le16() in the C translation unit
// (no linker symbol, so the logic is mirrored here).
#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    let b0 = *ptr as u16;
    let b1 = *ptr.wrapping_add(1) as u16;
    b0 | (b1 << 8)
}

// Equivalent of the static inline get_le32().
#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    let b0 = *ptr as u32;
    let b1 = *ptr.wrapping_add(1) as u32;
    let b2 = *ptr.wrapping_add(2) as u32;
    let b3 = *ptr.wrapping_add(3) as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

#[no_mangle]
pub extern "C" fn virtio_config_read__part3763(
    config_space: *const u8,
    config_space_size: c_uint,
    offset: c_uint,
    size_log2: c_int,
) -> c_uint {
    match size_log2 {
        0 => {
            if offset < config_space_size {
                unsafe { *config_space.wrapping_add(offset as usize) as c_uint }
            } else {
                0
            }
        }
        1 => {
            if offset < config_space_size.wrapping_sub(1) {
                unsafe { get_le16(config_space.wrapping_add(offset as usize)) as c_uint }
            } else {
                0
            }
        }
        2 => {
            if offset < config_space_size.wrapping_sub(3) {
                unsafe { get_le32(config_space.wrapping_add(offset as usize)) }
            } else {
                0
            }
        }
        _ => unsafe { abort() },
    }
}
}

// ── `return val;` · virtio.c:575-575 · chunk 3764 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3764 {
// Ported piece of virtio_config_read: return the computed value.
use core::ffi::c_uint;

#[no_mangle]
pub extern "C" fn virtio_config_read__part3764(val: c_uint) -> c_uint {
    val
}
}

// ── `switch (size_log2)` · virtio.c:581-603 · chunk 3766 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     switch(size_log2) {
//         case 0:
//             if (offset < s->config_space_size) {
//                 s->config_space[offset] = val;
//                 if (s->config_write)
//                     s->config_write(s);
//             }
//             break;
//         case 1:
//             if (offset < s->config_space_size - 1) {
//                 put_le16(s->config_space + offset, val);
//                 if (s->config_write)
//                     s->config_write(s);
//             }
//             break;
//         case 2:
//             if (offset < s->config_space_size - 3) {
//                 put_le32(s->config_space + offset, val);
//                 if (s->config_write)
//                     s->config_write(s);
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3766 {
// Port of virtio_config_write's size_log2 switch (virtio.c lines 581-603).

use core::ffi::c_int;

/// Opaque stand-in for the C `VIRTIODevice` struct: only ever handled by pointer.
#[repr(C)]
pub struct VIRTIODevice {
    _private: [u8; 0],
}

type ConfigWriteFn = unsafe extern "C" fn(*mut VIRTIODevice);

#[no_mangle]
pub unsafe extern "C" fn virtio_config_write__part3766(
    config_space: *mut u8,
    config_space_size: u32,
    config_write: Option<ConfigWriteFn>,
    s: *mut VIRTIODevice,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    match size_log2 {
        0 => {
            if offset < config_space_size {
                *config_space.add(offset as usize) = val as u8;
                if let Some(f) = config_write {
                    f(s);
                }
            }
        }
        1 => {
            // C: offset < s->config_space_size - 1  (uint32 arithmetic, wraps)
            if offset < config_space_size.wrapping_sub(1) {
                let p = config_space.add(offset as usize);
                // put_le16(p, (uint16_t)val)
                *p = val as u8;
                *p.add(1) = (val >> 8) as u8;
                if let Some(f) = config_write {
                    f(s);
                }
            }
        }
        2 => {
            // C: offset < s->config_space_size - 3  (uint32 arithmetic, wraps)
            if offset < config_space_size.wrapping_sub(3) {
                let p = config_space.add(offset as usize);
                // put_le32(p, val)
                *p = val as u8;
                *p.add(1) = (val >> 8) as u8;
                *p.add(2) = (val >> 16) as u8;
                *p.add(3) = (val >> 24) as u8;
                if let Some(f) = config_write {
                    f(s);
                }
            }
        }
        _ => {}
    }
}
}

// ── `if (offset >= 0x100)` · virtio.c:611-613 · chunk 3768 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset >= 0x100) {
//             return virtio_config_read(s, offset - 0x100, size_log2);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3768 {
use core::ffi::{c_int, c_void};

// Mirror of the C QueueState (not directly used, but part of VIRTIODevice layout).
#[repr(C)]
struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: c_int,
}

// Mirror of the C struct VIRTIODevice; opaque pointer fields are just pointers.
#[repr(C)]
struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut c_void,
    get_ram_ptr: *mut c_void,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: *mut c_void,
    config_write: *mut c_void,
    config_space_size: u32,
    config_space: [u8; 256],
}

extern "C" {
    fn abort() -> !;
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (core::ptr::read(ptr) as u16) | ((core::ptr::read(ptr.add(1)) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (core::ptr::read(ptr) as u32)
        | ((core::ptr::read(ptr.add(1)) as u32) << 8)
        | ((core::ptr::read(ptr.add(2)) as u32) << 16)
        | ((core::ptr::read(ptr.add(3)) as u32) << 24)
}

// Port of the C `static uint32_t virtio_config_read(VIRTIODevice *s, uint32_t offset, int size_log2)`.
unsafe fn virtio_config_read(s: *mut VIRTIODevice, offset: u32, size_log2: c_int) -> u32 {
    let val: u32;
    match size_log2 {
        0 => {
            if offset < (*s).config_space_size {
                val = (*s).config_space[offset as usize] as u32;
            } else {
                val = 0;
            }
        }
        1 => {
            if offset < (*s).config_space_size.wrapping_sub(1) {
                let base = core::ptr::addr_of!((*s).config_space) as *const u8;
                val = get_le16(base.add(offset as usize)) as u32;
            } else {
                val = 0;
            }
        }
        2 => {
            if offset < (*s).config_space_size.wrapping_sub(3) {
                let base = core::ptr::addr_of!((*s).config_space) as *const u8;
                val = get_le32(base.add(offset as usize));
            } else {
                val = 0;
            }
        }
        _ => {
            abort();
        }
    }
    val
}

#[no_mangle]
pub unsafe extern "C" fn virtio_mmio_read__part3768(
    opaque: *mut c_void,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    let s = opaque as *mut VIRTIODevice;
    virtio_config_read(s, offset.wrapping_sub(0x100), size_log2)
}
}

// ── `if (size_log2 == 2) … else` · virtio.c:615-692 · chunk 3769 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (size_log2 == 2) {
//             switch(offset) {
//             case 0x000:
//                 val = 0x74726976;
//                 break;
//             case 0x004:
//                 val = 2;
//                 break;
//             case 0x008:
//                 val = s->device_id;
//                 break;
//             case 0x00c:
//                 val = s->vendor_id;
//                 break;
//             case 0x010:
//                 switch(s->device_features_sel) {
//                 case 0:
//                     val = s->device_features;
//                     break;
//                 case 1:
//                     val = 1;
//                     break;
//                 default:
//                     val = 0;
//                     break;
//                 }
//                 break;
//             case 0x014:
//                 val = s->device_features_sel;
//                 break;
//             case 0x030:
//                 val = s->queue_sel;
//                 break;
//             case 0x034:
//                 val = 16;
//                 break;
//             case 0x038:
//                 val = s->queue[s->queue_sel].num;
//                 break;
//             case 0x080:
//                 val = s->queue[s->queue_sel].desc_addr;
//                 break;
//             case 0x090:
//                 val = s->queue[s->queue_sel].avail_addr;
//                 break;
//             case 0x0a0:
//                 val = s->queue[s->queue_sel].used_addr;
//                 break;
//     
//             case 0x084:
//                 val = s->queue[s->queue_sel].desc_addr >> 32;
//                 break;
//             case 0x094:
//                 val = s->queue[s->queue_sel].avail_addr >> 32;
//                 break;
//             case 0x0a4:
//                 val = s->queue[s->queue_sel].used_addr >> 32;
//                 break;
//     
//             case 0x044:
//                 val = s->queue[s->queue_sel].ready;
//                 break;
//             case 0x060:
//                 val = s->int_status;
//                 break;
//             case 0x070:
//                 val = s->status;
//                 break;
//             case 0x0fc:
//                 val = 0;
//                 break;
//             default:
//                 val = 0;
//                 break;
//             }
//         } else {
//             val = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3769 {
// Rust port of the register-read switch inside virtio_mmio_read.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
fn queue_ptr(dev: &VIRTIODevice) -> *const QueueState {
    (dev.queue.as_ptr() as *const QueueState).wrapping_add(dev.queue_sel as usize)
}

#[no_mangle]
pub extern "C" fn virtio_mmio_read__part3769(
    s: *mut VIRTIODevice,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    let val: u32;
    if size_log2 == 2 {
        unsafe {
            let dev: &VIRTIODevice = &*s;
            val = match offset {
                0x000 => 0x74726976,
                0x004 => 2,
                0x008 => dev.device_id,
                0x00c => dev.vendor_id,
                0x010 => match dev.device_features_sel {
                    0 => dev.device_features,
                    1 => 1,
                    _ => 0,
                },
                0x014 => dev.device_features_sel,
                0x030 => dev.queue_sel,
                0x034 => 16,
                0x038 => (*queue_ptr(dev)).num,
                0x080 => (*queue_ptr(dev)).desc_addr as u32,
                0x090 => (*queue_ptr(dev)).avail_addr as u32,
                0x0a0 => (*queue_ptr(dev)).used_addr as u32,
                0x084 => ((*queue_ptr(dev)).desc_addr >> 32) as u32,
                0x094 => ((*queue_ptr(dev)).avail_addr >> 32) as u32,
                0x0a4 => ((*queue_ptr(dev)).used_addr >> 32) as u32,
                0x044 => (*queue_ptr(dev)).ready,
                0x060 => dev.int_status,
                0x070 => dev.status,
                0x0fc => 0,
                _ => 0,
            };
        }
    } else {
        val = 0;
    }
    val
}
}

// ── `return val;` · virtio.c:699-699 · chunk 3770 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3770 {
// Port of the tail of virtio_mmio_read: return the locally computed value.
#[no_mangle]
pub extern "C" fn virtio_mmio_read__part3770(val: u32) -> u32 {
    val
}
}

// ── `*paddr = (*paddr & ~(virtio_phys_addr_t)0xffffffff) | val;` · virtio.c:705-705 · chunk 3772 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *paddr = (*paddr & ~(virtio_phys_addr_t)0xffffffff) | val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3772 {
// Rewrite of the single assignment inside set_low32().
// virtio_phys_addr_t is uint64_t; val (uint32_t) is widened to 64 bits,
// the low 32 bits of *paddr are cleared and replaced by val.

#[no_mangle]
pub extern "C" fn set_low32__part3772(paddr: *mut u64, val: u32) {
    unsafe {
        let cur = *paddr;
        *paddr = (cur & !(0xffff_ffffu64)) | (val as u64);
    }
}
}

// ── `*paddr = (*paddr & 0xffffffff) | ((virtio_phys_addr_t)val << 32);` · virtio.c:710-710 · chunk 3774 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     *paddr = (*paddr & 0xffffffff) | ((virtio_phys_addr_t)val << 32);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3774 {
// Port of the set_high32 body: replace the high 32 bits of a 64-bit
// physical address with val, preserving the low 32 bits.

#[no_mangle]
pub unsafe extern "C" fn set_high32__part3774(paddr: *mut u64, val: u32) {
    let cur: u64 = unsafe { *paddr };
    let new_val: u64 = (cur & 0xffff_ffffu64) | ((val as u64) << 32);
    unsafe {
        *paddr = new_val;
    }
}
}

// ── `if (offset >= 0x100)` · virtio.c:731-734 · chunk 3776 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (offset >= 0x100) {
//             virtio_config_write(s, offset - 0x100, val, size_log2);
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3776 {
// Port of virtio_mmio_write's tag offset >= 0x100 branch plus the static
// helper virtio_config_write (which has no linker symbol and is inlined here).

#![allow(dead_code)]

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: i32,
}

#[repr(C)]
pub struct VirtioDevice {
    pub mem_map: *mut u8,
    pub mem_range: *mut u8,
    pub pci_dev: *mut u8,
    pub irq: *mut u8,
    pub get_ram_ptr: *mut u8,
    pub debug: i32,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut u8,
    pub config_write: Option<unsafe extern "C" fn(*mut VirtioDevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

unsafe fn virtio_config_write(s: *mut VirtioDevice, offset: u32, val: u32, size_log2: i32) {
    match size_log2 {
        0 => {
            if offset < (*s).config_space_size {
                *(*s).config_space.as_mut_ptr().add(offset as usize) = val as u8;
                if let Some(f) = (*s).config_write {
                    f(s);
                }
            }
        }
        1 => {
            if offset < (*s).config_space_size.wrapping_sub(1) {
                put_le16((*s).config_space.as_mut_ptr().add(offset as usize), val as u16);
                if let Some(f) = (*s).config_write {
                    f(s);
                }
            }
        }
        2 => {
            if offset < (*s).config_space_size.wrapping_sub(3) {
                put_le32((*s).config_space.as_mut_ptr().add(offset as usize), val);
                if let Some(f) = (*s).config_write {
                    f(s);
                }
            }
        }
        _ => {}
    }
}

// Returns nonzero when the C caller must return from virtio_mmio_write.
#[no_mangle]
pub unsafe extern "C" fn virtio_mmio_write__part3776(
    s: *mut VirtioDevice,
    offset: u32,
    val: u32,
    size_log2: i32,
) -> i32 {
    if offset >= 0x100 {
        virtio_config_write(s, offset.wrapping_sub(0x100), val, size_log2);
        return 1;
    }
    0
}
}

// ── `if (size_log2 == 2)` · virtio.c:736-792 · chunk 3777 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (size_log2 == 2) {
//             switch(offset) {
//             case 0x014:
//                 s->device_features_sel = val;
//                 break;
//             case 0x030:
//                 if (val < 8)
//                     s->queue_sel = val;
//                 break;
//             case 0x038:
//                 if ((val & (val - 1)) == 0 && val > 0) {
//                     s->queue[s->queue_sel].num = val;
//                 }
//                 break;
//             case 0x080:
//                 set_low32(&s->queue[s->queue_sel].desc_addr, val);
//                 break;
//             case 0x090:
//                 set_low32(&s->queue[s->queue_sel].avail_addr, val);
//                 break;
//             case 0x0a0:
//                 set_low32(&s->queue[s->queue_sel].used_addr, val);
//                 break;
//     
//             case 0x084:
//                 set_high32(&s->queue[s->queue_sel].desc_addr, val);
//                 break;
//             case 0x094:
//                 set_high32(&s->queue[s->queue_sel].avail_addr, val);
//                 break;
//             case 0x0a4:
//                 set_high32(&s->queue[s->queue_sel].used_addr, val);
//                 break;
//     
//             case 0x070:
//                 s->status = val;
//                 if (val == 0) {
//     
//                     set_irq(s->irq, 0);
//                     virtio_reset(s);
//                 }
//                 break;
//             case 0x044:
//                 s->queue[s->queue_sel].ready = val & 1;
//                 break;
//             case 0x050:
//                 if (val < 8)
//                     queue_notify(s, val);
//                 break;
//             case 0x064:
//                 s->int_status &= ~val;
//                 if (s->int_status == 0) {
//                     set_irq(s->irq, 0);
//                 }
//                 break;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3777 {
// Rust port of the virtio MMIO register-write dispatch (32-bit accesses only).
#![allow(dead_code, non_snake_case)]

use core::ffi::{c_int, c_void};

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type VIRTIODeviceRecvFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, c_int, c_int, c_int, c_int) -> c_int>;
pub type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: VIRTIOGetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: VIRTIODeviceRecvFunc,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

// ---- ported static C helpers -------------------------------------------------

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
unsafe fn queue_ptr(s: *mut VIRTIODevice, idx: u32) -> *mut QueueState {
    (*s).queue.as_mut_ptr().add(idx as usize)
}

#[inline]
unsafe fn set_low32(paddr: *mut u64, val: u32) {
    let cur = core::ptr::read(paddr);
    core::ptr::write(paddr, (cur & 0xffff_ffff_0000_0000u64) | (val as u64));
}

#[inline]
unsafe fn set_high32(paddr: *mut u64, val: u32) {
    let cur = core::ptr::read(paddr);
    core::ptr::write(paddr, (cur & 0xffff_ffffu64) | ((val as u64) << 32));
}

#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn virtio_reset(s: *mut VIRTIODevice) {
    (*s).status = 0;
    (*s).queue_sel = 0;
    (*s).device_features_sel = 0;
    (*s).int_status = 0;
    let mut i: c_int = 0;
    while i < 8 {
        let qs = (*s).queue.as_mut_ptr().add(i as usize);
        (*qs).ready = 0;
        (*qs).num = 16;
        (*qs).desc_addr = 0;
        (*qs).avail_addr = 0;
        (*qs).used_addr = 0;
        (*qs).last_avail_idx = 0;
        i += 1;
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    let mut buf = buf;
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & 4095)) as c_int);
        let ptr = match (*s).get_ram_ptr {
            Some(f) => f(s, addr, 0),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        memcpy(
            buf as *mut c_void,
            ptr as *const c_void,
            l as usize,
        );
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let off = (desc_idx as i64 as u64).wrapping_mul(16);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        16,
    )
}

unsafe fn get_desc_rw_size(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    mut desc_idx: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;

    get_desc(s, &mut desc, queue_idx, desc_idx);

    loop {
        if (desc.flags & 2) != 0 {
            break;
        }
        read_size = read_size.wrapping_add(desc.len as c_int);
        if (desc.flags & 1) == 0 {
            // goto done
            *pread_size = read_size;
            *pwrite_size = write_size;
            return 0;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        if (desc.flags & 2) == 0 {
            return -1;
        }
        write_size = write_size.wrapping_add(desc.len as c_int);
        if (desc.flags & 1) == 0 {
            break;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    *pread_size = read_size;
    *pwrite_size = write_size;
    0
}

unsafe fn queue_notify(s: *mut VIRTIODevice, queue_idx: c_int) {
    let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let mut desc_idx: c_int;
    let mut read_size: c_int;
    let mut write_size: c_int;

    if (*qs).manual_recv != 0 {
        return;
    }

    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    while (*qs).last_avail_idx != avail_idx {
        let slot = ((*qs).last_avail_idx as u32) & (*qs).num.wrapping_sub(1);
        desc_idx = virtio_read16(
            s,
            (*qs)
                .avail_addr
                .wrapping_add(4)
                .wrapping_add((slot.wrapping_mul(2)) as u64),
        ) as c_int;
        read_size = 0;
        write_size = 0;
        if get_desc_rw_size(s, &mut read_size, &mut write_size, queue_idx, desc_idx) == 0 {
            let ret = match (*s).device_recv {
                Some(f) => f(s, queue_idx, desc_idx, read_size, write_size),
                None => 0,
            };
            if ret < 0 {
                break;
            }
        }
        (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    }
}

// ---- exported target ---------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn virtio_mmio_write__part3777(
    opaque: *mut c_void,
    offset: u32,
    val: u32,
    size_log2: c_int,
) {
    let s = opaque as *mut VIRTIODevice;

    if size_log2 != 2 {
        return;
    }

    match offset {
        0x014 => {
            (*s).device_features_sel = val;
        }
        0x030 => {
            if val < 8 {
                (*s).queue_sel = val;
            }
        }
        0x038 => {
            if (val & val.wrapping_sub(1)) == 0 && val > 0 {
                let qs = queue_ptr(s, (*s).queue_sel);
                (*qs).num = val;
            }
        }
        0x080 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_low32(core::ptr::addr_of_mut!((*qs).desc_addr), val);
        }
        0x090 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_low32(core::ptr::addr_of_mut!((*qs).avail_addr), val);
        }
        0x0a0 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_low32(core::ptr::addr_of_mut!((*qs).used_addr), val);
        }
        0x084 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_high32(core::ptr::addr_of_mut!((*qs).desc_addr), val);
        }
        0x094 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_high32(core::ptr::addr_of_mut!((*qs).avail_addr), val);
        }
        0x0a4 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            set_high32(core::ptr::addr_of_mut!((*qs).used_addr), val);
        }
        0x070 => {
            (*s).status = val;
            if val == 0 {
                set_irq((*s).irq, 0);
                virtio_reset(s);
            }
        }
        0x044 => {
            let qs = queue_ptr(s, (*s).queue_sel);
            (*qs).ready = val & 1;
        }
        0x050 => {
            if val < 8 {
                queue_notify(s, val as c_int);
            }
        }
        0x064 => {
            (*s).int_status &= !val;
            if (*s).int_status == 0 {
                set_irq((*s).irq, 0);
            }
        }
        _ => {}
    }
}
}

// ── `VIRTIODevice *s = opaque; uint32_t offset; uint32_t val = 0; offset = offset1 & 0xfff;` · virtio.c:797-801 · chunk 3779 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VIRTIODevice *s = opaque;
//         uint32_t offset;
//         uint32_t val = 0;
//     
//         offset = offset1 & 0xfff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3779 {
// Ported piece of virtio_pci_read: local setup for the register read.
// Reads the opaque device pointer, derives the 12-bit register offset and
// initialises the result value to zero.

use core::ffi::{c_uint, c_void};

#[no_mangle]
pub extern "C" fn virtio_pci_read__part3779(
    opaque: *mut c_void,
    offset1: c_uint,
    val_out: *mut c_uint,
) -> c_uint {
    // s = opaque (the pointer itself is handed back to the C caller, which
    // keeps the local declaration); nothing is dereferenced here.
    let _ = opaque;

    // uint32_t val = 0;
    if !val_out.is_null() {
        unsafe {
            *val_out = 0;
        }
    }

    // offset = offset1 & 0xfff;
    offset1 & 0xfff
}
}

// ── `switch (offset1 >> 12)` · virtio.c:802-879 · chunk 3780 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(offset1 >> 12) {
//         case 0x0000 >> 12:
//             if (size_log2 == 2) {
//                 switch(offset) {
//                 case 0x004:
//                     switch(s->device_features_sel) {
//                     case 0:
//                         val = s->device_features;
//                         break;
//                     case 1:
//                         val = 1;
//                         break;
//                     default:
//                         val = 0;
//                         break;
//                     }
//                     break;
//                 case 0x000:
//                     val = s->device_features_sel;
//                     break;
//                 case 0x020:
//                     val = s->queue[s->queue_sel].desc_addr;
//                     break;
//                 case 0x028:
//                     val = s->queue[s->queue_sel].avail_addr;
//                     break;
//                 case 0x030:
//                     val = s->queue[s->queue_sel].used_addr;
//                     break;
//     
//                 case 0x024:
//                     val = s->queue[s->queue_sel].desc_addr >> 32;
//                     break;
//                 case 0x02c:
//                     val = s->queue[s->queue_sel].avail_addr >> 32;
//                     break;
//                 case 0x034:
//                     val = s->queue[s->queue_sel].used_addr >> 32;
//                     break;
//     
//                 }
//             } else if (size_log2 == 1) {
//                 switch(offset) {
//                 case 0x012:
//                     val = 16;
//                     break;
//                 case 0x016:
//                     val = s->queue_sel;
//                     break;
//                 case 0x018:
//                     val = s->queue[s->queue_sel].num;
//                     break;
//                 case 0x01c:
//                     val = s->queue[s->queue_sel].ready;
//                     break;
//                 case 0x01e:
//                     val = 0;
//                     break;
//                 }
//             } else if (size_log2 == 0) {
//                 switch(offset) {
//                 case 0x014:
//                     val = s->status;
//                     break;
//                 }
//             }
//             break;
//         case 0x1000 >> 12:
//             if (offset == 0 && size_log2 == 0) {
//                 val = s->int_status;
//                 s->int_status = 0;
//                 set_irq(s->irq, 0);
//             }
//             break;
//         case 0x2000 >> 12:
//             val = virtio_config_read(s, offset, size_log2);
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3780 {
// Port of the register-switch inside virtio_pci_read (virtio.c).
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int, // BOOL
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

extern "C" {
    fn abort() -> !;
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u32 {
    (*ptr as u32) | ((*ptr.add(1) as u32) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

// Ported from the static C helper virtio_config_read (no linker symbol).
unsafe fn virtio_config_read(s: *mut VIRTIODevice, offset: u32, size_log2: c_int) -> u32 {
    match size_log2 {
        0 => {
            if offset < (*s).config_space_size {
                *(*s).config_space.as_ptr().add(offset as usize) as u32
            } else {
                0
            }
        }
        1 => {
            if offset < (*s).config_space_size.wrapping_sub(1) {
                get_le16((*s).config_space.as_ptr().add(offset as usize))
            } else {
                0
            }
        }
        2 => {
            if offset < (*s).config_space_size.wrapping_sub(3) {
                get_le32((*s).config_space.as_ptr().add(offset as usize))
            } else {
                0
            }
        }
        _ => abort(),
    }
}

#[inline]
unsafe fn queue_ptr(s: *mut VIRTIODevice) -> *mut QueueState {
    (*s).queue.as_mut_ptr().add((*s).queue_sel as usize)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_pci_read__part3780(
    opaque: *mut c_void,
    offset1: u32,
    size_log2: c_int,
) -> u32 {
    let s = opaque as *mut VIRTIODevice;
    let offset: u32 = offset1 & 0xfff;
    let mut val: u32 = 0;

    match offset1 >> 12 {
        0 => {
            if size_log2 == 2 {
                match offset {
                    0x004 => {
                        val = match (*s).device_features_sel {
                            0 => (*s).device_features,
                            1 => 1,
                            _ => 0,
                        };
                    }
                    0x000 => {
                        val = (*s).device_features_sel;
                    }
                    0x020 => {
                        val = (*queue_ptr(s)).desc_addr as u32;
                    }
                    0x028 => {
                        val = (*queue_ptr(s)).avail_addr as u32;
                    }
                    0x030 => {
                        val = (*queue_ptr(s)).used_addr as u32;
                    }
                    0x024 => {
                        val = ((*queue_ptr(s)).desc_addr >> 32) as u32;
                    }
                    0x02c => {
                        val = ((*queue_ptr(s)).avail_addr >> 32) as u32;
                    }
                    0x034 => {
                        val = ((*queue_ptr(s)).used_addr >> 32) as u32;
                    }
                    _ => {}
                }
            } else if size_log2 == 1 {
                match offset {
                    0x012 => {
                        val = 16;
                    }
                    0x016 => {
                        val = (*s).queue_sel;
                    }
                    0x018 => {
                        val = (*queue_ptr(s)).num;
                    }
                    0x01c => {
                        val = (*queue_ptr(s)).ready;
                    }
                    0x01e => {
                        val = 0;
                    }
                    _ => {}
                }
            } else if size_log2 == 0 {
                match offset {
                    0x014 => {
                        val = (*s).status;
                    }
                    _ => {}
                }
            }
        }
        1 => {
            if offset == 0 && size_log2 == 0 {
                val = (*s).int_status;
                (*s).int_status = 0;
                let irq = (*s).irq;
                if !irq.is_null() {
                    if let Some(f) = (*irq).set_irq {
                        f((*irq).opaque, (*irq).irq_num, 0);
                    }
                }
            }
        }
        2 => {
            val = virtio_config_read(s, offset, size_log2);
        }
        _ => {}
    }

    val
}
}

// ── `return val;` · virtio.c:886-886 · chunk 3781 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3781 {
// Ported piece of virtio_pci_read: the final `return val;`.
#[no_mangle]
pub extern "C" fn virtio_pci_read__part3781(val: u32) -> u32 {
    val
}
}

// ── `VIRTIODevice *s = opaque; uint32_t offset; offset = offset1 & 0xfff;` · virtio.c:892-901 · chunk 3783 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VIRTIODevice *s = opaque;
//         uint32_t offset;
//     
//     
//     
//     
//     
//     
//     
//         offset = offset1 & 0xfff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3783 {
// Computes the register offset from the raw PCI offset.
// Ported from virtio_pci_write() in virtio.c.
#[no_mangle]
pub extern "C" fn virtio_pci_write__part3783(offset1: core::ffi::c_uint) -> core::ffi::c_uint {
    offset1 & 0xfff
}
}

// ── `s->debug = debug;` · virtio.c:970-970 · chunk 3786 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->debug = debug;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3786 {
// Portion of virtio_set_debug: store the debug flag into the device state.
use core::ffi::{c_int, c_void};

// Mirror of `struct VIRTIODevice` up to (and including) the `debug` field so
// that the field offsets match the C layout exactly.
#[repr(C)]
struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut c_void,
    get_ram_ptr: *mut c_void,
    debug: c_int,
}

#[no_mangle]
pub extern "C" fn virtio_set_debug__part3786(s: *mut VIRTIODevice, debug: c_int) {
    unsafe {
        (*s).debug = debug;
    }
}
}

// ── `s->int_status |= 2; set_irq(s->irq, 1);` · virtio.c:976-977 · chunk 3788 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->int_status |= 2;
//         set_irq(s->irq, 1);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3788 {
// Ported piece of virtio_config_change_notify (virtio.c lines 976-977):
//   s->int_status |= 2;
//   set_irq(s->irq, 1);
// set_irq() is a static inline helper in C, so its body is inlined here:
//   irq->set_irq(irq->opaque, irq->irq_num, level);

use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// C struct VIRTIODevice: fields are declared in order; only the prefix up to
// int_status is needed here, but the offsets must match the C layout exactly.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
}

#[no_mangle]
pub unsafe extern "C" fn virtio_config_change_notify__part3788(s: *mut VIRTIODevice) {
    (*s).int_status |= 2;
    let irq = (*s).irq;
    let f = (*irq).set_irq;
    let opaque = (*irq).opaque;
    let irq_num = (*irq).irq_num;
    match f {
        Some(cb) => cb(opaque, irq_num, 1),
        None => {}
    }
}
}

// ── `VIRTIODevice *s = opaque; VIRTIOBlockDevice *s1 = (VIRTIOBlockDevice *)s; virtio_block_req_end(…` · virtio.c:1052-1060 · chunk 3798 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     VIRTIODevice *s = opaque;
//         VIRTIOBlockDevice *s1 = (VIRTIOBlockDevice *)s;
//     
//         virtio_block_req_end(s, ret);
//     
//         s1->req_in_progress = FALSE;
//     
//     
//         queue_notify((VIRTIODevice *)s, s1->req.queue_idx);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3798 {
// Rust port of the body of virtio_block_req_cb (virtio.c lines 1052-1060).
//
// The helpers used by the TARGET are `static` C functions (no linker symbol),
// so their logic is reproduced here as private Rust fns:
//   virtio_block_req_end -> virtio_consume_desc -> memcpy_to_queue
//   -> memcpy_to_from_queue -> get_desc -> virtio_memcpy_from/to_ram
//   queue_notify -> get_desc_rw_size -> virtio_read16 -> set_irq (inline)
// C library functions (memcpy, free, abort) are declared in the extern block.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;
type virtio_phys_addr_t = u64;

const FALSE: BOOL = 0;
const TRUE: BOOL = 1;

#[repr(C)]
struct VIRTIODesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C)]
struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: virtio_phys_addr_t,
    avail_addr: virtio_phys_addr_t,
    used_addr: virtio_phys_addr_t,
    manual_recv: BOOL,
}

type VIRTIODeviceRecvFunc =
    unsafe extern "C" fn(s1: *mut VIRTIODevice, queue_idx: c_int, desc_idx: c_int,
                         read_size: c_int, write_size: c_int) -> c_int;

type VIRTIOGetRAMPtrFunc =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: virtio_phys_addr_t, is_rw: BOOL) -> *mut u8;

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut IRQSignal,
    get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: Option<VIRTIODeviceRecvFunc>,
    config_write: Option<unsafe extern "C" fn(s: *mut VIRTIODevice)>,
    config_space_size: u32,
    config_space: [u8; 256],
}

#[repr(C)]
struct BlockRequest {
    r#type: u32,
    buf: *mut u8,
    write_size: c_int,
    queue_idx: c_int,
    desc_idx: c_int,
}

#[repr(C)]
struct VIRTIOBlockDevice {
    common: VIRTIODevice,
    bs: *mut c_void,
    req_in_progress: BOOL,
    req: BlockRequest,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn abort() -> !;
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// ---- static inline helpers from the translation unit ----

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, FALSE);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t, val: u16) {
    if (addr & 1) != 0 {
        return;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return,
    };
    let ptr = f(s, addr, TRUE);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: virtio_phys_addr_t, val: u32) {
    if (addr & 3) != 0 {
        return;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return,
    };
    let ptr = f(s, addr, TRUE);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    mut addr: virtio_phys_addr_t,
    mut count: c_int,
) -> c_int {
    let mut cur = buf;
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, FALSE);
        if ptr.is_null() {
            return -1;
        }
        memcpy(cur as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        cur = cur.offset(l as isize);
        count = count.wrapping_sub(l);
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: virtio_phys_addr_t,
    buf: *const u8,
    mut count: c_int,
) -> c_int {
    let mut cur = buf;
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, TRUE);
        if ptr.is_null() {
            return -1;
        }
        memcpy(ptr as *mut c_void, cur as *const c_void, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        cur = cur.offset(l as isize);
        count = count.wrapping_sub(l);
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_ptr().offset(queue_idx as isize);
    let off = ((desc_idx as i64) as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: BOOL,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let f_write_flag: u16;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc, queue_idx, desc_idx);

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if (desc.flags & 2) == f_write_flag {
                break;
            }
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if (desc.flags & 2) != f_write_flag {
            return -1;
        }
        if (offset as u32) < desc.len {
            break;
        }
        if (desc.flags & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = offset.wrapping_sub(desc.len as c_int);
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    let mut cur = buf;
    loop {
        let d = desc.len.wrapping_sub(offset as u32) as c_int;
        let l = min_int(count, d);
        let a = desc.addr.wrapping_add(offset as i64 as u64);
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, a, cur, l);
        } else {
            virtio_memcpy_from_ram(s, cur, a, l);
        }
        count = count.wrapping_sub(l);
        if count == 0 {
            break;
        }
        offset = offset.wrapping_add(l);
        cur = cur.offset(l as isize);
        if (offset as u32) == desc.len {
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
            if (desc.flags & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

unsafe fn memcpy_to_queue(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    buf: *const c_void,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, TRUE)
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let mut addr = (*qs).used_addr.wrapping_add(2);
    let index: u32 = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, index.wrapping_add(1) as u16);

    let x: u32 = (index & (*qs).num.wrapping_sub(1)).wrapping_mul(8);
    addr = (*qs).used_addr.wrapping_add(4).wrapping_add(x as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    let irq = (*s).irq;
    if irq.is_null() {
        return;
    }
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, 1);
    }
}

unsafe fn get_desc_rw_size(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    mut desc_idx: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;

    get_desc(s, &mut desc, queue_idx, desc_idx);

    loop {
        if (desc.flags & 2) != 0 {
            break;
        }
        read_size = read_size.wrapping_add(desc.len as c_int);
        if (desc.flags & 1) == 0 {
            *pread_size = read_size;
            *pwrite_size = write_size;
            return 0;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        if (desc.flags & 2) == 0 {
            return -1;
        }
        write_size = write_size.wrapping_add(desc.len as c_int);
        if (desc.flags & 1) == 0 {
            break;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    *pread_size = read_size;
    *pwrite_size = write_size;
    0
}

unsafe fn queue_notify(s: *mut VIRTIODevice, queue_idx: c_int) {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;

    if (*qs).manual_recv != 0 {
        return;
    }

    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    while (*qs).last_avail_idx != avail_idx {
        let x: u32 = ((*qs).last_avail_idx as u32) & (*qs).num.wrapping_sub(1);
        let addr = (*qs).avail_addr.wrapping_add(4).wrapping_add(x.wrapping_mul(2) as u64);
        let desc_idx: c_int = virtio_read16(s, addr) as c_int;
        if get_desc_rw_size(s, &mut read_size, &mut write_size, queue_idx, desc_idx) == 0 {
            let recv = match (*s).device_recv {
                Some(f) => f,
                None => break,
            };
            if recv(s, queue_idx, desc_idx, read_size, write_size) < 0 {
                break;
            }
        }
        (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    }
}

unsafe fn virtio_block_req_end(s: *mut VIRTIODevice, ret: c_int) {
    let s1 = s as *mut VIRTIOBlockDevice;
    let queue_idx = (*s1).req.queue_idx;
    let desc_idx = (*s1).req.desc_idx;
    let mut buf1: [u8; 1] = [0];

    match (*s1).req.r#type {
        0 => {
            let write_size = (*s1).req.write_size;
            let buf = (*s1).req.buf;
            if ret < 0 {
                *buf.offset(write_size.wrapping_sub(1) as isize) = 1;
            } else {
                *buf.offset(write_size.wrapping_sub(1) as isize) = 0;
            }
            memcpy_to_queue(
                s,
                queue_idx,
                desc_idx,
                0,
                buf as *const c_void,
                write_size,
            );
            free(buf as *mut c_void);
            virtio_consume_desc(s, queue_idx, desc_idx, write_size);
        }
        1 => {
            if ret < 0 {
                *buf1.as_mut_ptr() = 1;
            } else {
                *buf1.as_mut_ptr() = 0;
            }
            memcpy_to_queue(
                s,
                queue_idx,
                desc_idx,
                0,
                buf1.as_ptr() as *const c_void,
                core::mem::size_of::<[u8; 1]>() as c_int,
            );
            virtio_consume_desc(s, queue_idx, desc_idx, 1);
        }
        _ => abort(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_block_req_cb__part3798(opaque: *mut c_void, ret: c_int) {
    let s = opaque as *mut VIRTIODevice;
    let s1 = s as *mut VIRTIOBlockDevice;

    virtio_block_req_end(s, ret);

    (*s1).req_in_progress = FALSE;

    queue_notify(s, (*s1).req.queue_idx);
}
}

// ── `if (s1->req_in_progress)` · virtio.c:1074-1075 · chunk 3800 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s1->req_in_progress)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3800 {
// Port of `virtio_block_recv_request__part3800`:
//     if (s1->req_in_progress) return -1;
// Returns 1 when the request slot is busy (the C caller then returns -1),
// 0 otherwise.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct BlockRequest {
    pub r#type: u32,
    pub buf: *mut u8,
    pub write_size: c_int,
    pub queue_idx: c_int,
    pub desc_idx: c_int,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIOBlockDevice {
    pub common: VIRTIODevice,
    pub bs: *mut c_void,
    pub req_in_progress: c_int,
    pub req: BlockRequest,
}

/// Returns 1 if a request is already in progress, else 0.
#[no_mangle]
pub extern "C" fn virtio_block_recv_request__part3800(s1: *mut VIRTIOBlockDevice) -> c_int {
    if s1.is_null() {
        return 0;
    }
    let in_progress = unsafe { (*s1).req_in_progress };
    if in_progress != 0 {
        1
    } else {
        0
    }
}
}

// ── `if (memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, sizeof(h)) < 0)` · virtio.c:1077-1078 · chunk 3801 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, sizeof(h)) < 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3801 {
// Port of the `memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, sizeof(h)) < 0`
// check inside virtio_block_recv_request (virtio.c lines 1077-1078).
// The whole chain memcpy_from_queue -> memcpy_to_from_queue -> get_desc ->
// virtio_memcpy_from_ram consists of C `static` functions (no linker symbols),
// so their logic is reproduced here as private Rust fns.

use core::ffi::{c_int, c_void};

type Bool = c_int;

const VIRTIODESC_SIZE: c_int = 16;

#[repr(C)]
struct VirtioDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C)]
struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: Bool,
}

type GetRamPtrFn = unsafe extern "C" fn(*mut VirtioDevice, u64, Bool) -> *mut u8;

// Exact mirror of `struct VIRTIODevice` (only `queue[].desc_addr` and
// `get_ram_ptr` are actually used here, the rest keeps the layout right).
#[repr(C)]
struct VirtioDevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut c_void,
    get_ram_ptr: Option<GetRamPtrFn>,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: *mut c_void,
    config_write: *mut c_void,
    config_space_size: u32,
    config_space: [u8; 256],
}

#[inline]
unsafe fn rs_min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
//                                   virtio_phys_addr_t addr, int count)
unsafe fn rs_virtio_memcpy_from_ram(
    s: *mut VirtioDevice,
    buf: *mut u8,
    addr: u64,
    count: c_int,
) -> c_int {
    let mut addr = addr;
    let mut buf = buf;
    let mut count = count;

    while count > 0 {
        let l = rs_min_int(count, 4096 - ((addr & (4096 - 1)) as c_int));
        let get = (*s).get_ram_ptr;
        let ptr = match get {
            Some(f) => f(s, addr, 0),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr as *const u8, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

// static int virtio_memcpy_to_ram(VIRTIODevice *s, virtio_phys_addr_t addr,
//                                 const uint8_t *buf, int count)
unsafe fn rs_virtio_memcpy_to_ram(
    s: *mut VirtioDevice,
    addr: u64,
    buf: *const u8,
    count: c_int,
) -> c_int {
    let mut addr = addr;
    let mut buf = buf;
    let mut count = count;

    while count > 0 {
        let l = rs_min_int(count, 4096 - ((addr & (4096 - 1)) as c_int));
        let get = (*s).get_ram_ptr;
        let ptr = match get {
            Some(f) => f(s, addr, 1),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

// static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
//                     int queue_idx, int desc_idx)
unsafe fn rs_get_desc(
    s: *mut VirtioDevice,
    desc: *mut VirtioDesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    rs_virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as u64).wrapping_mul(VIRTIODESC_SIZE as u64)),
        VIRTIODESC_SIZE,
    )
}

// static int memcpy_to_from_queue(VIRTIODevice *s, uint8_t *buf,
//                                 int queue_idx, int desc_idx,
//                                 int offset, int count, BOOL to_queue)
unsafe fn rs_memcpy_to_from_queue(
    s: *mut VirtioDevice,
    buf: *mut u8,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    count: c_int,
    to_queue: Bool,
) -> c_int {
    let mut desc: VirtioDesc = core::mem::zeroed();
    let mut desc_idx = desc_idx;
    let mut offset = offset;
    let mut count = count;
    let mut buf = buf;
    let f_write_flag: c_int;

    if count == 0 {
        return 0;
    }

    rs_get_desc(s, &mut desc, queue_idx, desc_idx);

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if (desc.flags & 2) as c_int == f_write_flag {
                break;
            }
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            rs_get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if (desc.flags & 2) as c_int != f_write_flag {
            return -1;
        }
        if offset < desc.len as c_int {
            break;
        }
        if (desc.flags & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = offset.wrapping_sub(desc.len as c_int);
        rs_get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        let l = rs_min_int(count, (desc.len as c_int).wrapping_sub(offset));
        if to_queue != 0 {
            rs_virtio_memcpy_to_ram(s, desc.addr.wrapping_add(offset as u64), buf, l);
        } else {
            rs_virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as u64), l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset = offset.wrapping_add(l);
        buf = buf.add(l as usize);
        if offset == desc.len as c_int {
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            rs_get_desc(s, &mut desc, queue_idx, desc_idx);
            if (desc.flags & 2) as c_int != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

// static int memcpy_from_queue(VIRTIODevice *s, void *buf,
//                              int queue_idx, int desc_idx,
//                              int offset, int count)
unsafe fn rs_memcpy_from_queue(
    s: *mut VirtioDevice,
    buf: *mut u8,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    count: c_int,
) -> c_int {
    rs_memcpy_to_from_queue(s, buf, queue_idx, desc_idx, offset, count, 0)
}

// TARGET: `if (memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, sizeof(h)) < 0) return 0;`
// Returns 1 when the original condition was true (caller returns 0), else 0.
// `count` is sizeof(BlockRequestHeader) == 16, matching the C expression.
#[no_mangle]
pub extern "C" fn virtio_block_recv_request__part3801(
    s: *mut c_void,
    h: *mut c_void,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    unsafe {
        let ret = rs_memcpy_from_queue(
            s as *mut VirtioDevice,
            h as *mut u8,
            queue_idx,
            desc_idx,
            0,
            VIRTIODESC_SIZE,
        );
        if ret < 0 {
            1
        } else {
            0
        }
    }
}
}

// ── `s1->req.type = h.type; s1->req.queue_idx = queue_idx; s1->req.desc_idx = desc_idx;` · virtio.c:1079-1081 · chunk 3802 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s1->req.type = h.type;
//         s1->req.queue_idx = queue_idx;
//         s1->req.desc_idx = desc_idx;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3802 {
// Ported piece of virtio_block_recv_request: fill in the pending request fields.

use core::ffi::c_int;

#[repr(C)]
pub struct BlockRequest {
    pub r#type: u32,
    pub buf: *mut u8,
    pub write_size: c_int,
    pub queue_idx: c_int,
    pub desc_idx: c_int,
}

#[repr(C)]
pub struct BlockRequestHeader {
    pub r#type: u32,
    pub ioprio: u32,
    pub sector_num: u64,
}

#[no_mangle]
pub unsafe extern "C" fn virtio_block_recv_request__part3802(
    req: *mut BlockRequest,
    h: *const BlockRequestHeader,
    queue_idx: c_int,
    desc_idx: c_int,
) {
    if req.is_null() || h.is_null() {
        return;
    }
    (*req).r#type = (*h).r#type;
    (*req).queue_idx = queue_idx;
    (*req).desc_idx = desc_idx;
}
}

// ── `return 0;` · virtio.c:1114-1114 · chunk 3804 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3804 {
// Piece of virtio_block_recv_request: the final `return 0;`.
// No locals or parameters are read, so nothing is passed across the boundary.

#[no_mangle]
pub extern "C" fn virtio_block_recv_request__part3804() -> core::ffi::c_int {
    0
}
}

// ── `if (queue_idx == 1)` · virtio.c:1163-1173 · chunk 3812 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (queue_idx == 1) {
//     
//             if (memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, s1->header_size) < 0)
//                 return 0;
//             len = read_size - s1->header_size;
//             buf = malloc(len);
//             memcpy_from_queue(s, buf, queue_idx, desc_idx, s1->header_size, len);
//             es->write_packet(es, buf, len);
//             free(buf);
//             virtio_consume_desc(s, queue_idx, desc_idx, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3812 {
// Port of the `if (queue_idx == 1) { ... }` block of virtio_net_recv_request
// (virtio.c).  The `static` helpers reachable from that block
// (memcpy_from_queue -> memcpy_to_from_queue, get_desc, virtio_memcpy_from_ram,
// virtio_consume_desc, virtio_read16/write16/write32) have no linker symbols,
// so their logic is ported here as private Rust functions.

use core::ffi::{c_int, c_void};
use core::mem::MaybeUninit;

type VirtioPhysAddr = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, VirtioPhysAddr, c_int) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct EthernetDevice {
    pub mac_addr: [u8; 6],
    pub write_packet: Option<unsafe extern "C" fn(*mut EthernetDevice, *const u8, c_int)>,
    pub opaque: *mut c_void,
    pub select_fill: *mut c_void,
    pub select_poll: *mut c_void,
    pub device_opaque: *mut c_void,
    pub device_can_write_packet: *mut c_void,
    pub device_write_packet: *mut c_void,
    pub device_set_carrier: *mut c_void,
}

#[repr(C)]
pub struct VIRTIONetDevice {
    pub common: VIRTIODevice,
    pub es: *mut EthernetDevice,
    pub header_size: c_int,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: VirtioPhysAddr,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & 4095u64)) as c_int);
        let get_ram_ptr = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr as *const u8, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.wrapping_offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: VirtioPhysAddr,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & 4095u64)) as c_int);
        let get_ram_ptr = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.wrapping_offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as u32 as u64).wrapping_mul(16)),
        16,
    )
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: c_int,
) -> c_int {
    let mut desc = MaybeUninit::<VIRTIODesc>::uninit();
    let f_write_flag: c_int;

    if count == 0 {
        return 0;
    }

    get_desc(s, desc.as_mut_ptr(), queue_idx, desc_idx);
    let mut desc = desc.assume_init();

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if ((desc.flags & 2) as c_int) == f_write_flag {
                break;
            }
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if ((desc.flags & 2) as c_int) != f_write_flag {
            return -1;
        }
        if (offset as u32) < desc.len {
            break;
        }
        if (desc.flags & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = offset.wrapping_sub(desc.len as c_int);
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(
            count,
            desc.len.wrapping_sub(offset as u32) as c_int,
        );
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, desc.addr.wrapping_add(offset as u64), buf, l);
        } else {
            virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as u64), l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset += l;
        buf = buf.wrapping_offset(l as isize);
        if offset as u32 == desc.len {
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
            if ((desc.flags & 2) as c_int) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

unsafe fn memcpy_from_queue(
    s: *mut VIRTIODevice,
    buf: *mut c_void,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 0)
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: VirtioPhysAddr) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: VirtioPhysAddr, val: u16) {
    if (addr & 1) != 0 {
        return;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return,
    };
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val)
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: VirtioPhysAddr, val: u32) {
    if (addr & 3) != 0 {
        return;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return,
    };
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val)
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let num = (*qs).num;

    let mut addr = (*qs).used_addr.wrapping_add(2);
    let index = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, (index.wrapping_add(1) & 0xffff) as u16);

    addr = (*qs)
        .used_addr
        .wrapping_add(4)
        .wrapping_add((index & num.wrapping_sub(1)).wrapping_mul(8) as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    let irq = (*s).irq as *mut IRQSignal;
    if !irq.is_null() {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

// Returns 1 when the original block would have executed `return 0;`, else 0.
#[no_mangle]
pub unsafe extern "C" fn virtio_net_recv_request__part3812(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    read_size: c_int,
    _write_size: c_int,
) -> c_int {
    if queue_idx == 1 {
        let s1 = s as *mut VIRTIONetDevice;
        let es = (*s1).es;
        let header_size = (*s1).header_size;

        let mut h = MaybeUninit::<[u8; 12]>::uninit();
        if memcpy_from_queue(
            s,
            h.as_mut_ptr() as *mut c_void,
            queue_idx,
            desc_idx,
            0,
            header_size,
        ) < 0
        {
            return 1;
        }

        let len = read_size.wrapping_sub(header_size);
        let buf = malloc(len as usize) as *mut u8;
        memcpy_from_queue(
            s,
            buf as *mut c_void,
            queue_idx,
            desc_idx,
            header_size,
            len,
        );
        if !es.is_null() {
            if let Some(f) = (*es).write_packet {
                f(es, buf, len);
            }
        }
        free(buf as *mut c_void);
        virtio_consume_desc(s, queue_idx, desc_idx, 0);
    }
    0
}
}

// ── `return 0;` · virtio.c:1174-1174 · chunk 3813 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3813 {
// Ported piece of virtio_net_recv_request: the function's final `return 0;`.
#[no_mangle]
pub extern "C" fn virtio_net_recv_request__part3813() -> core::ffi::c_int {
    0
}
}

// ── `if (!qs->ready)` · virtio.c:1183-1184 · chunk 3815 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return FALSE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3815 {
// Port of `if (!qs->ready) return FALSE;` from virtio_net_can_write_packet.
// Returns 1 when the queue is ready (caller continues), 0 otherwise
// (caller executes `return FALSE;`).

use core::ffi::{c_int, c_uint};

#[repr(C)]
pub struct QueueState {
    pub ready: c_uint,
    pub num: c_uint,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[no_mangle]
pub extern "C" fn virtio_net_can_write_packet__part3815(qs: *const QueueState) -> c_int {
    if qs.is_null() {
        return 0;
    }
    let ready = unsafe { (*qs).ready };
    if ready == 0 {
        0
    } else {
        1
    }
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2); return qs->last_avail_idx != avail_idx;` · virtio.c:1185-1186 · chunk 3816 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
//         return qs->last_avail_idx != avail_idx;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3816 {
// Port of virtio_net_can_write_packet's tail: read the avail ring index and
// compare it with last_avail_idx.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;
type virtio_phys_addr_t = u64;

const FALSE: BOOL = 0;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: virtio_phys_addr_t,
    pub avail_addr: virtio_phys_addr_t,
    pub used_addr: virtio_phys_addr_t,
    pub manual_recv: BOOL,
}

type VIRTIOGetRAMPtrFunc = unsafe extern "C" fn(
    s: *mut VIRTIODevice,
    paddr: virtio_phys_addr_t,
    is_rw: BOOL,
) -> *mut u8;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

// Ported from the C static virtio_read16() (no linker symbol exists for it).
unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, FALSE);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_net_can_write_packet__part3816(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
) -> BOOL {
    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    ((*qs).last_avail_idx != avail_idx) as BOOL
}
}

// ── `if (!qs->ready)` · virtio.c:1200-1201 · chunk 3818 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3818 {
// Ported piece of virtio_net_write_packet: the `if (!qs->ready) return;` guard.
// Returns non-zero when the C code must return early (queue not ready).

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: i32,
}

#[no_mangle]
pub extern "C" fn virtio_net_write_packet__part3818(qs: *const QueueState) -> i32 {
    let ready = unsafe { (*qs).ready };
    let early = ready == 0;
    early as i32
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2);` · virtio.c:1202-1202 · chunk 3819 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3819 {
// Port of virtio_read16(s, qs->avail_addr + 2) for virtio_net_write_packet.
// virtio_read16 is a static C function, so its logic is reproduced here.

use core::ffi::{c_int, c_void};

// Prefix of the C struct VIRTIODevice (fields in order, identical types),
// sufficient to reach get_ram_ptr.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr:
        Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8>,
}

#[no_mangle]
pub extern "C" fn virtio_net_write_packet__part3819(
    s: *mut VIRTIODevice,
    addr: u64,
) -> u16 {
    // static uint16_t virtio_read16(VIRTIODevice *s, virtio_phys_addr_t addr)
    if (addr & 1) != 0 {
        return 0;
    }
    if s.is_null() {
        return 0;
    }
    let get_ram_ptr = unsafe { (*s).get_ram_ptr };
    let ptr = match get_ram_ptr {
        Some(f) => unsafe { f(s, addr, 0) },
        None => return 0,
    };
    if ptr.is_null() {
        return 0;
    }
    unsafe { core::ptr::read_unaligned(ptr as *const u16) }
}
}

// ── `if (qs->last_avail_idx == avail_idx)` · virtio.c:1203-1204 · chunk 3820 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (qs->last_avail_idx == avail_idx)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3820 {
// Port of the "no new available descriptor" early-out from
// virtio_net_write_packet:  if (qs->last_avail_idx == avail_idx) return;

use core::ffi::c_int;

// Mirror of the C QueueState (virtio.c) -- exact fields, in order, same types.
#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

/// Returns nonzero when last_avail_idx already equals avail_idx, i.e. when the
/// caller must return early.
#[no_mangle]
pub extern "C" fn virtio_net_write_packet__part3820(
    qs: *const QueueState,
    avail_idx: u16,
) -> c_int {
    if qs.is_null() {
        return 0;
    }
    let last_avail_idx = unsafe { (*qs).last_avail_idx };
    if last_avail_idx == avail_idx {
        1
    } else {
        0
    }
}
}

// ── `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2);` · virtio.c:1205-1206 · chunk 3821 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     desc_idx = virtio_read16(s, qs->avail_addr + 4 +
//                                  (qs->last_avail_idx & (qs->num - 1)) * 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3821 {
// Port of the `desc_idx = virtio_read16(...)` statement of
// virtio_net_write_packet, together with the logic of the static C helper
// virtio_read16 (which has no linker symbol, so it is reimplemented here).

use core::ffi::{c_int, c_void};

pub type BOOL = c_int;
pub type VirtioPhysAddr = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: BOOL,
}

type VirtioGetRAMPtrFn =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: VirtioPhysAddr, is_rw: BOOL) -> *mut u8;

// Only the fields up to (and including) get_ram_ptr are needed; the remaining
// C fields come later in the struct and do not affect these offsets.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<VirtioGetRAMPtrFn>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: VirtioPhysAddr) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_net_write_packet__part3821(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
) -> u16 {
    let last_avail_idx = (*qs).last_avail_idx as u32;
    let mask = (*qs).num.wrapping_sub(1);
    let slot = (last_avail_idx & mask) as u64;
    let addr = (*qs)
        .avail_addr
        .wrapping_add(4)
        .wrapping_add(slot.wrapping_mul(2));
    virtio_read16(s, addr)
}
}

// ── `if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))` · virtio.c:1207-1208 · chunk 3822 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3822 {
// Port of the static helper get_desc_rw_size() called from
// virtio_net_write_packet(). Since get_desc_rw_size (and get_desc,
// virtio_memcpy_from_ram) are file-static and have no linker symbol,
// their logic is reproduced here as private Rust functions.

use core::ffi::{c_int, c_void};

type VirtioPhysAddr = u64; // typedef uint64_t virtio_phys_addr_t
type Bool = c_int; // typedef int BOOL

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: Bool,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

type GetRAMPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: VirtioPhysAddr, is_rw: Bool) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: GetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

// static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
//                                   virtio_phys_addr_t addr, int count)
unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: VirtioPhysAddr,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let chunk = (4096u64 - (addr & 4095)) as c_int;
        let l = min_int(count, chunk);
        let ptr = match (*s).get_ram_ptr {
            Some(f) => f(s, addr, 0),
            None => core::ptr::null_mut(),
        };
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

// static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
//                     int queue_idx, int desc_idx)
unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().offset(queue_idx as isize);
    let off = (desc_idx as i64 as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

// static int get_desc_rw_size(VIRTIODevice *s,
//                             int *pread_size, int *pwrite_size,
//                             int queue_idx, int desc_idx)
#[no_mangle]
pub unsafe extern "C" fn virtio_net_write_packet__part3822(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;
    let mut desc_idx = desc_idx;

    get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);

    loop {
        if (desc.flags as c_int) & 2 != 0 {
            break;
        }
        read_size = read_size.wrapping_add(desc.len as c_int);
        if ((desc.flags as c_int) & 1) == 0 {
            // goto done
            *pread_size = read_size;
            *pwrite_size = write_size;
            return 0;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    loop {
        if ((desc.flags as c_int) & 2) == 0 {
            return -1;
        }
        write_size = write_size.wrapping_add(desc.len as c_int);
        if ((desc.flags as c_int) & 1) == 0 {
            break;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    // done:
    *pread_size = read_size;
    *pwrite_size = write_size;
    0
}
}

// ── `len = s1->header_size + buf_len;` · virtio.c:1209-1209 · chunk 3823 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     len = s1->header_size + buf_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3823 {
// Computes the total descriptor length written to the tx queue:
//   len = s1->header_size + buf_len;
// Both operands are read by value; the result is C `int` and wraps like C.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_net_write_packet__part3823(header_size: c_int, buf_len: c_int) -> c_int {
    header_size.wrapping_add(buf_len)
}
}

// ── `if (len > write_size)` · virtio.c:1210-1211 · chunk 3824 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (len > write_size)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3824 {
use core::ffi::c_int;

// Corresponds to: if (len > write_size) return;
// Returns nonzero when the length exceeds the writable size, meaning the
// caller (virtio_net_write_packet) must return early.
#[no_mangle]
pub extern "C" fn virtio_net_write_packet__part3824(len: c_int, write_size: c_int) -> c_int {
    if len > write_size { 1 } else { 0 }
}
}

// ── `if (queue_idx == 1)` · virtio.c:1276-1283 · chunk 3832 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (queue_idx == 1) {
//     
//             buf = malloc(read_size);
//             memcpy_from_queue(s, buf, queue_idx, desc_idx, 0, read_size);
//             cs->write_data(cs->opaque, buf, read_size);
//             free(buf);
//             virtio_consume_desc(s, queue_idx, desc_idx, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3832 {
// Rust port of virtio_console_recv_request, lines 1276-1283 (the `if (queue_idx == 1)`
// block). The static C helpers it needs (memcpy_from_queue, get_desc,
// virtio_memcpy_from_ram, virtio_read16/write16/write32, virtio_consume_desc,
// set_irq) have no linker symbols, so their logic is ported here as private fns.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: Option<
        unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8,
    >,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct CharacterDevice {
    pub opaque: *mut c_void,
    pub write_data: Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *const u8, len: c_int)>,
    pub read_data:
        Option<unsafe extern "C" fn(opaque: *mut c_void, buf: *mut u8, len: c_int) -> c_int>,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
unsafe fn get_ram_ptr(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, paddr, is_rw),
        None => core::ptr::null_mut(),
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64 - (addr & 4095u64)) as c_int);
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        memcpy(buf as *mut c_void, ptr as *const c_void, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.wrapping_add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as i64 as u64).wrapping_mul(16u64)),
        16 as c_int,
    )
}

// memcpy_from_queue(s, buf, queue_idx, desc_idx, offset, count)
// == memcpy_to_from_queue(..., to_queue = FALSE)
unsafe fn memcpy_from_queue(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed::<VIRTIODesc>();

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);

    // to_queue is FALSE here, so f_write_flag == 0.
    loop {
        if (desc.flags & 2) != 0 {
            return -1;
        }
        if (offset as u32) < desc.len {
            break;
        }
        if (desc.flags & 1) == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = (offset as u32).wrapping_sub(desc.len) as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(count, desc.len.wrapping_sub(offset as u32) as c_int);
        virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as i64 as u64), l);
        count -= l;
        if count == 0 {
            break;
        }
        offset = offset.wrapping_add(l);
        let buf = buf.wrapping_add(l as usize);
        if (offset as u32) == desc.len {
            if (desc.flags & 1) == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
            if (desc.flags & 2) != 0 {
                return -1;
            }
            offset = 0;
        }
        let _ = buf;
    }
    0
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if (addr & 1) != 0 {
        return 0;
    }
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: u64, val: u16) {
    if (addr & 1) != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: u64, val: u32) {
    if (addr & 3) != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let mut addr: u64 = (*qs).used_addr.wrapping_add(2);
    let index: u16 = virtio_read16(s, addr);
    virtio_write16(s, addr, index.wrapping_add(1));

    addr = (*qs)
        .used_addr
        .wrapping_add(4)
        .wrapping_add(((index as u32) & (*qs).num.wrapping_sub(1)).wrapping_mul(8) as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    set_irq((*s).irq, 1);
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_recv_request__part3832(
    s: *mut VIRTIODevice,
    cs: *mut CharacterDevice,
    queue_idx: c_int,
    desc_idx: c_int,
    read_size: c_int,
) {
    if queue_idx == 1 {
        let buf: *mut u8 = malloc(read_size as usize) as *mut u8;
        memcpy_from_queue(s, buf, queue_idx, desc_idx, 0, read_size);
        if let Some(f) = (*cs).write_data {
            f((*cs).opaque, buf, read_size);
        }
        free(buf as *mut c_void);
        virtio_consume_desc(s, queue_idx, desc_idx, 0);
    }
}
}

// ── `return 0;` · virtio.c:1284-1284 · chunk 3833 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3833 {
// Ported piece of virtio_console_recv_request: the final `return 0;`.
// The target reads and writes no locals or parameters; it simply yields 0.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_console_recv_request__part3833() -> c_int {
    0
}
}

// ── `if (!qs->ready)` · virtio.c:1292-1293 · chunk 3835 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return FALSE;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3835 {
// Ported piece of virtio_console_can_write_data: the "queue not ready" guard.
use core::ffi::c_int;

pub type VirtioPhysAddr = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: c_int,
}

// BOOL is `typedef int BOOL;` in C. Returns 0 to continue, 1 when the target
// C code takes `return FALSE;`.
#[no_mangle]
pub extern "C" fn virtio_console_can_write_data__part3835(qs: *const QueueState) -> c_int {
    if qs.is_null() {
        return 0;
    }
    let ready = unsafe { (*qs).ready };
    if ready == 0 {
        return 1;
    }
    0
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2); return qs->last_avail_idx != avail_idx;` · virtio.c:1294-1295 · chunk 3836 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
//         return qs->last_avail_idx != avail_idx;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3836 {
// Port of virtio_console_can_write_data's tail:
//   avail_idx = virtio_read16(s, qs->avail_addr + 2);
//   return qs->last_avail_idx != avail_idx;
// virtio_read16 is a `static` C function (no linker symbol), so its logic is
// reproduced here as a private Rust fn.

use core::ffi::{c_int, c_void};

type Bool = c_int;
type VirtioPhysAddr = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: Bool,
}

type GetRamPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: VirtioPhysAddr, is_rw: Bool) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: GetRamPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

// Private port of the C `static uint16_t virtio_read16(VIRTIODevice *s,
// virtio_phys_addr_t addr)`.
unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: VirtioPhysAddr) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_can_write_data__part3836(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
) -> Bool {
    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    if (*qs).last_avail_idx != avail_idx {
        1
    } else {
        0
    }
}
}

// ── `if (!qs->ready)` · virtio.c:1306-1307 · chunk 3838 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3838 {
// Port of the `if (!qs->ready) return 0;` guard of virtio_console_get_write_len.
// Returns nonzero when the caller must bail out with 0 (i.e. the queue is not ready).

#[no_mangle]
pub extern "C" fn virtio_console_get_write_len__part3838(ready: u32) -> core::ffi::c_int {
    if ready == 0 {
        1
    } else {
        0
    }
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2);` · virtio.c:1308-1308 · chunk 3839 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3839 {
// Port of virtio_console_get_write_len's `avail_idx = virtio_read16(s, qs->avail_addr + 2);`
// virtio_read16 is a static C function, so its logic is mirrored here as a private fn.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;
type virtio_phys_addr_t = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: virtio_phys_addr_t,
    pub avail_addr: virtio_phys_addr_t,
    pub used_addr: virtio_phys_addr_t,
    pub manual_recv: BOOL,
}

type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: virtio_phys_addr_t, is_rw: BOOL) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: VIRTIOGetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    if s.is_null() {
        return 0;
    }
    let f = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_get_write_len__part3839(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
) -> u16 {
    if qs.is_null() {
        return 0;
    }
    virtio_read16(s, (*qs).avail_addr.wrapping_add(2))
}
}

// ── `if (qs->last_avail_idx == avail_idx)` · virtio.c:1309-1310 · chunk 3840 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (qs->last_avail_idx == avail_idx)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3840 {
// Portion of virtio_console_get_write_len: check whether the queue has no
// pending available descriptor.

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: core::ffi::c_int,
}

#[no_mangle]
pub extern "C" fn virtio_console_get_write_len__part3840(
    qs: *const QueueState,
    avail_idx: u16,
) -> core::ffi::c_int {
    unsafe {
        if (*qs).last_avail_idx == avail_idx {
            1
        } else {
            0
        }
    }
}
}

// ── `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2);` · virtio.c:1311-1312 · chunk 3841 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     desc_idx = virtio_read16(s, qs->avail_addr + 4 +
//                                  (qs->last_avail_idx & (qs->num - 1)) * 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3841 {
// Port of the descriptor index lookup piece of virtio_console_get_write_len.
// Mirrors of the C types accessed through pointers (identical fields, order, types).

use core::ffi::c_int;
use core::ffi::c_void;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    // padding inserted by the C compiler before the 8-byte aligned fields
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

// Only the fields up to and including get_ram_ptr are needed; they are laid out
// first in the C struct, so the offsets are identical.
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
}

// `virtio_read16` is a static C function (no linker symbol): ported here.
#[inline]
unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub extern "C" fn virtio_console_get_write_len__part3841(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
) -> u16 {
    unsafe {
        let avail_addr = (*qs).avail_addr;
        let last_avail_idx = (*qs).last_avail_idx;
        let num = (*qs).num;
        // (qs->last_avail_idx & (qs->num - 1)) is computed in uint32_t
        let slot = (last_avail_idx as u32) & num.wrapping_sub(1);
        // the *2 product wraps in uint32_t before being widened to uint64_t
        let offset = (slot.wrapping_mul(2)) as u64;
        let addr = avail_addr.wrapping_add(4).wrapping_add(offset);
        virtio_read16(s, addr)
    }
}
}

// ── `if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))` · virtio.c:1313-1314 · chunk 3842 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3842 {
// Port of the static C helpers get_desc_rw_size / get_desc /
// virtio_memcpy_from_ram together with the TARGET statement of
// virtio_console_get_write_len.

use core::ffi::c_int;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

pub type VIRTIOGetRAMPtrFunc =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut core::ffi::c_void,
    pub mem_range: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub irq: *mut core::ffi::c_void,
    pub get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut core::ffi::c_void,
    pub config_write: *mut core::ffi::c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    addr: u64,
    count: c_int,
) -> c_int {
    let mut buf = buf;
    let mut addr = addr;
    let mut count = count;

    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let f = match (*s).get_ram_ptr {
            Some(f) => f,
            None => return -1,
        };
        let ptr = f(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs: *mut QueueState = &mut (*s).queue[queue_idx as usize];
    let off = (desc_idx as i64 as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs).desc_addr.wrapping_add(off),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

unsafe fn get_desc_rw_size(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    desc_idx_arg: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut read_size: c_int = 0;
    let mut write_size: c_int = 0;
    let mut desc_idx = desc_idx_arg;

    get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);

    loop {
        if desc.flags & 2 != 0 {
            break;
        }
        read_size = read_size.wrapping_add(desc.len as c_int);
        if desc.flags & 1 == 0 {
            // goto done
            *pread_size = read_size;
            *pwrite_size = write_size;
            return 0;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    loop {
        if desc.flags & 2 == 0 {
            return -1;
        }
        write_size = write_size.wrapping_add(desc.len as c_int);
        if desc.flags & 1 == 0 {
            break;
        }
        desc_idx = desc.next as c_int;
        get_desc(s, &mut desc as *mut VIRTIODesc, queue_idx, desc_idx);
    }

    // done:
    *pread_size = read_size;
    *pwrite_size = write_size;
    0
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_get_write_len__part3842(
    s: *mut VIRTIODevice,
    pread_size: *mut c_int,
    pwrite_size: *mut c_int,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    get_desc_rw_size(s, pread_size, pwrite_size, queue_idx, desc_idx)
}
}

// ── `return write_size;` · virtio.c:1315-1315 · chunk 3843 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return write_size;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3843 {
// Ported piece: `return write_size;` from virtio_console_get_write_len.
#[no_mangle]
pub extern "C" fn virtio_console_get_write_len__part3843(write_size: core::ffi::c_int) -> core::ffi::c_int {
    write_size
}
}

// ── `if (!qs->ready)` · virtio.c:1325-1326 · chunk 3845 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3845 {
// Port of virtio.c lines 1325-1326 (inside virtio_console_write_data):
//     if (!qs->ready)
//         return 0;
// Returns nonzero when the caller must return 0 (queue not ready).

use core::ffi::c_int;

// virtio_phys_addr_t is uint64_t; BOOL is int.
#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[no_mangle]
pub extern "C" fn virtio_console_write_data__part3845(qs: *const QueueState) -> c_int {
    if qs.is_null() {
        return 1;
    }
    let ready = unsafe { core::ptr::addr_of!((*qs).ready).read() };
    if ready == 0 {
        1
    } else {
        0
    }
}
}

// ── `avail_idx = virtio_read16(s, qs->avail_addr + 2);` · virtio.c:1327-1327 · chunk 3846 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     avail_idx = virtio_read16(s, qs->avail_addr + 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3846 {
// Port of the statement:
//   avail_idx = virtio_read16(s, qs->avail_addr + 2);
// The C helper virtio_read16 is a static function (no linker symbol), so its
// logic is reproduced here.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

pub type VirtioGetRamPtrFunc =
    Option<unsafe extern "C" fn(s: *mut VirtioDevice, paddr: u64, is_rw: c_int) -> *mut u8>;

#[repr(C)]
pub struct VirtioDevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: VirtioGetRamPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_write_data__part3846(
    s: *mut VirtioDevice,
    qs: *mut QueueState,
) -> u16 {
    // static uint16_t virtio_read16(VIRTIODevice *s, virtio_phys_addr_t addr)
    let addr: u64 = (*qs).avail_addr.wrapping_add(2);
    if addr & 1 != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    // FALSE == 0
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}
}

// ── `if (qs->last_avail_idx == avail_idx)` · virtio.c:1328-1329 · chunk 3847 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (qs->last_avail_idx == avail_idx)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3847 {
// Portion of virtio_console_write_data: early-exit check whether there is a
// pending available buffer in the queue. Returns nonzero when the caller
// should return 0 immediately (no new descriptor available).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_console_write_data__part3847(
    last_avail_idx: u16,
    avail_idx: u16,
) -> c_int {
    if last_avail_idx == avail_idx {
        1
    } else {
        0
    }
}
}

// ── `put_le16(s->config_space + 0, width); put_le16(s->config_space + 2, height); virtio_config_chan…` · virtio.c:1342-1345 · chunk 3850 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le16(s->config_space + 0, width);
//         put_le16(s->config_space + 2, height);
//     
//         virtio_config_change_notify(s);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3850 {
// Rust port of virtio_console_resize_event's body (config-space update +
// config change notification).
use core::ffi::{c_int, c_void};

// typedef int BOOL;
type Bool = c_int;
// typedef uint64_t virtio_phys_addr_t;
type VirtioPhysAddr = u64;

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

// typedef struct { SetIRQFunc *set_irq; void *opaque; int irq_num; } IRQSignal;
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// typedef struct { uint32_t ready; uint32_t num; uint16_t last_avail_idx;
//   virtio_phys_addr_t desc_addr, avail_addr, used_addr; BOOL manual_recv; } QueueState;
#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: Bool,
}

// typedef int VIRTIODeviceRecvFunc(VIRTIODevice *s1, int queue_idx,
//                                  int desc_idx, int read_size, int write_size);
#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

// static inline void put_le16(uint8_t *ptr, uint16_t v)
#[inline]
unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let sig = &mut *irq;
    if let Some(f) = sig.set_irq {
        f(sig.opaque, sig.irq_num, level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_console_resize_event__part3850(
    s: *mut VIRTIODevice,
    width: c_int,
    height: c_int,
) {
    let dev = &mut *s;
    let cs: *mut u8 = dev.config_space.as_mut_ptr();
    put_le16(cs, width as u16);
    put_le16(cs.add(2), height as u16);

    // virtio_config_change_notify(s) -- static in C, inlined here.
    dev.int_status |= 2;
    let irq = dev.irq;
    set_irq(irq, 1);
}
}

// ── `if (queue_idx == 1)` · virtio.c:1410-1414 · chunk 3858 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (queue_idx == 1) {
//     
//     
//             virtio_consume_desc(s, queue_idx, desc_idx, 0);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3858 {
// Port of the `if (queue_idx == 1) { virtio_consume_desc(...); }` piece of
// virtio_input_recv_request, together with the static helpers it relies on.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: c_int, // BOOL == int
}

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    mem_map: *mut c_void,
    mem_range: *mut c_void,
    pci_dev: *mut c_void,
    irq: *mut IRQSignal,
    get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: *mut c_void,
    config_write: *mut c_void,
    config_space_size: u32,
    config_space: [u8; 256],
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let get = (*s).get_ram_ptr;
    let f = match get {
        Some(f) => f,
        None => return 0,
    };
    let ptr = f(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    *(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: u64, val: u16) {
    if addr & 1 != 0 {
        return;
    }
    let get = (*s).get_ram_ptr;
    let f = match get {
        Some(f) => f,
        None => return,
    };
    let ptr = f(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    *(ptr as *mut u16) = val;
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: u64, val: u32) {
    if addr & 3 != 0 {
        return;
    }
    let get = (*s).get_ram_ptr;
    let f = match get {
        Some(f) => f,
        None => return,
    };
    let ptr = f(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    *(ptr as *mut u32) = val;
}

// Ported from the static C function of the same name.
unsafe fn virtio_consume_desc(s: *mut VIRTIODevice, queue_idx: c_int, desc_idx: c_int, desc_len: c_int) {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().offset(queue_idx as isize);

    let mut addr: u64 = (*qs).used_addr.wrapping_add(2);
    let index: u32 = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, index.wrapping_add(1) as u16);

    let num = (*qs).num;
    let slot: u32 = (index & num.wrapping_sub(1)).wrapping_mul(8);
    addr = (*qs).used_addr.wrapping_add(4).wrapping_add(slot as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    let irq = (*s).irq;
    if !irq.is_null() {
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, 1);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn virtio_input_recv_request__part3858(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
) {
    if queue_idx == 1 {
        virtio_consume_desc(s, queue_idx, desc_idx, 0);
    }
}
}

// ── `return 0;` · virtio.c:1415-1415 · chunk 3859 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3859 {
// Portion of virtio_input_recv_request: the final `return 0;`.
// It reads no parameters and no locals, so it takes no arguments.

#[no_mangle]
pub extern "C" fn virtio_input_recv_request__part3859() -> core::ffi::c_int {
    0
}
}

// ── `if (!qs->ready)` · virtio.c:1429-1430 · chunk 3861 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!qs->ready)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3861 {
// Ported piece of virtio_input_queue_event: check whether the queue is ready.
// Returns 1 when the original C code would have taken the early `return -1`
// path (i.e. the ready flag is zero), 0 otherwise.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_input_queue_event__part3861(ready: *const u32) -> c_int {
    let is_ready = unsafe { *ready };
    if is_ready == 0 {
        1
    } else {
        0
    }
}
}

// ── `put_le16(buf, type); put_le16(buf + 2, code); put_le32(buf + 4, value); buf_len = 8; avail_idx…` · virtio.c:1432-1437 · chunk 3862 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le16(buf, type);
//         put_le16(buf + 2, code);
//         put_le32(buf + 4, value);
//         buf_len = 8;
//     
//         avail_idx = virtio_read16(s, qs->avail_addr + 2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3862 {
// Rust rewrite of virtio_input_queue_event lines 1432-1437:
//   put_le16(buf, type); put_le16(buf + 2, code); put_le32(buf + 4, value);
//   buf_len = 8; avail_idx = virtio_read16(s, qs->avail_addr + 2);

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

// Ported from the static C function virtio_read16 (no linker symbol).
unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let get_ram_ptr = match (*s).get_ram_ptr {
        Some(f) => f,
        None => return 0,
    };
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

#[no_mangle]
pub unsafe extern "C" fn virtio_input_queue_event__part3862(
    s: *mut VIRTIODevice,
    qs: *mut QueueState,
    type_: u16,
    code: u16,
    value: u32,
    buf: *mut u8,
    buf_len: *mut c_int,
    avail_idx: *mut u16,
) {
    // put_le16(buf, type)
    core::ptr::write_unaligned(buf, type_ as u8);
    core::ptr::write_unaligned(buf.add(1), (type_ >> 8) as u8);
    // put_le16(buf + 2, code)
    core::ptr::write_unaligned(buf.add(2), code as u8);
    core::ptr::write_unaligned(buf.add(3), (code >> 8) as u8);
    // put_le32(buf + 4, value)
    core::ptr::write_unaligned(buf.add(4), value as u8);
    core::ptr::write_unaligned(buf.add(5), (value >> 8) as u8);
    core::ptr::write_unaligned(buf.add(6), (value >> 16) as u8);
    core::ptr::write_unaligned(buf.add(7), (value >> 24) as u8);

    *buf_len = 8;

    *avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
}
}

// ── `if (qs->last_avail_idx == avail_idx)` · virtio.c:1438-1439 · chunk 3863 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (qs->last_avail_idx == avail_idx)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3863 {
// Port of the guard inside virtio_input_queue_event:
//     if (qs->last_avail_idx == avail_idx) return -1;
// Returns nonzero when the caller must return -1 (queue has no new buffer).

#[no_mangle]
pub extern "C" fn virtio_input_queue_event__part3863(
    last_avail_idx: u16,
    avail_idx: u16,
) -> core::ffi::c_int {
    if last_avail_idx == avail_idx {
        1
    } else {
        0
    }
}
}

// ── `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2); mem…` · virtio.c:1440-1446 · chunk 3864 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     desc_idx = virtio_read16(s, qs->avail_addr + 4 +
//                                  (qs->last_avail_idx & (qs->num - 1)) * 2);
//     
//         memcpy_to_queue(s, queue_idx, desc_idx, 0, buf, buf_len);
//         virtio_consume_desc(s, queue_idx, desc_idx, buf_len);
//         qs->last_avail_idx++;
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3864 {
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Port of virtio.c lines 1440-1446 (inside virtio_input_queue_event).
// The static helpers used by the TARGET (memcpy_to_queue, virtio_consume_desc,
// get_desc, virtio_memcpy_to/from_ram, virtio_read16/write16/write32, set_irq)
// have no linker symbols, so their logic is ported here as private fns.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

pub type VIRTIOGetRAMPtrFunc =
    unsafe extern "C" fn(s: *mut VIRTIODevice, paddr: u64, is_rw: c_int) -> *mut u8;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: Option<VIRTIOGetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn vget_ram_ptr(s: *mut VIRTIODevice, addr: u64, is_rw: c_int) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, is_rw),
        None => core::ptr::null_mut(),
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let ptr = vget_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: u64, val: u16) {
    if addr & 1 != 0 {
        return;
    }
    let ptr = vget_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: u64, val: u32) {
    if addr & 3 != 0 {
        return;
    }
    let ptr = vget_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    addr: u64,
    count: c_int,
) -> c_int {
    let mut addr = addr;
    let mut buf = buf;
    let mut count = count;

    while count > 0 {
        let l = min_int(count, 4096 - (addr & 4095) as c_int);
        let ptr = vget_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    addr: u64,
    buf: *const u8,
    count: c_int,
) -> c_int {
    let mut addr = addr;
    let mut buf = buf;
    let mut count = count;

    while count > 0 {
        let l = min_int(count, 4096 - (addr & 4095) as c_int);
        let ptr = vget_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = core::ptr::addr_of_mut!((*s).queue) as *mut QueueState;
    let qs = qs.offset(queue_idx as isize);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as i64 as u64).wrapping_mul(16)),
        16,
    )
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    queue_idx: c_int,
    desc_idx_in: c_int,
    offset_in: c_int,
    count_in: c_int,
    to_queue: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut desc_idx = desc_idx_in;
    let mut offset = offset_in;
    let mut count = count_in;
    let mut buf = buf;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc, queue_idx, desc_idx);

    let f_write_flag: u16;
    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if (desc.flags & 2) == f_write_flag {
                break;
            }
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if (desc.flags & 2) != f_write_flag {
            return -1;
        }
        if offset < desc.len as c_int {
            break;
        }
        if desc.flags & 1 == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = offset.wrapping_sub(desc.len as c_int);
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        let rem = (desc.len).wrapping_sub(offset as u32) as c_int;
        let l = min_int(count, rem);
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, desc.addr.wrapping_add(offset as i64 as u64), buf, l);
        } else {
            virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as i64 as u64), l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset = offset.wrapping_add(l);
        buf = buf.offset(l as isize);
        if offset == desc.len as c_int {
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
            if (desc.flags & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

unsafe fn memcpy_to_queue(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    buf: *const c_void,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 1)
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs = (core::ptr::addr_of_mut!((*s).queue) as *mut QueueState).offset(queue_idx as isize);

    let mut addr = (*qs).used_addr.wrapping_add(2);
    let index: u32 = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, index.wrapping_add(1) as u16);

    addr = (*qs)
        .used_addr
        .wrapping_add(4)
        .wrapping_add((index & (*qs).num.wrapping_sub(1)).wrapping_mul(8) as u64);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    set_irq((*s).irq, 1);
}

#[no_mangle]
pub unsafe extern "C" fn virtio_input_queue_event__part3864(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    buf: *const u8,
    buf_len: c_int,
) -> c_int {
    let qs = (core::ptr::addr_of_mut!((*s).queue) as *mut QueueState).offset(queue_idx as isize);

    let desc_idx: c_int = virtio_read16(
        s,
        (*qs)
            .avail_addr
            .wrapping_add(4)
            .wrapping_add(
                ((*qs).last_avail_idx as u32 & (*qs).num.wrapping_sub(1)).wrapping_mul(2) as u64,
            ),
    ) as c_int;

    memcpy_to_queue(
        s,
        queue_idx,
        desc_idx,
        0,
        buf as *const c_void,
        buf_len,
    );
    virtio_consume_desc(s, queue_idx, desc_idx, buf_len);
    (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    0
}
}

// ── `if (s1->type != VIRTIO_INPUT_TYPE_KEYBOARD)` · virtio.c:1455-1456 · chunk 3866 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s1->type != VIRTIO_INPUT_TYPE_KEYBOARD)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3866 {
use core::ffi::c_int;

// VIRTIO_INPUT_TYPE_KEYBOARD == 0 in VirtioInputTypeEnum.
// Returns nonzero (true) when the device's type is NOT a keyboard, which is
// exactly the condition under which the caller must return -1.
#[no_mangle]
pub extern "C" fn virtio_input_send_key_event__part3866(type_: c_int) -> c_int {
    if type_ != 0 { 1 } else { 0 }
}
}

// ── `if (ret)` · virtio.c:1458-1459 · chunk 3868 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (ret)
//             return ret;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3868 {
// Ported piece of virtio_input_send_key_event: decides whether the
// first queued event failed (nonzero return code) so the caller can
// propagate it.
#[no_mangle]
pub extern "C" fn virtio_input_send_key_event__part3868(ret: core::ffi::c_int) -> core::ffi::c_int {
    if ret != 0 {
        1
    } else {
        0
    }
}
}

// ── `return virtio_input_queue_event(s, 0x00, 0, 0);` · virtio.c:1460-1460 · chunk 3869 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     return virtio_input_queue_event(s, 0x00, 0, 0);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3869 {
// Port of the static virtio_input_queue_event(s, 0x00, 0, 0) call that is the
// trailing `return` of virtio_input_send_key_event().
// virtio_input_queue_event and everything it uses below are `static` in the
// translation unit, so their logic is mirrored here as private Rust fns.

use core::ffi::c_int;

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut core::ffi::c_void, c_int, c_int)>,
    opaque: *mut core::ffi::c_void,
    irq_num: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct QueueState {
    ready: u32,
    num: u32,
    last_avail_idx: u16,
    desc_addr: u64,
    avail_addr: u64,
    used_addr: u64,
    manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    mem_map: *mut core::ffi::c_void,
    mem_range: *mut core::ffi::c_void,
    pci_dev: *mut core::ffi::c_void,
    irq: *mut IRQSignal,
    get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, u64, c_int) -> *mut u8>,
    debug: c_int,
    int_status: u32,
    status: u32,
    device_features_sel: u32,
    queue_sel: u32,
    queue: [QueueState; 8],
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_recv: *mut core::ffi::c_void,
    config_write: *mut core::ffi::c_void,
    config_space_size: u32,
    config_space: [u8; 256],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VIRTIODesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

unsafe fn get_ram_ptr(s: *mut VIRTIODevice, addr: u64, is_rw: c_int) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, addr, is_rw),
        None => core::ptr::null_mut(),
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: u64) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: u64, val: u16) {
    if addr & 1 != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: u64, val: u32) {
    if addr & 3 != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: u64,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096 - (addr & (4096 - 1))) as c_int);
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: u64,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096 - (addr & (4096 - 1))) as c_int);
        let ptr = get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as i64 as u64);
        buf = buf.add(l as usize);
        count -= l;
    }
    0
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let size = core::mem::size_of::<VIRTIODesc>() as u64;
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as i64 as u64).wrapping_mul(size)),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    queue_idx: c_int,
    desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: c_int,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let mut desc_idx = desc_idx;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc, queue_idx, desc_idx);

    let f_write_flag: u16;
    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if (desc.flags & 2) == f_write_flag {
                break;
            }
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if (desc.flags & 2) != f_write_flag {
            return -1;
        }
        if (offset as u32) < desc.len {
            break;
        }
        if desc.flags & 1 == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset = offset.wrapping_sub(desc.len as c_int);
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(count, desc.len.wrapping_sub(offset as u32) as c_int);
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, desc.addr.wrapping_add(offset as i64 as u64), buf, l);
        } else {
            virtio_memcpy_from_ram(s, buf, desc.addr.wrapping_add(offset as i64 as u64), l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset = offset.wrapping_add(l);
        buf = buf.add(l as usize);
        if offset as u32 == desc.len {
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
            if (desc.flags & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

unsafe fn memcpy_to_queue(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    buf: *const core::ffi::c_void,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 1)
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    if irq.is_null() {
        return;
    }
    if let Some(f) = (*irq).set_irq {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let mut addr = (*qs).used_addr.wrapping_add(2);
    let index = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, index.wrapping_add(1) as u16);

    let t = (index & (*qs).num.wrapping_sub(1)).wrapping_mul(8) as u64;
    addr = (*qs).used_addr.wrapping_add(4).wrapping_add(t);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    set_irq((*s).irq, 1);
}

unsafe fn virtio_input_queue_event(s: *mut VIRTIODevice, ty: u16, code: u16, value: u32) -> c_int {
    let queue_idx: c_int = 0;
    let qs = (*s).queue.as_mut_ptr();
    let mut buf = [0u8; 8];

    if (*qs).ready == 0 {
        return -1;
    }

    put_le16(buf.as_mut_ptr(), ty);
    put_le16(buf.as_mut_ptr().add(2), code);
    put_le32(buf.as_mut_ptr().add(4), value);
    let buf_len: c_int = 8;

    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    if (*qs).last_avail_idx == avail_idx {
        return -1;
    }
    let t = ((*qs).last_avail_idx as u32 & (*qs).num.wrapping_sub(1)).wrapping_mul(2) as u64;
    let desc_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(4).wrapping_add(t)) as c_int;

    memcpy_to_queue(
        s,
        queue_idx,
        desc_idx,
        0,
        buf.as_ptr() as *const core::ffi::c_void,
        buf_len,
    );
    virtio_consume_desc(s, queue_idx, desc_idx, buf_len);
    (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    0
}

#[no_mangle]
pub extern "C" fn virtio_input_send_key_event__part3869(s: *mut VIRTIODevice) -> c_int {
    unsafe { virtio_input_queue_event(s, 0x00, 0, 0) }
}
}

// ── `if (s1->type != VIRTIO_INPUT_TYPE_MOUSE && s1->type != VIRTIO_INPUT_TYPE_TABLET)` · virtio.c:1470-1472 · chunk 3871 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s1->type != VIRTIO_INPUT_TYPE_MOUSE &&
//             s1->type != VIRTIO_INPUT_TYPE_TABLET)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3871 {
use core::ffi::c_int;

// VirtioInputTypeEnum values from the translation unit:
// VIRTIO_INPUT_TYPE_KEYBOARD = 0, VIRTIO_INPUT_TYPE_MOUSE = 1, VIRTIO_INPUT_TYPE_TABLET = 2
const VIRTIO_INPUT_TYPE_MOUSE: c_int = 1;
const VIRTIO_INPUT_TYPE_TABLET: c_int = 2;

#[no_mangle]
pub extern "C" fn virtio_input_send_mouse_event__part3871(input_type: c_int) -> c_int {
    if input_type != VIRTIO_INPUT_TYPE_MOUSE && input_type != VIRTIO_INPUT_TYPE_TABLET {
        return -1;
    }
    0
}
}

// ── `if (s1->type == VIRTIO_INPUT_TYPE_MOUSE) … else` · virtio.c:1473-1487 · chunk 3872 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s1->type == VIRTIO_INPUT_TYPE_MOUSE) {
//             ret = virtio_input_queue_event(s, 0x02, 0x00, dx);
//             if (ret != 0)
//                 return ret;
//             ret = virtio_input_queue_event(s, 0x02, 0x01, dy);
//             if (ret != 0)
//                 return ret;
//         } else {
//             ret = virtio_input_queue_event(s, 0x03, 0x00, dx);
//             if (ret != 0)
//                 return ret;
//             ret = virtio_input_queue_event(s, 0x03, 0x01, dy);
//             if (ret != 0)
//                 return ret;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3872 {
// Port of virtio_input_send_mouse_event lines 1473-1487 (the mouse/tablet
// relative-motion event pair) plus the private (static) helpers it needs from
// the same translation unit: virtio_input_queue_event, memcpy_to_queue,
// memcpy_to_from_queue, get_desc, virtio_memcpy_to_ram/_from_ram,
// virtio_read16/write16/write32, virtio_consume_desc, set_irq, put_le16/32.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

pub type VirtioPhysAddr = u64;

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type GetRAMPtrFunc = unsafe extern "C" fn(*mut VIRTIODevice, VirtioPhysAddr, c_int) -> *mut u8;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: Option<GetRAMPtrFunc>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIOInputDevice {
    pub common: VIRTIODevice,
    pub r#type: c_int,
    pub buttons_state: u32,
}

const VIRTIO_INPUT_TYPE_MOUSE: c_int = 1;

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn put_le16(ptr: *mut u8, v: u16) {
    unsafe {
        *ptr = v as u8;
        *ptr.add(1) = (v >> 8) as u8;
    }
}

unsafe fn put_le32(ptr: *mut u8, v: u32) {
    unsafe {
        *ptr = v as u8;
        *ptr.add(1) = (v >> 8) as u8;
        *ptr.add(2) = (v >> 16) as u8;
        *ptr.add(3) = (v >> 24) as u8;
    }
}

unsafe fn virtio_get_ram_ptr(s: *mut VIRTIODevice, addr: VirtioPhysAddr, is_rw: c_int) -> *mut u8 {
    unsafe {
        match (*s).get_ram_ptr {
            Some(f) => f(s, addr, is_rw),
            None => core::ptr::null_mut(),
        }
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: VirtioPhysAddr) -> u16 {
    unsafe {
        if addr & 1 != 0 {
            return 0;
        }
        let ptr = virtio_get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return 0;
        }
        core::ptr::read_unaligned(ptr as *const u16)
    }
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: VirtioPhysAddr, val: u16) {
    unsafe {
        if addr & 1 != 0 {
            return;
        }
        let ptr = virtio_get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return;
        }
        core::ptr::write_unaligned(ptr as *mut u16, val);
    }
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: VirtioPhysAddr, val: u32) {
    unsafe {
        if addr & 3 != 0 {
            return;
        }
        let ptr = virtio_get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return;
        }
        core::ptr::write_unaligned(ptr as *mut u32, val);
    }
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    mut addr: VirtioPhysAddr,
    mut count: c_int,
) -> c_int {
    unsafe {
        while count > 0 {
            let l = min_int(count, (4096u64 - (addr & 4095)) as c_int);
            let ptr = virtio_get_ram_ptr(s, addr, 0);
            if ptr.is_null() {
                return -1;
            }
            core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
            addr = addr.wrapping_add(l as u64);
            buf = buf.wrapping_add(l as usize);
            count -= l;
        }
        0
    }
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: VirtioPhysAddr,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    unsafe {
        while count > 0 {
            let l = min_int(count, (4096u64 - (addr & 4095)) as c_int);
            let ptr = virtio_get_ram_ptr(s, addr, 1);
            if ptr.is_null() {
                return -1;
            }
            core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
            addr = addr.wrapping_add(l as u64);
            buf = buf.wrapping_add(l as usize);
            count -= l;
        }
        0
    }
}

unsafe fn get_desc(
    s: *mut VIRTIODevice,
    desc: *mut VIRTIODesc,
    queue_idx: c_int,
    desc_idx: c_int,
) -> c_int {
    unsafe {
        let qs = (*s).queue.as_ptr().add(queue_idx as usize);
        let off = (desc_idx as u32 as u64).wrapping_mul(16);
        virtio_memcpy_from_ram(
            s,
            desc as *mut u8,
            (*qs).desc_addr.wrapping_add(off),
            16,
        )
    }
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: c_int,
) -> c_int {
    unsafe {
        let mut desc: VIRTIODesc = core::mem::zeroed();
        let descp: *mut VIRTIODesc = &mut desc;
        let f_write_flag: c_int;

        if count == 0 {
            return 0;
        }

        get_desc(s, descp, queue_idx, desc_idx);

        if to_queue != 0 {
            f_write_flag = 2;
            loop {
                if (desc.flags & 2) as c_int == f_write_flag {
                    break;
                }
                if desc.flags & 1 == 0 {
                    return -1;
                }
                desc_idx = desc.next as c_int;
                get_desc(s, descp, queue_idx, desc_idx);
            }
        } else {
            f_write_flag = 0;
        }

        loop {
            if (desc.flags & 2) as c_int != f_write_flag {
                return -1;
            }
            if (offset as u32) < desc.len {
                break;
            }
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            offset = (offset as u32).wrapping_sub(desc.len) as c_int;
            get_desc(s, descp, queue_idx, desc_idx);
        }

        loop {
            let l = min_int(count, desc.len.wrapping_sub(offset as u32) as c_int);
            let daddr = desc.addr.wrapping_add(offset as u64);
            if to_queue != 0 {
                virtio_memcpy_to_ram(s, daddr, buf as *const u8, l);
            } else {
                virtio_memcpy_from_ram(s, buf, daddr, l);
            }
            count -= l;
            if count == 0 {
                break;
            }
            offset = offset.wrapping_add(l);
            buf = buf.wrapping_offset(l as isize);
            if (offset as u32) == desc.len {
                if desc.flags & 1 == 0 {
                    return -1;
                }
                desc_idx = desc.next as c_int;
                get_desc(s, descp, queue_idx, desc_idx);
                if (desc.flags & 2) as c_int != f_write_flag {
                    return -1;
                }
                offset = 0;
            }
        }
        0
    }
}

unsafe fn memcpy_to_queue(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    buf: *const c_void,
    count: c_int,
) -> c_int {
    unsafe { memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 1) }
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    unsafe {
        if irq.is_null() {
            return;
        }
        if let Some(f) = (*irq).set_irq {
            f((*irq).opaque, (*irq).irq_num, level);
        }
    }
}

unsafe fn virtio_consume_desc(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    desc_len: c_int,
) {
    unsafe {
        let qs = (*s).queue.as_mut_ptr().add(queue_idx as usize);
        let mut addr = (*qs).used_addr.wrapping_add(2);
        let index = virtio_read16(s, addr) as u32;
        virtio_write16(s, addr, index.wrapping_add(1) as u16);

        let off = (index & (*qs).num.wrapping_sub(1)).wrapping_mul(8) as u64;
        addr = (*qs).used_addr.wrapping_add(4).wrapping_add(off);
        virtio_write32(s, addr, desc_idx as u32);
        virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

        (*s).int_status |= 1;
        set_irq((*s).irq, 1);
    }
}

unsafe fn virtio_input_queue_event(
    s: *mut VIRTIODevice,
    ev_type: u16,
    code: u16,
    value: u32,
) -> c_int {
    unsafe {
        let qs: *mut QueueState = (*s).queue.as_mut_ptr();
        let mut buf = [0u8; 8];

        if (*qs).ready == 0 {
            return -1;
        }

        put_le16(buf.as_mut_ptr(), ev_type);
        put_le16(buf.as_mut_ptr().add(2), code);
        put_le32(buf.as_mut_ptr().add(4), value);
        let buf_len: c_int = 8;

        let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
        if (*qs).last_avail_idx == avail_idx {
            return -1;
        }
        let slot: u32 = (*qs).last_avail_idx as u32 & (*qs).num.wrapping_sub(1);
        let desc_idx: c_int = virtio_read16(
            s,
            (*qs)
                .avail_addr
                .wrapping_add(4)
                .wrapping_add(slot.wrapping_mul(2) as u64),
        ) as c_int;

        memcpy_to_queue(s, 0, desc_idx, 0, buf.as_ptr() as *const c_void, buf_len);
        virtio_consume_desc(s, 0, desc_idx, buf_len);
        (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
        0
    }
}

#[no_mangle]
pub extern "C" fn virtio_input_send_mouse_event__part3872(
    s: *mut VIRTIODevice,
    dx: c_int,
    dy: c_int,
) -> c_int {
    unsafe {
        let s1 = s as *mut VIRTIOInputDevice;
        let mut ret: c_int;

        if (*s1).r#type == VIRTIO_INPUT_TYPE_MOUSE {
            ret = virtio_input_queue_event(s, 0x02, 0x00, dx as u32);
            if ret != 0 {
                return ret;
            }
            ret = virtio_input_queue_event(s, 0x02, 0x01, dy as u32);
            if ret != 0 {
                return ret;
            }
        } else {
            ret = virtio_input_queue_event(s, 0x03, 0x00, dx as u32);
            if ret != 0 {
                return ret;
            }
            ret = virtio_input_queue_event(s, 0x03, 0x01, dy as u32);
            if ret != 0 {
                return ret;
            }
        }

        ret
    }
}
}

// ── `if (dz != 0)` · virtio.c:1488-1492 · chunk 3873 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (dz != 0) {
//             ret = virtio_input_queue_event(s, 0x02, 0x08, dz);
//             if (ret != 0)
//                 return ret;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3873 {
// Port of the `if (dz != 0) { ... }` piece of virtio_input_send_mouse_event,
// together with the (static) helpers it needs, which are private Rust fns.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_int, c_void};

type BOOL = c_int;
type virtio_phys_addr_t = u64;

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: virtio_phys_addr_t,
    pub avail_addr: virtio_phys_addr_t,
    pub used_addr: virtio_phys_addr_t,
    pub manual_recv: BOOL,
}

#[repr(C)]
pub struct VIRTIODesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut IRQSignal,
    pub get_ram_ptr: Option<unsafe extern "C" fn(*mut VIRTIODevice, virtio_phys_addr_t, BOOL) -> *mut u8>,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

unsafe fn get_ram_ptr(s: *mut VIRTIODevice, paddr: virtio_phys_addr_t, is_rw: BOOL) -> *mut u8 {
    match (*s).get_ram_ptr {
        Some(f) => f(s, paddr, is_rw),
        None => core::ptr::null_mut(),
    }
}

unsafe fn virtio_read16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t) -> u16 {
    if addr & 1 != 0 {
        return 0;
    }
    let ptr = get_ram_ptr(s, addr, 0);
    if ptr.is_null() {
        return 0;
    }
    core::ptr::read_unaligned(ptr as *const u16)
}

unsafe fn virtio_write16(s: *mut VIRTIODevice, addr: virtio_phys_addr_t, val: u16) {
    if addr & 1 != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u16, val);
}

unsafe fn virtio_write32(s: *mut VIRTIODevice, addr: virtio_phys_addr_t, val: u32) {
    if addr & 3 != 0 {
        return;
    }
    let ptr = get_ram_ptr(s, addr, 1);
    if ptr.is_null() {
        return;
    }
    core::ptr::write_unaligned(ptr as *mut u32, val);
}

unsafe fn virtio_memcpy_from_ram(
    s: *mut VIRTIODevice,
    buf: *mut u8,
    mut addr: virtio_phys_addr_t,
    mut count: c_int,
) -> c_int {
    let mut buf = buf;
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let ptr = get_ram_ptr(s, addr, 0);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(ptr, buf, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn virtio_memcpy_to_ram(
    s: *mut VIRTIODevice,
    mut addr: virtio_phys_addr_t,
    mut buf: *const u8,
    mut count: c_int,
) -> c_int {
    while count > 0 {
        let l = min_int(count, (4096u64.wrapping_sub(addr & (4096 - 1))) as c_int);
        let ptr = get_ram_ptr(s, addr, 1);
        if ptr.is_null() {
            return -1;
        }
        core::ptr::copy_nonoverlapping(buf, ptr, l as usize);
        addr = addr.wrapping_add(l as u64);
        buf = buf.offset(l as isize);
        count -= l;
    }
    0
}

unsafe fn get_desc(s: *mut VIRTIODevice, desc: *mut VIRTIODesc, queue_idx: c_int, desc_idx: c_int) -> c_int {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    virtio_memcpy_from_ram(
        s,
        desc as *mut u8,
        (*qs)
            .desc_addr
            .wrapping_add((desc_idx as u64).wrapping_mul(core::mem::size_of::<VIRTIODesc>() as u64)),
        core::mem::size_of::<VIRTIODesc>() as c_int,
    )
}

unsafe fn memcpy_to_from_queue(
    s: *mut VIRTIODevice,
    mut buf: *mut u8,
    queue_idx: c_int,
    mut desc_idx: c_int,
    mut offset: c_int,
    mut count: c_int,
    to_queue: BOOL,
) -> c_int {
    let mut desc: VIRTIODesc = core::mem::zeroed();
    let f_write_flag: c_int;

    if count == 0 {
        return 0;
    }

    get_desc(s, &mut desc, queue_idx, desc_idx);

    if to_queue != 0 {
        f_write_flag = 2;
        loop {
            if (desc.flags as c_int & 2) == f_write_flag {
                break;
            }
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
        }
    } else {
        f_write_flag = 0;
    }

    loop {
        if (desc.flags as c_int & 2) != f_write_flag {
            return -1;
        }
        if offset < desc.len as c_int {
            break;
        }
        if desc.flags & 1 == 0 {
            return -1;
        }
        desc_idx = desc.next as c_int;
        offset -= desc.len as c_int;
        get_desc(s, &mut desc, queue_idx, desc_idx);
    }

    loop {
        let l = min_int(count, (desc.len as c_int) - offset);
        let at = desc.addr.wrapping_add(offset as i64 as u64);
        if to_queue != 0 {
            virtio_memcpy_to_ram(s, at, buf, l);
        } else {
            virtio_memcpy_from_ram(s, buf, at, l);
        }
        count -= l;
        if count == 0 {
            break;
        }
        offset += l;
        buf = buf.offset(l as isize);
        if offset == desc.len as c_int {
            if desc.flags & 1 == 0 {
                return -1;
            }
            desc_idx = desc.next as c_int;
            get_desc(s, &mut desc, queue_idx, desc_idx);
            if (desc.flags as c_int & 2) != f_write_flag {
                return -1;
            }
            offset = 0;
        }
    }
    0
}

unsafe fn memcpy_to_queue(
    s: *mut VIRTIODevice,
    queue_idx: c_int,
    desc_idx: c_int,
    offset: c_int,
    buf: *const c_void,
    count: c_int,
) -> c_int {
    memcpy_to_from_queue(s, buf as *mut u8, queue_idx, desc_idx, offset, count, 1)
}

unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    match (*irq).set_irq {
        Some(f) => f((*irq).opaque, (*irq).irq_num, level),
        None => {}
    }
}

unsafe fn virtio_consume_desc(s: *mut VIRTIODevice, queue_idx: c_int, desc_idx: c_int, desc_len: c_int) {
    let qs: *mut QueueState = (*s).queue.as_mut_ptr().add(queue_idx as usize);
    let mut addr = (*qs).used_addr.wrapping_add(2);
    let index: u32 = virtio_read16(s, addr) as u32;
    virtio_write16(s, addr, index.wrapping_add(1) as u16);

    addr = (*qs)
        .used_addr
        .wrapping_add(4)
        .wrapping_add((index & (*qs).num.wrapping_sub(1)) as u64 * 8);
    virtio_write32(s, addr, desc_idx as u32);
    virtio_write32(s, addr.wrapping_add(4), desc_len as u32);

    (*s).int_status |= 1;
    set_irq((*s).irq, 1);
}

unsafe fn virtio_input_queue_event(s: *mut VIRTIODevice, ty: u16, code: u16, value: u32) -> c_int {
    let queue_idx: c_int = 0;
    let qs: *mut QueueState = (*s).queue.as_mut_ptr();

    if (*qs).ready == 0 {
        return -1;
    }

    let mut buf = [0u8; 8];
    put_le16(buf.as_mut_ptr(), ty);
    put_le16(buf.as_mut_ptr().add(2), code);
    put_le32(buf.as_mut_ptr().add(4), value);
    let buf_len: c_int = 8;

    let avail_idx = virtio_read16(s, (*qs).avail_addr.wrapping_add(2));
    if (*qs).last_avail_idx == avail_idx {
        return -1;
    }
    let slot = ((*qs).last_avail_idx as u32) & (*qs).num.wrapping_sub(1);
    let desc_idx = virtio_read16(
        s,
        (*qs)
            .avail_addr
            .wrapping_add(4)
            .wrapping_add(slot as u64 * 2),
    ) as c_int;

    memcpy_to_queue(s, queue_idx, desc_idx, 0, buf.as_mut_ptr() as *const c_void, buf_len);
    virtio_consume_desc(s, queue_idx, desc_idx, buf_len);
    (*qs).last_avail_idx = (*qs).last_avail_idx.wrapping_add(1);
    0
}

#[no_mangle]
pub extern "C" fn virtio_input_send_mouse_event__part3873(s: *mut VIRTIODevice, dz: c_int) -> c_int {
    unsafe {
        if dz != 0 {
            let ret = virtio_input_queue_event(s, 0x02, 0x08, dz as u32);
            if ret != 0 {
                return ret;
            }
        }
        0
    }
}
}

// ── `tab[k >> 3] |= 1 << (k & 7);` · virtio.c:1513-1513 · chunk 3877 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     tab[k >> 3] |= 1 << (k & 7);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3877 {
// Ported piece: single statement of static void set_bit(uint8_t *tab, int k)
//   tab[k >> 3] |= 1 << (k & 7);
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn set_bit__part3877(tab: *mut u8, k: c_int) {
    unsafe {
        if tab.is_null() {
            // C would fault here too; do not touch memory, just return.
            return;
        }
        let idx = (k >> 3) as isize;
        let p = tab.offset(idx);
        let cur = *p;
        let bit = (1i32 << (k & 7)) as u8;
        *p = cur | bit;
    }
}
}

// ── `switch (config[0])` · virtio.c:1523-1629 · chunk 3879 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     switch(config[0]) {
//         case VIRTIO_INPUT_CFG_UNSET:
//             break;
//         case VIRTIO_INPUT_CFG_ID_NAME:
//             {
//                 const char *name;
//                 int len;
//                 switch(s1->type) {
//                 case VIRTIO_INPUT_TYPE_KEYBOARD:
//                     name = "virtio_keyboard";
//                     break;
//                 case VIRTIO_INPUT_TYPE_MOUSE:
//                     name = "virtio_mouse";
//                     break;
//                 case VIRTIO_INPUT_TYPE_TABLET:
//                     name = "virtio_tablet";
//                     break;
//                 default:
//                     abort();
//                 }
//                 len = strlen(name);
//                 config[2] = len;
//                 memcpy(config + 8, name, len);
//             }
//             break;
//         default:
//         case VIRTIO_INPUT_CFG_ID_SERIAL:
//         case VIRTIO_INPUT_CFG_ID_DEVIDS:
//         case VIRTIO_INPUT_CFG_PROP_BITS:
//             config[2] = 0;
//             break;
//         case VIRTIO_INPUT_CFG_EV_BITS:
//             config[2] = 0;
//             switch(s1->type) {
//             case VIRTIO_INPUT_TYPE_KEYBOARD:
//                 switch(config[1]) {
//                 case 0x01:
//                     config[2] = 128 / 8;
//                     memset(config + 8, 0xff, 128 / 8);
//                     break;
//                 case 0x14:
//                     config[2] = 1;
//                     break;
//                 default:
//                     break;
//                 }
//                 break;
//             case VIRTIO_INPUT_TYPE_MOUSE:
//                 switch(config[1]) {
//                 case 0x01:
//                     config[2] = 512 / 8;
//                     memset(config + 8, 0, 512 / 8);
//                     for(i = 0; i < (sizeof(buttons_list) / sizeof(buttons_list[0])); i++)
//                         set_bit(config + 8, buttons_list[i]);
//                     break;
//                 case 0x02:
//                     config[2] = 2;
//                     config[8] = 0;
//                     config[9] = 0;
//                     set_bit(config + 8, 0x00);
//                     set_bit(config + 8, 0x01);
//                     set_bit(config + 8, 0x08);
//                     break;
//                 default:
//                     break;
//                 }
//                 break;
//             case VIRTIO_INPUT_TYPE_TABLET:
//                 switch(config[1]) {
//                 case 0x01:
//                     config[2] = 512 / 8;
//                     memset(config + 8, 0, 512 / 8);
//                     for(i = 0; i < (sizeof(buttons_list) / sizeof(buttons_list[0])); i++)
//                         set_bit(config + 8, buttons_list[i]);
//                     break;
//                 case 0x02:
//                     config[2] = 2;
//                     config[8] = 0;
//                     config[9] = 0;
//                     set_bit(config + 8, 0x08);
//                     break;
//                 case 0x03:
//                     config[2] = 1;
//                     config[8] = 0;
//                     set_bit(config + 8, 0x00);
//                     set_bit(config + 8, 0x01);
//                     break;
//                 default:
//                     break;
//                 }
//                 break;
//             default:
//                 abort();
//             }
//             break;
//         case VIRTIO_INPUT_CFG_ABS_INFO:
//             if (s1->type == VIRTIO_INPUT_TYPE_TABLET && config[1] <= 1) {
//     
//                 config[2] = 5 * 4;
//                 put_le32(config + 8, 0);
//                 put_le32(config + 12, 32768 - 1) ;
//                 put_le32(config + 16, 0);
//                 put_le32(config + 20, 0);
//                 put_le32(config + 24, 0);
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3879 {
use core::ffi::{c_char, c_int};

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memset(dst: *mut u8, c: c_int, n: usize) -> *mut u8;
    fn abort() -> !;
}

const VIRTIO_INPUT_CFG_ID_NAME: u8 = 0x01;
const VIRTIO_INPUT_CFG_EV_BITS: u8 = 0x11;
const VIRTIO_INPUT_CFG_ABS_INFO: u8 = 0x12;

const VIRTIO_INPUT_TYPE_KEYBOARD: c_int = 0;
const VIRTIO_INPUT_TYPE_MOUSE: c_int = 1;
const VIRTIO_INPUT_TYPE_TABLET: c_int = 2;

static BUTTONS_LIST: [u16; 3] = [0x110, 0x111, 0x112];

const KEY_BITS_LEN: u8 = 16;
const ABS_BITS_LEN: u8 = 64;

#[inline]
unsafe fn set_bit(tab: *mut u8, k: c_int) {
    let idx = (k >> 3) as isize;
    *tab.offset(idx) |= 1u8 << ((k & 7) as u32);
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr.offset(0) = v as u8;
    *ptr.offset(1) = (v >> 8) as u8;
    *ptr.offset(2) = (v >> 16) as u8;
    *ptr.offset(3) = (v >> 24) as u8;
}

#[no_mangle]
pub extern "C" fn virtio_input_config_write__part3879(config: *mut u8, s1_type: c_int) {
    unsafe {
        match *config {
            0x00 => {}
            VIRTIO_INPUT_CFG_ID_NAME => {
                let name: *const c_char = match s1_type {
                    VIRTIO_INPUT_TYPE_KEYBOARD => b"virtio_keyboard\0".as_ptr() as *const c_char,
                    VIRTIO_INPUT_TYPE_MOUSE => b"virtio_mouse\0".as_ptr() as *const c_char,
                    VIRTIO_INPUT_TYPE_TABLET => b"virtio_tablet\0".as_ptr() as *const c_char,
                    _ => abort(),
                };
                let len = strlen(name);
                *config.offset(2) = len as u8;
                memcpy(config.offset(8), name as *const u8, len);
            }
            VIRTIO_INPUT_CFG_EV_BITS => {
                *config.offset(2) = 0;
                let sel = *config.offset(1);
                match s1_type {
                    VIRTIO_INPUT_TYPE_KEYBOARD => match sel {
                        0x01 => {
                            *config.offset(2) = KEY_BITS_LEN;
                            memset(config.offset(8), 0xff, KEY_BITS_LEN as usize);
                        }
                        0x14 => {
                            *config.offset(2) = 1;
                        }
                        _ => {}
                    },
                    VIRTIO_INPUT_TYPE_MOUSE => match sel {
                        0x01 => {
                            *config.offset(2) = ABS_BITS_LEN;
                            memset(config.offset(8), 0, ABS_BITS_LEN as usize);
                            for b in BUTTONS_LIST.iter() {
                                set_bit(config.offset(8), *b as c_int);
                            }
                        }
                        0x02 => {
                            *config.offset(2) = 2;
                            *config.offset(8) = 0;
                            *config.offset(9) = 0;
                            set_bit(config.offset(8), 0x00);
                            set_bit(config.offset(8), 0x01);
                            set_bit(config.offset(8), 0x08);
                        }
                        _ => {}
                    },
                    VIRTIO_INPUT_TYPE_TABLET => match sel {
                        0x01 => {
                            *config.offset(2) = ABS_BITS_LEN;
                            memset(config.offset(8), 0, ABS_BITS_LEN as usize);
                            for b in BUTTONS_LIST.iter() {
                                set_bit(config.offset(8), *b as c_int);
                            }
                        }
                        0x02 => {
                            *config.offset(2) = 2;
                            *config.offset(8) = 0;
                            *config.offset(9) = 0;
                            set_bit(config.offset(8), 0x08);
                        }
                        0x03 => {
                            *config.offset(2) = 1;
                            *config.offset(8) = 0;
                            set_bit(config.offset(8), 0x00);
                            set_bit(config.offset(8), 0x01);
                        }
                        _ => {}
                    },
                    _ => abort(),
                }
            }
            VIRTIO_INPUT_CFG_ABS_INFO => {
                if s1_type == VIRTIO_INPUT_TYPE_TABLET && *config.offset(1) <= 1 {
                    *config.offset(2) = 5 * 4;
                    put_le32(config.offset(8), 0);
                    put_le32(config.offset(12), 32768 - 1);
                    put_le32(config.offset(16), 0);
                    put_le32(config.offset(20), 0);
                    put_le32(config.offset(24), 0);
                }
            }
            _ => {
                *config.offset(2) = 0;
            }
        }
    }
}
}

// ── `for (el = (&s->fid_list)->next; el != (&s->fid_list; el = el->next)` · virtio.c:1668-1672 · chunk 3887 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(el = (&s->fid_list)->next; el != (&s->fid_list); el = el->next) {
//             f = ((FIDDesc *)((uint8_t *)(el) - ((size_t) &((FIDDesc *)0)->link)));
//             if (f->fid == fid)
//                 return f;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3887 {
// Port of the fid lookup loop inside fid_find1 (virtio.c).
// The C loop walks s->fid_list looking for the FIDDesc whose fid matches,
// leaving `el` at the end of the list when nothing is found.

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn fid_find1__part3887(
    head: *mut ListHead,
    el_out: *mut *mut ListHead,
    fid: u32,
) -> *mut FIDDesc {
    unsafe {
        let mut el: *mut ListHead = (*head).next;
        while el != head {
            /* &((FIDDesc *)0)->link is the first member, so its offset is 0 */
            let f: *mut FIDDesc = el as *mut FIDDesc;
            if (*f).fid == fid {
                *el_out = el;
                return f;
            }
            el = (*el).next;
        }
        *el_out = el;
        core::ptr::null_mut()
    }
}
}

// ── `return ((void *)0) ;` · virtio.c:1673-1673 · chunk 3888 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 
//               ((void *)0)
//                   ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3888 {
// Ported piece of fid_find1(): the fall-through "not found" return.
// The C caller keeps the `return` statement and forwards our value.

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut core::ffi::c_void,
}

#[no_mangle]
pub extern "C" fn fid_find1__part3888() -> *mut FIDDesc {
    core::ptr::null_mut()
}
}

// ── `FIDDesc *f; f = fid_find1(s, fid);` · virtio.c:1678-1680 · chunk 3890 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FIDDesc *f;
//     
//         f = fid_find1(s, fid);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3890 {
// Port of the `f = fid_find1(s, fid);` statement from fid_find(), plus the
// static helper fid_find1 (no linker symbol, so its logic lives here).

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

// typedef struct { struct list_head link; uint32_t fid; FSFile *fd; } FIDDesc;
#[repr(C)]
pub struct FIDDesc {
    pub link: list_head,
    pub fid: u32,
    pub fd: *mut c_void,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIO9PDevice {
    pub common: VIRTIODevice,
    pub fs: *mut c_void,
    pub msize: c_int,
    pub fid_list: list_head,
    pub req_in_progress: c_int,
}

// static FIDDesc *fid_find1(VIRTIO9PDevice *s, uint32_t fid)
unsafe fn fid_find1(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    let head: *mut list_head = core::ptr::addr_of_mut!((*s).fid_list);
    let mut el: *mut list_head = (*head).next;
    while el != head {
        // link is the first member of FIDDesc, so the container pointer is el.
        let f: *mut FIDDesc = el as *mut FIDDesc;
        if (*f).fid == fid {
            return f;
        }
        el = (*el).next;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn fid_find__part3890(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    fid_find1(s, fid)
}
}

// ── `if (!f)` · virtio.c:1681-1682 · chunk 3891 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!f)
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3891 {
// Ported piece of fid_find(): the NULL check on the FIDDesc pointer.
// Returns nonzero when `f` is NULL, in which case the caller returns NULL.

use core::ffi::{c_int, c_void};

#[no_mangle]
pub extern "C" fn fid_find__part3891(f: *const c_void) -> c_int {
    if f.is_null() {
        1
    } else {
        0
    }
}
}

// ── `return f->fd;` · virtio.c:1683-1683 · chunk 3892 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return f->fd;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3892 {
// Port of the `return f->fd;` statement of `fid_find` (virtio.c).
use core::ffi::c_void;

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

// Mirror of the C `FIDDesc` struct (fields in declaration order).
#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut c_void,
}

#[no_mangle]
pub unsafe extern "C" fn fid_find__part3892(f: *mut FIDDesc) -> *mut c_void {
    if f.is_null() {
        return core::ptr::null_mut();
    }
    (*f).fd
}
}

// ── `FIDDesc *f; f = fid_find1(s, fid);` · virtio.c:1688-1690 · chunk 3894 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     FIDDesc *f;
//     
//         f = fid_find1(s, fid);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3894 {
// Port of the fid lookup part of fid_delete() from virtio.c.
// The C helper `fid_find1` is static (no linker symbol), so its logic is
// reproduced here as a private Rust function.

use core::ffi::c_int;

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut core::ffi::c_void,
    // note: `fd` is an FSFile*; only its address is ever used here, so an
    // opaque pointer is sufficient and keeps the layout exact.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int, // BOOL
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut core::ffi::c_void,
    pub mem_range: *mut core::ffi::c_void,
    pub pci_dev: *mut core::ffi::c_void,
    pub irq: *mut core::ffi::c_void,
    pub get_ram_ptr: *mut core::ffi::c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut core::ffi::c_void,
    pub config_write: *mut core::ffi::c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct VIRTIO9PDevice {
    pub common: VIRTIODevice,
    pub fs: *mut core::ffi::c_void,
    pub msize: c_int,
    pub fid_list: ListHead,
    pub req_in_progress: c_int, // BOOL
}

/// Faithful port of the static C helper `fid_find1`:
/// walk `s->fid_list` and return the FIDDesc whose `fid` matches (the
/// container_of() on `link` is a no-op because `link` is the first member).
unsafe fn fid_find1(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    let head: *mut ListHead = core::ptr::addr_of_mut!((*s).fid_list);
    let mut el: *mut ListHead = (*head).next;
    while el != head {
        let f: *mut FIDDesc = el as *mut FIDDesc;
        if (*f).fid == fid {
            return f;
        }
        el = (*el).next;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn fid_delete__part3894(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    if s.is_null() {
        return core::ptr::null_mut();
    }
    unsafe { fid_find1(s, fid) }
}
}

// ── `if (f)` · virtio.c:1691-1695 · chunk 3895 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (f) {
//             s->fs->fs_delete(s->fs, f->fd);
//             list_del(&f->link);
//             free(f);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3895 {
// Port of the `if (f) { ... }` body of fid_delete() in virtio.c.
use core::ffi::{c_int, c_void};

pub type BOOL = c_int;
pub type VirtioPhysAddr = u64;

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut c_void,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: VirtioPhysAddr,
    pub avail_addr: VirtioPhysAddr,
    pub used_addr: VirtioPhysAddr,
    pub manual_recv: BOOL,
}

pub type VIRTIODeviceRecvFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, c_int, c_int, c_int, c_int) -> c_int>;
pub type VIRTIOGetRAMPtrFunc =
    Option<unsafe extern "C" fn(*mut VIRTIODevice, VirtioPhysAddr, BOOL) -> *mut u8>;

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: VIRTIOGetRAMPtrFunc,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: VIRTIODeviceRecvFunc,
    pub config_write: Option<unsafe extern "C" fn(*mut VIRTIODevice)>,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct FSStatFS {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
}

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
pub struct FSStat {
    pub qid: FSQID,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_nlink: u64,
    pub st_rdev: u64,
    pub st_size: u64,
    pub st_blksize: u64,
    pub st_blocks: u64,
    pub st_atime_sec: u64,
    pub st_atime_nsec: u32,
    pub st_mtime_sec: u64,
    pub st_mtime_nsec: u32,
    pub st_ctime_sec: u64,
    pub st_ctime_nsec: u32,
}

#[repr(C)]
pub struct FSLock {
    pub r#type: u8,
    pub flags: u32,
    pub start: u64,
    pub length: u64,
    pub proc_id: u32,
    pub client_id: *mut i8,
}

pub type FSOpenCompletionFunc =
    Option<unsafe extern "C" fn(*mut FSDevice, *mut FSQID, c_int, *mut c_void)>;

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(*mut FSDevice)>,
    pub fs_delete: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void)>,
    pub fs_statfs: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSStatFS)>,
    pub fs_attach: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut c_void,
            *mut FSQID,
            u32,
            *const i8,
            *const i8,
        ) -> c_int,
    >,
    pub fs_walk: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut c_void,
            *mut FSQID,
            *mut c_void,
            c_int,
            *mut *mut i8,
        ) -> c_int,
    >,
    pub fs_mkdir: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut c_void,
            *const i8,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_open: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut c_void,
            u32,
            FSOpenCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub fs_create: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut c_void,
            *const i8,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_stat: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut FSStat) -> c_int>,
    pub fs_setattr: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut c_void,
            u32,
            u32,
            u32,
            u32,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> c_int,
    >,
    pub fs_close: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void)>,
    pub fs_readdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_read: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_write: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, u64, *const u8, c_int) -> c_int,
    >,
    pub fs_link: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut c_void, *const i8) -> c_int,
    >,
    pub fs_symlink: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSQID, *mut c_void, *const i8, *const i8, u32)
            -> c_int,
    >,
    pub fs_mknod: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut c_void,
            *const i8,
            u32,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_readlink: Option<unsafe extern "C" fn(*mut FSDevice, *mut i8, c_int, *mut c_void) -> c_int>,
    pub fs_renameat: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const i8, *mut c_void, *const i8) -> c_int,
    >,
    pub fs_unlinkat: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const i8) -> c_int>,
    pub fs_lock: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void, *const FSLock) -> c_int>,
    pub fs_getlock: Option<unsafe extern "C" fn(*mut FSDevice, *mut c_void, *mut FSLock) -> c_int>,
}

#[repr(C)]
pub struct VIRTIO9PDevice {
    pub common: VIRTIODevice,
    pub fs: *mut FSDevice,
    pub msize: c_int,
    pub fid_list: ListHead,
    pub req_in_progress: BOOL,
}

extern "C" {
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fid_delete__part3895(s: *mut VIRTIO9PDevice, f: *mut FIDDesc) {
    if f.is_null() {
        return;
    }

    let fs = (*s).fs;
    if !fs.is_null() {
        let del = (*fs).fs_delete;
        if let Some(del) = del {
            del(fs, (*f).fd);
        }
    }

    // list_del(&f->link)
    let el: *mut ListHead = core::ptr::addr_of_mut!((*f).link);
    let prev = (*el).prev;
    let next = (*el).next;
    (*prev).next = next;
    (*next).prev = prev;
    (*el).prev = core::ptr::null_mut();
    (*el).next = core::ptr::null_mut();

    free(f as *mut c_void);
}
}

// ── `FIDDesc *f; f = fid_find1(s, fid);` · virtio.c:1700-1702 · chunk 3897 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     FIDDesc *f;
//     
//         f = fid_find1(s, fid);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3897 {
// Rust port of the lookup piece of fid_set() (virtio.c), inlining the logic of
// the C static function fid_find1() since it has no linker symbol.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

#[repr(C)]
pub struct QueueState {
    pub ready: u32,
    pub num: u32,
    pub last_avail_idx: u16,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub manual_recv: c_int,
}

#[repr(C)]
pub struct VIRTIODevice {
    pub mem_map: *mut c_void,
    pub mem_range: *mut c_void,
    pub pci_dev: *mut c_void,
    pub irq: *mut c_void,
    pub get_ram_ptr: *mut c_void,
    pub debug: c_int,
    pub int_status: u32,
    pub status: u32,
    pub device_features_sel: u32,
    pub queue_sel: u32,
    pub queue: [QueueState; 8],
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_features: u32,
    pub device_recv: *mut c_void,
    pub config_write: *mut c_void,
    pub config_space_size: u32,
    pub config_space: [u8; 256],
}

#[repr(C)]
pub struct FIDDesc {
    pub link: ListHead,
    pub fid: u32,
    pub fd: *mut c_void,
}

#[repr(C)]
pub struct VIRTIO9PDevice {
    pub common: VIRTIODevice,
    pub fs: *mut c_void,
    pub msize: c_int,
    pub fid_list: ListHead,
    pub req_in_progress: c_int,
}

// Equivalent of list_for_each() + list_entry(el, FIDDesc, link).
fn fid_find1(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    unsafe {
        if s.is_null() {
            return core::ptr::null_mut();
        }
        let head: *mut ListHead = core::ptr::addr_of_mut!((*s).fid_list);
        let mut el: *mut ListHead = (*head).next;
        while el != head {
            // `link` is the first member of FIDDesc, so list_entry() subtracts 0.
            let f = el as *mut FIDDesc;
            if !f.is_null() && (*f).fid == fid {
                return f;
            }
            el = (*el).next;
        }
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn fid_set__part3897(s: *mut VIRTIO9PDevice, fid: u32) -> *mut FIDDesc {
    fid_find1(s, fid)
}
}

// ── `if (f) … else` · virtio.c:1703-1711 · chunk 3898 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (f) {
//             s->fs->fs_delete(s->fs, f->fd);
//             f->fd = fd;
//         } else {
//             f = malloc(sizeof(*f));
//             f->fid = fid;
//             f->fd = fd;
//             list_add(&f->link, &s->fid_list);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3898 {
// Port of the if/else tail of fid_set() from virtio.c.
// The C caller passes s->fs, &s->fid_list, fid, fd and the current FIDDesc*
// (NULL when no descriptor exists); we return the FIDDesc* in use.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

// Opaque types only ever used through pointers.
pub enum FSFile {}
pub enum FSStat {}
pub enum FSLock {}
pub enum FSOpenCompletionFunc {}

#[repr(C)]
pub struct FSStatFS {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
}

#[repr(C)]
pub struct FSQID {
    pub r#type: u8,
    pub version: u32,
    pub path: u64,
}

#[repr(C)]
pub struct FSLockStruct {
    pub r#type: u8,
    pub flags: u32,
    pub start: u64,
    pub length: u64,
    pub proc_id: u32,
    pub client_id: *mut c_char,
}

#[repr(C)]
pub struct FSDevice {
    pub fs_end: Option<unsafe extern "C" fn(*mut FSDevice)>,
    pub fs_delete: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_statfs: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSStatFS)>,
    pub fs_attach: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            u32,
            *const c_char,
            *const c_char,
        ) -> c_int,
    >,
    pub fs_walk: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut *mut FSFile,
            *mut FSQID,
            *mut FSFile,
            c_int,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub fs_mkdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSQID, *mut FSFile, *const c_char, u32, u32) -> c_int,
    >,
    pub fs_open: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            u32,
            *mut FSOpenCompletionFunc,
            *mut c_void,
        ) -> c_int,
    >,
    pub fs_create: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_stat: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSStat) -> c_int>,
    pub fs_setattr: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSFile,
            u32,
            u32,
            u32,
            u32,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> c_int,
    >,
    pub fs_close: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile)>,
    pub fs_readdir: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_read: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *mut u8, c_int) -> c_int,
    >,
    pub fs_write: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, u64, *const u8, c_int) -> c_int,
    >,
    pub fs_link: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSFile, *const c_char) -> c_int,
    >,
    pub fs_symlink: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            *const c_char,
            u32,
        ) -> c_int,
    >,
    pub fs_mknod: Option<
        unsafe extern "C" fn(
            *mut FSDevice,
            *mut FSQID,
            *mut FSFile,
            *const c_char,
            u32,
            u32,
            u32,
            u32,
        ) -> c_int,
    >,
    pub fs_readlink: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut c_char, c_int, *mut FSFile) -> c_int,
    >,
    pub fs_renameat: Option<
        unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const c_char, *mut FSFile, *const c_char) -> c_int,
    >,
    pub fs_unlinkat: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const c_char) -> c_int>,
    pub fs_lock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *const FSLockStruct) -> c_int>,
    pub fs_getlock: Option<unsafe extern "C" fn(*mut FSDevice, *mut FSFile, *mut FSLockStruct) -> c_int>,
}

#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}

#[repr(C)]
pub struct FIDDesc {
    pub link: list_head,
    pub fid: u32,
    pub fd: *mut FSFile,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
}

// static inline in C: no linker symbol, so the logic lives here.
// list_add(el, head) -> __list_add(el, head, head->next)
unsafe fn list_add(el: *mut list_head, head: *mut list_head) {
    let prev = head;
    let next = (*head).next;
    (*prev).next = el;
    (*el).prev = prev;
    (*el).next = next;
    (*next).prev = el;
}

#[no_mangle]
pub extern "C" fn fid_set__part3898(
    fs: *mut FSDevice,
    fid_list: *mut list_head,
    fid: u32,
    fd: *mut FSFile,
    f: *mut FIDDesc,
) -> *mut FIDDesc {
    unsafe {
        if !f.is_null() {
            let fsm = &mut *fs;
            if let Some(fs_delete) = fsm.fs_delete {
                let old_fd = (*f).fd;
                fs_delete(fs, old_fd);
            }
            (*f).fd = fd;
            f
        } else {
            let nf = malloc(core::mem::size_of::<FIDDesc>()) as *mut FIDDesc;
            (*nf).fid = fid;
            (*nf).fd = fd;
            list_add(core::ptr::addr_of_mut!((*nf).link), fid_list);
            nf
        }
    }
}
}

// ── `free(oi);` · virtio.c:2024-2024 · chunk 3909 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     free(oi);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3909 {
// Port of the `free(oi)` statement in virtio_9p_open_reply.

use core::ffi::c_void;

extern "C" {
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn virtio_9p_open_reply__part3909(oi: *mut c_void) {
    free(oi);
}
}

// ── `if (queue_idx != 0)` · virtio.c:2054-2055 · chunk 3913 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (queue_idx != 0)
//             return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3913 {
// Port of the queue-index guard of virtio_9p_recv_request.
// Returns nonzero when the request must be dropped (queue_idx != 0).
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3913(queue_idx: c_int) -> c_int {
    if queue_idx != 0 {
        1
    } else {
        0
    }
}
}

// ── `if (s->req_in_progress)` · virtio.c:2057-2058 · chunk 3914 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->req_in_progress)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3914 {
// Ported piece of virtio_9p_recv_request: rejects a new request while one is
// already in progress. Returns -1 (so the C caller returns -1) when
// req_in_progress is nonzero, otherwise 0 (C caller continues).
#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3914(req_in_progress: core::ffi::c_int) -> core::ffi::c_int {
    if req_in_progress != 0 {
        -1
    } else {
        0
    }
}
}

// ── `offset = 0; header_len = 4 + 1 + 2;` · virtio.c:2060-2061 · chunk 3915 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     offset = 0;
//         header_len = 4 + 1 + 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3915 {
// Rust rewrite of the TARGET:
//     offset = 0;
//     header_len = 4 + 1 + 2;
// Both variables are `int` locals of the enclosing C function, so they are
// passed by address.

#[no_mangle]
pub unsafe extern "C" fn virtio_9p_recv_request__part3915(
    offset: *mut core::ffi::c_int,
    header_len: *mut core::ffi::c_int,
) {
    if !offset.is_null() {
        *offset = 0;
    }
    if !header_len.is_null() {
        *header_len = 7;
    }
}
}

// ── `id = buf[4]; tag = get_le16(buf + 5); offset += header_len;` · virtio.c:2067-2069 · chunk 3917 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     id = buf[4];
//         tag = get_le16(buf + 5);
//         offset += header_len;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3917 {
use core::ffi::c_int;

// Port of the target: id = buf[4]; tag = get_le16(buf + 5); offset += header_len;
// get_le16 is a static inline helper, so its logic is reproduced here.
#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3917(
    buf: *const u8,
    header_len: c_int,
    id: *mut u8,
    tag: *mut u16,
    offset: *mut c_int,
) {
    unsafe {
        *id = *buf.add(4);
        let lo = *buf.add(5) as u16;
        let hi = *buf.add(6) as u16;
        *tag = lo | (hi << 8);
        *offset += header_len;
    }
}
}

// ── `return 0;` · virtio.c:2614-2614 · chunk 3919 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3919 {
// Ported piece of virtio_9p_recv_request: the dispatcher's trailing
// `return 0;`. It reads no locals and simply yields success (0).
#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3919() -> core::ffi::c_int {
    0
}
}

// ── `return 0;` · virtio.c:2617-2617 · chunk 3921 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3921 {
// Portion of virtio_9p_recv_request: successful return from the switch.
// The original C code is simply `return 0;`.

#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3921() -> core::ffi::c_int {
    0
}
}

// ── `protocol_error: fid_not_found: err = -71;` · virtio.c:2618-2620 · chunk 3922 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     protocol_error:
//      fid_not_found:
//         err = -71;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3922 {
use core::ffi::c_int;

// Sets the 9p protocol error code (-71) into the caller's `err` local.
#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3922(err: *mut c_int) {
    if !err.is_null() {
        unsafe {
            *err = -71;
        }
    }
}
}

// ── `goto error;` · virtio.c:2621-2621 · chunk 3923 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     goto error;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk3923 {
// Portion of virtio_9p_recv_request: the jump from the protocol_error /
// fid_not_found labels to the "error" label (which sends an error reply and
// returns 0).  The control transfer itself has to stay in C (goto), so this
// helper only reports that the error path must be taken.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn virtio_9p_recv_request__part3923() -> c_int {
    // Nonzero means: take the `error:` path.
    1
}
}

