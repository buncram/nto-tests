// (c) Copyright CrossBar, Inc. 2024.
//
// This documentation describes Open Hardware and is licensed under the
// [CERN-OHL-W-2.0].
//
// You may redistribute and modify this documentation under the terms of the
// [CERN-OHL- W-2.0 (http://ohwr.org/cernohl)]. This documentation is
// distributed WITHOUT ANY EXPRESS OR IMPLIED WARRANTY, INCLUDING OF
// MERCHANTABILITY, SATISFACTORY QUALITY AND FITNESS FOR A PARTICULAR PURPOSE.
// Please see the [CERN-OHL- W-2.0] for applicable conditions.

use utralib::generated::*;
use xous_pl230::*;

use crate::utils::*;
use crate::*;

const ALIGNMENT: usize = 32;

const QUICK_TESTS: usize = 8;
const CORNER_TESTS: usize = 12;
const CORNERS: usize = 4;
const CORNERS_TOTAL: usize = CORNER_TESTS * CORNERS * size_of::<u32>();

/// CoreuserId is the one-hot encoded user ID as read back by the STATUS_COREUSER register.
///
/// Note that the actual user ID written into the table mapping is a binary-encoded value
/// that represents the shift of the bit in the one-hot encoded field: so the hardware
/// takes care of going from dense to one-hot encoding on the ASID to coreuser table.
#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum CoreuserId {
    Boot0 = 1,
    Boot1 = 2,
    Fw0 = 4,
    Fw1 = 8,
}
impl Into<&'static str> for CoreuserId {
    fn into(self) -> &'static str {
        match self {
            Self::Boot0 => "Bt0",
            Self::Boot1 => "Bt1",
            Self::Fw0 => "Fw0",
            Self::Fw1 => "Fw1",
        }
    }
}
impl From<u32> for CoreuserId {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Boot0,
            2 => Self::Boot1,
            4 => Self::Fw0,
            8 => Self::Fw1,
            _ => unreachable!("invalid coreuser one-hot integer value"),
        }
    }
}

