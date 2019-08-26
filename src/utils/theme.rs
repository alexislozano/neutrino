/// # Theme
///
/// A theme is used to change the style of the application.
pub enum Theme {
    Breeze,
}

impl Theme {
    /// Return the CSS string corresponding to the theme
    pub fn css(&self) -> &str {
        match self {
            Theme::Breeze => include_str!(concat!(env!("OUT_DIR"), "/breeze.css")),
        }
    }
}