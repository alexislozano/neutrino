use std::env;
use rsass::{compile_scss_file, OutputStyle};
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Theming commons
    compile_write(
        "src/www/app.scss",
        &format!("{}/app.css", out_dir)
    );

    // Breeze theme
    compile_write(
        "src/www/breeze.scss",
        &format!("{}/breeze.css", out_dir)
    );
}

fn compile_write(path: &str, out: &str) {
    let css = compile_scss_file(
        path.as_ref(),
        OutputStyle::Compressed
    ).unwrap();

    fs::write(out, css).unwrap();
}
