use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    theme: String,
    max_items: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let settings = Settings {
        theme: "Dark".to_string(),
        max_items: 12,
    };

    let json = serde_json::to_string(&settings)?;
    println!("{}", json);

    let parsed: Settings = serde_json::from_str(&json)?;
    println!("{:#?}", parsed);

    Ok(())
}