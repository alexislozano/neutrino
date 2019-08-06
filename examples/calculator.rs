use std::rc::Rc;
use std::cell::RefCell;

use neutrino::{App, Window};
use neutrino::widgets::input::Input;
use neutrino::widgets::container::Container;

mod calculator_mod;

use calculator_mod::models::Calculator;
use calculator_mod::listeners::{Input1Listener, Input2Listener};
use calculator_mod::observables::{Result1Observable, Input1Observable, Input2Observable};

fn main() {
    let calculator = Calculator::new();
    let rcalculator = Rc::new(RefCell::new(calculator));

    let input1listener = Input1Listener::new(Rc::clone(&rcalculator));
    let input1observable = Input1Observable::new(Rc::clone(&rcalculator));

    let input1 = Input::new("input1")
        .listener(Box::new(input1listener))
        .observable(Box::new(input1observable));

    let input2listener = Input2Listener::new(Rc::clone(&rcalculator));
    let input2observable = Input2Observable::new(Rc::clone(&rcalculator));

    let input2 = Input::new("input2")
        .listener(Box::new(input2listener))
        .observable(Box::new(input2observable));

    let result1observable = Result1Observable::new(Rc::clone(&rcalculator));

    let result1 = Input::new("result1")
        .observable(Box::new(result1observable));

    let mut container = Container::horizontal();

    container.add(Box::new(input1));
    container.add(Box::new(input2));
    container.add(Box::new(result1));

    let mut window = Window::new();
    window.add(Box::new(container));

    let app = App::new()
        .title("Calculator")
        .size(320, 240)
        .resizable(true);

    app.run(window);
}