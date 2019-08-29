use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction, Position};
use neutrino::widgets::tabs::Tabs;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

fn main() {
    
    // UI 1
    let mut ui1 = Container::new("ui1").direction(Direction::Vertical).position(Position::Between).stretched();

    let tl1 = Button::new("tl").text("TL").stretched();
    let tr1 = Button::new("tr").text("TR").stretched();
    let bl1 = Button::new("br").text("BL");
    let br1 = Button::new("br").text("BR");

    let mut t1 = Container::new("t1").direction(Direction::Horizontal);
    let mut b1 = Container::new("b1").direction(Direction::Horizontal).position(Position::Between);

    t1.add(Box::new(tl1));
    t1.add(Box::new(tr1));
    b1.add(Box::new(bl1));
    b1.add(Box::new(br1));

    ui1.add(Box::new(t1));
    ui1.add(Box::new(b1));

    // UI 2
    let mut ui2 = Container::new("ui2").direction(Direction::Horizontal).position(Position::Between).stretched();

    let tl2 = Button::new("tl").text("TL").stretched();
    let bl2 = Button::new("br").text("BL").stretched();
    let tr2 = Button::new("tr").text("TR");
    let br2 = Button::new("br").text("BR");

    let mut l2 = Container::new("l2").direction(Direction::Vertical);
    let mut r2 = Container::new("r2").direction(Direction::Vertical).position(Position::Between);

    l2.add(Box::new(tl2));
    l2.add(Box::new(bl2));
    r2.add(Box::new(tr2));
    r2.add(Box::new(br2));

    ui2.add(Box::new(l2));
    ui2.add(Box::new(r2));

    // UI 3
    let mut ui3 = Container::new("ui3").direction(Direction::Vertical).position(Position::Between).stretched();

    let b3 = Button::new("button3").text("Button").stretched();
    ui3.add(Box::new(b3));
    
    // UI 4
    let mut ui4 = Container::new("ui4").direction(Direction::Vertical).position(Position::End).stretched();
    let mut t4 = Container::new("t4").direction(Direction::Horizontal).stretched();

    let b4 = Button::new("button4").text("Button").stretched();
    t4.add(Box::new(b4));

    ui4.add(Box::new(t4));

    // Tabs
    let mut tabs = Tabs::new("tabs").stretched();
    tabs.add(("UI 1", Box::new(ui1)));
    tabs.add(("UI 2", Box::new(ui2)));
    tabs.add(("UI 3", Box::new(ui3)));

    // Root
    let mut root = Container::new("root").direction(Direction::Horizontal);
    root.add(Box::new(tabs));
    root.add(Box::new(ui4));

    let window = Window::new()
        .title("Custom")
        .size(800, 600)
        .resizable(true)
        .child(Box::new(root))
        .theme(Theme::Breeze);

    App::run(window);
}
