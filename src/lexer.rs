#[derive(Debug)]
pub struct Token {
	content: String,
	taipu: Kind,
}


#[derive(Debug)]
enum Kind {
	Identifier,
	Operator,
	Literal,
	Seperator,
	Keyword,
}
/// **the main function for lexing.**
/// 
/// Takes a `Vec<&str>`, for each char in the slice, it will decide what token it is, and return back a `Vec<Token>`
/// 
/// because the Vec is split up at ; semicolons, each Vec that lex() returns will be one statement.
pub fn lex(strings: &str) -> Vec<Token> {
	let ret = part1(strings);
	let ret = part2(ret);
	
	return ret;
}

/// Vec<&str> Vec<&str> Vec<String> Vec<&str>
fn part1(strings: &str) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut current = String::new();

	for ch in strings.chars() {
		current.push(ch);
		if !current.chars().all(char::is_alphanumeric) {
			// remove last char, push to tokens as temporarly identifier
			let ch2 = current.pop().unwrap();
			tokens.push(Token{content: current.clone(), taipu: Kind::Identifier});
			current.clear();

			match ch2 {
				// ops
				'+' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Operator}),
				'-' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Operator}),
				'*' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Operator}),
				'/' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Operator}),
				'=' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Operator}),
				// seperators
				';' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Seperator}),
				'(' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Seperator}),
				')' => tokens.push(Token {content: String::from(ch2), taipu: Kind::Seperator}),
				_ => {},
			}
		}
	}
	return tokens;
}

fn part2(token_vec: Vec<Token>) -> Vec<Token>{


	todo!();
}