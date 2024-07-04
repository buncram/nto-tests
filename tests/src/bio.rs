use xous_bio_bdma::*;

use crate::*;

const BIO_TESTS: usize =
    // get_id
    1
    // dma
    + 4 * 4 + 1
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
    + 1;
crate::impl_test!(BioTests, "BIO", BIO_TESTS);
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
        let mut iox = cramium_hal::iox::Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        iox.set_gpio_pullup(cramium_hal::iox::IoxPort::PB, 2, cramium_hal::iox::IoxEnable::Enable);
        iox.set_gpio_pullup(cramium_hal::iox::IoxPort::PB, 3, cramium_hal::iox::IoxEnable::Enable);

        self.passing_tests += bio_tests::units::hello_world();

        self.passing_tests += bio_tests::dma::dmareq_test();
        self.passing_tests += bio_tests::units::event_aliases();
        self.passing_tests += bio_tests::units::fifo_alias_tests();

        self.passing_tests += bio_tests::dma::dma_basic(); // 4
        self.passing_tests += bio_tests::dma::dma_bytes(); // 4
        self.passing_tests += bio_tests::dma::dma_u16(); // 4
        self.passing_tests += bio_tests::dma::dma_multicore(); // 1
        self.passing_tests += bio_tests::dma::dma_coincident(); // 4

        self.passing_tests += bio_tests::arith::stack_test();

        self.passing_tests += bio_tests::units::hello_multiverse();
        self.passing_tests += bio_tests::units::aclk_tests();
        self.passing_tests += bio_tests::units::fifo_basic();
        self.passing_tests += bio_tests::units::host_fifo_tests();

        self.passing_tests += bio_tests::spi::spi_test();

        self.passing_tests += bio_tests::i2c::i2c_test();
        self.passing_tests += bio_tests::i2c::complex_i2c_test();

        // note: this test runs without any cores, as all FIFO levels can be tested from the host directly
        self.passing_tests += bio_tests::units::fifo_level_tests();

        // this sequence triggers an end of simulation on s32
        let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
        test_cfg.wo(utra::csrtest::WTEST, 0xc0ded02e);
        test_cfg.wo(utra::csrtest::WTEST, 0xc0de600d);
    }
}
