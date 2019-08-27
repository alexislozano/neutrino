use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

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
    text: String,
    disabled: bool,
    listener: Option<Box<dyn Listener>>,
    observer: Option<Box<dyn Observer>>,
    stretch: String,
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
        Button {
            name: name.to_string(),
            text: "Button".to_string(),
            disabled: false,
            listener: None,
            observer: None,
            stretch: "".to_string(),
        }
    }

    /// Set the text
    pub fn text(self, text: &str) -> Self {
        Button {
            name: self.name,
            text: text.to_string(),
            disabled: self.disabled,
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the disabled flag
    pub fn disabled(self, disabled: bool) -> Self {
        Button {
            name: self.name,
            text: self.text,
            disabled: disabled,
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        Button {
            name: self.name,
            text: self.text,
            disabled: self.disabled,
            listener: Some(listener),
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<dyn Observer>) -> Self {
        Button {
            name: self.name,
            text: self.text,
            disabled: self.disabled,
            listener: self.listener,
            observer: Some(observer),
            stretch: self.stretch,
        }
    }

    pub fn stretch(self) -> Self {
        Button {
            name: self.name,
            text: self.text,
            disabled: self.disabled,
            listener: self.listener,
            observer: self.observer,
            stretch: "stretch".to_string(),
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
        let disabled = if self.disabled { "disabled" } else { "" };
        format!(
            r#"<div onmousedown="{}" class="button {} {}">{}</div>"#,
            Event::change_js(&self.name, "''"),
            disabled,
            self.stretch,
            self.text
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
                if source == &self.name {
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
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.text = observer.observe()["text"].to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_disabled() {
        let button = Button::new("button").text("Hello").disabled(true);
        assert_eq!(
            button.eval(), 
            format!(
                r#"<div onmousedown="{}" class="button disabled">Hello</div>"#,
                Event::change_js("button", "''"),
            )
        );
    }

    #[test]
    fn eval_enabled() {
        let button = Button::new("button").text("Hello").disabled(false);
        assert_eq!(
            button.eval(), 
            format!(
                r#"<div onmousedown="{}" class="button ">Hello</div>"#,
                Event::change_js("button", "''"),
            )
        );
    }
}