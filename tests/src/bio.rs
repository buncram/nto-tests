use crate::*;

const BIO_TESTS: usize = 1;
crate::impl_test!(BioTests, "BIO", BIO_TESTS);
impl TestRunner for BioTests {
    fn run(&mut self) {
        // TODO: break this into separate tests
        xous_bio_bdma::bio_tests::bio_tests();
        self.passing_tests += 1;
    }
}
