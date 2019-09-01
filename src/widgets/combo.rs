use crate::utils::event::Event;
use crate::widgets::widget::Widget;
use crate::utils::pixmap::Pixmap;
use crate::utils::icon::Icon;

/// # The state of a Combo
///
/// ## Fields
/// 
/// ```text
/// choices: Vec<String>
/// selected: u32
/// opened: bool
/// stretched: bool
/// arrow_data: Option<String>
/// arrow_extension: Option<String>
/// ```
pub struct ComboState {
    choices: Vec<String>,
    selected: u32,
    opened: bool,
    stretched: bool,
    arrow_data: Option<String>,
    arrow_extension: Option<String>,
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

    /// Get the stretched flag 
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the arrow
    pub fn arrow(&self) -> Option<Pixmap> {
        match (&self.arrow_data, &self.arrow_extension) {
            (Some(data), Some(extension)) => Some(Pixmap::new(data, extension)),
            _ => None,
        }
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.choices = choices.iter().map(
            |c| c.to_string()
        ).collect::<Vec<String>>();
    }

    /// Set the selected flag
    pub fn set_selected(&mut self, selected: u32) {
        self.selected = selected;
    }

    /// Set the opened flag
    pub fn set_opened(&mut self, opened: bool) {
        self.opened = opened;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the arrow from a path
    pub fn set_arrow_from_path(&mut self, path: &str) {
        let pixmap = Pixmap::from_path(path);
        self.arrow_data = Some(pixmap.data().to_string());
        self.arrow_extension = Some(pixmap.extension().to_string());
    }

    /// Set the arrow from an icon
    pub fn set_arrow_from_icon(&mut self, icon: Box<dyn Icon>) {
        let pixmap = Pixmap::from_icon(icon);
        self.arrow_data = Some(pixmap.data().to_string());
        self.arrow_extension = Some(pixmap.extension().to_string());
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
///     stretched: false,
///     arrow_data: None,
///     arrow_extension: None
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
                stretched: false,
                arrow_data: None,
                arrow_extension: None,
            },
            listener: None,
        }
    }

    /// Set the choices
    pub fn set_choices(&mut self, choices: Vec<&str>) {
        self.state.set_choices(choices);
    }

    /// Set the index of the selected choice
    pub fn set_selected(&mut self, selected: u32){
        self.state.set_selected(selected);
    }

    /// Set the opened flag to true
    pub fn set_opened(&mut self) {
        self.state.set_opened(true);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the arrow from a path
    pub fn set_arrow_from_path(&mut self, path: &str) {
        self.state.set_arrow_from_path(path);
    }

    /// Set the arrow from an icon
    pub fn set_arrow_from_icon(&mut self, icon: Box<dyn Icon>) {
        self.state.set_arrow_from_icon(icon);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ComboListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Combo {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        let opened = if self.state.opened() { "opened" } else { "" };
        let mut s = match self.state.arrow() {
            Some(arrow) => {
                format!(
                    r#"<div class="combo {}"><div onmousedown="{}" class="combo-button {}">{}<div><img src="data:image/{};base64,{}" /></div></div>"#,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    opened,
                    self.state.choices()[self.state.selected() as usize],
                    arrow.extension(),
                    arrow.data(),
                )
            },
            None => {
                format!(
                    r#"<div class="combo {}"><div onmousedown="{}" class="combo-button">{}</div>"#,
                    stretched,
                    Event::change_js(&self.name, "'-1'"),
                    self.state.choices()[self.state.selected() as usize],
                )
            }
        };
        if self.state.opened() {
            s.push_str(r#"<div class="combo-choices">"#);
            for (i, choice) in self.state.choices().iter().enumerate() {
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

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value);
                } else {
                    self.state.set_opened(false);
                }
            },
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
