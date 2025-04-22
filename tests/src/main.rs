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

#![no_std]
#![no_main]
#![allow(unreachable_code)] // allow debugging of failures to jump out of the bootloader

use cramium_hal::iox::Iox;
use utralib::generated::*;

mod aes;
mod bio;
mod cam;
mod debug;
mod gpio;
mod init;
mod irqs;
mod mbox;
#[cfg(feature = "pio")]
mod pio;
mod pl230;
mod ramtests;
mod rram;
mod satp;
mod sce;
mod timer0;
mod udma;
mod utils;

mod asm;

mod bioquick;

pub use init::*;
#[cfg(feature = "pio")]
pub use pio::*;
pub use ramtests::*;
pub use rram::*;
pub use sce::*;
pub use utils::*;

#[cfg(feature = "apb-test")]
mod apb_check;
mod daric_generated;
#[cfg(feature = "apb-test")]
use apb_check::apb_test;

/// Boilerplate that we can build into a macro
pub trait TestBoilerplate {
    fn name(&self) -> &'static str;
    fn total_tests(&self) -> usize;
    fn passing_tests(&self) -> usize;
    fn set_enable(&mut self, ena: bool);
    fn is_enabled(&self) -> bool;
}

/// Single method that is variable for each test
pub trait TestRunner {
    fn run(&mut self);
}

trait Test: TestBoilerplate + TestRunner {}
impl<T> Test for T where T: TestBoilerplate + TestRunner {}

/// Macro for implementing all the test boilerplate
#[macro_export]
macro_rules! impl_test {
    ($struct_name:ident, $test_name:expr, $test_count:ident) => {
        pub struct $struct_name {
            name: &'static str,
            passing_tests: usize,
            enabled: bool,
        }
        impl $struct_name {
            pub fn new(enabled: bool) -> Self { Self { name: $test_name, passing_tests: 0, enabled } }
        }
        impl TestBoilerplate for $struct_name {
            fn set_enable(&mut self, ena: bool) { self.enabled = ena }

            fn is_enabled(&self) -> bool { self.enabled }

            fn name(&self) -> &'static str { self.name }

            fn passing_tests(&self) -> usize { self.passing_tests }

            fn total_tests(&self) -> usize { $test_count }
        }
    };
}

// TODO:
//  - add interrupt output to the mbox AHB client
//  - add a simple GPIO mapping test