// Previously, the cases were hand-picked. Now we algorithmically generate
// them. But it's handy to keep this around for code reference.
const _READ_CASES: [(&'static str, usize); 15] = [
    ("keyselu0", KEYSEL_START + 0 * 32),
    ("keyselu1", KEYSEL_START + 1 * 32),
    ("keyselu2", KEYSEL_START + 2 * 32),
    ("keyselu3", KEYSEL_START + 3 * 32),
    ("keyselu0Rw", KEYSEL_START + (0 + 1 * 4) * 32),
    ("keyselu0rW", KEYSEL_START + (0 + 2 * 4) * 32),
    ("keyselu0RW", KEYSEL_START + (0 + 3 * 4) * 32),
    ("dataselu0", DATASEL_START + 0 * 32),
    ("dataselu1", DATASEL_START + 1 * 32),
    ("dataselu2", DATASEL_START + 2 * 32),
    ("dataselu3", DATASEL_START + 3 * 32),
    ("dataselu0Rw", DATASEL_START + (0 + 1 * 4) * 32),
    ("dataselu0rW", DATASEL_START + (0 + 2 * 4) * 32),
    ("dataselu0RW", DATASEL_START + (0 + 3 * 4) * 32),
    ("acram", ACRAM_START),
];
const LIFECYCLE_TESTS: usize =
    // READ_CASES.len() * 8 * 4 // each case has 8 words; checked across each of 4 coreuser modes
    16 * 8 * 4 * 2  // key slots each checked against 4 core user modes for read and write
    + 16 * 8 * 4 * 2; // data slots each checked against 4 core user modes for read and write

const TOTAL_TESTS: usize = QUICK_TESTS + CORNERS_TOTAL;
crate::impl_test!(RramTests, "RRAM", TOTAL_TESTS);

impl TestRunner for RramTests {
    fn run(&mut self) {
        self.passing_tests += rram_tests_early();
        self.passing_tests += rram_tests_corners(false);
    }
}

crate::impl_test!(RramDisturbTests, "RRAM Disturb", CORNERS_TOTAL);
/// A test runner just for verifying that RRAM was not disturbed.
impl TestRunner for RramDisturbTests {
    fn run(&mut self) { self.passing_tests += rram_tests_corners(true); }
}

pub fn rram_tests_corners(verify_only: bool) -> usize {
    let mut reram = Reram::new();
    let mut test = [0u32; CORNER_TESTS];
    let mut seed = 0xe692_b0f6;
    let mut passing = 0;
    let byte_offsets = [
        // aligned mid-write
        0x20_0000,
        // end cap
        0x40_0000 - CORNER_TESTS * size_of::<u32>(),
        // beginning
        0x0,
        // unaligned
        0x12_3455,
    ];

    for offset in byte_offsets {
        for d in test.iter_mut() {
            seed = crate::lfsr_next_u32(seed);
            *d = seed;
        }
        // safety: safe because u8 can align into the u32 storage, and all values can be represented.
        let data =
            unsafe { core::slice::from_raw_parts(test.as_ptr() as *const u8, test.len() * size_of::<u32>()) };
        // if just verifying that the write succeeded, skip the write
        if !verify_only {
            reram.write_slice(offset, data);
            cache_flush();
        }
        let rram_check = unsafe {
            core::slice::from_raw_parts(
                (offset + utralib::HW_RERAM_MEM) as *const u8,
                test.len() * size_of::<u32>(),
            )
        };
        for (i, (&s, &d)) in rram_check.iter().zip(data.iter()).enumerate() {
            if s == d {
                passing += 1;
            } else {
                crate::println!("Err: s {:x} -> d {:x} @ {:x}", s, d, offset + utralib::HW_RERAM_MEM + i);
            }
        }
    }

    crate::println!(
        "Corners {}: passing {} of {}",
        if verify_only { "reverify" } else { "write" },
        passing,
        CORNERS_TOTAL
    );
    passing
}

pub fn rram_tests_early() -> usize {
    let mut reram = Reram::new();
    let mut rbk = [0u32; QUICK_TESTS];
    let byte_offset = 0x10_0000;
    // readback
    {
        let rslice = &reram.read_slice()[byte_offset / core::mem::size_of::<u32>()
            ..byte_offset / core::mem::size_of::<u32>() + rbk.len()];
        rbk.copy_from_slice(rslice);
    }
    /*
    for (i, &r) in rbk.iter().enumerate() {
        crate::println!("{}: {:x}", i, r);
    }
    */
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    crate::println!("RRAM writing...");
    let test_data: [u32; QUICK_TESTS] = [
        0xeeee_eeee,
        0xbabe_600d,
        0x3141_5926,
        0x3333_3333,
        0xc0de_f00d,
        0xace0_bace,
        0x600d_c0de,
        0x1010_1010,
    ];

    unsafe {
        reram.write_u32_aligned(byte_offset, &test_data);
    }
    cache_flush();
    let mut passing = 0;
    {
        let rslice = &reram.read_slice()[byte_offset / core::mem::size_of::<u32>()
            ..byte_offset / core::mem::size_of::<u32>() + rbk.len()];
        for (i, (&s, &d)) in test_data.iter().zip(rslice.iter()).enumerate() {
            if s != d {
                crate::println!("@{}: w {:x} -> r {:x}", i, s, d);
            } else {
                passing += 1;
            }
        }
    }
    crate::println!("Base: passing {} of {}", passing, QUICK_TESTS);
    passing
}

pub fn rram_tests_late() {
    let mut uart = crate::debug::Uart {};
    let rram_base = 0x6000_0000 as *const u32;
    uart.tiny_write_str("0x6000_0000:\r");
    for i in 0..8 {
        report_api(unsafe { rram_base.add(i).read_volatile() });
    }
    uart.tiny_write_str("0x6020_0000:\r");
    for i in 0..8 {
        report_api(unsafe { rram_base.add(i + 0x20_0000 / core::mem::size_of::<u32>()).read_volatile() });
    }
    uart.tiny_write_str("0x603f8000:\r");
    for i in 0..8 {
        report_api(unsafe { rram_base.add(i + 0x3F_8000 / core::mem::size_of::<u32>()).read_volatile() });
    }
    /*
    let mut reram = Reram::new();
    let test_data: [u32; 8] = [
        0x1234_1234,
        0xfeed_face,
        0x3141_5926,
        0x1111_1111,
        0xc0de_f00d,
        0xace0_bace,
        0x600d_c0de,
        0x2222_2222
    ];
    unsafe {reram.write_u32_aligned(0, &test_data)};*/
    unsafe {
        let rram_ctl = 0x4000_0000 as *mut u32;
        let rram = 0x6000_0000 as *mut u32;
        rram.add(0).write_volatile(0x1234_1234);
        rram.add(1).write_volatile(0xfeed_face);
        rram.add(2).write_volatile(0x3141_5926);
        rram.add(3).write_volatile(0x1111_1111);
        rram.add(4).write_volatile(0xc0de_f00d);
        rram.add(5).write_volatile(0xace0_bace);
        rram.add(6).write_volatile(0x600d_c0de);
        rram.add(7).write_volatile(0x2222_2222);
        rram_ctl.add(0).write_volatile(2);
        rram.add(0).write_volatile(0x5200);
        rram.add(0).write_volatile(0x9528);
        rram_ctl.add(0).write_volatile(0);
    }

    uart.tiny_write_str("0x6020_0000:\r");
    for i in 0..8 {
        report_api(unsafe { rram_base.add(i + 0x20_0000 / core::mem::size_of::<u32>()).read_volatile() });
    }
    uart.tiny_write_str("0x6000_0000:\r");
    for i in 0..8 {
        report_api(unsafe { rram_base.add(i).read_volatile() });
    }
}

pub mod rrc {
    pub const RRC_LOAD_BUFFER: u32 = 0x5200;
    pub const RRC_WRITE_BUFFER: u32 = 0x9528;
    pub const RRC_CR: utralib::Register = utralib::Register::new(0, 0xffff_ffff);
    pub const RRC_CR_NORMAL: u32 = 0;
    pub const RRC_CR_POWERDOWN: u32 = 1;
    pub const RRC_CR_WRITE_DATA: u32 = 0;
    pub const RRC_CR_WRITE_CMD: u32 = 2;
    pub const RRC_FD: utralib::Register = utralib::Register::new(1, 0xffff_ffff);
    pub const RRC_SR: utralib::Register = utralib::Register::new(2, 0xffff_ffff);
    pub const HW_RRC_BASE: usize = 0x4000_0000;
}

#[repr(align(4))]
struct AlignedBuffer([u8; ALIGNMENT]);
impl AlignedBuffer {
    pub fn as_slice_u32(&self) -> &[u32] {
        // safety: this is safe because the #repr(align) ensures that our alignment is correct,
        // and the length of the internal data structure is set correctly by design. Furthermore,
        // all values in both the source and destination transmutation are representable and valid.
        unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *const u32, self.0.len() / 4) }
    }
}

