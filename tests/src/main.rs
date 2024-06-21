#![no_std]
#![no_main]
#![allow(unreachable_code)] // allow debugging of failures to jump out of the bootloader

use core::convert::TryFrom;
use core::convert::TryInto;
use core::mem::size_of;

use utralib::generated::*;
use utralib::utra::sysctrl;

mod debug;
mod init;
mod irqs;
mod ramtests;
mod rram;
mod satp;
mod sce;
mod utils;

mod asm;

pub use init::*;
pub use ramtests::*;
pub use rram::*;
pub use sce::*;
pub use utils::*;

mod daric_generated;
// you know what's irritating? if this file is named apb_test, clippy complains because
// it's not a #test. wtf yo. not all tests are just for you, clippy!
#[cfg(feature = "apb-test")]
mod apb_check;
#[cfg(feature = "apb-test")]
use apb_check::apb_test;

/*
    Notes about printing:
      - the println! and write! macros are actually quite expensive in the context of a 32kiB ROM (~4k overhead??)
      - we are trying to get away with direct putc() and tiny_write_str() calls. looks weird for Rust, but it saves a few bytes
*/

#[cfg(target_os = "none")]
mod panic_handler {
    use core::panic::PanicInfo;

    use crate::debug;
    #[panic_handler]
    fn handle_panic(arg: &PanicInfo) -> ! {
        //crate::println!("{}", _arg);
        let mut uart = debug::Uart {};
        if let Some(s) = arg.payload().downcast_ref::<&str>() {
            uart.tiny_write_str(s);
        } else {
            uart.tiny_write_str("unspecified panic!\n\r");
        }
        loop {}
    }
}

pub fn xip_test() {
    report_api(0x61D0_0000);
    // a code snippet that adds 0x400 to the argument and returns
    let code = [0x4005_0513u32, 0x0000_8082u32];

    // shove it into the XIP region
    let xip_dest = unsafe { core::slice::from_raw_parts_mut(satp::XIP_VA as *mut u32, 2) };
    xip_dest.copy_from_slice(&code);

    // run the code
    let mut test_val: usize = 0x5555_0000;
    let mut expected: usize = test_val;
    for _ in 0..8 {
        test_val = crate::asm::jmp_remote(test_val, satp::XIP_VA);
        report_api(test_val as u32);
        expected += 0x0400;
        assert!(expected == test_val);
    }

    // prep a second region, a little bit further away to trigger a second access
    // self-modifying code is *not* supported on Vex
    const XIP_OFFSET: usize = 0;
    let xip_dest2 = unsafe { core::slice::from_raw_parts_mut((satp::XIP_VA + XIP_OFFSET) as *mut u32, 2) };
    let code2 = [0x0015_0513u32, 0x0000_8082u32];
    xip_dest2.copy_from_slice(&code2);
    // this forces a reload of the i-cache
    unsafe {
        core::arch::asm!("fence.i",);
    }

    // run the new code and see that it was updated?
    for _ in 0..8 {
        test_val = crate::asm::jmp_remote(test_val, satp::XIP_VA + XIP_OFFSET);
        report_api(test_val as u32);
        expected += 1;
        assert!(expected == test_val);
    }
    report_api(0x61D0_600D);
}

pub fn reset_ticktimer() {
    let mut tt = CSR::new(utra::ticktimer::HW_TICKTIMER_BASE as *mut u32);
    // tt.wo(utra::ticktimer::CLOCKS_PER_TICK, 160);
    tt.wo(utra::ticktimer::CLOCKS_PER_TICK, 369560); // based on 369.56MHz default clock
    tt.wfo(utra::ticktimer::CONTROL_RESET, 1);
    tt.wo(utra::ticktimer::CONTROL, 0);
}

pub fn snap_ticks(title: &str) {
    let tt = CSR::new(utra::ticktimer::HW_TICKTIMER_BASE as *mut u32);
    let mut uart = debug::Uart {};
    uart.tiny_write_str(title);
    uart.tiny_write_str(" time: ");
    uart.print_hex_word(tt.rf(utra::ticktimer::TIME0_TIME));
    // write!(uart, "{} time: {} ticks\n", title, elapsed).ok();
    uart.tiny_write_str(" ticks\r");
}

