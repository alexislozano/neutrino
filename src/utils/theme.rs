/// # Theme
///
/// A theme is used to change the style of the application.
pub enum Theme {
    Default,
    Breeze,
}

impl Theme {
    /// Return the CSS string corresponding to the theme
    pub fn css(&self) -> &str {
        match self {
            Theme::Default => include_str!(concat!(env!("OUT_DIR"), "/themes/default.css")),
            Theme::Breeze => include_str!(concat!(env!("OUT_DIR"), "/themes/breeze.css")),
        }
    }
}