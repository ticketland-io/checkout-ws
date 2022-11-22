use std::env;

pub struct Config {
  pub port: u64,
  pub firebase_auth_key: String,
  pub cors_origin: Vec<String>,
  pub rabbitmq_uri: String,
  pub retry_ttl: u16,
}

impl Config {
  pub fn new() -> Result<Self, env::VarError> {
    Result::Ok(
      Self {
        port: env::var("PORT").unwrap().parse::<u64>().unwrap(),
        firebase_auth_key: env::var("FIREBASE_API_KEY").unwrap(),
        cors_origin: env::var("CORS_ORIGIN").unwrap().split(",").map(|val| val.to_owned()).collect(),
        rabbitmq_uri: env::var("RABBITMQ_URI").unwrap(),
        retry_ttl: env::var("RETRY_TTL").unwrap().parse::<u16>().unwrap(),
      }
    )
  }
}
