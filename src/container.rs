use crate::widget::Widget;
use crate::utils::Event;

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
        let mut s = format!("{{ type: \"container\", style: \"{}\", children: [", self.style);
        for (i, widget) in self.children.iter().enumerate() {
            s.push_str(&widget.eval());
            if i != self.children.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push_str("] }");
        s
    }

    fn trigger(&mut self, event: &Event) {
        for widget in self.children.iter_mut() {
            widget.trigger(event);
        }
    }
}