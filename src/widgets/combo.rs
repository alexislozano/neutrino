use crate::utils::event::Event;
use crate::widgets::widget::Widget;
use crate::utils::pixmap::Pixmap;
use crate::utils::icon::Icon;

pub struct ComboState {
    choices: Vec<String>,
    selected: u32,
    opened: bool,
    stretched: bool,
    arrow: Option<Pixmap>
}

pub trait ComboListener {
    fn on_change(&self, state: &ComboState);
    fn on_update(&self, state: &mut ComboState);
}

/// # Combo
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
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
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
    state: ComboState,
    listener: Option<Box<dyn ComboListener>>,
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
        Self {
            name: name.to_string(),
            state: ComboState {
                choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                selected: 0,
                opened: false,
                stretched: false,
                arrow: None,
            },
            listener: None,
        }
    }

    /// Set the choices
    pub fn choices(self, choices: Vec<&str>) -> Self {
        Self {
            name: self.name,
            state: ComboState {
                choices: choices.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>(),
                selected: self.state.selected,
                opened: self.state.opened,
                stretched: self.state.stretched,
                arrow: self.state.arrow,
            },
            listener: self.listener,
        }
    }

    /// Set the index of the selected choice
    pub fn selected(self, selected: u32) -> Self {
        Self {
            name: self.name,
            state: ComboState {
                choices: self.state.choices,
                selected: selected,
                opened: self.state.opened,
                stretched: self.state.stretched,
                arrow: self.state.arrow,
            },
            listener: self.listener,
        }
    }

    /// Set the opened flag
    pub fn opened(self) -> Self {
        Self {
            name: self.name,
            state: ComboState {
                choices: self.state.choices,
                selected: self.state.selected,
                opened: true,
                stretched: self.state.stretched,
                arrow: self.state.arrow,
            },
            listener: self.listener,
        }
    }

    /// Set the stretced flag
    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: ComboState {
                choices: self.state.choices,
                selected: self.state.selected,
                opened: self.state.opened,
                stretched: true,
                arrow: self.state.arrow,
            },
            listener: self.listener,
        }
    }

    /// Set the arrow from a path
    pub fn arrow_from_path(self, path: &str) -> Self {
        let pixmap = Pixmap::from_path(path);
        Self {
            name: self.name,
            state: ComboState {
                choices: self.state.choices,
                selected: self.state.selected,
                opened: self.state.opened,
                stretched: self.state.stretched,
                arrow: Some(pixmap),
            },
            listener: self.listener,
        }
    }

    /// Set the arrow from an icon
    pub fn arrow_from_icon(self, icon: Box<dyn Icon>) -> Self {
        let pixmap = Pixmap::from_icon(icon);
        Self {
            name: self.name,
            state: ComboState {
                choices: self.state.choices,
                selected: self.state.selected,
                opened: self.state.opened,
                stretched: self.state.stretched,
                arrow: Some(pixmap),
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn ComboListener>) -> Self {
        Self {
            name: self.name,
            state: self.state,
            listener: Some(listener),
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
        let stretched = if self.state.stretched { "stretched" } else { "" };
        let mut s = match &self.state.arrow {
            Some(arrow) => {
                format!(
                    r#"<div class="combo {}"><div onmousedown="{}" class="combo-button">{}<div><img src="data:image/{};base64,{}" /></div></div>"#,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    self.state.choices[self.state.selected as usize],
                    arrow.extension(),
                    arrow.data(),
                )
            },
            None => {
                format!(
                    r#"<div class="combo {}"><div onmousedown="{}" class="combo-button">{}</div>"#,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    self.state.choices[self.state.selected as usize],
                )
            }
        };
        if self.state.opened {
            s.push_str(r#"<div class="combo-choices">"#);
            for (i, choice) in self.state.choices.iter().enumerate() {
                s.push_str(&format!(
                    r#"<div class="combo-choice" onmousedown="{}">{}</div>"#,
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
                    self.on_change(value);
                } else {
                    self.state.opened = false;
                }
            },
            _ => self.state.opened = false,
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
        self.state.opened = !self.state.opened;
        let selected = value.parse::<i32>().unwrap();
        if selected > -1 {
            self.state.selected = selected as u32;
        }
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
