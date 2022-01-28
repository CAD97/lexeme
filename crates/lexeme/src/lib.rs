#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use {once_cell::sync::OnceCell, regex::Regex};

#[doc(inline)]
pub use lexeme_derive::Lexeme;

pub trait Lexeme: Sized {
    fn lex(input: &str) -> Option<(Self, usize)>;
}
