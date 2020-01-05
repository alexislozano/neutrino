use neutrino::utils::event::Key;
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
    MyButtonListener, MyCheckBoxDisabledListener, MyCheckBoxListener,
    MyComboListener, MyLabelListener, MyMenuBarListener, MyProgressBarListener,
    MyRadioListener, MyRangeListener, MyTabsListener, MyTextInputListener,
    MyWindowListener,
};
use demo_mod::models::{Panes, State};

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn main() {
    let panes = Rc::new(RefCell::new(Panes::new()));

    let state = Rc::new(RefCell::new(State::new()));

    let textinput_listener = MyTextInputListener::new(Rc::clone(&state));

    let mut textinput1 = TextInput::new("input1");
    textinput1.set_listener(Box::new(textinput_listener));
    textinput1.set_value("0");
    textinput1.set_placeholder("0-100");
    textinput1.set_size(4);

    let button_listener = MyButtonListener::new(Rc::clone(&state));

    let mut button1 = Button::new("button1");
    button1.set_text("Button");
    button1.set_stretched();
    button1.set_icon(Box::new(BreezeIcon::Check));
    button1.set_listener(Box::new(button_listener));

    let progressbar_listener = MyProgressBarListener::new(Rc::clone(&state));

    let mut progressbar1 = ProgressBar::new("progressbar1");
    progressbar1.set_listener(Box::new(progressbar_listener));
    progressbar1.set_value(0);
    progressbar1.set_stretched();

    let label_listener = MyLabelListener::new(Rc::clone(&state));

    let mut label1 = Label::new("label1");
    label1.set_listener(Box::new(label_listener));
    label1.set_text("0%");

    let checkbox_listener = MyCheckBoxListener::new(Rc::clone(&state));

    let mut checkbox1 = CheckBox::new("checkbox1");
    checkbox1.set_text("Checkbox");
    checkbox1.set_checked();
    checkbox1.set_listener(Box::new(checkbox_listener));

    let checkbox_disabled_listener =
        MyCheckBoxDisabledListener::new(Rc::clone(&state));

    let mut checkbox_disabled = CheckBox::new("checkbox_disabled");
    checkbox_disabled.set_text("Disabled");
    checkbox_disabled.set_listener(Box::new(checkbox_disabled_listener));

    let radio_listener = MyRadioListener::new(Rc::clone(&state));

    let mut radio1 = Radio::new("radio1");
    radio1.set_choices(vec!["Radio Button", "Radio Button"]);
    radio1.set_selected(0);
    radio1.set_listener(Box::new(radio_listener));

    let combo_listener = MyComboListener::new(Rc::clone(&state));

    let mut combo1 = Combo::new("combo1");
    combo1.set_choices(vec!["Combo Box", "Jumbo Fox"]);
    combo1.set_selected(0);
    combo1.set_listener(Box::new(combo_listener));

    let range_listener = MyRangeListener::new(Rc::clone(&state));

    let mut range1 = Range::new("range1");
    range1.set_listener(Box::new(range_listener));
    range1.set_min(0);
    range1.set_max(100);
    range1.set_value(0);
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
    container6.add(Box::new(checkbox_disabled));

    let mut label2 = Label::new("label2");
    label2.set_text("This is Tab 2.");

    let mut label3 = Label::new("label3");
    label3.set_unselectable();
    label3.set_text("This label text is unselectable");

    let mut container7 = Container::new("container7");
    container7.set_direction(Direction::Vertical);
    container7.add(Box::new(label2));
    container7.add(Box::new(label3));

    let mut label4 = Label::new("label4");
    label4.set_text("This is Tab 3");

    let tabs_listener = MyTabsListener::new(Rc::clone(&panes));

    let mut tabs1 = Tabs::new("tabs1");
    tabs1.set_selected(0);
    tabs1.set_listener(Box::new(tabs_listener));
    tabs1.add("Tab 1", Box::new(container6));
    tabs1.add("Tab 2", Box::new(container7));
    tabs1.add("Tab 3", Box::new(label4));

    let mut quitter = MenuFunction::new("Exit");
    quitter.set_shortcut("Ctrl-Q");

    let mut fichier = MenuItem::new("File", Key::F, 0);
    fichier.add(quitter);

    let mut onglet1 = MenuFunction::new("Tab 1");
    onglet1.set_shortcut("Ctrl-1");

    let mut onglet2 = MenuFunction::new("Tab 2");
    onglet2.set_shortcut("Ctrl-2");

    let mut onglet3 = MenuFunction::new("Tab 3");
    onglet3.set_shortcut("Ctrl-3");

    let mut onglets = MenuItem::new("Tabs", Key::T, 0);
    onglets.add(onglet1);
    onglets.add(onglet2);
    onglets.add(onglet3);

    let menubar_listener = MyMenuBarListener::new(Rc::clone(&panes));

    let mut menu_bar = MenuBar::new();
    menu_bar.set_listener(Box::new(menubar_listener));
    menu_bar.add(fichier);
    menu_bar.add(onglets);

    let app_listener = MyWindowListener::new(Rc::clone(&panes));

    let mut window = Window::new();
    window.set_title("Demo");
    window.set_size(440, 260);
    window.set_resizable();
    window.set_child(Box::new(tabs1));
    window.set_menubar(menu_bar);
    window.set_listener(Box::new(app_listener));
    window.set_debug();

    let args: Vec<String> = env::args().collect();

    window.set_theme(if args.len() == 2 {
        match args[1].as_str() {
            "adwaita" => Theme::Adwaita,
            "breeze" => Theme::Breeze,
            "fluent" => Theme::Fluent,
            "osx" => Theme::OSX,
            _ => Theme::Default,
        }
    } else {
        Theme::Default
    });

    App::run(window);
}
