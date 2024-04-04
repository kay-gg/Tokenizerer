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

	let mut opHM: HashMap<&str, OpPrecedence> = HashMap::new();
	opHM.insert("*", OpPrecedence { prec: 4, associativity: false });
	opHM.insert("/", OpPrecedence { prec: 4, associativity: false });
	opHM.insert("+", OpPrecedence { prec: 3, associativity: false });
	opHM.insert("-", OpPrecedence { prec: 3, associativity: false });
	opHM.insert("=", OpPrecedence { prec: 1, associativity: false });

	// shunting yard
	let mut ops: Vec<Token> = Vec::new();
	let mut output: Vec<Token> = Vec::new();

	for t in token_vec {
		if t.kind == Kind::Operator {
			// only if there is something in ops
			if let Some(op) = ops.pop() {
				// check precedence
				let temp = opHM.get(op.content.as_str()).unwrap();
				let tPrec = opHM.get(t.content.as_str()).unwrap();

				// if precedence of t is >, we push to ops,
				// else, pop op off stack and add to output, then push t onto stack
				if tPrec.prec > temp.prec {
					ops.push(t);
				} else {
					output.push(ops.pop().unwrap());
					ops.push(t);
				}
			} else {
				ops.push(t);
			}
		} else if t.kind == Kind::Seperator {
			match t.content.as_str() {
				"(" => ops.push(t),
				")" => {
					while let findParenthesis = ops.pop().unwrap() {
						if findParenthesis.content == "(" {
							break;
						} else {
							output.push(findParenthesis);
							panic!("mismatched parenthesis!!");
						}
					}
				},
				";" => ops.push(t),
				_ => {}
			}
		} else {
			output.push(t);
		}
	}
	// poop rest of op stack
	for t in ops {
		output.push(t);
	}
	
	for xx in output {
			print!("{:?}\n", xx);
	}
}