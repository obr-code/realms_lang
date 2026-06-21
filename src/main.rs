use realms_lang::interpretor::lexer;
use realms_lang::interpretor::parser;
use realms_lang::interpretor::ast::SyntaxTree;
use std::env;
use std::fs::File;


fn main() -> std::io::Result<()> {
	let path = env::args().nth(1).expect("Path not provided.");
	let file = File::open(path).expect("File not accessible.");
	let tokens = lexer::tokenize(file)?;

	let ast = SyntaxTree::new(tokens.into_iter());
	println!("{:#?}", ast);

	Ok(())
}
