use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::{Button, ButtonListener, ButtonState};
use neutrino::{App, Window};

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
        self.counter.borrow_mut().increment();
    }

    fn on_update(&self, state: &mut ButtonState) {
        let value = self.counter.borrow().value();
        state.set_text(&value.to_string());
        if value % 2 == 0 {
            state.set_style("background-color: red; color: white");
        } else {
            state.set_style("");
        }
    }
}

fn main() {
    let counter = Rc::new(RefCell::new(Counter::new()));

    let listener = MyButtonListener::new(Rc::clone(&counter));

    let mut button = Button::new("my_button");
    button.set_text("0");
    button.set_listener(Box::new(listener));

    let style = r#"
        #my_button {
            background: lime;
        }
    "#;

    let mut window = Window::new();
    window.set_title("Add data");
    window.set_size(320, 240);
    window.set_child(Box::new(button));
    window.set_style(style);
    window.set_debug();

    App::run(window);
}
