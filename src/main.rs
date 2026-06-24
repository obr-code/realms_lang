use realms_lang::{
	frontend::{
		token,
	},
};
use std::env;
use std::fs::File;


fn main() -> std::io::Result<()> {
	// Tokens
	let path = env::args().nth(1).expect("Path not provided.");
	let file = File::open(path).expect("File not accessible.");
	let tokens = token::tokenize(file);
	println!("{:#?}", tokens);
	// let mut program = Interpreter::new(ast);
	// program.walk();
	// println!("{:#?}", program);

	Ok(())
}
