use core::convert::TryFrom;
use core::convert::TryInto;
use core::mem::size_of;

use utralib::generated::*;

use crate::satp;
use crate::utils::*;

pub unsafe fn check_byte_strobes() {
    let u8_test = utra::duart::HW_DUART_BASE as *mut u8;
    let u16_test = utra::duart::HW_DUART_BASE as *mut u16;

    // quick test to check byte and word write strobes on the
    unsafe {
        u8_test.write_volatile(0x31);
        u8_test.add(1).write_volatile(32);
        u8_test.add(2).write_volatile(33);
        u8_test.add(3).write_volatile(34);

        u16_test.write_volatile(0x44);
        u16_test.add(1).write_volatile(0x55);
    }
}
pub unsafe fn caching_tests() {
    // test of the 0x500F cache flush instruction - this requires manual inspection of the report values
    report_api(0x000c_ac7e);
    const CACHE_WAYS: usize = 4;
    const CACHE_SET_SIZE: usize = 4096 / size_of::<u32>();
    let test_slice = core::slice::from_raw_parts_mut(satp::PT_LIMIT as *mut u32, CACHE_SET_SIZE * CACHE_WAYS);
    // bottom of cache
    for set in 0..4 {
        report_api((&mut test_slice[set * CACHE_SET_SIZE] as *mut u32) as u32);
        (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).write_volatile(0x0011_1111 * (1 + set as u32));
    }
    // top of cache
    for set in 0..4 {
        report_api((&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32) as u32);
        (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32)
            .write_volatile(0x1100_2222 * (1 + set as u32));
    }
    // read cached values - first iteration populates the cache; second iteration should be cached
    for iter in 0..2 {
        report_api(0xb1d0_0000 + iter + 1);
        for set in 0..4 {
            let a = (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).read_volatile();
            report_api(a);
            let b = (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32).read_volatile();
            report_api(b);
        }
    }
    // flush cache
    report_api(0xff00_ff00);
    core::arch::asm!(".word 0x500F",);
    report_api(0x0f0f_0f0f);
    // read cached values - first iteration populates the cache; second iteration should be cached
    for iter in 0..2 {
        report_api(0xb2d0_0000 + iter + 1);
        for set in 0..4 {
            let a = (&mut test_slice[set * CACHE_SET_SIZE] as *mut u32).read_volatile();
            report_api(a);
            let b = (&mut test_slice[set * CACHE_SET_SIZE + CACHE_SET_SIZE - 1] as *mut u32).read_volatile();
            report_api(b);
        }
    }
    report_api(0x600c_ac7e);

    // check that caching is disabled for I/O regions
    report_api(0xc520_0000);
    let mut csrtest = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
    let mut passing = true;
    for i in 0..4 {
        csrtest.wfo(utra::csrtest::WTEST_WTEST, i);
        let val = csrtest.rf(utra::csrtest::RTEST_RTEST);
        report_api(val);
        if val != i + 0x1000_0000 {
            passing = false;
        }
    }
    if passing {
        report_api(0xc520_600d);
    } else {
        report_api(0xc520_dead);
    }
}

pub unsafe fn ram_tests() {
    const BASE_ADDR: u32 = satp::PT_LIMIT as u32; // don't overwrite our PT data
    // 'random' access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 512);
    ramtest_lfsr(&mut test_slice, 3);

    // now some basic memory read/write tests
    // entirely within cache access test
    // 256-entry by 32-bit slice at start of RAM
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 256);
    ramtest_all(&mut test_slice, 4);
    // byte access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u8, 256);
    ramtest_fast(&mut test_slice, 5);
    // word access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u16, 512);
    ramtest_fast(&mut test_slice, 6); // 1ff00

    // outside cache test
    // 6144-entry by 32-bit slice at start of RAM - should cross outside cache boundary
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 0x1800);
    ramtest_fast(&mut test_slice, 7); // c7f600

    // this passed, now that the AXI state machine is fixed.
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u32, 0x1800);
    ramtest_fast_specialcase1(&mut test_slice, 8); // c7f600

    // u64 access test
    let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u64, 0xC00);
    ramtest_fast(&mut test_slice, 9);

    // random size/access test
    // let mut test_slice = core::slice::from_raw_parts_mut(BASE_ADDR as *mut u8, 0x6000);
}

