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

#[cfg(feature = "coreuser-onehot")]
use utra::coreuser::*;
#[cfg(feature = "coreuser-lutop")]
use utra::coreuser::{
    CONTROL, CONTROL_ENABLE, CONTROL_MPP, CONTROL_PRIVILEGE, CONTROL_SHIFT, CONTROL_USE8BIT, STATUS_COREUSER,
    USERVALUE,
};
#[cfg(feature = "coreuser-compression")]
use utralib::generated::*;

use crate::utra::coreuser::{STATUS, STATUS_MM};
use crate::*;

const SATP_TESTS: usize = 1;
crate::impl_test!(SatpTests, "SATP", SATP_TESTS);
impl TestRunner for SatpTests {
    fn run(&mut self) {
        // This relies on both SATP and IRQs being setup
        self.passing_tests += satp_test();
    }
}
const SATP_SETUP: usize = 1;
crate::impl_test!(SatpSetup, "SATP Setup", SATP_SETUP);
impl TestRunner for SatpSetup {
    fn run(&mut self) {
        satp_setup();
        print!("vmem enabled\r");
        self.passing_tests += 1;
    }
}

pub const PAGE_SIZE: usize = 4096;
const WORD_SIZE: usize = core::mem::size_of::<u32>();

const FLG_VALID: usize = 0x1;
const FLG_X: usize = 0x8;
const FLG_W: usize = 0x4;
const FLG_R: usize = 0x2;
#[allow(dead_code)]
const FLG_U: usize = 0x10;
#[allow(dead_code)]
const FLG_A: usize = 0x40;
#[allow(dead_code)]
const FLG_D: usize = 0x80;

#[repr(C)]
pub struct PageTable {
    entries: [usize; PAGE_SIZE / WORD_SIZE],
}

// locate the page table entries
const ROOT_PT_PA: usize = 0x6100_0000; // 1st level at base of sram
// 2nd level PTs
const SRAM_PT_PA: usize = 0x6100_1000;
const CODE_PT_PA: usize = 0x6100_2000;
const CSR_PT_PA: usize = 0x6100_3000;
const PERI_PT_PA: usize = 0x6100_4000;
const PIO_PT_PA: usize = 0x6100_5000;
const XIP_PT_PA: usize = 0x6100_6000;
const RV_PT_PA: usize = 0x6100_7000;
// exception handler pages. Mapped 1:1 PA:VA, so no explicit remapping needed as RAM area is already mapped.
pub const SCRATCH_PAGE: usize = 0x6100_8000; // update this in irq.rs _start_trap() asm! as the scratch are
pub const _EXCEPTION_STACK_LIMIT: usize = 0x6100_9000; // update this in irq.rs as default stack pointer. The start of stack is this + 0x1000 & grows down
const BSS_PAGE: usize = 0x6100_A000; // this is manually read out of the link file. Equal to "base of RAM"
pub const PT_LIMIT: usize = 0x6100_B000; // this is carved out in link.x by setting RAM base at BSS_PAGE start

// VAs
const CODE_VA: usize = 0x6000_0000;
const CSR_VA: usize = 0x5800_0000;
const PERI_VA: usize = 0x4000_0000;
const SRAM_VA: usize = 0x6100_0000;
const PIO_VA: usize = 0x5000_0000;
pub const XIP_VA: usize = 0x7000_0000;
const RV_VA: usize = 0xE000_0000;

// PAs (when different from VAs)
const RERAM_PA: usize = 0x6000_0000;

fn set_l1_pte(from_va: usize, to_pa: usize, root_pt: &mut PageTable) {
    let index = from_va >> 22;
    root_pt.entries[index] = ((to_pa & 0xFFFF_FC00) >> 2) // top 2 bits of PA are not used, we don't do 34-bit PA featured by Sv32
        | FLG_VALID;
}

fn set_l2_pte(from_va: usize, to_pa: usize, l2_pt: &mut PageTable, flags: usize) {
    let index = (from_va >> 12) & 0x3_FF;
    l2_pt.entries[index] = ((to_pa & 0xFFFF_FC00) >> 2) // top 2 bits of PA are not used, we don't do 34-bit PA featured by Sv32
        | flags
        | FLG_VALID;
}

