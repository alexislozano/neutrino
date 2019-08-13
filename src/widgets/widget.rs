use crate::utils::event::Event;

/// Trait that any of the widgets have to implement
pub trait Widget {

    /// Return the HTML representation of the widget
    fn eval(&self) -> String;
    
    /// Trigger changes depending on the event
    fn trigger(&mut self, event: &Event);
    
    /// Set the values of the widget using the fields of the HashMap 
    /// defining the model
    fn on_update(&mut self) {}
}