use crate::utils::event::Event;
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::widget::Widget;

/// # The state of a Range
///
///  ## Fields
///
/// ```text
/// min: i32
/// max: i32
/// value: i32
/// disabled: bool
/// stretched: bool
/// style: String
/// ```
pub struct RangeState {
    min: i32,
    max: i32,
    value: i32,
    disabled: bool,
    stretched: bool,
    style: String,
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

    /// Get the disabled flag
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the style
    pub fn style(&self) -> &str {
        &self.style
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

    /// Set the disabled flag
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
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
///     disabled: false
///     stretched: false
///     style: "".to_string()
/// listener: None
/// ```
///
/// ## Style
///
/// ```text
/// div.range[.disabled]
///     div.inner-range
///         ::-webkit-slider-runnable-track
///         ::-webkit-slider-thumb
///         ::-ms-track
///         ::-ms-thumb
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
                disabled: false,
                stretched: false,
                style: "".to_string(),
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

    /// Set the disabled flag to true
    pub fn set_disabled(&mut self) {
        self.state.set_disabled(true);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn RangeListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for Range {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let disabled = if self.state.disabled() {
            "disabled"
        } else {
            ""
        };
        let style = inline_style(&scss_to_css(&format!(
            r##"#{}{{{}}}"##,
            self.name,
            self.state.style(),
        )));
        let html = format!(
            r#"
            <div id="{}" class="range {} {}">
                <input {} onchange="{}" oninput="{}" type="range" 
                    min="{}" max="{}" value="{}" class="inner-range"
                >
            </div>
            "#,
            self.name,
            disabled,
            stretched,
            disabled,
            Event::change_js(&self.name, "value"),
            Event::change_js(&self.name, "value"),
            self.state.min(),
            self.state.max(),
            self.state.value(),
        );
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled() {
                    self.on_change(value);
                }
            }
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
