use crate::interpretor::lexer::Token;
use crate::interpretor::parser;

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
	pub fn new(mut tokens: impl Iterator<Item = Token>) -> Self {
		let mut body = vec![];

		while let Some(item) = parser::parse_item(&mut tokens) {
			body.push(item);
		};

		Self { body }
	}
}