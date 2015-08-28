use std::fs;
use std::path::Path;

fn main() {
    match fs::create_dir("_site") {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
