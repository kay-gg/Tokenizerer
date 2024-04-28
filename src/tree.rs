use core::panic;

use crate::lexer::Token;
use crate::lexer::Kind;

pub fn create_tree(mut token_vec: Vec<Token>) {
	// shunting yard
	let mut op_stack: Vec<Token> = Vec::new();
	let mut output: Vec<Token> = Vec::new();

	for t in token_vec {
		if t.kind == Kind::Operator {
			while let Some(op_in_stack) = op_stack.pop() {
				if op_in_stack.content == "(" {
					op_stack.push(op_in_stack);
					break;
				}
				if get_precedence(&op_in_stack.content) > get_precedence(&t.content) {
					output.push(op_in_stack);
				} else {
					op_stack.push(op_in_stack);
					break;
				}
			}
			op_stack.push(t);
		} else if t.kind == Kind::Seperator {
			if t.content == "(" {
				op_stack.push(t);
			} else if t.content == ")" {
				while let Some(find_parenthesis) = op_stack.pop() {
					if find_parenthesis.content != "(" {
						output.push(find_parenthesis);
						break;
					}
				}
			}
		} else {
			output.push(t);
		}
	}
	while let Some(op_in_stack) = op_stack.pop() {
		output.push(op_in_stack);
	}
	// end shunting yard
	
	for xx in output {
			print!("{:?}\n", xx);
	}
}

fn get_precedence(op: &String) -> i8 {
	match op.as_str() {
		"*" => 4,
		"/" => 4,
		"+" => 3,
		"-" => 3,
		"=" => 2,
		_ 	=> panic!("Unknown precedence: {}", op),
	}
}