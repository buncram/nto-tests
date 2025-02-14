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

// lifecycle tests are not yet complete, because the code for handling lifecycles
// is not yet defined.
const CASES: [(&'static str, usize); 15] = [
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
const LIFECYCLE_TESTS: usize = CASES.len() * 8;

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
        crate::print!("@ {:x} > ", addr);
        for (outer, d) in data.chunks_exact(8).enumerate() {
            // write the data to the buffer
            for (inner, &datum) in d.iter().enumerate() {
                crate::print!(" {:x}", datum);
                self.array
                    .as_mut_ptr()
                    .add(addr / core::mem::size_of::<u32>() + outer * 8 + inner)
                    .write_volatile(datum);
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            }
            crate::println!("");

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
#   0          1   ena  ena   false
#   1          2   ena  ena   false
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

    let coreuser: CSR<u32> = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
    let mut default = 0x3;
    // set to 0 so we can safely mask it later on
    coreuser.wo(utra::coreuser::USERVALUE, 0);
    coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
    // straightforward mapping; creative mappings are tested elsewhere
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

    let hmac_ok = false;
    let mut check_array = [0u32; 8];

    for test_user in 1..=4 {
        // set the ASID, which updates the coreuser value
        let satp: u32 = 0x8000_0000 | test_user << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
        // extract the current coreuser ID so we can know if we should be accessing the data or not
        let user_id = coreuser.rf(utra::coreuser::STATUS_COREUSER) >> 4;
        crate::println!("*** Checking user_id: {:x}");
        for (_k, &(case, base)) in CASES.iter().enumerate() {
            // enable all error detection
            reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);

            // attempt to read data
            let slice = unsafe { core::slice::from_raw_parts(base as *mut u32, 8) };
            check_array.copy_from_slice(slice);
            crate::print!("{} base: @{:x} -> ", case, base);
            for (index, &data) in check_array.iter().enumerate() {
                let offset = base + index * size_of::<u32>();
                let expected = match case_region(offset) {
                    AccessRegion::Key => {
                        // first two slots are always readable
                        if hmac_ok || ((offset & 0xFFFF) < 0x40) {
                            if case_readable(offset) {
                                if case_user_id(offset) == user_id { key_default(offset) } else { 0 }
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }
                    AccessRegion::Data => {
                        if case_readable(offset) {
                            if case_user_id(offset) == user_id { data_default(offset) } else { 0 }
                        } else {
                            0
                        }
                    }
                    AccessRegion::Acram => acram_default(offset),
                    _ => 0,
                };
                if expected == data {
                    crate::print!("{:08x} ", data);
                    passing += 1;
                } else {
                    crate::print!("(e){:08x}(g){:08x} ", expected, data);
                }
            }
            crate::println!("");

            /*
            // has to write in 4's
            let mut testdata = [0u32; 8];
            for (j, d) in testdata.iter_mut().enumerate() {
                *d = (j + k * 0x100) as u32;
            }
            unsafe {
                reram.write_u32_aligned(base - utralib::HW_RERAM_MEM, &testdata);
            }
            cache_flush();
            // enable all error detection - must be re-enabled after the write operation
            reram.csr.wo(utra::rrc::SFR_RRCCR, 0b1111_1100_0000_0000);
            for (index, &data) in check_array.iter().enumerate() {
                let offset = base + index * size_of::<u32>();
                let expected = if case_writeable(offset) {
                    testdata[index]
                } else {
                    // default data modulo access controls
                    match case_region(offset) {
                        // key region is currently blocked because HMAC is not setup
                        AccessRegion::Key => 0,
                        AccessRegion::Data => {
                            if case_readable(offset) {
                                data_default(offset)
                            } else {
                                0
                            }
                        }
                        AccessRegion::Acram => acram_default(offset),
                        _ => 0,
                    }
                };
                if expected == data {
                    passing += 1;
                } else {
                    crate::println!("wr mismatch: want {:08x}, got {:08x}", expected, data);
                }
            } */
        }
    }

    // tuple of number passing, total cases
    passing
}
