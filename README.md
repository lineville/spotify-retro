# Spotify-Retro

Need a quick themed playlist for your upcoming Sprint Retro based on the `sprint_number` your team is on?

Spotify-Retro is a simple CLI tool written in Rust that will generate a Spotify playlist for you based on the sprint number you provide

## Usage

```bash
cargo install spotify-retro # Install spotify-retro from cargo
spotify-retro -s 22 # Generate a playlist for your Taylor Swift Sprint
spotify-retro -s <sprint_number> -t <total_songs> 
```

- `-s` or `--sprint-number`: Sprint number (required)
- `-t` or `--total`: Number of songs to generate (default: 20)
