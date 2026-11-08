//! From `compress.c`.

// ── `detect_algorithm` · compress.c:37-46 · chunk 400 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     static algorithm_t detect_algorithm(const uint8_t *buf, int len)
//     {
//         if (len < 2 || buf[0] != 0x1f) return ALGORITHM_UNKNOWN;
//     
//         switch (buf[1]) {
//             case 0x08: return ALGORITHM_DEFLATE;
//             case 0x8b: return ALGORITHM_GZIP;
//             default: return ALGORITHM_UNKNOWN;
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk400 {
// Rust rewrite of the static C helper `detect_algorithm` from compress.c.
// The C enumeration is:
//   typedef enum { ALGORITHM_UNKNOWN, ALGORITHM_DEFLATE, ALGORITHM_GZIP } algorithm_t;
// which has an int-compatible ABI, so the return value is a c_int.
use core::ffi::c_int;

const ALGORITHM_UNKNOWN: c_int = 0;
const ALGORITHM_DEFLATE: c_int = 1;
const ALGORITHM_GZIP: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn detect_algorithm(buf: *const u8, len: c_int) -> c_int {
    if len < 2 {
        return ALGORITHM_UNKNOWN;
    }
    if buf.is_null() {
        // C would fault here; never dereference a null pointer.
        return ALGORITHM_UNKNOWN;
    }
    if *buf != 0x1f {
        return ALGORITHM_UNKNOWN;
    }

    match *buf.add(1) {
        0x08 => ALGORITHM_DEFLATE,
        0x8b => ALGORITHM_GZIP,
        _ => ALGORITHM_UNKNOWN,
    }
}
}

// ── `if (len < 2 || buf[0] != 0x1f)` · compress.c:39-39 · chunk 401 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (len < 2 || buf[0] != 0x1f) return ALGORITHM_UNKNOWN;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk401 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn detect_algorithm__part401(buf: *const u8, len: c_int) -> c_int {
    if len < 2 || unsafe { *buf } != 0x1f {
        return 1;
    }
    0
}
}

// ── `switch (buf[1])` · compress.c:41-45 · chunk 402 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     switch (buf[1]) {
//             case 0x08: return ALGORITHM_DEFLATE;
//             case 0x8b: return ALGORITHM_GZIP;
//             default: return ALGORITHM_UNKNOWN;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk402 {
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn detect_algorithm__part402(buf: *const core::ffi::c_uchar, len: c_int) -> c_int {
    let byte = unsafe { core::ptr::read_volatile(buf.offset(1)) };
    match byte {
        0x08 => 1,
        0x8b => 2,
        _ => 0,
    }
}
}

// ── `compress_detect_magic` · compress.c:48-51 · chunk 403 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     int compress_detect_magic(const uint8_t *buf, int len)
//     {
//         return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk403 {
// Port of compress.c: compress_detect_magic (and the static helper
// detect_algorithm it depends on, inlined here as a private Rust fn).

use core::ffi::c_int;

const ALGORITHM_UNKNOWN: c_int = 0;
const ALGORITHM_DEFLATE: c_int = 1;
const ALGORITHM_GZIP: c_int = 2;

// static algorithm_t detect_algorithm(const uint8_t *buf, int len)
fn detect_algorithm(buf: *const u8, len: c_int) -> c_int {
    if len < 2 || buf.is_null() {
        return ALGORITHM_UNKNOWN;
    }
    // SAFETY: caller guarantees at least `len` valid bytes when len >= 2.
    let b0 = unsafe { *buf };
    if b0 != 0x1f {
        return ALGORITHM_UNKNOWN;
    }
    // SAFETY: as above; len >= 2 so buf[1] is in bounds.
    let b1 = unsafe { *buf.add(1) };
    match b1 {
        0x08 => ALGORITHM_DEFLATE,
        0x8b => ALGORITHM_GZIP,
        _ => ALGORITHM_UNKNOWN,
    }
}

