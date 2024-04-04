use std::{fs};
mod lexer;
mod tree;

use crate::lexer::Token;
use crate::lexer::lex;

fn main() {
	let file_path = "./simpleinput";

	let content = fs::read_to_string(file_path).expect("file read error");
	let content: Vec<&str> = content.split_inclusive(";").collect();

	// each vec is a statement
	let mut token_vec: Vec<Vec<Token>> = Vec::new();
	for strings in content {
		// dbg!(strings);
		let temp = lex(strings);
		token_vec.push(temp);
	}

	// create some tree to run from tokens
	for vector in token_vec {
		tree::create_tree(vector);
		println!("\nend\n");
	}
}