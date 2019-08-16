use super::models::Person;
use neutrino::utils::observer::Observer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Custom1Observer {
    person: Rc<RefCell<Person>>,
}

impl Custom1Observer {
    pub fn new(person: Rc<RefCell<Person>>) -> Self {
        Custom1Observer { person: person }
    }
}

impl Observer for Custom1Observer {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("firstname".to_string(), self.person.borrow().firstname());
        fields.insert("lastname".to_string(), self.person.borrow().lastname());
        fields
    }
}
