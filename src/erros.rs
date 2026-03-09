use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidArguments(String),
    InvalidTime(String),
    InvalidNumber(String),
    InvalidString(String),
    InvalidStructure(String),
}

#[derive(Debug)]
pub enum FFmpregError {
    NotFound,
    CommandFailed(String),
    OutputFileError(String),
    InvalidTime(String),
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidArguments(s) => write!(f, "Invalid arguments: {}", s),
            ParseError::InvalidTime(s) => write!(f, "Invalid time format: {}", s),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::InvalidString(s) => write!(f, "Invalid string: {}", s),
            ParseError::InvalidStructure(s) => write!(f, "Invalid structure: {}", s),
        }
    }
}

impl fmt::Display for FFmpregError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FFmpregError::NotFound => write!(f, "FFmpeg not found in PATH. Please install FFmpeg."),
            FFmpregError::CommandFailed(s) => write!(f, "FFmpeg command failed: {}", s),
            FFmpregError::OutputFileError(s) => write!(f, "Output file error: {}", s),
            FFmpregError::InvalidTime(s) => write!(f, "Invalid time format: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}
impl std::error::Error for FFmpregError {}