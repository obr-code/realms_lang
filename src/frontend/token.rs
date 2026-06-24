


pub type ClassID = String;
pub type IdentID = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
	BinaryOperator(u8),
	BraceClose,
	BraceOpen,
	BracketClose,
	BracketOpen,
	Class(ClassID),
	Colon,
	Let,
	Empty,
	EOF,
	Eq,
	Fn,
	Ident(IdentID),
	Numeric {
		digits: String,
		suffix: String,
	},
	ParentClose,
	ParentOpen,
	Return,
	Scope(Vec<Token>),
	Semi,
	Struct,
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
				b"fn" => tokens.push(Token::Fn),
				b"let" => tokens.push(Token::Let),
				b"struct" => tokens.push(Token::Struct),
				b"return" => tokens.push(Token::Return),

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
	let tokens = tokens.into_iter().parse_scope(Token::Empty);
	Ok(tokens)
}

pub fn token_num(stack: &mut Vec<u8>) -> Token {
	let p = stack.partition_point(|bit| bit.is_ascii_alphanumeric() || *bit == b'_');
	let digits = String::from_utf8(stack.drain(..p).collect()).unwrap();
	let suffix = String::from_utf8(stack.drain(..).collect()).unwrap();

	Token::Numeric { digits, suffix }
}
pub fn token_ident(stack: &mut Vec<u8>) -> Token {
	let ident_id = IdentID::from_utf8(stack.drain(..).collect()).unwrap();

	Token::Ident(ident_id)
}

pub trait ParseScope {
	fn parse_scope(&mut self, open: Token) -> Vec<Token>;
}
impl<T> ParseScope for T
where
	T: Iterator<Item = Token>
{
	fn parse_scope(&mut self, open: Token) -> Vec<Token> {
		let close = match open {
			Token::BraceOpen => Token::BraceClose,
			Token::BracketOpen => Token::BracketClose,
			Token::ParentOpen => Token::ParentClose,
			_ => Token::Semi,
		};
		let mut inner = vec![];

		while let Some(token) = self.next() {
			inner.push(match token {
				Token::BraceOpen => Token::Scope(self.parse_scope(Token::BraceOpen)),
				Token::BracketOpen => Token::Scope(self.parse_scope(Token::BracketOpen)),
				Token::ParentOpen => Token::Scope(self.parse_scope(Token::ParentOpen)),
				token if token == close || token == Token::EOF => return inner,
				token => token,
			});
		}

		inner
	}
}
