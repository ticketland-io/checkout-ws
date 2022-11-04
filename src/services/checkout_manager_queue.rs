use eyre::Result;
use borsh::{BorshSerialize};
use amqp_helpers::producer::retry_producer::RetryProducer;
use crate::ws::types::WsMethod;

pub struct CheckoutManagerQueue {
  checkout_manager_producer: RetryProducer,
}

impl CheckoutManagerQueue {
  pub async fn new(
    rabbitmq_uri: String,
    retry_ttl: u16,
  ) -> Self {
    let checkout_manager_producer = RetryProducer::new(
      &rabbitmq_uri,
      &"checkout_session",
      &"checkout_session",
      &"checkout_session.new",
      retry_ttl,
    ).await.unwrap();

    Self {
      checkout_manager_producer,
    }
  }

  pub async fn new_checkout_session(&self, create_checkout_session: WsMethod) -> Result<()> {
    if let WsMethod::CreateCheckoutSession {..} = create_checkout_session {
      let msg = {};
      
      self.checkout_manager_producer.publish(
        &"checkout_session",
        &"checkout_session.new",
        &msg.try_to_vec().unwrap()
      ).await
    } else {
      panic!("Create session method should be used")
    }
  }
}
