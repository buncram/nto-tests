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

use xous_bio_bdma::*;

use crate::*;

const BIO_TESTS: usize =
    // get_id
    1
    // dma
    + 4 * 5 + 1
    // clocking modes
    + 4 + 4 + 4 + 4
    // stack test
    + 1
    // hello word, hello multiverse, aclk_tests
    + 3
    // fifo_basic
    + 1
    // host_fifo_tests
    + 1
    // spi_test
    + 1
    // i2c_test
    + 1
    // complex_i2c_test
    + 1
    // fifo_level_tests
    + 1
    // fifo_alias_tests
    + 1
    // event_aliases
    + 1
    // dmareq_test
    + 1
    // filter test
    + 1;

// config hack that only works for one optional feature like this :-/
#[cfg(not(feature = "bio-mul"))]
const BIO_TESTS_FINAL: usize = BIO_TESTS;
#[cfg(feature = "bio-mul")]
const BIO_TESTS_FINAL: usize = BIO_TESTS + 1;

crate::impl_test!(BioTests, "BIO", BIO_TESTS_FINAL);
impl TestRunner for BioTests {
    fn run(&mut self) {
        let id = get_id();
        crate::println!("BIO ID: {:x}", id);
        if (id >> 16) as usize == BIO_PRIVATE_MEM_LEN {
            self.passing_tests += 1;
        } else {
            crate::println!("Error: ID mem size does not match: {} != {}", id >> 16, BIO_PRIVATE_MEM_LEN);
        }

        // map the BIO ports to GPIO pins
        let iox = cramium_hal::iox::Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        iox.set_gpio_pullup(cramium_hal::iox::IoxPort::PB, 2, cramium_hal::iox::IoxEnable::Enable);
        iox.set_gpio_pullup(cramium_hal::iox::IoxPort::PB, 3, cramium_hal::iox::IoxEnable::Enable);

        self.passing_tests += bio_tests::units::hello_world();

        self.passing_tests += bio_tests::arith::stack_test();
        #[cfg(feature = "bio-mul")]
        {
            // safety: this is safe only if the target supports multiplication
            self.passing_tests += unsafe { bio_tests::arith::mac_test() }; // 1
        }

        bio_tests::dma::dma_filter_off();
        self.passing_tests += bio_tests::dma::dma_basic(false, 0); // 4
        self.passing_tests += bio_tests::dma::dma_basic(true, 0); // 4
        self.passing_tests += bio_tests::dma::dma_bytes(); // 4
        self.passing_tests += bio_tests::dma::dma_u16(); // 4
        self.passing_tests += bio_tests::dma::dma_multicore(); // 1
        self.passing_tests += bio_tests::dma::dma_coincident(0); // 4

        // test clocking modes
        crate::println!("*** CLKMODE 1 ***");
        self.passing_tests += bio_tests::dma::dma_basic(true, 1); // 4
        self.passing_tests += bio_tests::dma::dma_coincident(1); // 4
        crate::println!("*** CLKMODE 3 ***");
        self.passing_tests += bio_tests::dma::dma_basic(true, 3); // 4
        self.passing_tests += bio_tests::dma::dma_coincident(3); // 4

        self.passing_tests += bio_tests::units::aclk_tests();

        self.passing_tests += bio_tests::dma::filter_test();

        bio_tests::dma::dma_filter_off();
        self.passing_tests += bio_tests::dma::dmareq_test();
        self.passing_tests += bio_tests::units::event_aliases();
        self.passing_tests += bio_tests::units::fifo_alias_tests();

        self.passing_tests += bio_tests::units::hello_multiverse();
        self.passing_tests += bio_tests::units::fifo_basic();
        self.passing_tests += bio_tests::units::host_fifo_tests();

        self.passing_tests += bio_tests::spi::spi_test();
        self.passing_tests += bio_tests::i2c::i2c_test();
        self.passing_tests += bio_tests::i2c::complex_i2c_test();

        // note: this test runs without any cores, as all FIFO levels can be tested from the host directly
        self.passing_tests += bio_tests::units::fifo_level_tests();
    }
}
