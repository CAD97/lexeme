#[derive(Debug, Clone, Copy, PartialEq, lexeme::Lexeme)]
pub enum Token {
    #[lexeme(r"[ \n\t\f]")]
    Invalid,

    #[lexeme("private")]
    Private,

    #[lexeme("primitive")]
    Primitive,

    #[lexeme("protected")]
    Protected,

    #[lexeme("in")]
    In,

    #[lexeme("instanceof")]
    Instanceof,

    #[lexeme("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[lexeme(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,

    #[lexeme(r"\.\.\.")]
    Ellipsis,

    #[lexeme(r"\.")]
    Accessor,

    #[lexeme(r"\(")]
    ParenOpen,

    #[lexeme(r"\)")]
    ParenClose,

    #[lexeme(r"\{")]
    BraceOpen,

    #[lexeme(r"\}")]
    BraceClose,

    #[lexeme(r"\+")]
    OpAddition,

    #[lexeme(r"\+\+")]
    OpIncrement,

    #[lexeme("===")]
    OpStrictEquality,

    #[lexeme("==")]
    OpEquality,

    #[lexeme("=")]
    OpAssign,

    #[lexeme("=>")]
    FatArrow,
}