/// chunks through the entire bank of data
pub unsafe fn ramtest_all<T>(test_slice: &mut [T], test_index: u32)
where
    T: TryFrom<usize> + TryInto<u32> + Default + Copy,
{
    let mut sum: u32 = 0;
    for (index, d) in test_slice.iter_mut().enumerate() {
        // Convert the element into a `u32`, failing
        (d as *mut T).write_volatile(index.try_into().unwrap_or_default());
        sum += TryInto::<u32>::try_into(index).unwrap();
    }
    let mut checksum: u32 = 0;
    for d in test_slice.iter() {
        let a = (d as *const T).read_volatile().try_into().unwrap_or_default();
        checksum += a;
        // report_api(a);
    }

    if sum == checksum {
        report_api(checksum as u32);
        report_api(0x600d_0000 + test_index);
    } else {
        report_api(checksum as u32);
        report_api(sum as u32);
        report_api(0x0bad_0000 + test_index);
    }
}

/// only touches two words on each cache line
/// this one tries to write the same word twice to two consecutive addresses
/// this causes the valid strobe to hit twice in a row. seems to pass.
pub unsafe fn ramtest_fast_specialcase1<T>(test_slice: &mut [T], test_index: u32)
where
    T: TryFrom<usize> + TryInto<u32> + Default + Copy,
{
    const CACHE_LINE_SIZE: usize = 32;
    let mut sum: u32 = 0;
    for (index, d) in test_slice.chunks_mut(CACHE_LINE_SIZE / size_of::<T>()).enumerate() {
        let idxp1 = index + 0;
        // unroll the loop to force b2b writes
        sum += TryInto::<u32>::try_into(index).unwrap();
        sum += TryInto::<u32>::try_into(idxp1).unwrap();
        // Convert the element into a `u32`, failing
        (d.as_mut_ptr() as *mut T).write_volatile(index.try_into().unwrap_or_default());
        // Convert the element into a `u32`, failing
        (d.as_mut_ptr().add(1) as *mut T).write_volatile(idxp1.try_into().unwrap_or_default());
    }
    let mut checksum: u32 = 0;
    for d in test_slice.chunks(CACHE_LINE_SIZE / size_of::<T>()) {
        checksum += (d.as_ptr() as *const T).read_volatile().try_into().unwrap_or_default();
        checksum += (d.as_ptr().add(1) as *const T).read_volatile().try_into().unwrap_or_default();
    }

    if sum == checksum {
        report_api(checksum as u32);
        report_api(0x600d_0000 + test_index);
    } else {
        report_api(checksum as u32);
        report_api(sum as u32);
        report_api(0x0bad_0000 + test_index);
    }
}

/// only touches two words on each cache line
pub unsafe fn ramtest_fast<T>(test_slice: &mut [T], test_index: u32)
where
    T: TryFrom<usize> + TryInto<u32> + Default + Copy,
{
    const CACHE_LINE_SIZE: usize = 32;
    let mut sum: u32 = 0;
    for (index, d) in test_slice.chunks_mut(CACHE_LINE_SIZE / size_of::<T>()).enumerate() {
        let idxp1 = index + 1;
        // unroll the loop to force b2b writes
        sum += TryInto::<u32>::try_into(index).unwrap();
        sum += TryInto::<u32>::try_into(idxp1).unwrap();
        // Convert the element into a `u32`, failing
        (d.as_mut_ptr() as *mut T).write_volatile(index.try_into().unwrap_or_default());
        // Convert the element into a `u32`, failing
        (d.as_mut_ptr().add(1) as *mut T).write_volatile(idxp1.try_into().unwrap_or_default());
    }
    let mut checksum: u32 = 0;
    for d in test_slice.chunks(CACHE_LINE_SIZE / size_of::<T>()) {
        let a = (d.as_ptr() as *const T).read_volatile().try_into().unwrap_or_default();
        let b = (d.as_ptr().add(1) as *const T).read_volatile().try_into().unwrap_or_default();
        checksum = checksum + a + b;
        // report_api(a);
        // report_api(b);
    }

    if sum == checksum {
        report_api(checksum as u32);
        report_api(0x600d_0000 + test_index);
    } else {
        report_api(checksum as u32);
        report_api(sum as u32);
        report_api(0x0bad_0000 + test_index);
    }
}

