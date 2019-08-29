use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct LabelState {
    text: String,
    stretched: bool,
}

impl LabelState {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

pub trait LabelListener {
    fn on_update(&self, state: &mut LabelState);
}

/// # Label
///
/// A label.
///
/// ## Fields
/// 
/// ```text
/// pub struct Label {
///     name: String,
///     text: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_label = Label::new("my_label")
///     .text("Text")
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Label {
    name: String,
    state: LabelState,
    listener: Option<Box<dyn LabelListener>>,
}

impl Label {
    /// Create a Label
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// text: "Label".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: LabelState { 
                text: "Label".to_string(),
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the text
    pub fn text(self, text: &str) -> Self {
        Self {
            name: self.name,
            state: LabelState { 
                text: text.to_string(),
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretch(self) -> Self {
        Self {
            name: self.name,
            state: LabelState { 
                text: self.state.text,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn LabelListener>) -> Self {
        Self {
            name: self.name,
            state: LabelState { 
                text: self.state.text,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
        }
    }
}

impl Widget for Label {
    /// Return the HTML representation
    ///
    /// # Events
    ///
    /// ```text
    /// click -> ""
    /// ```
    ///
    /// # Styling
    ///
    /// ```text
    /// class = label
    /// ```
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="label {}">{}</div>"#,
            stretched,
            self.state.text
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
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
