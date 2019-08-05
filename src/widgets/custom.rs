use strfmt::strfmt;
use std::collections::HashMap;
use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

pub struct Custom {
    name: String,
    fields: HashMap<String, String>,
    template: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<i32>>>,
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

    pub fn fields(self, fields: Vec<&str>) -> Custom {
        let mut hash = HashMap::new();
        for key in fields.iter() {
            hash.insert(key.to_string(), "".to_string());
        }
        Custom {
            name: self.name,
            fields: hash,
            template: self.template,
            listener: self.listener,
            observable: self.observable,
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

    pub fn observable(self, observable: Box<Observable<i32>>) -> Custom {
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
            Some(_observable) => ()
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