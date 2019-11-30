use crate::utils::event::Event;
use crate::utils::event::Key;

/// # The state of a MenuBar
///
/// ## Fields
///
/// ```text
/// selected_item: Option<u32>
/// selected_function: Option<u32
/// ```
pub struct MenuBarState {
    selected_item: Option<u32>,
    selected_function: Option<u32>,
    hovered_function: Option<u32>,
    underlined: bool,
}

impl MenuBarState {
    /// Get selected item index
    pub fn selected_item(&self) -> Option<u32> {
        self.selected_item
    }

    /// Get selected function index
    pub fn selected_function(&self) -> Option<u32> {
        self.selected_function
    }

    /// Get hovered function index
    pub fn hovered_function(&self) -> Option<u32> {
        self.hovered_function
    }

    /// Get underlined flag
    pub fn underlined(&self) -> bool {
        self.underlined
    }

    /// Set selected item index
    pub fn set_selected_item(&mut self, selected_item: Option<u32>) {
        self.selected_item = selected_item;
    }

    /// Set selected function index
    pub fn set_selected_function(&mut self, selected_function: Option<u32>) {
        self.selected_function = selected_function;
    }

    /// Set hovered function index
    pub fn set_hovered_function(&mut self, hovered_function: Option<u32>) {
        self.hovered_function = hovered_function;
    }

    /// Set underlined flag
    pub fn set_underlined(&mut self, underlined: bool) {
        self.underlined = underlined;
    }
}

/// # The listener of a MenuBar
pub trait MenuBarListener {
    /// Function triggered on change event
    fn on_change(&self, state: &MenuBarState);
}

/// # A MenuBar
///
/// ## Fields
///
/// ```text
/// items: Vec<MenuItem>
/// state: MenuBarState
/// listener: Option<Box<dyn MenuBarListener>>
/// ```
///
/// ## Default values
///
/// ```text
/// items: vec![]
/// state:
///     selected_item: None
///     selected_function: None
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::menubar::{
///     MenuBar,
///     MenuBarState,
///     MenuBarListener,
///     MenuItem,
///     MenuFunction
/// };
/// use neutrino::utils::theme::Theme;
/// use neutrino::{App, Window};
///
///
/// struct DocumentList {
///     values: Vec<String>,
/// }
///
/// impl DocumentList {
///     fn new() -> Self {
///         Self { values: vec![] }
///     }
///
///     fn add(&mut self, document: &str) {
///         self.values.push(document.to_string());
///     }
/// }
///
///
/// struct MyMenuBarListener {
///     document_list: Rc<RefCell<DocumentList>>,
/// }
///
/// impl MyMenuBarListener {
///    pub fn new(document_list: Rc<RefCell<DocumentList>>) -> Self {
///        Self { document_list }
///    }
/// }
///
/// impl MenuBarListener for MyMenuBarListener {
///     fn on_change(&self, state: &MenuBarState) {
///         match state.selected_item() {
///             None => (),
///             Some(selected_item) => {
///                 if selected_item == 0 {
///                     self.document_list.borrow_mut().add(
///                         "New document"
///                     );
///                 }
///             }
///         }
///     }
/// }
///
///
/// fn main() {
///     let document_list = Rc::new(RefCell::new(DocumentList::new()));
///     
///     let mut new = MenuFunction::new("New");
///     new.set_shortcut("Ctrl-N");
///     
///     let mut file = MenuItem::new("File");
///     file.add(new);
///     
///     let my_menubarlistener = MyMenuBarListener::new(
///         Rc::clone(&document_list)
///     );
///     
///     let mut my_menubar = MenuBar::new();
///     my_menubar.set_listener(Box::new(my_menubarlistener));
///     my_menubar.add(file);
/// }
/// ```
pub struct MenuBar {
    items: Vec<MenuItem>,
    state: MenuBarState,
    listener: Option<Box<dyn MenuBarListener>>,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self {
            items: vec![],
            state: MenuBarState {
                selected_item: None,
                selected_function: None,
                hovered_function: None,
                underlined: false,
            },
            listener: None,
        }
    }
}

