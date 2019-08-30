use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction, Position, Alignment};
use neutrino::widgets::custom::Custom;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

mod custom_mod;

use custom_mod::listeners::{MyButtonListener, MyCustomListener};
use custom_mod::models::Person;

fn main() {
    let mut person = Person::new();
    person.set_firstname("Ada");
    person.set_lastname("Lovelace");
    let rperson = Rc::new(RefCell::new(person));

    let mycustomlistener = MyCustomListener::new(Rc::clone(&rperson));

    let custom1 = Custom::new("custom1")
        .listener(Box::new(mycustomlistener));

    let mybutton1listener = MyButtonListener::new(Rc::clone(&rperson), "Ada", "Lovelace");

    let button1 = Button::new("button1")
        .text("Ada Lovelace")
        .listener(Box::new(mybutton1listener));

    let mybutton2listener = MyButtonListener::new(Rc::clone(&rperson), "Harry", "Potter");

    let button2 = Button::new("button2")
        .text("Harry Potter")
        .listener(Box::new(mybutton2listener));

    let mybutton3listener = MyButtonListener::new(Rc::clone(&rperson), "Sigmund", "Freud");

    let button3 = Button::new("button3")
        .text("Sigmund Freud")
        .listener(Box::new(mybutton3listener));

    let mut container1 = Container::new("container1")
        .direction(Direction::Horizontal)
        .position(Position::Center);
    container1.add(Box::new(button1));
    container1.add(Box::new(button2));
    container1.add(Box::new(button3));

    let mut container2 = Container::new("container2")
        .direction(Direction::Vertical)
        .position(Position::Center)
        .alignment(Alignment::Center);
    container2.add(Box::new(custom1));
    container2.add(Box::new(container1));

    let window = Window::new()
        .title("Custom")
        .size(400, 200)
        .resizable(true)
        .child(Box::new(container2))
        .theme(Theme::Breeze);

    App::run(window);
}
