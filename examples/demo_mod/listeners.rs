use neutrino::utils::listener::Listener;
use neutrino::utils::event::Key;
use super::models::Panes;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppListener {
    panes: Rc<RefCell<Panes>>,
}

impl AppListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        AppListener {
            panes: panes,
        }
    }
}

impl Listener for AppListener {
    fn on_key(&self, key: Key) {
        match key {
            Key::Digit1 => self.panes.borrow_mut().set_value(0),
            Key::Digit2 => self.panes.borrow_mut().set_value(1),
            _ => (),
        }
    }
}

pub struct TabsListener {
    panes: Rc<RefCell<Panes>>,
}

impl TabsListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        TabsListener {
            panes: panes,
        }
    }
}

impl Listener for TabsListener {
    fn on_change(&self, value: &str) {
        self.panes.borrow_mut().set_value(value.parse::<u8>().unwrap());
    }
}