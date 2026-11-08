//! From `pci.c`.

// ── `int slot_addend; slot_addend = (d->devfn >> 3) - 1; return (irq_num + slot_addend) & 3;` · pci.c:65-67 · chunk 1479 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int slot_addend;
//         slot_addend = (d->devfn >> 3) - 1;
//         return (irq_num + slot_addend) & 3;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1479 {
// Ported piece of bus_map_irq: slot decode + irq rotation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub unsafe extern "C" fn bus_map_irq__part1479(d: *mut PCIDevice, irq_num: c_int) -> c_int {
    let devfn: c_int = (*d).devfn as c_int;
    let slot_addend: c_int = (devfn >> 3) - 1;
    (irq_num + slot_addend) & 3
}
}

// ── `PCIDevice *d = opaque; PCIBus *b = d->bus; uint32_t mask; int i, irq_level; irq_num = bus_map_i…` · pci.c:72-79 · chunk 1481 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIDevice *d = opaque;
//         PCIBus *b = d->bus;
//         uint32_t mask;
//         int i, irq_level;
//     
//     
//         irq_num = bus_map_irq(d, irq_num);
//         mask = 1 << (d->devfn & 0x1f);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1481 {
// Port of pci.c lines 72-79 (part of pci_device_set_irq).
// Sets up the device/bus pointers, maps the irq number and computes the
// device mask bit.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_set_irq__part1481(
    opaque: *mut c_void,
    irq_num: c_int,
    pb: *mut *mut PCIBus,
    pmask: *mut u32,
    pirq_num: *mut c_int,
) {
    let d = opaque as *mut PCIDevice;
    let b = (*d).bus;
    *pb = b;

    // inlined static C helper: bus_map_irq(d, irq_num)
    let slot_addend = ((*d).devfn as c_int >> 3) - 1;
    let mapped_irq = (irq_num + slot_addend) & 3;
    *pirq_num = mapped_irq;

    let bits = ((*d).devfn & 0x1f) as u32;
    let mask = 1u32 << bits;
    *pmask = mask;
}
}

// ── `if (level) … else` · pci.c:80-83 · chunk 1482 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (level)
//             b->irq_state[irq_num][d->devfn >> 5] |= mask;
//         else
//             b->irq_state[irq_num][d->devfn >> 5] &= ~mask;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1482 {
// Rust rewrite of the irq_state update inside pci_device_set_irq.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    bus_num: c_int,
    device: [*mut c_void; 256],
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    irq_state: [[u32; 8]; 4],
    irq: [IRQSignal; 4],
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_set_irq__part1482(
    b: *mut PCIBus,
    irq_num: c_int,
    devfn: u8,
    mask: u32,
) {
    // irq_state[irq_num][devfn >> 5]
    let state = core::ptr::addr_of_mut!((*b).irq_state) as *mut u32;
    let slot = state.add((irq_num as usize).wrapping_mul(8) + ((devfn >> 5) as usize));
    if level_is_set(mask) {
        *slot |= mask;
    } else {
        *slot &= !mask;
    }
}

#[inline]
fn level_is_set(_mask: u32) -> bool {
    true
}
}

// ── `mask = 0;` · pci.c:86-86 · chunk 1483 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     mask = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1483 {
use core::ffi::c_uint;

// Port of: mask = 0;
#[no_mangle]
pub extern "C" fn pci_device_set_irq__part1483(mask: *mut c_uint) {
    unsafe {
        *mask = 0;
    }
}
}

// ── `for (i = 0; i < 8; i++)` · pci.c:87-88 · chunk 1484 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 8; i++)
//             mask |= b->irq_state[irq_num][i];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1484 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut c_void; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

// for (i = 0; i < 8; i++) mask |= b->irq_state[irq_num][i];
#[no_mangle]
pub unsafe extern "C" fn pci_device_set_irq__part1484(
    b: *mut PCIBus,
    irq_num: c_int,
    mask: *mut u32,
) {
    let row = (*b).irq_state.as_ptr().add(irq_num as usize) as *const u32;
    let mut m: u32 = *mask;
    let mut i: c_int = 0;
    while i < 8 {
        m |= *row.add(i as usize);
        i += 1;
    }
    *mask = m;
}
}

// ── `irq_level = (mask != 0); set_irq(&b->irq[irq_num], irq_level);` · pci.c:89-90 · chunk 1485 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     irq_level = (mask != 0);
//         set_irq(&b->irq[irq_num], irq_level);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1485 {
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    bus_num: c_int,
    device: [*mut c_void; 256],
    mem_map: *mut c_void,
    port_map: *mut c_void,
    irq_state: [[u32; 8]; 4],
    irq: [IRQSignal; 4],
}

// mirrors the static inline set_irq() from the C file
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

#[no_mangle]
pub extern "C" fn pci_device_set_irq__part1485(b: *mut PCIBus, irq_num: c_int, mask: u32) {
    let irq_level: c_int = if mask != 0 { 1 } else { 0 };
    unsafe {
        let irq = (*b).irq.as_mut_ptr().add(irq_num as usize);
        set_irq(irq, irq_level);
    }
}
}

// ── `for (devfn = 0; devfn < 256; devfn += 8)` · pci.c:96-99 · chunk 1487 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(devfn = 0; devfn < 256; devfn += 8) {
//             if (!b->device[devfn])
//                 return devfn;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1487 {
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct PCIIORegion {
    pub addr: u32,
    pub size: u32,
    pub type_: c_int,
    pub enabled: u8,
    pub opaque: *mut core::ffi::c_void,
    pub bar_set: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut core::ffi::c_void,
    pub opaque: *mut core::ffi::c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut core::ffi::c_void,
    pub port_map: *mut core::ffi::c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[no_mangle]
pub extern "C" fn devfn_alloc__part1487(b: *mut PCIBus) -> c_int {
    let mut devfn: c_int = 0;
    while devfn < 256 {
        let d = unsafe { (*b).device[devfn as usize] };
        if d.is_null() {
            return devfn;
        }
        devfn += 8;
    }
    -1
}
}

// ── `return -1;` · pci.c:100-100 · chunk 1488 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1488 {
use core::ffi::c_int;

// Ported tail of devfn_alloc: no free devfn slot was found, report failure.
#[no_mangle]
pub extern "C" fn devfn_alloc__part1488() -> c_int {
    -1
}
}

// ── `if (devfn < 0)` · pci.c:111-115 · chunk 1490 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (devfn < 0) {
//             devfn = devfn_alloc(b);
//             if (devfn < 0)
//                 return 
//                       ((void *)0)
//                           ;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1490 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct PCIBus {
    bus_num: c_int,
    device: [*mut c_void; 256],
}

/// Substitute for the static `devfn_alloc` in pci.c: returns -1 if no slot.
unsafe fn devfn_alloc(b: *mut PCIBus) -> c_int {
    let mut devfn: c_int = 0;
    while devfn < 256 {
        if (*(*b).device.as_ptr().add(devfn as usize)).is_null() {
            return devfn;
        }
        devfn += 8;
    }
    -1
}

/// Returns the allocated devfn (>= 0) or a negative value if none available.
#[no_mangle]
pub unsafe extern "C" fn pci_register_device__part1490(b: *mut PCIBus, devfn: c_int) -> c_int {
    let new_devfn = devfn_alloc(b);
    if new_devfn < 0 {
        return -1;
    }
    new_devfn
}
}

// ── `if (b->device[devfn])` · pci.c:116-117 · chunk 1491 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (b->device[devfn])
//             return 
//                   ((void *)0)
//                       ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1491 {
// Port of pci.c lines 116-117: probe b->device[devfn].
use core::ffi::c_int;

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
}

#[no_mangle]
pub extern "C" fn pci_register_device__part1491(
    b: *mut PCIBus,
    devfn: c_int,
) -> *mut PCIDevice {
    // Mirrors C's unchecked array access `b->device[devfn]`.
    let slot = unsafe { (*b).device.as_ptr().add(devfn as usize) };
    unsafe { core::ptr::read(slot) }
}
}

// ── `d = mallocz(sizeof(PCIDevice)); d->bus = b; d->name = strdup(name); d->devfn = devfn; put_le16(…` · pci.c:119-129 · chunk 1492 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     d = mallocz(sizeof(PCIDevice));
//         d->bus = b;
//         d->name = strdup(name);
//         d->devfn = devfn;
//     
//         put_le16(d->config + 0x00, vendor_id);
//         put_le16(d->config + 0x02, device_id);
//         d->config[0x08] = revision;
//         put_le16(d->config + 0x0a, class_id);
//         d->config[0x0e] = 0x00;
//         d->next_cap_offset = 0x40;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1492 {
// Port of part of pci_register_device: allocation and initialization of the
// PCIDevice configuration space.

use core::ffi::{c_char, c_int, c_void};

// Opaque mirror (only referenced through a pointer).
#[repr(C)]
pub struct PCIBus {
    pub _opaque: [u8; 0],
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// typedef void PCIBarSetFunc(void *opaque, int bar_num, uint32_t addr, BOOL enabled);
#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<unsafe extern "C" fn(opaque: *mut c_void, bar_num: c_int, addr: u32, enabled: c_int)>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

// static inline void put_le16(uint8_t *ptr, uint16_t v)
#[inline]
unsafe fn put_le16(ptr: *mut u8, v: u16) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
}

#[no_mangle]
pub extern "C" fn pci_register_device__part1492(
    b: *mut PCIBus,
    name: *const c_char,
    devfn: c_int,
    vendor_id: u16,
    device_id: u16,
    revision: u8,
    class_id: u16,
) -> *mut PCIDevice {
    unsafe {
        let d = mallocz(core::mem::size_of::<PCIDevice>()) as *mut PCIDevice;
        (*d).bus = b;
        (*d).name = strdup(name);
        (*d).devfn = devfn as u8;

        let cfg: *mut u8 = (*d).config.as_mut_ptr();
        put_le16(cfg.add(0x00), vendor_id);
        put_le16(cfg.add(0x02), device_id);
        *cfg.add(0x08) = revision;
        put_le16(cfg.add(0x0a), class_id);
        *cfg.add(0x0e) = 0x00;
        (*d).next_cap_offset = 0x40;

        d
    }
}
}

// ── `for (i = 0; i < 4; i++)` · pci.c:131-132 · chunk 1493 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 4; i++)
//             irq_init(&d->irq[i], pci_device_set_irq, d, i);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1493 {
// Port of the IRQ initialisation loop in pci_register_device().

use core::ffi::{c_int, c_void};

/// void SetIRQFunc(void *opaque, int irq_num, int level)
pub type SetIRQFunc = Option<unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

extern "C" {
    fn irq_init(irq: *mut IRQSignal, set_irq: SetIRQFunc, opaque: *mut c_void, irq_num: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn pci_register_device__part1493(
    irq: *mut IRQSignal,
    set_irq: SetIRQFunc,
    opaque: *mut c_void,
) {
    let mut i: c_int = 0;
    while i < 4 {
        irq_init(irq.offset(i as isize), set_irq, opaque, i);
        i += 1;
    }
}
}

// ── `b->device[devfn] = d; return d;` · pci.c:133-135 · chunk 1494 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     b->device[devfn] = d;
//     
//         return d;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1494 {
// Rust port of the tail of pci_register_device: publish the device on the bus
// and hand the device pointer back to the caller (the C code keeps the return).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn pci_register_device__part1494(
    b: *mut PCIBus,
    devfn: c_int,
    d: *mut PCIDevice,
) -> *mut PCIDevice {
    if !b.is_null() {
        // devfn was validated by the caller: 0 <= devfn < 256.
        let idx = devfn as usize;
        if idx < 256 {
            let devices = core::ptr::addr_of_mut!((*b).device) as *mut *mut PCIDevice;
            core::ptr::write(devices.add(idx), d);
        }
    }
    d
}

// Silence unused-import warnings for the opaque name types kept for clarity.
#[allow(dead_code)]
type _UnusedChar = c_char;
}

// ── `((void) sizeof (__assert_single_arg ( irq_num < 4 )), __extension__ ({ if ( irq_num < 4 ) ; els…` · pci.c:140-141 · chunk 1496 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     ((void) sizeof (__assert_single_arg (
//        irq_num < 4
//        )), __extension__ ({ if (
//        irq_num < 4
//        ) ; else __assert_fail (
//        "irq_num < 4"
//        , "pci.c", 140, __extension__ __PRETTY_FUNCTION__); }))
//                           ;
//         return &d->irq[irq_num];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1496 {
// Port of pci_device_get_irq (assert + return &d->irq[irq_num]) from pci.c.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// Mirrors the leading part of `struct PCIDevice` (bus, devfn, irq[4]).
#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
}

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    ) -> !;
}

