use std::rc::Rc;
use std::cell::RefCell;

use neutrino::{App, Window};
use neutrino::widgets::button::Button;
use neutrino::widgets::container::Container;
use neutrino::widgets::label::Label;
use neutrino::widgets::progressbar::ProgressBar;

mod counter_mod;

use counter_mod::models::Counter;
use counter_mod::listeners::{Button1Listener, Button2Listener, Label1Listener};
use counter_mod::observables::{Label1Observable, ProgressBar1Observable};

fn main() {
    let mut counter = Counter::new();
    counter.set_value(30);
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
        .listener(Box::new(label1listener))
        .observable(Box::new(label1observable));

    let progressbar1observable = ProgressBar1Observable::new(Rc::clone(&rcounter));

    let progressbar1 = ProgressBar::new("progressbar1")
        .observable(Box::new(progressbar1observable));

    let mut container = Container::vertical();

    container.add(Box::new(button1));
    container.add(Box::new(label1));
    container.add(Box::new(progressbar1));
    container.add(Box::new(button2));

    let mut window = Window::new();
    window.add(Box::new(container));

    let app = App::new()
        .title("Counter")
        .size(320, 240)
        .resizable(true);

    app.run(window);
}