use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct TabsState {
    children: Vec<(String, Box<dyn Widget>)>,
    selected: u32,
    stretched: bool,
}

pub trait TabsListener {
    fn on_update(&self, state: &mut TabsState);
    fn on_change(&self, state: &TabsState);
}

/// # Tabs
///
/// A list of tabs.
///
/// ## Fields
///
/// ```text
/// pub struct Tabs {
///     name: String,
///     children: Vec<(String, Box<dyn Widget>)>,
///     selected: u32,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_tabs = Tabs::new("my_tabs")
///     .children(vec![("Cake", cake_tab), ("Pie", pie_tab)])
///     .selected(0)
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Tabs {
    name: String,
    state: TabsState,
    listener: Option<Box<dyn TabsListener>>,
}

impl Tabs {
    /// Create a Tabs
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// children: vec![],
    /// selected: 0,
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: TabsState {
                children: vec![],
                selected: 0,
                stretched: false,
            },
            listener: None,
        }
    }

    // Set the index of the selected tab
    pub fn selected(self, selected: u32) -> Self {
        Self {
            name: self.name,
            state: TabsState {
                children: self.state.children,
                selected: selected,
                stretched: self.state.stretched,
            },
            listener: None,
        }
    }

    pub fn stretched(self) -> Self {
        Self {
            name: self.name,
            state: TabsState {
                children: self.state.children,
                selected: self.state.selected,
                stretched: true,
            },
            listener: None,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn TabsListener>) -> Self {
        Self {
            name: self.name,
            state: TabsState {
                children: self.state.children,
                selected: self.state.selected,
                stretched: self.state.stretched,
            },
            listener: Some(listener),
        }
    }

    /// Add a tab
    pub fn add(&mut self, child: (&str, Box<dyn Widget>)) {
        self.state.children.push((child.0.to_string(), child.1));
    }
}

impl Widget for Tabs {
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
    /// class = tabs
    /// class = tab-titles
    /// class = tab-title [selected]
    /// class = tab
    /// ```
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        let mut s = format!(
            r#"<div class="tabs {}"><div class="tab-titles">"#,
            stretched
        );
        for (i, child) in self.state.children.iter().enumerate() {
            let selected = if self.state.selected == i as u32 {
                "selected"
            } else {
                ""
            };
            s.push_str(&format!(
                r#"<div class="tab-title {}" onmousedown="{}">{}</div>"#,
                selected,
                Event::change_js(&self.name, &format!("'{}'", i)),
                child.0
            ));
        }
        s.push_str(&format!(
            r#"</div><div class="tab">{}</div>"#,
            self.state.children[self.state.selected as usize].1.eval()
        ));
        s.push_str("</div>");
        s
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.selected = index of the selected tab
    ///          self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value);
                } else {
                    self.state.children[self.state.selected as usize].1.trigger(event);
                };
            },
            _ => self.state.children[self.state.selected as usize].1.trigger(event),
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
