use crate::lexer::{Kind, Token};

pub fn evaluate(tokens: Vec<Token>) {
	let mut stack: Vec<Token> = Vec::new();

	for t in tokens {
		if t.kind == Kind::Operator {
			
		} else {
			stack.push(t);
		}
	}
}