use utralib::generated::*;
use utralib::utra::sysctrl;

use crate::debug;
use crate::utils::*;
use crate::*;

const SETUP_UART2_TESTS: usize = 0;
crate::impl_test!(SetupUart2Tests, "Setup UART2", SETUP_UART2_TESTS);
impl TestRunner for SetupUart2Tests {
    fn run(&mut self) { setup_uart2(); }
}

// returns the actual per_clk
pub unsafe fn init_clock_asic(freq_hz: u32) -> u32 {
    use utra::sysctrl;
    let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;
    /*
       Code notes from RTL:
       assign pll_m = ipc_pllmn[16:12];
       assign pll_n = ipc_pllmn[11: 0];
       assign pll_f = ipc_pllf[23: 0];
       assign pll_fen = ipc_pllf[24];
       assign pll_q00 = ipc_pllq[ 2: 0];
       assign pll_q10 = ipc_pllq[ 6: 4];
       assign pll_q01 = ipc_pllq[10: 8];
       assign pll_q11 = ipc_pllq[14:12];

       Clko0 = Fvco / (pllq[ 2:0] + 1) / (pllq[ 6:4] + 1)
       Clko1 = Fvco / (pllq[10:8] + 1) / (pllq[14:12] + 1)
       Fvco target is 2GHz (1-3GHz range)

      .gvco_bias ( pll_bias[7:6] ),
      .cpp_bias  ( pll_bias[5:3] ),
      .cpi_bias  ( pll_bias[2:0] ),
    */
    // Derive VCO frequency from legal, even dividers that get us close to our target frequency
    const TARGET_VCO_HZ: u32 = 1_600_000_000; // 1.6GHz
    let final_div: u32 = TARGET_VCO_HZ / freq_hz;
    // fclk_div has to be a power of 2
    let fclk_div =
        if (1 << final_div.ilog2()) != final_div { 1 << (final_div.ilog2() + 1) } else { final_div };
    let vco_actual: u32 = fclk_div * freq_hz;

    const TARGET_PERCLK_HZ: u32 = 100_000_000; // 100 MHz
    let perclk_np_div: u32 = vco_actual / TARGET_PERCLK_HZ;
    let perclk_div = if (1 << perclk_np_div.ilog2()) != perclk_np_div {
        1 << (perclk_np_div.ilog2() + 1)
    } else {
        perclk_np_div
    };
    let ilog2_fdiv = fclk_div.ilog2();
    let ilog2_pdiv = perclk_div.ilog2();
    let pll_q0_0 = (1 << (ilog2_fdiv / 2)) - 1;
    let pll_q1_0 = (1 << (ilog2_fdiv / 2 + ilog2_fdiv % 2)) - 1;
    let pll_q0_1 = (1 << (ilog2_pdiv / 2)) - 1;
    let pll_q1_1 = (1 << (ilog2_pdiv / 2 + ilog2_pdiv % 2)) - 1;

    // this is the pllq value
    let pllq = (pll_q0_0 & 7) | ((pll_q1_0 & 7) << 4) | ((pll_q0_1 & 7) << 8) | ((pll_q1_1 & 7) << 12);

    // now, program the VCO to get to as close to vco_actual
    const FREF_HZ: u32 = 48_000_000;
    // adjust m so that PFD runs between 4-16MHz (target 8MHz)
    const PREDIV_M: u32 = 6;
    let fref_hz = FREF_HZ / PREDIV_M;
    assert!(fref_hz == 8_000_000);

    let ni = vco_actual / fref_hz;
    let pllmn = (PREDIV_M << 12) | ni & 0xFFF; // m is set to PREDIV_M, lower 12 bits is nf
    let frac_n = ((vco_actual as f32 / fref_hz as f32) - ni as f32).max(0 as f32);
    let pllf: u32 = (frac_n * ((1 << 24) as f32)) as u32;
    let n_frac = if pllf != 0 { pllf | 1 << 24 } else { 0 }; // set the frac enable bit if needed

    crate::println!("pllq: 0x{:x}, pllmn: 0x{:x}, n_frac: 0x{:x}", pllq, pllmn, n_frac);

    daric_cgu.add(sysctrl::SFR_CGUSEL1.offset()).write_volatile(1); // 0: RC, 1: XTAL
    daric_cgu.add(sysctrl::SFR_CGUFSCR.offset()).write_volatile(48); // external crystal is 48MHz
    daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);

    if freq_hz < 1_000_000 {
        daric_cgu.add(sysctrl::SFR_IPCOSC.offset()).write_volatile(freq_hz);
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32
    }
    // switch to OSC
    daric_cgu.add(sysctrl::SFR_CGUSEL0.offset()).write_volatile(0); // clktop sel, 0:clksys, 1:clkpll0
    daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32); // commit

    if 0 == freq_hz {
        // do nothing
    } else {
        // PD PLL
        daric_cgu
            .add(sysctrl::SFR_IPCLPEN.offset())
            .write_volatile(daric_cgu.add(sysctrl::SFR_IPCLPEN.offset()).read_volatile() | 0x02);
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32

        // delay
        for _ in 0..1024 {
            unsafe { core::arch::asm!("nop") };
        }
        // crate::println!("PLL delay 1");

        daric_cgu.add(sysctrl::SFR_IPCPLLMN.offset()).write_volatile(pllmn); // 0x1F598;
        daric_cgu.add(sysctrl::SFR_IPCPLLF.offset()).write_volatile(n_frac); // 0x2812
        daric_cgu.add(sysctrl::SFR_IPCPLLQ.offset()).write_volatile(pllq); // 0x2401 TODO select DIV for VCO freq

        //               VCO bias   CPP bias   CPI bias
        //                1          2          3
        // DARIC_IPC->ipc = (3 << 6) | (5 << 3) | (5);
        daric_cgu.add(sysctrl::SFR_IPCCR.offset()).write_volatile((1 << 6) | (2 << 3) | (3));
        // daric_cgu.add(sysctrl::SFR_IPCCR.offset()).write_volatile((3 << 6) | (5 << 3) | (5));
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32

        daric_cgu
            .add(sysctrl::SFR_IPCLPEN.offset())
            .write_volatile(daric_cgu.add(sysctrl::SFR_IPCLPEN.offset()).read_volatile() & !0x02);
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32

        // delay
        for _ in 0..1024 {
            unsafe { core::arch::asm!("nop") };
        }
        // crate::println!("PLL delay 2");

        daric_cgu.add(sysctrl::SFR_CGUSEL0.offset()).write_volatile(1); // clktop sel, 0:clksys, 1:clkpll0
        daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32); // commit

        for _ in 0..1024 {
            unsafe { core::arch::asm!("nop") };
        }
        // crate::println!("PLL delay 3");

        crate::println!("fsvalid: {}", daric_cgu.add(sysctrl::SFR_CGUFSVLD.offset()).read_volatile());
        let _cgufsfreq0 = daric_cgu.add(sysctrl::SFR_CGUFSSR_FSFREQ0.offset()).read_volatile();
        let _cgufsfreq1 = daric_cgu.add(sysctrl::SFR_CGUFSSR_FSFREQ1.offset()).read_volatile();
        let _cgufsfreq2 = daric_cgu.add(sysctrl::SFR_CGUFSSR_FSFREQ2.offset()).read_volatile();
        let _cgufsfreq3 = daric_cgu.add(sysctrl::SFR_CGUFSSR_FSFREQ3.offset()).read_volatile();
        crate::println!(
            "Internal osc: {} -> {} MHz ({} MHz)",
            _cgufsfreq0,
            fsfreq_to_hz(_cgufsfreq0),
            fsfreq_to_hz_32(_cgufsfreq0)
        );
        crate::println!(
            "XTAL: {} -> {} MHz ({} MHz)",
            _cgufsfreq1,
            fsfreq_to_hz(_cgufsfreq1),
            fsfreq_to_hz_32(_cgufsfreq1)
        );
        crate::println!(
            "pll output 0: {} -> {} MHz ({} MHz)",
            _cgufsfreq2,
            fsfreq_to_hz(_cgufsfreq2),
            fsfreq_to_hz_32(_cgufsfreq2)
        );
        crate::println!(
            "pll output 1: {} -> {} MHz ({} MHz)",
            _cgufsfreq3,
            fsfreq_to_hz(_cgufsfreq3),
            fsfreq_to_hz_32(_cgufsfreq3)
        );

        // Hits a 16:8:4:2:1 ratio on fclk:aclk:hclk:iclk:pclk
        // Resulting in 800:400:200:100:50 MHz assuming 800MHz fclk
        #[cfg(feature = "fast-fclk")]
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_0.offset()).write_volatile(0x7fff); // fclk

        // Hits a 8:8:4:2:1 ratio on fclk:aclk:hclk:iclk:pclk
        // Resulting in 400:400:200:100:50 MHz assuming 800MHz fclk
        #[cfg(not(feature = "fast-fclk"))]
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_0.offset()).write_volatile(0x7f7f); // fclk
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_1.offset()).write_volatile(0x3f7f); // aclk
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_2.offset()).write_volatile(0x1f3f); // hclk
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_3.offset()).write_volatile(0x0f1f); // iclk
        daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_4.offset()).write_volatile(0x070f); // pclk
        // commit dividers
        daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);
    }
    crate::println!("PLL configured to {} MHz", freq_hz / 1_000_000);

    vco_actual / perclk_div
}

