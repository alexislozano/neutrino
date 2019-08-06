use neutrino::utils::observable::Observable;
use std::cell::RefCell;
use std::rc::Rc;
use super::models::Person;
use std::collections::HashMap;

pub struct Custom1Observable {
    person: Rc<RefCell<Person>>,
}

impl Custom1Observable {
    pub fn new(person: Rc<RefCell<Person>>) -> Self {
        Custom1Observable { person: person }
    }
}

impl Observable<String> for Custom1Observable {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert("firstname".to_string(), self.person.borrow().firstname());
        fields.insert("lastname".to_string(), self.person.borrow().lastname());
        fields
    }
}