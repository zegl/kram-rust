use libraries::Library;
use libraries;

use std::collections::HashMap;

pub struct VM {
	env: Environment,
	// libraries: HashMap<String, Box<Library>>,

	// Pushed classes
	classes: Vec<Box<Library>>,
}

impl VM {
	pub fn new() -> VM {
		let mut vm = VM {
			env: Environment::new(),
			// libraries: HashMap::new(),
			classes: Vec::new(),
		};

		// vm.libraries.insert("IO".to_string(), Box::new(libraries::IO::IO::new()));
		vm.env.set("IO".to_string(), Value::reference(Box::new(libraries::IO::IO::new())));

		vm
	}

	pub fn run(&mut self, instructions: Vec<Instruction>) {
		self.block(instructions);
	}

	fn ins(&mut self, instruction: Instruction) -> Value {

			println!("{:?}", instruction.instruction);

			match instruction.instruction {
				Ins::ASSIGN => self.assign(instruction),
				Ins::LITERAL => self.literal(instruction),
				Ins::NAME => self.name(instruction),
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

		self.env.set(instruction.name, val.clone());

		val
	}

	fn literal(&mut self, instruction: Instruction) -> Value {
		instruction.value
	}

	fn name(&mut self, instruction: Instruction) -> Value {
		Value::string(instruction.name)
	}

	fn push_class(&mut self, instruction: Instruction) -> Value {

		let name = self.ins(instruction.left[0].clone());

		if name.Type != Type::STRING {
			panic!("VM::push_class() expected string");
		}
 		
		{
			let name = name.String;

			if !self.env.exists(name.clone()) {
				panic!("No such class!");
			}

			let class = self.env.get(name);

			println!("{:#?}", instruction.left);
		}

		self.ins(instruction.right[0].clone())
	}

	fn call(&mut self, instruction: Instruction) -> Value {

		// All calls are basiacally the print method for now...

		for echo in instruction.right {
			let val = self.env.get(echo.name);
			println!("{:?}, {:?}, {:?}", val.Type, val.Number, val.String);
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

struct Environment {
	data: Vec<HashMap<String, Value>>,
}

impl Environment {
	fn new() -> Environment {
		let mut e = Environment {
			data: Vec::new()
		};

		e.push();

		e
	}

	fn set(&mut self, key: String, val: Value) {
		let i = self.data.len() - 1;
		self.data[i].insert(key, val);
	}

	fn get(&self, key: String) -> &Value {
		let len = self.data.len() - 1;

		for i in len..0 {
			match self.data[i].get(&key) {
				Some(entry) => return entry,
				None => continue,
			}
		}

		panic!("No such variable, {:?}", key);
	}

	fn exists(&self, key: String) -> bool {
		let len = self.data.len() - 1;

		for i in len..0 {
			match self.data[i].get(&key) {
				Some(entry) => return true,
				None => continue,
			}
		}

		false
	}

	fn push(&mut self) {
		self.data.push(HashMap::new())
	}

	fn pop(&mut self) {
		self.data.pop();	
	}
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Type {
	NULL,
	STRING,
	NUMBER,
	REFERENCE,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Value {
	Type: Type,
	String:  String,
	Number: f64,
	Reference: Box<Library>,
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

	pub fn reference(reference: Box<Library>) -> Value {
		let mut s = Value::new(Type::REFERENCE);
		s.Reference = reference;
		s
	}
}