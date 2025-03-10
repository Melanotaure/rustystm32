use rtt_target::rprintln;
use stm32_hal2::gpio::{Pin, PinMode, Port};

use crate::button::ButtonState;
use crate::channel::Receiver;
use crate::time::TimerEvent;

enum Led {
    D15,
    D14,
    D13,
    D12,
}

enum LedState {
    Toggle,
    Wait(TimerEvent),
}

pub struct LedTask<'a> {
    blue_led: Pin,
    red_led: Pin,
    orange_led: Pin,
    green_led: Pin,
    active_led: Led,
    state: LedState,
    receiver: Receiver<'a, ButtonState>,
}

impl<'a> LedTask<'a> {
    pub fn new(receiver: Receiver<'a, ButtonState>) -> Self {
        Self {
            blue_led: Pin::new(Port::D, 15, PinMode::Output),
            red_led: Pin::new(Port::D, 14, PinMode::Output),
            orange_led: Pin::new(Port::D, 13, PinMode::Output),
            green_led: Pin::new(Port::D, 12, PinMode::Output),
            active_led: Led::D15,
            state: LedState::Toggle,
            receiver,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            LedState::Toggle => {
                match self.active_led {
                    Led::D15 => {
                        rprintln!("Blinking Blue LED");
                        self.blue_led.toggle();
                    }
                    Led::D14 => {
                        rprintln!("Blinking Red LED");
                        self.red_led.toggle();
                    }
                    Led::D13 => {
                        rprintln!("Blinking Orange LED");
                        self.orange_led.toggle();
                    }
                    Led::D12 => {
                        rprintln!("Blinking Green LED");
                        self.green_led.toggle();
                    }
                }
                self.state = LedState::Wait(TimerEvent::new(500));
            }
            LedState::Wait(ref timer) => {
                if timer.is_ready() {
                    rprintln!("t: {}", timer.now());
                    self.state = LedState::Toggle;
                }

                if let Some(button_state) = self.receiver.receive() {
                    match button_state {
                        ButtonState::WaitForPressed => {
                            rprintln!("Button pressed...");
                            match self.active_led {
                                Led::D15 => {
                                    self.blue_led.set_low();
                                    self.active_led = Led::D14;
                                }
                                Led::D14 => {
                                    self.red_led.set_low();
                                    self.active_led = Led::D13;
                                }
                                Led::D13 => {
                                    self.orange_led.set_low();
                                    self.active_led = Led::D12;
                                }
                                Led::D12 => {
                                    self.green_led.set_low();
                                    self.active_led = Led::D15;
                                }
                            }
                        }
                        ButtonState::Debounce(_) => {}
                    }
                }
            }
        }
    }
}
