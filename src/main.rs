// ++ priority based on older recency date
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    let temp: Recipe = chooser(&json_data);

    for recipe in json_data.iter() {
        println!("{:#}", recipe.name);
    }

    println!("{:#}", temp.name);

    Ok(())
}

fn chooser(recipes: &[Recipe]) -> Recipe {

    return recipes[0].clone();

}