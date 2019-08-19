use super::models::Panes;
use neutrino::utils::observer::Observer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct TabsObserver {
    panes: Rc<RefCell<Panes>>,
}

impl TabsObserver {
    pub fn new(panes: Rc<RefCell<Panes>>) -> Self {
        TabsObserver { panes: panes }
    }
}

impl Observer for TabsObserver {
    fn observe(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();
        fields.insert(
            "selected".to_string(),
            format!("{}", self.panes.borrow().value()),
        );
        fields
    }
}
