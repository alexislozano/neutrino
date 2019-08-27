use std::path::Path;
use std::fs;
use base64::encode;
use crate::utils::icon::Icon;

pub struct Pixmap {
    data: String,
    extension: String,
}

impl Pixmap {
    pub fn from_path(path: &str) -> Self {
        let extension = match Path::new(path).extension() {
            Some(ext) => ext.to_str().unwrap().to_string(),
            None => "".to_string(),
        };
        let data = match fs::read(path) {
            Ok(file) => encode(&file),
            Err(_) => "".to_string(),
        };
        Pixmap { data, extension }
    }

    pub fn from_icon(icon: Box<Icon>) -> Self {
        let extension = icon.extension();
        let data = icon.data();
        Pixmap { data, extension }
    }

    pub fn data(&self) -> String {
        self.data.clone()
    } 

    pub fn extension(&self) -> String {
        match self.extension.as_ref() {
            "svg" => "svg+xml".to_string(),
            ext => ext.to_string(),
        }
    } 
}