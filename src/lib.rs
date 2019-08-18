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

use utils::event::Event;
use utils::theme::Theme;
use widgets::widget::Widget;

/// # App
///
/// An abstract application.
///
/// ## Example
///
/// ```
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
                <body>
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
            .debug(true)
            .invoke_handler(|webview, arg| {
                let event: Event = serde_json::from_str(arg).unwrap();
                window.trigger(&event);
                let update_event = Event::new("update", "app", "app");
                window.trigger(&update_event);
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
/// ```
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
/// ```
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
}

impl Window {
    /// Create a Window
    ///
    /// # Default values
    ///
    /// ```
    /// title: "Untitled".to_string(),
    /// width: 640,
    /// height: 480,
    /// resizable: true,
    /// theme: Theme::Breeze,
    /// child: Some(widget),
    /// ```
    pub fn new(widget: Box<Widget>) -> Self {
        Window {
            title: "Untitled".to_string(),
            width: 640,
            height: 480,
            resizable: true,
            theme: Theme::Breeze,
            child: Some(widget),
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
        }
    }

    /// Render the widget tree
    fn render(&self, webview: &mut WebView<&str>) -> WVResult {
        let tree = &format!(r#"render("{}")"#, &self.eval().replace(r#"""#, r#"\""#));
        webview.eval(tree)
    }

    /// Return the HTML representation of the widget tree
    fn eval(&self) -> String {
        match &self.child {
            Some(child) => format!("{}", child.eval()),
            None => "{{}}".to_string(),
        }
    }

    /// Trigger the events in the widget tree
    fn trigger(&mut self, event: &Event) {
        match &mut self.child {
            Some(child) => child.trigger(event),
            None => (),
        };
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
