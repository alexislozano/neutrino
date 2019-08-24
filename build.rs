use std::env;
use rsass::{compile_scss_file, OutputStyle};
use std::fs;
use base64::encode;

fn main() {
    let out = &env::var("OUT_DIR").unwrap();

    // Theming commons
    scss("app", out);

    // Breeze theme
    scss("breeze", out);
    icons("breeze");
}

fn scss(name: &str, out: &str) {
    let path = format!("src/www/{}/{}.scss", name, name);
    let outdir = format!("{}/{}.css", out, name);
    let css = compile_scss_file(
        path.as_ref(),
        OutputStyle::Compressed
    ).unwrap();
    fs::write(outdir, css).unwrap();
}

fn icons(name: &str) {
    let dir = format!("src/www/{}/icons", name);
    match fs::read_dir(dir) {
        Err(e) => panic!(e),
        Ok(entries) => for entry in entries {
            let path = entry.unwrap().path();
            let data = String::from_utf8_lossy(&fs::read(path).unwrap()).replace("\n", "");
            let b64 = encode(&data);
        } 
    }
}