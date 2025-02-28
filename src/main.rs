#![no_std]
#![no_main]

use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
};

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

enum Led {
    D15,
    D14,
    D13,
    D12,
}

#[entry]
fn main() -> ! {
    #[cfg(feature = "rtt")]
    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    let mut pd15 = Pin::new(Port::D, 15, PinMode::Output);
    let mut pd14 = Pin::new(Port::D, 14, PinMode::Output);
    let mut pd13 = Pin::new(Port::D, 13, PinMode::Output);
    let mut pd12 = Pin::new(Port::D, 12, PinMode::Output);

    let mut led_state = Led::D15;
    loop {
        match led_state {
            Led::D15 => {
                #[cfg(feature = "rtt")]
                rprintln!("Blue");
                pd15.set_high();
                pd14.set_low();
                pd13.set_low();
                pd12.set_low();
                led_state = Led::D14;
            }
            Led::D14 => {
                #[cfg(feature = "rtt")]
                rprintln!("Red");
                pd15.set_low();
                pd14.set_high();
                pd13.set_low();
                pd12.set_low();
                led_state = Led::D13;
            }
            Led::D13 => {
                #[cfg(feature = "rtt")]
                rprintln!("Orange");
                pd15.set_low();
                pd14.set_low();
                pd13.set_high();
                pd12.set_low();
                led_state = Led::D12;
            }
            Led::D12 => {
                #[cfg(feature = "rtt")]
                rprintln!("Green");
                pd15.set_low();
                pd14.set_low();
                pd13.set_low();
                pd12.set_high();
                led_state = Led::D15;
            }
        }

        delay.delay_ms(1000u32);
    }
}
