use utralib::generated::*;
use utralib::utra::sysctrl;

use crate::debug;
use crate::utils::*;

pub fn init_duart(etu: u32) {
    /*
    let mut duart = CSR::new(utra::duart::HW_DUART_BASE as *mut u32);
    // freq of 32MHz RC is low
    duart.wo(utra::duart::SFR_CR, 0);
    duart.wo(utra::duart::SFR_ETUC, etu);
    duart.wo(utra::duart::SFR_CR, 1);
    */
    /*
    let daric_sramwait = utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32;
    unsafe {
        let waitcycles = 3;
        daric_sramwait.add(utra::coresub_sramtrm::SFR_SRAM0.offset()).write_volatile(
            daric_sramwait.add(utra::coresub_sramtrm::SFR_SRAM0.offset()).read_volatile()
            & !0x18 | ((waitcycles << 3) & 0x18)
        );
        daric_sramwait.add(utra::coresub_sramtrm::SFR_SRAM1.offset()).write_volatile(
            daric_sramwait.add(utra::coresub_sramtrm::SFR_SRAM1.offset()).read_volatile()
            & !0x18 | ((waitcycles << 3) & 0x18)
        );
    } */
    let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;
    unsafe {
        daric_cgu.add(utra::sysctrl::SFR_CGUSEL1.offset()).write_volatile(1);
        daric_cgu.add(utra::sysctrl::SFR_CGUFSCR.offset()).write_volatile(48);
        daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);
        daric_cgu.add(utra::sysctrl::SFR_IPCOSC.offset()).write_volatile(16_000_000);
        daric_cgu.add(utra::sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32);

        daric_cgu.add(utra::sysctrl::SFR_CGUSEL0.offset()).write_volatile(0);
        daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);

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

    let duart = utra::duart::HW_DUART_BASE as *mut u32;
    unsafe {
        duart.add(utra::duart::SFR_CR.offset()).write_volatile(0);
        duart.add(utra::duart::SFR_ETUC.offset()).write_volatile(etu);
        duart.add(utra::duart::SFR_CR.offset()).write_volatile(1);
    }
}

