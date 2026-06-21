use crate::interpretor::lexer::Token;
use crate::interpretor::parser;

pub struct SyntaxTree {
	body: Vec<Item>,
}
pub enum Item {
	ItemLocal(ItemLocal),
}

#[derive(Default)]
pub struct ItemLocal {
	pub ident: String,
	pub expr: Expr,
}

#[derive(Default)]
pub enum Expr {
	#[default]
	Empty,
	ExprBinary(ExprBinary),
	ExprLit(ExprLit),
}
pub struct ExprBinary {
	pub l: Box<Expr>,
	pub r: Box<Expr>,
	pub op: u8,
}
pub enum ExprLit {
	Int {
		digits: String,
		suffix: Option<String>,
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