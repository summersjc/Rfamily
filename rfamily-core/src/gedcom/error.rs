use std::fmt;

/// Errors that can occur during GEDCOM parsing
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Invalid line format (doesn't match LEVEL TAG [VALUE])
    InvalidLineFormat { line_num: usize, line: String },

    /// Invalid level number
    InvalidLevel {
        line_num: usize,
        expected: usize,
        found: usize,
    },

    /// Missing required tag
    MissingRequiredTag { tag: String },

    /// Invalid xref format (should be @ID@)
    InvalidXref { line_num: usize, xref: String },

    /// Broken xref pointer (references non-existent record)
    BrokenXref {
        xref: String,
        referenced_from: String,
    },

    /// Invalid date format
    InvalidDate { line_num: usize, date: String },

    /// Invalid encoding
    InvalidEncoding { encoding: String },

    /// IO error
    IoError { message: String },

    /// UTF-8 decode error
    Utf8Error { message: String },

    /// Generic parse error
    Other { message: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidLineFormat { line_num, line } => {
                write!(f, "Invalid line format at line {}: {}", line_num, line)
            }
            ParseError::InvalidLevel {
                line_num,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Invalid level at line {}: expected {}, found {}",
                    line_num, expected, found
                )
            }
            ParseError::MissingRequiredTag { tag } => {
                write!(f, "Missing required tag: {}", tag)
            }
            ParseError::InvalidXref { line_num, xref } => {
                write!(f, "Invalid xref at line {}: {}", line_num, xref)
            }
            ParseError::BrokenXref {
                xref,
                referenced_from,
            } => {
                write!(
                    f,
                    "Broken xref {} referenced from {}",
                    xref, referenced_from
                )
            }
            ParseError::InvalidDate { line_num, date } => {
                write!(f, "Invalid date at line {}: {}", line_num, date)
            }
            ParseError::InvalidEncoding { encoding } => {
                write!(f, "Invalid encoding: {}", encoding)
            }
            ParseError::IoError { message } => {
                write!(f, "IO error: {}", message)
            }
            ParseError::Utf8Error { message } => {
                write!(f, "UTF-8 decode error: {}", message)
            }
            ParseError::Other { message } => {
                write!(f, "Parse error: {}", message)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError {
            message: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ParseError::Utf8Error {
            message: err.to_string(),
        }
    }
}

/// Parse warnings (non-fatal issues)
#[derive(Debug, Clone)]
pub enum ParseWarning {
    /// Unknown tag (not in GEDCOM spec)
    UnknownTag { line_num: usize, tag: String },

    /// Deprecated tag
    DeprecatedTag { line_num: usize, tag: String },

    /// Missing optional field
    MissingOptionalField { field: String },

    /// Suspicious data
    SuspiciousData { line_num: usize, message: String },
}

impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseWarning::UnknownTag { line_num, tag } => {
                write!(f, "Unknown tag at line {}: {}", line_num, tag)
            }
            ParseWarning::DeprecatedTag { line_num, tag } => {
                write!(f, "Deprecated tag at line {}: {}", line_num, tag)
            }
            ParseWarning::MissingOptionalField { field } => {
                write!(f, "Missing optional field: {}", field)
            }
            ParseWarning::SuspiciousData { line_num, message } => {
                write!(f, "Suspicious data at line {}: {}", line_num, message)
            }
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
