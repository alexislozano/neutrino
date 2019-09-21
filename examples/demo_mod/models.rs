/* current selected tab panel id */
pub struct Panes {
    value: u8,
}

impl Panes {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

/* range is the value controlled by the Range and
the TextInput widgets, shown in ProgressBar and a Label. disabled is used
for input widgets */
pub struct State {
    range: i32,
    disabled: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            range: 0,
            disabled: false,
        }
    }

    pub fn range(&self) -> i32 {
        self.range
    }

    pub fn set_range(&mut self, range: i32) {
        self.range = range;
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}
