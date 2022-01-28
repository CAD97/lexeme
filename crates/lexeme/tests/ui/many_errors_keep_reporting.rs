use lexeme::Lexeme;

#[derive(Lexeme)]
#[lexeme(crate = lexeme)]
#[lexeme(crate = wrong)]
#[lexeme(krate = wrong)]
pub enum Token {
    #[lexeme("a{2,1}")]
    #[lexeme("[a-zA-Z][a-zA-Z0-9]*")]
    #[lexeme(this = is::bad)]
    Ident(),
}

fn main() {}
