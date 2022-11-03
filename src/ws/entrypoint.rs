use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::utils::store::Store;
use super::ws_actor::WsActor;

/// do websocket handshake and start `MyWebSocket` actor
pub async fn ws_index(
  store: web::Data<Store>,
  req: HttpRequest,
  stream: web::Payload
) -> Result<HttpResponse, Error> {
  let res = ws::start(WsActor::new(store.clone()), &req, stream);
  res
}
