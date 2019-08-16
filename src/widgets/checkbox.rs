use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// Checkbox
///
/// # Fields
/// ```
/// pub struct CheckBox {
///    name: String,
///    checked: bool,
///    text: String,
///    listener: Option<Box<Listener>>,
///    observer: Option<Box<Observer>>,
/// }
/// ```
///
/// # Example
///
/// ```
/// let my_checkbox = CheckBox::new("my_checkbox")
///     .text("Toggle me !")
///     .checked(true)
///     .listener(Box::new(my_checkbox_listener))
///     .observer(Box::new(my_checkbox_observer));
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
    /// ```
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

    /// Set the checked flag of a CheckBox
    pub fn checked(self, checked: bool) -> Self {
        CheckBox {
            name: self.name,
            checked: checked,
            text: self.text,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the label of a CheckBox
    pub fn text(self, text: &str) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener of a CheckBox
    pub fn listener(self, listener: Box<Listener>) -> Self {
        CheckBox {
            name: self.name,
            checked: self.checked,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer of a CheckBox
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
    /// Return the HTML representation of a CheckBox
    ///
    /// # Events
    ///
    /// ```
    /// click -> ""
    /// ```
    ///
    /// # Styling
    ///
    /// ```
    /// class = checkbox
    /// class = checkbox-outer [checked]
    /// class = checkbox-inner [checked]
    /// ```
    ///
    fn eval(&self) -> String {
        let checked = if self.checked { "checked" } else { "" };
        format!(
            r#"<div class="checkbox" onclick="{}"><div class="checkbox-outer {}"><div class="checkbox-inner {}"></div></div><label>{}</label></div>"#, 
            Event::js("click", &self.name, "''"), checked, checked, self.text
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```
    /// update -> self.on_update()
    /// click -> self.checked = != self.checked
    ///          self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "click" {
                self.checked = !self.checked;
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_click();
                    }
                }
            }
        };
    }

    /// Set the values of the widget using the fields of the HashMap
    /// defining the model
    ///
    /// # Fields
    ///
    /// ```
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
