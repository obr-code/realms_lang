pub enum Token {
	BinaryOperator {
		id: u8,
	},
	Equal,
	Identifier {
		ident: Vec<u8>,
	},
	Let,
	Number {
		val: i64,
	},
	ParentOpen,
	ParentClose,
}
use std::fmt;
impl fmt::Debug for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::BinaryOperator { id } => f.debug_struct("BinaryOperator")
				.field("operator", &char::from(*id))
				.finish(),
			Token::Equal => write!(f, "Equal"),
			Token::Identifier { ident } => f.debug_struct("Identifier")
				.field("id", &String::from_utf8(ident.clone()).unwrap())
				.finish(),
			Token::Let => write!(f, "Let"),
			Token::Number { val } => f.debug_struct("Number")
				.field("val", &val)
				.finish(),
		}
	}
}

use std::fs::File;
use std::io::Read;
pub fn tokenize(source: File) -> Result<Vec<Token>, std::io::Error> {
	let mut tokens = vec![];
	let mut stack = vec![];

	for b in source.bytes() {
		if let Ok(b) = b {
			match b {
				// BinaryOperator
				b'+' | b'-' | b'*' | b'/' => tokens.push(
					Token::BinaryOperator {
						id: b,
					}
				),
			
				// Equal
				b'=' => tokens.push(
					Token::Equal
				),

				// Parentheses
				b'(' => tokens.push(
					Token::ParentOpen
				),
				b')' => tokens.push(
					Token::ParentClose
				),

				// Space
				b'\n' | b'\r' | b' ' => {
					if let Some(&b) = stack.first() {
						match b {
							b'0'..b'9' => tokens.push(
								Token::Number {
									val: stack.drain(0..).fold(0, |val, digit| val + digit as i64),
								}
							),
							_ => match &stack[..] {
								// Let
								b"let" => {
									tokens.push(
										Token::Let
									);
									stack.clear();
								},
								// Identifier
								_ => tokens.push(
									Token::Identifier {
										ident: stack.drain(0..).collect(),
									}
								),
							}
						};
					};
				},

				// Digit
				digit => stack.push(digit),
			}
		}
	}
	// End
	if let Some(&b) = stack.first() {
		match b {
			b'0'..b'9' => tokens.push(
				Token::Number {
					val: stack.drain(0..).fold(0, |val, digit| val + digit as i64),
				}
			),
			_ => tokens.push(
				Token::Identifier {
					ident: stack.drain(0..).collect(),
				}
			),
		};
	}

	return Ok(tokens);
}