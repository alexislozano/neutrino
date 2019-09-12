use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::{Button, ButtonListener, ButtonState};
use neutrino::utils::event::Key;
use neutrino::{App, Window, WindowListener};

struct Counter {
    value: u8,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn value(&self) -> u8 {
        self.value
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn reset(&mut self) {
        self.value = 0;
    }
}

struct MyButtonListener {
    counter: Rc<RefCell<Counter>>,
}

impl MyButtonListener {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter }
    }
}

impl ButtonListener for MyButtonListener {
    fn on_change(&self, _state: &ButtonState) {
        self.counter.borrow_mut().reset();
    }

    fn on_update(&self, state: &mut ButtonState) {
        state.set_text(&format!(
            "{} seconds since last reset",
            self.counter.borrow().value().to_string()
        ));
    }
}

struct MyWindowListener {
    counter: Rc<RefCell<Counter>>,
}

impl MyWindowListener {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Self { counter }
    }
}

impl WindowListener for MyWindowListener {
    fn on_key(&self, _key: Key) {}

    fn on_tick(&self) {
        self.counter.borrow_mut().increment();
    }
}

fn main() {
    let counter = Rc::new(RefCell::new(Counter::new()));

    let listener = MyButtonListener::new(Rc::clone(&counter));

    let mut button = Button::new("my_button");
    button.set_text("0 seconds since last reset");
    button.set_listener(Box::new(listener));

    let wlistener = MyWindowListener::new(Rc::clone(&counter));

    let mut window = Window::new();
    window.set_title("Timer");
    window.set_size(320, 240);
    window.set_child(Box::new(button));
    window.set_timer(1000);
    window.set_listener(Box::new(wlistener));

    App::run(window);
}
