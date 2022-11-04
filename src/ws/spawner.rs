use std::sync::Arc;
use actix::prelude::*;

use super::{
  ws_actor::WsActor,
  types::{
    WsMessage,
    WsMethod,
    WsResponse,
    WsResponseMsg,
    Status,
  },
  response::{
    unauthorized,
  },
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Spawn(pub WsMessage);

impl Handler<Spawn> for WsActor {
  type Result = ResponseActFuture<Self, ()>;

  fn handle(&mut self, msg: Spawn, _: &mut Self::Context) -> Self::Result  {
    let store = Arc::clone(&self.store);

    let fut = async move {
      match &msg.0.method {
        WsMethod::CreateCheckoutLink {access_token, ..} => {
          if let Ok(_) = store.auth_guard.authenticate(access_token).await {
            WsResponse {
              status: Status::Ok,
              result: Some(WsResponseMsg::CheckoutSessionInProgress),
            }
          } else {
            unauthorized()
          }
        },
        _ => {
          return WsResponse {
            status: Status::Err("Method not supported".to_string()),
            result: None,
          }
        }
      }
    }
    .into_actor(self)
    .map(|result, _, ctx| {
      ctx.text(serde_json::to_string(&result).unwrap());
    });

    Box::pin(fut)
  }
}