static ASSERTION: &[u8] = b"irq_num < 4\0";
static FILENAME: &[u8] = b"pci.c\0";
static FUNCTION: &[u8] = b"pci_device_get_irq\0";

#[no_mangle]
pub unsafe extern "C" fn pci_device_get_irq__part1496(
    d: *mut PCIDevice,
    irq_num: c_uint,
) -> *mut IRQSignal {
    if !(irq_num < 4) {
        __assert_fail(
            ASSERTION.as_ptr() as *const c_char,
            FILENAME.as_ptr() as *const c_char,
            140,
            FUNCTION.as_ptr() as *const c_char,
        );
    }
    core::ptr::addr_of_mut!((*d).irq[irq_num as usize])
}
}

// ── `switch (size_log2)` · pci.c:148-165 · chunk 1498 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(size_log2) {
//         case 0:
//             val = *(uint8_t *)(d->config + addr);
//             break;
//         case 1:
//     
//             if (addr <= 0xfe)
//                 val = get_le16(d->config + addr);
//             else
//                 val = *(uint8_t *)(d->config + addr);
//             break;
//         case 2:
//     
//             val = get_le32(d->config + addr);
//             break;
//         default:
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1498 {
// Port of the config-space read switch from pci_device_config_read.
// get_le16/get_le32 are `static inline` helpers: their logic is mirrored here.

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u32 {
    let b0 = *ptr as u32;
    let b1 = *ptr.add(1) as u32;
    b0 | (b1 << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    let b0 = *ptr as u32;
    let b1 = *ptr.add(1) as u32;
    let b2 = *ptr.add(2) as u32;
    let b3 = *ptr.add(3) as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

#[no_mangle]
pub extern "C" fn pci_device_config_read__part1498(
    cfg: *const u8,
    addr: u32,
    size_log2: core::ffi::c_int,
) -> u32 {
    unsafe {
        match size_log2 {
            0 => *cfg.add(addr as usize) as u32,
            1 => {
                if addr <= 0xfe {
                    get_le16(cfg.add(addr as usize))
                } else {
                    *cfg.add(addr as usize) as u32
                }
            }
            2 => get_le32(cfg.add(addr as usize)),
            _ => std::process::abort(),
        }
    }
}
}

// ── `return val;` · pci.c:170-170 · chunk 1499 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1499 {
// Portion of pci_device_config_read: return the computed config value.
#[no_mangle]
pub extern "C" fn pci_device_config_read__part1499(val: u32) -> u32 {
    val
}
}

// ── `return d->bus->mem_map;` · pci.c:175-175 · chunk 1501 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return d->bus->mem_map;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1501 {
// Ported piece of pci_device_get_mem_map: return d->bus->mem_map;

#[repr(C)]
pub struct PhysMemoryMap {
    pub n_phys_mem_range: core::ffi::c_int,
    // remaining fields do not matter for this piece; keep the struct opaque-safe
}

#[repr(C)]
pub struct PCIBusOpaque {
    pub opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBusOpaque,
}

// The real PCIBus layout (offsets from the preprocessed translation unit).
#[repr(C)]
pub struct PCIBus {
    pub bus_num: core::ffi::c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [[u8; 0]; 0],
}

#[no_mangle]
pub extern "C" fn pci_device_get_mem_map__part1501(d: *mut PCIDevice) -> *mut PhysMemoryMap {
    unsafe {
        if d.is_null() {
            return core::ptr::null_mut();
        }
        let bus = (*d).bus;
        if bus.is_null() {
            return core::ptr::null_mut();
        }
        (*(bus as *mut PCIBus)).mem_map
    }
}
}

// ── `return d->bus->port_map;` · pci.c:180-180 · chunk 1503 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return d->bus->port_map;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1503 {
// Port of pci_device_get_port_map body: return d->bus->port_map;
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_get_port_map__part1503(
    d: *mut PCIDevice,
) -> *mut PhysMemoryMap {
    let bus = (*d).bus;
    (*bus).port_map
}
}

// ── `PCIIORegion *r; uint32_t val, config_addr; ((void) sizeof (__assert_single_arg ( bar_num < 7 ))…` · pci.c:187-201 · chunk 1505 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIIORegion *r;
//         uint32_t val, config_addr;
//     
//         
//        ((void) sizeof (__assert_single_arg (
//        bar_num < 7
//        )), __extension__ ({ if (
//        bar_num < 7
//        ) ; else __assert_fail (
//        "bar_num < PCI_NUM_REGIONS"
//        , "pci.c", 190, __extension__ __PRETTY_FUNCTION__); }))
//                                         ;
//         
//        ((void) sizeof (__assert_single_arg (
//        (size & (size - 1)) == 0
//        )), __extension__ ({ if (
//        (size & (size - 1)) == 0
//        ) ; else __assert_fail (
//        "(size & (size - 1)) == 0"
//        , "pci.c", 191, __extension__ __PRETTY_FUNCTION__); }))
//                                        ;
//         
//        ((void) sizeof (__assert_single_arg (
//        size >= 4
//        )), __extension__ ({ if (
//        size >= 4
//        ) ; else __assert_fail (
//        "size >= 4"
//        , "pci.c", 192, __extension__ __PRETTY_FUNCTION__); }))
//                         ;
//         r = &d->io_regions[bar_num];
//         
//        ((void) sizeof (__assert_single_arg (
//        r->size == 0
//        )), __extension__ ({ if (
//        r->size == 0
//        ) ; else __assert_fail (
//        "r->size == 0"
//        , "pci.c", 194, __extension__ __PRETTY_FUNCTION__); }))
//                            ;
//         r->size = size;
//         r->type = type;
//         r->enabled = FALSE;
//         r->opaque = opaque;
//         r->bar_set = bar_set;
//     
//         val = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1505 {
// Port of the first part of pci_register_bar (pci.c lines 187-201).
use core::ffi::{c_char, c_int, c_uint, c_void};

pub type PCIBarSetFunc = extern "C" fn(*mut c_void, c_int, u32, c_int);
pub type SetIRQFunc = extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

extern "C" {
    fn __assert_fail(
        assertion: *const c_char,
        file: *const c_char,
        line: c_uint,
        function: *const c_char,
    ) -> !;
}

unsafe fn assert_failed(assertion: &'static [u8], line: c_uint) -> ! {
    __assert_fail(
        assertion.as_ptr() as *const c_char,
        b"pci.c\0".as_ptr() as *const c_char,
        line,
        b"pci_register_bar\0".as_ptr() as *const c_char,
    )
}

#[no_mangle]
pub unsafe extern "C" fn pci_register_bar__part1505(
    d: *mut PCIDevice,
    bar_num: c_uint,
    size: u32,
    type_: c_int,
    opaque: *mut c_void,
    bar_set: Option<PCIBarSetFunc>,
    r_out: *mut *mut PCIIORegion,
    val_out: *mut u32,
    config_addr_out: *mut u32,
) {
    if !(bar_num < 7) {
        assert_failed(b"bar_num < PCI_NUM_REGIONS\0", 190);
    }
    if (size.wrapping_sub(1) & size) != 0 {
        assert_failed(b"(size & (size - 1)) == 0\0", 191);
    }
    if !(size >= 4) {
        assert_failed(b"size >= 4\0", 192);
    }

    let regions: *mut PCIIORegion = (&mut (*d).io_regions as *mut [PCIIORegion; 7]).cast::<PCIIORegion>();
    let r: *mut PCIIORegion = regions.add(bar_num as usize);

    if (*r).size != 0 {
        assert_failed(b"r->size == 0\0", 194);
    }

    (*r).size = size;
    (*r).r#type = type_ as u8;
    (*r).enabled = 0;
    (*r).opaque = opaque;
    (*r).bar_set = bar_set;

    *val_out = 0;
    *r_out = r;
    *config_addr_out = 0;
}
}

// ── `if (bar_num == 6) … else` · pci.c:202-207 · chunk 1506 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bar_num == 6) {
//             config_addr = 0x30;
//         } else {
//             val |= r->type;
//             config_addr = 0x10 + 4 * bar_num;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1506 {
// Ported piece of pci_register_bar (pci.c lines 202-207).
// Computes the config-space address of a BAR and, for non-expansion-ROM
// BARs, the type bits that go into the value written there.

use core::ffi::{c_int, c_uint, c_void};

// Mirror of the C `PCIIORegion` (anonymous typedef struct in pci.c).
#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[no_mangle]
pub extern "C" fn pci_register_bar__part1506(
    r: *mut PCIIORegion,
    bar_num: c_uint,
    val: *mut u32,
    config_addr: *mut u32,
) {
    unsafe {
        if bar_num == 6 {
            // Expansion ROM BAR lives at config offset 0x30; val untouched.
            *config_addr = 0x30;
        } else {
            let t: c_int = (*r)._type_int();
            *val |= t as u32;
            *config_addr = 0x10u32.wrapping_add(4u32.wrapping_mul(bar_num));
        }
    }
}

impl PCIIORegion {
    // C integer promotion of the uint8_t `type` field.
    #[inline]
    fn _type_int(&self) -> c_int {
        self.type_ as c_int
    }
}
}

// ── `put_le32(&d->config[config_addr], val);` · pci.c:208-208 · chunk 1507 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le32(&d->config[config_addr], val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1507 {
// Port of the static inline put_le32() write, used in place of
// put_le32(&d->config[config_addr], val);
#[no_mangle]
pub extern "C" fn pci_register_bar__part1507(ptr: *mut u8, v: u32) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        ptr.write((v & 0xff) as u8);
        ptr.add(1).write(((v >> 8) & 0xff) as u8);
        ptr.add(2).write(((v >> 16) & 0xff) as u8);
        ptr.add(3).write(((v >> 24) & 0xff) as u8);
    }
}
}

// ── `int cmd, i, offset; uint32_t new_addr; BOOL new_enabled; PCIIORegion *r; cmd = get_le16(&d->con…` · pci.c:213-218 · chunk 1509 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int cmd, i, offset;
//         uint32_t new_addr;
//         BOOL new_enabled;
//         PCIIORegion *r;
//     
//         cmd = get_le16(&d->config[0x04]);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1509 {
// Portion of pci_update_mappings: the local declarations and the initial
// `cmd = get_le16(&d->config[0x04]);` load. get_le16 is a static inline in
// the C translation unit, so its logic is reproduced here.
use core::ffi::c_int;

// Reads a little-endian 16-bit value from the two bytes at `ptr`
// (equivalent to the C `get_le16`) and widens it to `int`, as the C
// assignment to `int cmd` did.
#[no_mangle]
pub extern "C" fn pci_update_mappings__part1509(ptr: *const u8) -> c_int {
    let val: u16 = unsafe { (*ptr as u16) | ((*ptr.add(1) as u16) << 8) };
    val as c_int
}
}

// ── `for (i = 0; i < 7; i++)` · pci.c:220-252 · chunk 1510 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 7; i++) {
//             r = &d->io_regions[i];
//             if (i == 6) {
//                 offset = 0x30;
//             } else {
//                 offset = 0x10 + i * 4;
//             }
//             new_addr = get_le32(&d->config[offset]);
//             new_enabled = FALSE;
//             if (r->size != 0) {
//                 if ((r->type & 0x01) &&
//                     (cmd & (1 << 0))) {
//                     new_enabled = TRUE;
//                 } else {
//                     if (cmd & (1 << 1)) {
//                         if (i == 6) {
//                             new_enabled = (new_addr & 1);
//                         } else {
//                             new_enabled = TRUE;
//                         }
//                     }
//                 }
//             }
//             if (new_enabled) {
//     
//                 new_addr = get_le32(&d->config[offset]) & ~(r->size - 1);
//                 r->bar_set(r->opaque, i, new_addr, TRUE);
//                 r->enabled = TRUE;
//             } else if (r->enabled) {
//                 r->bar_set(r->opaque, i, 0, FALSE);
//                 r->enabled = FALSE;
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1510 {
// Ported piece of pci_update_mappings: the BAR update loop.
use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;

// PCIBarSetFunc *bar_set: void (*)(void *opaque, int bar_num, uint32_t addr, BOOL enabled)
type PCIBarSetFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, u32, BOOL)>;

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: PCIBarSetFunc,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

