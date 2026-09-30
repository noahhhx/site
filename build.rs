//! Renders every `content/*.md` to `$OUT_DIR/<name>.html`, for pages to pull
//! in with `include_str!(concat!(env!("OUT_DIR"), "/<name>.html"))`.

use std::{env, fs, path::Path};

use pulldown_cmark::{Options, Parser, html};

const OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_SMART_PUNCTUATION);

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo::rerun-if-changed=content");

    for entry in fs::read_dir("content").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        let markdown = fs::read_to_string(&path).unwrap();
        let mut rendered = String::new();
        html::push_html(&mut rendered, Parser::new_ext(&markdown, OPTIONS));

        let name = path.file_stem().unwrap();
        fs::write(Path::new(&out_dir).join(name).with_extension("html"), rendered).unwrap();
    }
}
