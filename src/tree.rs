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

pub fn create_tree(mut token_vec: Vec<Token>) {
	let tree = BinaryTree{left: Option::None, right: Option::None};

	let mut precedence_hm: HashMap<&str, OpPrecedence> = HashMap::new();
	precedence_hm.insert("*", OpPrecedence { prec: 4, associativity: false });
	precedence_hm.insert("/", OpPrecedence { prec: 4, associativity: false });
	precedence_hm.insert("+", OpPrecedence { prec: 3, associativity: false });
	precedence_hm.insert("-", OpPrecedence { prec: 3, associativity: false });
	precedence_hm.insert("=", OpPrecedence { prec: 1, associativity: false });
	// should not be done
	precedence_hm.insert("(", OpPrecedence { prec: 0, associativity: false });
	precedence_hm.insert(";", OpPrecedence { prec: 0, associativity: false});

	// shunting yard
	// i dont know if i can call this shunting yard anymore. im not following the algo
	let mut op_stack: Vec<Token> = Vec::new();
	let mut output: Vec<Token> = Vec::new();

	token_vec.reverse();

	for t in token_vec {
		if t.kind == Kind::Operator {
			//let t_content = t.content.as_str();
			// only if there is something in op_stack
			while let Some(op_in_stack) = op_stack.pop() {
				if precedence_hm.get(op_in_stack.content.as_str()).unwrap().prec > precedence_hm.get(t.content.as_str()).unwrap().prec {
					output.push(op_in_stack);
				} else {
					op_stack.push(op_in_stack);
					break;
				}
			}
			op_stack.push(t);	
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
				// ignore ; not needed 
				";" => (),
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