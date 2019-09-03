use std::cell::RefCell;
use std::rc::Rc;

use neutrino::utils::pixmap::Pixmap;
use neutrino::widgets::image::{ImageListener, ImageState};
use neutrino::widgets::button::{ButtonState, ButtonListener};

use super::models::Images;

pub struct MyImageListener {
    images: Rc<RefCell<Images>>,
}

impl MyImageListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ImageListener for MyImageListener {
    fn on_update(&self, state: &mut ImageState) {
        let pixmap = Pixmap::from_path(self.images.borrow().selected_path());
        state.set_data(pixmap.data());
        state.set_extension(pixmap.extension());
    }
}

pub struct MyPrevButtonListener {
    images: Rc<RefCell<Images>>,
}

impl MyPrevButtonListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ButtonListener for MyPrevButtonListener {
    fn on_update(&self, _state: &mut ButtonState) {}

    fn on_change(&self, _state: &ButtonState) {
        self.images.borrow_mut().previous();
    }
}

pub struct MyNextButtonListener {
    images: Rc<RefCell<Images>>,
}

impl MyNextButtonListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ButtonListener for MyNextButtonListener {
    fn on_update(&self, _state: &mut ButtonState) {}

    fn on_change(&self, _state: &ButtonState) {
        self.images.borrow_mut().next();
    }
}