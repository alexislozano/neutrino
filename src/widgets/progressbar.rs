use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;
use crate::widgets::widget::Widget;

pub struct ProgressBar {
    name: String,
    value: u8,
    observable: Option<Box<Observable>>,
    listener: Option<Box<Listener>>,
}

impl ProgressBar {
    pub fn new(name: &str) -> Self {
        ProgressBar {
            name: name.to_string(),
            value: 0,
            observable: None,
            listener: None,
        }
    }

    pub fn value(self, value: u8) -> Self {
        ProgressBar {
            name: self.name,
            value: value,
            observable: self.observable,
            listener: self.listener,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observable: self.observable,
            listener: Some(listener),
        }
    }

    pub fn observable(self, observable: Box<Observable>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observable: Some(observable),
            listener: self.listener,
        }
    }

    fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                self.value = observable.observe()["value"].parse::<u8>().unwrap();
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
