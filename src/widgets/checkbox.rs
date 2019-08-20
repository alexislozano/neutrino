use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

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
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
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
    checked: bool,
    text: String,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
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
        CheckBox {
            name: name.to_string(),
            checked: false,
            text: "CheckBox".to_string(),
            listener: None,
            observer: None,
        }
    }

    /// Set the checked flag
    pub fn checked(self, checked: bool) -> Self {
        CheckBox {
            name: self.name,
            checked: checked,
            text: self.text,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the label
    pub fn text(self, text: &str) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: self.listener,
            observer: Some(observer),
        }
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
        let checked = if self.checked { "checked" } else { "" };
        format!(
            r#"<div class="checkbox" onclick="{}"><div class="checkbox-outer {}"><div class="checkbox-inner {}"></div></div><label>{}</label></div>"#, 
            Event::change_js(&self.name, "''"), checked, checked, self.text
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
                    self.checked = !self.checked;
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
    /// text
    /// checked
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                let hash = observer.observe();
                self.text = hash["text"].to_string();
                self.checked = hash["checked"].parse().unwrap();
            }
        }
    }
}
