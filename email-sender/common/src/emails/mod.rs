pub mod address;
pub mod receiver;
pub mod sender;

pub use self::{address::EmailAddress, receiver::Receiver, sender::Sender};
use crate::data::{self, Data, DataMap, DataSet};
use crate::{prelude::DynError, template};
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::{Credentials, Mechanism},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

type AsyncTokio = AsyncSmtpTransport<Tokio1Executor>;

pub struct Email {
    sender: Sender,
    receivers: DataSet<Receiver>,
    subject: Data,
    template: template::Template,
    global_context: DataMap,
}

impl Email {
    pub fn new<T>(
        sender: sender::Sender,
        receivers: DataSet<Receiver>,
        subject: T,
        template: template::Template,
        global_context: DataMap,
    ) -> Self
    where
        T: Into<Data>,
    {
        let subject = subject.into();

        Self {
            sender,
            receivers,
            subject,
            template,
            global_context,
        }
    }

    pub fn get_from(&self) -> &EmailAddress {
        self.sender.email()
    }

    fn generate_credentials(&self) -> Credentials {
        Credentials::new(
            self.sender.email().email().clone(),
            self.sender.token().as_str().to_string(),
        )
    }

    fn generate_mailer(&self) -> Result<AsyncTokio, DynError> {
        Ok(AsyncTokio::relay(self.sender.server())?
            .authentication(vec![Mechanism::Xoauth2])
            .credentials(self.generate_credentials())
            .build())
    }

    fn generate_body(&self, receiver: &Receiver) -> Result<MultiPart, DynError> {
        let mut local_context: DataMap = receiver.local_context().clone();

        local_context = data::merge_data_maps(local_context, &self.global_context);

        let plain_text: SinglePart = SinglePart::builder()
            .header(header::ContentType::TEXT_PLAIN)
            .body(self.template.get_plain()); // Every message should have a plain text fallback.

        let html_message: SinglePart = SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(self.template.get_html(&local_context)?);

        let message_body: MultiPart = MultiPart::alternative() // This is composed of two parts.
            .singlepart(plain_text)
            .singlepart(html_message);

        Ok(message_body)
    }

    fn generate_message(&self, receiver: &Receiver) -> Result<Message, DynError> {
        let message_body = self.generate_body(receiver)?;
        let from = self.sender.to_string().parse()?;
        let subject = self.subject.clone();

        let message = Message::builder()
            .from(from)
            .to(receiver.to_string().parse()?)
            .subject(subject)
            .multipart(message_body)
            .expect("failed to build email");

        Ok(message)

        // add DKIM signature
    }

    pub async fn send_email_smtp(&self, receiver: &Receiver) -> Result<(), DynError> {
        let message = self.generate_message(receiver)?;
        self.generate_mailer()?.send(message).await?;
        Ok(())
    }

    pub async fn send_all_email_smtp(&self) -> Result<(), DynError> {
        for receiver in &self.receivers {
            match self.send_email_smtp(receiver).await {
                Ok(_) => println!("Sent email to {}", receiver.to_string()),
                Err(e) => println!("Could not send email: {e:?}"),
            }
        }

        Ok(())
    }
}
