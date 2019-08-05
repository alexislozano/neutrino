use neutrino::{App, Window};
use neutrino::widgets::custom::Custom;

fn main() {
    let custom1 = Custom::new("custom1")
        .fields(vec!["word", "name"])
        .template("<h1>{word}, it is {name} !</h1>");

    let mut window = Window::new();
    window.add(Box::new(custom1));

    let app = App::new()
        .title("Custom")
        .size(320, 240)
        .resizable(true);

    app.run(window);
}