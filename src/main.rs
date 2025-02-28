#![no_std]
#![no_main]

use stm32_hal2 as hal;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    #[cfg(feature = "rtt")]
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    p.RCC.ahb1enr.write(|w| w.gpioden().enabled());
    let mut pd15 = hal::gpio::Pin::new(hal::gpio::Port::D, 15, hal::gpio::PinMode::Output);

    let mut blink = true;
    loop {
        if blink {
            // Set pin 15 high
            pd15.set_high();
            #[cfg(feature = "rtt")]
            rprintln!("LED ON");
        } else {
            // Set pin 15 low
            pd15.set_low();
            #[cfg(feature = "rtt")]
            rprintln!("LED OFF");
        }
        for _ in 0..100_000 {
            nop();
        }
        blink = !blink;
    }
}