pub struct Reram {
    pl230: xous_pl230::Pl230,
    csr: CSR<u32>,
    array: &'static mut [u32],
}

impl Reram {
    pub fn new() -> Self {
        Reram {
            csr: CSR::new(rrc::HW_RRC_BASE as *mut u32),
            pl230: xous_pl230::Pl230::new(),
            array: unsafe {
                core::slice::from_raw_parts_mut(
                    utralib::HW_RERAM_MEM as *mut u32,
                    utralib::HW_RERAM_MEM_LEN / core::mem::size_of::<u32>(),
                )
            },
        }
    }

    pub fn read_slice(&self) -> &[u32] { self.array }

    /// This is a crappy "unsafe" initial version that requires the write
    /// destination address to be aligned to a 256-bit boundary, and the data
    /// to be exactly 256 bits long.
    pub unsafe fn write_u32_aligned(&mut self, addr: usize, data: &[u32]) {
        assert!(addr % 0x20 == 0, "unaligned destination address!");
        assert!(data.len() % 8 == 0, "unaligned source data!");
        // crate::print!("@ {:x} > ", addr);
        for (outer, d) in data.chunks_exact(8).enumerate() {
            // write the data to the buffer
            for (inner, &datum) in d.iter().enumerate() {
                // crate::print!(" {:x}", datum);
                self.array
                    .as_mut_ptr()
                    .add(addr / core::mem::size_of::<u32>() + outer * 8 + inner)
                    .write_volatile(datum);
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
            // crate::println!("");

            self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_WRITE_CMD);
            self.array
                .as_mut_ptr()
                .add(addr / core::mem::size_of::<u32>() + outer * 8)
                .write_volatile(rrc::RRC_LOAD_BUFFER);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            self.array
                .as_mut_ptr()
                .add(addr / core::mem::size_of::<u32>() + outer * 8)
                .write_volatile(rrc::RRC_WRITE_BUFFER);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_NORMAL);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    /// This is a general unaligned write primitive for the RRAM that can handle any length
    /// slice and alignment of data.
    pub fn write_slice(&mut self, offset: usize, data: &[u8]) {
        let mut buffer = AlignedBuffer([0u8; ALIGNMENT]);

        // ragged start
        let start_len = ALIGNMENT - (offset % ALIGNMENT);
        if start_len != 0 {
            let start_offset = offset & !(ALIGNMENT - 1);
            let dest_slice = unsafe {
                core::slice::from_raw_parts(
                    (start_offset + utralib::HW_RERAM_MEM) as *const u8,
                    buffer.0.len(),
                )
            };
            // populate from old data first
            buffer.0.copy_from_slice(&dest_slice);
            buffer.0[offset % ALIGNMENT..].copy_from_slice(&data[..start_len]);
            // safe because alignment and buffer sizes are guaranteed
            unsafe {
                self.write_u32_aligned(start_offset, buffer.as_slice_u32());
            }
        }

        // aligned middle & end
        let mut cur_offset = offset + start_len;
        if data.len() - start_len > 0 {
            for chunk in data[start_len..].chunks(buffer.0.len()) {
                // full chunk
                if chunk.len() == buffer.0.len() {
                    buffer.0.copy_from_slice(&chunk);
                    // safe because alignment and buffer sizes are guaranteed
                    unsafe {
                        self.write_u32_aligned(cur_offset, &buffer.as_slice_u32());
                    }
                } else {
                    let dest_slice = unsafe {
                        core::slice::from_raw_parts(
                            (cur_offset + utralib::HW_RERAM_MEM) as *const u8,
                            buffer.0.len(),
                        )
                    };
                    // read in the destination full contents
                    buffer.0.copy_from_slice(&dest_slice);
                    // now overwrite the "ragged end"
                    buffer.0[..chunk.len()].copy_from_slice(&chunk);
                    // safe because alignment and buffer sizes are guaranteed
                    unsafe {
                        self.write_u32_aligned(cur_offset, &buffer.as_slice_u32());
                    }
                }
                cur_offset += chunk.len();
            }
        }
    }

    pub unsafe fn write_u32_aligned_dma(&mut self, _addr: usize, data: &[u32]) {
        //assert!(addr % 0x20 == 0, "unaligned destination address!");
        //assert!(data.len() % 8 == 0, "unaligned source data!");
        let init_ptr = utralib::HW_IFRAM1_MEM as *mut u32;
        for i in 0..(4 * 8 * 2) {
            unsafe { init_ptr.add(i).write_volatile(0) };
        }
        let cc_struct: &mut ControlChannels =
            unsafe { (utralib::HW_IFRAM0_MEM as *mut ControlChannels).as_mut().unwrap() };

        // read the status register
        self.pl230.csr.wfo(utra::pl230::CFG_MASTER_ENABLE, 1); // enable

        //cc_struct.channels[0].dst_end_ptr = (&self.array[addr / core::mem::size_of::<u32>() + data.len() -
        // 1]) as *const u32 as u32;
        cc_struct.channels[0].dst_end_ptr = 0x6010_003C;
        cc_struct.channels[0].src_end_ptr = (&data[data.len() - 1]) as *const u32 as u32;
        let mut cc = DmaChanControl(0);
        cc.set_src_size(DmaWidth::Word as u32);
        cc.set_src_inc(DmaWidth::Word as u32);
        cc.set_dst_size(DmaWidth::Word as u32);
        cc.set_dst_inc(DmaWidth::Word as u32);
        cc.set_r_power(ArbitrateAfter::Xfer1024 as u32);
        cc.set_n_minus_1(data.len() as u32 - 1);
        cc.set_cycle_ctrl(DmaCycleControl::AutoRequest as u32);
        cc_struct.channels[0].control = cc.0;

        self.pl230.csr.wo(utra::pl230::CTRLBASEPTR, cc_struct.channels.as_ptr() as u32);
        self.pl230.csr.wo(utra::pl230::CHNLREQMASKSET, 1);
        self.pl230.csr.wo(utra::pl230::CHNLENABLESET, 1);

        // this should kick off the DMA
        self.pl230.csr.wo(utra::pl230::CHNLSWREQUEST, 1);

        let mut timeout = 0;
        while (DmaChanControl(cc_struct.channels[0].control).cycle_ctrl() != 0) && timeout < 16 {
            // report_api("dma progress ", cc_struct.channels[0].control);
            report_api(unsafe { cc_struct.channels.as_ptr().read() }.control);
            timeout += 1;
        }

        /*
        // does this also need to be a DMA?
        self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_WRITE_CMD);
        self.array[addr / core::mem::size_of::<u32>()] = rrc::RRC_LOAD_BUFFER;
        self.array[addr / core::mem::size_of::<u32>()] = rrc::RRC_WRITE_BUFFER;
        self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_NORMAL); */
    }
}

#[inline(always)]
fn cache_flush() {
    unsafe {
        // cache flush
        #[rustfmt::skip]
        core::arch::asm!(
            ".word 0x500F",
            "nop",
            "nop",
            "nop",
            "nop",
            "fence",
            "nop",
            "nop",
            "nop",
            "nop",
        );
    }
}

pub const KEYSEL_START: usize = 0x603F_0000;
pub const DATASEL_START: usize = 0x603E_0000;
pub const ACRAM_START: usize = 0x603D_C000;
pub const ONEWAY_START: usize = 0x603D_A000;
pub const ONEWAY2_START: usize = 0x603D_B000;
pub const CODESEL_END: usize = 0x603D_A000;

crate::impl_test!(RramLifecycle, "RRAM Lifecycle", LIFECYCLE_TESTS);
impl TestRunner for RramLifecycle {
    fn run(&mut self) { self.passing_tests += rram_lockzones(); }
}

/*

# Key cases are striped as follows:
#   offset   user  rd   wr    wrena
#   0          1   ena  ena   false * special case always readable to all users
#   1          2   ena  ena   false * special case always readable to all users
#   2          4   ena  ena   false
#   3          8   ena  ena   false
#   4          1   dis  ena   false
#   5          2   dis  ena   false
#   6          4   dis  ena   false
#   7          8   dis  ena   false
#   8          1   ena  dis   false
#   9          2   ena  dis   false
#   10         4   ena  dis   false
#   11         8   ena  dis   false
#   12         1   dis  dis   false
#   13         2   dis  dis   false
#   14         4   dis  dis   false
#   15         8   dis  dis   false
# (repeat but with wrena = true, 16-31)

*/

#[derive(PartialEq, Eq)]
enum AccessRegion {
    Key,
    Data,
    Acram,
    Invalid,
}

fn case_readable(offset: usize) -> bool {
    let offset = (offset & 0xFFFF) / 32;
    (offset & 0b100) == 0
}
fn case_writeable(offset: usize) -> bool {
    let offset = (offset & 0xFFFF) / 32;
    (offset & 0b1000) == 0
}
fn case_wrena(offset: usize) -> bool {
    let offset = (offset & 0xFFFF) / 32;
    (offset & 0b1_0000) != 0
}
/// Returns the one-hot encoded version of the coreuser ID, so that the
/// value corresponds to what is read back in the STATUS_COREUSER register
fn case_user_id(offset: usize) -> u32 {
    let offset = (offset & 0xFFFF) / 32;
    (1 << (offset & 0b11)) as u32
}
fn case_region(offset: usize) -> AccessRegion {
    const ASSUMED_ACRAM_MATCH: usize = ACRAM_START & 0xFFFF_0000;
    match offset & 0xFFFF_0000 {
        KEYSEL_START => AccessRegion::Key,
        DATASEL_START => AccessRegion::Data,
        ASSUMED_ACRAM_MATCH => AccessRegion::Acram,
        _ => AccessRegion::Invalid,
    }
}
fn acram_encode(readable: bool, writeable: bool, onehot_user_id: CoreuserId, prog_only: bool) -> u32 {
    let mut ret = 0;
    ret |= if !readable { 1 } else { 0 };
    ret |= if !writeable { 2 } else { 0 };
    ret |= (onehot_user_id as u32) << 20;
    ret |= if prog_only { 1 << 24 } else { 0 };
    ret
}

fn data_default(offset: usize) -> u32 {
    let sentinels = [0xfaceface, 0xf00df00d, 0xd00dd00d, 0x600d600d];
    // mask out lower two bits
    let offset = offset & 0xFFFF_FFFC;
    if (offset & 0b100) != 0 {
        sentinels[(offset & 0b11_000) >> 3]
    } else {
        ((offset as u32) >> 5) & 0x00FF_FFFF
    }
}

fn key_default(offset: usize) -> u32 {
    let sentinels = [0xabcdef00, 0x12345678, 0x77778888, 0xccccdddd];
    // mask out lower two bits
    let offset = offset & 0xFFFF_FFFC;
    if (offset & 0b100) != 0 {
        sentinels[(offset & 0b11_000) >> 3]
    } else {
        ((offset as u32) >> 5) & 0x00FF_FFFF
    }
}
fn acram_default(offset: usize) -> u32 {
    let array_size = 2048 * 4;
    let access_offset = if offset - ACRAM_START < (2048 * 4) {
        // data base
        DATASEL_START + (offset & (array_size - 1)) * 8
    } else {
        // key base
        KEYSEL_START + (offset & (array_size - 1)) * 8
    };
    let mut value = 0;
    if !case_readable(access_offset) {
        value |= 1;
    }
    if !case_writeable(access_offset) {
        value |= 2;
    }
    value |= case_user_id(access_offset) << 20;
    if case_region(access_offset) == AccessRegion::Data {
        if case_wrena(access_offset) {
            value |= 1 << 24;
        }
    }
    value
}

const ROOT_PT_PA: usize = 0x6100_0000; // 1st level at base of sram
pub fn rram_lockzones() -> usize {
    let mut reram = Reram::new();
    let mut passing = 0;

    let mut coreuser: CSR<u32> = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
    let default = 0x3;
    // set to 0 so we can safely mask it later on
    coreuser.wo(utra::coreuser::USERVALUE, 0);
    coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
    // straightforward mapping: the decoder functions are user_to_asid() and asid_to_user()
    let trusted_asids = [(1, 0), (2, 1), (3, 2), (4, 3), (1, 0), (1, 0), (1, 0), (1, 0)];
    let asid_fields = [
        (utra::coreuser::MAP_LO_LUT0, utra::coreuser::USERVALUE_USER0),
        (utra::coreuser::MAP_LO_LUT1, utra::coreuser::USERVALUE_USER1),
        (utra::coreuser::MAP_LO_LUT2, utra::coreuser::USERVALUE_USER2),
        (utra::coreuser::MAP_LO_LUT3, utra::coreuser::USERVALUE_USER3),
        (utra::coreuser::MAP_HI_LUT4, utra::coreuser::USERVALUE_USER4),
        (utra::coreuser::MAP_HI_LUT5, utra::coreuser::USERVALUE_USER5),
        (utra::coreuser::MAP_HI_LUT6, utra::coreuser::USERVALUE_USER6),
        (utra::coreuser::MAP_HI_LUT7, utra::coreuser::USERVALUE_USER7),
    ];
    for (&(asid, value), (map_field, uservalue_field)) in trusted_asids.iter().zip(asid_fields) {
        coreuser.rmwf(map_field, asid);
        coreuser.rmwf(uservalue_field, value);
    }

    coreuser.wo(utra::coreuser::CONTROL, 0);
    coreuser.rmwf(utra::coreuser::CONTROL_ENABLE, 1);

    crate::println!("*** Running write cases on keyslots");
    for slot in (KEYSEL_START..KEYSEL_START + 32 * 16).step_by(32) {
        passing += write_testcase(slot, &mut reram, &mut coreuser)
    }

    crate::println!("*** Running write cases on dataslots");
    for slot in (DATASEL_START..DATASEL_START + 32 * 16).step_by(32) {
        passing += write_testcase(slot, &mut reram, &mut coreuser)
    }

    crate::println!("*** Running ACRAM update test case");
    // this corresponds to a default setting of (2)rw (user 2, read/write allowed)
    let base = DATASEL_START + 1 * 32;
    passing += acram_update_case(base, &mut reram, &mut coreuser);

    passing
}

/// This matches values programmed into trusted_asids[]. They are not fixed and are
/// software adjustable, so these mappings only apply to this test program.
fn user_to_asid(user_id: CoreuserId) -> usize {
    match user_id {
        CoreuserId::Boot0 => 1,
        CoreuserId::Boot1 => 2,
        CoreuserId::Fw0 => 3,
        CoreuserId::Fw1 => 4,
    }
}
/// This matches values programmed into trusted_asids[]. They are not fixed and are
/// software adjustable, so these mappings only apply to this test program.
fn asid_to_user(asid: usize) -> CoreuserId {
    match asid {
        1 => CoreuserId::Boot0,
        2 => CoreuserId::Boot1,
        3 => CoreuserId::Fw0,
        4 => CoreuserId::Fw1,
        _ => unimplemented!("ASID not valid for this test"),
    }
}

fn acram_update_case(base: usize, reram: &mut Reram, coreuser: &mut CSR<u32>) -> usize {
    let mut passing = 0;
    let init_user = case_user_id(base);
    let init_coreuser = CoreuserId::from(init_user);
    let init_coreuser_mnemomic: &'static str = init_coreuser.into();
    let init_readable = case_readable(base);
    let init_writeable = case_writeable(base);
    let init_state = base as u32 ^ 0x3141_5926;
    let mut state = lfsr_next_u32(init_state);

    // enable all error detection
    reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);
    cache_flush();

