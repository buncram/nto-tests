use utralib::generated::*;

use crate::*;

pub fn report_api(d: u32) {
    let mut uart = debug::Uart {};
    let mut report = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    report.wo(utra::csrtest::WTEST, d);
    uart.print_hex_word(d);
    uart.putc(0xdu8); // add a CR character
}

/// used to generate some test vectors
pub fn lfsr_next_u32(state: u32) -> u32 {
    let bit = ((state >> 31) ^ (state >> 21) ^ (state >> 1) ^ (state >> 0)) & 1;

    (state << 1) + bit
}

/* some LFSR terms
    3 3,2
    4 4,3
    5 5,3
    6 6,5
    7 7,6
    8 8,6,5,4
    9 9,5  <--
    10 10,7
    11 11,9
    12 12,6,4,1
    13 13,4,3,1
    14 14,5,3,1
    15 15,14
    16 16,15,13,4
    17 17,14
    18 18,11
    19 19,6,2,1
    20 20,17

    32 32,22,2,1:
    let bit = ((state >> 31) ^
               (state >> 21) ^
               (state >>  1) ^
               (state >>  0)) & 1;

*/
/// our desired test length is 512 entries, so pick an LFSR with a period of 2^9-1...
pub fn lfsr_next(state: u16) -> u16 {
    let bit = ((state >> 8) ^ (state >> 4)) & 1;

    ((state << 1) + bit) & 0x1_FF
}

#[allow(dead_code)]
/// shortened test length is 16 entries, so pick an LFSR with a period of 2^4-1...
pub fn lfsr_next_16(state: u16) -> u16 {
    let bit = ((state >> 3) ^ (state >> 2)) & 1;

    ((state << 1) + bit) & 0xF
}

pub fn reset_ticktimer() {
    let mut tt = CSR::new(utra::ticktimer::HW_TICKTIMER_BASE as *mut u32);
    // tt.wo(utra::ticktimer::CLOCKS_PER_TICK, 160);
    tt.wo(utra::ticktimer::CLOCKS_PER_TICK, 369560); // based on 369.56MHz default clock
    tt.wfo(utra::ticktimer::CONTROL_RESET, 1);
    tt.wo(utra::ticktimer::CONTROL, 0);
}

const RESET_TESTS: usize = 1;
crate::impl_test!(ResetValue, "Reset value", RESET_TESTS);
impl TestRunner for ResetValue {
    fn run(&mut self) {
        let resetvalue = CSR::new(utra::resetvalue::HW_RESETVALUE_BASE as *mut u32);
        let val = resetvalue.r(utra::resetvalue::PC);
        report_api(val);
        if val == 0x6000_0000 {
            self.passing_tests += 1;
        }
    }
}
