use neutrino::widgets::button::Button;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

fn main() {
    
    let mut button = Button::new("button");
    button.set_text("Hello World!");
    button.set_stretched();

    let mut window = Window::new();
    window.set_title("Hello World");
    window.set_size(800, 600);
    window.set_resizable();
    window.set_child(Box::new(button));
    window.set_theme(Theme::Adwaita);

    App::run(window);
}
