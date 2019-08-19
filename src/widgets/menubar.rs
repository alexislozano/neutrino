use crate::widgets::widget::Widget;
use crate::utils::event::Event;

pub struct MenuBar {
    items: Vec<MenuItem>,
    selected: Option<u32>,
}

impl MenuBar {
    pub fn new() -> Self {
        MenuBar {
            items: vec![],
            selected: None,
        }
    }

    pub fn add(&mut self, item: MenuItem) {
        self.items.push(item);
    }
}

impl Widget for MenuBar {
    fn eval(&self) -> String {
        let mut s = r#"<div class="menubar">"#.to_string();
        for (i, item) in self.items.iter().enumerate() {
            let selected = match self.selected {
                None => false,
                Some(selected) => selected == i as u32
            };
            s.push_str(&item.eval(i, selected));
        }
        s.push_str(r#"</div>"#);
        s
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Key { key: _ } => (),
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == "menubar" {
                    self.selected = Some(value.parse::<u32>().unwrap());
                }
            },
        }
    }
}

pub struct MenuItem {
    name: String,
    functions: Vec<MenuFunction>,
}

impl MenuItem {
    pub fn new(name: &str) -> Self {
        MenuItem {
            name: name.to_string(),
            functions: vec![],
        }
    }

    pub fn add(&mut self, function: MenuFunction) {
        self.functions.push(function);
    }
    
    fn eval(&self, index: usize, selected: bool) -> String {
        let selected_str = if selected {
            "selected"
        } else {
            ""
        };
        let mut s = format!(
            r#"<div class="menuitem"><div class="menuitem-title {}" onclick="{}">{}</div>"#,
            selected_str, Event::change_js("menubar", &format!("'{}'", index)), self.name
        );
        if selected {
            s.push_str(r#"<div class="menufunctions">"#);
            for (_i, function) in self.functions.iter().enumerate() {
                s.push_str(&function.eval());
            }
            s.push_str(r#"</div>"#);   
        }
        s.push_str(r#"</div>"#);
        s
    }
}

pub struct MenuFunction {
    name: String,
}

impl MenuFunction {
    pub fn new(name: &str) -> Self {
        MenuFunction {
            name: name.to_string(),
        }
    }

    fn eval(&self) -> String {
        format!(
            r#"<div class="menufunction">{}</div>"#,
            self.name
        )
    }
}