# CUeTTER - ffmpeg wrapper for splitting audio by CUE sheets

CUeTTER is a convenient wrapper around ffmpeg that splits audio files according to CUE sheet timing information. All heavy lifting is done by ffmpeg - this tool simply generates and executes the appropriate ffmpeg commands.

## SYNOPSIS

```bash
cuetter <cue-file> [OPTIONS] [-- FFMPEG_ARGS]
cat <cue-file> | cuetter [OPTIONS] [-- FFMPEG_ARGS]
```

## DESCRIPTION

A wrapper that splits audio files based on CUE sheet timing information. The CUE sheet can be provided either as a file path or via stdin using piping.

### Audio File Discovery

You can either specify the audio file explicitly with `-i/--input`, or let the tool automatically find it. When `-i` is omitted, the tool will:
1. Parse the CUE sheet to find the referenced audio filename
2. Look for this file in the same directory as the CUE sheet
3. If not found, look in the current working directory (./)

## OPTIONS

### Input Option
| Option                     | Description       |
|----------------------------|-------------------|
| `-i, --input <file>`       | Input audio file (any format supported by ffmpeg) |
| `-o, --output <dir>`       | Output directory for split tracks (default: ./) |
| `-f, --format <format>`    | Output container/codec (flac, mp3, wav, ogg, etc.) Passed directly to ffmpeg as `-f {format}` |
| `-b, --bitrate <rate>`     | Audio bitrate (e.g., 320k, 192k, ...)  `-b:a {rate}` |
| `-r, --sample-rate <hz>`   | Sample rate in Hz (e.g., 44100, 48000) `-ar {hz}` |
| `-c, --channels <n>`       | Number of channels (1=mono, 2=stereo) `-ac {n}` |
| `-a, --album-cover <file>` | Embed album cover image. Maps to: `-i {file} -map 1 -c:v copy` (requires ffmpeg with image2 support) |
| `--skip-metadata`          | Skip embedding metadata from CUE to output files. Maps to: `-map_metadata 0 -id3v2_version 3` |
| `--embed-cue`              | Embed original CUE sheet in tracks metadata. Maps to: `-metadata CUE="{cue_content}"` |
| `--pregap`                 | Include pregap silence |
| `--postgap <ms>`           | Include postgap silence |
| `-w, --overwrite`          | Overwrite output files without asking. Maps to: `-y` (auto-confirm ffmpeg overwrites) |
| `-n, --dry-run`            | Show ffmpeg commands without executing them |
| `-v, --verbose`            | Show ffmpeg command lines and progress  |
| `--version`                | Display version information  |
| `--help`                   | Display this help message  |

### FFmpeg Passthrough
| Option | Description |
|--------|-------------|
| `-- [args]` | All arguments after -- are passed directly to every ffmpeg invocation (must be last option) |

## EXAMPLES

Basic split:
```bash
cuetter album.cue -o ./tracks/
```

Split to high-quality MP3:
```bash
cuetter album.cue -f mp3 -b 320k -o ./mp3s
```

Split with cover art and metadata:
```bash
cuetter album.cue -a cover.jpg --embed-metadata
```

Preview ffmpeg commands (dry run):
```bash
cuetter album.cue -n -v
```

Pass additional ffmpeg filters:
```bash
cuetter album.cue -o ./tracks -- -af "loudnorm"
```

Using windows-1251 encoded cue file:
```bash
iconv -f windows-1251 -t utf-8 album.cue | cuetter -i ~/album/
```

## DEPENDENCIES

ffmpeg - Must be installed and accessible in PATH or provided via environment variable `ffmpeg=`

## AUTHOR

Written by Hlupa (aka Hlupich, aka 715MrDydosinka (t.me/@mr_dydosinka))

## REPORTING BUGS

Report bugs to: Nowhere

## LICENSE

[Fuck Your License](./LICENSE)