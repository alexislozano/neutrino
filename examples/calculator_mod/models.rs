pub struct Calculator {
    value1: i32,
    value2: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { 
            value1: 0,
            value2: 0,
        }
    }

    pub fn set_value1(&mut self, value1: i32) {
        self.value1 = value1;
    }

    pub fn set_value2(&mut self, value2: i32) {
        self.value2 = value2;
    }

    pub fn value1(&self) -> i32 {
        self.value1
    }

    pub fn value2(&self) -> i32 {
        self.value2
    }

    pub fn result(&self) -> i32 {
        self.value1 + self.value2
    }
}