use curl_http_client::request::HttpRequest;
use derive_deref_rs::Deref;
use http::{HeaderMap, HeaderValue};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::{EmailError, EmailResult},
    interface::Interface,
};

#[derive(Serialize, Deserialize, Deref, Debug)]
pub struct SenderName(pub String);
#[derive(Serialize, Deserialize, Deref, Debug)]
pub struct SenderEmail(pub String);
#[derive(Serialize, Deserialize, Deref, Debug)]
pub struct RecipientName(pub String);
#[derive(Serialize, Deserialize, Deref, Debug)]
pub struct RecipientEmail(pub String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MicrosoftProfile {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    #[serde(rename = "@odata.id")]
    odata_id: String,
    id: String,
    email_address: String,
    display_name: String,
    alias: String,
    mailbox_guid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleProfile {
    id: String,
    email: String,
    verified_email: bool,
    name: String,
    given_name: String,
    picture: String,
    locale: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Profile {
    Microsoft(MicrosoftProfile),
    Google(GoogleProfile),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileUrl(pub Url);

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileParam {
    access_token: AccessToken,
    profile_endpoint: ProfileUrl,
}

impl Profile {
    pub async fn get_sender_profile<I>(
        param: &ProfileParam,
        interface: I,
    ) -> EmailResult<(SenderName, SenderEmail)>
    where
        I: Interface + Send + Sync + 'static,
    {
        let mut headers = HeaderMap::new();

        let header_val = format!("Bearer {}", param.access_token.secret().as_str());
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&header_val).map_err(EmailError::from)?,
        );

        let request = HttpRequest {
            url: param.profile_endpoint.0.to_owned(),
            method: http::method::Method::GET,
            headers,
            body: None,
        };
        let response = interface.http_request(request).await?;

        let body = response
            .body
            .ok_or(EmailError::Curl("No body".to_string()))?;
        let body = String::from_utf8(body).unwrap_or_default();

        let sender_profile: Profile = serde_json::from_str(&body)?;
        let (name, email) = match sender_profile {
            Profile::Microsoft(profile) => {
                log::trace!("Response: {:?}", profile);
                (
                    SenderName(profile.display_name),
                    SenderEmail(profile.email_address),
                )
            }
            Profile::Google(profile) => {
                log::trace!("Response: {:?}", profile);
                (SenderName(profile.given_name), SenderEmail(profile.email))
            }
        };
        log::info!("Sender Name: {}", name.as_str());
        log::info!("Sender E-mail: {}", email.as_str());
        Ok((name, email))
    }
}

#[cfg(test)]
mod tests {
    use crate::get_profile::Profile;

    #[test]
    fn test_google_profile() {
        let google_json = r#"{
            "id": "1525363627",
            "email": "test@gmail.com",
            "verified_email": true,
            "name": "My Name",
            "given_name": "My Name",
            "picture": "https://picutre",
            "locale": "en"
          }"#;

        let google: Profile = serde_json::from_str(google_json).unwrap();

        if let Profile::Google(google) = google {
            println!("deserialize = {:?}", &google);
            println!("serialize = {:?}", serde_json::to_string(&google).unwrap());
        } else {
            panic!("Not Google");
        }
    }

    #[test]
    fn test_microsoft_profile() {
        let ms_json = r#"{
            "@odata.context": "data context",
            "@odata.id": "data id",
            "Id": "sample id",
            "EmailAddress": "test@outlook.com",
            "DisplayName": "My Name",
            "Alias": "Haxxx",
            "MailboxGuid": "en"
          }"#;

        let ms: Profile = serde_json::from_str(ms_json).unwrap();

        if let Profile::Microsoft(ms) = ms {
            println!("deserialize = {:?}", &ms);
            println!("serialize = {:?}", serde_json::to_string(&ms).unwrap());
        } else {
            panic!("Not Microsoft");
        }
    }
}
