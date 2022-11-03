use super::types::{WsResponse, Status};

pub fn unauthorized() -> WsResponse {
  WsResponse {
    status: Status::Err("Unauthorized".to_owned()),
    result: None,
  }
}

pub fn internal_server_error() -> WsResponse {
  WsResponse {
    status: Status::Err("Internal Server Error".to_owned()),
    result: None,
  }
}