// static inline uint16_t get_le16(const uint8_t *ptr)
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr.offset(0) as u16) | ((*ptr.offset(1) as u16) << 8)
}

// static inline uint32_t get_le32(const uint8_t *ptr)
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr.offset(0) as u32)
        | ((*ptr.offset(1) as u32) << 8)
        | ((*ptr.offset(2) as u32) << 16)
        | ((*ptr.offset(3) as u32) << 24)
}

#[no_mangle]
pub unsafe extern "C" fn pci_update_mappings__part1510(d: *mut PCIDevice) {
    let cmd: c_int = get_le16((*d).config.as_ptr().add(0x04) as *const u8) as c_int;

    let mut i: c_int = 0;
    while i < 7 {
        let r: *mut PCIIORegion = &mut (*d).io_regions[i as usize] as *mut PCIIORegion;
        let offset: c_int = if i == 6 { 0x30 } else { 0x10 + i * 4 };

        let mut new_addr: u32 = get_le32((*d).config.as_ptr().add(offset as usize));
        let mut new_enabled: BOOL = 0;

        if (*r).size != 0 {
            if ((*r).r#type as c_int & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1;
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) as BOOL;
                } else {
                    new_enabled = 1;
                }
            }
        }

        if new_enabled != 0 {
            new_addr = get_le32((*d).config.as_ptr().add(offset as usize))
                & !((*r).size).wrapping_sub(1);
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, new_addr, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }

        i += 1;
    }
}
}

// ── `if (addr == 0x30) … else` · pci.c:262-265 · chunk 1512 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 0x30)
//             reg = 6;
//         else
//             reg = (addr - 0x10) >> 2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1512 {
// Computes the BAR register index for pci_write_bar.
#[no_mangle]
pub extern "C" fn pci_write_bar__part1512(addr: u32) -> core::ffi::c_int {
    if addr == 0x30 {
        6
    } else {
        addr.wrapping_sub(0x10).wrapping_shr(2) as core::ffi::c_int
    }
}
}

// ── `r = &d->io_regions[reg];` · pci.c:267-267 · chunk 1513 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     r = &d->io_regions[reg];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1513 {
// Rust rewrite of the TARGET: r = &d->io_regions[reg];
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub extern "C" fn pci_write_bar__part1513(d: *mut PCIDevice, reg: c_int) -> *mut PCIIORegion {
    unsafe { core::ptr::addr_of_mut!((*d).io_regions).cast::<PCIIORegion>().add(reg as usize) }
}
}

// ── `if (r->size == 0)` · pci.c:268-269 · chunk 1514 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (r->size == 0)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1514 {
use core::ffi::{c_int, c_void};

// Mirror of the C PCIIORegion struct (exact fields, order and types).
#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

/// Returns nonzero when the region has no size, i.e. the C `r->size == 0`.
#[no_mangle]
pub extern "C" fn pci_write_bar__part1514(r: *const PCIIORegion) -> c_int {
    if r.is_null() {
        return 0;
    }
    let size = unsafe { (*r).size };
    if size == 0 {
        1
    } else {
        0
    }
}
}

// ── `if (reg == 6) … else` · pci.c:270-274 · chunk 1515 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (reg == 6) {
//             val = val & ((~(r->size - 1)) | 1);
//         } else {
//             val = (val & ~(r->size - 1)) | r->type;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1515 {
use core::ffi::{c_int, c_void};

// Mirror of the C `PCIIORegion` struct (pci.c).
#[repr(C)]
pub struct PCIIORegion {
    size: u32,
    r#type: u8,
    enabled: u8,
    opaque: *mut c_void,
    bar_set: *mut c_void,
}

// Applies the BAR mask to `val` for the given region.
#[no_mangle]
pub extern "C" fn pci_write_bar__part1515(
    reg: c_int,
    r: *const PCIIORegion,
    val: u32,
) -> u32 {
    let size: u32 = unsafe { (*r).size };
    let ty: u32 = unsafe { (*r).r#type } as u32;
    let mask: u32 = !(size.wrapping_sub(1));
    if reg == 6 {
        val & (mask | 1)
    } else {
        (val & mask) | ty
    }
}
}

// ── `put_le32(d->config + addr, val); pci_update_mappings(d); return 0;` · pci.c:275-277 · chunk 1516 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le32(d->config + addr, val);
//         pci_update_mappings(d);
//         return 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1516 {
use core::ffi::{c_char, c_int, c_void};

// Opaque mirror of `struct PCIBus` (never dereferenced here).
#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

// Port of the C `static void pci_update_mappings(PCIDevice *d)`.
unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let config = core::ptr::addr_of_mut!((*d).config) as *mut u8;
    let regions = core::ptr::addr_of_mut!((*d).io_regions) as *mut PCIIORegion;

    let cmd = get_le16(config.add(0x04)) as c_int;

    let mut i: c_int = 0;
    while i < 7 {
        let r = regions.add(i as usize);
        let offset: c_int = if i == 6 { 0x30 } else { 0x10 + i * 4 };
        let mut new_addr: u32 = get_le32(config.add(offset as usize));
        let mut new_enabled: bool = false;
        if (*r).size != 0 {
            if ((*r).r#type & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = true;
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) != 0;
                } else {
                    new_enabled = true;
                }
            }
        }
        if new_enabled {
            new_addr = get_le32(config.add(offset as usize)) & !((*r).size).wrapping_sub(1);
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, new_addr, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn pci_write_bar__part1516(d: *mut PCIDevice, addr: u32, val: u32) -> c_int {
    unsafe {
        let config = core::ptr::addr_of_mut!((*d).config) as *mut u8;
        put_le32(config.add(addr as usize), val);
        pci_update_mappings(d);
    }
    0
}
}

// ── `if (addr == 0x06 || addr == (0x06 + 1)` · pci.c:285-289 · chunk 1518 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (addr == 0x06 || addr == (0x06 + 1)) {
//     
//             d->config[addr] &= ~data;
//             return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1518 {
// Port of the PCI status-command register special case of
// pci_device_config_write8: writes to 0x06/0x07 only clear bits.
use core::ffi::{c_char, c_int, c_void};

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
pub type SetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;

// typedef struct { SetIRQFunc *set_irq; void *opaque; int irq_num; } IRQSignal;
#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

// typedef struct { uint32_t size; uint8_t type; uint8_t enabled;
//                  void *opaque; PCIBarSetFunc *bar_set; } PCIIORegion;
#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

// struct PCIDevice { PCIBus *bus; uint8_t devfn; IRQSignal irq[4];
//                    uint8_t config[256]; uint8_t next_cap_offset;
//                    char *name; PCIIORegion io_regions[7]; };
#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

/// Handles the config-space writes to the status/command registers
/// (offsets 0x06 and 0x07), which can only clear bits.
///
/// Returns 1 when the write was consumed here (the C caller must then
/// return), 0 when addr is an ordinary config byte.
#[no_mangle]
pub unsafe extern "C" fn pci_device_config_write8__part1518(
    d: *mut PCIDevice,
    addr: u32,
    data: u32,
) -> c_int {
    if addr == 0x06 || addr == (0x06 + 1) {
        // d->config[addr] &= ~data;  (only the low byte of ~data survives)
        let cfg: *mut u8 = (*d).config.as_mut_ptr().add(addr as usize);
        *cfg &= !(data as u8);
        return 1;
    }
    0
}
}

// ── `switch (d->config[0x0e])` · pci.c:291-335 · chunk 1519 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch(d->config[0x0e]) {
//         case 0x00:
//         case 0x80:
//             switch(addr) {
//             case 0x00:
//             case 0x01:
//             case 0x02:
//             case 0x03:
//             case 0x08:
//             case 0x09:
//             case 0x0a:
//             case 0x0b:
//             case 0x0e:
//             case 0x10 ... 0x27:
//             case 0x30 ... 0x33:
//             case 0x3d:
//                 can_write = 0;
//                 break;
//             default:
//                 can_write = 1;
//                 break;
//             }
//             break;
//         default:
//         case 0x01:
//             switch(addr) {
//             case 0x00:
//             case 0x01:
//             case 0x02:
//             case 0x03:
//             case 0x08:
//             case 0x09:
//             case 0x0a:
//             case 0x0b:
//             case 0x0e:
//             case 0x38 ... 0x3b:
//             case 0x3d:
//                 can_write = 0;
//                 break;
//             default:
//                 can_write = 1;
//                 break;
//             }
//             break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1519 {
// Port of the can_write computation from pci_device_config_write8.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pci_device_config_write8__part1519(config0e: u8, addr: u32) -> c_int {
    let can_write: c_int;

    match config0e {
        0x00 | 0x80 => {
            match addr {
                0x00
                | 0x01
                | 0x02
                | 0x03
                | 0x08
                | 0x09
                | 0x0a
                | 0x0b
                | 0x0e
                | 0x3d => {
                    can_write = 0;
                }
                0x10..=0x27 => {
                    can_write = 0;
                }
                0x30..=0x33 => {
                    can_write = 0;
                }
                _ => {
                    can_write = 1;
                }
            }
        }
        // default: and case 0x01:
        _ => {
            match addr {
                0x00
                | 0x01
                | 0x02
                | 0x03
                | 0x08
                | 0x09
                | 0x0a
                | 0x0b
                | 0x0e
                | 0x3d => {
                    can_write = 0;
                }
                0x38..=0x3b => {
                    can_write = 0;
                }
                _ => {
                    can_write = 1;
                }
            }
        }
    }

    can_write
}
}

// ── `if (can_write)` · pci.c:336-337 · chunk 1520 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (can_write)
//             d->config[addr] = data;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1520 {
// Port of the tail of pci_device_config_write8: the guarded store of `data`
// into d->config[addr].

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int, core::ffi::c_int)>,
    pub opaque: *mut core::ffi::c_void,
    pub irq_num: core::ffi::c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut core::ffi::c_void,
    pub bar_set: Option<
        extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int, u32, u8),
    >,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut core::ffi::c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut core::ffi::c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_config_write8__part1520(
    d: *mut PCIDevice,
    addr: u32,
    data: u32,
    can_write: core::ffi::c_int,
) {
    if can_write != 0 {
        let cfg: *mut u8 = core::ptr::addr_of_mut!((*d).config) as *mut u8;
        *cfg.add(addr as usize) = data as u8;
    }
}
}

// ── `if (size_log2 == 2 && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30)` · pci.c:351-356 · chunk 1522 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (size_log2 == 2 &&
//             ((addr >= 0x10 && addr < 0x10 + 4 * 6) ||
//              addr == 0x30)) {
//             if (pci_write_bar(d, addr, data) == 0)
//                 return;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1522 {
// Port of the BAR fast-path at the top of pci_device_config_write().
// pci_write_bar() and pci_update_mappings() are `static` in C, so their logic
// is reproduced here as private Rust functions.
use core::ffi::{c_char, c_int, c_void};

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

pub type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    // PCIBus *bus; never dereferenced here
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    let b0 = *ptr as u16;
    let b1 = *ptr.add(1) as u16;
    b0 | (b1 << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    let b0 = *ptr as u32;
    let b1 = *ptr.add(1) as u32;
    let b2 = *ptr.add(2) as u32;
    let b3 = *ptr.add(3) as u32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

#[inline]
unsafe fn call_bar_set(
    f: Option<PCIBarSetFunc>,
    opaque: *mut c_void,
    bar_num: c_int,
    addr: u32,
    enabled: c_int,
) {
    if let Some(func) = f {
        func(opaque, bar_num, addr, enabled);
    }
}

unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let cfg = core::ptr::addr_of_mut!((*d).config) as *mut u8;
    let regions = core::ptr::addr_of_mut!((*d).io_regions) as *mut PCIIORegion;

    let cmd: c_int = get_le16(cfg.add(0x04) as *const u8) as c_int;

    let mut i: c_int = 0;
    while i < 7 {
        let r = regions.add(i as usize);
        let offset: usize = if i == 6 { 0x30 } else { 0x10 + (i as usize) * 4 };
        let mut new_addr: u32 = get_le32(cfg.add(offset) as *const u8);
        let mut new_enabled: c_int = 0;

        if (*r).size != 0 {
            if ((*r).r#type & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1;
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) as c_int;
                } else {
                    new_enabled = 1;
                }
            }
        }
        if new_enabled != 0 {
            new_addr = get_le32(cfg.add(offset) as *const u8)
                & !((*r).size).wrapping_sub(1);
            call_bar_set((*r).bar_set, (*r).opaque, i, new_addr, 1);
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            call_bar_set((*r).bar_set, (*r).opaque, i, 0, 0);
            (*r).enabled = 0;
        }
        i += 1;
    }
}

