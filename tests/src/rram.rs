use utralib::generated::*;
use xous_pl230::*;

use crate::utils::*;
use crate::*;

const TOTAL_TESTS: usize = 0;
crate::impl_test!(RramTests, "RRAM", TOTAL_TESTS);

impl TestRunner for RramTests {
    fn run(&mut self) {
        rram_tests_late();
        self.passing_tests += 1;
    }
}

pub fn rram_tests_early() {
    let mut rbk = [0u32; 16];
    let rram = 0x6010_0000 as *mut u32;
    let rram_ctl = 0x4000_0000 as *mut u32;
    crate::println!("RRAM early");
    report_api(0x3e3a_1770);
    unsafe {
        // readback
        for (i, r) in rbk.iter_mut().enumerate() {
            *r = rram.add(i).read_volatile();
        }
        for &r in rbk.iter() {
            report_api(r);
        }

        // this writes bytes in linear order
        // rram.add(0).write_volatile(0x1234_1234);
        // rram.add(1).write_volatile(0xfeed_face);
        // rram.add(2).write_volatile(0x3141_5926);
        // rram.add(3).write_volatile(0x1111_1111);
        // rram.add(4).write_volatile(0xc0de_f00d);
        // rram.add(5).write_volatile(0xace0_bace);
        // rram.add(6).write_volatile(0x600d_c0de);
        // rram.add(7).write_volatile(0x2222_2222);

        // this was an attempt to reverse/swap byte writing orders to
        // see how this impacts the RRAM receiver
        rram.add(1).write_volatile(0x2222_2222);
        rram.add(0).write_volatile(0x1111_1111);
        rram.add(2).write_volatile(0x3333_3333);
        rram.add(3).write_volatile(0x4444_4444);
        rram.add(4).write_volatile(0x5555_5555);
        rram.add(5).write_volatile(0x6666_6666);
        rram.add(7).write_volatile(0x8888_8888);
        rram.add(6).write_volatile(0x7777_7777);
        rram_ctl.add(0).write_volatile(2);
        rram.add(0).write_volatile(0x5200);
        rram.add(0).write_volatile(0x9528);
        rram_ctl.add(0).write_volatile(0);
    }
    report_api(0x3e3a_1771); // make some delay while RRAM processes
    let mut reram = Reram::new();
    let test_data: [u32; 8] = [
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
        reram.write_u32_aligned_dma(8, &test_data);
        rram_ctl.add(0).write_volatile(2);
        rram.add(0).write_volatile(0x5200);
        rram.add(0).write_volatile(0x9528);
        rram_ctl.add(0).write_volatile(0);

        core::arch::asm!(".word 0x500F",);
        report_api(0x3e3a_1772); // make some delay while RRAM processes
        // readback
        for (i, r) in rbk.iter_mut().enumerate() {
            *r = rram.add(i).read_volatile();
        }
        for (i, &r) in rbk.iter().enumerate() {
            if i == 8 {
                report_api(0x3e3a_1773);
            }
            report_api(r);
        }
    };
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

    /// This is a crappy "unsafe" initial version that requires the write
    /// destination address to be aligned to a 256-bit boundary, and the data
    /// to be exactly 256 bits long.
    pub unsafe fn write_u32_aligned(&mut self, addr: usize, data: &[u32]) {
        assert!(addr % 0x20 == 0, "unaligned destination address!");
        assert!(data.len() % 8 == 0, "unaligned source data!");
        for d in data.chunks_exact(8) {
            // write the data to the buffer
            for (&src, dst) in d.iter().zip(
                self.array[addr / core::mem::size_of::<u32>()..addr / core::mem::size_of::<u32>() + 8]
                    .iter_mut(),
            ) {
                *dst = src;
            }
            self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_WRITE_CMD);
            self.array[addr / core::mem::size_of::<u32>()] = rrc::RRC_LOAD_BUFFER;
            self.array[addr / core::mem::size_of::<u32>()] = rrc::RRC_WRITE_BUFFER;
            self.csr.wo(rrc::RRC_CR, rrc::RRC_CR_NORMAL);
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
