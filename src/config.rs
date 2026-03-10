use std::env;
use std::collections::HashMap;
use crate::erros::ConfigError;

//cuetter
// -i (--input) [audio]
// -o (--output) [dir]
// -a (--album-cover) [file]
// -f (--format) [flac / mp3 / wav / etc...]
// -b [--bitrate] [number]
// -r [--sample-rate] [number]
// -c [--channels] [number]
// --embed-metadata 
// --embed-cue
// -w [--overwrite]
// --pregap
// --postgap
// -n [--dry-run]
// -v [verbose]
// --version

// -- [args] to pass in ffmpreg

use std::path::PathBuf;

#[derive(Default)]
pub struct Config {
    pub ffmpreg_path: String,
    pub input_cue: Option<PathBuf>,
    pub input_file: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub album_cover: Option<PathBuf>,
    pub format: String,
    pub bitrate: Option<u64>,
    pub sample_rate: Option<u64>,
    pub channels: Option<u8>,
    pub embed_cue: bool,
    pub embed_metadata: bool,
    pub overwrite: bool,
    pub include_pregap: bool,
    pub include_postgap: bool,
    pub dry_run: bool,

    pub verbose: bool,
    pub extra_args: Vec<String>,
}

impl Config {
    pub fn default() -> Self {
        Config { 
            input_cue: None,
            input_file: None,
            album_cover: None,
            ffmpreg_path: "ffmpeg".to_owned(),
            format: "flac".to_owned(), 
            output_dir: None, 
            bitrate: None, 
            sample_rate: None, 
            channels: None, 
            embed_cue: false,
            embed_metadata: false, 
            overwrite: false, 
            include_pregap: false, 
            include_postgap: false,
            dry_run: false,

            verbose: false,
            extra_args: Vec::new()
        }
    }
}


#[derive(Debug, Default)]
struct Args {
    flags: Vec<String>,
    options: HashMap<String, String>,
    positionals: Vec<String>,
    extra_args: Vec<String>,
}

impl Args {
    fn parse() -> Self {
        let mut args = Args::default();
        let raw_args: Vec<String> = env::args().skip(1).collect();
        
        let mut i = 0;
        let mut after_double_dash = false;
        
        while i < raw_args.len() {
            let arg = raw_args[i].clone();
            
            if arg == "--" {
                after_double_dash = true;
                i += 1;
                continue;
            }
            
            if after_double_dash {
                args.extra_args.push(arg);
                i += 1;
                continue;
            }
            
            if arg.starts_with("--") {
                if i + 1 < raw_args.len() && !raw_args[i + 1].starts_with('-') {
                    args.options.insert(arg, raw_args[i + 1].clone());
                    i += 2;
                } else {
                    args.flags.push(arg);
                    i += 1;
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                if i + 1 < raw_args.len() && !raw_args[i + 1].starts_with('-') {
                    args.options.insert(arg, raw_args[i + 1].clone());
                    i += 2;
                } else {
                    args.flags.push(arg);
                    i += 1;
                }
            } else {
                args.positionals.push(arg);
                i += 1;
            }
        }
        
        args
    }
    
}

fn validate_path(path_str: &str) -> Result<PathBuf, ConfigError> {
    let path = PathBuf::from(path_str);
    if path.exists() {
        Ok(path)
    } else {
        Err(ConfigError::FileNotFound(path.display().to_string()))
    }
}

pub fn parse_config() -> Result<Config, ConfigError> {
    let args = Args::parse();
    let mut config = Config::default();

    if let Ok(path) = env::var("ffmpeg") { 
        println!("{}", path);
        config.ffmpreg_path = path;
    }

    let mut fl_iter = args.flags.iter();
    while let Some(arg) = fl_iter.next() {
        match arg.as_str() {
            "-v" | "--verbose"      => config.verbose         = true,
            "--embed_cue"           => config.embed_cue       = true,
            "--embed_metadata"      => config.embed_metadata  = true,
            "--pregap"              => config.include_pregap  = true,
            "--postgep"             => config.include_postgap = true,
            "-n" | "--dry-run"      => config.dry_run         = true,
            "-w" | "--overwrite"    => config.overwrite       = true,
            "--about" | "--version" => {},
            "-h" | "--help"         => {},
            _ => eprintln!("Unknown flag or bad usage: {}", arg),
        }
    }

    for (a, b) in &args.options {
        match a.as_ref() {
            "-i" | "--input"       => config.input_file  = Some(validate_path(b)?),
            "-o" | "--output"      => config.output_dir  = Some(validate_path(b)?),
            "-a" | "--album-cover" => config.album_cover = Some(validate_path(b)?),
            "-f" | "--format"      => config.format      = b.clone(),
            "-b" | "--bitrate"     => config.bitrate     = Some(b.parse::<u64>().map_err(|e| ConfigError::InvalidNumber(e.to_string()))?),
            "-r" | "--sample-rate" => config.sample_rate = Some(b.parse::<u64>().map_err(|e| ConfigError::InvalidNumber(e.to_string()))?),
            "-c" | "--channels"    => config.channels    = Some(b.parse::<u8 >().map_err(|e| ConfigError::InvalidNumber(e.to_string()))?),
            _ => eprintln!("Unknown argument or bad usage: {}", a)
        }
    }
    
    if !&args.positionals.is_empty() {
        config.input_cue = Some(validate_path(&args.positionals[0])?);
    }

    config.extra_args = args.extra_args;

    Ok(config)

}
