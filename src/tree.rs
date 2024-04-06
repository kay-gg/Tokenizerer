use std::collections::HashMap;

use crate::lexer::Token;
use crate::lexer::Kind;

struct BinaryTree {
	left: Option<Box<BinaryTree>>,
	right: Option<Box<BinaryTree>>,
}

struct OpPrecedence {
	prec: i8,
	associativity: bool, //left = false, right = true
}

pub fn create_tree(token_vec: Vec<Token>) {
	let tree = BinaryTree{left: Option::None, right: Option::None};

	let mut precedence_hm: HashMap<&str, OpPrecedence> = HashMap::new();
	precedence_hm.insert("*", OpPrecedence { prec: 4, associativity: false });
	precedence_hm.insert("/", OpPrecedence { prec: 4, associativity: false });
	precedence_hm.insert("+", OpPrecedence { prec: 3, associativity: false });
	precedence_hm.insert("-", OpPrecedence { prec: 3, associativity: false });
	precedence_hm.insert("=", OpPrecedence { prec: 1, associativity: false });
	// should not be done
	precedence_hm.insert("(", OpPrecedence { prec: 0, associativity: false });

	// shunting yard
	let mut op_stack: Vec<Token> = Vec::new();
	let mut output: Vec<Token> = Vec::new();

	for t in token_vec {
		if t.kind == Kind::Operator {
			// only if there is something in ops
			if let Some(op) = op_stack.pop() {
				// check precedence
				let op_in_stack = precedence_hm.get(op.content.as_str()).unwrap();
				let new_op = precedence_hm.get(t.content.as_str()).unwrap();

				// if precedence of t is >, we push to ops,
				// else, pop op off stack and add to output, then push t onto stack
				if new_op.prec > op_in_stack.prec {
					op_stack.push(op);
					op_stack.push(t);
				} else {
					output.push(op);
					op_stack.push(t);
				}
			} else {
				op_stack.push(t);
			}
		} else if t.kind == Kind::Seperator {
			match t.content.as_str() {
				"(" => op_stack.push(t),
				")" => {
					while let Some(find_parenthesis) = op_stack.pop() {
						if find_parenthesis.content == "(" {
							break;
						} else {
							output.push(find_parenthesis);
						}
					}
				},
				";" => op_stack.push(t),
				_ => {}
			}
		} else {
			output.push(t);
		}
	}
	// poop rest of op stack
	for t in op_stack {
		output.push(t);
	}
	// end shunting yard
	
	for xx in output {
			print!("{:?}\n", xx);
	}
}