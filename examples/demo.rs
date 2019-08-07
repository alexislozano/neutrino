use neutrino::{App, Window};
use neutrino::widgets::textinput::TextInput;
use neutrino::widgets::button::Button;
use neutrino::widgets::container::Container;
use neutrino::widgets::progressbar::ProgressBar;
use neutrino::widgets::label::Label;
use neutrino::widgets::checkbox::CheckBox;
use neutrino::widgets::radio::Radio;
use neutrino::widgets::combo::Combo;

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
        .choices(vec!["Bouton radio 1", "Bouton radio 2"])
        .selected(0);

    let combo1 = Combo::new("combo1")
        .choices(vec!["Combo 1", "Combo 2", "Combo 3"])
        .selected(0)
        .opened(true);

    let mut container = Container::vertical();

    container.add(Box::new(textinput1));
    container.add(Box::new(button1));
    container.add(Box::new(progressbar1));
    container.add(Box::new(label1));
    container.add(Box::new(checkbox1));
    container.add(Box::new(radio1));
    container.add(Box::new(combo1));

    let mut window = Window::new();
    window.add(Box::new(container));

    let app = App::new()
        .title("Demo")
        .size(640, 480)
        .resizable(true);

    app.run(window);
}