use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::CueSheet;
use crate::config::Config;
use crate::erros::FFmpregError;

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

pub fn split(cue: CueSheet, config: &Config) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    
    
    if cue.files.is_empty() {
        return Err(Box::new(FFmpregError::EmptyCue("No FILE entries found in CUE sheet".into())))
    }
    
    let audio_path = match &config.input_file {
        Some(path) => path.to_path_buf(),
        None => {
            let cue_dir = config.input_cue.clone().unwrap_or_else(|| PathBuf::from("."));
            let cue_dir_parent = cue_dir.parent().unwrap_or_else(|| Path::new("."));
            let audio_filename = &cue.files[0].filename;
            cue_dir_parent.join(audio_filename)
        }
    };
    
    if !audio_path.exists() {
        return Err(Box::new(FFmpregError::InputFileError(audio_path.display().to_string())))
    }

    let mut output_files: Vec<PathBuf> = Vec::new();
    
    for file_entry in cue.files {
        for track in &file_entry.tracks {
            let start_idx = track.indexes.iter().find(|idx| idx.number == 1);
            if start_idx.is_none() {
                eprintln!("Track {} has no INDEX 01, skipping", track.number);
                continue;
            }
            let start_time = start_idx.unwrap().time.clone();
            
            let end_time = if let Some(next_track) = file_entry.tracks.iter().find(|t| t.number > track.number).and_then(|t| t.indexes.iter().find(|idx| idx.number == 1))
            {
                Some(next_track.time.clone())
            } 
            else {
                None
            };
            
            let effective_start = if config.include_pregap {
                if let Some(pregap) = track.indexes.iter().find(|idx| idx.number == 0) {
                    pregap.time.clone()
                } else {
                    start_time
                }
            } 
            else {
                start_time
            };
            
            let track_num = format!("{:02}", track.number);
            let mut output_filename = format!("{}_{}", track_num, config.format);
            
            if let (Some(performer), Some(title)) = (&track.performer, &track.title) {
                output_filename = format!("{} - {} - {}.{}", 
                    track_num, 
                    sanitize_filename(performer), 
                    sanitize_filename(title), 
                    config.format
                );
            } 
            else if let Some(title) = &track.title {
                output_filename = format!("{} - {}.{}", track_num, sanitize_filename(title), config.format);
            } 
            else {
                output_filename = format!("track{}.{}", track_num, config.format);
            }
            
            let output_path = &config.output_dir.clone().unwrap_or_else(|| PathBuf::from(".")).join(output_filename);

            if output_path.exists() && !config.overwrite {
                println!("Skipping existing file: {:?}", output_path);
                output_files.push(output_path.to_path_buf());
                continue;
            }
            
            let mut cmd = Command::new("ffmpeg");
            cmd.arg("-i").arg(format!("{}", audio_path.display().to_string()));
            
            cmd.arg("-ss").arg(effective_start.as_seconds().to_string());
            
            if let Some(end) = end_time {
                let duration = end.as_seconds() - effective_start.as_seconds();
                if duration > 0.0 {
                    cmd.arg("-t").arg(duration.to_string());
                }
            }
            
            if let Some(bitrate) = &config.bitrate {
                cmd.arg("-b:a").arg(format!("{}", bitrate));
            }
            
            if let Some(sample_rate) = &config.sample_rate {
                cmd.arg("-ar").arg(format!("{}", sample_rate));
            }
            
            if let Some(channels) = config.channels {
                cmd.arg("-ac").arg(channels.to_string());
            }
            
            if !config.skip_metadata {
                if let Some(album) = &file_entry.title {
                    cmd.arg("-metadata").arg(format!("ALBUM={}", album ));
                }
                if let Some(performer) = &track.performer {
                    cmd.arg("-metadata").arg(format!("artist={}", performer));
                }
                if let Some(title) = &track.title {
                    
                    cmd.arg("-metadata").arg(format!("title={}", &title));
                }
                if let Some(songwriter) = &track.songwriter {
                    cmd.arg("-metadata").arg(format!("composer={}", songwriter));
                }
                if let Some(isrc) = &track.isrc {
                    cmd.arg("-metadata").arg(format!("isrc={}", isrc));
                }

                cmd.arg("-metadata").arg(format!("track={}", track.number));
                
                cmd.arg("-metadata").arg(format!("totaltracks={}", file_entry.tracks.len()));
                
                for comment in &cue.comments {
                    if comment.starts_with("REM DATE") {
                        let parts: Vec<&str> = comment.split_whitespace().collect();
                        if parts.len() >= 3 {
                            cmd.arg("-metadata").arg(format!("date={}", parts[2].trim_matches('"')));
                        }
                    } else if comment.starts_with("REM GENRE") {
                        let parts: Vec<&str> = comment.split_whitespace().collect();
                        if parts.len() >= 3 {
                            cmd.arg("-metadata").arg(format!("genre={}", parts[2].trim_matches('"')));
                        }
                    }
                }
            }

            if config.embed_cue {
                match &config.input_cue {
                    Some(cue_path) => {
                        match fs::read_to_string(cue_path) {
                            Ok(cue_content) => {
                                cmd.arg("-metadata").arg(format!("CUE={}", cue_content));
                            }
                            Err(e) => {
                                eprintln!("Failed to read CUE file {}: {}", cue_path.display(), e);
                            }
                        }
                    }
                    None => { }
                }
            }
            
            for arg in &config.extra_args {
                cmd.arg(arg);
            }

            if config.overwrite {
                cmd.arg("-y");
            }
            
            cmd.arg(&output_path);

            if config.verbose{
                let command = cmd.get_args().map(|s| s.to_string_lossy()).collect::<Vec<_>>().join(" ");
                println!("\nFFmpreg args: \"{}\"\n", command);
            }
            
            println!("Splitting track {}...", track.number);
            
            if !config.dry_run{            
                let status = cmd.status()?;
                
                if !status.success() {
                    return Err(Box::new(FFmpregError::CommandFailed(format!("Failed to split track {}", track.number))));
                }
            }
            output_files.push(output_path.to_path_buf());
        }
    }
    
    Ok(output_files)
}
