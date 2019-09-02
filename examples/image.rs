use neutrino::widgets::image::Image;
use neutrino::utils::theme::Theme;
use neutrino::utils::icon::BreezeIcon;
use neutrino::{App, Window};

fn main() {
    let mut image1 = Image::from_icon("image1", Box::new(BreezeIcon::Plus));
    image1.set_keep_ratio_aspect();
    image1.set_background_color("fuschia");

    let mut window = Window::new();
    window.set_title("Image");
    window.set_size(320, 240);
    window.set_resizable();
    window.set_child(Box::new(image1));
    window.set_theme(Theme::Breeze);

    App::run(window);
}
