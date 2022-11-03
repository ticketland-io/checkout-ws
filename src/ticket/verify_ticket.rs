use actix::prelude::*;
use std::sync::Arc;
use crate::{
  utils::store::Store,
  ws::{
    types::{
      WsMethod,
      WsMessage,
      WsResponse,
      Status,
      WsResponseMsg,
    },
  },
};

#[derive(Message)]
#[rtype(result = "WsResponse")]
pub struct VerifyTicket(pub WsMessage);

pub struct VerifyTicketActor {
  store: Arc<Store>,
}

impl VerifyTicketActor {
  pub fn new(store: Arc<Store>) -> Self {
    Self {store}
  }
}

impl Actor for VerifyTicketActor {
  type Context = Context<Self>;
}

impl Handler<VerifyTicket> for VerifyTicketActor {
  type Result = ResponseActFuture<Self, WsResponse>;
  
  fn handle(&mut self, msg: VerifyTicket, ctx: &mut Self::Context) -> Self::Result {
    let fut = async move {
      if let WsMethod::VerifyTicket {..} = msg.0.method {
        // TODO: verify the user ticket
        return WsResponse {
          status: Status::Ok,
          result: Some(WsResponseMsg::VerifyTicketResponse),
        }
      } else {
        panic!("This actor can handle only VerifyTicket method");
      }
    }
    .into_actor(self);
    
    Box::pin(fut)
  }
}
