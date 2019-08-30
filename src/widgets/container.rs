use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct ContainerState {
    children: Vec<Box<dyn Widget>>,
    direction: Direction,
    position: Position,
    alignment: Alignment,
}

pub trait ContainerListener {
    fn on_update(&self, state: &mut ContainerState);
}

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
    name: String,
    state: ContainerState,
    listener: Option<Box<dyn ContainerListener>>
}

impl Container {
    /// Create an empty Container
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ContainerState {
                children: vec![],
                direction: Direction::Vertical,
                position: Position::Start,
                alignment: Alignment::None,
            },
            listener: None,
        }
    }

    pub fn direction(self, direction: Direction) -> Self {
        Self {
            name: self.name,
            state: ContainerState {
                children: self.state.children,
                direction: direction,
                position: self.state.position,
                alignment: self.state.alignment,
            },
            listener: self.listener,
        }
    }

    pub fn position(self, position: Position) -> Self {
        Self {
            name: self.name,
            state: ContainerState {
                children: self.state.children,
                direction: self.state.direction,
                position: position,
                alignment: self.state.alignment,
            },
            listener: self.listener,
        }
    }

    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            name: self.name,
            state: ContainerState {
                children: self.state.children,
                direction: self.state.direction,
                position: self.state.position,
                alignment: alignment,
            },
            listener: self.listener,
        }
    }

    /// Add a widget
    pub fn add(&mut self, widget: Box<dyn Widget>) {
        self.state.children.push(widget);
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
            r#"<div class="container {} {} {}">"#, 
            self.state.position.css(),
            self.state.direction.css(),
            self.state.alignment.css(),
        );
        for widget in self.state.children.iter() {
            s.push_str(&widget.eval());
        }
        s.push_str("</div>");
        s
    }

    /// Trigger the trigger function of the children of the Container
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            _ => (),
        };
        for widget in self.state.children.iter_mut() {
            widget.trigger(event);
        }
    }

    fn on_update(&mut self) {
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_update(&mut self.state);
            }
        }
    }

    fn on_change(&mut self, _value: &str) {}
}


pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn css(&self) -> &str {
        match &self {
            Direction::Horizontal => "direction-horizontal",
            Direction::Vertical => "direction-vertical",
        }
    }
}

pub enum Position {
    Center,
    Start,
    End,
    Between,
    Around,
}

impl Position {
    fn css(&self) -> &str {
        match &self {
            Position::Center => "position-center",
            Position::Start => "position-start",
            Position::End => "position-end",
            Position::Between => "position-between",
            Position::Around => "position-around",
        }
    }
}

pub enum Alignment {
    None,
    Stretched,
    Center,
    Start,
    End,
}

impl Alignment {
    fn css(&self) -> &str {
        match &self {
            Alignment::None => "",
            Alignment::Stretched => "stretched",
            Alignment::Center => "alignment-center",
            Alignment::Start => "alignment-start",
            Alignment::End => "alignment-end",
        }
    }
}