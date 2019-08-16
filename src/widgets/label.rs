use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct Label {
    name: String,
    text: String,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Label {
    pub fn new(name: &str) -> Self {
        Label {
            name: name.to_string(),
            text: "Label".to_string(),
            listener: None,
            observer: None,
        }
    }

    pub fn text(self, text: &str) -> Self {
        Label {
            name: self.name,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: self.listener,
            observer: Some(observer),
        }
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.text = observer.observe()["text"].to_string();
            }
        }
    }
}

impl Widget for Label {
    fn eval(&self) -> String {
        format!(
            r#"<div class="label" onclick="{}">{}</div>"#,
            Event::js("click", &self.name, "''"),
            self.text
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
