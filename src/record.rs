use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::Config;

/// A Record is a simple struct that holds a value.
///
/// The Record is in charge of saving and fetching 
/// the score from the database.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub value: u16
}

impl Record {
  /// An asynchronous function that saves the record score.
  ///
  pub async fn save(score: u16) -> Result<u16, reqwest::Error> {
    let config = envy::from_env::<Config>().expect("error loading config");
    let client = Client::new();
    let url = format!("{}/score.json?auth={}", config.db_url, config.api_key);
    client.patch(url)
      .json(&json!({"value": score}))
      .send()
      .await?;
    Ok(score)
  }

  /// An asynchronous function that fetches the record score.
  ///
  pub async fn fetch() -> Result<Record, reqwest::Error> {
    let config = envy::from_env::<Config>().expect("error loading config");
    let client = Client::new();
    let url = format!("{}/score.json?auth={}", config.db_url, config.api_key);
    let score = client.get(url)
      .send()
      .await?
      .json::<Record>()
      .await?;
    Ok(score)
  }
}