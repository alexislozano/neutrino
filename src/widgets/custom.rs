use crate::utils::event::Event;
use crate::widgets::widget::Widget;

pub struct CustomState {
    template: String,
    stretched: bool,
}

pub trait CustomListener {
    fn on_update(&self, state: &mut CustomState);
}

/// # Custom
///
/// An HTML templated widget. 
///
/// ## Fields
/// 
/// ```text
/// pub struct Custom {
///     name: String,
///     fields: HashMap<String, String>
///     template: String,
///     listener: Option<Box<dyn Listener>>,
///     observer: Option<Box<dyn Observer>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_custom = Custom("my_custom")
///     .template("<h1>My name is {firstname} {lastname}</h1>")
///     .listener(Box::new(my_listener))
///     .observer(Box::new(my_observer));
/// ```
pub struct Custom {
    name: String,
    state: CustomState,
    listener: Option<Box<dyn CustomListener>>,
}

impl Custom {
    /// Create a Custom
    ///
    /// # Default values
    ///
    /// ```text
    /// name: name.to_string(),
    /// fields: HashMap::new(),
    /// template: "".to_string(),
    /// listener: None,
    /// observer: None,
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: CustomState {
                template: "".to_string(),
                stretched: false,
            },
            listener: None,
        }
    }

    /// Set the template
    pub fn template(self, template: &str) -> Self {
        Self {
            name: self.name,
            state: CustomState {
                template: template.to_string(),
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretch(self) -> Self {
        Self {
            name: self.name,
            state: CustomState {
                template: self.state.template,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn CustomListener>) -> Self {
        Self {
            name: self.name,
            state: self.state,
            listener: Some(listener),
        }
    }
}

impl Widget for Custom {
    /// Return the HTML representation
    fn eval(&self) -> String {
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="custom {}">{}</div>"#,
            stretched,
            self.state.template,
        )
    }

    /// Trigger changes depending on the event
    ///
    /// # Events
    ///
    /// ```text
    /// update -> self.on_update()
    /// click -> self.listener.on_click()
    /// ```
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            _ => (),
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

    fn on_change(&mut self, _value: &str) {}
}