/// uses an LFSR to cycle through "random" locations. The slice length
/// should equal the (LFSR period+1), so that we guarantee that each entry
/// is visited once.
pub unsafe fn ramtest_lfsr<T>(test_slice: &mut [T], test_index: u32)
where
    T: TryFrom<usize> + TryInto<u32> + Default + Copy,
{
    if test_slice.len() != 512 {
        report_api(0x0bad_000 + test_index + 0x0F00); // indicate a failure due to configuration
        return;
    }
    let mut state: u16 = 1;
    let mut sum: u32 = 0;
    const MAX_STATES: usize = 511;
    (&mut test_slice[0] as *mut T).write_volatile(0.try_into().unwrap_or_default()); // the 0 index is never written to by this, initialize it to 0
    for i in 0..MAX_STATES {
        let wr_val = i * 3;
        (&mut test_slice[state as usize] as *mut T).write_volatile(wr_val.try_into().unwrap_or_default());
        sum += wr_val as u32;
        state = lfsr_next(state);
    }

    // flush cache
    report_api(0xff00_ff00);
    core::arch::asm!(".word 0x500F",);
    report_api(0x0f0f_0f0f);

    // we should be able to just iterate in-order and sum all the values, and get the same thing back as above
    let mut checksum: u32 = 0;
    for d in test_slice.iter() {
        let a = (d as *const T).read_volatile().try_into().unwrap_or_default();
        checksum += a;
        // report_api(a);
    }

    if sum == checksum {
        report_api(checksum as u32);
        report_api(0x600d_0000 + test_index);
    } else {
        report_api(checksum as u32);
        report_api(sum as u32);
        report_api(0x0bad_0000 + test_index);
    }
}

pub fn xip_test() {
    report_api(0x61D0_0000);
    // a code snippet that adds 0x400 to the argument and returns
    let code = [0x4005_0513u32, 0x0000_8082u32];

    // shove it into the XIP region
    let xip_dest = unsafe { core::slice::from_raw_parts_mut(satp::XIP_VA as *mut u32, 2) };
    xip_dest.copy_from_slice(&code);

    // run the code
    let mut test_val: usize = 0x5555_0000;
    let mut expected: usize = test_val;
    for _ in 0..8 {
        test_val = crate::asm::jmp_remote(test_val, satp::XIP_VA);
        report_api(test_val as u32);
        expected += 0x0400;
        assert!(expected == test_val);
    }

    // prep a second region, a little bit further away to trigger a second access
    // self-modifying code is *not* supported on Vex
    const XIP_OFFSET: usize = 0;
    let xip_dest2 = unsafe { core::slice::from_raw_parts_mut((satp::XIP_VA + XIP_OFFSET) as *mut u32, 2) };
    let code2 = [0x0015_0513u32, 0x0000_8082u32];
    xip_dest2.copy_from_slice(&code2);
    // this forces a reload of the i-cache
    unsafe {
        core::arch::asm!("fence.i",);
    }

    // run the new code and see that it was updated?
    for _ in 0..8 {
        test_val = crate::asm::jmp_remote(test_val, satp::XIP_VA + XIP_OFFSET);
        report_api(test_val as u32);
        expected += 1;
        assert!(expected == test_val);
    }
    report_api(0x61D0_600D);
}
