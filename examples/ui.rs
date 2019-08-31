use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction, Position, Alignment};
use neutrino::widgets::tabs::Tabs;
use neutrino::utils::theme::Theme;
use neutrino::{App, Window};

fn main() {
    
    // UI 1
    let mut ui1 = Container::new("ui1");
    ui1.set_direction(Direction::Vertical);
    ui1.set_position(Position::Between);
    ui1.set_alignment(Alignment::Stretched);

    let mut tl1 = Button::new("tl");
    tl1.set_text("TL");
    tl1.set_stretched();

    let mut tr1 = Button::new("tr");
    tr1.set_text("TR");
    tr1.set_stretched();
    
    let mut bl1 = Button::new("br");
    bl1.set_text("BL");
    
    let mut br1 = Button::new("br");
    br1.set_text("BR");

    let mut t1 = Container::new("t1");
    t1.set_direction(Direction::Horizontal);
    
    let mut b1 = Container::new("b1");
    b1.set_direction(Direction::Horizontal);
    b1.set_position(Position::Between);

    t1.add(Box::new(tl1));
    t1.add(Box::new(tr1));
    b1.add(Box::new(bl1));
    b1.add(Box::new(br1));

    ui1.add(Box::new(t1));
    ui1.add(Box::new(b1));

    // UI 2
    let mut ui2 = Container::new("ui2");
    ui2.set_direction(Direction::Horizontal);
    ui2.set_position(Position::Between);
    ui2.set_alignment(Alignment::Stretched);

    let mut tl2 = Button::new("tl");
    tl2.set_text("TL");
    tl2.set_stretched();
    
    let mut bl2 = Button::new("br");
    bl2.set_text("BL");
    bl2.set_stretched();
    
    let mut tr2 = Button::new("tr");
    tr2.set_text("TR");
    
    let mut br2 = Button::new("br");
    br2.set_text("BR");

    let mut l2 = Container::new("l2");
    l2.set_direction(Direction::Vertical);
    
    let mut r2 = Container::new("r2");
    r2.set_direction(Direction::Vertical);
    r2.set_position(Position::Between);

    l2.add(Box::new(tl2));
    l2.add(Box::new(bl2));
    r2.add(Box::new(tr2));
    r2.add(Box::new(br2));

    ui2.add(Box::new(l2));
    ui2.add(Box::new(r2));

    // UI 3
    let mut ui3 = Container::new("ui3");
    ui3.set_direction(Direction::Vertical);
    ui3.set_position(Position::Between);
    ui3.set_alignment(Alignment::Stretched);

    let mut b3 = Button::new("button3");
    b3.set_text("Button");
    b3.set_stretched();
    ui3.add(Box::new(b3));
    
    // UI 4
    let mut ui4 = Container::new("ui4");
    ui4.set_direction(Direction::Vertical);
    ui4.set_position(Position::End);
    ui4.set_alignment(Alignment::Stretched);

    let mut t4 = Container::new("t4");
    t4.set_direction(Direction::Horizontal);
    t4.set_alignment(Alignment::Stretched);

    let mut b4 = Button::new("button4");
    b4.set_text("Button");
    b4.set_stretched();
    
    t4.add(Box::new(b4));

    ui4.add(Box::new(t4));

    // Tabs
    let mut tabs = Tabs::new("tabs");
    tabs.set_stretched();
    tabs.add("UI 1", Box::new(ui1));
    tabs.add("UI 2", Box::new(ui2));
    tabs.add("UI 3", Box::new(ui3));

    // Root
    let mut root = Container::new("root");
    root.set_direction(Direction::Horizontal);
    root.add(Box::new(tabs));
    root.add(Box::new(ui4));

    let window = Window::new()
        .title("Custom")
        .size(800, 600)
        .resizable(true)
        .child(Box::new(root))
        .theme(Theme::Adwaita);

    App::run(window);
}
