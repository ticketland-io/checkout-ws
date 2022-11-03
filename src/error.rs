use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Wrong Method")]
  WrongMethod
}