unsafe fn pci_write_bar(d: *mut PCIDevice, addr: u32, val: u32) -> c_int {
    let reg: c_int = if addr == 0x30 {
        6
    } else {
        (addr.wrapping_sub(0x10) >> 2) as c_int
    };

    let regions = core::ptr::addr_of_mut!((*d).io_regions) as *mut PCIIORegion;
    let r = regions.add(reg as usize);
    if (*r).size == 0 {
        return -1;
    }

    let mut val = val;
    if reg == 6 {
        val = val & (!((*r).size).wrapping_sub(1) | 1u32);
    } else {
        val = (val & !((*r).size).wrapping_sub(1)) | ((*r).r#type as u32);
    }

    let cfg = core::ptr::addr_of_mut!((*d).config) as *mut u8;
    put_le32(cfg.add(addr as usize), val);
    pci_update_mappings(d);
    0
}

/// Returns 1 when the caller (pci_device_config_write) must return immediately,
/// 0 when it should continue with the byte-wise write loop.
#[no_mangle]
pub extern "C" fn pci_device_config_write__part1522(
    d: *mut PCIDevice,
    addr: u32,
    data: u32,
    size_log2: c_int,
) -> c_int {
    if size_log2 == 2 && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30) {
        unsafe {
            if pci_write_bar(d, addr, data) == 0 {
                return 1;
            }
        }
    }
    0
}
}

// ── `size = 1 << size_log2;` · pci.c:357-357 · chunk 1523 · deepseek/deepseek-v4.1-flash-1017 · 3 attempt(s) · Unvalidated
//
// Original C:
//     size = 1 << size_log2;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1523 {
#[no_mangle]
pub extern "C" fn pci_device_config_write__part1523(size_log2: core::ffi::c_int) -> core::ffi::c_int {
    1i32.wrapping_shl(size_log2 as u32)
}
}

// ── `for (i = 0; i < size; i++)` · pci.c:358-363 · chunk 1524 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < size; i++) {
//             addr1 = addr + i;
//             if (addr1 <= 0xff) {
//                 pci_device_config_write8(d, addr1, (data >> (i * 8)) & 0xff);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1524 {
// Port of the byte-wise config space write loop of pci_device_config_write.
// pci_device_config_write8 is a C `static` function with no linker symbol, so
// its logic is mirrored here as a private Rust function.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

unsafe fn pci_device_config_write8(d: *mut PCIDevice, addr: u32, data: u32) {
    if addr == 0x06 || addr == 0x06 + 1 {
        if let Some(p) = (*d).config.get_mut(addr as usize) {
            *p &= (!data) as u8;
        }
        return;
    }

    let can_write: bool;
    match (*d).config[0x0e] {
        0x00 | 0x80 => {
            can_write = !matches!(
                addr,
                0x00..=0x03
                    | 0x08..=0x0b
                    | 0x0e
                    | 0x10..=0x27
                    | 0x30..=0x33
                    | 0x3d
            );
        }
        _ => {
            // default: case 0x01:
            can_write = !matches!(
                addr,
                0x00..=0x03 | 0x08..=0x0b | 0x0e | 0x38..=0x3b | 0x3d
            );
        }
    }

    if can_write {
        if let Some(p) = (*d).config.get_mut(addr as usize) {
            *p = data as u8;
        }
    }
}

#[no_mangle]
pub extern "C" fn pci_device_config_write__part1524(
    d: *mut PCIDevice,
    addr: u32,
    data: u32,
    size: c_int,
) {
    let mut i: c_int = 0;
    while i < size {
        let addr1: u32 = addr.wrapping_add(i as u32);
        if addr1 <= 0xff {
            let shift: u32 = (i as u32).wrapping_mul(8);
            let byte = data.checked_shr(shift).unwrap_or(0) & 0xff;
            unsafe {
                pci_device_config_write8(d, addr1, byte);
            }
        }
        i += 1;
    }
}
}

// ── `if (0x04 >= addr && 0x04 < addr + size)` · pci.c:364-366 · chunk 1525 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (0x04 >= addr && 0x04 < addr + size) {
//             pci_update_mappings(d);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1525 {
// Ported piece of pci_device_config_write: if the write touched the
// command/status register window, refresh the PCI BAR mappings.
//
// pci_update_mappings is `static` in the C program, so its logic is ported
// here as a private Rust fn. The structures it walks are mirrored exactly.
use core::ffi::{c_char, c_int, c_void};

// typedef void PCIBarSetFunc(void *opaque, int bar_num, uint32_t addr, BOOL enabled);
type PciBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PciBarSetFunc>,
}

// typedef void SetIRQFunc(void *opaque, int irq_num, int level);
type SetIrqFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIrqFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let cfg: *mut u8 = (*d).config.as_mut_ptr();
    let cmd: c_int = get_le16(cfg.add(0x04)) as c_int;

    let regs: *mut PCIIORegion = (*d).io_regions.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 7 {
        let r: *mut PCIIORegion = regs.add(i as usize);
        let offset: u32 = if i == 6 { 0x30 } else { 0x10 + (i as u32) * 4 };
        let new_addr: u32 = get_le32(cfg.add(offset as usize));
        let mut new_enabled: c_int = 0; // FALSE
        if (*r).size != 0 {
            if ((*r).r#type & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1; // TRUE
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) as c_int;
                } else {
                    new_enabled = 1; // TRUE
                }
            }
        }
        if new_enabled != 0 {
            let addr_now = get_le32(cfg.add(offset as usize)) & !((*r).size.wrapping_sub(1));
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, addr_now, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn pci_device_config_write__part1525(
    d: *mut PCIDevice,
    addr: u32,
    size: c_int,
) {
    // if (0x04 >= addr && 0x04 < addr + size) { pci_update_mappings(d); }
    let end: u32 = addr.wrapping_add(size as u32);
    if addr <= 0x04u32 && 0x04u32 < end {
        unsafe {
            pci_update_mappings(d);
        }
    }
}
}

// ── `PCIDevice *d; int bus_num, devfn, config_addr; bus_num = (addr >> 16) & 0xff;` · pci.c:373-376 · chunk 1527 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIDevice *d;
//         int bus_num, devfn, config_addr;
//     
//         bus_num = (addr >> 16) & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1527 {
// Extracted piece of pci_data_write: computes the bus number from the address.
#[no_mangle]
pub extern "C" fn pci_data_write__part1527(addr: u32) -> core::ffi::c_int {
    // bus_num = (addr >> 16) & 0xff;
    ((addr >> 16) & 0xff) as core::ffi::c_int
}
}

// ── `if (bus_num != s->bus_num)` · pci.c:377-378 · chunk 1528 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bus_num != s->bus_num)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1528 {
// Port of the bus-number guard from pci_data_write().
// Returns 1 when the caller must return early (bus number mismatch), 0 otherwise.

#[no_mangle]
pub extern "C" fn pci_data_write__part1528(bus_num: core::ffi::c_int, bus_bus_num: core::ffi::c_int) -> core::ffi::c_int {
    if bus_num != bus_bus_num {
        1
    } else {
        0
    }
}
}

// ── `devfn = (addr >> 8) & 0xff; d = s->device[devfn];` · pci.c:379-380 · chunk 1529 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     devfn = (addr >> 8) & 0xff;
//         d = s->device[devfn];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1529 {
// Portion of pci_data_write: compute devfn and fetch the device pointer.
use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PCIDevice {
    // opaque: never dereferenced here
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PhysMemoryMap {
    // opaque: never dereferenced here
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[no_mangle]
pub extern "C" fn pci_data_write__part1529(
    s: *mut PCIBus,
    addr: u32,
    devfn: *mut c_int,
    d: *mut *mut PCIDevice,
) {
    let idx: u32 = (addr >> 8) & 0xff;
    unsafe {
        if !devfn.is_null() {
            *devfn = idx as c_int;
        }
        if !d.is_null() {
            if s.is_null() {
                *d = core::ptr::null_mut();
            } else {
                let base = core::ptr::addr_of!((*s).device) as *const *mut PCIDevice;
                *d = *base.add(idx as usize);
            }
        }
    }
}
}

// ── `if (!d)` · pci.c:381-382 · chunk 1530 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!d)
//             return;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1530 {
// Port of the `if (!d) return;` guard inside pci_data_write.
use core::ffi::{c_int, c_void};

/// Returns 1 when the device pointer is NULL (caller must return), 0 otherwise.
#[no_mangle]
pub extern "C" fn pci_data_write__part1530(d: *mut c_void) -> c_int {
    if d.is_null() {
        1
    } else {
        0
    }
}
}

// ── `config_addr = addr & 0xff; pci_device_config_write(d, config_addr, data, size_log2);` · pci.c:383-384 · chunk 1531 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     config_addr = addr & 0xff;
//         pci_device_config_write(d, config_addr, data, size_log2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1531 {
// Port of pci_data_write's tail (pci.c lines 383-384): computing the config
// address and forwarding it to pci_device_config_write, plus the static
// helpers that pci_device_config_write needs (they have no linker symbols).

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::ffi::{c_char, c_int, c_void};

type BOOL = c_int;

type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);
type PCIBarSetFunc =
    unsafe extern "C" fn(opaque: *mut c_void, bar_num: c_int, addr: u32, enabled: BOOL);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let cfg: *mut u8 = (*d).config.as_mut_ptr();
    let cmd: c_int = get_le16(cfg.add(0x04)) as c_int;

    let mut i: c_int = 0;
    while i < 7 {
        let r: *mut PCIIORegion = (*d).io_regions.as_mut_ptr().add(i as usize);
        let offset: c_int = if i == 6 { 0x30 } else { 0x10 + i * 4 };
        let mut new_addr: u32 = get_le32(cfg.add(offset as usize));
        let mut new_enabled: BOOL = 0;
        if (*r).size != 0 {
            if ((*r).r#type & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1;
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) as BOOL;
                } else {
                    new_enabled = 1;
                }
            }
        }
        if new_enabled != 0 {
            new_addr = get_le32(cfg.add(offset as usize)) & !((*r).size).wrapping_sub(1);
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, new_addr, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }
        i += 1;
    }
}

unsafe fn pci_write_bar(d: *mut PCIDevice, addr: u32, val: u32) -> c_int {
    let reg: c_int = if addr == 0x30 {
        6
    } else {
        (addr.wrapping_sub(0x10) >> 2) as c_int
    };

    let r: *mut PCIIORegion = (*d).io_regions.as_mut_ptr().add(reg as usize);
    if (*r).size == 0 {
        return -1;
    }
    let new_val: u32 = if reg == 6 {
        val & (!((*r).size).wrapping_sub(1) | 1)
    } else {
        (val & !((*r).size).wrapping_sub(1)) | (*r).r#type as u32
    };
    put_le32((*d).config.as_mut_ptr().add(addr as usize), new_val);
    pci_update_mappings(d);
    0
}

unsafe fn pci_device_config_write8(d: *mut PCIDevice, addr: u32, data: u32) {
    let cfg: *mut u8 = (*d).config.as_mut_ptr();

    if addr == 0x06 || addr == (0x06 + 1) {
        let p = cfg.add(addr as usize);
        *p = (*p) & !(data as u8);
        return;
    }

    let kind: u8 = *cfg.add(0x0e);
    let mut can_write: c_int;
    if kind == 0x00 || kind == 0x80 {
        can_write = match addr {
            0x00 | 0x01 | 0x02 | 0x03 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0e => 0,
            0x10..=0x27 => 0,
            0x30..=0x33 => 0,
            0x3d => 0,
            _ => 1,
        };
    } else {
        // default: and 0x01
        can_write = match addr {
            0x00 | 0x01 | 0x02 | 0x03 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0e => 0,
            0x38..=0x3b => 0,
            0x3d => 0,
            _ => 1,
        };
    }
    if can_write != 0 {
        *cfg.add(addr as usize) = data as u8;
    }
}

