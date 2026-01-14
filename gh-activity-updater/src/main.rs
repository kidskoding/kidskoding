extern crate dotenvy;

use octocrab::Octocrab;
use octocrab::models::events::{Event, EventType};
use octocrab::Page;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN")?;
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    let events: Page<Event> = octocrab
        .get(
            "/users/kidskoding/events/public",
            Some(&[("per_page", "100")]),
        )
        .await?;

    let mut activity_md = String::new();

    if let Some(event) = events.items.iter().find(|e| {
        e.repo.name.starts_with("kidskoding/") && matches!(e.r#type, EventType::PushEvent)
    }) {
        let repo = &event.repo.name;
        let card_svg = format!("https://gh-card.dev/repos/{}.svg", repo);
        let repo_url = format!("https://github.com/{}", repo);

        activity_md.push_str(&format!(
            "[![{repo}]({card_svg})]({repo_url})\n\n"
        ));
    }

    let readme_path = "../README.md";
    let content = fs::read_to_string(readme_path)?;

    let section_header = "## Currently Working On";
    let start = content
        .find(section_header)
        .expect("could not find '## Currently Working On' section in README.md!");
    let after_header = start + section_header.len();

    let rest = &content[after_header..];
    let next_section = rest.find("\n## ").map(|i| after_header + i).unwrap_or(content.len());

    let mut updated_content = String::new();
    updated_content.push_str(&content[..after_header]);
    updated_content.push_str("\n\n");
    updated_content.push_str(&activity_md);
    updated_content.push_str(&content[next_section..]);

    fs::write(readme_path, updated_content)?;

    Ok(())
}