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

const TOTAL_TESTS: usize = QUICK_TESTS + CORNERS_TOTAL;
crate::impl_test!(RramTests, "RRAM", TOTAL_TESTS);

impl TestRunner for RramTests {
    fn run(&mut self) {
        self.passing_tests += rram_tests_early();
        self.passing_tests += rram_tests_corners();
    }
}

pub fn rram_tests_corners() -> usize {
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
        reram.write_slice(offset, data);
        cache_flush();
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

    crate::println!("Corners: passing {} of {}", passing, CORNERS_TOTAL);
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
