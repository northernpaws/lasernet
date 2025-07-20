use cortex_m::singleton;

use defmt::info;

use embassy_stm32::{
    bind_interrupts, gpio::{
        self,
        Level,
        Output,
        Speed
    }, interrupt::InterruptExt as _, mode, pac::Interrupt, peripherals, spi, time::Hertz, usart, Config
};

// use assign_resources::assign_resources;

// assign_resources! {
//     uart: SerialResources {
//         uart: USART1,
//         rx: PA3,
//         tx: PA2
//     }
// }

bind_interrupts!(struct Irqs {
    USART2 => usart::BufferedInterruptHandler<peripherals::USART2>;
});

pub(crate) struct Hardware <'a> {
    pub led: gpio::Output<'a>,

    pub lcd_spi: spi::Spi<'a, mode::Blocking>,

    pub lcd_dc: gpio::Output<'a>,
    pub lcd_cs: gpio::Output<'a>,
    pub lcd_rst: gpio::Output<'a>,
        
}

impl Default for Hardware<'_> {
    fn default() -> Self {
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;

            config.rcc.hsi = true;
            config.rcc.pll_src = PllSource::HSI;
            config.rcc.pll = Some(Pll {
                prediv: PllPreDiv::DIV16,
                mul: PllMul::MUL336,
                divp: Some(PllPDiv::DIV4),// 16 / 16 * 336 / 4 = 84Mhz
                divq: None,
                divr: None, 
            });
            config.rcc.sys = Sysclk::PLL1_P;
            config.rcc.ahb_pre = AHBPrescaler::DIV1;
            config.rcc.apb1_pre = APBPrescaler::DIV2;
            config.rcc.apb2_pre = APBPrescaler::DIV1;
        }
        
        // Initialize the STM32 chip using our custom clock config.
        let p = embassy_stm32::init(config);

        // let r = split_resources!(peripherals);

        let led = Output::new(p.PB13, Level::High, Speed::Low);

        let mut spi_config = spi::Config::default();
        spi_config.frequency = Hertz(1_000_000);
        let mut lcd_spi = spi::Spi::new_blocking(p.SPI3, p.PC10, p.PC12, p.PC11, spi_config);

        let lcd_dc = Output::new(p.PB13, Level::High, Speed::Low);
        let lcd_cs = Output::new(p.PB13, Level::High, Speed::Low);
        let lcd_rst = Output::new(p.PB13, Level::High, Speed::Low);
        
        Self {
            led,

            lcd_spi,
            lcd_dc,
            lcd_cs,
            lcd_rst
        }
    }
}