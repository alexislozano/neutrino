use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct CheckBoxState {
    text: String,
    checked: bool,
    stretched: bool,
}

impl CheckBoxState {
    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the text
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// Get the text
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set the checked flag
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Set the streched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
}

pub trait CheckBoxListener {
    fn on_change(&self, state: &CheckBoxState);
    fn on_update(&self, state: &mut CheckBoxState);
}

/// # Checkbox
///
/// A togglable checkbox with a label.
///
/// ## Fields
///
/// ```text
/// pub struct CheckBox {
///     name: String,
///     checked: bool,
///     text: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_checkbox = CheckBox::new("my_checkbox")
///     .text("Toggle me !")
///     .checked(true)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct CheckBox {
    name: String,
    state: CheckBoxState,
    listener: Option<Box<dyn CheckBoxListener>>,
}

impl CheckBox {
    /// Create a CheckBox
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// checked: false,
    /// text: "CheckBox".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: CheckBoxState {
                text: "CheckBox".to_string(),
                checked: false,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the label
    pub fn set_text(&mut self, text: &str) {
        self.state.set_text(text);
    }

    /// Set the checked flag
    pub fn set_checked(&mut self) {
        self.state.set_checked(true);
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn CheckBoxListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for CheckBox {
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
    /// class = checkbox
    /// class = checkbox-outer [checked]
    /// class = checkbox-inner [checked]
    /// ```
    fn eval(&self) -> String {
        let checked = if self.state.checked() { "checked" } else { "" };
        let stretched = if self.state.stretched() { "checked" } else { "" };
        format!(
            r#"<div class="checkbox {}" onmousedown="{}"><div class="checkbox-outer {}"><div class="checkbox-inner {}"></div></div><label>{}</label></div>"#, 
            stretched,
            Event::change_js(&self.name, "''"), 
            checked, 
            checked, 
            self.state.text,
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.checked = != self.checked
    ///          self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
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

    fn on_change(&mut self, _value: &str) {
        self.state.set_checked(!self.state.checked());
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
