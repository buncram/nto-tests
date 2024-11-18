use cramium_hal::board::SPIM_RAM_IFRAM_ADDR;
use cramium_hal::ifram::IframRange;
use cramium_hal::iox::Iox;
use cramium_hal::udma::*;

use crate::*;

const UDMA_TESTS: usize = 1;
crate::impl_test!(UdmaTests, "UDMA", UDMA_TESTS);
impl TestRunner for UdmaTests {
    fn run(&mut self) {
        crate::println!("starting UDMA SPIM stress test");
        let udma_global = GlobalConfig::new(utralib::generated::HW_UDMA_CTRL_BASE as *mut u32);

        // setup the I/O pins
        let iox = Iox::new(utralib::generated::HW_IOX_BASE as *mut u32);
        let channel = cramium_hal::board::setup_memory_pins(&iox);
        udma_global.clock_on(PeriphId::from(channel));
        crate::println!("using SPIM channel {:?}", channel);

        let mut ram_spim = unsafe {
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
                1024, // this is limited by the page length
                1024,
                Some(6),
                None, // initially in default mode
                IframRange::from_raw_parts(SPIM_RAM_IFRAM_ADDR, SPIM_RAM_IFRAM_ADDR, 4096),
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
        ram_spim.mem_qpi_mode(true);
        crate::println!("read ID");
        let ram_id = ram_spim.mem_read_id_ram();
        crate::println!("fake RAM ID: {:x}", ram_id);
        crate::println!("initiate read");
        if ram_spim.mem_read(0x55A00, &mut dest, false) {
            crate::println!("ram_read success!");
        } else {
            crate::println!("ram_read failed");
        }
    }
}
