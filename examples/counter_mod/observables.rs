use super::models::Counter;
use neutrino::utils::observable::Observable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Label1Observable {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Observable {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Observable { counter: counter }
    }
}

impl Observable<String> for Label1Observable {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert(
            "text".to_string(),
            format!("{}", self.counter.borrow().value()),
        );
        fields
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
    fn observe(&self) -> HashMap<String, u8> {
        let mut fields = HashMap::new();
        let value = self.counter.borrow().value();
        if value < 0 {
            fields.insert("value".to_string(), 0);
        } else if value > 100 {
            fields.insert("value".to_string(), 100);
        } else {
            fields.insert("value".to_string(), value as u8);
        }
        fields
    }
}
