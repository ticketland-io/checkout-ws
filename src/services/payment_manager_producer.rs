use eyre::Result;
use borsh::{BorshSerialize};
use amqp_helpers::producer::retry_producer::RetryProducer;
use fiat_checkout_manager::models::create_payment::CreatePayment;

pub struct PaymentManagerProducer {
  producer: RetryProducer,
}

impl PaymentManagerProducer {
  pub async fn new(
    rabbitmq_uri: String,
    retry_ttl: u16,
  ) -> Self {
    let producer = RetryProducer::new(
      &rabbitmq_uri,
      &"create_payment",
      &"create_payment",
      &"create_payment.new",
      retry_ttl,
    ).await.unwrap();

    Self {
      producer,
    }
  }

  pub async fn new_checkout_session(&self, msg: CreatePayment) -> Result<()> {
    self.producer.publish(
      &"create_payment",
      &"create_payment.new",
      &msg.try_to_vec().unwrap()
    ).await
  }
}
