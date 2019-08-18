use std::collections::HashMap;

/// # Observer
///
/// Trait that any of the observers have to implement
pub trait Observer {
    /// Return the fields which will be used to change the values of the fields
    /// of the widget of the Observer
    fn observe(&self) -> HashMap<String, String>;
}
