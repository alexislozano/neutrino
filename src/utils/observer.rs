use std::collections::HashMap;

pub trait Observer {
    fn observe(&self) -> HashMap<String, String>;
}
