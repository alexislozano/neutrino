use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

pub struct CheckBox {
    name: String,
    checked: bool,
    text: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
}

impl CheckBox {
    pub fn new(name: &str) -> Self {
        CheckBox { 
            name: name.to_string(),
            checked: false, 
            text: "CheckBox".to_string(),
            listener: None,
            observable: None,
        }
    }

    pub fn checked(self, checked: bool) -> Self {
        CheckBox { 
            name: self.name,
            checked: checked, 
            text: self.text,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn text(self, text: &str) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked, 
            text: text.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked,
            text: self.text, 
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked,
            text: self.text, 
            listener: self.listener,
            observable: Some(observable),
        }
    }

    pub fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                let hash = observable.observe();
                self.text = hash["text"].to_string();
                self.checked = hash["checked"].parse().unwrap();
            }
        }
    }
}

impl Widget for CheckBox {
    fn eval(&self) -> String {
        let checked = if self.checked {
            "checked"
        } else {
            ""
        };
        format!(
            r#"<div class="checkbox" onclick="{}"><div class="checkbox-outer {}"><div class="checkbox-inner {}"></div></div><label>{}</label></div>"#, 
            Event::js("click", &self.name, "''"), checked, checked, self.text
        )
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "click" {
                self.checked = !self.checked;
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_click();
                    }
                } 
            }
        };
    }
}
