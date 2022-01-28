use {lexeme::Lexeme, std::fmt};

#[track_caller]
fn check<T: Lexeme + PartialEq + fmt::Debug>(src: &str, lexemes: &[(T, &str)]) {
    let mut ix = 0;
    for expected in lexemes {
        let (lexeme, len) = T::lex(&src[ix..]).unwrap();
        assert_eq!(
            (lexeme, &src[ix..ix + len]),
            *expected,
            "\n{src}\n{empty:ix$}{empty:^^len$}\n",
            empty = "",
        );
        ix += len;
    }
    assert_eq!(
        src.len(),
        ix,
        "\n{src}\n{empty:ix$}{empty:^^len$}\n",
        empty = "",
        len = src.len() - ix,
    );
}

#[test]
fn lex_words() {
    #[derive(Lexeme, Debug, PartialEq, Eq)]
    enum Text {
        #[lexeme(r"\w+")]
        Word,
        #[lexeme(r"\W+")]
        NotWord,
    }
    use Text::*;

    check(
        "parse HTML with regex",
        &[
            (Word, "parse"),
            (NotWord, " "),
            (Word, "HTML"),
            (NotWord, " "),
            (Word, "with"),
            (NotWord, " "),
            (Word, "regex"),
        ],
    );
}

#[test]
fn lex_keywords() {
    #[derive(Lexeme, Debug, PartialEq, Eq)]
    enum Text {
        #[lexeme(r"unfortunate")]
        Keyword,
        #[lexeme(r"\w+")]
        Word,
        #[lexeme(r"\W+")]
        NotWord,
    }
    use Text::*;

    check(
        "unfortunately...",
        &[(Keyword, "unfortunate"), (Word, "ly"), (NotWord, "...")],
    );
}
