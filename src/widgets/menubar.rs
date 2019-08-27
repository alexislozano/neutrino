use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;

pub struct MenuBar {
    items: Vec<MenuItem>,
    selected: Option<u32>,
    listener: Option<Box<dyn Listener>>,
}

impl MenuBar {
    pub fn new() -> Self {
        MenuBar {
            items: vec![],
            selected: None,
            listener: None,
        }
    }

    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        MenuBar {
            items: self.items,
            selected: self.selected,
            listener: Some(listener),
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
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == "menuitem" {
                    let values = value.split(";").collect::<Vec<&str>>();
                    let e = values[0];
                    let index = values[1].parse::<u32>().unwrap();
                    self.selected = match self.selected {
                        Some(_) => match e {
                            "click" => None,
                            _ => Some(index),
                        },
                        None => match e {
                            "click" => Some(index),
                            _ => None,
                        }
                    }
                } else if source == "menufunction" {
                    let selected = value.parse::<u32>().unwrap();
                    match &self.listener {
                        None => (),
                        Some(listener) => {
                            listener.on_change(&format!(
                                "{};{}",
                                self.selected.unwrap(), selected
                            ));
                        }
                    };
                    self.selected = None;
                } else {
                    self.selected = None;
                }
            },
            _ => self.selected = None,
        }
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// ```
    fn on_update(&mut self) {}
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
            r#"<div class="menuitem"><div class="menuitem-title {}" onmousedown="{}" onmouseover="{}">{}</div>"#,
            selected_str, Event::change_js("menuitem", &format!("'click;{}'", index)), 
            Event::change_js("menuitem", &format!("'over;{}'", index)), self.name
        );
        if selected {
            s.push_str(r#"<div class="menufunctions">"#);
            for (i, function) in self.functions.iter().enumerate() {
                s.push_str(&function.eval(i));
            }
            s.push_str(r#"</div>"#);   
        }
        s.push_str(r#"</div>"#);
        s
    }
}

pub struct MenuFunction {
    name: String,
    shortcut: Option<String>,
}

impl MenuFunction {
    pub fn new(name: &str) -> Self {
        MenuFunction {
            name: name.to_string(),
            shortcut: None,
        }
    }

    pub fn shortcut(self, shortcut: &str) -> Self {
        MenuFunction {
            name: self.name,
            shortcut: Some(shortcut.to_string()),
        }
    }

    fn eval(&self, index: usize) -> String {
        format!(
            r#"<div class="menufunction" onmousedown="{}"><span class="title">{}</span><span class="shortcut">{}</span></div>"#,
            Event::change_js("menufunction", &format!("'{}'", index)),
            self.name, match &self.shortcut {
                None => "",
                Some(shortcut) => shortcut,
            },
        )
    }
}