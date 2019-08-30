use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::{ButtonListener, ButtonState};
use neutrino::widgets::label::{LabelListener, LabelState};
use neutrino::widgets::progressbar::{ProgressBarListener, ProgressBarState};

use super::models::Counter;


pub struct Button1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button1Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter: counter }
    }
}

impl ButtonListener for Button1Listener {
    fn on_change(&self, _state: &ButtonState) {
        let new_value = self.counter.borrow().value() - 1;
        self.counter.borrow_mut().set_value(new_value);
    }

    fn on_update(&self, _state: &mut ButtonState) {}
}

pub struct Button2Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button2Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter: counter }
    }
}

impl ButtonListener for Button2Listener {
    fn on_change(&self, _state: &ButtonState) {
        let new_value = self.counter.borrow().value() + 1;
        self.counter.borrow_mut().set_value(new_value);
    }

    fn on_update(&self, _state: &mut ButtonState) {}
}

pub struct Label1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter: counter }
    }
}

impl LabelListener for Label1Listener {
    fn on_update(&self, state: &mut LabelState) {
        state.set_text(&self.counter.borrow().value().to_string());
    }
}

pub struct ProgressBar1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl ProgressBar1Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter: counter }
    }
}

impl ProgressBarListener for ProgressBar1Listener {
    fn on_update(&self, state: &mut ProgressBarState) {
        let value = self.counter.borrow().value();
        let new_value = if value < 0 {
            0
        } else if value > 100 {
            100
        } else {
            value as u8
        };
        state.set_value(new_value);        
    }
}