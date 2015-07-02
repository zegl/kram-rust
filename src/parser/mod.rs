use lexer::Lexer;
use lexer::Token;
use lexer::Type;
use vm::Instruction;
use vm::Ins;
use vm::Value;
use std::collections::HashMap;

pub struct Parser {
	tokens: Vec<Token>,

	index: usize,
	lenght: usize,
	has_advanced: bool,

	comparisions: HashMap<String, bool>,
	startOperators: HashMap<String, bool>,
	leftOnlyInfix: HashMap<String, bool>,
	rightOnlyInfix: HashMap<String, bool>,

	stack: Stack,
}

impl Parser {
	pub fn new() -> Parser {
		let mut parser = Parser {
			tokens: Vec::new(),
			comparisions: HashMap::new(),
			startOperators: HashMap::new(),
			leftOnlyInfix: HashMap::new(),
			rightOnlyInfix: HashMap::new(),
			stack: Stack::new(),

			index: 0,
			lenght: 0,
			has_advanced: false,
		};

		// Hashmap of comparisions
		parser.comparisions.insert("==".to_string(), true);
		parser.comparisions.insert(">".to_string(), true);
		parser.comparisions.insert(">=".to_string(), true);
		parser.comparisions.insert("<".to_string(), true);
		parser.comparisions.insert("<=".to_string(), true);
		parser.comparisions.insert("&&".to_string(), true);
		parser.comparisions.insert("||".to_string(), true);

		// 123++
		parser.leftOnlyInfix.insert("++".to_string(), true);
		parser.leftOnlyInfix.insert("--".to_string(), true);

		// -123
		parser.rightOnlyInfix.insert("-".to_string(), true);

		// List of all operators starting a new sub-expression
		// Starting off with a clonse of parser.comparisions
		parser.startOperators.insert("==".to_string(), true);
		parser.startOperators.insert(">".to_string(), true);
		parser.startOperators.insert(">=".to_string(), true);
		parser.startOperators.insert("<".to_string(), true);
		parser.startOperators.insert("<=".to_string(), true);
		parser.startOperators.insert("&&".to_string(), true);
		parser.startOperators.insert("||".to_string(), true);
		parser.startOperators.insert("++".to_string(), true);
		parser.startOperators.insert("--".to_string(), true);
		parser.startOperators.insert("-".to_string(), true);
		parser.startOperators.insert("+".to_string(), true);
		parser.startOperators.insert("*".to_string(), true);
		parser.startOperators.insert("/".to_string(), true);
		parser.startOperators.insert("(".to_string(), true);
		parser.startOperators.insert("=".to_string(), true);
		parser.startOperators.insert("..".to_string(), true);
		parser.startOperators.insert("...".to_string(), true);

		parser
	}

	pub fn run(&mut self, tokens: Vec<Token>) {
		self.tokens = tokens;
		self.lenght = self.tokens.len();

		let res = self.read_file();

		println!("\n\n\n\n\n\n\n");
		println!("FINAL OUTPUT");
		println!("{:#?}", res);
	}

	fn read_file(&mut self) -> Vec<Instruction> {
		self.read_until(&vec![Token::new(Type::EOF, "".to_string())])
	}

	fn read_until_eol(&mut self) -> Vec<Instruction> {
		self.read_until(&vec![
			Token::new(Type::EOL, "".to_string()),
			Token::new(Type::EOF, "".to_string()),
		])
	}

	fn read_until(&mut self, until: &Vec<Token>) -> Vec<Instruction> {
		let mut res : Vec<Instruction> = Vec::new();

		println!("READING UNTIL: {:#?}", until);

		let mut first = true;

		loop {
			
			let next = self.get_token();

			if !first {
				for unt in until {
					if (unt.Type == Type::EOL ||unt.Type == Type::EOF || (unt.Type == Type::OPERATOR && unt.Value == ";".to_string())) && next.Type == unt.Type {
						println!("Stopped reading until (early)");
						println!("{:#?}", res);
						return res
					}
				}
			}

			// Advance and get again
			self.advance();
			let next = self.get_token();

			println!("read_until() HAS {:?}", next);

			for unt in until {
				if unt.Type == next.Type && unt.Value == next.Value {
					println!("Stopped reading until");
					println!("{:#?}", res);
					//self.reverse();
					return res;
				}
			}

			if next.Type == Type::EOF {
				println!("FOUND EOF, QUIT!");
				return res;
			}

			res.push(self.symbol(next));

			first = false;
		}
	}

