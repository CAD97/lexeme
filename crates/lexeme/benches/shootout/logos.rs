#[derive(Debug, Clone, Copy, PartialEq, logos::Logos)]
pub enum Token {
    #[error]
    #[regex(r"[ \n\t\f]")]
    Invalid,

    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

    #[token("private")]
    Private,

    #[token("primitive")]
    Primitive,

    #[token("protected")]
    Protected,

    #[token("in")]
    In,

    #[token("instanceof")]
    Instanceof,

    #[token(".")]
    Accessor,

    #[token("...")]
    Ellipsis,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("+")]
    OpAddition,

    #[token("++")]
    OpIncrement,

    #[token("=")]
    OpAssign,

    #[token("==")]
    OpEquality,

    #[token("===")]
    OpStrictEquality,

    #[token("=>")]
    FatArrow,
}
