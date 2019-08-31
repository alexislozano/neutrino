use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct LabelState {
    text: String,
    stretched: bool,
}

impl LabelState {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn stretched(&self) -> bool {
        self.stretched
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
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
    pub fn set_text(&mut self, text: &str) {
        self.state.set_text(text);
    }

    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn LabelListener>) {
        self.listener = Some(listener);
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
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div class="label {}">{}</div>"#,
            stretched,
            self.state.text()
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
