mod lexer;

use std::error::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
    }

    let source = read_source(args[1]);

    let mut lexer = lexer::Lexer::new();
}

fn read_source(path: String) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}