use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// # Label
///
/// A label.
///
/// ## Fields
/// 
/// ```text
/// pub struct Label {
///     name: String,
///     text: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_label = Label::new("my_label")
///     .text("Text")
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Label {
    name: String,
    text: String,
    listener: Option<Box<dyn Listener>>,
    observer: Option<Box<dyn Observer>>,
    stretch: String,
}

impl Label {
    /// Create a Label
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// text: "Label".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Label {
            name: name.to_string(),
            text: "Label".to_string(),
            listener: None,
            observer: None,
            stretch: "".to_string(),
        }
    }

    /// Set the text
    pub fn text(self, text: &str) -> Self {
        Label {
            name: self.name,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<dyn Observer>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: self.listener,
            observer: Some(observer),
            stretch: self.stretch,
        }
    }

    pub fn stretch(self) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: self.listener,
            observer: self.observer,
            stretch: "stretch".to_string(),
        }
    }
}

impl Widget for Label {
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
    /// class = label
    /// ```
    fn eval(&self) -> String {
        format!(
            r#"<div class="label {}" onmousedown="{}">{}</div>"#,
            self.stretch,
            Event::change_js(&self.name, "''"),
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
