use crate::interpretor::{ast::*, lexer::*};

pub fn parse_item(tokens: &mut impl Iterator<Item = Token>) -> Option<Item> {
	match tokens.next() {
		Some(Token::Let) => Some(parse_stmt(tokens)),
		Some(Token::EOF) => None,
		token => panic!("Err: Tried to parse: {:?}", token),
	}
}

pub fn parse_stmt(tokens: &mut impl Iterator<Item = Token>) -> Item {
	let mut local = ItemLocal::default();

	loop {
		match tokens.next() {
			Some(Token::Ident(s)) => local.ident = s,
			Some(Token::Eq) => break,
			_ => panic!("Statement Err"),
		}
	}
	local.expr = parse_expr(tokens);

	Item::ItemLocal(local)
}

pub fn parse_expr(tokens: &mut impl Iterator<Item = Token>) -> Expr {
	let mut expr = Expr::Empty;

	while let Some(token) = tokens.next() {
		match token {
			Token::BinaryOperator(id) => {
				expr = Expr::ExprBinary(
					ExprBinary {
						l: Box::new(expr),
						r: Box::new(parse_expr(tokens)),
						op: id,
					}
				);
				return expr;
			},
			Token::Numeric { digits, suffix } => {
				expr = Expr::ExprLit(
					ExprLit::Int {
						digits,
						suffix,
					}
				)
			}
			Token::Semi => break,
			_ => panic!("Expression Err"),
		}
	}
	expr
}