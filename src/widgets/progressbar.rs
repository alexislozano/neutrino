use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct ProgressBarState {
    value: u8,
    stretched: bool,
}

impl ProgressBarState {
    pub fn set_value(&mut self, value: u8) {
        self.value = if value > 100 {
            100
        } else {
            value
        };
    }
}

pub trait ProgressBarListener {
    fn on_update(&self, state: &mut ProgressBarState);
}

/// # ProgressBar
///
/// A progress bar able to display numbers from 0 to 100.
///
/// ## Fields
/// 
/// ```text
/// pub struct ProgressBar {
///     name: String,
///     value: u8,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_progressbar = ProgressBar::new("my_progressbar")
///     .value(55)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct ProgressBar {
    name: String,
    state: ProgressBarState,
    listener: Option<Box<dyn ProgressBarListener>>,
}

impl ProgressBar {
    /// Create a ProgressBar
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// value: 0,
    /// listener: None,
    /// observer: None,
    /// ```
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
    pub fn value(self, value: u8) -> Self {
        let mut state = ProgressBarState {
            value: 0,
            stretched: self.state.stretched,
        };
        state.set_value(value);
        Self {
            name: self.name,
            state: state,
            listener: self.listener,
        }
    }

    // Set the stretched flag
    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: ProgressBarState {
                value: self.state.value,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn ProgressBarListener>) -> Self {
        Self {
            name: self.name,
            state: ProgressBarState {
                value: self.state.value,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
        }
    }
}

impl Widget for ProgressBar {
    /// Return the HTML representation
    ///
    /// # Styling
    ///
    /// ```text
    /// class = progressbar
    /// class = inner-progressbar
    /// ```
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="progressbar {}"><div class="inner-progressbar" style="width: {}%;"></div></div>"#, 
            stretched,
            self.state.value
        )
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
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
