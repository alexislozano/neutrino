use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Theming commons
    Command::new("cmd").args(&[
        "/c",
        "sass",
        "src/www/app.scss", 
        &format!("{}/app.css", out_dir)
    ]).status().unwrap();

    // Breeze theme
    Command::new("cmd").args(&[
        "/c",
        "sass",
        "src/www/breeze.scss", 
        &format!("{}/breeze.css", out_dir)
    ]).status().unwrap();
}