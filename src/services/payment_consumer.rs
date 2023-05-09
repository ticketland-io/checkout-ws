use std::str::FromStr;
use actix::prelude::*;
use eyre::Result;
use tracing::info;
use async_trait::async_trait;
use uuid::Uuid;
use lapin::{
  message::{Delivery},
};
use amqp_helpers::core::types::Handler;
use fiat_checkout_manager::models::payment_intent::{PaymentIntent, PaymentSecret};
use crate::{
  session::session_manager::{SessionManager, SendMsg},
  ws::types::{
    WsResponse,
    Status,
    WsResponseMsg,
  }
};

pub struct PaymentHandler {
  session_manager: Addr<SessionManager>
}

impl PaymentHandler {
  pub fn new(session_manager: Addr<SessionManager>) -> Self {
    Self {
      session_manager,
    }
  }
}

#[async_trait]
impl Handler<PaymentIntent> for PaymentHandler {
  async fn handle(&mut self, msg: PaymentIntent, _: &Delivery, _: i64,) -> Result<()> {
    info!("Receive new payment {:?} for {}", &msg.payment_secret, &msg.ws_session_id);

    let response = match msg.payment_secret {
      PaymentSecret::Ok(payment_secret) => WsResponse {
        status: Status::Ok,
        result: Some(WsResponseMsg::PaymentIntentCreated(payment_secret)),
      },
      PaymentSecret::Err(error) => WsResponse {
        status: Status::Err(error),
        result: None,
      },
    };

    self.session_manager.do_send(SendMsg {
      response,
      session_id: Uuid::from_str(&msg.ws_session_id)?,
    });

    Ok(())
  }
}