#[no_mangle]
pub extern "C" fn compress_detect_magic(buf: *const u8, len: c_int) -> c_int {
    (detect_algorithm(buf, len) != ALGORITHM_UNKNOWN) as c_int
}
}

// ── `return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;` · compress.c:50-50 · chunk 404 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk404 {
use core::ffi::c_uchar;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn compress_detect_magic__part404(buf: *const c_uchar, len: c_int) -> c_int {
    if len < 2 {
        return 0;
    }
    let first = unsafe { *buf };
    if first != 0x1f {
        return 0;
    }
    let second = unsafe { *buf.offset(1) };
    match second {
        0x08 | 0x8b => 1,
        _ => 0,
    }
}
}

// ── `decompress` · compress.c:53-87 · chunk 405 · deepseek/deepseek-v4.1-flash-1017 · 2 attempt(s) · Unvalidated
//
// Original C:
//     int decompress(const uint8_t *in_buf, int in_len,
//                    const uint8_t *out_buf, int out_len)
//     {
//         int windowBits;
//         switch (detect_algorithm(in_buf, in_len)) {
//             case ALGORITHM_UNKNOWN:
//                 return -1;
//             case ALGORITHM_DEFLATE:
//                 windowBits = -
//                              15
//                                       ;
//                 break;
//             case ALGORITHM_GZIP:
//                 windowBits = 
//                             15 
//                                       | 16;
//                 break;
//         }
//     
//         z_stream strm;
//         strm.zalloc = 
//                      0
//                            ;
//         strm.zfree = 
//                     0
//                           ;
//         strm.opaque = 
//                      0
//                            ;
//         strm.avail_in = 0;
//         strm.next_in = 
//                       0
//                             ;
//         if (
//            inflateInit2_((
//            &strm
//            ), (
//            windowBits
//            ), "1.3.2", (int)sizeof(z_stream)) 
//                                            != 
//                                               0
//                                                   ) return -1;
//     
//         strm.avail_in = in_len;
//         strm.next_in = (Bytef *)in_buf;
//         strm.avail_out = out_len;
//         strm.next_out = (Bytef *)out_buf;
//         if (inflate(&strm, 
//                           4
//                                   ) != 
//                                        1
//                                                    ) {
//             fprintf(
//                    stderr
//                          , "inflate() failed: %s\n", strm.msg);
//             inflateEnd(&strm);
//             return -1;
//         }
//     
//         return strm.total_out;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk405 {
// Rust rewrite of `decompress` from compress.c
//
// The original calls into zlib (inflateInit2/inflate/inflateEnd), which is not
// present at link time here, so the raw DEFLATE (RFC 1951) and gzip (RFC 1952)
// decoding that `inflate()` performed is implemented in this file.  The
// observable behaviour of the C function is preserved: the number of
// decompressed bytes is returned, or -1 (after an "inflate() failed" report on
// stderr) if the stream cannot be inflated to completion.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static stderr: *mut c_void;
}

const ERR_FMT: &[u8] = b"inflate() failed: %s\n\0";

const M_HEADER_CHECK: &[u8] = b"incorrect header check\0";
const M_UNKNOWN_METHOD: &[u8] = b"unknown compression method\0";
const M_HEADER_FLAGS: &[u8] = b"unknown header flags set\0";
const M_HEADER_CRC: &[u8] = b"header crc mismatch\0";
const M_DATA_CHECK: &[u8] = b"incorrect data check\0";
const M_LENGTH_CHECK: &[u8] = b"incorrect length check\0";
const M_BLOCK_TYPE: &[u8] = b"invalid block type\0";
const M_STORED_LENGTHS: &[u8] = b"invalid stored block lengths\0";
const M_TOO_MANY_LENS: &[u8] = b"too many length or distance symbols\0";
const M_CODE_LENGTHS: &[u8] = b"invalid code lengths set\0";
const M_BIT_LENGTH_REPEAT: &[u8] = b"invalid bit length repeat\0";
const M_LITLEN_SET: &[u8] = b"invalid literal/lengths set\0";
const M_DIST_SET: &[u8] = b"invalid distances set\0";
const M_MISSING_EOB: &[u8] = b"invalid code -- missing end-of-block\0";
const M_LITLEN_CODE: &[u8] = b"invalid literal/length code\0";
const M_DIST_CODE: &[u8] = b"invalid distance code\0";
const M_DIST_TOO_FAR: &[u8] = b"invalid distance too far back\0";