    let test_asid = user_to_asid(init_user.into());
    // confirm base permissions are correct by setting us to the expected user
    let satp: u32 = 0x8000_0000 | (test_asid as u32) << 22 | (ROOT_PT_PA as u32 >> 12);
    unsafe {
        core::arch::asm!(
            "csrw        satp, {satp_val}",
            "sfence.vma",
            satp_val = in(reg) satp,
        );
    }
    let user_id = coreuser.rf(utra::coreuser::STATUS_COREUSER) >> 4;
    if init_user != user_id {
        crate::println!("Can't run ACRAM test due to precondition failure: coreuser setting is buggy.");
        return 0;
    }

    // read the data in
    let secure_slice = unsafe { core::slice::from_raw_parts(base as *mut u32, 8) };
    let mut array_data = [0u32; 8];
    array_data.copy_from_slice(&secure_slice);
    let mut new_data = [0u32; 8];
    let mut check_data = [0u32; 8];
    // this saves the last valid write to our check spot
    let mut final_check_data = [0u32; 8];
    check_data.copy_from_slice(secure_slice);
    if init_readable {
        // if we got all 0's, assume we couldn't read when we *should* be able to
        if check_data.iter().all(|&x| x == 0) {
            crate::println!("Can't run ACRAM test due to precondition failure (unreadable)");
            return 0;
        }
    } else {
        if !check_data.iter().all(|&x| x == 0) {
            crate::println!("Can't run ACRAM test due to precondition failure (read protect fail)");
            return 0;
        }
    }
    if init_writeable {
        for d in new_data.iter_mut() {
            *d = state;
            state = lfsr_next_u32(state);
        }
        unsafe {
            reram.write_u32_aligned(base - utralib::HW_RERAM_MEM, &new_data);
        }
        cache_flush();
        // enable all error detection - must be re-enabled after the write operation
        reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);

