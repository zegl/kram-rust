use std::collections::HashMap;



#[derive(Debug)]
enum Type {
	STRING,
	NUMBER,
}

struct Value {
	t: Type,
	string:  String,
	nr: f64,
}

impl Value {
	fn new() -> Value {
		Value {
			t: Type::STRING,
			string: "Hej".to_string(),
			nr: 0.0,
		}
	}
}

#[derive(Debug)]
enum Instruction_Type {
	SET,
	GET,
	DEBUG,
}

struct Instruction {
	instruction: Instruction_Type,
	name: String,
	value: Value,
}

impl Instruction {
	fn new(ins: Instruction_Type) -> Instruction {
		Instruction {
			instruction: ins,
			name: "".to_string(),
			value: Value::new(),
		}
	}

	fn set(name: String, val: Value) -> Instruction {
		let mut ins = Instruction::new(Instruction_Type::SET);
		ins.name = name;
		ins.value = val;
		return ins;
	}

	fn get(name: String) -> Instruction {
		let mut ins = Instruction::new(Instruction_Type::GET);
		ins.name = name;
		return ins;
	}

	fn debug(name: String) -> Instruction {
		let mut ins = Instruction::new(Instruction_Type::DEBUG);
		ins.name = name;
		return ins;
	}
}

struct VM {
	env: HashMap<String, Value>,
}

impl VM {
	fn new() -> VM {
		VM {
			env: HashMap::new(),
		}
	}

	fn run(&mut self, instructions: Vec<Instruction>) {
		for ins in instructions {
			self.operation(ins)
		}
	}

	fn operation(&mut self, ins: Instruction) {
		match ins.instruction {
			Instruction_Type::SET => self.set(ins),
			Instruction_Type::GET => self.get(ins),
			Instruction_Type::DEBUG => self.debug(ins),
		}
	}

	fn get(&self, ins: Instruction) {
		println!("GET {}", ins.name);
	}

	fn set(&mut self, ins: Instruction) {
		println!("SET {} to {}", ins.name, ins.value.string);
		self.env.insert(ins.name, ins.value);
	}

	fn debug(&self, ins: Instruction) {
		match self.env.get(&ins.name) {
	        Some(value) => println!("{}: {}", ins.name, value.string),
	        None => println!("{} is not set.", ins.name)
	    }
	}
}

/*let instructions = vec![
		Instruction::set("abc".to_string(), Value::new()),
		Instruction::debug("abc".to_string()),
	];

	let mut vm = VM::new();
	vm.run(instructions)*/
