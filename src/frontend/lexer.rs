

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
	BinaryOperator(u8),
	BraceClose,
	BraceOpen,
	BracketClose,
	BracketOpen,
	Colon,
	Const,
	Empty,
	EOF,
	Eq,
	Fn,
	Ident(String),
	Mut,
	ParentClose,
	ParentOpen,
	Numeric {
		digits: String,
		suffix: String,
	},
	Semi,
	Static,
}

use std::fs::File;
use std::io::Read;

pub fn tokenize(source: File) -> Result<Vec<Token>, std::io::Error> {
	let mut bytes = source.bytes();
	let mut stack = vec![];
	let mut tokens = vec![];

	while let Some(Ok(bit)) = bytes.next() {
		if match bit {
			b'0'..b'9' => true,
			b'a'..b'z' => true,
			b'_' => true,
			_ => false,
		} == true {
			stack.push(bit);
			continue;
		}

		if !stack.is_empty() {
			match &stack[..] {
				b"C" => tokens.push(Token::Const),
				b"F" => tokens.push(Token::Fn),
				b"M" => tokens.push(Token::Mut),
				b"S" => tokens.push(Token::Static),

				_ => match stack.first().unwrap() {
					// Numeric
					b'0'..b'9' => tokens.push(token_num(&mut stack)),
					// Alphabetic
					b'a'..b'z' | b'A'..b'Z' | b'_' => tokens.push(token_ident(&mut stack)),
					x => panic!("Unexpected character: {}", x),
				},
			}
			stack.clear();
		}

		match bit {
			// Binary Operator
			b'+' | b'-' | b'*' | b'/' | b'%' => tokens.push(Token::BinaryOperator(bit)),
			// Colon
			b':' => tokens.push(Token::Colon),
			// Eq
			b'=' => tokens.push(Token::Eq),
			// Parentheses
			b'(' => tokens.push(Token::ParentOpen),
			b')' => tokens.push(Token::ParentClose),
			// Braces
			b'{' => tokens.push(Token::BraceOpen),
			b'}' => tokens.push(Token::BraceClose),
			// Brackets
			b'[' => tokens.push(Token::BracketOpen),
			b']' => tokens.push(Token::BracketClose),
			// Semicolons
			b';' => tokens.push(Token::Semi),
			// White Spaces
			b'\r' | b'\t' | b'\n' | b' ' | b',' => (),
			// Digits
			digit => {
				stack.push(digit);
				continue;
			},
		};
		
	}
	tokens.push(Token::EOF);
	Ok(tokens)
}

pub fn token_num(stack: &mut Vec<u8>) -> Token {
	let p = stack.partition_point(|bit| bit.is_ascii_alphanumeric() || *bit == b'_');
	let digits = String::from_utf8(stack.drain(..p).collect()).unwrap();
	let suffix = String::from_utf8(stack.drain(..).collect()).unwrap();

	Token::Numeric { digits, suffix }
}
pub fn token_ident(stack: &mut Vec<u8>) -> Token {
	let ident = String::from_utf8(stack.drain(..).collect()).unwrap();

	Token::Ident(ident)
}