const MAXBITS: usize = 15;

/// `Fail::Msg` mirrors `strm.msg`; `Fail::NoMsg` mirrors zlib's Z_BUF_ERROR
/// (ran out of input or output space) where `strm.msg` stays NULL.
enum Fail {
    NoMsg,
    Msg(&'static [u8]),
}

fn detect_algorithm(buf: *const u8, len: c_int) -> c_int {
    // ALGORITHM_UNKNOWN = 0, ALGORITHM_DEFLATE = 1, ALGORITHM_GZIP = 2
    if len < 2 || buf.is_null() {
        return 0;
    }
    unsafe {
        if *buf != 0x1f {
            return 0;
        }
        match *buf.add(1) {
            0x08 => 1,
            0x8b => 2,
            _ => 0,
        }
    }
}

struct Bits<'a> {
    buf: &'a [u8],
    pos: usize,
    bitbuf: u32,
    bitcnt: u32,
}

impl<'a> Bits<'a> {
    fn new(buf: &'a [u8]) -> Bits<'a> {
        Bits {
            buf,
            pos: 0,
            bitbuf: 0,
            bitcnt: 0,
        }
    }

    /// LSB-first bit fetch, `need` <= 16.
    fn get(&mut self, need: u32) -> Result<u32, Fail> {
        let mut val = self.bitbuf;
        while self.bitcnt < need {
            let b = match self.buf.get(self.pos) {
                Some(&b) => b,
                None => return Err(Fail::NoMsg),
            };
            self.pos += 1;
            val |= (b as u32) << self.bitcnt;
            self.bitcnt += 8;
        }
        self.bitbuf = val >> need;
        self.bitcnt -= need;
        Ok(val & ((1u32 << need) - 1))
    }

    fn align(&mut self) {
        self.bitbuf = 0;
        self.bitcnt = 0;
    }

    fn get_byte(&mut self) -> Result<u8, Fail> {
        if self.bitcnt == 0 {
            match self.buf.get(self.pos) {
                Some(&b) => {
                    self.pos += 1;
                    Ok(b)
                }
                None => Err(Fail::NoMsg),
            }
        } else {
            Ok(self.get(8)? as u8)
        }
    }

    fn get_le32(&mut self) -> Result<u32, Fail> {
        let mut v: u32 = 0;
        let mut i: u32 = 0;
        while i < 4 {
            v |= (self.get_byte()? as u32) << (8 * i);
            i += 1;
        }
        Ok(v)
    }
}

struct Out {
    ptr: *mut u8,
    pos: usize,
    cap: usize,
}

impl Out {
    fn new(ptr: *mut u8, cap: usize) -> Out {
        Out { ptr, pos: 0, cap }
    }

    fn put(&mut self, b: u8) -> Result<(), Fail> {
        if self.pos >= self.cap {
            return Err(Fail::NoMsg);
        }
        unsafe {
            *self.ptr.add(self.pos) = b;
        }
        self.pos += 1;
        Ok(())
    }