        check_data.copy_from_slice(secure_slice);
        if check_data != new_data {
            crate::println!("Can't run ACRAM test due to precondition failure (write fail)");
            return 0;
        }
        // update the expected array data
        array_data.copy_from_slice(&new_data);
    } else {
        crate::println!(
            "Warning: can't determine if write perms were updated correctly, pick a different base case"
        );
    }

    crate::println!("Preconditions met, running ACRAM set test case on slot @ {:08x}!", base);
    // Precondition assumed: we are testing a bank that is readable, writeable; and we are at it's user ID.
    // Test loop:
    //   For each user ID
    //     For every combination of read/write
    //       Confirm that settings are respected.
    // Finally set to original value
    //   Confirm that settings are respected.

    // We can only patch a portion of ACRAM with a write, so we have to read back all the previous contents
    let mut old_acram = [0u32; 8];
    let mut new_acram = [0u32; 8];
    // resolve the ACRAM offset versus the current base address
    let acram_offset = match case_region(base) {
        AccessRegion::Key => ((base & 0xFFFF) / 0x20 + 2048) * size_of::<u32>() + ACRAM_START,
        AccessRegion::Data => ((base & 0xFFFF) / 0x20) * size_of::<u32>() + ACRAM_START,
        _ => unreachable!("Invalid base offset, neither key nor data."),
    };
    let acram_base = acram_offset & !(32 - 1);
    let acram_index = (acram_offset & (32 - 1)) / size_of::<u32>();
    let acram_slice = unsafe { core::slice::from_raw_parts(acram_base as *mut u32, 8) };
    crate::println!(
        "offset of interest: {:x}, acram base: {:x}, acram_index: {}",
        acram_offset,
        acram_base,
        acram_index
    );
    old_acram.copy_from_slice(acram_slice);
    new_acram.copy_from_slice(&old_acram);
    crate::println!("acram contents: {:x?}", old_acram);
    // check that ACRAM contents match our expected defaults
    let mut pass = true;
    for (index, &acram) in old_acram.iter().enumerate() {
        let expected = acram_default(acram_base + index * 4);
        if acram != expected {
            crate::println!("ACRAM default mismatch: (e){:08x}(g){:08x}", expected, acram);
            pass = false;
        }
    }
    if !pass {
        crate::println!("Can't run test because ACRAM defaults are incorrect");
        return 0;
    }

    for coreuser_as_asid in 1..=4 {
        let onehot_coreuser = asid_to_user(coreuser_as_asid);
        let coreuser_mnemonic: &'static str = onehot_coreuser.into();
        crate::println!("*** {}", coreuser_mnemonic);
        let rw_cases = [(false, false), (false, true), (true, false), (true, true)];

        for (readable, writeable) in rw_cases {
            let mnemonic = if readable && writeable {
                "rw"
            } else if !readable && writeable {
                "-w"
            } else if readable && !writeable {
                "r-"
            } else {
                "--"
            };
            let acram_code = acram_encode(readable, writeable, onehot_coreuser, false);
            new_acram[acram_index] = acram_code;
            crate::println!("  ACRAM({},{}): {:08x}", coreuser_mnemonic, mnemonic, acram_code);

            unsafe {
                reram.write_u32_aligned(acram_base - utralib::HW_RERAM_MEM, &new_acram);
            }
            cache_flush();
            // enable all error detection - must be re-enabled after the write operation
            reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);

            // now test that the new permissions were respected
            // user ID match
            // Read test
            check_data.fill(1);
            check_data.copy_from_slice(&secure_slice);
            if readable && (onehot_coreuser == init_coreuser) {
                if check_data != array_data {
                    crate::println!(
                        "  {} ({},{})r: {:x?}(e) {:x?}(g)",
                        init_coreuser_mnemomic,
                        coreuser_mnemonic,
                        mnemonic,
                        array_data,
                        check_data
                    );
                } else {
                    passing += 1;
                }
            } else {
                if !check_data.iter().all(|&x| x == 0) {
                    crate::println!(
                        "  {} ({},{})r: expected 0, got {:x?}",
                        init_coreuser_mnemomic,
                        coreuser_mnemonic,
                        mnemonic,
                        check_data
                    );
                } else {
                    passing += 1;
                }
            }
            // Write test
            for d in new_data.iter_mut() {
                *d = state;
                state = lfsr_next_u32(state);
            }
            unsafe {
                reram.write_u32_aligned(base - utralib::HW_RERAM_MEM, &new_data);
            }
            cache_flush();
            // enable all error detection - must be re-enabled after the write operation
            reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);
            check_data.fill(1);
            let mut displaced_array = [0u32; 8];
            displaced_array.copy_from_slice(&array_data);
            if writeable && (onehot_coreuser == init_coreuser) {
                array_data.copy_from_slice(&new_data);
                final_check_data.copy_from_slice(&new_data);
                crate::println!("copied final check {:x?}", final_check_data);
            }
            if readable {
                check_data.copy_from_slice(&secure_slice);
                if onehot_coreuser == init_coreuser {
                    if check_data != array_data {
                        crate::println!(
                            "  {} ({},{})w: {:x?}(e) {:x?}(g)",
                            init_coreuser_mnemomic,
                            coreuser_mnemonic,
                            mnemonic,
                            array_data,
                            check_data
                        );
                        crate::println!("      {:x?}(d)", displaced_array);
                    } else {
                        passing += 1;
                    }
                } else {
                    if check_data.iter().all(|&x| x == 0) {
                        passing += 1;
                    } else {
                        crate::println!(
                            "  {} ({},{})w: expected 0; {:x?}(g)",
                            init_coreuser_mnemomic,
                            coreuser_mnemonic,
                            mnemonic,
                            check_data
                        );
                        crate::println!("      {:x?}(d)", displaced_array);
                    }
                }
            } else {
                // assumed pass, we can't verify it
                passing += 1;
            }
        }
    }
    // reset to original
    new_acram[acram_index] = acram_default(acram_offset);
    unsafe {
        reram.write_u32_aligned(acram_base - utralib::HW_RERAM_MEM, &new_acram);
    }
    cache_flush();
    // enable all error detection - must be re-enabled after the write operation
    reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);
    // just a simple read to test that the permissions went back to the original state
    check_data.fill(1);
    // copy from secure_slice is necessary otherwise the values are optimized-in by the compiler
    check_data.copy_from_slice(&secure_slice);
    if init_readable {
        if secure_slice != check_data {
            crate::println!("closing access check: {:x?}(e), {:x?}(g)", check_data, secure_slice);
        } else {
            passing += 1;
        }
    } else {
        if !secure_slice.iter().all(|&x| x == 0) {
            crate::println!("closing access check: expected read deny, got {:x?}", secure_slice)
        } else {
            passing += 1;
        }
    }
    passing
}

