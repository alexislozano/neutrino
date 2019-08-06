use strfmt::strfmt;
use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;
use std::collections::HashMap;

pub struct Custom {
    name: String,
    fields: HashMap<String, String>,
    template: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
}

impl Custom {
    pub fn new(name: &str) -> Self {
        Custom { 
            name: name.to_string(),
            fields: HashMap::new(),
            template: "".to_string(),
            listener: None,
            observable: None, 
        }
    }

    pub fn template(self, template: &str) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: template.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable<String>>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                for (key, value) in observable.observe() {
                    self.fields.insert(key, value);
                }
            }
        }
    }
}

impl Widget for Custom {
    fn eval(&self) -> String {
        strfmt(&self.template, &self.fields).unwrap_or(self.template.to_string())
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            match &self.listener {
                None => (),
                Some(listener) => {
                    if event.event == "click" {
                        listener.on_click();
                    }
                }
            } 
        };
    }
}