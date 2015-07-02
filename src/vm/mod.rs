#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Ins {
	// name, right
	ASSIGN,

	// value
	LITERAL,

	// name
	NAME,

	// left, right, name (the operator)
	MATH,

	// left (true), right (false), center (the if-statement)
	IF,

	// Nothign, indicates an empty instruction in case of failure
	IGNORE,

	// left (the class to push), right (what comes after)
	PUSH_CLASS,

	// left (the method name), right (the parameters)
	CALL,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Instruction {
	pub instruction: Ins,
	pub name: String,
	pub value: Value,
	pub left: Vec<Instruction>,
	pub right: Vec<Instruction>,
	pub center: Vec<Instruction>,
}

impl Instruction {
	pub fn new(ins: Ins) -> Instruction {
		Instruction {
			instruction: ins,
			name: "".to_string(),
			value: Value::null(),
			left: Vec::new(),
			right: Vec::new(),
			center: Vec::new(),
		}
	}
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Type {
	NULL,
	STRING,
	NUMBER,
}

#[derive(Debug)]
#[derive(Clone)]
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

	pub fn number(num: f64) -> Value {
		let mut s = Value::new(Type::NUMBER);
		s.Number = num;
		s
	}
}