impl MenuBar {
    /// Create a MenuBar
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn MenuBarListener>) {
        self.listener = Some(listener);
    }

    /// Add a MenuItem
    pub fn add(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    /// Return the HTML representation of the widget
    pub fn eval(&self) -> String {
        let mut s = r#"<div class="menubar">"#.to_string();
        for (i, item) in self.items.iter().enumerate() {
            s.push_str(&item.eval(
                i,
                self.state.selected_item,
                self.state.hovered_function,
                self.state.underlined,
            ));
        }
        s.push_str(r#"</div>"#);
        s
    }

    /// Trigger functions depending on the event
    pub fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => (),
            Event::Change { source, value } => {
                if *source == "menuitem" {
                    self.on_item_change(value);
                    self.state.set_hovered_function(None);
                } else if *source == "menufunction" {
                    self.on_function_change(value);
                } else {
                    self.state.set_selected_item(None);
                    self.state.set_hovered_function(None);
                }
            }
            Event::Keypress { source, keys } => {
                if source == "app" {
                    if keys.contains(&Key::Alt) {
                        self.state.set_underlined(true);
                        for (i, item) in self.items.iter().enumerate() {
                            if keys.contains(&item.key) {
                                self.state.set_selected_item(Some(i as u32));
                            };
                        }
                    } else {
                        self.state.set_underlined(false);
                    }
                    if let Some(i) = self.state.selected_item {
                        if keys.contains(&Key::Escape) {
                            self.state.set_selected_item(None);
                            self.state.set_hovered_function(None);
                        } else if keys.contains(&Key::Left) {
                            self.state.set_selected_item(Some(if i == 0 {
                                self.items.len() as u32 - 1
                            } else {
                                i - 1
                            }));
                            self.state.set_hovered_function(None);
                        } else if keys.contains(&Key::Right) {
                            self.state.set_selected_item(Some(
                                if i == self.items.len() as u32 - 1 {
                                    0
                                } else {
                                    i + 1
                                },
                            ));
                            self.state.set_hovered_function(None);
                        } else if keys.contains(&Key::Down) {
                            let functions = &self.items[i as usize].functions;
                            match self.state.hovered_function() {
                                Some(j) => self.state.set_hovered_function(
                                    if j == functions.len() as u32 - 1 {
                                        Some(0)
                                    } else {
                                        Some(j + 1)
                                    }
                                ),
                                None => self.state.set_hovered_function(
                                    Some(0)
                                )
                            }
                        } else if keys.contains(&Key::Up) {
                            let functions = &self.items[i as usize].functions;
                            match self.state.hovered_function() {
                                Some(j) => self.state.set_hovered_function(
                                    if j == 0 {
                                        Some(functions.len() as u32 - 1)
                                    } else {
                                        Some(j - 1)
                                    }
                                ),
                                None => self.state.set_hovered_function(
                                    Some(functions.len() as u32 - 1)
                                )
                            }
                        } else if keys.contains(&Key::Enter) {
                            if let Some(j) = self.state.hovered_function() {
                                self.on_function_change(&format!(
                                    "click;{}", 
                                    j
                                ));
                            }
                        }
                    }
                }
            }
            _ => self.state.set_selected_item(None),
        }
    }

    /// Function triggered on MenuItem change event
    fn on_item_change(&mut self, value: &str) {
        let values = value.split(';').collect::<Vec<&str>>();
        let e = values[0];
        let index = values[1].parse::<u32>().unwrap();
        self.state
            .set_selected_item(match self.state.selected_item() {
                Some(_) => match e {
                    "click" => None,
                    _ => Some(index),
                },
                None => match e {
                    "click" => Some(index),
                    _ => None,
                },
            });
    }

    /// Function triggered on MenuFunction change event
    fn on_function_change(&mut self, value: &str) {
        let values = value.split(';').collect::<Vec<&str>>();
        let e = values[0];
        let index = values[1].parse::<u32>().unwrap();
        match e {
            "click" => {
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        self.state.set_selected_function(Some(index));
                        listener.on_change(&self.state);
                        self.state.set_selected_function(None);
                    }
                };
                self.state.set_selected_item(None)
            }
            _ => self.state.set_hovered_function(Some(index)),
        }
    }
}

