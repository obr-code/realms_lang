pub struct Structure {
	pub ident: Ident,
	pub parameters: Vec<(Ident, Class)>,
}

impl Structure {
	pub fn new(ident: Ident, parameters: Vec<(Ident, Class)>) -> Self {
		Self { ident, parameters }
	}
}