	fn lookahead(&mut self, prev: Instruction, on: ON) -> Instruction {
		self.advance();
		let next = self.get_token();

		println!("lookahead() HAS {:?}", next);

		// PushClass
		// IO.Println("123")
		//   ^
		if next.Type == Type::OPERATOR && next.Value == ".".to_string() {
			return self.push_class(prev);
		}

		// Call
		// IO.Println("123")
		//           ^
		if next.Type == Type::OPERATOR && next.Value == "(".to_string() {
			return self.call(prev, on);
		}

		// We encountered an operator, check the type of the previous expression
		if next.Type == Type::OPERATOR {
			if self.startOperators.contains_key(&next.Value.to_string()) {
				return self.math(prev);
			}
		}

		println!("lookahead() did nothign, reverting {:?}", next);

		self.reverse();

		prev
	}	

	fn get_token(&self) -> Token {
		if self.index >= self.lenght {
			Token::new(Type::EOF, "".to_string())
		} else {
			println!("get_token() {:?}", self.tokens[self.index].clone());
			self.tokens[self.index].clone()
		}
	}

	fn get_and_expect_token(&self, exp: Type) -> Token {
		let tok = self.get_token();

		if tok.Type == exp {
			tok
		} else {
			panic!("get_and_expect_token() was not fulfilled")
		}
	}

	fn advance(&mut self) {
		if self.has_advanced {
			self.index += 1
		} else {
			self.has_advanced = true
		}
	}

	fn reverse(&mut self) {
		self.index -= 1
	}

	fn symbol_next(&mut self) -> Instruction {
		self.advance();
		let tok = self.get_token();

		if tok.Type == Type::EOF {
			println!("EOF IN SYMBOL_NEXT() (this should not happen)");
		}

		self.symbol(tok)
	}

	fn symbol(&mut self, tok: Token) -> Instruction {

		println!("Symbol: {:?}", tok);

		match tok.Type {
			Type::EOL => self.ignore(tok),
			Type::EOF => self.ignore(tok),
			Type::KEYWORD => self.keyword(tok),
			Type::NAME => self.name(tok),
			Type::NUMBER => self.number(tok),
			Type::STRING => self.string(tok),
			//Type::BOOL => self.bool(tok),
			//Type::OPERATOR => self.operator(tok),
			//Type::IGNORE => self.ignore(tok),
			_ => panic!("symbol() - Unknown Type, {:?}", tok),
		}
	}

	fn infix_priority(infix: String) -> u8 {
		let s : &str = infix.trim();

		match s {
			"&&" => 30,
			"||" => 30,
			"==" => 40,
			"!=" => 40,
			"<" => 40,
			"<=" => 40,
			">" => 40,
			">=" => 40,

			"[" => 5,
			"{" => 5,

			"+" => 50,
			"-" => 50,
			"*" => 60,
			"/" => 60,
			".." => 70,
			"..." => 70,
			"." => 80,
			"(" => 80,
			"=" => 80,
			"++" => 80,
			"--" => 80,

			_ => 0,
		}
	}

	fn keyword(&mut self, tok: Token) -> Instruction {

		let s : &str = tok.Value.trim();

		println!("Keyword: {:?}", tok);

		match s {
			"var" => self.keyword_var(tok.clone()),
			"if" => self.keyword_if(tok.clone()),
			// "class" => self.keyword_var(tok),
			// "static" => self.keyword_var(tok),
			// "new" => self.keyword_var(tok),
			// "return" => self.keyword_var(tok),
			// "for" => self.keyword_var(tok),
			_ => panic!("keyword() - Unknown keyword, {:?}", tok),
		}
	}

	fn keyword_var(&mut self, tok: Token) -> Instruction {
		let mut assign = Instruction::new(Ins::ASSIGN);

		self.advance();
		let next = self.get_and_expect_token(Type::NAME);

		assign.name = next.Value;

		self.advance();
		let next = self.get_and_expect_token(Type::OPERATOR);

		if next.Value != "=" {
			panic!("keyword_var() expected = after Type::NAME");
		}

		// Read until EOL
		let value = self.read_until_eol();

		assign.right = value;

		assign
	}

	fn keyword_if(&mut self, tok: Token) -> Instruction {
		let mut if_case = Instruction::new(Ins::IF);

		if_case.center = self.read_until(&vec![Token::new(Type::OPERATOR, "{".to_string())]);

		if_case.left = self.read_until(&vec![Token::new(Type::OPERATOR, "}".to_string())]);

		// Look ahead and see if there is an else-part to this if-case
		self.advance();

		let next = self.get_token();

			if next.Type == Type::KEYWORD && next.Value == "else".to_string() {

			self.advance();
			let curly = self.get_and_expect_token(Type::OPERATOR);

			if curly.Value != "{".to_string() {
				panic!("An else needs to be followed by an {");
			}

			if_case.right = self.read_until(&vec![Token::new(Type::OPERATOR, "}".to_string())]);

		} else {
			// Revert back to how it was
			self.reverse();
		}

		if_case
	}

