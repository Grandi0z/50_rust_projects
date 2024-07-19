use postgres::{Client, NoTls, Error};
use std::collections::HashMap;
fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgres://joskal:1357j@localhost:5432/library", NoTls)?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS author (
            id      SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )"
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book (
        id        SERIAL PRIMARY KEY,
            title     VARCHAR NOT NULL,
            author_id INTEGER NOT NULL REFERENCES author(id)
            )
        "
    )?;

    let mut authors = HashMap::new();

    authors.insert(String::from("Zamenga Mabeto"), String::from("DRC"));
    authors.insert(String::from("Sony Labou Tansi"), String::from("Congo"));
    authors.insert(String::from("Ferdinand Oyono"), String::from("Cameroon"));

    for(key, value) in authors.iter() {
        let author = Author{
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",&[&author.name, &author.country],
        )?;
    }

    for row in client.query("SELECT * FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };

        println!("Author: {}, Country: {}", author.name, author.country);
    }

    Ok(())
}

struct Author {
    _id: i32,
    name: String,
    country: String,
}