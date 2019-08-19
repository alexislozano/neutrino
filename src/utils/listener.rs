use crate::utils::event::Key;

/// # Listener
///
/// Trait that any of the listeners have to implement
pub trait Listener {
    /// Triggered by a Javascript "change" event
    fn on_change(&self, _value: &str) {}

    /// Triggered by a Javascript "keydown" event
    fn on_key(&self, _key: Key) {}
}
