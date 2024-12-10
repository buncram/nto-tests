use core::convert::TryInto;

use cramium_hal::board::SPIM_FLASH_IFRAM_ADDR;
use cramium_hal::ifram::IframRange;
use cramium_hal::iox::Iox;
use cramium_hal::iox::{IoSetup, IoxDir, IoxDriveStrength, IoxEnable, IoxFunction, IoxPort};
use cramium_hal::udma::*;

use crate::*;

// 2 for spim (one with reset in the middle)
// 1 for i2c
const UDMA_TESTS: usize = 2 + 1;
crate::impl_test!(UdmaTests, "UDMA", UDMA_TESTS);
impl TestRunner for UdmaTests {
    fn run(&mut self) {
        crate::println!("starting UDMA SPIM stress test");
        let udma_global = GlobalConfig::new(utralib::generated::HW_UDMA_CTRL_BASE as *mut u32);

        // setup the I/O pins
        let iox = Iox::new(utralib::generated::HW_IOX_BASE as *mut u32);

        // give regular I/O ownership of the I/O block, but just the UDMA pins
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
        iox.set_ports_from_pio_bitmask(0x00_7f_e7ff);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));

        self.passing_tests += self.i2c_test();

        let channel = cramium_hal::board::setup_memory_pins(&iox);
        udma_global.clock_on(PeriphId::from(channel));
        crate::println!("using SPIM channel {:?}", channel);

        for test_iter in 0..UDMA_TESTS {
            let mut flash_spim = unsafe {
                Spim::new_with_ifram(
                    channel,
                    25_000_000,
                    50_000_000,
                    SpimClkPol::LeadingEdgeRise,
                    SpimClkPha::CaptureOnLeading,
                    SpimCs::Cs0,
                    0,
                    0,
                    None,
                    16, // this is limited by the page length
                    4096,
                    Some(6),
                    None, // initially in default mode
                    IframRange::from_raw_parts(SPIM_FLASH_IFRAM_ADDR, SPIM_FLASH_IFRAM_ADDR, 4096 * 2),
                )
            };
            /*
            crate::println!("regression test");
            unsafe {
                let saddr = ram_spim.csr().base().add(Bank::Rx as usize).add(DmaReg::Saddr.into());
                crate::println!("saddr addr: {:x}", saddr as usize);
                let mut rbk = [0u32; 4];
                for (i, d) in rbk.iter_mut().enumerate() {
                    *saddr = 0xface_0000 + i as u32 * 4;
                    *d = saddr.read_volatile();
                }
                for r in rbk {
                    crate::println!("SADDR {:x}", r);
                }
            }
            crate::println!("regression test done");
            */
            crate::println!("zero dest");
            let mut dest = [0u8; 256];
            crate::println!("QPI mode");
            flash_spim.mem_qpi_mode(true);
            crate::println!("read ID");
            let ram_id = flash_spim.mem_read_id_flash();
            crate::println!("FLASH ID: {:x}", ram_id);
            crate::println!("initiate read");
            if flash_spim.mem_read(0x1000 + (test_iter as u32 + 1) * 16, &mut dest, false) {
                crate::println!("rom_read done!");
                for chunk in dest[..32].chunks(4) {
                    crate::println!("{:x}", u32::from_le_bytes(chunk.try_into().unwrap()));
                }
                for (i, chunk) in dest.chunks(4).enumerate() {
                    let checkval = u32::from_le_bytes(chunk.try_into().unwrap());
                    assert!(
                        checkval
                            == 0xface_8000 + i as u32 + (test_iter as u32 + 1) * 16 / size_of::<u32>() as u32
                    )
                }
                crate::println!("rom_read check passed!");
                self.passing_tests += 1;
            } else {
                crate::println!("rom_read failed");
            }

            // perform a reset
            crate::println!("resetting SPIM channel");
            udma_global.reset(PeriphId::from(channel));
        }

        // revert to PIO ownership of I/O pins
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
    }
}

pub const TEST_I2C_MASK: u32 = 0b00001;
pub const I2C_IFRAM_ADDR: usize = utralib::HW_IFRAM0_MEM + utralib::HW_IFRAM0_MEM_LEN - 8 * 4096;
impl UdmaTests {
    pub fn i2c_test(&mut self) -> usize {
        let mut test_cfg = CSR::new(utra::csrtest::HW_CSRTEST_BASE as *mut u32);
        test_cfg.wo(utra::csrtest::WTEST, 0);
        test_cfg.wo(utra::csrtest::WTEST, TEST_I2C_MASK);
    
        let perclk = 100_000_000;
        let udma_global = GlobalConfig::new(utralib::generated::HW_UDMA_CTRL_BASE as *mut u32);

        // setup the I/O pins
        let iox = Iox::new(utralib::generated::HW_IOX_BASE as *mut u32);
        let i2c_channel = setup_i2c_pins(&iox);
        udma_global.clock(PeriphId::from(i2c_channel), true);
        let i2c_ifram =
            unsafe { cramium_hal::ifram::IframRange::from_raw_parts(I2C_IFRAM_ADDR, I2C_IFRAM_ADDR, 4096) };
        let mut i2c = unsafe {
            cramium_hal::udma::I2c::new_with_ifram(i2c_channel, 400_000, perclk, i2c_ifram, &udma_global)
        };

        crate::println!("i2c test");
        for addr in 10..14 {
            i2c.i2c_write(addr, 0xA0u8 + addr as u8 - 10, &[]).expect("write failed");
            let mut rx = [0u8; 1];
            i2c.i2c_read(addr, 0xA0u8 + addr as u8 - 10, &mut rx, false).expect("read failed");
            crate::println!("i2c result: {:x?}", rx);
        }

        test_cfg.wo(utra::csrtest::WTEST, 0);

        1
    }
}

pub fn setup_i2c_pins(iox: &dyn IoSetup) -> crate::udma::I2cChannel {
    // I2C_SCL_B[0]
    iox.setup_pin(
        IoxPort::PB,
        11,
        Some(IoxDir::Output),
        Some(IoxFunction::AF1),
        None,
        None,
        Some(IoxEnable::Enable),
        Some(IoxDriveStrength::Drive2mA),
    );
    // I2C_SDA_B[0]
    iox.setup_pin(
        IoxPort::PB,
        12,
        Some(IoxDir::Output),
        Some(IoxFunction::AF1),
        Some(IoxEnable::Enable),
        None,
        Some(IoxEnable::Enable),
        Some(IoxDriveStrength::Drive2mA),
    );
    crate::udma::I2cChannel::Channel0
}