	fn name(&mut self, tok: Token) -> Instruction {
		let mut name = Instruction::new(Ins::NAME);
		name.name = tok.Value;

		self.lookahead(name, ON::DEFAULT)
	}

	// Convert tok.Value (a String) to a f64 and simply get a LITERAL with a number value
	fn number(&mut self, tok: Token) -> Instruction {
		let num : f64 = tok.Value.parse().unwrap();
		let mut literal = Instruction::new(Ins::LITERAL);
		literal.value = Value::number(num);

		self.lookahead(literal, ON::DEFAULT)
	}

	// tok.Value is already a String, so this is easy
	fn string(&mut self, tok: Token) -> Instruction {		
		let mut literal = Instruction::new(Ins::LITERAL);
		literal.value = Value::string(tok.Value);
		
		self.lookahead(literal, ON::DEFAULT)
	}

	fn operator(&self, tok: Token) {
		// *
		// ||
	}

	fn ignore(&self, tok: Token) -> Instruction {
		Instruction::new(Ins::IGNORE)
	}

	fn bool(&self, tok: Token) {
		
	}

	fn math(&mut self, prev: Instruction) -> Instruction {
		let current = self.get_token();
		let mut math = Instruction::new(Ins::MATH);

		// The mathematical operator, eg + or -
		math.name = current.Value;

		if prev.instruction == Ins::LITERAL || prev.instruction == Ins::NAME {
			math.left = vec![prev];
			math.right = vec![self.symbol_next()];

			// Verify that the ordering (infix_priority()) is correct
			// Left is either a LITERAL or NAME, and right is a (new) MATH
			if math.right[0].instruction == Ins::MATH {

				// The ordering is wrong, and we need to correct this
				// [a, *, [b, +, c]] -> [[a, *, b], +, c]
				// This part is a little bit well, confusing and tight. But hey, it is a side-project after all.
				if Parser::infix_priority(math.name.clone()) > Parser::infix_priority(math.right[0].clone().name) {

					let right = math.right[0].clone();

					let mut res = Instruction::new(Ins::MATH);

					let mut left = Instruction::new(Ins::MATH);
					left.name = math.name.clone();
					left.left = math.left;
					left.right = right.left;

					res.left = vec![left];

					res.name = right.name;
					res.right = right.right;

					return res;
				}
			}

			println!("Next after math: {:?}", self.get_token());

			return math;
		}

		prev

		/*
			current := parser.nextToken(0)

			parser.advance()

			math := ins.Math{}
			math.Method = current.Value // + - * /

			// Differentiate between comparisions and arithmetic operators
			if _, ok := parser.comparisions[math.Method]; ok {
				math.IsComparision = true
			} else {
				math.IsComparision = false
			}

			if prev, ok := previous.(ins.Math); ok {
				if parser.getOperatorImportance(prev.Method) < parser.getOperatorImportance(math.Method) {
					math.Left = prev.Left
					math.Method = prev.Method
					math.Right = ins.Math{
						Method: current.Value,
						Left:   prev.Right,
						Right:  parser.parseNext(true),
					}
				} else {
					math.Left = previous
					math.Right = parser.parseNext(true)
				}

				return parser.lookAhead(math)
			}

			_, isLeftOnly := parser.leftOnlyInfix[math.Method]
			_, isRightOnly := parser.rightOnlyInfix[math.Method]

			if _, ok := previous.(ins.Literal); ok {
				math.Left = previous

				if !isLeftOnly {
					math.Right = parser.parseNext(true)
				}

				return parser.lookAhead(math)
			}

			if _, ok := previous.(ins.Variable); ok {
				math.Left = previous

				if !isLeftOnly {
					math.Right = parser.parseNext(true)
				}

				return parser.lookAhead(math)
			}

			if isRightOnly {
				math.Left = parser.parseNext(true)

				return parser.lookAhead(math)
			}

			math.Left = previous
			math.Right = parser.parseNext(true)

			return parser.lookAhead(math)
		*/
	}

	fn push_class(&mut self, prev: Instruction) -> Instruction {
		let mut push = Instruction::new(Ins::PUSH_CLASS);
		push.left = vec![prev];
		push.right = vec![self.symbol_next()];
		push
	}

	fn call(&mut self, prev: Instruction, on: ON) -> Instruction {
		let mut call = Instruction::new(Ins::CALL);
		call.left = vec![prev];
		call.right = self.read_until(&vec![
			Token::new(Type::OPERATOR, ")".to_string()),
			Token::new(Type::OPERATOR, ",".to_string())
		]);

		call
	}
}

pub struct Stack;

impl Stack {
	pub fn new() -> Stack {
		Stack
	}
}

#[derive(Debug)]
enum ON {
	DEFAULT,
	CLASS_BODY,
	PUSH_CLASS,
	METHOD_PARAMETERS,
	ARGUMENTS,
}
