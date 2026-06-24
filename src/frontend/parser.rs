use std::collections::VecDeque;
use crate::frontend::token::*;


// Parser
#[derive(Debug)]
pub struct Parser {
	tokens: VecDeque<Token>,
}
impl Parser {
	pub fn new(tokens: impl Iterator<Item = Token>) -> Self {
		Self { tokens: tokens.collect() }
	}

	pub fn next(&mut self) -> Result<Item, Error> {
		if let Some(token) = self.tokens.pop_front() {
			match token {
				// Function Item
				Token::Fn => Ok(Item::ItemFunction(self.parse_function()?)),
				// EOF
				Token::EOF => Ok(Item::EOF),
				_ => Err(Error::InvalidToken(token)),
			}
		} else {
			unreachable!()
		}
	}

	pub fn parse_function(&mut self) -> Result<ItemFunction, Error> {
		// Ident
		let Some(mut token) = self.tokens.pop_front()
		else { return Err(Error::MissingToken); };
		let Token::Ident(ident) = token
		else { return Err(Error::InvalidToken(token)); };
		// ParentOpen
		matches!(Token::Ident, ident);
		assert_eq!(Some(Token::ParentOpen), self.tokens.pop_front());
		// Set arguments' vec
		let mut args = Vec::new();
		// Arguments' paths
		while let Some(token) = self.tokens.pop_front() {
			match token {
				Token::Ident(s) => args.push(s),
				Token::ParentClose => break,
				_ => return Err(Error::InvalidToken(token)),
			}
		}
		// CloseBrace
		let Some(parent_close) = self.tokens.iter()
			.position(|token| *token == Token::BraceClose)
		else { return Err(Error::MissingToken); };
		// Parser for function's inner statements
		let mut sub_parser = Parser::new(self.tokens.drain(..=parent_close));
		// Remove ParentClose
		sub_parser.tokens.pop_back();
		// Return
		Ok(
			ItemFunction {
				ident,
				args,
				body: Block {
					stmts: sub_parser.parse_stmts()?
				}
			}
		)
	}

	pub fn parse_stmts(&mut self) -> Result<Vec<Statement>, Error> {
		// BraceOpen
		assert_eq!(Some(Token::BraceOpen), self.tokens.pop_front());
		// Set statements' vec
		let mut stmts = Vec::new();
		// Scope
		while let Some(token) = self.tokens.front() {
			stmts.push(match token {
				// Variable statement
				Token::Let => Statement::Local(Self.parse_local()?),
				// Numeric
				Token::Numeric {..} | Token::BraceOpen => Statement::Expr(self.parse_expr()?),
				// Others
				_ => return Err(Error::InvalidStatement(token.clone())),
			});
		}
		// Return
		Ok(stmts)
	}

	pub fn parse_local(&mut self) -> Result<Local, Error> {
		let mut ident = String::new();

		while let Some(token) = self.tokens.pop_front() {
			match token {
				Token::Eq => {
					if let Some(semi) = self.tokens.iter().position(|token| *token == Token::Semi) {
						let mut sub_parser = Parser::new(self.tokens.drain(..=semi));
						// Remove Semi
						sub_parser.tokens.pop_back();
						
						return Ok(Statement::Local {
							ident: ident,
							expr: sub_parser.parse_expr()?,
						});
					} else {
						return Err(Error::MissingToken);
					}
				},
				Token::Ident(s) => ident = s,
				token => return Err(Error::InvalidToken(token)), 
			}
		}

		Err(Error::MissingToken)
	}

	pub fn parse_expr(&mut self) -> Result<Expr, Error> {
		self.parse_additive_expr() 
	}

	pub fn parse_additive_expr(&mut self) -> Result<Expr, Error> {
		let mut expr = self.parse_multiplicative_expr()?;

		while let Some(Token::BinaryOperator(id)) = self.tokens.back()
		&& (*id == b'+' || *id == b'-') {
			let id = *id;
			self.tokens.pop_back();
			expr = Expr::ExprBinary(ExprBinary {
				l: Box::new(self.parse_multiplicative_expr()?),
				op: id,
				r: Box::new(expr),
			});
		}
		
		Ok(expr)
	}