    fn at(&self, i: usize) -> u8 {
        unsafe { *self.ptr.add(i) }
    }
}

struct Huffman {
    count: [u16; MAXBITS + 1],
    symbol: [u16; 288],
}

impl Huffman {
    fn new() -> Huffman {
        Huffman {
            count: [0; MAXBITS + 1],
            symbol: [0; 288],
        }
    }
}

fn construct(
    h: &mut Huffman,
    length: &[u8],
    n: usize,
    is_codes: bool,
    errmsg: &'static [u8],
) -> Result<(), Fail> {
    let mut i = 0;
    while i <= MAXBITS {
        h.count[i] = 0;
        i += 1;
    }

    let mut max_len = 0usize;
    let mut s = 0usize;
    while s < n {
        let l = match length.get(s) {
            Some(&l) => l as usize,
            None => return Err(Fail::Msg(errmsg)),
        };
        if l > MAXBITS {
            return Err(Fail::Msg(errmsg));
        }
        if let Some(c) = h.count.get_mut(l) {
            *c += 1;
        }
        if l > max_len {
            max_len = l;
        }
        s += 1;
    }

    if max_len == 0 {
        // No symbols at all: any decoding attempt will fail, like zlib's table
        // filled with invalid-code markers.
        return Ok(());
    }

    let mut left: i32 = 1;
    let mut len = 1usize;
    while len <= MAXBITS {
        left <<= 1;
        left -= h.count[len] as i32;
        if left < 0 {
            return Err(Fail::Msg(errmsg)); // over-subscribed
        }
        len += 1;
    }
    if left > 0 && (is_codes || max_len != 1) {
        return Err(Fail::Msg(errmsg)); // incomplete set
    }

    let mut offs = [0u16; MAXBITS + 2];
    offs[1] = 0;
    let mut l = 1usize;
    while l < MAXBITS {
        offs[l + 1] = offs[l] + h.count[l];
        l += 1;
    }

    let mut s = 0usize;
    while s < n {
        let l = match length.get(s) {
            Some(&l) => l as usize,
            None => return Err(Fail::Msg(errmsg)),
        };
        if l != 0 {
            let idx = match offs.get(l) {
                Some(&o) => o as usize,
                None => return Err(Fail::Msg(errmsg)),
            };
            match h.symbol.get_mut(idx) {
                Some(slot) => *slot = s as u16,
                None => return Err(Fail::Msg(errmsg)),
            }
            if let Some(o) = offs.get_mut(l) {
                *o += 1;
            }
        }
        s += 1;
    }
    Ok(())
}

fn decode(br: &mut Bits, h: &Huffman, errmsg: &'static [u8]) -> Result<u16, Fail> {
    let mut code: i32 = 0;
    let mut first: i32 = 0;
    let mut index: i32 = 0;
    let mut len = 1usize;
    while len <= MAXBITS {
        code |= br.get(1)? as i32;
        let count = match h.count.get(len) {
            Some(&c) => c as i32,
            None => return Err(Fail::Msg(errmsg)),
        };
        if code - first < count {
            let idx = index + (code - first);
            let sym = match h.symbol.get(idx as usize) {
                Some(&sym) => sym,
                None => return Err(Fail::Msg(errmsg)),
            };
            return Ok(sym);
        }
        index += count;
        first = (first + count) << 1;
        code <<= 1;
        len += 1;
    }
    Err(Fail::Msg(errmsg))
}

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

fn fixed_tables(lencode: &mut Huffman, distcode: &mut Huffman) -> Result<(), Fail> {
    let mut lens = [0u8; 288];
    let mut i = 0usize;
    while i < 144 {
        lens[i] = 8;
        i += 1;
    }
    while i < 256 {
        lens[i] = 9;
        i += 1;
    }
    while i < 280 {
        lens[i] = 7;
        i += 1;
    }
    while i < 288 {
        lens[i] = 8;
        i += 1;
    }
    construct(lencode, &lens, 288, false, M_LITLEN_SET)?;

    let mut dlens = [5u8; 32];
    construct(distcode, &mut dlens[..], 32, false, M_DIST_SET)?;
    Ok(())
}

fn codes(
    br: &mut Bits,
    out: &mut Out,
    lencode: &Huffman,
    distcode: &Huffman,
) -> Result<(), Fail> {
    loop {
        let symbol = decode(br, lencode, M_LITLEN_CODE)? as usize;
        if symbol < 256 {
            out.put(symbol as u8)?;
        } else if symbol == 256 {
            return Ok(());
        } else {
            let ym = symbol - 257;
            let base = match LEN_BASE.get(ym) {
                Some(&b) => b as usize,
                None => return Err(Fail::Msg(M_LITLEN_CODE)),
            };
            let lextra = match LEN_EXTRA.get(ym) {
                Some(&e) => e as u32,
                None => return Err(Fail::Msg(M_LITLEN_CODE)),
            };
            let len = base + br.get(lextra)? as usize;

            let dsym = decode(br, distcode, M_DIST_CODE)? as usize;
            let dbase = match DIST_BASE.get(dsym) {
                Some(&b) => b as usize,
                None => return Err(Fail::Msg(M_DIST_CODE)),
            };
            let dextra = match DIST_EXTRA.get(dsym) {
                Some(&e) => e as u32,
                None => return Err(Fail::Msg(M_DIST_CODE)),
            };
            let dist = dbase + br.get(dextra)? as usize;

            if dist > out.pos {
                return Err(Fail::Msg(M_DIST_TOO_FAR));
            }
            let mut k = 0usize;
            while k < len {
                let b = out.at(out.pos - dist);
                out.put(b)?;
                k += 1;
            }
        }
    }
}

fn stored(br: &mut Bits, out: &mut Out) -> Result<(), Fail> {
    br.align();
    let len = br.get(16)?;
    let nlen = br.get(16)?;
    if len != ((!nlen) & 0xffff) {
        return Err(Fail::Msg(M_STORED_LENGTHS));
    }
    let mut k: u32 = 0;
    while k < len {
        let b = br.get_byte()?;
        out.put(b)?;
        k += 1;
    }
    Ok(())
}

const ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

fn dynamic(br: &mut Bits, out: &mut Out) -> Result<(), Fail> {
    let nlen = br.get(5)? as usize + 257;
    let ndist = br.get(5)? as usize + 1;
    let ncode = br.get(4)? as usize + 4;
    if nlen > 286 || ndist > 30 {
        return Err(Fail::Msg(M_TOO_MANY_LENS));
    }

    let mut cl = [0u8; 19];
    let mut i = 0usize;
    while i < ncode {
        let v = br.get(3)? as u8;
        match ORDER.get(i) {
            Some(&o) => match cl.get_mut(o) {
                Some(slot) => *slot = v,
                None => return Err(Fail::Msg(M_CODE_LENGTHS)),
            },
            None => return Err(Fail::Msg(M_CODE_LENGTHS)),
        }
        i += 1;
    }

    let mut clcode = Huffman::new();
    construct(&mut clcode, &cl, 19, true, M_CODE_LENGTHS)?;

    let total = nlen + ndist;
    let mut lens = [0u8; 320];
    let mut have = 0usize;
    while have < total {
        let sym = decode(br, &clcode, M_CODE_LENGTHS)? as usize;
        if sym < 16 {
            match lens.get_mut(have) {
                Some(slot) => *slot = sym as u8,
                None => return Err(Fail::Msg(M_CODE_LENGTHS)),
            }
            have += 1;
        } else {
            let (val, rep) = match sym {
                16 => {
                    if have == 0 {
                        return Err(Fail::Msg(M_BIT_LENGTH_REPEAT));
                    }
                    let prev = match lens.get(have - 1) {
                        Some(&p) => p,
                        None => return Err(Fail::Msg(M_BIT_LENGTH_REPEAT)),
                    };
                    (prev, 3 + br.get(2)? as usize)
                }
                17 => (0u8, 3 + br.get(3)? as usize),
                _ => (0u8, 11 + br.get(7)? as usize),
            };
            if have + rep > total {
                return Err(Fail::Msg(M_BIT_LENGTH_REPEAT));
            }
            let mut k = 0usize;
            while k < rep {
                match lens.get_mut(have) {
                    Some(slot) => *slot = val,
                    None => return Err(Fail::Msg(M_BIT_LENGTH_REPEAT)),
                }
                have += 1;
                k += 1;
            }
        }
    }

    // there had better be an end-of-block code
    match lens.get(256) {
        Some(&0) => return Err(Fail::Msg(M_MISSING_EOB)),
        None => return Err(Fail::Msg(M_MISSING_EOB)),
        _ => {}
    }

    let lenslice = match lens.get(..nlen) {
        Some(s) => s,
        None => return Err(Fail::Msg(M_LITLEN_SET)),
    };
    let dslice = match lens.get(nlen..total) {
        Some(s) => s,
        None => return Err(Fail::Msg(M_DIST_SET)),
    };

    let mut lencode = Huffman::new();
    let mut distcode = Huffman::new();
    construct(&mut lencode, lenslice, nlen, false, M_LITLEN_SET)?;
    construct(&mut distcode, dslice, ndist, false, M_DIST_SET)?;

    codes(br, out, &lencode, &distcode)
}

fn inflate_blocks(br: &mut Bits, out: &mut Out) -> Result<(), Fail> {
    let mut lencode = Huffman::new();
    let mut distcode = Huffman::new();
    loop {
        let last = br.get(1)?;
        let btype = br.get(2)?;
        match btype {
            0 => stored(br, out)?,
            1 => {
                fixed_tables(&mut lencode, &mut distcode)?;
                codes(br, out, &lencode, &distcode)?;
            }
            2 => dynamic(br, out)?,
            _ => return Err(Fail::Msg(M_BLOCK_TYPE)),
        }
        if last != 0 {
            return Ok(());
        }
    }
}

fn gzip_header(br: &mut Bits) -> Result<(), Fail> {
    br.align();
    let b0 = br.get_byte()?;
    let b1 = br.get_byte()?;
    if b0 != 0x1f || b1 != 0x8b {
        return Err(Fail::Msg(M_HEADER_CHECK));
    }
    let cm = br.get_byte()?;
    let flg = br.get_byte()?;
    if cm != 8 {
        return Err(Fail::Msg(M_UNKNOWN_METHOD));
    }
    if flg & 0xe0 != 0 {
        return Err(Fail::Msg(M_HEADER_FLAGS));
    }
    // MTIME (4), XFL (1), OS (1)
    let mut i = 0;
    while i < 6 {
        br.get_byte()?;
        i += 1;
    }
    if flg & 4 != 0 {
        // FEXTRA
        let xlen = br.get_byte()? as usize | ((br.get_byte()? as usize) << 8);
        let mut k = 0usize;
        while k < xlen {
            br.get_byte()?;
            k += 1;
        }
    }
    if flg & 8 != 0 {
        // FNAME
        loop {
            if br.get_byte()? == 0 {
                break;
            }
        }
    }
    if flg & 16 != 0 {
        // FCOMMENT
        loop {
            if br.get_byte()? == 0 {
                break;
            }
        }
    }
    if flg & 2 != 0 {
        // FHCRC: skip the stored header CRC
        br.get_byte()?;
        br.get_byte()?;
    }
    Ok(())
}

fn crc32_of(out: &Out) -> u32 {
    let mut crc: u32 = 0xffff_ffff;
    let mut i = 0usize;
    while i < out.pos {
        crc ^= out.at(i) as u32;
        let mut k = 0;
        while k < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
            k += 1;
        }
        i += 1;
    }
    !crc
}

