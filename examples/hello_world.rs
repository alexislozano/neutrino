use neutrino::widgets::label::Label;
use neutrino::{App, Window};

fn main() {
    let mut label = Label::new("my_label");
    label.set_text("Hello World !");

    let mut window = Window::new();
    window.set_title("Hello World");
    window.set_size(320, 240);
    window.set_child(Box::new(label));

    App::run(window);
}
