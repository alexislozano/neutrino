use neutrino::widgets::image::Image;
use neutrino::utils::theme::Theme;
use neutrino::utils::icon::BreezeIcon;
use neutrino::{App, Window};

fn main() {
    let image1 = Image::from_icon("image1", Box::new(BreezeIcon::Plus))
        .keep_ratio_aspect()
        .background_color("fuschia");

    let window = Window::new()
        .title("Image")
        .size(320, 240)
        .resizable(true)
        .child(Box::new(image1))
        .theme(Theme::Breeze);

    App::run(window);
}
