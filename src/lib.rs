//! # Preamble
//!
//! [Docs](https://docs.rs/neutrino) |
//! [Repo](https://github.com/alexislozano/neutrino) |
//! [Wiki](https://github.com/alexislozano/neutrino/wiki) |
//! [Crate](https://crates.io/crates/neutrino)
//!
//! Neutrino is a MVC GUI framework written in Rust. It lets users create GUI
//! applications by positioning widgets on a window and by handling events.
//! Neutrino is based on the [web-view](https://crates.io/crates/web-view) 
//! crate provided by Boscop. As such, Neutrino renders the application using 
//! web technologies as HTML and CSS.
//!
//! As it is based on web-view, Neutrino does not embed a whole web browser.
//! So don't worry, due to the very lightweight footprint of web-view, you 
//! won't have to buy more memory for your computer.
//!
//! # Install
//!
//! In order to use Neutrino, you will have to use cargo. Just add the
//! following line to your `Cargo.toml` and you'll be done :
//!
//! ```text
//! neutrino = "<last_version>"
//! ```
//!
//! # Examples
//!
//! ![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/
//! image_viewer/3.png)
//!
//! ![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/
//! styling/3.png)
//!
//! ![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/
//! styling/4.png)
//!
//! ![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/
//! styling/5.png)
//!
//! ![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/
//! styling/6.png)

use web_view::*;

pub mod utils;
pub mod widgets;

use utils::event::{Event, Key};
use utils::style::{inline_script, inline_style, scss_to_css};
use utils::theme::Theme;
use widgets::menubar::MenuBar;
use widgets::widget::Widget;

