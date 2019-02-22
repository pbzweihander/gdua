use std::{env, fs::{File}, path::Path, io::{Write, Read}};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let ui_path = Path::new(&manifest_dir).join("../gdua-ui").canonicalize().unwrap();
    let deploy_path = Path::new(&manifest_dir).join("../target/deploy").canonicalize().unwrap();

    println!("cargo:rerun-if-changed={}", ui_path.display());
    println!("cargo:rerun-if-changed={}", deploy_path.display());

    // let mut ui_build_command = Command::new("cargo");
    // ui_build_command.args(&["web", "deploy", "-p", "gdua-ui"]);
    // if !cfg!(debug_assertions) {
    //     ui_build_command.arg("--release");
    // }
    // ui_build_command.status().unwrap();

    let output_index_path = Path::new(&out_dir).join("index.html");
    let index_path = Path::new(&manifest_dir).join("../target/deploy/index.html");
    let css_path = Path::new(&manifest_dir).join("../target/deploy/styles.css");
    let js_path = Path::new(&manifest_dir).join("../target/deploy/gdua-ui.js");

    let mut index_file = File::open(index_path).unwrap();
    let mut css_file = File::open(css_path).unwrap();
    let mut js_file = File::open(js_path).unwrap();

    let mut index = String::new();
    let mut css = String::new();
    let mut js = String::new();

    index_file.read_to_string(&mut index).unwrap();
    css_file.read_to_string(&mut css).unwrap();
    js_file.read_to_string(&mut js).unwrap();

    drop(index_file);
    drop(css_file);
    drop(js_file);

    index = index.replace(r#"<link rel="stylesheet" href="styles.css" />"#, &inline_style(&css));
    index = index.replace(r#"<script src="/gdua-ui.js"></script>"#, &inline_script(&js));

    let mut output_index_file = File::create(output_index_path).unwrap();

    output_index_file.write_all(index.as_bytes()).unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
