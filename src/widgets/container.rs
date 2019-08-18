use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # Container
///
/// A container for other widgets.
///
/// ## Fields
/// 
/// ```text
/// pub struct Container {
///     children: Vec<Box<Widget>>,
///     style: String,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_container = Container::horizontal();
/// ```
pub struct Container {
    children: Vec<Box<Widget>>,
    style: String,
}

impl Container {
    /// Create an empty vertical Container
    pub fn vertical() -> Self {
        Container {
            children: vec![],
            style: "flex-direction: column;".to_string(),
        }
    }

    /// Create an empty horizontal Container
    pub fn horizontal() -> Self {
        Container {
            children: vec![],
            style: "flex-direction: row;".to_string(),
        }
    }

    /// Add a widget
    pub fn add(&mut self, widget: Box<Widget>) {
        self.children.push(widget);
    }
}

impl Widget for Container {
    /// Return the HTML representation of the Container and of the children of 
    /// the Container
    ///
    /// # Styling
    ///
    /// ```text
    /// class = container
    /// ```
    fn eval(&self) -> String {
        let mut s = format!(r#"<div class="container" style="{}">"#, self.style);
        for widget in self.children.iter() {
            s.push_str(&widget.eval());
        }
        s.push_str("</div>");
        s
    }

    /// Trigger the trigger function of the children of the Container
    fn trigger(&mut self, event: &Event) {
        for widget in self.children.iter_mut() {
            widget.trigger(event);
        }
    }
}
