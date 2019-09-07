use base64::encode;
use rsass::{compile_scss_file, OutputStyle};
use std::env;
use std::fs;
use std::str;

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Commons
    scss("app", out_dir);

    // Themes
    themes(out_dir);

    // Icons
    icons(out_dir);
}

fn scss(name: &str, out_dir: &str) {
    let path = format!("src/www/{}/{}.scss", name, name);
    let out = format!("{}/{}.css", out_dir, name);
    let css =
        compile_scss_file(path.as_ref(), OutputStyle::Compressed).unwrap();
    fs::write(out, css).unwrap();
}

fn themes(out_dir: &str) {
    let mut enum_data = r#"
        /// # A theme
        pub enum Theme {
    "#
    .to_string();
    let mut impl_data = r#"
        impl Theme {
            /// Get a string containing the CSS defining the theme 
            pub fn css(&self) -> &str {
                match self {
    "#
    .to_string();

    fs::create_dir_all(format!("{}/themes", out_dir)).unwrap();
    match fs::read_dir("src/www/themes") {
        Err(e) => panic!(e),
        Ok(entries) => {
            for entry in entries {
                let path = entry.unwrap().path();
                let filestem = path
                    .clone()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let css =
                    compile_scss_file(path.as_ref(), OutputStyle::Compressed)
                        .unwrap();
                let css_str = str::from_utf8(&css).unwrap();
                enum_data.push_str(&format!(r#"{},"#, &filestem));
                impl_data.push_str(&format!(
                    r##"Theme::{} => r#"{}"#,"##,
                    &filestem, css_str
                ))
            }
        }
    };

    enum_data.push_str("}");
    impl_data.push_str("}}}");
    fs::write(format!("{}/themes/enum.rs", out_dir), enum_data).unwrap();
    fs::write(format!("{}/themes/impl.rs", out_dir), impl_data).unwrap();
}

fn icons(out_dir: &str) {
    let mut enum_data = r#""#.to_string();
    let mut impl_data = r#""#.to_string();

    fs::create_dir_all(format!("{}/icons/", out_dir)).unwrap();
    match fs::read_dir("src/www/icons/") {
        Err(e) => panic!(e),
        Ok(dirs) => {
            for dir in dirs {
                let path = dir.unwrap().path();
                let dirstem = path
                    .clone()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                enum_data.push_str(&format!(
                    r#"
                /// # The {} icon set
                pub enum {}Icon {{
            "#,
                    &dirstem, &dirstem
                ));
                impl_data
                    .push_str(&format!(r#"impl Icon for {}Icon {{"#, dirstem));

                let mut impl_function_data = r#"fn data(&self) -> String { 
                match self {
            "#
                .to_string();
                let mut impl_function_extension =
                    r#"fn extension(&self) -> String { 
                match self {
            "#
                    .to_string();

                match fs::read_dir(&path) {
                    Err(e) => panic!(e),
                    Ok(entries) => {
                        for entry in entries {
                            let path = entry.unwrap().path();
                            let path1 = path.clone();
                            let path2 = path.clone();

                            let filestem = path
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();

                            enum_data.push_str(&format!(r#"{},"#, &filestem));

                            let extension =
                                path1.extension().unwrap().to_str().unwrap();
                            let data = encode(
                                &String::from_utf8_lossy(
                                    &fs::read(path2).unwrap(),
                                )
                                .replace("\n", ""),
                            );

                            impl_function_extension.push_str(&format!(
                                r#"{}Icon::{} => "{}".to_string(),"#,
                                dirstem, filestem, &extension
                            ));

                            impl_function_data.push_str(&format!(
                                r#"{}Icon::{} => "{}".to_string(),"#,
                                dirstem, filestem, &data
                            ));
                        }
                    }
                };

                enum_data.push_str(r#"}"#);

                impl_function_data.push_str(r#"}}"#);
                impl_function_extension.push_str(r#"}}"#);

                impl_data.push_str(&impl_function_data);
                impl_data.push_str(&impl_function_extension);
                impl_data.push_str(r#"}"#);
            }
        }
    };
    fs::write(format!("{}/icons/enum.rs", out_dir), enum_data).unwrap();
    fs::write(format!("{}/icons/impl.rs", out_dir), impl_data).unwrap();
}
