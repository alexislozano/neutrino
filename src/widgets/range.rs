use crate::utils::event::Event;
use crate::widgets::widget::Widget;

struct RangeState {
    min: i32,
    max: i32,
    value: i32,
    stretched: bool,
}

trait RangeListener {
    fn on_update(&self, state: &mut RangeState);
    fn on_change(&self, state: &RangeState);
}

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
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
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
    state: RangeState,
    listener: Option<Box<dyn RangeListener>>,
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
        Self {
            name: name.to_string(),
            state: RangeState {
                min: 0,
                max: 100,
                value: 0,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the min
    pub fn min(self, min: i32) -> Self {
        Self {
            name: self.name,
            state: RangeState {
                min: min,
                max: self.state.max,
                value: self.state.value,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the max
    pub fn max(self, max: i32) -> Self {
        Self {
            name: self.name,
            state: RangeState {
                min: self.state.min,
                max: max,
                value: self.state.value,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the value
    pub fn value(self, value: i32) -> Self {
        Self {
            name: self.name,
            state: RangeState {
                min: self.state.min,
                max: self.state.max,
                value: value,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: RangeState {
                min: self.state.min,
                max: self.state.max,
                value: self.state.value,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn RangeListener>) -> Self {
        Self {
            name: self.name,
            state: RangeState {
                min: self.state.min,
                max: self.state.max,
                value: self.state.value,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
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
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="range {}"><input oninput="{}" type="range" min="{}" max="{}" value="{}" class="inner-range"></div>"#, 
            stretched,
            Event::change_js(&self.name, "value"), 
            self.state.min, 
            self.state.max, 
            self.state.value,
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
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value);
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

    fn on_change(&mut self, value: &str) {
        self.state.value = value.parse::<i32>().unwrap();
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
