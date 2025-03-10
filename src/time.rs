use core::cell::RefCell;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::interrupt::{free, Mutex};
use stm32_hal2::clocks::Clocks;
use stm32_hal2::pac::{interrupt, TIM2};
use stm32_hal2::timer::{Timer, TimerConfig, TimerInterrupt};

static TICKER: Ticker = Ticker {
    ovf_count: AtomicU32::new(0),
    tim: Mutex::new(RefCell::new(None)),
};

pub struct Ticker {
    ovf_count: AtomicU32,
    tim: Mutex<RefCell<Option<Timer<TIM2>>>>,
}

pub struct TimerEvent {
    end_time: u32, // in milliseconds
}

impl TimerEvent {
    pub fn new(duration: u32) -> Self {
        Self {
            end_time: Ticker::now() + duration,
        }
    }

    pub fn is_ready(&self) -> bool {
        Ticker::now() >= self.end_time
    }

    pub fn now(&self) -> u32 {
        Ticker::now()
    }
}

impl Ticker {
    pub fn init(reg: TIM2, clock: &Clocks) {
        let mut tim = Timer::new_tim2(reg, 0.1, TimerConfig::default(), clock);
        tim.enable();
        tim.enable_interrupt(TimerInterrupt::Update);
        unsafe {
            stm32_hal2::pac::NVIC::unmask(stm32_hal2::pac::interrupt::TIM2);
        }
        free(|cs| {
            TICKER.tim.borrow(cs).replace(Some(tim));
        });
    }

    pub fn now() -> u32 {
        let mut counter = free(|cs| {
            if let Some(ref mut tim) = TICKER.tim.borrow(cs).borrow_mut().deref_mut() {
                tim.now().as_millis()
            } else {
                0
            }
        });
        let ovf = TICKER.ovf_count.load(Ordering::SeqCst);
        counter += ovf * 10_000;
        counter
    }
}

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim) = TICKER.tim.borrow(cs).borrow_mut().deref_mut() {
            tim.clear_interrupt(TimerInterrupt::Update);
            TICKER.ovf_count.fetch_add(1, Ordering::Relaxed);
        }
    });
}
