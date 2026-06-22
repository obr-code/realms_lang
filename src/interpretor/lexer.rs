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
				b"let" => tokens.push(Token::Let),

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
			// Eq
			b'=' => tokens.push(Token::Eq),
			// Parentheses
			b'(' => tokens.push(Token::ParentOpen),
			b')' => tokens.push(Token::ParentClose),
			// Semicolons
			b';' => tokens.push(Token::Semi),
			// White Spaces
			b'\r' | b'\t' | b'\n' | b' ' => (),
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
