use crate::utils::event::Event;
use crate::widgets::widget::Widget;
use crate::utils::style::{scss_to_css, inline_style};

/// # The state of a CheckBox
///
/// ## Fields
///
/// ```text
/// text: String
/// checked: bool
/// disabled: bool
/// stretched: bool
/// style: String
/// ```
pub struct CheckBoxState {
    text: String,
    checked: bool,
    disabled: bool,
    stretched: bool,
    style: String,
}

impl CheckBoxState {
    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the checked flag
    pub fn checked(&self) -> bool {
        self.checked
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

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Set the checked flag
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Set the disabled flag
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set the streched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
    }
}

/// # The listener of a Checkbox
pub trait CheckBoxListener {
    /// Function triggered on change event
    fn on_change(&self, state: &CheckBoxState);

    /// Function triggered on update event
    fn on_update(&self, state: &mut CheckBoxState);
}

/// # A togglable checkbox with a label
///
/// ## Fields
///
/// ```text
/// name: String
/// state: CheckBoxState
/// listener: Option<Box<dyn CheckBoxListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     text: "CheckBox".to_string()
///     checked: false
///     disabled: false
///     stretched: false
///     style: "".to_string()
/// listener: None
/// ```
///
/// ## Style
/// 
/// ```text
/// div.checkbox[.disabled][.checked]
///     div.checkbox-outer
///         div.checkbox-inner
///     label
/// ```
/// 
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::checkbox::{CheckBox, CheckBoxListener, CheckBoxState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct Switch {
///     value: bool,
/// }
///
/// impl Switch {
///     fn new() -> Self {
///         Self { value: false }
///     }
///
///     fn value(&self) -> bool {
///         self.value
///     }
///
///     fn toggle(&mut self) {
///         self.value = !self.value;
///     }
/// }
///
///
/// struct MyCheckBoxListener {
///     switch: Rc<RefCell<Switch>>,
/// }
///
/// impl MyCheckBoxListener {
///    pub fn new(switch: Rc<RefCell<Switch>>) -> Self {
///        Self { switch }
///    }
/// }
///
/// impl CheckBoxListener for MyCheckBoxListener {
///     fn on_change(&self, _state: &CheckBoxState) {
///         self.switch.borrow_mut().toggle();
///     }
///
///     fn on_update(&self, state: &mut CheckBoxState) {
///         state.set_checked(self.switch.borrow().value());
///     }
/// }
///
///
/// fn main() {
///     let switch = Rc::new(RefCell::new(Switch::new()));
///
///     let my_listener = MyCheckBoxListener::new(Rc::clone(&switch));
///
///     let mut my_checkbox = CheckBox::new("my_checkbox");
///     my_checkbox.set_text("Toggle me !");
///     my_checkbox.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct CheckBox {
    name: String,
    state: CheckBoxState,
    listener: Option<Box<dyn CheckBoxListener>>,
}

impl CheckBox {
    /// Create a CheckBox
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: CheckBoxState {
                text: "CheckBox".to_string(),
                checked: false,
                disabled: false,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Set the text
    pub fn set_text(&mut self, text: &str) {
        self.state.set_text(text);
    }

    /// Set the checked flag to true
    pub fn set_checked(&mut self) {
        self.state.set_checked(true);
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
    pub fn set_listener(&mut self, listener: Box<dyn CheckBoxListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for CheckBox {
    fn eval(&self) -> String {
        let checked = if self.state.checked() { "checked" } else { "" };
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
            r#"<div id="{}" class="checkbox {} {} {}" onmousedown="{}"><div class="checkbox-outer"><div class="checkbox-inner"></div></div><label>{}</label></div>"#, 
            self.name,
            disabled,
            checked,
            stretched,
            Event::change_js(&self.name, "''"), 
            self.state.text,
        );
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled() {
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

    fn on_change(&mut self, _value: &str) {
        self.state.set_checked(!self.state.checked());
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
