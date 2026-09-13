




#[derive(serde::Deserialize,serde::Serialize)]
pub struct SignUpForm {
  pub name: Box<str>,
  pub email: Box<str>,
  pub password_hash: Box<str>,
}
















