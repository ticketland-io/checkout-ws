use std::sync::Arc;
use actix::prelude::*;
use fiat_checkout_manager::models::create_payment::CreatePayment;
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
    let ws_session_id = self.ws_session_id.to_string();

    let fut = async move {
      match &msg.0.method {
        WsMethod::CreatePrimaryPayment {
          access_token,
          sale_account,
          event_id,
          ticket_nft,
          ticket_type_index,
          recipient,
          seat_index,
          seat_name,
        } => {
          if let Ok(user) = store.auth_guard.authenticate(access_token).await {
            let result = store.checkout_manager_queue.new_checkout_session(CreatePayment::Primary {
              ws_session_id,
              buyer_uid: user.local_id.clone(),
              sale_account: sale_account.clone(),
              event_id: event_id.clone(),
              ticket_nft: ticket_nft.clone(),
              ticket_type_index: ticket_type_index.clone(),
              recipient: recipient.clone(),
              seat_index: seat_index.clone(),
              seat_name: seat_name.clone(),
            }).await;
            
            if let Err(error) = result {
              return  WsResponse {
                status: Status::Err(error.to_string()),
                result: None,
              }
            }

            WsResponse {
              status: Status::Ok,
              result: Some(WsResponseMsg::CheckoutSessionInProgress),
            }
          } else {
            unauthorized()
          }
        },
        WsMethod::CreateSecondaryPayment {
          access_token,
          sale_account,
          event_id,
          ticket_nft,
          ticket_type_index,
          recipient,
        } => {
          if let Ok(user) = store.auth_guard.authenticate(access_token).await {
            let result = store.checkout_manager_queue.new_checkout_session(CreatePayment::Secondary {
              ws_session_id,
              buyer_uid: user.local_id.clone(),
              sale_account: sale_account.clone(),
              event_id: event_id.clone(),
              ticket_nft: ticket_nft.clone(),
              ticket_type_index: ticket_type_index.clone(),
              recipient: recipient.clone(),
            }).await;
            
            if let Err(error) = result {
              return  WsResponse {
                status: Status::Err(error.to_string()),
                result: None,
              }
            }

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
