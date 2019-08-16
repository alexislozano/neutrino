use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

pub struct Tabs {
    name: String,
    children: Vec<(String, Box<Widget>)>,
    selected: u32,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Tabs {
    pub fn new(name: &str) -> Self {
        Tabs {
            name: name.to_string(),
            children: vec![],
            selected: 0,
            listener: None,
            observer: None,
        }
    }

    pub fn selected(self, selected: u32) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: selected,
            listener: self.listener,
            observer: self.observer,
        }
    }

    pub fn listener(self, listener: Box<Listener>) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: self.selected,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    pub fn observer(self, observer: Box<Observer>) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: self.selected,
            listener: self.listener,
            observer: Some(observer),
        }
    }

    pub fn add(&mut self, child: (&str, Box<Widget>)) {
        self.children.push((child.0.to_string(), child.1));
    }

    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(_observer) => {}
        }
    }
}

impl Widget for Tabs {
    fn eval(&self) -> String {
        let mut s = r#"<div class="tabs"><div class="tab-titles">"#.to_string();
        for (i, child) in self.children.iter().enumerate() {
            let selected = if self.selected == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(&format!(
                r#"<div class="tab-title {}" onclick="{}">{}</div>"#,
                selected,
                Event::js("click", &self.name, &format!("'{}'", i)),
                child.0
            ));
        }
        s.push_str(&format!(
            r#"</div><div class="tab">{}</div>"#,
            self.children[self.selected as usize].1.eval()
        ));
        s.push_str("</div>");
        s
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "click" {
                let selected = event.value.parse::<i32>().unwrap();
                if selected > -1 {
                    self.selected = selected as u32;
                }
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_click();
                    }
                }
            }
        } else {
            self.children[self.selected as usize].1.trigger(event);
        };
    }
}
