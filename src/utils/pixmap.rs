use crate::utils::icon::Icon;
use base64::encode;
use std::fs;
use std::path::Path;

/// # A model for an image
///
/// As a webview does not have access to the local file system, the given
/// images are encoded into text (Base64) to be displayed.
///
/// ## Fields
///
/// ```text
/// data: String
/// extension: String
/// ```
pub struct Pixmap {
    data: String,
    extension: String,
}

impl Pixmap {
    /// Create a Pixmap from text data
    pub fn new(data: &str, extension: &str) -> Self {
        Pixmap {
            data: data.to_string(),
            extension: extension.to_string(),
        }
    }

    /// Create a Pixmap from a file path
    pub fn from_path(path: &str) -> Self {
        let extension = match Path::new(path).extension() {
            Some(ext) => ext.to_str().unwrap().to_string(),
            None => "".to_string(),
        };
        let data = match fs::read(path) {
            Ok(file) => encode(&file),
            Err(_) => "".to_string(),
        };
        Self { data, extension }
    }

    /// Create a Pixmap from an Icon
    pub fn from_icon(icon: Box<dyn Icon>) -> Self {
        let extension = icon.extension();
        let data = icon.data();
        Self { data, extension }
    }

    /// Get the data
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Get the extension
    pub fn extension(&self) -> &str {
        match self.extension.as_ref() {
            "svg" => "svg+xml",
            ext => ext,
        }
    }
}
