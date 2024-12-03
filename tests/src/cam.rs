use cramium_hal::ifram::IframRange;
use cramium_hal::ifram::UdmaWidths;
use cramium_hal::iox::IoSetup;
use cramium_hal::iox::*;
use cramium_hal::ov2640::Format;
use cramium_hal::udma::Udma;
use cramium_hal::udma::*;
use utralib::CSR;
use utralib::utra;
use utralib::utra::udma_camera::REG_CAM_CFG_GLOB;

pub const CAM_IFRAM_LEN_PAGES: usize = 30;
pub const CAM_IFRAM_ADDR: usize =
    utralib::HW_IFRAM1_MEM + utralib::HW_IFRAM1_MEM_LEN - CAM_IFRAM_LEN_PAGES * 4096;

use crate::*;

pub const CFG_FRAMEDROP_EN: utralib::Field = utralib::Field::new(1, 0, REG_CAM_CFG_GLOB);
#[allow(dead_code)]
pub const CFG_FRAMEDROP_VAL: utralib::Field = utralib::Field::new(6, 1, REG_CAM_CFG_GLOB);
pub const CFG_FRAMESLICE_EN: utralib::Field = utralib::Field::new(1, 7, REG_CAM_CFG_GLOB);
pub const CFG_FORMAT: utralib::Field = utralib::Field::new(3, 8, REG_CAM_CFG_GLOB);
pub const CFG_SHIFT: utralib::Field = utralib::Field::new(4, 11, REG_CAM_CFG_GLOB);
pub const CFG_GLOB_EN: utralib::Field = utralib::Field::new(1, 31, REG_CAM_CFG_GLOB);

pub(crate) const CFG_EN: u32 = 0b01_0000; // start a transfer
pub(crate) const CFG_SIZE_16: u32 = 0b00_0010; // 16-bit transfer

const ROWS: usize = 8;
const COLS: usize = 12;

const CAM_TESTS: usize = 1;
crate::impl_test!(CamTests, "Camera", CAM_TESTS);

pub struct TestCamera {
    csr: CSR<u32>,
    ifram: IframRange,
}
impl TestCamera {
    pub fn new() -> TestCamera {
        let mut csr = CSR::new(utra::udma_camera::HW_UDMA_CAMERA_BASE as *mut u32);
        let ifram =
            unsafe { IframRange::from_raw_parts(CAM_IFRAM_ADDR, CAM_IFRAM_ADDR, CAM_IFRAM_LEN_PAGES * 4096) };

        let vsync_pol = 0;
        let hsync_pol = 0;
        csr.wo(
            utra::udma_camera::REG_CAM_VSYNC_POLARITY,
            csr.ms(utra::udma_camera::REG_CAM_VSYNC_POLARITY_R_CAM_VSYNC_POLARITY, vsync_pol)
                | csr.ms(utra::udma_camera::REG_CAM_VSYNC_POLARITY_R_CAM_HSYNC_POLARITY, hsync_pol),
        );

        // multiply by 1
        csr.wo(utra::udma_camera::REG_CAM_CFG_FILTER, 0x01_01_01);

        let (x, _y) = (ROWS, COLS);
        csr.wo(utra::udma_camera::REG_CAM_CFG_SIZE, (x as u32 - 1) << 16);

        let global = csr.ms(CFG_FRAMEDROP_EN, 0)
            | csr.ms(CFG_FORMAT, Format::BypassLe as u32)
            | csr.ms(CFG_FRAMESLICE_EN, 0)
            | csr.ms(CFG_SHIFT, 0);
        csr.wo(utra::udma_camera::REG_CAM_CFG_GLOB, global);

        TestCamera { csr, ifram }
    }

    #[allow(dead_code)]
    pub fn rx_buf<T: UdmaWidths>(&self) -> &[T] { &self.ifram.as_slice() }

    pub unsafe fn rx_buf_phys<T: UdmaWidths>(&self) -> &[T] { &self.ifram.as_phys_slice() }
}

impl Udma for TestCamera {
    fn csr_mut(&mut self) -> &mut CSR<u32> { &mut self.csr }

    fn csr(&self) -> &CSR<u32> { &self.csr }
}

impl TestRunner for CamTests {
    fn run(&mut self) {
        // give all the pins to iox
        let iox = Iox::new(utra::iox::HW_IOX_BASE as *mut u32);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
        iox.set_ports_from_pio_bitmask(0x0);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));

        // camera interface proper
        for pin in 2..11 {
            iox.setup_pin(
                IoxPort::PB,
                pin,
                Some(IoxDir::Input),
                Some(IoxFunction::AF1),
                None,
                None,
                Some(IoxEnable::Enable),
                Some(IoxDriveStrength::Drive2mA),
            );
        }
        let mut tc = TestCamera::new();

        // initiate a capture
        let total_len = ROWS * COLS;
        crate::println!("frame 1");
        unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
        tc.csr_mut().rmwf(CFG_GLOB_EN, 1);

        while tc.udma_busy(Bank::Rx) {}

        // initiate a second, unsynchronized capture
        unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
        tc.csr_mut().rmwf(CFG_GLOB_EN, 1);
        crate::println!("frame 2");
        while tc.udma_busy(Bank::Rx) {}
        crate::println!("done");

        // revert to PIO ownership of I/O pins
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));

        // always "passes"
        self.passing_tests += 1;
    }
}
