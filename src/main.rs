use std::process::Command;
use regex::Regex;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    ignore_file: String,
}

fn main() {
    let args = Args::parse();

    let packages_to_update = get_packages_to_update();
    let ignored_packages = collect_ignored_packages(args);

    let regex = Regex::new(r"^(?P<name>.+?)\s+(?P<id>\S+?)\s+(?P<cur_vers>\S+?)\s+(?P<new_vers>\S+?)\s+(?P<source>\S+)$").unwrap();

    for line in packages_to_update {
        let captures = regex.captures(&line).unwrap();
        let package_id = &captures["id"];
        
        if !ignored_packages.contains(&package_id.to_string()) {
            println!("Updating {}", package_id);

            update_package(package_id);
        } else {
            println!("Ignored package {}", package_id);
        }
    }
    println!("Packages updated")
}

fn get_packages_to_update() -> Vec<String> {
    let winget_output = Command::new("cmd")
                                .args(["/C", "winget upgrade"])
                                .output()
                                .expect("failed to execute 'winget upgrade'");
    let result = String::from_utf8_lossy(&winget_output.stdout);
    let lines: Vec<_> = result.lines().collect();
    let packages_to_update = lines[2..lines.len()-1].into_iter().map(|x| x.to_string()).collect();
    packages_to_update
}

fn update_package(package: &str) {
    let update = Command::new("cmd")
                    .args(["/C", "winget upgrade", package])
                    .output()
                    .expect("failed to execute 'winget upgrade <package>'");
    let update_result = String::from_utf8_lossy(&update.stdout);
    println!("{}", update_result);
}

fn collect_ignored_packages(args: Args) -> Vec<String> {
    let file_content = fs::read_to_string(args.ignore_file).expect("Something went wrong reading the file");
    let ignored_packages: Vec<_> = file_content.lines().map(str::to_string).collect();
    ignored_packages
}
