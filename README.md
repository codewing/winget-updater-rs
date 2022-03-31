# winget updater

This tool exist because `winget upgrade --all` doesn't support ignoring certain updates yet.

## Development Setup

1. Download this repository
2. [Install Rust](https://www.rust-lang.org/tools/install)

## Usage

1. Create a file with the ignored ids (e.g. ignored.txt)
2. Run the tool `cargo run ignored.txt`
