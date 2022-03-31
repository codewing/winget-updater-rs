# winget updater

This tool exist because `winget upgrade --all` doesn't support ignoring certain updates yet.

## Development Setup

1. Download this repository
2. [Install Rust](https://www.rust-lang.org/tools/install)

## Usage

1. Create a file with the ignored ids (e.g. ignored_packages.txt)
2. Add a new entry on each line with the ID of the package to ignore  
   e.g. Ignoring 2 packages:
   ```
   UnityTechnologies.Unity.2020
   BlenderFoundation.Blender
   ```
4. Run the tool  
   - Prebuild: `./winget-updater.exe --ignore-file ignored_packages.txt`
   - With building `cargo run -- --ignore-file ./ignored_packages.txt`
