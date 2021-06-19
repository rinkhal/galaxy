mod group_by;
use std::fs::File;

use common_rs::Result;
use group_by::GroupBy;
use handlebars::Handlebars;
use reqwest::blocking::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Star {
    name: String,
    html_url: String,
    language: Option<String>,
}

fn main() -> Result<()> {
    let client = Client::builder().build()?;
    let stars = client
        .get("https://api.github.com/users/rajatsharma/starred")
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36"
        )
        .send()?
        .json::<Vec<Star>>()?;

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