/// Very simple Sv32 setup that drops into supervisor (kernel) mode, with most
/// mappings being 1:1 between VA->PA, except for code which is remapped to address 0x0 in VA space.
#[inline(never)] // correct behavior depends on RA being set.
pub fn satp_setup() {
    unsafe {
        let pt_clr = ROOT_PT_PA as *mut u32;
        for i in 0..(PT_LIMIT - ROOT_PT_PA) / core::mem::size_of::<u32>() {
            pt_clr.add(i).write_volatile(0x0000_0000);
        }
    }
    // root page table is at p0x6100_0000 == v0x6100_0000
    let mut root_pt = unsafe { &mut *(ROOT_PT_PA as *mut PageTable) };
    let mut sram_pt = unsafe { &mut *(SRAM_PT_PA as *mut PageTable) };
    let mut code_pt = unsafe { &mut *(CODE_PT_PA as *mut PageTable) };
    let mut csr_pt = unsafe { &mut *(CSR_PT_PA as *mut PageTable) };
    let mut peri_pt = unsafe { &mut *(PERI_PT_PA as *mut PageTable) };
    let mut pio_pt = unsafe { &mut *(PIO_PT_PA as *mut PageTable) };
    let mut xip_pt = unsafe { &mut *(XIP_PT_PA as *mut PageTable) };
    let mut rv_pt = unsafe { &mut *(RV_PT_PA as *mut PageTable) };

    set_l1_pte(CODE_VA, CODE_PT_PA, &mut root_pt);
    set_l1_pte(CSR_VA, CSR_PT_PA, &mut root_pt);
    set_l1_pte(PERI_VA, PERI_PT_PA, &mut root_pt);
    set_l1_pte(SRAM_VA, SRAM_PT_PA, &mut root_pt); // L1 covers 16MiB, so SP_VA will cover all of SRAM
    set_l1_pte(PIO_VA, PIO_PT_PA, &mut root_pt);
    set_l1_pte(XIP_VA, XIP_PT_PA, &mut root_pt);
    set_l1_pte(RV_VA, RV_PT_PA, &mut root_pt);

    // map code space. This is the only one that has a difference on VA->PA
    const CODE_LEN: usize = 0x40_0000; // 4 megs, a whole superpage.
    for offset in (0..CODE_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(CODE_VA + offset, RERAM_PA + offset, &mut code_pt, FLG_X | FLG_R | FLG_U | FLG_W);
    }

    print!("mem pte\r");
    // map sram. Mapping is 1:1, so we use _VA and _PA targets for both args
    const SRAM_LEN: usize = 0x20_0000; // 2 megs
    // make the page tables not writeable
    for offset in (0..SCRATCH_PAGE - utralib::HW_SRAM_MEM).step_by(PAGE_SIZE) {
        set_l2_pte(SRAM_VA + offset, SRAM_VA + offset, &mut sram_pt, FLG_R | FLG_U);
    }
    // rest of RAM is r/w/x
    for offset in ((SCRATCH_PAGE - utralib::HW_SRAM_MEM)..SRAM_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(SRAM_VA + offset, SRAM_VA + offset, &mut sram_pt, FLG_W | FLG_R | FLG_U | FLG_X);
    }
    // map SoC-local peripherals (ticktimer, etc.)
    const CSR_LEN: usize = 0x2_0000;
    for offset in (0..CSR_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(CSR_VA + offset, CSR_VA + offset, &mut csr_pt, FLG_W | FLG_R | FLG_U);
    }
    // map APB peripherals (includes a window for the simulation bench)
    print!("peri pte\r");
    const PERI_LEN: usize = 0x10_0000; // 1M window - this will also map all the peripherals, including SCE, except PIO
    for offset in (0..PERI_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(PERI_VA + offset, PERI_VA + offset, &mut peri_pt, FLG_W | FLG_R | FLG_U);
    }
    // map the IF AMBA matrix (includes PIO + UDMA IFRAM)
    const PIO_OFFSET: usize = 0x00_0000;
    const PIO_LEN: usize = 0x21_0000; // this will map all the interfaces up to and including the UDC.
    for offset in (PIO_OFFSET..PIO_OFFSET + PIO_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(PIO_VA + offset, PIO_VA + offset, &mut pio_pt, FLG_W | FLG_R | FLG_U);
    }
    // map the RV peripherals
    const RV_LEN: usize = 0x2_0000; // 128k window
    print!("rv pte\r");
    for offset in (0..RV_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(RV_VA + offset, RV_VA + offset, &mut rv_pt, FLG_W | FLG_R | FLG_U);
    }
    // map the XIP memory, just for testing
    const XIP_LEN: usize = 0x1_0000; // just 64k of it for testing
    for offset in (0..XIP_LEN).step_by(PAGE_SIZE) {
        set_l2_pte(XIP_VA + offset, XIP_VA + offset, &mut xip_pt, FLG_X | FLG_W | FLG_R | FLG_U);
    }
    // clear BSS
    unsafe {
        let bss_region = core::slice::from_raw_parts_mut(BSS_PAGE as *mut u32, PAGE_SIZE / WORD_SIZE);
        for d in bss_region.iter_mut() {
            *d = 0;
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
    }

    let asid: u32 = 1;
    let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);

    print!("vmem pivot\r");
    unsafe {
        core::arch::asm!(
            // Delegate as much as we can supervisor mode
            "li          t0, 0xffffffff",
            "csrw        mideleg, t0",
            "csrw        medeleg, t0",

            // Return to Supervisor mode (1 << 11) when we call `reti`.
            // Disable interrupts (0 << 5), allow supervisor mode to run user mode code (1 << 18)
            "li		    t0, (1 << 11) | (0 << 5) | (1 << 18)",
            "csrw	    mstatus, t0",

            // Enable the MMU (once we issue `mret`) and flush the cache
            "csrw        satp, {satp_val}",
            "sfence.vma",
            ".word 0x500F",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            satp_val = in(reg) satp,
        );
        core::arch::asm!(
            // When loading with GDB we don't use a VM offset so GDB is less confused
            "li          t0, 0x0",
        );
        core::arch::asm!(
            "sub         a4, ra, t0",
            "csrw        mepc, a4",
            // sp "shouldn't move" because the mapping will take RAM mapping as 1:1 for VA:PA

            // Issue the return, which will jump to $mepc in Supervisor mode
            "mret",
        );
    }
}

