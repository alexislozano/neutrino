use neutrino::{App, Window};
use neutrino::button::Button;
use neutrino::container::Container;
use neutrino::label::Label;
use std::rc::Rc;
use std::cell::RefCell;
use neutrino::utils::{Listener, Observable};

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn value(&mut self, value: i32) {
        self.value = value;
    }
}

struct Button1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button1Listener {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Button1Listener { counter: counter }
    }
}

impl Listener for Button1Listener {
    fn on_click(&self) {
        let new_value = self.counter.borrow().value - 1;
        self.counter.borrow_mut().value(new_value);
    }
}

struct Button2Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Button2Listener {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Button2Listener { counter: counter }
    }
}

impl Listener for Button2Listener {
    fn on_click(&self) {
        let new_value = self.counter.borrow().value + 1;
        self.counter.borrow_mut().value(new_value);
    }
}

struct Label1Listener {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Listener {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Listener { counter: counter }
    }
}

impl Listener for Label1Listener {
    fn on_click(&self) {
        self.counter.borrow_mut().value(0);
    }
}

struct Label1Observable {
    counter: Rc<RefCell<Counter>>,
}

impl Label1Observable {
    fn new(counter: Rc<RefCell<Counter>>) -> Self {
        Label1Observable { counter: counter }
    }
}

impl Observable for Label1Observable {
    fn value(&self) -> String {
        format!("{}", self.counter.borrow().value)
    }
}

fn main() {
    let counter = Counter::new();
    let rcounter = Rc::new(RefCell::new(counter));

    let button1listener = Button1Listener::new(Rc::clone(&rcounter));

    let button1 = Button::new("button1")
        .text("Decrement")
        .listener(Box::new(button1listener));
        
    let button2listener = Button2Listener::new(Rc::clone(&rcounter));

    let button2 = Button::new("button2")
        .text("Increment")
        .disabled(false)
        .listener(Box::new(button2listener));

    let label1listener = Label1Listener::new(Rc::clone(&rcounter));
    let label1observable = Label1Observable::new(Rc::clone(&rcounter));

    let label1 = Label::new("label1")
        .text("Wow")
        .listener(Box::new(label1listener))
        .observable(Box::new(label1observable));

    let mut container = Container::vertical();

    container.add(Box::new(button1));
    container.add(Box::new(label1));
    container.add(Box::new(button2));

    let mut window = Window::new();
    window.add(Box::new(container));

    let app = App::new()
        .title("Counter")
        .size(320, 240)
        .resizable(true);

    app.run(window);
}