use cramium_api::camera::Format;
use cramium_api::*;
use cramium_hal::ifram::IframRange;
use cramium_hal::iox::*;
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

#[allow(dead_code)]
pub(crate) const CFG_CONT: u32 = 0b00_0001; // continuous mode
pub(crate) const CFG_EN: u32 = 0b01_0000; // start a transfer
pub(crate) const CFG_SIZE_16: u32 = 0b00_0010; // 16-bit transfer

const ROWS: usize = 8;
const COLS: usize = 64;

const CAM_TESTS: usize = 1;
crate::impl_test!(CamTests, "Camera", CAM_TESTS);

pub struct TestCamera {
    csr: CSR<u32>,
    ifram: IframRange,
}
impl TestCamera {
    pub fn new() -> TestCamera {
        let udma_global = cramium_hal::udma::GlobalConfig::new();
        udma_global.clock_on(PeriphId::Cam);
        /*
        udma_global.map_event(
            PeriphId::Cam,
            PeriphEventType::Cam(EventCamOffset::Rx),
            EventChannel::Channel0,
        );*/
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
            | csr.ms(CFG_SHIFT, 0)
            | 1 << 30; // new field to activate sof snapping of rx
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
        unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
        tc.csr_mut().rmwf(CFG_GLOB_EN, 1);
        crate::println!("frame 1");

        // while tc.udma_busy(Bank::Rx) {}
        while (tc.csr.r(REG_CAM_CFG_GLOB) & 0x8000_0000) != 0 {}

        // initiate a second, poorly timed capture
        for _ in 0..7 {
            crate::println!("time passes...");
        }

        // Pixel pipe needs time to clear, use multiple writes to cause the necessary delay
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);
        for i in 0..2 {
            unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
            // this register needs to be read before written to reliably restart the pipe
            let _ = tc.csr().r(utra::udma_camera::REG_CAM_CFG_GLOB);
            tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x8000_0400);
            // while tc.udma_busy(Bank::Rx) {}
            while (tc.csr.r(REG_CAM_CFG_GLOB) & 0x8000_0000) != 0 {}
            crate::println!("frame {} done", i + 2);
        }

        // now confirm legacy behavior is OK by turning off the sync bit
        let val = tc.csr().r(REG_CAM_CFG_GLOB) & !0x4000_0000;
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, val);

        unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
        tc.csr_mut().rmwf(CFG_GLOB_EN, 1);
        crate::println!("legacy frame 1");

        while tc.udma_busy(Bank::Rx) {}

        // initiate a second, poorly timed capture
        for _ in 0..7 {
            crate::println!("time passes...");
        }
        unsafe { tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16) }
        while tc.udma_busy(Bank::Rx) {}

        crate::println!("legacy frame 2 done");

        // setup interrupts
        crate::println!("Continuous + interrupt driven");
        // drain the previous data
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);
        tc.csr_mut().wo(REG_CAM_CFG_GLOB, 0x400);

        // this one is continuous so it never has to be re-initiated
        unsafe {
            tc.udma_enqueue(Bank::Rx, &tc.rx_buf_phys::<u16>()[..total_len], CFG_EN | CFG_SIZE_16 | CFG_CONT)
        }
        // dummy read is necessary to drain the Tx FIFO
        let _ = tc.csr().r(utra::udma_camera::REG_CAM_CFG_GLOB);
        // re-enable frame sync for rx start
        let global = tc.csr().ms(CFG_FRAMEDROP_EN, 0)
            | tc.csr().ms(CFG_FORMAT, Format::BypassLe as u32)
            | tc.csr().ms(CFG_FRAMESLICE_EN, 0)
            | tc.csr().ms(CFG_SHIFT, 0)
            | 1 << 30 // new field to activate sof snapping of rx
            | tc.csr().ms(CFG_GLOB_EN, 1);
        tc.csr_mut().wo(utra::udma_camera::REG_CAM_CFG_GLOB, global);

        let mut irq8 = CSR::<u32>::new(utra::irqarray8::HW_IRQARRAY8_BASE as *mut u32);
        // clear any pending interrupts
        irq8.wo(utra::irqarray8::EV_PENDING, 0xFFFF);

        for i in 0..2 {
            while irq8.rf(utra::irqarray8::EV_PENDING_CAM_RX) == 0 {}
            // clear pending
            irq8.wfo(utra::irqarray8::EV_PENDING_CAM_RX, 1);
            crate::println!("IRQ frame {} done", i + 1);
        }

        // revert to PIO ownership of I/O pins
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));
        iox.set_ports_from_pio_bitmask(0xFFFF_FFFF);
        println!("piosel {:x}", iox.csr.r(utra::iox::SFR_PIOSEL));

        // always "passes"
        self.passing_tests += 1;
    }
}
