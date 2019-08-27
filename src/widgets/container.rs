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
///     children: Vec<Box<dyn Widget>>,
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
    children: Vec<Box<dyn Widget>>,
    direction: String,
    position: String,
    alignment: String,
    stretch: String,
}

impl Container {
    /// Create an empty Container
    pub fn new() -> Self {
        Container {
            children: vec![],
            direction: "vertical".to_string(),
            position: "".to_string(),
            alignment: "".to_string(),
            stretch: "".to_string(),
        }
    }

    pub fn direction(self, direction: Direction) -> Self {
        let direction_str = match direction {
            Direction::Horizontal => "direction-horizontal",
            Direction::Vertical => "direction-vertical",
        };
        Container {
            children: self.children,
            direction: direction_str.to_string(),
            position: self.position,
            alignment: self.alignment,
            stretch: self.stretch,
        }
    }

    pub fn position(self, position: Position) -> Self {
        let position_str = match position {
            Position::Center => "position-center",
            Position::Start => "position-start",
            Position::End => "position-end",
            Position::Between => "position-between",
            Position::Around => "position-around",
        };
        Container {
            children: self.children,
            direction: self.direction,
            position: position_str.to_string(),
            alignment: self.alignment,
            stretch: self.stretch,
        }
    }

    pub fn alignment(self, alignment: Alignment) -> Self {
        let alignment_str = match alignment {
            Alignment::Center => "alignment-center",
            Alignment::Start => "alignment-start",
            Alignment::End => "alignment-end",
        };
        Container {
            children: self.children,
            direction: self.direction,
            position: self.position,
            alignment: alignment_str.to_string(),
            stretch: self.stretch,
        }
    }

    pub fn stretch(self) -> Self {
        Container {
            children: self.children,
            direction: self.direction,
            position: self.position,
            alignment: self.alignment,
            stretch: "stretch".to_string(),
        }
    }

    /// Add a widget
    pub fn add(&mut self, widget: Box<dyn Widget>) {
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
        let mut s = format!(
            r#"<div class="container {} {} {} {}">"#, 
            self.position,
            self.direction,
            self.alignment,
            self.stretch,
        );
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

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// ```
    fn on_update(&mut self) {}
}


pub enum Direction {
    Horizontal,
    Vertical,
}

pub enum Position {
    Center,
    Start,
    End,
    Between,
    Around,
}

pub enum Alignment {
    Center,
    Start,
    End,
}