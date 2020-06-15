use std::fs;
use std::io::Error;
use std::process;

use crate::css;

pub fn css(file: &str) -> Result<(), Error> {
    let contents = match fs::read_to_string(file) {
        Ok(str) => str,
        Err(e) => return Err(e),
    };

    let optimized = match css::optimize(contents) {
        Ok(opt) => opt,
        Err(e) => {
            eprintln!("Error parsing file {}!", file);
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Add min to file
    let mut split: Vec<&str> = file.split(".").collect();
    split.insert(split.len() - 1, "min");

    let out_file = split.join(".");

    fs::write(out_file, optimized)
}
