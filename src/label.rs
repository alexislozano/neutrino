use crate::widget::Widget;
use crate::utils::{Event, Listener, Observable};

pub struct Label {
    name: String,
    text: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable>>,
}

impl Label {
    pub fn new(name: &str) -> Self {
        Label { 
            name: name.to_string(),
            text: "".to_string(),
            listener: None,
            observable: None,
        }
    }

    pub fn text(self, text: &str) -> Self {
        Label { 
            name: self.name,
            text: text.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                self.text = observable.value();
            }
        }
    }
}

impl Widget for Label {
    fn eval(&self) -> String {
        format!(
            "{{ type: \"label\", text: \"{}\", name: \"{}\" }}", 
            self.text, self.name
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