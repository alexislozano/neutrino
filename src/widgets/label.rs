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
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
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
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
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
        }
    }

    /// Set the text
    pub fn text(self, text: &str) -> Self {
        Label {
            name: self.name,
            text: text.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        Label {
            name: self.name,
            text: self.text,
            listener: self.listener,
            observer: Some(observer),
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
            r#"<div class="label" onclick="{}">{}</div>"#,
            Event::js("click", &self.name, "''"),
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
