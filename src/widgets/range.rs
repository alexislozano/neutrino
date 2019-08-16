use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct Range {
    name: String,
    min: i32,
    max: i32,
    value: i32,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Range {
    pub fn new(name: &str) -> Self {
        Range {
            name: name.to_string(),
            min: 0,
            max: 100,
            value: 0,
            observer: None,
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
            observer: self.observer,
        }
    }

    pub fn max(self, max: i32) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: max,
            value: self.value,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn value(self, value: i32) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: value,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: self.listener,
            observer: Some(observer),
        }
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].parse::<i32>().unwrap();
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
