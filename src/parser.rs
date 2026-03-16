use std::num::ParseIntError;

use erros::ParseError;

use crate::{CueSheet, Time, Track, File, Index, erros};


fn parse_quoted_string(args: &[&str]) -> Result<String, ParseError> {
    if args.is_empty() {
        return Err(ParseError::InvalidString("Empty string".to_string()));
    }
    
    let mut result = String::new();
    let mut i = 0;
    
    if args[0].starts_with('"') {
        result.push_str(&args[0][1..]);
        i += 1;
        
        while i < args.len() && !result.ends_with('"') {
            result.push(' ');
            result.push_str(args[i]);
            i += 1;
        }
        
        if !result.ends_with('"') {
            return Err(ParseError::InvalidString("Unterminated quoted string".to_string()));}
        result.pop();
    } else {
        result = args.join(" ");
    }
    
    Ok(result)
}

fn parse_time(s: &str) -> Result<Time, ParseError> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(ParseError::InvalidTime(s.to_string()));
    }
    let minutes: u8 = parts[0].parse().map_err(|e: ParseIntError| ParseError::InvalidNumber(e.to_string()))?;
    let seconds: u8 = parts[1].parse().map_err(|e: ParseIntError| ParseError::InvalidNumber(e.to_string()))?;
    let frames: u8 = parts[2].parse().map_err(|e: ParseIntError| ParseError::InvalidNumber(e.to_string()))?;

    if seconds >= 60 || frames >= 75 {
        return Err(ParseError::InvalidTime(format!("seconds must be <60, frames <75: {}", s )));
    }

    Ok(Time {minutes, seconds, frames})
}

fn parse_file_line(args: &[&str]) -> Result<(String, String), ParseError> {
    let mut filename = String::new();
    let mut i = 0;
    
    if args[0].starts_with('"') {
        filename.push_str(&args[0][1..]);
        i += 1;
        while i < args.len() && !filename.ends_with('"') {
            filename.push(' ');
            filename.push_str(args[i]);
            i += 1;
        }
        if !filename.ends_with('"') {
            return Err(ParseError::InvalidString("Unterminated quoted filename".to_string()));
        }
        filename.pop();
    } else {
        filename = args[0].to_string();
        i = 1;
    }

    let file_type = args[i..].join(" ");

    Ok((filename, file_type))
}

