use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

pub struct Input {
    name: String,
    value: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
}

impl Input {
    pub fn new(name: &str) -> Self {
        Input { 
            name: name.to_string(),
            value: "".to_string(),
            listener: None,
            observable: None,
        }
    }

    pub fn value(self, value: &str) -> Self {
        Input { 
            name: self.name,
            value: value.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Input { 
            name: self.name,
            value: self.value,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
        Input { 
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

impl Widget for Input {
    fn eval(&self) -> String {
        format!(
            "<div class=\"input\"><input value=\"{}\" onchange=\"(function(invut){{invoke({{event:'change', source:'{}', value: value}})}})()\" /></div>", 
            self.value, self.name
        )
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            match &self.listener {
                None => (),
                Some(listener) => {
                    if event.event == "change" {
                        listener.on_change(&event.value);
                    }
                }
            } 
        };
    }
}