#![no_std]
#![no_main]
#![allow(unreachable_code)] // allow debugging of failures to jump out of the bootloader

use utralib::generated::*;

mod debug;
mod init;
mod irqs;
mod pio;
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
    #[cfg(feature = "rram-testing")]
    rram_tests_early();

    reset_ticktimer();
    snap_ticks("sysctrl: ipen ");

    uart.tiny_write_str("setting clocks\r");
    init_clock_asic(800_000_000);

    uart.tiny_write_str("booting... 001\r");

    report_api(0x600dc0de);

    // report the measured reset value
    let resetvalue = CSR::new(utra::resetvalue::HW_RESETVALUE_BASE as *mut u32);
    report_api(resetvalue.r(utra::resetvalue::PC));

    #[cfg(feature = "check-byte-strobes")]
    check_byte_strobes();

    #[cfg(feature = "sce-dma")]
    sce_dma_tests();

    #[cfg(feature = "uart2")]
    setup_uart2();

    #[cfg(feature = "apb-test")]
    apb_test();

    #[cfg(feature = "virtual-memory")]
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

    #[cfg(feature = "pio-quick-test")]
    pio_quick_test();

    #[cfg(feature = "pio-test")]
    xous_pio::pio_tests::pio_tests();

    #[cfg(feature = "bio-test")]
    {
        print!("bio start\r");
        xous_bio::bio_tests::bio_tests();
        print!("bio end\r");
    }

    // This also sets up exceptions for all other tests that depend on IRQs
    #[cfg(feature = "irq-test")]
    irqs::irq_setup();

    #[cfg(feature = "pl230-test")]
    {
        // setup IOs for the PL230 test outputs
        let iox_csr = utra::iox::HW_IOX_BASE as *mut u32;
        unsafe {
            iox_csr.add(0x8 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBL
            iox_csr.add(0xC / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBH
            iox_csr.add(0x10 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCL
            iox_csr.add(0x14 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCH
            iox_csr.add(0x200 / core::mem::size_of::<u32>()).write_volatile(0xffffffff); // PIO sel port D31-0
        }
        xous_pl230::pl230_tests::pl230_tests();
        uart.tiny_write_str("PL230 done\r");
    }

    #[cfg(feature = "satp-test")]
    satp::satp_test();

    #[cfg(feature = "irq-test")]
    irqs::irq_test();

    #[cfg(feature = "xip")]
    xip_test();

    #[cfg(feature = "wfi-test")]
    irqs::wfi_test();

    #[cfg(feature = "caching-test")]
    caching_tests();

    #[cfg(feature = "ram-test")]
    ram_tests();

    #[cfg(feature = "rram-testing")]
    rram_tests_late();

    uart.tiny_write_str("End of tests\r");

    // this triggers the simulation to end using a sim-only verilog hook
    let mut report = CSR::new(utra::main::HW_MAIN_BASE as *mut u32);
    report.wfo(utra::main::DONE_DONE, 1);
    loop {}
}
