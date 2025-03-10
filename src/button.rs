use stm32_hal2::gpio::Pin;

use crate::channel::Sender;
use crate::time::TimerEvent;

pub enum ButtonState {
    WaitForPressed,
    Debounce(TimerEvent),
}

pub struct ButtonTask<'a> {
    pin: Pin,
    state: ButtonState,
    sender: Sender<'a, ButtonState>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(pin: Pin, sender: Sender<'a, ButtonState>) -> Self {
        Self {
            pin,
            state: ButtonState::WaitForPressed,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            ButtonState::WaitForPressed => {
                if self.pin.is_high() {
                    self.sender.send(ButtonState::WaitForPressed);
                    self.state = ButtonState::Debounce(TimerEvent::new(200));
                }
            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_low() {
                    self.state = ButtonState::WaitForPressed;
                }
            }
        }
    }
}
