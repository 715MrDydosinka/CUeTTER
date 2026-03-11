use std::fmt::{self};

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
    InputFileError(String),
    //OutputFileError(String),
    EmptyCue(String),
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(String),
    InvalidNumber(String),
    UnspecifiedCue,
    GetHelp,
    GetAbout
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidArguments(s) => write!(f, "Invalid arguments: {}", s),
            ParseError::InvalidTime(s)      => write!(f, "Invalid time format: {}", s),
            ParseError::InvalidNumber(s)    => write!(f, "Invalid number: {}", s),
            ParseError::InvalidString(s)    => write!(f, "Invalid string: {}", s),
            ParseError::InvalidStructure(s) => write!(f, "Invalid structure: {}", s),
        }
    }
}

impl fmt::Display for FFmpregError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FFmpregError::NotFound                    => write!(f, "FFmpeg not found in PATH. Please install FFmpeg or provide custom path via 'ffmpeg' enviroment variable."),
            FFmpregError::CommandFailed(s)   => write!(f, "FFmpeg command failed: {}", s),
            FFmpregError::InputFileError(s)  => write!(f, "Audio track not specified or missing: {}", s),
            //FFmpregError::OutputFileError(s) => write!(f, "Output file error: {}", s),
            FFmpregError::EmptyCue(s)        => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(s)  => write!(f, "File not found: {}", s),
            ConfigError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ConfigError::UnspecifiedCue            => write!(f, "Cue file not specified. Use -h (--help) to read more"),
            ConfigError::GetHelp                   => write!(f, ""),
            ConfigError::GetAbout                  => write!(f, "")
        }
    }
}

impl std::error::Error for ParseError   {}
impl std::error::Error for FFmpregError {}
impl std::error::Error for ConfigError  {}