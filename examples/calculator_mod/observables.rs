use neutrino::utils::observable::Observable;
use std::cell::RefCell;
use std::rc::Rc;
use super::models::Calculator;
use std::collections::HashMap;

pub struct Result1Observable {
    calculator: Rc<RefCell<Calculator>>,
}

impl Result1Observable {
    pub fn new(calculator: Rc<RefCell<Calculator>>) -> Self {
        Result1Observable { calculator: calculator }
    }
}

impl Observable<String> for Result1Observable {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("value".to_string(), format!("{}", self.calculator.borrow().result()));
        fields
    }
}

pub struct Input1Observable {
    calculator: Rc<RefCell<Calculator>>,
}

impl Input1Observable {
    pub fn new(calculator: Rc<RefCell<Calculator>>) -> Self {
        Input1Observable { calculator: calculator }
    }
}

impl Observable<String> for Input1Observable {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("value".to_string(), format!("{}", self.calculator.borrow().value1()));
        fields
    }
}

pub struct Input2Observable {
    calculator: Rc<RefCell<Calculator>>,
}

impl Input2Observable {
    pub fn new(calculator: Rc<RefCell<Calculator>>) -> Self {
        Input2Observable { calculator: calculator }
    }
}

impl Observable<String> for Input2Observable {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("value".to_string(), format!("{}", self.calculator.borrow().value2()));
        fields
    }
}