use std::fs;
use regex::Regex;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "kidskoding";

    let url = format!("https://api.github.com/users/{}/events", username);
    let client = Client::new();
    let res = client.get(&url)
        .header("User-Agent", "gh_activity")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let mut latest_repo = "No recent activity".to_string();
    if let Some(events) = res.as_array() {
        for event in events {
            if event["type"] == "PushEvent" {
                if let Some(repo) = event["repo"]["name"].as_str() {
                    latest_repo = repo.split('/').last().unwrap_or("").to_string();
                    break;
                }
            }
        }
    }

    let card_url = format!(
        "https://github-readme-stats.vercel.app/api/pin/?username={}&repo={}",
        username, latest_repo
    );

    let image_response = client.get(&card_url).send().await?;
    let image_bytes = image_response.bytes().await?;

    fs::write("current_repo_card.svg", &image_bytes)?;

    let readme_path = "README.md";
    let mut readme_content = match fs::read_to_string(readme_path) {
        Ok(content) => content,
        Err(_) => String::new(),
    };

    let new_section = format!(
        "## Currently working on\n\n[![{}](./current_repo_card.svg)](https://github.com/{}/{})\n\n",
        latest_repo, username, latest_repo
    );

    let regex = Regex::new(r"(?s)## Currently working on\n\n\[!\[.*?\]\(.*?\)\]\(.*?\)\n?").unwrap();

    if regex.is_match(&readme_content) {
        readme_content = regex.replace_all(&readme_content, new_section).to_string();
    } else {
        if !readme_content.is_empty() {
            readme_content.push_str("\n\n");
        }
        readme_content.push_str(&new_section);
    }

    fs::write(readme_path, readme_content)?;

    println!("Updated README.md with local image for: {}", latest_repo);
    Ok(())
}
