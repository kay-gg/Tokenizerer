use std::{arch::x86_64, char, fs::{self, File}, io::Read, task::Context};
mod lexer;

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
	for x in token_vec {
		dbg!(x);
	}
}