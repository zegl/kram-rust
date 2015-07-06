use vm::{Object, Value};

pub struct IO;

impl IO {
    pub fn init() -> Object {
       let mut io = Object::new("IO");

       io
    }

    fn println(input: Value) {
        println!("Printing: {:?}, {:?}", input.Type, input.String);
    }
}