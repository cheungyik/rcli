use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub number: u8,
}

impl Player {
    #[allow(unused)]
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
    #[allow(unused)]
    pub fn from_json(json: &str) -> Result<Player> {
        let player: Player = serde_json::from_str(json)?;
        Ok(player)
    }
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
