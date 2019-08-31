use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct TextInputState {
    value: String,
    size: u32,
    stretched: bool,
}

impl TextInputState {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn stretched(&self) -> bool {
        self.stretched
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
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
    pub fn set_value(&mut self, value: &str) {
        self.state.set_value(value);
    }

    /// Set the size
    pub fn set_size(&mut self, size: u32) {
        self.state.set_size(size);
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TextInputListener>) {
        self.listener = Some(listener);
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
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div class="textinput {}"><input size="{}" maxlength="{}" value="{}" onchange="{}" /></div>"#,
            stretched,
            self.state.size(),
            self.state.size(),
            self.state.value(),
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
        self.state.set_value(value);
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
