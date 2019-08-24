/// # Theme
///
/// A theme is used to change the style of the application.
pub enum Theme {
    Breeze,
}

impl Theme {
    /// Return the CSS class corresponding to the theme
    pub fn class(&self) -> &str {
        match self {
            Theme::Breeze => "breeze",
        }
    }
}