use std::sync::Arc;
use actix::prelude::*;
use ticketland_core::{
  actor::neo4j::Neo4jActor,
};
use crate::{
  ws::auth::AuthGuard,
  services::checkout_manager_queue::CheckoutManagerQueue,
};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub neo4j: Arc<Addr<Neo4jActor>>,
  pub auth_guard: Arc<AuthGuard>,
  pub checkout_manager_queue: CheckoutManagerQueue,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::new().unwrap();

    let neo4j = Arc::new(
      Neo4jActor::new(
        config.neo4j_host.clone(),
        config.neo4j_domain.clone(),
        config.neo4j_username.clone(),
        config.neo4j_password.clone(),
        config.neo4j_database.clone(),
      )
      .await
      .start(),
    );

    let auth_guard = Arc::new(AuthGuard::new(config.firebase_auth_key.clone()));

    let checkout_manager_queue = CheckoutManagerQueue::new(
      config.rabbitmq_uri.clone(),
      config.retry_ttl,
    ).await;

    Self {
      config,
      neo4j,
      auth_guard,
      checkout_manager_queue,
    }
  }
}
