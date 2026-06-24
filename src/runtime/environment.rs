use std::collections::HashMap;
use crate::runtime::values::*;





#[derive(Debug, Default)]
pub struct Environment {
	variables: HashMap<String, RuntimeVal>,

	stack: Vec<Vec<String>>,
}
impl Environment {
	pub const fn new() -> Self {
		Self {
			variables: HashMap::new(),
		}
	}
	pub fn declare_var(&mut self, path: String, val: RuntimeVal) {
		self.variables
	}
	pub fn add_scope(&mut self) {
		self.stack.push(vec![]);
	}
	pub fn sub_scope(&mut self) {
		let scope = self.stack.pop().expect("Attempted to exit program scope");
		for path in scope.into_iter() {
			self.l_decls.remove(&path);
		}
	}
	pub fn get(&self, path: &str) -> &RuntimeVal {
		self.l_decls.get(path)
			.unwrap_or(self.g_decls.get(path)
			.expect(&format!("Unexpected path: {}", path)))
	}
}