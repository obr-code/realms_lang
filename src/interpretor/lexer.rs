pub struct Token {
	value: String,
	ty: TokenType,
}

pub enum TokenType {
	BinaryOperator,
	Identifier,
	Let,
	Number,
}

pub fn tokenize(source: String) -> Vec<Token> {
	
}