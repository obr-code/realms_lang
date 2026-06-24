use crate::frontend::parser::ItemFunction;


#[derive(Debug, Clone)]
pub enum RuntimeVal {
	Primitive(Primitive),
	Trait(Trait),
	Collection(Collection),
}
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
	F32(f32),
	I32(i32),
	U32(u32),
}
#[derive(Debug, Clone)]
pub enum Trait {
	Fn(ItemFunction),
}
#[derive(Debug, Clone)]
pub enum Collection {
	Vec(Vec<RuntimeVal>),
}