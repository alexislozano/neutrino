use std::cell::RefCell;
use std::rc::Rc;

use neutrino::utils::event::Key;
use neutrino::widgets::menubar::{MenuBarListener, MenuBarState};
use neutrino::widgets::tabs::{TabsListener, TabsState};
use neutrino::widgets::range::{RangeListener, RangeState};
use neutrino::widgets::progressbar::{ProgressBarListener, ProgressBarState};
use neutrino::widgets::label::{LabelListener, LabelState};
use neutrino::widgets::textinput::{TextInputListener, TextInputState};
use neutrino::WindowListener;

use super::models::{Panes, RangeValue};


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
            Key::Num3 => self.panes.borrow_mut().set_value(2),
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


pub struct MyRangeListener {
    range: Rc<RefCell<RangeValue>>,
}

impl MyRangeListener {
    pub fn new(range: Rc<RefCell<RangeValue>>) -> Self {
        Self { range }
    }
}

impl RangeListener for MyRangeListener {
    fn on_update(&self, state: &mut RangeState) {
        state.set_value(self.range.borrow().value());
    }
    fn on_change(&self, state: &RangeState) {
        self.range.borrow_mut().set_value(state.value());
    }
}

pub struct MyProgressBarListener {
    range: Rc<RefCell<RangeValue>>,
}

impl MyProgressBarListener {
    pub fn new(range: Rc<RefCell<RangeValue>>) -> Self {
        Self { range }
    }
}

impl ProgressBarListener for MyProgressBarListener {
    fn on_update(&self, state: &mut ProgressBarState) {
        state.set_value(self.range.borrow().value());
    }
}

pub struct MyLabelListener {
    range: Rc<RefCell<RangeValue>>,
}

impl MyLabelListener {
    pub fn new(range: Rc<RefCell<RangeValue>>) -> Self {
        Self { range }
    }
}

impl LabelListener for MyLabelListener {
    fn on_update(&self, state: &mut LabelState) {
        let text = format!("{}%", self.range.borrow().value());
        state.set_text(&text);
    }
}


pub struct MyTextInputListener {
    range: Rc<RefCell<RangeValue>>,
}

impl MyTextInputListener {
    pub fn new(range: Rc<RefCell<RangeValue>>) -> Self {
        Self { range }
    }
}

impl TextInputListener for MyTextInputListener {
    fn on_update(&self, state: &mut TextInputState) {
        state.set_value(&self.range.borrow().value().to_string());
    }
    fn on_change(&self, state: &TextInputState) {
        self.range.borrow_mut().set_value(state.value().parse().unwrap());
    }
}
