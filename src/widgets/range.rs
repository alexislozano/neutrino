use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::observable::Observable;
use crate::utils::listener::Listener;

pub struct Range {
    name: String,
    min: i32,
    max: i32,
    value: i32,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<i32>>>,
}

impl Range {
    pub fn new(name: &str) -> Self {
        Range { 
            name: name.to_string(),
            min: 0,
            max: 100,
            value: 0,
            observable: None,
            listener: None,
        }
    }

    pub fn min(self, min: i32) -> Self {
        Range { 
            name: self.name,
            min: min,
            max: self.max,
            value: self.value,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn max(self, max: i32) -> Self {
        Range { 
            name: self.name,
            min: self.min,
            max: max,
            value: self.value,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn value(self, value: i32) -> Self {
        Range { 
            name: self.name,
            min: self.min,
            max: self.max,
            value: value,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Range { 
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable<i32>>) -> Self {
        Range { 
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                self.value = observable.observe()["value"];
            }
        }
    }
}

impl Widget for Range {
    fn eval(&self) -> String {
        format!(
            r#"<div class="range"><input oninput="{}" type="range" min="{}" max="{}" value="{}" class="inner-range"></div>"#, 
            Event::js("change", &self.name, "value"), self.min, self.max, self.value
        )
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "change" {
                self.value = event.value.parse::<i32>().unwrap();
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_change(&event.value);
                    }
                }
            } 
        };
    }
}