//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

/// Lexical error handler
pub enum Error<'a> {
    InvalidCharacter {
        position: SourcePosition,
        character: char,
        source_ref: &'a SourceCode,
    },
    UnterminatedMultilineComment {
        multiline_comment_position: SourcePosition,
        source_reference: &'a SourceCode,
    },
}

impl<'a> Error<'a> {
    pub fn print_error(&self) {
        unimplemented!()
    }
}
