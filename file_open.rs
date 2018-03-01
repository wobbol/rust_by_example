//file signature %PDF

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "hello.txt";
    if args.len() > 1 {
        filename = &args[1];
    }
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}",
            display,
            why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}",
            display,
            why.description()),
        Ok(_) => print!("{} contains:\n{}",
            display,
            s),
    }
}