fn gzip_trailer(br: &mut Bits, out: &Out) -> Result<(), Fail> {
    br.align();
    let crc = br.get_le32()?;
    let isize = br.get_le32()?;
    if crc != crc32_of(out) {
        return Err(Fail::Msg(M_DATA_CHECK));
    }
    if isize != (out.pos as u32) {
        return Err(Fail::Msg(M_LENGTH_CHECK));
    }
    Ok(())
}

fn run(br: &mut Bits, out: &mut Out, gzip: bool) -> Result<(), Fail> {
    if gzip {
        gzip_header(br)?;
    }
    inflate_blocks(br, out)?;
    if gzip {
        gzip_trailer(br, out)?;
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn decompress(
    in_buf: *const u8,
    in_len: c_int,
    out_buf: *const u8,
    out_len: c_int,
) -> c_int {
    let gzip = match detect_algorithm(in_buf, in_len) {
        0 => return -1,
        1 => false, // windowBits = -MAX_WBITS: raw deflate
        _ => true,  // windowBits = MAX_WBITS | 16: gzip wrapper
    };

    if in_buf.is_null() {
        return -1;
    }
    let in_size = in_len as usize;
    let input: &[u8] = unsafe { core::slice::from_raw_parts(in_buf, in_size) };
    let cap = if out_len > 0 { out_len as usize } else { 0 };

    let mut br = Bits::new(input);
    let mut out = Out::new(out_buf as *mut u8, cap);

    match run(&mut br, &mut out, gzip) {
        Ok(()) => out.pos as c_int,
        Err(fail) => {
            let msg: *const c_char = match fail {
                Fail::Msg(m) => m.as_ptr() as *const c_char,
                Fail::NoMsg => core::ptr::null(),
            };
            unsafe {
                fprintf(stderr, ERR_FMT.as_ptr() as *const c_char, msg);
            }
            -1
        }
    }
}
}

// ── `switch (detect_algorithm(in_buf, in_len))` · compress.c:57-66 · chunk 406 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     switch (detect_algorithm(in_buf, in_len)) {
//             case ALGORITHM_UNKNOWN:
//                 return -1;
//             case ALGORITHM_DEFLATE:
//                 windowBits = -
//                              15
//                                       ;
//                 break;
//             case ALGORITHM_GZIP:
//                 windowBits = 
//                             15 
//                                       | 16;
//                 break;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk406 {
use core::ffi::c_int;
use core::ffi::c_uchar;

#[no_mangle]
pub extern "C" fn decompress__part406(in_buf: *const c_uchar, in_len: c_int) -> c_int {
    unsafe {
        if in_len < 2 || *in_buf != 0x1f {
            return -1;
        }
        let second = *in_buf.offset(1);
        match second {
            0x08 => -15,
            0x8b => 31,
            _ => -1,
        }
    }
}
}

