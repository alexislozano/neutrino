use std::cell::RefCell;
use std::rc::Rc;

use neutrino::utils::event::Key;
use neutrino::widgets::menubar::{MenuBarListener, MenuBarState};
use neutrino::widgets::tabs::{TabsListener, TabsState};
use neutrino::WindowListener;

use super::models::Panes;

pub struct MyWindowListener {
    panes: Rc<RefCell<Panes>>,
}

impl MyWindowListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        Self { panes }
    }
}

impl WindowListener for MyWindowListener {
    fn on_key(&self, key: Key) {
        match key {
            Key::Num1 => self.panes.borrow_mut().set_value(0),
            Key::Num2 => self.panes.borrow_mut().set_value(1),
            Key::Q => std::process::exit(0),
            _ => (),
        }
    }
}

pub struct MyTabsListener {
    panes: Rc<RefCell<Panes>>,
}

impl MyTabsListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        Self { panes }
    }
}

impl TabsListener for MyTabsListener {
    fn on_update(&self, state: &mut TabsState) {
        state.set_selected(u32::from(self.panes.borrow().value()));
    }

    fn on_change(&self, state: &TabsState) {
        self.panes.borrow_mut().set_value(state.selected() as u8);
    }
}

pub struct MyMenuBarListener {
    panes: Rc<RefCell<Panes>>,
}

impl MyMenuBarListener {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        Self { panes }
    }
}

impl MenuBarListener for MyMenuBarListener {
    fn on_change(&self, state: &MenuBarState) {
        match state.selected_item() {
            None => (),
            Some(selected_item) => {
                if selected_item == 0 {
                    std::process::exit(0);
                } else if selected_item == 1 {
                    match state.selected_function() {
                        None => (),
                        Some(selected_function) => {
                            self.panes
                                .borrow_mut()
                                .set_value(selected_function as u8);
                        }
                    }
                }
            }
        }
    }
}
