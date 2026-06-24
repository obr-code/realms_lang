use std::collections::HashMap;
use crate::frontend::lexer;
use crate::frontend::parser::*;
use crate::runtime::environment::*;
use crate::runtime::values::*;


#[derive(Debug)]
pub struct Interpreter {
	ast: SyntaxTree,
	env: Environment,
}
impl Interpreter {
	pub fn new(ast: SyntaxTree) -> Self {
		Self {
			ast,
			env: Environment::default(),
		}
	}
	pub fn eval(&mut self) {
		for item in self.ast.items.iter() {
			match item {
				Item::ItemConstant(_) => todo!(),
				
	}

	pub fn eval_stmt(&mut self, stmt: &Statement) {
		match stmt {
			Statement::Local(expr) => {
				self.env.set_local(path, val);
			}
		}
	}