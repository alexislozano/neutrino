pub struct Panes {
    value: u8,
}

impl Panes {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}
