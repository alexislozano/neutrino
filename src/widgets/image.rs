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

impl ImageState {
    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }

    pub fn background_color(&self) -> &str {
        &self.background_color
    }

    pub fn keep_ratio_aspect(&self) -> bool {
        self.keep_ratio_aspect
    }

    pub fn stretched(&self) -> bool {
        self.stretched
    }

    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }

    pub fn set_extension(&mut self, extension: &str) {
        self.extension = extension.to_string();
    }

    pub fn set_background_color(&mut self, background_color: &str) {
        self.background_color = background_color.to_string();
    }

    pub fn set_keep_ratio_aspect(&mut self, keep_ratio_aspect: bool) {
        self.keep_ratio_aspect = keep_ratio_aspect;
    }

    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }
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
                data: pixmap.data().to_string(),
                extension: pixmap.extension().to_string(),
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
                data: pixmap.data().to_string(),
                extension: pixmap.extension().to_string(),
                background_color: "black".to_string(),
                keep_ratio_aspect: false,
                stretched: false,
            },
            listener: None,
        }
    }

    pub fn set_background_color(&mut self, background_color: &str) {
        self.state.set_background_color(background_color);
    }

    pub fn set_keep_ratio_aspect(&mut self) {
        self.state.set_keep_ratio_aspect(true);
    }

    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ImageListener>) {
        self.listener = Some(listener);
    }
}

impl Widget for Image {
    fn eval(&self) -> String {
        let ratio = if self.state.keep_ratio_aspect() { "" } else { r#"width="100%" height="100%""# };
        let stretched = if self.state.stretched() { "stretched" } else { "" };
        format!(
            r#"<div class="image {}" style="background-color:{};"><img {} src="data:image/{};base64,{}" /></div>"#, 
            stretched, 
            self.state.background_color(), 
            ratio, 
            self.state.extension(), 
            self.state.data(),
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