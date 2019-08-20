/// # Event
/// 
/// Rust equivalent of Javascript events.
#[derive(Debug)]
pub enum Event {
    Undefined,
    Update,
    Change { source: String, value: String },
    Key { key: Key },
}

impl Event {
    pub fn change_js(source: &str, value: &str) -> String {
        format!(
            r#"(function(){{ invoke( {{ type: 'Change', source: '{}', value: {} }} ); event.stopPropagation(); }})()"#,
            source, value
        )
    }

    pub fn key_js() -> String {
        r#"(function() { if (event.ctrlKey && event.key !== 'Control') { invoke( { type: 'Key', key: event.key } ); } event.stopPropagation(); } )()"#.to_string()
    }

    pub fn undefined_js() -> String {
        r#"(function() { invoke( { type: 'Undefined' } ); event.stopPropagation(); } )()"#.to_string()
    }
}

/// # Key
/// 
/// An enum holding a keyboard key.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl Key {
    pub fn new(key: &str) -> Option<Self> {
        match key {
            "a" | "A" => Some(Key::A),
            "b" | "B" => Some(Key::B),
            "c" | "C" => Some(Key::C),
            "d" | "D" => Some(Key::D),
            "e" | "E" => Some(Key::E),
            "f" | "F" => Some(Key::F),
            "g" | "G" => Some(Key::G),
            "h" | "H" => Some(Key::H),
            "i" | "I" => Some(Key::I),
            "j" | "J" => Some(Key::J),
            "k" | "K" => Some(Key::K),
            "l" | "L" => Some(Key::L),
            "m" | "M" => Some(Key::M),
            "n" | "N" => Some(Key::N),
            "o" | "O" => Some(Key::O),
            "p" | "P" => Some(Key::P),
            "q" | "Q" => Some(Key::Q),
            "r" | "R" => Some(Key::R),
            "s" | "S" => Some(Key::S),
            "t" | "T" => Some(Key::T),
            "u" | "U" => Some(Key::U),
            "v" | "V" => Some(Key::V),
            "w" | "W" => Some(Key::W),
            "x" | "X" => Some(Key::X),
            "y" | "Y" => Some(Key::Y),
            "z" | "Z" => Some(Key::Z),
            "0" => Some(Key::Num0),
            "1" => Some(Key::Num1),
            "2" => Some(Key::Num2),
            "3" => Some(Key::Num3),
            "4" => Some(Key::Num4),
            "5" => Some(Key::Num5),
            "6" => Some(Key::Num6),
            "7" => Some(Key::Num7),
            "8" => Some(Key::Num8),
            "9" => Some(Key::Num9),
            _ => None
        }
    }
}