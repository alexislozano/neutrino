use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;

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
    children: Vec<(String, Box<dyn Widget>)>,
    selected: u32,
    listener: Option<Box<dyn Listener>>,
    observer: Option<Box<dyn Observer>>,
    stretch: String,
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
        Tabs {
            name: name.to_string(),
            children: vec![],
            selected: 0,
            listener: None,
            observer: None,
            stretch: "".to_string(),
        }
    }

    // Set the index of the selected tab
    pub fn selected(self, selected: u32) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: selected,
            listener: self.listener,
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn Listener>) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: self.selected,
            listener: Some(listener),
            observer: self.observer,
            stretch: self.stretch,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<dyn Observer>) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: self.selected,
            listener: self.listener,
            observer: Some(observer),
            stretch: self.stretch,
        }
    }

    pub fn stretch(self) -> Self {
        Tabs {
            name: self.name,
            children: self.children,
            selected: self.selected,
            listener: self.listener,
            observer: self.observer,
            stretch: "stretch".to_string(),
        }
    }

    /// Add a tab
    pub fn add(&mut self, child: (&str, Box<dyn Widget>)) {
        self.children.push((child.0.to_string(), child.1));
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
        let mut s = format!(
            r#"<div class="tabs {}"><div class="tab-titles">"#,
            self.stretch
        );
        for (i, child) in self.children.iter().enumerate() {
            let selected = if self.selected == i as u32 {
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
            self.children[self.selected as usize].1.eval()
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
                    self.children[self.selected as usize].1.trigger(event);
                };
            },
            _ => self.children[self.selected as usize].1.trigger(event),
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
            Some(observer) => {
                self.selected = observer.observe()["selected"].parse::<u32>().unwrap();
            }
        }
    }
}
