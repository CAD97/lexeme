use lexeme::Lexeme;

#[derive(Lexeme)]
pub enum Token {
    #[lexeme("*")]
    Ident,
}

fn main() {}
