#[derive(Debug, Clone)]
pub enum Expr {
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

	impl Expr {
	pub fn parse_from(&mut self) -> Result<Expr, Error> {
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

