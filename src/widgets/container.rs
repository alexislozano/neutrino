use crate::widgets::widget::Widget;
use crate::utils::event::Event;

pub struct Container {
    children: Vec<Box<Widget>>,
    style: String,
}

impl Container {
    pub fn vertical() -> Self {
        Container { 
            children: vec![], 
            style: "flex-direction: column;".to_string() 
        }
    }

    pub fn horizontal() -> Self {
        Container { 
            children: vec![], 
            style: "flex-direction: row;".to_string()
        }
    }

    pub fn add(&mut self, widget: Box<Widget>) {
        self.children.push(widget);
    }
}

impl Widget for Container {
    fn eval(&self) -> String {
        let mut s = format!(
            r#"<div class="container" style="{}">"#, 
            self.style
        );
        for widget in self.children.iter() {
            s.push_str(&widget.eval());
        }
        s.push_str("</div>");
        s
    }

    fn trigger(&mut self, event: &Event) {
        for widget in self.children.iter_mut() {
            widget.trigger(event);
        }
    }
}