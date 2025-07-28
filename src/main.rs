use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs;
use std::io::Write;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.leetcodestats.com/?username=anikoni";
    let body = get(url)?.text()?;

    let document = Html::parse_document(&body);

    let selector = Selector::parse("div.stat-card div.value").unwrap();
    let mut values = document.select(&selector);

    let total_solved = values
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(""))
        .unwrap_or_else(|| "N/A".to_string());

    let readme_path = "README.md";
    let readme_contents = fs::read_to_string(readme_path)?;

    let new_stats = format!(
        "<!-- LEETCODE-STATS-START -->\n**LeetCode Stats (via leetcodestats.com):** {} problems solved\n<!-- LEETCODE-STATS-END -->",
        total_solved
    );

    let re = Regex::new(r"<!-- LEETCODE-STATS-START -->[\s\S]*<!-- LEETCODE-STATS-END -->")?;

    let updated = if re.is_match(&readme_contents) {
        re.replace(&readme_contents, &new_stats).to_string()
    } else {
        format!("{}\n\n{}", readme_contents, new_stats)
    };

    let mut file = fs::File::create(readme_path)?;
    file.write_all(updated.as_bytes())?;

    println!("Updated README.md with LeetCode stats: {} problems solved", total_solved);

    Ok(())
}
