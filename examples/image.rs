use neutrino::widgets::image::Image;
use neutrino::utils::theme::Theme;
use neutrino::utils::pixmap::Icon;
use neutrino::{App, Window};

fn main() {
    let image1 = Image::from_theme(Theme::Breeze, Icon::Plus)
        .keep_ratio_aspect(false)
        .background_color("fzfdgz");

    let window = Window::new()
        .title("Image")
        .size(320, 240)
        .resizable(true)
        .child(Box::new(image1))
        .theme(Theme::Breeze);

    App::run(window);
}
