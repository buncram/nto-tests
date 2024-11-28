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
mod debug;
mod gpio;
mod init;
mod irqs;
mod mbox;
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

pub use init::*;
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

    #[cfg(not(feature = "quirks-pll"))]
    {
        uart.tiny_write_str("setting clocks\r");
        let perclk = init_clock_asic(800_000_000);
        print!("perclk: {}\r", perclk);
    }
    uart.tiny_write_str("booting... 001\r");
    reset_ticktimer();

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
    let mut udma_tests = udma::UdmaTests::new(cfg!(feature = "udma-tests"));

    // legacy tests - not run on NTO
    let mut setup_uart2_test = init::SetupUart2Tests::new(false);
    let mut pio_quick_tests = pio::PioQuickTests::new(false);
    let mut byte_strobe_tests = ramtests::ByteStrobeTests::new(false);
    let mut xip_tests = ramtests::XipTests::new(false);
    let mut sce_dma_tests = sce::SceDmaTests::new(false);
    let mut pl230_tests = pl230::Pl230Tests::new(false);

    let mut tests: [&mut dyn Test; 20] = [
        &mut reset_value_test,
        // stuff to run first
        &mut rram_tests, // full-chip only, but run early
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
        &mut pio_quick_tests,
        &mut xip_tests,
        &mut sce_dma_tests,
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
