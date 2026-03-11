

use std::{fmt, fs, process::Command};

mod erros;
mod parser;
mod config;
mod splitter;

use parser::parse_cue;

use crate::{config::parse_config, erros::{ConfigError, FFmpregError}, splitter::split};

#[derive(Clone)]
pub struct Time {
    pub minutes: u8, 
    pub seconds: u8,
    pub frames: u8
}

impl Time {
    pub fn total_frames(&self) -> u64 {
        (self.minutes as u64 * 60) + (self.seconds as u64 * 75) + self.frames as u64
    }
    pub fn as_seconds(&self) -> f64 {
        (self.minutes as f64 * 60.0) + (self.seconds as f64) + (self.frames as f64 / 75.0)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.minutes, self.seconds, self.frames)
    }
}

#[derive(Default)]
pub struct CueSheet {
    pub files       : Vec<File>,
    pub comments    : Vec<String>,
    pub catalog     : Option<String>,
    pub cd_text_file: Option<String>
}


pub struct File {
    pub filename : String,
    pub file_type: String,
    pub performer: Option<String>,
    pub title    : Option<String>,
    pub tracks   : Vec<Track>
}

pub struct Track {
    pub number    : u8,
    pub track_type: String,
    pub isrc      : Option<String>,
    pub flags     : Vec<String>,
    pub indexes   : Vec<Index>,
    pub performer : Option<String>,
    pub songwriter: Option<String>,
    pub title     : Option<String>,
    pub pregap    : Option<Time>,
    pub postgap   : Option<Time>
}

pub struct Index {
    pub number: u8,
    pub time  : Time
}

pub fn check_ffmpreg(ffmpeg_cmd: &str) -> bool {
    Command::new(ffmpeg_cmd).arg("-version").output().is_ok()
}

fn print_parsed_cue(cue: &CueSheet) {
    println!("CUE file parsed successfully:");
    println!("  Files: {}", cue.files.len());
    for file in &cue.files {
        println!("  FILE: {} ({})", file.filename, file.file_type);
        println!("  Tracks: {}", file.tracks.len());
        if let Some(album) = &file.title {
            println!("  Album: {}", album);
        }
        for track in &file.tracks {
            println!("    Track {:02}: {}", track.number, track.track_type);
            if let Some(title) = &track.title {
                println!("      Title: {}", title);
            }
            if let Some(performer) = &track.performer {
                println!("      Performer: {}", performer);
            }
            for index in &track.indexes {
                println!("      INDEX {:02}: {}", index.number, index.time);
            }
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_config()?;

    if !cli.dry_run && !check_ffmpreg(&cli.ffmpreg_path) {
        return Err(Box::new(FFmpregError::NotFound));
    }

    let cue_content = match cli.input_cue.as_ref() {
        Some(path) => {
            fs::read_to_string(path)?
        }
        None => return Err(Box::new(ConfigError::UnspecifiedCue)),
    };

    let cue = parse_cue(&cue_content)?;

    if cli.verbose {
        print_parsed_cue(&cue);
    }

    let hui = split(cue, &cli)?;

    for i in hui {
        println!("Output: {}", i.display().to_string())
    }

    Ok(())
}

fn print_help() {
    println!("{}", include_str!("../rsrc/help.txt"));
}

fn print_about() {
    println!("{}", include_str!("../rsrc/about.txt"));
}

fn main() {
    if let Err(e) = run() {

        match e.downcast_ref::<ConfigError>() {
            Some(ConfigError::ShowAbout) => {
                print_about();
            },
            Some(ConfigError::ShowHelp) => {
                print_help();
            },
            _ => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        
    }
}