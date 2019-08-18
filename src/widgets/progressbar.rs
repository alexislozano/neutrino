use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// # ProgressBar
///
/// A progress bar able to display numbers from 0 to 100.
///
/// ## Fields
/// ```
/// pub struct ProgressBar {
///     name: String,
///     value: u8,
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```
/// let my_progressbar = ProgressBar::new("my_progressbar")
///     .value(55)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct ProgressBar {
    name: String,
    value: u8,
    observer: Option<Box<Observer>>,
    listener: Option<Box<Listener>>,
}

impl ProgressBar {
    /// Create a ProgressBar
    ///
    /// # Default values
    ///
    /// ```
    /// name: name.to_string(),
    /// value: 0,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        ProgressBar {
            name: name.to_string(),
            value: 0,
            observer: None,
            listener: None,
        }
    }

    // Set the value
    pub fn value(self, value: u8) -> Self {
        ProgressBar {
            name: self.name,
            value: value,
            observer: self.observer,
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observer: self.observer,
            listener: Some(listener),
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        ProgressBar {
            name: self.name,
            value: self.value,
            observer: Some(observer),
            listener: self.listener,
        }
    }
}

impl Widget for ProgressBar {
    /// Return the HTML representation
    ///
    /// # Styling
    ///
    /// ```
    /// class = progressbar
    /// class = inner-progressbar
    /// ```
    fn eval(&self) -> String {
        format!(
            r#"<div class="progressbar"><div class="inner-progressbar" style="width: {}%;"></div></div>"#, 
            self.value
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```
    /// update -> self.on_update()
    /// ```
    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        }
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```
    /// value
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].parse::<u8>().unwrap();
            }
        }
    }
}
