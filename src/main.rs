use std::process::Command;

fn main() {
    let output = Command::new("cmd")
                                .args(["/C", "winget upgrade"])
                                .output()
                                .expect("failed to execute 'winget upgrade'");

    if output.status.success()
    {
        let result = String::from_utf8_lossy(&output.stdout);
        
        let lines: Vec<_> = result.lines().collect();
        for line in &lines[2..lines.len()-1] {
            println!("{}", &line);
        }

    }
}
