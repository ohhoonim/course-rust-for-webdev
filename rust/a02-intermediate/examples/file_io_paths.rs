use std::fs; 
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    theme: String,
    max_items: u32,
}

fn main() -> Result <(), Box<dyn std::error::Error>>{
    let path = Path::new("config").join("settings.json");

    let contents = fs::read_to_string(&path)?;
    let mut settings: Settings = serde_json::from_str(&contents)?; 

    settings.max_items = 50;
    let updated = serde_json::to_string_pretty(&settings)?;

    fs::write(&path, updated)?;

    Ok(())
}