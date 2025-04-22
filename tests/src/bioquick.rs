use utralib::generated::*;
use utralib::utra::sysctrl;
use xous_bio_bdma::*;

use crate::print;
use xous_bio_bdma::bio_tests::dma::TestPattern;

#[allow(dead_code)]
pub unsafe fn bio_bypass() -> ! {
    /*
    // enable DUART
    let duart = utra::duart::HW_DUART_BASE as *mut u32;
    duart.add(utra::duart::SFR_CR.offset()).write_volatile(0);
    duart.add(utra::duart::SFR_ETUC.offset()).write_volatile(24);
    duart.add(utra::duart::SFR_CR.offset()).write_volatile(1);

    // sram0 requires 1 wait state for writes
    let mut sramtrm = CSR::new(utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32);
    sramtrm.wo(utra::coresub_sramtrm::SFR_SRAM0, 0x8);
    sramtrm.wo(utra::coresub_sramtrm::SFR_SRAM1, 0x0);

    #[cfg(feature = "v0p9")]
    {
        print!("t0\r");
        let mut sramtrm = CSR::new(utra::coresub_sramtrm::HW_CORESUB_SRAMTRM_BASE as *mut u32);
        sramtrm.wo(utra::coresub_sramtrm::SFR_CACHE, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_ITCM, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_DTCM, 0x3);
        sramtrm.wo(utra::coresub_sramtrm::SFR_VEXRAM, 0x1);
        print!("t1\r");
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
    */
    /*
    let daric_cgu = sysctrl::HW_SYSCTRL_BASE as *mut u32;
    // let mut cgu = CSR::new(daric_cgu);

    (0x40040030 as *mut u32).write_volatile(0x1);
    (0x40040094 as *mut u32).write_volatile(0x0);

    // Turn on PLL
    (0x400400A0 as *mut u32).write_volatile(0x3057); //clk0=500MHz, clk1=200MHz
    (0x400400A4 as *mut u32).write_volatile(0x1000000);
    (0x400400A8 as *mut u32).write_volatile(0x3301);
    (0x40040090 as *mut u32).write_volatile(0x32);

    // Switch to PLL
    (0x40040010 as *mut u32).write_volatile(0x1); //clktop/per=pll0
    */
    /*
    (0x40040014 as *mut u32).write_volatile(0x00001F7F); //fd fclk = freq(clk0)
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
    /*
    // Hits a 8:8:4:2:1 ratio on fclk:aclk:hclk:iclk:pclk
    // Resulting in 400:400:200:100:50 MHz assuming 800MHz fclk
    daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_0.offset()).write_volatile(0x7f7f); // fclk
    daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_1.offset()).write_volatile(0x3f7f); // aclk
    daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_2.offset()).write_volatile(0x1f3f); // hclk
    daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_3.offset()).write_volatile(0x0f1f); // iclk
    daric_cgu.add(utra::sysctrl::SFR_CGUFD_CFGFDCR_0_4_4.offset()).write_volatile(0x070f); // pclk
    // perclk divider - set to divide by 8 off of an 800Mhz base. Only found on NTO.
    daric_cgu.add(utra::sysctrl::SFR_CGUFDPER.offset()).write_volatile(0x03_ff_ff);

    // turn off gates
    daric_cgu.add(utra::sysctrl::SFR_ACLKGR.offset()).write_volatile(0x2f);
    daric_cgu.add(utra::sysctrl::SFR_HCLKGR.offset()).write_volatile(0xff);
    daric_cgu.add(utra::sysctrl::SFR_ICLKGR.offset()).write_volatile(0x8f);
    daric_cgu.add(utra::sysctrl::SFR_PCLKGR.offset()).write_volatile(0xff);
    // commit dividers
    daric_cgu.add(utra::sysctrl::SFR_CGUSET.offset()).write_volatile(0x32);

    for _ in 0..1024 {
        unsafe { core::arch::asm!("nop") };
    }
    crate::println!("PLL delay 1");
    crate::println!("fsvalid: {}", daric_cgu.add(sysctrl::SFR_CGUFSVLD.offset()).read_volatile());
    let _cgufsfreq3 = daric_cgu.add(sysctrl::SFR_CGUFSSR_FSFREQ3.offset()).read_volatile();
    crate::println!(
        "PLL1: {} -> {} MHz ({} MHz)",
        _cgufsfreq3,
        fsfreq_to_hz(_cgufsfreq3),
        fsfreq_to_hz_32(_cgufsfreq3)
    );
    */

    // clocks setup. now try DMA
    bio_tests::dma::dma_filter_off();
    print!("DMA quick\r");
    const TEST_LEN: usize = 64;

    // clear prior test config state
    let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    test_cfg.wo(utra::csrtest::WTEST, 0);

    let mut bio_ss = BioSharedState::new();
    // stop all the machines, so that code can be loaded
    bio_ss.bio.wo(utra::bio_bdma::SFR_CTRL, 0x0);
    bio_ss.load_code(dma_basic2_code(), 0, BioCore::Core0);
    bio_ss.load_code(idle_code(), 0, BioCore::Core1);
    bio_ss.load_code(idle_code(), 0, BioCore::Core2);
    bio_ss.load_code(idle_code(), 0, BioCore::Core3);

    // These actually "don't matter" because there are no synchronization instructions in the code
    // Everything runs at "full tilt"
    bio_ss.bio.wo(utra::bio_bdma::SFR_QDIV0, 0x1_0000);
    bio_ss.bio.wo(utra::bio_bdma::SFR_QDIV1, 0x1_0000);
    bio_ss.bio.wo(utra::bio_bdma::SFR_QDIV2, 0x1_0000);
    bio_ss.bio.wo(utra::bio_bdma::SFR_QDIV3, 0x1_0000);
    // start the machine
    bio_ss.bio.wo(utra::bio_bdma::SFR_CTRL, 0xFFF); // start all the machines to get them to stop being stupid
    print!("st\r");
    bio_ss.bio.wo(utra::bio_bdma::SFR_CTRL, 0x111); // now restart the machine of interest

    let mut main_mem_src: [u32; TEST_LEN] = [0u32; TEST_LEN];
    let mut main_mem_dst: [u32; TEST_LEN] = [0u32; TEST_LEN];
    // just conjure some locations out of thin air. Yes, these are weird addresses in decimal, meant to
    // just poke into some not page aligned location in IFRAM.
    let ifram_src =
        unsafe { core::slice::from_raw_parts_mut((utralib::HW_IFRAM0_MEM + 8200) as *mut u32, TEST_LEN) };
    let ifram_dst =
        unsafe { core::slice::from_raw_parts_mut((utralib::HW_IFRAM1_MEM + 10000) as *mut u32, TEST_LEN) };
    ifram_src.fill(0);
    ifram_dst.fill(0);
    basic_u32(&mut bio_ss, &mut main_mem_src, &mut main_mem_dst, 0, "Main->main", false);

    main_mem_src.fill(0);
    main_mem_dst.fill(0);
    basic_u32(&mut bio_ss, ifram_src, &mut main_mem_dst, 0x40, "ifram0->main", false);

    ifram_src.fill(0);
    main_mem_dst.fill(0);
    basic_u32(&mut bio_ss, &mut main_mem_src, ifram_dst, 0x80, "Main->ifram1", false);

    main_mem_src.fill(0);
    ifram_dst.fill(0);
    basic_u32(&mut bio_ss, ifram_src, ifram_dst, 0xC0, "ifram0->ifram1", false);

    print!("DMA test done.\r");

    // this sequence triggers an end of simulation on s32
    let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    test_cfg.wo(utra::csrtest::WTEST, 0xc0ded02e);
    test_cfg.wo(utra::csrtest::WTEST, 0xc0de600d);
    loop {}
}

