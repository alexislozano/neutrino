use std::path::Path;
use std::fs;
use base64::encode;
use crate::utils::theme::Theme;

pub enum Icon {
    Bell,
    Bookmark,
    Check,
    Clock,
    Down,
    Edit,
    Heart,
    Home,
    Minus,
    Left,
    Lock,
    Plus,
    Right,
    Save,
    Star,
    Trash,
    Unlock,
    Up,
    ZoomIn,
    ZoomOut,
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
                Icon::Plus => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/default/plus.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/default/plus.extension")).to_string()
                },
                Icon::Down => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/default/down.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/default/down.extension")).to_string()
                },
                _ => Pixmap {
                    data: "".to_string(),
                    extension: "".to_string(),
                }
            },
            Theme::Breeze => match icon {
                Icon::Bell => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/bell.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/bell.extension")).to_string()
                },
                Icon::Bookmark => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/bookmark.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/bookmark.extension")).to_string()
                },
                Icon::Check => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/check.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/check.extension")).to_string()
                },
                Icon::Clock => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/clock.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/clock.extension")).to_string()
                },
                Icon::Down => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/down.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/down.extension")).to_string()
                },
                Icon::Edit => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/edit.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/edit.extension")).to_string()
                },
                Icon::Heart => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/heart.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/heart.extension")).to_string()
                },
                Icon::Home => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/home.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/home.extension")).to_string()
                },
                Icon::Minus => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/minus.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/minus.extension")).to_string()
                },
                Icon::Left => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/left.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/left.extension")).to_string()
                },
                Icon::Lock => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/lock.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/lock.extension")).to_string()
                },
                Icon::Plus => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/plus.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/plus.extension")).to_string()
                },
                Icon::Right => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/right.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/right.extension")).to_string()
                },
                Icon::Save => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/save.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/save.extension")).to_string()
                },
                Icon::Star => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/star.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/star.extension")).to_string()
                },
                Icon::Trash => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/trash.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/trash.extension")).to_string()
                },
                Icon::Unlock => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/unlock.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/unlock.extension")).to_string()
                },
                Icon::Up => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/up.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/up.extension")).to_string()
                },
                Icon::ZoomIn => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/zoom-in.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/zoom-in.extension")).to_string()
                },
                Icon::ZoomOut => Pixmap {
                    data: include_str!(concat!(env!("OUT_DIR"), "/breeze/zoom-out.data")).to_string(),
                    extension: include_str!(concat!(env!("OUT_DIR"), "/breeze/zoom-out.extension")).to_string()
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