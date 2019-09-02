use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a TextInput
/// 
/// ## Fields
/// 
/// ```text
/// value: String
/// size: u32
/// stretched: bool
/// ```
pub struct TextInputState {
    value: String,
    size: u32,
    stretched: bool,
}

impl TextInputState {
    /// Get the value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the size
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the value
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    /// Set the size
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
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
///     size: 10
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
                size: 10,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the value
    pub fn set_value(&mut self, value: &str) {
        self.state.set_value(value);
    }

    /// Set the size
    pub fn set_size(&mut self, size: u32) {
        self.state.set_size(size);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TextInputListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for TextInput {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div id="{}" class="textinput {}"><input size="{}" maxlength="{}" value="{}" onchange="{}" /></div>"#,
            self.name,
            stretched,
            self.state.size(),
            self.state.size(),
            self.state.value(),
            Event::change_js(&self.name, "value")
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
        self.state.set_value(value);
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
