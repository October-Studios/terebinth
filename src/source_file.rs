//     terebinth - lightweight interpreted programming language
//     Copyright (C) 2024 Cameron Howell
//
//     Licensed under the MIT License

/// Represents a Terebinth source file
pub struct SourceCode {
    source_code: String,
    source_name: String,
    new_line_ranges: Vec<Range<usize>>,
}

/// Represents a position in the code
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}
