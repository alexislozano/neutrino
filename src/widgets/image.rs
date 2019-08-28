use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::icon::Icon;
use crate::utils::pixmap::Pixmap;

pub struct Image {
    data: String,
    keep_ratio_aspect: bool,
    background_color: String,
    extension: String,
    stretch: String,
}

impl Image {
    pub fn from_path(path: &str) -> Self {
        let pixmap = Pixmap::from_path(path);
        Image { 
            data: pixmap.data(),
            keep_ratio_aspect: false,
            background_color: "black".to_string(),
            extension: pixmap.extension(),
            stretch: "".to_string(),
        } 
    }

    pub fn from_icon(icon: Box<dyn Icon>) -> Self {
        let pixmap = Pixmap::from_icon(icon);
        Image {
            data: pixmap.data(),
            keep_ratio_aspect: false,
            background_color: "black".to_string(),
            extension: pixmap.extension(),
            stretch: "".to_string(),
        }
    }

    pub fn keep_ratio_aspect(self, keep_ratio_aspect: bool) -> Self {
        Image { 
            data: self.data,
            keep_ratio_aspect: keep_ratio_aspect,
            background_color: self.background_color,
            extension: self.extension,
            stretch: self.stretch,
        } 
    }

    pub fn background_color(self, background_color: &str) -> Self {
        Image { 
            data: self.data,
            keep_ratio_aspect: self.keep_ratio_aspect,
            background_color: background_color.to_string(),
            extension: self.extension,
            stretch: self.stretch,
        } 
    }

    pub fn stretch(self) -> Self {
        Image { 
            data: self.data,
            keep_ratio_aspect: self.keep_ratio_aspect,
            background_color: self.background_color,
            extension: self.extension,
            stretch: "stretch".to_string(),
        } 
    }
}

impl Widget for Image {
    fn eval(&self) -> String {
        let ratio = match self.keep_ratio_aspect {
            true => "",
            false => r#"width="100%" height="100%""#
        };
        format!(
            r#"<div class="image {}" style="background-color:{};"><img {} src="data:image/{};base64,{}" /></div>"#, 
            self.stretch, 
            self.background_color, 
            ratio, 
            self.extension, 
            self.data,
        )
    }

    /// Set the values of the widget using the fields of the HashMap
    /// returned by the observer
    ///
    /// # Fields
    ///
    /// ```text
    /// ```
    fn on_update(&mut self) {}

    /// Trigger changes depending on the event
    fn trigger(&mut self, _event: &Event) {}
}