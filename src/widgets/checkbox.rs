use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

/// Checkbox
/// 
/// # Fields
/// ```
/// pub struct CheckBox {
///    name: String,
///    checked: bool,
///    text: String,
///    listener: Option<Box<Listener>>,
///    observable: Option<Box<Observable<String>>>,
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
///     .observable(Box::new(my_checkbox_observable));
/// ```
pub struct CheckBox {
    name: String,
    checked: bool,
    text: String,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
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
    /// observable: None,
    /// ```
    pub fn new(name: &str) -> Self {
        CheckBox { 
            name: name.to_string(),
            checked: false, 
            text: "CheckBox".to_string(),
            listener: None,
            observable: None,
        }
    }

    /// Set the checked flag of a CheckBox
    pub fn checked(self, checked: bool) -> Self {
        CheckBox { 
            name: self.name,
            checked: checked, 
            text: self.text,
            listener: self.listener,
            observable: self.observable,
        }
    }

    /// Set the label of a CheckBox
    pub fn text(self, text: &str) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked, 
            text: text.to_string(),
            listener: self.listener,
            observable: self.observable,
        }
    }

    /// Set the listener of a CheckBox
    pub fn listener(self, listener: Box<Listener>) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked,
            text: self.text, 
            listener: Some(listener),
            observable: self.observable,
        }
    }

    /// Set the observable of a CheckBox
    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
        CheckBox { 
            name: self.name,
            checked: self.checked,
            text: self.text, 
            listener: self.listener,
            observable: Some(observable),
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
        let checked = if self.checked {
            "checked"
        } else {
            ""
        };
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
        match &self.observable {
            None => (),
            Some(observable) => {
                let hash = observable.observe();
                self.text = hash["text"].to_string();
                self.checked = hash["checked"].parse().unwrap();
            }
        }
    }
}
