use realms_lang::frontend::lexer;
use realms_lang::frontend::parser;
use parser::Program;
// use realms_lang::runtime::interpreter::*;
use realms_lang::runtime::values::*;
use std::env;
use std::fs::File;


fn main() -> std::io::Result<()> {
	// Tokens
	let path = env::args().nth(1).expect("Path not provided.");
	let file = File::open(path).expect("File not accessible.");
	let tokens = lexer::tokenize(file)?;
	println!("{:#?}", &tokens);

	// AST
	let program = Program::new(tokens);
	println!("{:#?}", &program);

	// let mut program = Interpreter::new(ast);
	// program.walk();
	// println!("{:#?}", program);

	Ok(())
}