#[export_name = "rust_entry"]
pub unsafe extern "C" fn rust_entry(_unused1: *const usize, _unused2: u32) -> ! {
    let mut uart = debug::Uart {};

    unsafe {
        // this is MANDATORY for any chip stapbility in real silicon, as the initial
        // clocks are too unstable to do anything otherwise. However, for the simulation
        // environment, this can (should?) be dropped
        let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;
        daric_cgu.add(sysctrl::SFR_CGUSEL1.offset()).write_volatile(1); // 0: RC, 1: XTAL
        daric_cgu.add(sysctrl::SFR_CGUFSCR.offset()).write_volatile(48); // external crystal is 48MHz

        daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);

        let duart = utra::duart::HW_DUART_BASE as *mut u32;
        duart.add(utra::duart::SFR_CR.offset()).write_volatile(0);
        duart.add(utra::duart::SFR_ETUC.offset()).write_volatile(24);
        duart.add(utra::duart::SFR_CR.offset()).write_volatile(1);
    }
    // this block is mandatory in all cases to get clocks set into some consistent, expected mode
    unsafe {
        let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_0.offset()).write_volatile(0x7f7f);
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_1.offset()).write_volatile(0x7f7f);
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_2.offset()).write_volatile(0x3f3f);
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_3.offset()).write_volatile(0x1f1f);
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_4.offset()).write_volatile(0x0f0f);
        daric_cgu.add(utra::sysctrl::SFR_ACLKGR.offset()).write_volatile(0xFF);
        daric_cgu.add(utra::sysctrl::SFR_HCLKGR.offset()).write_volatile(0xFF);
        daric_cgu.add(utra::sysctrl::SFR_ICLKGR.offset()).write_volatile(0xFF);
        daric_cgu.add(utra::sysctrl::SFR_PCLKGR.offset()).write_volatile(0xFF);
        daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);

        let duart = utra::duart::HW_DUART_BASE as *mut u32;
        // enable DUART
        duart.add(utra::duart::SFR_CR.offset()).write_volatile(1);
    }
    // pre-boot establishment of ... anything working at all ...?
    uart.tiny_write_str("hello world?\r");

    #[cfg(feature = "sram-margin")]
    {
        // set SRAM delay to max - opens up timing margin as much a possible, supposedly?
        let sram_ctl = utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32;
        let waitcycles = 3;
        sram_ctl.add(utra::coresub_sramtrm::SFR_SRAM0.offset()).write_volatile(
            (sram_ctl.add(utra::coresub_sramtrm::SFR_SRAM0.offset()).read_volatile() & !0x18)
                | ((waitcycles << 3) & 0x18),
        );
        sram_ctl.add(utra::coresub_sramtrm::SFR_SRAM1.offset()).write_volatile(
            (sram_ctl.add(utra::coresub_sramtrm::SFR_SRAM1.offset()).read_volatile() & !0x18)
                | ((waitcycles << 3) & 0x18),
        );
    }
    #[cfg(feature = "rram-testing")]
    {
        let mut rbk = [0u32; 16];
        let rram = 0x6010_0000 as *mut u32;
        let rram_ctl = 0x4000_0000 as *mut u32;
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
    uart.tiny_write_str("hello world!\r");

    #[cfg(feature = "check-byte-strobes")]
    {
        let u8_test = crate::debug::duart::HW_DUART_BASE as *mut u8;
        let u16_test = crate::debug::duart::HW_DUART_BASE as *mut u16;

        // quick test to check byte and word write strobes on the
        unsafe {
            u8_test.write_volatile(0x31);
            u8_test.add(1).write_volatile(32);
            u8_test.add(2).write_volatile(33);
            u8_test.add(3).write_volatile(34);

            u16_test.write_volatile(0x44);
            u16_test.add(1).write_volatile(0x55);
        }
    }

    reset_ticktimer();
    snap_ticks("sysctrl: ipen ");

    uart.tiny_write_str("setting clocks\r");
    init_clock_asic(128_000_000);
    // early_init();
    uart.tiny_write_str("booting... 001\r");

    let mut report = CSR::new(utra::main::HW_MAIN_BASE as *mut u32);
    report_api(0x600dc0de);

    // report the measured reset value
    let resetvalue = CSR::new(utra::resetvalue::HW_RESETVALUE_BASE as *mut u32);
    report_api(resetvalue.r(utra::resetvalue::PC));

    // sce_dma_tests();

    setup_uart2();

    // ---------- if activated, run the APB test. This is based off of Philip's "touch all the registers"
    // test.
    #[cfg(feature = "apb-test")]
    apb_test();

    // ---------- vm setup -------------------------
    satp::satp_setup(); // at the conclusion of this, we are running in "supervisor" (kernel) mode, with Sv32 semantics
    report_api(0x5a1d_6060);

    #[cfg(feature = "daric")]
    {
        let mut uart = debug::Uart {};
        uart.tiny_write_str("hello world!\r");
    }
    #[cfg(feature = "pio-test")]
    xous_pio::pio_tests::setup_reporting(
        (utra::main::REPORT.offset() + utra::main::HW_MAIN_BASE) as *mut u32,
    );

    // ---------- PIO hack-test ----------------
    //#[cfg(feature="pio-test")]
    //{
    //    uart.tiny_write_str("spi test\r");
    //    pio_hack_test();
    //    uart.tiny_write_str("spi test done\r");
    //}

    // ---------- pio test option -------------
    #[cfg(feature = "pio-test")]
    xous_pio::pio_tests::pio_tests();

    // ---------- bio test option -------------
    #[cfg(feature = "bio-test")]
    print!("bio start\r");
    #[cfg(feature = "bio-test")]
    xous_bio::bio_tests::bio_tests();
    #[cfg(feature = "bio-test")]
    print!("bio end\r");

    // ---------- exception setup ------------------
    irqs::irq_setup();

    // ---------- PL230 test option ----------------
    #[cfg(feature = "pl230-test")]
    {
        let iox_csr = utra::iox::HW_IOX_BASE as *mut u32;
        unsafe {
            iox_csr.add(0x8 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBL
            iox_csr.add(0xC / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBH
            iox_csr.add(0x10 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCL
            iox_csr.add(0x14 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCH
            iox_csr.add(0x200 / core::mem::size_of::<u32>()).write_volatile(0xffffffff); // PIO sel port D31-0
        }
        xous_pl230::pl230_tests::pl230_tests();
    }

    uart.tiny_write_str("done\r");

    // ---------- coreuser test --------------------
    satp::satp_test();
    uart.tiny_write_str("satp done\r");

    // ---------- exception test -------------------
    irqs::irq_test();

    // ---------- xip region test ------------------
    #[cfg(feature = "xip")]
    xip_test();

    // ---------- CPU CSR tests --------------
    report_api(0xc520_0000);
    let mut csrtest = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    let mut passing = true;
    for i in 0..4 {
        csrtest.wfo(utra::csrtest::WTEST_WTEST, i);
        let val = csrtest.rf(utra::csrtest::RTEST_RTEST);
        report_api(val);
        if val != i + 0x1000_0000 {
            passing = false;
        }
    }
    if passing {
        report_api(0xc520_600d);
    } else {
        report_api(0xc520_dead);
    }

    // ---------- wfi test -------------------------
    irqs::wfi_test();

    // ----------- caching tests -------------
    // test of the 0x500F cache flush instruction - this requires manual inspection of the report values
    report_api(0x000c_ac7e);
    const CACHE_WAYS: usize = 4;
    const CACHE_SET_SIZE: usize = 4096 / size_of::<u32>();
    let test_slice = core::slice::from_raw_parts_mut(satp::PT_LIMIT as *mut u32, CACHE_SET_SIZE * CACHE_WAYS);
    // bottom of cache
    for set in 0..4 {
        report_api((&mut test_slice[set * CACHE_SET_SIZE] as *mut u32) as u32);
        (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).write_volatile(0x0011_1111 * (1 + set as u32));
    }
    // top of cache
    for set in 0..4 {
        report_api((&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32) as u32);
        (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32)
            .write_volatile(0x1100_2222 * (1 + set as u32));
    }
    // read cached values - first iteration populates the cache; second iteration should be cached
    for iter in 0..2 {
        report_api(0xb1d0_0000 + iter + 1);
        for set in 0..4 {
            let a = (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).read_volatile();
            report_api(a);
            let b = (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32).read_volatile();
            report_api(b);
        }
    }
    // flush cache
    report_api(0xff00_ff00);
    core::arch::asm!(".word 0x500F",);
    report_api(0x0f0f_0f0f);
    // read cached values - first iteration populates the cache; second iteration should be cached
    for iter in 0..2 {
        report_api(0xb2d0_0000 + iter + 1);
        for set in 0..4 {
            let a = (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).read_volatile();
            report_api(a);
            let b = (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32).read_volatile();
            report_api(b);
        }
    }
    report_api(0x600c_ac7e);

    // check that caching is disabled for I/O regions
    // TODO: figure out how to test this on regular DARIC build
    // these register do not exist on the full chip, it's only in the local validation framework
    {
        let mut checkstate = 0x1234_0000;
        report.wfo(utra::main::WDATA_WDATA, 0x1234_0000);
        let mut checkdata = 0;
        for _ in 0..100 {
            checkdata = report.rf(utra::main::RDATA_RDATA); // RDATA = WDATA + 5, computed in hardware
            report.wfo(utra::main::WDATA_WDATA, checkdata);
            // report_api(checkdata);
            checkstate += 5;
        }
        if checkdata == checkstate {
            report_api(checkstate);
            report_api(0x600d_0001);
        } else {
            report_api(checkstate);
            report_api(checkdata);
            report_api(0x0bad_0001);
        }

        // check that repeated reads of a register fetch new contents
        let mut checkdata = 0; // tracked value via simulation
        let mut computed = 0; // computed value by reading the hardware block
        let mut devstate = 0; // what the state should be
        for _ in 0..20 {
            let readout = report.rf(utra::main::RINC_RINC);
            computed += readout;
            // report_api(readout);
            checkdata += devstate;
            devstate += 3;
        }
        if checkdata == computed {
            report_api(checkdata);
            report_api(0x600d_0002);
        } else {
            report_api(checkdata);
            report_api(computed);
            report_api(0x0bad_0002);
        }
    }

    // ----------- bus tests -------------
    const BASE_ADDR: u32 = satp::PT_LIMIT as u32; // don't overwrite our PT data
    // 'random' access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 512);
    ramtest_lfsr(&mut test_slice, 3);

    // now some basic memory read/write tests
    // entirely within cache access test
    // 256-entry by 32-bit slice at start of RAM
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 256);
    ramtest_all(&mut test_slice, 4);
    // byte access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u8, 256);
    ramtest_fast(&mut test_slice, 5);
    // word access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u16, 512);
    ramtest_fast(&mut test_slice, 6); // 1ff00

    // outside cache test
    // 6144-entry by 32-bit slice at start of RAM - should cross outside cache boundary
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 0x1800);
    ramtest_fast(&mut test_slice, 7); // c7f600

    // this passed, now that the AXI state machine is fixed.
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 0x1800);
    ramtest_fast_specialcase1(&mut test_slice, 8); // c7f600

    // u64 access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u64, 0xC00);
    ramtest_fast(&mut test_slice, 9);

    // random size/access test
    // let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u8, 0x6000);

    report.wfo(utra::main::DONE_DONE, 1);

    #[cfg(feature = "rram-testing")]
    {
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

    uart.tiny_write_str("test finished\r");
    loop {
        #[cfg(feature = "daric")]
        {
            // uart.tiny_write_str("test finished\r");
        }
    }
}

use xous_pio::pio_tests::spi::*;
use xous_pio::*;

pub fn spi_test_core_boot(pio_sm: &mut PioSm) -> bool {

    report_api(0x0D10_05D1);

    const BUF_SIZE: usize = 20;
    let mut state: u16 = 0xAF;
    let mut tx_buf = [0u8; BUF_SIZE];
    let mut rx_buf = [0u8; BUF_SIZE];
    // init the TX buf
    for d in tx_buf.iter_mut() {
        state = crate::utils::lfsr_next(state);
        *d = state as u8;
        report_api(*d as u32);
    }
    pio_spi_write8_read8_blocking(pio_sm, &tx_buf, &mut rx_buf);
    let mut pass = true;
    for (&s, &d) in tx_buf.iter().zip(rx_buf.iter()) {
        if s != d {
            report_api(0xDEAD_0000 | (s as u32) << 8 | ((d as u32) << 0));
            pass = false;
        }
    }
    report_api(0x600D_05D1);
    pass
}

pub fn pio_hack_test() -> bool {
    let iox_csr = utra::iox::HW_IOX_BASE as *mut u32;
    unsafe {
        iox_csr.add(0x8 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBL
        iox_csr.add(0xC / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBH
        iox_csr.add(0x10 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCL
        iox_csr.add(0x14 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCH
        iox_csr.add(0x200 / core::mem::size_of::<u32>()).write_volatile(0xffffffff); // PIO sel port D31-0
    }

    const PIN_SCK: usize = 16; // PC00
    const PIN_MOSI: usize = 17; // PC01
    const PIN_MISO: usize = 17; // loopback    18; // PC02

    let mut pio_csr = CSR::new(utra::rp_pio::HW_RP_PIO_BASE as *mut u32);

    report_api(0x0D10_05D1);

    let mut pio_ss = PioSharedState::new();
    let mut pio_sm = pio_ss.alloc_sm().unwrap();

    // spi_cpha0 example
    let spi_cpha0_prog =
        pio_proc::pio_asm!(".side_set 1", "out pins, 1 side 0 [1]", "in pins, 1  side 1 [1]",);
    // spi_cpha1 example
    let spi_cpha1_prog = pio_proc::pio_asm!(
        ".side_set 1",
        "out x, 1    side 0",     // Stall here on empty (keep SCK deasserted)
        "mov pins, x side 1 [1]", // Output data, assert SCK (mov pins uses OUT mapping)
        "in pins, 1  side 0"      // Input data, deassert SCK
    );
    let prog_cpha0 = LoadedProg::load(spi_cpha0_prog.program, &mut pio_ss).unwrap();
    report_api(0x05D1_0000);
    let prog_cpha1 = LoadedProg::load(spi_cpha1_prog.program, &mut pio_ss).unwrap();
    report_api(0x05D1_0001);

    let clkdiv: f32 = 137.25;
    let mut passing = true;
    let mut cpol = false;
    pio_csr.wo(utra::rp_pio::SFR_IRQ0_INTE, pio_sm.sm_bitmask());
    pio_csr.wo(utra::rp_pio::SFR_IRQ1_INTE, (pio_sm.sm_bitmask()) << 4);
    loop {
        // pha = 1
        report_api(0x05D1_0002);
        pio_spi_init(
            &mut pio_sm,
            &prog_cpha0, // cpha set here
            8,
            clkdiv,
            cpol,
            PIN_SCK,
            PIN_MOSI,
            PIN_MISO,
        );
        report_api(0x05D1_0003);
        if spi_test_core_boot(&mut pio_sm) == false {
            passing = false;
        };

        // pha = 0
        report_api(0x05D1_0004);
        pio_spi_init(
            &mut pio_sm,
            &prog_cpha1, // cpha set here
            8,
            clkdiv,
            cpol,
            PIN_SCK,
            PIN_MOSI,
            PIN_MISO,
        );
        report_api(0x05D1_0005);
        if spi_test_core_boot(&mut pio_sm) == false {
            passing = false;
        };
        if cpol {
            break;
        }
        // switch to next cpol value for test
        cpol = true;
    }
    // cleanup external side effects for next test
    pio_sm.gpio_reset_overrides();
    pio_csr.wo(utra::rp_pio::SFR_IRQ0_INTE, 0);
    pio_csr.wo(utra::rp_pio::SFR_IRQ1_INTE, 0);
    pio_csr.wo(utra::rp_pio::SFR_SYNC_BYPASS, 0);

    if passing {
        report_api(0x05D1_600D);
    } else {
        report_api(0x05D1_DEAD);
    }
    assert!(passing);
    passing
}
