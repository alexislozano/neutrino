use crate::utils::event::Event;

/// # Widget
///
/// Trait that any of the widgets have to implement
pub trait Widget {
    /// Return the HTML representation of the widget
    fn eval(&self) -> String;

    /// Trigger changes depending on the event
    fn trigger(&mut self, _event: &Event);

    fn on_update(&mut self);

    fn on_change(&mut self, _value: &str);
}
