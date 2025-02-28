#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    const GPIOD_BASE: u32 = 0x4002_0C00;
    const RCC_BASE: u32 = 0x4002_3800;
    const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;
    const GPIOD_MODER: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;
    const GPIOD_ODR: *mut u32 = (GPIOD_BASE + 0x14) as *mut u32;
    const PIN: u32 = 15; // Blue LED
    unsafe {
        // Enable the clock for GPIOD
        *RCC_AHB1ENR |= 1 << 3;
        // Set pin 12 as output
        *GPIOD_MODER |= 1 << (PIN * 2);
    }

    let mut blink = true;
    loop {
        if blink {
            unsafe {
                // Set pin 12 high
                write_volatile(GPIOD_ODR, *GPIOD_ODR | (1 << PIN));
            }
        } else {
            unsafe {
                // Set pin 12 low
                write_volatile(GPIOD_ODR, *GPIOD_ODR & !(1 << PIN));
            }
        }
        for _ in 0..100_000 {
            nop();
        }
        blink = !blink;
    }
}
