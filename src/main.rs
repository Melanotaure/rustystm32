#![no_std]
#![no_main]

use stm32_hal2 as hal;

use cortex_m::asm::nop;
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
    let p = hal::pac::Peripherals::take().unwrap();
    p.RCC.ahb1enr.write(|w| w.gpioden().enabled());
    let mut pd15 = hal::gpio::Pin::new(hal::gpio::Port::D, 15, hal::gpio::PinMode::Output);
    let mut pd14 = hal::gpio::Pin::new(hal::gpio::Port::D, 14, hal::gpio::PinMode::Output);
    let mut pd13 = hal::gpio::Pin::new(hal::gpio::Port::D, 13, hal::gpio::PinMode::Output);
    let mut pd12 = hal::gpio::Pin::new(hal::gpio::Port::D, 12, hal::gpio::PinMode::Output);

    let mut led_state = Led::D15;
    loop {
        match led_state {
            Led::D15 => {
                #[cfg(feature = "rtt")]
                rprintln!("D15");
                pd15.set_high();
                pd14.set_low();
                pd13.set_low();
                pd12.set_low();
                led_state = Led::D14;
            }
            Led::D14 => {
                #[cfg(feature = "rtt")]
                rprintln!("D14");
                pd15.set_low();
                pd14.set_high();
                pd13.set_low();
                pd12.set_low();
                led_state = Led::D13;
            }
            Led::D13 => {
                #[cfg(feature = "rtt")]
                rprintln!("D13");
                pd15.set_low();
                pd14.set_low();
                pd13.set_high();
                pd12.set_low();
                led_state = Led::D12;
            }
            Led::D12 => {
                #[cfg(feature = "rtt")]
                rprintln!("D12");
                pd15.set_low();
                pd14.set_low();
                pd13.set_low();
                pd12.set_high();
                led_state = Led::D15;
            }
        }

        for _ in 0..100_000 {
            nop();
        }
    }
}
