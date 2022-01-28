use lexeme::Lexeme;

#[derive(Lexeme)]
pub enum Token {
    #[lexeme(unknown)]
    Ident,
}

fn main() {}
