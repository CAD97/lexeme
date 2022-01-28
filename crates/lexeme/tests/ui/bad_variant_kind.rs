use lexeme::Lexeme;

#[derive(Lexeme)]
pub enum Token {
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    Unit,
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    Tuple(),
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    Struct {},
}

fn main() {}
