#![no_std]
#![no_main]

use stm32f4::stm32f407;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    #[cfg(feature = "rtt")]
    rtt_init_print!();
    let p = stm32f407::Peripherals::take().unwrap();
    p.RCC.ahb1enr.write(|w| w.gpioden().enabled());
    p.GPIOD.moder.write(|w| w.moder15().output());

    let mut blink = true;
    loop {
        if blink {
            // Set pin 15 high
            p.GPIOD.odr.write(|w| w.odr15().set_bit());
            #[cfg(feature = "rtt")]
            rprintln!("LED ON");
        } else {
            // Set pin 15 low
            p.GPIOD.odr.write(|w| w.odr15().clear_bit());
            #[cfg(feature = "rtt")]
            rprintln!("LED OFF");
        }
        for _ in 0..100_000 {
            nop();
        }
        blink = !blink;
    }
}
