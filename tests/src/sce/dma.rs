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

use utralib::generated::*;

use crate::debug;
use crate::utils::*;
use crate::{TestBoilerplate, TestRunner};

const SCE_TESTS: usize = 2;
crate::impl_test!(SceDmaTests, "SCE DMA", SCE_TESTS);
impl TestRunner for SceDmaTests {
    fn run(&mut self) { self.passing_tests += sce_dma_tests(); }
}

pub fn sce_dma_tests() -> usize {
    let mut passing = 0;
    let mut uart = debug::Uart {};
    let mut sce_ctl_csr = CSR::new(utra::sce_glbsfr::HW_SCE_GLBSFR_BASE as *mut u32);
    sce_ctl_csr.wfo(utra::sce_glbsfr::SFR_SUBEN_CR_SUBEN, 0x1F);
    let mut sdma_csr = CSR::new(utra::scedma::HW_SCEDMA_BASE as *mut u32);
    const BLOCKLEN: usize = 16; // blocks must be pre-padded or of exactly this length
    const DMA_LEN: usize = BLOCKLEN; // FIFO buffers
    let sk: [u32; 72] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5, 0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
        0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3, 0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
        0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC, 0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
        0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7, 0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
        0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13, 0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
        0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3, 0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
        0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5, 0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
        0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208, 0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2,
    ];
    uart.tiny_write_str("init hash\r");
    // setup the sk region
    let sk_mem = unsafe { core::slice::from_raw_parts_mut(utralib::HW_SEG_LKEY_MEM as *mut u32, sk.len()) };
    // zeroize
    for d in sk_mem.iter_mut() {
        *d = 0;
    }
    // then init hash value
    sk_mem[..sk.len()].copy_from_slice(&sk);

    // setup the SCEDMA to do a simple transfer between two memory regions
    let mut region_a = [0u32; DMA_LEN];
    let region_b = [0u32; DMA_LEN];
    let region_c = [0u32; DMA_LEN];
    if false {
        let mut state = 0xF0F0_A0A0;
        for d in region_a.iter_mut() {
            *d = state;
            state = lfsr_next_u32(state);
        }
    } else {
        for d in region_a.iter_mut() {
            *d = 0x9999_9999; // palindromic value just to rule out endianness in first testing
        }
    }

    uart.tiny_write_str("init done\r");
    // enable the hash FIFO (bit 1) -- this must happen first
    sce_ctl_csr.wfo(utra::sce_glbsfr::SFR_FFEN_CR_FFEN, 0b00010);

    // -------- combohash tests --------
    let mut hash_csr = CSR::new(utra::combohash::HW_COMBOHASH_BASE as *mut u32);
    hash_csr.wfo(utra::combohash::SFR_CRFUNC_CR_FUNC, 0); // HF_SHA256
    hash_csr.wfo(utra::combohash::SFR_OPT1_CR_OPT_HASHCNT, 0); // run the hash on two DMA blocks
    hash_csr.wfo(utra::combohash::SFR_OPT2_CR_OPT_IFSTART, 1); // start from 1st block
    hash_csr.rmwf(utra::combohash::SFR_OPT2_CR_OPT_IFSOB, 1); // write data to seg-sob when done
    hash_csr.wfo(utra::combohash::SFR_SEGPTR_SEGID_MSG_SEGID_MSG, 0); // message goes from location 0
    hash_csr.wfo(utra::combohash::SFR_SEGPTR_SEGID_HOUT_SEGID_HOUT, 0); // message goes to location in HOUT area
    // trigger start hash, but it should wait until the DMA runs
    hash_csr.wfo(utra::combohash::SFR_AR_SFR_AR, 0x5A);

    // dma the data in region_a to the hash engine; device should automatically ensure no buffers are
    // overfilled
    sdma_csr.wfo(utra::scedma::SFR_XCH_AXSTART_XCHCR_AXSTART, region_a.as_ptr() as u32);
    sdma_csr.wfo(utra::scedma::SFR_XCH_OPT_XCHCR_OPT, 0b1_0000); // endian swap
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGID_XCHCR_SEGID, 4); // HASH_MSG region
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGSTART_XCHCR_SEGSTART, 0);
    sdma_csr.wfo(utra::scedma::SFR_XCH_TRANSIZE_XCHCR_TRANSIZE, DMA_LEN as u32);
    sdma_csr.wfo(utra::scedma::SFR_SCH_FUNC_SCHCR_FUNC, 0); // 0 == AXI read, 1 == AXI write
    sdma_csr.wfo(utra::scedma::SFR_SCHSTART_AR_SFR_SCHSTART_AR, 0xA5); // 0x5a ich start, 0xa5 xch start, 0xaa sch start

    // observe the hash done output
    for _ in 0..2 {
        uart.print_hex_word(sce_ctl_csr.r(utra::combohash::SFR_FR));
        uart.tiny_write_str(" <- hash FR\r")
    }

    // print the hash output
    let hout_mem = unsafe {
        core::slice::from_raw_parts(
            utralib::HW_SEG_HOUT_MEM as *mut u32,
            utralib::HW_SEG_HOUT_MEM_LEN / core::mem::size_of::<u32>(),
        )
    };
    sce_ctl_csr.wfo(utra::sce_glbsfr::SFR_APBS_CR_APBSOPT, 0b1_0000); // endian swap APB read
    uart.tiny_write_str("HOUT (BE): ");
    for i in 0..8 {
        // should be big-endian
        uart.print_hex_word(hout_mem[i]);
    }
    uart.tiny_write_str("\r");
    sce_ctl_csr.wfo(utra::sce_glbsfr::SFR_APBS_CR_APBSOPT, 0b0_0000);
    uart.tiny_write_str("HOUT (LE): ");
    for i in 0..8 {
        // should be big-endian
        uart.print_hex_word(hout_mem[i]);
    }
    uart.tiny_write_str("\r");

    uart.tiny_write_str("HIN ");
    for d in region_a {
        // big-endian, so make it one big string
        uart.print_hex_word(d);
    }
    uart.tiny_write_str("\r");

    // -------- AES tests ---------
    // fifo 2 = AES in, fifo 3 = AES out -- this must happen first
    sce_ctl_csr.wfo(utra::sce_glbsfr::SFR_FFEN_CR_FFEN, 0b00100);

    // make sure that the destination is empty
    let mut errs = 0;
    for (src, dst) in region_a.iter().zip(region_b.iter()) {
        if *src != *dst {
            errs += 1;
        }
    }
    if errs == 0 {
        passing += 1;
    }
    uart.tiny_write_str("dest mismatch count (should not be 0): ");
    uart.print_hex_word(errs);
    uart.tiny_write_str("\r");

    let mut aes_csr = CSR::new(utra::aes::HW_AES_BASE as *mut u32);
    // schedule the 0-key
    aes_csr.wo(utra::aes::SFR_SEGPTR_PTRID_AKEY, 0);
    aes_csr.rmwf(utra::aes::SFR_OPT_OPT_KLEN0, 0b10); // 256 bit key
    aes_csr.rmwf(utra::aes::SFR_OPT_OPT_MODE0, 0b000); // ECB
    aes_csr.wfo(utra::aes::SFR_CRFUNC_SFR_CRFUNC, 0x0); // AES-KS
    aes_csr.wo(utra::aes::SFR_AR, 0x5a);
    uart.tiny_write_str("AES KS\r");

    // setup the encryption
    aes_csr.wo(utra::aes::SFR_SEGPTR_PTRID_AIB, 0);
    aes_csr.wo(utra::aes::SFR_SEGPTR_PTRID_AOB, 0);
    aes_csr.rmwf(utra::aes::SFR_OPT_OPT_KLEN0, 0b10); // 256 bit key
    aes_csr.rmwf(utra::aes::SFR_OPT_OPT_MODE0, 0b000); // ECB
    aes_csr.wfo(utra::aes::SFR_CRFUNC_SFR_CRFUNC, 0x1); // AES-ENC

    // start the AES op, should not run until FIFO fills data...
    uart.tiny_write_str("start AES op\r");
    aes_csr.wfo(utra::aes::SFR_OPT1_SFR_OPT1, DMA_LEN as u32 / (128 / 32));
    aes_csr.wo(utra::aes::SFR_AR, 0x5a);

    // dma the data in region_a to the AES engine
    sdma_csr.wfo(utra::scedma::SFR_XCH_AXSTART_XCHCR_AXSTART, region_a.as_ptr() as u32);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGID_XCHCR_SEGID, 14); // 13 AKEY, 14 AIB, 15, AOB
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGSTART_XCHCR_SEGSTART, 0);
    sdma_csr.wfo(utra::scedma::SFR_XCH_TRANSIZE_XCHCR_TRANSIZE, DMA_LEN as u32);
    sdma_csr.wfo(utra::scedma::SFR_SCH_FUNC_SCHCR_FUNC, 0); // 0 == AXI read, 1 == AXI write
    sdma_csr.wfo(utra::scedma::SFR_SCHSTART_AR_SFR_SCHSTART_AR, 0xA5); // 0x5a ich start, 0xa5 xch start, 0xaa sch start

    uart.tiny_write_str("scdma op 1 in progress\r"); // waste some time while the DMA runs...
    // while sce_ctl_csr.rf(utra::sce_glbsfr::SFR_SRBUSY_SR_BUSY) != 0 {
    uart.print_hex_word(sce_ctl_csr.rf(utra::sce_glbsfr::SFR_SRBUSY_SR_BUSY));
    uart.tiny_write_str(" ");
    uart.print_hex_word(sce_ctl_csr.rf(utra::sce_glbsfr::SFR_FRDONE_FR_DONE));
    uart.tiny_write_str(" waiting\r");
    // }

    // wait for aes op to be done
    // while aes_csr.rf(utra::sce_glbsfr::SFR_FRDONE_FR_DONE) != 0 {
    uart.print_hex_word(aes_csr.rf(utra::aes::SFR_SEGPTR_PTRID_AOB_PTRID_AOB));
    uart.tiny_write_str(" aes waiting\r");
    // }

    // dma the data in region_b from the segment
    sdma_csr.wfo(utra::scedma::SFR_XCH_AXSTART_XCHCR_AXSTART, region_b.as_ptr() as u32);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGID_XCHCR_SEGID, 15);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGSTART_XCHCR_SEGSTART, 0);
    sdma_csr.wfo(utra::scedma::SFR_XCH_TRANSIZE_XCHCR_TRANSIZE, DMA_LEN as u32);
    sdma_csr.wfo(utra::scedma::SFR_SCH_FUNC_SCHCR_FUNC, 1); // 0 == AXI read, 1 == AXI write
    sdma_csr.wfo(utra::scedma::SFR_SCHSTART_AR_SFR_SCHSTART_AR, 0xA5); // 0x5a ich start, 0xa5 xch start, 0xaa sch start
    uart.tiny_write_str("scdma op 2 in progress\r"); // waste some time while the DMA runs...

    // flush the cache, otherwise we won't see the updated values in region_b
    unsafe {
        core::arch::asm!(".word 0x500F", "nop", "nop", "nop", "nop", "nop",);
    }

    for (i, (src, dst)) in region_a.iter().zip(region_b.iter()).enumerate() {
        if *src != *dst {
            uart.tiny_write_str("error in iter ");
            uart.print_hex_word(i as u32);
            uart.tiny_write_str(": ");
            uart.print_hex_word(*src);
            uart.tiny_write_str(" s<->d ");
            uart.print_hex_word(*dst);
            uart.tiny_write_str("\r");
            break; // just print something so we can know the intermediate is "ok"
        }
    }

    // decode the data to see if it's at least symmetric
    aes_csr.wfo(utra::aes::SFR_CRFUNC_SFR_CRFUNC, 0x2); // AES-DEC

    // dma the data in region_a to the AES engine
    sdma_csr.wfo(utra::scedma::SFR_XCH_AXSTART_XCHCR_AXSTART, region_b.as_ptr() as u32);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGID_XCHCR_SEGID, 14); // 13 AKEY, 14 AIB, 15, AOB
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGSTART_XCHCR_SEGSTART, 0);
    sdma_csr.wfo(utra::scedma::SFR_XCH_TRANSIZE_XCHCR_TRANSIZE, DMA_LEN as u32);
    sdma_csr.wfo(utra::scedma::SFR_SCH_FUNC_SCHCR_FUNC, 0); // 0 == AXI read, 1 == AXI write
    sdma_csr.wfo(utra::scedma::SFR_SCHSTART_AR_SFR_SCHSTART_AR, 0xA5); // 0x5a ich start, 0xa5 xch start, 0xaa sch start

    // start the AES op
    uart.tiny_write_str("start AES op\r");
    aes_csr.wfo(utra::aes::SFR_OPT1_SFR_OPT1, DMA_LEN as u32 / (128 / 32));
    aes_csr.wo(utra::aes::SFR_AR, 0x5a);
    uart.tiny_write_str("scdma op 3 in progress\r"); // waste some time while the DMA runs...

    // dma the data in region_b from the segment
    sdma_csr.wfo(utra::scedma::SFR_XCH_AXSTART_XCHCR_AXSTART, region_c.as_ptr() as u32);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGID_XCHCR_SEGID, 15);
    sdma_csr.wfo(utra::scedma::SFR_XCH_SEGSTART_XCHCR_SEGSTART, 0);
    sdma_csr.wfo(utra::scedma::SFR_XCH_TRANSIZE_XCHCR_TRANSIZE, DMA_LEN as u32);
    sdma_csr.wfo(utra::scedma::SFR_SCH_FUNC_SCHCR_FUNC, 1); // 0 == AXI read, 1 == AXI write
    sdma_csr.wfo(utra::scedma::SFR_SCHSTART_AR_SFR_SCHSTART_AR, 0xA5); // 0x5a ich start, 0xa5 xch start, 0xaa sch start
    uart.tiny_write_str("scdma op 4 in progress\r"); // waste some time while the DMA runs...

    // flush the cache, otherwise we won't see the updated values in region_b
    unsafe {
        core::arch::asm!(".word 0x500F", "nop", "nop", "nop", "nop", "nop",);
    }

    errs = 0;
    // compare a to c: these should now be identical, with enc->dec
    for (i, (src, dst)) in region_a.iter().zip(region_c.iter()).enumerate() {
        if *src != *dst {
            uart.tiny_write_str("error in iter ");
            uart.print_hex_word(i as u32);
            uart.tiny_write_str(": ");
            uart.print_hex_word(*src);
            uart.tiny_write_str(" s<->d ");
            uart.print_hex_word(*dst);
            uart.tiny_write_str("\r");
            errs += 1;
        }
    }
    uart.tiny_write_str("errs: ");
    uart.print_hex_word(errs);
    uart.tiny_write_str("\r");
    if errs != 0 {
        passing += 1;
    }

    passing
}
