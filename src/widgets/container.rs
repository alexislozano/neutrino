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
    direction: String,
    position: String,
    alignment: String,
}

impl Container {
    /// Create an empty Container
    pub fn new() -> Self {
        Container {
            children: vec![],
            direction: "flex-direction: column;".to_string(),
            position: "".to_string(),
            alignment: "".to_string(),
        }
    }

    pub fn direction(self, direction: Direction) -> Self {
        let direction_str = match direction {
            Direction::Horizontal => "flex-direction: row;",
            Direction::Vertical => "flex-direction: column;",
        };
        Container {
            children: self.children,
            direction: direction_str.to_string(),
            position: self.position,
            alignment: self.alignment,
        }
    }

    pub fn position(self, position: Position) -> Self {
        let position_str = match position {
            Position::Center => "justify-content: center;",
            Position::Start => "justify-content: flex-start;",
            Position::End => "justify-content: flex-end;",
            Position::SpaceBetween => "justify-content: space-between;",
            Position::SpaceAround => "justify-content: space-around;",
        };
        Container {
            children: self.children,
            direction: self.direction,
            position: position_str.to_string(),
            alignment: self.alignment,
        }
    }

    pub fn alignment(self, alignment: Alignment) -> Self {
        let alignment_str = match alignment {
            Alignment::Center => "align-items: center;",
            Alignment::Start => "align-items: flex-start;",
            Alignment::End => "align-items: flex-end;",
            Alignment::Stretch => "align-items: stretch;",
        };
        Container {
            children: self.children,
            direction: self.direction,
            position: self.position,
            alignment: alignment_str.to_string(),
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
        let mut s = format!(
            r#"<div class="container" style="{}{}{}">"#, 
            self.position,
            self.direction,
            self.alignment,
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
    SpaceBetween,
    SpaceAround,
}

pub enum Alignment {
    Center,
    Start,
    End,
    Stretch,
}