pub fn parse_cue(input: &str) -> Result<CueSheet, ParseError> {
    let mut cue = CueSheet::default();
    let mut current_file: Option<File> = None;
    let mut current_track: Option<Track> = None;
    let mut buf_title: Option<String> = None;
    let mut buf_performer: Option<String> = None;
    let mut buf_songwriter: Option<String> = None;

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_uppercase();
        let args = &parts[1..];

        match command.as_str() {
            "REM"        => {
                cue.comments.push(line.to_string());
                
                if args.len() >= 2 {
                    let rem_type = args[0].to_uppercase();
                    let rem_value = args[1..].join(" ").trim_matches('"').to_string();
                    
                    if let Some(track) = current_track.as_mut() {
                        match rem_type.as_str() {
                            "PERFORMER" => track.performer = Some(rem_value),
                            "SONGWRITER" => track.songwriter = Some(rem_value),
                            "TITLE" => track.title = Some(rem_value),
                            _ => {}
                        }
                    }
                }
            }
            "PERFORMER"  => {
                if args.is_empty() {
                    return Err(ParseError::InvalidArguments(
                        format!("PERFORMER needs at least one argument at line {}", line_num + 1)));
                }
                let performer = args.join(" ").trim_matches('"').to_string();
                if let Some(track) = current_track.as_mut() {
                    track.performer = Some(performer);
                } else if let Some(file) = current_file.as_mut() {
                    file.performer = Some(performer);
                }
                else if buf_performer.is_none() {
                    buf_performer = Some(performer);
                }
            }
            "SONGWRITER" => {
                if args.is_empty() {
                    return Err(ParseError::InvalidArguments(
                        format!("SONGWRITER needs at least one argument at line {}", line_num + 1)));
                }
                let songwriter = args.join(" ").trim_matches('"').to_string();
                if let Some(track) = current_track.as_mut() {
                    track.songwriter = Some(songwriter);
                }
                else if buf_songwriter.is_none() {
                    buf_songwriter = Some(songwriter)
                }
            }
            "TITLE"      => {
                if args.is_empty() {
                    return Err(ParseError::InvalidArguments(
                        format!("TITLE needs at least one argument at line {}",line_num + 1)));
                }
                let title = args.join(" ").trim_matches('"').to_string();
                if let Some(track) = current_track.as_mut() {
                    track.title = Some(title.clone());
                } 
                else if let Some(file) = current_file.as_mut() {
                    file.title = Some(title)
                }
                else if buf_title.is_none() {
                    buf_title = Some(title);
                }
            }
            "FILE"       => {
                let (filename, file_type) = parse_file_line(args)?;
                if let Some(mut file) = current_file.take() {
                    if let Some(track) = current_track.take() {
                        file.tracks.push(track);
                    }
                    cue.files.push(file);
                }
                current_file = Some(File {
                    filename,
                    file_type,
                    tracks: Vec::new(),
                    performer: buf_performer.clone(),
                    title: buf_title.clone(),
                });
            }
            "TRACK"      => {
                if args.len() < 2 {
                    return Err(ParseError::InvalidArguments(
                        format!("TRACK needs at least 2 arguments at line {}", line_num + 1)));
                }
                let track_number: u8 = args[0].parse().map_err(|e: ParseIntError| ParseError::InvalidNumber(e.to_string()))?;
                let track_type = args[1..].join(" "); // e.g. "AUDIO" or "MODE1/2048"
                if current_file.is_none() {
                    return Err(ParseError::InvalidStructure(
                        format!("TRACK outside FILE at line {}", line_num + 1)));
                }
                if let Some(track) = current_track.take() {
                    if let Some(file) = current_file.as_mut() {
                        file.tracks.push(track);
                    }
                }
                current_track = Some(Track {
                    number: track_number,
                    track_type,
                    isrc: None,
                    flags: Vec::new(),
                    indexes: Vec::new(),
                    performer: buf_performer.clone(),
                    songwriter: buf_songwriter.clone(),
                    title: buf_title.clone(),
                    pregap: None,
                    postgap: None
                });
            }
            "INDEX"      => {
                if args.len() != 2 {
                    return Err(ParseError::InvalidArguments(
                        format!("INDEX needs exactly 2 arguments at line {}", line_num + 1)));
                }
                let index_number: u8 = args[0].parse().map_err(|e: ParseIntError| ParseError::InvalidNumber(e.to_string()))?;
                let time = parse_time(args[1])?;
                if current_track.is_none() {
                    return Err(ParseError::InvalidStructure(
                        format!("INDEX outside TRACK at line {}", line_num + 1)));
                }
                if let Some(track) = current_track.as_mut() {
                    track.indexes.push(Index {number: index_number, time});
                }
            }
            "CATALOG"    => {
                if args.len() != 1 {
                    return Err(ParseError::InvalidArguments(format!(
                        "CATALOG needs exactly 1 argument at line {}", line_num + 1)));
                }
                cue.catalog = Some(args[0].to_string());
            }
            "CDTEXTFILE" => {
                if args.len() != 1 {
                    return Err(ParseError::InvalidArguments(
                        format!("CDTEXTFILE needs exactly 1 argument at line {}", line_num + 1)));
                }
                cue.cd_text_file = Some(parse_quoted_string(args)?);
            }
            "ISRC"       => {
                if args.len() != 1 {
                    return Err(ParseError::InvalidArguments(
                        format!("ISRC needs exactly 1 argument at line {}", line_num + 1)));
                }
                let code = args[0].to_string();
                if let Some(track) = current_track.as_mut() {
                    track.isrc = Some(code);
                } else {
                    return Err(ParseError::InvalidStructure(
                        format!("ISRC outside TRACK at line {}", line_num + 1)));
                }
            }
            "FLAGS"      => {
                if args.is_empty() {
                    return Err(ParseError::InvalidArguments(
                        format!("FLAGS needs at least one flag at line {}", line_num + 1)));
                }
                let flags = args.iter().map(|s| s.to_string()).collect();
                if let Some(track) = current_track.as_mut() {
                    track.flags = flags;
                } else {
                    return Err(ParseError::InvalidStructure(
                        format!("FLAGS outside TRACK at line {}", line_num + 1)));
                }
            }
            "PREGAP"     => {
                if args.len() != 1 {
                    return Err(ParseError::InvalidArguments(format!(
                        "PREGAP needs exactly 1 argument at line {}", line_num + 1)));
                }
                let pregap = parse_time(args[0])?;
                if let Some(track) = current_track.as_mut() {
                    track.pregap = Some(pregap);
                }
            }
            "POSTGAP"    => {
                if args.len() != 1 {
                    return Err(ParseError::InvalidArguments(format!(
                        "POSTGAP needs exactly 1 argument at line {}", line_num + 1)));
                }
                let postgap = parse_time(args[0])?;
                if let Some(track) = current_track.as_mut() {
                    track.postgap = Some(postgap);
                }
            }
            _ => {
                eprintln!("Unknown command <{}> at line {}", command, line_num + 1);
            }
        }
    }

    if let Some(mut file) = current_file {
        if let Some(mut track) = current_track {
            
            track.performer.get_or_insert("Unknown".to_owned());
            track.songwriter.get_or_insert("Unknown".to_owned());
            track.title.get_or_insert("Unknown".to_owned());
            
            file.tracks.push(track);
        }
        cue.files.push(file);
    } else if let Some(_) = current_track {
        return Err(ParseError::InvalidStructure(
            "TRACK without enclosing FILE at end of input".to_string(),
        ));
    }

    Ok(cue)
}