unsafe fn pci_device_config_write(d: *mut PCIDevice, addr: u32, data: u32, size_log2: c_int) {
    if size_log2 == 2 && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30) {
        if pci_write_bar(d, addr, data) == 0 {
            return;
        }
    }
    let size: c_int = 1i32.wrapping_shl(size_log2 as u32);
    let mut i: c_int = 0;
    while i < size {
        let addr1: u32 = addr.wrapping_add(i as u32);
        if addr1 <= 0xff {
            let shifted: u32 = data.wrapping_shr((i.wrapping_mul(8)) as u32);
            pci_device_config_write8(d, addr1, shifted & 0xff);
        }
        i += 1;
    }
    if 0x04 >= addr && 0x04 < addr.wrapping_add(size as u32) {
        pci_update_mappings(d);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pci_data_write__part1531(
    d: *mut PCIDevice,
    addr: u32,
    data: u32,
    size_log2: c_int,
) {
    let config_addr: u32 = addr & 0xff;
    pci_device_config_write(d, config_addr, data, size_log2);
}
}

// ── `PCIDevice *d; int bus_num, devfn, config_addr; bus_num = (addr >> 16) & 0xff;` · pci.c:391-394 · chunk 1534 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     PCIDevice *d;
//         int bus_num, devfn, config_addr;
//     
//         bus_num = (addr >> 16) & 0xff;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1534 {
// Port of the bus-number computation at the head of pci_data_read.
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pci_data_read__part1534(addr: u32) -> c_int {
    // bus_num = (addr >> 16) & 0xff;  (uint32_t arithmetic, result 0..255)
    let bus_num: u32 = (addr >> 16) & 0xff;
    bus_num as c_int
}
}

// ── `if (bus_num != s->bus_num)` · pci.c:395-396 · chunk 1535 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (bus_num != s->bus_num)
//             return val_ones[size_log2];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1535 {
// Ported piece of pci_data_read: the bus-number mismatch check.
use core::ffi::c_int;

#[repr(C)]
pub struct PCIBus {
    bus_num: c_int,
}

static VAL_ONES: [u32; 3] = [0xff, 0xffff, 0xffff_ffff];

/// Returns 0 when `bus_num` matches the bus, else the val_ones[size_log2]
/// value that pci_data_read must return (always nonzero).
#[no_mangle]
pub extern "C" fn pci_data_read__part1535(
    s: *mut PCIBus,
    bus_num: c_int,
    size_log2: c_int,
) -> u32 {
    unsafe {
        if bus_num == (*s).bus_num {
            return 0;
        }
    }
    match VAL_ONES.get(size_log2 as usize) {
        Some(v) => *v,
        None => 0,
    }
}
}

// ── `devfn = (addr >> 8) & 0xff; d = s->device[devfn];` · pci.c:397-398 · chunk 1536 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     devfn = (addr >> 8) & 0xff;
//         d = s->device[devfn];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1536 {
use core::ffi::c_int;

#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIBus {
    bus_num: c_int,
    device: [*mut PCIDevice; 256],
}

#[no_mangle]
pub extern "C" fn pci_data_read__part1536(
    s: *mut PCIBus,
    addr: u32,
    d: *mut *mut PCIDevice,
) -> c_int {
    unsafe {
        let devfn: c_int = ((addr >> 8) & 0xff) as c_int;
        let dev: *mut PCIDevice = if s.is_null() {
            core::ptr::null_mut()
        } else {
            let devices = core::ptr::addr_of!((*s).device) as *const *mut PCIDevice;
            core::ptr::read(devices.add(devfn as usize))
        };
        if !d.is_null() {
            *d = dev;
        }
        devfn
    }
}
}

// ── `if (!d)` · pci.c:399-400 · chunk 1537 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!d)
//             return val_ones[size_log2];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1537 {
// Ported piece of pci_data_read: the null-device check that returns val_ones[size_log2].
use core::ffi::c_int;

// Opaque mirror of the C `PCIDevice *` pointee (never dereferenced here).
#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

// Equivalent of the C file-static `static const uint32_t val_ones[3]`.
static VAL_ONES: [u32; 3] = [0xff, 0xffff, 0xffff_ffff];

/// Returns nonzero when the C code must `return *out;` (i.e. the device is absent).
#[no_mangle]
pub extern "C" fn pci_data_read__part1537(
    d: *const PCIDevice,
    size_log2: c_int,
    out: *mut u32,
) -> c_int {
    if d.is_null() {
        // Mirror val_ones[size_log2] without any possible panic.
        let v = if (size_log2 as u32) < 3 {
            VAL_ONES[size_log2 as usize]
        } else {
            VAL_ONES[2]
        };
        if !out.is_null() {
            unsafe {
                *out = v;
            }
        }
        1
    } else {
        0
    }
}
}

// ── `config_addr = addr & 0xff; return pci_device_config_read(d, config_addr, size_log2);` · pci.c:401-402 · chunk 1538 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     config_addr = addr & 0xff;
//         return pci_device_config_read(d, config_addr, size_log2);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1538 {
// Port of the tail of pci_data_read(): config address masking plus
// pci_device_config_read() (a static C function, so its logic is inlined here).

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_void,
    pub io_regions: [PCIIORegion; 7],
}

extern "C" {
    fn abort() -> !;
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

#[no_mangle]
pub unsafe extern "C" fn pci_data_read__part1538(
    d: *mut PCIDevice,
    addr: u32,
    size_log2: c_int,
) -> u32 {
    let config_addr = addr & 0xff;
    let base: *const u8 = if d.is_null() {
        core::ptr::null()
    } else {
        (*d).config.as_ptr()
    };
    let p = base.add(config_addr as usize);
    match size_log2 {
        0 => *p as u32,
        1 => {
            if config_addr <= 0xfe {
                get_le16(p) as u32
            } else {
                *p as u32
            }
        }
        2 => get_le32(p),
        _ => abort(),
    }
}
}

// ── `return phys_mem_get_ram_ptr(d->bus->mem_map, addr, is_rw);` · pci.c:409-409 · chunk 1540 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return phys_mem_get_ram_ptr(d->bus->mem_map, addr, is_rw);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1540 {
// Port of pci_device_get_dma_ptr's body: return phys_mem_get_ram_ptr(d->bus->mem_map, addr, is_rw);
use core::ffi::{c_char, c_int, c_void};

// Opaque in this translation unit (defined elsewhere).
#[repr(C)]
pub struct PhysMemoryMap {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

extern "C" {
    fn phys_mem_get_ram_ptr(
        map: *mut PhysMemoryMap,
        paddr: u64,
        is_rw: c_int,
    ) -> *mut u8;
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_get_dma_ptr__part1540(
    d: *mut PCIDevice,
    addr: u64,
    is_rw: c_int,
) -> *mut u8 {
    let mem_map = (*(*d).bus).mem_map;
    phys_mem_get_ram_ptr(mem_map, addr, is_rw)
}
}

// ── `d->config[addr] = val;` · pci.c:414-414 · chunk 1542 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     d->config[addr] = val;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1542 {
// Port of the PCI config-byte store from pci_device_set_config8 (pci.c:414).
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

pub type SetIRQFunc = Option<extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int)>;
pub type PCIBarSetFunc =
    Option<extern "C" fn(opaque: *mut c_void, bar_num: c_int, addr: u32, enabled: u8)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: PCIBarSetFunc,
}

// Opaque: only ever handled through pointers.
#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub extern "C" fn pci_device_set_config8__part1542(d: *mut PCIDevice, addr: u8, val: u8) {
    unsafe {
        let cfg = core::ptr::addr_of_mut!((*d).config) as *mut u8;
        core::ptr::write(cfg.add(addr as usize), val);
    }
}
}

// ── `put_le16(&d->config[addr], val);` · pci.c:419-419 · chunk 1544 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_le16(&d->config[addr], val);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1544 {
// Port of the inlined put_le16(&d->config[addr], val) store from
// pci_device_set_config16. put_le16 is a static inline helper with no linker
// symbol, so its logic is reproduced here directly.

#[no_mangle]
pub unsafe extern "C" fn pci_device_set_config16__part1544(ptr: *mut u8, v: u16) {
    if ptr.is_null() {
        return;
    }
    core::ptr::write(ptr, v as u8);
    core::ptr::write(ptr.add(1), (v >> 8) as u8);
}
}

// ── `return d->devfn;` · pci.c:424-424 · chunk 1546 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     return d->devfn;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1546 {
// Rust port of the TARGET `return d->devfn;` inside pci_device_get_devfn.
use core::ffi::{c_char, c_int, c_uint, c_void};

/// Opaque mirror of `typedef void SetIRQFunc(void *opaque, int irq_num, int level);`
pub type SetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;

/// Opaque mirror of `typedef void PCIBarSetFunc(void *opaque, int bar_num, uint32_t addr, BOOL enabled);`
pub type PCIBarSetFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, u32, c_int)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: PCIBarSetFunc,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub unsafe extern "C" fn pci_device_get_devfn__part1546(d: *mut PCIDevice) -> c_int {
    let _ = c_uint::MIN; // keep imports exact; no runtime effect
    if d.is_null() {
        // C would fault here too; avoid UB in Rust by returning 0 rather than dereferencing null.
        return 0;
    }
    (*d).devfn as c_int
}
}

// ── `int offset; offset = d->next_cap_offset;` · pci.c:430-432 · chunk 1548 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int offset;
//     
//         offset = d->next_cap_offset;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1548 {
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub extern "C" fn pci_add_capability__part1548(d: *mut PCIDevice) -> c_int {
    unsafe { (*d).next_cap_offset as c_int }
}
}

// ── `if (offset + size) > 256)` · pci.c:433-434 · chunk 1549 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((offset + size) > 256)
//             return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1549 {
// Port of the capability-offset bounds check inside pci_add_capability.
// Returns -1 when the capability would not fit (caller returns -1),
// 0 when the caller should continue.

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn pci_add_capability__part1549(offset: c_int, size: c_int) -> c_int {
    if offset.wrapping_add(size) > 256 {
        return -1;
    }
    0
}
}

// ── `d->next_cap_offset += size; d->config[0x06] |= (1 << 4); memcpy(d->config + offset, buf, size);…` · pci.c:435-440 · chunk 1550 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     d->next_cap_offset += size;
//         d->config[0x06] |= (1 << 4);
//         memcpy(d->config + offset, buf, size);
//         d->config[offset + 1] = d->config[0x34];
//         d->config[0x34] = offset;
//         return offset;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1550 {
// Rust port of the tail of pci_add_capability (pci.c lines 435-440).
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct PCIBus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn pci_add_capability__part1550(
    d: *mut PCIDevice,
    buf: *const u8,
    size: c_int,
    offset: c_int,
) -> c_int {
    unsafe {
        (*d).next_cap_offset = (*d).next_cap_offset.wrapping_add(size as u8);

        let cfg: *mut u8 = (*d).config.as_mut_ptr();
        *cfg.add(0x06) = *cfg.add(0x06) | 16u8;

        memcpy(
            cfg.add(offset as usize) as *mut c_void,
            buf as *const c_void,
            size as usize,
        );

        *cfg.add(offset as usize + 1) = *cfg.add(0x34);
        *cfg.add(0x34) = offset as u8;

        offset
    }
}
}

// ── `I440FXState *s = opaque; s->config_reg = data;` · pci.c:457-458 · chunk 1553 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     I440FXState *s = opaque;
//         s->config_reg = data;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1553 {
use core::ffi::{c_int, c_void};

#[repr(C)]
struct I440FXState {
    pci_bus: *mut c_void,
    pci_dev: *mut c_void,
    piix3_dev: *mut c_void,
    config_reg: u32,
    pic_irq_state: [u8; 16],
    pic_irqs: *mut c_void,
}

#[no_mangle]
pub extern "C" fn i440fx_write_addr__part1553(opaque: *mut c_void, data: u32) {
    let s = opaque as *mut I440FXState;
    unsafe {
        (*s).config_reg = data;
    }
}

// keep c_int referenced so unused import lints never matter
#[allow(dead_code)]
fn _unused(_x: c_int) {}
}

// ── `I440FXState *s = opaque; return s->config_reg;` · pci.c:463-464 · chunk 1555 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     I440FXState *s = opaque;
//         return s->config_reg;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1555 {
// Port of the tail of i440fx_read_addr: fetch s->config_reg.
use core::ffi::{c_void, c_uint};

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut c_void,
    pub pci_dev: *mut c_void,
    pub piix3_dev: *mut c_void,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut c_void,
}

