use super::models::Counter;
use neutrino::utils::listener::Listener;
use std::cell::RefCell;
use std::rc::Rc;
use neutrino::utils::event::Key;

pub struct Button1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button1Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Button1Listener { counter: counter }
    }
}

impl Listener for Button1Listener {
    fn on_change(&self, _value: &str) {
        let new_value = self.counter.borrow().value() - 1;
        self.counter.borrow_mut().set_value(new_value);
    }

    fn on_key(&self, _key: Key) {}
}

pub struct Button2Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button2Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Button2Listener { counter: counter }
    }
}

impl Listener for Button2Listener {
    fn on_change(&self, _value: &str) {
        let new_value = self.counter.borrow().value() + 1;
        self.counter.borrow_mut().set_value(new_value);
    }

    fn on_key(&self, _key: Key) {}
}

pub struct Label1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Listener {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Listener { counter: counter }
    }
}

impl Listener for Label1Listener {
    fn on_change(&self, _value: &str) {
        self.counter.borrow_mut().set_value(0);
    }

    fn on_key(&self, _key: Key) {}
}
