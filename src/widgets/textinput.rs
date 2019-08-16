use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct TextInput {
    name: String,
    value: String,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl TextInput {
    pub fn new(name: &str) -> Self {
        TextInput {
            name: name.to_string(),
            value: "TextInput".to_string(),
            listener: None,
            observer: None,
        }
    }

    pub fn value(self, value: &str) -> Self {
        TextInput {
            name: self.name,
            value: value.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: self.listener,
            observer: Some(observer),
        }
    }

    pub fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].to_string();
            }
        }
    }
}

impl Widget for TextInput {
    fn eval(&self) -> String {
        format!(
            r#"<div class="textinput"><input value="{}" onchange="{}" /></div>"#,
            self.value,
            Event::js("change", &self.name, "value")
        )
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "change" {
                self.value = event.value.to_string();
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_change(&event.value);
                    }
                }
            }
        };
    }
}
