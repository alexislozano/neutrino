use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

/// # ComboBox
///
/// A collapsible list of strings.
///
/// ## Fields
/// 
/// ```text
/// pub struct Combo {
///     name: String,
///     choices: Vec<String>,
///     selected: u32,
///     opened: bool,
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_combo = Combo::new("my_combo")
///     .choices(vec!["Cake", "Pie"])
///     .selected(0)
///     .opened(false)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Combo {
    name: String,
    choices: Vec<String>,
    selected: u32,
    opened: bool,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
}

impl Combo {
    /// Create a Combo
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
    /// selected: 0,
    /// opened: false,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Combo {
            name: name.to_string(),
            choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
            selected: 0,
            opened: false,
            listener: None,
            observer: None,
        }
    }

    /// Set the choices
    pub fn choices(self, choices: Vec<&str>) -> Self {
        Combo {
            name: self.name,
            choices: choices
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>(),
            selected: self.selected,
            opened: self.opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the index of the selected choice
    pub fn selected(self, selected: u32) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: selected,
            opened: self.opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the opened flag
    pub fn opened(self, opened: bool) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: opened,
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: self.opened,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Self {
        Combo {
            name: self.name,
            choices: self.choices,
            selected: self.selected,
            opened: self.opened,
            listener: self.listener,
            observer: Some(observer),
        }
    }
}

impl Widget for Combo {
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
    /// class = combo
    /// class = combo-button
    /// class = combo-choices
    /// class = combo-choice
    /// ```
    fn eval(&self) -> String {
        let mut s = format!(
            r#"<div class="combo"><div onclick="{}" class="combo-button">{}</div>"#,
            Event::change_js(&self.name, "'-1'"),
            self.choices[self.selected as usize]
        );
        if self.opened {
            s.push_str(r#"<div class="combo-choices">"#);
            for (i, choice) in self.choices.iter().enumerate() {
                s.push_str(&format!(
                    r#"<div class="combo-choice" onclick="{}">{}</div>"#,
                    Event::change_js(&self.name, &format!("'{}'", i)),
                    choice
                ));
            }
            s.push_str(r#"</div>"#);
        }
        s.push_str("</div>");
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
                    self.opened = !self.opened;
                    let selected = value.parse::<i32>().unwrap();
                    if selected > -1 {
                        self.selected = selected as u32;
                    }
                    match &self.listener {
                        None => (),
                        Some(listener) => {
                            listener.on_change(value);
                        }
                    }
                } else {
                    self.opened = false;
                }
            },
            _ => self.opened = false,
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
