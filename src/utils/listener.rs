/// # Listener
///
/// Trait that any of the listeners have to implement
pub trait Listener {
    /// Triggered by a Javascript "click" event
    fn on_click(&self) {}

    /// Triggered by a Javascript "change" event
    fn on_change(&self, _value: &str) {}
}
