use std::{fs, io::Write};
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
    let readme_content = format!(
        "# Hi, I am kidskoding\n\n**Currently working on:**\n\n
        [![{}](./current_repo_card.svg)](https://github.com/{}/{})",
        latest_repo, username, latest_repo
    );

    let mut file = fs::File::create(readme_path)?;
    file.write_all(readme_content.as_bytes())?;

    println!("Updated README.md with local image for: {}", latest_repo);
    Ok(())
}
