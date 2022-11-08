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
use fiat_checkout_manager::models::checkout_session::{CheckoutSession, CheckoutSessionId};
use crate::{
  session::session_manager::{SessionManager, SendMsg},
  ws::types::{
    WsResponse,
    Status,
    WsResponseMsg,
  }
};

pub struct CheckoutSessionHandler {
  session_manager: Addr<SessionManager>
}

impl CheckoutSessionHandler {
  pub fn new(session_manager: Addr<SessionManager>) -> Self {
    Self {
      session_manager,
    }
  }
}

#[async_trait]
impl Handler<CheckoutSession> for CheckoutSessionHandler {
  async fn handle(&self, msg: CheckoutSession, _: &Delivery) -> Result<()> {
    info!("Receive new checkout session {:?} for {}", &msg.checkout_session_id, &msg.ws_session_id);

    let response = match msg.checkout_session_id {
      CheckoutSessionId::Ok(checkout_session_id) => WsResponse {
        status: Status::Ok,
        result: Some(WsResponseMsg::CheckoutSessionCreated(checkout_session_id)),
      },
      CheckoutSessionId::Err(error) => WsResponse {
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
