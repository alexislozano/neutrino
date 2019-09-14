use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Radio
///
/// ## Fields
///
/// ```text
/// choices: Vec<String>
/// selected: u32
/// disabled: bool
/// stretched: bool
/// ```
pub struct RadioState {
    choices: Vec<String>,
    selected: u32,
    disabled: bool,
    stretched: bool,
}

impl RadioState {
    /// Get the choices
    pub fn choices(&self) -> &Vec<String> {
        &self.choices
    }

    /// Get the selected index
    pub fn selected(&self) -> u32 {
        self.selected
    }

    /// Get the disabled flag
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.choices = choices
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
    }

    /// Set the selected index
    pub fn set_selected(&mut self, selected: u32) {
        self.selected = selected;
    }

    /// Set the disabled flag
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
}

/// # The listener of a Radio
pub trait RadioListener {
    /// Function triggered on change event
    fn on_change(&self, state: &RadioState);

    /// Function triggered on update event
    fn on_update(&self, state: &mut RadioState);
}

/// # A list of radio buttons
///
/// Only one can be selected at a time.
///
/// ## Fields
///
/// ```text
/// name: String
/// state: RadioState
/// listener: Option<Box<dyn RadioListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
///     selected: 0
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
/// use neutrino::widgets::radio::{Radio, RadioListener, RadioState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct Dessert {
///     index: u32,
///     value: String,
/// }
///
/// impl Dessert {
///     fn new() -> Self {
///         Self { index: 0, value: "Cake".to_string() }
///     }
///
///     fn index(&self) -> u32 {
///         self.index
///     }
///
///     fn value(&self) -> &str {
///         &self.value
///     }
///
///     fn set(&mut self, index: u32, value: &str) {
///         self.index = index;
///         self.value = value.to_string();
///     }
/// }
///
///
/// struct MyRadioListener {
///     dessert: Rc<RefCell<Dessert>>,
/// }
///
/// impl MyRadioListener {
///    pub fn new(dessert: Rc<RefCell<Dessert>>) -> Self {
///        Self { dessert }
///    }
/// }
///
/// impl RadioListener for MyRadioListener {
///     fn on_change(&self, state: &RadioState) {
///         let index = state.selected();
///         self.dessert.borrow_mut().set(
///             index,
///             &state.choices()[index as usize]
///         );
///     }
///
///     fn on_update(&self, state: &mut RadioState) {
///         state.set_selected(self.dessert.borrow().index());
///     }
/// }
///
///
/// fn main() {
///     let dessert = Rc::new(RefCell::new(Dessert::new()));
///
///     let my_listener = MyRadioListener::new(Rc::clone(&dessert));
///
///     let mut my_radio = Radio::new("my_radio");
///     my_radio.set_choices(vec!["Cake", "Ice Cream", "Pie"]);
///     my_radio.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Radio {
    name: String,
    state: RadioState,
    listener: Option<Box<dyn RadioListener>>,
}

impl Radio {
    /// Create a Radio
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: RadioState {
                choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                selected: 0,
                disabled: false,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.state.set_choices(choices);
    }

    /// Set the selected index
    pub fn set_selected(&mut self, selected: u32) {
        self.state.set_selected(selected);
    }

    /// Set the disabled flag to true
    pub fn set_disabled(&mut self) {
        self.state.set_disabled(true);
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn RadioListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Radio {
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
        let mut s = "".to_string();
        for (i, choice) in self.state.choices().iter().enumerate() {
            let selected = if self.state.selected() == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(
                &format!(
                    r#"<div id="{}" class="radio {} {} {}" onmousedown="{}"><div class="radio-outer"><div class="radio-inner"></div></div><label>{}</label></div>"#, 
                    self.name,
                    stretched,
                    disabled,
                    selected,
                    Event::change_js(&self.name, &format!("'{}'", i)), 
                    choice
                )
            );
        }
        s
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled {
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
        self.state.set_selected(value.parse::<u32>().unwrap());
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
