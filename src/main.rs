use std::fs;
mod lexer;
mod rpn;
mod evaluator;

use rpn::rpn_from_tokens;

use crate::lexer::Token;
use crate::lexer::lex;

fn main() {
	let file_path = "./input";

	let content = fs::read_to_string(file_path).expect("file read error");
	let content: Vec<&str> = content.split_inclusive(";").collect();

	// each vec is a statement
	let mut token_vec: Vec<Vec<Token>> = Vec::new();
	for strings in content {
		// dbg!(strings);
		let temp = lex(strings);
		token_vec.push(temp);
	}

	let mut rpn_vec: Vec<Vec<Token>> = Vec::new();
	// move tokens into RPN form
	for statement in token_vec {
		rpn_vec.push(rpn_from_tokens(statement));
	}

	//evaluate the RPN
	evaluator::evaluate(rpn_vec);
}