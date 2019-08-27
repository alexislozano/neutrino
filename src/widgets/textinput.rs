use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

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
    value: String,
    listener: Option<Box<dyn Listener>>,
    observer: Option<Box<dyn Observer>>,
    stretch: String,
    size: u32,
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
        TextInput {
            name: name.to_string(),
            value: "TextInput".to_string(),
            listener: None,
            observer: None,
            stretch: "".to_string(),
            size: 20,
        }
    }

    /// Set the value
    pub fn value(self, value: &str) -> Self {
        TextInput {
            name: self.name,
            value: value.to_string(),
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
            size: self.size,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: Some(listener),
            observer: self.observer,
            stretch: self.stretch,
            size: self.size,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<dyn Observer>) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: self.listener,
            observer: Some(observer),
            stretch: self.stretch,
            size: self.size,
        }
    }

    pub fn stretch(self) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: self.listener,
            observer: self.observer,
            stretch: "stretch".to_string(),
            size: self.size,
        }
    }

    pub fn size(self, size: u32) -> Self {
        TextInput {
            name: self.name,
            value: self.value,
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
            size: size,
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
        format!(
            r#"<div class="textinput {}"><input size="{}" maxlength="{}" value="{}" onchange="{}" /></div>"#,
            self.stretch,
            self.size,
            self.size,
            self.value,
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
                    self.value = value.to_string();
                    match &self.listener {
                        None => (),
                        Some(listener) => {
                            listener.on_change(value);
                        }
                    }
                }
            },
            _ => (),
        }
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// value
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].to_string();
            }
        }
    }
}
