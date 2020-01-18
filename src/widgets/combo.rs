use crate::utils::event::{Event, Key};
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::widget::Widget;

/// # The state of a Combo
///
/// ## Fields
///
/// ```text
/// choices: Vec<String>
/// selected: u32
/// hovered: u32
/// opened: bool
/// disabled: bool
/// stretched: bool
/// style: String
/// ```
pub struct ComboState {
    choices: Vec<String>,
    selected: u32,
    hovered: u32,
    opened: bool,
    disabled: bool,
    stretched: bool,
    style: String,
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

    /// Get the hovered flag
    pub fn hovered(&self) -> u32 {
        self.hovered
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

    /// Get the style
    pub fn style(&self) -> &str {
        &self.style
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

    /// Set the hovered flag
    pub fn set_hovered(&mut self, hovered: u32) {
        self.hovered = hovered;
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

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
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
///     hovered: 0,
///     opened: false,
///     disabled: false,
///     stretched: false,
///     style: "".to_string()
/// listener: None
/// ```
///
/// ## Style
///
/// ```text
/// div.combo[.opened][.disabled]
///     div.combo-button
///         div.combo-icon
///     div.combo-choices
///         div.combo-choice
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
                hovered: 0,
                opened: false,
                disabled: false,
                stretched: false,
                style: "".to_string(),
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

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ComboListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
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
        let style = inline_style(&scss_to_css(&format!(
            r##"#{}{{{}}}"##,
            self.name,
            self.state.style(),
        )));
        let mut html = format!(
            r#"
            <div id="{}" class="combo {} {} {}">
                <div onclick="{}" tabindex="0" class="combo-button"
                onkeydown="{}">
                    {}
                    <div class="combo-icon"></div>
                </div>
            "#,
            self.name,
            stretched,
            opened,
            disabled,
            Event::change_js(&self.name, "'click;-1'"),
            Event::keypress_js(&self.name, "down"),
            self.state.choices()[self.state.selected() as usize],
        );
        if self.state.opened() {
            html.push_str(r#"<div class="combo-choices">"#);
            let combos_length = self.state.choices().len();
            for (i, choice) in self.state.choices().iter().enumerate() {
                let last = if i == combos_length - 1 { "last" } else { "" };
                let hovered = if i == self.state.hovered as usize {
                    "hovered"
                } else {
                    ""
                };
                html.push_str(&format!(
                    r#"
                    <div class="combo-choice {} {}" onclick="{}"
                    onmouseover="{}">
                        {}
                    </div>
                    "#,
                    last,
                    hovered,
                    Event::change_js(&self.name, &format!("'click;{}'", i)),
                    Event::change_js(&self.name, &format!("'over;{}'", i)),
                    choice
                ));
            }
            html.push_str(r#"</div>"#);
        }
        html.push_str("</div>");
        format!("{}{}", style, html)
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
            Event::Keypress { source, keys } => {
                if source == &self.name && !self.state.disabled() {
                    if keys.contains(&Key::Enter) {
                        if self.state.opened {
                            self.state.set_selected(self.state.hovered);
                            self.state.set_opened(false);
                        } else {
                            self.state.set_opened(true);
                        }
                    } else if keys.contains(&Key::Down) {
                        self.state.set_hovered(
                            if self.state.hovered
                                == self.state.choices.len() as u32 - 1
                            {
                                0
                            } else {
                                self.state.hovered + 1
                            },
                        );
                        if !self.state.opened {
                            self.state.set_selected(self.state.hovered);
                        }
                    } else if keys.contains(&Key::Up) {
                        self.state.set_hovered(if self.state.hovered == 0 {
                            self.state.choices.len() as u32 - 1
                        } else {
                            self.state.hovered - 1
                        });
                        if !self.state.opened {
                            self.state.set_selected(self.state.hovered);
                        }
                    } else if keys.contains(&Key::Tab) {
                        self.state.set_opened(false);
                    }
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
        let values = value.split(';').collect::<Vec<&str>>();
        let e = values[0];
        let index = values[1].parse::<i32>().unwrap();
        match e {
            "click" => {
                self.state.set_opened(!self.state.opened());
                if index > -1 {
                    self.state.set_selected(index as u32);
                }
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_change(&self.state);
                    }
                }
            }
            _ => self.state.set_hovered(index as u32),
        }
    }
}
