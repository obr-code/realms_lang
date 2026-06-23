use std::collections::HashMap;
use crate::runtime::values::*;


#[derive(Debug, Default)]
pub struct Environment {
	g_decls: HashMap<String, RuntimeVal>,
	l_decls: HashMap<String, RuntimeVal>,

	stack: Vec<Vec<String>>,
}
impl Environment {
	pub fn set_global(&mut self, ident: String, val: RuntimeVal) {
		self.g_decls.insert(ident, val);
	}
	pub fn set_local(&mut self, ident: String, val: RuntimeVal) {
		self.l_decls.insert(ident.clone(), val);
		self.stack.last_mut()
			.expect("Attempted to insert local variable in global scope")
			.push(ident);
	}
	pub fn add_scope(&mut self) {
		self.stack.push(vec![]);
	}
	pub fn sub_scope(&mut self) {
		let scope = self.stack.pop().expect("Attempted to exit program scope");
		for ident in scope.into_iter() {
			self.l_decls.remove(&ident);
		}
	}
	pub fn get(&self, ident: &str) -> &RuntimeVal {
		self.l_decls.get(ident)
			.unwrap_or(self.g_decls.get(ident)
			.expect(&format!("Unexpected ident: {}", ident)))
	}
}