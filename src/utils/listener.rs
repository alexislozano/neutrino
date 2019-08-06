pub trait Listener {
    fn on_click(&self) {}
    fn on_change(&self, _value: &str) {}
}