/// # An item of a MenuBar
///
/// ## Fields
///
/// ```text
/// name: String
/// functions: Vec<MenuFunction>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// functions: vec![]
/// ```
pub struct MenuItem {
    name: String,
    key: Key,
    index: usize,
    functions: Vec<MenuFunction>,
}

impl MenuItem {
    /// Create a MenuItem
    pub fn new(name: &str, key: Key, index: usize) -> Self {
        Self {
            name: name.to_string(),
            key,
            index,
            functions: vec![],
        }
    }

    /// Add a MenuFunction
    pub fn add(&mut self, function: MenuFunction) {
        self.functions.push(function);
    }

    /// Return the HTML representation of the widget
    fn eval(
        &self,
        index: usize,
        selected_item: Option<u32>,
        hovered_function: Option<u32>,
        underlined: bool,
    ) -> String {
        let selected_str = match selected_item {
            Some(selected_index) => {
                if selected_index == index as u32 {
                    "selected"
                } else {
                    ""
                }
            }
            None => "",
        };
        let pre = match self.name.get(..self.index) {
            Some(pre) => pre.to_string(),
            None => self.name.to_string(),
        };
        let post = match self.name.get(self.index + 1..) {
            Some(post) => post.to_string(),
            None => "".to_string(),
        };
        let character = match self.name.get(self.index..=self.index) {
            Some(character) => {
                if underlined {
                    format!("<span class='underlined'>{}</span>", character)
                } else {
                    format!("<span>{}</span>", character)
                }
            }
            None => "".to_string(),
        };
        let mut s = format!(
            r#"
            <div class="menuitem">
                <div class="menuitem-title {}" 
                    onmousedown="{}" 
                    onmouseover="{}"
                >
                    {}{}{}
                </div>"#,
            selected_str,
            Event::change_js("menuitem", &format!("'click;{}'", index)),
            Event::change_js("menuitem", &format!("'over;{}'", index)),
            // self.name
            pre,
            character,
            post,
        );
        if let Some(selected_index) = selected_item {
            if selected_index == index as u32 {
                s.push_str(r#"<div class="menufunctions">"#);
                let functions_number = self.functions.len();
                for (i, function) in self.functions.iter().enumerate() {
                    s.push_str(&function.eval(
                        i,
                        i == 0,
                        i == functions_number - 1,
                        hovered_function,
                    ));
                }
                s.push_str(r#"</div>"#);
            }
        }
        s.push_str(r#"</div>"#);
        s
    }
}

/// # A function of a MenuItem
///
/// ## Fields
///
/// ```text
/// name: String
/// shortcut: Option<String>
/// ```
///
/// ## Default values
///
/// ```text
/// name: name.to_string()
/// shortcut: None
/// ```
pub struct MenuFunction {
    name: String,
    shortcut: Option<String>,
}

impl MenuFunction {
    /// Create a MenuFunction
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            shortcut: None,
        }
    }

    /// Set the shortcut
    pub fn set_shortcut(&mut self, shortcut: &str) {
        self.shortcut = Some(shortcut.to_string());
    }

    /// Return the HTML representation of the widget
    fn eval(
        &self,
        index: usize,
        first: bool,
        last: bool,
        hovered_function: Option<u32>,
    ) -> String {
        let hovered = match hovered_function {
            Some(hovered_index) => {
                if hovered_index == index as u32 {
                    "hovered"
                } else {
                    ""
                }
            }
            None => "",
        };
        format!(
            r#"
            <div class="menufunction {} {} {}" 
                onmousedown="{}" 
                onmouseover="{}"
            >
                <span class="title">{}</span>
                <span class="shortcut">{}</span>
            </div>
            "#,
            if first { "first" } else { "" },
            if last { "last" } else { "" },
            hovered,
            Event::change_js("menufunction", &format!("'click;{}'", index)),
            Event::change_js("menufunction", &format!("'over;{}'", index)),
            self.name,
            match &self.shortcut {
                None => "",
                Some(shortcut) => shortcut,
            },
        )
    }
}
