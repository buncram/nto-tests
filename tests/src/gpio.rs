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

use cramium_hal::iox::Iox;
use cramium_api::iox::{IoxDir, IoxEnable, IoxFunction, IoxPort, IoSetup};
use utralib::generated::*;

use crate::println;
use crate::{TestBoilerplate, TestRunner};

const TOTAL_TESTS: usize = 1;
crate::impl_test!(GpioTests, "GPIO", TOTAL_TESTS);

#[allow(dead_code)]
pub const TEST_I2C_MASK: u32 = 0b00001;
pub const TEST_FORCE_MASK: u32 = 0b00010;
pub const TEST_FORCE_OFFSET: u32 = 16; // top 16 bits

impl TestRunner for GpioTests {
    fn run(&mut self) {
        let mut passing = true;

        let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
        const VALS: [u16; 6] = [0x3e65u16, 0x8A01u16, 0x0000u16, 0xFFFFu16, 0xAAAAu16, 0x5555u16];

        let iox = Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
        // co-opt the GPIO ports
        iox.set_ports_from_pio_bitmask(0);

        for p in 0..16 {
            iox.setup_pin(
                IoxPort::PB,
                p,
                Some(IoxDir::Input),
                Some(IoxFunction::Gpio),
                None,
                Some(IoxEnable::Enable),
                None,
                None,
            );
            iox.setup_pin(
                IoxPort::PC,
                p,
                Some(IoxDir::Input),
                Some(IoxFunction::Gpio),
                None,
                Some(IoxEnable::Enable),
                None,
                None,
            );
        }

        for &val in VALS.iter() {
            // setup to force, with VAL1 on the pins
            test_cfg.wo(utra::csrtest::WTEST, TEST_FORCE_MASK | (val as u32) << TEST_FORCE_OFFSET);
            // expected result:
            //    - PB equals value
            //    - PC equals inverse value
            let pb = iox.get_gpio_bank(IoxPort::PB);
            let pc = iox.get_gpio_bank(IoxPort::PC);
            println!("PB: {:x}, PC: {:x}", pb, pc);
            if pb != val {
                passing = false;
            }
            if pc != !val {
                passing = false;
            }
        }

        // reset to PIO routed ports
        test_cfg.wo(utra::csrtest::WTEST, 0);
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        if passing {
            self.passing_tests = 1;
        } else {
            self.passing_tests = 0;
        }
    }
}
