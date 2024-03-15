#[derive(Debug)]
pub struct Token {
	content: String,
	taipu: Taipu,
}


#[derive(Debug)]
enum Taipu {
	Identifier,
	Operator,
	Literal,
	Seperator,
	Keyword,
}
/// Takes a Vec<&str>, for each char in the slice, it will decide what token it is, and return back a Vec<Token>
/// because the Vec is split up at ; semicolons, each Vec that lexer() returns will be 1 statement.
pub fn lex(strings: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut current = String::new();

	for ch in strings.chars() {
		current.push(ch);
		if !current.chars().all(char::is_alphanumeric) {
			// remove last char, push to tokens as temporarly identifier
			let ch2 = current.pop().unwrap();
			tokens.push(Token{content: current.clone(), taipu: Taipu::Identifier});
			current.clear();

			match ch2 {
				// ops
				'+' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Operator}),
				'-' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Operator}),
				'*' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Operator}),
				'/' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Operator}),
				'=' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Operator}),
				// seperators
				';' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Seperator}),
				'(' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Seperator}),
				')' => tokens.push(Token {content: String::from(ch2), taipu: Taipu::Seperator}),
				_ => {},
			}
		}
	}
	return tokens;
}