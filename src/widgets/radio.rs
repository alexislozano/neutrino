use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct RadioState {
    choices: Vec<String>,
    selected: u32,
    stretched: bool,
}

pub trait RadioListener {
    fn on_change(&self, state: &RadioState);
    fn on_update(&self, state: &mut RadioState);
}

/// # Radio
///
/// A list of radio buttons. Only one can be selected at a time.
///
/// ## Fields
///
/// ```text
/// pub struct Radio {
///     name: String,
///     choices: Vec<String>,
///     selected: u32,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_radio = Radio::new("my_radio")
///     .choices("Cake", "Pie")
///     .selected(0)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Radio {
    name: String,
    state: RadioState,
    listener: Option<Box<dyn RadioListener>>,
}

impl Radio {
    /// Create a Radio
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
    /// selected: 0,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: RadioState {
                choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                selected: 0,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the choices
    pub fn choices(self, choices: Vec<&str>) -> Self {
        Self {
            name: self.name,
            state: RadioState {
                choices: choices.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>(),
                selected: self.state.selected,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    /// Set the index of the selected choice
    pub fn selected(self, selected: u32) -> Self {
        Self {
            name: self.name,
            state: RadioState {
                choices: self.state.choices,
                selected: selected,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: RadioState {
                choices: self.state.choices,
                selected: self.state.selected,
                stretched: false,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn RadioListener>) -> Self {
        Self {
            name: self.name,
            state: RadioState {
                choices: self.state.choices,
                selected: self.state.selected,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
        }
    }
}

impl Widget for Radio {
    /// Return the HTML representation
    ///
    /// # Events
    ///
    /// ```text
    /// click -> index
    /// ```
    ///
    /// # Styling
    ///
    /// ```text
    /// class = radio
    /// class = radio-outer [checked]
    /// class = radio-inner [checked]
    /// ```
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        let mut s = "".to_string();
        for (i, choice) in self.state.choices.iter().enumerate() {
            let selected = if self.state.selected == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(
                &format!(
                    r#"<div class="radio {}" onmousedown="{}"><div class="radio-outer {}"><div class="radio-inner {}"></div></div><label>{}</label></div>"#, 
                    stretched,
                    Event::change_js(&self.name, &format!("'{}'", i)), 
                    selected, 
                    selected, 
                    choice
                )
            );
        }
        s
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.selected = selected choice index
    ///          self.listener.on_click()
    /// ```
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
        self.state.selected = value.parse::<u32>().unwrap();
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
