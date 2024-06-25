//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

/// The Terebinth lexer
pub struct Lexer<'a> {
    source_code: &'a SourceCode,
    chars: Peekable<CharIndices<'a>>,
    current_position: SourcePosition,
}

impl<'a> Lexer<'a> {
    fn next(&mut self) -> Option<(usize, char)> {
        let next = self.chars.next();

        match next {
            Some(char) => {
                if char.1 == '\n' {
                    self.current_position.line += 1;
                    self.current_position.column = 1;
                } else {
                    self.current_position.column += 1;
                }
            }
            _ => {}
        }
        next
    }

    pub fn lex(&mut self) -> Result<Token<'a>, Error<'a>> {}
}

lazy_static! {
    static ref KEYWORD: HashMap<&'static str, Keyword> = {
        let mut map = HashMap::new();
        map.insert("return", Keyword::Return);
        map
    };
}
