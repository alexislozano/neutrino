use neutrino::{App, Window};
use neutrino::widgets::textinput::TextInput;
use neutrino::widgets::button::Button;
use neutrino::widgets::container::Container;
use neutrino::widgets::progressbar::ProgressBar;
use neutrino::widgets::label::Label;
use neutrino::widgets::checkbox::CheckBox;
use neutrino::widgets::radio::Radio;
use neutrino::widgets::combo::Combo;
use neutrino::widgets::range::Range;
use neutrino::widgets::tabs::Tabs;
use neutrino::widgets::custom::Custom;

fn main() {
    
    let textinput1 = TextInput::new("input1")
        .value("0");
    
    let button1 = Button::new("button1")
        .text("Bouton à pousser");

    let progressbar1 = ProgressBar::new("progressbar1")
        .value(70);

    let label1 = Label::new("label1")
        .text("70%");

    let checkbox1 = CheckBox::new("checkbox1")
        .text("Case à cocher")
        .checked(true);

    let radio1 = Radio::new("radio1")
        .choices(vec!["Bouton radio", "Bouton radio"])
        .selected(0);

    let combo1 = Combo::new("combo1")
        .choices(vec!["Combo box", "Combo fox"])
        .selected(0);

    let range1 = Range::new("range1")
        .min(0)
        .max(100)
        .value(25);

    let mut container1 = Container::vertical();
    container1.add(Box::new(checkbox1));
    container1.add(Box::new(radio1));

    let mut container2 = Container::horizontal();
    container2.add(Box::new(button1));
    container2.add(Box::new(textinput1));

    let mut container3 = Container::vertical();
    container3.add(Box::new(combo1));
    container3.add(Box::new(container2));

    let mut container4 = Container::horizontal();
    container4.add(Box::new(container1));
    container4.add(Box::new(container3));

    let mut container5 = Container::horizontal();
    container5.add(Box::new(range1));
    container5.add(Box::new(progressbar1));
    container5.add(Box::new(label1));

    let custom1 = Custom::new("custom1")
        .template(r#"<h2 style="margin: 6px;">This is Tab 1</h2>"#);

    let mut container6 = Container::vertical();
    container6.add(Box::new(custom1));
    container6.add(Box::new(container4));
    container6.add(Box::new(container5));

    let custom2 = Custom::new("custom2")
        .template(r#"<h2 style="margin: 6px;">This is Tab 2</h2>"#);

    let mut tabs1 = Tabs::new("tabs1")
        .selected(0);
    tabs1.add(("Onglet 1", Box::new(container6)));
    tabs1.add(("Onglet 2", Box::new(custom2)));

    let mut window = Window::new();
    window.add(Box::new(tabs1));

    let app = App::new()
        .title("Demo")
        .size(440, 260)
        .resizable(true);

    app.run(window);
}