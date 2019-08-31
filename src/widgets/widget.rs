use crate::utils::event::Event;

/// # Trait that any of the widgets have to implement
pub trait Widget {
    /// Return the HTML representation of the widget
    fn eval(&self) -> String;

    /// Trigger functions depending on the event
    fn trigger(&mut self, _event: &Event);

    /// Function triggered on update event
    fn on_update(&mut self);

    /// Function triggered on change event
    fn on_change(&mut self, _value: &str);
}