	pub fn parse_multiplicative_expr(&mut self) -> Result<Expr, Error> {
		let mut expr = self.parse_primary_expr()?;

		while let Some(Token::BinaryOperator(id)) = self.tokens.back()
		&& (*id == b'*' || *id == b'/' || *id == b'%') {
			let id = *id;
			self.tokens.pop_back();
			expr = Expr::ExprBinary(ExprBinary {
				l: Box::new(self.parse_call_member_expr()?),
				op: id,
				r: Box::new(expr),
			});
		}

		Ok(expr)
	}

	pub fn parse_call_member_expr(&mut self) -> Result<Expr, Error> {
		self.parse_args()
	}

	pub fn parse_args(&mut self) -> Result<Expr, Error> {
		self.parse_primary_expr()
	}

	pub fn parse_primary_expr(&mut self) -> Result<Expr, Error> {
		if let Some(token) = self.tokens.pop_back() {
			match token {
				Token::Ident(s) => Ok(
					Expr::ExprPath(
						ExprPath {
							path: vec![s],
						}
					)
				),
				Token::Numeric { digits, suffix } => Ok(
					Expr::ExprLit(
						ExprLit {
							digits,
							suffix,
						}
					)
				),
				Token::ParentClose => {
					// ParentOpen
					let Some(parent_open) = self.tokens.iter()
						.position(|token| *token == Token::ParentOpen)
					else { return Err(Error::MissingToken); };
					// Parser for Parentheses' inner expressions
					let mut second_parser = Parser::new(self.tokens.drain(parent_open..));
					// Parentheses' inner expressions
					second_parser.parse_expr()
				},
				token => Err(Error::InvalidToken(token)),
			}
		} else {
			Err(Error::MissingToken)
		}
	}
} 

#[derive(Debug)]
pub enum Error {
	MissingToken,
	InvalidToken(Token),
	InvalidStatement(Token),
	Unexpected,
}

// AST
#[derive(Debug, Default)]
pub struct SyntaxTree {
	pub items: Vec<Item>,
}
impl SyntaxTree {
	pub fn new(tokens: Vec<Token>) -> Result<Self, Error> {
		let mut parser = Parser::new(tokens.into_iter());
		let mut ast = Self::default();

		loop {
			let item = parser.next()?;
			match item {
				Item::EOF => break,
				item => ast.items.push(item),
			}
		}

		Ok(ast)
	}
}

// Item
#[derive(Debug)]
pub enum Item {
	EOF,
	ItemFunction(ItemFunction),
}

// Function
#[derive(Debug, Clone)]
pub struct ItemFunction {
	pub ident: String,
	pub args: Vec<String>,
	pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
	stmts: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
	Local(Local),
	Expr(Expr),
}
pub struct Local {
	ident: String,
	expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Null,
	EOF,
	ExprBinary(ExprBinary),
	ExprBlock(ExprBlock),
	ExprCall(ExprCall),
	ExprLit(ExprLit),
	ExprForLoop(ExprForLoop),
	ExprPath(ExprPath),
}
#[derive(Debug, Clone)]
pub struct ExprBinary {
	l: Box<Expr>,
	op: u8,
	r: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct ExprBlock {
	body: Block,
}
#[derive(Debug, Clone)]
pub struct ExprCall {
	path: Vec<String>,
	args: Vec<Expr>,
}
#[derive(Debug, Clone)]
pub struct ExprLit {
	digits: String,
	suffix: String,
}
#[derive(Debug, Clone)]
pub struct ExprForLoop {
	path: String,
	expr: ExprPath,
	body: Block,
}
#[derive(Debug, Clone)]
pub struct ExprPath {
	path: Vec<String>,
}