use std::fs;
use base64::encode;
use crate::widgets::widget::Widget;

pub struct Image {
    data: String,
    keep_ratio_aspect: bool,
    background_color: String,
}

impl Image {
    pub fn from_path(path: &str) -> Self {
        Image { 
            data: match fs::read(path) {
                Ok(file) => encode(&file),
                Err(_) => "".to_string(),
            },
            keep_ratio_aspect: false,
            background_color: "black".to_string(),
        } 
    }

    pub fn keep_ratio_aspect(self, keep_ratio_aspect: bool) -> Self {
        Image { 
            data: self.data,
            keep_ratio_aspect: keep_ratio_aspect,
            background_color: self.background_color,
        } 
    }

    pub fn background_color(self, background_color: &str) -> Self {
        Image { 
            data: self.data,
            keep_ratio_aspect: self.keep_ratio_aspect,
            background_color: background_color.to_string(),
        } 
    }
}

impl Widget for Image {
    fn eval(&self) -> String {
        let ratio = if self.keep_ratio_aspect {
            ""
        } else {
            r#"width="100%" height="100%""#
        };
        format!(
            r#"<div class="image" style="background-color:{};"><img {} src="data:image;base64, {}" /></div>"#, 
            self.background_color, ratio, self.data,
        )
    }
}