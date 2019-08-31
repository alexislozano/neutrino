use neutrino::widgets::button::Button;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

fn main() {
    
    let mut button = Button::new("button");
    button.set_text("Hello World!");
    button.set_stretched();

    let window = Window::new()
        .title("Hello World")
        .size(800, 600)
        .resizable(true)
        .child(Box::new(button))
        .theme(Theme::Adwaita);

    App::run(window);
}
