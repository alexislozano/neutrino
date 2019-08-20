use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

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
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
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
    choices: Vec<String>,
    selected: u32,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
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
        Radio {
            name: name.to_string(),
            choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
            selected: 0,
            listener: None,
            observer: None,
        }
    }

    /// Set the choices
    pub fn choices(self, choices: Vec<&str>) -> Self {
        Radio {
            name: self.name,
            choices: choices
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
            selected: self.selected,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the index of the selected choice
    pub fn selected(self, selected: u32) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: selected,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        Radio {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            listener: self.listener,
            observer: Some(observer),
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
        let mut s = "".to_string();
        for (i, choice) in self.choices.iter().enumerate() {
            let selected = if self.selected == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(
                &format!(
                    r#"<div class="radio" onclick="{}"><div class="radio-outer {}"><div class="radio-inner {}"></div></div><label>{}</label></div>"#, 
                    Event::change_js(&self.name, &format!("'{}'", i)), selected, selected, choice
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
                    self.selected = value.parse::<u32>().unwrap();
                    match &self.listener {
                        None => (),
                        Some(listener) => {
                            listener.on_change(value);
                        }
                    }
                }
            },
            _ => (),
        }
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// ```
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(_observer) => {}
        }
    }
}
