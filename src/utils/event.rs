use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event {
    pub event: String,
    pub source: String,
    pub value: String,
}

impl Event {
    pub fn new(event: &str, source: &str, value: &str) -> Event {
        Event { 
            event: event.to_string(), 
            source: source.to_string(),
            value: value.to_string(),
        }
    }
}