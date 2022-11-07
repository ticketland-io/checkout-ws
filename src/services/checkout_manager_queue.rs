use eyre::Result;
use borsh::{BorshSerialize};
use amqp_helpers::producer::retry_producer::RetryProducer;
use fiat_checkout_manager::models::create_checkout::CreateCheckout;

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
      &"create_checkout_session",
      &"create_checkout_session",
      &"create_checkout_session.new",
      retry_ttl,
    ).await.unwrap();

    Self {
      checkout_manager_producer,
    }
  }

  pub async fn new_checkout_session(&self, msg: CreateCheckout) -> Result<()> {
    self.checkout_manager_producer.publish(
      &"create_checkout_session",
      &"create_checkout_session.new",
      &msg.try_to_vec().unwrap()
    ).await
  }
}
