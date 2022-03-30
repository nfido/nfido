use crate::mailer::mailer::MailDelivery;

pub struct MailerSmtp {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_user: String,
    pub smtp_password: String,
}

impl MailDelivery for MailerSmtp {
    fn send_email(&self, email_address: String, title: String, content: String) -> Result<(), Err> {
        //发送邮件，用smtp方式
    }
}