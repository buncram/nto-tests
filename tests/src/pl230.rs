use crate::*;

const PL230_TESTS: usize = 1;
crate::impl_test!(Pl230Tests, "PL230", PL230_TESTS);
impl TestRunner for Pl230Tests {
    fn run(&mut self) {
        // setup IOs for the PL230 test outputs
        let iox_csr = utra::iox::HW_IOX_BASE as *mut u32;
        unsafe {
            iox_csr.add(0x8 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBL
            iox_csr.add(0xC / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PBH
            iox_csr.add(0x10 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCL
            iox_csr.add(0x14 / core::mem::size_of::<u32>()).write_volatile(0b0101_0101_0101_0101); // PCH
            iox_csr.add(0x200 / core::mem::size_of::<u32>()).write_volatile(0xffffffff); // PIO sel port D31-0
        }
        xous_pl230::pl230_tests::pl230_tests();
        self.passing_tests += 1;
    }
}
