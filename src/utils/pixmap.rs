use std::path::Path;
use std::fs;
use base64::encode;
use crate::utils::theme::Theme;

pub enum Icon {
    Add,
    Down,
}

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

    pub fn from_theme(theme: Theme, icon: Icon) -> Self {
        match theme {
            Theme::Default => match icon {
                Icon::Add => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/default/add.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/default/add.extension")).to_string()
                },
                Icon::Down => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/default/down.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/default/down.extension")).to_string()
                },
            },
            Theme::Breeze => match icon {
                Icon::Add => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/add.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/add.extension")).to_string()
                },
                Icon::Down => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/down.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/down.extension")).to_string()
                },
            }
        }
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