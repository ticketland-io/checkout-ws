use eyre::Result;
use borsh::{BorshSerialize};
use amqp_helpers::producer::retry_producer::RetryProducer;

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
}
