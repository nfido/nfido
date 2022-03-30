use std::error::Error;
use crate::mailer::mailer::MailDelivery;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct MailerSmtp {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_email: String,
}

impl MailDelivery for MailerSmtp {
    fn send_email(&self,
                  email_address: String,
                  title: String,
                  content: String) -> Result<&str, Box<dyn Error>> {
        //发送邮件，用smtp方式
        let email = Message::builder()
            .from(format!("NFido <{}>", &self.from_email).parse().unwrap())
            .to(email_address.parse().unwrap())
            .subject(title)
            .body(content)
            .unwrap();

        let creds = Credentials::new(self.smtp_user.to_string(),
                                     self.smtp_password.to_string());

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => {
                log::info!("Email sent successfully!");
                return Ok("success");
            }
            Err(e) => {
                log::info!("Could not send email: {:?}", e);
                return Err(Box::try_from(e).unwrap());
            }
        }
    }
}