pub unsafe fn init_clock_asic(freq_hz: u32) {
    let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;

    const F_MHZ: u32 = 1_000_000;
    const FREQ_0: u32 = 16 * F_MHZ;

    const TBL_Q: [u16; 7] = [
        // keep later DIV even number as possible
        0x7777, // 16-32 MHz
        0x7737, // 32-64
        0x3733, // 64-128
        0x3313, // 128-256
        0x3311, // 256-512 // keep ~ 100MHz
        0x3301, // 512-1024
        0x3301, /* 1024-1500
                 * 0x1303, // 256-512
                 * 0x0103, // 512-1024
                 * 0x0001, // 1024-2048 */
    ];
    const TBL_MUL: [u32; 7] = [
        64, // 16-32 MHz
        32, // 32-64
        16, // 64-128
        8,  // 128-256
        4,  // 256-512
        2,  // 512-1024
        2,  // 1024-2048
    ];
    const M: u32 = 24 - 1;

    report_api(0xc0c0_0000);
    let f16_mhz_log2 = (freq_hz / FREQ_0).ilog2() as usize;
    report_api(f16_mhz_log2 as u32);
    let n_fxp24: u64 = (((freq_hz as u64) << 24) * TBL_MUL[f16_mhz_log2] as u64) / (2 * F_MHZ as u64);
    report_api(n_fxp24 as u32);
    report_api((n_fxp24 >> 32) as u32);
    let n_frac: u32 = (n_fxp24 & 0x00ffffff) as u32;
    report_api(n_frac);
    let pllmn = ((M << 12) & 0x0001F000) | ((n_fxp24 >> 24) & 0x00000fff) as u32;
    report_api(pllmn);
    let pllf = n_frac | (if 0 == n_frac { 0 } else { 1 << 24 });
    report_api(pllf);
    let pllq = TBL_Q[f16_mhz_log2] as u32;
    report_api(pllq);

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
        for _ in 0..4 {
            report_api(0xc0c0_dddd);
        }

        // printf ("%s(%4" PRIu32 "MHz) M = 24, N = %4lu.%08lu, Q = %2lu\n",
        //     __FUNCTION__, freqHz / 1000000, (uint32_t)(n_fxp24 >>
        // 24).write_volatile((uint32_t)((uint64_t)(n_fxp24 & 0x00ffffff) * 100000000/(1UL
        // <<24)).write_volatile(TBL_MUL[f16MHzLog2]);
        daric_cgu.add(sysctrl::SFR_IPCPLLMN.offset()).write_volatile(pllmn); // 0x1F598; // ??
        daric_cgu.add(sysctrl::SFR_IPCPLLF.offset()).write_volatile(pllf); // ??
        daric_cgu.add(sysctrl::SFR_IPCPLLQ.offset()).write_volatile(pllq); // ?? TODO select DIV for VCO freq

        //               VCO bias   CPP bias   CPI bias
        //                1          2          3
        // DARIC_IPC->ipc = (3 << 6) | (5 << 3) | (5);
        daric_cgu.add(sysctrl::SFR_IPCCR.offset()).write_volatile((1 << 6) | (2 << 3) | (3));
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32

        daric_cgu
            .add(sysctrl::SFR_IPCLPEN.offset())
            .write_volatile(daric_cgu.add(sysctrl::SFR_IPCLPEN.offset()).read_volatile() & !0x02);
        daric_cgu.add(sysctrl::SFR_IPCARIPFLOW.offset()).write_volatile(0x32); // commit, must write 32

        // delay
        for _ in 0..1024 {
            unsafe { core::arch::asm!("nop") };
        }
        for _ in 0..4 {
            report_api(0xc0c0_eeee);
        }
        // printf("read reg a0 : %08" PRIx32"\n", *((volatile uint32_t* )0x400400a0));
        // printf("read reg a4 : %04" PRIx16"\n", *((volatile uint16_t* )0x400400a4));
        // printf("read reg a8 : %04" PRIx16"\n", *((volatile uint16_t* )0x400400a8));

        // TODO wait/poll lock status?
        daric_cgu.add(sysctrl::SFR_CGUSEL0.offset()).write_volatile(1); // clktop sel, 0:clksys, 1:clkpll0
        daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32); // commit

        report_api(0xc0c0_ffff);
        // printf ("    MN: 0x%05x, F: 0x%06x, Q: 0x%04x\n",
        //     DARIC_IPC->pll_mn, DARIC_IPC->pll_f, DARIC_IPC->pll_q);
        // printf ("    LPEN: 0x%01x, OSC: 0x%04x, BIAS: 0x%04x,\n",
        //     DARIC_IPC->lpen, DARIC_IPC->osc, DARIC_IPC->ipc);
    }

    /*
    daric_cgu.add(sysctrl::SFR_CGUFD_CFGFDCR_0_4_0.offset()).write_volatile(0x3f7f); // fdfclk
    daric_cgu.add(sysctrl::SFR_CGUFD_CFGFDCR_0_4_1.offset()).write_volatile(0x3f7f); // fdaclk
    daric_cgu.add(sysctrl::SFR_CGUFD_CFGFDCR_0_4_2.offset()).write_volatile(0x3f7f); // fdhclk
    daric_cgu.add(sysctrl::SFR_CGUFD_CFGFDCR_0_4_3.offset()).write_volatile(0x3f7f); // fdiclk
    daric_cgu.add(sysctrl::SFR_CGUFD_CFGFDCR_0_4_4.offset()).write_volatile(0x3f7f); // fdpclk
    report_api(0xc0c0_0006);
    daric_cgu.add(sysctrl::SFR_CGUSET.offset()).write_volatile(0x32); // commit
    */

    // UDMACORE->CFG_CG = 0xffffffff; // everything on

    // SCB_InvalidateDCache();
    // __DMB();
    // printf("read reg 90 :%04" PRIx16"\n", *((volatile uint16_t* )0x40040090));

    // SCB_InvalidateDCache();
    // printf("read reg 14 : %04" PRIx16"\n", *((volatile uint16_t* )0x40040014));
    // printf("read reg 18 : %04" PRIx16"\n", *((volatile uint16_t* )0x40040018));
    // printf("read reg 1c : %04" PRIx16"\n", *((volatile uint16_t* )0x4004001c));
    // printf("read reg 20 : %04" PRIx16"\n", *((volatile uint16_t* )0x40040020));
    // printf("read reg 24 : %04" PRIx16"\n", *((volatile uint16_t* )0x40040024));
    // printf("read reg 10 : %04" PRIx16"\n", *((volatile uint16_t* )0x40040010));

    // IFRAM clear
    /*
    volatile uint32_t *const IFRAM = (uint32_t *)0x50000000;
    for (size_t i = 0; i < 256UL * 1024UL / sizeof(uint32_t); i++)
    {
        IFRAM[i] = 0;
    } */
    report_api(0xc0c0_0007);
}

pub fn early_init() {
    let mut uart = debug::Uart {};

    unsafe {
        (0x400400a0 as *mut u32).write_volatile(0x1F598); // F
        uart.print_hex_word((0x400400a0 as *const u32).read_volatile());
        uart.putc('\n' as u32 as u8);
        let poke_array: [(u32, u32, bool); 12] = [
            // commented out because the FPGA does not take kindly to this being set twice
            (0x400400a4, 0x2812, false), //  MN
            (0x400400a8, 0x3301, false), //  Q
            (0x40040090, 0x0032, true),  // setpll
            (0x40040014, 0x7f7f, false), // fclk
            (0x40040018, 0x7f7f, false), // aclk
            (0x4004001c, 0x3f3f, false), // hclk
            (0x40040020, 0x1f1f, false), // iclk
            (0x40040024, 0x0f0f, false), // pclk
            (0x40040010, 0x0001, false), // sel0
            (0x4004002c, 0x0032, true),  // setcgu
            (0x40040060, 0x0003, false), // aclk gates
            (0x40040064, 0x0003, false), // hclk gates
        ];
        for &(addr, dat, is_u32) in poke_array.iter() {
            let rbk = if is_u32 {
                (addr as *mut u32).write_volatile(dat);
                (addr as *const u32).read_volatile()
            } else {
                (addr as *mut u16).write_volatile(dat as u16);
                (addr as *const u16).read_volatile() as u32
            };
            uart.print_hex_word(rbk);
            if dat != rbk {
                uart.putc('*' as u32 as u8);
            }
            uart.putc('\n' as u32 as u8);
        }
    }
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
