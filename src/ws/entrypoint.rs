use std::sync::Arc;
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
  session_manager: web::Data<Arc<Addr<SessionManager>>>,
  req: HttpRequest,
  stream: web::Payload
) -> Result<HttpResponse, Error> {
  let res = ws::start(WsActor::new(store.clone(), session_manager.get_ref().clone()), &req, stream);
  res
}
