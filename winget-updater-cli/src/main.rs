use clap::Parser;
use std::fs;
use winget_updater_library::wud::{get_packages_to_update, update_package};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    ignore_file: String,
}

fn main() {
    let args = Args::parse();
    let ignored_package_ids = collect_ignored_packages(&args.ignore_file);

    for package in get_packages_to_update(ignored_package_ids) {
        update_package(&package);
    }
}

fn collect_ignored_packages(ignore_file: &str) -> Vec<String> {
    let file_content = fs::read_to_string(ignore_file).expect("Something went wrong reading the file");
    let ignored_packages: Vec<_> = file_content.lines().map(str::to_string).collect();
    ignored_packages
}
