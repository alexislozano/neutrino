use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Button
///
/// ## Fields
/// 
/// ```text
/// text: String
/// disabled: bool
/// stretched: bool
/// ```
pub struct ButtonState {
    text: String,
    disabled: bool,
    stretched: bool,
}

impl ButtonState {
    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the disabled flag
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set the disabled flag
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set the streched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
}

/// # The listener of a Button
pub trait ButtonListener {
    /// Function triggered on change event
    fn on_change(&self, state: &ButtonState);

    /// Function triggered on update event
    fn on_update(&self, state: &mut ButtonState);
}

/// # A clickable button with a label
///
/// ## Fields
/// 
/// ```text
/// name: String
/// state: ButtonState
/// listener: Option<Box<dyn ButtonListener>>
/// ```
/// 
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     text: "Button".to_string()
///     disabled: false
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
/// use neutrino::widgets::button::{Button, ButtonListener, ButtonState};
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
/// 
///     fn increment(&mut self) {
///         self.value += 1;
///     } 
/// }
/// 
/// 
/// struct MyButtonListener {
///     counter: Rc<RefCell<Counter>>,
/// }
/// 
/// impl MyButtonListener {
///    pub fn new(counter: Rc<RefCell<Counter>>) -> Self {
///        Self { counter }
///    }
/// }
/// 
/// impl ButtonListener for MyButtonListener {
///     fn on_change(&self, _state: &ButtonState) {
///         self.counter.borrow_mut().increment();
///     }
/// 
///     fn on_update(&self, state: &mut ButtonState) {
///         state.set_text(&self.counter.borrow().value().to_string());
///     }
/// }
/// 
/// 
/// fn main() {
///     let counter = Rc::new(RefCell::new(Counter::new()));
/// 
///     let my_listener = MyButtonListener::new(Rc::clone(&counter));
/// 
///     let mut my_button = Button::new("my_button");
///     my_button.set_text("Click me !");
///     my_button.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Button {
    name: String,
    state: ButtonState,
    listener: Option<Box<dyn ButtonListener>>,
}

impl Button {
    /// Create a Button
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ButtonState {
                text: "Button".to_string(),
                disabled: false,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.state.set_text(text);
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
    pub fn set_listener(&mut self, listener: Box<dyn ButtonListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Button {
    fn eval(&self) -> String {
        let disabled = if self.state.disabled() { "disabled" } else { "" };
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div onmousedown="{}" class="button {} {}">{}</div>"#,
            Event::change_js(&self.name, "''"),
            disabled,
            stretched,
            self.state.text()
        )
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled() {
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

    fn on_change(&mut self, _value: &str) {
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}