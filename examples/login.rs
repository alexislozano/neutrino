use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::button::{Button, ButtonListener, ButtonState};
use neutrino::widgets::container::{Container, Direction, Position};
use neutrino::widgets::label::Label;
use neutrino::widgets::textinput::{
    InputType, TextInput, TextInputListener, TextInputState,
};
use neutrino::{App, Window};

struct Login {
    username: String,
    password: String,
    ok: bool,
}

impl Login {
    fn new() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
            ok: false,
        }
    }

    fn check(&mut self) {
        self.ok = &self.username == "Neutrino" && &self.password == "is great !";
    }

    fn ok(&self) -> bool {
        self.ok
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }
}

struct MyButtonListener {
    login: Rc<RefCell<Login>>,
}

impl MyButtonListener {
    fn new(login: Rc<RefCell<Login>>) -> Self {
        Self { login }
    }
}

impl ButtonListener for MyButtonListener {
    fn on_change(&self, _state: &ButtonState) {
        self.login.borrow_mut().check();
    }

    fn on_update(&self, state: &mut ButtonState) {
        if self.login.borrow().ok() {
            state.set_style(
                r#"
                $color: forestgreen;
                background-color: $color;
                border-color: $color;
            "#,
            );
            state.set_text("OK");
        } else {
            state.set_style(
                r#"
                $color: crimson;
                background-color: $color;
                border-color: $color;
            "#,
            );
            state.set_text("KO");
        }
    }
}

struct MyUsernameListener {
    login: Rc<RefCell<Login>>,
}

impl MyUsernameListener {
    fn new(login: Rc<RefCell<Login>>) -> Self {
        Self { login }
    }
}

impl TextInputListener for MyUsernameListener {
    fn on_change(&self, state: &TextInputState) {
        self.login.borrow_mut().set_username(state.value());
    }

    fn on_update(&self, _state: &mut TextInputState) {}
}

struct MyPasswordListener {
    login: Rc<RefCell<Login>>,
}

impl MyPasswordListener {
    fn new(login: Rc<RefCell<Login>>) -> Self {
        Self { login }
    }
}

impl TextInputListener for MyPasswordListener {
    fn on_change(&self, state: &TextInputState) {
        self.login.borrow_mut().set_password(state.value());
    }

    fn on_update(&self, _state: &mut TextInputState) {}
}

fn main() {
    let login = Rc::new(RefCell::new(Login::new()));

    let mut username_label = Label::new("username_label");
    username_label.set_text("Username");

    let mut username_input = TextInput::new("username_input");

    let username_listener = MyUsernameListener::new(Rc::clone(&login));
    username_input.set_listener(Box::new(username_listener));

    let mut password_label = Label::new("password_label");
    password_label.set_text("Password");

    let mut password_input = TextInput::new("password_input");
    password_input.set_input_type(InputType::Password);

    let password_listener = MyPasswordListener::new(Rc::clone(&login));
    password_input.set_listener(Box::new(password_listener));

    let mut button = Button::new("button");
    button.set_text("Log in");

    let button_listener = MyButtonListener::new(Rc::clone(&login));
    button.set_listener(Box::new(button_listener));

    let mut form = Container::new("form");
    form.set_direction(Direction::Vertical);

    form.add(Box::new(username_label));
    form.add(Box::new(username_input));
    form.add(Box::new(password_label));
    form.add(Box::new(password_input));

    let mut root = Container::new("root");
    root.set_direction(Direction::Vertical);
    root.set_position(Position::Between);

    root.add(Box::new(form));
    root.add(Box::new(button));

    let style = r#"
        #app {
            background-color: beige;
        }

        .textinput {
            margin-bottom: 20px;

            input {
                width: 100%;
                box-sizing: border-box;
                border-color: lightgrey;
                border-radius: 4px;

                &:focus {
                    border-color: grey;
                }
            }
        }

        .button {
            border-radius: 4px;
            color: white;
        }
    "#;

    let mut window = Window::new();
    window.set_title("Login");
    window.set_size(320, 240);
    window.set_child(Box::new(root));
    window.set_style(style);

    App::run(window);
}
