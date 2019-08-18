use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// # Range
///
/// A progress bar showing a range instead of showing only values between 0 and 
/// 100.
///
/// ## Fields
/// 
/// ```text
/// pub struct Range {
///     name: String,
///     min: i32,
///     max: i32,
///     value: i32,
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_range = Range::new("my_range")
///     .min(-50)
///     .max(50)
///     .value(10)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Range {
    name: String,
    min: i32,
    max: i32,
    value: i32,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Range {
    /// Create a Range
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// min: 0,
    /// max : 100,
    /// value: 0,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Range {
            name: name.to_string(),
            min: 0,
            max: 100,
            value: 0,
            observer: None,
            listener: None,
        }
    }

    /// Set the min
    pub fn min(self, min: i32) -> Self {
        Range {
            name: self.name,
            min: min,
            max: self.max,
            value: self.value,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the max
    pub fn max(self, max: i32) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: max,
            value: self.value,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the value
    pub fn value(self, value: i32) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: value,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        Range {
            name: self.name,
            min: self.min,
            max: self.max,
            value: self.value,
            listener: self.listener,
            observer: Some(observer),
        }
    }
}

impl Widget for Range {
    /// Return the HTML representation
    ///
    /// # Events
    ///
    /// ```text
    /// change -> value
    /// ```
    ///
    /// # Styling
    ///
    /// ```text
    /// class = range
    /// class = inner-range
    /// ```
    fn eval(&self) -> String {
        format!(
            r#"<div class="range"><input oninput="{}" type="range" min="{}" max="{}" value="{}" class="inner-range"></div>"#, 
            Event::js("change", &self.name, "value"), self.min, self.max, self.value
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// change -> self.value = value
    ///           self.listener.on_change(value)
    /// ```
    fn trigger(&mut self, event: &Event) {
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            if event.event == "change" {
                self.value = event.value.parse::<i32>().unwrap();
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_change(&event.value);
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
    /// value
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                self.value = observer.observe()["value"].parse::<i32>().unwrap();
            }
        }
    }
}
