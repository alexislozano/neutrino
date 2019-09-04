use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Container
///
/// ## Fields
///
/// ```text
/// children: Vec<Box<dyn Widget>>
/// direction: Direction
/// position: Position
/// alignment: Alignment
/// ```
pub struct ContainerState {
    children: Vec<Box<dyn Widget>>,
    direction: Direction,
    position: Position,
    alignment: Alignment,
    stretched: bool,
}

impl ContainerState {
    /// Get the children
    pub fn children(&self) -> &Vec<Box<dyn Widget>> {
        &self.children
    }

    /// Get the direction
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    /// Get the position
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Get the alignment
    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the children
    pub fn set_children(&mut self, children: Vec<Box<dyn Widget>>) {
        self.children = children;
    }

    /// Set the direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Set the position
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Set the alignment
    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Add a child
    fn add(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

/// # The listener of a Container
pub trait ContainerListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut ContainerState);
}

/// # A container for other widgets
///
/// ## Fields
///
/// ```text
/// name: String
/// state: ContainerState
/// listener: Option<Box<dyn ContainerListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     children: vec![]
///     direction: Direction::Vertical
///     position: Position::Start
///     alignment: Alignment::None
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::container::{Container, ContainerListener, ContainerState};
/// use neutrino::widgets::label::Label;
/// use neutrino::widgets::widget::Widget;
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct Quotes {
///     values: Vec<String>,
/// }
///
/// impl Quotes {
///     fn new() -> Self {
///         Self { values: vec![] }
///     }
///
///     fn values(&self) -> &Vec<String> {
///         &self.values
///     }
/// }
///
///
/// struct MyContainerListener {
///     quotes: Rc<RefCell<Quotes>>,
/// }
///
/// impl MyContainerListener {
///    pub fn new(quotes: Rc<RefCell<Quotes>>) -> Self {
///        Self { quotes }
///    }
/// }
///
/// impl ContainerListener for MyContainerListener {
///     fn on_update(&self, state: &mut ContainerState) {
///         let labels = self.quotes.borrow().values().iter().enumerate().map(|(i, q)| {
///             let name = format!("quote-{}", i);
///             let mut w = Label::new(&name);
///             w.set_text(q);
///             let b: Box<dyn Widget> = Box::new(w);
///             b
///         }).collect::<Vec<Box<dyn Widget>>>();
///         state.set_children(labels);
///     }
/// }
///
///
/// fn main() {
///     let quotes = Rc::new(RefCell::new(Quotes::new()));
///
///     let my_listener = MyContainerListener::new(Rc::clone(&quotes));
///
///     let mut my_container = Container::new("my_container");
/// }
/// ```
pub struct Container {
    name: String,
    state: ContainerState,
    listener: Option<Box<dyn ContainerListener>>,
}

impl Container {
    /// Create a Container
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ContainerState {
                children: vec![],
                direction: Direction::Vertical,
                position: Position::Start,
                alignment: Alignment::None,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.state.set_direction(direction);
    }

    /// Set the position
    pub fn set_position(&mut self, position: Position) {
        self.state.set_position(position);
    }

    /// Set the alignment
    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.state.set_alignment(alignment);
    }

    /// Set the stretched flag to true. Alignment needs to be set to
    /// Alignment::None (default) for the Container to stretch.
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Add a widget
    pub fn add(&mut self, widget: Box<dyn Widget>) {
        self.state.add(widget);
    }
}

impl Widget for Container {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let mut s = format!(
            r#"<div id="{}" class="container {} {} {} {}">"#,
            self.name,
            self.state.position().css(),
            self.state.direction().css(),
            self.state.alignment().css(),
            stretched,
        );
        for widget in self.state.children.iter() {
            s.push_str(&widget.eval());
        }
        s.push_str("</div>");
        s
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value)
                }
            }
            _ => (),
        }
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

/// # The direction of a Container
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    // Return the CSS class corresponding to the direction
    fn css(&self) -> &str {
        match &self {
            Direction::Horizontal => "direction-horizontal",
            Direction::Vertical => "direction-vertical",
        }
    }
}

/// # The position of the elements inside of a Container
///
/// The position is defined on the direction axis.
///
/// ## Example
///
/// ```text
/// Direction::Horizontal
/// Position::Start
///
/// +-----------------------------+
/// | +--------+ +--------+       |
/// | | widget | | widget |       |
/// | +--------+ +--------+       |
/// +-----------------------------+
/// ```
pub enum Position {
    Center,
    Start,
    End,
    Between,
    Around,
}

impl Position {
    // Return the CSS class corresponding to the position
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

/// # The alignment of a Container
///
/// The alignment is defined on the perpendicular axis of the direction axis.
///
/// ## Example
///
/// ```text
/// Direction::Vertical
/// Alignement::Center
///
/// +----------------------+
/// |      +--------+      |
/// |      | widget |      |
/// |      +--------+      |
/// |      +--------+      |
/// |      | widget |      |
/// |      +--------+      |
/// +----------------------+
/// ```
pub enum Alignment {
    None,
    Center,
    Start,
    End,
}

impl Alignment {
    // Return the CSS class corresponding to the alignment
    fn css(&self) -> &str {
        match &self {
            Alignment::None => "",
            Alignment::Center => "alignment-center",
            Alignment::Start => "alignment-start",
            Alignment::End => "alignment-end",
        }
    }
}
