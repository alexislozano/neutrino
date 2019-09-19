use crate::utils::event::Event;
use crate::utils::icon::Icon;
use crate::utils::pixmap::Pixmap;
use crate::widgets::widget::Widget;
use crate::utils::style::{scss_to_css, inline_style};

/// # The state of a Button
///
/// ## Fields
///
/// ```text
/// text: Option<String>
/// icon_data: Option<String>
/// icon_extension: Option<String>
/// disabled: bool
/// stretched: bool
/// style: String
/// ```
pub struct ButtonState {
    text: Option<String>,
    icon_data: Option<String>,
    icon_extension: Option<String>,
    disabled: bool,
    stretched: bool,
    style: String,
}

impl ButtonState {
    /// Get the text
    pub fn text(&self) -> Option<&str> {
        self.text.as_ref().map(String::as_ref)
    }

    // Get the icon
    pub fn icon(&self) -> Option<Pixmap> {
        match (&self.icon_data, &self.icon_extension) {
            (Some(data), Some(extension)) => Some(Pixmap::new(data, extension)),
            _ => None,
        }
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
        self.text = Some(text.to_string());
    }

    /// Set the icon
    pub fn set_icon(&mut self, icon: Box<dyn Icon>) {
        let pixmap = Pixmap::from_icon(icon);
        self.icon_data = Some(pixmap.data().to_string());
        self.icon_extension = Some(pixmap.extension().to_string());
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
///     text: None
///     icon_data: None
///     icon_extension: None
///     disabled: false
///     stretched: false
///     style: "".to_string()
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
                text: None,
                icon_data: None,
                icon_extension: None,
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

    /// Set the icon
    pub fn set_icon(&mut self, icon: Box<dyn Icon>) {
        self.state.set_icon(icon);
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

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for Button {
    fn eval(&self) -> String {
        let disabled = if self.state.disabled() {
            "disabled"
        } else {
            ""
        };
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
        let html = match (self.state.text(), self.state.icon()) {
            (Some(text), Some(icon)) => format!(
                r#"<div id="{}" onmousedown="{}" class="button {} {}"><img src="data:image/{};base64,{}" /><span>{}</span></div>"#,
                self.name,
                Event::change_js(&self.name, "''"),
                disabled,
                stretched,
                icon.extension(),
                icon.data(),
                text,
            ),
            (Some(text), None) => format!(
                r#"<div id="{}" onmousedown="{}" class="button {} {}">{}</div>"#,
                self.name,
                Event::change_js(&self.name, "''"),
                disabled,
                stretched,
                text,
            ),
            (None, Some(icon)) => format!(
                r#"<div id="{}" onmousedown="{}" class="button {} {}"><img src="data:image/{};base64,{}" /></div>"#,
                self.name,
                Event::change_js(&self.name, "''"),
                disabled,
                stretched,
                icon.extension(),
                icon.data(),
            ),
            (None, None) => format!(
                r#"<div id="{}" onmousedown="{}" class="button {} {}">{}</div>"#,
                self.name,
                Event::change_js(&self.name, "''"),
                disabled,
                stretched,
                "No text",
            ),
        };
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
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