#[export_name = "rust_entry"]
pub unsafe extern "C" fn rust_entry(_unused1: *const usize, _unused2: u32) -> ! {
    early_init();
    let mut uart = debug::Uart {};
    uart.tiny_write_str("hello world!\r");

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
    // sram0 requires 1 wait state for writes
    let mut sramtrm = CSR::new(utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32);
    sramtrm.wo(utra::coresub_sramtrm::SFR_SRAM0, 0x8);
    sramtrm.wo(utra::coresub_sramtrm::SFR_SRAM1, 0x8);

    #[cfg(feature = "v0p9")]
    {
        /*
    logic [15:0] trm_ram32kx72      ; assign trm_ram32kx72      = trmdat[0 ]; localparam t_trm IV_trm_ram32kx72      = IV_sram_sp_uhde_inst_sram0;
    logic [15:0] trm_ram8kx72       ; assign trm_ram8kx72       = trmdat[1 ]; localparam t_trm IV_trm_ram8kx72       = IV_sram_sp_hde_inst_sram1;
    logic [15:0] trm_rf1kx72        ; assign trm_rf1kx72        = trmdat[2 ]; localparam t_trm IV_trm_rf1kx72        = IV_rf_sp_hde_inst_cache;
    logic [15:0] trm_rf256x27       ; assign trm_rf256x27       = trmdat[3 ]; localparam t_trm IV_trm_rf256x27       = IV_rf_sp_hde_inst_cache;
    logic [15:0] trm_rf512x39       ; assign trm_rf512x39       = trmdat[4 ]; localparam t_trm IV_trm_rf512x39       = IV_rf_sp_hde_inst_cache;
    logic [15:0] trm_rf128x31       ; assign trm_rf128x31       = trmdat[5 ]; localparam t_trm IV_trm_rf128x31       = IV_rf_sp_hde_inst_cache;
    logic [15:0] trm_dtcm8kx36      ; assign trm_dtcm8kx36      = trmdat[6 ]; localparam t_trm IV_trm_dtcm8kx36      = IV_sram_sp_hde_inst_tcm;
    logic [15:0] trm_itcm32kx18     ; assign trm_itcm32kx18     = trmdat[7 ]; localparam t_trm IV_trm_itcm32kx18     = IV_sram_sp_hde_inst_tcm;
    logic [15:0] trm_ifram32kx36    ; assign trm_ifram32kx36    = trmdat[8 ]; localparam t_trm IV_trm_ifram32kx36    = IV_sram_sp_uhde_inst;
    logic [15:0] trm_sce_sceram_10k ; assign trm_sce_sceram_10k = trmdat[9 ]; localparam t_trm IV_trm_sce_sceram_10k = IV_sram_sp_hde_inst;
    logic [15:0] trm_sce_hashram_3k ; assign trm_sce_hashram_3k = trmdat[10]; localparam t_trm IV_trm_sce_hashram_3k = IV_rf_sp_hde_inst;
    logic [15:0] trm_sce_aesram_1k  ; assign trm_sce_aesram_1k  = trmdat[11]; localparam t_trm IV_trm_sce_aesram_1k  = IV_rf_sp_hde_inst;
    logic [15:0] trm_sce_pkeram_4k  ; assign trm_sce_pkeram_4k  = trmdat[12]; localparam t_trm IV_trm_sce_pkeram_4k  = IV_rf_sp_hde_inst;
    logic [15:0] trm_sce_aluram_3k  ; assign trm_sce_aluram_3k  = trmdat[13]; localparam t_trm IV_trm_sce_aluram_3k  = IV_rf_sp_hde_inst;
    logic [15:0] trm_sce_mimmdpram  ; assign trm_sce_mimmdpram  = trmdat[14]; localparam t_trm IV_trm_sce_mimmdpram  = IV_rf_2p_hdc_inst;
    logic [15:0] trm_rdram1kx32     ; assign trm_rdram1kx32     = trmdat[15]; localparam t_trm IV_trm_rdram1kx32     = IV_rf_2p_hdc_inst_vex;
    logic [15:0] trm_rdram512x64    ; assign trm_rdram512x64    = trmdat[16]; localparam t_trm IV_trm_rdram512x64    = IV_rf_2p_hdc_inst_vex;
    logic [15:0] trm_rdram128x22    ; assign trm_rdram128x22    = trmdat[17]; localparam t_trm IV_trm_rdram128x22    = IV_rf_2p_hdc_inst_vex;
    logic [15:0] trm_rdram32x16     ; assign trm_rdram32x16     = trmdat[18]; localparam t_trm IV_trm_rdram32x16     = IV_rf_2p_hdc_inst_vex;
    logic [15:0] trm_bioram1kx32    ; assign trm_bioram1kx32    = trmdat[19]; localparam t_trm IV_trm_bioram1kx32    = IV_rf_sp_hde_inst_cache;
    logic [15:0] trm_tx_fifo128x32  ; assign trm_tx_fifo128x32  = trmdat[20]; localparam t_trm IV_trm_tx_fifo128x32  = IV_rf_2p_hdc_inst;
    logic [15:0] trm_rx_fifo128x32  ; assign trm_rx_fifo128x32  = trmdat[21]; localparam t_trm IV_trm_rx_fifo128x32  = IV_rf_2p_hdc_inst;
    logic [15:0] trm_fifo32x19      ; assign trm_fifo32x19      = trmdat[22]; localparam t_trm IV_trm_fifo32x19      = IV_rf_2p_hdc_inst;
    logic [15:0] trm_udcmem_share   ; assign trm_udcmem_share   = trmdat[23]; localparam t_trm IV_trm_udcmem_share   = IV_rf_2p_hdc_inst;
    logic [15:0] trm_udcmem_odb     ; assign trm_udcmem_odb     = trmdat[24]; localparam t_trm IV_trm_udcmem_odb     = IV_rf_2p_hdc_inst;
    logic [15:0] trm_udcmem_256x64  ; assign trm_udcmem_256x64  = trmdat[25]; localparam t_trm IV_trm_udcmem_256x64  = IV_rf_2p_hdc_inst;
    logic [15:0] trm_acram2kx64     ; assign trm_acram2kx64     = trmdat[26]; localparam t_trm IV_trm_acram2kx64     = IV_sram_sp_uhde_inst_sram0;
    logic [15:0] trm_aoram1kx36     ; assign trm_aoram1kx36     = trmdat[27]; localparam t_trm IV_trm_aoram1kx36     = IV_sram_sp_hde_inst;

         */
        uart.tiny_write_str("t0\r");
        let mut sramtrm = CSR::new(utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32);
        sramtrm.wo(utra::coresub_sramtrm::SFR_CACHE, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_ITCM, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_DTCM, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_VEXRAM, 0x1);

        let mut rbist = CSR::new(utra::rbist_wrp::HW_RBIST_WRP_BASE as *mut u32);
        // bio 0.9v settings
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (19 << 16) | 0b011_000_01_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);

        // vex 0.9v settings
        for i in 0..4 {
            rbist.wo(utra::rbist_wrp::SFRCR_TRM, ((15 + i) << 16) | 0b001_010_00_0_0_000_0_00);
            rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        }

        // sram 0.9v settings
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (0 << 16) | 0b011_000_01_0_1_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (1 << 16) | 0b011_000_00_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        uart.tiny_write_str("t1\r");

        // tcm 0.9v
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (6 << 16) | 0b011_000_00_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (7 << 16) | 0b011_000_00_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);

        // ifram 0.9v
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (8 << 16) | 0b010_000_00_0_1_000_1_01);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);

        // sce 0.9V
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (9 << 16) | 0b011_000_00_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        for i in 0..4 {
            rbist.wo(utra::rbist_wrp::SFRCR_TRM, ((10 + i) << 16) | 0b011_000_01_0_1_000_0_00);
            rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
        }
        rbist.wo(utra::rbist_wrp::SFRCR_TRM, (14 << 16) | 0b001_010_00_0_0_000_0_00);
        rbist.wo(utra::rbist_wrp::SFRAR_TRM, 0x5a);
    }

    #[cfg(not(feature = "quirks-pll"))]
    {
        #[cfg(feature = "altclk")]
        {
            uart.tiny_write_str("setting clocks asic2 400\r");
            let perclk = init_clock_asic2(400_000_000);
            print!("perclk: {}\r", perclk);
        }
        #[cfg(all(not(feature = "altclk"), feature = "v0p9"))]
        unsafe {
            uart.tiny_write_str("set clk asic2 700 fclk350\r");
            let perclk = init_clock_asic2(700_000_000);
            print!("perclk: {}\r", perclk);
            /*
            uart.tiny_write_str("bypass to 500\r");
            // Turn on PLL
            (0x400400A0 as *mut u32).write_volatile(0x103E8); //clk0=500MHz, clk1=200MHz
            (0x400400A4 as *mut u32).write_volatile(0x1000000);
            (0x400400A8 as *mut u32).write_volatile(0x2412);
            (0x40040090 as *mut u32).write_volatile(0x32);
            // Switch to PLL
            (0x40040010 as *mut u32).write_volatile(0x1); //clktop/per=pll0
            (0x40040014 as *mut u32).write_volatile(0x00001FFF); //fd fclk = freq(clk0)
            (0x40040018 as *mut u32).write_volatile(0x00011F7F); //fd aclk = freq(clk0 / 2)
            (0x4004001C as *mut u32).write_volatile(0x00033F3F); //fd hclk = freq(clk0 / 4)
            (0x40040020 as *mut u32).write_volatile(0x00033F1F); //fd iclk = freq(clk0 / 8)
            (0x40040024 as *mut u32).write_volatile(0x00033F0F); //fd pclk = freq(clk0 / 16)
            (0x4004003c as *mut u32).write_volatile(0x01_ff_ff); //perclk
            (0x40040060 as *mut u32).write_volatile(0x2f);
            (0x40040064 as *mut u32).write_volatile(0b1111_1101);
            (0x40040068 as *mut u32).write_volatile(0x8f);
            (0x4004006c as *mut u32).write_volatile(0xff);
            (0x4004002c as *mut u32).write_volatile(0x32);
            */
        }
        #[cfg(all(not(feature = "altclk"), not(feature = "v0p9")))]
        unsafe {
            uart.tiny_write_str("set clk asic2 500 fclk250\r");
            let perclk = init_clock_asic2(500_000_000);
            print!("perclk: {}\r", perclk);
        }
    }
    uart.tiny_write_str("booting... 001\r");
    reset_ticktimer();
    #[cfg(feature = "bio-quick")]
    unsafe {
        bioquick::bio_bypass();
    }

    setup_io();

    let mut aes_tests = aes::AesTests::new(cfg!(feature = "aes-tests"));
    let mut reset_value_test = utils::ResetValue::new(cfg!(feature = "reset-value-tests"));
    let mut bio_tests = bio::BioTests::new(cfg!(feature = "bio-tests"));
    let mut gpio_tests = gpio::GpioTests::new(cfg!(feature = "gpio-tests"));
    let mut satp_setup = satp::SatpSetup::new(cfg!(feature = "satp-tests"));
    let mut irq_setup = irqs::IrqSetup::new(cfg!(feature = "irq-tests"));
    let mut satp_tests = satp::SatpTests::new(cfg!(feature = "satp-tests"));
    let mut irq_tests = irqs::IrqTests::new(cfg!(feature = "irq-tests"));
    let mut wfi_tests = irqs::WfiTests::new(cfg!(feature = "wfi-tests"));
    let mut ram_tests = ramtests::RamTests::new(cfg!(feature = "ram-tests"));
    let mut timer0_tests = timer0::Timer0Tests::new(cfg!(feature = "timer0-tests"));

    let mut mbox_test = mbox::MboxTests::new(cfg!(feature = "mbox-tests"));
    let mut rram_tests = rram::RramTests::new(cfg!(feature = "rram-tests"));
    let mut rram_disturb_tests = rram::RramDisturbTests::new(cfg!(feature = "rram-tests"));
    let mut rram_lifecycle_tests = rram::RramLifecycle::new(cfg!(feature = "lifecycle-tests"));
    let mut udma_tests = udma::UdmaTests::new(cfg!(feature = "udma-tests"));

    // single-purpose test bench. Normally meant to be configured off unless looking specifically at these
    // features.
    let mut cam_tests = cam::CamTests::new(cfg!(feature = "cam-tests"));

    // legacy tests - not run on NTO
    let mut setup_uart2_test = init::SetupUart2Tests::new(false);
    #[cfg(feature = "pio")]
    let mut pio_quick_tests = pio::PioQuickTests::new(false);
    let mut byte_strobe_tests = ramtests::ByteStrobeTests::new(false);
    let mut xip_tests = ramtests::XipTests::new(false);
    let mut sce_dma_tests = sce::SceDmaTests::new(false);
    let mut pl230_tests = pl230::Pl230Tests::new(cfg!(feature = "pl230-tests"));

    let mut tests: [&mut dyn Test; 22] = [
        &mut reset_value_test,
        // stuff to run first
        &mut cam_tests,
        &mut rram_lifecycle_tests,
        &mut rram_tests, // full-chip only, but run early - this isn't passing right now
        &mut gpio_tests,
        // quick tests
        &mut aes_tests,
        &mut bio_tests,
        // tests that can only be run on the full chip
        &mut mbox_test,
        &mut udma_tests,
        // core function setup
        &mut satp_setup,
        &mut irq_setup,
        // test core function
        &mut satp_tests, // this relies on irq setup, so it can't be run right after satp setup
        &mut irq_tests,
        // irq-dependent tests
        &mut timer0_tests,
        &mut wfi_tests,
        // irq + satp dependent tests
        &mut pl230_tests,
        &mut setup_uart2_test,
        &mut byte_strobe_tests,
        &mut ram_tests,
        #[cfg(feature = "pio")]
        &mut pio_quick_tests,
        &mut xip_tests,
        &mut sce_dma_tests,
        // tests to be run at the end of all the tests
        &mut rram_disturb_tests,
    ];

    #[cfg(feature = "apb-test")]
    apb_test();

    for test in tests.iter_mut() {
        if test.is_enabled() {
            println!(">>> Running {}", test.name());
            test.run();
            println!("<<< {} done", test.name());
        }
    }

    for test in tests.iter_mut() {
        if test.is_enabled() {
            println!("Test {}: {}/{} passing", test.name(), test.passing_tests(), test.total_tests());
        }
    }

    println!("Tests done.");
    // this triggers the simulation to end using a sim-only verilog hook
    let mut report = CSR::new(utra::main::HW_MAIN_BASE as *mut u32);
    report.wfo(utra::main::DONE_DONE, 1);
    // this sequence triggers an end of simulation on s32
    let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    test_cfg.wo(utra::csrtest::WTEST, 0xc0ded02e);
    test_cfg.wo(utra::csrtest::WTEST, 0xc0de600d);

    loop {}
}

#[cfg(target_os = "none")]
/// Default panic handler
mod panic_handler {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn handle_panic(arg: &PanicInfo) -> ! {
        crate::println!("{}", arg);
        if let Some(location) = arg.location() {
            crate::println!("At '{}'@{}", location.file(), location.line(),);
        }
        // exit the simulation
        let mut test_cfg = utralib::CSR::new(utralib::utra::csrtest::HW_CSRTEST_BASE as *mut u32);
        test_cfg.wo(utralib::utra::csrtest::WTEST, 0xc0ded02e);
        test_cfg.wo(utralib::utra::csrtest::WTEST, 0xc0de600d);
        let mut report = utralib::CSR::new(utralib::utra::main::HW_MAIN_BASE as *mut u32);
        report.wfo(utralib::utra::main::DONE_DONE, 1);
        loop {}
    }
}

fn setup_io() {
    let iox = Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
    println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
    iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
    println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
}
