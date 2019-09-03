use std::cell::RefCell;
use std::rc::Rc;

use neutrino::{App, Window};
use neutrino::widgets::image::Image;
use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction};
use neutrino::utils::icon::BreezeIcon;

mod image_viewer_mod;
use image_viewer_mod::models::Images;
use image_viewer_mod::listeners::{MyImageListener, MyPrevButtonListener, MyNextButtonListener};

fn main() {
    let images = Rc::new(RefCell::new(Images::new()));

    let image_listener = MyImageListener::new(Rc::clone(&images));

    let mut image = Image::from_path("my_image", "");
    image.set_keep_ratio_aspect();
    image.set_listener(Box::new(image_listener));

    let prev_listener = MyPrevButtonListener::new(Rc::clone(&images));

    let mut button_prev = Button::new("button_prev");
    button_prev.set_icon(Box::new(BreezeIcon::Left));
    button_prev.set_listener(Box::new(prev_listener));

    let next_listener = MyNextButtonListener::new(Rc::clone(&images));

    let mut button_next = Button::new("button_next");
    button_next.set_icon(Box::new(BreezeIcon::Right));
    button_next.set_listener(Box::new(next_listener));

    let mut root = Container::new("root");
    root.set_direction(Direction::Horizontal);
    root.add(Box::new(button_prev));
    root.add(Box::new(image));
    root.add(Box::new(button_next));

    let mut window = Window::new();
    window.set_title("Image viewer");
    window.set_size(640, 480);
    window.set_child(Box::new(root));

    App::run(window);
}