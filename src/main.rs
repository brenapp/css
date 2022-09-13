use std::env;
use std::process;

mod css;
mod minimize;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Display the help
    if args.len() < 2 {
        println!("minify");
        println!("Quickly minimize CSS files\n");

        println!("Specify one or more CSS files");

        println!("Example:");
        println!(" minify main.css");

        process::exit(0);
    }

    for file in &args[1..] {
        // Split the file into all it's parts
        let mut split: Vec<&str> = file.split(".").collect();

        // Get the extension
        let ext = match split.pop() {
            Some(x) => x,
            None => {
                eprintln!("Could not parse filename {}", file);
                process::exit(1);
            }
        };

        // Handle files by ext
        let result = match ext {
            "css" => minimize::css(file),
            _ => {
                eprintln!(
                    "Invalid file extension in \"{}\". Make sure passed files has the .css extension.",
                    file
                );
                process::exit(1);
            }
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                eprint!("{}", e);
                process::exit(1)
            }
        }
    }
}
