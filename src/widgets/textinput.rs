use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;
use crate::widgets::widget::Widget;

pub struct TextInput {
    name: String,
    value: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
}

impl TextInput {
    pub fn new(name: &str) -> Self {
        TextInput {
            name: name.to_string(),
            value: "TextInput".to_string(),
            listener: None,
            observable: None,
        }
    }

    pub fn value(self, value: &str) -> Self {
        TextInput {
            name: self.name,
            value: value.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    pub fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                self.value = observable.observe()["value"].to_string();
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
