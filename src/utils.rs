use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event {
    pub event: String,
    pub source: String,
}

impl Event {
    pub fn new(event: &str, source: &str) -> Event {
        Event { event: event.to_string(), source: source.to_string() }
    }
}

pub trait Listener {
    fn on_click(&self);
}

pub trait Observable {
    fn value(&self) -> String;
}