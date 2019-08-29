use crate::utils::event::Event;

pub struct MenuBarState {
    items: Vec<MenuItem>,
    selected_item: Option<u32>,
    selected_function: Option<u32>,
}

pub trait MenuBarListener {
    fn on_change(&self, state: &MenuBarState);
}

pub struct MenuBar {
    state: MenuBarState,
    listener: Option<Box<dyn MenuBarListener>>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            state: MenuBarState {
                items: vec![],
                selected_item: None,
                selected_function: None,
            },
            listener: None,
        }
    }

    pub fn listener(self, listener: Box<dyn MenuBarListener>) -> Self {
        Self {
            state: MenuBarState {
                items: self.state.items,
                selected_item: self.state.selected_item,
                selected_function: self.state.selected_function,
            },
            listener: Some(listener),
        }
    }

    pub fn add(&mut self, item: MenuItem) {
        self.state.items.push(item);
    }
    
    pub fn eval(&self) -> String {
        let mut s = r#"<div class="menubar">"#.to_string();
        for (i, item) in self.state.items.iter().enumerate() {
            let selected_item = match self.state.selected_item {
                None => false,
                Some(selected_item) => selected_item == i as u32
            };
            s.push_str(&item.eval(i, selected_item));
        }
        s.push_str(r#"</div>"#);
        s
    }

    pub fn trigger(&mut self, event: &Event) {
        match event {
            Event::Change { source, value } => {
                if source == "menuitem" {
                    self.on_item_change(value);    
                } else if source == "menufunction" {
                    self.on_function_change(value);
                } else {
                    self.state.selected_item = None;
                }
            },
            _ => self.state.selected_item = None,
        }
    }

    fn on_item_change(&mut self, value: &str) {
        let values = value.split(";").collect::<Vec<&str>>();
        let e = values[0];
        let index = values[1].parse::<u32>().unwrap();
        self.state.selected_item = match self.state.selected_item {
            Some(_) => match e {
                "click" => None,
                _ => Some(index),
            },
            None => match e {
                "click" => Some(index),
                _ => None,
            }
        }
    }

    fn on_function_change(&mut self, value: &str) {
        self.state.selected_function = Some(value.parse::<u32>().unwrap());
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        };
        self.state.selected_item = None;
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
            r#"<div class="menuitem"><div class="menuitem-title {}" onmousedown="{}" onmouseover="{}">{}</div>"#,
            selected_str, 
            Event::change_js("menuitem", &format!("'click;{}'", index)), 
            Event::change_js("menuitem", &format!("'over;{}'", index)), 
            self.name
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