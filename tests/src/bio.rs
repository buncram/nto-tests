use xous_bio_bdma::*;

use crate::*;

const BIO_TESTS: usize = 10;
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

        self.passing_tests += bio_tests::units::hello_world();
        self.passing_tests += bio_tests::units::hello_multiverse();
        self.passing_tests += bio_tests::units::aclk_tests();
        self.passing_tests += bio_tests::units::fifo_basic();
        self.passing_tests += bio_tests::units::host_fifo_tests();

        self.passing_tests += bio_tests::spi::spi_test();

        self.passing_tests += bio_tests::i2c::i2c_test();
        self.passing_tests += bio_tests::i2c::complex_i2c_test();

        // note: this test runs without any cores, as all FIFO levels can be tested from the host directly
        self.passing_tests += bio_tests::units::fifo_level_tests();
    }
}