// ── `z_stream strm; strm.zalloc = 0 ; strm.zfree = 0 ; strm.opaque = 0 ; strm.avail_in = 0; strm.nex…` · compress.c:68-73 · chunk 407 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     z_stream strm;
//         strm.zalloc = 
//                      0
//                            ;
//         strm.zfree = 
//                     0
//                           ;
//         strm.opaque = 
//                      0
//                            ;
//         strm.avail_in = 0;
//         strm.next_in = 
//                       0
//                             ;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk407 {
use core::ffi::{c_uchar, c_uint, c_ulong, c_char, c_void, c_int};
use core::ptr::null_mut;

#[repr(C)]
struct z_stream_s {
    next_in: *mut c_uchar,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_uchar,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: *mut c_void,
    zfree: *mut c_void,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

#[no_mangle]
pub extern "C" fn decompress__part407(strm: *mut z_stream_s) {
    unsafe {
        (*strm).zalloc = null_mut();
        (*strm).zfree = null_mut();
        (*strm).opaque = null_mut();
        (*strm).avail_in = 0;
        (*strm).next_in = null_mut();
    }
}
}

// ── `if (inflateInit2_(( &strm ), ( windowBits ), "1.3.2", (int)sizeof(z_stream)) != 0)` · compress.c:74-74 · chunk 408 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     if (
//            inflateInit2_((
//            &strm
//            ), (
//            windowBits
//            ), "1.3.2", (int)sizeof(z_stream)) 
//                                            != 
//                                               0
//                                                   ) return -1;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk408 {
use std::os::raw::{c_int, c_uint, c_ulong, c_char, c_void};
use std::ffi::CString;

