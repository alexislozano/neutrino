use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Label
///
/// ## Fields
///
/// ```text
/// text: String
/// stretched: bool
/// unselectable: bool
/// ```
pub struct LabelState {
    text: String,
    stretched: bool,
    unselectable: bool
}

impl LabelState {
    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the selectable flag
    pub fn unselectable(&self) -> bool {
        self.unselectable
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the selectable flag
    pub fn set_unselectable(&mut self, unselectable: bool) {
        self.unselectable = unselectable;
    }
}

/// # The listener of a Label
pub trait LabelListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut LabelState);
}

/// # An element able to display text
///
/// ## Fields
///
/// ```text
/// name: String
/// state: LabelState
/// listener: Option<Box<dyn LabelListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     text: "Label".to_string()
///     stretched: false,
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::label::{Label, LabelListener, LabelState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct Paragraph {
///     text: String,
/// }
///
/// impl Paragraph {
///     fn new() -> Self {
///         Self { text: "".to_string() }
///     }
///
///     fn text(&self) -> &str {
///         &self.text
///     }
/// }
///
///
/// struct MyLabelListener {
///     paragraph: Rc<RefCell<Paragraph>>,
/// }
///
/// impl MyLabelListener {
///    pub fn new(paragraph: Rc<RefCell<Paragraph>>) -> Self {
///        Self { paragraph }
///    }
/// }
///
/// impl LabelListener for MyLabelListener {
///     fn on_update(&self, state: &mut LabelState) {
///         state.set_text(self.paragraph.borrow().text());
///     }
/// }
///
///
/// fn main() {
///     let paragraph = Rc::new(RefCell::new(Paragraph::new()));
///
///     let my_listener = MyLabelListener::new(Rc::clone(&paragraph));
///
///     let mut my_label = Label::new("my_label");
///     my_label.set_text("Hello world!");
///     my_label.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Label {
    name: String,
    state: LabelState,
    listener: Option<Box<dyn LabelListener>>,
}

impl Label {
    /// Create a Label
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: LabelState {
                text: "Label".to_string(),
                stretched: false,
                unselectable: false,
            },
            listener: None,
        }
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.state.set_text(text);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the unselectable flag to true
    pub fn set_unselectable(&mut self) {
        self.state.set_unselectable(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn LabelListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Label {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let selectable_class = if self.state.unselectable() {
            "unselectable"
        } else {
            "selectable"
        };
        format!(
            r#"<div id="{}" class="label {} {}">{}</div>"#,
            self.name,
            stretched,
            selectable_class,
            self.state.text()
        )
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
