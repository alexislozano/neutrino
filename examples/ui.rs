use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction, Position};
use neutrino::widgets::tabs::Tabs;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

fn main() {
    
    // UI 1
    let mut ui1 = Container::new().direction(Direction::Vertical).position(Position::Between).stretch();

    let tl1 = Button::new("tl").text("TL").stretch();
    let tr1 = Button::new("tr").text("TR").stretch();
    let bl1 = Button::new("br").text("BL");
    let br1 = Button::new("br").text("BR");

    let mut t1 = Container::new().direction(Direction::Horizontal);
    let mut b1 = Container::new().direction(Direction::Horizontal).position(Position::Between);

    t1.add(Box::new(tl1));
    t1.add(Box::new(tr1));
    b1.add(Box::new(bl1));
    b1.add(Box::new(br1));

    ui1.add(Box::new(t1));
    ui1.add(Box::new(b1));

    // UI 2
    let mut ui2 = Container::new().direction(Direction::Horizontal).position(Position::Between).stretch();

    let tl2 = Button::new("tl").text("TL").stretch();
    let bl2 = Button::new("br").text("BL").stretch();
    let tr2 = Button::new("tr").text("TR");
    let br2 = Button::new("br").text("BR");

    let mut l2 = Container::new().direction(Direction::Vertical);
    let mut r2 = Container::new().direction(Direction::Vertical).position(Position::Between);

    l2.add(Box::new(tl2));
    l2.add(Box::new(bl2));
    r2.add(Box::new(tr2));
    r2.add(Box::new(br2));

    ui2.add(Box::new(l2));
    ui2.add(Box::new(r2));
    
    // UI 3
    let mut ui3 = Container::new().direction(Direction::Horizontal).position(Position::Between).stretch();

    let tl3 = Button::new("tl").text("TL").stretch();
    let bl3 = Button::new("br").text("BL").stretch();
    let tr3 = Button::new("tr").text("TR");
    let br3 = Button::new("br").text("BR");

    let mut l3 = Container::new().direction(Direction::Vertical);
    let mut r3 = Container::new().direction(Direction::Vertical).position(Position::Between);

    l3.add(Box::new(tl3));
    l3.add(Box::new(bl3));
    r3.add(Box::new(tr3));
    r3.add(Box::new(br3));

    ui3.add(Box::new(l3));
    ui3.add(Box::new(r3));

    // Tabs
    let mut tabs = Tabs::new("root").stretch();
    tabs.add(("UI 1", Box::new(ui1)));
    tabs.add(("UI 2", Box::new(ui2)));

    // Root
    let mut root = Container::new().direction(Direction::Horizontal);
    root.add(Box::new(tabs));
    root.add(Box::new(ui3));

    let window = Window::new()
        .title("Custom")
        .size(800, 600)
        .resizable(true)
        .child(Box::new(root))
        .theme(Theme::Breeze);

    App::run(window);
}
