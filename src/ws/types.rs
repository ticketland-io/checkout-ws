use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsMessage {
  pub method: WsMethod
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsMethod {
  CreateCheckoutSession {
    buyer_uid: String,
    event_id: String,
    sale_account: String,
    ticket_nft: String,
    recipient: String,
    seat_index: String,
    seat_name: String,
    access_token: String,
  },
  CheckoutSessionCreated {
    session_id: String,
  }
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
  CheckoutSessionInProgress,
  CheckoutSessionCreated,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsResponse {
  pub status: Status,
  pub result: Option<WsResponseMsg>,
}
