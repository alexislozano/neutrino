use std::cell::RefCell;
use std::rc::Rc;

use neutrino::utils::icon::BreezeIcon;
use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction};
use neutrino::widgets::image::Image;
use neutrino::widgets::menubar::{MenuBar, MenuFunction, MenuItem};
use neutrino::{App, Window};

mod image_viewer_mod;
use image_viewer_mod::listeners::{
    MyImageListener, MyMenuBarListener, MyNextButtonListener,
    MyPrevButtonListener, MyWindowListener,
};
use image_viewer_mod::models::Images;

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

    let mut prev_function = MenuFunction::new("Previous");
    prev_function.set_shortcut("Ctrl+Left");

    let mut next_function = MenuFunction::new("Next");
    next_function.set_shortcut("Ctrl+Right");

    let mut menuitem = MenuItem::new("File");
    menuitem.add(prev_function);
    menuitem.add(next_function);

    let menubar_listener = MyMenuBarListener::new(Rc::clone(&images));

    let mut menubar = MenuBar::new();
    menubar.add(menuitem);
    menubar.set_listener(Box::new(menubar_listener));

    let window_listener = MyWindowListener::new(Rc::clone(&images));

    let mut window = Window::new();
    window.set_title("Image viewer");
    window.set_size(640, 480);
    window.set_child(Box::new(root));
    window.set_menubar(menubar);
    window.set_listener(Box::new(window_listener));

    App::run(window);
}
