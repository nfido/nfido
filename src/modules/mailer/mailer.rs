pub trait MailDelivery {
    fn send_email(&self, email_address: String, title: String, content: String) -> Result<(), Err> ;
}