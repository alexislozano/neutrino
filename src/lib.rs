use web_view::*;

pub mod widget;
pub mod button;
pub mod container;
pub mod label;
pub mod utils;

use widget::Widget;
use utils::Event;

pub struct App {
    title: String,
    width: i32,
    height: i32,
    resizable: bool,
}

impl App {
    pub fn new() -> Self {
        App { 
            title: "Untitled".to_string(), 
            width: 640, 
            height: 480, 
            resizable: true,
        }
    }

    pub fn title(self, title: &str) -> Self {
        App { 
            title: title.to_string(), 
            width: self.width, 
            height: self.height, 
            resizable: self.resizable, 
        }
    }

    pub fn size(self, width: i32, height: i32) -> Self {
        App { 
            title: self.title, 
            width: width, 
            height: height, 
            resizable: self.resizable,
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        App { 
            title: self.title, 
            width: self.width, 
            height: self.height, 
            resizable: resizable,
        }
    }

    pub fn run(&self, mut window: Window) {
    let html = format!(r#"
        <!doctype html>
        <html>
            <head>
                <meta charset="UTF-8">
                <link href="https://fonts.googleapis.com/css?family=Noto+Sans&display=swap" rel="stylesheet">
                {styles}
                {libraries}
            </head>
            <body>
                <div id="app"></div>
                {scripts}
            </body>
        </html>
        "#, 
        styles = inline_style(include_str!("www/app.css")),
        libraries = inline_script(include_str!("www/superfine.js")),
        scripts = inline_script(include_str!("www/widgets.js")) +
            &inline_script(include_str!("www/app.js"))
    );
    let webview = web_view::builder()
        .title(&self.title)
        .content(Content::Html(html))
        .size(self.width, self.height)
        .resizable(self.resizable)
        .user_data("")
        .debug(true)
        .invoke_handler(|webview, arg| {
            let event: Event = serde_json::from_str(arg).unwrap();
            window.trigger(&event);
            let update_event = Event::new("update", "app");
            window.trigger(&update_event);
            window.render(webview)
        })
        .build()
        .unwrap();

    webview.run().unwrap();
}
}

pub struct Window {
    child: Option<Box<Widget>>
}

impl Window {
    pub fn new() -> Self {
        Window { child: None }
    }

    pub fn add(&mut self, widget: Box<Widget>) {
        self.child = Some(widget);
    }

    fn render(&self, webview: &mut WebView<&str>) -> WVResult {
        let tree = &format!("render({})", &self.eval());
        webview.eval(tree)
    }

    fn eval(&self) -> String {
        match &self.child {
            Some(child) => format!("{}", child.eval()),
            None => "{{}}".to_string(),
        }
    }

    fn trigger(&mut self, event: &Event) {
        match &mut self.child {
            Some(child) => child.trigger(event),
            None => (),
        };
    }
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

