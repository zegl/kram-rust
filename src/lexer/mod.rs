use std::collections::HashMap;

struct Token {
	Type: String,
	Value: String,
}

pub struct Lexer {
	operators: HashMap<String, bool>,
	keywords: HashMap<String, bool>,
}

impl Lexer {

	pub fn new() -> Lexer {
		let mut lexer = Lexer {
			operators: HashMap::new(),
			keywords: HashMap::new(),
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
	
	pub fn run(&self) {

	}
}

