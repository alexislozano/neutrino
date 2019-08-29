use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct TextInputState {
    value: String,
    size: u32,
    stretched: bool,
}

pub trait TextInputListener {
    fn on_update(&self, state: &mut TextInputState);
    fn on_change(&self, state: &TextInputState);
}

/// # TextInput
///
/// A zone where text can be written.
///
/// ## Fields
/// 
/// ```text
/// pub struct TextInput {
///     name: String,
///     value: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_textinput = TextInput::new("my_textinput")
///     .value("Hey")
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct TextInput {
    name: String,
    state: TextInputState,
    listener: Option<Box<dyn TextInputListener>>,
}

impl TextInput {
    /// Create a TextInput
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// value: "TextInput".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: TextInputState {
                value: "TextInput".to_string(),
                size: 10,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the value
    pub fn value(self, value: &str) -> Self {
        Self {
            name: self.name,
            state: TextInputState {
                value: value.to_string(),
                size: self.state.size,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the size
    pub fn size(self, size: u32) -> Self {
        Self {
            name: self.name,
            state: TextInputState {
                value: self.state.value,
                size: size,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the stretched flag
    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: TextInputState {
                value: self.state.value,
                size: self.state.size,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn TextInputListener>) -> Self {
        Self {
            name: self.name,
            state: TextInputState {
                value: self.state.value,
                size: self.state.size,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
        }
    }
}

impl Widget for TextInput {
    /// Return the HTML representation
    ///
    /// # Events
    ///
    /// ```text
    /// change -> value
    /// ```
    ///
    /// # Styling
    ///
    /// ```text
    /// class = textinput
    /// ```
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="textinput {}"><input size="{}" maxlength="{}" value="{}" onchange="{}" /></div>"#,
            stretched,
            self.state.size,
            self.state.size,
            self.state.value,
            Event::change_js(&self.name, "value")
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.value = value
    ///          self.listener.on_click(value)
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value);
                }
            },
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

    fn on_change(&mut self, value: &str) {
        self.state.value = value.to_string();
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
