pub struct Person {
    firstname: String,
    lastname: String,
}

impl Person {
    pub fn new() -> Self {
        Person {
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
        }
    }

    pub fn firstname(&self) -> String {
        self.firstname.to_string()
    }

    pub fn lastname(&self) -> String {
        self.lastname.to_string()
    }

    pub fn set_firstname(&mut self, firstname: &str) {
        self.firstname = firstname.to_string();
    }

    pub fn set_lastname(&mut self, lastname: &str) {
        self.lastname = lastname.to_string();
    }
}