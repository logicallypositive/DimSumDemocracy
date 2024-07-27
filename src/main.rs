// ++ priority based on older recency date
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

fn main() -> std::io::Result<()> {

    println!("Deez");

    let file = File::open("data/recipes.json")?;
    let reader = BufReader::new(file);

    let json_data: Value = serde_json::from_reader(reader)?;

    println!("{:#}", json_data);

    Ok(())
}