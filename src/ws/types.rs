use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
  pub method: WsMethod
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsMethod {
  CreatePrimaryPayment {
    event_id: String,
    ticket_type_index: u8,
    recipient: String,
    access_token: String,
  },
  CreateSecondaryPayment {
    event_id: String,
    ticket_type_index: u8,
    recipient: String,
    access_token: String,
    seat_index: u32,
    cnt_sui_address: String,
    listing_sui_address: String,
  },
  PaymentIntentCreated {
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
  PaymentIntentCreated(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WsResponse {
  pub status: Status,
  pub result: Option<WsResponseMsg>,
}
