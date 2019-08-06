use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

pub struct Label {
    name: String,
    text: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
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

    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
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
                self.text = observable.observe()["text"].to_string();
            }
        }
    }
}

impl Widget for Label {
    fn eval(&self) -> String {
        format!(
            "<div class=\"label\" onclick=\"(function(){{invoke({{event:'click', source:'{}', value: ''}})}})()\">{}</div>", 
            self.name, self.text
        )
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