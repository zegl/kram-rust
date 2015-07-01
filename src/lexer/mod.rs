use std::collections::HashMap;
use std::clone::Clone;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Type {
	EOL,
	EOF,
	NAME,
	NUMBER,
	STRING,
	BOOL,
	KEYWORD,
	OPERATOR,
	IGNORE,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
	pub Type: Type,
	pub Value: String,
}

impl Token {
	pub fn new(t: Type, v: String) -> Token {
		Token {
			Type: t,
			Value: v,
		}
	}

	pub fn clone(&self) -> Token {
		Token {
			Type: self.Type.clone(),
			Value: self.Value.clone(),
		}
	}
}

pub struct Lexer {
	operators: HashMap<String, bool>,
	keywords: HashMap<String, bool>,
	chars: Vec<char>,
	current: char,
	index: usize,
}

impl Lexer {

	pub fn new() -> Lexer {
		let mut lexer = Lexer {
			operators: HashMap::new(),
			keywords: HashMap::new(),
			chars: Vec::new(),
			index: 0,
			current: ' ',
		};

		lexer.operators.insert("+".to_string(), true);
		lexer.operators.insert("-".to_string(), true);
		lexer.operators.insert("*".to_string(), true);
		lexer.operators.insert("/".to_string(), true);
		lexer.operators.insert("%".to_string(), true);
		lexer.operators.insert("**".to_string(), true);
		lexer.operators.insert("=".to_string(), true);
		lexer.operators.insert("==".to_string(), true);
		lexer.operators.insert(">".to_string(), true);
		lexer.operators.insert(">=".to_string(), true);
		lexer.operators.insert("<".to_string(), true);
		lexer.operators.insert("<=".to_string(), true);
		lexer.operators.insert("&&".to_string(), true);
		lexer.operators.insert("|".to_string(), true);
		lexer.operators.insert("||".to_string(), true);
		lexer.operators.insert("...".to_string(), true);
		lexer.operators.insert("..".to_string(), true);
		lexer.operators.insert(".".to_string(), true);
		lexer.operators.insert("{".to_string(), true);
		lexer.operators.insert("}".to_string(), true);
		lexer.operators.insert(":".to_string(), true);
		lexer.operators.insert(".to_string(),".to_string(), true);
		lexer.operators.insert("++".to_string(), true);
		lexer.operators.insert("--".to_string(), true);

		lexer.keywords.insert("if".to_string(), true);
		lexer.keywords.insert("else".to_string(), true);
		lexer.keywords.insert("var".to_string(), true);
		lexer.keywords.insert("class".to_string(), true);
		lexer.keywords.insert("static".to_string(), true);
		lexer.keywords.insert("return".to_string(), true);
		lexer.keywords.insert("for".to_string(), true);
		lexer.keywords.insert("in".to_string(), true);

		lexer
	}
	
	pub fn run(&mut self, source: String) -> Vec<Token> {
		self.chars = source.chars().collect();

		let mut tokens : Vec<Token> = Vec::new();

		loop {
			let token = self.parse_next();

			self.index += 1;

			match token.Type {
				Type::EOF => break,
				Type::IGNORE => continue,
				_ => tokens.push(token),
			}
		}

		println!("{:?}", tokens);

		tokens
	}

	fn char_at_pos(&self, index : usize) -> char {

		// Indicate nothingness
		if index >= self.chars.len() {
			return '\0'
		}

		return self.chars[index]
	}

	fn parse_next(&mut self) -> Token {
		// End of file
		if self.index >= self.chars.len() {
			return Token::new(Type::EOF, "".to_string())
		}

		// Get current char
		self.current = self.chars[self.index];

		// Line endings
		if self.current == '\n' || self.current == '\r' {
			return Token::new(Type::EOL, "".to_string())
		}

		// Ignore Whitespace
		if self.current.is_whitespace() {
			return Token::new(Type::IGNORE, "".to_string())
		}

		// Comments
		if self.current == '/' && self.char_at_pos(self.index+1) == '/' {
			return self.comment()
		}

		// Names
		// Begins with a char a-Z
		if (self.current >= 'a' && self.current <= 'z') || (self.current >= 'A' && self.current <= 'Z') {
			return self.name()
		}

		// Numbers
		if self.current >= '0' && self.current <= '9' {
			return self.number()
		}

		// Strings
		if self.current == '\"' {
			return self.string()
		}

		// operators
		if self.operators.contains_key(&self.current.to_string()) {
			return self.operator()
		}

		return Token::new(Type::OPERATOR, self.current.to_string())
	}

	fn name(&mut self) -> Token {
		let mut s : String = self.current.to_string();

		loop {
			let c = self.chars[self.index + 1];

			// After the beginning, a name can be a-Z0-9_
			if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_' {
				s.push(c);
				self.index += 1;
			} else {
				break
			}
		}

		if s == "true" || s == "false" {
			return Token::new(Type::BOOL, s)
		}

		if self.keywords.contains_key(&s) {
			return Token::new(Type::KEYWORD, s)
		}

		return Token::new(Type::NAME, s)
	}

	fn comment(&mut self) -> Token {
		loop {
			self.index += 1;
			self.current = self.chars[self.index + 1];

			if self.current == '\n' || self.current == '\r' {
				return Token::new(Type::EOL, "".to_string())
			}
		}
	}

	fn number(&mut self) -> Token {
		let mut s : String = self.current.to_string();

		// Look for more digits.
		loop {
			let c = self.char_at_pos(self.index + 1);

			if (c < '0' || c > '9') && c != '.' {
				break
			}

			// A dot needs to be followed by another digit to be valid
			if c == '.' {
				let cc = self.char_at_pos(self.index + 2);

				if cc < '0' || cc > '9' {
					break
				}
			}

			self.index += 1;
			s.push(c);
		}

		// TODO Decimal
		// TODO Verify that it ends with a space?

		return Token::new(Type::NUMBER, s)
	}

	fn string(&mut self) -> Token {
		let mut s : String = self.current.to_string();

		self.index += 1;

		loop {

			// End of string
			if self.char_at_pos(self.index) == '\"' {
				break
			}

			// Escaping
			if self.char_at_pos(self.index) == '\\' {
				self.index += 1
			}

			s.push(self.char_at_pos(self.index));

			self.index += 1;
		}

		return Token::new(Type::STRING, s)
	}

	fn operator(&mut self) -> Token {
		let mut s : String = self.current.to_string();

		loop {

			let next = self.char_at_pos(self.index + 1);

			// EOF
			if next == '\0' {
				break
			}

			let mut combined = s.clone();
			combined.push(next);

			if self.keywords.contains_key(&combined) {
				s.push(next);
				self.index += 1;
			} else {
				break
			}
		}

		return Token::new(Type::OPERATOR, s)
	}
}

