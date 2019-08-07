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

    pub fn js(event: &str, source: &str, value: &str) -> String {
        format!(
            r#"(function(){{invoke({{event:'{}', source:'{}', value: {}}})}})()"#,
            event, source, value
        )
    }
}