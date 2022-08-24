# Spotify-Retro ![Crates.io](https://img.shields.io/crates/v/spotify-retro)

Need a quick themed playlist for your upcoming Sprint Retro based on the `sprint_theme` your team is on?

Spotify-Retro is a simple CLI tool written in Rust that will generate a Spotify playlist for you based on the sprint theme you provide

## Install

### Homebrew

```bash
brew tap lineville/spotify-retro
brew install spotify-retro
```

### Cargo

```bash
cargo install spotify-retro
```

## Usage

```bash
spotify-retro -s <sprint_theme> -t <total_songs>
spotify-retro -s 22 -t 50 # Generate 50 songs for your Taylor Swift Sprint
```

- `-s` or `--sprint-theme`: Sprint theme (required String)
- `-t` or `--total`: Number of songs to generate (default: 20)
