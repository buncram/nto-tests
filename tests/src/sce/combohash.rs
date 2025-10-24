use crate::*;

const SHA_TESTS: usize = 3;
crate::impl_test!(ShaTests, "Sha", SHA_TESTS);

const K_DATA: [u8; 32] = [0u8; 32];
/*
const K_DATA: &'static [u8; 853] = b"The quick brown fox jumps over the lazy dog while contemplating \
the meaning of existence in a digital world. Numbers like 123456789 and symbols @#$%^&*() add variety to this test \
message. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore \
magna aliqua. Testing patterns: ABCDEFGHIJKLMNOPQRSTUVWXYZ and abcdefghijklmnopqrstuvwxyz provide full alphabet coverage. \
Special characters !@#$%^&*()_+-=[]{}|;':.,.<>?/ enhance the diversity of this sample text. The year 2024 brings new \
challenges and opportunities for software development and testing methodologies. Random words like elephant, butterfly, \
quantum, nebula, crystalline, harmonic, and serendipity fill the remaining space. Pi equals 3.14159265358979323846 \
approximately. This text serves as a placeholder for various testing scenarios!!!";
*/

impl TestRunner for ShaTests {
    fn run(&mut self) {
        println!("== SHA tests ==");
        init_hash(false);
        println!("== hash init done ==");
        use digest::Digest;
        use hex_literal::hex;
        use sha2_bao1x_break::Sha256;

        const K_EXPECTED_DIGEST_256: [u8; 32] =
            hex!("66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925"); // 256-bits of 0
            // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 -> null hash
            // hex!("de1b3b58e16d6b12c906898025d4bc5a594075f4fd4252fa88128b2e0b7a266a"); // long data test

        let mut pass: bool = true;
        let mut hasher = Sha256::new();

        hasher.update(K_DATA);
        // hasher.update(data);
        let digest = hasher.finalize();

        for (&expected, result) in K_EXPECTED_DIGEST_256.iter().zip(digest) {
            if expected != result {
                pass = false;
            }
        }
        if pass {
            println!("Sha256 passed.");
            self.passing_tests += 1;
        } else {
            println!("Sha256 failed: {:x?}", digest);
        }

        println!("HMAC test");
        let hmac_setup: [[u32; 8]; 8] = [
            // key1
            [0x00010203, 0x4050607,0x8090a0b, 0xc0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f],
            // secret1
            [0xe8499be4, 0xf1980d68, 0xf13222a4, 0x18df5cbd, 0x97d53fdd, 0xf590c210, 0x8e22d400, 0x05b70713],
            // key2
            [0x02010203, 0x4050607,0x8090a0b, 0xc0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f],
            // secret2
            [0x1f8ee48a, 0xb2fcff56, 0x8cc87fb7, 0x2baa2e63, 0x8e92ff10, 0x1cc2eda5, 0x409040bf, 0x536d29b6],
            // key3
            [0x03010203, 0x4050607,0x8090a0b, 0xc0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f],
            // secret3
            [0x29f977e8, 0x1c8074da, 0xa703f0d8, 0x48c42fbc, 0x1dce6eb9, 0x215321c7, 0xbba8725b, 0x300a08ca],
            // key4
            [0x04010203, 0x4050607,0x8090a0b, 0xc0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f],
            // secret4
            [0xc71dba1c, 0x87bd1566, 0xd859c15a, 0x97b4c5de, 0xc6e8fd49, 0x0d0d1eb9, 0xaf312b5b, 0x4f225087]
        ];
        let hmac_key_padding: [u32; 8]=  [ 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 ];
        let hmac_msg: [u32; 16] = [0x10203, 0x4050607, 0x8090a0b, 0xc0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f, 0x80000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x300];
        let mut sce = CSR::new(utra::sce_glbsfr::HW_SCE_GLBSFR_BASE as *mut u32);
        // help wanted: SFR_SCEMODE doesn't extract because it contains a complex expression,
        // need to patch scripts/headergen/rtl_to_svd.py to handle this case! for now, just override with a raw pointer.
        unsafe{ sce.base().add(0).write_volatile(2) };
        // sce.wo(utra::sce_glbsfr::SFR_SCEMODE, 2); // enter secure mode

        // copy the keys into the key array
        let keybase = unsafe{core::slice::from_raw_parts_mut(0x603f0000 as *mut u32, 8 * 8)};
        for (i, array) in hmac_setup.iter().enumerate() {
            keybase[i * array.len()..(i + 1) * array.len()].copy_from_slice(array);
        }

        // rram needs to write 0xc00 to the rrcr

        let mut sdma = CSR::new(utra::scedma::HW_SCEDMA_BASE as *mut u32);
        let mut combohash = CSR::new(utra::combohash::HW_COMBOHASH_BASE as *mut u32);
        for i in 0..4 {
            let src = hmac_setup[i * 2].as_ptr() as u32;
            crate::println!("src {:x}", src);
            // setup key
            sdma.wo(utra::scedma::SFR_SCH_OPT, 0x100); // secure + privileged mode
            sdma.wo(utra::scedma::SFR_SCH_FUNC, 0x0); // 0 = read, 1 = write
            sdma.wo(utra::scedma::SFR_SCH_AXSTART, src);
            sdma.wo(utra::scedma::SFR_SCH_SEGID, 0x01); // segid destination
            sdma.wo(utra::scedma::SFR_SCH_TRANSIZE, 8); // transfer size = 256 bits
            sdma.wo(utra::scedma::SFR_SCH_SEGSTART, 0x0); // segment pointer
            sce.wo(utra::sce_glbsfr::SFR_FRDONE, sce.r(utra::sce_glbsfr::SFR_FRDONE));
            sdma.wo(utra::scedma::SFR_SCHSTART_AR, 0xaa); // channel start
            while sce.r(utra::sce_glbsfr::SFR_FRDONE) & 0x40 != 0x40 {
                println!("setup key");
            } // wait for sch done flag

            // setup message
            crate::println!("msg: {:x}", hmac_msg.as_ptr() as u32);
            sdma.wo(utra::scedma::SFR_SCH_OPT, 0x100); // secure + privileged mode
            sdma.wo(utra::scedma::SFR_SCH_FUNC, 0x0); // 0 = read, 1 = write
            sdma.wo(utra::scedma::SFR_SCH_AXSTART, hmac_msg.as_ptr() as u32);
            sdma.wo(utra::scedma::SFR_SCH_SEGID, 0x04); // segid destination
            sdma.wo(utra::scedma::SFR_SCH_TRANSIZE, 32); // is this right? ref code says 32 but should be 16??
            sdma.wo(utra::scedma::SFR_SCH_SEGSTART, 0x0); // segment pointer
            sce.wo(utra::sce_glbsfr::SFR_FRDONE, sce.r(utra::sce_glbsfr::SFR_FRDONE));
            sdma.wo(utra::scedma::SFR_SCHSTART_AR, 0xaa); // channel start
            while sce.r(utra::sce_glbsfr::SFR_FRDONE) & 0x40 != 0x40 {
                println!("setup msg");
            } // wait for sch done flag

            // setup padding
            crate::println!("pad: {:x}", hmac_key_padding.as_ptr() as u32);
            sdma.wo(utra::scedma::SFR_SCH_OPT, 0x100); // secure + privileged mode
            sdma.wo(utra::scedma::SFR_SCH_FUNC, 0x0); // 0 = read, 1 = write
            sdma.wo(utra::scedma::SFR_SCH_AXSTART, hmac_key_padding.as_ptr() as u32);
            sdma.wo(utra::scedma::SFR_SCH_SEGID, 0x01); // segid destination
            sdma.wo(utra::scedma::SFR_SCH_TRANSIZE, 8); // 256 bits
            sdma.wo(utra::scedma::SFR_SCH_SEGSTART, 0x8); // segment pointer
            sce.wo(utra::sce_glbsfr::SFR_FRDONE, sce.r(utra::sce_glbsfr::SFR_FRDONE));
            sdma.wo(utra::scedma::SFR_SCHSTART_AR, 0xaa); // channel start
            while sce.r(utra::sce_glbsfr::SFR_FRDONE) & 0x40 != 0x40 {
                println!("setup padding");
            } // wait for sch done flag

            // pass 1
            combohash.wo(utra::combohash::SFR_CRFUNC, 0x50); // pass 1
            combohash.wo(utra::combohash::SFR_OPT1, 0x0);
            combohash.wo(utra::combohash::SFR_OPT2, 0x4);
            combohash.wo(utra::combohash::SFR_OPT3, 0x0);
            combohash.wo(utra::combohash::SFR_FR, combohash.r(utra::combohash::SFR_FR));
            combohash.wo(utra::combohash::SFR_AR, 0x5a);
            while combohash.r(utra::combohash::SFR_FR) & 0x1 != 0x1 {
                println!("pass1");
            } // wait for sch done flag

            combohash.wo(utra::combohash::SFR_KEYIDX, i as u32 * 2 + 1); // check keyslot 1, 3, 5, 7
            combohash.wo(utra::combohash::SFR_CRFUNC, 0x60); // pass 2
            combohash.wo(utra::combohash::SFR_OPT1, 0x0);
            combohash.wo(utra::combohash::SFR_OPT2, 0x25); // ts mode & check secret
            combohash.wo(utra::combohash::SFR_OPT3, 0x0);
            combohash.wo(utra::combohash::SFR_FR, combohash.r(utra::combohash::SFR_FR));
            combohash.wo(utra::combohash::SFR_AR, 0x5a);
            while combohash.r(utra::combohash::SFR_FR) & 0x1 != 0x1 {
                println!("pass2");
            } // wait for sch done flag
            crate::println!("secret check flag {:x}", combohash.r(utra::combohash::SFR_FR));

            crate::println!("trust state");
            // FIXME: wrong number of registers are extracted because the PARAM setting
            // is 128 in the file but it's overwridden to 256 elsewhere and we're not picking
            // that up...
            for i in 0 ..8 {
                crate::println!("  {:x}", unsafe{ sce.base().add(56 + i).read_volatile() });
            }
        }
        // leave SCE mode so hash can run "normally" again
        unsafe{ sce.base().add(0).write_volatile(0) };

        println!("Tamper with initial round constants...");
        init_hash(true);
        let mut hasher = Sha256::new();

        hasher.update(K_DATA);
        // hasher.update(data);
        let digest = hasher.finalize();

        for (&expected, result) in K_EXPECTED_DIGEST_256.iter().zip(digest) {
            if expected != result {
                pass = false;
            }
        }
        if pass {
            println!("Sha256 passed.");
            self.passing_tests += 1;
        } else {
            println!("Sha256 failed: {:x?}", digest);
        }
    }
}

