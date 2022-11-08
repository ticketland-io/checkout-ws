use std::{
  collections::HashMap,
  sync::Arc,
};
use actix::prelude::*;
use eyre::{Result, ContextCompat};
use uuid::Uuid;
use crate::{
  utils::store::Store,
  ws::{
    ws_actor::CheckoutMsg,
    types::{WsResponse, Status, WsResponseMsg},
  },
};

type Socket = Recipient<CheckoutMsg>;

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMsg {
  pub response: WsResponse,
  pub session_id: Uuid,
}


pub struct SessionManager {
  _store: Arc<Store>,
  sessions: HashMap<Uuid, Socket>,
}

impl SessionManager {
  pub fn new(_store: Arc<Store>) -> Self {
    Self {
      _store,
      sessions: HashMap::new(),
    }
  }

  pub fn send_message(&self, msg: WsResponse, session_id: &Uuid) -> Result<()> {
    let socket = self.sessions.get(session_id).context("session id does not exist")?;
    socket.do_send(CheckoutMsg(msg));

    Ok(())
  }
}

impl Actor for SessionManager {
  type Context = Context<Self>;
}

impl Handler<Connect> for SessionManager {
  type Result = ();
  
  fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
    self.sessions.insert(
      msg.session_id,
      msg.addr,
    );

    let _ = self.send_message(WsResponse {
      status: Status::Ok,
      result: Some(WsResponseMsg::Connect),
    }, &msg.session_id);
  }
}

impl Handler<Disconnect> for SessionManager {
  type Result = ();
  
  fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
    self.sessions.remove(&msg.session_id);

    let _ = self.send_message(WsResponse {
      status: Status::Ok,
      result: Some(WsResponseMsg::Disconnect),
    }, &msg.session_id);
  }
}

impl Handler<SendMsg> for SessionManager {
  type Result = ();
  
  fn handle(&mut self, msg: SendMsg, _: &mut Self::Context) -> Self::Result {
    let _ = self.send_message(
      msg.response,
      &msg.session_id,
    );
  }
}
