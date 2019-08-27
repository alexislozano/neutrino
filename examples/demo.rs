use neutrino::widgets::button::Button;
use neutrino::widgets::checkbox::CheckBox;
use neutrino::widgets::combo::Combo;
use neutrino::widgets::container::{Container, Direction};
use neutrino::widgets::custom::Custom;
use neutrino::widgets::label::Label;
use neutrino::widgets::progressbar::ProgressBar;
use neutrino::widgets::radio::Radio;
use neutrino::widgets::range::Range;
use neutrino::widgets::tabs::Tabs;
use neutrino::widgets::textinput::TextInput;
use neutrino::widgets::menubar::{MenuBar, MenuItem, MenuFunction};
use neutrino::utils::icon::BreezeIcon;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

mod demo_mod;

use demo_mod::listeners::{AppListener, TabsListener, MenuBarListener};
use demo_mod::observers::TabsObserver;
use demo_mod::models::Panes;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let panes = Panes::new();
    let rpanes = Rc::new(RefCell::new(panes));

    let textinput1 = TextInput::new("input1").value("0").size(4);

    let button1 = Button::new("button1").text("Bouton à pousser").stretch();

    let progressbar1 = ProgressBar::new("progressbar1").value(70).stretch();

    let label1 = Label::new("label1").text("70%");

    let checkbox1 = CheckBox::new("checkbox1")
        .text("Case à cocher")
        .checked(true);

    let radio1 = Radio::new("radio1")
        .choices(vec!["Bouton radio", "Bouton radio"])
        .selected(0);

    let combo1 = Combo::new("combo1")
        .choices(vec!["Combo box", "Combo fox"])
        .selected(0)
        .arrow_from_icon(Box::new(BreezeIcon::Down));

    let range1 = Range::new("range1").min(0).max(100).value(25).stretch();

    let mut container1 = Container::new()
        .direction(Direction::Vertical)
        .stretch();
    container1.add(Box::new(checkbox1));
    container1.add(Box::new(radio1));

    let mut container2 = Container::new()
        .direction(Direction::Horizontal);
    container2.add(Box::new(button1));
    container2.add(Box::new(textinput1));

    let mut container3 = Container::new()
        .direction(Direction::Vertical)
        .stretch();
    container3.add(Box::new(combo1));
    container3.add(Box::new(container2));

    let mut container4 = Container::new()
        .direction(Direction::Horizontal);
    container4.add(Box::new(container1));
    container4.add(Box::new(container3));

    let mut container5 = Container::new()
        .direction(Direction::Horizontal);
    container5.add(Box::new(range1));
    container5.add(Box::new(progressbar1));
    container5.add(Box::new(label1));

    let mut container6 = Container::new()
        .direction(Direction::Vertical);
    container6.add(Box::new(container4));
    container6.add(Box::new(container5));

    let custom2 = Custom::new("custom2").template(r#"<h2 style="margin: 0;">This is Tab 2</h2>"#);

    let tabs_observer = TabsObserver::new(Rc::clone(&rpanes));
    let tabs_listener = TabsListener::new(Rc::clone(&rpanes));

    let mut tabs1 = Tabs::new("tabs1")
        .selected(0)
        .observer(Box::new(tabs_observer))
        .listener(Box::new(tabs_listener));
    tabs1.add(("Onglet 1", Box::new(container6)));
    tabs1.add(("Onglet 2", Box::new(custom2)));

    let mut fichier = MenuItem::new("Fichier");
    fichier.add(MenuFunction::new("Quitter").shortcut("Ctrl-Q"));

    let mut onglets = MenuItem::new("Onglets");
    onglets.add(MenuFunction::new("Onglet 1").shortcut("Ctrl-1"));
    onglets.add(MenuFunction::new("Onglet 2").shortcut("Ctrl-2"));

    let menubar_listener = MenuBarListener::new(Rc::clone(&rpanes));

    let mut menu_bar = MenuBar::new()
        .listener(Box::new(menubar_listener));
    menu_bar.add(fichier);
    menu_bar.add(onglets);

    let app_listener = AppListener::new(Rc::clone(&rpanes));

    let window = Window::new()
        .title("Demo")
        .size(440, 260)
        .resizable(true)
        .child(Box::new(tabs1))
        .menubar(menu_bar)
        .listener(Box::new(app_listener))
        .theme(Theme::Breeze);

    App::run(window);
}
