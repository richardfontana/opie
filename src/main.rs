extern crate getopts;
use getopts::Options;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("", "source-dir", "Location of sources (default: .)", "DIR");
    opts.optopt("", "output-dir",
                "Location to output generated site (default: _site)", "DIR");
    opts.optflag("h", "help", "Print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let od = matches.opt_str("output-dir");
    let sd = matches.opt_str("source-dir");
    let mut src_dir = PathBuf::from(env::current_dir().unwrap());
    match sd {
        Some(sds) => {
            src_dir = PathBuf::from(sds);
        },
        None => {}
    }
    let mut out_dir = src_dir.clone();
    out_dir.push("_site");
    match od {
        Some(ods) => {
            let ods_slice = &ods[..];
            if PathBuf::from(ods_slice).is_absolute() {
                out_dir = PathBuf::from(ods_slice);
            } else {
                out_dir.push(ods_slice);
            }
        },
        None => {},
    }
}    

