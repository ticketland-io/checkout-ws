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
};
use super::{
  types::{
    WsMessage,
    WsMethod,
    WsResponse,
    Status,
  },
  response::{
    unauthorized,
    internal_server_error,
  },
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

fn unique_id() -> String {
  Uuid::new_v4().to_string()
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WsActor {
  store: web::Data<Store>,
  /// Client must send ping at least once per CLIENT_TIMEOUT,
  /// otherwise we drop connection.
  hb: Instant,
  subscription_id: String
}

impl WsActor {
  pub fn new(store: web::Data<Store>) -> Self {
    Self {
      store,
      hb: Instant::now(),
      subscription_id: unique_id(),
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
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    LOGGER.info("stopped ws connection");
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    LOGGER.info(&format!("WS: {:?}", msg));

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
struct Spawn(WsMessage);

impl Handler<Spawn> for WsActor {
  type Result = ResponseActFuture<Self, ()>;

  fn handle(&mut self, msg: Spawn, _: &mut Self::Context) -> Self::Result  {
    let store = Arc::clone(&self.store);

    let fut = async move {
      match &msg.0.method {
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
