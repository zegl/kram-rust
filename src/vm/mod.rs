#[derive(Debug)]
pub enum Ins {
	// name, right
	ASSIGN,

	// value
	LITERAL,

	// name
	NAME,
}

#[derive(Debug)]
pub struct Instruction {
	instruction: Ins,
	pub name: String,
	pub value: Value,
	pub right: Vec<Instruction>
}

impl Instruction {
	pub fn new(ins: Ins) -> Instruction {
		Instruction {
			instruction: ins,
			name: "".to_string(),
			value: Value::null(),
			right: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub enum Type {
	NULL,
	STRING,
	NUMBER,
}

#[derive(Debug)]
pub struct Value {
	Type: Type,
	String:  String,
	Number: f64,
}

impl Value {
	fn new(t: Type) -> Value {
		let mut res = Value::null();
		res.Type = t;
		res
	}

	fn null() -> Value {
		Value {
			Type: Type::NULL,
			String: "".to_string(),
			Number: 0.0,
		}
	}

	pub fn string(val: String) -> Value {
		let mut s = Value::new(Type::STRING);
		s.String = val;
		s
	}
}