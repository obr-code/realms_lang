use std::collections::VecDeque;
use crate::frontend::{ast::*, lexer::*};


pub fn parse_item(tokens: &mut VecDeque<Token>) -> Option<Item> {
	if let Some(token) = tokens.pop_front() {
		match token {
			Token::Let => Some(parse_stmt(tokens)),
			Token::EOF => None,
			_ => panic!("Unexpected item: {:?}", token),
		}
	} else {
		None
	}
}

pub fn parse_stmt(tokens: &mut VecDeque<Token>) -> Item {
	let index = tokens.iter()
		.position(|token| *token == Token::Semi)
		.expect("Let statement without closure");
	let mut tokens: VecDeque<Token> = tokens.drain(..=index).collect();
	tokens.pop_back(); // Semi
	println!("{:?}", tokens);
	let mut local = ItemLocal::default();

	while let Some(token) = tokens.pop_front() {
		match token {
			Token::Ident(s) => local.ident = s,
			Token::Eq => break,
			_ => panic!("Statement Err"),
		}
	}

	local.expr = parse_expr(&mut tokens);
	Item::ItemLocal(local)
}

pub fn parse_expr(mut tokens: &mut VecDeque<Token>) -> Expr {
	parse_additive_expr(&mut tokens)
}

pub fn parse_additive_expr(tokens: &mut VecDeque<Token>) -> Expr {
	let mut expr = parse_multiplicative_expr(tokens);

	while let Some(token) = tokens.back() {
		match token.clone() {
			Token::BinaryOperator(id) => {
				match id {
					b'+' | b'-' => {
						tokens.pop_back();
						expr = Expr::ExprBinary(
							ExprBinary {
								l: Box::new(parse_multiplicative_expr(tokens)),
								r: Box::new(expr),
								op: id,
							}
						);
					}
					_ => return expr,
				}
			},
			_ => return expr,
		}
	}
	expr
}

pub fn parse_multiplicative_expr(tokens: &mut VecDeque<Token>) -> Expr {
	let mut expr = parse_primary_expr(tokens);

	while let Some(token) = tokens.back() {
		match token.clone() {
			Token::BinaryOperator(id) => {
				match id {
					b'*' | b'/' | b'%' => {
						tokens.pop_back();
						expr = Expr::ExprBinary(
							ExprBinary {
								l: Box::new(parse_expr(tokens)),
								r: Box::new(expr),
								op: id,
							}
						);
					}
					_ => return expr,
				}
			},
			_ => return expr,
		}
	}
	expr
}

pub fn parse_primary_expr(tokens: &mut VecDeque<Token>) -> Expr {
	match tokens.pop_back().expect("Primary expression missing") {
		Token::Ident(ident) => Expr::ExprIdent(ident),
		Token::Numeric { digits, suffix } => Expr::ExprLit(
			ExprLit::Int { digits, suffix }
		),
		Token::ParentClose => {
			let index = tokens.iter()
				.position(|token| *token == Token::ParentOpen)
				.expect("Missing open parenthese");
			let mut tokens: VecDeque<Token> = tokens.drain(index..).collect();
			tokens.pop_front(); // parentOpen

			parse_expr(&mut tokens)
		}
		token => panic!("Unexpected: {:?}", token),
	}
}