fn basic_u32(
    bio_ss: &mut BioSharedState,
    src: &mut [u32],
    dst: &mut [u32],
    seed: u32,
    name: &'static str,
    concurrent: bool,
) -> usize {
    assert!(src.len() == dst.len());
    print!("  - {}\r", name);
    let mut tp = TestPattern::new(Some(seed));
    for d in src.iter_mut() {
        *d = tp.next();
    }
    for d in dst.iter_mut() {
        *d = tp.next();
    }
    let mut pass = 1;
    bio_ss.bio.wo(utra::bio_bdma::SFR_TXF2, src.as_ptr() as u32); // src address
    bio_ss.bio.wo(utra::bio_bdma::SFR_TXF1, dst.as_ptr() as u32); // dst address
    if !concurrent {
        bio_ss.bio.wo(utra::bio_bdma::SFR_TXF0, (src.len() * size_of::<u32>()) as u32); // bytes to move
        while bio_ss.bio.r(utra::bio_bdma::SFR_EVENT_STATUS) & 0x1 == 0 {}
        cache_flush();
        for (i, &d) in src.iter().enumerate() {
            let rbk = unsafe { dst.as_ptr().add(i).read_volatile() };
            if rbk != d {
                print!("{} DMA err @{}, {:x} rbk: {:x}\r", name, i, d, rbk);
                pass = 0;
            }
        }
    } else {
        // this flushes any read data from the cache, so that the CPU copy is forced to fetch
        // the data from memory
        cache_flush();
        let len = src.len();
        // note this kicks off a copy of only the first half of the slice
        bio_ss.bio.wo(utra::bio_bdma::SFR_TXF0, (src.len() * size_of::<u32>()) as u32 / 2);
        // the second half of the slice is copied by the CPU, simultaneously
        dst[len / 2..].copy_from_slice(&src[len / 2..]);
        cache_flush();
        // run it twice to generate more traffic
        dst[len / 2..].copy_from_slice(&src[len / 2..]);
        // ... and wait for DMA to finish, if it has not already
        while bio_ss.bio.r(utra::bio_bdma::SFR_EVENT_STATUS) & 0x1 == 0 {}
        cache_flush();
        for (i, &d) in src.iter().enumerate() {
            let rbk = unsafe { dst.as_ptr().add(i).read_volatile() };
            if rbk != d {
                print!("(c) {} DMA err @{}, {:x} rbk: {:x}\r", name, i, d, rbk);
                pass = 0;
            }
        }
    }
    pass
}

