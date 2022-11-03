use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsMessage {
  pub method: WsMethod
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsMethod {
  VerifyTicket {
    event_id: String,
    ticket_nft: String,
    sig: String,
    access_token: String,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Ok,
  Err(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsResponseMsg {
  VerifyTicketResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsResponse {
  pub status: Status,
  pub result: Option<WsResponseMsg>,
}
