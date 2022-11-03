use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub addr: Recipient<WsMessage>,
  pub lobby_id: Uuid,
  pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub room_id: Uuid,
  pub id: Uuid,
}


pub struct CheckoutManager {}

impl CheckoutManager {
  pub fn new(store: Arc<Store>) -> Self {
    Self {}
  }
}

impl Actor for CheckoutManager {
  type Context = Context<Self>;
}

impl Handler<Connect> for CheckoutManager {
  type Result = ResponseActFuture<Self, ()>;
  
  fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
    let fut = async move {
    }
    .into_actor(self);
    
    Box::pin(fut)
  }
}
