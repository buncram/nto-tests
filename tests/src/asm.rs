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

use core::arch::asm;

use utralib::generated::HW_SRAM_MEM as RAM_BASE;
use utralib::generated::HW_SRAM_MEM_LEN as RAM_SIZE;

// Assembly stubs for entering into the loader, and exiting it.

// Note: inline constants are not yet stable in Rust: https://github.com/rust-lang/rust/pull/104087
#[link_section = ".text.init"]
#[export_name = "_start"]
pub extern "C" fn _start() {
    unsafe {
        asm! (
            // cause default reset to fail; we can only boot if trimming_reset worked. Requires test bench
            // to set trimming_reset_ena to 1 and trimming_reset to 0x6000_0002
            // "j           _start",

            // decorate our stack area with a canary pattern
            "li          t1, 0xDEADC0DE",
            "mv          t0, {stack_limit}",
            "mv          t2, {ram_top}",
        // "100:", // fillstack
            "sw          t1, 0(t0)",
            // "lw          a0, 0(t0)",  // read back one byte
            "addi        t0, t0, 4",
            // "bltu        t0, t2, 100b",

            // Place the stack pointer at the end of RAM
            "mv          sp, {ram_top}",
            // subtract four from sp to make room for a DMA "gutter"
            "addi        sp, sp, -4",

            // Install a machine mode trap handler
            "la          t0, _start",
            "csrw        mtvec, t0",

            // Start Rust
            "j   rust_entry",

            ram_top = in(reg) (RAM_BASE + RAM_SIZE),
            // On Precursor - 0x40FFE01C: currently allowed stack extent - 8k - (7 words). 7 words are for kernel backup args - see bootloader in betrusted-soc
            stack_limit = in(reg) (RAM_BASE + RAM_SIZE - 8192 + 7 * core::mem::size_of::<usize>()),
            options(noreturn)
        );
    }
}

#[link_section = ".text.init"]
#[export_name = "abort"]
/// This is only used in debug mode
pub extern "C" fn abort() {
    unsafe {
        asm!(
            "300:", // abort
            "j 300b",
            options(noreturn)
        );
    }
}

#[inline(never)]
#[export_name = "jmp_remote"]
pub extern "C" fn jmp_remote(arg_buffer: usize, jmp_target: usize) -> usize {
    let ret: usize;
    unsafe {
        asm! (
            "move a0, {arg}",
            "jalr x0, {jmp_target}, 0",
            "move {ret}, a0",
            arg = in(reg) arg_buffer,
            jmp_target = in(reg) jmp_target,
            ret = out(reg) ret
        );
    }
    ret
}