#[allow(dead_code)]
fn fsfreq_to_hz(fs_freq: u32) -> u32 { (fs_freq * (48_000_000 / 32)) / 1_000_000 }

#[allow(dead_code)]
fn fsfreq_to_hz_32(fs_freq: u32) -> u32 { (fs_freq * (32_000_000 / 32)) / 1_000_000 }

pub unsafe fn early_init() {
    #[cfg(not(feature = "quirks-pll"))]
    {
        // This block is MANDATORY for any chip stability in real silicon, as the initial
        // clocks are too unstable to do anything otherwise.
        let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;

        daric_cgu.add(sysctrl::SFR_CGUSEL1.offset()).write_volatile(1); // 0: RC, 1: XTAL
        daric_cgu.add(sysctrl::SFR_CGUFSCR.offset()).write_volatile(48); // external crystal is 48MHz
        daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);
        daric_cgu.add(utra::sysctrl::SFR_IPCOSC.offset()).write_volatile(16_000_000);
        daric_cgu.add(utra::sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32);

        daric_cgu.add(utra::sysctrl::SFR_CGUSEL0.offset()).write_volatile(0);
        daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);
    }
    // this block is mandatory in all cases to get clocks set into some consistent, expected mode
    {
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
    }
    // enable DUART
    let duart = utra::duart::HW_DUART_BASE as *mut u32;
    duart.add(utra::duart::SFR_CR.offset()).write_volatile(0);
    duart.add(utra::duart::SFR_ETUC.offset()).write_volatile(24);
    duart.add(utra::duart::SFR_CR.offset()).write_volatile(1);
}

