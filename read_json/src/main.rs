use serde::{Deserialize, Serialize};
use std::error::Error;
fn main() {
    let sample_json = r#"{
        "title": "My article",
        "author": "Alice",
        "paragraphs": [
            {
                "name": "Introduction"
            },
            {
                "name": "Body"
            },
            {
                "name": "Conclusion"
            }
        ],
        "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
    }"#;
    let parsed = read_json_typed(sample_json).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    let paragraphs = parsed.paragraphs.iter().map(|p| p.name.clone()).collect::<Vec<String>>();
    println!("\n\n Title: {} by {} \n Description: {:?} \n\n Tel: {}", parsed.title, parsed.author, paragraphs, parsed.phones[0]);
}

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name:String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    paragraphs: Vec<Paragraph>,
    phones: Vec<String>,
}

fn read_json_typed(json: &str) -> Result<Article, Box<dyn Error>> {
    let article: Article = serde_json::from_str(json)?;
    Ok(article)
}
 
