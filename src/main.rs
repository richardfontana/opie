use std::fs;

fn main() {
    match fs::create_dir("_site") {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn version() -> &'static str {
    return env!("CARGO_PKG_VERSION");
}
