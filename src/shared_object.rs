use std::collections::HashMap;

use async_trait::async_trait;
use ipc_client::client::{error::Error, shared_object::SharedObject};
use json_elem::jsonelem::JsonElem;
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
    async fn remote_call(&self, method: &str, param: Option<JsonElem>) -> Result<JsonElem, Error> {
        log::trace!("Method: {} Param: {:?}", method, param);

        let result = match method {
            "getProfile" => {
                let param =
                    param.ok_or(Error::new(JsonElem::String("No parameter".to_string())))?;
                Profile::get_sender_profile(
                    &JsonElem::convert_to::<ProfileParam>(&param).map_err(|e| {
                        log::error!("{e:?}");
                        Error::new(JsonElem::String(e.to_string()))
                    })?,
                    self.interface.clone(),
                )
                .await
                .map(|(sender_name, sender_email)| {
                    let mut hash = HashMap::new();
                    hash.insert(
                        "sender_name".to_string(),
                        JsonElem::String(sender_name.to_string()),
                    );
                    hash.insert(
                        "sender_email".to_string(),
                        JsonElem::String(sender_email.to_string()),
                    );
                    JsonElem::HashMap(hash)
                })
                .map_err(|e| {
                    log::error!("{e:?}");
                    Error::new(JsonElem::String(e.to_string()))
                })?
            }
            "sendMail" => {
                let param =
                    param.ok_or(Error::new(JsonElem::String("No parameter".to_string())))?;
                let emailer = JsonElem::convert_to::<Emailer>(&param).map_err(|e| {
                    log::error!("{e:?}");
                    Error::new(JsonElem::String(e.to_string()))
                })?;
                emailer.send_email().await.map_err(|e| {
                    log::error!("{e:?}");
                    Error::new(JsonElem::String(e.to_string()))
                })?;
                JsonElem::String("success".to_string())
            }
            _ => {
                todo!();
            }
        };
        Ok(result)
    }
}
