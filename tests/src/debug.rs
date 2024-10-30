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

use utralib::generated::*;
pub struct Uart {
    // pub base: *mut u32,
}

pub mod duart {
    pub const UART_DOUT: utralib::Register = utralib::Register::new(0, 0xff);
    pub const UART_DOUT_DOUT: utralib::Field = utralib::Field::new(8, 0, UART_DOUT);
    pub const UART_CTL: utralib::Register = utralib::Register::new(1, 1);
    pub const UART_CTL_EN: utralib::Field = utralib::Field::new(1, 0, UART_CTL);
    pub const UART_BUSY: utralib::Register = utralib::Register::new(2, 1);
    pub const UART_BUSY_BUSY: utralib::Field = utralib::Field::new(1, 0, UART_BUSY);

    pub const HW_DUART_BASE: usize = 0x4004_2000;
}

#[allow(dead_code)]
impl Uart {
    fn put_digit(&mut self, d: u8) {
        let nyb = d & 0xF;
        let c = if nyb < 10 { nyb + 0x30 } else { nyb + 0x61 - 10 };
        assert!(c >= 0x30, "conversion failed!");
        self.putc(c);
    }

    pub fn put_hex(&mut self, c: u8) {
        self.put_digit(c >> 4);
        self.put_digit(c & 0xF);
    }

    pub fn newline(&mut self) {
        self.putc(0xa);
        self.putc(0xd);
    }

    pub fn print_hex_word(&mut self, word: u32) {
        for &byte in word.to_be_bytes().iter() {
            self.put_hex(byte);
        }
    }

    pub fn putc(&self, c: u8) {
        let base = duart::HW_DUART_BASE as *mut u32;
        let mut uart = CSR::new(base);

        if uart.rf(duart::UART_CTL_EN) == 0 {
            uart.wfo(duart::UART_CTL_EN, 1);
        }
        while uart.rf(duart::UART_BUSY_BUSY) != 0 {
            // spin wait
        }
        uart.wfo(duart::UART_DOUT_DOUT, c as u32);
    }

    pub fn getc(&self) -> Option<u8> {
        unimplemented!();
    }

    pub fn tiny_write_str(&mut self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
        }
    }
}

use core::fmt::{Error, Write};
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

#[macro_use]
pub mod debug_print_hardware {
    #[macro_export]
    macro_rules! print
    {
        ($($args:tt)+) => ({
                use core::fmt::Write;
                let _ = write!(crate::debug::Uart {}, $($args)+);
        });
    }
}

#[macro_export]
macro_rules! println
{
    () => ({
        $crate::print!("\r")
    });
    ($fmt:expr) => ({
        $crate::print!(concat!($fmt, "\r"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::print!(concat!($fmt, "\r"), $($args)+)
    });
}
