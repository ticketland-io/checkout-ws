use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::prelude::*;
use super::ws_actor::WsActor;
use crate::{
  utils::store::Store,
  session::session_manager::SessionManager
};

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(
  store: web::Data<Store>,
  checkout_manager: web::Data<Addr<SessionManager>>,
  req: HttpRequest,
  stream: web::Payload
) -> Result<HttpResponse, Error> {
  let res = ws::start(WsActor::new(store.clone(), checkout_manager.get_ref().clone()), &req, stream);
  res
}
