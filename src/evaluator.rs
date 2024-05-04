use std::{collections::HashMap, env::temp_dir};
use crate::lexer::{Kind, Token};

pub fn evaluate(rpn_vec: Vec<Vec<Token>>) {
	let mut stack: Vec<Token> = Vec::new();
	let mut variables: HashMap<String, i32> = create_vars(&rpn_vec);
	// temp variable used for temp calculations that arent assigned to a variable.
	variables.insert(String::from("temp"), 0);

	for statement in rpn_vec {
		for token in statement {
			if token.kind == Kind::Operator {
				let r_token = stack.pop().unwrap();
				let mut rvalue: i32;
				if r_token.kind == Kind::Identifier {
					// get value of that variable
					rvalue = variables.get(&r_token.content).unwrap().clone();
				} else {
					// if it is a literal (number)
					rvalue = r_token.content.parse().unwrap();
				}

				// this is gross lol
				let l_token = stack.pop().unwrap_or_else(|| Token { content: String::from("temp"), kind: Kind::Identifier });
				let mut lvalue: i32;
				if l_token.kind == Kind::Identifier {
					lvalue = variables.get(&l_token.content).unwrap().clone();
				} else {
					lvalue = l_token.content.parse().unwrap();
				}

				match token.content.as_str() {
					"+" => {
						stack.push(Token { content: String::from("temp"), kind: Kind::Identifier });
						variables.insert(String::from("temp"), lvalue + rvalue);
					},
					"*" => {
						stack.push(Token { content: String::from("temp"), kind: Kind::Identifier });
						variables.insert(String::from("temp"), lvalue * rvalue);
					},
					"-" => {
						stack.push(Token { content: String::from("temp"), kind: Kind::Identifier });
						variables.insert(String::from("temp"), lvalue - rvalue);
					},
					"/" => {
						stack.push(Token { content: String::from("temp"), kind: Kind::Identifier });
						variables.insert(String::from("temp"), lvalue / rvalue);
					},
					"=" => {
						if r_token.kind == Kind::Identifier {
							variables.insert(l_token.content, variables.get(&String::from("temp")).unwrap().clone());
						} else {
							variables.insert(l_token.content, rvalue);
						}
					},
					_ => (),
				};
				println!("temp: \t{}", variables.get(&String::from("temp")).unwrap());
				println!("x: \t{}", variables.get(&String::from("x")).unwrap());
				println!("y: \t{}\n", variables.get(&String::from("y")).unwrap());
			} else {
				stack.push(token);
			}
		}
	}
}

// creates entries in the hashmap for all the variables in the program
fn create_vars(tokens: &Vec<Vec<Token>>) -> HashMap<String, i32> {
	let mut variables: HashMap<String, i32> = HashMap::new();

	for statement in tokens {
		for token in statement {
			if token.kind == Kind::Identifier {
				variables.insert(token.content.clone(), 0);
			}
		}

	}

	return variables;
}