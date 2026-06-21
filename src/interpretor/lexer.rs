use crate::interpretor::ast::*;

#[derive(Debug)]
pub enum Token {
	BinaryOperator(u8),
	Empty,
	EOF,
	Eq,
	Ident(String),
	Let,
	ParentClose,
	ParentOpen,
	Numeric {
		digits: String,
		suffix: String,
	},
	Semi,
}

use std::fs::File;
use std::io::Read;
pub fn tokenize(source: File) -> Result<Vec<Token>, std::io::Error> {
	let mut tokens = vec![];
	let mut stack = vec![];

	for b in source.bytes() {
		if let Ok(b) = b {
			match b {
				// BinaryExpr
				b'+' | b'-' | b'*' | b'/' => tokens.push(
					Token::BinaryOperator(b)
				),
			
				// Eq
				b'=' => tokens.push(
					Token::Eq
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
							b'0'..b'9' => tokens.push({
								let mut w: Vec<&[u8]> = stack.split(|b| b.is_ascii_alphabetic()).collect();
								w.push(&[]);

								let digits = String::from_utf8(w[0].into()).unwrap();
								let suffix = String::from_utf8(w[1].into()).unwrap();

								stack.clear();

								Token::Numeric { digits, suffix }
							}),
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
									Token::Ident(String::from_utf8(stack.drain(0..).collect::<Vec<u8>>()).unwrap())
								),
							}
						};
					};
				},

				// Semicolon
				b';' => tokens.push(
					Token::Semi
				),

				// Digit
				digit => stack.push(digit),
			}
		}
	}
	// End
	if let Some(&b) = stack.first() {
		match b {
			b'0'..b'9' => tokens.push({
				let mut w: Vec<&[u8]> = stack.split(|b| b.is_ascii_alphabetic()).collect();
				w.push(&[]);

				Token::Numeric {
					digits: String::from_utf8(w[0].into()).unwrap(),
					suffix: String::from_utf8(w[1].into()).unwrap(),
				}
			}),
			_ =>  tokens.push(
				Token::Ident(String::from_utf8(stack.drain(0..).collect::<Vec<u8>>()).unwrap())
			),
		};
	}
	tokens.push(Token::EOF);

	return Ok(tokens);
}