use html_minifier::HTMLMinifier;
use json;
use std::collections::HashSet;

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
        let title = &window.title.to_owned();
        let width = window.width;
        let height = window.height;
        let resizable = window.resizable;
        let debug = window.debug;

        let context = if debug {
            ""
        } else {
            r#"(function() { event.preventDefault(); } )()"#
        };

        let timer = match window.timer {
            None => "".to_string(),
            Some(period) => {
                format!(r#"<script>{}</script>"#, Event::tick_js(period))
            }
        };

        let html = format!(
            r#"
            <!doctype html>
            <html>
                <head>
                    <meta charset="UTF-8">
                    {styles}
                </head>
                <body 
                    onkeydown="{keydown}"
                    onkeyup="{keyup}"
                    onclick="{click}" 
                    oncontextmenu="{context}"
                >
                    <div id="app"></div>
                    {scripts}
                    {timer}
                </body>
            </html>
            "#,
            styles = format!(
                "{}\n{}\n{}\n",
                inline_style(include_str!(concat!(
                    env!("OUT_DIR"),
                    "/app.css"
                ))),
                inline_style(&window.theme.css()),
                inline_style(&window.style),
            ),
            scripts = format!(
                "{}\n{}\n",
                inline_script(include_str!("www/app/morphdom.min.js")),
                inline_script(include_str!("www/app/app.js"))
            ),
            keydown = Event::keypress_js("app", "down"),
            keyup = Event::keypress_js("app", "up"),
            click = Event::change_js("app", "''"),
            context = context,
            timer = timer,
        );

        let webview = web_view::builder()
            .title(title)
            .content(Content::Html(html))
            .size(width, height)
            .resizable(resizable)
            .user_data("")
            .debug(debug)
            .invoke_handler(|webview, arg| {
                let event: Event = match json::parse(arg) {
                    Ok(value) => match value["type"].as_str() {
                        Some(type_str) => match type_str {
                            "Update" => Event::Update,
                            "Tick" => Event::Tick,
                            "Keypress" => {
                                let (key_str, source_str, state_str) = (
                                    value["key"].as_str(),
                                    value["source"].as_str(),
                                    value["state"].as_str(),
                                );
                                match (key_str, source_str, state_str) {
                                    (
                                        Some(key_str), 
                                        Some(source_str), 
                                        Some(state_str)
                                    ) => match Key::new(key_str) {
                                        Some(key) => {
                                            if state_str == "up" {
                                                window.keys.remove(&key);
                                            } else {
                                                window.keys.insert(key);
                                            }
                                            let s = source_str.to_string();
                                            Event::Keypress {
                                                source: s,
                                                keys: window.keys.clone(),
                                            }
                                        },
                                        None => Event::Undefined,
                                    },
                                    _ => Event::Undefined
                                }
                            }
                            "Change" => {
                                let (source_str, value_str) = (
                                    value["source"].as_str(), 
                                    value["value"].as_str()
                                );
                                match (source_str, value_str) {
                                    (
                                        Some(source_str), 
                                        Some(value_str)
                                    ) => Event::Change {
                                        source: source_str.to_string(),
                                        value: value_str.to_string(),
                                    },
                                    _ => Event::Undefined,
                                }
                            }
                            _ => Event::Undefined,
                        },
                        None => Event::Undefined,
                    },
                    Err(_) => Event::Undefined,
                };
                match event {
                    Event::Undefined => (),
                    _ => {
                        window.trigger(&event);
                        window.trigger(&Event::Update);
                    }
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
    /// Function triggered on keyup and keydown events
    fn on_keys(&self, _keys: HashSet<Key>);

    /// Function triggered on tick event
    fn on_tick(&self);
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
/// style: String
/// child: Option<Box<dyn Widget>>
/// menubar: Option<MenuBar>
/// listener: Option<Box<dyn WindowListener>>
/// timer: Option<u32>
/// keys: HashSet<Key>
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
/// style: "".to_string()
/// child: None
/// menubar: None
/// listener: None
/// timer: None
/// keys: HashSet::new()
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
    style: String,
    child: Option<Box<dyn Widget>>,
    menubar: Option<MenuBar>,
    listener: Option<Box<dyn WindowListener>>,
    timer: Option<u32>,
    keys: HashSet<Key>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 640,
            height: 480,
            resizable: false,
            debug: false,
            theme: Theme::Default,
            style: "".to_string(),
            child: None,
            menubar: None,
            listener: None,
            timer: None,
            keys: HashSet::new(),
        }
    }
}

impl Window {
    /// Create a Window
    pub fn new() -> Self {
        Default::default()
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

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = scss_to_css(style);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn WindowListener>) {
        self.listener = Some(listener);
    }

    /// Set the timer
    ///
    /// The app will send a Tick event with a defined period
    pub fn set_timer(&mut self, period: u32) {
        self.timer = Some(period);
    }

    /// Render the menubar and widget tree
    fn render(&self, webview: &mut WebView<&str>) -> WVResult {
        let rendered = format!(
            r#"render("<div id=\"app\">{}</div>")"#,
            self.eval().replace(r#"""#, r#"\""#)
        );
        let mut html_minifier = HTMLMinifier::new();
        webview.eval(&match html_minifier.digest(rendered) {
            Ok(_) => {
                let html = html_minifier.get_html();
                println!("{}", &html);
                html
            },
            Err(_) => {
                println!("error");
                "".to_string()
            },
        })
    }

    /// Return the HTML representation of the menubar and the widget tree
    fn eval(&self) -> String {
        match (&self.menubar, &self.child) {
            (Some(menubar), Some(child)) => {
                format!("{}{}", menubar.eval(), child.eval())
            }
            (None, Some(child)) => child.eval().to_string(),
            (Some(menubar), None) => menubar.eval().to_string(),
            (None, None) => "".to_string(),
        }
    }

    /// Trigger the events in the widget tree
    fn trigger(&mut self, event: &Event) {
        if self.debug {
            println!("{:?}", event);
        }
        match (&mut self.menubar, &mut self.child) {
            (Some(menubar), Some(child)) => {
                menubar.trigger(event);
                child.trigger(event);
            }
            (None, Some(child)) => child.trigger(event),
            (Some(menubar), None) => menubar.trigger(event),
            (None, None) => (),
        };
        match &self.listener {
            Some(listener) => match event {
                Event::Tick => listener.on_tick(),
                Event::Keypress { source, keys } => {
                    if source == "app" {
                        listener.on_keys(keys.clone());
                    }
                }
                _ => (),
            },
            None => (),
        }
    }
}
