use neutrino::utils::listener::Listener;
use std::cell::RefCell;
use std::rc::Rc;
use super::models::Calculator;

pub struct Input1Listener {
    calculator: Rc<RefCell<Calculator>>,
}

impl Input1Listener {
    pub fn new(calculator: Rc<RefCell<Calculator>>) -> Self {
        Input1Listener { calculator: calculator }
    }
}

impl Listener for Input1Listener {
    fn on_change(&self, value: &str) {
        let old_value = self.calculator.borrow().value1();
        self.calculator.borrow_mut().set_value1(match value.parse::<i32>() {
            Ok(number) => number,
            Err(_) => old_value,
        });
    }
}

pub struct Input2Listener {
    calculator: Rc<RefCell<Calculator>>,
}

impl Input2Listener {
    pub fn new(calculator: Rc<RefCell<Calculator>>) -> Self {
        Input2Listener { calculator: calculator }
    }
}

impl Listener for Input2Listener {
    fn on_change(&self, value: &str) {
        let old_value = self.calculator.borrow().value2();
        self.calculator.borrow_mut().set_value2(match value.parse::<i32>() {
            Ok(number) => number,
            Err(_) => old_value,
        });
    }
}