use crate::{
	frontend::{
		token::{
			ClassID,
			IdentID,
			Token,
		},
		expression::{
			Expr,
		}
	}
};

pub enum Statement {
	Function(Function),
	Expr(Expr),
	Variable(Variable),
}
use std::collections::VecDeque;
pub fn parse_stmts(tokens: impl Iterator<Item = Token>) -> Vec<Statement> {
	let mut tokens = tokens.collect::<VecDeque<Token>>();
	let mut stmts = vec![];

	while let Some(token) = tokens.pop_front() {
		match token {
			Token::Let => stmts.push(Statement::Variable(Variable::parse_from(tokens))),
			Token::Fn => stmts.push(Statement::Function(Function::parse_from(tokens))),
			_ => stmts.push(Statement::Expr(Expr::parse_from(tokens))),
		};
	}

	stmts
}

pub struct Function {
	pub ident: IdentID,
	pub args: Vec<(IdentID, ClassID)>,
	pub ret: ClassID,

	pub body: Vec<Statement>,
}
impl Function {
	pub fn parse_from(tokens: &mut VecDeque<Token>) -> Self {
		if let Some(Token::Fn)              = tokens.pop_front() 
		&& let Some(Token::Ident(ident))    = tokens.pop_front()
		&& let Some(Token::Scope(args_tok)) = tokens.pop_front()
		&& let Some(Token::Class(ret))      = tokens.pop_front()
		&& let Some(Token::Scope(body))     = tokens.pop_front()
		{
			let body = parse_stmts(body.into_iter());
			let mut args = vec![];
			for w in args_tok.windows(3) {
				match &w {
					&[
						Token::Ident(ident),
						Token::Colon,
						Token::Class(class),
					] => args.push((ident.clone(), class.clone())),
					_ => panic!(),
				}
			}
			return Self {
				ident,
				args,
				ret,
				body
			};
		} else {
			panic!()
		}
	}
}

pub struct Variable {
	pub ident: IdentID,
	pub class: ClassID,

	pub expr: Expr,
}
impl Variable {
	pub fn parse_from(tokens: &mut VecDeque<Token>) -> Self {
		if let Some(Token::Ident(ident)) = tokens.pop_front()
		&& let Some(Token::Colon)        = tokens.pop_front()
		&& let Some(Token::Class(class)) = tokens.pop_front()
		&& let Some(Token::Eq)           = tokens.pop_front()
		&& let Some(Token::Scope(expr))  = tokens.pop_front()
		{
			let expr = Expr::parse_from(expr);
			return Self {
				ident,
				class,
				expr,
			}
		} else {
			panic!()
		}
	}
}

pub enum Expression {
	Binary((Box<Expression>, Box<Expression>)),
	Literal(String),
}
impl Expression {
	pub fn parse_from(tokens: &mut VecDeque<Token>) -> Self {
		
	}
}

