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
            Key::Num1 => self.panes.borrow_mut().set_value(0),
            Key::Num2 => self.panes.borrow_mut().set_value(1),
            Key::Q => std::process::exit(0),
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

pub struct MenuBarListener {
    panes: Rc<RefCell<Panes>>,
}

impl MenuBarListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        MenuBarListener {
            panes: panes,
        }
    }
}

impl Listener for MenuBarListener {
    fn on_change(&self, value: &str) {
        let values = value.split(";").map(|v| {
            v.parse::<u8>().unwrap()
        }).collect::<Vec<u8>>();
        if values[0] == 0 {
            std::process::exit(0);
        } else if values[0] == 1 {
            self.panes.borrow_mut().set_value(values[1]);
        }
    }
}