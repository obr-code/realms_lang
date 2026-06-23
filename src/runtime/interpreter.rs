use std::collections::HashMap;
use crate::frontend::ast::*;
use crate::runtime::values::*;

pub struct Interpreter {
	ast: SyntaxTree,
	local_decls: HashMap<String, RuntimeVal>,
}
impl Interpreter {
	pub fn evaluate(expr: Expr) -> RuntimeVal {
		match expr {
			Expr::ExprBinary(ExprBinary { l, r, op }) => {
				match op {
					b'+' => 
				}
			}
			Expr::ExprLit(ExprLit::Int { digits, suffix }) => match suffix.as_str() {
				"f32" => RuntimeVal::F32(digits.parse::<f32>().expect("Invalid num")),
				"i32" => RuntimeVal::I32(digits.parse::<i32>().expect("Invalid num")),
				"u32" => RuntimeVal::U32(digits.parse::<u32>().expect("Invalid num")),
				_ => panic!("Unexpected suffix"),
			},
		}
	}
}