use logos::Logos;

// region: ---Lexer

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[\t\n\r ]+")]
pub enum JSONLexer {
    #[regex("\"[^\"]*\"")]
    String,
    #[regex(r"\-?\d+(\.\d+)?([Ee][+\-]?\d+)?")]
    Number,
    #[token("null")]
    Null,
    #[token("{")]
    OpenCurly,
    #[token("}")]
    CloseCurly,
    #[token("[")]
    OpenSquare,
    #[token("]")]
    CloseSquare,
    #[token(":")]
    Colon,
}

// endregion
