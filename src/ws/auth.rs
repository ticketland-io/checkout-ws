use api_helpers::auth::firebase_auth::{Error, User, FirebaseAuth};

pub struct AuthGuard {
  firebase_auth: FirebaseAuth,
}

impl AuthGuard {
  pub fn new(firebase_auth_key: String) -> Self {
    Self {
      firebase_auth: FirebaseAuth::new(firebase_auth_key.clone()),
    }
  }

  pub async fn authenticate(&self, access_token: &str) -> Result<User, Error> {
    self.firebase_auth.get_user_info(&access_token).await
  }
}
