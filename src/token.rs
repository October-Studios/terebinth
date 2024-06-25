//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    LineEnd,
    Identifier,
    Literal,
    StringLiteral,
    Operator,
    LineComment,
    BlockComment,
    Scope,
    Cast,
    Tuple,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    token_kind: TokenKind,
    pos_rng: Range<SourcePosition>,
    lexeme: &'a str,
}
