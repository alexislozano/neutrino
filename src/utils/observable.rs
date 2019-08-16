use std::collections::HashMap;

pub trait Observable {
    fn observe(&self) -> HashMap<String, String>;
}
