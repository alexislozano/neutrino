use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::{ButtonListener, ButtonState};
use neutrino::widgets::custom::{CustomListener, CustomState};

use super::models::Person;


pub struct MyButtonListener {
    person: Rc<RefCell<Person>>,
    firstname: String,
    lastname: String,
}

impl MyButtonListener {
    pub fn new(person: Rc<RefCell<Person>>, firstname: &str, lastname: &str) -> Self {
        Self {
            person: person,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
        }
    }
}

impl ButtonListener for MyButtonListener {
    fn on_change(&self, _state: &ButtonState) {
        self.person.borrow_mut().set_firstname(&self.firstname);
        self.person.borrow_mut().set_lastname(&self.lastname);
    }

    fn on_update(&self, _state: &mut ButtonState) {}
}


pub struct MyCustomListener {
    person: Rc<RefCell<Person>>,
}

impl MyCustomListener {
    pub fn new(person: Rc<RefCell<Person>>) -> Self {
        Self {
            person: person,
        }
    }
}

impl CustomListener for MyCustomListener {
    fn on_update(&self, state: &mut CustomState) {
        state.set_template(&format!(
            r#"<h1 style="margin: 0;">My name is {} {}.</h1>"#,
            self.person.borrow().firstname(),
            self.person.borrow().lastname(),
        ));
    }    
}