#[inline(never)] // correct behavior depends on RA being set.
pub fn to_user_mode() {
    unsafe {
        core::arch::asm!("csrw   sepc, ra", "sret",);
    }
}

pub fn satp_test() -> usize {
    let mut passing = 1;
    report_api(0x5a1d_0000);
    #[cfg(feature = "coreuser-compression")]
    {
        let mut coreuser = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
        // first, clear the ASID table to 0
        for asid in 0..512 {
            coreuser.wo(
                utra::coreuser::SET_ASID,
                coreuser.ms(utra::coreuser::SET_ASID_ASID, asid)
                    | coreuser.ms(utra::coreuser::SET_ASID_TRUSTED, 0),
            );
        }

        // set some ASIDs to trusted. Values picked to somewhat challenge the decoding
        let trusted_asids = [1, 0x17, 0x18, 0x52, 0x57, 0x5A, 0x5F, 0x60, 0x61, 0x62, 0x116, 0x18F];
        for asid in trusted_asids {
            coreuser.wo(
                utra::coreuser::SET_ASID,
                coreuser.ms(utra::coreuser::SET_ASID_ASID, asid)
                    | coreuser.ms(utra::coreuser::SET_ASID_TRUSTED, 1),
            );
        }
        // readback of table
        /* // this is too slow with the Daric UART model, but uncomment it later when running on real hardware
        for asid in 0..512 {
            coreuser.wfo(utra::coreuser::GET_ASID_ADDR_ASID, asid);
            report_api(
                coreuser.rf(utra::coreuser::GET_ASID_VALUE_VALUE) << 16 | asid
            );
        } */

        // setup window on our root page. Narrowly define it to *just* one page.
        coreuser.wfo(utra::coreuser::WINDOW_AH_PPN, (ROOT_PT_PA >> 12) as u32);
        coreuser.wfo(utra::coreuser::WINDOW_AL_PPN, (ROOT_PT_PA >> 12) as u32);

        // turn on the coreuser computation
        coreuser.wo(
            utra::coreuser::CONTROL,
            coreuser.ms(utra::coreuser::CONTROL_ASID, 1)
                | coreuser.ms(utra::coreuser::CONTROL_ENABLE, 1)
                | coreuser.ms(utra::coreuser::CONTROL_PPN_A, 1),
        );

        // turn off updates
        coreuser.wo(utra::coreuser::PROTECT, 1);

        // tries to "turn off" protect, but it should do nothing
        coreuser.wo(utra::coreuser::PROTECT, 0);
        // tamper with asid & ppn values, should not change result
        // add `2` to the trusted list (should not work)
        coreuser.wo(
            utra::coreuser::SET_ASID,
            coreuser.ms(utra::coreuser::SET_ASID_ASID, 2) | coreuser.ms(utra::coreuser::SET_ASID_TRUSTED, 1),
        );
        coreuser.wfo(utra::coreuser::WINDOW_AH_PPN, 0xface as u32);
        coreuser.wfo(utra::coreuser::WINDOW_AL_PPN, 0xdead as u32);
        // partial readback of table; `2` should not be trusted
        for asid in 0..4 {
            coreuser.wfo(utra::coreuser::GET_ASID_ADDR_ASID, asid);
            report_api(coreuser.rf(utra::coreuser::GET_ASID_VALUE_VALUE) << 16 | asid);
        }
    }
    #[cfg(feature = "coreuser-onehot")]
    {
        crate::println!("coreuser one-hot option testing...");

        let mut coreuser = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);

        // check if coreuser has the expected value on reset
        let expected_default = 0b1_0001_1000; // this maps to the `boot0` user, in supervisor mode
        crate::println!("coreuser expected default test ({:x})", expected_default);
        if coreuser.r(STATUS) != expected_default {
            crate::println!("coreuser init fail: {:x} != {:x}", coreuser.r(STATUS), expected_default);
            passing = 0;
        }

        crate::println!("coreuser 2bit basic");
        let mut default = 0x3;
        // set to 0 so we can safely mask it later on
        coreuser.wo(USERVALUE, 0);
        coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
        let trusted_asids = [(1, 0), (0x17, 1), (0x18, 2), (0x52, 3), (0x57, 2), (1, 0), (1, 0), (0x60, 1)];
        let asid_fields = [
            (utra::coreuser::MAP_LO_LUT0, utra::coreuser::USERVALUE_USER0),
            (utra::coreuser::MAP_LO_LUT1, utra::coreuser::USERVALUE_USER1),
            (utra::coreuser::MAP_LO_LUT2, utra::coreuser::USERVALUE_USER2),
            (utra::coreuser::MAP_LO_LUT3, utra::coreuser::USERVALUE_USER3),
            (utra::coreuser::MAP_HI_LUT4, utra::coreuser::USERVALUE_USER4),
            (utra::coreuser::MAP_HI_LUT5, utra::coreuser::USERVALUE_USER5),
            (utra::coreuser::MAP_HI_LUT6, utra::coreuser::USERVALUE_USER6),
            (utra::coreuser::MAP_HI_LUT7, utra::coreuser::USERVALUE_USER7),
        ];
        for (&(asid, value), (map_field, uservalue_field)) in trusted_asids.iter().zip(asid_fields) {
            coreuser.rmwf(map_field, asid);
            coreuser.rmwf(uservalue_field, value);
        }

        coreuser.wo(CONTROL, 0);
        coreuser.rmwf(CONTROL_ENABLE, 1);

        // set 2-bit with no shift, manual entries
        if !check_2bit(&trusted_asids, default, true) {
            passing = 0;
        };

        // now check inversion bit - in machine mode
        crate::println!("coreuser MPP - machine mode");
        // mpp should be 11 in this test
        // coreuser.rmwf(CONTROL_INVERT_PRIV, 0); // should already be 0
        if coreuser.rf(STATUS_MM) != 1 {
            // machine mode + no invert -> assert status
            crate::println!("machine mode output mismatch (mm, !inv, != 1)");
            passing = 0;
        }
        coreuser.rmwf(CONTROL_INVERT_PRIV, 1);
        if coreuser.rf(STATUS_MM) != 0 {
            // machine mode + invert -> deassert status
            crate::println!("machine mode output mismatch (mm, inv, != 0)");
            passing = 0;
        }

        // can't change actual priv state in test so these don't work
        /*
        use riscv::register::mstatus;
        // set MPP to 0
        crate::println!("coreuser MPP - user mode");
        unsafe { mstatus::set_mpp(mstatus::MPP::User) };
        crate::println!("coreuser MPP user mode rbk {:?}", mstatus::read().mpp());
        coreuser.rmwf(CONTROL_INVERT_PRIV, 1);
        if coreuser.rf(STATUS_MM) != 1 {
            // user mode + invert -> assert status
            crate::println!("machine mode output mismatch (user, inv, != 1)");
            passing = 0;
        }
        coreuser.rmwf(CONTROL_INVERT_PRIV, 0);
        if coreuser.rf(STATUS_MM) != 0 {
            // user mode + no invert -> de-assert status
            crate::println!("machine mode output mismatch (user, !inv, != 0)");
            passing = 0;
        }

        // set MPP to 01
        crate::println!("coreuser MPP - supervisor mode");
        unsafe { mstatus::set_mpp(mstatus::MPP::Supervisor) };

        if coreuser.rf(STATUS_MM) != 1 {
            // supervisor mode + no invert -> assert status
            crate::println!("machine mode output mismatch (sup, !inv, != 1)");
            passing = 0;
        }
        coreuser.rmwf(CONTROL_INVERT_PRIV, 1);
        if coreuser.rf(STATUS_MM) != 0 {
            // supervisor mode + invert -> de-assert status
            crate::println!("machine mode output mismatch (sup, inv, !=0)");
            passing = 0;
        }

        crate::println!("coreuser MPP - return to machine mode");
        unsafe { mstatus::set_mpp(mstatus::MPP::Machine) };
        */

        // do some pseudorandom values into the table + shift values
        let mut random_asids = [(0u32, 0u32); 8];
        let mut state = 0xface_f001;
        for i in 0..7 {
            crate::println!("coreuser rand {}", i);
            // pick a random default
            state = lfsr_next_u32(state);
            default = state & 0x3;
            coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);

            for (asid, val) in random_asids.iter_mut() {
                state = lfsr_next_u32(state);
                *asid = state & 0xFF;
                state = lfsr_next_u32(state);
                *val = state & 0x3;
            }
            for (&(asid, value), (map_field, uservalue_field)) in random_asids.iter().zip(asid_fields) {
                coreuser.rmwf(map_field, asid);
                coreuser.rmwf(uservalue_field, value);
            }
            default = lfsr_next_u32(state) & 0x3;
            coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
            if !check_2bit(&random_asids, default, true) {
                passing = 0;
            };
        }

        // restore some sanity
        for (&(asid, value), (map_field, uservalue_field)) in trusted_asids.iter().zip(asid_fields) {
            coreuser.rmwf(map_field, asid);
            coreuser.rmwf(uservalue_field, value);
        }
        let fixed_default = 0;
        coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, fixed_default);
        coreuser.rmwf(CONTROL_INVERT_PRIV, 0);

        crate::println!("coreuser protect test");
        // turn off updates
        coreuser.wo(utra::coreuser::PROTECT, 1);

        // tries to "turn off" protect, but it should do nothing
        coreuser.wo(utra::coreuser::PROTECT, 0);

        // check that coreuser mapping is as expected
        if !check_2bit(&trusted_asids, fixed_default, true) {
            passing = 0;
        };

        // now try to update mapping
        state = lfsr_next_u32(state);

        for (asid, val) in random_asids.iter_mut() {
            state = lfsr_next_u32(state);
            *asid = state & 0xFF;
            state = lfsr_next_u32(state);
            *val = state & 0x3;
        }
        for (&(asid, value), (map_field, uservalue_field)) in random_asids.iter().zip(asid_fields) {
            coreuser.rmwf(map_field, asid);
            coreuser.rmwf(uservalue_field, value);
        }
        // pick a value we definitely know is different from `fixed_default` of 0
        default = 1;
        coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);

        // now check that the mapping was not updated
        crate::println!("+test");
        if !check_2bit(&trusted_asids, fixed_default, true) {
            // positive check that the mapping is there
            passing = 0;
        };
        crate::println!("-test");
        if check_2bit(&random_asids, default, false) {
            // negative check that the new mapping is not there
            crate::println!("mappings were updated when they shouldn't be!");
            passing = 0;
        };
    }
    #[cfg(feature = "coreuser-lutop")]
    {
        crate::println!("coruser LUT option testing...");
        let mut coreuser = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);

        // check if coreuser has the expected value on reset
        let expected_default = 0x0; // this maps to the `boot0` user
        crate::println!("coreuser expected default: {}", expected_default);
        if coreuser.rf(STATUS_COREUSER) != expected_default {
            crate::println!("coreuser init fail: {} != 1", coreuser.rf(STATUS_COREUSER));
            passing = 0;
        }

        // setup the 8-bit wide test
        coreuser.wo(CONTROL, 0);
        coreuser.rmwf(CONTROL_USE8BIT, 1);
        coreuser.rmwf(CONTROL_ENABLE, 1);

        for asid in 0..512 {
            let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
            unsafe {
                core::arch::asm!(
                    "csrw        satp, {satp_val}",
                    "sfence.vma",
                    satp_val = in(reg) satp,
                );
            }
            if asid < 256 {
                if coreuser.rf(utra::coreuser::STATUS_COREUSER) != asid {
                    crate::println!(
                        "coreuser 8-bit fail {} != {}",
                        asid,
                        coreuser.rf(utra::coreuser::STATUS_COREUSER)
                    );
                    passing = 0;
                }
            } else {
                if coreuser.rf(utra::coreuser::STATUS_COREUSER) != 0xFF {
                    crate::println!("coreuser 8-bit > 255 did not spoil");
                    passing = 0;
                }
            }
        }
        // check that MPP requirement clamps coreuser to 0x0
        coreuser.rmwf(CONTROL_MPP, 0);
        coreuser.rmwf(CONTROL_PRIVILEGE, 1);
        // shorter test because just checking a gating function
        for asid in 0..16 {
            let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
            unsafe {
                core::arch::asm!(
                    "csrw        satp, {satp_val}",
                    "sfence.vma",
                    satp_val = in(reg) satp,
                );
            }
            if coreuser.rf(utra::coreuser::STATUS_COREUSER) != 0x0 {
                crate::println!("coreuser 8-bit did not gate on MPP");
                passing = 0;
            }
        }

        crate::println!("coreuser 2bit basic");
        coreuser.wo(CONTROL, 0);
        coreuser.rmwf(CONTROL_USE8BIT, 0);
        coreuser.rmwf(CONTROL_ENABLE, 1);
        let mut default = 0x3;
        // set to 0 so we can safely mask it later on
        coreuser.wo(USERVALUE, 0);
        coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
        let trusted_asids = [(1, 0), (0x17, 1), (0x18, 2), (0x52, 3), (0x57, 2), (1, 0), (1, 0), (0x60, 1)];
        let asid_fields = [
            (utra::coreuser::MAP_LO_LUT0, utra::coreuser::USERVALUE_USER0),
            (utra::coreuser::MAP_LO_LUT1, utra::coreuser::USERVALUE_USER1),
            (utra::coreuser::MAP_LO_LUT2, utra::coreuser::USERVALUE_USER2),
            (utra::coreuser::MAP_LO_LUT3, utra::coreuser::USERVALUE_USER3),
            (utra::coreuser::MAP_HI_LUT4, utra::coreuser::USERVALUE_USER4),
            (utra::coreuser::MAP_HI_LUT5, utra::coreuser::USERVALUE_USER5),
            (utra::coreuser::MAP_HI_LUT6, utra::coreuser::USERVALUE_USER6),
            (utra::coreuser::MAP_HI_LUT7, utra::coreuser::USERVALUE_USER7),
        ];
        for (&(asid, value), (map_field, uservalue_field)) in trusted_asids.iter().zip(asid_fields) {
            coreuser.rmwf(map_field, asid);
            coreuser.rmwf(uservalue_field, value);
        }

        // set 2-bit with no shift, manual entries
        if !check_2bit(&trusted_asids, 0, default) {
            passing = 0;
        };

        // now check MPP bit
        crate::println!("coreuser MPP");
        coreuser.rmwf(CONTROL_SHIFT, 0);
        let asid = 1; // this is a matching ASID in the existing table, and should match to non-default (0)
        let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
        if coreuser.rf(STATUS_COREUSER) != 0 {
            crate::println!("MPP test did not setup coreuser as expected");
            passing = 0;
        }
        // turn on MPP
        coreuser.rmwf(CONTROL_MPP, 0);
        coreuser.rmwf(CONTROL_PRIVILEGE, 1);
        // we should have a match fail because we're not in user mode (MPP != 0)
        if coreuser.rf(STATUS_COREUSER) != default {
            crate::println!("MPP bit did not de-activate coreuser as expected");
            passing = 0;
        }
        coreuser.rmwf(CONTROL_MPP, 1);
        // check that machine mode did match
        if coreuser.rf(STATUS_COREUSER) != 0x0 {
            crate::println!("MPP bit did not match on machine mode as expected");
            passing = 0;
        }

        // do some pseudorandom values into the table + shift values
        let mut random_asids = [(0u32, 0u32); 8];
        let mut state = 0xface_f001;
        for shift in 0..7 {
            crate::println!("coreuser shift {}", shift);
            coreuser.rmwf(CONTROL_SHIFT, shift);
            // pick a random default
            state = lfsr_next_u32(state);
            default = state & 0x3;
            coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);

            for (asid, val) in random_asids.iter_mut() {
                state = lfsr_next_u32(state);
                *asid = state & 0xFF;
                state = lfsr_next_u32(state);
                *val = state & 0x3;
            }
            for (&(asid, value), (map_field, uservalue_field)) in random_asids.iter().zip(asid_fields) {
                coreuser.rmwf(map_field, asid);
                coreuser.rmwf(uservalue_field, value);
            }
            default = lfsr_next_u32(state) & 0x3;
            coreuser.rmwf(utra::coreuser::USERVALUE_DEFAULT, default);
            if !check_2bit(&random_asids, shift as usize, default) {
                passing = 0;
            };
        }

        // restore some sanity
        coreuser.rmwf(CONTROL_SHIFT, 0);
        for (&(asid, value), (map_field, uservalue_field)) in trusted_asids.iter().zip(asid_fields) {
            coreuser.rmwf(map_field, asid);
            coreuser.rmwf(uservalue_field, value);
        }
    }

    // now try changing the SATP around and see that the coreuser value updates
    // since we are in supervisor mode we can diddle with this at will, normally
    // user processes can't change this
    report_api(0x5a1d_0001);
    for asid in 0..512 {
        let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
    }
    // restore ASID to 1
    let satp: u32 = 0x8000_0000 | 1 << 22 | (ROOT_PT_PA as u32 >> 12);
    unsafe {
        core::arch::asm!(
            "csrw        satp, {satp_val}",
            "sfence.vma",
            satp_val = in(reg) satp,
        );
    }

    // control is locked, but confirm the mm state before switching priv levels
    let coreuser = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
    if coreuser.rf(STATUS_MM) != 1 {
        // machine mode + not invert -> assert status
        crate::println!("machine mode output mismatch (mm, !inv, != 1): {:x}", coreuser.r(STATUS));
        passing = 0;
    }

    // switch to user mode
    report_api(0x5a1d_0002);
    to_user_mode();

    // attempt to change ASID. This should be ignored or cause a trap, depending on the config of the device!
    // confirmed that without interrupts configured this has no effect; although it causes the following three
    // instructions to be ignored on the error.
    report_api(0x5a1d_0003);
    let satp: u32 = 0x8000_0000 | 4 << 22 | (ROOT_PT_PA as u32 >> 12);
    unsafe {
        core::arch::asm!(
            "csrw        satp, {satp_val}",
            "sfence.vma",
            // this is interesting. any less than 3 `nop`s below cause the 0x5a1d_0002 value to
            // not appear in the final register, to varying degrees. it seems that the pipeline gets a bit
            // imprecise after this sequence...
            "nop",
            "nop",
            "nop",
            satp_val = in(reg) satp,
        );
    }
    report_api(0x5a1d_0004);

    // confirm that the MM signal has changed polarity
    if coreuser.rf(STATUS_MM) != 0 {
        // machine mode + invert -> deassert status
        crate::println!("machine mode output mismatch (user, !inv, != 0): {:x}", coreuser.r(STATUS));
        passing = 0;
    }

    report_api(0x5a1d_600d);
    passing
}

