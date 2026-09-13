
use email_address::EmailAddress;
use resend_rs::{
  Resend,
  types::CreateEmailBaseOptions,
};

#[derive(Clone)]
pub struct EmailHandler(Resend);

impl EmailHandler {
  #[inline(always)]
  pub fn new(resend: Resend)-> Self {
    Self(resend)
  }

  pub async fn send_verification_email(&self,email: &EmailAddress,base_url: &str,token: &str)-> resend_rs::Result<()> {
    let body=format!(
      r#"
      <h2>Verify your email</h2>
      <p>Click the button below to verify your email address.</p>
        <p>
          <a href="{base_url}/verify?token={token}">
            Verify email
          </a>
        </p>
      <p>This link expires in 5 minutes.</p>
      "#
    );

    let from="onboarding@resend.dev";
    let to=[email.as_str()];

    let email=CreateEmailBaseOptions::new(from,to,"Verify your email")
    .with_html(&body);

    self.0.emails.send(email).await?;
    Ok(())
  }
}













