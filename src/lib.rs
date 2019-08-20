//! # Neutrino
//!
//! Neutrino is a GUI library built onto [web-view](https://docs.rs/web-view). 
//! As such, it uses the native web component of the host system. Neutrino is
//! created with the idea of using the Model-View-Controller pattern used in
//! native GUI libraries.
//!
//! # Styling
//!
//! In order to accomodate the taste of the user, Neutrino is themable in CSS.
//! Basic widgets are already available and the Custom widget can be used with
//! a custom HTML template.
//!
//! # Examples
//!
//! TODO

use web_view::*;

pub mod utils;
pub mod widgets;

use utils::event::{Event, Key};
use utils::theme::Theme;
use widgets::widget::Widget;
use widgets::menubar::MenuBar;
use utils::listener::Listener;

use serde_json::Value;

/// # App
///
/// An abstract application.
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
                <body onkeydown="{key}" onclick="{click}">
                    <div id="app" class="{theme}"></div>
                    {scripts}
                </body>
            </html>
            "#,
            styles = format!(
                "{}\n{}\n",
                inline_style(include_str!(concat!(env!("OUT_DIR"), "/app.css"))),
                inline_style(include_str!(concat!(env!("OUT_DIR"), "/breeze.css"))),
            ),
            scripts = format!(
                "{}\n{}\n",
                inline_script(include_str!("www/morphdom.min.js")),
                inline_script(include_str!("www/app.js"))
            ),
            theme = window.theme.class(),
            key = Event::key_js(),
            click = Event::undefined_js(),
        );

        let title = &window.title.to_owned();
        let width = window.width;
        let height = window.height;
        let resizable = window.resizable;

        let webview = web_view::builder()
            .title(title)
            .content(Content::Html(html))
            .size(width, height)
            .resizable(resizable)
            .user_data("")
            .debug(false)
            .invoke_handler(|webview, arg| {
                let event: Event = match serde_json::from_str::<Value>(arg) {
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
    }
}

/// # Window
///
/// A window containing the widgets.
///
/// ## Fields
///
/// ```text
/// pub struct Window {
///     title: String,
///     width: i32,
///     height: i32,
///     resizable: bool,
///     theme: Theme,
///     child: Option<Box<Widget>>,
/// }
/// ```
///
/// ## Example
///
/// ```text
/// let my_window = Window::new(my_widget)
///     .title("Title")
///     .size(800, 600)
///     .resizable(true);
/// ```
pub struct Window {
    title: String,
    width: i32,
    height: i32,
    resizable: bool,
    theme: Theme,
    child: Option<Box<Widget>>,
    menubar: Option<MenuBar>,
    listener: Option<Box<Listener>>,
}

impl Window {
    /// Create a Window
    ///
    /// # Default values
    ///
    /// ```text
    /// title: "Untitled".to_string(),
    /// width: 640,
    /// height: 480,
    /// resizable: true,
    /// theme: Theme::Breeze,
    /// child: Some(widget),
    /// ```
    pub fn new() -> Self {
        Window {
            title: "Untitled".to_string(),
            width: 640,
            height: 480,
            resizable: true,
            theme: Theme::Breeze,
            child: None,
            menubar: None,
            listener: None,
        }
    }

    pub fn child(self, widget: Box<Widget>) -> Self {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            theme: self.theme,
            child: Some(widget),
            menubar: self.menubar,
            listener: self.listener,
        }
    }

    pub fn menubar(self, menubar: MenuBar) -> Self {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            theme: self.theme,
            child: self.child,
            menubar: Some(menubar),
            listener: self.listener,
        }
    }

    /// Set the title
    pub fn title(self, title: &str) -> Self {
        Window {
            title: title.to_string(),
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            theme: self.theme,
            child: self.child,
            menubar: self.menubar,
            listener: self.listener,
        }
    }

    /// Set the size
    pub fn size(self, width: i32, height: i32) -> Self {
        Window {
            title: self.title,
            width: width,
            height: height,
            resizable: self.resizable,
            theme: self.theme,
            child: self.child,
            menubar: self.menubar,
            listener: self.listener,
        }
    }

    /// Set the resizable flag
    pub fn resizable(self, resizable: bool) -> Self {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: resizable,
            theme: self.theme,
            child: self.child,
            menubar: self.menubar,
            listener: self.listener,
        }
    }

    /// Set the theme
    pub fn theme(self, theme: Theme) -> Self {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            theme: theme,
            child: self.child,
            menubar: self.menubar,
            listener: self.listener,
        }
    }

    /// Set the listener
    pub fn listener(self, listener: Box<Listener>) -> Self {
        Window {
            title: self.title,
            width: self.width,
            height: self.height,
            resizable: self.resizable,
            theme: self.theme,
            child: self.child,
            menubar: self.menubar,
            listener: Some(listener),
        }
    }

    /// Render the menubar and widget tree
    fn render(&self, webview: &mut WebView<&str>) -> WVResult {
        let rendered = format!(
            r#"render("<div id=\"app\" class=\"{}\">{}</div>")"#,
            self.theme.class(),
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
            (None, None) => "<div></div>".to_string(),
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
