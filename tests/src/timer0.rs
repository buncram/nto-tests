use utralib::generated::*;

use crate::*;

const TIMER0_TESTS: usize = 2;
crate::impl_test!(Timer0Tests, "Timer0", TIMER0_TESTS);
impl TestRunner for Timer0Tests {
    /// Note: this implementation is dependent upon the `irq` module!
    fn run(&mut self) {
        let mut timer0 = CSR::new(utra::timer0::HW_TIMER0_BASE as *mut u32);

        timer0.wfo(utra::timer0::EN_EN, 0);
        timer0.wo(utra::timer0::RELOAD, 1000);
        timer0.wo(utra::timer0::LOAD, 1000);
        timer0.wfo(utra::timer0::EN_EN, 1);
        timer0.wfo(utra::timer0::EV_PENDING_ZERO, 1);

        // trivial test
        timer0.wo(utra::timer0::UPDATE_VALUE, 1); // latch the latest value
        let value_a = timer0.r(utra::timer0::VALUE);
        timer0.wo(utra::timer0::UPDATE_VALUE, 1); // latch another value
        let value_b = timer0.r(utra::timer0::VALUE);
        if value_b < value_a {
            self.passing_tests += 1;
        } else {
            crate::println!("TIMER0 did not decrement: {}->{}", value_b, value_a);
        }

        // make sure the timer is setup for the next test
        timer0.wfo(utra::timer0::EN_EN, 0);
        timer0.wo(utra::timer0::RELOAD, 1000);
        timer0.wo(utra::timer0::LOAD, 1000);
        timer0.wfo(utra::timer0::EN_EN, 1);

        timer0.wfo(utra::timer0::EV_PENDING_ZERO, 1);
        timer0.wfo(utra::timer0::EV_ENABLE_ZERO, 1);

        // wait for the interrupt to happen
        let mut timeout = 0;
        loop {
            if timer0.r(utra::timer0::RELOAD) == 10_000 {
                // the enable will be de-activated by the interrupt handler
                crate::println!("TIMER0 interrupt caught");
                self.passing_tests += 1;
                break;
            }
            timeout += 1;
            if timeout >= 1000 {
                // timeout without incrementing passing_tests
                crate::println!("TIMER0 timed out");
                break;
            }
        }
        timer0.wfo(utra::timer0::EN_EN, 0);
        timer0.wfo(utra::timer0::EV_PENDING_ZERO, 0);
        timer0.wfo(utra::timer0::EV_ENABLE_ZERO, 0);
    }
}
