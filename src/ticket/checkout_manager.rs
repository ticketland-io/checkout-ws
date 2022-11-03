use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use tracing::{info, error};
use uuid::Uuid;

use crate::error;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub addr: Recipient<CheckoutMsg>,
  pub lobby_id: Uuid,
  pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub room_id: Uuid,
  pub id: Uuid,
}


pub struct CheckoutManager {
  sessions: HashMap<Uuid, Socket>,
}

impl CheckoutManager {
  pub fn new() -> Self {
    Self {
      sessions: HashMap::new(),
    }
  }

  fn send_checout_link(&self, checkout_link: String, session_id: &Uuid) {
    if let Some(socket_recipient) = self.sessions.get(session_id) {
      let _ = socket_recipient.do_send(CheckoutMsg(checkout_link));
    } else {
      error!("attempting to send message but couldn't find user id.");
    }
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
