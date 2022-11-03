use std::sync::Arc;
use actix::prelude::*;
use ticketland_core::{
  actor::neo4j::Neo4jActor,
};
use crate::{
  ws::auth::AuthGuard,
};
use super::config::Config;

pub struct Store {
  pub config: Config,
  pub neo4j: Arc<Addr<Neo4jActor>>,
  pub auth_guard: Arc<AuthGuard>,
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

    Self {
      config,
      neo4j,
      auth_guard,
    }
  }
}
