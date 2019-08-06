use std::collections::HashMap;

pub trait Observable<T> {
    fn observe(&self) -> HashMap<String, T>;
}