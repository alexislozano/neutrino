use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observable::Observable;

/// A clickable widget
/// 
/// # Fields
/// ```
/// pub struct Button {
///    name: String,
///    text: String,
///    disabled: bool,
///    listener: Option<Box<Listener>>,
///    observable: Option<Box<Observable<String>>>,
/// }
/// ```
/// 
/// # Example
/// 
/// ```
/// let my_button = Button::new("my_button")
///     .text("Click me !")
///     .disabled(true)
///     .listener(Box::new(my_button_listener))
///     .observable(Box::new(my_button_observable));
/// ```
pub struct Button {
    name: String,
    text: String,
    disabled: bool,
    listener: Option<Box<Listener>>,
    observable: Option<Box<Observable<String>>>,
}

impl Button {
    /// Create a Button
    /// 
    /// # Default values
    /// 
    /// ```
    /// name: name.to_string(),
    /// text: "Button".to_string(), 
    /// disabled: false,
    /// listener: None,
    /// observable: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Button { 
            name: name.to_string(),
            text: "Button".to_string(), 
            disabled: false,
            listener: None,
            observable: None,
        }
    }

    /// Set the text of a Button
    pub fn text(self, text: &str) -> Self {
        Button { 
            name: self.name,
            text: text.to_string(), 
            disabled: self.disabled,
            listener: self.listener,
            observable: self.observable,
        }
    }

    /// Set the disabled flag of a Button
    pub fn disabled(self, disabled: bool) -> Self {
        Button { 
            name: self.name,
            text: self.text, 
            disabled: disabled,
            listener: self.listener,
            observable: self.observable, 
        }
    }

    /// Set the listener of a Button
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Button { 
            name: self.name,
            text: self.text, 
            disabled: self.disabled,
            listener: Some(listener),
            observable: self.observable,
        }
    }

    /// Set the observable of a Button
    pub fn observable(self, observable: Box<Observable<String>>) -> Self {
        Button { 
            name: self.name,
            text: self.text, 
            disabled: self.disabled,
            listener: self.listener,
            observable: Some(observable),
        }
    }

    pub fn on_update(&mut self) {
        match &self.observable {
            None => (),
            Some(observable) => {
                self.text = observable.observe()["text"].to_string();
            }
        }
    }
}

impl Widget for Button {
    fn eval(&self) -> String {
        let disabled = if self.disabled {
            "disabled"
        } else {
            ""
        };
        format!(
            r#"<div onclick="{}" class="button {}">{}</div>"#, 
            Event::js("click", &self.name, "''"), disabled, self.text
        )
    }

    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            match &self.listener {
                None => (),
                Some(listener) => {
                    if event.event == "click" {
                        listener.on_click();
                    }
                }
            } 
        };
    }
}
