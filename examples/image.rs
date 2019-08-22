use neutrino::widgets::image::Image;
use neutrino::{App, Window};

fn main() {
    let image1 = Image::from_path("/home/alexis/Pictures/code.png")
        .keep_ratio_aspect(false)
        .background_color("fzfdgz");

    let window = Window::new()
        .title("Image")
        .size(320, 240)
        .resizable(true)
        .child(Box::new(image1));

    App::run(window);
}
