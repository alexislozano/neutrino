pub enum Theme {
    Breeze,
}

impl Theme {
    pub fn class(&self) -> &str {
        match self {
            Theme::Breeze => "breeze",
        }
    }
}
