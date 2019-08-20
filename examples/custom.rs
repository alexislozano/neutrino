use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::Button;
use neutrino::widgets::container::Container;
use neutrino::widgets::custom::Custom;
use neutrino::{App, Window};

mod custom_mod;

use custom_mod::listeners::ButtonListener;
use custom_mod::models::Person;
use custom_mod::observers::Custom1Observer;

fn main() {
    let mut person = Person::new();
    person.set_firstname("Ada");
    person.set_lastname("Lovelace");
    let rperson = Rc::new(RefCell::new(person));

    let custom1observer = Custom1Observer::new(Rc::clone(&rperson));

    let custom1 = Custom::new("custom1")
        .template("<h1>My name is {firstname} {lastname}.</h1>")
        .observer(Box::new(custom1observer));

    let button1listener = ButtonListener::new(Rc::clone(&rperson), "Ada", "Lovelace");

    let button1 = Button::new("button1")
        .text("Ada Lovelace")
        .listener(Box::new(button1listener));

    let button2listener = ButtonListener::new(Rc::clone(&rperson), "Harry", "Potter");

    let button2 = Button::new("button2")
        .text("Harry Potter")
        .listener(Box::new(button2listener));

    let button3listener = ButtonListener::new(Rc::clone(&rperson), "Sigmund", "Freud");

    let button3 = Button::new("button3")
        .text("Sigmund Freud")
        .listener(Box::new(button3listener));

    let mut container1 = Container::horizontal();
    container1.add(Box::new(button1));
    container1.add(Box::new(button2));
    container1.add(Box::new(button3));

    let mut container2 = Container::vertical();
    container2.add(Box::new(custom1));
    container2.add(Box::new(container1));

    let window = Window::new()
        .title("Custom")
        .size(340, 240)
        .resizable(true)
        .child(Box::new(container2));

    App::run(window);
}
