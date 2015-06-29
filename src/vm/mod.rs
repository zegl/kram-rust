#[derive(Debug)]
pub enum Ins {
	ASSIGN,
	LITERAL,
}

#[derive(Debug)]
pub struct Instruction {
	instruction: Ins,
	pub name: String,
	//pub value: Value,
	pub right: Vec<Instruction>
}

impl Instruction {
	pub fn new(ins: Ins) -> Instruction {
		Instruction {
			instruction: ins,
			name: "".to_string(),
			//value: Value::new(),
			right: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub enum Type {
	STRING,
	NUMBER,
}

#[derive(Debug)]
struct Value {
	t: Type,
	string:  String,
	nr: f64,
}

impl Value {
	fn new() -> Value {
		Value {
			t: Type::STRING,
			string: "".to_string(),
			nr: 0.0,
		}
	}
}