/// returns `true` if pass
#[cfg(feature = "coreuser-lutop")]
fn check_2bit(lut: &[(u32, u32); 8], shift: usize, default: u32) -> bool {
    let verbose = false;
    let coreuser: CSR<u32> = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
    let mut passing = true;

    for asid in 0..512 {
        let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
        let rbk = coreuser.rf(STATUS_COREUSER);
        if lut.iter().any(|&(test_asid, _value)| test_asid == asid) {
            let mut found = false;
            for &(test_asid, value) in lut.iter() {
                if asid == test_asid {
                    found = true;
                    if rbk != (value << shift as u32) {
                        crate::println!("coreuser 2-bit failed to match on {}: {} got {}", asid, value, rbk);
                        passing = false;
                    } else {
                        if verbose {
                            crate::println!("coreuser 2-bit match success {} -> {}", asid, rbk);
                        }
                    }
                }
            }
            assert!(found, "ASID was not found in test set when it should have been!");
        } else {
            if rbk != (default << shift as u32) {
                crate::println!(
                    "coreuser 2-bit match miss failed to map on {} to default: {} got {}",
                    asid,
                    default,
                    rbk,
                );
                passing = false;
            }
        }
    }

    passing
}

/// returns `true` if pass
#[cfg(feature = "coreuser-onehot")]
fn check_2bit(lut: &[(u32, u32); 8], default: u32, print_fail: bool) -> bool {
    let verbose = false;
    let coreuser: CSR<u32> = CSR::new(utra::coreuser::HW_COREUSER_BASE as *mut u32);
    let mut passing = true;

    for asid in 0..512 {
        let satp: u32 = 0x8000_0000 | asid << 22 | (ROOT_PT_PA as u32 >> 12);
        unsafe {
            core::arch::asm!(
                "csrw        satp, {satp_val}",
                "sfence.vma",
                satp_val = in(reg) satp,
            );
        }
        let rbk = coreuser.rf(STATUS_COREUSER);
        if lut.iter().any(|&(test_asid, _value)| test_asid == asid) {
            let mut found = false;
            for &(test_asid, value) in lut.iter() {
                if asid == test_asid {
                    found = true;
                    let checkval = generate_coreuser(value as u32);
                    if rbk != checkval {
                        crate::println!(
                            "coreuser fail match@{}: {:x}, got {:x} ({:x})",
                            asid,
                            checkval,
                            rbk,
                            value
                        );
                        passing = false;
                    } else {
                        if verbose {
                            crate::println!("coreuser 2-bit match success {} -> {}", asid, rbk);
                        }
                    }
                }
            }
            assert!(found, "ASID was not found in test set when it should have been!");
        } else {
            let checkval = generate_coreuser(default);
            if rbk != checkval {
                if print_fail {
                    crate::println!(
                        "coreuser 2-bit match miss failed to map on {} to default: {:x} got {:x}",
                        asid,
                        checkval,
                        rbk,
                    );
                }
                passing = false;
            }
        }
    }

    passing
}

#[cfg(feature = "coreuser-onehot")]
fn generate_coreuser(value: u32) -> u32 {
    ((1 << value) << 4)
    // form LSB
    | ((((1 << value) & 0b1000) >> 3)
    | (((1 << value) & 0b0100) >> 1)
    | (((1 << value) & 0b0010) << 1)
    | (((1 << value) & 0b0001) << 3))
}
