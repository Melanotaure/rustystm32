#![no_std]
#![no_main]

mod button;
mod channel;
mod led;
mod time;

use button::ButtonState;
use channel::Channel;
use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    pac,
};

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init_print;
use time::Ticker;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();
    Ticker::init(dp.TIM2, &clock_cfg);

    let channel: Channel<ButtonState> = Channel::new();
    let mut led_task = led::LedTask::new(channel.get_receiver());
    let button = Pin::new(Port::A, 0, PinMode::Input);
    let mut button_task = button::ButtonTask::new(button, channel.get_sender());

    loop {
        led_task.poll();
        button_task.poll();
    }
}
