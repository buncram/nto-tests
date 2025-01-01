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

use core::convert::TryFrom;

use utra::mailbox;
use utralib::generated::*;

use crate::*;

const MBOX_TESTS: usize = 3;
crate::impl_test!(MboxTests, "MBOX", MBOX_TESTS);
impl TestRunner for MboxTests {
    fn run(&mut self) {
        for i in 0..2 {
            crate::println!("waiting {}\n", i);
        }

        // each of these tests auto-incs the passed field
        self.knock();
        self.abort();
        self.knock();
    }
}

impl MboxTests {
    pub fn knock(&mut self) {
        let mut mbox = Mbox::new();

        let test_data = [0xC0DE_0000u32, 0x0000_600Du32, 0, 0, 0, 0, 0, 0];
        let mut expected_result = 0;
        for &d in test_data.iter() {
            expected_result ^= d;
        }
        let test_pkt =
            MboxToCm7Pkt { version: MBOX_PROTOCOL_REV, opcode: ToCm7Op::Knock, len: 2, data: test_data };
        // crate::println!("sending knock...\n");
        match mbox.try_send(test_pkt) {
            Ok(_) => {
                // crate::println!("Packet send Ok\n");
                let mut timeout = 0;
                while mbox.poll_not_ready() {
                    timeout += 1;
                    if (timeout % 1_000) == 0 {
                        crate::println!("Waiting {}...", timeout);
                    }
                    if timeout >= 10_000 {
                        crate::println!("Mbox timed out");
                        return;
                    }
                }
                // now receive the packet
                // crate::println!("try_rx()...");
                match mbox.try_rx() {
                    Ok(rx_pkt) => {
                        crate::println!("Knock result: {:x}", rx_pkt.data[0]);
                        if rx_pkt.version != MBOX_PROTOCOL_REV {
                            crate::println!("Version mismatch {} != {}", rx_pkt.version, MBOX_PROTOCOL_REV);
                        }
                        if rx_pkt.opcode != ToRvOp::RetKnock {
                            crate::println!(
                                "Opcode mismatch {} != {}",
                                rx_pkt.opcode as u16,
                                ToRvOp::RetKnock as u16
                            );
                        }
                        if rx_pkt.len != 1 {
                            crate::println!("Expected length mismatch {} != {}", rx_pkt.len, 1);
                        } else {
                            if rx_pkt.data[0] != expected_result {
                                crate::println!(
                                    "Expected data mismatch {:x} != {:x}",
                                    rx_pkt.data[0],
                                    expected_result
                                );
                            } else {
                                crate::println!("Knock test PASS: {:x}", rx_pkt.data[0]);
                                self.passing_tests += 1;
                            }
                        }
                    }
                    Err(e) => {
                        crate::println!("Error while deserializing: {:?}\n", e);
                    }
                }
            }
            Err(e) => {
                crate::println!("Packet send error: {:?}\n", e);
            }
        };
    }

    pub fn abort(&mut self) {
        let mut mbox = Mbox::new();
        match mbox.abort() {
            Ok(_) => self.passing_tests += 1,
            Err(e) => {
                crate::println!("Abort test failed with {:?}", e);
            }
        }
    }
}
/// This constraint is limited by the size of the memory on the CM7 side
const MAX_PKT_LEN: usize = 128;
const MBOX_PROTOCOL_REV: u32 = 0;
const TX_FIFO_DEPTH: u32 = 128;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum MboxError {
    None,
    NotReady,
    TxOverflow,
    TxUnderflow,
    RxOverflow,
    RxUnderflow,
    InvalidOpcode,
    AbortFailed,
}

#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ToRvOp {
    Invalid = 0,

    RetKnock = 128,
    RetDct8x8 = 129,
    RetClifford = 130,
}
impl TryFrom<u16> for ToRvOp {
    type Error = MboxError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ToRvOp::Invalid),
            128 => Ok(ToRvOp::RetKnock),
            129 => Ok(ToRvOp::RetDct8x8),
            130 => Ok(ToRvOp::RetClifford),
            _ => Err(MboxError::InvalidOpcode),
        }
    }
}

#[repr(u16)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum ToCm7Op {
    Invalid = 0,

    Knock = 1,
    Dct8x8 = 2,
    Clifford = 3,
}

const STATIC_DATA_LEN: usize = 8;
pub struct MboxToCm7Pkt {
    version: u32,
    opcode: ToCm7Op,
    len: usize,
    data: [u32; STATIC_DATA_LEN],
}

pub struct MboxToRvPkt {
    version: u32,
    opcode: ToRvOp,
    len: usize,
    data: [u32; STATIC_DATA_LEN],
}

pub struct Mbox {
    csr: CSR<u32>,
}
impl Mbox {
    pub fn new() -> Mbox { Self { csr: CSR::new(mailbox::HW_MAILBOX_BASE as *mut u32) } }

    fn expect_tx(&mut self, val: u32) -> Result<(), MboxError> {
        if (TX_FIFO_DEPTH - self.csr.rf(mailbox::STATUS_TX_WORDS)) == 0 {
            return Err(MboxError::TxOverflow);
        } else {
            self.csr.wo(mailbox::WDATA, val);
            Ok(())
        }
    }

    pub fn try_send(&mut self, to_cm7: MboxToCm7Pkt) -> Result<(), MboxError> {
        if to_cm7.len > MAX_PKT_LEN {
            Err(MboxError::TxOverflow)
        } else {
            self.expect_tx(to_cm7.version)?;
            self.expect_tx(to_cm7.opcode as u32 | (to_cm7.len as u32) << 16)?;
            for &d in to_cm7.data[..to_cm7.len].iter() {
                self.expect_tx(d)?;
            }
            // trigger the send
            self.csr.wfo(mailbox::DONE_DONE, 1);
            Ok(())
        }
    }

    fn expect_rx(&mut self) -> Result<u32, MboxError> {
        if self.csr.rf(mailbox::STATUS_RX_WORDS) == 0 {
            Err(MboxError::RxUnderflow)
        } else {
            Ok(self.csr.r(mailbox::RDATA))
        }
    }

    pub fn try_rx(&mut self) -> Result<MboxToRvPkt, MboxError> {
        let version = self.expect_rx()?;
        let op_and_len = self.expect_rx()?;
        let opcode = ToRvOp::try_from((op_and_len & 0xFFFF) as u16)?;
        let len = (op_and_len >> 16) as usize;
        let mut data = [0u32; STATIC_DATA_LEN];
        for d in data[..len.min(STATIC_DATA_LEN)].iter_mut() {
            *d = self.expect_rx()?;
        }
        Ok(MboxToRvPkt { version, opcode, len, data })
    }

    pub fn poll_not_ready(&self) -> bool { self.csr.rf(mailbox::EV_PENDING_AVAILABLE) == 0 }

    pub fn abort(&mut self) -> Result<(), MboxError> {
        crate::println!("Initiating abort");
        self.csr.wfo(utra::mailbox::CONTROL_ABORT, 1);
        const TIMEOUT: usize = 1000;
        for _ in 0..TIMEOUT {
            if self.csr.rf(utra::mailbox::STATUS_ABORT_IN_PROGRESS) == 0 {
                return Ok(());
            }
        }
        return Err(MboxError::AbortFailed);
    }
}
