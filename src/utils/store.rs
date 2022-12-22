use std::sync::Arc;
use crate::{
  ws::auth::AuthGuard,
  services::payment_manager_producer::PaymentManagerProducer,
};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub auth_guard: Arc<AuthGuard>,
  pub payment_producer: PaymentManagerProducer,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();

    let auth_guard = Arc::new(AuthGuard::new(config.firebase_auth_key.clone()));

    let payment_producer = PaymentManagerProducer::new(
      config.rabbitmq_uri.clone(),
      config.retry_ttl,
    ).await;

    Self {
      config,
      auth_guard,
      payment_producer,
    }
  }
}
