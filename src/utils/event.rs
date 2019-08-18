use serde::Deserialize;

/// # Event
/// 
/// Rust equivalent of Javascript events. It contains the type of event, the
/// source of the event and the value sent by the event if needed.
///
/// ## Fields
/// 
/// ```text
/// pub struct Event {
///     event: String,
///     source: String,
///     value: String,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_event = Event::new("click", "my_button", "");
/// ```
#[derive(Deserialize)]
pub struct Event {
    pub event: String,
    pub source: String,
    pub value: String,
}

impl Event {
    /// Create an Event
    ///
    /// # Default values
    ///
    /// ```text
    /// event: event.to_string(),
    /// source: source.to_string(),
    /// value: value.to_string(),
    /// ```
    pub fn new(event: &str, source: &str, value: &str) -> Event {
        Event {
            event: event.to_string(),
            source: source.to_string(),
            value: value.to_string(),
        }
    }

    /// Return a string containing a Javascript function that triggers an event
    ///
    /// # Example
    /// 
    /// ```text
    /// Event::js("click", "my_button", "");
    /// ```
    pub fn js(event: &str, source: &str, value: &str) -> String {
        format!(
            r#"(function(){{invoke({{event:'{}', source:'{}', value: {}}})}})()"#,
            event, source, value
        )
    }
}
