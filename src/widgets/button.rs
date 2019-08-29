use crate::utils::event::Event;
use crate::widgets::widget::Widget;

struct ButtonState {
    text: String,
    disabled: bool,
    stretched: bool,
}

trait ButtonListener {
    fn on_change(&self, state: &ButtonState);
    fn on_update(&self, state: &mut ButtonState);
}

/// # Button
///
/// A clickable button with a label.
///
/// ## Fields
/// 
/// ```text
/// pub struct Button {
///     name: String,
///     text: String,
///     disabled: bool,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_button = Button::new("my_button")
///     .text("Click me !")
///     .disabled(true)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Button {
    name: String,
    state: ButtonState,
    listener: Option<Box<dyn ButtonListener>>,
}

impl Button {
    /// Create a Button
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// text: "Button".to_string(),
    /// disabled: false,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ButtonState {
                text: "Button".to_string(),
                disabled: false,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the text
    pub fn text(self, text: &str) -> Self {
        Self {
            name: self.name,
            state: ButtonState {
                text: text.to_string(),
                disabled: self.state.disabled,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the disabled flag
    pub fn disabled(self) -> Self {
        Self {
            name: self.name,
            state: ButtonState {
                text: self.state.text,
                disabled: true,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: ButtonState {
                text: self.state.text,
                disabled: self.state.disabled,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn ButtonListener>) -> Self {
        Self {
            name: self.name,
            state: self.state,
            listener: Some(listener),
        }
    }
}

impl Widget for Button {
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
    /// class = button [disabled]
    /// ```
    fn eval(&self) -> String {
        let disabled = if self.state.disabled { "disabled" } else { "" };
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div onmousedown="{}" class="button {} {}">{}</div>"#,
            Event::change_js(&self.name, "''"),
            disabled,
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
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled {
                    self.on_change(value)
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

    fn on_change(&self, _value: String) {
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}