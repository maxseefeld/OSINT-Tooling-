use std::io::{self, Read};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;

fn search_username_on_snapchat(username: &str) -> io::Result<bool> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:58.0) Gecko/20100101 Firefox/58.0"));
    let mut response = reqwest::blocking::Client::new()
        .get(&format!("https://storysharing.snapchat.com/v1/fetch/{}?locale=en-US&client_id=storysharing_web_snapchat_com", username))
        .headers(headers)
        .send()?;
    let response_text = response.text()?;
    let json: Value = serde_json::from_str(&response_text)?;
    let success = json["success"].as_bool().unwrap_or(false);
    Ok(success)
}

fn main() -> io::Result<()> {
    let username = "johndoe";
    match search_username_on_snapchat(username) {
        Ok(true) => println!("Snapchat account found for {}.", username),
        Ok(false) => println!("No Snapchat account found for {}.", username),
        Err(_) => println!("Error searching for {} on Snapchat.", username),
    }
    Ok(())
}
