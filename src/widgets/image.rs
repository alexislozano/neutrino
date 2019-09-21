use crate::utils::event::Event;
use crate::utils::icon::Icon;
use crate::utils::pixmap::Pixmap;
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::widget::Widget;

/// # The state of an Image
///
/// ## Fields
///
/// ```text
/// data: String
/// extension: String
/// background: String
/// keep_ratio_aspect: bool
/// stretched: bool
/// style: String
/// ```
pub struct ImageState {
    data: String,
    extension: String,
    background: String,
    keep_ratio_aspect: bool,
    stretched: bool,
    style: String,
}

impl ImageState {
    /// Get the base64 encoded image data
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Get the extension
    pub fn extension(&self) -> &str {
        &self.extension
    }

    /// Get the background color
    pub fn background(&self) -> &str {
        &self.background
    }

    /// Get the keep_ratio_aspect flag
    pub fn keep_ratio_aspect(&self) -> bool {
        self.keep_ratio_aspect
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the style
    pub fn style(&self) -> &str {
        &self.style
    }

    /// Set the base64 encoded image data
    pub fn set_data(&mut self, data: &str) {
        self.data = data.to_string();
    }

    /// Set the extension
    pub fn set_extension(&mut self, extension: &str) {
        self.extension = extension.to_string();
    }

    /// Set the background color
    pub fn set_background(&mut self, background: &str) {
        self.background = background.to_string();
    }

    /// Set the keep_ratio_aspect flag
    pub fn set_keep_ratio_aspect(&mut self, keep_ratio_aspect: bool) {
        self.keep_ratio_aspect = keep_ratio_aspect;
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
    }
}

/// # The listener for an Image
pub trait ImageListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut ImageState);
}

/// # An element able to display images from icons and path
///
/// ## Fields
///
/// ```text
/// name: String
/// state: ImageState
/// listener: Option<Box<dyn ImageListener>>
/// ```
///
/// ## Default values
///
/// The variable `pixmap` is built in both constructors from the given Icon or
/// path.
///
/// ```text
/// name: name.to_string()
/// state:
///     data: pixmap.data().to_string()
///     extension: pixmap.extension().to_string()
///     background: "black".to_string()
///     keep_ratio_aspect: false
///     stretched: false
///     style: "".to_string()
/// listener: None
/// ```
/// 
/// ## Style
///
/// ```text
/// div.image
///     img
/// ```
/// 
/// ## Example
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use neutrino::widgets::image::{Image, ImageListener, ImageState};
/// use neutrino::utils::theme::Theme;
/// use neutrino::utils::pixmap::Pixmap;
/// use neutrino::{App, Window};
///
///
/// struct Painting {
///     path: String,
/// }
///
/// impl Painting {
///     fn new(path: &str) -> Self {
///         Self { path: path.to_string() }
///     }
///
///     fn path(&self) -> &str {
///         &self.path
///     }
/// }
///
///
/// struct MyImageListener {
///     painting: Rc<RefCell<Painting>>,
/// }
///
/// impl MyImageListener {
///    pub fn new(painting: Rc<RefCell<Painting>>) -> Self {
///        Self { painting }
///    }
/// }
///
/// impl ImageListener for MyImageListener {
///     fn on_update(&self, state: &mut ImageState) {
///         let pixmap = Pixmap::from_path(self.painting.borrow().path());
///         state.set_data(pixmap.data());
///         state.set_extension(pixmap.extension());
///     }
/// }
///
///
/// fn main() {
///     let painting = Rc::new(RefCell::new(Painting::new("/home/neutrino/le_radeau_de_la_meduse.jpg")));
///
///     let my_listener = MyImageListener::new(Rc::clone(&painting));
///
///     let mut my_image = Image::from_path("my_image", "/home/neutrino/le_radeau_de_la_meduse.jpg");
///     my_image.set_listener(Box::new(my_listener));
/// }
/// ```
pub struct Image {
    name: String,
    state: ImageState,
    listener: Option<Box<dyn ImageListener>>,
}

impl Image {
    /// Create an image from a path
    pub fn from_path(name: &str, path: &str) -> Self {
        let pixmap = Pixmap::from_path(path);
        Self {
            name: name.to_string(),
            state: ImageState {
                data: pixmap.data().to_string(),
                extension: pixmap.extension().to_string(),
                background: "black".to_string(),
                keep_ratio_aspect: false,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Create an image from an icon
    pub fn from_icon(name: &str, icon: Box<dyn Icon>) -> Self {
        let pixmap = Pixmap::from_icon(icon);
        Self {
            name: name.to_string(),
            state: ImageState {
                data: pixmap.data().to_string(),
                extension: pixmap.extension().to_string(),
                background: "black".to_string(),
                keep_ratio_aspect: false,
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Set the background color
    pub fn set_background(&mut self, background: &str) {
        self.state.set_background(background);
    }

    /// Set the keep_ratio_aspect flag to true
    pub fn set_keep_ratio_aspect(&mut self) {
        self.state.set_keep_ratio_aspect(true);
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn ImageListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

impl Widget for Image {
    fn eval(&self) -> String {
        let ratio = if self.state.keep_ratio_aspect() {
            ""
        } else {
            r#"width="100%" height="100%""#
        };
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let style = inline_style(&scss_to_css(&format!(
            r##"#{}{{{}}}"##,
            self.name,
            self.state.style(),
        )));
        let html = format!(
            r#"<div id="{}" class="image {}" style="background:{};"><img {} src="data:image/{};base64,{}" /></div>"#, 
            self.name,
            stretched,
            self.state.background(),
            ratio,
            self.state.extension(),
            self.state.data(),
        );
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value)
                }
            }
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
