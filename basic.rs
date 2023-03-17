use std::io::{self, Read};
use std::fs::File;
use std::collections::HashMap;

fn search_username_on_platform(username: &str, platform_url: &str) -> io::Result<bool> {
    let mut response = String::new();
    let mut platform = reqwest::blocking::get(platform_url)?;
    platform.read_to_string(&mut response)?;
    if response.contains(username) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn main() -> io::Result<()> {
    let username = "johndoe";
    let mut platforms = HashMap::new();
    let mut file = File::open("platforms.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        platforms.insert(parts[0], parts[1]);
    }
    for (platform, url) in platforms {
        match search_username_on_platform(username, url) {
            Ok(true) => println!("{} account found on {}.", username, platform),
            Ok(false) => println!("{} account not found on {}.", username, platform),
            Err(_) => println!("Error searching for {} on {}.", username, platform),
        }
    }
    Ok(())
}
