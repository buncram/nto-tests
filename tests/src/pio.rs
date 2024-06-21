use utralib::generated::*;
use xous_pio::pio_tests::spi::*;
use xous_pio::*;

use crate::debug;
use crate::utils::*;

pub fn pio_quick_test() {
    let mut uart = debug::Uart {};
    uart.tiny_write_str("spi test\r");
    pio_hack_test();
    uart.tiny_write_str("spi test done\r");
}

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
