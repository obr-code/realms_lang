use crate::interpretor::ast::*;
use crate::interpretor::lexer::Token::BinaryOperator;

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
use std::io::{Bytes, Read};

pub struct Lexer {
	bytes: Bytes<File>,
	stack: Vec<u8>,
	tokens: Vec<Token>,
}
impl Lexer {
	pub fn tokenize(source: File) -> Result<Vec<Token>, std::io::Error> {
		let mut lexer = Lexer {
			bytes: source.bytes(),
			stack: vec![],
			tokens: vec![],
		};


	}

	pub fn token_next(&mut self) -> Token {
		if let Some(bit) = self.bytes.next() {
			match bit {
				// Binary Operator
				b'+' | b'-' | b'*' | b'/' | b'%' => Token::BinaryOperator(bit),
				// Eq
				b'=' => Token::Eq,
				// Parentheses
				b'(' => Token::ParentOpen,
				b')' => Token::ParentClose,
				// Semicolons
				b';' => Token::Semi,
				// White Spaces
				digit => {
					self.stack.push(digit);
				},
				
			}
		else { Token::EOF }
	}
}

pub fn token_num(&mut self) -> Token {
	let p = self.stack.partition_point(|bit| bit.is_ascii_alphabetic());
	let digits = String::from_utf8(self.stack[..p]).unwrap();
	let suffix = String::from_utf8(self.stack[p..]).unwrap();
	self.stack.clear();

	Token::Numeric { digits, suffix }
}

pub fn token_ident(&mut self)

pub fn tokenize(source: File) -> Result<Vec<Token>, std::io::Error> {
	let x = source.bytes();
	let mut tokens = vec![];
	let mut stack = vec![];
	let mut stack_pop = || -> Vec<u8> {

	fn process(&mut &mut stack) -> Option<Token> {
		match tokens {
			// BinaryExpr
			b'+' | b'-' | b'*' | b'/' => Some(Token::BinaryOperator(b)),

			// Eq
			b'=' => Some(Token::Eq),

			// Parentheses
			b'(' => Some(Token::ParentOpen),
			b')' => Some(Token::ParentClose),

			// Space
			b'\n' | b'\r' | b' ' => {
				if let Some(&b) = stack.first() {
					match b {
						b'0'..b'9' => {
							let mut w: Vec<&[u8]> = stack.split(|b| b.is_ascii_alphabetic()).collect();
							w.push(&[]);

							let digits = String::from_utf8(w[0].into()).unwrap();
							let suffix = String::from_utf8(w[1].into()).unwrap();

							stack.clear();

							Some(Token::Numeric { digits, suffix })
						},
						_ => match &stack[..] {
							// Let
							b"let" => {
								stack.clear();
								Some(Token::Let)
							},
							// Identifier
							_ => Some(Token::Ident(String::from_utf8(stack.drain(0..).collect::<Vec<u8>>()).unwrap())),
						},
					}
				} else { None }
			},

			// Semicolon
			b';' => Some(Token::Semi),

			// Digit
			digit => { stack.push(digit); None },
		}
	}

	for b in source.bytes() {
		if let Ok(b) = b {
			
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