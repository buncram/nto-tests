#![no_std]
#![no_main]
#![allow(unreachable_code)] // allow debugging of failures to jump out of the bootloader

use utralib::generated::*;

mod bio;
mod debug;
mod init;
mod irqs;
mod pio;
mod pl230;
mod ramtests;
mod rram;
mod satp;
mod sce;
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
//  - add a simple mbox test

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
        init_clock_asic(800_000_000);
    }
    uart.tiny_write_str("booting... 001\r");
    reset_ticktimer();

    let mut reset_value_test = utils::ResetValue::new(true);
    let mut satp_tests = satp::SatpTests::new(true);
    let mut irq_tests = irqs::IrqTests::new(true);
    let mut wfi_tests = irqs::WfiTests::new(true);
    let mut rram_tests = rram::RramTests::new(true);
    let mut ram_tests = ramtests::RamTests::new(true);

    let mut setup_uart2_test = init::SetupUart2Tests::new(false);
    let mut pio_quick_tests = pio::PioQuickTests::new(false);
    let mut byte_strobe_tests = ramtests::ByteStrobeTests::new(false);
    let mut xip_tests = ramtests::XipTests::new(false);
    let mut sce_dma_tests = sce::SceDmaTests::new(false);
    let mut bio_tests = bio::BioTests::new(false);
    let mut pl230_tests = pl230::Pl230Tests::new(false);

    let mut tests: [&mut dyn Test; 13] = [
        &mut reset_value_test,
        // at the conclusion of this, we are running in "supervisor" (kernel) mode, with Sv32 semantics
        &mut satp_tests,
        &mut irq_tests,
        &mut wfi_tests,
        &mut setup_uart2_test,
        &mut byte_strobe_tests,
        &mut ram_tests,
        &mut rram_tests,
        &mut pio_quick_tests,
        &mut xip_tests,
        &mut sce_dma_tests,
        &mut bio_tests,
        &mut pl230_tests,
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

    // this triggers the simulation to end using a sim-only verilog hook
    let mut report = CSR::new(utra::main::HW_MAIN_BASE as *mut u32);
    report.wfo(utra::main::DONE_DONE, 1);
    println!("Tests done.");
    loop {}
}

#[cfg(target_os = "none")]
/// Default panic handler
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
