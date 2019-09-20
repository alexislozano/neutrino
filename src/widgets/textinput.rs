use crate::utils::event::Event;
use crate::widgets::widget::Widget;
use crate::utils::style::{scss_to_css, inline_style};

/// # The state of a TextInput
///
/// ## Fields
///
/// ```text
/// value: String
/// input_type: InputType,
/// placeholder: String
/// size: u32
/// disabled: bool
/// stretched: bool
/// style: String
/// ```
pub struct TextInputState {
    value: String,
    input_type: InputType,
    placeholder: String,
    size: u32,
    disabled: bool,
    stretched: bool,
    style: String,
}

impl TextInputState {
    /// Get the value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the input_type
    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    /// Get the placeholder
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// Get the size
    pub fn size(&self) -> u32 {
        self.size
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

    /// Set the value
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    /// Set the input_type
    pub fn set_input_type(&mut self, input_type: InputType) {
        self.input_type = input_type;
    }

    /// Set the placeholder
    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder = placeholder.to_string();
    }

    /// Set the size
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
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

/// # The listener of a TextInput
pub trait TextInputListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut TextInputState);

    /// Function triggered on change event
    fn on_change(&self, state: &TextInputState);
}

/// # A zone where text can be written.
///
/// ## Fields
///
/// ```text
/// name: String
/// state: TextInputState
/// listener: Option<Box<dyn TextInputListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     value: "TextInput".to_string()
///     input_type: InputType::Text
///     placeholder: "".to_string()
///     size: 10
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
/// use neutrino::widgets::textinput::{TextInput, TextInputListener, TextInputState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct Person {
///     name: String,
/// }
///
/// impl Person {
///     fn new() -> Self {
///         Self { name: "Ferris".to_string() }
///     }
///
///     fn name(&self) -> &str {
///         &self.name
///     }
///
///     fn set_name(&mut self, name: &str) {
///         self.name = name.to_string();
///     }
/// }
///
///
/// struct MyTextInputListener {
///     person: Rc<RefCell<Person>>,
/// }
///
/// impl MyTextInputListener {
///    pub fn new(person: Rc<RefCell<Person>>) -> Self {
///        Self { person }
///    }
/// }
///
/// impl TextInputListener for MyTextInputListener {
///     fn on_change(&self, state: &TextInputState) {
///         self.person.borrow_mut().set_name(&state.value());
///     }
///
///     fn on_update(&self, state: &mut TextInputState) {
///         state.set_value(&self.person.borrow().name());
///     }
/// }
///
///
/// fn main() {
///     let person = Rc::new(RefCell::new(Person::new()));
///
///     let my_listener = MyTextInputListener::new(Rc::clone(&person));
///
///     let mut my_textinput = TextInput::new("my_textinput");
///     my_textinput.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct TextInput {
    name: String,
    state: TextInputState,
    listener: Option<Box<dyn TextInputListener>>,
}

impl TextInput {
    /// Create a TextInput
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: TextInputState {
                value: "TextInput".to_string(),
                input_type: InputType::Text,
                placeholder: "".to_string(),
                size: 10,
                disabled: false,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Set the value
    pub fn set_value(&mut self, value: &str) {
        self.state.set_value(value);
    }

    /// Set the input_type
    pub fn set_input_type(&mut self, input_type: InputType) {
        self.state.set_input_type(input_type);
    }

    /// Set the placeholder
    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.state.set_placeholder(placeholder);
    }

    /// Set the size
    pub fn set_size(&mut self, size: u32) {
        self.state.set_size(size);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the disabled flag to true
    pub fn set_disabled(&mut self) {
        self.state.set_disabled(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TextInputListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for TextInput {
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
            r#"<div id="{}" class="textinput {} {}"><input {} type="{}" size="{}" maxlength="{}" placeholder="{}" value="{}" onchange="{}" /></div>"#,
            self.name,
            disabled,
            stretched,
            disabled,
            self.state.input_type().css(),
            self.state.size(),
            self.state.size(),
            self.state.placeholder(),
            self.state.value(),
            Event::change_js(&self.name, "value")
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
        self.state.set_value(value);
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}

pub enum InputType {
    Text,
    Password
}

impl InputType {
    fn css(&self) -> &str {
        match &self {
            InputType::Text => "text",
            InputType::Password => "password",
        }
    }
}