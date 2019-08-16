use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;
use std::collections::HashMap;
use strfmt::strfmt;

pub struct Custom {
    name: String,
    fields: HashMap<String, String>,
    template: String,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Custom {
    pub fn new(name: &str) -> Self {
        Custom {
            name: name.to_string(),
            fields: HashMap::new(),
            template: "".to_string(),
            listener: None,
            observer: None,
        }
    }

    pub fn template(self, template: &str) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: template.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: self.listener,
            observer: Some(observer),
        }
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                for (key, value) in observer.observe() {
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
