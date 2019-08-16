use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;
use crate::widgets::widget::Widget;

pub struct Radio {
    name: String,
    choices: Vec<String>,
    selected: u32,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable>>,
}

impl Radio {
    pub fn new(name: &str) -> Self {
        Radio {
            name: name.to_string(),
            choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
            selected: 0,
            listener: None,
            observable: None,
        }
    }

    pub fn choices(self, choices: Vec<&str>) -> Self {
        Radio {
            name: self.name,
            choices: choices
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
            selected: self.selected,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: selected,
            listener: self.listener,
            observable: self.observable,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    pub fn observable(self, observable: Box<Observable>) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    pub fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(_observable) => {}
        }
    }
}

impl Widget for Radio {
    fn eval(&self) -> String {
        let mut s = "".to_string();
        for (i, choice) in self.choices.iter().enumerate() {
            let selected = if self.selected == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(
                &format!(
                    r#"<div class="radio" onclick="{}"><div class="radio-outer {}"><div class="radio-inner {}"></div></div><label>{}</label></div>"#, 
                    Event::js("click", &self.name, &format!("'{}'", i)), selected, selected, choice
                )
            );
        }
        s
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "click" {
                self.selected = event.value.parse::<u32>().unwrap();
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
