use std::{sync::Arc, str::FromStr};
use eyre::Result;
use tracing::info;
use async_trait::async_trait;
use uuid::Uuid;
use lapin::{
  message::{Delivery},
};
use amqp_helpers::core::types::Handler;
use fiat_checkout_manager::models::checkout_session::CheckoutSession;
use crate::{
  checkout::checkout_manager::CheckoutManager,
  ws::types::{
    WsResponse,
    Status,
    WsResponseMsg,
  }
};

pub struct CheckoutSessionHandler {
  checkout_manager: Arc<CheckoutManager>
}

impl CheckoutSessionHandler {
  pub async fn new(checkout_manager: Arc<CheckoutManager>) -> Self {
    Self {
      checkout_manager,
    }
  }
}

#[async_trait]
impl Handler<CheckoutSession> for CheckoutSessionHandler {
  async fn handle(&self, msg: CheckoutSession, _: &Delivery) -> Result<()> {
    info!("Receive new checkout session {} for {}", &msg.checkout_session_id, &msg.ws_session_id);

    let response = WsResponse {
      status: Status::Ok,
      result: Some(WsResponseMsg::CheckoutSessionCreated(msg.checkout_session_id)),
    };

    self.checkout_manager.send_message(response, &Uuid::from_str(&msg.ws_session_id)?)?;

    Ok(())
  }
}
