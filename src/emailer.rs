use derive_deref_rs::Deref;
use mail_send::{mail_builder::MessageBuilder, Credentials, SmtpClientBuilder};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};

use crate::{
    error::{EmailError, EmailResult},
    get_profile::{RecipientEmail, RecipientName, SenderEmail, SenderName},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub name: SenderName,
    pub email: SenderEmail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipient {
    pub name: RecipientName,
    pub email: RecipientEmail,
}

#[derive(Serialize, Deserialize, Debug, Deref)]
pub struct SmtpHostName(pub String);

#[derive(Serialize, Deserialize, Debug, Deref)]
pub struct SmtpPort(pub u16);

#[derive(Serialize, Deserialize, Debug)]
pub struct Emailer {
    pub smtp_server: SmtpHostName,
    pub smtp_port: SmtpPort,
    pub sender: Sender,
    pub recipients: Vec<Recipient>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    pub access_token: AccessToken,
}

impl Emailer {
    pub async fn send_email(self) -> EmailResult<()> {
        // Start of sending Email
        let mut message = MessageBuilder::new()
            .from((self.sender.name.to_string(), self.sender.email.to_string()))
            .to(self
                .recipients
                .into_iter()
                .map(|recipient| (recipient.name.to_string(), recipient.email.to_string()))
                .collect::<Vec<_>>())
            .subject(self.subject);
        if let Some(html) = self.html_body {
            message = message.html_body(html);
        }
        if let Some(body) = self.text_body {
            message = message.text_body(body);
        }

        log::debug!("Message: {:?}", &message);
        let credentials = Credentials::new_xoauth2(
            self.sender.email.as_str(),
            self.access_token.secret().as_str(),
        );
        log::info!("Authenticating SMTP XOAUTH2 Credentials....");
        let email_connect = SmtpClientBuilder::new(self.smtp_server.as_ref(), *self.smtp_port)
            .implicit_tls(false)
            .credentials(credentials)
            .connect()
            .await;

        match email_connect {
            Ok(mut result) => {
                log::info!("SMTP XOAUTH2 Credentials accepted!");
                log::info!("Sending SMTP XOAUTH2 Email....");
                let send = result.send(message).await;
                match send {
                    Ok(_result) => {
                        log::info!("Sending Email success!");
                        Ok(())
                    }
                    Err(err) => {
                        log::info!("Sending Email failed!");
                        log::error!("Error Details: {err:?}");
                        Err(EmailError::MailSend(err.to_string()))
                    }
                }
            }
            Err(err) => {
                log::error!("SMTP XOAUTH2 Credentials rejected!");
                log::error!("Error Details: {err:?}");
                Err(EmailError::MailSend(err.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use oauth2::AccessToken;

    use crate::get_profile::{RecipientEmail, RecipientName, SenderEmail, SenderName};

    use super::{Emailer, Recipient, Sender, SmtpHostName, SmtpPort};

    #[test]
    fn test_emailer_struct() {
        let emailer = Emailer {
            smtp_server: SmtpHostName("smtp.office365.com".to_string()),
            smtp_port: SmtpPort(587),
            sender: Sender {
                name: SenderName("Sender Name".to_string()),
                email: SenderEmail("senderemail@outlook.com".to_string()),
            },
            recipients: vec![Recipient {
                name: RecipientName("Reciepient name".to_string()),
                email: RecipientEmail("recipient@gmail.com".to_string()),
            }],
            subject: String::from("This is a test"),
            html_body: None,
            text_body: Some("This is a test body.".to_string()),
            access_token: AccessToken::new("Access-token-12345".to_string()),
        };

        println!("{}", serde_json::to_string(&emailer).unwrap());
    }
}
