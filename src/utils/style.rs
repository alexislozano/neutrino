use rsass::{compile_scss, output::{Format, Style}};

/// Transform SCSS into CSS
pub fn scss_to_css(style: &str) -> String {
    match compile_scss(style.as_bytes(), Format { style: Style::Compressed,  precision: 5 }) {
        Ok(css) => match std::str::from_utf8(&css) {
            Ok(css) => css.to_string(),
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    }
    .replace("\n", "")
}

/// Return the HTML style tag
pub fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

/// Return the HTML script tag
pub fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
