use std::fs::File;

use common_rs::Result;
use group_by::GroupBy;
use handlebars::Handlebars;
use reqwest::blocking::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Star {
    name: String,
    html_url: String,
    language: Option<String>,
}

fn fetch_stars(stars: Vec<Vec<Star>>) -> Result<Vec<Star>> {
    if !stars.is_empty() && stars.len() * 100 == stars.concat().len() {
        return Ok(stars.concat());
    }

    let github_token = env::var("GITHUB_TOKEN")?;
    let client = Client::builder().build()?;

    let new_stars = client
        .get("https://api.github.com/users/rajatsharma/starred?per_page=100")
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36"
        )
        .header("Authorization", format!("token {}", github_token))
        .send()?
        .json::<Vec<Star>>()?;

    fetch_stars([&stars[..], &[new_stars][..]].concat())
}

fn main() -> Result<()> {
    let stars = fetch_stars(vec![])?;

    let mut languages = vec![];

    let language_map = stars
        .iter()
        .group_by(|star| star.language.unwrap_or_else(|| "None".into()));

    language_map
        .into_iter()
        .for_each(|(langauge, language_group)| {
            languages.push(json!({ "name": langauge, "stars": language_group }))
        });

    let mut handlebars = Handlebars::new();
    let mut file = File::create("README.md")?;
    handlebars.register_template_file("template", "./template/readme.md")?;
    handlebars.render_to_write("template", &json!({ "languages": languages }), &mut file)?;

    Ok(())
}
