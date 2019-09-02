use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Range
/// 
///  ## Fields
/// 
/// ```text
/// min: i32
/// max: i32
/// value: i32
/// stretched: bool
/// ```
pub struct RangeState {
    min: i32,
    max: i32,
    value: i32,
    stretched: bool,
}

impl RangeState {
    /// Get the min
    pub fn min(&self) -> i32 {
        self.min
    }

    /// Get the max
    pub fn max(&self) -> i32 {
        self.max
    }

    /// Get the value
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the min
    pub fn set_min(&mut self, min: i32) {
        self.min = min;
    }

    /// Set the max
    pub fn set_max(&mut self, max: i32) {
        self.max = max;
    }

    /// Set the value
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
}

/// # The listener of a Range
pub trait RangeListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut RangeState);

    /// Function triggered on change event
    fn on_change(&self, state: &RangeState);
}

/// # A progress bar with a handle
///
/// ## Fields
/// 
/// ```text
/// name: String
/// state: RangeState
/// listener: Option<Box<dyn RangeListener>>
/// ```
/// 
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     min: 0
///     max: 100
///     value: 0
///     stretched: false
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// 
/// use neutrino::widgets::range::{Range, RangeListener, RangeState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
/// 
/// 
/// struct Counter {
///     value: i32,
/// }
/// 
/// impl Counter {
///     fn new() -> Self {
///         Self { value: 0 }
///     }
/// 
///     fn value(&self) -> i32 {
///         self.value
///     }
/// 
///     fn set_value(&mut self, value: i32) {
///         self.value = value;
///     }
/// }
/// 
/// 
/// struct MyRangeListener {
///     counter: Rc<RefCell<Counter>>,
/// }
/// 
/// impl MyRangeListener {
///    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
///        Self { counter }
///    }
/// }
/// 
/// impl RangeListener for MyRangeListener {
///     fn on_change(&self, state: &RangeState) {
///         self.counter.borrow_mut().set_value(state.value());
///     }
/// 
///     fn on_update(&self, state: &mut RangeState) {
///         state.set_value(self.counter.borrow().value());
///     }
/// }
/// 
/// 
/// fn main() {
///     let counter = Rc::new(RefCell::new(Counter::new()));
/// 
///     let my_listener = MyRangeListener::new(Rc::clone(&counter));
/// 
///     let mut my_range = Range::new("my_range");
///     my_range.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Range {
    name: String,
    state: RangeState,
    listener: Option<Box<dyn RangeListener>>,
}

impl Range {
    /// Create a Range
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
    pub fn set_min(&mut self, min: i32) {
        self.state.set_min(min);
    }

    /// Set the max
    pub fn set_max(&mut self, max: i32) {
        self.state.set_max(max);
    }

    /// Set the value
    pub fn set_value(&mut self, value: i32) {
        self.state.set_value(value);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn RangeListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Range {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div class="range {}"><input oninput="{}" type="range" min="{}" max="{}" value="{}" class="inner-range"></div>"#, 
            stretched,
            Event::change_js(&self.name, "value"), 
            self.state.min(), 
            self.state.max(), 
            self.state.value(),
        )
    }

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
        self.state.set_value(value.parse::<i32>().unwrap());
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
