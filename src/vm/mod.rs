use std::collections::HashMap;

pub struct VM {
	env: HashMap<String, Value>,
}

impl VM {
	pub fn new() -> VM {
		VM {
			env: HashMap::new(),
		}
	}

	pub fn run(&mut self, instructions: Vec<Instruction>) {
		self.block(instructions);
	}

	fn ins(&mut self, instruction: Instruction) -> Value {

			println!("{:?}", instruction.instruction);

			match instruction.instruction {
				Ins::ASSIGN => self.assign(instruction),
				Ins::LITERAL => self.literal(instruction),
				Ins::PUSH_CLASS => self.push_class(instruction),
				Ins::CALL => self.call(instruction),
				_ => panic!("Unknown instruction: {:?}", instruction.instruction),
			}
	}

	fn block(&mut self, instructions: Vec<Instruction>) -> Value {
		for ins in instructions {
			self.ins(ins);
		}

		Value::null()
	}

	fn assign(&mut self, instruction: Instruction) -> Value {
		let val = self.ins(instruction.right[0].clone());

		self.env.insert(instruction.name, val.clone());

		val
	}

	fn literal(&mut self, instruction: Instruction) -> Value {
		instruction.value
	}

	fn push_class(&mut self, instruction: Instruction) -> Value {

		// TODO use instruction.left to actually push the class

		self.ins(instruction.right[0].clone())
	}

	fn call(&mut self, instruction: Instruction) -> Value {

		// All calls are basiacally the print method for now...

		for echo in instruction.right {
			match self.env.get(&echo.name) {
				Some(entry) => println!("{:?} = {:?}", echo.name, entry),
				None => println!("NOT SET: {:?}", echo.name),
			}
		}

		Value::null()
	}
}

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

	pub fn null() -> Value {
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