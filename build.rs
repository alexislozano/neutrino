use std::env;
use rsass::{compile_scss_file, OutputStyle};
use std::fs;
use base64::encode;

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Theming commons
    theme("app", out_dir);

    // Default theme
    theme("default", out_dir);
    icons("default", out_dir);

    // Breeze theme
    theme("breeze", out_dir);
    icons("breeze", out_dir);
}

fn theme(name: &str, out_dir: &str) {
    fs::create_dir_all(format!("{}/themes", out_dir)).unwrap();
    let path = format!("src/www/themes/{}.scss", name);
    let out = format!("{}/themes/{}.css", out_dir, name);
    let css = compile_scss_file(
        path.as_ref(),
        OutputStyle::Compressed
    ).unwrap();
    fs::write(out, css).unwrap();
}

fn icons(name: &str, out_dir: &str) {
    fs::create_dir_all(format!("{}/icons/{}", out_dir, name)).unwrap();
    let dir = format!("src/www/icons/{}", name);
    match fs::read_dir(dir) {
        Err(e) => panic!(e),
        Ok(entries) => for entry in entries {
            let path = entry.unwrap().path();
            let filestem = path.clone().file_stem().unwrap().to_str().unwrap().to_string();

            let extension = path.extension().unwrap().to_str().unwrap();
            let out_extension = format!("{}/icons/{}/{}.extension", out_dir, name, &filestem);
            fs::write(out_extension, extension).unwrap();

            let data = String::from_utf8_lossy(&fs::read(path).unwrap()).replace("\n", "");
            let b64 = encode(&data);
            let out_data = format!("{}/icons/{}/{}.data", out_dir, name, &filestem);
            fs::write(out_data, b64).unwrap();
        } 
    }
}
