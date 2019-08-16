use super::models::Counter;
use neutrino::utils::observer::Observer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Label1Observer {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Observer {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Observer { counter: counter }
    }
}

impl Observer for Label1Observer {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert(
            "text".to_string(),
            format!("{}", self.counter.borrow().value()),
        );
        fields
    }
}

pub struct ProgressBar1Observer {
    counter: Rc<RefCell<Counter>>,
}

impl ProgressBar1Observer {
    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
        ProgressBar1Observer { counter: counter }
    }
}

impl Observer for ProgressBar1Observer {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        let value = self.counter.borrow().value();
        if value < 0 {
            fields.insert("value".to_string(), 0.to_string());
        } else if value > 100 {
            fields.insert("value".to_string(), 100.to_string());
        } else {
            fields.insert("value".to_string(), value.to_string());
        }
        fields
    }
}
