use realms_lang::interpretor::lexer;
use std::env;
use std::fs::File;


fn main() -> std::io::Result<()> {
	let path = env::args().nth(1).expect("Path not provided.");
	let file = File::open(path).expect("File not accessible.");
	let tokens = lexer::tokenize(file)?;
	
	println!("{:#?}", tokens);
	Ok(())
}
