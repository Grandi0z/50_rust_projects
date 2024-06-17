use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    content: Vec<Paragraph>,
}
fn main() {
    let article = Article {
        title: "My article".to_string(),
        author: String::from("Me"),
        content: vec![
            Paragraph {
                name: String::from("First paragraph")
            },
            Paragraph {
                name: String::from("Second paragraph")
            },
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("\njson is:\n {}", json);
}