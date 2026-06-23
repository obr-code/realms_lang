use std::collections::VecDeque;
use crate::frontend::lexer::*;


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
				// Constant Item
				Token::Const => Ok(Item::ItemConstant(self.parse_const()?)),
				// Static Item
				Token::Static => todo!(),
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

	pub fn parse_const(&mut self) -> Result<ItemConstant, Error> {
		todo!()
		// let mut ident = String::new();
		// let mut expr = Expr::Null;

		// while let Some(token) = self.tokens.pop_front() {
		// 	match token {
		// 		Token::Ident(s) => ident = s,
		// 		Token::Eq => self.parse_expr(),
		// 		token => Err(Error::InvalidToken(token)),
		// 	}
		// }

		// Err(Error::MissingToken)
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
				// Constant
				Token::Const => todo!(),
				// Mutable
				Token::Mut => self.parse_local()?,
				// Numeric
				Token::Numeric {..} | Token::BraceOpen => Statement::Expr(self.parse_expr()?),
				// Others
				_ => return Err(Error::InvalidStatement(token.clone())),
			});
		}
		// Return
		Ok(stmts)
	}

	pub fn parse_local(&mut self) -> Result<Statement, Error> {
		let mut ident = String::new();

		while let Some(token) = self.tokens.pop_front() {
			match token {
				Token::Const => continue, // todo!()
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
				Token::Mut => continue, // todo!()
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
#[derive(Debug)]
pub struct Program {
	constants: Vec<ItemConstant>,
	functions: Vec<ItemFunction>,
}
impl Program {
	pub fn new(tokens: Vec<Token>) -> Result<Self, Error> {
		let mut parser = Parser::new(tokens.into_iter());
		let mut ast = Self {
			constants: vec![],
			functions: vec![],
		};
		loop {
			let item = parser.next()?;
			match item {
				Item::EOF => return Ok(ast),
				Item::ItemConstant(constant) => ast.constants.push(constant),
				Item::ItemFunction(function) => ast.functions.push(function),
			}
		}
	}
}

// Item
#[derive(Debug)]
pub enum Item {
	EOF,
	ItemConstant(ItemConstant),
	ItemFunction(ItemFunction),
}

// Constant
#[derive(Debug)]
pub struct ItemConstant {
	ident: String,
	expr: Expr,
}
// Function
#[derive(Debug)]
pub struct ItemFunction {
	ident: String,
	args: Vec<String>,
	body: Block,
}

#[derive(Debug)]
pub struct Block {
	stmts: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
	Local {
		ident: String,
		expr: Expr,
	},
	Expr(Expr),
}

#[derive(Debug)]
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
#[derive(Debug)]
pub struct ExprBinary {
	l: Box<Expr>,
	op: u8,
	r: Box<Expr>,
}
#[derive(Debug)]
pub struct ExprBlock {
	body: Block,
}
#[derive(Debug)]
pub struct ExprCall {
	path: Vec<String>,
	args: Vec<Expr>,
}
#[derive(Debug)]
pub struct ExprLit {
	digits: String,
	suffix: String,
}
#[derive(Debug)]
pub struct ExprForLoop {
	path: String,
	expr: ExprPath,
	body: Block,
}
#[derive(Debug)]
pub struct ExprPath {
	path: Vec<String>,
}
