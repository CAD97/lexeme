use lexeme::Lexeme;

#[derive(Lexeme)]
#[lexeme(crate = wrong)]
pub enum Token {}

fn main() {}