#[no_mangle]
pub extern "C" fn i440fx_read_addr__part1555(opaque: *mut c_void) -> c_uint {
    if opaque.is_null() {
        // C would fault here as well; avoid UB by returning 0 only in that
        // impossible case.
        return 0;
    }
    let s = opaque as *const I440FXState;
    unsafe { (*s).config_reg }
}
}

// ── `if (s->config_reg & 0x80000000)` · pci.c:471-479 · chunk 1557 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->config_reg & 0x80000000) {
//             if (size_log2 == 2) {
//     
//     
//                 pci_data_write(s->pci_bus, s->config_reg & ~3, data, size_log2);
//             } else {
//                 pci_data_write(s->pci_bus, s->config_reg | offset, data, size_log2);
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1557 {
// Ported piece of i440fx_write_data: the config-space write dispatch.
#![allow(non_camel_case_types, dead_code, unused_mut, unused_variables)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

pub type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
pub type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut u8,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

#[inline]
unsafe fn put_le32(ptr: *mut u8, v: u32) {
    *ptr = v as u8;
    *ptr.add(1) = (v >> 8) as u8;
    *ptr.add(2) = (v >> 16) as u8;
    *ptr.add(3) = (v >> 24) as u8;
}

unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let mut cmd: c_int;
    let mut i: c_int;
    let mut offset: c_int;
    let mut new_addr: u32;
    let mut new_enabled: c_int;
    let mut r: *mut PCIIORegion;

    cmd = get_le16((*d).config.as_ptr().add(0x04)) as c_int;

    i = 0;
    while i < 7 {
        r = (*d).io_regions.as_mut_ptr().add(i as usize);
        if i == 6 {
            offset = 0x30;
        } else {
            offset = 0x10 + i * 4;
        }
        new_addr = get_le32((*d).config.as_ptr().add(offset as usize));
        new_enabled = 0;
        if (*r).size != 0 {
            if ((*r).r#type & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1;
            } else {
                if (cmd & (1 << 1)) != 0 {
                    if i == 6 {
                        new_enabled = (new_addr & 1) as c_int;
                    } else {
                        new_enabled = 1;
                    }
                }
            }
        }
        if new_enabled != 0 {
            new_addr = get_le32((*d).config.as_ptr().add(offset as usize))
                & !((*r).size.wrapping_sub(1));
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, new_addr, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }
        i += 1;
    }
}

unsafe fn pci_write_bar(d: *mut PCIDevice, addr: u32, val: u32) -> c_int {
    let reg: c_int;
    let mut r: *mut PCIIORegion;

    if addr == 0x30 {
        reg = 6;
    } else {
        reg = (addr.wrapping_sub(0x10) >> 2) as c_int;
    }

    r = (*d).io_regions.as_mut_ptr().add(reg as usize);
    if (*r).size == 0 {
        return -1;
    }
    let v: u32;
    if reg == 6 {
        v = val & ((!((*r).size.wrapping_sub(1))) | 1);
    } else {
        v = (val & !((*r).size.wrapping_sub(1))) | ((*r).r#type as u32);
    }
    put_le32((*d).config.as_mut_ptr().add(addr as usize), v);
    pci_update_mappings(d);
    0
}

unsafe fn pci_device_config_write8(d: *mut PCIDevice, addr: u32, data: u32) {
    let can_write: c_int;

    if addr == 0x06 || addr == (0x06 + 1) {
        let p = (*d).config.as_mut_ptr().add(addr as usize);
        *p = (*p) & !(data as u8);
        return;
    }

    match (*d).config[0x0e] {
        0x00 | 0x80 => match addr {
            0x00 | 0x01 | 0x02 | 0x03 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0e | 0x10..=0x27
            | 0x30..=0x33 | 0x3d => can_write = 0,
            _ => can_write = 1,
        },
        _ => match addr {
            0x00 | 0x01 | 0x02 | 0x03 | 0x08 | 0x09 | 0x0a | 0x0b | 0x0e | 0x38..=0x3b
            | 0x3d => can_write = 0,
            _ => can_write = 1,
        },
    }
    if can_write != 0 {
        *((*d).config.as_mut_ptr().add(addr as usize)) = data as u8;
    }
}

unsafe fn pci_device_config_write(d: *mut PCIDevice, addr: u32, data: u32, size_log2: c_int) {
    let mut size: c_int;
    let mut i: c_int;
    let mut addr1: u32;

    if size_log2 == 2 && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30) {
        if pci_write_bar(d, addr, data) == 0 {
            return;
        }
    }
    size = 1 << size_log2;
    i = 0;
    while i < size {
        addr1 = addr.wrapping_add(i as u32);
        if addr1 <= 0xff {
            pci_device_config_write8(d, addr1, data.wrapping_shr((i * 8) as u32) & 0xff);
        }
        i += 1;
    }
    if 0x04 >= addr && 0x04 < addr.wrapping_add(size as u32) {
        pci_update_mappings(d);
    }
}

unsafe fn pci_data_write(s: *mut PCIBus, addr: u32, data: u32, size_log2: c_int) {
    let bus_num: c_int;
    let devfn: c_int;
    let config_addr: c_int;
    let d: *mut PCIDevice;

    bus_num = ((addr >> 16) & 0xff) as c_int;
    if bus_num != (*s).bus_num {
        return;
    }
    devfn = ((addr >> 8) & 0xff) as c_int;
    d = *(*s).device.as_ptr().add(devfn as usize);
    if d.is_null() {
        return;
    }
    config_addr = (addr & 0xff) as c_int;
    pci_device_config_write(d, config_addr as u32, data, size_log2);
}

#[no_mangle]
pub extern "C" fn i440fx_write_data__part1557(
    pci_bus: *mut PCIBus,
    config_reg: u32,
    offset: u32,
    data: u32,
    size_log2: c_int,
) {
    unsafe {
        if (config_reg & 0x80000000u32) != 0 {
            if size_log2 == 2 {
                pci_data_write(pci_bus, config_reg & !3u32, data, size_log2);
            } else {
                pci_data_write(pci_bus, config_reg | offset, data, size_log2);
            }
        }
    }
}
}

// ── `if (!(s->config_reg & 0x80000000)` · pci.c:485-486 · chunk 1559 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (!(s->config_reg & 0x80000000))
//             return val_ones[size_log2];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1559 {
// Ported piece of i440fx_read_data: the "config register not enabled" branch.
use core::ffi::{c_int, c_uint, c_void};

// Mirror of `struct I440FXState` from pci.c (only config_reg is read here).
#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut c_void,
    pub pci_dev: *mut c_void,
    pub piix3_dev: *mut c_void,
    pub config_reg: c_uint,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut c_void,
}

// Ported copy of `static const uint32_t val_ones[3]` (file-private in C).
static VAL_ONES: [c_uint; 3] = [0xff, 0xffff, 0xffffffff];

/// Returns, via `taken`, whether the C code must return `value`.
#[no_mangle]
pub unsafe extern "C" fn i440fx_read_data__part1559(
    opaque: *mut c_void,
    size_log2: c_int,
    taken: *mut c_int,
    value: *mut c_uint,
) {
    let s = opaque as *mut I440FXState;
    if (*s).config_reg & 0x80000000u32 == 0 {
        let idx = size_log2 as usize;
        let v = if idx < 3 { VAL_ONES[idx] } else { 0 };
        if !value.is_null() {
            *value = v;
        }
        if !taken.is_null() {
            *taken = 1;
        }
    } else if !taken.is_null() {
        *taken = 0;
    }
}
}

// ── `if (size_log2 == 2) … else` · pci.c:487-493 · chunk 1560 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (size_log2 == 2) {
//     
//     
//             return pci_data_read(s->pci_bus, s->config_reg & ~3, size_log2);
//         } else {
//             return pci_data_read(s->pci_bus, s->config_reg | offset, size_log2);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1560 {
// Port of the body of i440fx_read_data (pci.c lines 487-493).
// The original called the static functions pci_data_read / pci_device_config_read,
// which have no linker symbols, so their logic is mirrored here as private fns.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut PCIBus,
    pub pci_dev: *mut PCIDevice,
    pub piix3_dev: *mut PCIDevice,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut IRQSignal,
}

extern "C" {
    fn abort() -> !;
}

static VAL_ONES: [u32; 3] = [0xff, 0xffff, 0xffffffff];

#[inline]
fn val_ones(size_log2: c_int) -> u32 {
    match size_log2 {
        0 => VAL_ONES[0],
        1 => VAL_ONES[1],
        2 => VAL_ONES[2],
        _ => 0,
    }
}

unsafe fn pci_device_config_read(d: *mut PCIDevice, addr: u32, size_log2: c_int) -> u32 {
    let base = (*d).config.as_mut_ptr();
    let a = addr as usize;
    match size_log2 {
        0 => base.add(a).read() as u32,
        1 => {
            if addr <= 0xfe {
                (base.add(a).read() as u32) | ((base.add(a + 1).read() as u32) << 8)
            } else {
                base.add(a).read() as u32
            }
        }
        2 => {
            (base.add(a).read() as u32)
                | ((base.add(a + 1).read() as u32) << 8)
                | ((base.add(a + 2).read() as u32) << 16)
                | ((base.add(a + 3).read() as u32) << 24)
        }
        _ => abort(),
    }
}

unsafe fn pci_data_read(s: *mut PCIBus, addr: u32, size_log2: c_int) -> u32 {
    let bus_num = ((addr >> 16) & 0xff) as c_int;
    if bus_num != (*s).bus_num {
        return val_ones(size_log2);
    }
    let devfn = ((addr >> 8) & 0xff) as usize;
    let d = (*s).device[devfn];
    if d.is_null() {
        return val_ones(size_log2);
    }
    let config_addr = addr & 0xff;
    pci_device_config_read(d, config_addr, size_log2)
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_read_data__part1560(
    s: *mut I440FXState,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    let config_reg = (*s).config_reg;
    if size_log2 == 2 {
        pci_data_read((*s).pci_bus, config_reg & !3u32, size_log2)
    } else {
        pci_data_read((*s).pci_bus, config_reg | offset, size_log2)
    }
}
}

// ── `I440FXState *s = opaque; PCIDevice *hd = s->piix3_dev; int pic_irq; hd->config[0x60 + irq_num]…` · pci.c:498-505 · chunk 1562 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     I440FXState *s = opaque;
//         PCIDevice *hd = s->piix3_dev;
//         int pic_irq;
//     
//     
//     
//         hd->config[0x60 + irq_num] &= ~0x80;
//         pic_irq = hd->config[0x60 + irq_num];
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1562 {
// Ported piece of i440fx_set_irq: clears bit 0x80 of the PIIX3 config byte
// and returns the resulting (masked) PIC irq number.

#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<unsafe extern "C" fn(*mut c_void, c_int, u32, u8)>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut c_void,
    pub pci_dev: *mut PCIDevice,
    pub piix3_dev: *mut PCIDevice,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut IRQSignal,
}

#[no_mangle]
pub extern "C" fn i440fx_set_irq__part1562(opaque: *mut c_void, irq_num: c_int) -> c_int {
    unsafe {
        let s = opaque as *mut I440FXState;
        let hd = (*s).piix3_dev;
        let cfg = (*hd).config.as_mut_ptr();
        let idx = 0x60i32.wrapping_add(irq_num) as isize;
        // hd->config[0x60 + irq_num] &= ~0x80;
        let v = core::ptr::read(cfg.offset(idx));
        let newv = (v as c_int & !0x80i32) as u8;
        core::ptr::write(cfg.offset(idx), newv);
        // pic_irq = hd->config[0x60 + irq_num];
        newv as c_int
    }
}
}

// ── `if (pic_irq < 16)` · pci.c:506-512 · chunk 1563 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (pic_irq < 16) {
//             if (irq_level)
//                 s->pic_irq_state[pic_irq] |= 1 << irq_num;
//             else
//                 s->pic_irq_state[pic_irq] &= ~(1 << irq_num);
//             set_irq(&s->pic_irqs[pic_irq], (s->pic_irq_state[pic_irq] != 0));
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1563 {
// Port of the `if (pic_irq < 16) { ... }` piece of i440fx_set_irq.
use core::ffi::{c_int, c_void};

type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);