#[rustfmt::skip]
bio_code!(dma_basic2_code, DMA_BASIC2_START, DMA_BASIC2_END,
    // clear the write pipe with a gutter write
    "li x1, 0x6101FFFC", // gutter
    "sw x0, 0(x1)",
  "20:",
    "mv a3, x18",       // src address
    "mv a2, x17",       // dst address
    "li x29, 0x1",      // clear event done flag - just before the last parameter arrives
    "mv a1, x16",       // wait for # of bytes to move

    "add a4, a1, a3",   // a4 <- end condition based on source address increment

  "30:",
    "lw  t0, 0(a3)",    // blocks until load responds
    "addi a3, a3, 4",   // 3 cycles
    "sw  t0, 0(a2)",    // blocks until store completes
    "addi a2, a2, 4",   // 3 cycles
    "bne  a3, a4, 30b", // 5 cycles
    "li x28, 0x1",      // flip event done flag
    "j 20b"
);

#[rustfmt::skip]
bio_code!(idle_code, IDLE_START, IDLE_END,
  "20:",
    "j 20b"
);

#[allow(dead_code)]
fn fsfreq_to_hz(fs_freq: u32) -> u32 { (fs_freq * (48_000_000 / 32)) / 1_000_000 }

#[allow(dead_code)]
fn fsfreq_to_hz_32(fs_freq: u32) -> u32 { (fs_freq * (32_000_000 / 32)) / 1_000_000 }

#[inline(always)]
fn cache_flush() {
    unsafe {
        #[rustfmt::skip]
        core::arch::asm!(
        ".word 0x500F",
        "nop",
        "nop",
        "nop",
        "nop",
        "nop",
    );
    }
}
