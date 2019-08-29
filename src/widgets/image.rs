use crate::widgets::widget::Widget;
use crate::utils::event::Event;
use crate::utils::icon::Icon;
use crate::utils::pixmap::Pixmap;

pub struct ImageState {
    data: String,
    extension: String,
    background_color: String,
    keep_ratio_aspect: bool,
    stretched: bool,
}

pub trait ImageListener {
    fn on_update(&self, state: &mut ImageState);
}

pub struct Image {
    name: String,
    state: ImageState,
    listener: Option<Box<dyn ImageListener>>,
}

impl Image {
    pub fn from_path(name: &str, path: &str) -> Self {
        let pixmap = Pixmap::from_path(path);
        Self { 
            name: name.to_string(),
            state : ImageState {
                data: pixmap.data(),
                extension: pixmap.extension(),
                background_color: "black".to_string(),
                keep_ratio_aspect: false,
                stretched: false,
            },
            listener: None,
        } 
    }

    pub fn from_icon(name: &str, icon: Box<dyn Icon>) -> Self {
        let pixmap = Pixmap::from_icon(icon);
        Self {
            name: name.to_string(),
            state : ImageState {
                data: pixmap.data(),
                extension: pixmap.extension(),
                background_color: "black".to_string(),
                keep_ratio_aspect: false,
                stretched: false,
            },
            listener: None,
        }
    }

    pub fn keep_ratio_aspect(self) -> Self {
        Self {
            name: self.name,
            state : ImageState {
                data: self.state.data,
                extension: self.state.extension,
                background_color: self.state.background_color,
                keep_ratio_aspect: true,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn background_color(self, background_color: &str) -> Self {
        Self { 
            name: self.name,
            state : ImageState {
                data: self.state.data,
                extension: self.state.extension,
                background_color: background_color.to_string(),
                keep_ratio_aspect: self.state.keep_ratio_aspect,
                stretched: self.state.stretched,
            },
            listener: self.listener,
        }
    }

    pub fn stretched(self) -> Self {
        Self { 
            name: self.name,
            state : ImageState {
                data: self.state.data,
                extension: self.state.extension,
                background_color: self.state.background_color,
                keep_ratio_aspect: self.state.keep_ratio_aspect,
                stretched: true,
            },
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<dyn ImageListener>) -> Self {
        Self {
            name: self.name,
            state: self.state,
            listener: Some(listener),
        }
    }
}

impl Widget for Image {
    fn eval(&self) -> String {
        let ratio = if self.state.keep_ratio_aspect { "" } else { r#"width="100%" height="100%""# };
        let stretched = if self.state.stretched { "stretched" } else { "" };
        format!(
            r#"<div class="image {}" style="background-color:{};"><img {} src="data:image/{};base64,{}" /></div>"#, 
            stretched, 
            self.state.background_color, 
            ratio, 
            self.state.extension, 
            self.state.data,
        )
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            _ => (),
        }
    }

    fn on_update(&mut self) {
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_update(&mut self.state);
            }
        }
    }

    fn on_change(&mut self, _value: &str) {}
}