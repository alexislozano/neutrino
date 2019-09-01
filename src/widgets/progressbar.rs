use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a ProgressBar
/// 
/// ## Fields
/// 
/// ```text
/// value: u8
/// stretched: bool
/// ```
pub struct ProgressBarState {
    value: u8,
    stretched: bool,
}

impl ProgressBarState {
    /// Get the value
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Get the stretched flag 
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the value between 0 and 100
    pub fn set_value(&mut self, value: u8) {
        self.value = if value > 100 {
            100
        } else {
            value
        };
    }

    /// Set the stretched flqg
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
}

/// # The listener of a ProgressBar
pub trait ProgressBarListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut ProgressBarState);
}

/// # A progress bar able to display numbers from 0 to 100
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
/// use neutrino::widgets::progressbar::{ProgressBar, ProgressBarListener, ProgressBarState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
/// 
/// 
/// struct Counter {
///     value: u8,
/// }
/// 
/// impl Counter {
///     fn new() -> Self {
///         Self { value: 0 }
///     }
/// 
///     fn value(&self) -> u8 {
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
                value: 0,
                stretched: false,
            },
            listener: None,
        }
    }

    // Set the value
    pub fn set_value(&mut self, value: u8) {
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
}

impl Widget for ProgressBar {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div class="progressbar {}"><div class="inner-progressbar" style="width: {}%;"></div></div>"#, 
            stretched,
            self.state.value()
        )
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value)
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

    fn on_change(&mut self, _value: &str) {}
}
