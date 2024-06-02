use serde::Deserialize;

/// Config is a simple struct holding `.env` variables.
/// 
#[derive(Deserialize, Debug)]
pub struct Config {
  pub db_url: String,
  pub api_key: String
}