/// This function loads all the round constants into the combohasher's local memory.
pub fn init_hash(malicious: bool) {
    use bao1x_api::sce::combohash::*;
    // safety: this one is a little less clear from the register set extraction. But in the case of
    // system initialization, the SCE uses its entire RAM range (10kiB worth) as a single buffer, so
    // none of the buffer boundaries are respected. Thus we set the length of the segment to 10kiB
    // solely because as hardware designers, we know this is what's there. You can see the size of
    // the SCERAM's block definition via its RBIST wrapper here:
    // https://github.com/baochip/baochip-1x/blob/96ba390759ba361e50e57bd21f02c806ddafc4ff/rtl/modules/soc_coresub/rtl/soc_coresub.sv#L1018
    let sce_mem = unsafe {
        core::slice::from_raw_parts_mut(utralib::HW_SEG_LKEY_MEM as *mut u32, 10 * 1024 / size_of::<u32>())
    };
    #[rustfmt::skip]
    let constants =
        SHA256_H.iter().chain(
        SHA256_K.iter().chain(
        SHA512_H.iter().chain(
        SHA512_K.iter().chain(
        BLK2S_H.iter().chain(
        BLK2B_H.iter().chain(
        BLK2_X.iter().chain(
        BLK3_H.iter().chain(
        BLK3_X.iter().chain(
        RIPMD_H.iter().chain(
        RIPMD_K.iter().chain(
        RIPMD_X.iter().chain(
        RAMSEG_SHA3.iter()
    ))))))))))));
    for (dst, &src) in sce_mem.iter_mut().zip(constants) {
        if malicious {
            *dst = 0;
        } else {
            *dst = src;
        }
    }
    let mut combo_hash = CSR::new(utra::combohash::HW_COMBOHASH_BASE as *mut u32);
    combo_hash.wo(utra::combohash::SFR_OPT3, 0); // u32 big-endian constant load
    combo_hash.wfo(utra::combohash::SFR_CRFUNC_CR_FUNC, HashFunction::Init as u32);

    combo_hash.wo(utra::combohash::SFR_FR, 0xf); // clear completion flag
    combo_hash.wo(utra::combohash::SFR_AR, 0x5a); // start
    while combo_hash.rf(utra::combohash::SFR_FR_MFSM_DONE) == 0 {
        // wait for mem to copy
    }
    // clear the flag on exit
    combo_hash.rmwf(utra::combohash::SFR_FR_MFSM_DONE, 1);
}
