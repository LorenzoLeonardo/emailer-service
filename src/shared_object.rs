use std::collections::HashMap;

use async_trait::async_trait;
use ipc_client::client::{error::Error, message::JsonValue, shared_object::SharedObject};
use tokio::sync::mpsc::UnboundedSender;

use crate::emailer::Emailer;
use crate::{
    get_profile::{Profile, ProfileParam},
    interface::Interface,
    task_manager::TaskMessage,
};

pub struct EmailerObject<I>
where
    I: Interface + Send + Sync + 'static,
{
    interface: I,
    _tx: UnboundedSender<TaskMessage>,
}

impl<I> EmailerObject<I>
where
    I: Interface + Send + Sync + 'static,
{
    pub fn new(interface: I, tx: UnboundedSender<TaskMessage>) -> Self {
        Self { interface, _tx: tx }
    }
}

#[async_trait]
impl<I> SharedObject for EmailerObject<I>
where
    I: Interface + Send + Sync + 'static + Clone,
{
    async fn remote_call(
        &self,
        method: &str,
        param: Option<JsonValue>,
    ) -> Result<JsonValue, Error> {
        log::trace!("Method: {} Param: {:?}", method, param);

        let result = match method {
            "getProfile" => {
                let param =
                    param.ok_or(Error::new(JsonValue::String("No parameter".to_string())))?;
                Profile::get_sender_profile(
                    &JsonValue::convert_to::<ProfileParam>(&param)?,
                    self.interface.clone(),
                )
                .await
                .map(|(sender_name, sender_email)| {
                    let mut hash = HashMap::new();
                    hash.insert(
                        "sender_name".to_string(),
                        JsonValue::String(sender_name.to_string()),
                    );
                    hash.insert(
                        "sender_email".to_string(),
                        JsonValue::String(sender_email.to_string()),
                    );
                    JsonValue::HashMap(hash)
                })
                .map_err(|e| {
                    log::error!("{e:?}");
                    Error::new(JsonValue::String(e.to_string()))
                })?
            }
            "sendMail" => {
                let param =
                    param.ok_or(Error::new(JsonValue::String("No parameter".to_string())))?;
                let emailer = JsonValue::convert_to::<Emailer>(&param)?;
                emailer.send_email().await.map_err(|e| {
                    log::error!("{e:?}");
                    Error::new(JsonValue::String(e.to_string()))
                })?;
                JsonValue::String("success".to_string())
            }
            _ => {
                todo!();
            }
        };
        Ok(result)
    }
}
