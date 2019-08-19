use serde::Deserialize;

/// # Event
/// 
/// Rust equivalent of Javascript events.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    Update,
    Change { source: String, value: String },
    Key { key: Key },
}

impl Event {
    pub fn change_js(source: &str, value: &str) -> String {
        format!(
            r#"(function(){{ invoke( {{ type: 'Change', source: '{}', value: {} }} ) }})()"#,
            source, value
        )
    }

    pub fn key_js() -> String {
        r#"(function() { if (event.ctrlKey && event.key !== 'Control') { invoke( { type: 'Key', key: event.code } ) } } )()"#.to_string()
    }
}

/// # Key
/// 
/// An enum holding a keyboard key.
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Key {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Undefined,
}