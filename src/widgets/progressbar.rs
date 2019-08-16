use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct ProgressBar {
    name: String,
    value: u8,
    observer: Option<Box<Observer>>,
    listener: Option<Box<Listener>>,
}

impl ProgressBar {
    pub fn new(name: &str) -> Self {
        ProgressBar {
            name: name.to_string(),
            value: 0,
            observer: None,
            listener: None,
        }
    }

    pub fn value(self, value: u8) -> Self {
        ProgressBar {
            name: self.name,
            value: value,
            observer: self.observer,
            listener: self.listener,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observer: self.observer,
            listener: Some(listener),
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observer: Some(observer),
            listener: self.listener,
        }
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].parse::<u8>().unwrap();
            }
        }
    }
}

impl Widget for ProgressBar {
    fn eval(&self) -> String {
        format!(
            r#"<div class="progressbar"><div class="inner-progressbar" style="width: {}%;"></div></div>"#, 
            self.value
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
