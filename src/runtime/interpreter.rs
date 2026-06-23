use std::collections::HashMap;
use crate::frontend::lexer;
use crate::frontend::parser;
use parser::Program;
use crate::runtime::environment::*;
use crate::runtime::values::*;


#[derive(Debug)]
pub struct Interpreter {
	ast: Program,
	env: Environment,
}
impl Interpreter {
	pub fn new(ast: SyntaxTree) -> Self {
		Self {
			ast,
			env: Environment::default(),
		}
	}
	pub fn walk(&mut self) {
		for node in self.ast.body.iter() {
			match node {
				Item::ItemLocal(ItemLocal { ident, expr }) => {
					self.env.set_local(
						ident.clone(), 
						self.evaluate_expr(expr),
					);
				},
			}
		}
	}
	pub fn evaluate_expr(&self, expr: &Expr) -> RuntimeVal {
		match expr {
			Expr::ExprBinary(ExprBinary { l, r, op }) => {
				let l = self.evaluate_expr(l);
				let r = self.evaluate_expr(r);
				match op {
					b'+' => match (l, r) {
						(RuntimeVal::F32(l), RuntimeVal::F32(r)) => RuntimeVal::F32(l + r),
						(RuntimeVal::I32(l), RuntimeVal::I32(r)) => RuntimeVal::I32(l + r),
						(RuntimeVal::U32(l), RuntimeVal::U32(r)) => RuntimeVal::U32(l + r),
						_ => panic!("Attempted to add {:?} + {:?}", l, r),
					},
					b'-' => match (l, r) {
						(RuntimeVal::F32(l), RuntimeVal::F32(r)) => RuntimeVal::F32(l - r),
						(RuntimeVal::I32(l), RuntimeVal::I32(r)) => RuntimeVal::I32(l - r),
						(RuntimeVal::U32(l), RuntimeVal::U32(r)) => RuntimeVal::U32(l - r),
						_ => panic!("Attempted to sub {:?} - {:?}", l, r),
					},
					b'*' => match (l, r) {
						(RuntimeVal::F32(l), RuntimeVal::F32(r)) => RuntimeVal::F32(l * r),
						(RuntimeVal::I32(l), RuntimeVal::I32(r)) => RuntimeVal::I32(l * r),
						(RuntimeVal::U32(l), RuntimeVal::U32(r)) => RuntimeVal::U32(l * r),
						_ => panic!("Attempted to mul {:?} * {:?}", l, r),
					},
					b'/' => match (l, r) {
						(RuntimeVal::F32(l), RuntimeVal::F32(r)) => RuntimeVal::F32(l / r),
						(RuntimeVal::I32(l), RuntimeVal::I32(r)) => RuntimeVal::I32(l / r),
						(RuntimeVal::U32(l), RuntimeVal::U32(r)) => RuntimeVal::U32(l / r),
						_ => panic!("Attempted to div {:?} / {:?}", l, r),
					},
					b'%' => match (l, r) {
						(RuntimeVal::F32(l), RuntimeVal::F32(r)) => RuntimeVal::F32(l % r),
						(RuntimeVal::I32(l), RuntimeVal::I32(r)) => RuntimeVal::I32(l % r),
						(RuntimeVal::U32(l), RuntimeVal::U32(r)) => RuntimeVal::U32(l % r),
						_ => panic!("Attempted to mod {:?} % {:?}", l, r),
					},
					_ => unreachable!(),
				}
			},
			Expr::ExprIdent(ident) => *self.env.get(ident),
			Expr::ExprLit(ExprLit::Int { digits, suffix }) => match suffix.as_str() {
				"f32" => RuntimeVal::F32(digits.parse::<f32>().expect("Invalid num")),
				"i32" | "" => RuntimeVal::I32(digits.parse::<i32>().expect("Invalid num")),
				"u32" => RuntimeVal::U32(digits.parse::<u32>().expect("Invalid num")),
				_ => panic!("Unexpected suffix"),
			},
			_ => panic!("Unexpected expression"),
		}
	}
}