pub mod wud {
    use std::process::Command;
    use regex::Regex;

    #[derive(Debug)]
    pub struct WinPackage {
        pub name: String,
        pub id: String,
        pub installed_version: String,
        pub available_version: String,
        pub source: String
    }

    pub fn get_packages_to_update(ignored_package_ids: Vec<String>) -> Vec<WinPackage> {

        let packages_to_update = get_available_packages_to_update();
        
    
        let regex = Regex::new(r"^(?P<name>.+?)\s+(?P<id>\S+\.\S+(\.\S+)*)\s+(?P<cur_vers>(< )?\S+?)\s+(?P<new_vers>\S+?)\s+(?P<source>\S+)$").unwrap();

        let mut result_vec: Vec<WinPackage> = Vec::new();
    
        for line in packages_to_update {
            let captures = regex.captures(&line).unwrap();
            let package_id = &captures["id"];
            
            if ignored_package_ids.contains(&package_id.to_string()) { continue; } 

            let package = WinPackage {
                name: captures["name"].to_string(),
                id: package_id.to_string(),
                installed_version: captures["cur_vers"].to_string(),
                available_version: captures["new_vers"].to_string(),
                source: captures["source"].to_string()
            };

            result_vec.push(package);
        }

        result_vec
    }

    pub fn update_package(package_id: &str) {
        let update = Command::new("cmd")
                        .args(["/C", "winget upgrade", package_id])
                        .output()
                        .expect("failed to execute 'winget upgrade <package>'");
        let update_result = String::from_utf8_lossy(&update.stdout);
        println!("{}", update_result);
    }
    
    fn get_available_packages_to_update() -> Vec<String> {
        let winget_output = Command::new("cmd")
                                    .args(["/C", "winget upgrade"])
                                    .output()
                                    .expect("failed to execute 'winget upgrade'");
        let result = String::from_utf8_lossy(&winget_output.stdout);
        let lines: Vec<_> = result.lines().collect();
        let last_line = lines.iter().rposition(|&x| x.contains("upgrades available.")).unwrap();
        let packages_to_update = lines[2..last_line].into_iter().map(|x| x.to_string()).collect();
        packages_to_update
    }
    
}
