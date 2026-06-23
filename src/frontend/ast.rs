use std::collections::VecDeque;
use crate::frontend::lexer::Token;
use crate::frontend::parser;

#[derive(Debug)]
pub struct SyntaxTree {
	body: Vec<Item>,
}
#[derive(Debug)]
pub enum Item {
	ItemLocal(ItemLocal),
}
#[derive(Debug)]
#[derive(Default)]
pub struct ItemLocal {
	pub ident: String,
	pub expr: Expr,
}
#[derive(Debug)]
#[derive(Default)]
pub enum Expr {
	#[default]
	Empty,
	ExprBinary(ExprBinary),
	ExprIdent(String),
	ExprLit(ExprLit),
}
#[derive(Debug)]
pub struct ExprBinary {
	pub l: Box<Expr>,
	pub r: Box<Expr>,
	pub op: u8,
}
#[derive(Debug)]
pub enum ExprLit {
	Int {
		digits: String,
		suffix: String,
	}
}

impl SyntaxTree {
	pub fn new(tokens: impl Iterator<Item = Token>) -> Self {
		let mut body = vec![];
		let mut tokens: VecDeque<Token> = tokens.collect();
		while let Some(item) = parser::parse_item(&mut tokens) {
			body.push(item);
		};

		Self { body }
	}
}