#[repr(C)]
struct z_stream {
    next_in: *mut c_char,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_char,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: extern "C" fn(*mut c_void, c_uint, c_ulong) -> c_ulong,
    zfree: extern "C" fn(*mut c_void, *mut c_void),
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

extern "C" {
    fn inflateInit2_(
        strm: *mut z_stream,
        windowBits: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
}

#[no_mangle]
pub extern "C" fn decompress__part408(
    strm: *mut z_stream,
    windowBits: c_int,
) -> c_int {
    let version = CString::new("1.3.2").unwrap();
    unsafe { inflateInit2_(strm, windowBits, version.as_ptr(), std::mem::size_of::<z_stream>() as c_int) }
}
}

// ── `strm.avail_in = in_len; strm.next_in = (Bytef *)in_buf; strm.avail_out = out_len; strm.next_out…` · compress.c:76-79 · chunk 409 · nvidia/nemotron-3.5-lightning · 1 attempt(s) · Unvalidated
//
// Original C:
//     strm.avail_in = in_len;
//         strm.next_in = (Bytef *)in_buf;
//         strm.avail_out = out_len;
//         strm.next_out = (Bytef *)out_buf;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk409 {
use std::os::raw::{c_int, c_uint, c_ulong, c_char, c_void, c_uchar};

/// Mirror of C z_stream struct for FFI compatibility with zlib.
// Layout matches the C zlib z_stream struct with #[repr(C)].
#[repr(C)]
struct z_stream {
    next_in: *mut c_uchar,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_uchar,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: Option<extern "C" fn(c_uint, c_uint, c_void)>,
    zfree: Option<extern "C" fn(c_void, *mut c_void)>,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

#[no_mangle]
pub extern "C" fn decompress__part409(
    strm: *mut z_stream,
    in_buf: *const c_uchar,
    in_len: c_int,
    out_buf: *const c_uchar,
    out_len: c_int,
) {
    unsafe {
        if !strm.is_null() {
            (&mut *strm).avail_in = in_len as c_uint;
            (&mut *strm).next_in = in_buf as *mut c_uchar;
            (&mut *strm).avail_out = out_len as c_uint;
            (&mut *strm).next_out = out_buf as *mut c_uchar;
        }
    }
}
}

// ── `if (inflate(&strm, 4 ) != 1)` · compress.c:80-84 · chunk 410 · nvidia/nemotron-3.5-lightning · 2 attempt(s) · Unvalidated
//
// Original C:
//     if (inflate(&strm, 
//                           4
//                                   ) != 
//                                        1
//                                                    ) {
//             fprintf(
//                    stderr
//                          , "inflate() failed: %s\n", strm.msg);
//             inflateEnd(&strm);
//             return -1;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk410 {
use core::ffi::{c_int, c_uint, c_ulong, c_char, c_void};

// Mirror of C z_stream struct for ABI compatibility
#[repr(C)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut u8,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: *mut c_void,
    zfree: *mut c_void,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

extern "C" {
    pub fn inflate(strm: *mut z_stream_s, window_bits: c_int) -> c_int;
    pub fn inflateEnd(strm: *mut z_stream_s) -> c_int;
}

#[no_mangle]
pub extern "C" fn decompress__part410(strm: *mut z_stream_s) -> c_int {
    unsafe {
        if inflate(strm, 4) != 1 {
            inflateEnd(strm);
            return -1;
        }
    }
    0
}
}

// ── `return strm.total_out;` · compress.c:86-86 · chunk 411 · nvidia/nemotron-3.5-lightning · 3 attempt(s) · Unvalidated
//
// Original C:
//     return strm.total_out;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk411 {
use core::ffi::c_ulong;

#[no_mangle]
pub extern "C" fn decompress__part411(total_out: c_ulong) -> c_ulong {
    total_out
}
}

