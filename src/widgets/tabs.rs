use crate::utils::event::{Event, Key};
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::container::Direction;
use crate::widgets::widget::Widget;

/// # The state of a Tabs
///
/// ## Fields
///
/// ```text
/// titles: Vec<String>
/// children: Vec<Box<dyn Widget>>
/// selected: u32
/// direction: Direction
/// stretched: bool
/// style: String
/// ```
pub struct TabsState {
    titles: Vec<String>,
    children: Vec<Box<dyn Widget>>,
    selected: u32,
    direction: Direction,
    stretched: bool,
    style: String,
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

    /// Get the direction
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the style
    pub fn style(&self) -> &str {
        &self.style
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

    /// Set the direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
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
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// state:
///     title: vec![]
///     children: vec![]
///     selected: 0
///     direction: Direction::Horizontal
///     stretched: false
///     style: "".to_string()
/// listener: None
/// ```
///
/// ## Style
///
/// ```text
/// div.tabs[.direction-vertical]
///     div.tab-titles
///         div.tab-title[.selected]
///     div.tab
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
                direction: Direction::Horizontal,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Set the selected index
    pub fn set_selected(&mut self, selected: u32) {
        self.state.set_selected(selected);
    }

    /// Set the direction
    ///
    /// # Example
    ///
    /// ```text
    /// Direction::Horizontal
    ///
    /// +-------+-------+
    /// | Tab 1 | Tab 2 |   
    /// +-------+-------+
    /// |    Content    |
    /// |               |
    /// +---------------+
    ///
    /// Direction::Vertical
    ///
    /// +-------+-------------+
    /// | Tab 1 |             |
    /// +-------+   Content   |
    /// | Tab 2 |             |
    /// +-------+-------------+
    /// ```
    pub fn set_direction(&mut self, direction: Direction) {
        self.state.set_direction(direction);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TabsListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
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
        let style = inline_style(&scss_to_css(&format!(
            r##"#{}{{{}}}"##,
            self.name,
            self.state.style(),
        )));
        let mut html = format!(
            r#"
            <div id="{}" class="tabs {} {}">
                <div class="tab-titles" tabindex="0" onkeydown="{}">
            "#,
            self.name,
            stretched,
            self.state.direction().css(),
            Event::keypress_js(&self.name, "down"),
        );
        let tabs_number = self.state.titles.len();
        for (i, title) in self.state.titles.iter().enumerate() {
            let selected = if self.state.selected() == i as u32 {
                "selected"
            } else {
                ""
            };
            let first = if i == 0 { "first" } else { "" };
            let last = if i == tabs_number - 1 { "last" } else { "" };
            html.push_str(&format!(
                r#"
                <div class="tab-title {} {} {}" onclick="{}">
                    <label>{}</label>
                </div>
                "#,
                first,
                last,
                selected,
                Event::change_js(&self.name, &format!("'{}'", i)),
                title
            ));
        }
        html.push_str(&format!(
            r#"</div><div class="tab">{}</div></div>"#,
            self.state.children[self.state.selected() as usize].eval()
        ));
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => {
                self.state.children[self.state.selected as usize]
                    .trigger(event);
                self.on_update()
            }
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value);
                } else {
                    self.state.children[self.state.selected as usize]
                        .trigger(event);
                };
            }
            Event::Keypress { source, keys } => {
                if source == &self.name {
                    let new_selected = if keys.contains(&Key::Left) {
                        if self.state.selected == 0 {
                            self.state.titles.len() as u32 - 1
                        } else {
                            self.state.selected - 1
                        }
                    } else if keys.contains(&Key::Right) {
                        if self.state.selected
                            == self.state.titles.len() as u32 - 1
                        {
                            0
                        } else {
                            self.state.selected + 1
                        }
                    } else {
                        self.state.selected
                    };
                    self.on_change(&format!("{}", new_selected));
                } else {
                    self.state.children[self.state.selected as usize]
                        .trigger(event)
                }
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
