use neutrino::utils::icon::BreezeIcon;
use neutrino::utils::theme::Theme;
use neutrino::widgets::button::Button;
use neutrino::widgets::checkbox::CheckBox;
use neutrino::widgets::combo::Combo;
use neutrino::widgets::container::{Alignment, Container, Direction};
use neutrino::widgets::label::Label;
use neutrino::widgets::menubar::{MenuBar, MenuFunction, MenuItem};
use neutrino::widgets::progressbar::ProgressBar;
use neutrino::widgets::radio::Radio;
use neutrino::widgets::range::Range;
use neutrino::widgets::tabs::Tabs;
use neutrino::widgets::textinput::TextInput;
use neutrino::{App, Window};

mod demo_mod;

use demo_mod::listeners::{
    MyMenuBarListener, MyTabsListener, MyWindowListener,
};
use demo_mod::models::Panes;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let panes = Panes::new();
    let rpanes = Rc::new(RefCell::new(panes));

    let mut textinput1 = TextInput::new("input1");
    textinput1.set_value("0");
    textinput1.set_size(4);

    let mut button1 = Button::new("button1");
    button1.set_text("Button");
    button1.set_stretched();
    button1.set_icon(Box::new(BreezeIcon::Check));

    let mut progressbar1 = ProgressBar::new("progressbar1");
    progressbar1.set_value(70);
    progressbar1.set_stretched();

    let mut label1 = Label::new("label1");
    label1.set_text("70%");

    let mut checkbox1 = CheckBox::new("checkbox1");
    checkbox1.set_text("Checkbox");
    checkbox1.set_checked();

    let mut radio1 = Radio::new("radio1");
    radio1.set_choices(vec!["Radio Button", "Radio Button"]);
    radio1.set_selected(0);

    let mut combo1 = Combo::new("combo1");
    combo1.set_choices(vec!["Combo Box", "Jumbo Fox"]);
    combo1.set_selected(0);
    combo1.set_icon(Box::new(BreezeIcon::Down));

    let mut range1 = Range::new("range1");
    range1.set_min(0);
    range1.set_max(100);
    range1.set_value(25);
    range1.set_stretched();

    let mut container1 = Container::new("container1");
    container1.set_direction(Direction::Vertical);
    container1.set_stretched();
    container1.add(Box::new(checkbox1));
    container1.add(Box::new(radio1));

    let mut container2 = Container::new("container2");
    container2.set_direction(Direction::Horizontal);
    container2.set_alignment(Alignment::Center);
    container2.add(Box::new(button1));
    container2.add(Box::new(textinput1));

    let mut container3 = Container::new("container3");
    container3.set_direction(Direction::Vertical);
    container3.set_stretched();
    container3.add(Box::new(combo1));
    container3.add(Box::new(container2));

    let mut container4 = Container::new("container4");
    container4.set_direction(Direction::Horizontal);
    container4.add(Box::new(container1));
    container4.add(Box::new(container3));

    let mut container5 = Container::new("container5");
    container5.set_direction(Direction::Horizontal);
    container5.set_alignment(Alignment::Center);
    container5.add(Box::new(range1));
    container5.add(Box::new(progressbar1));
    container5.add(Box::new(label1));

    let mut container6 = Container::new("container6");
    container6.set_direction(Direction::Vertical);
    container6.add(Box::new(container4));
    container6.add(Box::new(container5));

    let mut label2 = Label::new("label2");
    label2.set_text("This is Tab 2.");

    let tabs_listener = MyTabsListener::new(Rc::clone(&rpanes));

    let mut tabs1 = Tabs::new("tabs1");
    tabs1.set_selected(0);
    tabs1.set_listener(Box::new(tabs_listener));
    tabs1.add("Tab 1", Box::new(container6));
    tabs1.add("Tab 2", Box::new(label2));

    let mut quitter = MenuFunction::new("Exit");
    quitter.set_shortcut("Ctrl-Q");

    let mut fichier = MenuItem::new("File");
    fichier.add(quitter);

    let mut onglet1 = MenuFunction::new("Tab 1");
    onglet1.set_shortcut("Ctrl-1");

    let mut onglet2 = MenuFunction::new("Tab 2");
    onglet2.set_shortcut("Ctrl-2");

    let mut onglets = MenuItem::new("Tabs");
    onglets.add(onglet1);
    onglets.add(onglet2);

    let menubar_listener = MyMenuBarListener::new(Rc::clone(&rpanes));

    let mut menu_bar = MenuBar::new();
    menu_bar.set_listener(Box::new(menubar_listener));
    menu_bar.add(fichier);
    menu_bar.add(onglets);

    let app_listener = MyWindowListener::new(Rc::clone(&rpanes));

    let mut window = Window::new();
    window.set_title("Demo");
    window.set_size(440, 260);
    window.set_resizable();
    window.set_child(Box::new(tabs1));
    window.set_menubar(menu_bar);
    window.set_listener(Box::new(app_listener));
    window.set_theme(Theme::Default);
    
    App::run(window);
}
