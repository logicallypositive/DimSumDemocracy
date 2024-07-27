// ++ priority based on older recency date
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Ingredient {
    name: String,
    quantity: f32,
    unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Recipe {
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Food {
    id: u32,
    name: String,
    vegan: bool,
    recency: String,
    nutrition_value: f32,
    yummy_value: f32,
    affordability: f32,
    preparation_time: u32,
    serving_size: u32,
    cuisine: String,
    spiciness_level: u32,
    caloric_information: u32,
    shelf_life: u32,
    meal_prep_friendly: bool,
    serving_suggestions: Vec<String>,
    recipe: Recipe,
}

fn main() -> std::io::Result<()> {

    let file = File::open("data/recipes.json")?;
    let reader = BufReader::new(file);

    let json_data: Vec<Food> = serde_json::from_reader(reader)?;

    let temp: Food = chooser(&json_data);

    for recipe in json_data.iter() {
        println!("{:#}", recipe.name);
    }

    println!("{:#}", temp.name);

    Ok(())
}

fn chooser(food: &[Food]) -> Food {

    return food[0].clone();

}