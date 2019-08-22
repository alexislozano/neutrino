use super::models::Person;
use neutrino::utils::listener::Listener;
use std::cell::RefCell;
use std::rc::Rc;
use neutrino::utils::event::Key;

pub struct ButtonListener {
    person: Rc<RefCell<Person>>,
    firstname: String,
    lastname: String,
}

impl ButtonListener {
    pub fn new(person: Rc<RefCell<Person>>, firstname: &str, lastname: &str) -> Self {
        ButtonListener {
            person: person,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
        }
    }
}

impl Listener for ButtonListener {
    fn on_change(&self, _value: &str) {
        self.person.borrow_mut().set_firstname(&self.firstname);
        self.person.borrow_mut().set_lastname(&self.lastname);
    }

    fn on_key(&self, _key: Key) {}
}
