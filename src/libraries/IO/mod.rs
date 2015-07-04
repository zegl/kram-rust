use libraries::Library;
use vm::Value;

#[derive(Debug)]
pub struct IO;

impl IO {
	pub fn new() -> IO {
		IO
	}

	fn println(&self, val: Value) -> Value {
		println!("{:?}", val);
		Value::null()
	}
}

impl Library for IO {
	fn call(&self, method: String) -> Value {
		let method : &str = method.trim();

		match method {
			"println" => self.println(Value::null()),
			_ => panic!("No such method {:?}", method),
		}
	}
}