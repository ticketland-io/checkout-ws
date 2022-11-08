use std::{
  sync::Arc,
  time::{Duration, Instant},
};
use actix_web::{web};
use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;
use ticketland_utils::logger::console_logger::{LOGGER};
use crate::{
  utils::store::Store,
  session::session_manager::{
    SessionManager,
    Connect,
    Disconnect,
  },
};
use super::{
  types::{WsMessage, WsResponse},
  spawner::Spawn,
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

fn unique_id() -> Uuid {
  Uuid::new_v4()
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WsActor {
  pub store: web::Data<Store>,
  /// Client must send ping at least once per CLIENT_TIMEOUT,
  /// otherwise we drop connection.
  hb: Instant,
  session_manager: Arc<Addr<SessionManager>>,
  pub ws_session_id: Uuid,
}

impl WsActor {
  pub fn new(store: web::Data<Store>, session_manager: Arc<Addr<SessionManager>>) -> Self {
    Self {
      store,
      hb: Instant::now(),
      session_manager,
      ws_session_id: unique_id(),
    }
  }

  /// helper method that sends ping to client every second.
  /// also this method checks heartbeats from client
  fn start_hb(&self, ctx: &mut <Self as Actor>::Context) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        LOGGER.info("Websocket Client heartbeat timed out, disconnecting!");
        ctx.stop();
      } else {
        ctx.ping(b"");
      }
    });
  }
}

impl Actor for WsActor {
  type Context = ws::WebsocketContext<Self>;

  /// Method is called on actor start. We start the heartbeat process here.
  fn started(&mut self, ctx: &mut Self::Context) {
    self.start_hb(ctx);

    // connect to the checkout manager
    self.session_manager.send(Connect {
      addr: ctx.address().recipient(),
      session_id: self.ws_session_id,
    })
    .into_actor(self)
    .then(|res, _, ctx| {
      if let Err(_) = res {
        ctx.stop()
      }

      fut::ready(())
    })
    .wait(ctx);
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    LOGGER.info("stopped ws connection");

    self.session_manager.do_send(Disconnect {
      session_id: self.ws_session_id,
    });
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(ref msg)) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      Ok(ws::Message::Pong(_)) => {
        self.hb = Instant::now();
      },
      Ok(ws::Message::Text(text)) => {
        if let Ok(msg) = serde_json::from_slice::<WsMessage>(text.as_bytes()) {
          ctx.notify(Spawn(msg));
        } 
      },
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        ctx.stop();
      },
      Err(ref error) => {
        LOGGER.error(&format!("ws message error {}", error));
        ctx.stop()
      },
      _ => {
        // TODO: patterns `Ok(Continuation(_))` and `Ok(Nop)` not covered
      }
    };
  }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CheckoutMsg(pub WsResponse);

impl Handler<CheckoutMsg> for WsActor {
  type Result = ();

  fn handle(&mut self, msg: CheckoutMsg, ctx: &mut Self::Context) {
    ctx.text(serde_json::to_string(&msg.0).unwrap());
  }
}
