use crate::widget::Widget;
use crate::utils::{Event, Listener};

pub struct Button {
    name: String,
    text: String,
    disabled: bool,
    listener: Option<Box<Listener>>,
}

impl Button {
    pub fn new(name: &str) -> Self {
        Button { 
            name: name.to_string(),
            text: "".to_string(), 
            disabled: false,
            listener: None,
        }
    }

    pub fn text(self, text: &str) -> Self {
        Button { 
            name: self.name,
            text: text.to_string(), 
            disabled: self.disabled,
            listener: self.listener,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Button { 
            name: self.name,
            text: self.text, 
            disabled: disabled,
            listener: self.listener, 
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Button { 
            name: self.name,
            text: self.text, 
            disabled: self.disabled,
            listener: Some(listener), 
        }
    }

    pub fn on_update(&self) {}
}

impl Widget for Button {
    fn eval(&self) -> String {
        format!(
            "{{ type: \"button\", name: \"{}\", text: \"{}\", disabled: {} }}", 
            self.name, self.text, self.disabled
        )
    }

    fn trigger(&mut self, event: &Event) {
        match &self.listener {
            None => (),
            Some(listener) => { 
                if event.event == "click" && event.source == self.name {
                    listener.on_click();
                } else if event.event == "update" {
                    self.on_update();
                }
            } 
        };
    }
}
