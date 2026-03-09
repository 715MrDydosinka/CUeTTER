use std::{fmt, process::Command};


pub struct Time {
    pub minutes: u8, 
    pub seconds: u8,
    pub frames: u8
}

pub struct CueSheet {
    pub files: Vec<File>,
    pub comments: Vec<String>,
    pub catalog: Option<String>,
    pub cd_text_file: Option<String>
}

pub struct File {
    pub filename: String,
    pub file_type: String,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub tracks: Vec<Track>
}

pub struct Track {
    pub number: u8,
    pub track_type: String,
    pub isrc: Option<String>,
    pub flags: Vec<String>,
    pub indexes: Vec<String>,
    pub performer: Option<String>,
    pub songwriter: Option<String>,
    pub title: Option<String>
}

pub struct Index {
    pub number: u8,
    pub time: Time
}

#[derive(Debug)]
pub enum ParseError {
    UnknownCommand(String),
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
            ParseError::UnknownCommand(s) => write!(f, "Unknown command: {}", s),
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

pub fn check_ffmpreg() -> bool {
    Command::new("ffmpeg").arg("-version").output().is_ok()
}

pub fn parse() -> Result<(), Box<FFmpregError>> {
    if !check_ffmpreg() {
        return Err(Box::new(FFmpregError::NotFound));  
    }
    Ok(())
}



fn main() {

    

}