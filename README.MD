# CUeTTER — Split Images by CUE Sheets

FFmpeg wrapper that splits audio files using CUE sheets.

## Usage
`cuetter <cue-file> [OPTIONS] [-- FFMPEG_ARGS]`

## Options

| Short | Long | Description |
|-------|------|-------------|
| `-i` | `--input` | Input audio file (required) |
| `-o` | `--output` | Output directory |
| `-a` | `--album-cover` | Cover image to embed |
| `-f` | `--format` | Output format: flac, mp3, wav, etc. |
| `-b` | `--bitrate` | Bitrate in kbps |
| `-r` | `--sample-rate` | Sample rate in Hz |
| `-c` | `--channels` | Number of channels |
| | `--skip-metadata` | Skip embeding CUE metadata |
| | `--embed-cue` | Embed CUE sheet into metadata |
| `-w` | `--overwrite` | Overwrite existing files |
| | `--pregap` | Include pregap silence |
| | `--postgap` | Include postgap silence |
| `-n` | `--dry-run` | Show commands without executing |
| `-v` | `--verbose` | Show detailed output |
| | `--version` | Show version |
| | `-- [args]` | Pass args to FFmpeg |

## Examples
```bash
# Basic split:
    cuetter album.cue -o ./tracks

# Split to high-quality MP3:
    cuetter album.cue -f mp3 -b 320k -o ./mp3s

# Split with cover art and metadata:
    cuetter album.cue -a cover.jpg --embed-metadata

# Preview ffmpeg commands (dry run):
    cuetter album.cue -n -v

# Pass additional ffmpeg filters:
    cuetter album.cue -o ./tracks -- -af "loudnorm"

```

## License
[Fuck-Your-License](./LICENSE)