// these register do not exist in our local simulation model
pub fn setup_uart2() {
    const UART_IFRAM_ADDR: usize = utralib::HW_IFRAM0_MEM + utralib::HW_IFRAM0_MEM_LEN - 4096;
    use cramium_hal::iox::{Iox, IoxDir, IoxEnable, IoxFunction, IoxPort};
    use cramium_hal::udma;
    use cramium_hal::udma::Udma;

    let mut uart = debug::Uart {};
    let sysctrl = CSR::new(utra::sysctrl::HW_SYSCTRL_BASE as *mut u32);
    uart.tiny_write_str("FREQ0: ");
    uart.print_hex_word(sysctrl.rf(utra::sysctrl::SFR_CGUFSSR_FSFREQ0_FSFREQ0));
    uart.tiny_write_str("\r");
    uart.tiny_write_str("FREQ1: ");
    uart.print_hex_word(sysctrl.rf(utra::sysctrl::SFR_CGUFSSR_FSFREQ1_FSFREQ1));
    uart.tiny_write_str("\r");
    uart.tiny_write_str("FREQ2: ");
    uart.print_hex_word(sysctrl.rf(utra::sysctrl::SFR_CGUFSSR_FSFREQ2_FSFREQ2));
    uart.tiny_write_str("\r");
    uart.tiny_write_str("FREQ3: ");
    uart.print_hex_word(sysctrl.rf(utra::sysctrl::SFR_CGUFSSR_FSFREQ3_FSFREQ3));
    uart.tiny_write_str("\r");

    uart.tiny_write_str("udma\r");
    //  UART_RX_A[1] = PD13
    //  UART_RX_A[1] = PD14
    let mut iox = Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
    iox.set_alternate_function(IoxPort::PD, 13, IoxFunction::AF1);
    iox.set_alternate_function(IoxPort::PD, 14, IoxFunction::AF1);
    // rx as input, with pull-up
    iox.set_gpio_dir(IoxPort::PD, 13, IoxDir::Input);
    iox.set_gpio_pullup(IoxPort::PD, 13, IoxEnable::Enable);
    // tx as output
    iox.set_gpio_dir(IoxPort::PD, 14, IoxDir::Output);

    // Set up the UDMA_UART block to the correct baud rate and enable status
    let mut udma_global = udma::GlobalConfig::new(utra::udma_ctrl::HW_UDMA_CTRL_BASE as *mut u32);
    udma_global.clock_on(udma::PeriphId::Uart1);
    udma_global.map_event(
        udma::PeriphId::Uart1,
        udma::PeriphEventType::Uart(udma::EventUartOffset::Rx),
        udma::EventChannel::Channel0,
    );
    udma_global.map_event(
        udma::PeriphId::Uart1,
        udma::PeriphEventType::Uart(udma::EventUartOffset::Tx),
        udma::EventChannel::Channel1,
    );

    let baudrate: u32 = 115200;
    let freq: u32 = 45_882_000;

    // the address of the UART buffer is "hard-allocated" at an offset one page from the top of
    // IFRAM0. This is a convention that must be respected by the UDMA UART library implementation
    // for things to work.
    let uart_buf_addr = UART_IFRAM_ADDR;
    let mut udma_uart = unsafe {
        // safety: this is safe to call, because we set up clock and events prior to calling new.
        udma::Uart::get_handle(utra::udma_uart_1::HW_UDMA_UART_1_BASE, uart_buf_addr, uart_buf_addr)
    };
    let div: u32 = (freq + baudrate / 2) / baudrate;
    uart.tiny_write_str("divder: 0x");
    report_api(div);
    udma_uart.set_baud(baudrate, freq);

    uart.print_hex_word(udma_uart.csr().r(utra::udma_uart_1::REG_UART_SETUP));
    let mut tx_buf = [0u8; 32];
    for (i, t) in tx_buf.iter_mut().enumerate() {
        *t = '0' as char as u8 + i as u8;
    }
    for _ in 0..16 {
        udma_uart.write(&tx_buf);
    }
    uart.tiny_write_str("udma done\r");
}
