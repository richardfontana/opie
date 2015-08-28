use std::fs;

fn main() {

    match fs::create_dir("_site") {
        Ok(_) => (),
        Err(_) => (),
    }

}


