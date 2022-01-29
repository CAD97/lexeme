#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use {
    once_cell::sync::OnceCell,
    regex::{Error as RegexError, Regex, RegexSet},
};

#[doc(inline)]
pub use lexeme_derive::Lexeme;

pub trait Lexeme: Sized {
    fn lex(input: &str) -> Option<(Self, usize)>;
}