// takes the address to test; returns number of passing words * 2 (once for read, once for write)
// attempts to write data based on an LFSR seeded with base
fn write_testcase(base: usize, reram: &mut Reram, coreuser: &mut CSR<u32>) -> usize {
    let mut check_array = [0u32; 8];
    // initialize the data array with the expected defaults - this mirrors what's in the rram
    let mut data_array = [0u32; 8];
    for (index, data) in data_array.iter_mut().enumerate() {
        let offset = base + index * size_of::<u32>();
        *data = match case_region(offset) {
            AccessRegion::Key => key_default(offset),
            AccessRegion::Data => data_default(offset),
            AccessRegion::Acram => acram_default(offset),
            _ => 0,
        };
    }
    // create human readable string for expected case outcome
    let mnemonic = if case_readable(base) && case_writeable(base) {
        "rw"
    } else if !case_readable(base) && case_writeable(base) {
        "-w"
    } else if case_readable(base) && !case_writeable(base) {
        "r-"
    } else {
        "--"
    };

    let hmac_ok = false;
    let mut passing = 0;
    let init_state = base as u32 ^ 0x3141_5926;
    let mut state = lfsr_next_u32(init_state);
    for test_asid in 1..=4 {
        let user_match =
            if asid_to_user(test_asid) as usize as u32 == case_user_id(base) { "+" } else { "-" };
        // set the ASID, which updates the coreuser value
        let satp: u32 = 0x8000_0000 | (test_asid as u32) << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
        // extract the current coreuser ID so we can know if we should be accessing the data or not
        let user_id = coreuser.rf(utra::coreuser::STATUS_COREUSER) >> 4;

        // enable all error detection
        reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);
        cache_flush();

        // 1. confirm read value is correct
        let slice = unsafe { core::slice::from_raw_parts(base as *mut u32, 8) };
        check_array.copy_from_slice(slice);
        let mut err_found = false;
        for (index, (&data, &in_rram)) in check_array.iter().zip(data_array.iter()).enumerate() {
            let offset = base + index * size_of::<u32>();
            let expected = match case_region(offset) {
                AccessRegion::Key => {
                    // first two key slots are always accessible by boot0 and boot1 (user_id 1 and 2)
                    if hmac_ok || (((offset & 0xFFFF) < 0x40) && (user_id <= 2)) {
                        if case_readable(offset) {
                            if case_user_id(offset) == user_id { in_rram } else { 0 }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                AccessRegion::Data => {
                    if case_readable(offset) {
                        if case_user_id(offset) == user_id { in_rram } else { 0 }
                    } else {
                        0
                    }
                }
                AccessRegion::Acram => {
                    if user_id == 0 || user_id == 1 {
                        in_rram
                    } else {
                        0
                    }
                }
                _ => 0,
            };
            if expected == data {
                if err_found {
                    crate::print!("{:08x} ", data);
                }
                passing += 1;
            } else {
                if !err_found {
                    crate::print!("RD({}){}{} err @{:x} -> ", test_asid, mnemonic, user_match, offset);
                    err_found = true;
                }
                crate::print!("(e){:08x}(g){:08x} ", expected, data);
            }
        }
        if err_found {
            crate::println!("");
        } else {
            crate::println!("RD({}){}{} pass @{:x}", test_asid, mnemonic, user_match, base);
        }

        // 2. Write something into the offset
        // has to write in 4's
        let mut testdata = [0u32; 8];
        for (index, d) in testdata.iter_mut().enumerate() {
            let offset = base + index * size_of::<u32>();
            *d = state;
            if case_writeable(offset) {
                data_array[index] = state;
            }
            state = lfsr_next_u32(state);
        }
        unsafe {
            reram.write_u32_aligned(base - utralib::HW_RERAM_MEM, &testdata);
        }
        cache_flush();
        // enable all error detection - must be re-enabled after the write operation
        reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);

        // now check readback value after write
        let slice = unsafe { core::slice::from_raw_parts(base as *mut u32, 8) };
        check_array.copy_from_slice(slice);
        err_found = false;
        for (index, (&data, &check)) in check_array.iter().zip(data_array.iter()).enumerate() {
            let offset = base + index * size_of::<u32>();
            let expected = match case_region(offset) {
                AccessRegion::Key => {
                    // first two slots are always readable & writeable
                    if hmac_ok || (((offset & 0xFFFF) < 0x40) && (user_id <= 2)) {
                        if case_readable(offset) {
                            if case_user_id(offset) == user_id { check } else { 0 }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                AccessRegion::Data => {
                    if case_readable(offset) {
                        if case_user_id(offset) == user_id { check } else { 0 }
                    } else {
                        0
                    }
                }
                AccessRegion::Acram => {
                    unreachable!("It's not valid to test ACRAM in this particular test");
                }
                _ => 0,
            };
            if expected == data {
                if err_found {
                    crate::print!("{:08x} ", data);
                }
                passing += 1;
            } else {
                if !err_found {
                    crate::print!("WR({}){}{} err @{:x} -> ", test_asid, mnemonic, user_match, offset);
                    err_found = true;
                }
                crate::print!("(e){:08x}(g){:08x} ", expected, data);
            }
            state = lfsr_next_u32(state);
        }
        if err_found {
            crate::println!("");
        } else {
            crate::println!("WR({}){}{} pass @{:x}", test_asid, mnemonic, user_match, base);
        }
    }

    passing
}