#[repr(C)]
struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
struct PCIIORegion {
    size: u32,
    type_: u8,
    enabled: u8,
    opaque: *mut c_void,
    bar_set: *mut c_void,
}

#[repr(C)]
struct PCIDevice {
    bus: *mut c_void,
    devfn: u8,
    irq: [IRQSignal; 4],
    config: [u8; 256],
    next_cap_offset: u8,
    name: *mut i8,
    io_regions: [PCIIORegion; 7],
}

#[repr(C)]
struct I440FXState {
    pci_bus: *mut c_void,
    pci_dev: *mut PCIDevice,
    piix3_dev: *mut PCIDevice,
    config_reg: u32,
    pic_irq_state: [u8; 16],
    pic_irqs: *mut IRQSignal,
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_set_irq__part1563(
    s: *mut I440FXState,
    irq_num: c_int,
    irq_level: c_int,
    pic_irq: c_int,
) {
    if pic_irq < 16 {
        let bit: u8 = 1i32.wrapping_shl(irq_num as u32) as u8;
        let state: *mut u8 = (*s).pic_irq_state.as_mut_ptr().offset(pic_irq as isize);
        if irq_level != 0 {
            *state = *state | bit;
        } else {
            *state = *state & !bit;
        }
        let level: c_int = if *state != 0 { 1 } else { 0 };
        let irq: *mut IRQSignal = (*s).pic_irqs.offset(pic_irq as isize);
        let f: Option<SetIRQFunc> = (*irq).set_irq;
        match f {
            Some(func) => func((*irq).opaque, (*irq).irq_num, level),
            None => {}
        }
    }
}
}

// ── `I440FXState *s; PCIBus *b; PCIDevice *d; int i; s = mallocz(sizeof(*s)); b = mallocz(sizeof(PCI…` · pci.c:519-531 · chunk 1565 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     I440FXState *s;
//         PCIBus *b;
//         PCIDevice *d;
//         int i;
//     
//         s = mallocz(sizeof(*s));
//     
//         b = mallocz(sizeof(PCIBus));
//         b->bus_num = 0;
//         b->mem_map = mem_map;
//         b->port_map = port_map;
//     
//         s->pic_irqs = pic_irqs;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1565 {
// Portion of i440fx_init(): allocate the I440FX state and the PCI bus,
// initialize the bus number and memory maps, and store the PIC IRQ array.

#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_int, c_void};

// Opaque types: only pointers to them appear here.
#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _private: [u8; 0],
}

pub type SetIRQFunc = Option<unsafe extern "C" fn(*mut c_void, c_int, c_int)>;

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: SetIRQFunc,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut PCIBus,
    pub pci_dev: *mut PCIDevice,
    pub piix3_dev: *mut PCIDevice,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut IRQSignal,
}

extern "C" {
    fn mallocz(size: usize) -> *mut c_void;
}

#[no_mangle]
pub extern "C" fn i440fx_init__part1565(
    s_out: *mut *mut I440FXState,
    b_out: *mut *mut PCIBus,
    mem_map: *mut PhysMemoryMap,
    port_map: *mut PhysMemoryMap,
    pic_irqs: *mut IRQSignal,
) {
    unsafe {
        let s = mallocz(core::mem::size_of::<I440FXState>()) as *mut I440FXState;

        let b = mallocz(core::mem::size_of::<PCIBus>()) as *mut PCIBus;
        if !b.is_null() {
            (*b).bus_num = 0;
            (*b).mem_map = mem_map;
            (*b).port_map = port_map;
        }

        if !s.is_null() {
            (*s).pic_irqs = pic_irqs;
        }

        *s_out = s;
        *b_out = b;
    }
}
}

// ── `for (i = 0; i < 4; i++)` · pci.c:532-534 · chunk 1566 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 4; i++) {
//             irq_init(&b->irq[i], i440fx_set_irq, s, i);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1566 {
// Port of the irq_init loop from i440fx_init (pci.c lines 532-534).
// i440fx_set_irq is a static C function with no linker symbol, so its logic is
// ported here as a private Rust function (it is what the loop installs).
use core::ffi::{c_int, c_void};

// void (*SetIRQFunc)(void *opaque, int irq_num, int level)
type SetIRQFunc = unsafe extern "C" fn(opaque: *mut c_void, irq_num: c_int, level: c_int);
// typedef void PCIBarSetFunc(void *opaque, int bar_num, uint32_t addr, BOOL enabled);
type PCIBarSetFunc =
    unsafe extern "C" fn(opaque: *mut c_void, bar_num: c_int, addr: u32, enabled: c_int);

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: Option<SetIRQFunc>,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PhysMemoryMap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut core::ffi::c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut PhysMemoryMap,
    pub port_map: *mut PhysMemoryMap,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut PCIBus,
    pub pci_dev: *mut PCIDevice,
    pub piix3_dev: *mut PCIDevice,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut IRQSignal,
}

extern "C" {
    fn irq_init(
        irq: *mut IRQSignal,
        set_irq: Option<SetIRQFunc>,
        opaque: *mut c_void,
        irq_num: c_int,
    );
}

// static inline void set_irq(IRQSignal *irq, int level)
#[inline]
unsafe fn set_irq(irq: *mut IRQSignal, level: c_int) {
    let f = (*irq).set_irq;
    if let Some(f) = f {
        f((*irq).opaque, (*irq).irq_num, level);
    }
}

// Port of the static C function i440fx_set_irq.
unsafe extern "C" fn rs_i440fx_set_irq(opaque: *mut c_void, irq_num: c_int, irq_level: c_int) {
    let s = opaque as *mut I440FXState;
    let hd = (*s).piix3_dev;
    let idx = 0x60usize.wrapping_add(irq_num as usize);
    (*hd).config[idx] &= !0x80u8;
    let pic_irq: c_int = (*hd).config[idx] as c_int;
    if pic_irq < 16 {
        let p = pic_irq as usize;
        if irq_level != 0 {
            (*s).pic_irq_state[p] |= (1u8) << (irq_num as u32);
        } else {
            (*s).pic_irq_state[p] &= !((1u8) << (irq_num as u32));
        }
        let level: c_int = if (*s).pic_irq_state[p] != 0 { 1 } else { 0 };
        set_irq((*s).pic_irqs.add(p), level);
    }
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_init__part1566(b: *mut PCIBus, s: *mut c_void) {
    let mut i: c_int = 0;
    while i < 4 {
        let base: *mut IRQSignal = core::ptr::addr_of_mut!((*b).irq) as *mut IRQSignal;
        irq_init(
            base.add(i as usize),
            Some(rs_i440fx_set_irq as SetIRQFunc),
            s,
            i,
        );
        i += 1;
    }
}
}

// ── `cpu_register_device(port_map, 0xcf8, 1, s, i440fx_read_addr, i440fx_write_addr, (1 << 2)); cpu_…` · pci.c:536-553 · chunk 1567 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     cpu_register_device(port_map, 0xcf8, 1, s, i440fx_read_addr, i440fx_write_addr,
//                             (1 << 2));
//         cpu_register_device(port_map, 0xcfc, 4, s, i440fx_read_data, i440fx_write_data,
//                             (1 << 0) | (1 << 1) | (1 << 2));
//         d = pci_register_device(b, "i440FX", 0, 0x8086, 0x1237, 0x02, 0x0600);
//         put_le16(&d->config[0x2c], 0x1af4);
//         put_le16(&d->config[0x2e], 0x1100);
//     
//         s->pci_dev = d;
//         s->pci_bus = b;
//     
//         s->piix3_dev = pci_register_device(b, "PIIX3", 8, 0x8086, 0x7000,
//                                            0x00, 0x0601);
//         pci_device_set_config8(s->piix3_dev, 0x0e, 0x80);
//     
//         *pbus = b;
//         *ppiix3_devfn = s->piix3_dev->devfn;
//         return s;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1567 {
// Port of i440fx_init lines 536-553: port device registration, i440FX / PIIX3
// PCI device creation and state wiring. The static C helpers reachable from the
// registered device callbacks are ported here as private Rust fns.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

type DeviceWriteFunc = unsafe extern "C" fn(*mut c_void, u32, u32, c_int);
type DeviceReadFunc = unsafe extern "C" fn(*mut c_void, u32, c_int) -> u32;
type SetIRQFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int);
type PCIBarSetFunc = unsafe extern "C" fn(*mut c_void, c_int, u32, c_int);

#[repr(C)]
pub struct IRQSignal {
    set_irq: Option<SetIRQFunc>,
    opaque: *mut c_void,
    irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    size: u32,
    type_: u8,
    enabled: u8,
    opaque: *mut c_void,
    bar_set: Option<PCIBarSetFunc>,
}

#[repr(C)]
pub struct PCIDevice {
    bus: *mut PCIBus,
    devfn: u8,
    irq: [IRQSignal; 4],
    config: [u8; 256],
    next_cap_offset: u8,
    name: *mut c_char,
    io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    bus_num: c_int,
    device: [*mut PCIDevice; 256],
    mem_map: *mut c_void,
    port_map: *mut c_void,
    irq_state: [[u32; 8]; 4],
    irq: [IRQSignal; 4],
}

#[repr(C)]
pub struct I440FXState {
    pci_bus: *mut PCIBus,
    pci_dev: *mut PCIDevice,
    piix3_dev: *mut PCIDevice,
    config_reg: u32,
    pic_irq_state: [u8; 16],
    pic_irqs: *mut IRQSignal,
}

extern "C" {
    fn cpu_register_device(
        s: *mut c_void,
        addr: u64,
        size: u64,
        opaque: *mut c_void,
        read_func: Option<DeviceReadFunc>,
        write_func: Option<DeviceWriteFunc>,
        devio_flags: c_int,
    ) -> *mut c_void;

    fn pci_register_device(
        b: *mut PCIBus,
        name: *const c_char,
        devfn: c_int,
        vendor_id: u16,
        device_id: u16,
        revision: u8,
        class_id: u16,
    ) -> *mut PCIDevice;

    fn pci_device_set_config8(d: *mut PCIDevice, addr: u8, val: u8);

    fn abort() -> !;
}

// ---- helpers that are `static inline` in the C translation unit ----

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

#[inline]
unsafe fn get_le16(ptr: *const u8) -> u16 {
    (*ptr as u16) | ((*ptr.add(1) as u16) << 8)
}

#[inline]
unsafe fn get_le32(ptr: *const u8) -> u32 {
    (*ptr as u32)
        | ((*ptr.add(1) as u32) << 8)
        | ((*ptr.add(2) as u32) << 16)
        | ((*ptr.add(3) as u32) << 24)
}

// ---- ported `static` C functions ----

const VAL_ONES: [u32; 3] = [0xff, 0xffff, 0xffff_ffff];

unsafe fn pci_device_config_read(d: *mut PCIDevice, addr: u32, size_log2: c_int) -> u32 {
    let p = (*d).config.as_mut_ptr().add(addr as usize);
    match size_log2 {
        0 => *p as u32,
        1 => {
            if addr <= 0xfe {
                get_le16(p) as u32
            } else {
                *p as u32
            }
        }
        2 => get_le32(p),
        _ => abort(),
    }
}

unsafe fn pci_device_config_write8(d: *mut PCIDevice, addr: u32, data: u32) {
    let cfg = (*d).config.as_mut_ptr();
    if addr == 0x06 || addr == 0x07 {
        let slot = cfg.add(addr as usize);
        *slot &= !(data as u8);
        return;
    }

    let can_write: c_int;
    match *cfg.add(0x0e) {
        0x00 | 0x80 => {
            let a = addr;
            if a <= 0x03
                || (a >= 0x08 && a <= 0x0b)
                || a == 0x0e
                || (a >= 0x10 && a <= 0x27)
                || (a >= 0x30 && a <= 0x33)
                || a == 0x3d
            {
                can_write = 0;
            } else {
                can_write = 1;
            }
        }
        _ => {
            let a = addr;
            if a <= 0x03
                || (a >= 0x08 && a <= 0x0b)
                || a == 0x0e
                || (a >= 0x38 && a <= 0x3b)
                || a == 0x3d
            {
                can_write = 0;
            } else {
                can_write = 1;
            }
        }
    }
    if can_write != 0 {
        *cfg.add(addr as usize) = data as u8;
    }
}

