pub mod IO;
use vm::Value;
use std;

pub trait Library {
	fn call(&self, String) -> Value;
}

impl std::fmt::Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("<Library>");
    }
}