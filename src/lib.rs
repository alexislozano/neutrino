use web_view::*;

pub mod utils;
pub mod widgets;

use utils::event::{Event, Key};
use utils::theme::Theme;
use widgets::widget::Widget;
use widgets::menubar::MenuBar;

use json;

/// # An abstract application
///
/// ## Example
///
/// ```text
/// App::run(my_window);
/// ```
pub struct App;

impl App {
    /// Run the application
    pub fn run(mut window: Window) {
        let html = format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <meta charset="UTF-8">
                    {styles}
                </head>
                <body onkeydown="{key}" onmousedown="{click}">
                    <div id="app"></div>
                    {scripts}
                </body>
            </html>
            "#,
            styles = format!(
                "{}\n{}\n{}\n",
                inline_style(include_str!(concat!(env!("OUT_DIR"), "/app.css"))),
                inline_style(&window.theme.css()),
                inline_style(&window.custom_css),
            ),
            scripts = format!(
                "{}\n{}\n",
                inline_script(include_str!("www/app/morphdom.min.js")),
                inline_script(include_str!("www/app/app.js"))
            ),
            key = Event::key_js(),
            click = Event::undefined_js(),
        );

        let title = &window.title.to_owned();
        let width = window.width;
        let height = window.height;
        let resizable = window.resizable;
        let debug = window.debug;

        let webview = web_view::builder()
            .title(title)
            .content(Content::Html(html))
            .size(width, height)
            .resizable(resizable)
            .user_data("")
            .debug(debug)
            .invoke_handler(|webview, arg| {
                let event: Event = match json::parse(arg) {
                    Ok(value) => match value["type"].as_str().unwrap() {
                        "Update" => Event::Update,
                        "Key" => match Key::new(value["key"].as_str().unwrap()) {
                            Some(key) => Event::Key { key: key },
                            None => Event::Undefined,
                        },
                        "Change" => Event::Change { 
                            source: value["source"].as_str().unwrap().to_string(), 
                            value: value["value"].as_str().unwrap().to_string(), 
                        },
                        _ => Event::Undefined,
                    },
                    Err(_) => Event::Undefined,
                };
                window.trigger(&event);
                match event {
                    Event::Undefined => (),
                    _ => window.trigger(&Event::Update)
                };
                window.render(webview)
            })
            .build()
            .unwrap();

        webview.run().unwrap();
        std::process::exit(0);
    }
}

/// # The listener of a Window
pub trait WindowListener {
    /// Function triggered on key event
    fn on_key(&self, _key: Key);
}

/// # A window containing the widgets
///
/// ## Fields
///
/// ```text
/// title: String
/// width: i32
/// height: i32
/// resizable: bool
/// debug: bool
/// theme: Theme
/// custom_css: String
/// child: Option<Box<dyn Widget>>
/// menubar: Option<MenuBar>
/// listener: Option<Box<dyn WindowListener>>
/// ```
/// 
/// # Default values
///
/// ```text
/// title: "Untitled".to_string(),
/// width: 640
/// height: 480
/// resizable: false
/// debug: false
/// theme: Theme::Default
/// custom_css: "".to_string()
/// child: None
/// menubar: None
/// listener: None
/// ```
///
/// ## Example
///
/// ```
/// use neutrino::{Window, App};
/// 
/// fn main() {
///     let mut my_window = Window::new();
///     my_window.set_title("Title");
///     my_window.set_size(800, 600);
///     my_window.set_resizable();
/// 
///     // App::run(window);
/// }
/// ```
pub struct Window {
    title: String,
    width: i32,
    height: i32,
    resizable: bool,
    debug: bool,
    theme: Theme,
    custom_css: String,
    child: Option<Box<dyn Widget>>,
    menubar: Option<MenuBar>,
    listener: Option<Box<dyn WindowListener>>,
}

impl Window {
    /// Create a Window
    pub fn new() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 640,
            height: 480,
            resizable: false,
            debug: false,
            theme: Theme::Default,
            custom_css: "".to_string(),
            child: None,
            menubar: None,
            listener: None,
        }
    }

    /// Set the child
    pub fn set_child(&mut self, widget: Box<dyn Widget>) {
        self.child = Some(widget);
    }

    /// Set the menubar
    pub fn set_menubar(&mut self, menubar: MenuBar) {
        self.menubar = Some(menubar);
    }

    /// Set the title
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// Set the size (width and height)
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    /// Set the resizable flag to true
    pub fn set_resizable(&mut self) {
        self.resizable = true;
    }

    /// Set the debug flag to true
    pub fn set_debug(&mut self) {
        self.debug = true;
    }

    /// Set the theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Set the custom CSS
    pub fn set_custom_css(&mut self, css: &str) {
        self.custom_css = css.to_string();
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn WindowListener>) {
        self.listener = Some(listener);
    }

    /// Render the menubar and widget tree
    fn render(&self, webview: &mut WebView<&str>) -> WVResult {
        let rendered = format!(
            r#"render("<div id=\"app\">{}</div>")"#,
            self.eval().replace(r#"""#, r#"\""#)
        );
        webview.eval(&rendered)
    }

    /// Return the HTML representation of the menubar and the widget tree
    fn eval(&self) -> String {
        match (&self.menubar, &self.child) {
            (Some(menubar), Some(child)) => format!("{}{}", menubar.eval(), child.eval()),
            (None, Some(child)) => format!("{}", child.eval()),
            (Some(menubar), None) => format!("{}", menubar.eval()),
            (None, None) => "".to_string(),
        }
    }

    /// Trigger the events in the widget tree
    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Change { source: _, value: _ } | Event::Update | Event::Undefined => {
                match (&mut self.menubar, &mut self.child) {
                    (Some(menubar), Some(child)) => {
                        menubar.trigger(event);
                        child.trigger(event);
                    },
                    (None, Some(child)) => child.trigger(event),
                    (Some(menubar), None) => menubar.trigger(event),
                    (None, None) => (),
                };
            },
            Event::Key { key } => {
                match &self.listener {
                    None => (),
                    Some(listener) => {
                        listener.on_key(*key);
                    }
                };
                match (&mut self.menubar, &mut self.child) {
                    (Some(menubar), Some(child)) => {
                        menubar.trigger(event);
                        child.trigger(event);
                    },
                    (None, Some(child)) => child.trigger(event),
                    (Some(menubar), None) => menubar.trigger(event),
                    (None, None) => (),
                };
            },
        }
    }
}

/// Return the HTML style tag
fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

/// Return the HTML script tag
fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
