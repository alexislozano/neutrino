use crate::utils::event::Event;

pub struct MenuBarState {
    selected_item: Option<u32>,
    selected_function: Option<u32>,
}

impl MenuBarState {
    pub fn selected_item(&self) -> Option<u32> {
        self.selected_item
    }

    pub fn selected_function(&self) -> Option<u32> {
        self.selected_function
    }

    pub fn set_selected_item(&mut self, selected_item: Option<u32>) {
        self.selected_item = selected_item;
    }

    pub fn set_selected_function(&mut self, selected_function: Option<u32>) {
        self.selected_function = selected_function;
    }
}

pub trait MenuBarListener {
    fn on_change(&self, state: &MenuBarState);
}

pub struct MenuBar {
    items: Vec<MenuItem>,
    state: MenuBarState,
    listener: Option<Box<dyn MenuBarListener>>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            items: vec![],
            state: MenuBarState {
                selected_item: None,
                selected_function: None,
            },
            listener: None,
        }
    }

    pub fn set_listener(&mut self, listener: Box<dyn MenuBarListener>) {
        self.listener = Some(listener);
    }

    pub fn add(&mut self, item: MenuItem) {
        self.items.push(item);
    }
    
    pub fn eval(&self) -> String {
        let mut s = r#"<div class="menubar">"#.to_string();
        for (i, item) in self.items.iter().enumerate() {
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
            Event::Update => (),
            Event::Change { source, value } => {
                if *source == "menuitem" {
                    self.on_item_change(value);    
                } else if *source == "menufunction" {
                    self.on_function_change(value);
                } else {
                    self.state.set_selected_item(None);
                }
            },
            _ => self.state.set_selected_item(None),
        }
    }

    fn on_item_change(&mut self, value: &str) {
        let values = value.split(";").collect::<Vec<&str>>();
        let e = values[0];
        let index = values[1].parse::<u32>().unwrap();
        self.state.set_selected_item(match self.state.selected_item() {
            Some(_) => match e {
                "click" => None,
                _ => Some(index),
            },
            None => match e {
                "click" => Some(index),
                _ => None,
            }
        });
    }

    fn on_function_change(&mut self, value: &str) {
        self.state.set_selected_function(Some(value.parse::<u32>().unwrap()));
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        };
        self.state.set_selected_item(None);
    }
}

pub struct MenuItem {
    name: String,
    functions: Vec<MenuFunction>,
}

impl MenuItem {
    pub fn new(name: &str) -> Self {
        Self {
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
        Self {
            name: name.to_string(),
            shortcut: None,
        }
    }

    pub fn set_shortcut(&mut self, shortcut: &str) {
        self.shortcut = Some(shortcut.to_string());
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