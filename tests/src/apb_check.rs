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

// use crate::daric_generated::*;

pub fn singlecheck(title: &str, addr: *mut u32, data: u32) {
    let mut uart = crate::debug::Uart {};

    uart.tiny_write_str(title);
    uart.tiny_write_str("::  [");
    uart.print_hex_word(addr as u32);
    uart.tiny_write_str("] wr:");
    uart.print_hex_word(data as u32);
    uart.tiny_write_str(" | rd:");

    unsafe { addr.write_volatile(data) };
    let r = unsafe { addr.read_volatile() };

    uart.print_hex_word(r);
    uart.tiny_write_str(" ");
    if r != data {
        uart.tiny_write_str("----[x!]");
        if r == 0 {
            uart.tiny_write_str("[0!]");
        }
    }
    uart.tiny_write_str("\n")
}

pub fn singlecheckread(title: &str, addr: *const u32) {
    let mut uart = crate::debug::Uart {};
    uart.tiny_write_str(title);
    uart.tiny_write_str("::  [");
    uart.print_hex_word(addr as u32);
    uart.tiny_write_str("] wr:-------- | rd:");
    let r = unsafe { addr.read_volatile() };
    uart.print_hex_word(r);
    uart.tiny_write_str(" \n");
}

pub fn apb_test() {
    let mut uart = crate::debug::Uart {};
    crate::snap_ticks("scan bus:: ");

    crate::snap_ticks("gluechain:: ");
    singlecheck("GLUECHAIN_SFR_GCMASK", 0x40054000 as *mut u32, 0x924770d3);
    singlecheck("GLUECHAIN_SFR_GCSR  ", 0x40054004 as *mut u32, 0x6dcbac50);
    singlecheck("GLUECHAIN_SFR_GCRST ", 0x40054008 as *mut u32, 0x93fdcab8);
    singlecheck("GLUECHAIN_SFR_GCTEST", 0x4005400c as *mut u32, 0x34c2da80);
    crate::snap_ticks("mbox_apb:: ");
    singlecheck("MBOX_APB_SFR_WDATA ", 0x40013000 as *mut u32, 0xd035d259);
    singlecheck("MBOX_APB_SFR_RDATA ", 0x40013004 as *mut u32, 0xd2d6b877);
    singlecheck("MBOX_APB_SFR_STATUS", 0x40013008 as *mut u32, 0x2904cdef);
    singlecheck("MBOX_APB_SFR_ABORT ", 0x40013018 as *mut u32, 0x854a9657);
    singlecheck("MBOX_APB_SFR_DONE  ", 0x4001301c as *mut u32, 0x53e8eb43);
    crate::snap_ticks("qfc:: ");
    singlecheck("QFC_SFR_IO             ", 0x40010000 as *mut u32, 0xff1e5bef);
    singlecheck("QFC_SFR_AR             ", 0x40010004 as *mut u32, 0x0b680c1c);
    singlecheck("QFC_SFR_IODRV          ", 0x40010008 as *mut u32, 0xdc338383);
    singlecheck("QFC_CR_XIP_ADDRMODE    ", 0x40010010 as *mut u32, 0x9a6ab329);
    singlecheck("QFC_CR_XIP_OPCODE      ", 0x40010014 as *mut u32, 0x61b0ee09);
    singlecheck("QFC_CR_XIP_WIDTH       ", 0x40010018 as *mut u32, 0xacca7f0d);
    singlecheck("QFC_CR_XIP_SSEL        ", 0x4001001c as *mut u32, 0x74f2e2ed);
    singlecheck("QFC_CR_XIP_DUMCYC      ", 0x40010020 as *mut u32, 0xaf949e5e);
    singlecheck("QFC_CR_XIP_CFG         ", 0x40010024 as *mut u32, 0xa96ec2b3);
    singlecheck("QFC_CR_AESKEY_AESKEYIN0", 0x40010040 as *mut u32, 0x220adb0a);
    singlecheck("QFC_CR_AESKEY_AESKEYIN1", 0x40010044 as *mut u32, 0xfb7f6f5d);
    singlecheck("QFC_CR_AESKEY_AESKEYIN2", 0x40010048 as *mut u32, 0xf829d29f);
    singlecheck("QFC_CR_AESKEY_AESKEYIN3", 0x4001004c as *mut u32, 0x9d02fc90);
    singlecheck("QFC_CR_AESENA          ", 0x40010050 as *mut u32, 0x0109c207);
    crate::snap_ticks("mdma:: ");
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL0", 0x40012000 as *mut u32, 0x224c0601);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL1", 0x40012004 as *mut u32, 0xe5f0307e);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL2", 0x40012008 as *mut u32, 0x8c8a18b2);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL3", 0x4001200c as *mut u32, 0x6f9fb997);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL4", 0x40012010 as *mut u32, 0x95a4d257);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL5", 0x40012014 as *mut u32, 0x69b1dcbf);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL6", 0x40012018 as *mut u32, 0x0973e89c);
    singlecheck("MDMA_SFR_EVSEL_CR_EVSEL7", 0x4001201c as *mut u32, 0x7f216822);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ0 ", 0x40012020 as *mut u32, 0xa86b8a6e);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ1 ", 0x40012024 as *mut u32, 0xdae98554);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ2 ", 0x40012028 as *mut u32, 0x2651f637);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ3 ", 0x4001202c as *mut u32, 0x99ef1857);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ4 ", 0x40012030 as *mut u32, 0x18bb28e9);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ5 ", 0x40012034 as *mut u32, 0xceb506f6);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ6 ", 0x40012038 as *mut u32, 0xf29c11ad);
    singlecheck("MDMA_SFR_CR_CR_MDMAREQ7 ", 0x4001203c as *mut u32, 0x6a013380);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ0 ", 0x40012040 as *mut u32, 0x4652f62d);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ1 ", 0x40012044 as *mut u32, 0x1e9667c2);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ2 ", 0x40012048 as *mut u32, 0x74ab0a38);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ3 ", 0x4001204c as *mut u32, 0xd230b46c);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ4 ", 0x40012050 as *mut u32, 0xc70afc92);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ5 ", 0x40012054 as *mut u32, 0x58fa6e1c);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ6 ", 0x40012058 as *mut u32, 0x6f4930c8);
    singlecheck("MDMA_SFR_SR_SR_MDMAREQ7 ", 0x4001205c as *mut u32, 0xb66b3284);
    crate::snap_ticks("coresub_sramtrm:: ");
    singlecheck("CORESUB_SRAMTRM_SFR_CACHE  ", 0x40014000 as *mut u32, 0x51c5b8b2);
    singlecheck("CORESUB_SRAMTRM_SFR_ITCM   ", 0x40014004 as *mut u32, 0x1475f78d);
    singlecheck("CORESUB_SRAMTRM_SFR_DTCM   ", 0x40014008 as *mut u32, 0x72b3cb0b);
    singlecheck("CORESUB_SRAMTRM_SFR_SRAM0  ", 0x4001400c as *mut u32, 0x9535971c);
    singlecheck("CORESUB_SRAMTRM_SFR_SRAM1  ", 0x40014010 as *mut u32, 0xde9a896f);
    singlecheck("CORESUB_SRAMTRM_SFR_VEXRAM ", 0x40014014 as *mut u32, 0xe918be9f);
    singlecheck("CORESUB_SRAMTRM_SFR_SRAMERR", 0x40014020 as *mut u32, 0x8525e8a8);
    crate::snap_ticks("bio:: ");
    singlecheck("BIO_SFR_CTRL        ", 0x50124000 as *mut u32, 0x9a238b6a);
    singlecheck("BIO_SFR_CFGINFO     ", 0x50124004 as *mut u32, 0x1011eeb4);
    singlecheck("BIO_SFR_CONFIG      ", 0x50124008 as *mut u32, 0x66b072b9);
    singlecheck("BIO_SFR_FLEVEL      ", 0x5012400c as *mut u32, 0x95be4da0);
    singlecheck("BIO_SFR_TXF0        ", 0x50124010 as *mut u32, 0x89e6156b);
    singlecheck("BIO_SFR_TXF1        ", 0x50124014 as *mut u32, 0x70fc1afc);
    singlecheck("BIO_SFR_TXF2        ", 0x50124018 as *mut u32, 0x00b97ea6);
    singlecheck("BIO_SFR_TXF3        ", 0x5012401c as *mut u32, 0x89112f0a);
    singlecheck("BIO_SFR_RXF0        ", 0x50124020 as *mut u32, 0x0cf25923);
    singlecheck("BIO_SFR_RXF1        ", 0x50124024 as *mut u32, 0x17be082f);
    singlecheck("BIO_SFR_RXF2        ", 0x50124028 as *mut u32, 0x7c2c966d);
    singlecheck("BIO_SFR_RXF3        ", 0x5012402c as *mut u32, 0x72745307);
    singlecheck("BIO_SFR_ELEVEL      ", 0x50124030 as *mut u32, 0xb9e93d52);
    singlecheck("BIO_SFR_ETYPE       ", 0x50124034 as *mut u32, 0xb2a7a18a);
    singlecheck("BIO_SFR_EVENT_SET   ", 0x50124038 as *mut u32, 0x6c5e1578);
    singlecheck("BIO_SFR_EVENT_CLR   ", 0x5012403c as *mut u32, 0x3cc279b3);
    singlecheck("BIO_SFR_EVENT_STATUS", 0x50124040 as *mut u32, 0x9e7e1fc3);
    singlecheck("BIO_SFR_EXTCLOCK    ", 0x50124044 as *mut u32, 0xafe08a13);
    singlecheck("BIO_SFR_FIFO_CLR    ", 0x50124048 as *mut u32, 0x7a9c0163);
    singlecheck("BIO_SFR_QDIV0       ", 0x50124050 as *mut u32, 0xcebe24d9);
    singlecheck("BIO_SFR_QDIV1       ", 0x50124054 as *mut u32, 0xf65cf3f3);
    singlecheck("BIO_SFR_QDIV2       ", 0x50124058 as *mut u32, 0xdbdd4dd9);
    singlecheck("BIO_SFR_QDIV3       ", 0x5012405c as *mut u32, 0xb7debb9b);
    singlecheck("BIO_SFR_SYNC_BYPASS ", 0x50124060 as *mut u32, 0xe380a176);
    singlecheck("BIO_SFR_IO_OE_INV   ", 0x50124064 as *mut u32, 0xf65e7737);
    singlecheck("BIO_SFR_IO_O_INV    ", 0x50124068 as *mut u32, 0x491597ca);
    singlecheck("BIO_SFR_IO_I_INV    ", 0x5012406c as *mut u32, 0xab8534c1);
    singlecheck("BIO_SFR_IRQMASK_0   ", 0x50124070 as *mut u32, 0x242a809b);
    singlecheck("BIO_SFR_IRQMASK_1   ", 0x50124074 as *mut u32, 0x985a9ef0);
    singlecheck("BIO_SFR_IRQMASK_2   ", 0x50124078 as *mut u32, 0x5990fe96);
    singlecheck("BIO_SFR_IRQMASK_3   ", 0x5012407c as *mut u32, 0x31d4ff08);
    singlecheck("BIO_SFR_IRQ_EDGE    ", 0x50124080 as *mut u32, 0xffec35fe);
    singlecheck("BIO_SFR_DBG_PADOUT  ", 0x50124084 as *mut u32, 0x6c69a172);
    singlecheck("BIO_SFR_DBG_PADOE   ", 0x50124088 as *mut u32, 0x8e368ce0);
    crate::snap_ticks("rp_pio:: ");
    singlecheck("RP_PIO_SFR_CTRL         ", 0x50123000 as *mut u32, 0x5d447060);
    singlecheck("RP_PIO_SFR_FSTAT        ", 0x50123004 as *mut u32, 0xa08b84f3);
    singlecheck("RP_PIO_SFR_FDEBUG       ", 0x50123008 as *mut u32, 0x35a7efc0);
    singlecheck("RP_PIO_SFR_FLEVEL       ", 0x5012300c as *mut u32, 0x9c1a1528);
    singlecheck("RP_PIO_SFR_TXF0         ", 0x50123010 as *mut u32, 0xdffb54ce);
    singlecheck("RP_PIO_SFR_TXF1         ", 0x50123014 as *mut u32, 0xa6faba7b);
    singlecheck("RP_PIO_SFR_TXF2         ", 0x50123018 as *mut u32, 0x0e24d9cc);
    singlecheck("RP_PIO_SFR_TXF3         ", 0x5012301c as *mut u32, 0x4133e4d7);
    singlecheck("RP_PIO_SFR_RXF0         ", 0x50123020 as *mut u32, 0x74f5add5);
    singlecheck("RP_PIO_SFR_RXF1         ", 0x50123024 as *mut u32, 0xc2680192);
    singlecheck("RP_PIO_SFR_RXF2         ", 0x50123028 as *mut u32, 0x4850e927);
    singlecheck("RP_PIO_SFR_RXF3         ", 0x5012302c as *mut u32, 0xb1f00bca);
    singlecheck("RP_PIO_SFR_IRQ          ", 0x50123030 as *mut u32, 0x81aa70ac);
    singlecheck("RP_PIO_SFR_IRQ_FORCE    ", 0x50123034 as *mut u32, 0x437cbc41);
    singlecheck("RP_PIO_SFR_SYNC_BYPASS  ", 0x50123038 as *mut u32, 0x576e3d4f);
    singlecheck("RP_PIO_SFR_DBG_PADOUT   ", 0x5012303c as *mut u32, 0xb9fef1d6);
    singlecheck("RP_PIO_SFR_DBG_PADOE    ", 0x50123040 as *mut u32, 0x7adb78cc);
    singlecheck("RP_PIO_SFR_DBG_CFGINFO  ", 0x50123044 as *mut u32, 0xb847f03e);
    singlecheck("RP_PIO_SFR_INSTR_MEM0   ", 0x50123048 as *mut u32, 0xcb256db7);
    singlecheck("RP_PIO_SFR_INSTR_MEM1   ", 0x5012304c as *mut u32, 0xfe0a9c6c);
    singlecheck("RP_PIO_SFR_INSTR_MEM2   ", 0x50123050 as *mut u32, 0xd6220b4f);
    singlecheck("RP_PIO_SFR_INSTR_MEM3   ", 0x50123054 as *mut u32, 0x5274f6a0);
    singlecheck("RP_PIO_SFR_INSTR_MEM4   ", 0x50123058 as *mut u32, 0xbd8d9d60);
    singlecheck("RP_PIO_SFR_INSTR_MEM5   ", 0x5012305c as *mut u32, 0x9c3d087c);
    singlecheck("RP_PIO_SFR_INSTR_MEM6   ", 0x50123060 as *mut u32, 0x2fb5a758);
    singlecheck("RP_PIO_SFR_INSTR_MEM7   ", 0x50123064 as *mut u32, 0x2b24ced4);
    singlecheck("RP_PIO_SFR_INSTR_MEM8   ", 0x50123068 as *mut u32, 0xef8c60c0);
    singlecheck("RP_PIO_SFR_INSTR_MEM9   ", 0x5012306c as *mut u32, 0xbeb2993a);
    singlecheck("RP_PIO_SFR_INSTR_MEM10  ", 0x50123070 as *mut u32, 0xf37e27a3);
    singlecheck("RP_PIO_SFR_INSTR_MEM11  ", 0x50123074 as *mut u32, 0x041bbaab);
    singlecheck("RP_PIO_SFR_INSTR_MEM12  ", 0x50123078 as *mut u32, 0x338a2f39);
    singlecheck("RP_PIO_SFR_INSTR_MEM13  ", 0x5012307c as *mut u32, 0x93017690);
    singlecheck("RP_PIO_SFR_INSTR_MEM14  ", 0x50123080 as *mut u32, 0x0cd5d8e2);
    singlecheck("RP_PIO_SFR_INSTR_MEM15  ", 0x50123084 as *mut u32, 0x90af95fe);
    singlecheck("RP_PIO_SFR_INSTR_MEM16  ", 0x50123088 as *mut u32, 0x65a3a495);
    singlecheck("RP_PIO_SFR_INSTR_MEM17  ", 0x5012308c as *mut u32, 0x28615e14);
    singlecheck("RP_PIO_SFR_INSTR_MEM18  ", 0x50123090 as *mut u32, 0xdd4ec4d8);
    singlecheck("RP_PIO_SFR_INSTR_MEM19  ", 0x50123094 as *mut u32, 0xfff5618b);
    singlecheck("RP_PIO_SFR_INSTR_MEM20  ", 0x50123098 as *mut u32, 0x596f5f89);
    singlecheck("RP_PIO_SFR_INSTR_MEM21  ", 0x5012309c as *mut u32, 0x65bcf7b6);
    singlecheck("RP_PIO_SFR_INSTR_MEM22  ", 0x501230a0 as *mut u32, 0x5902a9db);
    singlecheck("RP_PIO_SFR_INSTR_MEM23  ", 0x501230a4 as *mut u32, 0xdd959036);
    singlecheck("RP_PIO_SFR_INSTR_MEM24  ", 0x501230a8 as *mut u32, 0xb4dc7dee);
    singlecheck("RP_PIO_SFR_INSTR_MEM25  ", 0x501230ac as *mut u32, 0x9c1f741f);
    singlecheck("RP_PIO_SFR_INSTR_MEM26  ", 0x501230b0 as *mut u32, 0xb8f9a0fe);
    singlecheck("RP_PIO_SFR_INSTR_MEM27  ", 0x501230b4 as *mut u32, 0x2cd2e360);
    singlecheck("RP_PIO_SFR_INSTR_MEM28  ", 0x501230b8 as *mut u32, 0xd2fcb96e);
    singlecheck("RP_PIO_SFR_INSTR_MEM29  ", 0x501230bc as *mut u32, 0x755961a9);
    singlecheck("RP_PIO_SFR_INSTR_MEM30  ", 0x501230c0 as *mut u32, 0x31c00570);
    singlecheck("RP_PIO_SFR_INSTR_MEM31  ", 0x501230c4 as *mut u32, 0x270e1922);
    singlecheck("RP_PIO_SFR_SM0_CLKDIV   ", 0x501230c8 as *mut u32, 0xa2460e15);
    singlecheck("RP_PIO_SFR_SM0_EXECCTRL ", 0x501230cc as *mut u32, 0x91ec56e8);
    singlecheck("RP_PIO_SFR_SM0_SHIFTCTRL", 0x501230d0 as *mut u32, 0x0d453cdf);
    singlecheck("RP_PIO_SFR_SM0_ADDR     ", 0x501230d4 as *mut u32, 0xf3281b8e);
    singlecheck("RP_PIO_SFR_SM0_INSTR    ", 0x501230d8 as *mut u32, 0x2d3f0b91);
    singlecheck("RP_PIO_SFR_SM0_PINCTRL  ", 0x501230dc as *mut u32, 0x2f81f9e8);
    singlecheck("RP_PIO_SFR_SM1_CLKDIV   ", 0x501230e0 as *mut u32, 0x0af2ee97);
    singlecheck("RP_PIO_SFR_SM1_EXECCTRL ", 0x501230e4 as *mut u32, 0x6dc3ccee);
    singlecheck("RP_PIO_SFR_SM1_SHIFTCTRL", 0x501230e8 as *mut u32, 0x31e6ff39);
    singlecheck("RP_PIO_SFR_SM1_ADDR     ", 0x501230ec as *mut u32, 0x29ade05f);
    singlecheck("RP_PIO_SFR_SM1_INSTR    ", 0x501230f0 as *mut u32, 0xc6fe30a5);
    singlecheck("RP_PIO_SFR_SM1_PINCTRL  ", 0x501230f4 as *mut u32, 0xe0f003e9);
    singlecheck("RP_PIO_SFR_SM2_CLKDIV   ", 0x501230f8 as *mut u32, 0xe130db7c);
    singlecheck("RP_PIO_SFR_SM2_EXECCTRL ", 0x501230fc as *mut u32, 0x6baeea37);
    singlecheck("RP_PIO_SFR_SM2_SHIFTCTRL", 0x50123100 as *mut u32, 0x18239760);
    singlecheck("RP_PIO_SFR_SM2_ADDR     ", 0x50123104 as *mut u32, 0x1e4e0fad);
    singlecheck("RP_PIO_SFR_SM2_INSTR    ", 0x50123108 as *mut u32, 0x1b5908f8);
    singlecheck("RP_PIO_SFR_SM2_PINCTRL  ", 0x5012310c as *mut u32, 0xe4367cca);
    singlecheck("RP_PIO_SFR_SM3_CLKDIV   ", 0x50123110 as *mut u32, 0x9ea76f08);
    singlecheck("RP_PIO_SFR_SM3_EXECCTRL ", 0x50123114 as *mut u32, 0x9b9aaad1);
    singlecheck("RP_PIO_SFR_SM3_SHIFTCTRL", 0x50123118 as *mut u32, 0x8853ae61);
    singlecheck("RP_PIO_SFR_SM3_ADDR     ", 0x5012311c as *mut u32, 0xad544735);
    singlecheck("RP_PIO_SFR_SM3_INSTR    ", 0x50123120 as *mut u32, 0xb1834066);
    singlecheck("RP_PIO_SFR_SM3_PINCTRL  ", 0x50123124 as *mut u32, 0x796c1d88);
    singlecheck("RP_PIO_SFR_INTR         ", 0x50123128 as *mut u32, 0x37c4c6b9);
    singlecheck("RP_PIO_SFR_IRQ0_INTE    ", 0x5012312c as *mut u32, 0x5affe1ce);
    singlecheck("RP_PIO_SFR_IRQ0_INTF    ", 0x50123130 as *mut u32, 0x87ffadf0);
    singlecheck("RP_PIO_SFR_IRQ0_INTS    ", 0x50123134 as *mut u32, 0x132a7176);
    singlecheck("RP_PIO_SFR_IRQ1_INTE    ", 0x50123138 as *mut u32, 0xcfac3216);
    singlecheck("RP_PIO_SFR_IRQ1_INTF    ", 0x5012313c as *mut u32, 0xd65358aa);
    singlecheck("RP_PIO_SFR_IRQ1_INTS    ", 0x50123140 as *mut u32, 0xeb2a9dff);
    singlecheck("RP_PIO_SFR_IO_OE_INV    ", 0x50123180 as *mut u32, 0xba1c4b6b);
    singlecheck("RP_PIO_SFR_IO_O_INV     ", 0x50123184 as *mut u32, 0x5bc1c366);
    singlecheck("RP_PIO_SFR_IO_I_INV     ", 0x50123188 as *mut u32, 0xa015b0f6);
    singlecheck("RP_PIO_SFR_FIFO_MARGIN  ", 0x5012318c as *mut u32, 0x2bccd6b6);
    singlecheck("RP_PIO_SFR_ZERO0        ", 0x50123190 as *mut u32, 0x34a02a65);
    singlecheck("RP_PIO_SFR_ZERO1        ", 0x50123194 as *mut u32, 0xd216eea8);
    singlecheck("RP_PIO_SFR_ZERO2        ", 0x50123198 as *mut u32, 0x3f13091d);
    singlecheck("RP_PIO_SFR_ZERO3        ", 0x5012319c as *mut u32, 0x07f5e51d);
    crate::snap_ticks("sddc:: ");
    singlecheck(
        "SDDC_SFR_IO                                                             ",
        0x50121000 as *mut u32,
        0x0165f0f0,
    );
    singlecheck(
        "SDDC_SFR_AR                                                             ",
        0x50121004 as *mut u32,
        0xfaa97965,
    );
    singlecheck(
        "SDDC_CR_OCR                                                             ",
        0x50121010 as *mut u32,
        0x62a12347,
    );
    singlecheck(
        "SDDC_CR_RDFFTHRES                                                       ",
        0x50121014 as *mut u32,
        0xa9632e3d,
    );
    singlecheck(
        "SDDC_CR_REV                                                             ",
        0x50121018 as *mut u32,
        0x9fc745d0,
    );
    singlecheck(
        "SDDC_CR_BACSA                                                           ",
        0x5012101c as *mut u32,
        0xe6cc5bda,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC0                                   ",
        0x50121020 as *mut u32,
        0x8d3ba755,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC1                                   ",
        0x50121024 as *mut u32,
        0x7587e119,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC2                                   ",
        0x50121028 as *mut u32,
        0x55f7f60a,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC3                                   ",
        0x5012102c as *mut u32,
        0x08d6920b,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC4                                   ",
        0x50121030 as *mut u32,
        0xa0a1c6ef,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC5                                   ",
        0x50121034 as *mut u32,
        0x120ff957,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC6                                   ",
        0x50121038 as *mut u32,
        0x43eb96f3,
    );
    singlecheck(
        "SDDC_CR_BAIOFN_CFG_BASE_ADDR_IO_FUNC7                                   ",
        0x5012103c as *mut u32,
        0x2d13320e,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR0                                  ",
        0x50121040 as *mut u32,
        0x75f32b50,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR1                                  ",
        0x50121044 as *mut u32,
        0x2752aa63,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR2                                  ",
        0x50121048 as *mut u32,
        0x865fc874,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR3                                  ",
        0x5012104c as *mut u32,
        0x1ed188dc,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR4                                  ",
        0x50121050 as *mut u32,
        0xce80f7b1,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR5                                  ",
        0x50121054 as *mut u32,
        0xcf4c7d08,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR6                                  ",
        0x50121058 as *mut u32,
        0x93672832,
    );
    singlecheck(
        "SDDC_CR_FNCISPTR_CFG_REG_FUNC_CIS_PTR7                                  ",
        0x5012105c as *mut u32,
        0x827f9414,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE0                         ",
        0x50121060 as *mut u32,
        0xb68a272a,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE1                         ",
        0x50121064 as *mut u32,
        0xa2465d29,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE2                         ",
        0x50121068 as *mut u32,
        0x9bdb8549,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE3                         ",
        0x5012106c as *mut u32,
        0xa5748bd1,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE4                         ",
        0x50121070 as *mut u32,
        0x891b8273,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE5                         ",
        0x50121074 as *mut u32,
        0x36c35dc5,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE6                         ",
        0x50121078 as *mut u32,
        0x6b3d9c59,
    );
    singlecheck(
        "SDDC_CR_FNEXTSTDCODE_CFG_REG_FUNC_EXT_STD_CODE7                         ",
        0x5012107c as *mut u32,
        0xdbc1f2b0,
    );
    singlecheck(
        "SDDC_CR_WRITE_PROTECT                                                   ",
        0x50121080 as *mut u32,
        0x1d95cdb0,
    );
    singlecheck(
        "SDDC_CR_REG_DSR                                                         ",
        0x50121084 as *mut u32,
        0x4cae8b6b,
    );
    singlecheck(
        "SDDC_CR_REG_CID_CFG_REG_CID0                                            ",
        0x50121088 as *mut u32,
        0xd9e0e7fe,
    );
    singlecheck(
        "SDDC_CR_REG_CID_CFG_REG_CID1                                            ",
        0x5012108c as *mut u32,
        0xa03bdb07,
    );
    singlecheck(
        "SDDC_CR_REG_CID_CFG_REG_CID2                                            ",
        0x50121090 as *mut u32,
        0x071c80a0,
    );
    singlecheck(
        "SDDC_CR_REG_CID_CFG_REG_CID3                                            ",
        0x50121094 as *mut u32,
        0x9bfa3292,
    );
    singlecheck(
        "SDDC_CR_REG_CSD_CFG_REG_CSD0                                            ",
        0x50121098 as *mut u32,
        0xc5f0a710,
    );
    singlecheck(
        "SDDC_CR_REG_CSD_CFG_REG_CSD1                                            ",
        0x5012109c as *mut u32,
        0xab4677e9,
    );
    singlecheck(
        "SDDC_CR_REG_CSD_CFG_REG_CSD2                                            ",
        0x501210a0 as *mut u32,
        0x279ee973,
    );
    singlecheck(
        "SDDC_CR_REG_CSD_CFG_REG_CSD3                                            ",
        0x501210a4 as *mut u32,
        0x63adcdbb,
    );
    singlecheck(
        "SDDC_CR_REG_SCR_CFG_REG_SCR0                                            ",
        0x501210a8 as *mut u32,
        0x36334245,
    );
    singlecheck(
        "SDDC_CR_REG_SCR_CFG_REG_SCR1                                            ",
        0x501210ac as *mut u32,
        0x8514f49c,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS0                                ",
        0x501210b0 as *mut u32,
        0x8e73758e,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS1                                ",
        0x501210b4 as *mut u32,
        0xf044106f,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS2                                ",
        0x501210b8 as *mut u32,
        0x21412442,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS3                                ",
        0x501210bc as *mut u32,
        0x7528ffc4,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS4                                ",
        0x501210c0 as *mut u32,
        0xac79fc27,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS5                                ",
        0x501210c4 as *mut u32,
        0x859b4c3f,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS6                                ",
        0x501210c8 as *mut u32,
        0xc9763989,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS7                                ",
        0x501210cc as *mut u32,
        0xb8b00cb6,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS8                                ",
        0x501210d0 as *mut u32,
        0x20aa8c99,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS9                                ",
        0x501210d4 as *mut u32,
        0xfdcf4d10,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS10                               ",
        0x501210d8 as *mut u32,
        0xee412da7,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS11                               ",
        0x501210dc as *mut u32,
        0x6f6da855,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS12                               ",
        0x501210e0 as *mut u32,
        0xa8c7b926,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS13                               ",
        0x501210e4 as *mut u32,
        0x1731222b,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS14                               ",
        0x501210e8 as *mut u32,
        0x74bc1570,
    );
    singlecheck(
        "SDDC_CR_REG_SD_STATUS_CFG_REG_SD_STATUS15                               ",
        0x501210ec as *mut u32,
        0x4fb358cf,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC0                      ",
        0x50121100 as *mut u32,
        0xbfde6f8e,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC1                      ",
        0x50121104 as *mut u32,
        0x61d4c262,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC2                      ",
        0x50121108 as *mut u32,
        0x9925fe10,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC3                      ",
        0x5012110c as *mut u32,
        0xf5a31c96,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC4                      ",
        0x50121110 as *mut u32,
        0xfcc1650f,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC5                      ",
        0x50121114 as *mut u32,
        0xbfd67c42,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC6                      ",
        0x50121118 as *mut u32,
        0x21546acf,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC7                      ",
        0x5012111c as *mut u32,
        0xa8245fe4,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC8                      ",
        0x50121120 as *mut u32,
        0x74a8eaec,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC9                      ",
        0x50121124 as *mut u32,
        0x0303cd03,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC10                     ",
        0x50121128 as *mut u32,
        0x2bb9daa6,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC11                     ",
        0x5012112c as *mut u32,
        0x35322c61,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC12                     ",
        0x50121130 as *mut u32,
        0x557e8b56,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC13                     ",
        0x50121134 as *mut u32,
        0x96cd38a3,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC14                     ",
        0x50121138 as *mut u32,
        0x27aacf24,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC15                     ",
        0x5012113c as *mut u32,
        0x7c0dc237,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC16                     ",
        0x50121140 as *mut u32,
        0x5b066f19,
    );
    singlecheck(
        "SDDC_CR_BASE_ADDR_MEM_FUNC_CFG_BASE_ADDR_MEM_FUNC17                     ",
        0x50121144 as *mut u32,
        0x0141c505,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE0",
        0x50121148 as *mut u32,
        0xac981167,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE1",
        0x5012114c as *mut u32,
        0x0c4c3bc9,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE2",
        0x50121150 as *mut u32,
        0xb6cfad9e,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE3",
        0x50121154 as *mut u32,
        0x7593f011,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE4",
        0x50121158 as *mut u32,
        0xd6ba26ea,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE5",
        0x5012115c as *mut u32,
        0x17bb0d12,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_INTERFACE_CODE_CFG_REG_FUNC_ISDIO_INTERFACE_CODE6",
        0x50121160 as *mut u32,
        0x764f924c,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE0              ",
        0x50121168 as *mut u32,
        0x8cfd7d07,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE1              ",
        0x5012116c as *mut u32,
        0x9218bb1c,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE2              ",
        0x50121170 as *mut u32,
        0x15c19a7a,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE3              ",
        0x50121174 as *mut u32,
        0xf1c04236,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE4              ",
        0x50121178 as *mut u32,
        0xc0ca8d26,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE5              ",
        0x5012117c as *mut u32,
        0xd9d42713,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_CODE_CFG_REG_FUNC_MANUFACT_CODE6              ",
        0x50121180 as *mut u32,
        0xd0b1d953,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO0              ",
        0x50121188 as *mut u32,
        0x05efdd1d,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO1              ",
        0x5012118c as *mut u32,
        0xff5e6c3b,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO2              ",
        0x50121190 as *mut u32,
        0xb6307ed0,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO3              ",
        0x50121194 as *mut u32,
        0xb21564b9,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO4              ",
        0x50121198 as *mut u32,
        0xa82aed54,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO5              ",
        0x5012119c as *mut u32,
        0x73650b75,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_MANUFACT_INFO_CFG_REG_FUNC_MANUFACT_INFO6              ",
        0x501211a0 as *mut u32,
        0x3a0e8f7d,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE0  ",
        0x501211a8 as *mut u32,
        0x77bea723,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE1  ",
        0x501211ac as *mut u32,
        0xe8ad5157,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE2  ",
        0x501211b0 as *mut u32,
        0x8b60b627,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE3  ",
        0x501211b4 as *mut u32,
        0xd0c42bcf,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE4  ",
        0x501211b8 as *mut u32,
        0x4dad91a9,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE5  ",
        0x501211bc as *mut u32,
        0x3c01a871,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_ISDIO_TYPE_SUP_CODE_CFG_REG_FUNC_ISDIO_TYPE_SUP_CODE6  ",
        0x501211c0 as *mut u32,
        0x084b4fbc,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO0                                ",
        0x501211c8 as *mut u32,
        0x2a1a82c6,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO1                                ",
        0x501211cc as *mut u32,
        0x4ebba1e3,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO2                                ",
        0x501211d0 as *mut u32,
        0xc493daeb,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO3                                ",
        0x501211d4 as *mut u32,
        0x07462357,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO4                                ",
        0x501211d8 as *mut u32,
        0x778abe27,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO5                                ",
        0x501211dc as *mut u32,
        0x3ed664db,
    );
    singlecheck(
        "SDDC_CR_REG_FUNC_INFO_CFG_REG_FUNC_INFO6                                ",
        0x501211e0 as *mut u32,
        0x046cbf22,
    );
    singlecheck(
        "SDDC_CR_REG_UHS_1_SUPPORT                                               ",
        0x501211f0 as *mut u32,
        0xadeb2fa9,
    );
    crate::snap_ticks("pwm:: ");
    crate::snap_ticks("iox:: ");
    singlecheck("IOX_SFR_AFSEL_CRAFSEL0           ", 0x5012f000 as *mut u32, 0x5a1cdbaa);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL1           ", 0x5012f004 as *mut u32, 0xae22e854);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL2           ", 0x5012f008 as *mut u32, 0x4245e01a);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL3           ", 0x5012f00c as *mut u32, 0x279388b0);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL4           ", 0x5012f010 as *mut u32, 0x0ab84c77);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL5           ", 0x5012f014 as *mut u32, 0x3d3ed919);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL6           ", 0x5012f018 as *mut u32, 0xc4210beb);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL7           ", 0x5012f01c as *mut u32, 0x59bc0dd6);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL8           ", 0x5012f020 as *mut u32, 0x8d54bf68);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL9           ", 0x5012f024 as *mut u32, 0xb2ab5d29);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL10          ", 0x5012f028 as *mut u32, 0x4b6f4f2d);
    singlecheck("IOX_SFR_AFSEL_CRAFSEL11          ", 0x5012f02c as *mut u32, 0x0e73cca3);
    singlecheck("IOX_SFR_INTCR_CRINT0             ", 0x5012f100 as *mut u32, 0x5c15ba7e);
    singlecheck("IOX_SFR_INTCR_CRINT1             ", 0x5012f104 as *mut u32, 0xb9fd8716);
    singlecheck("IOX_SFR_INTCR_CRINT2             ", 0x5012f108 as *mut u32, 0x78916c18);
    singlecheck("IOX_SFR_INTCR_CRINT3             ", 0x5012f10c as *mut u32, 0x11e22e00);
    singlecheck("IOX_SFR_INTCR_CRINT4             ", 0x5012f110 as *mut u32, 0x94b7c27d);
    singlecheck("IOX_SFR_INTCR_CRINT5             ", 0x5012f114 as *mut u32, 0x525ce443);
    singlecheck("IOX_SFR_INTCR_CRINT6             ", 0x5012f118 as *mut u32, 0x32f86bf3);
    singlecheck("IOX_SFR_INTCR_CRINT7             ", 0x5012f11c as *mut u32, 0x95ec24b6);
    singlecheck("IOX_SFR_INTFR                    ", 0x5012f120 as *mut u32, 0xbbb989d4);
    singlecheck("IOX_SFR_GPIOOUT_CRGO0            ", 0x5012f130 as *mut u32, 0x7bad0e15);
    singlecheck("IOX_SFR_GPIOOUT_CRGO1            ", 0x5012f134 as *mut u32, 0x64ca7e0a);
    singlecheck("IOX_SFR_GPIOOUT_CRGO2            ", 0x5012f138 as *mut u32, 0x638fe1f6);
    singlecheck("IOX_SFR_GPIOOUT_CRGO3            ", 0x5012f13c as *mut u32, 0xd996625d);
    singlecheck("IOX_SFR_GPIOOUT_CRGO4            ", 0x5012f140 as *mut u32, 0xe0517bdb);
    singlecheck("IOX_SFR_GPIOOUT_CRGO5            ", 0x5012f144 as *mut u32, 0xeb17c9f7);
    singlecheck("IOX_SFR_GPIOOE_CRGOE0            ", 0x5012f148 as *mut u32, 0x7d0e94f9);
    singlecheck("IOX_SFR_GPIOOE_CRGOE1            ", 0x5012f14c as *mut u32, 0xb20c167a);
    singlecheck("IOX_SFR_GPIOOE_CRGOE2            ", 0x5012f150 as *mut u32, 0xa5b59758);
    singlecheck("IOX_SFR_GPIOOE_CRGOE3            ", 0x5012f154 as *mut u32, 0x2b72aee0);
    singlecheck("IOX_SFR_GPIOOE_CRGOE4            ", 0x5012f158 as *mut u32, 0x3ab7ff43);
    singlecheck("IOX_SFR_GPIOOE_CRGOE5            ", 0x5012f15c as *mut u32, 0x69ef80fb);
    singlecheck("IOX_SFR_GPIOPU_CRGPU0            ", 0x5012f160 as *mut u32, 0xd5aaec80);
    singlecheck("IOX_SFR_GPIOPU_CRGPU1            ", 0x5012f164 as *mut u32, 0xb6592d2f);
    singlecheck("IOX_SFR_GPIOPU_CRGPU2            ", 0x5012f168 as *mut u32, 0xcec638c1);
    singlecheck("IOX_SFR_GPIOPU_CRGPU3            ", 0x5012f16c as *mut u32, 0xbd1b7b4d);
    singlecheck("IOX_SFR_GPIOPU_CRGPU4            ", 0x5012f170 as *mut u32, 0xcbc95127);
    singlecheck("IOX_SFR_GPIOPU_CRGPU5            ", 0x5012f174 as *mut u32, 0x30703107);
    singlecheck("IOX_SFR_GPIOIN_SRGI0             ", 0x5012f178 as *mut u32, 0xb84af08d);
    singlecheck("IOX_SFR_GPIOIN_SRGI1             ", 0x5012f17c as *mut u32, 0x8a6c8e1d);
    singlecheck("IOX_SFR_GPIOIN_SRGI2             ", 0x5012f180 as *mut u32, 0xeb15a15c);
    singlecheck("IOX_SFR_GPIOIN_SRGI3             ", 0x5012f184 as *mut u32, 0x53dea12e);
    singlecheck("IOX_SFR_GPIOIN_SRGI4             ", 0x5012f188 as *mut u32, 0xdc59f0df);
    singlecheck("IOX_SFR_GPIOIN_SRGI5             ", 0x5012f18c as *mut u32, 0xcd515d43);
    singlecheck("IOX_SFR_PIOSEL                   ", 0x5012f200 as *mut u32, 0x3e1d6011);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL0 ", 0x5012f230 as *mut u32, 0x66515cf5);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL1 ", 0x5012f234 as *mut u32, 0x820b8d93);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL2 ", 0x5012f238 as *mut u32, 0x17b9f3ce);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL3 ", 0x5012f23c as *mut u32, 0x0f1a7156);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL4 ", 0x5012f240 as *mut u32, 0xb96ccff1);
    singlecheck("IOX_SFR_CFG_SCHM_CR_CFG_SCHMSEL5 ", 0x5012f244 as *mut u32, 0x4536886d);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW0", 0x5012f248 as *mut u32, 0xb34469da);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW1", 0x5012f24c as *mut u32, 0xc1b3305c);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW2", 0x5012f250 as *mut u32, 0x88f41a0b);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW3", 0x5012f254 as *mut u32, 0xd15740a1);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW4", 0x5012f258 as *mut u32, 0xd05c1cff);
    singlecheck("IOX_SFR_CFG_SLEW_CR_CFG_SLEWSLOW5", 0x5012f25c as *mut u32, 0xb3719b13);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL0", 0x5012f260 as *mut u32, 0xec957ad2);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL1", 0x5012f264 as *mut u32, 0x05e0a076);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL2", 0x5012f268 as *mut u32, 0x98b43e02);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL3", 0x5012f26c as *mut u32, 0x686f3b6f);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL4", 0x5012f270 as *mut u32, 0x79c0f09b);
    singlecheck("IOX_SFR_CFG_DRVSEL_CR_CFG_DRVSEL5", 0x5012f274 as *mut u32, 0x10f497a0);
    crate::snap_ticks("apb_thru:: ");
    crate::snap_ticks("evc:: ");
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL0", 0x40044000 as *mut u32, 0x14074c28);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL1", 0x40044004 as *mut u32, 0x0ea10477);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL2", 0x40044008 as *mut u32, 0xe5837aec);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL3", 0x4004400c as *mut u32, 0x0dbaff4f);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL4", 0x40044010 as *mut u32, 0x890cf72d);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL5", 0x40044014 as *mut u32, 0x5c4f178f);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL6", 0x40044018 as *mut u32, 0x979a65d0);
    singlecheck("EVC_SFR_CM7EVSEL_CM7EVSEL7", 0x4004401c as *mut u32, 0x552d8ad9);
    singlecheck("EVC_SFR_CM7EVEN           ", 0x40044020 as *mut u32, 0x606ab72e);
    singlecheck("EVC_SFR_CM7EVFR           ", 0x40044024 as *mut u32, 0x76ce8f4d);
    singlecheck("EVC_SFR_TMREVSEL          ", 0x40044030 as *mut u32, 0x67aabc95);
    singlecheck("EVC_SFR_PWMEVSEL          ", 0x40044034 as *mut u32, 0x4646f736);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN0    ", 0x40044040 as *mut u32, 0x00ef288b);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN1    ", 0x40044044 as *mut u32, 0x5c75d8c9);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN2    ", 0x40044048 as *mut u32, 0xe1c4524a);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN3    ", 0x4004404c as *mut u32, 0x28c72655);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN4    ", 0x40044050 as *mut u32, 0xe6a639ed);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN5    ", 0x40044054 as *mut u32, 0xab94b274);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN6    ", 0x40044058 as *mut u32, 0x0ccee22b);
    singlecheck("EVC_SFR_IFEVEN_IFEVEN7    ", 0x4004405c as *mut u32, 0x8cdbb24b);
    singlecheck("EVC_SFR_IFEVERRFR         ", 0x40044060 as *mut u32, 0x4b973246);
    singlecheck("EVC_SFR_CM7ERRFR          ", 0x40044080 as *mut u32, 0xeb62ca60);
    crate::snap_ticks("timer_intf:: ");
    crate::snap_ticks("wdg_intf:: ");
    crate::snap_ticks("duart:: ");
    singlecheck("DUART_SFR_TXD ", 0x40042000 as *mut u32, 0xc2465d93);
    singlecheck("DUART_SFR_CR  ", 0x40042004 as *mut u32, 0xfa269d59);
    singlecheck("DUART_SFR_SR  ", 0x40042008 as *mut u32, 0x12722219);
    singlecheck("DUART_SFR_ETUC", 0x4004200c as *mut u32, 0xcdb866c3);
    crate::snap_ticks("sce_glbsfr:: ");
    singlecheck("SCE_GLBSFR_SFR_SCEMODE     ", 0x40028000 as *mut u32, 0xd8fddbc1);
    singlecheck("SCE_GLBSFR_SFR_SUBEN       ", 0x40028004 as *mut u32, 0xa5974868);
    singlecheck("SCE_GLBSFR_SFR_AHBS        ", 0x40028008 as *mut u32, 0x3f0e4c1a);
    singlecheck("SCE_GLBSFR_SFR_SRBUSY      ", 0x40028010 as *mut u32, 0xae3d4aec);
    singlecheck("SCE_GLBSFR_SFR_FRDONE      ", 0x40028014 as *mut u32, 0xac0e9f1c);
    singlecheck("SCE_GLBSFR_SFR_FRERR       ", 0x40028018 as *mut u32, 0x8091f046);
    singlecheck("SCE_GLBSFR_SFR_ARCLR       ", 0x4002801c as *mut u32, 0x56a84a9a);
    singlecheck("SCE_GLBSFR_SFR_FRACERR     ", 0x40028020 as *mut u32, 0x0e4fdc43);
    singlecheck("SCE_GLBSFR_SFR_TICKCNT     ", 0x40028024 as *mut u32, 0x55a2ffa2);
    singlecheck("SCE_GLBSFR_SFR_FFEN        ", 0x40028030 as *mut u32, 0x7dac7733);
    singlecheck("SCE_GLBSFR_SFR_FFCLR       ", 0x40028034 as *mut u32, 0x896147e5);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF0", 0x40028040 as *mut u32, 0xee095a7c);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF1", 0x40028044 as *mut u32, 0x5f8c6e0e);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF2", 0x40028048 as *mut u32, 0xcb94fa18);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF3", 0x4002804c as *mut u32, 0x63317e84);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF4", 0x40028050 as *mut u32, 0x461d66f6);
    singlecheck("SCE_GLBSFR_SFR_FFCNT_SR_FF5", 0x40028054 as *mut u32, 0x57149339);
    singlecheck("SCE_GLBSFR_SFR_TS          ", 0x400280fc as *mut u32, 0x174b0a3b);
    crate::snap_ticks("scedma:: ");
    singlecheck("SCEDMA_SFR_SCHSTART_AR ", 0x40029000 as *mut u32, 0xdac5e35c);
    singlecheck("SCEDMA_SFR_XCH_FUNC    ", 0x40029010 as *mut u32, 0x6fadbb30);
    singlecheck("SCEDMA_SFR_XCH_OPT     ", 0x40029014 as *mut u32, 0x8da014ae);
    singlecheck("SCEDMA_SFR_XCH_AXSTART ", 0x40029018 as *mut u32, 0xa3f961e5);
    singlecheck("SCEDMA_SFR_XCH_SEGID   ", 0x4002901c as *mut u32, 0xb1fd44b7);
    singlecheck("SCEDMA_SFR_XCH_SEGSTART", 0x40029020 as *mut u32, 0x2a83265f);
    singlecheck("SCEDMA_SFR_XCH_TRANSIZE", 0x40029024 as *mut u32, 0x02739e6d);
    singlecheck("SCEDMA_SFR_SCH_FUNC    ", 0x40029030 as *mut u32, 0xe834916a);
    singlecheck("SCEDMA_SFR_SCH_OPT     ", 0x40029034 as *mut u32, 0x3025fec8);
    singlecheck("SCEDMA_SFR_SCH_AXSTART ", 0x40029038 as *mut u32, 0xc6fec3b8);
    singlecheck("SCEDMA_SFR_SCH_SEGID   ", 0x4002903c as *mut u32, 0x9104f4df);
    singlecheck("SCEDMA_SFR_SCH_SEGSTART", 0x40029040 as *mut u32, 0x8ebb66b2);
    singlecheck("SCEDMA_SFR_SCH_TRANSIZE", 0x40029044 as *mut u32, 0x82428606);
    singlecheck("SCEDMA_SFR_ICH_OPT     ", 0x40029050 as *mut u32, 0x97176b9c);
    singlecheck("SCEDMA_SFR_ICH_SEGID   ", 0x40029054 as *mut u32, 0x268e9559);
    singlecheck("SCEDMA_SFR_ICH_RPSTART ", 0x40029058 as *mut u32, 0x93c18f15);
    singlecheck("SCEDMA_SFR_ICH_WPSTART ", 0x4002905c as *mut u32, 0x0c34ff73);
    singlecheck("SCEDMA_SFR_ICH_TRANSIZE", 0x40029060 as *mut u32, 0xf4776fd1);
    crate::snap_ticks("pke:: ");
    singlecheck("PKE_SFR_CRFUNC           ", 0x4002c000 as *mut u32, 0x249256c2);
    singlecheck("PKE_SFR_AR               ", 0x4002c004 as *mut u32, 0xdb1fed6d);
    singlecheck("PKE_SFR_SRMFSM           ", 0x4002c008 as *mut u32, 0x7acd93b5);
    singlecheck("PKE_SFR_FR               ", 0x4002c00c as *mut u32, 0x042c397c);
    singlecheck("PKE_SFR_OPTNW            ", 0x4002c010 as *mut u32, 0x469fc6c7);
    singlecheck("PKE_SFR_OPTEW            ", 0x4002c014 as *mut u32, 0xb618f9a1);
    singlecheck("PKE_SFR_OPTRW            ", 0x4002c018 as *mut u32, 0x61709eca);
    singlecheck("PKE_SFR_OPTLTX           ", 0x4002c01c as *mut u32, 0x396743d0);
    singlecheck("PKE_SFR_OPTMASK          ", 0x4002c020 as *mut u32, 0x954af22e);
    singlecheck("PKE_SFR_SEGPTR_PTRID_PCON", 0x4002c030 as *mut u32, 0x258e26e4);
    singlecheck("PKE_SFR_SEGPTR_PTRID_PIB0", 0x4002c034 as *mut u32, 0x21bc4265);
    singlecheck("PKE_SFR_SEGPTR_PTRID_PIB1", 0x4002c038 as *mut u32, 0x45a019d5);
    singlecheck("PKE_SFR_SEGPTR_PTRID_PKB ", 0x4002c03c as *mut u32, 0xbe036699);
    singlecheck("PKE_SFR_SEGPTR_PTRID_POB ", 0x4002c040 as *mut u32, 0x7d419f3a);
    crate::snap_ticks("combohash:: ");
    singlecheck("COMBOHASH_SFR_CRFUNC            ", 0x4002b000 as *mut u32, 0xbaf44fb7);
    singlecheck("COMBOHASH_SFR_AR                ", 0x4002b004 as *mut u32, 0xd75d8c8f);
    singlecheck("COMBOHASH_SFR_SRMFSM            ", 0x4002b008 as *mut u32, 0x3694790f);
    singlecheck("COMBOHASH_SFR_FR                ", 0x4002b00c as *mut u32, 0xc11e7995);
    singlecheck("COMBOHASH_SFR_OPT1              ", 0x4002b010 as *mut u32, 0x076a592d);
    singlecheck("COMBOHASH_SFR_OPT2              ", 0x4002b014 as *mut u32, 0xead52082);
    singlecheck("COMBOHASH_SFR_OPT3              ", 0x4002b018 as *mut u32, 0x624f36f7);
    singlecheck("COMBOHASH_SFR_BLKT0             ", 0x4002b01c as *mut u32, 0xd0111ed9);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_LKEY ", 0x4002b020 as *mut u32, 0x78e76e99);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_KEY  ", 0x4002b024 as *mut u32, 0xf3e22009);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_SCRT ", 0x4002b02c as *mut u32, 0xb5143a64);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_MSG  ", 0x4002b030 as *mut u32, 0xe277feb8);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_HOUT ", 0x4002b034 as *mut u32, 0x7a049a2e);
    singlecheck("COMBOHASH_SFR_SEGPTR_SEGID_HOUT2", 0x4002b03c as *mut u32, 0x0bfe5c16);
    crate::snap_ticks("aes:: ");
    singlecheck("AES_SFR_CRFUNC           ", 0x4002d000 as *mut u32, 0x113a67ce);
    singlecheck("AES_SFR_AR               ", 0x4002d004 as *mut u32, 0x6d78b93f);
    singlecheck("AES_SFR_SRMFSM           ", 0x4002d008 as *mut u32, 0x2ba220e8);
    singlecheck("AES_SFR_FR               ", 0x4002d00c as *mut u32, 0x0cd2744e);
    singlecheck("AES_SFR_OPT              ", 0x4002d010 as *mut u32, 0x0bbdfeda);
    singlecheck("AES_SFR_OPT1             ", 0x4002d014 as *mut u32, 0xd98cd9f0);
    singlecheck("AES_SFR_OPTLTX           ", 0x4002d018 as *mut u32, 0x4bcf076d);
    singlecheck("AES_SFR_MASKSEED         ", 0x4002d020 as *mut u32, 0x635c38da);
    singlecheck("AES_SFR_MASKSEEDAR       ", 0x4002d024 as *mut u32, 0x00248421);
    singlecheck("AES_SFR_SEGPTR_PTRID_IV  ", 0x4002d030 as *mut u32, 0x11eacff0);
    singlecheck("AES_SFR_SEGPTR_PTRID_AKEY", 0x4002d034 as *mut u32, 0xb35514d6);
    singlecheck("AES_SFR_SEGPTR_PTRID_AIB ", 0x4002d038 as *mut u32, 0xd4d2d3a0);
    singlecheck("AES_SFR_SEGPTR_PTRID_AOB ", 0x4002d03c as *mut u32, 0x0903eb72);
    crate::snap_ticks("udma_adc:: ");
    crate::snap_ticks("udma_spis_1:: ");
    singlecheck("UDMA_SPIS_1_REG_RX_SADDR   ", 0x50113000 as *mut u32, 0x7770aaf1);
    singlecheck("UDMA_SPIS_1_REG_RX_SIZE    ", 0x50113004 as *mut u32, 0x52694c99);
    singlecheck("UDMA_SPIS_1_REG_RX_CFG     ", 0x50113008 as *mut u32, 0x28768923);
    singlecheck("UDMA_SPIS_1_REG_TX_SADDR   ", 0x50113010 as *mut u32, 0xe9b27f64);
    singlecheck("UDMA_SPIS_1_REG_TX_SIZE    ", 0x50113014 as *mut u32, 0x5c623642);
    singlecheck("UDMA_SPIS_1_REG_TX_CFG     ", 0x50113018 as *mut u32, 0x50ea0811);
    singlecheck("UDMA_SPIS_1_REG_SPIS_SETUP ", 0x50113020 as *mut u32, 0xa324343f);
    singlecheck("UDMA_SPIS_1_REG_SEOT_CNT   ", 0x50113024 as *mut u32, 0xbd699c09);
    singlecheck("UDMA_SPIS_1_REG_SPIS_IRQ_EN", 0x50113028 as *mut u32, 0x48ede8ec);
    singlecheck("UDMA_SPIS_1_REG_SPIS_RXCNT ", 0x5011302c as *mut u32, 0xb67e84f9);
    singlecheck("UDMA_SPIS_1_REG_SPIS_TXCNT ", 0x50113030 as *mut u32, 0x0d9c2026);
    singlecheck("UDMA_SPIS_1_REG_SPIS_DMCNT ", 0x50113034 as *mut u32, 0x07c0e820);
    crate::snap_ticks("udma_spis_0:: ");
    singlecheck("UDMA_SPIS_0_REG_RX_SADDR   ", 0x50112000 as *mut u32, 0x3a3454a3);
    singlecheck("UDMA_SPIS_0_REG_RX_SIZE    ", 0x50112004 as *mut u32, 0x891f14af);
    singlecheck("UDMA_SPIS_0_REG_RX_CFG     ", 0x50112008 as *mut u32, 0x9ba73b8c);
    singlecheck("UDMA_SPIS_0_REG_TX_SADDR   ", 0x50112010 as *mut u32, 0xeb3f20ee);
    singlecheck("UDMA_SPIS_0_REG_TX_SIZE    ", 0x50112014 as *mut u32, 0xd8f0efba);
    singlecheck("UDMA_SPIS_0_REG_TX_CFG     ", 0x50112018 as *mut u32, 0xb91d735e);
    singlecheck("UDMA_SPIS_0_REG_SPIS_SETUP ", 0x50112020 as *mut u32, 0xa9b5c06c);
    singlecheck("UDMA_SPIS_0_REG_SEOT_CNT   ", 0x50112024 as *mut u32, 0xf5947bc7);
    singlecheck("UDMA_SPIS_0_REG_SPIS_IRQ_EN", 0x50112028 as *mut u32, 0xff162495);
    singlecheck("UDMA_SPIS_0_REG_SPIS_RXCNT ", 0x5011202c as *mut u32, 0xeed0ea25);
    singlecheck("UDMA_SPIS_0_REG_SPIS_TXCNT ", 0x50112030 as *mut u32, 0x735eefe6);
    singlecheck("UDMA_SPIS_0_REG_SPIS_DMCNT ", 0x50112034 as *mut u32, 0x39b224e5);
    crate::snap_ticks("udma_scif:: ");
    singlecheck("UDMA_SCIF_REG_RX_SADDR  ", 0x50111000 as *mut u32, 0x891e962a);
    singlecheck("UDMA_SCIF_REG_RX_SIZE   ", 0x50111004 as *mut u32, 0x4ce60b12);
    singlecheck("UDMA_SCIF_REG_RX_CFG    ", 0x50111008 as *mut u32, 0x109d1dce);
    singlecheck("UDMA_SCIF_REG_TX_SADDR  ", 0x50111010 as *mut u32, 0xb35f3da7);
    singlecheck("UDMA_SCIF_REG_TX_SIZE   ", 0x50111014 as *mut u32, 0x2886b4b5);
    singlecheck("UDMA_SCIF_REG_TX_CFG    ", 0x50111018 as *mut u32, 0xf66a7449);
    singlecheck("UDMA_SCIF_REG_STATUS    ", 0x50111020 as *mut u32, 0x5ced9bad);
    singlecheck("UDMA_SCIF_REG_SCIF_SETUP", 0x50111024 as *mut u32, 0xa4624e76);
    singlecheck("UDMA_SCIF_REG_ERROR     ", 0x50111028 as *mut u32, 0x05e2db1f);
    singlecheck("UDMA_SCIF_REG_IRQ_EN    ", 0x5011102c as *mut u32, 0xabbab4f7);
    singlecheck("UDMA_SCIF_REG_VALID     ", 0x50111030 as *mut u32, 0xd1b6eefb);
    singlecheck("UDMA_SCIF_REG_DATA      ", 0x50111034 as *mut u32, 0x42e685fb);
    singlecheck("UDMA_SCIF_REG_SCIF_ETU  ", 0x50111038 as *mut u32, 0xfef8e5eb);
    crate::snap_ticks("udma_filter:: ");
    singlecheck("UDMA_FILTER_REG_TX_CH0_ADD ", 0x50110000 as *mut u32, 0x8ad6a150);
    singlecheck("UDMA_FILTER_REG_TX_CH0_CFG ", 0x50110004 as *mut u32, 0x133003a2);
    singlecheck("UDMA_FILTER_REG_TX_CH0_LEN0", 0x50110008 as *mut u32, 0x9a4c7c39);
    singlecheck("UDMA_FILTER_REG_TX_CH0_LEN1", 0x5011000c as *mut u32, 0x3bd0841b);
    singlecheck("UDMA_FILTER_REG_TX_CH0_LEN2", 0x50110010 as *mut u32, 0x20e56ca0);
    singlecheck("UDMA_FILTER_REG_TX_CH1_ADD ", 0x50110014 as *mut u32, 0x4b392a8a);
    singlecheck("UDMA_FILTER_REG_TX_CH1_CFG ", 0x50110018 as *mut u32, 0x6ec21a9f);
    singlecheck("UDMA_FILTER_REG_TX_CH1_LEN0", 0x5011001c as *mut u32, 0x6f30ce42);
    singlecheck("UDMA_FILTER_REG_TX_CH1_LEN1", 0x50110020 as *mut u32, 0x410098f0);
    singlecheck("UDMA_FILTER_REG_TX_CH1_LEN2", 0x50110024 as *mut u32, 0xc0ca2ed6);
    singlecheck("UDMA_FILTER_REG_RX_CH_ADD  ", 0x50110028 as *mut u32, 0xe54cdb7f);
    singlecheck("UDMA_FILTER_REG_RX_CH_CFG  ", 0x5011002c as *mut u32, 0x8d9755d1);
    singlecheck("UDMA_FILTER_REG_RX_CH_LEN0 ", 0x50110030 as *mut u32, 0xe2711846);
    singlecheck("UDMA_FILTER_REG_RX_CH_LEN1 ", 0x50110034 as *mut u32, 0x7df13d55);
    singlecheck("UDMA_FILTER_REG_RX_CH_LEN2 ", 0x50110038 as *mut u32, 0xb5b03cd4);
    singlecheck("UDMA_FILTER_REG_AU_CFG     ", 0x5011003c as *mut u32, 0x4261fee8);
    singlecheck("UDMA_FILTER_REG_AU_REG0    ", 0x50110040 as *mut u32, 0x28ee3537);
    singlecheck("UDMA_FILTER_REG_AU_REG1    ", 0x50110044 as *mut u32, 0xd32648e5);
    singlecheck("UDMA_FILTER_REG_BINCU_TH   ", 0x50110048 as *mut u32, 0xb9aa5368);
    singlecheck("UDMA_FILTER_REG_BINCU_CNT  ", 0x5011004c as *mut u32, 0x461efb45);
    singlecheck("UDMA_FILTER_REG_BINCU_SETUP", 0x50110050 as *mut u32, 0x1c8830a8);
    singlecheck("UDMA_FILTER_REG_BINCU_VAL  ", 0x50110054 as *mut u32, 0x6b6f2785);
    singlecheck("UDMA_FILTER_REG_FILT       ", 0x50110058 as *mut u32, 0xb3d49513);
    singlecheck("UDMA_FILTER_REG_STATUS     ", 0x50110060 as *mut u32, 0x21896835);
    crate::snap_ticks("udma_camera:: ");
    singlecheck("UDMA_CAMERA_REG_RX_SADDR          ", 0x5010f000 as *mut u32, 0x4e620692);
    singlecheck("UDMA_CAMERA_REG_RX_SIZE           ", 0x5010f004 as *mut u32, 0xd7c54ffe);
    singlecheck("UDMA_CAMERA_REG_RX_CFG            ", 0x5010f008 as *mut u32, 0x6c14ab40);
    singlecheck("UDMA_CAMERA_REG_CAM_CFG_GLOB      ", 0x5010f020 as *mut u32, 0x4824b7a4);
    singlecheck("UDMA_CAMERA_REG_CAM_CFG_LL        ", 0x5010f024 as *mut u32, 0x9bac06a6);
    singlecheck("UDMA_CAMERA_REG_CAM_CFG_UR        ", 0x5010f028 as *mut u32, 0xedaa8475);
    singlecheck("UDMA_CAMERA_REG_CAM_CFG_SIZE      ", 0x5010f02c as *mut u32, 0xb9812051);
    singlecheck("UDMA_CAMERA_REG_CAM_CFG_FILTER    ", 0x5010f030 as *mut u32, 0x123d2cde);
    singlecheck("UDMA_CAMERA_REG_CAM_VSYNC_POLARITY", 0x5010f034 as *mut u32, 0x9d02a659);
    crate::snap_ticks("udma_i2s:: ");
    singlecheck("UDMA_I2S_REG_RX_SADDR        ", 0x5010e000 as *mut u32, 0xd94ae36c);
    singlecheck("UDMA_I2S_REG_RX_SIZE         ", 0x5010e004 as *mut u32, 0x8b853bae);
    singlecheck("UDMA_I2S_REG_RX_CFG          ", 0x5010e008 as *mut u32, 0x35399729);
    singlecheck("UDMA_I2S_REG_TX_SADDR        ", 0x5010e010 as *mut u32, 0x402f7565);
    singlecheck("UDMA_I2S_REG_TX_SIZE         ", 0x5010e014 as *mut u32, 0x708eb5f7);
    singlecheck("UDMA_I2S_REG_TX_CFG          ", 0x5010e018 as *mut u32, 0xfcb510c3);
    singlecheck("UDMA_I2S_REG_I2S_CLKCFG_SETUP", 0x5010e020 as *mut u32, 0xd56c1055);
    singlecheck("UDMA_I2S_REG_I2S_SLV_SETUP   ", 0x5010e024 as *mut u32, 0x649b0645);
    singlecheck("UDMA_I2S_REG_I2S_MST_SETUP   ", 0x5010e028 as *mut u32, 0x7c18587b);
    singlecheck("UDMA_I2S_REG_I2S_PDM_SETUP   ", 0x5010e02c as *mut u32, 0xe718abba);
    crate::snap_ticks("udma_sdio:: ");
    singlecheck("UDMA_SDIO_REG_RX_SADDR  ", 0x5010d000 as *mut u32, 0x3de7972b);
    singlecheck("UDMA_SDIO_REG_RX_SIZE   ", 0x5010d004 as *mut u32, 0x2ba1f4df);
    singlecheck("UDMA_SDIO_REG_RX_CFG    ", 0x5010d008 as *mut u32, 0xc5978800);
    singlecheck("UDMA_SDIO_REG_TX_SADDR  ", 0x5010d010 as *mut u32, 0x899a3bc2);
    singlecheck("UDMA_SDIO_REG_TX_SIZE   ", 0x5010d014 as *mut u32, 0x9bdd4c0f);
    singlecheck("UDMA_SDIO_REG_TX_CFG    ", 0x5010d018 as *mut u32, 0xc69b4299);
    singlecheck("UDMA_SDIO_REG_CMD_OP    ", 0x5010d020 as *mut u32, 0xc90e49a5);
    singlecheck("UDMA_SDIO_REG_DATA_SETUP", 0x5010d028 as *mut u32, 0x77eca177);
    singlecheck("UDMA_SDIO_REG_START     ", 0x5010d02c as *mut u32, 0xabea588e);
    singlecheck("UDMA_SDIO_REG_RSP0      ", 0x5010d030 as *mut u32, 0xa45365c4);
    singlecheck("UDMA_SDIO_REG_RSP1      ", 0x5010d034 as *mut u32, 0xa5aa10d4);
    singlecheck("UDMA_SDIO_REG_RSP2      ", 0x5010d038 as *mut u32, 0x7ed4cd32);
    singlecheck("UDMA_SDIO_REG_RSP3      ", 0x5010d03c as *mut u32, 0x0ecc4ea7);
    singlecheck("UDMA_SDIO_REG_CLK_DIV   ", 0x5010d040 as *mut u32, 0xacdf81d9);
    singlecheck("UDMA_SDIO_REG_STATUS    ", 0x5010d044 as *mut u32, 0x3c4cccec);
    crate::snap_ticks("udma_i2c_3:: ");
    singlecheck("UDMA_I2C_3_REG_RX_SADDR ", 0x5010c000 as *mut u32, 0x76af56ac);
    singlecheck("UDMA_I2C_3_REG_RX_SIZE  ", 0x5010c004 as *mut u32, 0xfe866451);
    singlecheck("UDMA_I2C_3_REG_RX_CFG   ", 0x5010c008 as *mut u32, 0x0f8a44da);
    singlecheck("UDMA_I2C_3_REG_TX_SADDR ", 0x5010c010 as *mut u32, 0x70f44b4c);
    singlecheck("UDMA_I2C_3_REG_TX_SIZE  ", 0x5010c014 as *mut u32, 0x95672258);
    singlecheck("UDMA_I2C_3_REG_TX_CFG   ", 0x5010c018 as *mut u32, 0x1b628c89);
    singlecheck("UDMA_I2C_3_REG_CMD_SADDR", 0x5010c020 as *mut u32, 0xe9b4c599);
    singlecheck("UDMA_I2C_3_REG_CMD_SIZE ", 0x5010c024 as *mut u32, 0x465ffceb);
    singlecheck("UDMA_I2C_3_REG_CMD_CFG  ", 0x5010c028 as *mut u32, 0x8a306f8c);
    singlecheck("UDMA_I2C_3_REG_STATUS   ", 0x5010c030 as *mut u32, 0xf4d97b07);
    singlecheck("UDMA_I2C_3_REG_SETUP    ", 0x5010c034 as *mut u32, 0x00240aa9);
    singlecheck("UDMA_I2C_3_REG_ACK      ", 0x5010c038 as *mut u32, 0x00befa5f);
    crate::snap_ticks("udma_i2c_2:: ");
    singlecheck("UDMA_I2C_2_REG_RX_SADDR ", 0x5010b000 as *mut u32, 0x86b3e63e);
    singlecheck("UDMA_I2C_2_REG_RX_SIZE  ", 0x5010b004 as *mut u32, 0x9c230f74);
    singlecheck("UDMA_I2C_2_REG_RX_CFG   ", 0x5010b008 as *mut u32, 0x692aac58);
    singlecheck("UDMA_I2C_2_REG_TX_SADDR ", 0x5010b010 as *mut u32, 0x6e68234e);
    singlecheck("UDMA_I2C_2_REG_TX_SIZE  ", 0x5010b014 as *mut u32, 0x4c264c37);
    singlecheck("UDMA_I2C_2_REG_TX_CFG   ", 0x5010b018 as *mut u32, 0x58110437);
    singlecheck("UDMA_I2C_2_REG_CMD_SADDR", 0x5010b020 as *mut u32, 0xdd63793a);
    singlecheck("UDMA_I2C_2_REG_CMD_SIZE ", 0x5010b024 as *mut u32, 0x10e9e0f2);
    singlecheck("UDMA_I2C_2_REG_CMD_CFG  ", 0x5010b028 as *mut u32, 0xa201f3f0);
    singlecheck("UDMA_I2C_2_REG_STATUS   ", 0x5010b030 as *mut u32, 0x2f5a1fdb);
    singlecheck("UDMA_I2C_2_REG_SETUP    ", 0x5010b034 as *mut u32, 0x0b4c5413);
    singlecheck("UDMA_I2C_2_REG_ACK      ", 0x5010b038 as *mut u32, 0xd51be953);
    crate::snap_ticks("udma_i2c_1:: ");
    singlecheck("UDMA_I2C_1_REG_RX_SADDR ", 0x5010a000 as *mut u32, 0xc45068b5);
    singlecheck("UDMA_I2C_1_REG_RX_SIZE  ", 0x5010a004 as *mut u32, 0xb50c4faf);
    singlecheck("UDMA_I2C_1_REG_RX_CFG   ", 0x5010a008 as *mut u32, 0xadc9b1ee);
    singlecheck("UDMA_I2C_1_REG_TX_SADDR ", 0x5010a010 as *mut u32, 0x65ca93c0);
    singlecheck("UDMA_I2C_1_REG_TX_SIZE  ", 0x5010a014 as *mut u32, 0xa5e7f3a4);
    singlecheck("UDMA_I2C_1_REG_TX_CFG   ", 0x5010a018 as *mut u32, 0x4da50393);
    singlecheck("UDMA_I2C_1_REG_CMD_SADDR", 0x5010a020 as *mut u32, 0x0492b289);
    singlecheck("UDMA_I2C_1_REG_CMD_SIZE ", 0x5010a024 as *mut u32, 0x2cbc8cf4);
    singlecheck("UDMA_I2C_1_REG_CMD_CFG  ", 0x5010a028 as *mut u32, 0x02301377);
    singlecheck("UDMA_I2C_1_REG_STATUS   ", 0x5010a030 as *mut u32, 0x431d0ee5);
    singlecheck("UDMA_I2C_1_REG_SETUP    ", 0x5010a034 as *mut u32, 0x7850ffca);
    singlecheck("UDMA_I2C_1_REG_ACK      ", 0x5010a038 as *mut u32, 0xfcee3352);
    crate::snap_ticks("udma_i2c_0:: ");
    singlecheck("UDMA_I2C_0_REG_RX_SADDR ", 0x50109000 as *mut u32, 0xc9df85a9);
    singlecheck("UDMA_I2C_0_REG_RX_SIZE  ", 0x50109004 as *mut u32, 0xafff738f);
    singlecheck("UDMA_I2C_0_REG_RX_CFG   ", 0x50109008 as *mut u32, 0x1a87b4b6);
    singlecheck("UDMA_I2C_0_REG_TX_SADDR ", 0x50109010 as *mut u32, 0x68fe126e);
    singlecheck("UDMA_I2C_0_REG_TX_SIZE  ", 0x50109014 as *mut u32, 0xc5bf491a);
    singlecheck("UDMA_I2C_0_REG_TX_CFG   ", 0x50109018 as *mut u32, 0xba2d396e);
    singlecheck("UDMA_I2C_0_REG_CMD_SADDR", 0x50109020 as *mut u32, 0x55432468);
    singlecheck("UDMA_I2C_0_REG_CMD_SIZE ", 0x50109024 as *mut u32, 0xd6403f3b);
    singlecheck("UDMA_I2C_0_REG_CMD_CFG  ", 0x50109028 as *mut u32, 0x6ffd2e7b);
    singlecheck("UDMA_I2C_0_REG_STATUS   ", 0x50109030 as *mut u32, 0x139c7ba9);
    singlecheck("UDMA_I2C_0_REG_SETUP    ", 0x50109034 as *mut u32, 0xf7dab367);
    singlecheck("UDMA_I2C_0_REG_ACK      ", 0x50109038 as *mut u32, 0x445d1ced);
    crate::snap_ticks("udma_spim_3:: ");
    singlecheck("UDMA_SPIM_3_REG_RX_SADDR ", 0x50108000 as *mut u32, 0xfb11725c);
    singlecheck("UDMA_SPIM_3_REG_RX_SIZE  ", 0x50108004 as *mut u32, 0x1859d620);
    singlecheck("UDMA_SPIM_3_REG_RX_CFG   ", 0x50108008 as *mut u32, 0x36afce41);
    singlecheck("UDMA_SPIM_3_REG_TX_SADDR ", 0x50108010 as *mut u32, 0xecf246d6);
    singlecheck("UDMA_SPIM_3_REG_TX_SIZE  ", 0x50108014 as *mut u32, 0x9dc2d207);
    singlecheck("UDMA_SPIM_3_REG_TX_CFG   ", 0x50108018 as *mut u32, 0x1e0c72a7);
    singlecheck("UDMA_SPIM_3_REG_CMD_SADDR", 0x50108020 as *mut u32, 0x76e7f324);
    singlecheck("UDMA_SPIM_3_REG_CMD_SIZE ", 0x50108024 as *mut u32, 0x53a9bb93);
    singlecheck("UDMA_SPIM_3_REG_CMD_CFG  ", 0x50108028 as *mut u32, 0xcad803c7);
    singlecheck("UDMA_SPIM_3_REG_STATUS   ", 0x50108030 as *mut u32, 0x92e02a7f);
    crate::snap_ticks("udma_spim_2:: ");
    singlecheck("UDMA_SPIM_2_REG_RX_SADDR ", 0x50107000 as *mut u32, 0x42baecc4);
    singlecheck("UDMA_SPIM_2_REG_RX_SIZE  ", 0x50107004 as *mut u32, 0x21ed33b2);
    singlecheck("UDMA_SPIM_2_REG_RX_CFG   ", 0x50107008 as *mut u32, 0x9253dbd4);
    singlecheck("UDMA_SPIM_2_REG_TX_SADDR ", 0x50107010 as *mut u32, 0xb639268e);
    singlecheck("UDMA_SPIM_2_REG_TX_SIZE  ", 0x50107014 as *mut u32, 0x7987f106);
    singlecheck("UDMA_SPIM_2_REG_TX_CFG   ", 0x50107018 as *mut u32, 0x91516863);
    singlecheck("UDMA_SPIM_2_REG_CMD_SADDR", 0x50107020 as *mut u32, 0x4472f560);
    singlecheck("UDMA_SPIM_2_REG_CMD_SIZE ", 0x50107024 as *mut u32, 0x21d2dd57);
    singlecheck("UDMA_SPIM_2_REG_CMD_CFG  ", 0x50107028 as *mut u32, 0x326d2d8a);
    singlecheck("UDMA_SPIM_2_REG_STATUS   ", 0x50107030 as *mut u32, 0x780d05e8);
    crate::snap_ticks("udma_spim_1:: ");
    singlecheck("UDMA_SPIM_1_REG_RX_SADDR ", 0x50106000 as *mut u32, 0x7060de7e);
    singlecheck("UDMA_SPIM_1_REG_RX_SIZE  ", 0x50106004 as *mut u32, 0x1f498bd9);
    singlecheck("UDMA_SPIM_1_REG_RX_CFG   ", 0x50106008 as *mut u32, 0x89f69f87);
    singlecheck("UDMA_SPIM_1_REG_TX_SADDR ", 0x50106010 as *mut u32, 0x42a14429);
    singlecheck("UDMA_SPIM_1_REG_TX_SIZE  ", 0x50106014 as *mut u32, 0x5def5495);
    singlecheck("UDMA_SPIM_1_REG_TX_CFG   ", 0x50106018 as *mut u32, 0xc5aefd8b);
    singlecheck("UDMA_SPIM_1_REG_CMD_SADDR", 0x50106020 as *mut u32, 0x43a3a1da);
    singlecheck("UDMA_SPIM_1_REG_CMD_SIZE ", 0x50106024 as *mut u32, 0xcf882c7e);
    singlecheck("UDMA_SPIM_1_REG_CMD_CFG  ", 0x50106028 as *mut u32, 0x5bfd3f62);
    singlecheck("UDMA_SPIM_1_REG_STATUS   ", 0x50106030 as *mut u32, 0x710800ea);
    crate::snap_ticks("udma_spim_0:: ");
    singlecheck("UDMA_SPIM_0_REG_RX_SADDR ", 0x50105000 as *mut u32, 0xa51070d0);
    singlecheck("UDMA_SPIM_0_REG_RX_SIZE  ", 0x50105004 as *mut u32, 0xfef8b481);
    singlecheck("UDMA_SPIM_0_REG_RX_CFG   ", 0x50105008 as *mut u32, 0xd83fecb5);
    singlecheck("UDMA_SPIM_0_REG_TX_SADDR ", 0x50105010 as *mut u32, 0xfadd3dc6);
    singlecheck("UDMA_SPIM_0_REG_TX_SIZE  ", 0x50105014 as *mut u32, 0xc9224d40);
    singlecheck("UDMA_SPIM_0_REG_TX_CFG   ", 0x50105018 as *mut u32, 0x9061776a);
    singlecheck("UDMA_SPIM_0_REG_CMD_SADDR", 0x50105020 as *mut u32, 0x97a1a69b);
    singlecheck("UDMA_SPIM_0_REG_CMD_SIZE ", 0x50105024 as *mut u32, 0x7dfb3951);
    singlecheck("UDMA_SPIM_0_REG_CMD_CFG  ", 0x50105028 as *mut u32, 0x034a9b60);
    singlecheck("UDMA_SPIM_0_REG_STATUS   ", 0x50105030 as *mut u32, 0x6bd88f0f);
    crate::snap_ticks("udma_uart_3:: ");
    singlecheck("UDMA_UART_3_REG_RX_SADDR  ", 0x50104000 as *mut u32, 0x606f749d);
    singlecheck("UDMA_UART_3_REG_RX_SIZE   ", 0x50104004 as *mut u32, 0xc1574740);
    singlecheck("UDMA_UART_3_REG_RX_CFG    ", 0x50104008 as *mut u32, 0xb2580f59);
    singlecheck("UDMA_UART_3_REG_TX_SADDR  ", 0x50104010 as *mut u32, 0x6f9d9f80);
    singlecheck("UDMA_UART_3_REG_TX_SIZE   ", 0x50104014 as *mut u32, 0xbf14aab4);
    singlecheck("UDMA_UART_3_REG_TX_CFG    ", 0x50104018 as *mut u32, 0xdcab8379);
    singlecheck("UDMA_UART_3_REG_STATUS    ", 0x50104020 as *mut u32, 0xd04c9cd4);
    singlecheck("UDMA_UART_3_REG_UART_SETUP", 0x50104024 as *mut u32, 0xb99174ef);
    singlecheck("UDMA_UART_3_REG_ERROR     ", 0x50104028 as *mut u32, 0x8af00e31);
    singlecheck("UDMA_UART_3_REG_IRQ_EN    ", 0x5010402c as *mut u32, 0x168ccf6e);
    singlecheck("UDMA_UART_3_REG_VALID     ", 0x50104030 as *mut u32, 0x6977ce68);
    singlecheck("UDMA_UART_3_REG_DATA      ", 0x50104034 as *mut u32, 0xc6dcf381);
    crate::snap_ticks("udma_uart_2:: ");
    singlecheck("UDMA_UART_2_REG_RX_SADDR  ", 0x50103000 as *mut u32, 0x88c5f697);
    singlecheck("UDMA_UART_2_REG_RX_SIZE   ", 0x50103004 as *mut u32, 0x5134b237);
    singlecheck("UDMA_UART_2_REG_RX_CFG    ", 0x50103008 as *mut u32, 0x734e239b);
    singlecheck("UDMA_UART_2_REG_TX_SADDR  ", 0x50103010 as *mut u32, 0x6cf51de4);
    singlecheck("UDMA_UART_2_REG_TX_SIZE   ", 0x50103014 as *mut u32, 0x9dc771a8);
    singlecheck("UDMA_UART_2_REG_TX_CFG    ", 0x50103018 as *mut u32, 0x8dfbe7df);
    singlecheck("UDMA_UART_2_REG_STATUS    ", 0x50103020 as *mut u32, 0x23f3c880);
    singlecheck("UDMA_UART_2_REG_UART_SETUP", 0x50103024 as *mut u32, 0xb2398b52);
    singlecheck("UDMA_UART_2_REG_ERROR     ", 0x50103028 as *mut u32, 0x3e755420);
    singlecheck("UDMA_UART_2_REG_IRQ_EN    ", 0x5010302c as *mut u32, 0xed19242a);
    singlecheck("UDMA_UART_2_REG_VALID     ", 0x50103030 as *mut u32, 0x66764ed9);
    singlecheck("UDMA_UART_2_REG_DATA      ", 0x50103034 as *mut u32, 0x73de0aaa);
    crate::snap_ticks("udma_uart_1:: ");
    singlecheck("UDMA_UART_1_REG_RX_SADDR  ", 0x50102000 as *mut u32, 0x9904b859);
    singlecheck("UDMA_UART_1_REG_RX_SIZE   ", 0x50102004 as *mut u32, 0xbe87f587);
    singlecheck("UDMA_UART_1_REG_RX_CFG    ", 0x50102008 as *mut u32, 0xd4d49c42);
    singlecheck("UDMA_UART_1_REG_TX_SADDR  ", 0x50102010 as *mut u32, 0x18331146);
    singlecheck("UDMA_UART_1_REG_TX_SIZE   ", 0x50102014 as *mut u32, 0xab6bf8ad);
    singlecheck("UDMA_UART_1_REG_TX_CFG    ", 0x50102018 as *mut u32, 0x62448d36);
    singlecheck("UDMA_UART_1_REG_STATUS    ", 0x50102020 as *mut u32, 0x8bfb617d);
    singlecheck("UDMA_UART_1_REG_UART_SETUP", 0x50102024 as *mut u32, 0x1ea8018a);
    singlecheck("UDMA_UART_1_REG_ERROR     ", 0x50102028 as *mut u32, 0x7a368f54);
    singlecheck("UDMA_UART_1_REG_IRQ_EN    ", 0x5010202c as *mut u32, 0x559fc045);
    singlecheck("UDMA_UART_1_REG_VALID     ", 0x50102030 as *mut u32, 0xd4119e64);
    singlecheck("UDMA_UART_1_REG_DATA      ", 0x50102034 as *mut u32, 0x10cc0b5c);
    crate::snap_ticks("udma_uart_0:: ");
    singlecheck("UDMA_UART_0_REG_RX_SADDR  ", 0x50101000 as *mut u32, 0xdcc54c52);
    singlecheck("UDMA_UART_0_REG_RX_SIZE   ", 0x50101004 as *mut u32, 0x13defb65);
    singlecheck("UDMA_UART_0_REG_RX_CFG    ", 0x50101008 as *mut u32, 0x53c98197);
    singlecheck("UDMA_UART_0_REG_TX_SADDR  ", 0x50101010 as *mut u32, 0x6fdb0203);
    singlecheck("UDMA_UART_0_REG_TX_SIZE   ", 0x50101014 as *mut u32, 0x1013eea7);
    singlecheck("UDMA_UART_0_REG_TX_CFG    ", 0x50101018 as *mut u32, 0xe4ad2692);
    singlecheck("UDMA_UART_0_REG_STATUS    ", 0x50101020 as *mut u32, 0x3c377e25);
    singlecheck("UDMA_UART_0_REG_UART_SETUP", 0x50101024 as *mut u32, 0x409d2fed);
    singlecheck("UDMA_UART_0_REG_ERROR     ", 0x50101028 as *mut u32, 0x73b65ce0);
    singlecheck("UDMA_UART_0_REG_IRQ_EN    ", 0x5010102c as *mut u32, 0x7247ca65);
    singlecheck("UDMA_UART_0_REG_VALID     ", 0x50101030 as *mut u32, 0xeab5086b);
    singlecheck("UDMA_UART_0_REG_DATA      ", 0x50101034 as *mut u32, 0x0b8b322f);
    crate::snap_ticks("udma_ctrl:: ");
    singlecheck("UDMA_CTRL_REG_CG     ", 0x50100000 as *mut u32, 0xb637a7a7);
    singlecheck("UDMA_CTRL_REG_CFG_EVT", 0x50100004 as *mut u32, 0x0653af84);
    singlecheck("UDMA_CTRL_REG_RST    ", 0x50100008 as *mut u32, 0xf96d778b);
    singlecheckread("udc            ", 0x50202000 as *const u32);
    singlecheckread("udc            ", 0x50202004 as *const u32);
    singlecheckread("udc            ", 0x502020fc as *const u32);
    singlecheckread("udc            ", 0x50202084 as *const u32);
    singlecheckread("udc            ", 0x50202400 as *const u32);
    singlecheckread("udc            ", 0x50202410 as *const u32);
    singlecheckread("udc            ", 0x50202414 as *const u32);
    singlecheckread("udc            ", 0x502024fc as *const u32);
    singlecheckread("udc            ", 0x50202484 as *const u32);

    loop {
        uart.tiny_write_str("scan done\n");
    }
}
