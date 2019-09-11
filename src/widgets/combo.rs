use crate::utils::event::Event;
use crate::utils::icon::Icon;
use crate::utils::pixmap::Pixmap;
use crate::widgets::widget::Widget;

/// # The state of a Combo
///
/// ## Fields
///
/// ```text
/// choices: Vec<String>
/// selected: u32
/// opened: bool
/// disabled: bool
/// stretched: bool
/// arrow_data: Option<String>
/// arrow_extension: Option<String>
/// ```
pub struct ComboState {
    choices: Vec<String>,
    selected: u32,
    opened: bool,
    disabled: bool,
    stretched: bool,
    icon_data: Option<String>,
    icon_extension: Option<String>,
}

impl ComboState {
    /// Get the choices
    pub fn choices(&self) -> &Vec<String> {
        &self.choices
    }

    /// Get the selected flag
    pub fn selected(&self) -> u32 {
        self.selected
    }

    /// Get the opened flag
    pub fn opened(&self) -> bool {
        self.opened
    }

    /// Get the disabled flag
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the icon
    pub fn icon(&self) -> Option<Pixmap> {
        match (&self.icon_data, &self.icon_extension) {
            (Some(data), Some(extension)) => Some(Pixmap::new(data, extension)),
            _ => None,
        }
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.choices = choices
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
    }

    /// Set the selected flag
    pub fn set_selected(&mut self, selected: u32) {
        self.selected = selected;
    }

    /// Set the opened flag
    pub fn set_opened(&mut self, opened: bool) {
        self.opened = opened;
    }

    /// Set the disabled flag
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the icon
    pub fn set_icon(&mut self, icon: Box<dyn Icon>) {
        let pixmap = Pixmap::from_icon(icon);
        self.icon_data = Some(pixmap.data().to_string());
        self.icon_extension = Some(pixmap.extension().to_string());
    }
}

/// # The listener of a Combo
pub trait ComboListener {
    /// Function triggered on change event
    fn on_change(&self, state: &ComboState);

    /// Function triggered on update event
    fn on_update(&self, state: &mut ComboState);
}

/// # A collapsible list of strings
///
/// ## Fields
///
/// ```text
/// name: String
/// state: ComboState
/// listener: Option<Box<dyn ComboListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
///     selected: 0,
///     opened: false,
///     disabled: false,
///     stretched: false,
///     icon_data: None,
///     icon_extension: None
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::combo::{Combo, ComboListener, ComboState};
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
/// struct MyComboListener {
///     dessert: Rc<RefCell<Dessert>>,
/// }
///
/// impl MyComboListener {
///    pub fn new(dessert: Rc<RefCell<Dessert>>) -> Self {
///        Self { dessert }
///    }
/// }
///
/// impl ComboListener for MyComboListener {
///     fn on_change(&self, state: &ComboState) {
///         let index = state.selected();
///         self.dessert.borrow_mut().set(
///             index,
///             &state.choices()[index as usize]
///         );
///     }
///
///     fn on_update(&self, state: &mut ComboState) {
///         state.set_selected(self.dessert.borrow().index());
///     }
/// }
///
///
/// fn main() {
///     let dessert = Rc::new(RefCell::new(Dessert::new()));
///
///     let my_listener = MyComboListener::new(Rc::clone(&dessert));
///
///     let mut my_combo = Combo::new("my_combo");
///     my_combo.set_choices(vec!["Cake", "Ice Cream", "Pie"]);
///     my_combo.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Combo {
    name: String,
    state: ComboState,
    listener: Option<Box<dyn ComboListener>>,
}

impl Combo {
    /// Create a Combo
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: ComboState {
                choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                selected: 0,
                opened: false,
                disabled: false,
                stretched: false,
                icon_data: None,
                icon_extension: None,
            },
            listener: None,
        }
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.state.set_choices(choices);
    }

    /// Set the index of the selected choice
    pub fn set_selected(&mut self, selected: u32) {
        self.state.set_selected(selected);
    }

    /// Set the opened flag to true
    pub fn set_opened(&mut self) {
        self.state.set_opened(true);
    }

    /// Set the disabled flag to true
    pub fn set_disabled(&mut self) {
        self.state.set_disabled(true);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the icon
    pub fn set_icon(&mut self, icon: Box<dyn Icon>) {
        self.state.set_icon(icon);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ComboListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Combo {
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
        let opened = if self.state.opened() { "opened" } else { "" };
        let mut s = match self.state.icon() {
            Some(icon) => {
                format!(
                    r#"<div id="{}" class="combo {}"><div onmousedown="{}" class="combo-button {} {}">{}<img src="data:image/{};base64,{}" /></div>"#,
                    self.name,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    opened,
                    disabled,
                    self.state.choices()[self.state.selected() as usize],
                    icon.extension(),
                    icon.data(),
                )
            },
            None => {
                format!(
                    r#"<div id="{}" class="combo {}"><div onmousedown="{}" class="combo-button {} {}">{}</div>"#,
                    self.name,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    opened,
                    disabled,
                    self.state.choices()[self.state.selected() as usize],
                )
            }
        };
        if self.state.opened() {
            s.push_str(r#"<div class="combo-choices">"#);
            let combos_length = self.state.choices().len();
            for (i, choice) in self.state.choices().iter().enumerate() {
                let last = if i == combos_length - 1 {
                    "last"
                } else {
                    ""
                };
                s.push_str(&format!(
                    r#"<div class="combo-choice {}" onmousedown="{}">{}</div>"#,
                    last,
                    Event::change_js(&self.name, &format!("'{}'", i)),
                    choice
                ));
            }
            s.push_str(r#"</div>"#);
        }
        s.push_str("</div>");
        s
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name && !self.state.disabled() {
                    self.on_change(value);
                } else {
                    self.state.set_opened(false);
                }
            }
            _ => self.state.set_opened(false),
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
        self.state.set_opened(!self.state.opened());
        let selected = value.parse::<i32>().unwrap();
        if selected > -1 {
            self.state.set_selected(selected as u32);
        }
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_change(&self.state);
            }
        }
    }
}
