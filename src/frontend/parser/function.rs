pub struct Function {
	pub ident: Ident,
	pub args: Vec<(Ident, Class)>,
	pub ret: Class,
}

impl Function {
	pub fn new(ident: Ident, args: Vec<(Ident, Class)>, ret: Class) -> Self {
		Self { ident, args, ret }
	}
}