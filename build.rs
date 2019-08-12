use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("out : {}", out_dir);

    Command::new("sass").args(&["src/www/test.scss", &format!("{}/test.css", out_dir)])
        .status().unwrap();
}