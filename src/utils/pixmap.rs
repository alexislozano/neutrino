use std::path::Path;
use std::fs;
use base64::encode;
use crate::utils::icon::Icon;

pub struct Pixmap {
    data: String,
    extension: String,
}

impl Pixmap {
    pub fn new(data: &str, extension: &str) -> Self {
        Pixmap { data: data.to_string(), extension: extension.to_string() }
    }

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

    pub fn from_icon(icon: Box<dyn Icon>) -> Self {
        let extension = icon.extension();
        let data = icon.data();
        Self { data, extension }
    }

    pub fn data(&self) -> &str {
        &self.data
    } 

    pub fn extension(&self) -> &str {
        match self.extension.as_ref() {
            "svg" => "svg+xml",
            ext => ext,
        }
    } 
}