mod lexer;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 0 {   
        let path = args[1].clone();

        // This somehows converts a String to a &str ...
        // Can be replaced with .as_str() once it is stable
        let path : &str = path.trim();

        let source = read_from_file(path);

        let mut lexer = lexer::Lexer::new();

        lexer.run(source)
    } else {
        println!("You need to run kram with a file");
        println!("$ cargo run test.kr");
    }
}

fn read_from_file(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");

    file_contents
}