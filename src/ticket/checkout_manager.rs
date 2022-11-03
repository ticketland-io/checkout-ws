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


pub struct CheckoutManager {
  sessions: HashMap<Uuid, Socket>,
}

impl CheckoutManager {
  pub fn new() -> Self {
    Self {
      sessions: HashMap::new(),
    }
  }

  fn send_message(&self, message: &str, id_to: &Uuid) {
    if let Some(socket_recipient) = self.sessions.get(id_to) {
      let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
    } else {
      println!("attempting to send message but couldn't find user id.");
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
