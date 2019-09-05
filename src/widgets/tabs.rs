use crate::utils::event::Event;
use crate::widgets::widget::Widget;

/// # The state of a Tabs
///
/// ## Fields
///
/// ```text
/// titles: Vec<String>
/// children: Vec<Box<dyn Widget>>
/// selected: u32
/// stretched: bool
/// ```
pub struct TabsState {
    titles: Vec<String>,
    children: Vec<Box<dyn Widget>>,
    selected: u32,
    stretched: bool,
}

impl TabsState {
    /// Get the titles
    pub fn titles(&self) -> &Vec<String> {
        &self.titles
    }

    /// Get the children
    pub fn children(&self) -> &Vec<Box<dyn Widget>> {
        &self.children
    }

    /// Get the selected index
    pub fn selected(&self) -> u32 {
        self.selected
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Set the titles
    pub fn set_titles(&mut self, titles: Vec<&str>) {
        self.titles = titles
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
    }

    /// Set the children
    pub fn set_children(&mut self, children: Vec<Box<dyn Widget>>) {
        self.children = children;
    }

    /// Set the selected index
    pub fn set_selected(&mut self, selected: u32) {
        self.selected = selected;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Add a tab
    fn add(&mut self, name: &str, child: Box<dyn Widget>) {
        self.titles.push(name.to_string());
        self.children.push(child);
    }
}

/// # The listener of a Tabs
pub trait TabsListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut TabsState);

    /// Function triggered on change event
    fn on_change(&self, state: &TabsState);
}

/// # A list of tabs
///
/// ## Fields
///
/// ```text
/// name: String
/// state: TabsState    
/// listener: Option<Box<dyn TabsListener>>
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::tabs::{Tabs, TabsListener, TabsState};
/// use neutrino::widgets::label::Label;
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
/// struct MyTabsListener {
///     dessert: Rc<RefCell<Dessert>>,
/// }
///
/// impl MyTabsListener {
///    pub fn new(dessert: Rc<RefCell<Dessert>>) -> Self {
///        Self { dessert }
///    }
/// }
///
/// impl TabsListener for MyTabsListener {
///     fn on_change(&self, state: &TabsState) {
///         let index = state.selected();
///         self.dessert.borrow_mut().set(
///             index,
///             &state.titles()[index as usize]
///         );
///     }
///
///     fn on_update(&self, state: &mut TabsState) {
///         state.set_selected(self.dessert.borrow().index());
///     }
/// }
///
///
/// fn main() {
///     let dessert = Rc::new(RefCell::new(Dessert::new()));
///
///     let my_listener = MyTabsListener::new(Rc::clone(&dessert));
///
///     let mut my_label = Label::new("my_label");
///     my_label.set_text("World!");
///
///     let mut my_tabs = Tabs::new("my_tabs");
///     my_tabs.add("Hello", Box::new(my_label));
///     my_tabs.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Tabs {
    name: String,
    state: TabsState,
    listener: Option<Box<dyn TabsListener>>,
}

impl Tabs {
    /// Create a Tabs
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: TabsState {
                titles: vec![],
                children: vec![],
                selected: 0,
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the selected index
    pub fn set_selected(&mut self, selected: u32) {
        self.state.set_selected(selected);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TabsListener>) {
        self.listener = Some(listener);
    }

    /// Add a tab
    pub fn add(&mut self, name: &str, child: Box<dyn Widget>) {
        self.state.add(name, child);
    }
}

impl Widget for Tabs {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let mut s = format!(
            r#"<div id="{}" class="tabs {}"><div class="tab-titles">"#,
            self.name, stretched
        );
        let tabs_number = self.state.titles.len();
        for (i, title) in self.state.titles.iter().enumerate() {
            let selected = if self.state.selected() == i as u32 {
                "selected"
            } else {
                ""
            };
            let first = if i == 0 {
                "first"
            } else {
                ""
            };
            let last = if i == tabs_number - 1 {
                "last"
            } else {
                ""
            };
            s.push_str(&format!(
                r#"<div class="tab-title {} {} {}" onmousedown="{}">{}</div>"#,
                first,
                last,
                selected,
                Event::change_js(&self.name, &format!("'{}'", i)),
                title
            ));
        }
        s.push_str(&format!(
            r#"</div><div class="tab">{}</div>"#,
            self.state.children[self.state.selected() as usize].eval()
        ));
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
                    self.state.children[self.state.selected as usize]
                        .trigger(event);
                };
            }
            _ => {
                self.state.children[self.state.selected as usize].trigger(event)
            }
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
