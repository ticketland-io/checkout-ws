use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
  pub method: WsMethod
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsMethod {
  CreatePrimaryCheckoutSession {
    event_id: String,
    sale_account: String,
    ticket_nft: String,
    ticket_type_index: u8,
    recipient: String,
    seat_index: u32,
    seat_name: String,
    access_token: String,
  },
  CreateSecondaryCheckoutSession {
    event_id: String,
    sale_account: String,
    ticket_nft: String,
    ticket_type_index: u8,
    recipient: String,
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
  Connect,
  Disconnect,
  CheckoutSessionInProgress,
  CheckoutSessionCreated(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsResponse {
  pub status: Status,
  pub result: Option<WsResponseMsg>,
}
