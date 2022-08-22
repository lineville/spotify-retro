# Spotify-Retro ![Crates.io](https://img.shields.io/crates/v/spotify-retro)

Need a quick themed playlist for your upcoming Sprint Retro based on the `sprint_number` your team is on?

Spotify-Retro is a simple CLI tool written in Rust that will generate a Spotify playlist for you based on the sprint number you provide

## Install

### Homebrew

```bash
brew install spotify-retro
```

### Cargo

```bash
cargo install spotify-retro
```

## Usage

```bash
spotify-retro -s <sprint_number> -t <total_songs>
spotify-retro -s 22 -t 50 # Generate 50 songs for your Taylor Swift Sprint
```

- `-s` or `--sprint-number`: Sprint number (required)
- `-t` or `--total`: Number of songs to generate (default: 20)
