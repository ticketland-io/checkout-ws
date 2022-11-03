use std::sync::Arc;
use eyre::Result;
use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use tracing::{info, error};
use uuid::Uuid;

use crate::error;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
  pub addr: Recipient<CheckoutMsg>,
  pub session_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
  pub session_id: Uuid,
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

  fn send_message(&self, msg: String, session_id: &Uuid) -> Result<()> {
    let socket = self.sessions.get(session_id).context("session id does not exist")?;
    socket.do_send(CheckoutMsg(msg));

    Ok(())
  }
}

impl Actor for CheckoutManager {
  type Context = Context<Self>;
}

impl Handler<Connect> for CheckoutManager {
  type Result = ();
  
  fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
    self.sessions.insert(
      msg.session_id,
      msg.addr,
    );

    self.send_message(format!("New session id {}", msg.session_id), &msg.session_id);
  }
}
