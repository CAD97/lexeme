use lexeme::Lexeme;

#[derive(Lexeme)]
pub enum Token {
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    Ident,
}

fn main() {}
