use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction, Position, Alignment};
use neutrino::widgets::label::Label;
use neutrino::widgets::progressbar::ProgressBar;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

mod counter_mod;

use counter_mod::listeners::{Button1Listener, Button2Listener, Label1Listener, 
ProgressBar1Listener};
use counter_mod::models::Counter;

fn main() {
    let mut counter = Counter::new();
    counter.set_value(30);
    let rcounter = Rc::new(RefCell::new(counter));

    let button1listener = Button1Listener::new(Rc::clone(&rcounter));

    let mut button1 = Button::new("button1");
    button1.set_text("Decrement");
    button1.set_listener(Box::new(button1listener));

    let button2listener = Button2Listener::new(Rc::clone(&rcounter));

    let mut button2 = Button::new("button2");
    button2.set_text("Increment");
    button2.set_listener(Box::new(button2listener));

    let label1listener = Label1Listener::new(Rc::clone(&rcounter));

    let mut label1 = Label::new("label1");
    label1.set_listener(Box::new(label1listener));

    let progressbar1listener = ProgressBar1Listener::new(Rc::clone(&rcounter));

    let mut progressbar1 = ProgressBar::new("progressbar1");
    progressbar1.set_listener(Box::new(progressbar1listener));

    let mut container = Container::new("container");
    container.set_direction(Direction::Vertical);
    container.set_position(Position::Center);
    container.set_alignment(Alignment::Center);

    container.add(Box::new(button1));
    container.add(Box::new(label1));
    container.add(Box::new(progressbar1));
    container.add(Box::new(button2));

    let window = Window::new()
        .title("Counter")
        .size(320, 240)
        .resizable(true)
        .child(Box::new(container))
        .theme(Theme::Breeze);

    App::run(window);
}
