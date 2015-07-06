use libraries;
use std::iter;
use std::collections::HashMap;

pub struct VM {
	env: Environment,

	// Pushed classes
	classes: Vec<Value>,
}

impl VM {
	pub fn new() -> VM {
		let mut vm = VM {
			env: Environment::new(),
			classes: Vec::new(),
		};

		vm.add_objects();

		vm
	}

	pub fn run(&mut self, instructions: Vec<Instruction>) {
		self.block(instructions);
	}

	fn add_objects(&mut self) {
		let io = libraries::io::IO::init();
		self.env.set("IO".to_string(), Value::object(io));
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
		self.env.set(instruction.name, val);

		// Returns nothing
		Value::null()
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
				panic!("No such class, {:?}", name);
			}

			let class = self.env.get(name);

			// Push to the stack
			//self.classes.push(class);
		}

		let res = self.ins(instruction.right[0].clone());

		// Remove class from stack
		//self.classes.pop();

		// Return
		res
	}

	fn call(&mut self, instruction: Instruction) -> Value {

		// Get top class

		//let len = self.classes.len();
		//let class = self.classes[len - 1];

		// class.



		// All calls are basiacally the print method for now...

		/*for echo in instruction.right {
			let val = self.env.get(echo.name);
			println!("{:?}, {:?}, {:?}", val.Type, val.Number, val.String);
		}*/

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
		let len = self.data.len();

		for i in 0..len {

			// There probably is some better way of looping in reverse. Somehow.
			let i = len - i - 1;

			match self.data[i].get(&key) {
				Some(entry) => return entry,
				None => continue,
			}
		}

		panic!("No such variable, {:?}", key);
	}

	fn exists(&self, key: String) -> bool {

		println!("EXISTS: {:?}", key);

		let len = self.data.len();

		for i in 0..len {

			// There probably is some better way of looping in reverse. Somehow.
			let i = len - i - 1;

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
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Type {
	NULL,
	STRING,
	NUMBER,
	REFERENCE,
}

#[derive(Clone)]
pub struct Value {
	pub Type: Type,
	pub String: String,
	pub Number: f64,
	pub Object: Object,
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
			Object: Object::new("Unknown"),
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

	pub fn object(obj: Object) -> Value {
		let mut s = Value::new(Type::REFERENCE);
		s.Object = obj;
		s
	}
}

pub struct Object  {
    Name: String,
    Variables: HashMap<String, Value>,
    Methods: HashMap<String, Box<Fn(Value)>>,
}

impl Object {
    pub fn new(name: &str) -> Object {
        Object {
            Name: name.to_string(),
            Variables: HashMap::new(),
            Methods: HashMap::new(),
        }
    }
    
    fn add_fn(&mut self, name: String, func: Box<Fn(Value)>) {
        self.Methods.insert(name, func);
    }
    
    fn exec(&mut self, name: String, input: Value) {
        match self.Methods.get(&name) {
			Some(method) => method(input),
			None => return,
		}
    }
}

impl Clone for Object {
    fn clone(&self) -> Object {
        Object::new("Unknown")
    }
}