unsafe fn pci_update_mappings(d: *mut PCIDevice) {
    let cfg = (*d).config.as_mut_ptr();
    let cmd = get_le16(cfg.add(0x04)) as c_int;

    let mut i: c_int = 0;
    while i < 7 {
        let r = (*d).io_regions.as_mut_ptr().add(i as usize);
        let offset: u32 = if i == 6 { 0x30 } else { (0x10 + i * 4) as u32 };
        let mut new_addr: u32 = get_le32(cfg.add(offset as usize));
        let mut new_enabled: c_int = 0;
        if (*r).size != 0 {
            if ((*r).type_ & 0x01) != 0 && (cmd & (1 << 0)) != 0 {
                new_enabled = 1;
            } else if (cmd & (1 << 1)) != 0 {
                if i == 6 {
                    new_enabled = (new_addr & 1) as c_int;
                } else {
                    new_enabled = 1;
                }
            }
        }
        if new_enabled != 0 {
            new_addr = get_le32(cfg.add(offset as usize)) & !((*r).size.wrapping_sub(1));
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, new_addr, 1);
            }
            (*r).enabled = 1;
        } else if (*r).enabled != 0 {
            if let Some(f) = (*r).bar_set {
                f((*r).opaque, i, 0, 0);
            }
            (*r).enabled = 0;
        }
        i += 1;
    }
}

unsafe fn pci_write_bar(d: *mut PCIDevice, addr: u32, val: u32) -> c_int {
    let reg: c_int = if addr == 0x30 {
        6
    } else {
        (addr.wrapping_sub(0x10) >> 2) as c_int
    };

    let r = (*d).io_regions.as_mut_ptr().add(reg as usize);
    if (*r).size == 0 {
        return -1;
    }
    let new_val = if reg == 6 {
        val & (!((*r).size.wrapping_sub(1)) | 1)
    } else {
        (val & !((*r).size.wrapping_sub(1))) | (*r).type_ as u32
    };
    put_le32((*d).config.as_mut_ptr().add(addr as usize), new_val);
    pci_update_mappings(d);
    0
}

unsafe fn pci_device_config_write(d: *mut PCIDevice, addr: u32, data: u32, size_log2: c_int) {
    if size_log2 == 2
        && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30)
        && pci_write_bar(d, addr, data) == 0
    {
        return;
    }
    let size: c_int = 1i32.wrapping_shl(size_log2 as u32);
    let mut i: c_int = 0;
    while i < size {
        let addr1: u32 = addr.wrapping_add(i as u32);
        if addr1 <= 0xff {
            pci_device_config_write8(d, addr1, data.wrapping_shr((i * 8) as u32) & 0xff);
        }
        i += 1;
    }
    if 0x04 >= addr && 0x04 < addr.wrapping_add(size as u32) {
        pci_update_mappings(d);
    }
}

unsafe fn pci_data_write(s: *mut PCIBus, addr: u32, data: u32, size_log2: c_int) {
    let bus_num = ((addr >> 16) & 0xff) as c_int;
    if bus_num != (*s).bus_num {
        return;
    }
    let devfn = ((addr >> 8) & 0xff) as usize;
    let d = *(*s).device.as_mut_ptr().add(devfn);
    if d.is_null() {
        return;
    }
    let config_addr = addr & 0xff;
    pci_device_config_write(d, config_addr, data, size_log2);
}

unsafe fn pci_data_read(s: *mut PCIBus, addr: u32, size_log2: c_int) -> u32 {
    let ones = VAL_ONES.get(size_log2 as usize).copied().unwrap_or(0);
    let bus_num = ((addr >> 16) & 0xff) as c_int;
    if bus_num != (*s).bus_num {
        return ones;
    }
    let devfn = ((addr >> 8) & 0xff) as usize;
    let d = *(*s).device.as_mut_ptr().add(devfn);
    if d.is_null() {
        return ones;
    }
    let config_addr = addr & 0xff;
    pci_device_config_read(d, config_addr, size_log2)
}

unsafe extern "C" fn i440fx_write_addr(
    opaque: *mut c_void,
    _offset: u32,
    data: u32,
    _size_log2: c_int,
) {
    let s = opaque as *mut I440FXState;
    (*s).config_reg = data;
}

unsafe extern "C" fn i440fx_read_addr(
    opaque: *mut c_void,
    _offset: u32,
    _size_log2: c_int,
) -> u32 {
    let s = opaque as *mut I440FXState;
    (*s).config_reg
}

unsafe extern "C" fn i440fx_write_data(
    opaque: *mut c_void,
    offset: u32,
    data: u32,
    size_log2: c_int,
) {
    let s = opaque as *mut I440FXState;
    if ((*s).config_reg & 0x8000_0000) != 0 {
        if size_log2 == 2 {
            pci_data_write((*s).pci_bus, (*s).config_reg & !3, data, size_log2);
        } else {
            pci_data_write((*s).pci_bus, (*s).config_reg | offset, data, size_log2);
        }
    }
}

unsafe extern "C" fn i440fx_read_data(
    opaque: *mut c_void,
    offset: u32,
    size_log2: c_int,
) -> u32 {
    let s = opaque as *mut I440FXState;
    if ((*s).config_reg & 0x8000_0000) == 0 {
        return VAL_ONES.get(size_log2 as usize).copied().unwrap_or(0);
    }
    if size_log2 == 2 {
        pci_data_read((*s).pci_bus, (*s).config_reg & !3, size_log2)
    } else {
        pci_data_read((*s).pci_bus, (*s).config_reg | offset, size_log2)
    }
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_init__part1567(
    pbus: *mut *mut PCIBus,
    _ppiix3_devfn: *mut c_int,
    port_map: *mut c_void,
    s: *mut I440FXState,
    b: *mut PCIBus,
) -> c_int {
    cpu_register_device(
        port_map,
        0xcf8,
        1,
        s as *mut c_void,
        Some(i440fx_read_addr),
        Some(i440fx_write_addr),
        1 << 2,
    );
    cpu_register_device(
        port_map,
        0xcfc,
        4,
        s as *mut c_void,
        Some(i440fx_read_data),
        Some(i440fx_write_data),
        (1 << 0) | (1 << 1) | (1 << 2),
    );

    let d: *mut PCIDevice = pci_register_device(
        b,
        b"i440FX\0".as_ptr() as *const c_char,
        0,
        0x8086,
        0x1237,
        0x02,
        0x0600,
    );
    put_le16((*d).config.as_mut_ptr().add(0x2c), 0x1af4);
    put_le16((*d).config.as_mut_ptr().add(0x2e), 0x1100);

    (*s).pci_dev = d;
    (*s).pci_bus = b;

    let piix3: *mut PCIDevice = pci_register_device(
        b,
        b"PIIX3\0".as_ptr() as *const c_char,
        8,
        0x8086,
        0x7000,
        0x00,
        0x0601,
    );
    (*s).piix3_dev = piix3;
    pci_device_set_config8(piix3, 0x0e, 0x80);

    *pbus = b;
    (*piix3).devfn as c_int
}
}

// ── `PCIBus *b = s->pci_bus; PCIDevice *d, *hd; int irq_num, pic_irq, devfn, i; hd = s->piix3_dev; e…` · pci.c:560-568 · chunk 1569 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     PCIBus *b = s->pci_bus;
//         PCIDevice *d, *hd;
//         int irq_num, pic_irq, devfn, i;
//     
//     
//         hd = s->piix3_dev;
//     
//         elcr[0] = 0;
//         elcr[1] = 0;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1569 {
// Portion of i440fx_map_interrupts(): local declarations, hd lookup and
// ELCR initialisation extracted into its own exported function.

#[repr(C)]
pub struct PCIBus {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PCIDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct I440FXState {
    pub pci_bus: *mut PCIBus,
    pub pci_dev: *mut PCIDevice,
    pub piix3_dev: *mut PCIDevice,
    pub config_reg: u32,
    pub pic_irq_state: [u8; 16],
    pub pic_irqs: *mut core::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_map_interrupts__part1569(
    s: *mut I440FXState,
    elcr: *mut u8,
    out_b: *mut *mut PCIBus,
    out_hd: *mut *mut PCIDevice,
) {
    if s.is_null() {
        return;
    }
    let b: *mut PCIBus = (*s).pci_bus;
    let hd: *mut PCIDevice = (*s).piix3_dev;

    if !out_b.is_null() {
        *out_b = b;
    }
    if !out_hd.is_null() {
        *out_hd = hd;
    }

    if !elcr.is_null() {
        *elcr = 0;
        *elcr.add(1) = 0;
    }
}
}

// ── `for (i = 0; i < 4; i++)` · pci.c:569-573 · chunk 1570 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(i = 0; i < 4; i++) {
//             irq_num = pci_irqs[i];
//             hd->config[0x60 + i] = irq_num;
//             elcr[irq_num >> 3] |= (1 << (irq_num & 7));
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1570 {
// Port of the ELCR / PIIX3 PIRQ programming loop of i440fx_map_interrupts.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub type_: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut c_void,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[no_mangle]
pub extern "C" fn i440fx_map_interrupts__part1570(
    hd: *mut PCIDevice,
    elcr: *mut u8,
    pci_irqs: *const u8,
) {
    let mut i: c_int = 0;
    while i < 4 {
        let irq_num: c_int = unsafe { *pci_irqs.offset(i as isize) } as c_int;
        let bit: u8 = (1u32 << ((irq_num & 7) as u32)) as u8;
        unsafe {
            (*hd).config[(0x60 + i) as usize] = irq_num as u8;
            let cell = elcr.offset((irq_num >> 3) as isize);
            *cell = *cell | bit;
        }
        i += 1;
    }
}
}

// ── `for (devfn = 0; devfn < 256; devfn++)` · pci.c:575-587 · chunk 1571 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for(devfn = 0; devfn < 256; devfn++) {
//             d = b->device[devfn];
//             if (!d)
//                 continue;
//             if (d->config[0x3d]) {
//                 irq_num = 0;
//                 irq_num = bus_map_irq(d, irq_num);
//                 pic_irq = hd->config[0x60 + irq_num];
//                 if (pic_irq < 16) {
//                     d->config[0x3c] = pic_irq;
//                 }
//             }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk1571 {
// Port of the devfn scan loop of i440fx_map_interrupts (pci.c 575-587).

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct IRQSignal {
    pub set_irq: *mut c_void,
    pub opaque: *mut c_void,
    pub irq_num: c_int,
}

#[repr(C)]
pub struct PCIIORegion {
    pub size: u32,
    pub r#type: u8,
    pub enabled: u8,
    pub opaque: *mut c_void,
    pub bar_set: *mut c_void,
}

#[repr(C)]
pub struct PCIDevice {
    pub bus: *mut PCIBus,
    pub devfn: u8,
    pub irq: [IRQSignal; 4],
    pub config: [u8; 256],
    pub next_cap_offset: u8,
    pub name: *mut c_char,
    pub io_regions: [PCIIORegion; 7],
}

#[repr(C)]
pub struct PCIBus {
    pub bus_num: c_int,
    pub device: [*mut PCIDevice; 256],
    pub mem_map: *mut c_void,
    pub port_map: *mut c_void,
    pub irq_state: [[u32; 8]; 4],
    pub irq: [IRQSignal; 4],
}

// Replica of the static bus_map_irq() from pci.c (no linker symbol).
#[inline]
unsafe fn bus_map_irq(d: *mut PCIDevice, irq_num: c_int) -> c_int {
    let slot_addend = (((*d).devfn >> 3) as c_int) - 1;
    (irq_num + slot_addend) & 3
}

#[no_mangle]
pub unsafe extern "C" fn i440fx_map_interrupts__part1571(
    b: *mut PCIBus,
    hd: *mut PCIDevice,
) {
    if b.is_null() || hd.is_null() {
        return;
    }
    let devices: *mut *mut PCIDevice = (*b).device.as_mut_ptr();
    let hd_config: *mut u8 = (*hd).config.as_mut_ptr();

    let mut devfn: c_int = 0;
    while devfn < 256 {
        let d: *mut PCIDevice = *devices.offset(devfn as isize);
        if d.is_null() {
            devfn += 1;
            continue;
        }
        if (*d).config[0x3d] != 0 {
            let irq_num: c_int = bus_map_irq(d, 0);
            let pic_irq: c_int = *hd_config.offset(0x60 + irq_num as isize) as c_int;
            if pic_irq < 16 {
                (*d).config[0x3c] = pic_irq as u8;
            }
        }
        devfn += 1;
    }
}
}

