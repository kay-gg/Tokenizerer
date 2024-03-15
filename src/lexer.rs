#[derive(Debug)]
pub struct Token {
	content: String,
	kind: Kind,
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
			tokens.push(Token{content: current.clone(), kind: Kind::Identifier});
			current.clear();

			match ch2 {
				// ops
				'+' => tokens.push(Token {content: String::from(ch2), kind: Kind::Operator}),
				'-' => tokens.push(Token {content: String::from(ch2), kind: Kind::Operator}),
				'*' => tokens.push(Token {content: String::from(ch2), kind: Kind::Operator}),
				'/' => tokens.push(Token {content: String::from(ch2), kind: Kind::Operator}),
				'=' => tokens.push(Token {content: String::from(ch2), kind: Kind::Operator}),
				// seperators
				';' => tokens.push(Token {content: String::from(ch2), kind: Kind::Seperator}),
				'(' => tokens.push(Token {content: String::from(ch2), kind: Kind::Seperator}),
				')' => tokens.push(Token {content: String::from(ch2), kind: Kind::Seperator}),
				_ => {},
			}
		}
	}
	return tokens;
}

// can definitely be done during part 1 if want to optimize
fn part2(token_vec: Vec<Token>) -> Vec<Token>{
	let mut new: Vec<Token> = Vec::new();

	for tokens in token_vec {
		match tokens.kind {
			Kind::Identifier => {
				if tokens.content == "print" {
					new.push(Token {content: String::from("print"), kind: Kind::Keyword} )
				
				} else if tokens.content.chars().all(char::is_numeric) {
					// not ideal because content is a string, will need converting unfortunately
					new.push(Token {content: String::from(tokens.content), kind: Kind::Literal})
				} else if tokens.content.chars().all(char::is_alphabetic) { // check if identifier(no numbers in name)
					new.push(tokens);
				} else {
					panic!("Something broke!");
				}
			},
			_ => new.push(tokens),
		}
	}
	return new;
}