use std::collections::VecDeque;
// Tokens
#[derive(Debug)]
enum TokenType {
	// Operations
	Plus,
	Minus,
	Asterisk,
	Slash,

	// Order of operations
	OpenParenthesis,
	CloseParenthesis,

	// Numbers
	Int,
	// Float,
}

#[derive(Debug)]
struct Tok {
	tok_type: TokenType,
	val: String,
}

/**
 * Different types of binary operations
 */
#[derive(Debug)]
enum BinaryOperationType {
	Add,
	Subtract,
	Multiply,
	Divide,
}

/**
 * Represents a single binary operation
 */
#[derive(Debug)]
struct BinaryOperation {
	left_node: Option<Box<BinaryOperation>>,
	right_node: Option<Box<BinaryOperation>>,
	operation: BinaryOperationType,
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let input = args
		.get(1)
		.expect("Please provide a math expression to calculate");
	let tokens = lex_tokens(input);
	println!("tok: {:?}", tokens);
	let ast = parse_ast(tokens);
	println!("ast: {:?}", ast);
}

fn lex_tokens(input: &String) -> VecDeque<Tok> {
	let mut out: VecDeque<Tok> = VecDeque::new();
	let mut characters = VecDeque::from(Vec::from(input.as_bytes()));

	loop {
		let byte = characters.pop_front();
		if byte.is_none() {
			break;
		}

		let ch = byte.unwrap() as char;
		match ch {
			// Numbers
			'0'..='9' => {
				characters.push_front(ch as u8);
				let mut tok: Tok = Tok {
					tok_type: TokenType::Int,
					val: "".to_string(),
				};

				'outer: loop {
					let byte = characters.pop_front();
					if byte.is_none() {
						out.push_back(tok);
						break 'outer;
					}

					let ch = byte.unwrap() as char;
					match ch {
						'0'..='9' => tok.val += &ch.to_string(),
						_ => {
							// This character is "unused"; put it back
							characters.push_front(ch as u8);
							out.push_back(tok);
							break 'outer;
						}
					}
				}
			}

			// '+', '-', '*', & '/'
			'+' => out.push_back(Tok {
				tok_type: TokenType::Plus,
				val: ch.to_string(),
			}),
			'-' => out.push_back(Tok {
				tok_type: TokenType::Minus,
				val: ch.to_string(),
			}),
			'*' => out.push_back(Tok {
				tok_type: TokenType::Asterisk,
				val: ch.to_string(),
			}),
			'/' => out.push_back(Tok {
				tok_type: TokenType::Slash,
				val: ch.to_string(),
			}),
			'(' => out.push_back(Tok {
				tok_type: TokenType::OpenParenthesis,
				val: ch.to_string(),
			}),
			')' => out.push_back(Tok {
				tok_type: TokenType::CloseParenthesis,
				val: ch.to_string(),
			}),

			// whitespace
			' ' => { /* ignore */ }

			// non-matched characters
			_ => {
				println!("Unknown character {:?}", ch)
			}
		}
	}

	return out;
}

fn parse_ast(tokens: VecDeque<Tok>) -> Option<&'static BinaryOperation> {
	let out: Option<&BinaryOperation> = None;

	unimplemented!("Dank stuff");

	return out;
}
