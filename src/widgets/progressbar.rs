use crate::utils::event::Event;
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::widget::Widget;

/// # The state of a ProgressBar
///
/// ## Fields
///
/// ```text
/// min: i32
/// max: i32
/// value: i32
/// stretched: bool
/// style: String
/// ```
pub struct ProgressBarState {
    min: i32,
    max: i32,
    value: i32,
    stretched: bool,
    style: String,
}

impl ProgressBarState {
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
        self.value = if value > self.max {
            self.max
        } else if value < self.min {
            self.min
        } else {
            value
        };
    }

    /// Set the stretched flqg
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
    }
}

/// # The listener of a ProgressBar
pub trait ProgressBarListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut ProgressBarState);
}

/// # A progress bar
///
/// ## Fields
///
/// ```text
/// name: String
/// state: ProgressBarState
/// listener: Option<Box<dyn ProgressBarListener>>
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
///     style: "".to_string()
/// listener: None
/// ```
///
/// ## Style
///
/// ```text
/// div.progressbar
///     div.background
///     div.foreground
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::progressbar::{
///     ProgressBar,
///     ProgressBarListener,
///     ProgressBarState
/// };
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
/// }
///
///
/// struct MyProgressBarListener {
///     counter: Rc<RefCell<Counter>>,
/// }
///
/// impl MyProgressBarListener {
///    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
///        Self { counter }
///    }
/// }
///
/// impl ProgressBarListener for MyProgressBarListener {
///     fn on_update(&self, state: &mut ProgressBarState) {
///         state.set_value(self.counter.borrow().value());
///     }
/// }
///
///
/// fn main() {
///     let counter = Rc::new(RefCell::new(Counter::new()));
///
///     let my_listener = MyProgressBarListener::new(Rc::clone(&counter));
///
///     let mut my_progressbar = ProgressBar::new("my_progressbar");
///     my_progressbar.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct ProgressBar {
    name: String,
    state: ProgressBarState,
    listener: Option<Box<dyn ProgressBarListener>>,
}

impl ProgressBar {
    /// Create a ProgressBar
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ProgressBarState {
                min: 0,
                max: 100,
                value: 0,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    // Set the min
    pub fn set_min(&mut self, min: i32) {
        self.state.set_min(min);
    }

    // Set the max
    pub fn set_max(&mut self, max: i32) {
        self.state.set_max(max);
    }

    // Set the value
    pub fn set_value(&mut self, value: i32) {
        self.state.set_value(value);
    }

    // Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ProgressBarListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for ProgressBar {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
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
            <div id="{}" class="progressbar {}">
                <div class="background"></div>
                <div class="foreground" style="width: {}%;"></div>
            </div>
            "#,
            self.name,
            stretched,
            f64::from(self.state.value() - self.state.min())
                / f64::from(self.state.max() - self.state.min())
                * 100.0,
        );
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value)
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

    fn on_change(&mut self, _value: &str) {}
}
