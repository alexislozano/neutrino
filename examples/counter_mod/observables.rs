use neutrino::utils::observable::Observable;
use std::cell::RefCell;
use std::rc::Rc;
use super::models::Counter;

pub struct Label1Observable {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Observable {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Observable { counter: counter }
    }
}

impl Observable<String> for Label1Observable {
    fn observe(&self) -> String {
        format!("{}", self.counter.borrow().value())
    }
}

pub struct ProgressBar1Observable {
    counter: Rc<RefCell<Counter>>,
}

impl ProgressBar1Observable {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        ProgressBar1Observable { counter: counter }
    }
}

impl Observable<u8> for ProgressBar1Observable {
    fn observe(&self) -> u8 {
        let value = self.counter.borrow().value();
        if value < 0 {
            0
        } else if value > 100 {
            100
        } else {
            value as u8
        }
    }
}