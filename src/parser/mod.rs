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

		let mut first = true;

		loop {
			self.advance();

			let next = self.get_token();

			//if !first {
			//		for
			//	}

				/*if !first {
					for _, t := range until {
						if (t.Type == "EOL" || (t.Type == "operator" && t.Value == ";")) && parser.token.Type == t.Type {

							parser.stack.Pop()
							return
						}
					}
				}*/

			for unt in until {
				if unt.Type == next.Type {
					return res
				}
			}

			res.push(self.symbol(next));

			first = false;
		}
	}

	fn get_token(&self) -> Token {
		if self.index >= self.lenght {
			Token::new(Type::EOF, "".to_string())
		} else {
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

	fn symbol(&mut self, tok: Token) -> Instruction {

		println!("Symbol: {:?}", tok);

		match tok.Type {
			// Type::EOL => self.eof_eol(tok),
			// Type::EOF => self.eof_eol(tok),
			Type::KEYWORD => self.keyword(tok),
			Type::NAME => self.name(tok),
			//Type::NUMBER => self.number(tok),
			Type::STRING => self.string(tok),
			//Type::BOOL => self.bool(tok),
			//Type::OPERATOR => self.operator(tok),
			//Type::IGNORE => self.ignore(tok),
			_ => panic!("symbol() - Unknown Type, {:?}", tok),
		}
	}

	fn infix_priority(tok: Token) -> u8 {
		let s : &str = tok.Value.trim();

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

	fn eof_eol(&self, tok: Token) {

	}

	fn keyword(&mut self, tok: Token) -> Instruction {

		let s : &str = tok.Value.trim();

		println!("Keyword: {:?}", tok);

		match s {
			"var" => self.keyword_var(tok.clone()),
			/*"if" => self.keyword_var(tok),
			"class" => self.keyword_var(tok),
			"static" => self.keyword_var(tok),
			"new" => self.keyword_var(tok),
			"return" => self.keyword_var(tok),
			"for" => self.keyword_var(tok),*/
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

	fn name(&self, tok: Token) -> Instruction {
		let mut name = Instruction::new(Ins::NAME);
		name.name = tok.Value;
		name
	}

	fn number(&self, tok: Token) {
		
	}

	fn string(&self, tok: Token) -> Instruction {
		let mut literal = Instruction::new(Ins::LITERAL);
		literal.value = Value::string(tok.Value);
		literal
	}

	fn operator(&self, tok: Token) {
		// *
		// ||
	}

	fn ignore(&self, tok: Token) {
		
	}

	fn bool(&self, tok: Token) {
		
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
