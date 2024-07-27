// ++ priority based on older recency date
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    id: u32,
    name: String,
    vegan: bool,
    recency: String
}

fn main() -> std::io::Result<()> {

    let file = File::open("data/recipes.json")?;
    let reader = BufReader::new(file);

    let json_data: Vec<Recipe> = serde_json::from_reader(reader)?;

    for recipe in json_data.iter() {
        println!("{:#}", recipe.name);
    }

    Ok(())
}