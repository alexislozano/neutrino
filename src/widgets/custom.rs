use crate::utils::event::Event;
use crate::utils::listener::Listener;
use crate::utils::observer::Observer;
use crate::widgets::widget::Widget;
use std::collections::HashMap;
use strfmt::strfmt;

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
///     listener: Option<Box<Listener>>,
///     observer: Option<Box<Observer>>,
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
    fields: HashMap<String, String>,
    template: String,
    listener: Option<Box<Listener>>,
    observer: Option<Box<Observer>>,
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
        Custom {
            name: name.to_string(),
            fields: HashMap::new(),
            template: "".to_string(),
            listener: None,
            observer: None,
        }
    }

    /// Set the template
    pub fn template(self, template: &str) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: template.to_string(),
            listener: self.listener,
            observer: self.observer,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: Some(listener),
            observer: self.observer,
        }
    }

    /// Set the observer
    pub fn observer(self, observer: Box<Observer>) -> Custom {
        Custom {
            name: self.name,
            fields: self.fields,
            template: self.template,
            listener: self.listener,
            observer: Some(observer),
        }
    }
}

impl Widget for Custom {
    /// Return the HTML representation
    fn eval(&self) -> String {
        strfmt(&self.template, &self.fields).unwrap_or(self.template.to_string())
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
        if event.event == "update" {
            self.on_update();
        } else if event.source == self.name {
            match &self.listener {
                None => (),
                Some(listener) => {
                    if event.event == "click" {
                        listener.on_click();
                    }
                }
            }
        };
    }

    /// Set the fields of the widget using the fields of the HashMap
    /// returned by the observer
    fn on_update(&mut self) {
        match &self.observer {
            None => (),
            Some(observer) => {
                for (key, value) in observer.observe() {
                    self.fields.insert(key, value);
                }
            }
        }
    }
}
